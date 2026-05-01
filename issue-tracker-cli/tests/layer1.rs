use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn tracker(dir: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("tracker").unwrap();
    cmd.current_dir(dir.path());
    cmd
}

// --- create ---

#[test]
fn create_valid_title_exits_zero_and_prints_confirmation() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Fix bug"])
        .assert()
        .success()
        .stdout("Created issue #1: Fix bug\n")
        .stderr("");
}

#[test]
fn create_stores_issue_in_json() {
    let dir = TempDir::new().unwrap();
    tracker(&dir).args(["create", "Fix bug"]).assert().success();

    let raw = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
    let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(v[0]["title"], "Fix bug");
    assert_eq!(v[0]["id"], 1);
    assert_eq!(v[0]["status"], "open");
    assert_eq!(v[0]["priority"], "medium");
}

#[test]
fn create_empty_title_exits_one_with_error_on_stderr() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", ""])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Error: Title cannot be empty."))
        .stdout("");
}

#[test]
fn create_whitespace_title_exits_one() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "   "])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Error: Title cannot be empty."))
        .stdout("");
}

#[test]
fn create_trims_title() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "  Fix bug  "])
        .assert()
        .success();

    let raw = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
    let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(v[0]["title"], "Fix bug");
}

#[test]
fn create_second_issue_gets_id_2() {
    let dir = TempDir::new().unwrap();
    tracker(&dir).args(["create", "First"]).assert().success();
    tracker(&dir)
        .args(["create", "Second"])
        .assert()
        .success()
        .stdout("Created issue #2: Second\n");
}

#[test]
fn create_first_issue_unchanged_after_second_create() {
    let dir = TempDir::new().unwrap();
    tracker(&dir).args(["create", "First"]).assert().success();

    let raw_after_first = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
    let v1: serde_json::Value = serde_json::from_str(&raw_after_first).unwrap();
    let first_created_at = v1[0]["created_at"].as_str().unwrap().to_string();
    let first_updated_at = v1[0]["updated_at"].as_str().unwrap().to_string();

    tracker(&dir).args(["create", "Second"]).assert().success();

    let raw = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
    let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(v[0]["id"], 1);
    assert_eq!(v[0]["title"], "First");
    assert_eq!(v[0]["status"], "open");
    assert_eq!(v[0]["priority"], "medium");
    assert_eq!(v[0]["labels"], serde_json::json!([]));
    assert_eq!(v[0]["created_at"].as_str().unwrap(), first_created_at);
    assert_eq!(v[0]["updated_at"].as_str().unwrap(), first_updated_at);
}

#[test]
fn create_timestamps_equal_on_fresh_issue() {
    let dir = TempDir::new().unwrap();
    tracker(&dir).args(["create", "Fix bug"]).assert().success();

    let raw = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
    let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let created = v[0]["created_at"].as_str().unwrap();
    let updated = v[0]["updated_at"].as_str().unwrap();
    assert_eq!(created, updated);
}

// --- list ---

#[test]
fn list_with_no_json_shows_empty_state() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["list"])
        .assert()
        .success()
        .stdout("No open issues. Nice work!\n")
        .stderr("");
}

#[test]
fn list_shows_header_and_issues() {
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

    assert!(out.contains("ID"));
    assert!(out.contains("Status"));
    assert!(out.contains("Priority"));
    assert!(out.contains("Labels"));
    assert!(out.contains("Title"));
    assert!(out.contains("Fix bug"));
    assert!(out.contains("(none)"), "unlabeled issue should show '(none)' in Labels column");
}

#[test]
fn list_after_create_shows_issue() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Fix the login bug"])
        .assert()
        .success();

    tracker(&dir)
        .args(["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Fix the login bug"));
}

#[test]
fn list_truncates_title_at_50_chars_with_ellipsis() {
    let dir = TempDir::new().unwrap();
    let long_title = "A".repeat(60);
    tracker(&dir)
        .args(["create", &long_title])
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

    // Truncated to exactly 49 content chars + ellipsis; full 60-char title must not appear
    let expected = format!("{}…", "A".repeat(49));
    assert!(
        out.contains(&expected),
        "expected 49 'A's + '…' in output:\n{out}"
    );
    assert!(!out.contains(&long_title), "full title should be truncated");
}

// --- list ordering and multi-issue ---

#[test]
fn list_shows_multiple_issues_in_id_order() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "First issue"])
        .assert()
        .success();
    tracker(&dir)
        .args(["create", "Second issue"])
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

    assert!(out.contains("First issue"), "first issue should appear in list");
    assert!(out.contains("Second issue"), "second issue should appear in list");
    let pos_first = out.find("First issue").unwrap();
    let pos_second = out.find("Second issue").unwrap();
    assert!(
        pos_first < pos_second,
        "issue #1 should appear before issue #2 (id-ascending order within same priority tier)"
    );
}

// --- corrupt data variants ---

#[test]
fn zero_id_in_json_causes_error_exit() {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("tracker.json"),
        br#"[{"id":0,"title":"Fix bug","status":"open","priority":"medium","labels":[],"created_at":"2026-01-01T00:00:00Z","updated_at":"2026-01-01T00:00:00Z"}]"#,
    )
    .unwrap();
    tracker(&dir)
        .args(["list"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains(
            "The file may be corrupt. Delete tracker.json to start fresh.",
        ));
}

// --- error handling ---

#[test]
fn invalid_domain_values_in_json_causes_error_exit() {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("tracker.json"),
        br#"[{"id":1,"title":"Fix bug","status":"flying","priority":"medium","labels":[],"created_at":"2026-01-01T00:00:00Z","updated_at":"2026-01-01T00:00:00Z"}]"#,
    )
    .unwrap();
    tracker(&dir)
        .args(["list"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains(
            "The file may be corrupt. Delete tracker.json to start fresh.",
        ));
}

#[test]
fn malformed_json_causes_error_exit() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("tracker.json"), b"not json at all").unwrap();

    tracker(&dir)
        .args(["list"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains(
            "The file may be corrupt. Delete tracker.json to start fresh.",
        ));
}
