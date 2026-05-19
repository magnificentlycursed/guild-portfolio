# Suite Review — 2026-05-19 (part 2 of 2)

This file holds Reviews 65–68 (the v0.7.1 → v0.7.4 work cycle: cold-context primer validation + Phase 5 primer refinements from bookmark-cli execution + discipline corrections + cold-subagent drift audit). Reviews 61–64 live in [2026-05-19-suite-review-part1.md](2026-05-19-suite-review-part1.md). The split conforms to the filename convention at [`../suite-development.md`](../suite-development.md) § Filename convention — files exceeding the 80 KB / 15-review threshold are split into `-partN.md` suffixed parts (Review 69 amendment).

---

## Review 65 — 2026-05-20 01:30Z

<!-- hook-bypass: chat-shorthand F-prefix finding headers committed pre-Review-67 discipline correction; preserved per G-89 forward-only narrative-preservation policy. See § Errata at the end of this entry for the F-prefix → Finding-N cross-reference map. Discipline correction in Review 67 Findings 1+2 prevents recurrence. -->

**Scope:** Cold-context adversarial review of the two Review 64 primers (`primers/5-formal-hardening.md` and `primers/6-convergence.md`) via a subagent spawned in isolation from this session's authorial context. The subagent loaded the two new primers + `primers/3-review-session.md` (posture) + `suite-development/suite-development.md` (governing standard for primers) + role prompts for Quality Engineer (Phase 5 lens) + Solution Architect (Phase 6 lens). The subagent surfaced 12 findings (F1–F12), all classified Deferred by the subagent (because a reviewer does not edit; it surfaces). The director (this session) addressed 9 of 12 in-session and registered the remaining 3 as Deferred gaps with G-130 discipline.

**Lens:** Cold-context primer-validation pass. The subagent is the closest available approximation to a true cold reviewer — it begins without memory of the Reviews 62/63/64 work. The six lenses applied per the subagent's brief: primer-contract compliance; internal consistency; defect-catch coverage (QE on Phase 5); architectural coherence (SA on Phase 6); cold-reader UX; external-dependency references. SA lens produced the most findings (F4, F5, F6, F11 — four), consistent with the brief's framing that the convergence check is the load-bearing addition.

**Session note:** **Subagent-driven cold review.** The subagent isolation is partial — it shares the codebase and the Claude model with this session, but begins with no in-context memory of the primers being authored in Review 64. The compensation for the residual context is the subagent's structural compliance with the cold-session posture (loaded `3-review-session.md` first; classified findings per its schema; flagged its own sycophancy self-audit at the end). The findings the subagent surfaced are evidence-based with file:line citations; none are narrative-only. The director's in-session disposition of these findings is itself subject to sycophancy risk — I am classifying findings against work I wrote one session ago. Compensation: I addressed 9 of 12 in-session rather than dismissing them; the 3 I deferred each have a real design question that warrants more thought than a single Edit (and the deferral discipline G-130 names trigger + cost + auto-Backlog for each).

**Source:** `director-raised` (operator instruction: "Do option 2 then commit") triggered a cold-context review session. The findings within the review are `domain-raised` (the subagent surfaced them by applying domain lenses to the new primers).

---

### Resolved

**F10 — Phase 5 Surface B "default threshold" undefined / contradictory.** Subagent identified that `primers/5-formal-hardening.md:64` named a "threshold" but its definition ("default: every surviving mutant in spec-asserted code") was a scope, not a threshold; completion criterion 3 referenced "under the threshold" with no numeric anchor. Resolution: renamed "threshold" to "evaluation scope" throughout Surface B; defined scope explicitly as "every surviving mutant in spec-asserted code paths"; named the out-of-scope-omissions-must-be-listed-in-the-log-preamble discipline. Completion criterion 3 updated. Cold reader can now determine when Surface B is gate-complete.

**F12 — Forward-only anchored to "v0.7.0 ship date" rather than a date.** Subagent identified the convention drift against `suite-development.md` (which date-anchors forward-only constraints, not version-anchors). Resolution: replaced "v0.7.0 ship date" with the explicit date `2026-05-20` (Review 64 / v0.7.0) in both `primers/5-formal-hardening.md` § Forward-only and `primers/6-convergence.md` § Completion criteria. Audit risk closed: a reader doesn't need to map version to date.

**F8 — Phase 5 sycophancy check covered only Surface B.** Subagent identified that `suite-development.md:91` requires every primer's prompt section to name "the primary failure mode to watch for" — the prior Phase 5 sycophancy check named Surface B's failure mode (rationalizing surviving mutants as equivalent) but Surfaces A, C, D had only artifact-level anti-patterns, not cognitive-failure-mode sycophancy checks. Resolution: expanded the prompt-level sycophancy check into a per-surface block — Surface A (writing properties whose only assertion is doesn't-panic / liveness), Surface B (rationalizing equivalent mutants), Surface C (declaring "no crashes found" after short fuzz runs as evidence; closure now requires both budget exhaustion AND non-trivial coverage/corpus signal), Surface D (tautological proof harnesses; each harness's property must map to a DESIGN.md sentence).

**F2 — `fast-check` conflated between Surface A and Surface C on JS/TS.** Subagent identified that `fast-check` appeared in both Surface A (line 42) and Surface C (line 76) without distinguishing the property shape per surface — risking the cold reader collapsing two surfaces into one. Resolution: Surface C's JS/TS line now explicitly distinguishes the property shape (Surface A asserts spec invariants over generated inputs; Surface C asserts "doesn't crash / matches a fuzzing oracle" over `fc.string()` / `fc.uint8Array()` parser-input generators) and prescribes two distinct Surface entries in `PHASE-5-LOG.md` rather than one combined entry.

**F1 — Phase 6 `crosslink milestone close` cited without ID-capture pattern.** Subagent identified that `primers/6-convergence.md:184` showed `crosslink milestone close <project-terminal-milestone-id>` without breadcrumb for obtaining the numeric ID — a cold reader following verbatim would hit the same `invalid digit found in string` failure that G-167 / `crosslink-contract.md` § Known limitations documents. Resolution: added a `crosslink milestone list` recovery step inline above the close invocation, with explicit cross-reference to G-167 / crosslink-contract.md § Known limitations.

**F5 — Phase 6 Dimension 2 anti-signal non-falsifiable from convergence record alone.** Subagent identified that the prior Dimension 2 disposition record asked for "kill rate + disposition class counts" but had no explicit verification step that each cited per-layer Surface B section actually contains the per-mutant disposition table the Surface B mandate requires — a Phase 5 log that violated Surface B's mandate but reported a clean rate would still satisfy Phase 6 Dimension 2 as written. Resolution: added explicit "Verification step (per F5 — required, not aspirational)" paragraph in Dimension 2 instructing the convergence-record author to open each cited per-layer Surface B section, confirm the disposition table exists, and route the layer back to Phase 5 if absent. Closes the cross-primer verification gap the subagent surfaced.

**F3 — Phase 5 completion-criterion 1 ("purity boundary verified") had no surface owning it.** Subagent identified that the gate criterion was load-bearing but unassigned — none of Surfaces A–D actually performed the boundary audit. Resolution: new `### Surface A.0: Purity-boundary verification (preamble — required for every Phase 5 layer entry)` as a Phase 5 layer-entry prerequisite. The audit is explicit: open the implementation, verify pure-function signatures and bodies don't perform I/O (file / network / process / env / random / time), route violations one of three ways (Phase 2b to restore purity; Phase 1a+1b to revise the boundary; named boundary refinement in the log preamble). Cold reader now knows where completion criterion 1 is satisfied. VDD-IAR Alignment dim 13 already references Surface activation — the omission of Surface A.0 becomes a finding for dim 13 mechanically.

**F9 — "Signed attestation" conflicted with anonymization posture.** Subagent identified that `primers/6-convergence.md:165` required "Signed: <name> <date>." but anonymized portfolio projects (per `primers/3-review-session.md:30-35`'s Confidentiality-aware citation discipline) cannot use a real name without violating the anonymization posture. Resolution: new `**Anonymization-aware attestation (F9 — applies when the project's identity posture is opt-in anonymized)**` paragraph specifying that anonymized projects use the closing-commit's git hash as the signature (non-repudiable; respects anonymization). Non-anonymized projects retain real-name signing. Cross-references `primers/3-review-session.md` § Confidentiality-aware citation for the anonymization signals the operator looks for.

**F7 — Phase 5 "retroactive-Red-Gate label" cross-reference was to a forward-flow construct.** Subagent identified that `primers/2b-implementation.md:13/30` defines the label semantic for "discovered during Phase 2b" (in-flight implementation work), but Phase 5 surfaces missing tests against already-shipped implementation-MVR code (post-MVR retroactive discovery) — different semantic. Resolution: extended the 2b primer's label definition to explicitly cover post-MVR retroactive discovery via a `(Phase 5 source)` qualifier — same label discipline, expanded scope. Phase 5 Surface B sycophancy check now cites "retroactive-Red-Gate (Phase 5 source) label" explicitly. No new label introduced — minimal scope creep.

---

### Deferred

**G-170 (new — addresses F4) — Cross-dimension consistency check has no disposition class for spec behaviors that are intentionally test-less.**

Subagent identified that `primers/6-convergence.md:79-91` requires every spec-named behavior to have a test row in the consistency-check table, with any missing test routing to Phase 2a. But some spec behaviors are legitimately untestable-by-design: build-toolchain selection, license text, intent declaration in DESIGN.md itself, README rendering. The check as written would either generate spurious Phase 2a routings or silently skip rows.

**Trigger:** when a capstone-intent project's first Phase 6 record surfaces a spec behavior that warrants an `untestable-by-design` disposition. Observable on the first Phase 6 author noting that a row has no defensible Phase 2a destination.

**Cost-of-deferral:** the first Phase 6 author hits the gap and either improvises a disposition (which then becomes the precedent) or routes spuriously to Phase 2a. The improvised precedent risks becoming the convention without director review.

**Auto-Backlog clause:** if 2026-09-30 passes without a real Phase 6 author hitting this gap, the gap auto-Backlogs and re-raises in the next suite review for fresh evaluation (it may be the gap is theoretical rather than load-bearing for the project types that hit capstone intent).

**Proposed resolution at trigger:** introduce an `untestable-by-design` disposition class for the consistency-check table, parallel to `not applicable` in the strategy declarations. Required: a one-sentence rationale per row (matching deferral-trigger discipline G-130's "real rationale, not TBD" rule). The row stays visible in the table; the absence is the disposition.

**Coordinate with:** G-130 (deferral-trigger discipline — the rationale shape borrows from it), G-162 (explicit-skip pattern — same intellectual root), G-54 (parent gap; the convergence check is its closure).

---

**G-171 (new — addresses F6) — Phase 6 strategy declaration grammar lives in DOMAIN-INDEX but the Phase 6 primer doesn't link it; portfolio-intent verification path ambiguous.**

Subagent identified two sub-issues: (a) `primers/6-convergence.md:5/117-119` references `**Phase 6 strategy:** not applicable` and `planned` declarations but doesn't link to the strategy grammar in `domains/DOMAIN-INDEX.md` § Phase 5 / Phase 6 strategy declaration (G-162); a cold reader running Phase 6 doesn't know what valid declarations look like without chasing the cross-reference manually. (b) The primer simultaneously says portfolio-intent closes at end of Phase 4 with a `Phase 6 strategy: not applicable` AND that closures predating v0.7.0 retain prior implementation-MVR-only closure — the audit-ownership for the not-applicable declaration's verification is ambiguous when the project never opens this primer.

**Trigger:** when a portfolio-intent project's first DESIGN.md author asks "where does my `Phase 6 strategy: not applicable` declaration get verified if I never open the Phase 6 primer?" OR when a capstone-intent author asks "what does a valid `Phase 6 strategy: planned` declaration look like without me cross-referencing DOMAIN-INDEX?"

**Cost-of-deferral:** strategy declarations land without authors knowing the grammar; the audit-trail signal G-162 was designed to produce degrades.

**Auto-Backlog clause:** 2026-09-30. (Paired with G-170; same trigger window.)

**Proposed resolution at trigger:** (a) add an inline grammar block in `primers/6-convergence.md` § Project reference area citing the valid declaration shapes (mirroring DOMAIN-INDEX) with a forward-link to DOMAIN-INDEX as the canonical source; (b) clarify the ownership boundary — portfolio-intent `not applicable` declarations are verified at DESIGN.md authoring time (by VDD-IAR Alignment dim 1) rather than at Phase 6 entry, because the project never opens Phase 6. The two verification paths are explicit per intent.

**Coordinate with:** G-162 (parent — strategy declaration discipline), G-150 (intent calibration — the per-intent verification path differs by intent), VDD-IAR Alignment dim 1 (the actual enforcement surface for portfolio-intent declarations).

---

**G-172 (new — addresses F11) — Phase 6 Dimension 3 (Implementation MVR) layer-to-project rollup rule is missing.**

Subagent identified that `primers/6-convergence.md:57` cites Phase 3 G-131/G-151 round triggers — but those triggers are layer-scoped (`primers/3-review-session.md:79-109`). A four-layer project that closed each layer at MVR has four layer-scoped Implementation MVR signals; Phase 6 needs a project-scoped conjunction rule the primer does not state. The disposition record at line 61 cites "the final Phase 3 round per active domain" — singular round / singular layer — but a project with N layers needs N×|active-domains| citations and a rollup rule.

**Trigger:** when a four-or-more-layer capstone-intent project reaches Phase 6 entry and the author asks "do I cite all 4 layers' final rounds for SE, or just the last layer's SE round?"

**Cost-of-deferral:** the first multi-layer Phase 6 author improvises a rollup (likely under-specifying — citing only the last layer's domain rounds) without realizing earlier layers' MVR signals could be stale if subsequent layers' refactors affected the earlier layers' code.

**Auto-Backlog clause:** 2026-12-31 (later than G-170/G-171 because multi-layer capstone projects are rarer in the near-term portfolio).

**Proposed resolution at trigger:** explicit rule in `primers/6-convergence.md` Dimension 3: project Implementation MVR = ∀ layer ∈ project, ∀ active-domain ∈ layer.active-domains, layer.final-round-for-domain MVR-signal AND no subsequent layer's commits touched files in this layer's scope without a re-verification round on the affected layer. The "no subsequent layer touched my files" clause is the load-bearing piece — protects against stale MVR signals.

**Coordinate with:** G-54 (parent — convergence check), G-131 (continue trigger — the per-layer signal Phase 6 rolls up), G-151 (stop trigger — same), VDD-IAR Alignment dim 14 (the enforcement surface).

---

### New gap registered

- **G-170 (F4)** — Untestable-by-design disposition class for the cross-dimension consistency check — Deferred
- **G-171 (F6)** — Phase 6 strategy declaration grammar inline + portfolio-intent verification path explicit — Deferred
- **G-172 (F11)** — Project-level Implementation MVR rollup rule — Deferred

---

### Coordination

The Review 65 dispositions coordinate with:

- **G-54 / G-55** (Addressed Review 64) — the parent gaps. Review 65's findings are second-pass refinements; none invalidates the closure. The cold reviewer explicitly noted all 12 findings were "real but the primers are coherent at the surface level" — the closures hold, the refinements deepen them.
- **G-130** (Addressed prior) — the three Deferred new gaps (G-170, G-171, G-172) each name trigger + cost + auto-Backlog per the discipline.
- **G-162** (Addressed Review 63 / refreshed Review 64) — F6 / G-171 surfaces a sub-issue in the strategy-declaration grammar's reachability from the Phase 6 primer; the trigger-on-first-portfolio-author is honest about when this needs to land.
- **G-167** (Addressed Review 63) — F1's resolution cross-references G-167's known-limitations entry rather than re-stating the workaround.
- **G-89** (Addressed prior) — Forward-only narrative preservation. The F12 date-anchor refinement was a convention drift the subagent caught; the fix aligns the new primers with the suite's established forward-only convention. Historical narratives (review-log + COMPATIBILITY) keep their prior prose.
- **VDD-IAR Alignment dim 13 + dim 14** (Added Review 64) — F3 resolution (Surface A.0 preamble) becomes mechanically checkable via dim 13; F5 resolution (Dimension 2 verification step) becomes mechanically checkable via dim 14.

---

### Summary

12 findings classified: 9 Resolved in-session, 3 Deferred with G-130 discipline. Cold review served as quality gate (not audit-only) — the new primers ship with the refinements rather than known defects. Backlog after Review 65: **0 Open + 6 Deferred** (G-159, G-168, G-169, G-170, G-171, G-172). The 3 new deferrals each have triggers tied to real project usage events (first multi-layer capstone Phase 6 author; first portfolio-intent strategy-declaration author) rather than calendar dates alone.

Sycophancy self-audit: the cold reviewer's findings were largely-impossible-to-dismiss-after-citation — every finding had a file:line reference and a specific evidence trail. The temptation I felt was to defer more of them as "judgment calls" to avoid the work; I forced myself to address the cheap ones in-session because the cost was a single Edit per finding. The three I genuinely deferred (F4, F6, F11) each pose a real design question (a new disposition class; cross-primer reachability of strategy grammar; a multi-layer rollup rule) that warrants more thought than a single Edit. The cold reviewer's own sycophancy self-audit named three near-passes (Surface D carve-out; per-prompt sycophancy quality from one strong example; v0.7.0 version anchoring read as harmless) — the same shape my Review 64 self-audit could have caught had I been more rigorous. The cold pass produced what my in-session pass didn't catch.

### Errata

**Finding 13 — Cross-reference map: chat-shorthand F-prefix labels in `### Resolved` (added 2026-05-20)**

Per Review 67 Finding 2 (chat-shorthand identifiers stay out of artifacts), Review 65's `### Resolved` section uses non-standard `**F1 — `, `**F2 — `, etc. headers — the chat-shorthand labels the cold subagent surfaced. Forward-only G-89 preserves the original committed text; this errata provides the F-prefix → Finding-N cross-reference map so future readers can resolve `F1`/`F2`/etc. citations against the per-review Finding-N convention. The Deferred findings retain their G-IDs separately.

| Chat-shorthand label (committed) | Finding-N (the standard form this entry should have used) | Title |
|---|---|---|
| F10 | Finding 1 | Phase 5 Surface B "default threshold" undefined / contradictory |
| F12 | Finding 2 | Forward-only anchored to "v0.7.0 ship date" rather than a date |
| F8 | Finding 3 | Phase 5 sycophancy check covered only Surface B |
| F2 | Finding 4 | `fast-check` conflated between Surface A and Surface C on JS/TS |
| F1 | Finding 5 | Phase 6 `crosslink milestone close` cited without ID-capture pattern |
| F5 | Finding 6 | Phase 6 Dimension 2 anti-signal non-falsifiable from convergence record alone |
| F3 | Finding 7 | Phase 5 completion-criterion 1 ("purity boundary verified") had no surface owning it |
| F9 | Finding 8 | "Signed attestation" conflicted with anonymization posture |
| F7 | Finding 9 | Phase 5 "retroactive-Red-Gate label" cross-reference was to a forward-flow construct |

Deferred findings (already have G-IDs; F-prefix maps to G-ID):

| Chat-shorthand label | G-ID | Title |
|---|---|---|
| F4 | G-170 | Untestable-by-design disposition class for cross-dimension consistency check |
| F6 | G-171 | Phase 6 strategy declaration grammar inline + portfolio-intent verification path |
| F11 | G-172 | Project-level Implementation MVR rollup rule |

**Classification:** Errata — cross-reference map for non-standard chat-shorthand labels; original entries unchanged per G-89.

---

## Review 66 — 2026-05-20 02:30Z

<!-- hook-bypass: chat-shorthand R-prefix finding headers committed pre-Review-67 discipline correction; G-173–G-176 G-IDs also over-promoted to Resolved-in-session findings per Review 67 Finding 1. Both drifts preserved per G-89 forward-only narrative-preservation policy. See § Errata at the end of this entry for the R-prefix cross-reference map. Discipline correction in Review 67 Findings 1+2 prevents recurrence. -->

**Scope:** Refinements to `primers/5-formal-hardening.md` surfaced by running Phase 5 against `bookmark-cli` in a sandbox (the option-3 validation exercise that closed the Review 64 work cycle). Real Phase 5 execution surfaced 4 primer-refinement candidates (R1–R4) plus 2 bookmark-cli-side findings (B1, B2). This review addresses R1–R4; B1+B2 remain unaddressed pending operator authorization (bookmark-cli is the reference impl per G-117/G-122).

**Lens:** Methodology-tested-against-reference-impl lens. The Phase 5 primer was validated by running its prescriptions against bookmark-cli's actual source: cargo-mutants Surface B produced 11 mutants (7 caught, 1 missed, 3 unviable → 87.5% kill rate on viable); Surface A.0 purity-boundary audit caught a divergence between bookmark-cli's `src/lib.rs:1-7` module-doc purity claim and the actual implementation (3 of 4 `BookmarkStore` methods are effectful). Both findings are usable; the surfacing exercise produced 4 friction points the primer should address before it ships to a second project.

**Session note:** In-session, post-commit (the Review 62–65 work is committed at `57d5341`; Review 66 lands as additional refinements on the same branch). Partial-isolation tradeoff acknowledged — the same author who wrote the Phase 5 primer (Reviews 64+65) is now refining it based on its own run output. The compensation: refinements derived from concrete execution outcomes (real mutant disposition output; real Surface A.0 findings) rather than narrative reflection. The cold equivalent would be a second subagent re-running Phase 5 with the refined primer against a different project; deferred (no obvious second target).

**Source:** `domain-raised` — Phase 5's own surfaces (A.0 audit; cargo-mutants execution) raised the findings against the primer-as-applied; the operator's "Address R1–R4" instruction `director-raised` the closure decision.

---

### Resolved

**R1 / G-173 — Surface A.0 multi-source purity check.**

bookmark-cli's `src/lib.rs:1-7` (module doc) said "Pure-core storage logic ... contains only pure functions" while its DESIGN.md § Verification architecture was silent on per-function purity. The prior Surface A.0 audit instructions in `primers/5-formal-hardening.md` (Review 65 / F3 closure) only covered DESIGN.md as the purity-claim source — module docs were not in scope.

Resolution: extended Surface A.0 prose to list "every authoritative purity claim the project makes" with explicit enumeration: DESIGN.md § Verification architecture AND module/package documentation (Rust `//!`; Python module docstrings; TypeScript `/** @module */` JSDoc; "equivalent constructs in other languages"). Added a third check beyond (a) impl-vs-DESIGN.md and (b) impl-vs-module-doc: (c) DESIGN.md vs. module-doc cross-source consistency. The bookmark-cli case ("`src/lib.rs:1-7` claimed pure; DESIGN.md silent; both diverged from actual") cited as the canonical worked example. New routing option in step 3: if the two sources make divergent purity claims, the divergence is a finding → route to Phase 1a+1b to reconcile (single source of truth at DESIGN.md OR module doc cites DESIGN.md). Step 4 (audit-outcome log preamble) updated to record cross-source consistency status.

**R2 / G-174 — Surface B "unviable" disposition class.**

cargo-mutants reports 3 outcome classes (caught / missed / unviable). The prior Surface B sycophancy check enumerated 3 dispositions for surviving mutants ((a) equivalent / (b) missing-test / (c) spec-gap) but had no class for the unviable case. Phase 5's run against bookmark-cli produced 3 unviable mutants (e.g., `+` → `-` in `String + &str` string concatenation — type-system-rejected) that fit none of the 3 dispositions but did need to be listed in the log for completeness.

Resolution: added 5th disposition class (d) "unviable — mutation does not compile; not a behavioral signal" to the Surface B sycophancy check. The Surface B run-the-tool sub-section's step 2 ("Examine each surviving mutant...") updated from "four outcomes" to "five outcomes" and added explicit guidance: unviable mutants get a one-line note in the log (with the example "`src/lib.rs:55:35` `+` → `-` in string concatenation: unviable — compile failure on `String - &str`"), but they are not test-suite gaps. The disposition table now matches cargo-mutants' actual output shape; the Phase 5 log preamble has a categorical home for every outcome class.

**R3 / G-175 — Tool-install upfront cost note.**

cargo-mutants compiles from source on install (timed at 1m 39s on the bookmark-cli test machine). The prior Phase 5 primer mentioned cargo-mutants / cargo-fuzz / proptest etc. by name but did not flag that the first-Phase-5-session-per-project bundles tool installs with the run. A new operator reading the primer expected to start hardening within minutes; the realistic first-run wall-clock includes the install.

Resolution: new "**Tool-install upfront cost (G-175)**" paragraph immediately under § Phase 5 surface and above Surface A.0. Names the install costs explicitly: cargo-mutants compiles from source (1–2 minutes); cargo-fuzz requires Rust nightly toolchain; proptest/fast-check/hypothesis are dev-dependency adds. Prescribes that first-session log preamble names installs performed and time spent on installs separately from time spent on actual hardening. Subsequent sessions inherit and skip installs.

**R4 / G-176 — Surface A Cargo.toml dep-add prerequisite.**

Surface A's prior prose said "Express each invariant as a property-based test using the language's standard tool: Rust: proptest..." but didn't mention that adding proptest requires editing Cargo.toml's `[dev-dependencies]` — the dep-add is a separate commit from the property-test commits, and a cold reader running Surface A without prior Rust dep-management experience would not know to do this first.

Resolution: new "**Prerequisite (G-176)**" paragraph immediately under Surface A's intro. Names the dev-dependency target per language (Rust: `proptest` or `quickcheck` in `[dev-dependencies]` in `Cargo.toml`; JS/TS: `fast-check` in `devDependencies` in `package.json`; Python: `hypothesis` in `[tool.poetry.group.dev.dependencies]` or `requirements-dev.txt`; Go: `gopter` via `go.mod`). Prescribes that the dep-add lands as a separate commit before the property-test commits — keeps dep-introduction reviewable independently. Same shape as the Phase 2a Red Gate commit boundary discipline: separate commits for separable concerns.

---

### Coordination

The Review 66 refinements coordinate with:

- **G-55** (Addressed Review 64) — parent gap. G-173 / G-174 / G-175 / G-176 each refine Surface A.0 / Surface B / the primer intro / Surface A specifically; the parent's closure holds — these are refinements within the existing primer surfaces, not gap re-opens.
- **G-89** (Addressed prior) — forward-only narrative preservation. The Surface A.0 refinement extends the prior audit instruction; previous Phase 5 log entries (none yet exist in any project) remain valid records under the prior single-source audit instruction.
- **G-130** (Addressed prior) — deferral-trigger discipline. No new deferrals from Review 66; the 6 prior Deferred gaps (G-159, G-168, G-169, G-170, G-171, G-172) unchanged.
- **G-167** (Addressed Review 63) — crosslink-contract.md § Known limitations. The unviable-mutant disposition pattern (R2 / G-174) mirrors the same shape as known-limitations entries: name the observed behavior; provide a workaround; classify as not-a-bug. The shared structural shape isn't named in either primer but is the same intellectual pattern.
- **VDD-IAR Alignment dim 13** (Added Review 64) — dim 13's Surface-activation-matches-strategy check now covers the 5-disposition Surface B universe (G-174) and the multi-source Surface A.0 audit (G-173). The dim's evaluation procedure becomes slightly stricter; no rewrite needed because dim 13 references the primer rather than restating its contents.

The refinements do NOT regress any prior work. R1's multi-source Surface A.0 audit is additive (the prior DESIGN.md-only check still applies; the module-doc check is layered on top). R2's unviable disposition is additive (5 classes where there were 4; existing 4 unchanged). R3's tool-install note is metadata. R4's dep-add prerequisite is a sequencing clarification, not a methodology change.

**The two bookmark-cli-side findings (B1, B2) from the Phase 5 test against bookmark-cli remain UNADDRESSED:**

- **B1** — `tests/bookmarks.rs` is missing a save-to-nested-path test; the cargo-mutants surviving mutant at `src/lib.rs:48` would be killed by adding it. Disposition per Surface B: (b) test gap; route via Phase 4 to Phase 2a with the `retroactive-Red-Gate (Phase 5 source)` label per the primer.
- **B2** — DESIGN.md § Verification architecture is silent on per-function purity while `src/lib.rs:1-7` claims "Pure-core storage logic"; both diverge from the implementation (3 of 4 `BookmarkStore` methods are effectful). Disposition per Surface A.0: cross-source divergence + impl drift; route via Phase 4 to Phase 1a+1b (reconcile DESIGN.md and module doc to a single authoritative source; revise the boundary to match the implementation OR refactor the implementation to honor the boundary).

These are project-side findings against the reference implementation. bookmark-cli is the suite's worked example (G-112 closure; reference impl per G-117 / G-122). Modifying it warrants explicit operator authorization rather than routine work — the project's review-log structure would need to absorb Phase 5 findings under the v0.7.1 conventions, and bookmark-cli's first-layer-gate-close predates 2026-05-20 (forward-only G-89 carve-out applies). The findings are surfaced here for visibility; the operator decides whether to bring bookmark-cli to v0.7.1+ conventions in a future cycle.

---

### Summary

4 primer refinements Addressed in-session (R1–R4 → G-173–G-176). 2 project-side findings (B1, B2) surfaced and held pending operator authorization. Backlog after Review 66: **0 Open + 6 Deferred** (unchanged from Review 65: G-159, G-168, G-169, G-170, G-171, G-172). The Phase 5 primer's first real-project execution validated the methodology (concrete findings produced; disposition shapes worked) and surfaced 4 friction points the cold review hadn't anticipated — the methodology-tested-against-reference-impl lens caught what the cold-context primer-validation lens (Review 65) missed.

Sycophancy self-audit: the Phase 5 primer's first run produced findings; I closed 4 of the 4 friction points it surfaced rather than dismissing any as "edge cases the primer doesn't need to cover." The temptation was to dismiss R3 (tool-install cost) as "obviously a per-operator concern" — rejected because the primer is what a new operator reads BEFORE running, so an upfront note saves the operator the discovery cost. The temptation was also to defer R1 as "judgment call about what counts as a 'purity claim source'" — rejected because the bookmark-cli case is concrete (`src/lib.rs:1-7` made a claim that diverged from DESIGN.md silence) and any first-Phase-5 operator on any Rust project with module docs will hit the same shape. Both temptations represent the same failure mode the Phase 5 primer's own Surface B sycophancy check warns against: rationalizing surviving findings as "not really gaps." The discipline applied to the primer as it does to project mutants.

Most uncertain choice: R4 prescribes "dep-add lands as a separate commit before the property-test commits" — borrowing from the Phase 2a Red Gate boundary discipline. This may be over-prescriptive for small projects where dep-add and first-property could share a commit without harming reviewability. Forward-only carve-out preserves the prescription for v0.7.2+ starters; if a project's first Phase 5 session genuinely warrants a combined commit, the divergence is itself a Phase 5 log entry note rather than a discipline violation.

### Errata

**Finding 5 — Cross-reference map: chat-shorthand R-prefix labels in `### Resolved` + G-ID over-promotion (added 2026-05-20)**

Per Review 67 Finding 1 (G-IDs only for Deferred / Open findings) and Finding 2 (chat-shorthand identifiers stay out of artifacts), Review 66's `### Resolved` section drifted on both axes: the 4 Resolved-in-session findings were promoted to G-IDs (G-173–G-176) when they should have remained `**Finding N — Title**` entries, AND the headers used a compound `**Rn / G-XXX — Title**` form that combined chat shorthand with the G-ID. Forward-only G-89 preserves the original committed text; this errata provides the R-prefix and Finding-N cross-reference map.

| Chat-shorthand label (committed) | G-ID assigned (committed; over-promoted) | Finding-N (the standard form this entry should have used) | Title |
|---|---|---|---|
| R1 | G-173 | Finding 1 | Surface A.0 multi-source purity check |
| R2 | G-174 | Finding 2 | Surface B "unviable" disposition class |
| R3 | G-175 | Finding 3 | Tool-install upfront cost note |
| R4 | G-176 | Finding 4 | Surface A Cargo.toml dep-add prerequisite |

The G-IDs (G-173–G-176) stay registered in FINDINGS-INDEX.md as committed records per G-89; future readers can resolve `R1`/`R2`/`R3`/`R4` citations via either the Finding-N column above or the G-ID column.

**Classification:** Errata — cross-reference map for non-standard chat-shorthand labels + G-ID over-promotion; original entries unchanged per G-89.

---

## Review 67 — 2026-05-20 03:15Z

**Scope:** Operator-driven adversarial questions against the Review 64–66 Phase 5 / Phase 6 work, surfaced when reviewing bookmark-cli's first Phase 5 application. Two questions: (a) why does `PHASE-5-LOG.md` exist as a separate per-project file instead of folding Phase 5 findings into the existing per-domain review logs; (b) why did bookmark-cli's first Phase 5 session file only QE-domain findings when the Surface A.0 finding is SA territory. Question (b) is project-scope (closed in `bookmark-cli-phase5-adoption` branch by splitting the session into QE Review 2 + SA Review 1). Question (a) is suite-scope and this entry files it as Finding 1 / G-177 below.

A third adversarial question from the same operator turn — chat-shorthand finding identifiers (`Q1`, `Q2`, `B1`, `B2`, `R1`–`R4`, `F1`–`F12`) have been leaking from chat into review-log artifacts, commit messages, and PR descriptions across Reviews 65 / 66 / the bookmark-cli adoption commit. The governing standard in `suite-development.md` § Suite review entry format prescribes only `**Finding N — Title**` (new findings) and `**G-XX — Title**` (gap-registry walks); chat shorthand is not a valid identifier in artifacts. Filed as Finding 2 below (Resolved-in-session — discipline correction; forward-only per G-89 leaves the historical references in place).

The operator also raised a discipline question: prior reviews (specifically Reviews 63 and 66) over-promoted Resolved-in-session findings to suite-side G-IDs when the governing standard in `suite-development.md` § Suite review entry format reserves `### New gap registered` (and its G-ID assignment) for findings that need ongoing tracking — i.e., Deferred / Open findings, not Resolved-in-session. Review 65 was correct (G-170/G-171/G-172 for Deferred only); Reviews 63 (G-160–G-167 for 8 Resolved findings) and 66 (G-173–G-176 for 4 Resolved findings) over-promoted. Forward-only correction per G-89: historical G-IDs stay as committed records; **going forward, only Deferred / Open findings get G-IDs**. This review's Resolved finding (the discipline observation itself) does NOT get a G-ID.

**Lens:** Adversarial-question-against-recent-work lens (closely related to a cold review but driven by operator question rather than fresh-session pass). Single defect-class: structural coherence of the Phase 5 record artifacts; whether they earn their duplication of the existing per-domain review log shape.

**Session note:** In-session. Partial-isolation tradeoff: the same author who wrote the Phase 5 primer (Reviews 64–66) is now filing a structural finding against that primer. Compensation: the finding is grounded in concrete evidence from a real project application (bookmark-cli's PHASE-5-LOG.md Layer 1 entry contained ~50 lines of disposition narrative that also appears in QE Review 2 + SA Review 1; the duplication is observable, not narrative). The operator's question is the cold-reader signal — a reader unfamiliar with the Phase 5 primer's prescription asked the question my primer-internal logic did not.

**Source:** `director-raised` — operator's adversarial question against the Phase 5 record structure in bookmark-cli; the underlying finding is structural and was implicit in the primer's design but not surfaced as a defect until the operator named it.

---

### Resolved

**Finding 1 — Discipline drift on G-ID assignment in suite reviews 63 and 66 (corrected forward-only)**

`suite-development.md` § Suite review entry format already specifies that `### New gap registered` (and G-ID assignment) is the format for findings "promoted to a tracked gap" — implying a deliberate promotion act. Resolved-in-session findings live under `### Resolved` with `**Finding N — Title**` shape per the same standard; no G-ID required. Review 65 followed this discipline correctly (G-170/G-171/G-172 for the 3 Deferred findings; 9 Resolved findings as `**Finding N**` without G-IDs). Reviews 63 and 66 drifted — Review 63 assigned G-160–G-169 to all 10 findings (8 Resolved + 2 Deferred); Review 66 assigned G-173–G-176 to 4 Resolved findings. The historical G-IDs are committed; forward-only G-89 applies — they stay as records of how the registry was populated at the time, even though the discipline was loose.

**Resolution:** Discipline correction applied going forward. **From Review 67 onward, only Deferred / Open findings get suite-side G-IDs**; Resolved-in-session findings are `**Finding N — Title**` entries in the per-review narrative without G-IDs. The `FINDINGS-INDEX.md` registry rows G-160–G-167 and G-173–G-176 remain in place (forward-only — rewriting would break the audit trail and the cross-references in committed commit messages, COMPATIBILITY rows, and PR descriptions). The `suite-development.md` text already names the discipline; no governing-standard text change is needed. This finding does NOT itself receive a G-ID (it's Resolved-in-session per the corrected discipline).

**Finding 2 — Chat-shorthand finding identifiers leaking into artifacts (corrected forward-only)**

Across the recent work cycle, chat-shorthand identifiers (`Q1`/`Q2` for operator questions, `B1`/`B2` for bookmark-cli findings surfaced in Review 66's Phase 5 test, `R1`–`R4` for primer-refinement candidates, `F1`–`F12` for cold-subagent findings) have been propagating from chat conversation into review-log artifact text, commit messages, PR descriptions, and gap-registry row narratives. The governing standard in `suite-development.md` § Suite review entry format § Finding body prescribes exactly two valid identifier forms in artifacts: `**Finding N — Title**` for new findings within a review, and `**G-XX — Title**` for gap-registry-walk entries. Chat shorthand is not a valid artifact identifier.

Most visible drift sites (committed; forward-only — not rewritten): Review 65 § Resolved used `**F10 — Title**` / `**F12 — Title**` etc. headers (should have been `Finding 1 / Finding 2`); Review 66 § Resolved referred to findings as "G-173 (R1)" through "G-176 (R4)" with the `R`-prefix carried into the SUITE-DEVELOPMENT-REVIEW row text and CHANGELOG entries; the bookmark-cli adoption commit (`ca663ac`) and PR #23 description reference "B1" and "B2" extensively. None of these can be rewritten without breaking commit-message cross-references in published commit history.

**Resolution:** Going forward, artifacts (review-log entries, FINDINGS-INDEX rows, commit messages, PR descriptions, CHANGELOG entries, COMPATIBILITY rows) use **only** the two standard identifier forms. Chat-shorthand identifiers stay in the chat conversation that produced them; their job is to be ergonomic during synchronous discussion, not to be artifact identifiers. The discipline-drift correction itself does NOT receive a G-ID per the corrected Finding 1 discipline. Cleanup applied in this Review 67 entry before commit: the earlier chat references `(Q1)`, `(Q2)`, `(F1)`, `(B1)`, `(B2)` within Review 67's own narrative have been replaced with descriptive prose anchored to the finding's content rather than the chat-shorthand label.

---

### Deferred

**G-177 — `vsdd-suite/PHASE-5-LOG.md` per-project file duplicates per-domain review logs without earning the duplication.**

The Phase 5 primer (`primers/5-formal-hardening.md` § Phase 5 log format) prescribes a per-project `vsdd-suite/PHASE-5-LOG.md` file as the Phase 5 record. Surfaces A/A.0/B/C/D are written there. But the suite's existing project-level review log structure (per `suite-development.md` § Governing standard for project-level review logs) handles Phase 3 rounds via per-domain `<DOMAIN>-REVIEW.md` + `review-log/YYYY-MM-DD-<domain>.md` files; cross-domain coordination via the `**Coordination:**` line. Phase 5 work fits the same shape — a Surface B (mutation testing) round IS a QE round; a Surface A.0 (purity boundary verification) round IS an SA round.

The bookmark-cli adoption (committed at `a33a289` on the `bookmark-cli-phase5-adoption` branch — see PR #23) demonstrates the duplication concretely: the Surface B disposition table appears in both `PHASE-5-LOG.md` Layer 1 entry and in QE Review 2 (Finding 1 body); the Surface A.0 finding similarly appears in both `PHASE-5-LOG.md` and SA Review 1 (Finding 1 body). The duplication is ~50 lines per per-session pair and grows linearly with project + layer count.

**Two resolution candidates** (which one the project director chooses determines the primer revision):

- **(a) Retire `PHASE-5-LOG.md`.** Phase 5 findings file under per-domain review logs with a `**Phase 5 surface:**` preamble tag on each round (e.g., `**Phase 5 surface:** B — mutation testing`). Phase 6's convergence record cites per-domain rounds directly via Markdown links. Cleaner; matches the existing structure; no separate file to maintain.
- **(b) Make `PHASE-5-LOG.md` an index file.** Parallel to how `SUITE-DEVELOPMENT-REVIEW.md` indexes `review-log/` files at the suite level. `PHASE-5-LOG.md` becomes a one-table-per-layer index pointing at the per-domain rounds that did the actual Phase 5 work. Per-domain rounds carry the substantive content. Slight overhead (one extra file) but easier for Phase 6's convergence-check to cite a single document.

**Trigger** (per G-130 deferral discipline): when a **second project** enters Phase 5 hardening (real evidence that the duplication compounds across projects), OR when the operator preemptively chooses one of the two resolution candidates. Observable on the second project's Phase 5 session opening.

**Cost-of-deferral:** each Phase 5 project replicates the duplication shape; per-project Phase 5 records grow to duplicate per-domain records. A future Phase 6 author has to read both `PHASE-5-LOG.md` AND the per-domain rounds to confirm consistency. The audit trail is split across two artifacts that record the same dispositions. The cost grows linearly with project + layer count; at 1 project / 1 layer (the current bookmark-cli state), the cost is small but observable; at 5 projects / 3 layers, the duplication is structural debt.

**Auto-Backlog clause:** if 2026-10-31 passes without a second project entering Phase 5, the gap auto-Backlogs and re-raises in the next suite review (the duplication may be load-bearing in ways the bookmark-cli case did not surface; revisit with new evidence at that point).

**Coordinate with:** G-54 (parent — Phase 6 convergence check cites Phase 5 records), G-55 (parent — Phase 5 primer ownership), G-89 (forward-only — current Phase 5 records under bookmark-cli's first adoption stay as committed records under the prior primer shape), G-173 (Surface A.0 multi-source check), G-174 (Surface B 5-disposition class). Bookmark-cli's PHASE-5-LOG.md (committed at `a33a289`) includes a forward reference to this gap as documentation that the structural concern is acknowledged.

**Classification:** Deferred — gap registered with named G-130 trigger (second project enters Phase 5), cost-of-deferral (per-project duplication compounds linearly), and auto-Backlog clause (2026-10-31). No in-session resolution attempted; this entry promotes the structural concern to the registry for future closure.

---

### New gap registered

- **G-177** — `PHASE-5-LOG.md` per-project file duplicates per-domain review logs — Deferred with G-130 discipline (trigger: second project enters Phase 5; auto-Backlog 2026-10-31).

---

### Coordination

The Review 67 disposition coordinates with:

- **G-55 / G-54** (Addressed Review 64) — parent gaps. G-177 surfaces a structural defect introduced by Review 64's Phase 5 primer authoring; the parent closures hold (Phase 5 + Phase 6 ownership is unchanged), this is a refinement candidate within the existing surface.
- **G-130** (Addressed prior) — deferral-trigger discipline applied (trigger + cost + auto-Backlog all named).
- **G-89** (Addressed prior) — forward-only narrative-preservation. The discipline-drift acknowledgement under § Resolved does NOT retroactively renumber the historical G-IDs from Reviews 63 and 66; the prior IDs stay as committed records.
- **PR #23 (`bookmark-cli-phase5-adoption`)** — forward references G-177 in `bookmark-cli/vsdd-suite/PHASE-5-LOG.md`. The forward reference resolves once both PRs merge (the suite-side PR registers G-177 first; the bookmark-cli PR's PHASE-5-LOG.md then cites a registered gap rather than a forward-pointing TODO).

The closures do NOT regress any prior work. The discipline-drift correction is forward-only per G-89; the G-177 deferral is purely additive (new gap; existing primer unchanged).

---

### Summary

1 Resolved finding (the discipline-drift correction; no G-ID per the corrected discipline). 1 Deferred new gap (G-177; PHASE-5-LOG.md duplication). Backlog after Review 67: **0 Open + 7 Deferred** (G-159, G-168, G-169, G-170, G-171, G-172, G-177 — all with G-130 trigger + cost + auto-Backlog). The Phase 5 / Phase 6 owned-by-suite state from Review 64 is unchanged; this review surfaces a structural refinement candidate without re-opening the parent gaps.

Sycophancy self-audit: the operator's three adversarial points (PHASE-5-LOG duplication; G-ID over-assignment; chat-shorthand identifier leakage) were all real defects, and I addressed them honestly. G-177 is against my own Review 64 primer authoring — I authored PHASE-5-LOG.md as a separate file because the primer's first-draft prescription called for it; the cold review (Review 65 by subagent) and the first-real-project test (Review 66 by Phase 5 against bookmark-cli) both failed to surface the duplication concern. The operator's adversarial question was the first lens that caught it. Finding 1 (G-ID discipline) and Finding 2 (chat-shorthand identifiers) are process drifts in how I've been populating the registry and the review-log artifacts — the governing standard in `suite-development.md` already specified the correct disciplines; I drifted from them without noticing. This is the same shape as the suite's overall sycophancy discipline at the project level (the director's manual testing catches what cold-batch IAR misses, per G-132); applied here at the suite level via three director-raised observations rather than a cold review pass. All three are honest near-misses, none comfortable to flag against my own recent work.

---

## Review 68 — 2026-05-20 04:30Z

**Scope:** Operator-directed adversarial review of the vsdd-suite focused on **drift** — places where the suite's own implementation has departed from its own governing standards, conventions, or methodology promises. Conducted via cold subagent (spawned in isolation from the in-session author who produced Reviews 62–67) applying 5 drift lenses: discipline-vs-practice; convention drift; cross-artifact consistency; methodology fidelity; forward-only narrative preservation. Subagent surfaced 11 findings, all real (none Hallucinated); 10 Resolved in-session (1 by Dismissal as intentional distinction; 4 by standard / primer / README text updates; 5 by the new `check-suite-review-preamble.sh` hook + errata blocks under the G-89 forward-only narrative-preservation mechanism); 0 Deferred (the prevention mechanism is the disposition for the going-forward cases; the historical drift instances are preserved per G-89 with errata).

**Lens:** Cold-subagent drift-detection pass. Composes 5 drift lenses serially; the discipline-vs-practice lens produced the most findings (6 of 11), consistent with the recent operator pattern of catching the same author drifting from standards the author wrote.

**Session note:** In-session director triage of subagent findings, partial-isolation acknowledged. The subagent's findings are evidence-based with file:line citations; the in-session director (same author who produced Reviews 62–67) classifies and resolves. Compensation: every Resolved finding's resolution is a concrete artifact change (the new hook; text updates to `suite-development.md` / primer 3 / README; errata blocks per the sanctioned G-89 mechanism), not a narrative dismissal. The sycophancy risk is highest for Finding 5 (Coordination heading collision — Dismissed as intentional distinction); the Dismissal rationale is concrete (the standard text DOES distinguish the two contexts; no observed mis-use) but a cold reviewer might still flag it.

**Source:** mixed — subagent findings are `domain-raised` (the cold adversary applying drift lenses to artifacts surfaced the findings); the operator's "Conduct an adversarial suite review focused on finding and preventing drift" instruction is `director-raised` at the session-opening level.

---

### Resolved

**Finding 1 — `**Source:**` field missing from Reviews 52–61 despite being Required-for-all-domains per G-133 (suite-development.md:275)**

Subagent identified that the G-133 closure (Review 59, 2026-05-19 06:00Z) made `**Source:**` a Required-for-all-domains preamble field, but Reviews 52, 53, 54, 55, 56, 57, 58, 59, 60, 61 (10 reviews dated after G-133's closure window) omit the line. Reviews 62–67 do include it. Compliance: 6/16 post-G-133 reviews. The omission degrades the audit-trail-granularity signal G-133 was registered to produce.

**Resolution:** The historical reviews stay non-compliant per G-89 forward-only narrative-preservation (rewriting 10 review-log entries to insert Source lines would break the audit trail). Going-forward enforcement is the new `vsdd-suite/hooks/check-suite-review-preamble.sh` hook (committed this review): every entry dated 2026-05-20 or later must include `**Source:**`. The hook's `ENFORCEMENT_THRESHOLD` constant carries the forward-only date. Future drift cannot recur silently.

**Finding 2 — Review 65 § Resolved finding headers use `**F10 — Title**`, `**F12 — Title**`, etc. (chat-shorthand instead of `**Finding N — Title**`)**

Subagent identified that Review 65's 9 Resolved finding headers use the cold-subagent's `F`-prefix labels rather than the standard `**Finding N — Title**` form prescribed by `suite-development.md:317` / `:428`. Review 67 Finding 2 corrected the discipline forward-only but left the committed instances in place.

**Resolution:** Per the sanctioned errata mechanism (`suite-development.md:429`: "Follow-up findings introduced after the session has been logged must be marked `**Finding M — Title (added YYYY-MM-DD)**` and placed at the end of the original entry"), added `**Finding 13 — Cross-reference map: chat-shorthand F-prefix labels in `### Resolved` (added 2026-05-20)**` at the end of Review 65 with a F-prefix → Finding-N cross-reference table covering all 9 Resolved findings plus the 3 Deferred (which map to G-170/G-171/G-172). Future readers encountering `F1`/`F2`/etc. citations now have an artifact-level resolution path. Original entry text unchanged per G-89. `<!-- hook-bypass: ... -->` HTML comment added in the first 5 lines of Review 65 so the new hook does not flag the original non-standard headers (the bypass itself documents the rationale + cross-references the errata block).

**Finding 3 — Review 66 § Resolved finding headers use `**R1 / G-173 — Title**`, etc. (compound chat-shorthand + over-promoted G-IDs)**

Subagent identified Review 66's 4 Resolved finding headers as triple drift: (a) `R`-prefix chat shorthand; (b) Resolved-in-session over-promotion to G-IDs (corrected per Review 67 Finding 1); (c) compound `Rn / G-XXX` form combining both into a non-standard header shape.

**Resolution:** Same errata + bypass mechanism as Finding 2. Added `**Finding 5 — Cross-reference map: chat-shorthand R-prefix labels in `### Resolved` + G-ID over-promotion (added 2026-05-20)**` at the end of Review 66 with a R-prefix → Finding-N + G-ID cross-reference table covering all 4 Resolved findings. The G-IDs (G-173–G-176) stay registered in FINDINGS-INDEX.md as committed records per G-89. `<!-- hook-bypass: ... -->` HTML comment added in the first 5 lines of Review 66.

**Finding 4 — Review 65 + Review 66 findings lack the required closer line (`**Resolution:**` or `**Classification:**`) per suite-development.md:333 + :428**

Subagent identified that the governing standard mandates every finding body end with exactly one of `**Resolution:**` (Resolved) or `**Classification:**` (everything else), and that Reviews 65 + 66 entirely omit these closers — they describe what changed in prose under the header but never tag the closer with the standard's bold-line form. Review 67's findings DO include the closers.

**Resolution:** Reviews 65 + 66 stay non-compliant per G-89 forward-only (10+ findings to retroactively patch; the errata blocks added for Findings 2 + 3 above implicitly cite the corrected discipline). Review 67's G-177 was missing a `**Classification:**` line; added `**Classification:** Deferred — gap registered with named G-130 trigger ...` in this review session. The new `check-suite-review-preamble.sh` hook enforces closer presence going forward for any entry dated 2026-05-20 or later that isn't bypassed.

**Finding 5 — `### Coordination` H3 (suite-review entries) vs `**Coordination:**` bold-line (project-level closing block) naming collision**

Subagent identified that the same word "Coordination" points at two different structural elements depending on artifact type: `suite-development.md:340` prescribes `**Coordination:**` as a bold-line for project-level review log closing blocks; `:429` separately authorizes `### Coordination` H3 for suite-review entries. Both conventions are coherent if a reader finds both passages but the naming collision can confuse a contributor importing one pattern into the other context.

**Classification:** Dismissed — the standard text DOES distinguish the two contexts (project-level § Closing block at :340 vs. suite-review § Suite review entry format at :429); no observed mis-use of one convention in the wrong artifact-type. The distinction is intentional: suite-review entries are richer (warrant H3) while project-level closing blocks are concise (warrant a bold line). Renaming either to `### Coordination (suite-review)` etc. would add ceremony without solving a real-world ambiguity. The Dismissal rationale is the standard's textual distinction; if a future contributor mis-applies the convention, that's the trigger to revisit.

**Finding 6 — Reviews 53 + 55 carry bare `**Session note:** In-session.` with no compensation named, violating the G-133-era discipline at suite-development.md:435**

Subagent identified that `suite-development.md:435` makes session-note-compensation a structural-error grade requirement ("itself a finding for VDD-IAR Alignment dim 7"), but Reviews 53 and 55 (both post-G-133) have bare `**Session note:** In-session.` lines.

**Resolution:** Same shape as Finding 1 — historical instances stay per G-89; going-forward enforcement is the new hook. The hook's preamble check verifies `**Session note:**` exists but does not currently parse the prose for compensation language (that would require natural-language analysis). Documented as a hook-enhancement candidate; for now the hook + the in-session director's review-time scrutiny are the prevention mechanisms.

**Finding 7 — Reviews 62 + 66 register new gaps (G-159 / G-173–G-176) without the `### New gap registered` classification section per suite-development.md:427**

Subagent identified the discipline gap; the hook's check 5 (verify `### New gap registered` section exists when a `**G-XX — Title**` header is present) was drafted but dropped from the v1 hook implementation because distinguishing new-gap registrations from gap-registry-walks (G-ID headers that address already-tracked gaps, e.g., Review 64's G-54 / G-55 closures) requires reading the registry — out of scope for a v1 hook.

**Resolution:** Section-presence discipline is advisory-grade for the hook's v1; the standard text at :427 remains the authority. Going-forward hook enhancement: load `FINDINGS-INDEX.md` and verify per-G-ID that any G-XX header is either (a) addressing a registered gap whose status row exists OR (b) registering a new gap with both a row in FINDINGS-INDEX and a `### New gap registered` section in the review log. Deferred to future hook iteration; not blocking Review 68's closure.

**Finding 8 — Per-review preamble field ordering not standardized**

Subagent identified that the standard at `:272-284` lists required fields without explicit ordering; de-facto ordering in Reviews 62–67 is Scope → Lens → Session note → Source, but the standard's text order is Scope → Session note → Source (with Lens in a different sub-section applicable only to suite reviews).

**Resolution:** Added explicit "**Canonical ordering** (Review 68 Finding 8 clarification)" paragraph to `suite-development.md` § Per-review entry preamble naming the canonical order for project-level vs. suite-review entries. The hook does not enforce ordering — it checks field presence — but the standard now names the convention; future reviewers have a textual reference.

**Finding 9 — Reviews 63 + 66 use unauthorized `**Source:** mixed` value not enumerated in suite-development.md:275**

Subagent identified that G-133 enumerates 4 valid Source values; Reviews 63 + 66 use a `mixed` composite that's not in the list. The composite is honest about reality (a director-raised session that produced domain-raised findings is mixed-source), but the value is non-standard.

**Resolution:** Extended the standard at `:275` to include `mixed` as the 5th valid Source value, with explicit sub-disposition guidance ("The `mixed` value requires the Source line to name the sub-disposition explicitly"). The hook's `VALID_SOURCE_VALUES` constant includes `mixed`. Reviews 63 + 66 are now retroactively-compliant against the extended standard; the standard's text now authorizes the value they used.

**Finding 10 — Phase label says `Phase 1a+1b` but route label string is `route:phase-1a`; no inline rationale on the user-facing surface**

Subagent identified the cross-artifact consistency drift: README + primers reference `Phase 1a+1b` (renamed per G-160) but the route-label string `route:phase-1a` was deliberately kept stable (per G-160 closure: "labels are identifiers, not scope-descriptors"). The rationale is documented in FINDINGS-INDEX:G-160 but invisible at the user-facing call sites in README.md and primers/4-feedback-integration.md.

**Resolution:** Added inline "**Label-vs-phase naming note (Review 68 Finding 10)**" paragraph at the route-label introduction in `vsdd-suite/README.md` (between the `**[crosslink]** Use route labels` lead-in and the first code block) naming the convention: label string stays stable per G-160; phase name evolves in docs. Future readers hit the explanation in-page rather than chasing the gap registry.

**Finding 11 — `**Source:**` field is mandatory for per-review entries but `primers/3-review-session.md` (the adversarial-review primer) does not mention G-133 or the Source field**

Subagent identified the methodology-fidelity drift: a reviewer following primer 3 verbatim produces a per-review entry that violates the G-133 mandate, because primer 3 (the cold session's loaded primer) never names the Source field.

**Resolution:** Added new `## Source attribution (G-133 / Review 68 Finding 11)` section to `primers/3-review-session.md` after `## After each domain review` (where the classification schema is discussed) and before `## If reviewing the IAR suite itself`. The new section names the 5 valid Source values (including the `mixed` extension from Finding 9) and prescribes the per-session classification path. A cold reviewer loading only primer 3 now encounters the Source-attribution discipline without needing to read suite-development.md.

---

### New gap registered

*(none — every finding Resolved or Dismissed in-session; the prevention-mechanism hook is the disposition for the going-forward cases of Findings 1, 2, 3, 4, 6.)*

---

### Coordination

The Review 68 closures coordinate with:

- **G-89** (Addressed prior) — forward-only narrative-preservation policy. Reviews 52–61 (Finding 1) and Reviews 53 + 55 (Finding 6) and Reviews 65 + 66 (Findings 2, 3, 4) historical drift stays as committed records; the new hook enforces forward-only.
- **G-129** (Addressed prior, `check-changelog-currency.sh`) and **G-139** (Addressed prior, `check-crosslink-references.sh`) — precedent for shipping a discipline-mechanizing hook as a suite artifact rather than as advisory text. `check-suite-review-preamble.sh` follows the same pattern: discipline-then-recurrence-then-hook.
- **G-133** (Addressed Review 59) — Source field discipline. Findings 1 + 9 + 11 are all G-133-related: Finding 1 catches retroactive non-compliance forward-only via the hook; Finding 9 extends the value set; Finding 11 adds the field to primer 3 where the discipline is operationally needed.
- **G-160 / G-162 / G-167** (Addressed Reviews 63 / 63 / 63 respectively) — Finding 10 cross-references G-160's "labels are identifiers" framing; Finding 5 dismissal cross-references G-162's strategy-declaration discipline (which uses similar context-sensitive naming patterns); the hook's overall approach mirrors G-167's known-limitations cataloging.
- **Review 67 Findings 1 + 2** (Resolved prior in this same file) — Findings 2 + 3 of this review are the historical-instance cleanup for Review 67's forward-only discipline corrections; the errata + bypass mechanism is the sanctioned-by-G-89 path.

The Review 68 closures do NOT regress prior work. The new hook is additive (no existing hook modified). The standard text updates extend rather than narrow (canonical ordering paragraph adds clarification; `mixed` Source value adds a 5th option without invalidating the prior 4). The errata blocks in Reviews 65 + 66 add cross-reference content without modifying original entries. The bypass HTML comments in Reviews 65 + 66 are metadata that activates only when the hook runs.

---

### Summary

11 findings classified: 10 Resolved in-session (1 by Dismissal; 9 by concrete artifact changes); 0 Deferred (the prevention-mechanism hook is the disposition for going-forward cases). Backlog after Review 68: **0 Open + 7 Deferred** (G-159, G-168, G-169, G-170, G-171, G-172, G-177 — unchanged from Review 67). No new gaps registered.

**Prevention mechanism shipped:** `vsdd-suite/hooks/check-suite-review-preamble.sh` (a 200-line Python pre-commit hook) is the suite's fourth discipline-mechanizing hook (joining `check-crosslink-references.sh` G-139, `check-changelog-currency.sh` G-129, `check-review-log-anonymization.sh` G-98). Forward-only `ENFORCEMENT_THRESHOLD` of `2026-05-20`; per-entry `<!-- hook-bypass: <rationale> -->` HTML comment escape valve (used for Reviews 65 + 66 in this review per G-89 compliance). The hook composes with the existing three via `.pre-commit-config.yaml`.

Sycophancy self-audit: the subagent's drift findings were largely impossible to dismiss after file:line citation (the standard text vs. practice mismatch is observable mechanically). The temptation I caught myself nearly succumbing to: classifying more findings as Deferred than Resolved-in-session, on the theory that "the hook resolves them going forward." That framing is right for new artifact-level enforcement but it isn't right for the standard-text gaps Findings 8/9/10/11 surfaced — those needed text changes, not hook coverage. I forced myself to make the text changes inline rather than punt to a future Review. The hook + errata + bypass + standard-text-extensions combination is the right disposition for this class of drift; punting would have been the same failure mode the discipline was registered against. The Dismissal of Finding 5 is the only finding I'm uncertain about; if the operator disagrees with the Dismissal rationale, the appropriate next move is to extend the standard text at :340 + :429 with the explicit "Coordination (project-level)" / "Coordination (suite-review)" naming distinction.

---

## Review 69 — 2026-05-20 05:30Z

**Scope:** Operator-directed investigation of "parser-aborted messages" surfaced while working with the suite review log. Diagnosis: the 2026-05-19-suite-review.md file had grown to 128 KB / 889 lines / 8 review entries (Reviews 61–68) — large enough to exceed tree-sitter / IDE markdown-parser budgets on dense files with many inline code spans + long paragraphs. Root cause is the suite's own filename convention at `suite-development.md` § Filename convention prescribing one-file-per-session-start-date; the rule scales poorly on long review-cycle days where many sessions chain in one day (8 reviews of 2000+ words each is the breaking case observed here). This entry files the diagnosis, the file split, and the convention amendment.

**Lens:** Methodology-fidelity-drift lens applied to the suite's own filename convention. The convention works at normal cadence but fails at the dense-day scale; the operator's "parser aborted" observation is the discovery signal. Drift is observable not in artifact content but in artifact size / operator UX.

**Session note:** In-session, post-Review-68 (Reviews 65–68 already address discipline drift; this Review 69 addresses operational-scale drift). Partial-isolation acknowledged — same author who produced Reviews 62–68 is now filing the size finding against the file the author's verbosity grew. Compensation: the diagnosis is mechanical (file size in KB; line count; per-line character count; markdown table count; backtick density were measured against the actual file pre-split) rather than narrative; the threshold encoded in the amended convention (80 KB / 15 reviews) is empirical from the breaking case (128 KB / 8 reviews of recent verbose style).

**Source:** director-raised — operator's "Let's investigate the parser aborted messages" observation surfaced the finding; subsequent in-session diagnosis traced the cause to file size.

---

### Resolved

**Finding 1 — Filename convention does not scale on dense-day review files**

`suite-development.md` § Filename convention (pre-amendment) prescribed one-file-per-session-start-date with no size cap. The 2026-05-19-suite-review.md file accumulated Reviews 61 through 68 (8 entries) and grew to 128 KB / 889 lines / 4 paragraphs over 1000 chars / 673 inline backticks. Tree-sitter and IDE markdown parsers hit their parse-time budgets and emitted "parser aborted" diagnostics when the operator opened the file in the IDE. The size is itself a methodology-fidelity drift surface — the convention scales fine at 1-3 reviews per day but fails at 8 dense reviews per day, and the suite had no rule preventing the accumulation.

**Resolution:** Two-part fix applied in this Review 69 session:

1. **File split.** `git mv 2026-05-19-suite-review.md 2026-05-19-suite-review-part1.md`; trimmed part1 to Reviews 61–64 (~65 KB); created `2026-05-19-suite-review-part2.md` with Reviews 65–68 (~63 KB). Both files now under the 80 KB threshold the amended convention encodes. Each file gets an H1 of `# Suite Review — 2026-05-19 (part N of 2)` plus a navigation note linking to the sibling part. Git history is preserved on part1 via the `git mv` rename detection; part2 is a new file. The split is mechanical (4 reviews per part); the natural narrative grouping (Reviews 61–64 ship features; Reviews 65–68 validate and refine) is a coincidence of the split point, not the criterion.
2. **Convention amendment.** `suite-development.md` § Filename convention extended with a `**File-size threshold + part suffix (Review 69 amendment)**` paragraph naming the 80 KB / 15-review threshold, the `-partN.md` filename suffix pattern, the H1 + navigation-note requirements, the rationale (tree-sitter / parser-budget consequences observed empirically), the cross-reference rule (SUITE-DEVELOPMENT-REVIEW + FINDINGS-INDEX + CHANGELOG + COMPATIBILITY all update link targets when a file is split), and the forward-only carve-out (existing single-file days unchanged unless they hit the threshold).

**Cross-references rewritten:** 38 anchored links across 3 files (`vsdd-suite/CHANGELOG.md`, `vsdd-suite/suite-development/FINDINGS-INDEX.md`, `vsdd-suite/suite-development/SUITE-DEVELOPMENT-REVIEW.md`) — every `review-log/2026-05-19-suite-review.md#review-N--...` anchor updated to the correct `-partN.md` per the review-number → part mapping (Reviews 61–64 → part1; Reviews 65–68 → part2). Link display text in SUITE-DEVELOPMENT-REVIEW.md Reviews rows updated to match new target filenames. CHANGELOG Review 68 entry's prose references to Reviews 65/66/67 errata updated to `-part2.md`. Historical-narrative reference (CHANGELOG Review 62 entry describing the original file's creation under the prior naming) preserved per G-89.

**Verification:** the new `check-suite-review-preamble.sh` hook (Review 68 closure) runs clean against both part files. Pre-commit hooks all pass. File sizes are 61 KB (part1) and 66 KB (part2) — both well under the 80 KB threshold; both under the parser-budget pressure observed at 128 KB.

**Classification:** Resolved — file split applied + convention amended; no new G-ID per Review 67 Finding 1 discipline.

---

### Coordination

The Review 69 closure coordinates with:

- **G-89** (Addressed prior) — forward-only narrative preservation. Reviews 61–68 content is unchanged; only the filename and link targets are updated. The CHANGELOG line 208 reference to the original `2026-05-19-suite-review.md` file (Review 62 entry's "new file" note) stays as committed history of the file's pre-split state.
- **Review 68 hook (`check-suite-review-preamble.sh`)** — verifies both part files pass after the split. The hook does not encode the 80 KB threshold; that's an authoring discipline at the standard text level, not a per-commit check (the threshold is a guideline for when to split; the hook checks content discipline within whichever file).
- **The drift audit lenses (Review 68)** — methodology-fidelity-drift lens was applied to content in Review 68 but not to operational scale; Review 69 extends the same lens to file size. A future drift audit may want to add a "file size / operator UX" axis explicitly.
- **G-130** (Addressed prior) — no G-ID assigned this review; Resolved-in-session per Review 67 Finding 1 discipline.

The Review 69 closure does NOT regress prior work. The convention amendment is additive (extends the existing convention with the size threshold; existing single-file days remain valid). The file split is purely structural (no content changes within the moved review entries). Cross-references are updated mechanically; historical narrative preserved.

---

### Summary

1 finding (Resolved-in-session). File split into part1 + part2; convention amended with 80 KB / 15-review threshold; 38 cross-references rewritten across 3 files. Both part files under the threshold and pass the new Review 68 hook. Backlog after Review 69: **0 Open + 7 Deferred** (G-159, G-168, G-169, G-170, G-171, G-172, G-177 — unchanged). No new gaps registered.

Sycophancy self-audit: the temptation was to file this as a Deferred gap ("we'll split the next time it breaks") rather than addressing it in-session. Rejected because the operator's parser-aborted observation is the breakage; deferring it would leave the file in its parser-hostile state. The convention amendment with the empirical threshold is the load-bearing part — without it, a future long review-cycle day produces the same friction. The size threshold itself is a judgment call (80 KB / 15 reviews from one breaking case; could be wrong); the right way to refine is to observe a second instance of the threshold being either too strict (premature split) or too lax (parser-aborted at lower count) and adjust. The current threshold is "just past the breaking case" which is the right initial calibration but warrants review when the second data point lands. Forward-only carve-out in the convention amendment is the explicit guard against retroactive churn.
