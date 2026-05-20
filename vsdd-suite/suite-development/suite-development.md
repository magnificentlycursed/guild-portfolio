# Session Primer: Suite Development (Meta — Suite Contributors)

Use this prompt at the start of any session whose purpose is developing the VSDD suite itself — adding or modifying domain files, updating dimensions, running gap analysis, or revising primers. Do not use this for reviewing projects under the suite; use `../primers/3-review-session.md` for that.

---

## Prompt

You are helping develop the **VSDD Suite** (Iterative Adversarial Refinement — IAR — is the Phase 3 component, not the suite's full scope). The suite is itself a software artifact: it has a specification (the VSDD and VDD methodology documents), a design (the domain structure, dimensions, supplement architecture, and session primer set), and an implementation (the domain prompt files, session primers, README, supplements/ supplements, gap analysis log, and DOMAIN-INDEX).

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
| Gap analysis log | `FINDINGS-INDEX.md` | Running registry of identified suite gaps and their status |
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

**What a strong dimension looks like:** `**Test falsifiability** — Would each test catch a broken implementation? Named attacks: mutation testing. Named tools: Stryker (JS/TS), mutmut (Python), cargo-mutants (Rust).` — failure class named, why it matters explained, specific technique given.

**Finding classification schemas by domain type:**

- Most role domains: `resolved`, `deferred`, `dismissed`, `hallucinated`
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

---

## Governing standard for project-level review logs

A project-level review log is the artifact produced by running a domain review on a project under review. The domain prompt file specifies *what* to evaluate; this standard specifies *what the resulting log must contain*. Drift in log structure makes cross-domain reading harder and hides governance gaps — apply this standard whenever a new project-level log is created or an existing one is updated.

### Structure (per-domain index + per-session entries)

**Forward-only constraint:** This index-plus-session-file structure applies to projects starting after 2026-05-17 (G-89 closure date). Projects whose first IAR run predates that date retain their existing single-file-per-domain structure (one accumulating file per domain holding all rounds) and must not be retroactively split. Reference: G-89's row in [`FINDINGS-INDEX.md`](FINDINGS-INDEX.md).

For a new project, each active domain produces two file shapes:

| Artifact | Location | Content |
|---|---|---|
| **Per-domain index file** | `<project>/vsdd-suite/<DOMAIN>-REVIEW.md` (e.g., `vsdd-suite/QUALITY-ENGINEER-REVIEW.md`) | File-level header (see below) + a **Reviews** table indexing every round filed for that domain. One row per round, newest at the top. Each row links to the session file's anchor for that round. This file is the index; it does not contain finding narratives. |
| **Per-session file** | `<project>/vsdd-suite/review-log/YYYY-MM-DD-<domain-slug>.md` | The actual round entries. One file per UTC date; if multiple rounds for the same domain happen on the same date, they share a file (new rounds appended at the top of the file, newest first within the date). One file per (date, domain) pair. |

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

**Cross-domain references** between findings: link directly to the session file with the round's anchor, not to the index. Use the same `[text](path)` form the gap registry uses: `[QE Review 4](review-log/2026-06-15-quality-engineer.md#review-4--2026-06-15-1400z) Finding 2`. Linking through the index adds a navigation hop without informational value.

**Why the split:** Cross-domain reading is faster (the index is one screen, not hundreds of lines of accumulated rounds); session-file scoping makes scoped-search (`grep` for a specific date or round) cleaner; multi-round closure trails are visible at the index level (a reviewer can see at a glance how many rounds it took to reach MVR); large projects don't produce single domain files in the multi-thousand-line range. The pattern mirrors the suite's own [`SUITE-DEVELOPMENT-REVIEW.md`](SUITE-DEVELOPMENT-REVIEW.md) + `review-log/` structure, which has been load-tested with 38+ sessions and works.

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

A single `<project>/vsdd-suite/FINDINGS-INDEX.md` file holds the cross-cutting registry, structured like [`FINDINGS-INDEX.md`](FINDINGS-INDEX.md) — one row per finding with columns for ID, domain, layer, round, finding-number, title, classification, status, source, and a link to the per-session-file anchor for the full narrative. Quick lookup is via grep or markdown viewer with table filtering. Template at `vsdd-suite/templates/PROJECT-FINDINGS-INDEX-template.md`.

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

### File-level header (top of the per-domain index file)

The per-domain index file opens with these elements, in order:

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

The session file opens with a simple H1 (`# [Role] Review — YYYY-MM-DD`). No file-level metadata is duplicated from the index — a reader who landed on a session file without context can follow the rounds back to the index via the `[Index](../<DOMAIN>-REVIEW.md)` link in the H1's footer line if needed (this footer link is optional; the round entries inside reference back to the index implicitly through the index's reverse links).

```
# Quality Engineer Review — 2026-06-15

[Index](../QUALITY-ENGINEER-REVIEW.md)

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
- **Source (G-133):** how this round's findings were elicited. Valid values: `domain-raised` (the cold adversary, applying the domain's dimensions, found the finding) — the default if no Source line is present; `director-raised` (the operator running manual testing, post-MVR exploration, or any non-domain-prompt-driven adversarial pass found the finding; ITC L6 R3 SO R22 is the canonical example — director's manual execution of "delete highest-id, create" caught a spec violation 11 cold-batch IAR domain reviews missed); `regression-replay` (a prior layer's adversarial reproducer re-run against the current binary surfaced the finding); `external-feedback` (an upstream stakeholder, project consumer, or methodology author surfaced the finding through prose feedback rather than a structured review — dollspace.gay's `message-4.txt` evaluation of ITC, mined in Review 51, is the canonical example); `mixed` (Review 68 Finding 9 extension) when the round's findings span more than one of the above sources, e.g., a session opened by a director-raised question that then surfaces additional domain-raised findings via cold-context primer reload. The `mixed` value requires the Source line to name the sub-disposition explicitly (e.g., `**Source:** mixed — `domain-raised` for findings 1–3; `director-raised` for findings 4–5`). The Source field gives audit-trail granularity to the Portfolio Assessment dimensions on developer participation; a project whose findings cluster heavily in `director-raised` or `external-feedback` is a different developer-engagement profile than one whose findings cluster in `domain-raised`.

**Optional, only when applicable to the domain:**
- **Posture:** adversarial framing (Red Team)
- **Program phase:** apprentice phase context (VDD-IAR Alignment)
- **Reference:** non-DESIGN.md authoritative source the review evaluates against (Solution Owner reviewing against the assignment brief)
- **Regression check:** prior-review verification (any domain when a prior review for the same scope exists)
- **Assumption surfacing:** dependency and library-API verification (Quality Engineer, per the QE prompt's G-20/G-21/G-23 obligations) — one short paragraph per review naming assumptions verified or flagged

A reviewer who finds they need a preamble field that is not in either list should propose adding it to this standard rather than introducing it ad-hoc. Examples of fields that are **not** valid additions: `Preamble`, `Governing methodology`, `Mutation analysis method`, free-form `Test count` lines — these duplicate `Scope` or `Session note`, or belong inside individual findings or the closing summary.

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

**Exception — Portfolio Assessment:** Portfolio Assessment groups by dimension (`### Dim N — Name`), not by classification, because each portfolio dimension produces a per-dim assessment (`Demonstrated`/`Partial`/`Absent`/`Hallucinated`) rather than a defect to fix. The classification appears at the end of each dim section and a summary table appears at the close. Portfolio also adds a file-level `**Developer participation note:**` directly under the sycophancy check, naming which dimensions require direct developer interrogation rather than artifact analysis. These exceptions are intentional and limited to Portfolio Assessment; no other domain may use dim-first organization or the participation-note field.

### Finding body

Each finding follows this structure:

```
**Finding N — Title (Dim X)**

[Prose body — what was observed, why it matters, evidence]

**Resolution:** [for Resolved findings — what was changed and where]

— or —

**Classification:** [Dismissed | Deferred | Hallucinated | Accepted Risk | …]
[rationale; for Accepted Risk and similar, include the named owner]
```

- Finding title always includes the dim reference parenthetically (`(Dim 2)`, `(Dim 1, Dim 10)`, `(Rust supplement — path traversal)`)
- Numbering is continuous within a Review (1, 2, 3, … across all classifications), not restarted per classification
- Cross-references to other domain logs use Markdown links: `[QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md) Finding 4` — not prose ("Logged in QE log")
- Closer is exactly one of `**Resolution:**` (Resolved only) or `**Classification:**` (everything else). Mixing the two within a single domain's log is drift.

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
3. Check the gap registry (`FINDINGS-INDEX.md`). If the gap is tracked, add the gap ID to the CHANGELOG entry for this change. If it is not tracked, add it and immediately mark it Addressed.
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
9. Add a `FINDINGS-INDEX.md` entry if the domain addresses an existing open gap.

## Before modifying a domain

1. State what defect the current version fails to catch — name a specific scenario where the current prompt produces a false pass.
2. Make the change.
3. Update the gap registry: if the gap was tracked, mark it Addressed with today's date. If it was not tracked, add it and immediately mark it Addressed.
4. Log the change in `CHANGELOG.md`.
5. If the change is structural (new section, new classification schema, changed prompt format): add a suite review session entry in `review-log/YYYY-MM-DD-suite-review.md` and a corresponding row in the **Suite Reviews** table in `SUITE-DEVELOPMENT-REVIEW.md`.

## Running gap analysis

Read `FINDINGS-INDEX.md` for the current open gaps, then read all domain files and evaluate whether each open gap has been addressed by recent changes. Follow the instructions at the top of that file.

**Gap registry discipline:** When a gap is resolved by a suite change, update the original row's status in place — change `Open` to `Addressed` and update the `Last Reviewed` date. Do not append a new row for an existing gap. New gaps get new rows; status changes update existing rows.

A gap analysis session is one mode of suite review (registry-walk lens). Like any suite review session, it ends with:
- All recently addressed gaps marked Addressed in `FINDINGS-INDEX.md` with the date
- Any new gaps discovered added to the registry with a new G-ID
- A `## Review N — date` entry in `review-log/YYYY-MM-DD-suite-review.md` (creating the file if no entry exists for that date) summarizing scope, findings, decisions, and suite changes made
- A corresponding row added to the **Suite Reviews** table in `SUITE-DEVELOPMENT-REVIEW.md` linking to the new entry

`FINDINGS-INDEX.md` contains only the registry table. Run narratives belong in `review-log/`. `SUITE-DEVELOPMENT-REVIEW.md` is the index of those narratives, not their home.

## Suite review and review-log discipline

The IAR suite has three parallel review-record artifacts. Their roles do not overlap:

- **`SUITE-DEVELOPMENT-REVIEW.md`** is an index. It contains one table — **Suite Reviews** — each row pointing to a session entry in `review-log/`.
- **`review-log/YYYY-MM-DD-suite-review.md`** holds the actual session entries. One file per date; multiple sessions on the same date append to the same file (newest at the top).
- **`FINDINGS-INDEX.md`** is the gap registry. Status only — no narrative. One row per gap; status changes update the row in place.

Every non-trivial suite change requires a session entry in `review-log/` and a corresponding index row in `SUITE-DEVELOPMENT-REVIEW.md`. Non-trivial means: any addition or removal of a domain or primer, any new evaluation dimension, any structural change to the prompt format, or any change to sequencing or activation guidance.

Mechanical fixes (typos, filename renames, path updates) do not require a session entry but should be logged in `CHANGELOG.md`.

**One artifact type, multiple modes.** A suite review may apply fresh adversarial pressure (defect-search lens), walk the gap registry top-down (registry-walk lens), or both. The mode lives in the **Lens** field; it is not a separate artifact type. Sessions previously called "meta-reviews" and "gap analysis runs" are now both `Review N` entries — the distinction is mode, not kind.

### Filename convention

The filename date is the **session start date in UTC**. When a session crosses midnight UTC, it remains in the file matching its start date — do not split it across two files. Same-date sessions append to the existing file (newest at the top).

**File-size threshold + part suffix (Review 69 amendment).** When a same-date file would exceed **80 KB** OR **15 review entries**, split it into `-partN.md` suffixed parts (e.g., `2026-05-19-suite-review-part1.md`, `2026-05-19-suite-review-part2.md`). Each part holds a contiguous run of reviews (oldest at the top within each part, parts numbered by chronological order); both parts get an H1 of the form `# Suite Review — YYYY-MM-DD (part N of M)` and a navigation note linking to the sibling part(s). The split is mechanical (not topical) — the rule is file-size / review-count, not narrative cohesion.

**Why the threshold exists:** Markdown parsers (tree-sitter; IDE language servers; markdown linters) hit parse-time budgets on dense markdown files with many inline code spans, long paragraphs, and nested tables. Suite review entries contain all three patterns. A long review-cycle day (8+ verbose reviews) produces a file that exceeds the parse-time budget and shows up as "parser aborted" / "parser timed out" diagnostics in operator IDEs. The threshold is empirical (the 2026-05-19 file hit parser-aborted at ~128 KB / 8 reviews of recent verbose style); 80 KB / 15 reviews leaves headroom for both denser and longer reviews.

**Cross-reference rule.** When a file is split, any forward-facing artifact that cited the original by Markdown link must update the link target to the correct part. The `SUITE-DEVELOPMENT-REVIEW.md` Reviews table gets one row per review pointing at the part the review lives in. `FINDINGS-INDEX.md` gap-row anchor citations get rewritten to the part file. CHANGELOG / COMPATIBILITY entries describing the affected reviews update their prose references to the part filenames. Historical-narrative file references (older CHANGELOG entries describing the original file's creation, etc.) stay per G-89 forward-only.

**Forward-only:** the part-suffix rule applies to files split on or after 2026-05-20 (Review 69 amendment date). Existing single-file days that have not yet hit the threshold remain single-file; the rule kicks in only when the file would exceed it on the next append. A file already over threshold gets retroactively split (as `2026-05-19-suite-review.md` was at Review 69). The split decision is a per-file event, not a portfolio-wide migration.

### Suite review entry format

A `## Review N — date` entry in `review-log/YYYY-MM-DD-suite-review.md` must contain:

1. **Header** — `## Review N — YYYY-MM-DD HH:MMZ`. Review numbers are **sequence-wide across all suite-review files** (Review 30 follows Review 29 even if they live in different date-named files); the timestamp is the session start in UTC.
2. **Scope** — What artifacts were read this round (specific domain files, primers, supplements, README, gap registry rows, etc.) and what triggered the review (user request, follow-up to a prior finding, scheduled cadence, project type added). Cite specific files when narrow; "all 14 role domains, 2 meta domains, 5 primers" when broad.
3. **Lens** — The angle the reviewer applied. Valid forms:
   - A **named defect class** ("coordination link format compliance", "classification schema coverage", "lang supplement symmetry").
   - A **registry-walk scope** ("walk all open gaps", "review G-22 and G-30").
   - A **role-based lens** that applies one or more domain perspectives to the suite as artifact ("Solution Owner + Technical Writer + VDD-IAR Alignment"), or a named bundle of complementary defect-class lenses applied serially or in parallel ("five lenses applied serially — clarity, naming, ambiguity, consistency, transitional-state alignment").

   A diffuse lens produces a diffuse review. If a session has no specific lens, log it as a generalist pass and name the prior passes' specialization gaps it is filling.
4. **Findings** grouped by classification heading. Valid headings:
   - `### Resolved` — fix applied and verified during the session. Use both for newly-found defects fixed in-session and for previously-tracked gaps closed in-session (cite the G-ID).
   - `### Dismissed` — concern reviewed and rejected; rationale required. Use both for newly-raised defects rejected and for previously-tracked gaps dismissed.
   - `### Hallucinated` — adversary-invented concern that does not apply; rationale required.
   - `### New gap registered` — finding promoted to a tracked gap; G-ID stated; the registry row is added in `FINDINGS-INDEX.md`. This heading is **suite-review-specific** — it is not part of the project-level classification universe and is not valid in project-level review logs.
5. **Finding body** — same shape as project-level review logs: `**Finding N — Title**` for new findings, `**G-XX — Title**` for gap-registry walk entries; prose body; then `**Resolution:**` (Resolved) or `**Classification:**` (everything else). Cross-references to other suite artifacts use Markdown links.
6. **Closing** — no separate Summary required (the classification headings carry the tally). An optional `### Coordination` section may follow the classification sections when findings cluster around a single coordinated decision; use it to name the cluster and the bundled action (e.g., a single restructure pass at a future trigger). Cross-references inside the Coordination section use Markdown links to other suite artifacts. Follow-up findings introduced after the session has been logged must be marked `**Finding M — Title (added YYYY-MM-DD)**` and placed at the end of the original entry, not in a new entry. Do not silently amend prior findings.

### Session isolation

Suite reviews are typically conducted in the same session as the user's pre-flight discussion or the suite changes the review evaluates. Unlike domain reviews — where a cold, isolated session is the gold standard — suite reviews benefit from continuity with the suite's authorial context. The compensation is documented per-entry: each `**Session note:**` line names the session-isolation status and explicitly acknowledges sycophancy risk when in-session.

A cold-session suite review is permitted and produces stronger adversarial pressure on suite artifacts; it is not required. The minimum standard is that the session note explicitly states whether the session is cold or in-session and, if in-session, names a compensation (e.g., findings derived from artifact-state analysis rather than narrative judgment, independent evaluation of any user-named candidates against the lens criteria). A suite-review entry whose session note omits this acknowledgement is a structural error — the missing acknowledgement is itself a finding for VDD-IAR Alignment dim 7 (cross-session spec consistency) applied to the suite.

### Common discipline

The session entry is the narrative record. The `FINDINGS-INDEX.md` row is the status indicator for gaps. The `SUITE-DEVELOPMENT-REVIEW.md` row is the index pointer for the session. Never put narrative in the registry; never omit the registry update; never omit the index row. An unindexed session is invisible to future reviewers.

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

Before closing any suite development session, verify this table is still accurate. If a domain is added or a supplement section is added, update this table and the gap registry.
