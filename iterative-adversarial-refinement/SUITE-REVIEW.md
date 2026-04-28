# IAR Suite Meta-Review

The IAR suite is itself a software artifact. Like any artifact it has a specification (the VSDD and VDD methodology documents), a design (the domain structure, dimensions, and supplement architecture), and an implementation (the domain prompt files, README, and gap analysis log). The adversary should apply to the suite the same pressure it applies to projects under review.

This file logs adversarial review runs of the suite itself. The primary lens is VDD-IAR Alignment — governing doc compliance, process fidelity, and structural integrity. Cross-domain observations from QE and SE are included where they bear on the suite's fitness for purpose.

Governing references:
- VSDD whitepaper: https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00
- VDD whitepaper: https://gist.github.com/dollspace-gay/45c95ebfb5a3a3bae84d8bebd662cc25
- Apprentice-onboarding: https://github.com/Navigators-Guild/apprentice-onboarding
- Crosslink: https://github.com/forecast-bio/crosslink

---

## Suite Meta-Reviews

## Review 15 — 2026-04-27 09:00Z

**Scope:** Generalist adversarial pass. Read: `suite-development.md`, `review-session.md`, `SOLUTION-ARCHITECT-REVIEW.md`, `SECURITY-REVIEW.md`, `README.md`. Triggered by user request.

**Lens:** README Focus column accuracy and SA coordination link completeness. Previous passes had addressed classification schema gaps, domain format issues, and lang supplement additions — this pass checked whether the README domain table reflected the scope expansions made in Gap Analysis Runs 12 and 13.

---

### Resolved

**Finding 1 — `README.md` Security Engineer Focus column omits dims 7 and 8.**

Security dims 7 (Audit logging) and 8 (Data classification and control requirements) were added in Gap Analysis Run 12. The README core domain table Focus cell for Security Engineer still described only the pre-Run-12 scope: "Input handling, persistence data validation, dependency CVEs, secret handling, information exposure, authentication and authorization." A reviewer reading the README to select domains would not know audit logging or data classification coverage was present.

**Resolution:** Appended "audit logging, data classification and control requirements" to the Security Engineer Focus cell in the README core domain table.

**Finding 2 — `README.md` Solution Architect Focus column omits external service integration.**

The `### Extended: External Service Integration` section (dims 23–27) was added to SA in Gap Analysis Run 12. The README core domain table Focus cell for Solution Architect ended at "external interface contracts" — no mention of the new Extended section. A reviewer scanning the table would not know SA covers external service dependencies (inventory, failure handling, API drift, credential management, data transmission).

**Resolution:** Appended "external service integration" to the Solution Architect Focus cell in the README core domain table.

**Finding 3 — `SOLUTION-ARCHITECT-REVIEW.md` coordination links omit Privacy.**

SA dim 27 (data transmitted to external services) contains an explicit cross-reference to `PRIVACY-REVIEW.md` dim 6. SA's coordination section listed QE, UX, Security, PE, and DE — but not Privacy. A reviewer following only the coordination note would not route dim 27 data-transmission findings to Privacy even though the dimension text instructs it.

**Resolution:** Added Privacy to SA coordination links with scoping note: "dim 27 — data transmitted to external services; cross-reference with Privacy dim 6 when Privacy is active."

---

## Gap Analysis Run 13 — 2026-04-28 08:00Z

**Context:** Final pass on remaining open gaps. Two deferred items re-evaluated and found actionable.

**Scope:** G-22 and G-30.

**New gaps:** None.

**Addressed gaps:**
- G-22: Added cross-session spec consistency sub-section to VDD-IAR Alignment dim 7 (IAR iteration and feedback routing). Named failure mode: AI's behavioral assumptions shift between sessions without a DESIGN.md update — distinct from feedback routing fidelity (which handles explicit findings). Provides a concrete test: can the current DESIGN.md, read cold, reproduce the current implementation? Owner: VDD-IAR Alignment meta-domain.
- G-30: Added feature-enhancement activation note to SA `### Extended: External Interface Contracts` section. Dims 16 (backward compatibility) and 17 (contract testing) explicitly activate for feature enhancements — any change that existing callers, users, or stored data must survive. No new section needed; the existing dims already cover the failure class when correctly triggered.

**Suite changes made:** `VDD-IAR-ALIGNMENT-REVIEW.md`, `SOLUTION-ARCHITECT-REVIEW.md`, `GAP-ANALYSIS-LOG.md`.

---

## Gap Analysis Run 12 — 2026-04-28 07:00Z

**Context:** Follow-on to Gap Analysis Run 11. Ownership questions resolved for remaining open gaps; actionable dimension gaps implemented.

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

## Gap Analysis Run 11 — 2026-04-28 06:00Z

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

## Review 14 — 2026-04-28 05:00Z

**Scope:** Generalist adversarial pass. Read: spec-crystallization.md, decomposition.md, implementation.md, PRIVACY-REVIEW.md, LOCALIZATION-REVIEW.md, TECHNICAL-WRITER-REVIEW.md, DATA-ENGINEER-REVIEW.md, javascript-typescript.md, rust.md (Data Engineering section), README.md (full), DOMAIN-INDEX.md. Triggered by user request.

**Lens:** Governing standard format conformance for all role domain files, session primer structure compliance, lang supplement section name accuracy. Previous passes had cleared coordination gaps, classification schema issues, and supplement reference language — this pass looked for format deviations in domain header elements.

---

### Resolved

**Finding 1 — `DATA-ENGINEER-REVIEW.md` reviewer role line missing parenthetical job title variants.**

All role domain files follow the governing standard format `**Reviewer role: [Title]** ([Job title variants])`. DATA-ENGINEER-REVIEW.md's reviewer role line reads only `**Reviewer role: Data Engineer**` — no parenthetical. Every other core domain includes variants (e.g., Software Engineer: "Software Engineer / Backend Engineer / Frontend Engineer"; Security Engineer: "Security Engineer / Application Security Engineer"). The omission is consistent with the README core domain table, which also lists "Data Engineer" with no variants — while all other rows in that table use the slash-delimited variant format. A reviewer who skims the README's Job title column gets role context from the variants; DE gives none.

**Resolution:** Added variants to the reviewer role line in `DATA-ENGINEER-REVIEW.md`: `(Data Engineer / Database Engineer / Data Platform Engineer)`. Updated the README core domain table "Job title" cell for Data Engineer to match.

---

## Review 13 — 2026-04-28 04:00Z

**Scope:** Generalist adversarial pass. Read: RED-TEAM-REVIEW.md, VDD-IAR-ALIGNMENT-REVIEW.md, SOLUTION-OWNER-REVIEW.md, PERFORMANCE-ENGINEER-REVIEW.md, ACCESSIBILITY-REVIEW.md, suite-development.md classification schema, review-session.md. Triggered by user request.

**Lens:** Classification schema coverage in session primer. Previous passes had addressed coordination gaps, supplement reference accuracy, lang supplement symmetry, and domain format issues — this pass looked for gaps in the review-session.md classification reference table.

---

### Resolved

**Finding 1 — `review-session.md` Deferred exclusion list omits VDD-IAR Alignment.**

The Deferred bullet in review-session.md's classification table reads: "Not valid for Security or Red Team — security findings are not deferred." The governing standard (`suite-development.md` lines 73) explicitly establishes that VDD-IAR Alignment also has no `deferred` classification — process findings are binary (either the process ran or it didn't). Portfolio Assessment is separately represented in the table via the `Demonstrated / Partial / Absent` bullet. VDD-IAR Alignment was the only domain with a non-standard schema exclusion not called out — a reviewer following the primer's table could incorrectly defer a VDD-IAR Alignment finding rather than resolving or dismissing it.

**Resolution:** Added VDD-IAR Alignment to the Deferred exclusion note: "Not valid for Security, Red Team, or VDD-IAR Alignment — security findings are not deferred; VDD-IAR Alignment process findings are binary (either the process ran or it didn't)."

---

## Review 12 — 2026-04-28 03:00Z

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

## Review 11 — 2026-04-28 02:00Z

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

## Review 10 — 2026-04-28 01:00Z

**Scope:** Generalist adversarial pass — governing documents loaded (VSDD whitepaper, VDD whitepaper, apprentice-onboarding `01-how-we-build.md`), then full suite artifact read. Focus areas: README.md, PORTFOLIO-ASSESSMENT-REVIEW.md, DOMAIN-INDEX.md, VDD-IAR-ALIGNMENT-REVIEW.md, QE/SE/SO domains, all session primers. Triggered by user request.

**Lens:** Cross-cutting consistency, governing document alignment, and whether the suite's own content contradicts the methodology it describes. Specialist passes had cleared most structural issues — this pass looked for semantic problems hiding behind correct structure.

---

### Resolved

**Finding 1 — README.md: stale "Phase 4" reference in session primers section.**

Line 83: "The spec crystallization primer establishes the adversarial posture for spec *writing* — the adversary applies pressure during Phase 1, not only during **Phase 4**." This reference was not updated when IAR was renumbered to Phase 3 in Review 6. The renumbering pass updated the pipeline table, opening paragraph, review-session.md, and VDD-IAR-ALIGNMENT-REVIEW.md purpose statement — but missed this sentence in the Primers section.

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

## Review 9 — 2026-04-28 00:00Z

**Scope:** Second generalist adversarial pass. Read: all four lang supplements (`rust.md`, `javascript-typescript.md`, `cli.md`, `browser-app.md`), `GAP-ANALYSIS-LOG.md`. Triggered by user request following Review 8.

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

G-76 (added in gap analysis run 9) explicitly registered that G-20/21/23 had been partially addressed by QE and SE additions addressing AI assumption surfacing, hallucination detection, and dependency validation. Yet G-20, G-21, and G-23 themselves still showed "Open" with Last Reviewed dates of 2026-04-25 — predating the partial addressing by two days. G-76 and G-20/21/23 told contradictory stories about the same state.

**Resolution:** Updated G-20, G-21, and G-23 status to "Addressed (partial)" and Last Reviewed to 2026-04-27, consistent with G-76.

---

**Finding 4 — `lang/cli.md` intro and section header contradict each other on whether CLI dimensions replace or supplement UX dimensions.**

The intro paragraph said: "use the dimensions below **in place of (or alongside)** the standard UX dimensions." The section header two lines later said: "The following **replace** the browser-centric UX standard dimensions for CLI projects." "In place of (or alongside)" permits additive application (CLI + browser UX both active). "Replace" forecloses it. A practitioner reading the intro for guidance would not know which behavior to apply.

**Resolution:** Removed "(or alongside)" from the intro paragraph. Intro now reads "in place of the standard UX dimensions," matching the section header.

---

## Review 8 — 2026-04-27 23:30Z

**Scope:** Generalist adversarial pass — no domain framework, fresh pressure across all artifacts. Read: all 14 role domains, 2 meta domains, 5 primers, DOMAIN-INDEX.md, README.md, SUITE-REVIEW.md, governing documents. Triggered by user request following Reviews 6 and 7.

**Lens:** What did the specialist passes miss? Cross-cutting consistency, structural integrity, self-consistency after the phase renumbering, coordination link format compliance.

---

### Resolved

**Finding 1 — Implementation primer phase labels stale after Review 6 renumbering.**

`prompts/implementation.md` H1 still said "(VSDD Phase 2–3)". Internal section headers said "## Phase 2: Red Gate" and "## Phase 3: Implementation". After the renumbering, Phase 3 = Adversarial Refinement. These labels were left unchanged when the README and session primers were updated.

**Resolution:** Updated H1 to "VSDD Phase 2a–2b", prompt text from "Phase 2–3" to "Phase 2a–2b", section header "## Phase 2: Red Gate" → "## Phase 2a: Red Gate", "## Phase 3: Implementation" → "## Phase 2b: Implementation".

---

**Finding 2 — Program phase vs. VSDD pipeline phase naming collision.**

VDD-IAR Alignment's `## Program Phase Context` section uses "Phase 1, 2, 3, 4" for apprentice program tiers. After renumbering, VSDD Phase 3 = Adversarial Refinement and Program Phase 3 = "Crosslink required. Expect mature session discipline." These now collide at Phase 3 with no disambiguation. A practitioner reading both numbering systems could confuse them.

**Resolution:** Added a clarifying note at the start of the Program Phase Context section: "Phase" in this section refers to the apprentice program progression tier, not VSDD pipeline phases. Listed the VSDD phases as a contrast reference.

---

**Finding 3 — Accessibility dim 13 duplicates regression check paragraph.**

Accessibility had both a "Regression check:" paragraph in the Current Review Prompt (added in a prior review as a required element) and a dim 13 titled "**Regression**" in the standard dimensions with near-identical text. No other domain carries regression as both a prompt instruction and a standard dimension. The dim was a pre-existing element that became redundant when the regression check was standardized.

**Resolution:** Removed dim 13. Regression coverage is fully provided by the prompt-section regression check paragraph.

---

**Finding 4 — Coordination links use prose text in five domains (TW, Accessibility, Localization, Performance Engineer, Privacy) plus abbreviated display name in Red Team.**

The governing standard requires "named, linked, relative paths" for coordination links. TW, Accessibility, Localization, Performance Engineer, and Privacy all used plain-text abbreviations with context in parentheses ("SE (stale inline comments), SA (...)") instead of `[FILENAME.md](FILENAME.md)` links. Red Team had two correct links but one that displayed "[SA-REVIEW.md]" instead of "[SOLUTION-ARCHITECT-REVIEW.md]".

**Resolution:** Converted all six affected coordination sections to use full-filename Markdown links with relative paths, preserving the parenthetical context annotations. TW coordination updated to use relative path for the meta-domain link: `[../meta/VDD-IAR-ALIGNMENT-REVIEW.md](../meta/VDD-IAR-ALIGNMENT-REVIEW.md)`.

---

**Finding 5 — README portfolio-arc section not cross-referenced to Portfolio Assessment domain.**

The portfolio-arc review guidance and the Portfolio Assessment domain are complementary — per-project evaluation uses the domain; the arc review uses the README section — but the README section didn't reference the domain. A practitioner following the arc guidance wouldn't know about the per-project domain.

**Resolution:** Added a sentence to the portfolio-arc section referencing `domains/meta/PORTFOLIO-ASSESSMENT-REVIEW.md` with a link.

---

## Review 7 — 2026-04-27 23:00Z

**Scope:** Second VDD-IAR philosophy and methodology alignment pass. Read: VSDD whitepaper, original VDD whitepaper, apprentice-onboarding `02-the-methodology/01-how-we-build.md`, CLAUDE.md. Evaluated governing standard compliance in both meta domain files and completeness of VDD-IAR Alignment dimensions against VSDD Phase 4 (Feedback Integration) requirements. Triggered by: user request following Review 6.

**Lens:** Does the suite correctly operationalize every phase of the governing VSDD pipeline? Do meta domains conform to their own governing standard?

---

### Resolved

**Finding 1 — VDD-IAR-ALIGNMENT-REVIEW.md element ordering violations.**

Two violations against the governing standard's required element order (Classification → Regression check → Coordination → Sycophancy → Lang supplement):
- Regression check absent entirely (missing between classification and coordination)
- Sycophancy check appeared after lang supplement (should be before)

**Resolution:** Added regression check paragraph ("Process compliance confirmed in prior VDD-IAR runs... does not remain clean automatically") in the correct position between classification and coordination. Moved sycophancy check to before lang supplement.

---

**Finding 2 — PORTFOLIO-ASSESSMENT-REVIEW.md element ordering and completeness violations.**

Three violations:
- Missing "Read DESIGN.md / assignment brief first" instruction (required element 5.2)
- Regression check absent entirely
- Sycophancy check appeared before coordination (should be after)

**Resolution:** Added read instruction ("Read DESIGN.md and the assignment brief for each project before evaluating dimensions that require spec knowledge"). Added regression check ("If this developer has had a prior portfolio assessment, verify that competencies demonstrated in that assessment remain demonstrated"). Moved sycophancy to after coordination.

---

**Finding 3 — VDD-IAR dim 7 missing Feedback Integration routing check.**

VSDD Phase 4 (Feedback Integration Loop) requires that findings route back to the correct earlier phase: spec findings to DESIGN.md (Phase 1), test findings to the test suite (Phase 2a), implementation findings to code (Phase 2b). Dim 7 evaluated iteration count ("were rounds iterated when findings were substantial?") but not routing fidelity. A project where every finding was patched in code rather than the spec would pass dim 7 while violating the governing criterion.

**Resolution:** Added feedback routing fidelity sub-criterion to dim 7: findings should route back to the appropriate earlier phase; a spec gap fixed only in implementation without updating DESIGN.md propagates the error. Reviewer flags any round where the fix artifact does not match the finding type.

---

## Review 6 — 2026-04-27 22:00Z

**Scope:** VDD-IAR philosophy and methodology alignment pass. Read governing documents in full: VSDD whitepaper, original VDD whitepaper, apprentice-onboarding `02-the-methodology/01-how-we-build.md`, CLAUDE.md. Evaluated suite against those governing documents for structural and philosophical alignment. Triggered by explicit user request.

**Lens:** Does the suite correctly embody the VDD/VSDD methodology it claims to implement? Are there claims or descriptions in suite artifacts that contradict the governing documents?

---

### Resolved

**Finding 1 — Phase numbering inconsistency: suite called IAR "VSDD Phase 4" but the governing VSDD whitepaper designates Adversarial Refinement as Phase 3.**

The VSDD whitepaper is unambiguous: Phase 3 = Adversarial Refinement, Phase 4 = Feedback Integration Loop. The suite's README, review-session.md primer, and VDD-IAR-ALIGNMENT-REVIEW.md all stated "VSDD Phase 4 (Adversarial Refinement)" — a direct factual contradiction with the primary governing document. The root cause: the suite inserts Phase 1b (Decomposition) and splits VSDD Phase 2 into Red Gate (2a) and Implementation (2b) sub-phases, pushing every subsequent phase number up by one without documenting the divergence.

**Resolution:** Renumbered throughout to match VSDD. README pipeline table: suite phases 2→2a, 3→2b, 4→3. Added VSDD Phase 4 (Feedback Integration) row, which was previously absent from the table. Updated opening paragraph to "IAR owns Phase 3." Updated session primer table "Phase 2–3" → "Phase 2a–2b" and "Phase 4" → "Phase 3." Updated review-session.md H1 and prompt text to "VSDD Phase 3." Updated VDD-IAR-ALIGNMENT-REVIEW.md purpose statement to "VSDD Phase 3."

---

**Finding 2 — Same-model review limitation undocumented.**

The original VDD methodology was explicitly designed for cross-model adversarial review — Builder (Claude) and Adversary (Gemini/Sarcasmotron) are different agents. The apprentice-onboarding governing doc names this by model. The suite's Session isolation section says context resets mirror "fresh eyes every time" from VDD, but never acknowledges that same-model review carries elevated sycophancy risk that context resets only partially compensate for. A practitioner reading the governing docs and the suite would not learn this tradeoff.

**Resolution:** Added explicit paragraph to README Session isolation section: same-model review carries elevated sycophancy risk (shared failure modes and blind spots); the posture primer, context isolation, and sycophancy checks partially compensate but do not replicate cross-model pressure; highest-stakes reviews should consider a different model as adversary.

---

## Review 5 — 2026-04-27 21:00Z

**Scope:** Full adversarial pass. Evaluated: all 14 role domains, 2 meta domains, 5 primers, 4 lang supplements, DOMAIN-INDEX.md, README.md, GAP-ANALYSIS-LOG.md, SUITE-REVIEW.md, CHANGELOG.md. Triggered by: completion of Review 4 resolution pass.

**Lens:** Does the current suite meet its own governing standard? What did the Review 4 fixes introduce?

---

### Resolved

**Finding 1 — Regression check ordering wrong in all five domains added in Review 4 resolution pass.**

All five regression checks added to TW, Localization, Accessibility, Privacy, and Performance Engineer were inserted after the Coordination paragraph instead of before it. The governing standard (element 5 order: Classification → Regression check → Coordination → Sycophancy → Lang supplement) was violated in all five files simultaneously by the fix that resolved Finding 6 from Review 4. The ordering matters: the regression check scopes the whole review (prior layers always in scope) before the reviewer reads what to flag to other domains.

**Resolution:** Swapped order in all five domain files — regression check now precedes Coordination.

---

**Finding 2 — SOLUTION-OWNER-REVIEW.md missing "Regression check:" paragraph.**

SO had no regression check. The governing standard lists it as a required element. A SO reviewer would not explicitly verify that previously-confirmed spec compliance hasn't been degraded by additions from subsequent layers.

**Resolution:** Added regression check paragraph in the correct position (before Coordination).

---

**Finding 3 — RED-TEAM-REVIEW.md missing "Regression check:" paragraph.**

Same issue. Red Team had no regression check. Implementation changes can silently reopen attack surfaces that prior Red Team passes confirmed as adequately defended.

**Resolution:** Added regression check paragraph in the correct position (before Coordination).

---

**Finding 4 — DOMAIN-INDEX.md broken relative paths to meta domain files.**

The DOMAIN-INDEX.md is at `domains/role/DOMAIN-INDEX.md`. Both meta domain entries used `../../meta/` which resolves outside the repo. Correct path is `../meta/`.

**Resolution:** Fixed both meta domain paths to `../meta/`.

---

**Finding 5 — README and DOMAIN-INDEX contradict each other on Data Engineer domain.**

README said "Optional for projects without a meaningful data layer." DOMAIN-INDEX said "may be scoped down... but must be logged." These are incompatible instructions.

**Root cause analysis — README vs. DOMAIN-INDEX redundancy question:** These files are not redundant — they serve different purposes. README describes what each domain covers (Focus column) and serves as suite documentation. DOMAIN-INDEX is the activation decision reference and the authoritative source for when and whether domains activate. The contradiction arose because README included activation guidance ("Optional for...") that belongs exclusively in DOMAIN-INDEX. Fix: README's Focus column should describe what a domain covers; DOMAIN-INDEX should be the authoritative source for activation criteria, scope-down conditions, and run-or-skip decisions. README should reference DOMAIN-INDEX rather than duplicate or contradict it.

**Resolution:** Removed activation guidance from README's Data Engineer Focus column; replaced with a reference to DOMAIN-INDEX. DOMAIN-INDEX remains authoritative.

---

**Finding 6 — Governing standard requires exactly one Reviewer role line but meta domains have zero, with no documented exception.**

Both meta domain files have no reviewer role line by design (README explicitly states meta domains have no job role persona), but the governing standard said "exactly one — not zero, not two" with no carve-out.

**Resolution:** Added explicit exception to governing standard element 3: "Meta domains (`domains/meta/`) are exempt — they have no job role persona by design."

---

**Finding 7 — VDD-IAR-ALIGNMENT-REVIEW.md `## Governing References` section misplaced.**

The section appeared before `## Current Review Prompt`, making it an element-0 pre-preamble. The governing standard places domain-specific structural sections between the prompt section and the dimensions (element 6). The Governing References section is a required preamble — analogous to Security's `## Threat Model` — that the reviewer must complete before applying the dimensions.

**Resolution:** Removed `## Governing References` from before the prompt, inserted it as a required structural section between the prompt and `## Standard Evaluation Dimensions`. Added prerequisite framing: "Before applying the standard dimensions, locate and read the governing methodology document... Record in the review log as a preamble entry (not a classified finding)."

---

**Finding 8 — SE Extended sections lack coordination notes with Performance Engineer and Technical Writer domains.**

SE Extended: Documentation and SE Extended: Performance had no guidance on what to do when the corresponding extended domains (TW and PE) are active. When both are active, findings land in different logs for the same defects, creating ownership ambiguity and duplicate findings.

**Resolution:** Added coordination notes to both extended sections: when TW is active, defer documentation finding ownership to TW; when PE is active, defer performance finding ownership to PE. SE extended dims apply as the floor when the corresponding domain is not activated.

---

**Finding 9 — spec-crystallization.md `## Project type` section placed before `## Prompt`, violating primer governing standard.**

The governing standard requires `## Prompt` before phase-specific sections. `## Project type` was between the `---` separator and `## Prompt`.

**Resolution:** Moved `## Project type` to after the `## Prompt` section content, between `## Prompt`'s closing `---` and `## Project description`. Added a brief introductory sentence to the section. The AI session still receives the project type framing before the driving questions.

---

**Finding 10 — DATA-ENGINEER-REVIEW.md coordination section missing PRIVACY-REVIEW.md.**

DE dim 9 directly overlaps with Privacy dims 1–2 (data inventory, data minimization). A DE reviewer finding sensitive data handling concerns had no guidance to escalate to Privacy.

**Resolution:** Added Privacy to DE coordination links with explicit note: when Privacy is active, escalate PII findings there rather than resolving them in DE.

---

**Finding 11 — PORTFOLIO-ASSESSMENT-REVIEW.md coordination links are prose, not Markdown links.**

All other domains use linked `[FILENAME.md](path)` references. Portfolio Assessment named domains as prose text.

**Resolution:** Added relative-path Markdown links to VDD-IAR-ALIGNMENT-REVIEW.md (same directory), SOLUTION-ARCHITECT-REVIEW.md, and SOFTWARE-ENGINEER-REVIEW.md (both `../role/`).

---

**Finding 12 — review-session.md lists classification types without noting domain-specific exclusions.**

The "After each domain review" section listed all five common classifications without noting that domain-specific schemas exclude some (Security/Red Team: no `deferred`; SO: `backlogged` not `deferred`; Accessibility/Localization/Portfolio Assessment: unique classifications). A reviewer who only read the primer might misclassify a Security finding as `deferred`.

**Resolution:** Rewrote the classification section as a complete taxonomy with domain-specific callouts for every non-standard classification. Prefaced with: "Valid classifications vary by domain — see the domain file's `## Current Review Prompt` section for the complete schema."

---

## Review 4 — 2026-04-27 20:00Z

**Scope:** Full adversarial pass against the suite-development primer as governing standard. Evaluated: all domain files (14 role, 2 meta), all session primers, all lang supplements, DOMAIN-INDEX.md, README.md, GAP-ANALYSIS-LOG.md, SUITE-REVIEW.md, CHANGELOG.md.

**Lens:** Does every artifact meet its own governing standard? What would a reviewer following these prompts get wrong?

**Suite state:** 14 role domains (8 core, 6 extended), 2 meta domains, 5 session primers, 4 lang supplements, DOMAIN-INDEX.md. All content produced or reorganized in the prior session.

---

### Resolved

**Finding 1 — TECHNICAL-WRITER-REVIEW.md supplement reference points to nonexistent content.**

The supplement line read "Consult `../../lang/` for language-specific documentation tooling (e.g., `rustdoc`, TypeDoc, JSDoc, Sphinx)." No Technical Writer section exists in any supplement file. The governing standard's coverage table explicitly marks this as a gap (G-84). A reviewer following this instruction consults lang/ and finds nothing — then has no basis to either apply or dismiss the section.

**Resolution:** Reworded to acknowledge the gap explicitly, reference G-84, and state that the supplement section applies once written.

---

**Finding 2 — LOCALIZATION-REVIEW.md supplement reference contradicts governing standard coverage table.**

The supplement line read "Consult `../../lang/` for language-specific i18n library recommendations and conventions." The suite-development.md coverage table marked Localization as "Language-agnostic" — a contradiction. The domain body references `Intl.NumberFormat`, `Intl.PluralRules`, and `toLocaleDateString()` — JS/TS APIs — and these are not covered anywhere in the supplements.

**Resolution:** Reworded supplement reference to acknowledge the gap and reference new G-85. Coverage table in suite-development.md updated to remove "Language-agnostic" and mark as gap. G-85 added to GAP-ANALYSIS-LOG.md.

---

**Finding 3 — RED-TEAM-REVIEW.md supplement reference names wrong section.**

The supplement line read "Apply the **Security** section." The Red Team domain has its own distinct supplement sections in both lang supplements; the Security section covers a different failure mode set. A reviewer following this instruction applies Security supplement content to a Red Team review and misses the Red Team-specific attack tooling and patterns.

**Resolution:** Changed to "Apply the **Red Team** section."

---

**Finding 4 — README candidate domain list includes implemented domains.**

The "Suggesting new domains" paragraph listed "Performance, Internationalisation, SEO, Privacy, Formal Verification" as candidate domains. Performance (PERFORMANCE-ENGINEER-REVIEW.md), Privacy (PRIVACY-REVIEW.md), and Internationalisation (LOCALIZATION-REVIEW.md) are all implemented extended domains. Listing them as candidates tells a reviewer that these domains do not exist.

**Resolution:** Removed the three implemented domains. Remaining candidates: SEO, Formal Verification.

---

**Finding 5 — SO, UX, and DE sycophancy checks are generic boilerplate (G-77 partial).**

All three used the identical generic text: "If the agent agreed with every decision reviewed in this domain without challenge, treat that as a finding. An AI agent that validates every choice it helped produce is not providing adversarial review." This is the same check applied to every domain — it names no domain-specific failure mode and gives the reviewer no concrete signal to watch for.

- **SO-specific failure:** An agent that helped write DESIGN.md will not flag scope creep it introduced — it treats the spec as authoritative.
- **UX-specific failure:** An AI cannot experience a user interface; it evaluates code, not the lived experience the code creates.
- **DE-specific failure:** An agent that designed the data model will not question schema decisions — only whether the implementation matches the schema it chose.

**Resolution:** Rewrote all three sycophancy checks with domain-specific failure modes. G-77 updated to Addressed.

---

**Finding 6 — Five domains added after Review 3 are missing the "Regression check:" paragraph.**

ACCESSIBILITY-REVIEW.md, PRIVACY-REVIEW.md, PERFORMANCE-ENGINEER-REVIEW.md, TECHNICAL-WRITER-REVIEW.md, and LOCALIZATION-REVIEW.md all lack a "Regression check:" paragraph in the Current Review Prompt section. Every other active domain has this paragraph. The omission means prior-layer regressions are not explicitly in scope for these five reviewers.

**Resolution:** Added regression check paragraphs to all five domain files.

---

**Finding 7 — Classification schemas for Privacy and Localization absent from governing standard; Portfolio Assessment absent.**

The `prompts/suite-development.md` classification table listed 6 schema variants. Privacy uses `accepted risk` (not in "most role domains"); Localization uses `accepted scope` (unique to this domain); Portfolio Assessment uses `demonstrated`/`partial`/`absent` (entirely different vocabulary). None were documented. A suite developer adding a new domain cannot verify their schema is correct without reading the individual domain files.

**Resolution:** Added Privacy, Localization, and Portfolio Assessment to the classification schemas table with notes on the non-default classifications.

---

**Finding 8 — README "Review logs" example file tree missing 6 extended domain log files.**

The example showed only the 8 core domain log files plus VDD-IAR Alignment. The 6 extended domains (Red Team, Performance Engineer, Technical Writer, Accessibility, Privacy, Localization) were absent, with no indication that they exist. A reviewer setting up logs for the first time would not know to create extended domain log files.

**Resolution:** Added extended domain log files to the example with a comment noting they are included only when active.

---

**Finding 9 — SUITE-REVIEW.md meta-reviews lack a "## Suite Meta-Reviews" section header.**

The gap analysis runs section has a `## Gap Analysis Runs` header that introduces and contextualizes the runs. The meta-reviews (Reviews 1–3) have no equivalent section header — they follow the intro directly under `---`. This creates structural asymmetry and makes the file's organization harder to parse.

**Resolution:** Added `## Suite Meta-Reviews` header before Review 4 (this entry).

---

**Finding 10 — No governing standard format definition for gap analysis run entries.**

The `suite-development.md` "SUITE-REVIEW.md discipline" section stated that every non-trivial change requires an entry and listed what counts as non-trivial — but gave no format for what a gap analysis run entry should contain. A new contributor writing a run entry has no template and no completeness criteria.

**Resolution:** Added a numbered format definition (7 elements: header, context, scope, new gaps, addressed gaps, dismissed gaps, suite changes made) to the "SUITE-REVIEW.md discipline" section.

---

**Finding 11 — UX dim 7 does not reference the Accessibility domain.**

UX dim 7 covers a floor-level accessibility check (labels, contrast, semantic HTML, axe pass). The Accessibility domain covers this at depth (screen reader testing, cognitive accessibility, dynamic announcements, ARIA correctness, zoom/reflow). A reviewer running only UX would not know that a dedicated deeper domain exists.

**Resolution:** Added a sentence to UX dim 7 directing deeper coverage to the Accessibility domain.

---

## Review 3 — 2026-04-27

**Scope:** Full suite pass against the suite-development primer as governing standard. Evaluated: all domain files, both meta domains, all session primers, DOMAIN-INDEX.md, README, lang/ supplements, GAP-ANALYSIS-LOG, SUITE-REVIEW.md, CHANGELOG.md.

**Lens:** Does every artifact meet its own governing standard? What does the suite still fail to catch?

**Suite state:** 14 role domains (8 core, 6 extended), 2 meta domains, 5 session primers, 4 lang supplements, DOMAIN-INDEX.md. New this session: RED-TEAM-REVIEW.md, PERFORMANCE-ENGINEER-REVIEW.md, TECHNICAL-WRITER-REVIEW.md, ACCESSIBILITY-REVIEW.md, PRIVACY-REVIEW.md, LOCALIZATION-REVIEW.md (role reorganization from prior sessions); prompts/implementation.md, prompts/review-session.md, prompts/suite-development.md, domains/role/DOMAIN-INDEX.md (new this session).

---

### Hallucinated

**Finding: `review-session.md` and `implementation.md` missing `## Prompt` heading.**

Claimed both new primers lacked the `## Prompt` heading required by the governing standard. Both files have the heading at line 9. The finding was fabricated — the files were not read before the claim was made. **Hallucinated.**

---

### Resolved

**Governing standard: lang supplement reference marked required without covering explicit opt-outs.**

The governing standard listed the lang supplement reference as a required element of the `## Current Review Prompt` section. VDD-IAR Alignment correctly opts out with a documented rationale ("process compliance is language-agnostic"), but the governing standard did not acknowledge this as a valid form of compliance. A reviewer checking the standard would flag VDD-IAR Alignment as incomplete.

**Resolution:** Updated `prompts/suite-development.md` element 5 to read: lang supplement reference is required, **or** an explicit opt-out line with rationale. Both forms satisfy the requirement.

---

**Portfolio Assessment missing lang supplement opt-out.**

The Portfolio Assessment domain has no lang supplement reference line and no opt-out explanation — unlike VDD-IAR Alignment which explicitly states its opt-out. An agent checking domain files for completeness would flag this as structurally incomplete.

**Resolution:** Added explicit opt-out line to `PORTFOLIO-ASSESSMENT-REVIEW.md`: "Not applicable. Portfolio assessment evaluates developer ownership, growth evidence, and decision rationale — concerns that are independent of implementation language or interface type."

---

**Security `## Threat Model` section: required vs optional not distinguished; no classification guidance for preamble output.**

The governing standard described all domain-specific structural sections as "optional." The Security threat model is required ("a reviewer who cannot state the threat model has not completed this review"). Additionally, the threat model section gave no guidance on how to log its output — reviewers following the classification schema (resolved/accepted risk/dismissed/hallucinated) would have no basis for classifying a threat model statement.

**Resolution (governing standard):** Updated element 6 in `prompts/suite-development.md` to distinguish required sections (prerequisite records, not classified findings; must state what to produce and how to log it) from optional extended sections (conditional sub-dimensions).

**Resolution (Security domain):** Added a logging note to the `## Threat Model` section: the threat model is logged as a preamble record in the review log, before numbered findings, not as a classified finding.

---

**SUITE-REVIEW.md and CHANGELOG.md not updated for this session's work.**

This session added three session primers, one DOMAIN-INDEX file, a Security threat model section, Red Team and Performance Engineer lang supplement sections (JS/TS and Rust), README updates, and defect fixes across 10 domain files. None were logged in SUITE-REVIEW.md or CHANGELOG.md. The suite-development primer's own discipline requires entries for all non-trivial changes.

**Resolution:** Added SUITE-REVIEW.md Run 3 entry (this entry) and a CHANGELOG.md entry covering the full session scope.

---

**G-06, G-19, G-27 status not updated in GAP-ANALYSIS-LOG.**

G-06 (Security: no threat modeling) was addressed by adding the `## Threat Model` section to SECURITY-REVIEW.md. G-19 (Documentation fidelity domain missing) and G-27 (Knowledge Transfer and Handoff domain missing) were both addressed by TECHNICAL-WRITER-REVIEW.md. All three remained Open in the registry.

**Resolution:** Updated all three to Addressed with date 2026-04-27.

---

**Technical Writer lang supplement gap not in registry.**

The suite-development.md coverage table explicitly marked Technical Writer as a gap (no JS/TS or Rust supplement sections for documentation tooling). This gap existed in the primer's table but had no GAP-ANALYSIS-LOG entry — no tracking, no ID, no status.

**Resolution:** Added G-84 to the gap registry.

---

## Review 2 — 2026-04-27

**Scope:** Full adversarial pass across all domain templates and lang/ supplements. Session primed with `prompts/spec-crystallization.md` (adversarial posture: assume the spec is incomplete; find what is missing) and `prompts/decomposition.md` (push back on dimensions that are too large, too vague, or that mix concerns). Governing docs used as the DESIGN.

**Lens:** What slop would this suite fail to catch? Every domain reviewed for production-critical gaps — not process compliance, but defect classes that would reach users undetected. Suite alignment against VSDD reviewed separately. Open gaps from GAP-ANALYSIS-LOG raised and resolved where appropriate.

---

### Quality Engineering

**QE: No coverage threshold in the base domain — CRITICAL**

The base QE domain has no coverage threshold requirement. Dim 13 (quality gates) asks whether thresholds are enforced; it does not state what the thresholds should be. A project with 15% coverage and a passing CI run clears QE review. Coverage thresholds exist only in `lang/rust.md` (80% minimum / 100% public API). JS/TS, Python, Go, and any other language project has no threshold.

A coverage threshold in one language supplement and absent from the domain means the domain is calibrated differently depending on which language is used. That is not a language-specific concern — it is a universal quality floor.

**Resolution:** Add explicit coverage guidance to QE dim 13 (quality gates) in the base domain: minimum meaningful threshold (80% line coverage as a floor), 100% for public API / exported functions. Note that thresholds below 80% require documented rationale. Language supplements may tighten this floor but not remove it.

---

**QE: No mutation testing — slop ships with 100% coverage**

A test suite with 100% line and branch coverage that asserts on the wrong thing passes all QE dimensions. An agent that writes both the implementation and the tests will naturally produce a consistent set — both will reflect the agent's interpretation of the requirement, not the spec. A mutation testing pass (mutmut for Python, Stryker for JS/TS, cargo-mutants for Rust) would kill the mutants that the tests miss.

Dim 2 (falsifiability) and dim 14 (TDD proxy indicators) address this partially through structural analysis. But structural analysis cannot catch a test that says `expect(result.length).toBe(3)` when the spec requires `expect(result.length).toBe(2)` — both are structurally indistinguishable; only running a mutation catches it.

**Resolution:** Add mutation testing as a named dimension or a named supplement item in dim 2 (falsifiability). Not mandatory for every project — mutation testing is slow — but should be recommended for pure functions, validation logic, and any code path where an off-by-one or wrong-comparison would ship silently.

---

**QE: Flaky test detection absent — flaky tests are worse than no tests**

A test that passes 90% of the time creates false confidence. Flaky tests train developers to ignore red CI runs. Nothing in the base QE domain asks whether tests are deterministic. Dim 5 (test architecture) asks about state sharing but does not name flakiness as a failure class.

**Resolution:** Add flaky test detection to dim 5 (test architecture). Named failure modes: timing dependencies (`setTimeout` in assertions), network calls in unit tests, random seed not fixed, `Date.now()` without injection, global state not reset between tests.

---

**QE: Coverage thresholds and mutation testing resolved; other findings resolved in domain file.**

---

### Security

**Security dim 6 (auth/authz) is a placeholder — CRITICAL**

Dim 6 reads: "If the application controls access to actions or data: are authentication and authorization checks present at the right boundaries?" That is one question covering the single most dangerous attack surface in any multi-user application. G-07 (no auth/authz review) has been open since Run 1 and the dim 6 addition is inadequate as a resolution. A real auth/authz review asks:

- Are authentication checks enforced at the API layer, or does the UI do them with no server-side enforcement?
- Can a user escalate privileges by modifying a request parameter, JWT claim, or URL?
- Are access control lists enforced on reads, not just writes?
- Are session tokens bound to user identity (IP, user agent, or other factor) to resist token theft?
- Is the logout path complete — are sessions invalidated server-side, not just client-side?
- Can a logged-out user access protected resources by holding a prior valid token until expiry?

For a portfolio/personal tool with no users, dim 6 as written is acceptable. For any project with auth, it is a finding that something dangerous will ship unchecked.

**Resolution:** Strengthen dim 6. Add specific sub-questions that scale with auth complexity. Note that for projects with authentication, G-07 is applicable and requires a more extensive review than dim 6 covers.

---

**Security: Secrets in logs not covered — HIGH**

Dim 4 (secret handling) asks: "Are credentials, API keys, tokens, and private keys excluded from source code and version control?" It does not ask whether they are excluded from logs, error messages, or crash reports. A secret injected via environment variable that gets included in a debug log, a stack trace printed to console, or a caught error that includes `error.message` with the full connection string is a real production vulnerability. The dim 4 wording implies "not in version control" is sufficient — it is not.

**Resolution:** Expand dim 4 to explicitly include logs, error messages, and monitoring/crash reporting output as surfaces where secrets must not appear.

---

**Security: Prototype pollution not covered (JS/TS) — HIGH**

The JS/TS supplement's Security section covers rendering safety, URL injection, JSON.parse runtime validation, CSP, and npm audit. It does not mention prototype pollution. A `JSON.parse` call on user-supplied or attacker-controlled JSON containing `{"__proto__": {"isAdmin": true}}` or `{"constructor": {"prototype": {"isAdmin": true}}}` can silently modify `Object.prototype` in older or unpatched environments. Libraries that use `Object.assign` or object spread with unsanitized data are particularly vulnerable. This is distinct from the runtime validation dimension — the data may be structurally valid and still pollute the prototype chain.

**Resolution:** Add prototype pollution to the JS/TS Security supplement. Mitigation pattern: `JSON.parse` followed by `Object.freeze` on parsed structures, or using `Object.create(null)` for dictionaries, or explicit prototype pollution detection.

---

**Security: Dependency confusion attack not named — MEDIUM**

The Security domain covers CVE auditing. It does not name dependency confusion attacks: an attacker publishes a public package with the same name as a private internal package, at a higher version number, causing the package manager to prefer the malicious public package. This is supply-chain-adjacent but distinct from CVE auditing — the malicious package has no CVE; it is simply a new package that wins the version resolution race.

**Resolution:** Add to Security dim 3 (dependency security) or PE dim 13 (supply chain integrity) as a named failure mode. Mitigation: private registry scoping, npm `--prefer-offline` flag, `publishConfig.access` enforcement.

---

### UX

**UX: Loading states and async failure entirely absent — HIGH**

The UX domain covers empty states, error messages, feedback patterns, and accessibility. It does not cover:

- **Loading states**: what does the user see while an async operation (fetch, file read, storage write) is in progress? A blank screen or frozen UI is a UX failure that no current dimension catches.
- **Async operation failure recovery**: if a save, load, or update fails mid-operation, does the UI recover cleanly? Does the user know what happened? Is there a retry path? An async failure that silently leaves the UI in a partial state would pass all 13 current UX dimensions.
- **Optimistic updates that fail**: if the UI updates optimistically and the underlying operation fails, is the rollback visible and graceful?

These are production-critical for any app with network or storage operations.

**Resolution:** Add dim 14 (async state and error recovery) to UX: loading states, operation failure recovery, optimistic update rollback, and partial-state avoidance.

---

**UX: Keyboard focus trap not named — MEDIUM**

Dim 3 (focus and keyboard behavior) asks whether every action can be completed with a keyboard and whether focus lands in the right place. It does not explicitly name focus traps — the accessibility failure where focus becomes trapped inside a component (modal, dialog, dropdown) and cannot escape without using the mouse. A focus trap is a WCAG 2.1 Level A failure (2.1.2). Axe will catch it if the component has role="dialog", but custom implementations may not be detected.

**Resolution:** Add focus trap detection explicitly to dim 3 and to the browser-app UX supplement. Include the expectation that custom modal implementations handle focus restoration on close.

---

**UX: Destructive action confirmation is incomplete — MEDIUM**

Dim 12 covers native dialog quality (`window.confirm` text specificity). It does not cover the broader pattern: are destructive actions (delete, overwrite, bulk operations) confirmation-gated at all? An app that deletes a record without any confirmation would have no native dialog at all — dim 12 would not trigger on it because there is nothing to evaluate. The dim only evaluates the quality of confirmations that exist; it does not check for the absence of confirmations that should exist.

**Resolution:** Split dim 12 into two concerns: (a) whether destructive actions have appropriate confirmation gates, and (b) whether those gates use specific, actionable language.

---

### Software Engineering

**SE: Flag arguments (boolean traps) not flagged — HIGH**

A function that takes a boolean parameter that fundamentally bifurcates its behavior (`renderBookmark(bookmark, isEditing)`) is a maintenance hazard and a testing hazard. The boolean is typically not self-documenting at the call site; callers must read the function signature to understand what `true` and `false` mean. More critically, it usually signals that the function has two separate responsibilities that should be two separate functions. Tests for boolean-parametrized functions require double the cases and typically test implementation structure rather than behavior.

Nothing in the SE domain flags this pattern. Dim 4 (function design) asks about "single responsibility" but does not name the boolean-parameter form.

**Resolution:** Add flag argument anti-pattern to SE dim 4 (function design). Name the specific failure mode: a function that takes a boolean controlling fundamentally different behavior paths should be two functions.

---

**SE: Primitive obsession not covered — MEDIUM**

Using raw primitives (strings, numbers, booleans) where a domain type would catch errors at the type system level is a well-known SE failure mode. In TypeScript: using `string` for a URL, `number` for a timestamp, `string` for an ID. In Rust: using `String` where `Url`, `Id`, or a newtype would provide safety. Type-level validation catches entire classes of bugs — passing a URL where an ID is expected — before tests are needed.

Dim 3 (naming) touches on this indirectly but does not name primitive obsession as a category.

**Resolution:** Add primitive obsession to SE dim 3 or create a dim for type safety patterns. Focus on: domain values represented as raw primitives when a newtype or branded type would enforce invariants.

---

### Solution Architect

**SA: Memory leaks and event listener cleanup absent — HIGH (browser apps)**

SA covers separation of concerns, coupling, state management, and the new purity boundary (dim 12). It does not cover the production failure mode most common in long-lived browser applications: memory leaks from event listeners, timers, and closures holding references to DOM nodes or large objects.

A browser app that adds event listeners in response to user actions without removing them when the associated DOM is removed will accumulate listeners indefinitely. This causes performance degradation and eventually crashes in long-running sessions. In a single-page app, this is a production failure that no current dimension catches — the code can be architecturally sound, pass all tests, and still leak.

**Resolution:** Add event listener and timer lifecycle to SA dim 5 (state management) or create a new SA dimension. Add to browser-app.md SA-equivalent notes.

---

**SA: Circular dependency detection absent — MEDIUM (JS/TS)**

A circular import between JS/TS modules can cause one module to receive `undefined` for values that haven't been initialized yet — a silent initialization order bug that is notoriously difficult to diagnose. The SA domain does not ask about circular dependencies, and neither does the JS/TS supplement.

**Resolution:** Add to JS/TS supplement SA section. Tool reference: `madge --circular` for detection.

---

### Data Engineering

**DE dim 3 (schema evolution) is too thin for apps with users — HIGH**

Dim 3 asks: "If the data model changes, can data written under the old schema still be read?" One question. For a deployed app with user data, schema evolution covers: explicit migration scripts, forward/backward compatibility windows, atomic migration rollout, data validation post-migration, and rollback strategy if the migration corrupts data. A project that answers "yes" to the single question with "we have a normalization function" passes dim 3 but may have no tested migration path.

**Resolution:** Expand dim 3 to require: (a) explicit migration strategy documented, (b) migration tested against real data samples, (c) rollback path defined, (d) forward compatibility if old clients may write data after new schema is deployed.

---

**DE: Data volume limits not tested — MEDIUM**

The DE domain does not ask whether the application has been tested with realistic data volumes. A `localStorage`-backed app silently stops accepting writes at ~5-10MB. A list rendered without virtual scrolling becomes unusable at 1000+ items. A synchronous sort of 10,000 items blocks the main thread. None of these are caught by any current dimension.

**Resolution:** Add dim 11 (data volume limits) to DE: has the application been tested with an order-of-magnitude more data than expected? Are storage limits known and enforced explicitly (with a user-visible error) rather than failing silently?

---

### Platform Engineering

**PE: Rollback plan documented ≠ rollback plan tested — HIGH**

PE dim 21 (disaster recovery) asks whether a documented and tested plan exists for recovering from infrastructure failure and whether backups are automated and verified. "Documented and tested" is in the dimension, which is good. But in practice, a reviewer will accept a documented plan with a dismissal of "tested implies documented; the plan exists." The dimension does not separate these two criteria explicitly.

For a deployment that has never been rolled back, the rollback plan is untested speculation, not a plan. The same applies to backup restoration — a backup that has never been restored may be unrestorable.

**Resolution:** Strengthen PE dim 21 to explicitly require that rollback and backup restoration have been tested in a non-production environment, with a record of when they were last tested.

---

### Suite Alignment Against Governing Docs

**VDD-IAR Alignment must gate Layer 1 close, not only final merge**

The README sequencing says "Run VDD-IAR Alignment last." VDD-IAR Alignment is correctly the last domain in the final merge gate. But it should also be run at each layer gate close — specifically to verify that the layer gate was executed correctly: acceptance criteria checked, tests passing, IAR complete. Running it only at the end means layer gate failures are discovered retrospectively, not at the time they occurred.

The VDD-IAR Alignment domain itself (dim 3: layer gate compliance) evaluates historical compliance. It cannot retroactively fix a layer that was opened before the previous one's gate closed. The earlier the check, the more actionable the finding.

**Resolution:** Add a note to the README sequencing section: VDD-IAR Alignment is run last in the final merge gate but should also be run at each layer gate close to verify dims 2–3 (layered decomposition and gate compliance) while the layer is still open and correctable.

---

**G-20 and G-21 (assumption surfacing + hallucination detection) still open — CRITICAL for AI workflow**

These two gaps have been registered as High priority since Run 2 (2026-04-25). They remain completely unaddressed. For a suite designed specifically for AI-accelerated development, these are the highest-impact gaps.

G-20: An AI agent working from a spec will make assumptions about requirements, library behavior, and what the client "probably" wants. None are explicit. A human reviewer catches surprising choices in code review. An AI agent produces confident, fluent code with no signal of uncertainty.

G-21: An AI agent will confidently cite APIs that do not exist, invent package names, and misremember library interfaces. The test suite catches some hallucinated implementations at runtime; it does not catch a hallucinated API with plausible-looking tests written against the hallucinated behavior.

These are not gaps that belong in a separate domain — they belong as cross-cutting prompts in the review header of each domain, instructing the reviewer to actively verify assumptions and check external references.

**Resolution:** Add explicit assumption surfacing and hallucination detection instructions to the base review prompt (Current Review Prompt section) of QE, SE, and SA domains. These are the three domains most likely to encounter AI-generated incorrect external references and unvalidated assumptions about library behavior. Address G-20 and G-21 as partially resolved.

---

**G-23 (dependency/API existence validation) still open — HIGH for AI workflow**

Related to G-21 but distinct: G-23 is checkable. Does the package actually exist in the registry? Does the API endpoint actually respond? Does the third-party service actually support this operation? This should be an explicit checklist item in QE and SA, not an incidental catch during testing.

**Resolution:** Add to QE dim 7 (logic errors) or as a new QE dimension: verify that all referenced external dependencies, APIs, and third-party services actually exist and behave as used. An AI-generated import of a plausible but nonexistent package name will compile if the package exists with that name for a different purpose.

---

**Sycophancy check is identical boilerplate — MEDIUM (structural)**

Nine domains, nine identical sycophancy check paragraphs. A reviewer processing multiple domains in sequence will read the first, recognize the pattern, and skim the rest. More critically, the generic text ("if the agent agreed with every decision reviewed in this domain") does not name the specific failure mode for each domain. Domain-specific text would name the specific risk:

- QE: "An agent that wrote both the implementation and the tests will write tests that validate its own interpretation of the requirement, not tests that would catch if its interpretation was wrong."
- Security: "An agent reviewing its own security implementation will dismiss risks it did not consider during generation as 'out of scope' or 'not applicable to this project.'"
- SA: "An agent that designed the architecture will find the architecture sound because it reflects the agent's own defaults, not because it is the right choice for this project's constraints."

The suite should not prescribe all nine rewrites in this run. But at least QE, Security, and SA — the three domains where AI self-review is most dangerous — should have domain-specific sycophancy checks.

**Resolution:** Rewrite sycophancy checks for QE, Security, and SA. Other domains deferred.

---

### Prompt Review

**spec-crystallization.md: UI-centric driving questions**

The "Features and behaviors" driving questions assume a user-facing application with operations, forms, and displayed data. They do not adapt for: libraries (exported functions with callers, not users), infrastructure tools (no UI, no "empty state"), research/speculative projects (no defined success behavior), or CLI tools (stdin/stdout instead of forms). A practitioner starting a library project with this primer would either skip the questions that don't apply or force-fit them.

**Resolution:** Add a project type framing section at the top of spec-crystallization.md. Before the driving questions, prompt the practitioner to characterize the project type: user-facing app / library / CLI tool / infrastructure / research. Provide brief alternative framings for driving questions where needed.

---

**decomposition.md: Crosslink conflated with all projects**

The decomposition prompt includes the crosslink issue hierarchy section as a standard step. Phase 1 projects do not use crosslink. A Phase 1 practitioner reading this primer will either be confused by the crosslink commands or skip the section — and might also skip the bead-string accountability principle it introduces, which *is* applicable to Phase 1 (just without the tooling).

**Resolution:** Separate the principle (every piece of work is explicitly planned and accountable) from the tool (crosslink commands). State the accountability principle for all projects; gate the crosslink commands behind a "Phase 2+ only" note.

---

### Resolved in this review

1. QE: Coverage threshold in base domain (dim 13)
2. QE: Mutation testing in dim 2 (falsifiability)
3. QE: Flaky test detection in dim 5 (test architecture)
4. Security: dim 4 expanded (secrets in logs)
5. Security: dim 6 expanded (auth/authz sub-questions)
6. Security: prototype pollution in JS/TS supplement
7. UX: dim 14 added (async state and error recovery)
8. UX: dim 3 expanded (focus trap named)
9. UX: dim 12 split (confirmation gate existence vs. quality)
10. SE: flag argument anti-pattern in dim 4
11. SE: primitive obsession in dim 3
12. SA: event listener / timer lifecycle in dim 5
13. SA JS/TS: circular dependency detection
14. DE: dim 3 expanded (migration strategy)
15. DE: dim 11 added (data volume limits)
16. PE: dim 21 strengthened (rollback and backup tested, not just documented)
17. Suite: VDD-IAR Alignment sequencing note (also at layer gate close)
18. QE/SE/SA: assumption surfacing + hallucination detection in review prompts (G-20/21 partial)
19. QE: dependency/API existence validation (G-23 partial)
20. Sycophancy check: domain-specific rewrite for QE, Security, SA
21. spec-crystallization.md: project type framing
22. decomposition.md: principle/tool separation

### Hallucinated

*(none)*

---

## Review 1 — 2026-04-27

**Scope:** All domain template files, lang/ supplements, README.md, GAP-ANALYSIS-LOG.md.
**Artifacts reviewed:** QUALITY-ENGINEER-REVIEW.md, UX-REVIEW.md, SECURITY-REVIEW.md, PLATFORM-ENGINEER-REVIEW.md, SOLUTION-ARCHITECT-REVIEW.md, SOLUTION-OWNER-REVIEW.md, SOFTWARE-ENGINEER-REVIEW.md, DATA-ENGINEER-REVIEW.md, VDD-IAR-ALIGNMENT-REVIEW.md, lang/rust.md, lang/javascript-typescript.md, lang/browser-app.md, lang/cli.md, README.md, GAP-ANALYSIS-LOG.md.
**Primary lens:** VDD-IAR Alignment — governing doc compliance and structural integrity. Secondary observations from QE and SE where relevant.

---

### Resolved

#### Session priming absent — Phase 1 and Phase 2 have no execution support

The suite evaluates whether Phase 1 spec crystallization was done correctly (VDD-IAR Alignment dim 1) and whether Phase 2 Red Gate discipline was followed (dims 4, QE dim 2). But the suite provides no support for *executing* these phases. A practitioner starting a new project with this suite has:

- Domain review prompts — but these are for Phase 4 (Adversarial Refinement). They cannot be used to write a spec or decompose a project.
- A description of what a complete spec looks like (dim 1) — but no prompt that helps produce one.
- A description of what Red Gate compliance looks like (dim 4) — but no session primer that primes the practitioner to enter a test-first mode before touching implementation.

The consequence: practitioners will write specs however seems natural, then be evaluated against criteria they did not know going in. This is not adversarial review — it is a rubric handed out after the exam.

VSDD Phase 1 (Spec Crystallization) and the decomposition step are specifically the phases where the most consequential decisions are made. The adversary should be present there too — not to review the output after the fact, but to prime the session before it begins.

**Resolution:** Created `prompts/spec-crystallization.md` and `prompts/decomposition.md`. These are not domain review prompts — they are Phase 1 and Phase 1b session primers that set the adversarial posture before writing begins. Added a Session Primers section to README.md.

---

#### README references VDD but the governing methodology is VSDD

The README opens: "IAR is the adversarial review mechanism of Verification-Driven Development (VDD)." This is accurate but incomplete. VSDD is the current governing methodology; VDD is its predecessor. The distinction matters because:

- VSDD adds Phase 1 (Spec Crystallization), Phase 2 (Red Gate), and Phase 6 (Four-Dimensional Convergence) as first-class concepts that VDD does not name.
- The IAR suite now enforces Red Gate (dim 4, QE dim 2) and spec completeness (dim 1) that came from VSDD, not VDD.
- A practitioner reading only the README has no indication that VSDD exists or that it is the more complete reference.

The governing references are buried in the VDD-IAR Alignment domain template, which a practitioner may not read until they are already running a review.

**Resolution:** README updated to position IAR as filling VSDD Phase 4 (Adversarial Refinement), describe the full VSDD pipeline briefly, surface governing references at the top level, and add a phase pipeline section.

---

#### VSDD purity boundary map unowned — no domain or dimension enforces it

VSDD's verification architecture principle requires explicit separation of the deterministic/pure core from the effectful shell. This separation is what makes formal verification tractable: pure functions with no I/O can be verified with Kani, Dafny, or property-based tests. Functions with effects cannot. The VSDD purity boundary map is a design artifact — defined at spec time, enforced at implementation time — that marks this boundary explicitly.

No IAR domain currently enforces this. SA dim 1 (separation of concerns) asks about business logic vs. rendering vs. storage separation. That is a different concern — it is about layering, not about purity. A codebase can have clean separation of concerns and still have pure business logic entangled with I/O in ways that preclude formal verification.

The gap is present across all language supplements. In Rust: is the pure core in `lib.rs` with a thin effectful shell in `main.rs`? In JavaScript: are validation and transformation functions pure (no localStorage, no DOM, no fetch), separated from the effectful code that calls them? The CLI supplement's `lib.rs`/`main.rs` split dimension (SA section) is the closest thing, but it's framed as a testability concern, not a VSDD verification architecture concern.

**Resolution:** SA dim 12 added (VSDD purity boundary map). JS/TS supplement SA section added. The JS/TS supplement previously had no SA section at all.

---

### Dismissed

#### No DESIGN.md for the suite itself

The suite evaluates whether projects have complete specs. The suite has no DESIGN.md of its own. This is not ironic — it is structurally appropriate. The suite is a living methodology tool whose requirements are discovered through use and validated against the methodology documents it implements. Its "spec" is the VSDD and VDD whitepapers; its "design doc" is the README plus the governing references; its "acceptance criteria" are the gap analysis runs that verify each domain against real projects.

A DESIGN.md for the suite would describe the suite's own features, which is already done in the README. Adding a formal DESIGN.md would be ceremonial — the right artifact for a deliverable project, the wrong artifact for an evolving methodology tool.

What the suite *does* need — and now has — is a clear articulation of its governing references, its phase pipeline context, and its own review history (this file).

**Classification:** Dismissed.

---

#### Suite has no layered development plan

The suite evolved through reactive gap analysis runs triggered by specific questions or project reviews, not through a planned layer sequence. This looks like a process deviation (VDD-IAR Alignment dim 2 — layered decomposition).

But the analogy doesn't hold. The suite's "layers" are not feature layers in a deliverable — they are methodology iterations driven by real-world use. Run 1 analyzed the suite against mission-critical contexts. Run 4 analyzed it against the guild apprentice-onboarding. Run 7 analyzed it against VSDD. Each run is analogous to a "new project type reveals new gaps" event. You cannot layer-decompose the discovery of requirements that didn't exist until you had real projects to review.

The relevant constraint from VDD-IAR Alignment dim 2 is "explicit bounded layers with defined acceptance criteria." The suite's gap analysis runs do not have defined acceptance criteria, which is a real gap — but it is better classified as a structural limitation of the gap analysis format, not as an undisciplined development process.

**Classification:** Dismissed. The reactive evolution is appropriate for a living methodology tool. The absence of per-run acceptance criteria is a known limitation (see G-57 deferred below).

---

### Deferred

#### Domain prompt effectiveness cannot be tested at the artifact level

The suite's correctness claim is: "these prompts, when given to an AI agent, will produce adversarial findings on projects that have real defects." This claim is not testable from the artifacts. Domain prompts are not source code — they cannot be unit tested, linted, or coverage-checked. Their effectiveness is verifiable only through application on real projects.

The closest thing to a test suite is the bookmark-manager review history: nine IAR domains applied across six layers, producing real findings (URL validation bugs, sort instability, ghost `activeTag` state, label association error, contrast failure) that were fixed before merge. This is a single-project efficacy data point.

A more rigorous approach would define: "here is a project with known defects of type X. A correct domain prompt must find defect X." This is analogous to mutation testing — a test suite that doesn't catch a known mutation is a quality failure. For IAR, a domain prompt that doesn't catch a known defect class is a quality failure. No such benchmark exists.

**Decision:** Log as G-57 (open). The suite is too young for a benchmark project. Reassess when the suite has been applied to 3+ projects with documented post-mortems.

---

#### Forced negativity principle not fully operationalized

The VDD whitepaper names "forced negativity" and "anti-slop bias" as active adversarial postures — the adversary assumes problems exist and must find them, not review neutrally and report what it observes. The methodology describes the adversary (Sarcasmotron) as having a "hyper-critical" stance enforced through negative prompting.

The suite currently states this as a goal: "classified as **hallucinated** (the adversary invented a problem that does not exist)" sets the expectation that the adversary should be finding real problems, not confirming quality. The sycophancy check adds a post-hoc test. But the domain prompts don't prime the adversary with the forced-negativity posture at the start of the session.

The new session primers (`prompts/spec-crystallization.md` and `prompts/decomposition.md`) address this for Phase 1. IAR domain prompts themselves should be preceded by a session priming step that establishes the adversarial stance before the first dimension is evaluated.

**Decision:** The `prompts/` directory is the foundation. A general IAR session primer (for priming a domain review session, not just spec/decomposition) is a candidate for a future prompt file. Deferred — the Phase 1 primers are the higher-priority need.

---

#### Supplement depth inconsistency across languages

The Rust supplement covers 6 IAR domains (QE, Security, SE, PE, DE, SA). JS/TS covers 5 (QE, Security, SE, PE, DE — SA was absent until this run). browser-app covers 3 (QE, Security, UX). CLI covers 3 (UX, QE, SE).

The inconsistency is not arbitrary — the supplements cover what is meaningfully language/interface-specific for each domain. But the principle "if a domain exists, it should have a supplement section if the language has meaningful language-specific concerns for that domain" is not consistently applied. For example:

- `browser-app.md` has no SA section — but browser apps have meaningful SA concerns (client-side state management, component coupling patterns, routing architecture).
- `cli.md` has no SA section — but CLIs have meaningful SA concerns (command enum dispatch, lib.rs/main.rs split, error type hierarchy), covered in `rust.md` because CLI and Rust overlap heavily in the suite's current project context.
- None of the supplements have a VDD-IAR Alignment section — but language-specific process concerns exist (Rust has cargo-fmt and clippy as layer gate requirements; the absence of these from a Rust layer gate is a process finding).

**Decision:** Deferred. The inconsistency is tolerable while the suite is calibrated for a single practitioner's project context. Revisit when the suite is applied to a project in a language with a thinner supplement (e.g., a Go project, a Python project).

---

### Cross-domain observations

#### QE: Hallucinated classification wording is domain-adapted — no finding

The "hallucinated" classification is defined differently across domains. QE: "the adversary invented a problem that does not exist." SO: "the adversary invented a scope deviation or compliance failure that does not exist." VDD-IAR Alignment: "the adversary invented a process failure that does not exist."

This looks like inconsistency. It is not. Each version names the specific category of hallucinated finding for that domain. The QE version is generic because QE covers diverse finding types. The SO version names scope deviations specifically because that is almost all of what SO reviews. These are domain-adapted definitions, not inconsistent ones.

**Classification:** Dismissed.

#### SE: Sycophancy check copy-paste reduces effectiveness

The sycophancy check is identical across all nine domains. A reviewer who opens five domain prompts in sequence will have processed the same paragraph five times before running a single dimension. Repetition reduces salience — the check becomes a pattern to skip.

More specifically: the current check asks "if the agent agreed with every decision reviewed in this domain without challenge, treat that as a finding." This is a retrospective check. The VDD methodology's forced-negativity principle suggests the posture should be established at the start, not checked at the end.

This finding is partially addressed by the session primers (which establish the adversarial posture before the domain review begins). The domain-level sycophancy check remains a weak retrospective catch rather than an active posture setter. Future improvement: domain-specific sycophancy language that names the specific failure mode for that domain (e.g., for QE: "an agent that writes all the tests and then reviews them will find them sufficient").

**Classification:** Dismissed as low priority given session primer addition. Logged as a known SE quality concern for the suite templates.

#### SE: "Language and interface supplement" instruction absent from VDD-IAR Alignment

Every other domain has: "**Language and interface supplement:** Consult `lang/` for the supplement matching the project's primary language..." The VDD-IAR Alignment domain does not, because process compliance is language-agnostic. This is correct behavior, but it should be stated explicitly rather than absent silently — a reviewer running VDD-IAR Alignment after all other domains might notice the missing instruction and wonder if it was overlooked.

**Resolution:** Add a note to VDD-IAR Alignment that language/interface supplements do not apply to this domain (process compliance is language-agnostic). Minor.

---

### Hallucinated

*(none)*

**Summary:** The suite's most significant structural gap — absent Phase 1 and Phase 2 session primers — is resolved by this run. The VSDD purity boundary map gap is resolved (SA dim 12, JS/TS SA section). The README now surfaces the full VSDD pipeline context and governing references. Remaining deferred items are known limitations appropriate to the suite's current maturity level and project context. MVR signal: a second pass is unlikely to produce new findings of comparable significance.

---

## Gap Analysis Runs

Gap analysis evaluates the IAR suite's coverage and fitness for different project contexts — which gaps exist, what they cost in different deployment environments, and whether recent suite changes have addressed them. These runs are distinct from the meta-reviews above, which apply IAR domain lenses to the suite as a software artifact. The gap registry with current statuses is in `GAP-ANALYSIS-LOG.md`.

---

## Gap Analysis Run 10 — 2026-04-27

**Context:** Drafting missing and overlooked technical domains. Prompted by question: what technical domains are overlooked or missing? User confirmed: draft all of them, plus Documentation/Knowledge Transfer/Maintainability and Localization/i18n.

**Suite state at time of run:** Nine core domains. Eight new domains drafted this run. README extended domain table added. Gap registry updated.

### New domains drafted

**Performance (G-02 addressed):** 10 dimensions — time-to-interactive, main thread saturation, asset optimization, data scaling, N+1 patterns, caching/memoization, memory growth, performance budget, testing methodology, regression risk. Calibrated for browser apps and data-intensive tools; light application for simple local tools.

**Accessibility (G-80 addressed):** 13 dimensions — axe scan baseline (floor, not ceiling), keyboard navigation completeness, focus management, focus trap compliance (WCAG 2.1 Level A), ARIA correctness, color contrast, form accessibility, semantic HTML, dynamic content announcements (aria-live), cognitive accessibility, reduced motion, zoom/reflow, regression. Separated from UX domain because accessibility has sufficient depth to warrant dedicated adversarial pressure.

**Privacy (G-03 addressed):** 10 dimensions — data inventory, necessity/data minimization, legal basis, retention policy, user rights (access/erasure/portability), third-party sharing, consent quality, PII in secondary storage, sensitive data categories, privacy by design. Distinct from Security: Security asks whether data can be exfiltrated; Privacy asks whether it should have been collected.

**Observability (G-81 addressed):** 10 dimensions — error surfacing, error classification (user/application/dependency), structured log emission, diagnostic completeness, health surfaces, correlation/request tracing, sensitive data exclusion, local/prod parity, silent success confirmation, runbook coverage. Distinct from PE observability: PE owns infrastructure; this domain owns application-layer instrumentation.

**API Contract (G-12 addressed):** 10 dimensions — contract documentation, breaking change definition, versioning strategy, backward compatibility, contract testing, error contract, input validation at boundary, deprecation process, API ergonomics, CLI contract stability. Applies to REST APIs, libraries, CLI tools, event schemas.

**Documentation (G-82 addressed):** 10 dimensions — README completeness, documentation accuracy, architecture documentation, decision rationale, inline comment quality, API/interface docs, operational docs, CHANGELOG quality, knowledge transfer test, AI session independence. Distinct from SE dim 11 and SA dim 11 — those are brief; this domain applies sustained pressure.

**Portfolio Assessment (G-34 addressed):** 8 dimensions — decision ownership, implementation understanding, directed development evidence, growth evidence, failure and recovery honesty, spec ownership, extensibility confidence, appropriate scope judgment. Uses "demonstrated/partial/absent/hallucinated" classification rather than standard. Requires developer participation. Portfolio and apprentice program submissions only.

**Localization (G-83 addressed):** 10 dimensions — string externalization, date/time/number formatting, text expansion tolerance, RTL support, plural rules, locale-sensitive validation, character encoding, cultural neutrality, locale testing strategy. Evaluates i18n readiness; L10n content out of scope.

### Suite changes

README domain table split into core domains and extended domains (active when project scope warrants). All eight new domain files added to `iterative-adversarial-refinement/`. G-02, G-03, G-12, G-34, G-80 through G-83 addressed.

**Remaining open:** G-36, G-54, G-55, G-57. G-20/21/23 partially resolved.

---

## Gap Analysis Run 9 — 2026-04-27

**Context:** Full adversarial roast of the suite — all domain templates reviewed for production slop that would pass undetected, plus suite alignment against governing docs and prompt review. Session primed with `prompts/spec-crystallization.md` and `prompts/decomposition.md`. User instruction: "I expect perfection. Any findings in gap-analysis-log are fair game to raise and resolve."

**Suite state at time of run:** Nine domains. SUITE-REVIEW.md established. Session primers created. 57 gaps registered before this run.

### Findings and resolutions

**QE (G-58–60):** Coverage threshold absent from base domain (any non-Rust project with 10% coverage passed). Mutation testing absent (100% coverage with wrong assertions passes all dims). Flaky test failure modes not named. All three addressed in QE dims 2, 5, 13.

**Security (G-61–64):** Secrets-in-logs not covered (dim 4 only checked source control). Auth/authz dim 6 was a single-line placeholder for the most critical attack surface in multi-user apps — strengthened with six sub-questions. Prototype pollution absent from JS/TS supplement. Dependency confusion attack not named in Security or PE.

**UX (G-65–67):** Loading states and async failure recovery entirely absent from a domain that reviews user-facing feedback patterns. Keyboard focus trap not named despite being WCAG 2.1 Level A. Destructive action gate absence not distinguished from gate quality — dim split into 12 (gate existence) and 13 (gate quality).

**SE (G-68–69):** Flag argument (boolean trap) not named as a function design failure. Primitive obsession not named as a type safety failure.

**SA (G-70–71):** Memory leaks and event listener lifecycle absent from a domain evaluating state management — production failure that tests don't catch. Circular dependency detection absent from JS/TS supplement.

**DE (G-72–73):** Schema evolution dim too thin — one question for a complex migration discipline. Data volume limits entirely absent.

**PE (G-74):** DR dim accepted "documented" as "tested" — distinguished and required test records with dates.

**Suite structural (G-75–77):** VDD-IAR Alignment sequencing added to each layer gate close, not only final merge. G-20/21/23 (assumption surfacing, hallucination detection, dependency validation) partially addressed as explicit instructions in QE, SE, SA review prompts. Sycophancy check rewritten for QE, Security, SA, SE with domain-specific failure modes.

**Prompt gaps (G-78–79):** spec-crystallization.md added project type framing (user-facing app / CLI / library / infrastructure / research). decomposition.md corrected: crosslink replaces TODO.md in Phase 2+ (not supplemented by it); accountability principle separated from tool reference.

**VDD-IAR Alignment dim 2:** Added note that TODO.md (Phase 1) is replaced by crosslink (Phase 2+) — not maintained in parallel.

**Remaining open:** G-34, G-36, G-54, G-55, G-57. G-20/21/23 partially resolved; full resolution requires a dedicated cross-cutting mechanism not yet designed.

---

## Gap Analysis Run 8 — 2026-04-27

**Context:** Meta-adversarial review — IAR suite applied to itself. Prompted by a request to apply the adversary to the suite using governing docs as context, update the README to reflect suite evolution, and add session priming prompts for methodology execution.

**Suite state at time of run:** Nine domains. VDD-IAR Alignment with 11 dimensions + program phase context. Red Gate enforced in dims 4 and QE dim 2. TDD proxy indicators in QE dim 14. Governing references in VDD-IAR Alignment. 55 gaps registered before this run.

**Governing references consulted:** VSDD whitepaper, VDD whitepaper, apprentice-onboarding, crosslink, chainlink (as full-text content).

**Key framing:** IAR suite fills VSDD Phase 4 (Adversarial Refinement). VDD-IAR Alignment evaluates whether the adversary ran with integrity. This run applied that same evaluation to the suite itself.

### Findings

**G-56 — VSDD purity boundary map unowned (High)**

VSDD requires a verification architecture that identifies the pure/deterministic core and the effectful shell. This separation enables unit testing without mocking, formal verification of pure functions, and clear testability boundaries. SA dim 1 (separation of concerns) touched on layering but did not enforce the purity concept. No language supplement named it.

*Decision:* Add SA dim 12 (VSDD purity boundary map). Add SA section to JS/TS supplement (the only supplement missing one) with purity boundary, module organization, state flow, and event handler coupling dimensions.

**G-56 addressed** — SA dim 12 added. `lang/javascript-typescript.md` SA section added.

**G-57 — No effectiveness test for domain prompts (Medium)**

The suite's correctness is verified only through application. There is no benchmark project with known defect types to validate that prompts catch what they claim. This is a real limitation but premature to address — the suite needs more project history before a benchmark is meaningful.

*Decision:* Log as open. Reassess after 3+ projects with documented post-mortems.

**Session priming absent (resolved)**

Two session primers created: `prompts/spec-crystallization.md` (VSDD Phase 1) and `prompts/decomposition.md` (VSDD Phase 1b). README updated with session primers section.

**VSDD not in README (resolved)**

README rewritten to position IAR as VSDD Phase 4, describe the full pipeline, surface governing references at top level, and link to session primers. Previous README referenced only "VDD" — now references VSDD throughout with a pipeline context table.

**VDD-IAR Alignment: language supplement note absent (resolved)**

Every other domain has a language supplement instruction. VDD-IAR Alignment was the only domain without a note explaining why — a reviewer might assume it was accidentally omitted. Added explicit note that language supplements do not apply (process compliance is language-agnostic).

### Suite changes made as a result of this run

**G-56 addressed** — SA dim 12 added; JS/TS supplement SA section added.
**G-57 registered** — Open. Medium severity.
**SUITE-REVIEW.md created** — Meta-review log for adversarial runs against the suite itself.
**prompts/ directory created** — Session priming prompts for Phase 1 (spec crystallization) and Phase 1b (decomposition).
**README.md rewritten** — VSDD pipeline context, governing references, session primers, phase pipeline table, updated domain table with current dimensions.
**VDD-IAR Alignment** — Language supplement N/A note added.

**Remaining open:** G-34, G-36, G-54, G-55, G-57.

---

## Gap Analysis Run 7 — 2026-04-27

**Context:** Suite evaluated against VSDD whitepapers, full apprentice-onboarding repo, and authoritative tool documentation (crosslink, chainlink). Prompted by the question: does the IAR suite accurately reflect VSDD's current methodology, and are tool/phase requirements correctly represented?

**Suite state at time of run:** Nine domains. VDD-IAR Alignment with 10 dimensions. TDD enforcement active (dim 4 hardened, QE dim 14 added). 52 gaps registered before this run.

**Governing references consulted:**
- VSDD whitepaper: https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00
- Original VDD whitepaper: https://gist.github.com/dollspace-gay/45c95ebfb5a3a3bae84d8bebd662cc25
- Apprentice-onboarding: https://github.com/Navigators-Guild/apprentice-onboarding
- CLAUDE.md (may be superseded): https://gist.github.com/dollspace-gay/ef132e60a27abe6d5f87297c1c040dca
- Crosslink: https://github.com/forecast-bio/crosslink
- Chainlink: https://github.com/dollspace-gay/chainlink

**Key clarification:** The IAR suite fills the role of the **adversary** in the VSDD pipeline — specifically VSDD Phase 4 (Adversarial Refinement). The suite is not just inspired by adversarial review; it IS the adversary mechanism. VDD-IAR Alignment evaluates whether the adversary ran with integrity. This framing is now captured in the VDD-IAR Alignment domain intro.

### Findings

**G-53 — Spec crystallization quality unowned (High)**

VSDD Phase 1 defines a spec completeness standard beyond "does a design doc exist": behavioral contracts (preconditions, postconditions, invariants), exhaustive edge case catalog, interface definitions, and verification architecture. No domain evaluated this. The SO domain checked whether the implementation matches the spec; the VDD-IAR Alignment domain checked whether the spec predated implementation. Neither checked whether the spec was complete enough to support valid verification.

A spec that enumerates only happy-path features is effectively unverifiable — the edge cases and failure modes are underdefined, so tests written against it cover only what was anticipated, not what could go wrong.

*Decision:* Add spec completeness criteria to VDD-IAR Alignment dim 1 expansion. This is the appropriate home: VDD-IAR Alignment owns the Phase 1 design gate, and completeness is a Phase 1 attribute.

**G-53 addressed** — VDD-IAR Alignment dim 1 expanded with VSDD Phase 1 spec completeness criteria: behavioral contracts, edge case catalog, interface definitions, verification architecture.

**G-54 — Four-dimensional convergence one-dimensional (High)**

VSDD Phase 6 defines a four-dimensional convergence exit: spec, tests, implementation, AND formal verification must all independently reach MVR. The IAR suite currently tracks only implementation MVR — the point where the adversary produces only hallucinated findings about the code. Spec MVR (the spec has no underdefined behaviors), test MVR (the test suite has no structural weaknesses), and verification MVR (formal proofs or proof harnesses pass) have no tracking mechanism.

In practice, a project where the implementation is refined to MVR but the spec still has gaps or the tests still have structural weaknesses has not fully converged. The exit signal would fire prematurely.

*Decision:* Log as open. This is a structural gap that may require adding dimensions to multiple domains or a new convergence-tracking mechanism. Defer to a future run when the suite is being applied to VSDD Phase 5+ work.

**G-55 — Formal hardening completely unowned (High)**

VSDD Phase 5 defines a formal hardening stage: proof harnesses (Kani for Rust, Dafny), fuzzing (AFL++, cargo-fuzz), mutation testing (mutmut, Stryker), and purity boundary audit. No IAR domain owns this. It is not even listed as a gap — meaning a Phase 5 project evaluated with this suite would get no adversarial pressure on its most sophisticated quality guarantees.

For personal portfolio projects (Phase 1–3), this gap is low severity — formal hardening is not required. For Phase 4 capstone or any VSDD Phase 5 work, it is a critical missing domain.

*Decision:* Log as open. This warrants a dedicated domain (Formal Verification Review) when the suite is first applied to Phase 5 work.

**Issue tracking compliance — not a gap, a phase sequencing clarification**

The suite had no mechanism for evaluating crosslink compliance (or its absence) in a phase-appropriate way. Phase 1 projects are exempt; Phase 2+ projects are required to use crosslink.

*Decision:* Add VDD-IAR Alignment dim 11 (issue tracking compliance) with explicit phase exemptions. Add program phase context section. Update SO dim 9 (assignment compliance) to clarify that absent Phase 2+ tools in Phase 1 projects are not scope deviations.

**Red Gate not explicit in QE dim 2**

VDD-IAR Alignment dim 4 states the Red Gate principle (tests must fail before implementation). QE dim 2 (falsifiability) asked whether tests catch broken implementations but did not explicitly ask whether tests would have passed against a pre-implementation stub — which is the Red Gate criterion.

*Decision:* Add Red Gate language to QE dim 2, cross-referencing VDD-IAR Alignment dim 4.

**lang/rust.md gaps from claude.md**

The claude.md governing reference specified cargo-deny, cargo-vet, stricter clippy lint configuration, and coverage thresholds (80% minimum / 100% public API). None were in lang/rust.md.

*Decision:* Add to lang/rust.md with sourcing note (claude.md, may be superseded). Applied in Security (cargo-deny, cargo-vet), Platform Engineering (cargo-deny, cargo-vet, coverage enforcement), Quality Engineering (coverage thresholds), and Software Engineering (clippy lint configuration).

### Suite changes made as a result of this run

**G-53 addressed** — VDD-IAR Alignment dim 1 expanded with VSDD Phase 1 spec completeness criteria.
**G-54 registered** — Four-dimensional convergence gap logged as Open. Context-dependent: low for Phase 1–3, high for Phase 4+.
**G-55 registered** — Formal hardening gap logged as Open. Context-dependent: low for Phase 1–3, critical for Phase 4+ and mission-critical.
**Dim 11 added** — VDD-IAR Alignment dim 11 (issue tracking compliance) and program phase context section added.
**SO dim 9 updated** — Phase-appropriate tool introduction language added.
**QE dim 2 updated** — Red Gate language added.
**VDD-IAR Alignment intro updated** — IAR-as-adversary framing and governing references section added.
**lang/rust.md updated** — cargo-deny, cargo-vet, clippy lint config, coverage thresholds added across QE, Security, SE, PE sections.

**Remaining open:** G-34, G-36, G-54, G-55. G-54 and G-55 are context-dependent; low severity for current portfolio work.

---

## Gap Analysis Run 6 — 2026-04-27

**Context:** Targeted review of TDD enforcement. Prompted by direct question: does the IAR suite enforce TDD best practices?

**Suite state at time of run:** Nine domains including VDD-IAR Alignment. Prompted evaluation of whether test-first discipline is enforced anywhere in the suite.

### Finding

**G-52 — Test discipline enforcement too weak; TDD proxy indicators absent from QE**

VDD-IAR Alignment dim 4 treated test-after patterns as "a yellow flag" rather than a finding. No domain evaluated whether tests exhibit structural characteristics of test-first development.

The gap has two layers:

1. **Process enforcement (VDD-IAR Alignment):** Dim 4 needed to be hardened — test-after is a finding, not a flag. Positive evidence of test-first should be defined (co-committed tests, failing-test CI evidence, behavior-named tests predating implementation). The "same commit" exception is acceptable with documented rationale; "I wanted to get the code working first" is not.

2. **Artifact enforcement (QE):** No dimension asked whether tests exhibit TDD fingerprints — interface focus, failure specificity against naive implementations, behavioral naming, earned branch distribution, absence of implementation coupling. These are observable from the test artifact without requiring knowledge of when tests were written.

Note: VDD's methodology document (01-how-we-build.md) sequences code-before-tests explicitly. The TDD enforcement here is a deliberate addition beyond VDD's baseline, not a correction to VDD alignment.

**G-52 addressed** — VDD-IAR Alignment dim 4 hardened: test-after is a finding; positive evidence criteria defined; cross-reference to QE dim 14 added. QE dim 14 (TDD proxy indicators) added: interface focus, failure specificity, behavioral naming, branch distribution, implementation coupling.

**Remaining open:** G-34, G-36. No new gaps identified in this run.

---

## Gap Analysis Run 5 — 2026-04-26 01:00Z

**Context:** Organizational evaluation of the suite's structure and alignment with VDD-IAR. Prompted by the question: is this the right organization? Are additional domains needed? Does the suite reflect what VDD-IAR actually is?

**Suite state at time of run:** Nine domains after this run (previously eight). README restructured. VDD-IAR Alignment domain created. SO stripped to spec-contract identity.

### Findings

**G-46 — SO split identity (High)**

The SO domain had accumulated two distinct adversarial postures: spec-contract review (does the implementation match what was asked?) and process governance (was the work done correctly?). These read different artifacts, apply different adversarial frames, and belong in separate sessions. Having them in one domain created a reviewer that had to context-switch mid-review.

*Decision:* Strip SO to spec-contract focus (9 dims). Move process concerns to VDD-IAR Alignment. Move complexity budget for one to SA.

**G-47 — Suite described as gate, not iterative loop (High)**

The README described IAR as a pre-merge gate: "a full run is required before merging." VDD-IAR is a loop: build → adversary → fix → adversary again → repeat until MVR. The suite name says "Iterative" but the documented structure was a single checkpoint. No guidance existed for within-layer iteration, round numbering, or when to stop iterating.

*Decision:* Rewrite README. Replace "Full run" with "Refinement loop" section. Add round-number requirement to log format. Update merging gate to require MVR, not just one passing run.

**G-48 — QE/SE domain overlap without explicit boundary (Medium)**

SE dim 1 (correctness, logic errors) and QE dim 7 (logic errors) covered the same ground. In practice both reviewers would find the same bugs. The distinction — QE owns tests, SE owns code — was valid but unstated, creating duplicated effort without the benefit of independent confirmation.

*Decision:* Add domain boundary statements to QE and SE prompts. QE flags missing tests when it finds a logic error; SE flags the bug. Both findings are valid independently.

**G-49 — PE posture misrepresented (Low)**

The generic sycophancy check ("if the agent agreed with every decision...") doesn't fit a domain where most dimensions are binary existence checks. The real sycophancy risk in PE is rationalized inapplicability decisions, not agreeing with code quality judgments.

*Decision:* Replace generic sycophancy check with a posture note specific to PE's compliance-check nature. Scope the adversarial pressure to judgment-dependent decisions.

**G-50 — No generalist adversary pass (Medium for personal use)**

The IAR specialists each apply a specific framework. The VDD methodology's adversary has no framework — it just finds everything wrong. No domain covered the gaps between specialist frameworks.

*Decision:* Document as an optional unstructured pass in the README (not a formal domain). It intentionally has no dimensions — adding structure would make it another specialist.

**G-51 — VDD-IAR Alignment domain missing (High)**

Process compliance had no owner. Test discipline, layer gate compliance, IAR fresh context, IAR iteration, role integrity, and retrospective quality were either scattered across SO (awkwardly) or unowned. The methodology's "process over product" principle had no adversarial review mechanism.

*Decision:* Create VDD-IAR-ALIGNMENT-REVIEW.md. Ten dimensions covering the full VDD-IAR loop. Runs last in the sequence (reviews artifacts produced by all other domain runs). Mandatory gate before merge.

### Suite changes made as a result of this run

**G-46 addressed** — SO reduced to 9 dimensions (spec/contract focus). Dims 9 (complexity for one), 11 (VDD fidelity), 12 (linear accountability) removed and redistributed. Complexity budget for one moved to SA dim 9 expansion.
**G-47 addressed** — README rewritten around VDD-IAR as the governing framework. "Refinement loop" replaces "Full run." Merging gate updated to require MVR and round numbers.
**G-48 addressed** — Domain boundary notes added to QE and SE prompts.
**G-49 addressed** — PE sycophancy check replaced with a posture note specific to compliance-check domains.
**G-50 addressed** — Generalist adversary pass documented as optional README note.
**G-51 addressed** — VDD-IAR-ALIGNMENT-REVIEW.md created. Added to domain table, sequencing, and merging gate.

---

## Gap Analysis Run 4 — 2026-04-26 00:00Z

**Context:** Personal developer, AI-accelerated workflow, portfolio-to-journeyman arc. Full evaluation of suite against the guild apprentice-onboarding methodology, the bookmark-manager project history (including the dollspace-gay guild review finding), and the upcoming issue-tracker-cli project. Goal: identify gaps that would cause the suite to pass a project that a guild portfolio reviewer would fail.

**Suite state at time of run:** Eight domains, all with sycophancy checks and language/interface supplements. lang/ subfolder with rust.md, javascript-typescript.md, cli.md, browser-app.md. GAP-ANALYSIS-LOG.md with 38 gaps registered.

### Findings

**G-39 — DESIGN.md fitness check missing (High)**

The SO domain treats DESIGN.md as the authoritative contract and checks whether the implementation matches it. But DESIGN.md is a student document, and it can itself be wrong. The bookmark-manager's DESIGN.md specified TypeScript, Vite, and a build toolchain — none of which were asked for by the assignment. Every SO dimension passed. The guild reviewer failed the project on assignment compliance.

The suite had no mechanism to catch scope creep that entered at the design stage, only scope creep that entered during implementation. The upstream assignment brief is the higher-level contract.

*Decision:* Add SO dim 10 (assignment compliance). Requires reading the assignment instructions alongside DESIGN.md and flagging deviations that entered at the design stage.

**G-40 — VDD process fidelity unowned (High)**

The guild portfolio review is explicitly "process over product." Reviewers look at whether the VDD loop was followed: design doc before code, layered development, IAR at each gate, tests alongside or before implementation. Nothing in the suite checked this. A project that is correctly built but built without process discipline would pass all eight domains and still fail a portfolio review.

*Decision:* Add SO dim 11 (VDD process fidelity). The SO domain is the right owner — it already guards the spec contract, and process fidelity is the meta-contract above it.

**G-41 — No MVR exit signal or hallucinated finding classification (High)**

The methodology document describes a specific exit condition: when the adversary starts hallucinating critiques because it cannot find real ones, the code has reached maximum viable refinement. This is the most important signal in the VDD loop. The suite had no classification for hallucinated findings, no guidance on recognizing the MVR signal, and no way to record that exit in the log.

This meant a reviewer running the suite could not distinguish: (a) a clean pass because the code is good, (b) a clean pass because the agent was too agreeable, or (c) an exit because the adversary ran out of real complaints. These are three very different states.

*Decision:* Add **hallucinated** classification to all 8 domain prompts. Add MVR exit signal explanation to README.

**G-42 — Manual testing checklists not owned (Medium)**

The bookmark-manager DESIGN.md included explicit manual testing checklists as part of the layer gate. The DECISIONS.md documents three cases where manual testing caught things automated tests missed (tag toggle deselect, empty URL message, focus after cancel). No domain asked whether manual checklists existed or were completed. This gap is especially significant for CLI projects where automated tests cannot cover all UX concerns.

*Decision:* Add QE dim 14 (manual testing checklists).

**G-43 — Commit history and linear accountability not evaluated (Medium)**

The methodology's "string of beads" principle: every piece of code traces to a sub-issue, every sub-issue to an issue. The portfolio review looks at commit history. A commit log of "fix stuff" or "wip" is a process failure. No domain evaluated commit message quality or traceability. A project could have excellent code and a useless commit history.

*Decision:* Add SO dim 12 (linear accountability).

**G-44 — Same-session sycophancy drift across domains (Medium)**

Each domain has a sycophancy check. But if all eight domains run in the same AI session, the agent accumulates context that softens its adversarial pressure. The methodology's "fresh eyes every time" principle argues for resetting context between rounds. The suite had no guidance on session isolation between domain reviews.

*Decision:* Add session isolation operational note to README under Full run.

**G-45 — Portfolio-arc perspective absent (High for personal use)**

Per-project IAR runs evaluate individual projects. The portfolio review evaluates the arc across all projects: growth, honest retrospectives, independence, assignment alignment patterns. The suite had no guidance for this cross-project perspective. A student who ran excellent IAR on each project but never assembled the arc-level view would miss what portfolio reviewers actually evaluate.

*Decision:* Add portfolio-arc review section to README.

### Suite changes made as a result of this run

**G-39 addressed** — SO dim 10 (assignment compliance) added.
**G-40 addressed** — SO dim 11 (VDD process fidelity) added.
**G-41 addressed** — **hallucinated** classification added to all 8 domain prompts. MVR exit signal explanation and session isolation note added to README.
**G-42 addressed** — QE dim 14 (manual testing checklists) added.
**G-43 addressed** — SO dim 12 (linear accountability) added.
**G-44 addressed** — Session isolation note added to README under Full run.
**G-45 addressed** — Portfolio-arc review section added to README.

**Remaining open:** G-34, G-36 (deferred from Run 3). G-01 through G-32 remain open; most are scoped to contexts (mission-critical teams, consulting engagements) not yet relevant to the current personal portfolio use case.

---

## Gap Analysis Run 3 — 2026-04-25 22:00Z

**Context:** Personal developer using AI-accelerated tools. Single-user scope, no team, no client. Project may be: personal use only, a portfolio piece, or a side business in development. Goal is professional-quality software with adversarial mitigation of AI workflow risks. This is also the context closest to the suite's origin project (bookmark-manager).

**Suite state at time of run:** Same eight domains. 32 gaps from Run 1, 15 new gaps from Run 2. Run 2 not yet committed.

**Prior gap review:** All prior gaps carried forward. Severities re-evaluated for personal context below where they change materially.

### Re-evaluation of prior gaps in personal context

**G-01 (Compliance/Legal):** Low — unless the project handles other people's data or grows into a side business with users. Re-evaluate at that transition.

**G-02 (Performance/Scalability):** Low — you are the only user. Promote to Medium if it becomes a side business.

**G-04 (Operational Readiness):** Low — you operate it yourself. "Know how to restart it" is sufficient.

**G-05 (Delivery Governance):** Changes character. No budget, no team. The personal equivalent is: *am I making progress toward something I will actually finish and use, or am I building indefinitely?* Completion discipline and "done" criteria are real risks on personal projects. Related to G-15 (kill criteria).

**G-18 (Requirements/BA):** Stays relevant but changes character. You are both client and implementer. The gap is no longer between client intent and written spec — it is between what you asked the agent for and what you actually needed. You are just as capable of mis-specifying your own requirements as a client is.

**G-20 (Assumption surfacing):** Promotes to Critical for personal use. In a team or consulting context, someone else may catch an unvalidated assumption. As a solo developer with an AI agent, you are the only human in the loop. If the agent makes a wrong assumption and you do not catch it, no one will.

**G-21 (Hallucination detection):** Promotes to Critical for the same reason. No peer review exists. The adversarial review process is the only check.

**G-22 (Context drift/consistency):** High. Long solo AI sessions are particularly prone to this. You may not notice that a decision made in session 1 was quietly reversed in session 4 because you were not tracking it consciously across sessions.

**G-24 (Test gaming):** High. You wrote the spec and you are the only reviewer. An agent that misunderstood your requirement will produce a consistent implementation and test suite that passes completely. The adversarial check that someone else would provide in code review does not exist.

**G-26 (Change Management):** Not applicable — no organization, no users to adopt.

**G-27 (Knowledge Transfer):** Changes character significantly — see G-37 below.

**G-28 (Client Alignment):** Not applicable — no client.

**G-31 (Engagement Liability):** Not applicable unless it grows into a business with contracts.

### New gaps

**G-33 — No sycophancy detection mechanism**

AI agents are trained to be helpful and agreeable. When you propose a direction, the agent will generally support it, improve on it, and implement it — even if it is wrong. It will not tell you that your data model is misconceived, that your feature idea does not solve the actual problem, or that the approach you are enthusiastic about is the wrong one. It will ask clarifying questions and then build what you described.

Human collaborators push back. The agent does not, unless explicitly prompted to. No domain in the current suite asks: *did the agent challenge any of the key decisions in this session, or did it agree with everything?* Agreement is a red flag in adversarial review. An AI reviewer that finds nothing is more suspicious than one that finds something.

This is the foundational risk of solo AI-accelerated workflows that the rest of the suite partially addresses but never names directly.

**G-34 — No learning and craft development assessment**

If you are building a portfolio piece, the adversarial question is not just whether the software is good — it is whether building it made you better. An agent that produces professional-quality code you do not fully understand is not a portfolio win; it is a liability when you are asked to explain or extend it. No domain asks: do you understand what was built well enough to own it? Could you reproduce the key decisions without the agent?

This matters for portfolio integrity (are you accurately representing your skills?) and for professional development (are you growing, or producing artifacts?). An AI-accelerated portfolio that demonstrates the agent's competence rather than yours is a specific and underappreciated risk.

**G-35 — No future-maintainer assessment (maintainability-for-one)**

In a team context, maintainability is about other engineers understanding the code. For a personal project, the other engineer is future-you — often six or twelve months later, with no memory of the original AI sessions, no access to the conversation history, and a different mental model of the problem.

AI-generated code tends to be correct and functional but written in a style that reflects training data rather than your natural idioms. It may use patterns you would not have chosen, at a level of abstraction that made sense in the original session but is opaque later. No domain currently asks: will future-you be able to understand and modify this without re-running the AI session?

This is distinct from SE dimension 10 (consistency) and from G-27 (knowledge transfer for handoff). It is about your own continuity of understanding over time.

**G-36 — No side-business transition readiness assessment**

A personal project that grows into a side business crosses a threshold where many previously-deferred gaps become relevant simultaneously: compliance (G-01), privacy (G-03), performance (G-02), operational readiness (G-04), and potentially security expansion (G-06 through G-10). If the project was not structured with that transition in mind, the cost of crossing that threshold is high.

No domain currently asks: if this project were to acquire its first paying user tomorrow, what would immediately break or expose liability? What would need to change before that could happen safely?

**G-37 — Knowledge transfer to future-self (session continuity)**

Distinct from G-35 (future maintainability of the code) and G-27 (handoff to another person). This is about the continuity of the AI-assisted development process itself. AI sessions have no memory across conversations. Decisions, context, constraints, and rationale established in one session are not available in the next unless explicitly preserved.

No domain currently asks: is the project's state — its decisions, its constraints, its open questions, its known debt — documented well enough that a new AI session can be productive without rediscovering everything from scratch?

**G-38 — Complexity trap: AI over-engineers for personal scale**

AI agents produce professional-grade complexity by default. Given a personal tool that needs to store 50 bookmarks, an unconstrained agent may produce a layered architecture, an abstracted storage interface, a full test suite with mocking infrastructure, and a CI pipeline. These choices may be technically correct and individually justifiable. Together, for a project you maintain alone, they create a complexity budget that exceeds what one person can comfortably own.

The Solution Owner domain guards against scope additions (features). This gap is different: it is about architectural and infrastructural complexity added not by user request but by the agent's default inclination toward "proper" engineering.

### Gaps that do not apply in personal context

The following gaps from prior runs are not applicable to a solo personal developer with no clients, no team, and no regulated data. Re-evaluate if the project acquires users, a team, or a business structure.

- G-26 (Change Management / Adoption) — no organization to manage
- G-28 (Client/Stakeholder Alignment) — no client
- G-29 (Discovery/Advisory research quality) — not an advisory engagement
- G-31 (Professional and engagement liability) — no client contract
- G-04 (Operational Readiness) — no SLA, no on-call, no users depending on uptime
- G-09 (Audit logging) — no multi-user accountability surface

### Decisions

**Decision — The core value of the IAR suite for a personal developer is adversarial peer review substitution.**
In a team context, the suite augments human review. For a solo developer, it replaces it entirely. This changes the stakes for every domain: there is no fallback. The suite is the only check. This should be stated explicitly in the suite README as a use-case note.

**Decision — G-33 (sycophancy detection) should be added as a cross-cutting dimension to every domain.**
Each domain review should explicitly ask whether the agent challenged any key decisions in its area or agreed with everything. Universal agreement is a warning sign, not a passing grade.

**Decision — G-34 (learning/craft) and G-36 (side-business transition readiness) are portfolio-specific concerns that warrant a lightweight checklist, not a full domain.**
These are pre-project or post-project questions rather than per-layer review dimensions.

**Decision — G-35 (future maintainability) and G-37 (session continuity) should be added as dimensions to SA and SE respectively.**
G-35 fits in SE (code understandable to future-you). G-37 fits in SA (architectural decisions preserved across sessions).

**Decision — G-38 (complexity trap) should be added to the SO domain as a named dimension.**
The SO currently blocks scope additions. It should also explicitly evaluate whether the complexity level — architectural, infrastructural, toolchain — is appropriate for the number of people who will maintain it.

### Suite changes made as a result of this run

**G-33 addressed** — Sycophancy check added to all eight domain prompts.
**G-35 addressed** — Future-self maintainability added as dimension 11 to SOFTWARE-ENGINEER-REVIEW.md.
**G-37 addressed** — Session continuity added as dimension 11 to SOLUTION-ARCHITECT-REVIEW.md.
**G-38 addressed** — Complexity budget for one added as dimension 9 to SOLUTION-OWNER-REVIEW.md.

**Remaining open from Run 3:** G-34 (learning/craft checklist), G-36 (side-business transition checklist). Tier 2 items — new documents, not domain edits. Deferred to next pass.

---

## Gap Analysis Run 2 — 2026-04-25 21:30Z

**Context:** General-purpose gap analysis against a professional consulting firm's software implementation practice. Evaluated across three engagement types: (1) discovery/advisory — research, current-state assessment, recommendations, roadmap; (2) greenfield implementation — full build from scratch; (3) feature enhancement — adding to an existing client-owned system. Specific lens: the suite is designed to mitigate the risks of AI-accelerated workflows and apply adversarial pressure to keep the agent honest and on task. Assessed which consulting roles and responsibilities have no corresponding review coverage.

**Suite state at time of run:** Same as Run 1 — eight domains, 32 existing gaps from Run 1 carried forward.

**Prior gap review:** All Run 1 gaps remain open unless noted. No suite changes were made between runs.

### Findings

#### AI-accelerated workflow gaps

These are the gaps most specific to this suite's stated purpose. An adversarial review process for human-written code has a different risk profile than one for AI-generated code. The agent introduces failure modes that human engineers do not.

**G-20 — No assumption surfacing mechanism across domains**

AI agents make assumptions constantly — about requirements, about what the client "probably" wants, about what constitutes standard practice, about what a library does. Most of these assumptions are never made explicit. They are baked silently into implementations, tests, and documentation. In a human workflow, a code review surfaces surprising choices for discussion. In an AI workflow, surprising choices look like confident, fluent code and are easy to miss.

No domain in the current suite asks: *what assumptions did the agent make, and are they correct?* This is the most pervasive AI-specific risk. Every domain review is implicitly checking outputs, but none is explicitly reconstructing and validating the premises behind those outputs.

In a consulting context, an unvalidated assumption is a change order waiting to happen. The client's understanding of what was agreed and the agent's interpretation of the spec will diverge silently if no one is looking for it.

**G-21 — No hallucination detection across domains**

AI agents confidently cite APIs that do not exist, invent package names, misremember library interfaces, and describe behaviors that are plausible but wrong. This is categorically different from a human writing incorrect code — the agent produces fluent, well-formatted, confident output that does not signal its own incorrectness.

The Quality Engineering domain requires tests to be falsifiable, which catches some hallucinated implementations at runtime. The Software Engineering domain checks correctness. But neither domain explicitly directs the reviewer to verify that referenced external components — libraries, APIs, services, third-party integrations — actually exist and behave as described. A hallucinated dependency is not a style issue; it is a project blocker.

For consulting work, hallucinated integrations discovered late in delivery are a scope and timeline crisis.

**G-22 — No context drift / consistency checking across domains**

AI agents working across long sessions or multiple sessions lose track of earlier decisions. An architectural choice made in session 1 may be silently contradicted in session 4. A constraint established in the spec may be forgotten by the time the relevant feature is implemented. Tests may be written against an earlier version of the interface than the implementation uses.

No domain currently asks: *are decisions made early in this project still reflected in the current state of the code?* This is distinct from a regression test — it is a coherence audit. In a consulting engagement where work spans weeks and multiple AI sessions, coherence drift is a first-class risk.

**G-23 — No dependency and API existence validation**

Distinct from hallucination detection in that this is checkable: does the package exist in the registry? Does the API endpoint exist and return the documented shape? Does the third-party service have the capabilities assumed? This should be an explicit checklist item, not an incidental catch.

**G-24 — QE: no test gaming detection**

An AI agent that writes both the implementation and the tests has an inherent conflict of interest. It will write tests that validate its own interpretation of the requirement, not tests that would catch if its interpretation was wrong. The existing QE falsifiability dimension asks whether tests would catch a *broken implementation*. This gap is different: it asks whether tests would catch a *correct implementation of the wrong requirement*. An agent that misunderstood the spec will often produce a consistent implementation-and-test-suite that passes completely while delivering the wrong thing.

This is the most dangerous AI-specific quality risk. A human engineer who misunderstands a requirement tends to ask a question. An agent produces a complete, passing solution.

**G-25 — Security: no AI-generated code anti-pattern review**

Large language models have documented tendencies toward specific insecure patterns: hardcoded credentials used as examples that persist into production, overly permissive CORS configurations, SQL concatenation that looks parameterized but is not, JWT verification that checks format but not signature, copy-paste of deprecated cryptographic functions. These patterns appear in AI output with higher frequency than in experienced human output because the model is pattern-matching on training data that includes insecure examples.

The Security domain currently reviews outputs against standard dimensions. It does not explicitly direct the reviewer to look for AI-specific generation anti-patterns. This should be a named checklist, not an implicit catch.

#### Consulting role and responsibility gaps

**G-18 — Requirements and Business Analysis domain missing**

The Business Analyst role bridges client needs and technical implementation. The SA review evaluates architectural soundness; the SO review evaluates spec compliance. Neither evaluates whether the spec itself was correct. In a consulting engagement, requirements are gathered from client interviews, existing documentation, and stakeholder workshops — all of which are lossy and subject to misinterpretation. AI agents working from these inputs will extrapolate, fill gaps, and make the spec more internally consistent than the client's actual intent.

No domain asks: do the requirements accurately reflect what the client needs? Are user stories testable and unambiguous? Are acceptance criteria written so that both the client and the implementation team would agree on whether they are met? Is there a traceability map from client need to implemented feature?

**G-26 — Change Management and Adoption domain missing**

A technically perfect solution that is rejected by end users is a failed consulting engagement. Change management covers: stakeholder communication, training materials, rollout planning, resistance identification, and adoption measurement. In consulting, adoption failure is a reputational risk — the client blames the firm for a system no one uses.

No domain in the current suite asks whether the deliverables enable successful adoption. For AI-accelerated workflows specifically, there is an additional risk: the agent has no model of organizational politics, change fatigue, or user resistance.

**G-27 — Knowledge Transfer and Handoff domain missing**

Consulting engagements end. The client must own and operate what was built, often with a team that was not present for the build. No domain evaluates whether the deliverables enable handoff: is the code understandable without the AI conversation history? Are architectural decisions documented in a way a new maintainer can act on? Are there onboarding materials for the client's engineering team?

For AI-accelerated workflows, this gap is acute. Code generated by an agent may be correct and functional but written in a style that reflects the agent's training rather than the team's conventions, making it harder for the client's engineers to maintain.

**G-28 — Client/Stakeholder Alignment domain missing**

The SO enforces spec compliance, but the spec is an artifact of a negotiation between the consulting firm and the client. As work progresses, the client's understanding of what was agreed and the firm's implementation may diverge without either side realizing it. Client expectations drift; the spec does not.

No domain regularly asks: would the client recognize this as what they asked for? Are demos and status updates accurately representing current system state? Are there unresolved ambiguities in the agreed scope that will surface as disputes at delivery?

**G-31 — Professional and engagement liability unowned**

Consulting firms carry professional liability: errors and omissions, breach of contract, intellectual property indemnification. No domain evaluates the firm's own exposure: are deliverables clearly scoped so that disputes about what was delivered can be resolved against a documented record? Is IP ownership of AI-generated code documented and agreed upon? Are there deliverables that could expose the firm to claims if they contain errors?

This is distinct from compliance (G-01), which covers the client's regulatory exposure. This covers the firm's own exposure as a service provider.

#### Engagement-type gaps

**G-29 — Discovery/Advisory: research quality and source validation unowned**

In a discovery or advisory engagement, the primary deliverable is analysis and recommendations. No domain evaluates the quality of the research underpinning those recommendations: are sources cited and verifiable? Are findings based on the client's actual situation or on generic best practices applied without validation? Are assumptions about the client's constraints made explicit and confirmed?

For AI-accelerated discovery work, this gap is critical. An agent conducting discovery analysis will produce confident, well-structured findings that may be based on pattern-matched generalizations rather than evidence from the specific client context.

**G-30 — Feature Enhancement: existing system compatibility and upgrade burden unowned**

When adding a feature to an existing client-owned system, the primary risks are: does the enhancement fit the existing codebase's patterns, conventions, and constraints? Does it create upgrade or maintenance burdens the client did not agree to (new dependencies, build toolchain changes, runtime version requirements)? Does it create technical debt in the existing system that will outlast the engagement?

**G-32 — SA: no integration architecture review**

For consulting engagements — especially greenfield implementations — the system must connect to what it needs to connect to: existing client systems, third-party services, authentication providers, data sources. No dimension in the SA review explicitly evaluates integration architecture: are integration points identified and designed? Are interface contracts with external systems documented? Are integration failure modes handled?

In AI-accelerated workflows, integrations are a hallucination risk. The agent will design integration patterns based on its training data, which may not reflect the specific versions, quirks, or constraints of the client's actual systems.

#### Documentation gap

**G-19 — Documentation fidelity domain missing**

AI agents generate documentation in parallel with code. This creates a specific risk: the documentation and the implementation are generated from the same prompt interpretation, so both can be consistently wrong in the same way. More commonly, documentation is generated once and the code changes; without a domain that owns documentation accuracy, the gap widens over time.

No domain currently asks: does the documentation accurately describe the system as it exists today? Are API contracts documented and correct? Do user guides match actual user flows? Are architectural diagrams current? In a consulting context, documentation is often a contractual deliverable — inaccurate documentation is a delivery failure.

### Decisions

**Decision — AI-workflow gaps (G-20 through G-25) should be incorporated as cross-cutting dimensions, not a separate domain.**
Each existing domain already reviews outputs in its area. Rather than creating an "AI Review" domain (which would be redundant), the AI-specific risks should be added as explicit named dimensions within the domains best positioned to catch them.

**Decision — G-18 (Requirements/BA), G-28 (Client Alignment), and G-31 (Engagement Liability) are critical for consulting use and require new domains.**
These three gaps represent consulting-specific failure modes with no current coverage.

**Decision — G-26 (Change Management) and G-27 (Knowledge Transfer) are high priority for greenfield and feature enhancement engagements, lower for discovery/advisory.**

**Decision — G-29 (Discovery/Advisory research quality) requires a domain or pre-engagement checklist before AI-accelerated discovery work.**

**Decision — G-30 (Feature enhancement compatibility) and G-32 (Integration architecture) can be addressed as dimensions within SA and SE, not new domains.**

**Decision — G-19 (Documentation fidelity) warrants a dedicated domain.**
Documentation accuracy is a contractual concern in consulting, a knowledge transfer concern at handoff, and a correctness concern in AI-accelerated workflows where docs and code drift from the same initial prompt.

### Suite changes made as a result of this run

None. All findings logged. Changes to domain files should be made in separate commits and referenced in Run 3.

---

## Gap Analysis Run 1 — 2026-04-25 20:00Z

**Context:** Initial gap analysis. Evaluated against two project types: (1) mission-critical software project with reputational, legal, and business-continuity stakes; (2) speculative/exploratory project that may become a product or business. Prompted by question: how thorough is this suite for a board-level presentation?

**Suite state at time of run:** Eight domains — Quality Engineering, UX, Security, Platform Engineering, Solution Architect, Solution Owner, Software Engineering, Data Engineering. Platform Engineering expanded to cover DevSecOps, infrastructure, and observability. Pre-commit hooks added as PE dimension 10.

### Findings

#### Missing domains

**G-01 — Compliance and Legal (mission-critical: Critical)**

No domain evaluates regulatory exposure. For any project handling personal data, financial transactions, healthcare information, or operating in a regulated industry, compliance is a distinct failure mode from security. GDPR applies to personal data collection from EU residents even in a prototype. PCI-DSS, HIPAA, SOX, ADA/WCAG legal mandates, and OSS license compliance all represent categories of legal and financial liability that none of the existing domains own. Security asks whether data can be exfiltrated; Compliance asks whether you had the right to collect it, whether you stored it correctly, and whether you can prove it to a regulator.

*For speculative projects:* reduced but not zero. Establish the minimum floor (what data are you collecting, under what legal basis) and defer the full apparatus.

**G-02 — Performance and Scalability (mission-critical: Critical)**

No domain evaluates whether the system performs under real load. Load testing, stress testing, performance budgets, latency SLAs, scalability projections, and capacity planning are unowned. A system that is functionally correct but unusable at production scale has failed.

*For speculative projects:* defer entirely. You do not yet know your load profile or whether the thing is worth scaling.

**G-03 — Privacy (mission-critical: Critical)**

Listed as a candidate domain in the suite README but not implemented. Privacy is distinct from Security. Security asks whether data can be exfiltrated; Privacy asks whether it should be collected in the first place, how long it is retained, who can access it, whether consent was properly obtained, and whether subjects can exercise rights (access, erasure, portability).

*For speculative projects:* medium priority. Know what you are collecting and why before you collect it.

**G-04 — Operational Readiness (mission-critical: Critical)**

No domain asks whether the team can operate the system in production. Runbooks, incident response procedures, on-call coverage, escalation paths, rollback plans, and deployment checklists are unowned. A system can pass every technical review and fail in production because no one documented how to restart the service.

*For speculative projects:* low priority. "How do we restart this" is sufficient.

**G-05 — Delivery Governance (mission-critical: Critical)**

The suite is a quality process, not a delivery process. No domain tracks whether the project is on time and on budget, flags milestone slippage, or forces tradeoff decisions when timeline pressure appears. The Solution Owner prevents scope creep at the feature level but has no mechanism for tracking engineering cost or surfacing delivery risk early.

*For speculative projects:* medium priority, different character. Replace milestone tracking with kill criteria and learning goals (see G-14, G-15).

#### Gaps within existing domains

**G-06 — Security: no threat modeling**

The current Security domain covers seven dimensions appropriate for a single-user web application. A mission-critical system requires formal threat modeling: enumerating assets, attack surfaces, threat actors, and mitigations systematically before implementation begins. Threat modeling finds architectural security flaws that a code review cannot catch.

**G-07 — Security: no authentication/authorization review**

The current domain does not evaluate whether users can access only what they are permitted to access. An authorization bypass that lets user A read user B's data would not be caught. For any multi-user system, auth/authz is the highest-impact security surface.

**G-08 — Security: no session management review**

Session lifecycle (creation, expiry, invalidation, fixation resistance) is unowned. Relevant for any system with authentication.

**G-09 — Security: no audit logging requirement**

No domain asks whether the system records who did what and when. For mission-critical systems in regulated industries, an audit trail is both a legal requirement and an incident response necessity.

**G-10 — Security: no data classification requirement**

No domain asks what data the system handles and whether it is handled appropriately for its sensitivity level. Classification (public, internal, confidential, regulated) is the prerequisite for proportionate controls.

**G-11 — Solution Owner: no budget dimension**

The SO enforces scope and prevents feature additions, but a feature that is in scope and takes ten times longer than estimated is a budget failure the SO would not flag. Effort estimation, burn rate, and budget variance are unowned.

**G-12 — Quality Engineering: no integration/contract testing mandate**

Unit tests and browser/end-to-end tests are covered. For systems with multiple components or third-party integrations, contract testing (Pact or equivalent) and integration testing are distinct concerns. A change to an upstream API contract that breaks the integration would not necessarily be caught.

**G-13 — Platform Engineering: DR dimension lacks RTO/RPO targets**

The existing DR dimension (PE-21) asks whether a plan exists and whether backups are verified. For mission-critical systems, this is insufficient. Recovery Time Objective and Recovery Point Objective should be defined, tested, and enforced — not just described.

#### Speculative project-specific gaps

**G-14 — No learning goals / validation structure domain**

A speculative project that is technically excellent but answers the wrong question has failed. No domain owns: what are we trying to learn, at what fidelity, and is the project structured to produce a valid answer? This is the most critical gap for speculative work — without it, the suite can tell you the software is well-built but not whether the exploration succeeded.

**G-15 — No kill criteria mechanism**

No domain defines stopping conditions. A speculative project without defined kill criteria can drift indefinitely, consuming runway without producing a decision. This is a governance failure unique to exploratory work.

**G-16 — No intentional technical debt tracking**

In speculative projects, taking on technical debt is sometimes the right call. The risk is debt accumulated unconsciously that compounds and makes pivoting harder. No domain distinguishes debt we chose from debt we accumulated, or tracks the former as a known liability.

**G-17 — Solution Architect: no pivot readiness dimension**

For speculative projects, the SA review should evaluate whether the architecture allows a pivot when the project learns something that changes direction. Current SA dimensions ask whether boundaries are clean; they do not ask whether the system is designed to change.

### Decisions

**Decision — Suite is strong for most portfolio projects as-is.**
For individual portfolio projects without regulatory exposure or production SLAs, the current eight domains provide coverage well above industry average. Gaps G-01 through G-13 are appropriate to defer until a project reaches production or handles real user data.

**Decision — G-14 and G-15 are high priority for the next speculative project.**
Before using this suite on an exploratory project, add lightweight mechanisms for learning goals and kill criteria.

**Decision — Security domain expansion (G-06 through G-10) is high priority if the suite is used for any multi-user or regulated project.**
The current Security domain is calibrated for a single-user local tool. It should be clearly marked as such and expanded before being applied to a system with authentication, multiple users, or regulated data.

**Decision — G-02 (Performance) and G-04 (Operational Readiness) are deferred indefinitely for portfolio projects.**
These gaps are real for production systems. They are not relevant to the current project context.

### Suite changes made as a result of this run

None. All findings logged as open gaps. Changes to domain files should be made in separate commits and referenced in the next run entry.
