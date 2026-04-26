# VDD-IAR Alignment Review

This review is part of the [Iterative Adversarial Refinement (IAR)](README.md) suite. It may be run independently or alongside other domains. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

The purpose of this review is to evaluate whether the project was built using the Verification-Driven Development (VDD) and Iterative Adversarial Refinement (IAR) methodology. The other IAR domains evaluate *what* was built. This domain evaluates *how* it was built — whether the process that is supposed to produce quality actually ran.

The reference for this domain is the governing VDD-IAR methodology document. For guild projects: `apprentice-onboarding/02-the-methodology/01-how-we-build.md`. Treat it as this domain's DESIGN.md equivalent: the adversary holds the project's process against the methodology, not against the code.

## Current Review Prompt

**Scope:** The full development history of the project — commit log, DECISIONS.md or equivalent, IAR review logs, layer gate records, and DESIGN.md. Code is read for evidence of process compliance (e.g., do tests appear before or after implementation commits?) rather than for correctness — that belongs to QE and SE.

Read the governing methodology document in full before reviewing any artifacts. Then read: DESIGN.md, the commit history, all IAR review logs, and any retrospective or decisions documents. Apply every standard dimension below as a floor.

For each finding, cite the specific artifact and location (commit hash, log entry, file and line). Classify as **resolved** (fix applied this review), **dismissed** (no action taken, rationale required), or **hallucinated** (the adversary invented a process failure that does not exist — push back is warranted. Consistent hallucinated findings are the maximum viable refinement signal: real issues have been exhausted).

**Coordination:** Process failures frequently explain defects found by other domains. If QE found that tests were added after implementation, flag it here as a test discipline finding. If SA found that architecture grew organically rather than being designed, flag it here as a decomposition finding. Coordinate with [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) on assignment compliance — SO owns whether DESIGN.md matches the assignment brief; this domain owns whether design-before-code discipline was followed. If this review suggests the need for a new IAR domain, log it as a finding.

**Sycophancy check:** Process failures are easy to rationalize. The agent reviewing this domain is likely the same agent that participated in building the project — it has every incentive to find the process acceptable. The absence of a layer gate record is not ambiguous. Batched test commits are not ambiguous. A single IAR pass that merged immediately after real findings is not ambiguous. Push back on any dimension where the agent reaches for benefit-of-the-doubt rather than evidence.

## Standard Evaluation Dimensions

1. **Design-before-code** — Was DESIGN.md or an equivalent design artifact created before implementation began? Does the first commit establish design documentation rather than code? Flag any project where the design doc was written after the fact or retroactively expanded to justify implementation choices already made.

2. **Layered decomposition** — Was the project broken into explicitly bounded layers or iterations with defined acceptance criteria? Does the commit history reflect this structure with clear layer boundaries, or is development one undifferentiated mass of commits? Flag any layer that has no defined acceptance criteria or whose commits span multiple stated layers.

3. **Layer gate compliance** — Was each layer fully verified and gated before the next began? Is there evidence that: acceptance criteria were checked, tests passed, IAR ran, and the layer was explicitly closed before the next opened? A commit that introduces features from a new layer before the previous layer's IAR log is complete is a finding.

4. **Test discipline** — Were tests written alongside or before implementation? Look at the commit pattern: does a feature commit have corresponding tests in the same or immediately prior commit, or do tests arrive in a batch at the end of a layer? Test-after patterns are a yellow flag; a single commit adding all tests for an already-completed layer is a finding. Exceptions require explicit documented rationale.

5. **Human verification** — Is there documented evidence that the human director verified each layer against intent — a completed manual testing checklist, an explicit sign-off, or a layer gate record? AI-only verification is not sufficient. Code that the builder wrote and the adversary reviewed but the human never ran is not verified. The absence of any human verification artifact is a finding.

6. **IAR fresh context** — Were adversarial review rounds conducted with fresh AI context, or batched into the same session where the code was written? A log entry covering a review that occurred in the same session as the implementation is weaker. A single log entry spanning multiple layers without a context reset is a finding. Assess from log structure, session framing, and timing evidence.

7. **IAR iteration** — Were rounds iterated when findings were substantial? A single adversarial pass followed immediately by a merge is a finding if that pass produced real findings. The expected pattern: findings → fix → second pass with fresh context → repeat until MVR → merge. Check the review log for round numbers and finding progression.

8. **Role integrity** — Did the human director actually direct — define goals, set constraints, make judgment calls, push back on agent defaults — or did the agent make all decisions? Review DESIGN.md, decisions logs, and commit messages. A design doc that shows no evidence of human scoping choices, or a commit history where every decision traces to an agent default, is a finding. The human's fingerprints should be visible in the work.

9. **Manual testing checklists** — Does the project have explicit manual testing checklists per layer or feature? Were they completed and recorded before the layer gate closed? Automated tests verify correctness; manual testing verifies that the experience is coherent — interaction flows, error states that feel right, "technically correct but not what I meant" failures. For CLI projects especially, the human must run the binary and evaluate output quality. Absence of a manual checklist is a finding regardless of automated test coverage.

10. **Retrospective quality** — Is there a post-mortem, DECISIONS.md, or equivalent that honestly names: what went wrong, what was cut, what the agent got wrong, and what was learned? A document that records only successes is a finding — real projects have problems. The specificity and honesty of the retrospective is proportional to the quality of self-assessment demonstrated by the project.

---

Review entries are logged in `iterative-adversarial-refinement/VDD-IAR-ALIGNMENT-REVIEW.md` inside the project being reviewed.
