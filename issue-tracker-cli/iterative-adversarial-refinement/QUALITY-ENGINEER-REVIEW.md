# Quality Engineer Review Log

This review is part of the [Iterative Adversarial Refinement (IAR)](README.md) suite. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

The purpose of this review is to evaluate the quality system: whether the testing strategy, coverage, tooling, and gates are structured to catch defects reliably. At this stage (pre-implementation), the review evaluates the test plans in `TODO.md` — their completeness, Red Gate compliance, and falsifiability.

**Language supplement applied:** `lang/rust.md` (QE section) + `lang/cli.md` (QE section).

---

## Review 1 — 2026-04-27 21:00Z

**Scope:** `TODO.md` Red Gate test plans and `DESIGN.md` Testing Methodology. No implementation code exists. Pre-implementation pass: evaluating the quality of the test plan before any code is written.

**Session note:** In-session with all other domain reviews and with project authorship. Acknowledged quality tradeoff.

**Assumption surfacing:** No implementation dependencies to validate yet. The test plan references integration tests that invoke the binary as a subprocess — consistent with the Rust supplement's requirement for CLI integration tests.

---

### Resolved

**Finding 1 — No Red Gate test for `created_at == updated_at` on fresh issue**

DESIGN.md states: "`created_at` and `updated_at` are equal on a freshly created issue." This is an acceptance criterion in Layer 1 (`TODO.md`) but no corresponding Red Gate integration test exists. A stub that sets `updated_at` to epoch or to a different timestamp would pass all existing Red Gate tests.

**Resolution:** Added to `TODO.md` Layer 1 Red Gate: `create_timestamps_equal_on_fresh_issue` — reads `tracker.json` after create, asserts `created_at == updated_at` — fails against stub that sets them to different values.

---

**Finding 2 — No Red Gate test for title 50-char truncation in list output**

Layer 1 acceptance criteria include: "Title truncates at 50 characters with `…` in list output." No corresponding Red Gate test exists. A stub that prints the full title would satisfy all existing Layer 1 tests.

**Resolution:** Added `list_truncates_title_at_50_chars_with_ellipsis` to `TODO.md` Layer 1 Red Gate.

---

**Finding 3 — No Red Gate test for `tracker list --status in-progress`**

Layer 2 Red Gate names `list_status_filter_shows_done` for the done case but has no corresponding test for the `in-progress` case. A stub that only handles `open` and `done` status filters would pass all Layer 2 Red Gate integration tests.

**Resolution:** Added `list_status_filter_shows_in_progress` to `TODO.md` Layer 2 Red Gate.

---

**Finding 4 — No Red Gate test for `created_at` immutability after status mutation**

Layer 2 acceptance criteria include: "All other fields on the issue are unchanged [after status change]." The `status_change_leaves_other_fields_unchanged` test covers this, but `created_at` is not explicitly named. An implementation that updates `created_at` on mutation would pass the existing test if "other fields" is interpreted loosely. A distinct test that explicitly asserts `created_at` is unchanged removes the ambiguity.

**Resolution:** Added `status_change_does_not_modify_created_at` to `TODO.md` Layer 2 Red Gate.

---

**Finding 5 — No Red Gate test for label 20-char truncation in list output**

Layer 4 acceptance criteria include: "Labels column truncates at 20 characters with `…` if longer." No corresponding Red Gate integration test exists.

**Resolution:** Added `list_label_value_truncated_at_20_chars` to `TODO.md` Layer 4 Red Gate.

---

### Dismissed

**Finding 6 — Write-failure error paths have no Red Gate integration tests**

DESIGN.md Storage edge cases include: write fails (disk full, permissions) → stderr `Error: Could not save tracker data: <reason>.` → exit 1; `tracker.json` is a directory → treated as I/O failure. These are acceptance criteria but cannot be reliably automated in cross-platform integration tests (require OS-level setup: filling a disk, revoking file permissions, creating a directory at the expected path).

**Classification:** Dismissed. Write-failure paths are listed in the Layer 1 and Layer 6 manual testing checklists. These are the only paths for which manual-only verification is acceptable. The acceptance criterion is present; its verification path is manual. The layer gate checklist must include explicit steps for these paths before Layer 1 merges.

---

**Finding 7 — Layer 5 test plan has only 4 integration tests; compound filter permutations are sparse**

`tracker list --status open --priority high`, `--status open --label bug`, `--priority high --label bug`, and all three together are listed. The case `--status done --priority high --label bug` (a three-filter combination with a non-default status) is not explicitly tested. A stub that only AND-combines when `--status open` is present would pass.

**Classification:** Dismissed. The `list_two_filter_and_combination` and `list_three_filter_and_combination` tests are defined against an implementation that must AND-combine any combination, not just open-status combinations. The acceptance criterion "An issue that matches two of three filters but not the third does NOT appear" is tested with setup that produces clear counter-examples. Additional permutation tests can be added during implementation if a gap is discovered; the Red Gate coverage is sufficient to fail a naive stub.

---

### Open

*(none)*

---

### Summary

Five real findings, all resolved via `TODO.md` additions. Two dismissed with rationale. The test plan now covers all primary acceptance criteria with Red Gate tests. Write-failure paths are explicitly manual. No open items.

**Coordination:** QE Finding 4 (created_at immutability) surfaces to SE as an implementation concern — the mutation path must never touch `created_at`. Logged in SE log.

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

**Classification: Raised to SO Review 7.** DESIGN.md is a controlled spec document. QE does not apply changes to DESIGN.md. This finding and the proposed resolution are handed to SO for decision. See SO Review 7 Finding 1.

**Cross-reference:** Data Engineer Review 2 Finding 1 (schema), SO Review 7 Finding 1 (resolution).

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

**Finding 1 — `list_truncates_title_at_50_chars_with_ellipsis` does not assert the specific truncation point**

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

**Finding 2 — `create_first_issue_unchanged_after_second_create` does not verify `labels`, `created_at`, or `updated_at`**

The acceptance criterion is: "first issue is unchanged." The test asserts `id`, `title`, `status`, `priority` — but not `labels`, `created_at`, or `updated_at`. An implementation that resets `labels` to `null` or regenerates timestamps on every write (re-serializing all issues with fresh timestamps) would pass all 17 Layer 1 tests. The `create_timestamps_equal_on_fresh_issue` test only checks timestamps for a single-create case — it does not verify timestamps survive a second create.

**Resolution:** Added assertions for `labels`, `created_at`, and `updated_at` in `create_first_issue_unchanged_after_second_create`. `labels` must be `[]`; `created_at` and `updated_at` must be non-null strings identical to what was stored after the first create (captured before the second create and compared after).

---

**Finding 3 — `malformed_json_causes_error_exit` asserts only a substring of the user-actionable error message**

The test checks `predicate::str::contains("Could not read tracker data")`. The acceptance criterion specifies the full message: `Error: Could not read tracker data. The file may be corrupt. Delete tracker.json to start fresh.`

Two distinct code paths in `load_issues` produce different messages:
- File read failure: `format!("Could not read tracker data: {}.", e)` — includes OS error, omits delete instruction
- JSON parse failure: `"Could not read tracker data. The file may be corrupt. Delete tracker.json to start fresh."` — user-actionable

The test triggers the JSON parse failure path but would also pass if the implementation accidentally routed malformed-JSON errors through the file-read-failure message format, which omits the critical delete instruction.

**Resolution:** Updated the stderr assertion to check the distinguishing suffix: `predicate::str::contains("The file may be corrupt. Delete tracker.json to start fresh.")` — this uniquely identifies the parse-failure path and verifies the full user-actionable text.

---

### Dismissed

**Finding 4 — No doc tests on public API functions**

`lib.rs` exports `validate_title`, `next_id`, `current_timestamp`, `load_issues`, `save_issues`, `cmd_create`, `cmd_list`. None have `///` doc comments or doc test examples. The rust.md QE supplement lists "Doc tests compile and pass" as a dimension.

**Classification:** Dismissed. This `lib.rs` exists to enable integration testing of a binary crate, not to expose a library API for external consumers. The `pub` visibility is structural, not a publication contract. Doc test coverage for binary-internal modules is a Technical Writer concern per the rust.md TW supplement. All exported functions are exercised through integration tests and unit tests. No open item.

---

**Finding 5 — `save_issues` uses `.unwrap()` on `serde_json::to_string_pretty`**

`serde_json::to_string_pretty(issues).unwrap()` in `save_issues` could panic if serialization fails.

**Classification:** Dismissed from QE. `Issue` fields are `u64`, `String`, `Vec<String>`, and `Option<String>` — none of which can produce serialization errors (no NaN, no Inf, no reference cycles). The `.unwrap()` is on a provably-safe value, not a user-input path. This is an SE domain finding. Noted for SE coordination.

---

### Open

*(none)*

---

### Summary

Three real findings, all resolved via `tests/layer1.rs` changes: the truncation test now asserts the exact 49-char prefix; the "unchanged" test now covers `labels`, `created_at`, and `updated_at`; the malformed-JSON test now asserts the full user-actionable error suffix. Two dismissed with rationale. No open items.

**Coordination:** Finding 5 (`save_issues` `.unwrap()`) noted for SE as an SE-domain observation — the panic path is unreachable given `Issue`'s field types, but SE should document this invariant if the field types ever expand.

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
