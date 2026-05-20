# VDD-IAR Alignment Review

This review is part of the [Iterative Adversarial Refinement (IAR)](../../README.md) suite. It may be run independently or alongside other domains. See [README.md](../../README.md) for sequencing, scoped runs, and domain coordination.

The purpose of this review is to evaluate whether the project was built using the Verification-Driven Development (VDD) and Iterative Adversarial Refinement (IAR) methodology. The other IAR domains evaluate *what* was built. This domain evaluates *how* it was built — whether the process that is supposed to produce quality actually ran.

This domain is part of the IAR suite — which serves as the **adversary** in the VSDD pipeline. IAR fills VSDD Phase 3 (Adversarial Refinement): it operates with fresh context, applies structured pressure across multiple lenses, iterates until MVR, and certifies that the exit condition was reached honestly. VDD-IAR Alignment evaluates whether that adversary itself ran with integrity.

## Current Review Prompt

**Scope:** The full development history of the project — commit log, DECISIONS.md or equivalent, IAR review logs, layer gate records, and DESIGN.md. Code is read for evidence of process compliance (e.g., do tests appear before or after implementation commits?) rather than for correctness — that belongs to QE and SE.

Read the governing methodology document in full before reviewing any artifacts. Then read: DESIGN.md, the commit history, all IAR review logs, and any retrospective or decisions documents. Apply every standard dimension below as a floor.

For each finding, cite the specific artifact and location (commit hash, log entry, file and line). Classify as **resolved** (fix applied this review), **dismissed** (no action taken, rationale required), or **hallucinated** (the adversary invented a process failure that does not exist — push back is warranted. Consistent hallucinated findings are the maximum viable refinement signal: real issues have been exhausted).

**Regression check:** Process compliance confirmed in prior VDD-IAR runs for earlier layers does not remain clean automatically — subsequent implementation changes can reopen process failures. Verify that layer gate records, test discipline, and IAR iteration patterns confirmed in prior runs have not degraded in the work under current review. Re-raise any prior finding if new evidence suggests it recurred.

**Coordination:** Process failures frequently explain defects found by other domains. If QE found that tests were added after implementation, flag it here as a test discipline finding. If SA found that architecture grew organically rather than being designed, flag it here as a decomposition finding. Coordinate with [SOLUTION-OWNER-REVIEW.md](../role/SOLUTION-OWNER-REVIEW.md) on assignment compliance — SO owns whether DESIGN.md matches the assignment brief; this domain owns whether design-before-code discipline was followed. If this review suggests the need for a new IAR domain, log it as a finding.

**DESIGN.md change authority:** If a finding requires a change to `DESIGN.md`, classify it "Raised to SO" and document the proposed change and rationale. Do not apply the change. `DESIGN.md` is a controlled spec document — the Solution Owner is the sole domain authorized to modify it. VDD-IAR should flag any instance where a non-SO domain applied a DESIGN.md change without SO approval as a dim 8 (role integrity) finding.

**Sycophancy check:** Process failures are easy to rationalize. The agent reviewing this domain is likely the same agent that participated in building the project — it has every incentive to find the process acceptable. The absence of a layer gate record is not ambiguous. Batched test commits are not ambiguous. A single IAR pass that merged immediately after real findings is not ambiguous. Push back on any dimension where the agent reaches for benefit-of-the-doubt rather than evidence.

**Language and interface supplement:** Not applicable. Process compliance is language-agnostic. The `supplements/` supplements add language-specific dimensions to implementation-focused domains; the VDD-IAR Alignment domain evaluates methodology compliance, which is independent of the implementation language or interface type.

## Governing References

Before applying the standard dimensions, locate and read the governing methodology document for this project. Record in the review log as a preamble entry (not a classified finding): the document URL, and the project's program phase (Phase 1, 2, 3, or 4 — see the Program Phase Context section below). A reviewer who cannot identify the governing document has not completed this review.

- **VSDD whitepaper** (primary): https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00
- **Original VDD whitepaper**: https://gist.github.com/dollspace-gay/45c95ebfb5a3a3bae84d8bebd662cc25
- **Apprentice-onboarding repo** (program methodology, tool introduction schedule, assignment briefs): https://github.com/Navigators-Guild/apprentice-onboarding
- **CLAUDE.md** (may be superseded — verify against current apprentice-onboarding content): https://gist.github.com/dollspace-gay/ef132e60a27abe6d5f87297c1c040dca
- **Crosslink** (issue tracker used from Phase 2 onward): https://github.com/forecast-bio/crosslink

The reference for the process methodology itself is: `apprentice-onboarding/02-the-methodology/01-how-we-build.md`. Treat it as this domain's DESIGN.md equivalent: the adversary holds the project's process against the methodology, not against the code.

## Standard Evaluation Dimensions

1. **Design-before-code** — Was DESIGN.md or an equivalent design artifact created before implementation began? Does the first commit establish design documentation rather than code? Flag any project where the design doc was written after the fact or retroactively expanded to justify implementation choices already made.

   Beyond existence, evaluate spec completeness using VSDD Phase 1a+1b criteria. A design doc that enumerates only happy-path behaviors is an incomplete spec. A complete spec contains:
   - **Behavioral contracts** — preconditions, postconditions, and invariants for each feature. Not just "the form saves the bookmark" but what the system guarantees before, during, and after every operation.
   - **Exhaustive edge case catalog** — boundary values, empty inputs, malformed inputs, concurrent operations, and failure modes. An edge case absent from the spec is likely absent from the tests.
   - **Interface definitions** — explicit data shapes, validation rules, and error responses at every boundary. Interfaces defined only implicitly in code rather than explicitly in the spec were not designed; they emerged.
   - **Verification architecture** — how will acceptance be determined? Which behaviors are automatable? Which require manual validation? A spec without a verification plan is aspirational, not actionable.
   - **Phase 5 / Phase 6 strategy declarations (G-162) — required at capstone and production intents.** `DESIGN.md` § Project intent must include a one-sentence `**Phase 5 strategy:**` line and a one-sentence `**Phase 6 strategy:**` line. Each line must say either `not applicable — <rationale>` (with a real rationale, not "TBD" / "future") or `planned — <named tooling and scope>`. The explicit-skip pattern is borrowed from Phase 2c's "no refactor required" annotation pattern (G-96) — silence at the gate is itself the finding. Learning-exercise and portfolio intents are exempt; the absence of declarations at those tiers is acceptable.

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

7. **IAR iteration and feedback routing** — Were rounds iterated when findings were substantial? A single adversarial pass followed immediately by a merge is a finding if that pass produced real findings. The expected pattern: findings → fix → second pass with fresh context → repeat until MVR → merge. Check the review log for round numbers and finding progression.

   Also evaluate **feedback routing fidelity** (VSDD Phase 4): findings should route back to the appropriate earlier phase for correction, not be patched in place regardless of their nature. Spec findings should result in DESIGN.md updates. Test-coverage findings should result in new or revised tests. Implementation findings should result in code changes. A spec gap fixed only in implementation without updating DESIGN.md propagates the error to future readers of the spec and to the next IAR reviewer. Flag any round where the fix artifact does not match the finding type.

   Also evaluate **cross-session spec consistency**: did the AI's interpretation of requirements shift between sessions without a corresponding DESIGN.md update? This is distinct from a finding being routed back — it is the silent case where no finding was raised but behavioral assumptions changed anyway. Named indicators: commit messages describing behavior that contradicts DESIGN.md; DECISIONS.md entries that expand or reinterpret a feature without a spec revision; IAR findings about unexpected behavior that had no prior spec mention (the spec did not define it, so it was not caught until the adversary looked). The test: can the current DESIGN.md, read cold, produce the current implementation? If a new AI session following DESIGN.md would build something meaningfully different, the spec has drifted from the implementation without being updated.

8. **Role integrity** — Did the human director actually direct — define goals, set constraints, make judgment calls, push back on agent defaults — or did the agent make all decisions? Review DESIGN.md, decisions logs, and commit messages. A design doc that shows no evidence of human scoping choices, or a commit history where every decision traces to an agent default, is a finding. The human's fingerprints should be visible in the work.

9. **Manual testing checklists** — Does the project have explicit manual testing checklists per layer or feature? Were they completed and recorded before the layer gate closed? Automated tests verify correctness; manual testing verifies that the experience is coherent — interaction flows, error states that feel right, "technically correct but not what I meant" failures. For CLI projects especially, the human must run the binary and evaluate output quality. Absence of a manual checklist is a finding regardless of automated test coverage. **File location:** projects subject to the Review 74 convention (first layer-gate close on or after 2026-05-20) carry the per-layer checklist in `manual-tests/layer-N.md` files; pre-cutoff projects carry inline `TODO.md` checklist sections — both are valid, but the per-layer-file convention is the going-forward standard per `../../primers/1c-decomposition.md` § Manual testing checklist. A project whose `TODO.md` Layer N has `**Manual Testing Checklist:**` pointer but no actual `manual-tests/layer-N.md` file is a finding (the pointer is the discipline; the absent target is the defect).

10. **Retrospective quality** — Is there a post-mortem, DECISIONS.md, or equivalent that honestly names: what went wrong, what was cut, what the agent got wrong, and what was learned? A document that records only successes is a finding — real projects have problems. The specificity and honesty of the retrospective is proportional to the quality of self-assessment demonstrated by the project.

11. **Issue tracking compliance** — Is work tracked in crosslink (or the equivalent tool required at this program phase) before it begins? Apply this dimension only for Phase 2 and later projects; crosslink is introduced in Phase 2 and Phase 1 projects are exempt.

   For Phase 2+ projects, evaluate:
   - **Issues before work** — Was a crosslink issue opened before the corresponding work began? Undocumented work (code committed with no corresponding issue) is a finding.
   - **Bead-string structure** — Are issues organized in an epic → issue → subissue hierarchy? A flat list of unrelated issues is not a bead string.
   - **Session discipline** — Were sessions opened at the start of work and closed with handoff notes (`crosslink session start` / `crosslink session end --notes "..."`)?
   - **Decision documentation** — Are significant decisions recorded as issue comments, not just in commit messages or DECISIONS.md? Crosslink is the primary record of in-flight reasoning; DECISIONS.md records outcomes.
   - **Blocking relationships** — Are dependencies between issues declared (`crosslink issue block <id> <blocker_id>`) rather than inferred?

   Commands for reference: `crosslink session start`, `crosslink quick <title> -p <priority> -l <label>`, `crosslink subissue <parent_id> <title>`, `crosslink issue comment <id> "..."`, `crosslink session end --notes "..."`, `crosslink issue next`.

12. **Phase 2c refactor discipline (G-161)** — Did Phase 2c either commit a refactor that adds **no new behavior paths** beyond the Phase 2b implementation, OR record an explicit "no refactor required" annotation? A Phase 2c commit that introduces a new validation, error path, conditional branch, or behavior under any input class is a finding — the refactor smuggled in feature work that bypassed the Phase 2a Red Gate. The check: diff `Phase 2b commit → Phase 2c commit` and look for added control-flow paths (new `if`/`match` arms, new error returns, new branches in pure functions). If found, the new path needs either a corresponding Red Gate test asserting the new behavior (with the retroactive-Red-Gate label per `primers/2b-implementation.md`) OR a back-out commit reverting the change to true refactor scope. A silent skip (no commit, no annotation in `TODO.md` / crosslink session note) is also a finding — Phase 2c must be visible in the audit trail per `primers/2c-refactor.md` § Completion criteria #5. Distinguish: an *intentional refactor that surfaced a spec gap* is fine if the gap was raised to Phase 1a+1b routing via Phase 4 — the discipline is "no silent new behavior," not "no new behavior ever discovered." A Phase 2c commit whose body declares "refactor: extract X" but whose diff shows new behavior is a labeling defect; the audit trail should match the diff.

13. **Phase 5 discipline (G-55 / G-162 / G-177) — capstone + production intents** — Apply only when `DESIGN.md` § Project intent declares `**Phase 5 strategy:** planned — <scope>`. For projects whose Phase 5 strategy is `not applicable`, this dim records the declaration's presence as Demonstrated and skips substantive evaluation; for projects whose Phase 5 strategy is `planned`, evaluate:
    - **Surface activation matches strategy** — the Surfaces named in the strategy declaration each appear as a per-domain review log round with the `**Phase 5 surface:**` preamble tag (per G-177 closure): Surfaces A / A.0 / D in `vsdd-suite/SOLUTION-ARCHITECT-REVIEW.md` + linked `review-log/<date>-solution-architect.md`; Surfaces B / C in `vsdd-suite/QUALITY-ENGINEER-REVIEW.md` + linked `review-log/<date>-quality-engineer.md`. A strategy that names Surfaces A+B+C but rounds covering only B is a finding for under-delivery against the declared scope.
    - **Per-mutant disposition completeness** — Surface B reports each surviving mutant's disposition (equivalent with named proof / missing-test-added / spec-gap-routed). Aggregate-only kill-rate reporting is a finding per `primers/5-formal-hardening.md` § Surface B anti-pattern.
    - **Property-test invariant fidelity** — Surface A properties express spec invariants from DESIGN.md, not generic liveness ("does not panic"). A property whose only assertion is "doesn't panic" is a finding per Surface A anti-pattern.
    - **Fuzz-corpus persistence** — Surface C fuzz corpora are committed under `tests/` (or the language's idiomatic location); a layer that ran fuzzing but didn't commit the corpus has lost the coverage-discovery audit trail.
    - **Phase 4 routing of Phase 5 findings** — counterexamples / surviving non-equivalent mutants / fuzzer crashes / failing proof harnesses each routed via Phase 4 to the correct destination phase (typically `1a+1b` for spec gaps, `2a` for missing tests, `2b` for implementation defects). A Phase 5 finding labeled "fixed" with no Phase 4 routing decision is a discipline gap.

14. **Phase 6 four-dimensional convergence (G-54 / G-162 / G-177) — capstone + production intents** — Apply only when `DESIGN.md` § Project intent declares `**Phase 6 strategy:** planned`. For projects whose Phase 6 strategy is `not applicable`, this dim records the declaration's presence as Demonstrated and skips substantive evaluation; for projects whose Phase 6 strategy is `planned`, evaluate the final VDD-IAR Alignment review round titled "Review N — Phase 6 four-dimensional convergence (project-terminal)" in `vsdd-suite/review-log/<close-date>-vdd-iar-alignment.md` (indexed in `vsdd-suite/VDD-IAR-ALIGNMENT-REVIEW.md`) against:
    - **Per-dimension citation specificity** — each of the four dimensions (Spec / Test / Implementation / Formal-verification) is established by a specific cited artifact (review log entry, mutation-test report, harness file), not by narrative recall. "The spec is at MVR because no spec gaps were raised in the last few rounds" is weak; "the spec is at MVR because SO Review N at Layer L produced only Hallucinated findings AND Phase 4 routing across Rounds N+1..M produced no `route:phase-1a+1b` destinations" is strong (per `primers/6-convergence.md` § Dimension 1 example).
    - **Cross-dimension consistency check completeness** — the convergence round's consistency-check table has one row per spec-named behavior and zero inconsistent rows at convergence-declaration time. Inconsistencies surfaced during the check must be routed via Phase 4 and the routed work landed before the convergence round signs.
    - **Out-of-scope dimensions explicitly named** — projects closing on three of four dimensions (e.g., formal-verification declared `not applicable`) list the skipped dimension(s) by name with the originating strategy declaration cited. Silent omission is a discipline gap.
    - **Convergence attestation signed + dated** — the final round's closing block ends with a signed/dated attestation from the convergence-round author. An unsigned round has not been ratified.
    - **Cost-of-production audit signal** — a Phase 6 round that takes 5 minutes to produce is weaker than one that takes 30+ minutes (per-dimension citation, per-behavior consistency check, explicit out-of-scope declarations). The audit signal is observable in the commit's content density: a 5-line round with vague citations vs. a multi-page round with per-behavior rows is the difference between a discipline-applied closure and a checkbox closure.

## Program Phase Context

**Note:** "Phase" in this section refers to the apprentice program progression tier (Phase 1 apprentice, Phase 2 apprentice, etc.), not to the VSDD pipeline phases (1a, 1b, 2a, 2b, 3, 4, 5, 6). The two numbering systems are distinct. Tool requirements and process expectations scale with program tier. When evaluating a project, establish its program tier before applying this domain:

- **Phase 1** — Crosslink not yet introduced. Issue tracking compliance (dim 11) is not applicable. Evaluate all other dimensions.
- **Phase 2** — Crosslink introduced. Issue tracking compliance is required from the first Phase 2 project onward. Retroactive application to Phase 1 projects is not appropriate.
- **Phase 3** — Crosslink required. Expect mature session discipline and bead-string structure.
- **Phase 4 (capstone)** — All Phase 3 requirements plus: release binaries, external user validation, and a retrospective that addresses the full arc from Phase 1 to capstone.

Do not penalize a Phase 1 project for absent crosslink usage. Do not waive crosslink requirements for a Phase 2+ project because the project "predates" the requirement — the requirement scales with phase, not with project start date.

---

Review entries are logged in per-session files at `vsdd-suite/review-log/YYYY-MM-DD-vdd-iar-alignment.md` inside the project being reviewed; the per-domain index at `vsdd-suite/VDD-IAR-ALIGNMENT-REVIEW.md` aggregates rounds (newest-first) and is the entry point for browsing the domain's review history. See `vsdd-suite/suite-development/suite-development.md` § Governing standard for project-level review logs.
