//! Command implementations + rendering layer.
//!
//! Each `cmd_*` function corresponds to one CLI subcommand and is the
//! orchestration point between storage I/O (load_tracker / save_tracker),
//! input validation (validate / parse helpers from `crate::validate`),
//! pure filter/sort/format helpers (this module), and stdout/stderr
//! emission.
//!
//! Rendering helpers (`format_show_block`, `format_list_header`,
//! `format_list_row`, `render_cell`, `wrap_color`, `priority_ansi`,
//! `status_ansi`, `show_label`) are pure functions modulo color
//! application — color injection is parameterized via `ColorMode`,
//! decided once at process start (`color_mode_from_env`) by `main.rs`
//! and threaded through. This single-decision-point pattern was
//! established by SE R17 F1 / SA R15 F2 closure.
//!
//! Color-output contract per DESIGN.md "Interface / Color output":
//! every highlighted value carries the `bold` SGR attribute for WCAG
//! 1.4.1 compliance; default-color values (`low`, `open`) remain plain;
//! ANSI emission is never present on a non-TTY stdout (pipe-cleanness
//! contract — `CLICOLOR_FORCE` is deliberately not honored).
//!
//! Module split per SA R13 F1 Trigger B closure. The `TRACKER_INTERNAL_FORCE_COLOR`
//! test seam (QE R17 F1 closure) lives in `color_mode_from_env`; the
//! `wrap_color` and `render_cell` debug_assert! contracts (Security R11
//! F1 + QE R17 F5 closures) live in this module.

use std::io::IsTerminal;
use std::path::Path;

use crate::storage::{load_tracker, save_tracker, Issue, PRIORITY_ORDER};
use crate::validate::{
    bump_next_id, current_timestamp, dedupe_labels, parse_id, parse_label, parse_priority,
    parse_status, validate_description, validate_title,
};

// --- Color helpers + constants ---

const ANSI_RESET: &str = "\x1b[0m";

/// Width of the `ID` column in `tracker list` output.
pub(crate) const ID_WIDTH: usize = 4;
/// Width of the `Status` column in `tracker list` output. Sized for
/// `"in-progress"` (11 chars), the widest legal value.
pub(crate) const STATUS_WIDTH: usize = 11;
/// Width of the `Priority` column in `tracker list` output. Sized for
/// `"medium"` (6 chars) plus 2 chars of trailing padding for visual
/// breathing room before the next column.
pub(crate) const PRIORITY_WIDTH: usize = 8;
/// Width of the `Labels` column in `tracker list` output. Truncates with
/// an ellipsis (`...`) past this many chars.
pub(crate) const LABELS_WIDTH: usize = 20;
/// Width of the `Title` column in `tracker list` output. Truncates with
/// an ellipsis past this many chars. The title column is the right-most
/// column and is not padded — the constant governs only the truncation
/// threshold.
pub(crate) const TITLE_WIDTH: usize = 50;
/// Width of the labelled-key column in `tracker show` output. The format
/// `Description:` is the longest legal label (12 chars + trailing colon
/// space → 13 total).
pub(crate) const LABEL_COLUMN_WIDTH: usize = 13;

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
pub(crate) fn priority_ansi(priority: &str, color: ColorMode) -> Option<&'static str> {
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
pub(crate) fn status_ansi(status: &str, color: ColorMode) -> Option<&'static str> {
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
pub(crate) fn wrap_color(value: &str, ansi: Option<&str>) -> String {
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
pub(crate) fn render_cell(value: &str, ansi: Option<&str>, total_width: usize) -> String {
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

// --- CreateArgs + cmd_create + cmd_status ---

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

// --- show + delete ---

/// Renders a single issue as the `tracker show` labelled key-value block.
///
/// Per DESIGN.md "Show output format": each label is right-padded to a fixed
/// width of `LABEL_COLUMN_WIDTH` (= 13) characters so values align. For
/// multi-line descriptions, the first line follows the `Description:` label;
/// each continuation line is indented by `LABEL_COLUMN_WIDTH` spaces.
///
/// Returns the formatted block including a trailing newline.
pub(crate) fn format_show_block(issue: &Issue, color: ColorMode) -> String {
    let labels_display = if issue.labels.is_empty() {
        "(none)".to_string()
    } else {
        issue.labels.join(", ")
    };
    let continuation_indent = format!("\n{:<width$}", "", width = LABEL_COLUMN_WIDTH);
    let description_display = match &issue.description {
        None => "(none)".to_string(),
        Some(d) => {
            let normalized = d.replace("\r\n", "\n");
            normalized.replace('\n', &continuation_indent)
        }
    };
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
pub(crate) fn show_label(name: &str) -> String {
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

// --- list + filter / sort / render row helpers ---

/// Sort rank for `p`: index in `PRIORITY_ORDER` (high=0, medium=1, low=2).
///
/// Returns `usize::MAX` for unknown values as a defensive fallback. The fallback
/// is unreachable for stored data: `issue_fields_are_valid` rejects priorities
/// outside `PRIORITY_ORDER` at load time. Routing an unrecognized priority to the
/// bottom of sort order is preferable to panicking on an internal-only path.
pub(crate) fn priority_rank(p: &str) -> usize {
    PRIORITY_ORDER
        .iter()
        .position(|&x| x == p)
        .unwrap_or(usize::MAX)
}

/// Sorts issues by priority (high → medium → low) then by ID ascending.
pub fn sort_issues(issues: &mut [Issue]) {
    issues.sort_by(|a, b| {
        priority_rank(&a.priority)
            .cmp(&priority_rank(&b.priority))
            .then(a.id.cmp(&b.id))
    });
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
pub(crate) fn issue_matches_filters(
    issue: &Issue,
    status: &str,
    priority: Option<&str>,
    label: Option<&str>,
) -> bool {
    issue.status == status
        && priority.is_none_or(|p| issue.priority == p)
        && label.is_none_or(|l| label_matches(&issue.labels, l))
}

pub(crate) fn truncate_with_ellipsis(s: &str, max_chars: usize) -> String {
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
pub(crate) fn filter_issues(
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
pub(crate) fn format_list_header() -> String {
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
pub(crate) fn format_list_row(issue: &Issue, color: ColorMode) -> String {
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
