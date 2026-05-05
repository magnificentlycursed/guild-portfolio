use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

mod common;
use common::tracker;

// --- create with --priority ---

#[test]
fn create_with_priority_stores_correct_value() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Fix bug", "--priority", "high"])
        .assert()
        .success();

    let raw = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
    let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(v[0]["priority"], "high");
}

#[test]
fn create_without_priority_defaults_to_medium() {
    let dir = TempDir::new().unwrap();
    tracker(&dir).args(["create", "Fix bug"]).assert().success();

    let raw = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
    let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(v[0]["priority"], "medium");
}

#[test]
fn create_invalid_priority_exits_one() {
    // DESIGN.md Feature 1: "Error: Invalid priority '<v>'. Expected: low, medium, or high."
    // Literal spec assertion — substring `Invalid priority` could be satisfied by a regression
    // that omitted the offending value or the actionable expected-list suffix.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Fix bug", "--priority", "critical"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains(
            "Error: Invalid priority 'critical'. Expected: low, medium, or high.",
        ))
        .stdout("");
}

// --- list: priority sort ---

#[test]
fn list_sorts_high_before_medium_before_low() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Low item", "--priority", "low"])
        .assert()
        .success();
    tracker(&dir)
        .args(["create", "High item", "--priority", "high"])
        .assert()
        .success();
    tracker(&dir)
        .args(["create", "Medium item", "--priority", "medium"])
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
    let high_pos = out
        .find("High item")
        .expect("high item should appear in list");
    let medium_pos = out
        .find("Medium item")
        .expect("medium item should appear in list");
    let low_pos = out
        .find("Low item")
        .expect("low item should appear in list");
    assert!(
        high_pos < medium_pos,
        "high should appear before medium in list output"
    );
    assert!(
        medium_pos < low_pos,
        "medium should appear before low in list output"
    );
}

#[test]
fn list_within_tier_sorted_by_id_ascending() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "First high", "--priority", "high"])
        .assert()
        .success();
    tracker(&dir)
        .args(["create", "Second high", "--priority", "high"])
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
    let first = out
        .find("First high")
        .expect("first high item should appear");
    let second = out
        .find("Second high")
        .expect("second high item should appear");
    assert!(
        first < second,
        "lower ID (#1) should appear before higher ID (#2) within the same priority tier"
    );
}

// --- list: priority filter ---

#[test]
fn list_priority_filter_shows_only_matching() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Low item", "--priority", "low"])
        .assert()
        .success();
    tracker(&dir)
        .args(["create", "Medium item", "--priority", "medium"])
        .assert()
        .success();
    tracker(&dir)
        .args(["create", "High item", "--priority", "high"])
        .assert()
        .success();

    let output = tracker(&dir)
        .args(["list", "--priority", "high"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let out = String::from_utf8(output).unwrap();
    assert!(
        out.contains("High item"),
        "high-priority issue should appear under --priority high"
    );
    assert!(
        !out.contains("Medium item"),
        "medium-priority issue should NOT appear under --priority high"
    );
    assert!(
        !out.contains("Low item"),
        "low-priority issue should NOT appear under --priority high"
    );
}

#[test]
fn list_invalid_priority_filter_exits_one() {
    // DESIGN.md Feature 2: "Error: Invalid priority '<v>'. Expected: low, medium, or high."
    // Literal spec assertion — substring `Invalid priority` is too lax; the spec mandates the
    // offending value and the expected-list to be reported.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["list", "--priority", "urgent"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains(
            "Error: Invalid priority 'urgent'. Expected: low, medium, or high.",
        ))
        .stdout("");
}

#[test]
fn list_priority_filter_no_match_shows_filter_message() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Low item", "--priority", "low"])
        .assert()
        .success();

    // SO Review 13 F2: filter messages route to stderr; stdout stays empty so
    // pipelines like `tracker list --priority high | wc -l` see 0, not 1.
    tracker(&dir)
        .args(["list", "--priority", "high"])
        .assert()
        .success()
        .stdout("")
        .stderr(predicate::str::contains(
            "No issues match the given filters.",
        ))
        .stderr(predicate::str::contains("Nice work!").not());
}

// --- list output format: column separators (DESIGN.md "exactly 2 spaces") ---

#[test]
fn list_columns_use_exactly_two_space_separator() {
    let dir = TempDir::new().unwrap();
    // "in-progress" exactly fills the 11-char Status column; verifies the gap to
    // the Priority column is the spec-required 2 spaces (not 1).
    tracker(&dir)
        .args(["create", "Working on it", "--priority", "medium"])
        .assert()
        .success();
    tracker(&dir)
        .args(["status", "1", "in-progress"])
        .assert()
        .success();

    let output = tracker(&dir)
        .args(["list", "--status", "in-progress"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let out = String::from_utf8(output).unwrap();

    // Header: "ID" + 4 spaces + "Status" + 7 spaces + "Priority" + 2 spaces + "Labels"
    assert!(
        out.contains("ID    Status       Priority  Labels"),
        "header column spacing must use 2-space separators:\n{out}"
    );
    // Row: "in-progress" exactly fills width-11; gap to "medium" must be 2 spaces.
    assert!(
        out.contains("in-progress  medium"),
        "Status (when full-width) must be followed by exactly 2 spaces before Priority:\n{out}"
    );
}
