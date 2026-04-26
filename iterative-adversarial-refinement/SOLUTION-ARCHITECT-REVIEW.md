# Solution Architect Review

This review is part of the [Iterative Adversarial Refinement (IAR)](README.md) suite. It may be run independently or alongside other domains. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

The purpose of this review is to evaluate whether the architecture — its structure, boundaries, decisions, and tradeoffs — is sound, coherent, and appropriate for the project's stated purpose and constraints. Every review targets the whole application, not only the most recently changed code.

## Current Review Prompt

**Scope:** Whole application by default. If a scope is provided (e.g., a specific feature or set of changed files), focus primary analysis there — but architectural concerns frequently cross boundaries, so adjacent code is always fair game.

Read DESIGN.md first as the authoritative statement of the project's purpose, constraints, and intended structure. Then read all source files and the refinement log. Apply every standard dimension below as a floor — add others as appropriate to the current state of the app. There is no restriction on what can be flagged.

For each finding, cite file and line number. Classify as **resolved** (fix applied this review), **deferred** (scheduled for a specific layer, reason given), **dismissed** (no action taken, rationale required), or **hallucinated** (the adversary invented a problem that does not exist — push back is warranted. Consistent hallucinated findings are the maximum viable refinement signal: real issues have been exhausted).

Regression check: verify that architectural decisions from prior layers are still intact and that new code does not silently violate established boundaries or contracts.

**Coordination:** Flag any findings that should be surfaced to [QUALITY-ENGINEERING-REVIEW.md](QUALITY-ENGINEERING-REVIEW.md), [UX-REVIEW.md](UX-REVIEW.md), [SECURITY-REVIEW.md](SECURITY-REVIEW.md), or [PLATFORM-ENGINEERING-REVIEW.md](PLATFORM-ENGINEERING-REVIEW.md). If this review suggests the need for a new IAR domain, log it as a finding.

**Sycophancy check:** If the agent agreed with every decision reviewed in this domain without challenge, treat that as a finding. An AI agent that validates every choice it helped produce is not providing adversarial review — it is confirming its own work. Flag any area where a significant decision went unquestioned but warranted scrutiny.

**Language and interface supplement:** Consult `lang/` for the supplement matching the project's primary language (e.g., `rust.md`, `javascript-typescript.md`). Apply the **Solution Architect** section from the relevant supplement file in addition to the standard dimensions below.

## Standard Evaluation Dimensions

1. **Separation of concerns** — Are business logic, rendering, and storage concerns cleanly separated? Do established layer boundaries hold consistently?
2. **Coupling and cohesion** — Are modules loosely coupled? Is each module's responsibility focused and internally cohesive?
3. **Data model integrity** — Is the data model well-defined and minimal for the use case? Are invariants enforced at the right boundaries? Are types as precise as needed?
4. **Interface contracts** — Are the APIs between components explicit and correctly typed? Are internal conventions documented or enforced rather than implicit?
5. **State management** — Is application state localized? Are mutations and side effects predictable and contained?
6. **Immutability** — Are data operations consistent in their mutation patterns? Does the code avoid unexpected shared-state side effects?
7. **Extensibility** — Can planned future features be added without restructuring? Does the architecture accommodate the project's stated growth path?
8. **Technology fitness** — Are the chosen technologies appropriate for the stated constraints? Are tradeoffs documented?
9. **Complexity budget** — Is complexity proportional to both the problem and the maintenance model? Flag unnecessary abstractions, over-engineering, or under-engineering for the stated scope. An AI agent defaults to team-scale engineering practices regardless of the project's actual maintenance team size — flag complexity that would be justified on a team but creates unnecessary burden for a single maintainer. Note the distinction: over-engineering (SO dimension 4) flags complexity beyond the spec; this dimension flags complexity proportionate to the spec but disproportionate to the team.
10. **Decision documentation** — Are significant architectural decisions recorded with rationale?
11. **Session continuity** — Are the architectural decisions, constraints, and rationale from this project documented in a form that a new AI session can act on without rediscovering them? Decisions that live only in conversation history are invisible to future sessions and to future-you. Flag significant decisions that have no durable record outside the code itself.

---

Review entries are logged in `iterative-adversarial-refinement/SOLUTION-ARCHITECT-REVIEW.md` inside the project being reviewed.
