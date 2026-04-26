# Solution Owner Review Log

This review is part of the [Iterative Adversarial Refinement (IAR)](README.md) suite. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

The purpose of this review is to guard the project against scope creep and over-engineering. DESIGN.md is a Scope of Work. The SO review holds the implementation to that contract: 100% of what was agreed, nothing that was not.

---

## Review 1 — 2026-04-25

**Scope:** Full application (portfolio retrospective). Reference: `DESIGN.md` as authoritative spec.

### Compliance Table

| Requirement | Status | Notes |
|---|---|---|
| Add bookmark: URL, title, optional note, optional tags | Met | Add form with all four fields |
| Display all bookmarks newest first | Met | `sortBookmarks` with `createdAt` desc + id tiebreaker |
| Click to open in new tab | Met | `<a target="_blank" rel="noopener noreferrer">` |
| Edit: title, URL, note, tags | Met | Inline edit form, all four fields |
| Delete a bookmark | Met | Inline confirmation → delete |
| Filter by tag | Met | Filter bar with "All" + per-tag buttons |
| Search by title or note in real time | Met | Search input with `oninput` handler |
| HTML, CSS, TypeScript (no frameworks) | Met | Plain DOM API, Vite build |
| localStorage | Met | `BookmarkStorage` interface backed by `localStorage` |
| Vite | Met | `vite.config.ts` present |
| Vitest unit tests | Met | `tests/unit/bookmarks.test.ts` |
| Playwright browser tests | Met | `tests/browser/bookmark-manager.spec.ts` |
| GitHub Actions CI | Met | `.github/workflows/bookmark-manager.yml` |
| No backend | Met | Static app, no server |
| Dark color scheme | Met | CSS custom properties, dark background |
| Add form collapses to button | Met | `hidden` attribute + toggle button |
| Search bar above list | Met | `role="search"` wrapper with label |
| Tag filter buttons above list | Met | `#tag-filters` div with "All" + tag buttons |
| Each entry: title, URL domain, note, tags | Met | All four rendered per bookmark |
| Tag filters highlight active | Met | `.filter-btn--active` class on active button |
| Search and tag filters work together | Met | `applyFilters` applies both |
| Persist across refreshes | Met | localStorage |
| 360px minimum width | Met | Responsive CSS |
| URL validation: http/https only, no protocol-only | Met | `validateUrl` via `URL` constructor + protocol check |
| Title validation: no empty/whitespace | Met | `validateTitle` trims before checking |
| Form state preserved on validation failure | Met | No form reset on error; early return |
| Deterministic order for identical timestamps | Met | `id.localeCompare` tiebreaker in `sortBookmarks` |
| Three axe scans in browser suite | Met | DESIGN.md Testing Methodology calls this out explicitly |

### Resolved

*(none)*

### Backlogged

*(none)*

### Dismissed

#### ESLint not explicitly in DESIGN.md — approved addition
`package.json` includes `eslint` and `typescript-eslint`. DESIGN.md does not name ESLint in the Technology section. This was introduced following a Platform Engineering left-shift recommendation (PE Review 2) and documented in DECISIONS.md.

ESLint is a development-time static analysis tool with no runtime impact, no behavioral change, and no scope expansion. It enforces a subset of what TypeScript compilation already checks, plus style conventions. Its addition is consistent with the design's intent ("TypeScript") and makes the CI pipeline more thorough without changing the product.

**Classification:** Dismissed. A linter is a tooling addition, not a feature addition. The PE domain explicitly exists to recommend these shifts. PE-recommended tooling that does not change the product is within the spirit of the spec even if not named in it.

#### `@vitest/ui` devDependency
`@vitest/ui` is present in `devDependencies`. This is a development convenience for running tests with a browser UI. No runtime impact, no behavioral change, no user-facing feature. DESIGN.md says "Vitest for unit tests" — the UI companion tool is a reasonable accompaniment.

**Classification:** Dismissed.

#### `normalizeBookmark` storage validation depth
DESIGN.md says data is stored as JSON in localStorage. `normalizeBookmark` validates every field, coerces missing fields to safe defaults, and filters non-string tags. This is more thorough than the spec requires, but it defends against a real risk (tampered or migrated storage) with no visible behavior change for valid data.

**Classification:** Dismissed. Defensive input validation at a storage boundary is not scope creep — it is correct implementation of "Data stored in the browser's local storage."

#### Assignment compliance (dim 9)
The upstream assignment brief (from `apprentice-onboarding/04-proving-it/03-the-portfolio-review.md`) describes the Bookmark Manager as "Phase 1 - Your first build. Design doc, code, process documentation." No prescriptive specification of features, technology, or constraints is given. DESIGN.md is a student-authored design document that defines its own scope. There is no external assignment spec to deviate from.

DESIGN.md's scope is internally consistent: a personal tool for a defined set of features, specific technology choices with rationale, and explicit constraints. No signs of design-stage scope creep that was not self-imposed.

**Classification:** Dismissed.

### Hallucinated

*(none)*

**Summary:** All 27 spec requirements are met. Two tooling additions (ESLint, `@vitest/ui`) are outside the literal text of DESIGN.md but are justified and non-behavioral. No scope deviations in features, behavior, UI elements, or user-facing interactions. The implementation is a clean match to the contract. MVR signal: SO is unlikely to produce findings in a second pass.
