//! CLI entry point for bookmark-cli.
//!
//! Effectful shell — dispatches to the pure-core in `lib.rs`. Per the
//! error contract in `DESIGN.md`:
//!   - exit 0: success (including empty `bm list`)
//!   - exit 1: user error (empty URL)
//!   - exit 2: storage error (read/write/parse failure)

use bookmark_cli::BookmarkStore;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "bm", version, about = "Capture URLs at the terminal; recall them later.")]
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
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("bookmarks.json"))
}

fn main() -> ExitCode {
    let cli = Cli::parse();
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
                    eprintln!("Error: {e:#}");
                    return ExitCode::from(2);
                }
            };
            store.add(url);
            if let Err(e) = store.save(&path) {
                eprintln!("Error: {e:#}");
                return ExitCode::from(2);
            }
            ExitCode::SUCCESS
        }
        Cmd::List => {
            let store = match BookmarkStore::load(&path) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error: {e:#}");
                    return ExitCode::from(2);
                }
            };
            if store.bookmarks.is_empty() {
                eprintln!("No bookmarks yet.");
                return ExitCode::SUCCESS;
            }
            for bm in store.newest_first() {
                println!("{} {}", bm.timestamp.to_rfc3339(), bm.url);
            }
            ExitCode::SUCCESS
        }
    }
}
