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

---

---

## Review 2 — 2026-04-27 22:00Z

**Scope:** Layer 1 stub code — `src/main.rs`, `src/lib.rs`, `tests/layer1.rs`, `Cargo.toml`. Evaluating implementation quality, API boundaries, naming, and correctness of the stub structure. No behavioral implementation exists.

**Session note:** In-session with all other Layer 1 domain reviews. Acknowledged quality tradeoff.

---

### Dismissed

**Finding 1 — `validate_title` and `next_id` parameter names produce unused-variable warnings (Dim 5)**

Both stub functions name their parameters (`raw`, `existing_ids`) but `todo!()` bodies never reference them, producing `unused variable` warnings from the compiler.

**Classification:** Dismissed. This is the expected and correct behavior for Red Gate stubs. Prefixing with `_` (`_raw`, `_existing_ids`) would suppress the warnings but would also make the stub signatures less readable as documentation of what the function will use. The warnings are intentional signals that the stubs are not yet implemented. They will resolve naturally when the function bodies are written. Suppressing them with `_` prefix would obscure this signal.

---

**Finding 2 — `validate_title` error type is `String`; `Result<String, String>` is stringly typed (Dim 3)**

The function signature uses `String` for both the success and error types. An `Err` value is indistinguishable from an `Ok` value at the type level except by which arm they're in. A dedicated error enum would allow callers to match on specific error variants.

**Classification:** Dismissed. For a Phase 1 learning project, `Result<String, String>` is appropriate. The error is a human-readable message rendered to stderr — no caller needs to branch on error variants. A dedicated error type would be correct production practice but is out of scope for the assignment's learning objectives. If the tool grows beyond Phase 1, the error type is the first thing to refactor. Cross-referenced in SA log.

---

**Finding 3 — `next_id` takes `&[u64]` (slice of IDs) rather than `&[Issue]` (slice of issues) (Dim 6)**

Passing extracted IDs rather than the full issue slice means the caller must extract IDs before calling `next_id`. This is a minor API clarity question.

**Classification:** Dismissed. A function that computes `max(ids) + 1` has no reason to know about the `Issue` type. Taking `&[u64]` keeps `next_id` a pure arithmetic function with no coupling to the domain model. The caller extracts IDs with `.iter().map(|i| i.id).collect()` or equivalent — a one-liner. The narrow API is correct.

---

**Finding 4 — `src/main.rs` is `fn main() {}` with no argument parsing stub (Dim 1)**

The binary entry point is completely empty. Any invocation of the binary exits 0 with no output.

**Classification:** Dismissed. This is the correct Red Gate stub for a binary that has not yet been implemented. The empty `main()` produces the expected Red Gate behavior: integration tests that assert exit codes fail because the binary exits 0 for everything; tests that assert stdout content fail because stdout is empty. The Red Gate is working correctly.

---

**Finding 5 — Unit test `id_assignment_increments_from_max` passes an unsorted slice (Dim 1)**

`next_id(&[1, 3, 5])` passes a sorted slice. The test would also pass if the implementation used `ids[ids.len() - 1] + 1` (last element) rather than `ids.iter().max()`. A test with an unsorted slice (`&[5, 1, 3]`) would distinguish these implementations.

**Classification:** Dismissed. The test correctly specifies `max(existing_ids) + 1`. Whether the implementation uses `.iter().max()` or a sort-and-take-last approach is an internal detail. The spec says "max" — any implementation that produces the correct answer for sorted and unsorted inputs satisfies the contract. Adding an unsorted-slice test variant would over-specify the test. The implementation should use `.iter().max()` — this is a natural Rust idiom and the test is sufficient.

---

### Open

*(none)*

---

### Summary

Five dismissed findings, all hallucinated or style-level concerns. The stub structure is correct. Library/binary split is appropriate. Public API boundary is minimal and correct. `todo!()` is the right Red Gate mechanism. No real findings.

**Deferred to SE Review 3 (Layer 1 implementation):** Verify that the status-change handler does not touch `created_at` (QE Finding 4 / SE Review 1 Finding 2 coordination note). This cannot be verified until implementation code exists.
