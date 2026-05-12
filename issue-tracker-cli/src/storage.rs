//! Storage layer: data types persisted to `tracker.json` and the load-time
//! invariant checking that validates them.
//!
//! Public types (`Tracker`, `Issue`) are the serde-Deserialize targets for
//! the on-disk JSON shape. Load-time invariants (`tracker_is_valid` and the
//! per-record `issue_fields_are_valid` chain) treat all loaded data as
//! untrusted: a hand-edited `tracker.json` with unknown status values,
//! malformed timestamps, duplicate IDs, or a `next_id` that doesn't strictly
//! exceed `max(issue.id)` is rejected with the standard corrupt-data error.
//!
//! Module split per SA R13 F1 Trigger B closure: previously these items
//! lived alongside command implementations and user-input validators in
//! `src/lib.rs`. The split separates the "data model + load-time
//! invariants" concern from "user-input validation" (`validate.rs`) and
//! "command implementations + rendering" (`commands.rs`).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

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
pub(crate) struct Tracker {
    /// All currently-stored issues. Order matches insertion order at write time;
    /// `cmd_list` sorts a working copy before rendering.
    pub(crate) issues: Vec<Issue>,
    /// The next ID to be assigned by `cmd_create`. Initialized to `1` for a fresh
    /// tracker; bumped via `checked_add(1)` on every create; never decreased by
    /// delete. Invariant at load: `next_id >= 1` and (if `issues` is non-empty)
    /// `next_id > max(issue.id)`.
    pub(crate) next_id: u64,
}

/// A single tracked issue, as stored in `tracker.json`.
///
/// All fields except `description` are required. `description` is omitted from
/// the JSON output when absent (`None`) rather than serialized as `null`.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Issue {
    /// Unique, monotonically-assigned positive integer; never reused (see `Tracker::next_id`).
    pub(crate) id: u64,
    /// Trimmed, non-empty issue title.
    pub(crate) title: String,
    /// Optional free-form description; stored verbatim (not trimmed). The JSON key
    /// is omitted entirely when `None`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) description: Option<String>,
    /// One of: `"open"`, `"in-progress"`, `"done"` (lowercase).
    pub(crate) status: String,
    /// One of: `"low"`, `"medium"`, `"high"` (lowercase).
    pub(crate) priority: String,
    /// Deduplicated, case-preserved labels in the order they were supplied at
    /// creation; may be empty.
    pub(crate) labels: Vec<String>,
    /// ISO 8601 UTC timestamp at second precision (e.g. `"2026-04-27T14:00:00Z"`);
    /// fixed at creation and never modified.
    pub(crate) created_at: String,
    /// ISO 8601 UTC timestamp at second precision; refreshed on every mutation.
    /// Always `>= created_at`.
    pub(crate) updated_at: String,
}

/// Standard error message for any load-time corrupt-data condition.
pub(crate) const CORRUPT_DATA_ERROR: &str =
    "Could not read tracker data. The file may be corrupt. Delete tracker.json to start fresh.";

/// Valid status values (case-sensitive, lowercase). Single source of truth used
/// by both input validation (`validate::parse_status`) and load-time invariant
/// checking (`issue_fields_are_valid`).
pub(crate) const VALID_STATUSES: &[&str] = &["open", "in-progress", "done"];

/// Priority values in sort order (highest first). Single source of truth for both
/// validity (membership — `validate::parse_priority` + `issue_fields_are_valid`)
/// and sort rank (index — `commands::priority_rank`).
pub(crate) const PRIORITY_ORDER: &[&str] = &["high", "medium", "low"];

/// Parses an ISO 8601 / RFC 3339 timestamp string. Returns `None` on any parse failure.
///
/// Accepts the same shapes `chrono::DateTime::parse_from_rfc3339` accepts; the
/// project produces second-precision UTC strings (e.g. `"2026-04-27T14:00:00Z"`)
/// but a stored file from a future schema may include sub-second precision or
/// offsets — those parse successfully and are accepted as valid timestamps.
pub(crate) fn parse_timestamp(s: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(s)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
}

/// Per-record validation: domain values, label hygiene, timestamp parseability,
/// and the `updated_at >= created_at` invariant.
///
/// Whole-tracker invariants (ID uniqueness, `next_id` counter constraints) are
/// enforced separately by `tracker_is_valid`.
pub(crate) fn issue_fields_are_valid(issue: &Issue) -> bool {
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
pub(crate) fn description_is_valid(description: &str) -> bool {
    !description.trim().is_empty() && !description.chars().any(|c| c.is_control() && c != '\n')
}

/// Stored-label hygiene predicate. Stored labels are post-trim; this predicate
/// checks the same hygiene rules `parse_label` enforces at the input boundary,
/// so a hand-edited `tracker.json` with a label that bypassed `parse_label`
/// (control character, comma, or whitespace-only) is rejected at load.
pub(crate) fn label_is_valid(label: &str) -> bool {
    !label.trim().is_empty() && !label.chars().any(char::is_control) && !label.contains(',')
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
pub(crate) fn tracker_is_valid(tracker: &Tracker) -> bool {
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
pub(crate) fn load_tracker(path: &Path) -> Result<Tracker, String> {
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
pub(crate) fn save_tracker(path: &Path, tracker: &Tracker) -> Result<(), String> {
    #[allow(clippy::unwrap_used)]
    // Tracker is always serializable: no floats, no cycles, all fields implement Serialize
    let contents = serde_json::to_string_pretty(tracker).unwrap();
    fs::write(path, contents).map_err(|e| format!("Could not save tracker data: {}.", e))
}
