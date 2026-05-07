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
| 5 | Compound filtering | 🟡 In review (PR #17) |
| 6 | Description, show, delete | 🔲 Not started |
| 7 | Polish (color, `--help`) | 🔲 Not started |
