# Security Review Log

This review is part of the [Iterative Adversarial Refinement (IAR)](README.md) suite. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

The purpose of this review is to apply adversarial pressure to find security vulnerabilities, unsafe patterns, validation gaps, and regressions. At pre-implementation stage, the review evaluates the threat model and `DESIGN.md` specification for security posture.

**Language supplement applied:** `lang/rust.md` (Security section).

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

**Finding 3 — File path hardcoded as `tracker.json` in CWD — no path traversal risk (Dim 1, Rust supplement)**

The storage path `tracker.json` in the current working directory is hardcoded in the spec. No user-supplied path component is involved in the file path. Path traversal attacks require user-controlled path segments — none exist here. The file path is entirely implementation-determined.

**Classification:** Hallucinated. There is no path traversal risk when the file path is hardcoded. Confirmed by spec review.

---

**Finding 4 — No authentication or authorization concern (Dim 6)**

The spec explicitly excludes multi-user scenarios. Single-user, local-only, no accounts, no sessions. No auth concern applies.

**Classification:** Hallucinated. The concern does not apply to this deployment context.

---

**Finding 5 — Error messages reveal OS error strings (Dim 5)**

`tracker.json` exists but is not readable (permissions) → stderr `Error: Could not read tracker data: permission denied.` The phrase "permission denied" is the OS error string, potentially revealing filesystem details.

**Classification:** Dismissed. For a single-user local tool, the OS error string is appropriate diagnostic information — the user needs to know it was a permissions problem to take corrective action. There is no confidentiality concern for a personal tool revealing the name of the OS error to its own user. The spec correctly routes this to stderr.

---

**Finding 6 — Dependency audit (Dim 3)**

No `Cargo.toml` exists yet. `cargo audit` cannot run. Deferred.

**Classification:** Deferred to Layer 1 gate, when dependencies are first declared.

---

### Open

*(none)*

---

### Summary

One real finding resolved (post-deserialization validation gap). One accepted risk (plaintext storage). Two dismissed, two deferred. The threat model is well-bounded for this deployment context. The critical finding — treating deserialized file data as trusted — is now specified. The implementation must apply domain validation after deserialization, not only after JSON parsing.

**Coordination:** Finding 1 is cross-referenced in Data Engineer log (schema validation) and Red Team log (crafted file attack). `cargo audit` deferred to Platform Engineer for CI gate setup.

---

---

## Review 2 — 2026-04-27 22:00Z

**Scope:** Layer 1 stub — `Cargo.toml`, `src/main.rs`, `src/lib.rs`, `tests/layer1.rs`. Evaluating security posture of the dependency declaration and stub code. No behavioral implementation exists.

**Session note:** In-session with all other Layer 1 domain reviews. Acknowledged quality tradeoff.

---

### Resolved

**Finding 6 (from Review 1) — `cargo audit` deferred because no Cargo.toml existed**

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

**Resolution:** Partially resolved. `cargo audit` is now runnable. The deferred finding is resolved for the stub phase. When runtime dependencies are declared during Layer 1 implementation, `cargo audit` must pass before the Layer 1 merge gate closes — this is already enforced by the CI pipeline.

---

### Dismissed

**Finding 7 — Dev-dependencies included in test binaries (Dim 3)**

`assert_cmd`, `predicates`, `serde_json`, `tempfile` are in `[dev-dependencies]`. They are not compiled into the production binary.

**Classification:** Hallucinated. Dev-dependencies are a Cargo mechanism for test-only dependencies — they produce no additional attack surface in the shipped binary. `cargo build --release` does not compile dev-dependencies. The CI pipeline already runs `cargo build` (not `cargo test`) to produce the release binary. No concern.

---

### Open

*(none)*

---

### Summary

Review 1 Finding 6 resolved: `Cargo.toml` exists; `cargo audit` is now runnable on zero runtime dependencies and will pass. Full audit effectiveness contingent on runtime dependencies being declared during Layer 1 implementation. CI enforces `cargo audit` on every push. No new security findings in stub code.

---

---

## Review 3 — 2026-04-28 05:30Z

**Scope:** Layer 1 implementation — `src/lib.rs`, `src/main.rs`, `Cargo.toml`, `tests/layer1.rs`. Evaluating implementation security posture: input handling, file I/O, dependency audit, and post-deserialization validation from Review 1's findings.

**Session note:** In-session with Layer 1 IAR suite. Acknowledged quality tradeoff. This is the implementation gate review; QE Review 3 (cold-session) satisfies the merge-gate cold-session requirement separately.

**Adversarial posture (review-session primer applied):** I am reading this code as an attacker looking for inputs that produce undefined behavior, panics, or silent data corruption. The small attack surface is a conclusion to earn, not an assumption to start from.

---

### Resolved

**Finding 1 — Post-deserialization validation not implemented (Dim 2 — Input validation)**

Security Review 1, Finding 1 specified that `tracker.json` data must be treated as untrusted: semantically invalid field values in structurally-valid JSON must trigger the corrupt-data error path. DESIGN.md Storage edge cases explicitly states: "`tracker.json` contains valid JSON but invalid domain values (e.g., `"status": "flying"`, `"priority": ""`, `"id": 0`, `"title": ""`) → stderr `Error: Could not read tracker data. The file may be corrupt. Delete tracker.json to start fresh.` → exit 1."

The Layer 1 implementation's `load_issues` in `lib.rs` called `serde_json::from_str::<Vec<Issue>>()` without any post-deserialization domain validation. A crafted `tracker.json` with `"status": "flying"` would deserialize successfully into the `Issue` struct (all fields are `String` typed), and `cmd_list` would then operate on the invalid data — silently sorting an issue with an unknown priority to the bottom of the list (via `usize::MAX` in `priority_rank`), hiding the corrupt record from the user rather than reporting the corruption.

**Resolution:** Added `issue_fields_are_valid()` validation function and `CORRUPT_DATA_ERROR` constant to `lib.rs`. `load_issues` now validates each deserialized issue against domain constraints (positive ID, non-empty title after trim, valid status, valid priority) and returns `Err(CORRUPT_DATA_ERROR)` if any issue fails validation. Cross-referenced: Data Engineer Review 3, Red Team Review 2, QE Review 4.

---

### Accepted Risk

**Finding 2 (Review 1, re-evaluated) — Plaintext storage**

Unchanged from Review 1 assessment. Accepted.

---

### Dismissed

**Finding 3 — `cargo audit` passes with 0 advisories (Dim 3 — Dependency audit)**

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

One real finding resolved: post-deserialization domain validation was absent and is now implemented. `cargo audit` passes with 0 advisories. No panic surface on user-facing paths. The attack surface remains extremely small: single hardcoded file path, no network, no auth, all user input validated at the CLI boundary. The implementation now treats both structurally-malformed and semantically-invalid file data as corrupt, as required by the spec.

**Coordination:** Finding 1 resolved jointly with Data Engineer Review 3 (domain validation) and Red Team Review 2 (crafted-file attack). QE Review 4 added the corresponding test (`invalid_domain_values_in_json_causes_error_exit`).
