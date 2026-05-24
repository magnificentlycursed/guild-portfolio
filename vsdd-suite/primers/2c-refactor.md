# Session Primer: Refactor (VSDD Phase 2c)

Use this prompt after [Phase 2b](2b-implementation.md) has produced a green test suite for the current layer and the implementation commit is on disk. The output of this session is a refactored implementation that holds the same green test suite, committed as a distinct [Phase 2c](2c-refactor.md) commit before the layer enters [Phase 3](3-review-session.md) (IAR).

Phase 2c is the third step of the test-driven loop named in the [VSDD whitepaper](https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00): red → green → refactor. The suite previously folded refactor pressure into Phase 3 review feedback (and continues to — IAR findings against the implementation drive refactor work via [Phase 4](4-feedback-integration.md) routing). This primer covers the *intra-Phase-2* refactor step that the whitepaper specifies as part of the TDD loop itself, before any cold-context adversarial review.

**Phase 2c is optional but not invisible.** A layer may legitimately have no refactor work — the minimal implementation may already be clean. In that case the layer's record names Phase 2c as "no refactor required, implementation passes the refactor checklist as-landed" with a one-line rationale. A silent skip is a discipline gap; an explicit "no work needed" annotation is the discipline working.

---

## Prompt

You are helping refactor a software layer's implementation under the Verified Spec-Driven Development (VSDD) methodology. This is Phase 2c: Refactor. The [[Phase 2a](2a-red-gate.md) Red Gate](2a-red-gate.md) is committed; the [Phase 2b implementation](2b-implementation.md) is committed; every test in the layer's Red Gate is green; no previously-passing test from prior layers is failing.

**Your posture:** Improve the implementation's internal shape without changing what it does. Every behavioral assertion in the Red Gate still holds. Every test stays green. No test is added, no test is removed, no test changes scope. The refactor commit reads as a refactor — a Phase 3 SE reviewer should be able to diff Phase 2b → 2c and see only structural improvement, not new behavior.

**The Red Gate stays green at every step.** Run the test suite after each refactor — `cargo test`, `npm test`, `pytest`, or the equivalent. A refactor that turns a previously-passing test red is a refactor that changed behavior — back out and either (a) revert to the Phase 2b commit and re-attempt with a smaller scope, or (b) surface the behavior change as a spec gap (the implementation was wrong; the test was wrong; or the behavior is undefined). A refactor session does not silently change behavior.

**Primary failure mode:** scope creep into new behavior. A refactor that introduces a new feature, a new validation, a new error path, or a new test is no longer a refactor — it is a new Phase 2a/2b cycle hiding inside a Phase 2c commit. The Phase 3 SE reviewer cannot distinguish refactor from feature-add in the diff, and the Red Gate discipline is undermined.

---

## Layer reference

*(Paste the layer's acceptance criteria, the Phase 2a Red Gate commit hash, and the Phase 2b implementation commit hash here. The Phase 2c diff is read against the Phase 2b commit, not against the prior layer.)*

---

## Refactor scope

Phase 2c refactor work falls into the following categories. A refactor commit may exercise one or several; none is mandatory.

1. **Extract and name.** A repeated pattern becomes a named helper. A magic number becomes a named constant. A nested conditional becomes a guard clause with an explicit condition name. The test suite is unchanged.
2. **Collapse and inline.** A helper used in one place inlines. A one-line wrapper that adds no clarity is removed. A defensive check for a condition that cannot occur (per spec contract) is removed.
3. **Reshape data flow.** A mutable accumulator becomes a fold. A multi-step pipeline becomes named intermediate values. The data shape at each step is unchanged from the test's perspective.
4. **Surface the purity boundary.** Move pure logic out of an effectful function so it can be tested directly. The existing test suite continues to exercise the effectful entry point; new tests for the pure function are deferred to a follow-on Phase 2a if warranted.
5. **Align with the language's idioms.** A `for` loop becomes a comprehension where idiomatic. An early-return chain replaces a deeply nested conditional. The language supplement's style notes (e.g., `supplements/rust.md` SE dimensions) name what idiomatic looks like in this language.
6. **Apply named refactor rules from the language supplement.** If [`supplements/rust.md`](../supplements/rust.md) or [`supplements/javascript-typescript.md`](../supplements/javascript-typescript.md) or [`supplements/python.md`](../supplements/python.md) or [`supplements/bash.md`](../supplements/bash.md) SE dimensions name specific refactor rules (e.g., "prefer `?` over `match` for error propagation in [Rust](https://www.rust-lang.org/)"), this is the place to apply them.

Out of scope for Phase 2c — defer to a later phase:

- **New features.** Belong in a future layer's Phase 2a/2b cycle.
- **New validation, new error paths, new behavior under any condition.** A spec gap; surface to [Phase 1a+1b](1ab-spec-development.md) routing.
- **Performance optimization that changes algorithmic complexity.** Treat as a separate behavior contract — requires a new spec assertion (memory bounds, latency target) and a new Phase 2a test. Phase 2c may apply micro-optimizations that the test suite does not measure (e.g., remove an unnecessary clone) but does not change Big-O.
- **Test refactoring.** Tests are part of the Red Gate contract; their structure changes only when the spec changes. A test-suite refactor is its own Phase 2a/2c cycle (red → still-green refactor of test code).
- **Renaming a public API.** Belongs in Phase 1a+1b (the spec defines the API surface). A rename that breaks the test names is a behavioral change; the test contract includes the public API names.

---

## Driving questions

Work through these questions on the implementation. Stop refactoring when a "no" answer to question 1 would change.

1. Would a Phase 3 SE reviewer read this diff and call it a refactor, or call it a feature-add?
2. After this refactor, does the test suite still exercise every behavior the layer was supposed to implement?
3. Did I add a check or branch that no test asserts? If yes — either back out, or surface the behavior as a spec gap (one of: the spec was incomplete; the implementation was over-engineered; the new check is dead code).
4. Did I remove a check or branch? If yes — was that check defending against a condition the spec says cannot occur, or did I just silence a test I didn't understand?
5. Does the test suite take longer to run now? If yes — by enough to be worth knowing (≥ 2x)? If yes — log the change in the commit body.
6. Is the implementation easier to read for a Phase 3 reviewer who has never seen it before?

---

## Completion criteria

A Phase 2c refactor session is complete when:

1. The full test suite is green (`cargo test`, `npm test`, `pytest`, etc., per the project's language).
2. The diff against the Phase 2b commit contains no new tests, no removed tests, no test name changes, and no test-scope changes.
3. No new behavior path exists that is not exercised by an existing test (no dead code added).
4. The refactor commit is on disk with a message naming the refactor category from the list above (e.g., `Phase 2c: extract bookmark-format helper from add/list code paths`).
5. The layer's PROCESS.md / TODO.md entry annotates Phase 2c status: either the commit hash + one-line description of what was refactored, or `Phase 2c: no refactor required` with a one-line rationale.

After Phase 2c commits (or is explicitly skipped with annotation), the layer enters Phase 3 — open fresh chats per active domain and run the cold-context adversarial review per `3-review-session.md`. Phase 3 SE and SA reviewers will exercise the refactor's choices alongside the implementation's choices; a refactor that landed a clean shape under no test pressure is exactly the kind of choice IAR catches.

---

## Crosslink mode (Phase 2+ projects using crosslink)

In [crosslink](https://github.com/forecast-bio/crosslink) mode, Phase 2c is recorded as a session segment within the active layer's session:

```sh
# After Phase 2b commits and tests are green:
# (open a fresh chat with 2c-refactor.md primer; refactor; run the test suite)
cargo test                              # expect: green (same as Phase 2b)
git commit -am "Phase 2c: <refactor category> — <one-line description>"

# If no refactor work is needed for this layer:
crosslink issue comment "$L1" "Phase 2c: no refactor required — minimal Phase 2b implementation passes the refactor checklist as-landed." --kind decision

# Run the layer gate as the Phase 2c → 3 boundary (same gate as the Phase 2b → 3 boundary; the gate fires once per layer regardless of whether 2c committed):
crosslink swarm gate layer-1
```

The crosslink session does not need a dedicated `session work` re-attach for Phase 2c; the layer issue remains the focus.

---

## Manual mode

Same discipline; the boundary is tracked in `TODO.md` rather than crosslink. After Phase 2b implementation commits, open a fresh chat with this primer, refactor or annotate-no-refactor, commit (or record the no-refactor decision in `TODO.md` Layer N), run the test suite to confirm green, then proceed to Phase 3.


## Three-audience lens

This refactor primer serves all three audiences of the [three-audience design principle](../suite-development/suite-development.md#three-audience-design-principle-review-80-finding-3) ([Review 80](../suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z) Finding 3; renamed in [Review 84](../suite-development/review-log/2026-05-21-suite-review.md#review-84--2026-05-21-1100z) Finding 4):

- **Suite developers** evolving this primer treat the prose as the methodology-authoring surface for Phase 2c refactor — changes here are methodology shifts requiring their own Review.
- **Suite users** running a session against this primer treat it as the canonical step-by-step for Phase 2c refactor on their own project; the completion criteria are what their next layer-gate or phase-close commit is checked against.
- **AI agents** loaded with this primer as cold-session context treat it as the spec for the session's authoring shape (file locations, classification vocabulary, the audit-trail entries this session produces); the primer's named artifacts + their schemas are the agent-API contract for what the session writes.

See [`../suite-development/suite-development.md`](../suite-development/suite-development.md) [§ Three-audience design principle](../suite-development/suite-development.md#three-audience-design-principle-review-80-finding-3) for the full discipline.
