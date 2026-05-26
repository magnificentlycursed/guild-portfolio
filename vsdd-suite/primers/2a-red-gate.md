# Session Primer: Red Gate (VSDD Phase 2a)

**Frequency:** Per layer.

**Whitepaper alignment ([Review 79](../suite-development/review-log/2026-05-20-suite-review.md#review-79--2026-05-20-1730z) Finding 1):** the [VSDD whitepaper](https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00) names this step **"Test Suite Generation"** — the activity of authoring the layer's test surface. The suite's primer is named **"Red Gate"** to emphasize the suite-specific commit-discipline within Test Suite Generation: the failing-test state is committed as a distinct boundary before any Phase 2b implementation lands. Both names are used in the suite: "Test Suite Generation" is the canonical whitepaper-aligned activity name (used in cross-references that match the whitepaper); "Red Gate" is the suite's commit-discipline name (used in cross-references that emphasize the boundary commit). Choose by context — the broader activity is "Test Suite Generation"; the specific commit boundary within it is the "Red Gate".

Use this prompt at the start of a Phase 2a session — after the layer plan (`TODO.md` or the crosslink issue hierarchy for Phase 2+ projects) is complete and the layer is opened. The output of this session is a set of failing tests committed to the working tree — the "Red Gate" state (the commit boundary) within Phase 2a's Test Suite Generation activity that Phase 2b implementation will turn green.

Do not start Phase 2a without a layer plan. Tests written against undefined acceptance criteria produce a Red Gate that cannot be layer-gated.

---

## Prompt

You are helping write Red Gate tests for a software layer under the Verified Spec-Driven Development (VSDD) methodology. This is Phase 2a: Red Gate.

**Your posture:** Tests before code. For every acceptance criterion, write a test that currently fails because the feature does not exist. If you find yourself reasoning about implementation details rather than the behavior the test asserts, stop — the test specifies what the implementation must satisfy, not how it should be built.

**The Red Gate principle:** Before any test is written, ask: "What test would currently fail if this function did not exist?" Write that test. Run it. Confirm it fails. Do not implement to make it pass in this session — that is Phase 2b's job, against your committed Red Gate.

**DESIGN.md is the spec. The layer plan is the scope.** Both are binding. A test for behavior not in the spec is out of scope regardless of how obvious it seems. An acceptance criterion in the layer plan is a test commitment.

---

## Layer reference

*(Paste the current layer's acceptance criteria here, or reference the open crosslink issue.)*

---

## Phase 2a: Red Gate

1. For each acceptance criterion in the layer plan, write the corresponding test.
2. Run the test suite. Every new test must fail. A new test that passes against a stub or empty function body was not written first — revise it.
3. Confirm the failure reason is what you expect: the test fails because the feature does not exist, not because of a setup error.
4. **Commit the Red Gate state before Phase 2b begins.** A Red Gate that exists only in the working tree is not verifiable from the project history. The commit is the boundary between Phase 2a and Phase 2b — every file change after it is implementation. If implementation begins before this commit, the commit history cannot distinguish test-first from test-after, and VDD-IAR Alignment dim 4 cannot be verified.

**Driving questions for test writing:**

- What behavior does this criterion describe? Name the test for the behavior, not the function: `"rejects empty URL"`, not `"tests validateUrl"`.
- What input produces what output? State it explicitly in the test.
- What does this test assert against an empty function body? If it would pass — the test is wrong.
- Are boundary conditions represented? The edge case catalog in DESIGN.md is the source of truth for what inputs to test.
- What failure mode does this test protect against? If you cannot name it, the test may not be testing behavior.

**Red Gate anti-patterns to reject:**

- A test that only checks that a function was called (spy/mock-only assertion) — calls confirm invocation, not behavior
- A test that passes with `return null` or `return undefined` as the implementation
- A test named `"works correctly"` or `"functions as expected"` — names that do not describe behavior cannot be Red Gate tests
- A test that imports implementation internals rather than calling the public interface

---

## Completion criteria

Phase 2a is complete and ready to hand off to Phase 2b when:

1. Every acceptance criterion in the layer plan has a corresponding test
2. Every new test fails when run, and the failure reason confirms the missing-feature cause (not a setup error)
3. The Red Gate state is committed — the commit hash is the verifiable boundary between Phase 2a and Phase 2b
4. No implementation logic has been written this session — `2b-implementation.md` runs next against the Red Gate commit

## Verifiable git-history check ([Review 91](../suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) Finding 1)

The Red Gate commit is verifiable only when the layer-branch's commit history preserves it as a distinct boundary. The check has two shapes:

**Canonical (two-commit) shape — default:**

1. **First commit on the layer-branch** is Phase 2a-only: the new failing tests + any test-harness scaffolding required to run them. CI confirms RED (the new tests fail; pre-existing tests pass).
2. **Second commit on the layer-branch** is Phase 2b implementation: the same tests pass. CI confirms GREEN.
3. The Phase 2c refactor (if applied) lands as a third commit (or is annotated as `no refactor required` in [TODO.md](../templates/TODO-template.md) § Layer N Phase 2c per the Phase 2c primer).

The boundary commit is the audit-trail anchor: a reader looking at `git log` on the layer-branch sees Phase 2a → Phase 2b → Phase 2c as separate steps. [VDD-IAR Alignment Dim 4](../domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md) verifies this from the commit log without per-finding inspection.

**Single-commit deviation — operator-acceptance required pre-cycle:**

A project that combines Phase 2a + Phase 2b in one commit (the reference example pattern at [bookmark-cli-manual L1](../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-17-quality-engineer.md#review-1--2026-05-17-0325z) + [L2 commit `326e25d`](https://github.com/magnificentlycursed/guild-portfolio/blob/main/vsdd-suite-reference-examples/bookmark-cli-manual/TODO.md)) MUST document the deviation explicitly **before** the cycle begins:

- In the project's [`TODO.md`](../templates/TODO-template.md) § Layer N Phase 2c (or a dedicated § Phase 2a evidence-shape annotation), state: "Single-commit Phase 2a + Phase 2b deviation accepted by operator. Rationale: [specific reason — reference-implementation context; trivial layer with no meaningful inter-phase reviewer surface; etc.]. Red Gate failure evidence preserved at [location — sub-agent spawn output; CI run URL; etc.]."
- In the cycle's pre-cycle declaration per [`primers/3-review-session.md`](3-review-session.md) § Pre-cycle methodology check, name the deviation in the new Phase-2a-evidence-shape field (added at Review 91).

The deviation is acceptable when documented pre-cycle; it is a finding when surfaced post-hoc by [VDD-IAR Alignment Dim 4](../domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md) without prior documentation.

**Why this discipline is hard:** the pattern of "Phase 2a + 2b combined in single commit, then documented post-hoc when VDD-IAR Alignment surfaces it" recurred across [bookmark-cli-manual L1 (QE R1 F1)](../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-17-quality-engineer.md#review-1--2026-05-17-0325z) AND [L2 (VDD-IAR Alignment R4 F1)](../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-21-vdd-iar-alignment.md) despite the L1 PROCESS.md retrospective naming the lesson. The L2 recurrence is the same memory-feedback-insufficient pattern as [Review 90 Finding 1](../suite-development/review-log/2026-05-23-suite-review.md#review-90--2026-05-23-1200z)'s lettering-violation recurrence — codifying the rule in PROCESS.md alone does not propagate it forward to the next cycle.

**Escalation path (deferred per "earned by recurrence"):** if a third recurrence happens after this Review 91 codification — on any project where the operator did NOT pre-declare the single-commit deviation — escalate to a pre-commit hook scanning the layer-branch's first commit for the `tests/` + `src/` co-modification pattern that signals undeclared Phase-2a/2b consolidation. Mechanizing this against two recurrences is over-investment; the third-recurrence trigger parallels the lettering-violation hook proposal in [Review 90 Finding 1](../suite-development/review-log/2026-05-23-suite-review.md#review-90--2026-05-23-1200z).


## Three-audience lens

This Red Gate primer serves all three audiences of the [three-audience design principle](../suite-development/suite-development.md#three-audience-design-principle-review-80-finding-3) ([Review 80](../suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z) Finding 3; renamed in [Review 84](../suite-development/review-log/2026-05-21-suite-review.md#review-84--2026-05-21-1100z) Finding 4):

- **Suite developers** evolving this primer treat the prose as the methodology-authoring surface for Phase 2a Red Gate — changes here are methodology shifts requiring their own Review.
- **Suite users** running a session against this primer treat it as the canonical step-by-step for Phase 2a Red Gate on their own project; the completion criteria are what their next layer-gate or phase-close commit is checked against.
- **AI agents** loaded with this primer as cold-session context treat it as the spec for the session's authoring shape (file locations, classification vocabulary, the audit-trail entries this session produces); the primer's named artifacts + their schemas are the agent-API contract for what the session writes.

See [`../suite-development/suite-development.md`](../suite-development/suite-development.md) [§ Three-audience design principle](../suite-development/suite-development.md#three-audience-design-principle-review-80-finding-3) for the full discipline.
