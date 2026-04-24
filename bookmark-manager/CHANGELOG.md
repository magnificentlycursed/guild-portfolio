# Bookmark Manager — Changelog

## 2026-04-23 — GitHub Actions CI pipeline

### Added
- `.github/workflows/ci.yml` — CI pipeline triggered on every push to any branch and on pull requests to main; steps in order: checkout → Node 20 setup (with npm cache) → `npm ci` → typecheck → unit tests → Playwright browser cache → Playwright browser install → browser tests → build; all steps must pass for the job to succeed
- Playwright browser binaries cached by `package-lock.json` hash to avoid re-downloading on repeated runs; system deps installed separately when cache is hit

### Changed
- `DESIGN.md` — added GitHub Actions to the Technology section

> **Note:** To enforce the CI check as a merge gate, enable branch protection on `main` in GitHub → Settings → Branches → Add rule → require the `ci` status check to pass before merging.

---

## 2026-04-23 — Manual testing checklists and layer gate process

### Changed
- `DESIGN.md` — added Manual Testing paragraph to Testing Methodology; added manual testing to the layer completion gate alongside automated tests and adversarial review
- `TODO.md` — added manual testing requirement to the layer-transition gate in the header; added human-readable testing checklists for all six layers covering happy path, edge cases, validation errors, persistence, and UI state; added QA review status lines between layers (completed reviews link to ADVERSARIAL.md; upcoming layers marked Pending)

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
