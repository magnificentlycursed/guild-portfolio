# Security Review Log

This review is part of the [Iterative Adversarial Refinement (IAR)](README.md) suite. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

**Reviewer role: Security Engineer** (Security Engineer / Application Security Engineer)

The purpose of this review is to apply adversarial pressure to find security vulnerabilities, unsafe patterns, validation gaps, and regressions. The review evaluates the threat model, `DESIGN.md`, and source code as they exist at each round.

**Language supplement applied:** `lang/rust.md` (Security section) + `lang/cli.md` (Security section).

**Sycophancy check:** An agent reviewing its own security implementation will rationalize the risks it did not consider during generation as out of scope or not applicable. The most dangerous finding is not a missed CVE — it is a vulnerability class that was never considered at all. Treat every "not applicable" determination with extra scrutiny: verify it genuinely does not apply, not that the reviewer did not think to check. Flag any dimension where the answer is "this project doesn't have X" without verifying that the project cannot be made to have X by an attacker.

---

## Review 1 — 2026-04-27 21:00Z

**Scope:** `DESIGN.md` threat model and specification-level security posture. No source code exists.

**Session note:** In-session with all other domain reviews. Acknowledged quality tradeoff.

---

### Threat Model

**Plausible attackers:** The primary "attacker" is the user themselves — accidental misuse, corrupt data, or unintended input. A secondary concern is a malicious script that pre-stages a crafted `tracker.json` on the filesystem before the user invokes the binary. No network exposure; no multi-user attack surface. The tool runs exclusively on the developer's local machine.

**Crown jewel:** `tracker.json` — integrity and availability of the issue list. Loss means loss of issue history; corruption means errors or silent data loss on the next invocation.

**Entry points:**
1. CLI arguments (all five subcommands — user-controlled, untrusted at the boundary)
2. `tracker.json` on disk (read at startup on every mutation and read command — should be treated as untrusted despite being the tool's own output)

**Threat actors relevant to deployment context:** Single developer's local machine. No remote access, no network. The threat surface is extremely small. The primary risk is data corruption from a bug in the tool itself or from the user manually editing `tracker.json` in a way that violates domain invariants.

---

### Resolved

**Finding 1 — Post-deserialization validation not specified (Dim 2)**

`DESIGN.md` Storage edge cases specified:
- Malformed JSON → error (exit 1)
- Unknown fields → ignored

But the spec did not define behavior when `tracker.json` contains valid JSON with invalid domain values: e.g., `"status": "flying"`, `"priority": ""`, `"id": 0`, `"title": ""`. A Rust `#[derive(Deserialize)]` implementation would successfully deserialize this into the struct — the JSON is structurally valid — and the subsequent commands would operate on corrupted data silently.

This is a boundary violation: data read from disk must be treated as untrusted and validated after deserialization, not just after parsing. The spec must define what happens when the deserialized data violates domain invariants.

Cross-referenced: Data Engineer Finding 1, Red Team Finding 1.

**Resolution:** Added to `DESIGN.md` Storage edge cases: "- `tracker.json` contains valid JSON but invalid domain values (e.g., `"status": "flying"`, `"priority": ""`, `"id": 0`, `"title": ""`) → stderr `Error: Could not read tracker data. The file may be corrupt. Delete tracker.json to start fresh.` → exit 1."

The implementation must validate each deserialized issue's field values against domain constraints after deserialization. A deserialized issue that violates any invariant (invalid status, invalid priority, non-positive ID, empty title) causes the entire load to fail with the corrupt-data error. This is consistent with the spec's approach to malformed JSON.

---

### Accepted Risk

**Finding 2 — `tracker.json` stored as plaintext (Dim 8)**

Issue titles and descriptions may contain sensitive work information (e.g., "Fix vulnerability in authentication — tokens expire after 1 hour"). These are stored as plaintext JSON in the current working directory.

**Accepted risk.** This is a single-user personal tool with no other users. The data is the user's own project notes, stored locally on their own machine. Encrypting issue data would add complexity inconsistent with the assignment's scope and the tool's purpose. The deployment context (developer's local filesystem) does not require data-at-rest encryption for personal notes. This risk is accepted.

**Risk owner:** The user/developer.

---

### Dismissed

**Finding 5 — Error messages reveal OS error strings (Dim 5)**

`tracker.json` exists but is not readable (permissions) → stderr `Error: Could not read tracker data: permission denied.` The phrase "permission denied" is the OS error string, potentially revealing filesystem details.

**Classification:** Dismissed. For a single-user local tool, the OS error string is appropriate diagnostic information — the user needs to know it was a permissions problem to take corrective action. There is no confidentiality concern for a personal tool revealing the name of the OS error to its own user. The spec correctly routes this to stderr.

---

**Finding 6 — Dependency audit not yet runnable (Dim 3)**

No `Cargo.toml` exists yet. `cargo audit` cannot run.

**Classification:** Dismissed at Review 1 — re-evaluated in Review 2 once `Cargo.toml` exists. Tracked forward, not deferred (Security has no `deferred` classification).

---

### Hallucinated

**Finding 3 — File path hardcoded as `tracker.json` in CWD — no path traversal risk (Dim 1, Rust supplement — path traversal)**

The storage path `tracker.json` in the current working directory is hardcoded in the spec. No user-supplied path component is involved in the file path. Path traversal attacks require user-controlled path segments — none exist here. The file path is entirely implementation-determined.

**Classification:** Hallucinated. There is no path traversal risk when the file path is hardcoded. Confirmed by spec review.

---

**Finding 4 — No authentication or authorization concern (Dim 6)**

The spec explicitly excludes multi-user scenarios. Single-user, local-only, no accounts, no sessions. No auth concern applies.

**Classification:** Hallucinated. The concern does not apply to this deployment context.

---

### Open

*(none)*

---

### Summary

One real finding resolved (post-deserialization validation gap). One accepted risk (plaintext storage). Two dismissed, two hallucinated. The threat model is well-bounded for this deployment context. The critical finding — treating deserialized file data as trusted — is now specified. The implementation must apply domain validation after deserialization, not only after JSON parsing.

**Coordination:** Finding 1 cross-referenced in [DATA-ENGINEER-REVIEW.md](DATA-ENGINEER-REVIEW.md) (schema validation) and [RED-TEAM-REVIEW.md](RED-TEAM-REVIEW.md) (crafted file attack). `cargo audit` re-evaluated in Review 2 once `Cargo.toml` exists; CI gate setup tracked in [PLATFORM-ENGINEER-REVIEW.md](PLATFORM-ENGINEER-REVIEW.md).

---

---

## Review 2 — 2026-04-27 22:00Z

**Scope:** Layer 1 stub — `Cargo.toml`, `src/main.rs`, `src/lib.rs`, `tests/layer1.rs`. Evaluating security posture of the dependency declaration and stub code. No behavioral implementation exists.

**Session note:** In-session with all other Layer 1 domain reviews. Acknowledged quality tradeoff.

---

### Resolved

**Finding 1 — `cargo audit` re-evaluated now that `Cargo.toml` exists (Dim 3)**

`Cargo.toml` now exists. `cargo audit` can run. Dependency declaration:

```toml
[dev-dependencies]
assert_cmd = "2"
predicates = "3"
serde_json = "1"
tempfile = "3"
```

No `[dependencies]` section exists — the runtime binary has zero external crate dependencies. Dev-dependencies are only compiled into test binaries. The production binary surface area is exclusively the Rust standard library.

`cargo audit` on a binary with no runtime dependencies will pass with no advisories. The audit becomes meaningful when runtime dependencies are declared (Layer 1 implementation: serde, serde_json, clap, or equivalent). At that point the CI `cargo audit` step will catch any known vulnerabilities in those crates.

**Resolution:** `cargo audit` is now runnable. The carried-forward finding is resolved for the stub phase. When runtime dependencies are declared during Layer 1 implementation, `cargo audit` must pass before the Layer 1 merge gate closes — this is already enforced by the CI pipeline.

---

### Dismissed

*(none)*

### Hallucinated

**Finding 2 — Dev-dependencies included in test binaries (Dim 3)**

`assert_cmd`, `predicates`, `serde_json`, `tempfile` are in `[dev-dependencies]`. They are not compiled into the production binary.

**Classification:** Hallucinated. Dev-dependencies are a Cargo mechanism for test-only dependencies — they produce no additional attack surface in the shipped binary. `cargo build --release` does not compile dev-dependencies. The CI pipeline already runs `cargo build` (not `cargo test`) to produce the release binary. No concern.

---

### Open

*(none)*

---

### Summary

Review 1 Finding 6 resolved (now Review 2 Finding 1): `Cargo.toml` exists; `cargo audit` is now runnable on zero runtime dependencies and will pass. Full audit effectiveness contingent on runtime dependencies being declared during Layer 1 implementation. CI enforces `cargo audit` on every push. No new security findings in stub code.

**Coordination:** *(none)*

---

---

## Review 3 — 2026-04-28 05:30Z

**Scope:** Layer 1 implementation — `src/lib.rs`, `src/main.rs`, `Cargo.toml`, `tests/layer1.rs`. Evaluating implementation security posture: input handling, file I/O, dependency audit, and post-deserialization validation from Review 1's findings. Review-session primer applied — reading this code as an attacker looking for inputs that produce undefined behavior, panics, or silent data corruption. The small attack surface is a conclusion to earn, not an assumption to start from.

**Session note:** In-session with Layer 1 IAR suite. Acknowledged quality tradeoff. This is the implementation gate review; [QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md) Review 3 (cold-session) satisfies the merge-gate cold-session requirement separately.

---

### Resolved

**Finding 1 — Post-deserialization validation not implemented (Dim 2 — Input validation)**

Security Review 1, Finding 1 specified that `tracker.json` data must be treated as untrusted: semantically invalid field values in structurally-valid JSON must trigger the corrupt-data error path. DESIGN.md Storage edge cases explicitly states: "`tracker.json` contains valid JSON but invalid domain values (e.g., `"status": "flying"`, `"priority": ""`, `"id": 0`, `"title": ""`) → stderr `Error: Could not read tracker data. The file may be corrupt. Delete tracker.json to start fresh.` → exit 1."

The Layer 1 implementation's `load_issues` in `lib.rs` called `serde_json::from_str::<Vec<Issue>>()` without any post-deserialization domain validation. A crafted `tracker.json` with `"status": "flying"` would deserialize successfully into the `Issue` struct (all fields are `String` typed), and `cmd_list` would then operate on the invalid data — silently sorting an issue with an unknown priority to the bottom of the list (via `usize::MAX` in `priority_rank`), hiding the corrupt record from the user rather than reporting the corruption.

**Resolution:** Added `issue_fields_are_valid()` validation function and `CORRUPT_DATA_ERROR` constant to `lib.rs`. `load_issues` now validates each deserialized issue against domain constraints (positive ID, non-empty title after trim, valid status, valid priority) and returns `Err(CORRUPT_DATA_ERROR)` if any issue fails validation. Cross-referenced: [DATA-ENGINEER-REVIEW.md](DATA-ENGINEER-REVIEW.md) Review 3, [RED-TEAM-REVIEW.md](RED-TEAM-REVIEW.md) Review 2, [QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md) Review 4.

---

### Accepted Risk

**Finding 2 — Plaintext storage (regression check from Review 1) (Dim 8)**

Unchanged from Review 1 assessment.

**Classification:** Accepted Risk. Single-user personal tool with no other users; data is the user's own project notes, stored locally on their own machine. Risk owner: the user/developer.

---

### Dismissed

**Finding 3 — `cargo audit` passes with 0 advisories (Dim 3)**

Runtime dependencies declared: serde 1.x, serde_json 1.x, clap 4.x, chrono 0.4. `cargo audit` run against `Cargo.lock` (100 locked packages): 0 vulnerabilities found. The CI pipeline enforces this check on every push.

**Classification:** Dismissed. No action required.

---

**Finding 4 — No unsafe `.unwrap()` on user-facing paths (Dim 2 — Panic surface)**

Reviewed all `.unwrap()` in `lib.rs`:
- `serde_json::to_string_pretty(issues).unwrap()` in `save_issues` — infallible for `Vec<Issue>` (all fields serializable; no NaN, no reference cycles). Previously dismissed in SE Review 3; confirmed here.
- No `.unwrap()` on values derived from user input or external file content. The file-read and JSON-parse paths both return `Result` with `?` propagation.

**Classification:** Dismissed. No panic surface on user-facing code paths.

---

**Finding 5 — `CORRUPT_DATA_ERROR` constant deduplicates error message (Positive observation)**

Both the JSON parse failure and domain validation failure produce the same user-actionable error message. The refactored code uses a `const CORRUPT_DATA_ERROR: &str` shared between both code paths. This removes a potential future inconsistency where one path's message drifts from the other.

**Classification:** Dismissed — observation noted positively. No action required.

---

### Open

*(none)*

---

### Summary

One real finding resolved: post-deserialization domain validation was absent and is now implemented. One accepted risk (plaintext storage) carried forward unchanged. Three dismissed. `cargo audit` passes with 0 advisories. No panic surface on user-facing paths. The attack surface remains extremely small: single hardcoded file path, no network, no auth, all user input validated at the CLI boundary. The implementation now treats both structurally-malformed and semantically-invalid file data as corrupt, as required by the spec.

**Coordination:** Finding 1 resolved jointly with [DATA-ENGINEER-REVIEW.md](DATA-ENGINEER-REVIEW.md) Review 3 (domain validation) and [RED-TEAM-REVIEW.md](RED-TEAM-REVIEW.md) Review 2 (crafted-file attack). [QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md) Review 4 added the corresponding test (`invalid_domain_values_in_json_causes_error_exit`).

---

---

## Review 4 — 2026-04-30 00:00Z

**Scope:** Layer 1 gate closure pass — no code changes since Review 3.

**Session note:** In-session with all other domain reviews. Acknowledged quality tradeoff.

---

### Dismissed

*(none)*

### Hallucinated

*(none)*

---

### Open

*(none)*

---

### Summary

No new Security findings. Post-deserialization validation in place. Pre-commit hooks (including `detect-private-key`) active. `cargo audit` 0 advisories. Attack surface unchanged. MVR reached for Layer 1.

**Coordination:** *(none)*

---

---

## Review 5 — 2026-05-01 00:00Z

**Scope:** Layer 2 implementation — new entry points: `tracker status <id> <status>` and `tracker list --status <s>`. Evaluating input validation, panic surface, and dependency audit.

**Session note:** In-session with full Layer 2 IAR suite. Acknowledged quality tradeoff. Review-session primer applied.

**Posture:** Adversarial — looking for crash paths, validation gaps, and information exposure in the Layer 2 additions.

---

### Dismissed

**Finding 1 — `parse_id` and `parse_status` validation coverage (Dim 2 — Input validation)**

All user-supplied strings in Layer 2 are validated before use:
- `parse_id`: rejects non-u64 strings and zero via `.parse::<u64>().ok().filter(|&n| n > 0)`. No panic. ✓
- `parse_status`: rejects any string not in `{"open", "in-progress", "done"}` (now via `VALID_STATUSES` after SA Review 6). No panic. ✓
- `cmd_list`: the `--status` flag value is validated via `parse_status` before any filtering. ✓

**Classification:** Dismissed. Validation is complete and correct at all entry points.

---

**Finding 2 — No new `.unwrap()` on user-facing paths (Dim 2 — Panic surface)**

`parse_id`, `parse_status`, `cmd_status`, and the updated `cmd_list` contain no `.unwrap()`. All error paths use `?` propagation. `#![deny(clippy::unwrap_used)]` enforces this. ✓

**Classification:** Dismissed. Clean.

---

**Finding 3 — No new runtime dependencies (Dim 3 — Dependency audit)**

Layer 2 added no crates. `cargo audit` runs against an unchanged `Cargo.lock`. 0 advisories. ✓

**Classification:** Dismissed.

---

### Hallucinated

**Finding 4 — `tracker status -1 done` treated as a flag by the CLI parser (Dim 2)**

`parse::<u64>()` on a negative-looking string passed through the CLI will never be reached — clap treats `-1` as a flag name and produces a usage error at the argument parsing layer. This is the specified behavior (DESIGN.md Edge Cases / IDs). The implementation is correct.

**Classification:** Hallucinated. The concern does not produce a new attack surface; the behavior is specified and handled by clap.

---

### Accepted Risk

**Finding 5 — Plaintext storage (regression check from Review 1) (Dim 8)**

Layer 2 adds status mutation but does not change the storage model.

**Classification:** Accepted Risk. Carried forward from Review 1 Finding 2. Risk owner: the user/developer.

---

### Open

*(none)*

---

### Summary

No new security findings. Layer 2 adds two entry points; both are fully validated at the boundary. No panic surface. No new dependencies. Attack surface unchanged from Layer 1. MVR reached for Layer 2.

**Coordination:** *(none)*

---

---

## Review 6 — 2026-05-04

**Scope:** Layer 3 implementation — `tracker create --priority`, `tracker list --priority`. Whole-application regression sweep. Cold-session adversarial pass against all source, tests, dependency lock, CI, pre-commit, and toolchain configuration.

**Session note:** Cold session per primer; parallel batch run with other domains; Red Team running concurrently in separate session.

**Posture:** Adversarial. Re-evaluating prior dismissals and looking for vulnerability classes the warm-session reviews never considered. Specific focus on: panic-as-DoS, integer overflow in release mode, post-deserialization invariant gaps beyond field-level validity, supply-chain hygiene, and CI workflow pinning.

---

### Open

**Finding 1 — Panic on broken pipe is a DoS and violates the stderr contract (Dim 5 — Information exposure; Rust supplement — Panic surface; Red Team — Panic-as-DoS)**

Demonstrated reproducer (release build):

```
$ for i in $(seq 1 100); do tracker create "test issue $i" >/dev/null; done
$ tracker list | head -1
ID    Status       Priority  Labels                Title

thread 'main' (16992335) panicked at /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library/std/src/io/stdio.rs:1165:9:
failed printing to stdout: Broken pipe (os error 32)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Two distinct security violations from one root cause:

1. **Panic-as-DoS.** `cmd_list` uses `println!` in a loop (`src/lib.rs:264`). When stdout is closed mid-write (any pipe to `head`, `less` exited early, etc.), the next `println!` panics. Rust's default SIGPIPE handler is `SIG_IGN`, so writes return `EPIPE` rather than terminating the process; `println!`'s internal `expect("failed printing to stdout")` then panics. This is a `.expect()` on a value derived from external state (the pipe consumer) — the exact pattern the Rust supplement Red Team section names as panic-as-DoS. It is not crafted-input dependent: it fires for any list output piped to a short consumer.

2. **Stderr-contract violation / information exposure.** `DESIGN.md` Interface section states "stderr contract: ... No stack traces or internal detail are exposed to the user." The panic emits the rustc commit hash (`e408947bfd200af42db322daf0fadfe7e26d3bd1`), the absolute path to a file inside the Rust standard library source tree (`/rustc/.../library/std/src/io/stdio.rs:1165:9`), the Rust thread model wording, and a `RUST_BACKTRACE` hint. This is exactly the "stack traces or internal detail" the spec forbids. It reaches stderr without the `Error:` prefix and does not exit 1 in a controlled manner — the process aborts with SIGABRT-equivalent semantics.

The `tracker list` use case "pipe to grep / head / less to navigate large issue lists" is a routine CLI usage pattern, not an adversarial input. Once Layer 4 adds labels and Layer 6 adds `tracker show` with multi-line descriptions, the surface widens.

This finding was not raised in any prior Security review. Reviews 1–5 examined `.unwrap()` on user-derived values and dismissed the panic surface based on "no `.unwrap()` on user-facing paths." `println!` is the implicit `.expect()` inside the standard library — the warm sessions did not consider that `println!` itself is a panic site when piped output is interrupted. This is the sycophancy failure mode the primer warns about: passing a dimension because no counterexample came to mind, rather than verifying the control holds.

**Recommended remediation:**
- Install a SIGPIPE handler that resets to `SIG_DFL` early in `main()` (the conventional Unix CLI fix: `unsafe { libc::signal(libc::SIGPIPE, libc::SIG_DFL); }` — requires `libc` crate, two lines), or
- Replace `println!` in `cmd_list` with explicit `writeln!(io::stdout(), ...)` and propagate `io::Error` through the existing `Result<(), String>` error path, treating broken-pipe specifically as a clean exit-0.

The `signal(SIGPIPE, SIG_DFL)` approach is the smaller change and the standard fix for Rust CLIs; it produces a clean exit-141 on pipe closure with no panic, no stack trace, and no stderr noise.

**Classification:** **Open.** Recommendation provided; defer fix selection to the human director. Cannot be dismissed: the spec contract is violated and a routine pipe usage triggers a panic.

Cross-reference: [RED-TEAM-REVIEW.md](RED-TEAM-REVIEW.md) (running in parallel — likely to surface independently as a panic exploit vector).

---

**Finding 2 — `next_id` integer overflow on crafted `tracker.json` (Dim 1, Dim 2 — Persistence data validation; Rust supplement Red Team — integer overflow in release builds)**

`src/lib.rs:39-41`:

```rust
pub fn next_id(existing_ids: &[u64]) -> u64 {
    existing_ids.iter().max().copied().unwrap_or(0) + 1
}
```

A crafted `tracker.json` containing an issue with `"id": 18446744073709551615` (`u64::MAX`) deserializes successfully. `issue_fields_are_valid` accepts it (only checks `id > 0`). `cmd_create` then calls `next_id`, which computes `u64::MAX + 1`:

- **Debug builds** (including `cargo test`): plain `+` panics with "attempt to add with overflow." Panic-as-DoS, same severity class as Finding 1. The DESIGN.md spec describes IDs as "ID larger than any existing issue but within u64 range → error" — the spec does not contemplate IDs at the boundary.
- **Release builds**: silently wraps to `0`. The new issue is created with `id=0`, immediately violating the `id > 0` field invariant. The next time `load_issues` runs, the file fails `issue_fields_are_valid` and the user sees `Could not read tracker data. The file may be corrupt. Delete tracker.json to start fresh.` — silent corruption that bricks the tracker until manual deletion.

Threat model fit: "malicious file content (someone hands the user a malformed tracker.json)" is explicitly named in the task brief. A `tracker.json` containing `id: u64::MAX` is malformed in a way the post-deserialization validator does not catch.

**Recommended remediation:** In `next_id`, use `existing_ids.iter().max().copied().unwrap_or(0).checked_add(1)` and return `Result<u64, String>`, surfacing an "Issue ID space exhausted" error to the user. Alternatively, add a max-ID ceiling check in `issue_fields_are_valid` (e.g., reject any `id > i64::MAX` or any `id == u64::MAX`).

**Classification:** **Open.** Recommendation provided. Real failure mode demonstrated by inspection; not yet exploited in an integration test, but the arithmetic is unambiguous.

Cross-reference: [DATA-ENGINEER-REVIEW.md](DATA-ENGINEER-REVIEW.md) (boundary validation), [RED-TEAM-REVIEW.md](RED-TEAM-REVIEW.md) (crafted-file attack).

---

**Finding 3 — Duplicate IDs in `tracker.json` are not rejected (Dim 2 — Persistence data validation)**

`issue_fields_are_valid` validates each issue independently. A crafted `tracker.json` with two issues sharing the same ID deserializes successfully and passes domain validation. `DESIGN.md` Field invariants: "`id` is unique across all issues and never reused" — a Storage invariant the loader does not enforce.

Behavior on duplicate IDs:
- `cmd_status <dup_id> done`: `position` finds and mutates the first matching issue; the second remains untouched. The user thinks they updated "issue #N" but only one of the two records changed.
- `cmd_create`: `next_id` returns `max + 1`, so create still produces a unique ID — but the duplicates persist.
- Layer 4+ `tracker show <dup_id>` and `tracker delete <dup_id>` will exhibit the same first-match-only behavior, deleting only one of two duplicates and leaving the user with a corrupted file they cannot diagnose without opening it manually.

This is the same vulnerability class as Review 1 Finding 1 (post-deserialization domain validation): the warm sessions caught field-level invariants but missed cross-record invariants. The spec explicitly names ID uniqueness as a load-time guarantee; the implementation does not enforce it.

**Recommended remediation:** Add a uniqueness check in `load_issues` after `issue_fields_are_valid`: collect IDs into a `HashSet`; if `set.len() != issues.len()`, return `Err(CORRUPT_DATA_ERROR)`.

**Classification:** **Open.** Recommendation provided. Demonstrable by file inspection; satisfies the threat model's "crafted tracker.json" actor.

Cross-reference: [DATA-ENGINEER-REVIEW.md](DATA-ENGINEER-REVIEW.md).

---

**Finding 4 — No `deny.toml`; `cargo deny` not configured (Rust supplement — cargo-deny)**

The Rust supplement Security section: "Is `cargo deny check` configured with a `deny.toml`? A complete `deny.toml` configures four sections: `[advisories]`, `[licenses]`, `[bans]`, `[sources]`. Missing or incomplete `deny.toml` is a finding."

No `deny.toml` exists at the project root or anywhere in the `guild-portfolio` tree. CI runs `cargo audit` only, which gates on `[advisories]` (CVEs) but provides no license policy, no banned-crate policy, and no allowed-sources policy. A typosquatted dependency (e.g., a future contributor adds `serdde` instead of `serde`), a GPL-licensed transitive that conflicts with portfolio licensing intent, or a duplicate-version proliferation cannot be detected by the current pipeline.

The Rust supplement Platform Engineering section is explicit: "`cargo audit` alone is insufficient if `cargo deny` is not also present."

This is a regression check that the prior Security reviews did not perform. Reviews 1–5 evaluated `cargo audit` alone and dismissed dependency security based on "0 advisories." That dismissal addresses the `[advisories]` section only.

The single-user portfolio context does not dismiss this control: license policy and banned-source policy apply regardless of user count. The supplement explicitly notes `cargo-vet` may be deferred for portfolio projects, but does not extend that deferral to `cargo-deny`.

**Recommended remediation:** Create `issue-tracker-cli/deny.toml` with all four sections populated: `[advisories]` mirroring `cargo audit` policy, `[licenses]` enumerating allowed SPDX identifiers (MIT, Apache-2.0, BSD-3-Clause, etc.) and explicitly denying GPL, AGPL, etc., `[bans]` with `multiple-versions = "warn"` and any explicit denylist, `[sources]` with `unknown-registry = "deny"` and `allow-registry = ["https://github.com/rust-lang/crates.io-index"]`. Add `cargo install cargo-deny --locked` and `cargo deny check` to the CI pipeline (Platform Engineer cross-reference).

**Classification:** **Open.** Recommendation provided. Cross-reference: [PLATFORM-ENGINEER-REVIEW.md](PLATFORM-ENGINEER-REVIEW.md) for CI integration.

---

### Accepted Risk

**Finding 5 — Plaintext storage (regression check from Review 1) (Dim 8)**

Layer 3 adds `--priority` to `create` and `list`. Storage model unchanged. Issue titles, descriptions, priorities, and labels remain plaintext JSON in the working directory.

**Classification:** Accepted Risk. Carried forward from Review 1 Finding 2. Risk owner: the user/developer.

---

### Dismissed

**Finding 6 — `cargo audit` against current `Cargo.lock` (Dim 3 — Dependency audit)**

`cargo audit` run against `Cargo.lock` (100 crate dependencies) on 2026-05-04: **0 vulnerabilities found**. Advisory database loaded fresh from `RustSec/advisory-db`. CI pipeline still enforces this on every push.

**Classification:** Dismissed. The CVE surface is clean as of this review date. Note: this only covers `[advisories]`; see Finding 4 for the broader supply-chain hygiene gap.

---

**Finding 7 — `unsafe` code (Rust supplement — Unsafe usage)**

`grep "unsafe" src/*.rs` returns zero matches. The crate uses no `unsafe` blocks. Standard library `unsafe` is outside scope of this review.

**Classification:** Dismissed.

---

**Finding 8 — `parse_priority` and `--priority` filter validation (Dim 1 — Input handling)**

The Layer 3 additions validate priority at the CLI boundary via `parse_priority` (`src/lib.rs:182-192`), called in both `cmd_create` and `cmd_list`. Case-insensitive normalization to canonical lowercase. Rejects all values outside `{"low", "medium", "high"}`. No `.unwrap()` on the user-supplied string; `?` propagation throughout. Tested at `tests/layer3.rs:34-43` and `tests/layer3.rs:163-172`.

**Classification:** Dismissed. Validation is correct and tested at the boundary.

---

### Hallucinated

**Finding 9 — `priority_rank` returning `usize::MAX` for unknown priority is a sort-order injection vector**

A crafted issue with `priority: "xyzzy"` would sort to the bottom (rank `usize::MAX`) rather than producing an error.

**Classification:** Hallucinated. `issue_fields_are_valid` rejects any priority outside `PRIORITY_ORDER` at load time, so the unknown-priority code path is unreachable from external input. The `unwrap_or(usize::MAX)` is documented at `src/lib.rs:166-171` as a defensive fallback for an internal-only path, and the Software Engineer review presumably verified the dead-code branch. Confirmed unreachable from the external interface.

---

**Finding 10 — `dtolnay/rust-toolchain@master` in CI is a supply-chain risk**

The CI workflow at `.github/workflows/issue-tracker-cli.yml:28` references `dtolnay/rust-toolchain@master`, a mutable git ref. A compromised maintainer account could replace the action's behavior between CI runs.

**Classification:** Hallucinated *for this Security domain* — properly belongs to [PLATFORM-ENGINEER-REVIEW.md](PLATFORM-ENGINEER-REVIEW.md). The Security supplement scopes Security/Red Team dimensions to crates.io supply chain (`cargo-deny check` for sources). GitHub Actions pinning is a Platform CI concern, not a Rust-language Security concern. Routing rather than dismissing: flag for Platform Engineer review at Layer 4 — recommend pinning `dtolnay/rust-toolchain` to a commit SHA or a fixed tag.

---

### Summary

Round **6** logged. Cold-session sweep produced **four Open findings**, **one Accepted Risk** (carried forward), **three Dismissed**, **two Hallucinated** (one routed to Platform Engineer).

The four Open findings represent vulnerability classes the warm-session reviews 1–5 did not consider:
- **Finding 1** (panic-on-broken-pipe): the warm reviews dismissed panic surface based on absence of explicit `.unwrap()`, missing that `println!` is itself a panic site. Demonstrated reproducer; violates DESIGN.md stderr contract.
- **Finding 2** (`next_id` overflow on `u64::MAX`): integer overflow on crafted file content; debug panic / release silent corruption.
- **Finding 3** (duplicate IDs not validated on load): cross-record invariant the loader does not enforce despite the spec naming it.
- **Finding 4** (no `deny.toml`): supplement-mandated control absent; `cargo audit` alone covers only the `[advisories]` slice.

Findings 1–3 share a common origin: the prior reviews validated controls field-by-field and command-by-command but did not consider system-level invariants (pipe interruption, arithmetic boundaries, cross-record uniqueness). Finding 4 is a tooling gap that was never raised at all.

No re-raise of resolved findings (Review 1 Finding 1 / Review 3 Finding 1 — post-deserialization field validation — verified intact at `src/lib.rs:57-62, 77-79`).

**Coordination:**
- [RED-TEAM-REVIEW.md](RED-TEAM-REVIEW.md) — Findings 1, 2, 3 are likely to surface independently as exploit demonstrations in the parallel Red Team session.
- [DATA-ENGINEER-REVIEW.md](DATA-ENGINEER-REVIEW.md) — Findings 2 and 3 are post-deserialization domain validation gaps in the same family as the existing Data Engineer schema review.
- [PLATFORM-ENGINEER-REVIEW.md](PLATFORM-ENGINEER-REVIEW.md) — Finding 4 requires CI integration of `cargo-deny`; Hallucinated Finding 10 (action pinning) routed for separate evaluation.

**Files modified:** Only this review log appended; no source or configuration changes applied (Open findings + recommendations per Security domain prompt).

---

### Update — 2026-05-04 16:00Z: Layer 3 follow-up resolution pass

All four Open findings from Review 6 closed in the parallel-batch resolution pass. See `CHANGELOG.md` § "Layer 3 follow-up: Open finding resolution pass" for the consolidated diff.

- **F1 (SIGPIPE panic on broken pipe) → Resolved.** `src/main.rs` restores default SIGPIPE handling (`#[cfg(unix)] libc::signal(libc::SIGPIPE, libc::SIG_DFL)`); panic-on-EPIPE no longer occurs. Regression locked by `tests/layer1.rs:list_does_not_panic_on_broken_pipe`. Closes the stack-trace-on-stderr information-disclosure path and the panic-as-DoS path simultaneously.
- **F2 (`next_id` integer overflow) → Resolved.** `next_id` signature changed to `Result<u64, String>`; uses `checked_add(1)` and returns `"Cannot assign new issue ID: maximum ID reached."` on overflow. `cmd_create` propagates with `?`. Regression locked by `tests/layer1.rs:u64_max_id_in_json_blocks_next_create_with_clean_error` and unit test `id_assignment_at_u64_max_returns_error`.
- **F3 (duplicate-ID rejection at load) → Resolved.** `issues_collection_invariants_hold` (HashSet membership walk) added and called from `load_issues`. The `cmd_status` "first match only" silent corruption path is no longer reachable from valid stored data. Regression locked by `tests/layer1.rs:duplicate_ids_in_json_causes_error_exit`.
- **F4 (no `deny.toml` / `cargo-deny`) → Resolved.** `deny.toml` added at the project root with all four supplement-required sections (`[advisories]`, `[licenses]` with explicit allowlist, `[bans]`, `[sources]` restricted to crates.io). New CI step `cargo deny --locked check` runs after `cargo audit` (`.github/workflows/issue-tracker-cli.yml`). The `[sources]` allowlist is `https://github.com/rust-lang/crates.io-index` only — partially mitigates Red Team Review 5 Finding 5 (Cargo.lock supply-chain watch item) by gating any unknown registry/git source.

**Carry-forward Open** (cross-domain, surfaced through Platform-8 in this batch): coverage measurement in CI (Platform F3); pre-commit bypass / CI-side secret scanning (Platform F7) — defense-in-depth gaps that backstop Security but not Security-owned. No new Security findings this round.

---

---

## Review 7 — 2026-05-05 21:35Z

**Scope:** Layer 4 implementation — `tracker create --label <l>...` and `tracker list --label <l>`. New input vector: label values from CLI (repeatable on create; single value on list). Whole-application regression sweep with primary focus on label input validation, JSON serialization round-trip, and error-message information exposure. No new dependencies in Layer 4 (`Cargo.toml` / `Cargo.lock` unchanged vs. `main`).

**Session note:** Cold session per primer; parallel batch run with other Tier-2/3 domains; Red Team scheduled to follow.

**Posture:** Adversarial. Re-evaluating prior dismissals. Specific focus on whether the title control-character defense (Review 1 → resolved at `validate_title`) was extended to the new free-form text field that also flows into the same `list` rendering pipeline.

---

### Threat Model (preamble — not a finding)

- **Plausible attackers:** the local user themselves (typo / pasted clipboard content with embedded control chars), a third party who hands the user a `tracker.json` (named in DESIGN.md threat surface as "crafted file content"), and any process that writes / overwrites `tracker.json` while the user is not looking. No remote attackers; no multi-user surface.
- **Crown jewel:** the integrity of the tabular `tracker list` output and the `show` rendering surface. Compromise of the rendering pipeline enables terminal-escape injection (red text claiming "RESOLVED", hyperlink spoofing, cursor-up rewrite) in any tool that displays issue data — a broader blast radius than the data file itself, because terminal output is rendered without the user inspecting the underlying bytes. Storage corruption of `tracker.json` is a secondary concern (the user can `rm` and start over).
- **Entry points (Layer 4 additions, in scope):** `--label <value>` on `create` (repeatable); `--label <value>` on `list` (single). Pre-existing entry points (regression scope): positional `<title>` on create; `<id>` and `<status>` positional args on `status`; `--status`/`--priority` filters on `list`; `tracker.json` deserialization at `load_issues`.
- **Deployment context:** local developer machine. DESIGN.md "Constraints" specifies "Single user. No network. No accounts." Single-user threat model is documented and consistent with the deployment context.

---

### Open

**Finding 1 — Labels accept control characters; same `list`-output / terminal-escape injection class as the title control-char defense (Dim 1 — Input handling; Dim 5 — Information exposure; Rust supplement Red Team — terminal-escape injection)**

Demonstrated reproducer (debug build, 2026-05-05):

```
$ tracker create "Real" --label $'bug\nFAKE'
Created issue #1: Real
$ tracker list
ID    Status       Priority  Labels                Title
1     open         medium    bug
FAKE              Real
```

The label `bug\nFAKE` deserializes into `issue.labels`, then `cmd_list` renders it via `issue.labels.join(", ")` (`src/lib.rs:450`) and `truncate_with_ellipsis` (line-count agnostic). The newline emerges raw on stdout, breaking the DESIGN.md "tabular format, one issue per line" contract. A label tuned to the column widths can fabricate an entire fake issue row that any line-oriented consumer (`grep`, `awk`, `head`, `wc -l`) treats as a real record.

Demonstrated reproducer for ESC-sequence injection (`cat -v` to expose the bytes):

```
$ tracker create "Real" --label $'\x1b[31mEVIL\x1b[0m'
$ tracker list | cat -v
ID    Status       Priority  Labels                Title
1     open         medium    ^[[31mEVIL^[[0m         Real
```

Without `cat -v` the label renders as red-colored text in any ANSI-capable terminal; OSC 8 hyperlink leaders (`ESC ] 8 ; ; URL ST`) similarly enable hyperlink spoofing in any tool that displays a label. Layer 7 of DESIGN.md adds explicit ANSI coloring of priority/status — a label-injected ESC has identical reach as the title-injected ESC the project already defends against.

DESIGN.md "Edge Cases / Title" rationale (line 290) names exactly this attack class — "control characters break the one-issue-per-line contract of `list` output (newline/CR), corrupt column alignment (tab), and enable terminal-escape injection in any tool that displays the title (ESC)." The same rationale applies verbatim to labels because labels flow through the same rendering pipeline. The implementation defends titles but not labels; `parse_label` (`src/lib.rs:339-346`) checks only `trim().is_empty()`. `issue_fields_are_valid` likewise only checks `!l.trim().is_empty()` (`src/lib.rs:131`) — a hand-edited `tracker.json` with `"labels": ["bug\nFAKE"]` loads cleanly.

This is the sycophancy failure mode named in the primer: every prior layer's resolution to title-control-char injection (Review 1 Finding 1, resolved by `validate_title` rejecting `char::is_control`) created a control whose scope was *implicitly* "the title field." The control was not generalized to "every user-supplied string that can appear in `list` output." Layer 4 introduces a second such field, and the existing control does not cover it. No prior Security review flagged this — Reviews 1–6 evaluated label-empty validation and label deduplication but not label control-char hygiene, because labels did not exist before this layer.

**Recommended remediation (raised to SE; raised to QE for regression test; raised to SO for DESIGN.md amendment):**

- **SO:** Amend DESIGN.md to extend the title control-character prohibition to labels. Suggested edit to "Edge Cases / Labels" (line 300-306): add bullet `- Label containing a control character (Unicode general category Cc) → error: Label cannot contain control characters. Same rationale as Title (above): preserves the one-issue-per-line list contract and prevents terminal-escape injection.` Update Feature 1 "Preconditions" and "Error states" sections accordingly. Also add the label control-char case to "Edge Cases / Storage" line 325 ("invalid domain values" enumeration) so loaded data is treated as corrupt under the same check.
- **SE:** Extend `parse_label` (`src/lib.rs:339-346`) to additionally check `trimmed.chars().any(char::is_control)` and return `Err("Label cannot contain control characters.")`. Extend `issue_fields_are_valid` (`src/lib.rs:131`) to additionally enforce `issue.labels.iter().all(|l| !l.chars().any(char::is_control))` so hand-edited `tracker.json` is rejected at load.
- **QE:** Add unit tests in `src/lib.rs#tests` mirroring the title control-char tests (`label_with_newline_is_rejected`, `label_with_tab_is_rejected`, `label_with_escape_sequence_is_rejected`, `label_with_nul_or_del_is_rejected`, `label_with_printable_unicode_is_accepted`). Add an integration test in `tests/layer4.rs` asserting `tracker create "x" --label $'bug\nFAKE'` exits 1 with `Error: Label cannot contain control characters.` on stderr. Add a `tracker.json` corruption test (parallel to existing duplicate-ID test) for `"labels": ["bug\nFAKE"]`.

**Severity:** Medium-High. No remote attacker; the threat actor is "third-party who hands user a `tracker.json`" or "user pastes clipboard content with embedded controls." The terminal-escape injection class is broad enough that the title defense was deemed worth implementing — symmetry alone justifies the same defense on labels. The DOS-by-fake-row scenario is not crafted-input dependent; an honest user can break their own `list` output by typo.

**Classification:** **Open. Raised to SE / Raised to QE / Raised to SO.** Cannot be Dismissed: the spec contract ("one issue per line") is violable from a Layer 4 entry point and the existing per-DESIGN.md rationale for the title defense applies verbatim. Cannot be Deferred: per the IAR domain prompt, security findings are not deferred.

Cross-references:
- [RED-TEAM-REVIEW.md](RED-TEAM-REVIEW.md) — likely independent surfacing as a label-injection exploit at Tier 4.
- [DATA-ENGINEER-REVIEW.md](DATA-ENGINEER-REVIEW.md) — post-deserialization domain validation gap (same family as Review 6 Findings 2 and 3).
- [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) / DESIGN.md amendment for the spec line.

---

### Dismissed

**Finding 2 — Empty / whitespace-only label rejected at create boundary (Dim 1 — Input handling)**

`parse_label` (`src/lib.rs:339-346`) trims and rejects empty input with `Err("Label cannot be empty.")`. Tested at `tests/layer4.rs:60-83` (`create_with_empty_label_exits_one`, `create_with_whitespace_label_exits_one`). Empty-label corruption rejected at load by `issue_fields_are_valid` (`src/lib.rs:131`). Validation at the boundary is correct. ✓

**Classification:** Dismissed. Validation present and tested.

---

**Finding 3 — Multiple `--label` flags on `list` produce a clean error (Dim 1 — Input handling; Dim 5 — Information exposure)**

`Commands::List.label` is `Option<String>` (`src/main.rs:34`); clap's default behavior is to reject a second `--label` with a usage error, which is transformed by the `try_parse` handler in `main` (`src/main.rs:59-70`) to exit 1 with `Error:`-prefixed stderr and empty stdout. Tested at `tests/layer4.rs:238-257`. The error message does not leak file paths, stack traces, or internal types. ✓

**Classification:** Dismissed. Behavior matches DESIGN.md "Edge Cases / Labels (additional)" and the stderr contract.

---

**Finding 4 — JSON serialization round-trip for labels preserves user input verbatim (Dim 2 — Persistence data validation)**

`Issue.labels: Vec<String>` is serialized by `serde_json::to_string_pretty` and round-trips byte-for-byte through `serde_json::from_str`. No string interpolation, no shell expansion, no `format!` with the label as a format specifier — the labels never reach a sink that interprets them as anything other than opaque text within the JSON document. The CHANGELOG.md note that "labels with quotes / backslashes round-trip as JSON-escaped strings" was verified by inspection. ✓

**Classification:** Dismissed. JSON serialization is safe by construction; the only injection surface is the *display* path (Finding 1).

---

**Finding 5 — `cargo audit` against current `Cargo.lock` (Dim 3 — Dependency audit)**

`cargo audit` run against `Cargo.lock` (100 crate dependencies) on 2026-05-05 from `issue-tracker-cli/`: exit 0; advisory database loaded fresh; no advisories reported. `Cargo.toml` and `Cargo.lock` are unchanged from `main` on the `issue-tracker-cli-labels` branch — Layer 4 added zero crates. ✓

**Classification:** Dismissed. CVE surface clean as of this review date; CI continues to enforce.

---

**Finding 6 — `cargo deny` configuration intact (Rust supplement — cargo-deny)**

`deny.toml` (resolved in Review 6 Finding 4) intact at the project root. All four sections (`[advisories]`, `[licenses]`, `[bans]`, `[sources]`) present and populated. `[sources]` allowlist restricts to `https://github.com/rust-lang/crates.io-index` only. `[licenses]` allowlist matches the dependency tree. No Layer 4 changes to dependencies, so the configuration's coverage of the current tree is unchanged. ✓

**Classification:** Dismissed. Control intact.

---

**Finding 7 — Title control-char defense intact under regression check (Dim 1 — Input handling; regression of Review 1 Finding 1)**

`validate_title` (`src/lib.rs:68-77`) and `issue_fields_are_valid` (`src/lib.rs:128`) continue to enforce `!chars().any(char::is_control)` on title at create and load. Unit tests at `src/lib.rs:478-516` (`title_with_newline_is_rejected`, `title_with_tab_is_rejected`, `title_with_escape_sequence_is_rejected`, `title_with_nul_or_del_is_rejected`, `title_with_printable_unicode_is_accepted`, `issue_field_validation_rejects_control_char_in_title`) exercise the canonical attack vectors. ✓

**Classification:** Dismissed. Regression intact.

---

**Finding 8 — JSON-corruption error path intact under regression check (Dim 5 — Information exposure; regression of Review 1)**

`load_issues` continues to map all deserialization failures and validation failures to the constant `CORRUPT_DATA_ERROR` ("Could not read tracker data. The file may be corrupt. Delete tracker.json to start fresh.") — no leakage of `serde_json::Error` chain, file path beyond the constant message, or column/row from the parser. I/O failure path uses `format!("Could not save tracker data: {}.", e)` / `format!("Could not read tracker data: {}.", e)` which expose the OS error string (e.g. "permission denied", "is a directory") — these are user-actionable diagnostics, not internal-detail leaks; consistent with DESIGN.md "Edge Cases / Storage" lines 327-328 specifying these exact error shapes. ✓

**Classification:** Dismissed. Information-exposure surface unchanged from prior layers.

---

**Finding 9 — Panic surface (Dim 2 — Panic-as-DoS; regression of Review 6 Finding 1)**

`#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]` at the crate root (`src/lib.rs:1-6`). The single `unwrap()` (`src/lib.rs:186`) is gated by `#[allow(clippy::unwrap_used)]` with a documented rationale ("`Vec<Issue>` is always serializable: no floats, no cycles, all fields implement Serialize"). The SIGPIPE handler (`src/main.rs:51-54`) is intact. No new panic sites introduced by Layer 4 — all label handling is `Result<_, String>` propagation via `?`. ✓

**Classification:** Dismissed. Regression intact; no new panic surface.

---

### Hallucinated

**Finding 10 — `--label` filter on `list` accepts arbitrary input including control chars; potential injection through filter rendering**

Concern: `cmd_list`'s `label_filter` parameter (`src/lib.rs:403`) is not validated. A user passing `tracker list --label $'bug\n...'` could ...

**Classification:** Hallucinated. The `label_filter` value is consumed only by `label_matches` (`src/lib.rs:367-369`), which performs `Vec<String>` equality comparison and is never written to stdout/stderr. A control-char filter value simply matches no records (because no stored label can match — Finding 1 notwithstanding, the filter side is rendered nowhere). No injection sink exists on the filter side; the rendering vulnerability is exclusively on the stored-label side covered by Finding 1.

---

**Finding 11 — Label deduplication is case-sensitive and could allow duplicate-effective labels via case variation (e.g. `bug` and `Bug` both stored)**

Concern: a malicious file with both `bug` and `Bug` labels would visually appear duplicated in `show` output and confuse the user.

**Classification:** Hallucinated. DESIGN.md explicitly specifies labels are case-preserved at storage and case-sensitive at filtering ("Edge Cases / Labels" line 305: `--label Bug` does not match `bug`). The implementation matches the spec. This is a UX clarity question, not a security vulnerability — there is no privilege boundary or injection sink involved. Routed (informationally) to UX-REVIEW.md if the apprentice wants to evaluate user clarity, but not a Security finding.

---

### Accepted Risk

**Finding 12 — Plaintext storage (regression check from Review 1) (Dim 8)**

Layer 4 adds `--label` to `create` and `list`. Storage model unchanged: `tracker.json` is plaintext JSON in the working directory; no encryption at rest, no access control beyond filesystem permissions.

**Classification:** Accepted Risk. Carried forward from Review 1 Finding 2, Review 5 Finding 5, Review 6 Finding 5. **Risk owner:** the user/developer (named in DESIGN.md "Constraints" — "Single user. No network. No accounts."). Threat model fits: a developer-local single-user tool's data classification is "internal" at most; encryption at rest is not proportionate to threat.

---

### Summary

Round **7** logged. Cold-session sweep produced **one Open finding (Raised to SE / Raised to QE / Raised to SO)**, **one Accepted Risk** (carried forward), **eight Dismissed** (incl. five regression checks intact), **two Hallucinated**.

The single Open finding (Finding 1 — label control-character injection) is the same vulnerability *class* that title defense (Review 1, resolved) was designed to mitigate, applied to a new field introduced in Layer 4. The control was scoped by-field rather than by-property when first implemented; Layer 4 reveals the cost of that scoping. This is the classic "regression created by additive feature" pattern: every new free-form text field that flows to terminal output reopens the terminal-escape / line-break attack surface unless the defense is generalized.

Three vulnerability classes that the prior reviews dismissed and this review confirms remain intact at Layer 4: title control-char defense (R1), JSON-corruption error masking (R1), panic surface / SIGPIPE handling (R6 F1). Two supply-chain controls intact: `cargo audit` clean, `deny.toml` covering all four sections.

No new dependencies in Layer 4 — `Cargo.toml` and `Cargo.lock` byte-identical to `main`.

**Coordination:**
- [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) — Finding 1 requires a DESIGN.md amendment to extend the title control-char prohibition to labels (only SO modifies DESIGN.md).
- [SOFTWARE-ENGINEER-REVIEW.md](SOFTWARE-ENGINEER-REVIEW.md) — Finding 1 requires `parse_label` and `issue_fields_are_valid` extension (only SE modifies `src/**/*.rs` per CLOSURE-PROTOCOL.md).
- [QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md) — Finding 1 requires symmetric unit + integration tests for label control-char rejection (only QE modifies tests per CLOSURE-PROTOCOL.md).
- [RED-TEAM-REVIEW.md](RED-TEAM-REVIEW.md) — Finding 1 likely to surface independently in the parallel Red Team session as a label terminal-escape exploit. Pressure-test next tier: (a) does the fix close the load path as well as the create path? (b) are there other Layer 4+ surfaces (`show` output for the label, future descriptions in Layer 6) that also flow to the rendering pipeline?
- [DATA-ENGINEER-REVIEW.md](DATA-ENGINEER-REVIEW.md) — Finding 1 includes a load-time validator extension (`issue_fields_are_valid` for labels) in the same family as Review 6 Findings 2 and 3.

**Files modified:** Only this review log appended. No source, tests, or DESIGN.md changes applied per IAR domain authority boundaries (CLOSURE-PROTOCOL.md).

---
