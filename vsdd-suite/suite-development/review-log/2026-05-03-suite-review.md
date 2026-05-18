# 2026-05-03 Suite Reviews

## Review 34 — 2026-05-03 23:30Z

**Scope:** Apply the now-Open gaps G-90 (VSDD phase numbering inconsistency) and G-94 (smaller naming/location bundle, sub-issues 1, 2, 3, 5) following the decoupling decision in [Review 33](#review-33--2026-05-03-2300z). Pre-application step for G-90: fetched the upstream VSDD whitepaper (https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00) and verified phase-labelling convention. Files touched: `README.md` (pipeline table, `## Suite scope` references, supplement table, governance pointers), all forward-facing references to `lang/`, `DOMAIN-INDEX.md`, `SUITE-REVIEW.md`, `Phase 1` (VSDD pipeline meaning only). Triggered by user direction: "Do the actionable ones now."

**Lens:** Two named defect-class lenses applied serially — (1) **Upstream-tracking compliance** for G-90 (does the suite's labelling match the canonical whitepaper after one is consulted?), (2) **Mechanical bundle execution with decoupled sub-issue evaluation** for G-94 (which sub-issues can be applied today; which legitimately defer further?). Sycophancy compensation: the whitepaper fetch produced an unexpected discovery — the whitepaper's sub-step semantics differ from the suite's Phase 1a/1b semantics. Rather than glossing over the divergence, this is registered as G-96.

**Session note:** Same session as Reviews 32 and 33 and the user's direction to act on G-90/G-94. Not cold. Sycophancy risk acknowledged. Compensation: the whitepaper-fetch step produced an artifact-level claim that is independently verifiable; G-90's resolution path is mechanical given the whitepaper evidence; G-94 sub-issue 4 (CHANGELOG release tagging) was honestly deferred when its trigger condition was determined not to exist yet, rather than forced through.

---

### Resolved

**Finding 1 — G-90 (VSDD phase numbering inconsistency) addressed via Option 1 (introduce `1a` for symmetry).**

Upstream-whitepaper verification confirmed that the canonical VSDD whitepaper uses **symmetric letter-based sub-step labelling**: Phase 1 has Steps 1a/1b/1c, Phase 2 has Steps 2a/2b/2c. The suite's pre-existing `Phase 1` (no letter) for spec crystallization plus `Phase 1b` for decomposition was therefore the asymmetric form against an upstream symmetric convention. Applied Option 1 from G-90: renamed the suite's spec-crystallization phase from `Phase 1` to `Phase 1a` across forward-facing artifacts. The apprentice-program "Phase 1, 2, 3, 4" tier numbering is a distinct system (per VDD-IAR Alignment's `## Program Phase Context` note) and was deliberately not changed.

Files touched (VSDD-pipeline meaning only): `README.md` (pipeline table column, "spec issues to Phase 1" → "Phase 1a", "during Phase 1" → "during Phase 1a"), `prompts/spec-crystallization.md` (H1 parenthetical, in-prompt phase reference, governing-standard reference), `prompts/suite-development.md` (suite-history paragraph), `domains/role/SOLUTION-OWNER-REVIEW.md` (sycophancy-check VSDD-context reference), `domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md` (dim 1 spec-criteria reference, Program Phase Context note pipeline-list).

**Resolution:** G-90 marked Addressed in the registry. Forward-only constraint preserved: existing project review logs that reference `VSDD Phase 1` remain semantically coherent because Phase 1 still exists; only the label refines.

---

**Finding 2 — G-94 sub-issues 1, 2, 3, 5 resolved; sub-issue 4 deferred with rationale.**

- **Sub-issue 1 (`lang/` folder rename):** `lang/` directory renamed to `supplements/` via `git mv`. Folder contents unchanged (`browser-app.md`, `cli.md`, `javascript-typescript.md`, `rust.md`). Bulk path-reference update applied across all `*.md` files. Forward-facing terminology in `prompts/suite-development.md` updated: section heading `## Lang supplement coverage` → `## Supplement coverage`; suite-structure table row `Lang supplements | supplements/` → `Language and interface supplements | supplements/`; "Lang supplement reference" → "Language and interface supplement reference"; "lang supplement sections" → "language and interface supplement sections". Historical narrative in older review-log files now reads `supplements/` instead of `lang/` as a side effect of the bulk rename — accepted as the cost of mechanical path normalization. Some prior-session narrative was repaired post-bulk-update where the sed produced nonsensical strings (e.g., `lang/`→`supplements/` arrows that became `supplements/`→`supplements/`).
- **Sub-issue 2 (`DOMAIN-INDEX.md` move):** `domains/role/DOMAIN-INDEX.md` moved to `domains/DOMAIN-INDEX.md` via `git mv`. Internal references inside the file updated: README link `../../README.md` → `../README.md`; meta-domain backticked paths `../meta/` → `meta/`; role-domain backticked paths gained the `role/` prefix (e.g., `SOFTWARE-ENGINEER-REVIEW.md` → `role/SOFTWARE-ENGINEER-REVIEW.md`) for symmetry with the meta entries' `meta/` prefix. External references in `README.md` (two occurrences) and `prompts/review-session.md` updated.
- **Sub-issue 3 (`SUITE-REVIEW.md` rename):** Renamed to `SUITE-REVIEW-INDEX.md` via `git mv` (top-level retained, not relocated to `review-log/INDEX.md`). Forward-facing references updated in `README.md`, `prompts/suite-development.md` (multiple occurrences via `replace_all`), `prompts/review-session.md`, `GAP-ANALYSIS-LOG.md` (current-spec lines, not historical gap descriptions). Historical references in CHANGELOG entries and prior-session narratives describing actions taken when the file was named `SUITE-REVIEW.md` are preserved as historical record.
- **Sub-issue 4 (CHANGELOG release tagging) — deferred:** The proposed alternatives are (a) tag the next milestone as `1.0.0` (e.g., spinoff-MVP), or (b) rename the framing from `## Unreleased — DATE` to `## Session N — DATE`. Neither is actionable today: (a) requires a real release event, which does not exist yet — the suite is still in active flux; the natural release is the spinoff-MVP, which is the same trigger that bundles G-88/G-89/G-91/G-92/G-93/G-95 (and is Deferred). (b) would require renumbering 30+ historical entries, which conflicts with "do not silently amend prior findings." The honest resolution is to defer this sub-issue to the spinoff-MVP boundary alongside the other Deferred restructure work. Registered as a partial-Addressed status on G-94 with sub-issue 4 explicitly deferred.
- **Sub-issue 5 (suite-development primer H1 convention):** `prompts/suite-development.md` H1 updated from `# Session Primer: Suite Development` to `# Session Primer: Suite Development (Meta — Suite Contributors)` — the parenthetical now signals this is a meta-primer (not a VSDD phase primer), parallel to other primers' `(VSDD Phase N)` parentheticals.

**Resolution:** G-94 marked `Addressed (partial — sub-issue 4 deferred to spinoff-MVP)` in the registry with the description rewritten to record what was done.

---

### New gap registered

**G-96 — Suite's VSDD sub-phase semantics diverge from the upstream whitepaper.**

Discovered during the G-90 whitepaper-verification step. The whitepaper organizes:
- **Phase 1 — Spec Crystallization** with sub-steps **Step 1a** (Behavioral Specification), **Step 1b** (Verification Architecture), **Step 1c** (Spec Review Gate). All three sub-steps stay within Phase 1.
- **Phase 2 — Test-First Implementation** with sub-steps **Step 2a** (Test Suite Generation), **Step 2b** (Minimal Implementation), **Step 2c** (Refactor). All three sub-steps stay within Phase 2.

The suite, post-G-90 fix, uses:
- **Phase 1a** — Spec Crystallization
- **Phase 1b** — Decomposition (project broken into layered TODO.md, Red Gate test plans written per layer)
- **Phase 2a** — Red Gate
- **Phase 2b** — Implementation
- (no Phase 1c, no Phase 2c equivalents)

The suite's `Phase 1b` is **decomposition** — a different activity than the whitepaper's `Step 1b` (Verification Architecture). The suite's `Phase 2a` (Red Gate, all tests written and failing) maps to the whitepaper's `Step 2a` (Test Suite Generation), which is a closer alignment but the framing differs. The suite has no equivalent of the whitepaper's `Step 1c` (Spec Review Gate) — though IAR's review-session.md primer arguably plays this role at Phase 3. The suite has no equivalent of `Step 2c` (Refactor).

This is a real divergence, not a labelling fix. The G-90 resolution (introduce `1a`) made the suite's labels symmetric; it did not align the suite's sub-phase semantics with the whitepaper. Registered as G-96 (Open) for future evaluation. Two resolution paths exist: (a) harmonize the suite's sub-phase taxonomy with the whitepaper (rename or reposition decomposition; introduce a Refactor primer); or (b) document the deliberate divergence in `README.md` `## VSDD pipeline context` with rationale (e.g., decomposition is a distinct activity in this suite's experience, and Refactor is implicitly part of Phase 2b).

**Classification:** Documented as G-96 in [GAP-ANALYSIS-LOG.md](../GAP-ANALYSIS-LOG.md). Forward-only — existing project review logs that reference Phase 1a/1b under the suite's convention remain valid records of how the suite was applied at the time. Forward harmonization, if chosen, would apply to projects starting after the decision.

---

### Coordination

This review's actions complete G-90 entirely and G-94 partially (sub-issue 4 defers naturally to the spinoff-MVP boundary, joining the G-88/G-89/G-91/G-92/G-93/G-95 cluster). G-96 is newly Open and independent — its resolution does not bundle with the spinoff-restructure pass and may be acted on at any cadence after evaluation.

---



## Review 33 — 2026-05-03 23:00Z

**Scope:** Per-gap dependency analysis of the eight gaps deferred to "after `issue-tracker-cli` completes" (G-88, G-89, G-90, G-91, G-92, G-93, G-94, G-95) under the decoupling path enabled by Review 32 Finding 2. Read: `GAP-ANALYSIS-LOG.md` (registry rows for the eight gaps; the new `## Reactivation triggers` subsection), [Review 31](#review-31--2026-05-03-1800z) (original registration narrative), [Review 30](#review-30--2026-05-03-1200z) (G-88, G-89 registration context), the relevant suite artifacts referenced by each gap (`README.md` `## VSDD pipeline context`, `prompts/implementation.md`, primer file paths and cross-references, `supplements/` folder layout, `domains/role/DOMAIN-INDEX.md` location, `SUITE-REVIEW.md` filename and lead paragraph, `CHANGELOG.md` `## Unreleased` framing). Triggered by user direction on Review 32 Finding 5 (the bundle-deferral antipattern raised in chat but not registered as a Review 32 finding) — Option 1 selected: decouple now and act on the independent ones.

**Lens:** Single defect-class lens — **Bundled-deferral dependency analysis**. For each of the eight gaps, evaluate: does the gap's substance actually depend on `issue-tracker-cli`-derived feedback (real-project pressure, reading patterns, layer-merge experience), or was it bundled to the trigger purely for restructure-coordination convenience? Promote those whose substance does not depend on the trigger; keep deferred those whose substance does.

**Session note:** Same session as Review 32 and the user's selection of Option 1. Not cold. Sycophancy compensation: each per-gap decision is reduced to a structural claim — what artifact does the gap touch, what references that artifact in completed/in-flight projects, and would a forward-only edit applied today break those references? When the answer is "no broken references and no real-project-feedback dependency," the gap is decoupled. When either condition fails, the gap remains Deferred. The decision is not "what feels right to coordinate" but "what does the forward-only constraint actually require."

---

### Resolved

**Finding 1 — G-90 (VSDD phase numbering inconsistency) and G-94 (smaller naming/location bundle) promoted from Deferred to Open.**

Both gaps' substance depends on inputs other than `issue-tracker-cli`-derived feedback:

- **G-90** depends on the upstream VSDD whitepaper (the pre-application step is "verify upstream VSDD whitepaper labelling"). The whitepaper is independent of `issue-tracker-cli`'s lifecycle. Once the whitepaper is checked, the resolution path is mechanical (Option 1 — symmetrize by introducing `1a`, or document upstream-tracking). No completed project's review log uses `Phase 1a` such that an introduction would break references. No in-flight project's DESIGN.md references the phase label in a way that breaks under renaming. The forward-only constraint is satisfied trivially: existing logs that reference `VSDD Phase 1` remain semantically coherent because Phase 1 still exists; only the label refines.

- **G-94** is a bundle of five mechanical or low-coordination sub-issues (`lang/` → `supplements/` folder rename, `DOMAIN-INDEX.md` move from `domains/role/` to `domains/`, `SUITE-REVIEW.md` rename to `SUITE-REVIEW-INDEX.md`, CHANGELOG release tagging, primer H1 convention). None of the five depend on real-project pressure or reading patterns. The folder rename is the most coordination-heavy sub-issue (every domain prompt file references the supplements folder) but is still mechanical — a single sed-style update across the relevant domain files. The remaining four are smaller. The bundling to the spinoff trigger was a coordination convenience, not a dependency.

**Resolution:** Both registry rows updated in place — Status `Deferred` → `Open`, descriptions amended to remove the `issue-tracker-cli` trigger language and link to this Review 33 entry as the decoupling record. Forward-only constraint preserved: any edits made under these gaps apply to projects whose first IAR run is after the edit, with the caveat that for G-90 (Phase 1 label) and G-94 sub-issues that touch only suite-internal artifacts, "forward-only" is largely a non-constraint because no completed project's review log materially depends on the suite's own filenames (it depends on the project's own filenames under `iterative-adversarial-refinement/`).

---

**Finding 2 — G-88, G-89, G-91, G-92, G-93, G-95 reviewed and remain Deferred.**

Six gaps' substance genuinely depends on inputs the `issue-tracker-cli` trigger is structured to provide:

- **G-88** (suite directory rename) — explicitly registered as needing post-`issue-tracker-cli` evidence: "by that point the suite will have absorbed two additional rounds of real-project pressure and the question can be evaluated against fresh evidence rather than against the speculative naming-confusion failure mode."
- **G-89** (project-level review log structure standardization) — explicitly registered as needing real reading patterns: "by that point the suite has accumulated enough project-level review log experience that the right index/narrative split is informed by real reading patterns rather than speculation about scaling."
- **G-91** (`prompts/` folder rename, primer file naming) — primer file paths are referenced by completed and in-flight project review logs (DESIGN.md, PROCESS.md, IAR-domain-log preambles). A rename today breaks live references. Forward-only requires waiting until in-flight projects are complete and not adding new references during the wait window — i.e., the trigger condition.
- **G-92** (suite-meta vs suite-running directory restructure) — moves multiple files (`SUITE-REVIEW.md`, `GAP-ANALYSIS-LOG.md`, `review-log/`, `prompts/suite-development.md`) into a `suite-development/` subfolder. The relative-path web between these artifacts is dense; the restructure naturally bundles. Acting independently produces churn (each move requires updating cross-references) without producing meaningful intermediate value.
- **G-93** (user vs contributor README split) — coordinated with G-92 (the contributor-facing README lives in `suite-development/`). Decoupling G-93 from G-92 produces a half-restructured state.
- **G-95** (`prompts/implementation.md` split into Phase 2a / Phase 2b primers) — coordinated with G-91 (primer file naming convention). The new files would either keep the unsignalled `implementation.md` naming (preserving G-91's defect class) or adopt phase-prefixed naming (pre-empting G-91's resolution). Both options worsen if G-91 is still Deferred.

**Resolution:** No registry edit required — these rows remain Deferred with their existing descriptions and trigger language. The decoupling analysis itself is the resolution; future contributors looking at these rows next month should see the same trigger they see today, because the analysis confirms the trigger is appropriate for these six.

---

### Coordination

The bundled-deferral cluster is now smaller and more honestly scoped. G-88, G-89, G-91, G-92, G-93, G-95 remain coordinated as the spinoff-time restructure pass; G-90 and G-94 are independent and may be acted on at any cadence. The `## Reactivation triggers` subsection in `GAP-ANALYSIS-LOG.md` (added in [Review 32](#review-32--2026-05-03-2200z)) is the operational definition for the bundled trigger; the explicit decoupling-permission paragraph in that subsection is the standard this Review 33 applies.

---



## Review 32 — 2026-05-03 22:00Z

**Scope:** Adversarial review of suite-review entry-format and deferral discipline using Review 31's own output as a representative artifact. Read: `prompts/suite-development.md` (`## Suite review and review-log discipline` and the entry-format subsection), `prompts/review-session.md`, `GAP-ANALYSIS-LOG.md` (registry, with attention to G-88–G-95 deferral pattern), `SUITE-REVIEW.md` (Reviews 30 and 31 rows), `review-log/2026-05-03-suite-review.md` Reviews 30 and 31, `README.md` `## Suite scope` and pipeline-context table, three sample domain files (`domains/role/QUALITY-ENGINEER-REVIEW.md`, `domains/role/SECURITY-REVIEW.md`, `domains/role/SOLUTION-OWNER-REVIEW.md`) for sycophancy-check fidelity. Triggered by user request following Review 31; the user explicitly asked for a fresh adversarial pass that does not re-litigate Review 31's deferred findings.

**Lens:** Four lenses applied in parallel — (1) **Suite-review entry-format compliance** (does the most recent entry conform to its own governing standard?), (2) **Deferral trigger falsifiability** (is the bundled-trigger phrase operationally defined?), (3) **Standard-vs-practice grammar** (does the standard's enumeration of valid `Lens` forms cover what reviews actually use?), (4) **Session-isolation policy completeness** (does the suite have a documented standard for cold vs. in-session suite reviews, parallel to the domain-review standard?). Findings F1–F4 are the resolved set.

**Session note:** Same session as Review 31's adversarial pass and the user's directive to apply F1–F4. Not a cold isolated session. Sycophancy risk acknowledged — these findings emerged from analysis the user prompted. Compensation: each finding identifies a concrete artifact location (line number or section reference) where the standard and practice diverge, so the finding is reducible to a verifiable structural claim rather than narrative judgment. Two hallucinated findings are recorded to demonstrate push-back against the symmetry instinct.

---

### Resolved

**Finding 1 — `### Coordination` heading is undeclared in the suite-review entry-format standard. (Suite-review entry-format compliance lens)**

Review 31 closes with a `### Coordination` section at lines 201–203 of `review-log/2026-05-03-suite-review.md`, naming how G-88–G-95 cluster into a single bundled restructure pass. Review 30 (same file, earlier in the day) does not have one. The governing standard in `prompts/suite-development.md` `## Suite review entry format` item 4 enumerates exactly four valid heading forms (`### Resolved`, `### Dismissed`, `### Hallucinated`, `### New gap registered`); item 6 (Closing) explicitly says "no separate Summary required" and lists no other valid sections. The `### Coordination` section in Review 31 is therefore a heading the standard does not authorize.

The cluster-coordination paragraph is real signal — naming how multiple findings bundle into a single deferred action is exactly the kind of cross-finding reasoning a future reviewer would want to see. But the standard either authorizes the heading or it does not. Letting Review 31 remain non-conformant invites future entries to invent their own closing sections.

**Resolution:** Updated `prompts/suite-development.md` `### Suite review entry format` item 6 (Closing) to permit an optional `### Coordination` section after the classification sections, used to name a cross-finding cluster and bundled action. Cross-references in the Coordination section use Markdown links. Review 31's `### Coordination` is thereby retroactively conformant; the standard now reflects the practice rather than contradicting it.

---

**Finding 2 — "After `issue-tracker-cli` completes" is an underspecified reactivation trigger across eight registry rows. (Deferral trigger falsifiability lens)**

G-88, G-89, G-90, G-91, G-92, G-93, G-94, and G-95 all carry the same reactivation phrase: "after `issue-tracker-cli` completes." Neither `GAP-ANALYSIS-LOG.md`, nor `prompts/suite-development.md`, nor any session entry defines what "completes" means: final layer merged? all IAR domains MVR? spinoff-to-standalone-repo executed? archived per portfolio convention? A future contributor reading the registry next month either acts prematurely on a fuzzy signal or defers indefinitely. This is the SO domain's own dim 4 + dim 9 failure class (a deferral whose trigger is not falsifiable cannot be honored) applied to the suite's own deferrals — the suite is failing its own Solution Owner standard on its own work.

**Resolution:** Added `## Reactivation triggers` subsection to `GAP-ANALYSIS-LOG.md` between `## How to run a gap analysis` and `## Gap Registry`. Defines the bundled trigger with three required conditions: (a) all active layers merged, (b) final-merge VDD-IAR Alignment review classified, (c) project archived per portfolio convention. Names the abandonment/pivot path explicitly (gaps re-evaluated individually if the trigger never fires). Permits gaps whose substance does not actually depend on `issue-tracker-cli`-derived feedback (e.g., upstream-whitepaper checks, mechanical renames) to be promoted from Deferred to Open at any time without waiting on the bundled trigger; the decoupling is recorded in `CHANGELOG.md` and the registry row is updated in place. The bundle-vs-decouple decision for individual G-88–G-95 rows is left for separate user direction.

---

**Finding 3 — `Lens` field grammar is incomplete; recent reviews use forms not enumerated in the standard. (Standard-vs-practice grammar lens)**

`prompts/suite-development.md` `### Suite review entry format` item 3 names two valid lens forms: a named defect class OR a registry-walk scope. Review 30's Lens field uses a third form — domain-role lenses ("Solution Owner + Technical Writer + VDD-IAR Alignment lenses"). Review 31's Lens uses a fourth form — a named bundle of complementary defect-class lenses applied serially ("five lenses applied serially — clarity, naming, ambiguity, consistency, transitional-state alignment"). Both are intuitive and useful; neither is in the standard. The standard is now stricter than practice.

**Resolution:** Updated item 3 to enumerate three valid forms: named defect class, registry-walk scope, and role-based lens (with the role-based form covering both the domain-perspective variant and the named-bundle variant). The "diffuse lens produces a diffuse review" guidance is preserved. Review 30's and Review 31's Lens fields are thereby retroactively conformant.

---

**Finding 4 — Suite-review session-isolation policy is unstated. (Session-isolation policy completeness lens)**

`prompts/review-session.md` is explicit that domain reviews benefit from cold isolated sessions and that batching is a quality tradeoff requiring explicit acknowledgement. `prompts/suite-development.md` is silent on the equivalent question for suite reviews — whether they must be cold, may be in-session, or have a different standard altogether. Review 31's `**Session note:**` ("Same session as the user's pre-flight discussion; not a cold isolated session") acknowledges sycophancy risk inline; Review 30's session note also addresses it. This pattern is consistently applied but undocumented. A future contributor running a suite review with no Session note (or with a generic "cold session" boilerplate that does not name compensation) has no standard to be held to.

**Resolution:** Added `### Session isolation` subsection to `prompts/suite-development.md` `## Suite review and review-log discipline`, between the entry-format subsection and `### Common discipline`. Documents that suite reviews are typically in-session (unlike domain reviews where cold is the gold standard); cold-session is permitted and stronger but not required; the minimum standard is an explicit session note that names cold-vs-in-session status and, if in-session, names a compensation. A missing session note is itself a finding for VDD-IAR Alignment dim 7 applied to the suite.

---

### Hallucinated

**Finding 5 — Suite directory should be renamed to reflect the broader scope.**

A strict reading of "the directory name no longer matches the artifact set" could push for a rename. This concern was already raised in Review 30 (Finding 5) and classified hallucinated, then registered as G-88 (Deferred) for revisit after `issue-tracker-cli` completes. Re-raising it in Review 32 without new evidence would be re-litigating dismissed material — exactly what `prompts/review-session.md` warns against ("Do raise findings dismissed without adequate rationale" — but the rationale here is documented and adequate). The rename question is not closed; it is properly deferred. **Hallucinated** as a current-session finding.

---

**Finding 6 — Each suite-review entry should carry a domain-specific sycophancy check section parallel to the per-domain prompt files.**

Domain reviews carry per-entry sycophancy checks because each domain has a domain-specific failure mode (G-77 corrected the boilerplate-across-domains version of this). Suite-review entries are not per-domain — they are session-level. The session-level posture is already established by `prompts/suite-development.md` "Apply the same adversarial standard…" and now by the `### Session isolation` subsection (Finding 4). Adding a per-entry sycophancy check section to every suite-review entry would reintroduce the boilerplate problem G-77 corrected — there is no per-entry domain-specific failure mode to name. The Session note line, when used to acknowledge in-session sycophancy risk explicitly, already serves this purpose. **Hallucinated** as a defect; the structural absence is intentional.

---

### Coordination

This review's resolved findings (F1–F4) are internal-suite consistency edits and do not bundle with the G-88–G-95 deferral cluster. They are applied in-session per the user's directive. F5 (the bundle-deferral antipattern raised in the prior session) was carried out for separate user decision and is not addressed in this entry.

---



## Review 31 — 2026-05-03 18:00Z

**Scope:** Full adversarial review of the suite under five lenses — clarity, naming, ambiguity, consistency, and alignment with the transitional state of suite scope. Read: `README.md`, `SUITE-REVIEW.md`, `GAP-ANALYSIS-LOG.md`, `CHANGELOG.md`, all five primer files (`prompts/spec-crystallization.md`, `prompts/decomposition.md`, `prompts/implementation.md`, `prompts/review-session.md`, `prompts/suite-development.md`), `domains/role/DOMAIN-INDEX.md`, all 14 role domain files, both meta domain files, all four `supplements/` supplements, today's prior session entry (Review 30). Triggered by user request: review through the lens of a third-party reader landing on this directory cold, with the additional context that this suite will be spun off into its own repository once it reaches stable MVP.

**Lens:** Five lenses applied serially — (1) **Clarity** (would a reader who has never seen this directory follow the docs from the README forward without prior context?), (2) **Naming** (do file, folder, and section names predict their content for a fresh reader, and are conventions applied consistently?), (3) **Ambiguity** (where are two readers likely to disagree on what an instruction or label means?), (4) **Consistency** (where do two artifacts disagree on a fact, label, or convention?), (5) **Transitional-state alignment** (does the documentation reflect the suite's actual current scope as a VSDD-pipeline support library, including its trajectory toward standalone-repo spinoff and its center of gravity remaining adversarial review?). The user's pre-flight notes named four candidate findings (phase numbering inconsistency, primer folder/file naming, suite-meta vs. suite-use separation, user vs. contributor delineation); this review evaluates those and surfaces additional findings against the same lenses.

**Session note:** Same session as the user's pre-flight discussion; not a cold isolated session. Sycophancy risk acknowledged — the four user-named candidates are pre-validated, which biases toward registering them. Compensation: each user-named candidate is evaluated against the lens criteria independently, and additional findings are surfaced through unprompted artifact analysis (folder layout, file headers, table contents, naming patterns) so the review is not reducible to "agree with what the user said."

**Application discipline:** Per user instruction, all findings are **deferred** for application until after `issue-tracker-cli` completes. Forward-only: any structural change applies to projects whose first IAR run begins after the change is recorded; completed projects retain their existing review-log paths, primer references, and folder layout. This matches the precedent set by G-88 and G-89 in Review 30. Findings are registered as gaps so they are not lost during the deferral window.

---

### New gap registered

**G-90 — VSDD phase numbering is internally inconsistent: Phase 1 is the only sub-phase of Phase 1 not lettered, while Phase 2 is uniformly split into 2a/2b. (Clarity + Consistency lens)**

The README's `## VSDD pipeline context` table lists phases as `1`, `1b`, `2a`, `2b`, `3`, `4`, `5`, `6`. The implication is that Phase 1 has two sub-phases (`1` for spec crystallization and `1b` for decomposition), Phase 2 has two sub-phases (`2a` for Red Gate and `2b` for implementation), and Phases 3–6 are atomic. But the labelling is asymmetric: Phase 1's first sub-phase has no letter; Phase 2's first sub-phase does. A reader scanning the table cannot tell whether `1` is the parent or the first sub-phase, and the suite-development primer's line 11 reproduces the same inconsistency (`Phase 1 spec crystallization, Phase 1b decomposition, Phase 2a–2b implementation`). The `prompts/suite-development.md` line 11 phrasing, the README table, the VDD-IAR Alignment Program Phase Context section's parenthetical (which carefully distinguishes apprentice phases from `1, 1b, 2a, 2b, 3, 4, 5, 6`), and the primer H1 titles all carry this asymmetry forward.

A fresh reader will reasonably ask: "If decomposition is `1b`, what is `1a`?" The current answer is "`1` is `1a`." That answer is not derivable from the documentation; it requires telling the reader.

**Resolution options (defer choice to apply-time):**

1. **Symmetrize by introducing `1a`** — Rename Phase 1 (spec crystallization) to `Phase 1a`. Update README pipeline table, primer H1 (`# Session Primer: Spec Crystallization (VSDD Phase 1a)`), `prompts/suite-development.md` line 11, VDD-IAR Alignment phase parenthetical, all CHANGELOG and SUITE-REVIEW lead-paragraph references. Keep `1b`, `2a`, `2b`, `3`, `4`, `5`, `6` unchanged.
2. **De-letter Phase 2** — Rename `2a` → `2`, `2b` → `2.5` or split into separate top-level phases `Red Gate` and `Implementation`. Higher blast radius; touches more artifacts; risks divergence from the VSDD whitepaper.

**Pre-application step:** Verify the upstream VSDD whitepaper's phase labelling. The whitepaper is the canonical reference (`README.md` cites it as primary). If the whitepaper uses `Phase 1` (no letter) for spec crystallization and `Phase 1b` for decomposition, the suite is faithful to the whitepaper and the asymmetry is upstream; in that case, the suite either tracks the whitepaper exactly (close G-90 with a documentation note explaining the asymmetry comes from upstream) or normalizes internally (Option 1) and documents the deviation. If the whitepaper uses `1a/1b/2a/2b` symmetrically, Option 1 is the obvious fix.

**Why:** Asymmetric numbering forces every reader to learn the suite's labelling convention before reading. A primer file titled `(VSDD Phase 1a)` and another titled `(VSDD Phase 1b)` is self-explanatory; `(VSDD Phase 1)` followed by `(VSDD Phase 1b)` requires a reader to learn that `Phase 1` is the first half of itself.

**How to apply:** Defer to post-`issue-tracker-cli`. Forward-only; existing project review logs that reference `VSDD Phase 1` remain semantically coherent (Phase 1 still exists; only the label shifts). At application time: verify the whitepaper, choose Option 1 or document upstream-tracking, then perform a single sed-style update across the named files plus a CHANGELOG entry.

Status: Deferred. Trigger: post-`issue-tracker-cli` completion.

---

**G-91 — `prompts/` folder name and primer file names do not signal which VSDD phase each primer is for. (Clarity + Naming lens; reinforced by spinoff context)**

The folder `prompts/` contains five files: `spec-crystallization.md`, `decomposition.md`, `implementation.md`, `review-session.md`, `suite-development.md`. A fresh reader scanning `ls prompts/` cannot tell which file goes with which VSDD phase without opening each one. The README's `## Session primers` table is the only navigation aid, and a reader who lands in the folder via `ls`, a directory listing on GitHub, or a search-result link bypasses that table.

Two naming concerns compound:

1. **Folder name `prompts/`** is a misnomer. The README and every file inside the folder consistently calls these "session primers," not "prompts." The actual review prompts live in `domains/role/*-REVIEW.md` (each contains a `## Current Review Prompt` section). A reader who interprets `prompts/` literally will look for review prompts there and find primers instead. The five files are not "the prompts" — they are the framing primers loaded before "the prompts."

2. **File names lack VSDD phase prefix.** A reader cannot scan the folder and answer "which file do I open if I'm starting Phase 2a?" without opening each file's H1. A phase-prefixed name (`phase-2a-implementation.md` or `phase-2-implementation.md`) is self-documenting in `ls` output, GitHub directory listings, and IDE file trees.

**Resolution options (defer choice to apply-time):**

1. **Rename folder `prompts/` → `primers/`** to match the consistent in-document terminology. Update all relative-path references (README, all primer cross-references, `prompts/suite-development.md` self-reference, MEMORY.md if it points there, `prompts/review-session.md` line referencing `prompts/suite-development.md`).
2. **Add VSDD phase prefix to primer file names** — `phase-1a-spec-crystallization.md`, `phase-1b-decomposition.md`, `phase-2a-2b-implementation.md` (or split as 2a / 2b — see G-95 below), `phase-3-adversarial-review.md`. The suite-development primer is not a VSDD phase primer (see G-92) and gets a different naming convention.
3. **Combine 1 and 2** — `primers/phase-1a-spec-crystallization.md`, etc.

**Why:** A standalone-repo spinoff means the folder will be browsed via GitHub's directory view by readers who arrived via search or external link, with no project README context. File names are the first signal a reader gets. A primer named `implementation.md` could be for any phase of any methodology; `phase-2a-2b-implementation.md` is unambiguous.

**How to apply:** Defer to post-`issue-tracker-cli`. Forward-only — existing project review logs that link to `prompts/review-session.md` (typically inside DESIGN.md or PROCESS.md) remain valid until those projects are themselves migrated, which they are not. At application time: rename in a single commit; update all cross-references; CHANGELOG entry; add a footnote in SUITE-REVIEW.md or CHANGELOG noting the rename so future readers can resolve historical references.

Status: Deferred. Trigger: post-`issue-tracker-cli` completion. Coordinated with G-92 (the suite-development primer relocation), G-93 (folder restructure for user vs. contributor), and G-95 (Phase 2a/2b primer split question).

---

**G-92 — Suite-meta-development materials are interleaved with suite-running materials at every level of the directory. (Transitional-state + Clarity lens; sharpened by spinoff context)**

The current top-level layout mixes two audiences:

| Artifact | Audience |
|---|---|
| `README.md` | Both — opens with how to use the suite, ends with how to extend it |
| `domains/` | Suite users (apply to their projects) |
| `supplements/` | Suite users |
| `prompts/spec-crystallization.md`, `decomposition.md`, `implementation.md`, `review-session.md` | Suite users |
| `prompts/suite-development.md` | Suite contributors only |
| `SUITE-REVIEW.md` | Suite contributors only |
| `GAP-ANALYSIS-LOG.md` | Suite contributors only |
| `review-log/` | Suite contributors only |
| `CHANGELOG.md` | Both — but biased toward contributors |

The user's `prompts/` folder contains five files; one of them (`suite-development.md`) is for a fundamentally different audience and use case than the other four. A user opening `prompts/` to find their phase primer will encounter the suite-development primer with no signal that it does not apply to them. This is a category error in the folder.

The same pattern repeats at the directory root: `SUITE-REVIEW.md`, `GAP-ANALYSIS-LOG.md`, and `review-log/` are governance artifacts for contributors. A user evaluating the suite for their project must scroll past or learn-to-ignore four governance files to find the user-facing pieces (`domains/`, `supplements/`, the four phase primers).

**Resolution sketch (defer detail to apply-time):**

Move all suite-meta artifacts into a single subfolder, e.g. `suite-development/` (or `contributing/`, or `meta/` — exact name decided at apply-time). Proposed target layout:

```
iterative-adversarial-refinement/      (or new repo root after spinoff)
├── README.md                          (user-facing — how to use)
├── CHANGELOG.md
├── domains/
│   ├── role/
│   ├── meta/
│   └── DOMAIN-INDEX.md                (move up from role/ — see G-94)
├── supplements/                              (or supplements/ — see G-94)
├── primers/                           (renamed from prompts/ — see G-91)
│   ├── phase-1a-spec-crystallization.md
│   ├── phase-1b-decomposition.md
│   ├── phase-2a-2b-implementation.md
│   └── phase-3-adversarial-review.md
└── suite-development/
    ├── README.md                      (contributor-facing — how to extend)
    ├── SUITE-DEVELOPMENT-PRIMER.md   (renamed from prompts/suite-development.md)
    ├── SUITE-REVIEW.md
    ├── GAP-ANALYSIS-LOG.md
    └── review-log/
```

**Why:** A user landing on the README of the standalone-repo spinoff will scroll the top-level directory listing as part of evaluation. A clean top-level (only user-facing artifacts visible) signals "this is a tool you can adopt" rather than "this is a process artifact you must understand the governance of." Contributors who want to develop the suite navigate explicitly to `suite-development/` and find everything they need together — including the contributor-facing README, the primer for suite-development sessions, the gap registry, the suite-review index, and the session logs.

**How to apply:** Defer to post-`issue-tracker-cli`. Forward-only. At application time, this is a structural change — coordinate with G-91, G-93, G-94 for a single bundled restructure commit. Existing project review logs that reference `iterative-adversarial-refinement/SUITE-REVIEW.md` etc. remain valid until those projects are themselves migrated (which is out of scope per the forward-only rule). Add a CHANGELOG entry mapping old paths to new paths so external links can be resolved.

Status: Deferred. Trigger: post-`issue-tracker-cli` completion. Coordinated with G-91, G-93, G-94, and the open G-88 (directory-rename question) — all of these structural concerns naturally bundle into a single restructure pass at the spinoff boundary.

---

**G-93 — No clear delineation between "user of the suite" and "contributor to the suite" in entry-point documentation. (Clarity + Transitional-state lens; sharpened by spinoff context)**

The single `README.md` serves both audiences and explicitly bridges them: `## Suite scope` and `## VSDD pipeline context` are oriented to a user evaluating the suite; `## Suggesting new domains`, the cross-references to `SUITE-REVIEW.md` and `GAP-ANALYSIS-LOG.md`, the registry-walk discussion, and the `review-log/` filename convention are oriented to a contributor. A user reading top-to-bottom will encounter contributor-facing content (gap registry, review-log discipline, suite-review entry format) before reaching the practical "Running IAR" section. A contributor reading top-to-bottom will navigate past user-facing content (domain tables, primer table) to reach contributor concerns scattered across the latter half of the file.

**Resolution sketch (defer detail to apply-time):**

Two-README approach (paired with G-92's folder restructure):

- **`README.md`** — User-facing only. Sections: what IAR/the suite is; what it covers (VSDD pipeline table); the domain catalog (core / extended / meta tables); the primer catalog; the supplements/interface supplement catalog; how to run a review; how to wire the suite into your project; merging gate criteria; pointer to `suite-development/README.md` for contributors.
- **`suite-development/README.md`** — Contributor-facing only. Sections: how the suite is structured (artifact map); how to add or modify a domain; how to add or modify a primer; how to add or modify a lang supplement; suite review and gap-registry discipline; the governing standards currently in `prompts/suite-development.md` `## Governing standard for ...` sections; pointers to `SUITE-REVIEW.md`, `GAP-ANALYSIS-LOG.md`, `CHANGELOG.md`, `review-log/`.

Concretely, the existing `prompts/suite-development.md` is not just a session primer — it doubles as the suite's contributor governance documentation (governing standards for domain files, primers, project-level review logs; pre-change checklists; lang supplement coverage table). The contributor-facing concerns currently live partly in this primer and partly in README.md. Consolidation at apply-time should split the file into:

- A short `suite-development/SUITE-DEVELOPMENT-PRIMER.md` — the actual posture-setting primer text for a fresh AI session
- The longer governing-standard sections moved into `suite-development/README.md` or its own contributor handbook (e.g., `suite-development/GOVERNING-STANDARDS.md`)

**Why:** A spinoff repo's GitHub landing page is the user's first impression. A README that is half "how to use this" and half "how to govern this" signals "this is a process artifact, not a tool." The existing content does not need to be deleted — it needs to be relocated for audience separation.

**How to apply:** Defer to post-`issue-tracker-cli`. Bundled with G-91, G-92, G-94. Existing in-flight project READMEs (e.g. `issue-tracker-cli/README.md`) reference suite paths; those references would need to be checked for breakage during the spinoff's path-rewrite step but not retroactively rewritten in completed projects.

Status: Deferred. Trigger: post-`issue-tracker-cli` completion. Coordinated with G-91, G-92, G-94, G-88.

---

**G-94 — Several smaller naming and location issues that should bundle with the same restructure pass. (Naming + Consistency lens)**

These are individually minor but compound a fresh reader's onboarding cost. Bundling them with the larger restructure (G-91/G-92/G-93/G-88) avoids a second commit later.

1. **`supplements/` folder name conflates language and interface-type supplements.** The folder contains `rust.md`, `javascript-typescript.md` (languages) and `cli.md`, `browser-app.md` (interface types). The `prompts/suite-development.md` `## Lang supplement coverage` section calls all four "lang supplements" but the actual artifact set is broader. Proposed rename: `supplements/`, with optional sub-organization `supplements/language/{rust,javascript-typescript}.md` and `supplements/interface/{cli,browser-app}.md`. Alternatively, keep the flat layout but rename the folder to `supplements/` and update the section title.

2. **`DOMAIN-INDEX.md` lives in `domains/role/` but indexes both role and meta domains.** The file uses `../meta/...` paths for the two meta domains, which is structurally awkward — the index of all domains lives inside the role-specific subfolder. Move to `domains/DOMAIN-INDEX.md` (one level up); update the README cross-reference and the meta-domain links from `../meta/` to `meta/`.

3. **`SUITE-REVIEW.md` is named like a single review document but is actually the index of reviews in `review-log/`.** A fresh reader could reasonably expect this file to contain a review. Either rename to `SUITE-REVIEW-INDEX.md` (clearer) or move into `review-log/INDEX.md` (colocate with what it indexes). The reading-convention warning at the top mitigates the confusion but does not eliminate it.

4. **CHANGELOG `## Unreleased — DATE (description)` entries** with no actual release tag — the suite has accumulated 30+ "Unreleased" entries. The framing implies an eventual release event that has not occurred. Either start tagging releases at meaningful milestones (e.g. an initial `## 1.0.0 — DATE` for spinoff MVP) or rename the framing to `## Session N — DATE` to match the suite-review numbering. Aligned with the spinoff: a spinoff-MVP tag would be a natural first release.

5. **Primer file H1 titles are inconsistent in their VSDD-phase parenthetical convention.** Four primers have `(VSDD Phase N)` parentheticals; `prompts/suite-development.md` has none. The convention is correct (the suite-development primer is not a phase primer) but unsignalled. At apply-time, either explicitly mark the suite-development primer as a meta-primer in its H1 (`# Session Primer: Suite Development (Meta — Suite Contributors)`) or document the convention in the primer-files table.

**Why:** Each individually is a minor friction; together they accumulate into "the suite has a learnable but non-obvious local convention set." A standalone-repo audience is less tolerant of this than a project-internal audience.

**How to apply:** Defer to post-`issue-tracker-cli`. Bundle with G-91/G-92/G-93/G-88 in a single restructure pass.

Status: Deferred. Trigger: post-`issue-tracker-cli` completion.

---

**G-95 — `prompts/implementation.md` covers two distinct VSDD phases (2a and 2b) in one primer. (Clarity + Consistency lens)**

The implementation primer's H1 is `# Session Primer: Implementation (VSDD Phase 2a–2b)`. Inside, it has two sections: `## Phase 2a: Red Gate` and `## Phase 2b: Implementation`. Phase 2a is a distinct activity (write failing tests, commit Red Gate state) from Phase 2b (implement to pass tests). They are sequenced strictly — one finishes before the other begins, with a commit boundary between them per the Phase 2a step 4 added in Review 16.

A reader entering Phase 2a benefits from a primer that talks only about Red Gate discipline; a reader entering Phase 2b benefits from a primer that talks only about implementation discipline against an existing failing test suite. The current combined primer requires the reader to scroll past the phase they are not in. Worse, a reader who enters Phase 2b but accidentally re-reads Phase 2a's instructions could re-add tests during implementation (explicitly forbidden by Phase 2b step 2, but the proximity of the two phase sections in one file invites the confusion).

**Resolution options (defer choice to apply-time):**

1. **Split into two primer files** — `primers/phase-2a-red-gate.md` and `primers/phase-2b-implementation.md`. Each is self-contained. The README primer table gets two rows.
2. **Keep combined but mark phase boundary more strongly** — Add a `---` and a "before reading Phase 2b" gate at the top of the Phase 2b section (e.g. "Verify the Red Gate state from Phase 2a is committed before reading further. If not, return to Phase 2a.").

Option 1 is cleaner; Option 2 is lower-cost. The user's overall "name primers by phase" guidance (G-91) favors Option 1, since splitting also makes the file-name convention symmetric with the other phase primers.

**Why:** Phase 2a and Phase 2b are distinct enough to warrant distinct framing primers. The Red Gate commit boundary is a real phase boundary (Review 16 added a step explicitly because it wasn't being respected). One primer for two phases dilutes the framing for both.

**How to apply:** Defer to post-`issue-tracker-cli`. Bundled with G-91 (primer naming).

Status: Deferred. Trigger: post-`issue-tracker-cli` completion. Coordinated with G-91.

---

### Hallucinated

**Finding 1 — `SUITE-REVIEW.md` and `GAP-ANALYSIS-LOG.md` should be merged into a single contributor-status file.**

A symmetry argument could push for combining the suite-review index and the gap registry — both are contributor-facing status documents covering the same change history. But `prompts/suite-development.md` `## Suite review and review-log discipline` is explicit and well-justified about the three-way separation: the registry has only status (one row per gap, no narrative), the index has only pointers (one row per session), and the narrative lives in `review-log/`. Merging them would either bloat the registry with narrative or strip the index of its navigation purpose. The separation is load-bearing — a contributor reading the registry to triage open work needs status density; a contributor reading the index to find a session needs pointer density. **Hallucinated** as a defect — the separation is intentional, justified in the governing primer, and currently working.

---

**Finding 2 — Each primer should have a "completion criteria" section symmetric with the spec-crystallization primer.**

A symmetry argument could push for adding a `## Completion criteria` section to all primers — `spec-crystallization.md`, `decomposition.md`, and `implementation.md` already have one; `review-session.md` and `suite-development.md` do not. But:
- `review-session.md` is not a construction primer; it does not produce an artifact whose completeness can be checked against criteria. It establishes posture for a domain review, and the per-domain `## Standard Evaluation Dimensions` are the completion criteria. Adding a separate completion-criteria section would either duplicate the dimension list or invent generic criteria (e.g. "all findings classified") that already live in `## After each domain review`.
- `suite-development.md` is similarly a meta-primer. Suite-development sessions produce different artifacts depending on what changed (a new domain, a modified dimension, a new primer, a registry walk); a single completion-criteria section cannot cover all of them. The pre-change checklists in `## Before adding a dimension` / `## Before adding a domain` / `## Before modifying a domain` already serve this role.

The asymmetry is correct, not a defect. **Hallucinated.**

---

### Coordination

This entire review's findings cluster around a single underlying decision: at the `issue-tracker-cli` completion / spinoff-MVP boundary, perform one bundled structural pass that addresses G-88 (directory rename consideration), G-89 (project-level review log structure standardization), and G-90 through G-95 (the new findings from this review). Coordinating these into a single restructure commit avoids a sequence of incremental rewrites and produces a clean spinoff state. CHANGELOG should record the bundle as a single restructure milestone, with a path-mapping table covering all renames so external references can be resolved.

---



**Scope:** Adversarial review of the suite's documented scope and identity. Read: `README.md`, `prompts/suite-development.md`, `SUITE-REVIEW.md`, all five primer files (`spec-crystallization.md`, `decomposition.md`, `implementation.md`, `review-session.md`, `suite-development.md`), `domains/role/DOMAIN-INDEX.md`, `domains/role/SOLUTION-OWNER-REVIEW.md`, `domains/role/TECHNICAL-WRITER-REVIEW.md`, `domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md`, `CHANGELOG.md`, `GAP-ANALYSIS-LOG.md`. Triggered by user request: the suite started as IAR / VSDD Phase 3-aligned and has grown into a holistic VSDD prompt and process library; documentation should reflect the transitional state.

**Lens:** Three lenses applied in parallel — Solution Owner (does the suite "spec" still match the suite implementation?), Technical Writer (is the documentation accurate to current scope, and could a new reader navigate from the README alone?), VDD-IAR Alignment (cross-session spec consistency — has the suite's behavior shifted across sessions without a corresponding update to its spec?).

**Session note:** Same session as the documentation updates that resolve the findings. Sycophancy risk acknowledged; findings derive from artifact-state analysis (table contents, opening framing language, primer file headers, artifact lists in lead paragraphs) rather than narrative judgment.

---

### Resolved

**Finding 1 — README opening framing claims IAR-only scope; suite covers Phases 1, 1b, 2a–2b, 3, plus suite-development meta. (SO lens — scope drift)**

The README opens: "IAR fills the role of the adversary in the Verified Spec-Driven Development (VSDD) pipeline. VSDD structures software development as a six-phase cycle; IAR owns Phase 3 — Adversarial Refinement." But the directory contains primers for VSDD Phases 1 (`spec-crystallization.md`), 1b (`decomposition.md`), 2a–2b (`implementation.md`), and 3 (`review-session.md`), plus a meta-primer for suite development (`suite-development.md`).

The "Session primers" section later in the README acknowledges this, but the opening paragraphs lock the reader into "this is the adversary suite." A new reader looking for spec-crystallization guidance would not expect to find it under `iterative-adversarial-refinement/`. This is the suite's analog of "implementation grew beyond DESIGN.md" — Solution Owner's central concern when applied to the suite-as-artifact. The expansion was healthy; the spec was never updated to reflect it.

**Resolution:** Added new `## Suite scope` section to `README.md` between the IAR intro and `## VSDD pipeline context`. The section names the transitional state explicitly: directory began as IAR-only and has grown to house adjacent-phase primers; name and identity retained for continuity. Lists the four artifact categories (domain prompts, phase primers, lang supplements, suite governance) with brief descriptions. Names the Phase 4–6 gaps (G-86, G-55, G-54).

---

**Finding 2 — VSDD pipeline table lists "IAR's role" per phase but not where primers exist for each phase. (TW lens — knowledge transfer failure)**

The README's `## VSDD pipeline context` table has columns Phase / Name / What happens / IAR's role. The "IAR's role" column describes what IAR-the-evaluator does for each phase (e.g., "VDD-IAR Alignment dim 1 evaluates spec completeness" for Phase 1). It does not show where primers exist for each phase. A reader scanning the table would not learn that Phase 1, 1b, 2a, 2b, and 3 each have a primer in `prompts/`, even though the rest of the document references them.

The TW knowledge-transfer test (TW dim 9): could a new reader, using only the documentation, find the primer for the phase they are entering? If the table is the natural reference for "what does the suite cover for this phase?" it must answer both questions: where is the primer, and how is the phase evaluated.

**Resolution:** Added "Primer" column to the VSDD pipeline table with file references for Phases 1, 1b, 2a, 2b, 3. Phase 4 cell uses "— (G-86)"; Phase 5 cell uses "— (G-55)"; Phase 6 cell uses "—" (no primer is meaningful at the convergence phase). Updated the trailing sentence to point to the full primer table under `## Session primers` rather than naming only Phase 1/1b.

---

**Finding 3 — `prompts/suite-development.md` opening still frames the suite as adversarial-only. (TW + SO lens)**

The suite-development primer's `## Prompt` section opens: "You are helping develop the Iterative Adversarial Refinement (IAR) suite. The suite is itself a software artifact: it has a specification..." The artifact list in the same paragraph correctly enumerates session primers (this was added in an earlier session), but the framing of the opening sentence still treats "the suite" as IAR-only.

The adversarial-standard guidance that follows ("Apply the same adversarial standard to the suite that the suite applies to projects") was written for review prompts: it talks about dimensions, sycophancy checks, and domain overlap. It applies just as well to construction primers (does the primer prevent a real spec or process gap?), but the framing does not say so. A reader using this primer to develop a new session primer would have to translate the adversarial-standard guidance from "review domain" to "construction primer" without explicit instruction.

**Resolution:** Added a paragraph to `## Prompt` after the artifact list naming the broader scope explicitly: the suite has expanded beyond its original Phase 3 scope to include session primers for adjacent VSDD phases; the directory name and "IAR" identity are retained for continuity. Generalized the adversarial-standard paragraph with a sentence covering primers: a primer's `## Prompt` section without a concrete failure mode produces softer output; a completion-criteria section that is not falsifiable will pass against incomplete artifacts.

---

**Finding 4 — `SUITE-REVIEW.md` lead paragraph names a stale artifact set. (TW lens — accuracy regression)**

The SUITE-REVIEW.md lead paragraph names the implementation as "the domain prompt files, README, and gap analysis log." This list predates the addition of `prompts/` (Review 11) and the `supplements/` expansion. The index covers reviews of session primers (Review 16 reviewed all primers; Review 29 reviewed primer alignment; this Review 30 reviews scope documentation) but the lead paragraph does not name primers or lang supplements as part of the implementation surface.

This is the TW dim 2 failure mode (documentation accuracy): a claim that was correct when written and is no longer correct after additive changes. Stale documentation is actively harmful — it misleads a reader into thinking the suite reviews evaluate a narrower artifact set than they actually do.

**Resolution:** Updated the SUITE-REVIEW.md lead paragraph to include session primers and lang supplements in the implementation list, and to acknowledge that "the adversary should apply the same pressure" applies to both adversarial review prompts (defect detection) and constructive primers (spec/process gap prevention). Added a sentence pointing to `README.md` `## Suite scope` for the artifact map.

---

### New gap registered

**G-87 — Suite scope expansion was implicit; no record of the design decision until 2026-05-03.**

The suite expanded from "IAR adversarial review only" to "VSDD pipeline support library" via additive sessions. Review 11 (2026-04-27) added the spec-crystallization, decomposition, and review-session primers; Reviews 16/29 hardened them; the implementation primer was added later. The CHANGELOG entries describe specific primer changes but no entry frames the meta-transition: "the suite is no longer just IAR, it is the VSDD prompt library."

A future reviewer reading this directory from scratch would see a folder named `iterative-adversarial-refinement/` containing four non-IAR primers and no documented rationale for why those primers live here rather than in a separate `vsdd-prompts/` directory. The README opening was never refactored to acknowledge the scope shift; the SUITE-REVIEW.md framing remained adversarial-only; the suite-development primer continued to frame "the suite" as IAR.

This is the suite-level analog of VDD-IAR Alignment dim 7 (cross-session spec consistency): the suite's behavior shifted across sessions without a corresponding update to the suite's "spec" (the README opening framing and the suite-development governing primer). It is the silent case the dim names — no specific finding was raised in any prior session, but the assumption "this is the IAR suite" silently became false.

Registered as G-87 and immediately marked Addressed by Findings 1, 3, 4 above. The new `## Suite scope` section in the README documents the design decision; the suite-development.md framing update generalizes the adversarial standard to construction primers; the SUITE-REVIEW.md lead paragraph names the broader artifact set.

---

**G-88 — Revisit suite directory name and "IAR" identity after `issue-tracker-cli` completion. (added 2026-05-03)**

Follow-up to Finding 5 below: the rename concern was hallucinated as an action for *this session*, but the underlying question — does the directory name still match what the suite is? — remains live. Registering it as a tracked deferral so the decision is not lost.

Trigger: revisit after the `issue-tracker-cli` project completes. By that point the suite will have absorbed two additional rounds of real-project pressure (Layer 2+ IAR runs and project-level review log standardization), and the question can be evaluated against fresh evidence rather than against the speculative naming-confusion failure mode.

Constraint: any rename is forward-only. Completed projects retain their `iterative-adversarial-refinement/` review-log paths and must not be retroactively rewritten — historical review logs are dated artifacts that reference the suite as it was named at the time of writing. A rename, if adopted, applies to projects whose first IAR run occurs after the decision is recorded.

Registered as G-88. Status: Deferred (the gap registry has an explicit Deferred status for cases like this — tracked but with an explicit reactivation trigger rather than open-ended Open).

---

**G-89 — Standardize project-level domain review log structure on the suite-review pattern. (added 2026-05-03)**

Project-level domain reviews currently use a single accumulating file per domain — `{project}/iterative-adversarial-refinement/QUALITY-ENGINEER-REVIEW.md` holds Review 1, Review 2, ... in one file that grows without bound. Suite reviews use a cleaner pattern: `SUITE-REVIEW.md` is an index only; session entries live in `review-log/YYYY-MM-DD-suite-review.md` with explicit reading conventions, filename conventions, and indexing discipline (one session per row in the index, narrative in the dated file).

The suite-review pattern handles output, naming, reading, and indexing better than the project-level domain pattern: file size stays bounded; chronological discovery is direct (date in filename); index-vs-narrative roles are explicit; the reading convention documented in `SUITE-REVIEW.md` is reusable. The same shape would apply to project-level domain logs — for example, `{project}/iterative-adversarial-refinement/QUALITY-ENGINEER-REVIEW.md` becomes an index pointing into `{project}/iterative-adversarial-refinement/review-log/YYYY-MM-DD-quality-engineer.md` (exact target structure decided when this gap is acted on).

Trigger: revisit after `issue-tracker-cli` completes. By that point the suite has accumulated enough project-level review log experience that the right index/narrative split is informed by real reading patterns rather than speculation about scaling.

Constraint: forward-only. Completed projects retain their existing single-file domain log structure and must not be retroactively split — the historical logs are dated artifacts referencing the structure as it was at the time of writing. The new structure applies to projects whose first IAR run is after the decision is recorded.

Registered as G-89. Status: Deferred. Coordinated with G-88 (suite directory rename) — both gaps reactivate after `issue-tracker-cli` completes; if both are adopted together, a single CHANGELOG entry can document the structural transition.

---

### Hallucinated

**Finding 5 — Suite directory should be renamed to `vsdd-prompts/` or similar.**

A strict reading of "the directory name no longer matches its contents" would suggest renaming. But: (a) the IAR identity is the suite's center of gravity (8 core + 6 extended + 2 meta domains; only 4 phase primers and 1 meta-primer); (b) external project logs reference `iterative-adversarial-refinement/` paths in their own review-log file headers; (c) renaming would invalidate Markdown links across this and dependent repos and would itself require an SO-blessed scope change for every project that has logs under that path; (d) the user's request was to update documentation to reflect the transitional state, not to refactor the directory layout.

The documentation update is the proportionate action. A directory rename is a separate decision that requires its own session and its own gap registration if a clear naming-confusion failure mode is observed in practice. **Hallucinated** as a defect requiring action in this session — the concern is real but the resolution would be disproportionate, and the documentation update suffices to disambiguate for any reader entering through the README.

---

**Finding 6 — Suite-development.md primer governing standard should require a "scope" section in every primer.**

A symmetry argument between the domain governing standard and the primer governing standard could push for a "scope" section in every primer. But primers already establish scope via their H1 (`# Session Primer: [Phase] (VSDD Phase N)`) and their "when to use" instructions in the opening paragraphs. Adding a separate scope section would duplicate the H1 + usage block.

The defect this would catch — a primer with unclear scope — is already prevented by the existing structure. Adding a redundant section satisfies a symmetry instinct, not a real failure mode. **Hallucinated** — structural over-engineering without a corresponding defect class.
