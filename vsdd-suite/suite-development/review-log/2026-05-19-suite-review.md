# Suite Review — 2026-05-19

## Review 61 — 2026-05-19 09:00Z

**Scope:** Address G-149 (suite-development artifact naming alignment) per operator-selected resolution Option A — rename `SUITE-REVIEW-INDEX.md` → `SUITE-DEVELOPMENT-REVIEW.md` and `GAP-ANALYSIS-LOG.md` → `FINDINGS-INDEX.md` to match the project-template naming. Lands on a separate branch (`vsdd-suite-rename`) and PR per the option-A scoping rationale that a cross-cutting rename is more reviewable in isolation than mixed with substantive work.

**Lens:** Closure-by-mechanical-sweep + structural-consistency. The rename is mechanical (file mvs + reference sweep); the methodological value is dogfood-correctness — the suite now uses the same naming convention it ships for projects.

**Session note:** In-session, separate branch. Sycophancy compensation: the rename touches 17 forward-facing files plus 5 historical review-log files (link targets only). The temptation was to also re-frame the rename as some kind of methodological revelation; rejected — it's a naming alignment, a small unit of work that closes a gap registered weeks ago. The substantive part is the discipline of doing the sweep correctly: forward-facing files get both prose and link updates; historical files get link-target updates only (preserving the prose mention of the old name as audit trail per G-89). The sweep script enforces that distinction by checking `is_historical(path)` before doing the prose pass.

---

### Resolved

**G-149 — Suite-development artifact naming alignment via Option A rename.**

Files renamed (`git mv`):
- `vsdd-suite/suite-development/SUITE-REVIEW-INDEX.md` → `vsdd-suite/suite-development/SUITE-DEVELOPMENT-REVIEW.md`
- `vsdd-suite/suite-development/GAP-ANALYSIS-LOG.md` → `vsdd-suite/suite-development/FINDINGS-INDEX.md`

Both renames preserve git history via `git mv`'s rename detection.

**Reference sweep** (Python script, 17 files modified):

Forward-facing files (link targets AND prose references updated):
- `.pre-commit-config.yaml` (3 prose updates in comments)
- `vsdd-suite/README.md` (2 link-target + 6 prose updates)
- `vsdd-suite/crosslink-contract.md` (3 link-target + 3 prose updates)
- `vsdd-suite/domains/role/SOLUTION-OWNER-REVIEW.md` (1 link-target + 1 prose update — the Dim 10 reference added in Review 54)
- `vsdd-suite/hooks/check-crosslink-references.sh` (3 prose updates in docstring + HISTORICAL_NARRATIVE_MARKERS tuple)
- `vsdd-suite/primers/3-review-session.md` (2 prose updates)
- `vsdd-suite/primers/4-feedback-integration.md` (4 prose updates)
- `vsdd-suite/suite-development/README.md` (5 link-target + 7 prose updates)
- `vsdd-suite/suite-development/suite-development.md` (7 link-target + 24 prose updates)
- `vsdd-suite/templates/PROJECT-FINDINGS-INDEX-template.md` (1 link-target + 1 prose update)
- `vsdd-suite/templates/README.md` (1 prose update)
- `vsdd-suite/suite-development/FINDINGS-INDEX.md` (renamed file, 2 forward-facing self-references manually updated post-sweep: line 18 contributor instruction, line 212 instruction footer)

Historical-narrative files (link targets only — prose mentions preserved as audit trail per G-89):
- `vsdd-suite/COMPATIBILITY.md` (2 link-target updates)
- `vsdd-suite/suite-development/review-log/2026-05-03-suite-review.md` (1)
- `vsdd-suite/suite-development/review-log/2026-05-05-suite-review.md` (1)
- `vsdd-suite/suite-development/review-log/2026-05-06-suite-review.md` (1)
- `vsdd-suite/suite-development/review-log/2026-05-17-suite-review.md` (10)
- `vsdd-suite/suite-development/review-log/2026-05-18-suite-review.md` (5)

**Post-sweep manual cleanups:**
- `.pre-commit-config.yaml` exclude regex for the `check-crosslink-references` hook: dropped dead `(GAP-ANALYSIS-LOG|SUITE-REVIEW-INDEX)` alternatives (those filenames no longer exist at that path); regex now uses `SUITE-DEVELOPMENT-REVIEW\.md` and `.*/FINDINGS-INDEX\.md` only.
- `check-crosslink-references.sh` HISTORICAL_NARRATIVE_MARKERS tuple: deduped accidental duplicate `/FINDINGS-INDEX.md` entry (created when the sweep replaced `/GAP-ANALYSIS-LOG.md` with `/FINDINGS-INDEX.md` while a `/FINDINGS-INDEX.md` entry was already present); docstring updated to reflect the new tuple contents.

**COMPATIBILITY.md entry:** new v0.4.0 row in the Version anchors table documenting the rename, scope, forward-only positioning, and a migration note for any project with cross-references into `suite-development/` (rare — these are contributor-facing artifacts; projects rarely link into them).

**Verification:** pre-commit hooks (review-log-anonymization, check-crosslink-references, check-changelog-currency) all pass against the staged set.

**Resolution:** G-149 status flipped Open → Addressed. The suite now uses the same naming convention it ships for projects. Backlog after Review 61: **1 Open** (only G-146 — `crosslink knowledge` auto-injection, forward enhancement gated on `crosslink knowledge --help` verification per G-123/G-139 discipline).

---

### Coordination

The Review 61 closure coordinates with:

- **G-89** (Addressed, Review 39) — established the per-domain-index + per-session-file pattern that the project template inherited. G-149 brings the suite-development directory into structural alignment with that pattern.
- **G-138** (Addressed, Review 46) — established the FINDINGS-INDEX.md naming for the project-template manual-mode finding registry. G-149's rename of suite-level GAP-ANALYSIS-LOG.md → FINDINGS-INDEX.md is the dogfood application.
- **G-122** (Addressed, prior) — the suite-eats-its-own-cooking purity-boundary principle. G-149's rename is the same principle applied to naming convention.

The rename does NOT regress any prior work — every prior gap closure that referenced the old names by markdown link now points at the renamed file; every gap closure that referenced the old names in prose preserves the historical text (forward-only narrative-preservation policy).

**Backlog after Review 61: 1 Open** (G-146). The full mining cycle from Review 45's ITC pattern-mining and Review 51's dollspace.gay upstream-author mining is now closed: 14 + 7 + 2 (G-148, G-149) + 1 (G-158) = 24 gaps registered across this work cycle, of which 22 Addressed + 1 Deferred (G-135) + 1 Open (G-146). The remaining G-146 is a forward enhancement, not a backlog item — it requires external prerequisite work (verify `crosslink knowledge --help` surface) before any suite-side closure can begin.

Sycophancy self-audit: I considered cleaning up the renamed file's gap-row prose to use the new names throughout (rewriting "Resolution: A rename `SUITE-REVIEW-INDEX.md` → `SUITE-DEVELOPMENT-REVIEW.md`" to use only the new name). Rejected: the gap-row body describes the RESOLUTION the gap proposes, and that resolution names the old names as the source of the rename. Replacing "old name → new name" with "new name → new name" would lose the audit-trail signal of what was renamed. The forward-only policy applies inside gap-row bodies the same way it applies in review-log narratives — historical names stay, current names take their place in cross-references and instructions only.

---

### Summary

G-149 closed via Option A rename. 1 Open gap remaining in the active backlog (G-146, forward enhancement). The vsdd-suite naming convention is now consistent across user-facing (templates) and contributor-facing (suite-development) artifacts.
