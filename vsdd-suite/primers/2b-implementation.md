# Session Primer: Minimal Implementation (VSDD Phase 2b)

**Whitepaper alignment ([Review 79](../suite-development/review-log/2026-05-20-suite-review.md#review-79--2026-05-20-1730z) Finding 1):** the [VSDD whitepaper](https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00) names this step **"Minimal Implementation"** — the "Minimal" qualifier is load-bearing; Phase 2b's posture is "write only enough code to pass the failing tests; no scope creep beyond the Red Gate's surface." The suite's primer was previously titled just "Implementation"; this primer aligns to the whitepaper's canonical name.

Use this prompt at the start of a Phase 2b session — after `2a-red-gate.md` has been run, every layer test fails for the right reason, and the Red Gate state is committed. The output of this session is a passing test suite and a minimal working implementation that satisfies the layer's acceptance criteria — nothing beyond what the failing tests demand.

Do not start implementation without a committed Red Gate. An implementation session that begins before the Red Gate commit cannot be distinguished from test-after work by VDD-IAR Alignment dim 4.

---

## Prompt

You are helping implement a software layer under the Verified Spec-Driven Development (VSDD) methodology. This is Phase 2b: Implementation.

**Your posture:** Make failing tests pass — no more, no less. The Red Gate from Phase 2a is the contract; implementation has no scope beyond it. If you find yourself implementing behavior that no failing test asserts, stop — either add the test (with the retroactive-Red-Gate label, see below) or surface the gap as a spec defect.

**DESIGN.md is the spec. The Red Gate is the scope.** A feature not covered by a failing test is not in scope regardless of how obvious it seems.

---

## Layer reference

*(Paste the current layer's acceptance criteria and Red Gate commit hash here, or reference the open crosslink issue.)*

---

## Phase 2b: Implementation

Once the Red Gate is set, every new test is confirmed failing, and the Red Gate state is committed:

1. Implement to make failing tests pass — no more, no less.
2. Do not add tests during implementation. If you discover a missing test, note it; add it in a separate commit after the current feature is working, so the Red Gate record is clean. A retroactive test cannot satisfy the Red Gate (the implementation exists before the test fails), so log it as a **Red Gate deviation** in the commit message and review log: "retroactive Red Gate: [behavior name] — discovered during Phase 2b, test added post-implementation, confirmed passes against current implementation." This is a known limitation, not a workaround. Do not silently add retroactive tests without the label.

   The label extends to **post-MVR retroactive Red Gate** when Phase 5 (Formal Hardening) surfaces a missing test against already-shipped implementation-MVR code: use the same label but with a Phase 5 source qualifier — "retroactive Red Gate (Phase 5 source): [behavior name] — Surface [A|B|C|D] surfaced the gap; test added post-MVR; confirmed passes against current implementation." Distinguish from the Phase 2b-discovery case via the `(Phase 5 source)` qualifier; the discipline is the same (the test cannot retroactively satisfy the Phase 2a Red Gate), the visibility is greater (post-MVR discovery is a stronger audit signal than during-Phase-2b discovery).
3. Do not implement features not covered by a failing test. If a feature seems obviously needed but has no test, that is a spec gap — surface it rather than silently implementing it.
4. After each feature is complete, run the full test suite. No previously-passing test may begin failing. A regression requires a fix before moving to the next feature.

**Driving questions for implementation:**

- Which failing test am I currently making pass? Name it before writing code.
- Does this implementation do anything a test does not assert? If so, flag it — untested behavior is unverified behavior.
- Is this the minimal implementation that satisfies the test? Or is it anticipating future requirements not in the spec?
- Does any previously-passing test now fail? Stop and fix the regression.

---

## Completion criteria

A layer is implementation-complete and ready for IAR when:

1. All acceptance criteria from the layer plan have passing tests
2. All tests from the Red Gate phase pass, including regression tests from prior layers
3. The manual testing checklist from the layer plan has been executed by a human — automated tests do not substitute for human verification of interaction flows, error states, and "technically correct but wrong in context" failures
4. No implemented behavior exists that has no test covering it

The layer is ready for **IAR** (Phase 3), not merge. IAR runs next. Do not merge before the active domains reach MVR.


## Three-audience lens

This implementation primer serves all three audiences of the [three-audience design principle](../suite-development/suite-development.md#three-audience-design-principle-review-80-finding-3) ([Review 80](../suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z) Finding 3; renamed in [Review 84](../suite-development/review-log/2026-05-21-suite-review.md#review-84--2026-05-21-1100z) Finding 4):

- **Suite developers** evolving this primer treat the prose as the methodology-authoring surface for Phase 2b implementation — changes here are methodology shifts requiring their own Review.
- **Suite users** running a session against this primer treat it as the canonical step-by-step for Phase 2b implementation on their own project; the completion criteria are what their next layer-gate or phase-close commit is checked against.
- **AI agents** loaded with this primer as cold-session context treat it as the spec for the session's authoring shape (file locations, classification vocabulary, the audit-trail entries this session produces); the primer's named artifacts + their schemas are the agent-API contract for what the session writes.

See [`../suite-development/suite-development.md`](../suite-development/suite-development.md) [§ Three-audience design principle](../suite-development/suite-development.md#three-audience-design-principle-review-80-finding-3) for the full discipline.
