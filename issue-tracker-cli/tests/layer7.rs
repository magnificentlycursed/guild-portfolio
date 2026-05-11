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
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["frobnicate"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Error:"))
        .stderr(predicate::str::contains("unrecognized subcommand"))
        .stderr(predicate::str::contains("frobnicate"));
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

    tracker(&dir)
        .args(["list", "--status", "open"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\x1b[").not());
    tracker(&dir)
        .args(["list", "--status", "in-progress"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\x1b[").not());
    tracker(&dir)
        .args(["list", "--status", "done"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\x1b[").not());
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

    tracker(&dir)
        .args(["show", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\x1b[").not());
}
