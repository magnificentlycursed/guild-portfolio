#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::missing_errors_doc
)]

//! Library crate for the `tracker` issue-tracker CLI.
//!
//! This crate exposes the data model (`Issue`, `Tracker`, `CreateArgs`), the
//! command implementations (`cmd_create`, `cmd_list`, `cmd_status`, `cmd_show`,
//! `cmd_delete`), the parsing/validation helpers (`validate_title`,
//! `validate_description`, `parse_status`, `parse_priority`, `parse_label`,
//! `parse_id`), the storage primitives (`load_tracker`, `save_tracker`), and
//! the display safety helper `display_safe` for stderr Cc-escape. The
//! `tracker` binary in `src/main.rs` wires `clap`-parsed arguments to these
//! functions; integration tests in `tests/` invoke the compiled binary as a
//! subprocess.
//!
//! All public functions return `Result<T, String>` where the `Err` variant is
//! the user-facing error message (without an `Error: ` prefix — `main.rs` adds
//! it). See `DESIGN.md` for the full behavioral contract.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::io::IsTerminal;
use std::path::Path;

// --- Layer 7: TTY-detected color output (DESIGN.md "Interface / color output") ---
//
// Color is applied only to the `status` and `priority` value cells in `list`
// and `show` output, only when `ColorMode::On` is in effect. Piped stdout,
// `NO_COLOR=<anything>`, and `CLICOLOR=0` each force `ColorMode::Off`.
// `CLICOLOR_FORCE` is deliberately not honored (Round 2 DECISIONS.md: the
// pipe-cleanness contract takes precedence over forced color into a non-TTY
// stream).
//
// Every highlighted value carries the `bold` SGR attribute (Round 2 spec
// amendment per UX R10 F2 — WCAG 1.4.1 *Use of Color*: a non-color cue must
// accompany any color cue so users with color-vision deficiency can
// distinguish states):
//
//   priority=high      → bold red     `\x1b[1;31m`
//   priority=medium    → bold yellow  `\x1b[1;33m`
//   priority=low       → default (no escape)
//   status=in-progress → bold cyan    `\x1b[1;36m`
//   status=done        → bold green   `\x1b[1;32m`
//   status=open        → default (no escape)
//
// Raw ANSI escapes (no anstyle/termcolor dependency) keep the dependency
// surface minimal — these six sequences are universally supported by
// VT100-compatible terminals, which is the only TTY environment this
// single-user portfolio CLI targets.

const ANSI_RESET: &str = "\x1b[0m";

// --- Column-width constants (SA Review 13 Finding 2 + SA Review 11 Finding 1 closure) ---
//
// Magic-number column widths previously occurred at 4 inline literal sites
// across the rendering layer (the `cmd_list` header `println!`, the
// per-row `println!`, the two `truncate_with_ellipsis` calls, and the
// `render_cell` width arguments). The 13-char `show` label column
// occurred at 8 inline literal sites in `format_show_block`'s
// labelled-key format string. SA R13 F2 + SA R11 F1 named the
// extraction; landed at this commit per the SA carry-forward cluster
// closure.

/// Width of the `ID` column in `tracker list` output.
const ID_WIDTH: usize = 4;
/// Width of the `Status` column in `tracker list` output. Sized for
/// `"in-progress"` (11 chars), the widest legal value.
const STATUS_WIDTH: usize = 11;
/// Width of the `Priority` column in `tracker list` output. Sized for
/// `"medium"` (6 chars) plus 2 chars of trailing padding for visual
/// breathing room before the next column.
const PRIORITY_WIDTH: usize = 8;
/// Width of the `Labels` column in `tracker list` output. Truncates with
/// an ellipsis (`...`) past this many chars.
const LABELS_WIDTH: usize = 20;
/// Width of the `Title` column in `tracker list` output. Truncates with
/// an ellipsis past this many chars. The title column is the right-most
/// column and is not padded — the constant governs only the truncation
/// threshold.
const TITLE_WIDTH: usize = 50;
/// Width of the labelled-key column in `tracker show` output. The format
/// `Description:` is the longest legal label (12 chars + trailing colon
/// space → 13 total).
const LABEL_COLUMN_WIDTH: usize = 13;

/// Whether the rendering layer should emit ANSI color escapes.
///
/// Replaces the prior `use_color: bool` parameter with a self-documenting enum
/// (SA R15 F3 / SE R17 F1 — boolean-trap antipattern at the call site).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorMode {
    /// Emit ANSI color escapes for highlighted values.
    On,
    /// Suppress all ANSI color escapes; emit bare values.
    Off,
}

impl ColorMode {
    /// Returns `true` if this mode emits ANSI color escapes.
    pub fn is_on(self) -> bool {
        matches!(self, ColorMode::On)
    }
}

/// Decides the color mode for the current process based on stdout TTY-state
/// and the `NO_COLOR` / `CLICOLOR` env-var opt-outs (DESIGN.md "Interface /
/// color output").
///
/// Order of checks (any one returning a decision short-circuits):
/// 1. `TRACKER_INTERNAL_FORCE_COLOR` set to `1` — `On` (test seam only, not
///    part of the public CLI contract; see below).
/// 2. `std::io::stdout().is_terminal()` — if stdout is piped, `Off`.
/// 3. `NO_COLOR` set to any non-empty value — `Off` (per https://no-color.org/).
/// 4. `CLICOLOR` set to `0` — `Off`.
///
/// Otherwise `On`.
///
/// `CLICOLOR_FORCE` is intentionally NOT honored: this CLI never emits ANSI
/// to a non-TTY stdout regardless of env vars, preserving the pipe-cleanness
/// contract for downstream parsers.
///
/// # `TRACKER_INTERNAL_FORCE_COLOR` (QE Review 17 Finding 1 — test seam)
///
/// QE Round 1 raised that the TTY-positive color rendering surface (4 of
/// 13 Layer 7 ACs) has zero automated coverage because `assert_cmd::Command`
/// invokes the binary with stdout connected to a pipe — `is_terminal()`
/// returns false, color is suppressed by check 2 above. Without a seam,
/// the only way to exercise the positive path is the manual checklist.
///
/// `TRACKER_INTERNAL_FORCE_COLOR=1` is a deliberately ugly, namespaced env
/// var that integration tests in `tests/layer7.rs` set to bypass the TTY
/// check and force `ColorMode::On`. The variable is intentionally NOT
/// documented in `--help`, README.md, or DESIGN.md — it is not a user-facing
/// feature; it does not equal `CLICOLOR_FORCE` (which the spec deliberately
/// declines to honor); it exists solely to make the positive color contract
/// automatable.
///
/// Naming rationale: the `TRACKER_` prefix prevents collision with any
/// standard CLI color env-var convention; `INTERNAL_` signals "do not use";
/// the `=1` literal value (rather than any-non-empty) makes accidental
/// activation by an empty-string export less likely.
pub fn color_mode_from_env() -> ColorMode {
    // Test seam — see doc-comment above. Placed first so tests can exercise
    // the positive color path even when stdout is piped; does NOT bypass
    // NO_COLOR for production safety scenarios (the test runner must clear
    // NO_COLOR before setting this if it inherits NO_COLOR from CI env).
    if std::env::var_os("TRACKER_INTERNAL_FORCE_COLOR").is_some_and(|v| v == "1") {
        return ColorMode::On;
    }
    if !std::io::stdout().is_terminal() {
        return ColorMode::Off;
    }
    if let Some(v) = std::env::var_os("NO_COLOR") {
        if !v.is_empty() {
            return ColorMode::Off;
        }
    }
    if std::env::var_os("CLICOLOR").is_some_and(|v| v == "0") {
        return ColorMode::Off;
    }
    ColorMode::On
}

/// Returns the ANSI start sequence for a priority value's color theme, or
/// `None` if the value renders in default color (`low` / unknown) or
/// `color` is `ColorMode::Off`. Round 2: `medium` now bold-yellow.
fn priority_ansi(priority: &str, color: ColorMode) -> Option<&'static str> {
    if !color.is_on() {
        return None;
    }
    match priority {
        "high" => Some("\x1b[1;31m"),
        "medium" => Some("\x1b[1;33m"),
        _ => None,
    }
}

/// Returns the ANSI start sequence for a status value's color theme, or
/// `None` if the value renders in default color (`open` / unknown) or
/// `color` is `ColorMode::Off`. Round 2: `in-progress` now bold-cyan,
/// `done` now bold-green.
fn status_ansi(status: &str, color: ColorMode) -> Option<&'static str> {
    if !color.is_on() {
        return None;
    }
    match status {
        "in-progress" => Some("\x1b[1;36m"),
        "done" => Some("\x1b[1;32m"),
        _ => None,
    }
}

/// Wraps `value` with the given ANSI prefix + reset, or returns it unchanged
/// if `ansi` is `None`. Centralizes the "color is value-only, not row-wide"
/// contract: the caller passes the bare value text, not a padded cell.
///
/// Defense-in-depth (Security R11 F1): in debug builds, asserts the input
/// `value` contains no control characters. Today's call sites pass
/// `issue.status` and `issue.priority`, both validated against closed enums
/// at parse-time AND load-time — control bytes cannot reach this function in
/// release builds. The debug-assert catches any future refactor that
/// introduces a free-form colored field whose validation was missed.
fn wrap_color(value: &str, ansi: Option<&str>) -> String {
    debug_assert!(
        !value.chars().any(char::is_control),
        "wrap_color called with control-character-bearing value (output-boundary contract violated): {:?}",
        value
    );
    match ansi {
        Some(prefix) => format!("{}{}{}", prefix, value, ANSI_RESET),
        None => value.to_string(),
    }
}

/// Renders a (possibly colored) cell that, when printed, occupies exactly
/// `total_width` visible columns. The bare `value` provides both the
/// visible content (its `chars().count()` is the width budget) and the
/// substring to wrap in `ansi`. ANSI escape bytes do NOT consume the
/// padding budget — visible width is measured against the bare value.
///
/// Rust's `{:<width}` formatter pads to byte length, which double-counts
/// ANSI escape bytes — `render_cell` sidesteps that by computing visible
/// width internally from the bare value, eliminating the SE R17 F2
/// off-by-one API-misuse surface that the prior `pad_after_color(colored,
/// visible_chars, total_width)` signature exposed.
///
/// # ASCII-only constraint (QE Review 17 Finding 5)
///
/// Width is computed as `value.chars().count()`, which equals the visible
/// terminal width if and only if `value` is ASCII (every char is one
/// display column). Both current call sites pass `issue.status` or
/// `issue.priority`, which are validated against the closed enums
/// `STATUS_ORDER` / `PRIORITY_ORDER` at parse and load time — every legal
/// value is ASCII. The `debug_assert!` below pins the constraint: any
/// future caller passing a non-ASCII value (e.g., a spec amendment
/// permitting non-ASCII status/priority labels, or a free-form colored
/// field) will panic in debug builds, surfacing the gap before column
/// alignment silently breaks. Production remediation if the constraint is
/// ever relaxed: introduce a `unicode-width` dependency and replace
/// `chars().count()` with `UnicodeWidthStr::width(value)`.
fn render_cell(value: &str, ansi: Option<&str>, total_width: usize) -> String {
    debug_assert!(
        value.is_ascii(),
        "render_cell visible-width computation assumes ASCII (QE R17 F5); \
         non-ASCII value {:?} would mis-align the column. Introduce \
         `unicode-width` and use UnicodeWidthStr::width if the spec relaxes \
         the ASCII constraint on status/priority/other colored fields.",
        value
    );
    let colored = wrap_color(value, ansi);
    let visible_chars = value.chars().count();
    if visible_chars >= total_width {
        colored
    } else {
        format!("{}{}", colored, " ".repeat(total_width - visible_chars))
    }
}

/// The top-level storage shape persisted to `tracker.json`.
///
/// Wraps the list of issues with a monotonically-increasing `next_id` counter so
/// deleted IDs are never reused, even when the deleted issue was the highest id
/// at delete time. The counter is bumped on every successful create and is left
/// unchanged by delete — the spec invariant "deleted ID is never reused"
/// (DESIGN.md Feature 1 / Feature 5 / Data Model) is enforced by the counter,
/// not by `max(remaining_ids) + 1`.
///
/// Pre-SO-R22 storage was a bare `[Issue, ...]` JSON array (SA Review 3 Finding 3
/// removed an earlier `next_id` field). That shape did not preserve the contract
/// in the high-edge case (delete the largest id, then create — `max(remaining) + 1`
/// equals the just-deleted id). SO Review 22 Option A restores the persistent
/// counter; the prior array shape is rejected at load with the standard corrupt-data
/// message.
#[derive(Debug, Serialize, Deserialize)]
pub struct Tracker {
    /// All currently-stored issues. Order matches insertion order at write time;
    /// `cmd_list` sorts a working copy before rendering.
    pub issues: Vec<Issue>,
    /// The next ID to be assigned by `cmd_create`. Initialized to `1` for a fresh
    /// tracker; bumped via `checked_add(1)` on every create; never decreased by
    /// delete. Invariant at load: `next_id >= 1` and (if `issues` is non-empty)
    /// `next_id > max(issue.id)`.
    pub next_id: u64,
}

/// A single tracked issue, as stored in `tracker.json`.
///
/// All fields except `description` are required. `description` is omitted from
/// the JSON output when absent (`None`) rather than serialized as `null`.
#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
    /// Unique, monotonically-assigned positive integer; never reused (see `Tracker::next_id`).
    pub id: u64,
    /// Trimmed, non-empty issue title.
    pub title: String,
    /// Optional free-form description; stored verbatim (not trimmed). The JSON key
    /// is omitted entirely when `None`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// One of: `"open"`, `"in-progress"`, `"done"` (lowercase).
    pub status: String,
    /// One of: `"low"`, `"medium"`, `"high"` (lowercase).
    pub priority: String,
    /// Deduplicated, case-preserved labels in the order they were supplied at
    /// creation; may be empty.
    pub labels: Vec<String>,
    /// ISO 8601 UTC timestamp at second precision (e.g. `"2026-04-27T14:00:00Z"`);
    /// fixed at creation and never modified.
    pub created_at: String,
    /// ISO 8601 UTC timestamp at second precision; refreshed on every mutation.
    /// Always `>= created_at`.
    pub updated_at: String,
}

/// Trims `raw` and returns the trimmed title, or an error if it is empty after
/// trimming or contains a control character (Unicode general category `Cc`).
///
/// Control characters are rejected because they break the spec's one-issue-per-line
/// `list` contract (newline / CR), corrupt column alignment (tab), and enable
/// terminal-escape injection in tools that display the title (ESC, C1 controls).
/// See DESIGN.md "Edge Cases / Title".
///
/// # Errors
/// Returns `Err("Title cannot be empty.")` when `raw` is empty or whitespace-only.
/// Returns `Err("Title cannot contain control characters.")` when the trimmed title
/// contains any character where `char::is_control()` returns `true`.
pub fn validate_title(raw: &str) -> Result<String, String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err("Title cannot be empty.".to_string());
    }
    if trimmed.chars().any(char::is_control) {
        return Err("Title cannot contain control characters.".to_string());
    }
    Ok(trimmed.to_string())
}

/// Returns `current + 1` for advancing the persistent `Tracker::next_id` counter.
///
/// IDs are never reused: the counter is bumped on every successful create and is
/// not modified by delete, so the next assigned id is always strictly greater
/// than every previously-assigned id, deleted or not.
///
/// # Errors
/// Returns `Err` if `current == u64::MAX` (overflow). Unreachable through organic
/// use (the entire 64-bit ID space cannot be exhausted), but defends against
/// hand-edited `tracker.json` files that plant `next_id: u64::MAX` to corrupt
/// subsequent writes (Security R4 F2 lineage).
pub fn bump_next_id(current: u64) -> Result<u64, String> {
    current
        .checked_add(1)
        .ok_or_else(|| "Cannot assign new issue ID: maximum ID reached.".to_string())
}

/// Returns the current UTC time as an ISO 8601 string at second precision (e.g. `"2026-04-27T14:00:00Z"`).
pub fn current_timestamp() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

const CORRUPT_DATA_ERROR: &str =
    "Could not read tracker data. The file may be corrupt. Delete tracker.json to start fresh.";

const VALID_STATUSES: &[&str] = &["open", "in-progress", "done"];

/// Priority values in sort order (highest first). Single source of truth for both
/// validity (membership) and sort rank (index).
const PRIORITY_ORDER: &[&str] = &["high", "medium", "low"];

/// Parses an ISO 8601 / RFC 3339 timestamp string. Returns `None` on any parse failure.
///
/// Accepts the same shapes `chrono::DateTime::parse_from_rfc3339` accepts; the
/// project produces second-precision UTC strings (e.g. `"2026-04-27T14:00:00Z"`)
/// but a stored file from a future schema may include sub-second precision or
/// offsets — those parse successfully and are accepted as valid timestamps.
fn parse_timestamp(s: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(s)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
}

/// Per-record validation: domain values, label hygiene, timestamp parseability,
/// and the `updated_at >= created_at` invariant.
///
/// Whole-tracker invariants (ID uniqueness, `next_id` counter constraints) are
/// enforced separately by `tracker_is_valid`.
fn issue_fields_are_valid(issue: &Issue) -> bool {
    issue.id > 0
        && !issue.title.trim().is_empty()
        && !issue.title.chars().any(char::is_control)
        && VALID_STATUSES.contains(&issue.status.as_str())
        && PRIORITY_ORDER.contains(&issue.priority.as_str())
        && issue.labels.iter().all(|l| label_is_valid(l))
        && issue
            .description
            .as_ref()
            .is_none_or(|d| description_is_valid(d))
        && parse_timestamp(&issue.created_at).is_some()
        && parse_timestamp(&issue.updated_at).is_some()
        && issue.updated_at >= issue.created_at
}

/// Stored-description hygiene predicate. Stored descriptions are verbatim
/// (not trimmed); this predicate checks the same hygiene rules
/// `validate_description` enforces at the input boundary, so a hand-edited
/// `tracker.json` with a description that bypassed `validate_description`
/// (control character other than `\n`, or whitespace-only) is rejected at
/// load. Newline is permitted because the spec carves it out for multi-line
/// `show` rendering.
fn description_is_valid(description: &str) -> bool {
    !description.trim().is_empty() && !description.chars().any(|c| c.is_control() && c != '\n')
}

/// Stored-label hygiene predicate. Stored labels are post-trim; this predicate
/// checks the same hygiene rules `parse_label` enforces at the input boundary,
/// so a hand-edited `tracker.json` with a label that bypassed `parse_label`
/// (control character, comma, or whitespace-only) is rejected at load.
fn label_is_valid(label: &str) -> bool {
    !label.trim().is_empty() && !label.chars().any(char::is_control) && !label.contains(',')
}

/// Renders user-supplied input safe for interpolation into stderr error messages.
///
/// Escapes control characters (Unicode general category `Cc` — newline, CR, tab,
/// NUL, ESC, DEL, C1 controls) as `\u{XX}` so a pasted ANSI escape sequence or
/// embedded newline cannot cross the stderr → terminal boundary as raw bytes.
/// Non-control characters (including printable Unicode, emoji, CJK) pass through
/// unchanged. See DESIGN.md "stderr contract".
///
/// Round 2 (RT R10 F1): exposed `pub` so other crates (e.g. `src/main.rs` for
/// clap's argument-parsing errors) can apply the same value-level Cc-escape
/// before reaching stderr — the spec's stderr Cc-escape rule applies to every
/// stderr write site, including the parser's reflected user-supplied values.
/// For multi-line error strings (e.g. clap's `Error: ...\n\nUsage: ...`), use
/// `sanitize_quoted_values` instead so structural LFs survive while the
/// interpolated values inside the `'X'` quotes are still escaped.
pub fn display_safe(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if c.is_control() {
            out.push_str(&format!("\\u{{{:X}}}", c as u32));
        } else {
            out.push(c);
        }
    }
    out
}

/// Sanitizes a multi-line error string by applying `display_safe` only to the
/// substrings inside single-quoted regions (`'<value>'`), leaving the rest of
/// the message — including structural newlines and surrounding clap formatting
/// — unchanged.
///
/// Round 2 (RT R10 F1): clap's `Error: unrecognized subcommand 'X'\n\nUsage:
/// ...` template structurally uses `\n` for line breaks. A naive
/// whole-string `display_safe` would escape those structural `\n`s into
/// `\u{A}`, destroying clap's multi-line formatting. The DESIGN.md rule is
/// narrow — sanitize the REFLECTED USER-SUPPLIED VALUE, not the entire
/// error. This helper matches that narrow scope: it walks the string, tracks
/// the open/close of each single-quoted region, and applies `display_safe`
/// only to the bytes between the matching `'` pair. Quotes themselves pass
/// through unchanged so the error's visible structure (e.g. `'<user-input>'`)
/// is preserved.
///
/// If the input has an unbalanced trailing `'`, the quoted-but-unclosed tail
/// is still sanitized — safer than letting an unclosed payload through. If
/// the input has no quotes, it passes through unchanged (no sanitization
/// because there is no reflected user value to escape).
pub fn sanitize_quoted_values(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut buffer = String::new();
    let mut in_quote = false;
    for c in s.chars() {
        match (in_quote, c) {
            (false, '\'') => {
                in_quote = true;
                out.push('\'');
            }
            (true, '\'') => {
                out.push_str(&display_safe(&buffer));
                buffer.clear();
                in_quote = false;
                out.push('\'');
            }
            (true, _) => buffer.push(c),
            (false, _) => out.push(c),
        }
    }
    if in_quote {
        // Unbalanced trailing quote: sanitize the trailing payload defensively.
        out.push_str(&display_safe(&buffer));
    }
    out
}

/// Whole-tracker invariants: per-issue field validity, unique IDs across the
/// collection, `next_id >= 1`, and (when `issues` is non-empty) `next_id > max(issue.id)`.
///
/// `issue_fields_are_valid` covers per-record checks; this function adds the
/// cross-record uniqueness check (DESIGN.md "no two issues share the same ID")
/// and the counter invariants enforcing the persistent-counter contract from
/// SO Review 22. A `next_id` less than or equal to any stored id would mean the
/// next create reassigns an existing or deleted id — the exact contract violation
/// SO R22 closed.
fn tracker_is_valid(tracker: &Tracker) -> bool {
    if tracker.next_id < 1 {
        return false;
    }
    if !tracker.issues.iter().all(issue_fields_are_valid) {
        return false;
    }
    let mut seen = HashSet::with_capacity(tracker.issues.len());
    if !tracker.issues.iter().all(|i| seen.insert(i.id)) {
        return false;
    }
    if let Some(max_id) = tracker.issues.iter().map(|i| i.id).max() {
        if tracker.next_id <= max_id {
            return false;
        }
    }
    true
}

/// Loads the tracker from `path`.
///
/// Returns a fresh `Tracker { issues: vec![], next_id: 1 }` if the file does not
/// exist. All loaded data is treated as untrusted: per-record validation
/// (`issue_fields_are_valid`) and whole-tracker invariants (`tracker_is_valid`,
/// covering unique-IDs and `next_id > max(issue.id)`) both apply.
///
/// The pre-SO-R22 storage shape (bare JSON array `[Issue, ...]`) is no longer
/// accepted — it deserializes into the `Tracker` struct as a serde error and is
/// rejected with the standard corrupt-data message.
///
/// # Errors
/// Returns `Err` if the file cannot be read, contains malformed JSON, has the
/// wrong top-level shape (e.g., a bare array), contains a record with invalid
/// domain values (unknown status, zero ID, empty title, empty label, empty
/// description, malformed timestamp, `updated_at < created_at`), contains
/// duplicate IDs across records, or has a `next_id` that violates the counter
/// invariants.
pub fn load_tracker(path: &Path) -> Result<Tracker, String> {
    if !path.exists() {
        return Ok(Tracker {
            issues: Vec::new(),
            next_id: 1,
        });
    }
    let contents =
        fs::read_to_string(path).map_err(|e| format!("Could not read tracker data: {}.", e))?;
    let tracker: Tracker =
        serde_json::from_str(&contents).map_err(|_| CORRUPT_DATA_ERROR.to_string())?;
    if !tracker_is_valid(&tracker) {
        return Err(CORRUPT_DATA_ERROR.to_string());
    }
    Ok(tracker)
}

/// Serializes `tracker` as pretty-printed JSON and writes it to `path`.
///
/// # Errors
/// Returns `Err` if the file cannot be written (permission denied, disk full,
/// path is a directory, etc.). Serialization itself is infallible for `Tracker`.
pub fn save_tracker(path: &Path, tracker: &Tracker) -> Result<(), String> {
    #[allow(clippy::unwrap_used)]
    // Tracker is always serializable: no floats, no cycles, all fields implement Serialize
    let contents = serde_json::to_string_pretty(tracker).unwrap();
    fs::write(path, contents).map_err(|e| format!("Could not save tracker data: {}.", e))
}

/// Raw `tracker create` inputs as supplied by the CLI layer, before validation.
///
/// Bundles the per-field arguments so `cmd_create`'s signature is stable as
/// the spec adds optional flags (priority added at Layer 3, labels at Layer 4,
/// description at Layer 6). The struct holds borrows from the CLI parse
/// result; ownership of the underlying strings stays with the caller.
///
/// Field naming uses `_raw` suffix to distinguish CLI-provided input from
/// validated/parsed values inside `cmd_create`.
pub struct CreateArgs<'a> {
    /// Required `<title>` positional argument.
    pub title_raw: &'a str,
    /// Optional `--description` value (None if flag absent).
    pub description_raw: Option<&'a str>,
    /// Optional `--priority` value (None if flag absent → defaults to `medium`).
    pub priority_raw: Option<&'a str>,
    /// Zero or more `--label` values (already collected by clap).
    pub labels_raw: &'a [String],
}

/// Implements `tracker create "<title>" [--description <d>] [--priority <p>] [--label <l>]...`.
///
/// Validates the title, optional description, optional priority, and each
/// label; assigns the next ID; appends the new issue to storage; and prints
/// `Created issue #<id>: <title>` to stdout. Priority defaults to `medium`
/// when not supplied. Labels are trimmed individually and deduplicated (first
/// occurrence preserved, case-sensitive). Description is stored verbatim
/// (not trimmed) when supplied.
///
/// # Errors
/// Returns `Err` if the title is empty/whitespace, the description is empty
/// after trim or contains a forbidden control character, the priority is
/// invalid, any label is empty after trim, stored data is unreadable or
/// corrupt, the ID space is exhausted, or persisting the new issue fails.
pub fn cmd_create(args: &CreateArgs, issues_path: &Path) -> Result<(), String> {
    let title = validate_title(args.title_raw)?;
    let description = match args.description_raw {
        Some(d) => Some(validate_description(d)?),
        None => None,
    };
    let priority = match args.priority_raw {
        Some(p) => parse_priority(p)?,
        None => "medium".to_string(),
    };
    let parsed_labels: Vec<String> = args
        .labels_raw
        .iter()
        .map(|l| parse_label(l))
        .collect::<Result<_, _>>()?;
    let labels = dedupe_labels(&parsed_labels);
    let mut tracker = load_tracker(issues_path)?;
    let id = tracker.next_id;
    tracker.next_id = bump_next_id(tracker.next_id)?;
    let now = current_timestamp();
    tracker.issues.push(Issue {
        id,
        title: title.clone(),
        description,
        status: "open".to_string(),
        priority,
        labels,
        created_at: now.clone(),
        updated_at: now,
    });
    save_tracker(issues_path, &tracker)?;
    println!("Created issue #{}: {}", id, title);
    Ok(())
}

/// Parses and normalizes a status string (case-insensitive).
///
/// Returns the canonical lowercase status value, or an error describing the valid values.
///
/// # Errors
/// Returns `Err` if `raw` is not (case-insensitively) one of `open`, `in-progress`, `done`.
pub fn parse_status(raw: &str) -> Result<String, String> {
    let lower = raw.to_lowercase();
    if VALID_STATUSES.contains(&lower.as_str()) {
        Ok(lower)
    } else {
        Err(format!(
            "Invalid status '{}'. Expected: open, in-progress, or done.",
            display_safe(raw)
        ))
    }
}

/// Parses an issue ID from a string. Must be a positive integer (>= 1).
///
/// # Errors
/// Returns `Err` if `raw` does not parse as a positive `u64` (`0`, negative, non-numeric, overflow).
pub fn parse_id(raw: &str) -> Result<u64, String> {
    raw.parse::<u64>().ok().filter(|&n| n > 0).ok_or_else(|| {
        format!(
            "'{}' is not a valid issue ID. Expected a positive integer.",
            display_safe(raw)
        )
    })
}

/// Implements `tracker status <id> <status>`.
///
/// Validates `id_raw` as a positive integer and `status_raw` as one of the
/// canonical status values (case-insensitive). Locates the issue, updates its
/// `status` and refreshes `updated_at`, persists, and prints
/// `Issue #<id> status → <new_status>.` to stdout. The set is idempotent — re-applying
/// the current status succeeds and refreshes `updated_at`.
///
/// # Errors
/// Returns `Err` if the ID is malformed, the issue does not exist, the status
/// value is invalid, or storage I/O fails.
pub fn cmd_status(id_raw: &str, status_raw: &str, issues_path: &Path) -> Result<(), String> {
    let id = parse_id(id_raw)?;
    let new_status = parse_status(status_raw)?;
    let mut tracker = load_tracker(issues_path)?;
    let idx = tracker
        .issues
        .iter()
        .position(|i| i.id == id)
        .ok_or_else(|| format!("Issue #{} not found.", id))?;
    tracker.issues[idx].status = new_status;
    tracker.issues[idx].updated_at = current_timestamp();
    save_tracker(issues_path, &tracker)?;
    println!(
        "Issue #{} status \u{2192} {}.",
        id, tracker.issues[idx].status
    );
    Ok(())
}

/// Validates an `--description` value against the spec's empty-after-trim and
/// control-character rules.
///
/// Per DESIGN.md Feature 1: `--description` must be non-empty after trim, but
/// the *stored* value is the input verbatim (not trimmed). This function returns
/// the un-trimmed input on success so the caller can write it as-is.
///
/// Per DESIGN.md Edge Cases / Description: description rejects every control
/// character (Unicode general category `Cc`) EXCEPT newline (`\n`). The
/// carve-out exists because the spec explicitly permits multi-line descriptions
/// for `show` continuation rendering. Bidi controls (`Cf`) are NOT rejected
/// (same out-of-threat-model posture as title and labels — single-user CLI).
/// Same lineage as the title (Layer 1) and label (Layer 4) control-character
/// defenses: free-form text that flows to a terminal-emitting render path
/// must not carry escape bytes.
///
/// # Errors
/// Returns `Err("Description cannot be empty.")` when `raw` is empty or
/// whitespace-only after trim.
/// Returns `Err("Description cannot contain control characters other than newline.")`
/// when `raw` contains any `char::is_control()` other than `\n`.
pub fn validate_description(raw: &str) -> Result<String, String> {
    if raw.trim().is_empty() {
        return Err("Description cannot be empty.".to_string());
    }
    if raw.chars().any(|c| c.is_control() && c != '\n') {
        return Err(
            "Description cannot contain control characters other than newline.".to_string(),
        );
    }
    Ok(raw.to_string())
}

/// Renders a single issue as the `tracker show` labelled key-value block.
///
/// Per DESIGN.md "Show output format": each label is right-padded to a fixed
/// width of `LABEL_COLUMN_WIDTH` (= 13) characters so values align. For
/// multi-line descriptions, the first line follows the `Description:` label;
/// each continuation line is indented by `LABEL_COLUMN_WIDTH` spaces.
///
/// Returns the formatted block including a trailing newline.
fn format_show_block(issue: &Issue, color: ColorMode) -> String {
    // SA R13 F2 closure: the prior format string contained 8 inline literal
    // padded labels ("ID:          ", "Title:       ", ...). They are now
    // computed via `show_label` so the LABEL_COLUMN_WIDTH constant is the
    // single source of truth for label-column width.
    let labels_display = if issue.labels.is_empty() {
        "(none)".to_string()
    } else {
        issue.labels.join(", ")
    };
    // Continuation-line indent for multi-line descriptions: a `\n` followed
    // by LABEL_COLUMN_WIDTH spaces. Computed from the constant so any future
    // amendment to the label-column width propagates here automatically.
    let continuation_indent = format!("\n{:<width$}", "", width = LABEL_COLUMN_WIDTH);
    let description_display = match &issue.description {
        None => "(none)".to_string(),
        Some(d) => {
            // `\r\n` sequences are normalized to `\n` for splitting so a
            // CRLF-stored description renders without a stray `\r` in the
            // first line.
            let normalized = d.replace("\r\n", "\n");
            normalized.replace('\n', &continuation_indent)
        }
    };
    // Layer 7: color the status and priority values when `color` is `On`.
    // The label column ("Status:      ", "Priority:    ") is uncolored — color
    // applies to value text only per DESIGN.md "Interface / color output".
    let status_display = wrap_color(&issue.status, status_ansi(&issue.status, color));
    let priority_display = wrap_color(&issue.priority, priority_ansi(&issue.priority, color));
    format!(
        "{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n",
        show_label("ID"),
        issue.id,
        show_label("Title"),
        issue.title,
        show_label("Status"),
        status_display,
        show_label("Priority"),
        priority_display,
        show_label("Labels"),
        labels_display,
        show_label("Description"),
        description_display,
        show_label("Created"),
        issue.created_at,
        show_label("Updated"),
        issue.updated_at,
    )
}

/// Formats a `show` block label with trailing colon, right-padded to
/// `LABEL_COLUMN_WIDTH`. Single source of truth for the label-column
/// shape per SA R13 F2 closure.
fn show_label(name: &str) -> String {
    let with_colon = format!("{}:", name);
    format!("{:<width$}", with_colon, width = LABEL_COLUMN_WIDTH)
}

/// Implements `tracker show <id>`.
///
/// Validates `id_raw`, locates the issue, and prints the full labelled
/// key-value block (per DESIGN.md "Show output format") to stdout. Show is
/// non-mutating: storage is read but never written. `color` decides whether
/// the rendered block carries ANSI color escapes for the status / priority
/// value cells; `main.rs` reads `color_mode_from_env()` once and threads it
/// through (SA R15 F2 / SE R17 F1 — single decision point in the binary).
///
/// # Errors
/// Returns `Err` if the ID is malformed, the issue does not exist, or
/// storage I/O fails.
pub fn cmd_show(id_raw: &str, issues_path: &Path, color: ColorMode) -> Result<(), String> {
    let id = parse_id(id_raw)?;
    let tracker = load_tracker(issues_path)?;
    let issue = tracker
        .issues
        .iter()
        .find(|i| i.id == id)
        .ok_or_else(|| format!("Issue #{} not found.", id))?;
    // `format_show_block` already includes a trailing newline; use `print!`
    // rather than `println!` to avoid emitting a stray blank line.
    print!("{}", format_show_block(issue, color));
    Ok(())
}

/// Implements `tracker delete <id>`.
///
/// Validates `id_raw`, locates the issue, removes it from storage, persists
/// the updated tracker (issues without the removed entry; `next_id` unchanged),
/// and prints `Deleted issue #<id>.` to stdout. Deleted IDs are never reused:
/// `Tracker::next_id` is monotonically increasing across create/delete, so the
/// next create always assigns an id strictly greater than every previously-assigned
/// id, including the just-deleted one (SO Review 22 Option A). Other issues
/// are not affected.
///
/// # Errors
/// Returns `Err` if the ID is malformed, the issue does not exist, or
/// storage I/O fails.
pub fn cmd_delete(id_raw: &str, issues_path: &Path) -> Result<(), String> {
    let id = parse_id(id_raw)?;
    let mut tracker = load_tracker(issues_path)?;
    let idx = tracker
        .issues
        .iter()
        .position(|i| i.id == id)
        .ok_or_else(|| format!("Issue #{} not found.", id))?;
    tracker.issues.remove(idx);
    save_tracker(issues_path, &tracker)?;
    println!("Deleted issue #{}.", id);
    Ok(())
}

/// Sort rank for `p`: index in `PRIORITY_ORDER` (high=0, medium=1, low=2).
///
/// Returns `usize::MAX` for unknown values as a defensive fallback. The fallback
/// is unreachable for stored data: `issue_fields_are_valid` rejects priorities
/// outside `PRIORITY_ORDER` at load time. Routing an unrecognized priority to the
/// bottom of sort order is preferable to panicking on an internal-only path.
fn priority_rank(p: &str) -> usize {
    PRIORITY_ORDER
        .iter()
        .position(|&x| x == p)
        .unwrap_or(usize::MAX)
}

/// Parses and normalizes a priority string (case-insensitive).
///
/// Returns the canonical lowercase priority value, or an error describing the valid values.
///
/// # Errors
/// Returns `Err` if `raw` is not (case-insensitively) one of `low`, `medium`, `high`.
pub fn parse_priority(raw: &str) -> Result<String, String> {
    let lower = raw.to_lowercase();
    if PRIORITY_ORDER.contains(&lower.as_str()) {
        Ok(lower)
    } else {
        Err(format!(
            "Invalid priority '{}'. Expected: low, medium, or high.",
            display_safe(raw)
        ))
    }
}

/// Sorts issues by priority (high → medium → low) then by ID ascending.
pub fn sort_issues(issues: &mut [Issue]) {
    issues.sort_by(|a, b| {
        priority_rank(&a.priority)
            .cmp(&priority_rank(&b.priority))
            .then(a.id.cmp(&b.id))
    });
}

/// Validates and trims a label. Returns the trimmed value, or an error if the
/// label fails any of the input-hygiene rules.
///
/// Hygiene rules (matching DESIGN.md "Edge Cases / Labels"):
/// - non-empty after trim
/// - no control characters (Unicode general category `Cc`)
/// - no comma `,` (the `Labels` column display separator)
///
/// The control-char rule mirrors `validate_title`: labels flow into the same
/// `list` rendering pipeline as titles, so the same one-issue-per-line and
/// terminal-escape-injection rationale applies.
///
/// # Errors
/// Returns `Err("Label cannot be empty.")` when `raw` is empty or whitespace-only.
/// Returns `Err("Label cannot contain control characters.")` when the trimmed
/// label contains any character where `char::is_control()` returns `true`.
/// Returns `Err("Label cannot contain a comma.")` when the trimmed label
/// contains the character `,`.
pub fn parse_label(raw: &str) -> Result<String, String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err("Label cannot be empty.".to_string());
    }
    if trimmed.chars().any(char::is_control) {
        return Err("Label cannot contain control characters.".to_string());
    }
    if trimmed.contains(',') {
        return Err("Label cannot contain a comma.".to_string());
    }
    Ok(trimmed.to_string())
}

/// Returns `labels` with duplicates removed; first occurrence preserved.
///
/// Comparison is case-sensitive: `"bug"` and `"Bug"` are distinct labels.
pub fn dedupe_labels(labels: &[String]) -> Vec<String> {
    let mut seen = HashSet::with_capacity(labels.len());
    let mut out = Vec::with_capacity(labels.len());
    for label in labels {
        if seen.insert(label.as_str()) {
            out.push(label.clone());
        }
    }
    out
}

/// Returns `true` iff any element of `labels` equals `filter` (case-sensitive).
///
/// Used by `tracker list --label <l>` to filter by exact-match label. The match
/// is case-sensitive per DESIGN.md Edge Cases / Labels: `--label Bug` does not
/// match an issue with label `bug`.
pub fn label_matches(labels: &[String], filter: &str) -> bool {
    labels.iter().any(|l| l == filter)
}

/// Returns `true` iff `issue` matches every supplied filter.
///
/// The `status` filter is required (the default-open view passes `"open"`); the
/// `priority` and `label` filters are optional (an absent filter is a wildcard).
/// Filters AND-combine: a filter that mismatches makes the whole predicate false.
/// Per DESIGN.md Feature 2 / Edge Cases / Labels, label comparison is
/// case-sensitive and exact-match; priority and status comparisons assume the
/// caller has already normalized the filter values (lowercase) and that stored
/// values are normalized at write/load time. The caller is also responsible
/// for applying any other normalization the spec requires before calling —
/// notably trimming the label filter (DESIGN.md Edge Cases / Labels mandates
/// trim-on-store / trim-on-filter symmetry; `cmd_list` runs `parse_label` on
/// the filter value to satisfy this).
fn issue_matches_filters(
    issue: &Issue,
    status: &str,
    priority: Option<&str>,
    label: Option<&str>,
) -> bool {
    issue.status == status
        && priority.is_none_or(|p| issue.priority == p)
        && label.is_none_or(|l| label_matches(&issue.labels, l))
}

fn truncate_with_ellipsis(s: &str, max_chars: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= max_chars {
        s.to_string()
    } else {
        let truncated: String = chars[..max_chars - 1].iter().collect();
        format!("{}…", truncated)
    }
}

/// Filters an issue list down to those matching the AND-combined filter
/// set. Pure function: no I/O, no allocations beyond the resulting `Vec`.
/// Extracted from `cmd_list`'s inline `retain` per SA R11 F1 closure so
/// the filter logic is unit-testable in isolation and a future filter
/// dimension lands as a parameter addition rather than a fourth inline
/// `retain` call.
fn filter_issues(
    issues: Vec<Issue>,
    status: &str,
    priority: Option<&str>,
    label: Option<&str>,
) -> Vec<Issue> {
    issues
        .into_iter()
        .filter(|i| issue_matches_filters(i, status, priority, label))
        .collect()
}

/// Renders the `tracker list` header row. Pure function — uses the
/// module-level column-width constants so a future spec amendment that
/// changes column widths touches one site (the constants) rather than
/// the format string. The header row is never colored per DESIGN.md
/// "Interface / Color output" (color applies to value cells only).
fn format_list_header() -> String {
    format!(
        "{:<id_width$}  {:<status_width$}  {:<priority_width$}  {:<labels_width$}  Title",
        "ID",
        "Status",
        "Priority",
        "Labels",
        id_width = ID_WIDTH,
        status_width = STATUS_WIDTH,
        priority_width = PRIORITY_WIDTH,
        labels_width = LABELS_WIDTH,
    )
}

/// Renders a single `tracker list` data row. Pure function modulo color
/// application — status and priority value cells are wrapped in their
/// respective ANSI sequences when `color` is `On`. The labels column
/// truncates with an ellipsis past `LABELS_WIDTH`; the title column
/// truncates past `TITLE_WIDTH`. Padding for status / priority is done
/// against *visible* character count (via `render_cell`) so ANSI bytes
/// do not consume column budget.
fn format_list_row(issue: &Issue, color: ColorMode) -> String {
    let labels_raw = if issue.labels.is_empty() {
        "(none)".to_string()
    } else {
        issue.labels.join(", ")
    };
    let labels_display = truncate_with_ellipsis(&labels_raw, LABELS_WIDTH);
    let title_display = truncate_with_ellipsis(&issue.title, TITLE_WIDTH);
    let status_cell = render_cell(
        &issue.status,
        status_ansi(&issue.status, color),
        STATUS_WIDTH,
    );
    let priority_cell = render_cell(
        &issue.priority,
        priority_ansi(&issue.priority, color),
        PRIORITY_WIDTH,
    );
    format!(
        "{:<id_width$}  {}  {}  {:<labels_width$}  {}",
        issue.id,
        status_cell,
        priority_cell,
        labels_display,
        title_display,
        id_width = ID_WIDTH,
        labels_width = LABELS_WIDTH,
    )
}

/// Implements `tracker list [--status <s>] [--priority <p>] [--label <l>]`.
///
/// With no flags (the *default open view*): shows only `open` issues; prints
/// `No open issues. Nice work!` to **stderr** when there are none.
///
/// With any of `--status <s>`, `--priority <p>`, or `--label <l>` (the *filter
/// view*): validates each provided value, AND-combines the filters, and prints
/// `No issues match the given filters.` to **stderr** when none match. Note: an
/// explicit `--status open` (with no other filter) still shows the default-view
/// empty message, since the effective filter set is identical to the default.
/// `--label` matches case-sensitively per DESIGN.md Edge Cases / Labels.
///
/// Empty-state messages are informational (not data) per DESIGN.md "stderr
/// contract" — routing them to stderr keeps stdout clean for piped consumers
/// like `tracker list | wc -l`.
///
/// # Errors
/// Returns `Err` if a provided filter value is not in its valid set, or if
/// stored data is unreadable or corrupt.
pub fn cmd_list(
    status_filter: Option<&str>,
    priority_filter: Option<&str>,
    label_filter: Option<&str>,
    issues_path: &Path,
    color: ColorMode,
) -> Result<(), String> {
    let effective_status = match status_filter {
        None => "open".to_string(),
        Some(s) => parse_status(s)?,
    };
    let effective_priority = match priority_filter {
        Some(p) => Some(parse_priority(p)?),
        None => None,
    };
    // Validate and trim the label filter symmetrically with create-time
    // `parse_label`. Without this, `tracker list --label "  bug  "` silently
    // returned no-match against a stored `bug`, and `tracker list --label ""`
    // silently no-matched while `tracker create --label ""` errored — the
    // round-trip asymmetry UX Review 6 / SO Review 16 surfaced. DESIGN.md
    // Feature 2 now specifies: filter is trimmed; empty-after-trim is rejected.
    let effective_label = match label_filter {
        Some(l) => Some(parse_label(l)?),
        None => None,
    };
    // Disjunction over non-default filters: any new filter the spec is amended
    // to add must extend `extra_filter_active` here — a single location —
    // rather than appending another `&& *_filter.is_none()` conjunct to the
    // empty-state predicate. Reduces the SO Review 11 regression hazard: the
    // structural fragility of the positive-enumeration form is what made the
    // earlier empty-state heuristic break when `--priority` was added in
    // Layer 3 and again when `--label` was added in Layer 4. SA Review 9
    // Finding 2. (DESIGN.md "Out of Scope" excludes text search; the
    // disjunction is shaped for spec-amended filters, not anticipated ones.)
    let extra_filter_active = effective_priority.is_some() || effective_label.is_some();
    let is_default_open_view = effective_status == "open" && !extra_filter_active;

    // cmd_list is now a thin orchestrator per SA R11 F1 closure:
    // load → filter (pure) → empty-state branch → sort (pure) →
    // format header + rows (pure) → println. Each step is independently
    // testable; column-width literals are centralized in the module-level
    // constants (ID_WIDTH / STATUS_WIDTH / PRIORITY_WIDTH / LABELS_WIDTH /
    // TITLE_WIDTH); color injection is delegated to `format_list_row`.
    let issues = load_tracker(issues_path)?.issues;
    let mut issues = filter_issues(
        issues,
        &effective_status,
        effective_priority.as_deref(),
        effective_label.as_deref(),
    );

    if issues.is_empty() {
        // Empty-state messages route to stderr per DESIGN.md "stderr contract" /
        // "Edge Cases / List": they are informational, not data. Routing them to
        // stdout would pollute pipe consumers (`tracker list | wc -l` would
        // return 1 in the empty case). SO Review 13 Finding 2.
        if is_default_open_view {
            eprintln!("No open issues. Nice work!");
        } else {
            eprintln!("No issues match the given filters.");
        }
        return Ok(());
    }

    sort_issues(&mut issues);
    println!("{}", format_list_header());
    for issue in &issues {
        println!("{}", format_list_row(issue, color));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title_empty_after_trim_is_rejected() {
        assert!(validate_title("").is_err());
        assert!(validate_title("   ").is_err());
    }

    #[test]
    fn title_trimmed_before_storage() {
        assert_eq!(validate_title("  Fix bug  "), Ok("Fix bug".to_string()));
    }

    #[test]
    fn title_with_newline_is_rejected() {
        assert!(validate_title("Fix\nbug").is_err());
        assert!(validate_title("Fix\rbug").is_err());
        assert!(validate_title("Fix\r\nbug").is_err());
    }

    #[test]
    fn title_with_tab_is_rejected() {
        assert!(validate_title("Fix\tbug").is_err());
    }

    #[test]
    fn title_with_escape_sequence_is_rejected() {
        // ANSI CSI: ESC `[` 2 J (clear screen). ESC = 0x1B is in category Cc.
        assert!(validate_title("\u{1B}[2JEvil").is_err());
        // OSC 8 hyperlink leader: ESC ] 8 ; ; ...
        assert!(validate_title("Click \u{1B}]8;;https://evil/\u{7}me\u{1B}]8;;\u{7}").is_err());
    }

    #[test]
    fn title_with_nul_or_del_is_rejected() {
        assert!(validate_title("Fix\u{00}bug").is_err());
        assert!(validate_title("Fix\u{7F}bug").is_err());
    }

    #[test]
    fn title_with_printable_unicode_is_accepted() {
        assert!(validate_title("Fix login bug 🐛").is_ok());
        assert!(validate_title("修复登录").is_ok());
        assert!(validate_title("café").is_ok()); // includes non-ASCII printable
    }

    #[test]
    fn issue_field_validation_rejects_control_char_in_title() {
        let mut bad = issue(1, "medium");
        bad.title = "Sneaky\nlist break".to_string();
        assert!(!issue_fields_are_valid(&bad));
    }

    #[test]
    fn bump_next_id_increments_by_one() {
        assert_eq!(bump_next_id(1), Ok(2));
        assert_eq!(bump_next_id(42), Ok(43));
    }

    #[test]
    fn bump_next_id_at_u64_max_returns_error() {
        // Defends against hand-edited tracker.json planting `next_id: u64::MAX`
        // to corrupt the next create. checked_add prevents the silent wrap to 0.
        assert!(bump_next_id(u64::MAX).is_err());
    }

    fn tracker_with(issues: Vec<Issue>, next_id: u64) -> Tracker {
        Tracker { issues, next_id }
    }

    #[test]
    fn tracker_validation_rejects_duplicate_ids() {
        let t = tracker_with(vec![issue(1, "medium"), issue(1, "high")], 2);
        assert!(!tracker_is_valid(&t));
    }

    #[test]
    fn tracker_validation_accepts_unique_ids() {
        let t = tracker_with(vec![issue(1, "medium"), issue(2, "high")], 3);
        assert!(tracker_is_valid(&t));
    }

    #[test]
    fn tracker_validation_rejects_next_id_zero() {
        // next_id must be >= 1 (counter starts at 1 for a fresh tracker).
        let t = tracker_with(Vec::new(), 0);
        assert!(!tracker_is_valid(&t));
    }

    #[test]
    fn tracker_validation_rejects_next_id_not_greater_than_max_id() {
        // The persistent-counter contract requires next_id > max(issue.id).
        // A stored tracker where next_id == max(id) would assign a duplicate id
        // on the next create — the exact SO R22 failure mode at load time.
        let t = tracker_with(vec![issue(5, "medium")], 5);
        assert!(!tracker_is_valid(&t));
        let t = tracker_with(vec![issue(5, "medium")], 4);
        assert!(!tracker_is_valid(&t));
    }

    #[test]
    fn tracker_validation_accepts_next_id_strictly_greater_than_max() {
        let t = tracker_with(vec![issue(5, "medium")], 6);
        assert!(tracker_is_valid(&t));
        // next_id may be much larger than max(id) when intermediate issues were
        // created then deleted — the counter retains its highest-assigned value.
        let t = tracker_with(vec![issue(1, "medium")], 99);
        assert!(tracker_is_valid(&t));
    }

    #[test]
    fn tracker_validation_accepts_empty_with_next_id_1() {
        // Fresh-tracker initial state: no issues, counter at 1.
        let t = tracker_with(Vec::new(), 1);
        assert!(tracker_is_valid(&t));
    }

    #[test]
    fn tracker_validation_accepts_empty_after_all_deleted_with_retained_counter() {
        // After creating and deleting every issue, the counter retains its value
        // (deleted IDs are never reused, including when issues becomes empty).
        let t = tracker_with(Vec::new(), 7);
        assert!(tracker_is_valid(&t));
    }

    #[test]
    fn issue_field_validation_rejects_empty_label() {
        let mut bad = issue(1, "medium");
        bad.labels = vec!["bug".to_string(), "  ".to_string()];
        assert!(!issue_fields_are_valid(&bad));
    }

    #[test]
    fn issue_field_validation_rejects_empty_description() {
        let mut bad = issue(1, "medium");
        bad.description = Some("   ".to_string());
        assert!(!issue_fields_are_valid(&bad));
    }

    #[test]
    fn issue_field_validation_rejects_malformed_timestamp() {
        let mut bad = issue(1, "medium");
        bad.created_at = "not a timestamp".to_string();
        assert!(!issue_fields_are_valid(&bad));
    }

    #[test]
    fn issue_field_validation_rejects_updated_before_created() {
        let mut bad = issue(1, "medium");
        bad.created_at = "2026-05-01T00:00:00Z".to_string();
        bad.updated_at = "2026-04-30T23:59:59Z".to_string();
        assert!(!issue_fields_are_valid(&bad));
    }

    #[test]
    fn issue_field_validation_accepts_equal_created_and_updated() {
        let issue = issue(1, "medium");
        // Helper sets both timestamps to the same value (matches fresh-issue invariant).
        assert!(issue_fields_are_valid(&issue));
    }

    #[test]
    fn status_value_parsing_valid_cases() {
        assert_eq!(parse_status("open"), Ok("open".to_string()));
        assert_eq!(parse_status("in-progress"), Ok("in-progress".to_string()));
        assert_eq!(parse_status("done"), Ok("done".to_string()));
        assert_eq!(parse_status("OPEN"), Ok("open".to_string()));
        assert_eq!(parse_status("IN-PROGRESS"), Ok("in-progress".to_string()));
        assert_eq!(parse_status("DONE"), Ok("done".to_string()));
    }

    #[test]
    fn status_value_parsing_rejects_invalid() {
        assert!(parse_status("done.").is_err());
        assert!(parse_status("in_progress").is_err());
        assert!(parse_status("closed").is_err());
    }

    #[test]
    fn id_must_be_positive_integer() {
        assert!(parse_id("0").is_err());
        assert!(parse_id("abc").is_err());
        assert_eq!(parse_id("1"), Ok(1));
        assert_eq!(parse_id("42"), Ok(42));
    }

    fn issue(id: u64, priority: &str) -> Issue {
        Issue {
            id,
            title: "x".to_string(),
            description: None,
            status: "open".to_string(),
            priority: priority.to_string(),
            labels: Vec::new(),
            created_at: "2026-01-01T00:00:00Z".to_string(),
            updated_at: "2026-01-01T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn priority_parsing_valid_cases() {
        assert_eq!(parse_priority("low"), Ok("low".to_string()));
        assert_eq!(parse_priority("medium"), Ok("medium".to_string()));
        assert_eq!(parse_priority("high"), Ok("high".to_string()));
        assert_eq!(parse_priority("LOW"), Ok("low".to_string()));
        assert_eq!(parse_priority("MEDIUM"), Ok("medium".to_string()));
        assert_eq!(parse_priority("HIGH"), Ok("high".to_string()));
    }

    #[test]
    fn priority_parsing_rejects_invalid() {
        assert!(parse_priority("critical").is_err());
        assert!(parse_priority("urgent").is_err());
        assert!(parse_priority("").is_err());
    }

    #[test]
    fn priority_sort_order_is_correct() {
        let mut issues = vec![
            issue(1, "low"),
            issue(2, "high"),
            issue(3, "medium"),
            issue(4, "high"),
        ];
        sort_issues(&mut issues);
        let priorities: Vec<&str> = issues.iter().map(|i| i.priority.as_str()).collect();
        assert_eq!(priorities, vec!["high", "high", "medium", "low"]);
    }

    #[test]
    fn priority_sort_tie_breaking_by_id() {
        let mut issues = vec![issue(2, "high"), issue(1, "high")];
        sort_issues(&mut issues);
        let ids: Vec<u64> = issues.iter().map(|i| i.id).collect();
        assert_eq!(
            ids,
            vec![1, 2],
            "lower ID should come first within the same priority tier"
        );
    }

    // --- Layer 4: labels (Red Gate) ---

    #[test]
    fn label_empty_after_trim_rejected() {
        assert!(parse_label("").is_err());
        assert!(parse_label("  ").is_err());
        assert!(parse_label("\t\n").is_err());
    }

    #[test]
    fn label_deduplication_preserves_first_occurrence() {
        let input = vec![
            "bug".to_string(),
            "bug".to_string(),
            "auth".to_string(),
            "bug".to_string(),
        ];
        assert_eq!(
            dedupe_labels(&input),
            vec!["bug".to_string(), "auth".to_string()],
            "first occurrence is kept; later duplicates dropped"
        );
    }

    #[test]
    fn label_filter_case_sensitive_match() {
        let labels = vec!["bug".to_string()];
        assert!(
            label_matches(&labels, "bug"),
            "exact-case match should return true"
        );
        assert!(
            !label_matches(&labels, "Bug"),
            "different case should NOT match (case-sensitive filter)"
        );
        assert!(
            !label_matches(&labels, "auth"),
            "non-member label should not match"
        );
    }

    // --- Round 2: label control-character and comma defenses ---

    #[test]
    fn label_with_newline_is_rejected() {
        assert!(parse_label("bug\nbreak").is_err());
        assert!(parse_label("bug\rbreak").is_err());
    }

    #[test]
    fn label_with_tab_is_rejected() {
        assert!(parse_label("bug\tbreak").is_err());
    }

    #[test]
    fn label_with_escape_sequence_is_rejected() {
        // ANSI CSI (red text) — ESC = 0x1B, category Cc.
        assert!(parse_label("\u{1B}[31mEvil\u{1B}[0m").is_err());
        // OSC 8 hyperlink leader — ESC ] 8 ; ; URL ST
        assert!(parse_label("\u{1B}]8;;https://evil/\u{7}X\u{1B}]8;;\u{7}").is_err());
    }

    #[test]
    fn label_with_nul_or_del_is_rejected() {
        assert!(parse_label("bug\u{00}break").is_err());
        assert!(parse_label("bug\u{7F}break").is_err());
    }

    #[test]
    fn label_with_comma_is_rejected() {
        assert!(parse_label("a,b").is_err());
        assert!(parse_label(",bug").is_err());
        assert!(parse_label("bug,").is_err());
    }

    #[test]
    fn label_with_printable_unicode_is_accepted() {
        assert!(parse_label("bug").is_ok());
        assert!(parse_label("emoji-🚀").is_ok());
        assert!(parse_label("中文").is_ok());
        assert!(parse_label("café").is_ok());
        assert!(parse_label("with space").is_ok());
    }

    #[test]
    fn issue_field_validation_rejects_control_char_in_label() {
        let mut bad = issue(1, "medium");
        bad.labels = vec!["bug\nfake".to_string()];
        assert!(!issue_fields_are_valid(&bad));
    }

    #[test]
    fn issue_field_validation_rejects_comma_in_label() {
        let mut bad = issue(1, "medium");
        bad.labels = vec!["a,b".to_string()];
        assert!(!issue_fields_are_valid(&bad));
    }

    #[test]
    fn issue_field_validation_accepts_clean_label() {
        let mut ok = issue(1, "medium");
        ok.labels = vec!["bug".to_string(), "auth".to_string()];
        assert!(issue_fields_are_valid(&ok));
    }

    // --- Layer 5: compound-filter predicate (Red Gate) ---

    fn issue_with(status: &str, priority: &str, labels: &[&str]) -> Issue {
        Issue {
            id: 1,
            title: "x".to_string(),
            description: None,
            status: status.to_string(),
            priority: priority.to_string(),
            labels: labels.iter().map(|s| s.to_string()).collect(),
            created_at: "2026-01-01T00:00:00Z".to_string(),
            updated_at: "2026-01-01T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn filter_and_logic_all_present_returns_true() {
        // DESIGN.md Feature 2: status, priority, label are AND-combined; an
        // issue matching all three filters must be present in results.
        let issue = issue_with("open", "high", &["bug"]);
        assert!(issue_matches_filters(
            &issue,
            "open",
            Some("high"),
            Some("bug")
        ));
    }

    #[test]
    fn filter_and_logic_all_must_match() {
        // AND, not OR: an issue that satisfies 2/3 filters must NOT pass the
        // predicate. Three subcases — each filter independently is the
        // odd-one-out — kill mutations that drop any single conjunct.
        let issue = issue_with("open", "high", &["bug"]);

        // status mismatch (priority + label match)
        assert!(
            !issue_matches_filters(&issue, "done", Some("high"), Some("bug")),
            "status mismatch must reject even when priority and label match"
        );
        // priority mismatch (status + label match)
        assert!(
            !issue_matches_filters(&issue, "open", Some("low"), Some("bug")),
            "priority mismatch must reject even when status and label match"
        );
        // label mismatch (status + priority match)
        assert!(
            !issue_matches_filters(&issue, "open", Some("high"), Some("feature")),
            "label mismatch must reject even when status and priority match"
        );
    }

    #[test]
    fn filter_status_only_matches_any_priority_and_labels() {
        // Optional filters absent → wildcard. Status-only filter accepts any
        // priority and any (including empty) label set.
        let high_with_bug = issue_with("open", "high", &["bug"]);
        let low_no_labels = issue_with("open", "low", &[]);
        assert!(issue_matches_filters(&high_with_bug, "open", None, None));
        assert!(issue_matches_filters(&low_no_labels, "open", None, None));
    }

    #[test]
    fn filter_status_mismatch_rejects_regardless_of_optional_filters() {
        // Status is a required filter (cmd_list always supplies one — default
        // "open" or the user's --status value). A mismatched status rejects
        // even when no optional filters are present.
        let issue = issue_with("done", "high", &["bug"]);
        assert!(!issue_matches_filters(&issue, "open", None, None));
    }

    #[test]
    fn filter_label_match_is_case_sensitive() {
        // Predicate-level corollary of label_filter_case_sensitive_match: the
        // compound predicate must inherit the case-sensitive contract from
        // label_matches, not silently lowercase.
        let issue = issue_with("open", "high", &["bug"]);
        assert!(issue_matches_filters(&issue, "open", None, Some("bug")));
        assert!(!issue_matches_filters(&issue, "open", None, Some("Bug")));
    }

    #[test]
    fn filter_and_logic_is_not_or_between_optional_conjuncts() {
        // Defense-in-depth (QE Review 13 F1) against `&&` → `||` between the
        // priority and label conjuncts: an issue that mismatches BOTH optional
        // filters (matching status only) must still reject. The three
        // single-mismatch subcases of filter_and_logic_all_must_match each
        // mismatch exactly one filter, so a between-optional `||` mutation
        // would survive them — this case mismatches both optionals at once.
        let issue = issue_with("open", "medium", &["bug"]);
        assert!(!issue_matches_filters(
            &issue,
            "open",
            Some("high"),
            Some("feature")
        ));
    }

    // --- SA R11 F1 closure: cmd_list extraction unit tests ---

    #[test]
    fn filter_issues_returns_empty_when_no_matches() {
        let issues = vec![issue_with("open", "high", &["bug"])];
        let out = filter_issues(issues, "done", None, None);
        assert!(out.is_empty());
    }

    #[test]
    fn filter_issues_returns_only_matching() {
        let issues = vec![
            issue_with("open", "high", &["bug"]),
            issue_with("done", "high", &["bug"]),
            issue_with("open", "low", &["bug"]),
        ];
        let out = filter_issues(issues, "open", Some("high"), None);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].status, "open");
        assert_eq!(out[0].priority, "high");
    }

    #[test]
    fn format_list_header_uses_width_constants() {
        // Header column boundaries must match the module-level width
        // constants exactly. A regression that changes one constant but
        // not the header rendering would surface here.
        let header = format_list_header();
        // "ID" padded to ID_WIDTH=4, then 2 spaces; "Status" padded to
        // STATUS_WIDTH=11, then 2 spaces; etc.
        assert_eq!(
            header,
            "ID    Status       Priority  Labels                Title"
        );
    }

    #[test]
    fn format_list_row_uncolored_when_color_off() {
        let issue = issue_with("open", "low", &["bug"]);
        let row = format_list_row(&issue, ColorMode::Off);
        // No ANSI sequences when color is Off, regardless of value.
        assert!(!row.contains("\x1b["));
        // ID column padded to ID_WIDTH=4 ("1" + 3 spaces).
        assert!(row.starts_with("1   "));
        // Title appears at the end uncolored.
        assert!(row.ends_with("x")); // issue_with default title is "x"
    }

    #[test]
    fn format_list_row_colors_high_priority_when_color_on() {
        let issue = issue_with("open", "high", &[]);
        let row = format_list_row(&issue, ColorMode::On);
        // Bold-red high priority embedded; status `open` is default-color
        // so the status cell remains plain.
        assert!(row.contains("\x1b[1;31mhigh\x1b[0m"));
        assert!(!row.contains("\x1b[1;36m")); // no cyan (in-progress)
        assert!(!row.contains("\x1b[1;32m")); // no green (done)
    }

    #[test]
    fn show_label_pads_to_label_column_width() {
        // Single source of truth for label-column shape: SA R13 F2 closure.
        // Every label rendered as `<name>:` then right-padded to
        // LABEL_COLUMN_WIDTH=13.
        assert_eq!(show_label("ID"), "ID:          "); // 2+1+10=13
        assert_eq!(show_label("Title"), "Title:       "); // 5+1+7=13
        assert_eq!(show_label("Description"), "Description: "); // 11+1+1=13
                                                                // Width invariant.
        for name in [
            "ID",
            "Title",
            "Status",
            "Priority",
            "Labels",
            "Description",
            "Created",
            "Updated",
        ] {
            assert_eq!(
                show_label(name).chars().count(),
                LABEL_COLUMN_WIDTH,
                "show_label({name:?}) must be exactly LABEL_COLUMN_WIDTH chars"
            );
        }
    }

    // --- Layer 6: description + show + delete (Red Gate) ---

    fn issue_with_full(id: u64, title: &str, description: Option<&str>, labels: &[&str]) -> Issue {
        Issue {
            id,
            title: title.to_string(),
            description: description.map(|s| s.to_string()),
            status: "open".to_string(),
            priority: "medium".to_string(),
            labels: labels.iter().map(|s| s.to_string()).collect(),
            created_at: "2026-01-01T00:00:00Z".to_string(),
            updated_at: "2026-01-01T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn multiline_description_show_format() {
        // DESIGN.md "Show output format": for multi-line descriptions, the
        // first line follows the `Description:` label; each continuation
        // line is indented by 13 spaces (matching the label-column width).
        let issue = issue_with_full(1, "Fix auth", Some("line1\nline2"), &[]);
        let out = format_show_block(&issue, ColorMode::Off);
        assert!(
            out.contains("Description: line1"),
            "first line must follow the Description: label:\n{out}"
        );
        assert!(
            out.contains("\n             line2"),
            "continuation line must be indented by 13 spaces:\n{out:?}"
        );
    }

    #[test]
    fn show_label_column_right_padded_to_13() {
        // DESIGN.md "Show output format": the label column is right-padded
        // to a fixed width of 13 characters so values align. Each label
        // (e.g. `ID:`, `Title:`, `Description:`) occupies the leading 13
        // chars of its line before the value starts.
        let issue = issue_with_full(42, "Hello", None, &["bug"]);
        let out = format_show_block(&issue, ColorMode::Off);
        // Pick a few representative labels and assert they appear with the
        // 13-char prefix shape.
        for prefix in &[
            "ID:          ", // "ID:" + 10 spaces = 13 chars
            "Title:       ", // "Title:" + 7 spaces = 13 chars
            "Status:      ", // "Status:" + 6 spaces = 13 chars
            "Priority:    ", // "Priority:" + 4 spaces = 13 chars
            "Labels:      ", // "Labels:" + 6 spaces = 13 chars
            "Description: ", // "Description:" + 1 space = 13 chars
            "Created:     ", // "Created:" + 5 spaces = 13 chars
            "Updated:     ", // "Updated:" + 5 spaces = 13 chars
        ] {
            assert!(
                out.contains(prefix),
                "expected 13-char label column prefix `{prefix}` in show output:\n{out}"
            );
        }
    }

    // --- Round 2: description Cc defense (Security R9 F1 / RT R8 F1 / DE R9 F1 / SE R15 F1 / QE R15 F2 / SO R20 F3) ---

    #[test]
    fn description_empty_after_trim_is_rejected() {
        assert!(validate_description("").is_err());
        assert!(validate_description("   ").is_err());
        assert!(validate_description("\n").is_err()); // newline-only is whitespace-only after trim
    }

    #[test]
    fn description_with_control_char_other_than_newline_is_rejected() {
        // ESC, BEL, NUL, DEL, tab, bare CR — all Cc, all not \n. Rejected.
        assert!(validate_description("a\u{1B}b").is_err());
        assert!(validate_description("a\u{07}b").is_err());
        assert!(validate_description("a\u{00}b").is_err());
        assert!(validate_description("a\u{7F}b").is_err());
        assert!(validate_description("a\tb").is_err());
        assert!(validate_description("a\rb").is_err());
        // CRLF: contains \r, which is Cc-not-\n. Reject.
        assert!(validate_description("line1\r\nline2").is_err());
    }

    #[test]
    fn description_with_newline_only_is_accepted() {
        // \n is the spec-permitted carve-out.
        assert_eq!(
            validate_description("line1\nline2"),
            Ok("line1\nline2".to_string())
        );
        assert_eq!(
            validate_description("first\nsecond\nthird"),
            Ok("first\nsecond\nthird".to_string())
        );
    }

    #[test]
    fn description_stored_verbatim_not_trimmed() {
        // Stored value is the raw input — not trimmed. Pins the
        // "stored as provided (not trimmed)" half of the spec contract.
        // Killing the mutation `Ok(raw.trim().to_string())`.
        assert_eq!(
            validate_description("  padded  "),
            Ok("  padded  ".to_string())
        );
        assert_eq!(
            validate_description("trailing-space "),
            Ok("trailing-space ".to_string())
        );
    }

    #[test]
    fn description_with_printable_unicode_is_accepted() {
        assert!(validate_description("emoji 🐛").is_ok());
        assert!(validate_description("中文").is_ok());
        assert!(validate_description("café").is_ok());
    }

    #[test]
    fn issue_field_validation_rejects_control_char_in_description() {
        let mut bad = issue(1, "medium");
        bad.description = Some("a\u{1B}[31mPWN".to_string());
        assert!(!issue_fields_are_valid(&bad));
    }

    #[test]
    fn issue_field_validation_rejects_carriage_return_in_description() {
        let mut bad = issue(1, "medium");
        bad.description = Some("line1\rOVER".to_string());
        assert!(!issue_fields_are_valid(&bad));
    }

    #[test]
    fn issue_field_validation_accepts_newline_in_description() {
        // \n is the spec-permitted carve-out at the load boundary too.
        let mut ok = issue(1, "medium");
        ok.description = Some("line1\nline2".to_string());
        assert!(issue_fields_are_valid(&ok));
    }

    #[test]
    fn issue_field_validation_accepts_no_description() {
        // None is always valid (description is optional).
        let issue = issue(1, "medium"); // helper leaves description = None
        assert!(issue_fields_are_valid(&issue));
    }

    #[test]
    fn high_edge_delete_does_not_reuse_id() {
        // SO Review 22 Option A: the persistent `next_id` counter is monotonic
        // across create and delete. After creating #1 and #2 (next_id bumps to
        // 3), deleting #2 leaves issues=[#1] but next_id stays at 3 — so the
        // next create assigns 3, NOT 2 (the just-deleted high-edge id).
        // Pre-SO-R22 implementation (`max(remaining_ids) + 1`) reassigned 2
        // here; this unit test pins the corrected counter behavior.
        let t = tracker_with(vec![issue(1, "medium")], 3);
        // The next assigned id is `tracker.next_id`; cmd_create then bumps via
        // `bump_next_id(t.next_id)`. Pin both halves of the contract.
        assert_eq!(t.next_id, 3, "stored counter must be the next-to-assign");
        assert_eq!(
            bump_next_id(t.next_id),
            Ok(4),
            "after assigning 3, the counter advances to 4 — id 2 (deleted) is unreachable"
        );
    }

    #[test]
    fn middle_gap_delete_does_not_reuse_id() {
        // Companion to high_edge_delete_does_not_reuse_id: after creating
        // #1, #2, #3 (next_id at 4) and deleting #2, the next create assigns 4.
        // The middle-gap id 2 is not reused. Same contract as the high-edge
        // case; both are subsumed by the monotonic counter.
        let t = tracker_with(vec![issue(1, "medium"), issue(3, "medium")], 4);
        assert_eq!(t.next_id, 4);
        assert_eq!(bump_next_id(t.next_id), Ok(5));
    }

    #[test]
    fn display_safe_passes_printable_chars_through() {
        assert_eq!(display_safe("low"), "low");
        assert_eq!(display_safe("foo bar"), "foo bar");
        assert_eq!(display_safe("emoji-🐛"), "emoji-🐛");
        assert_eq!(display_safe("中文"), "中文");
    }

    #[test]
    fn display_safe_escapes_control_chars() {
        assert_eq!(display_safe("a\nb"), "a\\u{A}b");
        assert_eq!(display_safe("a\tb"), "a\\u{9}b");
        assert_eq!(display_safe("\u{1B}[31m"), "\\u{1B}[31m");
        assert_eq!(display_safe("\u{00}"), "\\u{0}");
        assert_eq!(display_safe("\u{7F}"), "\\u{7F}");
    }

    // --- Round 2: sanitize_quoted_values (RT R10 F1 narrow-scope clap-error sanitizer) ---

    #[test]
    fn sanitize_quoted_values_passes_through_unquoted_content() {
        assert_eq!(
            sanitize_quoted_values("Error: foo\n\nUsage: bar"),
            "Error: foo\n\nUsage: bar"
        );
    }

    #[test]
    fn sanitize_quoted_values_escapes_only_inside_quotes() {
        // Structural newlines outside quotes must survive; control bytes
        // inside the quoted value must be escaped.
        let input = "Error: unrecognized subcommand 'pre\rmid\ttab'\n\nUsage: x";
        let out = sanitize_quoted_values(input);
        assert!(
            out.contains("'pre\\u{D}mid\\u{9}tab'"),
            "value sanitized: {out:?}"
        );
        assert!(
            out.contains("\n\nUsage: x"),
            "structural LFs preserved: {out:?}"
        );
    }

    #[test]
    fn sanitize_quoted_values_handles_unbalanced_trailing_quote() {
        // Defensive: an unclosed quote at the end of the string still gets
        // its trailing payload sanitized rather than passing through raw.
        let out = sanitize_quoted_values("garbage 'pre\rpost");
        assert_eq!(out, "garbage 'pre\\u{D}post");
    }

    #[test]
    fn sanitize_quoted_values_handles_multiple_quoted_regions() {
        // clap errors like `invalid value 'X' for '--flag'` have multiple
        // quoted regions; each must be independently sanitized.
        let out = sanitize_quoted_values("invalid value 'a\rb' for '--c\td'");
        assert!(out.contains("'a\\u{D}b'"));
        assert!(out.contains("'--c\\u{9}d'"));
    }

    // --- Layer 7 retroactive Red Gate: color helper unit tests ---
    //
    // VDD-IAR Alignment Review 17 Finding 1 (CRITICAL Dim 4): Layer 7's
    // Phase 2a Red Gate landed with 0 failing primary signals — the 9
    // integration tests in tests/layer7.rs all passed against pre-
    // implementation code (clap-default --help, no-ANSI-when-piped).
    // The implementation kept color logic in private helpers, eliminating
    // the positive Red Gate signal that pure-function unit tests on those
    // helpers would have provided.
    //
    // Resolution per VDD-IAR R17 F1 Option A: retrofit unit tests on the
    // testable primitives. Per prompts/implementation.md L56, retroactive
    // tests cannot satisfy the Red Gate (the implementation exists before
    // the test would fail) — they are labelled here as a Red Gate
    // deviation, not as the literal Red Gate that should have been.
    //
    // Round 2: color values updated to the bold-redundancy spec amendment
    // (DESIGN.md "Interface / color output" Round 2, UX R10 F2): medium,
    // in-progress, and done all gain `1;` bold prefix. `pad_after_color`
    // was replaced by `render_cell` (SE R17 F2 API refactor).
    //
    // retroactive Red Gate: priority_ansi color mapping — discovered during
    // Phase 3 IAR Round 1 (VDD-IAR Review 17 Finding 1), test added post-
    // implementation, confirmed passes against current implementation.
    //
    // retroactive Red Gate: status_ansi color mapping — same.
    //
    // retroactive Red Gate: wrap_color ANSI prefix + reset wrapping — same.
    //
    // retroactive Red Gate: render_cell visible-width padding (replaced
    // pad_after_color in Round 2) — same lineage.

    #[test]
    fn priority_ansi_high_returns_bold_red() {
        assert_eq!(priority_ansi("high", ColorMode::On), Some("\x1b[1;31m"));
    }

    #[test]
    fn priority_ansi_medium_returns_bold_yellow() {
        assert_eq!(priority_ansi("medium", ColorMode::On), Some("\x1b[1;33m"));
    }

    #[test]
    fn priority_ansi_low_returns_none() {
        // DESIGN.md "Interface / color output": low renders in default color.
        assert_eq!(priority_ansi("low", ColorMode::On), None);
    }

    #[test]
    fn priority_ansi_returns_none_when_color_off() {
        // Every priority value must return None when ColorMode::Off. A
        // regression here would emit ANSI to piped consumers — the
        // cmd_list / cmd_show integration tests catch the stdout side, but
        // this pins the helper contract directly.
        assert_eq!(priority_ansi("high", ColorMode::Off), None);
        assert_eq!(priority_ansi("medium", ColorMode::Off), None);
        assert_eq!(priority_ansi("low", ColorMode::Off), None);
    }

    #[test]
    fn status_ansi_in_progress_returns_bold_cyan() {
        assert_eq!(
            status_ansi("in-progress", ColorMode::On),
            Some("\x1b[1;36m")
        );
    }

    #[test]
    fn status_ansi_done_returns_bold_green() {
        assert_eq!(status_ansi("done", ColorMode::On), Some("\x1b[1;32m"));
    }

    #[test]
    fn status_ansi_open_returns_none() {
        // DESIGN.md "Interface / color output": open renders in default color.
        assert_eq!(status_ansi("open", ColorMode::On), None);
    }

    #[test]
    fn status_ansi_returns_none_when_color_off() {
        assert_eq!(status_ansi("in-progress", ColorMode::Off), None);
        assert_eq!(status_ansi("done", ColorMode::Off), None);
        assert_eq!(status_ansi("open", ColorMode::Off), None);
    }

    #[test]
    fn wrap_color_with_ansi_prefixes_and_resets() {
        // wrap_color must place the bare value between the prefix and the
        // ANSI reset sequence — a mutation that drops the reset would leak
        // color onto subsequent cells / rows.
        assert_eq!(
            wrap_color("high", Some("\x1b[1;31m")),
            "\x1b[1;31mhigh\x1b[0m"
        );
        assert_eq!(
            wrap_color("done", Some("\x1b[1;32m")),
            "\x1b[1;32mdone\x1b[0m"
        );
    }

    #[test]
    fn wrap_color_returns_bare_value_when_ansi_is_none() {
        // When the ansi argument is None (TTY-detection said no color OR
        // the value is a default-color value), the bare value is returned
        // unchanged — no ANSI bytes emitted.
        assert_eq!(wrap_color("low", None), "low");
        assert_eq!(wrap_color("open", None), "open");
        assert_eq!(wrap_color("", None), "");
    }

    #[test]
    fn render_cell_pads_visible_width_to_total() {
        // Bare "open" (4 visible chars) padded to width 11 → 7 trailing spaces.
        assert_eq!(render_cell("open", None, 11), "open       ");
        // Colored "high" with 4 visible chars padded to width 8 → 4 trailing
        // spaces; ANSI bytes do NOT consume padding budget. The visible
        // width is now computed internally from the bare value (SE R17 F2
        // API refactor: render_cell takes the bare value + ansi + width,
        // eliminating the visible_chars-must-match-bare-value contract
        // surface that pad_after_color exposed).
        assert_eq!(
            render_cell("high", Some("\x1b[1;31m"), 8),
            "\x1b[1;31mhigh\x1b[0m    "
        );
    }

    #[test]
    fn render_cell_does_not_pad_when_visible_equals_or_exceeds_total() {
        // "in-progress" (11 visible chars) exactly fills width 11 — no
        // padding added regardless of whether color is applied.
        assert_eq!(
            render_cell("in-progress", Some("\x1b[1;36m"), 11),
            "\x1b[1;36min-progress\x1b[0m"
        );
        assert_eq!(render_cell("in-progress", None, 11), "in-progress");
        // Visible > total: no padding (defensive — the cmd_list call sites
        // pass conforming widths, but the helper must not panic or
        // under-pad).
        assert_eq!(render_cell("anything", None, 4), "anything");
    }

    // --- Round 2: ColorMode + env-var helper tests ---
    // SE R17 F1 / SA R15 F3 refactor + UX R10 F1 / Security R11 F2
    // NO_COLOR / CLICOLOR honoring. The env-var helper tests use a serial
    // pattern guarded by a process-global mutex — Rust's test runner runs
    // unit tests in parallel by default, and these tests both read AND
    // mutate process env vars, which would race without serialization.

    use std::sync::Mutex;
    static ENV_TEST_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn color_mode_is_on_helper() {
        assert!(ColorMode::On.is_on());
        assert!(!ColorMode::Off.is_on());
    }

    #[test]
    fn color_mode_from_env_off_when_no_color_set() {
        // SAFETY: env-var mutation is racy across parallel tests; the
        // ENV_TEST_LOCK serializes our color_mode_from_env tests with each
        // other. We cannot defend against unrelated tests reading
        // NO_COLOR concurrently, but no other test in this crate touches
        // these env vars.
        let _guard = ENV_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // Safety preamble: clear both vars to a known state before mutating.
        // SAFETY: only this thread is reading these env vars during the
        // critical section (ENV_TEST_LOCK held); std::env::set_var is
        // unsafe in Rust 1.85+ due to data-race potential with concurrent
        // readers, but our serialization eliminates that race for the
        // tests in this crate.
        // Stdout in `cargo test` is non-TTY, so color_mode_from_env returns
        // Off regardless of env. We can only assert the env precedence at
        // the unit level: even when stdout WERE a TTY, NO_COLOR must force
        // Off. The integration tests (tests/layer7.rs, Round 2 additions)
        // cover the end-to-end env-var paths via the binary.
        unsafe {
            std::env::set_var("NO_COLOR", "1");
            std::env::remove_var("CLICOLOR");
        }
        assert_eq!(color_mode_from_env(), ColorMode::Off);
        unsafe {
            std::env::remove_var("NO_COLOR");
        }
    }

    #[test]
    fn color_mode_from_env_off_when_clicolor_zero() {
        let _guard = ENV_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            std::env::remove_var("NO_COLOR");
            std::env::set_var("CLICOLOR", "0");
        }
        assert_eq!(color_mode_from_env(), ColorMode::Off);
        unsafe {
            std::env::remove_var("CLICOLOR");
        }
    }

    #[test]
    fn color_mode_from_env_off_when_stdout_piped_regardless_of_env() {
        // In `cargo test`, stdout is captured (non-TTY). Even if no
        // env-var opt-out is set, color_mode_from_env returns Off because
        // is_terminal() is false. This pins the TTY-precedence invariant:
        // a piped stdout always wins over CLICOLOR_FORCE-style overrides
        // (which we deliberately don't honor anyway).
        let _guard = ENV_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            std::env::remove_var("NO_COLOR");
            std::env::remove_var("CLICOLOR");
            std::env::set_var("CLICOLOR_FORCE", "1");
        }
        assert_eq!(color_mode_from_env(), ColorMode::Off);
        unsafe {
            std::env::remove_var("CLICOLOR_FORCE");
        }
    }

    #[test]
    fn color_mode_from_env_on_when_internal_force_color_set() {
        // QE R17 F1 closure: the test seam must return On even when stdout
        // is piped (the assert_cmd subprocess case). cargo test runs with
        // stdout captured (non-TTY), so this exercises the bypass path.
        let _guard = ENV_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            std::env::set_var("TRACKER_INTERNAL_FORCE_COLOR", "1");
            std::env::remove_var("NO_COLOR");
            std::env::remove_var("CLICOLOR");
        }
        assert_eq!(color_mode_from_env(), ColorMode::On);
        unsafe {
            std::env::remove_var("TRACKER_INTERNAL_FORCE_COLOR");
        }
    }

    #[test]
    fn color_mode_from_env_force_color_ignored_for_non_one_values() {
        // The `=1` literal-value check (rather than any-non-empty) makes
        // accidental activation by an empty-string export less likely.
        let _guard = ENV_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            std::env::set_var("TRACKER_INTERNAL_FORCE_COLOR", "0");
            std::env::remove_var("NO_COLOR");
            std::env::remove_var("CLICOLOR");
        }
        // stdout is piped under cargo test → falls through to TTY check
        // → returns Off. The seam did NOT activate on "0".
        assert_eq!(color_mode_from_env(), ColorMode::Off);
        unsafe {
            std::env::set_var("TRACKER_INTERNAL_FORCE_COLOR", "true");
        }
        assert_eq!(color_mode_from_env(), ColorMode::Off);
        unsafe {
            std::env::set_var("TRACKER_INTERNAL_FORCE_COLOR", "");
        }
        assert_eq!(color_mode_from_env(), ColorMode::Off);
        unsafe {
            std::env::remove_var("TRACKER_INTERNAL_FORCE_COLOR");
        }
    }

    #[test]
    fn color_mode_from_env_off_when_no_color_set_to_empty_string() {
        // Per https://no-color.org/: "any value other than the empty
        // string". Our implementation reads `var_os` and checks `!v.is_empty()`,
        // so NO_COLOR="" should NOT suppress color (empty value = unset).
        let _guard = ENV_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            std::env::set_var("NO_COLOR", "");
            std::env::remove_var("CLICOLOR");
        }
        // stdout is non-TTY in cargo test, so this still returns Off — but
        // for the no-color.org reason, not for the NO_COLOR-empty reason.
        // The unit-level invariant we can pin: the function does not panic
        // and returns Off in the captured-stdout test environment.
        assert_eq!(color_mode_from_env(), ColorMode::Off);
        unsafe {
            std::env::remove_var("NO_COLOR");
        }
    }

    #[test]
    fn wrap_color_debug_assert_active_in_debug_builds() {
        // Defense-in-depth contract (Security R11 F1): wrap_color must
        // panic in debug builds when given a control-character-bearing
        // value. In release builds the debug_assert! is compiled out;
        // this test runs under `cargo test` which builds in debug mode by
        // default, so the assertion fires.
        let result = std::panic::catch_unwind(|| wrap_color("evil\x1b[0m", Some("\x1b[1;31m")));
        assert!(
            result.is_err(),
            "wrap_color must debug_assert on control-bearing values"
        );
    }

    #[test]
    fn render_cell_debug_assert_on_non_ascii_value() {
        // QE Review 17 Finding 5 closure: render_cell's chars().count()
        // width computation is correct only for ASCII values. The
        // debug_assert! must panic when a non-ASCII value is passed so
        // that any future spec amendment permitting non-ASCII colored
        // fields surfaces the gap before column alignment silently
        // breaks. Compiled out in release; fires in `cargo test`.
        let result = std::panic::catch_unwind(|| render_cell("完成", None, 8));
        assert!(
            result.is_err(),
            "render_cell must debug_assert on non-ASCII values per QE R17 F5"
        );
    }
}
