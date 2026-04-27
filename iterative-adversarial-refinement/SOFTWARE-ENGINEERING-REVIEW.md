# Software Engineering Review

This review is part of the [Iterative Adversarial Refinement (IAR)](README.md) suite. It may be run independently or alongside other domains. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

The purpose of this review is to evaluate the quality of the implementation at the code level: correctness, clarity, error handling, naming, duplication, and complexity. Where the Solution Architect review evaluates structure and boundaries, the Software Engineering review evaluates the code within those boundaries. Both matter. A well-structured module can still contain poorly written code.

## Current Review Prompt

**Scope:** Whole application by default. If a scope is provided (e.g., a specific feature or set of changed files), focus primary analysis there — but the entire codebase is always fair game for findings.

Read DESIGN.md first for context on the project's intended scope, constraints, and feature set. Then read all source files. Apply every standard dimension below as a floor — add others as appropriate to the current state of the code. There is no restriction on what can be flagged.

For each finding, cite file and line number. Classify as **resolved** (fix applied this review), **deferred** (scheduled for a specific layer, reason given), **dismissed** (no action taken, rationale required), or **hallucinated** (the adversary invented a problem that does not exist — push back is warranted. Consistent hallucinated findings are the maximum viable refinement signal: real issues have been exhausted).

Regression check: verify that previously correct behavior has not been silently broken by implementation changes. A refactor that changes behavior without changing tests is a regression.

**Coordination:** Flag any findings that should be surfaced to [QUALITY-ENGINEERING-REVIEW.md](QUALITY-ENGINEERING-REVIEW.md), [UX-REVIEW.md](UX-REVIEW.md), [SECURITY-REVIEW.md](SECURITY-REVIEW.md), [PLATFORM-ENGINEERING-REVIEW.md](PLATFORM-ENGINEERING-REVIEW.md), or [SOLUTION-ARCHITECT-REVIEW.md](SOLUTION-ARCHITECT-REVIEW.md). If this review suggests the need for a new IAR domain, log it as a finding.

**Sycophancy check:** An agent that designed and implemented the code will find the implementation correct because it reflects its own intent. Push hardest on dim 1 (correctness) and dim 8 (defensive coding): these are the dimensions where implementation intent and spec requirement diverge most often. For every function, ask: "is this doing what was specified, or is it doing what was generated?" They are not the same thing. Flag any function where the implementation could be correct internally but wrong with respect to the spec without any test catching it.

**Assumption surfacing (G-20):** For each external dependency, library function, or framework behavior relied upon, identify the assumption being made. Verify it against the actual documentation for the version in use. AI-generated code frequently calls methods that do not exist, assumes method signatures that differ by one parameter, or relies on return value shapes that changed between versions. Confident code is not correct code.

**Language and interface supplement:** Consult `lang/` for the supplement matching the project's primary language (e.g., `rust.md`, `javascript-typescript.md`) and interface type (e.g., `cli.md`). Apply the **Software Engineering** section from the relevant supplement files in addition to the standard dimensions below.

**Domain boundary:** SE owns the implementation — correctness, naming, error handling, and complexity within module boundaries. QE owns the test system. When SE finds a bug, flag it here. If there is also no test covering that path, that is a separate QE finding — do not bundle them. Do not evaluate test architecture here; that belongs to QE.

## Standard Evaluation Dimensions

1. **Correctness** — Does the code do what it is intended to do? Are there logic errors, incorrect assumptions, or off-by-one mistakes? Does it handle all cases described in DESIGN.md?
2. **Error handling** — Are error cases handled explicitly? Are failures silent, or do they surface to the user or caller in a useful form? Are exceptions caught at the right level?
3. **Naming and type precision** — Are variables, functions, and types named to communicate intent? Would a reader understand what a name refers to without reading the implementation? Also evaluate **primitive obsession**: are raw primitives (`string`, `number`, `boolean`) used where a named type or newtype would prevent a class of bugs? A function that accepts `string` for both a URL and an ID creates a hole where the two can be confused; a function that accepts `Url` and `Id` catches the confusion at the type level. This is especially important for values that flow through multiple layers of the codebase.
4. **Function and method design** — Are functions focused and single-purpose? Are any functions doing too much? Are side effects clearly signaled by name or documented? Specifically: **flag argument (boolean trap)**: a function that takes a boolean parameter that fundamentally bifurcates behavior (`render(item, true)` vs. `render(item, false)`) should be two functions. The boolean is not self-documenting at the call site, and the function typically has two distinct responsibilities that should be separately testable. A boolean parameter that controls a minor variation (e.g., `includeTimestamp: boolean`) is acceptable; one that switches between fundamentally different code paths is not.
5. **Duplication** — Is logic repeated in ways that would require multiple changes to fix a single bug? Flag copy-paste duplication and near-duplication where a small abstraction would eliminate divergence risk.
6. **Complexity** — Is cognitive complexity proportional to the problem? Are there deeply nested conditionals, long functions, or tangled control flow that could be simplified without adding abstraction overhead?
7. **Type safety** — Are types used precisely? Are there unsafe casts, `any` types, or places where a stricter type would prevent a class of bugs?
8. **Defensive coding** — Are assumptions made about inputs, state, or external data that could be violated? Are internal invariants documented or enforced at the right level?
9. **Comments and self-documentation** — Is non-obvious logic explained? Are there misleading, stale, or redundant comments? Code that cannot be understood without a comment is a candidate for renaming or restructuring.
10. **Consistency** — Are patterns, naming conventions, and idioms applied consistently across the codebase? Inconsistency is a maintenance cost and a source of bugs.
11. **Future-self maintainability** — Will you be able to understand and modify this code in six months without access to the original AI session? Are the key decisions derivable from the code and its comments, or do they require conversation history to reconstruct? Flag logic that is correct but would be opaque to future-you — it is a candidate for renaming, restructuring, or a targeted comment.

---

Review entries are logged in `iterative-adversarial-refinement/SOFTWARE-ENGINEERING-REVIEW.md` inside the project being reviewed.
