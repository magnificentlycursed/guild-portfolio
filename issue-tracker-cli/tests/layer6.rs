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
    assert_eq!(v[0]["description"], "Auth token expires too soon");
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
    let obj = v[0].as_object().expect("issue must be a JSON object");
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
    let arr = v.as_array().expect("tracker.json must be a JSON array");
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
fn delete_id_not_reused() {
    // DESIGN.md Feature 5 invariant: "the deleted ID is never reused; the
    // next created issue receives max(remaining_ids) + 1". After deleting
    // issue #1, a new create must get id=2 (max(remaining=[2]) + 1) — the
    // next_id assignment must skip the deleted gap, not refill it.
    let dir = TempDir::new().unwrap();
    tracker(&dir).args(["create", "First"]).assert().success();
    tracker(&dir).args(["create", "Second"]).assert().success();
    tracker(&dir).args(["delete", "1"]).assert().success();
    tracker(&dir).args(["create", "Third"]).assert().success();

    let raw = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
    let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let arr = v.as_array().expect("tracker.json must be a JSON array");
    let ids: Vec<u64> = arr.iter().map(|i| i["id"].as_u64().unwrap()).collect();
    assert!(
        !ids.contains(&1),
        "deleted id=1 must not have been re-issued:\nids={ids:?}"
    );
    assert!(
        ids.contains(&3),
        "new issue must receive id=3 (max(remaining=[2]) + 1):\nids={ids:?}"
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
    let pre_second = pre_v[1].clone();

    tracker(&dir).args(["delete", "1"]).assert().success();

    let post = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
    let post_v: serde_json::Value = serde_json::from_str(&post).unwrap();
    let post_second = post_v
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
