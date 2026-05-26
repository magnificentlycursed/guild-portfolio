# Session Primer: Formal Hardening (VSDD Phase 5)

**Frequency:** Per layer with per-surface conditionals (strategy named at DESIGN.md § Project intent per [G-162](../suite-development/FINDINGS-INDEX.md#g-162); learning-exercise + portfolio intents may skip; capstone + production intents require execution or explicit `not applicable` declaration).

Use this prompt after a layer has reached implementation-MVR (maximum viable refinement) via [Phase 3](3-review-session.md) IAR (Iterative Adversarial Refinement) (only Hallucinated findings across all active domains for the layer's intent-calibrated domain set). The output of this session is one or more **hardening artifacts** that produce stronger evidence of correctness than IAR cold-batch review alone: property-based tests for the spec's invariants, Mutation Testing of the existing test suite, Fuzz Testing for parser / input-boundary surfaces, and (where the spec's verification architecture named formal-proof candidates) proof harnesses for designated pure functions.

[Phase 5](5-formal-hardening.md) is optional at every intent tier, but the decision to skip is **explicit** per `domains/DOMAIN-INDEX.md` § Phase 5 / [Phase 6](6-convergence.md) strategy declaration ([G-162](../suite-development/FINDINGS-INDEX.md#g-162)). A capstone-intent or production-intent project's `DESIGN.md` § Project intent must include a one-sentence `**Phase 5 strategy:**` line declaring either `not applicable — <rationale>` or `planned — <named tooling and scope>`. This primer is for projects that declared `planned` — it walks the planned scope into concrete artifacts.

**Phase 5 sits AFTER Phase 3 IAR reaches implementation-MVR for the layer.** Running Phase 5 against a layer that hasn't passed IAR yet wastes the hardening effort — the implementation is still moving, so Mutation Testing measures a moving target and proofs target code that will be refactored. The order: Phase 3 implementation-MVR → [Phase 5 hardening](5-formal-hardening.md) → Phase 6 four-dimensional convergence gate (when Phase 5 closes).

---

## Prompt

You are helping harden a software layer that has reached implementation-MVR under the Verified Spec-Driven Development (VSDD) methodology. This is Phase 5: Formal Hardening. Your role is to produce evidence of correctness that is *qualitatively different* from cold-batch adversarial review — IAR proves the implementation passes the spec's tests; Phase 5 probes whether the spec's tests are themselves strong enough to detect realistic defects.

**Your posture:** The implementation-MVR signal from Phase 3 says "the cold adversary ran out of complaints against the spec + tests + code." Phase 5 asks the harder question: "do the tests actually exercise the behavior the spec asserts, or do they pass by accident?" A test suite that passes a green run can still have low mutation-test kill rate (the tests assert weak invariants); a parser with full unit-test coverage can still crash on inputs no unit test enumerated (Fuzz Testing surfaces those); a pure function that types check and tests-green can still violate the spec's invariants on inputs the tests didn't pick (property-based testing finds them). Phase 5 produces evidence at the level of the *test suite's strength*, not just the implementation's behavior.

**Primary failure mode:** Treating Phase 5 as a checklist of "ran the tool" rather than as a discipline of "designed the hardening to target the spec's named verification-architecture surface." A `cargo-mutants` run with a 90% kill rate against a test suite that has 200 lines of trivial assertions is much weaker evidence than a 70% kill rate against a test suite that exercises the spec's invariants directly. Phase 5 evidence is judged against the spec's verification architecture (`primers/1ab-spec-crystallization.md` § Verification architecture), not against the tool's default reports.

**Sycophancy check (per surface — the cognitive failure mode the AI session will exhibit at each):**

- **property-based testing.** The AI will write a property whose only assertion is that the function does not panic (or returns a value of the expected type) — a liveness property that holds for an empty implementation. The AI must instead express *the spec's named invariants* from DESIGN.md; a property test that passes against a stub is the Phase 5 equivalent of a [Phase 2a](2a-red-gate.md) test that passes against an empty function. The check: re-read each property's assertions and verify that mutating the implementation in a way the spec forbids would cause the property to fail.
- **Mutation Testing.** The AI will rationalize surviving mutants as "equivalent" without proof. Each surviving mutant within the evaluation scope must be addressed via one of: (a) genuinely behavior-equivalent (named in writing with the proof of equivalence), (b) the test suite is missing a falsifying test (add it with the **retroactive-Red-Gate (Phase 5 source) label** per `primers/2b-implementation.md` — the same label discipline extends to post-MVR discovery), (c) the spec has a gap the implementation correctly handled but the spec doesn't assert (route the surviving mutant to [Phase 4](4-feedback-integration.md) / [Phase 1a+1b](1ab-spec-development.md)), or (d) **unviable — mutation does not compile; not a behavioral signal** (e.g., a mutation that changes a `+` to a `-` in a string-concatenation expression, or a type-system-rejected mutation; cargo-mutants reports these separately from missed/caught). Unviable mutations are listed in the Phase 5 log with a one-line note for completeness but are not test-suite gaps. A "this mutant is equivalent, trust me" line in the Phase 5 log is itself a finding for Phase 3's next round on the layer.
- **Fuzz Testing.** The AI will declare "no crashes found" after a short Fuzz Testing run as evidence of correctness — but a short Fuzz Testing run produces evidence of *only what the budget covered*. The check: name the time budget (or input-count budget) elapsed, the corpus growth observed, and the coverage signal (line / branch coverage delta). A Fuzz Testing run that did not grow the corpus and did not increase coverage produced no new evidence — it confirmed the existing corpus. Fuzz Testing closure requires both budget exhaustion and a non-trivial coverage / corpus signal.
- **Proof Execution.** The AI will write a proof harness whose property is a tautology — `forall x: f(x) == f(x)` or similar — and report the proof as established. Each harness must establish a *non-trivial* spec-asserted property: the harness's stated property maps to a DESIGN.md invariant via the Phase 5 log's Proof Execution narrative. A harness whose property cannot be traced to a DESIGN.md sentence is itself a finding.

---

## Layer reference

*(Paste the layer's DESIGN.md § Verification architecture sub-section, the [Phase 2c](2c-refactor.md) commit hash if any, the Phase 3 final-round summary, and the project's `**Phase 5 strategy:**` line from DESIGN.md § Project intent here.)*

---

## Phase 5 surface

Phase 5 hardening falls into four named surfaces. Each is independent — a layer may exercise all four, some, or one. The combination is keyed to the project's `**Phase 5 strategy:**` declaration:

**Tool-install upfront cost ([G-175](../suite-development/FINDINGS-INDEX.md#g-175)).** First-Phase-5-session-per-project bundles tool installs with the run — cargo-mutants compiles from source (1–2 minutes on a modern machine); cargo-fuzz requires the [Rust](https://www.rust-lang.org/) nightly toolchain; proptest / fast-check / hypothesis are dev-dependency adds on Cargo.toml / package.json / pyproject.toml. The Phase 5 log preamble for a project's first hardening session names the installs performed and the time spent on installs (separately from the time spent on actual hardening). Subsequent sessions inherit the installed tools and skip the install step.

### Purity Boundary Audit (Phase 5 — required preamble for every layer entry)

Before running any property-based tests, mutation tests, fuzzers, or proof harnesses, audit the implementation against **every authoritative purity claim the project makes**. Purity claims live in (at minimum) two places that can drift independently:

- `DESIGN.md` § Verification architecture — the spec's named purity boundary (which functions are pure, deterministic, formally verifiable in principle).
- **Module / package documentation in the implementation** — Rust module-level `//!` doc comments; [Python](https://www.python.org/) module docstrings; [TypeScript](https://www.typescriptlang.org/) `/** @module */` JSDoc blocks; equivalent constructs in other languages. A module that opens with "Pure-core storage logic" makes a purity claim that any maintainer reading the code will take as authoritative.

The audit checks (a) the implementation against the DESIGN.md claim, (b) the implementation against the module-doc claim, and (c) the DESIGN.md claim against the module-doc claim (cross-source consistency — discovered against the manual-method reference example at Review 66 / [G-173](../suite-development/FINDINGS-INDEX.md#g-173): `vsdd-suite-reference-examples/bookmark-cli-manual/src/lib.rs:1-7` claimed "Pure-core storage logic ... contains only pure functions" while its `DESIGN.md` § Verification architecture was silent on per-function purity; both diverged from the actual implementation).

For each function the project claims is pure (from either source):

1. Open the implementation. Verify the function's signature and body do not perform I/O (file system, network, process spawning, environment variable reads, random number generation, system time).
2. If the function violates the purity boundary in implementation, the violation routes one of three ways: (a) the function was correctly specified as pure but the implementation drifted — route to [Phase 2b](2b-implementation.md) to restore purity (extract the effectful behavior into a wrapper, keep the inner function pure); (b) the purity boundary in DESIGN.md was wrong — route to Phase 1a+1b to revise the boundary; (c) the function is "morally pure" (e.g., takes a clock as a parameter rather than reading the system clock directly) — name this in the Phase 5 log preamble as a noted boundary refinement.
3. If DESIGN.md and the module doc make divergent purity claims (one says X is pure; the other is silent or contradictory), the divergence is itself a finding — route to Phase 1a+1b to reconcile (single source of truth at DESIGN.md; module doc points at it, OR module doc is the authoritative source and DESIGN.md cites it).
4. Record the audit outcome in the Phase 5 log preamble per layer: "Purity boundary verified for functions: `<list>` (DESIGN.md + module-doc sources consistent)" OR "Boundary violations found and routed: `<list with routing>`" OR "Cross-source divergence found between `<DESIGN.md location>` and `<module-doc location>`; reconciliation routed to Phase 1a+1b."

A Phase 5 layer entry that omits the Purity Boundary Audit preamble is itself a finding for [VDD-IAR Alignment](../domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md) dim 13 — the gate criterion "purity boundary verified" (completion criteria #1) has no surface owning it otherwise.

### Property-based testing (Phase 5 — for the purity boundary)

The spec's verification architecture (per `primers/1ab-spec-crystallization.md`) names which functions are pure (deterministic, no I/O, formally verifiable in principle). Property-based testing exercises these functions across input ranges automatically, surfacing failures that fixed-example tests miss.

**Prerequisite ([G-176](../suite-development/FINDINGS-INDEX.md#g-176)):** add the language's property-based testing tool as a dev-dependency before authoring properties. Rust: `proptest` or `quickcheck` in `[dev-dependencies]` in `Cargo.toml`. JavaScript/TypeScript: `fast-check` in `devDependencies` in `package.json`. Python: `hypothesis` in `[tool.poetry.group.dev.dependencies]` or `requirements-dev.txt`. Go: `gopter` via `go.mod`. The dep-add is a separate commit before the property-test commits — keeps the dep-introduction reviewable independently of the property authorship.

For each function on the purity-boundary list:

1. Enumerate the spec's invariants for that function. Examples: `parse(format(x)) == x` (roundtrip); `sort(xs).len() == xs.len()` (length preservation); `validate(s).is_ok() implies parse(s).is_ok()` (validation-parse consistency).
2. Express each invariant as a property-based test using the language's standard tool:
   - **Rust:** `proptest` or `quickcheck`
   - **JavaScript / TypeScript:** `fast-check`
   - **Python:** `hypothesis`
   - **Go:** `gopter` or stdlib `testing/quick`
3. Run with a non-trivial input-size budget (default: 1000 cases per property; surface larger budgets in the Phase 5 log if a property's search space is small).
4. Each property-based test that surfaces a counterexample produces one of: a test failure that becomes a Phase 4 routing (the spec asserts the property; the implementation violates it → route to Phase 2b); a spec gap (the spec didn't assert the property; the implementation behavior is undefined for the counterexample → route to Phase 1a+1b); or an over-strict property (the test asserts more than the spec — narrow the property, document the narrowing).

**The named anti-pattern:** writing a property-based test whose only assertion is "doesn't panic." If the spec asserts X and your property only asserts not-panic, the property hasn't tested X; it's tested liveness. Properties must express the spec's invariants directly.

### Mutation Testing (Phase 5 — of the existing test suite)

Mutation testing measures whether the test suite would catch real defects by injecting small behavior-altering mutations into the source and re-running the tests. A surviving mutant (a mutation that did not cause a test failure) indicates the test suite has a blind spot.

For each layer's source files in scope (typically all files touched by the layer's implementation):

1. Run the mutation tool. Standard tools:
   - **Rust:** `cargo-mutants` (per `supplements/rust.md` § QE)
   - **JavaScript / TypeScript:** `Stryker`
   - **Python:** `mutmut`
2. Examine each surviving mutant. For each, choose one of the five outcomes named in the Mutation Testing sycophancy check above (equivalent / missing-test / spec-gap / unviable / out-of-scope). The Phase 5 log records the disposition per mutant. Note: cargo-mutants and equivalent tools report "unviable" mutations separately from missed/caught — these are mutations that fail to compile or are type-system-rejected, so they're not behavioral signals. List unviable mutants in the log with a one-line note (e.g., "`src/lib.rs:55:35` `+` → `-` in string concatenation: unviable — compile failure on `String - &str`") but do not treat them as test-suite gaps.
3. The hardening goal is **not 100% kill rate** — pursuing 100% leads to test bloat (adding trivial tests that catch trivial mutants). The goal is to surface the mutants in code paths the spec's invariants cover, then act on those. A surviving mutant in a logging statement may be equivalent; a surviving mutant in a validation predicate is a missing test.
4. Mutation-test kill rate is reported in the Phase 5 log, but the audit signal is the **disposition table** (per-mutant outcome), not the percentage.

**The named anti-pattern:** running Mutation Testing once and reporting only the aggregate kill rate. The aggregate number hides which mutants survived and why. The Phase 5 log must include the per-mutant disposition for **every surviving mutant within the project's evaluation scope** (default scope: every surviving mutant in spec-asserted code paths, where "spec-asserted" means code paths the project's DESIGN.md names as bearing a behavioral contract). Mutants outside the scope (e.g., in logging-only or telemetry-only code paths the spec does not constrain) may be omitted from the disposition table provided the omission is named in the Phase 5 log preamble. Equivalent-mutant exemptions within the scope are listed individually with their equivalence proof; aggregate exemption ("all surviving mutants in module X are equivalent") is the anti-pattern.

### Fuzz Testing (Phase 5 — for parser / input-boundary surfaces)

Fuzzing exercises code paths with semi-random input, surfacing crashes, assertion failures, and panics that fixed-example tests don't enumerate. Phase 5 Fuzz Testing targets functions on the spec's external-input surface (file parsers, network protocol decoders, CLI argument parsers, deserialization entrypoints).

For each named input-boundary surface:

1. Identify the entry point. The function that turns "bytes from the outside world" into "in-process data structure."
2. Run the language's Fuzz Testing tool:
   - **Rust:** `cargo-fuzz` (libFuzzer-based) or `AFL.rs`
   - **C / C++:** `libFuzzer` or `AFL++`
   - **JavaScript / TypeScript:** `fast-check` (the same tool used in property-based testing on JS/TS — but the property shape is different: property-based testing asserts spec invariants over generated inputs; Fuzz Testing asserts "doesn't crash / matches a Fuzz Testing oracle" over `fc.string()` / `fc.uint8Array()` parser-input generators; the two uses register as two distinct rounds in their respective per-domain logs — property-based testing in the SA log, Fuzz Testing in the QE log — not as one combined entry); specialist tools for protocol decoding
   - **Python:** `atheris` (libFuzzer for Python)
   - **Go:** stdlib `testing.Fuzz` (Go ≥ 1.18)
3. Run for a time budget appropriate to the project's intent — capstone-intent typically uses 1+ hour budgets per fuzzer per release; production-intent uses CI-scheduled multi-hour budgets.
4. Each crash or assertion failure surfaced by the fuzzer is a finding routed through Phase 4 like any other Phase 3 finding (typically `route:phase-2b` for implementation defects; `route:phase-1a+1b` for spec gaps; rarely `route:phase-2a` for missing tests).
5. Saved corpus from the fuzzer is committed as part of the layer's test infrastructure — future fuzz runs benefit from the prior runs' coverage discovery.

**The named anti-pattern:** running Fuzz Testing once at the end of the project, after every layer is built. Fuzz at each layer that adds a new input-boundary surface; the fuzz corpus grows alongside the implementation. A single end-of-project fuzz run on an integrated system has less per-finding actionability than a per-layer run that identifies which layer introduced the regression.

### Proof Execution (Phase 5 — for designated pure functions; advanced; strictly optional)

For projects whose verification architecture named formal-proof candidates (typically safety-critical control logic, cryptographic primitives, or financial-calculation kernels), Phase 5 produces proof harnesses establishing the named properties hold for all inputs in the function's domain.

For each formal-proof candidate:

1. Map the spec's invariant to a formally-statable property. The mapping is non-trivial — a property like "the function returns the smallest valid output" requires a formal definition of "smallest" and "valid" in the proof system's logic.
2. Choose the appropriate tool:
   - **Rust:** `kani` (bounded model checking; verifies properties for all inputs up to a configured bound)
   - **C:** `CBMC` (industry-standard bounded model checker); or specialized tools like `Frama-C` for ANSI/ISO C
   - **TLA+** for system-level properties (consensus, distributed protocols, eventual consistency)
   - **Coq / Lean / Idris / Agda** for full first-class proofs of pure mathematical properties
   - **Liquid Haskell** for refinement-typed Haskell code
3. Each proof harness is a separate file in `tests/` (or the language's idiomatic location for verification artifacts) and runs as part of CI alongside the standard test suite.
4. A failing proof harness blocks Phase 6 — the spec asserted a property the proof system disproved; the spec is wrong, the implementation is wrong, or the property's formalization is wrong. Route via Phase 4.

**The named anti-pattern:** writing a proof harness that proves a tautology (e.g., `forall x: f(x) == f(x)`). A proof harness must establish a non-trivial property the spec asserts. The Phase 5 log records what property the harness establishes and how that maps to a DESIGN.md invariant.

This surface is **strictly optional** even at capstone+ intent. A project that declares `**Phase 5 strategy:** planned — property-based testing + Mutation Testing + Fuzz Testing; Proof Execution not applicable (no safety-critical or cryptographic surface)` is closing Phase 5 correctly without Proof Execution. Only declare Proof Execution when the spec's verification architecture named formal-proof candidates and the project's intent justifies the proof effort.

---

## Driving questions

Work through these for each layer entering Phase 5:

1. **What does the spec's verification architecture say about this layer?** Which functions are pure? Which behaviors are automatable? Which are formal-proof candidates? If the spec didn't answer these (the verification-architecture sub-section in DESIGN.md is empty or vague), the Phase 5 work is blocked on a Phase 1a+1b spec gap — route there before continuing.
2. **Which Phase 5 surfaces apply to this layer?** A pure-function-heavy layer activates property-based testing (property-based) and possibly Proof Execution. A parser layer activates Fuzz Testing. Every layer with non-trivial logic activates Mutation Testing regardless. Document the activation in the Phase 5 log.
3. **What is the per-surface budget?** Mutation testing typically takes minutes per layer; Fuzz Testing takes hours and may run in CI; property-based testing runs in the same wall-clock budget as the unit tests. Set the budget at the start of the session and stop when reached — Phase 5 like any other phase has the cost-vs-value bound.
4. **How does each surface's output route?** Property-based counterexamples and fuzzer crashes route through Phase 4 like any other finding. Surviving mutants are recorded in the Phase 5 log with per-mutant dispositions. Failing proof harnesses are routed via Phase 4 to the appropriate phase (typically 1a+1b for spec gaps; 2b for implementation defects).
5. **What's the gate to Phase 6?** Phase 5 is gate-complete when (a) every property-based test passes for at least the budgeted input count; (b) every surviving mutant has a recorded disposition; (c) the fuzzer's time budget elapsed without surfacing a new crash class; (d) any formal-proof harnesses succeed. Each is a separate signal; a Phase 5 partial completion is a partial gate (Phase 6 entry blocked until the partial is closed).

---

## Phase 5 log format

Phase 5 work files under the existing per-domain review log structure — no separate per-project Phase 5 file ([G-177](../suite-development/FINDINGS-INDEX.md#g-177) closure, 2026-05-20). Each Phase 5 surface, per layer, becomes a new round entry in the per-domain log that owns that surface:

| Surface | Per-domain log | Rationale |
|---|---|---|
| A (property-based testing) + A.0 (purity preamble) + D (Proof Execution) | `vsdd-suite/SOLUTION-ARCHITECT-REVIEW.md` index + `review-log/<date>-solution-architect.md` session file | SA owns the purity-boundary map (Dim 12 — VSDD purity boundary) and formal-proof targets |
| B (Mutation Testing) + C (Fuzz Testing) | `vsdd-suite/QUALITY-ENGINEER-REVIEW.md` index + `review-log/<date>-quality-engineer.md` session file | QE owns the test system; Mutation Testing is QE Dim 2 (test falsifiability); Fuzz Testing exercises test coverage at the parser boundary |

A project may file Fuzz Testing under [Security](../domains/role/SECURITY-REVIEW.md) instead when the parser is named in the threat model — record the choice once in `DESIGN.md` § Verification architecture and stay consistent.

Per-round preamble (added to the standard per-review preamble per `suite-development/suite-development.md` § Per-review entry preamble):

```markdown
**Phase 5 hardening:** Mutation Testing — Mutation Testing for Layer 1 via cargo-mutants
```

Surface-letter, layer reference, and tool are all named. The round body follows the standard per-domain log structure (Scope, Session note, Source, findings grouped by classification, Summary, Coordination). Surface-specific output (per-property invariant, per-mutant disposition table, per-fuzz-entry corpus, per-harness proof) goes into the round body's finding sections; an Open finding for a surviving mutant uses the per-mutant disposition table format.

---

## Crosslink mode

Phase 5 work in [crosslink](https://github.com/forecast-bio/crosslink) mode treats each hardening surface as a session segment within the layer's session. The findings filed by Phase 5 work share the existing `review-finding` label so Phase 4 routing handles them uniformly with Phase 3 findings:

```sh
# Start the Phase 5 session for Layer N
crosslink session start
crosslink session work "$LN"             # the layer issue from Phase 1c

# Run the hardening surfaces; findings filed manually as they surface:
crosslink issue create "Prop test counterexample: parse(format(x)) != x for x={...}" \
    -l review-finding -l phase:5 -l surface:property-based -l layer:N -l domain:quality-engineer \
    --parent "$LN"

# Surviving-mutant dispositions logged as comments on the layer issue:
crosslink issue comment "$LN" "Phase 5 Mutation Testing: surviving mutant src/foo.rs:42 — equivalent, proof: <prose>" --kind note

# Phase 5 closure annotation:
crosslink issue comment "$LN" "Phase 5 complete: surfaces A+B+C activated; D declared not applicable per DESIGN.md § Verification architecture; ready for Phase 6 entry." --kind decision
```

Findings filed with `phase:5` label join the Phase 4 routing queue alongside Phase 3 findings.

---

## Manual mode

Same discipline; the canonical artifact is the per-domain review log round with the `**Phase 5 surface:**` preamble tag (per [G-177](../suite-development/FINDINGS-INDEX.md#g-177) closure). Each surface's tool output (mutation kill rate, fuzzer corpus, property-test summary) is captured in the appropriate per-domain round (SA for A/A.0/D; QE for B/C). Findings routed via Phase 4 are recorded in the project's `FINDINGS-INDEX.md` with `phase:5` in the Source column (extending the [G-133](../suite-development/FINDINGS-INDEX.md#g-133) Source taxonomy).

---

## Completion criteria

Phase 5 is gate-complete for a layer when:

1. The spec's verification architecture for this layer is verified — the purity boundary named in DESIGN.md matches the implementation's actual pure functions (catch the case where a "pure" function quietly took on I/O during implementation).
2. Surfaces activated per the project's `**Phase 5 strategy:** planned — <scope>` declaration each have a recorded round in the appropriate per-domain log (SA for A/A.0/D; QE for B/C) with the `**Phase 5 surface:**` preamble tag.
3. Every surviving mutant within the evaluation scope (default: spec-asserted code paths) has a per-mutant disposition (no aggregate-only reporting); out-of-scope omissions are named in the log preamble.
4. Every Phase 5 finding routed to Phase 4 has either been Resolved (with the resulting Phase 5 re-run confirming the resolution) or Deferred-with-named-trigger per [G-130](../suite-development/FINDINGS-INDEX.md#g-130).
5. The project's `**Phase 5 strategy:**` declaration's named scope is complete (a `planned — property-based testing + Mutation Testing` declaration cannot close until both have run).

When (1)–(5) hold, the layer is Phase-5-MVR. Phase 6 (Four-Dimensional Convergence) is the cross-layer convergence gate — see `primers/6-convergence.md` for its entry conditions.

**Forward-only:** Phase 5 ownership in the suite is new as of 2026-05-20 (Review 64 / v0.7.0; [G-55](../suite-development/FINDINGS-INDEX.md#g-55) closure). Projects whose first layer-gate close predates 2026-05-20 are not retroactively required to retro-fit Phase 5 work; they continue under the prior "skip Phase 5 unless safety-critical" framing. Capstone-intent projects whose first layer-gate close is on or after 2026-05-20 must declare Phase 5 strategy ([G-162](../suite-development/FINDINGS-INDEX.md#g-162) enforcement; not new — promoted 2026-05-19).

## Cold-session-vs-inline decision rubric ([Review 91](../suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) Finding 5)

Phase 3 IAR defaults to cold-session adversarial review (per [`primers/3-review-session.md`](3-review-session.md) § Session isolation). Phase 5 hardening has a different default: **inline execution is acceptable for tool-output-driven evidence with bounded judgment surface; cold-session is required only when the surface involves adversarial-framing judgment.**

**Cold-session REQUIRED when the Phase 5 surface involves adversarial-framing judgment:**

- **Purity Boundary Audit on a freshly-authored library** — module-doc claims must be evaluated against implementation + DESIGN.md by an adversarial reviewer who did NOT author the module-doc + implementation in the same session. Per the [bookmark-cli-manual SA Review 1 / F-004 evidence](../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-20-solution-architect.md) cross-source divergence finding ([G-173](../suite-development/FINDINGS-INDEX.md#g-173)): an in-session author normalizes their own divergences because each artifact looked defensible from the local position the author held when authoring it.
- **Fuzz Testing campaign** where the corpus design itself involves adversary-modeling (what inputs an attacker would prefer; what coverage gaps to target).
- **Proof Execution** where the property design requires adversarial pressure on what property to prove (a tautology-property is the AI Engineer Dim-2 over-investment failure mode; cold-session pressure surfaces it).
- **First-Phase-5-run on a layer** generally — establishing the baseline kill-rate / coverage / property-set deserves cold-session pressure to surface what in-author review missed.

**Inline execution ACCEPTABLE when the Phase 5 surface is tool-output-driven evidence with bounded disposition:**

- **Mutation Testing re-run against an unchanged purity boundary with a documented prior baseline** — `cargo-mutants` (or `mutmut`, `stryker`, etc.) produces deterministic per-mutant evidence; the analysis surface is "classify each surviving mutant as acceptable-survival / test-gap / surprise-survivor" which is mechanical disposition against a published rubric, not adversarial framing. Cold-session cluster spawn against tool-output is over-investment per [G-150](../suite-development/FINDINGS-INDEX.md#g-150) — the tool produces the same evidence regardless of session isolation.
- **Property-based test re-verification against a structurally-stable strategy** — re-running existing `proptest` / `fast-check` / `hypothesis` properties against a refactored implementation is empirical evidence; pass/fail is the judgment surface; no adversarial framing needed.
- **Phase-5-trigger follow-up rounds** — closing carry-forward items per a prior cycle's named triggers (e.g., `proptest restructure`, `scaling sentinel refactor`, `fsync filesystem-coverage caveat`) is bounded scope with named acceptance criteria; inline closure with empirical verification is sufficient. Per the [bookmark-cli-manual QE Review 7 / Layer 2 PR #47 evidence](../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-21-quality-engineer.md#review-7--2026-05-23-1600z) — canonical inline-acceptable worked example.
- **Purity Boundary Audit re-run against a layer whose pure-side declarations have not changed since the prior round's audit** — per the [bookmark-cli-manual SA Review 4 / Layer 2 PR #44 evidence](../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-22-solution-architect.md): when the pure-side surface extends incrementally (added `filter_by_tags` + `attach_tag` on top of an already-audited `newest_first` boundary), inline cross-source consistency check is sufficient if the operator can name the changed surface explicitly.

**Per-round declaration REQUIRED for every Phase 5 round:**

Each Phase 5 round's preamble MUST declare its cold-session-vs-inline choice + the trade-off rationale. Parallel to [`primers/3-review-session.md`](3-review-session.md) § Pre-cycle methodology check's discipline. Acceptable forms:

```
**Cold-session shape:** cold-session cluster spawn — Phase 5 surface involves adversarial-framing judgment (first-run Purity Boundary Audit on freshly-authored library; cross-source divergence pressure required).
```

OR

```
**Cold-session shape:** N/A — inline-run from the main session. Trade-off declared per the bounded-judgment-surface rubric (Mutation Testing re-run against unchanged purity boundary; cargo-mutants tool output IS the evidence; per-mutant disposition is mechanical classification not adversarial framing). Cold-session spawn would be over-investment per G-150.
```

**Why this rubric exists:** the [bookmark-cli-manual Layer 2 Phase 5 cycle](../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-21-quality-engineer.md#review-6--2026-05-22-2230z) used inline execution for three rounds (SA Review 4 Purity Boundary Audit re-run; QE Review 6 Mutation Testing re-run; QE Review 7 Phase-5-trigger follow-up) per the G-150 over-investment guard. The rationale was well-reasoned but lived only in per-round session notes; the methodology lacked a primer-level decision rubric for future cycles to inherit. [Review 91 Finding 5](../suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) named the codification gap; this sub-section closes it.

**Forward-only:** the rubric applies to Phase 5 rounds authored 2026-05-24 and later; pre-2026-05-24 rounds without explicit cold-session-vs-inline declaration are preserved as authored per [G-89](../suite-development/FINDINGS-INDEX.md#g-89).


## Three-audience lens

This hardening primer serves all three audiences of the [three-audience design principle](../suite-development/suite-development.md#three-audience-design-principle-review-80-finding-3) ([Review 80](../suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z) Finding 3; renamed in [Review 84](../suite-development/review-log/2026-05-21-suite-review.md#review-84--2026-05-21-1100z) Finding 4):

- **Suite developers** evolving this primer treat the prose as the methodology-authoring surface for Phase 5 formal hardening — changes here are methodology shifts requiring their own Review.
- **Suite users** running a session against this primer treat it as the canonical step-by-step for Phase 5 formal hardening on their own project; the completion criteria are what their next layer-gate or phase-close commit is checked against.
- **AI agents** loaded with this primer as cold-session context treat it as the spec for the session's authoring shape (file locations, classification vocabulary, the audit-trail entries this session produces); the primer's named artifacts + their schemas are the agent-API contract for what the session writes.

See [`../suite-development/suite-development.md`](../suite-development/suite-development.md) [§ Three-audience design principle](../suite-development/suite-development.md#three-audience-design-principle-review-80-finding-3) for the full discipline.
