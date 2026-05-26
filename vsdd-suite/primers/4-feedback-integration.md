# Session Primer: Feedback Integration Loop (VSDD Phase 4)

**Frequency:** **Per Phase 3 round** — every IAR round produces a routing record per round, not per layer (the canonical primer-4-shape post-bookmark-cli-manual-PR-#52 is per-domain `## Phase 4 routing — Round N` appendices in each per-domain review log entry; enforced via [`suite-development.md` § Layer-gate close criteria](../suite-development/suite-development.md#layer-gate-close-criteria-processmd-retrospective-discipline) criterion 8).

**Whitepaper alignment ([Review 79](../suite-development/review-log/2026-05-20-suite-review.md#review-79--2026-05-20-1730z) Finding 1):** the [VSDD whitepaper](https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00) names this phase **"Feedback Integration Loop"** — the "Loop" qualifier is load-bearing; Phase 4 is the routing-back-to-earlier-phases activity that closes the IAR refinement loop. The primer was previously titled "Feedback Integration" (missing "Loop"); this primer aligns to the whitepaper's canonical name.

Use this prompt after a Phase 3 IAR round has produced a classified finding set and before the next implementation pass begins. The output of this session is a routed finding set — each real finding is assigned to the earliest VSDD phase that can correctly fix it, with the corresponding work scheduled there. Phase 4 closes the loop back to Phases 1a / 1b / 1c / 2a / 2b / 2c / 5; the loop repeats until MVR.

Do not start feedback integration with unclassified findings. A finding that has not been classified Resolved / Dismissed / Hallucinated / Accepted (etc.) does not have a routing decision yet — re-run the relevant Phase 3 domain to classify before routing.

---

## Prompt

You are helping route adversarial review findings under the Verified Spec-Driven Development (VSDD) methodology. This is Phase 4: Feedback Integration. Your role is to send each finding to the *earliest* phase that can fix it correctly — not the most convenient phase.

**Your posture:** A finding fixed at the wrong phase creates a hidden cost. A spec defect patched in implementation leaves the spec wrong; the next implementer or reviewer hits the same gap. A missing test patched by adding a single regression assertion leaves the test discipline broken; the next layer inherits a Red Gate that no longer asserts intent. Routing is the discipline that keeps each phase's artifact authoritative.

**Routing is not triage.** Triage prioritizes; routing relocates. Every real finding needs both: a priority (when to fix) and a phase (where to fix). A high-priority spec finding still routes to Phase 1a+1b; a low-priority implementation finding still routes to Phase 2b. Priority does not change the phase a finding belongs to.

**Primary failure mode:** Routing every finding to Phase 2b ("the implementation is what's wrong"). This collapses the VSDD pipeline into a single phase. The signal that this is happening: the spec hasn't changed in N rounds, the test plan hasn't changed in N rounds, but the codebase keeps churning. The fix is to re-examine the finding set and ask: of these findings, which are *actually* implementation defects, and which are spec or test defects misclassified as implementation?

---

## Finding-to-phase routing table

For each finding from the Phase 3 round, identify the trace and route accordingly. The trace question is: **what artifact, had it been correct, would have prevented this finding?**

| Finding signal | Trace | Route to | Why |
|---|---|---|---|
| Behavior is undefined in DESIGN.md and the implementation guessed | Spec gap | **Phase 1a+1b** | Patch the spec; subsequent implementations inherit the contract |
| Edge case is in DESIGN.md but no test covers it | Test discipline gap | **Phase 2a** | Add the failing test first, then fix |
| Test exists but doesn't actually assert the behavior (passes against empty body) | Test quality gap | **Phase 2a** | Rewrite the test to fail against a stub, then re-implement |
| Layer's acceptance criteria don't cover the failed behavior | Decomposition gap | **Phase 1c** | Re-decompose the layer; revise acceptance criteria; re-open the layer with a new Red Gate |
| Implementation diverges from a correctly-specified, correctly-tested behavior | Implementation defect | **Phase 2b** | Standard bug-fix flow |
| Refactor regressed clarity or surfaced a code-smell the Phase 2b commit did not have | Refactor regression | **Phase 2c** | Re-refactor or back out the regressing change; tests stay green throughout |
| Property-based counterexample, surviving non-equivalent mutant, fuzzer crash, failing proof harness | Hardening gap | **Phase 5** | Address per Phase 5 primer's per-surface anti-patterns; multi-phase if the root cause is upstream (e.g., counterexample → Phase 1a+1b for spec gap) |
| Spec/test/impl/formal inconsistency surfaced by the convergence check | Convergence gap | **Phase 6** | Route the inconsistent dimension's destination (typically 1a+1b or 2a); re-run the convergence check after the destination closes |
| Architectural concern crosses layers in a way DESIGN.md did not anticipate | Spec architecture gap | **Phase 1a+1b** | Spec the cross-cutting concern; may force re-decomposition |
| Suite gap (the adversary couldn't have caught this with current dimensions) | Suite gap | **Suite-development** (not a project phase) | File in `suite-development/FINDINGS-INDEX.md`; do not route to a project phase |
| Process gap (Red Gate was skipped, layer merged without IAR, etc.) | VDD-IAR Alignment finding | **Phase 4 itself** | Document the deviation; the fix is the next round's discipline, not a code change |

**Multi-phase findings:** A finding can route to more than one phase when the defect chain has multiple breaks. Example: an edge case that is both undefined in the spec AND uncaught by tests AND mishandled by code routes to Phase 1a+1b (spec), then Phase 2a (test), then Phase 2b (implementation), in that order. Each phase's fix unblocks the next. Recording the route as `1a → 2a → 2b` is the correct shape; recording it as `2b only` is the failure mode.

---

## Driving questions

For each finding marked Resolved-pending, Deferred, or Accepted-with-remediation in the Phase 3 log, ask:

1. **What artifact, if it had been correct, would have prevented this finding?** That artifact's owning phase is the route. If the answer is "the implementation" — verify, because that's the easy answer and the failure mode. Ask the same question one level up: "what artifact would have caused the implementation to be correct?" If the answer is "a more complete spec" or "a test that asserted this," route up.
2. **If we fix this only in implementation, will the next layer / next project hit it again?** If yes, the route is not Phase 2b. The next-layer-inheriting heuristic catches mis-routed spec findings reliably.
3. **Is there a corresponding suite gap?** If the adversary could not have caught this with the current domain dimensions, the finding has a Phase 4 routing decision (which earlier project-phase to patch) AND a separate suite-development action (file in `suite-development/FINDINGS-INDEX.md`). These are independent — do both.
4. **What is the gate for the routed work?** Routing to Phase 1a+1b means a spec revision; the gate is the self-adversary check from `primers/1ab-spec-crystallization.md`. Routing to Phase 1c means re-decomposition; the gate is the layer-structure rules from `primers/1c-decomposition.md`. Routing to Phase 2a means new failing tests committed before code; the gate is the Red Gate commit. Routing to Phase 2c means refactor-or-back-out work while every test stays green; the gate is the Phase 2c → 3 boundary (`crosslink swarm gate <slug>` or clean test-suite exit per `primers/2c-refactor.md`). State the gate when recording the route.

---

## Routing output

For each routed finding, record:

- **Finding ID** (from the Phase 3 review log)
- **Route** — `1a`, `1c`, `2a`, `2b`, `2c`, `5`, `6`, `Suite`, or multi-phase chain like `1a → 2a → 2b` or `5 → 1a+1b`
- **Owning artifact** — `DESIGN.md`, `TODO.md` / crosslink layer issue, `tests/<file>`, `src/<file>`, `suite-development/FINDINGS-INDEX.md`
- **Gate** — what must be true before the routed work is considered done at that phase
- **Sequencing** — does the route block the next layer? Block merge? Defer to a named future layer?

**Cluster identifier discipline (Review 94 Finding 3 codification).** When a routing pass groups multiple findings into a thematic cluster for cross-domain coordination, the cluster MUST be identified by a descriptive name (e.g., `JSON-native escape design cluster`, `sorted-tag-comparison dedup cluster`, `imported-tag control-char rejection cluster`), never by a bare letter or numeric code (`Cluster <letter>`, `Cluster <number>`). Letter labels are working-session shorthand; they propagate into permanent artifacts (DESIGN.md spec amendments, CHANGELOG entries, the routing record itself) where a future reader has no recovery path for what an opaque letter means. Mechanical enforcement via [`hooks/check-no-letter-clusters.py`](../hooks/check-no-letter-clusters.py) (Review 94) trips at commit-time on the forbidden patterns (`Cluster <letter>`, `Surface <letter>(.<digit>)?`, `Path <letter>`, `Option <letter>` — see the hook source for the exact regex set). Domain-level catches at Round 2 IAR review per [Technical Writer Dim 12](../domains/role/TECHNICAL-WRITER-REVIEW.md) (the lookup-cost detector authored at Review 78) + [AI Engineer Dim 7](../domains/role/AI-ENGINEER-REVIEW.md) (cluster-batching letter-coded-filename failure mode). The hook moves the catch earlier (commit-time) without weakening the Round 2 domain catches.

---

The suite supports two operational modes for Phase 4 routing — `[crosslink]` (recommended, when crosslink is installed) and `[manual]` (first-class fallback, when it is not). Both modes carry the same routing discipline; only the mechanism differs. Pick the mode that matches the project's setup; do not mix them within a single layer.

## [crosslink] — Recommended path

If the project uses crosslink, Phase 4 routing has tooling support:

1. **Findings filed as issues.** If Phase 3 ran via `crosslink swarm review --file-issues`, each finding already exists as a crosslink issue labelled `review-finding` (or the label your project uses). If Phase 3 ran manually, file the findings now: `crosslink issue create "<finding title>" -l review-finding -p <priority> --parent <layer-issue-id>`.
2. **Route via labels.** Add a phase-route label to each filed finding: `route:phase-1a`, `route:phase-1c`, `route:phase-2a`, `route:phase-2b`, `route:phase-2c`, `route:phase-5`, `route:phase-6`, `route:suite`. Multi-phase chains get the *first* phase as the label; the issue body records the full chain.
3. **Block downstream layers when appropriate.** If a Phase 1a+1b or 1c route invalidates a future layer's plan, mark the future layer blocked: `crosslink issue block <future-layer-id> <finding-id>`. Phase 4 is not done until all blocking relationships are recorded.
4. **Relate cross-domain findings.** When the suite's `**Coordination:**` line cross-references findings across domains (e.g., a UX finding whose fix depends on an SE finding's resolution), mechanize the link as a structured issue-graph edge: `crosslink issue relate <ux-finding-id> <se-finding-id>`. This preserves the coordination signal without requiring a human to grep prose.
5. **Schedule the fix work.** For Phase 2b routes, `crosslink swarm fix --from-label route:phase-2b --budget-aware` dispatches one fix agent per finding. For Phase 1a+1b / 1c / 2a / 2c routes, the fix is human-driven (or a single agent with `crosslink kickoff run` against the routing issue) — these phases require judgement that doesn't parallelize well.
6. **Close routed findings only when the routed work is gated.** A finding labelled `route:phase-1a` is *not* closed when the routing decision is made — it is closed when the spec revision passes its gate. Use the comment-then-close pattern (`issue close` does not accept `--comment`; the rationale lives in the prior comment): `crosslink issue comment <id> "Routed to 1a; DESIGN.md §<X> revised in <commit>; self-adversary check passed in <session>." --kind resolution && crosslink issue close <id>`
7. **Re-open if the gate fails.** If a routed Phase 1a+1b revision is itself flagged by Phase 3 in a subsequent round, the original finding re-opens: `crosslink issue reopen <id>`. The route did not hold; route again with the new information.

## [manual] — First-class fallback path

Same routing discipline, recorded in the review log directly. Use this when crosslink is not installed, or when the project deliberately uses manual mode end-to-end. Record the routed finding set as a section in the Phase 3 review log file (`{project}/vsdd-suite/review-log/YYYY-MM-DD-<domain>.md`). Use this shape per finding:

```markdown
### Finding N — Title (Dim X) — ROUTED

**Original classification:** [Resolved / Deferred / Accepted / etc.]
**Route:** 1a → 2a → 2b
**Owning artifacts:** DESIGN.md §Auth, tests/auth_test.rs, src/auth.rs
**Gate:** Spec revision passes self-adversary check; tests fail against current implementation; new tests pass after fix
**Sequencing:** Blocks Layer 5 (Auth UI) — Layer 5 cannot open until 1a route lands
**Status:** Routed → pending Phase 1a+1b session
```

Update the status as each phase lands. The routed finding is considered closed when all phases in its route have been completed and the gate at each phase has held. Cross-domain coordination (the suite's `**Coordination:**` line) is recorded inline in the routed finding's narrative, e.g., "Blocks UX Finding 7; UX cannot route until this Phase 1a+1b revision lands."

---

## Completion criteria

Phase 4 is complete and the next round can begin when:

1. Every real finding from Phase 3 has a recorded route
2. Every route names the gate at each phase
3. Every blocking relationship is recorded (in crosslink with `issue block`, or in the review log narrative)
4. Suite findings are filed in `suite-development/FINDINGS-INDEX.md`, not collapsed into project-phase routes
5. The proportion of findings routed to Phase 2b matches reality — if every finding routed to 2b, re-run the routing pass with the spec-defect bias check

---

## Anti-patterns

- **"Quick fix in implementation"** — routing a spec defect to Phase 2b because the implementation patch is faster than the spec revision. The spec stays wrong; the next reviewer flags it again. Route up.
- **"Defer routing to the next round"** — leaving findings unrouted because routing requires judgment. Unrouted findings carry forward as ambient process debt. The next IAR round encounters the same finding and the loop never closes.
- **"Close on fix-applied"** — closing a routed finding when the *code* changes, not when the *gate* holds. A Phase 1a+1b route is not done because someone edited DESIGN.md; it is done when the spec passes the self-adversary check.
- **"Hallucinated by re-classification"** — a finding marked Hallucinated in Phase 3 should not be re-routed; Hallucinated means the control held. If a finding is being re-examined for routing, the original classification was wrong — fix the Phase 3 log, then route.
- **"Silent CI-failure fix-and-force-push"** ([GitHub Actions supplement § Workflow failure discipline](../supplements/github-actions.md); [Review 87](../suite-development/review-log/2026-05-21-suite-review.md#review-87--2026-05-21-1230z) Finding 3) — A failed CI/CD check on a PR motivates an in-author fix; the operator amends the commit or force-pushes a new commit; the fix lands; the check goes green; the failure is never recorded as a finding. The audit trail loses the signal that a defect-class existed AND that the CI workflow caught it; future cycles can't regression-check the defect class because there's no evidence the class ever existed. The discipline: every CI/CD check failure on a PR is logged as a finding in the appropriate domain's per-session review-log (Platform Engineer for workflow-config defects; Quality Engineer for test-discipline defects; Software Engineer for build defects; Security for deny/audit defects; Documentation Reviewer for link-check defects; AI Engineer for CI cost/efficiency defects); classified per the domain's universe (typically Resolved in-session with the fix that lands in the PR); routed per the Phase 4 routing table above if the defect surfaces an upstream phase. The PR template's Completion checklist names the discipline; the `pr-checklist.yml` merge-gate workflow verifies the attestation is ticked. The framing: **CI/CD checks are mechanical adversarial reviewers** — they have the same regression-check value as the cold-session domain reviewers; their failures deserve the same audit-trail treatment.
- **"Site-specific fix declared closure"** ([Doc Reviewer R3 pattern](../../vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-20-documentation-reviewer.md) — Rounds 1, 2, and 3 all surfaced carryforwards of the same defect class because the fix-cycle visited the cited sites but didn't sweep the project for adjacent sites with the same defect-class) — claiming a finding Resolved after fixing only the cited site(s) without running a project-wide `grep -rn` to confirm the defect class is closed elsewhere. Forward-only: every Resolved finding for a defect class (stale path reference; retired terminology; broken anchor link; duplicate-name sweep artifact; letter-coded identifier residue) requires evidence in the audit-trail that a project-wide grep returns clean for the defect class — name the grep command + the returned-line count (zero or explicitly enumerated remaining sites if any) in the finding's Resolution paragraph. The discipline applies to all defect classes derived from a project-wide sweep operation (rename, reword, retire, restructure); it does not apply to defect classes inherent to a single site (e.g., a single function's parse-error path; a single spec section's wording). The Round-N+1 cold pass regression-checks the grep-clean state — if the cited grep was clean at fix-close but is not clean at Round-N+1 spawn, that's a regression finding for Phase 4's own discipline.

---

## After Phase 4

The next pass begins. If the route included Phase 1a+1b (spec revision), re-enter `primers/1ab-spec-crystallization.md` for that section. If it included Phase 1c (re-decomposition), re-enter `primers/1c-decomposition.md` for the affected layers. If it included Phase 2a/2b, re-enter `primers/2b-implementation.md` (preceded by `primers/2a-red-gate.md` if new tests need to land) for the routed work. If it included Phase 2c (refactor regression), re-enter `primers/2c-refactor.md` to back out or re-shape the offending change while tests stay green. The IAR refinement loop continues until MVR — see `README.md` § The refinement loop.


## Three-audience lens

This feedback-integration primer serves all three audiences of the [three-audience design principle](../suite-development/suite-development.md#three-audience-design-principle-review-80-finding-3) ([Review 80](../suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z) Finding 3; renamed in [Review 84](../suite-development/review-log/2026-05-21-suite-review.md#review-84--2026-05-21-1100z) Finding 4):

- **Suite developers** evolving this primer treat the prose as the methodology-authoring surface for Phase 4 feedback integration — changes here are methodology shifts requiring their own Review.
- **Suite users** running a session against this primer treat it as the canonical step-by-step for Phase 4 feedback integration on their own project; the completion criteria are what their next layer-gate or phase-close commit is checked against.
- **AI agents** loaded with this primer as cold-session context treat it as the spec for the session's authoring shape (file locations, classification vocabulary, the audit-trail entries this session produces); the primer's named artifacts + their schemas are the agent-API contract for what the session writes.

See [`../suite-development/suite-development.md`](../suite-development/suite-development.md) [§ Three-audience design principle](../suite-development/suite-development.md#three-audience-design-principle-review-80-finding-3) for the full discipline.
