# Red Team Review Log

This review is part of the [Iterative Adversarial Refinement (IAR)](README.md) suite. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

**Reviewer role: Red Team Hacker** (Penetration Tester / Offensive Security Engineer)

**Activation:** User-controlled CLI input is present. Run after Security Engineer.

**Language supplement applied:** `lang/rust.md` (Red Team section).

---

## Review 1 — 2026-04-27 21:00Z

**Scope:** `DESIGN.md` attack surface review. No source code. Post-Security review pass.

**Session note:** In-session, post-Security review. Acknowledged quality tradeoff.

**Posture:** I am looking for ways to make the tool behave incorrectly, panic, corrupt data, or reveal information it should not. The threat model (single-user local CLI, no network, no auth) significantly constrains the attack surface — but that assessment must be earned, not assumed.

---

### Accepted Risk

**Finding 1 — Crafted `tracker.json` with invalid domain values (cross-reference Security Finding 1)**

A malicious or accidentally crafted `tracker.json` with valid JSON structure but invalid domain values (e.g., `"status": "flying"`, `"id": -1` encoded as a large u64 near `u64::MAX`, `"title": ""`) could produce undefined behavior in an implementation that trusts deserialized data.

For a single-user local tool, the scenario is: the user manually edits `tracker.json` and introduces an invalid value. The question is whether the tool crashes, silently uses the invalid data, or errors cleanly.

**Security Review 1 resolved this** by adding the post-deserialization validation requirement to DESIGN.md. Invalid domain values in deserialized data now trigger the corrupt-data error path (exit 1, informative stderr message).

**Classification:** Resolved by Security Finding 1. The spec now requires post-deserialization validation. The Red Team confirms this is the correct mitigation — the tool should treat all file-read data as untrusted.

---

### Dismissed

**Finding 2 — Integer overflow on ID counter at `u64::MAX` (Rust supplement — integer overflow)**

ID assignment: `max(existing_ids) + 1`. If `max(existing_ids)` equals `u64::MAX` (18,446,744,073,709,551,615), the `+ 1` wraps to 0 in Rust release builds (overflow is defined to wrap for release mode arithmetic, unlike debug mode which panics).

**Classification:** Accepted risk. A personal issue tracker would need to create 18.4 quintillion issues to reach this condition. This is not a real threat. Accepted without mitigation. If the implementation uses `checked_add` this is a free mitigation; if it uses `+`, the overflow risk is categorically acceptable.

---

**Finding 3 — Path traversal via `tracker.json` (Rust supplement — path traversal)**

The file path `tracker.json` is hardcoded in the implementation. No user-supplied path component is involved. There is no path traversal surface.

**Classification:** Hallucinated. Path traversal requires user-controlled path components. The spec contains none.

---

**Finding 4 — Panic via crafted CLI input (Rust supplement — panic as DoS)**

`.unwrap()` on values derived from user input can panic and crash the binary. For a single-user CLI, a crash is inconvenient but not a denial-of-service concern — the user is attacking themselves. The Rust supplement notes this is a DoS vector for server applications; for a CLI the stakes are much lower.

**Classification:** Deferred. No implementation exists. The Rust SE supplement requires `.unwrap()` discipline on user-facing paths. SE Review will verify when code exists. For a CLI tool, a panic from user input is a quality defect (unhelpful error message) not a security vulnerability. The DESIGN.md Constraints section requires crash-safe I/O for file operations specifically. No spec change needed.

---

**Finding 5 — Supply chain attack on Cargo dependencies (Rust supplement — crates.io supply chain)**

No `Cargo.toml` exists. No dependencies declared. Deferred to when dependencies are first introduced.

**Classification:** Deferred to Layer 1, when dependencies exist. Platform Engineer findings (cargo audit, cargo deny) cover this.

---

### Open

*(none)*

---

### Summary

The attack surface of this tool is extremely small: hardcoded file path, no network, no auth, single user, all input validated at the CLI boundary. The one real pre-implementation finding (crafted `tracker.json` with invalid domain values) was already resolved by Security Review 1. Two deferred findings (panic discipline, supply chain) will be evaluated in Review 2 when code exists. Two hallucinated. One accepted risk (u64 overflow — not a real threat).

Maximum viable refinement is close for the pre-implementation phase. The implementation-phase review will focus on `.unwrap()` discipline and dependency audit.

---

---

## Review 2 — 2026-04-28 05:30Z

**Scope:** Layer 1 implementation — `src/lib.rs`, `src/main.rs`, `Cargo.lock`. Adversarial posture: attempting to crash, corrupt, or cause undefined behavior through crafted inputs and files.

**Session note:** In-session with Layer 1 IAR suite. Acknowledged quality tradeoff.

**Posture:** I am looking for ways to break this binary. Small attack surface is a conclusion I must earn.

---

### Resolved

**Finding 4 (Review 1) — Panic via crafted CLI input (deferred from Review 1)**

Review 1 deferred verification of `.unwrap()` discipline to when implementation code exists.

Audit of all `.unwrap()` calls in `lib.rs` and `main.rs`:

- `lib.rs:49` — `serde_json::to_string_pretty(issues).unwrap()`: serialization of `Vec<Issue>` with known-serializable field types. Cannot fail. Accepted.
- `lib.rs` — `?` propagation on all user-facing paths (file read, JSON parse, domain validation). No unwrap on any code path reachable from user input or file content.
- `main.rs` — `Cli::parse()`: clap handles all argument parsing failures with its own error output and exit 1. No unwrap.

No user-reachable panic surface found.

**Classification:** Resolved. No unsafe unwrap on user-facing paths. Finding 4 discharged.

---

**Finding 5 (Review 1) — Supply chain attack on Cargo dependencies (deferred from Review 1)**

`cargo audit` run against `Cargo.lock` (100 packages: serde, serde_json, clap, chrono and all transitive deps): **0 advisories**. No known vulnerabilities in the dependency tree.

**Classification:** Resolved. Finding 5 discharged.

---

### Accepted Risk

**Finding 1 (Review 1) — Crafted `tracker.json` with invalid domain values**

Re-evaluated against the implementation. The initial implementation was vulnerable: `load_issues` deserialized without domain validation, allowing a crafted `tracker.json` with `"status": "flying"` to be silently processed. Security Review 3 / Data Engineer Review 3 identified and resolved this gap — `load_issues` now validates all field domain values and rejects any issue with an invalid value via the corrupt-data error path.

**Classification:** Resolved (via Security Review 3). The mitigated path now exits 1 with an informative message for any crafted file with invalid domain values.

---

### Dismissed

**Finding 2 (Review 1, re-evaluated) — Integer overflow on ID counter**

Unchanged assessment. `u64::MAX` + 1 overflow is unreachable in any realistic use. Accepted risk.

---

**Finding 3 (Review 1) — Path traversal**

Hallucinated. File path is hardcoded. Unchanged assessment.

---

**New Finding 1 — Zero-id crafted file**

A `tracker.json` with `"id": 0` is now caught by the `issue.id > 0` check in `issue_fields_are_valid()`. Exit 1, corrupt-data error. Verified by reading the validation logic. Test `invalid_domain_values_in_json_causes_error_exit` covers the adjacent case (`"status": "flying"`); zero-ID is structurally identical.

**Classification:** Dismissed — handled by the post-deserialization validation. No additional test required at Layer 1; the validation path is the same for any failing field.

---

### Open

*(none)*

---

### Summary

Both deferred findings from Review 1 resolved: `.unwrap()` discipline verified (no panic surface on user-facing paths); supply chain audit clean (0 advisories). The crafted-file attack path (Finding 1) is now mitigated by post-deserialization validation. The tool's attack surface is as small as its deployment context warrants. Maximum viable refinement reached for Layer 1.
