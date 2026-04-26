# Iterative Adversarial Refinement (IAR)

IAR is the adversarial review mechanism of Verification-Driven Development (VDD). VDD builds software through a loop: design → build → verify → adversarial refinement → fix → repeat until maximum viable refinement (MVR). IAR is not a pre-merge checkpoint. It is an active part of the build cycle.

## Domains

| Domain | Log file | Focus |
|---|---|---|
| Quality Engineering | [QA-REVIEW.md](QA-REVIEW.md) | Correctness, test coverage, falsifiability, logic errors, dead code, dependencies, accessibility, browser compatibility, responsive design, security surface, regression coverage |
| Software Engineering | [SOFTWARE-ENGINEERING-REVIEW.md](SOFTWARE-ENGINEERING-REVIEW.md) | Implementation correctness, naming, error handling, type safety, duplication, complexity, maintainability |
| Solution Owner | [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) | Spec compliance, scope creep, over-engineering, under-delivery, technology choices, assignment compliance |
| Solution Architect | [SOLUTION-ARCHITECT-REVIEW.md](SOLUTION-ARCHITECT-REVIEW.md) | Separation of concerns, coupling and cohesion, data model integrity, interface contracts, state management, immutability, extensibility, technology fitness, complexity budget, decision documentation |
| Platform Engineering | [PLATFORM-ENGINEERING-REVIEW.md](PLATFORM-ENGINEERING-REVIEW.md) | Pipeline completeness, gate enforcement, dependency installation, environment pinning, cache correctness, coverage thresholds, security scanning, artifact hygiene, action pinning, left-shift opportunities |
| Security | [SECURITY-REVIEW.md](SECURITY-REVIEW.md) | Rendering safety, URL injection, storage validation, dependency CVEs, CSP, information exposure, input handling, secret handling |
| UX | [UX-REVIEW.md](UX-REVIEW.md) | Empty states, error messages, focus and keyboard behavior, visual consistency, affordances, feedback patterns, accessibility, responsive design, browser compatibility, long content, reduced motion, native dialog quality |
| VDD-IAR Alignment | [VDD-IAR-ALIGNMENT-REVIEW.md](VDD-IAR-ALIGNMENT-REVIEW.md) | Design-before-code, layered decomposition, layer gate compliance, test discipline, human verification, IAR iteration, role integrity, retrospective quality |

Each domain maintains a log file with a current prompt, standard dimensions, and all past review entries.

This project is a **browser application built in TypeScript**. All domain reviews should consult the `lang/browser-app.md` and `lang/javascript-typescript.md` supplements in the portfolio IAR suite (`iterative-adversarial-refinement/lang/`) as applicable to their domain.

## Refinement loop

IAR runs in rounds within a layer until MVR:

1. First pass — all domains, fresh context
2. Fix findings
3. Second pass — fresh context, all domains or scoped to changed areas
4. Repeat until the adversary produces only hallucinated findings
5. **Merge**

A single IAR pass that produced real findings, followed immediately by a merge, is a process failure. Round numbers are required in log entries.

## Session isolation

Each domain run uses a fresh AI context. A review done in the same session as the implementation it evaluates is weaker — the builder and the adversary share the same blind spots. When a review is done in-session, note it in the log entry.

## Scoped runs

Provide a scope to focus primary analysis on a specific feature, layer, or set of changed files. Regression checks always cover the whole application regardless of scope.

Example scopes:
- `"Layer 5 Search — src/main.ts search wiring, index.html search bar"`
- `"handleDeleteClick in src/main.ts"`
- `"All files changed since last IAR run"`

## Sequencing

Default: run all domains in parallel. Sequence when one domain's output informs another:

- Run SO first when there is any risk of scope drift — SA, QE, PE can only optimize within a spec that SO has confirmed is correct
- Run SA first when there are significant structural or architectural changes — SA findings can change what QE, Security need to evaluate
- Run PE first when there are significant pipeline or build config changes — other domains depend on the pipeline running correctly
- Run Security before QE when there are significant changes to storage, rendering, or input handling
- Run QE before UX when QE finds bugs that change the implementation
- Run VDD-IAR Alignment last — it evaluates the process that produced all other reviews
- Re-run any domain that received a cross-domain flag after the flagged domain resolves its findings

## Merging gate

Before a layer may be merged:

1. All IAR domains have run at least one full pass on this layer
2. The refinement loop continued until MVR — findings were fixed and re-reviewed
3. Every finding is **resolved** (fix applied and verified), **dismissed** (rationale documented), or **accepted risk** (explicit rationale required)
4. VDD-IAR Alignment has run as the final gate step
5. Round numbers and session context are logged in respective domain files

No domain may be skipped. A domain with zero findings is a valid outcome — log it with `**Scope:**` so the record is complete.
