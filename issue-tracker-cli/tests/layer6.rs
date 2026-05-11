use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

mod common;
use common::tracker;

// Layer 6 — Description + Show + Delete
//
// DESIGN.md Feature 1 (--description), Feature 4 (Show), Feature 5 (Delete).
// Layer 6 adds:
// - `--description` flag on `create`: stored verbatim (not trimmed) when
//   provided; empty/whitespace-only after trim → error; absent → JSON key
//   omitted (not `null`).
// - `tracker show <id>`: prints the labelled key-value block per DESIGN.md
//   "Show output format" (13-char label column; multi-line description
//   indented by 13 spaces; full untruncated title and labels).
// - `tracker delete <id>`: removes the issue from storage; the deleted ID is
//   never reused (`next_id` returns `max(remaining) + 1`).

// --- create with --description ---

#[test]
fn create_with_description_stores_verbatim() {
    // DESIGN.md Feature 1 postcondition: "description is stored as provided
    // (not trimmed); absent if --description is not provided".
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args([
            "create",
            "Fix bug",
            "--description",
            "Auth token expires too soon",
        ])
        .assert()
        .success();

    let raw = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
    let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(v["issues"][0]["description"], "Auth token expires too soon");
}

#[test]
fn create_with_empty_description_exits_one() {
    // DESIGN.md Feature 1 error state: "--description value is empty or
    // whitespace-only after trim → Error: Description cannot be empty."
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Fix bug", "--description", ""])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains(
            "Error: Description cannot be empty.",
        ))
        .stdout("");
}

#[test]
fn create_with_whitespace_description_exits_one() {
    // AC: empty-after-trim is the rejection rule. Twin test of empty case;
    // matches Layer 1's title and Layer 4's label whitespace coverage.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Fix bug", "--description", "   "])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains(
            "Error: Description cannot be empty.",
        ))
        .stdout("");
}

#[test]
fn create_without_description_has_no_field_in_json() {
    // DESIGN.md Data Model: `description: Option<String>` is "absent if not
    // provided. Absent means the JSON key is omitted, not serialized as
    // null. Implementations must omit the key when the value is None."
    // (Cat B Red Gate deviation — Layer 1 already serializes None as absent
    // via #[serde(skip_serializing_if = "Option::is_none")]; this test pins
    // that contract for Layer 6.)
    let dir = TempDir::new().unwrap();
    tracker(&dir).args(["create", "Fix bug"]).assert().success();

    let raw = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
    let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let obj = v["issues"][0]
        .as_object()
        .expect("issue must be a JSON object");
    assert!(
        !obj.contains_key("description"),
        "description key must be omitted (not null/empty) when --description is absent:\n{obj:#?}"
    );
}

// --- show: happy path + rendering ---

#[test]
fn show_displays_all_fields() {
    // DESIGN.md Feature 4 postcondition: stdout shows all fields — ID, Title,
    // Status, Priority, Labels, Description, Created, Updated.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args([
            "create",
            "Fix auth",
            "--description",
            "Token refresh fails",
            "--priority",
            "high",
            "--label",
            "bug",
        ])
        .assert()
        .success();

    let output = tracker(&dir)
        .args(["show", "1"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let out = String::from_utf8(output).unwrap();
    for label in &[
        "ID:",
        "Title:",
        "Status:",
        "Priority:",
        "Labels:",
        "Description:",
        "Created:",
        "Updated:",
    ] {
        assert!(
            out.contains(label),
            "show output must include `{label}` row:\n{out}"
        );
    }
    assert!(
        out.contains("Fix auth"),
        "show must include the title value:\n{out}"
    );
    assert!(
        out.contains("Token refresh fails"),
        "show must include the description value:\n{out}"
    );
    assert!(
        out.contains("high"),
        "show must include the priority value:\n{out}"
    );
    assert!(
        out.contains("bug"),
        "show must include the label value:\n{out}"
    );
}

#[test]
fn show_displays_none_for_absent_description() {
    // DESIGN.md Edge Cases / Description: "--description not provided →
    // description is absent; not shown in list, shown as (none) in show".
    let dir = TempDir::new().unwrap();
    tracker(&dir).args(["create", "No desc"]).assert().success();

    tracker(&dir)
        .args(["show", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Description: (none)"));
}

#[test]
fn show_displays_none_for_no_labels() {
    // DESIGN.md "Show output format" example: `Labels:      (none)` when
    // the issue has no labels. The label column is right-padded to 13 chars
    // so `Labels:` (7 chars) + 6 spaces precedes `(none)`.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "No labels"])
        .assert()
        .success();

    tracker(&dir)
        .args(["show", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Labels:      (none)"));
}

#[test]
fn show_multiline_description_indents_continuation() {
    // DESIGN.md "Show output format": "for multi-line descriptions, the
    // first line follows the Description: label; each continuation line is
    // indented by 13 spaces (matching the label-column width) so the text
    // block remains visually aligned."
    let dir = TempDir::new().unwrap();
    // Use a literal newline in the description value. The shell-quoting
    // analogue would be `$'line1\nline2'`; passed directly here via Rust.
    tracker(&dir)
        .args(["create", "Multi-line", "--description", "line1\nline2"])
        .assert()
        .success();

    let output = tracker(&dir)
        .args(["show", "1"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let out = String::from_utf8(output).unwrap();
    assert!(
        out.contains("Description: line1"),
        "first description line must follow the Description: label:\n{out:?}"
    );
    assert!(
        out.contains("\n             line2"),
        "continuation line must be indented by 13 spaces:\n{out:?}"
    );
}

#[test]
fn show_does_not_truncate_title_or_labels() {
    // DESIGN.md Interface section: "show always displays the full,
    // untruncated values" (whereas list truncates titles at 50 and the
    // Labels column at 20 chars with `…`).
    let dir = TempDir::new().unwrap();
    let long_title = "x".repeat(60); // > 50 chars (list truncates)
    let long_label = "y".repeat(25); // > 20 chars (list truncates)
    tracker(&dir)
        .args(["create", &long_title, "--label", &long_label])
        .assert()
        .success();

    let output = tracker(&dir)
        .args(["show", "1"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let out = String::from_utf8(output).unwrap();
    assert!(
        out.contains(&long_title),
        "show must contain the full 60-char title untruncated:\n{out}"
    );
    assert!(
        out.contains(&long_label),
        "show must contain the full 25-char label untruncated:\n{out}"
    );
    assert!(
        !out.contains('\u{2026}'),
        "show output must not contain a `…` ellipsis (no truncation):\n{out}"
    );
}

// --- show: error states ---

#[test]
fn show_invalid_id_string_exits_one() {
    // DESIGN.md Feature 4 error state: "<id> is not a positive integer →
    // 'abc' is not a valid issue ID. Expected a positive integer."
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["show", "abc"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains(
            "Error: 'abc' is not a valid issue ID. Expected a positive integer.",
        ))
        .stdout("");
}

#[test]
fn show_zero_id_exits_one() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["show", "0"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains(
            "Error: '0' is not a valid issue ID. Expected a positive integer.",
        ))
        .stdout("");
}

#[test]
fn show_not_found_exits_one() {
    // DESIGN.md Feature 4 error state: "Issue not found → Issue #<id> not found."
    let dir = TempDir::new().unwrap();
    tracker(&dir).args(["create", "Real"]).assert().success();

    tracker(&dir)
        .args(["show", "99"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Error: Issue #99 not found."))
        .stdout("");
}

// --- delete: happy path ---

#[test]
fn delete_exits_zero_and_prints_confirmation() {
    // DESIGN.md Feature 5 postcondition: "stdout prints: Deleted issue #<id>."
    let dir = TempDir::new().unwrap();
    tracker(&dir).args(["create", "Doomed"]).assert().success();

    tracker(&dir)
        .args(["delete", "1"])
        .assert()
        .success()
        .stdout(predicate::eq("Deleted issue #1.\n"));
}

#[test]
fn delete_removes_issue() {
    // DESIGN.md Feature 5 postcondition: "the issue is removed from tracker.json".
    let dir = TempDir::new().unwrap();
    tracker(&dir).args(["create", "Doomed"]).assert().success();
    tracker(&dir).args(["delete", "1"]).assert().success();

    let raw = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
    let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let arr = v["issues"]
        .as_array()
        .expect("tracker.json must have an issues array");
    assert!(
        arr.iter().all(|i| i["id"] != 1),
        "deleted issue id=1 must not be present in tracker.json:\n{arr:#?}"
    );
}

#[test]
fn delete_then_show_returns_not_found() {
    // DESIGN.md Feature 5 + Edge Cases: "ID of a deleted issue → error:
    // Issue #3 not found." Pins the show/delete composition.
    let dir = TempDir::new().unwrap();
    tracker(&dir).args(["create", "Doomed"]).assert().success();
    tracker(&dir).args(["delete", "1"]).assert().success();

    tracker(&dir)
        .args(["show", "1"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Error: Issue #1 not found."))
        .stdout("");
}

#[test]
fn delete_id_not_reused_middle_gap() {
    // DESIGN.md Feature 5 invariant: "the deleted ID is never reused".
    // Middle-gap subcase: after deleting issue #1 (the lowest of [#1, #2]), a
    // new create must NOT reassign id=1. With the persistent `next_id` counter
    // (SO R22 Option A), `next_id` was bumped past 2 at the previous create,
    // so the next create gets 3 even though the middle id (1) is free.
    let dir = TempDir::new().unwrap();
    tracker(&dir).args(["create", "First"]).assert().success();
    tracker(&dir).args(["create", "Second"]).assert().success();
    tracker(&dir).args(["delete", "1"]).assert().success();
    tracker(&dir).args(["create", "Third"]).assert().success();

    let raw = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
    let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let arr = v["issues"]
        .as_array()
        .expect("tracker.json must have an issues array");
    let ids: Vec<u64> = arr.iter().map(|i| i["id"].as_u64().unwrap()).collect();
    assert!(
        !ids.contains(&1),
        "deleted id=1 must not have been re-issued:\nids={ids:?}"
    );
    assert!(
        ids.contains(&3),
        "new issue must receive id=3 (counter has been bumped past 2):\nids={ids:?}"
    );
}

#[test]
fn delete_id_not_reused_high_edge() {
    // SO Review 22 regression test: the high-edge case the pre-R22
    // `max(remaining_ids) + 1` implementation silently violated. Director
    // manual-test reproduction: create #1, create #2 (next_id bumps to 3),
    // delete #2 (the highest id), create — the new id must be 3, NOT 2.
    // Pre-R22 this assigned 2 because `max([1]) + 1 == 2`, reusing the
    // just-deleted id. The persistent `next_id` counter closes the hole.
    let dir = TempDir::new().unwrap();
    tracker(&dir).args(["create", "First"]).assert().success();
    tracker(&dir).args(["create", "Second"]).assert().success();
    tracker(&dir).args(["delete", "2"]).assert().success();
    tracker(&dir)
        .args(["create", "Third"])
        .assert()
        .success()
        .stdout("Created issue #3: Third\n");

    let raw = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
    let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let arr = v["issues"]
        .as_array()
        .expect("tracker.json must have an issues array");
    let ids: Vec<u64> = arr.iter().map(|i| i["id"].as_u64().unwrap()).collect();
    assert_eq!(
        ids,
        vec![1, 3],
        "after deleting the highest id, the new id must skip the deleted value:\nids={ids:?}"
    );
    let next_id = v["next_id"]
        .as_u64()
        .expect("next_id must be present after Option A");
    assert_eq!(
        next_id, 4,
        "counter must have advanced to 4 after the third create"
    );
}

#[test]
fn delete_other_issues_unchanged() {
    // DESIGN.md Feature 5 invariant: "no other issues are affected by the
    // delete". After deleting issue #1, issue #2's fields are identical to
    // pre-delete state.
    let dir = TempDir::new().unwrap();
    tracker(&dir).args(["create", "First"]).assert().success();
    tracker(&dir)
        .args(["create", "Second", "--priority", "high", "--label", "bug"])
        .assert()
        .success();

    let pre = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
    let pre_v: serde_json::Value = serde_json::from_str(&pre).unwrap();
    let pre_second = pre_v["issues"][1].clone();

    tracker(&dir).args(["delete", "1"]).assert().success();

    let post = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
    let post_v: serde_json::Value = serde_json::from_str(&post).unwrap();
    let post_second = post_v["issues"]
        .as_array()
        .unwrap()
        .iter()
        .find(|i| i["id"] == 2)
        .expect("issue #2 must survive the delete")
        .clone();
    assert_eq!(
        pre_second, post_second,
        "issue #2 must be byte-identical before and after deleting #1"
    );
}

// --- delete: error states ---

#[test]
fn delete_invalid_id_exits_one() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["delete", "abc"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains(
            "Error: 'abc' is not a valid issue ID. Expected a positive integer.",
        ))
        .stdout("");
}

#[test]
fn delete_not_found_exits_one() {
    let dir = TempDir::new().unwrap();
    tracker(&dir).args(["create", "Real"]).assert().success();

    tracker(&dir)
        .args(["delete", "99"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Error: Issue #99 not found."))
        .stdout("");
}

// --- list does not leak description ---

// --- Round 2: description control-character defense (Security R9 F1 / RT R8 F1 / SE R15 F1 / DE R9 F1 / QE R15 F2 / SO R20 F3) ---

#[test]
fn create_with_control_char_description_exits_one() {
    // DESIGN.md Feature 1 / Edge Cases / Description (Round 2 amendment):
    // description rejects every control character (Cc) except newline. ESC
    // is the canonical terminal-escape injection byte and the same defect
    // class that motivated the title (Layer 1) and label (Layer 4) defenses.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args([
            "create",
            "Real",
            "--description",
            "Auth\u{1B}[31mPWN\u{1B}[0m",
        ])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains(
            "Error: Description cannot contain control characters other than newline.",
        ))
        .stdout("");
}

#[test]
fn create_with_carriage_return_description_exits_one() {
    // Bare \r overprints the rendered line column 0 in show output — SE R15 F2
    // / DE R9 F2. Subsumed by the broader Cc rejection rule with the \n
    // carve-out: \r is Cc and is NOT \n, so reject.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Real", "--description", "line1\rOVERWRITE"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains(
            "Error: Description cannot contain control characters other than newline.",
        ))
        .stdout("");
}

#[test]
fn create_with_crlf_description_exits_one() {
    // \r\n contains \r which is Cc-not-\n. Reject at create time per the
    // Round-2 spec amendment. (The format_show_block CRLF normalization is
    // defense-in-depth for legacy stored data / hand-edited files, NOT a
    // sanction for accepting CRLF at create time.)
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Real", "--description", "line1\r\nline2"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains(
            "Error: Description cannot contain control characters other than newline.",
        ))
        .stdout("");
}

#[test]
fn create_with_tab_description_exits_one() {
    // Tab is Cc and is NOT \n. Reject. Same rule as title (Layer 1) and
    // label (Layer 4) tab rejection.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Real", "--description", "a\tb"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains(
            "Error: Description cannot contain control characters other than newline.",
        ))
        .stdout("");
}

#[test]
fn create_with_del_description_exits_one() {
    // DEL (U+007F) is Cc and not \n. Reject. (NUL — U+0000 — cannot be passed
    // through argv because the OS forbids it at process-spawn time; that path
    // is covered by the unit test `description_with_control_char_other_than_
    // newline_is_rejected` in src/lib.rs which calls validate_description in-
    // process.)
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Real", "--description", "a\u{7F}b"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains(
            "Error: Description cannot contain control characters other than newline.",
        ))
        .stdout("");
}

#[test]
fn create_with_osc8_hyperlink_description_exits_one() {
    // OSC 8 hyperlink leader: ESC ] 8 ; ; URL BEL. Both ESC (0x1B) and BEL
    // (0x07) are Cc and not \n. Reject. Same rule as label OSC 8 rejection
    // (Layer 4 RT R6 F1).
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args([
            "create",
            "Real",
            "--description",
            "\u{1B}]8;;https://evil/\u{07}click\u{1B}]8;;\u{07}",
        ])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains(
            "Error: Description cannot contain control characters other than newline.",
        ))
        .stdout("");
}

#[test]
fn create_with_newline_description_is_accepted() {
    // \n is the spec-permitted carve-out: multi-line descriptions render in
    // show with the continuation lines indented 13 spaces. Verify acceptance.
    // Pinning the carve-out at the input boundary kills any mutation that
    // tightens the predicate to reject all Cc indiscriminately.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Multi-line", "--description", "line1\nline2"])
        .assert()
        .success();

    let raw = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
    let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(v["issues"][0]["description"], "line1\nline2");
}

#[test]
fn create_preserves_description_verbatim_with_surrounding_whitespace() {
    // QE R15 F3: existing create_with_description_stores_verbatim does not
    // pin the "stored as provided, not trimmed" half of the postcondition
    // because its test value has no surrounding whitespace. A mutation
    // `Ok(raw.trim().to_string())` in validate_description would survive
    // the original test. This test pins the verbatim-with-whitespace
    // contract.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "X", "--description", "  padded  "])
        .assert()
        .success();

    let raw = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
    let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(
        v["issues"][0]["description"], "  padded  ",
        "description must be stored verbatim including leading/trailing whitespace (not trimmed)"
    );
}

// --- Round 2: load-time corruption rejection for description (DE R9 F1 / Security R9 F1 load-path corollary) ---

#[test]
fn corrupt_data_with_control_char_description_is_rejected() {
    // Hand-edited tracker.json with a JSON-escaped ESC in description must
    // be rejected as corrupt at load. issue_fields_are_valid enforces the
    // same Cc-except-\n rule on stored data that validate_description
    // enforces on input. Same load-path corollary pattern as Layer 4
    // corrupt_data_with_control_char_label_is_rejected.
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("tracker.json");
    fs::write(
        &path,
        r#"{"issues":[{"id":1,"title":"Real","description":"a[31mPWN","status":"open","priority":"medium","labels":[],"created_at":"2026-01-01T00:00:00Z","updated_at":"2026-01-01T00:00:00Z"}],"next_id":2}"#,
    )
    .unwrap();

    tracker(&dir)
        .args(["list"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains(
            "Could not read tracker data. The file may be corrupt.",
        ))
        .stdout("");
}

#[test]
fn corrupt_data_with_carriage_return_description_is_rejected() {
    // Load-path corollary for bare \r — overprints show alignment column;
    // rejected at load.
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("tracker.json");
    fs::write(
        &path,
        r#"{"issues":[{"id":1,"title":"Real","description":"line1\rOVER","status":"open","priority":"medium","labels":[],"created_at":"2026-01-01T00:00:00Z","updated_at":"2026-01-01T00:00:00Z"}],"next_id":2}"#,
    )
    .unwrap();

    tracker(&dir)
        .args(["list"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains(
            "Could not read tracker data. The file may be corrupt.",
        ))
        .stdout("");
}

#[test]
fn load_accepts_description_with_newline() {
    // Load-path corollary of the create-time \n carve-out: a stored
    // description with a literal newline must NOT be rejected as corrupt.
    // Pins the carve-out across the boundary; kills a load-time predicate
    // that rejects all Cc indiscriminately.
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("tracker.json");
    fs::write(
        &path,
        r#"{"issues":[{"id":1,"title":"Real","description":"line1\nline2","status":"open","priority":"medium","labels":[],"created_at":"2026-01-01T00:00:00Z","updated_at":"2026-01-01T00:00:00Z"}],"next_id":2}"#,
    )
    .unwrap();

    tracker(&dir).args(["show", "1"]).assert().success();
}

// --- Round 2: show rendering strictness (QE R15 F1 over-padding mutation) ---

#[test]
fn show_renders_exact_full_block_for_single_line_issue() {
    // QE R15 F1: substring contains assertions in show_displays_all_fields
    // do not catch over-padding mutations (e.g., "ID:          " → "ID:           ").
    // Pin one full single-line show block exactly to lock the rendering
    // contract. The created_at / updated_at timestamps vary by clock so we
    // assert on the deterministic prefix lines only via a single full-line
    // comparison per field.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Update README", "--priority", "low"])
        .assert()
        .success();

    let output = tracker(&dir)
        .args(["show", "1"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let out = String::from_utf8(output).unwrap();
    // Each row must match EXACTLY (full-line match, not substring). The label
    // column is right-padded to 13 characters; any over-padding mutation
    // (an extra space) fails the equality check.
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines[0], "ID:          1");
    assert_eq!(lines[1], "Title:       Update README");
    assert_eq!(lines[2], "Status:      open");
    assert_eq!(lines[3], "Priority:    low");
    assert_eq!(lines[4], "Labels:      (none)");
    assert_eq!(lines[5], "Description: (none)");
    // Lines 6 + 7 are Created: / Updated: with clock-dependent timestamps;
    // assert just the label column prefix shape on those.
    assert!(
        lines[6].starts_with("Created:     "),
        "Created: row must start with 13-char label column:\n{lines:?}"
    );
    assert!(
        lines[7].starts_with("Updated:     "),
        "Updated: row must start with 13-char label column:\n{lines:?}"
    );
    // No 9th line (no trailing blank line beyond the final \n that print! consumed).
    assert_eq!(
        lines.len(),
        8,
        "show output must be exactly 8 lines:\n{out}"
    );
}

#[test]
fn description_not_in_list_output() {
    // DESIGN.md Edge Cases / Description: "in list output, description is
    // never shown" — even when stored.
    let dir = TempDir::new().unwrap();
    let unique_marker = "DESCRIPTION_SHOULD_NOT_LEAK_INTO_LIST";
    tracker(&dir)
        .args(["create", "Issue", "--description", unique_marker])
        .assert()
        .success();

    let output = tracker(&dir)
        .args(["list"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let out = String::from_utf8(output).unwrap();
    assert!(
        !out.contains(unique_marker),
        "list output must not contain the description text:\n{out}"
    );
}
