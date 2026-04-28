# Accessibility Review

This review is part of the [Iterative Adversarial Refinement (IAR)](../../README.md) suite. It may be run independently or alongside other domains. See [README.md](../../README.md) for sequencing, scoped runs, and domain coordination.

**Reviewer role: Accessibility Engineer** (Accessibility Engineer / A11y Specialist)

The purpose of this review is to evaluate whether the application is usable by people with disabilities — including motor, visual, auditory, and cognitive disabilities. Accessibility is not a polish concern. An interface that cannot be operated by keyboard, or that is invisible to a screen reader, excludes users and in many jurisdictions creates legal exposure. WCAG 2.1 Level AA is the standard floor.

This domain applies to all browser-rendered applications and native UI applications. CLI tools and headless services are exempt. The UX domain covers accessibility in passing; this domain applies adversarial pressure to accessibility specifically and at depth.

## Current Review Prompt

**Scope:** Whole application by default. Every interactive element, every piece of dynamic content, every form, every error state.

Read DESIGN.md for stated accessibility requirements or constraints. Then test the application directly: run an automated scanner (axe-core, WAVE, or Lighthouse), then test manually with keyboard navigation, then test with a screen reader if the project warrants it. Automated scanning catches approximately 30–40% of WCAG failures. Manual testing is required — automated results alone are not a pass.

For each finding, cite the element, file, and line number. Classify as **resolved** (fix applied this review), **deferred** (scheduled for a specific layer, reason given), **dismissed** (no action taken, rationale required), **accepted deviation** (WCAG exception documented with rationale — rare; most WCAG failures are not acceptable deviations), or **hallucinated** (the adversary invented a problem that does not exist — push back is warranted. Consistent hallucinated findings are the maximum viable refinement signal).

Regression check: verify that all previously-addressed accessibility improvements remain intact. Automated scanning and manual focus testing must be repeated after every layer that touches the DOM — a refactor that does not change visible content can silently break ARIA attributes, focus order, or contrast ratios.

**Coordination:** Flag findings that overlap with [UX-REVIEW.md](UX-REVIEW.md) (cognitive accessibility, error messaging, affordances), [SECURITY-REVIEW.md](SECURITY-REVIEW.md) (ARIA attributes that leak implementation details), and [PLATFORM-ENGINEER-REVIEW.md](PLATFORM-ENGINEER-REVIEW.md) (CI integration of automated accessibility scanning).

**Sycophancy check:** Accessibility failures are the most commonly rationalized class of bug. "Users with disabilities aren't our target audience" is not a rationale — it is a scope decision that requires explicit justification and acceptance from the project owner. An agent reviewing its own accessibility implementation will frequently dismiss failures with "the semantic meaning is clear from context" or "screen reader users would understand this" without having tested with a screen reader. Every dismissed finding requires a specific, verifiable reason.

**Language and interface supplement:** See the **UX** section of `../../lang/browser-app.md` for browser-specific accessibility dimensions (focus trap testing, contrast requirements, semantic HTML, reduced motion). Browser-app.md has no separate Accessibility section — the browser-specific accessibility content lives in its UX section. This domain goes deeper than those dimensions.

## Standard Evaluation Dimensions

1. **Automated scan baseline** — Run axe-core (or equivalent) and confirm zero violations. This is the floor, not the ceiling — axe catches ~30–40% of WCAG 2.1 AA failures. Record the tool, version, and result. A passing axe run with uncorrected manual-testing failures is still a failing accessibility review.

2. **Keyboard navigation completeness** — Can every action in the application be completed using keyboard alone (Tab, Shift-Tab, Enter, Space, arrow keys, Escape)? Tab order follows logical reading order. No action requires a mouse gesture (hover, right-click, drag) without a keyboard equivalent. All interactive elements are focusable. Test by unplugging the mouse and using the application for its primary use cases end-to-end.

3. **Focus management** — Does focus land in the right place after every state change? After a modal opens: focus moves to the modal. After a modal closes: focus returns to the element that triggered it. After a form submission error: focus moves to the error message or the first field in error. After navigation: focus moves to the new content. Incorrect focus placement forces screen reader users to re-navigate the entire page to find what changed.

4. **Focus trap compliance** — Custom modal and dialog implementations must trap focus while open (Tab must cycle within the dialog, not escape to the background page) and release it on close. This is WCAG 2.1 Level A (2.1.2). Test manually: open a dialog, press Tab repeatedly, verify focus stays within it. Close the dialog and verify focus returns to the trigger. Automated scanners frequently miss this for custom implementations.

5. **ARIA correctness** — Are ARIA attributes used only where native HTML semantics are insufficient? Incorrect ARIA is worse than no ARIA — it misinforms assistive technology. Named failure modes: `role="button"` on an element that should be `<button>`; `aria-label` that duplicates visible text (redundant but not harmful) vs. contradicts it (harmful); `aria-hidden="true"` on focusable elements; `aria-expanded` not updated when the controlled element changes state; `aria-live` regions that announce every keystroke.

6. **Color contrast** — Text contrast meets WCAG AA: 4.5:1 for normal text, 3:1 for large text (18pt+ or 14pt+ bold) and UI component boundaries. Non-text elements that convey information (icons used without text, input borders, focus indicators) meet 3:1. Test with a color contrast checker (Colour Contrast Analyser, browser devtools, or axe) — do not rely on visual judgment. Verify that the contrast values are calculated for the actual rendered colors, including hover and focus states.

7. **Form accessibility** — Every form control has a programmatically associated label (`<label for>` or `aria-labelledby`). Required fields are indicated both visually and programmatically (`aria-required`, not just a visual asterisk). Error messages are associated with the field they describe (`aria-describedby` or `aria-errormessage`). Error state is communicated programmatically (`aria-invalid="true"`), not only through color. Group-related controls use `<fieldset>` and `<legend>`.

8. **Semantic HTML** — Are semantic elements used in place of generic `<div>` and `<span>` where appropriate? Named checks: page has a `<main>` landmark; navigation uses `<nav>`; headings are hierarchical and not skipped; lists use `<ul>` or `<ol>`; buttons use `<button>`, not `<div role="button">`; interactive custom elements have appropriate ARIA roles and keyboard handling. Native elements are preferred — every `<div>` with ARIA is an implementation of something a native element would have provided for free.

9. **Dynamic content announcements** — When content changes dynamically (search results update, error messages appear, success confirmations show, data loads), is the change announced to screen readers? Named patterns: `aria-live="polite"` for non-urgent updates; `aria-live="assertive"` for critical errors that need immediate announcement; `role="status"` for loading confirmations. An interface where content changes without screen reader notification requires sighted users to notice what changed — screen reader users have no mechanism to discover it.

10. **Cognitive accessibility** — Does the interface support users with cognitive disabilities? Named concerns: consistent navigation and layout (controls in the same place on every page); clear, plain language in instructions and error messages; no time limits on user actions without a mechanism to extend them; no auto-advancing content (carousels, auto-dismissing notifications) without a pause control; error recovery that explains what went wrong in plain terms.

11. **Motion and reduced motion** — Are animations and transitions disabled or reduced for `prefers-reduced-motion: reduce`? An interface that ignores this preference can trigger vestibular disorders in affected users. Verify with OS reduced-motion setting enabled or browser `prefers-reduced-motion` emulation.

12. **Zoom and text resize** — Does the application remain fully functional at 200% browser zoom and 200% text size? Content should not overflow, truncate, or become inaccessible. Horizontal scrolling at 200% zoom is a WCAG 1.4.10 failure.

---

Review entries are logged in `iterative-adversarial-refinement/ACCESSIBILITY-REVIEW.md` inside the project being reviewed.
