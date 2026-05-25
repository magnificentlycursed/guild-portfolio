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

use bookmark_cli::{
    display_safe, AttachTagError, BookmarkStore, ImportError, MAX_STDIN_BYTES_DEFAULT,
};
use clap::error::ErrorKind;
use clap::{ArgAction, Parser, Subcommand};
use std::io::Read;
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
                    bm add https://example.com           # capture a URL with current UTC timestamp\n  \
                    bm list                                # print bookmarks, newest-first\n  \
                    bm tag https://example.com rust        # attach a label to all matching bookmarks\n  \
                    bm list --tag rust                     # filter list by tag\n  \
                    bm list --tag rust --tag go            # OR-semantics across repeated --tag\n  \
                    bm --help                              # show this help text\n  \
                    bm --version                           # show version\n\
                  \n\
                  Exit codes:\n  \
                    0   success (including empty `bm list`)\n  \
                    1   user error (empty URL, empty tag label, or unknown URL on `bm tag`)\n  \
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
    ///
    /// Captures `<URL>` as a new bookmark record with the current UTC
    /// time as the `timestamp` field. The URL must be non-empty; any
    /// other string (including whitespace-only or arbitrarily long URLs)
    /// is accepted — the user is responsible for what they capture.
    /// Storage is the JSON file at `$BOOKMARK_CLI_DB` (default
    /// `./bookmarks.json`); writes are atomic + parent-dir-fsynced on Unix.
    ///
    /// Exits 0 on success (stdout silent). Exits 1 with `Error: URL
    /// cannot be empty.` if the URL is empty (also covers `bm add` with
    /// no positional). Exits 2 on storage error. Exits 64 on other CLI
    /// usage errors (unknown flag, etc.).
    Add {
        /// The URL to capture. Must be non-empty.
        url: String,
    },
    /// List bookmarks, newest-first. Optionally filter by tag(s).
    ///
    /// Without `--tag`, lists every bookmark in newest-first order, one
    /// per line, in `<RFC3339-timestamp> <url>` format. With one or
    /// more `--tag <label>` flags, lists only bookmarks whose `tags`
    /// field contains AT LEAST ONE of the supplied labels — repeated
    /// `--tag` composes with OR-semantics (a bookmark matches if it
    /// has ANY listed tag).
    ///
    /// Empty store: stderr `No bookmarks yet.` + exit 0. The empty-
    /// store empty-state takes precedence over the filter-empty-state
    /// even when `--tag` is supplied (a user with no bookmarks gets
    /// the more informative signal). Filter no-match: stderr
    /// `No bookmarks match the supplied filter.` + exit 0. Empty
    /// label (`bm list --tag ""`): stderr `Error: tag label cannot
    /// be empty.` + exit 1.
    List {
        /// Filter to bookmarks tagged with this label. Repeatable —
        /// repeated `--tag` is OR-semantics across labels per
        /// `DESIGN.md` § `bm list --tag <label>`.
        #[arg(long = "tag", action = ArgAction::Append)]
        tags: Vec<String>,
    },
    /// Attach a label to all bookmarks whose URL matches exactly. Idempotent.
    ///
    /// The URL is the identifier (not an index) — `bm tag <URL> <LABEL>`
    /// tags every bookmark with that exact URL string (case-sensitive).
    /// If two bookmarks share the same URL (append-only semantics
    /// permits this), both are tagged. Idempotent: a label already
    /// attached to a matching bookmark is not duplicated.
    ///
    /// On success, exits 0 with stdout silent and stderr
    /// `Tagged N bookmark(s).` (where N is the count of matching
    /// bookmarks; N >= 1 because zero matches is the error path).
    /// If no bookmark has the URL: exit 1 with stderr `Error: no
    /// bookmark found with URL <url>.` (typos surface as user-errors;
    /// silent no-op would mask them). Empty URL or empty label:
    /// exit 1 with the corresponding `Error: ...` message. Exits 2
    /// on storage error.
    Tag {
        /// The URL whose matching bookmarks should be tagged. Must be non-empty.
        url: String,
        /// The label to attach. Must be non-empty.
        label: String,
    },
    /// Emit bookmarks as storage-format JSON to stdout. Optionally filter by tag(s).
    ///
    /// Emits the storage-format object-wrapped shape
    /// (`{"bookmarks":[...]}`) to stdout with newest-first ordering
    /// preserved. With one or more `--tag <label>` flags, only bookmarks
    /// matching at least one supplied label are emitted (OR-semantics
    /// parallel to `bm list --tag`). Empty / absent store: emits
    /// `{"bookmarks":[]}` + exit 0 (stderr silent — pipeline-rendering
    /// audience, not human-rendering). Filter no-match: same empty-array
    /// shape + exit 0. Empty label (`bm export --tag ""`): exit 1 with
    /// `Error: tag label cannot be empty.`
    ///
    /// `display_safe` wraps URL + tag-label strings at the serialization
    /// boundary so the emitted JSON is escape-clean for downstream
    /// pipeline-renderable surfaces (terminals, log aggregators).
    Export {
        /// Filter exported bookmarks to those tagged with this label.
        /// Repeatable; repeated `--tag` composes with OR-semantics.
        #[arg(long = "tag", action = ArgAction::Append)]
        tags: Vec<String>,
    },
    /// Read bookmarks from stdin (storage-format JSON) and append to the store.
    ///
    /// Reads a payload matching the storage-format object-wrapped shape
    /// (`{"bookmarks":[...]}`) from stdin and appends new records to the
    /// existing store. Dedup-on-exact-tuple-match (`url`+`timestamp`+`tags`)
    /// runs both against existing destination state AND within the
    /// imported payload — byte-equal records collapse to one appended
    /// record. Emits `Imported N bookmark(s).` to stderr (singular for
    /// N=1, plural otherwise). All imported bookmarks land in one atomic
    /// save; partial imports are forbidden (any validation failure
    /// rejects the entire payload).
    ///
    /// Empty stdin: exit 1 with `Error: stdin is empty; nothing to import.`
    /// Invalid JSON: exit 1 with `Error: stdin is not valid JSON.` + parse
    /// detail. Schema mismatch (including bare-array stdin): exit 1 with
    /// `Error: stdin JSON does not match storage-format schema; ...`
    /// Stdin exceeds `--max-stdin-bytes` (default 10 MB): exit 1.
    Import {
        /// Maximum stdin byte count accepted. Defaults to 10 MB matching
        /// the project's existing scale ceiling. Operator override for
        /// legitimately-larger imports.
        #[arg(long = "max-stdin-bytes", default_value_t = MAX_STDIN_BYTES_DEFAULT)]
        max_stdin_bytes: usize,
    },
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
        // required positionals in our grammar are `<URL>` on `bm add`,
        // `<URL>` and `<LABEL>` on `bm tag`. Per `DESIGN.md` § `bm add`
        // and § `bm tag` failure contracts, missing-positional surfaces
        // as the same exit-1 spec-contracted message as the empty-arg
        // path. `<LABEL>` is checked before `<URL>` because in `bm tag
        // <url>` (only one positional) the URL is supplied and the
        // missing one is the label.
        let missing = err
            .get(clap::error::ContextKind::InvalidArg)
            .map(|v| format!("{v}"));
        if matches!(missing.as_deref(), Some(m) if m.contains("<LABEL>") || m.contains("<label>") || m.contains("LABEL"))
        {
            eprintln!("Error: tag label cannot be empty.");
            return ExitCode::from(1);
        }
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

fn run_add(path: &std::path::Path, url: String) -> ExitCode {
    if url.is_empty() {
        eprintln!("Error: URL cannot be empty.");
        return ExitCode::from(1);
    }
    let mut store = match BookmarkStore::load(path) {
        Ok(s) => s,
        Err(e) => {
            emit_storage_error(&e, "load");
            return ExitCode::from(2);
        }
    };
    if let Err(e) = store.add(url) {
        // Library invariant rejection — empty URL is the only case at
        // present, which is also screened above; preserve the spec
        // message for that path (no hint — this is user input, not
        // storage).
        eprintln!("Error: {}", display_safe(&format!("{e:#}")));
        return ExitCode::from(1);
    }
    if let Err(e) = store.save(path) {
        emit_storage_error(&e, "save");
        return ExitCode::from(2);
    }
    ExitCode::SUCCESS
}

fn run_list(path: &std::path::Path, tags: &[String]) -> ExitCode {
    let store = match BookmarkStore::load(path) {
        Ok(s) => s,
        Err(e) => {
            emit_storage_error(&e, "load");
            return ExitCode::from(2);
        }
    };
    // AC 11 / DESIGN.md § `bm list --tag <label>` failure contract: empty
    // label is an INPUT-INVARIANT rejection that fires before any store-
    // state-dependent branch. Per Layer 2 Round 1 SE Finding 3: previously
    // the empty-store branch fired first, so `bm list --tag ""` against an
    // empty store emitted "No bookmarks yet." (exit 0) instead of the spec-
    // contracted exit-1 empty-label error. Layered as: validate inputs
    // first, then branch on state.
    if tags.iter().any(String::is_empty) {
        eprintln!("Error: tag label cannot be empty.");
        return ExitCode::from(1);
    }
    // Per `DESIGN.md` § Edge case catalog Layer 2: the empty-store
    // empty-state takes precedence over the filter empty-state — a user
    // with no bookmarks at all gets the more informative
    // "No bookmarks yet." even if they passed `--tag <non-empty-label>`.
    if store.bookmarks().is_empty() {
        eprintln!("No bookmarks yet.");
        return ExitCode::SUCCESS;
    }
    if tags.is_empty() {
        for bm in store.newest_first() {
            println!("{} {}", bm.timestamp().to_rfc3339(), display_safe(bm.url()));
        }
        return ExitCode::SUCCESS;
    }
    let labels: Vec<&str> = tags.iter().map(String::as_str).collect();
    let matched = store.filter_by_tags(&labels);
    if matched.is_empty() {
        eprintln!("No bookmarks match the supplied filter.");
        return ExitCode::SUCCESS;
    }
    for bm in matched {
        println!("{} {}", bm.timestamp().to_rfc3339(), display_safe(bm.url()));
    }
    ExitCode::SUCCESS
}

fn run_tag(path: &std::path::Path, url: &str, label: &str) -> ExitCode {
    // Empty-arg rejection per `DESIGN.md` § `bm tag` failure contract.
    // Both the empty-string-positional and the missing-positional cases
    // route here (the missing-positional case is intercepted by
    // `handle_parse_error`'s MissingRequiredArgument branch).
    if url.is_empty() {
        eprintln!("Error: URL cannot be empty.");
        return ExitCode::from(1);
    }
    if label.is_empty() {
        eprintln!("Error: tag label cannot be empty.");
        return ExitCode::from(1);
    }
    let mut store = match BookmarkStore::load(path) {
        Ok(s) => s,
        Err(e) => {
            emit_storage_error(&e, "load");
            return ExitCode::from(2);
        }
    };
    match store.attach_tag(url, label) {
        Ok(n) => {
            if let Err(e) = store.save(path) {
                emit_storage_error(&e, "save");
                return ExitCode::from(2);
            }
            // Affordance — emit the match count to stderr so the multi-match
            // semantic (a `bm tag` that touched 2 bookmarks because two share
            // the same URL) is discoverable from user behavior. Stdout stays
            // silent so pipelines (`bm tag ... | jq ...` or similar) are
            // unaffected. Singular/plural conditional per Layer 2 Round 2 UX
            // F4 — "Tagged 1 bookmark." reads naturally; "Tagged 2 bookmarks."
            // pluralizes correctly. Closes Layer 2 Round 1 UX F2 + SE F2 +
            // Layer 2 Round 2 UX F4.
            let noun = if n == 1 { "bookmark" } else { "bookmarks" };
            eprintln!("Tagged {n} {noun}.");
            ExitCode::SUCCESS
        }
        Err(AttachTagError::EmptyUrl) => {
            // Defense-in-depth — pre-screened above but the library
            // boundary asserts the same invariant.
            eprintln!("Error: URL cannot be empty.");
            ExitCode::from(1)
        }
        Err(AttachTagError::EmptyLabel) => {
            eprintln!("Error: tag label cannot be empty.");
            ExitCode::from(1)
        }
        Err(e @ AttachTagError::NoMatch(_)) => {
            // The variant now carries the URL per Layer 2 carry-forward
            // close (SE Round 1 Finding 1); use the variant's Display
            // impl directly with `display_safe` to escape the URL
            // before stderr. The CLI shell no longer constructs the
            // message from out-of-band scope.
            eprintln!("Error: {}.", display_safe(&format!("{e}")));
            ExitCode::from(1)
        }
    }
}

fn run_export(path: &std::path::Path, tags: &[String]) -> ExitCode {
    if tags.iter().any(String::is_empty) {
        eprintln!("Error: tag label cannot be empty.");
        return ExitCode::from(1);
    }
    let store = match BookmarkStore::load(path) {
        Ok(s) => s,
        Err(e) => {
            emit_storage_error(&e, "load");
            return ExitCode::from(2);
        }
    };
    let filter: Option<Vec<&str>> = if tags.is_empty() {
        None
    } else {
        Some(tags.iter().map(String::as_str).collect())
    };
    let json = store.export_json(filter.as_deref());
    // `print!` (not `println!`) — `export_json` already supplies the
    // trailing newline per the spec contract; double-newline would
    // surprise pipeline consumers.
    print!("{json}");
    ExitCode::SUCCESS
}

fn run_import(path: &std::path::Path, max_stdin_bytes: usize) -> ExitCode {
    // Read stdin with a hard byte cap. `take(cap + 1)` lets us
    // distinguish "exactly at the cap" from "exceeded" without buffering
    // the entire stream into memory uncapped — single-shot read up to
    // cap+1 bytes; if length > cap, reject. Per `DESIGN.md` § Threat
    // model addition for stdin-fed attacker input.
    let mut bytes = Vec::new();
    let cap_plus_one = u64::try_from(max_stdin_bytes)
        .unwrap_or(u64::MAX)
        .saturating_add(1);
    if let Err(e) = std::io::stdin().take(cap_plus_one).read_to_end(&mut bytes) {
        eprintln!(
            "Error: failed to read stdin: {}",
            display_safe(&format!("{e:#}"))
        );
        return ExitCode::from(2);
    }
    if bytes.len() > max_stdin_bytes {
        eprintln!("Error: stdin exceeded maximum byte limit of {max_stdin_bytes}.");
        return ExitCode::from(1);
    }
    if bytes.is_empty() {
        eprintln!("Error: stdin is empty; nothing to import.");
        return ExitCode::from(1);
    }
    let Ok(payload) = String::from_utf8(bytes) else {
        // JSON is by spec UTF-8 — a non-UTF-8 stream is by definition
        // invalid JSON. Route through the invalid-JSON error path.
        eprintln!("Error: stdin is not valid JSON.");
        eprintln!("(stdin is not valid UTF-8)");
        return ExitCode::from(1);
    };

    let mut store = match BookmarkStore::load(path) {
        Ok(s) => s,
        Err(e) => {
            emit_storage_error(&e, "load");
            return ExitCode::from(2);
        }
    };

    match store.import_json(&payload) {
        Ok(n) => {
            // Save only when there are records to persist. Skipping the
            // save on the zero-appended path (empty payload or
            // all-records-dedup'd) keeps the on-disk byte state
            // unchanged, satisfying the empty-payload no-op test.
            if n > 0 {
                if let Err(e) = store.save(path) {
                    emit_storage_error(&e, "save");
                    return ExitCode::from(2);
                }
            }
            let noun = if n == 1 { "bookmark" } else { "bookmarks" };
            eprintln!("Imported {n} {noun}.");
            ExitCode::SUCCESS
        }
        Err(ImportError::InvalidJson(detail)) => {
            eprintln!("Error: stdin is not valid JSON.");
            eprintln!("{}", display_safe(&detail));
            ExitCode::from(1)
        }
        Err(ImportError::SchemaMismatch(detail)) => {
            eprintln!(
                "Error: stdin JSON does not match storage-format schema; expected {{\"bookmarks\": [...]}}."
            );
            eprintln!("{}", display_safe(&detail));
            ExitCode::from(1)
        }
    }
}

fn main() -> ExitCode {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(err) => return handle_parse_error(&err),
    };
    let path = store_path();

    match cli.command {
        Cmd::Add { url } => run_add(&path, url),
        Cmd::List { tags } => run_list(&path, &tags),
        Cmd::Tag { url, label } => run_tag(&path, &url, &label),
        Cmd::Export { tags } => run_export(&path, &tags),
        Cmd::Import { max_stdin_bytes } => run_import(&path, max_stdin_bytes),
    }
}
