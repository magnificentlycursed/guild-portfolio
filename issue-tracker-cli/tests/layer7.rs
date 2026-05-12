// Layer 7 — Polish: --help, color output, error specificity.
//
// Red Gate notes (Phase 2a):
//
// Layer 7 is a polish layer. clap's default `--help` plumbing and the Layer 1
// `try_parse` transform in `src/main.rs` already satisfy most of the
// `--help` / unknown-subcommand / `Error:` acceptance criteria *against
// current code* — the tests below pass against `main` before any Phase 2b
// work. They are nevertheless valid Layer 7 Red Gate tests because they pin
// the help/error *contract* (valid-value enumerations, exit codes, stderr
// routing) that prior layers established only by convention; a future
// refactor that drops the `--priority <low|medium|high>` enumeration from
// `create --help`, or that lets an unknown subcommand exit 0, would now
// fail a named test rather than silently regress.
//
// The genuinely new Layer 7 behavior — TTY-detected color output for
// `priority` and `status` values, with ANSI codes suppressed when stdout is
// piped — is testable only on the piped side from a subprocess test
// (TempDir + `assert_cmd` produces a non-TTY stdout by construction). The
// `list_piped_has_no_ansi_codes` and `show_piped_has_no_ansi_codes` tests
// below pass trivially against pre-color code and become real regression
// guards once color is added in Phase 2b: a naive `println!("\x1b[31m...")`
// without TTY detection would break them. TTY-positive rendering is
// covered by the manual checklist in TODO.md per the layer plan
// ("Manual only (TTY-detection cannot be automated in subprocess tests)").

use predicates::prelude::*;
use tempfile::TempDir;

mod common;
use common::tracker;

// --- --help exit code and content ---

#[test]
fn help_flag_binary_exits_zero() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage: tracker"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("status"))
        .stdout(predicate::str::contains("show"))
        .stdout(predicate::str::contains("delete"));
}

#[test]
fn help_flag_create_exits_zero() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--description"))
        .stdout(predicate::str::contains("--priority"))
        .stdout(predicate::str::contains("--label"))
        .stdout(predicate::str::contains("low, medium, high"));
}

#[test]
fn help_flag_list_exits_zero() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--status"))
        .stdout(predicate::str::contains("--priority"))
        .stdout(predicate::str::contains("--label"))
        .stdout(predicate::str::contains("open, in-progress, done"))
        .stdout(predicate::str::contains("low, medium, high"));
}

#[test]
fn help_flag_status_exits_zero() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["status", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("<ID>"))
        .stdout(predicate::str::contains("<STATUS>"))
        .stdout(predicate::str::contains("open, in-progress, done"));
}

#[test]
fn help_flag_show_exits_zero() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["show", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("<ID>"));
}

#[test]
fn help_flag_delete_exits_zero() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["delete", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("<ID>"));
}

// --- unknown subcommand ---

#[test]
fn unknown_subcommand_exits_one() {
    // Round 2 (QE R17 F3): assert stdout is empty — clap error output must
    // route to stderr per the DESIGN.md stderr contract.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["frobnicate"])
        .assert()
        .failure()
        .code(1)
        .stdout("")
        .stderr(predicate::str::contains("Error:"))
        .stderr(predicate::str::contains("unrecognized subcommand"))
        .stderr(predicate::str::contains("frobnicate"));
}

#[test]
fn unknown_subcommand_with_cc_payload_escapes_in_stderr() {
    // Round 2 (RT R10 F1 — extended DESIGN.md stderr contract): user-
    // supplied bytes reflected by clap's `unrecognized subcommand '<name>'`
    // error MUST be Cc-escaped before reaching stderr. Plant CR + TAB in
    // the subcommand name and assert each appears as its `\u{XX}` escape
    // per `sanitize_quoted_values`. Structural LFs from clap's multi-line
    // error template (`\n\nUsage: ...`) survive because the sanitizer
    // narrows escaping to the inside of single-quoted regions only.
    //
    // ESC (`\x1B`) is excluded from the payload — clap's own error
    // pipeline strips raw ESC bytes from reflected values before they
    // reach our `sanitize_quoted_values` transform, so we cannot assert
    // on what we never receive. The defense remains in place via clap's
    // upstream sanitization; our test pins our own sanitizer's behavior.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["pre\rmid\ttab"])
        .assert()
        .failure()
        .code(1)
        .stdout("")
        .stderr(predicate::str::contains("\\u{D}")) // CR escaped inside the quoted value
        .stderr(predicate::str::contains("\\u{9}")) // TAB escaped inside the quoted value
        .stderr(predicate::str::contains("\n\nUsage:")); // Structural LFs preserved
}

// --- no ANSI escape codes when stdout is piped ---
//
// `assert_cmd::Command` invokes the binary with stdout connected to a pipe
// (non-TTY), so these tests exercise the piped branch of the TTY-detection
// logic added in Phase 2b. They pass against pre-color code (nothing emits
// ANSI yet) and serve as regression guards against a naive Phase 2b
// implementation that emits color unconditionally.

#[test]
fn list_piped_has_no_ansi_codes() {
    let dir = TempDir::new().unwrap();
    // Seed enough issues to exercise every priority and every status value
    // so a naive "always colorize" implementation would necessarily emit at
    // least one ANSI escape into the captured stdout.
    tracker(&dir)
        .args(["create", "High", "--priority", "high"])
        .assert()
        .success();
    tracker(&dir)
        .args(["create", "Medium", "--priority", "medium"])
        .assert()
        .success();
    tracker(&dir)
        .args(["create", "Low", "--priority", "low"])
        .assert()
        .success();
    tracker(&dir)
        .args(["status", "1", "in-progress"])
        .assert()
        .success();
    tracker(&dir)
        .args(["status", "2", "done"])
        .assert()
        .success();

    // Round 2 (QE R17 F4): assert ALSO that stderr is ANSI-clean — empty-
    // state messages from `list` route to stderr per DESIGN.md, and the
    // color contract is symmetric (no ANSI to stderr regardless of TTY).
    for st in ["open", "in-progress", "done"] {
        tracker(&dir)
            .args(["list", "--status", st])
            .assert()
            .success()
            .stdout(predicate::str::contains("\x1b[").not())
            .stderr(predicate::str::contains("\x1b[").not());
    }
}

#[test]
fn list_empty_state_stderr_has_no_ansi_codes() {
    // Round 2 (QE R17 F4): when `list` produces an empty-state message on
    // stderr, that stderr stream must also be ANSI-clean — color
    // suppression applies symmetrically across stdout and stderr.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["list"])
        .assert()
        .success()
        .stdout("")
        .stderr(predicate::str::contains("No open issues. Nice work!"))
        .stderr(predicate::str::contains("\x1b[").not());
}

#[test]
fn show_piped_has_no_ansi_codes() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Important", "--priority", "high"])
        .assert()
        .success();
    tracker(&dir)
        .args(["status", "1", "in-progress"])
        .assert()
        .success();

    // Round 2 (QE R17 F4): assert stderr is ANSI-clean too.
    tracker(&dir)
        .args(["show", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\x1b[").not())
        .stderr(predicate::str::contains("\x1b[").not());
}

#[test]
fn no_color_env_does_not_break_piped_invocation() {
    // Round 2 (UX R10 F1 / Security R11 F2 / Round-2 DESIGN.md amendment):
    // NO_COLOR is honored. assert_cmd pipes stdout (non-TTY), so color is
    // already suppressed by TTY-detection alone; this test verifies the
    // env-var path doesn't crash, alter exit code, or change output shape.
    // Real TTY-positive verification of NO_COLOR is in the Layer 7 manual
    // testing checklist (TODO.md: forthcoming Round-2 amendment item).
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Test", "--priority", "high"])
        .assert()
        .success();
    for (k, v) in [
        ("NO_COLOR", "1"),
        ("CLICOLOR", "0"),
        ("CLICOLOR_FORCE", "1"),
    ] {
        tracker(&dir)
            .env(k, v)
            .args(["list"])
            .assert()
            .success()
            .stdout(predicate::str::contains("\x1b[").not())
            .stdout(predicate::str::contains("Test"));
    }
}

// --- TTY-positive color rendering via TRACKER_INTERNAL_FORCE_COLOR test seam ---
//
// QE Review 17 Finding 1 closure: the 4 TTY-positive Layer 7 ACs (color
// emission for high/medium/in-progress/done values) previously had zero
// automated coverage — `assert_cmd::Command` pipes stdout, so the TTY
// check in `color_mode_from_env` returns Off and no ANSI is emitted.
// `TRACKER_INTERNAL_FORCE_COLOR=1` bypasses the TTY check (test seam
// only, not part of the public CLI contract; see `color_mode_from_env`
// doc-comment). The tests below set the seam and assert the exact ANSI
// sequences the DESIGN.md "Interface / Color output" table specifies,
// closing the mutation-resilience gap that R1 + R2 retrofit unit tests
// did not cover at the integration boundary.

#[test]
fn force_color_emits_bold_red_for_high_priority() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Urgent", "--priority", "high"])
        .assert()
        .success();
    tracker(&dir)
        .env("TRACKER_INTERNAL_FORCE_COLOR", "1")
        .env_remove("NO_COLOR")
        .env_remove("CLICOLOR")
        .args(["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\x1b[1;31mhigh\x1b[0m"));
}

#[test]
fn force_color_emits_bold_yellow_for_medium_priority() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Routine", "--priority", "medium"])
        .assert()
        .success();
    tracker(&dir)
        .env("TRACKER_INTERNAL_FORCE_COLOR", "1")
        .env_remove("NO_COLOR")
        .env_remove("CLICOLOR")
        .args(["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\x1b[1;33mmedium\x1b[0m"));
}

#[test]
fn force_color_does_not_color_low_priority() {
    // `low` is the default-color value per spec; no ANSI even when forced.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Maybe", "--priority", "low"])
        .assert()
        .success();
    tracker(&dir)
        .env("TRACKER_INTERNAL_FORCE_COLOR", "1")
        .env_remove("NO_COLOR")
        .env_remove("CLICOLOR")
        .args(["list"])
        .assert()
        .success()
        // The status column for the open issue must be colorless ("open"
        // is also a default-color value) AND the priority cell must not
        // wrap "low" in any ANSI sequence. Asserting no `\x1b[` at all is
        // tighter than asserting on a specific sequence absence.
        .stdout(predicate::str::contains("\x1b[").not());
}

#[test]
fn force_color_emits_bold_cyan_for_in_progress_status() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Working", "--priority", "low"])
        .assert()
        .success();
    tracker(&dir)
        .args(["status", "1", "in-progress"])
        .assert()
        .success();
    tracker(&dir)
        .env("TRACKER_INTERNAL_FORCE_COLOR", "1")
        .env_remove("NO_COLOR")
        .env_remove("CLICOLOR")
        .args(["list", "--status", "in-progress"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\x1b[1;36min-progress\x1b[0m"));
}

#[test]
fn force_color_emits_bold_green_for_done_status() {
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Finished", "--priority", "low"])
        .assert()
        .success();
    tracker(&dir)
        .args(["status", "1", "done"])
        .assert()
        .success();
    tracker(&dir)
        .env("TRACKER_INTERNAL_FORCE_COLOR", "1")
        .env_remove("NO_COLOR")
        .env_remove("CLICOLOR")
        .args(["list", "--status", "done"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\x1b[1;32mdone\x1b[0m"));
}

#[test]
fn force_color_does_not_color_header_row() {
    // DESIGN.md "Interface / Color output": color applies only to the
    // value text in its column cell, NOT to the row or header. A mutation
    // that colored the header (`ID Status Priority Labels Title`) would
    // be a real regression.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Bug", "--priority", "high"])
        .assert()
        .success();
    let output = tracker(&dir)
        .env("TRACKER_INTERNAL_FORCE_COLOR", "1")
        .env_remove("NO_COLOR")
        .env_remove("CLICOLOR")
        .args(["list"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let out = String::from_utf8(output).unwrap();
    let lines: Vec<&str> = out.lines().collect();
    // First line is the header row; must contain no ANSI escapes.
    assert!(
        !lines[0].contains("\x1b["),
        "header row must be uncolored, got: {:?}",
        lines[0]
    );
    // Header row literal content sanity check (helps diagnose if the
    // ordering changes — the column order is part of the AC contract).
    assert!(lines[0].contains("ID"), "header missing ID column");
    assert!(lines[0].contains("Status"), "header missing Status column");
    assert!(
        lines[0].contains("Priority"),
        "header missing Priority column"
    );
}

#[test]
fn force_color_data_row_emits_columns_in_status_then_priority_order() {
    // Round 3 (QE R19 F1 closure): the prior `force_color_emits_*` tests
    // used unanchored `contains` assertions; a `format_list_row` mutation
    // that swapped the status and priority positional args in the format
    // string would not break those tests because both colored bytes
    // would still be `contains`-present somewhere in stdout. This test
    // pins the COLUMN ORDER specifically by asserting that the
    // status-cell ANSI sequence appears BEFORE the priority-cell ANSI
    // sequence in the same row. A swap mutation flips this ordering.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Bug", "--priority", "high"])
        .assert()
        .success();
    tracker(&dir)
        .args(["status", "1", "in-progress"])
        .assert()
        .success();
    let output = tracker(&dir)
        .env("TRACKER_INTERNAL_FORCE_COLOR", "1")
        .env_remove("NO_COLOR")
        .env_remove("CLICOLOR")
        .args(["list", "--status", "in-progress"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let out = String::from_utf8(output).unwrap();
    // Find the data row (lines[1] — first row after header).
    let lines: Vec<&str> = out.lines().collect();
    let data_row = lines
        .get(1)
        .expect("expected at least one data row after the header");
    let cyan_pos = data_row
        .find("\x1b[1;36m")
        .expect("expected bold-cyan in-progress ANSI in data row");
    let red_pos = data_row
        .find("\x1b[1;31m")
        .expect("expected bold-red high-priority ANSI in data row");
    assert!(
        cyan_pos < red_pos,
        "Status column must appear before Priority column. Got cyan(status) at byte {cyan_pos} \
         and red(priority) at byte {red_pos} in row: {data_row:?}"
    );
}

#[test]
fn force_color_show_renders_colored_status_and_priority_value_cells() {
    // The `show` subcommand renders one issue per call as a labelled
    // key-value block. Color applies to the status / priority *value*
    // cells; the label column (`Status:      `, `Priority:    `) and
    // every other line must be uncolored.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Important", "--priority", "high"])
        .assert()
        .success();
    tracker(&dir)
        .args(["status", "1", "in-progress"])
        .assert()
        .success();
    let output = tracker(&dir)
        .env("TRACKER_INTERNAL_FORCE_COLOR", "1")
        .env_remove("NO_COLOR")
        .env_remove("CLICOLOR")
        .args(["show", "1"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let out = String::from_utf8(output).unwrap();
    // Status line: label uncolored, value bold-cyan.
    assert!(
        out.contains("Status:      \x1b[1;36min-progress\x1b[0m"),
        "expected colored status value after uncolored label:\n{out}"
    );
    // Priority line: label uncolored, value bold-red.
    assert!(
        out.contains("Priority:    \x1b[1;31mhigh\x1b[0m"),
        "expected colored priority value after uncolored label:\n{out}"
    );
    // ID / Title / Labels / Description / Created / Updated lines must
    // not carry ANSI (color is value-only, on status/priority only).
    let lines: Vec<&str> = out.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        if line.starts_with("ID:")
            || line.starts_with("Title:")
            || line.starts_with("Labels:")
            || line.starts_with("Description:")
            || line.starts_with("Created:")
            || line.starts_with("Updated:")
        {
            assert!(
                !line.contains("\x1b["),
                "line {} ({:?}) must not contain ANSI escapes",
                i,
                line
            );
        }
    }
}

#[test]
fn force_color_wins_over_no_color_when_both_env_vars_set() {
    // Pins the precedence ordering: TRACKER_INTERNAL_FORCE_COLOR is
    // checked BEFORE NO_COLOR in `color_mode_from_env`, so the test seam
    // wins when both are set. Renamed at Round 3 closure (QE R19 F2) —
    // the prior name `force_color_with_no_color_env_set_does_not_force`
    // read contradictorily against its assertion. The new name names the
    // actual behavior (force-color WINS over NO_COLOR in this precedence
    // ordering).
    //
    // Rationale for the precedence: the seam is purely test-only; tests
    // fully control their env. If NO_COLOR is inherited from CI, tests
    // intending to exercise the colored-output path explicitly
    // `.env_remove("NO_COLOR")` (see other force_color_* tests in this
    // file). Production users do not set TRACKER_INTERNAL_FORCE_COLOR
    // (it's not documented anywhere user-facing), so the precedence
    // ordering has no production effect.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Test", "--priority", "high"])
        .assert()
        .success();
    tracker(&dir)
        .env("TRACKER_INTERNAL_FORCE_COLOR", "1")
        .env("NO_COLOR", "1")
        .env_remove("CLICOLOR")
        .args(["list"])
        .assert()
        .success()
        // Force-color wins; bold-red ANSI for high priority emits despite
        // NO_COLOR being set.
        .stdout(predicate::str::contains("\x1b[1;31m"));
}
