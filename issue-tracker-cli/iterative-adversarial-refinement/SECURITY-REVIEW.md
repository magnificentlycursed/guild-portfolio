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
