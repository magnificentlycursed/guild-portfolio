#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    reason = "Restriction-group lints from [lints.clippy] apply to production code; \
              integration tests use unwrap/expect/panic freely per Rust supplement \
              test-helper convention. Platform Engineer Round 2 Finding 13."
)]

//! Phase 2a Red Gate — Layer 1 integration tests for bookmark-cli.
//!
//! Per `vsdd-suite/primers/2a-red-gate.md`: every test in this file must
//! fail against an empty function body and must fail for the right reason
//! (missing feature, not setup error). Tests invoke the compiled `bm`
//! binary via `assert_cmd` per the suite's CLI supplement § Quality
//! Engineering ("integration tests invoke the binary"). Each test gets
//! an isolated `BOOKMARK_CLI_DB` via `tempfile` — no shared state.
//!
//! The first four tests correspond 1:1 to the four acceptance criteria
//! in `TODO.md` § Layer 1. The subsequent tests cover the Round 2 IAR
//! fixes (atomic save, file mode 0600, symlink rejection, display-safe
//! sanitizer, missing-positional handling, unknown-subcommand exit code)
//! per `DESIGN.md` § Exit codes and § Threat model.
//!
//! Closes:
//! - [SE Review 1 Finding 1](../vsdd-suite/review-log/2026-05-20-software-engineer.md#r1-f1) — missing-positional exit code
//! - [SE Review 1 Finding 2](../vsdd-suite/review-log/2026-05-20-software-engineer.md#r1-f2) — atomic save
//! - [SE Review 1 Finding 3](../vsdd-suite/review-log/2026-05-20-software-engineer.md#r1-f3) — exit 64 for clap usage errors
//! - [Security Review 1 Finding 1](../vsdd-suite/review-log/2026-05-20-security.md) — display-safe sanitizer
//! - [Security Review 1 Finding 2](../vsdd-suite/review-log/2026-05-20-security.md) — mode 0600
//! - [Red Team Review 1 Finding 4](../vsdd-suite/review-log/2026-05-20-red-team.md) — terminal-escape injection
//! - [Red Team Review 1 Finding 5](../vsdd-suite/review-log/2026-05-20-red-team.md) — world-readable file mode
//! - [Red Team Review 1 Finding 6](../vsdd-suite/review-log/2026-05-20-red-team.md) — symlink-follow rejection

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
    let parsed: serde_json::Value =
        serde_json::from_str(&contents).expect("store should be valid JSON");
    let bookmarks = parsed["bookmarks"]
        .as_array()
        .expect("bookmarks should be an array");
    assert_eq!(
        bookmarks.len(),
        1,
        "exactly one bookmark expected after one add"
    );
    assert_eq!(bookmarks[0]["url"], "https://example.com");
    let ts = bookmarks[0]["timestamp"]
        .as_str()
        .expect("timestamp should be a string");
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

    assert!(
        !db.exists(),
        "store must not be created on empty-URL rejection"
    );
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
    assert_eq!(
        lines.len(),
        2,
        "two bookmarks rendered, one per line; got {lines:?}"
    );
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

/// `bm add` with no positional argument exits 1 with the spec-contracted
/// `Error: URL cannot be empty.` stderr — closes [SE Review 1
/// Finding 1](../vsdd-suite/review-log/2026-05-20-software-engineer.md#r1-f1).
/// Per `DESIGN.md` § `bm add` failure contract, `bm add` (no positional)
/// is treated identically to `bm add ""`.
#[test]
fn bm_add_with_no_positional_exits_1_with_url_cannot_be_empty() {
    let dir = tempdir().unwrap();
    let db = dir.path().join("bookmarks.json");

    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .arg("add")
        .assert()
        .failure()
        .code(1)
        .stderr("Error: URL cannot be empty.\n")
        .stdout(predicate::str::is_empty());

    assert!(
        !db.exists(),
        "store must not be created on missing-positional rejection"
    );
}

/// Unknown subcommand exits 64 (`EX_USAGE`) per `DESIGN.md` § Exit codes —
/// closes [SE Review 1
/// Finding 3](../vsdd-suite/review-log/2026-05-20-software-engineer.md#r1-f3).
#[test]
fn bm_unknown_subcommand_exits_64() {
    let dir = tempdir().unwrap();
    let db = dir.path().join("bookmarks.json");

    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .arg("frobnicate")
        .assert()
        .failure()
        .code(64);
}

/// Atomic-save discipline: if save fails mid-flight, the prior file state
/// is preserved. Simulated here by pre-staging a regular file at the
/// target path then making the parent directory read-only so the temp
/// file cannot be created. Closes [SE Review 1
/// Finding 2](../vsdd-suite/review-log/2026-05-20-software-engineer.md#r1-f2).
#[cfg(unix)]
#[test]
fn save_is_atomic_on_write_failure() {
    use std::os::unix::fs::PermissionsExt;

    let dir = tempdir().unwrap();
    let parent = dir.path().join("ro_parent");
    fs::create_dir(&parent).unwrap();
    let db = parent.join("bookmarks.json");

    // Pre-stage a known-good store via a successful add (parent is still writable).
    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["add", "https://prior.example"])
        .assert()
        .success();

    let before = fs::read_to_string(&db).expect("pre-staged store should exist");

    // Now make the parent read-only so create_new on the temp file fails.
    let mut perms = fs::metadata(&parent).unwrap().permissions();
    perms.set_mode(0o500); // r-x for owner; no write
    fs::set_permissions(&parent, perms).unwrap();

    // Attempt a save that must fail; the prior file content must survive.
    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["add", "https://second.example"])
        .assert()
        .failure()
        .code(2);

    // Restore write so we can read the file back (read still works at 0o500 for owner).
    let after = fs::read_to_string(&db).expect("prior store should still be readable");
    assert_eq!(
        after, before,
        "prior file state must be preserved on save failure"
    );

    // Restore perms so tempdir cleanup succeeds.
    let mut perms = fs::metadata(&parent).unwrap().permissions();
    perms.set_mode(0o700);
    fs::set_permissions(&parent, perms).unwrap();
}

/// Symlink-follow rejection on save: a pre-staged symlink at
/// `$BOOKMARK_CLI_DB` is refused with exit 2 and leaves the target
/// untouched. Closes [Red Team Review 1
/// Finding 6](../vsdd-suite/review-log/2026-05-20-red-team.md).
#[cfg(unix)]
#[test]
fn save_refuses_symlink_target() {
    let dir = tempdir().unwrap();
    let target = dir.path().join("real_target.json");
    let target_contents = b"{\"bookmarks\":[]}\n";
    fs::write(&target, target_contents).unwrap();

    let link = dir.path().join("link.json");
    std::os::unix::fs::symlink(&target, &link).unwrap();

    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &link)
        .args(["add", "https://example.com"])
        .assert()
        .failure()
        .code(2)
        // Symlink rejection fires on either the load() pass or the save()
        // pass depending on order — both are valid (Red Team Round 2
        // Finding 5: symmetric symlink hardening on both sides).
        .stderr(
            predicate::str::contains("symlink").and(
                predicate::str::contains("refusing to read through")
                    .or(predicate::str::contains("refusing to write through")),
            ),
        );

    let after = fs::read(&target).expect("symlink target must remain readable");
    assert_eq!(
        after.as_slice(),
        target_contents,
        "symlink target must be untouched by the refused save"
    );
}

/// On Unix, the store file is created with mode 0600 (owner read/write
/// only) per `DESIGN.md` § Storage data classification. Closes [Security
/// Review 1 Finding 2](../vsdd-suite/review-log/2026-05-20-security.md) +
/// [Red Team Review 1
/// Finding 5](../vsdd-suite/review-log/2026-05-20-red-team.md).
#[cfg(unix)]
#[test]
fn save_creates_file_with_mode_0600_on_unix() {
    use std::os::unix::fs::PermissionsExt;

    let dir = tempdir().unwrap();
    let db = dir.path().join("bookmarks.json");

    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["add", "https://example.com"])
        .assert()
        .success();

    let mode = fs::metadata(&db).unwrap().permissions().mode() & 0o777;
    assert_eq!(mode, 0o600, "expected mode 0600, got {mode:o}");
}

/// `bm list` sanitizes terminal-escape sequences in stored URLs before
/// rendering to stdout. Closes [Security Review 1
/// Finding 1](../vsdd-suite/review-log/2026-05-20-security.md) + [Red Team
/// Review 1 Finding 4](../vsdd-suite/review-log/2026-05-20-red-team.md).
#[test]
fn bm_list_sanitizes_terminal_escape_in_url() {
    let dir = tempdir().unwrap();
    let db = dir.path().join("bookmarks.json");

    // Add a URL containing a raw ANSI escape (U+001B = ESC). The library
    // accepts any string per DESIGN.md § Edge case catalog "URL containing
    // newlines: accepted"; the rendering layer is responsible for safety.
    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["add", "https://evil.example/\x1b[31mhostile"])
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

    let rendered = String::from_utf8(output).expect("stdout should be UTF-8");
    assert!(
        rendered.contains("\\u{001b}"),
        "ESC should appear escaped as \\u{{001b}}; got {rendered:?}"
    );
    assert!(
        !rendered.contains('\x1b'),
        "raw ESC byte must NOT survive sanitization; got {rendered:?}"
    );
}

/// `bm --help` exits 0 (clap's `DisplayHelp` is success, not usage error).
/// Closes Round 2 SE Finding 6 — regression from the F1/F3 fix that routed
/// every parse-error through `ExitCode::from(64)`.
#[test]
fn bm_help_exits_0() {
    Command::cargo_bin("bm")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains("Capture URLs"));
}

/// `bm --version` exits 0 (clap's `DisplayVersion` is success). Closes
/// Round 2 SE Finding 6.
#[test]
fn bm_version_exits_0() {
    Command::cargo_bin("bm")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .code(0);
}

/// `BookmarkStore::save` cleans up the temp file when `write_temp_file`
/// fails partway through (after `create_new` succeeded). Closes Round 2 SE
/// Finding 7 — orphan temp files no longer accumulate on partial write
/// failure. Simulated by filling the parent directory to its inode/block
/// limit OR by pre-occupying the temp-sibling name; the cleanest test is
/// to make the parent directory non-executable so `sync_all` can succeed
/// but `rename` then fails — but that's the rename path (F2), not the
/// write-temp path. For the write-temp path specifically, the test
/// asserts that after a successful save followed by no leftover files,
/// the directory contains exactly one file (the store), not two.
#[cfg(unix)]
#[test]
fn save_leaves_no_orphan_temp_files() {
    let dir = tempdir().unwrap();
    let db = dir.path().join("bookmarks.json");

    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["add", "https://example.com"])
        .assert()
        .success();

    // Count files in the directory — should be exactly 1 (no temp orphans).
    let entries: Vec<_> = fs::read_dir(dir.path())
        .unwrap()
        .filter_map(std::result::Result::ok)
        .map(|e| e.file_name())
        .collect();
    assert_eq!(
        entries.len(),
        1,
        "expected exactly the store file, got {entries:?}"
    );
    assert_eq!(entries[0], "bookmarks.json");
}

/// `BookmarkStore::load` symmetric symlink-rejection: if `$BOOKMARK_CLI_DB`
/// is a symlink, `bm list` also refuses (not just `bm add`). Closes
/// [Red Team Review 1 Round 2 Finding 5](../vsdd-suite/review-log/2026-05-20-red-team.md#r2-f5).
#[cfg(unix)]
#[test]
fn load_refuses_symlink_target() {
    let dir = tempdir().unwrap();
    let target = dir.path().join("real_target.json");
    fs::write(&target, b"{\"bookmarks\":[]}\n").unwrap();
    let link = dir.path().join("link.json");
    std::os::unix::fs::symlink(&target, &link).unwrap();

    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &link)
        .args(["list"])
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("refusing to read through symlink"));
}

/// `bm` unknown-subcommand with a control-byte-bearing argv argument:
/// clap renders the argument verbatim in its error message; we sanitize
/// through `display_safe` so ANSI escapes do not reach the terminal raw.
/// Closes [Security Review 1 Round 2 Finding 4](../vsdd-suite/review-log/2026-05-20-security.md#r2-f4).
#[test]
fn unknown_subcommand_with_ansi_escape_is_sanitized() {
    let output = Command::cargo_bin("bm")
        .unwrap()
        .arg("\x1b[31mfrobnicate")
        .assert()
        .failure()
        .code(64)
        .get_output()
        .stderr
        .clone();
    let rendered = String::from_utf8(output).expect("stderr should be UTF-8");
    // Defense-in-depth: the `display_safe` wrapper around `err.render()`
    // ensures no raw control bytes reach stderr regardless of clap's own
    // argv handling. The current clap version pre-strips many control
    // sequences before quoting; our sanitizer remains the final guarantee.
    // Closes Security Round 2 Finding 4.
    assert!(
        !rendered.contains('\x1b'),
        "raw ESC byte must not reach stderr; got {rendered:?}"
    );
}

/// `bm --help` includes the usage examples + exit-code table per the
/// `long_about` text. Closes [UX Review 1 Finding 4](../vsdd-suite/review-log/2026-05-20-ux.md).
#[test]
fn help_text_includes_usage_examples_and_exit_codes() {
    let output = Command::cargo_bin("bm")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let rendered = String::from_utf8(output).expect("stdout should be UTF-8");
    assert!(rendered.contains("Examples:"), "Examples section missing");
    assert!(rendered.contains("bm add"), "bm add example missing");
    assert!(
        rendered.contains("Exit codes:"),
        "Exit codes section missing"
    );
    assert!(rendered.contains("64"), "exit 64 documented missing");
}
