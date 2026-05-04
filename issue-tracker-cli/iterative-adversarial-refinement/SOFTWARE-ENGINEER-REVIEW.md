# Software Engineer Review Log

This review is part of the [Iterative Adversarial Refinement (IAR)](README.md) suite. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

**Reviewer role: Software Engineer** (Software Engineer / Backend Engineer / Frontend Engineer)

The purpose of this review is to evaluate implementation quality: correctness, error handling, naming, duplication, and complexity. The review evaluates source code against DESIGN.md, and at pre-implementation stage evaluates DESIGN.md itself for specification clarity issues that would produce implementation defects.

**Language supplement applied:** `lang/rust.md` (SE section) + `lang/cli.md` (SE section).

**Sycophancy check:** An agent that designed and implemented the code will find the implementation correct because it reflects its own intent. Push hardest on dim 1 (correctness) and dim 8 (defensive coding): these are the dimensions where implementation intent and spec requirement diverge most often. For every function, ask: "is this doing what was specified, or is it doing what was generated?" They are not the same thing. Flag any function where the implementation could be correct internally but wrong with respect to the spec without any test catching it.

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

**Finding 2 — `updated_at` mutation semantics at creation time are implicitly ambiguous (Dim 1)**

DESIGN.md Data Model: "`updated_at` is refreshed on every mutation (status change); equals `created_at` on a freshly created issue." The create operation is not described as a mutation, but `updated_at` must still be set at creation. An implementer reading only "refreshed on every mutation" might not initialize `updated_at` at create time.

**Classification:** Dismissed. The phrase "equals `created_at` on a freshly created issue" explicitly specifies that `updated_at` is set at creation and equals `created_at`. The create postcondition also states: "`created_at` and `updated_at` are set to the current UTC timestamp (ISO 8601, second precision)." Both fields are explicitly set at creation. An implementer reading the spec in full cannot miss this. The Red Gate test `create_timestamps_equal_on_fresh_issue` (added by QE) also catches an implementation that fails to set `updated_at` at creation.

---

**Finding 3 — Description validation: trim-for-validation vs. store-verbatim tension (Dim 1)**

DESIGN.md specifies: "Description is not trimmed; stored verbatim." AND "`--description \"\"` (empty or whitespace-only after trim) → Error: Description cannot be empty. → exit 1." These two together require: trim the input for validation only, then store the original untrimmed value if validation passes.

**Classification:** Dismissed. The pattern is consistent with title handling and is explicitly stated in the edge cases section. Both the "not trimmed" and the "checked after trim" clauses are present. An implementation that trims for validation and stores verbatim correctly implements both constraints. The behavior is non-obvious but fully specified. A targeted comment in the implementation at the description validation path is the appropriate mitigation.

---

**Finding 4 — `→` Unicode character in status confirmation message (Dim 1)**

DESIGN.md: `stdout prints: "Issue #<id> status → <new_status>."` The right arrow `→` (U+2192) is non-ASCII. Implementations must ensure the output is UTF-8 encoded. On modern terminals this is standard. On legacy or non-UTF-8 terminals the character may render incorrectly.

**Classification:** Dismissed. The character is part of the spec by design. Rust strings are UTF-8. Modern macOS terminals (the target platform) support UTF-8. The behavior on non-UTF-8 terminals is outside the project's stated deployment context. Accepted as a design choice. Cross-referenced in UX log.

---

### Open

*(none)*

---

### Summary

One real finding resolved (stale library reference). Three dismissed with rationale. No open items. Pre-implementation SE review is limited in scope — the primary value will come in Layer 1 review when code exists.

**Coordination:** Finding 2 (`updated_at` at creation) surfaced from [QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md) Review 1 Finding 4 — both note the `created_at` immutability invariant as an implementation concern. SE will verify in Review 2 that the status-change handler does not touch `created_at`.

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

Five dismissed findings, all hallucinated or style-level concerns. The stub structure is correct. Library/binary split is appropriate. Public API boundary is minimal and correct. `todo!()` is the right Red Gate mechanism. No real findings. Deferred to SE Review 3 (Layer 1 implementation): verify that the status-change handler does not touch `created_at` ([QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md) Review 1 Finding 4 / SE Review 1 Finding 2 coordination note). This cannot be verified until implementation code exists.

**Coordination:** *(none)*

---

---

## Review 3 — 2026-04-28 05:07Z

**Scope:** Layer 1 implementation — `src/lib.rs`, `src/main.rs`, `Cargo.toml`, `tests/layer1.rs`. Evaluating implementation correctness, error handling, naming, and structural quality against the Layer 1 acceptance criteria.

**Session note:** Cold-session review. No other domain reviews in-session. Implementation claims all 17 Red Gate tests passing.

---

### Resolved

*(none)*

---

### Dismissed

**Finding 1 — `save_issues` uses `.unwrap()` on serialization (Dim 5)**

`lib.rs:49`: `serde_json::to_string_pretty(issues).unwrap()`. Serialization of `Vec<Issue>` with serde cannot fail in practice — all field types are serializable and the struct derives `Serialize`. The unwrap is a naked panic site with no diagnostic message.

**Classification:** Dismissed. The unwrap cannot be triggered by any reachable input: `Vec<Issue>` serialization with serde_json is infallible for this struct shape. Promoting this to a `Result`-returning path would require changing the return type of `save_issues` or the call sites without any real safety gain. A follow-up note: if this function is ever extended with field types that can fail serialization (e.g., custom `Serialize` impls), the unwrap should be replaced with `.expect("BUG: issue list serialization failed")` at minimum. Acceptable for Layer 1.

---

**Finding 2 — Hardcoded `"tracker.json"` CWD-relative path in `main.rs` (Dim 1)**

`main.rs:23`: `Path::new("tracker.json")`. The storage file is resolved relative to the current working directory at invocation time. Running the binary from different directories produces different, invisible data files.

**Classification:** Dismissed. This is consistent with DESIGN.md's storage spec, which does not specify a fixed path. CWD-relative is the correct behavior for a project-scoped issue tracker — analogous to how `git` looks for `.git` in or above CWD. The manual testing checklist also assumes CWD-relative behavior. No action needed at Layer 1; Layer 7 polish is the appropriate place to revisit if a `$HOME`-based default or `--data-file` override is ever wanted.

---

**Finding 3 — `truncate_with_ellipsis` allocates `Vec<char>` on every call (Dim 7)**

`lib.rs:84`: `let chars: Vec<char> = s.chars().collect()` allocates a heap vector to compute a truncation point. Called twice per list row (title + labels). For the dataset sizes this tool will encounter, the allocation is immaterial.

**Classification:** Dismissed. CLI tools rendering a human-readable list are not performance-sensitive. The `Vec<char>` approach is readable and correct. Using `s.char_indices()` with an early-exit iterator would be more efficient but would also make the function harder to read for no practical gain. Acceptable.

---

**Finding 4 — `priority_rank` returns `usize::MAX` for unrecognized priority values (Dim 5)**

`lib.rs:80`: unrecognized priority strings sort silently to the bottom of the list. An issue with a corrupt or future priority value produces no warning.

**Classification:** Dismissed. Layer 1 does not accept user-supplied priority values — every issue is created with the hardcoded default `"medium"`. The only way an unrecognized priority reaches `priority_rank` at Layer 1 is from a manually edited `tracker.json`. Silently sorting to the bottom is a safe, non-destructive fallback. Layer 3 adds priority parsing with validation; at that point, all new writes are validated and `usize::MAX` remains a correct defensive backstop for externally modified files. Acceptable.

---

**Finding 5 — `created_at` is not touched by any mutation path (SE Review 1 / QE coordination deferred item)**

SE Review 1 coordinated with QE Review: verify at implementation time that the status-change handler does not touch `created_at`. Layer 1 has no status-change handler. In `cmd_create`, `created_at` and `updated_at` are both set once from a single `current_timestamp()` call and cloned — they are equal at creation and `created_at` is never written again by any function in `lib.rs`. The invariant is structurally enforced by the absence of any code path that modifies `created_at` after construction.

**Classification:** Dismissed. The invariant holds. The coordination note is discharged. When the Layer 2 status-change handler is implemented, SE Review 4 must verify the same invariant for the mutation path.

---

### Open

*(none)*

---

### Summary

Five dismissed findings. No real defects. The implementation is correct and complete for Layer 1: all acceptance criteria are met by the code as written, the 17 Red Gate tests (13 integration + 4 unit) cover the specified behaviors, and no structural issues carry forward. Deferred to SE Review 4 (Layer 2 implementation): verify that the status-change handler does not write to `created_at` ([QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md) Review 1 Finding 4 coordination, SE Review 1 Finding 2, SE Review 3 Finding 5 discharge note).

**Coordination:** *(none)*

---

---

## Review 4 — 2026-04-28 05:30Z

**Scope:** Post-Security-finding changes to `lib.rs` — specifically the addition of `CORRUPT_DATA_ERROR`, `VALID_STATUSES`, `VALID_PRIORITIES`, `issue_fields_are_valid()`, and the updated `load_issues`. Evaluating correctness, naming, and structural quality of the additions.

**Session note:** In-session with Layer 1 IAR suite. Acknowledged quality tradeoff.

---

### Resolved

*(none)*

---

### Dismissed

**Finding 1 — `CORRUPT_DATA_ERROR` constant is `&str`; two code paths share it (Dim 5)**

`lib.rs` defines `const CORRUPT_DATA_ERROR: &str = "Could not read tracker data. ..."` and uses it in both the JSON parse failure path and the domain validation failure path. The two code paths share an identical message.

Observation: the two error paths are not identical in cause — parse failure means the file is not JSON; domain validation failure means the file is JSON but contains invalid values. A more precise UX might distinguish them. However, DESIGN.md specifies the same error message for both cases, and the message is user-actionable ("Delete tracker.json to start fresh."). The user needs one action for both conditions. Deduplication is correct.

**Classification:** Dismissed. The constant is appropriate. No action required.

---

**Finding 2 — `VALID_STATUSES` and `VALID_PRIORITIES` constants are module-level but not `pub` (Dim 6)**

The constants are declared at module level but are not public. They are used only by `issue_fields_are_valid()`. In a future layer, status and priority parsing will also need these values. If the parsing functions are defined in the same module, the constants will be shared. If they are defined in a separate module, the constants will need to be exported.

**Classification:** Dismissed. At Layer 1, only one consumer exists (`issue_fields_are_valid`). The constants are not exported because no external consumer needs them yet. Promoting to `pub` now would be anticipatory exposure. The Layer 3 implementation of priority parsing and Layer 2 status parsing should either reuse these constants (if in the same module) or define their own enum-based representations. No action required at Layer 1.

---

**Finding 3 — `issue_fields_are_valid` validates `title.trim().is_empty()` rather than `title.is_empty()` (Dim 1)**

The validation uses `!issue.title.trim().is_empty()` — checking for whitespace-only stored titles. A title written by this implementation would always be non-whitespace-only (it was validated by `validate_title` before storage). However, a manually-edited `tracker.json` with `"title": "   "` would fail this check, consistent with DESIGN.md's intent for the corrupt-data path.

**Classification:** Dismissed. The behavior is correct: stored titles may be manually edited, and a whitespace-only stored title violates the domain invariant. Using `.trim()` here is the right defensive check.

---

**Finding 4 — SE Review 1/3 deferred item: `created_at` immutability**

Layer 1 has no status-change handler. No mutation path exists that could touch `created_at`. The invariant is structurally enforced by the absence of any code that modifies `created_at` after construction (same conclusion as SE Review 3 Finding 5). The deferred verification for the status-change handler remains deferred to SE Review 5 (Layer 2 implementation), when the handler will exist.

**Classification:** Dismissed. No new evidence; finding remains deferred to Layer 2. Note: deferred to SE Review **5** (correcting earlier note which said "SE Review 4" — that review is this one).

---

### Open

*(none)*

---

### Summary

No real findings in the added validation code. The `CORRUPT_DATA_ERROR` constant deduplication is correct. The `VALID_STATUSES`/`VALID_PRIORITIES` constants are appropriately scoped for Layer 1. The `issue_fields_are_valid()` validation is correct and defensive. `created_at` immutability remains deferred to the Layer 2 status-change implementation. SE Review 5 takes that deferred item: verify that the status-change handler does not write to `created_at`.

**Coordination:** *(none)*

---

---

## Review 5 — 2026-04-30 00:00Z

**Scope:** Layer 1 gate closure pass — general adversarial pass. No code changes since Review 4 except test assertion added (`(none)` in `list_shows_header_and_issues`). Reviewing test change and deferred item status.

**Session note:** In-session with all other domain reviews. Acknowledged quality tradeoff.

---

### Dismissed

**Finding 1 — Test assertion added (`(none)` in Labels column) (Dim 1)**

The assertion `assert!(out.contains("(none)"))` added to `list_shows_header_and_issues` is correct. `cmd_list` renders `"(none)"` when `issue.labels.is_empty()` (lib.rs:129–131). The assertion validates the correct branch.

**Classification:** Dismissed. No defect — test change is correct.

---

**Finding 2 — Deferred item (`created_at` immutability) (Dim 1)**

No status-change handler exists at Layer 1. Deferred item remains at SE Review 5 (Layer 2).

**Classification:** Dismissed at Layer 1; deferred to Layer 2 implementation.

---

### Open

*(none)*

---

### Summary

No SE findings. MVR reached for Layer 1.

**Coordination:** *(none)*

---

---

## Review 6 — 2026-04-30 00:00Z

**Scope:** General adversarial review, pre-merge gate. Review-session primer loaded. Applying Rust SE supplement. Assumption surfacing on library behavior.

**Session note:** In-session review. Acknowledged quality tradeoff.

---

### Resolved

**Finding 1 — No `#![deny(clippy::unwrap_used)]` at crate level (Rust supplement — Clippy lint configuration)**

The Rust SE supplement specifies the standard deny set includes `clippy::unwrap_used`. The CI runs `cargo clippy -- -D warnings` which denies all default-warning lints, but `clippy::unwrap_used` is not a default-warning lint — it requires explicit opt-in. The `.unwrap()` in `save_issues` (`lib.rs`) has its safety documented only in the IAR review logs, not at the call site. A future developer introducing a second `.unwrap()` on a user-facing path would face no CI enforcement.

**Resolution:** Added `#![deny(clippy::unwrap_used)]` to `lib.rs` line 1. Added `#[allow(clippy::unwrap_used)]` with an inline safety comment on the `serde_json::to_string_pretty` call in `save_issues`. Fixed the unit test `title_trimmed_before_storage` to use `assert_eq!(validate_title(...), Ok(...))` instead of `.unwrap()`. Clippy clean verified.

---

### Dismissed

**Finding 2 — `PRIORITY_ORDER` constant position (Dim 10)**

The constant at `lib.rs:90` is defined between `cmd_create` and `priority_rank`. In Rust, constant position does not affect visibility or compilation.

**Classification:** Dismissed. Style preference, not a defect.

---

**Finding 3 — `truncate_with_ellipsis` underflow risk for `max_chars = 0` (Dim 8)**

The function subtracts 1 from `max_chars` (usize), which would wrap to `usize::MAX` if `max_chars = 0`. However, the function is private and is only called with hardcoded constants `50` and `20`.

**Classification:** Dismissed. The risk is hypothetical for the current call sites and unreachable in practice.

---

### Open

*(none)*

---

### Summary

One finding resolved: `#![deny(clippy::unwrap_used)]` added to enforce inline safety documentation on any future `.unwrap()` use. The single existing `.unwrap()` is annotated with an inline safety rationale. All dismissed findings are structural observations with no defect implications.

**Coordination:** *(none)*

---

---

## Review 7 — 2026-05-01 00:00Z

**Scope:** Layer 2 implementation — `src/lib.rs`, `src/main.rs`. Evaluating correctness, naming, error handling, and structural quality of Layer 2 additions. Deferred item from SE Review 4/5: verify that the status-change handler does not write to `created_at`. Verified: `cmd_status` mutates only `issue.status` and `issue.updated_at`. `issue.created_at` is not referenced. Deferred item discharged.

**Session note:** In-session with full Layer 2 IAR suite. Acknowledged quality tradeoff. Review-session primer applied.

---

### Resolved

**Finding 1 — `cmd_status` uses `new_status.clone()` unnecessarily (Dim 7 — Unnecessary clone)**

`lib.rs`:
```rust
issue.status = new_status.clone();
issue.updated_at = current_timestamp();
save_issues(issues_path, &issues)?;
println!("Issue #{} status \u{2192} {}.", id, new_status);
```

`new_status` is cloned into `issue.status` (line 1) and then the original is used in `println!` (line 4). This allocates a second `String` unnecessarily.

Naively replacing the clone with a move (`issue.status = new_status`) and reading `issue.status` in the `println!` caused a borrow conflict: the `&mut Issue` returned by `iter_mut().find()` holds a mutable borrow of `issues` that NLL extends through the `println!` line, conflicting with the immutable borrow `save_issues(issues_path, &issues)` requires.

The correct fix replaces `iter_mut().find()` with `iter().position()`, returning a `usize` index instead of a `&mut Issue`. The index carries no borrow, so all three operations are borrow-conflict-free:

```rust
let idx = issues
    .iter()
    .position(|i| i.id == id)
    .ok_or_else(|| format!("Issue #{} not found.", id))?;
issues[idx].status = new_status;
issues[idx].updated_at = current_timestamp();
save_issues(issues_path, &issues)?;
println!("Issue #{} status \u{2192} {}.", id, issues[idx].status);
```

`new_status` is moved into `issues[idx].status` (zero clones). `save_issues` takes an immutable borrow of `issues` after the index-based mutations complete. `println!` takes another immutable borrow after `save_issues` returns — no conflict.

`clippy::needless_clone` does not catch the original case because `new_status` is used after the clone point, but the clone is unnecessary given the correct restructuring.

**Resolution:** Refactored `cmd_status` from `iter_mut().find()` to `iter().position()`. `new_status` moved directly into `issues[idx].status`. `println!` reads `issues[idx].status`. Zero clones, no borrow conflict. `#![deny(clippy::unwrap_used)]` is unaffected. All 41 tests pass.

---

**Finding 2 — `parse_status` called `raw.to_lowercase()` twice (Dim 7 — Redundant allocation)**

Before SA Review 6 fix:
```rust
match raw.to_lowercase().as_str() {
    "open" | "in-progress" | "done" => Ok(raw.to_lowercase()),
    ...
}
```
`raw.to_lowercase()` allocated two `String`s for the common (success) path.

**Classification:** Resolved by SA Review 6 Finding 1 — `parse_status` now uses `VALID_STATUSES` with a single `let lower = raw.to_lowercase()`. This finding is discharged by the SA fix.

---

### Dismissed

**Finding 3 — `parse_id` collapses all error cases into one message (Dim 5 — Error specificity)**

`parse_id("abc")`, `parse_id("0")`, and `parse_id("-5")` (parsed by u64 as a parse error) all produce the same message: `'<input>' is not a valid issue ID. Expected a positive integer.` The spec requires this message for all three cases (DESIGN.md Feature 3 error states). The implementation is spec-correct.

**Classification:** Dismissed. The unified message is the spec-required behavior.

---

**Finding 4 — `parse_id` uses `.ok().filter().ok_or_else()` chain (Dim 6 — Readability)**

```rust
raw.parse::<u64>().ok().filter(|&n| n > 0).ok_or_else(|| format!(...))
```

This is idiomatic Rust for "parse → filter → convert to Result." An alternative with explicit `match` would be more verbose. The chain is correct and readable for someone familiar with Rust iterators.

**Classification:** Dismissed. The chain is idiomatic and correct.

---

**Finding 5 — `created_at` immutability (deferred item discharged)**

`cmd_status` does not reference `created_at`. The `Status` subcommand in `main.rs` passes only `id` and `status` strings to `cmd_status`. No code path in the Layer 2 additions touches `created_at`. Structural invariant holds. ✓

**Classification:** Dismissed. Deferred item fully discharged at Layer 2.

---

**Finding 6 — No new `unwrap()` on user-facing paths (Dim 5)**

`parse_status`, `parse_id`, `cmd_status`: no `.unwrap()` calls. The existing `save_issues` `.unwrap()` is unchanged and carries its inline safety comment. `#![deny(clippy::unwrap_used)]` continues to enforce documentation on any future `.unwrap()`.

**Classification:** Dismissed. Clean.

---

### Open

*(none)*

---

### Summary

One real finding resolved: `cmd_status` refactored from `iter_mut().find()` to `iter().position()` — eliminates unnecessary `new_status.clone()` and the resulting borrow conflict. `new_status` is moved into `issues[idx].status`; `println!` reads `issues[idx].status`. Finding 2 discharged by the SA fix. `created_at` immutability deferred item fully discharged. No panic surface on user-facing paths. No new `.unwrap()` without safety documentation.

**Coordination:** Finding 2 resolved by [SOLUTION-ARCHITECT-REVIEW.md](SOLUTION-ARCHITECT-REVIEW.md) Review 6 Finding 1 (deduplicated `parse_status` against `VALID_STATUSES`).
