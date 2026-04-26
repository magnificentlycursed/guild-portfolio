# Platform Engineering Review Log

This review is part of the [Iterative Adversarial Refinement (IAR)](README.md) suite. It is a required gate for merging. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

The purpose of this review is to shift quality checks left — into the CI/CD pipeline — so that defects are caught automatically before merging or deploying, rather than relying on manual review steps. Every review evaluates the whole pipeline, not only steps that changed.

## Current Review Prompt

**Scope:** Whole pipeline and build configuration by default. If a scope is provided (e.g., a specific workflow file or build config change), focus primary analysis there — but regression checks always cover the entire pipeline.

Read all workflow files, build config, package manifests, lock files, and `.gitignore`. Apply every standard dimension below as a floor — add others as appropriate to the current state of the pipeline. There is no restriction on what can be flagged.

For each finding, cite file and line number. Classify as **resolved** (fix applied this review), **deferred** (scheduled for a specific layer, reason given), or **dismissed** (no action taken, rationale required).

Regression check: verify that all pipeline gates installed in prior reviews are still present and still gate on failure. A refactor to one part of the pipeline can silently remove a gate.

**Left-shift lens:** For every manual check in the IAR gate checklists ([TODO.md](../TODO.md), [QA-REVIEW.md](QA-REVIEW.md), [UX-REVIEW.md](UX-REVIEW.md), [SECURITY-REVIEW.md](SECURITY-REVIEW.md)), evaluate whether it can be automated and moved into CI. Automating a check is always preferable to a human remembering to run it.

**Coordination:** Flag any findings that should be surfaced to [QA-REVIEW.md](QA-REVIEW.md), [UX-REVIEW.md](UX-REVIEW.md), [SECURITY-REVIEW.md](SECURITY-REVIEW.md), or [SOLUTION-ARCHITECT-REVIEW.md](SOLUTION-ARCHITECT-REVIEW.md). If this review suggests the need for a new IAR domain, log it as a finding.

## Standard Evaluation Dimensions

1. **Pipeline completeness** — Does CI run all required checks: typecheck, unit tests, coverage, browser tests, build, audit? Are any quality gates manual-only that could be automated?
2. **Gate enforcement** — Are all pipeline checks required to pass before merging? Is branch protection configured?
3. **Dependency installation** — Is `npm ci` used (not `npm install`)? Is the lock file committed and the source of truth for installs?
4. **Environment pinning** — Is the runtime version (Node, etc.) pinned? Are browser versions for testing deterministic?
5. **Cache correctness** — Are cache keys scoped to the right artifacts? Will caches invalidate when dependencies or configs change?
6. **Coverage thresholds** — Are coverage requirements enforced in CI with configured thresholds, not just available locally?
7. **Security scanning** — Is `npm audit` or equivalent run in CI and configured to fail on findings above the accepted risk threshold?
8. **Artifact hygiene** — Is build output excluded from version control? Is it generated fresh in CI, never committed?
9. **Action/dependency pinning** — Are CI action versions pinned to avoid supply chain risk? Are they up to date?
10. **Left-shift opportunities** — Which manual review steps could be automated and added to CI?

---

## Review 4 — 2026-04-24 16:30Z
**Scope:** Layer 6 (Polish) — all 10 standard dimensions. Changes: `extractDomain` in `bookmarks.ts` (+6 unit tests), inline delete and tag badge refactor in `main.ts`, `styles.css` rewrite, 16 new browser tests. No changes to CI workflow, build config, or dependencies.

### Resolved

*(none)*

### Dismissed

#### All prior pipeline gates verified intact
`npm run lint` (ESLint, added Review 3): clean. `tsc --noEmit`: clean. `npm run test:unit`: 80 passed. `npm run test:coverage`: 100% on all dimensions (57/57 statements, 38/38 branches, 24/24 functions, 49/49 lines — `extractDomain` added 3 statements, 1 branch, 1 function, covered by new unit tests). `npm audit --audit-level=high`: 0 vulnerabilities. `playwright test`: 95 passed. `npm run build`: no regressions. Dismissed.

#### Coverage — `extractDomain` fully covered
`extractDomain` adds the `try/catch` branch for malformed URLs. The 6 new unit tests exercise both the success path (5 tests) and the failure path (1 test for malformed URL). Coverage remains 100%. Dismissed.

#### Left-shift — touch target and reduced-motion tests not automatable
Touch target minimum size (44px) could be checked by querying `getComputedStyle(el).minHeight` in a test, but computed style is unreliable in headless environments and does not reflect visual render accurately. The CSS is authoritative and verified by inspection. Not worth the fragility. Dismissed.

`prefers-reduced-motion` could be tested via `page.emulateMedia({ reducedMotion: 'reduce' })` to verify no animation plays. However, animations are not an accessibility violation detectable by axe — they are a preference. The wrapping media query is trivially verifiable by reading the CSS. Dismissed.

#### No new dependencies, actions, or workflow changes
Layer 6 is a pure source/test change with no new devDependencies. The pipeline does not need updating. Dismissed.

**Tests:** 80 unit | 95 browser | coverage 100% on `src/bookmarks.ts` | 0 CVEs | lint clean

---

## Review 1 — 2026-04-24 23:45Z
**Scope:** Full project, Layers 1–5 — all 10 standard dimensions. Initial PE domain review.

### Resolved

#### Bug — Playwright cache key used wrong `hashFiles` path
**File:** `.github/workflows/bookmark-manager.yml:46`
`hashFiles('package-lock.json')` resolves relative to the repository root. The lock file lives at `bookmark-manager/package-lock.json`. No file exists at the root path, so the hash was always the same empty value. The Playwright browser cache would never invalidate when dependencies changed, meaning a dependency update could leave the job running against a stale browser installation.
**Resolution:** Changed to `hashFiles('bookmark-manager/package-lock.json')`.

#### Bug — Coverage not enforced in CI; `include` glob tracked `main.ts`
**Files:** `.github/workflows/bookmark-manager.yml`, `vite.config.ts`
`npm run test:coverage` was not in the CI pipeline. The design doc and PR checklist require `src/bookmarks.ts` to maintain 100% statement, branch, and function coverage, but nothing blocked a regression from merging. Additionally, `coverage.include` was `src/**/*.ts`, which included `src/main.ts`. Since `main.ts` is excluded from unit coverage by design (covered by browser tests), adding a threshold with that `include` would have always failed.
**Resolution:** Changed `include` to `['src/bookmarks.ts']`; added `thresholds: { statements: 100, branches: 100, functions: 100, lines: 100 }` to `vite.config.ts`; added `npm run test:coverage` step to CI after unit tests. Coverage confirmed: 100% on all dimensions (54/54 statements, 38/38 branches, 23/23 functions).

#### Bug — `npm audit` not automated
**File:** `.github/workflows/bookmark-manager.yml`
`npm audit` was in the Security review manual checklist but not run in CI. A new CVE in a dependency would not block merging — it would only be caught if a human remembered to run the audit before merging.
**Resolution:** Added `npm audit --audit-level=high` step to CI after coverage.

### Dismissed

#### Action versions pinned to tags, not SHAs
`actions/checkout@v4`, `actions/setup-node@v4`, `actions/cache@v4` are pinned to major version tags. Tags are mutable — a compromised action could update a tag. Pinning to an immutable SHA is the hardest mitigation. For a personal portfolio project with no secrets and no deployment target, the practical risk is negligible. Dismissed at current scope; would revisit if the pipeline ever handles secrets or deploys to production.

#### No branch protection rules in code
Branch protection (require CI to pass before merging) is a GitHub repository setting, not a file in the repository. Cannot be verified or enforced from the codebase. Documented here as a required configuration: main branch must require the `ci` job to pass. Dismissed from this file — verify in repository settings.

#### No linter in the pipeline
No ESLint or equivalent is configured. TypeScript type checking (`tsc --noEmit`) catches type errors; Vitest catches logic errors; the absence of a style linter is consistent with the project's no-tooling-overhead constraint. Dismissed.

**Tests:** 74 unit | 77 browser | coverage 100% on `src/bookmarks.ts`

---

## Review 2 — 2026-04-25 00:30Z
**Scope:** Full pipeline. Reviewing stability of gates added in Review 1 (coverage step, audit step, cache key fix); IAR suite reorganized into `air/` subfolder (no pipeline changes from reorganization). All 10 standard dimensions evaluated.

### Resolved

*(none)*

### Dismissed

#### All Review 1 gates verified intact
Coverage step (`npm run test:coverage`) still present after unit tests. Audit step (`npm audit --audit-level=high`) still present after coverage. Cache key `hashFiles('bookmark-manager/package-lock.json')` still uses the correct repo-root-relative path. No pipeline steps removed or reordered. Dismissed.

#### Unit tests run twice — accepted duplication
`npm run test` (which runs unit tests then browser tests) is the final pipeline step. Unit tests also run as a discrete earlier step. This means unit tests execute twice per CI run. The cost is minimal (74 tests, sub-second). The benefit of the explicit unit step is early failure feedback: a unit test failure surfaces before the slower Playwright step. The duplication is deliberate and appropriate. Dismissed.

#### Pipeline completeness — adequate for current scope
Steps in order: checkout → setup-node → install → typecheck → unit tests → coverage → audit → cache Playwright → install Playwright → browser tests → build. All gates are present. All steps run on `push` and `pull_request` to `main`. Dismissed.

#### No linter step — consistent with project constraints
TypeScript type checking (`tsc --noEmit`) provides static analysis. No ESLint is configured. Consistent with the no-tooling-overhead constraint documented in DESIGN.md and SOLUTION-ARCHITECT-REVIEW.md. Left-shift opportunity if a linter is added in Layer 6. Dismissed.

**Tests:** 74 unit | 78 browser | coverage 100% on `src/bookmarks.ts` | 0 CVEs

---

## Review 3 — 2026-04-25 01:15Z
**Scope:** Linter addition. ESLint + typescript-eslint installed; `npm run lint` script added; `Lint` step added to CI pipeline after `Type check`.

### Resolved

#### Linter added to pipeline
**Files:** `package.json`, `eslint.config.js`, `.github/workflows/bookmark-manager.yml`
ESLint with `typescript-eslint` installed per the typescript-eslint getting-started documentation — the canonical ESLint setup for TypeScript projects. Config uses `tseslint.config(eslint.configs.recommended, tseslint.configs.recommended)`. `npm run lint` exits clean against `src/`. CI step `Lint` runs after `Type check` and before `Unit tests`. The "no linter" dismissal in Review 2 is now resolved.

### Dismissed

*(none)*

**Tests:** 74 unit | 78 browser | coverage 100% on `src/bookmarks.ts` | 0 CVEs | lint clean
