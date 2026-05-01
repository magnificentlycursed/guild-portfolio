#![deny(clippy::unwrap_used)]

use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// A single tracked issue, as stored in `tracker.json`.
///
/// All fields except `description` are required. `description` is omitted from
/// the JSON output when absent (`None`) rather than serialized as `null`.
#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
    pub id: u64,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub labels: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Trims `raw` and returns the trimmed title, or an error if it is empty after trimming.
pub fn validate_title(raw: &str) -> Result<String, String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        Err("Title cannot be empty.".to_string())
    } else {
        Ok(trimmed.to_string())
    }
}

/// Returns `max(existing_ids) + 1`, or `1` if the slice is empty.
///
/// IDs are never reused: the next ID is always strictly greater than all existing IDs,
/// including those of deleted issues.
pub fn next_id(existing_ids: &[u64]) -> u64 {
    existing_ids.iter().max().copied().unwrap_or(0) + 1
}

/// Returns the current UTC time as an ISO 8601 string at second precision (e.g. `"2026-04-27T14:00:00Z"`).
pub fn current_timestamp() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

const CORRUPT_DATA_ERROR: &str =
    "Could not read tracker data. The file may be corrupt. Delete tracker.json to start fresh.";

const VALID_STATUSES: &[&str] = &["open", "in-progress", "done"];
const VALID_PRIORITIES: &[&str] = &["low", "medium", "high"];

fn issue_fields_are_valid(issue: &Issue) -> bool {
    issue.id > 0
        && !issue.title.trim().is_empty()
        && VALID_STATUSES.contains(&issue.status.as_str())
        && VALID_PRIORITIES.contains(&issue.priority.as_str())
}

/// Loads all issues from `path`.
///
/// Returns an empty `Vec` if the file does not exist. Returns `Err` if the file
/// cannot be read, contains malformed JSON, or contains issues with invalid domain
/// values (e.g. unknown status, zero ID, empty title).
pub fn load_issues(path: &Path) -> Result<Vec<Issue>, String> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let contents =
        fs::read_to_string(path).map_err(|e| format!("Could not read tracker data: {}.", e))?;
    let issues: Vec<Issue> =
        serde_json::from_str(&contents).map_err(|_| CORRUPT_DATA_ERROR.to_string())?;
    if issues.iter().any(|i| !issue_fields_are_valid(i)) {
        return Err(CORRUPT_DATA_ERROR.to_string());
    }
    Ok(issues)
}

/// Serializes `issues` as pretty-printed JSON and writes it to `path`.
pub fn save_issues(path: &Path, issues: &[Issue]) -> Result<(), String> {
    #[allow(clippy::unwrap_used)]
    // Vec<Issue> is always serializable: no floats, no cycles, all fields implement Serialize
    let contents = serde_json::to_string_pretty(issues).unwrap();
    fs::write(path, contents).map_err(|e| format!("Could not save tracker data: {}.", e))
}

/// Implements `tracker create "<title>"`.
///
/// Validates the title, assigns the next ID, appends the new issue to storage,
/// and prints `Created issue #<id>: <title>` to stdout.
pub fn cmd_create(title_raw: &str, issues_path: &Path) -> Result<(), String> {
    let title = validate_title(title_raw)?;
    let mut issues = load_issues(issues_path)?;
    let ids: Vec<u64> = issues.iter().map(|i| i.id).collect();
    let id = next_id(&ids);
    let now = current_timestamp();
    issues.push(Issue {
        id,
        title: title.clone(),
        description: None,
        status: "open".to_string(),
        priority: "medium".to_string(),
        labels: Vec::new(),
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
pub fn parse_status(raw: &str) -> Result<String, String> {
    match raw.to_lowercase().as_str() {
        "open" | "in-progress" | "done" => Ok(raw.to_lowercase()),
        _ => Err(format!(
            "Invalid status '{}'. Expected: open, in-progress, or done.",
            raw
        )),
    }
}

/// Parses an issue ID from a string. Must be a positive integer (>= 1).
pub fn parse_id(raw: &str) -> Result<u64, String> {
    raw.parse::<u64>().ok().filter(|&n| n > 0).ok_or_else(|| {
        format!(
            "'{}' is not a valid issue ID. Expected a positive integer.",
            raw
        )
    })
}

/// Implements `tracker status <id> <status>`.
pub fn cmd_status(id_raw: &str, status_raw: &str, issues_path: &Path) -> Result<(), String> {
    let id = parse_id(id_raw)?;
    let new_status = parse_status(status_raw)?;
    let mut issues = load_issues(issues_path)?;
    let issue = issues
        .iter_mut()
        .find(|i| i.id == id)
        .ok_or_else(|| format!("Issue #{} not found.", id))?;
    issue.status = new_status.clone();
    issue.updated_at = current_timestamp();
    save_issues(issues_path, &issues)?;
    println!("Issue #{} status \u{2192} {}.", id, new_status);
    Ok(())
}

const PRIORITY_ORDER: &[&str] = &["high", "medium", "low"];

fn priority_rank(p: &str) -> usize {
    PRIORITY_ORDER
        .iter()
        .position(|&x| x == p)
        .unwrap_or(usize::MAX)
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

/// Implements `tracker list [--status <s>]`.
///
/// Without `--status`: shows only `open` issues; prints `No open issues. Nice work!` when empty.
/// With `--status <s>`: validates and filters by that status; prints `No issues match the given
/// filters.` when empty — unless the effective filter is `open`, which keeps the original message.
pub fn cmd_list(status_filter: Option<&str>, issues_path: &Path) -> Result<(), String> {
    let effective_status = match status_filter {
        None => "open".to_string(),
        Some(s) => parse_status(s)?,
    };
    let is_open_view = effective_status == "open";

    let mut issues = load_issues(issues_path)?;
    issues.retain(|i| i.status == effective_status);

    if issues.is_empty() {
        if is_open_view {
            println!("No open issues. Nice work!");
        } else {
            println!("No issues match the given filters.");
        }
        return Ok(());
    }

    issues.sort_by(|a, b| {
        priority_rank(&a.priority)
            .cmp(&priority_rank(&b.priority))
            .then(a.id.cmp(&b.id))
    });

    println!(
        "{:<4} {:<11} {:<8} {:<20} Title",
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
            "{:<4} {:<11} {:<8} {:<20} {}",
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
    fn id_assignment_first_issue_is_1() {
        assert_eq!(next_id(&[]), 1);
    }

    #[test]
    fn id_assignment_increments_from_max() {
        assert_eq!(next_id(&[1, 3, 5]), 6);
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
}
