# Guild Portfolio

Apprentice portfolio for the Navigators Guild. Contains projects built to demonstrate software design, test-driven development, and engineering process.

## Projects

### [Bookmark Manager](./bookmark-manager)

A personal tool for saving and organizing web links with titles, notes, and tags. Built with [TypeScript](https://www.typescriptlang.org/), HTML, and CSS — no frameworks. Data lives in the browser's localStorage.

**Methodology:** design-first, TDD, Iterative Adversarial Refinement (IAR — [QA](./vsdd-suite/domains/role/QUALITY-ENGINEER-REVIEW.md), [UX](./vsdd-suite/domains/role/UX-REVIEW.md), [Security](./vsdd-suite/domains/role/SECURITY-REVIEW.md), [Platform Engineering](./vsdd-suite/domains/role/PLATFORM-ENGINEER-REVIEW.md), and [Solution Architect](./vsdd-suite/domains/role/SOLUTION-ARCHITECT-REVIEW.md) review domains), and manual testing at each layer before advancing.

| Layer | Feature | Status |
|---|---|---|
| 1 | Core (add, display, persist, validate) | ✅ Complete |
| 2 | Notes and tags | ✅ Complete |
| 3 | Edit and delete | ✅ Complete |
| 4 | Tag filtering | ✅ Complete |
| 5 | Search | ✅ Complete |
| 6 | Polish | ✅ Complete |

### [Issue Tracker CLI](./issue-tracker-cli)

A personal issue tracker for the terminal. Single user, no network, no accounts. Issues stored in a local JSON file. Built in [Rust](https://www.rust-lang.org/) with a git-style subcommand interface (`tracker create`, `tracker list`, and more across 7 planned layers).

**Methodology:** VSDD ([Verified Spec-Driven Development](https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00) by [dollspace.gay](https://github.com/dollspace-gay), building on [VDD](https://gist.github.com/dollspace-gay/45c95ebfb5a3a3bae84d8bebd662cc25)) — full behavioral specification before any code, [Red Gate](./vsdd-suite/primers/2a-red-gate.md) (tests written and confirmed failing before implementation), and a 10-domain + 2-meta Iterative Adversarial Refinement suite run at each layer gate.

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

The prompt and process library this portfolio's projects use. Contains [session primers per VSDD phase](./vsdd-suite/primers) (1a+1b through 6), [adversarial review prompts per role/meta domain](./vsdd-suite/domains) (16 domains: 7 core + 9 extended), [language and interface supplements](./vsdd-suite/supplements) (Rust, JS/TS, CLI, browser-app), [pre-commit hooks](./vsdd-suite/hooks) (review-log anonymization, [crosslink](https://github.com/forecast-bio/crosslink)-reference validation, changelog currency, suite-review preamble discipline), [project-scaffolding templates](./vsdd-suite/templates), and [contributor governance materials](./vsdd-suite/suite-development).

**Methodology:** the suite is itself a software artifact and gets reviewed adversarially with the same discipline projects receive — 79 suite reviews logged at this writing, governed by [the contributor primer](./vsdd-suite/suite-development/suite-development.md). See [`vsdd-suite/README.md`](./vsdd-suite/README.md) for usage; [`vsdd-suite/suite-development/README.md`](./vsdd-suite/suite-development/README.md) for evolving the suite itself.

| Component | Status |
|---|---|
| [Phase 1a+1b](./vsdd-suite/primers/1ab-spec-development.md) through [Phase 6](./vsdd-suite/primers/6-convergence.md) primers (the full VSDD pipeline) | ✅ Complete |
| 7 core role domains + [VDD-IAR Alignment](./vsdd-suite/domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md) meta | ✅ Complete |
| 9 extended domains ([Accessibility](./vsdd-suite/domains/role/ACCESSIBILITY-REVIEW.md), [Privacy](./vsdd-suite/domains/role/PRIVACY-REVIEW.md), [Performance Engineer](./vsdd-suite/domains/role/PERFORMANCE-ENGINEER-REVIEW.md), [Platform Engineer](./vsdd-suite/domains/role/PLATFORM-ENGINEER-REVIEW.md), [Data Engineer](./vsdd-suite/domains/role/DATA-ENGINEER-REVIEW.md), [Technical Writer](./vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md), [Localization](./vsdd-suite/domains/role/LOCALIZATION-REVIEW.md), [Red Team](./vsdd-suite/domains/role/RED-TEAM-REVIEW.md), [Portfolio Assessment](./vsdd-suite/domains/meta/PORTFOLIO-ASSESSMENT-REVIEW.md)) | ✅ Complete |
| [Language and interface supplements](./vsdd-suite/supplements) (Rust, JS/TS, CLI, browser-app) | ✅ Complete |
| [Crosslink](https://github.com/forecast-bio/crosslink) integration + manual-mode parity ([G-144](./vsdd-suite/suite-development/FINDINGS-INDEX.md#g-144) two-mode design principle) | ✅ Complete |
| 4 [pre-commit hooks](./vsdd-suite/hooks) enforcing suite discipline | ✅ Complete |

### [VSDD Suite reference examples](./vsdd-suite-reference-examples) — Worked-example projects

Reference implementations for the [VSDD Suite](./vsdd-suite). One minimal CLI bookmark capture project (`bm add <url>`, `bm list`) realized in two parallel variants — one per [operational mode](./vsdd-suite/README.md#two-modes-of-operation-design-principle) the suite supports. Both are intentionally small (Layer 1 only) and exist to validate the suite's documented workflow end-to-end against the actual conventions it teaches.

- **[`bookmark-cli-manual/`](./vsdd-suite-reference-examples/bookmark-cli-manual)** — reference for the manual method ([TODO.md](./vsdd-suite-reference-examples/bookmark-cli-manual/TODO.md) + per-domain review logs + maintained markdown registries; no [crosslink](https://github.com/forecast-bio/crosslink)). Closes [G-112](./vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112) in [the suite's findings registry](./vsdd-suite/suite-development/FINDINGS-INDEX.md).
- **[`bookmark-cli-crosslink/`](./vsdd-suite-reference-examples/bookmark-cli-crosslink)** — reference for the crosslink method (epic + layer issues + AC sub-issues; milestones per layer; `crosslink swarm review` for [Phase 3](./vsdd-suite/primers/3-review-session.md); `crosslink issue relate` for [Phase 4](./vsdd-suite/primers/4-feedback-integration.md)). Closes [G-106](./vsdd-suite/suite-development/FINDINGS-INDEX.md#g-106) in [the suite's findings registry](./vsdd-suite/suite-development/FINDINGS-INDEX.md).

A reader who wants to see what a project structured per the current suite actually looks like in either mode should start in the corresponding sub-folder.

**Methodology (both variants):** VSDD via the [VSDD Suite](./vsdd-suite) — scaffolded with [`vsdd-suite/templates/scaffold-project.sh`](./vsdd-suite/templates/scaffold-project.sh); one layer realized through all 6 VSDD phases ([1a+1b spec](./vsdd-suite/primers/1ab-spec-development.md), [1c decomposition](./vsdd-suite/primers/1c-decomposition.md), [2a Red Gate](./vsdd-suite/primers/2a-red-gate.md), [2b minimal implementation](./vsdd-suite/primers/2b-implementation.md), [2c refactor / annotated skip](./vsdd-suite/primers/2c-refactor.md), [3 adversarial refinement](./vsdd-suite/primers/3-review-session.md) with the full active-domain set, [4 feedback integration loop](./vsdd-suite/primers/4-feedback-integration.md), [5 formal hardening](./vsdd-suite/primers/5-formal-hardening.md), [6 convergence](./vsdd-suite/primers/6-convergence.md)) at **capstone intent**. Demonstrates the per-domain index + per-session-file review-log structure ([G-89](./vsdd-suite/suite-development/FINDINGS-INDEX.md#g-89)), the project-level findings index ([G-138](./vsdd-suite/suite-development/FINDINGS-INDEX.md#g-138)), the per-layer manual-test file convention ([Review 74](./vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-74--2026-05-20-1230z)), the post-[G-177](./vsdd-suite/suite-development/FINDINGS-INDEX.md#g-177) Phase 5 / Phase 6 per-domain-log pattern, and the AI-co-authored PROCESS.md retrospective discipline.

## Forward-only compatibility

`bookmark-manager/` and `issue-tracker-cli/` were built against the suite's earlier `iterative-adversarial-refinement/` shape and retain their inner subdirectories under that legacy name per the policy in [`vsdd-suite/COMPATIBILITY.md`](./vsdd-suite/COMPATIBILITY.md). The [`vsdd-suite-reference-examples/`](./vsdd-suite-reference-examples) projects are built against the current `vsdd-suite/` shape and stay current with each convention shift as part of being the worked example; the suite itself follows its own forward-only narrative-preservation policy ([G-89](./vsdd-suite/suite-development/FINDINGS-INDEX.md#g-89)) so historical review-log entries, CHANGELOG entries, and `G-XX`-ID anchors remain valid as audit-trail records throughout the portfolio.
