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
    about = "Capture URLs at the terminal; recall them later."
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

/// Treat a clap parse error as either an empty-URL rejection (exit 1) or
/// a generic usage error (exit 64) per `DESIGN.md` § Exit codes. Closes
/// [SE Review 1 Finding 1](../vsdd-suite/review-log/2026-05-20-software-engineer.md#r1-f1)
/// + [SE Review 1 Finding 3](../vsdd-suite/review-log/2026-05-20-software-engineer.md#r1-f3).
fn handle_parse_error(err: &clap::Error) -> ExitCode {
    // `--help` and `--version` surface as `Err` from `try_parse` but are not
    // usage errors — clap prints the help/version text to stdout and the
    // contract is exit 0. Closes Round 2 SE Finding 6 (regression from the
    // F1/F3 fix that routed every parse-error through ExitCode::from(64)).
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
    // Default clap path — print clap's formatted message + exit 64
    // (EX_USAGE). We use eprint! because clap's rendered message already
    // ends with a newline.
    let _ = err.print();
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
                    eprintln!("Error: {}", display_safe(&format!("{e:#}")));
                    return ExitCode::from(2);
                }
            };
            if let Err(e) = store.add(url) {
                // Library invariant rejection — empty URL is the only
                // case at present, which is also screened above; preserve
                // the spec message for that path.
                eprintln!("Error: {}", display_safe(&format!("{e:#}")));
                return ExitCode::from(1);
            }
            if let Err(e) = store.save(&path) {
                eprintln!("Error: {}", display_safe(&format!("{e:#}")));
                return ExitCode::from(2);
            }
            ExitCode::SUCCESS
        }
        Cmd::List => {
            let store = match BookmarkStore::load(&path) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error: {}", display_safe(&format!("{e:#}")));
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
