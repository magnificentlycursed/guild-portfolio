# Iterative Adversarial Refinement (IAR)

IAR fills the role of the adversary in the Verified Spec-Driven Development (VSDD) pipeline. VSDD structures software development as a six-phase cycle; IAR is Phase 4 — Adversarial Refinement. The adversary applies structured pressure across specialized domains, each with a different lens. It operates with fresh context, iterates until maximum viable refinement (MVR), and certifies that the exit condition was reached honestly.

IAR is not a pre-merge checkpoint. It is an active part of the build cycle. Rounds run during layer development, not just at the end. A layer does not merge when it passes one IAR run — it merges when an IAR run produces only **hallucinated** findings across all active domains. That is the maximum viable refinement signal: the adversary has run out of real complaints.

## VSDD pipeline context

VSDD defines six phases. IAR owns Phase 4. Understanding the full pipeline matters because IAR evaluates *whether the prior phases were executed correctly*, not just whether the code is good.

| Phase | Name | What happens | IAR's role |
|---|---|---|---|
| 1 | Spec Crystallization | Design doc written with behavioral contracts, edge case catalog, interface definitions, verification architecture | VDD-IAR Alignment dim 1 evaluates spec completeness |
| 1b | Decomposition | Project broken into layered TODO.md; Red Gate test plans written per layer; crosslink issue hierarchy created | VDD-IAR Alignment dims 2–3 evaluate layer structure and gate compliance |
| 2 | Red Gate | All tests written and failing before implementation begins | VDD-IAR Alignment dim 4 + QE dim 2 evaluate Red Gate compliance |
| 3 | Implementation | Tests made to pass; no new tests added during this phase | SE, QE, UX, Security domains evaluate implementation quality |
| **4** | **Adversarial Refinement** | **IAR runs until MVR** | **This is IAR** |
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

| Domain | Prompt file | Focus |
|---|---|---|
| Quality Engineering | [QUALITY-ENGINEERING-REVIEW.md](QUALITY-ENGINEERING-REVIEW.md) | Test system: acceptance criteria, falsifiability, Red Gate compliance, coverage meaningfulness, logic errors, dead code, dependencies, security surface, regression coverage, quality gates, TDD proxy indicators |
| UX | [UX-REVIEW.md](UX-REVIEW.md) | User experience: empty states, error messages, focus and keyboard behavior, visual consistency, affordances, feedback patterns, long content, native dialog quality. Standard dimensions assume browser interface — see `lang/cli.md` for CLI projects. |
| Security | [SECURITY-REVIEW.md](SECURITY-REVIEW.md) | Input handling, persistence data validation, dependency CVEs, secret handling, information exposure, authentication and authorization |
| Platform Engineering | [PLATFORM-ENGINEERING-REVIEW.md](PLATFORM-ENGINEERING-REVIEW.md) | CI/CD pipeline, gate enforcement, DevSecOps (pre-commit hooks, security scanning, secret management, supply chain integrity, least privilege), infrastructure as code, containerization, environment parity, observability |
| Solution Architect | [SOLUTION-ARCHITECT-REVIEW.md](SOLUTION-ARCHITECT-REVIEW.md) | Architecture: separation of concerns, coupling, data model integrity, interface contracts, state management, immutability, extensibility, technology fitness, complexity budget, decision documentation, session continuity, VSDD purity boundary map |
| Solution Owner | [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) | Spec contract: spec coverage, scope creep, technology compliance, over-engineering, under-delivery, design fidelity, backlog candidates, prior-review additions, assignment compliance (phase-appropriate). Opens with a compliance table. DESIGN.md is the contract. |
| Software Engineering | [SOFTWARE-ENGINEERING-REVIEW.md](SOFTWARE-ENGINEERING-REVIEW.md) | Implementation: correctness, error handling, naming, function design, duplication, complexity, type safety, defensive coding, comments, consistency, future-self maintainability |
| Data Engineering | [DATA-ENGINEERING-REVIEW.md](DATA-ENGINEERING-REVIEW.md) | Data layer: data model correctness, validation and normalization, schema evolution, data integrity, storage fitness, access patterns, serialization, consistency, sensitive data handling. Optional for projects without a meaningful data layer. |
| VDD-IAR Alignment | [VDD-IAR-ALIGNMENT-REVIEW.md](VDD-IAR-ALIGNMENT-REVIEW.md) | Process and governing doc compliance: design-before-code, spec completeness (VSDD Phase 1), layered decomposition, layer gate compliance, test discipline (Red Gate), human verification, IAR fresh context, IAR iteration, role integrity, manual testing checklists, retrospective quality, issue tracking compliance |

Each domain file contains the current prompt and standard dimensions. Review entries are logged separately under `iterative-adversarial-refinement/` inside the project being reviewed.

The suite's own adversarial review history is logged in [`SUITE-REVIEW.md`](SUITE-REVIEW.md).

## Session primers

Session primers prime a session for building, not reviewing. They are used at the start of a phase before any artifacts exist.

| Primer | File | When to use |
|---|---|---|
| Spec Crystallization | [`prompts/spec-crystallization.md`](prompts/spec-crystallization.md) | Starting a new project. Use this before writing DESIGN.md. Drives behavioral contracts, edge cases, interface definitions, verification architecture. |
| Decomposition | [`prompts/decomposition.md`](prompts/decomposition.md) | After DESIGN.md is complete and argued with. Use this to produce TODO.md with layered acceptance criteria, Red Gate test plans, manual testing checklists, and (Phase 2+) crosslink issue hierarchy. |

The spec crystallization primer establishes the adversarial posture for spec *writing* — the adversary applies pressure during Phase 1, not only during Phase 4. A spec that was never argued with before implementation began will produce IAR findings that trace back to spec incompleteness, not implementation error.

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

### The refinement loop

IAR is iterative. Within a single layer, rounds run until maximum viable refinement (MVR):

1. **First pass** — Run active domains when the layer is functionally complete. Log all findings. Fix substantive findings.
2. **Second pass** — Re-run affected domains with fresh AI context. Fix remaining findings.
3. **Continue** until a full pass across all active domains produces only **hallucinated** findings or no findings. That is the MVR signal: the adversary has run out of real complaints.
4. **Merge** — Once MVR is reached across all active domains.

Round numbers belong in the log. `QE Review 1`, `QE Review 2` is the expected pattern. The progression from real findings to hallucinated findings is evidence the process worked. A layer that merges after a single pass with unresolved real findings is a process failure — log it as one in VDD-IAR Alignment.

### Session isolation

An AI agent that reviews multiple domains in one conversation session accumulates context that softens its adversarial pressure. For strongest isolation, reset the AI session between domain reviews — start a fresh conversation for each domain, load only that domain's prompt and the code under review. Parallel sessions are the gold standard; batching domains in one long session is a quality tradeoff. This mirrors the "fresh eyes every time" principle from VDD.

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
- Run QE before UX when QE finds bugs that change the implementation — the UX reviewer should see the fixed version
- Run DE before SA when there are significant data model changes — DE findings can change what SA needs to evaluate
- Run VDD-IAR Alignment last — it reviews the process artifacts produced by all other domain runs
- Run all domains, then re-run any that received a cross-domain flag

### Generalist adversary pass (optional)

After all specialist domains pass, optionally run an unstructured general pass with a fresh AI session and no domain framework: read everything, apply no specific dimensions, find whatever the specialists missed. This is the adversary described in the VDD methodology — no categories, just problems. It is most useful when specialist domains are producing only hallucinated findings and you want a final confidence check before merge.

### Suggesting new domains

Any domain review may propose adding a new review domain to IAR. Log it as a finding — include a proposed name, purpose statement, and an initial set of standard dimensions. If adopted, create the prompt file here, add it to the table above, and update the project's design document, task list, and PR template.

Candidate domains to consider as a project grows: Performance, Internationalisation, SEO, Privacy, Formal Verification (for VSDD Phase 5+).

The `GAP-ANALYSIS-LOG.md` tracks gap analysis runs against the suite itself. Re-run it when the suite changes, a new project type is being evaluated, or a post-mortem reveals a class of defect the suite did not catch.

### Portfolio-arc review

Per-project IAR runs evaluate individual projects. Before submitting a portfolio, run a separate pass that spans all projects and evaluates the arc:

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
    QUALITY-ENGINEERING-REVIEW.md
    UX-REVIEW.md
    SECURITY-REVIEW.md
    PLATFORM-ENGINEERING-REVIEW.md
    SOLUTION-ARCHITECT-REVIEW.md
    SOLUTION-OWNER-REVIEW.md
    SOFTWARE-ENGINEERING-REVIEW.md
    DATA-ENGINEERING-REVIEW.md
    VDD-IAR-ALIGNMENT-REVIEW.md
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
