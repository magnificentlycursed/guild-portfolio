//! User-input validation and safety transforms.
//!
//! Validators (`validate_title`, `validate_description`, `parse_status`,
//! `parse_priority`, `parse_label`, `parse_id`) operate at the CLI input
//! boundary: they check user-supplied strings against the spec's domain
//! contracts and return normalized values (trimmed titles/labels;
//! lowercase status/priority; positive u64 IDs). Each validator's error
//! message uses `display_safe` to Cc-escape any interpolated user value
//! so a pasted ANSI sequence cannot cross the stderr → terminal boundary.
//!
//! `display_safe` and `sanitize_quoted_values` are the stderr-safety
//! transforms — applied across application errors (`display_safe`) and
//! clap argument-parsing errors (`sanitize_quoted_values`) per DESIGN.md
//! "stderr contract" + RT R10 F1 closure.
//!
//! `bump_next_id` and `current_timestamp` are arithmetic / time helpers
//! used by `commands::cmd_create` and `commands::cmd_status`.
//!
//! Module split per SA R13 F1 Trigger B closure. Constants
//! (`VALID_STATUSES`, `PRIORITY_ORDER`) live in `storage.rs` as the
//! single source of truth for the domain enums — validators import them.

use chrono::Utc;
use std::collections::HashSet;

use crate::storage::{PRIORITY_ORDER, VALID_STATUSES};

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
pub(crate) fn validate_title(raw: &str) -> Result<String, String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err("Title cannot be empty.".to_string());
    }
    if trimmed.chars().any(char::is_control) {
        return Err("Title cannot contain control characters.".to_string());
    }
    Ok(trimmed.to_string())
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
pub(crate) fn validate_description(raw: &str) -> Result<String, String> {
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

/// Parses and normalizes a status string (case-insensitive).
///
/// Returns the canonical lowercase status value, or an error describing the valid values.
///
/// # Errors
/// Returns `Err` if `raw` is not (case-insensitively) one of `open`, `in-progress`, `done`.
pub(crate) fn parse_status(raw: &str) -> Result<String, String> {
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

/// Parses and normalizes a priority string (case-insensitive).
///
/// Returns the canonical lowercase priority value, or an error describing the valid values.
///
/// # Errors
/// Returns `Err` if `raw` is not (case-insensitively) one of `low`, `medium`, `high`.
pub(crate) fn parse_priority(raw: &str) -> Result<String, String> {
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
pub(crate) fn parse_label(raw: &str) -> Result<String, String> {
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

/// Parses an issue ID from a string. Must be a positive integer (>= 1).
///
/// # Errors
/// Returns `Err` if `raw` does not parse as a positive `u64` (`0`, negative, non-numeric, overflow).
pub(crate) fn parse_id(raw: &str) -> Result<u64, String> {
    raw.parse::<u64>().ok().filter(|&n| n > 0).ok_or_else(|| {
        format!(
            "'{}' is not a valid issue ID. Expected a positive integer.",
            display_safe(raw)
        )
    })
}

/// Returns `labels` with duplicates removed; first occurrence preserved.
///
/// Comparison is case-sensitive: `"bug"` and `"Bug"` are distinct labels.
pub(crate) fn dedupe_labels(labels: &[String]) -> Vec<String> {
    let mut seen = HashSet::with_capacity(labels.len());
    let mut out = Vec::with_capacity(labels.len());
    for label in labels {
        if seen.insert(label.as_str()) {
            out.push(label.clone());
        }
    }
    out
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
pub(crate) fn bump_next_id(current: u64) -> Result<u64, String> {
    current
        .checked_add(1)
        .ok_or_else(|| "Cannot assign new issue ID: maximum ID reached.".to_string())
}

/// Returns the current UTC time as an ISO 8601 string at second precision (e.g. `"2026-04-27T14:00:00Z"`).
pub(crate) fn current_timestamp() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
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
pub(crate) fn display_safe(s: &str) -> String {
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
