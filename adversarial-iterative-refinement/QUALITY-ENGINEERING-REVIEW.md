# Quality Engineering Review

This review is part of the [Adversarial Iterative Refinement (AIR)](README.md) suite. It may be run independently or alongside other domains. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

The purpose of this review is to evaluate the quality system as a whole: not just whether bugs exist, but whether the testing strategy, coverage, tooling, and gates are structured to catch defects reliably and repeatedly. Quality Engineering owns the test architecture and the confidence it produces. A passing test suite that would not catch a broken implementation is a quality failure.

## Current Review Prompt

**Scope:** Whole application by default. If a scope is provided (e.g., a specific feature or set of changed files), focus primary analysis there — but regression checks always cover the entire application.

Read DESIGN.md first for context on the project's intended scope, constraints, and feature set. Then read all source files, test files, HTML, CSS, and config. Apply every standard dimension below as a floor — add others as appropriate to the current state of the app. There is no restriction on what can be flagged.

For each finding, cite file and line number. Classify as **resolved** (fix applied this review), **deferred** (scheduled for a specific layer, reason given), or **dismissed** (no action taken, rationale required).

Regression check: verify that all previously-working features still work. Prior layers' acceptance criteria are always in scope. A change to one part of the app can silently break another. A bug that was always present is still a bug.

**Coordination:** Flag any findings that should be surfaced to [UX-REVIEW.md](UX-REVIEW.md), [SECURITY-REVIEW.md](SECURITY-REVIEW.md), [PLATFORM-ENGINEERING-REVIEW.md](PLATFORM-ENGINEERING-REVIEW.md), [SOLUTION-ARCHITECT-REVIEW.md](SOLUTION-ARCHITECT-REVIEW.md), or [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md). If this review suggests the need for a new AIR domain, log it as a finding.

## Standard Evaluation Dimensions

1. **Acceptance criteria** — Are all criteria from DESIGN.md actually met by the implementation, not just implied? Trace each feature to its test coverage.
2. **Test falsifiability** — Would each test catch a broken implementation? Could any test pass against wrong code? A test that cannot fail on a defective implementation has no value.
3. **Test selector and assertion strength** — Are selectors, matchers, and assertions tight enough to fail on a broken implementation? Vague assertions (e.g., checking presence but not content) are a quality gap.
4. **Coverage meaningfulness** — Does coverage reflect genuine confidence, or are covered lines trivially exercised? Are branches, edge cases, and error paths tested, not just happy paths?
5. **Test architecture** — Is the test suite structured for maintainability? Are tests independent? Do they share state in ways that could cause order-dependent failures?
6. **Validation gaps** — What inputs slip through? What edge cases are untested? What happens at boundaries?
7. **Logic errors** — Are there bugs, off-by-one errors, or incorrect assumptions in the core logic?
8. **Dead code** — Any exported or declared code with no call sites?
9. **Unused dependencies** — Any direct dependencies not imported or used in the project?
10. **Dependency versions** — Are versions appropriate and not significantly outdated?
11. **Accessibility** — Is semantic HTML used correctly (landmarks, headings, lists)? Do all interactive elements have accessible labels? Is keyboard navigation complete? Are focus states visible? Are ARIA roles or attributes missing? Run axe and confirm zero violations.
12. **Browser compatibility** — Any JavaScript APIs, CSS properties, or HTML features that behave differently in Firefox or Safari?
13. **Responsive design** — Does the layout hold at 360px? Are touch targets at least 44×44px? Does content reflow cleanly?
14. **Security surface** — Is user content rendered safely? Are user-supplied URLs validated? Is storage data runtime-validated? Any new CVEs in dependencies?
15. **Regression coverage** — Does every bug logged in the review log have an identifiable regression test? Flag any whose regression path is untested.
16. **Quality gates** — Are coverage thresholds, linting, and test runs enforced automatically? Are any quality checks manual-only that a passing CI run could miss?

---

Review entries are logged in `adversarial-iterative-refinement/QUALITY-ENGINEERING-REVIEW.md` inside the project being reviewed.
