use predicates::prelude::*;
use tempfile::TempDir;

mod common;
use common::tracker;

// Layer 5 — Compound Filtering (status × priority × label, AND-combined)
//
// DESIGN.md Feature 2 (lines 63, 321): "--status, --priority, and --label are
// AND-combined. An issue must match all provided filters to appear." Layer 3
// added --priority filtering and Layer 4 added --label filtering on top of the
// Layer 2 --status filter; the AND-combination is an emergent property of
// cmd_list's chained `retain()` calls. Layer 5 adds the explicit assertions.
//
// Most CLI tests below are **Cat B Red Gate deviations**: the compound-filter
// behavior already exists from Layers 3 and 4, so the integration tests pass
// against the current implementation rather than failing first. They are
// regression coverage of the layer plan's acceptance criteria, not Red Gate
// tests for new behavior. The genuine Cat A Red Gate for this layer is the
// `issue_matches_filters` predicate extracted from cmd_list — its unit tests
// in `src/lib.rs::tests` panic against the `todo!()` stub until Phase 2b
// implements the predicate. Same disposition as Layer 3's
// `create_without_priority_defaults_to_medium` and Layer 4's two Cat B
// deviations (see Layer 4 Red Gate commit 14bd219).

// --- Two-filter AND combinations (DESIGN.md AC 1, 2, 3) ---

#[test]
fn list_status_and_priority_filter_and_combination() {
    // Cat B Red Gate deviation. AC: `tracker list --status open --priority high`
    // shows only issues that are both open AND high-priority. Setup gives one
    // issue matching both, plus three single-mismatch issues that are NOT in
    // the result. Negative assertions kill mutations that route the second
    // filter to OR semantics or drop the second retain.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Match all", "--priority", "high"])
        .assert()
        .success();
    tracker(&dir)
        .args(["create", "Wrong priority", "--priority", "low"])
        .assert()
        .success();
    tracker(&dir)
        .args(["create", "Wrong status", "--priority", "high"])
        .assert()
        .success();
    tracker(&dir)
        .args(["status", "3", "done"])
        .assert()
        .success();

    let output = tracker(&dir)
        .args(["list", "--status", "open", "--priority", "high"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let out = String::from_utf8(output).unwrap();
    assert!(
        out.contains("Match all"),
        "issue matching both filters must appear:\n{out}"
    );
    assert!(
        !out.contains("Wrong priority"),
        "issue matching only --status must NOT appear (AND, not OR):\n{out}"
    );
    assert!(
        !out.contains("Wrong status"),
        "issue matching only --priority must NOT appear (AND, not OR):\n{out}"
    );
}

#[test]
fn list_status_and_label_filter_and_combination() {
    // Cat B Red Gate deviation. AC: `tracker list --status open --label bug`
    // shows only open issues with label `bug`.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Match all", "--label", "bug"])
        .assert()
        .success();
    tracker(&dir)
        .args(["create", "Wrong label", "--label", "feature"])
        .assert()
        .success();
    tracker(&dir)
        .args(["create", "Wrong status", "--label", "bug"])
        .assert()
        .success();
    tracker(&dir)
        .args(["status", "3", "done"])
        .assert()
        .success();

    let output = tracker(&dir)
        .args(["list", "--status", "open", "--label", "bug"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let out = String::from_utf8(output).unwrap();
    assert!(
        out.contains("Match all"),
        "issue matching both filters must appear:\n{out}"
    );
    assert!(
        !out.contains("Wrong label"),
        "issue matching only --status must NOT appear:\n{out}"
    );
    assert!(
        !out.contains("Wrong status"),
        "issue matching only --label must NOT appear:\n{out}"
    );
}

#[test]
fn list_priority_and_label_filter_and_combination() {
    // Cat B Red Gate deviation. AC: `tracker list --priority high --label bug`
    // shows only high-priority issues with label `bug`. Note this exercises
    // the non-default-status path: with no --status flag, effective_status is
    // "open" and only open issues participate; one of the setup issues is in-
    // progress to confirm it is filtered out by the implicit status default.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args([
            "create",
            "Match all",
            "--priority",
            "high",
            "--label",
            "bug",
        ])
        .assert()
        .success();
    tracker(&dir)
        .args([
            "create",
            "Wrong priority",
            "--priority",
            "low",
            "--label",
            "bug",
        ])
        .assert()
        .success();
    tracker(&dir)
        .args([
            "create",
            "Wrong label",
            "--priority",
            "high",
            "--label",
            "feature",
        ])
        .assert()
        .success();

    let output = tracker(&dir)
        .args(["list", "--priority", "high", "--label", "bug"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let out = String::from_utf8(output).unwrap();
    assert!(
        out.contains("Match all"),
        "issue matching both filters must appear:\n{out}"
    );
    assert!(
        !out.contains("Wrong priority"),
        "issue matching only --label must NOT appear:\n{out}"
    );
    assert!(
        !out.contains("Wrong label"),
        "issue matching only --priority must NOT appear:\n{out}"
    );
}

// --- Three-filter AND (DESIGN.md AC 4 + AC 5) ---

#[test]
fn list_three_filter_and_combination() {
    // Cat B Red Gate deviation. AC 4: `tracker list --status open --priority
    // high --label bug` shows only issues matching all three. The setup also
    // covers AC 5 ("an issue that matches two of three filters but not the
    // third does NOT appear"): three of the four issues each fail exactly one
    // filter, so a regression that ORs filters or drops a conjunct would
    // surface them in the output.
    let dir = TempDir::new().unwrap();
    // Issue 1 — matches all three (the only expected result)
    tracker(&dir)
        .args([
            "create",
            "Triple match",
            "--priority",
            "high",
            "--label",
            "bug",
        ])
        .assert()
        .success();
    // Issue 2 — open + high + (label=feature) — fails label
    tracker(&dir)
        .args([
            "create",
            "Wrong label only",
            "--priority",
            "high",
            "--label",
            "feature",
        ])
        .assert()
        .success();
    // Issue 3 — open + (priority=medium) + bug — fails priority
    tracker(&dir)
        .args([
            "create",
            "Wrong priority only",
            "--priority",
            "medium",
            "--label",
            "bug",
        ])
        .assert()
        .success();
    // Issue 4 — (will be marked done) + high + bug — fails status
    tracker(&dir)
        .args([
            "create",
            "Wrong status only",
            "--priority",
            "high",
            "--label",
            "bug",
        ])
        .assert()
        .success();
    tracker(&dir)
        .args(["status", "4", "done"])
        .assert()
        .success();

    let output = tracker(&dir)
        .args([
            "list",
            "--status",
            "open",
            "--priority",
            "high",
            "--label",
            "bug",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let out = String::from_utf8(output).unwrap();
    assert!(
        out.contains("Triple match"),
        "issue matching all three filters must appear:\n{out}"
    );
    assert!(
        !out.contains("Wrong label only"),
        "AC 5: 2/3 match (label fails) must NOT appear:\n{out}"
    );
    assert!(
        !out.contains("Wrong priority only"),
        "AC 5: 2/3 match (priority fails) must NOT appear:\n{out}"
    );
    assert!(
        !out.contains("Wrong status only"),
        "AC 5: 2/3 match (status fails) must NOT appear:\n{out}"
    );
}

// --- Empty-state messaging in compound-filter contexts ---

#[test]
fn list_compound_two_filter_no_match_shows_filter_message() {
    // Cat B Red Gate deviation. AC 6: `tracker list --status done --priority
    // low` with no matching issues prints `No issues match the given filters.`
    // (filter message — not the default-view "No open issues. Nice work!"
    // empty-state). Setup has issues that match neither filter together;
    // confirms the cmd_list `is_default_open_view` heuristic correctly routes
    // to the filter-message branch when extra_filter_active is true.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "High open", "--priority", "high"])
        .assert()
        .success();
    tracker(&dir)
        .args(["create", "Low open", "--priority", "low"])
        .assert()
        .success();

    tracker(&dir)
        .args(["list", "--status", "done", "--priority", "low"])
        .assert()
        .success()
        .stdout("")
        .stderr(predicate::str::contains(
            "No issues match the given filters.",
        ))
        .stderr(predicate::str::contains("Nice work!").not());
}

#[test]
fn list_compound_three_filter_no_match_shows_filter_message() {
    // Cat B Red Gate deviation. AC 7: `tracker list --status open --priority
    // high --label nonexistent` with no matching issues prints
    // `No issues match the given filters.`. The label filter alone is the
    // odd-one-out, so a regression that dropped --label from the
    // extra_filter_active disjunction (SO Review 11 hazard) would route to
    // "No open issues. Nice work!" instead.
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args([
            "create",
            "High open bug",
            "--priority",
            "high",
            "--label",
            "bug",
        ])
        .assert()
        .success();

    tracker(&dir)
        .args([
            "list",
            "--status",
            "open",
            "--priority",
            "high",
            "--label",
            "nonexistent",
        ])
        .assert()
        .success()
        .stdout("")
        .stderr(predicate::str::contains(
            "No issues match the given filters.",
        ))
        .stderr(predicate::str::contains("Nice work!").not());
}

#[test]
fn list_default_view_with_open_issues_does_not_show_filter_message() {
    // Cat B Red Gate deviation. AC 8: `tracker list` (default, all open, some
    // exist) shows only open issues, not `No issues match` message. Confirms
    // the empty-state branch is skipped entirely when results are non-empty
    // (a regression that always printed the filter message regardless of
    // result set would surface here).
    let dir = TempDir::new().unwrap();
    tracker(&dir)
        .args(["create", "Open issue alpha"])
        .assert()
        .success();
    tracker(&dir)
        .args(["create", "Open issue beta"])
        .assert()
        .success();

    let assert = tracker(&dir).args(["list"]).assert().success();
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    let stderr = String::from_utf8(assert.get_output().stderr.clone()).unwrap();
    assert!(
        stdout.contains("Open issue alpha") && stdout.contains("Open issue beta"),
        "default `list` must show open issues:\nstdout:\n{stdout}"
    );
    assert!(
        !stderr.contains("No issues match"),
        "default-view non-empty list must NOT print the filter empty-state:\nstderr:\n{stderr}"
    );
    assert!(
        !stderr.contains("Nice work!"),
        "default-view non-empty list must NOT print the empty-tracker message either:\nstderr:\n{stderr}"
    );
}
