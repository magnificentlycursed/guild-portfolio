# VDD-IAR Alignment Review

This review is part of the [Iterative Adversarial Refinement (IAR)](README.md) suite. It may be run independently or alongside other domains. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

The purpose of this review is to evaluate whether the project was built using the Verification-Driven Development (VDD) and Iterative Adversarial Refinement (IAR) methodology. The other IAR domains evaluate *what* was built. This domain evaluates *how* it was built — whether the process that is supposed to produce quality actually ran.

This domain is part of the IAR suite — which serves as the **adversary** in the VSDD pipeline. IAR fills VSDD Phase 4 (Adversarial Refinement): it operates with fresh context, applies structured pressure across multiple lenses, iterates until MVR, and certifies that the exit condition was reached honestly. VDD-IAR Alignment evaluates whether that adversary itself ran with integrity.

## Governing References

- **VSDD whitepaper** (primary): https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00
- **Original VDD whitepaper**: https://gist.github.com/dollspace-gay/45c95ebfb5a3a3bae84d8bebd662cc25
- **Apprentice-onboarding repo** (program methodology, tool introduction schedule, assignment briefs): https://github.com/Navigators-Guild/apprentice-onboarding
- **CLAUDE.md** (may be superseded — verify against current apprentice-onboarding content): https://gist.github.com/dollspace-gay/ef132e60a27abe6d5f87297c1c040dca
- **Crosslink** (issue tracker used from Phase 2 onward): https://github.com/forecast-bio/crosslink

The reference for the process methodology itself is: `apprentice-onboarding/02-the-methodology/01-how-we-build.md`. Treat it as this domain's DESIGN.md equivalent: the adversary holds the project's process against the methodology, not against the code.

## Current Review Prompt

**Scope:** The full development history of the project — commit log, DECISIONS.md or equivalent, IAR review logs, layer gate records, and DESIGN.md. Code is read for evidence of process compliance (e.g., do tests appear before or after implementation commits?) rather than for correctness — that belongs to QE and SE.

Read the governing methodology document in full before reviewing any artifacts. Then read: DESIGN.md, the commit history, all IAR review logs, and any retrospective or decisions documents. Apply every standard dimension below as a floor.

For each finding, cite the specific artifact and location (commit hash, log entry, file and line). Classify as **resolved** (fix applied this review), **dismissed** (no action taken, rationale required), or **hallucinated** (the adversary invented a process failure that does not exist — push back is warranted. Consistent hallucinated findings are the maximum viable refinement signal: real issues have been exhausted).

**Coordination:** Process failures frequently explain defects found by other domains. If QE found that tests were added after implementation, flag it here as a test discipline finding. If SA found that architecture grew organically rather than being designed, flag it here as a decomposition finding. Coordinate with [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) on assignment compliance — SO owns whether DESIGN.md matches the assignment brief; this domain owns whether design-before-code discipline was followed. If this review suggests the need for a new IAR domain, log it as a finding.

**Language and interface supplement:** Not applicable. Process compliance is language-agnostic. The `lang/` supplements add language-specific dimensions to implementation-focused domains; the VDD-IAR Alignment domain evaluates methodology compliance, which is independent of the implementation language or interface type.

**Sycophancy check:** Process failures are easy to rationalize. The agent reviewing this domain is likely the same agent that participated in building the project — it has every incentive to find the process acceptable. The absence of a layer gate record is not ambiguous. Batched test commits are not ambiguous. A single IAR pass that merged immediately after real findings is not ambiguous. Push back on any dimension where the agent reaches for benefit-of-the-doubt rather than evidence.

## Standard Evaluation Dimensions

1. **Design-before-code** — Was DESIGN.md or an equivalent design artifact created before implementation began? Does the first commit establish design documentation rather than code? Flag any project where the design doc was written after the fact or retroactively expanded to justify implementation choices already made.

   Beyond existence, evaluate spec completeness using VSDD Phase 1 criteria. A design doc that enumerates only happy-path behaviors is an incomplete spec. A complete spec contains:
   - **Behavioral contracts** — preconditions, postconditions, and invariants for each feature. Not just "the form saves the bookmark" but what the system guarantees before, during, and after every operation.
   - **Exhaustive edge case catalog** — boundary values, empty inputs, malformed inputs, concurrent operations, and failure modes. An edge case absent from the spec is likely absent from the tests.
   - **Interface definitions** — explicit data shapes, validation rules, and error responses at every boundary. Interfaces defined only implicitly in code rather than explicitly in the spec were not designed; they emerged.
   - **Verification architecture** — how will acceptance be determined? Which behaviors are automatable? Which require manual validation? A spec without a verification plan is aspirational, not actionable.

   Flag design docs that are feature lists rather than behavioral specifications.

2. **Layered decomposition** — Was the project broken into explicitly bounded layers or iterations with defined acceptance criteria? Does the commit history reflect this structure with clear layer boundaries, or is development one undifferentiated mass of commits? Flag any layer that has no defined acceptance criteria or whose commits span multiple stated layers.

   The artifact holding the layer plan changes by phase: **Phase 1** uses `TODO.md`; **Phase 2+** uses crosslink (which replaces `TODO.md` as the source of truth — they are not maintained in parallel). Evaluate against whichever is appropriate for the project's phase. The structural requirement is the same regardless of tool: layers are explicit, bounded, and acceptance-criteria-bearing before work begins.

3. **Layer gate compliance** — Was each layer fully verified and gated before the next began? Is there evidence that: acceptance criteria were checked, tests passed, IAR ran, and the layer was explicitly closed before the next opened? A commit that introduces features from a new layer before the previous layer's IAR log is complete is a finding.

4. **Test discipline** — Were tests written before or alongside implementation, not after? This is a finding, not a yellow flag. Look at the commit pattern: does each feature commit include corresponding tests, or do tests arrive in a batch after the implementation is complete? A commit that adds all tests for an already-completed layer is a finding. A layer where implementation commits consistently precede any test commits is a finding. Exceptions require explicit documented rationale — "I wrote the test first but committed them together" is acceptable if other evidence supports it; "the tests came later because I wanted to get the code working first" is not.

   **Red Gate:** Tests must be in a failing state before implementation begins. A test that passes against an empty function body, a stub returning `null`, or an unimplemented module was not written first — it was written to match existing code. Flag any commit where tests could not have failed before the corresponding implementation existed. A test suite that would pass against a completely empty implementation is a process failure regardless of when it was committed.

   Positive evidence of test-first: tests committed in the same commit as or before the implementation they cover; CI history or commit messages showing tests failing before a fix was applied; test names that describe intended behavior before the implementation existed ("should reject empty titles" written before `validateTitle`).

   When evidence is ambiguous, assess the QE dimension 14 (TDD proxy indicators) for corroborating artifact evidence. Process and artifact evidence together are more reliable than either alone.

5. **Human verification** — Is there documented evidence that the human director verified each layer against intent — a completed manual testing checklist, an explicit sign-off, or a layer gate record? AI-only verification is not sufficient. Code that the builder wrote and the adversary reviewed but the human never ran is not verified. The absence of any human verification artifact is a finding.

6. **IAR fresh context** — Were adversarial review rounds conducted with fresh AI context, or batched into the same session where the code was written? A log entry covering a review that occurred in the same session as the implementation is weaker. A single log entry spanning multiple layers without a context reset is a finding. Assess from log structure, session framing, and timing evidence.

7. **IAR iteration** — Were rounds iterated when findings were substantial? A single adversarial pass followed immediately by a merge is a finding if that pass produced real findings. The expected pattern: findings → fix → second pass with fresh context → repeat until MVR → merge. Check the review log for round numbers and finding progression.

8. **Role integrity** — Did the human director actually direct — define goals, set constraints, make judgment calls, push back on agent defaults — or did the agent make all decisions? Review DESIGN.md, decisions logs, and commit messages. A design doc that shows no evidence of human scoping choices, or a commit history where every decision traces to an agent default, is a finding. The human's fingerprints should be visible in the work.

9. **Manual testing checklists** — Does the project have explicit manual testing checklists per layer or feature? Were they completed and recorded before the layer gate closed? Automated tests verify correctness; manual testing verifies that the experience is coherent — interaction flows, error states that feel right, "technically correct but not what I meant" failures. For CLI projects especially, the human must run the binary and evaluate output quality. Absence of a manual checklist is a finding regardless of automated test coverage.

10. **Retrospective quality** — Is there a post-mortem, DECISIONS.md, or equivalent that honestly names: what went wrong, what was cut, what the agent got wrong, and what was learned? A document that records only successes is a finding — real projects have problems. The specificity and honesty of the retrospective is proportional to the quality of self-assessment demonstrated by the project.

11. **Issue tracking compliance** — Is work tracked in crosslink (or the equivalent tool required at this program phase) before it begins? Apply this dimension only for Phase 2 and later projects; crosslink is introduced in Phase 2 and Phase 1 projects are exempt.

   For Phase 2+ projects, evaluate:
   - **Issues before work** — Was a crosslink issue opened before the corresponding work began? Undocumented work (code committed with no corresponding issue) is a finding.
   - **Bead-string structure** — Are issues organized in an epic → issue → subissue hierarchy? A flat list of unrelated issues is not a bead string.
   - **Session discipline** — Were sessions opened at the start of work and closed with handoff notes (`crosslink session start` / `crosslink session end --notes "..."`)?
   - **Decision documentation** — Are significant decisions recorded as issue comments, not just in commit messages or DECISIONS.md? Crosslink is the primary record of in-flight reasoning; DECISIONS.md records outcomes.
   - **Blocking relationships** — Are dependencies between issues declared (`crosslink issue block <id> <blocker_id>`) rather than inferred?

   Commands for reference: `crosslink session start`, `crosslink quick <title> -p <priority> -l <label>`, `crosslink subissue <parent_id> <title>`, `crosslink issue comment <id> "..."`, `crosslink session end --notes "..."`, `crosslink issue next`.

## Program Phase Context

Tool requirements and process expectations scale with program phase. When evaluating a project, establish its phase before applying this domain:

- **Phase 1** — Crosslink not yet introduced. Issue tracking compliance (dim 11) is not applicable. Evaluate all other dimensions.
- **Phase 2** — Crosslink introduced. Issue tracking compliance is required from the first Phase 2 project onward. Retroactive application to Phase 1 projects is not appropriate.
- **Phase 3** — Crosslink required. Expect mature session discipline and bead-string structure.
- **Phase 4 (capstone)** — All Phase 3 requirements plus: release binaries, external user validation, and a retrospective that addresses the full arc from Phase 1 to capstone.

Do not penalize a Phase 1 project for absent crosslink usage. Do not waive crosslink requirements for a Phase 2+ project because the project "predates" the requirement — the requirement scales with phase, not with project start date.

---

Review entries are logged in `iterative-adversarial-refinement/VDD-IAR-ALIGNMENT-REVIEW.md` inside the project being reviewed.
