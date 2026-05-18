# Suite Review — 2026-05-18

## Review 50 — 2026-05-18 19:30Z

**Scope:** Operator-raised observation pass surfacing two gaps off the back of the Review 49 polarity-sweep work. Triggered by the operator reading line 54 of `vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md` ("Review entries are logged in `vsdd-suite/TECHNICAL-WRITER-REVIEW.md` inside the project being reviewed") and asking whether the suite-development artifact names should align with the project template's names.

**Lens:** Operator-raised observation (source: director-raised per G-133). No adversarial sweep this session — just registration of two gaps the operator identified by reading. Both are structural defects of "stale instruction inherited from pre-G-89 framing" or "naming asymmetry inherited from suite-predates-template history" — well-suited to operator-direct-observation rather than to a cold review pass.

**Session note:** In-session, operator-driven. Sycophancy compensation: the second gap (rename) is a deliberation the operator framed as a question, not an assertion — I provided a recommendation in the gap body but did not flip the operator's question into a foregone conclusion. The first gap (16-file stale line) is a clear defect; flagged for fix in a follow-on session rather than addressing inline because the fix touches 16 files and crosses domain-prompt territory that should get its own commit for reviewability.

---

### New gaps registered

**G-148 — Domain prompt files cite outdated review-log path.** All 16 domain prompt files (14 role + 2 meta in `vsdd-suite/domains/role/` and `vsdd-suite/domains/meta/`) close with a line of shape "Review entries are logged in `vsdd-suite/<DOMAIN>-REVIEW.md` inside the project being reviewed." This predates the G-89 per-domain index + per-session-file structure (registered 2026-05-17). Under G-89, `<DOMAIN>-REVIEW.md` is now the INDEX file; actual review entries live in per-session files at `vsdd-suite/review-log/YYYY-MM-DD-<domain-slug>.md`. The stale line will lead a cold reader (human or AI) to append entries directly to the index file, violating G-89. Resolution: mechanical rewrite across all 16 files to the corrected wording (see G-148's row in GAP-ANALYSIS-LOG.md for the verbatim replacement). Forward-only — prior project review entries that may have followed the old framing remain valid records. **Status: Open.** Consider a follow-on G-139-style hook that asserts every `domains/{role,meta}/*-REVIEW.md` file references the per-session path correctly.

**G-149 — Suite-development artifact naming diverges from project-template naming.** The project template (per G-89 + G-138) uses `<DOMAIN>-REVIEW.md` / `review-log/...` / `FINDINGS-INDEX.md`; suite-development uses `SUITE-REVIEW-INDEX.md` / `review-log/...` / `GAP-ANALYSIS-LOG.md`. Same structural roles, divergent names. Historical reason: suite-development predates the project template; the template was derived and renamed for project-facing clarity, leaving the suite with its older naming. The asymmetry means the suite cannot serve as a worked example of its own template and contributors hold two parallel name systems. **Three resolution options** documented in G-149's row in GAP-ANALYSIS-LOG.md: (A) rename suite-development to match (recommended — dogfood-correct, mechanical rename, forward-only); (B) rename template to match (high-churn, not recommended); (C) document the divergence without renaming (lowest cost, preserves split mental model). **Status: Open** with recommendation (A) but pending operator decision. Rename, if approved, belongs in a dedicated PR (this PR is already substantive).

---

### Coordination

Both gaps are downstream of the structural decisions in G-89 (per-domain index + per-session-file) and G-138 (cross-cutting FINDINGS-INDEX.md):
- **G-148** is a stale-instruction defect: the 16 domain prompts were not swept when G-89 landed.
- **G-149** is a naming-alignment question: the project-template names that G-89/G-138 standardized never propagated back into the suite-development directory.

Both are forward-only fixes. Neither blocks any in-flight work; the suite is functional with the current names and stale instructions (contributors who know the structure work around them). The fixes are about cold-onboarding correctness (G-148) and worked-example coherence (G-149).

Sycophancy self-audit: I considered framing G-149 with a stronger recommendation than (A), since dogfooding is a long-standing suite principle. Rejected — the rename has cross-cutting cost (every internal reference to `GAP-ANALYSIS-LOG.md` and `SUITE-REVIEW-INDEX.md` in the suite must be updated; existing tools / scripts / docs that grep for the old names will break) and the operator's framing as a question signals deliberation, not a foregone conclusion. The body recommends (A) and explains the trade-off; the operator decides scope and timing.

No new findings beyond the two registered gaps. The 15 Open gaps now in the backlog (G-124–G-137 from Review 45 + G-146 from Review 49 + G-148 + G-149) remain as scoped; no re-prioritization triggered by this session.

---

## Review 49 — 2026-05-18 18:20Z

**Scope:** Adversarial review of the suite's value as a supplement to crosslink (driver-requested), followed by explicit articulation of the suite's two-mode operational design principle and a polarity sweep across user-facing docs to land the principle consistently. Files in scope of the sweep: `vsdd-suite/README.md` (Prerequisites, Quickstart, Worked Example Phases 1a/1b/2a/2b/3/4, Loop-until-MVR); `vsdd-suite/primers/4-feedback-integration.md` (§ With crosslink / § Without crosslink → `[crosslink]` / `[manual]`); `vsdd-suite/suite-development/suite-development.md` § Project-level finding index (two equivalent paths → two operational modes); `vsdd-suite/templates/README.md` (mode-independent scaffold + mode-specific usage); `vsdd-suite/suite-development/README.md` (added two-mode design principle statement for contributors).

**Lens:** Design-principle articulation + polarity-sweep. The adversarial review (this session's predecessor work, summarized in conversation) initially proposed a "crosslink-primary, manual fallback" framing that the operator corrected: "Crosslink is primary, manual is fallback is correct but manual must be a 1st class supported method." That correction is the design principle this Review crystallizes — manual mode is not a degraded path; it is a fully supported mode that the suite scaffolds, documents, and reviews with the same rigour as crosslink mode. The sweep re-frames every in-flight doc that previously read as "crosslink path / fallback" into parallel `[crosslink]` / `[manual]` blocks where both blocks carry the same VSDD discipline.

**Session note:** In-session — same operator that drafted the adversarial review and the original framings the sweep corrects. Sycophancy compensation: the predecessor adversarial-review session erred toward "crosslink should be primary, manual is the lesser fallback" and the operator pushed back; the inverse failure (validating crosslink-mode bias) is the one this session must avoid. I acknowledged the sycophancy inversion explicitly before authoring the sweep — the operator's "1st class supported method" constraint is binding on every edit. I tested the polarity by reading each touched section back and asking: does the manual block describe the same discipline with mechanical substitutions (grep instead of label filter, markdown rows instead of issue graph, inline narrative instead of `issue relate`), or does it describe a stripped-down lesser version? Where the answer was the latter, I rewrote until the discipline parity held. The five files listed in Scope all passed this check after the sweep.

---

### New gaps registered

**G-144 — Two-mode operational design principle implicit but never stated.** The suite's structure has always supported both crosslink-mode and manual-mode operation (every primer carries both paths; the templates scaffold both shapes; the finding-index pattern has both routes), but no user-facing doc stated this as a design principle. The README's earlier framing ("crosslink amplifier" / "without crosslink fallback") read as crosslink-first with manual as a degraded escape hatch; an AI authoring agent operating against that framing will continue to drift toward stripping the manual mode. Addressed by adding a "Two modes of operation (design principle)" section to `vsdd-suite/README.md` (user-facing) and `vsdd-suite/suite-development/README.md` (contributor-facing, binding on future contributions). See G-144 row in [GAP-ANALYSIS-LOG.md](../GAP-ANALYSIS-LOG.md) for the full Resolution.

**G-145 — Crosslink-mode additive operations under-used in Phase 4 routing.** The suite's `**Coordination:**` line on cross-domain findings was previously documented only as prose; in crosslink mode this is mechanizable as a structured issue-graph edge via `crosslink issue relate <a> <b>`, but no primer named the command. Addressed by adding `crosslink issue relate` to the `[crosslink]` block of Phase 4 in `vsdd-suite/README.md` and to the `[crosslink]` mode subsection in `vsdd-suite/primers/4-feedback-integration.md`. Manual mode retains the same discipline (coordination recorded inline in the routed finding's narrative) — G-145 is additive, not corrective.

**G-146 — Suite primer auto-injection via `crosslink knowledge` not documented or wired.** Crosslink's `knowledge` subcommand can register reference material to be auto-injected into agent sessions; in crosslink mode this would let the suite register primers once at scaffold time so `crosslink kickoff run` / `crosslink swarm review` invocations load them automatically. Currently primers are loaded by hand in both modes. **Open** — needs verification of `crosslink knowledge`'s actual surface (G-123/G-139 discipline applies — `crosslink knowledge --help` is the source of truth, not speculation), a decision on whether `scaffold-project.sh` or a separate hook should do the registration, and a policy on primer versioning. Manual mode is unaffected by any future implementation; G-144 binds the resolution to preserve the manual path.

**G-147 — Polarity sweep across in-flight suite docs to land G-144's design principle.** Five files re-keyed to parallel `[crosslink]` / `[manual]` blocks in a single sweep. Forward-only: prior review logs and CHANGELOG entries preserve the original framings as audit trail per G-89 narrative-preservation policy. Addressed in this Review by direct edit; the audit trail is git history + this entry.

---

### Resolved

**G-144, G-145, G-147 — Addressed via direct edit in this Review.** The five files listed in Scope now carry the two-mode framing consistently. Specifically:

- `vsdd-suite/README.md` — added "Two modes of operation (design principle)" section above the Prerequisites; updated Prerequisites to split "Baseline (required for both modes)" from "For crosslink-primary mode (recommended)"; restructured Quickstart as two parallel quickstarts; restructured Worked Example Overview table with `[crosslink]` and `[manual]` columns; flipped every phase block (Setup, 1a, 1b, 2a, 2b, 3, 4, Loop-until-MVR) to lead with `[crosslink]` (recommended) then `[manual]` (first-class fallback) and verified the manual block carries the same discipline; added `crosslink issue relate` in Phase 4 per G-145.
- `vsdd-suite/primers/4-feedback-integration.md` — re-framed § "With crosslink (Phase 2+ projects)" → § "[crosslink] — Recommended path"; re-framed § "Without crosslink (manual / Phase 1 projects)" → § "[manual] — First-class fallback path"; added a mode-framing paragraph above both subsections; added Step 4 (`crosslink issue relate`) for G-145; added a coordination-recording sentence to the manual mode's per-finding shape so cross-domain coordination is captured inline.
- `vsdd-suite/suite-development/suite-development.md` § Project-level finding index — re-framed "Two equivalent paths" → "Two operational modes"; re-framed "Crosslink path (preferred when crosslink is in use)" → "[crosslink] mode — recommended path"; re-framed "Manual path (when crosslink is not in use)" → "[manual] mode — first-class fallback path"; added a discipline-parity paragraph stating that every IAR discipline is fully exercisable in manual mode and the trade-off is mechanical, not methodological.
- `vsdd-suite/templates/README.md` — replaced "Manual (suite-only path)" / "With the helper script" / "Crosslink-enabled projects: templates are independent" with a single Usage section that states templates are mode-independent, leads with the recommended scaffold script, then provides a manual scaffold block as the first-class equivalent; added the `cp ... FINDINGS-INDEX.md` step to the manual scaffold block with the "manual mode only" callout per G-138.
- `vsdd-suite/suite-development/README.md` — added a "Two operational modes (design principle)" section between the structural-split paragraph and the "What lives here" section; the section names the principle, what it binds on future contributors (every crosslink-only mechanism MUST have a manual-mode equivalent), and what the trade-off is.

**Resolution:** Statuses flipped Open → Addressed in [GAP-ANALYSIS-LOG.md](../GAP-ANALYSIS-LOG.md) for G-144, G-145, G-147. G-146 remains Open as a forward enhancement.

---

### Coordination

The Review 49 work coordinates with the in-flight PR #20 cluster:
- **G-138** (Addressed, Review 46) — the finding-index pattern is what gives crosslink-mode's `issue relate` edges their queryable target population. G-145's `crosslink issue relate` example is only useful because findings are filed as issues per G-138.
- **G-139** (Addressed, Review 48) — the polarity sweep added new `crosslink <subcommand> --flag` citations in five files; the G-139 hook validates these automatically on commit. The sweep's correctness is gated on the hook passing clean. Specifically, the Phase 4 `crosslink issue relate <a> <b>` addition has no `--flag` portion and so isn't validated, but the surrounding context (`issue comment <id> ... --kind <kind>`, `issue close <id>`, `swarm fix --from-label`, `swarm fix --budget-aware`) is in scope.
- **G-123** (Addressed, Review 43) — the parent discipline. G-144's principle constrains future G-123-style mechanism additions: any auto-verification added to crosslink mode must also preserve the manual mode's parity (e.g., a hook that auto-files `crosslink issue create` must not become required infrastructure that breaks manual-mode users).

The 14 Open gaps from Review 45 (G-124–G-137) remain as scoped; G-144 / G-145 / G-147 do not affect that backlog. G-146 adds one Open gap to the backlog as a candidate enhancement. The recommended sequencing for follow-on closure is unchanged; G-146 would slot into the operational/tooling cluster once `crosslink knowledge`'s surface is verified.

Sycophancy self-audit: I considered framing the sweep as "minor wording adjustments" given that the underlying structure already supported both modes. Rejected: the operator's correction was substantive (the prior framing did read as crosslink-first with manual degraded; the operator was right to push back), and the sweep's effect on doc reading order changes user behavior — a new user landing on the Quickstart now sees the manual quickstart as a peer, not as a footnote. "Minor wording" would have undersold the principle.

---

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
