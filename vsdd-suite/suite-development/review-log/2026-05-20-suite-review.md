# Suite Review — 2026-05-20

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
