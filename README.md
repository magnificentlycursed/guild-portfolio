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

### [Bookmark CLI](./bookmark-cli) — Reference implementation

A minimal CLI bookmark capture tool (`bm add <url>`, `bm list`) built as the **reference implementation** for the [VSDD Suite](./vsdd-suite)'s worked example. It is intentionally small — Layer 1 only — and exists to validate the suite's documented workflow end-to-end (closes G-112 in the suite's gap registry). A reader who wants to see what a project structured per the current suite actually looks like should start here.

**Methodology:** VSDD via the [VSDD Suite](./vsdd-suite) — scaffolded with `vsdd-suite/templates/scaffold-project.sh`; one layer through Phases 1a → 1b → 2a → 2b → 3 (QE first pass); demonstrates the per-domain index + per-session-file review-log structure (G-89 forward-only convention).

| Layer | Feature | Status |
|---|---|---|
| 1 | Add + list | ✅ Complete (8/8 tests pass; QE Review 1 filed) |
| 2 | Tag + filter | Scoped only (not in scope for the reference implementation) |
| 3 | Export + import | Scoped only (not in scope for the reference implementation) |

## The suite

The [VSDD Suite](./vsdd-suite) is the prompt and process library the projects above use. It contains session primers per VSDD phase, adversarial review prompts per role/meta domain, language and interface supplements, and contributor governance materials. See [`vsdd-suite/README.md`](./vsdd-suite/README.md) for usage; [`vsdd-suite/suite-development/README.md`](./vsdd-suite/suite-development/README.md) for evolving the suite itself.

**Forward-only compatibility:** `bookmark-manager/` and `issue-tracker-cli/` were built against the suite's earlier `iterative-adversarial-refinement/` shape and retain their inner subdirectories under that legacy name per the policy in [`vsdd-suite/COMPATIBILITY.md`](./vsdd-suite/COMPATIBILITY.md). `bookmark-cli/` is built against the current `vsdd-suite/` shape.
