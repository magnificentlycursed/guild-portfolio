# 2026-05-01 Meta-Reviews

## Review 16 — 2026-05-01 00:00Z

**Scope:** VDD-IAR and VSDD alignment review of all session primers in `prompts/`. Read: `suite-development.md` (governing standard), `spec-crystallization.md`, `decomposition.md`, `implementation.md`, `review-session.md`. Triggered by user request during Layer 2 implementation session.

**Lens:** Primer structure conformance to the governing standard (`suite-development.md`). The governing standard requires the `## Prompt` section to establish: AI's role, governing constraint, and primary failure mode to watch for. Also checked for VSDD phase coverage gaps, completion criteria structure, and cross-primer consistency.

**Session context:** Same session as Layer 2 implementation (issue-tracker-cli). This is a same-session review — the agent reviewed prompts it has used in prior sessions and will use again. Sycophancy risk is elevated; apply extra pressure to any finding that trends toward "it's fine."

---

### Resolved

**Finding 1 — `decomposition.md` `## Prompt` section missing explicit primary failure mode.**

The governing standard requires the `## Prompt` section to establish: AI's role, governing constraint, and **the primary failure mode to watch for**. `decomposition.md` establishes role ("You are helping decompose...") and constraints ("every layer must be independently verifiable," "push back on layers that are too large") — but never names the specific failure mode a decomposing AI is prone to.

The failure mode is: accepting the developer's proposed layer structure without challenge. An agent that approves all proposed layers is transcribing, not decomposing. This is the decomposition-phase equivalent of the sycophancy warning in `review-session.md`, and it is missing. A cold session running only the `## Prompt` section would not know what failure pattern to resist.

**Resolution:** Added "Primary failure mode" paragraph to the `## Prompt` section of `decomposition.md` naming the failure class explicitly.

---

**Finding 2 — `implementation.md` Phase 2b guidance creates an unresolvable Red Gate paradox for retroactive tests.**

Phase 2b item 2 instructs: "If you discover a missing test, note it; add it in a separate commit after the current feature is working, so the Red Gate record is clean." But completion criterion 4 requires: "No implemented behavior exists that has no test covering it." And the Red Gate principle requires tests to be in a failing state before implementation.

These three requirements are irreconcilable for a retroactive test: the implementation exists before the test is written, so the test cannot fail before the implementation exists. The primer creates a logical impossibility and provides no guidance on how to resolve it. A developer following the primer strictly would:
1. Discover a missing test during implementation
2. Add it "after the current feature is working" (per item 2)
3. Now have a test that passes immediately and never failed — a Red Gate violation
4. But still be required to add it to satisfy completion criterion 4

No guidance exists for how to label, log, or evaluate this case. A VDD-IAR Alignment reviewer looking at the commit would see a test commit after implementation with no explanation.

**Resolution:** Updated Phase 2b item 2 to explicitly name this case as a **Red Gate deviation**, require it to be labeled as such in the commit message and review log, and specify what "confirmed passes against current implementation" means. The deviation is not forbidden — it is a known limitation that must be surfaced, not silently patched.

---

**Finding 3 — `spec-crystallization.md` completion criteria embedded in `## Self-adversary check` section, not in a dedicated section.**

`implementation.md` has `## Completion criteria`. `decomposition.md` has `## Completion criteria`. `spec-crystallization.md` embeds its only completion criterion ("The spec is ready to move to Phase 1b when you cannot find an undefined behavior...") as the final sentence of the `## Self-adversary check` section. A developer scanning the primer for the gate condition would not find it where it appears in every other primer.

The embedded sentence is also weaker than it should be — it names only the adversarial pressure test without listing the structural completeness criteria (behavioral contracts, edge case catalog, interface definitions, verification architecture, out-of-scope section). These criteria exist throughout the driving questions section but are never assembled as a gate checklist.

**Resolution:** Added a dedicated `## Completion criteria` section to `spec-crystallization.md` with six numbered criteria drawn from the VSDD Phase 1 standard (behavioral contracts, edge cases, interface definitions, verification architecture, out-of-scope, and the adversarial pressure condition). The embedded completion sentence in `## Self-adversary check` removed to avoid duplication.

---

### New gap registered

**G-86 — No Phase 4 (Feedback Routing) session primer.**

VSDD Phase 4 is described in VDD-IAR Alignment dim 7 as the step where IAR findings route back to the appropriate earlier phase: spec findings → DESIGN.md updates; test-coverage findings → new or revised tests; implementation findings → code changes. The dim explicitly warns against patching findings at the wrong level.

No session primer exists for Phase 4. When a developer receives classified IAR findings and must act on them, they have no primer establishing: their role, what "correct routing" means, the primary failure mode (patching a spec gap only in code), or what a correctly routed correction looks like. The `review-session.md` classification schema identifies findings as resolved/deferred/dismissed/hallucinated but does not guide the developer on how to act on them after the session ends.

Current coverage of Phase 4: only dim 7 of VDD-IAR-ALIGNMENT-REVIEW.md. A developer with a "resolved" finding could close it as a code comment rather than a DESIGN.md change, and no primer would tell them that's wrong. Registered as G-86 — deferred to a future session; creating a Phase 4 primer is a new artifact, not a dimension patch.

---

### Hallucinated

**Finding 4 — `suite-development.md` H1 missing VSDD phase annotation.**

The governing standard says `# Session Primer: [Phase or Session Type] (VSDD Phase N)` **where applicable**. Suite development is not a VSDD project phase — it is a suite maintenance activity. The "where applicable" clause explicitly covers this exemption. No annotation is required or appropriate for a meta-activity that operates outside the VSDD pipeline. **Hallucinated.** The primer is correctly titled.

**Finding 5 — `review-session.md` domain selection placeholder not appropriate for a "paste-ready" primer.**

The primer says "Paste it into a cold session" and contains `Active domains for this project: *(list the domains active for this project...)*`. A strict reading says the placeholder means the primer is not paste-ready. But the purpose statement clarifies: "this primer establishes adversarial posture before loading any domain prompt." The domain list is project-specific context, not primer content. The placeholder is appropriate design intent — it marks where the human must add context before the session, which is the right behavior. **Hallucinated.** The design intent is sound; the concern was about mechanics, not substance.

---

## Review 15 — 2026-04-27 09:00Z

*Note: The date stamp in this entry is incorrect. Finding 6 (the follow-up) was added during the 2026-05-01 session; the entry is filed here with Review 16.*

**Scope:** Generalist adversarial pass. Read: `suite-development.md`, `review-session.md`, `SOLUTION-ARCHITECT-REVIEW.md`, `SECURITY-REVIEW.md`, `README.md`. Triggered by user request.

**Lens:** README Focus column accuracy and SA coordination link completeness. Previous passes had addressed classification schema gaps, domain format issues, and lang supplement additions — this pass checked whether the README domain table reflected the scope expansions made in Gap Analysis Runs 12 and 13.

---

### Resolved

**Finding 1 — `README.md` Security Engineer Focus column omits dims 7 and 8.**

Security dims 7 (Audit logging) and 8 (Data classification and control requirements) were added in Gap Analysis Run 12. The README core domain table Focus cell for Security Engineer still described only the pre-Run-12 scope: "Input handling, persistence data validation, dependency CVEs, secret handling, information exposure, authentication and authorization." A reviewer reading the README to select domains would not know audit logging or data classification coverage was present.

**Resolution:** Appended "audit logging, data classification and control requirements" to the Security Engineer Focus cell in the README core domain table.

**Finding 2 — `README.md` Solution Architect Focus column omits external service integration.**

The `### Extended: External Service Integration` section (dims 23–27) was added to SA in Gap Analysis Run 12. The README core domain table Focus cell for Solution Architect ended at "external interface contracts" — no mention of the new Extended section. A reviewer scanning the table would not know SA covers external service dependencies (inventory, failure handling, API drift, credential management, data transmission).

**Resolution:** Appended "external service integration" to the Solution Architect Focus cell in the README core domain table.

**Finding 3 — `SOLUTION-ARCHITECT-REVIEW.md` coordination links omit Privacy.**

SA dim 27 (data transmitted to external services) contains an explicit cross-reference to `PRIVACY-REVIEW.md` dim 6. SA's coordination section listed QE, UX, Security, PE, and DE — but not Privacy. A reviewer following only the coordination note would not route dim 27 data-transmission findings to Privacy even though the dimension text instructs it.

**Resolution:** Added Privacy to SA coordination links with scoping note: "dim 27 — data transmitted to external services; cross-reference with Privacy dim 6 when Privacy is active."

**Follow-up finding after session (resolved same session):**

**Finding 6 — `implementation.md` Phase 2a missing explicit commit requirement before Phase 2b.**

During Layer 2 implementation of issue-tracker-cli, Phase 2a was completed correctly (tests written, all confirmed failing in the working tree) but implementation began before the Red Gate state was committed. The result was a commit labeled "Red Gate — tests and stubs" that actually contained real implementations — the commit message was false. VDD-IAR Alignment dim 4 would have no way to verify test-first discipline from the commit history.

The Phase 2a section had three steps (write tests, confirm they fail, confirm the failure reason) but no step requiring the Red Gate commit before Phase 2b begins. The Phase 2b transition said "Once the Red Gate is set and every new test is confirmed failing" — which a developer could satisfy entirely within a single session without ever committing the failing state.

This is the same failure mode that VDD-IAR Alignment dim 4 detects from the outside: "implementation commits consistently precede any test commits." Here it was the process document itself that failed to prevent it.

**Resolution:** Added step 4 to Phase 2a: "Commit the Red Gate state before Phase 2b begins. The commit is the boundary between phases — every file change after it is implementation. If implementation begins before this commit, the commit history cannot distinguish test-first from test-after, and VDD-IAR Alignment dim 4 cannot be verified." Updated Phase 2b transition to reference "set, confirmed failing, and committed."
