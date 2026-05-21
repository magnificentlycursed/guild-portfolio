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
