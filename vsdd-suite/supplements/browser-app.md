# Browser Application Interface Type Supplement

These dimensions supplement the standard IAR domain reviews for browser-rendered web applications. Apply the relevant section during each domain review in addition to the standard dimensions.

---

## Quality Engineering

- **Accessibility automated scanning** — Is `axe-playwright`, `axe-core`, or an equivalent accessibility scanning tool run in the test suite and configured to fail on violations? Automated scanning is the floor; it does not replace manual testing of keyboard navigation and screen reader behavior.
- **Browser compatibility testing** — Are tests run against Chrome, Firefox, and Safari (or a documented subset with rationale for exclusions)? Are any CSS or JS features used that behave inconsistently across these browsers?
- **Responsive design testing** — Is the layout tested at 360px viewport width (narrow mobile)? Are touch targets at least 44×44px? Is there a test or visual check that content reflows cleanly without horizontal scroll at mobile widths?
- **Keyboard navigation coverage** — Do tests verify that all interactive elements are reachable and operable with keyboard alone? Focus order should be tested, not just assumed.

## Security

- **Rendering safety** — Is user-supplied content set via `.textContent` (safe) or `.innerHTML` (potentially unsafe)? Any `innerHTML` usage with user-controlled data is a finding. Evaluate all templating paths, not just direct assignments.
- **URL injection** — Can a user save a URL that executes code when clicked? Verify `javascript:`, `data:`, `vbscript:`, and other non-http(s) protocols are rejected at input time or blocked before being placed in `href` or `src` attributes.
- **Content Security Policy** — Is a CSP present (meta tag or response header)? Evaluate the attack surface without one. Inline scripts and `eval` in CSP are findings.
- **Storage data validation** — Is data loaded from `localStorage`, `sessionStorage`, `IndexedDB`, or cookies validated before use? Any `as` cast (TypeScript) or unchecked parse applied to storage data is a finding.
- **Third-party script integrity** — Are third-party scripts loaded with `integrity` (SRI) hashes? Can a compromised CDN inject arbitrary code?

## UX

- **Accessibility** — Does every interactive element have an accessible label (via `aria-label`, `aria-labelledby`, or visible text)? Is color contrast WCAG AA compliant (4.5:1 for normal text, 3:1 for large text and UI components)? Is semantic HTML used (landmarks, headings, lists)? Are focus indicators visible? **Focus trap:** Custom modal and dialog implementations must contain focus while open (Tab should not exit the modal) and restore focus to the trigger element on close. This is WCAG 2.1 Level A (2.1.2). Axe does not reliably catch custom implementations — test manually by opening the modal and verifying Tab cycles within it and Escape returns focus to the trigger.
- **Responsive design** — Does the layout hold and remain usable at 360px? Are touch targets at least 44×44px? Does content reflow cleanly without horizontal scroll between mobile and desktop widths?
- **Browser compatibility** — Are there visual or interaction differences across Chrome, Firefox, and Safari? Are any CSS or HTML features used that render inconsistently?
- **Reduced motion** — If any transitions or animations are present, are they disabled for `prefers-reduced-motion: reduce`?
- **Native dialog quality** — Does any `window.confirm` or `window.alert` dialog use specific, actionable text? Does it name the item being acted on?
