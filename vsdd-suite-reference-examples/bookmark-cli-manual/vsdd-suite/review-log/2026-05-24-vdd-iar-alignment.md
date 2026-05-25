# VDD-IAR Alignment Review — 2026-05-24

---

## Review 1 — 2026-05-25 01:15Z

**Scope:** Layer 3 cycle process-compliance audit covering the five-commit Layer 3 sequence — `79a9a83` (spec activation, AI-co-authored first-draft) + `654cbbf` (operator-confirmation pass) + `878d3b6` (Phase 2a Red Gate, two-commit canonical shape) + `fd21900` (Phase 2b implementation, GREEN) + `78bd3cf` (Phase 2c extract-and-name annotation). Verifies the 14 [VDD-IAR Alignment](../../../../vsdd-suite/domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md) dimensions against the Layer 3 cycle's process artifacts (DESIGN.md § Layer 3 spec contracts + § Project intent Phase 5/6 strategy lines; TODO.md § Layer 3 decomposition + Phase 2c annotation + Layer-gate criteria; tests/bookmarks.rs Layer 3 block; src/lib.rs + src/main.rs Phase 2b additions; CHANGELOG.md slim-form entries). Does NOT evaluate the Layer 3 Phase 3 IAR domain-review outputs themselves (those rounds have not been run yet — this is the first review session of the Layer 3 cycle, opened against the post-`78bd3cf` state ahead of Phase 3 cluster spawn).

**Session note:** Cold context. This reviewer did not author the five Layer 3 commits, did not author the Layer 2 carry-forward closures at `9a984ec` / PR #47 Phase 5 follow-up, and has no investment in the Layer 3 cycle's success. The Layer 3 spec is the project's first AI-co-authored spec ("AI authors first-draft; operator edits + owns"); this reviewer notes that authoring shape but holds the same discipline floor — the spec exists as a checked-in contract, and the methodology discipline is whether the cycle that landed it walked the canonical phases. Sycophancy guard applied: the temptation in a "first round of a new layer" review is to confirm "the precedent layers worked, so this layer probably works too." Each dim below cites the specific artifact line that confirms the discipline holds, not the precedent-by-association.

**Source:** `domain-raised` — the cold adversary applying the 14 VDD-IAR Alignment dimensions to the post-`78bd3cf` artifact state surfaced the findings below. The session-opening dispatch (Phase 3 IAR Round 1 against bookmark-cli-manual Layer 3) is `director-raised` at the spawn-orchestration level.

**Reference:** [VSDD whitepaper](https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00) (governing methodology); [VDD-IAR Alignment domain prompt](../../../../vsdd-suite/domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md) (14 dimensions); [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md).

**Program phase:** Phase 1 (reference-implementation tier — crosslink + Phase 2+ tooling are not introduced; dim 11 issue-tracking compliance is not applicable per the domain prompt § Program Phase Context).

**Regression-check against:** [VDD-IAR Alignment Review 5](2026-05-21-vdd-iar-alignment.md#review-5--2026-05-22-1630z) — the Layer 2 closing round. Specifically the Layer 2 R4 F1 / R5 F1 Red Gate evidence-preservation closure (which prescribed "the canonical shape is two commits — one for the Phase 2a Red Gate, a second for the Phase 2b implementation" as the discipline for Layer 3) and Layer 2 R4 F5 / R5 F5 Phase 6 NA declaration precedent. The Layer 3 cycle MUST inherit both annotations as forward-applied discipline; any regression is a finding.

**Round:** 1 (Layer 3 cycle; first VDD-IAR Alignment round for Layer 3).
**Active domain set:** 13 (12 role + 1 meta) per DESIGN.md § Project intent.

**MVR signal:** **NOT REACHED at Round 1.** This round surfaces 1 substantive Open finding ([Finding 4](#r1-f4) — promised `manual-tests/layer-3.md` artifact not yet authored), 4 discipline-honest Resolved closures, and 2 Dismissed-with-rationale dispositions for cross-phase items that cannot be evaluated at Round 1 timing. Per [G-131](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-131) continue-trigger discipline, the Layer 3 Phase 3 IAR cycle continues into the per-domain cluster reviews; this VDD-IAR Alignment round is one of N parallel domain reviews opened against the post-Phase-2c state.

---

### Resolved

**Finding 1 — Phase-progression discipline holds; the five-commit Layer 3 sequence walks Phases 1a/1b/1c → 2a → 2b → 2c in canonical order with the two-commit Phase 2a/2b shape prescribed by Layer 2 R4 F1 closure (Dim 1 + Dim 3 + Dim 4 + Dim 7)**

<a id="r1-f1"></a>

**Owner:** vdd-iar-alignment
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

Phase-progression discipline check (Dim 1 + Dim 3):

| Commit | Phase | Evidence |
|---|---|---|
| `79a9a83` | 1a + 1b + 1c (spec activation, AI-co-authored first-draft) | DESIGN.md § Behavioral contracts § bm export (Layer 3) + § bm import (Layer 3); DESIGN.md § Edge case catalog § Layer 3 additions; DESIGN.md § Interface definitions § Command surface (Layer 3 additions); TODO.md § Layer 3 with 15 ACs + 15-entry Red Gate test plan + layer-gate criteria |
| `654cbbf` | 1a + 1b operator-confirmation pass (no new phase) | Spec refinement only; 6 confirmed at AI-author-default + 2 operator-revised; all `**AI-author note for operator:**` callouts removed |
| `878d3b6` | 2a Red Gate (15 failing tests; tests-only commit) | tests/bookmarks.rs +627 lines; 15 new tests; commit message preserves canonical Red Gate failure mode evidence (`error: unrecognized subcommand 'export'` / `'import'` exit 64) |
| `fd21900` | 2b implementation (GREEN) | src/lib.rs + src/main.rs additions; 45/45 + 3/3 tests pass; 0 clippy warnings |
| `78bd3cf` | 2c extract-and-name annotation (no code changes) | TODO.md:144 annotation; CHANGELOG.md entry |

Phase order is canonical. Layer 2 closed at PR #47 (Phase-5-trigger follow-up) BEFORE Layer 3 Phase 1a/1b opened at `79a9a83` — no phase overlap. ✓

Phase 2a/2b two-commit canonical shape check (Dim 4 — Red Gate commit precedence):

The Layer 3 cycle adopts the **two-commit canonical shape** that Layer 2 R4 F1 / R5 F1 prescribed for Layer 3 via the TODO.md:85 Red Gate evidence-preservation annotation. `878d3b6` lands the 15 failing tests alone with the Red Gate failure-mode evidence preserved in the commit message (the `error: unrecognized subcommand` exit-64 failure cited per-test). `fd21900` lands the implementation that makes the same 15 tests pass. The git-history is reviewable end-to-end:

- `git checkout 878d3b6 && cargo test --test bookmarks tests_export --no-fail-fast` reproduces the RED state (6 tests fail with the canonical exit-64 failure mode).
- `git checkout fd21900 && cargo test --test bookmarks` reproduces the GREEN state (45/45 pass).

This is the **stronger form** of the Red Gate discipline than the Layer 2 single-commit-with-annotation form — Layer 3's git-history-as-audit-trail is structurally stronger than Layer 2's prose-annotation-as-audit-trail. The Layer 2 R5 F1 closure's forward-looking discipline ("Layer 3 + future projects: the canonical shape is two commits") IS the precedent that Layer 3 inherits cleanly. ✓

Cross-session spec consistency check (Dim 7): the Phase 2b implementation matches the operator-confirmed spec from `654cbbf`. Spot-checks:

- Dedup applies BOTH against existing destination state AND within imported payload (per `654cbbf` operator-revision): src/lib.rs `import_json` — the `contains` check joins destination state for each push, so subsequent records dedup against earlier-imported records within the same payload. ✓
- 10 MB default cap + `--max-stdin-bytes <N>` override: src/lib.rs `MAX_STDIN_BYTES_DEFAULT = 10 * 1024 * 1024`; src/main.rs `Cmd::Import { max_stdin_bytes: usize }`. ✓
- Empty-stdin treated as user-error exit 1: src/main.rs `run_import` validates stdin BEFORE loading store, so empty-stdin doesn't create file. ✓
- Strict-object-wrapped JSON only (bare arrays rejected): src/lib.rs `import_json` deserializes against a `StorageFormat` struct, not a top-level array — bare-array would fail at the schema-mismatch path. ✓
- `display_safe` placement at serialization step (deferred to Phase 2b verification per `654cbbf`): tests/bookmarks.rs `tests_export_applies_display_safe_to_pathological_url` exercises and verifies — pass per `fd21900` GREEN state. ✓

The spec did not drift between sessions; the implementation tracks the operator-confirmed spec. The DESIGN.md, read cold, would produce the current implementation.

**Resolution:** Phase-progression discipline holds across the five-commit Layer 3 sequence; the Layer 2 R5 F1 two-commit canonical shape forward-applied cleanly; Phase 2b implementation matches the operator-confirmed spec from `654cbbf`. The Layer 3 cycle is the canonical Red Gate evidence-preservation worked example for future projects.

**Classification:** Resolved — phase-progression + Red Gate two-commit shape + cross-session spec consistency all operative.

---

**Finding 2 — Phase 2c extract-and-name annotation honesty: TODO.md:144 names the pre-planned extraction trigger (Layer 2 precedent, not lint-driven) + independent justification + rejected counter-refactor; G-161 closure operative (Dim 12)**

<a id="r1-f2"></a>

**Owner:** vdd-iar-alignment
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

[TODO.md:144](../../TODO.md) Phase 2c annotation reads:

> "Phase 2c (refactor): extract-and-name applied at Phase 2b commit `fd21900` ... The trigger here was NOT clippy's `too_many_lines` lint (the consolidated `main()` would have been ~145 lines, above the 100-line limit — but the extraction was pre-planned per the Layer 2 precedent, not lint-driven). The refactor is justified independent of the lint floor: each helper reads as a complete top-to-bottom subcommand contract ... No further refactor warranted at Phase 2c — the five `run_*` helpers ... share variations on the load-store-emit pattern but their per-subcommand control flow + validation order differs (e.g., `run_import` validates stdin BEFORE loading the store so empty-stdin doesn't create the file); a `load_store_or_emit` helper would obscure those differences without reducing line count materially."

Annotation discipline checklist (vs. the Layer 2 R4 F2 honest-two-part pattern):

1. ✓ **Trigger named honestly** — "The trigger here was NOT clippy's `too_many_lines` lint ... but the extraction was pre-planned per the Layer 2 precedent." The annotation distinguishes itself from the Layer 2 case (where clippy WAS the proximate trigger) and names the actual trigger (Layer 2 precedent + pre-plan).
2. ✓ **Independent justification named** — "each helper reads as a complete top-to-bottom subcommand contract" + matches the established Layer 2 R2 per-subcommand-helper pattern.
3. ✓ **Counter-refactor explicitly considered and rejected** — "a `load_store_or_emit` helper would obscure those differences without reducing line count materially" with the per-subcommand control-flow-differs rationale (specifically the `run_import` empty-stdin-validation-before-load case).
4. ✓ **G-161 cited** — "Phase 2c satisfies VDD-IAR Alignment dim 12 per G-161 — the extract-and-name annotation here is the alternative to a silent-skip finding."

Cross-check: does the refactor in src/main.rs match the annotation? Yes — `fd21900` introduces `run_export` + `run_import` parallel to the Layer 2 `run_add` / `run_list` / `run_tag` extraction. The diff shows no new behavior paths beyond the Phase 2a Red Gate's specified contracts. Phase 2c primer's "no new behavior paths beyond Phase 2b" requirement satisfied. ✓

Phase 2c primer § Completion criteria #5 specifically requires either a commit OR an annotation in TODO.md / crosslink session note. `78bd3cf` is a dedicated Phase 2c annotation commit (parallel to Layer 2's `98b5886`); the audit trail is doubly satisfied (annotation in TODO.md + dedicated commit). ✓

**Resolution:** Phase 2c annotation is honest about the refactor's source (Layer 2 precedent + pre-plan, NOT lint-driven — distinguishing itself from Layer 2's annotation explicitly). The discipline is operative; G-161 closed cleanly.

**Classification:** Resolved — Phase 2c annotation discipline operative; the dedicated `78bd3cf` annotation commit is the cleanest possible audit trail form.

---

**Finding 3 — Phase 5 + Phase 6 strategy declarations for Layer 3 are present in DESIGN.md § Project intent with named-rationale-bearing form per G-162 strict-form completeness (Dim 1 + Dim 13 + Dim 14)**

<a id="r1-f3"></a>

**Owner:** vdd-iar-alignment
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

[DESIGN.md:15](../../DESIGN.md) § Phase 5 strategy for Layer 3 declares:

> "Layer 3 (AI-co-authored; operator-owned): Purity Boundary Audit re-runs against the extended pure surface (export-serialize + import-deserialize + dedup-on-exact-tuple-match — all pure functions of the input JSON + existing store state); Mutation Testing re-runs against the extended impl with the same 100%-kill-rate budget; proptest extends with a round-trip property — for any valid storage-state X, parse(serialize(X)) == X (export + re-import round-trip invariant) AND with an `import(import(X)) == import(X)` idempotence property exercising the dedup rule; Fuzz Testing now warranted — bm import is the project's first untrusted-input surface (stdin-fed JSON from an attacker-controlled pipe), making it the natural first fuzz target. The fuzz harness uses cargo-fuzz with libFuzzer to feed arbitrary byte sequences as stdin to the import deserialize path; the bug class targets are parse-panic / parse-OOM / parse-stack-overflow / any non-error-result behavior outside the spec'd Exit 1 / Exit 2 paths. Proof Execution remains not applicable (no safety-critical / cryptographic primitives even with Layer 3)."

G-162 strict-form check for Phase 5:

- ✓ `planned — <named tooling and scope>` form (not `TBD` / `future`)
- ✓ Surfaces named: Purity Boundary Audit + Mutation Testing + property-based testing (proptest) + Fuzz Testing (cargo-fuzz with libFuzzer)
- ✓ Surfaces explicitly skipped named with rationale: Proof Execution `not applicable — no safety-critical / cryptographic primitives even with Layer 3`
- ✓ Per-surface specific scope: round-trip + idempotence properties named for proptest; per-bug-class targets named for fuzz (parse-panic / parse-OOM / parse-stack-overflow)
- ✓ Layer-specific motivation: "bm import is the project's first untrusted-input surface" justifies Fuzz Testing activation specifically at Layer 3

[DESIGN.md:17](../../DESIGN.md) § Phase 6 strategy for Layer 3 declares:

> "Layer 3 four-dimensional convergence (AI-co-authored; operator-owned): NOT APPLICABLE per the same G-150 + G-112 rationale as Layer 2 — capstone gates at project-terminal MVR per primer 6, not per-layer; running Phase 6 for Layer 3 would re-teach the same not-applicable disposition the Layer 2 declaration already documents. The Phase 5 hardening at Layer 3 still occurs (Purity Boundary Audit re-run + Mutation Testing re-run + proptest round-trip + cargo-fuzz on bm import); Phase 6 specifically (four-dimensional convergence attestation) is the not-applicable part."

G-162 strict-form check for Phase 6:

- ✓ `not applicable — <real rationale>` form (not silence; not `TBD`)
- ✓ Named rationale: G-150 over-investment guard + G-112 reference-implementation-purpose-already-satisfied
- ✓ Methodology-precedent concern addressed: "running Phase 6 for Layer 3 would re-teach the same not-applicable disposition the Layer 2 declaration already documents" — explicitly invokes the Layer 2 precedent (R5 F5 closure)
- ✓ Phase 5 vs. Phase 6 disposition split made explicit: "The Phase 5 hardening at Layer 3 still occurs ... Phase 6 specifically ... is the not-applicable part"

Both declarations meet G-162 strict-form completeness. The Phase 6 NA disposition forward-applies the Layer 2 R5 F5 precedent cleanly — a future capstone-project author reading bookmark-cli-manual sees Layer 1 Phase 6 attested + Layer 2 NA + Layer 3 NA and infers the canonical pattern: Phase 6 is per-project (not per-layer); multi-layer projects attest once at project-terminal MVR.

**Resolution:** G-162 strict-form Phase 5 + Phase 6 completeness met for Layer 3; both strategy lines are named-rationale-bearing; Phase 6 NA disposition forward-applies the Layer 2 precedent cleanly with explicit "re-teach the same disposition" framing.

**Classification:** Resolved — Phase 5 + Phase 6 strategy declarations operative per G-162.

---

**Finding 4 — Phase 5 surface deferral is correct timing: proptest round-trip / cargo-fuzz harness / Mutation Testing re-run / Purity Boundary Audit re-run are NOT required at Phase 2b commit; they file under TODO.md § Layer 3 Layer-gate criterion #5 for layer-close (Dim 13)**

<a id="r1-f5"></a>

**Owner:** vdd-iar-alignment
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

Adversarial check: does the absence of `tests/properties.rs` round-trip property + `fuzz/` directory + Layer 3 Mutation Testing rerun + Layer 3 Purity Boundary Audit rerun at the Phase 2b commit constitute a Phase 5 discipline gap?

[TODO.md:152](../../TODO.md) § Layer 3 Layer-gate criterion #5 reads: "Phase 5 Layer 3 rounds at closure: Purity Boundary Audit re-runs against the extended pure surface (export-serialize + import-deserialize + dedup-on-exact-tuple-match); Mutation Testing re-runs with 100% kill rate maintenance or named-rationale drop; proptest round-trip property passes against 1024+ generated cases; cargo-fuzz harness runs for at least 1 CPU-hour against `import_stdin` with no findings."

The "at closure" qualifier scopes Phase 5 surfaces to layer-close (parallel to Layer 2's Phase 5 surfaces filing at the Layer 2 layer-close PR #47 Phase-5-trigger follow-up, NOT at the Phase 2b implementation commit `326e25d`). The Layer 2 precedent is direct evidence — Layer 2 Phase 2b landed at `326e25d` (2026-05-18) and Phase 5 surfaces landed at `1f53540` + `c186d0b` + `1ed337e` + `9989fa8` (2026-05-22 → 2026-05-23), spanning the Layer 2 Round 1 + Round 2 cluster cycles. The Phase 5 surfaces are explicitly NOT a Phase 2b/2c prerequisite — they are a Phase 5 hardening cycle the operator schedules after the IAR cycle's domain-review feedback shapes the Phase 5 budget.

Phase 5 primer § Cold-session-vs-inline decision rubric supports this — Phase 5 surfaces are intentionally scheduled in a separate cycle, not bundled with Phase 2b. The Layer 3 cycle's current shape (Phase 2c annotated at `78bd3cf`; Phase 3 IAR Round 1 opening next; Phase 5 surfaces deferred to layer-close per TODO.md § Layer-gate criterion #5) matches the Layer 2 precedent timing.

**Discipline-honest framing for the Round 1 reviewer:** the Phase 5 surfaces are not yet committed, but they are NOT YET DUE. The TODO.md § Layer-gate criterion is the audit-trail commitment; the deferral is correctly scheduled, not silent-skipped. If Phase 5 surfaces remain absent at the Layer 3 layer-close attempt, THAT is the regression check this finding sets up — re-raise if the Layer 3 close PR ships without the four Phase 5 surfaces named in criterion #5.

**Resolution:** Phase 5 surface deferral to layer-close is correct timing per Phase 5 primer + Layer 2 precedent + TODO.md § Layer-gate criterion #5. The deferral is committed in the audit trail (TODO.md:152) — not silent-skipped. Round N+1 (likely the Layer 3 close round) will regression-check whether the four Phase 5 surfaces actually land.

**Classification:** Resolved — Phase 5 deferral correctly scheduled per Layer 2 precedent + TODO.md commitment; not a discipline gap at Round 1 timing.

---

### Raised to SO

**Finding 4 — `manual-tests/layer-3.md` artifact promised in TODO.md:138 "to be authored alongside the Phase 2a Red Gate commit" is NOT YET PRESENT after the Phase 2c commit `78bd3cf`; pointer-without-target is the Review 74 manual-test convention defect (Dim 5 + Dim 9)**

<a id="r1-f4"></a>

**Owner:** solution-owner
**Status:** raised
**Blocked by:** *(none — operator-authoring task; no upstream dependency)*
**Validator:** solution-owner

[TODO.md:138](../../TODO.md) § Layer 3 manual testing checklist declares:

> "Layer 3 manual testing checklist: `manual-tests/layer-3.md` (to be authored alongside the Phase 2a Red Gate commit) — parallel to `manual-tests/layer-{1,2}.md`. Includes the `bm export | bm import` round-trip canonical workflow + the cross-machine sync workflow via file-transfer-pipe."

Actual state at post-`78bd3cf`:

- `manual-tests/` contains `install-verification.md` + `layer-1.md` + `layer-2.md` only — no `layer-3.md`.
- The Phase 2a Red Gate commit `878d3b6` introduced tests/bookmarks.rs +627 lines and ONLY that file (per `git show --stat 878d3b6`); the promised `manual-tests/layer-3.md` was NOT authored at that commit.
- The Phase 2b commit `fd21900` (src/lib.rs + src/main.rs + CHANGELOG.md) did NOT introduce `manual-tests/layer-3.md`.
- The Phase 2c commit `78bd3cf` (TODO.md + CHANGELOG.md) did NOT introduce `manual-tests/layer-3.md`.

Dim 9 framing per domain prompt: "A project whose `TODO.md` Layer N has `**Manual Testing Checklist:**` pointer but no actual `manual-tests/layer-N.md` file is a finding (the pointer is the discipline; the absent target is the defect)."

This is the Review 74 manual-test-split convention defect at the Layer 3 surface. The pointer at TODO.md:138 commits the project to authoring `manual-tests/layer-3.md`; the absent file is the discipline defect.

**Dim 5 framing (Human verification):** the manual-test checklist is the human-verification artifact. Phase 5 will need it (proptest harness validates `import(export(X)) == X` algebraically; the `bm export | bm import` round-trip canonical workflow as a human-execution-path is what the manual-tests checklist captures — distinct from the proptest). Without it, the operator cannot run-through the Layer 3 capability in the same shape they ran-through Layers 1 + 2. The Layer 3 cycle has no other "human-ran-the-binary" artifact — the Phase 3 IAR Round 1 will need to surface this from multiple domains (likely UX, TW, QE, Doc Reviewer, this domain) because the absence breaks the audit-trail-of-record for human verification.

**Why Raised to SO:** the resolution requires SO-authority operator-action — authoring the `manual-tests/layer-3.md` file matching the `manual-tests/layer-{1,2}.md` pattern. This is operator-authoring work routed back to the Phase 2a/Phase 2c boundary (the file should have landed alongside or shortly after the Red Gate commit per the TODO.md:138 promise). Phase 4 routing applies — this finding routes to Phase 2a (the layer's test-artifact phase; the manual-tests file is the manual-testing-checklist analog to the Red Gate's automated-test plan).

**Phase 6 attestation implication:** none (Layer 3 Phase 6 NA per [Finding 3](#r1-f3)).

**Discipline-honest forward path:**

- Author `manual-tests/layer-3.md` matching the `manual-tests/layer-{1,2}.md` shape; include the `bm export | bm import` round-trip canonical workflow + the cross-machine sync workflow via file-transfer-pipe (both named in TODO.md:138); apply the Nathan-thread "silent on success; the fenced block below is intentionally empty" wording discipline per primer 1c § Manual testing checklist § Empty-output wording discipline.
- Cite the operator-execution-trail (a manual-tests run-through against the binary at the Phase 2c state) as the Dim 5 human-verification evidence for Layer 3 close.
- Route via Phase 4 to Phase 2a (the test-artifact phase); the fix lands in the Layer 3 cycle's Round 2 (or Round 1's fix-cycle commit) before any Phase 5 surfaces open.

**Classification:** Raised to SO — pointer-without-target defect at TODO.md:138 → no `manual-tests/layer-3.md` file. Operator-authoring routes via Phase 4 to Phase 2a per the routing table § "Edge case is in DESIGN.md / TODO.md but no test covers it → Phase 2a."

---

### Dismissed

**Finding 5 — Phase 4 routing has not been applied to Layer 3 Phase 3 IAR findings; this is correct sequencing because Phase 3 IAR Round 1 has not yet completed at the time of this Round 1 (Dim 10)**

<a id="r1-f5b"></a>

**Owner:** vdd-iar-alignment
**Blocked by:** *(none — sequencing-correct dismissal)*

Adversarial check: should Phase 4 routing have been applied to Layer 3 findings before this Round 1 opened?

No — Phase 4 routing is the activity that runs AFTER Phase 3 IAR produces a classified finding set. Per [primer 4](../../../../vsdd-suite/primers/4-feedback-integration.md) § opening prose: "Use this prompt after a Phase 3 IAR round has produced a classified finding set and before the next implementation pass begins."

The Layer 3 cycle's Phase 3 IAR Round 1 is happening NOW — this VDD-IAR Alignment review is one of N parallel domain reviews opened against the post-`78bd3cf` Phase 2c state. The operator's per-domain prompt notes that the main session "is about to run Phase 4 routing AFTER your review." This is the correct sequencing: Phase 3 IAR Round 1 produces the classified finding set across all 13 active domains → main session runs Phase 4 routing on the aggregate finding set → routed work lands in Round 1.5 fix commits → Round 2 cold pass verifies.

**Discipline-honest framing:** the absence of Phase 4 routing at the time of this Round 1 is NOT a Dim 10 defect — it's the correct sequencing. The Dim 10 defect would be Phase 4 routing applied BEFORE Phase 3 IAR completes (which would route un-classified findings, violating primer 4's opening discipline) OR Phase 4 routing SKIPPED after Phase 3 IAR completes (which would let findings be patched without routing-to-earliest-phase analysis).

**Phase 4 routing-readiness signal for the main session:** the operator-action queue closing this round has everything needed to run Phase 4 routing per primer 4 immediately after the Round 1 finding-aggregate closes. The routing table at primer 4 § Finding-to-phase routing table is the operative reference; the Layer 3 Round 1 finding-set will mostly route to Phase 2b (implementation defects) + Phase 2a (test-coverage gaps) + Phase 1a/1b (spec gaps surfaced by the cold adversary). The single Open finding from this round ([Finding 4](#r1-f4) — `manual-tests/layer-3.md` absence) routes to Phase 2a (the test-artifact phase).

**Classification:** Dismissed — sequencing-correct; Phase 4 routing IS the next-step activity, not a Round 1 prerequisite. The dismissal IS the Phase 4 routing-readiness signal to the main session.

---

**Finding 6 — Adversarial-pair separation (Dim 11) cannot be evaluated at Round 1 of the Layer 3 cycle because the Phase 3 IAR cluster spawn for Layer 3 has not yet happened; defer to a later round for the cluster-shape audit (Dim 11)**

<a id="r1-f6"></a>

**Owner:** vdd-iar-alignment
**Blocked by:** *(none — defer-to-future-evidence dismissal)*

Adversarial check: should adversarial-pair separation (Security ↔ Red Team and TW ↔ Doc Reviewer on different cluster agents per the Phase 3 primer § Pre-cycle methodology check) be evaluated at this Round 1?

No — this Round 1 is opening as the first review session of the Layer 3 cycle. Adversarial-pair separation is verified via:

- review-log filenames (each domain produces a per-date per-domain log) for cross-domain timing
- per-Review entry preamble's Cold-session shape field (which names the cluster composition)

The Layer 3 cycle's Phase 3 IAR cluster spawn has not yet been committed at the time of this review — no `2026-05-24-{security,red-team,technical-writer,documentation-reviewer,...}.md` review logs exist yet. The cluster shape will be evaluable AFTER the Phase 3 IAR Round 1 cluster spawn lands its per-domain review-log files.

**Discipline-honest framing:** the absence of the cluster-shape evidence at the time of this Round 1 is NOT a Dim 11 defect — it's the correct sequencing (VDD-IAR Alignment is one of the cluster's domains; the cluster's spawn shape is verifiable from the aggregate per-domain commits, not from the in-flight first-domain-to-commit). The Layer 2 R5 F3 cluster-shape evaluation happened at Round 2 (post-fix-cycle); the Layer 3 cluster-shape evaluation similarly fits at the equivalent forward round.

**Forward-looking regression check for Round 2:** Round 2 (or whichever round opens after the Layer 3 Phase 3 IAR Round 1 fix cycle) should verify:

- Security ↔ Red Team split across cluster agents ✓ (Layer 2 precedent)
- TW ↔ Doc Reviewer split across cluster agents ✓ (Layer 2 precedent)
- The Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster composition is the natural meta-cluster placement (the operator's prompt note about "the SO ↔ VDD-IAR Alignment validator-pair is co-located in this cluster" is acceptable per Review 77 lifecycle — validator-pair, not cold-reader-vs-author adversarial pair).

**Classification:** Dismissed — defer-to-future-evidence; adversarial-pair separation is verifiable from cross-domain cluster-spawn artifacts that do not exist yet at Round 1 of the Layer 3 cycle. Round 2 (post-Phase-3-IAR-Round-1) is the appropriate evaluation point.

---

### Hallucinated

*(none)*

---

### Summary

**Round 1 closes at:** 1 Open ([Finding 4](#r1-f4) — `manual-tests/layer-3.md` artifact-promised-but-absent at TODO.md:138) + 4 Resolved (Phase-progression discipline + Phase 2c annotation honesty + Phase 5 + Phase 6 G-162 strict-form completeness + Phase 5 surface deferral correctly scheduled) + 2 Dismissed (Phase 4 routing not-yet-applied is correct sequencing; adversarial-pair separation defer-to-future-evidence) + 0 Hallucinated.

**MVR signal:** **NOT REACHED.** The single Open finding ([Finding 4](#r1-f4)) requires operator-authoring of `manual-tests/layer-3.md` to close. Per [G-131](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-131) continue-trigger discipline, a Round 2 cold pass after the fix lands is mandatory. Per the canonical Layer 2 R4 → R5 → R6 round shape, Round 2 verifies the fix held + spot-checks adjacent surfaces for regression.

**Phase 4 routing-readiness signal:** The main session has everything needed to run Phase 4 routing per [primer 4](../../../../vsdd-suite/primers/4-feedback-integration.md) immediately after Round 1's aggregate finding-set closes:

- The classified finding set across all 13 active domains will be the routing-input (this VDD-IAR Alignment round is one of those 13 inputs).
- The Phase 4 routing table at primer 4 § Finding-to-phase routing table is the operative reference.
- This round's single Open finding ([Finding 4](#r1-f4)) routes to **Phase 2a** per the routing table § "Edge case in DESIGN.md / TODO.md but no test covers it → Phase 2a" — specifically the manual-tests-as-test-plan phase. The fix is authoring `manual-tests/layer-3.md` per the TODO.md:138 promise.
- Cross-domain coordination: [Finding 4](#r1-f4) is likely raised by UX + TW + Doc Reviewer + QE rounds from their respective lenses; Phase 4 routing should consolidate these into the single Phase 2a fix.

**Regression-check disposition:** the Layer 2 R5 F1 Red Gate evidence-preservation closure's forward-looking discipline ("Layer 3 + future projects: the canonical shape is two commits") was inherited cleanly by the Layer 3 cycle ([Finding 1](#r1-f1)). The methodology-recurrence prevention is operative — the Layer 2 annotation's payload reached the Layer 3 author.

**Source:** `domain-raised`
**Round:** 1
**Validator pair:** Per [VDD-IAR Alignment domain prompt](../../../../vsdd-suite/domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md) § Validator pair — `solution-owner` is the natural validator for VDD-IAR Alignment findings (SO confirms the process-discipline finding doesn't conflict with the project's declared intent). [Finding 4](#r1-f4)'s validator at fix-time is `solution-owner` (the operator-authoring of the `manual-tests/layer-3.md` file is the SO-routed fix). The four Resolved findings carry `sanity-check` per the meta-validator-of-last-resort default — no cross-domain validation surface beyond this round's discipline-check.

**Coordination:** [Finding 4](#r1-f4) (manual-tests/layer-3.md artifact-promised-but-absent) is likely raised by parallel domain reviews from UX + TW + Doc Reviewer + QE lenses (each domain has a stake in the manual-tests artifact's presence — UX for the user-flow narrative; TW for the prose-quality + Nathan-thread wording discipline; Doc Reviewer for the cold-reader pass; QE for the human-verification-of-automated-tests-coverage gap). Phase 4 routing should consolidate any cross-domain raises into a single Phase 2a fix; the consolidated fix re-validates this round's [Finding 4](#r1-f4) at Round 2. [Finding 5](#r1-f5b) (Phase 4 routing not-yet-applied at Round 1) is the routing-readiness signal to the main session — no cross-domain coordination needed. [Finding 6](#r1-f6) (adversarial-pair separation defer-to-future-evidence) coordinates with the upcoming Phase 3 IAR Round 1 cluster-spawn audit at the equivalent Round 2 timing.

---

## Review 2 — 2026-05-25 04:30Z

**Round:** Layer 3 Phase 3 IAR Round 2.
**Scope:** Layer 3 cycle process-compliance audit for the Round 1 fix-work commit sequence (`fdfa989` Phase 1a+1b → `ba6a4a9` Phase 2a → `bfc0713` Phase 2b → `795bc25` Phase 2c-equivalent + manual-tests/layer-3.md) PLUS the Round 2 launch sequence itself PLUS two out-of-band methodology events that landed mid-cycle: (a) the Phase-2b architectural correction sub-decision (display_safe removal from `export_json` beyond Round 1 routing scope), and (b) the in-cycle suite-hardening landing at [`e4b6701`](../../../../vsdd-suite/suite-development/review-log/2026-05-24-suite-review.md#review-94--2026-05-25-0300z) (Review 94 + new [`check-no-letter-clusters.py`](../../../../vsdd-suite/hooks/check-no-letter-clusters.py) hook + [primer 4](../../../../vsdd-suite/primers/4-feedback-integration.md) § Routing output Cluster identifier discipline paragraph). Re-verifies the 14 [VDD-IAR Alignment](../../../../vsdd-suite/domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md) dimensions against the post-fix-work artifact state.

**Session note:** Cold context — this reviewer did not author any of the four Round 1 fix-work commits, did not adjudicate the Phase-2b architectural correction sub-decision, did not author Suite Review 94, and did not write the Round 2 launch prompt. Sycophancy guard applied: the temptation in a Round 2 review is "the fixes landed, the methodology held, the cycle is at MVR" — every dim below cites the specific artifact line that confirms the discipline, not the cycle's apparent forward motion. The Round 1 Resolved findings are re-checked for regression (Layer 2 R5 F1 inheritance discipline applied to the Round 1 → Round 2 boundary).

**Source:** `domain-raised` — the cold adversary applying the 14 VDD-IAR Alignment dimensions to the post-`795bc25` + post-`e4b6701` artifact state surfaced the findings below. The session-opening dispatch (Phase 3 IAR Round 2 against bookmark-cli-manual Layer 3) is `director-raised` at the spawn-orchestration level; the per-cycle Round 2 launch sequence is itself one of the audit targets.

**Reference:** [VSDD whitepaper](https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00); [VDD-IAR Alignment domain prompt](../../../../vsdd-suite/domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md); [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md); [Phase 4 primer](../../../../vsdd-suite/primers/4-feedback-integration.md) (post-[Review 94](../../../../vsdd-suite/suite-development/review-log/2026-05-24-suite-review.md#review-94--2026-05-25-0300z) Cluster identifier discipline amendment).

**Program phase:** Phase 1 (reference-implementation tier; dim 11 issue-tracking compliance not applicable per the domain prompt § Program Phase Context).

**Regression-check against:** [Review 1](#review-1--2026-05-25-0115z) — re-verify the 4 Resolved findings hold at the post-fix-work state, and verify the single Raised-to-SO finding ([R1 F4](#r1-f4) manual-tests/layer-3.md absence) closed at `795bc25`.

**Round:** 2 (Layer 3 cycle; this is the second VDD-IAR Alignment round for Layer 3).
**Active domain set:** 13 (12 role + 1 meta) per [DESIGN.md](../../DESIGN.md) § Project intent.

**MVR signal:** **NOT REACHED at Round 2.** This round surfaces 2 substantive Open findings — [Finding 1](#r2-f1) (Round 2 pre-cycle declaration was operator-asserted in the launch prompt but NOT committed as a suite-side audit-trail entry per [AIE R1 F6 closure](2026-05-24-ai-engineer.md#r1-f6) routing) and [Finding 2](#r2-f2) (architectural correction sub-decision at `bfc0713` was operator-authorized via AskUserQuestion but the spec-vs-impl alignment at [DESIGN.md](../../DESIGN.md) § `bm export` (Layer 3) § Success-output is NOT re-amended to reflect the display_safe-removed-from-export_json shape) — 4 discipline-honest Resolved closures, and 1 Dismissed-with-rationale disposition. Round 3 trigger fires per [G-131](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-131): the 2 Open findings require fix-work + Round 3 cold re-verify before MVR.

---

### Resolved

<a id="r2-f3"></a>
**Finding 3 — Round 1 fix-work commit sequence (`fdfa989` → `ba6a4a9` → `bfc0713` → `795bc25`) walks Phase 1a+1b → 2a → 2b → 2c-equivalent in canonical order; the two-commit canonical Phase 2a/2b shape from Layer 2 R5 F1 forward-applied cleanly across the Round 1 fix boundary (Dim 1 + Dim 3 + Dim 4 + Dim 7 + Dim 9)**

**Owner:** vdd-iar-alignment
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

Phase-progression discipline check across the Round 1 fix-work boundary:

| Commit | Phase | Timestamp | Evidence |
|---|---|---|---|
| `fdfa989` | Phase 1a+1b (spec amendments + narrative updates) | 2026-05-24 19:36:34 -0700 | 13 finding-cluster fixes; DESIGN.md + README.md + CHANGELOG.md + PROCESS.md + FINDINGS-INDEX.md + install-verification.md changes; spec + narrative-only; no test or impl changes |
| `ba6a4a9` | Phase 2a Round 1 fix (6 new tests; tests-only commit) | 2026-05-24 19:40:44 -0700 | `tests/bookmarks.rs` +254 lines; commit message preserves canonical Red Gate failure mode evidence for 3 RED tests (`tests_export_import_round_trip_preserves_pathological_bytes`, `tests_import_dedup_treats_tags_as_set_under_reorder`, `tests_import_rejects_control_char_in_tags`) + 3 GREEN coverage tests |
| `bfc0713` | Phase 2b Round 1 fix (4 impl fixes + architectural correction sub-decision) | 2026-05-24 19:54:47 -0700 | `src/lib.rs` + `src/main.rs` + `DESIGN.md` (minor edge-case entry edits only) + `CHANGELOG.md` + `tests/bookmarks.rs` (minor; existing-test adjustments for the architectural correction); 51/51 tests GREEN; 0 clippy warnings |
| `795bc25` | Phase 2c-equivalent (manual-tests/layer-3.md) + Phase 2c follow-up annotation | 2026-05-24 20:00:24 -0700 | New 572-line `manual-tests/layer-3.md`; `TODO.md:146` Phase 2c follow-up annotation for the `bfc0713` Round 1 fix-work commit |

Commit-timestamp order is canonical: Phase 1a+1b (19:36) → Phase 2a (19:40) → Phase 2b (19:54) → Phase 2c-equivalent + annotation (20:00). The two-commit canonical Phase 2a/2b shape is preserved at the Round 1 fix boundary: `ba6a4a9` lands the 3 RED regression tests alone (verifiable: `git checkout ba6a4a9 && cargo test --test bookmarks tests_export_import_round_trip_preserves_pathological_bytes` would fail per the commit-message's `test result: FAILED. 1 failed (RED — expected)` preserved evidence); `bfc0713` lands the impl that turns the 3 RED tests GREEN. ✓

Dim 9 (no-fix-without-rationale) check: each fix commit's message explicitly cites the routing record at per-domain Phase 4 routing appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md` (per-domain Phase 4 appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md`):

- `fdfa989`: "spec amendments + narrative updates for Round 1 routed findings (13 finding-cluster fixes; 39 FINDINGS-INDEX rows backfilled) ... per the routing record at per-domain Phase 4 routing appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md`."
- `ba6a4a9`: "6 new regression + coverage tests for routed Round 1 findings (3 RED defects + 3 GREEN QE coverage gaps) ... per the routing record at per-domain Phase 4 routing appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md`."
- `bfc0713`: "impl fixes for 4 routed substantive findings ... per the routing record at per-domain Phase 4 routing appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md`."
- `795bc25`: "manual-tests/layer-3.md per Round 1 Phase 4 routing (closes the 7-domain Layer-3-docs-staleness sub-cluster on manual-tests-absence + SO R1 F1 Backlogged + criterion 3 of the Layer 3 layer-gate criteria)."

Dim 7 cross-session spec consistency check (subset; full spec-vs-impl audit deferred to [Finding 2](#r2-f2) below for the architectural correction sub-decision):

- Sorted-tag-comparison dedup spec ([DESIGN.md](../../DESIGN.md) § Why dedup-on-sorted-tag-comparison) matches impl ([`src/lib.rs:680`](../../src/lib.rs) `bookmark_set_eq` private helper). ✓
- Control-char tag rejection spec ([DESIGN.md](../../DESIGN.md) § `bm import` (Layer 3) § Failure (imported record contains control-char tag)) matches impl ([`src/lib.rs`](../../src/lib.rs) new `ImportError::TagContainsControlChars` variant). ✓
- `manual-tests/layer-3.md` exists (572 lines, parallel to `manual-tests/layer-{1,2}.md`); the [R1 F4](#r1-f4) Raised-to-SO finding is closed at `795bc25` per the Phase 4 routing record § Layer-3-docs-staleness cluster A1 sub-cluster. ✓

**Resolution:** Round 1 fix-work phase-progression discipline holds; canonical Phase 2a/2b two-commit shape forward-applied cleanly across the Round 1 fix boundary; each fix commit names the routing destination per Dim 9; [R1 F4](#r1-f4) closed at `795bc25`. The Round 1 → Round 2 boundary inherits Layer 2 R5 F1 discipline cleanly.

**Classification:** Resolved — phase-progression + Red Gate two-commit shape + routing-citation + R1 F4 closure all operative.

---

<a id="r2-f4"></a>
**Finding 4 — Phase 2a Round 1 fix tests are behavior-assertive (not just exit-code probes); each of the 6 new tests asserts spec-named behavior at a specific assertion line and would fail against a stub or wrong-behavior impl (Dim 5)**

**Owner:** vdd-iar-alignment
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

Dim 5 check on the 6 new Phase 2a tests at [`ba6a4a9`](../../tests/bookmarks.rs):

| Test | Assertion lines | Behavior-assertion shape |
|---|---|---|
| `tests_export_import_round_trip_preserves_pathological_bytes` | [1761-1764](../../tests/bookmarks.rs) | `assert_eq!(dst_url, pathological_url, ...)` — byte-equal comparison of destination URL against source URL after round-trip; fails against pre-fix Rust-syntax `\u{HHHH}` impl per preserved commit-message RED evidence |
| `tests_import_dedup_treats_tags_as_set_under_reorder` | [1803-1807](../../tests/bookmarks.rs) | `assert_eq!(parsed["bookmarks"].as_array().unwrap().len(), 1, "tag-reorder must NOT create a duplicate row per sorted-tag-comparison dedup")` — length assertion against the post-dedup store state |
| `tests_import_rejects_control_char_in_tags` | [1829-1837](../../tests/bookmarks.rs) | `predicate::str::starts_with("Error: imported bookmark tags contain disallowed control characters.")` + `predicate::str::is_empty()` on stdout + `assert!(!db.exists())` on no-file-write — three-pronged behavior assertion |
| `tests_import_dedup_collapses_within_payload_byte_equal_records` | [1869-1873](../../tests/bookmarks.rs) | `assert_eq!(parsed["bookmarks"].as_array().unwrap().len(), 1, "within-payload byte-equal records must collapse to one appended")` |
| `tests_export_applies_display_safe_to_pathological_tag` | [1907-1922](../../tests/bookmarks.rs) | Two-stage assertion: `assert!(!rendered.contains('\u{001b}'))` (raw byte absent from JSON output) + `assert!(tag.contains('\u{001b}'))` (byte present after JSON parse round-trip per byte-preservation) — encodes the architectural correction sub-decision's intent as a test invariant |
| `tests_import_max_stdin_bytes_operator_override` | [1942-1946](../../tests/bookmarks.rs) | `predicate::str::starts_with("Error: stdin exceeded maximum byte limit of 50 bytes")` — specific error-message + cap-value assertion (not just `.failure()`) |

Each test asserts a spec-named behavior contract, not just exit code or file existence. The 3 RED tests' commit-message preserves the failure-mode-evidence (`test result: FAILED. 1 failed (RED — expected)`); the 3 GREEN coverage tests close pre-existing implementation-correct-but-untested paths surfaced by QE Round 1 F1/F2/F3. ✓

Adversarial spot-check: would `tests_import_dedup_treats_tags_as_set_under_reorder` pass against an `import_json` impl that always succeeds without dedup? No — the second import would append a second record, the array length would be 2, and the assertion would fire with the named-rationale failure message. ✓

**Resolution:** Dim 5 behavior-assertive test discipline operative at the Round 1 fix boundary; the 6 new tests assert spec-named behavior at named assertion lines; the 3 RED tests' preserved failure-mode evidence is the Phase 2a canonical Red Gate audit trail.

**Classification:** Resolved — Phase 2a Round 1 fix tests are behavior-assertive per Dim 5.

---

<a id="r2-f5"></a>
**Finding 5 — Phase 4 routing record at per-domain Phase 4 routing appendices (per-domain Phase 4 appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md`) exists + each per-domain Round 1 finding has a routing decision; adversarial-pair separation operative (Security ↔ Red Team and Technical Writer ↔ Documentation Reviewer on independent cold sessions); the [R1 F5](#r1-f5b) (Phase 4 routing not-yet-applied) and [R1 F6](#r1-f6) (adversarial-pair separation defer-to-future-evidence) Dismissals from Round 1 are now closed by direct evidence (Dim 10 + Dim 11)**

**Owner:** vdd-iar-alignment
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

Dim 10 (Phase 4 routing applied) check: per-domain Phase 4 routing appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md` (per-domain Phase 4 appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md`) exists (398 lines; committed at `e233ad8`). The routing record's § Routing summary by cluster enumerates each routable Round 1 finding across 13 thematic clusters (JSON-native escape design; sorted-tag-comparison dedup; imported-tag control-char rejection; imported-tag classification extension; Layer-3-docs-staleness sub-clusters A1-A7; QE test-coverage gaps; UX help-and-error-remediation; DESIGN.md verification-architecture refresh; dedup-complexity accepted-limit; ImportError variant detail deferred; import_json doc-comment fix; AIE process-discipline carry-forward; Phase-5 cargo-fuzz tracking; per-domain numbering convention) + § Routing summary by cluster's per-cluster Route field names the destination phase per primer 4's routing table. ✓

Dim 10 reflection (per primer 4 § Anti-patterns) check: the routing record's § Phase 4 reflection table enumerates per-phase finding count — Phase 1a+1b = ~13; Phase 2a = ~7; Phase 2b = ~8; Phase 2c = 0; Phase 5 = 1 tracking; Phase 4 itself = 2 (AIE F6 + F7); Suite-development = 1 (numbering convention); Terminal-no-route = ~5. Phase 2b count (8) is moderate not dominant — primer 4 § Anti-patterns primary failure mode ("routing every finding to Phase 2b") is correctly avoided. ✓

Dim 11 (adversarial-pair separation) check: the Round 1 per-domain review logs at [`vsdd-suite/review-log/2026-05-24-*.md`](.) confirm Security ↔ Red Team and TW ↔ Documentation Reviewer split across independent cold sessions:

- [Security Review 1](2026-05-24-security.md) names "Adversarial pair: [Red Team Review 1 Layer 3](2026-05-24-red-team.md) (separate cold-session, structurally isolated per the cluster discipline)" + own session-note ("Cold-context single-domain session").
- [Red Team Review 1](2026-05-24-red-team.md) is at a distinct review-log filename + carries `Source: domain-raised` per the cold-adversary discipline; the two files are separately committed at `2acc418`.
- [Technical Writer Review 1](2026-05-24-technical-writer.md) and [Documentation Reviewer Review 1](2026-05-24-documentation-reviewer.md) similarly separate cold sessions.

The adversarial-pair separation that [R1 F6](#r1-f6) deferred to Round 2 evidence-collection is operative — the cluster-spawn artifacts the prior round couldn't audit are now in-place. ✓

Dim 12 (Phase 2c discipline) check on the `795bc25` Phase 2c follow-up annotation at [`TODO.md:146`](../../TODO.md):

> "Phase 2c follow-up annotation for Round 1 fix-work commit `bfc0713` (no additional refactor required): The Round 1 Phase 2b fix-work landed at `bfc0713` added a new `ImportError::TagContainsControlChars` variant + new `bookmark_set_eq` private helper + new per-validation-step branches in `import_json` (control-char rejection; sorted-tag-comparison dedup) + new `run_import` validation branches (lower-bound `--max-stdin-bytes` check; size-cap remediation hint; new ImportError arm). These additive changes preserve the established per-subcommand-helper structure from `fd21900`; the five `run_*` helpers remain the canonical organizational pattern. The `display_safe` architectural correction sub-decision (export_json now delegates to serde's native encoder rather than wrapping with `display_safe`) is structural simplification, not a new refactor — the function got shorter + cleaner. No new helper-extraction opportunities surfaced during the Round 1 fix-work; the additive changes stayed inside the existing helper boundaries. Phase 2c satisfies VDD-IAR Alignment dim 12 per G-161 for the Round 1 fix-work commit batch as well — silent-skip would be a finding; this follow-up annotation is the alternative."

Honest-discipline check: the annotation distinguishes between (a) additive control-flow paths that preserve the existing helper structure (refactor-warranted-no) and (b) the display_safe-removal architectural correction (structural-simplification-no-new-helpers-warranted). The annotation is honest about the additive new branches in `import_json` + `run_import` and does not pretend the Phase 2b commit was diff-zero. ✓

Dim 13 (Phase 5) + Dim 14 (Phase 6) check: [DESIGN.md:15](../../DESIGN.md) § Phase 5 strategy + [DESIGN.md:17](../../DESIGN.md) § Phase 6 strategy unchanged from the Round 1 state per [R1 F3](#r1-f3); Phase 5 deferral to post-Round-2-MVR per [TODO.md](../../TODO.md) § Layer 3 Layer-gate criterion #5 remains operative; Phase 6 NA declaration unchanged. ✓

**Resolution:** Phase 4 routing applied per Dim 10 with anti-pattern audit operative; adversarial-pair separation per Dim 11 confirmed by independent per-domain review-log filenames; Phase 2c discipline per Dim 12 operative for the Round 1 fix-work commit batch via the [`TODO.md:146`](../../TODO.md) follow-up annotation; Phase 5 + Phase 6 strategy declarations per Dim 13/14 unchanged from Round 1's [R1 F3](#r1-f3) Resolved state. The Round 1 Dismissals at [R1 F5](#r1-f5b) + [R1 F6](#r1-f6) are now closed by direct evidence.

**Classification:** Resolved — Dim 10 + Dim 11 + Dim 12 + Dim 13 + Dim 14 all operative at the post-Round-1-fix-work state.

---

<a id="r2-f6"></a>
**Finding 6 — In-cycle suite-hardening landing at [`e4b6701`](../../../../vsdd-suite/suite-development/review-log/2026-05-24-suite-review.md#review-94--2026-05-25-0300z) (Suite Review 94 + new [`check-no-letter-clusters.py`](../../../../vsdd-suite/hooks/check-no-letter-clusters.py) hook + [primer 4](../../../../vsdd-suite/primers/4-feedback-integration.md) § Routing output Cluster identifier discipline) is an acceptable in-cycle escalation per operator authorization; the landing did NOT bypass methodology discipline and is correctly framed as "the catch fires too late at Round 2; mechanical commit-time hook is the timing-shift fix" (Dim 12 cross-cycle reading + meta-methodology integrity check)**

**Owner:** vdd-iar-alignment
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

Adversarial check: was the in-cycle suite-hardening at `e4b6701` (Suite Review 94 + new check-no-letter-clusters.py hook + primer 4 amendment, landed mid-PR-#52 between Round 1 fix-work and Round 2 launch) a methodology-discipline gap (scope-creep against the PR's declared Layer 3 IAR cycle scope) or an acceptable in-cycle escalation?

Suite Review 94's commit message + entry frames the three meta-findings as "all about WHEN the methodology catches a defect rather than WHETHER it catches it" — Phase 4 routing bypass (deferred); VSDD phase-frequency guidance gap (deferred); letter-label anti-pattern 4th recurrence (partially-resolved in-cycle). The partial in-cycle fix scope (Finding 3 only) is operator-authorized per the commit message ("Operator authorized in-cycle partial fix on PR #52") + the entry's § Resolution path.

Discipline-honest framing checks:

1. ✓ **The fix scope is bounded.** Only Finding 3 (letter-label 4th recurrence) lands in-cycle; Findings 1 + 2 are explicitly Deferred to post-PR-#52-merge suite-hardening cycle. The scope-creep guard fires correctly: only the one finding with active in-cycle cost (letter labels in the just-created Phase 4 routing record (per-domain Phase 4 appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md`) that the Round 2 reviewers would have to navigate) is fixed in-cycle.
2. ✓ **The mechanical hook + primer amendment are scoped to the future-prevention surface.** [`hooks/check-no-letter-clusters.py`](../../../../vsdd-suite/hooks/check-no-letter-clusters.py) (new file; pre-commit hook) catches forbidden patterns (`Cluster <letter>`, `Surface <letter>(.<digit>)?`, `Path <letter>`, `Option <letter>`) at commit-time. [Primer 4](../../../../vsdd-suite/primers/4-feedback-integration.md) § Routing output § Cluster identifier discipline paragraph codifies the rule. Neither change affects the project's Layer 3 implementation, tests, spec, or audit trail — they are pure future-prevention surfaces.
3. ✓ **The legacy preservation via hook-bypass is forward-only.** The [`e4b6701`](../../../../vsdd-suite/suite-development/review-log/2026-05-24-suite-review.md#review-94--2026-05-25-0300z) commit adds hook-bypass markers to `vsdd-suite/CHANGELOG.md` + `vsdd-suite/suite-development/FINDINGS-INDEX.md` + `vsdd-suite/suite-development/SUITE-DEVELOPMENT-REVIEW.md` per the G-89 forward-only narrative-preservation convention — historical letter-label references are preserved as historical audit trail; new artifacts are constrained.
4. ✓ **The future-revisit scope is named, not buried.** Two future-revisit items (co-authoring evaluation for shape+content enforcement domains; staleness-hook layered defense) are explicitly queued for post-merge cycles per the operator directive "When we revisit this...". The deferral is committed in the audit trail; not silent-skipped.

The in-cycle landing is methodology-correct: an out-of-band escalation that (a) is operator-authorized + (b) is bounded to the one finding with active in-cycle cost + (c) updates suite-side future-prevention surfaces only + (d) preserves legacy artifacts via hook-bypass + (e) names the future-revisit deferrals explicitly. This is the discipline shape, not the discipline gap. ✓

**Resolution:** The in-cycle suite-hardening at `e4b6701` is an acceptable operator-authorized in-cycle escalation, not a methodology-discipline gap. The landing's scope-creep guard fires correctly (1 of 3 findings fixed in-cycle; 2 deferred); the mechanical hook + primer amendment are future-prevention-only surfaces; the legacy preservation via hook-bypass is forward-only per G-89; future-revisit scope is named-not-buried. The pattern is canonical: a suite-development meta-finding surfaced during a project IAR cycle MAY land in-cycle if scope-bounded + operator-authorized + future-prevention-scoped.

**Classification:** Resolved — in-cycle suite-hardening landing is discipline-shape, not discipline-gap.

---

### Raised to SO

<a id="r2-f1"></a>
**Finding 1 — Round 2 launch pre-cycle methodology declaration (per [AIE R1 F6](2026-05-24-ai-engineer.md#r1-f6) routing closure) was operator-asserted in the launch prompt but NOT committed as a suite-side audit-trail entry per the [primer 3 § Pre-cycle methodology check](../../../../vsdd-suite/primers/3-review-session.md) discipline; the Round 2 cycle is in-flight without a verifiable pre-cycle declaration anchor (Dim 10 + Phase 4 routing fidelity for AIE F6)**

**Owner:** solution-owner
**Status:** raised
**Blocked by:** *(none — operator-authoring task; the declaration content was asserted in the Round 2 spawn prompts but needs persisting to a committed artifact)*
**Validator:** solution-owner

[AIE R1 F6](2026-05-24-ai-engineer.md#r1-f6) (Pre-cycle methodology declaration absent at Layer 3 IAR Round 1 cycle launch) was routed at Phase 4 routing § AI-Engineer process-discipline carry-forward (per-domain Phase 4 appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md`) to "Round 2 launch includes the pre-cycle methodology declaration per primer 3 § Pre-cycle methodology check Path 2" with gate "Round 2 declaration lands; Validator: AIE." The closure mechanism IS the Round 2 launch's pre-cycle declaration in a committed suite-side artifact.

Actual state at Round 2 launch:

- `ls vsdd-suite/suite-development/review-log/ | grep 2026-05-25` → empty (no new suite-side review-log entries dated 2026-05-25 exist as of this Round 2 in-flight audit).
- [`2026-05-24-suite-review.md`](../../../../vsdd-suite/suite-development/review-log/2026-05-24-suite-review.md) (the most recent suite-side review-log file) contains Reviews 92/93/94 + the Review 94 entry's discussion of letter-label 4th recurrence, but does NOT contain a new Review N entry naming the Layer 3 IAR Round 2 pre-cycle declaration fields (spawn shape; per-cycle budget; rate-limit headroom; model selection per task class; AI tool + plan tier + execution method; Phase-2a-evidence-shape).
- The Round 2 launch prompt itself (the prompt this reviewer received) asserts the launch-pre-cycle declaration as "Pre-cycle methodology declaration applied to Round 2 launch per AIE R1 F6 closure (this very Round 2 launch)" — operator-asserted in-prompt, but the assertion does not constitute a verifiable suite-side audit-trail entry.

Dim 10 framing per [primer 4](../../../../vsdd-suite/primers/4-feedback-integration.md) routing fidelity discipline: a Phase 4 routing decision whose fix-artifact does not match the gate is a routing-fidelity defect. AIE R1 F6's gate is "Round 2 declaration lands"; the in-prompt assertion is not "landed" per the suite-side review-log convention. The Round 2 launch IS happening; the declaration's audit-trail-anchor is NOT.

**Why Raised to SO:** the resolution requires operator-action — authoring a Round N entry in [`vsdd-suite/suite-development/review-log/2026-05-25-suite-review.md`](../../../../vsdd-suite/suite-development/review-log/) (or appending to the 2026-05-24 file) that names the Round 2 cycle's pre-cycle declaration fields per the primer 3 § Pre-cycle methodology check § Required pre-cycle declaration fields shape. The fields the operator should commit:

1. **Spawn shape:** N agents per round (per-domain vs cluster-batching); adversarial-pair-separation invariant statement.
2. **Per-cycle budget:** max-rounds; max-agents-per-round; per-cycle estimated token consumption against the intent-tier expected band.
3. **Rate-limit headroom:** operator-confirmable per the [`claude-code-cli.md` supplement](../../../../vsdd-suite/supplements/claude-code-cli.md) § Plan tiers + rate-limit windows.
4. **Model selection per task class:** Opus 4.7 for highest-complexity (Security / Red Team / SA / VDD-IAR Alignment / AI Engineer); Sonnet 4.6 for mid-complexity (SE / UX / PE / PFE / TW / DR / QE); Haiku 4.5 for mechanical sweeps if any.
5. **AI tool + plan tier + execution method:** per AIE Dim 14.
6. **Phase-2a-evidence-shape:** the Round 1 fix-work used the canonical two-commit shape at `ba6a4a9` → `bfc0713`; Round 2 has no new Phase 2a/2b work scheduled (it's a re-verify round), but the declaration field should be filled per the discipline.

**Discipline-honest forward path:** the gap is the audit-trail-anchor missing for a discipline whose payload is operator-asserted but not committed. The fix is the operator commits the pre-cycle declaration entry to a suite-side review-log file before Round 2 closes. Round 3 (or whichever round opens after Round 2's fix cycle) re-verifies the declaration entry exists + is well-formed per primer 3.

**Coordination:** [AIE R2](2026-05-24-ai-engineer.md) (this round, if AIE spawns) is the natural cross-domain raiser. Phase 4 routing should consolidate any cross-domain raises into a single Phase 4-itself fix (the discipline gap is at the methodology layer, not the project artifact layer); the fix lands as a suite-side review-log entry, not a project-side commit.

**Classification:** Raised to SO — pre-cycle methodology declaration anchor-not-committed defect; the declaration content was operator-asserted in the Round 2 spawn prompts but is not committed as a suite-side audit-trail entry. Operator-authoring routes via Phase 4 to Phase 4 itself per the routing table § "Process gap (Red Gate was skipped, layer merged without IAR, etc.) → Phase 4 itself."

---

<a id="r2-f2"></a>
**Finding 2 — Architectural correction sub-decision at `bfc0713` (`display_safe` removed from `export_json`; serde_json native encoder used instead) is operator-authorized per the commit message + Round 1 fix-work CHANGELOG.md entry, but the spec-vs-impl alignment at [DESIGN.md](../../DESIGN.md) § `bm export` (Layer 3) § Success-output is NOT updated to reflect the display_safe-removed-from-export_json shape; the Round 1 routing record's JSON-native escape design cluster gate ("DESIGN.md amendment lands with the spec-vs-impl alignment in writing") is partially-unmet (Dim 6 + Dim 7 cross-session spec consistency)**

**Owner:** solution-owner
**Status:** raised
**Blocked by:** *(none — operator-authoring task; SO is the sole domain authorized to modify DESIGN.md per the domain prompt § DESIGN.md change authority)*
**Validator:** solution-owner

[`bfc0713`](../../src/lib.rs) commit message names the architectural correction sub-decision explicitly:

> "Architectural correction sub-decision (Round 1 Phase 4 routing scope extension): Phase 2b implementation discovered the Round 1 Path-C decision had an incorrect technical premise (display_safe pre-escaping double-escapes through serde_json). Operator authorized the architectural correction (2026-05-25 main-session AskUserQuestion pass): remove display_safe from export_json entirely; leverage serde_json's native control-char escaping. The byte-preservation intent of Round 1 routing is preserved; only the implementation path changed."

The Round 1 Phase 4 routing record (per-domain Phase 4 appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md`) § JSON-native escape design cluster originally specified a multi-phase chain `Phase 2a → Phase 2b → Phase 1a+1b` with the Phase 1a+1b sub-step as:

> "Phase 1a+1b (spec amendment): DESIGN.md § `bm export` (Layer 3) update the byte-preservation paragraph to name the JSON-native escape design explicitly. Also update DESIGN.md § Edge case catalog § Layer 3 entry on display_safe."

[`fdfa989`](../../DESIGN.md) (Phase 1a+1b commit) implemented the original Round 1 Path-C decision — DESIGN.md § `bm export` (Layer 3) § Success-output was rewritten to name "the JSON-native `\uHHHH` 6-char escape vs `\u{HHHH}` 8-byte Rust-syntax literal" per the commit message. But the architectural correction at `bfc0713` invalidated that spec premise — the impl now does NOT apply display_safe at the export serialization step; it lets serde_json's native encoder handle Cc-range chars.

Actual state check at [`src/lib.rs:432-473`](../../src/lib.rs):

> "/// - **`display_safe` at the serialization boundary:** URL strings + tag-label strings route through `display_safe` BEFORE serialization ..."

The doc-comment + DESIGN.md § `bm export` (Layer 3) still describes the "display_safe applied at serialization boundary" shape; the impl does NOT do this. This is a spec-vs-impl drift surface: a future implementer reading DESIGN.md § `bm export` (Layer 3) would build the wrong shape (apply display_safe at serialization step → double-escape failure). The cross-session spec consistency test ([Dim 7](../../../../vsdd-suite/domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md)) fails: "can the current DESIGN.md, read cold, produce the current implementation?" — currently no.

Dim 6 framing: the architectural correction was operator-authorized per the commit message + the CHANGELOG entry, BUT the gate for the JSON-native escape design routing cluster ("DESIGN.md amendment lands with the spec-vs-impl alignment in writing") is partially-unmet. The Round 1 routing's Phase 1a+1b sub-step landed before the architectural correction was made; the post-architectural-correction spec re-amendment was not added.

**Why Raised to SO:** the resolution requires SO-authority operator-action — amending [DESIGN.md](../../DESIGN.md) § `bm export` (Layer 3) § Success-output paragraph to reflect the post-architectural-correction shape: serde_json's native encoder handles Cc-range chars at the export serialization step; `display_safe` stays at the render boundary (`bm list` eprintln/println) but is NOT applied inside `export_json`. The DESIGN.md edge-case-catalog entry on display_safe should be updated parallel. The [`src/lib.rs:432-473`](../../src/lib.rs) doc-comment should also be corrected for the doc-comment-matches-impl floor.

**Discipline-honest forward path:**

- Amend [DESIGN.md](../../DESIGN.md) § `bm export` (Layer 3) § Success-output to name the post-architectural-correction shape explicitly: "the serialization step uses serde_json's native encoder which emits JSON-native `\uHHHH` escapes for Cc-range characters per RFC 8259 § 7 (NOT the pre-Round-1 display_safe wrap); `display_safe` continues to apply at the render boundary (`bm list` eprintln/println paths)."
- Update [DESIGN.md](../../DESIGN.md) § Edge case catalog § Layer 3 entry on display_safe parallel.
- Correct [`src/lib.rs:432-473`](../../src/lib.rs) `export_json` doc-comment to match (the doc-comment currently misclaims display_safe is applied at serialization).
- The 6 Phase 2a test `tests_export_applies_display_safe_to_pathological_tag` ([line 1880](../../tests/bookmarks.rs)) already encodes the post-correction invariant as test (assert raw byte NOT in JSON output + byte present after parse) — the spec re-amendment makes the test's behavior contract explicit in the canonical spec, closing the spec-vs-test-vs-impl alignment cycle.
- Route via Phase 4 to Phase 1a+1b (spec amendment) per primer 4 routing table; the fix lands in the Round 2 fix-cycle commits before Round 3 cold re-verify.

**Phase 4 routing implication:** the JSON-native escape design cluster's gate ("DESIGN.md amendment lands with the spec-vs-impl alignment in writing") is partially-unmet; the architectural correction sub-decision IS the Phase 4-routing-scope-extension the `bfc0713` commit message names, but the spec re-amendment closing-step did not land. Round 2 routing should add a Phase 1a+1b fix to close the architectural-correction-sub-decision's spec gate.

**Classification:** Raised to SO — DESIGN.md § `bm export` (Layer 3) § Success-output spec-vs-impl drift surface introduced by the architectural correction sub-decision at `bfc0713`; spec re-amendment requires SO-authority. Routes via Phase 4 to Phase 1a+1b.

---

### Dismissed

<a id="r2-f7"></a>
**Finding 7 — The 3 DESIGN.md edge-case-catalog spec amendments at `fdfa989` are AC-shaped per Dim 2 ("Spec contracts have measurable behaviors"); spot-checks against the post-amendment DESIGN.md confirm each amendment encodes a measurable contract (Dim 2 — defer-to-spot-check dismissal)**

**Owner:** vdd-iar-alignment
**Blocked by:** *(none — defer-to-spot-check dismissal)*

Adversarial check: are the Round 1 Phase 1a+1b spec amendments at `fdfa989` AC-shaped (each amendment encodes a measurable behavior, not a vague intent)?

Spot-check against [DESIGN.md](../../DESIGN.md) post-`fdfa989` state on three amendments:

1. **Sorted-tag-comparison dedup** (§ Why dedup-on-sorted-tag-comparison): encodes "(url, timestamp, sorted(tags))" comparison shape — measurable (a test can construct two records and assert dedup behavior on tag-reorder).
2. **Control-char tag rejection** (§ `bm import` (Layer 3) § Failure (imported record contains control-char tag)): encodes "stderr `Error: imported bookmark tags contain disallowed control characters.` + exit 1 + no file write" — three-pronged measurable contract.
3. **Tag-injection threat-model addition** (§ Threat model): encodes "active mitigation on tags vs accepted-risk on URLs" with the named rationale ("write-access assumption broken at Layer 3") — measurable as a spec-vs-impl mitigation-effectiveness check.

The 3 spot-checks confirm AC-shape. Full audit deferred to the post-Round-2-fix-work Round 3 — the cross-session spec consistency check at full scope requires the [Finding 2](#r2-f2) spec re-amendment to land first (otherwise the architectural correction sub-decision's spec drift conflates the audit signal).

**Discipline-honest framing:** the Round 1 Phase 1a+1b spec amendments are AC-shaped at the spot-check level; the full Dim 2 audit is deferred to Round 3 timing per the [Finding 2](#r2-f2) sequencing dependency. This is a defer-to-spot-check Dismissal, not a defer-to-future-evidence Dismissal — the evidence exists; the full-scope audit is sequencing-blocked.

**Classification:** Dismissed — defer-to-spot-check; the Round 1 Phase 1a+1b spec amendments are AC-shaped at the spot-check level; full Dim 2 audit at Round 3 timing post-Finding 2 fix.

---

### Hallucinated

*(none)*

---

### Summary

**Round 2 closes at:** 2 Open ([Finding 1](#r2-f1) — pre-cycle declaration anchor-not-committed; [Finding 2](#r2-f2) — architectural correction sub-decision spec-vs-impl drift at DESIGN.md § `bm export` (Layer 3)) + 4 Resolved (Round 1 fix-work phase-progression + canonical Phase 2a/2b shape preserved; Phase 2a Round 1 fix tests behavior-assertive per Dim 5; Phase 4 routing + adversarial-pair separation + Phase 2c follow-up annotation + Phase 5/6 strategy operative; in-cycle suite-hardening is discipline-shape) + 1 Dismissed (Dim 2 spec amendment full audit deferred to Round 3 per [Finding 2](#r2-f2) sequencing dependency) + 0 Hallucinated.

**MVR signal:** **NOT REACHED at Round 2.** Per [G-131](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-131) continue-trigger discipline: 2 substantive Open findings require fix-work + Round 3 cold re-verify. [Finding 1](#r2-f1) is the AIE R1 F6 routing-fidelity closure (suite-side review-log entry authoring); [Finding 2](#r2-f2) is the architectural-correction-sub-decision spec re-amendment (DESIGN.md update + src/lib.rs doc-comment correction).

**Round 1 regression-check verdict:** PASSES with one carry-over. The Round 1 Resolved findings ([R1 F1](#r1-f1) phase-progression + [R1 F2](#r1-f2) Phase 2c annotation honesty + [R1 F3](#r1-f3) G-162 strict-form Phase 5/6 + [R1 F5](#r1-f5) Phase 5 surface deferral) all hold at the post-Round-1-fix-work state per [Finding 3](#r2-f3) + [Finding 5](#r2-f5) re-verification. The Round 1 Raised-to-SO [R1 F4](#r1-f4) (manual-tests/layer-3.md absence) is closed at `795bc25`. The Round 1 Dismissals at [R1 F5b](#r1-f5b) + [R1 F6](#r1-f6) are now closed by direct evidence at Round 2 [Finding 5](#r2-f5). One carry-over surface: the architectural correction sub-decision at `bfc0713` introduced a spec-vs-impl drift NOT present at Round 1 timing; this is a new Round-2 finding, not a Round-1 regression.

**Phase 4 routing-readiness signal:** Round 2's 2 Open findings have explicit routing destinations:

- [Finding 1](#r2-f1) → **Phase 4 itself** (process gap; the fix is a suite-side review-log entry authoring per primer 3 § Pre-cycle methodology check). Operator-authoring; coordinates with AIE Round 2 if AIE spawns.
- [Finding 2](#r2-f2) → **Phase 1a+1b** (spec amendment; the architectural correction sub-decision's spec re-amendment closing the JSON-native escape design cluster gate per the Phase 4 routing record). Operator-authoring (SO-authority required); the [`src/lib.rs:432-473`](../../src/lib.rs) doc-comment correction is the Phase 2b sub-step.

**Cross-domain coordination expected:** [Finding 2](#r2-f2) likely raised by parallel Round 2 SA + SE + Sec + RT (the original 4-domain convergence on the JSON-native escape design cluster); Phase 4 routing should consolidate cross-domain raises into a single Phase 1a+1b fix.

**Round 2 routing-readiness signal:** OPEN — Round 2 has 2 Open findings; the main session has everything needed to run Phase 4 routing per [primer 4](../../../../vsdd-suite/primers/4-feedback-integration.md) immediately after Round 2's aggregate finding-set closes. Round 3 cold re-verify is mandatory post-fix-work per G-131.

---

#### Cost-tally (per AIE F7 carry-forward — agent-self-verifiable tier only)

Per [primer 3 § Cost-tally report shape § Per-field auditability tier](../../../../vsdd-suite/primers/3-review-session.md), agent-self-verifiable fields populated below; operator-verifiable + operator-confirmable fields are `*pending operator /cost paste*` placeholders.

| Field | Value | Tier |
|---|---|---|
| 1. AI tool | `claude-code CLI` (sub-agent dispatch from main session per operator's Round 2 launch) | Agent-self-verifiable |
| 2. Plan tier | *pending operator confirmation* | Operator-confirmable |
| 3. Execution method | Sub-agent cold-session spawn from main-session orchestrator (1 of 13 Round 2 per-domain agent spawns expected) | Agent-self-verifiable |
| 4. Model | `claude-opus-4-7` (per primer 3 § Tuning levers § Model-tier right-sizing — VDD-IAR Alignment is a meta-domain warranting Opus tier) | Agent-self-verifiable |
| 5. Raw tokens | *pending operator `/cost` paste* | Operator-verifiable |
| 6. Would-be API cost | *pending operator `/cost` paste* | Operator-verifiable |
| 7. Actual cost to operator | *pending operator confirmation* (likely `$0 marginal within Max plan` if Max-tier; substitute operator's actual plan) | Operator-confirmable |
| 8. Rate-limit-window utilization | *pending operator `/cost` paste* | Operator-verifiable |
| 9. Wall-clock duration | ~15-20 min cold-session work (Read tool calls + Bash greps + Edit append) per the in-session tool-call log | Agent-self-verifiable |
| 10. Findings/100k tokens | NOT COMPUTABLE — pending operator `/cost` paste | Derived (requires operator-verifiable raw tokens) |

**Tool-call inventory** (agent-self-verifiable; from this session's tool-call log):

- Read tool calls: 5 (VDD-IAR-ALIGNMENT-REVIEW.md prompt; Review 1 + phase-4-routing.md log; primer 3; primer 4 head; tests/bookmarks.rs slice)
- Bash invocations: ~12 (git log + git show --stat ×4; ls/find for directory layout; grep over DESIGN.md / TODO.md / src/lib.rs / per-domain logs)
- Edit tool calls: 1 (append Review 2 to this file)

**Operator-action queue:** if cost-tally precision is load-bearing for cross-cycle calibration (per primer 3 § Cost-tally report shape § Per-field auditability tier), operator runs `/cost` in this session and pastes the output here as an append-only addendum, replacing the *pending operator …* placeholders with measured values.

**Source:** `domain-raised`
**Round:** 2
**Validator pair:** Per [VDD-IAR Alignment domain prompt](../../../../vsdd-suite/domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md) § Validator pair — `solution-owner` is the natural validator for VDD-IAR Alignment findings. [Finding 1](#r2-f1) + [Finding 2](#r2-f2)'s validators at fix-time are `solution-owner` (operator-authoring of the pre-cycle declaration suite-side review-log entry + DESIGN.md spec re-amendment). The 4 Resolved findings + 1 Dismissed carry `sanity-check` per the meta-validator-of-last-resort default.

**Coordination:** [Finding 1](#r2-f1) coordinates with AIE Round 2 (cross-domain raiser of the same gap from the AI-Engineer lens); Phase 4 routing should consolidate. [Finding 2](#r2-f2) coordinates with SA + SE + Sec + RT Round 2 (the original 4-domain JSON-native escape design convergence) — these domains' Round 2 entries are likely raisers of the same spec-vs-impl drift surface; Phase 4 routing should consolidate the cross-domain raises into a single Phase 1a+1b fix at SO authority.

---

## Phase 4 routing — Round 1 (2026-05-25 02:00Z)

Per [`vsdd-suite/primers/4-feedback-integration.md`](../../../../vsdd-suite/primers/4-feedback-integration.md) § [manual] First-class fallback path. SO-decisions captured via main-session AskUserQuestion pass on 2026-05-25 across the cross-domain finding clusters. This appendix lists this domain's routable findings in the primer-4-canonical per-finding shape; cross-domain coordination signals live in each Round 1 finding's `**Coordination:**` line. Cross-cluster sequencing matrix lives in the commit message + the CHANGELOG slim-form entry that recorded this Phase 4 pass (refactored from a prior consolidated routing record per operator directive 2026-05-25 — the consolidated file was an anti-pattern; primer-4-canonical is per-domain appendices).

#### Finding `r1-f4` — manual-tests/layer-3.md promised at TODO.md:138 does NOT exist (pointer-without-target Dim 9) — ROUTED

**Cluster:** manual-tests/layer-3.md authoring
**Route:** `Phase 2a-equivalent artifact authoring`
**Gate:** (see DR R1 F3 + SO R1 F1 routings — same cluster)
**Sequencing:** Blocks Layer 3 layer-gate close (criterion 3)
