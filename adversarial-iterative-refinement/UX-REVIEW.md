# UX Review

This review is part of the [Adversarial Iterative Refinement (AIR)](README.md) suite. It is a required gate for merging. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

The purpose of this review is to apply iterative adversarial pressure to find, document, and resolve UX defects, inconsistencies, accessibility gaps, and regressions. Every review targets the whole application — not only the most recently changed feature.

## Current Review Prompt

**Scope:** Whole application by default. If a scope is provided (e.g., a specific feature or set of changed files), focus primary analysis there — but regression checks always cover the entire application.

Read all source files, styles, HTML, and tests. Apply every standard dimension below as a floor — add others as appropriate for the current state of the app. There is no restriction on what can be flagged.

For each finding, cite the element, file, and line number. Classify as **resolved** (fix applied this review), **deferred** (scheduled for a specific layer, reason given), or **dismissed** (no action taken, rationale required).

Regression check: verify that all previously-addressed UX concerns remain intact. Prior layers' UX changes are always in scope. A visual or interaction change to one part of the app can silently break another.

**Coordination:** Flag any findings that should be surfaced to [QA-REVIEW.md](QA-REVIEW.md), [SECURITY-REVIEW.md](SECURITY-REVIEW.md), [PLATFORM-ENGINEERING-REVIEW.md](PLATFORM-ENGINEERING-REVIEW.md), or [SOLUTION-ARCHITECT-REVIEW.md](SOLUTION-ARCHITECT-REVIEW.md). If this review suggests the need for a new AIR domain, log it as a finding.

## Standard Evaluation Dimensions

1. **Empty states** — What does the user see when content is absent? Is there a clear prompt or explanation?
2. **Error messages** — Are they specific, correctly placed, and do they clear at the right time?
3. **Focus and keyboard behavior** — Can every action be completed with a keyboard alone? Does focus land in the right place when forms open or content changes?
4. **Visual consistency** — Are equivalent UI surfaces treated the same?
5. **Interactive affordances** — Do users know what they can interact with? Do interactive elements look interactive?
6. **Feedback patterns** — Are success, error, loading, and empty states present and appropriate?
7. **Accessibility** — Does every interactive element have an accessible label? Is color contrast WCAG AA compliant (4.5:1 for normal text, 3:1 for large text and UI components)? Is semantic HTML used (landmarks, headings, lists)? Are focus indicators visible? Run axe and confirm zero violations.
8. **Responsive design** — Does the layout hold and remain usable at 360px? Are touch targets at least 44×44px? Does content reflow cleanly without horizontal scroll between mobile and desktop widths?
9. **Browser compatibility** — Are there visual or interaction differences across Chrome, Firefox, and Safari? Are any CSS or HTML features used that render inconsistently?
10. **Long content** — What renders when a text field contains a very long unbroken string? Does text overflow its container horizontally?
11. **Reduced motion** — If any transitions or animations are present, are they disabled for `prefers-reduced-motion: reduce`?
12. **Native dialog quality** — Does any `window.confirm` or `window.alert` dialog use specific, actionable text? Does it name the item being acted on?
13. **Cross-layer regression** — Do new changes visually or interactively break features from earlier layers?

---

Review entries are logged in `adversarial-iterative-refinement/UX-REVIEW.md` inside the project being reviewed.
