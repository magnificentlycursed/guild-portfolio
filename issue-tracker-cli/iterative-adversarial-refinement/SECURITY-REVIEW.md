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
