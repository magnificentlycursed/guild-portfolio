//! Phase 2a Red Gate — Layer 1 integration tests for bookmark-cli.
//!
//! Per `vsdd-suite/primers/2a-red-gate.md`: every test in this file must
//! fail against an empty function body and must fail for the right reason
//! (missing feature, not setup error). Tests invoke the compiled `bm`
//! binary via `assert_cmd` per the suite's CLI supplement § Quality
//! Engineering ("integration tests invoke the binary"). Each test gets
//! an isolated `BOOKMARK_CLI_DB` via `tempfile` — no shared state.
//!
//! The four tests below correspond 1:1 to the four acceptance criteria
//! in `TODO.md` § Layer 1.

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::thread;
use std::time::Duration;
use tempfile::tempdir;

/// AC 1 — `bm add <url>` creates a bookmark; exit 0; stdout silent.
#[test]
fn tests_add_creates_bookmark() {
    let dir = tempdir().unwrap();
    let db = dir.path().join("bookmarks.json");

    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["add", "https://example.com"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty());

    let contents = fs::read_to_string(&db).expect("store should exist after add");
    let parsed: serde_json::Value = serde_json::from_str(&contents).expect("store should be valid JSON");
    let bookmarks = parsed["bookmarks"].as_array().expect("bookmarks should be an array");
    assert_eq!(bookmarks.len(), 1, "exactly one bookmark expected after one add");
    assert_eq!(bookmarks[0]["url"], "https://example.com");
    let ts = bookmarks[0]["timestamp"].as_str().expect("timestamp should be a string");
    assert!(
        chrono::DateTime::parse_from_rfc3339(ts).is_ok(),
        "timestamp {ts:?} should be RFC 3339"
    );
}

/// AC 2 — `bm add ""` exits 1 with specific stderr; no file written.
#[test]
fn tests_add_rejects_empty_url() {
    let dir = tempdir().unwrap();
    let db = dir.path().join("bookmarks.json");

    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["add", ""])
        .assert()
        .failure()
        .code(1)
        .stderr("Error: URL cannot be empty.\n")
        .stdout(predicate::str::is_empty());

    assert!(!db.exists(), "store must not be created on empty-URL rejection");
}

/// AC 3 — `bm list` prints bookmarks newest-first.
#[test]
fn tests_list_orders_newest_first() {
    let dir = tempdir().unwrap();
    let db = dir.path().join("bookmarks.json");

    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["add", "https://first.example"])
        .assert()
        .success();

    // Ensure the second bookmark has a strictly later timestamp.
    thread::sleep(Duration::from_millis(1100));

    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["add", "https://second.example"])
        .assert()
        .success();

    let output = Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["list"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let rendered = String::from_utf8(output).unwrap();
    let lines: Vec<&str> = rendered.lines().collect();
    assert_eq!(lines.len(), 2, "two bookmarks rendered, one per line; got {lines:?}");
    assert!(
        lines[0].ends_with("https://second.example"),
        "newest bookmark (second.example) should be first line; got {:?}",
        lines[0]
    );
    assert!(
        lines[1].ends_with("https://first.example"),
        "older bookmark (first.example) should be second line; got {:?}",
        lines[1]
    );
}

/// AC 4 — `bm list` against absent store exits 0 with empty-state stderr.
#[test]
fn tests_list_empty_state() {
    let dir = tempdir().unwrap();
    let db = dir.path().join("bookmarks.json");
    // Deliberately do NOT create the store file.

    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["list"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::is_empty())
        .stderr("No bookmarks yet.\n");
}
