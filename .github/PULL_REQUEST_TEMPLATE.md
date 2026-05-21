<!--
This template is the canonical PR-validation surface for guild-portfolio.
Per the VSDD suite's [GitHub Actions supplement](vsdd-suite/supplements/github-actions.md) § PR template + merge-gate integration:

- The completion checklist below is MERGE-GATING — the .github/workflows/pr-checklist.yml workflow parses this PR body and fails CI if any `- [ ]` items remain unchecked at merge time.
- Test plan references the project's `manual-tests/layer-N.md` files directly. It does NOT duplicate their content (per Review 84 documentation-rot-via-duplication discipline).
- Spot-check items belong in the completion checklist with verifiable evidence linked, NOT in informal prose.

Keep the headings as-is. Remove or replace bracketed placeholders. Delete sections that don't apply (e.g., delete "## Operator-action queue" if this PR has no post-merge operator tasks).
-->

## Summary

<!-- One paragraph: what this PR does + why. Link to the relevant Review N in the suite-side audit trail and the bookmark-cli-manual project audit trail for full narrative. -->

## Audit trail references

<!-- Links to the canonical narratives:
- Suite-side Review N entry in vsdd-suite/suite-development/review-log/YYYY-MM-DD-suite-review.md
- Project-side review-log entries in <project>/vsdd-suite/review-log/YYYY-MM-DD-<domain-slug>.md (for project IAR cycle PRs)
- Suite CHANGELOG vX.Y.Z entry
- Project CHANGELOG vX.Y.Z entry (if a reference example was updated)
-->

## Test plan

<!--
References to the canonical manual-test contracts. Do NOT duplicate manual-tests/*.md content here.

For reference-example projects, every executed manual-test step is closed with a one-line note in the per-domain review-log file (per primer 3 § Manual testing is a second adversarial surface to IAR; G-132).

For suite-development PRs (no project artifact changes), the test plan covers pre-commit hook verification + audit-trail discipline checks.
-->

- Manual test plan executed: [link to the closure note in `<project>/vsdd-suite/review-log/YYYY-MM-DD-<domain-slug>.md` § Closure protocol per session]
- Install-verification: [link to `<project>/manual-tests/install-verification.md` Outcome row OR "operator-pending per G-155" if AI-unsatisfiable]
- CI workflow status: [link to the latest workflow run for the PR head]

## Completion checklist (merge-gating)

<!--
Every `- [ ]` item must be `- [x]` before merge. The .github/workflows/pr-checklist.yml workflow enforces this on every pull_request event.

Items are categorized by discipline; each item has a verification step.
-->

### Pre-commit + CI

- [ ] All pre-commit hooks pass locally (`pre-commit run --all-files` clean)
- [ ] All GitHub Actions checks pass on the PR's HEAD commit
- [ ] `cargo fmt --check` clean (for Rust projects)
- [ ] `cargo clippy --all-targets -- -D warnings` clean (for Rust projects)
- [ ] `cargo test` passes (for Rust projects)

### Audit-trail discipline

- [ ] Suite-side Review N entry authored in `vsdd-suite/suite-development/review-log/YYYY-MM-DD-suite-review.md` (if this PR is a suite-development cycle)
- [ ] Suite-side `SUITE-DEVELOPMENT-REVIEW.md` Reviews-table updated with new Review N row
- [ ] Suite-side `FINDINGS-INDEX.md` registry updated with rN-fM rows for each finding
- [ ] Suite `CHANGELOG.md` Unreleased entry covers the PR's substantive changes
- [ ] Project-side per-session review-log entries authored (if this PR is a project IAR cycle)
- [ ] Project-side `FINDINGS-INDEX.md` updated (if applicable)
- [ ] Project `CHANGELOG.md` entry authored (if a reference example was updated)
- [ ] Cost-tally declared for multi-agent IAR cycle entries at capstone+ intent (per `suite-development.md` § Per-review entry preamble § Cost-tally)

### Methodology discipline

- [ ] `grep -rn before claiming closure` applied to every Resolved finding for a defect class (rename, reword, retire, restructure); grep evidence cited in the Resolution paragraph (per `primers/4-feedback-integration.md` § Anti-patterns — "Site-specific fix declared closure")
- [ ] Forward-only narrative-preservation discipline honored per [G-89](vsdd-suite/suite-development/FINDINGS-INDEX.md#g-89) (historical references in CHANGELOG / COMPATIBILITY / prior-review-log entries preserved as-is; only forward-facing prose updated)
- [ ] Three-audience lens applied to new artifacts (per `suite-development.md` § Three-audience design principle): suite developers + suite users + AI agents
- [ ] Sycophancy compensation declarations included in Review N entry (named what was resisted)

### PR draft + spot-checks

<!--
Spot-check items go here, NOT in informal prose. Each item names a verifiable condition; each has a verification step (a command, a manual-test reference, a CI status, a grep with named expected output). Each is ticked with evidence when verified.
-->

- [ ] Spot-check: [name the condition + the verification command + the expected output]
- [ ] [add more spot-checks as needed]

### Operator-action queue (post-merge)

<!--
Items the operator must take action on AFTER merge — typically upstream filings, CI configuration changes, branch-protection updates, install-verification runs. NOT merge-gating; these are tracked separately.
-->

- [ ] [post-merge operator action 1]
- [ ] [post-merge operator action 2]

## Sycophancy compensation declarations

<!-- What was resisted during authoring? E.g., "Resisted bundling PR #41 into this PR; resisted treating external feedback as exemption from future adversarial pressure." -->

## Notes

<!-- Anything worth calling out: dismissed findings, deferred scope, known issues, design decisions made during implementation. -->
