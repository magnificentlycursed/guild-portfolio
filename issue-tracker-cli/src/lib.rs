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
        && issue.labels.iter().all(|l| !l.trim().is_empty())
        && issue
            .description
            .as_ref()
            .is_none_or(|d| !d.trim().is_empty())
        && parse_timestamp(&issue.created_at).is_some()
        && parse_timestamp(&issue.updated_at).is_some()
        && issue.updated_at >= issue.created_at
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

/// Implements `tracker create "<title>" [--priority <p>] [--label <l>]...`.
///
/// Validates the title, optional priority, and each label; assigns the next ID;
/// appends the new issue to storage; and prints `Created issue #<id>: <title>`
/// to stdout. Priority defaults to `medium` when not supplied. Labels are
/// trimmed individually and deduplicated (first occurrence preserved,
/// case-sensitive).
///
/// # Errors
/// Returns `Err` if the title is empty/whitespace, the priority is invalid,
/// any label is empty after trim, stored data is unreadable or corrupt, the ID
/// space is exhausted, or persisting the new issue fails.
pub fn cmd_create(
    title_raw: &str,
    priority_raw: Option<&str>,
    labels_raw: &[String],
    issues_path: &Path,
) -> Result<(), String> {
    let title = validate_title(title_raw)?;
    let priority = match priority_raw {
        Some(p) => parse_priority(p)?,
        None => "medium".to_string(),
    };
    let parsed_labels: Vec<String> = labels_raw
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
        description: None,
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
            raw
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
            raw
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
            raw
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
/// label is empty after trimming.
///
/// # Errors
/// Returns `Err("Label cannot be empty.")` when `raw` is empty or whitespace-only.
pub fn parse_label(raw: &str) -> Result<String, String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        Err("Label cannot be empty.".to_string())
    } else {
        Ok(trimmed.to_string())
    }
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
    // Disjunction over non-default filters: any future filter (e.g. Layer 6's
    // `--description-contains`) must extend `extra_filter_active` here — a single
    // location — rather than appending another `&& *_filter.is_none()` conjunct
    // to the empty-state predicate. Reduces the SO Review 11 regression hazard:
    // the structural fragility of the positive-enumeration form is what made the
    // earlier empty-state heuristic break when `--priority` was added in Layer 3
    // and again when `--label` was added in Layer 4. SA Review 9 Finding 2.
    let extra_filter_active = effective_priority.is_some() || label_filter.is_some();
    let is_default_open_view = effective_status == "open" && !extra_filter_active;

    let mut issues = load_issues(issues_path)?;
    issues.retain(|i| i.status == effective_status);
    if let Some(p) = &effective_priority {
        issues.retain(|i| &i.priority == p);
    }
    if let Some(l) = label_filter {
        issues.retain(|i| label_matches(&i.labels, l));
    }

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
}
