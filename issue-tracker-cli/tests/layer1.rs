use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

mod common;
use common::tracker;

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
fn create_title_with_newline_exits_one() {
    // SO Review 13 F1: control characters in titles break the one-issue-per-line
    // `list` contract. Reject at create time with a specific error message.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Sneaky\nlist break"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains(
            "Error: Title cannot contain control characters.",
        ))
        .stdout("");
}

#[test]
fn create_title_with_ansi_escape_exits_one() {
    // SO Review 13 F1: ESC (0x1B) is a control character (Cc); rejecting it at
    // validate_title kills the ANSI-injection attack surface (UX F3 / Red Team F1).
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "\u{1B}[2JEvil"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains(
            "Error: Title cannot contain control characters.",
        ))
        .stdout("");
}

#[test]
fn create_title_with_printable_unicode_succeeds() {
    // Verify the rule rejects controls only — printable Unicode (emoji, CJK)
    // remains accepted.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Fix login 🐛 修复"])
        .assert()
        .success()
        .stdout("Created issue #1: Fix login 🐛 修复\n")
        .stderr("");
}

#[test]
fn control_char_title_in_json_causes_error_exit() {
    // SO Review 13 F1: the same rule applies at load time. Stored data with a
    // control-character title is corrupt; otherwise an attacker who delivered a
    // hand-edited tracker.json could bypass create-time validation.
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("tracker.json"),
        br#"[{"id":1,"title":"Sneaky\nbreak","status":"open","priority":"medium","labels":[],"created_at":"2026-01-01T00:00:00Z","updated_at":"2026-01-01T00:00:00Z"}]"#,
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
fn list_with_no_json_shows_empty_state_on_stderr() {
    // DESIGN.md "stderr contract" / "Edge Cases / List": empty-state messages
    // are informational, not data — they go to stderr so a piped consumer sees
    // an empty stdout when no records match. SO Review 13 Finding 2.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["list"])
        .assert()
        .success()
        .stdout("")
        .stderr("No open issues. Nice work!\n");
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
    assert!(
        out.contains("(none)"),
        "unlabeled issue should show '(none)' in Labels column"
    );
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
    // Off-by-one guard: 50 content chars + ellipsis would be 51 display chars, exceeding the 50-char column limit
    let not_expected = format!("{}…", "A".repeat(50));
    assert!(
        !out.contains(&not_expected),
        "title must not truncate to 50 chars + ellipsis (would exceed 50-char display limit)"
    );
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

    assert!(
        out.contains("First issue"),
        "first issue should appear in list"
    );
    assert!(
        out.contains("Second issue"),
        "second issue should appear in list"
    );
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

#[test]
fn duplicate_ids_in_json_causes_error_exit() {
    // DESIGN.md: "no two issues share the same ID". Duplicate IDs in stored
    // data must be rejected at load — otherwise `tracker status <id>` mutates
    // only the first match and silently desynchronizes the duplicate.
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("tracker.json"),
        br#"[
            {"id":1,"title":"First","status":"open","priority":"medium","labels":[],"created_at":"2026-01-01T00:00:00Z","updated_at":"2026-01-01T00:00:00Z"},
            {"id":1,"title":"Duplicate","status":"open","priority":"medium","labels":[],"created_at":"2026-01-01T00:00:00Z","updated_at":"2026-01-01T00:00:00Z"}
        ]"#,
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
fn empty_label_in_json_causes_error_exit() {
    // DESIGN.md edge case: empty/whitespace labels rejected at create. Stored
    // data with an empty label is equally corrupt — without this check, the
    // labels column renders an empty string between commas.
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("tracker.json"),
        br#"[{"id":1,"title":"x","status":"open","priority":"medium","labels":["bug",""],"created_at":"2026-01-01T00:00:00Z","updated_at":"2026-01-01T00:00:00Z"}]"#,
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
fn malformed_timestamp_in_json_causes_error_exit() {
    // DESIGN.md Data Model: created_at/updated_at are ISO 8601 UTC strings.
    // A non-parseable timestamp violates the schema.
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("tracker.json"),
        br#"[{"id":1,"title":"x","status":"open","priority":"medium","labels":[],"created_at":"yesterday","updated_at":"2026-01-01T00:00:00Z"}]"#,
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
fn updated_before_created_in_json_causes_error_exit() {
    // DESIGN.md Data Model: "updated_at always >= created_at" — a stored
    // record violating this invariant is corrupt regardless of how it got there.
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("tracker.json"),
        br#"[{"id":1,"title":"x","status":"open","priority":"medium","labels":[],"created_at":"2026-05-01T00:00:00Z","updated_at":"2026-04-30T23:59:59Z"}]"#,
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
#[cfg(unix)]
fn list_does_not_panic_on_broken_pipe() {
    // Without SIGPIPE-default handling, Rust's runtime panics when `println!`
    // hits a closed pipe — emitting a backtrace on stderr and exiting 101.
    // That violates DESIGN.md's `Error:` stderr contract and exit-{0,1} set.
    // Reproduced by piping `tracker list` to a reader that closes early.
    use std::fmt::Write as _;
    use std::process::{Command, Stdio};

    let dir = TempDir::new().unwrap();
    // Build a tracker.json large enough to overflow the OS pipe buffer
    // (64 KiB Linux, 16 KiB macOS), so the writer is still emitting when the
    // reader closes. ~600 rows × ~120 bytes ≈ 70 KiB of list output.
    let mut json = String::from("[");
    for i in 1..=600u64 {
        if i > 1 {
            json.push(',');
        }
        write!(
            json,
            r#"{{"id":{i},"title":"Issue {i:04}","status":"open","priority":"medium","labels":[],"created_at":"2026-01-01T00:00:00Z","updated_at":"2026-01-01T00:00:00Z"}}"#
        )
        .unwrap();
    }
    json.push(']');
    fs::write(dir.path().join("tracker.json"), json.as_bytes()).unwrap();

    let bin = assert_cmd::cargo::cargo_bin("tracker");
    let mut child = Command::new(&bin)
        .arg("list")
        .current_dir(dir.path())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    // Close the read end of stdout before the writer finishes.
    drop(child.stdout.take());

    let output = child.wait_with_output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("panicked"),
        "broken pipe must not panic; stderr was:\n{stderr}"
    );
    assert_ne!(
        output.status.code(),
        Some(101),
        "exit 101 indicates a Rust panic; stderr was:\n{stderr}"
    );
}

#[test]
fn u64_max_id_in_json_blocks_next_create_with_clean_error() {
    // Hand-edited tracker.json plants id: u64::MAX. On the next create, the
    // unguarded `next_id` would overflow (debug: panic; release: wrap to 0,
    // bricking the tracker). With `checked_add`, the user sees a clean error
    // and the file is unchanged.
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("tracker.json"),
        format!(
            r#"[{{"id":{},"title":"sentinel","status":"open","priority":"medium","labels":[],"created_at":"2026-01-01T00:00:00Z","updated_at":"2026-01-01T00:00:00Z"}}]"#,
            u64::MAX
        )
        .as_bytes(),
    )
    .unwrap();
    tracker(&dir)
        .args(["create", "Should fail"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains(
            "Cannot assign new issue ID: maximum ID reached.",
        ))
        .stdout("");
}

// --- clap error contract: DESIGN.md stderr requires "Error:" prefix and exit code 1 ---

#[test]
fn unknown_subcommand_uses_capital_error_prefix_and_exits_one() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["bogus_subcommand"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::starts_with("Error:"));
}

#[test]
fn missing_required_arg_uses_capital_error_prefix_and_exits_one() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::starts_with("Error:"));
}

// --- spec-literal stdout assertions (DESIGN.md "stdout prints exactly: ...") ---

#[test]
fn create_stdout_uses_trimmed_title_not_raw() {
    // DESIGN.md Feature 1 postcondition: "stdout prints exactly: `Created issue #<id>: <title>` (trimmed title)".
    // A regression that printed `title_raw` (untrimmed) would still pass create_trims_title (which only inspects JSON).
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "  Fix bug  "])
        .assert()
        .success()
        .stdout("Created issue #1: Fix bug\n")
        .stderr("");
}

// --- DESIGN.md Storage edge case: tracker.json is a directory ---

#[test]
fn tracker_json_is_a_directory_causes_io_error_exit() {
    // DESIGN.md Edge Cases > Storage: "tracker.json is a directory → read error,
    // treated as I/O failure → exit 1". Cross-platform-testable via fs::create_dir.
    // Without this test, a regression that panicked on EISDIR (e.g., .unwrap()) would pass the suite.
    let dir = TempDir::new().unwrap();
    fs::create_dir(dir.path().join("tracker.json")).unwrap();
    tracker(&dir)
        .args(["list"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::starts_with(
            "Error: Could not read tracker data",
        ))
        .stdout("");
}

// --- CLI supplement: --help exits 0 cleanly ---

#[test]
fn help_flag_exits_zero_and_lists_subcommands() {
    // DESIGN.md Interface: "--help is supported for the binary and each subcommand."
    // QE Testing Methodology calls out: "--help output: verify it does not crash and exits 0".
    // No prior test covered this contract — a regression in main.rs's clap-error routing
    // (e.g., treating --help as an error path → exit 1) would be silently introduced.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("status"));
}

#[test]
fn subcommand_help_flag_exits_zero() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--priority"));
}
