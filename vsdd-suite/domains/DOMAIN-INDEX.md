# Domain Index

Quick reference for domain classification and activation. The `role/` folder contains both **core** domains (run on every project) and **extended** domains (run when project scope warrants). This file is the authoritative list of which is which, and the criteria for activating each extended domain.

For full domain descriptions, sequencing, and coordination guidance, see [`README.md`](../README.md).

---

## Core domains — run on every project

Six core role domains plus the VDD-IAR Alignment meta domain (seven total) apply to all projects regardless of type, deployment context, or scale. A project may not skip a core domain. A domain with zero findings is a valid outcome — log it with scope and round number so the record is complete.

The VDD-IAR Alignment meta domain is the seventh core domain and is listed in the Meta domains table below; the six role domains are:

| Domain file | Role | Notes |
|---|---|---|
| `role/SOFTWARE-ENGINEER-REVIEW.md` | Software Engineer | Implementation quality within module boundaries |
| `role/QUALITY-ENGINEER-REVIEW.md` | Quality Engineer | Test system correctness, coverage, Red Gate compliance |
| `role/UX-REVIEW.md` | UX Designer | User experience — browser apps use standard dims; CLI projects use `supplements/cli.md` replacement dims |
| `role/SECURITY-REVIEW.md` | Security Engineer | Defensive controls — run before Red Team |
| `role/SOLUTION-ARCHITECT-REVIEW.md` | Solution Architect | Architecture, boundaries, data model, technology fitness |
| `role/SOLUTION-OWNER-REVIEW.md` | Solution Owner | Spec contract — DESIGN.md compliance, scope creep, over-engineering |

**Forward-only constraint (G-178, 2026-05-20):** Platform Engineer and Data Engineer were classified as core role domains prior to v0.7.6 and are reclassified as extended-with-strong-presumption in v0.7.6 to align with operating reality — the scaffold default, the intent calibration, and the worked example had treated them as conditionally active since v0.3.0. Projects whose first IAR run predates v0.7.6 retain their existing PE/DE-as-core classification in their review logs and DESIGN.md notes; this reclassification does not invalidate prior records. New projects scaffolded at v0.7.6+ follow the table above. Reference: G-178's row in [`../suite-development/FINDINGS-INDEX.md`](../suite-development/FINDINGS-INDEX.md).

---

## Extended domains — activate when scope warrants

Each extended domain has explicit activation criteria. Activation decisions should be recorded in the project's DESIGN.md or task file before the first IAR run. A project that meets the activation criteria for an extended domain but does not run it requires documented rationale.

Platform Engineer and Data Engineer are **extended-with-strong-presumption**: most projects beyond a local-toolchain CLI activate at least one of them, and at capstone or production intent they are typically active. The activation criteria below name the operational thresholds.

| Domain file | Role | Activate when |
|---|---|---|
| `role/PLATFORM-ENGINEER-REVIEW.md` | Platform Engineer | The project ships through a CI/CD pipeline; deploys to managed infrastructure; requires container or orchestration configuration; defines pre-commit, security, or observability hooks; or has any operational deployment surface beyond a manual local-toolchain install. Local-only CLIs that ship via `cargo install` / `npm i -g` / `pip install` with no managed pipeline do not require PE. Strong presumption at capstone and production intent. |
| `role/DATA-ENGINEER-REVIEW.md` | Data Engineer | The project manages persistent data through a database, a managed schema, structured storage with non-trivial integrity constraints, or external data systems (ETL, ingestion, replication). Flat-file or single-JSON-blob CLI projects without schema concerns do not require DE; if borderline, activate and scope-down. Strong presumption at capstone and production intent. |
| `role/RED-TEAM-REVIEW.md` | Red Team Hacker | Application has authentication, user-controlled input, network exposure, or data belonging to users other than the developer. Run after Security Engineer. |
| `role/PERFORMANCE-ENGINEER-REVIEW.md` | Performance Engineer | Browser applications, server-side applications with network latency or SLAs, data-intensive tools, or any application where startup time or throughput is a user-visible concern. |
| `role/TECHNICAL-WRITER-REVIEW.md` | Technical Writer | Any project intended for handoff, portfolio submission, external use, or operational deployment. For library projects: always active (API documentation is the primary deliverable). |
| `role/ACCESSIBILITY-REVIEW.md` | Accessibility Engineer | All browser-rendered applications and native UI applications. Not applicable to headless services or CLI tools. When active, run in addition to (not instead of) UX domain. |
| `role/PRIVACY-REVIEW.md` | Privacy Officer | Any application that collects, processes, or stores information about identifiable individuals — including single-user applications that store behavioral data. For applications with users other than the developer, full evaluation is required. |
| `role/LOCALIZATION-REVIEW.md` | Localization Engineer | Any user-facing application that may be used by people speaking languages other than the implementation language, or with locale-specific formatting expectations. For applications explicitly scoped to a single locale, activate to verify the scope is correctly enforced. |

---

## Meta domains — evaluate process and portfolio artifacts

Meta domains do not evaluate the software itself. They evaluate methodology compliance and portfolio evidence. They have no job role persona.

| Domain file | Scope | When to run |
|---|---|---|
| `meta/VDD-IAR-ALIGNMENT-REVIEW.md` | Process compliance: design-before-code, test discipline, layer gates, IAR integrity | At each layer gate close (dims 2–3) and as the final domain in the merge gate |
| `meta/PORTFOLIO-ASSESSMENT-REVIEW.md` | Developer ownership: decision ownership, implementation understanding, directed development evidence | Portfolio projects and apprentice program submissions only |

---

## Intent calibration

The "all cores plus warranted extended domains" default above is the right starting point for **portfolio**-intent projects (the scaffold-default per G-121). Other project intents calibrate the active set up or down so the IAR intensity matches the project's purpose level, not just the scaffold default. The intent is declared in the project's `DESIGN.md` § Project intent (per `templates/DESIGN-template.md`); the active-domain set is selected at decomposition time per `primers/1c-decomposition.md` § Right-size the IAR.

The over-investment failure mode is harder to catch in-project than the under-investment one because more findings *feel* like more value — that's the framing G-150 names and the calibration this section operationalizes.

The seven core domains (SE, QE, UX, Security, SA, SO, VDD-IAR Alignment) are abbreviated "7 core" below. The extended pool draws from {PE, DE, Red Team, Performance Engineer, Technical Writer, Accessibility, Privacy, Localization} per the activation criteria above.

| Intent | Active core domains | Active extended domains | Stop-signal sensitivity (G-151) | Notes |
|---|---|---|---|---|
| **learning-exercise** | SE + QE + SO + VDD-IAR Alignment (4 fixed) + one rotating optional fourth role domain (SA / Security / UX, rotated across layers) | None by default | High — stop early once a round produces only Hallucinated findings; the cost of one missed defect is low relative to process-drift fatigue cost | The rotating fourth exposes the apprentice to different role lenses across layers without running all 7 every layer. Goal: learning, not shipping. The full 7-core + extended treatment is methodological over-investment relative to the assignment bar. PE and DE remain extended at this intent — they activate only on the rare learning-exercise that involves managed infrastructure or a persistent data layer. |
| **portfolio** | All 7 core (default; G-121 scaffold-default) | Technical Writer (when intended for external reading); PE and DE per their activation criteria; others per their activation criteria | Standard — every round until MVR | The suite's default treatment. Apprentice-portfolio level work intended for handoff and external review. A portfolio CLI shipped via `cargo install` with no infrastructure and a flat-file data layer typically activates only TW; a portfolio web service typically activates PE + DE + TW + Accessibility. |
| **capstone** | All 7 core + Performance Engineer | All extended domains meeting activation criteria — PE and DE typically active given capstone-level operational scope | Standard — every round until MVR | Apprentice-graduation-level work. Adds fresh-system install verification discipline (G-155) — a documented third-party install attempt is a gate criterion. |
| **production** | All 7 core + Performance Engineer | All extended domains meeting activation criteria (PE, DE, Red Team, Privacy, Accessibility, Localization as applicable) | Strict — MVR must be reached across all rounds before merge; deferral discipline (G-130) tightens to require an explicit auto-Backlog trigger or named target layer | Software for ongoing operational use by people other than the developer. PE and DE are almost always active at this intent — production software ships through a pipeline and persists data. |

**Intent-calibration discipline:** A project's intent is declared once in `DESIGN.md` § Project intent and is forward-only at that intent. Promoting intent (learning-exercise → portfolio; portfolio → capstone; capstone → production) is allowed and triggers retroactive activation of the additional domains for the next layer onward (prior layers' reviews remain valid records at their intent). Demoting intent (portfolio → learning-exercise) is not allowed — once a project has been reviewed at higher intensity, the surface findings become part of the project's record.

**The calibration is not a license to skip findings.** A learning-exercise project that runs only 3 cores still owes the cores it runs full IAR discipline — Red Gate, sycophancy check, MVR, etc. The calibration narrows the active-domain set; it does not weaken the per-domain bar.

### Phase 5 / Phase 6 strategy declaration (G-162 — capstone + production intents)

Phase 5 (Formal Hardening — [`../primers/5-formal-hardening.md`](../primers/5-formal-hardening.md)) and Phase 6 (Four-Dimensional Convergence — [`../primers/6-convergence.md`](../primers/6-convergence.md)) are owned by the suite as of v0.7.0 (Review 64 closure of G-54 + G-55). Both remain optional at every intent — projects close at the end of Phase 4 by default — but capstone and production intents must declare a strategy for each.

**At capstone and production intents, `DESIGN.md` § Project intent must include a one-sentence Phase 5 strategy and a one-sentence Phase 6 strategy.** Valid declarations (per the explicit-skip pattern generalized from `primers/2c-refactor.md` G-96):

- **`Phase 5: not applicable — <one-sentence rationale>.`** Acceptable rationales: not safety-critical, not cryptographic, no formal-verification candidates in the spec's verification-architecture map. Unacceptable: silence; "TBD"; "future work."
- **`Phase 5: planned — <named tooling and scope, mapped to surfaces in primers/5-formal-hardening.md>.`** Example: "property-based testing via proptest on the purity boundary + mutation testing via cargo-mutants per layer + fuzzing via cargo-fuzz on the parser surface (Surfaces A + B + C; Surface D not applicable — no formal-proof candidates)."
- **`Phase 6: not applicable — <one-sentence rationale>.`** Acceptable rationales: project intent does not promise four-dimensional convergence; closure at end of Phase 4 is sufficient evidence.
- **`Phase 6: planned — <which of the four dimensions converge and how>.`** Example: "all four dimensions; formal verification dimension established via Phase 5 Surface D harnesses on the safety-critical control-loop functions."

Learning-exercise and portfolio intents are exempt — the explicit-skip declaration is not required (silence is acceptable at those tiers because the methodology does not require Phase 5/6 at those intents). The strategy declaration is a gate criterion at capstone+ intent: a capstone-intent project whose `DESIGN.md` § Project intent omits either strategy line fails VDD-IAR Alignment dim 1 (spec completeness). This is forward-only: capstone/production projects starting after this discipline's introduction must declare; earlier projects retain their prior implicit-skip framing.

**Why this is a gate, not advisory:** at capstone+ intent the methodology covers Phase 5 + Phase 6 explicitly; a project that doesn't even *name* the decision is operating outside the methodology rather than choosing not to apply it. The declaration is cheap (two sentences in DESIGN.md); the audit signal is non-trivial (the choice was deliberate, not forgotten). The discipline is enforced by VDD-IAR Alignment dim 1 (spec completeness — strategy declarations are part of the spec).
