# Iterative Adversarial Refinement (IAR)

IAR is the adversarial review mechanism of Verification-Driven Development (VDD). VDD builds software through a loop: design → build → verify → adversarial refinement → fix → repeat until maximum viable refinement (MVR). IAR is not a pre-merge checkpoint. It is an active part of the build cycle.

## Domains

| Domain | Log file | Focus |
|---|---|---|
| Solution Owner | [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) | Spec compliance, scope creep, over-engineering, under-delivery, technology choices, assignment compliance |
| Solution Architect | [SOLUTION-ARCHITECT-REVIEW.md](SOLUTION-ARCHITECT-REVIEW.md) | Separation of concerns, coupling and cohesion, data model integrity, interface contracts, state management, complexity budget, decision documentation |
| Quality Engineer | [QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md) | Test correctness, Red Gate compliance, acceptance criteria coverage, integration tests, error path coverage |
| Software Engineer | [SOFTWARE-ENGINEER-REVIEW.md](SOFTWARE-ENGINEER-REVIEW.md) | Implementation correctness, naming, error handling, type safety, complexity, maintainability |
| Security | [SECURITY-REVIEW.md](SECURITY-REVIEW.md) | Input validation, file path handling, I/O error surfaces, dependency CVEs, information exposure |
| VDD-IAR Alignment | [VDD-IAR-ALIGNMENT-REVIEW.md](VDD-IAR-ALIGNMENT-REVIEW.md) | Design-before-code, layered decomposition, layer gate compliance, test discipline, human verification, IAR iteration |

## Project context

This is a **CLI tool built in Rust**. All domain reviews should consult the `lang/rust.md` and `lang/cli.md` supplements in the portfolio IAR suite (`iterative-adversarial-refinement/lang/`) as applicable to their domain.

Assignment: `apprentice-onboarding/02-the-methodology/02-tracking-your-work.md`

## Refinement loop

IAR runs in rounds within a layer until MVR:

1. First pass — active domains, fresh context
2. Fix findings
3. Second pass — fresh context, scoped to changed areas
4. Repeat until the adversary produces only hallucinated findings
5. **Merge**

A single IAR pass that produced real findings, followed immediately by a merge, is a process failure. Round numbers are required in log entries.

## Session isolation

Each domain run uses a fresh AI context. A review done in the same session as the implementation it evaluates is weaker — the builder and the adversary share the same blind spots. When a review is done in-session, note it in the log entry.

## Sequencing

Default: run all domains in parallel. Sequence when one domain's output informs another:

- Run SO first — SA, QE, Security can only optimize within a spec that SO has confirmed matches the assignment
- Run SA before QE when spec-level complexity findings may change the implementation shape
- Run Security before QE when there are significant changes to file I/O or input handling
- Run VDD-IAR Alignment last — it evaluates the process that produced all other reviews

## Merging gate

Before a layer may be merged:

1. All active IAR domains have run at least one full pass on this layer
2. The refinement loop continued until MVR
3. Every finding is **resolved**, **dismissed**, or **deferred** (with a specific future layer named)
4. VDD-IAR Alignment has run as the final gate step
5. Round numbers and session context are logged in respective domain files
