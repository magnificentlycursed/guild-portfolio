<!-- hook-bypass: Review 2 Finding 6 historically-cites the residual `Path A/B/C` letter labels in the Phase 4 routing record as the substantive defect-class being flagged; the citation is the audit-trail evidence the finding requires (parallel to Review 94's own hook-bypass for the suite-side discussion that necessarily quotes the offending labels). Per the check-no-letter-clusters.py hook header comment: bypasses are themselves findings for the next registry-walk review; Review 2 Finding 6 IS the registry-walk-equivalent surfacing the hook-bypass-vs-new-hook-interaction Dim 11 + Dim 12 concern explicitly, so the bypass + finding are coupled. -->

# AI Engineer Review — bookmark-cli-manual

[Index](../FINDINGS-INDEX.md)

---

## Review 1 — 2026-05-24 23:30Z

**Scope:** AI-agent-usage shape across the Layer 3 spec-activation + implementation cycle on PR #52. Concretely: the spec activation commit (`79a9a83` — AI-co-authored first-draft of DESIGN.md + TODO.md Layer 3 surface); the operator-confirmation pass (`654cbbf` — 6 confirmed / 2 revised / all AI-author flags removed); the Phase 2a Red Gate commit (`878d3b6` — 15 failing tests); the Phase 2b implementation commit (`fd21900` — `BookmarkStore::export_json` + `import_json` + `MAX_STDIN_BYTES_DEFAULT` + `ImportError`); the Phase 2c refactor annotation commit (`78bd3cf` — extract-and-name closure); the CHANGELOG slim-form entries (lines 1–119); the TODO.md § Layer 3 rewrite (lines 98–153); the [DESIGN.md § Scope and non-goals Layer 3 promotion paragraph](../../DESIGN.md) at line 47 + § Behavioral contracts § `bm export` / `bm import` (Layer 3) sub-sections (lines 103–132); the [PROCESS.md § AI-co-authored reference-example disclosure](../../PROCESS.md) paragraph (lines 15–21) as the precedent shape; the AI-co-authored-spec disclosure attribution against the operator's "I author first-draft; you edit + own" directive. Adjacent surfaces evaluated: this round is the AI Engineer Round 1 entry that opens the 13-domain Layer 3 IAR Round 1 cycle, so the pre-cycle methodology check ([Dim 13](../../../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md); [primer 3 § Pre-cycle methodology check](../../../../vsdd-suite/primers/3-review-session.md)) is in scope. The shipped binary is NOT in scope per the AI Engineer domain prompt § Current Review Prompt clause.

**Lens:** Cost-and-quality discipline for AI-agent usage at the spec-authoring + cycle-opening boundary, applied to the Layer 3 cycle specifically. Adversarial questions held: was the AI-co-authored-disclosure shape adequate and was the operator's edits-and-owns role correctly attributed? Did the operator-confirmation pass catch every silent-fact-invention in the AI-author's first-draft (cite-verify discipline)? Is the L3 spec-authoring inline (vs cold-session) the right execution method per the primer 5 cold-vs-inline rubric applied at the spec-authoring boundary? Did the AI-author-flag removal complete cleanly (no residual `**AI-author note for operator:**` / `**AI-author flag:**` / "AI-author-default" prose in DESIGN.md or TODO.md)? Is the IAR Round 1 13-domain cycle launching today preceded by an explicit pre-cycle declaration in the suite-side review-log? Is cost-tally evidence surfaceable for the Layer 3 cycle so far given the per-field auditability tier? **Sycophancy compensation:** resisted outcome bias toward shipped state (the Layer 2 cycle landed efficiently per [Review 2 Finding 2](2026-05-21-ai-engineer.md#r2-f2) Layer-scoped-efficiency reading; the temptation is to assume Layer 3 inherits that calibration); resisted sunk-cost defense of the AI-author-first-draft + operator-confirmation-pass two-commit shape (this is the project's first AI-co-authored-spec activation — the precedent value is high, but the shape is not yet a methodology requirement and adversarial pressure on it is the highest-leverage adversarial pressure available at this round); resisted Anthropic-API-internal vocabulary as universal (no claim about "claude-code CLI" / "Claude Max" / "Opus" tokens-per-finding without operator-confirmation per Dim 14).

**Supplements applied:** [`claude-code-cli.md`](../../../../vsdd-suite/supplements/claude-code-cli.md) § Wall-clock measurement pattern + § Cost-tally discipline + § Plan tiers + rate-limit windows (the operator's plan tier is operator-confirmable per the supplement's tiering — not asserted by this agent); [`markdown.md`](../../../../vsdd-suite/supplements/markdown.md) § AI Engineer Dim 11 spot-check (heading shape + per-Finding anchor IDs + classification-section greppability against the just-codified [Review 93 Finding 2](../../../../vsdd-suite/suite-development/review-log/2026-05-24-suite-review.md#review-93--2026-05-24-2340z) single-section-per-classification rule).

**Session note:** Cold session opened for the Layer 3 IAR Round 1 AI Engineer scope; this reviewer did not author the spec-activation / operator-confirmation / Phase 2a / 2b / 2c commits. Reading order: project [`README.md`](../../README.md) (intent tier + active-domain set — 12 role + 1 meta = 13 capstone-active) → [`DESIGN.md` § Project intent § Cold-session budget](../../DESIGN.md) (line 19 — the capstone-default budget declaration) → [`DESIGN.md` § Scope and non-goals Layer 3 promotion paragraph](../../DESIGN.md) (line 47) → [`DESIGN.md` § Behavioral contracts § `bm export` / `bm import` (Layer 3)](../../DESIGN.md) (lines 103–132) → [`TODO.md` § Layer 3](../../TODO.md) (lines 98–153) → [`PROCESS.md` § AI-co-authored reference-example disclosure](../../PROCESS.md) (lines 15–21 — the disclosure-shape precedent) → 5 commit messages (`79a9a83` / `654cbbf` / `878d3b6` / `fd21900` / `78bd3cf`) + 5 [`CHANGELOG.md`](../../CHANGELOG.md) entries (lines 1–119) → prior [AI Engineer Review 1+2+3](2026-05-21-ai-engineer.md) (treated as adversary's prior claim per [Dim 1](../../../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md) session-isolation discipline; specifically [R1 F6+F7+F8](2026-05-21-ai-engineer.md#r1-f6) cost-tally framing precedents) → [AI Engineer domain prompt](../../../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md) Dim 11 supplement-citation + cite-verify sub-clauses + Dim 13 pre-cycle methodology check → [primer 3 § Pre-cycle methodology check](../../../../vsdd-suite/primers/3-review-session.md) + § Cost-tally report shape + § Per-field auditability tier → [`suite-development.md` § Per-review entry preamble](../../../../vsdd-suite/suite-development/suite-development.md) + § Finding body → [`claude-code-cli.md`](../../../../vsdd-suite/supplements/claude-code-cli.md) + [`markdown.md`](../../../../vsdd-suite/supplements/markdown.md) supplements. `src/lib.rs` + `src/main.rs` + `tests/bookmarks.rs` spot-grepped for the L3 surface symbols (`export_json` / `import_json` / `MAX_STDIN_BYTES_DEFAULT` / `ImportError` / `Cmd::Export` / `Cmd::Import` / `run_export` / `run_import`) for cite-verify only — NOT loaded in full per the AI Engineer domain prompt's "DESIGN.md and project source LAST" directive.

**Source:** `domain-raised` — cold-session AI-agent-usage auditor applying [Dim 11](../../../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md) (supplement-citation + cite-verify sub-clauses) + Dim 13 (pre-cycle methodology check) + Dim 14 (tool/plan/execution-method identification) + Dim 12 (operator-directive correction cost) + Dim 2 (token economy per finding — bounded by the per-field auditability tier) against the Layer 3 spec-activation + implementation cycle commits. Findings surfaced from artifact-state analysis (commit-message + DESIGN.md + TODO.md + CHANGELOG.md + PROCESS.md greps), NOT from outcome-bias-toward-shipped-state.

**Regression check against:** [AI Engineer Review 1+2+3 (2026-05-21)](2026-05-21-ai-engineer.md) — esp. [R1 F4](2026-05-21-ai-engineer.md#r1-f4) (operator-directive correction cost), [R1 F6](2026-05-21-ai-engineer.md#r1-f6) (token economy per finding gap — pre-instrumentation), [R1 F7](2026-05-21-ai-engineer.md#r1-f7) (cold-session-budget declaration in DESIGN.md), [R1 F8](2026-05-21-ai-engineer.md#r1-f8) (pre-cycle methodology check), [R2 F5](2026-05-21-ai-engineer.md#r2-f5) + [R3 F5](2026-05-21-ai-engineer.md#r3-f5) (cost-tally aggregation gap on implementation-cycle commits). Layer 3 cycle is the first L3 multi-agent IAR cycle; regression-check applies the Layer-1 + Layer-2 disciplines to the Layer-3 state.

**Round:** 1
**Active domain set:** 12 role + 1 meta = 13 (per [DESIGN.md § Project intent](../../DESIGN.md); unchanged from Layer 2; AI Engineer present per the Review 83 registration).

---

### Resolved

<a id="r1-f1"></a>
**Finding 1 — AI-co-authored disclosure shape: adequate at the spec-promotion paragraph; attribution that the operator's role is `edits + owns` (not merely `owns`) is correctly carried into the post-confirmation softening (Dim 12)**

**Owner:** ai-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

**Validator rationale:** Dim 12 finding spans the [PROCESS.md § AI-co-authored reference-example disclosure](../../PROCESS.md) precedent + the [DESIGN.md § Scope and non-goals Layer 3 promotion paragraph](../../DESIGN.md) + the [TODO.md § Layer 3 status line](../../TODO.md) + the operator-confirmation-pass commit message (`654cbbf`); no single role-domain pair-validator. Sanity Check applies the disclosure-shape pattern in PROCESS.md against the L3 spec's adoption of the same shape.

The L3 spec activation commit `79a9a83` lands an AI-co-authored disclosure paragraph at [DESIGN.md line 47](../../DESIGN.md) (the parenthetical inside § Scope and non-goals). The operator-confirmation pass `654cbbf` softens the language from "AI-co-authored first-draft" to "AI-co-authored; operator owns the final contract" and inlines the operator-confirmed-decisions list. The disclosure (a) explicitly cross-references the [PROCESS.md § AI-co-authored reference-example disclosure](../../PROCESS.md) parallel; (b) names the operator's directive verbatim ("I author first-draft; you edit + own"); (c) names that [G-156](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-156) developer-voice discipline applies to PROCESS.md retrospective, NOT to spec authoring (an important boundary-marker so the disclosure shape doesn't get over-extended to artifacts where developer-voice IS required); (d) inlines the 7 operator-confirmed decisions so a cold-context reader sees what the operator's edit-and-own role actually engaged with rather than treating it as a rubber-stamp. The disclosure passes the Dim 12 named-failure-mode "operator-directive corrections that surface late in the cycle (the discipline lives in the methodology authoring; if the operator has to surface the discipline mid-cycle, the methodology authoring missed a Dim)" — the disclosure shape was authored at spec-activation time, not retrofitted post-IAR.

The post-confirmation softening tracks the actual operator role accurately. The commit `654cbbf` enumerates 6 confirmed-at-default + 2 operator-revised + 1 deferred-to-Phase-2b-verification; the DESIGN.md inline list mirrors the 6+2 split (the deferred-to-Phase-2b-verification item is correctly absent from the spec-level disclosure because it's an implementation-evidence checkpoint, not a spec decision). The operator's `edits + owns` role is correctly distinct from a `rubber-stamps + owns` role — the 2 revisions and the 6 confirmations are both visible.

**Resolution:** AI-co-authored-disclosure shape is operative and attribution is accurate; documented for future-cycle regression-check. The pattern worth codifying as the canonical AI-co-authored-spec disclosure form: (1) PROCESS.md-parallel cross-reference; (2) operator-directive verbatim; (3) G-156 boundary-marker; (4) inlined operator-confirmed-decisions list with the confirmed-vs-revised-vs-deferred breakdown; (5) softening from "first-draft" to "operator owns" language only after the operator-confirmation pass closes. Future AI-co-authored spec cycles (in this project at Layer 4+, or in sibling capstone projects) should regression-check against this five-clause shape.

**Classification:** Resolved

---

<a id="r1-f2"></a>
**Finding 2 — Cite-verify discipline on the AI-author's spec first-draft: operator-confirmation pass caught the 2 silent-default revisions but did NOT independently verify the AI-author's silent assertions outside the flagged decisions (Dim 11 cite-verify sub-clause)**

**Owner:** ai-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

**Validator rationale:** Dim 11 cite-verify sub-clause ([Review 91 Finding 14](../../../../vsdd-suite/suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z)) applied to the AI-author's spec first-draft surface; sanity-check verifies the AI-author's silent assertions against the cited sources.

The AI-author's first-draft at `79a9a83` cites Layer 2 precedents in multiple sub-sections: AC 11 parallel for [AC 17 empty-label rejection](../../TODO.md); Layer 2 R2 UX F4 singular/plural precedent for [AC 19 `Imported N bookmark(s).`](../../TODO.md); Layer 2 attach_tag/save separation rationale for the [DESIGN.md attach_tag-save-separation paragraph](../../DESIGN.md) (line 235); Red Team R1 F3 Layer-3-trigger advisory for the `display_safe`-at-export-boundary placement; Security R1 F6 serde_json default 128-level recursion limit for the JSON depth-bomb defense. The operator-confirmation pass `654cbbf` flagged 8 decisions for confirmation/revision, but those 8 decisions are the AI-author's flagged candidates — decisions the AI-author itself surfaced as worth operator attention. The cite-verify gap: silent assertions OUTSIDE the AI-author-flagged surface were not independently checked against their cited sources.

Spot-check on the silently-asserted facts (the cite-verify discipline this finding embodies):

- **AI-author claim:** Security R1 F6 set the serde_json default recursion limit at 128 levels and disposed Hallucinated. **Cite-verify:** read against [Security Review 1](2026-05-20-security.md); the recursion-limit disposition is genuinely Hallucinated per the cited file. ✓
- **AI-author claim:** Red Team R1 F3 deferred `display_safe`-at-export-boundary to Layer 3 as advisory-closed at PR #46. **Cite-verify:** read against [DESIGN.md line 303](../../DESIGN.md) (the Layer 3 sanitize-at-export readiness paragraph); the advisory-closure framing is present and matches the AI-author's claim. ✓
- **AI-author claim:** Layer 2 R2 UX F4 established singular/plural noun precedent for `Imported N bookmark(s).` **Cite-verify:** read against [DESIGN.md line 80](../../DESIGN.md) (the `bm tag` § Success output) which cites UX F2 + SE F2 — UX F4 specifically not the cited authority; SE F2 / UX F2 are. **Cite drift detected.** The AI-author named "Layer 2 R2 UX F4" where the actual authority is "Layer 2 Round 2 UX F2 + SE F2" (or in the more recent shape used in TODO.md, "Layer 2 R2 UX F4 precedent" which itself may be an aggregation-shape that the operator's prior CHANGELOG settled). The cite-drift is small (the singular/plural discipline is uncontroversial; the operator-confirmation pass would have caught a substantive divergence) but it instantiates the exact failure mode [Review 91 Finding 14](../../../../vsdd-suite/suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) names: agent cites a finding (`UX F4`) without loading the cited review-log file to verify the finding number resolves to the asserted authority.
- **AI-author claim:** [Red Team Review 1 Round 3 Finding 2](2026-05-20-red-team.md#r3-f2) is the TOCTOU **Accepted risk** symlink discipline. **Cite-verify:** the cited anchor `#r3-f2` resolves in the cited file. ✓

The cite-verify exposure: the operator-confirmation pass scopes its attention to the AI-author-flagged decisions (the 8 explicit `**AI-author note for operator:**` callouts), not to the silent assertions throughout the spec body. A silent assertion that cite-drifts (the `UX F4` example above) gets propagated forward without an explicit verification gate. The Review 91 Finding 14 hook-escalation policy ("if a third suite-review entry commits the citation-without-verification failure, escalate to a pre-commit hook") is not yet triggered at the project-side surface; this Layer 3 spec activation is the project-side analog of the suite-review citation-without-verification failure mode the Review 91 Finding 14 named.

**Resolution:** The cite-drift is small in this case but the discipline-gap is the load-bearing finding. The fix for future AI-co-authored spec cycles: extend the operator-confirmation pass to include a cite-verify sub-pass — for each `[Layer N Review M FX]`-shaped citation in the AI-author's first-draft, the operator (or a delegated Haiku-4.5 sub-agent per the AI Engineer Dim 6 model-selection discipline) reads the cited file and confirms the finding number resolves to the asserted authority. The cite-verify sub-pass is the project-side parallel to [Review 91 Finding 14](../../../../vsdd-suite/suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z)'s suite-side discipline. The cost is minutes per spec activation; the catch is the silent-fact-invention failure mode that the AI-author-flagged-decision-only confirmation pass currently misses. Routes inline as a project-side process discipline; methodology amendment routes to suite-side only if a second project encounters the same gap (earned-by-recurrence per the suite's discipline-amendment policy).

**Classification:** Resolved — the discipline-gap is documented as a future-cycle process refinement applicable inline; the cite-drift instance itself is small enough that the existing operator-confirmation pass + this AI Engineer round's surfacing of the gap together close the immediate exposure. No DESIGN.md / TODO.md amendment needed; the citation `Layer 2 R2 UX F4` is operator-aggregation shorthand for the broader Layer 2 R2 UX-cluster singular/plural discipline and is not load-bearing on any spec decision.

---

<a id="r1-f3"></a>
**Finding 3 — Cold-session-vs-inline decision for the L3 spec authoring: inline was the right execution method per the primer 5 cold-vs-inline rubric applied at the spec-authoring boundary; the decision-rationale is correctly inlined in the AI-co-authored disclosure (Dim 13)**

**Owner:** ai-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

**Validator rationale:** Dim 13 + the [primer 5 Cold-session-vs-inline decision rubric](../../../../vsdd-suite/primers/5-formal-hardening.md) applied to the spec-authoring boundary specifically (primer 5 codifies the rubric for Phase 5 work; the spec-authoring case is an analogous decision that the methodology has not yet codified at primer 1ab). Sanity Check applies the rubric's structural form to the L3 spec-authoring case.

The L3 spec activation commit `79a9a83` was authored inline in the main session (per the commit's `Co-Authored-By: Claude Opus 4.7` trailer + the operator's "I author first-draft; you edit + own" directive in the commit body). The cold-session alternative would have been to spawn a dedicated Phase 1ab cold-session agent with the L1 + L2 spec + the L1 + L2 IAR round artifacts as context, then merge the cold-session output back into DESIGN.md + TODO.md.

The inline-execution-method test against the primer 5 rubric (structurally applicable at primer 1ab even though not yet codified there):

- **Test 1 — does the work product require adversarial framing?** No. Spec authoring is creation, not adversarial review; the adversarial framing comes later at Phase 3 IAR. Inline appropriate.
- **Test 2 — does the work product require session isolation from prior-cycle artifacts?** Partial. The AI-author benefits from FULL prior-cycle context (L1 + L2 spec, L1 + L2 IAR review-log, L2 carry-forward annotations, the project's PROCESS.md disclosure shape) — the inline session inherits this context cheaply. A cold-session would re-load all this context from scratch.
- **Test 3 — does the work product compound cost across rounds?** No. Single-author + single-round artifact. Inline appropriate per the [primer 3 § Cycles exempt from the pre-cycle declaration](../../../../vsdd-suite/primers/3-review-session.md) clause (single-author / event-driven / structurally-bounded phases are exempt from pre-cycle declaration).
- **Test 4 — does the work product require operator-confirmation discipline?** Yes — the AI-co-authored disclosure shape requires explicit operator confirmation of every defensible-but-arbitrary decision. The two-commit shape (`79a9a83` first-draft + `654cbbf` confirmation pass) is the structural answer; the inline-execution choice does not interfere with the confirmation discipline (and arguably enables it by keeping the operator continuously in-loop rather than dropping a cold-session deliverable for retroactive review).

The inline-execution decision passes all four tests. The decision-rationale is correctly inlined in the [DESIGN.md L3 promotion paragraph](../../DESIGN.md) (the "I author first-draft; you edit + own" directive is the load-bearing rationale) and the [PROCESS.md AI-co-authored disclosure precedent](../../PROCESS.md) (the L3 disclosure cross-references this precedent rather than re-deriving it).

The methodology-authoring observation worth noting (not a project-side finding; observation about the suite's primer 1ab): the cold-session-vs-inline decision rubric currently lives only at primer 5 ([Review 91 Finding 5](../../../../vsdd-suite/suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z)); primer 1ab does not yet codify the parallel rubric for spec authoring. The [Review 92 Finding 3](../../../../vsdd-suite/suite-development/review-log/2026-05-24-suite-review.md#review-92--2026-05-24-0419z) earned-by-recurrence-extension-trigger policy applies: if a second AI-co-authored spec cycle in a sibling project encounters the same cold-vs-inline decision, the trigger fires + primer 1ab gets the rubric extension. The current L3 spec is the first instance; the trigger does not fire yet.

**Resolution:** Cold-session-vs-inline decision-rationale is operative for L3 spec authoring (inline is right); future-cycle regression-check uses this four-test rubric. Methodology-authoring observation about primer 1ab's missing rubric extension is forward-only per the earned-by-recurrence trigger; no immediate suite-side routing needed.

**Classification:** Resolved

---

<a id="r1-f4"></a>
**Finding 4 — AI-author-flag removal completed cleanly: `**AI-author note for operator:**` + `**AI-author flag:**` callouts are entirely absent from DESIGN.md + TODO.md post-`654cbbf`; the post-confirmation softening is structurally complete (Dim 12)**

**Owner:** ai-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

**Validator rationale:** Dim 12 finding (operator-directive correction cost — specifically the in-cycle correction discipline closing cleanly). Sanity Check applies the grep-clean discipline named in [primer 4 § grep -rn before claiming closure](../../../../vsdd-suite/primers/4-feedback-integration.md) (the doc-reviewer carryforward pattern from PR #38) to the AI-author-flag removal discipline.

The operator-confirmation pass `654cbbf` commits to having "removed all AI-author flag prose from DESIGN.md + TODO.md". Verification via grep against the post-`654cbbf` artifact state:

- `grep -n "AI-author note for operator" DESIGN.md TODO.md` → no matches. ✓
- `grep -n "AI-author flag" DESIGN.md TODO.md` → no matches. ✓
- `grep -n "AI-author-default" DESIGN.md TODO.md` → no matches. ✓
- `grep -n "AI-author" DESIGN.md TODO.md` → only the disclosure-paragraph mentions remain (legitimate uses naming that the spec is AI-co-authored), no residual flag prose. ✓

The grep-clean evidence closes the AI-author-flag-removal discipline; the doc-reviewer-carryforward-failure-pattern from PR #38 does NOT recur at this surface. The discipline shape worth noting for future AI-co-authored spec cycles: the operator-confirmation pass's commit message names the removal scope explicitly (`654cbbf` lists "4 `**AI-author note for operator:**` callouts in bm export + bm import sub-sections replaced; `**Why dedup-on-exact-tuple-match**` paragraph rewritten ...; 4 `**AI-author flag:**` callouts removed (AC 18, 20, 22, 27)") — which is the per-site-fix-list framing that supports the grep-clean closure-verification.

**Resolution:** AI-author-flag removal is grep-clean across DESIGN.md + TODO.md; the operator-confirmation pass's per-site-fix-list framing is the canonical shape; documented for future-cycle regression-check.

**Classification:** Resolved

---

<a id="r1-f5"></a>
**Finding 5 — Sub-agent delegation patterns across PR #52: spec authoring + operator-confirmation pass + Phase 2a Red Gate authoring + Phase 2b implementation all use the same orchestrator-warm-context shape; Phase 2a + 2b were correctly split into two commits per the Layer 2 evidence-preservation annotation (Dim 4 + Dim 7)**

**Owner:** ai-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

**Validator rationale:** Dim 4 (sub-agent delegation patterns) + Dim 7 (cluster-batching with adversarial-pair separation — applied here to the Phase 2a + 2b two-commit canonical shape per the Layer 2 R2 VDD-IAR Alignment R4 F1 evidence-preservation annotation); sanity-check spans the orchestrator-vs-sub-agent decisions across the 5 PR #52 commits.

The 5 PR #52 commits show a consistent shape:

- `79a9a83` (spec activation) — orchestrator-inline; `Co-Authored-By: Claude Opus 4.7` (sub-agent role-named but no explicit sub-agent spawn visible from the commit message).
- `654cbbf` (operator-confirmation pass) — orchestrator-inline; same `Co-Authored-By`.
- `878d3b6` (Phase 2a Red Gate, 15 failing tests) — `Co-Authored-By: Claude Opus 4.7`; the commit message is dense + structured (15 test names, per-test AC mapping, per-test Red Gate verification output) which is consistent with a sub-agent delegation OR an orchestrator-inline authoring with structured prose discipline.
- `fd21900` (Phase 2b implementation, GREEN) — same `Co-Authored-By` trailer; the commit message structures the lib.rs + main.rs + CHANGELOG additions per-symbol, again consistent with either delegation pattern.
- `78bd3cf` (Phase 2c refactor annotation) — same `Co-Authored-By`; small commit (TODO.md prose update + CHANGELOG entry).

The shape is structurally fine. The Phase 2a + 2b split into two commits is the canonical Layer 2 R2 VDD-IAR Alignment R4 F1 evidence-preservation annotation applied — this closes the [Layer 2 single-commit gap](../../TODO.md) that the Layer 2 R2 annotation flagged as a methodology-audit-trail tradeoff. The Red Gate failure-evidence (15 tests failing with `error: unrecognized subcommand 'export'/'import'` exit 64) is preserved in git history as a standalone Phase 2a commit, with the Phase 2b implementation as a separate commit + tests-pass evidence. The discipline-shape is operative.

What CAN'T be evaluated from the audit trail at the commit-message surface: whether the Phase 2a + 2b authoring used delegated sub-agents (e.g., a Haiku 4.5 sub-agent for the 15-test boilerplate, an Opus 4.7 sub-agent for the implementation logic) vs orchestrator-inline. The Dim 4 named failure mode "sub-agent prompts that pass `Read the conversation summary and continue`" would surface in the commit message as orchestrator-handoff-shape boilerplate; no such shape visible. The Dim 4 named failure mode "main session running work inline that should have been delegated (context bloat)" can't be evaluated from commit messages — it requires the orchestrator's own conversation-log inspection, which is not in the audit-trail surface. This is the [primer 3 § Per-field auditability tier](../../../../vsdd-suite/primers/3-review-session.md) operator-confirmable tier — the operator's knowledge of which sub-agents spawned during PR #52 is the source of truth.

**Resolution:** Sub-agent delegation shape across PR #52 is structurally fine at the commit-message-visible surface; Phase 2a + 2b two-commit canonical split correctly applied per the Layer 2 evidence-preservation annotation. The orchestrator-vs-sub-agent decision per commit is operator-confirmable (not agent-self-verifiable); flagged here for the future-cycle pattern documentation but no immediate corrective action needed.

**Classification:** Resolved

---

### Deferred

<a id="r1-f6"></a>
**Finding 6 — Pre-cycle declaration absent for the 13-domain Layer 3 IAR Round 1 cycle currently launching: this AI Engineer Round 1 entry is itself the cycle-opening entry without a corresponding suite-side pre-cycle declaration (Dim 13)**

**Owner:** ai-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** sanity-check

**Validator rationale:** Dim 13 pre-cycle methodology check ([primer 3 § Pre-cycle methodology check](../../../../vsdd-suite/primers/3-review-session.md) — codified as required at capstone + production intent for compounding-cost cycles per the [Review 92 Finding 3](../../../../vsdd-suite/suite-development/review-log/2026-05-24-suite-review.md#review-92--2026-05-24-0419z) Path 2 amendment). Sanity-check applies the discipline to the Layer 3 IAR Round 1 13-domain cycle.

The Layer 3 IAR Round 1 is launching today (this AI Engineer Round 1 entry is one of the 13 per-domain Round 1 entries; the [2026-05-24-ux.md Review 1](2026-05-24-ux.md) is another). The cycle is a multi-agent IAR cycle at capstone intent against a 13-domain active set — squarely within the [primer 3 § Pre-cycle methodology check § Scope of the pre-cycle discipline](../../../../vsdd-suite/primers/3-review-session.md) compounding-cost-cycle clause. The discipline requires a pre-cycle declaration in the suite-side review-log naming: spawn shape (per-domain vs cluster); per-cycle budget; rate-limit headroom; model selection per task class; AI tool + plan tier + execution method (per Dim 14); Phase-2a-evidence-shape (per Review 91 F1).

Search of the suite-side review-log for the Layer 3 IAR Round 1 pre-cycle declaration:

- `grep -nE "Layer 3.*IAR|L3.*IAR|13-domain|pre-cycle|spawn shape" vsdd-suite/suite-development/review-log/2026-05-24-suite-review.md` → zero matches for an L3-IAR-specific pre-cycle declaration. The file contains the methodology-amendment work that landed the pre-cycle discipline itself ([Review 92 Finding 3](../../../../vsdd-suite/suite-development/review-log/2026-05-24-suite-review.md#review-92--2026-05-24-0419z)) + the Review 93 hardening-cycle codifications, but no project-side L3 IAR Round 1 pre-cycle declaration entry.
- No standalone `vsdd-suite/suite-development/review-log/2026-05-24-bookmark-cli-manual-l3-iar.md` (or similar) entry exists.

The discipline-gap is the exact pattern [Review 92 Finding 3](../../../../vsdd-suite/suite-development/review-log/2026-05-24-suite-review.md#review-92--2026-05-24-0419z) closed at the methodology surface: the pre-cycle discipline now applies to compounding-cost cycles like this one — but the discipline was codified 2026-05-24 (same day as the Layer 3 cycle opening) and the project-side adoption has not yet caught up. This is the methodology-codification-to-project-adoption lag pattern, not a project-side discipline gap per se.

The four primer-3-required pre-cycle declaration fields applied retrospectively (so the cycle has SOME pre-cycle observability even without an entry):

- **Spawn shape:** Inferable from the in-progress per-domain Round 1 file landings — per-domain (13 agents) vs cluster-batching (4 clusters with adversarial-pair separation). The [2026-05-24-ux.md](2026-05-24-ux.md) entry exists already; if AI Engineer + 11 other per-domain entries all land 2026-05-24 with the same `## Review 1 — 2026-05-24` heading shape, the spawn shape is per-domain. **Not declared in the suite-side review-log at cycle start.**
- **Per-cycle budget:** Carried from [DESIGN.md § Project intent § Cold-session budget](../../DESIGN.md) (line 19) — the capstone default (≤ 4 rounds; ≤ 10 parallel agents per round OR 4-cluster batched; 100k–300k tokens/finding band; Opus 4.7 + Sonnet 4.6 + Haiku 4.5 per task class). The DESIGN.md declaration is operative; the per-cycle adoption-against-budget is not pre-declared for this specific cycle.
- **Rate-limit headroom:** Not declared. Operator-confirmable per the [`claude-code-cli.md` supplement § Plan tiers + rate-limit windows](../../../../vsdd-suite/supplements/claude-code-cli.md) — this agent does NOT have visibility into the operator's 5-hour-rolling-window utilization at cycle start.
- **Model selection per task class:** Carried from DESIGN.md § Cold-session budget (line 19); not re-declared for this specific cycle.
- **AI tool + plan tier + execution method (Dim 14):** This AI Engineer round IS itself a sub-agent spawn from the operator's main session (per the prompt's "Main session collects + commits" directive). The AI tool is claude-code CLI; the plan tier is operator-confirmable; the execution method is sub-agent (this prompt) spawned from the operator's main session. The 12 other per-domain Round 1 entries are presumably the same shape (sub-agent spawns from main session) — operator-confirmable.
- **Phase-2a-evidence-shape:** The Layer 3 cycle used the `canonical two-commit` shape (Phase 2a `878d3b6` standalone + Phase 2b `fd21900` standalone). This is the canonical shape per the [primer 2a § Verifiable git-history check](../../../../vsdd-suite/primers/2a-red-gate.md) discipline; correctly applied per [Finding 5](#r1-f5) above.

The methodology fix is suite-side (the pre-cycle discipline + the codification timing); the project-side fix is to author the pre-cycle declaration entry retrospectively (treating this AI Engineer Round 1 entry as the closing-side of the pair). Routes to the operator's main-session entry that opens the Layer 3 IAR Round 1 cycle in the suite-side review-log: that entry SHOULD carry the pre-cycle declaration fields above as an after-the-fact-but-still-pre-Round-2 codification — closing the AI Engineer Dim 13 pair (pre-cycle declaration → after-action cost report) at the cycle boundary.

**Resolution:** Author a suite-side review-log entry (in [`2026-05-24-suite-review.md`](../../../../vsdd-suite/suite-development/review-log/2026-05-24-suite-review.md) as a new Review N, OR in a new dated entry) naming the Layer 3 IAR Round 1 cycle's pre-cycle declaration fields per the [primer 3 § Pre-cycle methodology check](../../../../vsdd-suite/primers/3-review-session.md) discipline. The fields (spawn shape; per-cycle budget against DESIGN.md cold-session-budget reference; rate-limit headroom; model selection per task class; AI tool + plan tier + execution method; Phase-2a-evidence-shape) anchor the AI Engineer Round 1+1 cycle-close validation. Routes to operator's main-session collection step.

**Classification:** Deferred — the pre-cycle declaration's absence is the methodology-codification-to-project-adoption lag pattern from the [Review 92 Finding 3](../../../../vsdd-suite/suite-development/review-log/2026-05-24-suite-review.md#review-92--2026-05-24-0419z) Path 2 amendment landing 2026-05-24 + the Layer 3 IAR cycle opening 2026-05-24 in the same day; the discipline is operative going forward; the project-side authoring routes to the operator's main-session collection step that closes this PR.

---

<a id="r1-f7"></a>
**Finding 7 — Cost-tally evidence for the Layer 3 spec-activation + Phase 2a/2b/2c cycle is absent from the commit messages; the [R2 F5](2026-05-21-ai-engineer.md#r2-f5) + [R3 F5](2026-05-21-ai-engineer.md#r3-f5) implementation-cycle commit-message `**Cost-tally:**` gap from Layer 2 has carried forward into Layer 3 unchanged (Dim 2)**

**Owner:** ai-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** sanity-check

**Validator rationale:** Dim 2 (token economy per finding) regression-check against [R2 F5](2026-05-21-ai-engineer.md#r2-f5) + [R3 F5](2026-05-21-ai-engineer.md#r3-f5) — the Layer 2 finding flagged that Phase 2a/2b sub-agent commits do not carry `**Cost-tally:**` lines in commit messages and routed the fix to suite-side methodology authoring. Sanity-check verifies whether the discipline landed in time for Layer 3.

The 5 PR #52 commits (`79a9a83` / `654cbbf` / `878d3b6` / `fd21900` / `78bd3cf`) carry `Co-Authored-By: Claude Opus 4.7 <noreply@anthropic.com>` trailers but do NOT carry `**Cost-tally:**` lines in their commit message bodies. The Layer 2 [R2 F5](2026-05-21-ai-engineer.md#r2-f5) + [R3 F5](2026-05-21-ai-engineer.md#r3-f5) named this gap explicitly and routed the methodology-authoring fix to extend the commit-message convention to require per-commit `**Cost-tally:**` + `**Model:**` lines for sub-agent-authored or operator-directed implementation/fix commits. Inspection of the [`suite-development.md` § Cost-tally schema](../../../../vsdd-suite/suite-development/suite-development.md) (per [Review 91 Finding 13](../../../../vsdd-suite/suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z)) shows the cost-tally schema codified at the per-Review-entry-preamble surface (the AI Engineer / suite-review / cycle-closing entry surface) but NOT extended to the per-commit-message surface — the Layer 2 [R2 F5](2026-05-21-ai-engineer.md#r2-f5) + [R3 F5](2026-05-21-ai-engineer.md#r3-f5) fix did not land at the commit-message convention layer.

The carry-forward shape is: the Layer 2 finding identified the gap; the suite-side codification work focused on the per-Review-entry surface (which is the higher-leverage surface — the cost-tally lives at the cycle-close boundary, not at every-commit boundary); the per-commit-message surface remains unsubsidized. This is a calibrated tradeoff (not all commits warrant cost-tally; the cycle-close boundary is the right granularity) — but the Layer 2 [R2 F5](2026-05-21-ai-engineer.md#r2-f5) Deferred-with-named-trigger framing has not been explicitly re-classified to Dismissed (the trigger "discipline lands at commit-message convention layer" did not fire); the finding remains Open across the cycle.

The cost-tally evidence for the Layer 3 spec-activation + implementation cycle that IS observable from the audit-trail:

- **Commit count:** 5 commits in PR #52 (`79a9a83` + `654cbbf` + `878d3b6` + `fd21900` + `78bd3cf`).
- **Lines-of-change:** spec activation `79a9a83` ≈ ~500 lines DESIGN.md + TODO.md + CHANGELOG.md edits; operator-confirmation `654cbbf` ≈ ~200 lines edits; Phase 2a `878d3b6` = 627 lines test additions; Phase 2b `fd21900` = 376 lines src + CHANGELOG; Phase 2c `78bd3cf` = 13 lines.
- **Sub-agent vs inline:** operator-confirmable (the commit-message trailer says `Co-Authored-By: Claude Opus 4.7` but doesn't distinguish orchestrator-inline from delegated-sub-agent).
- **Per-finding token cost for this AI Engineer round:** NOT COMPUTABLE — pending operator `/cost` paste per the [primer 3 § Per-field auditability tier](../../../../vsdd-suite/primers/3-review-session.md) operator-verifiable tier (raw tokens + would-be API cost + rate-limit-window utilization are operator-verifiable, NOT agent-self-verifiable).

This is the [R1 F6](2026-05-21-ai-engineer.md#r1-f6) precedent applied to the Layer 3 surface: the audit-trail does not record per-agent token consumption at the granularity needed for the Dim 2 expected-band test to run cleanly. The per-field auditability tier ([primer 3 § Per-field auditability tier](../../../../vsdd-suite/primers/3-review-session.md)) is now codified; agents authoring inline cost-tallies MUST fill only the agent-self-verifiable tier with hard counts. This finding's cost-tally section below adheres to that discipline.

**Resolution:** No project-side amendment required at this commit boundary; the suite-side methodology authoring (per the [R2 F5](2026-05-21-ai-engineer.md#r2-f5) + [R3 F5](2026-05-21-ai-engineer.md#r3-f5) framing) remains the right surface for the per-commit-message convention extension. This AI Engineer Round 1 entry carries the cost-tally per the codified per-field auditability tier (see Cost-tally section below); the cycle-close cost-tally for the full 13-domain L3 IAR Round 1 lives at the cycle-close suite-side review-log entry (deferred to the after-action cost report per [primer 3](../../../../vsdd-suite/primers/3-review-session.md)).

**Classification:** Deferred — regression-carries [R2 F5](2026-05-21-ai-engineer.md#r2-f5) / [R3 F5](2026-05-21-ai-engineer.md#r3-f5) forward. The trigger remains "per-commit-message convention extension lands at suite-side". This Layer 3 cycle is the second cycle the gap has been observed at; per the earned-by-recurrence-extension-trigger policy in [Review 92 Finding 5](../../../../vsdd-suite/suite-development/review-log/2026-05-24-suite-review.md#review-92--2026-05-24-0419z) (cost-tally surface AI-Engineer-owned), the second-cycle recurrence is the discipline-amendment trigger but NOT yet the hook-escalation trigger; the methodology amendment routes to a suite-side discussion (whether per-commit cost-tally extension warrants the convention vs. the per-Review-entry surface is sufficient).

---

### Dismissed

*(none)*

---

### Hallucinated

*(none)*

---

### Summary

7 substantive findings (5 Resolved + 2 Deferred) + 0 Dismissed + 0 Hallucinated.

**AI-co-authored disclosure shape evaluation:** Operative + correctly attributed per [Finding 1](#r1-f1). The PROCESS.md-parallel cross-reference + operator-directive verbatim + G-156 boundary-marker + inlined operator-confirmed-decisions list + post-confirmation language softening is the canonical AI-co-authored-spec disclosure shape; future cycles regression-check against this five-clause shape.

**Cite-verify discipline evaluation:** The operator-confirmation pass `654cbbf` scoped to the 8 AI-author-flagged decisions did NOT independently verify silent assertions outside the flagged surface per [Finding 2](#r1-f2); one small cite-drift detected (`Layer 2 R2 UX F4` cited where actual authority is the broader UX-cluster singular/plural discipline aggregating UX F2 + SE F2 + UX F4). The cite-drift is small enough that the operator-confirmation pass + this AI Engineer round's surfacing-of-the-gap together close the immediate exposure; the discipline-fix for future cycles is to extend the operator-confirmation pass to include a cite-verify sub-pass (optionally delegated to a Haiku 4.5 sub-agent per Dim 6).

**Cold-session-vs-inline decision evaluation:** Inline was the right execution method per the [primer 5 cold-vs-inline rubric](../../../../vsdd-suite/primers/5-formal-hardening.md) applied at the spec-authoring boundary per [Finding 3](#r1-f3). The four-test rubric (does the work require adversarial framing / session isolation / cost-compounding / operator-confirmation discipline) all pass inline as the right choice for L3 spec authoring.

**AI-author-flag removal evaluation:** Grep-clean across DESIGN.md + TODO.md post-`654cbbf` per [Finding 4](#r1-f4); the per-site-fix-list framing in the operator-confirmation-pass commit message is the canonical removal-discipline shape.

**Sub-agent delegation evaluation:** Phase 2a + 2b two-commit canonical shape correctly applied per [Finding 5](#r1-f5); orchestrator-vs-sub-agent per-commit decision is operator-confirmable per the per-field auditability tier (not agent-self-verifiable).

**Pre-cycle methodology check evaluation:** Pre-cycle declaration absent for the 13-domain L3 IAR Round 1 cycle launching today per [Finding 6](#r1-f6); the methodology-codification-to-project-adoption lag from the [Review 92 Finding 3](../../../../vsdd-suite/suite-development/review-log/2026-05-24-suite-review.md#review-92--2026-05-24-0419z) Path 2 amendment landing 2026-05-24 (same day) is the proximate cause; routes to the operator's main-session entry that opens the L3 IAR Round 1 cycle in the suite-side review-log.

**Cost-tally framing evaluation:** [R2 F5](2026-05-21-ai-engineer.md#r2-f5) / [R3 F5](2026-05-21-ai-engineer.md#r3-f5) carry-forward at the per-commit-message surface per [Finding 7](#r1-f7); the per-Review-entry-preamble surface (where the suite-side codification work landed) is the right granularity; this AI Engineer Round 1 entry carries its cost-tally per the codified per-field auditability tier below.

**MVR signal:** NOT REACHED at Round 1. Per the [primer 3 § Round triggers G-131 continue-trigger](../../../../vsdd-suite/primers/3-review-session.md): this round produced 7 substantive findings (5 Resolved + 2 Deferred), so AI Engineer Round 2 against bookmark-cli-manual Layer 3 is mandatory after the Deferred findings (F6, F7) are addressed at the cycle-close + the Layer 3 IAR Round 1 13-domain cycle closes. The Round 2 cold pass verifies the pre-cycle declaration landed (F6 closure) + verifies the cost-tally framing held at the per-Review-entry surface (F7 closure) + looks for adjacent defects the fixes may have created.

**Coordination:** [Finding 6](#r1-f6) routes to the operator's main-session entry that opens the L3 IAR Round 1 cycle in the suite-side review-log (the pre-cycle declaration discipline); [Finding 7](#r1-f7) regression-carries the [R2 F5](2026-05-21-ai-engineer.md#r2-f5) / [R3 F5](2026-05-21-ai-engineer.md#r3-f5) per-commit-message cost-tally gap forward to the suite-side methodology-amendment discussion (whether the per-Review-entry surface is sufficient or the per-commit-message convention warrants extension). The 5 Resolved findings (F1 + F2 + F3 + F4 + F5) document operative disciplines for future AI-co-authored spec cycles (in this project at Layer 4+, or in sibling capstone projects); no suite-side amendment from the Resolved findings until a second project encounters the same surface (earned-by-recurrence per [Review 92 Finding 3 Path 2](../../../../vsdd-suite/suite-development/review-log/2026-05-24-suite-review.md#review-92--2026-05-24-0419z) + [Review 92 Finding 5](../../../../vsdd-suite/suite-development/review-log/2026-05-24-suite-review.md#review-92--2026-05-24-0419z)).

---

**Cost-tally:** (per [`suite-development.md`](../../../../vsdd-suite/suite-development/suite-development.md) § Per-review entry preamble § Cost-tally + [primer 3 § Cost-tally report shape](../../../../vsdd-suite/primers/3-review-session.md) + § Per-field auditability tier; field numbering per [primer 3 § Cost-tally report shape](../../../../vsdd-suite/primers/3-review-session.md) 10-field schema)

**Agent-self-verifiable fields (countable from this session's tool-call log):**

1. **AI tool:** claude-code CLI (per the prompt's "Main session collects + commits" framing + the sub-agent execution model)
3. **Execution method:** sub-agent spawn from operator's main session (this prompt invocation is itself the sub-agent execution; operator's main session orchestrates the 13-domain L3 IAR Round 1 cycle)
4. **Model:** claude-opus-4-7 (per the project's [DESIGN.md § Cold-session budget](../../DESIGN.md) line 19 declaration — Opus 4.7 for AI Engineer)
- **Tool-call counts:** ~9 Read invocations (AI-ENGINEER-REVIEW.md domain prompt; 3-review-session.md primer; suite-development.md preamble + cost-tally sections; claude-code-cli.md supplement spot-greps; 2026-05-21-ai-engineer.md prior round; DESIGN.md L3 sections; TODO.md L3 section; CHANGELOG.md L3 entries; 2026-05-24-ux.md preamble shape reference); ~14 Bash invocations (grep + git log + git show + ls + date — most for cite-verify + structural verification); 1 Write (this file).
- **Files read:** 9 source files (paths above) totaling ~3,200 lines read (subset of full file extents where Read offset+limit applied).
- **Mechanical sweeps run:** 6 greps (`AI-author note`, `AI-author flag`, `AI-author-default`, `Layer 3.*IAR`, finding ID resolution, anchor-id sweep).
- **Wall-clock anchors:** Session-end Bash `date -u` = 2026-05-25T01:15Z. Session-start anchor not captured (this finding makes that omission visible per the [Review 91 Finding 15](../../../../vsdd-suite/suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) wall-clock measurement pattern; future AI Engineer rounds should anchor at session-start with the first Bash tool-call).

**Operator-verifiable fields (requires operator `/cost` paste; operator-action queue):**

5. **Raw tokens:** *pending operator `/cost` paste*
6. **Would-be API cost:** *pending operator `/cost` paste*
8. **Rate-limit-window utilization:** *pending operator `/cost` paste*

**Operator-confirmable fields (operator-declared per session; NOT inherited from prior context):**

2. **Plan tier:** *pending operator confirmation* (do NOT inherit from prior cycle context per the [primer 3 § Per-field auditability tier](../../../../vsdd-suite/primers/3-review-session.md) hard rule)
7. **Actual cost to operator:** *pending operator declaration of plan tier — `$0 marginal (within plan limits)` if subscription plan; would-be-API-cost if API-direct*

**Derived metric (computable only when all inputs measured):**

10. **Findings/100k tokens:** NOT COMPUTABLE — pending operator `/cost` paste (Field 5 unmeasured)

**Wall-clock duration (Field 9):** Session-end anchor 2026-05-25T01:15Z captured via Bash `date -u`; session-start anchor not captured; elapsed duration NOT COMPUTABLE for this session per the [`claude-code-cli.md` supplement § Wall-clock measurement pattern](../../../../vsdd-suite/supplements/claude-code-cli.md) — agent did NOT count time between tool calls; the session-end-only anchor is insufficient. Future AI Engineer rounds should invoke `date -u` as the first Bash tool-call to capture the session-start anchor.

**Operator-action queue:** if cost-tally precision is load-bearing for cross-cycle calibration (e.g., for AI Engineer Round 2's regression-check against this cycle's actual cost vs. the [DESIGN.md § Cold-session budget](../../DESIGN.md) declared band of 100k–300k tokens/finding), operator runs `/cost` in this session and pastes the output here as an append-only addendum, replacing the *pending operator …* placeholders with measured values. Operator also confirms plan tier (Claude Max / Claude API direct / etc.) so Field 2 + Field 7 + Field 8 can be filled.

**Honest agent-self-assessment of this cost-tally:** the 4 agent-self-verifiable fields filled with hard counts above are this agent's honest measurement; the 6 operator-verifiable/confirmable/derived fields are correctly placeholdered per the [Review 91 Finding 8](../../../../vsdd-suite/suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) hard-rule against fabrication. The prior AI Engineer rounds ([R1 cost-tally](2026-05-21-ai-engineer.md#r1-f6), [R2 cost-tally](2026-05-21-ai-engineer.md#r2-f5), [R3 cost-tally](2026-05-21-ai-engineer.md#r3-f5)) authored cost figures inline (~21k tokens/finding band for R2; ~$0.61 for R2 5 verification entries; etc.) — those figures pre-dated the [Review 91 Finding 8](../../../../vsdd-suite/suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) per-field auditability tier codification and are preserved per [G-89](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-89) forward-only narrative-preservation; this Round 1 entry applies the post-codification discipline.

---

---

## Review 2 — 2026-05-25 04:30Z

<!-- hook-bypass: this Round 2 entry uses **Bold-paragraph emphasis** as inline subsection emphasis within Round-1-cycle mining + operator-action-queue blocks. These bold lines are paragraph-level emphasis, not Finding headers; actual Round 2 Findings in this entry use the canonical `**Finding N — Title**` form. The check-suite-review-preamble hook's `**X — Y**` regex matches both; the bypass-mechanism is itself a finding for the next registry-walk review. -->

**Round:** Layer 3 Phase 3 IAR Round 2.
**Session-start (Bash `date -u`):** 2026-05-25T03:06Z
**Session-end (Bash `date -u`):** 2026-05-25T03:09Z (anchored at cost-tally authoring; see Wall-clock field)

**Scope:** AI-agent-usage shape across the Layer 3 Round 1 fix-work cycle (4 commits: `fdfa989` Phase 1a+1b spec amendments + `ba6a4a9` Phase 2a regression tests + `bfc0713` Phase 2b impl + `795bc25` manual-tests/layer-3.md + Phase 2c annotation) AND the Round 2 launch itself (this cold-session AI Engineer round + the 12 sibling per-domain Round 2 spawns visible at `## Review 2 — Layer 3 Phase 3 IAR Round 2 — 2026-05-25 04:30Z` headings across the per-domain review-log files); the in-cycle suite-hardening at commit `e4b6701` (Review 94 + `check-no-letter-clusters.py` hook + primer 4 § Routing output Cluster identifier discipline + `.pre-commit-config.yaml` wiring); the architectural correction sub-decision recorded in `bfc0713`'s commit body (display_safe removal from `export_json`); the Round 1 routing record at per-domain Phase 4 routing appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md` (per-domain Phase 4 appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md`). Re-verification targets per the Round 1 closure context: [AIE R1 F6](#r1-f6) (pre-cycle methodology declaration absent) → routed for Round 2 launch closure; [AIE R1 F7](#r1-f7) (per-commit cost-tally gap) → continued carry-forward status. Adjacent surfaces: 12 sibling per-domain Round 2 entry preambles (UX Review 2; SE Review 2; QE Review 9; PE Review 8 — `## Review` headings spot-grepped to confirm per-domain spawn shape, NOT loaded in full per cold-session discipline against adversary's prior claim).

**Lens:** Cost-and-quality discipline for AI-agent usage at the Round-1-fix-work-close + Round-2-launch boundary, applied to the Layer 3 cycle specifically. Adversarial questions held: did the Round 2 launch close [AIE R1 F6](#r1-f6) cleanly (pre-cycle methodology declaration adequacy)? Did the Round 1 fix-work commits address [AIE R1 F7](#r1-f7) at the per-commit-message surface (cost-tally inclusion)? Was the in-cycle suite-hardening at `e4b6701` within AI Engineer [Dim 9](../../../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md) cold-session-budget acceptable scope or over-investment for the cycle? Was the architectural correction sub-decision pattern (operator AskUserQuestion mid-Phase-2b authorizing display_safe removal from export_json beyond Round 1 routing) sound + auditable as a future-cycle precedent? Were the 13 cold-session agent spawns for Round 2 launched with adversarial-pair separation preserved + cluster-letter labels avoided in the spawn prompts (Dim 7)? Did the Round 2 spawn prompts include Round 1 closure context as the scope-reducer framing (Dim 8)? Do the Round 1 fix-work + suite-hardening commits cite the source-of-claim documents verifiably (Dim 11 cite-verify)? **Sycophancy compensation:** resisted outcome bias toward shipped state (the Round 1 fix-work landed cleanly at 51/51 GREEN + the Round 2 launch executed on time; the temptation is to read "the cycle is on-track" as "the cost-discipline is calibrated"); resisted sunk-cost defense of the in-cycle suite-hardening choice (operator authorized it but the post-PR-merge-then-hardening pattern is the suite's standing convention — the in-cycle exception was operator-authorized, but adversarial pressure on whether the in-cycle authorization was the right call is the load-bearing posture); resisted confirmation bias toward Round 1's framing (the Round 2 launch prompt frames Round 2 as scope-reducer — adversarial pressure on whether the scope is ACTUALLY reduced is required).

**Supplements applied:** [`claude-code-cli.md`](../../../../vsdd-suite/supplements/claude-code-cli.md) § Plan tiers + rate-limit windows + § Prompt-cache discipline + § Cost-tally discipline + § Wall-clock measurement pattern (the operator's plan tier is operator-confirmable; the 13-parallel-spawn shape against the 5-min cache TTL is the Dim 3 surface; the wall-clock anchor pattern this round applies per the pattern); [`markdown.md`](../../../../vsdd-suite/supplements/markdown.md) § AI Engineer Dim 11 spot-check (heading shape + per-Finding anchor IDs + classification-section greppability against the just-codified [Review 93 Finding 2](../../../../vsdd-suite/suite-development/review-log/2026-05-24-suite-review.md#review-93--2026-05-24-2340z) single-section-per-classification rule applied to this Round 2 entry).

**Session note:** Cold session opened for the Layer 3 IAR Round 2 AI Engineer scope; this reviewer did NOT author the 4 fix-work commits, the routing record, the suite-hardening commit, or the 12 sibling Round 2 spawns. Reading order: Bash `date -u` for session-start anchor (per [`claude-code-cli.md` § Wall-clock measurement pattern](../../../../vsdd-suite/supplements/claude-code-cli.md)) → this file's [Review 1](#review-1--2026-05-24-2330z) (treated as adversary's prior claim per [Dim 1](../../../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md) session-isolation discipline; specifically [R1 F6](#r1-f6) + [R1 F7](#r1-f7) deferred-status continuation candidates) → [AI Engineer domain prompt](../../../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md) (full re-read for Dim 7 cluster-batching + Dim 8 Phase-4-routing + Dim 9 cold-session-budget + Dim 11 cite-verify + Dim 13 pre-cycle methodology check) → [primer 3 § Pre-cycle methodology check § Scope of the pre-cycle discipline](../../../../vsdd-suite/primers/3-review-session.md) (full re-read for the operator-policy Path 2 amendment landing) → [primer 4 § Routing output § Cluster identifier discipline](../../../../vsdd-suite/primers/4-feedback-integration.md) (full re-read for the Review 94 Finding 3 codification) → [`hooks/check-no-letter-clusters.py`](../../../../vsdd-suite/hooks/check-no-letter-clusters.py) (full read for the Dim 11 + Dim 7 hook-discipline anchor) → [`claude-code-cli.md` supplement](../../../../vsdd-suite/supplements/claude-code-cli.md) (full read for Dim 3 + Dim 6 + Dim 14 surfaces) → [`suite-development.md` § Per-review entry preamble § Cost-tally](../../../../vsdd-suite/suite-development/suite-development.md) (offset-read for cost-tally schema confirmation) → Phase 4 routing record (per-domain Phase 4 appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md`) (full read for the AI-Engineer-process-discipline carry-forward cluster + the cluster identifier discipline observed shape) → 5 fix-work + suite-hardening commit messages (`fdfa989` / `ba6a4a9` / `bfc0713` / `795bc25` / `e4b6701` via Bash `git log --format=fuller -1 <sha>`) → 12 sibling per-domain Round 2 entries (`## Review` heading spot-greps only, NOT loaded in full per cold-session discipline) → [Suite Review 94](../../../../vsdd-suite/suite-development/review-log/2026-05-24-suite-review.md#review-94--2026-05-25-0300z) (heading sweep + Finding 3 deep-read for the in-cycle hardening rationale). [`hooks/check-no-letter-clusters.py`](../../../../vsdd-suite/hooks/check-no-letter-clusters.py) run against the routing record + Review 1 file in this session for empirical hook-discipline evidence.

**Source:** `domain-raised` — cold-session AI-agent-usage auditor applying [Dim 7](../../../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md) (cluster-batching with adversarial-pair separation) + [Dim 8](../../../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md) (Phase 4 routing as Round-2+ scope-reducer) + [Dim 9](../../../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md) (cold-session-budget declaration per project intent tier) + [Dim 11](../../../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md) (audit-trail machine-readability cost + cite-verify sub-clause + supplement-citation sub-clause) + [Dim 12](../../../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md) (operator-directive correction cost) + [Dim 13](../../../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md) (pre-cycle methodology check) + [Dim 14](../../../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md) (tool/plan/execution-method identification) against the Layer 3 Round 1 fix-work cycle + the Round 2 launch. Findings surfaced from artifact-state analysis (commit-message + routing-record + Round 2 spawn-prompt + suite-side review-log + hook-execution) NOT from outcome-bias-toward-shipped-state.

**Regression check against:** [Review 1 (2026-05-24)](#review-1--2026-05-24-2330z) — esp. [R1 F6](#r1-f6) (pre-cycle methodology declaration absent for the 13-domain Layer 3 IAR Round 1 cycle) + [R1 F7](#r1-f7) (per-commit cost-tally gap carry-forward from Layer 2 R2 F5 / R3 F5). Round 1 5 Resolved findings (F1 disclosure-shape; F2 cite-verify discipline; F3 cold-session-vs-inline decision; F4 AI-author-flag removal; F5 sub-agent delegation pattern) are forward-only-documented disciplines for future AI-co-authored spec cycles; Round 2 regression-check spot-greps the Round 1 fix-work for evidence the disciplines held across the Round-1-fix-to-Round-2-launch transition.

**Round:** 2
**Active domain set:** 12 role + 1 meta = 13 (per [DESIGN.md § Project intent](../../DESIGN.md); unchanged from Round 1).

---

### Resolved

<a id="r2-f1"></a>
**Finding 1 — R1 F6 pre-cycle methodology declaration: Round 2 launch prompt visible to this sub-agent carries the declaration fields inline but the discipline is NOT codified as a suite-side review-log entry; F6 is partial-closure not full-closure (Dim 13)**

**Owner:** ai-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

**Validator rationale:** Dim 13 pre-cycle methodology check applied as regression-check against [R1 F6](#r1-f6); sanity-check verifies the partial-closure framing against the [primer 3 § Pre-cycle methodology check](../../../../vsdd-suite/primers/3-review-session.md) discipline's explicit "operator-authored at cycle-spawn time" clause.

[R1 F6](#r1-f6) routed the pre-cycle methodology declaration discipline to the operator's main-session entry that opens the L3 IAR Round 1 cycle in the suite-side review-log. The Round 2 launch prompt visible to THIS sub-agent (the prompt I was spawned with) names the methodology-declaration fields:

- **Spawn shape:** "Round 2 spawned 13 cold-session agents in parallel" (per-domain; not cluster-batched).
- **Per-cycle budget:** carried from [DESIGN.md § Project intent § Cold-session budget](../../DESIGN.md) line 19 (capstone default; same as Round 1).
- **Rate-limit headroom:** not declared in the spawn prompt (operator-confirmable per [`claude-code-cli.md` supplement § Plan tiers + rate-limit windows](../../../../vsdd-suite/supplements/claude-code-cli.md); this sub-agent does NOT have visibility into the operator's 5-hour-rolling-window utilization at cycle start; same opacity as Round 1).
- **Model selection per task class:** AI Engineer at Opus 4.7 per the DESIGN.md cold-session-budget table (model assignment matches; no drift).
- **AI tool + plan tier + execution method (Dim 14):** the prompt frames "this Round 2 launch's pre-cycle methodology declaration (visible in the main session prompt text)" — implicitly claude-code CLI sub-agent spawn from main session; plan tier remains operator-confirmable.
- **Phase-2a-evidence-shape:** the Round 1 fix-work landed Phase 2a (`ba6a4a9`) as standalone commit then Phase 2b (`bfc0713`) as second commit — canonical two-commit shape per [primer 2a § Verifiable git-history check](../../../../vsdd-suite/primers/2a-red-gate.md). Round 2 does not produce a Phase 2a evidence-shape (Phase 3 IAR is review-only).

The declaration is ADEQUATE in field coverage as the spawn-prompt visible to this sub-agent. The closure GAP: per [primer 3 § Pre-cycle methodology check](../../../../vsdd-suite/primers/3-review-session.md) explicit wording, the declaration "is operator-authored at cycle-spawn time, not retrospective. It exists so the AI Engineer Round-N+1 verification can regression-check actual cost against declared cost." The discipline's intent is that the declaration lives in the **suite-side review-log** as a persistent audit-trail artifact — not solely in the operator's spawn-prompt-to-sub-agents (which is ephemeral; the sub-agent's prompt is not committed to the audit trail; future readers cannot regression-check against a prompt that does not persist). Search of the suite-side review-log for the Round 2 pre-cycle declaration: `grep -n "Round 2\|pre-cycle" vsdd-suite/suite-development/review-log/2026-05-24-suite-review.md` returns hits for [Review 94 Finding 3 timing-table mentions of "Round 2 IAR review"](../../../../vsdd-suite/suite-development/review-log/2026-05-24-suite-review.md#review-94--2026-05-25-0300z) (about lettering-defense catch timing) but NO Layer-3-IAR-Round-2-specific pre-cycle declaration entry exists. No standalone `vsdd-suite/suite-development/review-log/2026-05-25-bookmark-cli-manual-l3-iar-round-2.md` exists either.

The partial-closure framing: F6's Round 1 routing intent was "open the cycle's suite-side review-log entry with a pre-cycle declaration"; the Round 2 launch satisfied the field-coverage requirement at the sub-agent-prompt boundary but NOT the persistent-audit-trail-artifact requirement. The R1 F6 routing record line in the Phase 4 routing record (per-domain Phase 4 appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md`) frames the routing as "F6: Round 2 launch includes the pre-cycle methodology declaration per primer 3 § Pre-cycle methodology check Path 2" — the routing-text scopes the closure to "launch includes the declaration" which is satisfied at the spawn-prompt surface, but the discipline-intent (persistent audit-trail) is not. The closure-gap is therefore an artifact-surface mismatch between F6's routing-text (closure satisfied) and the discipline's intent (closure not yet satisfied at the suite-side review-log).

**Resolution:** R1 F6 is closed at the spawn-prompt-text surface per its Round 1 routing-text framing. The discipline-intent (persistent audit-trail entry) is satisfied retrospectively by the operator's main-session collection step recording the Round 2 pre-cycle declaration in the suite-side review-log alongside the Round 2 cycle close (operator's main-session work post this sub-agent's return). Routes to operator's main-session collection step: the declaration fields above (Spawn shape: 13 per-domain agents; Per-cycle budget: capstone default per DESIGN.md; Rate-limit headroom: operator-confirmable; Model selection: per-task-class per DESIGN.md; AI tool + plan tier + execution method: claude-code CLI sub-agent spawn; Phase-2a-evidence-shape: not applicable for Phase 3 review-only round) should be inlined into the suite-side review-log entry that opens the L3 IAR Round 2 cycle — closing the pair (pre-cycle declaration → after-action cost report) at the Round 2 cycle boundary per the [primer 3 § Pre-cycle methodology check](../../../../vsdd-suite/primers/3-review-session.md) discipline. The artifact-surface mismatch is the load-bearing observation for future cycle launches: spawn-prompt declarations are necessary-but-not-sufficient; the persistent audit-trail entry is the discipline's actual closure artifact.

**Classification:** Resolved

---

<a id="r2-f2"></a>
**Finding 2 — Round 2 13-agent per-domain spawn shape preserves adversarial-pair separation by construction; cluster-letter labels avoided in spawn-prompt-to-this-sub-agent (Dim 7)**

**Owner:** ai-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

**Validator rationale:** Dim 7 (cluster-batching with adversarial-pair separation) applied to the Round 2 spawn shape; sanity-check confirms the per-domain (not cluster-batched) shape satisfies the adversarial-pair-separation invariant trivially.

The Round 2 launch is per-domain (13 parallel cold-session agents; one per active capstone domain) per the operator's prompt framing ("Round 2 spawned 13 cold-session agents in parallel"). Per-domain shape trivially satisfies the adversarial-pair-separation invariant (Security ↔ Red Team; Technical Writer ↔ Documentation Reviewer) because each pair-member runs in its own session — no cluster-batching at all. The Layer 2 Round 2 4-cluster shape (per the 4 commits `7e7d949` + `8eee3ff` + `73eb207` + `9616b82`) was the cluster-batching precedent; Layer 3 Round 1 + Round 2 both reverted to per-domain shape. The cost trade-off: per-domain at 13 agents spends ~3x the orchestrator-overhead of 4-cluster batching (per [`claude-code-cli.md` supplement § Cluster-batching with adversarial-pair separation (4-cluster shape)](../../../../vsdd-suite/supplements/claude-code-cli.md)) but produces higher per-domain adversarial pressure (no within-cluster context cross-contamination softening the framing per Dim 1 session isolation). The operator's choice to spawn per-domain at Layer 3 instead of cluster-batched is a defensible calibration choice — the cost is higher but the adversarial pressure is correspondingly higher; the Round 1 cycle produced 76 findings across 13 agents which is empirical evidence the per-domain shape is producing dense findings.

Cluster-letter label check applied via grep against the Round 2 spawn-prompt-text-to-this-sub-agent: the prompt visible to me uses descriptive identifiers throughout ("AI Engineer adversarial reviewer"; "Security ↔ Red Team; TW ↔ DR" adversarial-pair naming; no `Cluster A/B/C/D` letter-coded references). The prompt does NOT use the descriptive-name convention the [`claude-code-cli.md` supplement § Lettering-violation recurrence in cluster-spawn prompts](../../../../vsdd-suite/supplements/claude-code-cli.md) names as the recurrence pattern — per-domain spawn doesn't have a labeling surface to slip on. The check-no-letter-clusters hook would not fire on spawn-prompt text (the hook scans markdown files in the project + suite-side scope, not ephemeral sub-agent prompts) but the absence of letter-labels in the spawn shape is observable from the prompt-text visible to this sub-agent + from the Phase 4 routing record's post-sweep state (the routing record's section headers were swept from cluster-letter to descriptive identifiers per the operator's in-cycle hardening at `e4b6701`).

The per-domain-vs-cluster-batching cost calibration observation worth noting (not a finding; observation for future-cycle calibration): the Layer 2 Round 2 4-cluster shape was operator-chosen to reduce token cost (per [Layer 2 R2 VDD-IAR Alignment R4 F1 evidence-preservation annotation](../../TODO.md)); the Layer 3 Round 1 + Round 2 per-domain shape is operator-chosen to maximize adversarial pressure. Neither is wrong; the choice per layer is a Dim 9 cold-session-budget calibration decision. The audit-trail does not currently document the per-layer rationale for the cluster-vs-per-domain choice — future cycles should name the choice + the rationale in the pre-cycle declaration (Dim 13's "Spawn shape" field).

**Resolution:** Round 2 per-domain spawn shape satisfies adversarial-pair separation by construction; cluster-letter labels absent in the spawn-prompt-to-this-sub-agent; documented for future-cycle regression-check. The per-domain-vs-cluster-batching choice rationale (cost-tradeoff calibration per layer) should be named in future pre-cycle declarations as part of the Dim 13 "Spawn shape" field.

**Classification:** Resolved

---

<a id="r2-f3"></a>
**Finding 3 — Round 2 spawn prompt frames scope as "verify Round 1 fixes hold + surface NEW residuals" with Round 1 closure context included; Phase 4 routing scope-reducer applied cleanly (Dim 8)**

**Owner:** ai-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

**Validator rationale:** Dim 8 (Phase 4 routing as Round-2+ scope-reducer) applied to the Round 2 spawn shape; sanity-check confirms the spawn-prompt's scope-reducer framing against the Dim 8 named failure mode.

The Round 2 spawn-prompt-to-this-sub-agent frames the scope as: *"Round 2 scope (per your own Round 1 Dim 8 framing — Phase 4 routing as Round-2+ scope-reducer): Verify Round 1's process discipline held + surface NEW AI-Engineering residuals from the fix-work + the Round 2 launch itself."* The prompt includes:

- The Round 1 closure context (5 Resolved + 2 Deferred finding summary; F6 routing status; F7 carry-forward status).
- The Phase 4 routing record path (loaded as cold-session context).
- The 4 Round 1 fix-work commit shas (`fdfa989` → `ba6a4a9` → `bfc0713` → `795bc25`) explicitly enumerated.
- The in-cycle suite-hardening commit sha (`e4b6701`) explicitly enumerated.
- The architectural correction sub-decision context (display_safe removal from export_json beyond Round 1 routing).
- 7 explicit "Critical re-verification targets" naming the specific scope-reducer questions.

This is the canonical Dim 8 scope-reducer shape. The Dim 8 named failure modes ("Round 2 agents spawned with the same scope as Round 1"; "Round 2 prompts that omit the Round 1 finding list"; "Round 2 prompts that omit both Round 1 + Round 2") all do NOT apply at this spawn — the Round 1 finding list is included; the prior-round-resolution context is included; the scope-reduction framing is explicit. The prompt also names Phase 4 routing as the scope-reducer mechanism explicitly — making the discipline visible at the spawn surface, not just implicit.

What this finding CANNOT verify from the spawn-prompt-to-this-sub-agent: whether the 12 sibling per-domain Round 2 spawn-prompts received the same scope-reducer framing. The operator-confirmable observation: spot-greps of the 4 sibling Round 2 entries that already landed ([UX Review 2 line 194](2026-05-24-ux.md#review-2--layer-3-phase-3-iar-round-2--2026-05-25-0430z); [SE Review 2 line 231](2026-05-24-software-engineer.md); [QE Review 9 line 215](2026-05-24-quality-engineer.md); [PE Review 8 line 254](2026-05-24-performance-engineer.md)) show consistent `## Review 2 — Layer 3 Phase 3 IAR Round 2 — 2026-05-25 04:30Z` heading shape + Scope preambles that name the per-domain scope-reducer dimensions — empirical evidence the spawn-prompts converged on the scope-reducer framing. The UX Review 2 preamble explicitly names "Round 2 scope-reducer" framing ([line 194](2026-05-24-ux.md#review-2--layer-3-phase-3-iar-round-2--2026-05-25-0430z) — "Focus areas per the scope-reducer:"); this is the strongest cross-agent evidence of consistent framing.

**Resolution:** Round 2 spawn prompt for AI Engineer frames scope as canonical Dim 8 scope-reducer with Round 1 closure context included; the 4 sibling Round 2 entries already landed show consistent scope-reducer framing in their preambles; documented as evidence the Dim 8 discipline is operative at the Round 2 boundary. The full 13-agent spawn-shape verification is operator-confirmable at the cycle-close suite-side review-log entry.

**Classification:** Resolved

---

<a id="r2-f4"></a>
**Finding 4 — In-cycle suite-hardening at commit `e4b6701` (Review 94 + new hook + primer 4 amendment) within Dim 9 acceptable scope at the 4th-recurrence trigger; over-investment risk acknowledged + operator-authorized (Dim 9 + Dim 12)**

**Owner:** ai-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

**Validator rationale:** Dim 9 (cold-session-budget declaration per project intent tier) applied to the in-cycle suite-hardening choice + Dim 12 (operator-directive correction cost) applied to the operator's mid-cycle authorization sub-decision; sanity-check applies both lenses to the in-cycle expansion-of-scope question.

The PR #52 cycle scope at activation (per the spec-activation commit `79a9a83`) was Layer 3 Phase 1ab + Phase 2abc + Phase 3 IAR. The in-cycle suite-hardening at `e4b6701` added: (1) a new pre-commit hook (`check-no-letter-clusters.py`); (2) a primer 4 § Routing output Cluster identifier discipline paragraph; (3) `.pre-commit-config.yaml` wiring; (4) suite-side Review 94 with 3 meta-findings + hook-bypass markers across 4 suite-side audit-trail files for G-89 forward-only narrative-preservation. This is methodology-amendment scope landing inside a project's reference-implementation PR — beyond the project-side spec's scope at activation.

The over-investment risk per Dim 9's named failure mode ("capstone-intent project running Round 7 cold-session verification without a stopping rule" — pattern-matches to "capstone-intent project doing suite-side methodology work inside a project PR without a stopping rule"): the in-cycle suite-hardening expands the PR #52 surface beyond the project-side L3 scope. The defense against over-investment per [Suite Review 94 Finding 3 partial-fix authorization](../../../../vsdd-suite/suite-development/review-log/2026-05-24-suite-review.md#review-94--2026-05-25-0300z): operator authorized the in-cycle partial fix specifically because the 4th-recurrence of the letter-label anti-pattern (Review 78 Surface lettering → PR #38 Round 3 cluster lettering → PR #44 Round 1 cluster lettering → PR #52 Phase 4 routing cluster lettering) is the empirical trigger for the next mechanical-enforcement layer per the suite's earned-by-recurrence policy. The operator's authorization framing: "the methodology IS correctly calibrated; the catch happens too late for the operator-visible cost" — naming that the timing-gap is the substantive defect, and the in-cycle hook addition is the right intervention because the same defect class will recur in the next cycle without it.

The Dim 12 operator-directive correction cost evaluation: the operator-authorization sub-decision occurred mid-cycle (per the Suite Review 94 Resolution narrative + the `e4b6701` commit landing 2026-05-24 19:25Z, between the Phase 4 routing commit `e233ad8` and the Phase 1a+1b fix-work commit `fdfa989`). The mid-cycle nature of the sub-decision is itself a Dim 12 finding — the methodology-authoring missed catching the letter-label recurrence at primer 4 § Routing output during the routing-pass authoring, which forced the in-cycle correction. The discipline-fix landed: the primer 4 amendment + the hook close the gap going forward; the future-cycle regression-check is that no future Phase 4 routing record contains cluster-letter labels (the hook enforces this mechanically at commit time).

What CAN'T be evaluated from the audit-trail at the in-cycle-suite-hardening boundary: whether the operator-authorization sub-decision pattern (operator pausing mid-cycle to authorize a scope-expansion) generalizes to future cycles where the trigger is less clearly empirically-driven (the 4th-recurrence here is a clear earned-by-recurrence pattern; future operator-authorizations on less-clear triggers carry over-investment risk that the methodology does not yet codify a stopping rule for). The future-cycle observation: if the operator authorizes a similar mid-cycle scope-expansion without an explicit earned-by-recurrence trigger naming, that's a Dim 9 over-investment finding to surface in that cycle's AI Engineer round. **Routes to operator-pattern-documentation for future cycles** — not a project-side amendment.

The cost-asymmetry calibration: the in-cycle hook addition cost (one hook + one primer paragraph + one suite-side Review entry + 4 hook-bypass markers = ~6 small artifact additions in 1 commit) vs the defect-class cost (4 recurrences × operator-time-to-flag + audit-trail-noise per recurrence + the 5th recurrence's expected operator-time-cost). The in-cycle authorization saves the 5th recurrence's cost; the post-PR-merge alternative would have deferred the catch to the 5th recurrence (one more cycle of operator-time-cost). The cost-asymmetry favors the in-cycle authorization on the specific 4th-recurrence trigger; this is consistent calibration with the [Review 91 Finding 17](../../../../vsdd-suite/suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z)'s earned-by-recurrence-trigger policy applied to the lettering recurrence specifically.

**Resolution:** In-cycle suite-hardening at `e4b6701` is within Dim 9 acceptable scope at the 4th-recurrence trigger + the operator-authorization sub-decision is documented + the cost-asymmetry favors the in-cycle authorization. The future-cycle generalization observation (operator-authorization sub-decision pattern requires an explicit trigger-naming to avoid Dim 9 over-investment drift) is documented for future-cycle AI Engineer regression-check. No project-side amendment needed; routes to operator-pattern-documentation for future cycles.

**Classification:** Resolved

---

<a id="r2-f5"></a>
**Finding 5 — Architectural correction sub-decision (display_safe removal from export_json beyond Round 1 routing) sound + auditable per the `bfc0713` commit body; operator-authorization sub-decision pattern sets defensible precedent (Dim 12)**

**Owner:** ai-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

**Validator rationale:** Dim 12 (operator-directive correction cost) applied to the architectural correction sub-decision; sanity-check verifies the operator-authorization sub-decision is auditable + the precedent shape is defensible.

The Round 1 Phase 4 routing record at per-domain Phase 4 routing appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md` § JSON-native escape design (per-domain Phase 4 appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md`) routed the JSON-native escape design 4-domain convergence as "Path C: switch `display_safe` from Rust-syntax `\u{HHHH}` (8-byte literal) to JSON-native `\uHHHH` (6-char escape). Preserves both terminal-safety AND byte-round-trip." The routing-text scoped the Phase 2b fix to "`src/lib.rs` `display_safe` function — change the escape format from `format!(\"\\u{{{:04x}}}\", c as u32)` to `format!(\"\\u{:04x}\", c as u32)` for BMP codepoints" — preserving the call-site shape (display_safe still applied at the per-field serialization step in `export_json`).

The Phase 2b implementation at `bfc0713` discovered the routing-text's technical premise was incorrect: pre-escaping inside the JSON encoding path double-escapes (the literal text becomes the literal sequence in JSON output and parses back as the 6-char text, NOT the original byte). The architectural correction: remove display_safe from `export_json` entirely; leverage serde_json's native control-char escaping. The byte-preservation intent of Round 1 routing is preserved; only the implementation path changed.

The commit body's audit-trail surface (`bfc0713` lines 41-46): *"Architectural correction sub-decision (Round 1 Phase 4 routing scope extension): Phase 2b implementation discovered the Round 1 Path-C decision had an incorrect technical premise (display_safe pre-escaping double-escapes through serde_json). Operator authorized the architectural correction (2026-05-25 main-session AskUserQuestion pass): remove display_safe from export_json entirely; leverage serde_json's native control-char escaping. The byte-preservation intent of Round 1 routing is preserved; only the implementation path changed."* This is the canonical operator-authorization sub-decision audit-trail shape: (1) name the discovery surface (Phase 2b implementation); (2) name the incorrect technical premise; (3) name the operator-authorization mechanism (AskUserQuestion); (4) name the date + main-session source; (5) name what changed + what was preserved (architectural-path vs intent). The audit-trail is sufficient for a cold-context reader to reconstruct the sub-decision.

The Dim 12 named failure modes for this surface: "operator-directives that surface late in the cycle (the discipline lives in the methodology authoring; if the operator has to surface the discipline mid-cycle, the methodology authoring missed a Dim)" — applies partially here. The Round 1 Phase 4 routing-text's incorrect technical premise (Path C as authored assumed display_safe pre-escaping was compatible with JSON encoding) was a routing-pass error that Phase 2b implementation caught. This is NOT a methodology-authoring miss (the methodology doesn't require routing-pass-time technical-premise-verification at the depth of "does this pre-escape interact correctly with the downstream encoder"); it IS an operator-judgment-needed sub-decision that the architectural correction's audit-trail correctly captures. The methodology already supports this pattern via Phase 4's "Re-open if the gate fails" clause + the multi-phase chain shape; the architectural correction is the canonical instance of "the routed Phase 1a+1b revision is itself flagged by Phase 3 in a subsequent round" extended to a Phase 2b in-implementation flagging.

The precedent shape worth codifying: the operator-authorization sub-decision audit-trail shape (the 5 elements above) is the canonical form for future cycles where a Phase 2b implementation discovers a Phase 4 routing technical-premise error. The shape preserves the routing-intent (the JSON-native-escape-design 4-domain convergence intent is preserved) while documenting the architectural-path change cleanly. Future similar sub-decisions in this project (Layer 4+) or sibling capstone projects should regression-check against this 5-element shape: discovery-surface + incorrect-premise + operator-authorization-mechanism + date + intent-vs-path framing.

What can't be evaluated from the audit-trail: whether the operator's main-session AskUserQuestion + ME sub-agent's prompt visible to me preserved the operator's authorization rationale beyond the commit-body sentence. The Phase 4 routing record's JSON-native escape design (per-domain Phase 4 appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md`) section was not amended with the architectural correction (the routing record still describes Path C as authored; the architectural correction lives only in `bfc0713`'s commit body + the DESIGN.md § bm export (Layer 3) Success-output paragraph). This is a small audit-trail-coherence gap — future readers comparing the routing record to the implementation will need to read the commit body to see the architectural correction. **Routes to operator-pattern-documentation**: the routing record should ideally be amended (post-commit) with a forward-only architectural-correction-note paragraph cross-referencing the `bfc0713` commit body — preserving the routing record as the canonical Phase 4 routing artifact while documenting the in-Phase-2b correction. Low-priority improvement; documented for future-cycle consideration.

**Resolution:** Architectural correction sub-decision is sound + auditable at the commit-body surface; the operator-authorization sub-decision pattern sets defensible precedent for future cycles. The 5-element shape (discovery-surface + incorrect-premise + operator-authorization-mechanism + date + intent-vs-path) is the canonical form. The routing-record-vs-commit-body audit-trail-coherence gap is a low-priority improvement documented for future-cycle consideration.

**Classification:** Resolved

---

<a id="r2-f6"></a>
**Finding 6 — Routing record's hook-bypass marker is hook-agnostic (bypasses ALL hooks, including `check-no-letter-clusters.py`); leaves residual `Path A/B/C` letter-label references in the routing record that the new hook would otherwise catch (Dim 11 + Dim 12)**

**Owner:** ai-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

**Validator rationale:** Dim 11 (audit-trail machine-readability cost — hook-discipline applied to the just-codified `check-no-letter-clusters.py` hook) + Dim 12 (operator-directive correction cost — the in-cycle hardening's hook does not catch the routing record's letter-label residue because the bypass marker pre-dates the hook + the bypass is hook-agnostic). Sanity-check verifies the hook-execution against the routing record empirically.

The Phase 4 routing record at per-domain Phase 4 routing appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md` (per-domain Phase 4 appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md`) opens with a hook-bypass marker at line 1: *`<!-- hook-bypass: this is a Phase 4 routing record per primers/4-feedback-integration.md § [manual] First-class fallback path, not a per-domain review log; the per-domain review-discipline hook's classification-section convention does not apply to routing records. -->`*. The bypass marker's rationale is scoped to the per-domain review-discipline hook's classification-section convention (the hook that requires `### Resolved` / `### Deferred` / `### Dismissed` headings — the routing record is not a per-domain review log, so the convention doesn't apply). But the [`hooks/check-no-letter-clusters.py` hook](../../../../vsdd-suite/hooks/check-no-letter-clusters.py) (added 2026-05-24 at `e4b6701`) implements bypass-detection via a generic `<!-- hook-bypass: ... -->` marker check in the first 5 lines of the file (see [hook source lines 142-149](../../../../vsdd-suite/hooks/check-no-letter-clusters.py)) — the bypass-marker-detection is hook-agnostic; ANY hook-bypass marker in the file head bypasses the new hook too.

Empirical hook-execution evidence (run during this session): `python3 vsdd-suite/hooks/check-no-letter-clusters.py vsdd-suite-reference-examples/bookmark-cli-manual/per-domain Phase 4 routing appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md` returns exit 0 (no violations) — the bypass marker is being honored. Grep evidence of the actual letter-label residue: `grep -nE "Path [A-Z]" vsdd-suite-reference-examples/bookmark-cli-manual/per-domain Phase 4 routing appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md` returns 8 matches across lines 14-18 + 28 + 51 + 72 + 94 — all references to operator-decision `Path A` / `Path B` / `Path C` labels (the AskUserQuestion option-naming pattern the new hook's forbidden-pattern `\bPath [A-Z]\b` regex specifically targets). The hook is designed to catch this exact pattern; the hook-bypass marker is silently preventing the catch.

The Phase 1a+1b commit `fdfa989` lines 60-62 (the routing record subsection): *"Letter-cluster shorthand sweep applied: cluster-letter section headers replaced with descriptive identifiers (JSON-native escape design, sorted-tag-comparison dedup, imported-tag control-char rejection, etc.) per the global Python sweep documented at the suite-side Review 94."* The sweep correctly replaced the cluster-letter SECTION HEADERS (e.g., `### Cluster B — JSON-native escape design` became `### JSON-native escape design`); but the operator-decision Path-letter INLINE REFERENCES (`Operator decision: Path C — switch...`) were NOT swept. The post-sweep state has descriptive section headers + letter-label-prefixed operator decisions — half-migration. The descriptive-name-first discipline (per the [hook source comment lines 39-41](../../../../vsdd-suite/hooks/check-no-letter-clusters.py): "Descriptive names with optional ordering suffixes ... descriptive identifier first, no opaque letter") is satisfied at the section headers but not at the operator-decision inline references.

The Dim 11 audit-trail machine-readability cost: a future reader (cold-session sub-agent at Layer 4+ reading the routing record) sees `Operator decision: Path C` and has no recovery path for what `Path C` means — the descriptive identifier IS in the section heading right above, but the letter-label residue requires the reader to re-scope back up to the section heading to disambiguate. The exact failure mode the [Review 94 Finding 3 partial-fix justification](../../../../vsdd-suite/suite-development/review-log/2026-05-24-suite-review.md#review-94--2026-05-25-0300z) targets: "a future reader (operator a month later; cold-session sub-agent reading the audit trail) sees `Cluster B` and has no recovery path — the letter carries no meaning at point of use".

The Dim 12 operator-directive correction cost: the bypass-marker-hook-agnostic interaction is the methodology-authoring issue — the [`hooks/check-no-letter-clusters.py` hook source lines 142-149](../../../../vsdd-suite/hooks/check-no-letter-clusters.py) implements bypass detection without per-hook namespacing; the [primer 4 § Routing output Cluster identifier discipline paragraph](../../../../vsdd-suite/primers/4-feedback-integration.md) names the hook + the discipline; but the bypass marker convention is shared across all hooks via the `<!-- hook-bypass: ... -->` syntax. A per-hook-namespaced bypass marker (e.g., `<!-- hook-bypass:check-no-letter-clusters: <rationale> -->` or `<!-- hook-bypass[check-no-letter-clusters]: <rationale> -->`) would let a file bypass one hook while still being subject to other hooks. The current shared-bypass-marker is a methodology-authoring gap that the in-cycle hardening at `e4b6701` did not catch — the hook adds the new enforcement but the bypass convention is unchanged, so the new hook is silently bypassable by ANY pre-existing bypass marker. The routing record is the first observable instance of this gap.

The fix shape (deferred to suite-side methodology-amendment cycle, not in-cycle): (a) extend the hook to namespace bypass markers — `<!-- hook-bypass:check-no-letter-clusters: <rationale> -->` specifically targets THIS hook; bare `<!-- hook-bypass: ... -->` doesn't bypass this hook; (b) OR: amend the routing record's bypass marker to be scoped to the per-domain-review-discipline hook only (the operator's mid-cycle fix would be to either narrow the bypass marker's wording OR sweep the `Path A/B/C` letter-labels to descriptive form). The earned-by-recurrence trigger: if a second hook-bypass-vs-new-hook interaction surfaces in a different project, the per-hook-namespacing methodology amendment fires; for this single instance, the in-project fix (sweep the letter-labels) is the right grain.

**Resolution:** Routing record's hook-bypass marker is hook-agnostic + silently bypasses the just-codified `check-no-letter-clusters.py` hook + 8 `Path A/B/C` letter-label residues remain. The fix at the in-project surface is to sweep the `Path A/B/C` references to descriptive identifiers (e.g., `Operator decision: switch to JSON-native escape syntax` rather than `Operator decision: Path C — switch to JSON-native escape syntax`); the routing record's section headers already carry the descriptive identifier so the inline operator-decision lines can drop the letter-label prefix without losing information. The suite-side methodology-amendment (per-hook-namespaced bypass markers) is deferred per the earned-by-recurrence trigger. Routes to operator's main-session collection step: sweep `Path A/B/C` in the routing record + optionally amend the bypass marker's wording to scope explicitly to the per-domain-review-discipline hook.

**Self-aware bypass acknowledgment.** This Round 2 entry itself adds a hook-bypass marker at file-top precisely to allow the discussion of the letter-label residues this finding flags (parallel to Review 94's own hook-bypass for the audit-trail discussion that necessarily quotes the offending labels). The hook's own header comment names "bypasses are themselves findings for the next registry-walk review" — this Finding 6 is the registry-walk-equivalent surfacing the hook-bypass-vs-new-hook-interaction concern explicitly; the bypass + finding are coupled. The self-reference instantiates the exact methodology-authoring tension the finding names: every file that needs to discuss letter-label defects must currently add a hook-agnostic bypass that opens the door to ALL hook-bypass interactions. The per-hook-namespaced bypass marker is the methodology-amendment that would close this self-reference loop.

**Classification:** Resolved

---

### Deferred

<a id="r2-f7"></a>
**Finding 7 — R1 F7 per-commit cost-tally gap: 4 Round 1 fix-work commits (`fdfa989` + `ba6a4a9` + `bfc0713` + `795bc25`) + the suite-hardening commit `e4b6701` carry no `**Cost-tally:**` lines; F7 continues as carry-forward (Dim 2)**

**Owner:** ai-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** sanity-check

**Validator rationale:** Dim 2 (token economy per finding) regression-check against [R1 F7](#r1-f7); sanity-check verifies the cost-tally-at-commit-message gap held across the Round 1 fix-work + suite-hardening commits.

The 4 Round 1 fix-work commits + the in-cycle suite-hardening commit `e4b6701` all carry `Co-Authored-By: Claude Opus 4.7 <noreply@anthropic.com>` trailers but do NOT carry `**Cost-tally:**` lines in their commit message bodies. Per the [R1 F7](#r1-f7) framing: the suite-side codification work for cost-tally landed at the per-Review-entry-preamble surface (per [`suite-development.md` § Per-review entry preamble § Cost-tally](../../../../vsdd-suite/suite-development/suite-development.md)) — NOT extended to the per-commit-message surface. The discipline's trigger ("per-commit-message convention extension lands at suite-side") has not fired in the Round-1-to-Round-2 transition; the gap holds.

The Round 1 + Round 2 fix-work + suite-hardening commit-body inspection:

- `fdfa989` (Phase 1a+1b, ~50 named DESIGN.md/CHANGELOG/README/PROCESS edits + 39 FINDINGS-INDEX rows): no `**Cost-tally:**`.
- `ba6a4a9` (Phase 2a, 6 new tests with per-test RED/GREEN verification output): no `**Cost-tally:**`.
- `bfc0713` (Phase 2b, 4 substantive impl fixes + the architectural correction): no `**Cost-tally:**`.
- `795bc25` (manual-tests/layer-3.md + Phase 2c annotation): no `**Cost-tally:**`.
- `e4b6701` (suite-hardening: hook + primer 4 amendment + Review 94): no `**Cost-tally:**`.

The carry-forward shape is identical to [R1 F7](#r1-f7) plus the suite-hardening commit which is the first AI-Engineer-surface commit (the meta-finding about the lettering recurrence is the substantive content + the methodology amendment is the disposition). The cost-tally for `e4b6701` would be high-leverage to capture — it's the methodology-amendment cost surface for the in-cycle hardening choice — but the convention is not codified at the commit-message layer yet, so the cost-tally evidence is not captured anywhere in the audit trail.

The second-cycle recurrence of the gap (Layer 2 R2 F5 → Layer 3 R1 F7 → Layer 3 R2 F7) is now the trigger for a suite-side methodology-amendment discussion per the [Review 92 Finding 5](../../../../vsdd-suite/suite-development/review-log/2026-05-24-suite-review.md#review-92--2026-05-24-0419z) cost-tally-surface AI-Engineer-owned earned-by-recurrence policy. The methodology-amendment options (deferred to suite-side discussion; not in-project amendment):

- **Path 1 — Extend cost-tally convention to per-commit-messages for AI-co-authored implementation/fix commits.** Cost: every AI-co-authored implementation commit gains a `**Cost-tally:**` block. Benefit: per-commit cost evidence captured at the granularity needed for Dim 2 expected-band tests. Risk: convention bloat per commit; operator-fatigue.
- **Path 2 — Codify the per-Review-entry-preamble surface as sufficient + dismiss the per-commit-message convention extension.** Cost: zero (the convention stays as-is). Benefit: minimal authoring overhead. Risk: cost-tally evidence remains aggregated at cycle-close granularity, not per-commit; per-commit cost-asymmetry (e.g., the architectural correction sub-decision at `bfc0713` cost more than `795bc25` did) is not observable.
- **Path 3 — Hybrid: per-cycle cost-tally at the suite-side review-log entry (which captures the aggregate per-cycle cost) + per-commit cost-tally OPTIONAL for high-leverage commits (architectural corrections; suite-hardening commits; first-of-pattern commits).** Cost: opt-in convention at the per-commit-message surface; consistent suite-side cost-tally per cycle. Benefit: high-leverage commits get cost capture; routine commits don't pay overhead. Risk: which commits qualify as high-leverage is a judgment call.

Path 3 matches the suite's existing opt-in pattern at the per-Review-entry-preamble surface ([`suite-development.md` § Cost-tally opt-in shape](../../../../vsdd-suite/suite-development/suite-development.md)). This finding routes the discussion to the suite-side methodology-amendment cycle (post this PR's merge) without prescribing a specific path — operator-decision needed.

**Resolution:** No project-side amendment required at this commit boundary; the suite-side methodology-amendment discussion fires per the earned-by-recurrence trigger (Path 1 vs Path 2 vs Path 3 decision pending operator-policy). This AI Engineer Round 2 entry carries its cost-tally per the codified per-field auditability tier (see Cost-tally section below); the Round 2 cycle-close cost-tally for the full 13-domain L3 IAR Round 2 lives at the cycle-close suite-side review-log entry.

**Classification:** Deferred — second-cycle-recurrence carries forward to suite-side methodology-amendment discussion. The trigger remains the operator-policy decision on Path 1 vs Path 2 vs Path 3.

---

### Dismissed

*(none)*

---

### Hallucinated

*(none)*

---

### Summary

7 substantive findings (6 Resolved + 1 Deferred) + 0 Dismissed + 0 Hallucinated.

**R1 F6 regression-check (pre-cycle methodology declaration closure):** Closed at the spawn-prompt-text surface per its Round 1 routing-text framing; partial-closure at the persistent-audit-trail surface — the discipline-intent requires a suite-side review-log entry naming the Round 2 pre-cycle declaration fields, which has not yet landed (no Review 95+ in `2026-05-24-suite-review.md`; no standalone `2026-05-25-bookmark-cli-manual-l3-iar-round-2.md`). Routes to operator's main-session collection step per [Finding 1](#r2-f1). The artifact-surface mismatch between F6's routing-text (closure satisfied) and the discipline's intent (closure not yet satisfied) is the load-bearing observation for future cycle launches.

**R1 F7 regression-check (per-commit cost-tally gap):** Carry-forward holds across the 4 Round 1 fix-work commits + the suite-hardening commit `e4b6701`; second-cycle-recurrence trigger for suite-side methodology-amendment discussion per [Finding 7](#r2-f7). Path 1 vs Path 2 vs Path 3 decision pending operator-policy.

**Round 2 spawn shape evaluation (Dim 7):** Per-domain 13-agent shape preserves adversarial-pair separation by construction; cluster-letter labels absent in spawn-prompt-to-this-sub-agent per [Finding 2](#r2-f2). The per-domain-vs-cluster-batching choice rationale (cost-tradeoff calibration per layer) should be named in future pre-cycle declarations.

**Round 2 scope-reducer framing evaluation (Dim 8):** Spawn prompt frames scope as canonical Dim 8 scope-reducer with Round 1 closure context included; 4 sibling Round 2 entries show consistent scope-reducer framing per [Finding 3](#r2-f3).

**In-cycle suite-hardening evaluation (Dim 9 + Dim 12):** `e4b6701` is within Dim 9 acceptable scope at the 4th-recurrence trigger + operator-authorization sub-decision is documented + cost-asymmetry favors the in-cycle authorization per [Finding 4](#r2-f4). Future-cycle generalization observation (operator-authorization sub-decision pattern requires explicit trigger-naming) documented.

**Architectural correction sub-decision evaluation (Dim 12):** Sound + auditable at the `bfc0713` commit-body surface; operator-authorization sub-decision pattern sets defensible precedent (the 5-element shape: discovery-surface + incorrect-premise + operator-authorization-mechanism + date + intent-vs-path) per [Finding 5](#r2-f5). The routing-record-vs-commit-body audit-trail-coherence gap is a low-priority improvement documented.

**Hook-bypass-vs-new-hook interaction evaluation (Dim 11 + Dim 12):** Routing record's hook-bypass marker is hook-agnostic + silently bypasses the just-codified `check-no-letter-clusters.py` hook + 8 `Path A/B/C` letter-label residues remain per [Finding 6](#r2-f6). In-project fix (sweep `Path A/B/C` to descriptive identifiers) is the right grain; suite-side methodology-amendment (per-hook-namespaced bypass markers) deferred per the earned-by-recurrence trigger.

**Cite-verify discipline evaluation (Dim 11 sub-clause):** Round 1 fix-work commits carry cite-verifiable cross-references — `fdfa989` cites the routing record at per-domain Phase 4 routing appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md`; `bfc0713` cites the routing record + the architectural correction context; `795bc25` cites the routing record + Layer-gate criteria. `e4b6701` cites Review 94 + the prior letter-label recurrences (PR #38, PR #44, PR #52) + the upstream VSDD whitepaper WebFetch evidence. All cited references resolve in their cited locations (spot-checked during this session). No cite-drift detected at the fix-work commit surface.

**MVR signal:** PARTIAL at Round 2. Per the [primer 3 § Round triggers G-131 continue-trigger](../../../../vsdd-suite/primers/3-review-session.md): this round produced 7 substantive findings (6 Resolved + 1 Deferred). The Resolved findings document the Round 2 cycle's empirical disciplines; the Deferred finding ([F7](#r2-f7)) is the persistent per-commit cost-tally gap carry-forward that does NOT trigger an AI Engineer Round 3 by itself (the per-Review-entry-preamble cost-tally surface is the codified granularity; per-commit extension is suite-side methodology discussion). Per the [primer 3 § Round triggers G-151 stop-trigger](../../../../vsdd-suite/primers/3-review-session.md): no AI Engineer Round 3 is mandated by AI Engineer's own findings; whether the full 13-domain Layer 3 IAR cycle continues to Round 3 depends on whether OTHER domains produced new findings at Round 2 (operator-confirmable at the Round 2 cycle-close cross-domain MVR check).

**Coordination:** [Finding 1](#r2-f1) routes to operator's main-session collection step (Round 2 pre-cycle declaration as suite-side review-log entry); [Finding 6](#r2-f6) routes to operator's main-session collection step (sweep `Path A/B/C` in routing record + optionally amend the bypass marker's wording); [Finding 7](#r2-f7) routes to suite-side methodology-amendment discussion (Path 1 vs Path 2 vs Path 3 decision). The 4 other Resolved findings (F2 + F3 + F4 + F5) document operative disciplines for future-cycle regression-check; no immediate suite-side amendment until similar instances surface in sibling capstone projects (earned-by-recurrence per [Review 92 Finding 3 Path 2](../../../../vsdd-suite/suite-development/review-log/2026-05-24-suite-review.md#review-92--2026-05-24-0419z)).

---

**Cost-tally:** (per [`suite-development.md`](../../../../vsdd-suite/suite-development/suite-development.md) § Per-review entry preamble § Cost-tally + [primer 3 § Cost-tally report shape](../../../../vsdd-suite/primers/3-review-session.md) + § Per-field auditability tier; field numbering per [primer 3 § Cost-tally report shape](../../../../vsdd-suite/primers/3-review-session.md) 10-field schema)

**Agent-self-verifiable fields (countable from this session's tool-call log):**

1. **AI tool:** claude-code CLI (per the prompt's "Main session collects + commits" framing + the sub-agent execution model)
3. **Execution method:** sub-agent spawn from operator's main session (this prompt invocation is the sub-agent execution; operator's main session orchestrates the 13-domain L3 IAR Round 2 cycle)
4. **Model:** claude-opus-4-7 (per the project's [DESIGN.md § Cold-session budget](../../DESIGN.md) line 19 declaration — Opus 4.7 for AI Engineer)
- **Tool-call counts:** 9 Read invocations (Round 1 file; AI Engineer domain prompt; primer 3; routing record; check-no-letter-clusters hook; primer 4; claude-code-cli supplement; suite-development.md preamble; phase-4-routing routing record); 13 Bash invocations (Bash `date -u` session-start anchor; Bash `date -u` session-end anchor; `git log` × 5 for commit-body inspection; `grep` × 4 for cross-reference verification; `find` + `ls` × 2 for directory layout discovery; hook execution × 2); 1 Edit (this file).
- **Files read:** 9 source files totaling ~4,500 lines read (full reads on domain prompt + primer 3 + primer 4 + hook + claude-code-cli supplement + routing record; offset-reads on suite-development.md + Round 1 entry).
- **Mechanical sweeps run:** 4 greps (`Round 2|pre-cycle|13 cold-session|spawn shape`; `Cluster [A-Z]|Surface [A-Z]|Path [A-Z]|Option [A-Z]` across routing record + all 2026-05-24 review-log files; `^## Review` heading sweep across 4 sibling Round 2 files; `Cost-tally|per-commit` across suite-development.md).
- **Wall-clock anchors:** Session-start Bash `date -u` = 2026-05-25T03:06Z; Session-end Bash `date -u` = 2026-05-25T03:09Z.
- **Wall-clock elapsed (Field 9):** ~3 minutes (Bash-instrumented per the [`claude-code-cli.md` § Wall-clock measurement pattern](../../../../vsdd-suite/supplements/claude-code-cli.md); agent did NOT count time between tool calls; gaps include operator-discussion intervals + idle periods + tool execution time + agent authoring time, in unknown proportions). Honest framing: this is wall-clock elapsed time, NOT agent-active time.

**Operator-verifiable fields (requires operator `/cost` paste; operator-action queue):**

5. **Raw tokens:** *pending operator `/cost` paste*
6. **Would-be API cost:** *pending operator `/cost` paste*
8. **Rate-limit-window utilization:** *pending operator `/cost` paste*

**Operator-confirmable fields (operator-declared per session; NOT inherited from prior context):**

2. **Plan tier:** *pending operator confirmation* (do NOT inherit from prior cycle context per the [primer 3 § Per-field auditability tier](../../../../vsdd-suite/primers/3-review-session.md) hard rule)
7. **Actual cost to operator:** *pending operator declaration of plan tier — `$0 marginal (within plan limits)` if subscription plan; would-be-API-cost if API-direct*

**Derived metric (computable only when all inputs measured):**

10. **Findings/100k tokens:** NOT COMPUTABLE — pending operator `/cost` paste (Field 5 unmeasured). Naive comparator if raw-tokens were ~150k for this session: 7 findings / 150k = ~4.7 findings/100k. Capstone expected band per [`AI-ENGINEER-REVIEW.md` Dim 2](../../../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md): 100k–300k tokens/finding = 0.33–1.0 findings/100k. **Caveat: comparator is illustrative only — Field 5 unmeasured; agent CANNOT compute this metric per the hard rule against fabrication.**

**Round 2 scope-reducer cost-comparison observation (Operator-action queue addressable):** The prompt framing "Round 2 is the scope-reducer it was framed as?" requires comparing Round 2 wall-clock + tokens against Round 1's empirical cost. This sub-agent CANNOT compute the comparison (Round 1's wall-clock + tokens are operator-verifiable from `/cost` paste at Round 1 close; this sub-agent did NOT have access to that paste). Honest framing: Round 2's wall-clock elapsed for THIS sub-agent (~3 min Bash-anchored) IS substantially shorter than Round 1's wall-clock per the Round 1 cost-tally section's session-end anchor at 2026-05-25T01:15Z (Round 1 session-start anchor was not captured per the Round 1 cost-tally's own honest limitation, so the Round-1-elapsed figure is itself NOT COMPUTABLE). The scope-reducer hypothesis (Round 2 produces fewer findings per token than Round 1; Round 2 verifies prior findings hold + surfaces a smaller residual set) is consistent with the 7-vs-7 finding count (Round 1 = 7 findings; Round 2 = 7 findings) — the count alone does NOT support the scope-reducer hypothesis; the per-finding cost decomposition would be needed for the claim, which is operator-verifiable.

**Mining of the Round 1 cycle (Operator-action queue addressable):** The prompt frames "Round 1 cycle had ~76 findings across 13 agents × ~5-8 minutes wall-clock each. Per-finding cost falls in what band?" This sub-agent CANNOT compute the per-finding band from agent-self-verifiable fields alone — the per-agent token consumption is operator-verifiable. Operator-action queue: if the operator pastes per-agent `/cost` outputs from the 13 Round 1 spawns, this AI Engineer Round 2 entry can be amended (append-only addendum) with the actual band calculation. Naive comparator if per-agent tokens were ~100-200k: 76 findings / (13 × ~150k tokens) = ~3.9 findings/100k → well above the capstone expected-band floor (0.33-1.0 findings/100k) — would read as efficient adversarial review running below the band. **Comparator is illustrative only; the actual band is operator-verifiable.**

**Operator-action queue:** if cost-tally precision is load-bearing for cross-cycle calibration (specifically: Round 1 vs Round 2 cost-comparison; per-finding band against the DESIGN.md cold-session-budget declared band of 100k–300k tokens/finding; the Round-2-as-scope-reducer empirical evaluation), operator runs `/cost` in this session AND retrieves per-agent `/cost` outputs from the 13 Round 1 spawns + the 12 sibling Round 2 spawns + this Round 2 AI Engineer spawn, pastes the outputs here as an append-only addendum, replacing the *pending operator …* placeholders with measured values + the Round 1 vs Round 2 comparator calculation. Operator also confirms plan tier (Claude Max / Claude API direct / etc.) so Field 2 + Field 7 + Field 8 can be filled.

**Honest agent-self-assessment of this cost-tally:** the 4 agent-self-verifiable fields filled with hard counts above (plus the Bash-anchored wall-clock at Field 9) are this agent's honest measurement; the 6 operator-verifiable/confirmable/derived fields are correctly placeholdered per the [Review 91 Finding 8](../../../../vsdd-suite/suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) hard-rule against fabrication. The Round 2 scope-reducer empirical-evaluation + the Round 1 cycle mining are both operator-verifiable computations the sub-agent flags as the load-bearing operator-action-queue items.

---

## Phase 4 routing — Round 1 (2026-05-25 02:00Z)

Per [`vsdd-suite/primers/4-feedback-integration.md`](../../../../vsdd-suite/primers/4-feedback-integration.md) § [manual] First-class fallback path. SO-decisions captured via main-session AskUserQuestion pass on 2026-05-25 across the cross-domain finding clusters. This appendix lists this domain's routable findings in the primer-4-canonical per-finding shape; cross-domain coordination signals live in each Round 1 finding's `**Coordination:**` line. Cross-cluster sequencing matrix lives in the commit message + the CHANGELOG slim-form entry that recorded this Phase 4 pass (refactored from a prior consolidated routing record per operator directive 2026-05-25 — the consolidated file was an anti-pattern; primer-4-canonical is per-domain appendices).

#### Finding `r1-f6` — Pre-cycle methodology declaration absent for 13-domain Layer 3 IAR Round 1 — ROUTED

**Cluster:** AI-Engineer process-discipline carry-forward
**Route:** `Phase 4 itself (process discipline)`
**Gate:** Round 2 launch includes the pre-cycle methodology declaration per primer 3 § Pre-cycle methodology check Path 2; Validator: AIE
**Sequencing:** Folded into Round 2 work; does not block Layer 3 gate close

#### Finding `r1-f7` — Per-commit-message cost-tally gap carry-forward from R2 F5 + R3 F5 — ROUTED

**Cluster:** AI-Engineer process-discipline carry-forward
**Route:** `Phase 4 itself (process discipline)`
**Gate:** Carry-forward across PRs; document operator-time commitment in TODO.md; Validator: AIE
**Sequencing:** Process improvement; not Round 1 fix work


---

## Phase 4 routing — Round 2 (2026-05-25 07:30Z)

Per [`vsdd-suite/primers/4-feedback-integration.md`](../../../../vsdd-suite/primers/4-feedback-integration.md) § [manual] First-class fallback path. SO-decisions for substantive routings captured via main-session AskUserQuestion pass on 2026-05-25 (empty-string tag rejection consistency; tests/scaling.rs Phase 5 sentinel addition; Round 3 verification mini-cycle for the hallucination cluster). Verification evidence for `Hallucinated` dispositions: Round 3 PFE + QE + SE + UX cold-session re-spawn (per-domain Review N+1 entries authored 2026-05-25).

#### Finding `r2-f1` — R1 F6 pre-cycle methodology declaration: visible to sub-agent — RESOLVED-NO-FINDING

**Disposition:** Resolved-no-finding
**Evidence:** Process discipline confirmation: Round 2 launch prompt included the declaration; AIE R1 F6 closed.

#### Finding `r2-f2` — Round 2 13-agent per-domain spawn preserves adversarial separation — RESOLVED-NO-FINDING

**Disposition:** Resolved-no-finding
**Evidence:** Methodology confirmation: per-domain cold-session shape preserves adversarial-pair separation by construction.

#### Finding `r2-f3` — Round 2 spawn prompt frames scope as verification — RESOLVED-NO-FINDING

**Disposition:** Resolved-no-finding
**Evidence:** Methodology confirmation: scope-reducer per AIE R1 F8 applied correctly.

#### Finding `r2-f4` — In-cycle suite-hardening commit scope discipline — RESOLVED-NO-FINDING

**Disposition:** Resolved-no-finding
**Evidence:** Process confirmation: in-cycle hook landing did not destabilize the cycle.

#### Finding `r2-f5` — Architectural correction sub-decision (display_safe removal beyond Round 1 spec) — RESOLVED-NO-FINDING

**Disposition:** Resolved-no-finding
**Evidence:** Process confirmation: operator-authorized mid-Phase-2b architectural correction with explicit sub-decision recording.

#### Finding `r2-f6` — Routing record hook-bypass marker hook-agnostic — CARRY-FORWARD-TO-SUITE-HARDENING

**Disposition:** Carry-forward-to-suite-hardening
**Evidence:** Per-hook bypass mechanism queued at task #41; AIE R2 F6 routing target for the next suite PR.

#### Finding `r2-f7` — R1 F7 per-commit cost-tally gap: 4 Round 1 fix-work commits — CARRY-FORWARD

**Disposition:** Carry-forward
**Evidence:** Cost-tally gap persists; operator-time documentation in TODO.md remains the agent-self-verifiable path. Process improvement; not fix work.
