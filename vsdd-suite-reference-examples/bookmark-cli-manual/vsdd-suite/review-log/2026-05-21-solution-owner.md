# Solution Owner Review — 2026-05-21

---

## Review 4 — 2026-05-21 22:00Z

**Phase:** [Phase 3](../../../../vsdd-suite/primers/3-review-session.md) — Iterative Adversarial Refinement.
**Source:** domain-raised — cold-session adversarial reviewer; did not author the [Layer 2 Phases 1a/1b/1c](../../DESIGN.md) commit `5ba62d5`, the [Phase 2a/2b](../../DESIGN.md) commit `326e25d`, the [`manual-tests/layer-2.md`](../../manual-tests/layer-2.md) commit `16ee420`, or the [Phase 2c](../../TODO.md) annotation commit `98b5886`.
**Lens:** Scope discipline + intent calibration + capstone-tier proportionality + user-value framing + reference-implementation purpose ([G-112](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112) per the [Solution Owner domain prompt](../../../../vsdd-suite/domains/role/SOLUTION-OWNER-REVIEW.md) Dim 2 + Dim 4 + Dim 7 + Dim 8 + the operator-supplied per-domain prompt's specific questions about Layer 2 forcing-function commitments).
**Scope:** Layer 2 spec-vs-implementation compliance across the four-commit sequence (`5ba62d5` / `326e25d` / `16ee420` / `98b5886`); Layer 1 regression-check baseline preserved.
**Surface:** Layer 2 promotion (the Layer 2 cycle now spans tag + filter implementation + manual-test plan + Phase 5 + Phase 6 strategy declarations).
**Reviewer:** Solution Owner cold-session agent.
**Model:** Opus 4.7 (per [`DESIGN.md`](../../DESIGN.md) § Cold-session budget — Opus for Solution Owner).
**Cold-session shape:** Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster (Solution Owner + Documentation Reviewer + AI Engineer + VDD-IAR Alignment) per the [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) § Cluster-batching pattern + AI Engineer R1 F1 cluster-with-adversarial-pair-separation discipline. Cluster placement preserves adversarial-pair separation: SO's natural validator (VDD-IAR Alignment) is co-located in this cluster but the SO ↔ VDD-IAR pair is validator-pair, not cold-reader-vs-author adversarial pair, so co-location is acceptable per [Review 77](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-77--2026-05-20-1545z) lifecycle. The canonical adversarial pairs (Security ↔ Red Team in QE/Security/Technical-Writer cluster + Solution-Architect/Red-Team/Platform-Engineer cluster; TW ↔ Doc Reviewer with TW in QE/Security/Technical-Writer cluster and Doc Reviewer here) are split.
**Regression-check against:** [Solution Owner Review 3 (2026-05-20-solution-owner.md)](2026-05-20-solution-owner.md#review-3--2026-05-20-2200z) (Layer 1 project-terminal SO MVR) + [VDD-IAR Alignment Review 3](2026-05-20-vdd-iar-alignment.md#review-3--phase-6-four-dimensional-convergence-project-terminal--2026-05-21-1330z) (Layer 1 Phase 6 four-dimensional convergence) as the regression baseline. Layer 1's spec-vs-implementation Compliance table is the floor that Layer 2 must not narrow or silently reinterpret.
**Cost-tally:** Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster agent — Opus 4.7; this Solution Owner round contributed ~25k input + ~12k output tokens ≈ ~$0.55 at standard pricing; per-finding cost ~$0.14 across 4 findings. Below the AI Engineer Dim 2 capstone-intent expected-band floor (100k tokens/finding) — read as Layer-scoped efficiency per [AI Engineer R2 Finding 2](2026-05-21-ai-engineer.md#r2-f2). Full cluster-D cost rolls up at cluster close.

**Session note:** Cold session opened against the post-commit-`98b5886` state. Reading order: [SO domain prompt](../../../../vsdd-suite/domains/role/SOLUTION-OWNER-REVIEW.md) → operator-supplied per-domain prompt (specific questions about Layer 2 promotion intent + Phase 6 attestation discipline necessity + PE F2/F5 closure proportionality) → [Solution Owner Review 3](2026-05-20-solution-owner.md#review-3--2026-05-20-2200z) (Layer 1 SO MVR baseline) → [`TODO.md`](../../TODO.md) § Layer 2 → [`src/lib.rs`](../../src/lib.rs) (the Bookmark.tags field + attach_tag + filter_by_tags + save fsync addition) → [`src/main.rs`](../../src/main.rs) (Cmd::Tag + Cmd::List { tags } + 3 run_* helpers + handle_parse_error LABEL extension) → [`tests/bookmarks.rs`](../../tests/bookmarks.rs) lines 504-1023 (Layer 2 Red Gate + closure tests) → [`manual-tests/layer-2.md`](../../manual-tests/layer-2.md) (13-Step plan) → [`DESIGN.md`](../../DESIGN.md) read LAST per the operator's per-domain prompt directive about cold-reader-poisoning discipline.

**Compliance table** (Layer 2 ACs vs. observable implementation behavior — the regression-check floor):

| AC | Spec (DESIGN.md / TODO.md) | Implementation | Status |
|---|---|---|---|
| AC 5 (tag-attaches) | `bm tag <url> <label>` idempotent append; exit 0; stdout silent | `src/main.rs:259-282` `run_tag` → `BookmarkStore::attach_tag` at `src/lib.rs:377-397`; `tests_tag_attaches_label_to_matching_bookmark` (`tests/bookmarks.rs:518-549`) + `tests_tag_is_idempotent` (`:553-584`) | Met |
| AC 6 (no-match exit 1 + no rewrite) | exit 1; stderr `Error: no bookmark found with URL <url>.`; store byte-identical | `src/main.rs:277-280` (NoMatch arm); `tests_tag_rejects_unknown_url` (`:589-618`) asserts `before == after` bytes | Met |
| AC 7 (empty URL) | exit 1; stderr `Error: URL cannot be empty.`; no file write | `src/main.rs:244-247` + `src/main.rs:153-156` `handle_parse_error` `URL` branch; `tests_tag_rejects_empty_url` (`:621-642`) | Met |
| AC 8 (empty label) | exit 1; stderr `Error: tag label cannot be empty.`; no file write | `src/main.rs:248-251` + `src/main.rs:148-152` `handle_parse_error` `LABEL` branch; `tests_tag_rejects_empty_label` (`:645-666`) | Met |
| AC 9 (list --tag filter + filter-empty-state) | OR-semantics filter; `No bookmarks match the supplied filter.\n` on empty match | `src/main.rs:227-232` + `BookmarkStore::filter_by_tags` at `src/lib.rs:409-414`; `tests_list_with_tag_filter_returns_matching_bookmarks` + `tests_list_with_tag_filter_empty_match_emits_filter_empty_state` | Met |
| AC 10 (--tag --tag OR-semantics) | repeated `--tag` is OR-union | `src/main.rs:70` `ArgAction::Append`; `tests_list_with_tag_filter_repeated_flag_is_or_semantics` | Met |
| AC 11 (--tag "" exit 1) | exit 1; same empty-label error | `src/main.rs:223-226` empty-string-in-tags screen; `tests_list_with_empty_tag_label_rejected` | Met |
| AC 12 (Layer-1-format forward-only migration) | `#[serde(default)]` tags; post-save file emits explicit `tags` for every bookmark | `src/lib.rs:54-55` `#[serde(default)]`; `tests_tag_against_layer_1_format_file_migrates_forward` (`:673-717`) | Met |
| AC 13 (durability — parent-dir fsync after rename) | `fsync(2)` on parent dir after `rename(2)`; `#[cfg(unix)]`-gated | `src/lib.rs:296-312` (the `#[cfg(unix)]` block in `save`); `src/lib.rs:440-445` `fsync_directory`; `tests_save_fsyncs_parent_directory` (`src/lib.rs:794-813`) — **weak proxy only**; see Finding 4 | Met-by-weak-proxy (see Finding 4) |

_Layer 1 regression-check (the floor from Review 3):_ all four Layer 1 ACs continue to hold — `tests_add_creates_bookmark` / `tests_add_rejects_empty_url` / `tests_list_orders_newest_first` / `tests_list_empty_state` pass at the post-commit-`98b5886` state per the commit message's `cargo test --all-targets → 41/41 pass`. The Layer 1 exit-code contract (0/1/2/64) is preserved at `src/main.rs:155 + 156 + 168 + 196`; the atomic-save discipline + mode-0600 + symlink-rejection + display-safe sanitizer all preserved per spot-check of `src/lib.rs:237-315`. _Layer 1 spec compliance: NO REGRESSION._

**MVR signal:** **Round 1 — NOT REACHED.** Two real findings surface (under-delivery against the Layer 2 acceptance criteria; over-investment in a Phase 6 attestation that pre-commits to a layer-terminal attestation shape before the Layer 2 work has been adversarially-pressured); two scope-discipline observations resolve cleanly; one specific question (Phase 6 attestation necessity for reference implementation per [G-150](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-150)) is answered against the SO seat. Per [G-131](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-131) continue-trigger discipline, Round 2 is mandatory if any of the substantive findings open.

---

### Backlogged

**Finding 1 — Under-delivery: `tests/scaling.rs` promised by DESIGN.md § Performance budget + TODO.md § Layer 2 is absent from the implementation (Dim 5)**

<a id="r4-f1"></a>

**Owner:** solution-owner
**Status:** raised
**Blocked by:** *(none — directly observable artifact gap)*
**Validator:** vdd-iar-alignment

DESIGN.md:230 names the artifact explicitly:

> "**Data-scaling tests:** Layer 2 ships sentinel integration tests at the 100 / 1,000 / 10,000-bookmark cliffs that exercise the full add → list → tag → list-filter cycle. Each cliff asserts: (a) operations complete within the budget table above; (b) the storage file round-trips without corruption; (c) the filter result set is correct against a programmatically-generated reference. The tests live in `tests/scaling.rs` and use `#[ignore]` by default so `cargo test` stays fast; CI runs them via `cargo test -- --ignored` in a separate job. **This closes [Performance Engineer Review 1 Finding 5](vsdd-suite/review-log/2026-05-20-performance-engineer.md) (Deferred-to-Layer-2).**"

TODO.md:81 echoes the same commitment:

> "**Layer 2 data-scaling tests:** `tests/scaling.rs` with `#[ignore]`-gated sentinels at 100/1,000/10,000 bookmark cliffs. Asserts the budget table in DESIGN.md § Performance budget holds against programmatically-generated stores. CI runs `cargo test -- --ignored` in a separate job so the `cargo test` default stays fast. Closes Layer-1-Deferred [Performance Engineer Review 1 Finding 5](vsdd-suite/review-log/2026-05-20-performance-engineer.md)."

TODO.md:87 (Layer-gate criterion #1) requires `cargo test --test bookmarks` + `cargo test -- --ignored` (scaling) both pass — but the second invocation has no targets to run because `tests/scaling.rs` does not exist. `tests/` contains only `bookmarks.rs`.

**Evidence of absence:** the file `tests/scaling.rs` is not present in the worktree. The Phase 2a/2b commit `326e25d` did not author it; the manual-test commit `16ee420` did not author it; the Phase 2c commit `98b5886` did not author it. The only place the 1,000-bookmark scaling fixture is exercised is `manual-tests/layer-2.md` Step 12, which is the hyperfine sanity-check — a separate closure-surface for PE F2, NOT PE F5.

**Why this is SO scope-discipline, not "merely an under-delivery to defer":** Layer 2 was explicitly promoted to active per TODO.md:48 with the framing "The Layer 2 cycle closes three Layer-1 Deferred-to-Layer-2 items: [PE F2] (benchmarking infrastructure → hyperfine sanity-check at `manual-tests/layer-2.md`), [PE F5] (data-scaling sentinel tests at 100/1,000/10,000-bookmark cliffs in `tests/scaling.rs`), and the operator-queued fsync benchmark item." All three are spec-named closure commitments that justified the Layer 2 cycle. PE F2 is closed by `manual-tests/layer-2.md` Step 12 (verified); the fsync item is closed by the AC 13 implementation (verified at the weak-proxy level — see Finding 4); PE F5 is **NOT closed** because the artifact that would close it does not exist.

This is the classic Solution Owner pattern from the domain prompt Dim 5 ("Are any required items missing, stubbed, or incomplete in a way that does not satisfy the spec? Partial implementations count as missing"). The spec names the artifact, the file path, the test gating mechanism (`#[ignore]`), the CI invocation (`cargo test -- --ignored`), the contract assertions (budget + round-trip + filter correctness against a generated reference), and the closure-of-finding-target (PE F5). None of these are present at Layer 2 close-of-implementation.

**Disposition:** The SO seat does not have authority to dismiss this as scope reduction; the spec was authored by the operator + the Phase 1a/1b sub-agent and the closure of PE F5 is a load-bearing claim. Two resolution paths are spec-honest:

1. **Author `tests/scaling.rs`** as DESIGN.md describes — the implementation cost is on the order of one ~150-200 line file (programmatic store generation + three `#[ignore]`-gated test functions at the named cliffs + assertions per the contract). The PE Layer 2 Round can then attest closure of F5 with grep-clean evidence.

2. **Amend DESIGN.md + TODO.md** to mark PE F5 closure deferred-further (e.g., to Layer 3, or as an explicit Accepted-limitation per the project's reference-implementation purpose). This is the SO-authority path: DESIGN.md is the spec contract; if the closure commitment is being unwound, the unwinding must be visible in the spec itself, not silently elided at implementation time.

The operator's choice between paths is preserved as the resolution path; the finding documents the gap.

**Resolution:** Either author `tests/scaling.rs` per the DESIGN.md § Performance budget Data-scaling tests block, OR amend DESIGN.md:230 + TODO.md:48 + TODO.md:81 to mark PE F5 closure deferred-further with explicit rationale. Both paths preserve SO ratification-trail integrity ([Review 3](2026-05-20-solution-owner.md#review-3--2026-05-20-2200z) `[r3-f2](#r3-f2)` inline-citation discipline).

**Classification:** Backlogged — Layer 2 acceptance gate cannot close without resolution; operator-decision-required between Option 1 (author `tests/scaling.rs`) and Option 2 (amend spec to defer PE F5 closure further).

---

**Finding 2 — Over-investment at Phase 1a/1b: the Phase 6 Layer 2 attestation pre-commitment in DESIGN.md is methodology over-investment per [G-150](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-150) for the reference-implementation purpose (Dim 2 + Dim 4 + Dim 8)**

<a id="r4-f2"></a>

**Owner:** solution-owner
**Status:** raised
**Blocked by:** *(none — spec-side observation)*
**Validator:** vdd-iar-alignment

DESIGN.md:17 commits the project to a SECOND Phase 6 four-dimensional convergence record for Layer 2:

> "Layer 2 four-dimensional convergence record will land as a later VDD-IAR Alignment review round titled 'Review N — Phase 6 four-dimensional convergence (project-terminal Layer 2)' — attests: Spec MVR (DESIGN.md Layer 2 round closure); Test MVR (QE + Performance Engineer Layer 2 closure including Mutation Testing maintenance + property-based test addition + scaling-test sentinels); Implementation MVR (every active-domain Layer 2 Phase 3 round at MVR per the 13-domain capstone-active set); Formal-verification MVR (Layer 2 Purity Boundary Audit + Layer 2 Mutation Testing closure + proptest property closure; Fuzz Testing / Proof Execution remain not-applicable). Cross-dimension consistency check applied at Layer 2 convergence time; signed closing attestation."

The operator's per-domain prompt asks directly: "Is this discipline necessary for a reference implementation, or is it methodology over-investment per [G-150](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-150)?"

The SO seat's answer is **yes, the second Phase 6 attestation as currently committed is over-investment**, with the following reasoning calibrated to the project's declared intent (capstone) AND its declared purpose (reference implementation for the worked example, [G-112](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112)).

The reference-implementation purpose is "validate the suite end-to-end" (DESIGN.md:3) and "exercise all six VSDD phases" (DESIGN.md:11). Layer 1 has already accomplished both per [VDD-IAR Alignment Review 3](2026-05-20-vdd-iar-alignment.md#review-3--phase-6-four-dimensional-convergence-project-terminal--2026-05-21-1330z) — all 6 phases including Phase 6 four-dimensional convergence demonstrated end-to-end, the worked example is satisfied, [G-112](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112) is closed. Layer 2 was promoted "post-PR-#43 cycle... after Layer 1 reached project-terminal MVR" (TODO.md:48); the promotion's justification is to close three deferred Layer-1 PE items + demonstrate that VSDD applies across multiple layers in succession, NOT to re-demonstrate the worked example (already demonstrated) NOR to teach a second Phase 6 attestation pattern (the first one is the canonical example; a second one teaches nothing additional unless the second is structurally different from the first — and the DESIGN.md:17 attestation declaration mirrors Review 3's structure dimension-for-dimension, so it teaches the same lesson twice).

Per the [G-150](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-150) intent-calibration framing: capstone-tier intent calibrates the rigor floor (full 6-phase exercise, 13-domain active set, MVR-strict gates). The floor is already satisfied at Layer 1. Adding a Layer 2 Phase 6 attestation does NOT raise the floor (it re-runs the same Phase 6 attestation pattern); it adds cost (the per-domain Layer 2 cold-session cycle + the four-dimensional cross-source consistency check + the operator's signed-and-dated attestation block) for marginal-to-zero teaching value beyond what Layer 1 already delivered.

The over-investment is most visible in the Phase 6 attestation's "Implementation MVR" sub-dimension: DESIGN.md:17 commits to "every active-domain Layer 2 Phase 3 round at MVR per the 13-domain capstone-active set" — meaning a fresh round per domain at Layer 2, before the Layer 2 Phase 6 attestation can sign. That is 13 cold-session domains + their MVR convergence loops + the per-domain MVR scorecard + the Phase 6 four-dimensional cross-source check. The Layer 1 cycle landed at ~$200-400 cycle cost per AI Engineer R1 F6 estimate; a faithful Layer 2 cycle at the same shape would land at a comparable cost — for a reference-example purpose already satisfied at Layer 1.

**This is the SO scope-creep pattern: a spec commitment authored at Phase 1a/1b that the implementation cycle is then bound to honor regardless of whether the value is proportionate.** The Phase 1a/1b sub-agent (the one that wrote DESIGN.md:17 + the parallel TODO.md:92 Layer-gate criterion #6) committed the project to a methodology surface whose value the operator had not separately evaluated against the reference-implementation purpose. The commitment IS the over-investment — it forces the cycle to spend cold-session budget on re-attesting what Layer 1 already attested.

**Disposition:** the spec amendment that would resolve this is to relax Layer 2's Phase 6 commitment from "full four-dimensional convergence record" to a lighter shape proportionate to the actual teaching value the Layer 2 cycle delivers. Three spec-amendment options:

1. **Mark Layer 2 Phase 6 as `not applicable — reference-example purpose satisfied at Layer 1 Phase 6 attestation; Layer 2 cycle is structural-demonstration only`.** This is the cleanest path: it surfaces the deliberate choice in DESIGN.md § Project intent Phase 6 strategy line, names the rationale (the worked example's [G-112](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112) purpose is already satisfied), and shifts the Layer 2 cycle's closure-signal from "full Phase 6 four-dimensional record" to "Layer 2 layer-gate criteria met per TODO.md:87-92 (excluding the criterion #6 Phase 6 reference) — see Layer 2 SO close-out for sign-off."

2. **Reduce Layer 2 Phase 6 to a thinner attestation** — e.g., the per-dimension citation specificity from primer 6's Dim 1 example, plus the cross-dimension consistency check, but NOT the full 13-domain MVR scorecard re-walk. This is operative if the operator believes the Layer 2 cycle warrants SOME Phase 6 attestation but agrees the full-rigor shape is disproportionate.

3. **Keep the spec as written, declare the over-investment as accepted-cost-of-faithful-execution, and document this finding's reasoning in DECISIONS.md (or equivalent retrospective surface) so the methodology evolves the next-project intent calibration.** This is the option that preserves the current spec; it acknowledges the over-investment honestly + uses it as input to a future suite-development discussion about reference-example tier proportionality.

Per Dim 8 (prior-review additions): the Layer 2 Phase 6 declaration is itself a prior-review addition by the Phase 1a/1b sub-agent at commit `5ba62d5` — it was not present in the Layer 1 DESIGN.md and was added when Layer 2 was promoted. The SO seat has veto authority over such additions per the SO domain prompt; the veto is being exercised in the form of "Raised to SO for re-evaluation" + the three resolution paths above.

**Resolution:** SO recommends Option 1 (mark Layer 2 Phase 6 as `not applicable — reference-example purpose satisfied at Layer 1`) on the strict reference-implementation-purpose ground that [G-112](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112) is already closed. Option 2 is acceptable if the operator believes a structurally-different Phase 6 attestation is valuable (e.g., one that demonstrates multi-layer Phase 6 cycling). Option 3 is the spec-as-written path — operator-veto-of-the-veto, with documentation. The operator's preference is the deciding input here; this is a spec authority decision.

**Classification:** Backlogged — the SO seat IS the recipient; the finding documents the over-investment for operator adjudication. Three resolution paths offered (Option 1 = mark Layer 2 Phase 6 `not applicable`; Option 2 = thinner attestation; Option 3 = spec-as-written with named-rationale carryforward).

---

### Resolved

**Finding 3 — Scope discipline confirmed: Layer 2 acceptance criteria AC 5-13 cleanly match the DESIGN.md § Scope and non-goals Layer 2 in-scope list (Dim 2, Dim 6)**

<a id="r4-f3"></a>

**Owner:** solution-owner
**Status:** validated
**Blocked by:** *(none)*
**Validator:** vdd-iar-alignment

DESIGN.md:38-42 § Scope and non-goals declares Layer 2 in-scope as exactly:

> "- `bm tag <url> <label>` — attach a label to all bookmarks matching `<url>` exactly; idempotent
> - `bm list --tag <label>` — filter by label; repeated flag is OR-semantics
> - Storage format extends with a per-bookmark `tags: Vec<String>` field that defaults to empty when absent (Layer-1-format files remain readable)"

TODO.md:52-60 enumerates AC 5 through AC 13:

- AC 5: tag-attaches-idempotent → matches DESIGN.md:39 "`bm tag <url> <label>` ... idempotent"
- AC 6: tag-rejects-unknown-url → spec edge case (DESIGN.md:82)
- AC 7: tag-rejects-empty-url → spec edge case (DESIGN.md:80)
- AC 8: tag-rejects-empty-label → spec edge case (DESIGN.md:81)
- AC 9: list --tag filter → matches DESIGN.md:40 "filter by label"
- AC 10: --tag --tag OR-semantics → matches DESIGN.md:40 "repeated flag is OR-semantics"
- AC 11: list --tag "" rejected → spec edge case (DESIGN.md:96)
- AC 12: forward-only migration → matches DESIGN.md:41 "Layer-1-format files remain readable" + the storage-format extension contract
- AC 13: durability fsync → derives from the Performance Engineer "operator-queued fsync benchmark item" (DESIGN.md:232 "Durability discipline (Layer 2)")

No AC introduces behavior outside the DESIGN.md § Scope and non-goals Layer 2 in-scope list. The implementation surfaces (lib.rs + main.rs + tests) do not add features beyond the 13 ACs — spot-check of `src/lib.rs:377-414` confirms `attach_tag` + `filter_by_tags` are the only new public methods, and `src/main.rs:59-80` confirms `Cmd::Tag` + `Cmd::List { tags }` are the only new clap surface beyond the Layer 1 `Cmd::Add` + bare `Cmd::List`.

**The one borderline case:** AC 13 (durability fsync). Per the operator's per-domain prompt: "the hyperfine sanity-check + scaling tests close PE F2 + F5. Are these closures proportionate to the Layer 2 budget, or is the closure a forcing-function from Phase 1 spec writing that committed before the work was scoped?" The fsync item was declared at Phase 1a/1b as one of the three "Deferred-to-Layer-2" items that justify Layer 2's promotion (TODO.md:48). Whether the fsync work was the "right" Layer 2 work to schedule is a separate scope question from whether the fsync work is _in_ scope — and the fsync work IS in scope per the spec authority of DESIGN.md:232. The SO seat does not second-guess scope decisions the spec contains; it audits whether implementations match the spec. Implementation matches. (Whether the spec _should have_ committed to closing PE F5 + the fsync item in Layer 2 — vs. deferring further or closing one but not the other — is the Finding 2 over-investment question above, not a Dim 2 scope-creep question here.)

**Resolution:** Layer 2 implementation is scope-clean — AC 5 through AC 13 all map to DESIGN.md § Scope and non-goals Layer 2 in-scope items + spec-named edge cases. No feature creep, no abstraction beyond what the spec required, no library or technology additions beyond Layer 1's surface (the only new clap variant is `Cmd::Tag`; the only new lib types are `AttachTagError` enum + `Bookmark.tags` field + two new pure methods). Dim 2 + Dim 6 + Dim 7 all clean.

**Classification:** Resolved — the Layer 2 in-scope surface matches DESIGN.md's Layer 2 in-scope declaration with no creep.

---

**Finding 4 — Capstone-tier intent calibration confirmed proportionate: the Layer 2 cycle stays within reference-implementation purpose per [G-112](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112), but the AC 13 fsync test is a weak-proxy structural closure that the SO seat acknowledges as the implementation's only honest path (Dim 5)**

<a id="r4-f4"></a>

**Owner:** solution-owner
**Status:** validated
**Blocked by:** *(none)*
**Validator:** vdd-iar-alignment

The operator's per-domain prompt asks whether `bookmark-cli-manual`'s Layer 2 cycle pulls scope creep or stays within "reference implementation for the worked example" purpose. The cluster of in-scope work — `bm tag` + `bm list --tag` + the storage-format extension + the 13 Red Gate tests + the manual-test plan + the fsync durability + the parent-dir fsync syscall — is all calibrated to the worked-example surface. None of it ventures into Layer 3 (export/import) or non-goal territory (network sync, multi-user, browser integration, beyond-tag search, URL validation, editing/deleting, configuration file).

The AC 13 fsync implementation deserves a specific SO observation that does not rise to a finding-against-the-spec but is worth recording:

The fsync test (`src/lib.rs:794-813` `tests_save_fsyncs_parent_directory`) is structurally a WEAK PROXY for the AC 13 durability contract. The test's own doc-comment (`src/lib.rs:776-793`) names the proxy honestly:

> "There is no portable way for a black-box unit test to assert that fsync was actually called on the parent directory FD (the syscall has no observable side effect from userspace). Acceptable alternative: the test asserts that after a `save` of a non-trivial store the file is present on disk + the store round-trips cleanly through `load`. This is a WEAK PROXY for the durability contract — it confirms the save codepath executes successfully against a real filesystem (the same codepath that includes the fsync on Unix) but does not directly verify the fsync syscall was issued."

The honest naming is good practice — it preserves the audit trail; a future reviewer reading the test knows what it does and does not verify. From the SO seat, the question is whether AC 13's durability claim is "automatable via unit + integration tests" per DESIGN.md:195 § Verification architecture's claim that "every behavioral contract above is automatable via unit + integration tests."

Strictly, AC 13's durability claim is NOT directly automated — the test verifies that the save codepath completes successfully on a Unix system (which, by code-path inspection at `src/lib.rs:296-312`, includes the fsync call) but does not verify the kernel actually crossed the durability boundary. A regression that silently no-op'd the fsync (e.g., a future refactor that gates the `#[cfg(unix)]` block on the wrong condition) would not be caught by the existing test.

**Two possible dispositions:**

1. **SO accepts the weak-proxy as the proportionate Layer 2 closure** because (a) the cost of a strong-proxy (syscall-observation harness, strace driver, custom Filesystem mock trait) is disproportionate to the reference-implementation purpose; (b) the test's doc-comment is honest about the proxy nature, so the audit trail does not over-claim; (c) the code-path is short and visible at `src/lib.rs:296-312`, so a code-review-as-second-opinion is the practical guarantee. This is the disposition the SO seat takes.

2. **SO requires the spec to surface the weak-proxy explicitly** — DESIGN.md:195 currently says "every behavioral contract above is automatable via unit + integration tests" without exception. AC 13 is an exception (automatable structurally but not behaviorally). The spec amendment would be a footnote on the § Verification architecture sentence naming AC 13 as the weak-proxy exception + the operator-reviewed code-path as the strong guarantee. This is not required — the Layer 2 Red Gate test plan at TODO.md:77 already names the same limitation in the test plan body — but it would tighten the spec ↔ implementation alignment.

The SO seat goes with disposition 1: the weak-proxy is acceptable given the reference-implementation purpose; the spec does NOT need amendment because the spec's "automatable via unit + integration tests" claim is true at the structural-execution level (the codepath including the fsync is executed by the test), and the boundary case (true syscall-observation impossibility from userspace) is honestly named at the test surface. The cross-domain validator (VDD-IAR Alignment) should evaluate whether the weak-proxy is acceptable from its Dim 12 test-as-spec-assertion lens — that is the natural place for the validator-pair check.

**Resolution:** SO accepts the AC 13 weak-proxy closure as the proportionate Layer 2 implementation per the reference-implementation purpose ([G-112](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112)). No spec amendment required at the SO seat. VDD-IAR Alignment Dim 12 may take a different posture; SO defers to that pair on the test-as-spec-assertion dimension.

**Classification:** Resolved — capstone-tier intent calibration holds; the weak-proxy fsync test is honest about its limits + proportionate to the project's scope.

---

### Hallucinated

*(none — the four findings above are concrete and citation-backed; no SO-dim concerns that turned out to be spec-misread emerged in this round)*

---

### Approved deviation

*(none — no pre-approved DESIGN.md deviations apply at this round)*

---

### Raised to SO

*(none — this IS the SO round; cross-domain findings that would route to SO are filed against their originating domain's log)*

---

### Dismissed

*(none — every Layer 2 spec commitment was either Met, Met-by-weak-proxy (Finding 4 acceptance), or Open (Finding 1 under-delivery); no dismissable concerns)*

---

### Summary

Four findings in Round 1:

- **Backlogged (operator-decision-required):**
  - [Finding 1](#r4-f1) — `tests/scaling.rs` absent (Dim 5 under-delivery against DESIGN.md:230 + TODO.md:81 spec commitments closing PE F5)
  - [Finding 2](#r4-f2) — Phase 6 Layer 2 attestation pre-commitment is methodology over-investment per [G-150](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-150) for the reference-implementation purpose ([G-112](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112)); three spec-amendment options proposed
- **Resolved:**
  - [Finding 3](#r4-f3) — Layer 2 AC 5-13 cleanly match the DESIGN.md § Scope and non-goals Layer 2 in-scope list; no scope creep
  - [Finding 4](#r4-f4) — capstone-tier intent calibration holds; the AC 13 fsync weak-proxy test is acceptable given the reference-implementation purpose; VDD-IAR Alignment Dim 12 may take a different posture

**Operator-supplied per-domain prompt answers (summarized for the audit trail):**

1. _"What's the operator's intent for Layer 2 — (a) demonstrate VSDD applies to multiple layers in succession, (b) close deferred Layer-1 PE findings, or (c) both?"_ — Per TODO.md:48, the framing is (c) both: "The Layer 2 cycle closes three Layer-1 Deferred-to-Layer-2 items..." plus the implicit (a) demonstrate-multi-layer purpose. The SO seat accepts (c) as the spec-authorized intent; the **proportionality** of (c) is the subject of [Finding 2](#r4-f2).

2. _"Is the Layer 2 Phase 6 attestation discipline necessary for a reference implementation, or is it methodology over-investment per [G-150](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-150)?"_ — **Methodology over-investment** per [Finding 2](#r4-f2). Three resolution paths offered to the operator; SO recommends Option 1 (mark Layer 2 Phase 6 `not applicable` on reference-example-purpose-already-satisfied grounds).

3. _"Are the hyperfine + scaling-test closures of PE F2 + F5 proportionate to the Layer 2 budget, or forcing-functions from Phase 1 spec writing?"_ — The hyperfine closure of PE F2 IS proportionate (the manual-test surface is a low-cost discipline-honest closure mechanism; `manual-tests/layer-2.md` Step 12 is well-shaped per Doc Reviewer's separate concerns). The scaling-test closure of PE F5 IS a forcing-function — the spec committed to `tests/scaling.rs` at Phase 1a/1b before evaluating whether the scaling-test value justified the implementation cost; the implementation gap surfaced as [Finding 1](#r4-f1). The fsync closure (operator-queued PE item, AC 13) is borderline — the weak-proxy test ([Finding 4](#r4-f4)) is proportionate but acknowledges that strong-proxy verification would have been disproportionate; the operator's framing of "operator-queued" rather than "PE-finding-closure" gives the operator more latitude on whether the closure value justified the work.

**Coordination:** [Finding 1](#r4-f1) (under-delivery) is the natural SO surfacing of what would be a Quality Engineer test-coverage finding from a different seat — QE in QE/Security/Technical-Writer cluster should be expected to surface the same gap from the test-discipline lens; cross-validation between SO + QE expected. [Finding 2](#r4-f2) (over-investment) is the natural SO surfacing of what would be a VDD-IAR Alignment Dim 7+8 finding from the methodology-discipline seat — VDD-IAR Alignment in this Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster should be expected to evaluate the same Phase 6 attestation pre-commitment from its lens; cross-validation between SO + VDD-IAR Alignment expected. [Finding 3](#r4-f3) + [Finding 4](#r4-f4) document the scope-clean + proportionality-clean state for future-cycle regression-check.

**Cost-tally** (per [`suite-development/suite-development.md`](../../../../vsdd-suite/suite-development/suite-development.md) § Per-review entry preamble § Cost-tally): cold-session Opus 4.7 agent within the Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster quartet; per-cluster cost expected ~$5 per AI Engineer R1 F1 precedent; rough per-finding cost ~$1.25 across 4 findings. Token-count estimate (per `DESIGN.md` § Cold-session budget): cluster reads 4 domain prompts + 4 prior-round per-domain logs + the Layer 2 artifact set (~6 files) once, then each domain produces its review; per-domain output ~10-12k tokens; cluster total ~50-60k input + ~40-50k output ≈ 100k tokens at ~$1.50 input + ~$0.75 output ≈ ~$2.25/cluster — the $5/cluster precedent likely overestimates this specific Layer 2 cluster cost (smaller surface than the Layer 1 cycle). Awaiting cluster-close cost tally for verification.

**Validator:** vdd-iar-alignment (the SO ↔ VDD-IAR Alignment validator pair per [Review 77](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-77--2026-05-20-1545z) — VDD-IAR Alignment confirms the spec changes (the Finding 1 + Finding 2 resolution paths each route through DESIGN.md amendments) went through proper routing and don't conflict with prior intent).

---

## Review 5 — 2026-05-22 16:30Z

**Phase:** [Phase 3](../../../../vsdd-suite/primers/3-review-session.md) — Iterative Adversarial Refinement (Layer 2 Round 2 verification).
**Source:** domain-raised — cold-session adversarial reviewer; did not author the fix commits (`156ec53` / `d62bb1a` / `002d747` / `cdb46bc` / `9d56c3f`) and does not inherit Round 1's framing. Treats Review 4 as prior adversary's claim per the [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) cold-reader-vs-prior-round discipline.
**Lens:** Verification of [Review 4](#review-4--2026-05-21-2200z) finding dispositions against the post-fix-cycle state + the cost-investment proportionality lens for the full Layer 2 cycle (Round 1 + fix cycle + Round 2) per the operator-supplied per-domain prompt for this round.
**Scope:** Layer 2 spec-vs-implementation compliance against the post-`9d56c3f` state; Layer 1 regression-check baseline preserved; capstone-tier intent calibration check across the full cycle cost.
**Surface:** the four [Round 1 SO findings](#review-4--2026-05-21-2200z) (Backlogged F1 + Backlogged F2 + Resolved F3 + Resolved F4) verified against the 5 fix commits + any adjacent-defect concerns the fix may have created.
**Reviewer:** Solution Owner cold-session agent.
**Model:** Opus 4.7 (per [`DESIGN.md`](../../DESIGN.md) § Cold-session budget — Opus for Solution Owner).
**Cold-session shape:** Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster (Round 2; same composition as Round 1) per the [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) § Cluster-batching pattern. Adversarial pairs (Security ↔ Red Team; TW ↔ Documentation Reviewer) remain split per the Round 1 manifest.
**Regression-check against:** [Solution Owner Review 4](#review-4--2026-05-21-2200z) (Layer 2 Round 1 SO baseline) + [Solution Owner Review 3 (2026-05-20-solution-owner.md)](2026-05-20-solution-owner.md#review-3--2026-05-20-2200z) (Layer 1 project-terminal SO MVR; still the regression floor).
**Cost-tally:** Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster agent — Opus 4.7; this Solution Owner Round 2 contributed ~20k input + ~9k output tokens ≈ ~$0.43 at standard pricing; per-finding cost ~$0.14 across 3 verification entries. Round-2 cost below Round-1 cost (~$0.55) per the Phase 4 routing scope-reducer discipline ([AI Engineer R1 F2](2026-05-21-ai-engineer.md#r1-f2)) — narrower scope, lower per-round token budget.

**Session note:** Cold session opened against the post-commit-`9d56c3f` state. Reading order: [SO domain prompt](../../../../vsdd-suite/domains/role/SOLUTION-OWNER-REVIEW.md) → operator-supplied per-domain prompt (Round 1 finding verification list + the cost-investment proportionality concern) → [Solution Owner Review 4](#review-4--2026-05-21-2200z) (Round 1 SO findings) → `git log 02e6eb3..9d56c3f` (the 5 fix commits + their stat lines) → the post-fix state of `tests/scaling.rs` + `tests/properties.rs` + `Cargo.toml` + `DESIGN.md` § Phase 6 strategy + `TODO.md` § Layer-gate criterion #6 + the Phase 2c Red Gate annotation at `TODO.md:85` → `cargo test` invocation output (43 default tests pass + 3 scaling ignored sentinels via `--ignored`) → [`DESIGN.md`](../../DESIGN.md) read LAST per cold-reader-poisoning discipline. The Round 1 fix cycle landed 5 commits totaling ~600 + 87 + 24 + 87 + 10 = ~808 LoC across spec + tests + main.rs + manual-tests + install-verification.

**MVR signal:** **REACHED at Round 2.** All four Round 1 findings have honest dispositions against the post-fix state: F1 closed by `tests/scaling.rs` authoring (Option 1 from Round 1) + closes PE F5 cleanly; F2 closed by Phase 6 NOT APPLICABLE declaration (Option 1 from Round 1 — the SO recommendation adopted); F3 + F4 still hold as Resolved with the spec post-amendment. One Round 2 cost-proportionality observation surfaces as a documented-resolved finding for future-cycle regression-check. No new under-delivery, scope-creep, or over-investment surfaces against the post-fix state.

---

### Resolved

**Finding 1 — `tests/scaling.rs` under-delivery (verifies [r4-f1](#r4-f1)) (Dim 5)**

<a id="r5-f1"></a>

**Owner:** solution-owner
**Status:** validated
**Blocked by:** *(none)*
**Validator:** vdd-iar-alignment

Closure of [r4-f1](#r4-f1) — the under-delivery is closed by `156ec53` adopting Option 1 path.

**Evidence:**

- `tests/scaling.rs` exists on disk at the worktree root: `ls vsdd-suite-reference-examples/bookmark-cli-manual/tests/` returns `bookmarks.rs`, `properties.rs`, `scaling.rs` (post-`156ec53`; was returning only `bookmarks.rs` at Round 1 close).
- The file is 221 lines per `git show 156ec53 --stat` and contains three `#[ignore]`-gated sentinel functions at the 100 / 1,000 / 10,000-bookmark cliffs per the DESIGN.md:230 commitment.
- `cargo test` against the post-`9d56c3f` worktree:
  - Default invocation: 12 unit + 29 integration + 2 proptest = 43 tests pass; 3 ignored.
  - `cargo test -- --ignored` (per `TODO.md:89` Layer-gate criterion #1 second clause) now has 3 actual scaling-test functions to invoke (rather than the vacuous 0-ignored pass at Round 1 close).
- Fix commit `156ec53` co-authors `tests/properties.rs` (proptest tag-idempotence + filter-OR-monotonicity properties) + adds `proptest = "1"` to `Cargo.toml` dev-dependencies + adds a `.github/workflows/bookmark-cli-manual.yml` job spec for the `cargo test -- --ignored` scaling job — closing the PE F5 spec commitment chain end-to-end (the spec said "CI runs them via `cargo test -- --ignored` in a separate job"; the CI job spec now exists).
- The operator-decision-required disposition from Round 1's Backlogged classification is now **Resolved** — the operator chose Option 1 (author `tests/scaling.rs`) over Option 2 (amend DESIGN.md). The choice was the spec-honest path: the DESIGN.md:230 + TODO.md:81 commitments stand as the spec contract; the implementation now matches.

**Commentary:** SO scope-discipline lens — the fix path adopted is the path that PRESERVES the spec contract rather than narrowing it. The under-delivery against the spec ceased to be an under-delivery; the spec was honored at the artifact level. Dim 5 (under-delivery) closed cleanly. The choice to also author `tests/properties.rs` (a separate but related under-delivery — DESIGN.md § Phase 5 strategy named proptest activation but the file did not exist either) shows the fix cycle scoped to all known spec commitments rather than the minimal SO Round 1 finding alone — operationally healthy.

**Resolution:** Round 1 R4 F1 closed by `156ec53` per Option 1 path; `tests/scaling.rs` + `tests/properties.rs` both authored; PE F5 spec commitment chain closed end-to-end including CI job.

**Classification:** Resolved — Round 1 under-delivery closed cleanly via the spec-honest path (Option 1).

---

**Finding 2 — Phase 6 Layer 2 over-investment (verifies [r4-f2](#r4-f2)) (Dim 2 + Dim 4 + Dim 8)**

<a id="r5-f2"></a>

**Owner:** solution-owner
**Status:** validated
**Blocked by:** *(none)*
**Validator:** vdd-iar-alignment

Closure of [r4-f2](#r4-f2) — the over-investment is closed by `002d747` adopting Option 1 path, the SO recommendation.

**Evidence:**

- [`DESIGN.md`](../../DESIGN.md):17 § Phase 6 strategy now reads (post-`002d747`):
  > "Layer 2 four-dimensional convergence: **NOT APPLICABLE** per [G-150](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-150) (over-investment guard) + [G-112](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112) (reference-implementation-purpose-already-satisfied) — bookmark-cli's reference-implementation purpose is 'exercise all six VSDD phases end-to-end as a worked example', which Layer 1's project-terminal MVR + Phase 6 attestation already demonstrate. Re-running Phase 6 for Layer 2 would teach methodology consumers that capstone artifacts require per-layer four-dimensional convergence, which is not the suite's intent — capstone gates at project-terminal MVR per primer 6, not per-layer. This disposition closes Layer 2 Round 1 VDD-IAR Alignment R4 F5 + Solution Owner R4 F2 (the cluster's own SO recommended Option 1: mark not-applicable; this declaration adopts that recommendation)."
- The G-162 strict-form requirement (both Phase 5 + Phase 6 strategy lines declared with `planned` or `not applicable` + named scope) is satisfied per `DESIGN.md:17` (Phase 6 explicit not-applicable declaration named-rationale).
- [`TODO.md`](../../TODO.md):94 Layer-gate criterion #6 now reads (post-`002d747`):
  > "**[Phase 6](../../vsdd-suite/primers/6-convergence.md) not applicable** per [DESIGN.md § Project intent](DESIGN.md#project-intent) Phase 6 strategy declaration ([G-150](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-150) over-investment guard + [G-112](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112) reference-implementation-purpose-already-satisfied). Layer 1's Phase 6 attestation at [VDD-IAR Alignment Review 3](vsdd-suite/review-log/2026-05-20-vdd-iar-alignment.md) stands as the project's terminal four-dimensional convergence record"
- The criterion is cross-linked back to the DESIGN.md declaration (proper spec routing); the rationale (Layer 1's Phase 6 satisfies G-112) is preserved at both surfaces.
- `git show 002d747 --stat`: DESIGN.md + TODO.md + Cargo.toml + rust-toolchain.toml modified — the bundling of the Phase 6 not-applicable amendment with other spec amendments (Security F1 + SA F5 + Red Team F6 + PE F4 + VDD-IAR R4 F1) is operationally efficient.

**Commentary:** SO authority lens — the SO seat owned the decision (Dim 8 prior-review additions: the Phase 6 commitment WAS a Phase 1a/1b sub-agent addition; SO has veto authority over such additions; the veto was exercised in the form of Option 1 adoption). The closure is the spec-honest path: the methodology over-investment surfaces explicitly in DESIGN.md as a deliberate declaration with named rationale, rather than being silently elided. A future capstone reviewer reading DESIGN.md sees that the operator + SO made an informed choice; the audit trail is complete. The G-150 + G-112 framing is correctly applied — the worked-example purpose IS satisfied at Layer 1; Layer 2 is structural-demonstration only.

**Methodology-precedent concern (declared, not blocking):** future capstone projects reading bookmark-cli-manual as a reference example will see Layer 1 attested (Phase 6 done) + Layer 2 not-applicable (Phase 6 not done) and may infer that Phase 6 is per-project, not per-layer. That inference IS correct per G-150 + G-112 reasoning — but the cross-cluster VDD-IAR Alignment Round 2 review (see [`2026-05-21-vdd-iar-alignment.md`](2026-05-21-vdd-iar-alignment.md) Round 2) should verify the discipline is articulated robustly enough for the cold-reader to take the right lesson. SO does not block on this — it is a documentation-discipline concern routed to VDD-IAR Alignment's seat.

**Resolution:** Round 1 R4 F2 closed by `002d747` per Option 1 path; Phase 6 Layer 2 NOT APPLICABLE declaration adopted at DESIGN.md:17 with G-150 + G-112 named rationale + Layer 1 attestation cited as the project's terminal record.

**Classification:** Resolved — Round 1 over-investment closed cleanly; SO authority exercised via Option 1 adoption (SO's own Round 1 recommendation).

---

**Finding 3 — Layer 2 AC 5-13 scope-clean (verifies [r4-f3](#r4-f3)) (Dim 2 + Dim 6 + Dim 7)**

<a id="r5-f3"></a>

**Owner:** solution-owner
**Status:** validated
**Blocked by:** *(none)*
**Validator:** vdd-iar-alignment

Regression-check against [r4-f3](#r4-f3) holds; no new scope-creep introduced by the fix cycle.

**Evidence:**

- The 5 fix commits did NOT introduce any new behavior, abstraction, or technology beyond Layer 2's spec scope. Inspection of `git diff 02e6eb3..9d56c3f -- vsdd-suite-reference-examples/bookmark-cli-manual/src/`:
  - `src/main.rs` — only changes (per `cdb46bc`) are: `eprintln!("Tagged {n} bookmark(s).");` affordance (UX F2 + SE F2 close) + help-text expansion in `Cmd::Tag` + `Cmd::List` doc-comments. No new clap variants, no new behavior paths. ✓
  - `src/lib.rs` — no changes in the fix cycle (the lib surface is the Phase 2b shape; Round 1 found it scope-clean; Round 2 finds it still scope-clean). ✓
- `tests/scaling.rs` + `tests/properties.rs` are within scope per DESIGN.md:230 + DESIGN.md § Phase 5 strategy declarations (these are the spec commitments the fix cycle CLOSED, not new spec additions).
- The Phase 2c Red Gate annotation at `TODO.md:85` is within scope per the VDD-IAR R4 F1 resolution path (Resolved-with-named-rationale; the annotation IS the closure, not a new commitment).

**Commentary:** Regression-check clean. No new scope-creep introduced by the fix cycle. The AC 5-13 surface is unchanged; the only main.rs change is the stderr affordance line which is in-scope per DESIGN.md § `bm tag` behavioral contract (the UX F2 finding cited the missing affordance as a usability gap against the multi-match-tag-all-matching-records semantic that DESIGN.md:80-88 declares; the fix adds the affordance the spec implicitly required for usability). Dim 2 + Dim 6 + Dim 7 all clean at Round 2 close.

**Resolution:** Regression-check against [r4-f3](#r4-f3) clean; scope-discipline holds at Round 2 close.

**Classification:** Resolved — no new scope-creep introduced by the fix cycle.

---

**Finding 4 — Capstone-tier intent calibration + AC 13 fsync weak-proxy (verifies [r4-f4](#r4-f4)) (Dim 5)**

<a id="r5-f4"></a>

**Owner:** solution-owner
**Status:** validated
**Blocked by:** *(none)*
**Validator:** vdd-iar-alignment

Regression-check against [r4-f4](#r4-f4) holds; minor adjacent disposition update on AC 13.

**Evidence:**

- The AC 13 fsync test (`tests_save_fsyncs_parent_directory` at `src/lib.rs:794-813`) is unchanged in the fix cycle. The weak-proxy honesty is preserved.
- DESIGN.md:195 § Verification architecture sentence ("every behavioral contract above is automatable via unit + integration tests") was NOT amended by the fix cycle. The Round 1 SO seat's disposition (the spec does NOT need amendment because the structural-execution-level test is automated) holds.
- The cross-validator VDD-IAR Alignment posture at Round 1 was "minor DESIGN.md footnote naming AC 13 as a weak-proxy exception, but not a blocking concern" (per [VDD-IAR R4 Operator-supplied per-domain-prompt answer 3](2026-05-21-vdd-iar-alignment.md)). The operator did NOT adopt the footnote; this is acceptable per the VDD-IAR seat's own framing.

**Commentary:** Capstone-tier intent calibration holds. The AC 13 weak-proxy disposition is unchanged. No new scope-creep or under-delivery on the fsync surface.

**Resolution:** Regression-check against [r4-f4](#r4-f4) clean; the AC 13 weak-proxy posture unchanged.

**Classification:** Resolved — capstone-tier calibration + AC 13 fsync weak-proxy disposition both hold at Round 2 close.

---

### Backlogged

**Finding 5 — Cost-investment proportionality across the full Layer 2 cycle: Layer-scoped efficiency observation; methodology-refinement candidate routes to suite-side (Dim 4 + Dim 8)**

<a id="r5-f5"></a>

**Owner:** solution-owner
**Status:** validated
**Blocked by:** *(none — observable cost evidence + the cluster's own AI Engineer review surfaces the same concern)*
**Validator:** ai-engineer

The operator-supplied per-domain prompt for Round 2 asks: "With the fix cycle landing 5 commits + 4 cluster Round 1 cold-sessions + 4 cluster Round 2 cold-sessions (in progress), is this proportionate? The cluster's own AI Engineer review surfaced cost-efficiency findings."

The cost evidence assembled at the time of this Round 2 close:

- **Layer 2 implementation cycle** (4 commits: `5ba62d5` / `326e25d` / `16ee420` / `98b5886`) — per AI Engineer R2 F5, per-commit cost evidence is NOT in the audit trail (a methodology-authoring gap routed to suite-side). Rough estimate: ~$10-15 total for the implementation cycle (the Phase 1a/1b/1c spec extension + the Phase 2a/2b implementation + manual-tests + Phase 2c annotation).
- **Round 1 cluster cold-sessions** (4 clusters × ~$2.25-5 per cluster per the AI Engineer R2 F2 estimate) — ~$10-20 total.
- **Round 1 fix cycle** (5 commits: `156ec53` / `d62bb1a` / `002d747` / `cdb46bc` / `9d56c3f`) — operator-directed inline fixes; cost-tally not surfaced in commit messages (same gap as the implementation cycle). Rough estimate: ~$5-10 total.
- **Round 2 cluster cold-sessions** (4 clusters; in progress; expected per AI Engineer R2 F2 estimate) — ~$10-15 total.
- **Full cycle cost estimate:** ~$35-60 total across implementation + Round 1 + fix cycle + Round 2.

**Calibration against capstone-intent expected band:** Layer 2's per-finding cost is below the project-cycle-calibrated band floor (100k tokens/finding) — read as Layer-scoped efficiency per [AI Engineer R2 F2](2026-05-21-ai-engineer.md#r2-f2), NOT under-investment. The methodology-refinement candidate (project-cycle vs. layer-cycle expected-band split) is the right framing — Layer-cycle cost calibrates to a smaller surface; the under-band per-finding cost is consistent with the smaller surface and does NOT indicate the review pipeline missed defects.

**SO seat's adjudication on proportionality:**

1. **The Layer 2 cycle's cost is proportionate** to its purpose (close three Layer-1-Deferred PE items + demonstrate VSDD applies across multiple layers in succession). The full-cycle cost (~$35-60) is well under the project-level Layer 1 cycle cost (per AI Engineer R1 F6 estimate: ~$200-400 cycle-wide). The Layer 2 cycle ran at ~15-30% of Layer 1's cost — which matches the ratio between Layer 2's smaller surface (~700 LoC delta) and Layer 1's full project surface (~4,000+ LoC including the initial commit + 3 review rounds + fix cycles).

2. **The fix cycle adopted Option 1 paths** (author the missing artifacts; adopt the not-applicable Phase 6 declaration) rather than Option 2 paths (amend the spec to defer further) at both load-bearing decision points (SO R4 F1 + SO R4 F2). The Option 1 path costs more in the implementation moment but preserves the spec contract's reference-implementation purpose end-to-end. The proportionality is honest about the operator's intent — the worked-example purpose IS satisfied by the spec being honored, not by the spec being weakened.

3. **The Round 2 cluster spawn** (4 parallel agents; this cluster being one of them) is itself proportionate. The Round 2 scope is narrower than Round 1 (verification of prior findings + adjacent-defect detection, not full re-scan) per the Phase 4 routing scope-reducer discipline ([AI Engineer R1 F2](2026-05-21-ai-engineer.md#r1-f2)). Lower per-round cost. The 4-cluster shape preserves adversarial-pair separation (Security ↔ Red Team; TW ↔ Doc Reviewer split per the [VDD-IAR R4 F3](2026-05-21-vdd-iar-alignment.md#r4-f3) cluster-shape verification).

**Disposition:** the Layer 2 cycle's cost-investment is proportionate to its purpose. No SO scope-discipline finding against the cycle's cost. The cluster's own AI Engineer review (Round 2; see [`2026-05-21-ai-engineer.md`](2026-05-21-ai-engineer.md) Round 2) carries the detailed cost-discipline analysis from the cost-discipline seat; the SO seat acknowledges the analysis + accepts the proportionality conclusion.

**Methodology-refinement candidate carried forward (not a Layer 2 defect, routed informally to suite-side):**

- The audit-trail cost-evidence gap on implementation-cycle commits (AI Engineer R2 F5) + on fix-cycle commits (this round's observation) is a methodology-authoring concern. Future cycles should record per-commit cost-tally in commit message bodies, matching the per-Review preamble discipline. Routes to suite-side as the AI Engineer R2 F5 carryforward already names; not a Layer 2 closure blocker.

**Classification:** Documented — the cost-proportionality concern surfaces as a documented disposition rather than a blocking finding; the operator's cost evidence is acceptable; the methodology-refinement carryforward is named for future-cycle regression-check.

---

### Summary

Round 2 verification: all four Round 1 SO findings have honest dispositions against the post-fix state. One new Round 2 documented disposition surfaced about cost-investment proportionality.

- **Round 1 Finding 1 verification ([r5-f1](#r5-f1))** — Resolved; `tests/scaling.rs` authored per Option 1; closes PE F5 chain end-to-end.
- **Round 1 Finding 2 verification ([r5-f2](#r5-f2))** — Resolved; Phase 6 NOT APPLICABLE declaration adopted per Option 1 (SO's own Round 1 recommendation); G-150 + G-112 named rationale preserved.
- **Round 1 Finding 3 verification ([r5-f3](#r5-f3))** — Resolved-and-holds; no new scope-creep introduced by fix cycle.
- **Round 1 Finding 4 verification ([r5-f4](#r5-f4))** — Resolved-and-holds; AC 13 weak-proxy posture unchanged.
- **New Round 2 disposition ([r5-f5](#r5-f5))** — Documented; cost-investment proportionality acceptable; methodology-refinement candidate (implementation-cycle cost-tally) routes to suite-side.

**MVR signal:** **REACHED at Round 2.** All four Round 1 SO findings closed cleanly; no new under-delivery or scope-creep surfaces; cost-proportionality acceptable.

**Coordination:** [r5-f5](#r5-f5) (cost-proportionality) cross-validates with [AI Engineer R2 Round 2](2026-05-21-ai-engineer.md) (the cluster's cost-discipline seat; the same evidence; same conclusion). [r5-f2](#r5-f2) (Phase 6 not-applicable) cross-validates with [VDD-IAR Alignment R5 Round 2](2026-05-21-vdd-iar-alignment.md) (the SO ↔ VDD-IAR validator-pair confirms the spec-amendment routing was clean).

**Phase 5 / Phase 6 closure-blocker check:** none. SO seat does NOT block Layer 2 from declaring closure of the project-terminal layer cycle. The Phase 6 NOT APPLICABLE declaration at DESIGN.md:17 is the project's terminal record for the Layer 2 layer-cycle (per G-150 + G-112); Phase 5 closure remains on the Layer 2 path per TODO.md:93 (Purity Boundary Audit + Mutation Testing re-runs + proptest now active at `tests/properties.rs`). No SO blocker.

**Cost-tally:** Round 2 contributed ~$0.43 across 3 verification entries + 1 new finding + 1 summary = ~$0.11 per-finding. Below the AI Engineer Dim 2 capstone-intent band floor consistent with [AI Engineer R2 F2](2026-05-21-ai-engineer.md#r2-f2) Layer-scoped efficiency reading.

**Validator:** vdd-iar-alignment (the SO ↔ VDD-IAR Alignment validator pair per [Review 77](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-77--2026-05-20-1545z); VDD-IAR R5 Round 2 confirms the Phase 6 not-applicable spec-amendment routing was clean + the SO disposition matches the prior-cycle intent).

---
