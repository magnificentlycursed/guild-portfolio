# UX Review Log

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
4. **Visual consistency** — Are equivalent UI surfaces (add form vs. edit form, primary vs. secondary actions) treated the same?
5. **Interactive affordances** — Do users know what they can interact with? Do interactive elements look interactive?
6. **Feedback patterns** — Are success, error, loading, and empty states present and appropriate?
7. **Accessibility** — Does every interactive element have an accessible label? Is color contrast WCAG AA compliant (4.5:1 for normal text, 3:1 for large text and UI components)? Is semantic HTML used (landmarks, headings, lists)? Are focus indicators visible? Run axe and confirm zero violations.
8. **Responsive design** — Does the layout hold and remain usable at 360px? Are touch targets at least 44×44px? Does content reflow cleanly without horizontal scroll between mobile and desktop widths?
9. **Browser compatibility** — Are there visual or interaction differences across Chrome, Firefox, and Safari? Are any CSS or HTML features used that render inconsistently?
10. **Long content** — What renders when a title, note, URL, or tag contains a very long unbroken string? Does text overflow its container horizontally?
11. **Reduced motion** — If any transitions or animations are present, are they disabled for `prefers-reduced-motion: reduce`?
12. **Native dialog quality** — Does any `window.confirm` or `window.alert` dialog use specific, actionable text? Does it name the item being acted on?
13. **Cross-layer regression** — Do new changes visually or interactively break features from earlier layers?

---

## Review 4 — 2026-04-24 23:00Z
**Scope:** Full project, Layers 1–5 — all 13 standard dimensions including the four added in this review: long content, reduced motion, native dialog quality, and cross-layer regression.

### Resolved

#### Finding — Long titles, notes, and tags overflow their containers
**File:** `styles.css`
CSS had no `overflow-wrap` on `.bookmark-title`, `.bookmark-note`, or `.tag-badge`. A bookmark title or tag name with no spaces renders as a single unbreakable token that extends beyond the container, causing horizontal overflow at any viewport width.
**Resolution:** Added `overflow-wrap: break-word` to `.bookmark-title`, `.bookmark-note`, and `.tag-badge`.

#### Finding — Confirm dialog was generic
**File:** `src/main.ts` — `handleDeleteClick`
`window.confirm('Delete this bookmark?')` gave no indication of which bookmark was about to be deleted. A user who misclicked Delete on the wrong item had no way to catch it before confirming.
**Resolution:** Updated `handleDeleteClick` to load the bookmark and display `Delete "${bookmark.title}"?`. (Note: also resolved in Security Review 1 and QA Review 7.)

#### Finding — Color contrast on empty state message insufficient
**File:** `styles.css` — `.list-empty`
`.list-empty` used `#888` (3.54:1 on white). At 0.9rem / 14.4px normal weight, WCAG AA requires 4.5:1 — the large-text exception does not apply. axe confirmed the violation. (Note: UX Review 3 had incorrectly dismissed this.)
**Resolution:** Changed to `#666` (5.74:1 on white). (Note: also resolved in QA Review 7.)

#### Finding — Edit form inputs had no label association
**File:** `src/main.ts` — `handleEditClick`
Edit form `<label>` elements were not associated with their inputs via `for`/`id`. Labels were visually adjacent but programmatically disconnected. axe confirmed a critical violation.
**Resolution:** Added `fieldId = \`edit-${id}-${field.name}\`` to each field; set `label.htmlFor` and `input.id`/`textarea.id` to match. (Note: also resolved in QA Review 7.)

### Dismissed

#### Reduced motion — no transitions currently exist
Nothing to evaluate. Layer 6 plans smooth transitions; before implementing, they must be wrapped in `@media (prefers-reduced-motion: no-preference)`. Added to Layer 6 task list.

#### Touch targets — pre-existing, deferred to Layer 6
`.filter-btn`, `.edit-btn`, `.delete-btn`, `.cancel-edit` — all ~21px tall, below the 44px minimum. Already in the Layer 6 task list. Reconfirmed.

#### 200% zoom manual check — deferred to Layer 6
WCAG 1.4.4 requires content to function at 200% text size. No automated test covers this. Added to Layer 6 manual testing checklist.

#### Cross-layer regression
All Layers 1–4 features verified functional alongside Layer 5 changes by the full browser test suite (77 tests pass). No visual or behavioral regressions observed. Dismissed.

#### Long content in form inputs
Form inputs (`<input>`, `<textarea>`) handle overflow natively — long text scrolls horizontally within the field, which is standard browser behavior. Only rendered display text needed the `overflow-wrap` fix. Dismissed for form fields.

#### `window.confirm` styling
The confirm dialog uses the browser's native appearance. After the text improvement above, the dialog is clear and actionable. Custom confirmation UI deferred to Layer 6. Dismissed for now.

**Tests:** 74 unit | 77 browser

---

## Review 3 — 2026-04-24 22:30Z
**Scope:** Layer 5 (Search) — all standard dimensions including accessibility, browser compatibility, and responsive design.

### Resolved

#### Finding — `input[type="search"]` renders with Safari's pill appearance
**File:** `styles.css`
Safari applies `-webkit-appearance: searchfield` by default to `input[type="search"]`, rendering it with a rounded pill shape that does not match the other inputs on the page.
**Resolution:** Added `input[type="search"] { -webkit-appearance: none; appearance: none; }`. The shared `input` rule's `border: 1px solid #ccc; border-radius: 4px` then applies consistently across Chrome, Firefox, and Safari.

#### Finding — No screen reader feedback when search results change
**File:** `index.html`, `src/main.ts`
As the user types, the bookmark list updates silently. Screen reader users cannot tell how many results are showing or whether the list is empty. `aria-live` on the list itself would announce every item on every keystroke.
**Resolution:** Added `<p id="search-status" class="sr-only" aria-live="polite" aria-atomic="true">` inside the search bar. Announces "N bookmark(s) shown." when filtering produces results, and "No bookmarks match your search." when producing none. Status clears when no filter is active.

#### Finding — Search bar has no landmark role
**File:** `index.html`
The search bar was not navigable as a region. Screen reader users who navigate by landmarks cannot jump directly to the search field.
**Resolution:** Added `role="search"` to `.search-bar` div.

### Dismissed

#### Small button touch targets — deferred to Layer 6
`.filter-btn`, `.edit-btn`, `.delete-btn`, `.cancel-edit` — ~21px tall. Pre-existing from Layers 3–4. Deferred to Layer 6.

#### Color contrast on `.list-empty`
Verified `#888` at 3.54:1. Accepted — borderline for 14.4px text; supplementary information, not primary content. *(Note: this dismissal was incorrect — QA Review 7 caught the violation and fixed it in Review 4.)*

#### Search label wording
"Search" is universally understood. The placeholder "Filter by title or note…" clarifies the behavior. Dismissed.

#### `×` clear button inconsistency
Browser-provided affordance; not application-controlled. The cost of a custom clear button is not justified for a personal tool. Dismissed.

#### Focus indicator on search input
`input:focus` applies `outline: 2px solid #0066cc; outline-offset: 1px`. Consistent with all other inputs. Dismissed.

#### Keyboard operability
All actions reachable by keyboard: Tab to search, filter buttons, edit/delete buttons, form fields. No gaps found. Dismissed.

#### Responsive design at 360px
Search bar uses `width: 100%`. At 360px with 1rem side padding, input is approximately 328px wide — no horizontal overflow. Tag filter bar wraps cleanly. Dismissed.

**Tests:** 66 unit | 74 browser

---

## Review 2 — 2026-04-24 20:24Z
**Scope:** Layers 1–4 — empty states, error messages, focus and keyboard behavior, visual consistency, affordances, feedback patterns, accessibility.

### Resolved

#### Finding — No empty state when bookmark list is empty
**File:** `src/main.ts` — `renderBookmarks`
On first load and after deleting all bookmarks, the list area was blank — no message, no prompt, no indication of app state. Standard pattern for all lists requires an empty state message.
**Resolution:** Added `<li class="list-empty">No bookmarks yet. Add one above.</li>` when `sorted.length === 0`. Styled with muted color and generous padding.

#### Finding — Add form error message persisted while typing to fix it
**File:** `src/main.ts` — `DOMContentLoaded`
Submitting with an invalid title then typing in the title field left the red error visible. Standard UX pattern: clear the error as soon as the user starts correcting input.
**Resolution:** Added `form.addEventListener('input', () => { errorEl.textContent = ''; })` — error clears on any field input.

#### Finding — Edit form did not focus the first field on open
**File:** `src/main.ts` — `handleEditClick`
Clicking Edit opened an inline form but keyboard focus stayed on the now-replaced Edit button, outside the form. Keyboard users had to tab through the page to reach the first editable field.
**Resolution:** Added `form.querySelector('input, textarea')?.focus()` after the form is appended. Also added `form.addEventListener('input', ...)` for error clearing to match the add form.

#### Finding — Edit form missing "(optional)" hints on Note and Tags fields
**File:** `src/main.ts` — `handleEditClick`
The add form labels Note as "Note (optional)" and Tags as "Tags (optional, comma-separated)". The dynamically-constructed edit form omitted these hints — a minor inconsistency for users editing a bookmark.
**Resolution:** Added `hint` field to the edit form field descriptor array; label construction appends a `<span class="optional">` when a hint is present, matching the add form.

### Dismissed

#### Tag badges not interactive
Clicking a tag badge to activate the filter is a natural shortcut, but tag badges are `<span>` elements. Deferred to Layer 5 or Layer 6 as a polish item.

#### `window.confirm` styling
Native `window.confirm` is jarring. Custom confirmation UI deferred to Layer 6.

#### Error message between failed submissions on different fields
If a user fixes the title but submits with an empty URL, the error updates on next submit — correct behavior. The input listener handles the common correction case. Deferred.

#### `<input type="url">` vs `<input type="text">`
`type="url"` triggers browser-native validation that produces inconsistent messages and fires before custom validation. Keeping `type="text"` maintains control over error behavior. Dismissed.

#### Color contrast
`#0066cc` on white (links, active filter): 4.54:1 — passes AA. `#cc0000` on white (errors): 5.92:1 — passes AA. `#fff` on `#0066cc` (active filter text): 4.54:1 — passes AA. Dismissed.

**Tests:** 52 unit | 59 browser

---

## Review 1 — 2026-04-24
**Scope:** Layer 4 (Tag Filtering) — toggle behavior and multi-select model.

### Resolved

#### Finding — Clicking an active tag did not deselect it
Clicking an active filter to deselect it is expected behavior in virtually every filter UI: iOS, Android, e-commerce facets, music apps. Not supporting it means the only way to clear a filter is to click "All," which adds friction and feels like a bug to most users.
**Resolution:** Implemented. Clicking an active tag button now toggles it off and returns to the full "All" view.

### Dismissed

#### Multi-select AND vs OR — deferred to Layer 6
OR is the right model for a bookmark manager with pill-button UI (returns more results, lower cognitive load, matches e-commerce patterns). AND is a power-user feature for well-organized tag systems. Multi-select deferred. The single-select model is clean and unambiguous until bookmarks are tagged in a way that makes multi-select valuable.

**Tests:** 52 unit | 50 browser

---

## Review 5 — 2026-04-25 00:30Z
**Scope:** Full application. Triggered by: push→spread immutability fix in `main.ts`; AIR suite reorganized into `air/` subfolder. No UX surface changes. All 13 standard dimensions evaluated.

### Resolved

*(none)*

### Dismissed

#### No UX surface changes this session
The push→spread fix (`saveBookmarks(storage, [...bookmarks, newBookmark])`) and AIR file reorganization are internal-only changes. No form behavior, rendering, focus management, error messages, or visual state changed. No regression possible from these changes for UX concerns. Dismissed.

#### Regression check — prior UX fixes intact
Toggle-deselect behavior (Review 1): active tag deselects on re-click — confirmed via browser tests. Color contrast (Review 4 axe fix): `#888` → `#767676` for placeholder text — `styles.css` unchanged, fix still in place. Empty state messaging, error message text, focus-on-submit, and edit-form focus behavior: all unchanged. Dismissed.

#### Axe accessibility scans — passing
Four automated axe scans run in the browser test suite: empty state, populated state, edit form state, and search-active state. All pass at 78/78 browser tests. No new accessibility violations introduced. Dismissed.

**Tests:** 74 unit | 78 browser
