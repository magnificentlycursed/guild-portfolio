# Quality Engineering Review

This review is part of the [Iterative Adversarial Refinement (IAR)](README.md) suite. It may be run independently or alongside other domains. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

The purpose of this review is to evaluate the quality system as a whole: not just whether bugs exist, but whether the testing strategy, coverage, tooling, and gates are structured to catch defects reliably and repeatedly. Quality Engineering owns the test architecture and the confidence it produces. A passing test suite that would not catch a broken implementation is a quality failure.

## Current Review Prompt

**Scope:** Whole application by default. If a scope is provided (e.g., a specific feature or set of changed files), focus primary analysis there — but regression checks always cover the entire application.

Read DESIGN.md first for context on the project's intended scope, constraints, and feature set. Then read all source files, test files, HTML, CSS, and config. Apply every standard dimension below as a floor — add others as appropriate to the current state of the app. There is no restriction on what can be flagged.

For each finding, cite file and line number. Classify as **resolved** (fix applied this review), **deferred** (scheduled for a specific layer, reason given), **dismissed** (no action taken, rationale required), or **hallucinated** (the adversary invented a problem that does not exist — push back is warranted. Consistent hallucinated findings are the maximum viable refinement signal: real issues have been exhausted).

Regression check: verify that all previously-working features still work. Prior layers' acceptance criteria are always in scope. A change to one part of the app can silently break another. A bug that was always present is still a bug.

**Coordination:** Flag any findings that should be surfaced to [UX-REVIEW.md](UX-REVIEW.md), [SECURITY-REVIEW.md](SECURITY-REVIEW.md), [PLATFORM-ENGINEERING-REVIEW.md](PLATFORM-ENGINEERING-REVIEW.md), [SOLUTION-ARCHITECT-REVIEW.md](SOLUTION-ARCHITECT-REVIEW.md), or [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md). If this review suggests the need for a new IAR domain, log it as a finding.

**Sycophancy check:** If the agent agreed with every decision reviewed in this domain without challenge, treat that as a finding. An AI agent that validates every choice it helped produce is not providing adversarial review — it is confirming its own work. Flag any area where a significant decision went unquestioned but warranted scrutiny.

**Language and interface supplement:** Consult `lang/` for the supplement matching the project's primary language (e.g., `rust.md`, `javascript-typescript.md`) and interface type (e.g., `cli.md`, `browser-app.md`). Apply the **Quality Engineering** section from the relevant supplement files in addition to the standard dimensions below.

**Domain boundary:** QE owns the test system — whether tests exist, whether they are structured to catch defects, and whether coverage reflects genuine confidence. SE owns the code — whether the implementation is correct, well-named, and well-structured. When QE finds a logic error in code that has no test for it, flag the missing test here. SE flags the bug. Both findings are valid and non-duplicative: the missing test is a quality failure independent of whether the bug gets fixed.

## Standard Evaluation Dimensions

1. **Acceptance criteria** — Are all criteria from DESIGN.md actually met by the implementation, not just implied? Trace each feature to its test coverage.
2. **Test falsifiability** — Would each test catch a broken implementation? Could any test pass against wrong code? A test that cannot fail on a defective implementation has no value.

   **Red Gate:** A test that passes against an empty function body or a trivially wrong stub implementation was not written first. Evaluate whether each test would have failed before its corresponding implementation existed. A test suite where every test passes against `return null` or `return undefined` is a quality failure regardless of when the tests were written. This is corroborating evidence for VDD-IAR Alignment dim 4 — if tests could not have been written before the implementation (because they would have passed immediately), the Red Gate was not enforced.
3. **Test selector and assertion strength** — Are selectors, matchers, and assertions tight enough to fail on a broken implementation? Vague assertions (e.g., checking presence but not content) are a quality gap.
4. **Coverage meaningfulness** — Does coverage reflect genuine confidence, or are covered lines trivially exercised? Are branches, edge cases, and error paths tested, not just happy paths?
5. **Test architecture** — Is the test suite structured for maintainability? Are tests independent? Do they share state in ways that could cause order-dependent failures?
6. **Validation gaps** — What inputs slip through? What edge cases are untested? What happens at boundaries?
7. **Logic errors** — Are there bugs, off-by-one errors, or incorrect assumptions in the core logic?
8. **Dead code** — Any exported or declared code with no call sites?
9. **Unused dependencies** — Any direct dependencies not imported or used in the project?
10. **Dependency versions** — Are versions appropriate and not significantly outdated?
11. **Security surface** — Is user content rendered safely? Are user-supplied inputs validated before storage or output? Is data loaded from storage runtime-validated? Any new CVEs in dependencies? (See language and interface supplement for language-specific tooling.)
12. **Regression coverage** — Does every bug logged in the review log have an identifiable regression test? Flag any whose regression path is untested.
13. **Quality gates** — Are coverage thresholds, linting, and test runs enforced automatically? Are any quality checks manual-only that a passing CI run could miss?
14. **TDD proxy indicators** — Does the test suite exhibit the structural characteristics of test-first development? IAR cannot directly observe when tests were written, but TDD leaves a fingerprint in the artifact. Evaluate:
   - *Interface focus:* Do tests call the implementation at its public interface, or do they reach into internal details? Test-first forces interface-first design — the caller (the test) must exist before the implementation, so the interface must be defined first.
   - *Failure specificity:* Would each test have failed against an empty or trivially wrong implementation? A test that could pass before the function existed was not written first. A test suite where every test would pass against `return null` is a quality failure regardless of when it was written.
   - *Behavioral naming:* Are tests named for expected behavior ("returns null for empty input") rather than code structure ("tests the validation function")? Test-first produces behavior-named tests because the test describes intent before the code expresses it.
   - *Branch distribution:* Does branch coverage look earned — tests for each case implying the case was considered before it was handled — or does it look bolted on, with one long happy-path test covering most lines and separate small tests added to hit missed branches?
   - *Absence of implementation coupling:* Do refactors that preserve behavior break tests? If they do, tests are bound to implementation, not behavior — a sign they were written after the fact to match existing code rather than to specify required behavior.

   Findings here are corroborating evidence for VDD-IAR Alignment dim 4. A clean pass here alongside test-after commit patterns is a signal that tests were written quickly after implementation rather than long after; it does not clear the process finding.

---

Review entries are logged in `iterative-adversarial-refinement/QUALITY-ENGINEERING-REVIEW.md` inside the project being reviewed.
