# Software Engineer Review Log

This review is part of the [Iterative Adversarial Refinement (IAR)](README.md) suite. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

The purpose of this review is to evaluate implementation quality: correctness, error handling, naming, duplication, and complexity. At pre-implementation stage, the review evaluates DESIGN.md for specification clarity issues that would produce implementation defects.

**Language supplement applied:** `lang/rust.md` (SE section) + `lang/cli.md` (SE section).

---

## Review 1 — 2026-04-27 21:00Z

**Scope:** `DESIGN.md` for specification-level implementation concerns. No source code exists. Pre-implementation pass.

**Session note:** In-session with all other domain reviews. Acknowledged quality tradeoff.

---

### Resolved

**Finding 1 — Stale library reference in Edge Cases/IDs (Dim 1, Dim 10)**

`DESIGN.md` Edge Cases/IDs: "Negative number (`tracker delete -1`) → clap treats `-1` as a flag; command will fail with a usage error."

SO Review 3 (Finding 2) removed all named crate references from the Technology section — specifically because the observable interface contract must be library-agnostic. This reference survived. It also makes a behavioral claim that is clap-specific: not all CLI parsing libraries treat a bare `-1` as a flag name. An implementation using a library that accepts `-1` as a negative integer ID would diverge from this spec note, yet might handle the case correctly in another way (e.g., a value parser that rejects non-positive integers). The spec should describe the required behavior, not the mechanism.

**Resolution:** Updated `DESIGN.md` Edge Cases/IDs to: "the CLI parser treats `-1` as a flag and produces a usage error; the command exits 1." Implementation-agnostic. Done.

---

### Dismissed

**Finding 2 — `updated_at` mutation semantics at creation time are implicitly ambiguous**

DESIGN.md Data Model: "`updated_at` is refreshed on every mutation (status change); equals `created_at` on a freshly created issue." The create operation is not described as a mutation, but `updated_at` must still be set at creation. An implementer reading only "refreshed on every mutation" might not initialize `updated_at` at create time.

**Classification:** Dismissed. The phrase "equals `created_at` on a freshly created issue" explicitly specifies that `updated_at` is set at creation and equals `created_at`. The create postcondition also states: "`created_at` and `updated_at` are set to the current UTC timestamp (ISO 8601, second precision)." Both fields are explicitly set at creation. An implementer reading the spec in full cannot miss this. The Red Gate test `create_timestamps_equal_on_fresh_issue` (added by QE) also catches an implementation that fails to set `updated_at` at creation.

---

**Finding 3 — Description validation: trim-for-validation vs. store-verbatim tension**

DESIGN.md specifies: "Description is not trimmed; stored verbatim." AND "`--description \"\"` (empty or whitespace-only after trim) → Error: Description cannot be empty. → exit 1." These two together require: trim the input for validation only, then store the original untrimmed value if validation passes.

**Classification:** Dismissed. The pattern is consistent with title handling and is explicitly stated in the edge cases section. Both the "not trimmed" and the "checked after trim" clauses are present. An implementation that trims for validation and stores verbatim correctly implements both constraints. The behavior is non-obvious but fully specified. A targeted comment in the implementation at the description validation path is the appropriate mitigation.

---

**Finding 4 — `→` Unicode character in status confirmation message**

DESIGN.md: `stdout prints: "Issue #<id> status → <new_status>."` The right arrow `→` (U+2192) is non-ASCII. Implementations must ensure the output is UTF-8 encoded. On modern terminals this is standard. On legacy or non-UTF-8 terminals the character may render incorrectly.

**Classification:** Dismissed. The character is part of the spec by design. Rust strings are UTF-8. Modern macOS terminals (the target platform) support UTF-8. The behavior on non-UTF-8 terminals is outside the project's stated deployment context. Accepted as a design choice. Cross-referenced in UX log.

---

### Open

*(none)*

---

### Summary

One real finding resolved (stale library reference). Three dismissed with rationale. No open items. Pre-implementation SE review is limited in scope — the primary value will come in Layer 1 review when code exists.

**Coordination:** Finding 2 (`updated_at` at creation) surfaced from QE Finding 4 — both note the `created_at` immutability invariant as an implementation concern. SE will verify in Review 2 that the status-change handler does not touch `created_at`.
