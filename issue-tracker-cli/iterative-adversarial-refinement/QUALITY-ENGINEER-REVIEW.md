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

---

---

## Review 9 — 2026-05-04 05:50Z

**Scope:** Layer 3 implementation — `tests/layer3.rs` (7 integration tests pre-review), 4 unit tests added in `src/lib.rs`, the `cmd_list` empty-state heuristic fix from SO Review 11, and the priority-constants unification from SA Review 7. Files read: `DESIGN.md`, `TODO.md`, `src/lib.rs`, `src/main.rs`, `tests/layer1.rs`, `tests/layer2.rs`, `tests/layer3.rs`, `Cargo.toml`. All Layer 1 and Layer 2 acceptance criteria re-traced — no regression detected.

**Session note:** Same-session-as-other-domains adversarial review (orchestrator did not spawn a fresh subagent for QE in this round; user rejected the cold-session subagent invocation). Acknowledged quality tradeoff per session-primer guidance: a same-session reviewer shares context with prior domains in this round (SO Review 11, SA Review 7) and is more likely to confirm their findings than to find new ones. Round 1 of Layer 3 QE — recommend a follow-up cold-session pass before the layer gate closes.

**Assumption surfacing:** All test-crate APIs in `tests/layer3.rs` verified against current versions: `assert_cmd::Command::cargo_bin`, `Command::current_dir`, `assert::success/failure().code(1).stderr(predicate::str::contains).stdout(predicate::str::contains.not())`, `tempfile::TempDir`, `serde_json::Value` indexing with `["priority"]`. No hallucinated APIs. `predicate::str::contains(...).not()` used correctly (prefix-form) in the new regression test.

### Layer 3 acceptance criteria → test trace

| Criterion (TODO.md Layer 3) | Test(s) covering it | Verdict |
|---|---|---|
| `--priority high` stores `"high"` | `create_with_priority_stores_correct_value` | ✓ |
| no flag → `"medium"` default | `create_without_priority_defaults_to_medium` | ✓ |
| `--priority HIGH` (uppercase) → stored `"high"` | `priority_parsing_valid_cases` (unit) — covers `to_lowercase` | ✓ via unit (mirrors Layer 2 status case-insensitivity test pattern) |
| `--priority critical` → exit 1, stderr | `create_invalid_priority_exits_one` | ✓ |
| sort high → medium → low | `list_sorts_high_before_medium_before_low` (integration) + `priority_sort_order_is_correct` (unit) | ✓ |
| within-tier ID ascending | `priority_sort_tie_breaking_by_id` (unit, with reverse-order input that exercises tie-breaker) + `list_within_tier_sorted_by_id_ascending` (integration, weak — see Finding 2) | ✓ via unit |
| `--priority high` shows only high | `list_priority_filter_shows_only_matching` | ✓ |
| `--priority medium` shows only medium | `priority_parsing_valid_cases` (parse) — retain logic shared with `high` | ✓ via shared code path (see Finding 3) |
| `--priority low` shows only low | same | ✓ via shared code path |
| `--priority invalid` → exit 1 | `list_invalid_priority_filter_exits_one` | ✓ |
| `--status open --priority high` AND-combined | TODO defers full compound-filter verification to Layer 5 | ✓ (deferral honored; positive case implicit from `list_priority_filter_shows_only_matching` with default-open status) |

---

### Resolved

**Finding 1 — Missing regression test for `is_open_view` empty-state heuristic with `--priority` filter (Dim 12 — Regression coverage)**

SO Review 11 fixed `cmd_list` to print `No issues match the given filters.` instead of `No open issues. Nice work!` when `--priority X` is passed and no matching issues exist. The fix is in `src/lib.rs:225`:

```rust
let is_open_view = effective_status == "open" && effective_priority.is_none();
```

No test currently asserts this behavior. The pre-fix bug (`is_open_view = effective_status == "open"`) passes all 52 prior tests. A future change reverting the fix — accidentally or intentionally — would not be caught by the test suite. Per QE prompt dim 12 ("Does every bug logged in the review log have an identifiable regression test?"), this is a regression coverage gap.

Mutation evaluation: removing `&& effective_priority.is_none()` (reverting the SO fix) on a tracker containing one open low-priority issue, then running `tracker list --priority high`:
- Pre-fix output: `"No open issues. Nice work!\n"`
- Post-fix output: `"No issues match the given filters.\n"`

The regression test must assert both the positive (post-fix message present) and the negative (pre-fix message absent) so the bug cannot pass either assertion alone.

**Resolution:** Added `list_priority_filter_no_match_shows_filter_message` to `tests/layer3.rs`:

```rust
#[test]
fn list_priority_filter_no_match_shows_filter_message() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Low item", "--priority", "low"])
        .assert()
        .success();

    tracker(&dir)
        .args(["list", "--priority", "high"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "No issues match the given filters.",
        ))
        .stdout(predicate::str::contains("Nice work!").not());
}
```

Both assertions kill the SO Review 11 mutation:
- Positive assertion fails: pre-fix output is `Nice work!`, not `No issues match`.
- Negative assertion fails: pre-fix output contains `Nice work!`.

Test count: layer3.rs grows from 7 → 8 integration tests. Suite total: 53 (11 unit + 16 layer1 + 18 layer2 + 8 layer3). All passing. `cargo clippy -- -D warnings` clean.

This finding mirrors QE Review 7 Finding 3 (`list_nonempty_status_filter_with_no_match_shows_filter_message` for the Layer 2 equivalent of this same heuristic), and applies the same regression-coverage discipline to the new `--priority` path.

---

### Dismissed

**Finding 2 — `list_within_tier_sorted_by_id_ascending` does not exercise the tie-breaker (Dim 2 — Test falsifiability / mutation testing)**

The integration test creates "First high" (id=1) then "Second high" (id=2). Storage order matches sorted order. A mutation that removes the `.then(a.id.cmp(&b.id))` clause from `sort_issues` produces stable sort (preserving original order) — the test still passes because original order already matches the asserted output order.

To kill this mutation at the integration layer, the test would need to insert in reverse ID order, but the CLI assigns IDs sequentially — there is no way to make issue id=2 appear before issue id=1 in storage via the public CLI surface. The mutation is unkillable via integration tests with the current ID assignment contract.

**Classification:** Dismissed. The unit test `priority_sort_tie_breaking_by_id` constructs `vec![issue(2, "high"), issue(1, "high")]` directly and verifies `sort_issues` returns `[1, 2]` — this kills the mutation at the unit level. The integration test serves a different purpose: end-to-end verification that creation order does not corrupt sort order through the full CLI pipeline. Both tests have value; neither makes the other redundant.

---

**Finding 3 — `--priority medium` and `--priority low` filter cases not covered by integration tests (Dim 4 — Coverage meaningfulness)**

Layer 3 acceptance criteria 8 and 9 explicitly enumerate `tracker list --priority medium` and `tracker list --priority low`. Only `--priority high` has an integration test covering the filter path.

**Classification:** Dismissed. The retain logic in `cmd_list` (`src/lib.rs`) is `issues.retain(|i| &i.priority == p)` — pure string equality with no special-casing of priority values. The `list_priority_filter_shows_only_matching` test exercises this code path with `--priority high`. The `parse_priority` unit test (`priority_parsing_valid_cases`) covers all three priority strings — `low`, `medium`, `high` — through the parser. Adding `--priority medium` and `--priority low` integration tests would be tautological: they would exercise the same retain branch with three different equality comparisons, killing no additional mutations beyond the symmetry already established by the parser unit tests. Test economy preferred.

If the implementation ever introduced priority-value-specific branches (e.g., a special filter for `--priority urgent` that mapped to multiple stored values), the symmetry assumption would break and explicit tests for each value would become necessary. The current code does not justify that overhead.

---

**Finding 4 — `--priority HIGH` (uppercase, integration-level) not tested at the CLI surface (Dim 4)**

Layer 3 acceptance criterion 3 requires that `tracker create "X" --priority HIGH` stores `"high"` in JSON. The unit test `priority_parsing_valid_cases` covers `parse_priority("HIGH") == Ok("high")`, but no integration test invokes the CLI with `--priority HIGH`.

**Classification:** Dismissed. Mirrors the Layer 2 status case-insensitivity test pattern (Layer 2 also covers uppercase status via `parse_status` unit test rather than a CLI integration test). The CLI value flows directly from clap into `parse_priority` with no intermediate transformation; the parser unit test is the right level of test for this transform. Adding a CLI integration test would exercise additional plumbing (clap → cmd_create → parse_priority → JSON write) but the failure modes covered are already addressed by `create_with_priority_stores_correct_value` (CLI-to-JSON for `high`) and `priority_parsing_valid_cases` (case-insensitivity for `HIGH` in parser).

---

### Hallucinated

*(none)*

### Open

*(none)*

### Raised to SO

*(none)*

---

### Summary

One real finding resolved: regression test added for the SO Review 11 `is_open_view` fix, closing the regression-coverage gap. Three dismissed: integration tie-breaker test gap (covered by unit test); medium/low filter symmetry (covered by shared retain code path + parser unit test); uppercase `--priority HIGH` integration (covered by parser unit test, mirroring Layer 2 pattern).

**Layer 3 test architecture verdict:** Sound. Total: 53 tests (11 unit + 42 integration). All pass. No flakiness, no shared state, no ordering dependencies. `tempfile::TempDir` per-test isolation preserved. Layer 3 introduced 11 new tests (7 integration + 4 unit) before this review; QE Review 9 added one more for regression coverage.

**Dimensions audited and cleared:**
- **Dim 1 — Acceptance criteria:** All Layer 3 acceptance criteria traced to tests (see table above). No uncovered criterion found.
- **Dim 2 — Test falsifiability:** Mutation analysis on each Layer 3 test (priority parsing, sort, filter, regression) — all relevant mutations either caught or shown unkillable via integration surface (with unit test backstop).
- **Dim 5 — Test architecture:** No flakiness, no order-dependence; `TempDir` isolation maintained.
- **Dim 7 — Logic errors:** SO Review 11 caught and fixed the only logic error (`is_open_view`); no others detected on this pass.
- **Dim 8 — Dead code:** `priority_rank` is private, called only from `sort_issues` (called from `cmd_list`). `PRIORITY_ORDER` is now used in 3 places (post-SA-7 unification). No dead code.
- **Dim 9 — Unused dependencies:** No new dependencies in Layer 3. Existing `serde`, `serde_json`, `clap`, `chrono` all used.
- **Dim 14 — TDD proxy indicators:** Layer 3 Red Gate test names are behavior-named (`create_with_priority_stores_correct_value`, `list_sorts_high_before_medium_before_low`); they call the public CLI surface; assertions are tight on stdout content and JSON structure. Red Gate commit (`71d2137`) precedes implementation commit (`caf5f9a`) — TDD discipline preserved at the commit level.

**Coordination:**
- **SE:** No new SE-owned findings — SO Review 11 already addressed the only logic error in Layer 3.
- **SA:** No structural test-architecture findings beyond the test-helper extraction already raised in SA Review 7 Finding 2 (Open).
- **VDD-IAR Alignment:** Same-session quality tradeoff noted; recommend a follow-up cold-session QE pass before the Layer 3 gate closes if MVR rigor is required.
- **Cargo.lock / coverage CI:** Same status as QE Review 8 Dim 4 carve-out — coverage tooling is a Platform Engineer concern; no Platform Engineer review is scheduled for Layer 3 per `TODO.md`. No new escalation.

---

---

## Review 10 — 2026-05-04 16:00Z

**Scope:** Layer 3 cold-session adversarial pass. Files read: `DESIGN.md`, prior `QUALITY-ENGINEER-REVIEW.md` (Reviews 1–9), `src/lib.rs`, `src/main.rs`, `tests/{common/mod.rs,layer1.rs,layer2.rs,layer3.rs}`, `Cargo.toml`, `Cargo.lock`. `cargo test --all-targets` and `cargo clippy --all-targets -- -D warnings` both run pre-review (54 tests passing, no clippy warnings) and post-review (60 tests passing, no clippy warnings). Mutation-testing thought experiment applied per primer; spec-mandated literal-string assertions audited.

**Session note:** Cold session per primer; parallel batch run with other domains. No prior participation in Layer 1–3 implementation or in-session reviews. Adversarial obligation is to the spec, not the developer or prior reviewers' conclusions.

**Assumption surfacing:** `assert_cmd::Command::cargo_bin`, `predicates::str::{contains, starts_with}`, `tempfile::TempDir`, `serde_json::Value` indexing, `std::fs::create_dir`, `clap::Parser::try_parse` all verified against the versions resolved in `Cargo.lock`. No hallucinated APIs in the new test code.

---

### Resolved

**Finding 1 — `Created issue #<id>: <title>` stdout never tested with a title that requires trimming (Dim 2 — Test falsifiability)**

DESIGN.md Feature 1 postcondition: "stdout prints exactly: `Created issue #<id>: <title>` (trimmed title)". The two existing tests that reach the stdout-print path are `create_valid_title_exits_zero_and_prints_confirmation` (exact stdout assertion, but title `"Fix bug"` requires no trimming) and `create_trims_title` (asserts the JSON file, not stdout). A mutation in `cmd_create` substituting `title_raw` for the locally-bound `title` (post-validate) in the `println!` call survives both: JSON would still hold the trimmed title (because `validate_title`'s return value is what gets pushed into the `Issue`), while stdout would print `"Created issue #1:   Fix bug  "`. Zero existing tests catch this.

**Resolution:** Added `create_stdout_uses_trimmed_title_not_raw` (`tests/layer1.rs`) — runs `tracker create "  Fix bug  "` and asserts stdout is exactly `"Created issue #1: Fix bug\n"` (and stderr empty). Fails on the `title_raw` mutation; passes on the correct implementation.

---

**Finding 2 — `tracker.json` is a directory: DESIGN.md edge case has no automated test (Dim 1 — Acceptance criteria; Dim 6 — Validation gaps)**

DESIGN.md Edge Cases > Storage explicitly enumerates: "`tracker.json` is a directory → read error, treated as I/O failure → exit 1." This is a spec-listed acceptance criterion. QE Review 1 Finding 6 dismissed write-failure paths as manual-only because they require OS-level setup (filling a disk, revoking permissions). The directory-at-tracker.json case is **not** OS-privileged — it requires only `std::fs::create_dir`, which works in any `TempDir` on every OS the project supports. The dismissal in Review 1 conflated this case with the genuinely manual-only ones. Verified manually: `mkdir tracker.json && tracker list` exits 1 with `Error: Could not read tracker data: Is a directory (os error 21).` — current implementation handles it correctly. Without an automated test, a future regression that panicked on `EISDIR` (e.g., reintroducing `.unwrap()` on `fs::read_to_string`) would survive the suite.

**Resolution:** Added `tracker_json_is_a_directory_causes_io_error_exit` (`tests/layer1.rs`) — creates a directory at `tracker.json`, runs `tracker list`, asserts exit 1, stderr starts with `"Error: Could not read tracker data"`, stdout empty. The assertion uses `starts_with` rather than full equality because the OS error suffix (`Is a directory (os error 21)` vs. equivalent on other Unixes / Windows) is platform-dependent — but the spec-mandated `Error: Could not read tracker data` prefix is the falsifiable contract.

---

**Finding 3 — `--help` exit-code-0 contract has no test (Dim 1; CLI supplement Testing Methodology)**

DESIGN.md Interface: "`--help` is supported for the binary and each subcommand. The output must accurately describe all flags and their valid values." The CLI supplement Testing Methodology calls out: "`--help` output: verify it does not crash and exits 0." `main.rs` routes clap errors through a custom branch — `Err(e) if e.use_stderr() => exit(1)` for usage errors; `else => exit(0)` for `--help`/`--version`. This routing is exactly the kind of clap-quirk handler that silently breaks under a clap upgrade or a refactor. Zero existing tests exercise either branch's `exit(0)` path.

**Resolution:** Added two tests (`tests/layer1.rs`):
- `help_flag_exits_zero_and_lists_subcommands` — `tracker --help` exits 0 with stdout containing `create`, `list`, `status` (the registered subcommands).
- `subcommand_help_flag_exits_zero` — `tracker create --help` exits 0 with stdout containing `--priority` (a documented flag).

Either test fails if `--help` ever routes through the error branch (exit 1 with usage error on stderr instead of help text on stdout).

---

**Finding 4 — Spec-literal stderr assertions reduced to substring checks across 6 error-path tests (Dim 3 — Assertion strength; primer literal-string mandate)**

The primer mandates: "Spec-mandated assertions should be tested literally (full message, not substring)." DESIGN.md specifies the full text for each error message; the existing tests assert only loose substrings. Affected tests and the spec-vs-test gap:

| Test | Existing assertion | Spec-mandated full text |
|---|---|---|
| `status_invalid_id_string_exits_one` | `contains("not a valid issue ID")` | `Error: 'abc' is not a valid issue ID. Expected a positive integer.` |
| `status_zero_id_exits_one` | `contains("not a valid issue ID")` | `Error: '0' is not a valid issue ID. Expected a positive integer.` |
| `status_invalid_value_exits_one` | `contains("Invalid status")` | `Error: Invalid status 'flying'. Expected: open, in-progress, or done.` |
| `list_invalid_status_filter_exits_one` | `contains("Invalid status")` | `Error: Invalid status 'flying'. Expected: open, in-progress, or done.` |
| `create_invalid_priority_exits_one` | `contains("Invalid priority")` | `Error: Invalid priority 'critical'. Expected: low, medium, or high.` |
| `list_invalid_priority_filter_exits_one` | `contains("Invalid priority")` | `Error: Invalid priority 'urgent'. Expected: low, medium, or high.` |

Mutations that survive `contains("Invalid status")` include: dropping the offending value (regression to a static string `"Invalid status."`), dropping the actionable expected-list suffix, or accidentally reusing the priority error template for status validation (or vice versa). The spec wrote these as user-actionable contracts; the tests treat them as opaque "an error occurred" labels.

**Resolution:** Updated all six tests to assert the full spec-mandated stderr text via `predicate::str::contains("Error: <full message>")`. The leading `Error: ` prefix is included so the assertion would also catch a regression that bypassed the `eprintln!("Error: {}", e)` wrapping in `main.rs`. All 60 tests pass.

---

**Finding 5 — `status_is_case_insensitive_on_input` verifies stored value but not stdout normalization (Dim 2 — Test falsifiability)**

The test runs `tracker status 1 DONE`, then asserts `v[0]["status"] == "done"` in JSON. It does not assert stdout. `cmd_status` prints the stored (lowercase) status via `issues[idx].status`. A mutation substituting `status_raw` for `issues[idx].status` in the `println!` would print `"Issue #1 status → DONE.\n"` while still writing `"done"` to JSON. The test passes on the mutated implementation — the case-insensitivity contract is asserted only at the storage boundary, not at the user-visible stdout boundary.

**Resolution:** Added `.stdout("Issue #1 status \u{2192} done.\n")` to the existing assertion chain. The test now verifies that both the stored value and the printed confirmation reflect the normalized lowercase form. All 60 tests pass.

---

### Dismissed

**Finding 6 — `list_default_excludes_done_issues` does not specifically assert the empty-state message (Dim 3)**

The test creates one issue, marks it done, runs `tracker list`, and asserts `contains("Fix bug").not()`. The actual stdout in this case is `"No open issues. Nice work!\n"` (the only open-tracker-empty path). A mutation that changed the empty-state message (e.g., swapped `"Nice work!"` for `"All clean!"`) would survive this test.

**Classification:** Dismissed. The empty-state message under exactly this scenario (all issues done → default list view) is the explicit contract of `list_all_done_default_shows_empty_state` (`tests/layer2.rs`), which uses an exact-stdout assertion. `list_default_excludes_done_issues` has a different, narrow purpose: verifying that a done issue is not shown in the default view. Loading it with the empty-state message assertion would create a test with two unrelated falsifiability claims; cleanly separated tests are preferable.

---

**Finding 7 — No integration test for `--priority HIGH` (uppercase) end-to-end CLI path (Dim 4)**

QE Review 9 Finding 4 was dismissed on the grounds that `parse_priority("HIGH")` is unit-tested and the CLI surface is "thin plumbing." On cold-session re-examination: the dismissal is defensible because (a) `cmd_create` calls `parse_priority` directly with the clap-supplied string — no intermediate transformation; (b) the `create_with_priority_stores_correct_value` test exercises the CLI-to-JSON path for `"high"`; (c) a regression that broke uppercase normalization would fail `priority_parsing_valid_cases`. Adding a CLI-level uppercase test would duplicate coverage with no additional mutation-killing power.

**Classification:** Dismissed (consistent with QE Review 9 Finding 4).

---

**Finding 8 — Coverage measurement infrastructure absent (Dim 13; Rust supplement Coverage thresholds)**

`cargo tarpaulin`, `cargo llvm-cov`, and `cargo mutants` are not installed; no coverage gate is configured in the repo. The Rust supplement specifies 80% line coverage minimum, 100% public API coverage, both enforced in CI.

**Classification:** Dismissed from QE — already escalated to Platform Engineer in QE Review 8 Dim 4 carve-out. No new action.

---

### Hallucinated

*(none — every finding was verified against the running code or executed manually.)*

---

### Open

*(none)*

---

### Summary

Five real findings, all resolved this session via test additions/strengthening (+4 tests in layer1.rs, in-place strengthening of 6 assertions in layer2.rs/layer3.rs, in-place strengthening of `status_is_case_insensitive_on_input`). Three dismissed with rationale. Test count: 54 → 60. `cargo test --all-targets` green; `cargo clippy --all-targets -- -D warnings` green.

**The two cold-session-specific findings:**
- Finding 2 (directory-at-tracker.json) was a long-standing acceptance-criterion gap dismissed in QE Review 1 Finding 6 under the conflated "I/O failure" umbrella. Cold-session re-examination separated the cross-platform-testable directory case from the genuinely manual-only permission/disk-full cases. This is the kind of dismissal-without-rationale-recheck the primer flags as cold-session value.
- Finding 4 (substring assertions across six error-path tests) reflects the primer's explicit literal-string mandate. Each affected test was authored and reviewed in prior in-session passes; cold-session adversarial pressure was required to force the literal-vs-substring framing.

**Dimensions audited and cleared:**
- **Dim 1 — Acceptance criteria:** All Layer 1–3 acceptance criteria traced to tests post-additions. Directory-at-tracker.json case now covered.
- **Dim 2 — Test falsifiability:** Mutation thought experiment applied to `cmd_create` (title-raw vs. trimmed), `cmd_status` (status-raw vs. normalized), `is_default_open_view`, `truncate_with_ellipsis`, `priority_rank`, `sort_issues`. All identified surviving mutations now caught.
- **Dim 3 — Test selector and assertion strength:** Six substring-only assertions tightened to spec-literal text.
- **Dim 5 — Test architecture:** TempDir per-test isolation maintained; new tests follow the established `mod common; use common::tracker;` pattern. No flakiness.
- **Dim 7 — Logic errors:** No new logic errors found; SO Review 11's `is_default_open_view` fix verified untouched and now regression-tested via QE Review 9 Finding 1.
- **Dim 8 — Dead code:** `priority_rank`, `truncate_with_ellipsis`, `issue_fields_are_valid` private and reachable via `cmd_list` / `load_issues`. No dead exports.
- **Dim 9 — Unused dependencies:** All four runtime deps (`serde`, `serde_json`, `clap`, `chrono`) and three dev deps (`assert_cmd`, `predicates`, `tempfile`) used.
- **Dim 12 — Regression coverage:** Every prior-review-logged bug now has an identifiable regression test (most recently QE Review 9 Finding 1's regression for SO Review 11).
- **Dim 14 — TDD proxy indicators:** Tests are behavior-named, call public CLI surface, assert tight contracts. No implementation coupling.

**Files modified:**
- `tests/layer1.rs` — added 4 tests (`create_stdout_uses_trimmed_title_not_raw`, `tracker_json_is_a_directory_causes_io_error_exit`, `help_flag_exits_zero_and_lists_subcommands`, `subcommand_help_flag_exits_zero`).
- `tests/layer2.rs` — strengthened 4 tests (`status_invalid_id_string_exits_one`, `status_zero_id_exits_one`, `status_invalid_value_exits_one`, `list_invalid_status_filter_exits_one`, `status_is_case_insensitive_on_input`).
- `tests/layer3.rs` — strengthened 2 tests (`create_invalid_priority_exits_one`, `list_invalid_priority_filter_exits_one`).

**Coordination:**
- **SO:** Empty-state messages on stdout (QE Review 8 Finding 3) remain Raised to SO — not re-raised but noted as still pending.
- **PE:** Coverage tooling absence remains escalated (QE Review 8 Coordination) — no new escalation.
- **No new domain coordination required.**

---

---

## Review 11 — 2026-05-05 22:30Z

**Scope:** Layer 4 (`--label` on `create` and `list`) on the `issue-tracker-cli-labels` branch. Files read: `DESIGN.md`, `PROCESS.md`, `src/lib.rs`, `src/main.rs`, `tests/{common/mod.rs,layer1.rs,layer2.rs,layer3.rs,layer4.rs}`, prior `QUALITY-ENGINEER-REVIEW.md` Reviews 8/9/10, `iterative-adversarial-refinement/SECURITY-REVIEW.md` Review 7. `cargo test --all-targets`, `cargo clippy --all-targets -- -D warnings`, and `cargo fmt --check` all run pre- and post-review. Pre-review: 98 tests pass clean. Post-review: 100 tests pass clean (added 1 test, strengthened 2). Mutation analysis applied per primer's literal-string mandate; secondary regression sweep over Layers 1-3 produced no new findings.

**Session note:** Cold session (subagent context, not the implementer's session) per primer; session-isolated parallel batch with other Tier-2/3 domains. No prior participation in Layer 4 implementation, Red Gate authoring, or in-session reviews.

**Red Gate verdict (Layer 4):** Compliant at the commit-pattern level. Commit `14bd219` ("Layer 4 Red Gate — labels tests and stubs", 2026-05-05 11:19 PDT) introduced 12 integration tests in `tests/layer4.rs` and 3 unit tests in `src/lib.rs`, with `parse_label`, `dedupe_labels`, and `label_matches` as `todo!()` stubs and no `--label` clap arg. The Red Gate state was confirmed (10 integration failures from clap unknown-arg; 3 unit failures from `todo!()` panics; 2 explicitly logged Cat B deviations against existing Layer 1 defaults — `create_without_labels_stores_empty_array` and `list_shows_none_for_no_labels`). Implementation commit `ec5c966` ("Layer 4 implementation — --label on create + list") followed. The Cat B deviations are correctly classified as regression coverage rather than Red Gate tests for new behavior, mirroring Layer 3's `create_without_priority_defaults_to_medium`. **Verdict: Red Gate satisfied for Layer 4.**

**Assumption surfacing:** `assert_cmd::Command::cargo_bin`, `predicates::str::contains/.not()`, `tempfile::TempDir`, `serde_json::Value` indexing/`json!` macro, `clap::Parser` with `Vec<String>` (repeatable) vs. `Option<String>` (single) all verified against versions resolved in `Cargo.lock`. `clap`'s default behavior for `Option<String>` flag passed twice — emit usage error with text `"the argument '--label <LABEL>' cannot be used multiple times"` — confirmed by direct execution against the compiled binary. No hallucinated APIs in new test code.

### Layer 4 acceptance criteria → test trace

| Criterion (DESIGN.md Feature 1 / Feature 2 / Edge Cases / Labels) | Test(s) | Verdict |
|---|---|---|
| `--label X` stores `["X"]` | `create_with_label_stores_label` | ✓ |
| `--label X --label Y` stores `["X","Y"]` (insertion order) | `create_with_multiple_labels_stores_all` | ✓ |
| `--label X --label X` deduplicated to `["X"]` | `create_with_duplicate_labels_deduplicates` | ✓ |
| no `--label` flag → `labels: []` | `create_without_labels_stores_empty_array` (Cat B) | ✓ |
| `--label ""` → exit 1, `Error: Label cannot be empty.` | `create_with_empty_label_exits_one` (literal stderr) | ✓ |
| `--label "  "` → exit 1, `Error: Label cannot be empty.` | `create_with_whitespace_label_exits_one` | ✓ |
| labels rendered comma-separated in `list` | `list_shows_labels_comma_separated` | ✓ |
| `(none)` rendered for empty labels | `list_shows_none_for_no_labels` (Cat B) | ✓ |
| `Labels` column truncates at 20 chars with `…` | `list_label_value_truncated_at_20_chars` | ✓ |
| `tracker list --label X` shows only matching | `list_label_filter_shows_matching` | ✓ |
| `--label Bug` does NOT match `bug` (case-sensitive) | `list_label_filter_is_case_sensitive` | ✓ |
| multiple `--label` flags on `list` → exit 1 | `list_multiple_label_flags_exits_one` (strengthened — Finding 2) | ✓ |
| labels with control characters → rejected | **NO TEST** (see Open Finding 4) | ✗ (depends on SE/SO closing Sec R7 F1) |
| label case preserved at storage as provided | `create_preserves_label_case_at_storage` (added — Finding 1) | ✓ |
| AND-combined `--status` / `--priority` / `--label` | DESIGN.md line 313 example covered at Layer 5 per TODO; Layer 4 has no integration test (Open Finding 5) | partial |

---

### Resolved

**Finding 1 — Label case preservation at storage has no falsifiable test (Dim 2 — Test falsifiability)**

DESIGN.md Feature 1 postcondition (line 28) specifies: "labels is the deduplicated list of `--label` values; order is preserved, **case is preserved as provided**". A mutation in `parse_label` (`src/lib.rs:339-346`) that lowercased the trimmed value before returning — `Ok(trimmed.to_lowercase())` instead of `Ok(trimmed.to_string())` — would survive every existing Layer 4 test:

- `create_with_label_stores_label` uses lowercase `bug`; `assert_eq!(v[0]["labels"], json!(["bug"]))` passes either way.
- `create_with_multiple_labels_stores_all` uses lowercase `bug`, `auth`; same.
- `create_with_duplicate_labels_deduplicates` uses lowercase `bug`, `auth`; same.
- `list_label_filter_is_case_sensitive` uses `--label Bug` (filter side, mismatched case) — under the mutation, the stored label is now `"bug"` and the filter is `"Bug"` — the test still passes (no match expected and observed).

The mutation is undetected. The user-visible contract — "case is preserved as provided" — has zero test coverage at the integration level.

**Resolution:** Added `create_preserves_label_case_at_storage` (`tests/layer4.rs`):

```rust
tracker(&dir)
    .args(["create", "x", "--label", "Bug", "--label", "BUG", "--label", "bug"])
    .assert()
    .success();

let raw = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
assert_eq!(v[0]["labels"], serde_json::json!(["Bug", "BUG", "bug"]));
```

The assertion fails on the `to_lowercase` mutation (output would be `["bug"]` after dedup) and also fails on a `to_uppercase` mutation, on a mutation that case-folded `dedupe_labels`'s comparator, or on a mutation that returned only the first-seen label. Tests pass on the correct implementation. All 100 tests pass.

---

**Finding 2 — `list_multiple_label_flags_exits_one` asserts only `contains("Error:")` — too lax to detect routing regressions (Dim 3 — Assertion strength)**

The test asserts `predicate::str::contains("Error:")` for the multiple-`--label` case. Every error path in the binary begins with `Error:` (the global stderr contract added in `main.rs:62`), so this assertion is satisfied by literally any failure-with-stderr — including a regression that routed the multiple-flag case to a generic "unexpected argument" handler, dropped the offending flag name from the message, or invoked a completely different validation path with the same exit code. Direct execution against the compiled binary confirms the actual clap message is:

```
Error: the argument '--label <LABEL>' cannot be used multiple times
```

The flag name and the "cannot be used multiple times" phrase are user-actionable diagnostics per CLI supplement Dim 8 ("error message quality: what failed, why, and what next"). Asserting on them kills routing mutations and detects accidental upgrades that change clap's default error format.

**Resolution:** Tightened to `predicate::str::contains("Error: the argument '--label <LABEL>' cannot be used multiple times")`. Test passes on current implementation; would fail under any of the regressions enumerated above. All 100 tests pass.

---

**Finding 3 — `list_label_filter_is_case_sensitive` lacks the negative `Nice work!` assertion that QE Review 9 Finding 1 established for `--priority` (Dim 12 — Regression coverage)**

`cmd_list`'s `is_default_open_view` heuristic at `src/lib.rs:414-415` reads:

```rust
let is_default_open_view =
    effective_status == "open" && effective_priority.is_none() && label_filter.is_none();
```

A mutation removing `&& label_filter.is_none()` causes `tracker list --label Bug` (with one open issue labelled `bug`) to print `"No open issues. Nice work!"` instead of `"No issues match the given filters."`. The current `list_label_filter_is_case_sensitive` test asserts `stderr.contains("No issues match the given filters.")` — which does fail on this mutation (`Nice work!` does not contain that substring), so the regression IS caught at the positive-assertion side.

But the symmetric pattern QE Review 9 Finding 1 established for the `--priority` empty-state regression — `.stderr(contains("No issues match...")).stderr(contains("Nice work!").not())` — provides defense-in-depth: it would fail loudly on a future mutation that produced both messages (e.g., a fall-through case in a refactored conditional) and aligns the Layer 4 regression-coverage discipline with Layer 3's. The PROCESS.md Layer 3 retrospective explicitly called this lens out: "Layer 4 (labels) and Layer 5 (compound filters) need this lens applied at Red Gate time, not at SO review time" (PROCESS.md line 203). The Red Gate did not apply it; this finding closes that gap.

**Resolution:** Added `.stderr(predicate::str::contains("Nice work!").not())` to `list_label_filter_is_case_sensitive`. All 100 tests pass.

---

### Open

**Finding 4 — No test for label control-character rejection; depends on SE / SO closing Security Review 7 Finding 1 (Dim 1 — Acceptance criteria; Dim 12 — Regression coverage)**

Security Review 7 Finding 1 (2026-05-05 21:35Z, Open, Raised to SE/QE/SO) demonstrated that labels containing control characters (newline, ESC, tab) corrupt `tracker list` output and enable terminal-escape injection — the same attack class the title control-char defense (Review 1, resolved) was designed to mitigate. The fix path requires:

1. **SO:** amend DESIGN.md "Edge Cases / Labels" (lines 302-306) to add `- Label containing a control character (Unicode general category Cc) → error: Label cannot contain control characters.` and extend the storage-invalid-domain-values list at line 325.
2. **SE:** extend `parse_label` (`src/lib.rs:339-346`) and `issue_fields_are_valid` (`src/lib.rs:131`) to enforce `!chars().any(char::is_control)`.
3. **QE:** add unit tests mirroring the title control-char tests (`label_with_newline_is_rejected`, `label_with_tab_is_rejected`, `label_with_escape_sequence_is_rejected`, `label_with_nul_or_del_is_rejected`, `label_with_printable_unicode_is_accepted`), plus an integration test for `tracker create "x" --label $'bug\nFAKE'` exits 1, plus a `tracker.json` corruption test for `"labels": ["bug\nFAKE"]`.

QE cannot apply the test additions yet: per CLOSURE-PROTOCOL.md, QE may not modify `src/**/*.rs` (so the unit tests would have nothing to call against — `parse_label` does not yet have the check), and the integration test for the create path would fail (the implementation does not yet reject the input). Authoring tests against not-yet-implemented behavior would violate Red Gate discipline — the tests would correctly be Red, but they would block the regression-test merge gate until SE applies the fix, by which point the QE fix needs to land in the same atomic step.

**Classification: Open. Raised to SE / Raised to SO.** Discipline: when SE applies the parse_label / issue_fields_are_valid extension and SO amends DESIGN.md, QE adds the test in the same change set (or in a follow-up immediately after SE's merge). Until then, the Layer 4 test suite has a known coverage gap on a Security-classified finding. This finding **must close before the Layer 4 merge gate** — security findings cannot be deferred per the IAR domain prompt.

If SE later commits the fix without amending the test suite, this finding stands as an audit signal that the regression-test discipline broke down.

---

**Finding 5 — DESIGN.md AND-logic example (`tracker list --status open --priority high --label bug`) has no Layer 4 integration test (Dim 1 — Acceptance criteria; Dim 4 — Coverage meaningfulness)**

DESIGN.md "Edge Cases / List" line 313 explicitly lists: `tracker list --status open --priority high --label bug → AND-logic; only issues matching all three`. The implementation in `cmd_list` (`src/lib.rs:418-424`) AND-combines all three filters via three sequential `retain` calls. No Layer 4 test exercises the three-filter compound case at the CLI integration surface. `TODO.md` defers full compound-filter verification to Layer 5 (per QE Review 9's trace), but DESIGN.md treats the example as a Layer 4-applicable spec line — the `--label` filter is the new piece, and the spec example uses it in a compound.

A mutation removing the second or third `retain` (e.g., dropping the priority filter while keeping status and label) would survive every Layer 4 test: each test exercises only one or two filters at most. `list_label_filter_shows_matching` uses no `--status` or `--priority`; `list_label_filter_is_case_sensitive` uses only `--label`.

**Classification: Open / Deferred to Layer 5.** Defensible deferral if and only if Layer 5 is committed to introducing a compound-filter test that includes `--label`. This finding is the marker — Layer 5 must produce `list_status_priority_label_compound_AND_filter` (or equivalent) covering the spec line 313 example. If Layer 5's TODO does not enumerate this test before Red Gate, the deferral has slipped and a same-layer test addition is required.

This is corroborating evidence for VDD-IAR Alignment Dim 1 (spec → test traceability): a spec example explicitly enumerated in DESIGN.md is currently uncovered by any test in the suite.

---

### Dismissed

**Finding 6 — `list_shows_labels_comma_separated` does not assert exact column alignment of the comma-separated value (Dim 3)**

The test asserts `out.contains("bug, auth")` and nothing else. A mutation in the format string that emitted "bug,auth" (no space) or "bug | auth" (different separator) would fail. A mutation in column padding that produced "bug, auth   " with wrong padding would still satisfy the substring assertion.

**Classification: Dismissed.** Layer 3's `list_columns_use_exactly_two_space_separator` (`tests/layer3.rs:208-240`) already asserts the column-spacing contract for a full-width Status column. The comma-separator test is correctly scoped to its own falsifiability claim (the Labels rendering, not the column padding); separating the two contracts into focused tests is the established pattern (see QE Review 10 Finding 6 dismissal of `list_default_excludes_done_issues` for the same reason). Adding column-alignment assertions here would conflate two contracts.

---

**Finding 7 — `list --label ""` (empty filter on list) is undocumented and untested (Dim 6 — Validation gaps)**

DESIGN.md Feature 2 says `--label <l>` "shows only issues that have that label (exact match, case-sensitive)" and does not address the empty-string case explicitly. Manual testing confirms current behavior: `tracker list --label ""` exits 0 with `No issues match the given filters.` on stderr — consistent with the spec by interpretation (no stored label can be empty due to `parse_label` validation, so an empty filter trivially matches nothing). No test asserts this.

**Classification: Dismissed.** Behavior is consistent with the spec and harmless. The reverse case (`--label ""` on `create`) IS rejected with `Error: Label cannot be empty.` (`create_with_empty_label_exits_one`). Asymmetric validation is intentional: create-time empty labels are corrupt input; list-time empty labels are a degenerate filter that legitimately matches no records. If future spec evolution requires the create-time rejection to apply to list as well (consistency, defense-in-depth), a test addition becomes warranted; under the current spec, this is a non-finding.

---

**Finding 8 — Coverage measurement infrastructure absent (Dim 13; Rust supplement Coverage thresholds)**

Same status as QE Reviews 8 and 10: no `cargo tarpaulin`, `cargo llvm-cov`, or `cargo mutants`; no CI coverage gate. Already escalated to Platform Engineer.

**Classification: Dismissed from QE — escalation carry-forward.** No new action.

---

**Finding 9 — `create_preserves_label_case_at_storage` does not assert `cmd_create`'s stdout uses the trimmed-but-case-preserved label in any rendered output (Dim 2)**

The `cmd_create` stdout is `Created issue #<id>: <title>` — labels are never echoed on `create`'s stdout. So this is structurally impossible to test; there is no rendered-label sink at create time. Labels appear in `list` (covered by `list_shows_labels_comma_separated`) and will appear in `show` (Layer 6).

**Classification: Dismissed.** Hallucinated stricter-than-spec assertion — the spec does not echo labels on create, and the test correctly verifies the storage boundary, which is the only sink at this layer.

---

### Hallucinated

**Finding 10 — `parse_label` should reject labels with leading/trailing whitespace as a normalization concern**

Concern: `parse_label("  bug  ")` returns `"bug"` (trimmed) but `parse_label(" bug ")` and `parse_label("bug")` would both produce the same stored value, allowing two `--label "  bug"` and `--label "bug"` flags to be treated as duplicates after dedup. A test should verify that ` bug ` and `bug` are case-and-whitespace-identical post-normalization.

**Classification: Hallucinated.** This is exactly the documented behavior — `parse_label` trims, and `dedupe_labels` deduplicates the trimmed values. The behavior is correct (otherwise `--label bug` and `--label "bug "` would store as two distinct labels, contrary to user intent). The "concern" is the desired behavior, not a bug. `create_with_duplicate_labels_deduplicates` already covers the duplicate-after-trim case implicitly; adding a whitespace-around-duplicate variant would be tautological with no additional mutation-killing power.

---

**Finding 11 — `--label` filter should be trimmed on the list side too (else `tracker list --label "bug "` matches nothing)**

Concern: filter-side trimming would produce more user-friendly behavior; absence of trimming on filter creates an asymmetry with create-time trimming.

**Classification: Hallucinated.** The spec explicitly says `--label X` is "exact match, case-sensitive" (DESIGN.md Feature 2). Trimming the filter would violate "exact match." Asymmetry between create-time normalization and list-time strict equality is intentional: create-time enforces stored data hygiene; list-time honors what the user typed. No quality finding here. (Was raised because of an in-session manual test where I confirmed `tracker list --label "  bug"` matches nothing — but that's the spec.)

---

### Summary

Three findings resolved inline (case preservation test added; multiple-flag error message strengthened; symmetric `Nice work!` negative assertion added). Two Open: (4) label control-char rejection has no test pending SE/SO closure of Security Review 7 Finding 1 — security findings cannot be deferred per IAR prompt, **must close before Layer 4 merge gate**; (5) compound-filter AND-logic spec line 313 has no integration test, deferred to Layer 5 with the explicit marker that Layer 5 Red Gate must enumerate the test before merge or the deferral has slipped. Four Dismissed (separator coverage by Layer 3 column test; empty `--label ""` filter is consistent-with-spec; coverage tooling escalated to PE; create-time labels structurally absent from stdout). Two Hallucinated (whitespace-around-duplicate is intended behavior; filter-side trimming would violate exact-match spec).

**Test count:** 100 tests pass post-review (28 unit + 32 layer1 + 18 layer2 + 9 layer3 + 13 layer4). Pre-review: 98. Net delta: +1 new test, +2 strengthened assertions. `cargo test --all-targets`, `cargo clippy --all-targets -- -D warnings`, `cargo fmt --check` all green.

**Red Gate compliance verdict for Layer 4:** **Compliant.** Commit ordering verified (`14bd219` Red Gate precedes `ec5c966` implementation); 12 integration + 3 unit tests confirmed Red against stubs at Red Gate time; 2 Cat B deviations explicitly logged with the Layer 3 precedent. The Cat B classification is honest — `create_without_labels_stores_empty_array` and `list_shows_none_for_no_labels` test pre-existing Layer 1 defaults, not new Layer 4 behavior, and the commit message says so. Test names are behavior-named; assertions call the public CLI surface; no implementation coupling.

**Dimensions audited and cleared:**
- **Dim 1 — Acceptance criteria:** 13 of 14 spec contracts traced to tests; 1 gap (label control-char) is Open Finding 4 awaiting upstream resolution; 1 partial (compound AND-filter) is Open Finding 5 deferred to Layer 5 with marker.
- **Dim 2 — Test falsifiability:** Mutation analysis on `parse_label`, `dedupe_labels`, `label_matches`, `cmd_list`'s three `retain` calls, and `is_default_open_view`. All identified surviving mutations now caught (case-preservation mutation killed via Finding 1; AND-filter retain-removal flagged via Finding 5 deferral).
- **Dim 3 — Test selector and assertion strength:** Multiple-flag error message tightened from `contains("Error:")` to the actual clap text. All other layer-4 spec error messages are already literal-asserted (per QE Review 10's broader sweep).
- **Dim 5 — Test architecture:** TempDir-per-test isolation maintained; new tests follow `mod common; use common::tracker;` pattern. No flakiness, no order dependence, no shared state.
- **Dim 7 — Logic errors:** None new in Layer 4. SO Review 11's `is_default_open_view` heuristic correctly extended in Layer 4 to include `label_filter.is_none()` (`src/lib.rs:415`).
- **Dim 8 — Dead code:** All exports reachable. `parse_label`, `dedupe_labels`, `label_matches` all called from `cmd_create` / `cmd_list`. No dead code.
- **Dim 9 — Unused dependencies:** No new dependencies in Layer 4 (`Cargo.toml` and `Cargo.lock` byte-identical to `main` per Security Review 7). Existing four runtime + three dev deps all in use.
- **Dim 12 — Regression coverage:** Every Layer 1-3 review-logged bug retains its regression test (re-verified by running prior tests under post-Layer-4 source). Layer 4 SO Review 11-class pattern (empty-state heuristic regression) now symmetric across `--status`, `--priority`, and `--label` paths post-Finding 3.
- **Dim 14 — TDD proxy indicators:** Layer 4 Red Gate test names are behavior-named (`create_with_label_stores_label`, `list_label_filter_is_case_sensitive`); they call the public CLI surface; assertions are tight on stdout/stderr literal text where the spec mandates literals. Red Gate commit precedes implementation commit. No implementation coupling detected.

**Files modified:**
- `tests/layer4.rs` — added `create_preserves_label_case_at_storage`; strengthened `list_multiple_label_flags_exits_one` (clap-message-text assertion); strengthened `list_label_filter_is_case_sensitive` (negative `Nice work!` assertion).

**Coordination:**
- **SE:** Open Finding 4 requires `parse_label` + `issue_fields_are_valid` extension to reject control chars. Once applied, a follow-up QE pass adds the symmetric label control-char tests (5 unit + 1 integration + 1 corruption — pattern enumerated in Security R7 Finding 1).
- **SO:** Open Finding 4 requires DESIGN.md amendment to "Edge Cases / Labels" and "Edge Cases / Storage" lines 302-306 / 325 (specific edits in Security R7 Finding 1).
- **VDD-IAR Alignment:** Layer 4 merge gate must verify Open Finding 4 is closed (security finding — cannot be deferred). Open Finding 5 may be merged with the explicit Layer 5 Red Gate marker.
- **PE:** Coverage tooling absence remains escalated; no new escalation.
- **No new domain coordination beyond the existing Security R7 chain.**

---

## Review 12 — 2026-05-06 02:30Z

**Round:** QE Review 12 (Round-2 closure for Layer 4)
**Scope:** Add test coverage for the Round-1 cluster fixes per SO Review 17 + SE Review 12. QE owns `tests/**/*.rs` per CLOSURE-PROTOCOL.md.
**Session context:** Warm-resolution session paired with SE Review 12. Tests were added in the same commit as the source fixes (`67ef920`) so the change set is coherent — Red Gate discipline is intact at the commit level (the tests are Red without the source fix, Green with it; verified by reverting the source change locally and confirming each new test fails as designed).

### Resolved

#### Finding 4 (Round-1) — Label control-character test coverage

Added to `tests/layer4.rs`:
- `create_with_control_char_label_exits_one` — `--label $'bug\nFAKE'` rejected with the spec-literal `Error: Label cannot contain control characters.`.
- `create_with_escape_sequence_label_exits_one` — ESC sequence rejected.
- `create_with_comma_label_exits_one` — comma rejected with spec-literal `Error: Label cannot contain a comma.` (UX R6 F4 follow-up).
- `corrupt_data_with_control_char_label_is_rejected` — hand-edited `tracker.json` with `\n` in a label is rejected at load with `Could not read tracker data...`.
- `corrupt_data_with_comma_label_is_rejected` — same for comma.

Added to `src/lib.rs#tests`:
- `label_with_newline_is_rejected`, `label_with_tab_is_rejected`, `label_with_escape_sequence_is_rejected`, `label_with_nul_or_del_is_rejected`, `label_with_comma_is_rejected`, `label_with_printable_unicode_is_accepted` — mirror the title control-char unit tests.
- `issue_field_validation_rejects_control_char_in_label`, `issue_field_validation_rejects_comma_in_label`, `issue_field_validation_accepts_clean_label` — load-time validator coverage.

**Resolved.**

#### Finding (new) — Filter-side validation tests (UX R6 F1 / SO R16 F2)

Added to `tests/layer4.rs`:
- `list_label_filter_is_trimmed_to_match_stored` — `tracker list --label "  bug  "` matches a stored `bug`.
- `list_empty_label_filter_exits_one` — empty filter rejected with `Error: Label cannot be empty.` (symmetric with create).
- `list_whitespace_label_filter_exits_one` — whitespace-only filter rejected.
- `list_control_char_label_filter_exits_one` — control char in filter rejected (defense in depth).

#### Finding (new) — Error-formatter escape interpolation tests (RT R6 F2)

Added to `tests/layer4.rs`:
- `invalid_priority_with_escape_chars_is_escaped_in_error` — ESC sequence in `--priority` value renders as `\u{1B}` literal in stderr; raw ESC byte (0x1B) MUST NOT appear (`predicate::str::contains("\u{1B}").not()`).
- `invalid_status_with_newline_is_escaped_in_error` — newline in `<status>` argument renders as `\u{A}`; no embedded newline in stderr.
- `invalid_id_with_escape_chars_is_escaped_in_error` — ESC in `<id>` argument escaped.

Added to `src/lib.rs#tests`:
- `display_safe_passes_printable_chars_through` — printable Unicode unchanged.
- `display_safe_escapes_control_chars` — Cc → `\u{XX}` round trip.

#### Finding 5 (carried) — Compound-filter AND-logic test (DESIGN.md line 313)

Unchanged from Round 1: still deferred to Layer 5 with the named marker (Layer 5 Red Gate must enumerate `list_status_priority_label_compound_AND_filter` or equivalent). Round-2 source changes do not alter this disposition.

### Verification

`cargo test --locked` — **123 pass / 0 fail** (39 unit + 32 layer1 + 18 layer2 + 9 layer3 + 25 layer4). Up from 100. Pre-source-fix Red verification: locally reverted `parse_label` and `display_safe` changes, confirmed each new test fails for the right reason, then re-applied source fix and confirmed all 123 pass. Mutation-test discipline intact.

### Open

#### Finding 5 (carried) — Compound-filter test deferred to Layer 5

Unchanged. The Layer 5 Red Gate must include the test or this deferral has slipped.

### Files modified

- `tests/layer4.rs` — +12 integration tests.
- `src/lib.rs#tests` — +11 unit tests.

---

## Review 13 — 2026-05-07 00:24Z

**Round:** QE Review 13 (Layer 5 — Compound Filtering)
**Scope:** Layer 5 lands on the `issue-tracker-cli-compound-filtering` branch in three commits — `7d1ca57` (Phase 2a Red Gate: 7 integration tests + 5 unit tests + `issue_matches_filters` `todo!()` stub), `bd15a9d` (Phase 2b: predicate body + `cmd_list` collapses three `retain` calls into one), `da0fd8d` (manual testing checklist completion). Files read: `DESIGN.md`, `TODO.md` lines 239-275, `tests/layer5.rs`, `tests/layer{1,2,3,4}.rs`, `src/lib.rs` Layer 5 surface (`issue_matches_filters` + refactored `cmd_list`), `src/lib.rs#tests` Layer 5 block (lines 851-948), prior QE Reviews 10/11/12 for prior-finding context. Full-suite verification: `cargo test --no-fail-fast --locked` → **135 passed / 0 failed** (49 unit + 32 layer1 + 18 layer2 + 9 layer3 + 25 layer4 + **7 layer5**). Pre-review state: 135 passed. Net delta from this review: 0 source/test changes (no findings warrant inline resolution; see Open / Dismissed / Hallucinated below).

**Session note:** Cold session, parallel batch with SO Review 18 / SA Review 11 / SE Review 13 / VDD-IAR Review 13. No prior participation in Layer 5 implementation, Red Gate authoring, or in-session reviews. Carried-forward marker from Review 12: Open Finding 5 ("Compound-filter test deferred to Layer 5") closes via Layer 5's Red Gate inclusion of `list_three_filter_and_combination`; the deferral was honored (see AC mapping below).

**Red Gate verdict (Layer 5):** Compliant. The Red Gate commit `7d1ca57` introduced (a) 7 integration tests in `tests/layer5.rs` explicitly self-classified as "Cat B Red Gate deviations" — the AND-combination is an emergent property of Layers 3-4's chained `retain()` calls, so the integration tests pass against the unrefactored implementation as regression coverage; (b) 5 unit tests against an `issue_matches_filters` `todo!()` stub that genuinely panic at Red Gate time. The Phase-2a-only `#[allow(dead_code)]` on the stub is honest about why the predicate is not yet wired into `cmd_list`. Implementation commit `bd15a9d` replaces the stub body with the AND predicate and collapses the three chained `retain` calls into one over the predicate, preserving observable behavior. The Cat B classification is itself adversarially honest — see Dim 2 audit below.

**Assumption surfacing:** `Option::is_none_or` (stable since Rust 1.82, present in this MSRV per `Cargo.toml`), `predicate::str::contains(...).not()` chaining, `assert_cmd::Command::cargo_bin`, `tempfile::TempDir` all verified against the lockfile. No hallucinated APIs in new test or source code.

### Layer 5 acceptance criteria → test trace

| AC (TODO.md lines 244-251) | Test(s) | Category |
|---|---|---|
| AC 1 — `--status open --priority high` shows only matching | `list_status_and_priority_filter_and_combination` | Cat B integration |
| AC 2 — `--status open --label bug` shows only matching | `list_status_and_label_filter_and_combination` | Cat B integration |
| AC 3 — `--priority high --label bug` shows only matching | `list_priority_and_label_filter_and_combination` | Cat B integration |
| AC 4 — `--status open --priority high --label bug` (three-filter AND) | `list_three_filter_and_combination` (also closes QE R11/R12 carried-forward Open Finding 5) | Cat B integration |
| AC 5 — 2/3 match but not 3rd → does NOT appear | `list_three_filter_and_combination` (three subcase `!contains` assertions) + unit `filter_and_logic_all_must_match` (three predicate-level subcases) | Cat B + Cat A unit |
| AC 6 — `--status done --priority low` no match → filter message | `list_compound_two_filter_no_match_shows_filter_message` | Cat B integration |
| AC 7 — `--status open --priority high --label nonexistent` no match → filter message | `list_compound_three_filter_no_match_shows_filter_message` | Cat B integration |
| AC 8 — `tracker list` (default, open issues exist) shows them, NOT filter message | `list_default_view_with_open_issues_does_not_show_filter_message` | Cat B integration |

**Predicate-level Cat A unit tests (5):** `filter_and_logic_all_present_returns_true` (true case), `filter_and_logic_all_must_match` (three 2/3-mismatch subcases — status-only-fails, priority-only-fails, label-only-fails), `filter_status_only_matches_any_priority_and_labels` (None=wildcard for priority + label), `filter_status_mismatch_rejects_regardless_of_optional_filters` (status is required), `filter_label_match_is_case_sensitive` (case-sensitivity preserved through the predicate boundary).

**Coverage verdict: 8/8 ACs traced to at least one passing automated test that fails if the AC is violated.**

### Mutation analysis — `issue_matches_filters`

The predicate body is:

```rust
issue.status == status
    && priority.is_none_or(|p| issue.priority == p)
    && label.is_none_or(|l| label_matches(&issue.labels, l))
```

For each unit test, one mutation killed and one not killed:

| Unit test | Mutation killed | Mutation NOT killed (gap or covered elsewhere) |
|---|---|---|
| `filter_and_logic_all_present_returns_true` | constant-`false` body | swap `==` → `!=` on status alone (would surface only via the all-must-match test); covered by `filter_and_logic_all_must_match` and `filter_status_mismatch_rejects_regardless_of_optional_filters` |
| `filter_and_logic_all_must_match` | drop any single conjunct (replace with `true`) — three subcases pin the three conjuncts independently | swap `&&` → `\|\|` between the first two AND'd terms when neither short-circuits to false on its own (e.g., status-mismatch + priority-mismatch with label-match); not exercised — see Open Finding 1 |
| `filter_status_only_matches_any_priority_and_labels` | swap `is_none_or` → `is_some_and` (None branch becomes false) | a mutation that always returns `true` when both optional filters are `None` regardless of status; covered by `filter_status_mismatch_rejects_regardless_of_optional_filters` (this test alone wouldn't catch it) |
| `filter_status_mismatch_rejects_regardless_of_optional_filters` | route status to `!=` | a mutation that ignores `issue.status` and uses `priority.unwrap_or("open") == status` instead — covered transitively by `filter_and_logic_all_must_match` priority subcase |
| `filter_label_match_is_case_sensitive` | inserting `.to_lowercase()` on either side of `label_matches`'s `==` | a mutation that swaps the compare to `labels.iter().all(\|l\| l == filter)` instead of `.any(...)` — predicate-level NOT caught here (test issue has only one label so `any` and `all` agree); covered at integration via Layer 4 `list_label_filter_shows_matching` "No-label item" subcase, which would surface a vacuous-`all`-true (an unlabeled issue would appear under `--label bug`) |

**Aggregate mutation score:** All five "drop-a-conjunct" mutations are killed. All three "single-clause inversion" mutations (status `==`→`!=`, priority `==`→`!=`, label predicate inversion) are killed. The case-sensitivity contract is killed by `filter_label_match_is_case_sensitive`. The optional-filter wildcard contract is killed by `filter_status_only_matches_any_priority_and_labels`. The required-status-filter contract is killed by `filter_status_mismatch_rejects_regardless_of_optional_filters`. **Predicate-level coverage is tight.**

### Open

**Finding 1 — `&&` → `||` mutation between status and priority conjuncts is not caught at the predicate level when only one disjunct fires (Dim 3 — Mutation resilience)**

The predicate is `issue.status == status && priority.is_none_or(...) && label.is_none_or(...)`. Consider the mutation `&&` → `||` between the **first two** conjuncts: `issue.status == status || priority.is_none_or(...) && label.is_none_or(...)`. Rust precedence binds `&&` tighter than `||`, so this parses as `status == status || (priority.is_none_or(...) && label.is_none_or(...))`.

- `filter_and_logic_all_must_match` status-mismatch subcase: `issue=("open","high",["bug"])`, call `(issue, "done", Some("high"), Some("bug"))`. Mutated predicate: `false || (true && true)` = **true**. Expected: false. **Caught.**
- `filter_and_logic_all_must_match` priority-mismatch subcase: call `(issue, "open", Some("low"), Some("bug"))`. Mutated predicate: `true || (false && true)` = **true**. Expected: false. The original predicate also returns false. **NOT caught — survives.**
- `filter_and_logic_all_must_match` label-mismatch subcase: `(issue, "open", Some("high"), Some("feature"))`. Mutated: `true || (true && false)` = **true**. Original: false. **NOT caught — survives.**

So the status-subcase catches the `&&`→`||` mutation between conjuncts 1 and 2, but only because it's the only subcase where the LHS of the `||` is false. A symmetric mutation `&&` → `||` between conjuncts 2 and 3 (i.e., `status == status && (priority.is_none_or(...) || label.is_none_or(...))`) is **not caught by any unit test**: the priority-mismatch subcase yields `true && (false || true)` = true (expected false); label-mismatch subcase yields `true && (true || false)` = true (expected false). Both survive. The status-mismatch subcase yields `false && (...)` = false (expected false) — also survives.

At integration level, `list_three_filter_and_combination` does cover this: each "wrong-X-only" subcase has a negative `!contains` assertion, and the implementation goes through the predicate, so the `&&`→`||` between conjuncts 2 and 3 mutation would surface "Wrong priority only" or "Wrong label only" in the output. Verified by mental execution: with the mutation, `issue.status == "open" && (priority.is_none_or(|p| p == "medium") || label.is_none_or(|l| label_matches(&["bug"], l)))` evaluated against the priority-mismatch issue ("Wrong priority only", priority=medium, labels=["bug"]) and filter `(open, high, bug)` becomes `true && (false || true)` = true. The issue would appear in output. The `!contains("Wrong priority only")` assertion fires. **Caught at integration.**

**Severity: Low.** The mutation is killed at the integration boundary, just not at the predicate-unit boundary. The unit tests are slightly thinner than they appear: their AC-pinning value is real (each conjunct's role is demonstrated), but the structural argument that "five focused unit tests fully cover the predicate" overstates the case — the integration test is doing real work that the unit tests do not.

**Evidence:** `src/lib.rs:425-434` (predicate); `src/lib.rs#tests:880-901` (`filter_and_logic_all_must_match`); `tests/layer5.rs:185-279` (`list_three_filter_and_combination`).

**Rationale:** Defense-in-depth at the predicate boundary is cheap. A single additional unit subcase — e.g., `filter_or_logic_between_optional_conjuncts_is_not_a_substitute` asserting `!issue_matches_filters(&issue_with("open","medium",["bug"]), "open", Some("high"), Some("feature"))` — would kill the `&&`→`||`-between-conjuncts-2-and-3 mutation at the unit level. It's slightly redundant with the integration test, but the cost is one assertion line.

**Classification: Open / Low severity / non-blocking.** This is a sharper-than-required mutation analysis; the integration test catches the mutation. Recommend addition as a defense-in-depth strengthening, not a merge-gate concern.

**Proposed action:** Add to `mod tests` in `src/lib.rs`:
```rust
#[test]
fn filter_and_logic_is_not_or_between_optional_conjuncts() {
    // Defense-in-depth against `&&` → `||` between the priority and label
    // conjuncts: an issue that mismatches BOTH optional filters (matching
    // status only) must still reject. Catches a mutation that the three
    // single-mismatch subcases of filter_and_logic_all_must_match do not.
    let issue = issue_with("open", "medium", &["bug"]);
    assert!(!issue_matches_filters(&issue, "open", Some("high"), Some("feature")));
}
```

---

### Dismissed

**Finding 2 — Cat B classification of the 7 integration tests is "honest sycophancy" — could the dev have made them Cat A by extracting the predicate before Layer 3/4? (Dim 2 — Red Gate compliance)**

Adversarial probe: the Red Gate commit message says "the AND-combination is an emergent property of cmd_list's chained retain() calls (Layer 3 added --priority retain, Layer 4 added --label retain), so the CLI behavior was implemented incrementally rather than as a single Layer 5 change." Could the dev have written these as Cat A by sequencing differently — e.g., introducing `issue_matches_filters` at Layer 3 with priority-only support, then extending in Layer 4, then asserting AND-combination as the Layer 5 Red Gate?

**Classification: Dismissed.** The actual decomposition followed the layer plan in `TODO.md` (line 270 explicitly enumerates `filter_and_logic_all_must_match` as the Layer 5 unit Red Gate, not the integration tests). The integration tests for AC 1-3 (two-filter AND combinations) genuinely could not be Cat A under the layer plan — once Layer 3 added `--priority` filtering and Layer 4 added `--label` filtering with chained `retain` calls, the two-filter AND was *already implemented*. The Cat B classification accurately describes regression coverage of pre-existing emergent behavior. The same disposition applied at Layer 3 (`create_without_priority_defaults_to_medium` was Cat B because Layer 1 already defaulted via `priority: "medium"`) and Layer 4 (two Cat B deviations on Layer 1 defaults). The pattern is honest, consistent, and correctly self-classified.

The only path to genuinely Cat A integration tests at Layer 5 would have been to *not* implement the per-filter `retain` calls at Layers 3 and 4, deferring all filtering until a single Layer 5 implementation. That contradicts the layer plan's per-flag layering. **No finding.**

---

**Finding 3 — `list_default_view_with_open_issues_does_not_show_filter_message` exit-status assertion absent (Dim 3 — Assertion strength)**

The test calls `.assert().success()` then extracts stdout/stderr separately for content checks. `success()` already asserts exit code 0; no separate `.code(0)` is needed. Other Layer 5 tests follow the same pattern and use `success()`.

**Classification: Dismissed.** `assert_cmd::Assert::success()` is documented to assert `code == 0` per its Rust docs; this is the canonical idiom in the rest of the suite (see Layer 1/2/3/4). Adding a redundant `.code(0)` would be cargo-cult. No finding.

---

**Finding 4 — AC 6 ("`--status done --priority low` no match") setup uses one open-high and one open-low issue; neither has `--status done`. Could a mutation still surface? (Dim 5 — Empty-state coverage)**

`list_compound_two_filter_no_match_shows_filter_message` creates two issues — `(open, high)` and `(open, low)` — then filters `--status done --priority low`. Neither matches both. The test asserts the filter message appears and `Nice work!` does not. The negative `Nice work!` assertion is the structural defense (per QE Review 9 Finding 1 / Review 11 Finding 3 pattern), and it is present.

But: is the setup adversarial enough? An issue that's `(done, low)` would match. A mutation that broke the filter would surface either issue. The test would detect both. **Dismissed — the setup is correct.** The negative `Nice work!` assertion specifically guards against the SO Review 11 hazard where adding a new filter and forgetting to extend `extra_filter_active` routes the empty-state path back to "Nice work!" — and `list_compound_two_filter_no_match_shows_filter_message`'s `effective_status == "done"` setup means `is_default_open_view` is false regardless of the disjunction — so this test is somewhat weaker than `list_compound_three_filter_no_match_shows_filter_message` for that specific hazard. The three-filter sibling closes the gap (its `effective_status == "open"` setup makes `is_default_open_view` route on the disjunction).

**Classification: Dismissed.** The two tests are complementary — between them, both branches of `is_default_open_view` are exercised in compound contexts. No new finding.

---

**Finding 5 — Cat B integration test assertions use substring `contains` rather than full-line matching for issue rendering (Dim 3)**

E.g., `list_three_filter_and_combination` asserts `out.contains("Triple match")` — a substring match. A mutation that printed only `Triple match` with no other column data would still pass this assertion.

**Classification: Dismissed.** Layer 1's `list_shows_header_and_issues` and Layer 3's `list_columns_use_exactly_two_space_separator` already pin the column-rendering contract. Layer 5's tests are correctly scoped to filter selection (which titles appear / don't appear), not column formatting. Conflating two contracts in a single test is the anti-pattern QE Review 11 Finding 6 explicitly dismissed at Layer 4. Same disposition here.

---

**Finding 6 — No Layer 5 test for `list --priority X --label Y` with `--status` defaulting to `"open"` AND only `done` issues exist (Dim 5 — Empty-state path interaction)**

Adversarial probe: when `--priority` and `--label` are both supplied but `--status` is not, `effective_status = "open"`, `extra_filter_active = true`, so `is_default_open_view = false`. If all stored issues are `done`, the result is empty and the filter-message branch fires. Is this covered?

`list_priority_and_label_filter_and_combination` does have an in-progress issue intended to confirm the implicit-status-default filter, BUT the test asserts a **non-empty** result ("Match all" appears), not an empty one. So the empty-result-with-implicit-default path is not directly asserted at Layer 5.

**Classification: Dismissed.** The path is exercised transitively by `list_compound_two_filter_no_match_shows_filter_message` (effective_status="done", priority="low") and `list_compound_three_filter_no_match_shows_filter_message` (effective_status="open" via `--status open`, priority="high", label="nonexistent"). The latter exercises the `extra_filter_active = true && effective_status == "open"` routing precisely. The implicit-default vs. explicit-`--status open` distinction is parser-side and identical past `parse_status` (both produce `effective_status == "open"`). Adding a third empty-state test for "implicit `--status open` + non-default filters + no match" would be tautological with the explicit form. No finding.

---

**Finding 7 — Manual testing checklist relies on developer attestation; no automated verification of the four-issue setup or two-filter / three-filter outputs (Dim 7)**

TODO.md lines 256-261 list 6 manual checks, all marked complete in commit `da0fd8d`. The integration tests in `tests/layer5.rs` cover all 6 automated equivalents (the four-issue setup matches `list_three_filter_and_combination`'s setup; the empty-state messages match the two compound-no-match tests; the default-view non-empty case matches the eighth test). The manual checklist is reproducible from TODO.md alone.

**Classification: Dismissed.** Each manual check has an automated counterpart with stronger assertions (negative `!contains` for non-matching issues, exact stderr literal for empty-state messages). The dev's claim of completion is reproducible by anyone running `cargo test --test layer5` against the same source. No finding.

---

### Hallucinated

**Finding 8 — `cmd_list` refactor changed evaluation order from three-pass `retain` to single-pass `retain`; could a mutation in the now-fused predicate body silently change short-circuit semantics? (Dim 6 — Regression coverage)**

Concern: previously each `retain` call walked the full vector once; the refactor walks once and short-circuits on the first false. A side-effecting mutation in any conjunct would behave differently across the two forms.

**Classification: Hallucinated.** No conjunct has side effects — `==` on `String`, `Option::is_none_or`, and `label_matches` (slice scan) are all pure. AND is commutative and associative for booleans; the ordering between the three `retain` passes (which always saw the full surviving set after each) and the single fused predicate (which short-circuits per-element) is observably identical. The refactor preserves behavior. No finding.

---

**Finding 9 — The unit test `filter_status_only_matches_any_priority_and_labels` asserts both `high_with_bug` and `low_no_labels` match — the second is redundant**

Concern: the test creates two issues but the predicate's behavior is element-by-element; one issue suffices to assert "None=wildcard."

**Classification: Hallucinated.** The two issues exercise distinct shapes (high+labelled vs. low+unlabelled) — both should pass when filters are None, and the test catches a subtle mutation where `is_none_or` is replaced with a non-empty-labels constraint (which would reject `low_no_labels` but accept `high_with_bug`). The two cases are intentionally complementary, not redundant. No finding.

---

**Finding 10 — `list_three_filter_and_combination` does not assert that issue ordering in the (single-result) output is correct**

Concern: the test asserts `Triple match` is present but doesn't verify it's at position 1 in the table.

**Classification: Hallucinated.** With one matching result, ordering is degenerate (only one issue can be in any position). Sort ordering is contract-pinned by Layer 3's `list_sorts_high_before_medium_before_low` and `list_within_tier_sorted_by_id_ascending`, which exercise a multi-issue result set. Layer 5's tests are correctly scoped to filter selection. No finding.

---

### Summary

**Test count:** 135/135 pass post-review (49 unit + 32 layer1 + 18 layer2 + 9 layer3 + 25 layer4 + 7 layer5). Pre-review: 135/135. Net delta: 0 inline source/test changes. One Open Low-severity strengthening proposed (Finding 1: `&&`→`||` between optional conjuncts at predicate level — caught at integration but not at unit; recommend a one-test addition).

**AC coverage:** 8/8 Layer 5 ACs traced to passing automated tests. The carried-forward Open Finding 5 from QE Reviews 11 and 12 (compound-filter test deferred to Layer 5) **closes** via `list_three_filter_and_combination` — the deferral was honored; the Layer 5 Red Gate enumerated the test before merge.

**Cat B disposition:** **Honest.** The 7 integration tests are correctly classified as Cat B Red Gate deviations: AND-combination was an emergent property of Layers 3-4's chained `retain()` calls. The 5 unit tests are correctly Cat A — they exercise `issue_matches_filters` directly and panicked against the `todo!()` stub at Phase 2a. Per-conjunct mutation pinning is tight at the predicate level for single-clause inversions and drop-a-conjunct mutations; the only gap is the cross-conjunct `&&`→`||` mutation at the predicate-unit boundary, which is caught at integration via `list_three_filter_and_combination`'s three negative-subcase assertions.

**Mutation resilience:** Strong at integration boundary; tight at predicate-unit boundary with one identified gap. The five unit tests collectively kill all single-conjunct-drop mutations, all single-clause-inversion mutations, all `is_none_or` ↔ `is_some_and` mutations, and all `to_lowercase` injections in the label compare. The cross-conjunct `&&`→`||` mutation between conjuncts 2-and-3 survives at the unit level but dies at the integration level via the AC 5 negative assertions.

**Top concern:** None merge-blocking. Finding 1 is a defense-in-depth strengthening at the unit level that the integration test already covers. Layer 5 ships with stronger test coverage than Layers 3 or 4 did at their respective merge gates: the Red Gate self-classification practice has matured (Cat A unit + Cat B integration + adversarial empty-state coverage with both `effective_status` branches exercised).

**Sycophancy check:** Did I pass any dimension because I couldn't think of a counterexample? Re-audited Dim 2 (Cat B) and Dim 3 (mutation resilience). On Cat B: I considered alternative decompositions and confirmed the actual one was forced by the layer plan, not a convenient story. On mutation resilience: I deliberately constructed `&&`→`||` mutations between specific conjunct pairs and traced each through every unit test rather than pattern-matching to "looks tight." Found one survival (Finding 1), filed it Low-severity, did not soften. On AC coverage: I checked each AC has a *failing*-on-violation test, not just a passing one — `list_default_view_with_open_issues_does_not_show_filter_message`'s `!stderr.contains("No issues match")` assertion specifically catches the inverse, so AC 8 is genuinely covered, not just nominally. **No softening detected.**

**Coordination:**
- **SE:** Open Finding 1 proposes adding one unit test (`filter_and_logic_is_not_or_between_optional_conjuncts`). QE owns `tests/**` and `src/lib.rs#tests` per CLOSURE-PROTOCOL.md; this can land in QE's same-round closure if the round produces a follow-up commit. If not, deferred to next layer.
- **SO / SA:** No spec or architecture concerns surfaced.
- **VDD-IAR Alignment:** Carried-forward Open Finding 5 (QE R11/R12) closes via Layer 5 Red Gate inclusion of `list_three_filter_and_combination`. Layer 5 merge gate is unblocked from the QE side.
- **PE:** Coverage tooling absence (Finding 8 from QE R10/R11) remains escalated; no new escalation.
- **No new domain coordination required.**

---

## Review 14 — 2026-05-07 00:41Z

**Round:** QE Review 14 (Round-2 closure for Layer 5)
**Scope:** Verify QE R13 F1 (defense-in-depth unit test for `&&`→`||` between optional conjuncts) is resolved by commit `7f9bae4`. Warm closure-verification.

### Round-1 finding closure

- **F1 (predicate-unit `&&`→`||` mutation between priority and label conjuncts survives all 5 Layer-5 unit tests):** **Resolved.** `src/lib.rs` `mod tests` adds `filter_and_logic_is_not_or_between_optional_conjuncts` — issue `(open, medium, [bug])` filtered with `priority=Some("high"), label=Some("feature")` mismatches both optionals at once, killing the inter-conjunct `||` mutation. The three single-mismatch subcases of `filter_and_logic_all_must_match` did not catch this mutation (each mismatched only one filter, so `||` would short-circuit-true on the matching conjunct). Test count: 136/136 (was 135 + 1 new defense unit test). Verified `cargo test --no-fail-fast --locked` green.

### Catalog of un-killed mutations remaining

After the new test, predicate-unit mutation analysis shows the AND-logic is mutation-tight at the unit level for: drop-a-conjunct, single-conjunct-flip (`==` → `!=`), `is_none_or` ↔ `is_some_and`, `to_lowercase` injection, single inter-conjunct `&&` → `||`. The only mutations that survive predicate-unit are CLI-wiring mutations (e.g., `cmd_list` passing the wrong filter into the wrong slot), which integration tests in `tests/layer5.rs` cover.

### Summary

1/1 Round-1 QE finding Resolved. 0 new findings this round. Layer 5 QE-domain is closed at MVR for the predicate-unit boundary.

**Coordination:** *(none — closure pass)*

---

## Review 15 — 2026-05-11 01:08Z

**Round:** QE Review 15 (Layer 6 — Description + Show + Delete)
**Scope:** Layer 6 lands in two commits — `4fb5e67` (Phase 2a Red Gate: 20 integration tests + 3 unit tests + `validate_description`/`format_show_block`/`cmd_show`/`cmd_delete` stubs) and `c91676a` (Phase 2b implementation). Files read in full: `iterative-adversarial-refinement/QUALITY-ENGINEER-REVIEW.md` Reviews 13–14, `DESIGN.md` (Features 1/4/5 + Edge Cases), `TODO.md` lines 279-345, `tests/layer6.rs` (all 20 integration tests), `src/lib.rs` Layer 6 surface (`validate_description` 335-340, `format_show_block` 350-387, `cmd_show` 398-409, `cmd_delete` 422-433, `cmd_create` 229-267 with new `description_raw` parameter, `next_id` 88-92), `src/lib.rs#tests` Layer 6 block lines 1069-1138 plus the load-time validator at `issue_field_validation_rejects_empty_description` line 758. Full-suite verification: `cargo test --no-fail-fast --locked` → **159/159 passed** (48 unit + 32 layer1 + 18 layer2 + 9 layer3 + 25 layer4 + 7 layer5 + **20 layer6**). Pre-review state: 159/159. Net delta from this review: 0 source/test changes (reviewer scope; findings filed for SE/SO/follow-up QE round).

**Session note:** Cold session, parallel batch. No prior participation in Layer 6 implementation, Red Gate authoring, or in-session reviews.

**Assumption surfacing:** `serde_json::Value` indexing semantics; `assert_cmd` `success()`/`failure()` and `predicate::eq` / `contains` chains; `TempDir` cwd-isolation via `mod common`. All consistent with the lockfile and prior layers.

### Layer 6 acceptance criteria → test trace

19 AC bullets per `TODO.md` lines 283-301.

| AC | Test(s) | Category |
|---|---|---|
| AC 1 — `--description "..."` stores verbatim | `create_with_description_stores_verbatim` (integration) | Cat A integration |
| AC 2 — `--description ""` exits 1 with literal error | `create_with_empty_description_exits_one` | Cat A integration |
| AC 3 — `--description "  "` exits 1 (whitespace-only) | `create_with_whitespace_description_exits_one` | Cat A integration |
| AC 4 — No flag → no `description` JSON key | `create_without_description_has_no_field_in_json` | Cat B integration (pre-existing serde behavior) |
| AC 5 — `show 1` displays all 8 fields | `show_displays_all_fields` (integration) + `show_label_column_right_padded_to_13` (unit, prefix shapes) | Cat A integration + Cat A unit |
| AC 6 — Label column right-padded to 13 chars | `show_label_column_right_padded_to_13` (unit) + `show_displays_none_for_no_labels` (integration: exact `Labels:      (none)`) + `show_displays_none_for_absent_description` (integration: exact `Description: (none)`) | Cat A unit + Cat A integration |
| AC 7 — Multi-line continuation indented 13 spaces | `show_multiline_description_indents_continuation` (integration) + `multiline_description_show_format` (unit) | Cat A integration + Cat A unit |
| AC 8 — Show untruncated title/labels | `show_does_not_truncate_title_or_labels` | Cat A integration |
| AC 9 — `show abc` exits 1 with parse error | `show_invalid_id_string_exits_one` | Cat A integration |
| AC 10 — `show 0` exits 1 | `show_zero_id_exits_one` | Cat A integration |
| AC 11 — `show 99` exits 1 not found | `show_not_found_exits_one` | Cat A integration |
| AC 12 — `delete 1` exits 0 + confirmation + removed | `delete_exits_zero_and_prints_confirmation` + `delete_removes_issue` | Cat A integration ×2 |
| AC 13 — After delete, `show <deleted>` not-found | `delete_then_show_returns_not_found` | Cat A integration |
| AC 14 — Deleted IDs never reused (max+1) | `delete_id_not_reused` (integration) + `max_id_plus_one_skips_deleted_ids` (unit) | Cat A integration + Cat B unit |
| AC 15 — `delete abc` exits 1 parse error | `delete_invalid_id_exits_one` | Cat A integration |
| AC 16 — `delete 99` exits 1 not found | `delete_not_found_exits_one` | Cat A integration |
| AC 17 — Other issues unchanged after delete | `delete_other_issues_unchanged` | Cat A integration |
| AC 18 — Description never in list output | `description_not_in_list_output` | Cat B integration (pre-existing `cmd_list` behavior) |
| AC 19 — `show 0` parse error (same as AC 10) | covered by AC 10 | — |

**Coverage verdict:** 19/19 AC bullets traced to a passing test that would fail if the AC were violated. (AC 19 is a duplicate of AC 10 in `TODO.md`; both shapes are pinned by `show_zero_id_exits_one`.)

### Red Gate compliance (`4fb5e67`)

Verified the commit-message classification:
- **3 unit tests** authored. 2 are Cat A (`multiline_description_show_format` + `show_label_column_right_padded_to_13` — both call `format_show_block` which is a `todo!()` stub at Red Gate, so they panic and Red). 1 is Cat B (`max_id_plus_one_skips_deleted_ids` — `next_id` was implemented in Layer 1 and already returns `max+1`; the unit test pins the contract for Layer 6's delete-id-never-reused invariant). Classification **honest**.
- **18 integration tests** classified Cat A: each exercises behavior new in Layer 6 — `cmd_show` / `cmd_delete` were stubbed with `todo!()` so any invocation panics with exit signal 101; `cmd_create`'s new `description_raw` parameter is intentionally discarded at Phase 2a so `create_with_description_stores_verbatim`, `create_with_empty_description_exits_one`, and `create_with_whitespace_description_exits_one` all see the wrong behavior. Mental run-through confirms each of the 18 fails Red.
- **2 integration tests** classified Cat B: `create_without_description_has_no_field_in_json` (serde `#[serde(skip_serializing_if = "Option::is_none")]` from Layer 1 already omits the key) and `description_not_in_list_output` (`cmd_list` from Layer 1 has never rendered description). Both are correctly self-classified — they cannot be Red against the stubs because the contract they pin pre-exists Layer 6. Same disposition pattern as Layer 4's two Cat B deviations (`create_without_labels_stores_empty_array`, `list_shows_none_for_no_labels`) and Layer 5's seven Cat B integration tests. **Honest classification.**

Red Gate verdict: **Compliant.** Commit ordering verified (`4fb5e67` precedes `c91676a`); 18 Cat A integration + 2 Cat A unit + 1 Cat B unit + 2 Cat B integration; the Phase-2a-only `#[allow(dead_code)]` on `validate_description` (called by `cmd_create` but discarded at Phase 2a — wait, re-read: it IS called at Phase 2a, see commit message — `cmd_create` discards the description, but `validate_description` was the stub that `cmd_create` does not yet call. Verified by the commit message: `validate_description (todo!())` + `cmd_create` `description_raw: Option<&str>` parameter `currently discarded so create still stores description: None`. So the Phase-2a wiring is: `validate_description` is unwired and `dead_code`-suppressed; `cmd_create` accepts the parameter and ignores it. At Phase 2b, the discard is replaced with `Some(validate_description(d)?)` per `src/lib.rs:237-240`, and the `#[allow(dead_code)]` is removed. Honest staging.

### Mutation analysis — `format_show_block`

Three mutations against `format_show_block` (`src/lib.rs:350-387`):

1. **Mutation A: change `"ID:          "` (10 spaces) → `"ID:           "` (11 spaces).** Test `show_label_column_right_padded_to_13` asserts `out.contains("ID:          ")`. Substring matching does NOT pin the trailing edge — `"ID:           "` (11 spaces) contains the 10-space prefix as a substring. **The mutation SURVIVES** the unit test. See Open Finding 1.
2. **Mutation B: drop the `\r\n` normalization (`let normalized = d.replace("\r\n", "\n");` → `let normalized = d.clone();`).** Currently no test feeds a CRLF description. `validate_description` (lines 335-340) only rejects empty-after-trim; it does NOT reject control characters (unlike `validate_title` and `parse_label`). Combined with the absence of a `description.chars().any(char::is_control)` check in `issue_fields_are_valid` (lines 132-135 only check `!d.trim().is_empty()`), a hand-edited `tracker.json` with `"description": "line1\r\nline2"` would load cleanly and reach `format_show_block`. The normalization line is currently UNTESTED — dropping it would change the rendered output for CRLF inputs, and no test fails. **The mutation SURVIVES.** See Open Finding 2 (Dim 5 cross-cut).
3. **Mutation C: change `\n             ` (newline + 13 spaces) → `\n            ` (12 spaces) for continuation indent.** Test `multiline_description_show_format` asserts `out.contains("\n             line2")` (13 spaces). The 12-space output contains `\n            line2` — substring match fails on the 13-space pattern (the 13th space is "l" in the mutated output). **The mutation IS killed.**
4. **Mutation D (bonus): change `(none)` for empty labels → `none`.** `show_displays_none_for_no_labels` asserts `Labels:      (none)`. Mutation **killed.**

**Mutation score for `format_show_block`:** 2 of 4 named mutations killed at the unit-test boundary (mutations C, D); 1 survives at unit but caught at integration *if* the test included a CRLF input (it does not); 1 survives both layers (mutation A — the unit prefix-shape test is too loose because of substring matching). Mutation A is the **top mutation gap**.

### Mutation analysis — `cmd_delete`

Per the prompt, two mutations against `cmd_delete` (`src/lib.rs:422-433`):

1. **Mutation E: `issues.remove(idx)` → no-op (`let _ = idx;`).** Killed by `delete_removes_issue` (post-delete JSON read asserts id=1 absent in the array). Killed by `delete_then_show_returns_not_found` (post-delete `show 1` returns `not found`). Killed by `delete_id_not_reused` (the new id assignment would be 3 only if the remove actually happened).
2. **Mutation F: swap order — call `save_issues` BEFORE `issues.remove(idx)`.** Killed by `delete_removes_issue` (saving the unchanged Vec writes id=1 back; post-delete JSON read sees id=1 still present).
3. **Mutation G: change the not-found error format from `"Issue #{} not found."` → `"Issue {} not found."` (no `#`).** Killed by `delete_not_found_exits_one` which asserts `contains("Error: Issue #99 not found.")` — exact literal pinning, missing `#` fails.
4. **Mutation H (bonus): change `println!("Deleted issue #{}.", id)` → `println!("Deleted #{}.", id)`.** Killed by `delete_exits_zero_and_prints_confirmation` which uses `predicate::eq("Deleted issue #1.\n")` — strict equality on stdout.

**Mutation score for `cmd_delete`:** All four named mutations killed. **Tight.**

### Open

#### Finding 1 — `show_label_column_right_padded_to_13` substring assertions do not pin the trailing edge of the padding (Dim 6 — Show rendering invariants; Dim 3 — Mutation resilience)

The unit test asserts each label prefix via `out.contains("ID:          ")` (3 chars + 10 spaces = 13). Substring matching does not detect a mutation that inserts ONE extra space (3 chars + 11 spaces = 14 char prefix in the rendered output). A 14-char prefix string still contains the 13-char substring. The same problem applies to `Title:       ` (6 + 7 spaces), `Status:      ` (7 + 6), `Priority:    ` (9 + 4), `Labels:      ` (7 + 6), `Created:     ` (8 + 5), `Updated:     ` (8 + 5).

The exceptions are `Description: ` (12 + 1 space) — where `multiline_description_show_format` asserts `out.contains("Description: line1")` which pins the immediately-following character as 'l', killing a "Description:  line1" (2 spaces) mutation; and the `Labels:      (none)` / `Description: (none)` integration tests where `(none)` immediately follows, pinning the trailing edge by adjacency.

So the unit test is loose for ID/Title/Status/Priority/Created/Updated (6 of 8 rows), tight for Labels and Description due to adjacent-value matching at the integration boundary.

The spec contract per DESIGN.md "Show output format" is **fixed-width 13 chars**. Any padding mutation that produces 12 or 14 would silently survive 6 of the 8 row-shape assertions in `show_label_column_right_padded_to_13`. The 12-char mutation (one space less) IS caught for those rows because the 13-char substring no longer matches a 12-char prefix (the asserted padding ends in a space, and the next character in the output would be a digit — the substring would still find a hit only if some other 13-space sequence happened, which is implausible). So the test catches *under-padding* mutations but NOT *over-padding* mutations.

**Severity: Medium.** The Dim 6 question in the prompt named this exact mutation: "Are tests strict enough that a mutation that changed `"ID:          "` to `"ID:           "` (one extra space) would fail?" — answer: **no, the unit test does not fail.** The integration test `show_displays_all_fields` also uses `contains` for each row label without exact-shape anchoring, so it shares the gap. Only `show_displays_none_for_absent_description` and `show_displays_none_for_no_labels` are tight, and only for the Description and Labels rows.

**Evidence:** `src/lib.rs:1102-1126` (`show_label_column_right_padded_to_13`); `tests/layer6.rs:124-138` (`show_displays_all_fields` row-label loop).

**Proposed action (for next QE round):** Add a unit subcase that asserts `out` contains the prefix as a regex / exact-match boundary, e.g., `out.lines().any(|l| l.starts_with("ID:          ") && l.chars().nth(13).map(|c| c != ' ').unwrap_or(false))` — or equivalently, assert the exact byte length of the first line, or use `predicate::str::is_match(r"^ID:\s{10}\d")` on the first line. The simplest defense: change the assertion to `out.contains("ID:          1")` for the issue with id=1 (forces the digit boundary). One line change per row. Cost: 6 strengthened assertions; benefit: kills the over-padding mutation at the unit boundary.

**Classification: Open / Medium severity / non-blocking for the merge but warrants a Round-2 QE strengthening.**

---

#### Finding 2 — `validate_description` does not reject control characters, and no test asserts it should (Dim 5 — Description as user input; Dim 4 — Error-path coverage; cross-cut with Security/RT)

**Symmetry break:** `validate_title` (line 685 has `validate_title("Fix\r\nbug").is_err()`) rejects control characters per Layer 1; `parse_label` rejects them per Layer 4 (per QE Review 11 Finding 4 → Review 12 closure). `validate_description` (lines 335-340) **only** checks `raw.trim().is_empty()`. A user can pass `tracker create "x" --description $'line1[31mFAKE'` (ESC + ANSI red sequence in description) and the binary accepts and stores it.

**Stored-data side:** `issue_fields_are_valid` (lines 125-139) checks the description with `is_none_or(|d| !d.trim().is_empty())` — no control-char check. A hand-edited `tracker.json` with `"description": "fake\n[31mERR[0m"` loads cleanly. When `tracker show <id>` renders this through `format_show_block`, the embedded ANSI sequence reaches stdout as raw bytes — terminal-escape injection on the show surface.

This is **the description-side equivalent** of the security finding QE Review 11 Finding 4 closed for labels (Security R7 Finding 1). The pattern is identical:
- Layer 1: title control-char rejection landed (QE R3-era).
- Layer 4: label control-char rejection landed (QE R11 → R12, closed under Security R7 atomic chain).
- Layer 6: description should land the same — Security/RT will surface it; QE Review 15 surfaces the **test-side gap**.

**Missing tests** (test side; SE owns the source extension; SO owns the DESIGN.md amendment):
- `description_with_newline_is_rejected` — `validate_description("a\nb").is_err()` (unit)
- `description_with_tab_is_rejected` — `validate_description("a\tb").is_err()` (unit)
- `description_with_escape_sequence_is_rejected` — `validate_description("\u{1B}[31mERR").is_err()` (unit)
- `description_with_nul_or_del_is_rejected` — `validate_description("a\u{0}b").is_err()` + `validate_description("a\u{7F}b").is_err()` (unit)
- `description_with_printable_unicode_is_accepted` — emoji / CJK passes (unit)
- `issue_field_validation_rejects_control_char_in_description` — load-time validator (unit)
- `create_with_control_char_description_exits_one` — `--description $'line1\nFAKE'` integration → exit 1, literal `Error: Description cannot contain control characters.`
- `corrupt_data_with_control_char_description_is_rejected` — hand-edited `tracker.json` with `\n` in description rejected at load (integration)

These are exactly parallel to the seven tests QE R12 added for labels. Pattern-named for direct copy-paste from `tests/layer4.rs::create_with_control_char_label_exits_one` etc.

**Severity: High.** Surface that emits user-supplied text to the terminal without escape sanitization. The threat model in DESIGN.md "Edge Cases / Labels" line 314 explicitly bounds the surface to "the user attacking themselves with hand-pasted clipboard content or a hand-edited `tracker.json`" — but the threat is precisely that surface here: a multi-line description with a hand-pasted ANSI sequence renders as terminal control on `tracker show`.

**Cross-coordination expected:** Security Review 9 will (or should) surface the source-side gap; SO Review 20 will need to amend DESIGN.md "Edge Cases / Description" to add `- Description containing a control character → error: Description cannot contain control characters.` (currently lines 339-345 do not enumerate this); SE Review 15 will need to extend `validate_description` and `issue_fields_are_valid`. QE owns the test additions in the same atomic round.

**Classification: Open / High severity / merge-blocking on the Layer 6 close gate per Security-finding policy** (security findings cannot be deferred per the IAR domain prompt; QE R11 → R12 set the precedent for labels).

**Evidence:** `src/lib.rs:335-340` (`validate_description`); `src/lib.rs:132-135` (`issue_fields_are_valid` description branch); `src/lib.rs:685` (Layer-1 precedent `validate_title("Fix\r\nbug").is_err()`); `tests/layer4.rs::create_with_control_char_label_exits_one` (Layer-4 pattern); DESIGN.md Edge Cases / Description lines 339-345 (currently silent on control chars).

**Side-effect on Mutation B above:** The `\r\n` normalization in `format_show_block` (line 365) is downstream defense-in-depth; once `validate_description` rejects `\r` as a control character, the normalization becomes unreachable for created issues. It remains reachable only for hand-edited `tracker.json` files that bypass `parse_label`-style validation — but `issue_fields_are_valid` would also reject control chars in description once extended, closing that surface. The normalization line becomes a code-comment justification rather than runtime path. Mutation B (Open Finding 1 above) is partially subsumed by the resolution path: with control chars rejected at load, the only CRLF source disappears.

---

#### Finding 3 — `create_with_description_stores_verbatim` does not pin the "not trimmed" half of the contract (Dim 1 — AC coverage; Dim 3 — Mutation resilience)

DESIGN.md Feature 1 postcondition: "description is stored as provided **(not trimmed)**; absent if --description is not provided." The current test uses the value `"Auth token expires too soon"` — no leading or trailing whitespace. A mutation in `validate_description` from `Ok(raw.to_string())` to `Ok(raw.trim().to_string())` would PASS the test (the test value has no surrounding whitespace; trimmed and untrimmed are identical strings). The "not trimmed" half of the postcondition is unpinned at the test surface.

**Severity: Medium.** A real falsifiability gap on a load-bearing postcondition that DESIGN.md explicitly distinguishes from the trim-on-store rules for title and labels. The asymmetric contract (description stores verbatim, title/labels trim) is precisely the kind of distinction that bit-rots if not tested.

**Proposed action:** Either (a) change the asserted value in `create_with_description_stores_verbatim` from `"Auth token expires too soon"` to `"  Auth token expires too soon  "` with `assert_eq!(v[0]["description"], "  Auth token expires too soon  ")`; or (b) add a sibling test `create_description_preserves_leading_trailing_whitespace` with the same shape. Cost: one assertion change OR one test addition (~10 lines).

**Classification: Open / Medium severity / non-blocking for merge but warrants Round-2 strengthening alongside Finding 2's test additions.**

**Evidence:** `tests/layer6.rs:24-41`; `src/lib.rs:335-340`; DESIGN.md Feature 1 postcondition.

---

### Dismissed

#### Finding 4 — No automated test for show non-mutation (Dim 7 — File integrity)

The TODO.md manual checklist line 309 says "Show is non-mutating: `tracker show 1` twice produces identical output; `tracker.json` unchanged." No automated test asserts that `tracker.json` is byte-identical before and after a `show` call.

**Classification: Dismissed.** `cmd_show` (lines 398-409) calls only `load_issues` (read) and `format_show_block` (pure) followed by `print!`; no path to `save_issues`. The non-mutating property is a structural consequence of the code, not a behavior subject to mutation-style regression. A test that reads `tracker.json` before, calls `show`, reads after, and asserts byte-equality would catch only a future implementation regression where someone wires `save_issues` into the show path — which is implausible. The Layer 2's `status_change_leaves_other_fields_unchanged` precedent applies to mutations; for a read-only command, the spec is satisfied by the call-graph. **No finding.** (If a paranoid round wanted defense-in-depth, a one-line `assert_eq!(pre, post)` test would suffice; it's a polish item, not a quality gap.)

---

#### Finding 5 — `delete_id_not_reused` only exercises a 2-issue-then-delete-then-create scenario, not delete-the-max-id case (Dim 3 — Mutation resilience)

Adversarial probe: the test creates 1, creates 2, deletes 1, creates → new id should be 3. What about: create 1, create 2, **delete 2**, create → should the new id be 3 (because `next_id` is `max+1` over the remaining `[1]` → 2? wait: max over `[1]` is 1, plus 1 is 2 — same as the deleted id!). Re-read DESIGN.md Feature 5 invariant: "the deleted ID is never reused; the next created issue receives `max(remaining_ids) + 1`, **which will always be greater than the deleted ID**."

If I delete the max id, `max(remaining) + 1` would re-create the just-deleted id, violating "greater than the deleted ID."

**Classification: Dismissed at QE.** Re-reading the invariant: "max(remaining_ids) + 1, which will always be greater than the deleted ID." This is the **spec invariant**, but the actual `next_id` implementation (`src/lib.rs:88-92`) computes max(remaining)+1, which after deleting the max id would re-use it. This is a **spec-vs-implementation gap**, not a test gap — and it's an SO/SA concern, not a QE one. Actually, looking at the DESIGN.md text more carefully: the invariant uses "always be greater than the deleted ID" as the rationale, but `max(remaining)+1` does NOT always satisfy this. If issues are `[1, 2, 3]` and 3 is deleted, `next_id` returns `max([1,2])+1 = 3`, re-using the deleted id.

This is a **real spec/implementation concern but it belongs to SA/SO/SE**, not QE-test-coverage. From a QE perspective: `delete_id_not_reused` correctly tests one path (delete non-max, then create — id=3 ≠ deleted id=1). It does NOT test the delete-the-max-id path. A test covering that case would expose the spec/implementation mismatch.

**Re-classification: Open / Low severity / cross-cut to SO and SA.** QE could add `delete_max_id_then_create_does_not_reuse_id` and let it fail (or pass with `id=3` against a `delete-then-create` scenario where the create yields id=3, which is the deleted id — failing the spec literal "greater than the deleted id"). Filing as a quiet observation; the primary owner is SO/SA. **For now: hold.** Will surface again in next round if SO/SA do not flag it.

---

#### Finding 6 — `description_not_in_list_output` uses only one issue, not the multi-issue case (Dim 5 — Empty-state coverage)

`description_not_in_list_output` creates a single issue with a description and asserts the description text is absent from `list` stdout. Would a multi-issue test catch additional mutations?

**Classification: Dismissed.** `cmd_list` renders rows uniformly; per-row rendering is contract-pinned by Layer 1's `list_shows_header_and_issues` and Layer 3's column-spacing test. A multi-issue variant would not exercise a different code path. The unique-marker pattern (`"DESCRIPTION_SHOULD_NOT_LEAK_INTO_LIST"`) is the correct falsifiability anchor — if any rendering path leaked the description, the marker substring would appear. **No finding.**

---

#### Finding 7 — `show_displays_all_fields` uses `contains` for each label, not anchored line-match (Dim 3 — Assertion strength)

The test asserts `out.contains("ID:")`, `out.contains("Title:")`, etc. A mutation that printed all labels on a single line without values would still satisfy each `contains` assertion (though `Fix auth`, `high`, `bug` etc. would also be required to appear, somewhat constraining the form).

**Classification: Dismissed.** Layer 6's primary surface for label-shape pinning is `show_label_column_right_padded_to_13` (unit) and the integration tests with adjacent values (`show_displays_none_for_no_labels` and `show_displays_none_for_absent_description`). The all-fields-present test is correctly scoped to "every field appears at least once"; the shape pinning is delegated to other tests. Conflating two contracts in a single test is the anti-pattern QE R11 F6 / QE R13 F5 explicitly dismissed for the same reason. **No finding** (but see Finding 1 for the strengthening that addresses the residual over-padding gap).

---

### Hallucinated

#### Finding 8 — `delete_id_not_reused` could fail in a stale-cache scenario where `tracker create "Third"` reads pre-delete state

Concern: the test runs four sequential subprocess invocations. Subprocess #4 (the post-delete create) might see a stale file if the OS write-cache hadn't flushed by subprocess #3 (the delete).

**Classification: Hallucinated.** `save_issues` calls `fs::write` which closes the file handle before returning; each subprocess exits before the next begins; POSIX fs semantics on Darwin and Linux guarantee that a file closed in process A is visible to a `read` in process B started after A exits. No stale-cache vector. Test is reliable. **No finding.**

---

#### Finding 9 — `format_show_block` returns a String but `cmd_show` uses `print!` not `println!` — could miss a trailing newline

Concern: `cmd_show` calls `print!("{}", format_show_block(issue))` and the `format!` block ends with `Updated:     {}\n` — there IS a trailing newline. But what if a mutation removed it? Would tests catch?

**Classification: Hallucinated.** Mutation killing is the test's job, but the spec doesn't require any specific trailing whitespace beyond the eight-row block. None of the tests assert "exactly one trailing newline and no extra" because that's not in DESIGN.md. The current code is correct; a mutation that doubled the newline (`println!` instead of `print!`) would not be caught, but it's also not a spec violation. **No finding.**

---

#### Finding 10 — `cmd_delete`'s `idx` from `position(...)` could differ from index-by-id if duplicate ids existed

Concern: `position` returns the first index, but if two issues had id=1 (duplicate), `remove(idx)` removes only the first.

**Classification: Hallucinated.** `issues_collection_invariants_hold` (lines 173-176) rejects duplicate ids at load. The load path is the only entry to the in-memory `Vec<Issue>`. Duplicates cannot exist past `load_issues`. `position` and `find` are correct against the deduplicated invariant. **No finding.**

---

### Process observation (not a defect)

**Dim 8 — Manual testing checklist.** The 13 items at `TODO.md:303-316` are all unchecked. This is a state observation: Layer 6 implementation has landed (`c91676a`); the manual checklist closure is the developer's next step toward the Layer 6 close gate, parallel to (or following) the IAR review batch. Layers 1-5 closed the equivalent checklists before merge per the established pattern (Layer 5 commit `da0fd8d`). This is not a quality-system defect, but the Layer 6 merge gate must verify the checklist closure as a precondition. **Process flag, not a finding.**

---

### Summary

**AC coverage:** 19/19 Layer 6 ACs traced to passing automated tests. No undocumented tests, no documented tests missing.

**Top mutation gap:** Finding 1 — the unit test `show_label_column_right_padded_to_13` uses substring `contains` assertions on the 13-char label prefixes (`"ID:          "`, `"Title:       "`, etc.) and does not pin the trailing edge of the padding. A mutation that adds ONE extra space to the padding for 6 of 8 rows (ID, Title, Status, Priority, Created, Updated) silently survives BOTH the unit test AND the integration test `show_displays_all_fields`. The Dim 6 question in the prompt asked this exact question; answer is "no, the test does not catch one-extra-space over-padding." The fix is one assertion strengthening per row (e.g., assert `out.contains("ID:          1")` instead of `out.contains("ID:          ")`).

**Top concern:** Finding 2 — `validate_description` does not reject control characters. Title and labels both have control-char rejection landed in prior layers (Layer 1, Layer 4). Description does NOT, and this is asymmetric on a surface that emits user text to the terminal via `cmd_show`. The same risk pattern that landed for labels in QE R11→R12 (Security R7 chain) applies here. **High severity, merge-blocking for Layer 6 close gate per security-finding policy.** Cross-cuts to Security Review 9, SO Review 20, SE Review 15 — expected to surface there; QE Review 15 surfaces the test-side gap (8 missing tests pattern-named for direct copy from Layer 4's label-control-char suite).

**Red Gate compliance verdict for Layer 6:** **Compliant.** Commit ordering verified; classifications honest (2 Cat A unit + 1 Cat B unit; 18 Cat A integration + 2 Cat B integration); the Phase-2a stub staging (`todo!()` panics + parameter-discard in `cmd_create`) is faithful to the Red-then-Green discipline. No test-implementation co-authorship coupling detected.

**Mutation resilience:** **Mixed.** Tight for `cmd_delete` (4/4 named mutations killed). Loose for `format_show_block` (over-padding mutation survives; CRLF-normalization line is currently untested and currently reachable due to Finding 2). Good for the description-stored-verbatim contract on the trim half but loose on the no-trim half (Finding 3).

**Sycophancy check:** Did I pass any dimension because I couldn't think of a counterexample? Re-audited Dim 6 (show invariants — found Finding 1's over-padding gap by deliberately enumerating substring-vs-anchored-match), Dim 5 (description as user input — found Finding 2 by cross-referencing the Layer-1/4 hardening pattern), and Dim 1 (AC coverage — found Finding 3's not-trimmed-half gap by re-reading the postcondition wording rather than the test's assertion). Did not soften: Finding 2 is filed High severity / merge-blocking per security policy, not deferred. Finding 1 is filed Medium severity even though all 159 tests are green and the developer would prefer no findings on a recent layer. **No softening detected.**

**Files modified:** None (QE Review 15 is read-only; per CLOSURE-PROTOCOL.md, QE owns `tests/**` and `src/lib.rs#tests`, but this review is the cold-batch surfacing pass — the test additions land in the Round-2 closure pass after SE/SO/Security have applied source-side and spec-side fixes).

**Coordination:**
- **SE Review 15:** Finding 2 requires `validate_description` extension to reject control characters (mirror `validate_title` line 685 pattern) AND `issue_fields_are_valid` extension to reject control chars in description at load (mirror `label_is_valid` line 145-147 pattern). Once applied, QE Round-2 adds the 8 enumerated tests.
- **SO Review 20:** Finding 2 requires DESIGN.md amendment to "Edge Cases / Description" (lines 339-345) to add the control-char rejection rule, and to "Edge Cases / Storage" (line 333) to add description-control-char to the invalid-domain-values list.
- **Security Review 9:** Finding 2 is in your domain (terminal-escape injection on the `show` surface). Expected to surface independently; QE provides the test-side complement.
- **SA Review 13:** Possible cross-cut on Dismissed Finding 5 (`max(remaining)+1` vs. "always greater than deleted id" wording in DESIGN.md Feature 5 invariant) — left for SA's spec-compliance read. Not raising as a QE finding.
- **VDD-IAR Alignment Review 15:** Layer 6 merge gate must verify Finding 2 is closed (security finding cannot be deferred). Findings 1 and 3 may be merged with the explicit Round-2 strengthening commitment.
- **PE Review 10:** Coverage tooling absence remains escalated; no new escalation.

---

## Review 16 — 2026-05-11 02:00Z

**Round:** QE Review 16 (Round-2 closure for Layer 6)
**Scope:** Verify the three QE R15 findings + cross-domain description-Cc-defense cluster are resolved by commit `9b775f0`. Warm closure-verification.

### Round-1 finding closures

- **F2 (description Cc defense, High / merge-blocking):** **Resolved by commit `9b775f0`.** `validate_description` rejects `is_control()` except `\n`; `description_is_valid` enforces the same at load time via `issue_fields_are_valid`. Tests added:
  - Integration (tests/layer6.rs): `create_with_control_char_description_exits_one` (ESC), `create_with_carriage_return_description_exits_one`, `create_with_crlf_description_exits_one`, `create_with_tab_description_exits_one`, `create_with_del_description_exits_one`, `create_with_osc8_hyperlink_description_exits_one`, `create_with_newline_description_is_accepted`, `corrupt_data_with_control_char_description_is_rejected`, `corrupt_data_with_carriage_return_description_is_rejected`, `load_accepts_description_with_newline`.
  - Unit (src/lib.rs#tests): `description_empty_after_trim_is_rejected`, `description_with_control_char_other_than_newline_is_rejected`, `description_with_newline_only_is_accepted`, `description_stored_verbatim_not_trimmed`, `description_with_printable_unicode_is_accepted`, `issue_field_validation_rejects_control_char_in_description`, `issue_field_validation_rejects_carriage_return_in_description`, `issue_field_validation_accepts_newline_in_description`, `issue_field_validation_accepts_no_description`.
  - (Note: NUL byte cannot be tested via subprocess argv per OS constraint; covered by the unit test in-process.)
- **F1 (over-padding mutation in show output):** **Resolved by commit `9b775f0`.** New test `show_renders_exact_full_block_for_single_line_issue` uses full-line `assert_eq!` on all 8 rendered rows; an over-padding mutation (e.g., `"ID:          "` → `"ID:           "`) now fails. Plus a `lines.len() == 8` assertion that catches any extra-line-emit mutation.
- **F3 (verbatim-storage half of description postcondition untested):** **Resolved by commit `9b775f0`.** `create_preserves_description_verbatim_with_surrounding_whitespace` (integration) + `description_stored_verbatim_not_trimmed` (unit) both kill the `Ok(raw.trim().to_string())` mutation in `validate_description`.

### New findings

*(none this round.)*

### Test suite delta

- Pre-R2: 159/159 pass (48 unit + 32 layer1 + 18 layer2 + 9 layer3 + 25 layer4 + 7 layer5 + 20 layer6).
- Post-R2: **180/180 pass** (57 unit + 32 + 18 + 9 + 25 + 7 + 32 layer6). Delta: +9 unit + +12 integration.

### Summary

3/3 Round-1 QE findings Resolved. The cross-domain description-Cc-defense cluster is closed at the test boundary with 21 new tests (12 integration + 9 unit). Layer 6 QE-domain is at MVR for the predicate + render boundaries.

**Coordination:** *(none — closure pass)*

---

## Review 17 — 2026-05-11 22:30Z

**Round:** QE Review 17 (Round 1 — Layer 7 cold-batch surfacing pass)
**Scope:** Layer 7 polish (branch `issue-tracker-cli-polish`, commits `7b461aa` Red Gate + `a2b8062` impl + `603c689` manual closure). Whole-app regression baseline. Read-only surfacing — Round-2 closure pass owns test additions per CLOSURE-PROTOCOL.md §1 (QE owns `tests/**`).

### Regression check (whole-suite)

- `cargo test --no-fail-fast --locked` — **195/195 pass** (62 unit + 32 layer1 + 18 layer2 + 9 layer3 + 25 layer4 + 7 layer5 + 33 layer6 + 9 layer7). Re-run locally this session, confirmed.
- No prior-layer test removed or weakened in the Layer 7 commits; the `format_show_block(false)` call-site updates to the two pre-existing unit tests preserve uncolored-layout pinning and do not loosen any assertion. No regression detected at the test boundary.

### AC coverage map for Layer 7

DESIGN.md / TODO.md Layer 7 ACs (L353-366) mapped to tests:

| AC | Test | Category |
|---|---|---|
| AC 1 — `tracker --help` exits 0, describes all subcommands | `help_flag_binary_exits_zero` (asserts `Usage: tracker` + each of `create`/`list`/`status`/`show`/`delete`) | Cat B (clap default) |
| AC 2 — `tracker create --help` exits 0, describes `--description`/`--priority`/`--label` with valid values | `help_flag_create_exits_zero` (incl. `"low, medium, high"`) | Cat B (clap default) |
| AC 3 — `tracker list --help` exits 0, describes filters with valid values | `help_flag_list_exits_zero` (incl. both enumerations) | Cat B (clap default) |
| AC 4 — `tracker status --help` exits 0, describes positional args + valid statuses | `help_flag_status_exits_zero` | Cat B (clap default) |
| AC 5 — `tracker show --help` exits 0, describes `<ID>` | `help_flag_show_exits_zero` | Cat B (clap default) |
| AC 6 — `tracker delete --help` exits 0, describes `<ID>` | `help_flag_delete_exits_zero` | Cat B (clap default) |
| AC 7 — TTY: `high` red/bold, `medium` yellow, `low` default | **Manual checklist only** (TODO.md L370) | No automated coverage |
| AC 8 — TTY: `in-progress` cyan, `done` green, `open` default | **Manual checklist only** (TODO.md L370/L372) | No automated coverage |
| AC 9 — Piped: no ANSI escape codes | `list_piped_has_no_ansi_codes` + `show_piped_has_no_ansi_codes` (stdout-only) | Cat A regression guard for Phase 2b |
| AC 10 — Color is value-only, not row/header | **Manual checklist only** (TODO.md L370) | No automated coverage; relies on `wrap_color` contract |
| AC 11 — Color appears in both `list` and `show` (TTY) | **Manual checklist only** (TODO.md L370/L372) | No automated coverage |
| AC 12 — All error messages begin with `Error:` | Inherited from Layer 1-6 (`main.rs` `try_parse` transform) | Cat B (pre-existing) |
| AC 13 — Unknown subcommand exits 1 with usage error on stderr | `unknown_subcommand_exits_one` | Cat A |

**Coverage verdict:** 13/13 ACs traced to a passing test or named manual-only item. 4 of 13 ACs (the TTY-positive ACs 7, 8, 10, 11) have **zero automated coverage** — the entire color-rendering surface is gated only by the manual checklist. See Open Finding 1.

### Red Gate compliance — Phase 2a (`7b461aa`) — process deviation

The Layer 7 Red Gate commit message and `tests/layer7.rs` top-comment (L1-25) explicitly disclose that **all 9 tests pass against pre-implementation code**. The framing is: clap defaults and the Layer 1 `try_parse` transform already satisfy the help/error contract; the two piped tests exercise the non-TTY branch (which is the only branch reachable from a subprocess test) and pass trivially against pre-color code, becoming regression guards in Phase 2b.

The framing is **honest about the deviation** (the comment is verbatim accurate against `git show 7b461aa`), but the deviation itself is a real process gap, not a virtue. Three sub-points:

1. **Tests that *could* have been written failing-first but were not.** `unknown_subcommand_exits_one` asserts exit code **1**. Pre-Layer-1, clap's default for an unrecognized subcommand is exit **2**; the `try_parse` transform in `src/main.rs:74-78` is the code that converts to exit 1. That transform predates Layer 7 — but if the test were authored against a hypothetical "naive" stub (raw `Cli::parse()` with no transform), it would Red on exit code. The fact that it passes today reflects that the contract was already pinned by Layer 1's behavior, not that the test is meaningless — it is correctly a regression guard. **Honest framing.**
2. **Tests that *could not* have been Red.** The 6 help-flag tests are Cat B: clap's `--help` plumbing is the entire implementation. No "stub that exits 1" stage of Layer 7 was ever planned. The comment acknowledges this.
3. **The deviation that is real.** The piped-no-ANSI tests would have been Red against a *strictly correct* Phase 2a stub: one that emitted ANSI unconditionally (the naive implementation the spec warns against). The Red Gate discipline says "write the test, watch it fail, then implement." Here the test was written and watched to *pass* — because no stub existed. The test serves its intended purpose (regression guard against the naive Phase 2b implementation), but the **Red-then-Green discipline was not exercised on the color contract**. There was no point at which a developer could have shipped color-always and discovered the test broken.

**Verdict:** Red Gate compliance is **deviation-disclosed, not enforced.** The disclosure makes the deviation auditable; that is the floor, not the ceiling. A genuinely Red-first approach to the color contract would have introduced a `force_color()` test seam (env var or constructor parameter) so that TTY-positive rendering became automatable — see Finding 1.

### Findings

#### Open

##### Finding 1 — TTY-positive color rendering has zero automated coverage; a `force_color` test seam would have made it automatable (Dim 1, Dim 2, Dim 4, Dim 13)

ACs 7, 8, 10, 11 — all four TTY-positive ACs — are gated by manual checklist only (TODO.md L370-373). The implementation chose `std::io::stdout().is_terminal()` as the sole TTY signal. There is no override seam — no env-var check (e.g., `FORCE_COLOR`, `CLICOLOR_FORCE`, `NO_COLOR` honored bidirectionally), no constructor parameter on `cmd_list` / `cmd_show` injecting `use_color`. As a consequence:

- A mutation that flips `priority_ansi("high", true)` from `Some("\x1b[1;31m")` to `Some("\x1b[31m")` (drop the bold) survives every automated test.
- A mutation that swaps the cyan and green sequences in `status_ansi` survives every automated test.
- A mutation that colors the **label column** instead of the value (violating AC 10's "value only, not entire row or header") survives every automated test, because the only color-related automated tests assert the **absence** of `\x1b[` from piped output.
- A mutation that colors the **header row** of `cmd_list` (violating AC 10) survives every automated test for the same reason.

The piped-no-ANSI tests pin the *suppression* contract; they cannot pin the *application* contract because `assert_cmd::Command` connects stdout to a pipe by construction. The four TTY-positive ACs depend on the human-eyeball manual checklist to catch any of the above mutations.

A `force_color` seam (env var honored at TTY-detection time, like `cargo` itself does with `CARGO_TERM_COLOR=always`) would make TTY-positive rendering automatable: a test sets `CLICOLOR_FORCE=1`, invokes `tracker list`, and asserts `predicate::str::contains("\x1b[1;31mhigh\x1b[0m")` on stdout. The TODO.md L389 comment ("TTY-detection cannot be automated in subprocess tests") is true given the current implementation but false in principle — the seam is the standard idiom for exactly this case.

**Severity: Medium-High.** Quality-system gap: the four TTY-positive ACs are CI-invisible. A future refactor that breaks color application is detected only when a human runs through the manual checklist — which is exactly the kind of "manual gate that a passing CI run could miss" the QE Standard Dim 13 names as a finding.

**Evidence:** `tests/layer7.rs:1-25` (top-comment disclosure); `src/lib.rs:591` (`cmd_show` TTY detection); `src/lib.rs:835` (`cmd_list` TTY detection); TODO.md L370-373, L389 (manual-only carve-out); DESIGN.md L239-250 (color contract).

**Proposed action (Round 2):** Add a `force_color` env-var seam in the TTY-detection paths (e.g., `is_terminal() || env::var("TRACKER_FORCE_COLOR").is_ok()`). Add integration tests setting the env var that assert the exact ANSI sequences appear on stdout for each priority and status value and **do not** appear in the header row or label column. Cost: ~2 lines in lib.rs, ~6 integration tests. Cross-cuts SE (env-var consumption) and UX (env-var name follows community convention).

**Classification:** Open / Medium-High severity / non-blocking for Layer 7 merge (manual checklist closure satisfies the floor) but a real quality-system gap that should not survive a second cold-batch round.

---

##### Finding 2 — No unit tests for `priority_ansi` / `status_ansi` / `wrap_color` / `pad_after_color`; the exact-sequence contract for color helpers is unpinned (Dim 2 Mutation testing, Dim 13 Coverage)

Four new pure functions were added to `src/lib.rs` in commit `a2b8062`: `priority_ansi` (L51-60), `status_ansi` (L65-74), `wrap_color` (L79-84), and `pad_after_color` (L91-97). The `src/lib.rs#tests` module gained zero tests for any of them. They are private (no `pub`), so the QE supplement's "100% public API coverage" floor does not literally apply, but they are the implementation of a load-bearing spec contract (DESIGN.md L239-250 color table) and they are pure — they are exactly the functions that unit tests are designed for.

Named mutations that survive every test in the current 195-suite:

1. `priority_ansi("high", true)` returns `Some("\x1b[1;31m")` → mutation to `Some("\x1b[31m")` (drop the `1;` bold marker). Spec says "Red / bold." Survives.
2. `priority_ansi("medium", true)` returns `Some("\x1b[33m")` → mutation to `Some("\x1b[32m")` (yellow → green). Survives.
3. `status_ansi("in-progress", true)` returns `Some("\x1b[36m")` → mutation to `Some("\x1b[35m")` (cyan → magenta). Survives.
4. `status_ansi("done", true)` returns `Some("\x1b[32m")` → mutation to `Some("\x1b[31m")` (green → red). Survives.
5. `priority_ansi("low", true)` returns `None` → mutation to `Some("\x1b[31m")` (color the default-color value). Survives.
6. `status_ansi("open", true)` returns `None` → mutation to `Some("\x1b[33m")` (color the default-color value). Survives.
7. `wrap_color(value, Some(prefix))` returns `format!("{}{}{}", prefix, value, ANSI_RESET)` → mutation that drops the reset (`format!("{}{}", prefix, value)`). Survives — and is a real bug shape (leaked color onto subsequent rows in `cmd_list`).
8. `pad_after_color` `if visible_chars >= total_width` → `>` (drop equality) mutation. For a value whose visible width exactly equals the column width (e.g., `in-progress` at 11 chars vs. column width 11), the mutated branch falls through to `" ".repeat(total_width - visible_chars)` = `" ".repeat(0)` — still correct in this case, so the mutation is functionally equivalent on the current values. Hallucinated for this exact code, but flagged because the boundary is fragile against a future column-width change.
9. `pad_after_color` `total_width - visible_chars` → `total_width.saturating_sub(visible_chars + 1)` (off-by-one). Survives at the unit level entirely.

Mutations 1-7 are real coverage gaps. They are killable by ~8 unit tests (one per mutation class) of about 3 lines each.

**Severity: Medium-High.** Pure-function mutation gaps on a load-bearing spec contract. The QE Standard Dim 2 mutation-testing prompt names exactly this case ("a mutation that no test catches is a coverage gap regardless of reported coverage percentage").

**Evidence:** `src/lib.rs:51-97` (the four helpers); the `src/lib.rs#tests` module (L869-1506) has no tests referencing any of the four function names.

**Proposed action (Round 2):** Add unit tests pinning each helper's exact return for each input. Pattern:

```rust
#[test]
fn priority_ansi_high_returns_bold_red() {
    assert_eq!(priority_ansi("high", true), Some("\x1b[1;31m"));
}
```

…one per (value, color) pair, plus tests for `use_color=false` returning `None` for every value, plus `wrap_color` round-trip with reset, plus `pad_after_color` boundary cases (visible < total, visible == total, visible > total). ~10-12 unit tests, ~40 LOC.

**Classification:** Open / Medium-High severity / non-blocking for Layer 7 merge but the most-mutable surface in the layer; pairs with Finding 1 for the Round-2 closure pass.

---

##### Finding 3 — `unknown_subcommand_exits_one` does not assert stdout is empty; `help_flag_*_exits_zero` tests do not assert stderr is clean (Dim 3 — selector / assertion strength; cross-cut to UX stdout/stderr discipline)

DESIGN.md "stdout contract" / "stderr contract" (L220-222) is explicit: error messages go to stderr, data to stdout. AC 13 routes the unknown-subcommand usage error to stderr. The Layer 7 test asserts `.stderr(contains("Error:"))` + `.stderr(contains("unrecognized subcommand"))` + `.stderr(contains("frobnicate"))` — but does **not** assert `.stdout(predicate::str::is_empty())`. A mutation that prints the usage error to **both** streams (a common refactor regression) would pass.

Symmetrically, the six `help_flag_*_exits_zero` tests assert content on stdout (clap routes `--help` to stdout per CLI convention) but do not assert stderr is clean. A mutation that leaked the help text to stderr — or that emitted a stray informational message to stderr alongside the help text — would survive every test. This matters for the `--help | head -1` pipeline shape: a non-empty stderr surfaces in the user's terminal even when stdout is captured.

**Severity: Medium.** The stdout/stderr separation is a DESIGN.md contract (L220-222) and a CLI supplement Dim ("stdout / stderr / exit code assertions" — supplements/cli.md L26-27 specifically: "A test that only checks stdout will miss error-path regressions"). The current Layer 7 tests assert *positively* on one stream per test but never *negatively* on the other.

**Evidence:** `tests/layer7.rs:36-107` (help flag tests, no stderr negative assertion); `tests/layer7.rs:111-122` (`unknown_subcommand_exits_one`, no stdout negative assertion); DESIGN.md L220-222 (stdout/stderr contract); supplements/cli.md L26-27.

**Proposed action (Round 2):** Add `.stdout(predicate::str::is_empty())` to `unknown_subcommand_exits_one`. Add `.stderr(predicate::str::is_empty())` to each `help_flag_*_exits_zero` test (clap routes `--help` to stdout; stderr should be empty for a successful help invocation). Cost: 7 added lines.

**Classification:** Open / Medium severity.

---

##### Finding 4 — `list_piped_has_no_ansi_codes` and `show_piped_has_no_ansi_codes` check only stdout; ANSI on stderr is not validated (Dim 3, Dim 5 — empty-state messages route to stderr per DESIGN.md)

Empty-state messages (`No open issues. Nice work!` and `No issues match the given filters.`) route to **stderr** per DESIGN.md L222. If a future implementation extended color application to those informational messages (e.g., a green "Nice work!" celebration coloring), it would emit ANSI to stderr. The piped-stdout tests do not catch this. Neither does any other test in the suite.

The current implementation does not color stderr messages, so this is a latent-risk finding, not a present bug. But the **automated regression boundary for "no ANSI codes when piped"** is asymmetric: it pins stdout and silently allows stderr. A reasonable contract reading of DESIGN.md L239 ("Color is suppressed when stdout is piped or redirected") is that color suppression applies to user-visible output channels in general — `tracker list | cat 2>&1` would mix the streams and re-introduce ANSI through the back door.

**Severity: Low-Medium.** Latent / preventive. The exact form depends on whether the spec intends "no ANSI on stderr when stderr-is-piped" as a separate rule (currently DESIGN.md is silent on stderr coloring). If so, the test gap is real; if not, this is a documentation gap (SO-raisable).

**Evidence:** `tests/layer7.rs:163-173` and `:191-192` (stdout-only assertions); `src/lib.rs:822-827` (`eprintln!` empty-state messages, uncolored, no TTY check); DESIGN.md L222, L239.

**Proposed action (Round 2):** Extend the piped tests to also assert `.stderr(predicate::str::contains("\x1b[").not())`. Cost: 4 added lines. If SO confirms stderr-color is out of scope, this becomes a doc clarification instead.

**Classification:** Open / Low-Medium severity / cross-cut to SO (spec scope).

---

##### Finding 5 — `pad_after_color`'s `visible_chars` is computed by the caller as `issue.status.chars().count()`; relies on chars-count equaling display-width, which is true for the current ASCII values but not in general (Dim 6 — validation gaps; latent risk)

`cmd_list` (L856, L859) calls `pad_after_color(&status_colored, issue.status.chars().count(), 11)` and analogously for priority. The `pad_after_color` doc-comment (L91-97) and the code-site comment (L851-854) both acknowledge "All status/priority values are ASCII so visible character count equals byte count of the bare value" — but `chars().count()` is the **character count**, not the **display width**. For the current valid set (`open`, `in-progress`, `done`, `low`, `medium`, `high`) all values are ASCII so character count equals display columns; the current code is correct.

But: `issue_fields_are_valid` validates the *set* of valid status/priority strings against `VALID_STATUSES` / `PRIORITY_ORDER`, so a non-ASCII value cannot reach this code path through any legitimate flow. The defense is layered. **However**: the comment treats this as "fine for current values" — if a future spec amendment added a CJK status value (the prompt's `"完成"` hypothetical), `chars().count()` would return 2 while display width would be 4, mis-aligning columns. The `unicode-width` crate is the standard fix.

**Severity: Low / Latent.** Not a present bug — the validated value set is ASCII-only and `tracker_is_valid` rejects anything else at load. Flagging because: (a) the comment frames the constraint as a coincidence ("All ... values are ASCII") rather than as a *validated invariant*, which is fragile to a future spec amendment; (b) there is no unit test asserting that `pad_after_color` aligns correctly for the current value set (i.e., that `chars().count() == display_columns` for every member of `VALID_STATUSES` and `PRIORITY_ORDER`).

**Evidence:** `src/lib.rs:851-859` (caller comment); `src/lib.rs:91-97` (helper doc); `src/lib.rs:203-207` (`VALID_STATUSES`, `PRIORITY_ORDER`).

**Proposed action (Round 2):** Add a unit test enumerating each valid status and priority value and asserting `chars().count() == <expected ASCII width>`. If the project adopts CJK status values in the future, the test breaks loudly and points at the fix (`unicode-width::UnicodeWidthStr::width`). Cost: 1 unit test, ~15 LOC.

**Classification:** Open / Low severity / latent risk.

---

#### Dismissed

##### Finding 6 — Manual checklist closure of 7/7 in a single commit (`603c689`) may have been performative

The manual checklist (TODO.md L368-376) shows all 7 items checked. Commit `603c689` closed them together. Adversarial question: was each item materially executed, or batch-ticked?

**Classification: Dismissed.** Cross-referencing the CHANGELOG.md L27-32 "Manual TTY verification" section, the developer recorded specific evidence: `script -q /dev/null tracker list ...` invocations for each priority/status combination, with the exact ANSI sequences observed (`\x1b[1;31mhigh\x1b[0m`, `\x1b[33mmedium\x1b[0m`, etc.). The `tracker frobnicate` exit-1 case is independently covered by the integration test. The `tracker show <id>` piped case is covered by `show_piped_has_no_ansi_codes`. The remaining items (error-message review, TTY show with high/in-progress) are described with detail consistent with actual execution. The shape of the CHANGELOG narrative — specific escape sequences, column-alignment claim across all 9 status × priority combinations — would be hard to fabricate plausibly. **No finding.** (Caveat: the developer is the same agent that wrote the tests and implementation. The closure-validation chain ultimately depends on director trust. This is a process-level concern, not a QE-domain finding — surfacing only as a cross-domain flag to VDD-IAR Alignment.)

---

##### Finding 7 — `help_flag_create_exits_zero` asserts the literal `"low, medium, high"` ordering of the priority enumeration in the help text; a refactor reordering to `"high, medium, low"` would break the test even though the AC is satisfied (Dim 5 — test bound to implementation, not behavior)

The test asserts `predicate::str::contains("low, medium, high")`. DESIGN.md L21 specifies the valid values as `low`, `medium`, `high` but does not pin the help-text rendering order. A reader could reasonably order them by priority severity (`high, medium, low`) and the test would fail despite no AC regression.

**Classification: Dismissed.** The assertion is on the substring; if a future refactor changes the order, the test fails *loudly* and the developer can choose whether the spec means to pin order or not. This is appropriate falsifiability — looser than a regex would allow the test to silently pass against a re-ordering, which is *less* useful as a regression guard. The test correctly reflects current behavior; any future re-ordering surfaces the ambiguity for SO adjudication. **No finding.**

---

#### Hallucinated

##### Finding 8 — `wrap_color` allocates a `String` on every call; in a 1000-issue list this is 1000 allocations on the hot path

**Classification: Hallucinated.** `cmd_list` is the only caller; it runs at human-interactive cadence on a personal tracker. Performance is not a Layer 7 dimension and DESIGN.md does not specify allocation budgets. The `String::to_string()` fallback in the `None` branch already commits to allocation. This is a non-issue for the spec's threat model (single user, <1000 issues, terminal-interactive use). **No finding.**

---

### Mutation-resilience verdict

**Mixed-to-loose.** The color-helper surface (Findings 1, 2) is the most mutation-vulnerable in the project: 7+ named mutations on `priority_ansi` / `status_ansi` / `wrap_color` survive every test in the 195-suite. The piped-no-ANSI integration tests pin the *suppression* half of the contract (which is the genuinely new Layer 7 risk surface for regression) but cannot pin the *application* half because of the structural TTY-detection limitation that the implementation chose to leave unautomatable. The stdout/stderr separation contract (Finding 3) is asserted positively per stream but not negatively, which is a Dim 3 assertion-strength gap.

The `--help` / unknown-subcommand surface is appropriately covered by Cat B regression-guard tests (Red Gate compliance disclosed, not enforced).

**Compared to Layer 6 (Review 15):** Layer 6's QE round surfaced 3 substantive findings (1 High security-cross-cut + 2 Medium) that landed in Round 2. Layer 7 surfaces 5 (3 Medium-High / Medium on the test surface + 2 Low latent), all of which are quality-system gaps rather than security findings — none merge-blocking, but Findings 1-3 should not survive a second cold-batch round.

### Summary

**Verdict:** **MVR-pending.** Layer 7 implementation is correct against DESIGN.md per manual testing, but the automated test boundary is loose on the color contract. 5 Open findings / 2 Dismissed / 1 Hallucinated. Findings 1-3 are recommended for a Round-2 QE closure pass (adding ~20 unit tests + a `force_color` env-var test seam in `src/lib.rs` co-authored with SE).

**Top mutation gap:** Finding 2 — `priority_ansi` / `status_ansi` / `wrap_color` have no unit tests; 7 named mutations on the color-sequence contract survive the suite.

**Top concern:** Finding 1 — the entire TTY-positive rendering surface (ACs 7, 8, 10, 11 — 4 of 13 Layer 7 ACs) has zero automated coverage. The TODO.md "Manual only" carve-out is true for the current implementation but is a structural choice, not a constraint — a `force_color` seam is the standard idiom.

**Sycophancy check:** 195/195 + clippy clean is the result that softens reviewers. Re-audited Dim 1 (AC coverage — found 4 ACs at zero automated coverage by enumerating the AC list rather than reading the test list and reverse-mapping); Dim 2 (mutation testing — enumerated 7 surviving mutations on pure functions rather than asserting "tests pass = mutation resilience"); Dim 13 (quality gates — flagged manual-only checklist as a CI-invisible gate per the Standard Dim's literal text). Did not soften any finding to Dismissed without specific control demonstration; Finding 6 demonstrates the control via cross-reference to CHANGELOG evidence with shape-of-narrative reasoning. **No softening detected.**

**Red Gate verdict:** Deviation disclosed in tests/layer7.rs L1-25 and in the `7b461aa` commit message; framing is honest. The deviation is real, not a hallucinated process gap — Finding 1's `force_color` seam is the concrete alternative that would have enabled a Red-first approach to the color contract. Logged as a process observation cross-cut to VDD-IAR Alignment.

**Coordination:**
- **SE Review (next):** Finding 1's `force_color` env-var seam is a `src/lib.rs` change (one line in each of `cmd_list` / `cmd_show`); QE provides the test additions in Round 2 once the seam lands. Finding 5's `unicode-width` defense is SE's call on whether to adopt the dependency or document the ASCII-invariant.
- **UX Review (next):** Finding 1's env-var name choice (`TRACKER_FORCE_COLOR` vs. `CLICOLOR_FORCE` vs. `NO_COLOR` honored) is a UX-convention question. Finding 3 (stdout/stderr separation) cross-cuts to UX dim 4 (CLI supplement).
- **SO Review (next):** Finding 4 (ANSI-on-stderr) is silent in DESIGN.md L239 — needs SO adjudication on whether color suppression applies to stderr empty-state messages or whether stderr is out of scope for the color contract. Finding 5 (CJK / display-width) is a spec-amendment question if status values ever widen beyond ASCII.
- **Platform Engineer Review (next):** No new escalation. The 195/195 green CI run does not change the standing Platform finding on coverage-gating absence (Layer 7 does not move that needle).
- **VDD-IAR Alignment (next):** Process observation — the Red Gate deviation is disclosed in the commit and test file (compliant disclosure); the deviation itself is a process gap that the `force_color` seam (Finding 1) would close architecturally. Manual checklist closure (Finding 6 dismissed) materially executed per CHANGELOG narrative review.
- **Security / Red Team:** No new escalation; no security-class findings this round.

---

## Review 18 — 2026-05-12 00:00Z

**Round:** QE Review 18 (Layer 7 IAR Round 2 closure pass). Warm verification per CLOSURE-PROTOCOL.md §5; not a new adversarial round.

**Scope:** Verify R17 Open findings closed by commits `fbbb8a3` (12 retroactive unit tests) and `09b1905` (test tightening + Round-2 unit and integration tests). Inputs: `tests/layer7.rs` (now 12 tests, was 9); `src/lib.rs#tests` (now 84 tests, was 62).

### Round-1 finding closures

- **F1 — TTY-positive color rendering has zero automated coverage; `force_color` test seam would have made it automatable:** **Deferred (Round-2 partial mitigation applied).** A `force_color` test seam still requires refactoring (a public `set_color_mode_for_test` hook or env-var override path that bypasses TTY detection). Partial mitigation in `09b1905`: 4 new `color_mode_from_env_*` unit tests pin the env-var precedence logic (NO_COLOR / NO_COLOR-empty / CLICOLOR=0 / piped-stdout-wins-over-CLICOLOR_FORCE), serialized via `ENV_TEST_LOCK` mutex to avoid env-var races. The unit-level color contract is now comprehensively covered; the integration-level TTY-positive path remains manual-only per TODO.md. Deferred to a future polish layer with a force_color seam.
- **F2 — No unit tests for `priority_ansi` / `status_ansi` / `wrap_color` / `pad_after_color`:** **Resolved by `fbbb8a3` (12 retroactive tests) + `09b1905` (test updates).** Tests now reflect Round-2 ColorMode enum signatures and bold-redundancy values: `priority_ansi("medium", On)` → `\x1b[1;33m`, `status_ansi("in-progress", On)` → `\x1b[1;36m`, `status_ansi("done", On)` → `\x1b[1;32m`. The previously enumerated 7 mutation-survival permutations are now killed: bold-drop on high, bold-drop on medium, yellow↔green swap, reset-drop in `wrap_color`, color-emitted-when-Off, missing color when value highlighted, padding miscalculated under color.
- **F3 — `unknown_subcommand_exits_one` does not assert stdout empty; `help_flag_*` tests don't assert stderr clean:** **Resolved by `09b1905`.** `unknown_subcommand_exits_one` now asserts `stdout("")`. The new `unknown_subcommand_with_cc_payload_escapes_in_stderr` test additionally asserts `stdout("")` while pinning the Cc-escape rule on stderr.
- **F4 — `list_piped_has_no_ansi_codes` / `show_piped_has_no_ansi_codes` check only stdout:** **Resolved by `09b1905`.** Both tests now assert `stderr(predicate::str::contains("\x1b[").not())`. New `list_empty_state_stderr_has_no_ansi_codes` test pins the symmetric stderr cleanliness when `list` emits the "No open issues. Nice work!" empty-state message — closing the previously-untested stderr ANSI-leak surface.
- **F5 — `pad_after_color`'s `visible_chars` arg relies on chars-count-equals-display-width:** **Deferred — latent risk only.** No current status/priority value exercises non-ASCII (all enum values are ASCII); the SE R17 F2 refactor renamed the helper to `render_cell` with the bare value passed directly (visible_chars computed internally), reducing the API surface that could mis-handle wide characters, but the helper still uses `.chars().count()` rather than `unicode-width`. Re-evaluation trigger: any future spec amendment allowing non-ASCII status/priority values.

### Test delta

- Pre-R2: 195/195 pass (62 unit + 32+18+9+25+7+33+9 layer 1-7 integration).
- Post-R2: **220/220 pass** (84 unit + 32+18+9+25+7+33+12 layer 1-7 integration). Delta: +22 unit (12 retrofit in `fbbb8a3` + 6 ColorMode/env-var/debug_assert + 4 sanitize_quoted_values); +3 integration (unknown_subcommand_with_cc_payload_escapes_in_stderr, list_empty_state_stderr_has_no_ansi_codes, no_color_env_does_not_break_piped_invocation).

### New findings

*(none — closure pass.)*

### Summary

3 of 5 R1 findings Resolved (F2, F3, F4); 2 Deferred with rationale (F1 force_color seam, F5 CJK display-width latent). Mutation resilience materially improved: the 7 enumerated R1 mutation-survival permutations on the color-helper contract are now killed by the retrofit + R2 update tests. Stderr discipline tightened symmetrically with the previously-stdout-only piped tests.

**Coordination:** VDD-IAR R18 — Red Gate retroactive-test evidence chain documented per implementation.md L56 (12 retrofit tests labelled `// retroactive Red Gate:` in source comments); SE R18 — `render_cell` refactor verified by the rewritten `render_cell_*` unit tests.

**Files modified:** This log appended only. The test additions and tightenings landed in `09b1905` under QE authority (tests/** + src/lib.rs#tests per CLOSURE-PROTOCOL.md §1).

---

## Review 19 — 2026-05-12 12:00Z

**Round:** QE Review 19 (Layer 7 IAR Round 3). Cold adversarial session.

**Scope:** R3 change set (5 commits since `b853a81`): clippy pre-commit hook (`ff0e85c`), `render_cell` ASCII debug_assert + unit test (`c341a54`), `TRACKER_INTERNAL_FORCE_COLOR` seam + 8 integration + 2 unit tests (`bd7511e`), cmd_list rendering extraction + column-width constants + 6 unit tests (`3fa1f3c`), three-module split (`8db9437`). Test count 195 → 237 (+42). Regression check across all 7 layer suites.

**Verdict:** R3 closes the R17 Deferred items materially (F1 force_color seam + 8 integration tests, F5 debug_assert + 1 unit test). Mutation resilience on the colored-cell surface is materially better than R2. Three new substantive Open findings on the R3 commits — none merge-blocking, but each surfaces a gap the cold session's adversarial pressure surfaced that the warm closure pass did not.

### Findings

#### Open

##### Finding 1 — `format_list_row_colors_high_priority_when_color_on` uses unanchored substring containment; a column-order swap (status ↔ priority) survives the test (Dim 2 mutation testing, Dim 3 assertion strength)

`src/lib.rs:551-560` asserts `row.contains("\x1b[1;31mhigh\x1b[0m")` and the absence of cyan/green sequences. The format string at `src/commands.rs:572-582` is `"{:<id_width$}  {}  {}  {:<labels_width$}  {}"` where positionals are `issue.id`, `status_cell`, `priority_cell`, `labels_display`, `title_display`. A mutation that swaps the `status_cell` and `priority_cell` positional arguments (a one-character refactor regression) produces a row where the priority value renders in the status column and vice versa. `row.contains("\x1b[1;31mhigh\x1b[0m")` is true under the mutation; absence-of-cyan/green is also true (the test issue is `open` + `high`, neither carries cyan or green). The mutation survives.

Companion test `format_list_row_uncolored_when_color_off` (`src/lib.rs:540-549`) asserts `row.starts_with("1   ")` and `row.ends_with("x")` — the title and ID columns are pinned, but the status/priority cells between them are unanchored. Same mutation survives.

The R3 integration tests in `tests/layer7.rs:280-501` partially compensate at the integration boundary (`force_color_emits_bold_red_for_high_priority` etc.) but use `predicate::str::contains` — same unanchored-substring shape. Only `force_color_show_renders_colored_status_and_priority_value_cells` uses a positionally-anchored assertion (`"Status:      \x1b[1;36min-progress\x1b[0m"` — the label prefix pins where the colored value appears). The `list` integration tests have no equivalent positional anchor.

**Severity: Medium.** Real mutation gap on the load-bearing column-ordering contract from DESIGN.md "List output format" (the example shows `ID Status Priority Labels Title` order). A future refactor that re-orders columns silently passes the R3 unit + integration tests; the only catch is the manual checklist.

**Proposed action:** Add positional assertions to `format_list_row_colors_high_priority_when_color_on` — e.g., assert `row` matches a regex anchored at column boundaries, or assert the full row equals a known-good string for a fixed test issue. Mirror on the integration side: `force_color_emits_bold_red_for_high_priority` should assert the colored `high` value appears at the priority column offset (after status, before labels), not just somewhere in the row.

**Classification:** Open / Medium severity.

##### Finding 2 — `force_color_with_no_color_env_set_does_not_force` test name is contradictory to its assertion; the body comment acknowledges this but the function-name reads opposite to what is tested (Dim 3 — assertion strength / behavioral naming; TDD proxy indicator dim 14)

`tests/layer7.rs:474-502` is named `force_color_with_no_color_env_set_does_not_force` — the natural English reading is "when NO_COLOR is set, force-color does NOT win." The assertion is `.stdout(predicate::str::contains("\x1b[1;31m"))` — force-color DID win and ANSI WAS emitted. The body comment from L481-485 explicitly acknowledges: "Actual current behavior: the seam... DOES win over NO_COLOR. This test documents that fact and pins it as the deliberate test-ergonomics choice."

The naming-vs-behavior inversion is a real falsifiability hazard. A future reader scanning the test list (e.g., a developer reading `cargo test -- --list` or a reviewer auditing AC coverage) sees a test name that says force-color is suppressed by NO_COLOR — exactly the opposite of what the test pins. If the precedence were later reversed (NO_COLOR wins over the seam, which is arguably the user-safer default), the test name would suddenly become accurate but the test body would fail — the test that should have changed to match the new contract is the one whose name was already right. This is the structural inverse of "behavioral naming" — the QE Standard Dim 14 specifically asks "Are tests named for expected behavior rather than code structure?"

The body comment is honest and shows the developer noticed the inversion. The correct remedy is renaming, not commenting around the inversion.

**Severity: Low-Medium.** Documentation / naming bug, not a logic bug. But the test is documenting a deliberate precedence choice (seam wins over user opt-out) that is the kind of choice that should be unambiguous in its test surface, since the precedence itself is a security-adjacent decision (a test-only env var overruling a documented user-safety opt-out).

**Proposed action:** Rename to `force_color_seam_wins_over_no_color_in_tests` or `force_color_precedence_seam_first_then_no_color`. The body comment becomes redundant once the name accurately describes the assertion. Cross-cut to Security R12 (the precedence ordering itself).

**Classification:** Open / Low-Medium severity.

##### Finding 3 — Unit tests post-split live in `lib.rs#tests` accessing `commands.rs` and `storage.rs` internals via `pub(crate)` + glob imports; the module split widened the test-visible API surface and broke the rust supplement's "colocated `#[cfg(test)]` modules" idiom (rust supplement Quality Engineering / Test structure)

The Rust supplement names: "Are unit tests colocated with the code they test (`#[cfg(test)]` modules)?" Post-split (`8db9437`), every unit test still lives in `lib.rs#tests` (L51-1133) — 93 tests testing items that now live in `commands.rs` (filter / sort / render helpers, `ColorMode`, color_mode_from_env, etc.) and `storage.rs` (Tracker / Issue / issue_fields_are_valid / tracker_is_valid). The tests reach those items via `use crate::commands::*; use crate::storage::*;` (L54-55) glob imports.

Two consequences:

1. **Test-target colocation lost.** A maintainer modifying `commands.rs::format_list_row` does not see the test in the same file. The split's stated purpose (SA R13 F1 Trigger B closure — separating concerns) is undermined for the test surface: the tests are no longer concern-aligned with the implementation they exercise.

2. **Test-visible API surface widened.** Items that were previously module-private (`fn issue_matches_filters`, `fn priority_rank`, `fn truncate_with_ellipsis`, `fn format_show_block`, `fn show_label`, `fn priority_ansi`, `fn status_ansi`, `fn wrap_color`, `fn render_cell`, `fn filter_issues`, `fn format_list_header`, `fn format_list_row`, const `ID_WIDTH`/`STATUS_WIDTH`/etc., `const LABEL_COLUMN_WIDTH`) now all carry `pub(crate)` to be accessible from `lib.rs#tests`. The split did not introduce a new public API — but it did promote every previously-private item to crate-visible. A future contributor inspecting the module's surface sees ~15 `pub(crate)` items where there were ~0 before. The rust supplement's intent — colocated tests against truly private items — is silently inverted.

The Round-2 closure rationale at SA R13 F1 was that lib.rs exceeded a threshold and concerns should be separated. The split achieves that for production code. The test surface should follow: each module's `#[cfg(test)] mod tests` accessing items via `use super::*` (without `pub(crate)`), and `lib.rs#tests` retaining only cross-module integration-style unit tests.

**Severity: Medium.** Quality-system architectural finding. The 93 tests still pass; this is not a correctness gap. It is a maintainability + idiom-compliance gap that the cold session surfaces against the rust supplement's named criterion. The fact that 237/237 pass against a structurally degraded test layout is exactly the "passing test suite + clippy clean lulls the reviewer" trap.

**Proposed action:** Move tests by target module: `commands.rs`-targeted tests (filter, sort, render, color helpers, ColorMode, color_mode_from_env, debug_assert tests) into `src/commands.rs` `#[cfg(test)] mod tests`. `storage.rs`-targeted tests (issue_fields_are_valid, tracker_is_valid, description_is_valid, label_is_valid, parse_timestamp) into `src/storage.rs`. `validate.rs`-targeted tests (parse_status, parse_priority, parse_label, parse_id, validate_title, validate_description, bump_next_id, display_safe, sanitize_quoted_values, dedupe_labels) into `src/validate.rs`. Demote `pub(crate)` markers on items that are now reachable from a same-module test via `use super::*` (specifically: the rendering helpers, the column-width constants, `priority_rank`, `truncate_with_ellipsis`, `issue_matches_filters`, `format_show_block`, `format_list_header`, `format_list_row`, `filter_issues`, `show_label`, `LABEL_COLUMN_WIDTH`). Cost: ~93 test-move operations + ~15 visibility-marker reductions. No test logic changes. Cross-cut to SA (the split's intent) and SE (the visibility changes).

**Classification:** Open / Medium severity. Deferrable to a follow-up layer; the R3 split was net-positive even with this gap.

##### Finding 4 — `wrap_color_debug_assert_active_in_debug_builds` and `render_cell_debug_assert_on_non_ascii_value` rely on `catch_unwind` + a panicking `debug_assert!`; both tests print panic-noise to test stderr and tie the test contract to the default `panic=unwind` strategy (Dim 5 — test architecture / flakiness; Dim 2 — falsifiability)

`src/lib.rs:1106-1117` and `src/lib.rs:1119-1132` both use the pattern `std::panic::catch_unwind(|| <call>); assert!(result.is_err(), ...)`. Two structural concerns:

1. **Panic-noise in `cargo test` output.** A `debug_assert!` panic, even under `catch_unwind`, prints the panic message to stderr unless a custom panic hook silences it. Running `cargo test -- wrap_color_debug_assert_active_in_debug_builds 2>&1` shows the `thread '...' panicked at...` line in the test output. This is cosmetic but degrades the "237 tests; 0 failed" signal — a developer scanning for unexpected output sees panic stderr that is part of the test's expected behavior. The standard remedy is `std::panic::set_hook(Box::new(|_| {}))` + restore-after, but the current tests skip this.

2. **Contract bound to `panic=unwind`.** Cargo.toml (`issue-tracker-cli/Cargo.toml:1-33`) does not set a panic strategy — the default for test binaries is `unwind`, so `catch_unwind` works. If a future contributor adds `[profile.test] panic = "abort"` (common in embedded / no_std contexts; explicitly compatible with `cargo test --release` benchmarks), `catch_unwind` no longer catches — the test process aborts on the `debug_assert!` and the test framework records it as a non-test-failure crash. Neither test surfaces this constraint in its body comment.

Reliability across threads: the closures are self-contained and do not mutate shared state, so cross-thread races are not the concern. The concerns are noise + future-portability of the test contract.

**Severity: Low-Medium.** Tests pass today and reliably. The pattern is fragile to (a) panic-strategy changes and (b) developer attention budget when a clean test run includes panic stderr that has to be cognitively filtered. Both are real but small.

**Proposed action:** Wrap the catch_unwind calls in `set_hook` + restore, or — preferred — refactor `wrap_color` and `render_cell` to return `Result<String, &'static str>` for the contract-violation case, eliminating the panic surface entirely. The latter is a SE-domain refactor (it changes a private API); the former is a QE-domain test fix.

**Classification:** Open / Low-Medium severity.

#### Resolved

##### Finding 5 — R17 F1 closure verification: TTY-positive color rendering coverage

The R17 deferred F1 (force_color seam) is closed by `bd7511e`. The 8 new integration tests in `tests/layer7.rs:280-501` exercise: bold-red on high priority (L280-294), bold-yellow on medium (L297-311), no-color on low (L314-333), bold-cyan on in-progress (L336-354), bold-green on done (L357-375), header-uncolored (L378-414), show-renders-colored-value-cells (L417-471), force-color-precedence (L474-502 — but see Finding 2 for the name issue). The 2 new unit tests in `src/lib.rs` (`color_mode_from_env_on_when_internal_force_color_set`, `color_mode_from_env_force_color_ignored_for_non_one_values`) pin the env-var precedence logic.

Mutation analysis: the prior R17 unkilled mutations (bold-drop on high, yellow↔green swap, color-leaked-to-header) are now killed. Coverage covers every color value in the DESIGN.md table (high, medium, low, open, in-progress, done — 6/6). Open gap: column ordering (Finding 1 above) — a position-swap survives.

**Classification:** Resolved with residual mutation gap captured in Finding 1.

##### Finding 6 — R17 F5 closure verification: CJK display-width debug_assert

The R17 deferred F5 (`render_cell` `chars().count()` correctness for non-ASCII) is closed by `c341a54`. `src/commands.rs:218-234` adds a `debug_assert!(value.is_ascii(), ...)` with a doc-comment naming the constraint and the remediation path (`unicode-width` crate). The accompanying unit test `render_cell_debug_assert_on_non_ascii_value` (`src/lib.rs:1119-1132`) verifies the panic fires.

Honest closure analysis: the debug_assert is debug-only — release builds compile it out, so the latent risk in a release binary remains. But the call-site invariant (`render_cell` is called only with `issue.status` or `issue.priority`, both validated against ASCII enums at parse and load time) means the constraint cannot be violated in production by current code. The debug_assert is defense-in-depth, not a fix — and the doc-comment is honest about that.

The closure is appropriate for the latent risk: a future spec amendment permitting non-ASCII status/priority values would (a) require validating the new value set, (b) hit the debug_assert in the test suite first (current path is to extend `VALID_STATUSES` / `PRIORITY_ORDER` first, which would surface the gap before the new value reaches `render_cell`), and (c) trigger the named remediation (introduce `unicode-width`).

**Classification:** Resolved as defense-in-depth; production risk surface unchanged (already gated by upstream validation).

#### Dismissed

##### Finding 7 — clippy pre-commit hook does not fire on Cargo.toml changes; a dependency-version bump or feature-flag toggle that affects clippy output is missed

The hook fires on `^issue-tracker-cli/.*\.rs$` (`.pre-commit-config.yaml:50`). A change to `Cargo.toml` (e.g., enabling a new feature flag, bumping `clap` major version, adding a new dependency) can change clippy's analysis surface — new lints become applicable, or existing suppressions silently no-op. The hook would not fire on a Cargo.toml-only commit.

**Classification: Dismissed.** Verified the R3 commit set: no Cargo.toml changes in any R3 commit (`git diff b853a81..HEAD --stat` shows only `.pre-commit-config.yaml`, `PROCESS.md`, `PORTFOLIO-ASSESSMENT-REVIEW.md`, `src/*.rs`, `tests/layer7.rs`, and the lib.rs split). The hypothetical Cargo.toml-only commit is not in scope for R3. The hook's narrow `*.rs` filter is a deliberate ergonomic choice — running cargo clippy on every docs commit would be wasteful. A future Cargo.toml bump that broke clippy would fail in CI (`cargo clippy -- -D warnings` runs there) within the same PR cycle. Cross-cut to PE for a wider hook trigger if Cargo.toml-related clippy regressions ever land.

#### Hallucinated

##### Finding 8 — `filter_issues_returns_only_matching` does not pin the issue identity in the returned vec; a mutation that returns the wrong matching issue passes

The test (`src/lib.rs:513-523`) asserts `out.len() == 1` and `out[0].status == "open"` and `out[0].priority == "high"`. A mutation that returns a different issue with the same (status, priority) signature could pass.

**Classification: Hallucinated.** The three test inputs are `("open", "high")`, `("done", "high")`, `("open", "low")`. The filter is `(status="open", priority="high")`. Only the first input matches — there is exactly one issue with that (status, priority) signature in the test. The test assertion `len==1 && status="open" && priority="high"` uniquely identifies the right issue against the test's specific input set. A mutation that returned the wrong issue would either change the count or change the asserted fields. No real coverage gap. **No finding.**

### Mutation-resilience verdict

**Materially improved over R2.** The 8 force_color integration tests + 2 force_color unit tests close the most-mutable surface from R17 (color application on TTY-positive path). The retroactive Red Gate tests on color helpers from R2 remain. New gaps surfaced by cold session: column-order mutation in `format_list_row` (Finding 1) survives. The 6 newly extracted `filter_issues`/`format_list_header`/`format_list_row`/`show_label` unit tests are mostly sound; `show_label` has the strongest mutation resilience (exact-string + width-invariant loop); `filter_issues` has 2 tests covering a 4-dim filter space (status × priority × label × issue-list-shape) — adequate floor, not exhaustive.

### Summary

**Verdict:** R17 deferred items materially closed; R3 introduces 3 new substantive findings (Finding 1 column-ordering mutation gap, Finding 2 test-name inversion, Finding 3 post-split test colocation) + 1 latent (Finding 4 debug_assert + catch_unwind pattern). 237/237 + clippy clean + fmt clean is the result that lulls reviewers — the cold session specifically pressed on coverage *meaningfulness* and surfaced mutation gaps the warm closure pass did not. None merge-blocking.

**Top mutation gap:** Finding 1 — `format_list_row` column-order swap survives every R3 unit and integration test for the colored-row case.

**Top architectural concern:** Finding 3 — post-split tests still live in `lib.rs#tests` with widened `pub(crate)` surface; the rust supplement's "colocated `#[cfg(test)]` modules" criterion is silently inverted by the split.

**Sycophancy check:** Cold session explicitly searched for mutations the warm R18 closure pass did not enumerate — found 2 real ones (column-order in Finding 1, panic-noise + panic-strategy fragility in Finding 4). Did not pass any dimension because "tests exist and pass"; mutation-enumerated where the supplement names mutation testing. Re-read R3's integration tests against the *contract* (column ordering per DESIGN.md "List output format" example), not against the *implementation* (the format string positional order); Finding 1 surfaced from that gap. Finding 3 required reading the supplement criterion in conflict with the split's stated rationale rather than accepting the doc-comment's framing. **No softening detected.**

**Coordination:**
- **SE Review (next):** Finding 3 cross-cuts SE — the test-relocation + visibility reduction is a src/** change. Finding 4's preferred remediation (Result-returning helpers replacing debug_assert) is SE-authored.
- **SA Review (next):** Finding 3 directly cross-cuts SA — the split's stated rationale (SA R13 F1 Trigger B) is undermined for the test surface. SA should adjudicate whether the test relocation is part of the split's intent.
- **UX Review (next):** Finding 2 is a behavioral-naming question; UX may have a view on whether the test name conveys the correct user-facing contract about NO_COLOR honoring.
- **Security Review (next):** Finding 2 names a security-adjacent precedence (test-only env var overrides user-safety opt-out); Security R12 may want to verify the precedence is correctly scoped to the test-only seam and cannot leak into production.
- **VDD-IAR Alignment (next):** No new escalation. The R17 deferred items closed materially; the Red Gate retroactive-test framing from R2 is unchanged.
- **Platform Engineer (next):** Finding 7 (Dismissed) is a forward-looking PE consideration if the hook trigger needs widening; not actionable now.

**Files modified:** This log appended only. No source / test changes proposed for R3 closure beyond the proposed-action sections above; classification owner (Solution Owner) decides whether R3 Open findings merit a Round 4 or are deferred to a later polish layer.
