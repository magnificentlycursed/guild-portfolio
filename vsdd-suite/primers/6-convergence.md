# Session Primer: Four-Dimensional Convergence (VSDD Phase 6)

Use this prompt after every layer of the project has reached Phase 5-MVR (or has explicitly declared Phase 5 not-applicable per the project's `**Phase 5 strategy:**` line in DESIGN.md). The output of this session is a **convergence record**: a project-level attestation that the four artifact dimensions — spec, tests, implementation, formal verification — have each independently reached MVR, with cross-dimension consistency verified.

Phase 6 is **not a build phase**. It is the project's terminal verification gate. Nothing is implemented in Phase 6; instead, the project-level record demonstrates that the four dimensions converged. A capstone-intent or production-intent project closes by passing Phase 6; a portfolio-intent project may close at the end of Phase 4 (per its `**Phase 6 strategy:** not applicable — <rationale>` declaration); a learning-exercise project typically closes at the end of Phase 4 by design.

**Phase 6 sits AFTER every layer's Phase 5 closes.** Phase 6 is project-scoped, not layer-scoped. A four-layer project that runs Phase 5 per layer (or declares it not applicable per layer) reaches Phase 6 entry only after all four layers' Phase 5 dispositions are recorded. Running Phase 6 against a project with layers still in Phase 3 is a discipline gap — the per-layer signals haven't stabilized.

---

## Prompt

You are helping verify four-dimensional convergence for a project under the Verified Spec-Driven Development (VSDD) methodology. This is Phase 6: Four-Dimensional Convergence. Your role is to evaluate, dimension by dimension, whether each artifact has reached its independent MVR, and to confirm that the four artifacts agree about the system's behavior.

**Your posture:** The four dimensions can converge by accident (every artifact records the same wrong thing) or by discipline (every artifact independently records the same right thing). Phase 6 distinguishes them. The cross-dimension consistency check asks: "if you read the spec, the tests, the implementation, and the formal-verification artifacts in isolation, do they agree about what the system does?" An accidentally-converging project passes this check because all four are wrong in the same way; a disciplined-converging project passes because each was developed against the same intent and reached MVR independently.

**Primary failure mode:** Treating Phase 6 as a single signal ("did we ship?") rather than four independent signals. A project that says "Phase 6 closed" because the test suite is green and the implementation builds has not exercised the convergence discipline — it has confirmed that two of four dimensions agree. Phase 6 evidence is per-dimension; the gate is the four-way conjunction.

**Sycophancy check:** An AI session running Phase 6 against a project the same agent built will pattern-match "looks finished" to "all four converged." Phase 6 evidence must be *artifact-driven*, not narrative-driven. For each dimension, name the specific artifact that establishes MVR for that dimension and cite the round / commit / harness that produced it. A Phase 6 record that says "the spec is at MVR because no spec gaps were raised in the last few rounds" is weak; one that says "the spec is at MVR because Phase 3 SO Review N produced only Hallucinated findings and Phase 4 routing surfaced no Phase 1a+1b destinations across all post-N rounds" is strong.

---

## Project reference

*(Paste the project's `DESIGN.md` § Project intent + `**Phase 5 strategy:**` + `**Phase 6 strategy:**` lines; the per-layer Phase 5 closure rounds from the per-domain logs (SA log for property-based testing/A.0/D rounds; QE log for mutation testing/C rounds — each with the `**Phase 5 surface:**` preamble tag per G-177 closure); the final Phase 3 round summaries per active domain; the project's CHANGELOG.md final-layer entry.)*

---

## The four dimensions

Phase 6 evaluates four independent MVR signals. Each dimension has a distinct exit criterion; the gate is the conjunction.

### Dimension 1: Spec MVR

**Question:** Has `DESIGN.md` reached the point where the cold adversary cannot surface a new spec gap that affects observable behavior?

**Signal:** Solution Owner (SO) cold-batch reviews across the final 2+ layers produced only Hallucinated findings, AND Phase 4 routing across the final 2+ layers produced no `route:phase-1a+1b` destinations.

**Anti-signal:** SO Review N (final layer) closed cleanly but Phase 4 routing in Rounds N-1 / N-2 surfaced spec gaps that were Resolved by silent DESIGN.md amendments — i.e., the spec changed but the change wasn't separately verified by a subsequent SO round. Silent amendment is a failure mode the convergence check surfaces.

**Disposition record:** the Phase 6 convergence record cites the final SO Review (by domain + round + date) and confirms the post-amendment SO re-pass was clean.

### Dimension 2: Test MVR

**Question:** Has the test suite reached the point where it would catch realistic defects?

**Signal:** Phase 5 mutation testing (mutation testing) produced a kill-rate report for each layer with every surviving mutant having a recorded disposition (per `primers/5-formal-hardening.md` § mutation testing). For projects that declared `**Phase 5 strategy:** not applicable — <rationale>`, the project's mutation-test signal is replaced by the QE final-round attestation that the test suite satisfies QE Dim 2 (test falsifiability) without the mutation-tool evidence — explicitly weaker, named in the convergence record as such.

**Anti-signal:** Phase 5 ran and produced a 90%+ kill rate with no per-mutant disposition (aggregate-only reporting). The aggregate hides which mutants survived; a 90% rate with the surviving 10% all in spec-asserted invariants is materially weaker than 70% with surviving mutants all in logging code.

**Verification step (per F5 — required, not aspirational):** before marking Dimension 2 Established, open each cited per-layer mutation testing round in `vsdd-suite/QUALITY-ENGINEER-REVIEW.md` + the linked `review-log/<date>-quality-engineer.md` session file and confirm a per-mutant disposition table exists (rows: mutant location, mutation kind, disposition, rationale). A per-layer entry that reports only a kill-rate number with no disposition table fails Dimension 2 regardless of the rate. The Phase 6 convergence round must cite the specific dispositions, not just the aggregate — otherwise the gate is rubber-stamping aggregate metrics rather than verifying the Phase 5 discipline held.

**Disposition record:** the convergence record links the per-layer QE mutation testing round in `vsdd-suite/review-log/<date>-quality-engineer.md` (entered with `**Phase 5 hardening:** mutation testing` preamble per G-177) + names the test-suite's kill rate + names the count of surviving mutants by disposition class (equivalent / missing-test-added / spec-gap-routed). If any cited layer's disposition table is absent or aggregate-only, Dimension 2 is Not Established and the layer is routed back to Phase 5 mutation testing before Phase 6 can close.

### Dimension 3: Implementation MVR

**Question:** Has the implementation reached the point where the cold adversary cannot surface a real (non-Hallucinated) defect against the spec?

**Signal:** Phase 3 final-round summaries per active domain (per the project's intent calibration) all read "no real findings; only Hallucinated findings" or "no findings" — and `primers/3-review-session.md` § Round triggers (G-131 continue + G-151 stop) was applied at each layer's terminal round. Implementation MVR is the suite's *original* MVR signal; this is the dimension the suite has owned end-to-end since v0.1.0.

**Anti-signal:** A round closed with "no findings" but the cold-session-isolation discipline was relaxed (one fresh chat reused for multiple domains; AI tool context bleed). The implementation-MVR signal is contaminated.

**Disposition record:** the convergence record cites the final Phase 3 round per active domain (by domain + layer + round + date) and confirms cold-session isolation was preserved across the active-domain set.

### Dimension 4: Formal-verification MVR

**Question:** For the project's declared formal-verification scope, has each formal-proof candidate either had its property proved OR been explicitly deferred with rationale?

**Signal:** Phase 5 formal proof harnesses (proofs / bounded model checks / TLA+ specs) each have a recorded outcome in the per-layer formal proof round under `vsdd-suite/SOLUTION-ARCHITECT-REVIEW.md` + the linked `review-log/<date>-solution-architect.md` session file (entered with `**Phase 5 hardening:** formal proof` preamble per G-177) per the project's `**Phase 5 strategy:** planned — <scope>` declaration. For projects that declared `**Phase 5 strategy:** not applicable — <rationale>`, Dimension 4's signal is the rationale itself — the project is closing convergence on three of four dimensions, with the formal-verification dimension explicitly declared not applicable.

**Anti-signal:** The project's `**Phase 5 strategy:**` listed formal-proof candidates but the harnesses never landed. Phase 6 cannot close with planned-but-not-executed Phase 5 scope.

**Disposition record:** the convergence record cites each formal proof harness (file + property + tool) OR records the explicit `**Phase 6 strategy:** not applicable — <rationale>` declaration verbatim.

---

## The convergence check

After each dimension has its own MVR record, the convergence check asks whether the four agree about what the system does. Inconsistency between dimensions is the load-bearing finding Phase 6 surfaces.

For each of the spec's named behaviors:

1. **Spec assertion:** What does `DESIGN.md` say the system does in this case?
2. **Test assertion:** Does the test suite include a falsifying test for this behavior? Cite the test by file + name.
3. **Implementation behavior:** Does the implementation exercise the behavior? Cite the source by file + function.
4. **Formal-verification statement:** If the behavior is in scope for formal verification, does a formal proof harness establish it?

A consistent project has every spec behavior backed by a test + implementation + (where applicable) formal proof, all in agreement. An inconsistent project has one of:

- **Spec asserts X; tests don't.** The spec moved during implementation; the test wasn't updated. Route via Phase 4 to Phase 2a (add the test).
- **Tests assert X; spec doesn't.** The test asserts a behavior the spec doesn't require — accidental over-specification, or spec drift. Route via Phase 4 to Phase 1a+1b (the spec should assert it) or to Phase 2a (the test is over-specific; relax it).
- **Implementation does X; tests + spec don't say.** Behavior exists in the code that no contract demands. Route via Phase 4 to Phase 1a+1b (the spec should assert it if intended) or Phase 2b (dead behavior; remove it).
- **Formal-verification statement contradicts the spec or tests.** The proof system disproved a property the spec asserted — the most serious convergence failure. Resolve via Phase 4 routing to Phase 1a+1b (the property statement was wrong) or Phase 2b (the implementation violates the spec).

The convergence check is *not* checking everything; it's checking the spec's *named* behaviors. A project whose spec under-specifies behavior compared to the implementation has a Phase 1a+1b gap surfaced by the convergence check itself.

---

## Driving questions

1. **What is the project's intent (per `DESIGN.md` § Project intent)?** Phase 6 applies at capstone and production intents per `domains/DOMAIN-INDEX.md` § Intent calibration; lower intents close at the end of Phase 4 by design.
2. **Per dimension, what is the artifact that establishes MVR?** Cite the artifact specifically — not "the test suite," but `tests/foo.rs:42-100` exercising the layer's named acceptance criteria.
3. **Are all four dimensions consistent across the spec's named behaviors?** Walk the inconsistency list above; route any inconsistencies via Phase 4 before declaring convergence.
4. **What did the project NOT verify?** A project that declared `**Phase 5 strategy:** not applicable` is closing convergence on a strict subset of dimensions. Name the dimensions that were not exercised; future-you reading the convergence record needs to know.
5. **What is the convergence record's audit signal?** A Phase 6 record that takes 5 minutes to produce ("looks done") is weaker than one that takes 30+ minutes to produce (per-dimension citation; per-behavior consistency check; explicit declarations of out-of-scope dimensions). The cost-of-production of the convergence record is itself the audit signal.

---

## Phase 6 convergence record format

The convergence record IS the final VDD-IAR Alignment review round (G-177 closure, 2026-05-20). No separate per-project Phase 6 file. The record is written once at project close as a new round in `vsdd-suite/VDD-IAR-ALIGNMENT-REVIEW.md` index + the linked `review-log/<close-date>-vdd-iar-alignment.md` session file (subsequent re-opens for follow-up work re-trigger a fresh Phase 6 convergence round; the original round is preserved as audit trail per the standard per-domain review log structure).

The round entry uses the standard per-review preamble per `suite-development/suite-development.md` § Per-review entry preamble plus a Phase 6 marker:

```markdown
## Review N — Phase 6 four-dimensional convergence (project-terminal) — YYYY-MM-DD HH:MMZ

**Scope:** project-terminal convergence — all layers L1..LN closed Phase 5 (or declared `Phase 5 strategy: not applicable`); all per-domain Phase 3 final rounds reached MVR; cross-dimension consistency check pending in this round.

**Session note:** [cold-session vs. in-session; sycophancy-compensation per the standard]

**Source:** domain-raised — VDD-IAR Alignment dim 14 (Phase 6 four-dimensional convergence)

**Phase 6 marker:** project-terminal convergence round.

**Project intent:** <intent-level per DESIGN.md>
**Phase 5 strategy:** <verbatim from DESIGN.md>
**Phase 6 strategy:** <verbatim from DESIGN.md>

### Dimension 1: Spec MVR

**Established by:** [Solution Owner Review N at <layer> on <date>, with Phase 4 routing across Rounds N+1..N-X producing no `route:phase-1a+1b` destinations.]

**Citations:** <cite the SO review log entries by link>

### Dimension 2: Test MVR

**Established by:** [Phase 5 mutation testing across layers L1..LN with kill rates <list>, surviving-mutant dispositions in the cited per-layer QE mutation testing rounds.]

(Or for not-applicable: "Phase 5 declared not applicable per DESIGN.md; test-suite MVR signal is QE final-round attestation against Dim 2 — explicitly weaker than mutation-tested signal.")

**Citations:** <cite per-layer QE mutation testing rounds in `review-log/<date>-quality-engineer.md` + QE final Phase 3 round>

### Dimension 3: Implementation MVR

**Established by:** [Phase 3 final round per active domain across layers L1..LN producing only Hallucinated findings.]

**Citations:** <cite per-domain final review log entries>

### Dimension 4: Formal-verification MVR

**Established by:** [Phase 5 formal proof harnesses <list> establishing properties <list>, cited from the per-layer SA formal proof rounds.]

(Or for not-applicable: "Phase 5 declared `formal proof not applicable — <rationale>`. Convergence closes on three of four dimensions; formal-verification dimension explicitly out of scope.")

**Citations:** <cite per-layer SA formal proof rounds in `review-log/<date>-solution-architect.md` + per-harness proof reports>

### Cross-dimension consistency check

For each of the spec's named behaviors:

| Behavior (DESIGN.md ref) | Spec assertion | Test (file:line) | Implementation (file:fn) | Formal verification (harness) | Consistent? |
|---|---|---|---|---|---|
| <behavior name> | <quote> | tests/foo.rs:42 | src/foo.rs:fn_name | proofs/foo_proof.rs | Yes |

(One row per spec-named behavior. Inconsistent rows are routed via Phase 4 before convergence is declared.)

### Out-of-scope dimensions

[Name the dimensions explicitly skipped, with the project intent + strategy declaration that authorized the skip.]

### Convergence attestation

[One paragraph: the convergence record's author asserts that the four (or three with formal-verification out-of-scope) dimensions have independently reached MVR and the cross-dimension consistency check holds. Signature line per the project's identity posture (see Anonymization-aware attestation below).]

### Summary

[One short paragraph: tally of dimensions Established, dimensions declared not applicable with rationale, cross-dimension inconsistencies routed via Phase 4 (zero at the closing round per the gate). Required per the per-review entry closing block standard.]

**Coordination:** [Optional — if the convergence round prompted any spec amendments, link the SO log; if the round prompted any retroactive Phase 5 work, link the relevant per-domain rounds.]
```

**Anonymization-aware attestation (F9 — applies when the project's identity posture is opt-in anonymized).** For projects that signal "scrub me" per `primers/3-review-session.md` § Confidentiality-aware citation (a `block local home directory paths` pre-commit hook; a `.gitconfig` with a noreply email; a scrubbed `Cargo.toml` author/repository fields), the attestation signature is **the closing-commit's git hash**, not the developer's real name. Format: `Signed: <commit-sha> on <YYYY-MM-DD HH:MMZ>.` where `<commit-sha>` is the commit that adds the final round to the VDD-IAR Alignment review log. The git hash is non-repudiable (a future reader can verify the commit is signed-off by the project's anonymized identity per the project's pre-commit hooks) and respects the anonymization discipline. For non-anonymized projects, the real name is acceptable; the audit signal is the same — a reader can verify the attestation against the project's commit history.

---

## Crosslink mode

Phase 6 round is committed alongside the rest of the project's `vsdd-suite/` artifacts. Crosslink integration is light — Phase 6 is project-terminal, so there's no ongoing session to instrument:

```sh
# Author the final VDD-IAR Alignment round (typically in a fresh chat with this primer loaded):
crosslink session start
crosslink session work <project-epic-id>     # the project's top-level epic
# (work the driving questions; produce the new round entry in
#  vsdd-suite/review-log/<close-date>-vdd-iar-alignment.md and update the
#  vsdd-suite/VDD-IAR-ALIGNMENT-REVIEW.md index)
git add vsdd-suite/VDD-IAR-ALIGNMENT-REVIEW.md vsdd-suite/review-log/*-vdd-iar-alignment.md
git commit -m "Phase 6: four-dimensional convergence round (final VDD-IAR Alignment round, signed)"
crosslink session end --notes "Project closed at four-dimensional convergence (or three-of-four with formal-verification declared not applicable)."

# Final milestone close — `crosslink milestone close` takes a numeric ID, not a name (G-167 / crosslink-contract.md § Known limitations).
# Recover the project-terminal milestone's numeric ID via `crosslink milestone list`:
crosslink milestone list                                  # lists numeric IDs alongside titles
crosslink milestone close <project-terminal-milestone-id> # substitute the numeric ID from the line above
```

If a Phase 6 inconsistency is discovered mid-write, route via Phase 4 like any other finding — file a crosslink issue with `phase:6` + `route:phase-<destination>` labels and resume the convergence round after the routed work closes.

---

## Manual mode

Same final VDD-IAR Alignment round in `vsdd-suite/review-log/<close-date>-vdd-iar-alignment.md` with the index row appended to `vsdd-suite/VDD-IAR-ALIGNMENT-REVIEW.md`. Inconsistencies routed via Phase 4 are recorded in the project's `FINDINGS-INDEX.md` with `phase:6` in the Source column.

---

## Completion criteria

Phase 6 is complete and the project is closed at four-dimensional convergence when:

1. A new round titled "Review N — Phase 6 four-dimensional convergence (project-terminal)" exists in `vsdd-suite/review-log/<close-date>-vdd-iar-alignment.md` with all four dimensions populated (or three populated and the fourth explicitly declared out of scope). The round is indexed in `vsdd-suite/VDD-IAR-ALIGNMENT-REVIEW.md`.
2. The cross-dimension consistency check table in the round body has zero inconsistent rows.
3. Every inconsistency surfaced during the check has been routed via Phase 4 to its earliest-correct phase and the routed work has landed (the resolution is part of the round's citations).
4. The convergence attestation is signed and dated in the round's closing block.
5. The project's CHANGELOG.md final entry references the Phase 6 convergence round by link.

Phase 6 is not iterative — there is no "Round N+1 of Phase 6." Subsequent project work that touches the spec, tests, implementation, or formal-verification artifacts re-opens the project and triggers a fresh Phase 6 round (preserving the original as audit trail per the standard per-domain review log structure). Forward-only: a project's first Phase 6 closure is anchored to 2026-05-20 (Review 64 / v0.7.0); closures predating 2026-05-20 retain their prior implementation-MVR-only closure shape (G-54 carve-out).

**Coordination with G-54 (Four-Dimensional Convergence partial ownership):** v0.7.0's Phase 6 primer closes G-54 by giving the suite ownership of all four dimensions' MVR signals and the cross-dimension consistency check. The check itself was the gap G-54 named — until v0.7.0, the suite only tracked implementation MVR; Phase 6 makes the other three dimensions equal first-class participants.
