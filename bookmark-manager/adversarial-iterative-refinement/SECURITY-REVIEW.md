# Security Review Log

This review is part of the [Adversarial Iterative Refinement (AIR)](README.md) suite. It is a required gate for merging. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

The purpose of this review is to apply iterative adversarial pressure to find, document, and resolve security vulnerabilities, unsafe patterns, validation gaps, and regressions. Every review targets the whole application — not only the most recently changed code.

## Current Review Prompt

**Scope:** Whole application by default. If a scope is provided (e.g., a specific feature or set of changed files), focus primary analysis there — but regression checks always cover the entire application.

Read all source files, test files, HTML, CSS, and config. Apply every standard dimension below as a floor — add others as appropriate to the current state of the app. There is no restriction on what can be flagged.

For each finding, cite file and line number. Classify as **resolved** (fix applied this review), **accepted risk** (no fix, explicit rationale required), or **dismissed** (no action taken, rationale required).

Regression check: verify that all previously-addressed security controls remain intact. Prior layers' security findings are always in scope. A change to validation, rendering, or storage handling can silently remove a control.

**Coordination:** Flag any findings that should be surfaced to [QA-REVIEW.md](QA-REVIEW.md), [UX-REVIEW.md](UX-REVIEW.md), [PLATFORM-ENGINEERING-REVIEW.md](PLATFORM-ENGINEERING-REVIEW.md), or [SOLUTION-ARCHITECT-REVIEW.md](SOLUTION-ARCHITECT-REVIEW.md). If this review suggests the need for a new AIR domain, log it as a finding.

## Standard Evaluation Dimensions

1. **Rendering safety** — Is user-supplied content set via `.textContent` (safe) or `.innerHTML` (potentially unsafe)? Any `innerHTML` usage with user-controlled data is a finding.
2. **URL injection** — Can a user save a URL that executes code when clicked? Verify `javascript:`, `data:`, `vbscript:`, and other non-http(s) protocols are rejected by `validateUrl`.
3. **Storage data validation** — Is data loaded from localStorage validated before use? TypeScript `as` casts are compile-time only; they provide no runtime guarantee against malformed or tampered data.
4. **Dependency security** — Are there known CVEs in direct or transitive dependencies? Check with `npm audit`.
5. **Content Security Policy** — Is a CSP present (meta tag or response header)? Evaluate risk surface without one.
6. **Information exposure** — Do error messages, comments, or headers reveal internal structure, stack traces, or sensitive paths?
7. **Input handling** — Are all user inputs treated as untrusted before being stored or rendered?

---

## Review 1 — 2026-04-24 23:00Z
**Scope:** Full project, Layers 1–5 — all 7 standard dimensions.

### Resolved

#### Finding — `javascript:` and `data:` URLs not explicitly tested
**File:** `tests/unit/bookmarks.test.ts`
`validateUrl` uses the `URL` constructor and protocol check, so both inputs are rejected — but the unit test suite had no dedicated tests for them. An inadvertent change to the validation logic (e.g., switching to a blocklist with a typo) would not be caught.
**Resolution:** Added `'returns an error message for a javascript: URL'` and `'returns an error message for a data: URL'` to the `validateUrl` suite.

#### Finding — localStorage data not validated at runtime
**File:** `src/bookmarks.ts` — `loadBookmarks`
`JSON.parse(data) as Bookmark[]` is a TypeScript type assertion with no runtime effect. Malformed, tampered, or schema-migrated data would cause runtime errors (e.g., `bookmark.tags.includes` on a null field).
**Resolution:** Updated `loadBookmarks` to verify the parsed value is an array and map each element through `normalizeBookmark`, a private function that: discards entries missing required string fields (`id`, `url`, `title`); coerces `note` to `''`, `tags` to `[]` (filtering non-string elements), and `createdAt` to `0` when absent or wrong type. Added 6 unit tests covering the normalization cases.

#### Finding — Confirm dialog did not identify the target bookmark
**File:** `src/main.ts` — `handleDeleteClick`
`window.confirm('Delete this bookmark?')` gave no indication of which bookmark was targeted. A user who misclicked had no way to catch it before confirming a destructive action.
**Resolution:** Updated to load the bookmark before showing the dialog and display `Delete "${bookmark.title}"?`.

### Accepted Risk

#### No Content Security Policy
The app is a personal single-user tool served as a static local file. It has no external network requests, no CDN-loaded scripts, no user authentication, and no sensitive data beyond URLs, titles, notes, and tags. The attack surface for CSP-relevant threats (XSS via injected scripts, data exfiltration) is negligible given the deployment context. Would revisit if the app were ever served over the network to multiple users.

#### Floating dependency versions
All dependencies use caret (`^`) semver. `package-lock.json` pins exact versions for local installs. No known CVEs per `npm audit`. Accepted risk — single-developer portfolio project.

### Dismissed

#### Rendering uses `.textContent` throughout — no XSS surface
**File:** `src/main.ts`
All user-supplied content (`bookmark.title`, `bookmark.note`, `bookmark.url`, tag strings) is set exclusively via `.textContent` or element `.value`. No `innerHTML` usage with user data exists anywhere in the codebase. `.textContent` automatically escapes HTML entities, making XSS via stored data structurally impossible. Documented so any future contributor who introduces `innerHTML` can be evaluated against this baseline.

#### Multi-tab localStorage race condition
Personal single-user tool. Concurrent multi-tab usage is not a supported scenario. No session state, no authentication, no server. Dismissed.

#### localStorage data readable by same-origin scripts
The app is a static local file. There are no other scripts on the same origin. Dismissed.

**Tests:** 74 unit | 77 browser

---

## Review 2 — 2026-04-25 00:30Z
**Scope:** Full application. Triggered by: push→spread immutability fix in `main.ts`; `npm audit` step added to CI; AIR suite reorganized into `air/` subfolder. All 7 standard dimensions evaluated.

### Resolved

*(none)*

### Accepted Risk

*(carry-forward from Review 1 — no change)*

#### No Content Security Policy
Personal single-user tool, static local file, no external network requests. Attack surface for CSP-relevant threats remains negligible. Would revisit if served over the network to multiple users.

#### Floating dependency versions
Caret semver with pinned `package-lock.json`. 0 CVEs per `npm audit`. Accepted at current scope.

### Dismissed

#### Rendering safety — no change
All user-supplied content (`bookmark.title`, `bookmark.note`, `bookmark.url`, tags) still set exclusively via `.textContent` or element `.value`. No `innerHTML` usage with user data. XSS via stored data remains structurally impossible. Dismissed.

#### push→spread fix — no security implication
`saveBookmarks(storage, [...bookmarks, newBookmark])` vs `bookmarks.push(newBookmark)` before save: no change to what data is validated, sanitized, or stored. `normalizeBookmark` still validates on load. Dismissed.

#### `npm audit` now automated in CI
`npm audit --audit-level=high` exits 0. 0 high or critical CVEs. The manual checklist item for running `npm audit` is now a CI gate — a new CVE at high severity will block merging. Dismissed.

#### `normalizeBookmark` storage validation — intact
On load, `normalizeBookmark` coerces `id`, `url`, `title`, `note`, `tags`, `createdAt` to expected types. Tampered or malformed localStorage data cannot cause undefined property access. Unchanged. Dismissed.

**Tests:** 74 unit | 78 browser | 0 CVEs
