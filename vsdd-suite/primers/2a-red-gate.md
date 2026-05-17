# Session Primer: Red Gate (VSDD Phase 2a)

Use this prompt at the start of a Phase 2a session — after the layer plan (`TODO.md` or the crosslink issue hierarchy for Phase 2+ projects) is complete and the layer is opened. The output of this session is a set of failing tests committed to the working tree — the "Red Gate" state that Phase 2b implementation will turn green.

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
