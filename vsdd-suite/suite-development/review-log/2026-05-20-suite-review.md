# Suite Review — 2026-05-20

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
