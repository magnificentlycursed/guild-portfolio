# 2026-04-27 Suite Reviews

## Review 18 — 2026-04-27 23:30Z

**Scope:** Generalist adversarial pass — no domain framework, fresh pressure across all artifacts. Read: all 14 role domains, 2 meta domains, 5 primers, DOMAIN-INDEX.md, README.md, SUITE-REVIEW.md, governing documents. Triggered by user request following Reviews 16 and 17.

**Lens:** What did the specialist passes miss? Cross-cutting consistency, structural integrity, self-consistency after the phase renumbering, coordination link format compliance.

---

### Resolved

**Finding 1 — Implementation primer phase labels stale after Review 16 renumbering.**

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

## Review 17 — 2026-04-27 23:00Z

**Scope:** Second VDD-IAR philosophy and methodology alignment pass. Read: VSDD whitepaper, original VDD whitepaper, apprentice-onboarding `02-the-methodology/01-how-we-build.md`, CLAUDE.md. Evaluated governing standard compliance in both meta domain files and completeness of VDD-IAR Alignment dimensions against VSDD Phase 4 (Feedback Integration) requirements. Triggered by: user request following Review 16.

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

## Review 16 — 2026-04-27 22:00Z

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

## Review 15 — 2026-04-27 21:00Z

**Scope:** Full adversarial pass. Evaluated: all 14 role domains, 2 meta domains, 5 primers, 4 lang supplements, DOMAIN-INDEX.md, README.md, GAP-ANALYSIS-LOG.md, SUITE-REVIEW.md, CHANGELOG.md. Triggered by: completion of Review 14 resolution pass.

**Lens:** Does the current suite meet its own governing standard? What did the Review 14 fixes introduce?

---

### Resolved

**Finding 1 — Regression check ordering wrong in all five domains added in Review 14 resolution pass.**

All five regression checks added to TW, Localization, Accessibility, Privacy, and Performance Engineer were inserted after the Coordination paragraph instead of before it. The governing standard (element 5 order: Classification → Regression check → Coordination → Sycophancy → Lang supplement) was violated in all five files simultaneously by the fix that resolved Finding 6 from Review 14. The ordering matters: the regression check scopes the whole review (prior layers always in scope) before the reviewer reads what to flag to other domains.

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

## Review 14 — 2026-04-27 20:00Z

**Scope:** Full adversarial pass against the suite-development primer as governing standard. Evaluated: all domain files (14 role, 2 meta), all session primers, all lang supplements, DOMAIN-INDEX.md, README.md, GAP-ANALYSIS-LOG.md, SUITE-REVIEW.md, CHANGELOG.md.

**Lens:** Does every artifact meet its own governing standard? What would a reviewer following these prompts get wrong?

**Suite state:** 14 role domains (8 core, 6 extended), 2 meta domains, 5 session primers, 4 lang supplements, DOMAIN-INDEX.md. All content produced or reorganized in the prior session.

---

### Resolved

**Finding 1 — TECHNICAL-WRITER-REVIEW.md supplement reference points to nonexistent content.**

The supplement line read "Consult `../../supplements/` for language-specific documentation tooling (e.g., `rustdoc`, TypeDoc, JSDoc, Sphinx)." No Technical Writer section exists in any supplement file. The governing standard's coverage table explicitly marks this as a gap (G-84). A reviewer following this instruction consults supplements/ and finds nothing — then has no basis to either apply or dismiss the section.

**Resolution:** Reworded to acknowledge the gap explicitly, reference G-84, and state that the supplement section applies once written.

---

**Finding 2 — LOCALIZATION-REVIEW.md supplement reference contradicts governing standard coverage table.**

The supplement line read "Consult `../../supplements/` for language-specific i18n library recommendations and conventions." The suite-development.md coverage table marked Localization as "Language-agnostic" — a contradiction. The domain body references `Intl.NumberFormat`, `Intl.PluralRules`, and `toLocaleDateString()` — JS/TS APIs — and these are not covered anywhere in the supplements.

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

**Finding 6 — Five domains added after Review 13 are missing the "Regression check:" paragraph.**

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

The gap analysis runs section has a `## Gap Analysis Runs` header that introduces and contextualizes the runs. The meta-reviews (Reviews 11–13) have no equivalent section header — they follow the intro directly under `---`. This creates structural asymmetry and makes the file's organization harder to parse.

**Resolution:** Added `## Suite Meta-Reviews` header before Review 14 (this entry).

---

**Finding 10 — No governing standard format definition for gap analysis run entries.**

The `suite-development.md` "SUITE-REVIEW.md discipline" section stated that every non-trivial change requires an entry and listed what counts as non-trivial — but gave no format for what a gap analysis run entry should contain. A new contributor writing a run entry has no template and no completeness criteria.

**Resolution:** Added a numbered format definition (7 elements: header, context, scope, new gaps, addressed gaps, dismissed gaps, suite changes made) to the "SUITE-REVIEW.md discipline" section.

---

**Finding 11 — UX dim 7 does not reference the Accessibility domain.**

UX dim 7 covers a floor-level accessibility check (labels, contrast, semantic HTML, axe pass). The Accessibility domain covers this at depth (screen reader testing, cognitive accessibility, dynamic announcements, ARIA correctness, zoom/reflow). A reviewer running only UX would not know that a dedicated deeper domain exists.

**Resolution:** Added a sentence to UX dim 7 directing deeper coverage to the Accessibility domain.

---

## Review 13 — 2026-04-27

**Scope:** Full suite pass against the suite-development primer as governing standard. Evaluated: all domain files, both meta domains, all session primers, DOMAIN-INDEX.md, README, supplements/ supplements, GAP-ANALYSIS-LOG, SUITE-REVIEW.md, CHANGELOG.md.

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

**Resolution:** Added SUITE-REVIEW.md Review 13 entry (this entry) and a CHANGELOG.md entry covering the full session scope.

---

**G-06, G-19, G-27 status not updated in GAP-ANALYSIS-LOG.**

G-06 (Security: no threat modeling) was addressed by adding the `## Threat Model` section to SECURITY-REVIEW.md. G-19 (Documentation fidelity domain missing) and G-27 (Knowledge Transfer and Handoff domain missing) were both addressed by TECHNICAL-WRITER-REVIEW.md. All three remained Open in the registry.

**Resolution:** Updated all three to Addressed with date 2026-04-27.

---

**Technical Writer lang supplement gap not in registry.**

The suite-development.md coverage table explicitly marked Technical Writer as a gap (no JS/TS or Rust supplement sections for documentation tooling). This gap existed in the primer's table but had no GAP-ANALYSIS-LOG entry — no tracking, no ID, no status.

**Resolution:** Added G-84 to the gap registry.

---

## Review 12 — 2026-04-27

**Scope:** Full adversarial pass across all domain templates and supplements/ supplements. Session primed with `prompts/spec-crystallization.md` (adversarial posture: assume the spec is incomplete; find what is missing) and `prompts/decomposition.md` (push back on dimensions that are too large, too vague, or that mix concerns). Governing docs used as the DESIGN.

**Lens:** What slop would this suite fail to catch? Every domain reviewed for production-critical gaps — not process compliance, but defect classes that would reach users undetected. Suite alignment against VSDD reviewed separately. Open gaps from GAP-ANALYSIS-LOG raised and resolved where appropriate.

---

### Quality Engineering

**QE: No coverage threshold in the base domain — CRITICAL**

The base QE domain has no coverage threshold requirement. Dim 13 (quality gates) asks whether thresholds are enforced; it does not state what the thresholds should be. A project with 15% coverage and a passing CI run clears QE review. Coverage thresholds exist only in `supplements/rust.md` (80% minimum / 100% public API). JS/TS, Python, Go, and any other language project has no threshold.

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

Dim 6 reads: "If the application controls access to actions or data: are authentication and authorization checks present at the right boundaries?" That is one question covering the single most dangerous attack surface in any multi-user application. G-07 (no auth/authz review) has been open since Review 1 and the dim 6 addition is inadequate as a resolution. A real auth/authz review asks:

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

These two gaps have been registered as High priority since Review 2 (2026-04-25). They remain completely unaddressed. For a suite designed specifically for AI-accelerated development, these are the highest-impact gaps.

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

## Review 11 — 2026-04-27

**Scope:** All domain template files, supplements/ supplements, README.md, GAP-ANALYSIS-LOG.md.
**Artifacts reviewed:** QUALITY-ENGINEER-REVIEW.md, UX-REVIEW.md, SECURITY-REVIEW.md, PLATFORM-ENGINEER-REVIEW.md, SOLUTION-ARCHITECT-REVIEW.md, SOLUTION-OWNER-REVIEW.md, SOFTWARE-ENGINEER-REVIEW.md, DATA-ENGINEER-REVIEW.md, VDD-IAR-ALIGNMENT-REVIEW.md, supplements/rust.md, supplements/javascript-typescript.md, supplements/browser-app.md, supplements/cli.md, README.md, GAP-ANALYSIS-LOG.md.
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

But the analogy doesn't hold. The suite's "layers" are not feature layers in a deliverable — they are methodology iterations driven by real-world use. Review 1 analyzed the suite against mission-critical contexts. Review 4 analyzed it against the guild apprentice-onboarding. Review 7 analyzed it against VSDD. Each review is analogous to a "new project type reveals new gaps" event. You cannot layer-decompose the discovery of requirements that didn't exist until you had real projects to review.

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

Every other domain has: "**Language and interface supplement:** Consult `supplements/` for the supplement matching the project's primary language..." The VDD-IAR Alignment domain does not, because process compliance is language-agnostic. This is correct behavior, but it should be stated explicitly rather than absent silently — a reviewer running VDD-IAR Alignment after all other domains might notice the missing instruction and wonder if it was overlooked.

**Resolution:** Add a note to VDD-IAR Alignment that language/interface supplements do not apply to this domain (process compliance is language-agnostic). Minor.

---

### Hallucinated

*(none)*

**Summary:** The suite's most significant structural gap — absent Phase 1 and Phase 2 session primers — is resolved by this run. The VSDD purity boundary map gap is resolved (SA dim 12, JS/TS SA section). The README now surfaces the full VSDD pipeline context and governing references. Remaining deferred items are known limitations appropriate to the suite's current maturity level and project context. MVR signal: a second pass is unlikely to produce new findings of comparable significance.

---

## Review 10 — 2026-04-27

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
## Review 9 — 2026-04-27

**Context:** Full adversarial roast of the suite — all domain templates reviewed for production slop that would pass undetected, plus suite alignment against governing docs and prompt review. Session primed with `prompts/spec-crystallization.md` and `prompts/decomposition.md`. User instruction: "I expect perfection. Any findings in gap-analysis-log are fair game to raise and resolve."

**Suite state at time of run:** Nine domains. SUITE-REVIEW.md established. Session primers created. 57 gaps registered before this run.

### Findings and resolutions

**QE (G-58–60):** Coverage threshold absent from base domain (any non-Rust project with 10% coverage passed). Mutation testing absent (100% coverage with wrong assertions passes all dims). Flaky test failure modes not named. All three addressed in QE dims 2, 5, 13.

**Security (G-61–64):** Secrets-in-logs not covered (dim 4 only checked source control). Auth/authz dim 6 was a single-line placeholder for the most critical attack surface in multi-user apps — strengthened with six sub-questions. Prototype pollution absent from JS/TS supplement. Dependency confusion attack not named in Security or PE.

**UX (G-65–67):** Loading states and async failure recovery entirely absent from a domain that reviews user-facing feedback patterns. Keyboard focus trap not named despite being WCAG 2.1 Level A. Destructive action gate absence not distinguished from gate quality — dim split into 12 (gate existence) and 13 (gate quality).

**SE (G-68–69):** Flag argument (boolean trap) not named as a function design failure. Primitive obsession not named as a type safety failure.

**SA (G-70–71):** Memory leaks and event listener lifecycle absent from a domain evaluating state management — production failure that tests don't catch. Circular dependency detection absent from JS/TS supplement.

**DE (G-72–73):** Schema evolution dim too thin — one question, no migration testing, no rollback, no forward-compat. Data volume limits entirely absent.

**PE (G-74):** DR dim accepted "documented" as "tested" — distinguished and required test records with dates.

**Suite structural (G-75–77):** VDD-IAR Alignment sequencing added to each layer gate close, not only final merge. G-20/21/23 (assumption surfacing, hallucination detection, dependency validation) partially addressed as explicit instructions in QE, SE, SA review prompts. Sycophancy check rewritten for QE, Security, SA, SE with domain-specific failure modes.

**Prompt gaps (G-78–79):** spec-crystallization.md added project type framing (user-facing app / CLI / library / infrastructure / research). decomposition.md corrected: crosslink replaces TODO.md in Phase 2+ (not supplemented by it); accountability principle separated from tool reference.

**VDD-IAR Alignment dim 2:** Added note that TODO.md (Phase 1) is replaced by crosslink (Phase 2+) — not maintained in parallel.

**Remaining open:** G-34, G-36, G-54, G-55, G-57. G-20/21/23 partially resolved; full resolution requires a dedicated cross-cutting mechanism not yet designed.

---

## Review 8 — 2026-04-27

**Context:** Meta-adversarial review — IAR suite applied to itself. Prompted by a request to apply the adversary to the suite using governing docs as context, update the README to reflect suite evolution, and add session priming prompts for methodology execution.

**Suite state at time of run:** Nine domains. VDD-IAR Alignment with 11 dimensions + program phase context. Red Gate enforced in dims 4 and QE dim 2. TDD proxy indicators in QE dim 14. Governing references in VDD-IAR Alignment. 55 gaps registered before this run.

**Governing references consulted:** VSDD whitepaper, VDD whitepaper, apprentice-onboarding, crosslink, chainlink (as full-text content).

**Key framing:** IAR suite fills VSDD Phase 4 (Adversarial Refinement). VDD-IAR Alignment evaluates whether the adversary ran with integrity. This run applied that same evaluation to the suite itself.

### Findings

**G-56 — VSDD purity boundary map unowned (High)**

VSDD requires a verification architecture that identifies the pure/deterministic core and the effectful shell. This separation enables unit testing without mocking, formal verification of pure functions, and clear testability boundaries. SA dim 1 (separation of concerns) touched on layering but did not enforce the purity concept. No language supplement named it.

*Decision:* Add SA dim 12 (VSDD purity boundary map). Add SA section to JS/TS supplement (the only supplement missing one) with purity boundary, module organization, state flow, and event handler coupling dimensions.

**G-56 addressed** — SA dim 12 added. `supplements/javascript-typescript.md` SA section added.

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

## Review 7 — 2026-04-27

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

**supplements/rust.md gaps from claude.md**

The claude.md governing reference specified cargo-deny, cargo-vet, stricter clippy lint configuration, and coverage thresholds (80% minimum / 100% public API). None were in supplements/rust.md.

*Decision:* Add to supplements/rust.md with sourcing note (claude.md, may be superseded). Applied in Security (cargo-deny, cargo-vet), Platform Engineering (cargo-deny, cargo-vet, coverage enforcement), Quality Engineering (coverage thresholds), and Software Engineering (clippy lint configuration).

### Suite changes made as a result of this run

**G-53 addressed** — VDD-IAR Alignment dim 1 expanded with VSDD Phase 1 spec completeness criteria.
**G-54 registered** — Four-dimensional convergence gap logged as Open. Context-dependent: low for Phase 1–3, high for Phase 4+.
**G-55 registered** — Formal hardening gap logged as Open. Context-dependent: low for Phase 1–3, critical for Phase 4+ and mission-critical.
**Dim 11 added** — VDD-IAR Alignment dim 11 (issue tracking compliance) and program phase context section added.
**SO dim 9 updated** — Phase-appropriate tool introduction language added.
**QE dim 2 updated** — Red Gate language added.
**VDD-IAR Alignment intro updated** — IAR-as-adversary framing and governing references section added.
**supplements/rust.md updated** — cargo-deny, cargo-vet, clippy lint config, coverage thresholds added across QE, Security, SE, PE sections.

**Remaining open:** G-34, G-36, G-54, G-55. G-54 and G-55 are context-dependent; low severity for current portfolio work.

---

## Review 6 — 2026-04-27

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

