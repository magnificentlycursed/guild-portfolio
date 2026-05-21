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

**Finding 3 — Audit-trail machine-readability holds under the Agent-API surface contract; the 9 per-domain review-log files parse cleanly under the dual-audience design principle (Dim 11)**

<a id="r1-f3"></a>

Spot-check against [Review 80](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z) Finding 3 Agent-API surface contract:

- **Review heading regex** (`^## Review \d+ — \d{4}-\d{2}-\d{2} \d{2}:\d{2}Z$`): 8 of 9 forward-facing per-domain review-log files match cleanly. The two pre-2026-05-21 migrated files ([2026-05-17-quality-engineer.md](2026-05-17-quality-engineer.md) and [2026-05-20-quality-engineer.md](2026-05-20-quality-engineer.md) / [2026-05-20-solution-architect.md](2026-05-20-solution-architect.md)) carry migration notes per Review 78 Finding 1 and are exempt from the post-Review-77 standard.
- **Per-Finding anchor IDs** (`<a id="rN-fM"></a>`): counted across the 9 forward-facing files = 25 (Doc Reviewer R1) + 28 (Red Team R1+R2+R3) + 27 (UX R1+R2+R3) + 20 (Security R1+R2+R3) + 16 (Platform Engineer R1+R2+R3) + 14 (SE R1+R2+R3) + 15 (TW R1+R2+R3) + 12 (SO R1+R2+R3) + 9 (Performance Engineer R1+R2+R3) + 8 (VDD-IAR Alignment R1+R2) = ~174 anchors across the ~100 substantive findings. Agents reading prose can navigate Finding → registry → cross-references in one hop without constructing anchors from heading text.
- **Classification sub-section headings** (`^### (Resolved|Deferred|Dismissed|Hallucinated|Open|Raised to SO|Accepted risk|Backlogged|Accepted limitation)$`): grep against the 9 files returns clean section-boundary hits per the [hook's DOMAIN_CLASSIFICATIONS dictionary](../../../../vsdd-suite/hooks/check-project-review-discipline.py); the section heading universe matches the per-domain classification universe.
- **Required-closer presence** (`**Coordination:**` line per Review): present at every Review N's closing block.

The 4 Round 3 intermediate cluster files (`engineering-cluster-round-3.md`, `cluster-b-round-3.md`, `cluster-c-round-3.md`, `cluster-d-round-3.md`) were deleted at consolidation per [Review 82](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z) Finding 5's file-consolidation-note, so the canonical audit trail never sees the cluster-file shape. An Agent-API consumer landing on the per-session-file directory sees the canonical per-domain shape with `## Review 1 / Review 2 / Review 3` headings inside; no consolidation cost is paid by the consumer.

The Dim 11 cost is operative at near-zero — the dual-audience design principle ([Review 80](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z) Finding 3) was authored before the bookmark-cli-manual cycle ran, so every per-domain file was authored against the contract from the start. The Round-1-fix-cycle didn't have to retrofit the anchor IDs because they were already in the spawn-prompt template.

**Owner:** ai-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

**Validator rationale:** Dim-11 finding spans the [`markdown.md`](../../../../vsdd-suite/supplements/markdown.md) supplement + the suite-development.md § Agent-API surface contract + the 9 per-domain review-log files; no single role-domain pair-validator. Sanity Check applies the dual-audience design principle's named regex/grep patterns to the observed audit trail.

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
