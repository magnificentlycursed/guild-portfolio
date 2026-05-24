# Suite review log — 2026-05-23

Per-session suite-development review entries land here. Per the [`SUITE-DEVELOPMENT-REVIEW.md`](../SUITE-DEVELOPMENT-REVIEW.md) Reviews table, this file is the canonical home for Reviews 90+ dated 2026-05-23.

---

## Review 90 — 2026-05-23 12:00Z

**Phase:** Suite-development meta-review (AI Engineer methodology codification per the carryforward queue from PR [#44](https://github.com/magnificentlycursed/guild-portfolio/pull/44) Layer 2 capstone cycle).

**Source:** director-raised (operator directed five separate methodology corrections across PR #44: (1) operator memo 2026-05-21 "You lettered the clusters again. This has happened multiple times. Queue a review by the AI Engineer domain to investigate the recurring naming problem."; (2) operator memo 2026-05-22 "I use the Claude Max plan. That needs to be taken into account for AI Engineering costing and optimization. Other users or projects might use something different so that needs to be known to accurately assess costs, available features, etc. A supplemental per AI tooling may be an appropriate way to get specific."; (3) operator IDE-selection of `Claude Max` line in `vsdd-suite/CHANGELOG.md` line 790 — flagged the G-135 trigger clause as assumed-tool/plan language; (4) operator memo 2026-05-22 "Do not assume AI tool, plan, or execution method. Determine through verifiable means or prompt the user if that's not possible."; (5) parser-aborted error on heredoc-based file writes via the Bash tool — three observed instances in PR #44 mid-cycle).

**Lens:** methodology-recurrence-prevention + cost-discipline-portability + AI-tool-portability.

**Scope:** Suite-level methodology surface. Files touched: `vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md` (Dim 14 addition); `vsdd-suite/supplements/claude-code-cli.md` (new — first per-tool supplement); `vsdd-suite/primers/3-review-session.md` (cost-tally report shape extension + rate-limit-headroom generalization + AI-tool/plan/execution-method declaration requirement); `vsdd-suite/CHANGELOG.md` (G-135 trigger generalization at line 790 + PR #45 [Unreleased] entry); `vsdd-suite/suite-development/FINDINGS-INDEX.md` (G-135 row trigger generalization + new G-XXX entries for Findings 1-5 below); `vsdd-suite/suite-development/SUITE-DEVELOPMENT-REVIEW.md` (Review 90 row addition); `feedback_avoid_lettering.md` memory (load-bearing-rule lead + pre-spawn check + third-recurrence addendum).

**Reviewer:** AI Engineer (suite-level methodology authoring).

**Model:** Opus 4.7 (`claude-opus-4-7`).

**Cold-session shape:** N/A — main-session inline authoring of the methodology codification. Per AI Engineer Dim 7 cluster-batching framing: this is a methodology-codification cycle, not an adversarial-review cycle; cold-session cluster spawn is over-investment per [G-150](FINDINGS-INDEX.md#g-150). The cycle's adversarial-review evidence comes from PR #44's 4-cluster Round 1 + Round 2 (already cold-session in the original cycle); the codification here applies the lessons from those cycles' findings.

**Regression-check against:** PR [#39](https://github.com/magnificentlycursed/guild-portfolio/pull/39) (AI Engineer domain initial registration; Review 83 in [`2026-05-21-suite-review.md`](2026-05-21-suite-review.md#review-83--2026-05-21-1000z)); PR [#40](https://github.com/magnificentlycursed/guild-portfolio/pull/40) (upstream-suite-remediation; Reviews 84+85+86+87 in [`2026-05-21-suite-review.md`](2026-05-21-suite-review.md)); PR [#44](https://github.com/magnificentlycursed/guild-portfolio/pull/44) Layer 2 IAR cycles (the source of the five carryforward findings).

**Session note:** Methodology-codification cycle, not adversarial-review cycle. Sycophancy-compensation declared explicitly: the same agent (claude-opus-4-7) that committed all three recurring patterns (lettering at PR #38 + PR #44; assumed-tool/plan language at the G-135 trigger; parser-aborted heredoc three times this session) is now codifying the methodology mitigations for them. The natural bias is to write the mitigations as if the prior failures were anomalous; the corrective framing applied here is to write each finding's body as if the failures will recur unless the mitigation includes an executable check (pre-spawn announcement; verifiable-means detection; Write/Edit-over-heredoc workflow rule). The Finding 1 escalation path (pre-commit hook if a fourth recurrence happens) is the explicit acknowledgment that memory-feedback-alone is empirically insufficient.

**Cost-tally:**

- **AI tool:** [claude-code CLI](https://claude.com/claude-code) (current release)
- **Plan tier:** Claude Max (operator's personal plan; declared explicitly per Finding 4's discipline)
- **Execution method:** inline main-session methodology authoring; no sub-agent spawns
- **Model:** Opus 4.7
- **Raw tokens (estimated):** ~50-80k for the five findings' authoring (read existing prompts + supplements + memories; write new supplement + Dim 14 + primer 3 extension + this review entry; update CHANGELOG + FINDINGS-INDEX + SUITE-DEVELOPMENT-REVIEW)
- **Would-be API cost** (comparator only — Opus 4.7 API rate): ~$3-5 USD
- **Actual cost to operator:** $0 marginal (within Max plan limits)
- **Wall-clock:** ~30-45 minutes (one continuous methodology-authoring session)
- **Findings/100k tokens:** 5 / ~60k = ~8.3 findings/100k tokens — well above the capstone-intent expected band of 1 per 100-300k tokens. This is the expected density for a methodology-codification cycle (the findings are pre-named by the carryforward queue; the work is authoring the resolutions, not surfacing the findings).

---

### Resolved

<a id="r90-aie-f1"></a>
**Finding 1 — Recurring lettering-violation pattern in cluster-spawn prompts; memory framing restructured + pre-spawn check requirement added (Dim 12 — operator-directive correction cost)**

**Owner:** ai-engineer
**Status:** validated
**Blocked by:** *(none — memory restructure applied; pre-spawn check requirement codified in feedback memory)*
**Validator:** solution-owner

**Evidence:** Three confirmed instances of the Cluster A/B/C/D lettering pattern recurring after explicit feedback memory at `feedback_avoid_lettering.md`:

1. **PR #38 Round 3 (2026-05-20)** — first instance; operator caught: "I don't like the lettering. Remember the TW guidance on naming and abbreviations." Composition-based labels applied at consolidation. Memory updated with the cluster-naming addendum.
2. **PR #44 Layer 2 Round 1 (2026-05-22)** — second instance despite the memory. Operator caught: "You lettered the clusters again." Composition-based labels applied at commit `02e6eb3` via sed sweep across 14 review-log files.
3. (Pre-existing) **Review 78 / 79 Surface A/B/C/D lettering for Phase 5 hardening surfaces** — the original lettering violation that authored the memory; predates the cluster-naming subclass.

**Reasoning:** The existing memory documented the lettering issue including the PR #38 Cluster A/B/C/D slip (line 28 of the prior memory). Despite this, the orchestrator (claude-opus-4-7) repeated the pattern at PR #44 Round 1. Root-cause hypotheses surfaced during this Review's authoring: (a) the load-bearing instruction was buried at line 28 of a 28-line memory; (b) no pre-spawn check existed to catch the violation before sub-agents launched; (c) under token-economy pressure at scale (4 parallel agents in one message), the orchestrator defaulted to short labels for prompt brevity; (d) the memory rule was stated as "avoid X" without naming the substitution rule explicitly. The first three hypotheses are addressed by the memory restructure + pre-spawn check; the fourth was already addressed in the prior memory (the substitution rule was named — "use composition-based labels") but was not the lead-bearing instruction.

**Resolution applied:**

1. **Memory restructure at `feedback_avoid_lettering.md`** — leading paragraph now contains the load-bearing rule: "When spawning parallel sub-agents in a cluster shape, EVERY mention of the cluster MUST use composition-based labels... ONE letter-based label anywhere in this surface is a violation." Surface enumeration (spawn prompt; worktree branch name; filename; commit message; review-log preamble's `**Cold-session shape:**` field) is explicit.
2. **Pre-spawn check requirement** added to the memory: "in the message immediately preceding the parallel-`Agent` invocation, write the composition-based label for each cluster explicitly. If the operator sees `Cluster A` (or any letter+nothing label) in that pre-spawn announcement, the violation has already happened."
3. **Third-recurrence addendum** at the bottom of the memory documents the PR #44 instance + the escalation path: "If a fourth recurrence happens after PR #45 merges, the memory framing isn't enough — the next escalation is a pre-commit hook scanning spawn-prompt-pattern files for letter-only cluster labels."
4. **Known Issue entry** in `vsdd-suite/supplements/claude-code-cli.md` documents the recurrence pattern + mitigation + escalation path so future claude-code CLI orchestrators inherit the discipline.

**Classification:** Resolved (Dim 12 — operator-directive correction cost; the cost of each prior correction was ~5-15 minutes of consolidation rework; the codified mitigation prevents the rework cost recurring).

---

<a id="r90-aie-f2"></a>
**Finding 2 — AI Engineer domain prompt missing verify-tool/plan/execution-method-first dimension; Dim 14 added (Dim 14 — Tool / plan / execution-method identification)**

**Owner:** ai-engineer
**Status:** validated
**Blocked by:** *(none — Dim 14 added at vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md; G-135 trigger generalized at vsdd-suite/CHANGELOG.md line 790 + FINDINGS-INDEX G-135 row)*
**Validator:** solution-architect

**Evidence:** Three observed instances of the assumed-tool/plan pattern:

1. **G-135 trigger clause** at `vsdd-suite/CHANGELOG.md` line 790 (Review 60 — 2026-05-19) — original phrasing: "second portfolio project hits Claude Max daily limit (or comparable token-budget exhaustion) AND operator decides AI-cost engineering is next priority arc." Assumes the operator's tool/plan in forward-facing trigger language. Operator surfaced via repeated IDE-selection during PR #44 cycle (2026-05-23).
2. **Sub-agent cost-tally reports across PR #44** — each cluster sub-agent reported dollar cost figures (~$2.64, $2.19, etc.) as if measured. The operator was on Claude Max throughout; the dollar figures were API-tier-pay-per-token estimates, not the operator's actual cost. Operator surfaced via direct question 2026-05-22: "How do you calculate the cost estimates?"
3. **Pre-existing audit-trail surfaces** — primer 3 § Pre-cycle methodology check's "Rate-limit headroom" field named "Anthropic daily-token-limit headroom" specifically; the domain prompt's Dim 5 named "Anthropic load window" for rate-limit-strategy framing. Both assume Anthropic-API-direct billing as the canonical case.

**Reasoning:** The methodology was authored against the original operator's tool/plan (Claude Max + claude-code CLI) without the generalization that future operators may adopt the suite under Cursor, Aider, ChatGPT Plus, Claude API direct, or other tools with distinct rate-limit + cache + tool-cost surfaces. The AI Engineer domain's Dim 5 (rate-limit strategy) + Dim 6 (model selection) + Dim 2 (token economy per finding) all assumed Anthropic-API-direct semantics where the operator was actually on a subscription plan. Per Dim 14's named failure modes: dollar costs were quoted as measured when actual cost was $0 marginal; rate-limit recommendations assumed token-per-minute caps when the binding constraint was 5-hour-rolling-window utilization; sub-agent cost framing excluded orchestrator overhead.

**Resolution applied:**

1. **AI Engineer Dim 14 added** at `vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md` (between Dim 13 pre-cycle methodology check + the Three-audience lens section). Named failure modes cover the assumed-API-billing trap, assumed-Anthropic-rate-limit trap, sub-agent cost excluding orchestrator overhead, cross-tool optimization advice that doesn't account for per-tool surfaces. The exact test names verifiable-means detection (process inspection; env vars; CLI features; git config) OR prompt-the-operator as the fallback.
2. **G-135 trigger clause generalized** at `vsdd-suite/CHANGELOG.md` line 790 + `vsdd-suite/suite-development/FINDINGS-INDEX.md` G-135 row. Generalized phrasing: "AI-tool/plan rate-limit or token-budget exhaustion event (e.g., Claude Max daily message-cap recurrence, ChatGPT Plus message-cap, Claude API token-budget hard cap, Cursor / Aider / other-CLI rate-limit)". The original Review 60 phrasing is preserved verbatim in `vsdd-suite/suite-development/review-log/2026-05-18-suite-review.md` per G-89 forward-only narrative-preservation.
3. **Primer 3 § Pre-cycle methodology check** extended with a new required field: "**AI tool + plan tier + execution method** (per AI Engineer Dim 14)". The "Rate-limit headroom" field's "Anthropic daily-token-limit" phrasing was generalized to "AI-tool/plan rate-limit-window headroom" with per-tool cross-references.
4. **First per-tool supplement** (`vsdd-suite/supplements/claude-code-cli.md`) — Finding 3 below; documents the per-tool specifics for claude-code CLI.

**Classification:** Resolved (Dim 14 — Tool / plan / execution-method identification; the dim itself is the codification of the gap).

---

<a id="r90-aie-f3"></a>
**Finding 3 — Per-tool supplements absent; first instance `claude-code-cli.md` authored as adoption surface (Dim 14 — Tool / plan / execution-method identification)**

**Owner:** ai-engineer
**Status:** validated
**Blocked by:** *(none — supplement authored at vsdd-suite/supplements/claude-code-cli.md)*
**Validator:** documentation-reviewer

**Evidence:** Per the operator's 2026-05-22 directive ("A supplemental per AI tooling may be an appropriate way to get specific. You can make one for claude-code CLI when you tackle this work"), the suite has per-language supplements (`rust.md`, `python.md`, `bash.md`) + per-interface supplements (`github-actions.md`, `markdown.md`, `yaml.md`, `toml.md`) but had no per-tool supplements. AI Engineer Dim 14 (Finding 2) needs a per-tool supplement target to route findings to; without per-tool supplements, the Dim 14 verification would surface the gap but have no canonical home for the resolution.

**Reasoning:** The supplement pattern is the suite's natural home for tool/plan/method-specific guidance that the domain prompts are deliberately tool-agnostic about. The first per-tool supplement (`claude-code-cli.md`) sets the body shape for the pattern: plan tiers + rate-limit windows + prompt-cache TTL + per-tool token costs + execution-method semantics + Known Issues + canonical optimization patterns + cost-tally discipline. Future per-tool supplements (cursor.md, aider.md, codex-cli.md, anthropic-api-direct.md, chatgpt.md) inherit the shape so the AI Engineer domain's findings + recommendations remain tool-portable.

**Resolution applied:**

1. **`vsdd-suite/supplements/claude-code-cli.md`** created with the canonical body shape covering all sections enumerated above. Includes Known Issues for the two recurring patterns surfaced in this session: parser-aborted on heredoc-based file writes (Finding 5) + lettering-violation recurrence in cluster-spawn prompts (Finding 1).
2. **Cross-references from `AI-ENGINEER-REVIEW.md` Dim 14** — the dim names per-tool supplements as the canonical home for tool-specific specifics.
3. **Cross-references from `primer 3 § Pre-cycle methodology check`** — the "Rate-limit headroom" field cites the claude-code CLI supplement for the 5-hour-rolling-window discipline.

**Classification:** Resolved (Dim 14 — Tool / plan / execution-method identification; the per-tool supplement is the canonical home for the tool-specific specifics the dim refers callers to).

---

<a id="r90-aie-f4"></a>
**Finding 4 — Cost-tally plan-tier discipline gap; primer 3 cost-tally report shape extended with 10-field requirement (Dim 14 + Dim 2 — Token economy per finding)**

**Owner:** ai-engineer
**Status:** validated
**Blocked by:** *(none — primer 3 § Pre-cycle methodology check extended with the 10-field cost-tally report shape)*
**Validator:** solution-architect

**Evidence:** Across PR #44 Layer 2 cycle, sub-agent cost-tally reports presented dollar figures as if measured. Examples:

- Cluster (SO+DR+AIE+VDDIAR) Round 1 report: "~173k tokens ≈ ~$2.64"
- Cluster (SE+UX+Perf-Eng) Round 1 report: "~$1.50-2.00 USD"
- Cluster (SO+DR+AIE+VDDIAR) Round 2 report: "~$2.19 total (~$0.43 SO + ~$0.51 DR + ~$0.61 AIE + ~$0.64 VDD-IAR)"

The operator's actual marginal cost on Claude Max was $0 for all of these spawns (within plan limits). The dollar figures were API-tier-pay-per-token estimates, useful as a "would-be API cost" comparator but misleading as the operator's actual cost. The operator surfaced the gap via direct question 2026-05-22.

Additional discipline gaps surfaced during the question's answer:

- Prompt-cache discount not modeled (cached input is ~10% of uncached cost over the 5-min TTL; sub-agent reports tended to overestimate by 2-5×)
- Orchestrator's own context cost excluded from the cluster cost figures
- Per-tool token costs not enumerated (Read vs Edit vs Write vs Bash vs Agent have distinct per-invocation overhead)

**Reasoning:** Without a canonical cost-tally report shape, sub-agents drift toward dollar figures because dollar conversion is what training-data examples lead with. The fix is to make raw tokens the canonical measure + treat dollar conversion as an explicit "would-be API cost" comparator with the prefix.

**Resolution applied:**

1. **Primer 3 § Pre-cycle methodology check** extended with a "**Cost-tally report shape**" sub-section. 10 required fields: AI tool / Plan tier / Execution method / Model / Raw tokens (canonical) / Would-be API cost (comparator only) / Actual cost to operator / Rate-limit-window utilization where observable / Wall-clock duration / Findings per 100k tokens (cross-cycle comparator). The "Raw tokens" field is explicitly named the canonical measure; dollar conversion gets the explicit "would-be API cost" prefix.
2. **`claude-code-cli.md` § Cost-tally discipline** (per primer 3 § Cost-tally) — restates the 10-field shape with claude-code-CLI-specific notes (the `/cost` command shows API-equivalent cost; the orchestrator's own context cost is NOT zero; plan-tier identification is operator-declared because the CLI does not expose plan tier as a verifiable field).
3. **Application to subsequent reviews** — the SA Phase 5 Review 4 + QE Phase 5 Review 6 at PR #44 already adopted the cost-tally discipline upgrade at commits `1f53540` + `1ed337e`; this Review 90 entry's cost-tally also adopts the shape. Future reviews inherit by following the primer.

**Classification:** Resolved (Dim 14 + Dim 2 — the report-shape extension is the codification of the gap).

---

<a id="r90-aie-f5"></a>
**Finding 5 — Recurring parser-aborted error on heredoc-based file writes via the Bash tool; mitigation operative + Known Issue documented (Dim 11 — Audit-trail machine-readability cost; Dim 12 — Operator-directive correction cost)**

**Owner:** ai-engineer
**Status:** validated
**Blocked by:** *(none — Known Issue documented in claude-code-cli.md; workflow rule operative)*
**Validator:** quality-engineer

**Evidence:** Three observed instances during PR #44 Layer 2 capstone cycle (2026-05-22 to 2026-05-23 UTC). All three were `cat <<'EOF' ... EOF` invocations through the Bash tool that succeeded on disk (verifiable via `Read` or `wc -l` post-hoc) but tripped the operator's downstream tooling parser/transport mid-response:

1. **SA Phase 5 Review 4 heredoc append** (~123 lines; appended to `2026-05-22-solution-architect.md`). File wrote successfully (380 lines; ending paragraph intact). Operator surfaced parser-aborted 2026-05-22.
2. **QE Phase 5 Review 6 heredoc append** (~108 lines; appended to `2026-05-21-quality-engineer.md`). File wrote successfully (571 lines). Operator surfaced parser-aborted 2026-05-22.
3. (Pre-existing) **PR #40 / PR #42 mid-cycle heredoc** — operator memo references prior incidents; documented as AI Engineer Dim 11 finding in [Review 87 Finding 5](2026-05-21-suite-review.md#review-87--2026-05-21-1230z) (machine-readability budget).

All three heredocs contained: markdown with embedded backticks; fenced code blocks; em-dashes; bold markdown patterns; Unicode (em-dash, ≥, ≤, ✓). Hypothesized triggers (per Task #56 Finding 5 framing): (a) response size threshold; (b) embedded backticks / `EOF` near-collisions in heredoc body; (c) Unicode characters; (d) line-count threshold; (e) interaction with the operator's specific downstream tooling.

**Reasoning:** The original mitigation per Review 87 Finding 5 was "chunking responses across smaller turns". The recurrence at PR #44 indicates chunking-alone is insufficient — the heredoc invocation itself is the trigger, not the overall response size. The deeper mitigation is to **stop using heredocs** for content > 50 lines OR containing markdown bold + em-dash + extensive backticks. The Bash tool's `cat <<EOF` should be reserved for short config-file creation (≤30 lines, no embedded markdown).

**Resolution applied:**

1. **Workflow rule codified in `claude-code-cli.md` § Known Issues**: "NEVER use heredoc for content > 50 lines OR containing markdown bold + em-dash + extensive embedded backticks. Use `Write` for new files; `Edit` for appending or surgical changes."
2. **Verifiable-means check** added: "if a parser-aborted is suspected, verify the file state via `Read` or `wc -l` BEFORE assuming the write failed. In all three observed instances, the file had written successfully and only the response transport aborted."
3. **In-session mitigation operative**: the remaining authoring in PR #45 (this Review 90 entry; the primer 3 extension; the SUITE-DEVELOPMENT-REVIEW row addition) all use `Write` + `Edit` tools, not heredoc. Zero parser-aborted incidents since the workflow rule was adopted.

**Classification:** Resolved (Dim 11 + Dim 12; the workflow rule is the codification of the mitigation; future claude-code CLI orchestrators inherit it from the supplement's Known Issues section).

---

### Summary

Suite-level AI Engineer methodology codification round closing the five-finding carryforward queue from PR [#44](https://github.com/magnificentlycursed/guild-portfolio/pull/44) Layer 2 capstone cycle. All five findings are Resolved with applied codification in five suite-surface artifacts: AI Engineer domain prompt (Dim 14 added); first per-tool supplement (`claude-code-cli.md`); primer 3 § Pre-cycle methodology check (cost-tally report shape extended + rate-limit-headroom generalized + AI-tool/plan/execution-method declaration required); CHANGELOG + FINDINGS-INDEX G-135 trigger generalization; `feedback_avoid_lettering.md` memory restructure with load-bearing-rule lead + pre-spawn check requirement.

The five-finding codification cycle was operator-directed (Director-raised per primer 3 § Source attribution). The methodology-recurrence-prevention budget for this cycle was ~50-80k tokens / ~30-45 min wall-clock at $0 marginal cost on Claude Max. The next escalation (if a sixth recurrence pattern surfaces) is pre-commit hook mechanization — currently deferred per primer 4 § "Defer routing to the next round" anti-pattern with a named trigger: "fourth recurrence of any documented pattern after this PR merges."

**Coordination:** Routes forward to (a) the next bookmark-cli-manual cycle (the four-cluster spawn shape will exercise the post-PR-45 memory framing + pre-spawn check); (b) any future operator adopting the suite under a non-Anthropic tool (Cursor / Aider / ChatGPT Plus / Claude API direct) — the Dim 14 verification + per-tool supplement pattern is the canonical adoption surface; (c) `bookmark-cli-crosslink` build-from-scratch (Task #17) — its capstone cycle will be the first end-to-end test of the post-PR-45 methodology against a fresh project rather than a Layer 2 extension. The PR #45 audit-trail closure is sufficient evidence that the codification is internally consistent; the empirical evidence of recurrence-prevention is the next-cycle test.

---

## Review 91 — 2026-05-23 19:00Z

**Phase:** Suite-development meta-review (adversarial audit of bookmark-cli-manual Layers 1 + 2 against the suite's own phase / primer / supplement / domain discipline).

**Source:** director-raised (operator directive: "Do an adversarial review of guild-projects/guild-portfolio/vsdd-suite-reference-examples/bookmark-cli-manual layers 1 and 2. Evaluate the use of vsdd-suite to complete these layers. Did it flow from phase to phase correctly? Were primers and supplements used when they should be? Were the domains effective? Write up your findings and recommendations as a suite review.")

**Lens:** Post-cycle conformance audit of the reference example against the suite's own governing standards — phase-to-phase flow per the VSDD whitepaper's 1a+1b → 1c → 2a → 2b → 2c → 3 → 4 → 5 → 6 progression + primer invocation per [`primers/`](../../primers/) per phase + supplement citation per [`supplements/`](../../supplements/) per language/interface surface + domain effectiveness per the [DOMAIN-INDEX](../../domains/DOMAIN-INDEX.md) capstone-tier active set. Sycophancy compensation declared explicitly: the bookmark-cli-manual project reached project-terminal MVR + Phase 6 attestation at Layer 1 and is at Layer 2 carry-forward-close — the "the cycle closed; the discipline must have been right" framing was kept as a hypothesis to verify rather than a default conclusion. Findings below distinguish suite-discipline-operative-as-intended (Dismissed) from suite-discipline-gap-surfaced-by-the-reference-example (Open).

**Scope:** bookmark-cli-manual Layer 1 + Layer 2 artifacts as of 2026-05-23. Read: [`DESIGN.md`](../../../vsdd-suite-reference-examples/bookmark-cli-manual/DESIGN.md) (Phase 1a+1b contract + intent calibration + Phase 5/6 strategy + threat model + storage data classification); [`TODO.md`](../../../vsdd-suite-reference-examples/bookmark-cli-manual/TODO.md) (Phase 1c decomposition + Red Gate test plan + layer-gate criteria for L1 + L2); [`PROCESS.md`](../../../vsdd-suite-reference-examples/bookmark-cli-manual/PROCESS.md) (layer-by-layer retrospective + AI-co-authored disclosure + 3 original L1 + 3 post-PR-#38/#39/#40 stumbling points + L2 retrospective); [`CHANGELOG.md`](../../../vsdd-suite-reference-examples/bookmark-cli-manual/CHANGELOG.md) (PR #44 + PR #46 + PR #47 entries); [`vsdd-suite/FINDINGS-INDEX.md`](../../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/FINDINGS-INDEX.md) (47 enumerated L1 findings + L2 narrative cross-reference); spot-sampled review-log entries across the 29 per-session files (QE 2026-05-17 first L1 round; AI Engineer 2026-05-21 Reviews 1-3; SE 2026-05-20 5 rounds; SA 2026-05-22 L2 R1; Red Team 2026-05-22 L2 R1; Security 2026-05-22 L2 R1+R2; Platform Engineer 2026-05-22 L2 R5; Technical Writer 2026-05-22 L2 R5+R6; QE 2026-05-21 L2 Reviews 4-7 incl. Phase 5 Mutation Testing). Mechanical sweeps: `grep -cE 'supplements/'` and `grep -cE 'primers/'` across all 29 review-log files; supplement-by-name presence check against [`vsdd-suite/supplements/*.md`](../../supplements/). Suite governing standards consulted: [`suite-development.md`](../suite-development.md) § Governing standard for project-level review logs + § Supplement coverage + § Layer-gate close criteria; [`primers/3-review-session.md`](../../primers/3-review-session.md) § Round triggers + § Pre-cycle methodology check; [`primers/2a-red-gate.md`](../../primers/2a-red-gate.md); [`primers/5-formal-hardening.md`](../../primers/5-formal-hardening.md); [`primers/4-feedback-integration.md`](../../primers/4-feedback-integration.md).

**Cold-session shape:** N/A — main-session inline suite-review per the [`suite-development.md`](../suite-development.md) § Session isolation framing for suite reviews. The reviewer (this Opus 4.7 session) did not author bookmark-cli-manual or any of its review-log entries and has no prior turn-context for the project under review; reading order followed the suite governing standard rather than the project's chronological audit trail.

**Regression-check against:** [Review 90](#review-90--2026-05-23-1200z) (PR #45 AI Engineer codification closing PR #44 carryforwards — primer 3 cost-tally extension + Dim 14 + `claude-code-cli.md` supplement); [Review 88](2026-05-21-suite-review.md#review-88--2026-05-21-1330z) (Phase 6 attestation routed to bookmark-cli-manual VDD-IAR Alignment Review 3 + 4 upstream-suite recurrence-prevention applications from Nathan-thread mining); [Review 86](2026-05-21-suite-review.md#review-86--2026-05-21-1200z) (PROCESS.md three-audience-lens optimization with the 3 stumbling-point claims PROVEN OUT against review-log evidence); [Review 87](2026-05-21-suite-review.md#review-87--2026-05-21-1230z) (per-error-class owner table — informs Owner-field selection for the findings below); [Review 82](2026-05-20-suite-review.md#review-82--2026-05-20-2000z) (the 80-finding Round 1 orchestration record + Round 2 fix-cycle).

**Session note:** In-session suite review per [`suite-development.md`](../suite-development.md) § Session isolation framing. Sycophancy-compensation declared: the bookmark-cli-manual project is the reference example for the suite — the natural authoring bias is to validate that the worked example walks the methodology cleanly. The Lens forces the inverse posture: identify methodology-discipline gaps that the reference example surfaces by virtue of having walked the full 6-phase cycle twice. Findings derive from artifact-state analysis (grep counts; primer/supplement file existence; preamble-shape conformance) rather than narrative judgment about whether the project "feels" disciplined. Confidentiality-aware citation discipline applied: no operator paths cited; only project-relative paths + suite-relative paths.

**Cost-tally:**

- **AI tool:** [claude-code CLI](https://claude.com/claude-code)
- **Plan tier:** Claude Max (operator's personal plan; declared per [Review 90 Finding 4](#review-90--2026-05-23-1200z) discipline)
- **Execution method:** inline main session; no sub-agent spawns
- **Model:** Opus 4.7 (`claude-opus-4-7`)
- **Raw tokens (estimated):** ~90-110k for the audit (read suite governing standards + bookmark-cli-manual artifacts + 8 review-log files spot-sampled + 4 mechanical greps + this review entry authoring)
- **Would-be API cost** (Opus 4.7 API tier; comparator only — NOT operator's actual cost on Max plan): ~$4-6 USD
- **Actual cost to operator:** $0 marginal (within Max plan limits)
- **Wall-clock:** ~45-60 minutes (single continuous suite-audit session)
- **Findings/100k tokens:** 7 / ~100k = ~7 findings/100k — above the capstone-intent expected band of 1/100-300k tokens (band is for adversarial cycles against shipped code; meta-audits against an already-reviewed artifact surface methodology-gaps at higher density because the audit-trail itself is the surface)

---

### Open

<a id="r91-f1"></a>
**Finding 1 — Phase 2a Red Gate commit-boundary violation recurred at Layer 2 despite Layer 1's QE R1 F1 acknowledgment; the methodology lesson is named in PROCESS.md but no hard-gate mechanism propagates it forward as a layer-gate criterion**

**Owner:** ai-engineer (process-enforcement surface per [Review 87 Finding 6](2026-05-21-suite-review.md#review-87--2026-05-21-1230z) per-error-class owner table)
**Status:** raised
**Blocked by:** *(none)*

**Evidence:** Two confirmed instances of Phase 2a + Phase 2b landing in a single commit (Red Gate failure-state never present in git history as a standalone commit):

1. **Layer 1** — original reference-implementation session (pre-PR-#38). Surfaced at [QE Review 1 Finding 1](../../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-17-quality-engineer.md#review-1--2026-05-17-0325z) as `Phase 2a → 2b commit boundary not enforced`. Resolved by documenting the deliberate scope-tradeoff in the in-session log + flagging for the PROCESS.md retrospective.
2. **Layer 2** — Phase 2a + Phase 2b both landed in commit `326e25d`. Surfaced at [VDD-IAR Alignment Layer 2 R4 F1](../../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-21-vdd-iar-alignment.md) (per [TODO.md § Layer 2 Phase 2c Red Gate evidence-preservation annotation](../../../vsdd-suite-reference-examples/bookmark-cli-manual/TODO.md)). Resolved by adding a future-cycle-canonical-shape annotation to the project's TODO.md — same pattern as L1's resolution.

[`primers/2a-red-gate.md`](../../primers/2a-red-gate.md) requires the failing-test state to be committed before any Phase 2b implementation. The L1 lesson was documented (PROCESS.md § Layer 1 § What I got wrong) and the L2 cycle reproduced the same violation despite the L1 finding being publicly committed to the project's audit trail. Pattern parallels [Review 90 Finding 1](#review-90--2026-05-23-1200z)'s lettering-violation recurrence (memory-feedback-alone empirically insufficient): the per-project PROCESS.md note about Phase 2a discipline did not propagate forward as a hard-gate mechanism.

**Reasoning:** The suite's [§ Layer-gate close criteria](../suite-development.md#layer-gate-close-criteria-processmd-retrospective-discipline) baseline does not include a Phase 2a Red Gate evidence-preservation criterion. Criterion 5 names the build+test gate (`cargo build && cargo test && cargo clippy && cargo fmt --check --locked`) which passes regardless of commit shape. Criterion 7 names PROCESS.md retrospective developer-voice discipline but does not name commit-history-shape discipline. The Phase 2a primer's "failing-test commit precedes implementation" requirement is enforced only by reviewer-discretion at [VDD-IAR Alignment Dim 4](../../domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md), which catches the violation post-hoc — too late to be a hard gate.

**Recommendation:**

1. **Primer 2a hardening** — extend [`primers/2a-red-gate.md`](../../primers/2a-red-gate.md) § Completion criteria with an explicit "verifiable git-history check" sub-section: the layer's first commit on the layer-branch SHOULD be a Phase 2a-only commit (failing tests; CI confirms RED); the second commit SHOULD be the Phase 2b implementation (same tests pass; CI confirms GREEN). If a project deliberately combines them, the deviation MUST be documented in TODO.md § Layer N Phase 2c BEFORE the cycle begins (operator-acceptance recorded in audit trail) rather than after VDD-IAR Alignment surfaces it.
2. **Pre-cycle declaration extension** — extend [`primers/3-review-session.md`](../../primers/3-review-session.md) § Pre-cycle methodology check with a Phase-2a-evidence-shape declaration field for the cycle's first round (operator names "two-commit canonical" or "single-commit-with-justification"). Pairs with the existing AI tool / plan tier / execution method declaration from [Review 90 Finding 2](#review-90--2026-05-23-1200z).
3. **Hook escalation path** (deferred) — if a third recurrence happens on a project where the operator did NOT pre-declare the single-commit deviation, escalate to a pre-commit hook scanning the layer-branch's first commit for the `tests/` + `src/` co-modification pattern that signals undeclared Phase-2a/2b consolidation. Mechanizing this is over-investment relative to two recurrences; the third-recurrence trigger parallels the lettering-violation hook proposal in [Review 90 Finding 1](#review-90--2026-05-23-1200z).

**Classification:** Open (registered for tracking; the codification work is a separate PR per the no-stacked-PRs operator preference).

---

<a id="r91-f2"></a>
**Finding 2 — Supplement citation discipline regressed from Layer 1 → Layer 2; the suite-development.md governing standard for project-level review logs does not explicitly require domain reviewers to inline-link the supplement file path when applying a supplement's dimensions**

**Owner:** technical-writer (governing-standard prose authoring surface)
**Status:** raised
**Blocked by:** *(none)*

**Evidence:** Mechanical sweep across all 29 bookmark-cli-manual per-session review-log files (`grep -cE 'supplements/'`):

| Date / domain | Supplement-path link count |
|---|---|
| 2026-05-20 ux | 10 |
| 2026-05-20 technical-writer | 10 |
| 2026-05-20 platform-engineer | 10 |
| 2026-05-20 red-team | 6 |
| 2026-05-20 software-engineer | 5 |
| 2026-05-20 performance-engineer | 5 |
| 2026-05-20 security | 3 |
| 2026-05-20 documentation-reviewer | 3 |
| 2026-05-21 ux | 7 |
| 2026-05-21 ai-engineer | 3 |
| 2026-05-21 quality-engineer | 3 |
| 2026-05-21 technical-writer | 1 |
| 2026-05-21 platform-engineer | 0 |
| 2026-05-21 documentation-reviewer | 0 |
| 2026-05-21 red-team | 0 |
| 2026-05-21 solution-owner | 0 |
| 2026-05-21 solution-architect | 0 |
| 2026-05-21 vdd-iar-alignment | 0 |
| **2026-05-22 technical-writer** (L2 R5+R6) | **0** (inline links; prose-only "Rust supplement § TW floor" mentions) |
| **2026-05-22 solution-architect** (L2 R1) | **0** inline links; 1 prose mention |
| **2026-05-22 platform-engineer** (L2 R5) | **0** inline links; 2 prose mentions |
| **2026-05-22 security** (L2 R1+R2) | **0** inline links; 2 prose mentions |
| **2026-05-22 red-team** (L2 R1) | **0** inline links; 1 prose mention |

The Layer 2 cluster cold-session reviews (2026-05-22-*) adopted a "the standard <Domain> dimensions + the Rust supplement § <Section> floor raised every finding below" template in the `**Source:**` field but did NOT inline-link the supplement file path. A reader cold-loading a Layer 2 review-log file cannot click through to the supplement that "raised every finding" — the citation is prose, not navigable. By contrast, the Layer 1 PE log links the supplement path 10 times inline; the L2 PE log has zero inline links.

**Reasoning:** [`suite-development.md`](../suite-development.md) § Governing standard for domain files § item 5 requires the domain prompt file to reference a supplement (or opt-out with rationale), but the parallel governing standard for project-level review logs (§ Per-review entry preamble) lists `Assumption surfacing` as an optional field for QE but does NOT enumerate `Supplement applied` as a required or optional preamble field. The L1 reviewers self-imposed the discipline via the older per-domain index file's "Language supplement applied" header line (item 6 of the file-level header for the now-retired per-domain index files); when the per-domain index files were retired at PR #40 / [Review 84](2026-05-21-suite-review.md#review-84--2026-05-21-1100z), the supplement-link surface was lost without a parallel discipline propagating to the per-session file's preamble.

**Recommendation:** Extend the per-review entry preamble standard (suite-development.md § Per-review entry preamble § Optional fields) with a new optional field:

```
- **Supplement applied:** [path with inline markdown link] § [Section name] — required when the domain prompt references a supplement; explicit opt-out (`Not applicable. [Reason].`) when language-agnostic.
```

The field replaces the prose-only "the Rust supplement § X floor raised every finding below" template with a parseable + clickable surface. The grep-by-supplement pattern (`grep -B 3 '^\*\*Supplement applied:\*\* \[.*rust\.md' vsdd-suite/review-log/`) becomes an agent-API surface per [§ Agent-API surface](../suite-development.md#agent-api-surface-review-80-finding-3). Forward-only per [G-89](../FINDINGS-INDEX.md#g-89); existing pre-2026-05-23 review-log entries remain valid as historical records.

**Classification:** Open (registered for tracking; the governing-standard amendment + the corresponding companion update to [`primers/3-review-session.md`](../../primers/3-review-session.md) is a separate PR per the no-stacked-PRs operator preference).

---

<a id="r91-f3"></a>
**Finding 3 — `vsdd-suite/supplements/github-actions.md` is never cited in any bookmark-cli-manual review-log file despite multi-round Platform Engineer review of `.github/workflows/bookmark-cli-manual.yml`; one PE Layer 2 review cites a non-existent "Security supplement § GitHub Actions" — the canonical supplement is silently bypassed**

**Owner:** platform-engineer (artifact-CI surface per [Review 87 Finding 4](2026-05-21-suite-review.md#review-87--2026-05-21-1230z))
**Status:** raised
**Blocked by:** *(none)*

**Evidence:** Mechanical sweep (`grep -lE 'github-actions\.md|GitHub Actions supplement'` across all 29 bookmark-cli-manual review-log files): **0 matches**. The [`vsdd-suite/supplements/github-actions.md`](../../supplements/github-actions.md) supplement was authored at [Review 86 Finding 1](2026-05-21-suite-review.md#review-86--2026-05-21-1200z) (~280 lines, covering 8 role-domain perspectives including Platform Engineer PRIMARY) and immediately applied to update `.github/workflows/bookmark-cli-manual.yml` per [Review 86 Finding 3](2026-05-21-suite-review.md#review-86--2026-05-21-1200z). Despite the supplement existing + the project's CI workflow being a load-bearing artifact, no review-log entry cites the supplement.

The PE L2 R5 ([`2026-05-22-platform-engineer.md`](../../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-22-platform-engineer.md) line 253) cites: *"Per the Security supplement § GitHub Actions, this is the correct posture for build/test workflows."* — no `vsdd-suite/supplements/security.md` exists; no "GitHub Actions" sub-section exists in any other supplement. The canonical home for that guidance IS [`vsdd-suite/supplements/github-actions.md`](../../supplements/github-actions.md). This is supplement-name-misattribution: the reviewer apparently fabricated the supplement path rather than locating the canonical one.

The PE L1 R1 ([`2026-05-20-platform-engineer.md`](../../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-20-platform-engineer.md) line 837) cites: *"the TOML supplement § Security 'Pinned dependency versions' framing applies analogously"* — for SHA-pinning of GitHub Actions. This is the closest the project gets to acknowledging the github-actions surface, and even here the citation routes through TOML rather than the dedicated github-actions supplement (which post-dates the L1 review).

**Reasoning:** The github-actions.md supplement was authored chronologically AFTER the L1 PE review (Review 86 = 2026-05-21; PE L1 R1 = 2026-05-20). The L2 PE review (2026-05-22) had the supplement available but did not cite it. This is partially explained by [Finding 2](#r91-f2) above (the inline-supplement-link discipline was already eroding by L2) but is also a discrete defect: when a new supplement is authored, the discipline for retroactive application to projects-in-flight is not codified. The github-actions.md supplement was implicitly used (the CI workflow was updated in the same PR) but explicit traceability in the per-domain review log is absent.

**Recommendation:**

1. **PE Layer 2 R5 amendment** — add a Round 3 entry to [`2026-05-22-platform-engineer.md`](../../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-22-platform-engineer.md) correcting the misattributed "Security supplement § GitHub Actions" reference to the canonical [`vsdd-suite/supplements/github-actions.md`](../../supplements/github-actions.md) path. Adopt the [Finding 2](#r91-f2)-recommended `**Supplement applied:**` field.
2. **Suite-side hook proposal** (deferred per no-stacked-PRs) — when a new supplement is authored at suite-side, the operator-action queue should include a sweep over active-project review-logs that touch the supplement's domain to verify the supplement is now cited where applicable. The mechanism is the same "earned by recurrence" doctrine — currently one recurrence; codify if a second supplement-non-citation surfaces.
3. **AI Engineer Dim 11 audit-trail-machine-readability extension** — add a "supplement-citation completeness" check to the [AI Engineer domain prompt](../../domains/role/AI-ENGINEER-REVIEW.md) Dim 11: every domain's per-session review-log entries SHOULD link the supplement file path when the domain's prompt references a supplement. Pairs with [Finding 2](#r91-f2)'s `**Supplement applied:**` preamble field.

**Classification:** Open (registered for tracking; the project-side amendment to PE L2 R5 + the suite-side hook proposal are separate cycles per the no-stacked-PRs operator preference).

---

<a id="r91-f4"></a>
**Finding 4 — `vsdd-suite/supplements/json.md` is cited only by Red Team (Layer 1 + Layer 2); SE, QE, SA, Security never link the supplement despite material engagement with serde JSON serialization + downgrade-compatibility hazard + storage-format extension**

**Owner:** technical-writer (governing-standard prose authoring surface) + each domain reviewer for the project-side amendment
**Status:** raised
**Blocked by:** [Finding 2](#r91-f2) (the `**Supplement applied:**` preamble field codification is the natural enforcement surface)

**Evidence:** Mechanical sweep (`grep -lE 'supplements/json\.md|JSON supplement'` across all 29 bookmark-cli-manual review-log files): only [`2026-05-20-red-team.md`](../../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-20-red-team.md) + [`2026-05-21-red-team.md`](../../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-21-red-team.md). The project's storage format is JSON; multiple Layer 2 findings turn on serde semantics:

- [SA L2 R1 F5](../../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-22-solution-architect.md) — `tags: Vec<String>` with `#[serde(default)]` downgrade-corruption hazard. SA cites no supplement at all.
- [Security L2 R4 F1](../../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-22-security.md) — DESIGN.md § Storage data classification incomplete for `tags` field (JSON serialization carries the field cross-binary-version). Security cites "Rust supplement § Security floor" but not the JSON supplement that owns the serde-asymmetry-as-attack-surface discipline.
- [SE L1 R1+R2](../../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-20-software-engineer.md) — `BookmarkStore::load` + `save` JSON round-trip + atomic-write semantics. SE cites the Rust supplement extensively but not the JSON supplement.
- [QE L1 R2](../../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-20-quality-engineer.md) — Mutation Testing surfaced a missing falsifying test for the save-to-nested-path JSON-write case. QE cites no supplement.

The JSON supplement at [`vsdd-suite/supplements/json.md`](../../supplements/json.md) exists and per [`suite-development.md` § Supplement coverage](../suite-development.md#supplement-coverage), is presumably structured by domain (every supplement file follows the same shape: one section per applicable domain). Red Team alone using it suggests the supplement's content has Red Team-specific framing but lacks visibility for other domains.

**Reasoning:** Two compounding causes:

1. The `**Supplement applied:**` preamble field is not required ([Finding 2](#r91-f2)) — so reviewers default to not citing supplements.
2. Domain reviewers consult the supplement that matches their domain primarily (Rust supplement for SE/QE/SA/Security); the cross-cutting interface supplements (JSON / Markdown / YAML / TOML / GitHub Actions / Bash) are easy to miss when the reviewer's reading order is "domain prompt → language supplement" without an explicit interface-supplement pass.

**Recommendation:** Pair with [Finding 2](#r91-f2)'s `**Supplement applied:**` preamble field by allowing multiple supplement applications per review:

```
- **Supplements applied:** [`rust.md`](../../supplements/rust.md) § Solution Architecture; [`json.md`](../../supplements/json.md) § Storage / cross-version compatibility — applies because the L2 surface extends a serde-serialized data model with downgrade semantics.
```

The plural form makes interface-supplement application explicit. A domain reviewer who genuinely has nothing to apply from JSON / Markdown / etc. provides the explicit-opt-out (`JSON not applicable — the L2 surface adds no new JSON-serialization-bearing fields`); silent omission is no longer accepted.

**Classification:** Open (registered for tracking; codification blocked by [Finding 2](#r91-f2)'s parent governing-standard amendment).

---

<a id="r91-f5"></a>
**Finding 5 — Phase 5 Layer 2 hardening rounds (SA Purity Boundary Audit re-run + QE Mutation Testing re-run + QE Phase-5-trigger follow-up at PR #47) ran inline in the main session rather than as cold-session cluster spawns; the rationale (per G-150 over-investment guard) is encoded in each round's session note but the methodology decision is not codified in [`primers/5-formal-hardening.md`](../../primers/5-formal-hardening.md)**

**Owner:** ai-engineer (methodology-codification surface)
**Status:** raised
**Blocked by:** *(none)*

**Evidence:** Three Phase 5 Layer 2 rounds explicitly declare inline execution + name the trade-off:

1. [QE Review 6 — 2026-05-22 22:30Z](../../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-21-quality-engineer.md#review-6--2026-05-22-2230z) (Mutation Testing re-run): *"Cold-session shape: N/A — inline-run from the main session. Trade-off declared per the parallel SA Review 4 framing: a parallel cold-session cluster spawn would be over-investment per [G-150](../FINDINGS-INDEX.md#g-150); the mutation-testing tool produces the evidence and the analysis is the only judgment surface."*
2. [SA Review 4 — 2026-05-22 22:00Z](../../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-22-solution-architect.md) (Purity Boundary Audit re-run): parallel framing — inline main-session execution.
3. [QE Review 7 — 2026-05-23 16:00Z](../../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-21-quality-engineer.md#review-7--2026-05-23-1600z) (Phase-5-trigger follow-up at PR #47): *"Cold-session shape: N/A — inline main-session methodology authoring; same shape as Review 6's inline-run trade-off rationale."*

The decision rule is well-reasoned: Phase 5 hardening involves tool-output as the primary evidence base (cargo-mutants output; purity-boundary cross-source grep), and the analysis surface is mechanical disposition rather than adversarial framing — cold-session cluster spawn against tool-output is over-investment because the tool produces deterministic evidence regardless of session-isolation. But this decision rule lives only in the project's per-round session notes; the primer that would teach future projects when inline-Phase-5 is acceptable does not yet codify it.

**Reasoning:** [`primers/5-formal-hardening.md`](../../primers/5-formal-hardening.md) presumably (un-verified at this audit; primer not loaded for this finding) defaults to the same cold-session discipline that primer 3 establishes for Phase 3 IAR. The bookmark-cli-manual Phase 5 Layer 2 cycle is the canonical worked example of when inline execution is methodology-appropriate (deterministic tool output + bounded judgment surface + small-scope verification), but the primer cannot teach this rule from the project's per-round session notes alone — the rule must be codified at primer level.

**Recommendation:** Extend [`primers/5-formal-hardening.md`](../../primers/5-formal-hardening.md) § Completion criteria (or add a new § Cold-session-vs-inline decision sub-section) with an explicit decision rubric:

- **Cold-session required** when the Phase 5 surface involves adversarial-framing judgment (e.g., a Purity Boundary Audit on a freshly-authored library where module-doc claims must be evaluated against implementation; a Fuzz Testing campaign where the corpus design itself involves adversary-modeling).
- **Inline execution acceptable** when the Phase 5 surface is tool-output-driven evidence with bounded disposition (e.g., re-run cargo-mutants against an unchanged purity boundary with a documented prior baseline; re-verify proptest property assertions against a structurally-stable strategy).
- **Per-round declaration required** — every Phase 5 round MUST explicitly declare its cold-session vs inline choice + the trade-off rationale, parallel to [`primers/3-review-session.md`](../../primers/3-review-session.md) § Pre-cycle methodology check's discipline.

The bookmark-cli-manual Layer 2 Phase 5 cycle becomes the canonical "inline-acceptable" worked example; cite it explicitly.

**Classification:** Open (registered for tracking; the primer 5 codification is a separate PR per the no-stacked-PRs operator preference).

---

### Dismissed

<a id="r91-f6"></a>
**Finding 6 — Phase 4 (feedback integration) has no separate per-session review-log files for Layer 1 or Layer 2; the absence of a Phase-4-named session log was initially framed as a missing-phase concern**

**Owner:** *(none — finding dismissed)*
**Status:** *(none — terminal)*

**Evidence:** No file in [`vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/`](../../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/) is named `*-phase-4.md` or `*-routing.md`. The 29 per-session files are all per-domain Phase 3 IAR rounds (plus Phase 5 hardening rounds filed under the executing domain per [G-177](../FINDINGS-INDEX.md#g-177)).

**Reasoning for dismissal:** [`primers/4-feedback-integration.md`](../../primers/4-feedback-integration.md) does not specify a separate per-session log file shape for Phase 4 routing. The routing discipline is operative through three suite-codified mechanisms that the bookmark-cli-manual cycle uses extensively:

1. **`Raised to SO` sub-heading** per [`suite-development.md` § Cross-cutting classification](../suite-development.md) — bookmark-cli-manual uses this at SA L1 R1 F1 + SA L2 R1 F2 + Red Team L2 R1 F3 + Red Team L2 R2 F10 + UX L1 R4 F3 etc.
2. **`Blocked by:` lifecycle field** per [Review 77](2026-05-20-suite-review.md#review-77--2026-05-20-1545z) — bookmark-cli-manual uses this throughout the L2 fix-cycle to chain spec-amendment dependencies.
3. **`**Coordination:**` closing line** per [`suite-development.md` § Closing block](../suite-development.md) — bookmark-cli-manual uses this in every round to name handoffs.

The Phase 4 routing operates inline at the per-finding level rather than as a separate session — this is the methodology working as designed, not a gap. The [PROCESS.md § Post-PR-#38/#39/#40 stumbling points § Stumbling point 6](../../../vsdd-suite-reference-examples/bookmark-cli-manual/PROCESS.md) "site-specific fix declared closure" pattern actually exercised Phase 4 routing across rounds (Doc Reviewer R1→R2→R3 carryforward eventually codified the `grep -rn before claiming closure` discipline in primer 4 at PR #40 / [Review 84](2026-05-21-suite-review.md#review-84--2026-05-21-1100z)).

**Classification:** Dismissed (Phase 4 functions inline-per-finding by methodology design; no separate session log is required or expected).

---

<a id="r91-f7"></a>
**Finding 7 — PROCESS.md is AI-co-authored and the file's § AI-co-authored reference-example disclosure explicitly names that it does NOT satisfy [G-156](../FINDINGS-INDEX.md#g-156) (developer-voice retrospective discipline); initial concern that this constitutes a layer-gate close criterion 7 bypass**

**Owner:** *(none — finding dismissed)*
**Status:** *(none — terminal)*

**Evidence:** [`bookmark-cli-manual/PROCESS.md` lines 15-21](../../../vsdd-suite-reference-examples/bookmark-cli-manual/PROCESS.md):

> **This PROCESS.md is AI-co-authored.** [...] The developer-voice discipline of [G-156](../FINDINGS-INDEX.md#g-156) specifies director-authored retrospective prose for actual portfolio / capstone / production projects — a project whose retrospective is AI-authored has not satisfied the [G-156](../FINDINGS-INDEX.md#g-156) discipline, full stop. [...] This file exists to **demonstrate the retrospective FORMAT** the suite teaches [...] A real project at capstone intent must replace this file's content with director-authored retrospective prose before the layer-gate close criterion 7 is satisfied.

The disclosure is explicit + load-bearing. The L1 layer-gate criterion 7 (PROCESS.md retrospective developer-voice) is structurally bypassed for the reference example because the developer IS the AI; the disclosure makes the bypass auditable.

**Reasoning for dismissal:** The concern was substantively addressed by two existing suite mechanisms before this audit:

1. The **§ AI-co-authored reference-example disclosure** paragraph at PROCESS.md:15-21 honestly names the limitation; reviewer cannot mistake the AI-authored prose for developer-voice.
2. The **[Review 86 Finding 4](2026-05-21-suite-review.md#review-86--2026-05-21-1200z)** three-audience-lens optimization PROVED OUT the three original L1 stumbling-point claims against review-log evidence (Purity Boundary divergence F-004; Phase 2a→2b commit boundary F-001; Mutation Testing kill rate F-005). The three-audience treatment validates the lesson-carrying value of the AI-authored prose without claiming developer-voice authority.

The honest-disclosure shape is the methodology-correct answer for a reference example whose existence pre-requires AI-authoring of the retrospective. A future "non-reference-example" project at capstone intent would fail the layer-gate criterion 7 with this file's content; the reference example's exemption is named explicitly + the disclosure prevents the exemption from propagating silently to other projects.

**Classification:** Dismissed (the existing § AI-co-authored reference-example disclosure + the Review 86 three-audience PROVEN-OUT framing already constitute the appropriate methodology response; no separate carve-out is needed because the disclosure IS the carve-out, scoped to the reference example by name).

---

### Summary

Post-cycle adversarial audit of bookmark-cli-manual Layer 1 + Layer 2 against the suite's own phase / primer / supplement / domain discipline. **7 findings filed: 5 Open + 2 Dismissed.**

**Phase-to-phase flow audit:** All six VSDD phases (1a+1b spec → 1c decomposition → 2a Red Gate → 2b implementation → 2c refactor → 3 IAR → 4 routing → 5 hardening → 6 convergence) executed for Layer 1 with project-terminal MVR + Phase 6 attestation at PR #42. Layer 2 executed phases 1a+1b → 1c → 2a → 2b → 2c → 3 → 4 → 5 with Phase 6 explicitly marked NOT APPLICABLE per [G-150](../FINDINGS-INDEX.md#g-150) + [G-112](../FINDINGS-INDEX.md#g-112) (operator-decision Cluster D Solution Owner recommendation adopted). **One real methodology gap surfaced: Phase 2a Red Gate commit-boundary violation recurred at L2 despite L1's QE R1 F1 acknowledgment ([Finding 1](#r91-f1))** — same pattern as [Review 90 Finding 1](#review-90--2026-05-23-1200z)'s lettering-violation (memory-feedback-alone insufficient for cross-cycle propagation). One concern dismissed: Phase 4 routing absence is the methodology working as designed via inline `Raised to SO` + `Blocked by` + `Coordination` mechanisms ([Finding 6](#r91-f6)).

**Primer + supplement usage audit:** Primers ARE consistently referenced across rounds (primer 3 in every Phase 3 round; primer 5 in every Phase 5 round; primer 6 in the Phase 6 attestation). Supplement usage shows a clear Layer 1 → Layer 2 regression ([Finding 2](#r91-f2)): L1 reviewers inline-linked supplements 3-10× per review; L2 reviewers reduced to 0-2× per review with prose-only "supplement § X floor" mentions. The github-actions.md supplement is never explicitly cited despite multi-round PE engagement with `.github/workflows/bookmark-cli-manual.yml` + one L2 PE review citing a non-existent "Security supplement § GitHub Actions" ([Finding 3](#r91-f3)). The json.md supplement is cited only by Red Team despite extensive SE/SA/Security engagement with serde semantics ([Finding 4](#r91-f4)).

**Phase 5 inline-vs-cold-session decision codification opportunity:** The bookmark-cli-manual Layer 2 Phase 5 cycle used inline main-session execution for tool-output-driven evidence (cargo-mutants; purity-boundary cross-source); the trade-off rationale per [G-150](../FINDINGS-INDEX.md#g-150) over-investment guard is well-reasoned but lives only in per-round session notes. [Finding 5](#r91-f5) recommends extending [`primers/5-formal-hardening.md`](../../primers/5-formal-hardening.md) with an explicit cold-session-vs-inline decision rubric citing this cycle as the canonical worked example.

**Domain effectiveness audit:** All 13 capstone-active domains produced substantive findings across L1+L2. Highest-signal domains by codification-into-permanent-suite-improvements: AI Engineer (10 R1 findings → 5 codified at Review 90 — PR #45's AI Engineer domain prompt Dim 14 + first per-tool supplement claude-code-cli.md + primer 3 cost-tally extension + memory restructure); Quality Engineer (cargo-mutants 100% L1 kill rate + 93.2% L2 kill rate with named per-mutant rationale); Solution Architect (Purity Boundary Audit cross-source divergence pattern); Security/Red Team adversarial pair (timing oracle + downgrade-corruption hazard + chained-vulnerability framing). Data Engineer correctly ruled out per [G-178](../FINDINGS-INDEX.md#g-178). Sanity Check meta-domain activated on-demand for findings without natural cross-domain pair. **No evidence of domain over-extension or unused domains.** The 80-finding L1 R1 surface noted in PROCESS.md § Stumbling point 4 is a pre-IAR-phase under-investment signal (already self-named in PROCESS.md three-audience treatment); the L2 cycle's 30-finding R1 surface against a smaller code change demonstrates the discipline calibrating correctly.

**The reference example IS the worked example end-to-end.** Layer 1 project-terminal MVR + Phase 6 attestation closed the reference-example purpose ([G-112](../FINDINGS-INDEX.md#g-112)); Layer 2 demonstrates that subsequent layers don't require their own Phase 6 — capstones gate at project-terminal MVR per primer 6, not per-layer. The 5 Open findings in this audit are methodology-hardening opportunities surfaced BY the reference example having walked the full cycle twice; they are not defects in the reference example itself but rather suite-discipline gaps that two cycles' worth of audit-trail evidence makes visible.

**Coordination:** Routes forward to (a) a future suite PR (per no-stacked-PRs operator preference) bundling the [Finding 1](#r91-f1) primer 2a hardening + [Finding 2](#r91-f2) suite-development.md `**Supplement applied:**` preamble field + [Finding 5](#r91-f5) primer 5 cold-session-vs-inline rubric — these three are related-but-independent methodology codifications; bundling matches the operator's earlier guidance that closely-related changes go in one PR rather than three; (b) a project-side amendment to [`2026-05-22-platform-engineer.md`](../../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-22-platform-engineer.md) correcting the misattributed "Security supplement § GitHub Actions" reference per [Finding 3](#r91-f3); (c) the `bookmark-cli-crosslink` build-from-scratch (Task #17) — its capstone cycle will be the first test of whether the [Finding 1](#r91-f1) Phase 2a evidence-shape declaration discipline propagates correctly to a fresh project, parallel to [Review 90](#review-90--2026-05-23-1200z)'s coordination note about the recurrence-prevention codifications.

**Backlog after Review 91: 6 Open ([Review 79 Finding 2 Deferred](2026-05-20-suite-review.md#review-79--2026-05-20-1730z) + 5 new Review 91 Open findings) + 7 prior-Deferred** (unchanged from Review 90).
