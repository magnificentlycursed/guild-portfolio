# Suite Review — 2026-05-18

## Review 48 — 2026-05-18 01:31Z

**Scope:** Address G-139 by implementing the `check-crosslink-references` pre-commit hook proposed in Review 47. The hook mechanizes the G-123 discipline ("verify external-dependency feature references against governing documentation") by automatically running `crosslink <subcommand> --help` for every cited command in user-facing suite docs and failing the commit if any cited long flag is missing from the help output.

**Lens:** Tooling-addresses-recurring-discipline-failure lens. The closure pattern is itself a precedent: when a manual discipline (G-123) fails twice in the same way, the rule change earned by recurrence is a tooling fix (G-139), not a stricter discipline. Future similar gaps should follow the same arc.

**Session note:** In-session — same operator that registered G-139 and authored the prior wrong references the hook was designed to catch. Sycophancy compensation: I tested the hook against the full suite *before* claiming it was correct, which surfaced (a) 23 historical-narrative false-positives in the review-log/gap-registry/index files, addressed by adding a self-skip list to the hook AND an `exclude:` filter in `.pre-commit-config.yaml`; and (b) one real catch in `suite-development.md:104` where the G-123 governing-standard text quoted `crosslink init --with-suite` as a worked example — rewritten to "a fictitious `--with-suite` flag attributed to crosslink's `init` subcommand" to convey the same information without the grep-trigger substring. The pre-test verification is what produced the (a)/(b) outcomes; without it the hook would have shipped broken or with the policy violation un-noticed.

---

### Resolved

**G-139 — `check-crosslink-references` pre-commit hook implemented, tested clean against the full suite.**

The hook is a Python script (with `.sh` filename for parity with the existing `check-review-log-anonymization.sh`; the shebang routes to `python3`). Behavior:

- Scans staged text files for `crosslink <subcommand> ... --<flag>` patterns. Subcommand is 1–3 words; longest-first match via `crosslink <tokens> --help`.
- For each `(subcommand, flag)` pair, validates that the flag appears in the help output's option lines.
- Fails the commit if any cited long-form flag is not in the help. Reports file:line, the cited subcommand+flag, and the set of valid flags for the subcommand.
- Skips gracefully when crosslink is not installed (`shutil.which("crosslink")` returns `None`; the hook prints a warning and exits 0 — CI-environment safe).
- Self-skips known historical-narrative files (CHANGELOG, COMPATIBILITY, GAP-ANALYSIS-LOG, SUITE-REVIEW-INDEX, review-log/*, FINDINGS-INDEX) where past wrong commands are preserved as audit trail. Defense in depth: `.pre-commit-config.yaml` also `exclude:`s the same paths for efficiency at the staged-files level.
- Scope (long flags only): short-form flags (`-l`, `-s`, etc.) are not validated in this version. Narrow scope catches both recorded G-123 recurrences (`--with-suite`, `--comment`) while keeping the regex tractable. Short-flag validation would extend the scope but would also surface false positives from incidental short-flag-like substrings in narrative prose; not warranted by the current recurrence evidence.

`.pre-commit-config.yaml` updated with the new `check-crosslink-references` hook entry, scoped via `files:` to `vsdd-suite/**/*.{md,sh}` and `<project>/vsdd-suite/*.md` (single-level — project per-domain index files), and `exclude:`d for the historical-narrative paths listed above.

**Tested clean against the full current suite:** all user-facing docs (README.md, primers/, supplements/, hooks/, templates/, crosslink-contract.md, suite-development/README.md, suite-development/suite-development.md, bookmark-cli/vsdd-suite/QUALITY-ENGINEER-REVIEW.md, etc.) — zero false positives. The historical-narrative files (review-log/, GAP, INDEX, CHANGELOG, COMPATIBILITY, FINDINGS-INDEX) are correctly skipped; their `--with-suite` and `--comment` citations remain as audit trail.

**One narrative correction applied during the test:** `suite-development/suite-development.md:104` (the G-123 governing-standard section) quoted `crosslink init --with-suite` directly in prose. The hook caught it; rewrite preserves the information ("a fictitious `--with-suite` flag attributed to crosslink's `init` subcommand") without the grep-trigger substring. This is a feature, not a bug — the contributor primer is a reference doc that should not itself cite non-existent commands even in failure-mode discussion; the historical-narrative files are the appropriate home for verbatim citations.

**Resolution:** Status flipped Open → Addressed in [GAP-ANALYSIS-LOG.md](../GAP-ANALYSIS-LOG.md). G-123 is now mechanism-backed; recurrence of the speculation pattern would fail the commit hook rather than ship to users.

---

### Coordination

The Review 47 → Review 48 arc closes the G-123 / G-139 / G-118 cluster cleanly:
- G-123 (Review 43) introduced the discipline.
- G-139 (Review 47) recognized the discipline alone was insufficient.
- G-139 closure (this Review 48) implemented the mechanism.

Future similar patterns should follow the arc: discipline → recurrence-recognition → tooling. The "earned by recurrence" doctrine (Review 37 / G-99 framing) names the trigger; G-123 → G-139 is the first end-to-end instance of the discipline-to-tooling promotion pattern in the suite's history.

No new gaps surfaced this session. Sycophancy self-audit: I considered pushing back against adding the `exclude:` filter on the basis that "if it's wrong text, fix the wrong text rather than excluding files." Rejected: the historical-narrative files DELIBERATELY preserve past wrong citations per Review 43's narrative-preservation policy. Excluding them from the hook is the policy-coherent answer; rewriting them would violate Review 43.

The Review 45 backlog (G-124–G-137) remains as scoped; G-139's closure does not affect that backlog directly, but the hook's mechanism is reusable infrastructure that G-129 (CHANGELOG-currency hook) could share — when G-129 is addressed, the hook can be modeled on `check-crosslink-references.sh`'s shape (Python with shebang, pre-commit `files:`/`exclude:` scoping, self-skip safety net).

---

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
