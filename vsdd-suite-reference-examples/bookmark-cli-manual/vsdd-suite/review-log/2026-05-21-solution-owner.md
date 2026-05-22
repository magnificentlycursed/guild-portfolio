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
