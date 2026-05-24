# Solution Architect Review

This review is part of the [Iterative Adversarial Refinement (IAR)](../../README.md) suite. It may be run independently or alongside other domains. See [README.md](../../README.md) for sequencing, scoped runs, and domain coordination.

**Reviewer role: [Solution Architect](SOLUTION-ARCHITECT-REVIEW.md)** (Solution Architect / Software Architect / Technical Lead)

The purpose of this review is to evaluate whether the architecture — its structure, boundaries, decisions, and tradeoffs — is sound, coherent, and appropriate for the project's stated purpose and constraints. Every review targets the whole application, not only the most recently changed code.

## Current Review Prompt

**Scope:** Whole application by default. If a scope is provided (e.g., a specific feature or set of changed files), focus primary analysis there — but architectural concerns frequently cross boundaries, so adjacent code is always fair game.

Read DESIGN.md first as the authoritative statement of the project's purpose, constraints, and intended structure. Then read all source files and the refinement log. Apply every standard dimension below as a floor — add others as appropriate to the current state of the app. There is no restriction on what can be flagged.

For each finding, cite file and line number. Classify as **resolved** (fix applied this review), **deferred** (scheduled for a specific layer, reason given), **dismissed** (no action taken, rationale required), or **hallucinated** (the adversary invented a problem that does not exist — push back is warranted. Consistent hallucinated findings are the maximum viable refinement signal: real issues have been exhausted).

Regression check: verify that architectural decisions from prior layers are still intact and that new code does not silently violate established boundaries or contracts.

**Coordination:** Flag any findings that should be surfaced to [QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md), [UX-REVIEW.md](UX-REVIEW.md), [SECURITY-REVIEW.md](SECURITY-REVIEW.md), [PLATFORM-ENGINEER-REVIEW.md](PLATFORM-ENGINEER-REVIEW.md), [DATA-ENGINEER-REVIEW.md](DATA-ENGINEER-REVIEW.md) (data model integrity findings from dim 3 — when SA identifies a data model concern at the architectural level, DE applies the deeper data-layer evaluation), or [PRIVACY-REVIEW.md](PRIVACY-REVIEW.md) (dim 27 — data transmitted to external services; cross-reference with [Privacy](PRIVACY-REVIEW.md) dim 6 when Privacy is active). If this review suggests the need for a new IAR domain, log it as a finding.

**DESIGN.md change authority:** If a finding requires a change to `DESIGN.md`, classify it "Raised to SO" and document the proposed change and rationale. Do not apply the change. `DESIGN.md` is a controlled spec document — the [Solution Owner](SOLUTION-OWNER-REVIEW.md) is the sole domain authorized to modify it.

**Sycophancy check:** An agent that designed the architecture will find it sound because it reflects its own training distribution and defaults, not because it is right for this project's constraints. Push hardest on dim 9 (complexity budget) and dim 8 (technology fitness): these are the dimensions where agent defaults most consistently diverge from what a single maintainer or small project actually needs. For each technology choice and architectural pattern, ask: "would this choice have been made by a human engineer working alone on a project of this scope, or is it a team-scale default?"

**Language and interface supplement:** Consult [`../../supplements/`](../../supplements/) for the supplement matching the project's primary language (e.g., [`rust.md`](../../supplements/rust.md), [`javascript-typescript.md`](../../supplements/javascript-typescript.md), [`python.md`](../../supplements/python.md), [`bash.md`](../../supplements/bash.md)). Apply the **Solution Architect** section from the relevant supplement file in addition to the standard dimensions below.


**Validator pair (Review 77):** `solution-owner` is the natural validator for SA findings routed via Raised-to-SO (the spec-change subset). For pure architecture-doctrine findings without an SO routing (e.g., a Dim 12 purity-boundary refactor that touches only implementation), declare `**Validator:** sanity-check` per the meta-validator-of-last-resort pattern (Review 77 Finding 2) — [Sanity Check](../meta/SANITY-CHECK-REVIEW.md) confirms the architecture-doctrine finding's resolution coheres with DESIGN.md's verification architecture and doesn't introduce a spec-drift that should have been Raised-to-SO.
## Standard Evaluation Dimensions

1. **Separation of concerns** — Are business logic, rendering, and storage concerns cleanly separated? Do established layer boundaries hold consistently?
2. **Coupling and cohesion** — Are modules loosely coupled? Is each module's responsibility focused and internally cohesive?
3. **Data model integrity** — Is the data model well-defined and minimal for the use case? Are invariants enforced at the right boundaries? Are types as precise as needed?
4. **Interface contracts** — Are the APIs between components explicit and correctly typed? Are internal conventions documented or enforced rather than implicit?
5. **State management** — Is application state localized? Are mutations and side effects predictable and contained? For browser applications: **event listener and timer lifecycle** — are event listeners removed when the associated DOM is removed or the component is destroyed? Are timers (`setInterval`, `setTimeout`) cleared when no longer needed? Accumulating unreleased listeners and timers is the most common cause of memory leaks in long-running browser sessions. A codebase that adds listeners in a `mount` or `render` function without a corresponding `unmount` or `cleanup` is a finding. This becomes a production failure — not a test failure — because tests typically exercise short sessions that don't accumulate enough leaks to manifest.
6. **Immutability** — Are data operations consistent in their mutation patterns? Does the code avoid unexpected shared-state side effects?
7. **Extensibility** — Can planned future features be added without restructuring? Does the architecture accommodate the project's stated growth path?
8. **Technology fitness** — Are the chosen technologies appropriate for the stated constraints? Are tradeoffs documented?
9. **Complexity budget** — Is complexity proportional to both the problem and the maintenance model? Flag unnecessary abstractions, over-engineering, or under-engineering for the stated scope. An AI agent defaults to team-scale engineering practices regardless of the project's actual maintenance team size — flag complexity that would be justified on a team but creates unnecessary burden for a single maintainer. Note the distinction: over-engineering (SO dimension 4) flags complexity beyond the spec; this dimension flags complexity proportionate to the spec but disproportionate to the team.
10. **Decision documentation** — Are significant architectural decisions recorded with rationale?
11. **Session continuity** — Are the architectural decisions, constraints, and rationale from this project documented in a form that a new AI session can act on without rediscovering them? Decisions that live only in conversation history are invisible to future sessions and to future-you. Flag significant decisions that have no durable record outside the code itself.

12. **VSDD purity boundary map** — Is there an explicit boundary between the pure/deterministic core and the effectful shell? Pure functions (no I/O, no side effects, deterministic output for identical input) are verifiable — they can be unit tested without mocking, property-tested, and in [Phase 5](../../primers/5-formal-hardening.md)+ formally proven. Effectful code (I/O, storage, network, DOM, randomness, time) cannot be formally verified and should form a thin shell around the pure core.

   Evaluate:
   - Are validation, transformation, and business logic functions pure — do they take input and return output with no side effects?
   - Are storage reads/writes, DOM manipulation, fetch calls, and other effects isolated to dedicated effectful functions that call pure functions for logic?
   - Is the boundary respected consistently, or do pure functions occasionally reach out for I/O (reading from storage mid-computation, logging, calling APIs)?
   - Is the boundary documented in DESIGN.md as a verification architecture decision, or only implicit in the code structure?

   This boundary matters immediately (pure functions are trivially unit-testable) and long-term (pure functions are candidates for formal verification in VSDD Phase 5). A codebase where pure logic and effects are entangled throughout is harder to test, harder to maintain, and harder to verify.

---

### Extended: External Interface Contracts

These dimensions apply when the project has external-facing interfaces: published libraries, REST/GraphQL/gRPC APIs, CLI tools used in scripts or pipelines, event-producing or event-consuming services. They may be omitted for purely internal interfaces where producer and consumer are always deployed together by the same team. **Also activate when adding features to an existing system** — dims 16 (backward compatibility) and 17 (contract testing) apply to any change that existing callers, users, or stored data must survive.

13. **Contract documentation** — Is the external interface documented in a machine-readable or testable form? Named formats: OpenAPI/Swagger for REST; `rustdoc` for [Rust](https://www.rust-lang.org/) libraries; TypeDoc/JSDoc for JS/TS packages; a CLI's `--help` output and man page. Documentation that exists only as prose cannot be validated against the implementation automatically. An undocumented API is an unknown contract.

14. **Breaking change definition** — Is there an explicit definition of what constitutes a breaking change? Named breaking changes: removing a response field, changing a field's type, renaming a CLI flag, changing exit code semantics, removing a previously exported function. Named non-breaking: adding an optional field, adding a new flag, adding a new export. No defined policy means callers cannot safely depend on the interface.

15. **Versioning strategy** — How are breaking changes communicated and deployed? For REST APIs: URL versioning or header versioning. For libraries: semantic versioning with a defined major/minor/patch policy. For CLIs: semantic versioning and a documented flag stability tier (stable, experimental, deprecated). The strategy must be documented before the first breaking change.

16. **Backward compatibility** — Can callers written against the previous interface version continue to function after this release? Evaluate: are any previously-present fields removed? Are previously-optional fields now required? Are previously-accepted inputs now rejected?

17. **Contract testing** — Are there tests that verify the contract from the consumer's perspective, not from the producer's implementation? Named patterns: consumer-driven contract tests (Pact), golden file tests for CLI output, integration tests that invoke the API as a caller would. Unit tests on internal functions do not validate the external contract.

18. **Error contract** — Are error responses part of the documented contract? Named checks: error codes stable across versions; error message formats documented; callers can distinguish error categories (user error vs. server error vs. rate limit) from response shape or status code.

19. **Input validation at the boundary** — Is all caller-supplied input validated at the entry point before it reaches business logic? API boundaries are trust boundaries. Validate required fields, types, value ranges, and string constraints. Return actionable error messages: "field 'url' must be a valid http(s) URL" rather than "validation error."

20. **Deprecation process** — When a part of the interface must change, is there a documented deprecation process? Named steps: announce the deprecation with the new equivalent; provide a migration guide; run old and new in parallel for a defined period; remove after the period expires. Callers cannot migrate if they do not know about the deprecation.

21. **API design ergonomics** — From the caller's perspective: is the API predictable and consistent? Named concerns: inconsistent naming conventions across endpoints; operations requiring multiple calls that could be one; required parameters with sensible defaults that are missing; response shapes that force callers to navigate nested structures for basic data.

22. **CLI contract stability** — For CLI tools intended to be scripted or composed with other tools: is the stdout/stderr/exit code contract explicit and stable? Named checks: documented exit codes for each failure mode; structured output (`--json` flag) for machine-readable use; `--help` text that accurately describes the contract; behavior on stdin in a pipeline.

---

### Extended: External Service Integration

These dimensions apply when the project consumes external services at runtime: third-party REST/GraphQL/gRPC APIs, authentication providers, payment processors, email/SMS services, analytics platforms, AI/ML APIs, CDNs, or any service the application calls over the network. They may be omitted for fully self-contained projects with no runtime external dependencies.

23. **External dependency inventory** — What external services does the application call at runtime? Name each: service, purpose, data sent, and whether the call is in the critical path (failure blocks core functionality) or non-critical (failure degrades a secondary feature). An undocumented external dependency is an unreviewed failure mode and an unreviewed data-sharing arrangement. If DESIGN.md does not enumerate external dependencies, that is a spec gap.

24. **Failure and timeout handling** — What happens when an external service is unavailable, slow, or returns an unexpected error? Named failure modes: no timeout configured (the call blocks the event loop or request thread indefinitely); no retry logic for transient failures; raw error from external service surfaced to the user with internal detail; application enters a broken state rather than degrading gracefully. Every external call must have an explicit timeout, an explicit error-handling path, and defined degraded behavior when the service is unreachable.

25. **API contract drift** — What happens if the external API changes its response shape, removes a field, or changes a field's type between releases? Named failure modes: no runtime validation of external API responses (a [TypeScript](https://www.typescriptlang.org/) `as` cast or Rust `serde` deserialization silently ignores unexpected shapes); no monitoring or alerting on unexpected response structures; no process for detecting upstream API changes. Tests pass against a snapshot of the external API but production breaks silently when the provider deploys a change.

26. **Credentials to external services** — Are API keys, client secrets, OAuth tokens, and webhook signing secrets for external services handled as first-class secrets? Named checks: excluded from version control; injected via environment variables or a secrets manager; rotatable without a code change or redeployment; scoped to the minimum permissions required; separate credentials for development and production. A key committed to a repository — even a private one — is the most common single source of credential compromise.

27. **Data transmitted to external services** — What data does the application send to external services, and is that transmission justified and known? Named concerns: user inputs forwarded to an AI API without sanitization or consent; behavioral data sent to analytics platforms; error payloads containing stack traces, user identifiers, or PII sent to crash reporting services; third-party scripts that run in the browser and independently collect data outside the application's control. Cross-reference with [PRIVACY-REVIEW.md](PRIVACY-REVIEW.md) dim 6 (third-party data sharing) when Privacy is active.

---

Findings from this domain serve all three audiences of the methodology (suite developers + suite users + AI agents) — see [`suite-development.md`](../../suite-development/suite-development.md#three-audience-design-principle-review-80-finding-3) § Three-audience design principle ([Review 80](../../suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z) Finding 3; renamed in [Review 84](../../suite-development/review-log/2026-05-21-suite-review.md#review-84--2026-05-21-1100z) Finding 4).

Review entries are logged in per-session files at `vsdd-suite/review-log/YYYY-MM-DD-solution-architect.md` inside the project being reviewed; the project's `vsdd-suite/FINDINGS-INDEX.md` aggregates cross-cutting findings. The optional per-domain index at `vsdd-suite/SOLUTION-ARCHITECT-REVIEW.md` activates when the project opts in via the scaffold script's `--with-per-domain-indexes` flag. See `vsdd-suite/suite-development/suite-development.md` § Governing standard for project-level review logs.
