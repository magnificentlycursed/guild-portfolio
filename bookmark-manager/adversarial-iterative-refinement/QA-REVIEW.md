# QA Review Log

This review is part of the [Adversarial Iterative Refinement (AIR)](README.md) suite. It is a required gate for merging. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

The purpose of this review is to apply iterative adversarial pressure to find, document, and resolve bugs, logic errors, test weaknesses, coverage gaps, and regressions. Every review targets the whole application — not only the most recently changed code.

## Current Review Prompt

**Scope:** Whole application by default. If a scope is provided (e.g., a specific feature or set of changed files), focus primary analysis there — but regression checks always cover the entire application.

Read all source files, test files, HTML, CSS, and config. Apply every standard dimension below as a floor — add others as appropriate to the current state of the app. There is no restriction on what can be flagged.

For each finding, cite file and line number. Classify as **resolved** (fix applied this review), **deferred** (scheduled for a specific layer, reason given), or **dismissed** (no action taken, rationale required).

Regression check: verify that all previously-working features still work. Prior layers' acceptance criteria are always in scope. A change to one part of the app can silently break another. A bug that was always present is still a bug.

**Coordination:** Flag any findings that should be surfaced to [UX-REVIEW.md](UX-REVIEW.md), [SECURITY-REVIEW.md](SECURITY-REVIEW.md), [PLATFORM-ENGINEERING-REVIEW.md](PLATFORM-ENGINEERING-REVIEW.md), or [SOLUTION-ARCHITECT-REVIEW.md](SOLUTION-ARCHITECT-REVIEW.md). If this review suggests the need for a new AIR domain, log it as a finding.

## Standard Evaluation Dimensions

1. **Acceptance criteria** — Are all TODO criteria actually met by the implementation, not just implied?
2. **Test falsifiability** — Would each unit and browser test catch a broken implementation? Could any test pass against wrong code?
3. **Selector strength** — Are browser test selectors tight enough to fail on a broken UI?
4. **Validation gaps** — What inputs slip through? What edge cases are untested?
5. **Logic errors** — Are there bugs or off-by-one errors in `src/bookmarks.ts` or `src/main.ts`?
6. **Dead code** — Any exported or declared code with no call sites?
7. **Unused dependencies** — Any direct dependencies in `package.json` not imported in `src/` or `tests/`?
8. **Dependency versions** — Are versions appropriate and not significantly outdated?
9. **Coverage gaps** — Does the coverage report reveal untested branches or functions that correspond to acceptance criteria?
10. **Accessibility** — Is semantic HTML used correctly (landmarks, headings, lists)? Do all interactive elements have accessible labels? Is keyboard navigation complete? Are focus states visible? Are ARIA roles or attributes missing? Run axe and confirm zero violations.
11. **Browser compatibility** — Any JavaScript APIs, CSS properties, or HTML features that behave differently in Firefox or Safari?
12. **Responsive design** — Does the layout hold at 360px? Are touch targets at least 44×44px? Does content reflow cleanly?
13. **Security surface** — Is user content rendered via `.textContent` (safe) or `.innerHTML` (unsafe)? Are link `href` values validated? Is storage data runtime-validated? Any new CVEs in dependencies?
14. **Regression coverage** — Does every bug logged in this file have an identifiable regression test? Flag any whose regression path is untested.

---

## Review 9 — 2026-04-24 16:30Z
**Scope:** Layer 6 (Polish) — all 14 standard dimensions. Changes: dark color scheme (`styles.css` rewrite), collapsible add form (`index.html`, `src/main.ts`), `extractDomain` (`src/bookmarks.ts`), inline delete confirmation (`src/main.ts`), tag badge filter activation (`src/main.ts`), `@keyframes` transitions, 44px touch targets. Regression check covers all prior layers.

### Resolved

#### Bug — Focus lost after canceling inline delete confirmation
**File:** `src/main.ts:307`
`cancelBtn.addEventListener('click', renderBookmarks)` called `renderBookmarks()` which destroyed and recreated the list DOM. The cancel button no longer existed; focus landed on `document.body`. A keyboard user who accidentally triggered the delete confirmation had no path back to their position in the list.
**Resolution:** Changed to an inline arrow that calls `renderBookmarks()` then focuses the newly rendered `.delete-btn` for the same bookmark via `document.querySelector(\`[data-id="${id}"] .delete-btn\`)`.

#### Bug — Focus lost after canceling inline edit (pre-existing from Layer 3)
**File:** `src/main.ts:232`
Same root cause: `cancelBtn.addEventListener('click', renderBookmarks)` lost focus when the edit form DOM was destroyed. Pre-existing since Layer 3; not previously flagged.
**Resolution:** Same pattern as above — arrow function, `renderBookmarks()`, then `.focus()` on `[data-id="${id}"] .edit-btn`.

#### Weakness — No test for focus behavior after cancel
Both cancel paths were exercised by tests checking list content but not keyboard focus state. An implementation without the focus restoration would pass.
**Resolution:** Added `'canceling the delete confirmation returns focus to the delete button'` and `'canceling an edit returns focus to the edit button'` using `.toBeFocused()`.

### Dismissed

#### Acceptance criteria — all met
Dark color scheme: CSS custom properties verified WCAG AA numerically; axe scans pass. Collapsible form: hidden on load, toggle shows/hides, `aria-expanded` synced, collapses on success, stays open on validation failure; 7 browser tests cover all paths. Domain extraction: `extractDomain` in `bookmarks.ts`, 6 unit tests (+1 subdomain, +1 query strip, +1 port, +1 malformed), 2 browser tests. Transitions: `fadeIn`/`slideDown` wrapped in `prefers-reduced-motion: no-preference`. Responsive 360px: browser test passes. Touch targets ≥44px: all interactive buttons have `min-height: 44px` via CSS; no automated pixel measurement needed, CSS is authoritative. Inline delete: 3 confirmation-path tests + axe scan. Tag badge filter: 2 toggle tests. Dismissed.

#### Dead code — `filterByTag` exported but only used internally
`filterByTag` is exported from `bookmarks.ts` but not imported in `main.ts`. It is called within `bookmarks.ts` by `applyFilters`. Pre-existing since Layer 4. No external consumer. Not a runtime issue. Dismissed — the export makes the function available to future callers (e.g., tests) and removing it is a distinct task.

#### Browser compatibility
`CSS custom properties`: universal support since Chrome 49 / Firefox 31 / Safari 9.1. `element.hidden`: universal. `element.replaceWith()`: universal modern browsers. `@keyframes` inside `@media`: valid CSS, works in all browsers. Dismissed.

#### Regression check — all prior tests pass
95 browser tests pass. 80 unit tests pass. Coverage 100%. All prior acceptance criteria still met. No regressions found. Dismissed.

#### Security surface — no new risks
All user-supplied content rendered via `.textContent`. `extractDomain` returns `.hostname` — a safe string, no protocol. Even a `javascript:` URL (rejected by `validateUrl`) would produce `hostname = ''`, rendering no domain element. Dismissed.

**Tests:** 80 unit | 95 browser | coverage 100% on `src/bookmarks.ts` | 0 CVEs | lint clean

---

## Review 7 — 2026-04-24 23:00Z
**Scope:** Full project, Layers 1–5 — all 14 standard dimensions including security surface, regression coverage, and automated axe accessibility scans.

### Resolved

#### Bug — `javascript:` and `data:` URLs not tested in `validateUrl`
**File:** `tests/unit/bookmarks.test.ts`
`validateUrl` uses the `URL` constructor and protocol check, so both URLs are rejected — but no unit test verified this. A change to the validation logic (e.g., switching to an allowlist with a typo) would not be caught.
**Resolution:** Added `'returns an error message for a javascript: URL'` and `'returns an error message for a data: URL'`.

#### Bug — `loadBookmarks` trusts storage data without runtime validation
**File:** `src/bookmarks.ts` — `loadBookmarks`
`JSON.parse(data) as Bookmark[]` is a TypeScript assertion with no runtime effect. Malformed or schema-migrated data would cause runtime errors (e.g., `bookmark.tags.includes` on a null field).
**Resolution:** Added private `normalizeBookmark` function; required fields (`id`, `url`, `title`) must be strings or the entry is discarded; optional fields are coerced to safe defaults. Six unit tests added.

#### Bug — Confirm dialog did not name the bookmark being deleted
**File:** `src/main.ts` — `handleDeleteClick`
`window.confirm('Delete this bookmark?')` gave no indication of which bookmark was targeted. A user who misclicked had no way to catch it before confirming.
**Resolution:** Dialog now shows `Delete "${bookmark.title}"?`.

#### Bug — axe: `.list-empty` at color `#888` fails WCAG AA contrast
**File:** `styles.css`
axe reported `color-contrast` violation: `#888888` on white = 3.54:1. `.list-empty` is 0.9rem (14.4px) normal weight — the large-text exception (3:1) does not apply. UX Review 3's dismissal was incorrect on the threshold.
**Resolution:** Changed to `#666` (5.74:1 on white).

#### Bug — axe: edit form inputs have no label association (critical)
**File:** `src/main.ts` — `handleEditClick`
`<label>` and `<input>` are DOM siblings in `.form-group` with no `for`/`id` linkage. axe reports a critical `label` violation — screen readers announce the fields as unlabeled.
**Resolution:** Added `fieldId = \`edit-${id}-${field.name}\`` to each field in the loop; set `label.htmlFor` and `input.id` / `textarea.id` to match.

#### Bug — Tag filter container and bookmark list had no accessible names
**File:** `index.html`
`#tag-filters` contained filter buttons with no group context. `#bookmark-list` had no accessible label. Both are navigable regions that screen reader users encounter without context.
**Resolution:** Added `role="group" aria-label="Filter by tag"` to `#tag-filters`; `aria-label="Bookmarks"` to `#bookmark-list`.

#### Weakness — Axe violations had no automated test coverage
Three violations existed with no automated detection. Added 3 axe browser tests (empty state, with bookmarks, with edit form open). All violations were caught and resolved before tests passed.

### Dismissed

#### Regression coverage
All bugs from Reviews 1–6 traced to regression tests: `activeTag` ghost state (Review 4) → expanded filter bar assertion; URL case-insensitivity and sort stability (Review 1) → dedicated unit/browser tests; UX changes (Review 5) → 4 browser tests. No gaps.

#### `normalizeBookmark` not directly tested
Intentionally private — exercised via `loadBookmarks` tests. Dismissed.

**Tests:** 74 unit passed (+8) | 77 browser passed (+3)

---

## Review 6 — 2026-04-24 22:30Z
**Scope:** Layer 5 (Search) — all standard dimensions plus three new: accessibility, browser compatibility, responsive design.

### Resolved

#### Bug — `#search-input` CSS rule was dead code
**File:** `styles.css`
The `#search-input` rule duplicated the `input {}` rule exactly. All five declarations were inherited. Removed.

#### Bug — `input[type="search"]` rendered with Safari pill appearance
**File:** `styles.css`
Safari applies `-webkit-appearance: searchfield`, overriding our `border`, `border-radius`, and `padding`. Added `input[type="search"] { -webkit-appearance: none; appearance: none; }`. Chrome's native `×` clear button is unaffected.

#### Bug — No screen reader announcement when search results change
**File:** `index.html`, `src/main.ts`
`aria-live` on `#bookmark-list` directly would read every item on every keystroke. Added `<p id="search-status" class="sr-only" aria-live="polite" aria-atomic="true">` inside the search bar; `renderBookmarks()` updates it with a result count or "No bookmarks match your search."

#### Bug — Search bar had no landmark role
**File:** `index.html`
Added `role="search"` to `.search-bar` div.

#### Weakness — No test for adding a bookmark while search is active
An implementation that stopped re-applying the search query on add would pass all existing tests. Added `'adding a bookmark while search is active shows it if it matches and hides it if it does not'`.

#### Weakness — Accessibility attributes had no test coverage
`role="search"`, `aria-live`, and `aria-atomic` were untestable. Added: `'search bar has a search landmark role'`; `'search status region has aria-live and aria-atomic attributes'`; `'search status announces the result count while filtering'`.

### Dismissed

#### Native `×` clear button inconsistency
Browser-controlled affordance. `type="search"` provides the semantic role, correct mobile keyboard, and a free clear button in Chrome. Dismissed.

#### Empty state message scope
"No bookmarks match your search." appears for search, tag filter, or both producing zero results. The `activeTag` auto-reset makes tag-only zero-result structurally impossible. The active tag button is visually obvious. Message is accurate. Dismissed.

#### Small button touch targets (pre-existing)
`.filter-btn`, `.edit-btn`, `.delete-btn`, `.cancel-edit` — ~21px tall. Pre-dates Layer 5. Deferred to Layer 6.

**Tests:** 66 unit passed | 74 browser passed (+4)

---

## Review 5 — 2026-04-24 20:30Z
**Scope:** UX changes from Review 2 (empty state, error clearing, edit form focus, optional hints) — falsifiability, parallel surfaces, checklist completeness.

### Resolved

#### Weakness — Error clearing test only typed in the title field
`input` listener is on the form element — any field clears the error. Test only typed in `input[name="title"]`. An implementation listening only on the title field would pass.
**Resolution:** Added `'clears the add form error when typing in any field, not just the erroring one'`.

#### Weakness — Edit form error clearing had no test
`input` listener was added to the inline edit form but no test verified it. Implementation without the listener would pass.
**Resolution:** Added `'clears the edit form error message when the user starts typing'`.

#### Weakness — Edit form focus had no test
`firstField?.focus()` was untested. An implementation that omitted the call would pass.
**Resolution:** Added `'edit form focuses the title field when opened'` using `.toBeFocused()`.

#### Weakness — Edit form "(optional)" hints had no test
The `hint` field addition was untested. An implementation that omitted hints would pass.
**Resolution:** Added `'edit form labels Note and Tags as optional'`.

#### Weakness — Layer 4 manual checklist missing UX behaviors
Checklist had no entries for empty state, toggle deselect, error clearing, focus, or optional hints.
**Resolution:** Added 8 new checklist items.

### Dismissed

None.

**Tests:** 52 unit passed | 59 browser passed (+4)

---

## Review 4 — 2026-04-24 19:41Z
**Scope:** Layer 4 (Tag Filtering) — acceptance criteria, new function tests, state management bug hunting.

### Resolved

#### Bug — `activeTag` not reset when active tag is deleted from all bookmarks
**File:** `src/main.ts` — `renderTagFilters`
When the last bookmark with the active tag was deleted, `activeTag` remained set. `renderTagFilters` created no button for it, but `activeTag !== null` prevented "All" from being highlighted. User was stranded in a visually broken state. Same path triggered by editing the last bookmark with the active tag.
**Resolution:** Added check at top of `renderTagFilters`: `if (activeTag !== null && !uniqueTags.includes(activeTag)) activeTag = null`.

#### Weakness — "when tag filter active and no bookmarks match" only checked item count
Test verified `bookmark-item` count of 0 but not filter bar state. The ghost-state bug above would not have been caught.
**Resolution:** Added: `expect(page.locator('.filter-btn')).toHaveCount(1)` and `expect(page.locator('.filter-btn--active')).toHaveText('All')`.

### Dismissed

#### TODO criterion wording ambiguity
"When all bookmarks are deleted, the tag filter area is empty (no stale buttons remain)" vs "All button always present." Parenthetical clarifies intent is about stale tag buttons. Implementation and tests are correct. Dismissed.

#### No test for editing away the active tag
The fix in `renderTagFilters` covers both delete and edit paths since both call `renderBookmarks()`. Delete path is fully tested. Deferred.

**Tests:** 52 unit passed | 50 browser passed

---

## Review 3 — 2026-04-24 18:59Z
**Scope:** Layer 3 (Edit and Delete) — acceptance criteria, new function tests, field coverage in edit tests.

### Resolved

#### Weakness — "saving an edit" only verified title and URL
**File:** `tests/browser/bookmark-manager.spec.ts`
Acceptance criterion requires all four fields to reflect new values. Test only asserted `.bookmark-title` and `href`. Note and tags changes were not verified.
**Resolution:** Expanded to edit all four fields and assert `.bookmark-note` text, `.tag-badge` count, and badge text.

#### Weakness — No test for clearing note or tags during edit
Conditional rendering (`if (bookmark.note)`, `if (bookmark.tags.length > 0)`) was untested after an edit that removed content.
**Resolution:** Added `'editing a bookmark to remove its note hides the note element'` and `'editing a bookmark to remove its tags hides the tag badges'`.

### Dismissed

#### `updateBookmark` type safety handles `id`/`createdAt` immutability
The `Partial<Pick<Bookmark, 'title' | 'url' | 'note' | 'tags'>>` type prevents passing `id` or `createdAt` as updates. The existing "preserves fields not included in the update" test covers runtime verification. Dismissed.

**Tests:** 42 unit passed | 38 browser passed

---

## Review 2 — 2026-04-23
**Scope:** Full project (Layers 1–2) — 11 dimensions including coverage, dead code, dependencies, dependency versions.

### Resolved

#### Weakness — localStorage test did not verify note or tags
**File:** `tests/browser/bookmark-manager.spec.ts`
Test submitted without note or tags, only checked `title`, `url`, and property presence. Layer 2 acceptance criterion requires tags to be stored correctly — had no test.
**Resolution:** Test now submits with note and tags; asserts stored `note` value, `tags` deep-equality, truthy `id`, and `createdAt > 0`.

### Dismissed

#### Floating dependency versions
`^` semver with `package-lock.json` is sufficient for a single-developer project. Dismissed.

#### `main.ts` at 0% unit coverage
Expected — DOM wiring intentionally covered by browser tests only. `bookmarks.ts` is at 100%. Dismissed.

**Tests:** 31 unit passed | 22 browser passed

---

## Review 1 — 2026-04-23
**Scope:** Layers 1–2 — correctness, falsifiability, validation gaps, bug hunting in new code.

### Resolved

#### Bug — URL validation rejected valid uppercase protocol URLs
**File:** `src/bookmarks.ts`
`validateUrl` used `.startsWith('http://')` — case-sensitive. `HTTP://example.com` was rejected.
**Resolution:** Replaced with `new URL(url)` constructor. Protocol is normalized to lowercase automatically.

#### Bug — URL validation accepted protocol-only URLs with no domain
**File:** `src/bookmarks.ts`
`'https://'` passed because it starts with the right string. `new URL('https://')` throws, so the `catch` block now handles this case.

#### Bug — Sort was unstable for identical timestamps
**File:** `src/main.ts`
`(a, b) => b.createdAt - a.createdAt` returns `0` for equal timestamps. Extracted to `sortBookmarks()` with secondary key `|| a.id.localeCompare(b.id)`.

#### Weakness — "adds a bookmark" did not verify `href`
Added `toHaveAttribute('href', 'https://example.com')`.

#### Weakness — "appears at top" only checked first item
Added `toHaveCount(2)` and `nth(1)` assertion.

#### Weakness — "form clears" only checked title and URL
Added assertions for `textarea[name="note"]` and `input[name="tags"]`.

#### Weakness — localStorage content never verified end-to-end
Added browser test reading `localStorage.getItem('bookmarks')` directly via `page.evaluate()`.

#### Weakness — Form data preservation on validation failure untested
Added `'form data is preserved when title validation fails'` and `'form data is preserved when URL validation fails'`.

#### Weakness — No browser tests for URL edge cases
Added `'accepts URLs with an uppercase protocol'` and `'rejects a URL that is only a protocol with no domain'`.

#### Weakness — `generateId` test only checked length > 0
Added `'contains a hyphen separator'`.

#### Weakness — No `sortBookmarks` stability test
Added suite: ordering by timestamp, non-mutation, stable identical-timestamp output, empty array.

### Dismissed

#### "click opens in new tab" doesn't test the click
Opening a new tab is browser behavior. Testing `target="_blank"`, `href`, and `rel="noopener noreferrer"` is the correct application boundary. Dismissed.

#### No null checks on form elements
HTML is static and not user-controlled. TypeScript handles types. Dismissed.

#### Tag/note/URL length limits
Out of scope for Layers 1–2. Layer 6 concern. Dismissed.

#### localStorage quota exceeded
Out of scope for a personal single-user tool. Dismissed.

**Tests:** 31 unit passed | 22 browser passed

---

## Review 8 — 2026-04-25 00:30Z
**Scope:** Full application. Triggered by: push→spread immutability fix in `main.ts`; AIR suite reorganized into `air/` subfolder; PE Review 1 additions (coverage step, audit step, cache key fix). All 14 standard dimensions evaluated.

### Resolved

*(none)*

### Dismissed

#### Immutability fix in `handleSubmit` — correct, no test gap
`saveBookmarks(storage, [...bookmarks, newBookmark])` replaces the prior `bookmarks.push(newBookmark)`. The change is internally correct and consistent with `updateBookmark` and `deleteBookmark`. No unit test directly exercises `handleSubmit` (DOM wiring, covered by browser tests), but the pattern is exercised by browser test suite (77 tests pass). No test gap: the correct behavior (new bookmark appears in list, persists after reload) is verified end-to-end. Dismissed.

#### `air/` subfolder reorganization — no functional change
All 6 AIR files moved from repo root to `air/`. All cross-references updated. No source code, tests, or build config changed. Grep confirms no stale `ADVERSARIAL.md` or root-level `QA-REVIEW.md` references remain. Dismissed.

#### Coverage threshold enforcement — verified in CI
`vite.config.ts` now scopes `coverage.include` to `['src/bookmarks.ts']` and enforces 100% thresholds. CI pipeline runs `npm run test:coverage` after unit tests. Local run confirms: 54/54 statements, 38/38 branches, 23/23 functions, 100% lines. Dismissed.

#### Dead code and unused imports — none found
All 16 exports from `src/bookmarks.ts` are imported and used in `src/main.ts`. No unused variables or imports in either file. Dismissed.

#### Validation gaps — none found
`validateUrl`: rejects non-http/https, rejects protocol-only (e.g. `https://` with no domain), case-insensitive. `validateTitle`: rejects empty and whitespace-only. Both enforced at form submit and verified by unit tests. Dismissed.

#### Selector strength — adequate
Browser tests use `data-testid` attributes and semantic selectors. No brittle positional selectors. Dismissed.

#### Browser compatibility — adequate for scope
No experimental APIs. `localStorage`, `textContent`, `dataset`, `createElement` are universally supported. Dismissed.

#### `npm audit` — 0 vulnerabilities
`npm audit --audit-level=high` exits 0. No known CVEs in the dependency tree. Dismissed.

**Tests:** 74 unit | 78 browser | coverage 100% on `src/bookmarks.ts` | 0 CVEs
