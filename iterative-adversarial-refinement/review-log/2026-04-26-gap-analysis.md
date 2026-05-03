# 2026-04-26 Gap Analysis Runs

## Gap Analysis Run 4 — 2026-04-26 00:00Z

**Context:** Personal developer, AI-accelerated workflow, portfolio-to-journeyman arc. Full evaluation of suite against the guild apprentice-onboarding methodology, the bookmark-manager project history (including the dollspace-gay guild review finding), and the upcoming issue-tracker-cli project. Goal: identify gaps that would cause the suite to pass a project that a guild portfolio reviewer would fail.

**Suite state at time of run:** Eight domains, all with sycophancy checks and language/interface supplements. lang/ subfolder with rust.md, javascript-typescript.md, cli.md, browser-app.md. GAP-ANALYSIS-LOG.md with 38 gaps registered.

### Findings

**G-39 — DESIGN.md fitness check missing (High)**

The SO domain treats DESIGN.md as the authoritative contract and checks whether the implementation matches it. But DESIGN.md is a student document, and it can itself be wrong. The bookmark-manager's DESIGN.md specified TypeScript, Vite, and a build toolchain — none of which were asked for by the assignment. Every SO dimension passed. The guild reviewer failed the project on assignment compliance.

The suite had no mechanism to catch scope creep that entered at the design stage, only scope creep that entered during implementation. The upstream assignment brief is the higher-level contract.

*Decision:* Add SO dim 10 (assignment compliance). Requires reading the assignment instructions alongside DESIGN.md and flagging deviations that entered at the design stage.

**G-40 — VDD process fidelity unowned (High)**

The guild portfolio review is explicitly "process over product." Reviewers look at whether the VDD loop was followed: design doc before code, layered development, IAR at each gate, tests alongside or before implementation. Nothing in the suite checked this. A project that is correctly built but built without process discipline would pass all eight domains and still fail a portfolio review.

*Decision:* Add SO dim 11 (VDD process fidelity). The SO domain is the right owner — it already guards the spec contract, and process fidelity is the meta-contract above it.

**G-41 — No MVR exit signal or hallucinated finding classification (High)**

The methodology document describes a specific exit condition: when the adversary starts hallucinating critiques because it cannot find real ones, the code has reached maximum viable refinement. This is the most important signal in the VDD loop. The suite had no classification for hallucinated findings, no guidance on recognizing the MVR signal, and no way to record that exit in the log.

This meant a reviewer running the suite could not distinguish: (a) a clean pass because the code is good, (b) a clean pass because the agent was too agreeable, or (c) an exit because the adversary ran out of real complaints. These are three very different states.

*Decision:* Add **hallucinated** classification to all 8 domain prompts. Add MVR exit signal explanation to README.

**G-42 — Manual testing checklists not owned (Medium)**

The bookmark-manager DESIGN.md included explicit manual testing checklists as part of the layer gate. The DECISIONS.md documents three cases where manual testing caught things automated tests missed (tag toggle deselect, empty URL message, focus after cancel). No domain asked whether manual checklists existed or were completed. This gap is especially significant for CLI projects where automated tests cannot cover all UX concerns.

*Decision:* Add QE dim 14 (manual testing checklists).

**G-43 — Commit history and linear accountability not evaluated (Medium)**

The methodology's "string of beads" principle: every piece of code traces to a sub-issue, every sub-issue to an issue. The portfolio review looks at commit history. A commit log of "fix stuff" or "wip" is a process failure. No domain evaluated commit message quality or traceability. A project could have excellent code and a useless commit history.

*Decision:* Add SO dim 12 (linear accountability).

**G-44 — Same-session sycophancy drift across domains (Medium)**

Each domain has a sycophancy check. But if all eight domains run in the same AI session, the agent accumulates context that softens its adversarial pressure. The methodology's "fresh eyes every time" principle argues for resetting context between rounds. The suite had no guidance on session isolation between domain reviews.

*Decision:* Add session isolation operational note to README under Full run.

**G-45 — Portfolio-arc perspective absent (High for personal use)**

Per-project IAR runs evaluate individual projects. The portfolio review evaluates the arc across all projects: growth, honest retrospectives, independence, assignment alignment patterns. The suite had no guidance for this cross-project perspective. A student who ran excellent IAR on each project but never assembled the arc-level view would miss what portfolio reviewers actually evaluate.

*Decision:* Add portfolio-arc review section to README.

### Suite changes made as a result of this run

**G-39 addressed** — SO dim 10 (assignment compliance) added.
**G-40 addressed** — SO dim 11 (VDD process fidelity) added.
**G-41 addressed** — **hallucinated** classification added to all 8 domain prompts. MVR exit signal explanation and session isolation note added to README.
**G-42 addressed** — QE dim 14 (manual testing checklists) added.
**G-43 addressed** — SO dim 12 (linear accountability) added.
**G-44 addressed** — Session isolation note added to README under Full run.
**G-45 addressed** — Portfolio-arc review section added to README.

**Remaining open:** G-34, G-36 (deferred from Run 3). G-01 through G-32 remain open; most are scoped to contexts (mission-critical teams, consulting engagements) not yet relevant to the current personal portfolio use case.

---

## Gap Analysis Run 5 — 2026-04-26 01:00Z

**Context:** Organizational evaluation of the suite's structure and alignment with VDD-IAR. Prompted by the question: is this the right organization? Are additional domains needed? Does the suite reflect what VDD-IAR actually is?

**Suite state at time of run:** Nine domains after this run (previously eight). README restructured. VDD-IAR Alignment domain created. SO stripped to spec-contract identity.

### Findings

**G-46 — SO split identity (High)**

The SO domain had accumulated two distinct adversarial postures: spec-contract review (does the implementation match what was asked?) and process governance (was the work done correctly?). These read different artifacts, apply different adversarial frames, and belong in separate sessions. Having them in one domain created a reviewer that had to context-switch mid-review.

*Decision:* Strip SO to spec-contract focus (9 dims). Move process concerns to VDD-IAR Alignment. Move complexity budget for one to SA.

**G-47 — Suite described as gate, not iterative loop (High)**

The README described IAR as a pre-merge gate: "a full run is required before merging." VDD-IAR is a loop: build → adversary → fix → adversary again → repeat until MVR. The suite name says "Iterative" but the documented structure was a single checkpoint. No guidance existed for within-layer iteration, round numbering, or when to stop iterating.

*Decision:* Rewrite README. Replace "Full run" with "Refinement loop" section. Add round-number requirement to log format. Update merging gate to require MVR, not just one passing run.

**G-48 — QE/SE domain overlap without explicit boundary (Medium)**

SE dim 1 (correctness, logic errors) and QE dim 7 (logic errors) covered the same ground. In practice both reviewers would find the same bugs. The distinction — QE owns tests, SE owns code — was valid but unstated, creating duplicated effort without the benefit of independent confirmation.

*Decision:* Add domain boundary statements to QE and SE prompts. QE flags missing tests when it finds a logic error; SE flags the bug. Both findings are valid independently.

**G-49 — PE posture misrepresented (Low)**

The generic sycophancy check ("if the agent agreed with every decision...") doesn't fit a domain where most dimensions are binary existence checks. The real sycophancy risk in PE is rationalized inapplicability decisions, not agreeing with code quality judgments.

*Decision:* Replace generic sycophancy check with a posture note specific to PE's compliance-check nature. Scope the adversarial pressure to judgment-dependent decisions.

**G-50 — No generalist adversary pass (Medium for personal use)**

The IAR specialists each apply a specific framework. The VDD methodology's adversary has no framework — it just finds everything wrong. No domain covered the gaps between specialist frameworks.

*Decision:* Document as an optional unstructured pass in the README (not a formal domain). It intentionally has no dimensions — adding structure would make it another specialist.

**G-51 — VDD-IAR Alignment domain missing (High)**

Process compliance had no owner. Test discipline, layer gate compliance, IAR fresh context, IAR iteration, role integrity, and retrospective quality were either scattered across SO (awkwardly) or unowned. The methodology's "process over product" principle had no adversarial review mechanism.

*Decision:* Create VDD-IAR-ALIGNMENT-REVIEW.md. Ten dimensions covering the full VDD-IAR loop. Runs last in the sequence (reviews artifacts produced by all other domain runs). Mandatory gate before merge.

### Suite changes made as a result of this run

**G-46 addressed** — SO reduced to 9 dimensions (spec/contract focus). Dims 9 (complexity for one), 11 (VDD fidelity), 12 (linear accountability) removed and redistributed. Complexity budget for one moved to SA dim 9 expansion.
**G-47 addressed** — README rewritten around VDD-IAR as the governing framework. "Refinement loop" replaces "Full run." Merging gate updated to require MVR and round numbers.
**G-48 addressed** — Domain boundary notes added to QE and SE prompts.
**G-49 addressed** — PE sycophancy check replaced with a posture note specific to compliance-check domains.
**G-50 addressed** — Generalist adversary pass documented as optional README note.
**G-51 addressed** — VDD-IAR-ALIGNMENT-REVIEW.md created. Added to domain table, sequencing, and merging gate.
