# Issue Tracker CLI — Development Plan

Assignment: `apprentice-onboarding/02-the-methodology/02-tracking-your-work.md`
Spec: `DESIGN.md` (SO-reviewed, cleared for Layer 1)
Phase: 1 (TODO.md is source of truth)

---

## Layer 1: Create + List (core skeleton)

**Goal:** The user can create an issue by title and see it in a list that persists across invocations.

**Acceptance Criteria:**
- [x] `cargo new tracker` project compiles with `cargo build` — no errors
- [x] `tracker create "Fix bug"` exits 0 and prints exactly `Created issue #1: Fix bug`
- [x] `tracker create "  Fix bug  "` stores the title as `Fix bug` (trimmed) and prints `Created issue #1: Fix bug`
- [x] `tracker create ""` exits 1 and prints `Error: Title cannot be empty.` to stderr (nothing on stdout)
- [x] `tracker create "   "` exits 1 and prints `Error: Title cannot be empty.` to stderr
- [x] After `tracker create "Fix bug"`, `tracker.json` exists and contains an issue object with `id=1`, `title="Fix bug"`, `status="open"`, `priority="medium"`, `labels=[]`, `created_at` and `updated_at` as ISO 8601 UTC timestamps
- [x] `created_at` and `updated_at` are equal on a freshly created issue
- [x] Second `tracker create` produces `id=2`; `tracker.json` contains both issues; first issue is unchanged
- [x] `tracker list` with no `tracker.json` prints `No open issues. Nice work!` and exits 0
- [x] `tracker list` after two creates shows both issues in a table with header row: `ID`, `Status`, `Priority`, `Labels`, `Title`
- [x] List output uses the full sort algorithm (priority descending, then ID ascending within the same priority tier); since all issues default to `medium` priority at this layer, the effective output order is ID ascending. The sort algorithm must be the full algorithm from the start — not a simplified ID-only sort that would require refactoring in Layer 3.
- [x] Title truncates at 50 characters with `…` in list output; full title is stored untruncated in `tracker.json`
- [x] `tracker.json` containing malformed JSON causes any command to exit 1 and print `Error: Could not read tracker data. The file may be corrupt. Delete tracker.json to start fresh.` to stderr

**Not in this layer:** `--priority`, `--label`, `--description`, `--status` filter, `tracker status`, `tracker show`, `tracker delete`

**Manual Testing Checklist:**
- [x] Happy path: from a clean directory (no `tracker.json`), run `tracker create "First issue"` → verify exit 0, output `Created issue #1: First issue`, `tracker.json` created
- [x] Run `tracker create "Second issue"` → output `Created issue #2: Second issue`; open `tracker.json` and confirm both issues present, first issue unchanged
- [x] Run `tracker list` → verify table shows both issues with correct header, both with status `open`, priority `medium`, labels `(none)`
- [x] Empty state: delete `tracker.json`, run `tracker list` → output is exactly `No open issues. Nice work!`
- [x] Error state — empty title: `tracker create ""` → verify exit 1, stderr shows `Error: Title cannot be empty.`, no `tracker.json` created (or existing data unchanged)
- [x] Error state — whitespace title: `tracker create "   "` → same error
- [x] Error state — malformed JSON: write `{bad json}` to `tracker.json`, run `tracker list` → exit 1, stderr shows corrupt-file message
- [x] Persistence: create two issues, reinstall binary with `cargo install --path .`, run `tracker list` → data is intact
- [x] Long title: create an issue with a 60-character title → `tracker list` shows the title truncated at 50 chars with `…`; `tracker.json` stores the full 60-char title

**Layer 1 status: complete — all 18 Red Gate tests passing; manual testing complete; portfolio assessment gate interview deferred.**

**Red Gate — tests to write first:**

Integration tests (invoke binary as subprocess):
- `create_valid_title_exits_zero_and_prints_confirmation` — asserts exit 0, stdout = `Created issue #1: Fix bug\n` — fails against stub that exits 1 or prints nothing
- `create_stores_issue_in_json` — asserts `tracker.json` exists after create and contains `"title": "Fix bug"` — fails against stub that does nothing
- `create_empty_title_exits_one_with_error_on_stderr` — asserts exit 1, stderr contains `Error: Title cannot be empty.`, stdout is empty — fails against stub that exits 0
- `create_whitespace_title_exits_one` — asserts exit 1, stderr contains `Error: Title cannot be empty.` — fails against stub that stores whitespace
- `create_trims_title` — creates `"  Fix bug  "`, reads `tracker.json`, asserts stored title is `"Fix bug"` — fails against stub that stores verbatim
- `create_second_issue_gets_id_2` — two creates, asserts second confirmation says `#2` — fails against stub that always assigns `#1`
- `create_first_issue_unchanged_after_second_create` — reads `tracker.json` after two creates, asserts first issue fields are identical — fails against stub that overwrites
- `create_timestamps_equal_on_fresh_issue` — reads `tracker.json` after create, asserts `created_at == updated_at` — fails against stub that sets `updated_at` to epoch or a different value
- `list_truncates_title_at_50_chars_with_ellipsis` — creates issue with 60-char title, asserts list stdout contains the 50-char prefix followed by `…` — fails against stub that prints full title
- `list_with_no_json_shows_empty_state` — asserts stdout = `No open issues. Nice work!\n` — fails against stub that crashes or shows nothing
- `list_shows_header_and_issues` — asserts stdout contains `ID` header and issue title — fails against stub that prints nothing
- `list_after_create_shows_issue` — create then list, assert issue title appears in output — fails against stub that ignores storage
- `malformed_json_causes_error_exit` — writes `{bad}` to `tracker.json`, asserts exit 1, stderr contains `Could not read tracker data` — fails against stub that ignores parse errors

Unit tests:
- `title_empty_after_trim_is_rejected` — validates `""` and `"   "` return an error — fails against validator that always returns Ok
- `title_trimmed_before_storage` — validates `"  Fix bug  "` produces `"Fix bug"` — fails against identity function
- `id_assignment_first_issue_is_1` — empty issue list produces id=1 — fails against function returning 0
- `id_assignment_increments_from_max` — list with max id=5 produces id=6 — fails against function returning 1

**IAR:** SO, SA, QE, SE, Security, Platform, Data Engineer, VDD-IAR Alignment

---

## Layer 2: Status Flow

**Goal:** The user can change an issue's status, and the default list shows only open issues.

**Acceptance Criteria:**
- [x] `tracker status 1 in-progress` exits 0 and prints `Issue #1 status → in-progress.`
- [x] `tracker status 1 done` exits 0 and prints `Issue #1 status → done.`
- [x] After `tracker status 1 done`, `tracker.json` has the issue's `status` updated to `done` and `updated_at` refreshed; all other fields are unchanged
- [x] `updated_at` after a status change is `>=` `updated_at` before the change
- [x] `tracker list` (no flags) shows only `open` issues; issues with status `done` or `in-progress` do not appear
- [x] `tracker list --status done` shows `done` issues; `open` and `in-progress` do not appear
- [x] `tracker list --status in-progress` shows `in-progress` issues
- [x] `tracker list --status open` behaves identically to `tracker list` (explicit `open` flag matches default)
- [x] `tracker status 1 IN-PROGRESS` (uppercase) exits 0; stored value is lowercase `in-progress`
- [x] `tracker status 1 in-progress` when issue is already `in-progress` exits 0, prints confirmation, refreshes `updated_at`
- [x] `tracker status abc open` exits 1, stderr `Error: 'abc' is not a valid issue ID. Expected a positive integer.`
- [x] `tracker status 0 open` exits 1, stderr `Error: '0' is not a valid issue ID. Expected a positive integer.`
- [x] `tracker status 99 open` (ID not found) exits 1, stderr `Error: Issue #99 not found.`
- [x] `tracker status 1 flying` exits 1, stderr `Error: Invalid status 'flying'. Expected: open, in-progress, or done.`
- [x] `tracker list --status flying` exits 1, stderr `Error: Invalid status 'flying'. Expected: open, in-progress, or done.`
- [x] All issues done, `tracker list` (default) prints `No open issues. Nice work!`

**Not in this layer:** `--priority` filter, `--label` filter, compound filters

**Manual Testing Checklist:**
- [x] Happy path: create two issues → `tracker status 1 in-progress` → `tracker status 2 done` → `tracker list` shows only issue #1 (open) → `tracker list --status done` shows only issue #2 → `tracker list --status in-progress` shows only issue #1
- [x] Re-open: `tracker status 2 open` → `tracker list` shows both issues
- [x] Idempotent: `tracker status 1 in-progress` again → exit 0, confirmation printed, `updated_at` updated in `tracker.json`
- [x] Empty open state: mark all issues done → `tracker list` prints `No open issues. Nice work!`
- [x] Error — invalid ID: `tracker status abc done` → exit 1, stderr shows ID error
- [x] Error — zero ID: `tracker status 0 done` → exit 1, stderr shows ID error
- [x] Error — not found: `tracker status 99 done` → exit 1, stderr shows not-found error
- [x] Error — invalid status: `tracker status 1 completed` → exit 1, stderr shows valid values
- [x] Error — invalid list status: `tracker list --status closed` → exit 1
- [x] Persistence: change status, reinstall binary, run `tracker list --status in-progress` → change is intact

**Red Gate — tests to write first:**

Integration tests:
- `status_change_exits_zero_and_prints_confirmation` — asserts exit 0, stdout = `Issue #1 status → in-progress.\n` — fails against stub that exits 1
- `status_change_updates_json` — asserts `tracker.json` has `"status": "in-progress"` after command — fails against stub that doesn't write
- `status_change_refreshes_updated_at` — asserts `updated_at` in JSON changes after status command — fails against stub that leaves field unchanged
- `status_change_leaves_other_fields_unchanged` — asserts title, priority, labels, created_at identical after status change — fails against stub that overwrites whole object
- `status_is_case_insensitive_on_input` — `tracker status 1 DONE`, asserts stored value is `"done"` — fails against stub that stores `"DONE"`
- `status_idempotent_same_value_succeeds` — set status to current value, assert exit 0 and updated_at refreshed — fails against stub that rejects no-op
- `list_default_excludes_done_issues` — create issue, mark done, list → issue not in output — fails against stub that shows all
- `list_status_filter_shows_done` — `tracker list --status done` shows done issue, not open — fails against stub that ignores filter
- `list_status_filter_shows_in_progress` — mark issue in-progress, `tracker list --status in-progress` shows it and no others — fails against stub that only handles open/done
- `status_change_does_not_modify_created_at` — reads `created_at` before and after status change, asserts they are identical — fails against stub that updates `created_at` on mutation
- `status_invalid_id_string_exits_one` — `tracker status abc open`, asserts exit 1, stderr contains `not a valid issue ID` — fails against stub that exits 0
- `status_zero_id_exits_one` — `tracker status 0 open`, asserts exit 1 — fails against stub that accepts zero
- `status_not_found_exits_one` — `tracker status 99 open`, asserts exit 1, stderr contains `not found` — fails against stub that exits 0
- `status_invalid_value_exits_one` — asserts exit 1, stderr contains valid values — fails against stub that accepts anything
- `list_invalid_status_filter_exits_one` — asserts exit 1, stderr contains valid values

Unit tests:
- `status_value_parsing_valid_cases` — `open`, `in-progress`, `done` (and uppercase variants) parse correctly — fails against parser that returns Err for all
- `status_value_parsing_rejects_invalid` — `done.`, `in_progress`, `closed` return Err — fails against parser that returns Ok for all
- `id_must_be_positive_integer` — `0`, negative, non-integer string return Err — fails against parser that accepts all

**IAR:** SO, SA, QE, SE, Security, VDD-IAR Alignment

---

## Layer 3: Priority

**Goal:** The user can set issue priority, and the list is sorted by priority with filtering support.

**Acceptance Criteria:**
- [x] `tracker create "Fix bug" --priority high` stores `"priority": "high"`
- [x] `tracker create "Fix bug"` (no flag) stores `"priority": "medium"` (default unchanged from Layer 1)
- [x] `tracker create "Fix bug" --priority HIGH` (uppercase) stores `"priority": "high"`
- [x] `tracker create "Fix bug" --priority critical` exits 1, stderr `Error: Invalid priority 'critical'. Expected: low, medium, or high.`
- [x] `tracker list` output is sorted: all `high` issues before all `medium`, all `medium` before all `low`
- [x] Within the same priority tier, issues are sorted by ID ascending
- [x] `tracker list --priority high` shows only `high` issues; `medium` and `low` do not appear
- [x] `tracker list --priority medium` shows only `medium` issues
- [x] `tracker list --priority low` shows only `low` issues
- [x] `tracker list --priority invalid` exits 1, stderr `Error: Invalid priority 'invalid'. Expected: low, medium, or high.`
- [x] `tracker list --status open --priority high` shows only open high-priority issues (AND-combined; full compound-filter verification in Layer 5)

**Not in this layer:** `--label` on create, `--label` filter, compound filter with three flags

**Manual Testing Checklist:**
- [x] Happy path: create three issues with priorities `low`, `high`, `medium` in that order → `tracker list` shows `high` first (lowest id in high tier), then `medium`, then `low`
- [x] Same-priority tie-breaking: create two `high` issues → both appear before any `medium`, ordered by ID ascending
- [x] Filter: `tracker list --priority high` → only high-priority issues visible
- [x] Default priority: create issue without `--priority` → `tracker list` shows `medium` in Priority column
- [x] Uppercase input: `tracker create "Test" --priority HIGH` → stored as `high`
- [x] Error — invalid priority on create: `tracker create "Test" --priority urgent` → exit 1
- [x] Error — invalid priority on list: `tracker list --priority urgent` → exit 1
- [x] Persistence: create prioritized issues, reinstall binary, `tracker list` → sort order preserved

**Red Gate — tests to write first:**

Integration tests:
- `create_with_priority_stores_correct_value` — creates with `--priority high`, reads JSON, asserts `"priority": "high"` — fails against stub that ignores flag
- `create_without_priority_defaults_to_medium` — reads JSON, asserts `"priority": "medium"` — fails against stub that stores empty string
- `create_invalid_priority_exits_one` — asserts exit 1, stderr contains `Invalid priority` — fails against stub that accepts anything
- `list_sorts_high_before_medium_before_low` — creates low/high/medium in that order, asserts high appears before medium in list stdout — fails against stub that sorts by insertion order
- `list_within_tier_sorted_by_id_ascending` — two high issues, asserts lower ID appears first — fails against stub that reverses
- `list_priority_filter_shows_only_matching` — `--priority high`, asserts medium and low issues absent — fails against stub that ignores filter
- `list_invalid_priority_filter_exits_one` — asserts exit 1, stderr lists valid values

Unit tests:
- `priority_parsing_valid_cases` — `low`, `medium`, `high` (and uppercase) parse correctly
- `priority_parsing_rejects_invalid` — `critical`, `urgent`, empty string return Err
- `priority_sort_order_is_correct` — `[low, high, medium, high]` sorted to `[high, high, medium, low]` (same-priority order by ID)
- `priority_sort_tie_breaking_by_id` — two issues with same priority, lower ID comes first

**IAR:** SO, SA, QE, SE, VDD-IAR Alignment

---

## Layer 4: Labels

**Goal:** The user can add labels to issues, see them in the list, and filter by label.

**Acceptance Criteria:**
- [ ] `tracker create "Fix bug" --label bug` stores `"labels": ["bug"]`
- [ ] `tracker create "Fix bug" --label bug --label auth` stores `"labels": ["bug", "auth"]` (order preserved)
- [ ] `tracker create "Fix bug" --label bug --label bug` stores `"labels": ["bug"]` (deduplicated)
- [ ] `tracker create "Fix bug" --label ""` exits 1, stderr `Error: Label cannot be empty.`
- [ ] `tracker create "Fix bug" --label "  "` exits 1, stderr `Error: Label cannot be empty.`
- [ ] `tracker create "Fix bug"` stores `"labels": []`; list shows `(none)` in Labels column
- [ ] Labels column in list shows comma-separated labels; truncates at 20 characters with `…` if longer
- [ ] `tracker list --label bug` shows only issues that have `bug` in their labels list
- [ ] `tracker list --label Bug` does NOT show issues labeled `bug` (case-sensitive match)
- [ ] `tracker list --label bug` on an issue with no labels → issue does not appear
- [ ] `tracker list --label bug --label auth` exits 1, stderr contains usage error, exits 1 (multiple `--label` flags on list are rejected)

**Not in this layer:** compound filter with all three flags simultaneously (Layer 5)

**Manual Testing Checklist:**
- [ ] Happy path: create issue with `--label bug --label auth` → `tracker list` shows `bug, auth` in Labels column
- [ ] Deduplication: `--label bug --label bug` → Labels column shows `bug` once
- [ ] No labels: create without `--label` → Labels column shows `(none)`
- [ ] Filter: `tracker list --label bug` → only issues with `bug` label appear
- [ ] Case-sensitive: create issue labeled `bug`, run `tracker list --label Bug` → no results (prints `No issues match the given filters.`)
- [ ] Error — empty label: `tracker create "Test" --label ""` → exit 1
- [ ] Error — multiple label filters: `tracker list --label bug --label auth` → exit 1
- [ ] Long labels: create issue with `--label averylonglabelthatexceedsthecolumnwidth` → Labels column truncates with `…` at 20 chars
- [ ] Persistence: labels survive reinstall

**Red Gate — tests to write first:**

Integration tests:
- `create_with_label_stores_label` — reads JSON, asserts `"labels": ["bug"]` — fails against stub that ignores `--label`
- `create_with_multiple_labels_stores_all` — asserts both labels present in correct order — fails against stub that stores only last
- `create_with_duplicate_labels_deduplicates` — asserts stored once — fails against stub that stores duplicates
- `list_label_value_truncated_at_20_chars` — create with a 25-char label, `tracker list` shows label truncated at 20 chars with `…` — fails against stub that prints full label
- `create_with_empty_label_exits_one` — asserts exit 1, stderr `Label cannot be empty` — fails against stub that accepts empty
- `create_with_whitespace_label_exits_one` — same as above for whitespace-only
- `create_without_labels_stores_empty_array` — reads JSON, asserts `"labels": []` — fails against stub that omits field
- `list_shows_labels_comma_separated` — assert output contains `bug, auth` — fails against stub that shows nothing
- `list_shows_none_for_no_labels` — assert `(none)` in output for unlabeled issue — fails against stub that shows empty string
- `list_label_filter_shows_matching` — `--label bug`, assert labeled issue present, unlabeled absent — fails against stub that shows all
- `list_label_filter_is_case_sensitive` — `--label Bug` returns no results for issue labeled `bug` — fails against case-insensitive stub
- `list_multiple_label_flags_exits_one` — `--label bug --label auth`, assert exit 1 — fails against stub that accepts both

Unit tests:
- `label_deduplication_preserves_first_occurrence` — `["bug", "bug", "auth"]` → `["bug", "auth"]`
- `label_empty_after_trim_rejected` — `""` and `"  "` return Err
- `label_filter_case_sensitive_match` — issue with `["bug"]` does not match filter `"Bug"`

**IAR:** SO, SA, QE, SE, VDD-IAR Alignment

---

## Layer 5: Compound Filtering

**Goal:** Status, priority, and label filters AND-combine correctly; all no-match states are correct.

**Acceptance Criteria:**
- [ ] `tracker list --status open --priority high` shows only issues that are both open AND high-priority
- [ ] `tracker list --status open --label bug` shows only open issues with label `bug`
- [ ] `tracker list --priority high --label bug` shows only high-priority issues with label `bug`
- [ ] `tracker list --status open --priority high --label bug` shows only issues matching all three
- [ ] An issue that matches two of three filters but not the third does NOT appear
- [ ] `tracker list --status done --priority low` with no matching issues prints `No issues match the given filters.`
- [ ] `tracker list --status open --priority high --label nonexistent` with no matching issues prints `No issues match the given filters.`
- [ ] `tracker list` (default, all open, some exist) shows only open issues, not `No issues match` message

**Not in this layer:** any new commands or flags

**Manual Testing Checklist:**
- [ ] Setup: create four issues — `(open, high, bug)`, `(open, medium, bug)`, `(done, high, bug)`, `(open, high, feature)` — then run each filter combination and verify only the correct issue(s) appear
- [ ] Two-filter AND: `--status open --priority high` → issues #1 and #4 only
- [ ] Three-filter AND: `--status open --priority high --label bug` → issue #1 only
- [ ] No-match from filters: `--status open --priority low` → `No issues match the given filters.`
- [ ] Default view (open exists): `tracker list` → shows open issues, not the no-match message
- [ ] `No open issues. Nice work!` message: mark all issues done, `tracker list` → correct empty-state message (not the filter message)

**Red Gate — tests to write first:**

Integration tests:
- `list_two_filter_and_combination` — two filters, asserts issue matching both present, issue matching only one absent — fails against stub that ORs filters
- `list_three_filter_and_combination` — all three filters, asserts only exact match present — fails against stub that uses OR
- `list_no_match_shows_filter_message` — filters that exclude all issues, asserts `No issues match the given filters.` — fails against stub that shows empty output
- `list_default_does_not_show_filter_message` — open issues exist, no flags, asserts output does NOT contain `No issues match` — fails against stub that always shows filter message

Unit tests:
- `filter_and_logic_all_must_match` — issue matching 2/3 criteria returns false from filter function — fails against OR implementation
- `filter_and_logic_all_present_returns_true` — issue matching all criteria returns true

**IAR:** SO, SA, QE, SE, VDD-IAR Alignment

---

## Layer 6: Description + Show + Delete

**Goal:** The user can add a description when creating an issue, view full issue details, and delete issues.

**Acceptance Criteria:**
- [ ] `tracker create "Fix bug" --description "Auth token expires too soon"` stores description verbatim in `tracker.json`
- [ ] `tracker create "Fix bug" --description ""` exits 1, stderr `Error: Description cannot be empty.`
- [ ] `tracker create "Fix bug" --description "  "` exits 1, stderr `Error: Description cannot be empty.`
- [ ] `tracker create "Fix bug"` (no flag) stores no `description` field (absent, not null or empty string)
- [ ] `tracker show 1` exits 0, stdout shows all fields: ID, Title, Status, Priority, Labels (comma-separated or `(none)`), Description (verbatim or `(none)` if absent), Created, Updated
- [ ] `tracker show 1` label column is right-padded to 13 characters so values align
- [ ] `tracker show 1` with a multi-line description: first line follows `Description:` label; each continuation line is indented by 13 spaces
- [ ] `tracker show 1` displays full untruncated title and labels (no truncation, unlike list)
- [ ] `tracker show abc` exits 1, stderr `Error: 'abc' is not a valid issue ID. Expected a positive integer.`
- [ ] `tracker show 0` exits 1, stderr `Error: '0' is not a valid issue ID. Expected a positive integer.`
- [ ] `tracker show 99` (not found) exits 1, stderr `Error: Issue #99 not found.`
- [ ] `tracker delete 1` exits 0, prints `Deleted issue #1.`, and removes the issue from `tracker.json`
- [ ] After `tracker delete 1`, `tracker show 1` exits 1 with not-found error
- [ ] After deleting issue #1 and creating a new issue, the new issue gets ID #3 (or `max(remaining)+1`), never re-using `#1`
- [ ] `tracker delete abc` exits 1, stderr `Error: 'abc' is not a valid issue ID. Expected a positive integer.`
- [ ] `tracker delete 99` exits 1, stderr `Error: Issue #99 not found.`
- [ ] All other issues are unchanged after a delete
- [ ] Description is never shown in `tracker list` output

**Manual Testing Checklist:**
- [ ] Happy path: `tracker create "Fix auth" --description "Token expires after 1 hour"` → `tracker show 1` displays all fields correctly
- [ ] No description: create without `--description` → `tracker show` shows `Description: (none)`
- [ ] No labels: issue with no labels → `tracker show` shows `Labels:      (none)`
- [ ] Multi-line description: create with description containing a newline (`$'line1\nline2'` in shell) → `tracker show` indents continuation line by 13 spaces
- [ ] Show alignment: verify all value columns align at the same horizontal position (13-char label column)
- [ ] Show is non-mutating: `tracker show 1` twice produces identical output; `tracker.json` unchanged
- [ ] Delete: `tracker delete 2` → `Deleted issue #2.` → `tracker list` no longer shows #2 → `tracker show 2` → not-found error → `tracker.json` does not contain issue #2
- [ ] ID not reused: delete issue #2, create new issue → new ID is #3 (or higher, never #2)
- [ ] Other issues unchanged after delete: `tracker show 1` after `tracker delete 2` shows issue #1 intact
- [ ] Error — empty description: `tracker create "Test" --description ""` → exit 1
- [ ] Error — show invalid ID: `tracker show 0` and `tracker show abc` → exit 1
- [ ] Error — delete not found: `tracker delete 99` → exit 1
- [ ] Persistence: create with description, reinstall binary, `tracker show` → description intact

**Red Gate — tests to write first:**

Integration tests:
- `create_with_description_stores_verbatim` — reads JSON, asserts description field matches input exactly — fails against stub that ignores `--description`
- `create_with_empty_description_exits_one` — asserts exit 1, stderr `Description cannot be empty` — fails against stub that stores empty string
- `create_without_description_has_no_field_in_json` — reads JSON, asserts no `description` key — fails against stub that stores `null`
- `show_displays_all_fields` — asserts stdout contains ID, Title, Status, Priority, Labels, Description, Created, Updated — fails against stub that prints nothing
- `show_displays_none_for_absent_description` — asserts `Description: (none)` in output — fails against stub that prints empty
- `show_displays_none_for_no_labels` — asserts `Labels:      (none)` — fails against stub that prints empty
- `show_multiline_description_indents_continuation` — description with embedded newline, asserts second line starts with 13 spaces — fails against stub that prints raw newline
- `show_does_not_truncate_title_or_labels` — full title longer than 50 chars appears untruncated in show — fails against stub that applies list truncation
- `show_invalid_id_string_exits_one` — `tracker show abc`, asserts exit 1, stderr contains `not a valid issue ID` — fails against stub that exits 0
- `show_zero_id_exits_one` — `tracker show 0`, asserts exit 1
- `show_not_found_exits_one` — asserts exit 1, stderr contains `not found`
- `delete_removes_issue` — after `tracker delete 1`, reads JSON, asserts id=1 not present — fails against stub that doesn't write
- `delete_exits_zero_and_prints_confirmation` — asserts exit 0, stdout = `Deleted issue #1.\n` — fails against stub that exits 1
- `delete_id_not_reused` — delete issue #1, create new issue, read JSON, assert new issue has id=2 (not 1) — fails against stub that reassigns
- `delete_other_issues_unchanged` — delete #1, read JSON, assert issue #2 is present with original fields — fails against stub that clears all
- `delete_not_found_exits_one` — asserts exit 1, stderr contains `not found`
- `delete_invalid_id_exits_one` — asserts exit 1, stderr contains `not a valid issue ID`
- `description_not_in_list_output` — create with description, `tracker list`, assert description text absent from stdout — fails against stub that prints all fields

Unit tests:
- `multiline_description_show_format` — renders `"line1\nline2"` as first line after label, second line indented 13 spaces — fails against pass-through renderer
- `show_label_column_right_padded_to_13` — each label name is right-padded to 13 chars — fails against unpadded renderer
- `max_id_plus_one_skips_deleted_ids` — issue list `[{id:1}, {id:3}]`, max+1 = 4 — fails against sequential counter that produces 2

**IAR:** SO, SA, QE, SE, Security, Data Engineer, VDD-IAR Alignment

---

## Layer 7: Polish

**Goal:** `--help` works for all subcommands, color output is applied to TTY and suppressed when piped, all error messages are reviewed for specificity.

**Acceptance Criteria:**
- [ ] `tracker --help` exits 0 and describes all subcommands
- [ ] `tracker create --help` exits 0 and describes all flags (`--description`, `--priority`, `--label`) with valid values
- [ ] `tracker list --help` exits 0 and describes all flags (`--status`, `--priority`, `--label`) with valid values
- [ ] `tracker status --help` exits 0 and describes the positional arguments and valid status values
- [ ] `tracker show --help` exits 0 and describes the `<id>` argument
- [ ] `tracker delete --help` exits 0 and describes the `<id>` argument
- [ ] When stdout is a TTY: `high` priority value is displayed in red/bold, `medium` in yellow, `low` in default color
- [ ] When stdout is a TTY: `in-progress` status is displayed in cyan, `done` in green, `open` in default color
- [ ] When stdout is piped (`tracker list | cat`): no ANSI escape codes in output
- [ ] Color is applied only to the value text in the cell, not the entire row or header
- [ ] Color appears in both `tracker list` and `tracker show` output when TTY
- [ ] All error messages begin with `Error:` followed by a human-readable description (no stack traces or internal detail)
- [ ] An unknown subcommand (`tracker frobnicate`) exits 1 with a usage error on stderr

**Manual Testing Checklist:**
- [ ] Run `tracker --help` and each subcommand `--help` → verify flags and valid values are accurately described
- [ ] Run `tracker list` in terminal → verify `high` priority is red/bold, `in-progress` is cyan, `done` is green
- [ ] Run `tracker list | cat` → verify output contains no `\033[` escape sequences
- [ ] Run `tracker show <id>` in terminal with an `in-progress`/`high` issue → verify coloring in show output
- [ ] Run `tracker show <id>` piped → no ANSI codes
- [ ] Review each error message from all prior layers manually: does it say what went wrong and what the valid alternatives are?
- [ ] `tracker frobnicate` → exit 1, stderr usage error

**Red Gate — tests to write first:**

Integration tests:
- `help_flag_binary_exits_zero` — `tracker --help`, asserts exit 0 — fails against stub that exits 1
- `help_flag_create_exits_zero` — `tracker create --help`, asserts exit 0
- `help_flag_list_exits_zero` — `tracker list --help`, asserts exit 0
- `help_flag_status_exits_zero` — `tracker status --help`, asserts exit 0
- `help_flag_show_exits_zero` — `tracker show --help`, asserts exit 0
- `help_flag_delete_exits_zero` — `tracker delete --help`, asserts exit 0
- `list_piped_has_no_ansi_codes` — `tracker list | cat`, asserts stdout contains no `\x1b[` — fails against implementation that always applies color
- `unknown_subcommand_exits_one` — `tracker frobnicate`, asserts exit 1 — fails against stub that exits 0

Manual only (TTY-detection cannot be automated in subprocess tests):
- Color rendering in TTY verified manually per checklist above

**IAR:** SO, SA, QE, SE, UX, Platform, VDD-IAR Alignment

---

## Coverage Check

| DESIGN.md section | Covered in layer |
|---|---|
| Feature 1: Create (title, trim, empty error) | Layer 1 |
| Feature 1: `--priority` | Layer 3 |
| Feature 1: `--label` (repeatable, dedup, empty error) | Layer 4 |
| Feature 1: `--description` (empty error, verbatim storage) | Layer 6 |
| Feature 2: List (default open, tabular format, empty state) | Layer 1 |
| Feature 2: `--status` filter | Layer 2 |
| Feature 2: `--priority` filter | Layer 3 |
| Feature 2: `--label` filter (single, case-sensitive, multiple=error) | Layer 4 |
| Feature 2: AND-combined filters, no-match message | Layer 5 |
| Feature 3: Change Status (transitions, idempotent, updated_at) | Layer 2 |
| Feature 4: Show (all fields, format, alignment, no-truncation) | Layer 6 |
| Feature 5: Delete (removal, ID not reused, non-mutating to others) | Layer 6 |
| Data Model: id, title, status, priority, labels, created_at, updated_at | Layer 1 |
| Data Model: description (optional) | Layer 6 |
| Storage: missing JSON = empty, malformed JSON = error | Layer 1 |
| Storage: unknown fields ignored | Layer 1 (deserialization) |
| Interface: color output (TTY detect, values only) | Layer 7 |
| Interface: `--help` for all subcommands | Layer 7 |
| Edge Cases: title (empty, whitespace, trim, special chars) | Layer 1 |
| Edge Cases: IDs (non-integer, zero, deleted, not-found) | Layer 2 (status), Layer 6 (show/delete) |
| Edge Cases: labels (duplicate, empty, whitespace, case-sensitive filter, multiple list flag) | Layer 4 |
| Edge Cases: list (no issues, filters no-match, all done) | Layer 1 + 2 + 5 |
| Edge Cases: status transitions (any to any, idempotent) | Layer 2 |
| Edge Cases: storage (missing, malformed, unknown fields, permission error, write failure) | Layer 1 + 6 |
| Edge Cases: description (empty, not trimmed, multi-line) | Layer 6 |
| Constraints: single user, no network, local storage, Rust only, crash-safe I/O, input validation | Across all layers |
