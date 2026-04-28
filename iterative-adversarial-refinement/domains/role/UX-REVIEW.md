# UX Review

This review is part of the [Iterative Adversarial Refinement (IAR)](../../README.md) suite. It may be run independently or alongside other domains. See [README.md](../../README.md) for sequencing, scoped runs, and domain coordination.

**Reviewer role: UX Designer** (UX Designer / UX Researcher / Product Designer)

The purpose of this review is to apply iterative adversarial pressure to find, document, and resolve UX defects, inconsistencies, accessibility gaps, and regressions. Every review targets the whole application — not only the most recently changed feature.

## Current Review Prompt

**Scope:** Whole application by default. If a scope is provided (e.g., a specific feature or set of changed files), focus primary analysis there — but regression checks always cover the entire application.

Read DESIGN.md first for context on the project's intended scope, constraints, and feature set. Then read all source files, styles, HTML, and tests. Apply every standard dimension below as a floor — add others as appropriate for the current state of the app. There is no restriction on what can be flagged.

For each finding, cite the element, file, and line number. Classify as **resolved** (fix applied this review), **deferred** (scheduled for a specific layer, reason given), **dismissed** (no action taken, rationale required), or **hallucinated** (the adversary invented a problem that does not exist — push back is warranted. Consistent hallucinated findings are the maximum viable refinement signal: real issues have been exhausted).

Regression check: verify that all previously-addressed UX concerns remain intact. Prior layers' UX changes are always in scope. A visual or interaction change to one part of the app can silently break another.

**Coordination:** Flag any findings that should be surfaced to [QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md), [SECURITY-REVIEW.md](SECURITY-REVIEW.md), [PLATFORM-ENGINEER-REVIEW.md](PLATFORM-ENGINEER-REVIEW.md), or [SOLUTION-ARCHITECT-REVIEW.md](SOLUTION-ARCHITECT-REVIEW.md). If this review suggests the need for a new IAR domain, log it as a finding.

**Sycophancy check:** An AI agent cannot experience a user interface — it cannot perceive latency, notice visual imbalance, or discover that a flow is confusing by trying to use it. An agent reviewing its own UI implementation will validate the decisions it made at generation time rather than evaluate the lived experience those decisions create. The adversary must flag any dimension where the review relies on reading code rather than observing the interface. If the project cannot be tested directly in a browser, state that explicitly — do not simulate user experience from source code and report it as a UX evaluation.

**Interface type:** The standard dimensions below assume a browser-rendered interface. For CLI projects, consult `../../lang/cli.md` — the CLI UX dimensions replace most of the standard dimensions below. For browser apps, also consult `../../lang/browser-app.md` for browser-specific accessibility, responsive design, and security UX concerns.

## Standard Evaluation Dimensions

1. **Empty states** — What does the user see when content is absent? Is there a clear prompt or explanation?
2. **Error messages** — Are they specific, correctly placed, and do they clear at the right time?
3. **Focus and keyboard behavior** — Can every action be completed with a keyboard alone? Does focus land in the right place when forms open or content changes? Specifically: can focus become trapped inside a modal, dialog, or dropdown where the user cannot exit without a mouse? A keyboard trap is a WCAG 2.1 Level A failure (2.1.2). Custom modal implementations must handle focus containment while the modal is open and focus restoration to the trigger element on close. Axe may not catch custom implementations — test manually.
4. **Visual consistency** — Are equivalent UI surfaces treated the same?
5. **Interactive affordances** — Do users know what they can interact with? Do interactive elements look interactive?
6. **Feedback patterns** — Are success, error, loading, and empty states present and appropriate?
7. **Accessibility** — Does every interactive element have an accessible label? Is color contrast WCAG AA compliant (4.5:1 for normal text, 3:1 for large text and UI components)? Is semantic HTML used (landmarks, headings, lists)? Are focus indicators visible? Run axe and confirm zero violations. For deeper coverage — screen reader testing, cognitive accessibility, dynamic content announcements, ARIA correctness, zoom and reflow — activate the Accessibility domain. This dimension is the floor, not the ceiling.
8. **Responsive design** — Does the layout hold and remain usable at 360px? Are touch targets at least 44×44px? Does content reflow cleanly without horizontal scroll between mobile and desktop widths?
9. **Browser compatibility** — Are there visual or interaction differences across Chrome, Firefox, and Safari? Are any CSS or HTML features used that render inconsistently?
10. **Long content** — What renders when a text field contains a very long unbroken string? Does text overflow its container horizontally?
11. **Reduced motion** — If any transitions or animations are present, are they disabled for `prefers-reduced-motion: reduce`?
12. **Destructive action confirmation gates** — Do all destructive or irreversible actions (delete, overwrite, bulk operations, permanent changes) have an explicit confirmation step before executing? This is a separate question from the quality of the confirmation text. An app that deletes a record with no confirmation at all has no dialog to evaluate — the gate is missing entirely. Flag absences, not only quality failures.
13. **Native dialog quality** — Where confirmation gates exist, do they use specific, actionable text? Does the dialog name the item being acted on ("Delete 'My Bookmark'?" not "Delete item?")? Does it make the consequence clear ("This cannot be undone")?
14. **Async state and error recovery** — What does the user see during an async operation (fetch, storage write, file read)? A UI that is frozen or blank during async work fails this dimension. If an async operation fails mid-execution, does the UI recover to a consistent state? Is the failure communicated specifically ("Failed to save — changes not stored") or silently dropped? For optimistic updates: if the underlying operation fails, is the rollback visible and graceful, or does the UI show state that was never actually persisted?
15. **Cross-layer regression** — Do new changes visually or interactively break features from earlier layers?

---

Review entries are logged in `iterative-adversarial-refinement/UX-REVIEW.md` inside the project being reviewed.
