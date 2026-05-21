//! CLI entry point for bookmark-cli.
//!
//! Effectful shell — dispatches to the pure-core in `lib.rs`. Per the
//! error contract in `DESIGN.md` § Exit codes:
//!   - exit 0: success (including empty `bm list`)
//!   - exit 1: user error (empty URL — both `bm add ""` and `bm add` with no positional)
//!   - exit 2: storage error (read/write/parse failure)
//!   - exit 64: CLI usage error (`EX_USAGE` per `sysexits.h` — unknown subcommand, unknown flag)
//!
//! Per the [Rust supplement](../../vsdd-suite/supplements/rust.md) § Security,
//! every user-derived value (storage path, captured URLs) is wrapped in
//! `bookmark_cli::display_safe` before reaching `eprintln!` / `println!` to
//! prevent terminal-escape injection — see [Security Review 1
//! Finding 1](../vsdd-suite/review-log/2026-05-20-security.md) +
//! [Red Team Review 1
//! Finding 4](../vsdd-suite/review-log/2026-05-20-red-team.md).

#![deny(unsafe_code)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use bookmark_cli::{display_safe, BookmarkStore};
use clap::error::ErrorKind;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(
    name = "bm",
    version,
    about = "Capture URLs at the terminal; recall them later.",
    long_about = "Capture URLs at the terminal; recall them later.\n\
                  \n\
                  Single-user local tool. Bookmarks are stored as a flat JSON file at\n\
                  $BOOKMARK_CLI_DB (default: ./bookmarks.json). See README.md and\n\
                  DESIGN.md for full behavioral contract.\n\
                  \n\
                  Examples:\n  \
                    bm add https://example.com   # capture a URL with current UTC timestamp\n  \
                    bm list                       # print bookmarks, newest-first\n  \
                    bm --help                     # show this help text\n  \
                    bm --version                  # show version\n\
                  \n\
                  Exit codes:\n  \
                    0   success (including empty `bm list`)\n  \
                    1   user error (empty URL or missing positional)\n  \
                    2   storage error (file unreadable, corrupt JSON, write failure)\n  \
                    64  CLI usage error (unknown subcommand, unknown flag)"
)]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Add a bookmark with the current UTC timestamp.
    Add {
        /// The URL to capture. Must be non-empty.
        url: String,
    },
    /// List bookmarks, newest-first.
    List,
}

fn store_path() -> PathBuf {
    std::env::var_os("BOOKMARK_CLI_DB")
        .map_or_else(|| PathBuf::from("bookmarks.json"), PathBuf::from)
}

/// Emit a storage error to stderr with a remediation hint per the UX
/// supplement § Storage-error remediation discipline. Closes
/// [UX Review 1 Finding 5](../vsdd-suite/review-log/2026-05-20-ux.md) —
/// the Round 1 error messages emitted the anyhow chain alone, leaving the
/// user without an actionable next step. The hint is generic enough to
/// apply to all three storage-error origins (load / add / save) without
/// false specificity, but concrete enough to point the user at the most
/// likely diagnostic action.
fn emit_storage_error(e: &anyhow::Error, kind: &str) {
    eprintln!("Error: {}", display_safe(&format!("{e:#}")));
    let path = store_path();
    eprintln!(
        "Hint: check that the storage file at {} {}",
        display_safe(&path.display().to_string()),
        match kind {
            // Load Hint covers BOTH the filesystem-cause cases AND the
            // JSON-content cause — the most common failure after first
            // successful use is corrupt JSON, not a missing/permission
            // failure. Round 3 UX Finding 7.
            "load" =>
                "exists, is a readable regular file (not a symlink), AND contains valid JSON \
                 conforming to the store shape — try `cat <path>` to inspect the contents.",
            "save" => "is writable, its parent directory exists, and the path is not a symlink.",
            _ => "is accessible to the current user.",
        }
    );
    eprintln!("Hint: $BOOKMARK_CLI_DB overrides the default path; verify the env var if set.");
}

/// Treat a clap parse error as either an empty-URL rejection (exit 1) or
/// a generic usage error (exit 64) per `DESIGN.md` § Exit codes. Closes
/// [SE Review 1 Finding 1](../vsdd-suite/review-log/2026-05-20-software-engineer.md#r1-f1)
/// + [SE Review 1 Finding 3](../vsdd-suite/review-log/2026-05-20-software-engineer.md#r1-f3).
fn handle_parse_error(err: &clap::Error) -> ExitCode {
    // `--help` and `--version` surface as `Err` from `try_parse` but are not
    // usage errors — clap prints the help/version text to stdout and the
    // contract is exit 0. The help/version rendering is from clap's own
    // strings (no user-controlled bytes), so emitting through
    // `clap::Error::print` directly is acceptable. Closes Round 2 SE
    // Finding 6 (regression from the F1/F3 fix that routed every
    // parse-error through ExitCode::from(64)).
    if matches!(
        err.kind(),
        ErrorKind::DisplayHelp | ErrorKind::DisplayVersion
    ) {
        let _ = err.print();
        return ExitCode::SUCCESS;
    }

    if err.kind() == ErrorKind::MissingRequiredArgument {
        // clap names the missing argument in the error's context. The
        // only required positional in our grammar is `<url>` on `bm add`;
        // any MissingRequiredArgument is therefore that case in scope.
        // We emit the spec-contracted message + exit 1.
        let missing = err
            .get(clap::error::ContextKind::InvalidArg)
            .map(|v| format!("{v}"));
        if matches!(missing.as_deref(), Some(m) if m.contains("<URL>") || m.contains("<url>") || m.contains("URL"))
        {
            eprintln!("Error: URL cannot be empty.");
            return ExitCode::from(1);
        }
    }
    // Default clap path — render clap's formatted message + emit through
    // `display_safe` to escape any control / format bytes that arrived from
    // argv (e.g., `bm $'\x1b[31mfrobnicate'` unknown-subcommand path). clap
    // quotes the offending argument literally in its error message; without
    // sanitization, ANSI escapes + bidi format chars reach the operator's
    // terminal raw. Closes [Security Review 1 Round 2
    // Finding 4](../vsdd-suite/review-log/2026-05-20-security.md#r2-f4).
    let rendered = err.render().to_string();
    eprint!("{}", display_safe(&rendered));
    ExitCode::from(64)
}

fn main() -> ExitCode {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(err) => return handle_parse_error(&err),
    };
    let path = store_path();

    match cli.command {
        Cmd::Add { url } => {
            if url.is_empty() {
                eprintln!("Error: URL cannot be empty.");
                return ExitCode::from(1);
            }
            let mut store = match BookmarkStore::load(&path) {
                Ok(s) => s,
                Err(e) => {
                    emit_storage_error(&e, "load");
                    return ExitCode::from(2);
                }
            };
            if let Err(e) = store.add(url) {
                // Library invariant rejection — empty URL is the only
                // case at present, which is also screened above; preserve
                // the spec message for that path (no hint — this is user
                // input, not storage).
                eprintln!("Error: {}", display_safe(&format!("{e:#}")));
                return ExitCode::from(1);
            }
            if let Err(e) = store.save(&path) {
                emit_storage_error(&e, "save");
                return ExitCode::from(2);
            }
            ExitCode::SUCCESS
        }
        Cmd::List => {
            let store = match BookmarkStore::load(&path) {
                Ok(s) => s,
                Err(e) => {
                    emit_storage_error(&e, "load");
                    return ExitCode::from(2);
                }
            };
            if store.bookmarks().is_empty() {
                eprintln!("No bookmarks yet.");
                return ExitCode::SUCCESS;
            }
            for bm in store.newest_first() {
                println!("{} {}", bm.timestamp().to_rfc3339(), display_safe(bm.url()));
            }
            ExitCode::SUCCESS
        }
    }
}
