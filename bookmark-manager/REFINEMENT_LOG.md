# Bookmark Manager — Refinement Log

### 2026-04-23 — Initial design document
Created DESIGN.md covering purpose, features, technology, interface, constraints, out of scope, and success criteria.

### 2026-04-23 — TypeScript
**DESIGN.md:** Changed technology from plain JavaScript to TypeScript compiled with `tsc`. Updated the technology section to reflect that a build step is required.

### 2026-04-23 — TODO.md created
**TODO.md:** Decomposed DESIGN.md features into six build layers with trackable tasks:
- Layer 1: Core (project setup, add/display/persist bookmarks, input validation)
- Layer 2: Notes and tags
- Layer 3: Edit and delete
- Layer 4: Tag filtering
- Layer 5: Search
- Layer 6: Polish

### 2026-04-23 — TDD acceptance criteria
**TODO.md:** Added testable acceptance criteria to every task following TDD best practices. Tasks may not be marked complete until all acceptance criteria for that task pass. Criteria are specific and verifiable: they describe exact inputs, expected outputs, and observable behavior rather than implementation details.

### 2026-04-24 — CI workflow relocated to repo root

GitHub Actions only scans `<repo-root>/.github/workflows/` for workflow files — subdirectory `.github/` folders are silently ignored. The workflow was moved from `bookmark-manager/.github/workflows/ci.yml` to `<repo-root>/.github/workflows/bookmark-manager.yml`. In a monorepo, each project gets its own named workflow file at the root, scoped to its directory via `paths` filters. This is the standard GitHub Actions monorepo pattern.

### 2026-04-23 — GitHub Actions CI pipeline

**`.github/workflows/ci.yml`:** Added CI pipeline that runs on every push to any branch and on pull requests targeting main. Steps: typecheck → unit tests → browser tests (Chromium) → build. Playwright browser binaries are cached by `package-lock.json` hash; system dependencies are installed separately when the browser cache is hit to avoid a full re-download on every run.

**`DESIGN.md`:** Added GitHub Actions to the Technology section.

**Branch protection:** The workflow job is named `ci`. To enforce it as a merge gate, branch protection must be configured in GitHub repository settings to require the `ci` check to pass before merging into main. This is a repository setting, not something the workflow file can enforce on its own.

**Rationale:** Tests and typecheck running locally are a developer courtesy, not a guarantee. CI makes the quality gate mandatory and machine-enforced regardless of whether local checks were run. The step order (typecheck → unit → browser → build) runs fastest checks first so failures are reported as early as possible.

### 2026-04-23 — Manual testing checklists

**TODO.md:** Added human-readable manual testing checklists for all six layers. Each checklist covers the full user-visible flow for that layer: happy path, edge cases, validation error behavior, persistence after refresh, and UI state transitions. Checklists are formatted as unchecked items so they can be worked through as literal checklists before advancing to the next layer.

**DESIGN.md:** Added Manual Testing paragraph to the Testing Methodology section. Clarifies the role split: automated tests verify correctness; manual tests verify that the experience is coherent from a user's perspective.

**TODO.md header:** Updated the layer-transition gate to explicitly require the manual testing checklist to be completed alongside the adversarial QA review.

**Rationale:** The adversarial QA review revealed that some acceptance criteria were marked complete without being tested at all. Automated tests catch correctness regressions, but a human walkthrough catches interaction and UX problems that tests don't exercise — error states that feel wrong, visual inconsistencies, and flows that work in isolation but are confusing in sequence. Adding a manual gate ensures both dimensions are checked before advancing.

### 2026-04-23 — Expanded QA checks (dead code, unused dependencies, coverage, dependency versions)

**DESIGN.md:** Restructured Testing Methodology from a flat list into named subsections. Added coverage expectation (`bookmarks.ts` at 100%; `main.ts` exclusion documented). Expanded the adversarial review checklist from 7 dimensions to 8 by adding: dead exports and unreachable code, unused direct dependencies, dependency version hygiene, and coverage gaps corresponding to acceptance criteria.

**TODO.md:** Added a formal layer-transition gate in the header requiring coverage check, dead code audit, unused dependency check, and all review findings logged before moving on. Added QA review status lines between every layer boundary.

**Rationale:** The first adversarial review was scoped to correctness and test quality. Expanding the prompt to cover code hygiene and dependency state ensures the project doesn't accumulate technical debt silently as layers are added. Coverage reporting makes the gate concrete — it's either 100% or it isn't.

### 2026-04-23 — Second adversarial QA review and coverage tooling

Added `@vitest/coverage-v8` and configured coverage reporting in `vite.config.ts` (`provider: 'v8'`, scoped to `src/**/*.ts`, text + json-summary reporters). Added `test:coverage` script to `package.json`. Coverage confirms `bookmarks.ts` is at 100% statements/branches/functions. `main.ts` reports 0% from unit tests by design — it is DOM wiring code covered exclusively by Playwright browser tests.

Second adversarial QA review run against the updated codebase using the expanded 11-point prompt (adds: dead code, unused dependencies, dependency versions, coverage). One test weakness found and resolved: the localStorage browser test was not verifying `note` or `tags` fields, only `title`, `url`, and property existence. Updated the test to submit with note and tags and assert their stored values directly.

Two findings dismissed: floating `^` versions (acceptable for a single-developer project with `package-lock.json`); `main.ts` 0% coverage (expected and correct — not a gap).

No bugs, no dead code, no unused dependencies found.

### 2026-04-23 — Post-QA criteria and process hardening

**DESIGN.md:** Expanded the Constraints section to capture three things the adversarial review revealed were missing: URL validation is case-insensitive (uppercase protocols are valid); protocol-only URLs with no domain are rejected; form input is preserved on validation failure. Added a Testing Methodology section codifying that automated tests must exist and pass before a task is complete — `tsc` compilation and code inspection are not sufficient.

**TODO.md:** Updated the header to state explicitly that "all acceptance criteria pass" means automated tests pass. Added detail to completed Layer 1 tasks to reflect what was actually missing and is now fixed:
- "Add a bookmark": `href` attribute verification, all-fields clear on success, form data preserved on failure
- "Display bookmarks in a list": count assertion, deterministic tiebreaker for equal timestamps
- "Click to open in a new tab": tightened to assert `href`, `target="_blank"`, and `rel="noopener noreferrer"` attributes
- "Persist bookmarks": direct `localStorage.getItem` inspection with field verification, replacing the vague "localStorage contains the bookmark data serialized as JSON"
- "Validate invalid URLs": uppercase protocol, protocol-only, and a full unit test coverage matrix

**TODO.md — Layer 3 (already updated):** Added "not just the first" checks for list-wide behavior; edit covers pre-population, cancel path, validation during edit, and count invariant; persistence tasks require direct localStorage inspection rather than UI observation after reload.

**TODO.md — Layers 4–6 (updated now):** Applied the same lessons forward:
- Tag filtering: explicit deduplication, stale-button edge case after deletion, count assertions on match results, unit tests for extraction logic
- Search: case-insensitivity made explicit, "no match shows empty list" distinction, combined filter + search matrix unit-tested, clear-search-while-filter-active behavior specified
- Polish: collapsible form must stay open on validation failure; domain extraction requires a subdomain test case and unit tests; responsive layout names all UI elements that must work at 360px

**Rationale:** The adversarial review revealed a pattern: criteria were written at the intent level ("newest first", "localStorage contains JSON") without specifying the observable, testable behavior that proves intent was met. Future layers now specify exact counts, attribute names, edge inputs, and unit test coverage matrices so there is no ambiguity about what "complete" means before writing a single test.

### 2026-04-23 — Adversarial QA review

Conducted a structured adversarial review using a QA prompt asking an independent agent to evaluate whether acceptance criteria were met, whether tests were falsifiable, and whether there were bugs or missing edge cases. Full prompt, findings, and dispositions recorded in `ADVERSARIAL.md`.

**Bugs fixed:**

- `validateUrl` used `.startsWith('http://')` — case-sensitive, rejected `HTTP://example.com`. Fixed by switching to `new URL(url)`, which normalizes the protocol to lowercase automatically.
- `validateUrl` accepted `https://` with no domain — `new URL('https://')` throws, so the same fix handles both cases.
- Inline sort in `src/main.ts` returned `0` for equal timestamps, leaving order undefined. Extracted to `sortBookmarks()` in `src/bookmarks.ts` with secondary key `|| a.id.localeCompare(b.id)`.

**Test weaknesses resolved:**

8 weaknesses identified and addressed — including missing `href` assertion, incomplete ordering test, missing field-clear assertions, no localStorage inspection, no form-data-preservation tests, no end-to-end coverage of URL edge cases, weak `generateId` format check, and no stability test for `sortBookmarks`. All resolved with new or updated tests.

**Dismissed findings:** "click opens new tab" (browser behavior, not app behavior), null checks on form elements (static HTML, TypeScript handles type safety), tag/note/URL length limits (Layer 6 scope), localStorage quota (single-user tool, out of scope).

### 2026-04-23 — Dependency injection for storage
**`src/bookmarks.ts`:** Added `BookmarkStorage` interface (`getItem`, `setItem`). `loadBookmarks` and `saveBookmarks` now accept a `BookmarkStorage` parameter instead of calling `localStorage` directly. `src/main.ts` passes `localStorage` at the call sites.

**`tests/unit/bookmarks.test.ts`:** Replaced `localStorage` usage with `createMockStorage()` — a plain `Map`-backed implementation of `BookmarkStorage`. No DOM simulation required.

**`vite.config.ts`:** Switched Vitest environment from `happy-dom` to `node`. Unit tests now run in pure Node.js with no browser API dependencies.

**Rationale:** `loadBookmarks` and `saveBookmarks` were directly calling `localStorage`, making them impure and dependent on a DOM environment to test. Injecting storage as a parameter separates the logic from the browser API, makes the functions testable in plain Node.js, and avoids the ESM/CJS conflict that required `happy-dom` as a workaround.

### 2026-04-23 — Testing infrastructure and build tooling
**DESIGN.md:** Updated technology section — replaced `tsc` as the sole build tool with Vite (dev server + build), Vitest (unit tests), and Playwright (browser tests). This change was required to support automated testing without ES module resolution issues.

**Project structure:** Pure logic extracted from `main.ts` into `src/bookmarks.ts` with named exports, making it importable in unit tests. DOM code moved to `src/main.ts` which imports from `src/bookmarks.ts`. Root `main.ts` and compiled `main.js` removed.

**New files:** `vite.config.ts`, `playwright.config.ts`, `tests/unit/bookmarks.test.ts`, `tests/browser/bookmark-manager.spec.ts`.

**Rationale:** Earlier tasks in Layers 1 and 2 were marked complete based on code inspection and TypeScript compilation alone — not actual test execution. The testing infrastructure was added to make completion criteria verifiable. All 23 unit tests and 17 browser tests pass.
