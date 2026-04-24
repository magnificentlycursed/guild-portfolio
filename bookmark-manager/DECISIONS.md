# Bookmark Manager — Refinement Log

### 2026-04-25 01:15Z — ESLint added

ESLint with `typescript-eslint` added following the typescript-eslint getting-started documentation — the approach the TypeScript team recommends. Config: `tseslint.config(eslint.configs.recommended, tseslint.configs.recommended)`. Exits clean against `src/` with no suppressions needed.

Added as `npm run lint` locally and as a `Lint` CI step after `Type check`. Resolves the left-shift gap noted in PE Review 2.

### 2026-04-25 00:30Z — Full AIR suite run; clean pass

All 5 domains ran against the full project after: push→spread immutability fix in `main.ts`, PE Review 1 additions (coverage step, audit step, cache key fix), and AIR suite reorganized into `air/`.

Zero findings across all domains. The reviews confirmed:

- **SA**: Architecture sound. push→spread fix completes the immutability pattern across all write paths. `bookmarks.ts`/`main.ts` boundary intact.
- **PE**: All Review 1 gates (coverage threshold, `npm audit`, corrected cache key) verified present and gating correctly. The double unit-test run (discrete step + `npm test`) is deliberate and acceptable.
- **QA**: 14 dimensions checked. No dead code, no validation gaps, no brittle selectors. 74 unit / 77 browser / 100% coverage / 0 CVEs.
- **UX**: No UX surface changes this session. Prior UX fixes intact (toggle-deselect, color contrast). Three axe scans passing.
- **Security**: All rendering safety controls intact (`.textContent` throughout). `normalizeBookmark` storage validation intact. `npm audit` now a CI gate.

A clean AIR run is a meaningful signal: the changes introduced this session have no detectable defects, regressions, or architectural drift across any domain.

### 2026-04-25 00:10Z — Solution Architect domain added to AIR; SA Review 1

The SA domain evaluates architecture: structural decisions, boundary integrity, data model soundness, and whether the complexity of the implementation is proportional to the problem. It is distinct from QA (which evaluates correctness) and from Security/UX (which evaluate specific concerns) — SA evaluates whether the overall shape of the solution is right.

**One real finding fixed:**

`handleSubmit` in `src/main.ts` called `bookmarks.push(newBookmark)` — a direct mutation of a local array. Every other data operation in the codebase uses immutable patterns: `updateBookmark`, `deleteBookmark`, and `sortBookmarks` all return new arrays via spread or `.filter`/`.map`. The mutation caused no observable bug (the mutated array was immediately saved and discarded), but it was an inconsistency that could mislead future readers or be copied into a context where mutation does cause a bug. Fixed to `saveBookmarks(storage, [...bookmarks, newBookmark])`.

**Architecture assessed as sound:**

The `bookmarks.ts` / `main.ts` separation is clean and consistent across all five layers — no DOM references have leaked into the logic module, and no business logic has accumulated in the DOM wiring module. The `BookmarkStorage` interface is a minimal, well-placed abstraction. The `Bookmark` data model is complete and appropriate for the use case. Module-level `let` state in `main.ts` is the correct pattern for a no-framework SPA at this scale. All significant decisions are documented in this log.

### 2026-04-24 23:55Z — Platform Engineering domain added to AIR; PE Review 1

**Rationale:** The Security, QA, and UX reviews identify defects; PE's job is to make sure those checks — and as many others as possible — are enforced automatically in CI rather than depending on a human running them before each merge. The review takes a left-shift lens: any check that lives only in a manual checklist is a gap.

**Three bugs found and fixed in Review 1:**

1. **Playwright cache key was broken:** `hashFiles('package-lock.json')` resolves from the repo root. The lock file is in `bookmark-manager/`. The hash was always empty, meaning the Playwright browser cache never invalidated on dependency changes. Fixed to `hashFiles('bookmark-manager/package-lock.json')`.

2. **Coverage not enforced in CI:** `npm run test:coverage` was a PR checklist item but not in the pipeline. Coverage could regress to 0% and a PR would still merge. Also, `coverage.include` was `src/**/*.ts` — including `main.ts`, which has 0% unit coverage by design. Any threshold would have always failed because of `main.ts`. Fixed: scoped `include` to `src/bookmarks.ts`; added thresholds (100% across all metrics); added the step to CI. Confirmed: 100% on all dimensions.

3. **`npm audit` not automated:** Manual checklist item in Security review. Added `npm audit --audit-level=high` to CI so a high-severity CVE blocks the pipeline rather than relying on a reviewer remembering to run it.

**Left-shift pattern established:** The axe accessibility scans (added in QA Review 7) are already in the browser test suite which CI runs — they're a good example of the left-shift model. Coverage and audit now follow the same pattern.

### 2026-04-24 23:45Z — AIR suite formalized

The three review domains (QA, UX, Security) have been formalized as the Adversarial Iterative Refinement (AIR) suite with a dedicated coordination document (`adversarial-iterative-refinement/README.md`).

**Rationale:** The reviews were already functioning as a suite — they ran together, shared the goal of adversarial pressure, and collectively formed the merge gate. Naming them as a suite makes the relationship explicit and adds two capabilities that were previously implicit:

1. **Scoped runs** — any domain can be focused on specific changed files or a feature while still running regression checks across the whole app. This prevents the gate from becoming heavyweight for small changes while keeping the regression safety net intact.

2. **Domain suggestion** — any review can propose a new AIR domain as a classified finding. This gives the process a defined path for growth (e.g., Performance, Privacy) without requiring a process redesign.

**Sequencing model:** Domains default to parallel. Sequencing is used when one domain's findings are likely to affect another's analysis (e.g., a Security finding that changes the implementation should precede a QA re-check of that implementation). The orchestrator (person or agent running AIR) decides based on context.

### 2026-04-24 23:00Z — Process expansion: security review added; full-project QA, UX, and security audit

**Process changes:** Security review is now a formal layer gate requirement alongside QA and UX review. `SECURITY-REVIEW.md` follows the same log format and prompt structure. Standard QA dimensions expanded to 14 (added: security surface, regression coverage with explicit axe requirement). Standard UX dimensions expanded to 13 (added: long content overflow, reduced motion, native dialog text quality, cross-layer regression). PR template updated to require security review sign-off.

**`@axe-core/playwright` integrated:** Three axe browser tests run in the test suite (empty state, with bookmarks, with edit form open). This makes accessibility a first-class test assertion that fails the test run rather than depending on reviewer recall. Two violations were found and fixed that had been missed by manual review:

1. **`.list-empty` color contrast** (#888 / 3.54:1): The UX Review 3 dismissal incorrectly applied the large-text exception — 0.9rem / 14.4px normal weight does not qualify. Fixed to `#666` (5.74:1). This is the concrete value of automated scanning: the manual review gave the wrong answer; axe gave the right one.

2. **Edit form unlabeled inputs**: The inline edit form constructed `<label>` and `<input>` as DOM siblings with no `for`/`id` association. Visually correct; programmatically disconnected. Fixed by generating stable IDs and linking labels.

**`loadBookmarks` normalization:** `JSON.parse(data) as Bookmark[]` is a TypeScript assertion with no runtime effect. If localStorage contains data from a previous app version (before a schema change) or a user's manual edit, accessing undefined fields would throw. Added `normalizeBookmark` to validate required fields and coerce optional fields to safe defaults. Discards rather than crashes on invalid entries.

**Long content overflow:** `overflow-wrap: break-word` added to `.bookmark-title`, `.bookmark-note`, `.tag-badge`. A title with no spaces (common for long URLs used as titles) would otherwise extend beyond its container and cause horizontal overflow.

**Confirm dialog specificity:** `window.confirm('Delete this bookmark?')` was generic. Changed to `Delete "${title}"?` — the user sees which bookmark they're about to delete before confirming.

**`prefers-reduced-motion` proactively flagged for Layer 6:** Layer 6 plans CSS transitions. Before implementing, all transitions must be wrapped in `@media (prefers-reduced-motion: no-preference)`. Added to Layer 6 task list and manual testing checklist now, before the work starts.

### 2026-04-24 22:30Z — QA review 6 and UX review 3 (Layer 5)

Four findings resolved across both reviews.

**Dead CSS:** `#search-input` in `styles.css` was an exact duplicate of the `input` rule — all five declarations were already inherited. Removed.

**Safari `type="search"` appearance:** Safari applies `-webkit-appearance: searchfield` to search inputs, overriding our `border`, `border-radius`, and `padding` with a native pill shape. Fixed with `appearance: none` reset. The Chrome native `×` clear button is unaffected — it is rendered outside the appearance model and fires the `input` event, so our search query state updates correctly when clicked.

**Screen reader announcements:** Putting `aria-live` on `#bookmark-list` directly would cause the reader to announce every list item on every keystroke — unusable. The correct pattern is a separate status region: a visually hidden `<p aria-live="polite" aria-atomic="true">` that announces only the result count. Implemented as `#search-status`, populated by `renderBookmarks()` whenever a filter is active. Screen readers announce count changes; sighted users are unaffected.

**Search landmark:** `role="search"` added to the `.search-bar` wrapper. Keyboard users navigating by landmarks can jump directly to the search field without tabbing through the add form and other controls.

One pre-existing issue identified: small button touch targets (~21px for `.filter-btn`, `.edit-btn`, `.delete-btn`, `.cancel-edit`) are below the 44×44px mobile minimum. Pre-dates Layer 5; added to Layer 6 task list.

### 2026-04-24 21:55Z — Layer 5: Search

Two pure functions added to `src/bookmarks.ts`: `searchBookmarks` (case-insensitive substring match on title and note; empty/whitespace query returns input unchanged) and `applyFilters` (composes `filterByTag` then `searchBookmarks`; both conditions must be satisfied).

Tag and search state interact cleanly: each filter click still calls `renderBookmarks()`, which now calls `applyFilters(bookmarks, activeTag, searchQuery)` instead of `filterByTag` directly. No changes to the tag filter logic — search composes on top of it for free.

Empty state message now distinguishes two cases: `bookmarks.length === 0` → "No bookmarks yet. Add one above."; `bookmarks.length > 0 && sorted.length === 0` → "No bookmarks match your search." The same path covers both tag-filter and search producing empty results, since `applyFilters` unifies them.

The search input uses `type="search"` to give users a native clear button on supported browsers with no extra code.

TDD followed throughout: 14 failing unit tests written first, then `searchBookmarks` and `applyFilters` implemented to pass them; 11 failing browser tests written next, then UI implemented to pass them. All 66 unit tests and 70 browser tests pass.

### 2026-04-24 20:38Z — Layer 4 merged into main; PR template updated

Layer 4 (Tag Filtering) merged into main after all gate requirements passed: automated tests (52 unit, 59 browser), manual testing checklist, adversarial QA review (Reviews 4 and 5), and UX review (Reviews 1 and 2).

PR template updated to include UX review as a required checklist item alongside adversarial QA review. All future layers must complete a UX review and log findings in `UX-REVIEW.md` before merging.

### 2026-04-24 20:30Z — QA review 5: UX change test gaps closed

Four test weaknesses found against the UX changes from Review 2. Every UX change (empty state, error clearing, edit form focus, optional hints) had no corresponding browser test — the implementation was correct but unverifiable. Added 4 browser tests covering all four changes, plus a fifth test verifying that error clearing works for any field on the form, not just the one that triggered the error. The Layer 4 manual checklist was also found to be missing all UX-related behaviors; 8 new checklist items added.

### 2026-04-24 20:24Z — UX review established as a formal layer gate; UX review 2 findings resolved

UX review is now a required part of the layer completion gate alongside adversarial QA review and manual testing. The prompt, evaluation criteria, and findings log live in `UX-REVIEW.md`. `DESIGN.md` and `TODO.md` updated to reflect this.

Four findings from Review 2 were implemented:

**Empty state:** The bookmark list had no message when empty — first-time users and users who deleted all bookmarks saw a blank area with no context. Added a `<li class="list-empty">` with a prompt to add a bookmark. Note: with the Layer 4 `activeTag` auto-reset fix, a filter-active-but-empty state is structurally impossible — the empty state message always means no bookmarks in storage.

**Error clearing on input:** The add form and inline edit form errors persisted visually while the user was typing to fix them. Added `input` event listeners on both forms to clear the error immediately when any field receives input.

**Edit form focus:** Opening the inline edit form left keyboard focus on the now-replaced Edit button. Added `firstField?.focus()` immediately after the form is injected into the DOM.

**Edit form "(optional)" hints:** The add form marks Note and Tags as optional; the edit form didn't. Added a `hint` field to the edit form field descriptor array so the same `<span class="optional">` pattern is used in both places.

Two findings deferred to Layer 6: `window.confirm` replacement (inline confirmation UI) and tag badge click-to-filter. Both added to Layer 6 task list in `TODO.md`.

### 2026-04-24 20:17Z — Tag filter toggle deselect and empty URL validation message

**Tag filter toggle:** Manual testing revealed that clicking an active tag button had no effect — the only way to deselect a filter was to click "All." UX review confirmed toggle deselect is a universal expectation across iOS, Android, e-commerce, and content apps. Fixed with a one-character change: `activeTag = tag` → `activeTag = activeTag === tag ? null : tag`.

**Empty URL message:** Manual testing revealed that submitting the form with no URL showed "URL must start with http:// or https://" — a message about format, not presence. Added an explicit empty check to `validateUrl` (mirroring `validateTitle`), returning "URL cannot be empty" before the URL constructor is invoked. This gives users the right signal about what's wrong.

**UX review on multi-select:** Review 1 in `UX-REVIEW.md` evaluates AND vs OR for multi-tag selection. OR is the correct model for this UI pattern and use case. Multi-select deferred — single-select is clean and unambiguous at this stage.

### 2026-04-24 19:41Z — Adversarial QA review 4 (Layer 4)

One bug and one test weakness found and resolved. No new dependencies. Coverage for `bookmarks.ts` remains 100%.

**Bug:** `activeTag` state was not reset when the active tag's last bookmark was deleted. `renderTagFilters` was building the filter bar without checking whether `activeTag` still corresponded to an existing tag. Result: no button highlighted, user stranded in empty filtered state with no visual cue. Fixed by checking `!uniqueTags.includes(activeTag)` at the top of `renderTagFilters` and resetting to `null` if true. The same path covers the edit scenario (editing the last bookmark with the active tag to remove it), since both delete and edit call `renderBookmarks()`.

**Test weakness:** The "when a tag filter is active and no bookmarks match" browser test only checked `bookmark-item` count. Expanded to also assert `filter-btn` count is 1 and `filter-btn--active` text is "All".

One finding dismissed: no separate test for the edit-removes-active-tag path, since the fix covers both paths and adding a dedicated test for the edit case is deferred.

### 2026-04-24 19:36Z — Layer 4: Tag Filtering

Two pure functions added to `src/bookmarks.ts`: `getUniqueTags` (Set-based deduplication, alphabetically sorted) and `filterByTag` (array filter by tag inclusion). Both follow the established immutable pattern — no input mutation.

Tag filter state is held as `let activeTag: string | null = null` at module level in `main.ts`. `renderBookmarks()` calls `renderTagFilters()` on each render, which rebuilds the filter bar from scratch (simpler than patching it in place and avoids stale button state after add/delete). The "All" button always appears first; tag buttons are ordered alphabetically via `getUniqueTags`. Active state is applied by comparing `activeTag` against each button's tag at render time rather than by toggling classes imperatively — consistent with how the rest of the render functions work.

Adding a bookmark while a filter is active is handled for free: `renderBookmarks()` already reads from storage and re-applies `filterByTag`, so the new bookmark appears immediately if its tags match and stays hidden if they don't.

TDD followed throughout: 10 failing unit tests written first, then `getUniqueTags` and `filterByTag` implemented to pass them; 12 failing browser tests written next, then UI implemented to pass them. All 52 unit tests and 50 browser tests pass. Coverage for `bookmarks.ts` remains at 100%.

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

### 2026-04-24 — Typecheck fixes for CI

Two errors surfaced when `tsc --noEmit` ran in CI that had not been caught locally.

**`vite.config.ts`:** `defineConfig` was imported from `vite`. The `vite` package's type for `defineConfig` does not include a `test` property — that extension is provided by `vitest/config`. Switching the import to `vitest/config` resolves the TS2769 overload error without any behavioral change.

**`tsconfig.json`:** `target` and `lib` were set to `ES2020`. `happy-dom` (pulled in as a transitive dependency by Vitest) references `WeakRef` in its type declarations. `WeakRef` was introduced in ES2021 and is absent from the ES2020 lib, causing `Cannot find name 'WeakRef'` errors. Bumped both `target` and `lib` to `ES2021` to resolve this. ES2021 is safe for this project — it targets modern browsers via Vite and Chromium via Playwright.

The errors did not appear locally because local `tsc` invocations were passing previously. CI exposed them by running `tsc` in a clean environment where the errors could not be masked.

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

### 2026-04-24 19:17Z — GitHub PR template

Added `.github/PULL_REQUEST_TEMPLATE.md` at the repo root. The template encodes the full layer gate process as a PR checklist so it is enforced at merge time rather than relying on memory. Mirrors the layer-transition gate in TODO.md: automated tests, coverage, manual testing, QA review, and all three log files must be checked before merging.

### 2026-04-24 19:12Z — MVP complete: Layers 1–3 gate closed

All three MVP layers passed human manual testing against the running app on 2026-04-24. Each layer has passed all three gate requirements: automated tests (unit + browser), adversarial QA review, and manual testing checklist. The branch is ready to merge into main.

Development will continue on the branch for Layers 4–6 (tag filtering, search, polish).

### 2026-04-24 18:59Z — Adversarial QA review 3 (Layer 3)

Two test weaknesses found and resolved. No bugs. Coverage 100% for `bookmarks.ts`.

The "saving an edit updates displayed values" test only verified title and URL after saving — the acceptance criterion explicitly requires all four fields (title, URL, note, tags) to reflect new values. Test expanded to edit all four fields and assert all four in the rendered output.

No test covered the case of clearing a note or tags during an edit (removing content rather than changing it). The conditional rendering in `renderBookmarks` (`if (bookmark.note)`, `if (bookmark.tags.length > 0)`) handles this but the behaviour after an edit was untested. Two new tests added to cover this path.

One dismissed finding: the `Partial<Pick<Bookmark, 'title' | 'url' | 'note' | 'tags'>>` type signature on `updateBookmark` structurally prevents `id` and `createdAt` from being passed as updates, so the TypeScript compiler is the primary guard. The existing "preserves fields not included in the update" test verifies this at runtime for those fields.

### 2026-04-24 18:52Z — Layer 3: Edit and Delete

Two new pure functions added to `src/bookmarks.ts`: `updateBookmark` (maps over the array, replacing the matched bookmark with spread + updates) and `deleteBookmark` (filters out the matched id). Both return new arrays and do not mutate their input, consistent with the existing immutable pattern in the module.

Inline editing replaces a bookmark `li`'s contents with a dynamically constructed form rather than toggling visibility of hidden fields. This avoids duplicating the form structure in HTML and keeps the DOM minimal. `data-id` added to each rendered `li` so `handleEditClick` can locate the right element by id. The edit form re-uses `validateTitle` and `validateUrl` from `bookmarks.ts`, maintaining a single source of truth for validation logic.

Delete uses `window.confirm` for confirmation — no custom modal needed at this layer. Playwright handles `window.confirm` via `page.on('dialog', ...)`, allowing both accept and dismiss paths to be tested end-to-end.

TDD followed throughout: 11 failing unit tests written first, then `updateBookmark` and `deleteBookmark` implemented to pass them; 14 failing browser tests written next, then UI implemented to pass them. All 42 unit tests and 36 browser tests pass.

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

Conducted a structured adversarial review using a QA prompt asking an independent agent to evaluate whether acceptance criteria were met, whether tests were falsifiable, and whether there were bugs or missing edge cases. Full prompt, findings, and dispositions recorded in `QA-REVIEW.md`.

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
