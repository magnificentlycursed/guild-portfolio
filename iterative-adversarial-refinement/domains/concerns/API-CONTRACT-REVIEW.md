# API Contract Review

This review is part of the [Iterative Adversarial Refinement (IAR)](../../README.md) suite. It may be run independently or alongside other domains. See [README.md](../../README.md) for sequencing, scoped runs, and domain coordination.

The purpose of this review is to evaluate the contract between this system and its callers — whether those callers are external consumers of a REST API, users of a library, callers of a CLI tool, or consumers of an event stream. SA reviews internal interface contracts within the codebase. This domain reviews external interface contracts: stability guarantees, versioning, breaking change management, and the caller's ability to rely on the interface.

This domain applies to: published libraries and packages, REST/GraphQL/gRPC APIs, CLI tools used in scripts or pipelines, event-producing or event-consuming services, and any interface where the contract between producer and consumer is maintained across independent release cycles. It may be omitted for purely internal interfaces where producer and consumer are always deployed together and owned by the same team.

## Current Review Prompt

**Scope:** All externally-facing interfaces — public API surfaces, published package exports, CLI argument/output contracts, webhook payloads, event schemas.

Read DESIGN.md for stated API design decisions and compatibility guarantees. Then read all source files with focus on public interfaces, request/response shapes, and CLI argument definitions.

For each finding, cite file and line number. Classify as **resolved** (fix applied this review), **deferred** (scheduled for a specific layer, reason given), **dismissed** (no action taken, rationale required), **deprecation scheduled** (breaking change planned with a documented timeline and migration path), or **hallucinated** (the adversary invented a problem that does not exist — push back is warranted. Consistent hallucinated findings are the maximum viable refinement signal).

**Coordination:** Flag findings that overlap with SE (correctness of the contract implementation), SA (architectural decisions that constrain the contract's evolvability), QE (contract testing coverage), PE (API versioning in CI/CD), and Security (input validation at the API boundary).

**Sycophancy check:** An agent generates APIs that work for the current implementation. It does not design APIs for stability over time, for the caller's mental model, or for the cost of breaking changes. Push hardest on versioning strategy and breaking change definition — these are the areas most likely to be omitted entirely from AI-generated designs.

**Language and interface supplement:** Consult `../../lang/` for language-specific API documentation tooling (e.g., `rustdoc` for Rust, JSDoc/TypeDoc for JS/TS) and contract testing frameworks (Pact, consumer-driven contract tests).

## Standard Evaluation Dimensions

1. **Contract documentation** — Is the external interface documented in a machine-readable or testable form? Named formats: OpenAPI/Swagger for REST, `rustdoc` for Rust libraries, TypeDoc/JSDoc for JS/TS packages, a CLI's `--help` output and man page. Documentation that exists only as prose in a README is not a contract — it cannot be validated against the implementation automatically. An undocumented API is an unknown contract.

2. **Breaking change definition** — Is there an explicit definition of what constitutes a breaking change for this interface? Named breaking changes: removing a field from a response, changing a field's type, renaming a command-line flag, changing exit code semantics, removing a previously exported function. Named non-breaking changes: adding an optional field to a response, adding a new command-line flag, adding a new exported function. The absence of a defined breaking change policy means callers cannot safely depend on the interface.

3. **Versioning strategy** — How are breaking changes communicated and deployed? For REST APIs: URL versioning (`/v1/`, `/v2/`), header versioning, or semantic versioning with a compatibility matrix. For libraries: semantic versioning with a defined major/minor/patch policy. For CLIs: semantic versioning and a documented flag stability tier (stable, experimental, deprecated). The strategy must be documented before the first breaking change — retrofitting a versioning strategy after callers depend on an unversioned interface is expensive.

4. **Backward compatibility** — Can callers written against the previous version of the interface continue to function after this release? Evaluate: are any previously-present fields removed? Are previously-present fields now required that were optional? Are any previously-accepted inputs now rejected? For CLIs: do previously-working invocations still produce the same output and exit codes?

5. **Contract testing** — Are there tests that verify the contract from the consumer's perspective, not just from the producer's implementation? Named patterns: consumer-driven contract tests (Pact), golden file tests for CLI output, integration tests that invoke the API as a caller would. Unit tests on internal functions do not validate the external contract — they validate the implementation. A change that preserves all unit tests while breaking the external contract is a regression not caught by unit tests.

6. **Error contract** — Are error responses part of the documented contract? Named checks: are error codes stable across versions? Are error message formats documented (or are they deliberately undocumented because callers should not parse them)? Can callers distinguish error categories (user error vs. server error vs. rate limit) from the response shape or status code? An API whose error format changes without a version bump has broken its contract.

7. **Input validation at the boundary** — Is all caller-supplied input validated at the entry point of the API, before it reaches business logic? API boundaries are trust boundaries — no assumption about caller behavior is safe. Validate: required fields are present, types match the contract, values are within allowed ranges, strings meet length and character constraints. Return actionable error messages for validation failures: "field 'url' must be a valid http(s) URL" is more useful than "validation error."

8. **Deprecation process** — When a part of the interface needs to change in a breaking way, is there a documented deprecation process? Named steps: announce the deprecation with the new equivalent; provide a migration guide; run old and new versions in parallel for a defined period; remove the deprecated surface after the period. Callers cannot migrate if they do not know about the deprecation.

9. **API design ergonomics** — From the caller's perspective: is the API predictable and consistent? Named concerns: inconsistent naming conventions (some endpoints use camelCase, some use snake_case); operations that require callers to make multiple calls that could be a single call; required parameters that could have sensible defaults; response shapes that force callers to navigate nested structures to reach the data they need. An AI agent will design an API that is convenient to implement; the adversary asks whether it is convenient to use.

10. **CLI contract stability** — For CLI tools intended to be scripted or composed with other tools: is the stdout/stderr/exit code contract explicit and stable? Named checks: documented exit codes for each failure mode; structured output (`--json` flag) for machine-readable use; `--help` text that accurately describes the contract; behavior on stdin when the tool is called in a pipeline. A CLI whose output format changes in patch releases cannot be safely scripted.

---

Review entries are logged in `iterative-adversarial-refinement/API-CONTRACT-REVIEW.md` inside the project being reviewed.
