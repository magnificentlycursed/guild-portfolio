# Adversarial Iterative Refinement (AIR)

AIR applies iterative adversarial pressure across multiple review domains to find, document, and resolve defects before merging. Every layer must pass a full AIR run before it may be merged.

Not all domains are applicable to every project. Select the domains relevant to the project's scope and technology when setting up AIR. Document which domains are active in the project's own design or task file.

## Domains

| Domain | Prompt file | Focus |
|---|---|---|
| Quality Engineering | [QUALITY-ENGINEERING-REVIEW.md](QUALITY-ENGINEERING-REVIEW.md) | Test strategy and architecture, acceptance criteria, falsifiability, coverage meaningfulness, logic errors, dead code, dependencies, accessibility, browser compatibility, responsive design, security surface, regression coverage, quality gates |
| UX | [UX-REVIEW.md](UX-REVIEW.md) | Empty states, error messages, focus and keyboard behavior, visual consistency, affordances, feedback patterns, accessibility, responsive design, browser compatibility, long content, reduced motion, native dialog quality |
| Security | [SECURITY-REVIEW.md](SECURITY-REVIEW.md) | Rendering safety, URL injection, storage validation, dependency CVEs, CSP, information exposure, input handling |
| Platform Engineering | [PLATFORM-ENGINEERING-REVIEW.md](PLATFORM-ENGINEERING-REVIEW.md) | CI/CD pipeline, gate enforcement, DevSecOps (security scanning, secret management, supply chain integrity, least privilege), infrastructure as code, containerization, environment parity, observability (logging, metrics, alerting, tracing) |
| Solution Architect | [SOLUTION-ARCHITECT-REVIEW.md](SOLUTION-ARCHITECT-REVIEW.md) | Separation of concerns, coupling and cohesion, data model integrity, interface contracts, state management, immutability, extensibility, technology fitness, complexity budget, decision documentation |
| Solution Owner | [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) | Spec coverage, scope creep, technology compliance, over-engineering, under-delivery, design fidelity, backlog candidates |
| Software Engineering | [SOFTWARE-ENGINEERING-REVIEW.md](SOFTWARE-ENGINEERING-REVIEW.md) | Correctness, error handling, naming, function design, duplication, complexity, type safety, defensive coding, comments, consistency |
| Data Engineering | [DATA-ENGINEERING-REVIEW.md](DATA-ENGINEERING-REVIEW.md) | Data model correctness, validation and normalization, schema evolution, data integrity, storage fitness, access patterns, serialization, consistency, sensitive data handling |

Each domain file contains the current prompt and standard dimensions for that domain. Review entries are logged separately under `adversarial-iterative-refinement/` inside the project being reviewed.

## Running AIR

All domains may be run independently at any time. A full run is required before merging; individual domains may be invoked mid-layer to catch issues early or validate a specific concern.

### Full run (required before merging)

Run all active domains. Domains are independent and may run in parallel. If a finding in one domain creates a new concern in another (e.g., a QE implementation change affects the security surface), flag it for that domain and sequence a targeted follow-up.

### Scoped run

Provide a scope to focus primary analysis on a specific feature, layer, or set of changed files. Regression checks always cover the whole application regardless of scope.

Example scopes:
- `"Layer 5 Search — src/search.ts, index.html search bar"`
- `"handleDeleteClick in src/main.ts"`
- `"All files changed since last AIR run"`

When a scope is given, each domain reads it, concentrates its analysis there, and flags regressions it finds in unscoped areas.

### Sequencing

Default: run all active domains in parallel. Sequence when one domain's output informs another:

- Run SA first when there are significant structural or architectural changes — SA findings can change what QE, UX, and Security need to evaluate
- Run PE first when there are significant pipeline or build config changes — other domains depend on the pipeline running correctly
- Run Security before QE when there are significant changes to storage, rendering, or input handling — QE tests may need to cover the security-relevant paths
- Run QE before UX when QE finds bugs that change the implementation — the UX reviewer should see the fixed version
- Run DE before SA when there are significant data model changes — DE findings can change what SA needs to evaluate
- Run all domains, then re-run any that received a cross-domain flag

### Suggesting new domains

Any domain review may propose adding a new review domain to AIR. Log it as a finding — include a proposed name, purpose statement, and an initial set of standard dimensions. If adopted, create the prompt file here, add it to the table above, and update the project's design document, task list, and PR template.

Candidate domains to consider as a project grows: Performance, Internationalisation, SEO, Privacy.

## Review logs

Review entries are stored outside the prompt files to keep the prompts stable and reusable. Logs live at:

```
{project}/
  adversarial-iterative-refinement/
    QUALITY-ENGINEERING-REVIEW.md
    UX-REVIEW.md
    SECURITY-REVIEW.md
    PLATFORM-ENGINEERING-REVIEW.md
    SOLUTION-ARCHITECT-REVIEW.md
    SOLUTION-OWNER-REVIEW.md
    SOFTWARE-ENGINEERING-REVIEW.md
    DATA-ENGINEERING-REVIEW.md
```

Only include log files for the domains active on the project. Each log file follows the same structure as a review entry: scope line, then **Resolved**, **Dismissed** (and **Accepted Risk** for Security and **Backlogged** for Solution Owner), and a **Tests:** footer line.

## Merging gate

Before a layer may be merged:

1. All active AIR domains have completed a full run scoped to that layer (or the whole project, if scope-creep is a risk)
2. Every finding is either **resolved** (fix applied and verified) or **dismissed** (rationale documented)
3. Accepted risks are explicitly documented with rationale
4. Results are logged in the respective log files under `{project}/adversarial-iterative-refinement/`

No active domain may be skipped. A domain with zero findings is a valid outcome — log it with `**Scope:**` and `**Tests:**` lines so the record is complete.
