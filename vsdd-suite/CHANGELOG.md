# Changelog

All notable changes to the suite are recorded here. Entries are in reverse chronological order. Timestamps are UTC (Zulu).

---

## Unreleased — 2026-05-18 (Review 47: crosslink-contract.md G-138 extension + `--comment`-on-close correction sweep; G-139 registered)

### Driver-requested
Update `crosslink-contract.md` for the G-138 finding-index commands. The verification pass — every cited crosslink command run against installed `crosslink <subcommand> --help` — surfaced a second-instance G-123 recurrence and closed both items in this commit.

### Changed
- **`crosslink-contract.md`** § Dependency surface — corrected the `crosslink issue close <id> --comment "<text>"` row: actual `close` subcommand has no `--comment` flag, only `<ID>` positional + `--no-changelog`. Replaced with three rows describing the verified comment-then-close pattern (`issue comment <id> "<text>" --kind <kind>` then `issue close <id>`). Refined `issue list` to use `-s` short form per verified help output. Added `issue unlabel` row.
- **`crosslink-contract.md`** — "Tested-against version" updated to "every command and flag in this file was verified against `crosslink <subcommand> --help` output on 2026-05-17 (Review 46 + 47 verification pass)."
- **`README.md`** § Worked example Phase 3 — corrected the Hallucinated example to `comment --kind decision` then `close`; corrected the Resolved example to `comment --kind resolution` then `close`; updated `issue list --status` to `issue list -s`.
- **`README.md`** § Worked example Phase 4 — corrected the routed-finding closure to the comment-then-close pattern with explanatory parenthetical.
- **`primers/4-feedback-integration.md`** § Step 5 — corrected the routed-finding closure prose to the `&&`-chained `comment --kind resolution && close` pattern with explanatory parenthetical.

### Added
- **`crosslink-contract.md`** § G-138 finding-index commands (crosslink path) — new section enumerating the verified surface for the G-138 crosslink path: `issue create -l axis:value` (with `-l` repeatable verified), single-axis `issue list -l <axis>:<value> -s <status>`, multi-axis via `--json | jq` (verified manual fallback — single-label filter only at the `-l` level), `crosslink tui` for interactive browse, `issue label`/`issue unlabel`, the reclassify sequence (unlabel-label-comment-close), `export -f json -o <path>` / `import <INPUT>` for manual↔crosslink migration.
- **`crosslink-contract.md`** § Crosslink commands the suite does not depend on — explicit out-of-scope enumeration for audit clarity (kickoff, container, sentinel, knowledge, style, mc, serve, tui-as-workflow-dep, trust, locks, sync, migrate, config, context, integrity, compact, prune, timer).

### Suite-development event
- **G-139 registered** — G-123 manual discipline insufficient against AI-agent recurrence; CLI-verification tooling needed. Pattern: two recurrences across four sessions (Reviews 40–43 `--with-suite`; Reviews 38–46 `--comment`-on-close) of the same speculation-then-late-correction pattern, both violations of the G-123 discipline that was supposed to prevent them. The discipline runs in the same context that produces the speculation, so the agent's confidence overrides verification. Per "earned by recurrence": the rule change is a tooling fix — add `vsdd-suite/hooks/check-crosslink-references.sh` that mechanically validates every cited crosslink flag against `--help` output at commit time. Resolution sketch in the gap row; Open pending the hook authorship.

### Note
The Review 47 correction is the second G-123 recurrence; the discipline is now tracking two failure events. G-139's tooling fix is the structural answer rather than relying on third-time-lucky discipline. The Review 47 entry in `suite-development/review-log/2026-05-18-suite-review.md` carries the full reasoning. New date file — first session entry on 2026-05-18.

---

## Unreleased — 2026-05-17 (Review 46: project-level finding index — G-138 registered and addressed; bookmark-cli reference impl populated)

### Driver-raised observation
Driver flagged ITC's 20K-line review files as a quick-lookup problem and proposed a TOC-and-split pattern. Clarification surfaced G-89's forward-only constraint (ITC stays untouched); driver confirmed forward-only intent ("This is for new projects only and is not applied retroactively"). G-138 captures the underlying need (cross-cutting findings index orthogonal to G-89's per-domain round index) and addresses it for new projects.

### Added
- **`suite-development/suite-development.md`** § Governing standard for project-level review logs gained a new `### Project-level finding index (cross-cutting registry)` subsection. Defines two equivalent paths — **crosslink path** (every classified finding is a crosslink issue with labels `domain:<slug>`, `layer:N`, `round:N`, `finding:N`, `classification:<class>`, `source:<source>`, queryable via `crosslink issue list -l <axis>:<value>`) and **manual path** (single `<project>/vsdd-suite/FINDINGS-INDEX.md` file structured like the suite's GAP-ANALYSIS-LOG.md). Same information shape; projects can migrate between paths via `crosslink import` / `export`. Forward-only per G-89's framing.
- **`templates/PROJECT-FINDINGS-INDEX-template.md`** (new) — manual-path template with header, quick-lookup recipes, findings registry table, cross-references. Two example rows demonstrate the shape; projects delete the examples when populating.
- **`templates/scaffold-project.sh`** updated to copy `PROJECT-FINDINGS-INDEX-template.md` to `vsdd-suite/FINDINGS-INDEX.md` during scaffolding (with deletion-instruction for crosslink-using projects).
- **`templates/README.md`** Contents table updated with the new template row.
- **`README.md`** § Worked example Phase 3 updated — one sentence after the per-domain round-filing instruction names the finding-index step (both paths).

### Demonstrated by use
- **`bookmark-cli/vsdd-suite/FINDINGS-INDEX.md`** populated with three rows (F-001 through F-003) — the three QE Review 1 findings from the reference impl. Demonstrates the manual-path structure with real finding data; future Phase 3 reviews on bookmark-cli would append rows.

### Addressed
- **G-138 — Project-level finding index not established.** Registered and Addressed in the same session. The recurrence evidence: ITC accumulated ~50+ findings across 13 domains over 7 layers with no cross-cutting index; PROCESS.md L6 operator quote names the gap implicitly. Resolution applied per the spec above. Status flipped Open → Addressed in `suite-development/GAP-ANALYSIS-LOG.md`.

### Note
Constraint-respecting addition. ITC's existing single-file-per-domain shape stays per G-89 forward-only. New projects starting after 2026-05-17 adopt both the per-domain index (G-89) and the cross-cutting findings index (G-138); the two are orthogonal and complementary, not alternatives.

Coordinates with:
- G-89 (per-domain round index — G-138 sits on top, not replacement)
- G-130 (deferral lifecycle — G-138's labels capture deferral metadata trivially)
- G-133 (Source field — G-138 includes source as a label axis)
- G-118 (crosslink-contract.md — follow-on update needed to enumerate `issue list -l`, `issue create --label`, etc. as part of G-138's full closure; not blocking)

---

## Unreleased — 2026-05-17 (Review 45: cross-project pattern-mining from issue-tracker-cli IAR + PROCESS.md — 14 gaps registered, none addressed yet)

### Suite-development event (registry-only, no artifact changes)
- **14 new gaps registered (G-124–G-137)** from pattern-mining `issue-tracker-cli/iterative-adversarial-refinement/` (~20K lines across 13 domain review logs) plus `issue-tracker-cli/PROCESS.md` (547 lines, 7-layer first-person director retrospective). The completed project's IAR corpus is treated as evidence: a defect class that recurred across multiple layers is signal that the suite's upstream primers and dimensions are letting the class through. Methodology innovation: this is the first cross-project pattern-mining lens applied in the suite-review log; Review 45 establishes it as precedent for future reviewers.
- **Three clusters:**
  - **A — Defect-class generalizations** (G-124 per-property text-field defense; G-125 error-message escape; G-126 create/load symmetry; G-127 empty-state regression per filter; G-128 mutation-resistant assertions). Each tied to specific ITC recurrences (Title L1, Labels L4, Description L6 for G-124; QE R3+R5+R8 mutations for G-128; etc.).
  - **B — Process / discipline gaps** (G-129 CHANGELOG-currency hook; G-130 promote CLOSURE-PROTOCOL §3 auto-Backlog to suite-default; G-131 loop-count trigger framing; G-132 manual-testing-as-peer-surface; G-133 Source: director-raised classification). G-129 has the largest single-change leverage (six TW-caught recurrences in ITC alone).
  - **C — Operational / tooling gaps** (G-134 cold-session dispatch script; G-135 AI Engineering meta-domain candidate for cost/token discipline; G-136 phase-flow diagram in README; G-137 rustdoc verification command in Rust supplement). G-135 warrants its own arc — multi-session work surfacing the operator's explicit "AI Engineering review domain for recommendations" request from L4 PROCESS.md.

### Method
- Two complementary sources: (a) Explore subagent in its own context scanned the 13 ITC domain logs for recurring finding patterns (10 patterns reported); (b) direct read of PROCESS.md surfaced 4 additional operator-experience patterns the agent could not see (cost/token, loop-rigidity, manual-test elevation, director-raised classification). 7 of the 10 agent-patterns were independently confirmed by explicit PROCESS.md text, providing cross-validation.

### Recommended sequencing for follow-on closure
- G-129 first (cheapest, largest catch — six layer closes recur the same TW finding); then G-124 + G-125 + G-126 together (per-property defense cluster); then G-130 (deferral lifecycle); then G-131 + G-132 (loop and manual-test framing). Seven closures land the highest-leverage gaps. Remaining seven (G-127, G-128, G-133, G-134, G-135, G-136, G-137) bundle opportunistically; G-135 is the only one large enough to warrant its own arc.

### Note
This is a registry-only event — no suite artifacts are modified in this commit. The 14 gaps establish the actionable backlog from ITC's accumulated evidence. Future closure sessions will land the substantive changes (primer text additions, new hook script, new meta-domain authorship, etc.). The Review 45 entry in `suite-development/review-log/2026-05-17-suite-review.md` carries the full reasoning and cross-coordination notes.

---

## Unreleased — 2026-05-17 (Review 44: reference implementation landed — G-112 closed; G-106 refined; dogfooding the scaffold work)

### Added (portfolio event, not a suite-internal change)
- **`bookmark-cli/`** at the portfolio root — first project built against the current `vsdd-suite/` shape per the G-117 manual-copy canonical default. Layer 1 complete through Phase 2b (8/8 tests pass via `cargo test`); Phase 3 QE Review 1 filed in the new per-domain index + per-session-file structure (G-89 forward-only convention exercised); other 6 active-core-domain indices scaffolded as template stubs (accurate for reference-impl scope, not a full Phase 3 run). Closes G-112.

### Dogfooding outcomes (the scaffold work landed in Reviews 40–42 validated by use)
- `templates/scaffold-project.sh` ran cleanly against the empty `bookmark-cli/` directory; created the 7 default-active core domain indices + DESIGN.md + project README skeletons.
- `templates/DOMAIN-REVIEW-template.md` placeholders (`{{ROLE_TITLE}}`, `{{ROLE_VARIANTS}}`, `{{SYCOPHANCY_CHECK}}`) were sufficient for one customization pass to produce a usable per-domain index file.
- `templates/DESIGN-template.md` and `templates/PROJECT-README-template.md` provided reasonable starting structure (replaced with project-specific content in the reference impl).
- The QE Review 1 entry exercises the per-session-file format from G-89 — confirms the path resolution, the round-anchor link convention, and the index → session-file navigation pattern.

### Refined status
- **G-106** stays Open with revised reason. The reference implementation closed G-112 but G-106 specifically asks for sample crosslink command outputs; the reference impl was built via the suite-only path per G-117 ratification and does not exercise crosslink. G-106's natural closure is a follow-on session that actively uses crosslink against a toy project — could be a future Layer 2 of bookmark-cli with crosslink, or a different demonstration project.

### Portfolio README
- `../README.md` (portfolio root) updated to include `bookmark-cli/` as the third portfolio project with "Reference implementation" framing. Forward-only compatibility note added pointing at `vsdd-suite/COMPATIBILITY.md` (bookmark-manager and issue-tracker-cli retain their `iterative-adversarial-refinement/` legacy paths; bookmark-cli uses `vsdd-suite/`).

### Note
The Review 40 + 41 + 42 + 43 + 44 onboarding-experience review arc now closes at **23 of 24 gaps addressed** (G-100–G-105, G-107–G-111, G-113–G-123 plus G-112; only G-106 remains Open and is scoped to a follow-on session). The reference implementation also serves as the dogfooding verification of the scaffold work — `scaffold-project.sh` and the templates worked as designed against a real empty project.

---

## Unreleased — 2026-05-17 (Review 43: correction — `crosslink init --with-suite` references removed; G-123 registered and addressed)

### Driver-raised correction
Reviews 40–42 introduced six references to a non-existent `crosslink init --with-suite` feature, framing it as a "coordination ask against crosslink upstream." The driver does not control the crosslink repository and the feature does not appear in crosslink's governing documentation; the references implicitly committed someone to a PR against an out-of-scope repo. The driver flagged this after Review 42 closed but before commit.

### Changed (corrections)
- **`README.md`** — Quickstart Phase 1 step rewritten to drop the `--with-suite` reference and clarify that crosslink and the suite are independent. The Accepted-variant table dropped the `crosslink init --with-suite` row entirely and gained a clarifying line stating that crosslink and the suite are independent tools with no shared scaffolding.
- **`templates/README.md`** — § Usage corrected to drop the "forthcoming `crosslink init --with-suite`" framing; replaced with a factual statement that crosslink and the suite are separate tools that each scaffold their own state.
- **`templates/scaffold-project.sh`** — header comment rewritten to remove the speculative future-feature mention.
- **`suite-development/README.md`** § Pure core / effectful shell — example of future effectful-shell expansion reworded to use a non-speculative example.
- **Historical narrative preserved** in `suite-development/GAP-ANALYSIS-LOG.md` (G-101, G-117, G-120 rows), `suite-development/SUITE-REVIEW-INDEX.md` (Review 42 row), and `suite-development/review-log/2026-05-17-suite-review.md` (Reviews 40, 41, 42 entries). These are historical records of what was decided at the time; Review 43's entry is the correction annotation.

### Added
- **`suite-development/suite-development.md`** — new `### External dependency references` subsection under § Governing standard for session primers. Three rules: reference only features that exist in the current released version of the dependency; do not speculate about "could be added later" features; do not treat missing features as coordination asks unless the suite's owner has authority to file the PR upstream. Names the failure mode (LLM extrapolation from "could integrate if X existed" to "X is coordination-asked") and cites `crosslink-contract.md` as the canonical record of the verified crosslink dependency surface. Closes G-123.

### Note
Doctrine ratifications from Review 42 (G-117 + G-121) unaffected — submodule and sibling-symlink remain accepted variants for explicit reasons; the `--with-suite` row simply did not belong in the table. The Review 40 + 41 + 42 + 43 onboarding-experience arc now closes at 22 of 24 gaps addressed (21 from prior arcs + G-123 closed in this commit); 2 remain Open (G-106, G-112) — both pending the reference-implementation work that was paused for this correction.

---

## Unreleased — 2026-05-17 (Review 42: Solution Owner doctrine ratification — G-117 and G-121 closed)

### Doctrine ratifications
- **G-117 ratified: manual copy via `scaffold-project.sh` is the canonical suite-to-project coupling default.** Submodule / sibling-symlink / `crosslink init --with-suite` preserved as accepted-variant options. Rationale: SO Dim 3 (technology compliance — manual copy matches the suite's existing markdown + bash surface), SO Dim 4 (over-engineering — submodule and `--with-suite` are team-scale-default for portfolio scale), SA Dim 9 sycophancy-check (a human engineer working alone on 3 projects would not author submodule infrastructure when `cp` + a 50-line bash script suffices). The forthcoming `crosslink init --with-suite` remains a coordination ask against crosslink upstream, not a suite-side responsibility.
- **G-121 ratified: the scaffold script's default-7-cores IS the starter-set doctrine.** No separate "starter set" concept introduced — that would have been over-engineering on top of an existing answer. Single-sentence affordance added to README naming "default activation is 7 cores; the 16-domain surface is *available*, not *required*; typical portfolio projects run 7–9 active domains per layer." Rationale: SO Dim 1 (the implicit default needed to be made explicit), SO Dim 4 (proposing a separate two-mode doctrine is itself team-scale-default).

### Changed
- **`README.md` § Bringing the suite into your project** — restructured to lead with "The canonical default: manual copy via `scaffold-project.sh`"; demoted the other three mechanisms to a labeled "Accepted-variant options" table; preserved all four with their trade-offs noted.
- **`README.md` § Quickstart** Phase 3 step — added the "default activation is 7 cores … typical portfolio projects run 7–9 active domains per layer" affordance.
- **`README.md` § Domains** opener — added a top-of-section paragraph stating "The 16-domain surface is *available*, not *required*. Default activation for new projects is the 7 core role domains plus VDD-IAR Alignment" with the Review 42 / G-121 closure reference.

### Note
The Architectural-doctrine cluster from Review 41 (G-117 + G-120 + G-121) is now fully closed. The full Review 40 + 41 + 42 onboarding-experience review arc reaches refinement-signal exhaustion: 21 of 23 gaps addressed; 2 remain Open (G-106 sample crosslink command outputs, G-112 end-to-end reference implementation) — both deferred pending a future session that builds the reference implementation (which produces the real command outputs for G-106 and serves as the contract-testing canary for G-118). The doctrine ratifications applied here did not surface additional scope-creep or compliance failures; no new gaps registered by Review 42.

---

## Unreleased — 2026-05-17 (Review 41: Solution Architect lens — 6 gaps registered; 4 addressed-set; 2 raised to SO)

### Suite-development event
- **6 new gaps registered (G-117–G-122)** from the Solution Architect adversarial review of the suite-as-system, with onboarding-relevance as the prioritizing filter. Two clusters: **Architectural-doctrine gaps** (G-117 coupling-mechanism choice, G-120 versioning strategy, G-121 complexity-budget / starter-set definition) — three filed Raised-to-SO-equivalent because SA proposes but SO is the doctrine authority. **Dogfooding gaps** (G-118 crosslink CLI contract, G-119 AI-tool dependency inventory + data-flow posture, G-122 suite's own purity boundary) — three closures applying the suite's own External Interface Contracts, External Service Integration, and Dim 12 discipline to the suite itself. Review 41 entry in `suite-development/review-log/2026-05-17-suite-review.md` carries the full reasoning. Solution Owner review recommended next to ratify Cluster A.

### Added
- **`crosslink-contract.md`** (new) — closes G-118. Suite's application of SA Extended dims 13–22 to its own crosslink CLI dependency. Enumerates: tested-against version (crosslink v0.8.0); the full dependency surface (8+ commands invoked by the worked example, with required flags and per-phase usage); breaking-change definition; error contract (expected vs unexpected response per command class); contract-testing plan that closes via G-112 (reference implementation). The suite teaches the External Interface Contracts dimensions; this file dogfoods them.
- **`COMPATIBILITY.md`** (new) — closes G-120. Documents the forward-only compatibility policy explicitly. Names the policy ("changes apply to projects starting after the change date; pre-change projects retain the pre-change shape"); enumerates retroactive version anchors (v0.1.0 = Review 36 baseline, v0.2.0 = Reviews 38+39 post-ITC restructure, v0.3.0 = Reviews 40+41 onboarding overhaul); defines breaking change vs non-breaking; documents the deprecation process the suite has been using implicitly. Promotes G-94 sub-issue 4 (CHANGELOG release tagging deferred to spinoff-MVP) from deferred to addressed at the policy level — actual git tags applied separately.
- **`README.md`** — new `### Data-flow and privacy posture` subsection under Prerequisites; closes G-119. Names what data flows to the AI tool per primer-loaded session; documents the per-tool training-on-input posture (Claude API / Claude Code / claude.ai plan-tier / Cursor); recommends sensitive-context guardrails. The suite teaches Privacy dim 6 (third-party data sharing); this section dogfoods the discipline.
- **`suite-development/README.md`** — new `### Pure core / effectful shell` subsection; closes G-122. Documents the suite's own pure/effectful boundary (pure: all markdown content; effectful: `hooks/check-review-log-anonymization.sh`, `templates/scaffold-project.sh`). Small section because the boundary is small by design; the value is closing the dogfooding gap on Dim 12.

### Raised to SO (Open — awaiting doctrine ratification)
- **G-117 — Suite-to-project coupling mechanism choice.** Four candidate mechanisms are documented (G-101 closure); SO must pick the canonical pattern.
- **G-121 — Complexity-budget for personal-portfolio scale.** Three resolution-sketch options (starter-set subset / "you don't have to use it all" affordance / accept-as-is); SO must pick the doctrine.

### Note
Both architectural-doctrine gaps (G-117, G-121) are blocking-the-decision-not-blocking-the-suite — current behavior continues as-is until SO ratifies; the suite remains usable in its current form.

---

## Unreleased — 2026-05-17 (Review 40: onboarding-experience adversarial review — 17 gaps registered, addressed-set landed below)

### Suite-development event
- **17 new gaps registered (G-100–G-116)** addressing the new-user onboarding experience for crosslink-enabled (and crosslink-free) projects. Three coordinated clusters surfaced — Entry-point repair (G-100/G-104/G-116), Suite-to-project scaffolding (G-101/G-102/G-107), and Worked-example fidelity (G-103/G-105/G-106/G-111/G-112/G-113) — plus standalone fixes (G-108/G-109/G-110/G-114/G-115). Review 40 entry in `suite-development/review-log/2026-05-17-suite-review.md` carries the full reasoning, the multi-lens framing (defect-class + Technical Writer + UX/CLI), and the cross-coordination notes. Solution Architect review is recommended next.

### Changed (addressed-set from Review 40 — see entries below this CHANGELOG for the specific edits)
- **`README.md`** — entry-point and orientation repairs landing in this same commit set; see the per-finding entries below.
- **`suite-development/suite-development.md`** — lead-text reframe (IAR → VSDD post-G-88) landing in this same commit set.
- **`templates/`** — new directory with per-domain index stubs landing in this same commit set.

### Note
The full addressed-set for Review 40's quick-win and medium-effort findings lands as part of this CHANGELOG entry; the larger items (G-112 end-to-end reference implementation; G-106 sample crosslink-command outputs) are deferred and remain Open pending the reference-implementation work that gives them a verifiable canary. Both crosslink-enabled and crosslink-free paths are documented in every addressed finding per the driver's reminder mid-session that crosslink is optional and the suite was designed for manual operation first.

---

## Unreleased — 2026-05-17 (Review 39: project-level review-log standard + worked-example reframe)

### Changed
- **`suite-development/suite-development.md`** § Governing standard for project-level review logs gained a new top-of-section `### Structure (per-domain index + per-session entries)` sub-section defining the per-domain index file shape (`<project>/vsdd-suite/<DOMAIN>-REVIEW.md` with file-level header + Reviews table) and per-session entry file shape (`<project>/vsdd-suite/review-log/YYYY-MM-DD-<domain-slug>.md` with round narratives). Enumerates the domain-slug convention for all 16 core/extended/meta domains (lowercase, hyphenated role name, no `-review` suffix). States the cross-domain reference convention (link directly to session-file anchor, not through the index). Forward-only carve-out cited at the top of the new sub-section: projects whose first IAR run predates 2026-05-17 retain their existing single-file form. Existing `### File-level header` and other sub-sections reframed for the new file shape; a new `### Per-session file header` sub-section added. Closes G-89.
- **`README.md`** § Review logs updated with the new project tree shape (per-domain index files + `review-log/` directory of per-session files) and the forward-only paragraph naming the two completed projects whose existing structure is preserved.
- **`README.md`** § Worked example reframed per driver feedback. The prior text positioned crosslink as the operational shell with the suite's primers loaded into it and `[manual fallback]` tags for non-crosslink users — implicitly making the suite-primer-only path a fallback. The new text leads each phase with the primer's prescription ("Per `primers/<phase>.md`: ..."), shows the manual path as the primary instruction, then adds an `[+crosslink]` block as an optional amplifier showing how the tracker mechanizes what the primer prescribes. Section header retained per driver instruction. Lead paragraph rewritten to frame the primers as the spine; crosslink as the optional amplifier. The "Where each primer fits" table at the end gained a "What the primer prescribes" column, with the crosslink-command column renamed "Optional crosslink amplifier". `[manual fallback]` tags removed throughout — the manual path is no longer a fallback but the primary path.

### Addressed
- **G-89 — Standardize project-level domain review log structure on the suite-review pattern.** See Changed above. Status flipped Deferred → Addressed in `suite-development/GAP-ANALYSIS-LOG.md` (Last Reviewed 2026-05-17). The "after `issue-tracker-cli` completes" bundled trigger that originally gated G-88, G-89, G-90, G-91, G-92, G-93, G-94, G-95 is now fully closed (G-89 was the final gap remaining against it). The trigger paragraph in `GAP-ANALYSIS-LOG.md` § Reactivation triggers updated to reflect this.

### Note
G-89 is a forward-only standard change — no existing project review log is restructured by this commit; both completed portfolio projects (`bookmark-manager/iterative-adversarial-refinement/` and `issue-tracker-cli/iterative-adversarial-refinement/`) retain their existing single-file-per-domain structure and remain valid records under the legacy convention. The new index + session structure becomes the default for any project that opens a `vsdd-suite/` directory for the first time on or after 2026-05-17. The worked-example reframe is a documentation framing change with no methodology consequences.

---

## Unreleased — 2026-05-17 (Review 38: post-ITC structural restructure — directory rename, primer renames, suite-development subfolder, README split, implementation.md split)

### Changed
- **Suite directory renamed** from `iterative-adversarial-refinement/` to `vsdd-suite/`. The H1 in `vsdd-suite/README.md` was retitled `# VSDD Suite` to match; the lead paragraph reframes the suite as a multi-phase prompt and process library rather than a Phase-3-only adversarial review suite. Forward-only constraint honored — `issue-tracker-cli/iterative-adversarial-refinement/` and `bookmark-manager/iterative-adversarial-refinement/` are untouched; the `.pre-commit-config.yaml` anonymization-hook `files:` regex was widened to `^(vsdd-suite/.*\.md|.*/iterative-adversarial-refinement/.*\.md|.*/vsdd-suite/.*\.md)$` so review-log anonymization continues to fire on completed projects' IAR markdown. Closes G-88.
- **`prompts/` folder renamed to `primers/`** and primer files prefixed with VSDD phase: `spec-crystallization.md` → `1a-spec-crystallization.md`, `decomposition.md` → `1b-decomposition.md`, `implementation.md` split into `2a-red-gate.md` + `2b-implementation.md` (see below), `review-session.md` → `3-review-session.md`, `feedback-integration.md` → `4-feedback-integration.md`. Bare-digit phase prefix chosen (not `phase-` prefix) for brevity. Closes G-91.
- **Suite-meta materials moved into `vsdd-suite/suite-development/`** subfolder: `suite-development.md` (the contributor primer), `SUITE-REVIEW-INDEX.md`, `GAP-ANALYSIS-LOG.md`, `review-log/`. `CHANGELOG.md` and `hooks/` stayed at suite-root (changelog per top-level convention; hooks wired from repo-root `.pre-commit-config.yaml`). Closes G-92.

### Added
- **`vsdd-suite/suite-development/README.md`** (new) — contributor-facing navigation document introducing the suite-development workflow: what artifacts live in `suite-development/`, the suite-development pass shape (load primer → pick lens → run review → update GAP-ANALYSIS-LOG.md → add SUITE-REVIEW-INDEX.md row → CHANGELOG if artifacts changed), reactivation-triggers mechanism, and the project-scoped → suite-level promotion doctrine. The user-facing `vsdd-suite/README.md` gained a "For contributors evolving this suite" pointer near the top directing here. Minimal variant of G-93 — the deeper variant (migrating governing-standards prose out of `suite-development.md` into a dedicated handbook) was not taken; current placement (primer carries long-form discipline, README is the entry point) works structurally; reactivation trigger is a future contributor reporting confusion. Closes G-93 (minimal).
- **`vsdd-suite/primers/2a-red-gate.md`** (new) — Phase 2a session primer split out of the prior combined `implementation.md`. Phase-2a-tailored posture (tests-first, focus on behavior not implementation), the Red Gate principle, anti-pattern catalog, and Phase-2a-tailored completion criteria explicitly naming the Phase 2a → 2b handoff (Red Gate commit hash as verifiable boundary, no implementation logic this session). Closes G-95 jointly with `2b-implementation.md`.
- **`vsdd-suite/primers/2b-implementation.md`** (new — split from prior `implementation.md`) — Phase 2b session primer with Phase-2b-tailored posture (make tests pass; do not add new tests), Red-Gate-commit prerequisite explicitly stated, retroactive-Red-Gate label paragraph retained, layer-implementation-complete criteria. The Layer 7 R19 / G-99 warm-finding-closure topic remains project-scoped per Review 37; not added to either of the new primers. Closes G-95.

### Updated cross-references
- All suite-internal markdown files and the two repo-root files (`.pre-commit-config.yaml`, `.github/PULL_REQUEST_TEMPLATE.md`) had their internal paths updated by a Python rewriter scoped to vsdd-suite/ + the two root files. Substring replacements: `iterative-adversarial-refinement/` → `vsdd-suite/` (with negative-lookbehind for completed-project prefixes); `prompts/` → `primers/`; primer-filename renames; depth-aware fixups for files in/around `suite-development/`. Two false-positive sweeps were caught and reverted before commit: double-prefix corruption in the two new 2a/2b primer files (`2b-2b-implementation.md` etc.) and overzealous narrative rewrites inside historical suite-development/ artifacts (GAP-ANALYSIS-LOG.md, SUITE-REVIEW-INDEX.md, review-log/ session files) — those files were `git restore`d to preserve historical-narrative fidelity, since their link forms were sibling-relative and didn't break.

### Addressed
- **G-88, G-91, G-92, G-93 (minimal), G-95** — see Changed/Added above. Status changes recorded in `suite-development/GAP-ANALYSIS-LOG.md` with closure citations linking to [Review 38](suite-development/review-log/2026-05-17-suite-review.md#review-38--2026-05-17-0200z). The "after `issue-tracker-cli` completes" reactivation trigger fired; G-89 remains Deferred as independent scope.

### Note
Backwards compatible for the two completed portfolio projects — their inner `iterative-adversarial-refinement/` subtrees are unchanged, including all internal cross-references. The pre-commit anonymization hook continues to run against those projects' markdown via the widened `files:` regex. Future-project IAR work will adopt the new `vsdd-suite/` convention naturally because the suite source-of-truth has moved.

---

## Unreleased — 2026-05-16 (Crosslink integration across VSDD phases; Phase 4 primer added)

### Added
- **`primers/4-feedback-integration.md`** (new) — VSDD Phase 4 session primer. Names the routing discipline (a finding is routed to the *earliest* phase that can fix it, not the most convenient phase), provides a finding-to-phase routing table covering spec defects, decomposition gaps, test-discipline gaps, implementation defects, multi-phase chains, and suite gaps; defines the routing output schema (Route, Owning artifact, Gate, Sequencing); names the primary failure mode (routing every finding to Phase 2b, collapsing the pipeline); provides a crosslink-augmented workflow (`route:phase-*` labels, `crosslink swarm fix --from-label`, `crosslink issue block` for downstream-layer dependencies) and a fully manual fallback for projects without crosslink. Addresses G-86.
- **`README.md`** — new `## Worked example: A VSDD session with crosslink` section between Language/Interface supplements and Running IAR. End-to-end walkthrough of one VSDD cycle on a hypothetical `bookmark-cli` project: Phase 1a (`crosslink design [--continue <slug>]` → DESIGN.md promotion), Phase 1b (epic + per-layer milestones + parent-flag subissues + Red Gate comment), Phase 2a (`crosslink session start/work`), Phase 2b (`crosslink swarm gate <slug>`), Phase 3 (`crosslink swarm review --agents N --file-issues` with manual-dispatch fallback for high-stakes reviews), Phase 4 (route labels, `swarm fix --from-label`, gate-anchored closure). Each step tagged with the manual fallback so projects without crosslink can follow the same walkthrough. Includes a "where each primer fits" summary table.
- **`README.md`** — `Feedback Integration` row added to the `## Session primers` table, linking the new primer.

### Changed
- **`primers/1a-spec-crystallization.md`** — Added a `**Session medium:**` paragraph to the header explaining that `crosslink design [description] [--issue <id>] [--gh-issue <id>]` is an alternative session medium (writes to `.design/<slug>.md`, iterates via `--continue <slug>`). The primer text is unchanged in either medium; crosslink provides the session container, not the spec discipline. Added a `**Promotion (crosslink users):**` paragraph to the completion criteria explaining the `.design/<slug>.md` → `DESIGN.md` promotion step.
- **`primers/1b-decomposition.md`** — `## Crosslink issue hierarchy (Phase 2+ projects)` rewritten. **Bug fix:** the prior example called `crosslink subissue <epic_id> <layer_issue_id>` and `crosslink subissue <layer_id> "..."`, neither of which is a valid crosslink command — there is no top-level `subissue` command; subissues are formed by passing `--parent <id>` to `crosslink quick` / `crosslink issue create`. Replaced with the correct `--parent` flag form. **Addition:** the example now creates one `crosslink milestone` per layer (the layer's first-class container, separate from labels) and attaches the layer issue via `milestone add`. **Addition:** `crosslink session work "$LAYER1"` shown after `session start` so the active focus issue is set explicitly. **Addition:** `crosslink swarm gate <phase-slug>` shown as the Phase 2b → 3 boundary (project test suite as formal layer gate). **Addition:** `**Verifying deployed policy:**` paragraph naming `crosslink workflow diff` as the Phase 1b completion check (deployed policy matches embedded defaults). Completion-criteria item 6 updated to require milestone population and a clean `workflow diff`.
- **`primers/3-review-session.md`** — `## Session isolation` section gained a `### Dispatch options` subsection naming two modes: **Manual dispatch** (default; required for highest-stakes reviews) keeps the existing one-chat-per-domain shape as the gold standard; **Swarm dispatch** (`crosslink swarm review --agents <N> [--mandate adversarial] [--doc <path>] [--file-issues] [--fix]`) provides hard context isolation by construction (separate worktrees) and is appropriate for routine refinement rounds where the dispatcher rhythm (aggregated findings batch) is acceptable. Explicit guidance: choose manual when approaching MVR and the marginal finding matters most; choose swarm for volume passes. Re-states that the human-in-the-loop classifier requirement is identical under both modes — `swarm review` parallelizes adversaries, not classifiers.

### Addressed
- **G-86 — No VSDD Phase 4 (Feedback Routing) session primer.** Resolved by adding `primers/4-feedback-integration.md` (see Added above). The new primer also reframes the README "Known scope gaps" sentence — G-86 removed from the open-gap list; G-54 (Phase 5 Formal Hardening) and G-55 (Phase 6 Four-Dimensional Convergence) remain. Marked Addressed in `suite-development/GAP-ANALYSIS-LOG.md` (Last Reviewed 2026-05-16).

### Note
Backwards compatible. Every primer change is additive — projects that do not use crosslink can ignore the crosslink-specific sections and use the existing manual flows. The fallback path is named explicitly in every modified primer (`**[manual fallback]**` tags in the README walkthrough; `## Without crosslink (manual / Phase 1 projects)` section in `4-feedback-integration.md`; existing TODO.md workflow preserved in `1b-decomposition.md`). Existing project review logs and TODO.md files are not affected. The only authoritative-content change is the bug fix in `1b-decomposition.md` (invalid `crosslink subissue` command replaced with the working `--parent` flag form); pre-existing crosslink-using projects that hand-typed the old incorrect command would have already hit an error and self-corrected — no migration needed.

---

## Unreleased — 2026-05-06 (Review 36: review-log self-disclosure / meta-leak mitigations)

### Added
- **`primers/3-review-session.md`** — new `## Confidentiality-aware citation` section between adversarial-posture and "Before starting a domain review." Names the publishable-artifact class, lists opt-in-anonymization signals a reviewer can detect (home-paths hooks, noreply git config, scrubbed `Cargo.toml`/`package.json`), states the principle ("an example illustrating what-not-to-do should never instantiate what-not-to-do"), prescribes abstract placeholders, and references the suite-level enforcement hook.
- **`domains/role/PLATFORM-ENGINEER-REVIEW.md`** — appended `**Confidentiality-aware citation (Platform-domain reminder).**` paragraph after the dimensions section. Anchors the primer rule to PE's typical findings (hook configs, environment values, secrets management).
- **`domains/role/SECURITY-REVIEW.md`** — appended `**Confidentiality-aware citation (Security-domain reminder).**` paragraph after the dimensions section. Anchors the primer rule to Security's typical findings (information exposure, identity disclosure, secrets management).
- **`hooks/check-review-log-anonymization.sh`** (new) — suite-level pre-commit script. Reads `git config user.name` / `user.email` / `$HOME` at runtime (no identity values hardcoded). Scans IAR review-log markdown line-by-line; reports any match outside the public-URL allowlist (`github.com/`, `gitlab.com/`, `bitbucket.org/`, `noreply.*`). Designed for `pass_filenames: true` invocation; the caller (`.pre-commit-config.yaml`) is responsible for `files:` scoping. Lives at the suite level so a future spinoff carries the hook.
- **`.pre-commit-config.yaml`** (portfolio repo root) — new `id: review-log-anonymization` entry wires the hook, scoped via `files:` to IAR review-log markdown only (`vsdd-suite/.*\.md` and `.*/vsdd-suite/.*\.md`). Defense-in-depth alongside the existing `no-home-dir-paths` source-code hook.

### Changed
- **`issue-tracker-cli/iterative-adversarial-refinement/TECHNICAL-WRITER-REVIEW.md`** — Review 7 Finding 4 line scrubbed: a legacy bare-username citation (` `magnificentlycursed/guild-portfolio` GitHub URL`) was rewritten as `https://github.com/<user>/guild-portfolio` to clear the new hook's baseline. The substantive finding text is unchanged.

### Addressed
- **G-98 — Adversarial review logs can themselves leak the values they document.** The three additions above operate in defense-in-depth (instruction → domain anchor → enforcement). Surfaced by the `issue-tracker-cli` Layer 1 PROCESS.md retrospective: the user described a Platform Engineer review log that meta-leaked the username its anonymization hooks were defending against, requiring git history rewrite to scrub. Class is broader than anonymization — Security reviews citing leaked credentials, Privacy reviews citing real personal data, exhibit the same pattern.

---

## Unreleased — 2026-05-05 (Review 35: manual-testing-checklist runnable-step standard)

### Changed
- **`primers/1b-decomposition.md`** — `### Manual testing checklist` section rewritten. Replaced the five-bullet shorthand prompt (happy path / error states / empty state / persistence / edge cases) with a runnable-step standard requiring four properties per step: exact command, expected outcome (stdout/stderr/exit code/on-disk state), explicit clean-state setup when required, and binary install/uninstall/reinstall lifecycle when relevant. Added an "Audience" paragraph specifying that the tester is unfamiliar with the toolchain and the project. Inlined an "Example shape (one expanded step)" so the standard is anchored in a copyable form. The `**Manual Testing Checklist:**` block in the `## TODO.md format` example was reframed: items are now placeholders that expand into runnable step blocks, with an explicit "Step 0 — Update the installed binary" item when the layer changes runtime behavior. Addresses G-97.
- **`primers/1b-decomposition.md`** — Follow-up refinement (Review 35 Finding 2): expected-outcome requirement (#2) tightened to require literal output blocks for invariant output, with prose descriptions reserved for variable output (timestamps, IDs, OS-chosen paths) anchored to a representative literal example. Help-command specificity sub-clause added to requirement #4 (`<binary> <subcommand> --help`, not `<binary> --help`, when help text is part of verification). Example shape updated to show literal expected-stderr / expected-stdout / on-disk-state blocks. Triggered by user-discovered defect in the rendered Layer 4 plan: the prose hint "expect: --label flag listed under create + list" named the wrong help command (top-level instead of subcommand) and the prose form was ambiguous enough that the error went undetected at authoring time.
- **`primers/1b-decomposition.md`** — Second follow-up refinement (Review 35 Finding 3): added "Help-output verification (CLI projects)" as a sixth bullet in the **Required items per layer** list. Per binary and per changed subcommand, run `<binary> <subcommand> --help` and include a literal expected-output block. Anchored to "Step 0 — Update the installed binary" so a stale-binary problem fails fast. Carves out an explicit exception for layers that don't change the CLI surface. The defect this catches: a CLI layer ships with a flag whose runtime behavior works but whose `--help` description is missing, stale, or contradicts the actual flag — discoverability drift that integration tests do not catch. Considered and rejected: placing the rule in `supplements/cli.md`; supplements are currently review-time artifacts, not authoring-time; the cross-reference pattern would be novel.
- **`primers/1b-decomposition.md`** — Third follow-up refinement (Review 35 Finding 4): added "Usage examples in `--help` (CLI projects with compound flags or filters)" as a seventh bullet in the **Required items per layer** list. For projects whose subcommands accept multiple optional flags or compound filters, the polish/help-finalization layer's acceptance criteria must require usage examples in the relevant subcommand's `--help` output (1–3 examples per subcommand, covering common scenarios like compound filtering). The defect this catches: a user reading `--help` against a layer with five filters sees orthogonal flag descriptions but cannot tell which combinations make sense without imagining them — examples answer "how do I do the thing I came to do?" while a flag list only answers "what can I do?" Cross-references existing `supplements/cli.md` UX dim 1 (review-time, top-level) and extends the expectation to subcommand-level for compound-flag cases. Forward-only.
- **`suite-development/GAP-ANALYSIS-LOG.md`** — G-97 added (status `Addressed`) — manual-testing-checklist format produced tester-familiarity-dependent items. Distinct from G-42 (which addressed *which domain evaluates* checklist completion); G-97 addresses *the format the checklist itself takes when produced by decomposition*. Refinement (literal blocks vs prose) recorded in Review 35 Finding 2 stays within G-97's scope.

### Added
- **`suite-development/review-log/2026-05-05-suite-review.md`** — new file. Review 35 logged: one finding Resolved (decomposition-checklist format tightened); G-97 registered and immediately Addressed.
- **`suite-development/SUITE-REVIEW-INDEX.md`** — Review 35 row added to the Suite Reviews index.

### Note
Forward-only. Existing `TODO.md` files in projects under review are not retroactively rewritten; new layer plans (and re-decomposed layers) inherit the new standard. The change does not require domain-prompt updates: VDD-IAR Alignment dim 9 evaluates whether checklists exist and were completed, not their format quality, and format quality is owned implicitly by SO (spec coverage) and TW (documentation accuracy) when reading project TODO.md.

---

## Unreleased — 2026-05-03 (Review 34: apply G-90 and G-94, register G-96)

### Changed
- **`primers/1a-spec-crystallization.md`** — H1 retitled `(VSDD Phase 1)` → `(VSDD Phase 1a)`; in-prompt phase reference and governing-standard reference updated. Addresses G-90.
- **`README.md`** — VSDD pipeline table phase column `1` → `1a`; "spec issues to Phase 1" and "during Phase 1" updated to `Phase 1a`. `## Suite scope` and pipeline cross-references for `domains/role/DOMAIN-INDEX.md` and `SUITE-REVIEW.md` updated to `domains/DOMAIN-INDEX.md` and `suite-development/SUITE-REVIEW-INDEX.md`. Addresses G-90 and G-94 sub-issues 2, 3.
- **`suite-development/suite-development.md`** — H1 retitled to `# Session Primer: Suite Development (Meta — Suite Contributors)` (G-94 sub-issue 5). Suite-history paragraph "Phase 1 spec crystallization" → "Phase 1a" (G-90). `## Lang supplement coverage` heading → `## Supplement coverage`; suite-structure table row "Lang supplements" → "Language and interface supplements"; "Lang supplement reference" terminology generalized; `domains/role/DOMAIN-INDEX.md` references updated to `domains/DOMAIN-INDEX.md`; all `SUITE-REVIEW.md` references in current-spec lines updated to `suite-development/SUITE-REVIEW-INDEX.md`. Addresses G-94 sub-issues 1, 2, 3, 5.
- **`primers/3-review-session.md`** — `domains/role/DOMAIN-INDEX.md` reference updated; `SUITE-REVIEW.md` reference updated to `suite-development/SUITE-REVIEW-INDEX.md`. Addresses G-94 sub-issues 2, 3.
- **`domains/role/SOLUTION-OWNER-REVIEW.md`** — "scope-crept during Phase 1" updated to "VSDD Phase 1a" (G-90); "lang supplement" terminology generalized in Language-and-interface-supplement reference (G-94 sub-issue 1).
- **`domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md`** — "VSDD Phase 1 criteria" → "VSDD Phase 1a criteria" (dim 1); pipeline-list `(1, 1b, 2a, 2b, ...)` → `(1a, 1b, 2a, 2b, ...)` in the Program Phase Context note. Addresses G-90.
- **`domains/DOMAIN-INDEX.md`** — File moved from `domains/role/DOMAIN-INDEX.md` (G-94 sub-issue 2). Internal references updated: README link `../../README.md` → `../README.md`; meta-domain backticked paths `../meta/` → `meta/`; role-domain entries gained the `role/` prefix for symmetry with the meta entries; `lang/cli.md` reference updated to `supplements/cli.md`.
- **`suite-development/SUITE-REVIEW-INDEX.md`** — File renamed from `SUITE-REVIEW.md` (G-94 sub-issue 3).
- **`supplements/`** — Folder renamed from `lang/`. Path references updated in 24 forward-facing and historical `*.md` files via bulk sed; nonsensical strings produced as side effects in some prior-session narratives were repaired post-bulk. Addresses G-94 sub-issue 1.
- **`suite-development/GAP-ANALYSIS-LOG.md`** — G-90 status `Open` → `Addressed`; description rewritten to record the resolution path (Option 1, whitepaper-confirmed). G-94 status `Open` → `Addressed (partial — sub-issue 4 deferred to spinoff-MVP)`; description rewritten to record per-sub-issue outcome. G-96 added (Open) — suite's Phase 1a/1b/2a/2b sub-phase semantics diverge from the whitepaper's Step 1a/1b/1c, 2a/2b/2c structure; resolution path is harmonize-or-document-the-divergence, deferred to evaluation.

### Added
- **`suite-development/review-log/2026-05-03-suite-review.md`** — Review 34 logged: applied G-90 and G-94 sub-issues 1, 2, 3, 5; deferred G-94 sub-issue 4 (CHANGELOG release tagging) to spinoff-MVP; registered G-96 (whitepaper sub-phase semantic divergence). Two findings resolved, one new gap registered.
- **`suite-development/SUITE-REVIEW-INDEX.md`** — Review 34 row added to the Suite Reviews index.

### Note
G-94 sub-issue 4 (CHANGELOG release tagging) remains Open inside G-94's "partial" status. Both proposed alternatives — tag spinoff-MVP as `1.0.0`, or rename `## Unreleased` framing to `## Session N` — require either an event that has not occurred or a renumbering that conflicts with the "do not silently amend prior findings" discipline. Reconsider at the spinoff-MVP boundary alongside the rest of the deferred restructure work. G-96 (whitepaper sub-phase divergence) is a deeper finding than G-90 — G-90 fixed the labelling asymmetry; G-96 names the underlying semantic mismatch and offers harmonize-vs-document-divergence as the resolution menu. No action taken on G-96 in this session beyond registration.

---

## Unreleased — 2026-05-03 (Review 33: bundled-deferral dependency analysis)

### Changed
- **`suite-development/GAP-ANALYSIS-LOG.md`** — G-90 (VSDD phase numbering inconsistency) status `Deferred` → `Open`. Decoupled from the `issue-tracker-cli` trigger because the gap depends on the upstream VSDD whitepaper, not on project feedback. Description amended in place; row links updated to reference Review 33 as the decoupling record. Addresses Review 33 Finding 1.
- **`suite-development/GAP-ANALYSIS-LOG.md`** — G-94 (smaller naming/location bundle) status `Deferred` → `Open`. Decoupled from the `issue-tracker-cli` trigger because the five sub-issues are mechanical or low-coordination and do not depend on real-project pressure or reading patterns. May still travel with the G-88/G-91/G-92/G-93 spinoff-time restructure if convenient, or be addressed independently. Description amended in place. Addresses Review 33 Finding 1.

### Added
- **`suite-development/review-log/2026-05-03-suite-review.md`** — Review 33 logged: per-gap dependency analysis of the eight gaps deferred to "after `issue-tracker-cli` completes." Two findings resolved (G-90 and G-94 promoted to Open with rationale; G-88, G-89, G-91, G-92, G-93, G-95 reviewed and confirmed properly Deferred).
- **`SUITE-REVIEW.md`** — Review 33 row added to the Suite Reviews index.

### Note
G-88, G-89, G-91, G-92, G-93, G-95 remain Deferred under the `issue-tracker-cli` trigger. Their substance genuinely depends on real-project feedback (G-88, G-89), forward-only path constraints driven by in-flight project references (G-91), or coordination with other deferred gaps (G-92, G-93, G-95). The bundled-trigger pattern is preserved for these six.

---

## Unreleased — 2026-05-03 (Review 32: suite-review entry-format and deferral-trigger consistency)

### Changed
- **`suite-development/suite-development.md`** — Updated `### Suite review entry format` item 3 (Lens) to enumerate three valid forms: named defect class, registry-walk scope, and role-based lens (covering both domain-perspective and named-bundle variants). Reflects practice in Reviews 30 and 31; addresses Review 32 Finding 3.
- **`suite-development/suite-development.md`** — Updated `### Suite review entry format` item 6 (Closing) to permit an optional `### Coordination` section after the classification sections, used to name a cross-finding cluster and bundled action. Markdown links required for cross-references. Addresses Review 32 Finding 1; retroactively conforms Review 31's `### Coordination` section.
- **`suite-development/suite-development.md`** — Added `### Session isolation` subsection to `## Suite review and review-log discipline`. Documents that suite reviews are typically in-session (unlike domain reviews); cold-session is permitted and stronger but not required; minimum standard is an explicit session note naming cold-vs-in-session status and, if in-session, naming a compensation. A missing session note is itself a finding for VDD-IAR Alignment dim 7 applied to the suite. Addresses Review 32 Finding 4.
- **`suite-development/GAP-ANALYSIS-LOG.md`** — Added `## Reactivation triggers` subsection between `## How to run a gap analysis` and `## Gap Registry`. Defines the bundled trigger "after `issue-tracker-cli` completes" with three required conditions (all layers merged, final-merge VDD-IAR Alignment classified, project archived). Names abandonment/pivot path; permits decoupling for gaps not actually dependent on `issue-tracker-cli` feedback. Addresses Review 32 Finding 2.

### Added
- **`suite-development/review-log/2026-05-03-suite-review.md`** — Review 32 logged: suite-review entry-format and deferral-trigger consistency. Four findings resolved in-session (F1 Coordination heading, F2 trigger definition, F3 lens grammar, F4 session isolation). Two hallucinated (re-raised directory rename — already deferred as G-88; per-entry sycophancy section — boilerplate that G-77 already corrected).
- **`SUITE-REVIEW.md`** — Review 32 row added to the Suite Reviews index.

---

## Unreleased — 2026-05-03 (Review 31: five-lens adversarial pass, structural findings deferred)

### Added
- **`suite-development/review-log/2026-05-03-suite-review.md`** — Review 31 logged: five-lens adversarial review (clarity, naming, ambiguity, consistency, transitional-state alignment) framed for the eventual standalone-repo spinoff. Six new gaps registered (G-90 through G-95), all Deferred to post-`issue-tracker-cli` completion under the same forward-only constraint that applies to G-88 and G-89. Two findings hallucinated (registry/index merge; symmetric completion-criteria sections in all primers).
- **`SUITE-REVIEW.md`** — Review 31 row added to the Suite Reviews index.
- **`suite-development/GAP-ANALYSIS-LOG.md`** — G-90 (phase numbering inconsistency), G-91 (primer folder + file naming), G-92 (suite-meta vs suite-running separation), G-93 (user vs contributor delineation), G-94 (smaller naming/location bundle: `supplements/` folder, `DOMAIN-INDEX.md` location, `SUITE-REVIEW.md` filename, CHANGELOG release tagging, primer H1 convention), G-95 (`2b-implementation.md` covers two distinct phases) — all registered Deferred. Coordinated with G-88 (directory rename) and G-89 (project-level review log structure standardization) as a single bundled restructure pass at the spinoff-MVP boundary.

### Note
No suite artifacts were modified in this session beyond the registry, index, session log, and changelog. Per user instruction and consistent with the G-88/G-89 deferral pattern, structural changes are forward-only and will not be retroactively applied to completed projects (notably `bookmark-manager` and the in-flight `issue-tracker-cli`). The bundled application pass is expected to coincide with the suite's spinoff into a standalone repository.

---

## Unreleased — 2026-05-03 (suite scope acknowledgement)

### Changed
- **`README.md`** — Added `## Suite scope` section between the IAR intro and `## VSDD pipeline context`. Names the transitional state explicitly: directory began as IAR-only and has grown to house session primers for adjacent VSDD phases; directory name and "IAR" identity retained for continuity. Lists the four artifact categories (domain prompts, phase primers, lang supplements, suite governance). References the Phase 4–6 gaps (G-86, G-55, G-54). Addresses Review 30 Finding 1.
- **`README.md`** — Added "Primer" column to the VSDD pipeline table referencing each phase's primer file. Phases 1, 1b, 2a, 2b, 3 link to their primers; Phase 4 cell shows `— (G-86)`; Phase 5 cell shows `— (G-55)`; Phase 6 cell shows `—`. Updated the trailing sentence below the table to point to the full primer table under `## Session primers` rather than naming only Phase 1/1b. Addresses Review 30 Finding 2.
- **`suite-development/suite-development.md`** — Updated `## Prompt` opening: added a paragraph after the artifact list naming the broader scope explicitly (suite has expanded beyond Phase 3; directory name retained for continuity; pointer to README scope section). Generalized the adversarial-standard paragraph with a sentence covering construction primers (a primer's `## Prompt` without a concrete failure mode produces softer output; a non-falsifiable completion-criteria section will pass against incomplete artifacts). Addresses Review 30 Finding 3.
- **`SUITE-REVIEW.md`** — Updated lead paragraph to include session primers and lang supplements in the implementation list, and to acknowledge the adversarial standard applies to both review prompts and constructive primers. Added pointer to `README.md` `## Suite scope` for the artifact map. Addresses Review 30 Finding 4.
- **`suite-development/GAP-ANALYSIS-LOG.md`** — Added G-87: scope expansion was implicit; directory still named/framed as IAR-only despite housing primers for adjacent VSDD phases. Marked Addressed in same session by Review 30 Findings 1, 3, 4.
- **`suite-development/GAP-ANALYSIS-LOG.md`** — Added G-88 (Deferred): revisit suite directory name and "IAR" identity after `issue-tracker-cli` completes. Forward-only constraint — any rename applies to projects whose first IAR run is after the decision; completed projects retain their existing `vsdd-suite/` review-log paths.
- **`suite-development/GAP-ANALYSIS-LOG.md`** — Added G-89 (Deferred): standardize project-level domain review log structure on the suite-review pattern (per-domain index file + dated session entries in `suite-development/review-log/`). Trigger: revisit after `issue-tracker-cli` completes. Forward-only constraint — completed projects retain their existing single-file domain logs.

### Added
- **`suite-development/review-log/2026-05-03-suite-review.md`** — Review 30 logged: SO + TW + VDD-IAR alignment scan of the suite's scope and identity. 4 findings resolved, 2 hallucinated (directory rename, primer scope-section), 3 new gaps registered (G-87 immediately addressed, G-88 and G-89 deferred to post-issue-tracker-cli).
- **`SUITE-REVIEW.md`** — Review 30 row added to the Suite Reviews index.

---

## Unreleased — 2026-05-02 (suite review collapse)

### Changed
- **`suite-development/suite-development.md`** — Collapsed two parallel suite-review artifact types (meta-review and gap analysis run) into a single **Suite Review** type. Replaced the two entry-format specifications with one **Suite review entry format** subsection covering both modes (defect-search lens and registry-walk lens). The mode now lives in the `Lens` field rather than in a separate artifact. Updated cross-cutting wording in "Before adding a domain", "Before modifying a domain", and "Running gap analysis" to reference the unified type. Added project-level review log governing standard (introduced earlier in this session) referenced for finding-body shape.
- **`primers/3-review-session.md`** — Updated "If reviewing the IAR suite itself" to reference the unified `suite-development/review-log/YYYY-MM-DD-suite-review.md` filename pattern and the single **Suite Reviews** index table. Removed dual-type framing.
- **`SUITE-REVIEW.md`** — Replaced the two tables (Suite Meta-Reviews + Gap Analysis Runs) with a single **Suite Reviews** table. Renumbered all 29 sessions chronologically as Review 1–29 (oldest = Run 1, newest = Review 16). Added migration footnote with old→new mapping. Reading-convention text updated.
- **`suite-development/review-log/`** — Renamed and merged session files: `2026-04-25-gap-analysis.md` → `2026-04-25-suite-review.md`; `2026-04-26-gap-analysis.md` → `2026-04-26-suite-review.md`; `2026-04-27-{gap-analysis,meta-review}.md` merged into `2026-04-27-suite-review.md`; `2026-04-28-{gap-analysis,meta-review}.md` merged into `2026-04-28-suite-review.md`; `2026-05-01-meta-review.md` → `2026-05-01-suite-review.md`. All H1 titles and `## Review N` / `## Gap Analysis Run N` headings updated to the unified numbering. Within-session prose references updated where unambiguous.
- **`suite-development/GAP-ANALYSIS-LOG.md`** — Updated all 87 Markdown links to point to renamed files and renumbered anchors. Updated step 6 of "How to run a gap analysis" to direct entries to `suite-development/review-log/YYYY-MM-DD-suite-review.md`. Updated trailing prose about narrative location.
- **Project-level review logs** under `issue-tracker-cli/iterative-adversarial-refinement/` — earlier in this session, applied the project-level review log governing standard to all 13 domain logs: file-level reviewer-role/activation/sycophancy headers, dim-ref parentheticals on every finding title, classification-first finding sections, Markdown-linked cross-references, unified Resolution/Classification closer, Summary + Coordination closing. Portfolio Assessment retained its dim-first organization as a documented exception.

### Note
Historical CHANGELOG entries below this one preserve their original wording and reference the pre-collapse artifact names ("meta-review", "gap analysis run", "Run N", "Review N" under the old numbering). Those entries describe state at points in time and are intentionally not rewritten.

---

## Unreleased — 2026-05-01 (session 18, follow-up)

### Changed
- **`primers/1b-decomposition.md`** — Added "Primary failure mode" paragraph to `## Prompt` section: names the specific failure mode for decomposition sessions (accepting all proposed layers without challenge). The governing standard requires the `## Prompt` section to name a primary failure mode; `1b-decomposition.md` lacked this while `3-review-session.md` and others had explicit equivalents. Addresses finding from Review 16.
- **`primers/2b-implementation.md`** — Updated Phase 2b item 2 to define the retroactive Red Gate deviation protocol. Retroactive tests (discovered during implementation) cannot satisfy the Red Gate; the primer now requires them to be labeled as deviations in commit message and review log. Addresses finding from Review 16.
- **`primers/1a-spec-crystallization.md`** — Added dedicated `## Completion criteria` section with six numbered criteria drawn from VSDD Phase 1 standard. Removed embedded completion sentence from `## Self-adversary check` to avoid duplication. Structural consistency with `2b-implementation.md` and `1b-decomposition.md`. Addresses finding from Review 16.
- **`suite-development/GAP-ANALYSIS-LOG.md`** — Added G-86: No VSDD Phase 4 (Feedback Routing) session primer. Open.
- **`SUITE-REVIEW.md`** — Review 16 logged: VDD-IAR and VSDD alignment review of all session primers in `primers/`. 3 findings resolved, 2 hallucinated, 1 new gap registered (G-86).

### Changed (follow-up — same session, post-violation detection)
- **`primers/2b-implementation.md`** — Added Phase 2a step 4: explicit requirement to commit the Red Gate state before Phase 2b begins. The commit is the boundary between phases; implementation before that commit makes test-first discipline unverifiable from history. Updated Phase 2b opening to reference "set, confirmed failing, and committed." Finding surfaced by actual violation during Layer 2 implementation: Red Gate was confirmed in the working tree but not committed before implementations were written; the resulting "Red Gate" commit contained real implementations, not stubs.
- **`SUITE-REVIEW.md`** — Finding 6 added to Review 16 record: describes the violation, the primer gap that permitted it, and the resolution.

---

## Unreleased — 2026-04-27 (session 17)

### Changed
- **`README.md`** — Security Engineer Focus cell updated to include "audit logging, data classification and control requirements" — reflects dims 7 and 8 added in session 15. Cell was stale after scope expansion.
- **`README.md`** — Solution Architect Focus cell updated to include "external service integration" — reflects `### Extended: External Service Integration` (dims 23–27) added in session 15. Cell was stale after scope expansion.
- **`domains/role/SOLUTION-ARCHITECT-REVIEW.md`** — Added Privacy to coordination links: "dim 27 — data transmitted to external services; cross-reference with Privacy dim 6 when Privacy is active." Privacy was already cross-referenced inside dim 27's text but absent from the coordination section.

---

## Unreleased — 2026-04-28 (session 16)

### Changed
- **`domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md`** — Added cross-session spec consistency sub-section to dim 7 (IAR iteration and feedback routing). Named failure mode: AI interpretation of requirements shifts between sessions without a DESIGN.md update. Concrete test: can the current DESIGN.md, read cold, produce the current implementation? Named artifact indicators: commits contradicting DESIGN.md, DECISIONS.md entries expanding features without a spec revision, IAR findings about behavior absent from the spec. Addresses G-22.
- **`domains/role/SOLUTION-ARCHITECT-REVIEW.md`** — Added feature-enhancement activation note to `### Extended: External Interface Contracts` section. Dims 16 (backward compatibility) and 17 (contract testing) now explicitly activate for feature enhancements — any change that existing callers, users, or stored data must survive. Addresses G-30.
- **`suite-development/GAP-ANALYSIS-LOG.md`** — Updated statuses: G-22 Open → Addressed; G-30 Open → Addressed.

---

## Unreleased — 2026-04-28 (session 15)

### Added
- **`domains/role/SECURITY-REVIEW.md`** — Added dim 7 (Audit logging): named audit events, tamper-evidence requirement, retention and separation from application logs, forensic reconstruction test, context-scoped note for single-user vs. enterprise deployment. Addresses G-09.
- **`domains/role/SECURITY-REVIEW.md`** — Added dim 8 (Data classification and control requirements): classification tiers (public/internal/confidential/restricted), proportionate control requirements, named failure modes, explicit cross-reference to Privacy dim 1 for coordination. Addresses G-10. Ownership decision: Privacy dim 1 owns data identification; Security dim 8 owns control mandates from classification.
- **`domains/role/SOLUTION-ARCHITECT-REVIEW.md`** — Added `### Extended: External Service Integration` section (dims 23–27): external dependency inventory, failure and timeout handling, API contract drift, credentials to external services, data transmitted to external services with cross-reference to Privacy dim 6. Addresses G-32.

### Changed
- **`domains/role/SECURITY-REVIEW.md`** — Added Privacy to coordination links (dim 8 data classification cross-references Privacy dim 1). Addresses G-09/G-10.
- **`suite-development/GAP-ANALYSIS-LOG.md`** — Updated statuses: G-09 Open → Addressed; G-10 Open → Addressed; G-32 Open → Addressed; G-36 Open → Dismissed (business viability is out of IAR scope, no natural reviewer role).

---

## Unreleased — 2026-04-28 (session 14)

### Added
- **`supplements/javascript-typescript.md`** — Added `## Technical Writer` section: TypeDoc/JSDoc generation config, TSDoc comment completeness (`@param`/`@returns`/`@throws`/`@example`), README example accuracy, `@deprecated` markers. Addresses G-84.
- **`supplements/javascript-typescript.md`** — Added `## Localization` section: `Intl.*` API usage with explicit locale parameters, i18next/react-i18next configuration, missing translation key handling, locale injection in tests. Addresses G-85.
- **`supplements/rust.md`** — Added `## Technical Writer` section: rustdoc coverage (`cargo doc --no-deps`), doc test quality (`cargo test --doc`), module-level `//!` docs, `#[doc(hidden)]` discipline, `cargo doc --document-private-items`. Addresses G-84.
- **`supplements/rust.md`** — Added `## Localization` section: fluent-rs bundle configuration with `LanguageIdentifier` and fallback chains, Fluent message completeness, missing message error handling, rust-i18n macro usage and key coverage. Addresses G-85.

### Changed
- **`domains/role/TECHNICAL-WRITER-REVIEW.md`** — Updated lang supplement note from gap-reference language ("not yet covered — see G-84") to standard "Apply the **Technical Writer** section" format. Addresses G-84.
- **`domains/role/LOCALIZATION-REVIEW.md`** — Updated lang supplement note from gap-reference language ("not yet covered — see G-85") to standard "Apply the **Localization** section" format. Addresses G-85.
- **`domains/role/SECURITY-REVIEW.md`** — Revised inline G-07 note at end of dim 6. Previous note said "G-07 is still open... single dimension above is insufficient." Security dim 6 has been substantially expanded since that note was written; the note was stale and contradicted the current dimension content. Replaced with forward-looking guidance: for complex multi-user auth, a dedicated domain may be warranted. Gap log updated to Addressed (partial) for G-07 and G-08.
- **`suite-development/suite-development.md`** — Removed gap markers (`**Gap**`) from Technical Writer and Localization rows in the lang supplement coverage table; updated to ✓. Removed now-stale trailing sentence referencing G-84/G-85. Addresses G-84, G-85.
- **`suite-development/GAP-ANALYSIS-LOG.md`** — Updated statuses: G-07 Open → Addressed (partial); G-08 Open → Addressed (partial); G-24 Open → Addressed (partial); G-25 Open → Addressed (partial); G-84 Open → Addressed; G-85 Open → Addressed.

---

## Unreleased — 2026-04-28 (session 13)

### Changed
- **`domains/role/DATA-ENGINEER-REVIEW.md`** — Added parenthetical job title variants to the reviewer role line. Previous: `**Reviewer role: Data Engineer**`. Updated: `**Reviewer role: Data Engineer** (Data Engineer / Database Engineer / Data Platform Engineer)`. Every other role domain follows the governing standard format `[Title] ([variants])`; DE was the only exception. Addresses Review 14 Finding 1.
- **`README.md`** — Updated core domain table "Job title" column for Data Engineer from "Data Engineer" to "Data Engineer / Database Engineer / Data Platform Engineer" to match the domain file and align with the slash-delimited variant format used by all other rows. Addresses Review 14 Finding 1.

---

## Unreleased — 2026-04-28 (session 12)

### Changed
- **`primers/3-review-session.md`** — Added VDD-IAR Alignment to the Deferred classification exclusion note. Previous note said "Not valid for Security or Red Team" — VDD-IAR Alignment also prohibits deferred (governing standard: process findings are binary). A reviewer following the primer could incorrectly defer a VDD-IAR Alignment finding. New note: "Not valid for Security, Red Team, or VDD-IAR Alignment." Addresses Review 13 Finding 1.

---

## Unreleased — 2026-04-28 (session 11)

### Changed
- **`CHANGELOG.md`** — Fixed "AIR suite" → "IAR suite" in the file description. Addresses Review 12 Finding 1.
- **`domains/role/PERFORMANCE-ENGINEER-REVIEW.md`** — Rewrote lang supplement note to match the standard format used by all other domains: "Apply the **Performance Engineer** section from the relevant supplement file in addition to the standard dimensions below." Previous note said only "Consult `../../supplements/`" without naming the section. Addresses Review 12 Finding 2.
- **`domains/role/SOLUTION-OWNER-REVIEW.md`** — Replaced supplement note with an explicit opt-out. Previous note directed reviewer to "consult the supplement" for technology choice verification, but no SO section exists in any supplement — `suite-development.md` table marks SO as Language-agnostic. New note clarifies: SO is language-agnostic; for technology fitness context, consult the SA section of the relevant supplement. Addresses Review 12 Finding 3.

---

## Unreleased — 2026-04-28 (session 10)

### Changed
- **`domains/role/SOLUTION-ARCHITECT-REVIEW.md`** — Added `[DATA-ENGINEER-REVIEW.md]` to SA's coordination links. SA dim 3 evaluates data model integrity; DE is the natural escalation target for deeper data-layer analysis. DE was the only core domain absent from SA coordination despite the explicit overlap. Addresses Review 11 Finding 1.
- **`supplements/javascript-typescript.md`** — Added "Coverage enforcement" bullet to Platform Engineering section naming Jest `coverageThreshold`, Vitest `coverage.thresholds`, `c8`, and `nyc` as the JS/TS-specific coverage tooling. Symmetric with `rust.md` Platform Engineering section. Addresses Review 11 Finding 2.
- **`domains/role/ACCESSIBILITY-REVIEW.md`** — Updated lang supplement note to specify "See the **UX** section of `../../supplements/browser-app.md`" and name the content. `browser-app.md` has no `## Accessibility` section; the relevant dimensions live in `## UX`. Addresses Review 11 Finding 3.

---

## Unreleased — 2026-04-28 (session 9)

### Changed
- **`README.md`** — Fixed stale "Phase 4" reference in session primers section: "the adversary applies pressure during Phase 1, not only during Phase 4" → "Phase 3". Renumbering pass in Review 6 missed this sentence. Addresses Review 10 Finding 1.
- **`domains/role/QUALITY-ENGINEER-REVIEW.md`** — Added `[SOFTWARE-ENGINEER-REVIEW.md]` as the first entry in QE's coordination links. SE was absent despite the domain boundary text explicitly describing the QE/SE split: QE flags missing tests, SE flags bugs. Addresses Review 10 Finding 2.
- **`domains/meta/PORTFOLIO-ASSESSMENT-REVIEW.md`** — Regression check now includes instruction to read the preceding project's `PORTFOLIO-ASSESSMENT-REVIEW.md` log and a note that the check is vacuously met if no prior assessment exists. Addresses Review 10 Finding 3.
- **`domains/meta/PORTFOLIO-ASSESSMENT-REVIEW.md`** — Dim 8 rewritten: replaced "the developer could have built this without AI assistance" with a framing based on ownership of scope. The governing methodology assumes AI does the building; the test is whether the developer directed and owns the complexity, not whether they could have built it solo. Addresses Review 10 Finding 4.

---

## Unreleased — 2026-04-28 (session 8)

### Changed
- **`supplements/rust.md`** — Removed six inline "(Source: claude.md; verify against current apprentice-onboarding content.)" annotations from dimension bullets across Quality Engineering, Security, Software Engineering, and Platform Engineering sections. Source provenance is now consolidated in the `**Source note:**` paragraph added at the top of the file. Addresses Review 9 Finding 1.
- **`suite-development/GAP-ANALYSIS-LOG.md`** — Updated G-12 status from "Addressed (API-CONTRACT-REVIEW.md)" to "Addressed (SA Extended: External Interface Contracts)". The referenced file does not exist; the gap was addressed by SA's Extended: External Interface Contracts section. Addresses Review 9 Finding 2.
- **`suite-development/GAP-ANALYSIS-LOG.md`** — Updated G-20, G-21, G-23 status from "Open" to "Addressed (partial)" and Last Reviewed from 2026-04-25 to 2026-04-27, consistent with G-76 which registered the partial addressing. Addresses Review 9 Finding 3.
- **`supplements/cli.md`** — Removed "(or alongside)" from intro paragraph. Intro now reads "in place of the standard UX dimensions," matching the section header which states the CLI dimensions replace browser-centric UX dimensions. Addresses Review 9 Finding 4.

---

## Unreleased — 2026-04-27 (session 7)

### Changed
- **`primers/2b-implementation.md`** — Updated H1, prompt text, and internal section headers from "Phase 2–3" / "Phase 2" / "Phase 3" to "Phase 2a–2b" / "Phase 2a" / "Phase 2b" to match the renumbering from Review 6. Addresses Review 8 Finding 1.
- **`domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md`** — Added disambiguation note to Program Phase Context section clarifying "Phase" refers to apprentice program tiers, not VSDD pipeline phases. Addresses Review 8 Finding 2.
- **`domains/role/ACCESSIBILITY-REVIEW.md`** — Removed dim 13 ("Regression") which duplicated the regression check paragraph already present in the Current Review Prompt section. Addresses Review 8 Finding 3.
- **`domains/role/TECHNICAL-WRITER-REVIEW.md`**, **`ACCESSIBILITY-REVIEW.md`**, **`LOCALIZATION-REVIEW.md`**, **`PERFORMANCE-ENGINEER-REVIEW.md`**, **`PRIVACY-REVIEW.md`** — Converted prose coordination sections to Markdown links with relative paths and parenthetical context preserved. **`RED-TEAM-REVIEW.md`** — Fixed abbreviated display name "[SA-REVIEW.md]" → "[SOLUTION-ARCHITECT-REVIEW.md]". Addresses Review 8 Finding 4.
- **`README.md`** — Added cross-reference to `PORTFOLIO-ASSESSMENT-REVIEW.md` in portfolio-arc review section. Addresses Review 8 Finding 5.

---

## Unreleased — 2026-04-27 (session 6)

### Changed
- **`domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md`** — Added regression check paragraph in correct position (before Coordination). Moved sycophancy check to before lang supplement (was after). Addresses Review 7 Finding 1.
- **`domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md`** — Dim 7 extended with feedback routing fidelity sub-criterion: findings must route to the appropriate earlier phase (spec findings → DESIGN.md, test findings → test suite, implementation findings → code). Addresses Review 7 Finding 3.
- **`domains/meta/PORTFOLIO-ASSESSMENT-REVIEW.md`** — Added "Read DESIGN.md and the assignment brief" instruction. Added regression check paragraph. Moved sycophancy check to after Coordination (was before). Addresses Review 7 Finding 2.

---

## Unreleased — 2026-04-27 (session 5)

### Changed
- **`README.md`** — Phase numbering aligned with VSDD whitepaper. Pipeline table: phases renamed 2→2a, 3→2b, 4→3; Phase 4 (Feedback Integration) row added (was absent). Opening paragraph and VSDD pipeline context section updated to "IAR owns Phase 3." Session primer table updated: "Phase 2–3" → "Phase 2a–2b"; "Phase 4" → "Phase 3." Addresses Review 6 Finding 1.
- **`README.md`** — Added same-model review limitation note to Session isolation section. Addresses Review 6 Finding 2.
- **`primers/3-review-session.md`** — H1 title and posture paragraph updated from "VSDD Phase 4" to "VSDD Phase 3." Addresses Review 6 Finding 1.
- **`domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md`** — Purpose statement updated from "VSDD Phase 4 (Adversarial Refinement)" to "VSDD Phase 3." Addresses Review 6 Finding 1.

---

## Unreleased — 2026-04-27 (session 4)

### Changed
- **`domains/role/TECHNICAL-WRITER-REVIEW.md`**, **`LOCALIZATION-REVIEW.md`**, **`ACCESSIBILITY-REVIEW.md`**, **`PRIVACY-REVIEW.md`**, **`PERFORMANCE-ENGINEER-REVIEW.md`** — Regression check paragraphs moved to correct position: before Coordination, not after. Ordering violation introduced in session 3 resolution pass. Addresses Finding 1 from Review 5.
- **`domains/role/SOLUTION-OWNER-REVIEW.md`** — Added regression check paragraph in correct position (before Coordination). Addresses Finding 2 from Review 5.
- **`domains/role/RED-TEAM-REVIEW.md`** — Added regression check paragraph in correct position (before Coordination). Addresses Finding 3 from Review 5.
- **`domains/role/DOMAIN-INDEX.md`** — Fixed broken relative paths to meta domain files: `../../meta/` → `../meta/`. Addresses Finding 4 from Review 5.
- **`README.md`** — Data Engineer Focus column: removed activation guidance ("Optional for projects without a meaningful data layer") which belongs exclusively in DOMAIN-INDEX; replaced with reference to DOMAIN-INDEX for scope-down guidance. Addresses Finding 5 from Review 5. **README vs. DOMAIN-INDEX:** These files serve distinct purposes and are not redundant. README describes what domains cover; DOMAIN-INDEX is authoritative for when and whether domains activate.
- **`domains/role/DATA-ENGINEER-REVIEW.md`** — Added PRIVACY-REVIEW.md to coordination links with escalation note for dim 9 findings. Addresses Finding 10 from Review 5.
- **`domains/meta/PORTFOLIO-ASSESSMENT-REVIEW.md`** — Coordination links converted from prose to relative-path Markdown links. Addresses Finding 11 from Review 5.
- **`domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md`** — Moved `## Governing References` section from before `## Current Review Prompt` to between the prompt section and `## Standard Evaluation Dimensions`. Added prerequisite preamble framing. Addresses Finding 7 from Review 5.
- **`domains/role/SOFTWARE-ENGINEER-REVIEW.md`** — Added coordination notes to both Extended sections: Extended: Documentation defers to TW when TW is active; Extended: Performance defers to PE when PE is active. Addresses Finding 8 from Review 5.
- **`primers/1a-spec-crystallization.md`** — Moved `## Project type` section from before `## Prompt` to after it (between `## Prompt` closing separator and `## Project description`), satisfying the governing standard's required element ordering. Addresses Finding 9 from Review 5.
- **`suite-development/suite-development.md`** — Added explicit meta-domain exception to element 3 (Reviewer role line): meta domains in `domains/meta/` are exempt from the reviewer role requirement by design. Addresses Finding 6 from Review 5.
- **`primers/3-review-session.md`** — Rewrote the "After each domain review" classification section as a complete taxonomy with per-domain callouts for non-standard classifications. Added guidance that domain file schemas are authoritative. Addresses Finding 12 from Review 5.
- **`SUITE-REVIEW.md`** — Added Review 5 entry covering all 12 findings.

---

## Unreleased — 2026-04-27 (session 3)

### Changed
- **`SUITE-REVIEW.md`** — Added `## Suite Meta-Reviews` section header before `## Review 3` to match the structural organization of `## Gap Analysis Runs`. Added Review 4 entry documenting all 11 findings from the adversarial pass this session.
- **`domains/role/TECHNICAL-WRITER-REVIEW.md`** — Supplement reference updated to acknowledge G-84 (open gap) and removed false implication that a `supplements/` section exists. Added regression check paragraph. Addresses Finding 1 and Finding 6 from Review 4.
- **`domains/role/LOCALIZATION-REVIEW.md`** — Supplement reference updated to acknowledge G-85 (new open gap) and removed false implication that a `supplements/` section exists. Added regression check paragraph. Addresses Finding 2 and Finding 6 from Review 4.
- **`domains/role/RED-TEAM-REVIEW.md`** — Supplement reference corrected: "Apply the **Security** section" → "Apply the **Red Team** section." Addresses Finding 3 from Review 4.
- **`domains/role/SOLUTION-OWNER-REVIEW.md`** — Sycophancy check rewritten with domain-specific failure mode: an agent that helped write DESIGN.md will not flag scope creep it introduced. Addresses Finding 5 from Review 4.
- **`domains/role/UX-REVIEW.md`** — Sycophancy check rewritten with domain-specific failure mode: an AI cannot experience a UI. Added note to dim 7 directing deeper accessibility coverage to the Accessibility domain. Addresses Finding 5 and Finding 11 from Review 4.
- **`domains/role/DATA-ENGINEER-REVIEW.md`** — Sycophancy check rewritten with domain-specific failure mode: an agent that designed the data model will not question schema decisions. Addresses Finding 5 from Review 4.
- **`domains/role/ACCESSIBILITY-REVIEW.md`** — Added regression check paragraph. Addresses Finding 6 from Review 4.
- **`domains/role/PRIVACY-REVIEW.md`** — Added regression check paragraph. Addresses Finding 6 from Review 4.
- **`domains/role/PERFORMANCE-ENGINEER-REVIEW.md`** — Added regression check paragraph. Addresses Finding 6 from Review 4.
- **`README.md`** — Candidate domains list updated: removed Performance, Privacy, and Internationalisation (all now implemented extended domains). Expanded "Review logs" example file tree to include the 6 extended domain log files with an "include only when active" note. Addresses Finding 4 and Finding 8 from Review 4.
- **`suite-development/suite-development.md`** — Classification schemas table expanded: added Privacy (`accepted risk`), Localization (`accepted scope`), and Portfolio Assessment (`demonstrated`/`partial`/`absent`/`hallucinated`). Added numbered format definition for gap analysis run entries in `SUITE-REVIEW.md` to "SUITE-REVIEW.md discipline" section. Coverage table: Localization row updated from "Language-agnostic" to gap (G-85); closing note updated to reference both G-84 and G-85. Addresses Finding 7, Finding 10, and Finding 2 from Review 4.
- **`suite-development/GAP-ANALYSIS-LOG.md`** — Added G-85 (Localization lang supplement absent). Updated G-77 status from `Addressed (QE/Security/SA/SE)` to `Addressed` (now fully resolved across SO, UX, DE as well).

---

## Unreleased — 2026-04-27 (session 2)

### Changed
- **`suite-development/GAP-ANALYSIS-LOG.md`** — Structural consolidation: run narratives (Runs 1–10) stripped from this file. The file now contains only the gap registry table and file header. Run narrative content moved to `SUITE-REVIEW.md`. Each registry ID is now a Markdown link to the `## Gap Analysis Run N` section in `SUITE-REVIEW.md` where that gap was first identified. G-84 links to `SUITE-REVIEW.md#review-3--2026-04-27` (identified in a meta-review, not a gap analysis run). "How to run a gap analysis" step 6 updated to direct run entries to `SUITE-REVIEW.md`.
- **`SUITE-REVIEW.md`** — Added `## Gap Analysis Runs` section containing all ten gap analysis run narratives (Runs 1–10) in reverse chronological order, consistent with the file's existing convention.
- **`suite-development/suite-development.md`** — "Running gap analysis" section updated: the closing entry requirement now explicitly names `SUITE-REVIEW.md` as the target for run narratives, with a clarifying note that `suite-development/GAP-ANALYSIS-LOG.md` contains only the registry.

---

## Unreleased — 2026-04-27

### Added
- **`domains/role/DOMAIN-INDEX.md`** — Authoritative classification of core vs. extended domains with activation criteria per extended domain. Supplements the README domain tables with filesystem-local reference and explicit conditions under which each extended domain becomes active.
- **`primers/2b-implementation.md`** — New session primer for VSDD Phase 2–3 (Red Gate and Implementation). Establishes tests-before-code posture, driving questions for test writing and implementation, Red Gate anti-patterns to reject, and completion criteria. Fills the phase coverage gap: spec-crystallization and decomposition primers existed for Phase 1/1b; implementation was unprimed.
- **`primers/3-review-session.md`** — New session primer for VSDD Phase 4 (Adversarial Review). Establishes adversarial posture before loading any domain prompt. Names sycophancy failure modes at the session level (not domain level), covers DESIGN.md prerequisite check, domain selection and sequencing, session isolation, and post-review classification requirements.
- **`suite-development/suite-development.md`** — New session primer for IAR suite development work. Governs adding/modifying domains, dimensions, and primers. Specifies the complete domain file structure, primer structure, pre-change checklists, gap registry discipline, SUITE-REVIEW.md and CHANGELOG.md requirements, and a lang supplement coverage table.
- **`SECURITY-REVIEW.md` — `## Threat Model` section** — Required prerequisite section added before Standard Evaluation Dimensions. Before applying the checklist, the reviewer must name threat actors, crown jewel, and entry points. Output is logged as a preamble record in the review log, not a classified finding. Addresses G-06.
- **`supplements/javascript-typescript.md` — Red Team section** — JS/TS-specific attack vectors: prototype pollution exploitation (payload format, mitigation patterns), DOM-based XSS sinks enumeration, JWT algorithm confusion (alg:none, library version verification), npm supply chain and dependency confusion, localStorage/sessionStorage as persistence injection surface.
- **`supplements/javascript-typescript.md` — Performance Engineer section** — Bundle size analysis tooling (webpack-bundle-analyzer, source-map-explorer), V8 profiling via Chrome DevTools, Web Vitals as performance contract (LCP/INP/CLS targets), event delegation efficiency.
- **`supplements/rust.md` — Red Team section** — Rust-specific attack vectors: integer overflow in release builds (wrapping arithmetic, `u32::MAX` boundaries), panic as DoS vector (`.unwrap()` on user-influenced paths), path traversal via `Path::join`, `unsafe` block exploitation, crates.io supply chain.
- **`supplements/rust.md` — Performance Engineer section** — Criterion benchmarking discipline, flamegraph profiling (`cargo flamegraph`), debug vs. release build performance differential, allocation patterns in hot paths, async blocking operations in executor threads.
- **`README.md` — Expanded primers table** — Added Implementation, Adversarial Review, and Suite Development primers with when-to-use descriptions. Primer table now covers all five session types.
- **`README.md` — Running IAR preamble** — Three new constraints before the refinement loop: human-in-the-loop requirement (IAR's adversarial value collapses without human classification decisions), DESIGN.md prerequisite (no domain reviews without a spec), domain activation guidance (pointer to DOMAIN-INDEX.md).

### Changed
- **Domain folder restructure** — All domain files moved from root to `domains/role/` (role domains) and `domains/meta/` (meta domains). All internal links updated to relative paths (`../../README.md`, `../../supplements/`).
- **Domain file renames** — `SOFTWARE-ENGINEERING-REVIEW.md` → `SOFTWARE-ENGINEER-REVIEW.md`, `QUALITY-ENGINEERING-REVIEW.md` → `QUALITY-ENGINEER-REVIEW.md`, `PLATFORM-ENGINEERING-REVIEW.md` → `PLATFORM-ENGINEER-REVIEW.md`, `DATA-ENGINEERING-REVIEW.md` → `DATA-ENGINEER-REVIEW.md`. H1 titles updated to match. All cross-references updated.
- **`PERFORMANCE-REVIEW.md` → `PERFORMANCE-ENGINEER-REVIEW.md`**, **`DOCUMENTATION-REVIEW.md` → `TECHNICAL-WRITER-REVIEW.md`** — Renamed to role-based titles. H1 titles and Reviewer role lines updated.
- **README domain tables** — Restructured into three categories: Core role domains (8, always active), Extended role domains (6, activation-conditional), Meta domains (2). Added Role and Job title columns. All file paths updated to new folder locations.
- **Reviewer role lines** — Added `**Reviewer role: [Title]** ([Job title variants])` to all domain files that were missing it. Duplicate lines removed from 10 files where the line was accidentally inserted twice.
- **`PORTFOLIO-ASSESSMENT-REVIEW.md`** — Added explicit lang supplement opt-out line with rationale.
- **`suite-development/suite-development.md`** — Governing standard updated: lang supplement reference is required OR must have explicit opt-out with rationale; element 6 now distinguishes required structural sections (prerequisite records) from optional extended sections (conditional sub-dimensions).

### Fixed
- **`RED-TEAM-REVIEW.md`** — Coordination link text corrected: `[QE-REVIEW.md]` → `[QUALITY-ENGINEER-REVIEW.md]`.
- **`suite-development/GAP-ANALYSIS-LOG.md`** — Duplicate rows for G-02, G-03, G-12, G-34 removed; original rows updated in-place to Addressed status. G-80–G-83 moved from appended duplicates to proper numeric position in registry table. G-06, G-19, G-27 updated to Addressed. G-84 added (Technical Writer lang supplement gap).

### Removed
- **`OBSERVABILITY-REVIEW.md`** — Content absorbed into `PLATFORM-ENGINEER-REVIEW.md` dims 27–33 (error surfacing, error classification, diagnostic completeness, health surfaces, sensitive data exclusion, silent success confirmation, runbook coverage).
- **`API-CONTRACT-REVIEW.md`** — Content absorbed into `SOLUTION-ARCHITECT-REVIEW.md` Extended: External Interface Contracts (dims 13–22).

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
- **QUALITY-ENGINEER-REVIEW.md** — Removed dim 14 (manual testing checklists → VDD-IAR Alignment). Added domain boundary statement: QE owns the test system; SE owns the bugs. When QE finds a logic error with no test, flag the missing test here; SE flags the bug.
- **SOFTWARE-ENGINEER-REVIEW.md** — Added domain boundary statement: SE owns the implementation; QE owns the test system. SE flags bugs; QE flags missing tests. Do not duplicate by evaluating test architecture in SE.
- **PLATFORM-ENGINEER-REVIEW.md** — Replaced generic sycophancy check with a posture note acknowledging that most PE dimensions are compliance checks, not adversarial judgment calls. Sycophancy risk is specifically scoped to inapplicability decisions and threshold acceptance. This is more honest about what PE actually does.

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
- **`supplements/` subfolder** — Language and interface type supplements. Domain files reference these during review; reviewers apply the relevant supplement's section alongside the standard dimensions for that domain.
  - `supplements/rust.md` — Rust-specific dimensions for QE (doc tests, clippy, integration tests against binary), Security (`.unwrap()` discipline, `cargo audit`, unsafe rationale), SE (error propagation, error type hierarchy, clippy as idiom proxy), PE (`cargo audit`, `cargo clippy --deny warnings`, `cargo fmt --check`, `Cargo.lock` for binaries, toolchain pinning), DE (`serde` boundary validation, `#[serde(default)]` for schema evolution), SA (CLI parsing separation, command enum dispatch, `lib.rs`/`main.rs` split)
  - `supplements/javascript-typescript.md` — JS/TS-specific dimensions for QE (`npm ci`, axe scanning, browser tests, type coverage), Security (rendering safety, URL injection, `JSON.parse` runtime validation, CSP, `npm audit`), SE (`as` casts require runtime validation, `any` types, non-null assertions, unhandled promise rejections), PE (`npm ci`, `package-lock.json`, `npm audit`, Node pinning, `tsc --noEmit`), DE (runtime schema validation, `JSON.parse` error handling, normalization, date handling)
  - `supplements/cli.md` — CLI interface type supplement. Replaces browser-centric UX dimensions with 11 CLI UX dimensions (command discoverability, stdout/stderr discipline, exit codes, empty state messages, destructive confirmation, machine-readable output, verbose/quiet modes, error message quality, interruption handling). Adds CLI-specific QE dimensions (integration tests invoke binary, full stdout/stderr/exit code assertions) and SE dimensions (output formatting as a code concern, structured result types before formatting).
  - `supplements/browser-app.md` — Browser interface type supplement with QE (axe scanning, browser compat, responsive testing, keyboard navigation), Security (rendering safety, URL injection, CSP, storage validation, SRI), and UX (accessibility, responsive design, browser compatibility, reduced motion, native dialog quality) dimensions.
- **Sycophancy check** — Added to all 8 domain prompts. Explicitly names AI self-validation as a failure mode: if the reviewing agent agrees with every decision without challenge, that agreement itself is a finding.
- **Solution Owner dim 9 — Complexity budget for one** — New dimension evaluating whether architectural complexity is proportionate to the maintenance team size. An AI agent defaults to team-scale practices regardless of the project's actual maintenance model. Distinct from over-engineering (which flags complexity beyond spec); this flags complexity that is proportionate to spec but disproportionate to the team.
- **Solution Owner — `approved deviation` classification** — New finding classification for deviations from DESIGN.md that were explicitly approved by the stakeholder prior to implementation. Requires documentation of the approval and rationale.
- **Solution Architect dim 11 — Session continuity** — New dimension: are architectural decisions and rationale documented in a form a new AI session can act on without rediscovering them? Decisions that live only in conversation history are invisible to future sessions.
- **Software Engineering dim 11 — Future-self maintainability** — New dimension: will you be able to understand and modify this code in six months without the original AI session? Are key decisions derivable from the code and its comments?
- **suite-development/GAP-ANALYSIS-LOG.md Run 2** — 2026-04-25 21:30Z. Context: AI-accelerated consulting team. Identified 15 new gaps (G-18–G-32) including: Requirements and Business Analysis domain, Documentation Fidelity domain, AI assumption surfacing, hallucination detection, context drift checking, dependency/API existence validation, test gaming detection, AI-generated code anti-patterns, Change Management, Knowledge Transfer, Client/Stakeholder Alignment, integration architecture.
- **suite-development/GAP-ANALYSIS-LOG.md Run 3** — 2026-04-25 22:00Z. Context: personal developer using AI-accelerated tools, portfolio-to-side-business trajectory. Identified 6 new gaps (G-33–G-38) including: sycophancy detection (G-33, addressed), future-maintainability-for-one assessment (G-35, addressed), session continuity across AI conversations (G-37, addressed), complexity trap from AI over-engineering (G-38, addressed).

### Changed
- **QUALITY-ENGINEER-REVIEW.md** — Removed browser-specific dimensions 11–13 (accessibility, browser compatibility, responsive design) from standard dimensions; these are now in `supplements/browser-app.md`. Generalized dim 14 (security surface) to remove npm-specific language. Renumbered to 13 dimensions. Added language and interface supplement instruction.
- **SECURITY-REVIEW.md** — Removed web-specific dimensions 1 (rendering safety), 2 (URL injection), and 5 (CSP) from standard dimensions; these are now in `supplements/browser-app.md` and `supplements/javascript-typescript.md`. Generalized remaining dimensions to be language-agnostic. Added dim 4 (secret handling) and dim 6 (authentication/authorization) as generic security dimensions. Renumbered to 6 dimensions. Added language and interface supplement instruction.
- **UX-REVIEW.md** — Added interface-type note: standard dimensions assume a browser-rendered interface; CLI projects should consult `supplements/cli.md`; browser apps should also consult `supplements/browser-app.md`.
- **PLATFORM-ENGINEER-REVIEW.md** — Generalized npm-specific language in dims 1, 3, 4, and 11 to be ecosystem-agnostic with ecosystem-appropriate examples. Added language and interface supplement instruction.
- **SOLUTION-ARCHITECT-REVIEW.md** — Added language and interface supplement instruction.
- **SOLUTION-OWNER-REVIEW.md** — Added language and interface supplement instruction (SO review is primarily spec-driven; supplement used to verify technology choices against the spec).
- **SOFTWARE-ENGINEER-REVIEW.md** — Added language and interface supplement instruction.
- **DATA-ENGINEER-REVIEW.md** — Added language and interface supplement instruction.
- **suite-development/GAP-ANALYSIS-LOG.md** — Fixed blank line between G-17 and G-18 rows that broke markdown table rendering. Updated gap registry statuses: G-33, G-35, G-37, G-38 marked Addressed.

---

## 2026-04-26 00:15Z — `db45cd2`

### Added
- **suite-development/GAP-ANALYSIS-LOG.md** — New living document for gap analysis runs against the AIR suite itself. Includes re-run trigger conditions, instructions, and a gap registry table. Initial run (Run 1, 2026-04-25 20:00Z) evaluated against mission-critical and speculative project contexts. Identified 17 gaps (G-01–G-17) across 5 missing domains and 12 dimension-level gaps. Per-context severity (Mission-Critical / Speculative) recorded for each gap.

---

## 2026-04-25 23:56Z — `59ee04e`

### Added
- **PE dim 10 — Pre-commit hooks** — Platform Engineering now owns pre-commit hooks as a DevSecOps control. Hooks cover: secret and credential detection (API keys, tokens, private keys, connection strings); PII detection (email addresses, phone numbers, government IDs); committer identity and local machine leakage (absolute paths with usernames, hostnames, local environment details in configs or build output); large or binary files. Includes evaluation of `--no-verify` bypass risk.

### Changed
- **SECURITY-REVIEW.md** — Added coordination note: Security flags sensitive data patterns it identifies to Platform Engineering for incorporation into pre-commit hook detection rules.

---

## 2026-04-25 23:51Z — `0bef3f6`

### Changed
- **PLATFORM-ENGINEER-REVIEW.md** — Massively expanded from CI/CD-only to full delivery platform ownership across four areas:
  - **CI/CD** (dims 1–9): pipeline completeness, gate enforcement, dependency installation, environment pinning, cache correctness, coverage thresholds, action/dependency pinning, artifact hygiene, left-shift opportunities
  - **DevSecOps** (dims 10–15): security scanning, secret management, supply chain integrity, least privilege, compliance gates
  - **Infrastructure** (dims 16–21): Infrastructure as Code, cloud/on-premise resource hygiene, containerization, container security, environment parity, disaster recovery
  - **Observability** (dims 22–26): logging, metrics, alerting, distributed tracing, dashboards
  - Inapplicable sections may be skipped with rationale. A static single-user tool has no cloud infrastructure to evaluate.

---

## 2026-04-25 23:40Z — `2b6446a`

### Added
- **SOFTWARE-ENGINEER-REVIEW.md** — New domain. Evaluates implementation quality at the code level: correctness, error handling, naming, function design, duplication, complexity, type safety, defensive coding, comments and self-documentation, consistency. Distinct from Solution Architect (which evaluates structure and boundaries) and Quality Engineering (which evaluates the test system). 10 standard dimensions.
- **DATA-ENGINEER-REVIEW.md** — New domain. Evaluates the data layer: data model correctness, validation and normalization, schema evolution, data integrity invariants, storage fitness, access patterns, serialization, data consistency, sensitive data handling, test coverage of data paths. Marked optional for projects without a meaningful data layer.

### Changed
- **QA-REVIEW.md → QUALITY-ENGINEER-REVIEW.md** — Renamed via `git mv`. Scope broadened from bug-finding to test architecture and quality system: added test falsifiability (dim 2, "a test that cannot fail on a defective implementation has no value"), coverage meaningfulness (dim 4), test architecture and independence (dim 5), and quality gates (dim 16).
- **All domain prompts** — Added DESIGN.md as required first read for all domain reviews. All domains now treat DESIGN.md as authoritative context for the project's scope, constraints, and feature set.
- **All cross-domain coordination links** — Updated from `QA-REVIEW.md` to `QUALITY-ENGINEER-REVIEW.md`.
- **README.md** — Added Software Engineering and Data Engineering to domain table. Updated domain count and descriptions. Added note that not all domains are required for all projects. Updated sequencing (run DE before SA when data model changes are significant). Updated merging gate and log structure to reflect 8 domains.

---

## 2026-04-25 23:03Z — `6ea9b30`

### Added
Initial AIR suite. Six review domains extracted from the bookmark-manager project, generalized into a reusable template.

- **README.md** — Suite index: domain table, running instructions (full run, scoped run, sequencing), candidate domains, review log structure, merging gate.
- **QA-REVIEW.md** — Quality assurance: acceptance criteria, test coverage, validation gaps, logic errors, dead code, unused dependencies, dependency versions, accessibility, browser compatibility, responsive design, security surface.
- **UX-REVIEW.md** — User experience: empty states, error messages, focus and keyboard behavior, visual consistency, affordances, feedback patterns, accessibility, responsive design, browser compatibility, long content, reduced motion, native dialog quality.
- **SECURITY-REVIEW.md** — Security: rendering safety, URL injection, storage data validation, dependency CVEs, CSP, information exposure, input handling.
- **PLATFORM-ENGINEER-REVIEW.md** — CI/CD pipeline and gate enforcement.
- **SOLUTION-ARCHITECT-REVIEW.md** — Architecture: separation of concerns, coupling, data model integrity, interface contracts, state management, immutability, extensibility, technology fitness, complexity budget, decision documentation.
- **SOLUTION-OWNER-REVIEW.md** — Scope and delivery: spec coverage, scope creep, technology compliance, over-engineering, under-delivery, design fidelity, backlog candidates, prior-review additions. Opens every review with a compliance table (Met/Partial/Missing). DESIGN.md treated as a Scope of Work contract. "Quality does not justify scope."

Review logs are stored outside the prompt files. Logs live at `{project}/vsdd-suite/` inside each reviewed project.
