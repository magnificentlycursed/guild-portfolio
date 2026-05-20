# Guild Portfolio

Apprentice portfolio for the Navigators Guild. Contains projects built to demonstrate software design, test-driven development, and engineering process.

## Projects

### [Bookmark Manager](./bookmark-manager)

A personal tool for saving and organizing web links with titles, notes, and tags. Built with TypeScript, HTML, and CSS — no frameworks. Data lives in the browser's localStorage.

**Methodology:** design-first, TDD, Iterative Adversarial Refinement (IAR — QA, UX, Security, Platform Engineering, and Solution Architect review domains), and manual testing at each layer before advancing.

| Layer | Feature | Status |
|---|---|---|
| 1 | Core (add, display, persist, validate) | ✅ Complete |
| 2 | Notes and tags | ✅ Complete |
| 3 | Edit and delete | ✅ Complete |
| 4 | Tag filtering | ✅ Complete |
| 5 | Search | ✅ Complete |
| 6 | Polish | ✅ Complete |

### [Issue Tracker CLI](./issue-tracker-cli)

A personal issue tracker for the terminal. Single user, no network, no accounts. Issues stored in a local JSON file. Built in Rust with a git-style subcommand interface (`tracker create`, `tracker list`, and more across 7 planned layers).

**Methodology:** VSDD (Verified Spec-Driven Development) — full behavioral specification before any code, Red Gate (tests written and confirmed failing before implementation), and a 10-domain + 2-meta Iterative Adversarial Refinement suite run at each layer gate.

| Layer | Feature | Status |
|---|---|---|
| 1 | Core create + list | ✅ Complete |
| 2 | Status flow | ✅ Complete |
| 3 | Priority | ✅ Complete |
| 4 | Labels | ✅ Complete |
| 5 | Compound filtering | ✅ Complete |
| 6 | Description, show, delete | ✅ Complete |
| 7 | Polish (color, `--help`) | ✅ Complete |

### [VSDD Suite](./vsdd-suite) — Methodology project

The prompt and process library this portfolio's projects use. Contains session primers per VSDD phase (1a+1b through 6), adversarial review prompts per role/meta domain (16 domains: 7 core + 9 extended), language and interface supplements (Rust, JS/TS, CLI, browser-app), pre-commit hooks (review-log anonymization, crosslink-reference validation, changelog currency, suite-review preamble discipline), project-scaffolding templates, and contributor governance materials.

**Methodology:** the suite is itself a software artifact and gets reviewed adversarially with the same discipline projects receive — 73 suite reviews logged at this writing, governed by the contributor primer at [`vsdd-suite/suite-development/`](./vsdd-suite/suite-development). See [`vsdd-suite/README.md`](./vsdd-suite/README.md) for usage; [`vsdd-suite/suite-development/README.md`](./vsdd-suite/suite-development/README.md) for evolving the suite itself.

| Component | Status |
|---|---|
| Phase 1a+1b through 6 primers (the full VSDD pipeline) | ✅ Complete |
| 7 core role domains + VDD-IAR Alignment meta | ✅ Complete |
| 9 extended domains (Accessibility, Privacy, Performance Engineer, Platform Engineer, Data Engineer, Technical Writer, Localization, Red Team, Portfolio Assessment) | ✅ Complete |
| Language and interface supplements (Rust, JS/TS, CLI, browser-app) | ✅ Complete |
| Crosslink integration + manual-mode parity (G-144 two-mode design principle) | ✅ Complete |
| 4 pre-commit hooks enforcing suite discipline | ✅ Complete |

### [VSDD Suite reference examples](./vsdd-suite-reference-examples) — Worked-example projects

Reference implementations for the [VSDD Suite](./vsdd-suite). One minimal CLI bookmark capture project (`bm add <url>`, `bm list`) realized in two parallel variants — one per [operational mode](./vsdd-suite/README.md#two-modes-of-operation-design-principle) the suite supports. Both are intentionally small (Layer 1 only) and exist to validate the suite's documented workflow end-to-end against the actual conventions it teaches.

- **[`bookmark-cli-manual/`](./vsdd-suite-reference-examples/bookmark-cli-manual)** — reference for the manual method (TODO.md + per-domain review logs + maintained markdown registries; no crosslink). Closes G-112 in the suite's findings registry.
- **[`bookmark-cli-crosslink/`](./vsdd-suite-reference-examples/bookmark-cli-crosslink)** — reference for the crosslink method (epic + layer issues + AC sub-issues; milestones per layer; `crosslink swarm review` for Phase 3; `crosslink issue relate` for Phase 4). Closes G-106 in the suite's findings registry.

A reader who wants to see what a project structured per the current suite actually looks like in either mode should start in the corresponding sub-folder.

**Methodology (both variants):** VSDD via the [VSDD Suite](./vsdd-suite) — scaffolded with `vsdd-suite/templates/scaffold-project.sh`; one layer realized through all 6 VSDD phases (1a+1b spec, 1c decomposition, 2a Red Gate, 2b implementation, 2c refactor / annotated skip, 3 adversarial review with the full active-domain set, 4 feedback integration routing, 5 formal hardening, 6 four-dimensional convergence) at **capstone intent**. Demonstrates the per-domain index + per-session-file review-log structure (G-89), the project-level findings index (G-138), the per-layer manual-test file convention (Review 74), the post-G-177 Phase 5 / Phase 6 per-domain-log pattern, and the AI-co-authored PROCESS.md retrospective discipline.

## Forward-only compatibility

`bookmark-manager/` and `issue-tracker-cli/` were built against the suite's earlier `iterative-adversarial-refinement/` shape and retain their inner subdirectories under that legacy name per the policy in [`vsdd-suite/COMPATIBILITY.md`](./vsdd-suite/COMPATIBILITY.md). The [`vsdd-suite-reference-examples/`](./vsdd-suite-reference-examples) projects are built against the current `vsdd-suite/` shape and stay current with each convention shift as part of being the worked example; the suite itself follows its own forward-only narrative-preservation policy (G-89) so historical review-log entries, CHANGELOG entries, and `G-XX`-ID anchors remain valid as audit-trail records throughout the portfolio.
