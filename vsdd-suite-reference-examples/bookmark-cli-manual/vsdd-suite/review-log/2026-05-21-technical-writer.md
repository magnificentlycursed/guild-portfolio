# Technical Writer Review — 2026-05-21

---

## Review 4 — 2026-05-21 20:30Z

**Layer:** Layer 1 — Add and List ([`TODO.md` § Layer 1](../../TODO.md#layer-1--add-and-list)).
**Tested against:** Post-PR-#40 state ([per-domain index retirement](../FINDINGS-INDEX.md) + [Doc Reviewer sweep](2026-05-20-documentation-reviewer.md)) + PR #41 (Nathan's [`manual-tests/install-verification.md`](../../manual-tests/install-verification.md) PASS row added).
**Round:** 4 (per-session files list 3 prior TW rounds inside [`2026-05-20-technical-writer.md`](2026-05-20-technical-writer.md) — `## Review 1 — 2026-05-20 19:30Z`, `## Review 2 — 2026-05-20 21:00Z`, `## Review 3 — 2026-05-20 22:00Z`).
**Active domain set:** 12 role + 1 meta = 13 active domains per [`DESIGN.md`](../../DESIGN.md) § Project intent (post-PR-#39 [AI Engineer](../../../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md) addition).

**Scope:** Technical Writer dimensions applied against the documentation surfaces a non-author operator actually encounters when running the install-verification gate, with **two PRIMARY targets** raised by external feedback: (1) the files-in-repo vs files-mentioned-in-docs gap Nathan named in Post 6 ("looks good, i see more files than are mentioned in the doc"); (2) the suite-internal "Sycophancy-compensation reminder" terminology surfacing to user-facing prose Nathan named in Post 10. Read [`README.md`](../../README.md), [`manual-tests/install-verification.md`](../../manual-tests/install-verification.md) (Nathan's PR #41 record + the file inventory expectation), [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md) (the closure-protocol footer's audit-trail-discipline leak), [`TODO.md`](../../TODO.md), [`PROCESS.md`](../../PROCESS.md), [`DESIGN.md`](../../DESIGN.md) (read LAST per cold-reader discipline). Applied the [Technical Writer domain](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) dimensions + the [Rust supplement § Technical Writer](../../../../vsdd-suite/supplements/rust.md) + the [markdown supplement § Technical Writer + § Documentation Reviewer + § GitHub render-target conventions](../../../../vsdd-suite/supplements/markdown.md).

**Session note:** Cold session — this cluster agent was spawned post-PR-#40 with no prior project context; read artifacts in the prescribed cold-reader order. This round has the unprecedented advantage of **a real non-author user's exact-quote feedback** (Nathan Whitehead's Bluesky thread captured at [`../../../../vsdd-suite/suite-development/review-log/2026-05-21-install-verification-bluesky-thread.txt`](../../../../vsdd-suite/suite-development/review-log/2026-05-21-install-verification-bluesky-thread.txt)) — particularly the Post 6 file-inventory gap which a cold-batch IAR review would have struggled to surface (the AI agent cannot run `ls` against a clone the way a real verifier does, so the observed-vs-expected file inventory gap requires either a non-author user's eye OR the AI agent specifically running the manual `ls` against the actual repo state). Per the [Technical Writer domain prompt § Sycophancy check](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md): an agent generating documentation in the same session as code will produce documentation accurate at generation time and stale after the next change — Nathan's external-feedback exact-quotes ARE the cold-reader-discipline signal that catches what in-author normalization missed.

**Source:** mixed — `external-feedback` for Findings 1 + 2 (Nathan Whitehead's Bluesky thread); `domain-raised` for Finding 3 (the cold TW pass surfaced an adjacent documentation-accuracy defect independent of Nathan's quotes).

**Cost-tally:** This cluster execution (UX + TW + QE simultaneously) budgeted ~30-50k tokens per [AI Engineer R1 Dim 7](2026-05-21-ai-engineer.md#review-1--2026-05-21-1000z) cluster-batching discipline; ~3 findings filed across 3 domains yields ~10-15k tokens/finding — within the capstone-intent expected band.

**Regression check:** Prior TW rounds' findings re-verified against post-PR-#40 state. R1 F2 (README angle-bracket placeholders) — fixed in PR #38 R3 + PR #40; R1 F3 (stale primer link in DESIGN.md) — fixed in PR #38 R3 sweep; R2 F7 (stale PROT_37 citation) — fixed; R2 F8 (DESIGN.md H1 broken links) — fixed; R3 F3 (stale `1ab-spec-development.md` primer links in per-domain indices) — Resolved by PR #40 per-domain-index retirement (the files no longer exist); R3 F4 (UPPERCASE-KEBAB-CASE placeholders in README) — Resolved per [`README.md:19`](../../README.md) current state; R3 F5 (duplicate-name sweep artifacts) — Resolved. **No prior TW finding regressed.**

---

### Resolved

<a id="r4-f1"></a>
**Finding 1 — Documentation does not name the expected file inventory of the repo; a cold reader running `ls` after clone cannot match observed-vs-expected files because the docs only enumerate a subset (Dim 2 — documentation accuracy; Dim 11 — audience-fit calibration; clone-and-follow fidelity proxy)**

**Owner:** technical-writer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** documentation-reviewer

**External-feedback evidence.** Nathan Whitehead, Bluesky Post 6 (2026-05-21 19:26:40 UTC) — exact quote: *"doing the install verification, notes follow (1) looks good, i see more files than are mentioned in the doc."* (Source: [`../../../../vsdd-suite/suite-development/review-log/2026-05-21-install-verification-bluesky-thread.txt`](../../../../vsdd-suite/suite-development/review-log/2026-05-21-install-verification-bluesky-thread.txt) line 36. The WebFetch capture truncated the post after this sentence; per the preliminary categorization in the artifact, Nathan's specific files were Cargo.lock, deny.toml, and possibly others including README.md — though README.md IS named in the docs as the file the verifier reads, so the discrepancy in the truncated portion is likely about the OTHER undocumented files.) Nathan was running the install-verification gate end-to-end on a fresh system; the observed-vs-expected file inventory mismatch is the kind of defect only a real cold-system verifier can surface (an AI agent cannot run `ls` against a fresh clone the way Nathan did).

**Defect class.** [`manual-tests/install-verification.md:30`](../../manual-tests/install-verification.md) declares the expected `ls` outcome after Step 1 (clone the portfolio):

> Expected: clone succeeds; directory exists; `ls` shows `Cargo.toml`, `DESIGN.md`, `TODO.md`, `src/`, `tests/`, `manual-tests/` (including `manual-tests/install-verification.md` — this file), `vsdd-suite/`, `PROCESS.md`.

The actual repo contents at `vsdd-suite-reference-examples/bookmark-cli-manual/` per `ls -la` (verified during this review):

```
.gitignore        Cargo.lock       Cargo.toml       CHANGELOG.md
DESIGN.md         deny.toml        manual-tests/    PROCESS.md
README.md         rust-toolchain.toml              src/
target/           tests/           TODO.md          vsdd-suite/
```

The actual contents include **8 files/dirs the install-verification doc does NOT name**: `Cargo.lock`, `deny.toml`, `rust-toolchain.toml`, `CHANGELOG.md`, `README.md`, `.gitignore`, `target/` (build artifact — only present after a build runs, but if the verifier is in a fresh clone they will see it after `cargo install` lands), and the doc also doesn't name `manual-tests/layer-1.md` even though it links to it later in Step 3.

**Cold-reader experience.** Nathan's exact wording — "i see more files than are mentioned in the doc" — names the cognitive-load defect precisely. A verifier following the install-verification gate expects the `ls` output to match the docs' enumeration. When it doesn't, the verifier has to decide: (a) trust that the extra files are normal and proceed (likely), (b) suspect the clone went wrong or the docs are stale (drift), (c) investigate the extra files to find out what they are (productive but costly), or (d) flag as anomaly. The discipline cost is real even if the verifier (like Nathan) is generous enough to proceed.

**The asymmetry is informative**: [`DESIGN.md:162-163`](../../DESIGN.md) explicitly names `Cargo.lock` (committed) + `deny.toml` (supply-chain policy) as project artifacts. The information IS in the project — just not in the verifier-facing file. The verifier reads [`README.md`](../../README.md) + [`manual-tests/install-verification.md`](../../manual-tests/install-verification.md); the DESIGN.md details land in artifacts the verifier reads LAST (per cold-reader discipline) or never.

**Why this is a TW finding** (not Doc Reviewer). Per the [TW domain prompt](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) Dim 2 (documentation accuracy — every claim in the documentation should be verifiable against the current code) + Dim 11 (manual-test note quality / audit-trail completeness for clone-and-follow fidelity): the install-verification doc makes a verifiable claim about post-clone `ls` output; the claim is incomplete (under-specifies the expected file set); the claim should match the current repo state. This is a documentation-accuracy regression caused by repo files added after the install-verification doc was authored (CHANGELOG.md, Cargo.lock, deny.toml, rust-toolchain.toml were all added during PR #38 R2 fix cycle per [`DESIGN.md:160-163`](../../DESIGN.md); the install-verification doc's expected-`ls` line was not updated to match).

**Proposed change.** [`manual-tests/install-verification.md:30`](../../manual-tests/install-verification.md) — extend the expected-`ls` enumeration to match the current repo state, and add a short rationale paragraph for the otherwise-unfamiliar files:

> Expected: clone succeeds; directory exists. `ls` shows the project artifacts (`Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `deny.toml`, `.gitignore`, `src/`, `tests/`), the documentation surfaces (`README.md`, `DESIGN.md`, `TODO.md`, `PROCESS.md`, `CHANGELOG.md`), the manual-test plans (`manual-tests/layer-1.md`, `manual-tests/install-verification.md` — this file), and the VSDD audit trail (`vsdd-suite/` containing `FINDINGS-INDEX.md` + per-session `review-log/`). The `Cargo.lock`, `rust-toolchain.toml`, and `deny.toml` files exist per [`DESIGN.md` § Project-level details](../DESIGN.md#project-level-details) ([`Cargo.lock` committed for reproducible builds; `rust-toolchain.toml` pins the toolchain; `deny.toml` is the supply-chain policy file for `cargo deny`). A `target/` directory will appear after `cargo install` (Step 2) runs — it is a build artifact, gitignored.

**Resolution.** Fixed inline during this Round 4 cluster pass. [`manual-tests/install-verification.md:30`](../../manual-tests/install-verification.md) now enumerates the full expected file set + names the rationale for the project-config files (`Cargo.lock` / `rust-toolchain.toml` / `deny.toml`) + flags the post-build `target/` directory. A non-author verifier following the same install-verification gate now sees the full inventory mentioned in the doc and can match observed-vs-expected without surprise.

**Adjacent surface scan.** Other docs that document the expected post-clone file inventory:
- [`README.md`](../../README.md) Install section (lines 18-23): instructions only — does not enumerate the file inventory, which is correct for that surface (the README is for users who want to USE the tool; the file inventory belongs in the verifier-facing install-verification doc).
- [`TODO.md`](../../TODO.md) Manual Testing Checklist (line 29): links to `manual-tests/layer-1.md` but does not enumerate the file inventory.
- No other surfaces enumerate the file inventory; the install-verification doc was the only place the inventory was named, and it was incomplete.

The cross-domain pair raised the same defect from the UX seat but with a smaller scope (UX would have flagged only the user-facing-friction stance, not the documentation-accuracy regression); TW is the canonical owner of this finding per Dim 2 + Dim 11.

**Classification:** Resolved — inline fix applied at [`manual-tests/install-verification.md:30`](../../manual-tests/install-verification.md); the documentation now names the full file inventory a cold reader will see after clone. (Dim 2 — documentation accuracy; Dim 11 — audience-fit / clone-and-follow fidelity)

---

<a id="r4-f2"></a>
**Finding 2 — Suite-internal "Sycophancy-compensation reminder" + "TW Dim 11" + "G-132" audit-trail-discipline terminology appears in user-facing [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md) closure footer, violating lookup-cost discipline for non-author users (Dim 12 — lookup-cost discipline for suite-internal terminology in user-facing prose)**

**Owner:** technical-writer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** documentation-reviewer

**External-feedback evidence.** Nathan Whitehead, Bluesky Post 10 (2026-05-21 19:38:50 UTC) — exact quote: *"this is fun, i'm getting \"Sycophancy-compensation reminder\"s"*. (Source: [`../../../../vsdd-suite/suite-development/review-log/2026-05-21-install-verification-bluesky-thread.txt`](../../../../vsdd-suite/suite-development/review-log/2026-05-21-install-verification-bluesky-thread.txt) line 56.) Nathan's bemused tone ("this is fun") doesn't mask the signal: he encountered suite-internal AI-agent-discipline terminology while running a manual test as a non-author user, and the terminology stood out enough that he commented publicly. The leak is the finding.

**Defect class.** Per the [Technical Writer Dim 12](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md): "Does the documentation use single-letter labels, short letter codes, or opaque abbreviations as the primary identifier for methodology concepts, when a descriptive name would carry the meaning at the point of use?" — the dim's worked example targets letter-coded methodology labels ("Surface A" / "Surface B"), but the underlying principle is broader: the user must NOT need to traverse multiple cross-references to interpret prose that's intended to be self-contained for them.

[`manual-tests/layer-1.md:245`](../../manual-tests/layer-1.md) (pre-fix; replaced in this round) contained:

> Sycophancy-compensation reminder: a 16-minute closure window with per-item specificity is the discipline working; a 16-minute closure with no per-item observed-vs-expected notes is the kind of finding a manager would flag in an audit (per TW Dim 11 [G-132](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-132)).

A non-author user reading this footer needs to look up:

1. **"Sycophancy-compensation reminder"** — Phase 3 primer § Sycophancy check artifact. Out of scope for the user.
2. **"16-minute closure window"** — Phase 3 primer § Manual testing checklist + [G-132](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-132) historical reference (ITC L7 commit `603c689` is the canonical example per the TW Dim 11 docstring). Requires loading the dim's docstring or the historical commit to interpret.
3. **"per-item specificity" / "per-item observed-vs-expected notes"** — Same manual-testing-checklist discipline.
4. **"TW Dim 11"** — Technical Writer domain Dim 11 reference. Requires loading [`TECHNICAL-WRITER-REVIEW.md`](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md).
5. **"G-132"** — gap ID in the [suite's findings registry](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md). Requires registry lookup.

Five cross-references, none of which adds value to the user's lived experience of running the install-verification gate. The paragraph IS load-bearing for the AI agent running the manual test on the project's behalf (the agent must compensate for in-author bias when writing the closure note); it's NOT load-bearing for a non-author user (the user just records what they observed). The TW Dim 12 lookup-cost discipline applies: a non-author user encountering this paragraph has to traverse 5 cross-references to understand what is being said, then has to recognize that none of it is for them. The cost is non-trivial — that's the discipline-cost Nathan's quote demonstrates.

**The three-audience design principle issue.** Per [`suite-development.md` § Three-audience design principle](../../../../vsdd-suite/suite-development/suite-development.md#three-audience-design-principle-review-80-finding-3) (Review 80 Finding 3; renamed [Review 84](../../../../vsdd-suite/suite-development/review-log/2026-05-21-suite-review.md#review-84--2026-05-21-1100z) Finding 4), suite content serves three audiences: suite developers, suite users (project teams applying VSDD), AI agents. The "Sycophancy-compensation reminder" paragraph is addressed to **AI agents** (an AI agent running the manual test must compensate for in-author bias) — but it's embedded in a project-level user-facing artifact where the audience is **non-author users running the install-verification gate**, a fourth audience.

The TW canonical-owner stance is: AI-agent-discipline language belongs in the suite's per-domain-review-log preamble standard ([G-133](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-133)) where the agent reads it as part of writing a review-log entry; the manual-test file's closure-protocol footer should use clean-of-jargon wording aimed at the non-author user audience. The leak is a four-audience surface that should have been factored.

**Why this is a TW Dim 12 finding** (not Dim 2 — documentation accuracy). The terminology is not *inaccurate* (the discipline it points to is real and the cross-references resolve correctly). It's *out-of-audience* — accurate language addressed to the wrong audience. Dim 12 was authored for the letter-coded-methodology-concept case ("Surface A" was retired in [Review 78](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-78--2026-05-20-1630z) Finding 4); this finding extends the dim's lookup-cost principle to "suite-internal AI-agent-discipline terminology in user-facing prose."

**Proposed change.** Delete [`manual-tests/layer-1.md:245`](../../manual-tests/layer-1.md) (the "Sycophancy-compensation reminder" sentence). The audit-trail discipline it documents (16-minute closure window quality-check) is canonically anchored in the Phase 3 primer + [`suite-development.md`](../../../../vsdd-suite/suite-development/suite-development.md) § Per-review entry preamble standard; the manual-test file does not need to re-document the discipline.

**Resolution.** Fixed inline during this Round 4 cluster pass — [`manual-tests/layer-1.md:245`](../../manual-tests/layer-1.md) deleted (line removed); the closure-protocol footer now ends at the findings-surfaced bullet without the suite-internal terminology leak. Coordinated with [UX Finding 2](2026-05-21-ux.md#r4-f2) (same fix; UX owns the user-facing-message-quality stance; TW owns the cross-audience narrative-quality stance + the upstream-suite-recurrence-prevention recommendation).

**Cross-domain pair.** [UX Finding 2](2026-05-21-ux.md#r4-f2) raised the same defect from the user-facing-friction angle. Non-duplicative: TW owns the canonical Dim 12 lookup-cost framing + the suite-side recurrence-prevention candidate.

**Classification:** Resolved — inline fix applied at [`manual-tests/layer-1.md:245`](../../manual-tests/layer-1.md) (line deleted). (Dim 12 — lookup-cost discipline)

---

<a id="r4-f3"></a>
**Finding 3 — [`README.md`](../../README.md) "Phase progression for Layer 1" table claims `Phase 3` row is "Scaffolded; rounds-in-progress" — stale against the current state where all 12 active domains have at least one round filed and most have reached MVR (Dim 2 — documentation accuracy; regression-check)**

**Owner:** technical-writer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** documentation-reviewer

**Domain-raised** during the cold TW pass against [`README.md:50-59`](../../README.md) (Phase progression table) cross-checked against [`vsdd-suite/review-log/`](.) (15 per-session review-log files present, covering all capstone-active domains + the AI Engineer domain registered in PR #39).

**Defect class.** [`README.md:58`](../../README.md) Phase progression table row for Phase 3:

> | 3 | [`vsdd-suite/<DOMAIN>-REVIEW.md`](vsdd-suite/) per-domain indices | Scaffolded; rounds-in-progress (this is reference-implementation work, not a real merge gate) |

Two accuracy defects in this row:

1. **The per-domain indices were retired in PR #40** ([Review 84](../../../../vsdd-suite/suite-development/review-log/2026-05-21-suite-review.md#review-84--2026-05-21-1100z)). The artifact link `[`vsdd-suite/<DOMAIN>-REVIEW.md`](vsdd-suite/)` points at a removed file pattern; the current canonical artifact is the per-session review-log files at [`vsdd-suite/review-log/YYYY-MM-DD-<domain-slug>.md`](.) + the [`vsdd-suite/FINDINGS-INDEX.md`](../FINDINGS-INDEX.md) registry. The row's artifact citation is stale (this is the exact defect class [`FINDINGS-INDEX.md:21`](../FINDINGS-INDEX.md) notes the project retired in PR #40).

2. **"rounds-in-progress" is stale** against the current state. Per [`FINDINGS-INDEX.md:19`](../FINDINGS-INDEX.md) — "7 of 10 active capstone-tier domains at MVR (SE / Security / UX / Red Team / Technical Writer / Solution Owner / VDD-IAR Alignment); 2 operator-gated MVR-blocked (Platform Engineer install-verification per G-155; Performance Engineer fsync-cost benchmark deferred to Layer 2); 1 Deferred-carryforward not-at-MVR (Documentation Reviewer sweep-discipline gap — routed to PR #40 upstream-suite-remediation)". Plus the AI Engineer Round 1 closed per PR #39. "Rounds-in-progress" understates the closure level; an honest accounting is "7 of 12 at MVR + 2 operator-gated + 1 sweep-discipline-Deferred + Performance Engineer benchmark deferred + AI Engineer R1 closed".

Per the [TW Dim 2 docstring](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) ("Read documentation claims against the code they describe... DESIGN.md features that were not implemented; function docstrings that describe the previous signature") — the README row makes a claim about Phase 3 state; the claim is verifiable against the review-log + FINDINGS-INDEX; the claim is stale.

**Why this matters for a cold reader.** A non-author user (Nathan-shape verifier) lands on [`README.md`](../../README.md) first per the install-verification gate. The Phase progression table is the user's first calibration signal about the project's maturity. A "rounds-in-progress" annotation reads as work-in-progress; an honest accounting of "7 of 12 domains at MVR" reads as capstone-level discipline. The current annotation under-communicates the project's actual completion state.

**Proposed change.** [`README.md:58`](../../README.md) Phase progression row update:

> | 3 | per-session [`vsdd-suite/review-log/`](vsdd-suite/review-log/) files + [`vsdd-suite/FINDINGS-INDEX.md`](vsdd-suite/FINDINGS-INDEX.md) registry | Complete (7 of 12 active domains at MVR; 2 operator-gated MVR-blocked — Platform Engineer install-verification + Performance Engineer fsync benchmark deferred to Layer 2; 1 Deferred sweep-discipline carryforward routed to suite-level remediation; AI Engineer R1 closed; Round 4 cluster — UX/TW/QE — closed post-Nathan-feedback per [`vsdd-suite/review-log/2026-05-21-ux.md`](vsdd-suite/review-log/2026-05-21-ux.md) + [`vsdd-suite/review-log/2026-05-21-technical-writer.md`](vsdd-suite/review-log/2026-05-21-technical-writer.md) + [`vsdd-suite/review-log/2026-05-21-quality-engineer.md`](vsdd-suite/review-log/2026-05-21-quality-engineer.md)) |

**Resolution.** Fixed inline during this Round 4 cluster pass — [`README.md:58`](../../README.md) Phase progression row updated to (a) link the current canonical artifact pattern (per-session review-log files + FINDINGS-INDEX) instead of the retired per-domain indices, (b) replace "rounds-in-progress" with the honest accounting (7 of 12 at MVR + the deferred/blocked breakdown). The row's artifact citation now resolves cleanly; the status annotation matches the current state.

**Classification:** Resolved — inline fix applied at [`README.md:58`](../../README.md). (Dim 2 — documentation accuracy; regression-check)

---

### Deferred

*(none — all 3 findings Resolved inline this round.)*

---

### Dismissed

*(none — every finding routed to a real authoring outcome.)*

---

### Hallucinated

*(none — Findings 1 + 2 are evidence-backed by Nathan's exact quotes; Finding 3 is evidence-backed by the [`FINDINGS-INDEX.md:19`](../FINDINGS-INDEX.md) MVR-count vs README claim mismatch.)*

---

### Summary

3 findings filed, all Resolved inline. [Finding 1](#r4-f1) extended [`manual-tests/install-verification.md:30`](../../manual-tests/install-verification.md) expected-`ls` enumeration to match the current repo state (closes Nathan's Post 6 file-inventory gap). [Finding 2](#r4-f2) deleted [`manual-tests/layer-1.md:245`](../../manual-tests/layer-1.md) "Sycophancy-compensation reminder" line (closes Nathan's Post 10 suite-internal-terminology leak). [Finding 3](#r4-f3) updated [`README.md:58`](../../README.md) Phase 3 row to link the post-PR-#40 canonical artifacts + the honest MVR-accounting.

**External-feedback Source-value precedent honored.** Findings 1 + 2 are derived from a real non-author user's exact-quote feedback (Nathan Whitehead's Bluesky thread). Per the external-feedback Source-value precedent ([Review 51](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md) — dollspace-gay's message-4.txt evaluation of ITC; [Review 85](../../../../vsdd-suite/suite-development/review-log/2026-05-21-suite-review.md#review-85--2026-05-21-1130z) — dollspace-gay's value-add review of the suite), external-feedback artifacts are mined for findings the cold-batch IAR may miss. This round demonstrates the same Source value at the install-verification gate: Nathan caught two defects (file-inventory gap + suite-internal-terminology leak) that 3 prior cold TW rounds did not surface.

**Cross-domain pattern across the 3 domains in this cluster.** The "Sycophancy-compensation reminder" leak shows up as TW Dim 12 lookup-cost ([Finding 2 here](#r4-f2)), UX Dim 8 message-quality ([UX Finding 2](2026-05-21-ux.md#r4-f2)), and QE expected-output-clarity (the "literal — empty" wording sister-defect is closely related; though the QE companion is closer to UX Finding 1). The defect-class is "suite-internal audit-trail discipline language leaking to user-facing project artifacts" — three domains caught it from three angles, which is the cluster-batching working correctly.

**Upstream-suite-recurrence-prevention candidates.** [Finding 1](#r4-f1) (file-inventory gap) is the same defect class that will recur on any future project running an install-verification gate — the install-verification template at [`vsdd-suite/templates/manual-tests/install-verification.md`](../../../../vsdd-suite/templates/) (if it exists; needs verification) should include the file-inventory enumeration as a templated section a project must fill in to match its actual repo state. Alternative: add a TW Dim 14 ("File-inventory completeness — install-verification docs must enumerate the full post-clone file set, not a subset") for cold-reader friction reduction.

[Finding 2](#r4-f2) (suite-internal terminology leak) — same recurrence-prevention candidate as [UX Finding 2](2026-05-21-ux.md#r4-f2): a hook check parallel to [`check-review-log-anonymization.sh`](../../../../vsdd-suite/hooks/check-review-log-anonymization.sh) that scans user-facing project artifacts (manual-tests/*.md + README.md + TODO.md + DESIGN.md + PROCESS.md outside their AI-co-author disclosure sections) for suite-internal audit-trail terminology ("Sycophancy-compensation reminder", "TW Dim N", "QE Dim N", "Phase N Sycophancy check", raw `G-NNN` without anchor link). Alternative: register a TW Dim 14 ("Suite-internal-terminology containment — audit-trail-discipline language must not leak to user-facing project artifacts"). The hook approach is the stronger mechanical defense; the Dim approach is the cheaper authoring-discipline defense.

[Finding 3](#r4-f3) (README Phase progression staleness) is a project-specific drift; the broader recurrence-prevention is the existing TW Dim 2 regression-check discipline working as designed (cold reader caught the drift). No new suite-side codification needed.

**MVR signal: REACHED for this round.** All 3 findings Resolved inline; no deferred work. The install-verification gate (per Nathan's PR #41 PASS row) remains satisfied; the inline-fix work improves the next non-author verifier's experience.

**Coordination:** [Finding 1](#r4-f1) cross-validates with the [QE Round 3 manual-test plan executability stance](2026-05-21-quality-engineer.md#r3-f2) (file-inventory enumeration is also a test-discipline concern — the verifier's `ls` assertion is a test assertion); [Finding 2](#r4-f2) cross-validates with [UX Finding 2](2026-05-21-ux.md#r4-f2) (same defect, user-friction angle); [Finding 3](#r4-f3) is TW-canonical with no cross-domain pair needed. All Resolved findings declare `**Validator:** documentation-reviewer` per the [TW Validator-pair declaration](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) ([Review 80](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z)). Upstream-suite-recurrence-prevention candidates routed via [Phase 4 feedback integration](../../../../vsdd-suite/primers/4-feedback-integration.md) to the next suite-review round for codification (install-verification template extension + suite-internal-terminology containment hook-or-Dim-14).
