# Suite Review — 2026-05-18

## Review 47 — 2026-05-18 01:21Z

**Scope:** G-118 follow-on (driver-requested) to update `crosslink-contract.md` with the verified surface for the G-138 finding-index pattern. The verification step (per the G-123 external-dependency discipline) ran `crosslink <subcommand> --help` against installed crosslink v0.8.0 for every command the suite references — and surfaced a second-instance G-123 recurrence: `crosslink issue close --comment "<text>"` was referenced in 5 places across the suite (1 in the existing `crosslink-contract.md`, 3 in `README.md` § Worked example Phase 3 and Phase 4, 1 in `primers/4-feedback-integration.md`) but the actual `close` subcommand does not accept `--comment`. The rationale belongs in a prior `crosslink issue comment <id> "<text>" --kind <kind>` followed by `crosslink issue close <id>`. Both the G-138 surface addition and the `--comment` correction land in this session.

**Lens:** Verification-against-installed-CLI applied as a sub-lens of suite-internal review. Specifically anchors every claim about crosslink CLI surface to a `crosslink <cmd> --help` invocation; rejects any claim that does not match.

**Session note:** In-session. Sycophancy compensation: I authored the `--comment`-on-close references in earlier session work (Reviews 38–43 + bookmark-cli reference impl). The G-123 discipline was supposed to prevent this exact pattern; that this is the SECOND instance (the first was `crosslink init --with-suite` in Reviews 40–42, corrected in Review 43) is a recurrence signal — not just "I was careless" but "the discipline as currently authored is insufficient against AI-agent recurrence." Registering G-139 as a tooling gap separately rather than treating the recurrence as a one-off correction.

---

### Addressed

**G-118 follow-on — `crosslink-contract.md` extended with G-138 finding-index commands; existing `--comment` row corrected.**

The pre-existing dependency-surface table (Phase 3 row 3) claimed `crosslink issue close <id> --comment "<text>"` with `--comment` as a required flag. Verification: `crosslink issue close --help` shows only `<ID>` positional + `--no-changelog` flag; no `--comment` exists. Corrected the row to `crosslink issue close <id>` and added two new rows above documenting the correct comment-then-close pattern (`issue comment <id> "<text>" --kind <kind>` then `issue close <id>`). Added `crosslink issue unlabel` and refined `issue list` to use `-s` (short form for `--status`) per the verified `--help` output.

Added a new section to `crosslink-contract.md`:

- **`### G-138 finding-index commands (crosslink path)`** — table enumerating: `issue create` with structured labels (`-l` repeatable; verified); `issue list -l <axis>:<value> -s <status>` for single-axis filter; multi-axis composition note (single-label filter only — use `--json | jq` for multi-axis); `crosslink tui` for interactive browse; `issue label` / `issue unlabel` for label adjustment; the reclassify sequence (unlabel-then-label-then-close, comment-then-close); `export -f json -o <path>` / `import <INPUT>` for manual ↔ crosslink migration.
- **`### Crosslink commands the suite does not depend on`** — explicit out-of-scope list (kickoff, container, sentinel, knowledge, style, mc, serve, tui, trust, locks, sync, migrate, config, context, integrity, compact, prune, timer). For audit clarity: a future contributor knows the contract surface is intentionally scoped, not accidentally narrow.

The "Tested-against version" line at the top updated to: "every command and flag in this file was verified against `crosslink <subcommand> --help` output on 2026-05-17 (Review 46 + 47 verification pass)."

**Correction sweep** for the `--comment`-on-close error:
- `README.md` § Worked example Phase 3 — corrected the Hallucinated example to use `comment --kind decision` then `close`; corrected the Resolved example to use `comment --kind resolution` then `close`; updated `issue list --status` to `issue list -s` (matching verified short-form).
- `README.md` § Worked example Phase 4 — corrected the routed-finding closure example to use `comment --kind resolution` then `close`; added explanatory note about `issue close` not accepting `--comment`.
- `primers/4-feedback-integration.md` § Step 5 — corrected the routed-finding closure prose to use the `&&`-chained `comment --kind resolution && close` pattern with explanatory parenthetical.

**Verification of remaining grep matches:** the 2 remaining matches for `issue close.*--comment` in the suite are both inside the CORRECTION TEXT itself (the explanatory phrase "`issue close` does not accept `--comment`"). Those are the correction notes; not actual command examples.

**Resolution:** G-118 follow-on closed via the contract extension + correction sweep. The crosslink-contract.md now serves as the suite's canonical record of the verified crosslink dependency surface AT THE FLAG LEVEL — any future suite documentation referencing a crosslink command must match this file or update both.

---

### New gap registered

**G-139 — G-123 manual discipline insufficient against AI-agent recurrence; CLI-verification tooling needed.**

The pattern: Reviews 38–43 introduced `crosslink init --with-suite` references that don't exist (corrected in Review 43, G-123 registered). Reviews 38–46 introduced `crosslink issue close --comment` references that don't exist (corrected in this Review 47). Both were violations of the G-123 discipline ("before referencing an external tool's feature, verify against that tool's governing documentation") that landed despite the discipline being explicitly documented.

The recurrence shape: an AI authoring agent operating inside a long session naturally pattern-matches "this command probably accepts this flag" against precedent from other CLIs (`gh pr close --comment "..."`; `jira issue close --comment "..."`; etc.). The G-123 discipline asks the agent to verify before writing — but the discipline runs *in the same context* that produced the speculation, so the agent's confidence in the speculation overrides the verification step.

**The discipline as currently authored is insufficient against this failure mode.** Two instances across four sessions = recurrence trigger per the "earned by recurrence" doctrine (Review 37 / G-99 framing). The rule change earned by the recurrence: add an automated verification step to the suite's pre-commit hook surface.

**Resolution sketch:** Add a `vsdd-suite/hooks/check-crosslink-references.sh` script that:
1. Greps the staged suite documentation files for `crosslink \w+( \w+)?( --\w+)*` patterns.
2. For each unique pattern found, runs `crosslink <subcommand> --help` and checks that every cited flag appears in the help output.
3. Fails the commit if any cited flag is not in the help — with a clear message naming the file:line, the cited flag, and the actual help output.
4. Wired into `.pre-commit-config.yaml` scoped to `vsdd-suite/**/*.md` and `vsdd-suite/**/*.sh`.

The hook converts G-123 from "discipline the author must remember" to "mechanism that fires automatically." Operates only when crosslink is installed (the hook checks for `command -v crosslink` and skips with a warning otherwise — a CI environment without crosslink reports the skip rather than failing).

**Severity:** Mission-critical High / Speculative Medium. Mission-critical because every recurrence of the pattern ships docs that mislead users (and may waste their time when they try the wrong command); speculative is medium because the tooling cost is modest and the pattern is well-understood.

**Status:** Open. Cross-coordinate: G-118 (the contract file is the source of truth that the hook would validate against, so the hook need not also re-run all `--help` invocations every commit if the contract file is up-to-date); G-123 (the discipline this hook mechanizes); G-129 (CHANGELOG-currency hook, similar tooling shape — could share infrastructure).

---

### Coordination

The Review 47 work coordinates with multiple recently-closed gaps:

- **G-118** — this Review extends the crosslink-contract.md surface. The contract file is now the canonical record at the flag level; any future suite documentation referencing crosslink must match.
- **G-138** — the new crosslink-finding-index commands documented in this Review's contract update are the verified surface for G-138's crosslink path. The path is now backed by tested CLI evidence.
- **G-123** — second recurrence acknowledged. Triggers G-139 (tooling fix) per "earned by recurrence" doctrine.

The 14 Open gaps from Review 45 (G-124 through G-137) are unaffected. The recommended-sequencing plan for follow-on closure still stands; G-139 inserts at the top of the operational/tooling cluster as the cheapest single addition that prevents further G-123 recurrence.

No new findings from the verification pass beyond the `--comment` correction and G-139 registration. The other crosslink commands referenced across the suite (`crosslink design`, `quick`, `milestone *`, `session *`, `swarm *`, `issue label`, `issue block`, `issue list`, `issue create`) all match verified `--help` output.
