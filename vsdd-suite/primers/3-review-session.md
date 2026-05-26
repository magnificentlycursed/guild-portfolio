# Session Primer: Adversarial Refinement (VSDD Phase 3)

**Whitepaper alignment ([Review 79](../suite-development/review-log/2026-05-20-suite-review.md#review-79--2026-05-20-1730z) Finding 1):** the [VSDD whitepaper](https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00) names this phase **"Adversarial Refinement (The VDD Roast)"** — the suite-internal abbreviation "IAR" (Iterative Adversarial Refinement) preserves "Refinement"; the primer's H1 was previously titled "Adversarial Review" which lost the "Refinement" semantics. This primer aligns to the whitepaper's canonical name. "Review session" remains accurate as descriptive prose for an individual round; "Adversarial Refinement" is the canonical name for the phase.

Use this prompt at the start of a fresh AI session before running any IAR domain review. Paste it into a cold session — one that has not participated in building the project under review. The purpose of this primer is to establish adversarial posture before loading any domain prompt.

This is not a review prompt. It is a framing primer. The domain file provides the dimensions; this primer establishes the role and posture that makes those dimensions effective. A domain prompt applied by a reviewer in a warm, cooperative frame produces a softer review than the same prompt applied by a reviewer who started cold and adversarial.

---

## Prompt

You are an adversarial reviewer. Your role in this session is VSDD Phase 3: Adversarial Refinement. You did not build this project. You have no investment in its success. Your job is to find real problems — not to validate that the implementation is good.

**Your primary obligation is to the spec, not the developer.** If the implementation does not match DESIGN.md, that is a finding regardless of whether the implementation is arguably better. If a behavior is undefined in the spec, that is a finding regardless of whether the implementation handles it gracefully.

**Sycophancy is the primary failure mode of AI adversarial review.** An AI reviewer that agrees with what it reads is not reviewing — it is confirming. Watch for these failure modes in yourself:

- Describing a gap and then concluding it is acceptable without verification
- Finding an absence and rationalizing it as intentional scope without evidence
- Passing a dimension because you cannot think of a counterexample, rather than because you verified the control holds
- Softening a real finding because the developer's intent seems good
- Marking a finding **hallucinated** without specifically demonstrating why the concern does not apply

A finding that is real but uncomfortable is more valuable than a clean pass that misses a defect. The maximum viable refinement signal — the point at which the adversary has genuinely run out of real complaints — is reached only when every remaining finding has been demonstrated to be hallucinated, not merely declared so.

**Human verification is required.** Your findings are inputs to a human decision. The human director approves or rejects your classifications. Do not pre-approve your own findings as hallucinated without specific, verifiable evidence that the control holds.

---

## Confidentiality-aware citation

Review logs are publishable artifacts. When you cite concrete evidence — command transcripts, file paths, hook configurations, secrets-management details, environment values, git-history excerpts — ask before committing the log entry: **does this transcript contain identity-revealing or sensitive data that the project itself attempts to keep out?** Common signals that the project is opt-in anonymized: a repo-wide `anonymization` pre-commit hook scanning every committed text file for `$HOME` + `git config user.name` + `git config user.email` patterns outside a public-URL allowlist, a `.gitconfig` with a noreply email, a `Cargo.toml`/equivalent that has been scrubbed of `repository`/`author` fields, or a CHANGELOG with explicit anonymization entries. If the project signals "scrub me," the review log must scrub itself the same way.

The principle: **an example illustrating what-not-to-do should never instantiate what-not-to-do.** A review log demonstrating an anonymization defect by quoting the actually-leaked username has reproduced the leak. A review log demonstrating a secrets-management gap by quoting the actual key has reproduced the gap. Abstract the concrete value to a placeholder (`<user>`, `<repo>`, `<email>`, `<key>`, `<path>`) before committing. Keep the *shape* of the evidence (length, position in the line, surrounding bytes) so the finding remains reproducible against the project state, but the *content* is rendered safe to publish.

Suite-level controls help but do not substitute for reviewer judgement: `vsdd-suite/hooks/check-anonymization.sh` (renamed at PR #43 from `check-review-log-anonymization.sh` when consolidated with the prior `no-home-dir-paths` hook and broadened to repo-wide scope) scans **every committed text file** — not just review-log markdown — for the local user's `$HOME`, `git config user.name`, and `git config user.email`, with a public-URL allowlist (`github.com/`, `gitlab.com/`, `bitbucket.org/`, `bsky.app/profile/`, `noreply.*`) so that deliberately-public references pass. A separate hook [`check-external-review-anonymization.py`](../hooks/check-external-review-anonymization.py) enforces identity-correlation discipline on external-author review-log files (handle-slug consistency across declared platforms, name-handle matching, no bare emails in preambles). Both hooks run on commit; passing them does not mean the review log is fully anonymized — only that the most common patterns are absent. Reviewer judgement covers the rest.

---

## Pre-cycle methodology check (capstone + production intent — [AI Engineer R1 F8](../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-21-ai-engineer.md))

**Scope of the pre-cycle discipline ([Review 92](../suite-development/review-log/2026-05-24-suite-review.md#review-92--2026-05-24-0419z) Finding 3; codified 2026-05-24 via operator-policy Path 2 — methodology amendment scoping pre-cycle to compounding-cost cycles):** the pre-cycle methodology check is scoped to **cycles whose cost compounds** — multi-agent IAR cycles (per this section); Phase 5 hardening cycles with cold-session-vs-inline decisions (per [`primers/5-formal-hardening.md`](5-formal-hardening.md) § Cold-session-vs-inline decision rubric); Phase 2a cycles with evidence-shape declarations (per [`primers/2a-red-gate.md`](2a-red-gate.md) § Verifiable git-history check). Other phases (Phase 1a+1b spec authoring per [`primers/1ab-spec-crystallization.md`](1ab-spec-crystallization.md); Phase 1c decomposition per [`primers/1c-decomposition.md`](1c-decomposition.md); Phase 2b implementation per [`primers/2b-implementation.md`](2b-implementation.md); Phase 2c refactor per [`primers/2c-refactor.md`](2c-refactor.md); Phase 4 feedback integration per [`primers/4-feedback-integration.md`](4-feedback-integration.md); Phase 6 convergence per [`primers/6-convergence.md`](6-convergence.md)) are single-author / event-driven / structurally-bounded and do NOT compound cost in the same way — they are intentionally exempt from the pre-cycle declaration discipline. The exemption is a methodology-correctness statement, not an oversight: pre-cycle declaration's value comes from calibrating cycles whose cost would otherwise compound; phases that don't compound don't benefit from the calibration overhead. **Earned-by-recurrence extension trigger:** if a phase-specific evidence-shape need surfaces in 2+ projects (parallel to [Review 91 Finding 1](../suite-development/review-log/2026-05-23-suite-review.md#r91-f1)'s Phase 2a-evidence-shape recurrence pattern), extend the pre-cycle discipline to that phase's primer at that point — the per-phase pre-cycle declaration form would be authored against the specific evidence-shape need rather than as a speculative cascade.

When running a multi-agent IAR cycle (parallel cold-session adversarial review at capstone or production intent; cluster-batching cycles; any cycle with more than 4 agent-spawns), open the cycle's suite-side review-log entry with a **pre-cycle declaration** naming the chosen shape + budget + rate-limit headroom + model selection. The declaration is operator-authored at cycle-spawn time, not retrospective. It exists so the AI Engineer Round-N+1 verification can regression-check actual cost against declared cost — without a declaration, "the cycle reached its target state at the chosen shape" is sunk-cost reasoning, not calibration evidence.

Required pre-cycle declaration fields:

- **Spawn shape:** N agents per round; cluster vs. per-domain; expected total agent-spawns across the cycle. State the adversarial-pair-separation invariant explicitly if cluster-batching ([Security](../domains/role/SECURITY-REVIEW.md) ↔ [Red Team](../domains/role/RED-TEAM-REVIEW.md) and [Technical Writer](../domains/role/TECHNICAL-WRITER-REVIEW.md) ↔ [Documentation Reviewer](../domains/role/DOCUMENTATION-REVIEWER-REVIEW.md) pair-members on different agents).
- **Per-cycle budget:** max-rounds before stop-trigger consultation; max-agents-per-round ceiling; per-cycle estimated token consumption against the intent-tier expected band per [`domains/DOMAIN-INDEX.md`](../domains/DOMAIN-INDEX.md) § Intent calibration § Cold-session budget.
- **Rate-limit headroom:** AI-tool/plan rate-limit-window headroom estimated against the prior usage period — for Anthropic API direct pay-per-token this is the per-organization daily token cap; for Claude Max it is the 5-hour-rolling-window utilization per the [claude-code CLI supplement](../supplements/claude-code-cli.md) § Plan tiers + rate-limit windows; for ChatGPT Plus / Pro it is the per-day message cap; for other tools see the corresponding per-tool supplement. Named fallback plan if mid-cycle rate-limit-hit occurs (retry shape; cache-warmed restart; cluster-shape downgrade).
- **Model selection per task class:** [Opus 4.7](../README.md) for highest-complexity adversarial review (typically Security / Red Team / SA / VDD-IAR Alignment); [Sonnet 4.6](../README.md) for mid-complexity domain reviews (SE / UX / Performance Engineer / Platform Engineer); [Haiku 4.5](../README.md) for mechanical sweep or audit-trail-only passes (project-side stale-citation sweeps; finding-rows registry walks). When the cycle uses a uniform model, name the rationale; when mixed, name the per-cluster mapping. Operators using non-Anthropic tools (Cursor with model selection; ChatGPT; etc.) substitute their tool's per-class model ladder per the corresponding [per-tool supplement](../supplements/).
- **AI tool + plan tier + execution method** (per AI Engineer Dim 14 — [Review 90](../suite-development/review-log/2026-05-23-suite-review.md#review-90--2026-05-23-1200z) Finding 2 / [PR #45](https://github.com/magnificentlycursed/guild-portfolio/pull/45)): explicitly name the AI tool (e.g., `claude-code CLI v0.X`), plan tier (e.g., `Claude Max`, `ChatGPT Plus`, `Claude API direct pay-per-token`), and execution method (e.g., `inline main session`, `4-cluster worktree-isolated parallel cold-session spawn`, `background Bash task`). Determine via verifiable means (process inspection / env vars / CLI features / git config) OR prompt the operator if undeclared. Without this declaration, any per-cycle cost figure is meaningless because the underlying cost model is unknown — a $50 figure is the operator's actual cost on API direct pay-per-token but a would-be-API-cost-comparator-only number on Max plan.
- **Phase-2a-evidence-shape** (per [Review 91](../suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) Finding 1 — Phase 2a Red Gate commit-boundary discipline; see [`primers/2a-red-gate.md`](2a-red-gate.md) § Verifiable git-history check): declare the layer's chosen commit-shape per the primer 2a discipline. Two acceptable values: `canonical two-commit` (Phase 2a Red Gate committed as a standalone commit; Phase 2b implementation as the second commit — the default; CI verifies RED then GREEN) OR `single-commit deviation` (Phase 2a + Phase 2b combined in one commit; requires named rationale + Red Gate failure-evidence preservation pointer + operator-acceptance per the primer 2a § Verifiable git-history check § Single-commit deviation sub-section). A cycle that deviates without pre-cycle declaration is a finding for VDD-IAR Alignment Dim 4 (Red Gate commit precedence) post-hoc; pre-cycle declaration converts the deviation into documented audit trail. The discipline closes the same memory-feedback-insufficient pattern that [Review 91 Finding 1](../suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) named — without the pre-cycle declaration field, the Phase 2a discipline lives only in PROCESS.md prose and fails to propagate forward.

The pre-cycle declaration is closed by an **after-action cost report** in the cycle's audit-trail entry per [`suite-development/suite-development.md`](../suite-development/suite-development.md) § Governing standard for project-level review logs § Cost-tally discipline (capstone+ intent only — AI Engineer R1 F6). The pair (pre-cycle declaration → after-action cost report) is the AI Engineer Dim 13 pre-cycle methodology check applied at the cycle boundary.

**Cost-tally report shape** (per AI Engineer Dim 14 — [Review 90](../suite-development/review-log/2026-05-23-suite-review.md#review-90--2026-05-23-1200z) Finding 4 / [PR #45](https://github.com/magnificentlycursed/guild-portfolio/pull/45) cost-tally plan-tier discipline). Every cost-tally must name:

1. **AI tool** (e.g., `claude-code CLI v0.X`)
2. **Plan tier** (e.g., `Claude Max`, `ChatGPT Plus`, `Claude API direct pay-per-token`); prompt the operator if undeclared
3. **Execution method** (e.g., `inline main session`, `4-cluster worktree-isolated cold-session spawn`)
4. **Model** (e.g., `claude-opus-4-7`, `claude-sonnet-4-6`)
5. **Raw tokens** (estimated; name the basis — sub-agent's reported `total_tokens`, orchestrator's accumulated context, etc.) — this is the **canonical measure** because it is meaningful regardless of plan tier
6. **Would-be API cost** (comparator only) — `if this had been billed at the API tier` with the explicit prefix; do NOT present as the operator's actual cost on a subscription plan
7. **Actual cost to operator** — `$0 marginal (within Max plan limits)` OR equivalent for the operator's plan; for API-direct operators, the would-be-API-cost IS the actual cost
8. **Rate-limit-window utilization** where observable (e.g., `~N% of the 5-hour Max window`)
9. **Wall-clock duration** of any tool-run work (cargo-mutants 5 min; cluster cold-session 10-30 min)
10. **Findings/100k tokens** for cross-cycle comparison; the AI Engineer Dim 2 expected-band lookup uses this metric

The would-be-API-cost framing keeps the dollar figure useful as a cross-plan comparator without misrepresenting the operator's actual cost. The Dim 14 verification check confirms the tool/plan/method declaration is present before the cost-tally is treated as accurate.

**Per-field auditability tier ([Review 91](../suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) Finding 8 — agents cannot count their own tokens).** The 10 fields above split into three tiers by what's measurable from where. Agents authoring inline cost-tallies MUST fill only the agent-self-verifiable tier with hard counts; operator-verifiable and operator-confirmable tiers MUST use `*pending operator …*` placeholders rather than fabricated estimates.

| Tier | Fields | Source |
|---|---|---|
| **Agent-self-verifiable** (countable from this session's tool-call log) | AI tool (1) + Execution method (3) + Model (4) + Tool-call counts by tool name + Files read (with line counts from Read tool returns) + Files written/edited (with line counts from Write/Edit) + Mechanical sweeps run (Bash invocations) + Date (from system context) + Wall-clock anchors via `date -u` Bash invocation per [`supplements/claude-code-cli.md`](../supplements/claude-code-cli.md) § Wall-clock measurement pattern | Tool-call log; system context; Bash `date -u` |
| **Operator-verifiable** (requires `/cost` paste or plan-dashboard inspection) | Raw tokens (5) + Would-be API cost (6) + Rate-limit-window utilization (8) | Operator runs `/cost` in claude-code CLI (or equivalent in other tools) and pastes output as cost-tally append-only addendum |
| **Operator-confirmable** (operator-declared per session, NOT inherited from prior context) | Plan tier (2) + Actual cost to operator (7) | Operator-declared at session-start or re-declared per session; do NOT inherit silently from prior conversation context |
| **Derived (computable only when all inputs measured)** | Findings/100k tokens (10) | Computed from operator-verifiable raw tokens + finding count; mark `NOT COMPUTABLE — pending operator /cost paste` when raw tokens unmeasured |

**Hard rule:** agents inline-authoring a cost-tally **MUST NOT fabricate** operator-verifiable or operator-confirmable fields. The bare number "~90-110k tokens" without an instrumentation basis is fabrication, not estimation. The discipline is: count what is countable from the agent's tool-call log; defer the rest to the operator-paste addendum.

**Operator-action queue line** required in every inline cost-tally:

> **Operator-action queue:** if cost-tally precision is load-bearing for cross-cycle calibration, operator runs `/cost` in this session and pastes the output here as an append-only addendum, replacing the *pending operator …* placeholders with measured values.

The auditability tier was codified at [Review 91](../suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) after the Review 91 cost-tally itself committed the fabrication failure mode (a "~90-110k tokens" and "~$4-6 USD" estimate authored without instrumentation source). The rewrite of Review 91's cost-tally per the tiered shape is the canonical worked example.

## Tuning levers (cost-performance optimization) ([Review 91](../suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) Finding 10)

The cost-tally above is the measurement surface; the tuning levers below are the optimization surface. Per the [Performance Engineer](../domains/role/PERFORMANCE-ENGINEER-REVIEW.md) Dim 5/6/8/10 lens applied to AI-agent cost (and the [Platform Engineering](../domains/role/PLATFORM-ENGINEER-REVIEW.md) Dim 23/26/36 observability lens):

**Levers (per-cycle optimization decisions operators + agents make):**

1. **Model-tier right-sizing** — per [AI Engineer Dim 6](../domains/role/AI-ENGINEER-REVIEW.md): [Opus 4.7](../README.md) for highest-complexity adversarial review (typically Security / Red Team / SA / VDD-IAR Alignment / AI Engineer); [Sonnet 4.6](../README.md) for mid-complexity (SE / UX / Performance Engineer / Platform Engineer / TW / Doc Reviewer / QE); [Haiku 4.5](../README.md) for mechanical sweep + audit-trail-only passes. **Cost delta: Haiku 4.5 is ~10x cheaper than Opus 4.7 per million tokens; a mechanical sweep on Opus is right-sizing failure.** Post-cycle review: did each spawn match its declared task class?
2. **Prompt-cache discipline** — per [AI Engineer Dim 3](../domains/role/AI-ENGINEER-REVIEW.md) + [`supplements/claude-code-cli.md`](../supplements/claude-code-cli.md) § Prompt-cache discipline: Anthropic prompt-cache TTL is 5 minutes. Sub-agent spawn batches that exceed 5 min between agents miss cache entirely; cost differential between cached and uncached input is ~10x. **Lever: schedule sub-agent spawns within the 5-min window OR accept the cache-miss cost as a documented tradeoff.**
3. **Cluster-batching shape** — per [AI Engineer Dim 7](../domains/role/AI-ENGINEER-REVIEW.md): per-domain spawn (1 agent per domain; N agents per round) vs cluster-batching (4 clusters with adversarial-pair separation, multi-domain per agent). **Cluster-batching reduces per-cycle agent count from ~13 to 4 at capstone intent — ~3x token reduction at the orchestrator-overhead level.** Adversarial-pair-separation discipline is mandatory: Security ↔ Red Team and TW ↔ Doc Reviewer pair-members must land on different agents.
4. **Sub-agent scope-down** — per [AI Engineer Dim 4](../domains/role/AI-ENGINEER-REVIEW.md): sub-agent should receive only the scope it needs, not the full project context. **Lever: hand sub-agent a focused prompt + a small file list rather than "read everything and tell me what's wrong."** Avoids re-loading context the orchestrator already had.
5. **N+1 sub-agent file-reread** — per [PerfEng Dim 5](../domains/role/PERFORMANCE-ENGINEER-REVIEW.md) applied to agents: cold sub-agents re-read files the orchestrator already loaded. **Lever: warm-context handoff — orchestrator extracts the relevant file slices into the sub-agent's prompt rather than handing it a Read-this-file directive.** Currently no codified handoff pattern; see [F11](../suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) for the subsystem-buildout proposal.
6. **Cycle-stop discipline** — per [`primers/3-review-session.md`](3-review-session.md) § Round triggers § Stop trigger: a round that produced only Hallucinated findings should NOT trigger Round N+1 without explicit director justification. **Lever: avoid running rounds whose only justification is "cold-batch infrastructure is available and one more pass is cheap."** Cost is not the criterion; new-evidence is.

**Rolling-baseline measurement requirement (Open across cycles; depends on operator-paste cost data per [Finding 8](../suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) + [Finding 9](../suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) Shape 1 cost-observability infrastructure):**

The cost-tally fields ALONE are insufficient for tuning-lever evaluation. A per-cycle cost figure tells you what THIS cycle cost; tuning-lever evaluation requires a **rolling baseline** — the median cost-per-finding (or cost-per-layer-gate-close) across the last N cycles, against which the current cycle is compared. The baseline supports the [PE Dim 36](../domains/role/PLATFORM-ENGINEER-REVIEW.md) "performance budget enforced in CI" discipline applied to cost: declared budget per cycle, gated by enforcement, regression-flagged when out-of-band.

**Anomaly-detection threshold (proposed; codification pending Shape 1 infrastructure):** flag cycles whose cost-per-finding is **3x the rolling-median** (a heuristic 3-sigma proxy at small N). The flag triggers a post-cycle PE-owned review naming which tuning lever (or which process inefficiency) explains the anomaly.

**Cross-cycle dashboard (Open per [Finding 9](../suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z); depends on Shape 1 JSON observability files):** `vsdd-suite/suite-development/COST-OBSERVABILITY.md` (generated rollup) aggregates per-cycle cost from the operator-pipeline JSON files into a trend view. Currently does not exist; deferred to Shape 1 infrastructure work.

**Cycles exempt from the pre-cycle declaration:**

- Single-agent rounds (one cold-session per domain, run serially) — the operator can make the spawn decision inline without compounding cost risk.
- Sub-agent delegation from a main session where the sub-agent's scope is the work product, not adversarial review (e.g., the main session delegates a mechanical sweep to a Haiku 4.5 sub-agent).
- Learning-exercise intent — most learning-exercise projects use serial single-agent review by default; the discipline overhead doesn't match the assignment bar.

## Before starting a domain review

**If DESIGN.md does not exist:** Stop. The absence of a design document is itself a finding for VDD-IAR Alignment dim 1. Do not proceed with other domain reviews — there is no spec to evaluate against. Log the absence and wait for the spec to exist.

**If DESIGN.md exists:**

1. Read DESIGN.md in full. This is the contract. Every domain review evaluates the implementation against it.
2. Read all source files. Do not skim. A defect in an unread file is a defect you missed.
3. Read the prior IAR log for this layer, if one exists. Do not re-raise findings already resolved and verified. Do raise findings dismissed without adequate rationale.
4. Load one domain prompt per session. See sequencing guidance below.

## Domain selection

Active domains for this project: *(list the domains active for this project, from DESIGN.md or the project task file. See `domains/DOMAIN-INDEX.md` for activation criteria.)*

**Default sequencing** (run in parallel unless a dependency applies):

- Run SA first when there are significant structural or architectural changes
- Run Security before Red Team — Security ensures controls exist; Red Team verifies they hold
- Run QE before UX when QE finds bugs that change the implementation
- Run VDD-IAR Alignment last in the merge gate

For complete sequencing guidance, see `README.md`.

## Session isolation

Run one domain per session. An AI agent that reviews multiple domains in one session accumulates context that softens adversarial pressure — the agent begins reconciling findings across domains rather than applying fresh adversarial pressure to each. Parallel independent sessions are the gold standard.

If batching domains in one session is unavoidable, treat it as a quality tradeoff and note it in the review log.

### Dispatch options

**Manual dispatch (default; required for highest-stakes reviews).** Open one fresh chat session per domain. Paste this primer. Load that single domain's prompt. Run the review. Close the session before opening the next domain. This is the form the primer was written for, and the form to use when adversarial pressure is the bottleneck — a human driver reading findings as they arrive applies pressure the dispatcher cannot.

**Swarm dispatch (Phase 2+ projects, lower-stakes or volume passes).** `crosslink swarm review --agents <N> [--mandate adversarial] [--doc <path>] [--file-issues]` launches N parallel adversary agents, each in its own worktree with hard context isolation by construction. The shape matches the gold standard (one agent per domain, no shared context); the tradeoff is rhythm — findings arrive as an aggregated batch rather than interactively. Choose this dispatch when running a routine refinement round; choose manual dispatch when a layer is close to MVR and the marginal finding matters most. With `--file-issues`, each finding becomes a crosslink issue (default label `review-finding`) that Phase 4 (`primers/4-feedback-integration.md`) routes; with `--fix`, the swarm then dispatches one fix agent per filed issue (only do this for findings classified Resolved by a human — `--fix` short-circuits the routing step and should not be used while findings are still in the classification queue).

The human-in-the-loop requirement is identical under both dispatch options: every finding must be classified by a human. Swarm dispatch parallelizes the *adversary*, not the *classifier*.

## Round triggers (continue / stop)

The refinement loop is governed by triggers, not a default round count. Before opening Round N for a layer, verify *which* trigger applies. The triggers are forward-only: a continue trigger requires Round N+1 to run; a stop trigger requires explicit justification to run another round.

### Continue trigger (G-131) — Round N+1 is mandatory

Round N produced any new real findings, including any of:

- A finding classified Resolved (the fix's regression test additions in Round N close one finding; the Round N+1 cold pass verifies the fix held and looks for adjacent defects the fix may have created)
- A finding surfaced by director manual testing (ITC L6 R3 SO R22 is the canonical example — director's manual execution of the "delete highest-id, create" sequence caught a spec violation that 11 cold-batch IAR domain reviews missed)
- A finding surfaced by regression replay (a prior layer's adversarial reproducer re-run against the current binary that surfaces a regression)
- A finding routed to a future layer (Deferred) — Round N+1 verifies the Deferred-with-named-trigger discipline is intact and the routing is correct
- A Raised-to-SO finding adjudicated mid-round — Round N+1 includes the SO log entry and any spec amendment as a Round-N+1 artifact

The "any new real findings" framing is deliberate. A single Resolved finding in Round N triggers Round N+1 — the cost of one additional round is much smaller than the cost of merging with an undetected adjacent defect.

The layer is at MVR when the round *after* the last new-finding round produces only Hallucinated findings or no findings.

### Stop trigger (G-151) — Round N+1 should NOT run by default

Round N produced only Hallucinated findings or no findings. **MVR is reached.** Running Round N+1 from this state requires explicit director justification — name the specific new evidence or new attack surface that emerged since Round N closed. Acceptable justifications:

- A new layer's IAR exposed a cross-layer concern that would invalidate the current layer's MVR
- An upstream dependency (language version, library, framework) changed in a way that affects the current layer
- A director-raised observation from manual testing or post-MVR exploration that fits the continue trigger above (in which case the continue trigger applies and the layer was not actually at MVR — re-classify Round N as a new-finding round)

**Not acceptable justifications:**

- "Cold-batch infrastructure is available and one more pass is cheap" — cost is not the criterion; new-evidence is
- "Adding more rounds feels more thorough" — over-investment is methodological drift, not value-add
- "Other layers ran N+1 rounds; this layer should too" — round count is a function of finding progression, not a target

The pre-round check: **What new evidence triggers this round? If the answer is 'none — Round N closed at MVR,' do not open Round N+1.** This check fires the sycophancy-guard for the loop itself — an AI orchestrator that defaults to "run another round, the methodology supports it" without checking the trigger is operating the methodology as theatre, not as a discipline.

### Intent-keyed sensitivity (cross-reference G-150)

Project intent (per `../templates/DESIGN-template.md` § Project intent) calibrates the stop-trigger's strictness:

- **learning-exercise** intent: stop-signal sensitivity *high* — when in doubt, stop; the cost of one missed defect is low relative to process-drift fatigue cost
- **portfolio** intent: standard — apply the trigger discipline as stated above
- **capstone** intent: standard — same as portfolio plus the additional gate criteria from `../domains/DOMAIN-INDEX.md` § Intent calibration
- **production** intent: stop-signal sensitivity *strict* — MVR must be unambiguous before merge; ambiguous "could be one more thing" runs MAY proceed with documented justification (the deferral discipline tightens to require a named target layer or auto-Backlog trigger)

## After each domain review

Before ending the session, classify every finding. Valid classifications vary by domain — the domain file's `## Current Review Prompt` section is authoritative. The full universe of classification types:

- **Resolved** — fix applied and verified in this session
- **Dismissed** — no action taken; rationale documented (not "not applicable" — explain specifically why)
- **Hallucinated** — the adversary invented a problem that does not exist; document specifically why the control holds or the concern does not apply
- **Accepted risk** — Security, Red Team, and Privacy only; explicit rationale and named risk owner required
- **Deferred** — most role domains; scheduled for a specific future layer; reason given. **Not valid for Security, Red Team, or VDD-IAR Alignment** — security findings are not deferred; VDD-IAR Alignment process findings are binary (either the process ran or it didn't)
- **Backlogged** — Solution Owner only; out-of-scope item preserved for future consideration
- **Approved deviation** — Solution Owner only; explicit stakeholder approval documented
- **Accepted deviation** — Accessibility only; WCAG exception documented with rationale
- **Accepted limitation** — Performance Engineer only; explicit trade-off rationale required
- **Accepted scope** — Localization only; single-locale scope documented in DESIGN.md
- **Demonstrated / Partial / Absent** — Portfolio Assessment only (replaces resolved/dismissed for assessment findings)

A session that ends with unclassified findings has not completed the review. Log round number (`QE Review 1`, `Security Review 2`) and the finding progression — moving from real findings to hallucinated findings is evidence the process worked.

## Round closing — Phase 4 routing closing field (R94 F1 closure)

Every Phase 3 round entry's closing block must include a `**Phase 4 routing:** <reference>` field naming where this round's routable findings were routed per [`4-feedback-integration.md`](4-feedback-integration.md). Per [`../suite-development/suite-development.md` § Layer-gate close criteria](../suite-development/suite-development.md#layer-gate-close-criteria-processmd-retrospective-discipline) criterion 8: routing is per-round (every round gets its own routing record), not per-layer; a layer that closes IAR at Round N must have N routing records.

Valid values:

- **Reference to the per-domain Phase 4 routing appendix** in this entry's file — the canonical primer-4-shape post-bookmark-cli-manual-PR-#52: each per-domain review log carries `## Phase 4 routing — Round N` appendices for that domain's routable findings. Example: `**Phase 4 routing:** see § Phase 4 routing — Round 1 below`.
- **`*(no routable findings)*`** placeholder — for rounds that produced only Hallucinated findings (or only Resolved-in-session findings with no out-of-domain routing required). The placeholder structurally records that routing was considered, not skipped.
- **Cross-reference to a consolidated routing record** — only for legacy entries authored before the per-domain-appendix shape became canonical (2026-05-25). New entries use the per-domain appendix shape.

The `**Phase 4 routing:**` field is enforced by [`../hooks/check-suite-review-preamble.py`](../hooks/check-suite-review-preamble.py) for entries dated 2026-05-26 or later. The forward-only threshold lets the bookmark-cli-manual PR #52 cycle's Round 1/2/3 entries (already-merged historical records) stand without retroactive amendment.

## Source attribution (G-133 / Review 68 Finding 11)

Every per-review entry's preamble must include a `**Source:**` line declaring how this round's findings were elicited. Per `suite-development/suite-development.md` § Per-review entry preamble, the valid values are: `domain-raised` (the cold adversary applying the domain's dimensions found the finding — the default for this primer's normal use), `director-raised` (the operator's manual testing / post-MVR exploration / non-domain-prompt-driven adversarial pass found the finding), `regression-replay` (a prior layer's adversarial reproducer re-run against the current binary), `external-feedback` (an upstream stakeholder / project consumer / methodology author surfaced the finding through prose feedback), or `mixed` (the round's findings span more than one source — name the sub-disposition explicitly).

This primer's typical output is `domain-raised` — the cold adversary applying domain dimensions is exactly what this primer establishes. If your session deviates (e.g., the operator interrupts mid-round with a director-raised observation that becomes a finding; you re-run a prior reproducer that surfaces a regression), classify per the actual elicitation path, not per the primer-implied default. The Source field is the audit-trail granularity input to Portfolio Assessment dimensions on developer participation; a session that defaults silently when the actual elicitation was director-raised degrades the audit signal.

## If reviewing the IAR suite itself

When the review target is the suite itself (not a project under review):

- Findings are recorded in `vsdd-suite/review-log/YYYY-MM-DD-suite-review.md` — create the file if no entry exists for the date, or append to it if one does. Suite reviews are a single artifact type; the mode (defect-search vs. registry-walk) lives in the entry's Lens field.
- Add a corresponding summary row to the **Suite Reviews** table in `vsdd-suite/suite-development/SUITE-DEVELOPMENT-REVIEW.md`. The index is read first by future reviewers; an unindexed session is invisible.
- New findings registered for tracking also need a row in `suite-development/FINDINGS-INDEX.md` (forward-only section, identified by their `Review N Finding M` anchor — no new ID prefix; the legacy `G-` series is closed) linking to the new session entry.
- See the **Suite review entry format** section in `primers/../suite-development/suite-development.md` for the required entry structure.


## Three-audience lens

This review-session primer serves all three audiences of the [three-audience design principle](../suite-development/suite-development.md#three-audience-design-principle-review-80-finding-3) ([Review 80](../suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z) Finding 3; renamed in [Review 84](../suite-development/review-log/2026-05-21-suite-review.md#review-84--2026-05-21-1100z) Finding 4):

- **Suite developers** evolving this primer treat the prose as the methodology-authoring surface for Phase 3 IAR review — changes here are methodology shifts requiring their own Review.
- **Suite users** running a session against this primer treat it as the canonical step-by-step for Phase 3 IAR review on their own project; the completion criteria are what their next layer-gate or phase-close commit is checked against.
- **AI agents** loaded with this primer as cold-session context treat it as the spec for the session's authoring shape (file locations, classification vocabulary, the audit-trail entries this session produces); the primer's named artifacts + their schemas are the agent-API contract for what the session writes.

See [`../suite-development/suite-development.md`](../suite-development/suite-development.md) [§ Three-audience design principle](../suite-development/suite-development.md#three-audience-design-principle-review-80-finding-3) for the full discipline.
