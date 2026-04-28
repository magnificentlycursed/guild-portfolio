# Iterative Adversarial Refinement (IAR)

IAR fills the role of the adversary in the Verified Spec-Driven Development (VSDD) pipeline. VSDD structures software development as a six-phase cycle; IAR owns Phase 3 — Adversarial Refinement. The adversary applies structured pressure across specialized domains, each with a different lens. It operates with fresh context, iterates until maximum viable refinement (MVR), and certifies that the exit condition was reached honestly.

IAR is not a pre-merge checkpoint. It is an active part of the build cycle. Rounds run during layer development, not just at the end. A layer does not merge when it passes one IAR run — it merges when an IAR run produces only **hallucinated** findings across all active domains. That is the maximum viable refinement signal: the adversary has run out of real complaints.

## VSDD pipeline context

VSDD defines six phases. IAR owns Phase 3. Understanding the full pipeline matters because IAR evaluates *whether the prior phases were executed correctly*, not just whether the code is good.

| Phase | Name | What happens | IAR's role |
|---|---|---|---|
| 1 | Spec Crystallization | Design doc written with behavioral contracts, edge case catalog, interface definitions, verification architecture | VDD-IAR Alignment dim 1 evaluates spec completeness |
| 1b | Decomposition | Project broken into layered TODO.md; Red Gate test plans written per layer; crosslink issue hierarchy created | VDD-IAR Alignment dims 2–3 evaluate layer structure and gate compliance |
| 2a | Red Gate | All tests written and failing before implementation begins | VDD-IAR Alignment dim 4 + QE dim 2 evaluate Red Gate compliance |
| 2b | Implementation | Tests made to pass; no new tests added during this phase | SE, QE, UX, Security domains evaluate implementation quality |
| **3** | **Adversarial Refinement** | **IAR runs until MVR** | **This is IAR** |
| 4 | Feedback Integration | Findings route back to the appropriate earlier phase: spec issues to Phase 1, test issues to Phase 2a, implementation issues to Phase 2b | IAR findings drive this loop; round count and finding progression are logged per domain |
| 5 | Formal Hardening | Proof harnesses, fuzzing, mutation testing (not yet owned by this suite — see GAP-ANALYSIS-LOG G-55) | — |
| 6 | Four-Dimensional Convergence | Spec, tests, implementation, and formal verification all independently at MVR | Partially owned — implementation MVR only (see G-54) |

**Session primers** for Phases 1 and 1b are in `prompts/`. These prime the session before writing begins — they are not review prompts.

## Governing references

- **VSDD whitepaper** (primary): https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00
- **Original VDD whitepaper**: https://gist.github.com/dollspace-gay/45c95ebfb5a3a3bae84d8bebd662cc25
- **Apprentice-onboarding** (program methodology, tool schedule, assignment briefs): https://github.com/Navigators-Guild/apprentice-onboarding
- **CLAUDE.md** (may be superseded — verify against current apprentice-onboarding): https://gist.github.com/dollspace-gay/ef132e60a27abe6d5f87297c1c040dca
- **Crosslink** (issue tracker, Phase 2+): https://github.com/forecast-bio/crosslink

## Domains

Domains are organized into three types. All domain prompt files live under `domains/`.

**Core role domains** — run on every project. The reviewer embodies a specific job role and brings that role's full professional lens:

| Role | Job title | Prompt file | Focus |
|---|---|---|---|
| Software Engineer | Software Engineer / Backend Engineer / Frontend Engineer | [SOFTWARE-ENGINEER-REVIEW.md](domains/role/SOFTWARE-ENGINEER-REVIEW.md) | Implementation: correctness, error handling, naming, function design, duplication, complexity, type safety, defensive coding, comments, consistency, future-self maintainability, documentation, performance |
| Quality Engineer | Quality Engineer / QA Engineer / Test Engineer | [QUALITY-ENGINEER-REVIEW.md](domains/role/QUALITY-ENGINEER-REVIEW.md) | Test system: acceptance criteria, falsifiability, Red Gate compliance, coverage meaningfulness, logic errors, dead code, dependencies, security surface, regression coverage, quality gates, TDD proxy indicators |
| UX Designer | UX Designer / UX Researcher / Product Designer | [UX-REVIEW.md](domains/role/UX-REVIEW.md) | User experience: empty states, error messages, focus and keyboard behavior, visual consistency, affordances, feedback patterns, long content, native dialog quality. Standard dimensions assume browser interface — see `lang/cli.md` for CLI projects. |
| Security Engineer | Security Engineer / Application Security Engineer | [SECURITY-REVIEW.md](domains/role/SECURITY-REVIEW.md) | Input handling, persistence data validation, dependency CVEs, secret handling, information exposure, authentication and authorization, audit logging, data classification and control requirements |
| Platform Engineer | Platform Engineer / DevOps Engineer / Infrastructure Engineer | [PLATFORM-ENGINEER-REVIEW.md](domains/role/PLATFORM-ENGINEER-REVIEW.md) | CI/CD pipeline, gate enforcement, DevSecOps (pre-commit hooks, security scanning, secret management, supply chain integrity, least privilege), infrastructure as code, containerization, environment parity, observability, performance |
| Solution Architect | Solution Architect / Software Architect / Technical Lead | [SOLUTION-ARCHITECT-REVIEW.md](domains/role/SOLUTION-ARCHITECT-REVIEW.md) | Architecture: separation of concerns, coupling, data model integrity, interface contracts, state management, immutability, extensibility, technology fitness, complexity budget, decision documentation, session continuity, VSDD purity boundary map, external interface contracts, external service integration |
| Solution Owner | Solution Owner / Product Owner / Product Manager | [SOLUTION-OWNER-REVIEW.md](domains/role/SOLUTION-OWNER-REVIEW.md) | Spec contract: spec coverage, scope creep, technology compliance, over-engineering, under-delivery, design fidelity, backlog candidates, prior-review additions, assignment compliance (phase-appropriate). Opens with a compliance table. DESIGN.md is the contract. |
| Data Engineer | Data Engineer / Database Engineer / Data Platform Engineer | [DATA-ENGINEER-REVIEW.md](domains/role/DATA-ENGINEER-REVIEW.md) | Data layer: data model correctness, validation and normalization, schema evolution, data integrity, storage fitness, access patterns, serialization, consistency, sensitive data handling. See [DOMAIN-INDEX.md](domains/role/DOMAIN-INDEX.md) for scope-down guidance. |

**Extended role domains** — active when a project's scope warrants them. Select based on deployment context and audience; document which are active in the project's design or task file:

| Role | Job title | Prompt file | Focus |
|---|---|---|---|
| Red Team Hacker | Penetration Tester / Offensive Security Engineer | [RED-TEAM-REVIEW.md](domains/role/RED-TEAM-REVIEW.md) | Offensive security: threat modeling, attack surface enumeration, authentication bypass, authorization flaws, business logic abuse, injection chains, client-side attacks, information leakage, chained vulnerabilities, insider threat, automated attack resilience, supply chain exploitation |
| Performance Engineer | Performance Engineer / Site Performance Engineer | [PERFORMANCE-ENGINEER-REVIEW.md](domains/role/PERFORMANCE-ENGINEER-REVIEW.md) | Runtime performance: time-to-interactive, main thread saturation, asset optimization, data scaling, N+1 patterns, caching, memory growth, performance budget, regression risk |
| Technical Writer | Technical Writer / Developer Experience Engineer | [TECHNICAL-WRITER-REVIEW.md](domains/role/TECHNICAL-WRITER-REVIEW.md) | README completeness, documentation accuracy, architecture documentation, decision rationale, inline comment quality, API/interface docs, operational docs, CHANGELOG quality, AI session independence |
| Accessibility Engineer | Accessibility Engineer / A11y Specialist | [ACCESSIBILITY-REVIEW.md](domains/role/ACCESSIBILITY-REVIEW.md) | WCAG 2.1 AA compliance at depth: automated scan baseline, keyboard navigation, focus management, focus traps, ARIA correctness, contrast, form accessibility, dynamic content announcements, cognitive accessibility, zoom/reflow |
| Privacy Officer | Privacy Officer / Privacy Engineer / DPO | [PRIVACY-REVIEW.md](domains/role/PRIVACY-REVIEW.md) | Data minimization, legal basis, retention policy, user rights (access/erasure/portability), third-party sharing, consent quality, PII in secondary storage, privacy by design |
| Localization Engineer | Localization Engineer / L10n Engineer | [LOCALIZATION-REVIEW.md](domains/role/LOCALIZATION-REVIEW.md) | i18n readiness: string externalization, date/time/number formatting, RTL support, text expansion tolerance, plural rules, locale-sensitive validation, character encoding, cultural neutrality |

**Meta domains** — evaluate process and portfolio artifacts rather than the software itself. No job role persona; the reviewer evaluates methodology compliance or portfolio evidence:

| Domain | Prompt file | Scope |
|---|---|---|
| VDD-IAR Alignment | [VDD-IAR-ALIGNMENT-REVIEW.md](domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md) | Every project — process compliance: design-before-code, spec completeness, layered decomposition, layer gates, test discipline (Red Gate), human verification, IAR integrity, issue tracking |
| Portfolio Assessment | [PORTFOLIO-ASSESSMENT-REVIEW.md](domains/meta/PORTFOLIO-ASSESSMENT-REVIEW.md) | Portfolio and apprentice submissions only — developer ownership: decision ownership, implementation understanding, directed development evidence, growth evidence, failure honesty, spec ownership |

Each domain file contains the current prompt and standard dimensions. Review entries are logged separately under `iterative-adversarial-refinement/` inside the project being reviewed.

The suite's own adversarial review history is logged in [`SUITE-REVIEW.md`](SUITE-REVIEW.md).

## Session primers

Session primers establish posture and context at the start of a phase. Use the primer for the phase you are entering before loading any other prompt.

| Primer | File | When to use |
|---|---|---|
| Spec Crystallization | [`prompts/spec-crystallization.md`](prompts/spec-crystallization.md) | Starting a new project. Use before writing DESIGN.md. Drives behavioral contracts, edge cases, interface definitions, verification architecture. |
| Decomposition | [`prompts/decomposition.md`](prompts/decomposition.md) | After DESIGN.md is complete and argued with. Produces TODO.md with layered acceptance criteria, Red Gate test plans, manual testing checklists, and (Phase 2+) crosslink issue hierarchy. |
| Implementation | [`prompts/implementation.md`](prompts/implementation.md) | At the start of each implementation layer (Phase 2a–2b). Establishes Red Gate discipline: tests before code, one failing test per acceptance criterion before the first line of implementation. |
| Adversarial Review | [`prompts/review-session.md`](prompts/review-session.md) | At the start of each fresh IAR review session (Phase 3). Establishes adversarial posture before loading any domain prompt. Use in a cold session that has not participated in building the project. |
| Suite Development | [`prompts/suite-development.md`](prompts/suite-development.md) | When developing the IAR suite itself — adding domains, updating dimensions, running gap analysis. Not for reviewing projects. |

The spec crystallization primer establishes the adversarial posture for spec *writing* — the adversary applies pressure during Phase 1, not only during Phase 3. A spec that was never argued with before implementation began will produce IAR findings that trace back to spec incompleteness, not implementation error.

## Language and interface supplements

Language-specific and interface-type-specific dimensions live in `lang/`. Domain prompts reference these — during a review, apply the relevant supplement's section for your domain alongside the standard dimensions.

| Supplement | When to use |
|---|---|
| [`lang/rust.md`](lang/rust.md) | Rust projects (all domains) |
| [`lang/javascript-typescript.md`](lang/javascript-typescript.md) | JavaScript or TypeScript projects (all domains) |
| [`lang/cli.md`](lang/cli.md) | CLI interface type — replaces browser-centric UX dimensions; adds CLI QE and SE concerns |
| [`lang/browser-app.md`](lang/browser-app.md) | Browser-rendered interface — browser-specific QE (axe, compatibility, responsive), Security (rendering safety, CSP, SRI), and UX dimensions |

A project may use more than one supplement. A TypeScript CLI uses both `javascript-typescript.md` and `cli.md`.

## Running IAR

All domains may be run independently at any time. A full run is required before merging; individual domains may be invoked mid-layer to catch issues early or validate a specific concern.

**Human-in-the-loop requirement:** IAR's adversarial value depends on a human reading every finding and making every classification decision. The sycophancy checks in each domain exist to prevent an AI agent from validating its own work — but those checks only work if a human is the final decision-maker on what is real, what is dismissed, and what is hallucinated. Automating finding classification without human review collapses the adversarial process into a rubber-stamp loop.

**DESIGN.md prerequisite:** Every domain prompt begins with "Read DESIGN.md first." If DESIGN.md does not exist, do not run domain reviews — there is no spec to evaluate against. Run VDD-IAR Alignment first; it will flag the absence as a process failure (dim 1). Other domains are not meaningful until the spec exists.

**Domain activation:** Core domains run on every project. Extended domains activate based on project type and deployment context — see [`domains/role/DOMAIN-INDEX.md`](domains/role/DOMAIN-INDEX.md) for activation criteria. Document which extended domains are active in the project's DESIGN.md or task file before the first IAR run.

### The refinement loop

IAR is iterative. Within a single layer, rounds run until maximum viable refinement (MVR):

1. **First pass** — Run active domains when the layer is functionally complete. Log all findings. Fix substantive findings.
2. **Second pass** — Re-run affected domains with fresh AI context. Fix remaining findings.
3. **Continue** until a full pass across all active domains produces only **hallucinated** findings or no findings. That is the MVR signal: the adversary has run out of real complaints.
4. **Merge** — Once MVR is reached across all active domains.

Round numbers belong in the log. `QE Review 1`, `QE Review 2` is the expected pattern. The progression from real findings to hallucinated findings is evidence the process worked. A layer that merges after a single pass with unresolved real findings is a process failure — log it as one in VDD-IAR Alignment.

### Session isolation

An AI agent that reviews multiple domains in one conversation session accumulates context that softens its adversarial pressure. For strongest isolation, reset the AI session between domain reviews — start a fresh conversation for each domain, load only that domain's prompt and the code under review. Parallel sessions are the gold standard; batching domains in one long session is a quality tradeoff. This mirrors the "fresh eyes every time" principle from VDD.

**Same-model review limitation:** The original VDD methodology was designed for cross-model review — the Builder (Claude) and the Adversary (Gemini/Sarcasmotron) are distinct agents with different training and different biases. Same-model review carries elevated sycophancy risk even with context resets: the adversary shares the builder's failure modes and blind spots. The posture primer, context isolation, and domain-specific sycophancy checks exist to partially compensate; they do not fully replicate cross-model adversarial pressure. For the highest-stakes reviews, consider using a genuinely different model as the adversary.

### Scoped run

Provide a scope to focus primary analysis on a specific feature, layer, or set of changed files. Regression checks always cover the whole application regardless of scope.

Example scopes:
- `"Layer 5 Search — src/search.ts, index.html search bar"`
- `"handleDeleteClick in src/main.ts"`
- `"All files changed since last IAR run"`

When a scope is given, each domain concentrates analysis there and flags regressions found in unscoped areas.

### Sequencing

Default: run all active domains in parallel. Sequence when one domain's output informs another:

- Run SA first when there are significant structural or architectural changes — SA findings can change what QE, UX, and Security need to evaluate
- Run PE first when there are significant pipeline or build config changes — other domains depend on the pipeline running correctly
- Run Security before QE when there are significant changes to storage, rendering, or input handling — QE tests may need to cover the security-relevant paths
- Run Security before Red Team — Security Engineer ensures controls exist; Red Team verifies they hold under attack. A Red Team finding that traces to an absent control is a Security finding too
- Run QE before UX when QE finds bugs that change the implementation — the UX reviewer should see the fixed version
- Run DE before SA when there are significant data model changes — DE findings can change what SA needs to evaluate
- Run VDD-IAR Alignment last in the final merge gate — it reviews the process artifacts produced by all other domain runs
- Run VDD-IAR Alignment also at each layer gate close (dims 2–3: layered decomposition and gate compliance) — layer gate failures are more actionable when caught while the layer is still open, not retrospectively at merge time
- Run all domains, then re-run any that received a cross-domain flag

### Generalist adversary pass (optional)

After all specialist domains pass, optionally run an unstructured general pass with a fresh AI session and no domain framework: read everything, apply no specific dimensions, find whatever the specialists missed. This is the adversary described in the VDD methodology — no categories, just problems. It is most useful when specialist domains are producing only hallucinated findings and you want a final confidence check before merge.

### Suggesting new domains

Any domain review may propose adding a new review domain to IAR. Log it as a finding — include a proposed name, purpose statement, and an initial set of standard dimensions. If adopted, create the prompt file here, add it to the table above, and update the project's design document, task list, and PR template.

Candidate domains to consider as a project grows: SEO, Formal Verification (for VSDD Phase 5+).

The `GAP-ANALYSIS-LOG.md` tracks gap analysis runs against the suite itself. Re-run it when the suite changes, a new project type is being evaluated, or a post-mortem reveals a class of defect the suite did not catch.

### Portfolio-arc review

Per-project IAR runs evaluate individual projects using the [`domains/meta/PORTFOLIO-ASSESSMENT-REVIEW.md`](domains/meta/PORTFOLIO-ASSESSMENT-REVIEW.md) domain. Before submitting a portfolio, also run a separate pass that spans all projects and evaluates the arc:

- **Growth** — Does the process documentation show improvement from the first project to the last? Design docs, test discipline, commit history, and IAR depth should all mature visibly.
- **Honest retrospective** — Does each project have a post-mortem or DECISIONS.md that admits what went wrong, what was cut, and what was learned? "Everything went perfectly" is a red flag.
- **Assignment alignment** — Does each project match what the assignment asked for, or did scope creep enter at the design stage?
- **Independence** — Does the commit history, problem selection, and scope decisions show that you directed the work, or did the agent make all the choices?
- **Process over product** — A simple tool built with disciplined process outranks a complex tool built chaotically.

## Review logs

Review entries are stored outside the prompt files to keep the prompts stable and reusable. Logs live at:

```
{project}/
  iterative-adversarial-refinement/
    # Core domains (always active)
    QUALITY-ENGINEER-REVIEW.md
    UX-REVIEW.md
    SECURITY-REVIEW.md
    PLATFORM-ENGINEER-REVIEW.md
    SOLUTION-ARCHITECT-REVIEW.md
    SOLUTION-OWNER-REVIEW.md
    SOFTWARE-ENGINEER-REVIEW.md
    DATA-ENGINEER-REVIEW.md
    VDD-IAR-ALIGNMENT-REVIEW.md
    # Extended domains (include only those active on the project)
    RED-TEAM-REVIEW.md
    PERFORMANCE-ENGINEER-REVIEW.md
    TECHNICAL-WRITER-REVIEW.md
    ACCESSIBILITY-REVIEW.md
    PRIVACY-REVIEW.md
    LOCALIZATION-REVIEW.md
```

The `lang/` folder, `GAP-ANALYSIS-LOG.md`, `SUITE-REVIEW.md`, and `prompts/` live in the suite template, not in individual projects.

Only include log files for the domains active on the project. Each log file follows the same structure: scope line, round number, then findings classified as **resolved**, **dismissed**, **hallucinated** (and **accepted risk** for Security, **backlogged** for Solution Owner).

## Merging gate

Before a layer may be merged:

1. All active IAR domains have completed at least one full run scoped to that layer
2. The refinement loop has run to MVR — the final round produces only hallucinated findings or no findings
3. Every finding is either **resolved** (fix applied and verified), **dismissed** (rationale documented), or **hallucinated** (push back documented)
4. Accepted risks are explicitly documented with rationale
5. VDD-IAR Alignment has been run and process compliance confirmed
6. Results are logged with round numbers in the respective log files under `{project}/iterative-adversarial-refinement/`

No active domain may be skipped. A domain with zero findings is a valid outcome — log it with `**Scope:**`, round number, and `**Tests:**` lines so the record is complete.
