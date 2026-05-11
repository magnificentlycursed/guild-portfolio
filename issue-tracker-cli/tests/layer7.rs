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
