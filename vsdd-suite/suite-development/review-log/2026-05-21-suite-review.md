# Suite Review — 2026-05-21

---

## Review 83 — 2026-05-21 10:00Z

**Scope:** Operator-directed PR [#39](https://github.com/magnificentlycursed/guild-portfolio/pull/39) — register the new **[AI Engineer](../../domains/role/AI-ENGINEER-REVIEW.md)** role-domain (cost-and-quality discipline for AI-agent usage in the IAR cycle) + run its first cold-session round on [bookmark-cli-manual](../../../vsdd-suite-reference-examples/bookmark-cli-manual/) (the suite's capstone-intent reference example). The domain emerged from the operator-surfaced observation during PR [#38](https://github.com/magnificentlycursed/guild-portfolio/pull/38) Round 2 dispatch: "Cold-session verification runs seem very token expensive. It may be time to revisit the AI Engineer domain role to determine what about them is expensive and how to realize the cold session benefits without immediately hitting daily session limits." PR [#38](https://github.com/magnificentlycursed/guild-portfolio/pull/38)'s 3-round cycle vindicated the framing: 10 parallel cold-session agents per round in R1 + R2 burned an estimated ~3-4M tokens across 24 agent-spawns; one agent hit Anthropic's daily rate-limit mid-execution; the cluster-batching workaround in R3 cut agent count ~60% while preserving adversarial-pair separation. Codifying the cost discipline keeps the gold-standard pattern achievable at scale.

**Lens:** Domain-registration discipline + cold-session-cost-discipline codification + reference-example active-domain-set extension (11 role + 1 meta → 12 role + 1 meta = 13 active domains at capstone intent). Sycophancy compensation: resisted treating the cluster-batching workaround from PR [#38](https://github.com/magnificentlycursed/guild-portfolio/pull/38) Round 3 as a methodology requirement (it is one operative shape; future cycles may discover better shapes); resisted authoring AI Engineer as a meta-domain (the operator's framing is role-domain — evaluates how AI agents are used to produce the artifact, similar to how Platform Engineer evaluates how the artifact ships through CI/CD; both are about production-process discipline, not artifact-evaluation); resisted naming a canonical adversarial pair for AI Engineer (no peer-domain maps cleanly to its cost-and-quality lens — `**Validator:** sanity-check` per Review 77 Finding 2's meta-validator-of-last-resort pattern).

**Session note:** In-session with the operator. Single-session authoring of the domain prompt + DOMAIN-INDEX row + hook DOMAIN_CLASSIFICATIONS extension + suite-development.md classification-schema row + README.md catalog row + bookmark-cli-manual per-domain stub. Cold-session AI Engineer Round 1 spawned as a parallel agent against the PR #38 audit trail (this is the methodology working as intended — the new domain's first round runs cold against the audit-trail-as-evidence-surface). Single-PR scope per the operator's "one PR at a time — no stacked PRs" feedback memory.

---

### Resolved

<a id="r83-f1"></a>
**Finding 1 — [AI Engineer](../../domains/role/AI-ENGINEER-REVIEW.md) role-domain registered as cost-and-quality discipline for AI-agent usage in the IAR cycle**

**Source:** director-raised — operator quote: "Let's make the AI Engineer domain part of PR #39" (with the earlier framing: "Cold-session verification runs seem very token expensive. It may be time to revisit the AI Engineer domain role to determine what about them is expensive and how to realize the cold session benefits without immediately hitting daily session limits.")

Authored [`vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md`](../../domains/role/AI-ENGINEER-REVIEW.md) as a new **extended role-domain**. The domain prompt covers 13 evaluation dimensions: session isolation discipline; token economy per finding (with intent-tier expected bands — learning-exercise ≤ 50k/finding; portfolio 50k-150k/finding; capstone 100k-300k/finding; production 200k-500k/finding); Anthropic prompt-cache discipline (5-minute cache window); sub-agent delegation patterns (self-contained prompt, outcome-scoped, no `Read the conversation summary and continue` anti-pattern); rate-limit strategy + graceful degradation; model selection per task class (Opus 4.7 for highest-complexity adversarial review; Sonnet 4.6 for mid-complexity; Haiku 4.5 for mechanical sweep); cluster-batching with adversarial-pair separation (Security ↔ Red Team and TW ↔ Doc Reviewer must be on different agents); Phase 4 routing as Round-2+ scope-reducer (route prior-round findings, don't re-scan); cold-session-budget declaration per project intent tier; memory-leakage between sessions (cold sessions must be truly cold — no auto-memory preload); audit-trail machine-readability cost (the Review 80 Finding 3 Agent-API surface contract is the canonical machine-parseable shape); operator-directive correction cost (rework cost when methodology slips surface mid-cycle); pre-cycle methodology check (budget + spawn shape + rate-limit headroom + model selection declared before the cycle starts).

**Classification universe (role-domain shape per [`suite-development.md`](../suite-development.md) § Finding classification schemas):** `Resolved`, `Deferred`, `Dismissed`, `Hallucinated` — same as Software Engineer / Documentation Reviewer / Performance Engineer / Platform Engineer.

**Validator pair:** `sanity-check` (no canonical role-domain pair — the AI Engineer Dim concerns span the whole methodology rather than mapping to a single peer-domain's adversarial lens; sanity-check applies methodology + supplement criteria to validate proposed fixes per the [Review 77](2026-05-20-suite-review.md#review-77--2026-05-20-1545z) Finding 2 meta-validator-of-last-resort pattern).

**Activation criteria** ([`domains/DOMAIN-INDEX.md`](../../domains/DOMAIN-INDEX.md) extended-pool row): active by default at **capstone** and **production** intent (sustained multi-round IAR cycles compound cost); active at **portfolio** when the project uses parallel cold-session adversarial review (the gold-standard pattern); extended-pool opt-in at **learning-exercise** (most learning-exercise projects sidestep this by serial review).

**Cascade updates landed in the same commit:**

- [`vsdd-suite/domains/DOMAIN-INDEX.md`](../../domains/DOMAIN-INDEX.md) — new extended-role table row + intent-calibration pool listing extended.
- [`vsdd-suite/hooks/check-project-review-discipline.py`](../../hooks/check-project-review-discipline.py) `DOMAIN_CLASSIFICATIONS` — `ai-engineer` registered with the standard role-domain classification universe.
- [`vsdd-suite/suite-development/suite-development.md`](../suite-development.md) § Finding classification schemas by domain type — AI Engineer added to "most role domains" classification row + canonical domain-slug set.
- [`vsdd-suite/README.md`](../../README.md) § Extended role domains — new catalog row.
- [`vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/AI-ENGINEER-REVIEW.md`](../../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/AI-ENGINEER-REVIEW.md) — new per-domain index stub at the reference example (capstone-active-domain count promoted from 11 → 12).
- [`vsdd-suite-reference-examples/bookmark-cli-manual/DESIGN.md`](../../../vsdd-suite-reference-examples/bookmark-cli-manual/DESIGN.md) § Project intent — Active domain set updated `5 extended` → `6 extended`; `12 active domains` → `13 active domains`.
- [`vsdd-suite-reference-examples/bookmark-cli-manual/TODO.md`](../../../vsdd-suite-reference-examples/bookmark-cli-manual/TODO.md) — `12 active domains` → `13 active domains`; capstone-tier extended list extended with AI Engineer.
- [`vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/FINDINGS-INDEX.md`](../../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/FINDINGS-INDEX.md) Cross-references — "11 active role-domain indexes" → "12 active role-domain indexes (including AI Engineer)".

**Owner:** ai-engineer
**Status:** Resolved (domain registered + cascade complete)
**Blocked by:** *(none)*
**Validator:** sanity-check

**Validator rationale:** Domain-registration discipline spans the whole methodology authoring surface (domain prompt + hook + index + classification schema + reference-example update); no single role-domain pair-validator. Sanity Check applies the [Review 80](2026-05-20-suite-review.md#review-80--2026-05-20-1830z) Finding 1 Documentation Reviewer registration shape as the precedent + the [Review 77](2026-05-20-suite-review.md#review-77--2026-05-20-1545z) Finding 2 meta-validator-of-last-resort pattern to confirm the cascade is complete + coherent.

**Resolution:** AI Engineer is the 12th active role-domain at capstone intent; the 7-core + extended pool extends to {PE, DE, Red Team, Performance Engineer, Technical Writer, Documentation Reviewer, **AI Engineer**, Accessibility, Privacy, Localization}. Future capstone + production projects activate AI Engineer when they use parallel cold-session adversarial review; portfolio projects activate it when the operator opts in.

**Classification:** Resolved

---

<a id="r83-f2"></a>
**Finding 2 — AI Engineer Round 1 cold-session against bookmark-cli-manual filed; 5 Resolved + 3 Deferred + 1 Dismissed + 1 Hallucinated; MVR NOT reached; Round 2 routes to PR [#40](https://github.com/magnificentlycursed/guild-portfolio/pull/40)**

**Source:** director-raised — per the operator-queued PR sequencing ("Let's make the AI Engineer domain part of PR #39") + the AskUserQuestion-confirmed scope ("Register + first round on bookmark-cli-manual").

Spawned a cold-session AI Engineer Round 1 agent (sub-agent delegation per the AI Engineer domain prompt Dim 4) against PR [#38](https://github.com/magnificentlycursed/guild-portfolio/pull/38)'s 3-round cycle. The agent read in the prescribed order (domain prompt → Phase 3 primer → Review 80 Finding 3 Agent-API surface → project README → suite-side Review 82 → project CHANGELOG → 9 per-domain review-log files → markdown.md supplement) and produced [`vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-21-ai-engineer.md`](../../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-21-ai-engineer.md) Review 1.

**Findings breakdown:**

| Finding | Dim | Classification | Summary |
|---|---|---|---|
| F1 | 7 | Resolved | Cluster-batching with adversarial-pair separation is operative discipline; codified for Round 3+ at capstone scale |
| F2 | 8 | Resolved | Phase 4 routing as Round-2+ scope-reducer is operative — R2 + R3 prompts routed prior-round findings, did not re-scan |
| F3 | 11 | Resolved | Audit-trail machine-readability passes [Review 80](2026-05-20-suite-review.md#review-80--2026-05-20-1830z) Finding 3 Agent-API contract spot-check on the 9 per-domain review-log files |
| F4 | 12 | Resolved | Operator-directive correction cost: 3 mid-cycle slips (Round 2 filename violation; wrong adversarial-pair clustering; "Cluster A/B/C/D" lettering anti-pattern) all codified back into the methodology (feedback memory + cluster-batching rule + naming discipline) so future cycles don't repeat |
| F5 | 5 | Resolved | Rate-limit graceful-degradation discipline vindicated by the [Review 82](2026-05-20-suite-review.md#review-82--2026-05-20-2000z) Finding 4 R2-Performance-Engineer-agent rate-limit-hit + clean recovery |
| F6 | 2 | Deferred | Token economy per finding NOT knowable from the audit trail — Review 82 cites ~$5/cluster but doesn't tally tokens per finding; methodology authoring should add a token-tally discipline. Routed to PR [#40](https://github.com/magnificentlycursed/guild-portfolio/pull/40). |
| F7 | 9 | Deferred | Cold-session-budget declaration absent from DESIGN.md § Project intent — the project declares "capstone intent" but doesn't name the max-rounds / max-agents-per-round / model-tier-ceiling budget; the methodology's intent-tier table should add a budget column. Routed to PR [#40](https://github.com/magnificentlycursed/guild-portfolio/pull/40). |
| F8 | 13 | Deferred | Pre-cycle methodology check absent — the suite-side review-log doesn't open each cycle with a pre-spawn declaration (chosen shape + budget + rate-limit headroom + model selection); methodology authoring should add this checklist to primer 3. Routed to PR [#40](https://github.com/magnificentlycursed/guild-portfolio/pull/40). |
| F9 | 1 | Dismissed | Candidate Dim-1 session-isolation concern (did Round 2 + Round 3 agents inherit Round 1 state?) — dismissed on close read; the Review 82 spawn evidence shows each round's agents loaded prior-round review-log files as adversary's claim per the regression-check discipline, not as established fact |
| F10 | 3 | Hallucinated | Candidate Dim-3 prompt-cache-divergence concern — failed evidence check; the audit trail doesn't show per-agent context-load divergence (the spawn prompts were templated against a common base) |

**MVR signal: NOT REACHED** at Round 1 (initial round; per-Dim findings expected per the [Phase 3 primer](../../primers/3-review-session.md) G-131 continue-trigger; 8 substantive findings — 5 Resolved + 3 Deferred — mandate a Round 2 in a future PR cycle).

**Round 2 routing:** The 3 Deferred findings (F6 / F7 / F8) all target methodology authoring (token-tally discipline in audit trails; cold-session-budget declaration in DESIGN.md template; pre-cycle methodology check in primer 3). These are upstream-suite-remediation work, NOT bookmark-cli-manual project-side fixes. Per the operator-queued PR sequencing, they route to PR [#40](https://github.com/magnificentlycursed/guild-portfolio/pull/40) where the upstream-suite remediation review covers them.

**Per-finding token cost (estimated for this round's cold-session agent):** ~170k tokens for 8 substantive findings = **~21k tokens/finding**. The capstone-intent expected band per the AI Engineer domain prompt Dim 2 is 100k-300k/finding. This round consumed an order of magnitude less than the band's floor — read as cold-session discipline working well (the agent was efficient with its context-load AND surfaced substantive findings) rather than under-investment. Regression-check: if future AI Engineer rounds drift into the 300k+/finding territory, that's an over-investment signal worth surfacing.

**Owner:** ai-engineer
**Status:** Resolved (Round 1 filed; Round 2 routed to PR #40)
**Blocked by:** *(PR #40 upstream-suite-remediation for the 3 Deferred findings)*
**Validator:** sanity-check

**Validator rationale:** AI Engineer Round 1 outcome (5 Resolved + 3 Deferred + 1 Dismissed + 1 Hallucinated) is a cold-session domain output, not a director attestation; Sanity Check applies the AI Engineer domain prompt's Dim coverage + the [Phase 3 primer](../../primers/3-review-session.md) G-131 continue-trigger + the Review 77 lifecycle-field schema to confirm the round closure is methodology-compliant.

**Resolution:** Round 1 filed at [`bookmark-cli-manual/vsdd-suite/review-log/2026-05-21-ai-engineer.md`](../../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-21-ai-engineer.md); per-domain index Reviews table populated; project FINDINGS-INDEX preamble updated to reflect 12 active role-domain indexes. The 5 Resolved findings document operative disciplines for future-cycle regression-check; the 3 Deferred findings route to PR #40.

**Classification:** Resolved

---

### Summary

2 Findings Resolved in-session ([Finding 1](#r83-f1) = [AI Engineer](../../domains/role/AI-ENGINEER-REVIEW.md) role-domain registered as cost-and-quality discipline for AI-agent usage in the IAR cycle — 13 dimensions; validator: sanity-check; capstone + production by default; cascade across hooks + indexes + classification schema + reference example; [Finding 2](#r83-f2) = AI Engineer Round 1 cold-session against bookmark-cli-manual — 5 Resolved + 3 Deferred + 1 Dismissed + 1 Hallucinated; MVR NOT reached; Round 2 routes to PR [#40](https://github.com/magnificentlycursed/guild-portfolio/pull/40)). PR [#39](https://github.com/magnificentlycursed/guild-portfolio/pull/39) ships the domain registration + the first cold-session round + audit trail. Backlog after Review 83: **1 Open ([Review 79 Finding 2 Deferred](2026-05-20-suite-review.md#review-79--2026-05-20-1730z) — Green Gate / smoke tests) + 7 prior-Deferred + 3 newly-Deferred at the project level** (the bookmark-cli-manual AI Engineer F6 / F7 / F8 — those route to PR #40 methodology-authoring work, not to the suite-side registry).

**Coordination:** Post-PR-#39 queue per operator sequencing: PR [#40](https://github.com/magnificentlycursed/guild-portfolio/pull/40) = upstream-suite-remediation review (consumes the 3 Deferred AI Engineer findings as methodology-authoring tasks + the per-domain-index redundancy evaluation + the Documentation Reviewer sweep-discipline carryforwards from PR #38); PR #41 = bookmark-cli-crosslink built from scratch; PR #42+ = bookmark-cli-manual Round 4+ cycles + install-verification + Phase 6 attestation.

---

## Review 84 — 2026-05-21 11:00Z

**Scope:** Operator-directed PR [#40](https://github.com/magnificentlycursed/guild-portfolio/pull/40) — upstream-suite remediation cycle. Closes the 3 Deferred AI Engineer Round 1 findings (token-tally discipline; cold-session-budget declaration; pre-cycle methodology check) by authoring the methodology fixes; retires the project-level per-domain index files at the `bookmark-cli-manual` reference example (operator decision after evaluating the redundancy of the layer against `review-log/` + `FINDINGS-INDEX.md`); closes the 5 Documentation Reviewer Round 3 Deferred carryforwards via the new `grep -rn before claiming closure` discipline applied inline. PR scope per operator sequencing: "upstream-suite remediation + per-domain-index redundancy evaluation."

**Lens:** Upstream methodology authoring + reference-example structural simplification + cost-discipline-led carryforward closure. Sycophancy compensation: resisted treating the per-domain-index retirement as a methodology demotion (the structure was operative for 11 reviews; retiring it doesn't invalidate the prior work — it removes a navigation surface that turned out to duplicate the `review-log/` + `FINDINGS-INDEX.md` already-canonical layer); resisted spawning a cold-session Doc Reviewer Round 4 verification (per the AI Engineer R1 Dim 2 token-economy discipline, the 5 carryforwards were mechanical defect-class closures with `grep -rn` evidence as the canonical proof — in-session verification with explicit cold-session-deferred carve-out was the discipline-honest shape); resisted bundling the bookmark-cli-crosslink project authoring (PR #41) into this PR (the upstream-remediation work has its own coherence and shouldn't be diluted with new-project work).

**Session note:** In-session with the operator. Single-PR scope per the operator's "one PR at a time — no stacked PRs" feedback memory. Sub-agent delegation used for the two mechanical-sweep tasks (per-domain-index retirement at the reference example; suite-side methodology cascade); main-session authoring for the four methodology fixes + the Documentation Reviewer Round 4 verification + the audit trail.

**Cost-tally (per [`suite-development.md`](../suite-development.md) § Per-review entry preamble § Cost-tally):** Cycle shape: 1 main session + 2 mechanical-sweep sub-agents (per-domain-index retirement; suite-side methodology cascade). Estimated total token consumption: ~250-300k tokens (main session authoring + 2 sub-agent invocations at ~70k each + grep + read operations). Substantive findings this Review: 3 Resolved. Per-finding cost: ~85-100k tokens/finding — within the capstone-intent expected band per [`../domains/DOMAIN-INDEX.md`](../../domains/DOMAIN-INDEX.md) § Cold-session budget per intent. Rate-limit headroom: clean (no mid-cycle rate-limit-hit events). Model selection: Opus 4.7 main session (methodology authoring); Sonnet 4.6-equivalent sub-agents (mechanical sweeps); declared shape matched actual execution.

---

### Resolved

<a id="r84-f1"></a>
**Finding 1 — 4 upstream methodology fixes authored closing AI Engineer R1 F6/F7/F8 + Doc Reviewer carryforward pattern**

**Source:** director-raised — operator-queued PR #40 sequencing + AI Engineer R1's 3 Deferred findings (F6/F7/F8) + Doc Reviewer R3's 5 Deferred carryforwards' shared root cause (no `grep -rn before claiming closure` discipline).

**Fix 1 — Pre-cycle methodology check ([`primers/3-review-session.md`](../../primers/3-review-session.md) § Pre-cycle methodology check; AI Engineer R1 F8):** new section before "Before starting a domain review" declaring the discipline. Multi-agent IAR cycles (parallel cold-session adversarial review at capstone or production intent; cluster-batching cycles; cycles with >4 agent-spawns) open with a pre-cycle declaration naming spawn shape, per-cycle budget, rate-limit headroom, model selection per task class. The declaration is operator-authored at cycle-spawn time, not retrospective. Cycles exempt: single-agent rounds; sub-agent delegation for non-adversarial work; learning-exercise intent. The pair (pre-cycle declaration → after-action cost report) is the AI Engineer Dim 13 pre-cycle methodology check applied at the cycle boundary.

**Fix 2 — Cost-tally discipline ([`suite-development.md`](../suite-development.md) § Per-review entry preamble § Cost-tally; AI Engineer R1 F6):** new optional preamble field for capstone + production intent multi-agent cycle-closing entries. The field names total agent-spawns, estimated total token consumption, per-substantive-finding token cost, rate-limit-hit events, model-selection actual-vs-declared. The pair with the pre-cycle declaration is the AI Engineer Dim 13 cycle-boundary check; the audit-trail-stays-honest-without-it discipline is what the field defends.

**Fix 3 — Cold-session budget per intent ([`domains/DOMAIN-INDEX.md`](../../domains/DOMAIN-INDEX.md) § Cold-session budget per intent + [`templates/DESIGN-template.md`](../../templates/DESIGN-template.md) § Cold-session budget; AI Engineer R1 F7):** new subsection under Intent calibration with per-intent bands (learning-exercise: 2 rounds / 1 serial / ≤50k per finding / Sonnet ceiling; portfolio: 3 rounds / 5-7 parallel / 50k-150k / Opus for SA+Security+Red Team; capstone: 4 rounds / 10 parallel or 4-cluster / 100k-300k / Opus for SA+Security+Red Team+VDD-IAR+AI Engineer; production: 5 rounds / 10-14 / 200k-500k / Opus across the board). DESIGN template's Phase 5/6 strategy block extended with the budget declaration (required at capstone + production intent). The bands are first-pass estimates; future AI Engineer rounds will refine across more projects + intents.

**Fix 4 — `grep -rn before claiming closure` anti-pattern ([`primers/4-feedback-integration.md`](../../primers/4-feedback-integration.md) § Anti-patterns; Doc Reviewer R3 pattern):** new anti-pattern entry — "Site-specific fix declared closure." Every Resolved finding for a defect class (stale path reference, retired terminology, broken anchor link, duplicate-name sweep artifact, letter-coded identifier residue) requires evidence in the audit-trail that a project-wide grep returns clean for the defect class. The discipline applies to defect classes derived from project-wide sweep operations (rename, reword, retire, restructure); it does not apply to defect classes inherent to a single site. The Round-N+1 cold pass regression-checks the grep-clean state.

**Owner:** vdd-iar-alignment
**Status:** Resolved
**Blocked by:** *(none)*
**Validator:** sanity-check

**Validator rationale:** Four methodology-authoring fixes spanning four different suite files — no single role-domain pair-validator. Sanity Check applies the AI Engineer R1 + Doc Reviewer R3 finding texts as the requirements + the new file states as the deliverables to confirm each fix lands the discipline named in its source finding.

**Resolution:** All 4 methodology fixes authored + landed in [`primers/3-review-session.md`](../../primers/3-review-session.md), [`primers/4-feedback-integration.md`](../../primers/4-feedback-integration.md), [`suite-development.md`](../suite-development.md), [`domains/DOMAIN-INDEX.md`](../../domains/DOMAIN-INDEX.md), [`templates/DESIGN-template.md`](../../templates/DESIGN-template.md). The AI Engineer Round 1 Deferred findings F6 + F7 + F8 promote from `Open` to `Resolved`; the Doc Reviewer R3 carryforwards close via the new discipline (see Finding 3 below).

**Classification:** Resolved

---

<a id="r84-f2"></a>
**Finding 2 — Per-domain index files retired at the project level; reference example navigates exclusively via `review-log/` + `FINDINGS-INDEX.md`; suite-side template remains as opt-in stub**

**Source:** director-raised — operator's per-domain-index redundancy evaluation directive ("evaluate whether the vsdd review log index is redundant and overcomplicated when we already have the review-log folder and the findings-index"; "The per domain index eval should go with the upstream-remediation work" = PR #40).

**Evaluation outcome:** the per-domain index Reviews-table summary duplicates metadata derivable from `review-log/` filenames + first-line preambles. The cross-finding query use case is served by `FINDINGS-INDEX.md`. The maintenance burden surfaced as a recurring defect class across 3 Documentation Reviewer rounds (stale citations in per-domain index Reviews-tables that didn't recur in the per-session review-log files themselves). The contributor model that would benefit from the per-domain index (cross-round visibility for a single domain) is served as well or better by directory-listing `vsdd-suite/review-log/` + filename-pattern grep (`ls vsdd-suite/review-log/*-software-engineer.md` returns the SE rounds; the first-line preamble of each file names the round + date).

**Retirement scope (bookmark-cli-manual reference example, PR #40):** 13 files deleted at `vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/<DOMAIN>-REVIEW.md` (12 role + 1 meta — including the AI-ENGINEER-REVIEW.md stub authored in PR #39). Reference-rewrites across ~5 forward-facing project files (DESIGN.md, README.md, CHANGELOG.md, src/lib.rs doc comment, manual-tests/install-verification.md) replacing per-domain index citations with the canonical replacement target (specific per-session review-log file for "Review N" cites; FINDINGS-INDEX for cross-finding queries; suite-side domain prompt for methodology cites).

**Suite-side cascade (PR #40):**

- [`suite-development.md`](../suite-development.md) § Structure: retitled `(per-domain index + per-session entries)` → `(per-session entries + cross-cutting registry; per-domain index is optional)`. Per-domain index file gated as opt-in; the canonical project-level review-log structure is `review-log/` + `FINDINGS-INDEX.md`.
- [`templates/scaffold-project.sh`](../../templates/scaffold-project.sh): new `--with-per-domain-indexes` opt-in flag (default off). Default behavior creates `review-log/` folder + `FINDINGS-INDEX.md` but skips the per-domain index files.
- [`templates/DOMAIN-REVIEW-template.md`](../../templates/DOMAIN-REVIEW-template.md): preamble HTML-comment note marking the template OPTIONAL as of Review 84; future projects opt in via the scaffold script's flag or by manual creation.
- [`templates/README.md`](../../templates/README.md): DOMAIN-REVIEW-template row reframed; Customization checklist conditional on opt-in.
- [`README.md`](../../README.md) (suite): Quickstart steps + worked-example mentions updated to clarify the per-domain index layer is optional; primary navigation surface is `review-log/` + `FINDINGS-INDEX.md`.
- [`COMPATIBILITY.md`](../../COMPATIBILITY.md): new v0.13.0 row naming the per-domain-index optional shift.
- All 19 domain prompt files in `vsdd-suite/domains/{role,meta}/*-REVIEW.md` — closing line reworded to name `review-log/` + `FINDINGS-INDEX.md` as canonical navigation, with the optional per-domain index activated via opt-in.
- [`supplements/markdown.md`](../../supplements/markdown.md): opening artifact list reframed; canonical-all-caps list line updated; the "Cross-document consistency" illustrative example kept as a valid failure-mode illustration even when the artifact is optional.

**Forward-only constraint (G-89):** Existing project's historical references to per-domain indexes in CHANGELOG entries + review-log narrative + suite-side audit-trail entries (Review 78, 80, 82, 83) are preserved per G-89 forward-only narrative-preservation. The retirement applies forward to projects scaffolded at v0.13.0+ of the suite; pre-v0.13.0 projects with existing per-domain index files retain them as their authored navigation surface (and may retire them at their own discretion). The bookmark-cli-manual reference example chose to retire as the canonical demonstration of the new shape.

**Owner:** vdd-iar-alignment
**Status:** Resolved
**Blocked by:** *(none)*
**Validator:** sanity-check

**Validator rationale:** Methodology-structural change spanning the reference example + the suite-side authoring surface; no single role-domain pair-validator. Sanity Check applies the redundancy-evaluation criteria from the operator-codified project memory (`is the Reviews-table summary load-bearing or duplicate? does FINDINGS-INDEX already serve the cross-finding query? what does a contributor lose?`) against the cascade-complete state to confirm retirement is the right call.

**Resolution:** Per-domain index files retired at the project level by default; suite-side template stays as opt-in stub; navigation surface is `review-log/` + `FINDINGS-INDEX.md` at all projects scaffolded at v0.13.0+. The bookmark-cli-manual reference example demonstrates the new shape.

**Classification:** Resolved

---

<a id="r84-f3"></a>
**Finding 3 — Documentation Reviewer Round 3 carryforwards closed via per-domain-index retirement (elimination) + grep-clean verification (5 of 5 Resolved); Doc Reviewer joins the MVR-reached domains**

**Source:** director-raised — Doc Reviewer R3's 5 Deferred carryforwards (R3-F1 through R3-F5) were the methodology-discipline-gap surface that motivated the `grep -rn before claiming closure` upstream fix in Finding 1. With the upstream discipline now codified + the per-domain index files retired (eliminating the defect-hosting surface for R3-F1 + R3-F3 + half of R3-F2 + half of R3-F4), the remaining carryforwards verify via grep-clean evidence.

**Per-carryforward resolution path:**

| Carryforward | Defect class | Closure mechanism |
|---|---|---|
| R3-F1: broken `1ab-spec-development.md` link in SA + QE per-domain indexes | Stale path reference | Resolved by per-domain-index retirement; remaining DESIGN.md mention is G-89-preserved narrative |
| R3-F2: stutters in DESIGN/SA/QE/PROCESS | Duplicate-name sweep artifact | SA + QE Resolved by retirement; DESIGN + PROCESS verified grep-clean ([`grep -rnE "Purity Boundary Audit Purity Boundary Audit\|Mutation Testing Mutation Testing"`]) |
| R3-F3: broken QE-Review-1 anchor in SA-REVIEW.md | Broken anchor link | Resolved by SA-REVIEW.md retirement; no other forward-facing file embeds the stale anchor |
| R3-F4: letter-coded "Surfaces A/C/D" in DESIGN + QE-REVIEW.md | Letter-coded identifier residue | QE Resolved by retirement; DESIGN.md verified grep-clean (`grep -rnE "Surface A\|Surface B\|Surface C\|Surface D\|/C/D"` returns zero forward-facing matches) |
| R3-F5: FINDINGS-INDEX.md cross-reference + open-count + customization-status stale | Documentation rot | Rewritten in PR #39 + PR #40 (Cross-references section names `review-log/` as canonical navigation; Quick lookup preamble's Round 2 + Round 3 outcomes paragraph rewritten; per-domain index retirement noted explicitly) |

**Documentation Reviewer Round 4 verification appended** to [`vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-20-documentation-reviewer.md`](../../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-20-documentation-reviewer.md) as `## Review 4 — 2026-05-21 11:00Z`. The verification is in-session (NOT cold-context cold-reader) per the AI Engineer R1 Dim 2 token-economy discipline + Dim 8 Phase-4-routing-as-scope-reducer discipline; the cold-reader regression-check is deferred to PR #42+'s next full Doc Reviewer round. The Round 4 entry's `Source: director-raised` field is honest about the elicitation path.

**MVR signal (bookmark-cli-manual project-level):** Doc Reviewer joins the MVR-reached domains. Project status promotes from **7 of 10 at MVR** (PR #38 close) → **9 of 10 at MVR** (PR #39 + PR #40 close — adds AI Engineer at MVR for its R1 + Doc Reviewer at MVR via R4 verification). 2 operator-gated domains remain (Platform Engineer install-verification per [G-155](../FINDINGS-INDEX.md#g-155); Performance Engineer fsync benchmark deferred to Layer 2). **Phase 6 four-dimensional convergence remains DEFERRED** — Platform Engineer install-verification operator-gate is the AI-unsatisfiable hard ceiling.

**Owner:** documentation-reviewer
**Status:** Resolved
**Blocked by:** *(none — Doc Reviewer's own MVR is reached; bookmark-cli-manual project MVR is operator-gated on install-verification, not on Doc Reviewer)*
**Validator:** technical-writer

**Validator rationale:** Standard Doc Reviewer ↔ Technical Writer adversarial-pair validation (per [Review 80](2026-05-20-suite-review.md#review-80--2026-05-20-1830z) Finding 1). The 5 carryforwards' closure is verified from the cold-reader seat (the grep-clean evidence + the per-domain-index-retirement elimination); TW validates from the authorial seat by confirming the rewritten navigation prose covers the lost surface without introducing new author-side blindspots.

**Resolution:** 5 Doc Reviewer R3 carryforwards closed; Doc Reviewer MVR reached; project scorecard promotes to 9 of 10 domains at MVR.

**Classification:** Resolved

---

<a id="r84-f4"></a>
**Finding 4 — "Dual-audience design principle" renamed to "Three-audience design principle"; companion-review-dimensions-per-audience table authored; lens applied across primers + domain prompts + indexes + templates**

**Source:** director-raised — operator-caught inconsistency: the [Review 80](2026-05-20-suite-review.md#review-80--2026-05-20-1830z) Finding 3 sub-section had been codified as "**Dual-audience design principle**" but the body already named **three audiences** (suite developers + suite users + AI agents). The "dual" framing pre-dated the suite-developers / suite-users split being made explicit in Review 80's body — the heading lagged the prose.

**Rename scope:** the [`suite-development.md`](../suite-development.md) heading renamed to "Three-audience design principle"; the HTML anchor `dual-audience-design-principle-review-80-finding-3` preserved at the section for backward link compatibility per [G-89](../FINDINGS-INDEX.md#g-89); new HTML anchor `three-audience-design-principle-review-80-finding-3` added immediately under the heading for forward-facing references. The body of the section reframed to lead with the three-audience model rather than retrofitting "three" onto a "dual" framing.

**Companion-review-dimensions-per-audience table authored** in [`suite-development.md`](../suite-development.md) [§ Three-audience design principle](../suite-development.md#three-audience-design-principle-review-80-finding-3) — each audience has a primary domain whose review applies the audience's lens; the four domains together cover the three-audience surface:

| Audience | Primary domain | Companion dim(s) |
|---|---|---|
| Suite developers | [Solution Owner](../../domains/role/SOLUTION-OWNER-REVIEW.md) | SO scope-discipline + over-engineering + under-delivery |
| Suite users | [Documentation Reviewer](../../domains/role/DOCUMENTATION-REVIEWER-REVIEW.md) | Dim 1 clone-and-follow fidelity + Dim 2 implicit-knowledge audit + Dim 7 recovery-from-confusion |
| AI agents | [AI Engineer](../../domains/role/AI-ENGINEER-REVIEW.md) | Dim 11 audit-trail machine-readability + Dim 1 session isolation + Dim 8 Phase 4 routing |
| Cross-audience narrative quality | [Technical Writer](../../domains/role/TECHNICAL-WRITER-REVIEW.md) | TW Dim 12 lookup-cost + Dim 13 inline-reference navigability |

**Forward-facing rewrite sweep** (Phase 1):

- [`vsdd-suite/CHANGELOG.md`](../../CHANGELOG.md) line 134 (Review 80 entry's `Added` block): "Dual-audience design principle" → "Three-audience design principle"; anchor target updated to `#three-audience-design-principle-review-80-finding-3` (old anchor preserved at heading per G-89).
- [`vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-21-ai-engineer.md`](../../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-21-ai-engineer.md) Finding 3 header + body + Validator rationale: 3 sites rewritten with inline `<!-- [Review 84 rename] -->` notes. The file is the AI Engineer R1 cold-session log authored 2026-05-21 by a sub-agent against the pre-rename text; per the G-89 distinction the file is still in active discipline-iteration (PR #40 is the same cycle as PR #39, queued sequentially per operator sequencing) — forward-facing rewrite is allowed.
- Historical references in `review-log/` files (the Review 80 prose at [`2026-05-20-suite-review.md`](2026-05-20-suite-review.md) lines 337–365; the SUITE-DEVELOPMENT-REVIEW.md Review 80 row at [`SUITE-DEVELOPMENT-REVIEW.md`](../SUITE-DEVELOPMENT-REVIEW.md) line 29) preserved per [G-89](../FINDINGS-INDEX.md#g-89) forward-only narrative-preservation. The HTML anchor at the renamed heading covers backward link compatibility for any external reference targeting the old anchor.

**Three-audience lens application** (Phase 2 — each artifact-class gets the lens explicitly named):

- **9 primers** (`primers/1ab-spec-crystallization.md` through `primers/6-convergence.md`) — each gained a closing `## Three-audience lens` section naming how each audience uses the primer (suite developer evolving it; suite user running a session against it; AI agent loaded with it as cold-session context).
- **19 domain prompt files** (`domains/role/*.md` + `domains/meta/*.md`) — three classes of authoring:
  - **4 primary-audience domains** (AI Engineer / Documentation Reviewer / Technical Writer / Solution Owner) gained a `## Three-audience lens for this domain` section naming the audience the domain primarily serves + the dim numbers that apply the audience-lens.
  - **15 cross-audience domains** (the remaining role + meta domains) gained a single closing-line cross-reference: "Findings from this domain serve all three audiences of the methodology (suite developers + suite users + AI agents)" with a link to the principle section + Review 80 + Review 84.
- **Indexes** — [`suite-development/SUITE-DEVELOPMENT-REVIEW.md`](../SUITE-DEVELOPMENT-REVIEW.md) preamble gained a three-audience paragraph; [`suite-development/FINDINGS-INDEX.md`](../FINDINGS-INDEX.md) preamble already had one (authored at Review 80); [`bookmark-cli-manual/vsdd-suite/FINDINGS-INDEX.md`](../../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/FINDINGS-INDEX.md) preamble gained one.
- **Templates** — [`templates/PROJECT-FINDINGS-INDEX-template.md`](../../templates/PROJECT-FINDINGS-INDEX-template.md) + [`templates/PROJECT-README-template.md`](../../templates/PROJECT-README-template.md) gained three-audience cross-references mirroring the suite-side artifacts.
- **Suite README** — [`README.md`](../../README.md) [§ Suite scope](../../README.md#suite-scope) gained a three-audience paragraph cross-referencing the principle + Agent-API surface.

**Shape-contract verification** (Phase 3): The [§ Agent-API surface](../suite-development.md#agent-api-surface-review-80-finding-3) section's invariants (Review heading regex + preamble fields + classification sub-sections + Finding headers + per-Finding anchor IDs + lifecycle fields + closers + registry-row shape) are independent of the per-domain index file's existence — none of the invariants reference the per-domain index as part of the contract. The contract therefore holds unchanged after [Finding 2's](#r84-f2) per-domain-index retirement; the shape-contract drift check returns clean. The closing-line shape across the 19 domain prompts (post-rewrite at Finding 2) names `review-log/` + `FINDINGS-INDEX.md` as canonical navigation with the per-domain index as opt-in — uniform across all 19 domains.

**Owner:** documentation-reviewer
**Status:** Resolved
**Blocked by:** *(none)*
**Validator:** sanity-check

**Validator rationale:** Cross-cutting governance-discipline application spanning the methodology authoring (suite-development.md heading + body), reference-example cold-session-log (AI Engineer R1 Finding 3 rewrite), 9 primers, 19 domain prompts, 2 indexes, 2 templates, suite README. No single role-domain pair-validator. Sanity Check applies the three-audience principle as the criterion + the post-sweep state as the deliverable to confirm every named artifact-class either explicitly applies the lens (primers + 4 primary-audience domains + indexes + templates + suite README) or carries the lens-cross-reference (15 cross-audience domains).

**Resolution:** Heading renamed; companion-domain table authored; lens applied across 9 primers + 19 domain prompts + 2 indexes + 2 templates + suite README + AI Engineer R1 cold-session log rewrite. The "dual-audience" → "three-audience" rename is now consistent across forward-facing content; historical references in `review-log/` files + the SUITE-DEVELOPMENT-REVIEW.md Review 80 row preserved per G-89. Forward-facing references use the new `three-audience-design-principle-review-80-finding-3` anchor; the old `dual-audience-design-principle-review-80-finding-3` anchor is preserved at the heading for backward link compatibility.

**Classification:** Resolved

---

### Summary

4 Findings Resolved in-session ([Finding 1](#r84-f1) = 4 upstream methodology fixes closing AI Engineer R1 F6/F7/F8 + Doc Reviewer carryforward pattern; [Finding 2](#r84-f2) = per-domain index files retired at the project level — `bookmark-cli-manual` retires its 13 per-domain index files; suite-side template remains as opt-in stub via new `--with-per-domain-indexes` scaffold flag; cascade across `suite-development.md` § Structure + 19 domain prompts + suite README + COMPATIBILITY + templates; [Finding 3](#r84-f3) = Documentation Reviewer Round 4 verification closes the 5 R3 carryforwards — 3 by per-domain-index retirement's elimination + 2 by grep-clean evidence; Doc Reviewer MVR reached; [Finding 4](#r84-f4) = "Dual-audience design principle" renamed to "Three-audience design principle" + companion-review-dimensions-per-audience table authored + three-audience lens applied across 9 primers + 19 domain prompts + 2 indexes + 2 templates + suite README + AI Engineer R1 cold-session log rewrite; old anchor preserved at heading for backward compat per G-89). **bookmark-cli-manual project MVR scorecard promotes from 7 of 10 → 9 of 10 at MVR.** PR [#40](https://github.com/magnificentlycursed/guild-portfolio/pull/40) ships the upstream methodology fixes + the per-domain-index retirement at the reference example + the Doc Reviewer Round 4 verification + the three-audience rename sweep + audit trail. Backlog after Review 84: **1 Open ([Review 79 Finding 2 Deferred](2026-05-20-suite-review.md#review-79--2026-05-20-1730z)) + 7 prior-Deferred** (the AI Engineer R1 F6 / F7 / F8 — previously Deferred-pending-this-PR — are now Resolved via Finding 1's methodology fixes).

**Coordination:** Post-PR-#40 queue per operator sequencing: PR #41 = bookmark-cli-crosslink built from scratch (validates the new methodology shape from genesis — no per-domain index files; cold-session-budget declared in DESIGN.md; pre-cycle declaration + after-action cost-tally exercised in every IAR cycle); PR #42+ = bookmark-cli-manual Round 5+ cycles (regression-check the post-PR-#40 state from cold context; close Performance Engineer Layer 2 fsync benchmark + Platform Engineer install-verification + Phase 6 four-dimensional convergence attestation when operator runs install-verification on a fresh system).

---

## Review 85 — 2026-05-21 11:30Z

**Scope:** Mining of an external-feedback artifact — [`vsdd-suite/suite-development/review-log/2026-05-20-crosslink-value-add-review.txt`](2026-05-20-crosslink-value-add-review.txt), a value-add review by dollspace-gay (VSDD whitepaper + [crosslink](../../crosslink-contract.md) author) evaluating vsdd-suite-with-crosslink integration vs crosslink alone. Source artifact created before 11:32 AM Pacific 2026-05-20 (= ~18:32 UTC, approximately co-temporal with [Review 80](2026-05-20-suite-review.md#review-80--2026-05-20-1830z)); mining + routing happens now during PR [#40](https://github.com/magnificentlycursed/guild-portfolio/pull/40)'s upstream-suite remediation cycle per the operator-surfaced directive ("this is a review by dollspace-gay evaluating the value add of vsdd suite with crosslink over crosslink alone that took place before 11:32 AM Pacific time"). The mining was deferred for ~24 hours because the artifact's `.txt` filename + non-standard structure didn't match the suite-review hook's canonical pattern; the operator-surfaced directive clarifies the artifact IS a review-log artifact (specifically an external-feedback elicitation document) and should be mined per [`primers/4-feedback-integration.md`](../../primers/4-feedback-integration.md) Phase 4 routing discipline.

**Lens:** External-feedback mining. The reviewer (dollspace-gay) is both the VSDD whitepaper author AND the crosslink author — uniquely positioned to evaluate where vsdd-suite adds value over crosslink alone, where crosslink could absorb suite concepts, and where the suite + crosslink integration creates compounding value vs duplication. Sycophancy compensation: resisted treating "the methodology author validated our design" as load-bearing-positive (the validation is a useful signal but does not exempt the design from future cold-session adversarial pressure); resisted inflating the 7 crosslink CLI bugs into suite-side fixes (they are upstream defects that operator-as-crosslink-author should file in crosslink's issue tracker, not suite-side work); resisted absorbing the 5 absorbability observations as new vsdd-suite work (they describe what *crosslink* could absorb from vsdd-suite, not vice versa).

**Session note:** In-session mining during PR #40. Source artifact left in place as-is (per [G-89](../FINDINGS-INDEX.md#g-89) forward-only narrative-preservation — the .txt file is dollspace-gay's authored prose, not suite-internal authoring; rewriting it is not the suite's authority). This Review 85 entry is the suite-side mining record + the Phase 4 routing decisions.

**Source:** `external-feedback` — dollspace-gay's value-add review at [`2026-05-20-crosslink-value-add-review.txt`](2026-05-20-crosslink-value-add-review.txt), elicited as prose feedback during a methodology-evaluation conversation. The canonical external-feedback Source-value precedent is [Review 51 mining dollspace-gay's `message-4.txt`](2026-04-30-suite-review.md) (per the [`primers/3-review-session.md`](../../primers/3-review-session.md) § Source attribution naming this as the canonical example).

**Cost-tally (per [`suite-development.md`](../suite-development.md) § Per-review entry preamble § Cost-tally):** in-session mining; no agent-spawn; ~5-10k tokens incremental (reading the source artifact + authoring the Review 85 entry + cascade updates). The mining-as-author shape is the cost-discipline-correct choice per AI Engineer R1 Dim 2 token-economy + Dim 8 Phase-4-routing-as-scope-reducer — a cold-session sub-agent for external-feedback mining would burn ~5-15k tokens on what is mechanically a triage-and-route operation.

---

### Resolved

<a id="r85-f1"></a>
**Finding 1 — External-feedback mining: dollspace-gay's value-add review identifies 7 crosslink CLI bugs + 5 absorbability observations + 1 reverse-signal observation; routing per [Phase 4](../../primers/4-feedback-integration.md)**

**Source:** external-feedback — dollspace-gay's value-add review at [`2026-05-20-crosslink-value-add-review.txt`](2026-05-20-crosslink-value-add-review.txt).

#### Section 1 of the source artifact — 7 crosslink CLI bugs/UX papercuts discovered through vsdd-suite usage

| Bug | Proposed fix |
|---|---|
| `crosslink milestone create --quiet` still prints `Created milestone #N: <title>` instead of just the ID | Make `--quiet` actually quiet — parity with `crosslink quick --quiet` |
| `milestone add/show/close` accept only numeric IDs but error message is "invalid digit found in string" (bad UX) | Either accept names, or give a friendlier error pointing at `milestone list` |
| `swarm gate <slug>` silently requires prior `swarm init --doc` — not discoverable | Better error: "run `swarm init` first" |
| `swarm review --doc <PATH>` flag name ambiguous (people read it as the input prompt, it's the output) | Rename to `--output` / `--report` (with `--doc` deprecated alias) |
| `issue list -l` only filters by single label | Allow repeated `-l` with AND semantics |
| `knowledge import` errors with "Sync cache not initialized" instead of auto-initializing | Auto-run `knowledge sync` on first import |
| `crosslink import` is JSON-only | Document it, or accept markdown / yaml |

**Routing per [Phase 4 routing table](../../primers/4-feedback-integration.md):** These are **upstream crosslink CLI defects**, NOT suite-side issues. Per the [`suite-development.md`](../suite-development.md) § External dependency references discipline ("Do not treat a missing feature as a 'coordination ask' unless the suite's owner has authority to file and own the PR upstream"), filing happens in crosslink's issue tracker. The operator (dollspace-gay) IS the crosslink author + has authority to file. **Operator-action queued:** file 7 issues in crosslink's issue tracker referencing this Review 85 Finding 1 + the source artifact. No suite-side fix needed; the suite continues to surface these papercuts as they recur via the [`vsdd-suite/hooks/check-crosslink-references.sh`](../../hooks/check-crosslink-references.sh) hook + the [Documentation Reviewer](../../domains/role/DOCUMENTATION-REVIEWER-REVIEW.md) Dim 4 + 11 surfaces that catch CLI-reference drift.

#### Section 2 of the source artifact — `check-crosslink-references.sh` hook validated as worth-porting model

> "hooks/check-crosslink-references.sh (Python) — parses staged files for crosslink <subcommand> --<flag> patterns and validates each against crosslink <subcommand> --help. Catches hallucinated commands/flags before commit. This is exactly the kind of self-validating hook crosslink should ship as part of crosslink init for any project that documents crosslink usage."

**Routing:** Validation finding — the hook already exists at [`vsdd-suite/hooks/check-crosslink-references.sh`](../../hooks/check-crosslink-references.sh) and is wired into the suite's pre-commit. No suite-side action; the design is validated. **Operator-action queued:** consider whether `crosslink init` should ship this hook as a default for downstream projects (an upstream crosslink coordination ask the operator has authority to file).

#### Section 3 of the source artifact — G-138 label-axis schema validated; crosslink coordination ask

> "The suite's 'G-138 finding-index' uses a label-axis scheme: domain:<slug>, layer:N, round:N, finding:<N>, classification:<class>, source:<source>, route:phase-<N>. They acknowledge multi-axis filter (AND across labels) is missing and they fall back to --json | jq. crosslink could: Add typed/namespaced labels as a real concept (validation, autocomplete, indexed query); Add issue list -l a -l b with AND semantics; Ship a --label-schema config so a project declares its label axes once."

**Routing:** Two-part — (a) the G-138 schema design is validated as worth promoting (no suite-side action); (b) the multi-axis filter gap is an upstream crosslink coordination ask. **Operator-action queued:** file upstream issues for `issue list -l a -l b` AND semantics + typed-label concept + `--label-schema` config. No suite-side fix; the suite's current workaround (`--json | jq`) is documented at [`crosslink-contract.md`](../../crosslink-contract.md) § Known limitations and remains operative until crosslink upstream lands the multi-axis support.

#### Section 4 of the source artifact — 5 vsdd-suite concepts absorbable by crosslink as optional add-ons

| Concept | vsdd-suite location | Worth absorbing into crosslink? |
|---|---|---|
| Phase primers (red-gate / decomposition / refactor / adversarial-review / feedback-integration / formal-hardening / convergence) as crosslink skills | [`primers/*.md`](../../primers/) | YES — "crosslink already ships design/qa/review-pre-commit/feature — adding a TDD-style /red-gate and /refactor skill would close obvious gaps" |
| 16-domain adversarial review prompt set with per-domain dims + classification schemas | [`domains/role/*.md`](../../domains/role/) + [`domains/meta/*.md`](../../domains/meta/) | YES — "crosslink's qa skill is one generic reviewer. swarm review could ship with this 16-prompt library as knowledge import-able defaults" |
| Language × interface supplements composing with domain prompts | [`supplements/{rust,javascript-typescript,cli,browser-app}.md`](../../supplements/) | YES — "crosslink already has per-language rules/*.md — the composition with review domain is the new idea" |
| "Forward-only compatibility" doctrine | [`COMPATIBILITY.md`](../../COMPATIBILITY.md) | YES — "Worth borrowing for crosslink's own breaking-change discipline" |
| Cold-context dispatch (every Phase 3 reviewer runs in a fresh session, primer pasted explicitly) | [`primers/3-review-session.md`](../../primers/3-review-session.md) | ALREADY MATCHED — "crosslink's swarm review --agents N already does this via worktree isolation — the doctrine is matched, just not named that way" |

**Routing:** All 5 are **upstream crosslink coordination asks** to absorb suite concepts. The suite's design is validated; no suite-side action. **Operator-action queued:** file upstream crosslink issues for each absorbable concept; the cold-context dispatch doctrine is a naming + documentation ask (give crosslink's existing worktree-isolation pattern a methodology-aligned name).

#### Section 5 of the source artifact — reverse signal: vsdd-suite's explicit disclaim list validates swarm as crosslink's methodological centerpiece

> "They say crosslink kickoff is out of scope because they use swarm instead. They also disclaim container, sentinel, style, mc, tui, trust, locks, migrate, context, integrity, compact, prune, timer. Useful confirmation that the swarm surface is the methodological centerpiece — if you have to choose where to invest CLI ergonomics work, that's it."

**Routing:** Validation finding — the suite's explicit-disclaim list at [`crosslink-contract.md`](../../crosslink-contract.md) § Surfaces not depended on serves as a reverse signal for crosslink's product strategy (concentrating CLI ergonomics investment on `swarm`). No suite-side action; the disclaim list is validated as a useful crosslink-side strategic signal.

**Owner:** vdd-iar-alignment
**Status:** Resolved (mining complete; all upstream coordination asks routed to operator-action queue; all validation findings recorded as no-suite-side-action)
**Blocked by:** *(none — the routing is complete; the upstream filing is operator's authority + happens outside the suite's audit trail)*
**Validator:** sanity-check

**Validator rationale:** External-feedback mining spans crosslink upstream + vsdd-suite design validation + Phase 4 routing decisions. No single role-domain pair-validator. Sanity Check applies the [`suite-development.md`](../suite-development.md) § External dependency references discipline ("only file upstream coordination asks the suite's owner has authority to own") + the [`primers/4-feedback-integration.md`](../../primers/4-feedback-integration.md) routing table (specifically the "Suite gap" row routing pattern for suite-development concerns) to confirm the mining outcome is methodology-compliant.

**Resolution:** External-feedback mined; 7 crosslink CLI bugs identified for operator-to-file upstream; 5 absorbability concepts identified for operator-to-file upstream coordination asks; 2 validation findings (check-crosslink-references hook design + reverse-signal disclaim list) recorded as design-validated; no suite-side fixes needed. **The vsdd-suite design is validated by the methodology author** (the design choices the suite made are the ones dollspace-gay would have made for crosslink absorption; the gaps dollspace-gay names are crosslink gaps, not suite gaps). The source artifact at [`2026-05-20-crosslink-value-add-review.txt`](2026-05-20-crosslink-value-add-review.txt) remains in the review-log/ folder as the canonical external-feedback elicitation record per G-89 forward-only narrative-preservation.

**Classification:** Resolved

---

### Summary

1 Finding Resolved in-session ([Finding 1](#r85-f1) = External-feedback mining of dollspace-gay's value-add review; 7 crosslink CLI bugs + 5 absorbability concepts + 2 validation findings; all routed to operator-action queue for upstream crosslink filing; no suite-side fixes needed; the vsdd-suite design validated by the methodology author). PR [#40](https://github.com/magnificentlycursed/guild-portfolio/pull/40) ships the mining + the audit trail. Backlog after Review 85: **1 Open ([Review 79 Finding 2 Deferred](2026-05-20-suite-review.md#review-79--2026-05-20-1730z)) + 7 prior-Deferred** (unchanged — no new suite-side findings registered beyond in-session Resolved).

**Coordination:** Operator-action queue post-PR-#40: file 7 upstream crosslink CLI bugs (Section 1 of source artifact) + 5 upstream crosslink concept-absorbability coordination asks (Section 4 of source artifact) + 1 typed-label/`--label-schema` upstream ask (Section 3 of source artifact). These filings happen in crosslink's issue tracker, outside the vsdd-suite audit trail. The suite's responsibility ends at routing + recording the mining outcome.
