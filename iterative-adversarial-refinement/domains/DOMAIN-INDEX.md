# Domain Index

Quick reference for domain classification and activation. The `role/` folder contains both **core** domains (run on every project) and **extended** domains (run when project scope warrants). This file is the authoritative list of which is which, and the criteria for activating each extended domain.

For full domain descriptions, sequencing, and coordination guidance, see [`README.md`](../README.md).

---

## Core domains — run on every project

These eight domains apply to all projects regardless of type, deployment context, or scale. A project may not skip a core domain. A domain with zero findings is a valid outcome — log it with scope and round number so the record is complete.

| Domain file | Role | Notes |
|---|---|---|
| `role/SOFTWARE-ENGINEER-REVIEW.md` | Software Engineer | Implementation quality within module boundaries |
| `role/QUALITY-ENGINEER-REVIEW.md` | Quality Engineer | Test system correctness, coverage, Red Gate compliance |
| `role/UX-REVIEW.md` | UX Designer | User experience — browser apps use standard dims; CLI projects use `supplements/cli.md` replacement dims |
| `role/SECURITY-REVIEW.md` | Security Engineer | Defensive controls — run before Red Team |
| `role/PLATFORM-ENGINEER-REVIEW.md` | Platform Engineer | CI/CD, DevSecOps, infrastructure, observability |
| `role/SOLUTION-ARCHITECT-REVIEW.md` | Solution Architect | Architecture, boundaries, data model, technology fitness |
| `role/SOLUTION-OWNER-REVIEW.md` | Solution Owner | Spec contract — DESIGN.md compliance, scope creep, over-engineering |
| `role/DATA-ENGINEER-REVIEW.md` | Data Engineer | Data layer — may be scoped down for projects with no meaningful data layer, but must be logged |

---

## Extended domains — activate when scope warrants

Each extended domain has explicit activation criteria. Activation decisions should be recorded in the project's DESIGN.md or task file before the first IAR run. A project that meets the activation criteria for an extended domain but does not run it requires documented rationale.

| Domain file | Role | Activate when |
|---|---|---|
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
