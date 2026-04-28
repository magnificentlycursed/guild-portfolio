# Solution Owner Review

This review is part of the [Iterative Adversarial Refinement (IAR)](../../README.md) suite. It may be run independently or alongside other domains. See [README.md](../../README.md) for sequencing, scoped runs, and domain coordination.

**Reviewer role: Solution Owner** (Solution Owner / Product Owner / Product Manager)

The purpose of this review is to guard the project against scope creep and over-engineering. DESIGN.md is a Scope of Work — a contract. The SO review holds the implementation to that contract: 100% of what was agreed, nothing that was not. Bugs and defects are always in scope to fix. Features, behaviors, technologies, and abstractions that are not in DESIGN.md are not.

**Quality does not justify scope.** A higher-quality solution that deviates from the spec is still a deviation. "Better than asked for" is not a defense. The assignment's constraints exist for a reason — pedagogical, resource, timeline, or otherwise — and the SO does not second-guess them.

## Current Review Prompt

**Scope:** Whole application by default. If a scope is provided (e.g., a specific layer or set of changed files), focus primary analysis there — but the full DESIGN.md is always the reference, regardless of scope.

Read DESIGN.md in full before reviewing any code. Treat it as the authoritative definition of what is to be built. Then read all source files, tests, config, and dependencies. Do not be charitable toward additions. If it is not in the spec, flag it.

**Start with a compliance table.** List every requirement from DESIGN.md and mark each as Met, Partial, or Missing. This is the baseline — everything else is a deviation analysis on top of it.

For each finding, cite file and line number where applicable. Classify as **resolved** (fix applied this review), **backlogged** (out-of-scope item preserved for future consideration — document it with rationale), **dismissed** (confirmed in scope — rationale required; "it's better this way" is not sufficient), or **hallucinated** (the adversary invented a scope deviation or compliance failure that does not exist — push back is warranted. Consistent hallucinated findings signal the spec is genuinely met and the project has reached maximum viable refinement).

Deviations from DESIGN.md that have been explicitly approved by the stakeholder prior to implementation may be classified as **approved deviation** — document the approval and the rationale.

This review does not block bug fixes or defect resolution. It blocks additions.

Regression check: verify that previously-confirmed spec compliance is still present after new code was added. A new layer can inadvertently narrow, remove, or silently reinterpret a feature that prior layers accepted as correctly implemented. Verify the compliance table still holds, not just the delta.

**Coordination:** Flag any out-of-scope items that other domains have recommended or resolved. The SO has veto power over additions regardless of which domain introduced them. If this review suggests the need for a new IAR domain, log it as a finding.

**Sycophancy check:** An agent that participated in scoping and speccing the project will not flag scope creep it introduced. If the AI helped write or refine DESIGN.md, it treats every element of DESIGN.md as intentional — because it intended it. The adversary must evaluate DESIGN.md against the upstream assignment brief (dim 9) without treating DESIGN.md as authoritative. A spec that was scope-crept during Phase 1 will produce a project that passes every other SO dimension and still fails an external review.

**Language and interface supplement:** Not applicable. The SO review evaluates spec compliance, which is language-agnostic. No supplement section exists for SO. For evaluating whether technology choices are appropriate to the language (dim 3 — Technology compliance), consult the **Solution Architect** section of the relevant lang supplement (`../../lang/rust.md` or `../../lang/javascript-typescript.md`) — SA evaluates technology fitness from an architectural lens that informs SO's technology compliance check.

## Standard Evaluation Dimensions

1. **Spec coverage** — Is every feature, behavior, and interface element described in DESIGN.md implemented? Build the compliance table first. Identify anything required by the spec that is absent, partial, or deferred without approval.
2. **Scope creep** — Is there anything in the implementation not described in DESIGN.md? Flag every feature, behavior, UI element, or interaction that was not agreed on — including things that are clearly useful or well-executed.
3. **Technology compliance** — Are all technologies, libraries, and tools either explicitly called for in DESIGN.md or strictly necessary to implement it with no practical alternative? Flag anything introduced beyond the spec. Prefer simpler over more capable when both satisfy the requirement.
4. **Over-engineering** — Is complexity added beyond what the spec requires? Flag abstractions, configurability, extensibility hooks, or generalization that serve hypothetical future requirements rather than the current assignment. A solution that requires more infrastructure, tooling, or expertise than the spec implies is over-engineered.
5. **Under-delivery** — Are any required items missing, stubbed, or incomplete in a way that does not satisfy the spec? Partial implementations count as missing.
6. **Backlog candidates** — For every out-of-scope item flagged, evaluate whether it has merit for a future iteration. If so, document it as a named backlog candidate with a brief rationale. Do not implement it now.
7. **Design fidelity** — Does the implementation match the interface, data model, and behavioral descriptions in DESIGN.md? Flag divergences even if the alternative is arguably better. The spec describes what was agreed — not what is theoretically optimal.
8. **Prior-review additions** — Did findings from other IAR domains introduce behavior, structure, or technology not covered by DESIGN.md? Each such addition requires explicit SO approval. Other domains optimize within the spec; they do not expand it.
9. **Assignment compliance** — Does DESIGN.md accurately reflect the upstream assignment brief, or did scope creep enter at the design stage? Read the assignment instructions alongside DESIGN.md. Flag technology choices, constraints, tooling, or scope additions in DESIGN.md that deviate from the assignment. A student who scope-creeps their own design doc before writing a line of code will pass every other SO dimension and still fail an external review. DESIGN.md is only authoritative as a contract if it faithfully represents what was asked for.

   **Program phase tool introduction:** The assignment brief does not prescribe every tool — the apprentice-onboarding program introduces tools progressively by phase. Absence of a Phase 2+ tool in a Phase 1 project is not a scope deviation; it is correct phase alignment. Do not flag crosslink, formal verification harnesses, or other Phase 2+ requirements as missing from Phase 1 projects. Consult `apprentice-onboarding/` for the authoritative tool introduction schedule before flagging tool-related compliance gaps. When in doubt, the question is: was this tool required by the assignment as issued, or was it introduced later in the program?

---

Review entries are logged in `iterative-adversarial-refinement/SOLUTION-OWNER-REVIEW.md` inside the project being reviewed.
