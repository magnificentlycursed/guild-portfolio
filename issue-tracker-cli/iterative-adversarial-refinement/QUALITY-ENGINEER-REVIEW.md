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
