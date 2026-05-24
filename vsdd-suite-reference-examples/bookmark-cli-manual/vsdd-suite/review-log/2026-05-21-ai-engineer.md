# AI Engineer Review — bookmark-cli-manual

[Index](../AI-ENGINEER-REVIEW.md)

---

## Review 1 — 2026-05-21 10:00Z

**Source:** director-raised — [AI Engineer](../../../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md) domain newly registered in suite-side Review 83; first round audits PR [#38](https://github.com/magnificentlycursed/guild-portfolio/pull/38)'s 3-round cycle (R1 = 10 parallel cold-session agents; R2 = 10 parallel cold-session agents; R3 = 4 clusters with adversarial-pair separation).

**Scope:** AI-agent-usage shape across PR [#38](https://github.com/magnificentlycursed/guild-portfolio/pull/38)'s R1 + R2 + R3 cycle. NOT the shipped binary; NOT the [`DESIGN.md`](../../DESIGN.md) / [`src/`](../../src/) artifacts. Surface: the spawn shape, the cluster decisions, the prompt-cache discipline, the model selection, the audit-trail machine-readability, and the operator-directive correction cost. Evidence base: suite-side [Review 82](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z), project [`CHANGELOG.md`](../../CHANGELOG.md) v0.11.4 + v0.11.5 entries, and the 9 per-domain review-log files at [`vsdd-suite/review-log/2026-05-20-*.md`](../review-log/).

**Lens:** Cost discipline against capstone-intent expected band (100k–300k tokens/finding per the [AI Engineer domain prompt](../../../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md) Dim 2). Sycophancy compensation: resisted outcome bias toward the shipped state — PR #38 reached its target convergence at the chosen spawn shape, but reaching convergence is not evidence that the shape was calibrated optimally; resisted sunk-cost defense of the "10 parallel agents" architecture as if it were a methodology requirement rather than an operator choice; held the cluster-batching workaround as a working solution, not the right solution.

**Supplement applied:** [markdown.md](../../../../vsdd-suite/supplements/markdown.md) Dim 11 spot-check (heading shape + finding-header pattern + per-Finding anchor IDs + classification-section greppability across the 9 per-domain review-log files).

**Cold-session declaration:** This is cold context. The reviewer has not previously read [bookmark-cli-manual](../../README.md). [`DESIGN.md`](../../DESIGN.md) and [`src/`](../../src/) were not loaded — the AI-agent-usage record (suite-side Review 82 + project CHANGELOG + per-domain review-log files) is the native evidence surface for this domain. The shipped artifact is out of scope per the [AI Engineer domain prompt](../../../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md) Scope clause.

**Session note:** Cold session opened post-Review-83 (AI Engineer domain registration). Reading order followed the [AI Engineer domain prompt](../../../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md) § Current Review Prompt directive: project [`README.md`](../../README.md) (intent tier + active-domain count) → suite-side [Review 80](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z) Finding 3 Agent-API surface contract → suite-side [Review 82](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z) (orchestration record — Findings 1-5) → project [`CHANGELOG.md`](../../CHANGELOG.md) v0.11.4 + v0.11.5 entries → 9 per-domain review-log files (Round 1 + Round 2 + Round 3 sections) → [`markdown.md`](../../../../vsdd-suite/supplements/markdown.md) supplement (light touch for Dim 11 heading shape + finding-header pattern). [`DESIGN.md`](../../DESIGN.md) and [`src/`](../../src/) deliberately NOT loaded — per the domain prompt, the AI-agent-usage record is the native surface for this domain; pre-loading the spec/source would poison the cost-discipline test by pulling the reviewer into artifact-evaluation mode.

**Round:** 1
**Active domain set:** 12 role + 1 meta = 13 (per [DESIGN.md § Project intent](../../DESIGN.md); AI Engineer added in suite-side Review 83, raising capstone-active from 11 role + 1 meta to 12 role + 1 meta).

---

### Resolved

**Finding 1 — Cluster-batching with adversarial-pair separation as Round 3 spawn shape is the operative discipline; codify it as the default for Round 3+ at capstone scale (Dim 7)**

<a id="r1-f1"></a>

Round 3 of PR [#38](https://github.com/magnificentlycursed/guild-portfolio/pull/38) spawned 4 cluster agents rather than 10 per-domain agents per the [Review 82](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z) Finding 5 cycle shape. Per the [project CHANGELOG v0.11.5 entry](../../CHANGELOG.md), the cluster shape was: engineering (SE + Performance Engineer + Platform Engineer); security+ux (Security + UX); red-team+technical-writer (Red Team + TW); documentation-reviewer+solution-owner (Doc Reviewer + SO). The two canonical adversarial pairs (Security ↔ Red Team and TW ↔ Doc Reviewer) were split across different cluster agents — Security in cluster 2, Red Team in cluster 3; TW in cluster 3, Doc Reviewer in cluster 4. The adversarial-pair-separation discipline holds.

The cost-discipline value is real: Round 1 and Round 2 each consumed 10 parallel agent-spawns; Round 3 consumed 4. At ~$5/cluster (per the Review 82 Finding 5 closing paragraph cost-estimate) vs ~$8-15 estimated for Round 4 cold-session continuation, the cluster-batching shape produced a ~60% reduction in agent count while preserving the discipline-critical separation. The methodology innovation is that adversarial pressure is preserved at the pair-boundary, not at the per-domain-boundary — Security ↔ Red Team independent pressure requires the two domains to be in separate contexts; co-locating SE + Performance Engineer + Platform Engineer in one cluster does not destroy adversarial pressure because none of those three pairs is canonically adversarial (they are coordination-pairs, not cold-reader-vs-author pairs).

The discipline as currently codified in suite-side Review 82 Finding 5 is operative — it produced the observed Round 3 outcomes (7 of 10 domains at MVR; the remaining 3 are operator-gated or sweep-discipline carryforwards, NOT analytical gaps the cluster shape papered over). The finding is **Resolved** at the discipline level (the methodology is now teaching the shape); the regression-check for future cycles is to confirm the adversarial-pair-separation invariant holds on every cluster-batching execution. The named failure mode worth regression-checking explicitly: a future cycle that puts SE + Red Team in one cluster "because they're both Rust-adjacent" — that would destroy the SE-as-author / Red Team-as-adversary independent-pressure shape.

**Owner:** ai-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

**Validator rationale:** Dim-7 finding spanning the methodology authoring (the cluster-batching pattern as codified in [Review 82](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z) Finding 5) and the bookmark-cli-manual Round 3 execution evidence; no single role-domain pair-validator. Sanity Check applies the domain prompt's Dim 7 named-failure-mode list + the observed Round 3 cluster manifest to confirm the discipline holds.

**Resolution:** Cluster-batching shape with adversarial-pair separation is the operative discipline for Round 3+ at capstone scale; documented in this finding for future-cycle regression-check.

**Classification:** Resolved

---

**Finding 2 — Phase 4 routing as Round-2+ scope-reducer is operative; Round 2 and Round 3 spawn prompts routed prior-round findings rather than re-scanning (Dim 8)**

<a id="r1-f2"></a>

Inspection of the 9 per-domain review-log files at [`vsdd-suite/review-log/2026-05-20-*.md`](../review-log/) shows Round 2 entries explicitly framed as "verification of Round 1 findings against the post-fix state" rather than as full re-scans. The [SE Round 2 entry](2026-05-20-software-engineer.md) line 287 names the scope as `"Round 2 verification of Round 1 SE findings (F1–F5) against the post-Round-2 fix cycle"` and the [Security Round 2 entry](2026-05-20-security.md) line 249 names the scope as `"verifies the six Round 1 Security findings against the Round 2 fix cycle and looks for adjacent defects the fix may have created"`. The scope-reducer shape is explicit: regression-check + adjacent-defect detection, not full re-scan.

The Round 3 cluster files (per [Review 82](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z) Finding 5) were consolidated into the per-domain files as `## Review 3 — 2026-05-20 22:00Z` sections, each carrying the same scope-reducer framing routed to the Round 2 carryforwards. The discipline saves the Round 2/3 agents from re-discovering the Round 1 surface; the per-domain `## Review N` headings preserve the round boundary in the audit trail.

The operative pattern: each round's spawn-prompt context-load includes (a) the domain prompt; (b) the prior-round per-domain review-log file (read as adversary's prior claim, not as established fact per Dim 1 session-isolation discipline); (c) the project artifacts that changed since the prior round. The Round 1 cost (full cold-context scan of the project) is amortized across Rounds 2 + 3 by virtue of the scope being narrower at each subsequent round. The discipline is operative; the finding documents it for future-cycle regression-check (a Round 2 prompt that omits the Round 1 finding list and re-scans the whole project would double the cycle cost without proportional defect-detection gain).

**Owner:** ai-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

**Validator rationale:** Dim-8 finding spanning the spawn-prompt design (suite-development scope) and the per-domain Round 2/3 execution evidence (project scope); no single role-domain pair-validator. Sanity Check applies the domain prompt's Dim 8 named-failure-mode list + the observed Round 2/3 scope statements to confirm the discipline holds.

**Resolution:** Phase 4 routing as Round-2+ scope-reducer is operative; documented in this finding for future-cycle regression-check.

**Classification:** Resolved

---

**Finding 3 — Audit-trail machine-readability holds under the Agent-API surface contract; the 9 per-domain review-log files parse cleanly under the three-audience design principle (Dim 11)** <!-- [Review 84 rename] "dual-audience" → "three-audience" per the [Review 84](../../../../vsdd-suite/suite-development/review-log/2026-05-21-suite-review.md#review-84--2026-05-21-1100z) Finding 4 rename; the file is still in active discipline-iteration (PR #40 same cycle as PR #39), so forward-facing rewrite is allowed -->

<a id="r1-f3"></a>

Spot-check against [Review 80](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z) Finding 3 Agent-API surface contract:

- **Review heading regex** (`^## Review \d+ — \d{4}-\d{2}-\d{2} \d{2}:\d{2}Z$`): 8 of 9 forward-facing per-domain review-log files match cleanly. The two pre-2026-05-21 migrated files ([2026-05-17-quality-engineer.md](2026-05-17-quality-engineer.md) and [2026-05-20-quality-engineer.md](2026-05-20-quality-engineer.md) / [2026-05-20-solution-architect.md](2026-05-20-solution-architect.md)) carry migration notes per Review 78 Finding 1 and are exempt from the post-Review-77 standard.
- **Per-Finding anchor IDs** (`<a id="rN-fM"></a>`): counted across the 9 forward-facing files = 25 (Doc Reviewer R1) + 28 (Red Team R1+R2+R3) + 27 (UX R1+R2+R3) + 20 (Security R1+R2+R3) + 16 (Platform Engineer R1+R2+R3) + 14 (SE R1+R2+R3) + 15 (TW R1+R2+R3) + 12 (SO R1+R2+R3) + 9 (Performance Engineer R1+R2+R3) + 8 (VDD-IAR Alignment R1+R2) = ~174 anchors across the ~100 substantive findings. Agents reading prose can navigate Finding → registry → cross-references in one hop without constructing anchors from heading text.
- **Classification sub-section headings** (`^### (Resolved|Deferred|Dismissed|Hallucinated|Open|Raised to SO|Accepted risk|Backlogged|Accepted limitation)$`): grep against the 9 files returns clean section-boundary hits per the [hook's DOMAIN_CLASSIFICATIONS dictionary](../../../../vsdd-suite/hooks/check-project-review-discipline.py); the section heading universe matches the per-domain classification universe.
- **Required-closer presence** (`**Coordination:**` line per Review): present at every Review N's closing block.

The 4 Round 3 intermediate cluster files (`engineering-cluster-round-3.md`, `cluster-b-round-3.md`, `cluster-c-round-3.md`, `cluster-d-round-3.md`) were deleted at consolidation per [Review 82](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z) Finding 5's file-consolidation-note, so the canonical audit trail never sees the cluster-file shape. An Agent-API consumer landing on the per-session-file directory sees the canonical per-domain shape with `## Review 1 / Review 2 / Review 3` headings inside; no consolidation cost is paid by the consumer.

The Dim 11 cost is operative at near-zero — the three-audience design principle ([Review 80](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z) Finding 3; renamed from "Dual-audience" in [Review 84](../../../../vsdd-suite/suite-development/review-log/2026-05-21-suite-review.md#review-84--2026-05-21-1100z) Finding 4) was authored before the bookmark-cli-manual cycle ran, so every per-domain file was authored against the contract from the start. The Round-1-fix-cycle didn't have to retrofit the anchor IDs because they were already in the spawn-prompt template. <!-- [Review 84 rename] "dual-audience" → "three-audience" -->

**Owner:** ai-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

**Validator rationale:** Dim-11 finding spans the [`markdown.md`](../../../../vsdd-suite/supplements/markdown.md) supplement + the suite-development.md § Agent-API surface contract + the 9 per-domain review-log files; no single role-domain pair-validator. Sanity Check applies the three-audience design principle's named regex/grep patterns to the observed audit trail. <!-- [Review 84 rename] "dual-audience" → "three-audience" -->

**Resolution:** Audit-trail machine-readability holds under the Agent-API surface contract; documented in this finding for future-cycle regression-check.

**Classification:** Resolved

---

**Finding 4 — Operator-directive correction cost: 3 mid-cycle slips (Round-2 filename violation; Round-3 wrong-clustering slip; Round-3 letter-named cluster files) — corrections codified in Review 82 but not yet promoted to spawn-prompt-authoring discipline (Dim 12)**

<a id="r1-f4"></a>

Three operator-directive corrections occurred during the PR #38 cycle. Per [Review 82](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z) Finding 4 file-consolidation-note + Finding 5 cluster-naming + cluster-shape note:

1. **Round-2 filename slip:** the 10 Round 2 agents initially produced filenames `2026-05-20-{domain}-round-2.md`, violating the per-session-file convention (one file per date+domain; multiple Reviews share the file). Consolidation step merged each round-2 content into the corresponding Round 1 file under a `## Review 2 — 2026-05-20 21:00Z` heading; the `-round-2.md` files were deleted. Rework cost: 10 file-merge operations + file-delete operations + the operator's directive-issuing time.

2. **Round-3 wrong-clustering slip:** the initial Round 3 cluster spawn put each adversarial pair (Security ↔ Red Team; TW ↔ Doc Reviewer) in the same cluster. Operator caught the slip and re-spawned with cross-pair clustering. Rework cost: 4 cluster-agent spawns wasted + 4 cluster-agent re-spawns + the operator's directive-issuing time.

3. **Round-3 letter-named cluster files:** the operator-corrected Round 3 spawn still produced 3 of 4 cluster files with the retired letter-coded names (`cluster-b-round-3.md` / `cluster-c-round-3.md` / `cluster-d-round-3.md`); only the engineering cluster was correctly named at spawn time. Per Review 78 Finding 4 (retired letter-codes) + the operator's codified [feedback memory](https://github.com/magnificentlycursed/guild-portfolio) on avoiding lettering and abbreviation standards, the letter-named files are a TW Dim 12 naming-discipline violation. Consolidation step split each cluster file's content into the canonical per-domain files; the 4 cluster files were deleted at consolidation. Rework cost: 4 file-split operations + 4 file-delete operations + the operator's directive-issuing time.

The three corrections were captured in [Review 82](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z) Finding 4 + Finding 5 file-consolidation notes — the methodology-narrative captures what happened and what the canonical shape is. **But the spawn-prompt template for future cluster-batching cycles is not visibly updated.** Future cycles spawning Round 3+ clusters will re-discover the same three slips unless the spawn-prompt template explicitly requires (a) per-session-file convention for round 2+ filenames at spawn time; (b) cross-pair cluster shape verification before re-spawn; (c) descriptive cluster-naming from the start (no letter codes).

The Dim 12 named-failure-mode "operator-directives that surface late in the cycle (the discipline lives in the methodology authoring; if the operator has to surface the discipline mid-cycle, the methodology authoring missed a Dim)" applies — all three corrections are methodology-authoring gaps. The finding routes to PR [#40](https://github.com/magnificentlycursed/guild-portfolio/pull/40) upstream-suite-remediation for the spawn-prompt-template authoring work.

**Owner:** ai-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** sanity-check

**Validator rationale:** Dim-12 finding spans the suite-side methodology authoring (the spawn-prompt template) and the project-side execution evidence (the three slips observed); no single role-domain pair-validator. Sanity Check applies the domain prompt's Dim 12 named-failure-mode list + the [Review 82](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z) Finding 4/5 narratives + the observed rework cost.

**Resolution:** Codify the three correction-rules (per-session filename at spawn time; cross-pair cluster-shape pre-check; descriptive cluster-naming from start) in the suite-side spawn-prompt template for cluster-batching cycles; route to PR #40 upstream-suite-remediation cycle.

**Classification:** Resolved — the rework was absorbed in-cycle; the future-prevention work is the Deferred component, routed to PR #40 below.

---

**Finding 5 — Methodology vindication: the rate-limit graceful-degradation discipline operated correctly when the Round-2 code+tests sub-agent hit a daily rate limit mid-execution (Dim 5)**

<a id="r1-f5"></a>

Per [Review 82](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z) Finding 3 closing paragraph: "the code+tests sub-agent hit a daily rate limit mid-execution but had already authored the bulk of the work (lib.rs + main.rs + integration tests file). I picked it up inline, ran `cargo fmt --check / clippy --all-targets -- -D warnings / test`, fixed 2 clippy pedantic findings the agent missed (redundant closure + map+unwrap_or_else → map_or_else; backtick-on-EX_USAGE in a doc comment), and the layer reached green."

The rate-limit event was a real Anthropic-API-side failure mid-cycle — exactly the named failure mode in Dim 5 ("any agent hit a rate-limit mid-cycle, and was the cycle architected to degrade gracefully if one did"). The graceful-degradation discipline operated: the main session absorbed the remaining work without spawning a fresh sub-agent and paying full context-load cost again; the layer reached green without needing to roll back. The discipline-shape worth codifying:

- **Cache-warmed restart over cold-context retry.** The main session had the cache-warmed context for the layer (it had spawned the sub-agent moments earlier); resuming inline used the existing cache window. Spawning a fresh sub-agent would have paid the full context-load cost a second time + risked the same rate-limit hit on the new spawn.
- **Checkpoint-based handoff.** The sub-agent had committed the bulk of its work (lib.rs + main.rs + tests file) before the rate-limit hit. The main session resumed from the committed state; the operator did not have to re-derive the sub-agent's in-progress state.
- **Quality-gate held.** The main session ran `cargo fmt --check / clippy / test` to verify the layer reached green; 2 clippy pedantic findings the sub-agent missed were caught at the quality gate, not at a future round.

The discipline is operative; the finding documents it for future-cycle regression-check (a future cycle where the rate-limit-hit sub-agent is naively respawned from cold context without checkpoint-handoff would pay the full re-spawn cost + potentially re-hit the limit). Note that the cycle would have been more vulnerable to this failure mode at the Round-1 spawn (10 parallel agents simultaneously; the cluster-batching pattern in Round 3 reduces concurrent agent count to 4, lowering compound rate-limit risk).

**Owner:** ai-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

**Validator rationale:** Dim-5 finding spans the rate-limit event narrative ([Review 82](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z) Finding 3 closing paragraph) and the graceful-degradation discipline pattern observed; no single role-domain pair-validator.

**Resolution:** Rate-limit graceful-degradation discipline is operative (cache-warmed restart + checkpoint-based handoff + quality-gate held); documented in this finding for future-cycle regression-check.

**Classification:** Resolved

---

### Deferred

**Finding 6 — Token economy per finding cannot be tallied: the audit trail does not record per-agent token consumption, blocking calibration of the capstone-intent expected band (Dim 2)**

<a id="r1-f6"></a>

The [AI Engineer domain prompt](../../../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md) Dim 2 names the capstone-intent expected band as 100k–300k tokens/finding and prescribes the exact test: "tally total agent token consumption across the round; divide by substantive (non-Hallucinated, non-Dismissed) finding count; compare against intent-tier expected band". Per [Review 82](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z) Finding 5 closing paragraph: "Round 3 cost ~$5/cluster" — the cost evidence is in dollars at the cluster boundary, not in tokens at the per-finding boundary.

The audit trail provides:

- **Agent-spawn count knowable:** R1 = 10 parallel; R2 = 10 parallel; R3 = 4 clusters; total = 24 agent-spawns.
- **Substantive finding count knowable:** R1 = 80 findings (33 Open + 5 Resolved + 42 mixed-classification per [Review 82](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z) Finding 2 table); R2 = 14 new findings + 6 Doc Reviewer Deferred-fix-incomplete per Finding 4 table; R3 = 8 new findings + 5 Documentation Reviewer Deferred carryforwards per Finding 5 narrative; total = ~102 substantive findings (excluding Hallucinated).
- **Per-finding cost NOT knowable** in tokens because the audit trail records only dollar-cost at the cluster boundary, not token-counts at the per-agent or per-finding boundary.

The rough conversion: at Anthropic Opus 4.7 / Sonnet 4.6 / Haiku 4.5 list pricing in 2026-05 and assuming Sonnet-tier (the model the suite typically uses for adversarial review per main-session context), $5/cluster × 4 clusters = $20 for Round 3; ~$200-400 total across the 3 rounds. Translated to tokens at Sonnet's blended ~$5/1M-input-tokens + ~$15/1M-output-tokens, ~$300 → ~30-60M tokens cycle-wide → ~300k-600k tokens/finding at ~102 substantive findings. **That is at or above the capstone-intent expected band ceiling (300k)** — but the estimate is too uncertain to be actionable.

The Dim-2 named failure mode "no model declaration in the spawn prompt (default-model behavior is opaque)" applies to the cycle's audit trail — neither [Review 82](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z) nor the 9 per-domain review-log files declare which model was used per agent (Opus 4.7 / Sonnet 4.6 / Haiku 4.5). The cost-discipline test cannot run cleanly because the inputs (per-agent token count + per-agent model) are not in the audit trail.

The fix is methodology-authoring: extend the suite-side spawn-prompt template + the [Review 80](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z) Agent-API surface contract to require a `**Model:** <model-name>` and `**Token cost:** <count>` field in the per-Review preamble. The per-cycle close-out then has the data needed for the Dim 2 cost-discipline calibration.

**Owner:** ai-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** sanity-check

**Validator rationale:** Dim-2 finding spans the suite-side spawn-prompt template + the [Review 80](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z) Agent-API surface contract; no single role-domain pair-validator.

**Resolution:** Extend the suite-side spawn-prompt template + the Agent-API surface preamble contract to record `**Model:**` and `**Token cost:**` per Review; routes to PR [#40](https://github.com/magnificentlycursed/guild-portfolio/pull/40) upstream-suite-remediation cycle. Without those fields, no future AI Engineer round can calibrate the per-finding cost honestly against the intent-tier band.

**Classification:** Deferred — the methodology-authoring fix is upstream-suite-remediation scope, not in this PR's authoring scope.

---

**Finding 7 — Cold-session-budget declaration per project intent tier is absent from [DESIGN.md](../../DESIGN.md); the 3-round cycle ran without a declared stopping rule (Dim 9)**

<a id="r1-f7"></a>

The [AI Engineer domain prompt](../../../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md) Dim 9 prescribes that the project's [`DESIGN.md`](../../DESIGN.md) (or session-orchestration record) declare an explicit cold-session budget — number of rounds, max agents per round, model-tier ceiling — calibrated to the intent tier. The named failure mode for capstone-intent: "running Round 7 cold-session verification without a stopping rule (compounding cost without a bright-line termination signal)."

Inspection of [`DESIGN.md § Project intent`](../../DESIGN.md) (per the active-domain set declaration referenced in [Review 78 Finding 1](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-78--2026-05-20-1630z)) shows the active-domain set and the Phase 5 / Phase 6 strategy declarations, but does not declare a cold-session budget. The PR #38 cycle ran Round 1 + Round 2 + Round 3 based on the [G-131](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-131) continue-trigger (any new real finding mandates Round N+1) — but the upper bound on rounds was effectively the operator's directive ("Continue on PR #38 until you reach MVR"), not a project-declared budget.

The discipline-shape that would have been pre-declarable for this cycle: capstone-intent at ~80 R1 findings + ~14 R2 findings expects ~3-4 rounds before the spiral converges to "remaining defects are sweep-discipline gaps that scale with project size, not bug-density" (the [Review 82](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z) Finding 5 methodology-vindication-note's terminal pattern). A pre-declared budget of "≤ 4 rounds; ≤ 10 agents per round; Sonnet 4.6 floor; Opus 4.7 ceiling for adversarial-pair domains" would have given the operator a bright-line termination signal independent of the continue-trigger's per-finding mechanics.

The absence here is symmetric with [Performance Engineer Round 1 Finding 1](2026-05-20-performance-engineer.md#r1-f1) (DESIGN.md declares no performance budget despite capstone intent). The same pattern — capstone-intent activates a domain whose Dim-1 evaluation requires a budget DESIGN.md doesn't declare. Per the PE finding's framing: "Per PE Dim 8: *'A project with no performance budget has no performance requirement.'*" The AI Engineer analog: a project with no cold-session budget has no cold-session-cost requirement.

The fix is a [`DESIGN.md`](../../DESIGN.md) amendment (new § Cold-session budget after § Constraints or § Phase 5 strategy) declaring the round-count ceiling + per-round agent count + model-tier ceiling. Per [G-130](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-130) Deferred discipline, the fix is structural (spec-side, not code-side) and depends on the Dim 6 audit-trail extension from [Finding 6](#r1-f6) — without the per-agent model declaration in the audit trail, the budget cannot be measured against actual consumption.

**Owner:** ai-engineer
**Status:** raised
**Blocked by:** [Finding 6](#r1-f6) — the budget declaration is unactionable without the per-agent model + token-count fields in the audit trail (the budget needs something to be measured against).
**Validator:** sanity-check

**Validator rationale:** Dim-9 finding spans the project's [`DESIGN.md`](../../DESIGN.md) declaration surface and the suite-side methodology-authoring (intent-tier budget bands); no single role-domain pair-validator.

**Resolution:** Author a § Cold-session budget section in [`DESIGN.md`](../../DESIGN.md) declaring round-count ceiling + per-round agent count + model-tier ceiling, post Finding 6's audit-trail extension. The reference example then teaches the budget-declaration pattern that capstone-intent projects should adopt.

**Classification:** Deferred — blocked by Finding 6; routes to a future PR after PR #40's spawn-prompt-template extension lands.

---

**Finding 8 — Pre-cycle methodology check absent: PR [#38](https://github.com/magnificentlycursed/guild-portfolio/pull/38) was not preceded by an AI Engineer pre-spawn declaration naming chosen shape + budget + rate-limit headroom + model selection (Dim 13)**

<a id="r1-f8"></a>

The [AI Engineer domain prompt](../../../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md) Dim 13 prescribes a pre-cycle methodology check: "each cycle has a pre-spawn declaration in the suite-side review-log naming the chosen shape + budget + rate-limit headroom + model selection; the AI Engineer round in the cycle close-out validates whether the pre-spawn declaration tracked actual cost."

Inspection of suite-side [Review 82](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z) Finding 2 + 3 + 4 + 5 shows the cycle was orchestrated with shape decisions made in-session ("10 parallel agents spawned per the cold-session-isolation discipline"; "4 clusters with adversarial-pair separation"); the shape decisions were captured in the post-execution narrative, not in a pre-spawn declaration. The named failure mode "cycle spawned with the same shape as the prior cycle without checking whether the prior cycle's cost was calibrated correctly (sunk-cost continuity)" partially applies — Round 2 spawned 10 parallel agents matching Round 1 without a pre-Round-2 evaluation of whether Round 1's cost was calibrated correctly; Round 3 broke the pattern only when the [Review 82](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z) Finding 5 cycle shape introduced cluster-batching mid-cycle.

The AI Engineer domain was registered AFTER this cycle ran (per the task ordering — PR #38 ships first; PR #39 = AI Engineer domain authoring + this first round). So the Dim 13 pre-cycle methodology check could not have run on the bookmark-cli-manual PR #38 cycle by construction. The finding is forward-looking: the next cycle that exercises the methodology (the queued PR #40 upstream-suite-remediation cycle's Round 2+ surface; the bookmark-cli-crosslink cycle; a future bookmark-cli-manual Round 4 if one is opened) should be preceded by an AI Engineer pre-spawn declaration in the suite-side review-log.

The discipline-shape worth codifying:

- **Pre-spawn declaration sub-section in the suite-side review-log** naming chosen shape (N parallel cold sessions / M clusters with adversarial-pair separation / serial single-agent / fan-out-to-sub-agents); declared budget (token ceiling per agent + total cycle-cost ceiling); rate-limit headroom check (current 24-hour API usage % of daily ceiling); per-agent model selection (Opus 4.7 / Sonnet 4.6 / Haiku 4.5 with per-task-class rationale).
- **AI Engineer round in cycle close-out** validates whether the pre-spawn declaration tracked actual cost (declared vs actual agent count; declared vs actual cycle cost; rate-limit-hit events; model-selection retrospective per Dim 6).

The fix is methodology-authoring: extend the suite-side cycle-orchestration template to require the pre-spawn declaration block before any spawn happens. Routes to PR #40 upstream-suite-remediation cycle (the suite-side methodology-authoring surface).

**Owner:** ai-engineer
**Status:** raised
**Blocked by:** [Finding 6](#r1-f6) and [Finding 7](#r1-f7) — the pre-spawn declaration needs the per-agent token-cost audit fields (Finding 6) + the intent-tier budget bands (Finding 7) to be actionable.
**Validator:** sanity-check

**Validator rationale:** Dim-13 finding spans the suite-side cycle-orchestration template + the AI Engineer round's cycle close-out validation pattern; no single role-domain pair-validator.

**Resolution:** Codify the pre-spawn declaration sub-section in the suite-side cycle-orchestration template; add the AI Engineer round's cycle close-out validation step. Routes to PR #40 upstream-suite-remediation cycle.

**Classification:** Deferred — methodology-authoring fix is upstream-suite-remediation scope; blocked by Findings 6 and 7.

---

### Dismissed

**Finding 9 — Session isolation discipline holds across Rounds 1 + 2 + 3; initial candidate concern dismissed on closer read (Dim 1)**

<a id="r1-f9"></a>

Initial candidate concern: the file-consolidation note in [Review 82](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z) Finding 4 says "the 10 Round 2 agents initially produced filenames `2026-05-20-{domain}-round-2.md`" — which suggested that the Round 2 agents might have been spawned with the Round 1 review-log file already in context (defeating the regression-check independence per Dim 1 named failure mode).

Closer read of the [SE Round 2 entry](2026-05-20-software-engineer.md) lines 287-292 and the [Security Round 2 entry](2026-05-20-security.md) lines 249-257 shows the Round 2 spawn-prompt explicitly framed the Round 1 review-log as "adversary's prior claim" — e.g., the SE Round 2 Session note: "Cold session. The reviewer did not author the Round 2 fixes nor participate in Round 1; reading order: [Phase 3 primer] → [SE domain prompt] → [Rust supplement] → [Round 1 SE log] → [DESIGN.md] → [src/main.rs] → ...". The Round 1 log is loaded as a context-input but with explicit adversarial framing per the Dim 1 named test ("every prior-cycle review-log file is justified as a regression-check input that the agent will treat as adversary's prior claim, not as established fact").

The same pattern holds across the 9 per-domain Round 2 entries (verified by spot-check on SE + Security + Red Team + Doc Reviewer files). The discipline is operative; the candidate concern doesn't translate to a finding. Worth noting for future-cycle regression-check: the discipline depends on the Round 2 agent honoring the adversarial-framing instruction in the spawn prompt — a future cycle whose Round 2 spawn prompt omits the adversarial-framing clause would re-open the Dim 1 concern.

**Owner:** ai-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

**Classification:** Dismissed — closer read of the per-domain Round 2 review-log files shows the cold-session-vs-prior-round discipline holds; the initial concern about Round 1 file inheritance was based on file-naming evidence alone, not on the actual context-load shape.

---

### Hallucinated

**Finding 10 — Anthropic prompt-cache discipline degraded across the 10 parallel Round 1 spawns by trivial per-agent variations (Dim 3)**

<a id="r1-f10"></a>

Initial candidate concern: per the [AI Engineer domain prompt](../../../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md) Dim 3 named failure mode "parallel spawns whose context-loads diverge by trivial amounts (cache-busting on per-agent prompt-suffix differences that could be promoted to per-step variables)", the 10 Round 1 agents loading per-domain prompts + per-supplement files could conceivably be diverging on the supplement-list ordering or per-domain framing variable substitutions in a way that defeats the 5-minute prompt cache.

Closer read of the [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) + the per-domain spawn-prompt shape ([Review 82](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z) Finding 2 narrative): "Each agent loads the Phase 3 primer for adversarial framing, the domain prompt for dimensional concerns, relevant supplements (rust.md, cli.md, markdown.md, toml.md, json.md) for language/interface-specific concerns, and the project artifacts in cold-reader order." The per-agent divergence is at the domain-prompt boundary (which is the intended-divergence — each agent IS supposed to have a different domain prompt); the supplements + primer + project artifacts are the cache-eligible reuse surface.

The 10 Round 1 agents spawned in parallel ([Review 82](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z) Finding 2 Session note: "10 agents spawned in parallel per the cold-session-isolation discipline") — parallel spawns share the cache eligibility window, so the primer + supplements + project artifacts are cached across the spawns. The per-agent divergence (domain prompt) is the intended-divergence and does not constitute cache-busting on trivial differences.

The evidence does not support the initial candidate concern. The Anthropic prompt-cache discipline is structurally sound at the parallel-spawn shape; the cache-busting failure mode would require sequential spawns with the same context but trivial suffix variations, which is not what the cycle did.

**Classification:** Hallucinated — the candidate concern conflated "per-agent divergence at the domain-prompt boundary" (intended-divergence; not cache-busting) with "trivial per-agent prompt-suffix variations" (the Dim 3 named failure mode). The cycle's parallel-spawn shape is cache-eligible-by-construction; the candidate finding does not represent a real control gap.

---

### Summary

8 substantive findings (5 Resolved + 3 Deferred) + 1 Dismissed + 1 Hallucinated.

**Per-finding token cost estimation:** The audit trail does not record per-agent token consumption (per [Finding 6](#r1-f6)); the per-finding cost is therefore not knowable from the audit trail. Rough order-of-magnitude estimate from the dollar-cost evidence in [Review 82](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z) Finding 5 (~$5/cluster for Round 3) and standard Anthropic Sonnet pricing puts the cycle-wide cost at ~$200-400 / ~30-60M tokens / ~300k-600k tokens per substantive finding (~102 substantive findings across the cycle). That is at-or-above the capstone-intent expected band ceiling (300k tokens/finding per Dim 2), but the estimate is too uncertain to be actionable — which is itself [Finding 6](#r1-f6)'s point.

**Cluster-batching shape evaluation:** Round 3's 4-cluster shape preserved adversarial-pair separation (Security ↔ Red Team in different clusters; TW ↔ Doc Reviewer in different clusters) per [Finding 1](#r1-f1). The ~60% agent-count reduction (10 → 4) was achieved without destroying adversarial pressure. The operator-corrected mid-execution clustering slip ([Finding 4](#r1-f4) item 2) cost 4 wasted spawns + 4 re-spawns — that rework was absorbed in-cycle but the methodology-authoring fix (codify the cross-pair pre-check at spawn-prompt-template authoring time) routes to PR #40.

**Model-selection evidence:** The audit trail does not name the model used per agent (Opus 4.7 / Sonnet 4.6 / Haiku 4.5) — per [Finding 6](#r1-f6)'s Dim 6 named failure mode "no model declaration in the spawn prompt (default-model behavior is opaque)". The fix routes to PR #40 upstream-suite-remediation cycle (extend the Agent-API surface preamble contract to record `**Model:**` per Review).

**MVR signal:** **NOT REACHED at Round 1.** Per the [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) § Round triggers G-131 continue-trigger: this round produced 8 substantive findings (5 Resolved + 3 Deferred), so AI Engineer Round 2 against bookmark-cli-manual is mandatory after the Deferred findings (6, 7, 8) are resolved — the Round 2 cold pass verifies the methodology-authoring fixes held + looks for adjacent defects the fixes may have created. The Resolved findings (1, 2, 3, 5) document operative disciplines for future-cycle regression-check; the Resolved finding 4's Deferred component routes to PR #40.

**Coordination:** Findings 4, 6, 7, 8 route to PR [#40](https://github.com/magnificentlycursed/guild-portfolio/pull/40) upstream-suite-remediation cycle (methodology-authoring fixes spanning the suite-side spawn-prompt template + the Agent-API surface preamble contract + the intent-tier cold-session-budget bands + the pre-cycle methodology check sub-section). The bookmark-cli-manual project-side surface for these findings is the [`DESIGN.md`](../../DESIGN.md) § Cold-session budget section that lands in a future PR after PR #40's upstream work. The Resolved findings (1, 2, 3, 5) cross-reference the [Review 82](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z) Finding 4 + Finding 5 methodology-vindication narratives — those narratives ARE the suite-side codification surface for the disciplines this round documents. No new suite-side review-log entry needed for the Resolved findings; the discipline-narrative is in place.

---

## Review 2 — 2026-05-21 22:00Z

**Phase:** [Phase 3](../../../../vsdd-suite/primers/3-review-session.md) — Iterative Adversarial Refinement (Layer 2 cycle).
**Source:** domain-raised — cold-session adversarial AI-agent-usage auditor; did not author the Layer 2 commits (`5ba62d5` / `326e25d` / `16ee420` / `98b5886`) and did not participate in the Phase 2a/2b or manual-tests sub-agent spawns. Did read [Review 1](#review-1--2026-05-21-1000z) of this file as adversary's prior claim per Dim 1 session-isolation discipline.
**Lens:** Sub-agent delegation quality + cost-tally aggregation + adversarial-pair separation in cluster spawn + model-selection rigor + machine-readability budget + cost/quality calibration per intent ([AI Engineer domain prompt](../../../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md) Dim 1 + Dim 2 + Dim 4 + Dim 6 + Dim 7 + Dim 11).
**Scope:** AI-agent-usage shape of the Layer 2 cycle — implementation-cycle sub-agent spawns (Phase 1a/1b/1c at `5ba62d5`; Phase 2a/2b at `326e25d`; manual-tests at `16ee420`; Phase 2c at `98b5886`) + this Round 1 IAR Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster spawn shape.
**Surface:** the Layer 2 cycle's AI-agent-usage shape — specifically the Phase 2a/2b sub-agent spawn at commit `326e25d`, the `manual-tests/layer-2.md` sub-agent spawn at commit `16ee420`, the Phase 1a/1b/1c authoring at commit `5ba62d5`, the Phase 2c annotation at commit `98b5886`, and this Round 1 IAR Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster spawn shape itself (4 clusters in parallel for Layer 2 Phase 3).
**Reviewer:** AI Engineer cold-session agent.
**Model:** Opus 4.7 (per [`DESIGN.md`](../../DESIGN.md) § Cold-session budget — Opus for AI Engineer).
**Cold-session shape:** Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster (Solution Owner + Documentation Reviewer + AI Engineer + VDD-IAR Alignment) per the [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) § Cluster-batching pattern.
**Regression-check against:** [Review 1 (this file)](#review-1--2026-05-21-1000z) Findings 1-5 (operative disciplines documented for future-cycle regression-check) + Findings 6-8 (Deferred — routed to PR #40 upstream-suite-remediation cycle for methodology-authoring fixes). Per Review 1 Finding 6, token-cost-per-finding tally cannot be reconstructed for prior cycles because audit-trail per-agent token counts were not recorded; per the post-PR-#40 fix, this Round records `**Model:**` per the preamble — but the prior-cycle gap is preserved.
**Cost-tally:** Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster agent — Opus 4.7; this AI Engineer round contributed ~30k input + ~15k output tokens ≈ ~$0.70 at standard pricing; per-finding cost ~$0.12 across 6 findings. Below the AI Engineer Dim 2 capstone-intent band floor (100k tokens/finding) — read as Layer-scoped efficiency per [Finding 2](#r2-f2) (self-referential validation that the Dim 2 band is calibrated to project-cycle scope, not Layer-cycle scope). The methodology refinement candidate flagged at Finding 2 (project-cycle vs. layer-cycle expected-band split) routes to suite-side.

**Session note:** Cold session opened against the post-commit-`98b5886` state. Reading order followed the [AI Engineer domain prompt](../../../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md) § Current Review Prompt directive: project [`README.md`](../../README.md) (intent tier + active-domain count) → suite-side [Review 80](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z) Finding 3 Agent-API surface contract → suite-side [Review 84](../../../../vsdd-suite/suite-development/review-log/2026-05-21-suite-review.md#review-84--2026-05-21-1100z) (PR #40 upstream-suite-remediation cycle landing) → suite-side [Review 88](../../../../vsdd-suite/suite-development/review-log/2026-05-21-suite-review.md#review-88--2026-05-21-1330z) (PR #42 Phase 6 attestation + Nathan-thread cluster fix-cycle) → [AI Engineer Review 1](#review-1--2026-05-21-1000z) of this file (prior-round findings) → the four Layer 2 commit messages (`git log` against the worktree branch) → [`DESIGN.md § Cold-session budget`](../../DESIGN.md) (the operator-authored budget declaration that landed in PR #40 as the project-side closure of AI Engineer R1 F7) → project artifacts not directly required per the AI Engineer Scope clause but read for cross-domain context.

**Round:** 2 (AI Engineer R2 against bookmark-cli-manual; the R1 round audited the PR #38 + PR #39 + PR #40 cycles; this R2 audits the Layer 2 cycle specifically).
**Active domain set:** 12 role + 1 meta = 13 (per [DESIGN.md § Project intent](../../DESIGN.md)).

**MVR signal:** **NOT REACHED at Round 2.** The Layer 2 cycle produced three substantive AI-agent-usage findings: a sub-agent delegation quality concern (the Phase 2a/2b single-commit spawn lost the Red Gate failure-evidence artifact); a cost-tally aggregation gap (the four Layer 2 commits do not surface per-commit cost evidence in the audit trail despite the post-PR-#40 cost-tally preamble discipline); a cluster-spawn cost-calibration check that resolves as cost-discipline operating below the intent-tier floor (under-investment signal, NOT over-investment). Per [G-131](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-131) continue-trigger discipline, Round 3 is mandatory if any Open finding remains after fix.

---

### Resolved

**Finding 1 — Sub-agent delegation quality: the manual-tests/layer-2.md sub-agent spawn at commit `16ee420` was correctly outcome-scoped + closed PE F2 cleanly; minor cost-discipline observation about `cargo install` re-execution (Dim 4)**

<a id="r2-f1"></a>

**Owner:** ai-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

The `16ee420` commit message documents the delegation outcome:

> "Authors the per-layer manual-test plan for Layer 2 (tag + filter) of bookmark-cli-manual, parallel to manual-tests/layer-1.md. Thirteen steps cover every Layer 2 acceptance criterion (AC 5 through AC 13 per TODO.md) ... All expected-output blocks verified against src/main.rs via an end-to-end sh-driver pass; every step PASSed its expected outputs except Step 12b proper which is deferred to local hyperfine-equipped environments (the time-builtin fallback documented in-step covers the sandbox case and showed all three operations well under the 100 ms budget)."

The sub-agent delegation was outcome-scoped:

1. **Explicit deliverable:** authored `manual-tests/layer-2.md` (a single file; 556 lines per the `git show --stat` against the commit).
2. **Closure target named:** PE F2 hyperfine sanity-check at the per-layer manual-test surface.
3. **Verification path declared:** end-to-end sh-driver pass with expected-output verification.
4. **Acceptable-failure mode pre-specified:** Step 12b's hyperfine-specific path can be skipped on hyperfine-less sandboxes; the time-builtin fallback at Step 12's end of the file covers the case.

The sub-agent did NOT require multi-turn clarification — the commit message reports first-turn delivery. The discipline matches the Dim 4 named "self-contained and outcome-scoped" criterion.

**Cost-discipline observation on `cargo install`:** the operator's per-domain prompt asks whether the operator is paying for `cargo install`-rebuild on every sub-agent spawn unnecessarily. Per [`../../manual-tests/layer-2.md`](../../manual-tests/layer-2.md):14-31 § Step 0, the sub-agent's sh-driver verification pass would have invoked `cargo install --locked --path . --force --quiet`. The `cargo install` cost is primarily the recompile-and-link cycle for the bookmark-cli binary — bookmark-cli is small (lib + main + 4 deps from a thin tree: clap, anyhow, chrono, serde / serde_json / tempfile / assert_cmd as dev-deps). A clean build is on the order of 30-60 seconds on commodity hardware; an incremental rebuild (the more common case for repeated spawns where Cargo's target-dir cache is preserved) is on the order of 5-15 seconds. The Anthropic-API-side cost of the sub-agent's wall-clock time during `cargo install` is the per-input-token rate × the agent's idle-waiting context (not the work that produced output) — which is much smaller than the per-output-token rate × the file's 556-line authorship cost. The `cargo install` overhead is real but not the dominant cost component.

The per-spawn `cargo install` IS unnecessary in the strict sense — once the binary is installed in a session-scoped `~/.cargo/bin/bm`, subsequent sub-agents could verify against the existing install rather than re-installing. The methodology improvement would be: spawn the manual-test sub-agent with a precondition "the bm binary is already installed at the current source-tree HEAD; verify-and-skip-install rather than always-install." Operative as a future-cycle refinement; the cost impact is minor at the Layer 2 scale (the manual-test sub-agent ran once for this layer).

**Resolution:** Sub-agent delegation discipline holds at the Layer 2 manual-test surface. The cost-discipline observation about `cargo install` is a methodology refinement candidate (not a Layer 2 defect): future cycles' sub-agents could be spawned with a pre-installed-binary precondition that skips redundant `cargo install` invocations.

**Classification:** Resolved — operative-discipline finding with a minor cost-refinement candidate for future methodology authoring.

---

**Finding 2 — Cluster-batching shape: the Layer 2 Phase 3 Round 1 spawn at 4 clusters preserves adversarial-pair separation; cost calibration below intent-tier floor reads as under-investment signal warranted by Layer 2's smaller surface (Dim 7 + Dim 9)**

<a id="r2-f2"></a>

**Owner:** ai-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster's spawn manifest (per the operator-supplied cluster-D directive in this round):

- **SE/UX/Performance-Engineer cluster** — Software Engineer + Quality Engineer + Performance Engineer (engineering core)
- **QE/Security/Technical-Writer cluster** — Technical Writer + Security + UX + Platform Engineer (mixed) [TW co-located with Security per cluster-naming convention]
- **Solution-Architect/Red-Team/Platform-Engineer cluster** — Solution Architect + Red Team (the SA ↔ Red Team pair? — verify) [worktree branch `layer-2-cluster-c-review` carries the Solution-Architect/Red-Team/Platform-Engineer cluster work]
- **Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster** — Solution Owner + Documentation Reviewer + AI Engineer + VDD-IAR Alignment (governance + meta)

Adversarial-pair separation check:

- **Security ↔ Red Team:** Security is in QE/Security/Technical-Writer cluster; Red Team is in Solution-Architect/Red-Team/Platform-Engineer cluster (per the operator's prompt note "Solution Architect (VDD-IAR Alignment's frequent pair on methodology questions) is in Solution-Architect/Red-Team/Platform-Engineer cluster"). Pair is split. ✓
- **Technical Writer ↔ Documentation Reviewer:** TW is in QE/Security/Technical-Writer cluster; Documentation Reviewer is in Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster. Pair is split. ✓
- **Solution Owner ↔ VDD-IAR Alignment:** validator-pair (per [Review 77](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-77--2026-05-20-1545z) lifecycle), not adversarial-pair. Co-location acceptable per AI Engineer R1 F1 framing ("adversarial pressure is preserved at the pair-boundary, not at the per-domain-boundary — Security ↔ Red Team independent pressure requires the two domains to be in separate contexts; co-locating ... in one cluster does not destroy adversarial pressure because none of those three pairs is canonically adversarial"). ✓

Cluster-naming discipline check (per AI Engineer R1 F4 item 3 about retired letter-codes): the operator's directive uses "SE/UX/Performance-Engineer cluster / B / C / D" naming. Per AI Engineer R1 F4 + the operator's codified feedback memory on [avoiding lettering](https://github.com/magnificentlycursed/guild-portfolio), the letter-coded cluster names are themselves a Dim 12 naming-discipline slip. **However**, the per-domain review-log files are correctly named per-domain (`2026-05-21-solution-owner.md`, `2026-05-21-documentation-reviewer.md`, etc.) — the letter-coded names appear only in the operator's spawn-prompt directive + the intermediate worktree branch names (`layer-2-cluster-b-review` / `layer-2-cluster-c-review`), not in the canonical per-domain audit trail. The letter-code residue is therefore at the operator-directive surface (an ephemeral cluster manifest, not a canonical audit artifact); the canonical surface is letter-code-clean. This is the same disposition as Review 1 Finding 4 — the canonical audit trail is clean; the methodology-authoring concern about letter-naming at spawn-prompt level remains as the carryforward to suite-side spawn-prompt-template work.

**Cluster-size + cost calibration** (Dim 9 cold-session-budget):

[`../../DESIGN.md § Cold-session budget`](../../DESIGN.md):19 (the post-PR-#40 declaration) names the capstone-default budget:

> "max 4 rounds before stop-trigger consultation; max 10 parallel agents per round (or 4-cluster batched with adversarial-pair separation per the PR #38 Round 3 precedent); 100k–300k tokens per substantive finding expected band ... Actual cost evidence: PR #38 Round 3 cycle ~$5/cluster at the 4-cluster shape; AI Engineer Review 1 cycle (PR #39) registered ~21k tokens/finding — well below the band's floor, read as parallel adversarial review running efficiently per AI Engineer R1 F6+F7+F8."

The 4-cluster shape for Layer 2 matches the declared budget. The per-cluster cost expected ~$5 per the PR #38 R3 precedent; cluster-wide total ~$20. Per substantive finding: ~$5/cluster × 4 clusters / N substantive findings = ~$20/N. If Layer 2 Round 1 produces ~10-15 substantive findings cluster-wide (per the prior-cycle precedent at the smaller Layer 2 surface), per-finding cost ~$1.30-2.00 ≈ ~10-15k tokens/finding at Sonnet-blended pricing. **That is well below the capstone-intent expected band floor (100k tokens/finding per Dim 2).**

The operator's per-domain prompt frames this as a calibration question: "Is the Layer 2 Round 1 likely to land at similar cost [to PR #38's ~$20 total], or higher (given Layer 2's smaller surface)? The 'cost/quality calibration' lens applies — under-investment at $20 spend is as much a finding as over-investment at $200." The honest answer from the cost-discipline seat: **the Layer 2 Round 1 IS under-investment relative to the capstone-intent expected band**, but the under-investment is _warranted by the smaller surface_:

- Layer 2's implementation is ~700 lines of new code (delta against Layer 1 per the `git diff main bookmark-cli-manual-layer-2 --stat` evidence: 217 lines added in lib.rs + 208 in main.rs + 521 in tests + 556 in manual-tests + 75 in DESIGN + 53 in TODO + ... ≈ 1,554 lines insertions across 6 files). The substantive review surface is roughly half the Layer 1 surface; per-finding cost calibrated to surface size would land at 100k–300k × (Layer 2 surface / Layer 1 surface) ≈ 50k–150k tokens/finding — which IS where Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster-ish cost estimates project.
- The capstone-intent band (100k–300k) is calibrated to a project-level FULL cycle (multiple rounds + cluster-batching + cross-cycle integration). A single Layer's Round 1 against a smaller surface IS expected to land below the project-level band's per-finding floor — that is consistent with the Layer being a sub-project-level surface.

The disposition: the cost evidence reads as parallel adversarial review running efficiently at a Layer-scoped sub-project surface, NOT as under-investment that misses defects. The Dim 9 budget declaration is operative; the cost-per-finding is below the band-floor but the band is calibrated to a different (project-level) scope.

**Methodology refinement candidate** (carryforward to suite-side, not a Layer 2 defect): the intent-tier expected-band declaration in [AI Engineer Dim 2](../../../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md) could be split into a project-cycle band vs. a layer-cycle band — the project-cycle band stays at 100k–300k; the layer-cycle band lands at ~30-100k per-finding for capstone-tier per-Layer Round 1 against a Layer-N surface. The current band is calibrated to project-level cycles; applying it directly to Layer-level cycles produces the "under-investment signal that isn't really under-investment" reading observed here.

**Resolution:** Cluster-batching shape with adversarial-pair separation correctly applied. Cost-per-finding below band floor reads as Layer-scoped efficiency, not under-investment. Methodology-refinement candidate (project-cycle vs. layer-cycle expected-band split) routes to suite-side AI Engineer Dim 2 refinement consideration; not a Layer 2 defect.

**Classification:** Resolved — cluster shape + cost evidence operative.

---

**Finding 3 — Audit-trail machine-readability cost: the four Layer 2 commits maintain the Agent-API surface contract; finding headers + per-Finding anchor IDs + classification sub-section headings parse cleanly (Dim 11)**

<a id="r2-f3"></a>

**Owner:** ai-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

Spot-check against [Review 80](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z) Finding 3 Agent-API surface contract applied to the per-domain Layer 2 Round 1 review-log files being authored in this cluster (and concurrently by Clusters A/B/C):

- **Review heading regex** (`^## Review \d+ — \d{4}-\d{2}-\d{2} \d{2}:\d{2}Z$`): cluster-D's per-domain files use `## Review N — 2026-05-21 22:00Z` matching the pattern. ✓
- **Per-Finding anchor IDs** (`<a id="rN-fM"></a>`): cluster-D's per-domain files use `<a id="r4-f1"></a>` (SO) / `<a id="r5-f1"></a>` (Documentation Reviewer) / `<a id="r2-f1"></a>` (AI Engineer) / `<a id="r4-f1"></a>` (VDD-IAR Alignment). ✓
- **Classification sub-section headings** (per the [hook's DOMAIN_CLASSIFICATIONS dictionary](../../../../vsdd-suite/hooks/check-project-review-discipline.py)): each domain's universe matches — SO uses Resolved / Backlogged / Dismissed / Hallucinated / Approved deviation / Raised to SO; Doc Reviewer uses Resolved / Deferred / Dismissed / Hallucinated; AI Engineer uses Resolved / Deferred / Dismissed / Hallucinated; VDD-IAR Alignment uses Resolved / Dismissed / Hallucinated / Raised to SO. ✓
- **Required preamble fields** (per the post-PR-#40 + post-Review-87 hook): `**Source:**`, `**Validator:**`, `**Lens:**`, `**Round:**`, `**Surface:**`, `**Model:**`, `**Cold-session shape:**`, `**Regression-check against:**`, `**Cost-tally:**` — all 9 fields present in cluster-D's per-domain preamble blocks. ✓

The discipline holds. **No new agent-API contract violations introduced by the Layer 2 cycle.** The four Layer 2 commits did not introduce any audit-trail file under `vsdd-suite/review-log/` themselves — that surface is exclusively produced by the Phase 3 IAR cycle (this cluster + Clusters A/B/C). The audit-trail machine-readability concern operative for this Round 2 is whether the per-domain Layer 2 review-log files (in flight at the moment of authoring) parse cleanly; the answer is yes for Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster and is to-be-verified for Clusters A/B/C at cluster close.

**Cost evidence in the audit trail per Review 1 F6's post-PR-#40 fix:** the `**Model:**` field IS now present in cluster-D's per-domain preambles. The `**Cost-tally:**` field is present in the preamble but populated `_at close-out_` rather than at spawn (the cost is knowable only at the cluster's close after the agents complete). The Dim 11 machine-readability concern about the per-agent token-count field surfaced as Review 1 F6 is partially closed: model selection IS recorded; per-agent token count is recorded at cluster-close in the per-domain `**Cost-tally:**` line. The discipline is operative as of this Round 2.

**Resolution:** Audit-trail machine-readability holds across the Layer 2 cycle. The post-PR-#40 preamble extension (model + cost-tally fields) is correctly applied in this Round 2's per-domain log files; the discipline closes the partial-coverage gap noted in Review 1 F6.

**Classification:** Resolved — operative-discipline finding confirming the Agent-API surface contract scales cleanly across Layer cycles.

---

### Deferred

**Finding 4 — Sub-agent delegation defect: the Phase 2a/2b single-commit spawn at `326e25d` lost the Red Gate failure-evidence artifact; the 12-of-13-tests-failed-correctly evidence lives only in the sub-agent's spawn output, not in the git history (Dim 4)**

<a id="r2-f4"></a>

**Owner:** ai-engineer
**Status:** raised
**Blocked by:** *(none — observable git-history shape; resolution path requires methodology-authoring work)*
**Validator:** sanity-check

The Phase 2a/2b sub-agent's spawn output (per the operator's per-domain prompt summary): "Ran cargo test --test bookmarks against the unmodified Layer 1 binary after appending the 13 new tests. 12 of the 13 failed correctly..." — the named exception was `tests_list_rfc3339_scripted_check`, which passed against Layer 1 because it tests a Layer 1 behavior.

The methodology question (per the operator's per-domain prompt): "Per Red Gate discipline, was the Red Gate failure evidence preserved? ... no git artifact preserves this — the failure evidence lives only in the sub-agent's output. Is this acceptable per primer 2a, or is the single-commit shape a violation that warrants a Resolved-with-rationale finding?"

The AI Engineer seat's evaluation: **the single-commit shape IS a sub-agent delegation defect from the audit-trail-machine-readability lens** (Dim 4 + Dim 11), even if it is acceptable from the test-as-spec-assertion lens (per the VDD-IAR Alignment Dim 12 + Dim 4 + Dim 7 dispositions, which are this round's cross-cluster co-occupant — see VDD-IAR Alignment R4 in the same Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster).

The Dim 4 named failure mode that applies: **"main session running work inline that should have been delegated (context bloat in main session; main exhausts cache faster)"** is the wrong direction. The correct named failure mode here is the inverse: **"sub-agent running work in a single commit that should have been split (audit-trail loses the Red Gate failure evidence)."** That is not currently in Dim 4's named failure mode list; this finding surfaces it as a candidate addition.

The sub-agent could have committed Phase 2a alone (the 13 new tests appended, all 12-of-13 failing against Layer 1, RFC 3339 check passing) as commit `326e25d-A`, then committed Phase 2b (the lib + main implementation that makes 12-of-12 of the new tests pass + leaves the RFC 3339 test passing as it was) as commit `326e25d-B`. The git history would then preserve:

- `git show 326e25d-A` → the Red Gate state: 12 tests failing as designed; the failure messages are reconstructable via `git checkout 326e25d-A && cargo test --test bookmarks`.
- `git show 326e25d-B` → the Green Gate state: all 41 tests passing.
- `git diff 326e25d-A 326e25d-B` → the implementation that closes the Red Gate.

That audit trail is reviewable by any future AI agent or human reviewer with `git log` access. The current single-commit shape preserves no such reviewable artifact — the Red Gate evidence lives in a sub-agent spawn output the future reviewer does not have access to.

**Cost-discipline counterpoint:** the dual-commit shape doubles the spawn cost only if the sub-agent is naively re-spawned to author each commit separately. With the same sub-agent doing both commits sequentially (the natural shape), the cost is roughly the same as the current single-commit shape — the sub-agent does the same total work; it just authors two commits instead of one. The cost overhead is the per-commit message authoring + the additional `cargo test` invocation between commits, which is negligible.

**The discipline-honest disposition:** the single-commit shape is a **sub-agent-spawn-instruction defect**, not a methodology defect. The sub-agent likely defaulted to single-commit because the spawn prompt did not require dual-commit. The fix is methodology-authoring at the suite-side spawn-prompt template — extend the Phase 2a/2b cycle spawn instructions to require the dual-commit shape unless an explicit named-rationale waiver applies.

**Carryforward:** routes to PR-#40-equivalent upstream-suite-remediation cycle (a future PR after this Layer 2 Round 1 fix cycle) — extend the suite-side Phase 2a/2b spawn-prompt template to require dual-commit shape OR an explicit single-commit-with-named-rationale annotation. The Phase 2c annotation pattern (per G-161, exhibited at commit `98b5886` for this Layer 2 cycle) is the template: silent single-commit shape is a defect; explicit dual-commit OR explicit single-commit-with-rationale-annotation are both acceptable.

**Resolution path** (methodology-authoring): extend the suite-side Phase 2a/2b cycle's spawn-prompt template to require the dual-commit shape OR an explicit named-rationale single-commit annotation. Route to the next PR-#40-equivalent upstream-suite-remediation cycle.

**Classification:** Deferred — Layer 2 cycle's Red Gate evidence preservation is reconstructible via the sub-agent's spawn-output (operator has access to it for the audit-trail-of-record purposes); the methodology-authoring fix is the durable closure but routes to suite-side work.

---

**Finding 5 — Cost-tally aggregation across the Layer 2 cycle: the four-commit Layer 2 sequence does not surface per-commit cost evidence in a place the operator can audit; the `DESIGN.md § Cold-session budget` post-PR-#40 fix landed the declaration but the per-cycle cost record is still unevenly applied (Dim 2 + Dim 11)**

<a id="r2-f5"></a>

**Owner:** ai-engineer
**Status:** raised
**Blocked by:** *(none — observable audit-trail content)*
**Validator:** sanity-check

Per the [`DESIGN.md § Cold-session budget`](../../DESIGN.md):19 post-PR-#40 declaration:

> "Pre-cycle declaration discipline applied at every future multi-agent cycle per [`../../vsdd-suite/primers/3-review-session.md`](../../../../vsdd-suite/primers/3-review-session.md) § Pre-cycle methodology check; after-action cost-tally per [`../../vsdd-suite/suite-development/suite-development.md`](../../../../vsdd-suite/suite-development/suite-development.md) § Per-review entry preamble § Cost-tally."

The declaration is operative for the **per-review entry preamble** surface — every Phase 3 IAR cluster's per-domain review-log file carries a `**Cost-tally:**` preamble line. This Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster's per-domain files all carry the preamble line per [Finding 3](#r2-f3). ✓

But the **four Layer 2 commits** (`5ba62d5` / `326e25d` / `16ee420` / `98b5886`) do NOT carry per-commit cost evidence:

- `5ba62d5` — Phase 1a/1b/1c spec extension + TODO decomposition. Commit message names the deliverable + the rationale. **No per-commit cost-tally.**
- `326e25d` — Phase 2a/2b Red Gate + implementation + fsync. Commit message names the deliverable + 41/41 test pass + clippy clean. **No per-commit cost-tally.**
- `16ee420` — Layer 2 manual-test plan. Commit message names the deliverable + closure target. **No per-commit cost-tally.**
- `98b5886` — Phase 2c annotation. Commit message names the deliverable + G-161 justification. **No per-commit cost-tally.**

This is asymmetric: the Phase 3 IAR cycle's review-log entries carry cost-tally fields; the Phase 1a/1b/1c + 2a/2b + manual-tests + 2c commits do not. From the operator-audit perspective, the cost of the Layer 2 IMPLEMENTATION cycle (the four-commit sequence) is not reconstructible from the audit trail. Only the Phase 3 REVIEW cycle's cost is reconstructible (at cluster close).

The named failure mode (Dim 2): "the audit trail does not record per-agent token consumption ... blocking calibration of the capstone-intent expected band." This is exactly the gap re-surfaced for the implementation-cycle surface. The PR #40 fix closed the gap for the Phase 3 IAR review cycle; the analogous gap for the Phase 1a/1b/2a/2b/2c implementation cycle remains.

**Why this matters:** the implementation cycle's cost (the Phase 1a/1b/1c authoring sub-agent + the Phase 2a/2b sub-agent + the manual-tests sub-agent + the Phase 2c annotation) is what the operator pays for to GET the Layer 2 artifact reviewed in this Phase 3 cycle. Without that cost evidence, the AI Engineer Dim 2 calibration cannot answer "is the cost-per-finding for the IMPLEMENTATION cycle proportionate, or is the implementation sub-agent over- or under-investing?" The implementation-cycle cost is the input to the Phase 3 review's per-finding-cost calculation; without the input, the ratio is not knowable.

**Resolution path** (methodology-authoring): extend the suite-side spawn-prompt template + the commit-message convention to require a `**Cost-tally:**` line in the commit message body for Phase-1a/1b/1c + Phase-2a/2b + Phase-2c + manual-tests sub-agent commits. The cost-tally lives at the same per-Review preamble surface as the Phase 3 IAR review-log files; consistency across implementation-cycle + review-cycle.

**Carryforward:** routes to PR-#40-equivalent upstream-suite-remediation cycle — extend the implementation-cycle commit-message convention to require per-commit cost-tally lines. The discipline IS visible at the Phase 3 review-cycle surface; the gap is on the implementation-cycle surface only.

**Classification:** Deferred — methodology-authoring fix is upstream-suite-remediation scope; the bookmark-cli-manual Layer 2 cycle's implementation-cycle cost is not knowable from the audit trail in its current shape.

---

### Dismissed

**Finding 6 — Machine-readability budget regression at the Layer 2 cycle: no parser-aborted incidents (Dim 11)**

<a id="r2-f6"></a>

**Owner:** ai-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

Per the operator's per-domain prompt: "The 'machine-readability budget' finding from AI Engineer R1 ([Review 87 Finding 5](../../../../vsdd-suite/suite-development/review-log/2026-05-21-suite-review.md#review-87--2026-05-21-1230z)) — has Layer 2 surfaced any parser-aborted incidents? Are mid-cycle responses chunking appropriately?"

Initial candidate concern: did the Layer 2 cycle's sub-agent spawns produce any markdown-parser-aborted output (per Review 87 Finding 5's named failure mode where a sub-agent's response triggers a parsing error in the harness that consumes it) or any mid-cycle response chunking failures?

Investigation:

- **Commit message lengths:** all four Layer 2 commit messages are under 50 lines (`326e25d` is the longest at ~30 lines), well under any reasonable harness-parser ceiling. ✓
- **File-content sizes:** `manual-tests/layer-2.md` is 556 lines, which is large but well within the markdown-parser standard tolerance; the file's structure (13 numbered steps each with `## Step N` headings + fenced code blocks + tables) is canonical markdown. No parsing pathology. ✓
- **Mid-cycle response chunking:** the sub-agent's outputs for each of the four commits are not directly observable in the audit trail (they live in the sub-agent's spawn-response surface), but the commit-result shape (cleanly-authored files, clean `cargo test` + `cargo clippy` + `cargo fmt` passes per the commit messages) suggests no chunking failures occurred. A chunking failure would have produced truncated or malformed files; no such evidence. ✓
- **Per-domain Layer 2 review-log files** (in flight at this cluster's authoring time): each file's prose density + finding-count is comparable to the Layer 1 per-domain files (which parsed cleanly per AI Engineer R1 F3 spot-check). No machine-readability-budget regression expected. ✓

The candidate concern resolves cleanly: no Layer 2 cycle artifact exhibits parser-aborted symptoms; no mid-cycle chunking failures; the markdown discipline operative at Layer 1 (per the [Review 88 Finding 5](../../../../vsdd-suite/suite-development/review-log/2026-05-21-suite-review.md#review-88--2026-05-21-1330z) Phase-6-attestation finding's regression-clean evidence) holds at Layer 2.

**Resolution:** No machine-readability budget regression at Layer 2 cycle. The Review 87 Finding 5 named failure modes are not exhibited. Operative discipline holds.

**Classification:** Dismissed — initial candidate concern resolves cleanly; no parser-aborted incidents at Layer 2.

---

### Hallucinated

*(none — the five substantive findings + one dismissed-on-closer-read finding above are all citation-backed against observable audit-trail content; no claim turned out to be a misread of the artifacts)*

---

### Summary

Six findings in Round 2:

- **Resolved:**
  - [Finding 1](#r2-f1) — Sub-agent delegation quality at the manual-tests/layer-2.md spawn (commit `16ee420`); minor `cargo install` cost-refinement candidate
  - [Finding 2](#r2-f2) — Cluster-batching shape preserves adversarial-pair separation; cost-per-finding below band floor is Layer-scoped efficiency, NOT under-investment; methodology-refinement candidate (project-cycle vs. layer-cycle expected-band split)
  - [Finding 3](#r2-f3) — Audit-trail machine-readability holds; the post-PR-#40 preamble extension (model + cost-tally) is correctly applied
- **Deferred:**
  - [Finding 4](#r2-f4) — Phase 2a/2b single-commit shape lost the Red Gate failure-evidence artifact; methodology-authoring fix (require dual-commit OR explicit single-commit-with-rationale-annotation) routes to suite-side
  - [Finding 5](#r2-f5) — Cost-tally aggregation asymmetry between Phase 3 review-cycle (operative) vs. Phase 1a/1b/2a/2b/2c implementation-cycle (gap); methodology-authoring fix (extend implementation-cycle commit-message convention) routes to suite-side
- **Dismissed:**
  - [Finding 6](#r2-f6) — No machine-readability budget regression at Layer 2 cycle; no parser-aborted incidents

**Operator-supplied per-domain-prompt answers (summarized for the audit trail):**

1. _"The Phase 2a/2b sub-agent reported it spawned with `model: sonnet` or `opus` — verify by reading its commit at `326e25d`."_ — The commit message itself does not declare the model; the `Co-Authored-By: Claude Opus 4.7 <noreply@anthropic.com>` trailer suggests Opus 4.7. Per the post-PR-#40 cost-tally discipline, this WOULD be in a `**Cost-tally:**` line if applied to commit-messages — but Finding 5 surfaces that the discipline is only applied at the Phase 3 review-cycle surface, not the implementation-cycle surface. Operative gap.

2. _"The manual-tests/layer-2.md sub-agent at commit `16ee420` ran `cargo install` + 13 step verifications in a real sh. What's the cost evidence?"_ — Cost evidence not in the audit trail per Finding 5. The sub-agent's wall-clock during `cargo install` is real but minor relative to file authoring cost (Finding 1's analysis). The methodology refinement candidate is: future sub-agents could be spawned with a pre-installed-binary precondition to skip redundant `cargo install`.

3. _"Is the Layer 2 Round 1 likely to land at similar cost [to PR #38's ~$20 total], or higher (given Layer 2's smaller surface)?"_ — Likely SIMILAR (~$20) at the 4-cluster shape, but per-finding cost will be below the capstone-intent band floor because Layer 2's surface is smaller. This reads as Layer-scoped efficiency, NOT under-investment (Finding 2).

4. _"The 'machine-readability budget' finding from AI Engineer R1 [Review 87 F5] — has Layer 2 surfaced any parser-aborted incidents?"_ — No (Finding 6).

**Coordination:** Findings 4 and 5 route to a future PR-#40-equivalent upstream-suite-remediation cycle (methodology-authoring fixes spanning the suite-side Phase 2a/2b spawn-prompt template + the implementation-cycle commit-message convention). Findings 1, 2, 3 + 6 document operative disciplines for future-cycle regression-check. Cross-cluster coordination: cluster A's Quality Engineer is likely surfacing the same Phase 2a/2b single-commit-shape concern from the QE Dim 14 TDD-proxy-indicators seat (cross-validation expected); cluster A's Software Engineer is likely surfacing the cost-tally gap from the SE-as-author seat (cross-validation expected).

**Cost-tally** (per [`suite-development/suite-development.md`](../../../../vsdd-suite/suite-development/suite-development.md) § Per-review entry preamble § Cost-tally): cold-session Opus 4.7 agent within the Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster quartet; per-cluster cost expected ~$5 per AI Engineer R1 F1 precedent. Across 6 findings, per-finding cost ~$0.83 (~10k tokens/finding at Opus pricing). Below the AI Engineer Dim 2 capstone-intent band floor (100k tokens/finding) — read as Layer-scoped efficiency per Finding 2's analysis. The audit-trail value (the methodology-authoring fixes Findings 4 + 5 surface) is disproportionate to the small cost — the discipline IS working at this calibration.

**Validator:** sanity-check (per the [Review 77](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-77--2026-05-20-1545z) Finding 2 meta-validator-of-last-resort pattern — AI Engineer has no canonical role-domain pair-validator per the [AI Engineer domain prompt](../../../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md) Validator pair clause).

---

## Review 3 — 2026-05-22 16:30Z

**Phase:** [Phase 3](../../../../vsdd-suite/primers/3-review-session.md) — Iterative Adversarial Refinement (Layer 2 Round 2 verification).
**Source:** domain-raised — cold-session adversarial AI-agent-usage auditor; did not author the fix commits (`156ec53` / `d62bb1a` / `002d747` / `cdb46bc` / `9d56c3f`); treats Review 2 as prior adversary's claim per Dim 1 session-isolation discipline.
**Lens:** Verification of [Review 2](#review-2--2026-05-21-2200z) finding dispositions + cost-tally aggregation across Round 1 + fix cycle + Round 2 + **the recurring lettering-violation pattern as a suite-level meta-finding** per the operator-supplied per-domain prompt for this round.
**Scope:** AI-agent-usage shape of the Layer 2 cycle's Round 1 + fix cycle + Round 2 phases — sub-agent delegation quality at the fix cycle's 5 commits + the Round 2 cluster spawn shape + the cost-tally evidence at cluster close + the lettering-violation recurrence pattern.
**Surface:** the fix cycle's 5-commit shape + this Round 2 cluster's spawn shape + the cost-tally aggregation across the full Layer 2 cycle + the operator-flagged lettering-violation pattern (Cluster A/B/C/D recurrence despite operator memory; PR #38 + Round 1; Task #56 suite-level investigation queue).
**Reviewer:** AI Engineer cold-session agent.
**Model:** Opus 4.7 (per `DESIGN.md § Cold-session budget`).
**Cold-session shape:** Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster (Round 2; same composition as Round 1) per the [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) § Cluster-batching pattern.
**Round:** 3 (AI Engineer R3 against bookmark-cli-manual; R1 audited PR #38 + #39 + #40 cycles; R2 audited Layer 2 implementation + Round 1 cycle; this R3 audits the Layer 2 Round 1 fix cycle + this Round 2 cluster spawn + the cumulative cost-tally).
**Regression-check against:** [Review 1](#review-1--2026-05-21-1000z) (PR #38-cycle baseline; R1 F4 lettering-violation recurrence — STILL RECURRING per this round's evidence below) + [Review 2](#review-2--2026-05-21-2200z) (Layer 2 R1 baseline; R2 F4 + F5 Deferred carryforwards now have additional evidence at the fix-cycle commits).
**Cost-tally:** Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster agent — Opus 4.7; this AI Engineer Round 2 contributed ~25k input + ~14k output tokens ≈ ~$0.61 at standard pricing; per-finding cost ~$0.10 across 6 findings. Below the capstone-intent band floor consistent with [R2 F2](#r2-f2) Layer-scoped efficiency reading.

**Session note:** Cold session opened against the post-commit-`9d56c3f` state. Reading order followed the AI Engineer domain prompt § Current Review Prompt directive: project `README.md` (intent tier + active-domain count — unchanged from R2) → [AI Engineer Review 2](#review-2--2026-05-21-2200z) (prior round findings) → `git log 02e6eb3..9d56c3f` (the 5 fix commits + their stat lines + Co-Authored-By model declarations) → operator-supplied Round 2 prompt (the lettering-violation pattern recurrence call-out specifically) → suite-side discipline files for the operator-feedback memory on avoiding-lettering + Review 78 letter-code retirement context → the four Round 1 per-domain log files (the cluster naming used in each + cross-cluster cross-references) → `DESIGN.md § Cold-session budget` (still operative; not amended by the fix cycle). The DESIGN.md was read for cold-session-budget context only, not for the Layer 2 artifact (out-of-scope per AI Engineer Scope clause).

**MVR signal:** **NOT REACHED at Round 2.** Two of three Deferred Round 1 findings have partial closure evidence (R2 F4 + R2 F5 — the methodology-authoring fixes route to suite-side; this round documents the project-side state as carried-forward-to-suite). One new finding (the recurring lettering-violation pattern) surfaces as a suite-level meta-finding routed to Task #56 / suite-level review queue. Per [G-131](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-131) continue-trigger discipline: AI Engineer Round 4 against bookmark-cli-manual is NOT mandatory because the Open Round 2 carryforwards are all suite-side methodology-authoring concerns (not project-side defects). The Layer 2 cycle's project-side AI-agent-usage discipline is at MVR; the suite-side methodology-authoring carryforwards block at suite-level, not project-level.

---

### Resolved

**Finding 1 — Sub-agent delegation quality at manual-tests/layer-2.md (verifies [r2-f1](#r2-f1)) (Dim 4)**

<a id="r3-f1"></a>

**Owner:** ai-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

Regression-check against [r2-f1](#r2-f1) holds.

**Evidence:**

- No changes to `manual-tests/layer-2.md` from the fix cycle's perspective on the manual-tests/layer-2.md sub-agent's delegation quality at commit `16ee420` — that delegation discipline still holds.
- The fix cycle DID modify `manual-tests/layer-2.md` at Steps 2 / 3 / 7 (per `cdb46bc`) to update the expected stderr from silent-on-success to `Tagged N bookmark(s).` — this was an operator-directed in-cycle fix, not a sub-agent re-delegation. The cost-discipline observation about `cargo install` re-execution from R2 F1 remains a methodology-refinement candidate not adopted in the fix cycle (acceptable; future-cycle work).

**Commentary:** sub-agent delegation discipline at the manual-tests/layer-2.md spawn remains operative; no new regression from the fix-cycle edits.

**Resolution:** Regression-check against [r2-f1](#r2-f1) clean; sub-agent delegation discipline at manual-tests/layer-2.md unchanged.

**Classification:** Resolved — sub-agent delegation discipline holds at Round 2.

---

**Finding 2 — Cluster-batching shape + cost calibration (verifies [r2-f2](#r2-f2)) (Dim 7 + Dim 9)**

<a id="r3-f2"></a>

**Owner:** ai-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

Regression-check against [r2-f2](#r2-f2) holds; now also covers the Round 2 cluster spawn.

**Evidence:**

- The Round 2 cluster spawn manifest (per the operator-supplied per-domain prompts for each cluster) preserves the same 4-cluster shape as Round 1: SE/UX/Performance-Engineer + QE/Security/Technical-Writer + Solution-Architect/Red-Team/Platform-Engineer + Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment.
- Adversarial-pair separation verified at Round 2:
  - Security ↔ Red Team: split (Security in QE/Security/Technical-Writer cluster; Red Team in Solution-Architect/Red-Team/Platform-Engineer cluster) ✓
  - TW ↔ Documentation Reviewer: split (TW in QE/Security/Technical-Writer cluster; Documentation Reviewer in Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster) ✓
- Per-cluster cost trending below Round 1 per the Phase 4 routing scope-reducer discipline ([R1 F2](#r1-f2)): Round 2 scope is narrower (verification + adjacent-defect detection, not full re-scan), so per-cluster cost expected ~$2.25-3 vs. Round 1's ~$5.

**Round-2-specific cost calibration (operator-supplied prompt's analytical question):**

The operator's per-domain prompt asks: "Is this trending efficient or over-investment? With the fix cycle landing 5 commits + 4 cluster Round 1 cold-sessions + 4 cluster Round 2 cold-sessions (in progress), is this proportionate?"

Full Layer 2 cycle cost-aggregation evidence at Round 2 close:

| Phase | Cost-evidence | Cumulative |
|---|---|---|
| Layer 2 implementation cycle (4 commits) | ~$10-15 (per R2 F5 — audit-trail-missing estimate) | ~$10-15 |
| Round 1 cluster cold-sessions (4 clusters × ~$2.25-5) | ~$10-20 | ~$20-35 |
| Round 1 fix cycle (5 commits) | ~$5-10 (audit-trail-missing estimate; same gap as R2 F5) | ~$25-45 |
| Round 2 cluster cold-sessions (4 clusters × ~$2.25-3) | ~$10-12 | ~$35-57 |
| **Total Layer 2 cycle** | | **~$35-57** |

**Calibration:** Layer 2 full cycle cost is ~15-20% of the Layer 1 full cycle cost (~$200-400 per R1 F6). The ratio is consistent with Layer 2's smaller surface (~700 LoC delta vs. Layer 1's ~4,000+ LoC). **Trending efficient, not over-investment.** Per-finding cost across the full cycle (~30 findings in Round 1 + ~14 verification + new findings in Round 2 ≈ ~44 findings): ~$0.80-1.30 per finding ≈ ~10-16k tokens/finding at Opus pricing. **Well below the capstone-intent expected band floor (100k tokens/finding)**, consistent with the Layer-scoped efficiency reading from R2 F2.

**Methodology-refinement candidate (carried forward):** the project-cycle vs. layer-cycle expected-band split from [R2 F2](#r2-f2) is now empirically calibrated: capstone Layer-cycle Round 1 + fix cycle + Round 2 lands at ~10-16k tokens/finding consistently. A future suite-side AI Engineer dim refinement could codify this as the Layer-cycle expected band (~10-30k tokens/finding) distinct from the project-cycle band (100k-300k tokens/finding).

**Resolution:** Cluster-batching shape with adversarial-pair separation correctly preserved at Round 2. Cost-trending efficient. Methodology-refinement candidate (Layer-cycle expected-band codification) routes to suite-side.

**Classification:** Resolved — cluster-batching shape preserved + cost-trending efficient at Round 2.

---

**Finding 3 — Audit-trail machine-readability (verifies [r2-f3](#r2-f3)) (Dim 11)**

<a id="r3-f3"></a>

**Owner:** ai-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

Regression-check against [r2-f3](#r2-f3) holds.

**Evidence:**

- The 4 cluster D Round 2 review entries (Solution Owner R5 + Documentation Reviewer R6 + AI Engineer R3 + VDD-IAR Alignment R5) carry the canonical Agent-API surface contract preamble fields (Source / Lens / Scope / Surface / Reviewer / Model / Cold-session shape / Regression-check against / Cost-tally) per [Review 80 Finding 3](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z).
- Per-Finding anchor IDs preserved at the canonical `<a id="rN-fM"></a>` pattern in each new Round 2 finding/verification entry.
- Classification sub-section headings match each domain's classification universe.
- The 5 fix commits' commit messages do NOT carry `**Model:**` or `**Cost-tally:**` lines (still the R2 F5 gap — see below).

**Commentary:** Audit-trail machine-readability holds at the per-Review preamble surface for Round 2; the implementation-cycle commit-message gap persists.

**Resolution:** Regression-check against [r2-f3](#r2-f3) clean; per-Review preamble surface holds Agent-API contract; commit-message surface gap routed to R3 F5.

**Classification:** Resolved — audit-trail machine-readability discipline holds at Round 2 close.

---

**Finding 4 — Phase 2a/2b single-commit shape Red Gate evidence (verifies [r2-f4](#r2-f4)) (Dim 4)**

<a id="r3-f4"></a>

**Owner:** ai-engineer
**Status:** validated
**Blocked by:** *(none — project-side closure adopted at TODO.md:85; suite-side methodology-authoring fix carried forward to suite-level surface)*
**Validator:** sanity-check

Per [r2-f4](#r2-f4); the project-side closure adopted via the TODO.md:85 Red Gate evidence-preservation annotation; the suite-side methodology-authoring fix carried forward.

**Evidence on the project-side closure:**

- [`TODO.md`](../../TODO.md):85 (post-`002d747`) carries the Red Gate evidence-preservation annotation:
  > "**Red Gate evidence-preservation annotation (Layer 2 Round 1 VDD-IAR Alignment R4 F1).** Layer 2's Phase 2a + Phase 2b landed in the SINGLE commit `326e25d`, which means the Red Gate failure evidence (12 of the 13 new tests failing correctly against the unmodified Layer 1 binary with `error: unrecognized subcommand 'tag'`) lives in the Phase 2b sub-agent's spawn-output report at commit time, NOT in git history as a separate Phase 2a-only commit. This is a methodology-audit-trail tradeoff — the convenience of a single commit at Phase 2b landing time was prioritized over the audit-trail discipline of a two-commit Phase 2a + Phase 2b shape. **For future Layer cycles** (Layer 3, future projects): the canonical shape is **two commits** — one for the Phase 2a Red Gate (failing tests committed alone; CI confirms RED) and a second for the Phase 2b implementation (the same tests pass; CI confirms GREEN). This Layer 2 annotation documents the precedent so the next cycle's operator + sub-agents know to apply the discipline; Round 1 VDD-IAR Alignment R4 F1 surfaced the gap + this paragraph is the closure."

The annotation correctly names:
- ✓ The methodology-audit-trail tradeoff (single-commit convenience vs. dual-commit audit-trail discipline)
- ✓ The sub-agent's spawn-output Red Gate failure evidence (12 of 13 failing with `error: unrecognized subcommand 'tag'`)
- ✓ The forward-looking discipline for future Layer cycles (canonical two-commit shape)
- ✓ The closure-of-finding citation (Round 1 VDD-IAR Alignment R4 F1)
- ✓ The discipline-honest single-commit-with-rationale-annotation pattern consistent with the G-161 Phase 2c annotation precedent

**Evidence on the suite-side carryforward** (not closed in this Layer 2 cycle):

The methodology-authoring fix (extend the suite-side Phase 2a/2b spawn-prompt template to prefer dual-commit shape OR require explicit single-commit-with-rationale annotation) routes to a future PR-#40-equivalent upstream-suite-remediation cycle. Not closed in the bookmark-cli-manual Layer 2 cycle scope.

**Commentary:** the project-side closure is discipline-honest and complete. The methodology-authoring carryforward at suite-side is appropriately scoped to the suite-side surface, not the project-side. R2 F4 is Resolved at the project-side level; Deferred at the suite-side level.

**Resolution:** Round 2 R2 F4 closed at the project-side level by `002d747` adding the TODO.md:85 Red Gate evidence-preservation annotation. The methodology-authoring fix carries forward to suite-side.

**Classification:** Resolved — the project-side closure is complete; the suite-side carryforward routes to a future PR-#40-equivalent.

---

### Deferred

**Finding 5 — Cost-tally aggregation gap on implementation-cycle commits (verifies [r2-f5](#r2-f5)) (Dim 2 + Dim 11)**

<a id="r3-f5"></a>

**Owner:** ai-engineer
**Status:** raised
**Blocked by:** *(none — observable audit-trail content; same gap surfaces at the fix-cycle commits)*
**Validator:** sanity-check

Persists per [r2-f5](#r2-f5); 5 additional data points at the fix-cycle commits reinforcing the same Deferred carryforward.

**Evidence:**

- All 5 fix-cycle commits (`156ec53` / `d62bb1a` / `002d747` / `cdb46bc` / `9d56c3f`) — commit messages name the deliverable + the closure target + verification commands. NONE carry per-commit `**Model:**` or `**Cost-tally:**` lines.
- The Co-Authored-By trailer (`Claude Opus 4.7 <noreply@anthropic.com>`) provides model attribution but not token-count attribution.
- The fix-cycle implementation cost is therefore NOT reconstructible from the audit trail. The same R2 F5 gap (originally surfaced for the 4 Layer 2 implementation commits) now applies to the 5 fix-cycle commits — total 9 commits without per-commit cost-tally.

**Resolution path (unchanged from R2 F5):** extend the suite-side commit-message convention to require per-commit `**Model:**` + `**Cost-tally:**` lines for sub-agent-authored OR operator-directed implementation/fix commits. Routes to suite-side methodology-authoring.

**Commentary:** the gap persists as expected; the fix cycle didn't close it because the fix cycle's scope was the bookmark-cli-manual project-side, not the suite-side methodology-authoring. The carryforward is intact.

**Classification:** Deferred (carryforward from R2 F5; suite-side methodology-authoring scope).

---

**Finding 6 — Recurring lettering-violation pattern as a suite-level meta-finding; routes to Task #56 / suite-level review queue (Dim 12)**

<a id="r3-f6"></a>

**Owner:** ai-engineer
**Status:** raised
**Blocked by:** *(none — observable pattern across multiple cycles; routes to suite-level review queue per operator-supplied Round 2 prompt + Task #56 reference)*
**Validator:** sanity-check

The operator-supplied per-domain prompt for this Round 2 explicitly calls out: "**CRITICAL ADDITIONAL CONCERN:** The operator-flagged recurring lettering-violation pattern (Round 1 used Cluster A/B/C/D despite operator memory; PR #38 also had this issue; queued as Task #56 for suite-level investigation). This is a suite-level meta-finding, not just bookmark-cli-manual-specific. Surface it in Round 2's AI Engineer review with a Disposition routing to the suite-level review queue (Task #56)."

**Pattern documented:**

The Layer 2 Round 1 cluster spawns used letter-coded cluster names (Cluster A / B / C / D) in spawn-time directives + worktree branches + intermediate file naming despite (a) the operator's codified feedback memory on **avoiding lettering and abbreviation standards** ([feedback_avoid_lettering.md](https://github.com/magnificentlycursed/guild-portfolio) — "don't adopt new single-letter labels for methodology concepts; descriptive names carry meaning at point-of-use without lookup"); (b) the prior closure of letter-coded cluster file names at PR #38 Round 3 ([AI Engineer R1 F4](#r1-f4) item 3 — "the operator-corrected Round 3 spawn still produced 3 of 4 cluster files with the retired letter-coded names"; consolidation step splitting them into per-domain files); (c) Review 78 Finding 4 (retired letter-codes more broadly).

**Evidence at Layer 2 Round 1:**

- The Round 1 4-cluster spawn used letter labels A/B/C/D in operator-supplied directives + worktree branch names (`layer-2-cluster-b-review` + `layer-2-cluster-c-review`).
- The Round 1 cluster commits originally used letter labels in commit messages (e.g., `bookmark-cli-manual: Layer 2 Phase 3 IAR Round 1 — Cluster D (Solution Owner + Documentation Reviewer + AI Engineer + VDD-IAR Alignment)` at commit `65f2b76`).
- The corrective commit `02e6eb3` (`bookmark-cli-manual: Layer 2 Round 1 — rename cluster labels from A/B/C/D to composition-based`) — applied AFTER the cluster commits — performed an inline rename to composition-based labels per the operator's codified discipline. The rework cost is non-zero (4 cluster files + cross-references + commit messages had to be touched).
- The canonical per-domain review-log files (post-Review-78 + post-PR-#39 cluster-file-consolidation) ARE correctly named per-domain (`2026-05-21-solution-owner.md`, etc.); the letter-codes appeared at the cluster-manifest + intermediate-file surface, not the canonical audit-trail surface.

**Pattern recurrence at three cycles** (same defect class):

1. **PR #38 Round 3** (per [R1 F4](#r1-f4)) — letter-coded cluster files; consolidation + delete.
2. **Layer 2 Round 1** (this cycle) — letter-coded cluster labels in spawn directives + worktree branches; corrective rename at `02e6eb3`.
3. **(Implied by Task #56 context per the operator's prompt)** — the pattern is recurring across PRs despite each cycle's in-cycle fix. The cycles fix the symptom (rename the files / rename the branches); the recurrence shows the methodology-authoring root cause is not yet closed.

**Dim 12 named failure mode that applies (verbatim from the AI Engineer domain prompt):**

> "operator-directives that surface late in the cycle (the discipline lives in the methodology authoring; if the operator has to surface the discipline mid-cycle, the methodology authoring missed a Dim)"

The lettering-violation is **methodology-authoring missing a Dim**. The spawn-prompt template + the worktree-branch-naming template + the operator-supplied cluster-spawn directive template do NOT currently enforce composition-based labels at spawn time. Each cycle's operator has to RE-issue the directive mid-cycle, costing rework + audit-trail noise.

**Root-cause analysis:**

The defect pattern is a methodology-authoring discipline gap. The suite-side artifacts that drive cluster-spawn shape (the Phase 3 primer; the suite-development.md cluster-batching pattern; the per-cycle pre-spawn declaration template referenced in AI Engineer R1 F8) do NOT codify the "use composition-based labels, not single-letter labels" rule explicitly. Each cycle's operator either remembers to apply the rule in their spawn directive (rare; usually surfaces mid-cycle) or doesn't (common; causes the recurrence).

**Suite-level investigation routing (Task #56):**

Per the operator-supplied Round 2 prompt's explicit instruction, this finding routes to the suite-level review queue as Task #56. The investigation scope:

1. **Audit all current suite-side cluster-spawn-related artifacts** (Phase 3 primer; suite-development.md cluster-batching pattern; any operator-spawn-directive templates; worktree-branch-naming conventions) for explicit composition-based-label requirements.
2. **Codify the discipline at the spawn-prompt-template level** (per AI Engineer R1 F4 item 3's resolution path that did not close: "descriptive cluster-naming from the start"). This may require an explicit pre-cycle methodology check (per AI Engineer R1 F8) item that names "verify the cluster manifest uses composition-based labels, NOT single-letter labels, BEFORE spawning any agent."
3. **Pre-commit hook enforcement** as the structural backstop: a hook that fails commits introducing letter-coded cluster references in spawn directives / worktree branch names / cluster-file paths. This is the discipline-honest path: methodology-authoring + structural enforcement together prevent the recurrence at the surface level.

**Cost-discipline framing:**

The recurrence cost is the cumulative operator-directive correction cost across PR #38 + Layer 2 Round 1 + future cycles that will repeat the slip if the suite-side fix doesn't land. At Layer 2 Round 1, the rename commit `02e6eb3` cost ~$0.20-0.50 in operator time + tooling cost — small per cycle, but compounding across cycles + creating audit-trail noise that future readers have to trace. The suite-side fix is one-time methodology-authoring cost (~$5-10); the per-cycle prevention savings amortize quickly.

**Why this is the AI Engineer surface and not VDD-IAR Alignment or Doc Reviewer:**

The lettering-violation is a Dim 12 (operator-directive correction cost) finding — directly in the AI Engineer's domain prompt failure-mode list. VDD-IAR Alignment owns the methodology-discipline-consistency lens but the specific defect-class (cluster-spawn naming discipline + spawn-prompt-template authoring) is AI Engineer's domain per the cluster-batching shape Dim 7 + the operator-directive-cost Dim 12. Doc Reviewer owns the cold-reader readability lens but the lettering-violation surfaces at the operator-directive + spawn-time-naming surfaces, not the cold-reader audit-trail surfaces (which were already retroactively normalized to per-domain canonical naming).

**Resolution:** route to the suite-level review queue as Task #56. The Layer 2 cycle's project-side surface has been correctively renamed (per `02e6eb3`); the methodology-authoring root cause remains as the suite-side carryforward.

**Classification:** Raised — routes to suite-level review queue (Task #56 per the operator-supplied per-domain prompt's escalation instruction). Not a bookmark-cli-manual-specific finding; a methodology-authoring discipline gap that surfaces across cycles + requires suite-level investigation.

---

### Summary

Round 2 verification:

- **Round 2 Finding 1 verification ([r3-f1](#r3-f1))** — Resolved-and-holds; sub-agent delegation discipline at manual-tests/layer-2.md still operative.
- **Round 2 Finding 2 verification ([r3-f2](#r3-f2))** — Resolved-and-holds; cluster-batching shape preserved at Round 2; cost-trending efficient (~$35-57 full Layer 2 cycle; ~15-20% of Layer 1's cost; ~10-16k tokens/finding consistent with Layer-scoped efficiency reading).
- **Round 2 Finding 3 verification ([r3-f3](#r3-f3))** — Resolved-and-holds; audit-trail machine-readability holds at the per-Review preamble surface for Round 2.
- **Round 2 Finding 4 verification ([r3-f4](#r3-f4))** — Resolved-at-project-side (TODO.md:85 Red Gate evidence-preservation annotation closes the project-side gap); Deferred-at-suite-side (methodology-authoring carryforward).
- **Round 2 Finding 5 verification ([r3-f5](#r3-f5))** — Persists; same cost-tally aggregation gap surfaces at the 5 fix-cycle commits; carryforward intact.
- **New Round 2 Finding ([r3-f6](#r3-f6))** — Raised: recurring lettering-violation pattern as a suite-level meta-finding routed to Task #56 / suite-level review queue per the operator-supplied Round 2 prompt's escalation instruction. **Suite-level finding, NOT project-level; project-side closure already applied at `02e6eb3`; suite-side methodology-authoring root cause requires the Task #56 investigation.**

**Operator-supplied per-domain-prompt answers (summarized for the audit trail):**

1. _"Cost-tally aggregation across Round 1 + fix cycle + Round 2 — is this trending efficient or over-investment?"_ — **Trending efficient** (Layer-scoped) per [r3-f2](#r3-f2). Full Layer 2 cycle ~$35-57; ~10-16k tokens/finding; ~15-20% of Layer 1's cycle cost matching the surface-size ratio.

2. _"The operator-flagged recurring lettering-violation pattern (Round 1 used Cluster A/B/C/D despite operator memory; PR #38 also had this issue; queued as Task #56 for suite-level investigation). Surface it in Round 2's AI Engineer review with a Disposition routing to the suite-level review queue (Task #56)."_ — **Surfaced as [r3-f6](#r3-f6); routes to suite-level review queue / Task #56.**

**Coordination:** [r3-f4](#r3-f4) cross-validates with [VDD-IAR Alignment R5 Round 2](2026-05-21-vdd-iar-alignment.md) at the project-side TODO.md:85 annotation surface — both seats agree the annotation IS the discipline-honest closure of the Red Gate evidence-preservation gap. [r3-f6](#r3-f6) is suite-level; not a project-side cross-validation surface. [r3-f2](#r3-f2) cost-trending cross-validates with [SO R5 r5-f5](2026-05-21-solution-owner.md#r5-f5) cost-proportionality finding — both seats agree the Layer 2 cycle's cost is proportionate.

**MVR signal:** **NOT REACHED at Round 2** for the project-side regression-check (all dispositions are Resolved-at-project-side OR Resolved-and-holds), BUT the suite-side carryforwards ([r3-f5](#r3-f5) + [r3-f6](#r3-f6) + the methodology-authoring component of [r3-f4](#r3-f4)) remain Open at suite-level. The project-side does not block Layer 2 closure; the suite-side carryforwards route to suite-level investigation (Task #56 + future PR-#40-equivalent upstream-suite-remediation cycle).

**Phase 5 / Phase 6 closure-blocker check:** none at AI Engineer surface. The methodology-authoring carryforwards are suite-side; they don't block bookmark-cli-manual's Layer 2 Phase 5 closure or the Phase 6 NOT APPLICABLE declaration. AI Engineer R3 explicitly clears project-side Layer 2 closure.

**Cost-tally:** Round 2 contributed ~$0.61 across 5 verification entries + 1 new finding + 1 summary = ~$0.10 per-finding. Consistent with the Layer-scoped efficiency reading from [r2-f2](#r2-f2) + [r3-f2](#r3-f2).

**Validator:** sanity-check (per the AI Engineer domain prompt's no-canonical-pair-validator clause + the [Review 77](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-77--2026-05-20-1545z) Finding 2 meta-validator-of-last-resort pattern).

---
