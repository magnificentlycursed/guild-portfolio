# Changelog

All notable changes to the IAR suite are recorded here. Entries are in reverse chronological order. Timestamps are UTC (Zulu).

---

## Unreleased — 2026-05-05 (Review 35: manual-testing-checklist runnable-step standard)

### Changed
- **`prompts/decomposition.md`** — `### Manual testing checklist` section rewritten. Replaced the five-bullet shorthand prompt (happy path / error states / empty state / persistence / edge cases) with a runnable-step standard requiring four properties per step: exact command, expected outcome (stdout/stderr/exit code/on-disk state), explicit clean-state setup when required, and binary install/uninstall/reinstall lifecycle when relevant. Added an "Audience" paragraph specifying that the tester is unfamiliar with the toolchain and the project. Inlined an "Example shape (one expanded step)" so the standard is anchored in a copyable form. The `**Manual Testing Checklist:**` block in the `## TODO.md format` example was reframed: items are now placeholders that expand into runnable step blocks, with an explicit "Step 0 — Update the installed binary" item when the layer changes runtime behavior. Addresses G-97.
- **`prompts/decomposition.md`** — Follow-up refinement (Review 35 Finding 2): expected-outcome requirement (#2) tightened to require literal output blocks for invariant output, with prose descriptions reserved for variable output (timestamps, IDs, OS-chosen paths) anchored to a representative literal example. Help-command specificity sub-clause added to requirement #4 (`<binary> <subcommand> --help`, not `<binary> --help`, when help text is part of verification). Example shape updated to show literal expected-stderr / expected-stdout / on-disk-state blocks. Triggered by user-discovered defect in the rendered Layer 4 plan: the prose hint "expect: --label flag listed under create + list" named the wrong help command (top-level instead of subcommand) and the prose form was ambiguous enough that the error went undetected at authoring time.
- **`prompts/decomposition.md`** — Second follow-up refinement (Review 35 Finding 3): added "Help-output verification (CLI projects)" as a sixth bullet in the **Required items per layer** list. Per binary and per changed subcommand, run `<binary> <subcommand> --help` and include a literal expected-output block. Anchored to "Step 0 — Update the installed binary" so a stale-binary problem fails fast. Carves out an explicit exception for layers that don't change the CLI surface. The defect this catches: a CLI layer ships with a flag whose runtime behavior works but whose `--help` description is missing, stale, or contradicts the actual flag — discoverability drift that integration tests do not catch. Considered and rejected: placing the rule in `supplements/cli.md`; supplements are currently review-time artifacts, not authoring-time; the cross-reference pattern would be novel.
- **`prompts/decomposition.md`** — Third follow-up refinement (Review 35 Finding 4): added "Usage examples in `--help` (CLI projects with compound flags or filters)" as a seventh bullet in the **Required items per layer** list. For projects whose subcommands accept multiple optional flags or compound filters, the polish/help-finalization layer's acceptance criteria must require usage examples in the relevant subcommand's `--help` output (1–3 examples per subcommand, covering common scenarios like compound filtering). The defect this catches: a user reading `--help` against a layer with five filters sees orthogonal flag descriptions but cannot tell which combinations make sense without imagining them — examples answer "how do I do the thing I came to do?" while a flag list only answers "what can I do?" Cross-references existing `supplements/cli.md` UX dim 1 (review-time, top-level) and extends the expectation to subcommand-level for compound-flag cases. Forward-only.
- **`GAP-ANALYSIS-LOG.md`** — G-97 added (status `Addressed`) — manual-testing-checklist format produced tester-familiarity-dependent items. Distinct from G-42 (which addressed *which domain evaluates* checklist completion); G-97 addresses *the format the checklist itself takes when produced by decomposition*. Refinement (literal blocks vs prose) recorded in Review 35 Finding 2 stays within G-97's scope.

### Added
- **`review-log/2026-05-05-suite-review.md`** — new file. Review 35 logged: one finding Resolved (decomposition-checklist format tightened); G-97 registered and immediately Addressed.
- **`SUITE-REVIEW-INDEX.md`** — Review 35 row added to the Suite Reviews index.

### Note
Forward-only. Existing `TODO.md` files in projects under review are not retroactively rewritten; new layer plans (and re-decomposed layers) inherit the new standard. The change does not require domain-prompt updates: VDD-IAR Alignment dim 9 evaluates whether checklists exist and were completed, not their format quality, and format quality is owned implicitly by SO (spec coverage) and TW (documentation accuracy) when reading project TODO.md.

---

## Unreleased — 2026-05-03 (Review 34: apply G-90 and G-94, register G-96)

### Changed
- **`prompts/spec-crystallization.md`** — H1 retitled `(VSDD Phase 1)` → `(VSDD Phase 1a)`; in-prompt phase reference and governing-standard reference updated. Addresses G-90.
- **`README.md`** — VSDD pipeline table phase column `1` → `1a`; "spec issues to Phase 1" and "during Phase 1" updated to `Phase 1a`. `## Suite scope` and pipeline cross-references for `domains/role/DOMAIN-INDEX.md` and `SUITE-REVIEW.md` updated to `domains/DOMAIN-INDEX.md` and `SUITE-REVIEW-INDEX.md`. Addresses G-90 and G-94 sub-issues 2, 3.
- **`prompts/suite-development.md`** — H1 retitled to `# Session Primer: Suite Development (Meta — Suite Contributors)` (G-94 sub-issue 5). Suite-history paragraph "Phase 1 spec crystallization" → "Phase 1a" (G-90). `## Lang supplement coverage` heading → `## Supplement coverage`; suite-structure table row "Lang supplements" → "Language and interface supplements"; "Lang supplement reference" terminology generalized; `domains/role/DOMAIN-INDEX.md` references updated to `domains/DOMAIN-INDEX.md`; all `SUITE-REVIEW.md` references in current-spec lines updated to `SUITE-REVIEW-INDEX.md`. Addresses G-94 sub-issues 1, 2, 3, 5.
- **`prompts/review-session.md`** — `domains/role/DOMAIN-INDEX.md` reference updated; `SUITE-REVIEW.md` reference updated to `SUITE-REVIEW-INDEX.md`. Addresses G-94 sub-issues 2, 3.
- **`domains/role/SOLUTION-OWNER-REVIEW.md`** — "scope-crept during Phase 1" updated to "VSDD Phase 1a" (G-90); "lang supplement" terminology generalized in Language-and-interface-supplement reference (G-94 sub-issue 1).
- **`domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md`** — "VSDD Phase 1 criteria" → "VSDD Phase 1a criteria" (dim 1); pipeline-list `(1, 1b, 2a, 2b, ...)` → `(1a, 1b, 2a, 2b, ...)` in the Program Phase Context note. Addresses G-90.
- **`domains/DOMAIN-INDEX.md`** — File moved from `domains/role/DOMAIN-INDEX.md` (G-94 sub-issue 2). Internal references updated: README link `../../README.md` → `../README.md`; meta-domain backticked paths `../meta/` → `meta/`; role-domain entries gained the `role/` prefix for symmetry with the meta entries; `lang/cli.md` reference updated to `supplements/cli.md`.
- **`SUITE-REVIEW-INDEX.md`** — File renamed from `SUITE-REVIEW.md` (G-94 sub-issue 3).
- **`supplements/`** — Folder renamed from `lang/`. Path references updated in 24 forward-facing and historical `*.md` files via bulk sed; nonsensical strings produced as side effects in some prior-session narratives were repaired post-bulk. Addresses G-94 sub-issue 1.
- **`GAP-ANALYSIS-LOG.md`** — G-90 status `Open` → `Addressed`; description rewritten to record the resolution path (Option 1, whitepaper-confirmed). G-94 status `Open` → `Addressed (partial — sub-issue 4 deferred to spinoff-MVP)`; description rewritten to record per-sub-issue outcome. G-96 added (Open) — suite's Phase 1a/1b/2a/2b sub-phase semantics diverge from the whitepaper's Step 1a/1b/1c, 2a/2b/2c structure; resolution path is harmonize-or-document-the-divergence, deferred to evaluation.

### Added
- **`review-log/2026-05-03-suite-review.md`** — Review 34 logged: applied G-90 and G-94 sub-issues 1, 2, 3, 5; deferred G-94 sub-issue 4 (CHANGELOG release tagging) to spinoff-MVP; registered G-96 (whitepaper sub-phase semantic divergence). Two findings resolved, one new gap registered.
- **`SUITE-REVIEW-INDEX.md`** — Review 34 row added to the Suite Reviews index.

### Note
G-94 sub-issue 4 (CHANGELOG release tagging) remains Open inside G-94's "partial" status. Both proposed alternatives — tag spinoff-MVP as `1.0.0`, or rename `## Unreleased` framing to `## Session N` — require either an event that has not occurred or a renumbering that conflicts with the "do not silently amend prior findings" discipline. Reconsider at the spinoff-MVP boundary alongside the rest of the deferred restructure work. G-96 (whitepaper sub-phase divergence) is a deeper finding than G-90 — G-90 fixed the labelling asymmetry; G-96 names the underlying semantic mismatch and offers harmonize-vs-document-divergence as the resolution menu. No action taken on G-96 in this session beyond registration.

---

## Unreleased — 2026-05-03 (Review 33: bundled-deferral dependency analysis)

### Changed
- **`GAP-ANALYSIS-LOG.md`** — G-90 (VSDD phase numbering inconsistency) status `Deferred` → `Open`. Decoupled from the `issue-tracker-cli` trigger because the gap depends on the upstream VSDD whitepaper, not on project feedback. Description amended in place; row links updated to reference Review 33 as the decoupling record. Addresses Review 33 Finding 1.
- **`GAP-ANALYSIS-LOG.md`** — G-94 (smaller naming/location bundle) status `Deferred` → `Open`. Decoupled from the `issue-tracker-cli` trigger because the five sub-issues are mechanical or low-coordination and do not depend on real-project pressure or reading patterns. May still travel with the G-88/G-91/G-92/G-93 spinoff-time restructure if convenient, or be addressed independently. Description amended in place. Addresses Review 33 Finding 1.

### Added
- **`review-log/2026-05-03-suite-review.md`** — Review 33 logged: per-gap dependency analysis of the eight gaps deferred to "after `issue-tracker-cli` completes." Two findings resolved (G-90 and G-94 promoted to Open with rationale; G-88, G-89, G-91, G-92, G-93, G-95 reviewed and confirmed properly Deferred).
- **`SUITE-REVIEW.md`** — Review 33 row added to the Suite Reviews index.

### Note
G-88, G-89, G-91, G-92, G-93, G-95 remain Deferred under the `issue-tracker-cli` trigger. Their substance genuinely depends on real-project feedback (G-88, G-89), forward-only path constraints driven by in-flight project references (G-91), or coordination with other deferred gaps (G-92, G-93, G-95). The bundled-trigger pattern is preserved for these six.

---

## Unreleased — 2026-05-03 (Review 32: suite-review entry-format and deferral-trigger consistency)

### Changed
- **`prompts/suite-development.md`** — Updated `### Suite review entry format` item 3 (Lens) to enumerate three valid forms: named defect class, registry-walk scope, and role-based lens (covering both domain-perspective and named-bundle variants). Reflects practice in Reviews 30 and 31; addresses Review 32 Finding 3.
- **`prompts/suite-development.md`** — Updated `### Suite review entry format` item 6 (Closing) to permit an optional `### Coordination` section after the classification sections, used to name a cross-finding cluster and bundled action. Markdown links required for cross-references. Addresses Review 32 Finding 1; retroactively conforms Review 31's `### Coordination` section.
- **`prompts/suite-development.md`** — Added `### Session isolation` subsection to `## Suite review and review-log discipline`. Documents that suite reviews are typically in-session (unlike domain reviews); cold-session is permitted and stronger but not required; minimum standard is an explicit session note naming cold-vs-in-session status and, if in-session, naming a compensation. A missing session note is itself a finding for VDD-IAR Alignment dim 7 applied to the suite. Addresses Review 32 Finding 4.
- **`GAP-ANALYSIS-LOG.md`** — Added `## Reactivation triggers` subsection between `## How to run a gap analysis` and `## Gap Registry`. Defines the bundled trigger "after `issue-tracker-cli` completes" with three required conditions (all layers merged, final-merge VDD-IAR Alignment classified, project archived). Names abandonment/pivot path; permits decoupling for gaps not actually dependent on `issue-tracker-cli` feedback. Addresses Review 32 Finding 2.

### Added
- **`review-log/2026-05-03-suite-review.md`** — Review 32 logged: suite-review entry-format and deferral-trigger consistency. Four findings resolved in-session (F1 Coordination heading, F2 trigger definition, F3 lens grammar, F4 session isolation). Two hallucinated (re-raised directory rename — already deferred as G-88; per-entry sycophancy section — boilerplate that G-77 already corrected).
- **`SUITE-REVIEW.md`** — Review 32 row added to the Suite Reviews index.

---

## Unreleased — 2026-05-03 (Review 31: five-lens adversarial pass, structural findings deferred)

### Added
- **`review-log/2026-05-03-suite-review.md`** — Review 31 logged: five-lens adversarial review (clarity, naming, ambiguity, consistency, transitional-state alignment) framed for the eventual standalone-repo spinoff. Six new gaps registered (G-90 through G-95), all Deferred to post-`issue-tracker-cli` completion under the same forward-only constraint that applies to G-88 and G-89. Two findings hallucinated (registry/index merge; symmetric completion-criteria sections in all primers).
- **`SUITE-REVIEW.md`** — Review 31 row added to the Suite Reviews index.
- **`GAP-ANALYSIS-LOG.md`** — G-90 (phase numbering inconsistency), G-91 (primer folder + file naming), G-92 (suite-meta vs suite-running separation), G-93 (user vs contributor delineation), G-94 (smaller naming/location bundle: `supplements/` folder, `DOMAIN-INDEX.md` location, `SUITE-REVIEW.md` filename, CHANGELOG release tagging, primer H1 convention), G-95 (`implementation.md` covers two distinct phases) — all registered Deferred. Coordinated with G-88 (directory rename) and G-89 (project-level review log structure standardization) as a single bundled restructure pass at the spinoff-MVP boundary.

### Note
No suite artifacts were modified in this session beyond the registry, index, session log, and changelog. Per user instruction and consistent with the G-88/G-89 deferral pattern, structural changes are forward-only and will not be retroactively applied to completed projects (notably `bookmark-manager` and the in-flight `issue-tracker-cli`). The bundled application pass is expected to coincide with the suite's spinoff into a standalone repository.

---

## Unreleased — 2026-05-03 (suite scope acknowledgement)

### Changed
- **`README.md`** — Added `## Suite scope` section between the IAR intro and `## VSDD pipeline context`. Names the transitional state explicitly: directory began as IAR-only and has grown to house session primers for adjacent VSDD phases; directory name and "IAR" identity retained for continuity. Lists the four artifact categories (domain prompts, phase primers, lang supplements, suite governance). References the Phase 4–6 gaps (G-86, G-55, G-54). Addresses Review 30 Finding 1.
- **`README.md`** — Added "Primer" column to the VSDD pipeline table referencing each phase's primer file. Phases 1, 1b, 2a, 2b, 3 link to their primers; Phase 4 cell shows `— (G-86)`; Phase 5 cell shows `— (G-55)`; Phase 6 cell shows `—`. Updated the trailing sentence below the table to point to the full primer table under `## Session primers` rather than naming only Phase 1/1b. Addresses Review 30 Finding 2.
- **`prompts/suite-development.md`** — Updated `## Prompt` opening: added a paragraph after the artifact list naming the broader scope explicitly (suite has expanded beyond Phase 3; directory name retained for continuity; pointer to README scope section). Generalized the adversarial-standard paragraph with a sentence covering construction primers (a primer's `## Prompt` without a concrete failure mode produces softer output; a non-falsifiable completion-criteria section will pass against incomplete artifacts). Addresses Review 30 Finding 3.
- **`SUITE-REVIEW.md`** — Updated lead paragraph to include session primers and lang supplements in the implementation list, and to acknowledge the adversarial standard applies to both review prompts and constructive primers. Added pointer to `README.md` `## Suite scope` for the artifact map. Addresses Review 30 Finding 4.
- **`GAP-ANALYSIS-LOG.md`** — Added G-87: scope expansion was implicit; directory still named/framed as IAR-only despite housing primers for adjacent VSDD phases. Marked Addressed in same session by Review 30 Findings 1, 3, 4.
- **`GAP-ANALYSIS-LOG.md`** — Added G-88 (Deferred): revisit suite directory name and "IAR" identity after `issue-tracker-cli` completes. Forward-only constraint — any rename applies to projects whose first IAR run is after the decision; completed projects retain their existing `iterative-adversarial-refinement/` review-log paths.
- **`GAP-ANALYSIS-LOG.md`** — Added G-89 (Deferred): standardize project-level domain review log structure on the suite-review pattern (per-domain index file + dated session entries in `review-log/`). Trigger: revisit after `issue-tracker-cli` completes. Forward-only constraint — completed projects retain their existing single-file domain logs.

### Added
- **`review-log/2026-05-03-suite-review.md`** — Review 30 logged: SO + TW + VDD-IAR alignment scan of the suite's scope and identity. 4 findings resolved, 2 hallucinated (directory rename, primer scope-section), 3 new gaps registered (G-87 immediately addressed, G-88 and G-89 deferred to post-issue-tracker-cli).
- **`SUITE-REVIEW.md`** — Review 30 row added to the Suite Reviews index.

---

## Unreleased — 2026-05-02 (suite review collapse)

### Changed
- **`prompts/suite-development.md`** — Collapsed two parallel suite-review artifact types (meta-review and gap analysis run) into a single **Suite Review** type. Replaced the two entry-format specifications with one **Suite review entry format** subsection covering both modes (defect-search lens and registry-walk lens). The mode now lives in the `Lens` field rather than in a separate artifact. Updated cross-cutting wording in "Before adding a domain", "Before modifying a domain", and "Running gap analysis" to reference the unified type. Added project-level review log governing standard (introduced earlier in this session) referenced for finding-body shape.
- **`prompts/review-session.md`** — Updated "If reviewing the IAR suite itself" to reference the unified `review-log/YYYY-MM-DD-suite-review.md` filename pattern and the single **Suite Reviews** index table. Removed dual-type framing.
- **`SUITE-REVIEW.md`** — Replaced the two tables (Suite Meta-Reviews + Gap Analysis Runs) with a single **Suite Reviews** table. Renumbered all 29 sessions chronologically as Review 1–29 (oldest = Run 1, newest = Review 16). Added migration footnote with old→new mapping. Reading-convention text updated.
- **`review-log/`** — Renamed and merged session files: `2026-04-25-gap-analysis.md` → `2026-04-25-suite-review.md`; `2026-04-26-gap-analysis.md` → `2026-04-26-suite-review.md`; `2026-04-27-{gap-analysis,meta-review}.md` merged into `2026-04-27-suite-review.md`; `2026-04-28-{gap-analysis,meta-review}.md` merged into `2026-04-28-suite-review.md`; `2026-05-01-meta-review.md` → `2026-05-01-suite-review.md`. All H1 titles and `## Review N` / `## Gap Analysis Run N` headings updated to the unified numbering. Within-session prose references updated where unambiguous.
- **`GAP-ANALYSIS-LOG.md`** — Updated all 87 Markdown links to point to renamed files and renumbered anchors. Updated step 6 of "How to run a gap analysis" to direct entries to `review-log/YYYY-MM-DD-suite-review.md`. Updated trailing prose about narrative location.
- **Project-level review logs** under `issue-tracker-cli/iterative-adversarial-refinement/` — earlier in this session, applied the project-level review log governing standard to all 13 domain logs: file-level reviewer-role/activation/sycophancy headers, dim-ref parentheticals on every finding title, classification-first finding sections, Markdown-linked cross-references, unified Resolution/Classification closer, Summary + Coordination closing. Portfolio Assessment retained its dim-first organization as a documented exception.

### Note
Historical CHANGELOG entries below this one preserve their original wording and reference the pre-collapse artifact names ("meta-review", "gap analysis run", "Run N", "Review N" under the old numbering). Those entries describe state at points in time and are intentionally not rewritten.

---

## Unreleased — 2026-05-01 (session 18, follow-up)

### Changed
- **`prompts/decomposition.md`** — Added "Primary failure mode" paragraph to `## Prompt` section: names the specific failure mode for decomposition sessions (accepting all proposed layers without challenge). The governing standard requires the `## Prompt` section to name a primary failure mode; `decomposition.md` lacked this while `review-session.md` and others had explicit equivalents. Addresses finding from Review 16.
- **`prompts/implementation.md`** — Updated Phase 2b item 2 to define the retroactive Red Gate deviation protocol. Retroactive tests (discovered during implementation) cannot satisfy the Red Gate; the primer now requires them to be labeled as deviations in commit message and review log. Addresses finding from Review 16.
- **`prompts/spec-crystallization.md`** — Added dedicated `## Completion criteria` section with six numbered criteria drawn from VSDD Phase 1 standard. Removed embedded completion sentence from `## Self-adversary check` to avoid duplication. Structural consistency with `implementation.md` and `decomposition.md`. Addresses finding from Review 16.
- **`GAP-ANALYSIS-LOG.md`** — Added G-86: No VSDD Phase 4 (Feedback Routing) session primer. Open.
- **`SUITE-REVIEW.md`** — Review 16 logged: VDD-IAR and VSDD alignment review of all session primers in `prompts/`. 3 findings resolved, 2 hallucinated, 1 new gap registered (G-86).

### Changed (follow-up — same session, post-violation detection)
- **`prompts/implementation.md`** — Added Phase 2a step 4: explicit requirement to commit the Red Gate state before Phase 2b begins. The commit is the boundary between phases; implementation before that commit makes test-first discipline unverifiable from history. Updated Phase 2b opening to reference "set, confirmed failing, and committed." Finding surfaced by actual violation during Layer 2 implementation: Red Gate was confirmed in the working tree but not committed before implementations were written; the resulting "Red Gate" commit contained real implementations, not stubs.
- **`SUITE-REVIEW.md`** — Finding 6 added to Review 16 record: describes the violation, the primer gap that permitted it, and the resolution.

---

## Unreleased — 2026-04-27 (session 17)

### Changed
- **`README.md`** — Security Engineer Focus cell updated to include "audit logging, data classification and control requirements" — reflects dims 7 and 8 added in session 15. Cell was stale after scope expansion.
- **`README.md`** — Solution Architect Focus cell updated to include "external service integration" — reflects `### Extended: External Service Integration` (dims 23–27) added in session 15. Cell was stale after scope expansion.
- **`domains/role/SOLUTION-ARCHITECT-REVIEW.md`** — Added Privacy to coordination links: "dim 27 — data transmitted to external services; cross-reference with Privacy dim 6 when Privacy is active." Privacy was already cross-referenced inside dim 27's text but absent from the coordination section.

---

## Unreleased — 2026-04-28 (session 16)

### Changed
- **`domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md`** — Added cross-session spec consistency sub-section to dim 7 (IAR iteration and feedback routing). Named failure mode: AI interpretation of requirements shifts between sessions without a DESIGN.md update. Concrete test: can the current DESIGN.md, read cold, produce the current implementation? Named artifact indicators: commits contradicting DESIGN.md, DECISIONS.md entries expanding features without a spec revision, IAR findings about behavior absent from the spec. Addresses G-22.
- **`domains/role/SOLUTION-ARCHITECT-REVIEW.md`** — Added feature-enhancement activation note to `### Extended: External Interface Contracts` section. Dims 16 (backward compatibility) and 17 (contract testing) now explicitly activate for feature enhancements — any change that existing callers, users, or stored data must survive. Addresses G-30.
- **`GAP-ANALYSIS-LOG.md`** — Updated statuses: G-22 Open → Addressed; G-30 Open → Addressed.

---

## Unreleased — 2026-04-28 (session 15)

### Added
- **`domains/role/SECURITY-REVIEW.md`** — Added dim 7 (Audit logging): named audit events, tamper-evidence requirement, retention and separation from application logs, forensic reconstruction test, context-scoped note for single-user vs. enterprise deployment. Addresses G-09.
- **`domains/role/SECURITY-REVIEW.md`** — Added dim 8 (Data classification and control requirements): classification tiers (public/internal/confidential/restricted), proportionate control requirements, named failure modes, explicit cross-reference to Privacy dim 1 for coordination. Addresses G-10. Ownership decision: Privacy dim 1 owns data identification; Security dim 8 owns control mandates from classification.
- **`domains/role/SOLUTION-ARCHITECT-REVIEW.md`** — Added `### Extended: External Service Integration` section (dims 23–27): external dependency inventory, failure and timeout handling, API contract drift, credentials to external services, data transmitted to external services with cross-reference to Privacy dim 6. Addresses G-32.

### Changed
- **`domains/role/SECURITY-REVIEW.md`** — Added Privacy to coordination links (dim 8 data classification cross-references Privacy dim 1). Addresses G-09/G-10.
- **`GAP-ANALYSIS-LOG.md`** — Updated statuses: G-09 Open → Addressed; G-10 Open → Addressed; G-32 Open → Addressed; G-36 Open → Dismissed (business viability is out of IAR scope, no natural reviewer role).

---

## Unreleased — 2026-04-28 (session 14)

### Added
- **`supplements/javascript-typescript.md`** — Added `## Technical Writer` section: TypeDoc/JSDoc generation config, TSDoc comment completeness (`@param`/`@returns`/`@throws`/`@example`), README example accuracy, `@deprecated` markers. Addresses G-84.
- **`supplements/javascript-typescript.md`** — Added `## Localization` section: `Intl.*` API usage with explicit locale parameters, i18next/react-i18next configuration, missing translation key handling, locale injection in tests. Addresses G-85.
- **`supplements/rust.md`** — Added `## Technical Writer` section: rustdoc coverage (`cargo doc --no-deps`), doc test quality (`cargo test --doc`), module-level `//!` docs, `#[doc(hidden)]` discipline, `cargo doc --document-private-items`. Addresses G-84.
- **`supplements/rust.md`** — Added `## Localization` section: fluent-rs bundle configuration with `LanguageIdentifier` and fallback chains, Fluent message completeness, missing message error handling, rust-i18n macro usage and key coverage. Addresses G-85.

### Changed
- **`domains/role/TECHNICAL-WRITER-REVIEW.md`** — Updated lang supplement note from gap-reference language ("not yet covered — see G-84") to standard "Apply the **Technical Writer** section" format. Addresses G-84.
- **`domains/role/LOCALIZATION-REVIEW.md`** — Updated lang supplement note from gap-reference language ("not yet covered — see G-85") to standard "Apply the **Localization** section" format. Addresses G-85.
- **`domains/role/SECURITY-REVIEW.md`** — Revised inline G-07 note at end of dim 6. Previous note said "G-07 is still open... single dimension above is insufficient." Security dim 6 has been substantially expanded since that note was written; the note was stale and contradicted the current dimension content. Replaced with forward-looking guidance: for complex multi-user auth, a dedicated domain may be warranted. Gap log updated to Addressed (partial) for G-07 and G-08.
- **`prompts/suite-development.md`** — Removed gap markers (`**Gap**`) from Technical Writer and Localization rows in the lang supplement coverage table; updated to ✓. Removed now-stale trailing sentence referencing G-84/G-85. Addresses G-84, G-85.
- **`GAP-ANALYSIS-LOG.md`** — Updated statuses: G-07 Open → Addressed (partial); G-08 Open → Addressed (partial); G-24 Open → Addressed (partial); G-25 Open → Addressed (partial); G-84 Open → Addressed; G-85 Open → Addressed.

---

## Unreleased — 2026-04-28 (session 13)

### Changed
- **`domains/role/DATA-ENGINEER-REVIEW.md`** — Added parenthetical job title variants to the reviewer role line. Previous: `**Reviewer role: Data Engineer**`. Updated: `**Reviewer role: Data Engineer** (Data Engineer / Database Engineer / Data Platform Engineer)`. Every other role domain follows the governing standard format `[Title] ([variants])`; DE was the only exception. Addresses Review 14 Finding 1.
- **`README.md`** — Updated core domain table "Job title" column for Data Engineer from "Data Engineer" to "Data Engineer / Database Engineer / Data Platform Engineer" to match the domain file and align with the slash-delimited variant format used by all other rows. Addresses Review 14 Finding 1.

---

## Unreleased — 2026-04-28 (session 12)

### Changed
- **`prompts/review-session.md`** — Added VDD-IAR Alignment to the Deferred classification exclusion note. Previous note said "Not valid for Security or Red Team" — VDD-IAR Alignment also prohibits deferred (governing standard: process findings are binary). A reviewer following the primer could incorrectly defer a VDD-IAR Alignment finding. New note: "Not valid for Security, Red Team, or VDD-IAR Alignment." Addresses Review 13 Finding 1.

---

## Unreleased — 2026-04-28 (session 11)

### Changed
- **`CHANGELOG.md`** — Fixed "AIR suite" → "IAR suite" in the file description. Addresses Review 12 Finding 1.
- **`domains/role/PERFORMANCE-ENGINEER-REVIEW.md`** — Rewrote lang supplement note to match the standard format used by all other domains: "Apply the **Performance Engineer** section from the relevant supplement file in addition to the standard dimensions below." Previous note said only "Consult `../../supplements/`" without naming the section. Addresses Review 12 Finding 2.
- **`domains/role/SOLUTION-OWNER-REVIEW.md`** — Replaced supplement note with an explicit opt-out. Previous note directed reviewer to "consult the supplement" for technology choice verification, but no SO section exists in any supplement — `suite-development.md` table marks SO as Language-agnostic. New note clarifies: SO is language-agnostic; for technology fitness context, consult the SA section of the relevant supplement. Addresses Review 12 Finding 3.

---

## Unreleased — 2026-04-28 (session 10)

### Changed
- **`domains/role/SOLUTION-ARCHITECT-REVIEW.md`** — Added `[DATA-ENGINEER-REVIEW.md]` to SA's coordination links. SA dim 3 evaluates data model integrity; DE is the natural escalation target for deeper data-layer analysis. DE was the only core domain absent from SA coordination despite the explicit overlap. Addresses Review 11 Finding 1.
- **`supplements/javascript-typescript.md`** — Added "Coverage enforcement" bullet to Platform Engineering section naming Jest `coverageThreshold`, Vitest `coverage.thresholds`, `c8`, and `nyc` as the JS/TS-specific coverage tooling. Symmetric with `rust.md` Platform Engineering section. Addresses Review 11 Finding 2.
- **`domains/role/ACCESSIBILITY-REVIEW.md`** — Updated lang supplement note to specify "See the **UX** section of `../../supplements/browser-app.md`" and name the content. `browser-app.md` has no `## Accessibility` section; the relevant dimensions live in `## UX`. Addresses Review 11 Finding 3.

---

## Unreleased — 2026-04-28 (session 9)

### Changed
- **`README.md`** — Fixed stale "Phase 4" reference in session primers section: "the adversary applies pressure during Phase 1, not only during Phase 4" → "Phase 3". Renumbering pass in Review 6 missed this sentence. Addresses Review 10 Finding 1.
- **`domains/role/QUALITY-ENGINEER-REVIEW.md`** — Added `[SOFTWARE-ENGINEER-REVIEW.md]` as the first entry in QE's coordination links. SE was absent despite the domain boundary text explicitly describing the QE/SE split: QE flags missing tests, SE flags bugs. Addresses Review 10 Finding 2.
- **`domains/meta/PORTFOLIO-ASSESSMENT-REVIEW.md`** — Regression check now includes instruction to read the preceding project's `PORTFOLIO-ASSESSMENT-REVIEW.md` log and a note that the check is vacuously met if no prior assessment exists. Addresses Review 10 Finding 3.
- **`domains/meta/PORTFOLIO-ASSESSMENT-REVIEW.md`** — Dim 8 rewritten: replaced "the developer could have built this without AI assistance" with a framing based on ownership of scope. The governing methodology assumes AI does the building; the test is whether the developer directed and owns the complexity, not whether they could have built it solo. Addresses Review 10 Finding 4.

---

## Unreleased — 2026-04-28 (session 8)

### Changed
- **`supplements/rust.md`** — Removed six inline "(Source: claude.md; verify against current apprentice-onboarding content.)" annotations from dimension bullets across Quality Engineering, Security, Software Engineering, and Platform Engineering sections. Source provenance is now consolidated in the `**Source note:**` paragraph added at the top of the file. Addresses Review 9 Finding 1.
- **`GAP-ANALYSIS-LOG.md`** — Updated G-12 status from "Addressed (API-CONTRACT-REVIEW.md)" to "Addressed (SA Extended: External Interface Contracts)". The referenced file does not exist; the gap was addressed by SA's Extended: External Interface Contracts section. Addresses Review 9 Finding 2.
- **`GAP-ANALYSIS-LOG.md`** — Updated G-20, G-21, G-23 status from "Open" to "Addressed (partial)" and Last Reviewed from 2026-04-25 to 2026-04-27, consistent with G-76 which registered the partial addressing. Addresses Review 9 Finding 3.
- **`supplements/cli.md`** — Removed "(or alongside)" from intro paragraph. Intro now reads "in place of the standard UX dimensions," matching the section header which states the CLI dimensions replace browser-centric UX dimensions. Addresses Review 9 Finding 4.

---

## Unreleased — 2026-04-27 (session 7)

### Changed
- **`prompts/implementation.md`** — Updated H1, prompt text, and internal section headers from "Phase 2–3" / "Phase 2" / "Phase 3" to "Phase 2a–2b" / "Phase 2a" / "Phase 2b" to match the renumbering from Review 6. Addresses Review 8 Finding 1.
- **`domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md`** — Added disambiguation note to Program Phase Context section clarifying "Phase" refers to apprentice program tiers, not VSDD pipeline phases. Addresses Review 8 Finding 2.
- **`domains/role/ACCESSIBILITY-REVIEW.md`** — Removed dim 13 ("Regression") which duplicated the regression check paragraph already present in the Current Review Prompt section. Addresses Review 8 Finding 3.
- **`domains/role/TECHNICAL-WRITER-REVIEW.md`**, **`ACCESSIBILITY-REVIEW.md`**, **`LOCALIZATION-REVIEW.md`**, **`PERFORMANCE-ENGINEER-REVIEW.md`**, **`PRIVACY-REVIEW.md`** — Converted prose coordination sections to Markdown links with relative paths and parenthetical context preserved. **`RED-TEAM-REVIEW.md`** — Fixed abbreviated display name "[SA-REVIEW.md]" → "[SOLUTION-ARCHITECT-REVIEW.md]". Addresses Review 8 Finding 4.
- **`README.md`** — Added cross-reference to `PORTFOLIO-ASSESSMENT-REVIEW.md` in portfolio-arc review section. Addresses Review 8 Finding 5.

---

## Unreleased — 2026-04-27 (session 6)

### Changed
- **`domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md`** — Added regression check paragraph in correct position (before Coordination). Moved sycophancy check to before lang supplement (was after). Addresses Review 7 Finding 1.
- **`domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md`** — Dim 7 extended with feedback routing fidelity sub-criterion: findings must route to the appropriate earlier phase (spec findings → DESIGN.md, test findings → test suite, implementation findings → code). Addresses Review 7 Finding 3.
- **`domains/meta/PORTFOLIO-ASSESSMENT-REVIEW.md`** — Added "Read DESIGN.md and the assignment brief" instruction. Added regression check paragraph. Moved sycophancy check to after Coordination (was before). Addresses Review 7 Finding 2.

---

## Unreleased — 2026-04-27 (session 5)

### Changed
- **`README.md`** — Phase numbering aligned with VSDD whitepaper. Pipeline table: phases renamed 2→2a, 3→2b, 4→3; Phase 4 (Feedback Integration) row added (was absent). Opening paragraph and VSDD pipeline context section updated to "IAR owns Phase 3." Session primer table updated: "Phase 2–3" → "Phase 2a–2b"; "Phase 4" → "Phase 3." Addresses Review 6 Finding 1.
- **`README.md`** — Added same-model review limitation note to Session isolation section. Addresses Review 6 Finding 2.
- **`prompts/review-session.md`** — H1 title and posture paragraph updated from "VSDD Phase 4" to "VSDD Phase 3." Addresses Review 6 Finding 1.
- **`domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md`** — Purpose statement updated from "VSDD Phase 4 (Adversarial Refinement)" to "VSDD Phase 3." Addresses Review 6 Finding 1.

---

## Unreleased — 2026-04-27 (session 4)

### Changed
- **`domains/role/TECHNICAL-WRITER-REVIEW.md`**, **`LOCALIZATION-REVIEW.md`**, **`ACCESSIBILITY-REVIEW.md`**, **`PRIVACY-REVIEW.md`**, **`PERFORMANCE-ENGINEER-REVIEW.md`** — Regression check paragraphs moved to correct position: before Coordination, not after. Ordering violation introduced in session 3 resolution pass. Addresses Finding 1 from Review 5.
- **`domains/role/SOLUTION-OWNER-REVIEW.md`** — Added regression check paragraph in correct position (before Coordination). Addresses Finding 2 from Review 5.
- **`domains/role/RED-TEAM-REVIEW.md`** — Added regression check paragraph in correct position (before Coordination). Addresses Finding 3 from Review 5.
- **`domains/role/DOMAIN-INDEX.md`** — Fixed broken relative paths to meta domain files: `../../meta/` → `../meta/`. Addresses Finding 4 from Review 5.
- **`README.md`** — Data Engineer Focus column: removed activation guidance ("Optional for projects without a meaningful data layer") which belongs exclusively in DOMAIN-INDEX; replaced with reference to DOMAIN-INDEX for scope-down guidance. Addresses Finding 5 from Review 5. **README vs. DOMAIN-INDEX:** These files serve distinct purposes and are not redundant. README describes what domains cover; DOMAIN-INDEX is authoritative for when and whether domains activate.
- **`domains/role/DATA-ENGINEER-REVIEW.md`** — Added PRIVACY-REVIEW.md to coordination links with escalation note for dim 9 findings. Addresses Finding 10 from Review 5.
- **`domains/meta/PORTFOLIO-ASSESSMENT-REVIEW.md`** — Coordination links converted from prose to relative-path Markdown links. Addresses Finding 11 from Review 5.
- **`domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md`** — Moved `## Governing References` section from before `## Current Review Prompt` to between the prompt section and `## Standard Evaluation Dimensions`. Added prerequisite preamble framing. Addresses Finding 7 from Review 5.
- **`domains/role/SOFTWARE-ENGINEER-REVIEW.md`** — Added coordination notes to both Extended sections: Extended: Documentation defers to TW when TW is active; Extended: Performance defers to PE when PE is active. Addresses Finding 8 from Review 5.
- **`prompts/spec-crystallization.md`** — Moved `## Project type` section from before `## Prompt` to after it (between `## Prompt` closing separator and `## Project description`), satisfying the governing standard's required element ordering. Addresses Finding 9 from Review 5.
- **`prompts/suite-development.md`** — Added explicit meta-domain exception to element 3 (Reviewer role line): meta domains in `domains/meta/` are exempt from the reviewer role requirement by design. Addresses Finding 6 from Review 5.
- **`prompts/review-session.md`** — Rewrote the "After each domain review" classification section as a complete taxonomy with per-domain callouts for non-standard classifications. Added guidance that domain file schemas are authoritative. Addresses Finding 12 from Review 5.
- **`SUITE-REVIEW.md`** — Added Review 5 entry covering all 12 findings.

---

## Unreleased — 2026-04-27 (session 3)

### Changed
- **`SUITE-REVIEW.md`** — Added `## Suite Meta-Reviews` section header before `## Review 3` to match the structural organization of `## Gap Analysis Runs`. Added Review 4 entry documenting all 11 findings from the adversarial pass this session.
- **`domains/role/TECHNICAL-WRITER-REVIEW.md`** — Supplement reference updated to acknowledge G-84 (open gap) and removed false implication that a `supplements/` section exists. Added regression check paragraph. Addresses Finding 1 and Finding 6 from Review 4.
- **`domains/role/LOCALIZATION-REVIEW.md`** — Supplement reference updated to acknowledge G-85 (new open gap) and removed false implication that a `supplements/` section exists. Added regression check paragraph. Addresses Finding 2 and Finding 6 from Review 4.
- **`domains/role/RED-TEAM-REVIEW.md`** — Supplement reference corrected: "Apply the **Security** section" → "Apply the **Red Team** section." Addresses Finding 3 from Review 4.
- **`domains/role/SOLUTION-OWNER-REVIEW.md`** — Sycophancy check rewritten with domain-specific failure mode: an agent that helped write DESIGN.md will not flag scope creep it introduced. Addresses Finding 5 from Review 4.
- **`domains/role/UX-REVIEW.md`** — Sycophancy check rewritten with domain-specific failure mode: an AI cannot experience a UI. Added note to dim 7 directing deeper accessibility coverage to the Accessibility domain. Addresses Finding 5 and Finding 11 from Review 4.
- **`domains/role/DATA-ENGINEER-REVIEW.md`** — Sycophancy check rewritten with domain-specific failure mode: an agent that designed the data model will not question schema decisions. Addresses Finding 5 from Review 4.
- **`domains/role/ACCESSIBILITY-REVIEW.md`** — Added regression check paragraph. Addresses Finding 6 from Review 4.
- **`domains/role/PRIVACY-REVIEW.md`** — Added regression check paragraph. Addresses Finding 6 from Review 4.
- **`domains/role/PERFORMANCE-ENGINEER-REVIEW.md`** — Added regression check paragraph. Addresses Finding 6 from Review 4.
- **`README.md`** — Candidate domains list updated: removed Performance, Privacy, and Internationalisation (all now implemented extended domains). Expanded "Review logs" example file tree to include the 6 extended domain log files with an "include only when active" note. Addresses Finding 4 and Finding 8 from Review 4.
- **`prompts/suite-development.md`** — Classification schemas table expanded: added Privacy (`accepted risk`), Localization (`accepted scope`), and Portfolio Assessment (`demonstrated`/`partial`/`absent`/`hallucinated`). Added numbered format definition for gap analysis run entries in `SUITE-REVIEW.md` to "SUITE-REVIEW.md discipline" section. Coverage table: Localization row updated from "Language-agnostic" to gap (G-85); closing note updated to reference both G-84 and G-85. Addresses Finding 7, Finding 10, and Finding 2 from Review 4.
- **`GAP-ANALYSIS-LOG.md`** — Added G-85 (Localization lang supplement absent). Updated G-77 status from `Addressed (QE/Security/SA/SE)` to `Addressed` (now fully resolved across SO, UX, DE as well).

---

## Unreleased — 2026-04-27 (session 2)

### Changed
- **`GAP-ANALYSIS-LOG.md`** — Structural consolidation: run narratives (Runs 1–10) stripped from this file. The file now contains only the gap registry table and file header. Run narrative content moved to `SUITE-REVIEW.md`. Each registry ID is now a Markdown link to the `## Gap Analysis Run N` section in `SUITE-REVIEW.md` where that gap was first identified. G-84 links to `SUITE-REVIEW.md#review-3--2026-04-27` (identified in a meta-review, not a gap analysis run). "How to run a gap analysis" step 6 updated to direct run entries to `SUITE-REVIEW.md`.
- **`SUITE-REVIEW.md`** — Added `## Gap Analysis Runs` section containing all ten gap analysis run narratives (Runs 1–10) in reverse chronological order, consistent with the file's existing convention.
- **`prompts/suite-development.md`** — "Running gap analysis" section updated: the closing entry requirement now explicitly names `SUITE-REVIEW.md` as the target for run narratives, with a clarifying note that `GAP-ANALYSIS-LOG.md` contains only the registry.

---

## Unreleased — 2026-04-27

### Added
- **`domains/role/DOMAIN-INDEX.md`** — Authoritative classification of core vs. extended domains with activation criteria per extended domain. Supplements the README domain tables with filesystem-local reference and explicit conditions under which each extended domain becomes active.
- **`prompts/implementation.md`** — New session primer for VSDD Phase 2–3 (Red Gate and Implementation). Establishes tests-before-code posture, driving questions for test writing and implementation, Red Gate anti-patterns to reject, and completion criteria. Fills the phase coverage gap: spec-crystallization and decomposition primers existed for Phase 1/1b; implementation was unprimed.
- **`prompts/review-session.md`** — New session primer for VSDD Phase 4 (Adversarial Review). Establishes adversarial posture before loading any domain prompt. Names sycophancy failure modes at the session level (not domain level), covers DESIGN.md prerequisite check, domain selection and sequencing, session isolation, and post-review classification requirements.
- **`prompts/suite-development.md`** — New session primer for IAR suite development work. Governs adding/modifying domains, dimensions, and primers. Specifies the complete domain file structure, primer structure, pre-change checklists, gap registry discipline, SUITE-REVIEW.md and CHANGELOG.md requirements, and a lang supplement coverage table.
- **`SECURITY-REVIEW.md` — `## Threat Model` section** — Required prerequisite section added before Standard Evaluation Dimensions. Before applying the checklist, the reviewer must name threat actors, crown jewel, and entry points. Output is logged as a preamble record in the review log, not a classified finding. Addresses G-06.
- **`supplements/javascript-typescript.md` — Red Team section** — JS/TS-specific attack vectors: prototype pollution exploitation (payload format, mitigation patterns), DOM-based XSS sinks enumeration, JWT algorithm confusion (alg:none, library version verification), npm supply chain and dependency confusion, localStorage/sessionStorage as persistence injection surface.
- **`supplements/javascript-typescript.md` — Performance Engineer section** — Bundle size analysis tooling (webpack-bundle-analyzer, source-map-explorer), V8 profiling via Chrome DevTools, Web Vitals as performance contract (LCP/INP/CLS targets), event delegation efficiency.
- **`supplements/rust.md` — Red Team section** — Rust-specific attack vectors: integer overflow in release builds (wrapping arithmetic, `u32::MAX` boundaries), panic as DoS vector (`.unwrap()` on user-influenced paths), path traversal via `Path::join`, `unsafe` block exploitation, crates.io supply chain.
- **`supplements/rust.md` — Performance Engineer section** — Criterion benchmarking discipline, flamegraph profiling (`cargo flamegraph`), debug vs. release build performance differential, allocation patterns in hot paths, async blocking operations in executor threads.
- **`README.md` — Expanded primers table** — Added Implementation, Adversarial Review, and Suite Development primers with when-to-use descriptions. Primer table now covers all five session types.
- **`README.md` — Running IAR preamble** — Three new constraints before the refinement loop: human-in-the-loop requirement (IAR's adversarial value collapses without human classification decisions), DESIGN.md prerequisite (no domain reviews without a spec), domain activation guidance (pointer to DOMAIN-INDEX.md).

### Changed
- **Domain folder restructure** — All domain files moved from root to `domains/role/` (role domains) and `domains/meta/` (meta domains). All internal links updated to relative paths (`../../README.md`, `../../supplements/`).
- **Domain file renames** — `SOFTWARE-ENGINEERING-REVIEW.md` → `SOFTWARE-ENGINEER-REVIEW.md`, `QUALITY-ENGINEERING-REVIEW.md` → `QUALITY-ENGINEER-REVIEW.md`, `PLATFORM-ENGINEERING-REVIEW.md` → `PLATFORM-ENGINEER-REVIEW.md`, `DATA-ENGINEERING-REVIEW.md` → `DATA-ENGINEER-REVIEW.md`. H1 titles updated to match. All cross-references updated.
- **`PERFORMANCE-REVIEW.md` → `PERFORMANCE-ENGINEER-REVIEW.md`**, **`DOCUMENTATION-REVIEW.md` → `TECHNICAL-WRITER-REVIEW.md`** — Renamed to role-based titles. H1 titles and Reviewer role lines updated.
- **README domain tables** — Restructured into three categories: Core role domains (8, always active), Extended role domains (6, activation-conditional), Meta domains (2). Added Role and Job title columns. All file paths updated to new folder locations.
- **Reviewer role lines** — Added `**Reviewer role: [Title]** ([Job title variants])` to all domain files that were missing it. Duplicate lines removed from 10 files where the line was accidentally inserted twice.
- **`PORTFOLIO-ASSESSMENT-REVIEW.md`** — Added explicit lang supplement opt-out line with rationale.
- **`prompts/suite-development.md`** — Governing standard updated: lang supplement reference is required OR must have explicit opt-out with rationale; element 6 now distinguishes required structural sections (prerequisite records) from optional extended sections (conditional sub-dimensions).

### Fixed
- **`RED-TEAM-REVIEW.md`** — Coordination link text corrected: `[QE-REVIEW.md]` → `[QUALITY-ENGINEER-REVIEW.md]`.
- **`GAP-ANALYSIS-LOG.md`** — Duplicate rows for G-02, G-03, G-12, G-34 removed; original rows updated in-place to Addressed status. G-80–G-83 moved from appended duplicates to proper numeric position in registry table. G-06, G-19, G-27 updated to Addressed. G-84 added (Technical Writer lang supplement gap).

### Removed
- **`OBSERVABILITY-REVIEW.md`** — Content absorbed into `PLATFORM-ENGINEER-REVIEW.md` dims 27–33 (error surfacing, error classification, diagnostic completeness, health surfaces, sensitive data exclusion, silent success confirmation, runbook coverage).
- **`API-CONTRACT-REVIEW.md`** — Content absorbed into `SOLUTION-ARCHITECT-REVIEW.md` Extended: External Interface Contracts (dims 13–22).

---

## Unreleased — 2026-04-26 (session 3)

### Added
- **`VDD-IAR-ALIGNMENT-REVIEW.md`** — New domain. Evaluates whether the VDD-IAR methodology was actually followed. The other domains evaluate what was built; this domain evaluates how. Reference document is the governing methodology doc (`apprentice-onboarding/02-the-methodology/01-how-we-build.md` for guild projects). Ten dimensions: design-before-code, layered decomposition, layer gate compliance, test discipline, human verification, IAR fresh context, IAR iteration, role integrity, manual testing checklists, retrospective quality. The sycophancy check is specifically scoped to the rationalization risk: the reviewing agent participated in building the project and has every incentive to find the process acceptable.

### Changed
- **README.md** — Major restructure to reflect VDD-IAR as the governing framework:
  - Opening now names AIR as the adversarial mechanism of VDD, describes the full loop (design → build → verify → adversarial refinement → fix → repeat until MVR), and states explicitly that AIR is not a pre-merge checkpoint but an active part of the build cycle
  - **Refinement loop** replaces "Full run" section: describes within-layer iteration (first pass → fix → second pass → repeat until MVR), requires round numbers in logs
  - **Session isolation** moved to its own paragraph with clearer framing
  - **Generalist adversary pass** added as an optional step: unstructured, no domain framework, finds what specialists missed; lives as a README note rather than a formal domain
  - Domain table updated with VDD-IAR Alignment; focus descriptions reformatted consistently
  - Sequencing updated: VDD-IAR Alignment runs last (reviews process artifacts produced by all other runs)
  - Merging gate updated: requires MVR (not just one passing run), adds VDD-IAR Alignment as a required gate, adds round numbers to log format
- **SOLUTION-OWNER-REVIEW.md** — Removed dims 9 (complexity budget for one → SA), 11 (VDD process fidelity → VDD-IAR Alignment), 12 (linear accountability → VDD-IAR Alignment). Assignment compliance renumbered to dim 9. SO returns to its original identity: the spec contract.
- **SOLUTION-ARCHITECT-REVIEW.md** — Dim 9 (complexity budget) expanded to include maintainer-scale complexity. Now covers both problem-proportionate complexity and team-proportionate complexity. Cross-references SO dim 4 (over-engineering) to distinguish the two concerns.
- **QUALITY-ENGINEER-REVIEW.md** — Removed dim 14 (manual testing checklists → VDD-IAR Alignment). Added domain boundary statement: QE owns the test system; SE owns the bugs. When QE finds a logic error with no test, flag the missing test here; SE flags the bug.
- **SOFTWARE-ENGINEER-REVIEW.md** — Added domain boundary statement: SE owns the implementation; QE owns the test system. SE flags bugs; QE flags missing tests. Do not duplicate by evaluating test architecture in SE.
- **PLATFORM-ENGINEER-REVIEW.md** — Replaced generic sycophancy check with a posture note acknowledging that most PE dimensions are compliance checks, not adversarial judgment calls. Sycophancy risk is specifically scoped to inapplicability decisions and threshold acceptance. This is more honest about what PE actually does.

---

## Unreleased — 2026-04-26 (session 2)

### Added
- **`hallucinated` finding classification** — Added to all 8 domain prompts. A finding is hallucinated when the adversary invented a problem that does not exist and push back is warranted. Consistent hallucinated findings are the maximum viable refinement signal: real issues have been exhausted. Security uses "invented a vulnerability"; SO uses "invented a scope deviation or compliance failure" to match each domain's adversarial framing.
- **Solution Owner dim 10 — Assignment compliance** — New dimension. Checks whether DESIGN.md itself accurately reflects the upstream assignment brief, not just whether the implementation matches DESIGN.md. Scope creep that enters at the design stage will pass every other SO dimension and still fail an external review. Requires reading the assignment instructions alongside DESIGN.md.
- **Solution Owner dim 11 — VDD process fidelity** — New dimension. Checks whether the VDD loop was actually followed: DESIGN.md before code, layered commit history, layer gates completed before advancing, tests written alongside or before implementation, AIR run before each merge. A correct product built without process discipline is not evidence of the capability the process is designed to develop.
- **Solution Owner dim 12 — Linear accountability** — New dimension. Can every piece of code be traced to a specific task, issue, or requirement? Are commit messages specific enough to identify what was built and why? Evaluates the string-of-beads principle: every change should have a bead, and every bead should be accounted for.
- **Quality Engineering dim 14 — Manual testing checklists** — New dimension. Does the project have a manual testing checklist per layer or feature? Was it completed before the layer was marked done? Automated tests verify correctness; manual testing catches intent mismatches, UX problems, and "technically correct but not what I meant" failures. Absence of a manual checklist is a quality gap regardless of coverage.
- **README — Session isolation** — Operational note under "Full run": resetting the AI session between domain reviews gives each domain the same adversarial intensity. An agent that reviews all 8 domains in one session accumulates context that softens its pressure. Parallel sessions are the gold standard.
- **README — Maximum viable refinement** — Explanation of the MVR exit signal in the context of the `hallucinated` classification. When an adversarial domain produces only hallucinated findings, real issues have been exhausted. Log the final round with push back reasoning so the record shows how the exit signal was reached.
- **README — Portfolio-arc review** — New section under "Suggesting new domains". Describes a cross-project pass to be run before portfolio submission, evaluating: growth across projects, honest retrospectives, assignment alignment patterns, independence evidence, and process over product.

---

## Unreleased — 2026-04-26

### Added
- **`supplements/` subfolder** — Language and interface type supplements. Domain files reference these during review; reviewers apply the relevant supplement's section alongside the standard dimensions for that domain.
  - `supplements/rust.md` — Rust-specific dimensions for QE (doc tests, clippy, integration tests against binary), Security (`.unwrap()` discipline, `cargo audit`, unsafe rationale), SE (error propagation, error type hierarchy, clippy as idiom proxy), PE (`cargo audit`, `cargo clippy --deny warnings`, `cargo fmt --check`, `Cargo.lock` for binaries, toolchain pinning), DE (`serde` boundary validation, `#[serde(default)]` for schema evolution), SA (CLI parsing separation, command enum dispatch, `lib.rs`/`main.rs` split)
  - `supplements/javascript-typescript.md` — JS/TS-specific dimensions for QE (`npm ci`, axe scanning, browser tests, type coverage), Security (rendering safety, URL injection, `JSON.parse` runtime validation, CSP, `npm audit`), SE (`as` casts require runtime validation, `any` types, non-null assertions, unhandled promise rejections), PE (`npm ci`, `package-lock.json`, `npm audit`, Node pinning, `tsc --noEmit`), DE (runtime schema validation, `JSON.parse` error handling, normalization, date handling)
  - `supplements/cli.md` — CLI interface type supplement. Replaces browser-centric UX dimensions with 11 CLI UX dimensions (command discoverability, stdout/stderr discipline, exit codes, empty state messages, destructive confirmation, machine-readable output, verbose/quiet modes, error message quality, interruption handling). Adds CLI-specific QE dimensions (integration tests invoke binary, full stdout/stderr/exit code assertions) and SE dimensions (output formatting as a code concern, structured result types before formatting).
  - `supplements/browser-app.md` — Browser interface type supplement with QE (axe scanning, browser compat, responsive testing, keyboard navigation), Security (rendering safety, URL injection, CSP, storage validation, SRI), and UX (accessibility, responsive design, browser compatibility, reduced motion, native dialog quality) dimensions.
- **Sycophancy check** — Added to all 8 domain prompts. Explicitly names AI self-validation as a failure mode: if the reviewing agent agrees with every decision without challenge, that agreement itself is a finding.
- **Solution Owner dim 9 — Complexity budget for one** — New dimension evaluating whether architectural complexity is proportionate to the maintenance team size. An AI agent defaults to team-scale practices regardless of the project's actual maintenance model. Distinct from over-engineering (which flags complexity beyond spec); this flags complexity that is proportionate to spec but disproportionate to the team.
- **Solution Owner — `approved deviation` classification** — New finding classification for deviations from DESIGN.md that were explicitly approved by the stakeholder prior to implementation. Requires documentation of the approval and rationale.
- **Solution Architect dim 11 — Session continuity** — New dimension: are architectural decisions and rationale documented in a form a new AI session can act on without rediscovering them? Decisions that live only in conversation history are invisible to future sessions.
- **Software Engineering dim 11 — Future-self maintainability** — New dimension: will you be able to understand and modify this code in six months without the original AI session? Are key decisions derivable from the code and its comments?
- **GAP-ANALYSIS-LOG.md Run 2** — 2026-04-25 21:30Z. Context: AI-accelerated consulting team. Identified 15 new gaps (G-18–G-32) including: Requirements and Business Analysis domain, Documentation Fidelity domain, AI assumption surfacing, hallucination detection, context drift checking, dependency/API existence validation, test gaming detection, AI-generated code anti-patterns, Change Management, Knowledge Transfer, Client/Stakeholder Alignment, integration architecture.
- **GAP-ANALYSIS-LOG.md Run 3** — 2026-04-25 22:00Z. Context: personal developer using AI-accelerated tools, portfolio-to-side-business trajectory. Identified 6 new gaps (G-33–G-38) including: sycophancy detection (G-33, addressed), future-maintainability-for-one assessment (G-35, addressed), session continuity across AI conversations (G-37, addressed), complexity trap from AI over-engineering (G-38, addressed).

### Changed
- **QUALITY-ENGINEER-REVIEW.md** — Removed browser-specific dimensions 11–13 (accessibility, browser compatibility, responsive design) from standard dimensions; these are now in `supplements/browser-app.md`. Generalized dim 14 (security surface) to remove npm-specific language. Renumbered to 13 dimensions. Added language and interface supplement instruction.
- **SECURITY-REVIEW.md** — Removed web-specific dimensions 1 (rendering safety), 2 (URL injection), and 5 (CSP) from standard dimensions; these are now in `supplements/browser-app.md` and `supplements/javascript-typescript.md`. Generalized remaining dimensions to be language-agnostic. Added dim 4 (secret handling) and dim 6 (authentication/authorization) as generic security dimensions. Renumbered to 6 dimensions. Added language and interface supplement instruction.
- **UX-REVIEW.md** — Added interface-type note: standard dimensions assume a browser-rendered interface; CLI projects should consult `supplements/cli.md`; browser apps should also consult `supplements/browser-app.md`.
- **PLATFORM-ENGINEER-REVIEW.md** — Generalized npm-specific language in dims 1, 3, 4, and 11 to be ecosystem-agnostic with ecosystem-appropriate examples. Added language and interface supplement instruction.
- **SOLUTION-ARCHITECT-REVIEW.md** — Added language and interface supplement instruction.
- **SOLUTION-OWNER-REVIEW.md** — Added language and interface supplement instruction (SO review is primarily spec-driven; supplement used to verify technology choices against the spec).
- **SOFTWARE-ENGINEER-REVIEW.md** — Added language and interface supplement instruction.
- **DATA-ENGINEER-REVIEW.md** — Added language and interface supplement instruction.
- **GAP-ANALYSIS-LOG.md** — Fixed blank line between G-17 and G-18 rows that broke markdown table rendering. Updated gap registry statuses: G-33, G-35, G-37, G-38 marked Addressed.

---

## 2026-04-26 00:15Z — `db45cd2`

### Added
- **GAP-ANALYSIS-LOG.md** — New living document for gap analysis runs against the AIR suite itself. Includes re-run trigger conditions, instructions, and a gap registry table. Initial run (Run 1, 2026-04-25 20:00Z) evaluated against mission-critical and speculative project contexts. Identified 17 gaps (G-01–G-17) across 5 missing domains and 12 dimension-level gaps. Per-context severity (Mission-Critical / Speculative) recorded for each gap.

---

## 2026-04-25 23:56Z — `59ee04e`

### Added
- **PE dim 10 — Pre-commit hooks** — Platform Engineering now owns pre-commit hooks as a DevSecOps control. Hooks cover: secret and credential detection (API keys, tokens, private keys, connection strings); PII detection (email addresses, phone numbers, government IDs); committer identity and local machine leakage (absolute paths with usernames, hostnames, local environment details in configs or build output); large or binary files. Includes evaluation of `--no-verify` bypass risk.

### Changed
- **SECURITY-REVIEW.md** — Added coordination note: Security flags sensitive data patterns it identifies to Platform Engineering for incorporation into pre-commit hook detection rules.

---

## 2026-04-25 23:51Z — `0bef3f6`

### Changed
- **PLATFORM-ENGINEER-REVIEW.md** — Massively expanded from CI/CD-only to full delivery platform ownership across four areas:
  - **CI/CD** (dims 1–9): pipeline completeness, gate enforcement, dependency installation, environment pinning, cache correctness, coverage thresholds, action/dependency pinning, artifact hygiene, left-shift opportunities
  - **DevSecOps** (dims 10–15): security scanning, secret management, supply chain integrity, least privilege, compliance gates
  - **Infrastructure** (dims 16–21): Infrastructure as Code, cloud/on-premise resource hygiene, containerization, container security, environment parity, disaster recovery
  - **Observability** (dims 22–26): logging, metrics, alerting, distributed tracing, dashboards
  - Inapplicable sections may be skipped with rationale. A static single-user tool has no cloud infrastructure to evaluate.

---

## 2026-04-25 23:40Z — `2b6446a`

### Added
- **SOFTWARE-ENGINEER-REVIEW.md** — New domain. Evaluates implementation quality at the code level: correctness, error handling, naming, function design, duplication, complexity, type safety, defensive coding, comments and self-documentation, consistency. Distinct from Solution Architect (which evaluates structure and boundaries) and Quality Engineering (which evaluates the test system). 10 standard dimensions.
- **DATA-ENGINEER-REVIEW.md** — New domain. Evaluates the data layer: data model correctness, validation and normalization, schema evolution, data integrity invariants, storage fitness, access patterns, serialization, data consistency, sensitive data handling, test coverage of data paths. Marked optional for projects without a meaningful data layer.

### Changed
- **QA-REVIEW.md → QUALITY-ENGINEER-REVIEW.md** — Renamed via `git mv`. Scope broadened from bug-finding to test architecture and quality system: added test falsifiability (dim 2, "a test that cannot fail on a defective implementation has no value"), coverage meaningfulness (dim 4), test architecture and independence (dim 5), and quality gates (dim 16).
- **All domain prompts** — Added DESIGN.md as required first read for all domain reviews. All domains now treat DESIGN.md as authoritative context for the project's scope, constraints, and feature set.
- **All cross-domain coordination links** — Updated from `QA-REVIEW.md` to `QUALITY-ENGINEER-REVIEW.md`.
- **README.md** — Added Software Engineering and Data Engineering to domain table. Updated domain count and descriptions. Added note that not all domains are required for all projects. Updated sequencing (run DE before SA when data model changes are significant). Updated merging gate and log structure to reflect 8 domains.

---

## 2026-04-25 23:03Z — `6ea9b30`

### Added
Initial AIR suite. Six review domains extracted from the bookmark-manager project, generalized into a reusable template.

- **README.md** — Suite index: domain table, running instructions (full run, scoped run, sequencing), candidate domains, review log structure, merging gate.
- **QA-REVIEW.md** — Quality assurance: acceptance criteria, test coverage, validation gaps, logic errors, dead code, unused dependencies, dependency versions, accessibility, browser compatibility, responsive design, security surface.
- **UX-REVIEW.md** — User experience: empty states, error messages, focus and keyboard behavior, visual consistency, affordances, feedback patterns, accessibility, responsive design, browser compatibility, long content, reduced motion, native dialog quality.
- **SECURITY-REVIEW.md** — Security: rendering safety, URL injection, storage data validation, dependency CVEs, CSP, information exposure, input handling.
- **PLATFORM-ENGINEER-REVIEW.md** — CI/CD pipeline and gate enforcement.
- **SOLUTION-ARCHITECT-REVIEW.md** — Architecture: separation of concerns, coupling, data model integrity, interface contracts, state management, immutability, extensibility, technology fitness, complexity budget, decision documentation.
- **SOLUTION-OWNER-REVIEW.md** — Scope and delivery: spec coverage, scope creep, technology compliance, over-engineering, under-delivery, design fidelity, backlog candidates, prior-review additions. Opens every review with a compliance table (Met/Partial/Missing). DESIGN.md treated as a Scope of Work contract. "Quality does not justify scope."

Review logs are stored outside the prompt files. Logs live at `{project}/iterative-adversarial-refinement/` inside each reviewed project.
