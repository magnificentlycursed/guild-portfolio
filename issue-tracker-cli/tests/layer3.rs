use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn tracker(dir: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("tracker").unwrap();
    cmd.current_dir(dir.path());
    cmd
}

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
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Fix bug", "--priority", "critical"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Invalid priority"))
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
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["list", "--priority", "urgent"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Invalid priority"))
        .stdout("");
}
