# QA Review

This review is part of the [Adversarial Iterative Refinement (AIR)](README.md) suite. It is a required gate for merging. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

The purpose of this review is to apply iterative adversarial pressure to find, document, and resolve bugs, logic errors, test weaknesses, coverage gaps, and regressions. Every review targets the whole application — not only the most recently changed code.

## Current Review Prompt

**Scope:** Whole application by default. If a scope is provided (e.g., a specific feature or set of changed files), focus primary analysis there — but regression checks always cover the entire application.

Read all source files, test files, HTML, CSS, and config. Apply every standard dimension below as a floor — add others as appropriate to the current state of the app. There is no restriction on what can be flagged.

For each finding, cite file and line number. Classify as **resolved** (fix applied this review), **deferred** (scheduled for a specific layer, reason given), or **dismissed** (no action taken, rationale required).

Regression check: verify that all previously-working features still work. Prior layers' acceptance criteria are always in scope. A change to one part of the app can silently break another. A bug that was always present is still a bug.

**Coordination:** Flag any findings that should be surfaced to [UX-REVIEW.md](UX-REVIEW.md), [SECURITY-REVIEW.md](SECURITY-REVIEW.md), [PLATFORM-ENGINEERING-REVIEW.md](PLATFORM-ENGINEERING-REVIEW.md), or [SOLUTION-ARCHITECT-REVIEW.md](SOLUTION-ARCHITECT-REVIEW.md). If this review suggests the need for a new AIR domain, log it as a finding.

## Standard Evaluation Dimensions

1. **Acceptance criteria** — Are all criteria actually met by the implementation, not just implied?
2. **Test falsifiability** — Would each unit and browser test catch a broken implementation? Could any test pass against wrong code?
3. **Selector strength** — Are test selectors tight enough to fail on a broken implementation?
4. **Validation gaps** — What inputs slip through? What edge cases are untested?
5. **Logic errors** — Are there bugs or off-by-one errors in the core logic?
6. **Dead code** — Any exported or declared code with no call sites?
7. **Unused dependencies** — Any direct dependencies not imported or used?
8. **Dependency versions** — Are versions appropriate and not significantly outdated?
9. **Coverage gaps** — Does the coverage report reveal untested branches or functions that correspond to acceptance criteria?
10. **Accessibility** — Is semantic HTML used correctly (landmarks, headings, lists)? Do all interactive elements have accessible labels? Is keyboard navigation complete? Are focus states visible? Are ARIA roles or attributes missing? Run axe and confirm zero violations.
11. **Browser compatibility** — Any JavaScript APIs, CSS properties, or HTML features that behave differently in Firefox or Safari?
12. **Responsive design** — Does the layout hold at 360px? Are touch targets at least 44×44px? Does content reflow cleanly?
13. **Security surface** — Is user content rendered safely? Are user-supplied URLs validated? Is storage data runtime-validated? Any new CVEs in dependencies?
14. **Regression coverage** — Does every bug logged in the review log have an identifiable regression test? Flag any whose regression path is untested.

---

Review entries are logged in `adversarial-iterative-refinement/QA-REVIEW.md` inside the project being reviewed.
