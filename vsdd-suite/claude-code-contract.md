# Claude Code CLI Dependency Contract

The VSDD suite teaches projects to apply the [Solution Architect](domains/role/SOLUTION-ARCHITECT-REVIEW.md) External Interface Contracts dimensions (Dims 13–22 in [`domains/role/SOLUTION-ARCHITECT-REVIEW.md`](domains/role/SOLUTION-ARCHITECT-REVIEW.md)). This file is the suite's own application of those dimensions to its dependency on the [Claude Code](https://github.com/anthropics/claude-code) CLI as the primary AI-author tool the methodology assumes.

**Reason this file exists:** registered as [Review 91 Finding 9](suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) in [`suite-development/FINDINGS-INDEX.md`](suite-development/FINDINGS-INDEX.md) — the cost-tally fabrication failure mode surfaced at Review 91 was rooted in agent-vs-operator observability asymmetry: the agent (claude-code CLI session) has access to a small surface; the operator (via `/cost` slash command + plan dashboard) has access to a larger surface; the suite's cost-tally schema authored against the aspirational full-instrumentation surface that does not exist on the agent side. This file makes the asymmetry explicit + names what the suite uses today vs what it needs upstream.

The suite was designed for tool-portability — every per-tool specific is in a per-tool supplement (per [`supplements/claude-code-cli.md`](supplements/claude-code-cli.md), the first instance; future per-tool supplements: cursor.md / aider.md / codex-cli.md / chatgpt.md / anthropic-api-direct.md). This contract is the claude-code-CLI-specific authority surface; other tools' contracts will sit alongside this file as they're authored.

---

## Tested-against version

**Claude Code CLI (current release as of 2026-05-24)** — every surface and feature in this file was verified against actual claude-code CLI behavior during the [Review 91](suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) session that authored this file. The reviewer (an Opus 4.7 session running inside claude-code CLI) directly observed the agent-side surface; the operator-side surface is named from the operator's prior memos + the [`supplements/claude-code-cli.md`](supplements/claude-code-cli.md) Plan tiers section. Updates to the claude-code CLI surface require re-validating this contract.

---

## Agent-observable surface (what claude-code CLI exposes to the AI agent)

The agent invokes tools through the claude-code CLI tool-call interface. Each tool returns observable data; the union of these returns is the agent-observable surface.

| Surface | What the agent sees | Use in the suite |
|---|---|---|
| **System context (session-start)** | Date (e.g., "Today's date is 2026-05-23"); working directory; OS; shell; model ID (`claude-opus-4-7` / `claude-sonnet-4-6` / `claude-haiku-4-5-20251001`); operator-author auto-memory (per `~/.claude/projects/.../memory/MEMORY.md`); CLAUDE.md content per directory; tool surface enumeration | AI tool / model / date capture for cost-tally; auto-memory load surfaces operator feedback |
| **Read tool returns** | File contents with `cat -n` line numbers (line + tab + content); error if file missing; image rendering for PNG/JPG | Per-file line counts countable from returns; substrate-measurable agent-self-verifiable cost-tally field |
| **Bash tool returns** | Command stdout + stderr (truncated past tool's output limit); exit code; wall-clock duration NOT exposed | Per-command output countable; `date -u` invocation IS the canonical wall-clock-anchor capture per [`supplements/claude-code-cli.md`](supplements/claude-code-cli.md) § Wall-clock measurement pattern |
| **Edit tool / Write tool returns** | Success confirmation; updated file's diff in some cases; error on read-before-edit violation | Files-touched count + per-file line-delta countable from invocation tracking |
| **Tool-call log (implicit)** | The agent's own message history including tool calls + results | Count tool calls per tool name; substrate-measurable cost-tally field |
| **TaskCreate / TaskUpdate / TaskList returns** | Task IDs; status; ownership; metadata | Task-cadence tracking for AI Engineer Dim 4 (sub-agent delegation patterns) |
| **AskUserQuestion returns** | Operator-selected answer + optional annotations / preview content | Operator-decision surface for methodology-shift questions |
| **ToolSearch returns** | Deferred-tool schema definitions | One-time setup; not per-cycle observable |
| **git via Bash** | `git log`, `git status`, `git diff`, `git config` outputs | Audit-trail verification surface; pre-commit hook outputs visible via commit attempt |
| **Hook outputs via Bash** | Per-hook PASS / FAIL + diagnostic text on commit attempt | Methodology-discipline enforcement; failures visible to agent for self-correction |

---

## NOT agent-observable (operator-only surfaces; the asymmetry)

Per [Review 91 Finding 8](suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) the cost-tally schema authored against fields that the agent CANNOT observe. These fields are operator-instrumented; the agent must defer them to `*pending operator …*` placeholders rather than fabricate.

| Surface | Operator-side access | Agent-side access | Cost-tally tier (per Finding 8) |
|---|---|---|---|
| **Token counts (raw_tokens; input/output split)** | `/cost` slash command displays per-session cumulative tokens | NONE — no API call returns token count to agent | Operator-verifiable |
| **Prompt-cache hit/miss ratio** | `/cost` displays cache-hit metrics | NONE — agent doesn't see whether its context-load was cache-served | Operator-verifiable |
| **Would-be API cost (computed from tokens × rate)** | `/cost` displays dollar figure | NONE — agent must defer (estimates from rate-mental-model are fabricated per Finding 8) | Operator-verifiable |
| **Rate-limit-window utilization** | Plan dashboard (claude.ai/settings) + claude-code CLI may surface warnings | NONE — agent doesn't see remaining window quota | Operator-verifiable |
| **Plan tier (Free / Pro / Max 5x / Max 20x / API direct)** | Operator knows from billing; CLI does NOT expose plan tier as a verifiable field | NONE — must be operator-declared per session per [AI Engineer Dim 14](domains/role/AI-ENGINEER-REVIEW.md) | Operator-confirmable |
| **Actual marginal cost to operator** | Operator computes from plan tier + window utilization + would-be cost | NONE — depends on operator-confirmable plan tier | Operator-confirmable |
| **Session-start clock time** | Operator's clock; potentially the CLI's session-log timestamps | System context names only date, not time; first Bash `date -u` invocation IS the session-start anchor per wall-clock pattern | Operator-confirmable (anchored via Bash) |
| **Cross-session conversation history** | claude.ai/code conversation browser | NONE — each session is fresh-context unless operator pastes prior content | N/A for cost-tally; relevant for AI Engineer Dim 1 (session isolation) |
| **CLI version** | `claude --version` operator-side command | NOT exposed via agent tools | Operator-confirmable; should be declared per cycle per [`supplements/claude-code-cli.md`](supplements/claude-code-cli.md) |

---

## Upstream coordination asks (legibility-only; not yet filed)

Per [`suite-development.md`](suite-development/suite-development.md) § External dependency references the suite's authority extends to suite-side artifacts; filing upstream PRs against [Claude Code](https://github.com/anthropics/claude-code) requires the suite-owner's deliberate decision. The following are legitimate gaps that would close most of the operator-verifiable asymmetry; they are documented here for legibility, not committed as plans.

1. **Agent-readable cost-export at session-end** — a tool invocation (e.g., `mcp__claude_code__cost_export` or a new `CostExport` tool) that returns the session's cumulative tokens + cache-hit ratio + would-be API cost as structured JSON the agent can read directly. Would close the operator-verifiable tier entirely; would let the cost-tally schema be agent-fillable end-to-end (modulo plan-tier which is still operator-confirmable).
2. **Plan-tier identification via env var or CLI feature** — an `ANTHROPIC_PLAN_TIER` env var or `claude config get plan` CLI feature that returns the current plan tier as a verifiable value. Would convert plan-tier from operator-confirmable to agent-self-verifiable. Would let AI Engineer Dim 14 verifiable-means detection succeed without operator prompting.
3. **Session-start clock anchor in system context** — adding `Session-start (UTC):` to the system context block at session-open, with HH:MMZ precision. Would eliminate the wall-clock-anchor-missing-on-first-invocation case noted in [Review 91 cost-tally](suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z); agents could compute elapsed directly without needing to capture both anchors.
4. **CLI version exposure to agent** — adding `Claude Code CLI version: vX.Y.Z` to the system context block. Would let cost-tally AI-tool field be self-verifying.

These asks parallel the [crosslink-contract.md](crosslink-contract.md) "Coordination asks (filed upstream)" pattern but are explicitly not-yet-filed. The suite's path to action: operator decides whether to engage with the upstream project; if yes, the suite-owner files the issue(s) with this contract document as the canonical "what we need + why" reference. Until then, the supplement-side codification (per [`supplements/claude-code-cli.md`](supplements/claude-code-cli.md) § Agents cannot count their own tokens + § Wall-clock measurement pattern + § Available observability surface for agents) is the canonical workaround.

---

## Cost-observability sibling JSON file (Shape 1 spec; per [Review 91 Finding 9](suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z))

While the upstream asks above are not filed, the suite codifies the **Shape 1 inline-vs-JSON cost-tally split** (per [Review 91 Finding 9](suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z)) as the interim observability infrastructure. The agent fills the inline cost-tally with substrate-measurable fields; the operator (manually or via post-session script) writes a sibling JSON file with measured fields.

**File location:** `vsdd-suite/suite-development/cost-observability/YYYY-MM-DD-review-N.json` for suite-side reviews; `<project>/vsdd-suite/cost-observability/YYYY-MM-DD-review-N-<domain>.json` for project-side reviews.

**Schema (JSON):**

```json
{
  "review_id": "r91",
  "review_anchor": "review-91--2026-05-23-1900z",
  "review_log_path": "vsdd-suite/suite-development/review-log/2026-05-23-suite-review.md",
  "session_window": {
    "session_start_utc": "2026-05-23T19:00Z",
    "session_end_utc": "2026-05-24T02:43Z",
    "session_start_anchor_source": "operator_clock",
    "session_end_anchor_source": "bash_date_u"
  },
  "ai_tool": "claude-code",
  "ai_tool_version": "v0.X",
  "plan_tier": "claude-max",
  "execution_method": "inline-main-session",
  "model": "claude-opus-4-7",
  "tokens": {
    "raw_input": 0,
    "raw_output": 0,
    "cache_creation": 0,
    "cache_read": 0,
    "total": 0,
    "source": "operator_cost_command_output"
  },
  "would_be_api_cost_usd": 0.0,
  "actual_cost_to_operator_usd": 0.0,
  "actual_cost_basis": "max_plan_within_limits",
  "rate_limit_window": {
    "utilization_percent": 0,
    "window_hours": 5,
    "source": "operator_dashboard"
  },
  "wall_clock": {
    "elapsed_seconds": 27780,
    "elapsed_human": "7h43m",
    "includes_operator_discussion": true,
    "includes_idle_time": true,
    "agent_active_fraction_estimated": 0.0
  },
  "findings_count": {
    "resolved": 8,
    "open": 10,
    "dismissed": 2,
    "total": 20
  },
  "findings_per_100k_tokens": 0.0,
  "tuning_levers_named": [
    "model-tier-right-sizing",
    "prompt-cache-discipline",
    "cluster-batching",
    "sub-agent-scope-down",
    "n-plus-1-detection",
    "cycle-stop"
  ],
  "anomaly_flags": []
}
```

**Fields the operator fills (post-session):** `ai_tool_version` (via `claude --version`); `tokens.*` (via `/cost`); `would_be_api_cost_usd` (via `/cost`); `actual_cost_to_operator_usd` (operator's billing knowledge); `rate_limit_window.utilization_percent` (operator's dashboard); `wall_clock.elapsed_seconds` if the agent did not capture both anchors; `findings_per_100k_tokens` (derived from tokens + findings_count once tokens populated).

**Fields the agent fills inline (pre-operator-paste):** `review_id`, `review_anchor`, `review_log_path`, `session_window.session_end_utc` (via Bash `date -u`), `ai_tool`, `plan_tier` (operator-declared inheritance), `execution_method`, `model`, `findings_count.*`, `tuning_levers_named`.

**Anomaly detection (deferred to Shape 2 subsystem per [Finding 11](suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z)):** `anomaly_flags` would be populated by a future aggregator checking the current cycle's cost-per-finding against the rolling-median baseline; flags >3σ above the baseline. Currently the array stays empty; flag-generation requires a cross-cycle data store + aggregator that does not yet exist.

**Cross-cycle dashboard (deferred to Shape 2 subsystem per [Finding 11](suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z)):** `vsdd-suite/suite-development/COST-OBSERVABILITY.md` generated rollup would consume the JSON files for trend analysis (cost-per-finding trajectory across cycles; per-tuning-lever cost-impact correlation; rate-limit-window utilization trends). Currently does not exist; generation requires the aggregator.

**Forward-only:** the JSON file shape applies to reviews authored 2026-05-24 and later; pre-2026-05-24 reviews carry their inline cost-tally (possibly fabricated per [Finding 8](suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z)) preserved per [G-89](suite-development/FINDINGS-INDEX.md#g-89).

---

## When this contract changes

Updates to the claude-code CLI surface (new tools; new env vars; CLI version bump that adds observable fields; system context shape changes) require:

1. Re-validating the [Agent-observable surface](#agent-observable-surface-what-claude-code-cli-exposes-to-the-ai-agent) table against actual behavior in a fresh session
2. Updating the [NOT agent-observable](#not-agent-observable-operator-only-surfaces-the-asymmetry) table per any surfaces that move from operator-only to agent-observable
3. Removing closed asks from [Upstream coordination asks](#upstream-coordination-asks-legibility-only-not-yet-filed) once upstream implements them; documenting the new surface in the relevant section
4. Bumping the Tested-against version line
5. Logging the contract update in a `vsdd-suite/CHANGELOG.md` entry per the G-129 changelog-currency discipline
6. Filing a suite-review entry capturing the change-of-contract per [`suite-development.md`](suite-development/suite-development.md) § Suite review entry format

The contract is **stable agent-API surface** under the same shape-stability commitment as the [Agent-API surface contract](suite-development/suite-development.md#agent-api-surface-review-80-finding-3) — agents authored against these invariants will not break across releases unless the upstream change is itself reflected here.
