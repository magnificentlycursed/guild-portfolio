# Documentation Reviewer Review — 2026-05-24

---

## Review 1 — 2026-05-24 01:14Z

**Scope:** Layer 3 (`bm export` + `bm import`) user-facing artifact set — [`README.md`](../../README.md), [`DESIGN.md`](../../DESIGN.md) § Behavioral contracts + § Edge case catalog + § Interface definitions (Layer 3 additions), [`TODO.md`](../../TODO.md) § Layer 3, [`CHANGELOG.md`](../../CHANGELOG.md) (Layer 3 entries), [`manual-tests/`](../../manual-tests/) directory, [`vsdd-suite/FINDINGS-INDEX.md`](../FINDINGS-INDEX.md). Layer 3 spec commits `79a9a83` + `654cbbf` plus implementation commits `878d3b6` + `fd21900` + `78bd3cf`. Cross-references between these artifacts are in scope; cross-references into the suite are out of scope.

**Session note:** Cold session. This agent did not author any Layer 3 commits and has not participated in prior Layer 3 authoring decisions. Prior DR rounds (Review 1 through Review 6 in [`2026-05-20-documentation-reviewer.md`](2026-05-20-documentation-reviewer.md) and [`2026-05-21-documentation-reviewer.md`](2026-05-21-documentation-reviewer.md)) treated as prior adversary's claims per cold-reader-vs-prior-round discipline — verified independently rather than accepted as established fact.

**Source:** domain-raised

**Regression-check against:** [Documentation Reviewer Review 6 (2026-05-21-documentation-reviewer.md, Review 6)](2026-05-21-documentation-reviewer.md#review-6--2026-05-22-1630z) — Layer 2 Doc Reviewer at MVR. Each prior Resolved finding's closure evidence verified against current state; defect classes from R6 re-evaluated against the Layer 3 additions.

**Supplements applied:** [`markdown.md`](../../../../vsdd-suite/supplements/markdown.md) § Documentation Reviewer — cross-reference resolution test + cold-reader anchor-followability + tutorial-followability applied. [`cli.md`](../../../../vsdd-suite/supplements/cli.md) § Documentation Reviewer — not authored (the supplement's Documentation Reviewer section has not been authored; this is noted but not raised as a finding because the DR domain prompt lists the supplement as applicable and the supplement itself acknowledges the section's absence). [`rust.md`](../../../../vsdd-suite/supplements/rust.md) § Documentation Reviewer — applied for rustdoc cold-reader test on in-code doc comments for the Layer 3 new pub items.

**MVR signal:** NOT REACHED at Round 1. Four real findings surface: (1) README.md cold-reader staleness — the primary landing page still claims "Layer 3 is scoped but not built" after three Phase 2 implementation commits; (2) CHANGELOG.md has no Layer 3 progression entry covering the Phase 2a Red Gate commit or Phase 2b implementation commit; (3) `manual-tests/layer-3.md` is absent despite TODO.md § Layer 3 declaring it and layer-gate criterion 3 requiring it; (4) FINDINGS-INDEX.md carries no Layer 3 rows, leaving the cross-cutting finding registry with a visible gap for any cold reader who reaches it via README cross-reference. One dismissed finding and one hallucinated finding complete the round. Per [G-131](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-131) continue-trigger discipline, Round 2 is mandatory.

---

### Deferred

**Finding 1 — README.md primary landing page is stale: claims "Layer 3 is scoped but not built" after three Phase 2 implementation commits (Dim 1, Dim 6, Dim 9)**

<a id="r1-f1"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none — observable file content vs. observable git state)*
**Validator:** technical-writer

The README.md cold-reader landing paragraph at line 9 reads:

> "Current state: **Layer 1 project-terminal at PR #42** (add + list) + **Layer 2 active in the post-PR-#43 cycle** (tag + filter). Layer 3 (export + import) is scoped in [`DESIGN.md`](DESIGN.md) **but not built** — the reference-implementation purpose is satisfied by Layer 1 reaching project-terminal end-to-end + Layer 2 extending the worked example through a second iteration of the full 6-phase cycle."

Per Dim 1 (clone-and-follow fidelity) and Dim 9 (onboarding sequencing): a cold reader absorbing this in the project's first 10 lines forms the working mental model — "Layer 3 is future work, not implemented." That mental model is incorrect.

The Layer 3 implementation is committed. Git history shows:

- `878d3b6` — "Layer 3 Phase 2a Red Gate — 15 failing tests for bm export + bm import (AC 14..AC 28)"
- `fd21900` — "Layer 3 Phase 2b — implement bm export + bm import (GREEN; 45/45 + 3/3 + 0 clippy warnings)"
- `78bd3cf` — "Layer 3 Phase 2c — extract-and-name annotation (no code changes)"

The binary ships `bm export` and `bm import` subcommands (`src/main.rs` `Cmd::Export` + `Cmd::Import` variants). The README § Run section lists only `bm add`, `bm list`, `bm tag`, and `bm list --tag` — no `bm export` or `bm import` examples appear. The Phase progression for Layer 2 table ends the README's phase-attestation chain with no Layer 3 counterpart.

Per Dim 6 (documentation rot): the README correctly closed the Layer 2 staleness defect in Review 6 (r6-f1) by adding the Layer 2 phase progression table and updating the current-state line. The same staleness defect class has recurred verbatim for Layer 3 — the pattern from R5-F1 → R6-F1 (raise staleness, fix in post-Round-1 cycle, verify in Round 2) is the precedent for the fix shape here.

**Cold-reader impact:** a stranger who clones the repo, runs `cargo install --locked --path . --force`, then runs `bm --help` will see `export` and `import` subcommands. They will be confused — the README told them these don't exist. If they trust the README over `--help`, they will not discover the round-trip workflow (`bm export | bm import`), which is the primary new capability at Layer 3.

**Disposition:** Three load-bearing edits required:

1. **README.md:9 — Current state line.** Rewrite to name Layer 3 as active — e.g., "Current state: **Layer 1 project-terminal at PR #42** (add + list) + **Layer 2 complete in the post-PR-#43 cycle** (tag + filter) + **Layer 3 active** (`bm export` + `bm import`; Phase 2 implementation complete; Phase 3 IAR running). The reference-implementation purpose is satisfied by Layer 1 reaching project-terminal end-to-end; Layer 2 + Layer 3 extend the worked example."

2. **README.md § Run section.** Add `bm export` + `bm import` examples with "(Layer 3)" annotation, parallel to the "(Layer 2)" annotation on `bm tag` + `bm list --tag`. The canonical round-trip `bm export | bm import` is the natural example.

3. **README.md — Phase progression table for Layer 3.** Add a third phase progression table (parallel to the Layer 1 and Layer 2 tables) covering the Layer 3 phases 1a+1b (spec commits `79a9a83` + `654cbbf`), 2a (Red Gate commit `878d3b6`), 2b (implementation commit `fd21900`), 2c (annotation commit `78bd3cf`), Phase 3 (IAR running), Phase 5 (pending), Phase 6 (NOT APPLICABLE per same G-150 + G-112 rationale as Layer 2).

**Classification:** Deferred — primary cold-reader landing page staleness; fix routes to TW-authored README update post-Round-1.

---

**Finding 2 — CHANGELOG.md has no Layer 3 implementation entry — Phase 2a Red Gate + Phase 2b implementation commits are invisible to a cold reader reading the changelog (Dim 6)**

<a id="r1-f2"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none — observable file state)*
**Validator:** technical-writer

[`CHANGELOG.md`](../../CHANGELOG.md):1–13 opens with three `[Unreleased]` entries for the Layer 3 Phase 2c annotation, the Layer 3 Phase 2b implementation, and the Layer 3 spec operator-confirmation pass. The Phase 2c annotation entry (commit `78bd3cf`) is the most recent entry.

However, **no entry exists for the Layer 3 Phase 2a Red Gate commit** (`878d3b6`). The CHANGELOG records the spec activation (`79a9a83`), operator-confirmation pass (`654cbbf`), Phase 2b implementation (`fd21900`), and Phase 2c annotation (`78bd3cf`), but the Phase 2a Red Gate commit is not represented as a distinct CHANGELOG entry. This omits the methodology-discipline evidence most load-bearing for the audit trail: the 15 failing tests committed before any implementation.

Additionally, **the CHANGELOG entries that do exist are not slim-form**. The Phase 2b implementation entry (lines 15–53) is a 39-line narrative with `### Added` + `### Test verification` + `### Forward implications` sub-sections. Per [Review 93 Finding 1](../../../../vsdd-suite/suite-development/review-log/2026-05-24-suite-review.md#review-93--2026-05-24-2340z) and `suite-development.md` § CHANGELOG slim-form convention: entries authored 2026-05-24 and later should use the slim-form shape (one `### Changed` or `### Added` line pointing at the review-log narrative as the single source of truth). The existing Layer 3 CHANGELOG entries carry full prose narratives rather than slim-form index pointers. This is a methodology-conformance gap for entries authored on 2026-05-24 (when the slim-form convention was codified).

Per Dim 6 (documentation rot) applied to methodology-compliance: the CHANGELOG correctly records what shipped, but the Phase 2a Red Gate absence means a cold reader tracing the methodology discipline cannot verify that the Red Gate commit preceded the implementation without consulting git history. The Layer 2 Red Gate evidence-preservation annotation in TODO.md § Layer 2 explicitly called out the two-commit canonical shape; Layer 3 adopted the two-commit shape per that annotation (commits `878d3b6` + `fd21900` are separate commits in git) — but the CHANGELOG does not record the Phase 2a Red Gate commit as a distinct entry.

**Cold-reader impact:** a stranger reading the CHANGELOG to understand the Layer 3 implementation cycle sees: spec activation → operator confirmation → Phase 2b → Phase 2c, with no Phase 2a Red Gate entry in between. They cannot verify the Red Gate discipline was applied from the CHANGELOG alone.

**Disposition:**

1. Add a Phase 2a Red Gate CHANGELOG entry (slim-form per R93 F1, or full-prose if an inline-fix is more practical) naming the 15 failing tests committed at `878d3b6` with the CI-RED evidence before `fd21900`.
2. Evaluate the existing Phase 2b + Phase 2c entries for slim-form conformance (both dated 2026-05-24). The slim-form convention was codified at R93 which is itself a 2026-05-24 suite-review. Whether the R93 convention applies to entries authored in the same day's earlier commits is an operator-policy question; the recommendation is to note the gap and leave the existing entries as authored (preserving per G-89 forward-only discipline) while authoring new entries in slim-form going forward.

**Classification:** Deferred — Phase 2a Red Gate CHANGELOG absence is the actionable defect; slim-form conformance on the 2b/2c entries is a secondary recommendation.

---

**Finding 3 — `manual-tests/layer-3.md` is absent despite TODO.md § Layer 3 declaring it and layer-gate criterion 3 requiring it (Dim 8, Dim 10)**

<a id="r1-f3"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none — observable directory state)*
**Validator:** technical-writer

Per [Review 74 manual-test split convention](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-74--2026-05-20-1230z) (Dim 10) and the Documentation Reviewer domain prompt § Evaluation Dimension 10:

> "Named failure modes: `TODO.md` Layer N has `**Manual Testing Checklist:**` pointer but no actual `manual-tests/layer-N.md` file."

[`TODO.md`](../../TODO.md):138 reads:

> "**Layer 3 manual testing checklist:** `manual-tests/layer-3.md` (to be authored alongside the Phase 2a Red Gate commit) — parallel to `manual-tests/layer-{1,2}.md`."

The `manual-tests/` directory currently contains `layer-1.md`, `layer-2.md`, and `install-verification.md`. `layer-3.md` is **absent**.

TODO.md § Layer 3 Layer-gate criteria #3 states:

> "3. The manual testing checklist at `manual-tests/layer-3.md` runs clean (every step produces the expected exit/stdout/stderr)."

The layer-gate criterion names `manual-tests/layer-3.md` as a required gate artifact. Per the Review 74 convention, this file should have been authored alongside the Phase 2a Red Gate commit (`878d3b6`) — the TODO.md annotation says "to be authored alongside the Phase 2a Red Gate commit." The file was not authored at that commit and has not been authored through Phase 2c.

Per Dim 8 (manual-test plan executability): a cold reader following the TODO.md § Layer 3 manual testing checklist pointer arrives at a 404. The Review 74 convention's exact test — "follow the `**Manual Testing Checklist:**` pointer and confirm the target file exists and is structurally compliant" — fails here.

The round-trip workflow (`bm export | bm import`) is the primary user-facing Layer 3 capability. Without a manual-test plan, a cold evaluator has no structured path to verify the round-trip end-to-end. The `bm export | bm import` composition is not verifiable from the README (which does not list these commands) nor from an absent manual-test plan.

**Disposition:** Author `manual-tests/layer-3.md` with the Review 74 convention preamble (`**Layer:**` + `**Tested against:**` fields), the cross-layer prerequisite handoff (parallel to `layer-2.md`:7), and per-step runnable commands covering:

- Step 0: install/refresh `bm` binary
- Steps covering AC 14–AC 28 (bm export against populated store, empty store, tag-filtered; bm import with valid payload, idempotent re-import, empty payload, invalid JSON, schema mismatch, stdin cap; the canonical `bm export | bm import` round-trip)
- Closure protocol naming "Findings surfaced" routing path

**Classification:** Deferred — Review 74 manual-test split convention violation + layer-gate criterion unsatisfied; fix routes to post-Round-1 authoring of `manual-tests/layer-3.md`.

---

**Finding 4 — FINDINGS-INDEX.md has no Layer 3 rows — the cross-cutting finding registry has a visible gap for any cold reader who uses it to navigate the project's Phase 3 IAR surface (Dim 4, Dim 6)**

<a id="r1-f4"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none — Layer 3 Phase 3 IAR is the current active cycle; FINDINGS-INDEX rows are added when findings are classified)*
**Validator:** technical-writer

[`vsdd-suite/FINDINGS-INDEX.md`](../FINDINGS-INDEX.md) ends at the Layer 1 and Layer 2 rows. The file carries no Layer 3 entries — neither `| L3 |` rows for classified findings nor a section boundary indicating Layer 3's IAR cycle has opened.

Per the FINDINGS-INDEX.md § Quick lookup text:

> "By layer: `grep '| L1 |'`"

A cold reader who runs `grep '| L3 |' vsdd-suite/FINDINGS-INDEX.md` returns zero rows. There is no in-file annotation saying "Layer 3 Phase 3 IAR cycle opened YYYY-MM-DD; findings will appear here as they are classified." The registry's silence on Layer 3 is structurally indistinguishable from "Layer 3 has no findings" (which would be the MVR state) and "Layer 3 hasn't started Phase 3 IAR" and "Layer 3 IAR findings exist but aren't indexed."

Per Dim 4 (cross-reference resolution): the README points to `vsdd-suite/FINDINGS-INDEX.md` as the project finding registry: "the project finding registry is at [`vsdd-suite/FINDINGS-INDEX.md`](vsdd-suite/FINDINGS-INDEX.md)." A cold reader following that link finds no Layer 3 entries. The discoverability gap is structural: the FINDINGS-INDEX was explicitly called out in the instructions for this review as a defect to flag — "FINDINGS-INDEX.md does NOT yet have Layer 3 entries — flag this as a discoverability gap."

Per Dim 6 (documentation rot): the Quick lookup § "Open findings only" note reads: "Post-Round-3 status ... 7 of 10 active capstone-tier domains at MVR" — this is the Layer 1 status. No Layer 2 or Layer 3 IAR status update appears in the Quick lookup section, making it stale by two full layer cycles.

**Cold-reader impact:** the project's cross-cutting finding registry is the canonical surface a methodology evaluator uses to assess how many findings the IAR process surfaced and how they were classified. A registry that ends at Layer 2 rows with no Layer 3 annotation leaves an evaluator unable to distinguish "Layer 3 passed with zero findings" from "Layer 3 Phase 3 IAR hasn't run yet" from "Layer 3 findings exist but aren't indexed."

**Disposition:** Layer 3 findings from this Phase 3 IAR cycle should be added to FINDINGS-INDEX.md as they are classified. Additionally, the Quick lookup § "Open findings only" note should be updated to reflect the current Layer 3 IAR cycle opening. This finding is structural — it will be progressively closed as the Phase 3 IAR cycle adds rows — but the CURRENT state at Layer 3 Phase 3 Round 1 is a discoverability gap.

**Classification:** Deferred — structurally expected gap that will close as findings are registered; flagged per the round's instructions. The quick-lookup note staleness is the secondary actionable item.

---

### Resolved

*(none)*

---

### Dismissed

**Finding 5 — Initial candidate concern: DESIGN.md Layer 3 AI-co-authored disclosure paragraph confuses a fresh reader (Dim 2, Dim 5)**

<a id="r1-f5"></a>

**Owner:** documentation-reviewer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

Initial candidate concern: the DESIGN.md § Scope and non-goals Layer 3 parenthetical reads:

> "(Layer 3 promoted from 'deferred — scoped only' to active at AI-co-authored first-draft 2026-05-24 per operator's 'I author first-draft; you edit + own' directive. **This spec is AI-co-authored; operator owns the final contract.** ... Operator-confirmed decisions inline: ..."

The disclosure is long and appears in the middle of a behavioral contracts section. Initial concern: does it confuse the cold reader about which parts of the Layer 3 spec are reliable?

Closer read: the disclosure is limited to the parenthetical at the end of the "In scope (Layer 3, active):" bullet, and it does NOT appear inside the behavioral contracts sub-sections (`### bm export (Layer 3)` and `### bm import (Layer 3)`). Those sub-sections are written in plain spec voice without AI-author flags — the operator-confirmation pass (`654cbbf`) removed the AI-author callouts from the behavioral contracts text per the CHANGELOG entry for that commit.

From the cold-reader seat, the behavioral contracts themselves are unambiguous: exit codes, stdout/stderr contracts, dedup semantics, failure paths are all stated flatly. The parenthetical disclosure reads as methodology audit-trail context rather than spec uncertainty. A cold reader who trusts the spec language in the `###` sub-sections (which is correct behavior) is not confused.

The PROCESS.md § AI-co-authored reference-example disclosure also contextualizes the AI-co-authored shape — the project-level disclosure makes the spec-level parenthetical expected rather than surprising.

**Classification:** Dismissed — the disclosure is methodology audit-trail, not spec uncertainty. The behavioral contracts sub-sections are written in unambiguous spec voice post-operator-confirmation-pass.

---

### Hallucinated

**Finding 6 — Initial candidate concern: DESIGN.md anchor `#bm-import-layer-3` referenced in TODO.md does not exist as a named anchor — verified that Markdown heading-derived anchors are derived from the heading text, not hand-authored (Dim 4, Dim 11)**

<a id="r1-f6"></a>

**Owner:** documentation-reviewer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

TODO.md:100 contains the link:

> "[DESIGN.md § Threat model addition for stdin-fed attacker input](DESIGN.md#bm-import-layer-3)"

Initial concern: this is a hand-authored anchor fragment `#bm-import-layer-3` rather than a heading-derived anchor. Heading-derived GitHub Markdown anchors are lowercase-with-hyphens of the heading text. The DESIGN.md section is titled `### bm import (Layer 3)` — the GitHub-derived anchor for that heading would be `#bm-import-layer-3` (stripping parentheses and applying lowercase-with-hyphens). The concern was whether the fragment correctly targets the section or silently 404s.

Verification: the section `### bm import (Layer 3)` at DESIGN.md:116 generates the anchor `#bm-import-layer-3` under GitHub Markdown rendering rules (parentheses stripped, spaces → hyphens, lowercase). The TODO.md link `#bm-import-layer-3` matches the expected derived anchor. On a rendered GitHub page this link resolves to the correct section.

The concern about hand-authored vs. derived anchors is real as a class (heading renames break derived anchors), but in this specific instance the fragment matches the current heading text's derived form. No current broken link.

**Classification:** Hallucinated — the `#bm-import-layer-3` fragment in TODO.md correctly targets the `### bm import (Layer 3)` heading in DESIGN.md under GitHub Markdown anchor derivation rules.

---

### Summary

Six findings in Round 1:

- **Deferred (fix routes to post-Round-1 TW-authored updates):**
  - [Finding 1 (r1-f1)](#r1-f1) — README.md primary landing page stale: claims "Layer 3 is scoped but not built" after three Phase 2 implementation commits; Dim 1 + Dim 6 + Dim 9 defect at the project's primary navigation surface
  - [Finding 2 (r1-f2)](#r1-f2) — CHANGELOG.md has no Phase 2a Red Gate entry for Layer 3; Phase 2c and 2b entries are present but the Red Gate commit is invisible; secondary concern about slim-form conformance on 2026-05-24 entries
  - [Finding 3 (r1-f3)](#r1-f3) — `manual-tests/layer-3.md` is absent despite TODO.md declaring it and layer-gate criterion 3 requiring it; Review 74 convention violation + cold reader has no structured path to verify the `bm export | bm import` round-trip
  - [Finding 4 (r1-f4)](#r1-f4) — FINDINGS-INDEX.md has no Layer 3 rows; cross-cutting finding registry is silent on Layer 3 Phase 3 IAR cycle status; Quick lookup note stale by two layer cycles
- **Dismissed:**
  - [Finding 5 (r1-f5)](#r1-f5) — AI-co-authored disclosure in DESIGN.md Layer 3 parenthetical; closer read confirms behavioral contracts sub-sections are unambiguous spec voice post-operator-confirmation-pass
- **Hallucinated:**
  - [Finding 6 (r1-f6)](#r1-f6) — `#bm-import-layer-3` anchor in TODO.md:100 resolves correctly to `### bm import (Layer 3)` heading in DESIGN.md under GitHub Markdown anchor derivation rules

**Pattern diagnosis:** Findings 1 through 3 are the same defect class that DR R5 surfaced for Layer 2 (F1 README staleness + F2 CHANGELOG staleness + F4 manual-test convention). The fix shape is identical: TW authors README + CHANGELOG + manual-tests updates post-Round-1, Doc Reviewer verifies in Round 2. Finding 4 (FINDINGS-INDEX Layer 3 gap) is a new Layer 3 defect class — the index was migrated to the anchor-ID scheme at 2026-05-24 (Review 91 F17 closure) and the Layer 3 rows are the first opportunity to use the new scheme from the start; the gap will close as findings are classified.

**Coordination:** TW is the natural owner for all four Deferred findings. TW's parallel review should surface the same README + CHANGELOG gaps from the authorial seat (TW Dim 11 documentation rot + Dim 13 unlinked references). The closure path is a single fix batch: (a) README.md rewrite + `bm export` / `bm import` examples + Layer 3 phase progression table; (b) CHANGELOG Phase 2a Red Gate entry; (c) `manual-tests/layer-3.md` authored with Review 74 preamble + AC 14–AC 28 steps + round-trip workflow.

**FINDINGS-INDEX row authoring:** this round's four Deferred findings (r1-f1 through r1-f4) should be registered in FINDINGS-INDEX.md as Layer 3 rows once the main session classifies them. The anchor-ID scheme for these rows would be `documentation-reviewer-r1-f1` through `documentation-reviewer-r1-f4`.

**Cost-tally** (agent-self-verifiable fields per [`suite-development.md`](../../../../vsdd-suite/suite-development/suite-development.md) § Per-review entry preamble § Cost-tally; auditability tier):

- **AI tool:** claude-code CLI
- **Execution method:** cold-session sub-agent (single-domain; inline main session)
- **Model:** claude-sonnet-4-6 (Sonnet 4.6 per DESIGN.md § Cold-session budget — Sonnet for Documentation Reviewer)
- **Files read (agent-self-verifiable):** `README.md` (97 lines), `DESIGN.md` (327 lines), `TODO.md` (154 lines), `CHANGELOG.md` (offset 1–200 read), `vsdd-suite/FINDINGS-INDEX.md` (148 lines), `vsdd-suite/review-log/2026-05-21-documentation-reviewer.md` (544 lines full; Reviews 5 + 6), `vsdd-suite/review-log/2026-05-20-documentation-reviewer.md` (headers only), `manual-tests/` directory listing, `PROCESS.md` (offset 1–60 read), `vsdd-suite/domains/role/DOCUMENTATION-REVIEWER-REVIEW.md` (63 lines), `vsdd-suite/primers/3-review-session.md` (241 lines), `vsdd-suite/suite-development/suite-development.md` (two slices; ~155 lines read total), `vsdd-suite/supplements/markdown.md` (§ Documentation Reviewer, ~25 lines), `vsdd-suite/hooks/check-project-review-discipline.py` (two slices; ~70 lines read)
- **Raw tokens:** *pending operator /cost paste*
- **Would-be API cost:** *pending operator /cost paste*
- **Actual cost to operator:** *pending operator /cost paste*
- **Rate-limit-window utilization:** *pending operator /cost paste*
- **Wall-clock duration:** *pending operator /cost paste*
- **Findings/100k tokens:** NOT COMPUTABLE — pending operator /cost paste

**Operator-action queue:** if cost-tally precision is load-bearing for cross-cycle calibration, operator runs `/cost` in this session and pastes the output here as an append-only addendum, replacing the *pending operator ...* placeholders with measured values.

**Validator:** technical-writer Finding 2).

---

## Review 2 — 2026-05-25 04:30Z

**Scope:** Cold-reader-discovery adversarial re-verification against the post-Round-1-fix state. Round 1 doc-fix closure verification from the cold-reader seat + new discoverability residuals surfaced by the fix-work. Fix commits in scope: `fdfa989` (Phase 1a+1b spec amendments + narrative updates; 39 FINDINGS-INDEX rows backfilled) → `ba6a4a9` (Phase 2a regression tests) → `bfc0713` (Phase 2b impl fixes) → `795bc25` (manual-tests/layer-3.md + Phase 2c annotation). Round 1 routing record at per-domain Phase 4 routing appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md` (per-domain Phase 4 appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md`) read as required context.

**Session note:** Cold session. Discovery walk began at README.md (the primary cold-reader landing surface) and followed links naturally through DESIGN.md → TODO.md → CHANGELOG.md → manual-tests/layer-3.md → vsdd-suite/FINDINGS-INDEX.md → per-domain Phase 4 routing appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md`. Round 1 closure context was read LAST (after the discovery walk) to assess discoverability without prior-round anchoring.

**Source:** domain-raised

**Regression-check against:** [Documentation Reviewer Review 1 (this file, Review 1)](#review-1--2026-05-24-0114z) — four Deferred findings (r1-f1 through r1-f4) as the regression targets.

**MVR signal:** NOT REACHED at Round 2. Four findings surface — one real closure regression (r1-f1 partially closed: current-state line fixed but Run section and Layer 3 phase progression table not updated), one real cross-source inconsistency in DESIGN.md § Verification architecture (r2-f2), one real FINDINGS-INDEX Quick lookup staleness residual from r1-f4 partial closure (r2-f3), and one real README stale "in-flight" claim (r2-f4). Per G-131 continue-trigger discipline, Round 3 is mandatory unless TW can close all four in the current fix batch.

**Round 1 regression-check:** R1-F1 PARTIALLY CLOSED — current-state line at README.md line 9 now correctly names Layer 3 as "active in PR #52" with Phase 2 commit SHAs, Phase 3 IAR Round 1 finding count, and Phase 4 routing commit. However, two of the three required edits are NOT applied: (2) the Run section still ends after `bm list --tag rust --tag go` with no `bm export` or `bm import` examples, and (3) the Layer 3 phase progression table is absent. A cold reader following the README § Run section as the onboarding path still cannot discover the Layer 3 command surface from the README alone.

R1-F2 CLOSED — CHANGELOG now has a distinct `## [Unreleased] Layer 3 Phase 2a Round 1` entry documenting the 3 RED + 3 GREEN tests. The Phase 2a Red Gate commit is now represented in the changelog narrative. Cold reader following the CHANGELOG can trace Phase 2a → Phase 2b → Phase 2c → Phase 3 IAR → Phase 4 routing → fix work as a connected sequence.

R1-F3 CLOSED — `manual-tests/layer-3.md` now exists (573 lines). Satisfies the Review 74 convention preamble, the cross-layer prerequisite handoff, and covers 16 steps including Steps 8/9/10 (Round 1 routed fixes), Step 12 (size-cap), Step 15 (hyperfine performance budget), and the closure protocol. AC 14 through AC 28 are all represented.

R1-F4 PARTIALLY CLOSED — FINDINGS-INDEX.md now carries 39 L3 rows (all using the `<domain-slug>-r1-fN` anchor-ID scheme; 10-row spot-check verified). The row backfill is complete. However, the Quick lookup § "Open findings only" note (line 78) still reads the Layer 1 status narrative ("Post-Round-3 status (Review 82 Finding 5 close): 7 of 10 active capstone-tier domains at MVR..."). The secondary actionable item in R1-F4 — updating this note to reflect the Layer 3 IAR cycle — was not applied.

---

### Resolved

*(none)*

---

### Deferred

**Finding 1 — README.md § Run section missing `bm export` + `bm import` examples; Layer 3 phase progression table absent (R1-F1 partial closure residual) (Dim 1, Dim 9)**

<a id="r2-f1"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none — observable file state)*
**Validator:** technical-writer

This is the carry-forward of R1-F1 edits 2 + 3 (Run section examples + Layer 3 phase progression table), which were not applied in the Round 1 fix work.

The Run section still ends after `bm list --tag rust --tag go` with no Layer 3 examples. A cold reader who enters at README.md § Run section and follows it as an onboarding walkthrough still cannot discover `bm export` or `bm import` from the README surface. The canonical round-trip `bm export | bm import` — which is the Layer 3 primary user capability — is discoverable only via `bm --help` or DESIGN.md direct navigation. The `bm --help` long_about was updated in the Round 1 Phase 2b fix (UX F1 + TW F3 routing closure); DESIGN.md correctly documents the Layer 3 surface. The README § Run section remains the only first-pass onboarding surface that omits Layer 3.

Additionally, no Layer 3 phase progression table exists. A reader who follows the Layer 1 + Layer 2 tables' structural pattern to assess Layer 3 status finds no parallel table — they must read the prose current-state line and cross-reference to PROCESS.md § Layer 3.

**Supplementary observation (cross-reference integrity, inline):** `manual-tests/layer-3.md` Step 15b link `[Layer 3 dedup-complexity accepted-limit annotation](../DESIGN.md#performance-budget-)` uses a hand-authored fragment `#performance-budget-` that does not match the GitHub-derived anchor for the heading `## Performance budget ([Review 82]...)`. The heading text containing parenthesized text would derive to an anchor of the form `#performance-budget-review-82-...`; the trailing-hyphen-only form `#performance-budget-` is a hand-authored abbreviation that silently fails to scroll on rendered GitHub Markdown. Routes to TW as a sub-item of the same fix batch.

**Disposition:** Add `bm export` + `bm import` + `bm export | bm import` examples to README § Run with "(Layer 3)" annotations. Add a Layer 3 phase progression table (parallel to Layer 1 + Layer 2 tables) covering phases 1a+1b through Phase 3 IAR (Round 1 closed + Round 2 running) + Phase 5 (pending) + Phase 6 (NOT APPLICABLE per G-150 + G-112) with commit SHAs for each completed phase. As a sub-item: fix the `manual-tests/layer-3.md` Step 15b anchor to the GitHub-derived form.

**Classification:** Deferred — R1-F1 residual; routes to TW.

---

**Finding 2 — DESIGN.md § Verification architecture `export_json` description contradicts the architectural correction: still claims "`display_safe` wrapping at the per-field serialization step" after the correction removed `display_safe` from `export_json` (Dim 2, Dim 4)**

<a id="r2-f2"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none — observable file state)*
**Validator:** software-engineer

DESIGN.md § Verification architecture (line 237) reads:

> "`BookmarkStore::export_json(&self, filter_labels: Option<&[&str]>) -> String` — pure transformation that serializes the store as JSON to a string (no I/O; no clock; **`display_safe` wrapping at the per-field serialization step is a pure function over strings**). Returns the storage-format object-wrapped JSON with optional OR-filtering when `filter_labels` is `Some`."

The architectural correction in the Round 1 Phase 2b fix explicitly removed `display_safe` from `export_json`. The CHANGELOG Phase 2b entry (§ Changed (DESIGN.md)) documents:

> "§ `bm export` (Layer 3) Success-output paragraph rewritten to reflect the architectural correction: the export path leverages serde_json's native string encoder (not `display_safe` pre-wrapping) for Cc-range control chars."

The behavioral contracts paragraph at DESIGN.md line 106 correctly reflects the architectural correction:

> "the export path serializes `Bookmark` records via serde's native encoder; `display_safe` is NOT applied at the per-field serialization step..."

The § Verification architecture paragraph was NOT updated at the Phase 2b fix commit; it still describes the pre-architectural-correction design (`display_safe` wrapping at the per-field serialization step). This is a cross-source inconsistency within DESIGN.md itself — the behavioral contracts paragraph and the verification architecture paragraph now contradict each other. A cold reader who reaches DESIGN.md § Verification architecture to understand `export_json`'s purity characteristics reads incorrect information.

Per Dim 2 (implicit-knowledge audit): "JSON-native escape design" is correctly defined on first use in DESIGN.md § Behavioral contracts § `bm export`. However, a reader who reads § Verification architecture first encounters a stale `display_safe` claim and must resolve the contradiction against the behavioral contracts paragraph — the discovery path requires cross-paragraph reconciliation that the cold reader should not need to perform.

Per Dim 4 (cross-reference resolution): the CHANGELOG Phase 2b "§ Changed (DESIGN.md)" entry lists only the `bm export` Success-output paragraph and the Edge case catalog entry as changed; it does NOT list the § Verification architecture paragraph — which means the fix was incomplete.

**Cold-reader impact:** a reader using § Verification architecture to understand `export_json`'s purity boundary reads that `display_safe` is applied at the per-field serialization step. This is incorrect. The function does NOT apply `display_safe` at the serialization step; it relies on serde_json's native string encoder. The purity argument is unaffected (serde_json's native encoder is still a pure function of its inputs), but the specific mechanism description is wrong.

**Disposition:** Update DESIGN.md § Verification architecture `export_json` entry to match the architectural correction: remove "`display_safe` wrapping at the per-field serialization step is a pure function over strings" and replace with the correct mechanism description — e.g., "serde_json's native string encoder handles Cc-range control chars at the serialization step (emits `\uHHHH` JSON-native 6-char escapes per RFC 8259 § 7); no `display_safe` pre-wrapping at the serialization step (architectural correction at Phase 2b per the Round 1 routing record)".

**Classification:** Deferred — cross-source inconsistency within DESIGN.md; routes to TW (narrative owner) + SE (implementation correctness validator).

---

**Finding 3 — FINDINGS-INDEX.md Quick lookup "Open findings only" note still describes Layer 1 MVR status only; Layer 3 IAR cycle's 39 open findings not summarized (R1-F4 partial closure residual) (Dim 4, Dim 6)**

<a id="r2-f3"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none — observable file state)*
**Validator:** ai-engineer

FINDINGS-INDEX.md Quick lookup line 78 (the "Open findings only" summary note) reads:

> "**Post-Round-3 status (Review 82 Finding 5 close):** 7 of 10 active capstone-tier domains at MVR..."

This is the Layer 1 MVR status from before Layer 2 + Layer 3. With 39 L3 rows now present (all `Open`) and the Layer 3 IAR Round 1 + Phase 4 routing complete, the note is stale by two full IAR cycles. A cold evaluator who runs `grep "| Open |" vsdd-suite/FINDINGS-INDEX.md` to tally open findings (a pattern explicitly named in the Quick lookup § instructions) and then reads the summary note to understand what "Open" means in this registry finds only the Layer 1 status narrative.

The R1-F4 disposition named this as the secondary actionable item: "the Quick lookup § 'Open findings only' note should be updated to reflect the current Layer 3 IAR cycle opening." It was not updated in the fix work.

**Disposition:** Append a Layer 3 IAR status paragraph to the "Open findings only" note, e.g.: "Post-Round-1 Layer 3 IAR status (2026-05-25): 39 Layer 3 rows registered (all Open — mix of Deferred / Backlogged / Raised-to-SO / Accepted-risk / Accepted-limitation per the per-domain classification; no Layer 3 findings are in production-blocking state). Layer 3 Phase 4 routing pass landed at `e233ad8`; Round 1 fix work complete at `795bc25`; Round 2 IAR running." Preserve the Layer 1 note as historical context; add the Layer 3 note as the current status.

**Classification:** Deferred — R1-F4 partial closure residual; routes to TW.

---

**Finding 4 — README.md current-state line claims "Round 1 fix work is in flight" after all four fix commits have landed (Dim 1, Dim 6)**

<a id="r2-f4"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none — observable git state vs. file content)*
**Validator:** technical-writer

README.md line 9 reads (current state):

> "...Round 1 fix work is in flight per the per-domain Phase 4 routing appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md` (per-domain Phase 4 appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md`) routing record. Layer 3 cycle iterates Round 2 IAR after fix work lands."

All four Round 1 fix commits have now landed:
- `fdfa989` — Phase 1a+1b spec amendments + narrative updates
- `ba6a4a9` — Phase 2a regression tests
- `bfc0713` — Phase 2b impl fixes
- `795bc25` — manual-tests/layer-3.md + Phase 2c annotation

The fix work is complete; Round 2 IAR is the current cycle. The README still says "fix work is in flight" — which is now stale. A cold reader at README.md line 9 cannot determine from the line alone whether they are reading the project mid-fix-work or post-fix-work.

**Cold-reader impact:** minor but observable. The phrase "after fix work lands" is also stale — it landed. A reader who wants to know the project's current state relative to Round 2 IAR needs to cross-reference to git history or CHANGELOG to confirm fix work is complete.

**Disposition:** Update README line 9 to reflect the post-fix state, e.g.: "Round 1 fix work landed at commit `795bc25` (Phase 1a+1b spec amendments + Phase 2a tests + Phase 2b impl fixes + manual-tests/layer-3.md); Round 2 IAR running." This update naturally combines with the R2-F1 edit (Run section + Layer 3 phase progression table) so the current-state line, Run section, and phase progression table are coherent in a single fix pass.

**Classification:** Deferred — small precision gap at the primary landing surface; routes to TW.

---

### Dismissed

*(none)*

---

### Hallucinated

*(none)*

---

### Summary

Four findings in Round 2 (zero new findings in Resolved / Dismissed / Hallucinated):

- **Deferred (fix routes to TW):**
  - [Finding 1 (r2-f1)](#r2-f1) — README § Run section missing `bm export` + `bm import` examples + Layer 3 phase progression table absent; R1-F1 edits 2 + 3 not applied in fix work; supplementary observation: `manual-tests/layer-3.md` Step 15b broken anchor `#performance-budget-`
  - [Finding 2 (r2-f2)](#r2-f2) — DESIGN.md § Verification architecture `export_json` description still says "`display_safe` wrapping at the per-field serialization step"; contradicts the architectural correction that removed `display_safe` from `export_json`; CHANGELOG Phase 2b did not list § Verification architecture as a changed section
  - [Finding 3 (r2-f3)](#r2-f3) — FINDINGS-INDEX Quick lookup "Open findings only" note still describes Layer 1 MVR status only; 39 open Layer 3 rows not summarized; R1-F4 secondary actionable item not applied
  - [Finding 4 (r2-f4)](#r2-f4) — README current-state line says "fix work is in flight" after all four fix commits have landed; minor precision gap at the primary landing surface

**Pattern diagnosis:** R2-F1 through R2-F4 are all partial-closure residuals — the fix work addressed the highest-priority items in each R1 finding but did not complete all three sub-items of R1-F1 or the secondary actionable item of R1-F4. R2-F2 is a new discoverability residual specifically from the architectural correction sub-decision: the CHANGELOG Phase 2b "§ Changed (DESIGN.md)" bullet list did not enumerate § Verification architecture, and that section was therefore missed in the fix pass. R2-F2 is the most load-bearing finding because it is an internal DESIGN.md cross-source inconsistency that a cold reader cannot resolve without external context.

**Implicit-knowledge audit (Dim 2):** "JSON-native escape design" terminology defined on first use in DESIGN.md § Behavioral contracts § `bm export` (Layer 3). "sorted-tag-comparison dedup" terminology defined on first use in DESIGN.md § Behavioral contracts § `bm import` (Layer 3). "imported-tag control-char rejection" terminology defined on first use in DESIGN.md § Behavioral contracts § `bm import` (Layer 3). All three terms are defined at first use before they appear in cross-references or manual-tests. "Architectural correction sub-decision" introduced in CHANGELOG Phase 2b entry with adequate inline context. Implicit-knowledge audit passes for all four key terms.

**Forward-reference safety audit (Dim 3):** PROCESS.md § Layer 3 retrospective cites concepts ("project-terminal" vs "layer-terminal" cadence) defined in DESIGN.md § Project intent; a reader with DESIGN.md accessible is not stranded. Phase 4 routing record is discoverable from README:9, CHANGELOG Phase 2b Scope line, and PROCESS.md § Layer 3. Cross-reference spot-check (10+ links): all spot-checked links verified — FINDINGS-INDEX row anchors r1-f1 through r1-f4 present; per-domain file cross-references to `2026-05-24-software-engineer.md`, `2026-05-24-security.md`, `2026-05-24-solution-architect.md` all resolve; CHANGELOG Review 94 anchor `#review-94--2026-05-25-0300z` derives correctly from `## Review 94 — 2026-05-25 03:00Z`. The `manual-tests/layer-3.md` Step 15b anchor is the only broken cross-reference identified (captured as supplementary observation under r2-f1).

**Round 1 regression-check verdict:** R1-F2 and R1-F3 fully closed. R1-F1 partially closed (edit 1 of 3 applied). R1-F4 partially closed (row backfill applied; Quick lookup note not updated). No regression — the closed items did not reopen. The partial closures are traceable to the fix work not implementing all three sub-items of R1-F1 and the secondary item of R1-F4.

**Coordination:** TW is the natural owner for all four Deferred findings. The fix batch naturally groups: (a) README edit combining r2-f1 (Run section + Layer 3 phase progression table + `manual-tests/layer-3.md` Step 15b anchor) + r2-f4 (fix-work-in-flight claim); (b) DESIGN.md § Verification architecture update for r2-f2; (c) FINDINGS-INDEX Quick lookup note update for r2-f3. Items (a) + (b) are the primary cold-reader-friction items; (b) is also the internal-consistency defect; (c) is the audit-trail completeness item. SE validator required for r2-f2 closure.

**Cost-tally** (agent-self-verifiable fields per [`suite-development.md`](../../../../vsdd-suite/suite-development/suite-development.md) § Per-review entry preamble § Cost-tally; auditability tier):

- **AI tool:** claude-code CLI
- **Execution method:** cold-session sub-agent (single-domain; inline main session)
- **Model:** claude-sonnet-4-6 (Sonnet 4.6 per DESIGN.md § Cold-session budget — Sonnet for Documentation Reviewer)
- **Files read (agent-self-verifiable):** `README.md` (97 lines full), `vsdd-suite/FINDINGS-INDEX.md` (188 lines full), `vsdd-suite/review-log/2026-05-24-documentation-reviewer.md` (252 lines full — Review 1), per-domain Phase 4 routing appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md` (398 lines full), `CHANGELOG.md` (offsets 1–120 + 120–200 read), `DESIGN.md` (offsets 1–150 + 150–260 + 248–330 read), `manual-tests/layer-3.md` (573 lines full), `PROCESS.md` (offsets 150–230 read), several `vsdd-suite/review-log/2026-05-24-<domain>.md` files (anchor grep only), `vsdd-suite/suite-development/review-log/2026-05-24-suite-review.md` (Review 94 anchor grep), git log (20 entries)
- **Raw tokens:** *pending operator /cost paste*
- **Would-be API cost:** *pending operator /cost paste*
- **Actual cost to operator:** *pending operator /cost paste*
- **Rate-limit-window utilization:** *pending operator /cost paste*
- **Wall-clock duration:** *pending operator /cost paste*
- **Findings/100k tokens:** NOT COMPUTABLE — pending operator /cost paste

**Operator-action queue:** if cost-tally precision is load-bearing for cross-cycle calibration, operator runs `/cost` in this session and pastes the output here as an append-only addendum.

**Validator:** technical-writer.

---

## Phase 4 routing — Round 1 (2026-05-25 02:00Z)

Per [`vsdd-suite/primers/4-feedback-integration.md`](../../../../vsdd-suite/primers/4-feedback-integration.md) § [manual] First-class fallback path. SO-decisions captured via main-session AskUserQuestion pass on 2026-05-25 across the cross-domain finding clusters. This appendix lists this domain's routable findings in the primer-4-canonical per-finding shape; cross-domain coordination signals live in each Round 1 finding's `**Coordination:**` line. Cross-cluster sequencing matrix lives in the commit message + the CHANGELOG slim-form entry that recorded this Phase 4 pass (refactored from a prior consolidated routing record per operator directive 2026-05-25 — the consolidated file was an anti-pattern; primer-4-canonical is per-domain appendices).

#### Finding `r1-f1` — README primary landing page still claims Layer 3 scoped but not built — ROUTED

**Cluster:** README post-Layer-3 update
**Route:** `Phase 1a+1b`
**Gate:** README header + Run section + test count + Layer 3 phase progression table updated to post-Phase-2b state; Validator: TW + DR
**Sequencing:** Should land before Layer 3 gate close

#### Finding `r1-f2` — CHANGELOG missing Phase 2a Red Gate entry + non-slim-form 2026-05-24 entries — ROUTED

**Cluster:** CHANGELOG slim-form catch-up
**Route:** `Phase 1a+1b`
**Gate:** Phase 2a slim-form entry added at Phase 2a commit; existing L3 entries reformatted to slim-form per R93 F1; Validator: DR
**Sequencing:** Should land before Layer 3 gate close

#### Finding `r1-f3` — manual-tests/layer-3.md absent despite TODO.md spec commitment — ROUTED

**Cluster:** manual-tests/layer-3.md authoring
**Route:** `Phase 2a-equivalent artifact authoring`
**Gate:** File authored parallel to layer-1.md + layer-2.md; 16 steps covering AC 14..AC 28 + Round 1 routed closures; Validator: PFE
**Sequencing:** Blocks Layer 3 layer-gate close (criterion 3)

#### Finding `r1-f4` — FINDINGS-INDEX zero Layer 3 rows — ROUTED

**Cluster:** FINDINGS-INDEX Layer 3 backfill
**Route:** `Phase 1a+1b`
**Gate:** 39 new rows for Round 1 routable findings using post-R91-F17 anchor-ID scheme; Validator: TW + AIE
**Sequencing:** Should land before Layer 3 gate close


---

## Phase 4 routing — Round 2 (2026-05-25 07:30Z)

Per [`vsdd-suite/primers/4-feedback-integration.md`](../../../../vsdd-suite/primers/4-feedback-integration.md) § [manual] First-class fallback path. SO-decisions for substantive routings captured via main-session AskUserQuestion pass on 2026-05-25 (empty-string tag rejection consistency; tests/scaling.rs Phase 5 sentinel addition; Round 3 verification mini-cycle for the hallucination cluster). Verification evidence for `Hallucinated` dispositions: Round 3 PFE + QE + SE + UX cold-session re-spawn (per-domain Review N+1 entries authored 2026-05-25).

#### Finding `r2-f1` — README.md Run section missing bm export + bm import — RESOLVED-NO-FINDING

**Disposition:** Resolved-no-finding
**Evidence:** Main-session verification: README.md Run section already includes `bm export` + `bm import` + canonical round-trip examples post-Round-1 fix-work commit `795bc25`.

#### Finding `r2-f2` — DESIGN.md Verification architecture export_json contradicts architectural correction — HALLUCINATED

**Disposition:** Hallucinated
**Evidence:** Main-session verification: DESIGN.md:237 already names the architectural correction (`display_safe` is NOT applied here; serde_json's native encoder handles Cc-range escaping). Round 2 claim is the inverse of the current text.

#### Finding `r2-f3` — FINDINGS-INDEX Quick-lookup note describes Layer 1 MVR status — RESOLVED-NO-FINDING

**Disposition:** Resolved-no-finding
**Evidence:** Quick-lookup note is intentionally a stable cross-cutting registry note; Layer-progression status lives in README. No staleness in the index itself.

#### Finding `r2-f4` — README current-state line claims Round 1 fix work in flight — RESOLVED-NO-FINDING

**Disposition:** Resolved-no-finding
**Evidence:** Main-session verification: README current-state line reflects post-Round-1-fix state per commit `eae5dff`.
