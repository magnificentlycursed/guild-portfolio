#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::missing_errors_doc
)]

//! Library crate for the `tracker` issue-tracker CLI.
//!
//! Module split (SA R13 F1 Trigger B closure):
//!
//! - [`storage`] — data types (`Tracker`, `Issue`), persistence
//!   (`load_tracker`, `save_tracker`), and load-time invariants
//!   (`tracker_is_valid`, `issue_fields_are_valid`).
//! - [`validate`] — user-input validators (`validate_title`,
//!   `validate_description`, `parse_status`, `parse_priority`,
//!   `parse_label`, `parse_id`), arithmetic + time helpers
//!   (`bump_next_id`, `current_timestamp`), and stderr-safety
//!   transforms (`display_safe`, `sanitize_quoted_values`).
//! - [`commands`] — command implementations (`cmd_create`, `cmd_list`,
//!   `cmd_status`, `cmd_show`, `cmd_delete`), `CreateArgs`, and the
//!   rendering / color layer (`ColorMode`, `color_mode_from_env`,
//!   `format_show_block`, `format_list_row`, etc.).
//!
//! Selective `pub use` re-exports below preserve the pre-split public API
//! surface (`tracker::cmd_create`, `tracker::Tracker`, `tracker::ColorMode`,
//! etc.) so the `tracker` binary in `src/main.rs` and the integration tests
//! in `tests/` need no changes when the modules move.
//!
//! All public functions return `Result<T, String>` where the `Err` variant is
//! the user-facing error message (without an `Error: ` prefix — `main.rs` adds
//! it). See `DESIGN.md` for the full behavioral contract.

pub mod commands;
pub mod storage;
pub mod validate;

// Public API re-exports. main.rs and integration tests consume these
// unqualified (e.g., `tracker::cmd_create`, `tracker::CreateArgs`,
// `tracker::ColorMode`); the underlying modules are also accessible
// directly for crate-internal navigation (e.g., docs).
pub use commands::{
    cmd_create, cmd_delete, cmd_list, cmd_show, cmd_status, color_mode_from_env, label_matches,
    sort_issues, ColorMode, CreateArgs,
};
pub use storage::{load_tracker, save_tracker, Issue, Tracker};
pub use validate::{
    bump_next_id, current_timestamp, dedupe_labels, display_safe, parse_id, parse_label,
    parse_priority, parse_status, sanitize_quoted_values, validate_description, validate_title,
};
#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::*;
    use crate::storage::*;
    // validate's public items are already re-exported by `pub use validate::{...}`
    // at the lib.rs hub level; `use super::*` brings them in here. No
    // crate-internal items live in validate that aren't re-exported, so a
    // `use crate::validate::*;` here would be a redundant glob.

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
