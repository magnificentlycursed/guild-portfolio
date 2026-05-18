# VSDD Suite

A multi-phase prompt and process library for the Verified Spec-Driven Development (VSDD) methodology. Provides session primers for each VSDD phase, adversarial review prompts for Phase 3 (Iterative Adversarial Refinement — IAR), language and interface supplements, and the operational discipline for running them.

The suite's Phase 3 component (IAR) is not a pre-merge checkpoint — it is an active part of the build cycle. Rounds run during layer development, not just at the end. A layer does not merge when it passes one IAR run; it merges when an IAR run produces only **hallucinated** findings across all active domains. That is the maximum viable refinement (MVR) signal: the adversary has run out of real complaints.

## Two modes of operation (design principle)

The suite supports two modes for operations: **crosslink-primary** (recommended) and **manual** (first-class supported fallback).

- **Crosslink-primary mode** uses [crosslink](https://github.com/forecast-bio/crosslink) to mechanize dispatch (`crosslink swarm review`, `crosslink swarm fix`), finding tracking (labeled crosslink issues), session handoff (`crosslink session end --notes`), cross-domain coordination (`crosslink issue relate`/`block`), and optionally primer auto-injection via `crosslink knowledge`. This is the recommended path because the mechanisms compound (queryable finding labels; structured issue graph; built-in audit trail; multi-agent worktree isolation by construction).
- **Manual mode** is the first-class fallback — every feature of the crosslink-primary mode has a documented manual equivalent at the same depth: per-domain index files + `review-log/YYYY-MM-DD-<slug>.md` per-session files for finding tracking; `FINDINGS-INDEX.md` for the cross-cutting registry; prose `**Coordination:**` and `**Session note:**` markers in review-log entries; manual chat-session ritual for dispatch. Manual mode is not a stripped-down subset; it is tested and documented to feature parity with the crosslink mode.

Cross-mode migration is supported via `crosslink import` / `crosslink export` (the labeled-issue and markdown finding-index shapes mirror each other deliberately for this purpose).

**Scaffolding is a separate decision** — per G-117 doctrine ratification, the canonical scaffolding mechanism is manual copy via [`templates/scaffold-project.sh`](templates/scaffold-project.sh) regardless of which operational mode the project uses. See [Bringing the suite into your project](#bringing-the-suite-into-your-project).

> **New here?** Read [Prerequisites](#prerequisites) → [Quickstart](#quickstart) (two parallel quickstarts — one per mode) → [Bringing the suite into your project](#bringing-the-suite-into-your-project) → [Worked example](#worked-example-a-vsdd-session-with-crosslink). For evolving the suite itself, see [`suite-development/README.md`](suite-development/README.md).

**Tested against:** crosslink v0.8.0 (2026-05-17) — when crosslink commands appear, they are verified against this version. Manual mode is crosslink-independent.

## Prerequisites

The suite is documentation + prompts that you load into an AI tool. Both operational modes (crosslink-primary and manual) share the same baseline AI/toolchain prerequisites; crosslink is additionally required for the recommended crosslink-primary mode.

**Baseline (required for both modes):**
- **An AI tool capable of cold-context chat sessions.** Concrete options: [Claude Code](https://claude.com/claude-code) (terminal-native), [claude.ai](https://claude.ai) (browser), [Cursor](https://cursor.com) (IDE), [GitHub Copilot Chat](https://github.com/features/copilot) (VS Code / JetBrains / GitHub.com). "Cold context" means a session with no system prompt, prior file context, or persistent memory carried over from a previous session — the cold-context discipline is load-bearing for Phase 3 adversarial review. **How to start a fresh chat:** in Claude Code, `/clear`; in claude.ai, open a new chat; in Cursor, new project window or `Cmd+Shift+P → New Chat`; in GitHub Copilot Chat, click "New Chat" in the chat panel (VS Code: `Ctrl+L` / `Cmd+L`). The suite is methodology-neutral about tool choice — any tool that provides a fresh-context chat session with sufficient context window for the primer + domain prompt + project artifacts will work; the cross-model cross-check (see § Same-model review limitation) is the highest-stakes pressure point on tool choice.
- **Your project's language toolchain.** The worked example uses Rust (`cargo`), but the suite is language-agnostic. For JavaScript/TypeScript: Node.js + npm/pnpm/yarn. For Python: Python 3 + pytest. For other languages, the relevant supplement in [`supplements/`](supplements/) names the appropriate tooling.
- **Git.** Phase 2a's Red Gate commit boundary is the spine of the methodology.

**For crosslink-primary mode (recommended):**
- **[crosslink](https://github.com/forecast-bio/crosslink)** v0.8.0 or compatible — issue tracker that mechanizes the suite's per-layer discipline, per-domain dispatch, and finding routing. The full dependency surface (commands, flags, breaking-change definition) is documented at [`crosslink-contract.md`](crosslink-contract.md). Without crosslink, the suite runs in manual mode at full feature parity.

### Data-flow and privacy posture

Every primer-loaded session sends content from your project to the AI tool you chose:

- The primer text (publishable — no concern)
- The relevant domain prompt and supplement (publishable — no concern)
- **Your project source code, `DESIGN.md`, layer plan, prior review logs** (often not publishable; may contain credentials, customer data, internal architecture)
- **Your prompts and the model's responses** (may contain operational context, decisions, business logic)

The implications depend on the AI tool and the plan tier:

| Tool / plan | Default training-on-input behavior | Suite-use posture |
|---|---|---|
| Claude API (direct) | No training on input by default; explicit terms | Safe for sensitive code |
| Claude Code | Uses the Claude API; same posture | Safe for sensitive code |
| claude.ai (Pro / Team / Enterprise) | No training on conversations by default | Safe for sensitive code (verify your plan's terms) |
| claude.ai (Free) | May use conversations to improve models per current terms | Avoid for code containing real credentials, PII, or proprietary business logic |
| GitHub Copilot Business / Enterprise | Per GitHub's published terms, code suggestions and prompts are not used to train the underlying models | Safe for sensitive code (verify your plan's current terms before relying on this) |
| GitHub Copilot Individual | Training-on-prompts is opt-in/opt-out via account settings depending on plan tier and feature; defaults have shifted over time | Verify the account-level "Allow GitHub to use my code snippets for product improvements" setting (or current equivalent) before using on sensitive code |
| Cursor / other IDE integrations | Tool-specific; check the tool's data-handling terms | Verify per tool before use on sensitive code |

**Recommended posture:** for a personal-portfolio project with mock data, any of the above is appropriate. For any project containing real PII, customer data, regulated data (PHI/PCI), or proprietary business logic, **use a tool with explicit no-training-on-input guarantees and review the tool's data-handling terms before pasting code into a session.** The suite's Privacy and Security domains apply to your project; this section applies the same discipline to the suite's own runtime data-sharing — Privacy dim 6 (third-party data sharing) treats the AI tool as a third party.

## Quickstart

Two parallel quickstarts — one per [operational mode](#two-modes-of-operation-design-principle). Both run the same VSDD pipeline; the difference is mechanism. Step 1 (scaffolding) is identical for both modes per G-117 — `templates/scaffold-project.sh` is the canonical scaffolding mechanism regardless of operational mode.

### Quickstart — crosslink-primary (recommended)

1. **Scaffold.** `cd <your-project> && <path-to-vsdd-suite>/templates/scaffold-project.sh && crosslink init`
2. **Phase 1a.** `crosslink design "<one-sentence project description>"` opens the Phase 1a session container; paste [`primers/1a-spec-crystallization.md`](primers/1a-spec-crystallization.md) at session start; iterate the `.design/<slug>.md` draft; promote to `DESIGN.md`; commit.
3. **Phase 1b.** Fresh chat. Paste [`primers/1b-decomposition.md`](primers/1b-decomposition.md) + `DESIGN.md`. Build the layer hierarchy with `crosslink quick "<project>" -l epic`, one `crosslink milestone create` per layer, layer issues with `--parent <epic>`, acceptance criteria as sub-issues.
4. **Phase 2a → 2b.** `crosslink session start && crosslink session work <layer-id>`. Fresh chat with [`primers/2a-red-gate.md`](primers/2a-red-gate.md) — write failing tests, commit. Fresh chat with [`primers/2b-implementation.md`](primers/2b-implementation.md) — make tests pass. Phase 2b → 3 gate: `crosslink swarm gate <phase-slug>`.
5. **Phase 3.** `crosslink swarm review --agents <N> --mandate adversarial --file-issues --doc vsdd-suite/<DOMAIN>-REVIEW.md` for routine refinement rounds (N parallel cold-context adversaries; findings filed with `review-finding` label). Use manual dispatch (see manual quickstart below) when approaching MVR. Add structured G-138 labels during classification (`domain:<slug>`, `layer:N`, `round:N`, `classification:<class>`, `source:<source>`); `crosslink issue comment <id> "<rationale>" --kind <kind>` then `crosslink issue close <id>`.
6. **Phase 4.** Fresh chat with [`primers/4-feedback-integration.md`](primers/4-feedback-integration.md). Apply route labels (`crosslink issue label <id> route:phase-<N>`); `crosslink issue block <future-layer-id> <route-finding-id>` for cross-layer dependencies; `crosslink swarm fix --from-label route:phase-2b --budget-aware` for the safely-parallelizable cohort. Loop until MVR.

### Quickstart — manual (first-class fallback, same VSDD pipeline)

1. **Scaffold.** `cd <your-project> && <path-to-vsdd-suite>/templates/scaffold-project.sh` — creates `vsdd-suite/` directory, per-domain index files, `FINDINGS-INDEX.md`, `DESIGN.md` skeleton, project `README.md`.
2. **Phase 1a.** Fresh chat. Paste [`primers/1a-spec-crystallization.md`](primers/1a-spec-crystallization.md). Write `DESIGN.md`. Commit.
3. **Phase 1b.** Fresh chat. Paste [`primers/1b-decomposition.md`](primers/1b-decomposition.md) + `DESIGN.md`. Write `TODO.md` per the primer's `## TODO.md format` section.
4. **Phase 2a → 2b.** Fresh chat per phase. Paste [`primers/2a-red-gate.md`](primers/2a-red-gate.md) → write failing tests → `git commit` the Red Gate boundary → paste [`primers/2b-implementation.md`](primers/2b-implementation.md) → make tests pass → run the test suite to verify clean (your language's runner: `cargo test`, `npm test`, `pytest`, etc.).
5. **Phase 3.** *One fresh chat per active domain* (cold context per domain is the gold standard). **Default activation is the 7 core domains** (SE, QE, UX, Security, SA, SO, VDD-IAR Alignment); the scaffold script populates index files for these. Extended domains activate per [`domains/DOMAIN-INDEX.md`](domains/DOMAIN-INDEX.md). For each active domain, paste [`primers/3-review-session.md`](primers/3-review-session.md) + the domain prompt + the language supplement + the code under review. Classify findings. File rounds to per-domain index + per-session file `review-log/YYYY-MM-DD-<domain-slug>.md`. Append cross-cutting rows to `vsdd-suite/FINDINGS-INDEX.md` per the G-138 manual-path schema. Repeat for every active domain in its own fresh chat (no context sharing).
6. **Phase 4.** Fresh chat. Paste [`primers/4-feedback-integration.md`](primers/4-feedback-integration.md). Record routing decisions in each finding's review-log entry per the primer's `## Without crosslink` section. Re-enter routed phases manually (fresh chat per re-routed phase). Loop Phase 3 → 4 → re-enter routed phases until MVR.

That's the whole pipeline in both modes. Full walkthrough with starter prompts and per-phase `[crosslink]` / `[manual]` blocks below.

## Bringing the suite into your project

Each project under the suite gets its own `<your-project>/vsdd-suite/` directory containing the per-domain index files (created from templates) and a `review-log/` directory. The suite source (this repo's `vsdd-suite/`) is the *upstream*; each project gets a *copy* of the parts it uses.

**The canonical default: manual copy via `scaffold-project.sh`.** Run the scaffold script from your new project's root — it copies the templates and prints next-step guidance. This is the suite's recommended mechanism per Review 42's Solution-Owner ratification (G-117 closure): minimum-viable for portfolio scale; matches the suite's existing tech surface (markdown + bash); no new infrastructure to learn. The script lives at `<path-to-vsdd-suite>/templates/scaffold-project.sh` — see the next section for invocation.

**Accepted-variant options** (use one of these only if you have an explicit reason):

| Variant | When it fits | Trade-off |
|---|---|---|
| Git submodule the suite | You want a pinned version reference with visible drift detection across a team-scale portfolio | Adds `.gitmodules`; submodule semantics confuse new git users |
| Clone the suite as a sibling directory + symlink | You run many projects against the same local suite checkout | Local-only; doesn't survive pushing the project to a fresh clone elsewhere |

Crosslink and the suite are independent tools. Crosslink does not currently scaffold the suite into a project; the scaffold script above is the suite's own mechanism and works the same whether or not crosslink is installed.

**For first-time users, use the scaffold helper:**

```sh
cd <your-project>
<path-to-vsdd-suite>/templates/scaffold-project.sh
# Scaffolds the 7 default core domains (SE, QE, UX, Security, SA, SO, VDD-IAR-Alignment),
# copies DESIGN.md and project README skeletons, and prints next-step guidance.
# If your project activates DATA-ENGINEER, PLATFORM-ENGINEER, or any extended domain
# (RED-TEAM, PERFORMANCE-ENGINEER, TECHNICAL-WRITER, ACCESSIBILITY, PRIVACY, LOCALIZATION),
# pass them as additional arguments — see domains/DOMAIN-INDEX.md for activation criteria.

# Then customize the placeholders in each copied file per templates/README.md § Customization checklist.
```

**Manual equivalent (if you prefer not to use the script):**

```sh
cd <your-project>
mkdir -p vsdd-suite/review-log

# Copy the generic per-domain index template, renamed per domain (one cp per active domain).
# Example for a CLI project (7 core domains, no PE/DE/extended):
for domain in SOFTWARE-ENGINEER QUALITY-ENGINEER UX SECURITY SOLUTION-ARCHITECT SOLUTION-OWNER VDD-IAR-ALIGNMENT; do
  cp <path-to-vsdd-suite>/templates/DOMAIN-REVIEW-template.md vsdd-suite/${domain}-REVIEW.md
done

# Copy the DESIGN.md and project README skeletons
cp <path-to-vsdd-suite>/templates/DESIGN-template.md DESIGN.md
cp <path-to-vsdd-suite>/templates/PROJECT-README-template.md README.md
```

**The primers, domain prompts, and supplements are NOT copied into your project** — they are loaded into AI chat sessions from the suite repo when needed. Only the per-project artifacts (index files, review-log session files, DESIGN.md, project README) live in your project tree.

After scaffolding, **customize the placeholders** in each copied file per [`templates/README.md`](templates/README.md) § Customization checklist — `{{ROLE_TITLE}}`, `{{ROLE_VARIANTS}}`, `{{SYCOPHANCY_CHECK}}` etc. come verbatim from the corresponding domain prompt file in `domains/role/<DOMAIN>-REVIEW.md`.

## Suite scope

What lives here:

- **Adversarial review prompts** (`domains/`) — role and meta domains for VSDD Phase 3, with evaluation dimensions and finding classification schemas
- **Phase session primers** (`primers/`) — posture-setting prompts for VSDD Phases 1a, 1b, 2a, 2b, 3, and 4
- **Language and interface supplements** (`supplements/`) — language- and interface-specific dimensions composed with domain reviews
- **Suite-development materials** (`suite-development/`) — the contributor primer, gap registry, suite-review index, and review-log session entries for evolving the suite itself; see [`suite-development/README.md`](suite-development/README.md)

Known scope gaps: VSDD Phases 5 (Formal Hardening) and 6 (Four-Dimensional Convergence) are partially or wholly unowned (G-54, G-55). VSDD Phase 4 (Feedback Integration) gained a primer in this revision — see [`primers/4-feedback-integration.md`](primers/4-feedback-integration.md) — closing G-86.

## VSDD pipeline context

VSDD defines six phases. IAR owns Phase 3. Understanding the full pipeline matters because IAR evaluates *whether the prior phases were executed correctly*, not just whether the code is good.

| Phase | Name | What happens | Primer | IAR's role |
|---|---|---|---|---|
| 1a | Spec Crystallization | Design doc written with behavioral contracts, edge case catalog, interface definitions, verification architecture | [`1a-spec-crystallization.md`](primers/1a-spec-crystallization.md) | VDD-IAR Alignment dim 1 evaluates spec completeness |
| 1b | Decomposition | Project broken into layered TODO.md; Red Gate test plans written per layer; crosslink issue hierarchy created | [`1b-decomposition.md`](primers/1b-decomposition.md) | VDD-IAR Alignment dims 2–3 evaluate layer structure and gate compliance |
| 2a | Red Gate | All tests written and failing before implementation begins | [`2a-red-gate.md`](primers/2a-red-gate.md) | VDD-IAR Alignment dim 4 + QE dim 2 evaluate Red Gate compliance |
| 2b | Implementation | Tests made to pass; no new tests added during this phase | [`2b-implementation.md`](primers/2b-implementation.md) | SE, QE, UX, Security domains evaluate implementation quality |
| **3** | **Adversarial Refinement** | **IAR runs until MVR** | [`3-review-session.md`](primers/3-review-session.md) | **This is IAR** |
| 4 | Feedback Integration | Findings route back to the appropriate earlier phase: spec issues to Phase 1a, test issues to Phase 2a, implementation issues to Phase 2b | [`4-feedback-integration.md`](primers/4-feedback-integration.md) | IAR findings drive this loop; round count and finding progression are logged per domain |
| 5 | Formal Hardening | Proof harnesses, fuzzing, mutation testing (not yet owned by this suite — see GAP-ANALYSIS-LOG G-55) | — (G-55) | — *(skip unless your project is safety-critical or cryptographic)* |
| 6 | Four-Dimensional Convergence | Spec, tests, implementation, and formal verification all independently at MVR | — | Partially owned — implementation MVR only (see G-54) *(skip unless your project requires formal verification across all four artifacts)* |

Session primers prime the session before writing or reviewing begins; they are not review prompts. The full primer table appears under [Session primers](#session-primers).

### Per-layer flow (within a project)

The pipeline table above is project-scoped. Within a single layer, the flow is a loop governed by the trigger discipline (`primers/3-review-session.md` § Round triggers). The diagram below makes that loop explicit so a reader does not have to reconstruct it from the primer set (G-136 closure).

```
   Phase 1b decomposition complete (layer's TODO + Red Gate plan written)
                              │
                              ▼
                  ┌───────────────────────┐
                  │ Phase 2a: Red Gate    │
                  │   write failing tests │
                  └───────────┬───────────┘
                              │ commit Red Gate (tests confirmed failing)
                              ▼
                  ┌───────────────────────┐
                  │ Phase 2b: Implement   │
                  │   make tests pass     │
                  └───────────┬───────────┘
                              │ commit implementation (all tests green)
                              ▼
                  ┌───────────────────────┐
                  │ Director manual test  │
                  │   (G-132: 2nd surface)│
                  └───────────┬───────────┘
                              │
                              ▼
              ┌───────────────────────────────┐
              │ Phase 3: IAR Round N          │
              │   active domains per intent   │
              │   (DOMAIN-INDEX § calibration)│
              └──────────────┬────────────────┘
                             │
                             ▼
                ┌────────────────────────┐
                │ Phase 4: Route findings│
                │   per primers/4-...    │
                └─────────┬──────────────┘
                          │
                          ▼
              ┌──────────────────────────┐
              │ Continue trigger? (G-131)│
              │ Any new real finding?    │
              └──┬───────────────────────┘
        yes      │     no (MVR — only Hallucinated / no findings)
       ◄─────────┘                            │
       │                                      │
       │                                      ▼
       │                  ┌──────────────────────────┐
       │                  │ Stop trigger? (G-151)    │
       │                  │ New evidence justifying  │
       │                  │ Round N+1? (cold-batch   │
       │                  │ availability ≠ basis)    │
       │                  └──┬───────────────────────┘
       │             yes     │     no
       │           ◄─────────┘     │
       │           │               ▼
       │           │   ┌──────────────────────────┐
       │           │   │ Layer-gate close criteria│
       │           │   │ (suite-development.md §  │
       │           │   │  Layer-gate close — 7    │
       │           │   │  baseline criteria)      │
       │           │   └──────────┬───────────────┘
       │           │              │ all 7 pass
       │           │              ▼
       │           │      ┌───────────────┐
       │           │      │ Merge layer   │
       │           │      └───────────────┘
       │           │
       ▼           ▼
   ┌────────────────────────────┐
   │ Re-enter relevant phase     │
   │ per Phase 4 routing:        │
   │   1a (spec gap)             │
   │   1b (re-decomposition)     │
   │   2a (test gap)             │
   │   2b (implementation defect)│
   │ then Round N+1              │
   └─────────────────────────────┘
```

The Round N+1 case the diagram captures includes both directions: G-131 forces a continuation when new findings surface (Resolved, director-raised, regression-replay, Deferred-routed, Raised-to-SO adjudicated mid-round); G-151 prevents over-investment when MVR is genuinely reached. Both triggers compose at the same decision point — see `primers/3-review-session.md` § Round triggers for the full discipline.

The merge-gate-criteria block expands per `suite-development/suite-development.md` § Layer-gate close criteria — seven baseline criteria including G-156's developer-voice retrospective requirement (criterion 7) and G-150's intent-calibrated active-domain set (informs criterion 1).

## Governing references

- **VSDD whitepaper** (primary): https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00
- **Original VDD whitepaper**: https://gist.github.com/dollspace-gay/45c95ebfb5a3a3bae84d8bebd662cc25
- **Apprentice-onboarding** (program methodology, tool schedule, assignment briefs): https://github.com/Navigators-Guild/apprentice-onboarding
- **CLAUDE.md** (may be superseded — verify against current apprentice-onboarding): https://gist.github.com/dollspace-gay/ef132e60a27abe6d5f87297c1c040dca
- **Crosslink** (issue tracker, Phase 2+): https://github.com/forecast-bio/crosslink

## Domains

Domains are organized into three types. All domain prompt files live under `domains/`.

**The 16-domain surface is *available*, not *required*.** Default activation for new projects is the 7 core role domains plus VDD-IAR Alignment — that's the scaffold script's default and the typical portfolio-project shape (7–9 active domains per layer review). Extended role domains activate per the conditions in [`domains/DOMAIN-INDEX.md`](domains/DOMAIN-INDEX.md); Portfolio Assessment activates only on portfolio submissions. Per Review 42's Solution-Owner ratification (G-121 closure): the default scaffold encodes the practical floor; the full surface is for projects whose scope warrants it.

**Core role domains** — run on every project. The reviewer embodies a specific job role and brings that role's full professional lens:

| Role | Job title | Prompt file | Focus |
|---|---|---|---|
| Software Engineer | Software Engineer / Backend Engineer / Frontend Engineer | [SOFTWARE-ENGINEER-REVIEW.md](domains/role/SOFTWARE-ENGINEER-REVIEW.md) | Implementation: correctness, error handling, naming, function design, duplication, complexity, type safety, defensive coding, comments, consistency, future-self maintainability, documentation, performance |
| Quality Engineer | Quality Engineer / QA Engineer / Test Engineer | [QUALITY-ENGINEER-REVIEW.md](domains/role/QUALITY-ENGINEER-REVIEW.md) | Test system: acceptance criteria, falsifiability, Red Gate compliance, coverage meaningfulness, logic errors, dead code, dependencies, security surface, regression coverage, quality gates, TDD proxy indicators |
| UX Designer | UX Designer / UX Researcher / Product Designer | [UX-REVIEW.md](domains/role/UX-REVIEW.md) | User experience: empty states, error messages, focus and keyboard behavior, visual consistency, affordances, feedback patterns, long content, native dialog quality. Standard dimensions assume browser interface — see `supplements/cli.md` for CLI projects. |
| Security Engineer | Security Engineer / Application Security Engineer | [SECURITY-REVIEW.md](domains/role/SECURITY-REVIEW.md) | Input handling, persistence data validation, dependency CVEs, secret handling, information exposure, authentication and authorization, audit logging, data classification and control requirements |
| Platform Engineer | Platform Engineer / DevOps Engineer / Infrastructure Engineer | [PLATFORM-ENGINEER-REVIEW.md](domains/role/PLATFORM-ENGINEER-REVIEW.md) | CI/CD pipeline, gate enforcement, DevSecOps (pre-commit hooks, security scanning, secret management, supply chain integrity, least privilege), infrastructure as code, containerization, environment parity, observability, performance |
| Solution Architect | Solution Architect / Software Architect / Technical Lead | [SOLUTION-ARCHITECT-REVIEW.md](domains/role/SOLUTION-ARCHITECT-REVIEW.md) | Architecture: separation of concerns, coupling, data model integrity, interface contracts, state management, immutability, extensibility, technology fitness, complexity budget, decision documentation, session continuity, VSDD purity boundary map, external interface contracts, external service integration |
| Solution Owner | Solution Owner / Product Owner / Product Manager | [SOLUTION-OWNER-REVIEW.md](domains/role/SOLUTION-OWNER-REVIEW.md) | Spec contract: spec coverage, scope creep, technology compliance, over-engineering, under-delivery, design fidelity, backlog candidates, prior-review additions, assignment compliance (phase-appropriate). Opens with a compliance table. DESIGN.md is the contract. |
| Data Engineer | Data Engineer / Database Engineer / Data Platform Engineer | [DATA-ENGINEER-REVIEW.md](domains/role/DATA-ENGINEER-REVIEW.md) | Data layer: data model correctness, validation and normalization, schema evolution, data integrity, storage fitness, access patterns, serialization, consistency, sensitive data handling. See [DOMAIN-INDEX.md](domains/DOMAIN-INDEX.md) for scope-down guidance. |

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

Each domain file contains the current prompt and standard dimensions. Review entries are logged separately under `vsdd-suite/` inside the project being reviewed.

The suite's own adversarial review history is indexed in [`suite-development/SUITE-DEVELOPMENT-REVIEW.md`](suite-development/SUITE-DEVELOPMENT-REVIEW.md), with individual session entries in `suite-development/review-log/YYYY-MM-DD-suite-review.md`. Suite reviews are a single artifact type — sessions vary in mode (defect-search lens vs. registry-walk lens), and the mode lives in each entry's Lens field. See [`suite-development/suite-development.md`](suite-development/suite-development.md) for the entry format.

## Session primers

Session primers establish posture and context at the start of a phase. Use the primer for the phase you are entering before loading any other prompt.

| Primer | File | When to use |
|---|---|---|
| Spec Crystallization | [`primers/1a-spec-crystallization.md`](primers/1a-spec-crystallization.md) | Starting a new project. Use before writing DESIGN.md. Drives behavioral contracts, edge cases, interface definitions, verification architecture. |
| Decomposition | [`primers/1b-decomposition.md`](primers/1b-decomposition.md) | After DESIGN.md is complete and argued with. Produces TODO.md with layered acceptance criteria, Red Gate test plans, manual testing checklists, and (Phase 2+) crosslink issue hierarchy. |
| Red Gate | [`primers/2a-red-gate.md`](primers/2a-red-gate.md) | At the start of Phase 2a — writing the failing-test scaffold for the layer. Establishes the Red Gate discipline: every acceptance criterion has a test that fails for the right reason, committed before Phase 2b begins. |
| Implementation | [`primers/2b-implementation.md`](primers/2b-implementation.md) | At the start of Phase 2b — implementing against the committed Red Gate. Make failing tests pass; do not add new tests during this phase (use the retroactive-Red-Gate label if you must). |
| Adversarial Review | [`primers/3-review-session.md`](primers/3-review-session.md) | At the start of each fresh IAR review session (Phase 3). Establishes adversarial posture before loading any domain prompt. Use in a cold session that has not participated in building the project. |
| Feedback Integration | [`primers/4-feedback-integration.md`](primers/4-feedback-integration.md) | After a Phase 3 round has produced a classified finding set. Routes each finding to the earliest phase that can fix it (spec / decomposition / Red Gate / implementation / suite). Closes the IAR refinement loop. |
| Suite Development | [`suite-development/suite-development.md`](suite-development/suite-development.md) | When developing the IAR suite itself — adding domains, updating dimensions, running gap analysis. Not for reviewing projects. |

The spec crystallization primer establishes the adversarial posture for spec *writing* — the adversary applies pressure during Phase 1a, not only during Phase 3. A spec that was never argued with before implementation began will produce IAR findings that trace back to spec incompleteness, not implementation error.

## Language and interface supplements

Language-specific and interface-type-specific dimensions live in `supplements/`. Domain prompts reference these — during a review, apply the relevant supplement's section for your domain alongside the standard dimensions.

| Supplement | When to use |
|---|---|
| [`supplements/rust.md`](supplements/rust.md) | Rust projects (all domains) |
| [`supplements/javascript-typescript.md`](supplements/javascript-typescript.md) | JavaScript or TypeScript projects (all domains) |
| [`supplements/cli.md`](supplements/cli.md) | CLI interface type — replaces browser-centric UX dimensions; adds CLI QE and SE concerns |
| [`supplements/browser-app.md`](supplements/browser-app.md) | Browser-rendered interface — browser-specific QE (axe, compatibility, responsive), Security (rendering safety, CSP, SRI), and UX dimensions |

A project may use more than one supplement. A TypeScript CLI uses both `javascript-typescript.md` and `cli.md`.

## Worked example: A VSDD session with crosslink

This walkthrough shows one full VSDD cycle in both [operational modes](#two-modes-of-operation-design-principle). Each phase has a `[crosslink]` block (the recommended primary path) and a `[manual]` block (the first-class supported fallback). Both blocks land at the same depth of concrete instruction — neither is the "main" path; both implement the same primer prescription via different mechanisms.

The example project is a hypothetical CLI bookmark manager (`bookmark-cli`); substitute your own project name throughout.

**Example language is Rust** (`cargo test`, `tests/bookmarks.rs`). For other languages, paste [`supplements/javascript-typescript.md`](supplements/javascript-typescript.md) (JS/TS), or the appropriate file in [`supplements/`](supplements/) (Python/Go/etc.), alongside the domain prompt during Phase 3 — and substitute the language's test command (`npm test`, `pytest`, `go test`, etc.) for `cargo test` throughout. The suite is language-agnostic; the example just has to pick one.

**The primer is the spec; crosslink and the manual ritual are two mechanisms for executing the spec.** Each VSDD primer prescribes the discipline (one issue per layer, Red Gate commit boundary, fresh-context review session per domain, finding routing through earliest-phase); the crosslink-primary mode mechanizes the prescription via `crosslink design` / `crosslink quick` / `crosslink swarm gate` / `crosslink swarm review` / `crosslink swarm fix`; the manual mode satisfies the same prescription via fresh chats + `git commit` + paste-the-primer + run-the-test-suite + maintained markdown files. Both are tested and documented to feature parity per the [Two modes of operation](#two-modes-of-operation-design-principle) design principle.

### Overview — what you do in each phase

| Phase | Primer | What the primer prescribes | `[crosslink]` (primary) | `[manual]` (first-class fallback) | Output |
|---|---|---|---|---|---|
| 1a | [`1a-spec-crystallization.md`](primers/1a-spec-crystallization.md) | DESIGN.md written through self-adversary | `crosslink design [--continue <slug>]` | Fresh chat + paste primer + write `DESIGN.md` directly | `DESIGN.md` |
| 1b | [`1b-decomposition.md`](primers/1b-decomposition.md) | Layered acceptance criteria + Red Gate test plans + manual testing checklists | `crosslink quick --parent`, `milestone create`, `milestone add`, `workflow diff` | Fresh chat + write `TODO.md` per primer's format | crosslink layer hierarchy or `TODO.md` |
| 2a | [`2a-red-gate.md`](primers/2a-red-gate.md) | Failing tests committed as the Phase 2a → 2b boundary | `crosslink session start`, `session work` then write + commit failing tests | Fresh chat + write + commit failing tests; tracking via TODO.md | Failing-test Red Gate commit |
| 2b | [`2b-implementation.md`](primers/2b-implementation.md) | Implementation that makes the Red Gate pass; no new tests | Implementation + `crosslink swarm gate <slug>` for the Phase 2b → 3 boundary | Implementation + manual `cargo test` (or equivalent) for the Phase 2b → 3 boundary | Passing test suite |
| 3 | [`3-review-session.md`](primers/3-review-session.md) | Cold-context adversarial review per active domain, classified findings | `crosslink swarm review --agents N --mandate adversarial --file-issues --doc <path>` for routine volume; manual dispatch (right column) for high-stakes / approaching-MVR | One fresh chat per active domain + paste primer + domain prompt + supplement + code; append round to per-domain index + per-session file; append row to `FINDINGS-INDEX.md` | Per-domain review logs + cross-cutting finding index |
| 4 | [`4-feedback-integration.md`](primers/4-feedback-integration.md) | Each finding routed to the earliest phase that can fix it | `crosslink issue label route:*`, `swarm fix --from-label`, `issue block` | Record routing in each finding's review-log entry per primer's `## Without crosslink` section; re-enter routed phases manually | Routed finding set; closure once each phase's gate holds |

Full walkthrough below. Scan the overview first; read the detail when you need the starter prompts and the per-phase commands.

### Setup (one-time per project)

Baseline (both modes):

```sh
mkdir bookmark-cli && cd bookmark-cli
git init
```

**[crosslink]** Initialize the tracker and verify the deployed policy matches the embedded defaults:

```sh
crosslink init                          # initializes .crosslink/, issues.db, embedded policy
crosslink workflow diff                 # verifies deployed policy == embedded defaults (expect: no diff)
crosslink agent --help                  # set up your driver identity if not already done globally
```

**[manual]** No additional setup beyond the baseline. Scaffold the suite into the project per [Bringing the suite into your project](#bringing-the-suite-into-your-project) (the same scaffolding step both modes do — run `templates/scaffold-project.sh` from the project root).

### Phase 1a — Spec Crystallization

Per `primers/1a-spec-crystallization.md`: write `DESIGN.md` against the primer's driving questions; treat the file as a contract once it passes the self-adversary check.

**[crosslink]** Use `crosslink design` to scaffold the iterative session container with a `.design/<slug>.md` working draft and `--continue` resumability — useful when a Phase 4 route brings you back to the spec later:

```sh
crosslink design "bookmark CLI for capturing and recalling URLs"
# Writes .design/bookmark-cli.md and opens a foreground Claude session pre-loaded
# with the 1a-spec-crystallization.md primer.

# Iterate; when the self-adversary check passes, promote draft → contract:
mv .design/bookmark-cli.md DESIGN.md
git add DESIGN.md && git commit -m "Phase 1a: DESIGN.md crystallized"

# Resume the draft later if a Phase 4 route brings you back:
crosslink design --continue bookmark-cli
```

**[manual]** Open a fresh chat session. Paste `primers/1a-spec-crystallization.md`. Then send a starter prompt like:

> I'm starting Phase 1a for a new project, `bookmark-cli` — a CLI tool for capturing URLs at the command line and recalling them later (default operations: `bm add <url>`, `bm list`, `bm search <term>`). Local-first; no network or accounts. Single-user.
>
> Walk me through the primer's driving questions one at a time. Push back when my answers are imprecise. Apply the self-adversary check before declaring `DESIGN.md` ready: name three concrete behaviors a downstream Phase 3 review could catch as undefined or contradictory, then revise the spec to close them.
>
> Output `DESIGN.md` directly; iterate against it.

Iterate until the self-adversary check passes, then commit:

```sh
git add DESIGN.md && git commit -m "Phase 1a: DESIGN.md crystallized"
```

**Both modes — also commit your project `README.md` in this phase.** The project README is the entry point for anyone reading the project (including future-you and any Phase 3 Technical Writer reviewer). Start from the `templates/PROJECT-README-template.md` skeleton you copied during [Bringing the suite into your project](#bringing-the-suite-into-your-project); fill in the project purpose, language/toolchain, how to run, how to test, and link to `DESIGN.md`. The README evolves alongside the implementation; this initial commit just establishes its existence so Phase 3 Technical Writer reviews have something to evaluate.

### Phase 1b — Decomposition

Per `primers/1b-decomposition.md`: break the project into layers; per layer, write acceptance criteria, a Red Gate test plan, and a manual testing checklist. The layer plan exists before any Phase 2 work starts.

**[crosslink]** Materialize the layer plan as a crosslink hierarchy — epic for the project, milestone per layer (the layer's first-class container), one issue per layer parented to the epic, acceptance criteria as sub-issues, the Red Gate test plan as a comment on the layer issue. The same plan, mechanized:

```sh
# Epic for the project
EPIC=$(crosslink quick "bookmark-cli" -p high -l epic --quiet)

# One milestone per layer (the layer's first-class container)
crosslink milestone create "Layer 1: add and list bookmarks"
crosslink milestone create "Layer 2: tag and filter"
crosslink milestone create "Layer 3: export and import"

# One issue per layer, parented to the epic
L1=$(crosslink quick "Layer 1: add and list bookmarks" -p high -l feature -l layer --parent "$EPIC" --quiet)
L2=$(crosslink quick "Layer 2: tag and filter" -p high -l feature -l layer --parent "$EPIC" --quiet)
L3=$(crosslink quick "Layer 3: export and import" -p high -l feature -l layer --parent "$EPIC" --quiet)

# Attach layer issues to their milestones
crosslink milestone add "Layer 1: add and list bookmarks" "$L1"
crosslink milestone add "Layer 2: tag and filter" "$L2"
crosslink milestone add "Layer 3: export and import" "$L3"

# Acceptance criteria as sub-issues
crosslink quick "AC: 'bm add <url>' creates a bookmark with timestamp" -l acceptance-criterion --parent "$L1"
crosslink quick "AC: 'bm list' shows bookmarks newest-first" -l acceptance-criterion --parent "$L1"
crosslink quick "AC: 'bm add' rejects empty URL with literal stderr 'Error: URL cannot be empty.'" -l acceptance-criterion --parent "$L1"

# Red Gate test plan as a comment on the layer issue
crosslink issue comment "$L1" "Red Gate: tests_add_creates_bookmark, tests_list_orders_newest_first, tests_add_rejects_empty_url. All three must fail before any implementation lands."

# Sanity check before opening Phase 2
crosslink workflow diff                     # verify policy unchanged since init
crosslink milestone show "Layer 1: add and list bookmarks"   # verify layer container is populated
```

**[manual]** Open a fresh session. Paste `primers/1b-decomposition.md`. Then send a starter prompt like:

> I'm starting Phase 1b for `bookmark-cli`. DESIGN.md is attached (the Phase 1a output). Decompose the project into the minimum number of layers such that each layer is independently testable and shippable, and each layer's acceptance criteria are observable behaviors (not implementation steps).
>
> For each layer, produce:
> - **Acceptance criteria** in the form `AC: <observable behavior>` — each AC must be falsifiable by a single test.
> - **Red Gate test plan** — the literal test names that will fail before any implementation lands for this layer.
> - **Manual testing checklist** per the runnable-step standard in the primer's `### Manual testing checklist` section (exact commands, expected stdout/stderr/exit code, clean-state setup, binary install/uninstall lifecycle if applicable).
>
> Output `TODO.md` per the primer's `## TODO.md format` section. If a layer's ACs require more than ~5 Red Gate tests, propose a finer-grained split.

(Attach `DESIGN.md` to the message.) The output `TODO.md` is your layer plan — the manual-mode equivalent of the crosslink hierarchy above.

### Phase 2a — Red Gate

Per `primers/2a-red-gate.md`: write the failing tests named in the Red Gate plan, run them, confirm every test fails for the right reason (missing feature, not setup error), then commit. The Red Gate commit is the verifiable Phase 2a → 2b boundary.

**[crosslink]** Attach the session to the active layer issue so the focus is recorded in the tracker, then write + commit the failing tests:

```sh
crosslink session start
crosslink session work "$L1"            # marks the active focus issue for the session
# (open a fresh chat with 2a-red-gate.md primer loaded; write the failing tests
#  named in the layer issue's Red Gate comment)
cargo test                              # expect: 3 failures, named per the Red Gate plan
git add tests/ && git commit -m "Phase 2a: Red Gate for Layer 1 (3 failing tests)"
```

**[manual]** Open a fresh chat. Paste `primers/2a-red-gate.md`. Then send a starter prompt like:

> I'm starting Phase 2a for Layer 1 of `bookmark-cli`. The Red Gate plan from Phase 1b prescribes three failing tests for this layer:
>
> - `tests_add_creates_bookmark` — `bm add <url>` creates a bookmark with a timestamp
> - `tests_list_orders_newest_first` — `bm list` shows bookmarks newest-first
> - `tests_add_rejects_empty_url` — `bm add` with empty URL exits non-zero with stderr `Error: URL cannot be empty.`
>
> DESIGN.md and the Layer 1 plan are attached. Write these tests in `tests/bookmarks.rs` as Rust integration tests. Do not implement anything to make them pass — they must fail for the right reason (function does not exist), not for a setup error. Name each test for the behavior, not the function. Reject any test that would pass against an empty function body.
>
> Run `cargo test` and confirm all three fail with the expected reason before handing back.

(Attach `DESIGN.md` and the Layer 1 plan section of `TODO.md`.) Then run the test suite and commit:

```sh
cargo test                              # expect: 3 failures, named per the Red Gate plan
git add tests/ && git commit -m "Phase 2a: Red Gate for Layer 1 (3 failing tests)"
```

**Both modes:** after this commit, only implementation may land in this layer until Phase 3. The commit is the verifiable Phase 2a → 2b boundary regardless of mode.

### Phase 2b — Implementation

Per `primers/2b-implementation.md`: implement to make failing tests pass; do not add new tests this phase; run the full suite after each feature — no previously-passing test may regress.

**[crosslink]** Open a fresh chat with `primers/2b-implementation.md`, implement against the Red Gate, then formalize the Phase 2b → 3 boundary as a layer gate the tracker enforces:

```sh
cargo test                              # expect: passing
git commit -am "Phase 2b: Layer 1 implementation — bm add, bm list, empty-URL rejection"
crosslink swarm gate layer-1            # runs the project's test suite as the layer gate
```

If the gate fails, fix and re-run; the layer does not open for Phase 3 until the gate passes.

**[manual]** Open a fresh chat. Paste `primers/2b-implementation.md`. Then send a starter prompt like:

> I'm starting Phase 2b for Layer 1 of `bookmark-cli`. The Red Gate is committed at `<sha>`. The three failing tests in `tests/bookmarks.rs` are:
>
> - `tests_add_creates_bookmark`
> - `tests_list_orders_newest_first`
> - `tests_add_rejects_empty_url`
>
> DESIGN.md and the layer plan are attached. Implement to make these tests pass — minimum implementation that satisfies the contracts, no new tests this phase, run `cargo test` after each feature. If a previously-passing test starts failing, stop and fix the regression before continuing. If you find yourself implementing behavior no failing test asserts, surface it as a spec gap rather than implementing silently.

(Attach `DESIGN.md`, the layer plan, and the contents of `tests/bookmarks.rs` to the message.) Then implement and commit:

```sh
cargo test                              # expect: passing
git commit -am "Phase 2b: Layer 1 implementation — bm add, bm list, empty-URL rejection"
```

When the project test suite passes clean (`cargo test` returns 0; equivalent for other languages), the Phase 2b → 3 boundary is reached. The manual mode treats the clean test-suite exit as the gate; the crosslink mode formalizes the gate via `crosslink swarm gate` above. Same boundary, different mechanism.

### Phase 3 — Adversarial Refinement

Per `primers/3-review-session.md`: open ONE fresh chat per active domain (cold context is the gold standard); paste the primer + that single domain's prompt + the code under review; classify every finding; log findings to the per-domain review log per the project-level review log governing standard in [`suite-development/suite-development.md`](suite-development/suite-development.md).

**Activation deduction for `bookmark-cli`:** SE, QE, UX, Security, SA, SO, VDD-IAR Alignment are active. PE not active because `bookmark-cli` ships via `cargo install` to the user's local toolchain with no server pipeline; DE not active because storage is flat JSON in a single file with no managed database. For projects with different shapes — a deployed web service activates PE; a SQLite-backed CLI activates DE — see [`domains/DOMAIN-INDEX.md`](domains/DOMAIN-INDEX.md) for the activation criteria per domain.

**Dispatch options (per primer § Dispatch options — applies in both modes).** Manual one-chat-per-domain dispatch is the gold standard for the highest-stakes reviews (a single human reviewer reading findings as they arrive applies pressure the dispatcher cannot). Crosslink mode additionally offers swarm dispatch for routine volume rounds — `crosslink swarm review` parallelizes the adversary agents across cold-context worktrees. The classifier remains human in either dispatch option; `swarm review` parallelizes adversaries, not classifiers. Choose manual dispatch when approaching MVR and the marginal finding matters most; choose swarm for routine refinement rounds.

**[crosslink]** Two dispatch sub-modes available; both file findings as crosslink issues with the `review-finding` label (G-138 schema).

Swarm dispatch (routine volume):

```sh
crosslink swarm review --agents 6 --mandate adversarial --file-issues \
    --doc vsdd-suite/SOFTWARE-ENGINEER-REVIEW.md
```

Manual dispatch (highest-stakes / approaching-MVR) — same as the manual-mode flow below, with findings ALSO filed as crosslink issues via `crosslink issue create -l domain:<slug> -l layer:N -l round:N -l finding:N -l classification:<class> -l source:<source>` (G-138 finding-index schema).

Classification — comment-then-close pattern (`issue close` does not accept `--comment`; rationale lives in the prior comment):

```sh
# Inspect findings for human classification
crosslink issue list -l review-finding -s open

# Hallucinated example
crosslink issue comment <id> "Hallucinated — control holds: <specific evidence>" --kind decision
crosslink issue close <id>

# Resolved example
crosslink issue comment <id> "Resolved in <commit>" --kind resolution
crosslink issue close <id>
```

**[manual]** For each active domain — open a fresh chat. Paste `primers/3-review-session.md`. Paste the domain prompt (e.g., `domains/role/QUALITY-ENGINEER-REVIEW.md`). Then send a starter prompt like:

> You are running the Quality Engineer review of Layer 1 of `bookmark-cli`, Review 1 (cold session).
>
> Attached: `DESIGN.md`, the Layer 1 plan, `src/bookmarks.rs`, `tests/bookmarks.rs`, the Phase 2a Red Gate commit (`<sha>`), and the Phase 2b implementation commit (`<sha>`).
>
> Apply every dimension in the loaded QE domain prompt. Cite file:line for each finding. Classify each finding per the schema in the prompt — Resolved (only if you applied a fix this session), Dismissed (with rationale), Hallucinated (with specific evidence the concern does not apply), or Deferred (with the named future layer). Watch your own sycophancy per the primer — a clean pass that misses defects is worse than an uncomfortable finding.
>
> Close the session with a `### Summary` line tallying findings by class and a `**Coordination:**` line naming any cross-domain handoffs.

Classify each finding. Append the round to the domain's index file + a session file in `review-log/YYYY-MM-DD-<domain-slug>.md` (slug convention in `suite-development/suite-development.md` § Structure). Append a row to `vsdd-suite/FINDINGS-INDEX.md` per the G-138 manual-path schema. Repeat for every active domain in its own fresh chat (no context sharing — the gold standard discipline that crosslink mode replicates via `swarm review`'s per-agent worktree isolation).

**Both modes — exit signal.** If a full domain pass produces only Hallucinated findings, MVR is reached for that domain. Repeat until all active domains hit MVR.

**Two end states reach MVR — both must be logged:** (a) the round produces only Hallucinated findings (the adversary tried and could not find anything real); (b) the round produces zero findings at all (the adversary looked and found nothing). Both are valid MVR signals, but **silent zero-finding rounds are a structural error** — a future reader of the per-domain log must be able to distinguish "we reviewed Layer 1 and found nothing" from "we forgot to review Layer 1." Log the zero-finding round explicitly with `*(none)*` placeholders under each classification heading per the governing standard in [`suite-development/suite-development.md`](suite-development/suite-development.md) § Finding sections.

**Pausing and resuming an in-progress Phase 3 review.** A full Phase 3 across 6+ active domains is ~3 hours of focused work; pausing is the common case. The cold-context discipline means **you cannot resume the same chat session for the same domain** — context staleness defeats the adversarial value. Two options:

- **Finish the current round before pausing.** Complete the domain you're in (write the `### Summary` line), file that domain's session entry, then pause. Next session opens a fresh chat for the next domain.
- **Checkpoint mid-round.** If you must stop mid-domain, save the in-progress findings as a draft session file with a literal `**Status:** in progress, paused at <YYYY-MM-DD HH:MMZ>` line and an explicit `**Dimensions evaluated so far:**` list. Next session opens a fresh chat with the partial-isolation tradeoff noted: the new session reads the draft as prior-round context (not as own-context), records dimensions still to evaluate, and finishes the round. Mark the round's Summary line with the partial-isolation caveat. Crosslink mode captures the handoff via `crosslink session end --notes` (see Loop-until-MVR below); manual mode saves the draft file directly.

### Phase 4 — Feedback Integration

Per `primers/4-feedback-integration.md`: route every real finding to the earliest phase that can fix it correctly (spec defect → 1a; missing test → 2a; impl bug → 2b; suite gap → suite-dev). The routing keeps each phase's artifact authoritative — a Phase 2b fix to a spec defect leaves the spec wrong.

**[crosslink]** Use route labels and `swarm fix --from-label` to parallelize the safe-to-parallelize subset (Phase 2b fixes, where the route is unambiguous and the test contract is already firm). Other routes (Phase 1a / 1b / 2a) re-enter those phases manually — they need judgement, not parallelism:

```sh
# Apply route labels to filed findings (the primer's route table guides which label to apply)
crosslink issue label <id-spec-defect> route:phase-1a
crosslink issue label <id-missing-test> route:phase-2a
crosslink issue label <id-impl-bug> route:phase-2b
crosslink issue label <id-suite-gap> route:suite

# If a routed Phase 1a finding invalidates Layer 2's plan, block Layer 2 explicitly
crosslink issue block "$L2" <id-spec-defect>

# Optionally: relate findings that are cross-domain coordinated (the suite's
# `**Coordination:**` line, mechanized as a structured issue graph edge):
crosslink issue relate <qe-finding-id> <se-finding-id>

# Dispatch fix agents only for the route:phase-2b cohort
crosslink swarm fix --from-label route:phase-2b --budget-aware

# Phase 1a / 1b / 2a routed work re-enters those phases manually
crosslink design --continue bookmark-cli    # for the routed Phase 1a fix
```

A finding labelled `route:phase-1a` is *not* closed when the fix lands — it is closed when the spec revision passes the self-adversary check (the Phase 1a gate). The discipline keeps each phase's artifact authoritative.

When the route holds, close (comment-then-close pattern — `issue close` does not accept `--comment`):

```sh
crosslink issue comment <id> "Routed to 1a; DESIGN.md §Add revised in <commit>; self-adversary check passed in session <ts>." --kind resolution
crosslink issue close <id>
```

**[manual]** Open a fresh session. Paste `primers/4-feedback-integration.md`. Then send a starter prompt like:

> I'm running Phase 4 for Layer 1 of `bookmark-cli` after Round 1 across SE, QE, UX, Security, SA, SO.
>
> Attached: the per-domain review logs from this round (`vsdd-suite/SE-REVIEW.md`, `vsdd-suite/QE-REVIEW.md`, …), DESIGN.md, the Layer 1 plan, and the implementation source.
>
> For every finding NOT classified Hallucinated, apply the primer's routing table to assign one of: `route:phase-1a` (spec defect), `route:phase-1b` (decomposition gap), `route:phase-2a` (missing/wrong test), `route:phase-2b` (implementation defect), `route:suite` (the adversary couldn't have caught it with current dimensions). Watch for the primary failure mode the primer names: routing every finding to Phase 2b, collapsing the pipeline.
>
> For each finding output a row: `<finding-id> | <route> | <owning artifact> | <gate that must hold for closure> | <sequencing — does another finding need to land first?>`. Multi-phase chains get one row per phase.

(Attach the per-domain review logs and DESIGN.md.) For each finding, record the route in the finding's review-log entry per the primer's `## Without crosslink` section AND in the row's Status column in `vsdd-suite/FINDINGS-INDEX.md`. Re-enter the routed phase (re-open the spec; re-write the Red Gate test; etc.) and close the finding (update the FINDINGS-INDEX row's Status to Closed and add a closure annotation in the review-log entry) when the phase's gate holds for the routed change.

### Loop until MVR

Phase 3 → Phase 4 → re-enter Phase 1a/1b/2a/2b as routes dictate → Phase 3 again. The layer merges when one full Phase 3 round across all active domains produces only Hallucinated findings or no findings (`primers/3-review-session.md` § Session isolation; README § The refinement loop).

Track the round-by-round progression in the per-domain review logs. The MVR signal is visible in the domain index file: a Review row whose Scope summary reads "no real findings (all Hallucinated)" is the exit condition. Same exit signal in both modes.

**[crosslink]** Close the layer milestone and end the session with handoff notes the next session will read:

```sh
crosslink milestone close "Layer 1: add and list bookmarks"
crosslink session end --notes "Layer 1 merged. Reached MVR after 3 rounds. Surprises: empty-URL handling required two spec revisions (R1 missing, R2 too permissive). Next: Layer 2."
git checkout -b layer-2 && git merge layer-1
```

`crosslink session last-handoff` shows the previous session's end notes — read it at the start of every new session to recover context cheaply.

**[manual]** Record the layer-close handoff in the project's CHANGELOG.md (or a `vsdd-suite/HANDOFF.md` if you prefer a dedicated handoff file). The discipline is the same as crosslink's `session end --notes` — capture what was hardest, what surprised you, what the next layer's first session needs to know. Read the prior handoff at the start of every new session. Then merge:

```sh
git checkout -b layer-2 && git merge layer-1
```

The manual mode loses crosslink's `session last-handoff` retrieval but the handoff content is the same.

## Running IAR

All domains may be run independently at any time. A full run is required before merging; individual domains may be invoked mid-layer to catch issues early or validate a specific concern.

**Human-in-the-loop requirement:** IAR's adversarial value depends on a human reading every finding and making every classification decision. The sycophancy checks in each domain exist to prevent an AI agent from validating its own work — but those checks only work if a human is the final decision-maker on what is real, what is dismissed, and what is hallucinated. Automating finding classification without human review collapses the adversarial process into a rubber-stamp loop.

**DESIGN.md prerequisite:** Every domain prompt begins with "Read DESIGN.md first." If DESIGN.md does not exist, do not run domain reviews — there is no spec to evaluate against. Run VDD-IAR Alignment first; it will flag the absence as a process failure (dim 1). Other domains are not meaningful until the spec exists.

**Domain activation:** Core domains run on every project. Extended domains activate based on project type and deployment context — see [`domains/DOMAIN-INDEX.md`](domains/DOMAIN-INDEX.md) for activation criteria. Document which extended domains are active in the project's DESIGN.md or task file before the first IAR run.

### The refinement loop

IAR is iterative. Within a single layer, rounds run until maximum viable refinement (MVR). **The number of rounds is determined by the finding-progression signal, not by a default.** "Two rounds" is a common shape, not a target — some layers reach MVR at Round 2, some require Round 3 (or more), some that look clean at Round 2 are caught by a director-raised regression that re-opens for Round 3. The trigger discipline (`primers/3-review-session.md` § Round triggers) governs both directions: when more rounds ARE needed (continue trigger, G-131) and when more rounds are NOT needed (stop trigger, G-151).

1. **First pass** — Run active domains when the layer is functionally complete. Log all findings. Fix substantive findings.
2. **Continue trigger (G-131).** If Round N closure produces *any* new real findings — including findings surfaced by director manual testing, regression replay, or any source other than the cold-batch (the ITC L6 R3 SO R22 director-raised ID-reuse regression is the canonical example, where 11 cold-batch domain reviews missed a spec violation that manual testing caught) — Round N+1 is **mandatory**. The layer is not at MVR until the round *after* the last new-finding round is clean.
3. **Stop trigger (G-151).** When a full pass across all active domains produces only **hallucinated** findings or no findings, that is the MVR signal — the adversary has run out of real complaints. Running another round after MVR has been reached is **not free**; it requires explicit director justification (specific new evidence or new attack surface that emerged since the MVR round closed). Cold-batch infrastructure being available is not justification. Running rounds past MVR because "the methodology is there" is the process-drift mode dollspace.gay identified at ITC Layer 7 R3 (Review 51 / G-150).
4. **Merge** — Once MVR is reached across all active domains and no Round N+1 trigger has fired.

Round numbers belong in the log. `QE Review 1`, `QE Review 2` is the expected pattern. The progression from real findings to hallucinated findings is evidence the process worked. A layer that merges after a single pass with unresolved real findings is a process failure — log it as one in VDD-IAR Alignment. A layer that runs Round N+1 after Round N reached MVR without a documented continue trigger is also a process failure — over-investment is as much a methodology drift as under-investment.

### Session isolation

An AI agent that reviews multiple domains in one conversation session accumulates context that softens its adversarial pressure. For strongest isolation, reset the AI session between domain reviews — start a fresh conversation for each domain, load only that domain's prompt and the code under review. Parallel sessions are the gold standard; batching domains in one long session is a quality tradeoff. This mirrors the "fresh eyes every time" principle from VDD.

**Same-model review limitation:** The original VDD methodology was designed for cross-model review — the Builder (Claude) and the Adversary (Gemini/Sarcasmotron) are distinct agents with different training and different biases. Same-model review carries elevated sycophancy risk even with context resets: the adversary shares the builder's failure modes and blind spots. The posture primer, context isolation, and domain-specific sycophancy checks exist to partially compensate; they do not fully replicate cross-model adversarial pressure. For the highest-stakes reviews, consider using a genuinely different model as the adversary. Concrete cross-model Adversary options: a Builder on Claude can use [GitHub Copilot Chat](https://github.com/features/copilot) (default model is OpenAI GPT-4-family with selectable alternatives including Anthropic Claude and Google Gemini in some plan tiers — when running Copilot as the cross-model Adversary, select a non-Claude model to preserve the cross-model property), or [Gemini](https://gemini.google.com) as the Adversary; a Builder on Copilot or GPT can use Claude as the Adversary. The cross-model property is preserved by the Builder/Adversary pairing being from genuinely-different model families — not just different products of the same family. Selecting Copilot's "Claude" model option to adversarially-review work done in claude.ai is *not* cross-model review; it is the same model under two product surfaces.

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

[`suite-development/FINDINGS-INDEX.md`](suite-development/FINDINGS-INDEX.md) is the gap registry — a status-only table of identified suite gaps. Narratives for sessions that registered, addressed, or dismissed gaps live in `suite-development/review-log/` and are indexed in `suite-development/SUITE-DEVELOPMENT-REVIEW.md`. Re-run a registry-walk suite review when the suite changes, a new project type is being evaluated, or a post-mortem reveals a class of defect the suite did not catch.

### Portfolio-arc review

Per-project IAR runs evaluate individual projects using the [`domains/meta/PORTFOLIO-ASSESSMENT-REVIEW.md`](domains/meta/PORTFOLIO-ASSESSMENT-REVIEW.md) domain. Before submitting a portfolio, also run a separate pass that spans all projects and evaluates the arc:

- **Growth** — Does the process documentation show improvement from the first project to the last? Design docs, test discipline, commit history, and IAR depth should all mature visibly.
- **Honest retrospective** — Does each project have a post-mortem or DECISIONS.md that admits what went wrong, what was cut, and what was learned? "Everything went perfectly" is a red flag.
- **Assignment alignment** — Does each project match what the assignment asked for, or did scope creep enter at the design stage?
- **Independence** — Does the commit history, problem selection, and scope decisions show that you directed the work, or did the agent make all the choices?
- **Process over product** — A simple tool built with disciplined process outranks a complex tool built chaotically.

## Review logs

Review entries are stored outside the prompt files to keep the prompts stable and reusable. The shape is **per-domain index + per-session entries** (parallel to the suite's own `SUITE-DEVELOPMENT-REVIEW.md` + `review-log/` pattern). Logs live at:

```
{project}/
  vsdd-suite/
    # Per-domain index files (one row per Review N, newest first; link to session file + anchor)
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
    review-log/
      # Per-session entry files (one per UTC date per domain; multiple rounds same day share file)
      YYYY-MM-DD-quality-engineer.md
      YYYY-MM-DD-security.md
      YYYY-MM-DD-platform-engineer.md
      # … one file per (date, domain) pair on which a round was filed
```

**Forward-only constraint:** This index-plus-session structure applies to projects starting after 2026-05-17 (G-89 closure date). Projects whose first IAR run predates that date (e.g., `bookmark-manager/iterative-adversarial-refinement/` and `issue-tracker-cli/iterative-adversarial-refinement/`) retain their existing single-file-per-domain structure (one accumulating file per domain holding all rounds) and must not be retroactively split.

The `supplements/` folder, `suite-development/FINDINGS-INDEX.md`, `suite-development/SUITE-DEVELOPMENT-REVIEW.md`, `suite-development/review-log/`, and `primers/` live in the suite template, not in individual projects.

Only include log files for the domains active on the project. Each per-domain index file conforms to the **project-level review log governing standard** in [`suite-development/suite-development.md`](suite-development/suite-development.md) § Structure / File-level header / Per-session file header: index file holds file-level header (reviewer role, activation if extended, language supplement applied, sycophancy check) + Reviews table; per-session files hold round entries with the standard per-review preamble (`Scope`, `Session note`, optional domain-specific fields), classification-first finding sections drawn from each domain's allowed schema (e.g., `### Resolved`, `### Dismissed`, `### Hallucinated`, plus `### Accepted Risk` for Security/Red Team/Privacy, `### Backlogged` for Solution Owner, etc.), each finding titled `**Finding N — Title (Dim X)**`, and a closing `### Summary` with a `**Coordination:**` line. Portfolio Assessment is the documented exception (dim-first organization).

**Domain slug convention for session files** (used in the per-session filename `review-log/YYYY-MM-DD-<slug>.md`; lowercase, hyphenated, no `-review` suffix — the `review-log/` directory conveys that):

| Domain | Slug | Domain | Slug |
|---|---|---|---|
| Software Engineer | `software-engineer` | Red Team | `red-team` |
| Quality Engineer | `quality-engineer` | Performance Engineer | `performance-engineer` |
| UX Designer | `ux` | Technical Writer | `technical-writer` |
| Security Engineer | `security` | Accessibility Engineer | `accessibility` |
| Platform Engineer | `platform-engineer` | Privacy Officer | `privacy` |
| Solution Architect | `solution-architect` | Localization Engineer | `localization` |
| Solution Owner | `solution-owner` | VDD-IAR Alignment | `vdd-iar-alignment` |
| Data Engineer | `data-engineer` | Portfolio Assessment | `portfolio-assessment` |

## Merging gate

Before a layer may be merged:

1. All active IAR domains have completed at least one full run scoped to that layer
2. The refinement loop has run to MVR — the final round produces only hallucinated findings or no findings
3. Every finding is either **resolved** (fix applied and verified), **dismissed** (rationale documented), or **hallucinated** (push back documented)
4. Accepted risks are explicitly documented with rationale
5. VDD-IAR Alignment has been run and process compliance confirmed
6. Results are logged with round numbers in the respective log files under `{project}/vsdd-suite/`

No active domain may be skipped. A domain with zero findings is a valid outcome — log it with `**Scope:**`, round number, and `**Tests:**` lines so the record is complete.
