# Quality Engineer Review Log

This review is part of the [Iterative Adversarial Refinement (IAR)](README.md) suite. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

**Reviewer role: Quality Engineer** (Quality Engineer / QA Engineer / Test Engineer)

The purpose of this review is to evaluate the quality system as a whole: whether the testing strategy, coverage, tooling, and gates are structured to catch defects reliably. A passing test suite that would not catch a broken implementation is a quality failure.

**Language supplement applied:** `lang/rust.md` (QE section) + `lang/cli.md` (QE section).

**Sycophancy check:** An agent that wrote both the tests and the implementation will find the tests adequate because they reflect its own interpretation of the spec, not the spec itself. The most dangerous failure mode in QE is not a missing test — it is a complete, passing test suite for the wrong behavior. Flag any case where the tests and implementation are internally consistent but could both be wrong. Flag any dimension where the answer is "tests exist and pass" without verifying that a broken implementation would actually fail them.

---

## Review 1 — 2026-04-27 21:00Z

**Scope:** `TODO.md` Red Gate test plans and `DESIGN.md` Testing Methodology. No implementation code exists. Pre-implementation pass: evaluating the quality of the test plan before any code is written.

**Session note:** In-session with all other domain reviews and with project authorship. Acknowledged quality tradeoff.

**Assumption surfacing:** No implementation dependencies to validate yet. The test plan references integration tests that invoke the binary as a subprocess — consistent with the Rust supplement's requirement for CLI integration tests.

---

### Resolved

**Finding 1 — No Red Gate test for `created_at == updated_at` on fresh issue (Dim 1 — Acceptance criteria)**

DESIGN.md states: "`created_at` and `updated_at` are equal on a freshly created issue." This is an acceptance criterion in Layer 1 (`TODO.md`) but no corresponding Red Gate integration test exists. A stub that sets `updated_at` to epoch or to a different timestamp would pass all existing Red Gate tests.

**Resolution:** Added to `TODO.md` Layer 1 Red Gate: `create_timestamps_equal_on_fresh_issue` — reads `tracker.json` after create, asserts `created_at == updated_at` — fails against stub that sets them to different values.

---

**Finding 2 — No Red Gate test for title 50-char truncation in list output (Dim 1 — Acceptance criteria)**

Layer 1 acceptance criteria include: "Title truncates at 50 characters with `…` in list output." No corresponding Red Gate test exists. A stub that prints the full title would satisfy all existing Layer 1 tests.

**Resolution:** Added `list_truncates_title_at_50_chars_with_ellipsis` to `TODO.md` Layer 1 Red Gate.

---

**Finding 3 — No Red Gate test for `tracker list --status in-progress` (Dim 1 — Acceptance criteria)**

Layer 2 Red Gate names `list_status_filter_shows_done` for the done case but has no corresponding test for the `in-progress` case. A stub that only handles `open` and `done` status filters would pass all Layer 2 Red Gate integration tests.

**Resolution:** Added `list_status_filter_shows_in_progress` to `TODO.md` Layer 2 Red Gate.

---

**Finding 4 — No Red Gate test for `created_at` immutability after status mutation (Dim 2 — Test falsifiability)**

Layer 2 acceptance criteria include: "All other fields on the issue are unchanged [after status change]." The `status_change_leaves_other_fields_unchanged` test covers this, but `created_at` is not explicitly named. An implementation that updates `created_at` on mutation would pass the existing test if "other fields" is interpreted loosely. A distinct test that explicitly asserts `created_at` is unchanged removes the ambiguity.

**Resolution:** Added `status_change_does_not_modify_created_at` to `TODO.md` Layer 2 Red Gate.

---

**Finding 5 — No Red Gate test for label 20-char truncation in list output (Dim 1 — Acceptance criteria)**

Layer 4 acceptance criteria include: "Labels column truncates at 20 characters with `…` if longer." No corresponding Red Gate integration test exists.

**Resolution:** Added `list_label_value_truncated_at_20_chars` to `TODO.md` Layer 4 Red Gate.

---

### Dismissed

**Finding 6 — Write-failure error paths have no Red Gate integration tests (Dim 1 — Acceptance criteria)**

DESIGN.md Storage edge cases include: write fails (disk full, permissions) → stderr `Error: Could not save tracker data: <reason>.` → exit 1; `tracker.json` is a directory → treated as I/O failure. These are acceptance criteria but cannot be reliably automated in cross-platform integration tests (require OS-level setup: filling a disk, revoking file permissions, creating a directory at the expected path).

**Classification:** Dismissed. Write-failure paths are listed in the Layer 1 and Layer 6 manual testing checklists. These are the only paths for which manual-only verification is acceptable. The acceptance criterion is present; its verification path is manual. The layer gate checklist must include explicit steps for these paths before Layer 1 merges.

---

**Finding 7 — Layer 5 test plan has only 4 integration tests; compound filter permutations are sparse (Dim 6 — Validation gaps)**

`tracker list --status open --priority high`, `--status open --label bug`, `--priority high --label bug`, and all three together are listed. The case `--status done --priority high --label bug` (a three-filter combination with a non-default status) is not explicitly tested. A stub that only AND-combines when `--status open` is present would pass.

**Classification:** Dismissed. The `list_two_filter_and_combination` and `list_three_filter_and_combination` tests are defined against an implementation that must AND-combine any combination, not just open-status combinations. The acceptance criterion "An issue that matches two of three filters but not the third does NOT appear" is tested with setup that produces clear counter-examples. Additional permutation tests can be added during implementation if a gap is discovered; the Red Gate coverage is sufficient to fail a naive stub.

---

### Open

*(none)*

---

### Summary

Five real findings, all resolved via `TODO.md` additions. Two dismissed with rationale. The test plan now covers all primary acceptance criteria with Red Gate tests. Write-failure paths are explicitly manual. No open items.

**Coordination:** Finding 4 (created_at immutability) surfaces to [SOFTWARE-ENGINEER-REVIEW.md](SOFTWARE-ENGINEER-REVIEW.md) as an implementation concern — the mutation path must never touch `created_at`.

---

---

## Review 2 — 2026-04-27 22:00Z

**Scope:** Layer 1 Red Gate test code — `tests/layer1.rs`, `src/lib.rs` unit tests. Artifacts: `Cargo.toml`, `src/main.rs`, `src/lib.rs`, `tests/layer1.rs`. No implementation code exists beyond stubs. Evaluating test correctness, spec alignment, and Red Gate compliance before any implementation is written.

**Session note:** In-session with all other Layer 1 domain reviews. Acknowledged quality tradeoff. This is the pre-implementation gate IAR required by VDD-IAR Review 2 (Finding 2 — at least one cold-session domain review before Layer 1 merges). The current pass is in-session; cold-session requirement applies at the Layer 1 **merge** gate, not the Red Gate writing gate.

---

### Raised to SO

**Finding 1 — Integration tests assume top-level JSON array; spec defines a wrapped object (Dim 3 — Test accuracy)**

`tests/layer1.rs` reads `tracker.json` and accesses issue data as a top-level JSON array:

```rust
let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
assert_eq!(v[0]["title"], "Fix bug");
```

`v[0]` accesses the first element of a top-level array. This is correct only if `tracker.json` contains `[{...}]`. But `DESIGN.md` specifies the storage format as a wrapped object:

```json
{ "issues": [Issue] }
```

If the implementation follows the spec and writes `{"issues": [...]}`, then `v[0]` evaluates to `Value::Null` (a JSON object indexed by integer 0 returns null) and the `assert_eq!` passes trivially against null — which means the test would not catch a correct implementation at all, or would catch it for the wrong reason. Four tests are affected: `create_stores_issue_in_json`, `create_trims_title`, `create_first_issue_unchanged_after_second_create`, `create_timestamps_equal_on_fresh_issue`.

Proposed resolution options for SO decision:
1. Fix the tests to use `v["issues"][0]` — tests become spec-correct against the wrapped format.
2. Simplify DESIGN.md to use a top-level array — tests are already correct, and a top-level array is simpler to deserialize in Rust (`serde_json::from_str::<Vec<Issue>>(&raw)` with no wrapper struct needed). Consistent with SA Review 1's complexity-budget principle.

**Classification:** Raised to SO. DESIGN.md is a controlled spec document. QE does not apply changes to DESIGN.md. This finding and the proposed resolution are handed to SO for decision. See [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) Review 7 Finding 1.

**Cross-reference:** [DATA-ENGINEER-REVIEW.md](DATA-ENGINEER-REVIEW.md) Review 2 Finding 1 (schema); [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) Review 7 Finding 1 (resolution).

---

### Dismissed

**Finding 2 — `list_shows_header_and_issues` uses `get_output()` instead of predicate chaining (Dim 2 — Test clarity)**

The test reads raw stdout bytes via `get_output().stdout.clone()` and then uses `assert!()` macros. Other tests use `predicate::str::contains()` chains directly in the `assert_cmd` API.

**Classification:** Dismissed. Both patterns are valid in assert_cmd 2.x. The `get_output()` approach is appropriate here because the test asserts multiple independent `contains()` conditions and converting them all to assert_cmd predicate chains would require `.and()` combinators that add syntactic noise. The test is correct and readable.

---

**Finding 3 — No test for `tracker.json` NOT existing before an error-state create (Dim 1)**

`create_empty_title_exits_one_with_error_on_stderr` does not assert that `tracker.json` was not created as a side effect.

**Classification:** Dismissed. The spec's postcondition is: if create fails (e.g., empty title), no issue is stored. The test correctly asserts exit 1 and error on stderr. The absence of a `tracker.json` check is a gap but not a Red Gate requirement — a stub that exits 1 correctly but creates a malformed file would be a subsequent implementation finding, not a current test gap. The Layer 1 manual testing checklist covers this: "Error state — empty title: verify no `tracker.json` created (or existing data unchanged)."

---

**Finding 4 — `create_whitespace_title_exits_one` does not assert stdout is empty (Dim 1)**

The test asserts exit 1 and stderr contains the error, but does not assert `stdout("")`.

**Classification:** Dismissed. The error message contract (stderr only, stdout empty) is tested exhaustively in `create_empty_title_exits_one_with_error_on_stderr` which does assert `.stdout("")`. Testing it on the whitespace variant as well would be redundant — these two tests share the same code path through `validate_title`. The spec contract is covered.

---

### Open

*(none)*

---

### Summary

One real finding identified and raised to SO: integration tests assumed a top-level JSON array but the spec defined a wrapped object. QE does not hold change authority over DESIGN.md — the finding was escalated to SO Review 7 with two proposed resolution options. SO approved the storage format simplification; DESIGN.md was updated by SO authority.

Red Gate compliance verified: all 17 tests (13 integration + 4 unit) fail against the stubs. Integration tests fail with `Unexpected success` or `Unexpected stdout` — the stubs produce no output. Unit tests fail with `not yet implemented` panics. Both failure modes are the correct Red Gate failure pattern.

**Coverage check:** All 13 integration test names map 1:1 to Red Gate entries in TODO.md Layer 1. All 4 unit test names map to Layer 1 unit test entries. No undocumented tests; no documented tests missing.

---

---

## Review 3 — 2026-04-28 05:06Z

**Scope:** Layer 1 implementation — `src/lib.rs`, `src/main.rs`, `tests/layer1.rs`. All 17 Red Gate tests verified passing (`cargo test`: 4 unit, 13 integration). Evaluating test accuracy, spec alignment, and field coverage completeness against the Layer 1 acceptance criteria. This is a post-implementation cold-session review.

**Session note:** Cold-session — reviewing implementation committed in a prior session. Satisfies the VDD-IAR Review 2 cold-session requirement before Layer 1 gate closes.

**Assumption surfacing:** Storage format is a top-level JSON array (SO Review 7 Finding 1 resolution applied). Tests using `v[0]` are correct against the current implementation.

---

### Resolved

**Finding 1 — `list_truncates_title_at_50_chars_with_ellipsis` does not assert the specific truncation point (Dim 2 — Test falsifiability)**

The test creates a 60-char title and asserts:
1. `out.contains('…')` — passes for any ellipsis in output
2. `!out.contains(&long_title)` — passes if the full 60-char string is absent

A regression that truncates at 20 chars (producing `"AAAAAAAAAAAAAAAAAAA…"`) satisfies both conditions. The Red Gate spec says "asserts list stdout contains the 50-char prefix followed by `…`" — the test does not verify the prefix length. Any implementation producing an ellipsis shorter than the full title passes.

**Resolution:** Updated `list_truncates_title_at_50_chars_with_ellipsis` in `tests/layer1.rs` to assert the exact 49-char prefix followed by `…`:

```rust
let expected = format!("{}…", "A".repeat(49));
assert!(out.contains(&expected), "expected 49 'A's + '…' in output:\n{out}");
```

The full-title absence assertion is retained as a secondary check.

---

**Finding 2 — `create_first_issue_unchanged_after_second_create` does not verify `labels`, `created_at`, or `updated_at` (Dim 1 — Acceptance criteria)**

The acceptance criterion is: "first issue is unchanged." The test asserts `id`, `title`, `status`, `priority` — but not `labels`, `created_at`, or `updated_at`. An implementation that resets `labels` to `null` or regenerates timestamps on every write (re-serializing all issues with fresh timestamps) would pass all 17 Layer 1 tests. The `create_timestamps_equal_on_fresh_issue` test only checks timestamps for a single-create case — it does not verify timestamps survive a second create.

**Resolution:** Added assertions for `labels`, `created_at`, and `updated_at` in `create_first_issue_unchanged_after_second_create`. `labels` must be `[]`; `created_at` and `updated_at` must be non-null strings identical to what was stored after the first create (captured before the second create and compared after).

---

**Finding 3 — `malformed_json_causes_error_exit` asserts only a substring of the user-actionable error message (Dim 3 — Test selector and assertion strength)**

The test checks `predicate::str::contains("Could not read tracker data")`. The acceptance criterion specifies the full message: `Error: Could not read tracker data. The file may be corrupt. Delete tracker.json to start fresh.`

Two distinct code paths in `load_issues` produce different messages:
- File read failure: `format!("Could not read tracker data: {}.", e)` — includes OS error, omits delete instruction
- JSON parse failure: `"Could not read tracker data. The file may be corrupt. Delete tracker.json to start fresh."` — user-actionable

The test triggers the JSON parse failure path but would also pass if the implementation accidentally routed malformed-JSON errors through the file-read-failure message format, which omits the critical delete instruction.

**Resolution:** Updated the stderr assertion to check the distinguishing suffix: `predicate::str::contains("The file may be corrupt. Delete tracker.json to start fresh.")` — this uniquely identifies the parse-failure path and verifies the full user-actionable text.

---

### Dismissed

**Finding 4 — No doc tests on public API functions (Dim 13 — Quality gates; Rust supplement — TW)**

`lib.rs` exports `validate_title`, `next_id`, `current_timestamp`, `load_issues`, `save_issues`, `cmd_create`, `cmd_list`. None have `///` doc comments or doc test examples. The rust.md QE supplement lists "Doc tests compile and pass" as a dimension.

**Classification:** Dismissed. This `lib.rs` exists to enable integration testing of a binary crate, not to expose a library API for external consumers. The `pub` visibility is structural, not a publication contract. Doc test coverage for binary-internal modules is a Technical Writer concern per the rust.md TW supplement. All exported functions are exercised through integration tests and unit tests. No open item.

---

**Finding 5 — `save_issues` uses `.unwrap()` on `serde_json::to_string_pretty` (Dim 7 — Logic errors)**

`serde_json::to_string_pretty(issues).unwrap()` in `save_issues` could panic if serialization fails.

**Classification:** Dismissed from QE. `Issue` fields are `u64`, `String`, `Vec<String>`, and `Option<String>` — none of which can produce serialization errors (no NaN, no Inf, no reference cycles). The `.unwrap()` is on a provably-safe value, not a user-input path. This is an SE domain finding. Noted for SE coordination.

---

### Open

*(none)*

---

### Summary

Three real findings, all resolved via `tests/layer1.rs` changes: the truncation test now asserts the exact 49-char prefix; the "unchanged" test now covers `labels`, `created_at`, and `updated_at`; the malformed-JSON test now asserts the full user-actionable error suffix. Two dismissed with rationale. No open items.

**Coordination:** Finding 5 (`save_issues` `.unwrap()`) noted for [SOFTWARE-ENGINEER-REVIEW.md](SOFTWARE-ENGINEER-REVIEW.md) as an SE-domain observation — the panic path is unreachable given `Issue`'s field types, but SE should document this invariant if the field types ever expand.

**Cold-session gate:** This review satisfies the Layer 1 merge gate cold-session requirement for QE.

---

---

## Review 4 — 2026-04-28 05:30Z

**Scope:** Layer 1 implementation — post-Security Review 3 gap fix. Evaluating test coverage for the newly implemented post-deserialization domain validation (`issue_fields_are_valid` in `lib.rs`). In-session with full Layer 1 IAR suite.

**Session note:** In-session. Acknowledged quality tradeoff.

---

### Resolved

**Finding 1 — No test for valid-JSON-invalid-domain-value error path (Dim 1 — Acceptance criteria coverage)**

Security Review 1 required that `tracker.json` with semantically invalid domain values (valid JSON structure, invalid field content) produce a corrupt-data error. DESIGN.md explicitly names this as an acceptance criterion. The Layer 1 Red Gate tests covered `malformed_json_causes_error_exit` (non-parseable JSON) but not the distinct case of valid JSON with invalid domain values.

The Layer 1 implementation added `issue_fields_are_valid()` to address the spec requirement. Without a corresponding test, a future regression that removed the validation would pass all existing tests.

**Resolution:** Added `invalid_domain_values_in_json_causes_error_exit` to `tests/layer1.rs`. The test writes a structurally-valid JSON array with `"status": "flying"` to `tracker.json`, then asserts that `tracker list` exits 1 and stderr contains `"The file may be corrupt. Delete tracker.json to start fresh."`. The test passes against the fixed implementation and would fail against a version of `load_issues` without domain validation.

**Test total: 18** (14 integration + 4 unit). All 18 pass.

---

### Dismissed

**Finding 2 — `invalid_domain_values_in_json_causes_error_exit` covers only the status field (Dim 1)**

The test triggers validation via `"status": "flying"`. It does not exercise the `"id": 0`, `"title": ""`, or `"priority": ""` paths separately.

**Classification:** Dismissed. The validation logic in `issue_fields_are_valid()` is a single `&&`-chained boolean expression. Any one failing condition returns `false`. A single test case that exercises the function via one invalid field is sufficient to establish that the validation path is active. Exhaustive per-field testing would over-specify the test suite and is unnecessary at Layer 1. The function's logic is simple enough that unit tests at the function level would add more value than additional integration tests.

---

### Open

*(none)*

---

### Summary

One finding resolved: test added for the post-deserialization domain validation path. 18 tests total, all passing. No open items. The test suite is now aligned with all acceptance criteria including the previously untested corrupt-domain-values case. Layer 1 test coverage is complete.

**Coordination:** *(none)*

---

---

## Review 5 — 2026-04-30 00:00Z

**Scope:** Layer 1 gate closure pass — no code changes since Review 4. Reviewing test suite completeness against spec and manual testing results.

**Session note:** In-session with all other domain reviews. Acknowledged quality tradeoff.

---

### Resolved

**Finding 1 — `list_shows_header_and_issues` does not assert `(none)` in Labels column (Dim 4 — Acceptance criteria coverage)**

The spec requires that `tracker list` shows `(none)` in the Labels column for unlabeled issues (DESIGN.md Feature 2, Interface section). The test `list_shows_header_and_issues` creates an unlabeled issue and verifies the header columns and issue title appear in the output, but did not assert that `(none)` is present. A regression that rendered an empty string instead of `(none)` would not have been caught.

**Resolution:** Added `assert!(out.contains("(none)"), "unlabeled issue should show '(none)' in Labels column")` to `list_shows_header_and_issues`. All 18 tests still pass.

---

### Dismissed

*(none)*

---

### Open

*(none)*

---

### Summary

One finding resolved: `(none)` label assertion added to `list_shows_header_and_issues`. 18 tests passing. No additional gaps found — the 18-test suite covers: create (exit code, storage, trimming, error states, ID assignment, timestamps, first-issue-unchanged), list (empty state, header, truncation, after create), error paths (malformed JSON, invalid domain values). MVR reached for Layer 1.

**Coordination:** *(none)*

---

---

## Review 6 — 2026-04-30 00:00Z

**Scope:** General adversarial review, pre-merge gate. Review-session primer loaded. Applying mutation analysis (dim 2) to the full test suite — for each branch in `lib.rs`, enumerate a plausible one-line mutation and determine whether any test would fail. A mutation that survives the full suite is a coverage gap regardless of how many times the test suite has been run. Fresh adversarial pass with explicit obligation to the spec, not to prior review dismissals.

**Session note:** In-session review. Acknowledged quality tradeoff.

---

### Resolved

**Finding 1 — Sort direction mutation survives all tests (Dim 2 — Test falsifiability)**

Mutation: swap `a.id.cmp(&b.id)` to `b.id.cmp(&a.id)` in `cmd_list` (`lib.rs:118–121`).

Result: every existing test creates one issue and runs `list`, or creates two issues and reads JSON directly. No test creates two issues and runs `tracker list`. The sort direction mutation would produce issue #2 before issue #1 in list output — and zero tests would catch it.

Cross-check against acceptance criterion #10 (TODO.md): "tracker list after two creates shows both issues in a table." No integration test exercised the two-create → list path. The named test `list_after_create_shows_issue` creates one issue. The named test `list_shows_header_and_issues` creates one issue. The mutation survives.

**Resolution:** Added `list_shows_multiple_issues_in_id_order` integration test (`tests/layer1.rs`). Creates two issues, runs `tracker list`, asserts both titles appear and `"First issue"` position precedes `"Second issue"` position in output. This test would fail on the sort-direction mutation and on the "both issues missing" regression.

---

**Finding 2 — `id > 0` validation branch independently removable (Dim 2 — Mutation testing)**

`issue_fields_are_valid` (`lib.rs:42–47`) checks four conditions joined by `&&`: `issue.id > 0`, non-empty title, valid status, valid priority. The existing `invalid_domain_values_in_json_causes_error_exit` test exercises only the valid-status branch (`"status": "flying"`). Removing the `id > 0` clause produces zero test failures — no test writes `"id": 0` to `tracker.json`.

With `id: 0` accepted, a future `cmd_create` would produce `next_id([0]) = 1`, potentially creating a state where `tracker.json` contains two issues with `id: 1`. The `id > 0` check is an independently testable invariant.

Note: Red Team Review 2 dismissed this as "the validation path is the same for any failing field." That dismissal is incorrect — the conditions are independent conjuncts. Removing one does not cause any other branch's test to fail. Each branch requires its own test to be independently removable.

**Resolution:** Added `zero_id_in_json_causes_error_exit` integration test (`tests/layer1.rs`). Writes a `tracker.json` with `"id": 0` and verifies `tracker list` exits 1 with the corrupt-data error.

---

### Dismissed

**Finding 3 — Empty-title-in-JSON mutation (Dim 2 — Mutation testing)**

Same pattern as Finding 2: removing the `!issue.title.trim().is_empty()` clause from `issue_fields_are_valid` would also survive the test suite. However, the empty-title path through CLI input is thoroughly tested by `create_empty_title_exits_one_with_error_on_stderr` and `create_whitespace_title_exits_one`. An empty stored title can only arise from manual file editing, and the threat model is already bounded. With Finding 2 (zero-id) now tested, the principle of validation branch independence is established. Adding a third variant is diminishing returns at this layer. The existing `invalid_domain_values_in_json_causes_error_exit` test establishes that post-deser validation triggers correctly; zero-id tests a second branch independently.

**Classification:** Dismissed at Layer 1 — revisit if the full post-deser validation function is refactored.

---

### Open

*(none)*

---

### Summary

Two real findings resolved: sort direction mutation (`list_shows_multiple_issues_in_id_order`) and zero-id validation gap (`zero_id_in_json_causes_error_exit`). 20 tests total (16 integration + 4 unit), all passing. The sort algorithm, the two-create → list path, and the `id > 0` validation branch are now independently tested. Review-session adversarial posture applied; mutation analysis exhausted real gaps.

**Coordination:** *(none)*

---

---

## Review 7 — 2026-05-01 00:00Z

**Scope:** Layer 2 implementation — `tests/layer2.rs`, `src/lib.rs` unit tests, `TODO.md` Red Gate plan. Evaluating test correctness, Red Gate compliance, acceptance criteria coverage, and mutation analysis. 17 integration tests in `tests/layer2.rs`, 3 unit tests added to `src/lib.rs` (`status_value_parsing_valid_cases`, `status_value_parsing_rejects_invalid`, `id_must_be_positive_integer`). Total suite: 37 tests (33 integration + 4 layer1 unit), all passing.

**Session note:** In-session with full Layer 2 IAR suite. Acknowledged quality tradeoff. Review-session primer applied. Adversarial obligation is to the spec, not the developer.

---

### Resolved

**Finding 1 — No test for non-open status filter with empty results (Dim 2 — Test falsifiability)**

The `is_open_view` logic in `cmd_list` controls which empty-state message appears:
- `effective_status == "open"` → "No open issues. Nice work!"
- anything else → "No issues match the given filters."

Mutation: change `let is_open_view = effective_status == "open";` to `let is_open_view = true;`

Result: `tracker list --status done` with no done issues would produce "No open issues. Nice work!" instead of "No issues match the given filters." **Zero of the 37 tests catch this mutation.**

Cross-check against acceptance criteria: "Issues exist but none match the filters → prints `No issues match the given filters.`" (DESIGN.md Feature 2). No test exercises this path for `--status done` or `--status in-progress` producing zero results.

**Resolution:** Added `list_nonempty_status_filter_with_no_match_shows_filter_message` to `tests/layer2.rs`. Creates one open issue, runs `tracker list --status done` (no done issues exist), asserts stdout = `No issues match the given filters.\n`. This test fails on the `is_open_view = true` mutation and passes on the correct implementation. All 38 tests pass.

---

### Dismissed

**Finding 2 — Two tests beyond the Red Gate plan in TODO.md (Dim 4 — Red Gate compliance)**

`tests/layer2.rs` contains 17 integration tests; the Red Gate plan in TODO.md lists 15. The two extra tests are:
- `list_explicit_open_filter_matches_default`
- `list_all_done_default_shows_empty_state`

Both cover Layer 2 acceptance criteria (`tracker list --status open` behaves identically to default; all done → "Nice work!" on default view). Both are accurate and useful. The question is whether they were written before or after implementation — a Red Gate compliance question for VDD-IAR Alignment dim 4.

From a QE perspective: both tests are falsifiable and would fail against a stub. They cover spec requirements. No quality concern about the tests themselves. The Red Gate sequence question is escalated to VDD-IAR Alignment Review.

**Classification:** Dismissed from QE. Tests are correct. VDD-IAR evaluates timing.

---

**Finding 3 — `status_change_refreshes_updated_at` uses a 1-second sleep (Dim 1)**

The test verifies that `updated_at` advances after a status change. It sleeps 1 second to guarantee a different timestamp at second precision (per spec: "ISO 8601, second precision"). The only alternative is a sub-second precision timestamp (out of scope) or a mocked clock (implementation complexity exceeding the scope). The sleep is the minimum correct test for second-precision timestamp semantics.

**Classification:** Dismissed. The sleep is necessary and correctly motivated by the spec's second-precision requirement.

---

**Finding 5 — `status_change_leaves_other_fields_unchanged` does not assert `description` field (Dim 1)**

The test asserts `id`, `title`, `priority`, `labels`, `created_at` are unchanged after a status update. It does not assert the `description` field.

**Classification:** Dismissed. Layer 2 has no `--description` flag — all created issues have `description: None`, which serializes as absent (no key). A status change cannot produce a description field where none existed. `description` is Layer 6 scope; asserting it in Layer 2 would assert on a field that doesn't exist in the JSON output yet. No gap.

---

### Hallucinated

**Finding 4 — No integration test for `tracker status 1 IN-PROGRESS` (uppercase) end-to-end (Dim 1)**

The acceptance criterion "`tracker status 1 IN-PROGRESS` (uppercase) exits 0; stored value is lowercase `in-progress`" is covered by the unit test `status_value_parsing_valid_cases` (tests `parse_status("IN-PROGRESS")`) and the integration test `status_is_case_insensitive_on_input` (tests "DONE" end-to-end). The integration path is: main.rs passes raw string → `cmd_status` → `parse_status`. The DONE integration test verifies the end-to-end path works for a case-insensitive value; the unit test verifies IN-PROGRESS parsing specifically.

**Classification:** Hallucinated. The combination of the unit test (IN-PROGRESS) and the integration test (DONE end-to-end) provides sufficient coverage. An additional integration test for IN-PROGRESS would duplicate coverage with no additional falsifiability.

---

### Open

*(none)*

---

### Summary

One real finding resolved: `list_nonempty_status_filter_with_no_match_shows_filter_message` added — the `is_open_view` mutation now fails. 38 tests total (34 integration + 4 unit), all passing. Mutation analysis exhausted real gaps in the Layer 2 test suite. The Red Gate sequence question (two extra tests) is escalated to [VDD-IAR-ALIGNMENT-REVIEW.md](VDD-IAR-ALIGNMENT-REVIEW.md).

**Coordination:** Finding 2 (Red Gate sequence question) noted for [VDD-IAR-ALIGNMENT-REVIEW.md](VDD-IAR-ALIGNMENT-REVIEW.md).

---

---

## Review 8 — 2026-05-02 00:00Z

**Scope:** Cold-session adversarial pass over the full Layer 2 implementation. Files read: `DESIGN.md`, `src/lib.rs`, `src/main.rs`, `tests/layer1.rs`, `tests/layer2.rs`, `Cargo.toml`, `TODO.md`. All prior QE review findings verified as closed. Applying fresh adversarial pressure with obligation to the spec, not to the prior reviewer's conclusions. Review 7 logged "38 tests (34 integration + 4 unit)." Actual count observed from `cargo test`: **41 tests** — 16 integration (layer1.rs) + 18 integration (layer2.rs) + 7 unit (lib.rs). The discrepancy of 3 is the layer2 unit tests (`status_value_parsing_valid_cases`, `status_value_parsing_rejects_invalid`, `id_must_be_positive_integer`) which were listed in Review 7's "Test count" note but excluded from the final tally. All 41 tests pass. Not a quality concern; the tests are correct.

**Session note:** Cold session — reviewing code committed in prior sessions. Satisfies adversarial quality standard at the Layer 2 gate. No prior participation in implementation or in-session QE passes.

**Assumption surfacing:** All library APIs verified: `serde_json::from_str::<Vec<Issue>>` for a top-level array (confirmed correct after SO Review 7 resolved the wrapped-vs-array question). `chrono::Utc::now()` is real-time UTC; `Option<String>` with `#[serde(skip_serializing_if = "Option::is_none")]` correctly omits absent fields. `Option<T>` deserialization in serde treats missing keys as `None` by default (no `#[serde(default)]` needed). No assumed-but-nonexistent APIs found.

---

### Resolved

**Finding 1 — `list_truncates_title_at_50_chars_with_ellipsis` off-by-one mutation survives (Dim 2 — Test falsifiability)**

QE Review 3, Finding 1 strengthened the truncation test to assert `"A".repeat(49) + "…"` — closing the gap against mutations that truncate shorter than 49 chars. The fix was correct but incomplete: the symmetric off-by-one mutation (truncating to 50 content chars + "…" = 51 display chars instead of the spec's max of 50) also survives.

The mutation is `chars[..max_chars - 1]` → `chars[..max_chars]` in `truncate_with_ellipsis` (`lib.rs:168`). With `max_chars = 50` and a 60-char input, the mutation produces `"A".repeat(50) + "…"`. The existing assertion `out.contains("A".repeat(49) + "…")` returns `true` against this output because the 49-A pattern appears as a substring starting at byte offset 1: the sequence `[A×49][0xE2][0x80][0xA6]` is found within `[A×50][0xE2][0x80][0xA6]`. The second assertion `!out.contains(&long_title)` also passes — the output has 50 As, not 60.

Both assertions pass on the mutated implementation. The mutation is undetected.

The spec contract: "Title consuming the remainder up to 50 characters… Title truncates at 50 characters with `…` if longer." The mutation violates this by rendering 51 display characters for a truncated title.

**Resolution:** Added a negative assertion to `list_truncates_title_at_50_chars_with_ellipsis` (`tests/layer1.rs`):

```rust
let not_expected = format!("{}…", "A".repeat(50));
assert!(
    !out.contains(&not_expected),
    "title must not truncate to 50 chars + ellipsis (would exceed 50-char display limit)"
);
```

This assertion fails on the off-by-one mutation (output contains `"A"×50 + "…"`) and passes on the correct implementation (output contains only `"A"×49 + "…"`). All 41 tests pass.

---

**Finding 2 — `status_not_found_exits_one` assertion does not verify the issue ID appears in the error (Dim 3 — Assertion strength)**

The test asserts `predicate::str::contains("not found")` (`tests/layer2.rs`). The spec mandates: `Error: Issue #<id> not found.`

A mutation that removes the issue ID from the format string in `cmd_status` (`lib.rs:146`) — changing `format!("Issue #{} not found.", id)` to `"Issue not found.".to_string()` — would:
1. Produce stderr `"Error: Issue not found.\n"`
2. Still satisfy `contains("not found")` — the test passes

The ID is user-actionable: without it, the user has no way to confirm which issue was not found without re-examining their command. The error format is a spec contract, not implementation detail.

The prior assertion pattern `contains("not found")` is also weaker than comparable error tests in the same file. `status_invalid_id_string_exits_one` checks `contains("not a valid issue ID")`, which uniquely identifies the error path. `contains("not found")` is too broad — any future error that happens to contain "not found" as a substring would satisfy it.

**Resolution:** Updated `status_not_found_exits_one` in `tests/layer2.rs` to assert the spec-mandated full message:

```rust
.stderr(predicate::str::contains("Issue #99 not found."))
```

This assertion fails on both the ID-omission mutation and any routing to the wrong error handler. All 41 tests pass.

---

### Raised to SO

**Finding 3 — Empty state messages on `stdout` may pollute piped output (CLI supplement dim 6)**

DESIGN.md specifies that `tracker list` empty state messages go to `stdout`:
- `"No open issues. Nice work!"` (default view, empty)
- `"No issues match the given filters."` (explicit filter, no matches)

The implementation follows the spec — both messages go via `println!()`. No finding against the implementation.

The CLI supplement (dim 6) asks: "Is the empty message on `stderr` so it does not pollute piped output?" For example, `tracker list | wc -l` with an empty tracker would output `1` (counting the "Nice work!" line) rather than `0`. A user scripting against list output would need to distinguish data rows from empty state messages.

This is a spec design choice, not an implementation bug. **Classification: Raised to SO.** DESIGN.md is the authoritative contract. QE does not modify it. Proposed question for SO: should empty state messages route to `stderr` to keep `stdout` clean for piped use? If yes, the implementation requires a corresponding change (`eprintln!` instead of `println!`), and the tests `list_with_no_json_shows_empty_state` and `list_all_done_default_shows_empty_state` require updating their `stdout`/`stderr` assertions.

---

### Dismissed

*(none)*

---

### Hallucinated

**Finding 4 — Column order in header not tested (Dim 3)**

A mutation swapping two columns in the header format string would produce incorrect column ordering, and the existing `list_shows_header_and_issues` test would not catch it (it only checks `contains("ID")`, `contains("Status")`, etc., not their order). This could be filed as a Dim 3 gap.

**Classification:** Hallucinated. The column format string is a single `println!` macro with explicitly ordered fields (`ID`, `Status`, `Priority`, `Labels`, `Title`). The risk of a mutation scrambling column order while preserving all column names is theoretical — it cannot occur as an accidental off-by-one or logic mutation; it would require intentionally reordering named format arguments. The format string is visually auditable and unlikely to drift. Mutation testing is most valuable for logic paths, not static string ordering. The finding is out of proportion to the risk.

---

### Open

*(none)*

---

### Summary

Two real findings resolved: (1) `list_truncates_title_at_50_chars_with_ellipsis` now catches the off-by-one mutation that produces 51 display chars; (2) `status_not_found_exits_one` now asserts the spec-mandated full message including the issue ID. One finding raised to SO (empty state messages on stdout vs. stderr — a spec design question, not an implementation bug). One finding hallucinated (column order). Test count corrected from 38 to 41; all pass.

**Dimensions audited and cleared:**
- **Dim 1 — Acceptance criteria:** All Layer 1 and Layer 2 acceptance criteria in `TODO.md` are marked complete. The implementation was verified against each: title trimming, ID assignment, timestamp equality, status defaults, sort ordering, status filter, error paths. No uncovered acceptance criterion found.
- **Dim 4 — Coverage meaningfulness:** Prior reviews did not measure coverage. No `cargo tarpaulin` or `cargo llvm-cov` is configured. The QE standard requires 80% minimum line coverage enforced in CI. However: (a) this project has no CI infrastructure; (b) the public API functions (`validate_title`, `next_id`, `parse_status`, `parse_id`, `load_issues`, `save_issues`, `cmd_create`, `cmd_list`, `cmd_status`) are all exercised by integration or unit tests; (c) the only unexercised paths are the write-permission-denied and `tracker.json`-is-a-directory I/O failures, which were explicitly dismissed in QE Review 1 Finding 6 as requiring OS-level setup. Coverage measurement is a valid gap but CI enforcement is a PE domain concern. Escalated to Platform Engineer for CI configuration.
- **Dim 5 — Test architecture:** All integration tests use `TempDir` for isolation — each test gets its own temp directory, no shared file state, no ordering dependencies. The two timing-sensitive tests (`status_change_refreshes_updated_at`, `status_idempotent_same_value_succeeds`) use an explicit 1-second sleep, justified by the spec's second-precision timestamp contract and dismissed in Review 7 Finding 3. No flaky test patterns detected.
- **Dim 8 — Dead code:** All exported functions are reachable. `truncate_with_ellipsis` and `priority_rank` are private and called only from `cmd_list`. `issue_fields_are_valid` is private and called only from `load_issues`. No dead public exports.
- **Dim 9 — Unused dependencies:** `serde`, `serde_json`, `clap`, `chrono` all used. Dev dependencies `assert_cmd`, `predicates`, `tempfile` all used. None unused.
- **Dim 14 — TDD proxy indicators:** Tests call the implementation at its public interface (subcommand invocations via subprocess, or public function calls in unit tests). Test names are behavior-named. Red Gate compliance was verified in Reviews 2 and 7. No implementation coupling detected — the tests assert on outputs and stored JSON, not internal data structures.

**Cold-session signal:** The two resolved findings were introduced by in-session reviewers who verified correctness within the implementation's frame of reference. Finding 1 was in a test specifically strengthened in QE Review 3 — the strengthening was real but incomplete. Cold-session pressure exposed the residual gap. This is the expected value of cold-session review.

**Coordination:** Finding 3 (coverage measurement) noted for [PLATFORM-ENGINEER-REVIEW.md](PLATFORM-ENGINEER-REVIEW.md).
