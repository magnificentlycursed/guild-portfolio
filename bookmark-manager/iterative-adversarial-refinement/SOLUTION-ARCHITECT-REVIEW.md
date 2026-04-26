# Solution Architect Review Log

This review is part of the [Iterative Adversarial Refinement (IAR)](README.md) suite. It is a required gate for merging. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

The purpose of this review is to evaluate whether the architecture — its structure, boundaries, decisions, and tradeoffs — is sound, coherent, and appropriate for the project's stated purpose and constraints. Every review targets the whole application, not only the most recently changed code.

## Current Review Prompt

**Scope:** Whole application by default. If a scope is provided (e.g., a specific feature or set of changed files), focus primary analysis there — but architectural concerns frequently cross boundaries, so adjacent code is always fair game.

Read all source files, the design document, and the refinement log. Apply every standard dimension below as a floor — add others as appropriate to the current state of the app. There is no restriction on what can be flagged.

For each finding, cite file and line number. Classify as **resolved** (fix applied this review), **deferred** (scheduled for a specific layer, reason given), or **dismissed** (no action taken, rationale required).

Regression check: verify that architectural decisions from prior layers are still intact and that new code does not silently violate established boundaries or contracts.

**Coordination:** Flag any findings that should be surfaced to [QA-REVIEW.md](QA-REVIEW.md), [UX-REVIEW.md](UX-REVIEW.md), [SECURITY-REVIEW.md](SECURITY-REVIEW.md), or [PLATFORM-ENGINEERING-REVIEW.md](PLATFORM-ENGINEERING-REVIEW.md). If this review suggests the need for a new IAR domain, log it as a finding.

## Standard Evaluation Dimensions

1. **Separation of concerns** — Are business logic, rendering, and storage concerns cleanly separated? Do the layer boundaries between `bookmarks.ts` (pure logic) and `main.ts` (DOM wiring) hold consistently?
2. **Coupling and cohesion** — Are modules loosely coupled? Is each module's responsibility focused and internally cohesive?
3. **Data model integrity** — Is the data model well-defined and minimal for the use case? Are invariants enforced at the right boundaries? Are types as precise as needed?
4. **Interface contracts** — Are the APIs between components explicit and correctly typed? Are internal conventions documented or enforced rather than implicit?
5. **State management** — Is application state localized? Are mutations and side effects predictable and contained?
6. **Immutability** — Are data operations consistent in their mutation patterns? Does the code avoid unexpected shared-state side effects?
7. **Extensibility** — Can planned future features be added without restructuring? Does the architecture accommodate the project's stated growth path?
8. **Technology fitness** — Are the chosen technologies appropriate for the stated constraints (no framework, browser localStorage, single user)? Are tradeoffs documented?
9. **Complexity budget** — Is complexity proportional to the problem? Are there unnecessary abstractions, over-engineering, or under-engineering for the stated scope?
10. **Decision documentation** — Are significant architectural decisions recorded in [DECISIONS.md](../DECISIONS.md) or [DESIGN.md](../DESIGN.md) with rationale?

---

## Review 3 — 2026-04-24 16:30Z
**Scope:** Layer 6 (Polish) — all 10 standard dimensions. Changes: `extractDomain` in `bookmarks.ts`, `setFormOpen` helper and inline delete in `main.ts`, `styles.css` rewrite. Regression check covers Layers 1–5 architectural decisions.

### Resolved

#### Bug — Cancel handlers called `renderBookmarks` without restoring focus (flagged from UX/QA)
**File:** `src/main.ts:232` (edit cancel), `src/main.ts:307` (inline delete cancel)
Both cancel paths passed `renderBookmarks` as a bare callback reference. When called, the DOM was rebuilt and focus was lost — an observable side effect of a state mutation (DOM rebuild) that left a secondary invariant (keyboard focus) in an undefined state. The pattern is inconsistent: `setFormOpen(false)` explicitly manages focus as part of the form-close contract; cancel handlers did not follow the same principle.
**Resolution:** Both handlers now call `renderBookmarks()` then explicitly focus the appropriate button via `document.querySelector`. The architectural rule established: any action that destroys and recreates DOM containing the focus target is responsible for restoring focus.

### Dismissed

#### `extractDomain` in `bookmarks.ts` — correct boundary placement
`new URL(url).hostname` is pure logic — no DOM, no side effects, deterministic on input. Placing it in `bookmarks.ts` alongside `validateUrl` (which uses the same `URL` constructor) is correct. The `if (domain)` conditional rendering in `main.ts` is appropriately handled at the DOM wiring layer. Dismissed.

#### `setFormOpen` — correct centralization
`setFormOpen` coordinates three things that must stay in sync: `form.hidden`, `toggle.setAttribute('aria-expanded')`, and focus. Centralizing in a single helper prevents drift between call sites (`handleSubmit`, `DOMContentLoaded` toggle listener). This is a small but correct abstraction for a state that has three observable components. Dismissed.

#### Inline delete — `actions.replaceWith(confirm)` pattern
The confirm div replaces the actions div in-place rather than triggering a full `renderBookmarks()`. This is architecturally preferable: only the targeted item's DOM changes; no other items re-animate; no storage read needed until the user confirms. The cancel path calls `renderBookmarks()` because restoring the original `.bookmark-actions` HTML by hand would be fragile (duplicating rendering logic). The full re-render on cancel is a minor inefficiency but the correct choice for simplicity and correctness. Dismissed.

#### Tag badges as `<button>` — correct element choice
Changed from `<span>` to `<button type="button">`. Native button elements have keyboard accessibility without extra ARIA. No behavioral difference from the architecture's perspective — the click handler toggles `activeTag` and calls `renderBookmarks()`, identical to the filter bar. The element change is a UX improvement, not an architectural concern. Dismissed.

#### `formOpen` module-level state — appropriate
Mirrors `activeTag` and `searchQuery` — module-level `let`, mutated only in `setFormOpen` (and the toggle listener), used by `setFormOpen`. Localized to `main.ts`, not shared across modules. Dismissed.

#### `styles.css` rewrite — no architectural implication
CSS custom properties (`:root` variables) replace hard-coded colors. Pure presentation layer, no behavioral change. Not an architectural concern. Dismissed.

#### Separation of concerns — intact
`bookmarks.ts` contains `extractDomain` (pure) and all prior pure logic. `main.ts` contains all DOM wiring including `setFormOpen` and the inline delete construction. No DOM references in `bookmarks.ts`. No business logic in `main.ts`. Dismissed.

#### Regression check — all prior architectural decisions hold
Storage re-read pattern, module-level state, `form.dataset.id!` assertion, `normalizeBookmark` invariant, `renderTagFilters` side-effect, immutability across all write paths: all unchanged. 95 browser tests pass. Dismissed.

**Tests:** 80 unit | 95 browser | coverage 100% on `src/bookmarks.ts`

---

## Review 1 — 2026-04-24 23:55Z
**Scope:** Full project, Layers 1–5 — all 10 standard dimensions. Initial SA domain review.

### Resolved

#### Bug — `handleSubmit` mutates the bookmarks array; inconsistent with the immutable pattern used everywhere else
**File:** `src/main.ts:293`
```typescript
bookmarks.push(newBookmark);
saveBookmarks(storage, bookmarks);
```
Every other data operation in the codebase returns a new array: `updateBookmark`, `deleteBookmark`, and `sortBookmarks` all use `[...bookmarks]` or `.map`/`.filter`. `handleSubmit` is the only site that mutates a local array, then saves it. No observable bug results — the mutated local is immediately saved and discarded — but the inconsistency is an architectural smell: a reader of the code may model mutation as safe in this module, and a future handler could copy the pattern into a context where mutation does cause a bug.
**Resolution:** Changed to `saveBookmarks(storage, [...bookmarks, newBookmark])`, eliminating the mutation and matching the immutable pattern used throughout.

### Dismissed

#### Separation of concerns — holds cleanly
`src/bookmarks.ts` contains zero DOM references and zero browser globals. `src/main.ts` contains zero business logic — all validation, sorting, filtering, and data transformation are delegated to `bookmarks.ts`. The boundary is strict and consistent across all five layers. Dismissed.

#### `BookmarkStorage` interface — appropriate abstraction
`{ getItem(key): string | null; setItem(key, value): void }` is the minimal interface that production (`localStorage`) and test (mock object) both satisfy. It is not over-engineered — there is no abstract factory or service locator; the interface is structurally satisfied without any extra boilerplate. Dismissed.

#### `Bookmark.note: string` (non-optional) — invariant is correctly enforced
Making `note` a required empty string rather than `note?: string` eliminates null checks throughout the codebase. The invariant is enforced at two boundaries: the add form (note defaults to `''` on submit) and `normalizeBookmark` (coerces missing or wrong-type `note` to `''` on load). No code outside those two sites needs to handle `undefined`. Dismissed.

#### Module-level `activeTag` and `searchQuery` state — appropriate for scope
Module-level `let` variables are the idiomatic state pattern for a no-framework single-page app. Both variables are localized to `main.ts`, mutated only in their respective event handlers and `renderTagFilters`, and reset on page load. There is no shared mutable state across modules. For the current scale and no-framework constraint, a more formal state management pattern would be over-engineering. Dismissed.

#### `renderTagFilters` mutates `activeTag` as a side effect of rendering
`renderTagFilters` checks whether `activeTag` is still valid and resets it to `null` if not. A render function with a state mutation side effect is a mild architectural smell — ideally, state normalization would be a separate step before rendering. However, the fix was introduced deliberately in QA Review 4 to address the ghost-tag bug, is contained to two lines at the top of the function, and is clearly readable. Extracting it would add a call site and reduce locality without improving correctness. Dismissed at current scale.

#### Storage re-read on every interaction — deliberate tradeoff, documented
`loadBookmarks(storage)` is called at the top of `renderBookmarks`, `handleEditClick`, `handleDeleteClick`, and `handleEditSave`. Every interaction reads from `localStorage` rather than from an in-memory cache. This trades a small read cost for simplicity: there is no in-memory state to keep in sync with the stored state, and concurrent multi-tab writes (dismissed in Security Review 1 as out-of-scope) would not cause stale reads. Appropriate for the project's stated constraints. Dismissed.

#### `form.dataset.id!` non-null assertion — internally guaranteed
`handleEditSave` accesses `form.dataset.id!`. The non-null assertion is safe because `handleEditSave` is only attached to forms constructed by `handleEditClick`, which always sets `form.dataset.id = id`. The convention is implicit but local — the handler and the form constructor are adjacent in the same file. At a larger scale, this would warrant a guard. Dismissed at current scope.

#### No pagination, event delegation, or virtualization — correct for scale
All bookmarks render as individual `<li>` elements with individual event listeners. For a personal tool expected to hold tens to low hundreds of bookmarks, the performance impact is negligible. The architecture does not preclude adding event delegation or virtualization later; it simply does not add complexity the current use case does not need. Dismissed.

#### Technology choices — appropriate for constraints
HTML, CSS, TypeScript, Vite, Vitest, Playwright: each choice matches the stated constraints (no framework, browser-native storage, single user, portfolio project). Tradeoffs (no framework means more DOM construction code in `main.ts`; localStorage means no cross-device sync) are documented in DESIGN.md and DECISIONS.md. Dismissed.

**Tests:** 74 unit | 78 browser | coverage 100% on `src/bookmarks.ts`

---

## Review 2 — 2026-04-25 00:30Z
**Scope:** Full application. Triggered by: push→spread immutability fix in `main.ts`; IAR suite reorganized into `air/` subfolder. All 10 standard dimensions evaluated.

### Resolved

*(none)*

### Dismissed

#### push→spread fix — architecture now fully consistent
`saveBookmarks(storage, [...bookmarks, newBookmark])` completes the immutability pattern across all write operations in `main.ts`. `handleSubmit`, `handleEditSave`, and `handleDeleteClick` all now call `saveBookmarks` with a new array. The boundary between `bookmarks.ts` (pure, immutable, side-effect-free) and `main.ts` (DOM wiring, storage orchestration) is intact and consistent. Dismissed.

#### `air/` subfolder reorganization — no architectural implication
Moving 6 documentation files into a subfolder improves navigability but changes nothing in the source, test, or build layers. Module boundaries, interface contracts, and state management are all unchanged. Dismissed.

#### Separation of concerns — intact
`bookmarks.ts`: pure logic, no DOM, no side effects, 100% unit-testable in isolation. `main.ts`: all DOM wiring, event handlers, rendering. No drift from this boundary in the current change set. Dismissed.

#### All Review 1 dismissals still apply
Storage re-read pattern, module-level state, `form.dataset.id!` assertion, `normalizeBookmark` invariant, `renderTagFilters` side-effect, no-pagination decision, technology choices: all unchanged. No new code that touches any of these concerns. Dismissed.

**Tests:** 74 unit | 78 browser | coverage 100% on `src/bookmarks.ts`
