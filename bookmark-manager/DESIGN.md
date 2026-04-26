# Bookmark Manager — Design Document

## Purpose
A personal tool for saving and organizing web links with titles, notes, and tags.
Single user, no accounts or authentication needed. Runs in a web browser, data stays on the device.

## Features
1. Add a bookmark: URL, title, optional note, optional tags
2. Display all bookmarks in a list, newest first
3. Click a bookmark to open the link in a new tab
4. Edit a bookmark's title, URL, note, or tags
5. Delete a bookmark
6. Filter bookmarks by tag
7. Search bookmarks by title or note content in real time

## Technology
- HTML, CSS, TypeScript (no frameworks)
- Data stored in the browser's local storage as JSON
- Vite as dev server and build tool
- Vitest for unit tests, Playwright for browser tests
- GitHub Actions for CI — type check, unit tests, browser tests, and build run on every push; all must pass before a branch can be merged into main
- No backend server required

## Interface
- Clean, minimal design with a dark color scheme
- Add form at the top, collapses to a button when not in use
- Search bar and tag filter buttons above the bookmark list
- Bookmark list below, each entry shows title, URL domain, note, and tags
- Tag filters are clickable buttons; active filter is visually highlighted
- Search and tag filters work together

## Constraints
- Must persist data across browser refreshes
- Must work on a phone screen (360px wide minimum)
- URL validation: reject URLs that don't start with `http://` or `https://` (case-insensitive — `HTTP://` and `HTTPS://` are valid); reject protocol-only URLs with no domain (e.g. `https://` alone is invalid)
- Title validation: do not allow empty or whitespace-only titles
- Form state on validation failure: user input must be preserved; do not wipe the form on error
- Bookmark ordering: display newest first; ordering must be deterministic when two bookmarks share an identical timestamp

## Testing Methodology

**Completion standard:** All acceptance criteria must be verified by automated tests (unit or browser) before a task may be marked complete. Code inspection and successful compilation are not sufficient — the tests must exist and pass.

**Unit tests** cover pure logic in isolation using mock storage. No DOM or browser APIs. Run with `npm run test:unit`.

**Browser tests** cover end-to-end behavior against the running app: form interaction, rendered output, localStorage state, and link attributes. Run with `npm run test:browser`. Three axe scans (`@axe-core/playwright`) run as part of the browser suite to catch accessibility violations automatically.

**Coverage** is measured with `npm run test:coverage`. `src/bookmarks.ts` (pure logic) must maintain 100% statement, branch, and function coverage. `src/main.ts` (DOM wiring) is excluded from unit coverage by design — it is covered by browser tests.

**Manual testing** is performed by a human against the running app at the end of each layer. Each layer has a checklist in `TODO.md` covering the full user-visible flow: happy path, edge cases, validation errors, persistence after refresh, and UI state. Automated tests verify correctness; manual tests verify that the experience is coherent and nothing obviously wrong slips through.

**Iterative Adversarial Refinement (IAR)** is run at the end of each layer and is a required gate before merging. IAR applies adversarial pressure across five review domains in parallel or sequence (see `iterative-adversarial-refinement/README.md` for orchestration):

- **QA** (`iterative-adversarial-refinement/QA-REVIEW.md`) — correctness, test falsifiability, selector strength, validation gaps, logic errors, dead code, unused dependencies, coverage gaps, accessibility, browser compatibility, responsive design, security surface, regression coverage
- **UX** (`iterative-adversarial-refinement/UX-REVIEW.md`) — empty states, error messages, focus and keyboard behavior, visual consistency, affordances, feedback patterns, accessibility, responsive design, browser compatibility, long content, reduced motion, native dialog quality
- **Security** (`iterative-adversarial-refinement/SECURITY-REVIEW.md`) — rendering safety, URL injection, storage validation, dependency CVEs, CSP, information exposure, input handling
- **Platform Engineering** (`iterative-adversarial-refinement/PLATFORM-ENGINEERING-REVIEW.md`) — pipeline completeness, gate enforcement, dependency installation, environment pinning, cache correctness, coverage thresholds in CI, security scanning in CI, artifact hygiene, left-shift opportunities
- **Solution Architect** (`iterative-adversarial-refinement/SOLUTION-ARCHITECT-REVIEW.md`) — separation of concerns, coupling and cohesion, data model integrity, interface contracts, state management, immutability, extensibility, technology fitness, complexity budget, decision documentation

Any domain may propose a new IAR domain. All findings are logged in the respective domain file with resolution, accepted-risk, or dismissal rationale.

## Out of Scope
- User accounts, login, or sharing
- Browser extension for quick saving
- Bookmark folders or nested organization
- Import/export
- Sync across devices

## Success Criteria
- I can add, view, edit, and delete bookmarks without opening devtools
- Tag filtering and search work together correctly
- My data survives a full browser restart
- The app rejects invalid input without crashing
- The layout is usable on a phone screen
