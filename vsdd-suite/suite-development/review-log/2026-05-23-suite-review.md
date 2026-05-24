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
