# Changelog

All notable changes to the AIR suite are recorded here. Entries are in reverse chronological order. Timestamps are UTC (Zulu).

---

## Unreleased — 2026-04-26 (session 3)

### Added
- **`VDD-IAR-ALIGNMENT-REVIEW.md`** — New domain. Evaluates whether the VDD-IAR methodology was actually followed. The other domains evaluate what was built; this domain evaluates how. Reference document is the governing methodology doc (`apprentice-onboarding/02-the-methodology/01-how-we-build.md` for guild projects). Ten dimensions: design-before-code, layered decomposition, layer gate compliance, test discipline, human verification, IAR fresh context, IAR iteration, role integrity, manual testing checklists, retrospective quality. The sycophancy check is specifically scoped to the rationalization risk: the reviewing agent participated in building the project and has every incentive to find the process acceptable.

### Changed
- **README.md** — Major restructure to reflect VDD-IAR as the governing framework:
  - Opening now names AIR as the adversarial mechanism of VDD, describes the full loop (design → build → verify → adversarial refinement → fix → repeat until MVR), and states explicitly that AIR is not a pre-merge checkpoint but an active part of the build cycle
  - **Refinement loop** replaces "Full run" section: describes within-layer iteration (first pass → fix → second pass → repeat until MVR), requires round numbers in logs
  - **Session isolation** moved to its own paragraph with clearer framing
  - **Generalist adversary pass** added as an optional step: unstructured, no domain framework, finds what specialists missed; lives as a README note rather than a formal domain
  - Domain table updated with VDD-IAR Alignment; focus descriptions reformatted consistently
  - Sequencing updated: VDD-IAR Alignment runs last (reviews process artifacts produced by all other runs)
  - Merging gate updated: requires MVR (not just one passing run), adds VDD-IAR Alignment as a required gate, adds round numbers to log format
- **SOLUTION-OWNER-REVIEW.md** — Removed dims 9 (complexity budget for one → SA), 11 (VDD process fidelity → VDD-IAR Alignment), 12 (linear accountability → VDD-IAR Alignment). Assignment compliance renumbered to dim 9. SO returns to its original identity: the spec contract.
- **SOLUTION-ARCHITECT-REVIEW.md** — Dim 9 (complexity budget) expanded to include maintainer-scale complexity. Now covers both problem-proportionate complexity and team-proportionate complexity. Cross-references SO dim 4 (over-engineering) to distinguish the two concerns.
- **QUALITY-ENGINEERING-REVIEW.md** — Removed dim 14 (manual testing checklists → VDD-IAR Alignment). Added domain boundary statement: QE owns the test system; SE owns the bugs. When QE finds a logic error with no test, flag the missing test here; SE flags the bug.
- **SOFTWARE-ENGINEERING-REVIEW.md** — Added domain boundary statement: SE owns the implementation; QE owns the test system. SE flags bugs; QE flags missing tests. Do not duplicate by evaluating test architecture in SE.
- **PLATFORM-ENGINEERING-REVIEW.md** — Replaced generic sycophancy check with a posture note acknowledging that most PE dimensions are compliance checks, not adversarial judgment calls. Sycophancy risk is specifically scoped to inapplicability decisions and threshold acceptance. This is more honest about what PE actually does.

---

## Unreleased — 2026-04-26 (session 2)

### Added
- **`hallucinated` finding classification** — Added to all 8 domain prompts. A finding is hallucinated when the adversary invented a problem that does not exist and push back is warranted. Consistent hallucinated findings are the maximum viable refinement signal: real issues have been exhausted. Security uses "invented a vulnerability"; SO uses "invented a scope deviation or compliance failure" to match each domain's adversarial framing.
- **Solution Owner dim 10 — Assignment compliance** — New dimension. Checks whether DESIGN.md itself accurately reflects the upstream assignment brief, not just whether the implementation matches DESIGN.md. Scope creep that enters at the design stage will pass every other SO dimension and still fail an external review. Requires reading the assignment instructions alongside DESIGN.md.
- **Solution Owner dim 11 — VDD process fidelity** — New dimension. Checks whether the VDD loop was actually followed: DESIGN.md before code, layered commit history, layer gates completed before advancing, tests written alongside or before implementation, AIR run before each merge. A correct product built without process discipline is not evidence of the capability the process is designed to develop.
- **Solution Owner dim 12 — Linear accountability** — New dimension. Can every piece of code be traced to a specific task, issue, or requirement? Are commit messages specific enough to identify what was built and why? Evaluates the string-of-beads principle: every change should have a bead, and every bead should be accounted for.
- **Quality Engineering dim 14 — Manual testing checklists** — New dimension. Does the project have a manual testing checklist per layer or feature? Was it completed before the layer was marked done? Automated tests verify correctness; manual testing catches intent mismatches, UX problems, and "technically correct but not what I meant" failures. Absence of a manual checklist is a quality gap regardless of coverage.
- **README — Session isolation** — Operational note under "Full run": resetting the AI session between domain reviews gives each domain the same adversarial intensity. An agent that reviews all 8 domains in one session accumulates context that softens its pressure. Parallel sessions are the gold standard.
- **README — Maximum viable refinement** — Explanation of the MVR exit signal in the context of the `hallucinated` classification. When an adversarial domain produces only hallucinated findings, real issues have been exhausted. Log the final round with push back reasoning so the record shows how the exit signal was reached.
- **README — Portfolio-arc review** — New section under "Suggesting new domains". Describes a cross-project pass to be run before portfolio submission, evaluating: growth across projects, honest retrospectives, assignment alignment patterns, independence evidence, and process over product.

---

## Unreleased — 2026-04-26

### Added
- **`lang/` subfolder** — Language and interface type supplements. Domain files reference these during review; reviewers apply the relevant supplement's section alongside the standard dimensions for that domain.
  - `lang/rust.md` — Rust-specific dimensions for QE (doc tests, clippy, integration tests against binary), Security (`.unwrap()` discipline, `cargo audit`, unsafe rationale), SE (error propagation, error type hierarchy, clippy as idiom proxy), PE (`cargo audit`, `cargo clippy --deny warnings`, `cargo fmt --check`, `Cargo.lock` for binaries, toolchain pinning), DE (`serde` boundary validation, `#[serde(default)]` for schema evolution), SA (CLI parsing separation, command enum dispatch, `lib.rs`/`main.rs` split)
  - `lang/javascript-typescript.md` — JS/TS-specific dimensions for QE (`npm ci`, axe scanning, browser tests, type coverage), Security (rendering safety, URL injection, `JSON.parse` runtime validation, CSP, `npm audit`), SE (`as` casts require runtime validation, `any` types, non-null assertions, unhandled promise rejections), PE (`npm ci`, `package-lock.json`, `npm audit`, Node pinning, `tsc --noEmit`), DE (runtime schema validation, `JSON.parse` error handling, normalization, date handling)
  - `lang/cli.md` — CLI interface type supplement. Replaces browser-centric UX dimensions with 11 CLI UX dimensions (command discoverability, stdout/stderr discipline, exit codes, empty state messages, destructive confirmation, machine-readable output, verbose/quiet modes, error message quality, interruption handling). Adds CLI-specific QE dimensions (integration tests invoke binary, full stdout/stderr/exit code assertions) and SE dimensions (output formatting as a code concern, structured result types before formatting).
  - `lang/browser-app.md` — Browser interface type supplement with QE (axe scanning, browser compat, responsive testing, keyboard navigation), Security (rendering safety, URL injection, CSP, storage validation, SRI), and UX (accessibility, responsive design, browser compatibility, reduced motion, native dialog quality) dimensions.
- **Sycophancy check** — Added to all 8 domain prompts. Explicitly names AI self-validation as a failure mode: if the reviewing agent agrees with every decision without challenge, that agreement itself is a finding.
- **Solution Owner dim 9 — Complexity budget for one** — New dimension evaluating whether architectural complexity is proportionate to the maintenance team size. An AI agent defaults to team-scale practices regardless of the project's actual maintenance model. Distinct from over-engineering (which flags complexity beyond spec); this flags complexity that is proportionate to spec but disproportionate to the team.
- **Solution Owner — `approved deviation` classification** — New finding classification for deviations from DESIGN.md that were explicitly approved by the stakeholder prior to implementation. Requires documentation of the approval and rationale.
- **Solution Architect dim 11 — Session continuity** — New dimension: are architectural decisions and rationale documented in a form a new AI session can act on without rediscovering them? Decisions that live only in conversation history are invisible to future sessions.
- **Software Engineering dim 11 — Future-self maintainability** — New dimension: will you be able to understand and modify this code in six months without the original AI session? Are key decisions derivable from the code and its comments?
- **GAP-ANALYSIS-LOG.md Run 2** — 2026-04-25 21:30Z. Context: AI-accelerated consulting team. Identified 15 new gaps (G-18–G-32) including: Requirements and Business Analysis domain, Documentation Fidelity domain, AI assumption surfacing, hallucination detection, context drift checking, dependency/API existence validation, test gaming detection, AI-generated code anti-patterns, Change Management, Knowledge Transfer, Client/Stakeholder Alignment, integration architecture.
- **GAP-ANALYSIS-LOG.md Run 3** — 2026-04-25 22:00Z. Context: personal developer using AI-accelerated tools, portfolio-to-side-business trajectory. Identified 6 new gaps (G-33–G-38) including: sycophancy detection (G-33, addressed), future-maintainability-for-one assessment (G-35, addressed), session continuity across AI conversations (G-37, addressed), complexity trap from AI over-engineering (G-38, addressed).

### Changed
- **QUALITY-ENGINEERING-REVIEW.md** — Removed browser-specific dimensions 11–13 (accessibility, browser compatibility, responsive design) from standard dimensions; these are now in `lang/browser-app.md`. Generalized dim 14 (security surface) to remove npm-specific language. Renumbered to 13 dimensions. Added language and interface supplement instruction.
- **SECURITY-REVIEW.md** — Removed web-specific dimensions 1 (rendering safety), 2 (URL injection), and 5 (CSP) from standard dimensions; these are now in `lang/browser-app.md` and `lang/javascript-typescript.md`. Generalized remaining dimensions to be language-agnostic. Added dim 4 (secret handling) and dim 6 (authentication/authorization) as generic security dimensions. Renumbered to 6 dimensions. Added language and interface supplement instruction.
- **UX-REVIEW.md** — Added interface-type note: standard dimensions assume a browser-rendered interface; CLI projects should consult `lang/cli.md`; browser apps should also consult `lang/browser-app.md`.
- **PLATFORM-ENGINEERING-REVIEW.md** — Generalized npm-specific language in dims 1, 3, 4, and 11 to be ecosystem-agnostic with ecosystem-appropriate examples. Added language and interface supplement instruction.
- **SOLUTION-ARCHITECT-REVIEW.md** — Added language and interface supplement instruction.
- **SOLUTION-OWNER-REVIEW.md** — Added language and interface supplement instruction (SO review is primarily spec-driven; supplement used to verify technology choices against the spec).
- **SOFTWARE-ENGINEERING-REVIEW.md** — Added language and interface supplement instruction.
- **DATA-ENGINEERING-REVIEW.md** — Added language and interface supplement instruction.
- **GAP-ANALYSIS-LOG.md** — Fixed blank line between G-17 and G-18 rows that broke markdown table rendering. Updated gap registry statuses: G-33, G-35, G-37, G-38 marked Addressed.

---

## 2026-04-26 00:15Z — `db45cd2`

### Added
- **GAP-ANALYSIS-LOG.md** — New living document for gap analysis runs against the AIR suite itself. Includes re-run trigger conditions, instructions, and a gap registry table. Initial run (Run 1, 2026-04-25 20:00Z) evaluated against mission-critical and speculative project contexts. Identified 17 gaps (G-01–G-17) across 5 missing domains and 12 dimension-level gaps. Per-context severity (Mission-Critical / Speculative) recorded for each gap.

---

## 2026-04-25 23:56Z — `59ee04e`

### Added
- **PE dim 10 — Pre-commit hooks** — Platform Engineering now owns pre-commit hooks as a DevSecOps control. Hooks cover: secret and credential detection (API keys, tokens, private keys, connection strings); PII detection (email addresses, phone numbers, government IDs); committer identity and local machine leakage (absolute paths with usernames, hostnames, local environment details in configs or build output); large or binary files. Includes evaluation of `--no-verify` bypass risk.

### Changed
- **SECURITY-REVIEW.md** — Added coordination note: Security flags sensitive data patterns it identifies to Platform Engineering for incorporation into pre-commit hook detection rules.

---

## 2026-04-25 23:51Z — `0bef3f6`

### Changed
- **PLATFORM-ENGINEERING-REVIEW.md** — Massively expanded from CI/CD-only to full delivery platform ownership across four areas:
  - **CI/CD** (dims 1–9): pipeline completeness, gate enforcement, dependency installation, environment pinning, cache correctness, coverage thresholds, action/dependency pinning, artifact hygiene, left-shift opportunities
  - **DevSecOps** (dims 10–15): security scanning, secret management, supply chain integrity, least privilege, compliance gates
  - **Infrastructure** (dims 16–21): Infrastructure as Code, cloud/on-premise resource hygiene, containerization, container security, environment parity, disaster recovery
  - **Observability** (dims 22–26): logging, metrics, alerting, distributed tracing, dashboards
  - Inapplicable sections may be skipped with rationale. A static single-user tool has no cloud infrastructure to evaluate.

---

## 2026-04-25 23:40Z — `2b6446a`

### Added
- **SOFTWARE-ENGINEERING-REVIEW.md** — New domain. Evaluates implementation quality at the code level: correctness, error handling, naming, function design, duplication, complexity, type safety, defensive coding, comments and self-documentation, consistency. Distinct from Solution Architect (which evaluates structure and boundaries) and Quality Engineering (which evaluates the test system). 10 standard dimensions.
- **DATA-ENGINEERING-REVIEW.md** — New domain. Evaluates the data layer: data model correctness, validation and normalization, schema evolution, data integrity invariants, storage fitness, access patterns, serialization, data consistency, sensitive data handling, test coverage of data paths. Marked optional for projects without a meaningful data layer.

### Changed
- **QA-REVIEW.md → QUALITY-ENGINEERING-REVIEW.md** — Renamed via `git mv`. Scope broadened from bug-finding to test architecture and quality system: added test falsifiability (dim 2, "a test that cannot fail on a defective implementation has no value"), coverage meaningfulness (dim 4), test architecture and independence (dim 5), and quality gates (dim 16).
- **All domain prompts** — Added DESIGN.md as required first read for all domain reviews. All domains now treat DESIGN.md as authoritative context for the project's scope, constraints, and feature set.
- **All cross-domain coordination links** — Updated from `QA-REVIEW.md` to `QUALITY-ENGINEERING-REVIEW.md`.
- **README.md** — Added Software Engineering and Data Engineering to domain table. Updated domain count and descriptions. Added note that not all domains are required for all projects. Updated sequencing (run DE before SA when data model changes are significant). Updated merging gate and log structure to reflect 8 domains.

---

## 2026-04-25 23:03Z — `6ea9b30`

### Added
Initial AIR suite. Six review domains extracted from the bookmark-manager project, generalized into a reusable template.

- **README.md** — Suite index: domain table, running instructions (full run, scoped run, sequencing), candidate domains, review log structure, merging gate.
- **QA-REVIEW.md** — Quality assurance: acceptance criteria, test coverage, validation gaps, logic errors, dead code, unused dependencies, dependency versions, accessibility, browser compatibility, responsive design, security surface.
- **UX-REVIEW.md** — User experience: empty states, error messages, focus and keyboard behavior, visual consistency, affordances, feedback patterns, accessibility, responsive design, browser compatibility, long content, reduced motion, native dialog quality.
- **SECURITY-REVIEW.md** — Security: rendering safety, URL injection, storage data validation, dependency CVEs, CSP, information exposure, input handling.
- **PLATFORM-ENGINEERING-REVIEW.md** — CI/CD pipeline and gate enforcement.
- **SOLUTION-ARCHITECT-REVIEW.md** — Architecture: separation of concerns, coupling, data model integrity, interface contracts, state management, immutability, extensibility, technology fitness, complexity budget, decision documentation.
- **SOLUTION-OWNER-REVIEW.md** — Scope and delivery: spec coverage, scope creep, technology compliance, over-engineering, under-delivery, design fidelity, backlog candidates, prior-review additions. Opens every review with a compliance table (Met/Partial/Missing). DESIGN.md treated as a Scope of Work contract. "Quality does not justify scope."

Review logs are stored outside the prompt files. Logs live at `{project}/iterative-adversarial-refinement/` inside each reviewed project.
