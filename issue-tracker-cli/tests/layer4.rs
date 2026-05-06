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
    // Negative `Nice work!` assertion mirrors `list_priority_filter_no_match_shows_filter_message`
    // (QE Review 9 F1): kills any mutation that drops `label_filter.is_none()` from the
    // `is_default_open_view` heuristic in `cmd_list` — without it, `tracker list --label X`
    // with no matches would print "No open issues. Nice work!" instead of the filter message.
    tracker(&dir)
        .args(["list", "--label", "Bug"])
        .assert()
        .success()
        .stdout("")
        .stderr(predicate::str::contains(
            "No issues match the given filters.",
        ))
        .stderr(predicate::str::contains("Nice work!").not());
}

#[test]
fn create_preserves_label_case_at_storage() {
    // DESIGN.md Feature 1 postcondition (line 28): "labels is the deduplicated list of
    // --label values; order is preserved, case is preserved as provided".
    // A mutation in `parse_label` that lowercased its input (e.g., `trimmed.to_lowercase()`)
    // would survive every other Layer 4 test — `create_with_label_stores_label` and the
    // multi-label / dedup tests all use lowercase inputs; `list_label_filter_is_case_sensitive`
    // checks the filter side. Case preservation at storage was untested.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args([
            "create", "x", "--label", "Bug", "--label", "BUG", "--label", "bug",
        ])
        .assert()
        .success();

    let raw = fs::read_to_string(dir.path().join("tracker.json")).unwrap();
    let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(
        v[0]["labels"],
        serde_json::json!(["Bug", "BUG", "bug"]),
        "case must be preserved as provided; case-distinct labels are not deduplicated"
    );
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

    // Strengthened from `contains("Error:")` (so loose any failure path satisfied it) to
    // assert the actual clap error message. Kills mutations that route to a different
    // error sink (e.g., a generic "Error: invalid argument") and verifies the offending
    // flag name appears in the message — user-actionable diagnostic per CLI supplement Dim 8.
    tracker(&dir)
        .args(["list", "--label", "bug", "--label", "auth"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains(
            "Error: the argument '--label <LABEL>' cannot be used multiple times",
        ))
        .stdout("");
}

// --- Round 2: label control-character and comma defenses ---

#[test]
fn create_with_control_char_label_exits_one() {
    // Security R7 F1 / RT R6 F1 / DE R7 F1: a label containing a newline
    // breaks the spec's one-issue-per-line `list` contract; ESC enables
    // terminal-escape injection. Round-2 fix mirrors the title defense.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Real", "--label", "bug\nFAKE"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains(
            "Error: Label cannot contain control characters.",
        ))
        .stdout("");
}

#[test]
fn create_with_escape_sequence_label_exits_one() {
    // OSC 8 / ANSI CSI in labels — ESC is category Cc and is rejected by the
    // same rule that protects titles. Round-2 fix.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Real", "--label", "\u{1B}[31mEvil\u{1B}[0m"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains(
            "Error: Label cannot contain control characters.",
        ))
        .stdout("");
}

#[test]
fn create_with_comma_label_exits_one() {
    // UX R6 F4: the `Labels` column joins values with `, ` for display, so a
    // label that itself contains `,` makes the display ambiguous. Reject at
    // input. Round-2 fix.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Real", "--label", "a,b"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains(
            "Error: Label cannot contain a comma.",
        ))
        .stdout("");
}

#[test]
fn corrupt_data_with_control_char_label_is_rejected() {
    // Load-path corollary of the create-time check (Security R7 F1 / RT R6 F1
    // load-path attack): a hand-edited tracker.json with a control char in a
    // label must be rejected as corrupt at load time. issue_fields_are_valid
    // enforces the same rule on stored data that parse_label enforces on input.
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("tracker.json");
    fs::write(
        &path,
        r#"[{"id":1,"title":"Real","status":"open","priority":"medium","labels":["bug\nfake"],"created_at":"2026-01-01T00:00:00Z","updated_at":"2026-01-01T00:00:00Z"}]"#,
    )
    .unwrap();

    tracker(&dir)
        .args(["list"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains(
            "Could not read tracker data. The file may be corrupt.",
        ))
        .stdout("");
}

#[test]
fn corrupt_data_with_comma_label_is_rejected() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("tracker.json");
    fs::write(
        &path,
        r#"[{"id":1,"title":"Real","status":"open","priority":"medium","labels":["a,b"],"created_at":"2026-01-01T00:00:00Z","updated_at":"2026-01-01T00:00:00Z"}]"#,
    )
    .unwrap();

    tracker(&dir)
        .args(["list"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains(
            "Could not read tracker data. The file may be corrupt.",
        ))
        .stdout("");
}

// --- Round 2: list filter symmetry with create-side validation ---

#[test]
fn list_label_filter_is_trimmed_to_match_stored() {
    // UX R6 F1 / SO R16 F1: a stored label is trimmed at create time, so the
    // filter side must trim too — otherwise `tracker list --label "  bug  "`
    // silently no-matches a stored `bug`. Round-2 fix: parse_label runs on the
    // filter value as well.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "X", "--label", "bug"])
        .assert()
        .success();

    tracker(&dir)
        .args(["list", "--label", "  bug  "])
        .assert()
        .success()
        .stdout(predicate::str::contains("bug").and(predicate::str::contains("X")));
}

#[test]
fn list_empty_label_filter_exits_one() {
    // SO R16 F2 / UX R6 F1: empty/whitespace-only filter must be rejected
    // symmetric with the create-side rule, not silently no-matched. Round-2
    // fix: cmd_list runs parse_label on the filter, surfacing the same error.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["list", "--label", ""])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Error: Label cannot be empty."))
        .stdout("");
}

#[test]
fn list_whitespace_label_filter_exits_one() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["list", "--label", "   "])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Error: Label cannot be empty."))
        .stdout("");
}

#[test]
fn list_control_char_label_filter_exits_one() {
    // Defense in depth: filter values are now validated by parse_label too,
    // so a control char in the filter rejects symmetrically with the create
    // side. (RT R6 F4 dismissed-the-original concern; this test makes the
    // post-Round-2 validation explicit.)
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["list", "--label", "bug\nfake"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains(
            "Error: Label cannot contain control characters.",
        ))
        .stdout("");
}

// --- Round 2: error-message escape-interpolation defense (RT R6 F2) ---

#[test]
fn invalid_priority_with_escape_chars_is_escaped_in_error() {
    // RT R6 F2: parse_priority interpolated raw user input into the error
    // message, so `--priority $'\x1b[31mPWN\x1b[0m'` rendered as red text on
    // stderr. Round-2 fix: display_safe escapes Cc chars as \u{XX}.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["list", "--priority", "\u{1B}[31mPWN\u{1B}[0m"])
        .assert()
        .failure()
        .code(1)
        // The escaped form contains `\u{1B}` literally (six chars). The raw
        // ESC byte (0x1B) must NOT appear in the rendered error.
        .stderr(predicate::str::contains("\\u{1B}"))
        .stderr(predicate::str::contains("\u{1B}").not());
}

#[test]
fn invalid_status_with_newline_is_escaped_in_error() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["status", "1", "foo\nbar"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("\\u{A}"))
        // The whole error must be on a single line — no embedded raw newline.
        .stderr(predicate::str::contains("\nbar").not());
}

#[test]
fn invalid_id_with_escape_chars_is_escaped_in_error() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["status", "abc\u{1B}[31mEVIL\u{1B}[0m", "done"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("\\u{1B}"))
        .stderr(predicate::str::contains("\u{1B}").not());
}
