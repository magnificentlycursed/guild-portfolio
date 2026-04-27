# Bookmark Manager — Changelog

## 2026-04-27 — IAR suite updated; VSDD governing references added; dim 11 N/A annotated

### Changed
- `iterative-adversarial-refinement/VDD-IAR-ALIGNMENT-REVIEW.md` — Dim 11 (issue tracking compliance) annotated as Not Applicable: Phase 1 project, crosslink not yet introduced; summary updated accordingly

---

## 2026-04-27 — QE dim 14 review; VDD-IAR Alignment dim 4 re-evaluated under hardened standard

### Changed
- `iterative-adversarial-refinement/QA-REVIEW.md` — Review 10 logged; dim 14 (TDD proxy indicators) applied retroactively; all five indicators positive; dismissed
- `iterative-adversarial-refinement/VDD-IAR-ALIGNMENT-REVIEW.md` — Dim 4 re-evaluated under hardened standard (test-after is a finding, not a yellow flag); co-commit pattern plus positive QE dim 14 evidence supports dismissal; QE dim 14 review pending noted

---

## 2026-04-25 — IAR suite expanded; renamed adversarial-iterative-refinement → iterative-adversarial-refinement

### Added
- `iterative-adversarial-refinement/SOFTWARE-ENGINEERING-REVIEW.md` — SE domain (Review 1, portfolio retrospective); all 11 standard dimensions + JS/TS supplement; no defects found
- `iterative-adversarial-refinement/SOLUTION-OWNER-REVIEW.md` — SO domain (Review 1, portfolio retrospective); 27-requirement compliance table; all spec requirements met; ESLint and `@vitest/ui` evaluated and dismissed as non-behavioral tooling
- `iterative-adversarial-refinement/VDD-IAR-ALIGNMENT-REVIEW.md` — VDD-IAR Alignment domain (Review 1, portfolio retrospective); all 10 process dimensions evaluated; strong alignment throughout; initial bundled commit noted and dismissed as workflow artifact
- `iterative-adversarial-refinement/SECURITY-REVIEW.md` — Review 4 appended; new generic dimensions (secret handling, authentication/authorization) from updated suite template; browser-app.md and JS/TS supplements applied; clean pass

### Changed
- `iterative-adversarial-refinement/README.md` — rewritten around VDD-IAR loop framing (8 domains, refinement loop protocol, session isolation); updated domain table to include SE, SO, VDD-IAR Alignment; merge gate updated with MVR requirement
- `PROCESS.md` — review history updated to reflect 8 domains and accurate review counts

### Renamed
- `adversarial-iterative-refinement/` → `iterative-adversarial-refinement/` — correct name is Iterative Adversarial Refinement (IAR), not AIR

### Changed (IAR/AIR terminology)
- All live documentation across `README.md`, `DESIGN.md`, `TODO.md`, `PROCESS.md`, `DECISIONS.md`, and all `iterative-adversarial-refinement/*.md` files updated from AIR to IAR

---

## 2026-04-24 17:00Z — AIR suite run (all 5 domains); Layer 6 focus bug fixed

### Fixed
- `src/main.ts:307` — inline delete Cancel button: replaced bare `renderBookmarks` callback with arrow function that calls `renderBookmarks()` then focuses the newly rendered `.delete-btn` for the same bookmark; clicking Cancel no longer strands keyboard focus on `document.body`
- `src/main.ts:232` — edit form Cancel button: same fix; focus restored to `.edit-btn` for the same bookmark after cancel (pre-existing since Layer 3, caught by Layer 6 AIR run)

### Changed
- `iterative-adversarial-refinement/QA-REVIEW.md` — Review 9 logged; 80 unit | 95 browser | coverage 100% | 0 CVEs | lint clean; 2 bugs resolved, 1 test weakness resolved
- `iterative-adversarial-refinement/UX-REVIEW.md` — Review 6 logged; inline delete cancel focus regression resolved; all other Layer 6 UX surfaces reviewed and dismissed
- `iterative-adversarial-refinement/SECURITY-REVIEW.md` — Review 3 logged; no findings; all controls verified intact
- `iterative-adversarial-refinement/PLATFORM-ENGINEERING-REVIEW.md` — Review 4 logged; all prior gates intact; coverage 100% with new `extractDomain` function
- `iterative-adversarial-refinement/SOLUTION-ARCHITECT-REVIEW.md` — Review 3 logged; cancel-focus fix resolved; boundary and immutability patterns intact

### Fixed (tests)
- `tests/browser/bookmark-manager.spec.ts` — added `'canceling the delete confirmation returns focus to the delete button'` and `'canceling an edit returns focus to the edit button'`

### Test results
- Unit tests: **80 passed**
- Browser tests: **95 passed** (+2 new)

---

## 2026-04-24 16:30Z — Layer 6: Polish

### Added
- `src/bookmarks.ts` — `extractDomain(url)`: returns `new URL(url).hostname`; returns `''` on invalid input; pure function, no mutation
- `index.html` — `#add-form-toggle` button (`aria-expanded`, `aria-controls`, `aria-label="Add bookmark"`) added before the add form; `hidden` attribute added to `#add-form`
- `styles.css` — complete rewrite; CSS custom properties at `:root` for dark color scheme; all text/background combinations verified WCAG AA (≥4.5:1); new classes: `.bookmark-domain`, `.delete-confirm`, `.delete-confirm-msg`, `.delete-confirm-btn`, `.delete-cancel-btn`; `min-height: 44px; display: inline-flex` on all interactive buttons for touch target compliance; `@keyframes fadeIn` (bookmark items) and `@keyframes slideDown` (add form) wrapped in `@media (prefers-reduced-motion: no-preference)`
- `src/main.ts` — `setFormOpen(open)` helper: sets `form.hidden`, `aria-expanded`, and focus; `extractDomain` import and domain label rendering per bookmark; `handleDeleteClick` rewritten as inline confirmation (replaces `.bookmark-actions` in-place with `.delete-confirm` div); tag badges changed from `<span>` to `<button type="button">` with click handler activating tag filter; `handleSubmit` calls `setFormOpen(false)` after success; `DOMContentLoaded` wires toggle button
- `tests/unit/bookmarks.test.ts` — 6 new `extractDomain` tests: standard URL, path strip, subdomain, query string, port, malformed URL
- `tests/browser/bookmark-manager.spec.ts` — `beforeEach` clicks `#add-form-toggle`; all `page.on('dialog', ...)` patterns replaced with `.delete-confirm-btn` / `.delete-cancel-btn` clicks; toggle clicks inserted after each submit where the test re-fills the form; 15 new Layer 6 tests: form hidden on load, toggle shows/hides form, `aria-expanded` state, form collapses on success, form stays open on validation failure, domain label display, hostname-only domain, inline delete confirmation UI, confirm deletes, cancel leaves unchanged, inline delete hides action buttons, tag badge activates filter, tag badge deactivates active filter, axe scan with delete confirmation visible, 360px layout

### Changed
- `TODO.md` — all Layer 6 acceptance criteria marked complete

### Test results
- Unit tests: **80 passed** (+6 new)
- Browser tests: **93 passed** (+16 new)

---

## 2026-04-25 01:30Z — Documentation renames for clarity; add ESLint

### Renamed
- `air/` → `iterative-adversarial-refinement/` — expands the AIR acronym
- `iterative-adversarial-refinement/AIR.md` → `iterative-adversarial-refinement/README.md` — conventional entry point for a folder
- `REFINEMENT_LOG.md` → `DECISIONS.md` — describes the content more precisely (design decisions and architectural rationale)
- `iterative-adversarial-refinement/PE-REVIEW.md` → `iterative-adversarial-refinement/PLATFORM-ENGINEERING-REVIEW.md` — expands the PE abbreviation, consistent with `SECURITY-REVIEW.md`
- `iterative-adversarial-refinement/SA-REVIEW.md` → `iterative-adversarial-refinement/SOLUTION-ARCHITECT-REVIEW.md` — expands the SA abbreviation

### Changed
- All live path references updated across `TODO.md`, `DESIGN.md`, `README.md`, `CHANGELOG.md`, `DECISIONS.md`, `.github/PULL_REQUEST_TEMPLATE.md`, and all files in `iterative-adversarial-refinement/`

---

## 2026-04-25 01:15Z — Add ESLint + typescript-eslint

### Added
- `eslint.config.js` — ESLint flat config; `tseslint.config(eslint.configs.recommended, tseslint.configs.recommended)`
- `package.json` — `lint` script (`eslint src/`)
- `.github/workflows/bookmark-manager.yml` — `Lint` step after `Type check`
- `package.json` devDependencies — `eslint`, `@eslint/js`, `typescript-eslint`

---

## 2026-04-25 01:00Z — Add axe scan for search-active state

### Added
- `tests/browser/bookmark-manager.spec.ts` — axe scan with search input active and results filtered (4th automated accessibility scan; covers search input, `aria-live` status region, and filtered list state)

### Changed
- `README.md` — browser test count updated to 78; axe scan count updated to 4

---

## 2026-04-25 00:30Z — AIR suite run (all 5 domains); no findings

### Changed
- `iterative-adversarial-refinement/QA-REVIEW.md` — Review 8 logged; 74 unit | 77 browser | coverage 100% | 0 CVEs; no findings
- `iterative-adversarial-refinement/UX-REVIEW.md` — Review 5 logged; no UX surface changes; axe scans passing; no findings
- `iterative-adversarial-refinement/SECURITY-REVIEW.md` — Review 2 logged; `npm audit` now CI-gated; all controls intact; no findings
- `iterative-adversarial-refinement/PLATFORM-ENGINEERING-REVIEW.md` — Review 2 logged; all Review 1 gates verified intact; no findings
- `iterative-adversarial-refinement/SOLUTION-ARCHITECT-REVIEW.md` — Review 2 logged; push→spread fix verified; boundary intact; no findings

---

## 2026-04-25 00:20Z — AIR suite moved to air/ subfolder

### Changed
- `air/` — new subdirectory containing all AIR suite files: `AIR.md`, `QA-REVIEW.md`, `UX-REVIEW.md`, `SECURITY-REVIEW.md`, `PE-REVIEW.md`, `SA-REVIEW.md`
- Internal links in `air/AIR.md`, `air/PE-REVIEW.md`, `air/SA-REVIEW.md` updated to use `../` for references to files outside the folder
- `DESIGN.md`, `TODO.md`, `README.md`, `.github/PULL_REQUEST_TEMPLATE.md` — all references updated from `FOO-REVIEW.md` to `air/FOO-REVIEW.md`

---

## 2026-04-25 00:10Z — Solution Architect domain added to AIR suite; SA Review 1

### Added
- `SA-REVIEW.md` — Solution Architect AIR domain; standard dimensions and Review 1 (full project Layers 1–5); covers separation of concerns, coupling and cohesion, data model integrity, interface contracts, state management, immutability, extensibility, technology fitness, complexity budget, decision documentation

### Fixed
- `src/main.ts:293` — `handleSubmit` called `bookmarks.push(newBookmark)` before saving; the only mutation in a codebase where every other data operation (`updateBookmark`, `deleteBookmark`, `sortBookmarks`) returns a new array. No observable bug, but inconsistent pattern. Changed to `saveBookmarks(storage, [...bookmarks, newBookmark])`.

### Changed
- `AIR.md` — Solution Architect added to domain table with link; SA-first sequencing guidance added for structural changes
- `QA-REVIEW.md`, `UX-REVIEW.md`, `SECURITY-REVIEW.md`, `PE-REVIEW.md` — coordination lines updated to include `SA-REVIEW.md`
- `DESIGN.md` — Solution Architect added to AIR domain list
- `TODO.md` — SA AIR gate items added to layer gate header
- `README.md` — `SA-REVIEW.md` added to documentation table
- `.github/PULL_REQUEST_TEMPLATE.md` — SA checklist item added

---

## 2026-04-24 23:55Z — Platform Engineering domain added to AIR suite; PE Review 1

### Added
- `PE-REVIEW.md` — Platform Engineering AIR domain; standard dimensions and Review 1 (full project Layers 1–5); covers pipeline completeness, gate enforcement, dependency installation, environment pinning, cache correctness, coverage thresholds in CI, security scanning in CI, artifact hygiene, left-shift opportunities

### Fixed
- `.github/workflows/bookmark-manager.yml` — Playwright cache key: changed `hashFiles('package-lock.json')` to `hashFiles('bookmark-manager/package-lock.json')`; the lock file is not at the repo root, so the previous key always produced the same empty hash and the cache never invalidated
- `.github/workflows/bookmark-manager.yml` — added `npm run test:coverage` step after unit tests; coverage was a PR checklist requirement but was not enforced in CI
- `.github/workflows/bookmark-manager.yml` — added `npm audit --audit-level=high` step; security audit was in the Security review manual checklist but not automated
- `vite.config.ts` — changed coverage `include` from `src/**/*.ts` to `src/bookmarks.ts`; `main.ts` is excluded from unit coverage by design and its inclusion would have caused any threshold to fail
- `vite.config.ts` — added coverage `thresholds`: 100% statements, branches, functions, lines; previously there was no threshold and CI could not enforce the 100% requirement

### Changed
- `AIR.md` — Platform Engineering added to domain table with link; PE-first sequencing guidance added
- `QA-REVIEW.md`, `UX-REVIEW.md`, `SECURITY-REVIEW.md` — coordination lines updated to include `PE-REVIEW.md`
- `DESIGN.md` — Platform Engineering added to AIR domain list
- `TODO.md` — PE AIR gate items added to layer gate header
- `README.md` — `PE-REVIEW.md` added to documentation table
- `.github/PULL_REQUEST_TEMPLATE.md` — PE checklist item added

---

## 2026-04-24 23:45Z — Iterative Adversarial Refinement (AIR) suite formalized

### Added
- `AIR.md` — suite description: domain table with links, full and scoped run instructions, sequencing guidance, domain suggestion protocol, merge gate requirements

### Changed
- `QA-REVIEW.md` — current prompt updated: AIR suite link, scope parameter, coordination/cross-domain flag instruction
- `UX-REVIEW.md` — current prompt updated: AIR suite link, scope parameter, coordination/cross-domain flag instruction
- `SECURITY-REVIEW.md` — current prompt updated: AIR suite link, scope parameter, coordination/cross-domain flag instruction
- `DESIGN.md` — Testing Methodology: three separate review descriptions consolidated into a single AIR section with domain summary table
- `TODO.md` — layer gate: updated to reference AIR suite; per-domain gate items labelled as AIR gate
- `README.md` — description updated to name AIR; `AIR.md` added to documentation table
- `.github/PULL_REQUEST_TEMPLATE.md` — nine individual review checklist items consolidated into four AIR-focused items
- `guild-portfolio/README.md` — methodology note updated to name AIR

---

## 2026-04-24 23:30Z — Renamed ADVERSARIAL.md to QA-REVIEW.md; restructured QA, UX, and Security review files

### Added
- `QA-REVIEW.md` — replaces `ADVERSARIAL.md`; restructured with results-focused current-prompt section at top (standard dimensions as a floor, not a ceiling; explicit regression check), followed by review entries newest-first; each entry has a brief `**Scope:**` line instead of a verbose persona prompt

### Removed
- `ADVERSARIAL.md` — renamed and restructured as `QA-REVIEW.md`

### Changed
- `UX-REVIEW.md` — restructured with results-focused current-prompt section at top (same format as QA-REVIEW.md); old `### Prompt` sections removed from each review entry; replaced with brief `**Scope:**` lines; findings reorganized into `### Resolved` and `### Dismissed`
- `SECURITY-REVIEW.md` — restructured with results-focused current-prompt section at top; old `### Prompt` removed from Review 1; findings reorganized into `### Resolved`, `### Accepted Risk`, and `### Dismissed`
- `DESIGN.md` — updated `ADVERSARIAL.md` reference to `QA-REVIEW.md`
- `TODO.md` — updated all `ADVERSARIAL.md` references to `QA-REVIEW.md`
- `README.md` — updated documentation table entry from `ADVERSARIAL.md` to `QA-REVIEW.md`
- `.github/PULL_REQUEST_TEMPLATE.md` — updated `ADVERSARIAL.md` reference to `QA-REVIEW.md`
- `REFINEMENT_LOG.md` — updated `ADVERSARIAL.md` reference to `QA-REVIEW.md`

---

## 2026-04-24 23:00Z — QA review 7, UX review 4, Security review 1: process expansion and full-project audit

### Added
- `SECURITY-REVIEW.md` — security review log; standard dimensions and Review 1 (full project Layers 1–5); covers rendering safety, URL injection, storage validation, dependency audit, CSP, information exposure
- `@axe-core/playwright` — automated accessibility scanning integrated into browser test suite

### Fixed
- `src/bookmarks.ts` — `loadBookmarks` now validates parsed storage data at runtime via `normalizeBookmark`: rejects non-arrays; filters entries missing required `id`/`url`/`title`; coerces missing optional fields to safe defaults; filters non-string values from `tags`
- `src/main.ts` — `handleDeleteClick` now loads the bookmark before showing the confirm dialog and displays `Delete "${title}"?` instead of generic "Delete this bookmark?"
- `src/main.ts` — inline edit form: `label.htmlFor` and `input.id`/`textarea.id` now set with matching values (`edit-${id}-${fieldName}`); fixes critical axe label violation
- `styles.css` — `.list-empty` color changed from `#888` (3.54:1, fails AA) to `#666` (5.74:1, passes AA); fixes axe color-contrast violation
- `styles.css` — `overflow-wrap: break-word` added to `.bookmark-title`, `.bookmark-note`, `.tag-badge`; prevents horizontal overflow on long unbroken strings
- `index.html` — `#tag-filters` div: added `role="group" aria-label="Filter by tag"`; `#bookmark-list`: added `aria-label="Bookmarks"`

### Changed
- `QA-REVIEW.md` — standard dimensions expanded: added security surface (dimension 13) and regression coverage (dimension 14); axe requirement added to accessibility dimension
- `UX-REVIEW.md` — standard dimensions expanded: added long content (10), reduced motion (11), native dialog quality (12), cross-layer regression (13)
- `DESIGN.md` — security review added to Testing Methodology section
- `TODO.md` — layer gate updated to require security review; Layer 6 tasks added: `prefers-reduced-motion` media query, 200% zoom checklist item, reduced-motion manual test
- `.github/PULL_REQUEST_TEMPLATE.md` — security review checklist items added
- `README.md` — test counts updated; security review and methodology note updated; `SECURITY-REVIEW.md` added to documentation table

### Fixed (tests)
- `tests/unit/bookmarks.test.ts` — 2 new URL injection tests (`javascript:`, `data:`); 6 new `loadBookmarks` normalization tests
- `tests/browser/bookmark-manager.spec.ts` — 3 axe scans (empty state, with bookmarks, with edit form open)

### Test results
- Unit tests: **74 passed** (+8 new)
- Browser tests: **77 passed** (+3 new)

---

## 2026-04-24 22:30Z — QA review 6 and UX review 3: accessibility and cross-browser fixes

### Fixed
- `styles.css` — removed dead `#search-input` rule (exact duplicate of `input {}`)
- `styles.css` — added `input[type="search"] { -webkit-appearance: none; appearance: none }` to normalize Safari's pill-shaped appearance on search fields
- `styles.css` — added `.sr-only` utility class (visually hidden, screen-reader readable)
- `index.html` — added `role="search"` to `.search-bar` div; added `<p id="search-status" class="sr-only" aria-live="polite" aria-atomic="true">` for screen reader result announcements
- `src/main.ts` — `renderBookmarks()` now updates `#search-status` text: "N bookmark(s) shown." while filtering with results, "No bookmarks match your search." when filtering to zero, empty otherwise

### Fixed (tests)
- `tests/browser/bookmark-manager.spec.ts` — added `'search bar has a search landmark role'`; `'search status region has aria-live and aria-atomic attributes'`; `'search status announces the result count while filtering'`; `'adding a bookmark while search is active shows it if it matches and hides it if it does not'`

### Deferred to Layer 6
- `TODO.md` — added "Increase touch target sizes for small buttons" task: `.filter-btn`, `.edit-btn`, `.delete-btn`, `.cancel-edit` are ~21px tall, below the 44×44px minimum

### Test results
- Unit tests: **66 passed**
- Browser tests: **74 passed** (+4 new)

---

## 2026-04-24 21:55Z — Layer 5: Search

### Added
- `src/bookmarks.ts` — `searchBookmarks(bookmarks, query)` returns bookmarks whose title or note contains the query (case-insensitive substring match); returns all bookmarks unchanged when query is empty or whitespace-only; pure, no mutation
- `src/bookmarks.ts` — `applyFilters(bookmarks, tag, query)` composes `filterByTag` and `searchBookmarks`; applies tag filter first, then search; both conditions must be satisfied
- `index.html` — `<div class="search-bar">` with `<input type="search" id="search-input">` inserted above `#tag-filters`
- `styles.css` — `.search-bar`, `.search-label`, `#search-input` styles; consistent with existing form input styles

### Changed
- `src/main.ts` — added `let searchQuery = ''` module-level state; `DOMContentLoaded` wires `input` event on `#search-input` to update `searchQuery` and call `renderBookmarks()`; `renderBookmarks` now calls `applyFilters(bookmarks, activeTag, searchQuery)` instead of `filterByTag` directly; empty state distinguishes "no bookmarks" (add one above) from "no match" (no bookmarks match your search)
- `src/main.ts` — `filterByTag` import replaced by `applyFilters`
- `TODO.md` — Layer 5 tasks marked complete

### Test results
- Unit tests: **66 passed** (+14 new: `searchBookmarks` ×8, `applyFilters` ×6)
- Browser tests: **70 passed** (+11 new: search bar visibility ×2, title filter, note filter, no-match empty state, case-insensitive, clear restores list, combined tag+search, clear search with tag active, click All with search active, search empty state message)

---

## 2026-04-24 20:38Z — Layer 4 merged into main

### Status
- Layer 1 (Core): automated tests ✅, manual tests ✅, QA review ✅, UX review ✅
- Layer 2 (Notes and Tags): automated tests ✅, manual tests ✅, QA review ✅, UX review ✅
- Layer 3 (Edit and Delete): automated tests ✅, manual tests ✅, QA review ✅, UX review ✅
- Layer 4 (Tag Filtering): automated tests ✅, manual tests ✅, QA review ✅, UX review ✅

### Changed
- `.github/PULL_REQUEST_TEMPLATE.md` — added UX review checklist items (run review, log in `UX-REVIEW.md`) to the layer gate; UX review is now a formal merge requirement alongside adversarial QA review

---

## 2026-04-24 20:35Z — Layer 4 manual testing passed

### Changed
- `TODO.md` — Layer 4 manual testing checklist marked complete (all 16 items passed by human verification against the running app on 2026-04-24)

### Status
- Layer 4 (Tag Filtering): automated tests ✅, manual tests ✅, QA review ✅, UX review ✅

---

## 2026-04-24 20:30Z — QA review 5: UX change test gaps closed

### Fixed (tests)
- `tests/browser/bookmark-manager.spec.ts` — added `'clears the add form error when typing in any field, not just the erroring one'`; `'edit form focuses the title field when opened'`; `'edit form labels Note and Tags as optional'`; `'clears the edit form error message when the user starts typing'`
- `TODO.md` — Layer 4 manual checklist expanded with 8 new items covering empty state, toggle deselect, error clearing, edit form focus, and edit form optional hints

### Test results
- Unit tests: **52 passed**
- Browser tests: **59 passed** (+4 new)

---

## 2026-04-24 20:24Z — UX review 2: empty states, error clearing, focus, edit form consistency

### Added
- `UX-REVIEW.md` — Review 2 added; full UX audit of Layers 1–4 with findings, resolutions, and dismissals
- `styles.css` — `.list-empty` style: muted color, generous padding for the empty state message

### Changed
- `src/main.ts` — empty state: appends `<li class="list-empty">No bookmarks yet. Add one above.</li>` to the bookmark list when no bookmarks exist
- `src/main.ts` — add form: `input` event listener on the form clears the error message as soon as the user starts typing
- `src/main.ts` — edit form: focuses the first input/textarea field immediately after the inline form is injected; also adds `input` event listener to clear the edit error on typing
- `src/main.ts` — edit form: Note and Tags fields now include `(optional)` and `(optional, comma-separated)` hints via `<span class="optional">`, matching the add form
- `DESIGN.md` — UX review added to Testing Methodology section with evaluation criteria and log reference
- `TODO.md` — layer gate header updated to include UX review alongside QA review; Layer 4 UX review marked complete; deferred UX items added to Layer 6 (inline delete confirmation, tag badge click-to-filter)

### Fixed (tests)
- `tests/browser/bookmark-manager.spec.ts` — added `'shows an empty state message when no bookmarks exist'`; `'shows an empty state message after all bookmarks are deleted'`; `'clears the add form error message when the user starts typing'`

### Test results
- Unit tests: **52 passed**
- Browser tests: **55 passed** (+3 new)

---

## 2026-04-24 20:17Z — Tag filter toggle deselect and empty URL validation

### Added
- `UX-REVIEW.md` — UX review log; Review 1 covers tag filter toggle deselect and AND vs OR multi-select analysis

### Changed
- `src/main.ts` — tag filter buttons now toggle: clicking an active tag deselects it (sets `activeTag = null`) instead of keeping it active; single-line change in the click handler (`activeTag = activeTag === tag ? null : tag`)
- `src/bookmarks.ts` — `validateUrl` now returns `'URL cannot be empty'` for empty or whitespace-only input before attempting URL parsing; previously fell through to the generic `'URL must start with http:// or https://'` message

### Fixed (tests)
- `tests/unit/bookmarks.test.ts` — updated `validateUrl` empty string test to assert `'URL cannot be empty'`
- `tests/browser/bookmark-manager.spec.ts` — added `'shows error and does not add bookmark when URL is empty'` test; added `'clicking an active tag filter deselects it and shows all bookmarks'` test

### Test results
- Unit tests: **52 passed**
- Browser tests: **52 passed** (+2 new)

---

## 2026-04-24 19:41Z — Adversarial QA review 4: bug fixed and test hardened

### Fixed
- `src/main.ts` — `renderTagFilters` now resets `activeTag = null` when the active tag no longer exists in any bookmark; previously, deleting the last bookmark with a given tag while that filter was active left `activeTag` pointing at a ghost tag, causing no filter button to be highlighted. Also deduplicates the `getUniqueTags` call: `uniqueTags` is computed once and used for both the reset check and the button loop.

### Fixed (tests)
- `tests/browser/bookmark-manager.spec.ts:226` — "when a tag filter is active and no bookmarks match the list is empty" test expanded to assert that after deletion the "All" button is highlighted and is the only filter button; previously only asserted `bookmark-item` count of 0

### Test results
- Unit tests: **52 passed**
- Browser tests: **50 passed**

---

## 2026-04-24 19:36Z — Layer 4: Tag Filtering

### Added
- `src/bookmarks.ts` — `getUniqueTags(bookmarks)` returns a sorted, deduplicated array of all tags across all bookmarks; `filterByTag(bookmarks, tag)` returns only bookmarks containing the given tag; both are pure and do not mutate their input
- `src/main.ts` — module-level `activeTag: string | null = null` state; `renderTagFilters(bookmarks)` builds the filter bar (All + one button per unique tag), re-renders on click; `renderBookmarks()` now calls `renderTagFilters` and applies `filterByTag` when `activeTag` is set
- `index.html` — `<div id="tag-filters" class="tag-filters">` inserted between the form and the bookmark list
- `styles.css` — `.tag-filters`, `.filter-btn`, `.filter-btn--active`, hover and focus styles for the filter bar

### Changed
- `TODO.md` — Layer 4 tasks marked complete

### Test results
- Unit tests: **52 passed** (10 new: `getUniqueTags` ×5, `filterByTag` ×5)
- Browser tests: **50 passed** (12 new: All button present, All highlighted on load, filter per tag, no duplicate buttons, click shows matching only, non-matching hidden, empty list when no match, All restores full list, active highlight, switching filters, deleting removes button, add while filter active)

---

## 2026-04-24 19:17Z — GitHub PR template

### Added
- `.github/PULL_REQUEST_TEMPLATE.md` — PR template with layer gate checklist: acceptance criteria, unit tests, browser tests, 100% coverage, manual testing checklist, adversarial QA review, QA-REVIEW.md log, CHANGELOG, REFINEMENT_LOG; includes test results table and notes section

---

## 2026-04-24 19:12Z — MVP complete: Layers 1–3 manual testing passed, ready to merge

### Changed
- `TODO.md` — manual testing checklists for Layers 1, 2, and 3 marked complete (all tests passed by human verification against the running app on 2026-04-24)

### Status
- Layer 1 (Core): automated tests ✅, manual tests ✅, QA review ✅
- Layer 2 (Notes and Tags): automated tests ✅, manual tests ✅, QA review ✅
- Layer 3 (Edit and Delete): automated tests ✅, manual tests ✅, QA review ✅

---

## 2026-04-24 18:59Z — Adversarial QA review 3: Layer 3 test weaknesses resolved

### Fixed (tests)
- `tests/browser/bookmark-manager.spec.ts` — "saving an edit updates displayed values" test expanded to fill in note and tags before editing, fill in new values for all four fields, and assert `.bookmark-note` text, `.tag-badge` count, and badge text after saving; previously only checked title and href
- `tests/browser/bookmark-manager.spec.ts` — added `'editing a bookmark to remove its note hides the note element'` and `'editing a bookmark to remove its tags hides the tag badges'`

### Test results
- Unit tests: **42 passed**
- Browser tests: **38 passed**

---

## 2026-04-24 18:52Z — Layer 3: Edit and Delete

### Added
- `src/bookmarks.ts` — `updateBookmark(bookmarks, id, updates)` returns a new array with the matched bookmark's fields replaced; `deleteBookmark(bookmarks, id)` returns a new array with the matched bookmark removed; both are pure and do not mutate their input
- `src/main.ts` — edit button and delete button rendered on each bookmark item; `handleEditClick` replaces the bookmark's `li` content with an inline edit form pre-populated with current values; `handleEditSave` validates, updates storage, and re-renders; `handleDeleteClick` uses `window.confirm` before deleting
- `styles.css` — styles for `.bookmark-actions`, `.edit-btn`, `.delete-btn`, `.cancel-edit`, `.edit-form`, `.edit-error`

### Changed
- `src/main.ts` — `data-id` attribute added to each `li` in `renderBookmarks()` so edit handler can locate the correct element; imports updated to include `updateBookmark` and `deleteBookmark`
- `TODO.md` — Layer 3 tasks marked complete

### Test results
- Unit tests: **42 passed** (11 new: `updateBookmark` ×6, `deleteBookmark` ×5)
- Browser tests: **36 passed** (14 new: edit button presence, inline form pre-population, save, cancel, validation errors, localStorage verification, persistence, delete with confirm/dismiss, localStorage verification, persistence)

---

## 2026-04-24 — Fix typecheck errors surfaced by CI

### Changed
- `vite.config.ts` — changed `defineConfig` import from `vite` to `vitest/config`; the `vite` version does not include the `test` property in its type signature, causing `tsc` to error with TS2769
- `tsconfig.json` — bumped `target` and `lib` from `ES2020` to `ES2021`; `happy-dom` (transitive dependency of Vitest) references `WeakRef` in its type declarations, which is not available in the ES2020 lib

---

## 2026-04-24 — Move CI workflow to repo root

### Changed
- Workflow moved from `bookmark-manager/.github/workflows/ci.yml` to `<repo-root>/.github/workflows/bookmark-manager.yml` — GitHub Actions only reads workflows from `.github/workflows/` at the repository root; the previous location would never have been picked up

---

## 2026-04-23 — GitHub Actions CI pipeline

### Added
- `<repo-root>/.github/workflows/bookmark-manager.yml` — CI pipeline triggered on push or PR to main when files under `bookmark-manager/` change; steps in order: checkout → Node 20 setup (npm cache keyed to `bookmark-manager/package-lock.json`) → `npm ci` → typecheck → unit tests → Playwright browser cache → Playwright browser install → browser tests → build; `defaults.run.working-directory` set to `bookmark-manager` so all run steps execute in the project folder without repeating it per step

### Changed
- `DESIGN.md` — added GitHub Actions to the Technology section

> **Note:** To enforce the CI check as a merge gate, enable branch protection on `main` in GitHub → Settings → Branches → Add rule → require the `ci` status check to pass before merging.

---

## 2026-04-23 — Manual testing checklists and layer gate process

### Changed
- `DESIGN.md` — added Manual Testing paragraph to Testing Methodology; added manual testing to the layer completion gate alongside automated tests and adversarial review
- `TODO.md` — added manual testing requirement to the layer-transition gate in the header; added human-readable testing checklists for all six layers covering happy path, edge cases, validation errors, persistence, and UI state; added QA review status lines between layers (completed reviews link to QA-REVIEW.md; upcoming layers marked Pending)

---

## 2026-04-23 — Expanded QA checks: coverage, dead code, dependencies

### Changed
- `DESIGN.md` — expanded Testing Methodology into structured subsections; added coverage requirement (`bookmarks.ts` must maintain 100%); documented `main.ts` 0% exclusion as intentional; expanded adversarial review checklist to 8 dimensions (added: dead code, unused dependencies, dependency versions, coverage gaps)
- `TODO.md` — added layer-transition gate checklist requiring coverage check, dead code check, unused dependency check, and all findings logged before advancing; added QA review status lines between all layers

---

## 2026-04-23 — Adversarial QA review 2: coverage tooling and test hardened

### Added
- `@vitest/coverage-v8` — coverage provider for Vitest
- `vite.config.ts` — coverage configuration: `provider: 'v8'`, `include: ['src/**/*.ts']`, reporters `text` and `json-summary`
- `package.json` — added `test:coverage` script (`vitest run --coverage`)

### Fixed (tests)
- `tests/browser/bookmark-manager.spec.ts:85` — localStorage test now fills in note and tags before submitting; explicitly asserts `stored[0].note`, `stored[0].tags`, `stored[0].id` (truthy), and `stored[0].createdAt` (> 0); previously only checked title, url, and property existence

### Coverage report (bookmarks.ts)
- Statements: 100% | Branches: 100% | Functions: 100%
- `main.ts` reports 0% (expected — DOM code covered by browser tests only)

### Test results
- Unit tests: **31 passed**
- Browser tests: **22 passed**

---

## 2026-04-23 — README and post-QA documentation

### Added
- `README.md` — project overview, feature summary, stack, getting-started instructions, script reference, source structure, and documentation index

### Changed
- `DESIGN.md` — expanded URL validation constraint to cover case-insensitivity and protocol-only rejection; added form-state-on-failure constraint; added Testing Methodology section codifying that automated tests must exist and pass before a task is complete
- `TODO.md` — header updated to state automated tests are required, not just compilation; completed Layer 1 criteria updated to reflect what was missing (href verification, all-fields clear, form data preservation, localStorage field inspection, URL edge case matrix); Layer 3 criteria hardened (pre-population, cancel path, validation during edit, count invariants, direct localStorage inspection); Layers 4–6 criteria hardened with count assertions, edge cases, unit test coverage matrices, and explicit empty-state behavior

---

## 2026-04-23 — Adversarial QA review: bugs fixed and tests hardened

### Fixed
- `src/bookmarks.ts:validateUrl` — replaced `.startsWith('http://')` string comparison with `new URL(url)` constructor; normalizes uppercase protocols automatically and rejects protocol-only URLs like `https://` with no host
- `src/bookmarks.ts` — extracted inline sort from `src/main.ts` into `sortBookmarks()`; added secondary sort key `|| a.id.localeCompare(b.id)` to produce deterministic ordering for bookmarks with identical timestamps
- `src/main.ts` — replaced inline sort with `sortBookmarks()`

### Added (tests)
- `tests/unit/bookmarks.test.ts` — 8 new tests: uppercase HTTP/HTTPS URL acceptance, protocol-only URL rejection, `generateId` hyphen format check, `sortBookmarks` suite (correct order, non-mutation, stable identical-timestamp order, empty array)
- `tests/browser/bookmark-manager.spec.ts` — 5 new tests: `href` attribute verification on added bookmark, both-bookmark ordering with count assertion, all-fields clear after submission, localStorage JSON content inspection, form data preservation on title and URL validation failure, uppercase protocol acceptance, protocol-only URL rejection

### Test results
- Unit tests: **31 passed**
- Browser tests: **22 passed**

---

## 2026-04-23 — Dependency injection for storage

### Changed
- `src/bookmarks.ts` — added `BookmarkStorage` interface (`getItem`, `setItem`); `loadBookmarks` and `saveBookmarks` now accept a `BookmarkStorage` parameter instead of calling `localStorage` directly
- `src/main.ts` — passes `localStorage` to `loadBookmarks` and `saveBookmarks` at call sites
- `tests/unit/bookmarks.test.ts` — replaced `localStorage` with `createMockStorage()`, a plain `Map`-backed mock; no DOM simulation required
- `vite.config.ts` — Vitest environment changed from `happy-dom` to `node`

### Test results
- Unit tests: **23 passed** (pure Node.js, no browser APIs)
- Browser tests: **17 passed**

---

## 2026-04-23 — Testing infrastructure

### Added
- `src/bookmarks.ts` — pure logic extracted from `main.ts` with named exports (`Bookmark`, `STORAGE_KEY`, `loadBookmarks`, `saveBookmarks`, `generateId`, `validateTitle`, `validateUrl`, `parseTags`)
- `src/main.ts` — DOM code, imports from `src/bookmarks.ts`
- `vite.config.ts` — Vite dev server config; Vitest configured with `happy-dom` environment
- `playwright.config.ts` — Playwright configured to run Chromium, start Vite dev server automatically
- `tests/unit/bookmarks.test.ts` — 23 unit tests covering all pure functions in `src/bookmarks.ts`
- `tests/browser/bookmark-manager.spec.ts` — 17 browser tests covering Layer 1 and Layer 2 acceptance criteria

### Changed
- `index.html` — script tag updated to `<script type="module" src="/src/main.ts">` for Vite
- `tsconfig.json` — updated to `"module": "ESNext"`, `"moduleResolution": "bundler"`, `"noEmit": true`; `include` updated to cover `src/` and `tests/`
- `package.json` — added `"type": "module"` and scripts: `dev`, `build`, `typecheck`, `test:unit`, `test:browser`, `test`

### Removed
- Root `main.ts` and `main.js` — replaced by `src/main.ts`

### Test results
- Unit tests: **23 passed**
- Browser tests: **17 passed**

---

## 2026-04-23 — Layer 2: Notes and Tags

### Changed
- `Bookmark` interface — added `note: string` and `tags: string[]` fields
- `handleSubmit` — reads note textarea and tags input; parses tags via `parseTags`
- `renderBookmarks` — renders note as `<p class="bookmark-note">` when present; renders tags as `<span class="tag-badge">` elements inside a `.bookmark-tags` container when present

### Added
- `parseTags` — splits comma-separated tag input, trims whitespace, filters empty entries
- `index.html` — note textarea and tags input added to add form, both marked optional
- `styles.css` — styles for `textarea`, `.optional` label hint, `.bookmark-note`, `.bookmark-tags`, and `.tag-badge`

### Layer 2 tasks completed
- [x] Add optional note field to the add form
- [x] Add optional tags field to the add form (comma-separated input)
- [x] Display note under each bookmark's title
- [x] Display tags as badges on each bookmark

---

## 2026-04-23 — Layer 1: Core

### Added
- `tsconfig.json` — TypeScript compiler config targeting ES2020 with strict mode and DOM lib
- `package.json` — TypeScript installed as a dev dependency via npm
- `index.html` — single-page app shell with add form (title + URL fields), error message area, and bookmark list container
- `styles.css` — base layout and typography; form, input, button, and bookmark list styles
- `main.ts` / `main.js` — compiled TypeScript implementing:
  - `Bookmark` interface (`id`, `url`, `title`, `createdAt`)
  - `loadBookmarks` / `saveBookmarks` — localStorage read/write
  - `generateId` — unique ID per bookmark using timestamp + random string
  - `validateTitle` — rejects empty titles
  - `validateUrl` — rejects URLs not starting with `http://` or `https://`
  - `renderBookmarks` — renders bookmark list sorted newest first
  - `handleSubmit` — form submission handler; validates, saves, re-renders, and resets the form

### Layer 1 tasks completed
- [x] Set up project structure
- [x] Define bookmark data type in TypeScript
- [x] Add a bookmark with URL and title
- [x] Display bookmarks in a list, newest first
- [x] Click a bookmark to open it in a new tab
- [x] Persist bookmarks in localStorage
- [x] Validate: reject empty titles
- [x] Validate: reject invalid URLs
