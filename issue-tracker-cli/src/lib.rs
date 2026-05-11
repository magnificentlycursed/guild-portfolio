#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::missing_errors_doc
)]

//! Library crate for the `tracker` issue-tracker CLI.
//!
//! This crate exposes the data model (`Issue`), the command implementations
//! (`cmd_create`, `cmd_list`, `cmd_status`), the parsing/validation helpers
//! (`validate_title`, `parse_status`, `parse_priority`, `parse_id`), and the
//! storage primitives (`load_issues`, `save_issues`). The `tracker` binary in
//! `src/main.rs` wires `clap`-parsed arguments to these functions; integration
//! tests in `tests/` invoke the compiled binary as a subprocess.
//!
//! All public functions return `Result<T, String>` where the `Err` variant is
//! the user-facing error message (without an `Error: ` prefix — `main.rs` adds
//! it). See `DESIGN.md` for the full behavioral contract.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// A single tracked issue, as stored in `tracker.json`.
///
/// All fields except `description` are required. `description` is omitted from
/// the JSON output when absent (`None`) rather than serialized as `null`.
#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
    /// Unique, monotonically-assigned positive integer; never reused (see `next_id`).
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

/// Returns `max(existing_ids) + 1`, or `1` if the slice is empty.
///
/// IDs are never reused: the next ID is always strictly greater than all existing IDs,
/// including those of deleted issues.
///
/// # Errors
/// Returns `Err` if `max(existing_ids) == u64::MAX` (overflow). Unreachable through
/// organic use (the entire 64-bit ID space cannot be exhausted), but defends against
/// hand-edited `tracker.json` files that plant `u64::MAX` to corrupt subsequent writes.
pub fn next_id(existing_ids: &[u64]) -> Result<u64, String> {
    let max = existing_ids.iter().max().copied().unwrap_or(0);
    max.checked_add(1)
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
/// Cross-record invariants (ID uniqueness) are enforced separately by
/// `issues_collection_invariants_hold`.
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
fn display_safe(s: &str) -> String {
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

/// Cross-record invariants: every ID is unique across the collection.
///
/// `issue_fields_are_valid` covers per-record checks; this function covers what
/// can only be evaluated by walking the whole array. DESIGN.md "no two issues
/// share the same ID" is enforced here.
fn issues_collection_invariants_hold(issues: &[Issue]) -> bool {
    let mut seen = HashSet::with_capacity(issues.len());
    issues.iter().all(|i| seen.insert(i.id))
}

/// Loads all issues from `path`.
///
/// Returns an empty `Vec` if the file does not exist. All loaded data is treated
/// as untrusted: per-record validation (`issue_fields_are_valid`) and cross-record
/// invariants (`issues_collection_invariants_hold`) both apply.
///
/// # Errors
/// Returns `Err` if the file cannot be read, contains malformed JSON, contains a
/// record with invalid domain values (unknown status, zero ID, empty title, empty
/// label, empty description, malformed timestamp, `updated_at < created_at`), or
/// contains duplicate IDs across records.
pub fn load_issues(path: &Path) -> Result<Vec<Issue>, String> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let contents =
        fs::read_to_string(path).map_err(|e| format!("Could not read tracker data: {}.", e))?;
    let issues: Vec<Issue> =
        serde_json::from_str(&contents).map_err(|_| CORRUPT_DATA_ERROR.to_string())?;
    if issues.iter().any(|i| !issue_fields_are_valid(i))
        || !issues_collection_invariants_hold(&issues)
    {
        return Err(CORRUPT_DATA_ERROR.to_string());
    }
    Ok(issues)
}

/// Serializes `issues` as pretty-printed JSON and writes it to `path`.
///
/// # Errors
/// Returns `Err` if the file cannot be written (permission denied, disk full,
/// path is a directory, etc.). Serialization itself is infallible for `Vec<Issue>`.
pub fn save_issues(path: &Path, issues: &[Issue]) -> Result<(), String> {
    #[allow(clippy::unwrap_used)]
    // Vec<Issue> is always serializable: no floats, no cycles, all fields implement Serialize
    let contents = serde_json::to_string_pretty(issues).unwrap();
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
    let mut issues = load_issues(issues_path)?;
    let ids: Vec<u64> = issues.iter().map(|i| i.id).collect();
    let id = next_id(&ids)?;
    let now = current_timestamp();
    issues.push(Issue {
        id,
        title: title.clone(),
        description,
        status: "open".to_string(),
        priority,
        labels,
        created_at: now.clone(),
        updated_at: now,
    });
    save_issues(issues_path, &issues)?;
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
    let mut issues = load_issues(issues_path)?;
    let idx = issues
        .iter()
        .position(|i| i.id == id)
        .ok_or_else(|| format!("Issue #{} not found.", id))?;
    issues[idx].status = new_status;
    issues[idx].updated_at = current_timestamp();
    save_issues(issues_path, &issues)?;
    println!("Issue #{} status \u{2192} {}.", id, issues[idx].status);
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
/// width of 13 characters so values align. For multi-line descriptions, the
/// first line follows the `Description:` label; each continuation line is
/// indented by 13 spaces (matching the label-column width).
///
/// Returns the formatted block including a trailing newline.
fn format_show_block(issue: &Issue) -> String {
    let labels_display = if issue.labels.is_empty() {
        "(none)".to_string()
    } else {
        issue.labels.join(", ")
    };
    let description_display = match &issue.description {
        None => "(none)".to_string(),
        Some(d) => {
            // Multi-line descriptions: first line after the label, each
            // continuation line indented 13 spaces to match the label column.
            // `\r\n` sequences are normalized to `\n` for splitting so a
            // CRLF-stored description renders without a stray `\r` in the
            // first line. The 13-space continuation indent applies to every
            // line after the first regardless of original separator.
            let normalized = d.replace("\r\n", "\n");
            normalized.replace('\n', "\n             ")
        }
    };
    format!(
        "ID:          {}\n\
         Title:       {}\n\
         Status:      {}\n\
         Priority:    {}\n\
         Labels:      {}\n\
         Description: {}\n\
         Created:     {}\n\
         Updated:     {}\n",
        issue.id,
        issue.title,
        issue.status,
        issue.priority,
        labels_display,
        description_display,
        issue.created_at,
        issue.updated_at,
    )
}

/// Implements `tracker show <id>`.
///
/// Validates `id_raw`, locates the issue, and prints the full labelled
/// key-value block (per DESIGN.md "Show output format") to stdout. Show is
/// non-mutating: storage is read but never written.
///
/// # Errors
/// Returns `Err` if the ID is malformed, the issue does not exist, or
/// storage I/O fails.
pub fn cmd_show(id_raw: &str, issues_path: &Path) -> Result<(), String> {
    let id = parse_id(id_raw)?;
    let issues = load_issues(issues_path)?;
    let issue = issues
        .iter()
        .find(|i| i.id == id)
        .ok_or_else(|| format!("Issue #{} not found.", id))?;
    // `format_show_block` already includes a trailing newline; use `print!`
    // rather than `println!` to avoid emitting a stray blank line.
    print!("{}", format_show_block(issue));
    Ok(())
}

/// Implements `tracker delete <id>`.
///
/// Validates `id_raw`, locates the issue, removes it from storage, persists
/// the updated array, and prints `Deleted issue #<id>.` to stdout. Deleted
/// IDs are never reused: the next `create` assigns `max(remaining_ids) + 1`,
/// which is strictly greater than any deleted ID. Other issues are not
/// affected.
///
/// # Errors
/// Returns `Err` if the ID is malformed, the issue does not exist, or
/// storage I/O fails.
pub fn cmd_delete(id_raw: &str, issues_path: &Path) -> Result<(), String> {
    let id = parse_id(id_raw)?;
    let mut issues = load_issues(issues_path)?;
    let idx = issues
        .iter()
        .position(|i| i.id == id)
        .ok_or_else(|| format!("Issue #{} not found.", id))?;
    issues.remove(idx);
    save_issues(issues_path, &issues)?;
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

    let mut issues = load_issues(issues_path)?;
    issues.retain(|i| {
        issue_matches_filters(
            i,
            &effective_status,
            effective_priority.as_deref(),
            effective_label.as_deref(),
        )
    });

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

    println!(
        "{:<4}  {:<11}  {:<8}  {:<20}  Title",
        "ID", "Status", "Priority", "Labels"
    );

    for issue in &issues {
        let labels_raw = if issue.labels.is_empty() {
            "(none)".to_string()
        } else {
            issue.labels.join(", ")
        };
        let labels_display = truncate_with_ellipsis(&labels_raw, 20);
        let title_display = truncate_with_ellipsis(&issue.title, 50);
        println!(
            "{:<4}  {:<11}  {:<8}  {:<20}  {}",
            issue.id, issue.status, issue.priority, labels_display, title_display
        );
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
    fn id_assignment_first_issue_is_1() {
        assert_eq!(next_id(&[]), Ok(1));
    }

    #[test]
    fn id_assignment_increments_from_max() {
        assert_eq!(next_id(&[1, 3, 5]), Ok(6));
    }

    #[test]
    fn id_assignment_at_u64_max_returns_error() {
        // Defends against hand-edited tracker.json planting `id: u64::MAX` to
        // corrupt the next create. checked_add prevents the silent wrap to 0.
        assert!(next_id(&[u64::MAX]).is_err());
    }

    #[test]
    fn collection_invariants_reject_duplicate_ids() {
        let issues = vec![issue(1, "medium"), issue(1, "high")];
        assert!(!issues_collection_invariants_hold(&issues));
    }

    #[test]
    fn collection_invariants_accept_unique_ids() {
        let issues = vec![issue(1, "medium"), issue(2, "high")];
        assert!(issues_collection_invariants_hold(&issues));
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
        let out = format_show_block(&issue);
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
        let out = format_show_block(&issue);
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
    fn max_id_plus_one_skips_deleted_ids() {
        // DESIGN.md Feature 5 / Invariants: the deleted ID is never reused.
        // For an issue list with IDs [1, 3] (id=2 was deleted), the next ID
        // is max+1=4 — NOT 2 (sequential counter would re-use the gap).
        // Cat B Red Gate deviation: `next_id` already implements max+1 from
        // Layer 1 (`next_id(&[1,3])` returns 4); this test pins the behavior
        // for the Layer 6 delete-id-never-reused contract.
        assert_eq!(next_id(&[1, 3]), Ok(4));
        assert_eq!(next_id(&[2, 5, 9]), Ok(10));
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
}
