# Platform Engineering Review

This review is part of the [Adversarial Iterative Refinement (AIR)](README.md) suite. It is a required gate for merging. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

The purpose of this review is to shift quality checks left — into the CI/CD pipeline — so that defects are caught automatically before merging or deploying, rather than relying on manual review steps. Every review evaluates the whole pipeline, not only steps that changed.

## Current Review Prompt

**Scope:** Whole pipeline and build configuration by default. If a scope is provided (e.g., a specific workflow file or build config change), focus primary analysis there — but regression checks always cover the entire pipeline.

Read all workflow files, build config, package manifests, lock files, and `.gitignore`. Apply every standard dimension below as a floor — add others as appropriate to the current state of the pipeline. There is no restriction on what can be flagged.

For each finding, cite file and line number. Classify as **resolved** (fix applied this review), **deferred** (scheduled for a specific layer, reason given), or **dismissed** (no action taken, rationale required).

Regression check: verify that all pipeline gates installed in prior reviews are still present and still gate on failure. A refactor to one part of the pipeline can silently remove a gate.

**Left-shift lens:** For every manual check in the project's AIR gate checklists, evaluate whether it can be automated and moved into CI. Automating a check is always preferable to a human remembering to run it.

**Coordination:** Flag any findings that should be surfaced to [QA-REVIEW.md](QA-REVIEW.md), [UX-REVIEW.md](UX-REVIEW.md), [SECURITY-REVIEW.md](SECURITY-REVIEW.md), or [SOLUTION-ARCHITECT-REVIEW.md](SOLUTION-ARCHITECT-REVIEW.md). If this review suggests the need for a new AIR domain, log it as a finding.

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

Review entries are logged in `adversarial-iterative-refinement/PLATFORM-ENGINEERING-REVIEW.md` inside the project being reviewed.
