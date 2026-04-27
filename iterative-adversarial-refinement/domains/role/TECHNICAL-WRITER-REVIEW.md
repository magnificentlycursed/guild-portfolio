# Technical Writer Review

This review is part of the [Iterative Adversarial Refinement (IAR)](../../README.md) suite. It may be run independently or alongside other domains. See [README.md](../../README.md) for sequencing, scoped runs, and domain coordination.

**Reviewer role: Technical Writer** (Technical Writer / Developer Experience Engineer)

The purpose of this review is to evaluate whether the project is understandable, maintainable, and handoff-ready — whether to another developer, a client, or future-you without access to the original AI sessions. The reviewer brings the lens of a technical writer: evaluating documentation as a standalone artifact that must work without its author present. Documentation fails in two ways: absent (nothing written) and inaccurate (written once, then the code changed). Both are failures. An AI agent generates documentation in parallel with code from the same prompt interpretation, which means both can be consistently wrong in the same direction, and documentation tends to become stale as code evolves.

This domain covers: project documentation (README, architecture docs, decision records), inline code documentation (comments, docstrings, API docs), operational documentation (runbooks, setup guides, deployment procedures), and knowledge transfer quality (could someone new use this project productively?).

This domain is related to but distinct from SE dims 11–16 (future-self maintainability and the Documentation extended section), which address documentation briefly as part of the implementation review. This domain applies sustained adversarial pressure to documentation as a first-class artifact.

## Current Review Prompt

**Scope:** All documentation artifacts — README, DESIGN.md, DECISIONS.md, PROCESS.md, CHANGELOG, inline comments and docstrings, runbooks, and any other documentation files.

Read DESIGN.md first to understand the project's intended scope. Then evaluate all documentation against the standard below.

For each finding, cite the specific file and section. Classify as **resolved** (fix applied this review), **deferred** (scheduled for a specific layer, reason given), **dismissed** (no action taken, rationale required), or **hallucinated** (the adversary invented a problem that does not exist — push back is warranted. Consistent hallucinated findings are the maximum viable refinement signal).

**Coordination:** Flag findings that overlap with SE (stale or incorrect inline comments), SA (architectural decisions without documented rationale), VDD-IAR Alignment (DECISIONS.md quality, retrospective honesty), and PE (runbook completeness, deployment procedure accuracy).

**Sycophancy check:** An agent generating documentation in the same session as code will produce documentation that is accurate at the moment of generation and stale after the next change. The adversary must verify that documentation describes the current implementation, not the implementation at the time it was written. Every claim in the documentation should be verifiable against the current code. Treat every "this function does X" statement as a claim that requires verification.

**Language and interface supplement:** Consult `../../lang/` for language-specific documentation tooling (e.g., `rustdoc`, TypeDoc, JSDoc, Sphinx).

## Standard Evaluation Dimensions

1. **README completeness** — Can someone new to the project understand what it does, how to install dependencies, how to run it, how to run the tests, and how to contribute — from the README alone, without the original AI conversation? Named checks: project purpose in one paragraph; prerequisites listed explicitly (runtime version, system dependencies); setup instructions that work from a clean checkout; test run command; known limitations or gotchas. The test: clone the repo into a fresh environment and follow the README. If any step fails, the README is incomplete.

2. **Documentation accuracy** — Does the documentation describe the current implementation? Read documentation claims against the code they describe. Named failure modes: README examples that use a command or API that no longer exists; inline comments that describe removed behavior; DESIGN.md features that were not implemented; function docstrings that describe the previous signature. Stale documentation is actively harmful — it misleads rather than informs.

3. **Architecture documentation** — Are the key structural decisions documented in a durable form outside of conversation history? Named content: module boundaries and their responsibilities; data flow; the purity boundary (if applicable); technology choices and the rationale for each; what was explicitly ruled out and why. A future developer (or future-you) should be able to understand why the system is structured as it is without reverse-engineering the code.

4. **Decision rationale** — Are significant decisions recorded with their rationale, not just their outcome? A DECISIONS.md or equivalent that says "chose localStorage" without saying "because the application is single-user, offline-capable, and requires no server infrastructure" is an outcome record, not a decision record. The rationale is what a future developer needs to evaluate whether the decision still applies.

5. **Inline comment quality** — Do inline comments explain *why*, not *what*? `// increment i` adds nothing. `// +1 to skip the header row` explains a non-obvious choice. Named failure modes: comments that duplicate the code (`// call validateUrl`); stale comments that describe removed behavior; TODOs that have been in the codebase for multiple layers without resolution. Non-obvious logic without a comment is also a finding — the adversary should flag code where the intent is not recoverable from the code alone.

6. **API and interface documentation** — Are public functions, types, and modules documented for callers? Named checks: exported functions have docstrings that describe inputs, outputs, and error conditions; data types have field-level documentation for non-obvious fields; the public interface surface is documented independently of the implementation. For library projects, this is the primary deliverable alongside the code itself.

7. **Operational documentation** — Can the application be set up, deployed, and operated from documentation alone? Named content: environment variable reference (name, purpose, example value, required vs. optional); deployment procedure (step by step, not "deploy to your preferred host"); local development setup (including any toolchain setup, database initialization, seed data); known failure modes and recovery steps. For portfolio projects, this may be a brief section; for any deployed project, it is required.

8. **CHANGELOG quality** — Is there a CHANGELOG that accurately records what changed, when, and why? Named checks: entries are dated; entries distinguish features, fixes, and breaking changes; entries reference the IAR rounds that drove them (for VDD projects); entries are written from the perspective of a user or caller, not a developer ("Fixed: delete button no longer visible while edit form is open" not "Removed conditional rendering bug in handleDeleteVisible"). A CHANGELOG written only for the developer is not a CHANGELOG — it is a personal commit log.

9. **Knowledge transfer test** — Could a developer who has never seen this project make a meaningful, correct change in one day using only the documentation? Apply this as a thought experiment: pick a simple, in-scope feature that is not yet implemented. Could a new developer implement it correctly from the README, DESIGN.md, and inline documentation? If they would need to read the entire codebase and reverse-engineer the patterns, the documentation has failed at knowledge transfer.

10. **AI session independence** — Is the knowledge required to understand and maintain this project documented in the project artifacts, or does it exist only in AI conversation history? Named failure modes: "we decided to use this approach in the session where we built Layer 3" — documented nowhere; architectural decisions that are only recoverable from git log messages; constraints that exist in the code but are not documented ("this function is always called after X" with no assertion or comment). A project whose maintenance requires access to its build sessions is fragile.

---

Review entries are logged in `iterative-adversarial-refinement/TECHNICAL-WRITER-REVIEW.md` inside the project being reviewed.
