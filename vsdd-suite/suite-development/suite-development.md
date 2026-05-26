<!-- hook-bypass[check-no-letter-clusters]: this file documents historical Phase 5 verification surface labels (`Surface A/B/C` per the pre-Review-78 naming) + retired `Option A/B` operator-decision menus from earlier suite-development cycles. Per G-89 forward-only narrative-preservation the historical labels stay as authored; new prose uses descriptive identifiers (Purity Boundary Audit, Mutation Testing, Fuzz Testing, Proof Execution) per the Review 78 rename + named decision options per the Review 94 amendment. The bypass-mechanism is itself a finding for the next registry-walk review. -->
# Session Primer: Suite Development (Meta — Suite Contributors)

Use this prompt at the start of any session whose purpose is developing the VSDD suite itself — adding or modifying domain files, updating dimensions, walking the findings registry, or revising primers. Do not use this for reviewing projects under the suite; use `../primers/3-review-session.md` for that.

---

## Prompt

You are helping develop the **VSDD Suite** (Iterative Adversarial Refinement — IAR — is the Phase 3 component, not the suite's full scope). The suite is itself a software artifact: it has a specification (the VSDD and VDD methodology documents), a design (the domain structure, dimensions, supplement architecture, and session primer set), and an implementation (the domain prompt files, session primers, README, supplements/ supplements, findings index, and DOMAIN-INDEX).

The suite began as adversarial review prompts and dimensions for VSDD Phase 3. It has since expanded to house session primers for every VSDD phase the suite owns (Phase 1a+1b Spec Crystallization, Phase 1c Decomposition / Spec Review Gate, Phase 2a Red Gate, Phase 2b Implementation, Phase 2c Refactor, Phase 3 Adversarial Refinement, Phase 4 Feedback Integration) plus a meta-primer for suite-development sessions. The Phase 1a+1b/1c/2c labeling tracks the VSDD whitepaper's sub-phase taxonomy per G-96 (Review 62); project review logs that reference the prior Phase 1b naming remain valid records under the forward-only constraint. The directory was renamed to `vsdd-suite/` in Review 38 (G-88 closure) to match the expanded scope; "IAR" remains the name for the Phase 3 portion specifically and is preserved in historical project review logs that pre-date the rename per the forward-only constraint. See `../README.md` `## Suite scope` for the full artifact map.

**Apply the same adversarial standard to the suite that the suite applies to projects.** A dimension that sounds rigorous but would not catch a real defect is a quality failure. A sycophancy check copied unchanged across all domains reduces to boilerplate — it is not designed for the domain's specific failure modes. A new domain that overlaps substantially with an existing one has not been scoped correctly. The same standard applies to session primers: a primer's `## Prompt` section that does not name a concrete failure mode for its phase produces softer output, and a completion-criteria section that is not falsifiable will pass against incomplete artifacts.

The adversary's question for every proposed change: **what defect would ship to a user if this change were not made?**

---

## Suite structure

The suite currently contains these artifact types:

| Artifact | Location | Purpose |
|---|---|---|
| Domain files | `../domains/role/` and `../domains/meta/` | Review prompts and evaluation dimensions per role |
| Language and interface supplements | `../supplements/` | Language- and interface-type-specific dimensions applied alongside domain reviews |
| Session primers | `../primers/` | Posture-setting prompts loaded at the start of a phase or session type |
| Domain index | `../domains/DOMAIN-INDEX.md` | Authoritative core/extended classification with activation criteria |
| README | `README.md` | Suite entry point: domain tables, primer table, running instructions, supplement table |
| Findings index | `FINDINGS-INDEX.md` | Cross-cutting registry of identified findings against the suite and their status. Same shape conventions as a project-level FINDINGS-INDEX so suite contributors and suite users encounter consistent registry conventions across scopes. |
| Suite review index | `SUITE-DEVELOPMENT-REVIEW.md` | Index of adversarial review runs of the suite itself |
| Changelog | `CHANGELOG.md` | Record of all non-trivial changes to suite artifacts |

---

## Governing standard for domain files

A complete domain file contains these elements, in order:

1. **H1 title** — `# [Role] Review`
2. **Suite membership line** — standard text linking to `../../README.md` and noting the domain may run independently
3. **Reviewer role line** — exactly one `**Reviewer role: [Title]** ([Job title variants])` line. Not zero, not two. **Exception:** meta domains (`../domains/meta/`) are exempt — they have no job role persona by design. The README explicitly notes this distinction. A meta domain with a reviewer role line is wrong; a meta domain without one is correct.
4. **Purpose statement** — one paragraph. Answers: what does this role evaluate? What failure mode does it own that no other domain owns?
5. **`## Current Review Prompt`** section containing, in order:
   - Scope statement
   - Instruction to read DESIGN.md first
   - Finding classification schema with all valid classifications for this domain
   - Regression check
   - Coordination links (named, linked, relative paths within the same `../domains/role/` folder)
   - Sycophancy check (domain-specific failure modes named — not boilerplate)
   - Language and interface supplement reference (`../../supplements/`) — **required**, or an explicit opt-out line with rationale (e.g., `**Language and interface supplement:** Not applicable. [Reason].`)
6. **Domain-specific structural sections** — some domains have additional sections between the prompt and the dimensions. These fall into two categories:
   - **Required sections** — must be completed before the dimensions apply; their output is prerequisite to the review, not a classified finding. Example: `SECURITY-REVIEW.md` `## Threat Model` — the reviewer must name threat actors, crown jewel, and entry points before reading source files. Required sections must state: what the reviewer must produce, and how that output is logged (as a preamble record, not a resolved/dismissed/hallucinated finding).
   - **Optional extended sections** — conditional sub-dimensions that apply to specific project types. Example: `SE` `### Extended: Documentation`, `SA` `### Extended: External Interface Contracts`. Optional sections state the conditions under which they apply.
7. **`## Standard Evaluation Dimensions`** — numbered list, each dimension:
   - Named in bold (the failure class, not a question)
   - Explains why it matters
   - Names specific failure modes or named attacks
   - Optional extended sub-sections for conditional concerns (e.g., `### Extended: Documentation` in SE, `### Extended: External Interface Contracts` in SA)
8. **Log pointer** — final line: `Review entries are logged in \`vsdd-suite/[FILENAME].md\` inside the project being reviewed.`

**What a weak dimension looks like:** "Are tests present and meaningful?" — a question, not a dimension; names no failure class; gives the reviewer no technique to apply.

**What a strong dimension looks like:** `**Test falsifiability** — Would each test catch a broken implementation? Named attacks: Mutation Testing. Named tools: Stryker (JS/TS), mutmut (Python), cargo-mutants (Rust).` — failure class named, why it matters explained, specific technique given.

**Finding classification schemas by domain type:**

- Most role domains: `resolved`, `deferred`, `dismissed`, `hallucinated` (including the new [AI Engineer](../domains/role/AI-ENGINEER-REVIEW.md) role-domain registered in [Review 83](review-log/2026-05-20-suite-review.md#review-83--2026-05-21-1000z))
- Security and Red Team: `resolved`, `accepted risk`, `dismissed`, `hallucinated` (no `deferred` — security findings are not deferred)
- Solution Owner: `resolved`, `backlogged`, `dismissed`, `hallucinated` (plus `approved deviation`)
- Accessibility: `resolved`, `deferred`, `dismissed`, `accepted deviation`, `hallucinated`
- Performance Engineer: `resolved`, `deferred`, `dismissed`, `accepted limitation`, `hallucinated`
- Privacy: `resolved`, `deferred`, `dismissed`, `accepted risk`, `hallucinated` (accepted risk requires explicit rationale and owner — "we don't have users yet" is not rationale)
- Localization: `resolved`, `deferred`, `dismissed`, `accepted scope`, `hallucinated` (accepted scope requires explicit documentation in DESIGN.md; silence is not acceptance)
- VDD-IAR Alignment (meta): `resolved`, `dismissed`, `hallucinated` (no `deferred` — process findings are binary)
- Portfolio Assessment (meta): `demonstrated`, `partial`, `absent`, `hallucinated` (no `resolved` — portfolio evidence is assessed, not fixed during the review)

**Cross-cutting classification — `raised to SO`:** Any role domain that finds a defect requiring a `DESIGN.md` change must classify it `raised to SO` rather than applying the change directly (only Solution Owner has DESIGN.md change authority). The classification is a valid sub-heading (`### Raised to SO`) for any non-meta role domain log; the body must include the proposed change, rationale, and a Markdown link to the SO log entry where the resolution will be tracked.

Verify the classification schema matches the domain's nature before finalizing a new domain file.

---

## Governing standard for session primers

A complete session primer contains:

1. **H1 title** — `# Session Primer: [Phase or Session Type] (VSDD Phase N)` where applicable
2. **Usage instructions** — when to use this primer, what it produces, what must exist before using it
3. **`---` separator**
4. **`## Prompt`** section — the text pasted into a fresh AI session. Must establish: the AI's role, the governing constraint (what binds the session), and the primary failure mode to watch for (the equivalent of a sycophancy check)
5. **Phase-specific sections** — driving questions, structure templates, completion criteria, checklists as appropriate to the session type. Each section should answer: what does the AI do with this? What does a correct output look like?

Primers are not domain files. They do not have dimensions, coordination links, or log pointers. They have posture and process.

### External dependency references

When suite documentation references a feature of an external tool (crosslink command, AI tool capability, language toolchain feature, etc.), verify the reference against that tool's governing documentation before writing. The check has three rules:

1. **Reference only features that exist in the current released version of the dependency.** If you cannot point at the dependency's documentation, command help text, or release notes for the feature, do not reference it.
2. **Do not speculate about features that "could be added later" as if they were planned.** Phrasing like "the forthcoming X," "the upcoming Y feature," or "tracked as a coordination ask" implies a commitment that the suite's authors generally do not have authority to make against an external repository.
3. **Do not treat a missing feature as a "coordination ask" unless the suite's owner has authority to file and own the PR upstream.** The suite's owner authority extends to suite-side artifacts; it does not extend to other repositories. A genuine coordination ask is filed in the external repo's issue tracker, not in the suite's documentation.

The failure mode this check defends against: an LLM-driven authoring session naturally extrapolates "the suite could integrate more deeply if X existed" into "X is coordination-asked," conflating speculative design with committed plan. The pattern recurred six times across Reviews 40–42 (a fictitious `--with-suite` flag attributed to crosslink's `init` subcommand) and was caught in Review 43 only because the driver flagged it; without the check, similar speculations would compound. The canonical record of the suite's verified crosslink dependency surface is [`../crosslink-contract.md`](../crosslink-contract.md) — any future expansion must update that file with explicit verification or not be referenced at all. The same pattern applies to AI tools (see the data-flow posture in `../README.md` § Prerequisites for the verified AI-tool surface).

### Naming and identifier discipline (Review 78 Finding 4)

When introducing a new methodology concept (a session type; a verification surface; a classification axis; a defect class), name it **descriptively** as the canonical identifier. Letters, short codes, and single-purpose abbreviations are anti-patterns when adopted as the primary identifier — they require the reader to look up what the letter means before any cross-reference downstream is interpretable. Descriptive names carry the meaning at the point of use.

**Canonical worked example (the Review 78 surfacing):** the Phase 5 hardening primer originally named its five forms `Surface A` (property-based testing) / `Surface A.0` (Purity Boundary Audit) / `Surface B` (Mutation Testing) / `Surface C` (Fuzz Testing) / `Surface D` (Proof Execution). The descriptive names existed in the primer alongside the letters, but every cross-reference in domain prompts, review-log entries, FINDINGS-INDEX rows, and CHANGELOG entries used the letter as the primary identifier. A reader encountering "Surface B" anywhere downstream had to look up what "B" meant. Review 78 retired the letters in favor of the descriptive names; the canonical identifier is now "Mutation Testing" / "Fuzz Testing" / etc.

**The discipline:**

1. **Descriptive names are the primary identifier.** When a methodology concept needs a name, the name carries the concept (`Mutation Testing`, `Purity Boundary Audit`, `validator-of-last-resort`). A letter-or-number label is at most an ordering aid (an enumeration in a table), never the primary identifier in cross-references.
2. **Existing well-established abbreviations stay.** `Dim N` / `Layer N` / `Round N` / `Finding N` are acceptable — the abbreviation includes the concept-word, so the meaning is at point-of-use. Domain slugs like `quality-engineer` / `solution-architect` are descriptive and short, not abbreviations. The discipline is forward-looking against NEW lettering / abbreviation adoptions; it does not retroactively rewrite established short forms that carry meaning.
3. **Historical references are preserved per G-89.** Prior-Review entries that used opaque lettering are preserved as historical narrative; reference examples migrate per G-177 precedent.

**Test before adopting a name:** could a cold reader interpret a cross-reference to this name without consulting a reference table? If yes, the name is acceptable. If no, find a descriptive substitute.

**Mechanical detector pattern (audit support):** grep for capital-letter labels next to methodology concept words — `Surface [A-Z]`, `Phase [0-9][a-z]`, `Mode [A-Z]`, `Form [A-Z]`, `Class [A-Z]`, `Type [A-Z]`, `Variant [A-Z]` — across forward-facing suite content (`primers/`, `domains/`, `supplements/`, `README.md`, `suite-development.md`) AND project content (project DESIGN.md, TODO.md, per-domain reviews). Each match is a candidate for the lookup-cost question: would a descriptive name carry the meaning here? The detector is mechanical; the judgment is human (or Sanity Check) — not every match is a defect, but every match is worth the question.

**Companion review dimension:** [Technical Writer](../domains/role/TECHNICAL-WRITER-REVIEW.md) Dim 12 ("Lettering / abbreviation lookup cost") evaluates project documentation against this discipline at Phase 3 review time. Suite-authoring discipline lives here (§ Naming and identifier discipline); project-review discipline lives in the TW domain prompt. The [Documentation Reviewer](../domains/role/DOCUMENTATION-REVIEWER-REVIEW.md) pair (registered in [Review 80](review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z)) validates TW Dim 12 findings via the standard cold-reader pair pattern.

### Anchor-link convention for cross-references ([Review 79](review-log/2026-05-20-suite-review.md#review-79--2026-05-20-1730z) Finding 3)

When authoring forward-facing suite content (primers, domains, README, supplements, suite-development.md) or reference-example project content, inline references SHOULD be markdown links so that a reader can click through to context. The convention covers two distinct categories — **internal navigability** (findings, reviews, files within the repo) and **external credit + sourceability** (software, people, documents authored elsewhere). Operator wording (for internal): "These should be markdown links so that a human can click through to the index and then to the appropriate header in the review." Operator wording (for external): "Mentions of software, people, documents, etc. should have links too to properly credit the projects and to make it easy for a human to read the sources/documentation."

**Convention table — internal navigability:**

| Reference shape | Link target |
|---|---|
| `G-N` in prose | `[G-N](FINDINGS-INDEX.md#g-N)` — anchor on the registry row (each row in [`FINDINGS-INDEX.md`](FINDINGS-INDEX.md) carries an `<a id="g-N"></a>` marker). Two-hop pattern: prose → registry row → Review entry (the registry's first cell links onward). |
| `Review N` (suite-side) | `[Review N](review-log/2026-MM-DD-suite-review.md#review-N--2026-MM-DD-HHMMZ)` — GitHub markdown auto-generates the heading anchor from `## Review N — 2026-MM-DD HH:MMZ` → lowercased, em-dash + colon + space → hyphens. |
| `Review N` (project-side) | `[Review N](vsdd-suite/review-log/2026-MM-DD-{domain}.md#review-N--2026-MM-DD-HHMMZ)` parallel pattern. |
| `Review N Finding M` | Same-file: `[Review N Finding M](#review-N--HHMMZ)`. Cross-file: full path + anchor. Per-Finding-specific anchors are uncommon — the Review heading is usually the right target (the Findings live below it). |
| Domain name (e.g., "Technical Writer", "Quality Engineer", "Sanity Check") | First mention per file → link to the domain prompt file: ``` [Technical Writer](domains/role/TECHNICAL-WRITER-REVIEW.md) ```. Subsequent mentions in same file may be plain text. Role-domain files live under `domains/role/`; meta-domain files under `domains/meta/`. Path is relative to the linking file. |
| Domain dimension (e.g., "TW Dim 13", "PE Dim 38") | Link the dim reference (with or without the domain prefix) to the section anchor in the domain prompt: ``` [TW Dim 13](domains/role/TECHNICAL-WRITER-REVIEW.md#dim-13--inline-reference-navigability-review-79-finding-3) ``` if the heading anchor exists, otherwise link to the domain file (anchor unresolved still better than no link). |
| Primer name / Phase name (e.g., "Phase 2a Red Gate", "Phase 5 hardening", "Convergence primer") | First mention per file → link to the primer file: ``` [Phase 2a Red Gate](primers/2a-red-gate.md) ``` / ``` [Phase 5 hardening](primers/5-formal-hardening.md) ``` / ``` [Convergence](primers/6-convergence.md) ```. Phase numbers that don't have a dedicated primer (e.g., "Phase 1c Spec Review Gate") link to the section in the closest primer or to the suite README phase-table row. |
| Log file mention (e.g., "the suite-review log", "the QE review log") | Link the descriptive phrase to the file or its containing directory: ``` [the suite-review log](review-log/) ``` (when generic) or ``` [the suite-review log for 2026-05-20](review-log/2026-05-20-suite-review.md) ``` (when dated). Project-side: ``` [the QE review log](vsdd-suite/review-log/2026-MM-DD-quality-engineer.md) ```. |
| File path in prose (e.g., ``` `primers/2a-red-gate.md` ```) | Wrap in markdown link: ``` [`primers/2a-red-gate.md`](primers/2a-red-gate.md) ```. Apply when the path is named; skip when the file is mentioned descriptively without a path. |
| § Section reference (e.g., "§ Naming and identifier discipline") | ``` [§ Naming and identifier discipline](suite-development.md#naming-and-identifier-discipline-review-78-finding-4) ``` — anchor from heading slug (GitHub lowercases, replaces spaces with `-`, strips most punctuation). |
| Descriptive cross-document reference (e.g., "the suite's findings registry") | Link the descriptive phrase to the file: ``` [the suite's findings registry](FINDINGS-INDEX.md) ```. The G-ID inline link is independent — both can coexist (e.g., "Closes [G-112](FINDINGS-INDEX.md#g-112) in [the suite's findings registry](FINDINGS-INDEX.md)."). |

**Convention table — external credit + sourceability:**

| Reference shape | Link target |
|---|---|
| Governing documents (VSDD / VDD whitepapers) | [VSDD whitepaper](https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00) ; [VDD whitepaper](https://gist.github.com/dollspace-gay/45c95ebfb5a3a3bae84d8bebd662cc25). Link on every mention until the immediate paragraph; subsequent same-paragraph mentions may be plain text. |
| Software dependencies declared in `crosslink-contract.md` (suite API surface — first-party) | [crosslink](https://github.com/forecast-bio/crosslink). |
| Software dependencies (third-party language tooling, well-known OSS) | Link to canonical homepage or GitHub repo on first mention per file: [Python](https://www.python.org/), [Rust](https://www.rust-lang.org/), [TypeScript](https://www.typescriptlang.org/), [pytest](https://docs.pytest.org/), [ruff](https://github.com/astral-sh/ruff), [mypy](https://mypy-lang.org/), [shellcheck](https://www.shellcheck.net/), [bats-core](https://github.com/bats-core/bats-core), [cargo](https://doc.rust-lang.org/cargo/), [Pre-commit](https://pre-commit.com/), [GitHub](https://github.com/), [Claude Code](https://github.com/anthropics/claude-code). Add new entries as they enter forward-facing prose. |
| People (operators, authors, contributors) | Link the name or handle to the canonical profile: [dollspace.gay](https://github.com/dollspace-gay) (VSDD/VDD whitepaper author + suite originator). New people: prefer GitHub profile; fall back to canonical homepage. |

**Forward-only constraint:** the convention applies to new prose authored on or after [Review 79](review-log/2026-05-20-suite-review.md#review-79--2026-05-20-1730z) adoption (2026-05-20). Historical CHANGELOG / COMPATIBILITY / pre-Review-79 review-log entries and the legacy registry rows are preserved per [G-89](FINDINGS-INDEX.md#g-89) — unlinked prose stays as authored. Future authoring uses the convention; mechanical sweep updates the highest-frequency entry points without retroactively rewriting historical narrative.

**First-mention-per-file rule:** external links land on the FIRST mention in each file (the highest-leverage placement — the reader clicks once and is anchored on the canonical source). Subsequent mentions in the same file are plain text. This avoids visual noise without sacrificing discoverability. Internal links land on every mention (low cost; same-page anchors).

**Companion review dimension:** [Technical Writer](../domains/role/TECHNICAL-WRITER-REVIEW.md) Dim 13 ("Inline-reference navigability") evaluates project documentation against this discipline at Phase 3 review time. The [Documentation Reviewer](../domains/role/DOCUMENTATION-REVIEWER-REVIEW.md) pair (registered in [Review 80](review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z)) Dim 11 ("Inline-reference clickthrough validation") validates TW Dim 13 findings from the cold-reader seat — TW catches unlinked references at authoring time; Doc Reviewer catches broken or miscredited links at review time.

<a id="dual-audience-design-principle-review-80-finding-3"></a>
### Three-audience design principle ([Review 80](review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z) Finding 3; renamed in [Review 84](review-log/2026-05-21-suite-review.md#review-84--2026-05-21-1100z) Finding 4 from "Dual-audience" — the historical "dual" framing pre-dated the suite-developers / suite-users split being made explicit in Review 80's body. The HTML anchor `dual-audience-design-principle-review-80-finding-3` is preserved above for backward link compatibility per [G-89](FINDINGS-INDEX.md#g-89); forward-facing references use the new `three-audience-design-principle-review-80-finding-3` anchor below.)

<a id="three-audience-design-principle-review-80-finding-3"></a>

The suite's audit-trail artifacts — [`FINDINGS-INDEX.md`](FINDINGS-INDEX.md), the [`SUITE-DEVELOPMENT-REVIEW.md`](SUITE-DEVELOPMENT-REVIEW.md) index, per-Review entries in [`review-log/`](review-log/), and per-project finding indexes + per-domain review logs in projects under review — are authored for **three audiences simultaneously**:

1. **Suite developers** (contributors adding domains, changing hooks, evolving conventions) — read these artifacts to understand the discipline they're extending.
2. **Suite users** (project teams applying VSDD to their own projects) — read these artifacts to understand the discipline they're following + to model what their own project-level audit trail should look like.
3. **AI agents** (performing structured lookups, filter by Owner, count by Status, follow Validator chains, navigate to a specific Finding by direct anchor link) — read these artifacts to query state efficiently across many files.

Operator wording: "The findings index and the review logs are intended for two audiences: a human looking at finding status and the review narratives and also an AI Agent to optimize lookups." + extension: "These contracts should hold for both developers and users of the suite."

**The principle:** every audit-trail artifact must serve all three audiences. Human readability is necessary but not sufficient — a narrative that's readable but un-grep-able loses the agent audience; a grep-friendly table without narrative loses the human audience; documentation that serves developers but not users (or vice versa) splits the audit-trail discipline across the suite/project boundary. The discipline is **triple-coding**: every load-bearing fact lands in (a) a narrative form (for humans of either class) AND (b) a structured form (for agents). The shape is symmetric across the suite/project boundary: what the suite teaches users about Finding-header format is what the suite enforces on its own contributors; what agents grep on the suite-side they grep identically on the project-side. The [`check-project-review-discipline.py`](../hooks/check-project-review-discipline.py) and [`check-suite-review-preamble.py`](../hooks/check-suite-review-preamble.py) hooks apply the same schema to both sides — by design.

**Practical implications:**

1. **Schema stability is a contract with all three audiences.** Review preamble shape, Finding header pattern, classification sub-section names, lifecycle field names — these are the agent-API surface AND the convention developers extend AND the convention users follow. The hook enforces the contract uniformly. Changes happen only via explicit methodology shifts in a Review (which itself updates the agent-API documentation below).
2. **Anchor IDs are the primary direct-link primitive.** Per the [§ Anchor-link convention](#anchor-link-convention-for-cross-references-review-79-finding-3) above: G-rows in the registry carry `<a id="g-N"></a>`; forward-only registry rows carry `<a id="rN-fM"></a>`; per-Finding anchors in review-log entries (post-Review-80 forward-only) carry `<a id="rN-fM"></a>` matching the registry row's anchor. The same anchor ID names the same Finding in both places, on both the suite-side and the project-side.
3. **Lookup patterns are part of the spec.** The Agent-API surface section below catalogs the grep / awk / regex idioms agents are expected to use; the suite commits to keeping these idioms stable across releases. Users running the same idioms on their own project-side audit trail get identical behavior. An idiom-breaking change requires its own methodology Review.
4. **Narrative + structured-fact pairs.** A Finding body has prose narrative for the human (developer or user) + the lifecycle fields ( `**Owner:**` / `**Status:**` / `**Blocked by:**` / `**Validator:**` ) for the agent. A Review entry's Summary is prose for the human + the classification-sub-section totals + Backlog count for the agent. The registry table is structured for the agent + the linked Review prose is narrative for the human.
5. **Suite-side and project-side parity.** What the suite enforces on its own audit trail (via [`check-suite-review-preamble.py`](../hooks/check-suite-review-preamble.py)) is what it teaches projects to apply (via [`check-project-review-discipline.py`](../hooks/check-project-review-discipline.py)). The shape is symmetric: a user reading a suite-side Review entry and then authoring a project-side review-log entry encounters the same Finding-header pattern, the same classification universe, the same lifecycle fields. A developer evolving the suite-side schema must update both hooks + the agent-API contract below + the templates in [`templates/`](../templates/) — the shape changes everywhere or nowhere.

**Companion review dimensions per audience** ([Review 84](review-log/2026-05-21-suite-review.md#review-84--2026-05-21-1100z) Finding 4 — each audience has a primary domain whose review applies the audience's lens; the four domains together cover the three-audience surface):

| Audience | Primary domain | Companion dim(s) | Coverage |
|---|---|---|---|
| **Suite developers** (contributors extending the methodology) | [Solution Owner](../domains/role/SOLUTION-OWNER-REVIEW.md) | SO scope-discipline + over-engineering + under-delivery | Ensures the methodology evolution stays calibrated to its declared scope; contributors extending the suite get spec-contract pressure. |
| **Suite users** (project teams applying VSDD) | [Documentation Reviewer](../domains/role/DOCUMENTATION-REVIEWER-REVIEW.md) | Doc Reviewer Dim 1 clone-and-follow fidelity + Dim 2 implicit-knowledge audit + Dim 7 recovery-from-confusion | Ensures the user audience can adopt the methodology without operator hand-holding; cold-reader pressure surfaces what the suite-developer authoring missed. |
| **AI agents** (parallel cold-session reviewers + main-session orchestrators) | [AI Engineer](../domains/role/AI-ENGINEER-REVIEW.md) | AI Engineer Dim 11 audit-trail machine-readability + Dim 1 session isolation + Dim 8 Phase 4 routing | Ensures the agent audience can parse + grep + cold-load the audit trail efficiently; cost-discipline pressure surfaces machine-readability defects + redundant context-load. |
| **Cross-audience narrative quality** | [Technical Writer](../domains/role/TECHNICAL-WRITER-REVIEW.md) | TW Dim 12 lookup-cost + Dim 13 inline-reference navigability | Ensures narrative is readable for both human audiences (developer + user); the agent-side schema-stability contract is enforced by hooks (see below). |

The four-domain coverage is the methodology's own three-audience check: TW writes the narrative; Doc Reviewer reads cold from the user seat; AI Engineer audits from the agent seat; SO scopes from the developer seat. The agent-side schema-stability contract is enforced by the [`check-suite-review-preamble.py`](../hooks/check-suite-review-preamble.py) + [`check-project-review-discipline.py`](../hooks/check-project-review-discipline.py) hooks. The three-audience principle names what the hooks defend + what the four-domain coverage validates.

**Three-audience-lens scope for supplements ([Review 92](review-log/2026-05-24-suite-review.md#review-92--2026-05-24-0419z) Finding 2; codified 2026-05-24 via operator-policy Hybrid path):** the three-audience lens applies to **per-language supplements with broad domain-perspective coverage** (those typically loaded by IAR cycle agents across many domain reviews) AND to **per-tool supplements** (such as [`claude-code-cli.md`](../supplements/claude-code-cli.md) when registered) — these are audit-trail-shaping artifacts in their own right. The lens does NOT apply directly to **narrow-interface supplements** whose scope is bounded to specific interface-types touched by a small set of domains — those supplements **inherit** the three-audience treatment from their host domain context (the domain prompt that loads the supplement per the domain's `**Language and interface supplement:**` preamble field is the surface that serves the three audiences; the supplement provides interface-specific dimensions consumed by that lens).

**Per-supplement classification (Review 92 F2 codification):**

| Supplement | Class | Three-audience-lens required? |
|---|---|---|
| [`rust.md`](../supplements/rust.md) | Per-language broad-coverage | Required ([Review 92 F2](review-log/2026-05-24-suite-review.md#review-92--2026-05-24-0419z) cascade-applied) |
| [`python.md`](../supplements/python.md) | Per-language broad-coverage | Required ([Review 92 F2](review-log/2026-05-24-suite-review.md#review-92--2026-05-24-0419z) cascade-applied) |
| [`javascript-typescript.md`](../supplements/javascript-typescript.md) | Per-language broad-coverage | Required ([Review 92 F2](review-log/2026-05-24-suite-review.md#review-92--2026-05-24-0419z) cascade-applied) |
| [`bash.md`](../supplements/bash.md) | Per-language broad-coverage | Required ([Review 92 F2](review-log/2026-05-24-suite-review.md#review-92--2026-05-24-0419z) cascade-applied) |
| [`claude-code-cli.md`](../supplements/claude-code-cli.md) | Per-tool | Required ([Review 92 F2](review-log/2026-05-24-suite-review.md#review-92--2026-05-24-0419z) cascade-applied) |
| [`github-actions.md`](../supplements/github-actions.md) | Per-interface multi-domain | Required (already present from authoring at [Review 86 Finding 1](review-log/2026-05-21-suite-review.md#review-86--2026-05-21-1200z)) |
| [`json.md`](../supplements/json.md) | Per-interface narrow | Inherits from host domain |
| [`markdown.md`](../supplements/markdown.md) | Per-interface narrow | Inherits from host domain |
| [`toml.md`](../supplements/toml.md) | Per-interface narrow | Inherits from host domain |
| [`yaml.md`](../supplements/yaml.md) | Per-interface narrow | Inherits from host domain |
| [`cli.md`](../supplements/cli.md) | Per-interface narrow (UX-CLI) | Inherits from host domain |
| [`browser-app.md`](../supplements/browser-app.md) | Per-interface narrow (browser) | Inherits from host domain |
| [`css.md`](../supplements/css.md) | Per-interface narrow | Inherits from host domain |
| [`html.md`](../supplements/html.md) | Per-interface narrow | Inherits from host domain |

**Why the split:** per-language supplements + per-tool supplements cover 10+ H2 sections each (the per-domain dimension catalog spans many roles); a reviewer loading the supplement is consulting it across multiple-audience surfaces and benefits from the explicit per-audience framing. Per-interface narrow supplements cover 3-14 H2 sections scoped to specific interface-types; the host domain that loads the supplement (typically UX for CLI; QE + Security for browser-app; SE for syntax-narrow) already applies the three-audience lens at the domain layer, and the supplement provides interface-specific dimensions consumed by that lens. The narrow-interface inheritance preserves the principle's "every audit-trail artifact serves all three audiences" claim without forcing the lens into supplements where the three-audience differentiation would be diffuse. Forward-only per [G-89](FINDINGS-INDEX.md#g-89): supplements existing before 2026-05-24 that match the "narrow-interface" class are NOT required to add a three-audience-lens section; the host-domain inheritance applies retroactively as the existing methodology shape.

### Agent-API surface ([Review 80](review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z) Finding 3)

The suite commits to a stable agent-readable surface across the audit-trail artifacts. This section documents every machine-parseable invariant — agents authored against these invariants will not break across releases unless the methodology shift is itself documented in a Review (which updates this section in lockstep). Forward-only constraint: invariants below apply to artifacts authored on or after [Review 80](review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z) (2026-05-20); pre-Review-80 artifacts may have looser conformance preserved per [G-89](FINDINGS-INDEX.md#g-89).

**Anchor-ID conventions ([Review 93](review-log/2026-05-24-suite-review.md#review-93--2026-05-24-2340z) Finding 5 — central registry of canonical schemes).** The suite uses multiple anchor-ID schemes per the artifact type + scope. Each scheme is mechanically derivable from the artifact's identity-fields; agents authored against these schemes can construct anchors without ambiguity.

| Scheme | Where used | Form | Example |
|---|---|---|---|
| Suite-side forward-only finding | [`suite-development/FINDINGS-INDEX.md`](FINDINGS-INDEX.md) + [`suite-development/review-log/`](review-log/) Review N Finding M entries | `<a id="rN-fM"></a>` | `<a id="r91-f17"></a>` (Review 91 Finding 17) |
| Suite-side legacy finding (closed namespace) | [`suite-development/FINDINGS-INDEX.md`](FINDINGS-INDEX.md) § Legacy registry | `<a id="g-N"></a>` | `<a id="g-89"></a>` (G-89 forward-only narrative-preservation) |
| Project-side forward-only finding (post-[R91 F17](review-log/2026-05-23-suite-review.md#r91-f17)) | `<project>/vsdd-suite/FINDINGS-INDEX.md` + `<project>/vsdd-suite/review-log/` per-session entries | `<a id="<domain-slug>-rN-fM"></a>` | `<a id="quality-engineer-r1-f1"></a>` (QE Round 1 Finding 1) |
| Per-Finding anchor within a per-session review-log file | `<project>/vsdd-suite/review-log/YYYY-MM-DD-<domain>.md` Finding bodies | `<a id="rN-fM"></a>` (domain context implicit from filename slug) | `<a id="r3-f2"></a>` (Round 3 Finding 2 within the file) |
| Review heading auto-anchor | All review-log Review N entries (suite-side + project-side) | GitHub-auto-slug: `#review-N--YYYY-MM-DD-HHMMZ` | `#review-91--2026-05-23-1900z` |
| H2/H3/H4 section auto-anchor | All markdown files | GitHub-auto-slug: lowercase + spaces-to-hyphens + strip-most-punctuation | `#three-audience-design-principle-review-80-finding-3` |
| Suite-development discipline-section anchor (manual) | [`suite-development.md`](suite-development.md) sub-sections explicitly anchored for backward compatibility per heading-rename events | `<a id="<slug>"></a>` immediately above the heading | `<a id="dual-audience-design-principle-review-80-finding-3"></a>` (preserved per G-89 after Review 84 rename) |

**Naming discipline:**

- All explicit anchor-IDs use **lowercase + hyphens** (matches GitHub auto-slug convention)
- Suite-side forward-only finding anchors use **`rN-fM`** (no domain qualifier; suite-side findings are uniquely identified by Review + Finding combo)
- Project-side forward-only finding anchors use **`<domain-slug>-rN-fM`** (domain qualifier required; multiple domains can have same Round + Finding numbers within a project)
- Per-Finding anchors WITHIN a per-session review-log file use the un-qualified `rN-fM` (domain context implicit from filename slug `YYYY-MM-DD-<domain>.md`); cross-cutting registry rows use the qualified form
- Legacy `g-N` namespace is **closed** — no new entries; preserved per G-89
- Heading auto-anchors are NOT explicitly authored; GitHub generates them deterministically — agents construct them by slugifying the heading

**Central-registry property:** all anchor-ID schemes used in the suite are documented here + no new scheme is introduced without amending this table. An agent grepping for a non-listed pattern is a methodology-drift signal worth surfacing as a finding.

**Forward-only:** the registry applies to anchors authored 2026-05-24 and later. Existing anchors (legacy `g-N`; pre-Review-80 ad-hoc forms; PR #50's F17 migration which used the documented `<domain-slug>-rN-fM` form preemptively) are preserved as authored.

**Review heading (per-session entry boundary).**

```
## Review N — YYYY-MM-DD HH:MMZ
```

- Regex: `^## Review (\d+) — (\d{4}-\d{2}-\d{2}) (\d{2}:\d{2}Z)$`
- `N` is a monotonic integer per the suite-wide Review counter.
- Timestamp is UTC (Zulu); `Z` suffix is mandatory.
- GitHub auto-generates the heading anchor: `#review-N--YYYY-MM-DD-HHMMZ` (lowercased; em-dash + colon collapse to hyphens). Example: `## Review 80 — 2026-05-20 18:30Z` → anchor `#review-80--2026-05-20-1830z`.

**Required preamble fields (per Review entry).**

```
**Scope:** ...
**Lens:** ...
**Session note:** ...
**Source:** ...
```

- All four fields appear before the `### Resolved` / `### Deferred` / etc. sub-sections.
- `Source` values are constrained to `director-raised`, `domain-raised`, `regression-replay`, `external-feedback`, `mixed`.
- Hook check: [`check-suite-review-preamble.py`](../hooks/check-suite-review-preamble.py).

**Classification sub-sections (per Review entry).**

- `### Resolved` — for Findings closed in-session.
- `### Deferred` — for Findings logged in-session but with implementation Deferred per G-130.
- `### Dismissed` — for Findings raised but determined to not be defects.
- `### Hallucinated` — for AI-raised Findings without basis in the project state.
- `### Open` — for Findings actively being worked across sessions.
- `### Raised to SO` — cross-cutting, valid for any non-meta role domain.
- Domain-specific classifications per [`DOMAIN_CLASSIFICATIONS`](../hooks/check-project-review-discipline.py): Security uses `Accepted risk`; Performance Engineer uses `Accepted limitation`; Localization uses `Accepted scope`; Solution Owner uses `Backlogged` and `Approved deviation`; Portfolio Assessment uses `Demonstrated` / `Partial` / `Absent` / `Hallucinated` only.

**Finding header (per Finding).**

```
**Finding N — Title**                              <- standard form
**Finding N — Title (Dim N)**                      <- with discipline reference
**Finding N — Title (added YYYY-MM-DD)**           <- errata form
**G-XX — Title**                                   <- legacy gap-ID form (pre-Review-73)
```

- Regex (standard): `^\*\*Finding (\d+) — (.+?)(?:\s+\(([^)]+)\))?\*\*$`
- `N` is monotonic per Review (Findings 1, 2, 3, ... within a single Review entry).
- Hook check: [`check-suite-review-preamble.py`](../hooks/check-suite-review-preamble.py) + [`check-project-review-discipline.py`](../hooks/check-project-review-discipline.py).

**Per-Finding anchor ID (post-Review-80 forward-only).**

```
**Finding N — Title**

<a id="rN-fM"></a>
```

- Place `<a id="rN-fM"></a>` immediately after the Finding header (one blank line between).
- `N` = Review number; `M` = Finding number within the Review.
- The same anchor ID names the Finding in the forward-only registry row in [`FINDINGS-INDEX.md`](FINDINGS-INDEX.md) — agents can navigate directly from prose to Finding to registry row in a single hop.
- Forward-only: pre-Review-80 review-log entries are not retroactively anchored.

**Lifecycle fields (per Finding body; [Review 77](review-log/2026-05-20-suite-review.md#review-77--2026-05-20-1545z) shape).**

```
**Owner:** <domain-slug | *self*>
**Status:** <raised | assigned | fix-landed | validated>
**Blocked by:** <Finding reference | *(none)*>
**Validator:** <domain-slug | sanity-check | *self* with rationale>
```

- Owner / Validator domain-slug values come from the canonical set in [`DOMAIN_CLASSIFICATIONS`](../hooks/check-project-review-discipline.py): `quality-engineer`, `software-engineer`, `ux`, `solution-architect`, `data-engineer`, `platform-engineer`, `technical-writer`, `documentation-reviewer`, `ai-engineer`, `localization`, `performance-engineer`, `accessibility`, `privacy`, `security`, `red-team`, `solution-owner`, `vdd-iar-alignment`, `portfolio-assessment`, `observability`, `sanity-check`.
- Strict self-validation policy: `*self*` requires explicit substantive rationale per [Review 77](review-log/2026-05-20-suite-review.md#review-77--2026-05-20-1545z) Finding 1; with the [Sanity Check meta domain](../domains/meta/SANITY-CHECK-REVIEW.md) registered, `sanity-check` is the preferred fallback.
- Hook check: [`check-project-review-discipline.py`](../hooks/check-project-review-discipline.py) gates lifecycle-field requirements on 2026-05-21+.

**Required closers (per Finding body).**

```
**Resolution:** ...           <- for Resolved findings
**Classification:** ...        <- for everything else (Deferred / Dismissed / Hallucinated / Open / Raised to SO / ...)
```

- Hook check: [`check-suite-review-preamble.py`](../hooks/check-suite-review-preamble.py) enforces presence of one of these closers per Finding body.

**Registry row shape (forward-only registry in [`FINDINGS-INDEX.md`](FINDINGS-INDEX.md)).**

```
| <a id="rN-fM"></a>Review N | Lens | Finding M | Title | Source | Classification | Owner | Validator | Status | [Review N Finding M](review-log/...#rN-fM) |
```

- Anchor ID `<a id="rN-fM"></a>` matches the per-Finding anchor in the review-log entry.
- Row columns are fixed-shape: 10 columns total per [§ Findings registry (forward-only)](FINDINGS-INDEX.md#findings-registry-forward-only).
- Classification + Status values come from the same vocabulary as the review-log sub-section names.

**Legacy registry row shape (G-rows in [`FINDINGS-INDEX.md`](FINDINGS-INDEX.md)).**

```
| <a id="g-N"></a>[G-N](review-log/...#review-N--HHMMZ) | <description> | <type> | <severity> | <difficulty> | <status> | <opened> | <closed> |
```

- Anchor ID `<a id="g-N"></a>` enables direct prose-to-row navigation per [Review 79](review-log/2026-05-20-suite-review.md#review-79--2026-05-20-1730z) Finding 3.
- 8 columns total; first cell is a markdown link from the G-ID to the originating Review.

**Cost-tally schema ([Review 91](review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) Finding 13 — Agent-API contract promotion).**

The cost-tally section is part of the agent-readable surface for capstone+ multi-agent cycles (per § Per-review entry preamble § Cost-tally and § Cost-tally opt-in shape). Schema:

```
**Cost-tally:** (or `**Cost-tally (minimal):**` for inline single-author per § Cost-tally opt-in shape)

**Agent-self-verifiable (countable from this session's tool-call log):**

- **AI tool:** <link>
- **Model:** <model-id>
- **Execution method:** <inline | foreground sub-agent | background sub-agent | worktree-isolated cluster spawn | background Bash task>
- **Tool calls executed:** <N>
- **Files read:** <N> across <scope>
- **Files written/edited:** <N> at <paths>
- **Mechanical sweeps run:** <N> via <Bash idiom>
- **Wall-clock anchors (Bash `date -u`):** session-start <ISO-8601 UTC> → session-end <ISO-8601 UTC>

**Operator-verifiable (requires `/cost` paste or plan-dashboard inspection):**

- **Raw tokens:** *pending operator `/cost` paste*
- **Cache-hit ratio:** *pending operator `/cost` paste*
- **Would-be API cost:** *pending operator `/cost` paste*
- **Rate-limit-window utilization:** *pending operator-dashboard check*

**Operator-confirmable (operator-declared or operator-clocked; should be re-confirmed per session):**

- **Plan tier:** <plan-tier> (source: <operator declaration in session N OR memo at MEMORY.md path>)
- **Actual cost to operator:** <$0 marginal IF on Max plan AND session did not trigger rate-limit | actual figure from operator>

**Derived metric (currently unverifiable + ambiguously interpreted):**

- **Findings/100k tokens:** <N findings / token estimate> = <density> OR `NOT COMPUTABLE — pending operator /cost paste`

**Operator-action queue:** if cost-tally precision is load-bearing for cross-cycle calibration, operator runs `/cost` in this session and pastes the output here as an append-only addendum, replacing the *pending operator …* placeholders with measured values.
```

Parse boundaries: cost-tally section starts at `**Cost-tally:**` or `**Cost-tally (minimal):**` heading; closes at the next `---` separator OR the next `### Open` / `### Resolved` / `### Dismissed` / `### Coordination` heading. Sub-section headings (`**Agent-self-verifiable ...:**`, `**Operator-verifiable ...:**`, `**Operator-confirmable ...:**`, `**Derived metric ...:**`) use parenthetical-form not em-dash-form per the No-em-dash-inside-bold-sub-headings rule below.

**No-em-dash-inside-bold-sub-headings rule ([Review 93](review-log/2026-05-24-suite-review.md#review-93--2026-05-24-2340z) Finding 3 — formalization of [Review 91](review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) commit `2da6ad6` fix + [PE R6 amendment](../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-22-platform-engineer.md#review-6--2026-05-24-0300z) workaround):** any bold sub-heading inside a Review entry that contains ` — ` (space em-dash space) between the bold markers triggers the [`check-suite-review-preamble.py`](../hooks/check-suite-review-preamble.py) finding-header regex `^\*\*\S.+ — .+\*\*\s*$` (the [`FINDING_HEADER_CANDIDATE`](../hooks/check-suite-review-preamble.py) pattern). The hook flags such lines as malformed finding headers + fails the commit. **Workaround at authoring time**: bold sub-headings use **parentheses** or **semicolons** as the inner separator, never em-dash. Examples:

- ✗ `**Operator-verifiable — requires /cost paste:**` — triggers the hook
- ✓ `**Operator-verifiable (requires /cost paste):**` — clean
- ✓ `**Operator-verifiable; requires /cost paste:**` — clean

The rule applies to ALL bold sub-headings in suite-review + project-review entries, NOT just cost-tally sub-headings. Worked around 3 times pre-codification (Review 91 cost-tally rewrite; PE Layer 2 R6 amendment in bookmark-cli-manual; this Review 93 codification's own authoring). Forward-only per [G-89](FINDINGS-INDEX.md#g-89): pre-2026-05-24 bold sub-headings with em-dashes that previously slipped past the hook are preserved as authored; new authoring uses the rule. **Why not tighten the hook regex instead:** the regex catches a real defect class (drift finding headers that omit `Finding N — ` prefix); tightening to require `Finding N` or `G-XX` prefix would mask legitimate drift. The discipline-on-authoring is the right intervention surface.

Grep idioms for cost-tally lookup (extends the Common agent lookup patterns table below):

- All Review entries with operator-pending raw-tokens: `grep -A 2 '^\*\*Raw tokens:' vsdd-suite/suite-development/review-log/*.md | grep 'pending operator'`
- All Review entries declaring inline execution: `grep '^- \*\*Execution method:\*\* inline' vsdd-suite/suite-development/review-log/*.md`
- All Review entries with Bash-instrumented wall-clock: `grep '^- \*\*Wall-clock anchors' vsdd-suite/suite-development/review-log/*.md`

Sibling JSON cost-observability files at `vsdd-suite/suite-development/cost-observability/YYYY-MM-DD-review-N.json` (per [`claude-code-contract.md`](../claude-code-contract.md) § Cost-observability sibling JSON file) provide the machine-readable counterpart to the inline cost-tally for operator-pipeline-filled cross-cycle aggregation. The pair (inline cost-tally + sibling JSON) is the canonical observability surface; agents authoring across releases inherit the stability commitment.

**Common agent lookup patterns.**

| Query | Idiom |
|---|---|
| All findings with `Owner: technical-writer` | `grep -B 1 '^\*\*Owner:\*\* technical-writer$' vsdd-suite/suite-development/review-log/*.md` |
| All Open findings in the forward-only registry | `awk -F'|' '$10 ~ / Open /' vsdd-suite/suite-development/FINDINGS-INDEX.md` |
| All findings validated by `sanity-check` | `grep -B 3 '^\*\*Validator:\*\* sanity-check' vsdd-suite/suite-development/review-log/*.md` |
| Locate Review N Finding M directly | URL: `<file>#rN-fM` (post-Review-80) or `<file>#review-N--HHMMZ` (Review heading anchor, all reviews) |
| Find a G-ID's row in the registry | URL: `vsdd-suite/suite-development/FINDINGS-INDEX.md#g-N` |
| All Findings authored in Review N | `awk '/^## Review N /{flag=1; next} /^## Review /{flag=0} flag && /^\*\*Finding/' vsdd-suite/suite-development/review-log/*.md` |
| Domain slug allowlist | `python3 -c "import re; print(re.findall(r'\"([a-z-]+)\":', open(\"vsdd-suite/hooks/check-project-review-discipline.py\").read()))"` |
| Classification universe for a domain | Look up `DOMAIN_CLASSIFICATIONS["<slug>"]` in [`check-project-review-discipline.py`](../hooks/check-project-review-discipline.py) |
| All Review entries with operator-pending raw-tokens (cost-tally fabrication-defense) | `grep -A 2 '^\*\*Raw tokens:' vsdd-suite/suite-development/review-log/*.md \| grep 'pending operator'` |
| All Review entries declaring inline execution | `grep '^- \*\*Execution method:\*\* inline' vsdd-suite/suite-development/review-log/*.md` |
| All Review entries with Bash-instrumented wall-clock | `grep '^- \*\*Wall-clock anchors' vsdd-suite/suite-development/review-log/*.md` |
| All Supplements applied entries (per Review 91 F2+F4) | `grep '^\*\*Supplements applied:' vsdd-suite/suite-development/review-log/*.md` |

**Preferred lookup pattern recommendation ([Review 91](review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) Finding 16).** Agents working WITHIN the suite SHOULD reach for the catalog'd `grep` / `awk` idioms above before defaulting to `Read` + visual parse. Reading-by-default for queries the catalog covers is itself a finding for [AI Engineer Dim 11 (audit-trail machine-readability cost)](../domains/role/AI-ENGINEER-REVIEW.md) — the audit-trail's machine-readability is wasted if agents don't use it. When the catalog covers the query, the agent's first reach should be the idiom; the catalog is reference material AND behavior recommendation.

Empirical evidence requirement (Open across cycles): if the next 3 suite-review cycles continue defaulting to `Read` over `grep`/`awk` for catalog-covered queries, escalate the discipline-vs-default tension per the "earned by recurrence" doctrine (perhaps: codify a soft-hook that surfaces `Read` calls against indexed files for which a catalog idiom exists, with a one-line warning suggesting the idiom).

The catalog is non-exhaustive; agents may compose new lookups from the documented invariants. Composing across invariants is supported (e.g., "all Resolved Findings owned by technical-writer in Reviews 75 onward" = combine the Review-section filter + Owner filter + Status filter). Invariant-breaking changes require their own methodology Review.

**Stability commitment.** The fields, regex patterns, anchor-ID shapes, classification vocabulary, cost-tally schema, and lookup idioms above are stable surface. Additions (new fields, new classifications, new anchor-ID shapes, new cost-tally tiers) require a Review entry and an update to this section in lockstep. Deletions or backward-incompatible renames are forbidden under the suite's [G-89](FINDINGS-INDEX.md#g-89) forward-only narrative-preservation policy except as a structured methodology shift documented in a Review.

---

## Governing standard for project-level review logs

A project-level review log is the artifact produced by running a domain review on a project under review. The domain prompt file specifies *what* to evaluate; this standard specifies *what the resulting log must contain*. Drift in log structure makes cross-domain reading harder and hides governance gaps — apply this standard whenever a new project-level log is created or an existing one is updated.

### Structure (per-session entries + cross-cutting registry; per-domain index is optional)

**Forward-only constraint:** This review-log-plus-FINDINGS-INDEX structure applies to projects starting after 2026-05-17 (G-89 closure date). Projects whose first IAR run predates that date retain their existing single-file-per-domain structure (one accumulating file per domain holding all rounds) and must not be retroactively split. Reference: G-89's row in [`FINDINGS-INDEX.md`](FINDINGS-INDEX.md).

For a new project, each active domain produces per-session files; the cross-cutting registry is project-wide; the per-domain index file is optional and activates only when a project wants a navigation surface organized by domain rather than by date+domain:

| Artifact | Required? | Location | Content |
|---|---|---|---|
| **Per-session file** | **Required.** One per (date, domain) pair on which a round is filed. | `<project>/vsdd-suite/review-log/YYYY-MM-DD-<domain-slug>.md` | The actual round entries. One file per UTC date per domain; if multiple rounds for the same domain happen on the same date, they share a file (new rounds appended at the top of the file, newest first within the date). |
| **Cross-cutting findings registry** | **Required.** Project-wide. | `<project>/vsdd-suite/FINDINGS-INDEX.md` (manual mode) or the crosslink issue tracker with `domain:<slug>` / `layer:N` / `round:N` / `classification:<class>` labels ([G-138](FINDINGS-INDEX.md#g-138)) | One row (or one labelled issue) per finding across every domain and layer. The cross-cutting view that answers "show me all Open findings" or "show me everything raised on Layer 2." |
| **Per-domain index file** | **Optional.** | `<project>/vsdd-suite/<DOMAIN>-REVIEW.md` (e.g., `vsdd-suite/QUALITY-ENGINEER-REVIEW.md`) | File-level header (see below) + a **Reviews** table indexing every round filed for that domain. One row per round, newest at the top. Each row links to the session file's anchor for that round. This file is the index; it does not contain finding narratives. Activates when the project wants a domain-organized navigation surface in addition to the date-organized `review-log/`. |

**Per-domain index file is optional as of Review 84.** Per [Review 84](review-log/2026-05-21-suite-review.md#review-84--2026-05-21-1100z), the `bookmark-cli-manual` reference example retired its 13 per-domain index files; the project now navigates exclusively via `review-log/` + `FINDINGS-INDEX.md`. The per-domain index was effectively required prior to PR #40; PR #40's operator decision is that the per-domain index is redundant with `review-log/` (date-organized navigation already names the domain via the filename slug) + `FINDINGS-INDEX.md` (cross-cutting view answers every multi-round / multi-domain query the per-domain index served). Future projects scaffolded at v0.13.0+ default to no per-domain index files; the template at [`templates/DOMAIN-REVIEW-template.md`](../templates/DOMAIN-REVIEW-template.md) remains for projects that opt in via the scaffold script's `--with-per-domain-indexes` flag or by manual creation. The `bookmark-cli-manual` post-retirement shape is the new canonical reference; pre-PR-#40 reference projects with per-domain index files in place retain them as historical record per the G-89 forward-only carve-out.

**Domain slug convention** for the session-file name: lowercase, hyphenated, derived from the role title (no `-review` suffix — the `review-log/` directory conveys it). Examples:

- Quality Engineer → `quality-engineer`
- Security Engineer → `security`
- Solution Owner → `solution-owner`
- Solution Architect → `solution-architect`
- Software Engineer → `software-engineer`
- Platform Engineer → `platform-engineer`
- Data Engineer → `data-engineer`
- UX Designer → `ux`
- Red Team → `red-team`
- Performance Engineer → `performance-engineer`
- Technical Writer → `technical-writer`
- Accessibility Engineer → `accessibility`
- Privacy Officer → `privacy`
- Localization Engineer → `localization`
- VDD-IAR Alignment → `vdd-iar-alignment`
- Portfolio Assessment → `portfolio-assessment`

**Cross-domain references** between findings: link directly to the session file with the round's anchor (never to the optional per-domain index, even when present). Use the same `[text](path)` form the gap registry uses: `[QE Review 4](review-log/2026-06-15-quality-engineer.md#review-4--2026-06-15-1400z) Finding 2`. The session file is the canonical narrative target for all cross-references.

**Why per-session files + a cross-cutting registry are the canonical default:** session-file scoping makes scoped-search (`grep` for a specific date or round) cleaner; the filename slug already names the domain, so date-organized navigation surfaces the same information a per-domain index would; the cross-cutting registry (`FINDINGS-INDEX.md` or labelled crosslink issues) answers every multi-round / multi-domain query that motivated the per-domain index in the prior shape. Large projects don't produce single domain files in the multi-thousand-line range. The pattern mirrors the suite's own [`SUITE-DEVELOPMENT-REVIEW.md`](SUITE-DEVELOPMENT-REVIEW.md) + `review-log/` structure (the suite retains its own per-domain index as a contributor-facing artifact because the suite's review cadence is denser than any single project's).

### Project-level finding index (cross-cutting registry)

**Forward-only constraint:** This finding-index pattern applies to projects starting after 2026-05-17 (G-138 closure date). Projects under the legacy single-file-per-domain shape (per the G-89 carve-out) continue without it. Reference: G-138's row in [`FINDINGS-INDEX.md`](FINDINGS-INDEX.md).

The per-domain index + per-session-file structure (above) indexes ROUNDS but not individual FINDINGS. A project with 50 findings across 10 domains has no cross-cutting view — answering "show me all Open findings" or "show me everything raised on Layer 2" requires reading every domain index. The finding-index gives that cross-cutting view; it is to project findings what [`FINDINGS-INDEX.md`](FINDINGS-INDEX.md) is to suite gaps.

Two operational modes — pick by whether crosslink is installed; the manual mode is a first-class fallback that carries the same discipline, not a second-class lite version.

**[crosslink] mode — recommended path:**

Every classified finding is also a crosslink issue. The markdown review-log entry remains the canonical narrative; the crosslink issue is the index entry with labels for fast filtering.

| Label key | Values | Example |
|---|---|---|
| `domain:<slug>` | Per the domain slug convention above | `domain:quality-engineer`, `domain:security` |
| `layer:N` | The layer the finding was raised on | `layer:2`, `layer:6` |
| `round:N` | The IAR round number within that layer/domain | `round:1`, `round:3` |
| `finding:N` | The finding number within that round | `finding:2` |
| `classification:<class>` | The closing classification per the domain's schema | `classification:resolved`, `classification:hallucinated`, `classification:dismissed`, `classification:accepted-risk`, `classification:backlogged`, `classification:deferred` |
| `source:<source>` | Per the per-review preamble Source field (see G-133) | `source:domain-raised`, `source:director-raised`, `source:regression-replay` |

The issue title is the finding's title verbatim. The issue body links to the session-file anchor (`Closes: [QE Review 4](vsdd-suite/review-log/2026-06-15-quality-engineer.md#review-4--2026-06-15-1400z) Finding 2`). Status follows: `open` while the finding is Open; `closed` once classified terminal. Quick lookup via the crosslink CLI:

```
crosslink issue list -l domain:quality-engineer --status open      # all open QE findings
crosslink issue list -l layer:2                                     # everything on Layer 2
crosslink issue list -l classification:accepted-risk                # all accepted risks
crosslink issue list -l source:director-raised                      # everything caught by the director (manual testing, etc.)
crosslink tui                                                       # interactive browse
```

`crosslink swarm review --file-issues` files findings automatically with the `review-finding` label; reviewers add the structured labels above during classification. The crosslink mode is recommended because the labels are queryable and the audit trail (label history, comment threads, close timestamps) is built-in.

**[manual] mode — first-class fallback path:**

A single `<project>/vsdd-suite/FINDINGS-INDEX.md` file holds the cross-cutting registry, structured like [`FINDINGS-INDEX.md`](FINDINGS-INDEX.md) — one row per finding with columns for **Anchor-ID**, layer, round, domain, finding-number, title, source, classification, owner, validator, status, and a link to the per-session-file anchor for the full narrative. Quick lookup is via grep or markdown viewer with table filtering. Template at [`vsdd-suite/templates/PROJECT-FINDINGS-INDEX-template.md`](../templates/PROJECT-FINDINGS-INDEX-template.md).

**Anchor-ID column shape ([Review 91 Finding 17](review-log/2026-05-23-suite-review.md#r91-f17) closure 2026-05-24; previously `F-XXX` ID prefix retired):** rows use `<a id="<domain-slug>-rN-fM"></a><domain-slug>-rN-fM` form where `<domain-slug>` is the row's Domain column value (per the domain slug convention above), `rN` is the row's Round column value, and `fM` is the row's Finding column value. Example: `<a id="quality-engineer-r1-f1"></a>quality-engineer-r1-f1` for QE Round 1 Finding 1. The anchor-ID is unique within the project-level FINDINGS-INDEX file + matches the per-Finding anchor scheme in each per-session review-log file at `review-log/YYYY-MM-DD-<domain-slug>.md` (which uses `<a id="rN-fM"></a>` form within the file — the domain context is implicit from the filename slug). Agent grep idiom: `grep '| <a id="' <project>/vsdd-suite/FINDINGS-INDEX.md` returns every row uniformly with the suite-side `grep '| <a id="r' vsdd-suite/suite-development/FINDINGS-INDEX.md` idiom (the scheme differs by the domain-slug prefix but the grep pattern matches both).

The `F-XXX` ID prefix scheme was retired at [Review 91 Finding 17](review-log/2026-05-23-suite-review.md#r91-f17) per operator-policy Option B full migration. The bookmark-cli-manual reference example migrated its 47 rows (F-001..F-047) to the anchor-ID scheme as the canonical worked example per the G-177 reference-examples-stay-current obligation. Legacy `F-XXX` cross-references in pre-2026-05-24 project artifacts (PROCESS.md, CHANGELOG.md, per-session review-log narratives) are preserved as historical record per [G-89](FINDINGS-INDEX.md#g-89) forward-only narrative-preservation; a compatibility table at the top of the project-level FINDINGS-INDEX maps each legacy F-XXX to its post-migration anchor-ID for reader discoverability.

The manual mode matches the crosslink mode's information shape exactly so a project that adopts crosslink later can mechanically migrate the markdown rows into crosslink issues. This is not a degraded path — every IAR discipline (per-domain index, per-session file, sycophancy check, Red Gate, MVR signal, routing table) is fully exercisable in manual mode. The trade-off is mechanical (queryability via grep vs. label filter, audit trail in git history vs. issue comment thread), not methodological.

**Either mode, both are forward-only:** A project chooses one mode at start. Switching modes mid-project (manual → crosslink) requires migrating existing rows; switching the other way (crosslink → manual) requires exporting via `crosslink export`. Switching is supported but not free; choose deliberately at scaffold time.

### Layer-gate close criteria (PROCESS.md retrospective discipline)

Layer-gate close criteria govern when a layer's IAR round may close and the layer may merge to the project's main branch. The full criteria set is project-scoped — a project may codify additional criteria in its own `iterative-adversarial-refinement/CLOSURE-PROTOCOL.md` (per ITC's precedent) — but the suite-level baseline below applies to every project regardless of whether it codifies a CLOSURE-PROTOCOL of its own.

**Baseline criteria (every project, every layer-gate close):**

1. Every active IAR domain (per the project's intent calibration — see `../domains/DOMAIN-INDEX.md` § Intent calibration) has completed at least one cold-session pass on this layer.
2. The refinement loop continued until MVR. A round that produced only Hallucinated findings closes the layer's IAR; a round that produced new real findings re-opens the layer for Round N+1 (per `../primers/3-review-session.md` § Round triggers — both the continue trigger from G-131 and the stop trigger from G-151 apply). Running Round N+1 after Round N reached MVR requires explicit director justification (specific new evidence or new attack surface); cold-batch infrastructure being available is not justification.
3. Every finding is in a terminal state (Resolved, Dismissed, Hallucinated, Backlogged, Approved deviation, Accepted risk, Accepted deviation, Accepted limitation, Accepted scope, Deferred per scope/timing rules).
4. CHANGELOG.md accurately describes what changed this layer (added/changed/removed/addressed sections per the closing block discipline).
5. The project's build and test gate is green per the project's tooling (`cargo build && cargo test && cargo clippy && cargo fmt --check --locked` for Rust; equivalent for other languages).
6. Any DESIGN.md changes during the layer have explicit SO authorship or SO ratification recorded in the SO log.
7. **PROCESS.md retrospective for the layer is at least started — with developer-voice prose, not just scaffolding.** A retrospective section is "at least started" when at least one first-person sentence from the developer follows the italicized scaffolding block. An unfilled italicized scaffolding block (the `*[First-person reflection on Layer N. Possible threads: ...]*` template prose alone, with no developer-written prose underneath) is NOT "at least started" — the scaffolding is the prompt; the developer's prose is the response. **Applies to each `## What was hardest`, `## What I got wrong`, and `## What the process felt like` section per layer.** Empty placeholder sections block layer-gate close regardless of other criteria.
8. **Every Phase 3 round has a Phase 4 routing record.** Routing is per-round, not per-layer — a layer that closes IAR at Round N must have N routing records per [`../primers/4-feedback-integration.md`](../primers/4-feedback-integration.md) § Routing output. The canonical record shape post-bookmark-cli-manual-PR-#52: per-domain `## Phase 4 routing — Round N` appendices in each per-domain review-log entry (NOT a standalone consolidated routing file — that shape was declared anti-pattern at bookmark-cli-manual PR #52 operator directive 2026-05-25). For rounds with zero routable findings (e.g., a round that produced only Hallucinated findings), the per-domain appendix uses `*(none — round produced no routable findings)*` placeholder text so the routing-was-considered signal is structurally visible. **Hook-enforced** via the `**Phase 4 routing:** <reference | *(no routable findings)*>` closing field that [`check-suite-review-preamble.py`](../hooks/check-suite-review-preamble.py) validates on every Phase 3 round entry per primer 3 § Round closing. Closes [Review 94 Finding 1](review-log/2026-05-24-suite-review.md#r94-f1) (Phase 4 routing bypass via Layer-gate-criteria omission).

**Why criterion 7 is a hard gate, not advisory:** The pattern of "PROCESS.md retrospective sections remain empty across multiple layer-gate closures" recurred in `issue-tracker-cli` across Portfolio Assessment Reviews 1, 2, 3, 4, 5 (five consecutive assessments documented the same gap). The R4 standing recommendation ("the nine first-person reflection placeholders are the cheapest single change") went partially-addressed (9 → 7) but two whole layer entries (Layer 6 + Layer 7) were absent at R5 time. Layer 6 was precisely the layer with the strongest single director-ownership artifact (SO R22 next_id reversal); its absence in PROCESS.md was a significant lost-evidence event. The "block portfolio assessment but not technical merge" framing that prior CLOSURE-PROTOCOL drafts used was what allowed the pattern to persist — recurrence trigger per "earned by recurrence" doctrine. The criterion is now a baseline hard gate. Reference: G-156's row in [`FINDINGS-INDEX.md`](FINDINGS-INDEX.md).

**Forward-only constraint:** The hardened criterion 7 applies to projects starting after 2026-05-18 (G-156 closure date). Projects whose first layer-gate close predates that date may have retrospective sections with the older advisory framing; do not retroactively fail those gates. The criterion applies to all *new* layer-gate closes in active projects regardless of when the project started.

**Promotion to a project-level CLOSURE-PROTOCOL.md:** A project that codifies its own closure protocol (per ITC's precedent in `issue-tracker-cli/iterative-adversarial-refinement/CLOSURE-PROTOCOL.md`) MUST include criterion 7 above as a baseline; the project's protocol may add criteria (auto-Backlog rules, warm-finding-closure carve-outs, etc.) but may not weaken the baseline. The suite does not currently ship a CLOSURE-PROTOCOL template — projects either inherit the baseline criteria above implicitly (the suite's standing rule) or codify their own. A future template addition is the natural next step if a third project codifies one (per "earned by recurrence").

### Deferral-trigger discipline (G-130)

Every `Deferred` finding in a project review log must name three things:

1. **The trigger** — a specific layer or measurable condition that releases the deferral, not "when we have time" or "future work." Valid examples: "Layer 5 (when the third file is introduced)"; "When the cmd_list size exceeds 400 lines (currently 287)"; "When user count exceeds 100 (currently 1)." Invalid examples: "Future"; "Soon"; "When convenient"; "TBD."
2. **The cost-of-deferral** — what worsens if the deferral persists past its trigger. Valid examples: "Each additional layer that adds a `parse_*` validator without the refactor compounds the duplication"; "Each new user the tool gains without the auth review increases the multi-user attack surface." Invalid examples: "It would be nice to have"; "No specific cost."
3. **The auto-Backlog clause** — an explicit fall-through if the trigger expires without action. Valid examples: "If Layer 5 closes without this refactor applied, the finding auto-Backlogs at Layer 5 R2 closure"; "If 6 months pass without the schema migration, the finding auto-Backlogs and re-raises as a Security review item."

**Auto-Backlog mechanism (promoted from ITC CLOSURE-PROTOCOL.md §3 to suite-default):** A finding that has been Open across three consecutive reviews of the originating domain without adjudication by the receiving authority should be auto-Backlogged by the originating domain at the start of the third subsequent review, with the original finding text plus a "carry-forward" annotation. This prevents the indefinite-Open pattern that recurred in ITC across multiple layers (SA R7 F2 → SA R9 F1 → SA R11 F1 → SA R13 F1+F2 → eventual auto-Backlog at L7 R2 — three layers of deferral expiring without action before the mechanism finally fired). The mechanism is reversible: if the receiving authority later adjudicates, the finding moves out of Backlogged into the appropriate terminal state. The point is to surface "this question has not been answered" as an explicit Backlog entry rather than as silent log noise.

**Counter-rule:** **Security**, **Red Team**, and **VDD-IAR Alignment** findings do not auto-Backlog. Process and security findings carry forward as Open until explicitly resolved; their visibility is the closure mechanism. The CLOSURE-PROTOCOL.md schema already forbids `Deferred` for these domains; auto-Backlog is the parallel rule for `Open`.

**Why deferral-trigger discipline is a hard standard:** ITC PROCESS.md L6 named this gap explicitly ("I need a mechanism to make sure deferred items are properly worked. Maybe like some sort of task manager lol lmao") and L7 ("Clearer task ownership will resolve this in future projects"). The deferral-as-procrastination pattern recurred across multiple layers before the §3 mechanism caught it. Per the "earned by recurrence" doctrine, two-layer recurrence in one project plus operator-named pain is sufficient to promote the discipline from project-scope to suite-default. Reference: G-130's row in [`FINDINGS-INDEX.md`](FINDINGS-INDEX.md). Coordinate with G-133 (Source field — director-raised findings are often what re-opens an auto-Backlogged finding the cold adversary missed).

### File-level header (top of the per-domain index file, when used)

When the project uses the optional per-domain index (per the structure table above, projects scaffolded at v0.13.0+ default to no per-domain index; this section applies only to projects that opt in via the scaffold script's `--with-per-domain-indexes` flag or by manual creation), the per-domain index file opens with these elements, in order:

1. **H1 title** — `# [Role] Review Log (Index)`
2. **Suite link line** — standard text linking to the project's `README.md` and noting the review is part of the VSDD suite
3. **Purpose statement** — one paragraph stating what this domain evaluates in this project
4. **Reviewer role line** — `**Reviewer role: [Title]** ([Job title variants])` — required for role domains, copied verbatim from the domain prompt file. **Exception:** meta domains are exempt — parallel to the prompt-file rule.
5. **Activation line** — `**Activation:** [conditions and rationale]` — required for **extended** domains active on this project; omitted for core domains and meta domains
6. **Language supplement applied** — required when the domain prompt file references a supplement file in `../../supplements/`; format: `**Language supplement applied:** \`../../supplements/<file>.md\` ([Section name]).` Domains marked language-agnostic in the prompt file include an explicit opt-out: `**Language supplement applied:** Not applicable. [Reason].`
7. **Sycophancy check** — required; one paragraph restating the domain-specific failure mode the reviewer must resist. Drawn from the domain prompt file's sycophancy check, not paraphrased into generic warnings.
8. **Reading convention line** — `Read individual rounds via the links in the Reviews table below. This file is the index only; round narratives live in `review-log/YYYY-MM-DD-<domain-slug>.md`.`
9. **Reviews table** — index of every round filed for this domain.

```
## Reviews

| Review | Date | File | Scope summary |
|---|---|---|---|
| Review N | YYYY-MM-DD HH:MMZ | [YYYY-MM-DD-<slug>.md](review-log/YYYY-MM-DD-<slug>.md#review-n--YYYY-MM-DD-HHMMZ) | One-sentence scope/outcome summary |
| Review N-1 | … | … | … |
```

Rows are ordered newest-first. The summary column is one sentence; the canonical narrative is in the linked session file.

### Per-session file header

The session file opens with a simple H1 (`# [Role] Review — YYYY-MM-DD`). No file-level metadata is duplicated from anywhere — the file is self-describing via the filename slug (domain) and the H1 date. When the project uses the optional per-domain index, a reader can follow the rounds back to it via an `[Index](../<DOMAIN>-REVIEW.md)` link in the H1's footer line if desired; this footer link is optional in all cases and is omitted entirely for projects that do not use the per-domain index.

```
# Quality Engineer Review — 2026-06-15

[Index](../QUALITY-ENGINEER-REVIEW.md)  <!-- optional; omit if the project does not use the per-domain index -->

---

## Review 4 — 2026-06-15 14:00Z

[entry per the structure below]

---

## Review 3 — 2026-06-15 10:30Z

[entry per the structure below]
```

Within a session file, rounds are ordered newest-first (matching the index ordering).

### Per-review entry preamble (under each `## Review N — YYYY-MM-DD HH:MMZ`)

**Canonical ordering** (Review 68 Finding 8 clarification): for project-level review logs, the required fields appear in the order **Scope** → **Session note** → **Source**. For suite-review entries (which additionally require `**Lens:**` per § Suite review entry format), the order is **Scope** → **Lens** → **Session note** → **Source**. Optional fields (when applicable) follow the required block in any order. Consistent ordering across entries makes cross-review reading faster — a reviewer scanning multiple session logs finds each field at the same relative position.

**Required for all domains:**
- **Scope:** what artifacts were reviewed in this round
- **Session note:** session-isolation status (cold session vs. in-session, with explicit acknowledgement of the quality tradeoff when in-session)
- **Source (G-133):** how this round's findings were elicited. Valid values: `domain-raised` (the cold adversary, applying the domain's dimensions, found the finding) — the default if no Source line is present; `director-raised` (the operator running manual testing, post-MVR exploration, or any non-domain-prompt-driven adversarial pass found the finding; ITC L6 R3 SO R22 is the canonical example — director's manual execution of "delete highest-id, create" caught a spec violation 11 cold-batch IAR domain reviews missed); `regression-replay` (a prior layer's adversarial reproducer re-run against the current binary surfaced the finding); `external-feedback` (an upstream stakeholder, project consumer, or methodology author surfaced the finding through prose feedback rather than a structured review — dollspace.gay's `message-4.txt` evaluation of ITC, mined in Review 51, is the canonical example); `mixed` (Review 68 Finding 9 extension) when the round's findings span more than one of the above sources. The Source field gives audit-trail granularity to the Portfolio Assessment dimensions on developer participation; a project whose findings cluster heavily in `director-raised` or `external-feedback` is a different developer-engagement profile than one whose findings cluster in `domain-raised`.

  **`mixed` Source sub-disposition schema ([Review 93](review-log/2026-05-24-suite-review.md#review-93--2026-05-24-2340z) Finding 4):** when the Source value is `mixed`, the Source line MUST name the per-finding-range sub-disposition in a canonical agent-greppable form. Required pattern:

  ```
  **Source:** mixed; `<source1>` for finding-range `<N>-<M>`; `<source2>` for finding-range `<P>-<Q>` [; `<sourceN>` for finding-range `<...>`...]
  ```

  Examples:

  - `**Source:** mixed; \`domain-raised\` for finding-range 1-3; \`director-raised\` for finding-range 4-5`
  - `**Source:** mixed; \`external-feedback\` for finding-range 1; \`domain-raised\` for finding-range 2-4`

  Agent grep idiom for mixed-source attribution: `grep -A 1 '^\*\*Source:\*\* mixed' vsdd-suite/suite-development/review-log/*.md` returns each mixed-source line with the sub-disposition attribution parseable by splitting on `; `. Finding-range form is `N` (single) or `N-M` (range, inclusive). Empty / non-canonical sub-disposition forms (e.g., free-form prose; mixed without sub-disposition; em-dash separator) are non-compliant per [Review 93 Finding 4](review-log/2026-05-24-suite-review.md#review-93--2026-05-24-2340z). The canonical separator is `; ` (semicolon-space) — NOT em-dash, per the [No-em-dash-inside-bold-sub-headings rule](#agent-api-surface-review-80-finding-3) (the Source value extends inside the bold preamble field; em-dash inside it would also trigger the hook's finding-header regex). Forward-only: pre-2026-05-24 mixed-Source entries (e.g., [Review 88 Source: mixed](review-log/2026-05-21-suite-review.md#review-88--2026-05-21-1330z)) preserved as authored per [G-89](FINDINGS-INDEX.md#g-89); new mixed-Source entries use the canonical form.

**Optional, only when applicable to the domain:**
- **Posture:** adversarial framing (Red Team)
- **Program phase:** apprentice phase context (VDD-IAR Alignment)
- **Reference:** non-DESIGN.md authoritative source the review evaluates against (Solution Owner reviewing against the assignment brief)
- **Regression check:** prior-review verification (any domain when a prior review for the same scope exists)
- **Assumption surfacing:** dependency and library-API verification (Quality Engineer, per the QE prompt's G-20/G-21/G-23 obligations) — one short paragraph per review naming assumptions verified or flagged
- **Supplements applied** ([Review 91](review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) Findings 2 + 4): required when the domain prompt references one or more supplements (per [`../supplements/`](../supplements/)) or when the review surface materially engages a language / interface / tool that has a supplement. Plural form — one entry per applied supplement; each entry inline-links the supplement file path + names the section consulted. Format: `**Supplements applied:** [\`rust.md\`](../supplements/rust.md) § Solution Architecture; [\`json.md\`](../supplements/json.md) § Storage / cross-version compatibility — applies because the L2 surface extends a serde-serialized data model with downgrade semantics.` Explicit opt-out form when a supplement-citing domain runs against a surface the supplement does not apply to: `**Supplements applied:** [\`rust.md\`](../supplements/rust.md) § Software Engineering; [\`json.md\`](../supplements/json.md) not applicable — the L3 export surface adds no new JSON-serialization-bearing fields beyond the existing storage format.` Silent omission of an applicable supplement is itself a finding for the next AI Engineer or TW round per [Review 91 Finding 3](review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) (the github-actions.md silent-non-use canonical case). Replaces the prior prose-only "the X supplement § Y floor raised every finding below" template form ([Review 91 Finding 2](review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) evidence) with a parseable + clickable surface so agent grep idioms (`grep -B 3 '^\*\*Supplements applied:\*\* \[.*rust\.md' vsdd-suite/review-log/`) work. Forward-only: applies to entries authored 2026-05-24 and later; pre-2026-05-24 entries preserved as authored per [G-89](FINDINGS-INDEX.md#g-89).
- **Cost-tally** (capstone + production intent; multi-agent cycle-closing entries — [AI Engineer R1 F6](../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-21-ai-engineer.md)): after-action cost report closing the pre-cycle declaration authored per [`primers/3-review-session.md`](../primers/3-review-session.md) § Pre-cycle methodology check. One short paragraph naming: total agent-spawns this cycle; estimated total token consumption (Anthropic API usage as observed or estimated from per-agent context-load size × turn count); per-substantive-finding token cost (total ÷ non-Hallucinated finding count); rate-limit-hit events (none / count + retry shape); model-selection actual-vs-declared (matched / drifted with rationale). The pair (pre-cycle declaration → after-action cost report) is the AI Engineer Dim 13 pre-cycle methodology check applied at the cycle boundary; the audit-trail-stays-honest-without-it discipline is what the cost-tally field defends. Cycles exempt from the pre-cycle declaration (single-agent rounds; sub-agent delegation for non-adversarial work; learning-exercise intent) are also exempt from the cost-tally field.

  **Cost-tally per-domain scope ([Review 92](review-log/2026-05-24-suite-review.md#review-92--2026-05-24-0419z) Finding 5; codified 2026-05-24 via operator-policy Hybrid path — Path 1 codification + earned-by-recurrence trigger):** the cost-tally surface is **AI Engineer-owned** per [Review 87 Finding 6](review-log/2026-05-21-suite-review.md#review-87--2026-05-21-1230z) per-error-class owner table. Per-domain prompts for other roles (SE / QE / SA / Security / Red Team / SO / TW / Doc Reviewer / UX / Accessibility / Privacy / Localization / Data Engineer / Platform Engineer / Performance Engineer / VDD-IAR Alignment / Portfolio Assessment / Sanity Check) have **no expected interaction with cost-tally evidence** beyond reading the cost-tally when scanning a Review entry. A finding from a non-AI-Engineer domain that references cost-tally evidence is an unexpected cross-cut; the methodology does not currently codify the cross-domain interaction shape.

  **Earned-by-recurrence trigger for cross-domain extension:** if a non-AI-Engineer finding routes substantively to cost-tally evidence (e.g., a PerfEng finding about slow agent execution citing tokens-per-finding; a Security finding about prompt-cache leakage citing cache-hit ratio; a SO finding about over-investment citing findings/100k metric; a PE finding about rate-limit exhaustion citing rate-limit-window-utilization) in **2 cycles within a 90-day window**, escalate to a methodology amendment cycle that codifies the cross-domain interaction shape — add cost-tally-interaction sub-clauses to the relevant domain prompts naming the lookup pattern + the routing path back to AI Engineer for cost-tally-owned remediation. Per [Review 91 Finding 10](review-log/2026-05-23-suite-review.md#r91-f10) tuning lever catalog, the tuning levers ARE cross-domain (model-tier selection touches SE / QE; prompt-cache touches Security; cluster-batching touches AI-Eng + PE) — the trigger fires when these cross-domain interactions become empirically visible in finding bodies rather than only in the tuning lever catalog's prose. Currently zero recurrence (per the [Review 92 Finding 5](review-log/2026-05-24-suite-review.md#review-92--2026-05-24-0419z) mechanical-sweep evidence: 4 files reference cost-tally; all 4 are AI-Engineer-surface artifacts).

A reviewer who finds they need a preamble field that is not in either list should propose adding it to this standard rather than introducing it ad-hoc. Examples of fields that are **not** valid additions: `Preamble`, `Governing methodology`, `Mutation analysis method`, free-form `Test count` lines — these duplicate `Scope` or `Session note`, or belong inside individual findings or the closing summary. The ownership / validation lifecycle fields (`Owner`, `Status`, `Blocked by`, `Validator`) live in the per-finding body, NOT the entry preamble — they describe per-finding state, not per-entry state.

### Required pre-review sections

Some domains require an output that must be produced before findings can be classified. These match the "Required sections" carve-out in the domain-file governing standard. The output is a preamble record, not a classified finding.

| Domain | Required section | Source |
|---|---|---|
| Security | `### Threat Model` | `SECURITY-REVIEW.md` prompt |
| Solution Owner | `### Compliance Table` | `SOLUTION-OWNER-REVIEW.md` prompt (assignment compliance audit) |

When a domain prompt file specifies a required section, the project-level review log must contain that section above the finding sections, populated for the round being logged. An absent required section is itself a finding for VDD-IAR Alignment dim 1.

### Finding sections

Group findings by classification heading. Use only the classifications listed for this domain in the **Finding classification schemas by domain type** table elsewhere in this document — no ad-hoc variants (`### Dismissed with Rationale`, `### Observation`, `### Note`, `### Raised to SO`). The classification universe per domain is the source of truth; the log structure must mirror it.

Empty classification sections use a `*(none)*` placeholder so the structure is visible:

```
### Open

*(none)*
```

A round that closes with no `### Open` section is a structural error — the absence of open findings is itself a state worth recording. The set of section headings should equal the domain's full classification set, with `*(none)*` used wherever empty.

**Single-section-per-classification rule ([Review 93](review-log/2026-05-24-suite-review.md#review-93--2026-05-24-2340z) Finding 2):** each classification heading appears **once** per Review entry. When findings are raised + immediately resolved in-cycle (the canonical raise-then-resolve shape), they go in the SAME `### Resolved` section as other Resolved findings — not in a separate `### Resolved (continued — ...)` heading. The "continued" form (used at [Review 91 entry](review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) and pre-codification [Review 92 entry](review-log/2026-05-24-suite-review.md#review-92--2026-05-24-0419z)) is **structural drift** — accumulates prose in the heading + makes the agent-API grep idiom (`grep "^### Resolved$"`) ambiguous when multiple sections exist. Pre-2026-05-24 entries with "continued"-form sections are preserved per [G-89](FINDINGS-INDEX.md#g-89) forward-only narrative-preservation; new entries use single-section-per-classification. If a Review entry's finding-count exceeds the F19 300-line target, split by lens-cluster into multiple Review entries (Review N.1 / N.2 / ...) rather than splitting the classification sections within one entry.

**Exception — Portfolio Assessment:** Portfolio Assessment groups by dimension (`### Dim N — Name`), not by classification, because each portfolio dimension produces a per-dim assessment (`Demonstrated`/`Partial`/`Absent`/`Hallucinated`) rather than a defect to fix. The classification appears at the end of each dim section and a summary table appears at the close. Portfolio also adds a file-level `**Developer participation note:**` directly under the sycophancy check, naming which dimensions require direct developer interrogation rather than artifact analysis. These exceptions are intentional and limited to Portfolio Assessment; no other domain may use dim-first organization or the participation-note field.

### Finding body

Each finding follows this structure:

```
**Finding N — Title (Dim X)**

**Owner:** [domain slug]                              ← Review 77; required for non-Hallucinated findings
**Status:** [raised | assigned | fix-landed | validated]   ← Review 77; required for non-Hallucinated findings
**Blocked by:** [cross-domain anchor reference]       ← Review 77; optional; *(none)* placeholder when no blockers
**Validator:** [domain slug OR *self* — <rationale>]  ← Review 77; required for Resolved findings

[Prose body — what was observed, why it matters, evidence]

**Resolution:** [for Resolved findings — what was changed and where]

— or —

**Classification:** [Dismissed | Deferred | Hallucinated | Accepted Risk | …]
[rationale; for Accepted Risk and similar, include the named owner]
```

- Finding title always includes the dim reference parenthetically (`(Dim 2)`, `(Dim 1, Dim 10)`, `(Rust supplement — path traversal)`, `(Phase 5 Mutation Testing)` for Phase 5 work). Any trailing `(...)` group at the end of the title is the discipline-reference parenthetical; the per-project-review hook accepts any form per Review 74.
- Numbering is continuous within a Review (1, 2, 3, … across all classifications), not restarted per classification
- Cross-references to other domain logs use Markdown links: `[QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md) Finding 4` — not prose ("Logged in QE log")
- Closer is exactly one of `**Resolution:**` (Resolved only) or `**Classification:**` (everything else). Mixing the two within a single domain's log is drift.
- The four lifecycle fields (Owner / Status / Blocked by / Validator) appear in that order at the top of the finding body, before the prose. Hallucinated findings are exempt (no Owner, no Validator — the finding didn't apply, so the lifecycle doesn't apply). Forward-only per G-89: applies to findings dated 2026-05-21 or later (Review 77 adoption cutoff). See § Validation loop discipline below for the lifecycle rules + strict self-validation policy.

### Validation loop discipline (Review 77)

The lifecycle fields above implement a four-axis ownership / blocking / validation model:

**Owner** — the domain accountable for the finding's resolution. The default is the raising domain (self-owned); when a finding routes to another domain (typically via Phase 4 routing or a `Raised to SO` sub-heading), `**Owner:**` updates to the receiving domain. Multi-domain ownership is itself a finding for VDD-IAR Alignment — it usually means routing was incomplete. The `### Raised to SO` sub-heading is preserved as a shorthand for `**Owner:** solution-owner`; either form is accepted by the hook.

**Status** — sub-state within the Open / non-terminal lifecycle:

| `**Status:**` value | Meaning | Next transition |
|---|---|---|
| `raised` | Finding registered; owner-acceptance pending | `assigned` (owner accepts) OR `dismissed`/`hallucinated` (terminal) |
| `assigned` | Owner accepted; fix in progress | `fix-landed` (after fix commit) OR back to `raised` (owner declines, routes back) |
| `fix-landed` | Fix committed; validation pending | `validated` → terminal Resolved, OR `validation-failed` → re-opens with validator's sub-finding |
| `validated` | Validator's cold pass confirmed | Terminal — moves to `### Resolved` classification |

Terminal classifications (Resolved / Dismissed / Hallucinated / Accepted risk / etc.) make `**Status:**` redundant — omit it on terminal findings except for Resolved findings, which carry `**Status:** validated` to record that the validation pass completed.

**Blocked by** — names other findings that must close first. Format: `[Domain Review N Finding M](path-anchor)` — same cross-reference form used elsewhere in the project's review logs. The hook (`check-project-review-discipline.py`) treats a Blocked-by reference whose target is still Open as a block on closing the dependent finding. `*(none)*` placeholder when no blockers exist.

**Validator** — the domain that cold-re-reviews to confirm the fix lands clean. For Resolved findings only; omit the field on non-terminal findings. Natural-pair defaults are documented in each domain prompt's `## Current Review Prompt` section (Validator-pair paragraph added in Review 77). For findings without a natural cross-domain pair, the recommended path is `**Validator:** sanity-check` (the meta-validator-of-last-resort — see Sanity Check meta domain below); `*self*` remains valid WITH a substantive rationale for cases where the work has no spec/architecture interface at all.

**Sanity Check meta domain (Review 77 Finding 2).** `domains/meta/SANITY-CHECK-REVIEW.md` registers a meta domain whose primary purpose is to validate findings that have no natural cross-domain pair (PE shift-left mechanizations; SA architecture-doctrine findings without Raised-to-SO routing; QE test-discipline meta-findings; Portfolio Assessment introspective dimensions; Security findings with no Red Team validation surface; TW findings pre-Doc-Reviewer-domain-registration). Sanity Check applies DESIGN.md + architecture context to confirm the resolution coheres with the spec. The domain's secondary purpose is rubber-ducking — a dedicated session type for developers working through problems whose solution emerges in articulation. Sanity Check uses the meta-domain classification universe (Resolved / Dismissed / Hallucinated — no Deferred); validator-of-last-resort sessions read the originating finding + resolution + DESIGN.md context and either validate or re-open with the gap surfaced; rubber-duck sessions end with explicit closure (insight-reached or insight-not-reached with the next session's purpose named).

**Strict self-validation policy.** The hook fails any Resolved finding with `**Validator:** *self*` that lacks a substantive rationale on the same line or the next. Acceptable rationales name WHY no cross-domain validator AND no Sanity Check validation applies (Sanity Check itself raising a finding owned by Sanity Check; truly meta-meta cases). Placeholder rationales (`TBD`, `N/A`, `no pair available` without further specificity) fail. **Domain-level allowlist:** retired as of Review 77 Finding 2 (Sanity Check supersedes the prior Portfolio Assessment blanket allowlist). The `SELF_VALIDATION_BLANKET_ALLOWLIST` set in the hook is now empty by default; future domain additions whose introspective work cannot be sanity-checked structurally would be added there.

**Why strict instead of soft-warn.** Self-validation is the seam where the validation-loop discipline degrades fastest. Soft-warn surfaces the warning in stderr; the author dismisses it once and proceeds. Strict makes the rationale a per-finding artifact future reviewers can audit — AND forces the author to ask "is this actually self-validation, or am I about to skip the cross-domain validator out of friction?" The friction cost is one sentence per legitimate self-validation; the discipline gain is that every self-validation is reasoned, not defaulted-to.

**Owner-field qualifier choice.** Owner is a single domain slug; layer/scope qualifiers are NOT used (`**Owner:** software-engineer @ Layer 3` is invalid). The Layer column in the project FINDINGS-INDEX registry already provides cross-cutting layer filtering; the per-finding narrative makes the layer-specific context explicit when needed. Adding `@ Layer N` to Owner conflates two orthogonal axes (who owns vs. which layer) and creates redundancy with the Layer column.

**Forward-only constraint:** the four lifecycle fields apply to findings dated 2026-05-21 or later (day-after-Review-77-adoption cutoff). Pre-cutoff findings in any project (including the existing 3 bookmark-cli-manual rounds dated 2026-05-17 + 2026-05-20) are NOT migrated by the hook's enforcement. The reference examples (`bookmark-cli-manual/` + forthcoming `bookmark-cli-crosslink/`) MAY migrate as part of their capstone-intent promotion under the G-177 precedent — reference examples are kept current with the conventions they teach — but the migration is a deliberate per-project decision, not a hook requirement.

### Closing block

Each review ends with:

1. **`### Summary`** — one short paragraph: tally of findings by class and what the round produced. Required.
2. **`**Coordination:**`** line — handoffs to other domain logs with linked relative paths to the receiving file. Required when any finding references another domain; `*(none)*` otherwise.

Reviews without a Summary are incomplete. Reviews with cross-domain implications and no Coordination line have an unrecorded handoff.

### Round numbering and ordering

- Reviews are numbered per domain (Review 1, Review 2, …) and ordered chronologically with most recent at the bottom
- Each `## Review N` is separated from the next by a double horizontal rule (`---\n\n---`)
- Round numbers are required in every entry (`3-review-session.md` reinforces this for the merge gate)

---

## Before adding a dimension

1. Name the failure class: what defect reaches users if this dimension is absent?
2. Check whether an existing dimension in any domain already owns this failure class. If it does, flag the gap there rather than adding cross-domain overlap.
3. Check the findings registry (`FINDINGS-INDEX.md`). If the finding is tracked, cite its anchor (legacy `G-XX` for pre-2026-05-20 entries; `Review N Finding M` for forward-only entries) in the CHANGELOG entry for this change. If it is not tracked, add it and immediately mark it Resolved.
4. Write the dimension using the standard form: failure class in bold, explanation, named failure modes or attacks.

## Before adding a domain

1. Name the defect class the domain would catch that no existing domain catches.
2. Evaluate the role-based taxonomy: what job title does this reviewer hold? Is it a job a real person would have at a real company?
3. Decide: core domain (every project) or extended domain (conditional on project type)? Document the activation criteria. Add the domain to `../domains/DOMAIN-INDEX.md`.
4. Create the domain file following the governing standard above. Verify the finding classification schema is appropriate for the domain's nature.
5. Add the domain to `README.md` in the appropriate table (Core or Extended), with Role, Job title, Prompt file, and Focus columns filled.
6. Add language and interface supplement sections where applicable. See **Supplement coverage** below.
7. Add a suite review session entry in `review-log/YYYY-MM-DD-suite-review.md` (creating the file if no entry exists for that date) documenting the addition, rationale, defect class addressed, and dimensions that were considered and rejected. Add a corresponding row to the **Suite Reviews** table in `SUITE-DEVELOPMENT-REVIEW.md` linking to the entry.
8. Add a `CHANGELOG.md` entry.
9. Add a `FINDINGS-INDEX.md` entry if the domain addresses an existing open finding.

## Before modifying a domain

1. State what defect the current version fails to catch — name a specific scenario where the current prompt produces a false pass.
2. Make the change.
3. Update the findings registry: if the finding was tracked, mark it Resolved (forward-only registry) or Addressed (legacy registry) with today's date. If it was not tracked, add it and immediately mark it Resolved/Addressed per the registry section it lands in.
4. Log the change in `CHANGELOG.md`.
5. If the change is structural (new section, new classification schema, changed prompt format): add a suite review session entry in `review-log/YYYY-MM-DD-suite-review.md` and a corresponding row in the **Suite Reviews** table in `SUITE-DEVELOPMENT-REVIEW.md`.

## Walking the findings registry

Read `FINDINGS-INDEX.md` for the current open findings (both registries — forward-only and legacy), then read all domain files and evaluate whether each open finding has been addressed by recent changes. Follow the instructions at the top of that file.

**Registry discipline:** When a finding is resolved by a suite change, update the original row's status in place — change `Open` to `Resolved`/`Closed` (forward-only registry) or `Open` to `Addressed` (legacy registry) and update the `Last Reviewed` date. Do not append a new row for an existing finding. New findings get new rows; status changes update existing rows.

A registry-walk session is one mode of suite review (registry-walk lens). Like any suite review session, it ends with:
- All recently resolved findings updated in `FINDINGS-INDEX.md` with the date
- Any new findings discovered added to the **Findings registry (forward-only)** section, identified by their originating `Review N Finding M` anchor (no `G-`/`F-` ID prefix — the legacy `G-` series is closed)
- A `## Review N — date` entry in `review-log/YYYY-MM-DD-suite-review.md` (creating the file if no entry exists for that date) summarizing scope, findings, decisions, and suite changes made
- A corresponding row added to the **Suite Reviews** table in `SUITE-DEVELOPMENT-REVIEW.md` linking to the new entry

`FINDINGS-INDEX.md` contains only the registry tables. Suite-review narratives belong in `review-log/`. `SUITE-DEVELOPMENT-REVIEW.md` is the index of those narratives, not their home.

## Suite review and review-log discipline

The VSDD Suite has three parallel review-record artifacts. Their roles do not overlap:

- **`SUITE-DEVELOPMENT-REVIEW.md`** is an index. It contains one table — **Suite Reviews** — each row pointing to a session entry in `review-log/`.
- **`review-log/YYYY-MM-DD-suite-review.md`** holds the actual session entries. One file per date; multiple sessions on the same date append to the same file (newest at the top).
- **`FINDINGS-INDEX.md`** is the findings registry. Status only — no narrative. One row per finding; status changes update the row in place. Two sections: forward-only registry (no ID prefix; identified by `Review N Finding M` anchor) and legacy registry (`G-01–G-182`, closed to new entries, preserved as historical anchors).

Every non-trivial suite change requires a session entry in `review-log/` and a corresponding index row in `SUITE-DEVELOPMENT-REVIEW.md`. Non-trivial means: any addition or removal of a domain or primer, any new evaluation dimension, any structural change to the prompt format, or any change to sequencing or activation guidance.

Mechanical fixes (typos, filename renames, path updates) do not require a session entry but should be logged in `CHANGELOG.md`.

**One artifact type, multiple modes.** A suite review may apply fresh adversarial pressure (defect-search lens), walk the findings registry top-down (registry-walk lens), or both. The mode lives in the **Lens** field; it is not a separate artifact type. Sessions previously called "meta-reviews" and "gap analysis runs" are now both `Review N` entries — the distinction is mode, not kind. (The "gap analysis run" framing is retired; the underlying mode is preserved as the `registry-walk` lens.)

### Filename convention

The filename date is the **session start date in UTC**. When a session crosses midnight UTC, it remains in the file matching its start date — do not split it across two files. Same-date sessions append to the existing file (newest at the top).

**File-size threshold + part suffix (Review 69 amendment).** When a same-date file would exceed **80 KB** OR **15 review entries**, split it into `-partN.md` suffixed parts (e.g., `2026-05-19-suite-review-part1.md`, `2026-05-19-suite-review-part2.md`). Each part holds a contiguous run of reviews (oldest at the top within each part, parts numbered by chronological order); both parts get an H1 of the form `# Suite Review — YYYY-MM-DD (part N of M)` and a navigation note linking to the sibling part(s). The split is mechanical (not topical) — the rule is file-size / review-count, not narrative cohesion.

**Why the threshold exists:** Markdown parsers (tree-sitter; IDE language servers; markdown linters) hit parse-time budgets on dense markdown files with many inline code spans, long paragraphs, and nested tables. Suite review entries contain all three patterns. A long review-cycle day (8+ verbose reviews) produces a file that exceeds the parse-time budget and shows up as "parser aborted" / "parser timed out" diagnostics in operator IDEs. The threshold is empirical (the 2026-05-19 file hit parser-aborted at ~128 KB / 8 reviews of recent verbose style); 80 KB / 15 reviews leaves headroom for both denser and longer reviews.

**Cross-reference rule.** When a file is split, any forward-facing artifact that cited the original by Markdown link must update the link target to the correct part. The `SUITE-DEVELOPMENT-REVIEW.md` Reviews table gets one row per review pointing at the part the review lives in. `FINDINGS-INDEX.md` gap-row anchor citations get rewritten to the part file. CHANGELOG / COMPATIBILITY entries describing the affected reviews update their prose references to the part filenames. Historical-narrative file references (older CHANGELOG entries describing the original file's creation, etc.) stay per G-89 forward-only.

**Forward-only:** the part-suffix rule applies to files split on or after 2026-05-20 (Review 69 amendment date). Existing single-file days that have not yet hit the threshold remain single-file; the rule kicks in only when the file would exceed it on the next append. A file already over threshold gets retroactively split (as `2026-05-19-suite-review.md` was at Review 69). The split decision is a per-file event, not a portfolio-wide migration.

### Suite review entry format

A `## Review N — date` entry in `review-log/YYYY-MM-DD-suite-review.md` must contain:

1. **Header** — `## Review N — YYYY-MM-DD HH:MMZ`. Review numbers are **sequence-wide across all suite-review files** (Review 30 follows Review 29 even if they live in different date-named files); the timestamp is the session start in UTC.
2. **Scope** — What artifacts were read this round (specific domain files, primers, supplements, README, findings registry rows, etc.) and what triggered the review (user request, follow-up to a prior finding, scheduled cadence, project type added). Cite specific files when narrow; "all 14 role domains, 2 meta domains, 5 primers" when broad.
3. **Lens** — The angle the reviewer applied. Valid forms:
   - A **named defect class** ("coordination link format compliance", "classification schema coverage", "lang supplement symmetry").
   - A **registry-walk scope** ("walk all open findings", "review G-22 and G-30", "walk forward-only Open entries").
   - A **role-based lens** that applies one or more domain perspectives to the suite as artifact ("Solution Owner + Technical Writer + VDD-IAR Alignment"), or a named bundle of complementary defect-class lenses applied serially or in parallel ("five lenses applied serially — clarity, naming, ambiguity, consistency, transitional-state alignment").

   A diffuse lens produces a diffuse review. If a session has no specific lens, log it as a generalist pass and name the prior passes' specialization gaps it is filling.
4. **Findings** grouped by classification heading. Valid headings mirror the project-level finding classification universe so a suite contributor and a suite user encounter consistent conventions across scopes:
   - `### Resolved` — fix applied and verified during the session. Use both for newly-found defects fixed in-session and for previously-tracked findings closed in-session (cite the anchor — legacy `G-XX` for pre-2026-05-20 entries; `Review N Finding M` for forward-only entries).
   - `### Dismissed` — concern reviewed and rejected; rationale required. Use both for newly-raised defects rejected and for previously-tracked findings dismissed.
   - `### Hallucinated` — adversary-invented concern that does not apply; rationale required.
   - `### Open` — finding registered as tracked work but not closed in-session; the registry row is added (forward-only section) or updated (legacy section) in `FINDINGS-INDEX.md`. Forward-only entries are identified by the originating `Review N Finding M` anchor; legacy entries continue to use their `G-XX` anchor.
   - `### Deferred` — finding registered against a named reactivation trigger; cost-of-deferral, trigger condition, and auto-Backlog clause all named per the G-130 deferral discipline. Registry row added (forward-only) or updated (legacy).

   `### New gap registered` is **retired** as of 2026-05-20 (Review 73). Existing session entries that used this heading remain valid as historical records per the forward-only narrative-preservation policy; new entries use `### Open` / `### Deferred` per the project-aligned classification universe.
5. **Finding body** — same shape as project-level review logs: `**Finding N — Title**` for findings (whether newly-resolved in-session or newly-registered for tracking); `**G-XX — Title**` is the accepted heading form for legacy-registry walks (re-walking an entry registered before 2026-05-20). Prose body; then `**Resolution:**` (Resolved) or `**Classification:**` (everything else). Cross-references to other suite artifacts use Markdown links.
6. **Closing** — no separate Summary required (the classification headings carry the tally). An optional `### Coordination` section may follow the classification sections when findings cluster around a single coordinated decision; use it to name the cluster and the bundled action (e.g., a single restructure pass at a future trigger). Cross-references inside the Coordination section use Markdown links to other suite artifacts. Follow-up findings introduced after the session has been logged must be marked `**Finding M — Title (added YYYY-MM-DD)**` and placed at the end of the original entry, not in a new entry. Do not silently amend prior findings.

### Session isolation

Suite reviews are typically conducted in the same session as the user's pre-flight discussion or the suite changes the review evaluates. Unlike domain reviews — where a cold, isolated session is the gold standard — suite reviews benefit from continuity with the suite's authorial context. The compensation is documented per-entry: each `**Session note:**` line names the session-isolation status and explicitly acknowledges sycophancy risk when in-session.

A cold-session suite review is permitted and produces stronger adversarial pressure on suite artifacts; it is not required. The minimum standard is that the session note explicitly states whether the session is cold or in-session and, if in-session, names a compensation (e.g., findings derived from artifact-state analysis rather than narrative judgment, independent evaluation of any user-named candidates against the lens criteria). A suite-review entry whose session note omits this acknowledgement is a structural error — the missing acknowledgement is itself a finding for VDD-IAR Alignment dim 7 (cross-session spec consistency) applied to the suite.

### Common discipline

The session entry is the narrative record. The `FINDINGS-INDEX.md` row is the status indicator for gaps. The `SUITE-DEVELOPMENT-REVIEW.md` row is the index pointer for the session. Never put narrative in the registry; never omit the registry update; never omit the index row. An unindexed session is invisible to future reviewers.

### CHANGELOG slim-form convention ([Review 93](review-log/2026-05-24-suite-review.md#review-93--2026-05-24-2340z) Finding 1)

`vsdd-suite/CHANGELOG.md` Unreleased entries are **version-diff records**, not content surfaces. The canonical narrative for each PR lives in the suite-side review-log entry that drove the PR's substantive changes; CHANGELOG entries link to that narrative rather than duplicating it. Entries authored 2026-05-24 and later use the slim-form shape:

```
## Unreleased — YYYY-MM-DD HH:MMZ (PR #N — one-sentence scope; links to Review N entry as canonical narrative)

Per [Review N entry](suite-development/review-log/YYYY-MM-DD-suite-review.md#review-N--YYYY-MM-DD-HHMMZ): one-sentence description of what this PR codified + named operator-policy decisions if any.

### Added (1-3 short bullets max)

- File or section name → 1-line description with link to canonical narrative

### Changed (1-3 short bullets max)

- File or section name → 1-line description with link to canonical narrative

### Cost-tally — see [Review N entry](suite-development/review-log/YYYY-MM-DD-suite-review.md#review-N--YYYY-MM-DD-HHMMZ) cost-tally section
```

**The full prose belongs in the review-log entry, not in the CHANGELOG.** Prior entries (pre-2026-05-24 — including the multi-paragraph PR #45 / PR #48 / PR #50 / PR #51 entries) carrying 30-70-line detailed Added/Changed/Pending sub-sections are preserved per [G-89](FINDINGS-INDEX.md#g-89) forward-only narrative-preservation; new entries + amendments to existing entries use the slim-form.

**Why:** parallel to [§ SUITE-DEVELOPMENT-REVIEW row slim-form convention](#suite-development-review-row-slim-form-convention-review-91-finding-18) below. CHANGELOG triple-encoded the same information that lives in the review-log entry + the SUITE-DEVELOPMENT-REVIEW row; updating one without the others created drift; the maintenance cost was operator-visible across PRs. Slim CHANGELOG entries point at the review-log entry as the single source of truth for the PR's narrative. The CHANGELOG retains its version-diff function (CI cross-version comparison; release notes; cross-cycle changelog scanning) without duplicating the narrative.

**Operator action when authoring a new CHANGELOG entry:** write the full narrative in the review-log entry's body + § Summary; render the CHANGELOG entry as a short version-diff record pointing at it.

### SUITE-DEVELOPMENT-REVIEW row slim-form convention ([Review 91](review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) Finding 18)

The `SUITE-DEVELOPMENT-REVIEW.md` Reviews-table row is an **index pointer**, not a content surface. Rows authored 2026-05-24 and later use the slim-form shape:

```
| Review N | YYYY-MM-DD HH:MMZ | [file](path#anchor) | One-sentence lens + finding-count tally (`N Resolved / N Open / N Dismissed`). Operator pointer to the review-log § Summary for the full narrative. |
```

**The full prose summary belongs in the review-log entry's `### Summary` section, not in the Reviews-table row.** Prior rows (pre-2026-05-24) carrying 500-3000-word per-row prose summaries are preserved per [G-89](FINDINGS-INDEX.md#g-89) forward-only narrative-preservation; new rows + amendments to existing rows use the slim-form.

**Why:** Reviews 84-90's per-row prose summaries duplicated the same content authored in the review-log entry's `### Summary` section. Updating one means updating the other; drift is inevitable; the maintenance cost is operator-visible. Parallel to the [Review 84 Finding 2 per-domain-index retirement](review-log/2026-05-21-suite-review.md#review-84--2026-05-21-1100z) — the same redundancy-evaluation directive applies here: the index row is the index pointer; the review-log entry is the narrative source-of-truth.

**Operator action when authoring a new Review:** write the full narrative in the review-log entry's body + § Summary; render the SUITE-DEVELOPMENT-REVIEW row as one-line slim-form pointing at it.

### Per-Review entry size discipline ([Review 91](review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) Finding 19)

Per-Review entries in `review-log/YYYY-MM-DD-suite-review.md` should target **under 300 lines** including preamble + findings + summary + coordination. Entries that exceed this should split into multiple `Review N` entries (e.g., Review N.1 / N.2 / N.3) OR summarize aggressively rather than authoring every finding in full inline.

**Why:** A reader (human or agent) scanning the review-log for a specific finding pays cognitive cost proportional to entry size. Entries past 300 lines cross a usability threshold; per-finding context becomes harder to locate; cross-cycle reading slows. The 80KB file-split rule (Review 69) bounds whole-file size but not per-entry size; this discipline bounds per-entry size to keep individual entries scan-fast.

**When splitting is appropriate:** a single Review's findings cluster around multiple distinct concerns (e.g., Review 91 spans audit + cost-observability + three-audience-effectiveness + structure-slop — four lens-clusters). Split by lens-cluster rather than authoring 500+ lines under one Review heading. Each split inherits the Review number with `.M` suffix; cross-references resolve to the most specific anchor.

**When summarizing is appropriate:** a Review with 8+ findings each warranting full body authoring exceeds the 300-line bound regardless of split. Summarize findings to ~30 lines each + link to per-finding extended-analysis files for the load-bearing ones. The `### Summary` section's role inverts: from "tally of what's above" to "navigation surface across what's elsewhere."

**Hook escalation path (deferred per "earned by recurrence"):** if a third Review entry exceeds 300 lines after this codification lands, escalate to a pre-commit hook that flags oversized entries + names the operator-action queue item ("split or summarize"). Mechanizing now against one recurrence is over-investment.

**Forward-only:** the bound applies to entries authored 2026-05-24 and later; existing oversized entries (pre-codification) remain as historical record per [G-89](FINDINGS-INDEX.md#g-89).

### Cost-tally opt-in shape ([Review 91](review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) Finding 20)

Per [Review 90 Finding 4](review-log/2026-05-23-suite-review.md#review-90--2026-05-23-1200z) the cost-tally schema requires 10 fields covering AI tool / plan tier / execution method / model / raw tokens / would-be API cost / actual cost / rate-limit utilization / wall-clock / findings per 100k tokens. Per [Review 91 Finding 8](review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) the schema is tiered by what's measurable from where (agent-self-verifiable / operator-verifiable / operator-confirmable). The full tiered schema is ~25-30 lines per Review.

**Opt-in shape (Review 91 Finding 20 codification):**

- **Full tiered cost-tally REQUIRED** for capstone+ intent multi-agent cycles (>4 agent-spawns per cycle); cluster-batching cycles; any cycle whose cost is load-bearing for cross-cycle calibration. Per [`primers/3-review-session.md`](../primers/3-review-session.md) § Pre-cycle methodology check.
- **Minimal cost-tally OPTIONAL** for inline single-author Review entries (no sub-agent spawns; mechanical sweep + analysis only). Acceptable minimal form (4-5 lines):

```
**Cost-tally (minimal):**

- **AI tool / Model / Execution method:** claude-code CLI / claude-opus-4-7 / inline main session
- **Date:** 2026-05-23
- **Wall-clock anchors (Bash `date -u`):** session-start [not captured] → session-end 2026-05-24 02:43Z
- **Files touched:** <N> read + <M> edited (substrate-countable; not exhaustively enumerated)
- **Operator-action queue:** if cost-tally precision becomes load-bearing, operator runs `/cost` for full tiered fields
```

- **Cost-tally OMITTED** is acceptable for trivial mechanical-fix reviews (typos; filename renames; path updates) — already exempt from full review-log entry per § Common discipline ("Mechanical fixes ... do not require a session entry but should be logged in `CHANGELOG.md`").

The opt-in shape prevents the cost-tally section from bloating every review entry; the full schema activates where it adds calibration value; the minimal form preserves the schema's signal (what cycle, what tool, what scope) without paying full tier-tracking cost.

**Forward-only:** the opt-in shape applies to entries authored 2026-05-24 and later; existing full-schema entries remain as authored.

### Domain-effectiveness audit shape ([Review 91](review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) Finding 15)

A **domain-effectiveness audit** evaluates whether an active domain in a project's IAR set is producing methodology-justified value or whether it's drifting into noise / over-investment / under-investment. The audit shape comes in two forms — rigorous + thin — both valid for different cycle scopes.

**Rigorous form** — applies when a methodology decision is at stake (intent-tier promotion; domain activation / deactivation; cross-project doctrine change). Required inputs:

1. **Domain prompt loaded into context** ([`domains/role/<DOMAIN>-REVIEW.md`](../domains/role/) or [`domains/meta/<DOMAIN>-REVIEW.md`](../domains/meta/) — not just cited from secondary references per [AI Engineer Dim 11](../domains/role/AI-ENGINEER-REVIEW.md) cite-verify discipline.
2. **All per-session review-log entries citing the domain** read end-to-end (not just grep-counted).
3. **Cross-cutting registry rows** ([`<project>/vsdd-suite/FINDINGS-INDEX.md`](../templates/PROJECT-FINDINGS-INDEX-template.md)) filtered by domain.
4. **Per-cycle cost evidence** from the cost-tally / cost-observability sibling JSON files (per [Review 91 Finding 9](review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) Shape 1 infrastructure).

Required analysis axes:

- **Per-dim coverage:** did findings exercise the dim's named failure modes, or did they cluster around one or two dims? Map findings → dims; flag dims with zero finding-coverage (over-extension signal — the dim may not apply to this project's surface) AND flag dims with disproportionate coverage (under-extension signal — other dims may be missing coverage).
- **Classification ratio:** Resolved / Hallucinated / Dismissed split. Over-investment signal if Hallucinated >50% (the domain is reaching for findings that don't apply); under-investment signal if Resolved <20% (the domain isn't finding actionable defects); right-fit signal when ~60-80% Resolved + ~10-30% Dismissed/Hallucinated combined.
- **Cost-per-finding:** per [AI Engineer Dim 2](../domains/role/AI-ENGINEER-REVIEW.md) expected-band lookup (intent-keyed: learning-exercise ≤50k/finding; portfolio 50k-150k; capstone 100k-300k; production 200k-500k). Out-of-band cost is a calibration signal.
- **Cross-cycle codification rate:** how many findings from this domain became permanent suite improvements (codified into primers / supplements / governing standards)? High rate = high-leverage domain; zero rate = domain may be over-extended at this project's scope OR the methodology is missing the surface the domain is finding.
- **Per-finding quality assessment:** substantive defect (vs methodology-observation vs noise). Subjective but auditable: a finding that closed by adding a test or fixing a behavior is substantive; a finding that closed by adding a comment or renaming a variable is methodology-observation; a finding closed by acknowledging it doesn't apply is noise (or Hallucinated).

Output: a per-domain effectiveness report with each axis assessed + an overall verdict (right-fit / over-extended / under-extended) + recommended methodology action (promote / demote / activate at lower intent tier / deactivate / retire dim N).

**Thin form** — applies when assessing the IAR cycle's discipline-health across many domains (cycle-close summary; periodic check). Inputs reduced to:

1. **Finding count per domain** (mechanical `grep`).
2. **Classification ratio per domain** (`grep` on classification sub-section headings).
3. **Cross-cycle codification recognition** (memory of which findings became suite improvements).

Output: thin-form effectiveness paragraph naming each domain's finding-density + ratio + codification rate, with overall verdict at the cycle level (no per-dim coverage analysis; no per-finding quality assessment).

**When each applies:**

| Cycle scope | Form |
|---|---|
| Cycle-close summary (per layer-gate close); periodic discipline check | Thin form |
| Domain activation / deactivation decision; intent-tier promotion | Rigorous form |
| Cross-project doctrine change (e.g., promoting an extended-pool domain to core) | Rigorous form against 2+ projects |
| External-feedback mining (suite contributor reviews multiple projects) | Rigorous form per project |
| Suite-developer's own audit of methodology calibration | Rigorous form against the reference example(s) |

**Rigorous-vs-thin distinction is load-bearing:** a thin-form audit producing the conclusion "all domains right-fit" without per-dim coverage analysis is acceptable for cycle-close but not for activation decisions. Per [Review 91 Finding 15](review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) the Review 91 own audit produced a thin-form conclusion ("all 13 capstone-active domains substantive; no over-extension") without loading any of the cited domain prompts — the conclusion was correct but thinly grounded. Future audits with activation-decision stakes use the rigorous form.

**Forward-only:** the rigorous/thin distinction applies to audits authored 2026-05-24 and later; pre-2026-05-24 audits preserved as authored per [G-89](FINDINGS-INDEX.md#g-89).

**Hook escalation (deferred per "earned by recurrence"):** if a third audit-cycle produces results inconsistent with prior cycles (a domain rated right-fit in one cycle + over-extended in the next without explicit context-change rationale), escalate to a methodology amendment requiring rigorous-form audits for any activation/deactivation decision.

### External-review-log subfolder pattern ([Review 88](review-log/2026-05-21-suite-review.md#review-88--2026-05-21-1330z))

External-feedback artifacts (Bluesky thread captures; emailed prose feedback; GitHub issue narratives; methodology-author commentary; any reviewer-produced prose received from outside the suite's authoring loop) live in [`review-log/external-review-log/`](review-log/external-review-log/) rather than at the `review-log/` root. The subfolder separation prevents external prose from drifting into the per-date suite-review canonical-pattern (which expects the `YYYY-MM-DD-suite-review.md` shape + the suite-review hook's preamble discipline) and provides a stable destination for reviewer-named archive files.

**Filename convention:** `<date>-<reviewer-handle-slug>.md`. The slug is the reviewer's authored handle (Bluesky / GitHub) — NOT a real name. Lowercase + hyphens + no `@`-prefix + no platform-prefix. One file per reviewer per date. Examples: `2026-05-20-dollspace-gay.md`, `2026-05-21-shimmermathlabs.md`.

#### File structure (standardized)

1. `# External Review — @<handle> — <Date>` (handle, not real name)
2. `## Reviewer` — Handle (+ link) / Pronouns (optional) / Relationship to suite
3. `## Source` — Type / URL / API URL / Captured / Archive provenance / Verbatim attestation
4. `## Scope of what the reviewer addressed` — one paragraph of operator-solicitation context + what the reviewer evaluated
5. `## Verbatim source content` — the prose verbatim; quoted with attribution per post / per message
6. `## Suite-side mining` — cross-reference to the canonical mining-Review + per-finding routing table
7. `## Notes` — operator-context, archiving notes, sycophancy-compensation declarations

#### Identity-correlation discipline (load-bearing)

**The rule: knowability ≠ surfacing.** The suite does NOT correlate identities that the reviewer engaged through different surfaces, even when correlation is knowable (e.g., a Bluesky thread + a GitHub PR signed with a real name + an email visible in git author info — all three may be technically observable; the suite still surfaces only the platform the reviewer used to engage THIS review).

The discipline:

- **Single-platform reviewer:** name only the platform the reviewer engaged on. Bluesky thread → only Bluesky handle. GitHub issue → only GitHub handle. Email-only feedback → handle/pseudonym the reviewer signed with.
- **Multi-platform reviewer, same-identity-string across platforms:** name both platforms only when the handle-string is the same across platforms (e.g., `dollspace-gay` on GitHub + `dollspace.gay` on Bluesky — same identity-name after slug-normalization). This is consistent-identity surfacing, not correlation between separate identities.
- **Multi-platform reviewer, different identity strings:** name ONLY the platform the reviewer engaged on for THIS review. Do NOT name the other platform's identity even when correlation is knowable. The principle: knowability is not surfacing; the suite does NOT surface what the reviewer didn't themselves surface in the engagement.
- **Real-name field:** declared as `**Name:**` ONLY when the name IS the handle (e.g., a reviewer who signs as their first/last legal name uses that). Otherwise omit. The suite never surfaces a real name that differs from the handle.
- **Pronouns:** optional + reviewer-authored. Default to no pronoun field; add the field only when the reviewer has declared them.
- **Email addresses:** never surfaced in the Reviewer or Source preamble. If an email appears in quoted source content (e.g., an emailed feedback artifact), it stays in the verbatim block but is NOT promoted to the preamble's identity-surface.
- **Downstream-artifact cross-references:** PR-number-references, commit-SHA-references, gist-URL-references are linked by their canonical artifact-identifier (PR #41; commit 98ead5b; gist URL). The reader who clicks through reaches the downstream identity-surface on their own; that is the reviewer's authored choice when they filed the downstream artifact, not the suite's correlation work. The external-review file MUST NOT name the downstream-artifact's identity-attribution alongside the engagement-platform's identity (e.g., "this Bluesky reviewer is also @<handle> on GitHub" is exactly the correlation-surfacing this discipline forbids).

**Why:** the operator (and many people who will be reviewing) are marginalized people. Surfacing real names + handles in correlated form has historically been a vector for harm. The single repo-wide pre-commit hook [`check-anonymization.sh`](../hooks/check-anonymization.sh) (wired in `.pre-commit-config.yaml` as `id: anonymization`; consolidated at PR #43 from the previously-separate `no-home-dir-paths` + `review-log-anonymization` hooks; scans every committed text file — not just review-log markdown — for `$HOME` + `git config user.name` + `git config user.email` patterns outside the `github.com/` / `gitlab.com/` / `bitbucket.org/` / `bsky.app/profile/` / `noreply.*` public-URL allowlist) enforces the local-author side of this discipline (protecting the local-user from leaking identity through any committed file). The companion hook [`check-external-review-anonymization.py`](../hooks/check-external-review-anonymization.py) ([Review 88](review-log/2026-05-21-suite-review.md#review-88--2026-05-21-1330z); wired in `.pre-commit-config.yaml`) extends the discipline to external-author content — it parses each file in `external-review-log/`, extracts handle-slug declarations, flags multi-platform handle declarations whose slugs don't share a normalized substring, flags `**Name:**` fields that don't match a declared handle slug, and flags bare email addresses in the Reviewer or Source preamble.

#### Hook ownership (per the [Review 87 Finding 6](review-log/2026-05-21-suite-review.md#review-87--2026-05-21-1230z) per-error-class owner table)

The `check-external-review-anonymization.py` hook is a **process-enforcement + early-detection script** — process-enforcement because it gates commits on identity-correlation discipline; early-detection because it catches authoring violations before the artifact reaches review. Per the Review 87 Finding 6 boundary, the hook is **owned by AI Engineer** (the meta-tooling-of-methodology surface). The discipline it enforces is informed by the **Privacy domain** (when active for a project; ~always active at capstone+ intent) — Privacy provides the substantive concern about identity-correlation harm; AI Engineer provides the hook's mechanization + the [Dim 11 audit-trail machine-readability](../domains/role/AI-ENGINEER-REVIEW.md) discipline that the hook's structured rule set implements. Three-audience compliance: the rules are human-readable (the hook's docstring + the `## Identity-correlation discipline` section above), structurally machine-parseable (per-rule predicates against an `external-review-log/*.md` file's preamble), and operator-overridable via the `<!-- hook-bypass: <rationale> -->` HTML-comment escape that bypasses are themselves findings.

#### Mining-Review Source-value

The mining-Review entry that processes an external-review file uses `**Source:** external-feedback` per [`primers/3-review-session.md`](../primers/3-review-session.md) § Source attribution. The mining-Review owns the per-finding classification + routing + per-domain ownership; the external-review file records WHO said WHAT WHEN (with the identity-correlation discipline applied). The two artifacts pair: external-review file = evidence-archive; mining-Review = methodology-interpretation.

Multiple external-review files from the same window may be batched into a single mining-Review (precedent: [Review 85](review-log/2026-05-21-suite-review.md#review-85--2026-05-21-1130z) mined `2026-05-20-dollspace-gay.md`; [Review 88](review-log/2026-05-21-suite-review.md#review-88--2026-05-21-1330z) mined `2026-05-21-shimmermathlabs.md` + codified this very pattern).

#### Companion review dimensions

- AI Engineer [Dim 11 audit-trail machine-readability](../domains/role/AI-ENGINEER-REVIEW.md) — the structured per-finding routing table at the bottom of each external-review file is the agent-readable surface; the file's H1 + H2 shape contract is parser-stable per the Review 80 Agent-API surface section above.
- Technical Writer [Dim 11 audience-fit calibration](../domains/role/TECHNICAL-WRITER-REVIEW.md) — the prose-quality of the file's preamble + Notes sections.
- Documentation Reviewer [Dim 4 cross-reference resolution](../domains/role/DOCUMENTATION-REVIEWER-REVIEW.md) — the mining-Review back-links.
- [Privacy](../domains/role/PRIVACY-REVIEW.md) — identity-correlation discipline as the substantive concern motivating the hook's rule set; when Privacy is active for a project, the external-review-log subfolder's adherence to the discipline is an active review surface for that domain.

## Supplement coverage

Supplements provide language-specific failure modes. Not every domain has language-specific concerns — process-compliance and portfolio domains (VDD-IAR Alignment, Portfolio Assessment, Solution Owner) are language-agnostic.

**Current supplement sections by domain:**

| Domain | `javascript-typescript.md` | `rust.md` | Notes |
|---|---|---|---|
| Quality Engineer | ✓ | ✓ | |
| Security | ✓ | ✓ | |
| Software Engineer | ✓ | ✓ | |
| Platform Engineer | ✓ | ✓ | |
| Data Engineer | ✓ | ✓ | |
| Solution Architect | ✓ | ✓ | |
| Red Team | ✓ | ✓ | |
| Performance Engineer | ✓ | ✓ | |
| UX | — | — | Covered by `../supplements/cli.md` (CLI) and `../supplements/browser-app.md` (browser) |
| Technical Writer | ✓ | ✓ | |
| Accessibility | — | — | Covered by `../supplements/browser-app.md` |
| Privacy | — | — | Language-agnostic |
| Localization | ✓ | ✓ | |
| Solution Owner | — | — | Language-agnostic |
| VDD-IAR Alignment | — | — | Language-agnostic (explicitly noted in domain file) |
| Portfolio Assessment | — | — | Language-agnostic |

Before closing any suite development session, verify this table is still accurate. If a domain is added or a supplement section is added, update this table and the findings registry.
