# Data Engineering Review

This review is part of the [Iterative Adversarial Refinement (IAR)](../../README.md) suite. It may be run independently or alongside other domains. See [README.md](../../README.md) for sequencing, scoped runs, and domain coordination.

The purpose of this review is to evaluate the data layer: how data is modeled, validated, stored, accessed, transformed, and evolved. Data bugs are often silent — a schema mismatch or missing validation does not throw an error until much later, often in production. This review applies adversarial pressure to the data model and the code that touches it.

This domain is most relevant to projects with a meaningful data layer: persistent storage, data pipelines, API contracts, schema migrations, or non-trivial data transformation. It may be omitted for projects where data concerns are minimal and fully covered by other domains.

## Current Review Prompt

**Scope:** Whole application by default. If a scope is provided (e.g., a specific data model, migration, or storage layer), focus primary analysis there — but all data-touching code is always in scope.

Read DESIGN.md first for context on the project's intended data model, storage strategy, and constraints. Then read all source files, schema definitions, migration files, and config. Apply every standard dimension below as a floor — add others as appropriate to the current state of the project. There is no restriction on what can be flagged.

For each finding, cite file and line number. Classify as **resolved** (fix applied this review), **deferred** (scheduled for a specific layer, reason given), **dismissed** (no action taken, rationale required), or **hallucinated** (the adversary invented a problem that does not exist — push back is warranted. Consistent hallucinated findings are the maximum viable refinement signal: real issues have been exhausted).

Regression check: verify that data written by prior versions of the application can still be read correctly. A schema change that silently discards or corrupts existing data is a regression even if all tests pass.

**Coordination:** Flag any findings that should be surfaced to [QUALITY-ENGINEERING-REVIEW.md](QUALITY-ENGINEERING-REVIEW.md), [SECURITY-REVIEW.md](SECURITY-REVIEW.md), [SOLUTION-ARCHITECT-REVIEW.md](SOLUTION-ARCHITECT-REVIEW.md), or [SOFTWARE-ENGINEERING-REVIEW.md](SOFTWARE-ENGINEERING-REVIEW.md). If this review suggests the need for a new IAR domain, log it as a finding.

**Sycophancy check:** If the agent agreed with every decision reviewed in this domain without challenge, treat that as a finding. An AI agent that validates every choice it helped produce is not providing adversarial review — it is confirming its own work. Flag any area where a significant decision went unquestioned but warranted scrutiny.

**Language and interface supplement:** Consult `../../lang/` for the supplement matching the project's primary language (e.g., `rust.md`, `javascript-typescript.md`). Apply the **Data Engineering** section from the relevant supplement file in addition to the standard dimensions below — language supplements cover serialization libraries, schema evolution patterns, and validation idioms specific to the ecosystem.

## Standard Evaluation Dimensions

1. **Data model correctness** — Does the data model accurately represent the domain described in DESIGN.md? Are types precise? Are optional vs. required fields correctly distinguished? Are there missing fields or fields with the wrong type?
2. **Validation and normalization** — Is data validated at every boundary where it enters the system (user input, storage reads, API responses)? Are type assertions (`as`, casting) backed by runtime validation? Are coercions to safe defaults applied consistently?
3. **Schema evolution** — If the data model changes, can data written under the old schema still be read? A migration strategy requires more than a normalization function:
   - Is the migration explicitly documented and triggered intentionally, not silently applied at read time?
   - Has the migration been tested against samples of real stored data (or a representative synthetic dataset), not only against newly-created test fixtures?
   - Is there a rollback path? If the migration corrupts data or the new schema causes errors, can the application revert to the prior schema?
   - For deployed applications: is there a forward-compatibility window — a period where old clients writing under the old schema coexist with new clients reading under the new schema? Does the migration handle data written by old clients after the schema change?
   - A normalization function that silently fills in missing fields is not a migration strategy — it is a recovery mechanism. Migration and recovery serve different purposes and should be documented separately.
4. **Data integrity invariants** — Are invariants enforced at the data layer (e.g., required fields never null, IDs unique, timestamps always set)? Are these enforced at write time, read time, or both?
5. **Storage fitness** — Is the storage mechanism appropriate for the data's shape, size, and access patterns? Are there known limitations of the chosen storage (e.g., size quotas, no cross-device sync, no transactions) that affect the design?
6. **Access patterns** — Are data reads and writes scoped correctly? Is data read from storage more often than necessary? Are writes atomic where they need to be?
7. **Serialization and deserialization** — Is serialized data human-readable or debuggable? Are there edge cases in serialization (e.g., dates, special characters, nested structures) that could cause corruption or data loss?
8. **Data consistency** — Can the application end up in a state where stored data is inconsistent with displayed data? Are there race conditions or ordering dependencies in data operations?
9. **Sensitive data handling** — Is any personally identifiable or sensitive data stored? If so, is it stored only what is necessary, and is it handled appropriately for the deployment context?
10. **Test coverage of data paths** — Are the normalization, validation, and migration paths covered by tests? Do tests verify the shape of stored data, not just UI behavior?
11. **Data volume limits** — Has the application been tested with an order-of-magnitude more data than the expected typical case? Named failure modes: `localStorage` silently stops accepting writes at ~5–10MB (behavior varies by browser); a list rendered without pagination or virtual scrolling becomes unusable at 1000+ items; a synchronous sort or filter over a large dataset blocks the main thread. Storage limits should be enforced explicitly with a user-visible error rather than failing silently. If the application has no explicit data volume limit in DESIGN.md, flag the assumption that the data set will remain small as a risk that needs a limit or a scale test.

---

Review entries are logged in `iterative-adversarial-refinement/DATA-ENGINEERING-REVIEW.md` inside the project being reviewed.
