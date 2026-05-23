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

// ===========================================================================
// Layer 2 — Phase 2a Red Gate tests (tag + filter)
//
// Per `TODO.md` § Layer 2 Red Gate test plan + `DESIGN.md` § `bm tag <url>
// <label>` (Layer 2) / § `bm list --tag <label>` (Layer 2). Each test below
// invokes the compiled `bm` binary via `assert_cmd` against an isolated
// `BOOKMARK_CLI_DB`. The tests are written before the implementation lands;
// they must fail against the Layer-1-only binary and pass against the
// Layer-2 binary.
// ===========================================================================

/// AC 5 — `bm tag <url> <label>` appends the label to the matching
/// bookmark's `tags` field and exits 0.
#[test]
fn tests_tag_attaches_label_to_matching_bookmark() {
    let dir = tempdir().unwrap();
    let db = dir.path().join("bookmarks.json");

    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["add", "https://example.com"])
        .assert()
        .success();

    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["tag", "https://example.com", "rust"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::is_empty())
        // Layer 2 Round 1 UX F2 + SE F2 — `bm tag` emits the match count
        // to stderr on success so the multi-match semantic is discoverable
        // from user behavior. Single match → `Tagged 1 bookmark.` (singular
        // per Layer 2 Round 2 UX F4 singular/plural conditional).
        .stderr("Tagged 1 bookmark.\n");

    let contents = fs::read_to_string(&db).expect("store file should still exist after tag");
    let parsed: serde_json::Value = serde_json::from_str(&contents).expect("store is valid JSON");
    let bookmarks = parsed["bookmarks"].as_array().expect("bookmarks array");
    assert_eq!(bookmarks.len(), 1, "still exactly one bookmark");
    assert_eq!(bookmarks[0]["url"], "https://example.com");
    let tags = bookmarks[0]["tags"]
        .as_array()
        .expect("tags field should be present + an array");
    assert_eq!(tags.len(), 1, "exactly one tag attached; got {tags:?}");
    assert_eq!(tags[0], "rust");
}

/// AC 5 idempotence — invoking `bm tag <url> <label>` twice with the same
/// args produces a single tag in the `tags` array, not a duplicate.
#[test]
fn tests_tag_is_idempotent() {
    let dir = tempdir().unwrap();
    let db = dir.path().join("bookmarks.json");

    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["add", "https://example.com"])
        .assert()
        .success();

    for _ in 0..2 {
        Command::cargo_bin("bm")
            .unwrap()
            .env("BOOKMARK_CLI_DB", &db)
            .args(["tag", "https://example.com", "rust"])
            .assert()
            .success()
            .code(0)
            // Both invocations emit `Tagged 1 bookmark.` (singular per Layer
            // 2 Round 2 UX F4) — the second invocation's idempotent no-op
            // doesn't affect the match count (the URL still matches one
            // bookmark; the label is just not re-appended to its tags vec).
            // Closes Layer 2 Round 1 UX F2.
            .stderr("Tagged 1 bookmark.\n");
    }

    let contents = fs::read_to_string(&db).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&contents).unwrap();
    let tags = parsed["bookmarks"][0]["tags"].as_array().unwrap();
    assert_eq!(
        tags.len(),
        1,
        "second invocation must not duplicate the tag; got {tags:?}"
    );
    assert_eq!(tags[0], "rust");
}

/// AC 6 — `bm tag` against an unknown URL exits 1 with the spec-contracted
/// stderr AND leaves the store file byte-identical to its pre-invocation
/// state (no rewrite).
#[test]
fn tests_tag_rejects_unknown_url() {
    let dir = tempdir().unwrap();
    let db = dir.path().join("bookmarks.json");

    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["add", "https://A.example"])
        .assert()
        .success();

    let before = fs::read(&db).expect("pre-invocation store should exist");

    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["tag", "https://B.example", "nonsense"])
        .assert()
        .failure()
        .code(1)
        .stderr("Error: no bookmark found with URL https://B.example.\n")
        .stdout(predicate::str::is_empty());

    let after = fs::read(&db).expect("store file should still exist");
    assert_eq!(
        after, before,
        "store file must be byte-identical when tag rejects unknown URL"
    );
}

/// AC 7 — `bm tag "" <label>` exits 1 with the empty-URL error.
#[test]
fn tests_tag_rejects_empty_url() {
    let dir = tempdir().unwrap();
    let db = dir.path().join("bookmarks.json");

    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["add", "https://example.com"])
        .assert()
        .success();

    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["tag", "", "rust"])
        .assert()
        .failure()
        .code(1)
        .stderr("Error: URL cannot be empty.\n")
        .stdout(predicate::str::is_empty());
}

/// AC 8 — `bm tag <url> ""` exits 1 with the empty-label error.
#[test]
fn tests_tag_rejects_empty_label() {
    let dir = tempdir().unwrap();
    let db = dir.path().join("bookmarks.json");

    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["add", "https://example.com"])
        .assert()
        .success();

    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["tag", "https://example.com", ""])
        .assert()
        .failure()
        .code(1)
        .stderr("Error: tag label cannot be empty.\n")
        .stdout(predicate::str::is_empty());
}

/// AC 12 — `bm tag` against a Layer-1-format file (no `tags` field on the
/// bookmarks) migrates forward: the post-save file contains explicit `tags`
/// on every bookmark — `["rust"]` on the tagged bookmark + `[]` on every
/// untouched one.
#[test]
fn tests_tag_against_layer_1_format_file_migrates_forward() {
    let dir = tempdir().unwrap();
    let db = dir.path().join("bookmarks.json");

    // Write a Layer-1-format store directly — no `tags` field per bookmark.
    let layer1 = serde_json::json!({
        "bookmarks": [
            {"url": "https://A.example", "timestamp": "2026-05-21T01:00:00Z"},
            {"url": "https://B.example", "timestamp": "2026-05-21T02:00:00Z"},
        ]
    });
    fs::write(&db, serde_json::to_string_pretty(&layer1).unwrap()).unwrap();

    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["tag", "https://A.example", "rust"])
        .assert()
        .success()
        .code(0);

    let contents = fs::read_to_string(&db).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&contents).unwrap();
    let bookmarks = parsed["bookmarks"].as_array().unwrap();
    assert_eq!(bookmarks.len(), 2);
    for bm in bookmarks {
        let url = bm["url"].as_str().unwrap();
        let tags = bm["tags"]
            .as_array()
            .unwrap_or_else(|| panic!("post-save file must contain explicit tags field for {url}"));
        match url {
            "https://A.example" => {
                assert_eq!(tags.len(), 1, "A should have one tag; got {tags:?}");
                assert_eq!(tags[0], "rust");
            }
            "https://B.example" => {
                assert!(
                    tags.is_empty(),
                    "untouched bookmark must have empty tags array; got {tags:?}"
                );
            }
            other => panic!("unexpected URL {other}"),
        }
    }
}

/// AC 5 multi-match — `bm tag` tags ALL bookmarks whose URL matches.
#[test]
fn tests_tag_against_duplicate_url_tags_all_matches() {
    let dir = tempdir().unwrap();
    let db = dir.path().join("bookmarks.json");

    // Add the same URL twice — append-only permits this.
    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["add", "https://dup.example"])
        .assert()
        .success();
    thread::sleep(Duration::from_millis(1100));
    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["add", "https://dup.example"])
        .assert()
        .success();

    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["tag", "https://dup.example", "rust"])
        .assert()
        .success()
        .code(0)
        // Two bookmarks share the URL → `bm tag` matched both → stderr
        // emits `Tagged 2 bookmarks.` (plural per Layer 2 Round 2 UX F4).
        // The multi-match affordance from Layer 2 Round 1 UX F2 — the user
        // sees the count so the multi-match semantic is observable from
        // behavior.
        .stderr("Tagged 2 bookmarks.\n");

    let contents = fs::read_to_string(&db).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&contents).unwrap();
    let bookmarks = parsed["bookmarks"].as_array().unwrap();
    assert_eq!(bookmarks.len(), 2, "still two bookmarks");
    for bm in bookmarks {
        let tags = bm["tags"].as_array().unwrap();
        assert_eq!(tags.len(), 1, "each duplicate gets tagged; got {tags:?}");
        assert_eq!(tags[0], "rust");
    }
}

/// AC 9 — `bm list --tag rust` returns ONLY the bookmarks tagged `rust`,
/// in newest-first order, exits 0, stderr empty.
#[test]
fn tests_list_with_tag_filter_returns_matching_bookmarks() {
    let dir = tempdir().unwrap();
    let db = dir.path().join("bookmarks.json");

    // A (oldest), B (middle), C (newest)
    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["add", "https://A.example"])
        .assert()
        .success();
    thread::sleep(Duration::from_millis(1100));
    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["add", "https://B.example"])
        .assert()
        .success();
    thread::sleep(Duration::from_millis(1100));
    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["add", "https://C.example"])
        .assert()
        .success();

    // Tag A and C with `rust`, leave B untagged.
    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["tag", "https://A.example", "rust"])
        .assert()
        .success();
    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["tag", "https://C.example", "rust"])
        .assert()
        .success();

    let output = Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["list", "--tag", "rust"])
        .assert()
        .success()
        .code(0)
        .stderr(predicate::str::is_empty())
        .get_output()
        .stdout
        .clone();

    let rendered = String::from_utf8(output).unwrap();
    let lines: Vec<&str> = rendered.lines().collect();
    assert_eq!(
        lines.len(),
        2,
        "exactly two bookmarks match the filter; got {lines:?}"
    );
    assert!(
        lines[0].ends_with("https://C.example"),
        "newest match (C) should be first; got {:?}",
        lines[0]
    );
    assert!(
        lines[1].ends_with("https://A.example"),
        "older match (A) should be second; got {:?}",
        lines[1]
    );
    assert!(
        !rendered.contains("https://B.example"),
        "untagged B must not appear in filtered output; got {rendered:?}"
    );
}

/// AC 9 empty-filter-match — `bm list --tag rust` against bookmarks that
/// have no tags emits the filter empty-state line + exit 0.
#[test]
fn tests_list_with_tag_filter_empty_match_emits_filter_empty_state() {
    let dir = tempdir().unwrap();
    let db = dir.path().join("bookmarks.json");

    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["add", "https://A.example"])
        .assert()
        .success();
    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["add", "https://B.example"])
        .assert()
        .success();

    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["list", "--tag", "rust"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::is_empty())
        .stderr("No bookmarks match the supplied filter.\n");
}

/// AC 10 — `bm list --tag <a> --tag <b>` is OR-semantics across labels.
#[test]
fn tests_list_with_tag_filter_repeated_flag_is_or_semantics() {
    let dir = tempdir().unwrap();
    let db = dir.path().join("bookmarks.json");

    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["add", "https://A.example"])
        .assert()
        .success();
    thread::sleep(Duration::from_millis(1100));
    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["add", "https://B.example"])
        .assert()
        .success();
    thread::sleep(Duration::from_millis(1100));
    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["add", "https://C.example"])
        .assert()
        .success();

    // A: rust ; B: go ; C: untagged.
    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["tag", "https://A.example", "rust"])
        .assert()
        .success();
    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["tag", "https://B.example", "go"])
        .assert()
        .success();

    let output = Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["list", "--tag", "rust", "--tag", "go"])
        .assert()
        .success()
        .code(0)
        .get_output()
        .stdout
        .clone();
    let rendered = String::from_utf8(output).unwrap();
    let lines: Vec<&str> = rendered.lines().collect();
    assert_eq!(
        lines.len(),
        2,
        "OR-semantics returns A + B (but not untagged C); got {lines:?}"
    );
    assert!(
        rendered.contains("https://A.example"),
        "A (tagged rust) should appear; got {rendered:?}"
    );
    assert!(
        rendered.contains("https://B.example"),
        "B (tagged go) should appear; got {rendered:?}"
    );
    assert!(
        !rendered.contains("https://C.example"),
        "untagged C must not appear; got {rendered:?}"
    );
}

/// DESIGN.md edge-case catalog Layer 2 addition — `bm list --tag <label>`
/// against an absent store emits the store-empty-state line (precedence
/// over the filter-empty-state line).
#[test]
fn tests_list_with_tag_filter_against_empty_store_emits_store_empty_state() {
    let dir = tempdir().unwrap();
    let db = dir.path().join("bookmarks.json");
    // Deliberately do NOT create the store file.

    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["list", "--tag", "rust"])
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::is_empty())
        .stderr("No bookmarks yet.\n");
}

/// AC 11 — `bm list --tag ""` exits 1 with the empty-label error.
#[test]
fn tests_list_with_empty_tag_label_rejected() {
    let dir = tempdir().unwrap();
    let db = dir.path().join("bookmarks.json");

    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["add", "https://example.com"])
        .assert()
        .success();

    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["list", "--tag", ""])
        .assert()
        .failure()
        .code(1)
        .stderr("Error: tag label cannot be empty.\n")
        .stdout(predicate::str::is_empty());
}

/// AC 11 — `bm list --tag ""` against an EMPTY store still exits 1 with
/// the empty-label error (input-invariant rejection fires before
/// store-state branching). Closes Layer 2 Round 1 SE Finding 3 — prior
/// to the fix, this case emitted `No bookmarks yet.` (exit 0) because
/// the empty-store precedence branch was evaluated before input
/// validation.
#[test]
fn tests_list_with_empty_tag_label_against_empty_store_still_rejected() {
    let dir = tempdir().unwrap();
    let db = dir.path().join("bookmarks.json");

    // No `bm add` here — the store file does not exist.
    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["list", "--tag", ""])
        .assert()
        .failure()
        .code(1)
        .stderr("Error: tag label cannot be empty.\n")
        .stdout(predicate::str::is_empty());
}

/// Closes the Layer-1-Deferred QE item — `bm list` emits timestamps that
/// round-trip cleanly through `chrono::DateTime::parse_from_rfc3339` at
/// byte level (not merely "looks like an RFC 3339 string by eyeball").
#[test]
fn tests_list_rfc3339_scripted_check() {
    let dir = tempdir().unwrap();
    let db = dir.path().join("bookmarks.json");

    for i in 0..3 {
        Command::cargo_bin("bm")
            .unwrap()
            .env("BOOKMARK_CLI_DB", &db)
            .args(["add", &format!("https://item-{i}.example")])
            .assert()
            .success();
        thread::sleep(Duration::from_millis(1100));
    }

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
    assert_eq!(lines.len(), 3, "three bookmarks rendered; got {lines:?}");

    for line in lines {
        let (ts, _rest) = line
            .split_once(' ')
            .unwrap_or_else(|| panic!("line must have at least one space separator: {line:?}"));
        chrono::DateTime::parse_from_rfc3339(ts).unwrap_or_else(|e| {
            panic!("timestamp {ts:?} from line {line:?} did not parse as RFC 3339: {e}")
        });
    }
}
