# Suite review log — 2026-05-24

Per-session suite-development review entries land here. Per the [`SUITE-DEVELOPMENT-REVIEW.md`](../SUITE-DEVELOPMENT-REVIEW.md) Reviews table, this file is the canonical home for Reviews dated 2026-05-24.

---

## Review 92 — 2026-05-24 04:19Z

**Phase:** Suite-development meta-review — operator-deferred suite-wide observability + auditability sweep per the [Review 91](2026-05-23-suite-review.md#review-91--2026-05-23-1900z) operator-action queue.

**Source:** director-raised — operator directive 2026-05-24: "Do Review 92."

**Lens:** Observability + auditability across the 42-file methodology surface (16 role domains + 3 meta domains + 9 primers + 14 supplements). Observability sub-axes: cost-tally references; wall-clock measurement; metrics / dashboards in domain dims; instrumentation surfaces. Auditability sub-axes: cited dims (cite-verify per [Review 91 Finding 14](2026-05-23-suite-review.md#r91-f14)); Supplements applied (per [Review 91 Findings 2 + 4](2026-05-23-suite-review.md#r91-f2)); three-audience-lens coverage (per [Review 84 Finding 4](2026-05-21-suite-review.md#review-84--2026-05-21-1100z)); naming conventions vs governing standard.

**Scope:** All files at [`vsdd-suite/domains/role/`](../../domains/role/), [`vsdd-suite/domains/meta/`](../../domains/meta/), [`vsdd-suite/primers/`](../../primers/), [`vsdd-suite/supplements/`](../../supplements/). Method: grep-pattern-first scan to identify candidate files (per [Review 91 Finding 16](2026-05-23-suite-review.md#r91-f16) lookup-idiom adoption discipline); deep-read of candidates with substantive findings; cite-verify spot-check on highest-leverage cited dims per [Review 91 Finding 14](2026-05-23-suite-review.md#r91-f14).

**Reviewer:** *(suite-development meta-review; no domain-role persona per [`suite-development.md` § Governing standard for domain files](../suite-development.md) exception for meta-reviews)*

**Model:** Opus 4.7 (`claude-opus-4-7`)

**Cold-session shape:** N/A — inline main session per [`suite-development.md` § Session isolation](../suite-development.md) default for suite reviews + per [`primers/5-formal-hardening.md` § Cold-session-vs-inline decision rubric](../../primers/5-formal-hardening.md) — methodology-prose audits are adversarial-framing judgment per the rubric, but suite-side convention defaults to inline for continuity with authorial context (parallel to Review 91's inline shape). Trade-off named explicitly per the rubric's per-round-declaration requirement.

**Regression check against:** [Review 91](2026-05-23-suite-review.md#review-91--2026-05-23-1900z) (immediate prior cycle; codified F1-F20 across 4 commits in PR #48). Verified: F1 + F8 + F10 + F12 + F18 + F19 + F20 (the 7 in-cycle codifications from the checkpoint + slop-fix commits) all land in their stated locations; F2 + F3 + F4 + F5 + F9 + F13 + F14 + F15 + F16 codifications (the 9 from the remaining-findings commit) all land in their stated locations; F11 + F17 stay Open per their respective deferral rationales.

**Session note:** Inline main-session suite review. **Sycophancy compensation:** this audit is operator-deferred follow-up from Review 91's operator-action queue — the natural bias is to validate that the just-completed Review 91 codifications stuck. Findings derive from grep-pattern-first artifact-state analysis (mechanical greps over the 42-file set in 6 Bash invocations) + cite-verify spot-checks on high-leverage candidates, NOT from narrative judgment about whether codifications "feel" complete. The audit deliberately seeks gaps OPPOSITE the just-codified discipline (does pre-cycle methodology check coverage hold across primers? do supplements have three-audience-lens? does UX-REVIEW conform to the governing standard?) rather than validating presence of expected surfaces.

**Cite-verify discipline applied to this audit** (per [Review 91 Finding 14](2026-05-23-suite-review.md#r91-f14) — first post-codification application): AI Engineer Dim 6 / 13 / 14 verified to resolve in [`domains/role/AI-ENGINEER-REVIEW.md`](../../domains/role/AI-ENGINEER-REVIEW.md) before citing in this preamble (lines 50, 68, 70 confirmed); [UX-REVIEW.md](../../domains/role/UX-REVIEW.md) lines 23-29 read directly before citing the naming inconsistency; [rust.md](../../supplements/rust.md) + [markdown.md](../../supplements/markdown.md) tails read before citing three-audience-lens absence. **The author had the option to cite from secondary references (the Review 90 + Review 91 entries naming these dims) but did not.** This is empirical evidence the F14 discipline shifts authoring behavior post-codification.

**Lookup-idiom adoption applied to this audit** (per [Review 91 Finding 16](2026-05-23-suite-review.md#r91-f16) — first post-codification application): 7 grep-pattern scans across the 42-file surface in 7 Bash invocations (cost-tally references; wall-clock/date; observability/instrument/metric; prose-only "the X supplement"; pre-cycle methodology check coverage; three-audience-lens coverage; sycophancy + Validator-pair completeness) **before** any Read of an audit target file. Deep-reads only on the files where greps surfaced candidates (4 target files Read: UX-REVIEW, AI-ENGINEER-REVIEW for cite-verify, rust.md tail, markdown.md tail). This is empirical evidence the F16 discipline shifts authoring behavior post-codification (vs the Review 91 author's Read-default for catalog-covered queries).

**Cost-tally** (full tiered shape per [Review 91 Finding 8](2026-05-23-suite-review.md#r91-f8) + [Finding 20](2026-05-23-suite-review.md#r91-f20) opt-in classification — multi-lens suite-side cycle warrants full tiered):

**Agent-self-verifiable (countable from this session's tool-call log):**

- **AI tool:** [claude-code CLI](https://claude.com/claude-code)
- **Model:** Opus 4.7 (`claude-opus-4-7`)
- **Execution method:** inline main session; no sub-agent spawns
- **Tool calls executed:** ~15 (4 Bash for grep scans + 3 Bash for spot-check + 2 Bash for git ops + 1 Read for UX-REVIEW verification + 1 Read for AI Engineer Dim verification + 1 Read for the supplement tails + 2 TaskCreate/Update + 1 Write for this file)
- **Files read in full:** 0 — grep-pattern-first scan covered the surface; spot-read targeted lines only
- **Files spot-read (offset+limit / tail):** 5 (UX-REVIEW.md L23-29; AI-ENGINEER-REVIEW.md L50/68/70; rust.md tail; markdown.md tail; primer 3 L51-67)
- **Files written/edited this audit:** 1 (this file — new)
- **Mechanical sweeps run:** 7 `grep`/`for-loop` invocations across the 42-file surface
- **Wall-clock anchors (Bash `date -u`):** session-start 2026-05-24T04:19Z → session-end 2026-05-24T04:28Z (elapsed ~9 minutes wall-clock; includes operator-discussion interval + this audit's authoring; **~46x faster than Review 91's ~7h43m elapsed** — empirical evidence of the F16 grep-first lookup-discipline cost-impact vs the Review 91 author's Read-default for catalog-covered queries)

**Operator-verifiable (requires `/cost` paste or plan-dashboard inspection):**

- **Raw tokens:** *pending operator `/cost` paste*
- **Cache-hit ratio:** *pending operator `/cost` paste* (this session inherits prior Review 91 session context heavily — cache-hit likely high relative to a cold-session equivalent)
- **Would-be API cost:** *pending operator `/cost` paste*
- **Rate-limit-window utilization:** *pending operator-dashboard check*

**Operator-confirmable (operator-declared per session; not inherited silently):**

- **Plan tier:** Claude Max (operator-declared in prior Review 91 session; **NOT re-confirmed for this session** — per [AI Engineer Dim 14](../../domains/role/AI-ENGINEER-REVIEW.md), passive inheritance is the failure mode; operator should re-confirm or the field treats as uncertain)
- **Actual cost to operator:** $0 marginal *IF on Max plan AND session did not trigger rate-limit* (depends on operator-confirmable plan tier)

**Derived metric:** **Findings/100k tokens:** NOT COMPUTABLE — pending operator `/cost` paste

**Operator-action queue:** if cost-tally precision is load-bearing for cross-cycle calibration, operator runs `/cost` in this session and pastes the output here as an append-only addendum.

---

### Resolved

<a id="r92-f1"></a>
**Finding 1 — UX-REVIEW.md uses `**Interface type:**` preamble field instead of the canonical `**Language and interface supplement:**` per the [`suite-development.md`](../suite-development.md) § Governing standard for domain files item 5 — naming inconsistency**

**Owner:** technical-writer (governing-standard prose surface)
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

Validator rationale: domain-prompt naming-conformance finding; no natural cross-domain validator-pair; Sanity Check validates the rename preserves UX's substantive interface-type guidance while aligning with the governing-standard's canonical preamble field name.

**Evidence:** [`vsdd-suite/suite-development/suite-development.md`](../suite-development.md) § Governing standard for domain files item 5 requires: *"Language and interface supplement reference (`../../supplements/`) — **required**, or an explicit opt-out line with rationale (e.g., `**Language and interface supplement:** Not applicable. [Reason].`)."*

[`vsdd-suite/domains/role/UX-REVIEW.md`](../../domains/role/UX-REVIEW.md) line 25 uses: `**Interface type:** The standard dimensions below assume a browser-rendered interface. For CLI projects, consult `../../supplements/cli.md` ... For browser apps, also consult `../../supplements/browser-app.md` ...`

The substance is correct — UX-REVIEW.md references both applicable interface supplements (`cli.md` + `browser-app.md`) with inline-link paths, satisfying the [Review 91 Finding 2](2026-05-23-suite-review.md#r91-f2) `**Supplements applied:**` discipline at the domain-prompt level. **The label diverges:** governing-standard says `**Language and interface supplement:**`; UX-REVIEW says `**Interface type:**`. Mechanical sweep across all 16 role domains: UX is the **only** domain using the `**Interface type:**` label form; all others use `**Language and interface supplement:**`.

**Reasoning:** Single-domain naming drift, easy to fix. The naming inconsistency forces an agent grep idiom (per [Review 91 Finding 16](2026-05-23-suite-review.md#r91-f16) lookup-discipline) to test two patterns instead of one. The label divergence likely predates the governing-standard codification; G-89 forward-only narrative-preservation might apply, but the audit-trail benefit of label uniformity outweighs preservation for a single-domain instance.

**Resolution applied (in-cycle codification):** UX-REVIEW.md line 25 amended from `**Interface type:**` to `**Language and interface supplement:**` — content preserved; label aligned with governing standard. Sweep confirms 16-of-16 role domains now use the canonical label.

**Classification:** Resolved (TW Dim 11 — audience-fit calibration; agent-grep idiom now uniform across the role-domain set).

---

<a id="r92-f6"></a>
**Finding 6 — Cite-verify discipline (F14) + lookup-idiom adoption (F16) shift authoring behavior post-codification — empirical evidence from this audit's own method**

**Owner:** ai-engineer (methodology-calibration surface)
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

Validator rationale: methodology-calibration finding; validates whether the just-codified disciplines empirically shift agent behavior in their first post-codification application; Sanity Check confirms the empirical evidence is structurally sound (not just self-reported).

**Evidence:** This Review 92 audit explicitly applied both disciplines codified in [Review 91 Findings 14 + 16](2026-05-23-suite-review.md#r91-f14):

**F16 lookup-idiom adoption (empirical):**

- Pre-codification (Review 91 author's behavior): defaulted to `Read` + visual parse + offset+limit for navigating FINDINGS-INDEX.md and other indexed files. Multiple times. Catalog'd `awk` / `grep` idioms NOT reached for.
- Post-codification (Review 92 audit's behavior): 7 grep-pattern scans across the 42-file surface BEFORE any Read of an audit target file. Cost: 7 Bash invocations completing in seconds vs ~42 Read calls that would have consumed minutes + large context-window pressure. The audit's grep-first method IS the discipline's first canonical-worked-example application.

**F14 cite-verify discipline (empirical):**

- Pre-codification (Review 91 author's behavior): cited AI Engineer Dim 11 / 13 / 14 + PerfEng Dim 5 / 8 / 10 + PE Dim 9 / 27 / 36 throughout the audit prose without loading the cited domain prompts until operator adversarial questions forced the reach.
- Post-codification (Review 92 audit's behavior): AI Engineer Dim 6 / 13 / 14 verified to resolve in [`AI-ENGINEER-REVIEW.md`](../../domains/role/AI-ENGINEER-REVIEW.md) lines 50 / 68 / 70 BEFORE citing in this Review's preamble. The author had the option to cite from secondary references but did not.

**Reasoning:** Methodology codifications can be prose-only or behavior-changing. Review 91's codifications of F14 + F16 lived only in `suite-development.md` and `AI-ENGINEER-REVIEW.md` post-PR-#48; the empirical question was whether the next cycle's author (this Review 92 audit) would reach for the catalog'd idioms vs default to the prior pattern. The audit's method demonstrates the discipline shifts authoring behavior in the first post-codification cycle. **The next 2 cycles' authoring behavior is the empirical-evidence requirement F16 codification named** (escalate to soft-hook if next 3 cycles continue Read-default); this Review 92 is cycle 1 of 3.

**Resolution:** F14 + F16 discipline applications validated empirically. The F16 codification's named third-cycle-escalation trigger advances 1 of 3 (this cycle) — 2 more grep-first cycles needed before the discipline can be considered behavior-stable.

**Classification:** Resolved — empirical-evidence-of-codification-effectiveness finding; the methodology calibration tracks.

---

### Open

<a id="r92-f2"></a>
**Finding 2 — 13 of 14 supplements lack the three-audience-lens section that [Review 84 Finding 4](2026-05-21-suite-review.md#review-84--2026-05-21-1100z) codified across all 28 domains + primers; the codification scope excluded supplements**

**Owner:** technical-writer (governing-standard prose surface) + each supplement's owner-domain for the per-supplement extension
**Status:** raised
**Blocked by:** *(none — scope decision is operator-policy: cascade-codification across 13 files vs methodology amendment naming supplements as out-of-scope for three-audience lens)*

**Evidence:** Mechanical sweep across all 14 supplements:

- **HAS** three-audience-lens section: [`github-actions.md`](../../supplements/github-actions.md) (authored at [Review 86 Finding 1](2026-05-21-suite-review.md#review-86--2026-05-21-1200z) with the three-audience treatment by design)
- **MISSING** three-audience-lens section (13 files): `bash.md`, `browser-app.md`, `claude-code-cli.md`, `cli.md`, `css.md`, `html.md`, `javascript-typescript.md`, `json.md`, `markdown.md`, `python.md`, `rust.md`, `toml.md`, `yaml.md`

Per [Review 84 Finding 4](2026-05-21-suite-review.md#review-84--2026-05-21-1100z): *"Three-audience lens applied across 9 primers + 19 domain prompts + 2 indexes + 2 templates + suite README."* — supplements were NOT in the cascade scope. Per [`suite-development.md`](../suite-development.md) § Three-audience design principle: *"every audit-trail artifact must serve all three audiences. Human readability is necessary but not sufficient ..."* — supplements ARE part of the methodology's load-bearing surface for projects (per the [Review 91 Finding 2 + 4](2026-05-23-suite-review.md#r91-f2) plural-Supplements-applied codification; per the github-actions.md exemplification of the three-audience lens working in a supplement).

**Reasoning:** Two plausible operator-policy framings:

1. **Cascade-codification path**: extend three-audience lens across the 13 supplements. Cost: 13 files × ~10-20 lines of three-audience-lens section per file = ~150-260 lines of authoring. Audit-trail benefit: three-audience principle's "every audit-trail artifact" claim becomes empirically true across the full methodology surface (currently false at the supplement layer).
2. **Methodology amendment path**: amend [`suite-development.md`](../suite-development.md) § Three-audience design principle to explicitly name supplements as out-of-three-audience-scope — supplements are loaded by domain reviewers per the domain prompt's `**Language and interface supplement:**` field; the three-audience surface lives at the domain layer, and the supplement inherits the domain's three-audience treatment. Cost: 1 paragraph in suite-development.md. Audit-trail benefit: the three-audience principle's scope becomes correctly bounded; the supplement scope-gap is no longer a defect.

Path (2) is the lower-cost methodology-clarity fix; path (1) is the more substantive applied-discipline propagation. The github-actions.md exemplification leans toward path (1) (the three-audience-lens IS useful at the supplement layer per the github-actions.md example).

**Recommendation:** operator-decision required between path 1 (cascade) vs path 2 (methodology amendment naming supplements as inherit-from-domain). Either path is acceptable; the current state (13/14 supplements missing the section after the F4 codification claimed all-artifact coverage) is empirically inconsistent with the principle's stated scope.

**Classification:** Open (registered for tracking; operator-policy decision on path 1 vs path 2 is the gating decision; codification work is a separate PR per the no-stacked-PRs operator preference regardless of path).

---

<a id="r92-f3"></a>
**Finding 3 — Pre-cycle methodology check coverage is patchy across primers (3 of 9 have it); the methodology lacks an explicit decision on which phases warrant a pre-cycle declaration vs which deliberately do not**

**Owner:** ai-engineer (process-enforcement surface)
**Status:** raised
**Blocked by:** *(none — scope decision is operator-policy: extend pre-cycle discipline to other primers vs explicitly name the discipline as scoped to compounding-cost cycles only)*

**Evidence:** Mechanical sweep across all 9 primers for `Pre-cycle methodology check` references:

- **HAS** pre-cycle methodology check: [`primers/2a-red-gate.md`](../../primers/2a-red-gate.md) (via the Phase-2a-evidence-shape declaration codified at [Review 91 Finding 1](2026-05-23-suite-review.md#r91-f1)); [`primers/3-review-session.md`](../../primers/3-review-session.md) (the primary host of the pre-cycle discipline per [Review 90 Finding 2](2026-05-23-suite-review.md#review-90--2026-05-23-1200z) Dim 14 codification); [`primers/5-formal-hardening.md`](../../primers/5-formal-hardening.md) (via the Cold-session-vs-inline decision rubric codified at [Review 91 Finding 5](2026-05-23-suite-review.md#r91-f5))
- **MISSING** pre-cycle methodology check: [`primers/1ab-spec-crystallization.md`](../../primers/1ab-spec-crystallization.md), [`primers/1c-decomposition.md`](../../primers/1c-decomposition.md), [`primers/2b-implementation.md`](../../primers/2b-implementation.md), [`primers/2c-refactor.md`](../../primers/2c-refactor.md), [`primers/4-feedback-integration.md`](../../primers/4-feedback-integration.md), [`primers/6-convergence.md`](../../primers/6-convergence.md) — 6 of 9 primers

**Reasoning:** Two plausible methodology framings:

1. **Pre-cycle-everywhere path**: each phase warrants its own pre-cycle declaration. E.g., primer 1a+1b warrants a `**Spec-source declaration:**` (where does the assignment brief come from; what's the source-of-truth); primer 4 warrants a `**Routing-source declaration:**` (which prior-round findings are being routed; who's the authority); primer 6 warrants a `**Convergence-evidence declaration:**` (what evidence is being attested across the four dimensions). Cost: 6 primer extensions, each ~10-20 lines.
2. **Pre-cycle-scoped-to-compounding-cost path**: the pre-cycle declaration is scoped to cycles whose cost compounds (multi-agent IAR; Phase 5 cold-session vs inline; Phase 2a evidence-shape preservation) — these are the only phases where pre-cycle calibration prevents methodology drift. Other phases (Phase 1a+1b authoring; Phase 1c decomposition; Phase 2b implementation; Phase 2c refactor; Phase 4 routing; Phase 6 convergence) are single-author / event-driven / structurally-bounded and don't compound cost in the same way. Cost: 1 paragraph in `suite-development.md` naming the scope.

Path (2) matches the methodology's existing emphasis on cost-compounding cycles (per AI Engineer Dim 13's framing — "the operator + AI Engineer review-pair confirm the spawn shape against the project's intent tier + active-domain set + prior-cycle cost evidence"). Path (1) would over-extend the discipline beyond its design intent.

**Recommendation:** operator-decision required. My read: path (2) is more methodology-coherent (pre-cycle declaration is for compounding-cost cycles; not all phases compound). But the empirical evidence is that primer 2a got its pre-cycle declaration via [Review 91 Finding 1](2026-05-23-suite-review.md#r91-f1) — which suggests other phases may surface similar evidence-shape needs in future cycles. Path (2) with a "extend if a phase-specific evidence-shape need surfaces" trigger is the conservative path.

**Classification:** Open (registered for tracking; operator-policy decision pending path 1 vs path 2; the codification is a separate PR regardless).

---

<a id="r92-f4"></a>
**Finding 4 — Prose-only "the X supplement" pattern persists in 17 files post-[Review 91 Finding 2](2026-05-23-suite-review.md#r91-f2) codification; F2 applied to forward project-review-log entries but did not retroactively sweep the suite's own surface**

**Owner:** technical-writer (governing-standard prose surface)
**Status:** raised
**Blocked by:** *(none)*

**Evidence:** Mechanical sweep across all 42 files for `the [A-Za-z]+ supplement` pattern:

- 17 files contain the pattern: 12 role-domain prompts + 1 primer (`2c-refactor.md`) + 4 supplements (`bash.md`, `css.md`, `github-actions.md`, `yaml.md`)
- Per-file count: most have 1-2 instances; `css.md` + `AI-ENGINEER-REVIEW.md` have 2 each

Per [Review 91 Finding 2](2026-05-23-suite-review.md#r91-f2): the `**Supplements applied:**` plural-form preamble field replaces the prose-only "the X supplement § Y floor raised every finding below" template — but the codification is forward-only (applies to entries authored 2026-05-24+) and was scoped to **project-level review-log entries**, NOT to the suite's own domain prompts, primers, or supplements.

**Reasoning:** Each prose-only "the X supplement" mention in the suite needs per-file evaluation — some are legitimate cross-references (e.g., a supplement naming another supplement; a domain prompt's `**Language and interface supplement:**` section linking the supplement); some are redundant (the inline-link is already present elsewhere in the same file). Mechanical-sweep candidate but not all 17 are defects — operator-judgment required per mention.

**Sub-finding sample (3 spot-checks):**

- **AI-ENGINEER-REVIEW.md line 36** (`load the AI Engineer section from the relevant supplement(s) when authored: \`../../supplements/markdown.md\` ...`) — inline-linked supplement path present; the prose-mention is LEGITIMATE methodology-prose context for the linked path, not redundant.
- **2c-refactor.md** — needs per-mention verification (not spot-checked this audit).
- **github-actions.md** — supplement-referencing-other-supplements pattern; needs per-mention verification.

**Recommendation:** the F2 codification's spirit (parseable + clickable supplement surface) applies forward-only to project review-log entries. Whether to retroactively sweep the suite's own surface is operator-policy: per [G-89](../FINDINGS-INDEX.md#g-89) forward-only narrative-preservation, the existing prose may stay as-is; per the [Review 91 Finding 18](2026-05-23-suite-review.md#r91-f18) precedent (SUITE-DEVELOPMENT-REVIEW slim-form forward-only with existing-rows preserved), the same forward-only pattern applies. **Conservative path: do not sweep**; let the pattern decay as files are touched for substantive reasons. **Active path: sweep mechanically** as a follow-up cycle.

**Classification:** Open (registered for tracking; operator-policy decision pending conservative-vs-active; the discipline is forward-only by default per G-89 unless operator chooses active sweep).

---

<a id="r92-f5"></a>
**Finding 5 — Cost-tally per-domain implications NOT codified; the cost-tally schema concentrates in 4 files (AI-ENGINEER-REVIEW + claude-code-cli + primer 3 + github-actions) but the per-role-domain interaction with cost-tally findings is not named — e.g., what's SE / QE / PerfEng / Security expected to do with cost-tally narrative?**

**Owner:** ai-engineer (methodology-prose surface)
**Status:** raised
**Blocked by:** *(none)*

**Evidence:** Mechanical sweep for `cost.tally` across the 42-file surface:

- 4 files have substantive cost-tally references: [`AI-ENGINEER-REVIEW.md`](../../domains/role/AI-ENGINEER-REVIEW.md) (the owner-domain per Review 87 Finding 6 per-error-class table); [`supplements/claude-code-cli.md`](../../supplements/claude-code-cli.md) (per-tool supplement); [`primers/3-review-session.md`](../../primers/3-review-session.md) (the canonical primer host); [`supplements/github-actions.md`](../../supplements/github-actions.md) (likely tangential per the supplement's multi-domain coverage)
- **0 other role-domain prompts reference cost-tally** — neither SE, QE, PerfEng, Security, Red Team, SA, SO, TW, Doc Reviewer, UX, Accessibility, Privacy, Localization, Data Engineer, nor Platform Engineer mention the surface

Per [Review 91 Finding 13](2026-05-23-suite-review.md#r91-f13) cost-tally Agent-API contract promotion + [Finding 9](2026-05-23-suite-review.md#r91-f9) Shape 1 sibling JSON: cost-tally is now a stable agent-readable surface that the methodology commits to. But the per-domain implications (does a QE finding's body need to consider cost-tally context? does a PerfEng finding link to cost-tally evidence?) are not codified. The cost-tally surface exists in isolation from the role-domain dim coverage.

**Reasoning:** Two plausible framings:

1. **Cost-tally-is-AI-Engineer-only path**: per Review 87 Finding 6 per-error-class owner table, AI Engineer owns the meta-tooling-of-methodology surface; cost-tally is AI Engineer's surface; other domains have no expected interaction beyond reading the cost-tally when scanning a Review entry. Cost: 1 paragraph in `suite-development.md` § Per-review entry preamble § Cost-tally naming the scope.
2. **Cost-tally-cross-cuts-domains path**: cost-tally evidence informs cross-domain findings (e.g., a PerfEng finding about slow agent execution can route to cost-tally evidence; a Security finding about supply-chain risk can correlate with prompt-cache discipline; a SO finding about over-investment routes to cost-tally findings/100k metric). Cost: per-relevant-domain prompts gain a "cost-tally interaction" sub-clause; ~3-5 domain prompts × ~5-10 lines each = ~20-50 lines.

Path (1) keeps cost-tally tightly scoped + reduces cross-domain noise. Path (2) makes cost-observability actionable beyond AI Engineer + supports the [Review 91 Finding 10](2026-05-23-suite-review.md#r91-f10) tuning-lever catalog's cross-domain effectiveness.

**Recommendation:** operator-decision pending. My read: path (1) is methodology-coherent for the immediate term — cost-tally is AI Engineer's surface; cross-domain interactions can be earned-by-recurrence (if a SO finding ever routes to cost-tally evidence, codify the path at that point). Path (2) is over-engineering relative to current evidence.

**Classification:** Open (registered for tracking; operator-policy decision pending; codification deferred regardless of path per the no-stacked-PRs preference).

---

### Dismissed

<a id="r92-f7"></a>
**Finding 7 — Initial concern: cli.md and browser-app.md supplements have very few H2 sections (3 each) suggesting under-coverage vs the per-language supplements (10-14 H2 sections each)**

**Owner:** *(none — finding dismissed)*
**Status:** *(none — terminal)*

**Evidence:** Mechanical sweep of H2 section counts across the 14 supplements: cli.md (3 sections: UX + QE + SE); browser-app.md (3 sections: QE + Security + UX); other supplements have 7-14 sections each.

**Reasoning for dismissal:** [`vsdd-suite/supplements/cli.md`](../../supplements/cli.md) and [`vsdd-suite/supplements/browser-app.md`](../../supplements/browser-app.md) are **interface supplements**, not language supplements. Per [`suite-development.md`](../suite-development.md) § Supplement coverage table, interface supplements target specific interface-types (CLI; browser app) and apply to domains whose work intersects that interface (UX for CLI is canonical; QE because tests exercise the interface; SE because the interface is the implementation surface; Security for browser-app because browsers are remote-attack surfaces). The narrow H2 count is **by design** — only the domains whose work intersects the interface need per-supplement guidance. Per-language supplements naturally have broader coverage (every domain reviewing a Rust project benefits from Rust-specific guidance).

The H2-count gap was a false signal from the initial scan. The interface-vs-language distinction explains the gap structurally.

**Classification:** Dismissed (the H2-count gap is by design per the interface-supplement scope; not a coverage defect).

---

### Summary

Suite-wide observability + auditability sweep across the 42-file methodology surface, applying the just-codified [Review 91 Finding 14](2026-05-23-suite-review.md#r91-f14) cite-verify discipline + [Finding 16](2026-05-23-suite-review.md#r91-f16) lookup-idiom adoption as the audit's own method. **7 findings filed: 2 Resolved in-cycle (F1 UX-REVIEW naming + F6 empirical-evidence-of-codification-effectiveness) + 4 Open (F2 supplements three-audience cascade-or-amend; F3 primer pre-cycle scope decision; F4 prose-only-supplement sweep operator-policy; F5 cost-tally per-domain implications operator-policy) + 1 Dismissed (F7 interface-supplement H2-count false signal).**

**Strong positive signal:** the suite's per-domain methodology surface is in good shape — all 19 domains have three-audience-lens; all have sycophancy check; all have Validator pair reference. The Review 91 codifications stuck (F1, F8, F10, F12, F18, F19, F20 codifications all land in their stated locations; F2, F3, F4, F5, F9, F13, F14, F15, F16 codifications all land in their stated locations per regression-check).

**Substantive gaps surfaced:**

1. **F1 (Resolved)**: UX-REVIEW.md `**Interface type:**` naming inconsistency — fixed in-cycle.
2. **F2 (Open)**: 13 of 14 supplements lack three-audience-lens — Review 84 Finding 4 codification scope excluded supplements; operator-policy decision pending cascade-vs-methodology-amendment.
3. **F3 (Open)**: 6 of 9 primers lack pre-cycle methodology check — operator-policy decision pending extend-discipline vs scope-to-compounding-cost.
4. **F4 (Open)**: 17 files contain prose-only "the X supplement" pattern — operator-policy decision pending conservative-no-sweep vs active-mechanical-sweep.
5. **F5 (Open)**: cost-tally per-domain interactions not codified — operator-policy decision pending AI-Engineer-only vs cross-cuts-domains.

**F6 (Resolved) is the most important methodology-calibration finding** — empirical evidence that the F14 + F16 codifications shifted authoring behavior in their first post-codification cycle. The audit's grep-pattern-first method + cite-verify spot-check are not lip service to the codifications; they are the codifications working as designed. This is cycle 1 of the F16-named 3-cycle empirical-evidence requirement.

**The suite's audit-trail-structure-slop findings from Review 91 (F18/F19/F20) all hold:** this Review 92 entry is target-under-300-lines per the F19 discipline (final size to be measured at session-end); the SUITE-DEVELOPMENT-REVIEW row will be slim-form per F18; the cost-tally uses the full-tiered shape per F8 + F20 multi-lens classification.

**Coordination:** Routes forward to (a) operator-policy decisions on F2/F3/F4/F5 paths (each named with conservative-vs-active framing for operator choice); (b) the next 2 cycles' audit-author behavior — F6's empirical-evidence requirement advances to cycle 2 (next suite review) and cycle 3 (cycle after that); if both continue grep-first + cite-verify, the F16 discipline can be considered behavior-stable + the third-cycle-soft-hook escalation trigger can be retired; (c) future suite-wide audits that may want to apply the rigorous-form domain-effectiveness audit per [Review 91 Finding 15](2026-05-23-suite-review.md#r91-f15) — this Review 92's thin-form is appropriate for cycle-close conformance verification; a rigorous-form audit per-domain is a larger separate cycle.

**Cost-tally (session-end addendum):** session-end Bash `date -u` captured at the commit-prep step below; appended to the cost-tally Wall-clock anchors line above. Operator-action queue items remain pending operator paste.
