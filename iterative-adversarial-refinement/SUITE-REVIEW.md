# IAR Suite Meta-Review

The IAR suite is itself a software artifact. Like any artifact it has a specification (the VSDD and VDD methodology documents), a design (the domain structure, dimensions, and supplement architecture), and an implementation (the domain prompt files, README, and gap analysis log). The adversary should apply to the suite the same pressure it applies to projects under review.

This file logs adversarial review runs of the suite itself. The primary lens is VDD-IAR Alignment — governing doc compliance, process fidelity, and structural integrity. Cross-domain observations from QE and SE are included where they bear on the suite's fitness for purpose.

Governing references:
- VSDD whitepaper: https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00
- VDD whitepaper: https://gist.github.com/dollspace-gay/45c95ebfb5a3a3bae84d8bebd662cc25
- Apprentice-onboarding: https://github.com/Navigators-Guild/apprentice-onboarding
- Crosslink: https://github.com/forecast-bio/crosslink

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
**Artifacts reviewed:** QUALITY-ENGINEERING-REVIEW.md, UX-REVIEW.md, SECURITY-REVIEW.md, PLATFORM-ENGINEERING-REVIEW.md, SOLUTION-ARCHITECT-REVIEW.md, SOLUTION-OWNER-REVIEW.md, SOFTWARE-ENGINEERING-REVIEW.md, DATA-ENGINEERING-REVIEW.md, VDD-IAR-ALIGNMENT-REVIEW.md, lang/rust.md, lang/javascript-typescript.md, lang/browser-app.md, lang/cli.md, README.md, GAP-ANALYSIS-LOG.md.
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
