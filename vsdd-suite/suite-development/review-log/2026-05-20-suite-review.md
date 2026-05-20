# Suite Review — 2026-05-20

---

## Review 72 — 2026-05-20 10:15Z

**Scope:** Multi-artifact suite-development pass driven by operator-directed review of (a) the vsdd-suite README's Phase 5 / Phase 6 coverage and (b) suite-development governance documentation currency. Mid-session the operator promoted G-177 (Deferred) to Addressed via the explicit G-130 preemption mechanism, broadening the scope to retire the `PHASE-5-LOG.md` + `PHASE-6-CONVERGENCE.md` per-project artifact prescription across the suite + the bookmark-cli reference example. Artifacts read this round: `suite-development/suite-development.md` (governing standard, as session primer); `vsdd-suite/README.md` (Quickstart, per-layer flow diagram, Worked example, project-tree example, Merging gate, Running IAR sections); `primers/5-formal-hardening.md`; `primers/6-convergence.md`; `domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md` dim 13 + dim 14; `suite-development/FINDINGS-INDEX.md`; `suite-development/SUITE-DEVELOPMENT-REVIEW.md`; `suite-development/README.md`; `bookmark-cli/vsdd-suite/PHASE-5-LOG.md` + `DESIGN.md` + `src/lib.rs` + the SA and QE review log files + project CHANGELOG.

**Lens:** Multi-lens cross-artifact consistency + currency audit — SO (spec / methodology scope), SA (architecture / convention coherence applied to the suite itself), TW (documentation currency and drift), VDD-IAR Alignment (process compliance for the suite as artifact). Three coordinated artifact-state checks: Phase 5/6 integration coverage; legacy IAR-suite verbiage cleanup; G-177 operator promotion (per-domain log pattern roll-out).

**Session note:** In-session with the suite's authorial context — the same operator who promoted G-177 mid-session, directed the README review, and made the "stacked PRs are wrong; one PR at a time" workflow correction. Sycophancy compensation: every finding anchored to a specific file path and line range (grep-verified); the operator's directive that bookmark-cli is the reference example (so it migrates rather than gets a forward-only carve-out) was applied to remove the forward-only paragraphs I had initially written. Two course corrections during the session were applied immediately (PHASE-5-LOG retirement + reference-example migration framing) rather than deferred. Findings derived from artifact-state analysis (grep over PHASE-5-LOG / PHASE-6-CONVERGENCE / IAR-Suite / gap-analysis-run references), the user's specific directive prompts, and the governing standard's currency check.

**Source:** mixed — `director-raised` for the session-opening Phase 5/6 README review prompt, the G-177 operator-promotion message, the bookmark-cli reference-example migration directive, and the workflow directives ("log suite-development sessions automatically" + "one PR at a time, no stacked PRs"); `domain-raised` for the legacy-verbiage findings (TW lens) and the cross-artifact consistency findings (SA lens) the operator-directed review surfaced.

### Resolved

**Finding 1 — `PHASE-5-LOG.md` + `PHASE-6-CONVERGENCE.md` per-project files retired (G-177 operator-promoted from Deferred to Addressed)**

G-177 (Deferred since Review 67 with trigger "second project enters Phase 5 OR operator preemption") was operator-promoted to Addressed mid-session. The operator's directive: "PHASE-5-LOG.md + PHASE-6-CONVERGENCE.md should not exist; they violate conventions and are an anti-pattern." Resolution candidate (a) from G-177's row applied across the suite: retire the per-project files; Phase 5 findings file under per-domain review logs with `**Phase 5 surface:**` preamble tag; Phase 6 convergence record IS the final VDD-IAR Alignment review round.

**Resolution scope:**

| Artifact | Change |
|---|---|
| `primers/5-formal-hardening.md` § Phase 5 log format | Rewrote section: per-domain log pattern with per-surface→domain mapping (A / A.0 / D → SA; B / C → QE) + `**Phase 5 surface:**` preamble tag format. Surface C JS/TS distinction reworded to cite per-domain logs not PHASE-5-LOG.md. |
| `primers/5-formal-hardening.md` § Manual mode + § Completion criteria #2 | Updated to cite per-domain rounds with preamble tag instead of PHASE-5-LOG.md. |
| `primers/6-convergence.md` § Phase 6 convergence record format | Substantial rewrite: the convergence record IS the final VDD-IAR Alignment review round titled "Review N — Phase 6 four-dimensional convergence (project-terminal)" with the four-dimension attestations + cross-dimension consistency check + signed closing per the round entry format. |
| `primers/6-convergence.md` § Crosslink mode + § Manual mode + § Completion criteria + § Anonymization-aware attestation + § Layer reference + Dimension 2 verification step / disposition record + Dimension 4 signal | All references to `vsdd-suite/PHASE-6-CONVERGENCE.md` and `vsdd-suite/PHASE-5-LOG.md` rewritten to cite the per-domain log rounds (with `**Phase 5 surface:**` preamble) and the final VDD-IAR Alignment round respectively. |
| `domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md` dim 13 | Surface-activation check updated: evaluate per-domain rounds with the `**Phase 5 surface:**` preamble tag instead of PHASE-5-LOG.md per-layer entries. |
| `domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md` dim 14 | Phase 6 convergence evaluation updated: evaluate the final VDD-IAR Alignment review round (the round titled "Review N — Phase 6 four-dimensional convergence (project-terminal)") instead of `vsdd-suite/PHASE-6-CONVERGENCE.md`. |
| `vsdd-suite/README.md` Quickstart steps 7 + 8 (both modes) | Phase 5 step describes per-surface session filing in the per-domain log; Phase 6 step describes the final VDD-IAR Alignment round. |
| `vsdd-suite/README.md` Worked example § Phase 5 + § Phase 6 walkthroughs (added in this session) | Authored from scratch using the per-domain log + final round pattern. |
| `bookmark-cli/vsdd-suite/PHASE-5-LOG.md` (reference example) | Deleted via `git rm`. The substantive content (purity-boundary audit + cargo-mutants outputs + per-mutant disposition table) was already present in the per-domain logs (`review-log/2026-05-20-solution-architect.md` Review 1 + `review-log/2026-05-20-quality-engineer.md` Review 2); PHASE-5-LOG.md was an index/coordination file the per-domain rounds duplicated. |
| `bookmark-cli/vsdd-suite/review-log/2026-05-20-solution-architect.md#review-1` | Added `**Phase 5 surface:** A.0 — purity-boundary verification for Layer 1` preamble tag; removed cross-references to `../PHASE-5-LOG.md` from Scope and Coordination lines. |
| `bookmark-cli/vsdd-suite/review-log/2026-05-20-quality-engineer.md#review-2` | Added `**Phase 5 surface:** B — mutation testing for Layer 1 via cargo-mutants` preamble tag; removed cross-references to `../PHASE-5-LOG.md` from Scope, unviable-mutants paragraph, and Coordination lines. |
| `bookmark-cli/vsdd-suite/QUALITY-ENGINEER-REVIEW.md` Reviews table | Row for Review 2 reworded to name the surface preamble explicitly + removed `../PHASE-5-LOG.md` citation. |
| `bookmark-cli/DESIGN.md` § Project intent Phase 5 strategy line + § Verification architecture Phase 5 bullet | Reworded to cite the per-domain logs instead of `PHASE-5-LOG.md`. |
| `bookmark-cli/src/lib.rs:148` doc comment | Updated to cite `vsdd-suite/QUALITY-ENGINEER-REVIEW.md` (Review 2 — Phase 5 Surface B) instead of `vsdd-suite/PHASE-5-LOG.md`. |
| `bookmark-cli/CHANGELOG.md` | New top entry documenting the v0.7.8 migration with the file delete + cross-reference update list + historical-narrative preservation note. |

**Forward-only narrative:** historical CHANGELOG / COMPATIBILITY / review-log entries that reference `PHASE-5-LOG.md` and `PHASE-6-CONVERGENCE.md` are preserved as audit-trail records per G-89. The CHANGELOG.md `## Unreleased — 2026-05-20 04:30Z (Review 68: ...)` entry mentioning `**G-177 (new)**` and the v0.7.0 / v0.7.1 / v0.7.3 COMPATIBILITY.md rows that reference the files reflect the state at the time of writing; the current state is described in this Review 72's CHANGELOG entry. PHASE-6-CONVERGENCE.md never existed on any project; no Phase 6 file deletion was needed.

**Classification:** Resolved.

**Finding 2 — README missing Phase 5 + Phase 6 operational integration (G-181)**

`vsdd-suite/README.md` had Phase 5 + Phase 6 named in `## Suite scope`, the `## VSDD pipeline context` table, and the `## Session primers` table — but no operational integration. Specifically:

- Both Quickstart sections (crosslink-primary and manual) stopped at Phase 4 / "Loop until MVR" with no step 7 (Phase 5) or step 8 (Phase 6).
- The Per-layer flow ASCII diagram (G-136 closure) ended at "Merge layer" — no Phase 5 box (per-layer, between Phase 3 MVR and merge) and no Phase 6 box (project-terminal).
- The Worked example walkthrough had `### Phase 1a+1b` through `### Phase 4 — Feedback Integration` + `### Loop until MVR` but no `### Phase 5 — Formal Hardening` or `### Phase 6 — Four-Dimensional Convergence` sections.
- The project-tree example listed per-domain index files but did not name the Phase 5 / Phase 6 artifacts the methodology produces (which, post-G-177, are per-domain rounds with `**Phase 5 surface:**` preamble + the final VDD-IAR Alignment round; no separate per-project files).

A new operator reading the README to learn the methodology would see Phase 5/6 named but have no operational guidance for executing them.

**Resolution:** added Phase 5 + Phase 6 steps to both Quickstart sections; extended Per-layer flow diagram with Phase 5 (conditional, between layer-gate close and merge) and Phase 6 (project-terminal after every layer's Phase 5); added `### Phase 5 — Formal Hardening` + `### Phase 6 — Four-Dimensional Convergence` walkthrough sections to the Worked example with `[crosslink]` + `[manual]` blocks (the four-surfaces table for Phase 5; the four-dimensions table for Phase 6). All new content reflects the post-G-177 per-domain log + VDD-IAR Alignment final round pattern. The project-tree example was intentionally not updated to add new per-project files — per G-177, those files are retired.

**Classification:** Resolved.

**Finding 3 — Legacy "IAR Suite" / "gap analysis" verbiage in suite-development governance files (G-182)**

Three suite-development files retained legacy IAR-suite / gap-analysis verbiage inconsistent with current VSDD Suite / Findings conventions:

- `suite-development/FINDINGS-INDEX.md:1` — H1 "# IAR Suite Gap Analysis Log" (file was renamed from `GAP-ANALYSIS-LOG.md` to `FINDINGS-INDEX.md` in v0.4.0 per G-149, but the H1 was not updated).
- `suite-development/FINDINGS-INDEX.md:3` — opening paragraph "This log tracks gap analysis runs against the IAR suite itself".
- `suite-development/FINDINGS-INDEX.md:11` — section header "## How to run a gap analysis" — the "gap analysis run" framing was retired by `suite-development.md:407`.
- `suite-development/SUITE-DEVELOPMENT-REVIEW.md:1` — H1 "# IAR Suite Review".
- `suite-development/SUITE-DEVELOPMENT-REVIEW.md:3` — "The IAR suite is itself a software artifact" + "gap analysis log".
- `suite-development/README.md:3` — "running gap analysis".
- `suite-development/README.md:60` — "Project IAR sessions sometimes produce findings".

A new contributor opening these files first would form a mental model out of date with the rest of the suite (where "VSDD Suite" is the current name and "suite review" is the unified session type per v0.4.0's mode-unification).

**Resolution:**

- FINDINGS-INDEX.md H1 → "# VSDD Suite Findings Index"; opening paragraph rewritten to "findings registry against the VSDD Suite itself"; § header "How to run a gap analysis" → "Adding and updating findings"; body rewritten to point at `suite-development.md` § Running gap analysis and § Suite review and review-log discipline as the canonical workflow source (single source of truth).
- SUITE-DEVELOPMENT-REVIEW.md H1 → "# VSDD Suite Review Index"; opening paragraph reworded to "The VSDD Suite is itself a software artifact" + "expanded beyond its original VSDD Phase 3 (IAR) scope to own every VSDD phase 1a+1b through 6".
- suite-development/README.md line 3 reworded to "registering and walking findings, logging suite reviews"; line 60 reworded to "Project-level review sessions sometimes produce findings whose substance generalizes."

Per G-89 narrative-preservation policy: "gap" remains valid as a concept-level term (the registry IS the gap registry; G-IDs identify gaps); "gap analysis run" specifically — the retired session-type framing — is replaced by "suite review" with the `Lens` field distinguishing modes (defect-search / registry-walk / role-based). Historical narrative in older review-log entries that uses "gap analysis run" prose remains as committed records.

**Classification:** Resolved.

### Coordination

This Review 72 entry registers and resolves three findings in-session (G-177 promoted from Deferred + G-181 + G-182). Cross-domain consequences:

- **G-177 closure ripples** to every project that may adopt Phase 5 or Phase 6 in the future (the per-domain log + VDD-IAR Alignment final round pattern is the active prescription). bookmark-cli (the reference example) is migrated in this session; no other project has reached Phase 5 yet, so no other project migrations are needed.
- **G-181 closure** depends on G-177's resolution (the README's Phase 5 + Phase 6 walkthroughs cite the post-G-177 per-domain pattern; if G-177 had been resolved with candidate (b) instead, the README content would have differed).
- **G-182 closure** is independent of G-177 / G-181 but ships in the same Review 72 because it surfaces from the same TW currency-audit lens.

**Operator workflow directives captured this session** (process feedback applicable to future suite-development sessions, saved as feedback memory at session close):

1. **Suite-development sessions should be logged proactively.** When the operator is doing suite-development work, the agent should be logging suite-review entries and registering findings as the session progresses — not waiting to bundle work at session end. This Review 72 entry started mid-session in response to the directive.
2. **No stacked PR pattern.** Reviews 70 + 71 were stacked PRs (#27 + #28) because they were authored as separate logical sessions that touched the same governance-file rows. Going forward: one PR at a time. This Review 72 ships as a single PR even though it folds in three findings (G-177 + G-181 + G-182) and a reference-example migration.

**Coordination with `bookmark-cli`:** the reference example's migration (PHASE-5-LOG.md deletion + per-domain round preamble tags + DESIGN.md / src/lib.rs / per-domain index updates) is part of this Review 72's scope rather than a separate bookmark-cli session because the migration IS the operational consequence of G-177's resolution at the suite scope. The bookmark-cli CHANGELOG entry cross-references this Review 72.

---

## Review 71 — 2026-05-20 09:15Z

**Scope:** Multi-artifact transition-progress assessment of the IAR-to-VSDD library expansion. Artifacts re-read in this session: `suite-development/suite-development.md` (governing standard); `primers/3-review-session.md` (Phase 3 adversarial review primer); `README.md` (full text, with attention to § Domains, § Quickstart, § Worked example, project-tree example at ~line 905, § Merging gate at ~line 951); `domains/DOMAIN-INDEX.md` (core/extended classification, intent calibration); `COMPATIBILITY.md` (full version history v0.1.0 → v0.7.6); `templates/README.md` (customization checklist); `suite-development/FINDINGS-INDEX.md` (full registry walk, 178 rows). Trigger: operator request for a transition-progress analysis across SO / SA / TW / UX / QE lenses.

**Lens:** Multi-lens transition-progress audit — SO (spec scope coverage), SA (architecture / classification coherence), TW (documentation drift / staleness), UX (developer-experience entry path), QE (suite-effectiveness instrumentation), VDD-IAR (process-compliance applied to the suite as artifact). Five lenses applied serially against the same artifact set to produce a comprehensive transition-completion picture.

**Session note:** In-session with the suite's authorial context (the same session that authored Review 70). Sycophancy compensation: each lens-finding was anchored to a specific file path and line range (grep-verified before recording); the analysis report disclosed both addressed and unaddressed gaps and named the open gaps that pre-date this session by months without re-litigating them as new findings. Findings derived from artifact-state analysis (grep over PE/DE/core-count refs, grep for "Merging gate" / "IAR" usage, file-by-file enumeration of customization checklists) rather than narrative judgment.

**Source:** domain-raised — multi-lens audit (SO / SA / TW / UX / QE) applied to the suite as artifact.

### Resolved

**Finding 1 — README § Merging gate stale relative to suite-development.md § Layer-gate close criteria (Dim 7 — TW / cross-artifact consistency) (G-179)**

`README.md` § Merging gate (prior lines 951–962) enumerated **6 layer-gate criteria**: (1) all active IAR domains have completed a run; (2) refinement loop ran to MVR; (3) every finding terminal; (4) accepted risks documented; (5) VDD-IAR Alignment run; (6) results logged with round numbers. `suite-development/suite-development.md` § Layer-gate close criteria has **7 baseline criteria** (the same 6 plus criterion 7: PROCESS.md retrospective with developer-voice prose as a hard gate, landed 2026-05-18 per G-156). The README's 6-criterion version was older and missing G-156's hard gate; the README also lacked the G-131/G-151 trigger-discipline framing the canonical version carries. A reader landing on the README's Merging gate first (the natural reading path for new adopters) would get a 6-criterion mental model that the canonical source has since superseded.

**Resolution:** replace the README's 6-criterion enumeration with a one-line pointer to the canonical 7-criteria set in `suite-development/suite-development.md` § Layer-gate close criteria. The replacement names criterion 7 (G-156 PROCESS.md retrospective) and the G-131/G-151 trigger discipline explicitly so a reader skimming the README's pointer understands what the canonical set adds. A two-sentence follow-up mentions the project-level `CLOSURE-PROTOCOL.md` precedent (ITC) — the canonical set is the baseline, and projects may add criteria but not weaken. Net change: −12 lines / +3 lines in `README.md`; criterion content lives in one place (suite-development.md) instead of two.

**Why a pointer rather than re-stating all 7:** the criterion set has evolved (6 → 7 via G-156) and will evolve again. Two sources of truth invite drift; one source plus a pointer eliminates the staleness vector. The README's `## Per-layer flow (within a project)` ASCII diagram (G-136) already references the canonical criteria from the diagram itself; this fix completes the single-source-of-truth pattern.

**Classification:** Resolved.

**Finding 2 — templates/README.md Customization checklist does not name DESIGN.md § Project intent declaration (Dim 1 — TW / spec completeness) (G-180)**

`templates/README.md` § Customization checklist enumerates 6 per-domain field substitutions (`{{ROLE_TITLE}}`, `{{ROLE_VARIANTS}}`, `{{PURPOSE}}`, etc.) and a closing paragraph each for `DESIGN.md` and the project `README.md`. The `DESIGN.md` paragraph names the primer to load (`primers/1ab-spec-crystallization.md`) but does not call out the **`§ Project intent` declaration** — the intent line is what gates the active-domain set, the stop-signal sensitivity, and (at capstone+ intent) the Phase 5 / Phase 6 strategy declarations. A first-time scaffolder following the checklist literally would customize the per-domain index files first, then write `DESIGN.md` from the skeleton, possibly without realizing the active-domain set the scaffold script picked should match the intent declared in `DESIGN.md`. The discoverability path is implicit (in the DESIGN-template.md skeleton itself) but the customization checklist is the first artifact the scaffolder reads — it should name the intent declaration explicitly.

**Resolution:** expand the `For DESIGN.md` paragraph in `templates/README.md` § Customization checklist into a 2-step ordered list: (1) work the driving questions in the primer (unchanged); (2) declare `§ Project intent` first, with a one-sentence rationale naming what the intent gates (active-domain set, stop-signal sensitivity, Phase 5/6 strategy declarations at capstone+) and a warning that the over-investment variant is hard to catch in-project. The fix lands in 4 lines of new prose with the cross-reference to `domains/DOMAIN-INDEX.md` § Intent calibration where the gating mechanism is documented.

**Classification:** Resolved.

### Dismissed

**Finding 3 — "IAR" terminology preserved in README (40 occurrences) and suite-development.md (19 occurrences) (Dim 6 — SA / naming consistency)**

The multi-lens audit surfaced that "IAR" still appears with high density across the user-facing surface — 40 occurrences in `README.md`, 19 in `suite-development/suite-development.md`. A cold reader landing on the README without context might read "IAR" as the suite's name rather than the Phase-3 component name. The transition-progress analysis flagged this as a potential drift signal.

**Classification:** Dismissed — intentional per the IAR-name-preservation policy stated explicitly in `suite-development/suite-development.md:11`: "the directory was renamed to `vsdd-suite/` in Review 38 (G-88 closure) to match the expanded scope; 'IAR' remains the name for the Phase 3 portion specifically and is preserved in historical project review logs that pre-date the rename per the forward-only constraint." The 40+19 occurrences are almost all contextually correct (referring to Phase 3 component, the VDD-IAR Alignment meta domain, legacy project paths, or forward-only narrative records). Mass-renaming "IAR" → "Phase 3" or similar would conflict with the explicit policy and would also break legacy project review log cross-references. The name-preservation is doing what the policy says it does.

**A one-sentence inline gloss in the README lead paragraph** ("IAR = Iterative Adversarial Refinement, the Phase 3 component of VSDD") was considered as a less-invasive alternative but rejected as redundant — the README's first sentence already names "Phase 3 (Iterative Adversarial Refinement — IAR)" and the Suite scope section reinforces it.

### Coordination

This Review 71 entry catalogues findings derived from a multi-lens transition-progress audit. The audit re-confirmed the status of **15 long-Open or Deferred gaps** without re-litigating them as new findings — the registry-walk classification universe explicitly authorizes this carry-over reading:

- **Open speculative-project / consulting-scope gaps** (G-01 Compliance and Legal; G-04 Operational Readiness; G-05 Delivery Governance; G-11 SO budget tracking; G-13 PE DR with RTO/RPO; G-14 learning goals; G-15 kill criteria; G-16 intentional tech debt; G-17 SA pivot readiness; G-18 Requirements/BA; G-26 Change Management; G-28 Client/Stakeholder Alignment; G-29 Discovery research quality; G-31 Engagement liability) — these are open by deliberate scope; the suite is a portfolio/apprenticeship tool, not a consulting or production-ops platform. Status unchanged. Reactivation trigger: if the suite's scope expands to consulting or speculative R&D contexts, the bundle becomes eligible.
- **G-57** (no effectiveness test for domain prompts) — long-Open since 2026-04-27; the only foundational QE-lens gap. The audit flagged it as the most-tractable next arc; status unchanged this session but elevated visibility for future selection.
- **Deferred (substantive)** — G-99 (warm-finding-closure Red Gate carve-out); G-135 (AI Engineering / cost-engineering meta-domain); G-159 (knowledge-page versioning); G-168, G-169 (suite-side gaps from Review 63); G-170, G-171, G-172 (Phase 6 refinement gaps from Review 65); G-177 (PHASE-5-LOG.md duplication from Review 67). All have named triggers + auto-Backlog dates per G-130; the audit confirmed their trigger conditions remain unfired and the auto-Backlog dates are still future. Status unchanged.

The audit also confirmed **Review 70 resolved G-178** (core-domain count inconsistency) — that finding's narrative is in Review 70's entry below, not duplicated here.

**Coordination:** **G-179** and **G-180** registered as new gaps in [`../FINDINGS-INDEX.md`](../FINDINGS-INDEX.md) and resolved in-session this round. The fixes were intra-artifact (single section in `README.md`; single section in `templates/README.md`) with no cross-domain implications. No project-level review logs are affected by Review 71's edits. The audit-derived inventory of long-Open gaps is informational; no auto-Backlog triggers fired this round.

---

## Review 70 — 2026-05-20 08:30Z

**Scope:** `domains/DOMAIN-INDEX.md` (core/extended classification tables and intent calibration table); `README.md` (Domains section core/extended tables and project-tree example); `templates/scaffold-project.sh` (header comments and default-domain list); `suite-development/FINDINGS-INDEX.md` (gap row addition); `CHANGELOG.md` (release entry); `COMPATIBILITY.md` (version row); `SUITE-DEVELOPMENT-REVIEW.md` (index row). Trigger: operator-raised three-way inconsistency surfaced during a transition-progress review of the IAR→VSDD library expansion ("the analysis identified PE in capstone calibration as 'All 7 core + Performance Engineer' — but PE is already in the core 8 per the DOMAIN-INDEX table; that's mathematically incoherent if PE is in the core 8").

**Lens:** Cross-artifact consistency — applied specifically to the core-domain count and PE/DE classification across all suite artifacts where the count is named.

**Session note:** In-session with the suite's authorial context (operator-driven structural change session, not a cold review). Sycophancy compensation: the reclassification direction was selected by the operator via an explicit AskUserQuestion with three options (demote, promote, or third-tier), each with a preview showing the resulting taxonomy; the agent's framing of the recommendation was disclosed and the operator chose Option A independently. Findings derived from artifact-state analysis (grep over every PE/DE/core-count reference in README, DOMAIN-INDEX, scaffold script, and templates) rather than narrative judgment.

**Source:** domain-raised — Solution Architect lens on the suite (classification scheme coherence is an SA dim 4 concern: data model integrity applied to the domain taxonomy itself).

### Resolved

**Finding 1 — Core-domain count inconsistency between DOMAIN-INDEX.md and README.md (Dim 4 — applied to suite taxonomy) (G-178)**

`domains/DOMAIN-INDEX.md` § Core domains opened with "These eight domains apply to all projects regardless of type, deployment context, or scale" and listed eight role domains in the core table (SE, QE, UX, Security, PE, SA, SO, DE). The same file's § Intent calibration table treated the count as seven ("All 7 core" for portfolio; "All 7 core + Performance Engineer" for capstone — incoherent if PE was already inside the 7). The `templates/scaffold-project.sh` script defaulted to seven (six role + VDD-IAR-Alignment meta, excluding PE+DE). `README.md` § Domains and Quickstart consistently said "7 core domains" and the worked example said "(7 core domains, no PE/DE/extended)". Three different mental models existed in parallel:

- DOMAIN-INDEX table: 8 core role
- DOMAIN-INDEX intent calibration: 7 (ambiguous about which)
- README + scaffold + worked example: 7 = 6 role + 1 meta

A new contributor or AI agent loading any one of these as authoritative would produce drift in the other two.

**Resolution:** demote Platform Engineer and Data Engineer from core role to extended-with-strong-presumption (operator selection from a three-option AskUserQuestion: A demote, B promote scaffold to 9, C add a third tier). Edits applied:

1. **`domains/DOMAIN-INDEX.md` § Core domains** — intro rewritten from "These eight domains apply to all projects" to "Six core role domains plus the VDD-IAR Alignment meta domain (seven total) apply to all projects." PE and DE rows removed from the core role table; a paragraph naming the seventh-core-is-VDD-IAR-Alignment meta domain was added. New forward-only-constraint paragraph cites the v0.7.6 cutoff date and the G-178 row for the reclassification's authority.
2. **`domains/DOMAIN-INDEX.md` § Extended domains** — PE and DE rows added at the top of the extended table with named activation criteria (PE: managed pipeline / infrastructure / observability hooks / any operational deployment surface beyond local-toolchain install; DE: persistent data through DB / managed schema / structured-storage integrity / external data systems). A new paragraph above the table establishes the "extended-with-strong-presumption" framing — both domains typically activate beyond local-toolchain CLI scope and are strongly presumed at capstone and production intent.
3. **`domains/DOMAIN-INDEX.md` § Intent calibration** — learning-exercise row reframed: SE+QE+SO+VDD-IAR Alignment as the four fixed cores plus one rotating fourth role drawn from {SA, Security, UX} (PE+DE removed from the rotation pool since they're now extended). Portfolio / capstone / production rows clarified to name PE+DE per their activation criteria; capstone and production now make explicit that PE+DE are typically active at those intents.
4. **`README.md` § Domains** — PE row and DE row moved from the Core role table to the top of the Extended role table. The lead paragraph "Default activation for new projects is the 7 core role domains plus VDD-IAR Alignment" reworded to "the 7 core domains — six core role domains (SE, QE, UX, Security, SA, SO) plus the VDD-IAR Alignment meta domain" — eliminates the "(7 role) + (1 meta) = 7?" arithmetic ambiguity. A new sentence under the core table names VDD-IAR Alignment as the seventh core domain (listed in the meta table). The extended table opens with the "extended-with-strong-presumption" framing for PE+DE.
5. **`README.md` project-tree example (~line 905)** — comment block reorganized: PE and DE moved from "# Core domains (always active)" to "# Extended domains (include only those active on the project; PE + DE are extended-with-strong-presumption per G-178 and typically active beyond local-toolchain CLI scope)".
6. **`templates/scaffold-project.sh`** — header comment block, `DEFAULT_DOMAINS` array comment, and the no-args echo block reworded from "core but conditional" to "extended-with-strong-presumption (G-178)". Script behavior unchanged (already defaulted to 7 since v0.3.0).

**Forward-only constraint (G-89 precedent):** projects whose first IAR run predates v0.7.6 (today, 2026-05-20) retain PE/DE-as-core in their existing review logs, DESIGN.md notes, and per-domain review-log files. The reclassification does not invalidate prior records. New projects scaffolded at v0.7.6+ follow the new classification automatically.

**Why this is non-breaking against COMPATIBILITY.md:** the PE and DE prompt files are unchanged (same dimensions, same sycophancy check, same finding classification schema). The classification (core vs. extended) is a metadata field about the domain, not a content field. Existing review logs that reference PE/DE remain syntactically valid against the suite's governing standard. The only behavioral change is in the scaffold-default activation set — which is already what the scaffold script does in practice.

**Why a third tier was rejected (Sycophancy self-audit):** the agent's initial framing in the analysis recommended Option A and previewed the result; the operator selected Option A. The third-tier option (Option C: "core-but-conditional") was rejected for a substantive reason: it would preserve the "core" label for PE+DE but require a new taxonomic concept to explain the difference between "always-core" and "core-presumed-with-scope-down". The operating reality already maps cleanly to a two-tier taxonomy; the third tier would be defending the prior label rather than the prior practice. (The README and scaffold script were always operating Option A semantics; only the DOMAIN-INDEX header was operating "core" semantics.) Per the "earned by recurrence" doctrine, taxonomic weight is added when a defect class recurs that the existing taxonomy can't catch — not when an existing taxonomy can be reorganized to match practice.

**Classification:** Resolved.

### Coordination

Edits propagated mechanically across all artifacts where the prior counts appeared:

- `domains/DOMAIN-INDEX.md` — primary canonical edit (core + extended tables + intent calibration)
- `README.md` — Domains section + project-tree example
- `templates/scaffold-project.sh` — header + comment block (no behavior change)
- `suite-development/FINDINGS-INDEX.md` — G-178 row added with full resolution narrative
- `CHANGELOG.md` — v0.7.6 entry added (additive non-breaking reclassification per COMPATIBILITY.md § Breaking change definition)
- `COMPATIBILITY.md` — v0.7.6 version row added
- `suite-development/SUITE-DEVELOPMENT-REVIEW.md` — Review 70 row added at top of Suite Reviews table

Coordinate with **G-121** (scaffold-default ratification — Review 42's Solution Owner ratification of the 7-core scaffold default; that ratification was the operating-reality precedent the reclassification now matches). Coordinate with **G-150** (intent calibration — already operating with 7 core + extensions; this reclassification removes the count ambiguity in that table). Coordinate with **G-89** (forward-only narrative-preservation policy — the v0.7.6 cutoff applies the same forward-only mechanism the prior structural changes used).
