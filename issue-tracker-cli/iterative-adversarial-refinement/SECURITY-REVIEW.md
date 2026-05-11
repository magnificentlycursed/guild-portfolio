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

## Review 8 — 2026-05-06 02:35Z

**Round:** Security Review 8 (Round-2 verification for Layer 4)
**Scope:** Verify Review 7 Finding 1 is closed by the SO/SE/QE round-2 work landed in commit `67ef920`. Re-run dependency audit; spot-check that no new attack surface was introduced by the round-2 source changes.
**Session context:** Warm-verification session per CLOSURE-PROTOCOL.md Section 5 step 4 (cold-batch will follow only if real new findings surface here). Targeted at the F1 reproducers from Review 7.

### Resolved

#### Finding 1 (Round-1) — Label control-character defense

Spec sanctioned (DESIGN.md Feature 1 + Edge Cases / Labels + Edge Cases / Storage per SO Review 17). Implementation lands the recommended fix on **both** `parse_label` and `issue_fields_are_valid` (per SE Review 12 — verified). Tests in place for the create-time path and the load-time path (per QE Review 12).

Adversarial reproduction against the release binary at HEAD (commit `67ef920`):
- `tracker create "Real" --label $'bug\nFAKE'` → `Error: Label cannot contain control characters.` exit 1 ✓
- `tracker create "Real" --label $'\x1b[31mEvil\x1b[0m'` → same error ✓
- Hand-edited `tracker.json` with `"labels": ["bug\nfake"]` → `Could not read tracker data...` exit 1 on `tracker list` ✓
- OSC 8 hyperlink leader (`\x1b]8;;...`) → ESC is `Cc`; rejected by the `is_control()` rule for free ✓

Symmetry with the title defense (SO R13 F1) is now uniform: both fields reject the same `Cc` class at both create-time and load-time.

**Resolved.**

### Dismissed

#### Finding (new) — `display_safe` helper expands stderr surface to Cc-escape errors; could it itself become an attack vector?

Concern: the new `display_safe` helper (`src/lib.rs:149-166`) interpolates `format!("\\u{{{:X}}}", c as u32)` per control char. Could a malicious input combine many Cc bytes to produce a stderr line that exceeds OS pipe-buffer limits or stalls the binary?

Empirical test: `tracker list --priority $(printf '\x1b%.0s' {1..100000})` runs in <50ms and produces a single 700KB stderr line. No buffer stall, no DoS. The threat surface is the user's own terminal, not a remote attacker. `display_safe` always produces bounded output (each Cc byte expands to 6-7 chars; UTF-8 chars pass through unchanged) and never panics. **Dismissed.**

#### Finding (regression) — `cargo audit` and `cargo deny` surface

`Cargo.toml` and `Cargo.lock` unchanged in commit `67ef920` (the `repository` field is added; no new dependencies). Prior audit (Review 7 Finding 5) ran clean against 100 crates; the dependency tree is byte-identical, so the audit verdict carries forward without re-running. **Dismissed (regression intact).**

### Accepted Risk

#### Finding 12 (carried) — Plaintext storage

Unchanged. Risk owner: the user/developer per DESIGN.md "Constraints".

### Summary

Round-2 verification passes. The Open finding from Round 1 (F1 — label control-char injection) is now closed at the spec, source, and test levels with reproducers verified against the release binary. No new findings from this round; the `display_safe` helper and `Cargo.toml` `repository` change introduce no new attack surface.

**Coordination:** Cross-references with Red Team Review 7 (independent verification of the same fix from the attacker lens); SE Review 12 (source-level resolution); QE Review 12 (test coverage); SO Review 17 (spec amendment). The label control-char vulnerability cluster is closed across all four domains.

**Files modified:** Only this log appended.

---

## Review 9 — 2026-05-11 01:08Z

**Round:** Security Review 9 (cold session, Layer 6 — Description + Show + Delete).
**Scope:** Commits `4fb5e67` (Red Gate — `--description` + `show` + `delete` stubs/tests) and `c91676a` (Phase 2b — `validate_description`, `format_show_block`, `cmd_show`, `cmd_delete` bodies; `--description` wired through `cmd_create`). New input vectors: `--description <value>` on `create`; `<id>` positional on `show` / `delete`. New rendering surface: `format_show_block` writing the labelled key-value block (including description) to stdout. New deletion path: `cmd_delete` mutating storage. Cold-session sweep with primary focus on whether the Layer 4 R7 F1 label control-char defense (resolved via `parse_label` + `display_safe` + load-time check) was generalized to description-the-third-free-form-text-field.

**Session note:** Cold session per primer; adversarial posture pressing each prior dismissal that touches description (none — description is new), each prior resolution that defends a free-form text field (title at R1, label at R7-F1), and each new surface.

**Posture:** Adversarial. Re-evaluating whether the title-and-label defense was generalized or scoped-by-field. Specific focus: does `validate_description` mirror `parse_label`? Does `issue_fields_are_valid` extend its control-char check to description? Is `format_show_block` a new injection sink?

---

### Threat Model (preamble — not a finding)

Unchanged from Review 7 preamble: plausible attackers are the local user (typo / pasted clipboard content), a third party who hands the user a `tracker.json`, and any process that overwrites `tracker.json`. Crown jewel is the integrity of the rendering pipeline — Layer 6 widens that pipeline by introducing `tracker show` as a second rendering sink alongside `tracker list`. Storage threat surface gains no new sinks (delete only removes; show is read-only).

---

### Open

**Finding 1 — Description accepts control characters; `tracker show` renders them raw to stdout — direct lineage regression of Layer 4 R7 F1 generalized to the third free-form text field (Dim 1 — Input handling; Dim 2 — Escape injection via `show` output; Dim 3 — Load-time defense gap; Rust supplement Red Team — terminal-escape injection)**

`validate_description` (`src/lib.rs:335-340`) checks only `trim().is_empty()`. It does not reject `char::is_control` and it does not reject any other byte class. The function exists at the same architectural slot as `parse_label` (the input-boundary validator) and as `validate_title` for titles — and is the only one of the three that does not enforce control-char hygiene. `issue_fields_are_valid` (`src/lib.rs:132-135`) likewise only checks `!d.trim().is_empty()` on description; titles get `chars().any(char::is_control)` at line 128 and labels get `label_is_valid` (which includes the same control-char check) at line 131. The load-time defense for description does not exist.

Demonstrated reproducer (release binary at HEAD, in a fresh tempdir):

```
$ tracker create "Real" --description $'<ESC>[31mPWN<ESC>[0m'
Created issue #1: Real
$ tracker show 1 | cat -v
ID:          1
Title:       Real
Status:      open
Priority:    medium
Labels:      (none)
Description: ^[[31mPWN^[[0m
Created:     ...
Updated:     ...
```

Without `cat -v`, the description value renders as actual red `PWN` text on any ANSI-capable terminal. OSC 8 hyperlink leader (`<ESC>]8;;URL<BEL>X<ESC>]8;;<BEL>`) would similarly enable hyperlink spoofing in the `show` output. The reproducer is identical in shape to the Layer 4 R7 F1 label reproducer; the field has changed but the rendering pipeline (raw value interpolation in `format_show_block`'s `format!` block at `src/lib.rs:369-386`) is unguarded.

Demonstrated reproducer for the load-time path (the named threat surface — "third-party hands user a crafted `tracker.json`"):

```
$ printf '%s' '[{"id":1,"title":"Real","description":"[31mPWN[0m","status":"open","priority":"medium","labels":[],"created_at":"2026-01-01T00:00:00Z","updated_at":"2026-01-01T00:00:00Z"}]' > tracker.json
$ tracker show 1 | cat -v
ID:          1
Title:       Real
Status:      open
Priority:    medium
Labels:      (none)
Description: ^[[31mPWN^[[0m
...
```

The JSON-escaped `` (ESC) deserializes through `serde_json::from_str` and `issue_fields_are_valid` accepts the issue (description-empty check only). The same JSON file would be rejected if the ESC bytes were in `title` (line 128 check) or in any `labels` entry (line 131 → `label_is_valid` → `chars().any(char::is_control)`). Description is the lone untrusted-string field exempt from the load-time hygiene rule.

Bare `\r` is a separate sub-case of the same defect. `format_show_block` normalizes `\r\n` → `\n` (line 365) but does not handle bare `\r`:

```
$ tracker create "Real" --description $'before<CR>OVERWRITTEN'
$ tracker show 1
... renders as: Description: OVERWRITTEN ...
```

The carriage return moves the cursor to start of line; `OVERWRITTEN` overwrites the literal `Description: before` prefix and the user reading their `show` output sees an entirely different description from what is stored. This is the one-issue-per-line-contract analogue for `show` (a one-line-per-field contract is implicit in the labelled block; bare CR violates it).

DESIGN.md "Edge Cases / Description" (line 339-345) is silent on control characters — it explicitly allows `\n` (and `\r\n` per the implementation's normalization). The spec does not currently prohibit ESC / CR-alone / tab / NUL / DEL / C1 in description. **This is itself a spec defect** — the rationale that justified the title rule (line 293) and the label rule (line 309) applies verbatim to description, because the description-via-`show` rendering sink has the same blast radius as the title-via-`list` rendering sink: a tool that displays issue data renders the bytes without inspecting them. The spec asymmetry is what allowed the defense to be scoped-by-field rather than generalized; Layer 6 reveals the same generalization-failure pattern Review 7 caught for labels.

This is the **direct lineage regression of Layer 4 R7 F1**: the resolution at Layer 4 (per Review 8) was scoped to `parse_label` / `issue_fields_are_valid` for labels, not generalized to "every user-supplied string that can appear in a rendering sink." A Layer-4-resolution-time generalization (e.g. an `is_control_safe(&str)` predicate shared by title, label, and any future free-form text field) would have made description-as-Layer-6-addition automatically defended. Instead, description repeats the by-field pattern and the same defect class re-opens. Red Team Review 7's pressure-test bullet ("are there other Layer 4+ surfaces (`show` output for the label, future descriptions in Layer 6) that also flow to the rendering pipeline?") flagged exactly this; this review confirms the apprehension was warranted.

**Recommended remediation (raised to SO; raised to SE; raised to QE):**

- **SO:** Amend DESIGN.md "Edge Cases / Description" (`DESIGN.md:339-345`) to add: `- Description containing a control character other than newline (Unicode general category Cc minus U+000A) → error: Description cannot contain control characters (newlines are allowed). Same rationale as Title (line 293) and Labels (line 309): preserves the integrity of show output and prevents terminal-escape injection in any tool that displays the description.` Decide whether to allow `\n` (per current spec which explicitly mentions newlines) and whether to normalize-and-allow `\r\n` (per current implementation) versus rejecting both. Recommendation: allow `\n` only; reject bare `\r` and `\r\n` (and require the user to use `\n` for line breaks); reject all other Cc. Also amend Feature 1 "Error states" with the corresponding error line. Amend "Edge Cases / Storage" (line 333) to include `a control character other than newline in description` in the enumerated invalid-stored-fields list.
- **SE:** Extend `validate_description` (`src/lib.rs:335-340`) to additionally reject `chars().any(|c| c.is_control() && c != '\n')` with `Err("Description cannot contain control characters (newlines are allowed).")`. Extend `issue_fields_are_valid` (`src/lib.rs:132-135`) to enforce the same predicate on stored description. Strongly consider factoring the shared check into a free function `is_control_safe(&str, allowed: &[char])` so the three text fields (title, label, description) share one source of truth — this is the generalization the Layer 4 resolution should have made and didn't. If `\r\n` continues to be allowed (per the current `replace("\r\n", "\n")` behavior in `format_show_block`), normalize before storage (in `validate_description`) rather than at render time — render-time normalization leaves stored data in a state that violates the new invariant.
- **QE:** Add unit tests in `src/lib.rs#tests` mirroring the title control-char tests: `description_with_escape_sequence_is_rejected`, `description_with_bare_cr_is_rejected`, `description_with_tab_is_rejected`, `description_with_nul_or_del_is_rejected`, `description_with_newline_is_accepted`, `description_with_printable_unicode_is_accepted`, `issue_field_validation_rejects_control_char_in_description`. Add an integration test in `tests/layer6.rs`: `create_with_escape_sequence_description_exits_one` asserting `Error: Description cannot contain control characters (newlines are allowed).` on stderr, exit 1, empty stdout. Add a corruption test parallel to the label one for a hand-edited `tracker.json` with `"description": "<JSON-escaped ESC>..."`.

**Severity:** Medium-High. Same severity as Layer 4 R7 F1 — the threat actor is "user pastes clipboard content with embedded controls" or "third party hands user a `tracker.json`". The terminal-escape injection class is identical (deemed worth implementing for title and label). The bare-CR overwrite is a UX-data-integrity bug as much as a security bug: the user reading `show` output may not see what is actually stored.

**Classification:** **Open. Raised to SO / Raised to SE / Raised to QE.** Cannot be Dismissed (the by-field rationale was explicitly identified as a sycophancy failure mode at Layer 4 R7; repeating it would be intellectually dishonest). Cannot be Accepted-Risk (no named owner has accepted; the Layer 4 R7 F1 precedent established that this class is worth fixing for ≤medium severity at this threat model). Cannot be Hallucinated (reproducer demonstrated against the release binary on both create-time and load-time paths).

Cross-references:
- [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) — DESIGN.md amendment is the gating step; without spec sanction the SE fix is a spec deviation.
- [SOFTWARE-ENGINEER-REVIEW.md](SOFTWARE-ENGINEER-REVIEW.md) — `validate_description` + `issue_fields_are_valid` extension; consider factoring shared `is_control_safe` predicate.
- [QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md) — symmetric unit + integration + corruption-file tests.
- [RED-TEAM-REVIEW.md](RED-TEAM-REVIEW.md) — likely independent surfacing as a description terminal-escape exploit and as a bare-CR show-output-overwrite exploit; this is the prediction Review 7's pressure-test bullet made.
- [DATA-ENGINEER-REVIEW.md](DATA-ENGINEER-REVIEW.md) — load-time validator extension (`issue_fields_are_valid` for description) in the same family as Review 6 Findings 2-3 and Review 7 Finding 1.

---

### Dismissed

**Finding 2 — Description JSON serialization round-trip (Dim 2 — Persistence)**

`Issue.description: Option<String>` is serialized by `serde_json::to_string_pretty` and round-trips byte-for-byte through `serde_json::from_str`. The `#[serde(skip_serializing_if = "Option::is_none")]` annotation produces the absent-key shape (verified by `tests/layer6.rs:create_without_description_has_no_field_in_json`). No string interpolation, no shell expansion. JSON itself is safe by construction; the injection surface is exclusively the `show` rendering sink (Finding 1). ✓

**Classification:** Dismissed. Serialization safe.

---

**Finding 3 — `cmd_delete` file-integrity on save failure (Dim 5)**

`cmd_delete` (`src/lib.rs:422-433`) executes load → position → remove → save → println. If `save_issues` returns `Err`, the function returns `Err` and the println does not run; `main.rs` prints the error and exits 1. The on-disk state on a save failure is whatever was there before the partial-write or directly-written failed bytes — the DESIGN.md "Storage invariants" (line 191) explicitly accepts this indeterminate-state behavior: "on I/O failure the file may be in an indeterminate state — the error is reported and the binary exits 1. Atomic writes are the correct production approach and are deferred." Documented design choice; not a security defect. The in-memory `issues` vec is dropped on function return regardless. ✓

**Classification:** Dismissed. Behavior matches DESIGN.md "Storage invariants" line 191 explicitly. Atomic writes are flagged as deferred at spec level.

---

**Finding 4 — Path-traversal / new file-I/O surface (Dim 6)**

`cmd_show` and `cmd_delete` both use the same `Path::new("tracker.json")` from `main.rs:84` that all other commands use; no new path is constructed from user input. The `<id>` argument flows only to `parse_id` (which produces a `u64`) and is never used to construct a path. No new file-handle, no temp-file, no shell-out. ✓

**Classification:** Dismissed. No new file-I/O surface introduced by Layer 6.

---

**Finding 5 — `cargo audit` regression check (Dim 8)**

`cargo audit` run against `Cargo.lock` on 2026-05-11: 1068 advisories loaded; 100 crate dependencies scanned; exit 0; no advisories reported. `git show c91676a --stat` and `git show 4fb5e67 --stat` confirm zero `Cargo.toml` / `Cargo.lock` changes in either Layer 6 commit — the dependency tree is byte-identical to Layer 4 (Review 7 Finding 5, Review 8 dependency check). ✓

**Classification:** Dismissed. CVE surface clean; CI continues to enforce.

---

**Finding 6 — `display_safe` use on new error-path interpolations (Dim 9)**

The new error paths introduced by Layer 6 are: `"Description cannot be empty."` (constant string, no interpolation), `"Issue #<id> not found."` from `cmd_show` and `cmd_delete` (where `<id>` is a `u64` already validated by `parse_id` and serialized via `format!("{}", id)` — `u64`'s `Display` impl emits only ASCII digits, no Cc bytes possible). The `parse_id` error message (`"'<raw>' is not a valid issue ID. Expected a positive integer."`) already passes `raw` through `display_safe` (`src/lib.rs:295`). No new untrusted-input interpolation surface introduced. ✓

**Classification:** Dismissed. New error paths either use constants or interpolate already-validated `u64` values.

---

**Finding 7 — Panic surface regression check (Dim 2 — Panic-as-DoS; regression of Review 6 Finding 1 / Review 7 Finding 9)**

The crate-root `#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]` (`src/lib.rs:1-6`) is intact. The `unwrap` in `save_issues` (`src/lib.rs:213`) remains the single `#[allow(clippy::unwrap_used)]` exception, documented. Layer 6 adds no new `unwrap` / `expect` / `panic` sites — `validate_description` returns `Result<_, String>`; `cmd_show` and `cmd_delete` propagate via `?`. SIGPIPE handler (`src/main.rs:64-67`) intact and applies to `tracker show` stdout as well as `tracker list`. ✓

**Classification:** Dismissed. Regression intact.

---

**Finding 8 — Title and label control-char defense regression check (regression of R1 / R7-F1)**

`validate_title`, `parse_label`, and `issue_fields_are_valid` for title + label all intact at the source lines this review walked. Unit tests at `src/lib.rs:907-969` for the label cluster and `src/lib.rs:682-712` for the title cluster all present. No regression in the two already-defended fields — the regression is in the *third* field's absence of the same defense (Finding 1). ✓

**Classification:** Dismissed. The defended fields remain defended; the new field is the gap.

---

### Hallucinated

**Finding 9 — `cmd_delete` allows arbitrary issue removal because there is no confirmation prompt; auditable-action gap**

Concern: a malicious script or accidental shell history replay could delete an issue without warning; absence of `--yes` flag is a security gap.

**Classification:** Hallucinated. DESIGN.md "Approved Deviations from Assignment" D1 (`DESIGN.md:413-420`) explicitly documents the non-confirmation choice with approver and rationale, including the threat model fit ("single user on a local machine, where accidental-deletion friction is the user's own concern, not a multi-stakeholder safety surface"). The re-evaluation trigger is named (multi-user / shared context). Not a Security domain finding under the documented single-user threat model. UX-domain concern at most.

---

**Finding 10 — `tracker.json` becomes empty after deleting the last issue, leaking "tracker existed" via file presence**

Concern: presence of an empty-array `tracker.json` reveals that the user once used the tool, even after deleting all issues — information disclosure.

**Classification:** Hallucinated. The DESIGN.md storage model (line 188-193) describes `tracker.json` as the sole data store and treats file presence/absence as a normal application artifact, not a secret. "Single user. No network. No accounts." threat model has no privileged-vs-unprivileged-observer distinction for the file's existence on the user's own filesystem. No injection sink, no privilege boundary, no protected information class. Not a Security domain concern.

---

### Accepted Risk

**Finding 11 (carried) — Plaintext storage**

Unchanged from Review 1 / 5 / 6 / 7 / 8. Risk owner: the user/developer per DESIGN.md "Constraints". Description is now stored verbatim in plaintext alongside title and labels; the data-classification posture is unchanged — single-user local tool with "internal" at most data classification.

---

### Summary

Round **9** logged. Cold-session sweep produced **one Open finding (Raised to SO / Raised to SE / Raised to QE)**, **one Accepted Risk** (carried forward), **seven Dismissed** (incl. five regression checks intact and the two new-field-but-no-new-attack-surface findings — JSON serialization and path-handling), **two Hallucinated**.

**Top concern.** The single Open finding (Finding 1 — description control-character injection in `show` output, with bare-CR sub-case) **is a direct lineage regression of Layer 4 R7 F1**. The Layer 4 resolution defended the second free-form text field (label) by extending `parse_label` and `issue_fields_are_valid`, but did not generalize the defense to "every user-supplied string that can appear in a rendering sink." Layer 6 introduces the third such field (description) and the same defect class re-opens. The pattern Review 7 named ("regression created by additive feature: every new free-form text field that flows to terminal output reopens the terminal-escape / line-break attack surface unless the defense is generalized") is exactly the pattern this review observed. Red Team Review 7 explicitly named this prediction in its pressure-test bullet — the prediction is confirmed. The SE remediation should include factoring an `is_control_safe` helper so a Layer 8+ free-form text field automatically inherits the defense.

`cargo audit` clean (0 advisories on 100 crates). `Cargo.toml` / `Cargo.lock` byte-identical to Layer 4 — no supply-chain regression.

**Coordination:**
- [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) — Finding 1 requires DESIGN.md amendments at lines 38 (Feature 1 error states), 339-345 (Edge Cases / Description), and 333 (Edge Cases / Storage). Only SO modifies DESIGN.md.
- [SOFTWARE-ENGINEER-REVIEW.md](SOFTWARE-ENGINEER-REVIEW.md) — Finding 1 requires `validate_description` + `issue_fields_are_valid` extension; strongly recommend factoring shared `is_control_safe(&str)` predicate to close the by-field-scoping pattern at the source level.
- [QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md) — Finding 1 requires symmetric unit + integration + corruption-file tests; mirror the Layer 4 R7 F1 test cluster shape.
- [RED-TEAM-REVIEW.md](RED-TEAM-REVIEW.md) — Finding 1 is the predicted-exploit-cluster from R7's pressure-test list; independent confirmation expected.
- [DATA-ENGINEER-REVIEW.md](DATA-ENGINEER-REVIEW.md) — load-time validator extension in the same family as Review 6 F2/F3 and Review 7 F1.

**Files modified:** Only this review log appended; no source, tests, or DESIGN.md changes applied per IAR domain authority boundaries (CLOSURE-PROTOCOL.md). Attack payloads in this review are abstracted (`<ESC>`, `<CR>`, `<BEL>`) per primer guidance.

---

## Review 10 — 2026-05-11 02:00Z

**Round:** Security Review 10 (Round-2 closure for Layer 6)
**Scope:** Verify Round-1 Open finding (description control-char defense) is resolved by commit `9b775f0`. Warm closure-verification.

### Round-1 finding closure

- **F1 (description control-char defense — third lineage instance after Title L1 / Labels L4):** **Resolved by commit `9b775f0`.** Resolution applied at three boundaries per the cross-domain consensus:
  1. **Spec (DESIGN.md):** Feature 1 error states adds `Error: Description cannot contain control characters other than newline.`. Edge Cases / Description enumerates the rule with `\n` carve-out rationale and Cf accepted-risk stance. Edge Cases / Storage adds the new corruption trigger.
  2. **Create-time (`src/lib.rs`):** `validate_description` rejects `char::is_control()` except `\n`. The `\n` carve-out is description-specific because the spec permits multi-line descriptions.
  3. **Load-time (`src/lib.rs`):** New `description_is_valid` helper enforces the same rule via `issue_fields_are_valid`. Mirrors `label_is_valid` from Layer 4 R2.

Adversarial reproducers from Round 1 re-executed against the release binary at `HEAD`:
- `tracker create "X" --description '<ESC>[31mPWN<ESC>[0m'` → now **exits 1** with `Error: Description cannot contain control characters other than newline.`
- Hand-edited tracker.json with JSON-escaped ESC in description + `tracker list` → now **exits 1** with corrupt-data error
- OSC 8 hyperlink leader / bare CR / tab / DEL: all rejected at create + load
- `\n` (legitimate multi-line description): accepted at both boundaries

### Carry-forward verification

- Layer 4 R2 lineage closures (label control-chars at create + load; `display_safe` at parse-error sites): No regression at Layer 6.
- Plaintext storage Accepted Risk (carried since Layer 1): unchanged.

### New findings

*(none this round.)*

### Summary

1/1 Round-1 Security finding Resolved. The Title L1 → Labels L4 → Description L6 generalization-failure pattern is now closed; all three free-form text fields carry the equivalent control-char defense at create + load. **No Security findings remain Open for Layer 6.**

**Coordination:** *(none — closure pass)*

---

## Review 11 — 2026-05-11 22:30Z

**Round:** Security Review 11 (cold session, Layer 7 — Polish: `--help`, TTY-detected color output, error-message specificity).
**Scope:** Commits `7b461aa` (Phase 2a Red Gate — `tests/layer7.rs`), `a2b8062` (Phase 2b — ANSI color emission added to `cmd_list` and `cmd_show` value cells, TTY-gated via `std::io::IsTerminal`), and `603c689` (manual closure). Primary focus: the new ANSI-emission code path in `src/lib.rs:46-97` (`ANSI_RESET`, `priority_ansi`, `status_ansi`, `wrap_color`, `pad_after_color`), TTY detection at `cmd_list:835` / `cmd_show:591`, and any pre-color regression in `validate_title` / `validate_description` / `parse_label` / `issue_fields_are_valid`. Cold-batch surfacing per CLOSURE-PROTOCOL.md §1 (Security may apply CVE fixes to `src/**`; no CVE fixes triggered this round).

**Session note:** Cold session per primer; loaded `prompts/review-session.md` and `domains/role/SECURITY-REVIEW.md` first, then `supplements/rust.md` + `supplements/cli.md`, then DESIGN.md / TODO.md L350-396 / CHANGELOG.md L1-38 / Cargo.toml / Cargo.lock (no diff vs. Layer 6 confirmed via `git diff main..HEAD -- Cargo.toml Cargo.lock`) / tests/layer7.rs / src/lib.rs (full read) / src/main.rs / prior SECURITY-REVIEW R9-R10. Anonymization: attack payloads abstracted as `<ESC>`, `<CR>`, `<TOKEN>`, `<PATH>` per primer; no developer identity, home directory, or git email surfaced.

**Posture:** Adversarial. Layer 7 emits raw ANSI escape bytes (`\x1b[…`) to stdout — the same byte class previously rejected as input by `validate_title` / `parse_label` / `validate_description`. The pressure point: is the input-side defense still load-bearing for the new output path, or is the output path independently safe? Sycophancy-guard challenge from the assignment: "Hardcoded constants, no user-controllable color, TTY-gated" sounds airtight — find the gap.

---

### Threat Model (preamble — not a finding)

Unchanged from R9-R10 preamble. Plausible attackers: the local user (typo / pasted clipboard content carrying ANSI sequences), a third party who hands the user a `tracker.json` (load-time injection vector), any process that overwrites `tracker.json` between commands. Crown jewel is **the integrity of the rendering pipeline to the user's terminal** — Layer 7 widens this by deliberately emitting ESC bytes to that pipeline. Entry points are unchanged (CLI args via clap; `tracker.json` via `load_tracker`); the new *output* surface is `cmd_list` row-formatting and `cmd_show`'s `format_show_block`. No network, no shared filesystem, no privilege boundary in the documented deployment context (single-user local CLI per DESIGN.md "Constraints").

Out-of-scope per documented threat model: an attacker who controls the user's pty (e.g., a hostile ssh-forwarded session) — DESIGN.md Edge Cases / Labels L319 names the "single user attacking themselves with hand-pasted clipboard content or a hand-edited tracker.json" surface; pty-spoofing is explicitly the wider threat model not in scope. Re-evaluation trigger named at the same line.

---

### Regression check

Pre-Layer-7 controls verified intact at the source lines walked:

- `validate_title` (`src/lib.rs:167-176`) — empty-after-trim rejection and `chars().any(char::is_control)` defense unchanged. Unit tests at `src/lib.rs:874-916` and the load-time mirror `issue_fields_are_valid` (`src/lib.rs:226-240`, title control-char check at line 229) all present.
- `parse_label` (`src/lib.rs:681-693`) — empty/control-char/comma rejection unchanged. Load-time mirror `label_is_valid` (`src/lib.rs:257-259`) intact; the predicate is called per-label inside `issue_fields_are_valid`.
- `validate_description` (`src/lib.rs:506-516`) — Round-2 (R9 F1) Cc-defense with `\n` carve-out intact. Load-time mirror `description_is_valid` (`src/lib.rs:249-251`) intact.
- `display_safe` (`src/lib.rs:268-278`) — escapes Cc as `\u{XX}` for stderr interpolation; `parse_status` / `parse_priority` / `parse_id` all route the user-supplied value through it (`src/lib.rs:437`, `649`, `450`). No new error-path interpolation introduced by Layer 7.
- `tracker_is_valid` (`src/lib.rs:289-306`) — per-record validity (calls `issue_fields_are_valid` which checks `PRIORITY_ORDER.contains(&issue.priority.as_str())` and `VALID_STATUSES.contains(&issue.status.as_str())`); unique-ID check; `next_id >= 1`; `next_id > max(issue.id)` when non-empty. The closed-enum check on `status` and `priority` is the **load-time precondition the new output path depends on** (see Finding 1 below).
- Panic surface: crate-root `#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::missing_errors_doc)]` (`src/lib.rs:1-6`) intact; the single `#[allow(clippy::unwrap_used)]` exception in `save_tracker` (`src/lib.rs:349-351`) unchanged; Layer 7 adds no new `unwrap` / `expect` / `panic` sites. SIGPIPE handler (`src/main.rs:64-67`) intact.
- `cargo audit --no-fetch` against `Cargo.lock` at HEAD on 2026-05-11: 1069 advisories loaded, 100 crate dependencies scanned, **exit 0, no advisories reported**. `git diff main..HEAD -- Cargo.toml Cargo.lock` is empty — Layer 7 added zero dependencies (raw ANSI escapes; no `anstyle` / `termcolor`).

No regression observed in any Layer-1–6 defense.

---

### Open

**Finding 1 — `wrap_color` trusts the caller's value-byte hygiene; defense-in-depth gap on the new ANSI-emitting code path (Dim 1 — Output handling; Dim 2 — Persistence-to-output trust chain; Rust supplement Red Team — terminal-escape injection)**

The Layer 7 ANSI-emitting helper `wrap_color` (`src/lib.rs:79-84`):

```
fn wrap_color(value: &str, ansi: Option<&str>) -> String {
    match ansi {
        Some(prefix) => format!("{}{}{}", prefix, value, ANSI_RESET),
        None => value.to_string(),
    }
}
```

emits `<ANSI_START><value><ANSI_RESET>` to stdout via `format_show_block` and the per-issue row of `cmd_list`. The function **does not inspect `value`** for Cc bytes. Its safety rests entirely on a multi-step input-side trust chain:

1. `parse_status` / `parse_priority` (`src/lib.rs:430-440` / `642-652`) reject values outside the closed enum sets `VALID_STATUSES` and `PRIORITY_ORDER` and lowercase the survivors.
2. `issue_fields_are_valid` (`src/lib.rs:226-240`) re-checks `VALID_STATUSES.contains(&issue.status.as_str())` and `PRIORITY_ORDER.contains(&issue.priority.as_str())` at load time, so a hand-edited `tracker.json` planting `"status": "<ESC>[31mpwn"` is rejected with the standard corrupt-data error (verified by reproducer below — the load path correctly rejects an ESC-bearing status value).

So today, end-to-end, no ESC byte can reach `wrap_color`'s `value` parameter through any documented path. The closed-enum invariants are load-bearing — and that is the gap. **The new output path is one refactor away from re-opening a terminal-escape injection surface that the input-side defense currently masks.** Concrete future failure modes that would slip past the closed-enum check:

- A Layer 8+ spec amendment introduces a user-supplied "category" or "milestone" value with a non-enumerated value space and threads it through a new `category_ansi` helper. The author cargo-cults `wrap_color` and now the new column carries an unguarded injection sink.
- A future relaxation of `parse_status` / `parse_priority` (e.g., to allow custom user-defined priorities) drops the closed-enum gate without adding a control-char gate. The output path silently becomes an injection vector.
- `wrap_color` is reused (it's not crate-private — though it is module-private at `fn` not `pub fn`) for a free-form text field at a future call site. The input-side defense for the new field would need to independently re-discover the Cc rule.

Demonstrated load-time positive control (reproducer against release binary at HEAD, in a fresh tempdir):

```
# Hand-edited tracker.json with ESC in status
$ printf '%s' '{"issues":[{"id":1,"title":"Real","status":"<ESC>[31mPWN","priority":"medium","labels":[],"created_at":"2026-01-01T00:00:00Z","updated_at":"2026-01-01T00:00:00Z"}],"next_id":2}' > tracker.json
$ tracker list
Error: Could not read tracker data. The file may be corrupt. Delete tracker.json to start fresh.
$ echo $?
1
```

This is the **happy regression result**: the existing load-time defense correctly catches the injection attempt. The finding is not that the system is currently exploitable — it is that the new output path has **no defense-in-depth on its own boundary**.

Lineage pattern. The R9 F1 closure rationale explicitly called for factoring shared invariants so a future text-field addition automatically inherits the defense (R9 closure recommendation: "Strongly consider factoring the shared check into a free function ... so the three text fields share one source of truth — this is the generalization the Layer 4 resolution should have made and didn't."). That generalization was applied at the *input* boundary across title/label/description. The Layer 7 output boundary has the **same shape of risk** (a free-form-ish text value flowing to a rendering sink) without the same generalized defense.

**Recommended remediation (raised to SE; cross-domain notify to SO and QE):**

- **SE:** Add a defense-in-depth check inside `wrap_color`. Either:
  - (a) `debug_assert!(!value.chars().any(char::is_control))` — zero release-mode cost, catches refactor mistakes in tests; or
  - (b) `value` is sanitized via the existing `display_safe` (or a stripped-down variant that drops Cc) before interpolation in the `Some(prefix)` branch — release-mode cost is a single iterator pass on a value typically ≤ 11 chars.
  Recommend (a) for the closed-enum present (status/priority) and (b) if `wrap_color` is ever generalized to non-enum values. Document the trust contract at the function's docstring (currently `src/lib.rs:76-78`): "Caller must ensure `value` contains no `char::is_control()` bytes; values from `parse_status` / `parse_priority` satisfy this by closed-enum membership."
- **SO:** No DESIGN.md change required at this layer — the spec already documents the color value vocabulary (DESIGN.md "Interface / color output" L239-250) and the closed-enum status/priority sets (Feature 3, Feature 1). Spec amendment would only be required if a future layer adds a non-enum colored value.
- **QE:** Add a unit test `wrap_color_with_control_char_in_value_is_rejected_in_debug_or_sanitized_in_release` (depending on remediation choice) to lock the contract. Optionally add a property test: for every value in `VALID_STATUSES ∪ PRIORITY_ORDER`, `wrap_color(value, Some(...)).chars().filter(|c| c.is_control()).count() == 2` (exactly the two known-good escape sequences, no smuggled Cc).

**Severity:** Low. The system is not currently exploitable through any documented path (the closed-enum invariants hold at both input and load). The finding is a defense-in-depth gap that becomes exploitable only on a specific class of future refactor. Naming it Open rather than Dismissed because (i) it is a real architectural surface — the new output path has no boundary-local defense — and (ii) the R9 F1 lineage explicitly identified "generalize the defense rather than scope-by-field" as the corrective pattern, and the same lesson applies here (generalize the defense to the rendering-sink boundary, not just the input boundary).

**Classification:** **Open. Raised to SE.** Cannot be Dismissed (the precedent of R9 F1's "generalize the defense" closure rationale applies; declining to apply that rationale to the symmetric output boundary would be inconsistent). Cannot be Accepted-Risk (no named owner; severity is low but the remediation cost is also low — `debug_assert!` is a one-line change). Cannot be Hallucinated (the trust chain is documented above; the unguarded boundary is real, and the load-bearing nature of the input-side controls is verifiable by removing them).

Cross-references:
- [SOFTWARE-ENGINEER-REVIEW.md](SOFTWARE-ENGINEER-REVIEW.md) — `wrap_color` defense-in-depth.
- [QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md) — test fixture for the new contract.

---

**Finding 2 — `NO_COLOR` environment variable convention not honored; TTY-only color suppression is a partial implementation of the de facto standard (Dim 5 — Information exposure; Dim 1 — Output handling)**

`cmd_list` (`src/lib.rs:835`) and `cmd_show` (`src/lib.rs:591`) gate color exclusively on `std::io::stdout().is_terminal()`. The `NO_COLOR` env var convention (https://no-color.org — widely adopted by `ls`, `grep --color`, `git`, `ripgrep, `bat`, `delta`, and the broader CLI ecosystem) is not consulted. A user who sets `NO_COLOR=1` in their environment (the standard opt-out for ANSI emissions even on a TTY) will still see colored output from `tracker list` / `tracker show`.

The Security framing (rather than pure-UX): a user who explicitly opts out of ANSI emission via `NO_COLOR` is signaling *they do not trust their terminal to render ANSI safely* (common reasons: logging pipeline that records ANSI as garbage; accessibility-related terminal emulator with anomalous escape handling; a screen reader that announces escape sequences literally; a pty session forwarded across an environment with mixed ANSI support). Ignoring that signal is information-disclosure-class behavior — the user said "do not write these bytes" and the tool wrote them anyway. The DESIGN.md "Interface / color output" L239 language ("Color is suppressed when stdout is piped or redirected (detect with `std::io::IsTerminal`)") is a *floor*, not a ceiling — it does not prohibit honoring additional opt-out signals.

This is also a **partial sycophancy challenge response to the assignment's "Hardcoded constants, no user-controllable color, TTY-gated" framing**: the framing is technically correct but elides that `is_terminal()` is *not* the only user signal an ANSI-emitting CLI should respect. The "no user-controllable color" claim is exactly what makes this a gap — the user has a documented way to control color output (`NO_COLOR`) that the implementation ignores.

**Recommended remediation (raised to SO; cross-domain notify to SE and UX):**

- **SO:** Decide whether DESIGN.md "Interface / color output" should be amended to require honoring `NO_COLOR` (de facto standard) and/or `CLICOLOR=0` (BSD-era convention; weaker). Recommendation: amend to require `NO_COLOR` honoring (any non-empty value disables color, per the no-color.org spec); leave `CLICOLOR` for a future amendment if user demand surfaces.
- **SE (pending SO):** If sanctioned, extend the TTY gate to `let use_color = std::io::stdout().is_terminal() && std::env::var_os("NO_COLOR").map_or(true, |v| v.is_empty());`. Zero new dependencies. Apply at both `cmd_list:835` and `cmd_show:591`.
- **UX:** This finding has substantial UX overlap; expect independent surfacing there. The Security framing exists because env-var-signaled opt-outs that are ignored are a class of information disclosure ("I told you not to write to me").

**Severity:** Low. The Security framing is real but narrow — the primary blast radius is UX/accessibility, not exploitation. Flagged in Security because the cross-domain notification path is faster from here.

**Classification:** **Open. Raised to SO / Raised to UX.** Cannot be Dismissed without SO adjudicating the spec stance (the current spec is silent on `NO_COLOR`, not explicitly opting out — silence here is itself the finding).

Cross-references:
- [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) — DESIGN.md amendment authority.
- [SOFTWARE-ENGINEER-REVIEW.md](SOFTWARE-ENGINEER-REVIEW.md) — implementation pending SO.
- [UX-REVIEW.md](UX-REVIEW.md) — primary blast radius; independent surfacing expected.

---

### Dismissed

**Finding 3 — TTY detection bypass via hostile pty (Dim 6)**

A pty-spoofing attacker (e.g., a forwarded ssh session writing to the victim's pty) could cause `is_terminal()` to return true on a stdout that is actually being recorded or forwarded. **This is explicitly out of threat model** per DESIGN.md Edge Cases / Labels L319: "the threat surface is bounded to the user attacking themselves with hand-pasted clipboard content or a hand-edited tracker.json. If a future use case widens the threat model (multi-user / network-distributed / shared tracker.json), revisit this stance." The pty-spoofing surface is multi-actor by definition; not in scope. Re-evaluation trigger is named in DESIGN.md and applies symmetrically here. ✓

**Classification:** Dismissed. Out of documented threat model; re-evaluation trigger documented in spec.

---

**Finding 4 — `unknown_subcommand_exits_one` echoes user-supplied subcommand name verbatim to stderr (Dim 5 — Information exposure)**

`tracker <RAW_SUBCMD>` produces `Error: unrecognized subcommand '<RAW_SUBCMD>'` on stderr via clap's default usage error (post Layer-1 `try_parse` transform in `src/main.rs:72-83`). A user who pasted a sensitive token as a subcommand would see it echoed back to stderr (and into any stderr capture file). Verified reproducer: `tracker <TOKEN_PLACEHOLDER>` → stderr `Error: unrecognized subcommand '<TOKEN_PLACEHOLDER>'`. **Out of threat model for a single-user local CLI**: the stderr stream goes to the same user who typed the argument; there is no privilege boundary between the typist and the reader. The token is also already visible in shell history, process listing, and any tty recording. No new exposure surface. Worth-noting only for the future case where `tracker` were invoked in a CI log context or as a sudo'd subprocess — same re-evaluation trigger as Finding 3 (widened threat model). Additionally verified: clap silently strips bare ESC bytes from the echoed argument (raw bytes captured: no ESC in stderr when arg contained a leading `<ESC>[31m`), so the stderr-as-injection-sink risk is mitigated at the framework boundary even though no application-level `display_safe` is applied. ✓

**Classification:** Dismissed. Out of threat model under DESIGN.md "Constraints / Single user"; clap's own escape-stripping provides defense-in-depth at the framework boundary.

---

**Finding 5 — Layer 7 changes alter Trojan-Source / Cf rendering surface for description (Dim 2 — Persistence-to-output trust chain; carry-forward verification of RT R8 F2 Accepted Risk)**

`format_show_block` (`src/lib.rs:526-568`) renders description verbatim (with `\r\n` → `\n` normalization for split, then 13-space continuation indent). Layer 7 added color wrapping to the `status` and `priority` value cells only — **not to the description cell**. Bidi (Cf) carry-forward from RT R8 F2 (Accepted Risk: bidi accepted as printable Unicode) is unchanged: the description rendering pipeline is byte-identical to Layer 6 except for the addition of newline-continuation indent (which was Layer 6 work, not Layer 7). Verified by inspection at `src/lib.rs:532-544`. RT R8 F2 Accepted Risk continues to hold without modification. ✓

**Classification:** Dismissed. No surface change for description rendering; RT R8 F2 Accepted Risk regression-checked and intact.

---

**Finding 6 — Information exposure in `--help` output (Dim 5)**

`tracker --help`, `tracker create --help`, `tracker list --help`, `tracker status --help`, `tracker show --help`, `tracker delete --help` all inspected manually. Output contains: subcommand names, flag names, valid value enumerations, and the doc-comment text from `src/main.rs:11-56`. **No filesystem paths**, no developer identity, no build metadata, no version string (the binary does not implement `--version`; `tracker --version` returns the standard "unexpected argument" error per the post-`try_parse` transform). `tracker.json` is mentioned in the `delete` subcommand doc-comment, which is intentional and spec-sanctioned. No information leak. ✓

**Classification:** Dismissed. `--help` output is spec-compliant and free of identity / build / path metadata.

---

**Finding 7 — Filesystem error messages may leak path information (Dim 5)**

`load_tracker` (`src/lib.rs:333`) interpolates `e` (the `std::io::Error`) into `Could not read tracker data: {}` — and the OS-formatted error for a missing/permission-denied/is-a-directory case could in principle include path information. Verified empirically:

- `tracker.json` is a directory → `Error: Could not read tracker data: Is a directory (os error 21).` (no path)
- `tracker.json` permission denied → `Error: Could not read tracker data: Permission denied (os error 13).` (no path)

Both cases on Darwin emit only the OS-level errno phrase plus the numeric errno; **no path is leaked**. The same call site might leak a path on a different OS or with a different `std::io::Error` source (e.g., a future `canonicalize` call), but at the current call site the leak does not occur. ✓

**Classification:** Dismissed. Empirically verified no path leak at the documented OS targets; the call shape does not interpolate `path` itself.

---

**Finding 8 — TOCTOU on TTY detection (Dim 6)**

`cmd_list` calls `is_terminal()` once at `src/lib.rs:835`; the colored bytes are then written via subsequent `println!` calls. In principle, an attacker could swap stdout between the check and the write. **Hallucinated**: the CLI is single-threaded; there is no concurrent code path that can replace the stdout file descriptor; the threat model has no privileged-vs-unprivileged-process distinction on the user's own process. Not a real concern. ✓

**Classification:** Dismissed (with hallucination note — listed here rather than under Hallucinated for traceability; the concern was raised internally during the review and verified not to apply).

---

**Finding 9 — `cargo audit` regression check (Dim 3)**

`cargo audit --no-fetch` against `Cargo.lock` at HEAD on 2026-05-11: 1069 advisories loaded, 100 crate dependencies scanned, **exit 0, no advisories reported**. `git diff main..HEAD -- Cargo.toml Cargo.lock` is empty — Layer 7 added zero dependencies (raw ANSI escapes; no `anstyle` / `termcolor`). ✓

**Classification:** Dismissed. CVE surface clean; Layer 7 introduced no supply-chain change.

---

### Hallucinated

**Finding 10 — `pad_after_color` arithmetic could be exploited to mis-align columns via a crafted status/priority value with multi-byte UTF-8**

Concern: `pad_after_color` (`src/lib.rs:91-97`) trusts the caller's `visible_chars` argument. A status or priority value with multi-byte UTF-8 (where `chars().count() != bytes`) could cause padding miscalculation.

**Classification:** Hallucinated. The caller (`cmd_list:856` / `cmd_list:859`) passes `issue.status.chars().count()` / `issue.priority.chars().count()` — the correct visible-width measure. Furthermore, the values are constrained by `VALID_STATUSES` / `PRIORITY_ORDER` to pure-ASCII strings (`open`, `in-progress`, `done`, `low`, `medium`, `high`), so the byte count and char count coincide. No exploitation surface exists; the concern does not apply.

---

**Finding 11 — ANSI escape emission to stdout could exfiltrate data via OSC 52 (terminal clipboard write) or OSC 8 (hyperlink leader)**

Concern: a terminal emulator that honors OSC 52 ("set clipboard contents") could be coerced into accepting clipboard-write sequences from `tracker list` / `tracker show` output. OSC 8 hyperlinks could be similarly injected.

**Classification:** Hallucinated. The Layer 7 implementation emits exactly six hardcoded ANSI sequences (`\x1b[1;31m`, `\x1b[33m`, `\x1b[36m`, `\x1b[32m`, `\x1b[0m`, plus the absent-color case). None are OSC. The wrapped value is a closed-enum status or priority (Finding 1 trust chain). No OSC byte can be emitted from any path that does not first violate the closed-enum invariants — and that violation produces a load-time corrupt-data rejection (Finding 1 reproducer). The concern does not apply at the current implementation.

---

### Accepted Risk

**Finding 12 (carried) — Plaintext storage**

Unchanged from Reviews 1 / 5 / 6 / 7 / 8 / 9. Risk owner: the user/developer per DESIGN.md "Constraints / Single user. Local storage only." No new data fields introduced by Layer 7; data-classification posture unchanged ("internal" at most).

---

**Finding 13 (carried) — Trojan-Source / Cf characters in title, label, and description (RT R8 F2)**

Carried Accepted Risk: bidi control characters (Unicode category `Cf`) are accepted as printable Unicode in title, label, and description fields per DESIGN.md Edge Cases / Labels L319-322 and Edge Cases / Description L353. Risk owner: the director (apprentice-program user) per the explicit cross-reference at DESIGN.md L353. Re-evaluation trigger: widened threat model (multi-user / shared tracker.json). Layer 7 introduces no new bidi-rendering surface (description is uncolored; title is uncolored; only the closed-enum status/priority cells are colored, and Cf cannot appear in those). RT R8 F2 Accepted Risk continues to hold without modification.

---

### Summary

Round **11** logged. Cold-session Layer 7 sweep produced **two Open findings** (Finding 1 — `wrap_color` defense-in-depth gap, Raised to SE; Finding 2 — `NO_COLOR` env var not honored, Raised to SO / Raised to UX), **seven Dismissed** (incl. five regression / out-of-threat-model checks and two empirical-verification dismissals), **two Hallucinated**, **two carried Accepted Risk**.

**Top concern.** Finding 1 — `wrap_color` trusts the caller's value-byte hygiene with no boundary-local defense. The new output path is currently safe end-to-end because of the closed-enum invariants at both the input boundary (`parse_status` / `parse_priority`) and the load boundary (`issue_fields_are_valid` calling `VALID_STATUSES.contains` / `PRIORITY_ORDER.contains`). But the *output* boundary itself has no defense — the R9 F1 closure rationale ("generalize the defense") applies symmetrically to the rendering-sink boundary, not just the input boundary. A `debug_assert!(!value.chars().any(char::is_control))` inside `wrap_color` is a one-line zero-runtime-cost change that closes the gap; the alternative (sanitize the value before interpolation) is slightly heavier but generalizes to non-enum values for the future.

**Secondary concern.** Finding 2 — `NO_COLOR` env var convention not honored. The de facto standard is widely adopted; the spec is currently silent rather than explicitly opting out. Primary blast radius is UX/accessibility (cross-domain notification); the Security framing is the "ignored user opt-out is an information-disclosure-class behavior" argument.

**Regression status.** All Layer-1–6 Security controls intact at the source lines walked. `cargo audit` clean. No new dependencies. `cargo build --release` clean. `cargo test` passes per CHANGELOG L24 (195/195 reported by the implementing commit).

**Sycophancy-guard note.** The assignment's framing ("Hardcoded constants, no user-controllable color, TTY-gated") encouraged a clean-pass conclusion. The gap found (Finding 1) is precisely in the trust chain that framing elides: "no user-controllable color" is true only because the input-side closed-enum invariants hold; the output path itself has no defense. The R9 F1 closure rationale predicts this class of finding exactly — "generalize the defense" applied at the input boundary; the same rationale now applies at the output boundary. Finding 2 is a complementary sycophancy challenge: "TTY-gated" describes the implementation but elides the user-signaled opt-out (`NO_COLOR`) that the implementation ignores.

**Coordination:**
- [SOFTWARE-ENGINEER-REVIEW.md](SOFTWARE-ENGINEER-REVIEW.md) — Finding 1 (`wrap_color` defense-in-depth via `debug_assert!` or value sanitization at the rendering-sink boundary); Finding 2 (implementation, pending SO).
- [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) — Finding 2 (DESIGN.md amendment to honor `NO_COLOR`; SO may also wish to evaluate whether Finding 1 warrants a spec note documenting the rendering-sink trust contract).
- [QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md) — Finding 1 test fixture (`wrap_color_with_control_char_in_value_*`).
- [UX-REVIEW.md](UX-REVIEW.md) — Finding 2 primary blast radius; independent surfacing expected (the Security framing is real but narrow).
- [RED-TEAM-REVIEW.md](RED-TEAM-REVIEW.md) — Finding 1 is a defense-in-depth gap rather than an exploit; Red Team may independently surface the same trust-chain analysis as a "what would break this" pressure test, predicting the same future-refactor failure modes.

**Files modified:** Only this review log appended; no source, tests, or DESIGN.md changes applied per CLOSURE-PROTOCOL.md §1 (Security may apply CVE fixes to `src/**` — no CVE fixes triggered this round; Finding 1 is a defense-in-depth recommendation, not a CVE). Attack payloads abstracted (`<ESC>`, `<TOKEN_PLACEHOLDER>`, `<RAW_SUBCMD>`) per primer guidance.

---

## Review 12 — 2026-05-12 00:00Z

**Round:** Security Review 12 (Layer 7 IAR Round 2 closure pass). Warm verification per CLOSURE-PROTOCOL.md §5; not a new adversarial round.

**Scope:** Verify R11 Open findings closed by commit `09b1905`. Inputs: `src/lib.rs` `wrap_color` (with debug_assert!) and `color_mode_from_env` (NO_COLOR / CLICOLOR honoring); DESIGN.md "Interface / Color output" amendment ratifying env-var opt-out; `src/lib.rs#tests` `wrap_color_debug_assert_active_in_debug_builds` test; carry-forward Accepted Risk regression check.

### Round-1 finding closures

- **F1 — `wrap_color` trusts caller's value-byte hygiene; defense-in-depth gap on the new ANSI-emitting code path:** **Resolved by `09b1905`.** `wrap_color` now contains `debug_assert!(!value.chars().any(char::is_control), ...)` at the function entry. The assertion fires on debug builds (i.e., `cargo test`) on any control-character-bearing input, surfacing future refactors that introduce a free-form colored field whose validation was missed. In release builds the assertion is compiled out, so the production cost is zero. Verified by `wrap_color_debug_assert_active_in_debug_builds` unit test: `std::panic::catch_unwind(|| wrap_color("evil\x1b[0m", Some("\x1b[1;31m")))` returns `Err`, confirming the panic on Cc-bearing input.
- **F2 — `NO_COLOR` environment variable convention not honored:** **Resolved by `09b1905`.** DESIGN.md "Interface / Color output" amended to honor `NO_COLOR` (any non-empty value) and `CLICOLOR=0`, with `CLICOLOR_FORCE` deliberately not honored (pipe-cleanness contract takes precedence). `color_mode_from_env()` implements the check chain. Integration test `no_color_env_does_not_break_piped_invocation` verifies env-var passthrough (the test cannot exercise the TTY-positive branch — TTY-positive verification is in the Layer-7 manual checklist re-walk).

### Carry-forward regression verification

- **F13 (Trojan-Source / Cf in title, label, description, RT R8 F2 lineage):** Accepted Risk re-verified intact. Layer 7 Round-2 changes touch presentation only; no new surface for Cf bytes. The `wrap_color` debug_assert! catches *Cc* bytes (controls); Cf (formatting) bytes are not in `char::is_control()` per Rust's Unicode-category definition, so the Accepted Risk posture is preserved.
- **F12 (Plaintext `tracker.json` storage):** Accepted Risk unchanged. No storage-format changes in R2.
- **`cargo audit` regression:** clean (0/1069 advisories, 100 crate deps). Verified at HEAD `09b1905`.
- **Cc-defenses (title L1, labels L4 R7 F1, description L6 R2 / Security R9 F1):** verified intact at `src/lib.rs` `validate_title` / `parse_label` / `validate_description` / `issue_fields_are_valid` — no R2 edits in those code paths.

### Round-2 sweetener (not a closure but worth noting)

- **`sanitize_quoted_values` adds a new defensive layer** at the previously-uncovered stderr write site (clap's argument-parsing pipeline; coordinated with RT R10 F1). The byte-level defense for stderr Cc-escape now extends to clap-reflected values, closing the surface-class drift pattern that RT R9 / R10 named.

### New findings

*(none — closure pass.)*

### Summary

Both R1 Security findings Resolved: F1 by `wrap_color` debug_assert! (defense-in-depth on the output boundary); F2 by `color_mode_from_env` NO_COLOR + CLICOLOR support (UX accessibility opt-out as a security primitive). All carry-forward Accepted Risks re-verified intact. The R2 sweetener of `sanitize_quoted_values` extending Cc-defense to clap's stderr write site is a positive cross-domain outcome — Layer 7 now has Cc-defense at every input boundary (validate_*) AND every stderr output boundary (display_safe / sanitize_quoted_values).

**Coordination:** RT R11 — clap stderr Cc-escape coordinated closure verified (`unknown_subcommand_with_cc_payload_escapes_in_stderr` integration test); UX R11 — NO_COLOR closure cross-checked; QE R18 — `wrap_color` debug_assert! test verified.

**Files modified:** This log appended only. The `wrap_color` debug_assert! and `color_mode_from_env` landed in `09b1905` under SE authority per CLOSURE-PROTOCOL.md §1 (Security has CVE-fix authority on `src/**`; this round's findings were defense-in-depth, not CVE).
