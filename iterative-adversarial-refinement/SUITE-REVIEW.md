# IAR Suite Meta-Review

The IAR suite is itself a software artifact. Like any artifact it has a specification (the VSDD and VDD methodology documents), a design (the domain structure, dimensions, and supplement architecture), and an implementation (the domain prompt files, README, and gap analysis log). The adversary should apply to the suite the same pressure it applies to projects under review.

This file logs adversarial review runs of the suite itself. The primary lens is VDD-IAR Alignment — governing doc compliance, process fidelity, and structural integrity. Cross-domain observations from QE and SE are included where they bear on the suite's fitness for purpose.

Governing references:
- VSDD whitepaper: https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00
- VDD whitepaper: https://gist.github.com/dollspace-gay/45c95ebfb5a3a3bae84d8bebd662cc25
- Apprentice-onboarding: https://github.com/Navigators-Guild/apprentice-onboarding
- Crosslink: https://github.com/forecast-bio/crosslink

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
