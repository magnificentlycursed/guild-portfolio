use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

mod common;
use common::tracker;

// --- create with --label ---

#[test]
fn create_with_label_stores_label() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Fix bug", "--label", "bug"])
        .assert()
        .success();

    let raw = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
    let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(v[0]["labels"], serde_json::json!(["bug"]));
}

#[test]
fn create_with_multiple_labels_stores_all() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Fix bug", "--label", "bug", "--label", "auth"])
        .assert()
        .success();

    let raw = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
    let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(
        v[0]["labels"],
        serde_json::json!(["bug", "auth"]),
        "labels must preserve insertion order"
    );
}

#[test]
fn create_with_duplicate_labels_deduplicates() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args([
            "create", "Fix bug", "--label", "bug", "--label", "bug", "--label", "auth",
        ])
        .assert()
        .success();

    let raw = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
    let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(
        v[0]["labels"],
        serde_json::json!(["bug", "auth"]),
        "duplicates must be collapsed; first occurrence wins"
    );
}

#[test]
fn create_with_empty_label_exits_one() {
    // DESIGN.md Feature 1: "Error: Label cannot be empty."
    // Literal spec assertion — substring `Label cannot be empty` is the spec phrase.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Fix bug", "--label", ""])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Error: Label cannot be empty."))
        .stdout("");
}

#[test]
fn create_with_whitespace_label_exits_one() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Fix bug", "--label", "   "])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Error: Label cannot be empty."))
        .stdout("");
}

#[test]
fn create_without_labels_stores_empty_array() {
    // Cat B Red Gate deviation — passes against the existing Layer 1 default
    // (`labels: Vec::new()`); regression coverage of the AC "default unchanged
    // from prior layers," not a Red Gate test for new behavior. Same disposition
    // as `create_without_priority_defaults_to_medium` in Layer 3.
    let dir = TempDir::new().unwrap();
    tracker(&dir).args(["create", "Fix bug"]).assert().success();

    let raw = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
    let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(v[0]["labels"], serde_json::json!([]));
}

// --- list: label display ---

#[test]
fn list_shows_labels_comma_separated() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Fix bug", "--label", "bug", "--label", "auth"])
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
        out.contains("bug, auth"),
        "Labels column must render comma-separated:\n{out}"
    );
}

#[test]
fn list_shows_none_for_no_labels() {
    // Cat B Red Gate deviation — `(none)` rendering already covered by Layer 1
    // tests/layer1.rs::list_shows_header_and_issues. Re-asserted at Layer 4 for
    // explicit acceptance-criterion coverage of the `--label` interaction with
    // unlabeled issues.
    let dir = TempDir::new().unwrap();
    tracker(&dir).args(["create", "Fix bug"]).assert().success();

    let output = tracker(&dir)
        .args(["list"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let out = String::from_utf8(output).unwrap();
    assert!(
        out.contains("(none)"),
        "unlabeled issue should show '(none)' in Labels column:\n{out}"
    );
}

#[test]
fn list_label_value_truncated_at_20_chars() {
    let dir = TempDir::new().unwrap();
    // 25-char label exceeds the 20-char Labels column.
    tracker(&dir)
        .args(["create", "Fix bug", "--label", "averylonglabelxxxxxxxxxxx"])
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
    // Truncation: 19 content chars + ellipsis = 20-char display, matching Title's
    // `chars[..max_chars - 1]` + `…` rule in `truncate_with_ellipsis`.
    let expected = format!("{}…", "averylonglabelxxxxx"); // first 19 chars
    assert!(
        out.contains(&expected),
        "label > 20 chars must truncate at 19 chars + … :\n{out}"
    );
    let not_expected_full = "averylonglabelxxxxxxxxxxx";
    assert!(
        !out.contains(not_expected_full),
        "full untruncated label must not appear in list output:\n{out}"
    );
}

// --- list: --label filter ---

#[test]
fn list_label_filter_shows_matching() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Login issue", "--label", "bug"])
        .assert()
        .success();
    tracker(&dir)
        .args(["create", "Search feature", "--label", "feature"])
        .assert()
        .success();
    tracker(&dir)
        .args(["create", "No-label item"])
        .assert()
        .success();

    let output = tracker(&dir)
        .args(["list", "--label", "bug"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let out = String::from_utf8(output).unwrap();
    assert!(
        out.contains("Login issue"),
        "labeled-bug issue should appear under --label bug:\n{out}"
    );
    assert!(
        !out.contains("Search feature"),
        "differently-labeled issue should NOT appear:\n{out}"
    );
    assert!(
        !out.contains("No-label item"),
        "unlabeled issue should NOT appear:\n{out}"
    );
}

#[test]
fn list_label_filter_is_case_sensitive() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Login issue", "--label", "bug"])
        .assert()
        .success();

    // DESIGN.md Edge Cases / Labels: `--label Bug` does NOT match an issue
    // with label `bug`. Spec contract is exact, case-sensitive equality.
    tracker(&dir)
        .args(["list", "--label", "Bug"])
        .assert()
        .success()
        .stdout("")
        .stderr(predicate::str::contains(
            "No issues match the given filters.",
        ));
}

#[test]
fn list_multiple_label_flags_exits_one() {
    // DESIGN.md Feature 2 / Edge Cases / Labels (additional): multiple `--label`
    // flags on `tracker list` are rejected as a usage error. (Multiple `--label`
    // flags on `tracker create` ARE accepted and deduplicated — see
    // `create_with_duplicate_labels_deduplicates` and
    // `create_with_multiple_labels_stores_all`.)
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Login issue", "--label", "bug"])
        .assert()
        .success();

    tracker(&dir)
        .args(["list", "--label", "bug", "--label", "auth"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Error:"))
        .stdout("");
}
