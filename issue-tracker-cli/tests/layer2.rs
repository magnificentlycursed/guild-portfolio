use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn tracker(dir: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("tracker").unwrap();
    cmd.current_dir(dir.path());
    cmd
}

// --- status: happy path ---

#[test]
fn status_change_exits_zero_and_prints_confirmation() {
    let dir = TempDir::new().unwrap();
    tracker(&dir).args(["create", "Fix bug"]).assert().success();
    tracker(&dir)
        .args(["status", "1", "in-progress"])
        .assert()
        .success()
        .stdout("Issue #1 status \u{2192} in-progress.\n")
        .stderr("");
}

#[test]
fn status_change_updates_json() {
    let dir = TempDir::new().unwrap();
    tracker(&dir).args(["create", "Fix bug"]).assert().success();
    tracker(&dir)
        .args(["status", "1", "in-progress"])
        .assert()
        .success();

    let raw = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
    let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(v[0]["status"], "in-progress");
}

#[test]
fn status_change_refreshes_updated_at() {
    let dir = TempDir::new().unwrap();
    tracker(&dir).args(["create", "Fix bug"]).assert().success();

    let raw_before = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
    let before: serde_json::Value = serde_json::from_str(&raw_before).unwrap();
    let updated_at_before = before[0]["updated_at"].as_str().unwrap().to_string();

    // Sleep 1 second to guarantee a different timestamp at second precision
    std::thread::sleep(std::time::Duration::from_secs(1));

    tracker(&dir)
        .args(["status", "1", "done"])
        .assert()
        .success();

    let raw_after = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
    let after: serde_json::Value = serde_json::from_str(&raw_after).unwrap();
    let updated_at_after = after[0]["updated_at"].as_str().unwrap().to_string();

    assert!(
        updated_at_after > updated_at_before,
        "updated_at should advance after status update: {} > {}",
        updated_at_after,
        updated_at_before
    );
}

#[test]
fn status_change_leaves_other_fields_unchanged() {
    let dir = TempDir::new().unwrap();
    tracker(&dir).args(["create", "Fix bug"]).assert().success();

    let raw_before = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
    let before: serde_json::Value = serde_json::from_str(&raw_before).unwrap();

    tracker(&dir)
        .args(["status", "1", "done"])
        .assert()
        .success();

    let raw_after = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
    let after: serde_json::Value = serde_json::from_str(&raw_after).unwrap();

    assert_eq!(after[0]["id"], before[0]["id"]);
    assert_eq!(after[0]["title"], before[0]["title"]);
    assert_eq!(after[0]["priority"], before[0]["priority"]);
    assert_eq!(after[0]["labels"], before[0]["labels"]);
    assert_eq!(after[0]["created_at"], before[0]["created_at"]);
}

#[test]
fn status_is_case_insensitive_on_input() {
    let dir = TempDir::new().unwrap();
    tracker(&dir).args(["create", "Fix bug"]).assert().success();
    tracker(&dir)
        .args(["status", "1", "DONE"])
        .assert()
        .success();

    let raw = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
    let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(v[0]["status"], "done", "stored value should be lowercase");
}

#[test]
fn status_idempotent_same_value_succeeds() {
    let dir = TempDir::new().unwrap();
    tracker(&dir).args(["create", "Fix bug"]).assert().success();
    tracker(&dir)
        .args(["status", "1", "in-progress"])
        .assert()
        .success();

    let raw_before = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
    let before: serde_json::Value = serde_json::from_str(&raw_before).unwrap();
    let updated_at_before = before[0]["updated_at"].as_str().unwrap().to_string();

    std::thread::sleep(std::time::Duration::from_secs(1));

    tracker(&dir)
        .args(["status", "1", "in-progress"])
        .assert()
        .success()
        .stdout("Issue #1 status \u{2192} in-progress.\n");

    let raw_after = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
    let after: serde_json::Value = serde_json::from_str(&raw_after).unwrap();
    let updated_at_after = after[0]["updated_at"].as_str().unwrap().to_string();

    assert!(
        updated_at_after > updated_at_before,
        "updated_at should refresh even when setting same status"
    );
}

#[test]
fn status_change_does_not_modify_created_at() {
    let dir = TempDir::new().unwrap();
    tracker(&dir).args(["create", "Fix bug"]).assert().success();

    let raw_before = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
    let before: serde_json::Value = serde_json::from_str(&raw_before).unwrap();
    let created_at_before = before[0]["created_at"].as_str().unwrap().to_string();

    tracker(&dir)
        .args(["status", "1", "done"])
        .assert()
        .success();

    let raw_after = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
    let after: serde_json::Value = serde_json::from_str(&raw_after).unwrap();
    let created_at_after = after[0]["created_at"].as_str().unwrap().to_string();

    assert_eq!(
        created_at_after, created_at_before,
        "created_at must not change after a status update"
    );
}

// --- list: status filtering ---

#[test]
fn list_default_excludes_done_issues() {
    let dir = TempDir::new().unwrap();
    tracker(&dir).args(["create", "Fix bug"]).assert().success();
    tracker(&dir)
        .args(["status", "1", "done"])
        .assert()
        .success();

    tracker(&dir)
        .args(["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Fix bug").not());
}

#[test]
fn list_status_filter_shows_done() {
    let dir = TempDir::new().unwrap();
    tracker(&dir).args(["create", "Fix bug"]).assert().success();
    tracker(&dir)
        .args(["create", "Add feature"])
        .assert()
        .success();
    tracker(&dir)
        .args(["status", "1", "done"])
        .assert()
        .success();

    let output = tracker(&dir)
        .args(["list", "--status", "done"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let out = String::from_utf8(output).unwrap();

    assert!(out.contains("Fix bug"), "done issue should appear");
    assert!(!out.contains("Add feature"), "open issue should not appear");
}

#[test]
fn list_status_filter_shows_in_progress() {
    let dir = TempDir::new().unwrap();
    tracker(&dir).args(["create", "Fix bug"]).assert().success();
    tracker(&dir)
        .args(["create", "Add feature"])
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

    assert!(out.contains("Fix bug"), "in-progress issue should appear");
    assert!(!out.contains("Add feature"), "open issue should not appear");
}

#[test]
fn list_explicit_open_filter_matches_default() {
    let dir = TempDir::new().unwrap();
    tracker(&dir).args(["create", "Fix bug"]).assert().success();
    tracker(&dir)
        .args(["create", "Add feature"])
        .assert()
        .success();
    tracker(&dir)
        .args(["status", "1", "done"])
        .assert()
        .success();

    let default_out = tracker(&dir)
        .args(["list"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let explicit_out = tracker(&dir)
        .args(["list", "--status", "open"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    assert_eq!(
        default_out, explicit_out,
        "--status open should produce identical output to no flag"
    );
}

#[test]
fn list_all_done_default_shows_empty_state() {
    let dir = TempDir::new().unwrap();
    tracker(&dir).args(["create", "Fix bug"]).assert().success();
    tracker(&dir)
        .args(["status", "1", "done"])
        .assert()
        .success();

    tracker(&dir)
        .args(["list"])
        .assert()
        .success()
        .stdout("No open issues. Nice work!\n");
}

// --- status: error paths ---

#[test]
fn status_invalid_id_string_exits_one() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["status", "abc", "open"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("not a valid issue ID"))
        .stdout("");
}

#[test]
fn status_zero_id_exits_one() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["status", "0", "open"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("not a valid issue ID"))
        .stdout("");
}

#[test]
fn status_not_found_exits_one() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["status", "99", "open"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Issue #99 not found."))
        .stdout("");
}

#[test]
fn status_invalid_value_exits_one() {
    let dir = TempDir::new().unwrap();
    tracker(&dir).args(["create", "Fix bug"]).assert().success();
    tracker(&dir)
        .args(["status", "1", "flying"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Invalid status"))
        .stdout("");
}

#[test]
fn list_invalid_status_filter_exits_one() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["list", "--status", "flying"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Invalid status"))
        .stdout("");
}

#[test]
fn list_nonempty_status_filter_with_no_match_shows_filter_message() {
    let dir = TempDir::new().unwrap();
    tracker(&dir).args(["create", "Fix bug"]).assert().success();
    // Issue is open; listing --status done should find nothing and print the filter message
    tracker(&dir)
        .args(["list", "--status", "done"])
        .assert()
        .success()
        .stdout("No issues match the given filters.\n");
}
