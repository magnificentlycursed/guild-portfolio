# claude-code CLI — AI-Engineering Supplement

**Tool surface:** [claude-code CLI](https://claude.com/claude-code) — Anthropic's official Claude-as-coding-agent CLI.

**Authored:** [Review 90](../suite-development/review-log/2026-05-23-suite-review.md#review-90--2026-05-23-1200z) (PR [#45](https://github.com/magnificentlycursed/guild-portfolio/pull/45)) — first per-tool supplement under the supplements/ directory's parallel-to-per-language-and-per-interface pattern (per-language: `rust.md`, `python.md`, `bash.md`; per-interface: `github-actions.md`, `markdown.md`, `yaml.md`, `toml.md`; per-tool: this file). This supplement is loaded by the AI Engineer domain when Dim 14 (Tool / plan / execution-method identification) determines the operator is using claude-code CLI specifically.

**Other per-tool supplements** that may warrant authoring as adoption surfaces them: Cursor (`cursor.md`); Aider (`aider.md`); Codex CLI (`codex-cli.md`); Claude API direct pay-per-token (`anthropic-api-direct.md`); ChatGPT Plus / Pro / Enterprise (`chatgpt.md`). The supplements share a body shape (plan tiers + rate-limit window + cache TTL + per-tool token costs + execution-method semantics + known issues + canonical optimization patterns) so the AI Engineer domain's findings + recommendations remain tool-portable.

---

## Plan tiers + rate-limit windows

Claude-Max plan operators (the canonical first-adopter cohort for this suite) experience cost-and-quality tradeoffs **fundamentally differently** from Anthropic API direct pay-per-token operators. The AI Engineer domain's Dim 2 (Token economy per finding) expected band of 100k–300k tokens/finding for capstone intent was calibrated against API pay-per-token; for Claude Max it should be interpreted as the **rate-limit-window utilization equivalent**, not as a dollar measure.

**Plan-tier rate-limit windows** (verify against the current Anthropic public pricing page; values below are approximate as of 2026-05-23):

| Plan tier | Marginal per-spawn cost | Rate-limit shape | Cycle implications |
|---|---|---|---|
| **Free** | $0 (within limits) | Very small daily message cap; hard reset | Not suitable for capstone-tier IAR cycles (4+ parallel agents would exhaust the cap mid-cycle); learning-exercise intent only |
| **Pro** | $0 (within limits) | Daily message-cap with 5-hour rolling window | Suitable for portfolio-intent single-agent work; capstone cycles risk window-cap exhaustion under cluster-batching shapes (4+ parallel agents in a Round) |
| **Max 5×** | $0 (within limits) | 5x Pro's 5-hour-rolling-window cap | Suitable for capstone-intent under careful pacing (one Round per 5-hour window); production-intent requires headroom checks |
| **Max 20×** | $0 (within limits) | 20x Pro's 5-hour-rolling-window cap | Suitable for capstone + production-intent including multi-round same-window cycles |
| **API direct pay-per-token** | Variable; tokens × per-model rate | No subscription-tier rate-limit-window; rate limits are tokens-per-minute organization-wide | The original "$X per finding" framing applies directly — but cluster spawns of N parallel agents are now N×-cost, not free |

**Implication for AI Engineer cost-tally findings:** the per-finding budget the domain prompt names (100k–300k tokens for capstone) is the RAW token measure; for Max plan operators the binding constraint is the 5-hour-rolling-window cap rather than the dollar conversion. If a cluster cycle exceeds 50% of the operator's window utilization, the AI Engineer finding shape is "rate-limit headroom risk", not "over-investment dollar cost".

**Plan-tier identification (verifiable means):** the `claude` CLI does not expose plan tier as a `--version` field; the operator must declare it explicitly during the pre-cycle methodology check (Dim 13 / Dim 14). If the operator's plan is undeclared, prompt them. Approximate verifiable signals: the `/cost` command (if present) shows session-cumulative API-equivalent cost — useful as a comparator but not the operator's plan tier directly.

---

## Prompt-cache discipline (the 5-minute TTL)

Anthropic's prompt cache lives for 5 minutes after the last cache-write. Cached input is approximately **10% of the uncached cost** at the API tier; for Max-plan operators the cache hit reduces rate-limit-window utilization by a similar factor.

**Cluster-spawn implications** for claude-code CLI:

- **Parallel cluster spawns** (4+ agents in one `<message>` containing multiple `Agent` tool_use blocks) share a cache window — the first agent's context-load primes the cache, subsequent agents in the same window hit. Per the AI Engineer Dim 3 framing, this is the cache-eligible-reuse pattern.
- **Sub-agent worktree isolation** does NOT bust the cache — the parent's context that the sub-agent inherits is the same input regardless of worktree path. Per the [Agent tool docs](https://docs.anthropic.com/en/docs/claude-code/agent-tool) (verify against current release): worktree isolation gives the sub-agent a separate filesystem-checkout, not a separate cache.
- **Sequential agent spawns more than 5 minutes apart** are cache-miss (each pays full context-load cost). This is the cache-bust failure mode AI Engineer Dim 3 names.
- **Background `run_in_background` agents** are independent cache windows — the parent task continues + the background task's first read pays full context-load cost (no warm cache from the parent unless the background task happens to be spawned with substantially overlapping context).

**The /cost command** — if claude-code CLI exposes `/cost`, it shows the session's API-equivalent cost. For Max-plan operators this is the **would-be API cost** comparator, not the actual cost. The number is useful for cross-plan reasoning ("if I were billed at the API tier, this cycle would have cost $X"); do not present it as the operator's actual cost in cost-tally findings.

---

## Per-tool token costs

Each tool invocation in claude-code CLI has its own input-pass cost (the system prompt + accumulated conversation context + tool-specific framing). Approximate token costs per tool invocation (verify against current behavior; values are first-pass estimates as of 2026-05-23):

| Tool | Per-invocation token cost (approximate) | Notes |
|---|---|---|
| `Read` | Low (~1k context + file content) | Most efficient way to consume file content |
| `Edit` | Low (~1k context + old-string + new-string) | Preferred for surgical edits |
| `Write` | Medium (~1k context + full content) | Use for new files OR full rewrites; does NOT go through shell-quoting like heredocs |
| `Bash` | Low to medium (~1k context + command + output) | Most common tool; cost scales with output volume |
| `Agent` (foreground) | High (~2k context + spawn prompt) + sub-agent's full cost | Sub-agent's full token consumption rolls up to operator's plan budget |
| `Agent` (background) | Same as foreground but parent continues | Useful for long-running tasks (cargo-mutants; cargo build); no parent context bloat |
| `Agent` (worktree-isolated) | Same as foreground + git-worktree overhead | Use when sub-agents need to commit independently; race-free for parallel writes |
| `Grep` | Low (~1k context + pattern + match output) | Most efficient way to find content across files |
| `TaskCreate` / `TaskUpdate` / `TaskList` | Minimal (~500 tokens) | Persistent task state; near-free |
| `Skill` | Variable (depends on the skill's loaded content) | Skill metadata loads on first call; cached for the session |

**The orchestrator's own context cost is NOT zero.** Every sub-agent spawn includes the orchestrator's accumulated context in the spawn prompt — if the main session has 100k tokens of conversation, that's 100k tokens added to each sub-agent's context-load (unless the spawn prompt is short-form and intentionally pares down the inherited context). The AI Engineer Dim 4 framing ("sub-agent delegation patterns") includes the orchestrator's cost: a finding that "the cluster spawn was efficient because each cluster used only ~50k tokens" misses the orchestrator's ~30k cost per spawn (4 spawns × 30k = 120k orchestrator overhead).

---

## Execution-method semantics

claude-code CLI exposes several execution methods that each have distinct cost + concurrency + race semantics:

- **Inline main-session work** — the orchestrator does the work in the active conversation. Lowest overhead; no spawn cost. Use for: small edits; targeted reads; methodology authoring; analysis that benefits from full conversation context.
- **Foreground sub-agent** (`Agent` tool with `run_in_background: false`) — sub-agent runs to completion; parent waits. Use for: work that blocks subsequent steps; bounded-time tasks (<5 min).
- **Background sub-agent** (`run_in_background: true`) — sub-agent runs while parent continues. Parent receives a completion notification. Use for: long-running tasks (cargo-mutants ~5 min; cargo build > 1 min; parallel cluster spawns where the orchestrator can interleave other work).
- **Worktree-isolated sub-agent** (`isolation: "worktree"`) — sub-agent runs in a temporary git worktree. Use for: parallel cluster spawns where each sub-agent commits independently; race-free for cluster shapes where multiple sub-agents touch overlapping files (though the canonical pattern is each sub-agent touches non-overlapping per-domain log files).
- **Bash `run_in_background`** — Bash command runs in the background; parent receives a completion notification when the command exits. Use for: polling loops (`until <check>; do sleep 30; done`); long shell commands; building artifacts in parallel with other work.

**Concurrency note:** parallel sub-agent spawns share the orchestrator's plan-tier rate-limit window. 4 parallel agents in one cycle Round consume ~4× the window utilization of a sequential equivalent. For Max-plan operators, this is the binding constraint, not the dollar cost.

---

## Known Issues

### Parser-aborted on heredoc-based file writes via the Bash tool

**Symptom:** Operator's downstream tooling reports "parser aborted" mid-response on `cat <<'EOF' ... EOF` invocations passed through the Bash tool. The file write itself succeeds (verifiable via `Read` or `wc -l` post-hoc) but the response transport stops streaming the rest of the response.

**Three observed instances in PR [#44](https://github.com/magnificentlycursed/guild-portfolio/pull/44)** (Layer 2 capstone cycle, 2026-05-22 to 2026-05-23 UTC). All three were appending 100-200 line review-log entries; all three contained markdown with embedded backticks, fenced code blocks, em-dashes, bold markdown, and Unicode (em-dash, ≥, ≤, ✓). The file writes were confirmed successful on disk in all three cases.

**Hypothesized triggers:** (a) response size threshold; (b) embedded backticks / `EOF` near-collisions in heredoc body; (c) Unicode characters; (d) line-count threshold; (e) interaction with the operator's specific downstream tooling (terminal multiplexer, IDE bridge, etc.).

**Mitigation (operative; sufficient to stop the recurrence within a session):**

1. **Prefer the `Write` tool** over `cat <<'EOF'` for new-file authoring. `Write` is a structured JSON parameter, not shell-quoted heredoc; it doesn't go through the same transport path that has been failing.
2. **Prefer multiple smaller `Edit` ops** over one large heredoc for appending content. Edit ops are surgical and go through the same structured JSON path as Write.
3. **When heredoc is necessary** (e.g., for short config-file creation), keep the content ≤30 lines AND avoid markdown bold + em-dash patterns + extensive embedded backticks in the body.
4. **Verifiable-means check:** if a parser-aborted is suspected, verify the file state via `Read` or `wc -l` BEFORE assuming the write failed. In all three observed instances, the file had written successfully and only the response transport aborted.

**Codified workflow rule for claude-code CLI orchestrators:** NEVER use heredoc for content > 50 lines OR containing markdown bold + em-dash + extensive embedded backticks. Use `Write` for new files; `Edit` for appending or surgical changes. The `cat <<EOF` shell pattern is reserved for short config-file creation (≤30 lines, no markdown).

### Lettering-violation recurrence in cluster-spawn prompts

**Symptom:** When generating spawn prompts for parallel sub-agent clusters, the orchestrator defaults to letter-based labels (`Cluster A`, `Cluster B`, `Cluster C`, `Cluster D`) for token economy in the prompt text. The operator's [feedback memory](https://github.com/anthropics/claude-code) explicitly bans letter-based labels for methodology concepts; recurrence has happened 3+ times across sessions.

**Mitigation (operative):** pre-spawn check — before invoking parallel `Agent` calls, write the composition-based label for each cluster explicitly in the message immediately preceding the parallel-agent invocation. Example: "SE/UX/Performance-Engineer cluster" / "QE/Security/Technical-Writer cluster" / "Solution-Architect/Red-Team/Platform-Engineer cluster" / "Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster". If the orchestrator finds itself reaching for "Cluster A" for token economy, that's the bias to resist — the composition-based label adds clarity at no token-cost to the sub-agent.

**Escalation path if a fourth recurrence happens:** a pre-commit hook scanning spawn-prompt-pattern files for letter-only cluster labels. Specification TBD; see [Task #56 Finding 1](../suite-development/review-log/2026-05-23-suite-review.md#review-90--2026-05-23-1200z).

---

## Canonical optimization patterns

These patterns apply specifically to claude-code CLI orchestrators using Claude Max + sub-agent worktree-isolated cluster spawns (the suite's canonical capstone-cycle shape):

### Cluster-batching with adversarial-pair separation (4-cluster shape)

For a 13-domain capstone-active set, the canonical 4-cluster shape (Cluster names by composition, not letter):

- **SE/UX/Performance-Engineer cluster** — code shape + UX surface + perf budget
- **QE/Security/Technical-Writer cluster** — test discipline + threat surface + clone-and-follow docs
- **Solution-Architect/Red-Team/Platform-Engineer cluster** — purity boundary + adversarial + CI/install
- **Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster** — purpose + cold-reader docs + AI-meta + methodology alignment

Adversarial pairs are separated: SE↔QE, Security↔Red Team, TW↔Documentation Reviewer.

### Background-task interleaving

Long-running tasks (cargo-mutants ~5 min; cargo build > 1 min; cargo install + manual-test sequence ~3 min) should use Bash `run_in_background` so the orchestrator can interleave authoring work + small edits during the wait. The orchestrator receives a completion notification when the background task exits; no polling required.

### Prompt-cache-warm cycle planning

Schedule sequential agent spawns within the 5-minute cache TTL where possible. If a cycle requires more than 5 minutes between spawns, accept the cache-miss cost OR re-architect to parallel spawns within one cache window.

---

## Cost-tally discipline (per primer 3 § Cost-tally)

When authoring a Review entry's cost-tally on claude-code CLI:

1. **AI tool**: `claude-code CLI v<version>` (run `claude --version` to confirm; check the [release notes](https://github.com/anthropics/claude-code/releases) for the current version).
2. **Plan tier**: prompt the operator if undeclared; the CLI does not expose plan tier as a verifiable field.
3. **Execution method**: name the specific shape (inline main session; foreground sub-agent; background sub-agent; worktree-isolated cluster spawn; background Bash task).
4. **Model**: `claude-opus-4-7` / `claude-sonnet-4-6` / `claude-haiku-4-5-20251001` per the [model ID table in the system prompt](https://docs.anthropic.com/en/docs/about-claude/models).
5. **Raw tokens (estimated)**: name the basis for the estimate (sub-agent's reported `total_tokens`, the orchestrator's accumulated context, etc.).
6. **Would-be API cost (comparator only)**: report as "if this had been billed at the API tier" with the explicit prefix; do NOT present as the operator's actual cost on Max plan.
7. **Actual cost to operator**: `$0 marginal (within Max plan limits)` OR `rate-limit-window utilization signal: ~N% of the 5-hour window` where observable.
8. **Wall-clock**: tool-run duration (cargo-mutants 5 min; cargo build 30s; cluster cold-session 10-30 min wall-clock for the slowest agent).

The "would-be API cost" framing keeps the dollar figure useful as a cross-plan comparator without misrepresenting the operator's actual cost.

### Agents cannot count their own tokens ([Review 91](../suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) Finding 8)

**Hard rule:** the agent (this Opus 4.7 session, or any sub-agent) has NO instrument to count tokens, observe cache-hit ratio, observe rate-limit-window utilization, or compute would-be API cost. Numbers in those fields without an instrumentation source are fabricated, not estimated. Per [`primers/3-review-session.md`](../primers/3-review-session.md) § Cost-tally report shape's per-field auditability tier (Review 91 Finding 8 codification):

- **Agent-self-verifiable (countable from this session's tool-call log):** AI tool, model, execution method, tool-call counts by tool name, files read with line counts, files written/edited with line counts, mechanical sweeps run (Bash invocations).
- **Operator-verifiable (requires `/cost` paste or plan-dashboard inspection):** raw tokens, cache-hit ratio, would-be API cost, rate-limit-window utilization.
- **Operator-confirmable (operator-declared per session, NOT inherited from prior context):** plan tier, actual marginal cost.

When authoring a cost-tally inline:
1. **Fill agent-self-verifiable fields with hard counts** — count tool calls from the conversation's tool-call log; sum file-read line counts from the Read tool returns; etc.
2. **Mark operator-verifiable fields `*pending operator /cost paste*`** — do NOT estimate; estimates are fabrications.
3. **Mark operator-confirmable fields with the operator's declaration source** — name the specific message or memo, not "inherited from prior context."
4. **Add an Operator-action queue line** — "operator runs `/cost` in this session and pastes the output here as an append-only addendum, replacing the *pending operator …* placeholders with measured values."

The cost-tally then becomes a **two-author artifact**: agent fills the substrate-measurable section inline; operator (or hook) fills the instrumentation-required section separately. This avoids the fabrication failure mode that Review 91 Finding 8 surfaced.

### Wall-clock measurement pattern ([Review 91](../suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) Finding 15)

The agent CANNOT observe elapsed wall-clock between tool calls or session-start. The agent CAN invoke `date -u` via the Bash tool at session boundaries.

**Pattern:**

1. **Session-start anchor** — at the start of the cycle's review-log authoring, invoke `date -u +%Y-%m-%dT%H:%MZ` via Bash. Record the output in the review's preamble as `**Session-start (Bash `date -u`):** 2026-MM-DDTHH:MMZ`.
2. **Session-end anchor** — before authoring the cost-tally, invoke `date -u +%Y-%m-%dT%H:%MZ` again. Record as `**Session-end (Bash `date -u`):** 2026-MM-DDTHH:MMZ` in the cost-tally Wall-clock field.
3. **Elapsed** — subtract the two anchors; record as `**Wall-clock elapsed:** HHhMMm (Bash-instrumented; agent did NOT count time between tool calls; gaps include operator-discussion intervals + idle periods + tool execution time + agent authoring time, in unknown proportions).`
4. **Honest framing** — the elapsed figure is wall-clock elapsed time, NOT agent-active time. A 7-hour session may have 2 hours of agent-active authoring + 5 hours of operator-discussion-and-idle; the agent has no signal to decompose them. Name this limitation explicitly when the elapsed figure is reported.

**Failure mode this pattern defends against:** the [Review 91 cost-tally pre-rewrite](../suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) named "Wall-clock: ~45-60 minutes (single continuous suite-audit session)" — the actual elapsed (captured post-rewrite via this pattern) was ~7h43m, a 16x discrepancy. The fabricated "feels like" estimate was 16x off from the instrumented measurement; the pattern eliminates the fabrication while keeping the agent-honest limitation visible.

### Available observability surface for agents (and what's missing)

Per [Review 91](../suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) Finding 9 + [`claude-code-contract.md`](../claude-code-contract.md): the claude-code CLI's current agent-observable surface is small. Enumeration:

**Agent-observable now:**

- System context fields (date, model, working directory, OS, shell) — read at session-start
- Tool call shapes + return values (Read returns numbered lines; Bash returns stdout/stderr/exit) — countable per call
- File system state (Read existing files; Bash `ls`, `wc -l`, `find`) — directly measurable
- Git state (Bash `git log`, `git status`, `git diff`) — directly measurable

**NOT agent-observable (operator-instrumented):**

- Token counts (`/cost` is operator-facing slash command; not exposed to agent)
- Prompt-cache hit/miss rates (no agent-side signal)
- Rate-limit-window utilization (no agent-side signal)
- Plan tier (operator-declared; not exposed via env var or CLI feature)
- Session-start clock time (system context names date only)

**Coordination ask upstream (not yet filed; legibility-only registration at [`claude-code-contract.md`](../claude-code-contract.md)):** agent-readable cost-export at session-end via tool / env var / session-log file would close most of the operator-verifiable gap. Currently no upstream commitment.
