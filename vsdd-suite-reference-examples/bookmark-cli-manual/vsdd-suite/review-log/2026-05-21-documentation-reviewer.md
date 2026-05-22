# Documentation Reviewer Review — 2026-05-21

---

## Review 5 — 2026-05-21 22:00Z

**Phase:** [Phase 3](../../../../vsdd-suite/primers/3-review-session.md) — Iterative Adversarial Refinement.
**Source:** domain-raised — cold-session adversarial cold-reader; did not author the Layer 2 commits (`5ba62d5` / `326e25d` / `16ee420` / `98b5886`), the Layer 2 manual-test plan, or any of the post-PR-#43 narrative.
**Lens:** Cold-reader friction + sweep-discipline + phase-attestation chain readability + operator-action-queue continuity ([Documentation Reviewer domain prompt](../../../../vsdd-suite/domains/role/DOCUMENTATION-REVIEWER-REVIEW.md) Dim 1 + Dim 4 + Dim 6 + Dim 10 + Dim 11). Reads DESIGN.md LAST per the domain prompt § Current Review Prompt directive about cold-reader-poisoning.
**Scope:** Layer 2 promotion's user-facing artifact set — README.md, CHANGELOG.md, TODO.md § Layer 2, manual-tests/layer-2.md, PROCESS.md cross-references; cross-file consistency under Documentation Reviewer Dim 1 + Dim 4 + Dim 6.
**Surface:** Layer 2 promotion's cross-file consistency, README.md + CHANGELOG.md currency against the Layer 2 reality, TODO.md § Layer 2 readability, manual-tests/layer-2.md cold-reader executability.
**Reviewer:** Documentation Reviewer cold-session agent.
**Model:** Sonnet 4.6 (conceptually, per [`DESIGN.md`](../../DESIGN.md) § Cold-session budget — Sonnet for Documentation Reviewer / Technical Writer; this round runs at Opus 4.7 in practice as the cluster-batched single-context agent).
**Cold-session shape:** Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster (Solution Owner + Documentation Reviewer + AI Engineer + VDD-IAR Alignment) per the [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) § Cluster-batching pattern + AI Engineer R1 F1 cluster-with-adversarial-pair-separation discipline. The TW ↔ Documentation Reviewer adversarial pair is split — TW in QE/Security/Technical-Writer cluster + Documentation Reviewer here in Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster.
**Regression-check against:** [Documentation Reviewer Review 4 (2026-05-20-documentation-reviewer.md)](2026-05-20-documentation-reviewer.md#review-4--2026-05-21-1100z) (Layer 1 Doc Reviewer at MVR after the PR #40 per-domain-index retirement + post-Nathan-thread fix-cycle) is the regression baseline. Every R4 Resolved finding's grep-clean evidence must still hold; the post-Layer-2-commit state may have introduced new defects in the same defect classes.
**Cost-tally:** Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster agent — Opus 4.7 in practice (Sonnet 4.6 declared by `DESIGN.md § Cold-session budget`); this Documentation Reviewer round contributed ~25k input + ~13k output tokens ≈ ~$0.59 at Opus standard pricing; per-finding cost ~$0.10 across 6 findings (4 substantive + 2 documented-and-dismissed/hallucinated). Below the AI Engineer Dim 2 capstone-intent band floor — read as Layer-scoped efficiency per [AI Engineer R2 Finding 2](2026-05-21-ai-engineer.md#r2-f2).

**Session note:** Reading order followed the domain prompt § Current Review Prompt directive: project [`README.md`](../../README.md) FIRST (cold-reader landing point) → [`CHANGELOG.md`](../../CHANGELOG.md) → [`TODO.md`](../../TODO.md) (where README points) → [`manual-tests/layer-2.md`](../../manual-tests/layer-2.md) (where TODO points) → [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md) (referenced as cross-layer prerequisite by layer-2.md Step 0) → grep sweeps for "Layer 2" / "layer-2" / "scaling.rs" / "Layer-2-Deferred" / "AC 5"-"AC 13" / "Phase 6" across the project tree → [`PROCESS.md`](../../PROCESS.md) → [`DESIGN.md`](../../DESIGN.md) (read LAST per the domain prompt's cold-reader-poisoning discipline). The grep-clean sweep is the canonical Doc Reviewer Dim 4 + Dim 11 verification mechanism per the [Documentation Reviewer Review 4](2026-05-20-documentation-reviewer.md#review-4--2026-05-21-1100z) "grep -rn before claiming closure" pattern.

**MVR signal:** **Round 1 — NOT REACHED.** Three real findings + one operative-discipline finding surface. Two real defects are cold-reader landing-page staleness (the README claims Layer 1 is project-terminal; the CHANGELOG ends at the v0.12.3 Phase 6 Layer 1 attestation with no Layer 2 entry); one is a phase-attestation-chain readability gap (the four-commit-sequence `5ba62d5 → 326e25d → 16ee420 → 98b5886` requires audit-trail reconstruction effort because no file-level annotation says "this is the Phase 1a/1b commit; that is the Phase 2a/2b commit"); one Resolved finding documents the operator-action-queue continuity (the Review 74 manual-test split convention is correctly applied at Layer 2). Per [G-131](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-131) continue-trigger discipline, Round 2 is mandatory if any Open finding remains unresolved.

---

### Deferred

**Finding 1 — README.md cold-reader landing page is stale: claims "Layer 1 complete" + "Layers 2 ... not built" while Layer 2 IS built (Dim 1 + Dim 6 + Dim 9)**

<a id="r5-f1"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none — observable file content + observable Layer 2 implementation state)*
**Validator:** technical-writer

[`../../README.md`](../../README.md):9 reads:

> "Current state: **Layer 1 complete** (add + list). Layers 2 (tag + filter) and 3 (export + import) are scoped in [`DESIGN.md`](DESIGN.md) but not built — the reference-implementation purpose is satisfied by one layer end-to-end."

Per Dim 1 (clone-and-follow fidelity) + Dim 9 (onboarding sequencing), the cold reader landing on README.md absorbs this load-bearing claim in the project's first ~10 lines: "Layer 2 ... not built." The reader's working mental model is: this project ships Layer 1 only; Layer 2 is documented-but-unimplemented future work.

Per Dim 6 (documentation rot), the claim is **stale**. The Layer 2 implementation IS committed:

- [`../../src/lib.rs`](../../src/lib.rs):50-56 — `Bookmark.tags: Vec<String>` field with `#[serde(default)]`
- [`../../src/lib.rs`](../../src/lib.rs):90-117 — `AttachTagError` enum with `Display` + `Error` impls
- [`../../src/lib.rs`](../../src/lib.rs):377-397 — `BookmarkStore::attach_tag(url, label)` pure method
- [`../../src/lib.rs`](../../src/lib.rs):408-414 — `BookmarkStore::filter_by_tags(labels)` pure method
- [`../../src/main.rs`](../../src/main.rs):58-80 — `Cmd::Tag` + `Cmd::List { tags }` clap surface
- [`../../tests/bookmarks.rs`](../../tests/bookmarks.rs):504-1023 — 13 new Layer 2 integration tests (AC 5-13 + RFC 3339 closure + fsync weak-proxy)
- [`../../manual-tests/layer-2.md`](../../manual-tests/layer-2.md) — 13-step manual-test plan (556 lines)

README.md:50-59 § Phase progression table reinforces the same stale framing — the table has rows for Layer 1 Phases 1a + 1b + 2a + 2b + 3 + 4 only, no Layer 2 column or row. README.md:58 (Phase 3 row) reads "(7 of 12 active domains at MVR ...; AI Engineer R1 closed in PR #39; Round 4 cluster ... closed in PR #42 post-Nathan-Bluesky-thread feedback)" — the 7-of-12 scorecard is the pre-Round-3 number; post-PR-#42 the project reached 10-of-10 at MVR per [CHANGELOG v0.12.3 MVR scorecard](../../CHANGELOG.md) (lines 47-60). The README's scorecard is stale by ~3 PRs.

**Cold-reader impact** (the named failure mode "Mental-model interpolation" from the domain prompt § Sycophancy check): a stranger landing on README.md leaves with the wrong mental model — they believe Layer 2 is future work, when in fact it is the project's current active layer. They will NOT seek out `manual-tests/layer-2.md` to verify Layer 2 functionality because the README has told them Layer 2 doesn't exist. They will NOT understand why the [`bm --help`](../../src/main.rs:38-45) text shows `bm tag` + `bm list --tag` examples (the help text in `long_about` includes Layer 2 — see `src/main.rs:41-43`) — they may suspect a documentation-vs-implementation drift bug rather than recognizing the help text reflects the actual current state.

This is a Dim 1 clone-and-follow fidelity defect of the highest severity for a cold reader: the project's primary landing page contradicts its actual implementation.

**Disposition:** README.md must be updated to reflect the Layer 2 reality. Three load-bearing edits:

1. **README.md:9 — Current state line.** Rewrite from "Current state: **Layer 1 complete** ... Layers 2 ... not built" to a Layer 2-current phrasing such as "Current state: **Layer 2 active** (add + list shipped at Layer 1; tag + filter implementation complete at Layer 2, [Phase 3](../../vsdd-suite/primers/3-review-session.md) IAR cycles running). Layer 3 (export + import) is scoped in [`DESIGN.md`](DESIGN.md) but deferred." This preserves the load-bearing fact (reference-implementation purpose, scope-deferral note) while making the cold reader's mental model match the actual state.

2. **README.md:50-59 § Phase progression table** — add a Layer 2 sub-section or a second table for Layer 2, showing the same Phase 1a / 1b / 2a / 2b / 2c / 3 progression — with citations to commits `5ba62d5` (Phase 1a+1b+1c), `326e25d` (Phase 2a + 2b), `98b5886` (Phase 2c annotation), `16ee420` (manual-tests/layer-2.md), and the post-this-round QE/Security/Technical-Writer cluster/C/D Phase 3 review-log files. Phase 5 + Phase 6 rows remain as "pending Layer 2 closure" since this round is mid-cycle.

3. **README.md:58 Phase 3 row's scorecard** — refresh from "7 of 12 active domains at MVR" to "10 of 10 active capstone-tier role-domains at MVR at Layer-1 scope (per [`CHANGELOG.md`](CHANGELOG.md) v0.12.3); Layer 2 Phase 3 cycle in progress." The 13-domain active-set framing (12 role + 1 meta = 13) should also surface — the README still says "12 active domains" which is the pre-AI-Engineer count.

The natural owner is `technical-writer` per the [Documentation Reviewer Review 4 Finding 5](2026-05-20-documentation-reviewer.md#r4-f5) coordination pattern (TW authors the prose; Doc Reviewer surfaces the staleness from the cold-reader seat).

**Resolution:** README.md cold-reader landing page must be rewritten to reflect Layer 2 reality. TW owns the prose; SO owns the spec-claim ratification (the "Current state" rewrite changes a load-bearing project status assertion).

**Classification:** Deferred — Layer 2 acceptance gate cannot close while the project's primary landing page contradicts the implementation; fix routes to a TW-authored README + CHANGELOG update post-Round-1.

---

**Finding 2 — CHANGELOG.md ends at v0.12.3 (Layer 1 Phase 6 attestation) with no Layer 2 entry — the changelog is stale by 4 commits (Dim 6 documentation rot)**

<a id="r5-f2"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none — observable file state)*
**Validator:** technical-writer

[`../../CHANGELOG.md`](../../CHANGELOG.md):1-3 opens with:

> "# Changelog
>
> ## v0.12.3 Phase 6 four-dimensional convergence ATTESTED + UX/TW/QE cluster fix-cycle from @shimmermathlabs.com install-verification thread — 2026-05-21 13:30Z ([Review 88](../../vsdd-suite/suite-development/review-log/2026-05-21-suite-review.md#review-88--2026-05-21-1330z))"

That is the most recent entry. Subsequent file content (CHANGELOG.md:4-358) walks through the v0.12.3 closure narrative, the PR #42 Nathan-thread fix-cycle, the MVR scorecard at PR #42, and earlier v0.12.x / v0.11.x history. **No entry exists for the four Layer 2 commits** (`5ba62d5` Phase 1a/1b/1c spec extension + TODO decomposition; `326e25d` Phase 2a/2b Red Gate tests + tag/filter implementation + fsync; `16ee420` `manual-tests/layer-2.md`; `98b5886` Phase 2c extract-and-name annotation).

Per memory [feedback_log_timestamps.md] — CHANGELOG.md entries must use UTC Zulu timestamps. Per [Documentation Reviewer Dim 6](../../../../vsdd-suite/domains/role/DOCUMENTATION-REVIEWER-REVIEW.md) named failure modes: "CHANGELOG entries that contradict the current state" + "claims `Layer 4 complete` but the project has only Layer 2" — the same failure-mode-class, inverted: the changelog claims project-terminal-at-Layer-1 because that is its most recent entry, while the project has advanced beyond Layer 1.

**Cold-reader impact:** a stranger reading the CHANGELOG to understand the project's recent activity sees a story that ends with "Phase 6 four-dimensional convergence ATTESTED (project-terminal at Layer 1)" — they conclude the project is finished. They are wrong; Layer 2 has shipped four commits since the v0.12.3 entry.

**Disposition:** Author a new CHANGELOG entry for the Layer 2 implementation cycle. Standard shape (rendered inline to avoid the per-domain-discipline-hook's classification-heading false-positive on fenced markdown):

- **H2 header:** `vX.Y.Z (Layer 2 implementation — tag + filter) — 2026-05-21 HH:MMZ`
- **Added sub-section:** `bm tag <url> <label>` clap surface; `bm list --tag <label>` filter; 13 new Layer 2 integration tests + 1 lib unit test; `manual-tests/layer-2.md`; DESIGN.md Layer 2 sections (§ Scope and non-goals Layer 2 in-scope + § Behavioral contracts `bm tag` + § Behavioral contracts `bm list --tag` + § Edge case catalog Layer 2 + § Performance budget Layer 2 durability/scaling + § Threat model + § Storage data classification updates).
- **Changed sub-section:** `src/lib.rs` Bookmark.tags field + Layer 2 forward-only migration shape; `src/main.rs` Cmd::Tag + Cmd::List { tags } + 3 run_* helpers + handle_parse_error LABEL extension.
- **Methodology / process sub-section:** Phase 1a+1b spec extension + Phase 1c decomposition committed at `5ba62d5`; Phase 2a + 2b committed at `326e25d` (single-commit per the sub-agent's spawn shape — see VDD-IAR Alignment Layer 2 R4 for Red Gate evidence preservation finding); Phase 2c extract-and-name annotation committed at `98b5886` (clippy::too_many_lines trigger + justification beyond the lint); Phase 3 Layer 2 IAR cycle: Round 1 cluster spawns (SE/UX/Performance-Engineer cluster + B + C + D) running.
- **Status sub-section:** Layer 2 implementation code-complete; Layer 2 Phase 3 IAR Round 1 in progress (Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster — this entry's authoring cluster); Layer 2 Phase 5 + Phase 6 pending per DESIGN.md § Project intent.

The cold-reader test for the new entry: a stranger reading the CHANGELOG end-to-end after the entry lands should understand that Layer 2 is active, that the four-commit sequence is the Phase 1a/1b → 2a/2b → manual-tests → 2c progression, and that Phase 3/5/6 are pending.

**Resolution:** CHANGELOG.md must be extended with a Layer 2 implementation entry. Per the operator's memory standard, the timestamp must be UTC Zulu (`YYYY-MM-DD HH:MMZ`).

**Classification:** Deferred — Documentation rot defect at the cold-reader changelog surface; CHANGELOG update routes to TW post-Round-1.

---

**Finding 3 — Phase-attestation chain readability: the four-commit Layer 2 sequence cannot be reconstructed from in-file annotations alone — the audit trail requires walking `git log` (Dim 1 + Dim 3 + Dim 6)**

<a id="r5-f3"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(partially blocked by [Finding 2](#r5-f2) — once CHANGELOG.md is updated with the four-commit phase map, this defect is partially resolved; the in-file phase annotations remain as the residual concern)*
**Validator:** technical-writer

Per the operator's per-domain prompt: "Layer 2 commits crossed Phase 1a/1b → 1c → 2a → 2b → 2c → manual-tests; can a future audit-trail reviewer follow this from the commit history + the file-level annotations?"

The commits as observed by `git log`:

- `5ba62d5` — message: "bookmark-cli-manual: Layer 2 Phases 1a/1b/1c — DESIGN.md spec extension + TODO.md decomposition"
- `326e25d` — message: "bookmark-cli-manual: Layer 2 Phase 2a/2b — Red Gate tests + tag/filter implementation + fsync parent dir"
- `16ee420` — message: "bookmark-cli-manual: Layer 2 manual-tests/layer-2.md (closes PE F2 hyperfine sanity-check at the per-layer manual-test surface)"
- `98b5886` — message: "bookmark-cli-manual: Layer 2 Phase 2c — extract-and-name annotation in TODO.md"

The commit messages themselves are well-shaped — they name the Phase each commit lands and the closure-target where relevant. **A future audit-trail reviewer with access to `git log` can reconstruct the phase sequence cleanly.** That part is fine.

The defect is in the **file-level annotation surface**: a cold reader who lands at the project root without `git log` access (e.g., reading a snapshot of the source tree, or reading a future state where the commits have been squash-merged into a PR-level commit) sees:

- [`../../DESIGN.md`](../../DESIGN.md) — Layer 2 sections (the § Scope and non-goals Layer 2 entry at line 38-42; the § Behavioral contracts Layer 2 § `bm tag <url> <label>` at line 75-88; the § Behavioral contracts § `bm list --tag <label>` at line 90-99; the § Edge case catalog Layer 2 additions at line 112-121; the § Performance budget Data-scaling + Durability sub-sections at line 227-232) — **no in-file annotation** identifying which of these sections landed in Phase 1a/1b vs. Phase 1c vs. Phase 2a/2b spec-routing-back vs. operator inline edits.

- [`../../TODO.md`](../../TODO.md) § Layer 2 (line 46-92) — the whole block is one section. The Phase 2c annotation IS inline (line 83 "extract-and-name applied at Phase 2b commit 326e25d") — which is good. But the Phase 1a/1b vs. Phase 1c spec/decomposition split is not annotated.

- [`../../src/lib.rs`](../../src/lib.rs) — the Layer 2 additions (Bookmark.tags + AttachTagError + attach_tag + filter_by_tags + fsync_directory + save Layer 2 durability block) are annotated with `**Layer 2 — ...**` markers in the doc-comments — see `src/lib.rs:45`, `:72`, `:99`, etc. **This part is good.** A cold reader landing in src/lib.rs CAN identify Layer 2 additions.

- [`../../tests/bookmarks.rs`](../../tests/bookmarks.rs):504-513 has a clear `// ===== Layer 2 — Phase 2a Red Gate tests (tag + filter) =====` section header. **Also good** for the tests file.

- [`../../manual-tests/layer-2.md`](../../manual-tests/layer-2.md):3 explicitly names "Phase 2b implementation committed" + the cross-layer prerequisite to layer-1.md. **Good.**

**What's missing:** the Phase progression table in README.md (current line 50-59, Layer 1 only) is the natural place to annotate "Layer 2 Phases 1a/1b/1c committed at 5ba62d5 ; Phases 2a/2b at 326e25d ; manual-tests at 16ee420 ; Phase 2c annotation at 98b5886 ; Phase 3 IAR cycle in progress (SE/UX/Performance-Engineer cluster/B/C/D pending close)." That table extension is the natural file-level audit-trail annotation. If [Finding 1](#r5-f1)'s README rewrite includes the Phase progression table extension, this defect is also resolved.

Additionally, a CHANGELOG.md Layer 2 entry per [Finding 2](#r5-f2) would carry the same phase-attestation-chain information for the user reading the changelog rather than the README.

**Cold-reader impact:** moderate. A future audit-trail reviewer with `git log` access has full reconstruction capability; a snapshot reader (e.g., a portfolio evaluator reading a static export of the project) cannot reconstruct the phase sequence without walking individual file annotations + their dates + their cross-references. The defect is more about the audit-trail surface's robustness to non-git-equipped readers than about the immediate cold-reader landing experience.

**Disposition:** The phase-attestation-chain readability is restorable by either:

1. **The README Phase progression table extension proposed in [Finding 1](#r5-f1).** This is the canonical place; the README is where cold readers land first and where they look for "what state is the project in." A Layer 2 sub-table or a continuation row provides the four-commit phase map at the highest-discoverability surface.

2. **A CHANGELOG.md Layer 2 entry per [Finding 2](#r5-f2)** that includes the commit-by-commit phase map.

3. **OPTIONAL:** A `PROCESS.md` retrospective entry for the Layer 2 cycle once it closes. The Layer 1 PROCESS.md narrative is the canonical retrospective shape; an analogous Layer 2 section in PROCESS.md would document the cycle's history end-to-end. This is not required at Round 1 close; it is a natural close-of-Layer-2 artifact.

Of these, paths 1 + 2 are the operative defect closures; path 3 is the optional close-of-cycle deliverable.

**Resolution:** Paths 1 + 2 above; path 3 deferred to Layer 2 close-of-cycle.

**Classification:** Deferred — partial blocking by [Finding 2](#r5-f2); reachable closure once README + CHANGELOG are updated.

---

### Resolved

**Finding 4 — Operator-action-queue continuity confirmed: the Review 74 manual-test split convention is correctly applied at Layer 2; `manual-tests/layer-2.md` carries the required preamble fields + cross-layer prerequisite handoff (Dim 8 + Dim 10)**

<a id="r5-f4"></a>

**Owner:** documentation-reviewer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** technical-writer

Per the operator's per-domain prompt: "Does Layer 2 respect [Review 78 Manual-test split convention; Review 88 external-review-log codification]?"

**Review 74 manual-test split convention check** (Dim 10):

- [`../../TODO.md`](../../TODO.md):79 carries the `**Manual Testing Checklist:** [`manual-tests/layer-2.md`](manual-tests/layer-2.md)` pointer — matches the convention's pointer-and-file-pair pattern.
- [`../../manual-tests/layer-2.md`](../../manual-tests/layer-2.md):1-6 carries the required preamble:
  ```
  # Manual Testing — Layer 2: Tag and Filter

  **Layer:** [`TODO.md` § Layer 2 — Tag and filter](../TODO.md#layer-2--tag-and-filter)
  **Tested against:** Layer 2 [Phase 2b](../../../vsdd-suite/primers/2b-implementation.md) implementation committed (extends Layer 1 with `bm tag <url> <label>` + `bm list --tag <label>` + forward-only `tags` migration); [Phase 3](../../../vsdd-suite/primers/3-review-session.md) IAR rounds + [Phase 5](../../../vsdd-suite/primers/5-formal-hardening.md) re-runs pending per `TODO.md` § Layer 2 Layer-gate criteria.
  **Convention:** Review 74 manual-test split — this file is the per-layer manual-test plan; the corresponding `TODO.md` Layer 2 block points here. Parallel to [`manual-tests/layer-1.md`](layer-1.md).
  ```
  Both `**Layer:**` and `**Tested against:**` fields are present per the Review 74 + post-cutoff convention.
- Path location is `manual-tests/layer-2.md` per the convention (not `tests/manual/layer-2.md` or similar wrong-location).

**Cross-layer-prerequisite handoff** (Dim 8 — manual-test plan executability):

[`../../manual-tests/layer-2.md`](../../manual-tests/layer-2.md):7 explicitly names the cross-layer prerequisite:

> "**Prerequisite (cross-layer):** Layer 2 inherits the install-verification discipline established in Layer 1. If [`manual-tests/layer-1.md`](layer-1.md) has not been run in this shell session, run Steps 0–1 of [`layer-1.md`](layer-1.md) first — this Layer 2 plan does NOT re-cover Layer 1 manual tests; it builds on the same `cargo install --locked --path . --force --quiet` install + `$BOOKMARK_CLI_DB`-via-`mktemp` fixture pattern."

The handoff is explicit + actionable. The cold reader who has not run `layer-1.md` knows they need to. The session-state-preamble note at layer-2.md:9 names the `BOOKMARK_CLI_DB`-via-`mktemp` + working-directory-from-Step-0 invariants honestly — including the alternative path (set `BOOKMARK_CLI_DB` to a stable absolute path) for readers running steps in independent shells.

**Step structure** (Dim 8 named failure mode "binary lifecycle steps missing"):

- Step 0 (lines 14-31) refreshes the installed `bm` via `cargo install --locked --path . --force --quiet` — closes the named failure mode "Layer 2 manual-test opens with the Layer 1 binary already installed and Layer 2's behavior changes silently because the installed binary is stale." Discipline correctly applied.
- Step 1 (lines 35-66) initializes a 3-bookmark fixture with 1-second sleeps between adds — fixture is shared with Steps 2-9, isolated from Steps 10-11. Clear shape.
- Steps 2-11 exercise AC 5 through AC 12 (plus the empty-store precedence edge case) one at a time, with literal expected-output blocks per primer 1c § Manual testing checklist § Runnable-step standard.
- Step 12 (lines 442-528) is the hyperfine sanity-check + a `time` builtin fallback for sandbox environments — proportionate Layer 2 closure of PE F2.
- Step 13 cleanup.

**Closure protocol** (Dim 8 named "no escape valve when a manual-test step doesn't reach the expected output"):

layer-2.md:545-548 explicitly names the closure protocol — "Insight-reached / no findings" or "Findings surfaced" with per-divergence-as-finding routing to the appropriate per-domain log. Match to layer-1.md's pattern; convention preserved.

**Review 88 external-review-log codification check:** the project tree at [`vsdd-suite/external-review-log/`](../) — Layer 2 has not yet produced any external-review-thread artifact (the Nathan Bluesky thread was Layer 1 closure feedback per CHANGELOG.md v0.12.3; no Layer 2 external review is currently in flight). The codification's scope (per [Review 88](../../../../vsdd-suite/suite-development/review-log/2026-05-21-suite-review.md#review-88--2026-05-21-1330z)) is about external-review-thread mining + the per-finding-anonymization hook; no Layer 2 application has been triggered yet because no external Layer 2 feedback has surfaced. Discipline does not yet apply; will apply if/when an external Layer 2 thread occurs.

**Resolution:** Layer 2's manual-test plan respects every operative documentation convention from [Review 74](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-74--2026-05-20-1230z) (manual-test split + preamble fields + per-layer-file location) and [Review 88](../../../../vsdd-suite/suite-development/review-log/2026-05-21-suite-review.md#review-88--2026-05-21-1330z) (external-review-log codification, not-yet-applicable). The cross-layer prerequisite handoff is explicit; the closure protocol matches layer-1.md; the binary-lifecycle Step 0 refreshes the installed binary correctly.

**Classification:** Resolved — operative-discipline finding confirming convention-conformance for future-cycle regression-check.

---

### Dismissed

**Finding 5 — Initial candidate concern: TODO.md § Layer 2 is "7 levels deep with extensive prose" — re-read shows the structure is conventional + readable (Dim 5)**

<a id="r5-f5"></a>

**Owner:** documentation-reviewer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

Per the operator's per-domain prompt: "The TODO.md § Layer 2 block is 7 levels deep with extensive prose. Is it readable, or does it accumulate detail that would be better in DESIGN.md?"

Initial candidate concern: the operator's per-domain prompt flagged the TODO.md § Layer 2 block as potentially over-detailed.

Closer read of [`../../TODO.md`](../../TODO.md):46-92 (the full Layer 2 section) shows the structure is conventional + readable:

- One H2 heading (`## Layer 2 — Tag and filter`, line 46)
- Five **bold-tagged sub-blocks**: `**Status:**` (line 48), `**Acceptance criteria**` (line 50-60), `**Red Gate test plan**` (line 62-77), `**Manual testing checklist:**` (line 79), `**Layer 2 data-scaling tests:**` (line 81), `**Phase 2c (refactor):**` (line 83), `**Layer-gate criteria:**` (line 85-92)
- Bullet lists within `**Acceptance criteria**` (13 ACs) and within `**Red Gate test plan**` (13 Red Gate tests) and within `**Layer-gate criteria:**` (6 criteria)

The "7 levels deep" framing in the operator's per-domain prompt is interpretable two ways:

1. **Markdown heading depth** — heading levels (H1 / H2 / H3 / etc.). The Layer 2 block has H2 only; no H3 / H4 nesting. So 7-levels-deep does NOT apply to heading depth.

2. **Bullet/sub-bullet indentation depth** — the Red Gate test plan bullets are flat (no nested bullets); the Acceptance criteria are flat; the Layer-gate criteria are flat. The indentation depth is shallow (one level of bullets at most). So 7-levels-deep does NOT apply to indentation depth either.

The operator's framing may have been about prose-density — the Layer 2 § Status paragraph at line 48 is one long sentence (~250 words) and the **Phase 2c** paragraph at line 83 is similarly dense (~270 words). Long paragraphs are not the same as deep nesting; they are a separate readability question. From the cold-reader seat, the long paragraphs are still readable — each names a specific topic (Status: where the layer is in the methodology cycle; Phase 2c: the extract-and-name refactor annotation per G-161). Splitting them into multiple paragraphs would be a stylistic improvement but is not a defect.

**Should the content be in DESIGN.md instead?** No. DESIGN.md is the spec contract — what to build, behaviorally. TODO.md is the layer plan — how to build it (which phases, which Red Gate tests, which acceptance gates). The Layer 2 § Status, the Red Gate test plan, the Phase 2c annotation, the Layer-gate criteria all belong in TODO.md per the primer 1c § Phase 1c decomposition convention. Moving them to DESIGN.md would conflate the spec with the layer plan + violate the spec/plan separation that primer 1a+1b + primer 1c are designed around.

**Resolution:** Initial candidate concern was misread. TODO.md § Layer 2 is conventionally structured + readable + correctly placed. No defect.

**Classification:** Dismissed — closer read confirms TODO.md § Layer 2 is conventionally structured for the Phase 1c decomposition output role.

---

### Hallucinated

**Finding 6 — Initial candidate concern: `tests/scaling.rs` top-level module docstring missing — verified that the file itself is missing, which routes to a different concern (Dim 11)**

<a id="r5-f6"></a>

**Owner:** documentation-reviewer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

Per the operator's per-domain prompt: "The `tests/scaling.rs` file (if it exists) — does it have a top-level module docstring that explains its purpose to a reader who arrives there cold?"

Verified state: `tests/scaling.rs` **does not exist**. The directory listing of [`../../tests/`](../../tests/) returns only `bookmarks.rs`. The Documentation Reviewer Dim 11 module-docstring inquiry presumes the file exists; the file is missing.

The missing-file question is therefore routed to [Solution Owner Review 4 Finding 1](2026-05-21-solution-owner.md#r4-f1) (the under-delivery against DESIGN.md:230 + TODO.md:81 spec commitments closing PE F5). It is NOT a Documentation Reviewer module-docstring finding — it is an SO under-delivery finding. The Documentation Reviewer concern about module-docstring readability for the hypothetical `tests/scaling.rs` becomes operative only after the SO finding is resolved (path 1: author the file) or the spec is amended (path 2: defer further). At that point, if `tests/scaling.rs` lands, Doc Reviewer would re-evaluate its top-level module docstring for cold-reader fitness.

**Classification:** Hallucinated — the candidate concern was malformed against the actual artifact state; the underlying defect (file missing) routes to a different domain's finding ([SO R4 F1](2026-05-21-solution-owner.md#r4-f1)).

---

### Summary

Six findings in Round 1:

- **Deferred (fix routes to post-Round-1 TW-authored update):**
  - [Finding 1](#r5-f1) — README.md cold-reader landing page stale (claims Layer 1 is project-terminal while Layer 2 IS built); Dim 1 + Dim 6 + Dim 9 defect at the project's primary navigation surface
  - [Finding 2](#r5-f2) — CHANGELOG.md ends at v0.12.3 (Layer 1 Phase 6 attestation) with no Layer 2 entry; Dim 6 documentation rot
  - [Finding 3](#r5-f3) — Phase-attestation chain readability requires `git log` reconstruction; partially resolved by [Finding 2](#r5-f2) once CHANGELOG is updated
- **Resolved:**
  - [Finding 4](#r5-f4) — Operator-action-queue continuity (Review 74 manual-test split convention) correctly applied at Layer 2; `manual-tests/layer-2.md` carries required preamble + cross-layer prerequisite handoff
- **Dismissed:**
  - [Finding 5](#r5-f5) — TODO.md § Layer 2 prose density was the operator's per-domain-prompt initial concern; closer read shows the structure is conventional + correctly placed
- **Hallucinated:**
  - [Finding 6](#r5-f6) — module-docstring check against `tests/scaling.rs` was malformed (file doesn't exist); underlying defect routes to [SO R4 F1](2026-05-21-solution-owner.md#r4-f1)

**Operator-supplied per-domain-prompt answers (summarized for the audit trail):**

1. _"Does README.md still claim Layer 1 is the artifact's terminal state? Does it advertise Layer 2's surface?"_ — Yes, README.md:9 + README.md:50-59 still claim Layer 1 is terminal + do NOT advertise Layer 2. This is [Finding 1](#r5-f1).

2. _"Has CHANGELOG.md been updated for Layer 2 changes, or is it stale (Layer 1's last entry being the most recent)?"_ — Stale. v0.12.3 (Layer 1 Phase 6 attestation) is the most recent entry; no Layer 2 entry exists. This is [Finding 2](#r5-f2).

3. _"Is the TODO.md § Layer 2 block readable, or does it accumulate detail that would be better in DESIGN.md?"_ — Readable. Conventional Phase 1c structure; spec content correctly placed in TODO vs. DESIGN. [Finding 5](#r5-f5) is the dismissed framing.

4. _"Does `tests/scaling.rs` have a top-level module docstring?"_ — File missing entirely. The question is malformed; underlying defect routes to SO. [Finding 6](#r5-f6) is the Hallucinated framing; the underlying defect is [SO R4 F1](2026-05-21-solution-owner.md#r4-f1).

**Coordination:** [Finding 1](#r5-f1) (README staleness) is the natural TW ownership per the [Documentation Reviewer Review 4 Finding 5](2026-05-20-documentation-reviewer.md#r4-f5) coordination pattern. [Finding 2](#r5-f2) (CHANGELOG staleness) is also TW-owned. [Finding 3](#r5-f3) (Phase-attestation chain) is partially resolved by [Finding 2](#r5-f2)'s CHANGELOG update; the residual concern is the README Phase progression table extension proposed under [Finding 1](#r5-f1). All three Open findings cluster around a single closure: the post-Layer-2-Round-1 fix-cycle authors README.md + CHANGELOG.md updates that close [Finding 1](#r5-f1) + [Finding 2](#r5-f2) + the residual [Finding 3](#r5-f3) component.

The cross-cluster coordination: TW in QE/Security/Technical-Writer cluster is likely surfacing the same README + CHANGELOG staleness from the authorial seat (TW Dim 13 unlinked-references check + TW Dim 11 documentation-rot check). Doc Reviewer here surfaces from the cold-reader seat. The closure path is one set of file edits closing both clusters' findings.

**Cost-tally** (per [`suite-development/suite-development.md`](../../../../vsdd-suite/suite-development/suite-development.md) § Per-review entry preamble § Cost-tally): cold-session within Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster quartet; per-cluster cost ~$5 estimate per AI Engineer R1 F1 precedent; per-finding cost ~$0.83 across 6 findings (4 substantive + 2 documented-and-dismissed). Token estimate: ~50-60k input + ~40-50k output ≈ 100k tokens cluster-wide ≈ ~$2.25/cluster across 4 domains ≈ ~$0.56 per domain ≈ ~$0.09 per finding at the small-cluster cost. The cost is well below the capstone-intent expected band floor (50k–150k tokens/finding for portfolio; 100k–300k for capstone per AI Engineer Dim 2) — read as parallel adversarial review running efficiently. Awaiting cluster-close cost tally.

**Validator:** technical-writer (the TW ↔ Documentation Reviewer adversarial-pair validator — TW validates from authorial seat that Doc Reviewer's cold-reader findings produce closable fixes without introducing author-side blindspots). The Dismissed + Hallucinated dispositions ([Finding 5](#r5-f5) + [Finding 6](#r5-f6)) carry `**Validator:** sanity-check` per the meta-validator-of-last-resort pattern ([Review 77](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-77--2026-05-20-1545z) Finding 2).

---
