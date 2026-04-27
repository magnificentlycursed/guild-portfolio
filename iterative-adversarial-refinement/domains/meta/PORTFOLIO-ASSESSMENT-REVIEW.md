# Portfolio Assessment Review

This review is part of the [Iterative Adversarial Refinement (IAR)](../../README.md) suite. It may be run independently or alongside other domains. See [README.md](../../README.md) for sequencing, scoped runs, and domain coordination.

The purpose of this review is to evaluate the work as a portfolio artifact — not whether it functions correctly, but whether it demonstrates genuine skill, learning, and independent judgment by the developer who built it. An AI-accelerated project that passes every technical domain may still fail as a portfolio piece if the developer cannot explain the decisions, did not direct the work, or does not understand the implementation well enough to own it.

This domain addresses G-34: the learning and craft development gap identified in Run 3. It applies specifically to portfolio projects, apprentice program submissions, and any work that will be evaluated as evidence of developer capability. It is not appropriate for production work or consulting deliverables — those are evaluated on outcomes, not on the learning process.

## Current Review Prompt

**Scope:** The whole project, including process documentation, commit history, DESIGN.md, DECISIONS.md, PROCESS.md, and the developer's stated learning objectives.

This review requires the developer's active participation. Some dimensions require direct questions to the developer rather than artifact analysis. The developer should answer as if explaining the project to a technical interviewer who has read the code.

For each dimension, classify as **demonstrated** (evidence is clear and specific), **partial** (evidence exists but is thin or indirect), **absent** (no evidence found), or **hallucinated** (the adversary invented a concern that does not apply — push back warranted).

**Sycophancy check:** This is the domain where sycophancy does the most harm. An agent reviewing portfolio work has every incentive to find it impressive — it helped build it. The adversary must push on the hardest question in this domain: could this developer reproduce the key decisions without the AI? That question cannot be answered by reading the code. It requires direct interrogation.

**Coordination:** Process evidence connects to VDD-IAR Alignment (was the methodology followed?). Decision ownership connects to SA and SE (are the architectural and implementation decisions the developer's?). Documentation connects to SE (is the knowledge transferred? See the Documentation section of SE).

## Standard Evaluation Dimensions

1. **Decision ownership** — Can the developer explain each significant design decision without referring to the AI conversation? Named decisions to probe: the technology choices in DESIGN.md, the layer breakdown structure, the data model design, the key architectural boundaries. The test is not "do you know what the decision was" but "do you know *why* it was made, and would you make the same decision again?" If the developer's answer is "the AI suggested it and it seemed right," the decision is not owned.

2. **Implementation understanding** — Can the developer explain how any part of the codebase works at the code level? Pick 3 non-trivial functions from different areas of the codebase. Ask the developer to explain: what it does, what assumptions it makes, what would break if a specific line were changed. The ability to explain code the developer directed an AI to write is the minimum bar; the ability to explain *why* it is written that way is the target.

3. **Directed development evidence** — Does the commit history, DESIGN.md, and DECISIONS.md show that the developer directed the work, or did the agent make all the decisions? Named indicators of developer direction: DESIGN.md contains constraints that are non-obvious and came from the developer's specific experience or preferences; commit messages show deliberate choices; DECISIONS.md entries show the developer pushing back on agent defaults; PROCESS.md acknowledges where the developer made the wrong call. Named indicators of passive rubber-stamping: every decision in DESIGN.md is a reasonable default; no decisions show evidence of the developer's specific context or preferences; no agent defaults were ever rejected.

4. **Growth evidence** — Does the project documentation show genuine learning, not just output? Named indicators: PROCESS.md that acknowledges specific mistakes made and lessons learned; DECISIONS.md entries that show a developer changing their mind after new evidence; known issues that are honestly assessed rather than minimized; a retrospective that would embarrass a developer who did not actually build the project (because they could not have learned those lessons). A process document that reads as a success story with no friction is evidence against genuine engagement.

5. **Failure and recovery honesty** — Did the developer experience any failures, bugs, or wrong turns during development? Can they describe what happened, why it happened, and what they would do differently? An AI-generated first attempt rarely has serious failures because the AI produces a consistent implementation. A developer who genuinely engaged with the project should have at least one story about something that did not work as expected.

6. **Spec ownership** — Does the developer understand the spec they wrote well enough to evaluate a proposed change? Present a hypothetical: "A user asks for feature X (something plausible but not in DESIGN.md). Should it be added? Why or why not?" The developer should be able to evaluate the request against the spec's constraints, out-of-scope list, and design principles — not just "the AI didn't build that."

7. **Extensibility confidence** — Could the developer implement a modest in-scope extension without AI assistance? This is not a test of speed or correctness under pressure — it is a test of familiarity. If the developer would need to re-read the codebase from scratch to understand where to add a new feature, the code is not owned.

8. **Appropriate scope judgment** — Does the scope of the project reflect the developer's own judgment, or the agent's defaults? Named concerns: the project implements features that were not asked for because the agent added them; the project's technical stack is more complex than the assignment required; the project's documentation is professional-quality but the developer cannot explain what it says. Appropriate scope for a portfolio project means: the developer could have built this without AI assistance, even if it would have taken longer.

---

Review entries are logged in `iterative-adversarial-refinement/PORTFOLIO-ASSESSMENT-REVIEW.md` inside the project being reviewed.
