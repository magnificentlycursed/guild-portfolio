# 2026-04-28 Suite Reviews

## Review 27 — 2026-04-28 08:00Z

**Context:** Final pass on remaining open gaps. Two deferred items re-evaluated and found actionable.

**Scope:** G-22 and G-30.

**New gaps:** None.

**Addressed gaps:**
- G-22: Added cross-session spec consistency sub-section to VDD-IAR Alignment dim 7 (IAR iteration and feedback routing). Named failure mode: AI's behavioral assumptions shift between sessions without a DESIGN.md update — distinct from feedback routing fidelity (which handles explicit findings). Provides a concrete test: can the current DESIGN.md, read cold, reproduce the current implementation? Owner: VDD-IAR Alignment meta-domain.
- G-30: Added feature-enhancement activation note to SA `### Extended: External Interface Contracts` section. Dims 16 (backward compatibility) and 17 (contract testing) explicitly activate for feature enhancements — any change that existing callers, users, or stored data must survive. No new section needed; the existing dims already cover the failure class when correctly triggered.

**Suite changes made:** `VDD-IAR-ALIGNMENT-REVIEW.md`, `SOLUTION-ARCHITECT-REVIEW.md`, `GAP-ANALYSIS-LOG.md`.

---

## Review 26 — 2026-04-28 07:00Z

**Context:** Follow-on to Review 25. Ownership questions resolved for remaining open gaps; actionable dimension gaps implemented.

**Scope:** G-09, G-10, G-32, G-36 targeted for action. Ownership decisions made for all remaining open gaps.

**New gaps:** None.

**Addressed gaps:**
- G-09: Added Security dim 7 (Audit logging) — named audit events, tamper evidence, retention, forensic reconstruction, context-scoped guidance for single-user vs. enterprise deployment. Owner: Security Engineer.
- G-10: Added Security dim 8 (Data classification and control requirements) — classification tiers, proportionate controls, named failure modes, explicit cross-reference to Privacy dim 1 (Privacy identifies data; Security determines control requirements). Owner: Security Engineer. Privacy dim 1 covers identification; this dimension covers control mandates.
- G-32: Added SA `### Extended: External Service Integration` section (dims 23–27) — external dependency inventory, failure and timeout handling, API contract drift, credentials to external services, data transmitted to external services with cross-reference to Privacy dim 6. Owner: Solution Architect.

**Dismissed gaps:**
- G-36 (side-business transition readiness): Not a software quality concern. Business viability assessment has no natural IAR reviewer role and is out of scope for the suite. Dismissed.

**Ownership decisions recorded for all open gaps** (no suite changes; context-specific domains deferred):
- G-01 (Compliance): Compliance Officer / Regulatory Affairs Engineer — extended domain, activates for regulated industries
- G-04 (Operational Readiness): SRE / Operations Engineer — extended domain, activates for production deployment
- G-05 (Delivery Governance): Delivery Manager / Program Manager — extended domain, activates for externally scoped projects. G-11 (budget tracking) belongs here, not SO.
- G-11 (SO budget): Reassigned to G-05 scope; budget is a delivery constraint, not a spec compliance concern
- G-13 (PE: RTO/RPO): Platform Engineer — dimension strengthening; deferred until deployed-systems context
- G-14 + G-15 (speculative project gaps): Principal Researcher / Research Lead — new Research Review domain, extended, activates when DESIGN.md type is speculative
- G-16, G-17 (speculative SA dims): Solution Architect — conditional dimensions; deferred until speculative project evaluated
- G-18 (Requirements and BA): Business Analyst / Requirements Engineer — extended domain, activates for externally commissioned projects
- G-22 (AI context drift): VDD-IAR Alignment — deferred; no concrete reviewable implementation path identified
- G-26 (Change Management): Change Manager / Organizational Change Manager — enterprise/consulting only
- G-28 (Client Alignment): Engagement Manager / Client Partner — consulting only
- G-29 (Discovery/Advisory): Principal Consultant / Technical Advisor — advisory engagements only
- G-30 (Feature Enhancement): SA Extended (activation note in External Interface Contracts) — minor; deferred
- G-31 (Professional liability): Legal Counsel / Risk Manager — consulting only
- G-54 (four-dimensional convergence): VDD-IAR Alignment — Phase 5+ concern; deferred
- G-55 (Formal hardening): Formal Verification Engineer — VSDD Phase 5 domain; deferred
- G-57 (Effectiveness test): Suite maintainer, not a reviewer role — requires a companion benchmark project, not a domain file

**Suite changes made:** `SECURITY-REVIEW.md`, `SOLUTION-ARCHITECT-REVIEW.md`, `GAP-ANALYSIS-LOG.md`.

---

## Review 25 — 2026-04-28 06:00Z

**Context:** User-directed review of all open gaps in GAP-ANALYSIS-LOG.md with adversarial prioritization: which gaps should be addressed now vs. deferred?

**Scope:** All 29 open gaps reviewed against the current suite state and the primary use case (Phase 1 apprentice portfolio project, single developer, no production deployment).

**New gaps:** None identified.

**Addressed gaps:**
- G-84: Added `## Technical Writer` sections to `javascript-typescript.md` (TypeDoc/JSDoc coverage, TSDoc comment completeness, README example accuracy, `@deprecated` markers) and `rust.md` (rustdoc coverage, doc test quality, module-level docs, `#[doc(hidden)]` discipline, `cargo doc --document-private-items`). Domain file supplement notes updated to standard "Apply the section" language.
- G-85: Added `## Localization` sections to `javascript-typescript.md` (`Intl.*` API usage, i18next configuration, missing key handling, locale injection in tests) and `rust.md` (fluent-rs bundle configuration, message completeness, missing message error handling, rust-i18n macro usage). Domain file supplement notes updated.

**Dismissed gaps:**
- G-07 (auth/authz): Current Security dim 6 substantially covers authentication and authorization with detailed multi-bullet content. Remaining gap is "dedicated auth domain for complex multi-user systems" — a future concern, not a missing dimension. Status updated to Addressed (partial). Inline G-07 note in Security domain revised to remove the stale "this is insufficient" framing.
- G-08 (session management): Session tokens, expiry, and logout completeness are now inside Security dim 6. No separate dimension needed. Status updated to Addressed (partial).
- G-24 (test gaming): QE dim 2 Red Gate subection and dim 14 TDD proxy indicators collectively cover this failure mode. Sycophancy check names "internally consistent but both wrong" explicitly. Status updated to Addressed (partial).
- G-25 (AI anti-patterns): Existing Security dims 1–6 catch the symptoms of AI-generated anti-patterns; Security sycophancy check explicitly warns against rationalizing unreviewed risks. No distinct failure class unowned. Status updated to Addressed (partial).

**Deferred gaps (confirmed):** G-01, G-04, G-05, G-09–G-13, G-14–G-18, G-22, G-26, G-28–G-32, G-36, G-54, G-55, G-57 — wrong context (enterprise/consulting/speculative/Phase 5+) or no concrete implementation path.

**Suite changes made:** `javascript-typescript.md`, `rust.md`, `TECHNICAL-WRITER-REVIEW.md`, `LOCALIZATION-REVIEW.md`, `SECURITY-REVIEW.md`, `prompts/suite-development.md`, `GAP-ANALYSIS-LOG.md`.

---

## Review 24 — 2026-04-28 05:00Z

**Scope:** Generalist adversarial pass. Read: spec-crystallization.md, decomposition.md, implementation.md, PRIVACY-REVIEW.md, LOCALIZATION-REVIEW.md, TECHNICAL-WRITER-REVIEW.md, DATA-ENGINEER-REVIEW.md, javascript-typescript.md, rust.md (Data Engineering section), README.md (full), DOMAIN-INDEX.md. Triggered by user request.

**Lens:** Governing standard format conformance for all role domain files, session primer structure compliance, lang supplement section name accuracy. Previous passes had cleared coordination gaps, classification schema issues, and supplement reference language — this pass looked for format deviations in domain header elements.

---

### Resolved

**Finding 1 — `DATA-ENGINEER-REVIEW.md` reviewer role line missing parenthetical job title variants.**

All role domain files follow the governing standard format `**Reviewer role: [Title]** ([Job title variants])`. DATA-ENGINEER-REVIEW.md's reviewer role line reads only `**Reviewer role: Data Engineer**` — no parenthetical. Every other core domain includes variants (e.g., Software Engineer: "Software Engineer / Backend Engineer / Frontend Engineer"; Security Engineer: "Security Engineer / Application Security Engineer"). The omission is consistent with the README core domain table, which also lists "Data Engineer" with no variants — while all other rows in that table use the slash-delimited variant format. A reviewer who skims the README's Job title column gets role context from the variants; DE gives none.

**Resolution:** Added variants to the reviewer role line in `DATA-ENGINEER-REVIEW.md`: `(Data Engineer / Database Engineer / Data Platform Engineer)`. Updated the README core domain table "Job title" cell for Data Engineer to match.

---

## Review 23 — 2026-04-28 04:00Z

**Scope:** Generalist adversarial pass. Read: RED-TEAM-REVIEW.md, VDD-IAR-ALIGNMENT-REVIEW.md, SOLUTION-OWNER-REVIEW.md, PERFORMANCE-ENGINEER-REVIEW.md, ACCESSIBILITY-REVIEW.md, suite-development.md classification schema, review-session.md. Triggered by user request.

**Lens:** Classification schema coverage in session primer. Previous passes had addressed coordination gaps, supplement reference accuracy, lang supplement symmetry, and domain format issues — this pass looked for gaps in the review-session.md classification reference table.

---

### Resolved

**Finding 1 — `review-session.md` Deferred exclusion list omits VDD-IAR Alignment.**

The Deferred bullet in review-session.md's classification table reads: "Not valid for Security or Red Team — security findings are not deferred." The governing standard (`suite-development.md` lines 73) explicitly establishes that VDD-IAR Alignment also has no `deferred` classification — process findings are binary (either the process ran or it didn't). Portfolio Assessment is separately represented in the table via the `Demonstrated / Partial / Absent` bullet. VDD-IAR Alignment was the only domain with a non-standard schema exclusion not called out — a reviewer following the primer's table could incorrectly defer a VDD-IAR Alignment finding rather than resolving or dismissing it.

**Resolution:** Added VDD-IAR Alignment to the Deferred exclusion note: "Not valid for Security, Red Team, or VDD-IAR Alignment — security findings are not deferred; VDD-IAR Alignment process findings are binary (either the process ran or it didn't)."

---

## Review 22 — 2026-04-28 03:00Z

**Scope:** Generalist adversarial pass. Read: PRIVACY-REVIEW.md, LOCALIZATION-REVIEW.md, PERFORMANCE-ENGINEER-REVIEW.md, RED-TEAM-REVIEW.md, SOLUTION-OWNER-REVIEW.md, CHANGELOG.md, and the full suite-development.md governing standard lang supplement coverage table. Triggered by user request.

**Lens:** Format consistency across domain files, supplement note accuracy vs. the suite-development.md table, and suite self-description accuracy. Previous passes had addressed coordination gaps and dimension gaps — this pass looked for format drift and internal contradictions.

---

### Resolved

**Finding 1 — `CHANGELOG.md` header says "AIR suite" — should be "IAR suite".**

The CHANGELOG.md description line reads "All notable changes to the AIR suite are recorded here." Every other suite artifact consistently uses "IAR" (Iterative Adversarial Refinement). "AIR" appears to be a transposition introduced at file creation and never caught.

**Resolution:** Changed "AIR suite" to "IAR suite."

---

**Finding 2 — `PERFORMANCE-ENGINEER-REVIEW.md` lang supplement note uses "Consult" — the only domain that doesn't name a section.**

All other domains with lang supplement references say "Apply the **[Role Name]** section from the relevant supplement file in addition to the standard dimensions below." PE's note said only "Consult `../../lang/` for language-specific performance tooling and patterns." Without a section name, a reviewer following the note doesn't know what to look for in the supplement. This is especially confusing since the supplements contain an explicit `## Performance Engineer` section that was never referenced by name.

**Resolution:** Rewrote the PE supplement note to match the standard format: "Apply the **Performance Engineer** section from the relevant supplement file in addition to the standard dimensions below."

---

**Finding 3 — `SOLUTION-OWNER-REVIEW.md` supplement note directs reviewer to a non-existent SO section.**

SO's supplement note said "Consult `../../lang/` for the supplement matching the project's primary language and interface type... use the language supplement to verify that technology choices (libraries, tools, frameworks) are appropriate to the language." But the `suite-development.md` lang supplement coverage table marks SO as "Language-agnostic" with dashes in both the JS/TS and Rust columns — no SO section exists in either supplement. A reviewer following the note would consult the supplement, find no SO section, and have no guidance on what to apply. The note also contradicted the governing table in the same primer.

**Resolution:** Replaced the note with an explicit opt-out that names what to use instead: "Not applicable. The SO review evaluates spec compliance, which is language-agnostic. For evaluating technology choices (dim 3), consult the **Solution Architect** section of the relevant supplement — SA evaluates technology fitness from an architectural lens that informs SO's technology compliance check."

---

## Review 21 — 2026-04-28 02:00Z

**Scope:** Generalist adversarial pass — governing documents loaded (VSDD whitepaper, VDD whitepaper, apprentice-onboarding `01-how-we-build.md`), then full suite artifact read. Focus areas: SA, DE, UX, Accessibility domains; both lang supplements (rust.md, javascript-typescript.md); browser-app.md; session primers. Triggered by user request.

**Lens:** Inter-domain coordination completeness, lang supplement symmetry, supplement reference actionability. Previous passes had cleared most structural issues — this pass looked for coordination gaps and cross-supplement inconsistencies.

---

### Resolved

**Finding 1 — SA coordination missing DE.**

SA's coordination paragraph lists QE, UX, Security, and PE — but not DE. SA dim 3 explicitly evaluates "data model integrity — is the data model well-defined and minimal for the use case? Are invariants enforced at the right boundaries?" This maps directly to DE dim 1 ("Data model correctness — Does the data model accurately represent the domain described in DESIGN.md?"). When SA identifies a data model concern at the architectural level, the natural escalation is DE for deeper data-layer analysis. Without the coordination link, a reviewer finding a data model issue at SA has no prescribed path to flag it to DE.

**Resolution:** Added `[DATA-ENGINEER-REVIEW.md](DATA-ENGINEER-REVIEW.md) (data model integrity findings from dim 3)` to SA's coordination links with inline context explaining the handoff boundary.

---

**Finding 2 — `javascript-typescript.md` Platform Engineering section missing coverage enforcement tooling.**

The `rust.md` supplement includes a "Coverage enforcement" bullet in its Platform Engineering section naming the enforcement requirement. `javascript-typescript.md` Platform Engineering has five bullets covering `npm ci`, lock file, audit, Node version pinning, and TypeScript strict — but no coverage tooling guidance. The base PE domain (dim 6) asks whether coverage thresholds are enforced in CI but names no tools. A reviewer evaluating a Jest or Vitest project with no coverage thresholds has no supplement guidance on where to look (Jest `coverageThreshold`, Vitest `coverage.thresholds`, `c8`/`nyc`). The rust.md supplement names tools; the JS/TS supplement should too.

**Resolution:** Added "Coverage enforcement" bullet to `javascript-typescript.md` Platform Engineering naming Jest, Vitest, c8, and nyc threshold configuration locations. Same 80%/100% threshold floors as the base QE domain and rust.md supplement.

---

**Finding 3 — `ACCESSIBILITY-REVIEW.md` supplement note unactionable — `browser-app.md` has no `## Accessibility` section.**

The governing standard says "Apply the **[Domain]** section from the relevant supplement file." `ACCESSIBILITY-REVIEW.md` says "See `../../lang/browser-app.md` for browser-specific accessibility dimensions" — but `browser-app.md` has no `## Accessibility` section. The relevant dimensions (focus trap testing, contrast requirements, semantic HTML, reduced motion) live inside the `## UX` section of `browser-app.md`. A reviewer following the governing standard instruction to "apply the Accessibility section" would look for that section, not find it, and conclude there are no browser-specific accessibility supplement dimensions.

**Resolution:** Updated the ACCESSIBILITY-REVIEW.md supplement note to explicitly say "See the **UX** section of `../../lang/browser-app.md`" and name the content (focus trap, contrast, semantic HTML, reduced motion) so the reference is actionable.

---

## Review 20 — 2026-04-28 01:00Z

**Scope:** Generalist adversarial pass — governing documents loaded (VSDD whitepaper, VDD whitepaper, apprentice-onboarding `01-how-we-build.md`), then full suite artifact read. Focus areas: README.md, PORTFOLIO-ASSESSMENT-REVIEW.md, DOMAIN-INDEX.md, VDD-IAR-ALIGNMENT-REVIEW.md, QE/SE/SO domains, all session primers. Triggered by user request.

**Lens:** Cross-cutting consistency, governing document alignment, and whether the suite's own content contradicts the methodology it describes. Specialist passes had cleared most structural issues — this pass looked for semantic problems hiding behind correct structure.

---

### Resolved

**Finding 1 — README.md: stale "Phase 4" reference in session primers section.**

Line 83: "The spec crystallization primer establishes the adversarial posture for spec *writing* — the adversary applies pressure during Phase 1, not only during **Phase 4**." This reference was not updated when IAR was renumbered to Phase 3 in Review 16. The renumbering pass updated the pipeline table, opening paragraph, review-session.md, and VDD-IAR-ALIGNMENT-REVIEW.md purpose statement — but missed this sentence in the Primers section.

**Resolution:** Changed "Phase 4" to "Phase 3".

---

**Finding 2 — QE coordination links missing SE — the domain QE coordinates with most.**

QE's coordination paragraph listed UX, Security, PE, SA, and SO — but not SE. The domain boundary text below the coordination paragraph explicitly states: "When QE finds a logic error in code that has no test for it, flag the missing test here. SE flags the bug. Both findings are valid and non-duplicative." SE is QE's most natural coordination target by the suite's own description of domain boundaries, yet was absent from the coordination link list. A QE reviewer following only the coordination links would have no pointer to SE despite the explicit split described a paragraph below.

**Resolution:** Added `[SOFTWARE-ENGINEER-REVIEW.md](SOFTWARE-ENGINEER-REVIEW.md) (logic bugs where a missing test is also a QE finding)` as the first entry in QE's coordination links.

---

**Finding 3 — PORTFOLIO-ASSESSMENT-REVIEW.md: regression check unactionable — no instruction to read prior assessment log.**

The regression check says: "If this developer has had a prior portfolio assessment, verify that competencies demonstrated in that assessment remain demonstrated in the current work." The Current Review Prompt read instructions say to read DESIGN.md and the assignment brief — no instruction to read the prior project's `PORTFOLIO-ASSESSMENT-REVIEW.md` log. A reviewer following the instructions cannot perform the regression check because they have not been told where prior assessment data lives. For a first project this check is vacuously met; for a second project the reviewer has no actionable path.

**Resolution:** Added explicit instruction to the regression check: "read that assessment log (`iterative-adversarial-refinement/PORTFOLIO-ASSESSMENT-REVIEW.md` in the preceding project) before evaluating the current work." Added a note for when no prior assessment exists: "vacuously met; note it in the log."

---

**Finding 4 — PORTFOLIO-ASSESSMENT-REVIEW.md dim 8: "could have built without AI" contradicts the methodology.**

Dim 8 summary sentence: "Appropriate scope for a portfolio project means: the developer could have built this without AI assistance, even if it would have taken longer." The governing apprentice-onboarding doc (the VDD methodology document) explicitly positions AI as the builder: "The Builder is the AI agent doing the actual construction... The Human (You) is the director." The methodology assumes AI does the building; the human directs. A developer who used AI to build something more ambitious than they could build solo is exemplifying the methodology correctly, not failing a scope criterion. The "could have built without AI" proxy conflates scope complexity (which is appropriate to scale with AI assistance) with scope ownership (which must be demonstrated regardless of how it was built).

**Resolution:** Rewrote the dim 8 summary sentence to correctly frame the test as ownership rather than solo buildability: "the developer understands and can account for everything that was built. The test is ownership of the complexity, not whether AI was the construction vehicle — the methodology assumes AI does the building. A developer who directed an ambitious implementation and can explain every decision passes this dimension. A developer who accepted the agent's scope expansions without directing them, and cannot account for the resulting complexity, fails it."

---

## Review 19 — 2026-04-28 00:00Z

**Scope:** Second generalist adversarial pass. Read: all four lang supplements (`rust.md`, `javascript-typescript.md`, `cli.md`, `browser-app.md`), `GAP-ANALYSIS-LOG.md`. Triggered by user request following Review 18.

**Lens:** What did the lang supplements and gap registry carry that the specialist domain passes didn't surface? Internal consistency of source citations, gap status accuracy.

---

### Resolved

**Finding 1 — `lang/rust.md` inline source citations scattered through six dimension bullets.**

Six dimension bullets across four sections (Quality Engineering, Security, Software Engineering, Platform Engineering) each ended with the annotation "(Source: claude.md; verify against current apprentice-onboarding content.)" or "(Source: claude.md; verify against current apprentice-onboarding content for authoritative thresholds.)". This is metadata about data provenance, not review guidance. Placed inline in dimension bullets, these annotations interrupt the text of each dimension and train the reviewer to read past them. The appropriate location is a single consolidated note at the file level.

**Resolution:** Added a `**Source note:**` paragraph at the top of `lang/rust.md` (below the H1, before the first section break) consolidating the provenance disclosure. Removed all six inline citations from dimension bullets.

---

**Finding 2 — `GAP-ANALYSIS-LOG.md` G-12 status references a nonexistent file.**

G-12 status showed "Addressed (API-CONTRACT-REVIEW.md)". No such file exists in the suite. The gap (Quality Engineering: no integration/contract testing mandate) was addressed by the Solution Architect domain's Extended: External Interface Contracts section, not by a dedicated review file. The stale reference pointed practitioners to a phantom artifact.

**Resolution:** Updated G-12 status to "Addressed (SA Extended: External Interface Contracts)".

---

**Finding 3 — G-20, G-21, G-23 status "Open" inconsistent with G-76 "Addressed (partial)".**

G-76 (added in Review 9) explicitly registered that G-20/21/23 had been partially addressed by QE and SE additions addressing AI assumption surfacing, hallucination detection, and dependency validation. Yet G-20, G-21, and G-23 themselves still showed "Open" with Last Reviewed dates of 2026-04-25 — predating the partial addressing by two days. G-76 and G-20/21/23 told contradictory stories about the same state.

**Resolution:** Updated G-20, G-21, and G-23 status to "Addressed (partial)" and Last Reviewed to 2026-04-27, consistent with G-76.

---

**Finding 4 — `lang/cli.md` intro and section header contradict each other on whether CLI dimensions replace or supplement UX dimensions.**

The intro paragraph said: "use the dimensions below **in place of (or alongside)** the standard UX dimensions." The section header two lines later said: "The following **replace** the browser-centric UX standard dimensions for CLI projects." "In place of (or alongside)" permits additive application (CLI + browser UX both active). "Replace" forecloses it. A practitioner reading the intro for guidance would not know which behavior to apply.

**Resolution:** Removed "(or alongside)" from the intro paragraph. Intro now reads "in place of the standard UX dimensions," matching the section header.
