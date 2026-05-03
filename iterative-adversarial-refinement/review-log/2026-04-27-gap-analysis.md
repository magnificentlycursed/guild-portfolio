# 2026-04-27 Gap Analysis Runs

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

## Gap Analysis Run 9 — 2026-04-27

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
