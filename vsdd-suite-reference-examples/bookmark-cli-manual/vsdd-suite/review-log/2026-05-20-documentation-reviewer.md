# Documentation Reviewer Review — 2026-05-20

[Index](../DOCUMENTATION-REVIEWER-REVIEW.md)

---

## Review 1 — 2026-05-20 19:30Z

**Scope:** Phase 3 [Adversarial Refinement](../../../vsdd-suite/primers/3-review-session.md) Round 1 cold-context cold-read of every user-facing artifact `bookmark-cli-manual` ships: [`README.md`](../../README.md), [`TODO.md`](../../TODO.md), [`CHANGELOG.md`](../../CHANGELOG.md), [`DESIGN.md`](../../DESIGN.md), [`PROCESS.md`](../../PROCESS.md), [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md), [`manual-tests/install-verification.md`](../../manual-tests/install-verification.md), the project's per-domain index [`vsdd-suite/DOCUMENTATION-REVIEWER-REVIEW.md`](../DOCUMENTATION-REVIEWER-REVIEW.md), [`vsdd-suite/FINDINGS-INDEX.md`](../FINDINGS-INDEX.md), and the per-domain index files customized in PR 6 ([`vsdd-suite/SOLUTION-ARCHITECT-REVIEW.md`](../SOLUTION-ARCHITECT-REVIEW.md), [`vsdd-suite/QUALITY-ENGINEER-REVIEW.md`](../QUALITY-ENGINEER-REVIEW.md)). Read in cold-reader order: README first, then what README pointed at, with DESIGN.md read last per the [Documentation Reviewer](../../../vsdd-suite/domains/role/DOCUMENTATION-REVIEWER-REVIEW.md) `## Current Review Prompt` discipline. Source code (`src/lib.rs`, `tests/bookmarks.rs`, `Cargo.toml`) consulted only to verify documentation claims after the cold pass.

**Lens:** Adversarial cold-reader pair to [Technical Writer](../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md). I did not build this project. I held every term as undefined until the project's own docs defined it. The exact tests applied per dimension: clone-and-follow fidelity (Dim 1), implicit-knowledge audit (Dim 2 — reader-built glossary; jargon expansion check), forward-reference safety (Dim 3), cross-reference resolution (Dim 4 — every `[text](path)` opened-and-confirmed), audience-fit calibration (Dim 5 — capstone reader profile), documentation rot (Dim 6 — every claim verified against current code/spec/process), recovery-from-confusion (Dim 7), manual-test plan executability (Dim 8 — read each command as if pasting into a fresh terminal), onboarding sequencing (Dim 9 — README first 10 lines), manual-test file structure consistency (Dim 10 — Review 74 convention), inline-reference clickthrough validation (Dim 11 — [Review 80](../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z) Finding 1; every markdown link opened to verify it resolves).

**Session note:** Cold-context AI session — no prior knowledge of `bookmark-cli-manual` beyond what its own docs supplied. The primer's sycophancy guard applied: mental-model interpolation from the suite primer / whitepaper / training data was explicitly held back; terms were taken as undefined until the project's own docs defined them. Where a finding required code consultation (e.g., the README's `expect: 8 tests pass` claim), the cold pass produced the candidate finding first; code was opened only to verify the divergence. The Read tool occasionally displayed `PROT_*` placeholder tokens in lines containing protocol-named CLI labels (e.g., the `bm add` contract heading and a few manual-test step titles); `grep` confirmed those tokens are tool-display artifacts, not file contents — they are not findings.

**Source:** `domain-raised` — every finding below was elicited by applying the 11 Documentation Reviewer dimensions to the project's user-facing docs in cold context. No director-raised observations interrupted the round.

**Assumption surfacing:** The Documentation Reviewer domain was registered in suite [Review 80](../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z) and activated on this project per the PR 6/Review 78 capstone-intent promotion; this Review 1 is the first Documentation Reviewer round filed against `bookmark-cli-manual`. No prior Documentation Reviewer round exists, so the standard regression check is vacuous for this round.

---

### Resolved

*(none — this round produces filed Findings; resolution belongs to the [Technical Writer](../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) Owner per the Doc Reviewer ↔ TW adversarial pair.)*

---

### Deferred

**Finding 1 — `README.md` install instructions name a directory that does not exist (Dim 1, Dim 4, Dim 6)**

<a id="r1-f1"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** technical-writer

[`README.md`](../../README.md):19-23 directs the reader to `cd <portfolio>/bookmark-cli` (and again at line 41 `cd <portfolio>/bookmark-cli`), but the project lives at `vsdd-suite-reference-examples/bookmark-cli-manual/`. The directory name `bookmark-cli` does not exist anywhere in the portfolio (`find . -maxdepth 3 -type d -name "bookmark-cli*"` returns only `bookmark-cli-manual`). A cold reader following the README literally will get `cd: no such file or directory: bookmark-cli` and stop.

The package name in [`Cargo.toml`](../../Cargo.toml):2 is `bookmark-cli` and the binary in [`Cargo.toml`](../../Cargo.toml):10 is `bm` — those are correct. The defect is the directory name in the install path. Compounded by [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md):13 which uses the correct path `cd vsdd-suite-reference-examples/bookmark-cli-manual` — the two docs contradict each other on where the project lives, so a reader who reads README first and then layer-1.md sees inconsistent instructions and has to guess.

Clone-and-follow fidelity (Dim 1) breaks at install step. Cross-reference resolution (Dim 4) — the path is not a markdown link but it is a load-bearing path the reader is told to `cd` into. Documentation rot (Dim 6) — the package's actual location moved at some point during the reference-example reorganization but README's install path was not updated.

Proposed fix: replace both `<portfolio>/bookmark-cli` references with `<portfolio>/vsdd-suite-reference-examples/bookmark-cli-manual` (or rewrite the Install / Test sections to be relative-cd-agnostic, instructing the reader to `cd` into the cloned project's `bookmark-cli-manual` directory wherever they cloned it).

**Classification:** Deferred — finding raised in Round 1; Round 2 verification required per the Phase 3 primer's continue-trigger discipline.

---

**Finding 2 — `README.md` claims "8 tests pass (Dim 6)**

<a id="r1-f2"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** technical-writer

[`README.md`](../../README.md):42-43 says:

```
cargo test
# expect: 8 tests pass (4 lib unit tests + 4 integration tests)
```

Verification against current code: [`src/lib.rs`](../../src/lib.rs) `#[cfg(test)] mod tests` contains 5 unit tests (`newest_first_sorts_descending_by_timestamp`, `load_returns_empty_for_missing_file`, `load_returns_empty_for_empty_file`, `save_then_load_roundtrips`, `save_creates_parent_directory_for_nested_path`), not 4. [`tests/bookmarks.rs`](../../tests/bookmarks.rs) has 4 integration tests as claimed. Actual total: 5 + 4 = 9.

The drift is traceable: per [`CHANGELOG.md`](../../CHANGELOG.md):80, the v0.7.2 Phase 5 adoption entry added `src/lib.rs::tests::save_creates_parent_directory_for_nested_path` (the 5th lib unit test) as a Mutation Testing falsifying test. The README's count was authored under the pre-Phase-5 4-test state and was not updated when Phase 5 added the 5th test.

A cold reader running `cargo test` will see `9 tests pass` in the actual output and either (a) doubt their install is current (if the README is the source of truth) or (b) lose trust in the README (if they trust the test runner). Either failure mode is a documentation-rot finding.

Proposed fix: update the line to `expect: 9 tests pass (5 lib unit tests + 4 integration tests)`. Better: omit the literal count and replace with `expect: all tests pass with no failures` — the count-claim is the unstable surface; the pass/fail invariant is what the reader actually needs.

**Classification:** Deferred — finding raised in Round 1; Round 2 verification required per the Phase 3 primer's continue-trigger discipline.

---

**Finding 3 — `README.md` and `DESIGN.md` link to the [VSDD](https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00) suite's `README.md` with the wrong relative depth (Dim 4, Dim 11)**

<a id="r1-f3"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** technical-writer

[`README.md`](../../README.md):7 and :47 contain `[VSDD Suite](../vsdd-suite/README.md)`. From `vsdd-suite-reference-examples/bookmark-cli-manual/README.md`, the relative path `../vsdd-suite/README.md` resolves to `vsdd-suite-reference-examples/vsdd-suite/README.md` — which does not exist. The suite's README is at `vsdd-suite/README.md` at the portfolio root, so the correct relative path needs two `..` segments: `../../vsdd-suite/README.md`. Verified by `ls`: `vsdd-suite-reference-examples/vsdd-suite/` does not exist; `vsdd-suite/README.md` exists at the portfolio root.

The same broken-depth pattern appears in [`DESIGN.md`](../../DESIGN.md):3, which uses both `[../vsdd-suite/primers/1ab-spec-crystallization.md](../vsdd-suite/primers/1ab-spec-crystallization.md)` and `[../vsdd-suite/README.md](../vsdd-suite/README.md)` — both single-`..`, both broken. (DESIGN.md:11 and downstream lines correctly use `../../vsdd-suite/...`, so the file is internally inconsistent on the relative-depth convention.)

Inline-reference clickthrough validation (Dim 11) — these are forward-facing markdown links from the cold reader's seat that 404 when clicked. Cross-reference resolution (Dim 4) — the cited paths don't resolve.

Proposed fix: replace every `../vsdd-suite/...` in `README.md` and `DESIGN.md` line 3 with `../../vsdd-suite/...`. A lychee link-checker pass against the project markdown would catch this mechanically; recommend wiring one into the suite's pre-commit per the [`markdown.md` § Quality Engineering](../../../vsdd-suite/supplements/markdown.md#quality-engineering) "Link checking with lychee" guidance.

**Classification:** Deferred — finding raised in Round 1; Round 2 verification required per the Phase 3 primer's continue-trigger discipline.

---

**Finding 4 — `PROCESS.md`, `DESIGN.md`, `SOLUTION-ARCHITECT-REVIEW.md`, and `QUALITY-ENGINEER-REVIEW.md` link to a non-existent primer `1ab-spec-development.md` (Dim 4, Dim 6, Dim 11)**

<a id="r1-f4"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** technical-writer

Four user-facing artifacts link to `vsdd-suite/primers/1ab-spec-development.md`, which does not exist. The actual primer is at `vsdd-suite/primers/1ab-spec-crystallization.md` (verified by `ls vsdd-suite/primers/`). Affected sites:

- [`PROCESS.md`](../../PROCESS.md):21 — `[Phase 1a+1b](../../vsdd-suite/primers/1ab-spec-development.md)`
- [`DESIGN.md`](../../DESIGN.md):3 — `[Phase 1a+1b](../../vsdd-suite/primers/1ab-spec-development.md)` (this same line ALSO has the wrong-depth links flagged in Finding 3 above; both defects coexist)
- [`vsdd-suite/SOLUTION-ARCHITECT-REVIEW.md`](../SOLUTION-ARCHITECT-REVIEW.md):21 — `[Phase 1a+1b](../../../vsdd-suite/primers/1ab-spec-development.md)`
- [`vsdd-suite/QUALITY-ENGINEER-REVIEW.md`](../QUALITY-ENGINEER-REVIEW.md):21 — `[Phase 1a+1b](../../../vsdd-suite/primers/1ab-spec-development.md)`

[`CHANGELOG.md`](../../CHANGELOG.md):85 already documents the rename: "DESIGN.md H1 / preamble — updated from 'Phase 1a contract' to 'Phase 1a+1b contract' per the G-96 / G-160 v0.6.0 rename; cites the renamed primer at `primers/1ab-spec-crystallization.md`." The change-note records the rename; the actual links in `DESIGN.md` line 3 (the H1 preamble it describes) still cite the pre-rename filename. The CHANGELOG also notes [G-96](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-96) renamed the primer; per [G-89](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-89) forward-only narrative-preservation policy, historical prose may stay — but a broken markdown LINK is not narrative; it is a load-bearing reference that drops the reader off a cliff.

Doc-rot (Dim 6) — the primer was renamed and the project's links did not follow. Inline-reference clickthrough validation (Dim 11) — every link 404s. Cross-reference resolution (Dim 4) — none of the four cited paths resolve.

Proposed fix: search-and-replace `1ab-spec-development.md` → `1ab-spec-crystallization.md` in all four files. (The forward-only preservation policy applies to prose narrative, not to broken markdown link targets — the link text "Phase 1a+1b" can stay; the path inside the parentheses needs updating.)

**Classification:** Deferred — finding raised in Round 1; Round 2 verification required per the Phase 3 primer's continue-trigger discipline.

---

**Finding 5 — `vsdd-suite/SOLUTION-ARCHITECT-REVIEW.md` companion-round link cites "QE Review 1" but the round is Review 2, and the cited anchor `#review-1--2026-05-20-0245z` does not exist (Dim 4, Dim 11)**

<a id="r1-f5"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** technical-writer

[`vsdd-suite/SOLUTION-ARCHITECT-REVIEW.md`](../SOLUTION-ARCHITECT-REVIEW.md):21 reads:

> Companion QE round (Mutation Testing Mutation Testing) at [QE Review 1](2026-05-20-quality-engineer.md#review-1--2026-05-20-0245z).

Two defects in one citation:

1. **Link text is wrong.** The QE companion round for the SA Phase 5 Purity Boundary Audit (filed under SA Review 1) is QE **Review 2** — confirmed by reading [`2026-05-20-quality-engineer.md`](2026-05-20-quality-engineer.md):9 which begins `## Review 2 — 2026-05-20 02:45Z`, and reading the Coordination line of QE Review 2 (`[`2026-05-20-quality-engineer.md`](2026-05-20-quality-engineer.md):72) which back-references "[SA Review 1 — 2026-05-20 02:45Z](2026-05-20-solution-architect.md#review-1--2026-05-20-0245z)". QE Review 1 lives in a DIFFERENT file ([`2026-05-17-quality-engineer.md`](2026-05-17-quality-engineer.md)) and is the pre-Phase-5 portfolio-era round, not the Phase 5 companion.

2. **Anchor target does not resolve.** The link points at `#review-1--2026-05-20-0245z` inside `2026-05-20-quality-engineer.md`, but that file has no `## Review 1` heading — only `## Review 2 — 2026-05-20 02:45Z` (anchor `#review-2--2026-05-20-0245z`). Clicking the link lands on the file but not on any heading; the reader is dumped at the top of the file with no signal where the cited content is.

Inline-reference clickthrough validation (Dim 11) — the anchor does not resolve. Cross-reference resolution (Dim 4) — the path resolves to the file but the in-file anchor is invalid. The defect is also a small documentation-rot (Dim 6) signal — the cross-file round-numbering wasn't reconciled when QE's per-date file structure was finalized.

Proposed fix: change link text from `QE Review 1` to `QE Review 2`; change anchor from `#review-1--2026-05-20-0245z` to `#review-2--2026-05-20-0245z`. While editing the line, also de-duplicate the "Mutation Testing Mutation Testing" stuttering captured in Finding 7 below.

**Classification:** Deferred — finding raised in Round 1; Round 2 verification required per the Phase 3 primer's continue-trigger discipline.

---

**Finding 6 — `TODO.md` and several `DESIGN.md` / per-domain prose lines still use retired letter-coded "Surface A.0 / B / C / D" identifiers (Dim 2, Dim 6, Dim 12)**

<a id="r1-f6"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** technical-writer

The [suite-development.md § Naming and identifier discipline (Review 78 Finding 4)](../../../vsdd-suite/suite-development/suite-development.md#naming-and-identifier-discipline-review-78-finding-4) discipline retired letter-coded Surface labels in favor of descriptive names ("Mutation Testing", "Purity Boundary Audit", "property-based testing", "Fuzz Testing", "Proof Execution") for forward-facing prose. Reference examples migrate per [G-177](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-177) precedent. PR 6 / Review 78 explicitly promoted `bookmark-cli-manual` to capstone intent post-Review-78; the prose authored in that PR must follow the discipline.

Currently-active prose still using letter codes as primary identifiers (NOT historical CHANGELOG narrative — those are preserved per [G-89](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-89)):

- [`TODO.md`](../../TODO.md):5 — "5 Surfaces A.0+B hardening" in the file's preamble. This is the file's current-state declaration of what the capstone IAR covers; not historical narrative.
- [`TODO.md`](../../TODO.md):41 — "Phase 5 Surfaces A.0 (purity boundary) + B (Mutation Testing) both at closure" in layer-gate criterion 5. Active gate criterion, not historical.
- [`DESIGN.md`](../../DESIGN.md):134 — "Phase 5 hardening ... per-layer Phase 5 rounds file in `vsdd-suite/SOLUTION-ARCHITECT-REVIEW.md` (Purity Boundary Audit / A / D) and `vsdd-suite/QUALITY-ENGINEER-REVIEW.md` (Mutation Testing / C)". Active spec, references letter codes `A`, `D`, `C`.
- [`DESIGN.md`](../../DESIGN.md):17 — Phase 6 strategy declaration references "property-based testing/C/D declared not-applicable" — mixes a descriptive name with letter codes inside the same parenthetical.

Implicit-knowledge audit (Dim 2) — a cold reader of the project's docs who has not read the suite's primer 5 has no idea what "Surface A.0", "B", "C", or "D" means. The descriptive name carries the meaning at point-of-use; the letter does not. (Cold-reader pair to [TW Dim 12](../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) "Lettering / abbreviation lookup cost".) Doc-rot (Dim 6) — the suite discipline shifted in Review 78; the project's PR-6-era forward-facing prose did not fully migrate.

Proposed fix: rewrite the four cited lines using descriptive names — e.g., TODO.md:5 → "5 Purity Boundary Audit + Mutation Testing hardening"; TODO.md:41 → "Phase 5 Purity Boundary Audit (purity boundary) + Mutation Testing both at closure"; DESIGN.md:134 → "per-layer Phase 5 rounds file in SOLUTION-ARCHITECT-REVIEW.md (Purity Boundary Audit + property-based testing + Proof Execution) and QUALITY-ENGINEER-REVIEW.md (Mutation Testing + Fuzz Testing)"; DESIGN.md:17 → spell out which surfaces are declared not-applicable ("property-based testing, Fuzz Testing, and Proof Execution declared not-applicable"). The CHANGELOG entries below the latest are preserved as historical narrative per [G-89](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-89).

**Classification:** Deferred — finding raised in Round 1; Round 2 verification required per the Phase 3 primer's continue-trigger discipline.

---

**Finding 7 — Search/replace artifacts from the descriptive-naming migration left stuttered phrases (Dim 6)**

<a id="r1-f7"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** technical-writer

Reviewing the descriptive-naming migration, several lines show search/replace stuttering — the original prose said something like "Surface B (Mutation Testing)" and a mechanical replace of `Surface B` → `Mutation Testing` produced "Mutation Testing (Mutation Testing)" rather than collapsing the parenthetical. Affected sites:

- [`DESIGN.md`](../../DESIGN.md):15 — Phase 5 strategy line contains "Mutation Testing (Mutation Testing via cargo-mutants)" and "property-based testing (property-based testing via proptest)". Each parenthetical is a stutter.
- [`DESIGN.md`](../../DESIGN.md):17 — Phase 6 strategy line contains "Purity Boundary Audit Purity Boundary Audit + Mutation Testing Mutation Testing closure" — two stutters in one phrase.
- [`vsdd-suite/SOLUTION-ARCHITECT-REVIEW.md`](../SOLUTION-ARCHITECT-REVIEW.md):21 — Reviews-table summary contains "Phase 5 Purity Boundary Audit Purity Boundary Audit" and "(Mutation Testing Mutation Testing)".
- [`vsdd-suite/QUALITY-ENGINEER-REVIEW.md`](../QUALITY-ENGINEER-REVIEW.md):21 — Reviews-table summary contains "`**Phase 5 hardening:** Mutation Testing — Mutation Testing for Layer 1 via cargo-mutants`" (acceptable inside a backtick-quoted preamble tag, but the surrounding prose is fine; this one is borderline — flagging in case the preamble tag itself is also a stutter rather than a deliberate `name — explanation` shape).

Documentation rot (Dim 6) — the stutters are mechanical artifacts of an incomplete editorial pass. A cold reader is not confused per se (they can parse the duplication) but the surface reads as low-quality, which on a capstone-intent reference example degrades the audit-trail signal.

Proposed fix: collapse each stutter to a single descriptive name. `Mutation Testing (Mutation Testing via cargo-mutants)` → `Mutation Testing (via cargo-mutants)`. `property-based testing (property-based testing via proptest)` → `property-based testing (via proptest)`. `Purity Boundary Audit Purity Boundary Audit` → `Purity Boundary Audit`. `Mutation Testing Mutation Testing closure` → `Mutation Testing closure`. The QE preamble-tag form `**Phase 5 hardening:** Mutation Testing — Mutation Testing for Layer 1 via cargo-mutants` should be reviewed against the [G-177](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-177) preamble-tag canonical shape — if the canonical shape is `**Phase 5 hardening:** <name> — <scope>`, then the second "Mutation Testing" should be either the layer scope ("for Layer 1") or omitted.

**Classification:** Deferred — finding raised in Round 1; Round 2 verification required per the Phase 3 primer's continue-trigger discipline.

---

**Finding 8 — `TODO.md` capstone-active-domain count and layer-gate criterion are stale: still says "10 active domains" / lists only 4 capstone extended, missing Documentation Reviewer (Dim 6)**

<a id="r1-f8"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** technical-writer

[`TODO.md`](../../TODO.md):5 says: "bookmark-cli-manual is at `capstone` intent with all 6 VSDD phases demonstrated end-to-end (1a+1b spec → 1c decomposition → 2a Red Gate → 2b implementation → 2c refactor (no-refactor annotation) → 3 IAR (**10 active domains**) → 4 routing → 5 Surfaces A.0+B hardening → 6 four-dimensional convergence)."

[`TODO.md`](../../TODO.md):40 layer-gate criterion 4 names: "7 cores (SE, QE, UX, Security, SA, SO, VDD-IAR Alignment) + capstone-tier extended (Performance Engineer, Red Team, Platform Engineer, Technical Writer)" — that is 7 + 4 = 11, AND Documentation Reviewer is missing from the list.

Compare to the current authoritative declaration in [`DESIGN.md`](../../DESIGN.md):11: "**11 role + 1 meta = 12 active domains**" with Documentation Reviewer explicitly included in the capstone extended set. The top entry of [`CHANGELOG.md`](../../CHANGELOG.md):3-13 documents that Review 80 / PR `#36` activated Documentation Reviewer on this reference example specifically and expanded the count from 10 role + 1 meta = 11 to 11 role + 1 meta = 12.

Documentation rot (Dim 6) — DESIGN.md and CHANGELOG record the activation; TODO.md was not updated alongside the activation. A cold reader cross-referencing TODO.md against DESIGN.md sees inconsistent active-domain counts and has no way to know which is current without reading CHANGELOG.

Proposed fix: update TODO.md:5 from "10 active domains" to "12 active domains" (or "11 role + 1 meta = 12"). Update TODO.md:40 layer-gate criterion 4 to add `[Documentation Reviewer](../../vsdd-suite/domains/role/DOCUMENTATION-REVIEWER-REVIEW.md)` to the capstone-tier extended list (and adjust the total accordingly).

**Classification:** Deferred — finding raised in Round 1; Round 2 verification required per the Phase 3 primer's continue-trigger discipline.

---

**Finding 9 — `vsdd-suite/FINDINGS-INDEX.md` has two stale cross-reference claims (Dim 4, Dim 6)**

<a id="r1-f9"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** technical-writer

[`vsdd-suite/FINDINGS-INDEX.md`](../FINDINGS-INDEX.md):38 — "DESIGN.md — Phase 1a contract" — but the current DESIGN.md H1 is "Phase 1a+1b contract" per the v0.6.0 / G-160 rename documented in [`CHANGELOG.md`](../../CHANGELOG.md):85. The Cross-references list in FINDINGS-INDEX.md still uses the pre-rename label.

[`vsdd-suite/FINDINGS-INDEX.md`](../FINDINGS-INDEX.md):40 — "Per-domain index files in this directory — round-level rollup per domain (QUALITY-ENGINEER-REVIEW.md and SOLUTION-ARCHITECT-REVIEW.md customized; **five others remain as scaffolded stubs**)" — but per [`CHANGELOG.md`](../../CHANGELOG.md):35-36 PR 6 customized 4 newly-capstone-activated extended domains plus 5 pre-existing scaffolded stubs (9 total customized); per [`CHANGELOG.md`](../../CHANGELOG.md):13 PR `#36` added the Documentation Reviewer index stub (customized at activation). The "five others remain as scaffolded stubs" claim is from the PR-5-or-earlier era and is now several PRs stale.

Documentation rot (Dim 6) — both claims are factually wrong against the current state. Cross-reference resolution (Dim 4) — the labels "Phase 1a contract" and "five others remain as scaffolded stubs" mislead a cold reader about what they will find when they open the cited artifacts.

Proposed fix: FINDINGS-INDEX.md:38 → "DESIGN.md — Phase 1a+1b contract"; FINDINGS-INDEX.md:40 → restate the customization status against the current count (12 active-domain index files; all 11 role-domain stubs customized to bookmark-cli-manual per PR 6 + PR `#36`; the VDD-IAR Alignment meta-domain stub is also customized; the only-scaffold-stub state no longer applies to any per-domain index).

**Classification:** Deferred — finding raised in Round 1; Round 2 verification required per the Phase 3 primer's continue-trigger discipline.

---

**Finding 10 — `manual-tests/install-verification.md` cross-references to `manual-tests/layer-1.md` use a relative path that doesn't resolve from the install-verification.md location (Dim 4, Dim 11)**

<a id="r1-f10"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** technical-writer

[`manual-tests/install-verification.md`](../../manual-tests/install-verification.md) is located at `bookmark-cli-manual/manual-tests/install-verification.md`. Two of its links to the sibling layer-1 test plan use the wrong relative path:

- Line 43: `Follow [`manual-tests/layer-1.md`](manual-tests/layer-1.md) end-to-end.`
- Line 63: `**[`manual-tests/layer-1.md`](manual-tests/layer-1.md)** — the test plan the verifier executes.`

Both links use `manual-tests/layer-1.md` as the relative target. From `manual-tests/install-verification.md`, that path resolves to `manual-tests/manual-tests/layer-1.md` — a doubled directory that does not exist. The correct relative target is simply `layer-1.md` (sibling file in the same directory).

The link text `[`manual-tests/layer-1.md`](...)` is reasonable — the operator wanted to communicate the project-relative path. But the target inside the parentheses needs to be `layer-1.md` for the link itself to resolve from the file's actual location. Alternatively, the target can be `../manual-tests/layer-1.md` if the link is intended to be portfolio-root-relative-via-up — but the sibling form is cleaner.

Inline-reference clickthrough validation (Dim 11) — both links 404 from the file's actual location. Cross-reference resolution (Dim 4) — the paths don't resolve.

Proposed fix: change both `manual-tests/layer-1.md` link targets to `layer-1.md` (keeping the visible link text as `manual-tests/layer-1.md` if the project-relative-path display is wanted, or shortening it to `layer-1.md` for consistency with the target).

**Classification:** Deferred — finding raised in Round 1; Round 2 verification required per the Phase 3 primer's continue-trigger discipline.

---

**Finding 11 — `manual-tests/layer-1.md` Step 5 expects "bm not found" from `which bm` after uninstall, but most shells emit different output (Dim 8)**

<a id="r1-f11"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** technical-writer

[`manual-tests/layer-1.md`](../../manual-tests/layer-1.md):162-178 prescribes:

```sh
# Confirm `bm` is gone
which bm 2>&1
echo "uninstall-exit: $?"
```

with Expected output (literal):

```
bm not found
```

The literal string "bm not found" is not what `which` produces on standard macOS / Linux toolchains:

- macOS `/usr/bin/which`: prints nothing to stdout, exits 1.
- Linux GNU `which` (debianutils): prints nothing to stdout, exits 1.
- `zsh` builtin `which`: returns `bm not found` on some versions, but `bm: aliased to ...` style on others.
- `bash` builtin: `which` is not a bash builtin; relies on `/usr/bin/which`.

A cold reader on macOS running the documented `which bm 2>&1` after uninstall will see *no output at all* and exit code 1, not "bm not found". The "Expected output (literal)" block tells them to expect "bm not found" and the documentation's "literal" framing means a divergence is a finding per the manual-test discipline.

Manual-test plan executability (Dim 8) — the expected output does not match what the user's shell will emit on the most common platforms. The literal-vs-invariant discipline applied elsewhere in the file (Step 1's "the URL is invariant; the timestamp is variable") is the right shape; Step 5's `which bm` expected output needs the same treatment.

Proposed fix: replace the expected-output block with an invariant statement: "`which bm` returns exit code 1 with no resolution; the binary is gone from PATH." Or: use `command -v bm; echo "exit: $?"` which is portable and produces a deterministic empty-output-plus-exit-1 result. Or: relax the literal-match to a behavioral check — "if `which bm` exits non-zero, the uninstall succeeded; if it still resolves, the install didn't fully remove the binary."

A weaker but still acceptable fix: leave the command, but annotate that the literal "bm not found" string is `zsh`-specific and that other shells emit no output; the invariant is exit code 1.

**Classification:** Deferred — finding raised in Round 1; Round 2 verification required per the Phase 3 primer's continue-trigger discipline.

---

**Finding 12 — `README.md` and `TODO.md` use "VSDD" / "IAR" / "MVR" without first-use expansion anywhere in the project's docs (Dim 2)**

<a id="r1-f12"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** technical-writer

The implicit-knowledge audit (Dim 2) maintains a reader-built glossary as you read; any term used without expansion is a finding. Cold-reader pass over the project's user-facing docs found these undefined abbreviations:

- "VSDD" appears in [`README.md`](../../README.md):7, :47, [`DESIGN.md`](../../DESIGN.md):3 (referenced in the file's preamble) and :11, [`TODO.md`](../../TODO.md):5, [`PROCESS.md`](../../PROCESS.md):3, [`CHANGELOG.md`](../../CHANGELOG.md) (multiple), [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md):4, [`manual-tests/install-verification.md`](../../manual-tests/install-verification.md):3, and the per-domain index files. **Nowhere is VSDD expanded to "Verified Spec-Driven Development" in any project-owned markdown.** Verified by `grep -rni "verified spec.driven" bookmark-cli-manual/ --include="*.md"` — zero matches. The reader follows the suite link to learn what VSDD stands for, which only works if (a) the link resolves (it doesn't, per Finding 3) and (b) the reader is willing to navigate out of the project. For a forward-facing reference example documenting an audit trail, expansion in the project's own README is the standard discipline ([`markdown.md` § GitHub render-target conventions § Acronyms and abbreviations](../../../vsdd-suite/supplements/markdown.md#github-render-target-conventions): "Spell out on first use; abbreviate after").
- "IAR" appears in [`TODO.md`](../../TODO.md):5, [`PROCESS.md`](../../PROCESS.md):21 (indirectly via "IAR" usage in cited contexts), [`DESIGN.md`](../../DESIGN.md):133 ("**IAR Phase 3** runs the 7 default-active core domains"), and the per-domain index files. Not expanded anywhere in the project's own docs.
- "MVR" appears in [`DESIGN.md`](../../DESIGN.md):17 ("Spec MVR (DESIGN.md round closure); Test MVR (...); Implementation MVR (...); Formal-verification MVR (...)"). Not expanded.

Implicit-knowledge audit (Dim 2) — a cold reader following the reference-example chain (which is the whole purpose of `bookmark-cli-manual` per [`README.md`](../../README.md):7 "exists to validate the suite's documented workflow") opens the README and immediately hits "VSDD Suite" with no expansion. The link to the suite README that would supply the expansion is broken (Finding 3). Even if it weren't broken, requiring an out-of-project navigation to expand the project's most-frequently-used abbreviation is poor first-use discipline.

Proposed fix: at the first mention in [`README.md`](../../README.md):7, change `[VSDD Suite](../../vsdd-suite/README.md)'s` to `[VSDD (Verified Spec-Driven Development) Suite](../../vsdd-suite/README.md)'s` — single-place expansion. Same shape in DESIGN.md § Project intent and TODO.md preamble. For IAR, expand at first use as "IAR (Iterative Adversarial Refinement)" or use the whitepaper's preferred name "Adversarial Refinement". For MVR, expand as "MVR (maximum viable refinement)" at first use in DESIGN.md.

**Classification:** Deferred — finding raised in Round 1; Round 2 verification required per the Phase 3 primer's continue-trigger discipline.

---

**Finding 13 — `manual-tests/layer-1.md` step boundaries silently assume a single uninterrupted shell session (Dim 1, Dim 8)**

<a id="r1-f13"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** technical-writer

[`manual-tests/layer-1.md`](../../manual-tests/layer-1.md) prescribes 6 steps (0 through 5) plus Cleanup, each in its own fenced block. The plan implicitly assumes the cold reader executes all six in the SAME shell session because:

- Step 1:37 exports `BOOKMARK_CLI_DB="$(mktemp -d)/bookmarks.json"`.
- Steps 2-5 then reference `$BOOKMARK_CLI_DB` (via `bm`, `cat`, `ls`, `rm`) without re-exporting.
- Step 5:166 issues `cd vsdd-suite-reference-examples/bookmark-cli-manual` — relative to wherever the shell was at session start; only works if the shell stayed at the portfolio root through the entire run.

There is no preamble naming this requirement. A cold reader who runs Step 0, switches to a new terminal for Step 1 (or who pastes Step 5's block into a fresh shell to test re-install), or whose shell session times out mid-run, will hit:

- `bm: BOOKMARK_CLI_DB is unset` or `$BOOKMARK_CLI_DB` resolves to empty → silent failure.
- `cd: no such file or directory` if the shell's cwd was different at Step 5 invocation.

Manual-test plan executability (Dim 8) — the implicit single-session assumption is a setup-step that isn't named. Clone-and-follow fidelity (Dim 1) — a reader who pauses mid-test loses state silently.

Proposed fix: add a preamble paragraph after the Authoring note: "Run all steps in a single shell session. Step 1 exports `BOOKMARK_CLI_DB`; subsequent steps depend on the export. Step 5's `cd` is relative to the directory the session started in (the portfolio root)." Better: each Step that depends on a prior step's environment state names the dependency explicitly. Best: each Step is self-contained — re-export `BOOKMARK_CLI_DB` at the top of every Step that uses it, and use absolute or `cd`-relative-to-known-anchor paths.

**Classification:** Deferred — finding raised in Round 1; Round 2 verification required per the Phase 3 primer's continue-trigger discipline.

---

### Deferred

*(none — every finding above is filed against the current docs and is actionable in this Round / its successor TW round.)*

---

### Dismissed

*(none.)*

---

### Hallucinated

*(none — every Finding above was verified against the current project state with file:line citations and direct cross-file comparison. The sycophancy-guard pre-classification was applied: each candidate finding was tested against the question "is this just the cold reader inventing a defect, or does the docs-as-the-contract test actually fail here?" — every finding kept passed the test by demonstrating an observable cold-reader divergence.)*

---

### Summary

13 findings filed, all Open. Source: `domain-raised`. Owners: 13 × `technical-writer` (the natural pair for documentation defects, per the Documentation Reviewer ↔ Technical Writer adversarial-pair convention registered in [Review 77](../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-77--2026-05-20-1545z) and [Review 80](../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z)). Validator: `technical-writer` on each — TW closes the loop from the authorial seat after applying the proposed fix; if TW disputes the framing, the disputed finding routes to `sanity-check` per the meta-validator-of-last-resort pattern ([Review 77](../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-77--2026-05-20-1545z) Finding 2).

**Finding progression (against the [G-131](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-131) continue trigger):** 13 real findings → Round 2 is mandatory once the TW fix pass lands. The fix pass should close the broken-link defects (Findings 3, 4, 5, 10) mechanically; the doc-rot / stale-claim defects (Findings 1, 2, 6, 7, 8, 9, 11, 12, 13) need substantive editorial work. Round 2 verifies the fixes held and looks for adjacent defects the fix may have created (especially Finding 6 / 7 — the descriptive-name migration sweep is precisely the kind of edit that produces new stutters and adjacent stale references).

**Coordination:** This round is the first [Documentation Reviewer](../../../vsdd-suite/domains/role/DOCUMENTATION-REVIEWER-REVIEW.md) round filed against `bookmark-cli-manual`; the [Technical Writer](../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) round filed in PR `#36` is the natural pair for validation. Several findings overlap with TW concerns — notably Finding 6 (lettering / abbreviation lookup cost — cold-reader pair to [TW Dim 12](../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md)) and Findings 3 / 4 / 5 / 10 (inline-reference clickthrough validation — cold-reader pair to [TW Dim 13](../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md)). If the TW round filed those defects from the authorial seat first, this round's findings are the cold-reader confirmation; if TW missed them, this round surfaces them and the adversarial-pair shape is the discipline working as designed.

**Note on `PROT_*` Read-tool display artifacts:** While reading [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md) and [`DESIGN.md`](../../DESIGN.md) with the Read tool, several Step / Section headings displayed `PROT_30`, `PROT_37`, `PROT_40`, `PROT_41`, `PROT_46` placeholder tokens in place of certain protocol-named CLI labels (e.g., the `bm add` contract heading, manual-test step titles for the happy-path / ordering / empty-state cases). `grep -rE "PROT_" bookmark-cli-manual/` returned zero matches across the project, confirming the tokens are Read-tool display sanitization, not file contents. They are NOT a finding against the docs.

---

## Review 2 — 2026-05-20 21:00Z

**Layer:** 1
**Tested against:** post-Round-2-fix-cycle state of `bookmark-cli-manual` (CHANGELOG v0.11.4 entry; current working tree as of 2026-05-20).
**Round:** 2
**Active domain set:** 11 role + 1 meta = 12 (per [DESIGN.md § Project intent](../../DESIGN.md)).
**Scope:** Phase 3 [Adversarial Refinement](../../../../vsdd-suite/primers/3-review-session.md) Round 2 verification of [Round 1 Documentation Reviewer findings](2026-05-20-documentation-reviewer.md) (F1–F13) against the v0.11.4 doc-batch fix cycle. Independent adversarial cold-reader pass also looks for adjacent defects the fix cycle may have created or missed per the Phase 3 primer's [G-131](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-131) continue-trigger framing ("the Round N+1 cold pass verifies the fix held and looks for adjacent defects the fix may have created"). Read in cold-reader order: [`README.md`](../../README.md) → [`TODO.md`](../../TODO.md) → [`CHANGELOG.md`](../../CHANGELOG.md) → [`PROCESS.md`](../../PROCESS.md) → [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md) → [`manual-tests/install-verification.md`](../../manual-tests/install-verification.md) → [`DESIGN.md`](../../DESIGN.md) (last per the [Documentation Reviewer](../../../../vsdd-suite/domains/role/DOCUMENTATION-REVIEWER-REVIEW.md) discipline). Per-domain index files in [`vsdd-suite/`](..) consulted only as cited targets of links in the user-facing artifacts above. Round 1 log re-read against the current state for the regression check.
**Lens:** Adversarial cold-reader pair to [Technical Writer](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md). The 11 Documentation Reviewer dimensions re-applied to the post-fix state: clone-and-follow fidelity (Dim 1), implicit-knowledge audit (Dim 2 — reader-built glossary), forward-reference safety (Dim 3), cross-reference resolution (Dim 4 — every `[text](path)` opened-and-confirmed), audience-fit calibration (Dim 5 — capstone reader profile), documentation rot (Dim 6 — every claim verified against current code/spec/process), recovery-from-confusion (Dim 7), manual-test plan executability (Dim 8), onboarding sequencing (Dim 9), manual-test file structure consistency (Dim 10 — Review 74 convention), inline-reference clickthrough validation (Dim 11 — every markdown link opened to verify it resolves).
**Source:** `domain-raised` — every finding below was elicited by applying the 11 Documentation Reviewer dimensions to the post-fix-cycle user-facing artifacts in cold context. No director-raised observations interrupted this round.
**Regression check:** Each Round 1 finding (F1–F13) was re-verified against the current state by re-locating the cited file:line and inspecting against the proposed fix in the Round 1 entry and the CHANGELOG v0.11.4 entry's claimed disposition. Outcomes recorded under § Resolved (fix held) and § Deferred (fix incomplete — proposed fix landed in some sites but not all the sites the original finding named). The CHANGELOG entry's *claim* of resolution was treated as a claim, not a fact — the verification reads the docs themselves.
**Validator:** `technical-writer` on each finding per [Review 80](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z) validator-pair convention.

**Session note:** Cold-context AI session — did not author the Round 2 fix cycle. The primer's sycophancy guard applied: each Round 1 finding was re-tested by re-reading the cited file:line in the current state, not by trusting the CHANGELOG's claim that the fix landed. Several Round 1 findings the CHANGELOG claims as resolved are demonstrably *still present* in the current state — those route to § Deferred as "fix incomplete," not § Resolved. The adversarial finding-progression discipline applies: a Resolved/Resolved pair across two rounds is the MVR signal; a Resolved-claim-with-the-defect-still-present is precisely the failure mode this round exists to catch.

**MVR signal:** **NOT reached.** New real findings surface in this round (R2-F1 through R2-F7 below; six fix-incomplete + one new adjacent-defect). Per the [G-131](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-131) continue trigger, a Round 3 is mandatory after the next fix pass lands; the Round 3 cold pass verifies the fix completion held and looks for further adjacent defects.

---

### Resolved

**Finding 1 — `README.md` install instructions name a directory that does not exist (Dim 1, Dim 4, Dim 6)**

<a id="r2-f1"></a>

**Owner:** technical-writer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** technical-writer

[Round 1 Finding 1](2026-05-20-documentation-reviewer.md#r1-f1) raised that [`README.md`](../../README.md) directed `cd <portfolio>/bookmark-cli` (and again `cd <portfolio>/bookmark-cli` in the Test section), but no such directory exists; the actual project is at `vsdd-suite-reference-examples/bookmark-cli-manual/`. Verifying the Round 2 fix:

1. [`README.md`](../../README.md):20 now reads `cd <portfolio>/vsdd-suite-reference-examples/bookmark-cli-manual` — the correct path.
2. [`README.md`](../../README.md):41 (Test section) now reads `cd <portfolio>/vsdd-suite-reference-examples/bookmark-cli-manual` — consistent with the Install section.
3. [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md):15 uses `cd vsdd-suite-reference-examples/bookmark-cli-manual` (portfolio-relative form, consistent with the README path now). The cross-doc consistency Round 1 flagged is intact.

Cold-reader clone-and-follow test passes: a reader following [`README.md`](../../README.md):16-23 verbatim arrives at the correct directory. **Resolution:** fix held; finding closed (Dim 1, Dim 4, Dim 6).

---

**Finding 2 — `README.md` claims "8 tests pass" (Dim 6)**

<a id="r2-f2"></a>

**Owner:** technical-writer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** technical-writer

[Round 1 Finding 2](2026-05-20-documentation-reviewer.md#r1-f2) raised the stale "8 tests pass (4 lib + 4 integration)" claim. The Round 1 proposed fix preferred the stability-friendly framing ("expect: all tests pass with no failures"); the actual fix applied lands close to that shape. Verifying:

1. [`README.md`](../../README.md):42-44 now reads `# expect: all tests pass — the test suite (currently ~19 lib + integration tests at Layer 1, post-Round-2 fix cycle) covers the behavioral contracts in DESIGN.md.` The literal-count was relaxed to a behavioral invariant ("all tests pass") + a parenthesized "currently ~19" advisory. The `~` hedges any minor drift; the load-bearing claim is "all tests pass."
2. Verified against current state: `cargo test` against the current `main` reports `11 passed + 0 passed + 10 passed + 0 passed = 21 tests pass`. The "~19" hedge is generous enough to absorb the +2 drift (Round 2 added test coverage as cited in CHANGELOG v0.11.4 § Changed — code); a cold reader running `cargo test` sees 21 pass and the README's "all tests pass — currently ~19" is consistent with that (the `~` carries the imprecision).

Cold-reader clone-and-follow test passes: the reader's `cargo test` output matches the README's "all tests pass" claim. **Resolution:** fix held; finding closed (Dim 6).

---

**Finding 8 — `TODO.md` capstone-active-domain count and layer-gate criterion are stale (Dim 6)**

<a id="r2-f8"></a>

**Owner:** technical-writer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** technical-writer

[Round 1 Finding 8](2026-05-20-documentation-reviewer.md#r1-f8) raised the "10 active domains" count and the missing-from-layer-gate-criterion-4 Documentation Reviewer reference. Verifying:

1. [`TODO.md`](../../TODO.md):5 now reads "`→ 3 IAR (Iterative Adversarial Refinement) (12 active domains)`" — count updated from 10 to 12 matching [`DESIGN.md`](../../DESIGN.md):11.
2. [`TODO.md`](../../TODO.md):40 layer-gate criterion 4 now includes `[Documentation Reviewer](../../vsdd-suite/domains/role/DOCUMENTATION-REVIEWER-REVIEW.md)` in the capstone-extended list.
3. The total now reconciles: 7 cores + 5 extended (Performance Engineer, Red Team, Platform Engineer, Technical Writer, Documentation Reviewer) = 12, matching the active domain set declared in DESIGN.md.

Cross-doc consistency check: TODO.md count matches DESIGN.md count matches the actual active-domain set. **Resolution:** fix held; finding closed (Dim 6).

---

**Finding 10 — `manual-tests/install-verification.md` cross-references to `manual-tests/layer-1.md` use a relative path that doesn't resolve (Dim 4, Dim 11)**

<a id="r2-f10"></a>

**Owner:** technical-writer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** technical-writer

[Round 1 Finding 10](2026-05-20-documentation-reviewer.md#r1-f10) raised the doubled-directory `manual-tests/manual-tests/layer-1.md` defect on two sibling-link sites in install-verification.md (Step 3 prose at line 43 and Coordination list at line 63). Verifying:

1. [`manual-tests/install-verification.md`](../../manual-tests/install-verification.md):43 now reads `Follow [`layer-1.md`](layer-1.md) (sibling file in this directory) end-to-end.` — sibling path resolves correctly.
2. [`manual-tests/install-verification.md`](../../manual-tests/install-verification.md):63 now reads `**[`layer-1.md`](layer-1.md)** — the test plan the verifier executes (sibling file in this `manual-tests/` directory).` — sibling path resolves correctly.
3. The CHANGELOG also documents companion sibling-link corrections for [`../PROCESS.md`](../../PROCESS.md) and [`../DESIGN.md`](../../DESIGN.md) references; both are present at lines 64 and 66 with `../` prefix and the visible text drops the doubled-path display in favor of the sibling-relative form.

Cold-reader clickthrough test passes: every link in [`manual-tests/install-verification.md`](../../manual-tests/install-verification.md) resolves to its intended target from the file's actual on-disk location. **Resolution:** fix held; finding closed (Dim 4, Dim 11).

---

**Finding 11 — `manual-tests/layer-1.md` Step 5 expects "bm not found" from `which bm` after uninstall (Dim 8)**

<a id="r2-f11"></a>

**Owner:** technical-writer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** technical-writer

[Round 1 Finding 11](2026-05-20-documentation-reviewer.md#r1-f11) raised the literal-match `bm not found` expected-output defect (not what `/usr/bin/which` emits on macOS or Linux). Verifying:

1. [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md):187 now reads: "Expected behavior for `which bm 2>&1` after uninstall: the `which bm` command MUST exit non-zero AND MUST NOT print a path. The exact textual output is shell-dependent (bash: typically empty stdout/stderr + exit 1; zsh: may print `bm not found` + exit 1; BSD `which` on macOS: typically empty + exit 1); do not assert on it." — the Round 1 proposed-fix's invariant-based framing is adopted verbatim.
2. The accompanying exit-code expectation at line 192 (`uninstall-exit: 1`) is the invariant; the textual output is explicitly annotated as shell-dependent.

Cold-reader clone-and-follow test passes: the manual-test plan now matches the deterministic invariant (exit 1 + no path printed) rather than a shell-specific text shape. **Resolution:** fix held; finding closed (Dim 8).

---

**Finding 12 — `README.md` and `TODO.md` use "VSDD" / "IAR" / "MVR" without first-use expansion (Dim 2)**

<a id="r2-f12"></a>

**Owner:** technical-writer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** technical-writer

[Round 1 Finding 12](2026-05-20-documentation-reviewer.md#r1-f12) raised the absence of acronym expansion for VSDD / IAR / MVR / TDD anywhere in the project's docs. Verifying:

1. [`README.md`](../../README.md):7 now reads `[VSDD (Verified Spec-Driven Development) Suite](../../vsdd-suite/README.md)` — VSDD expanded on first use.
2. [`README.md`](../../README.md):48 expands IAR ("IAR (Iterative Adversarial Refinement)"), MVR ("MVR (maximum viable refinement)"), and TDD ("TDD (test-driven development)") on first use in the "How this was built" section.
3. [`TODO.md`](../../TODO.md):5 reads `VSDD (Verified Spec-Driven Development) phases ... 3 IAR (Iterative Adversarial Refinement) (12 active domains)` — both VSDD and IAR expanded on first use.
4. [`TODO.md`](../../TODO.md):40 expands MVR on first use within that file ("each domain reaches MVR (maximum viable refinement)").

Cold-reader implicit-knowledge audit (Dim 2) test passes: a reader's reader-built glossary now has explicit expansions for VSDD, IAR, MVR, and TDD without leaving the project's own docs. **Resolution:** fix held; finding closed (Dim 2).

---

**Finding 13 — `manual-tests/layer-1.md` step boundaries silently assume a single uninterrupted shell session (Dim 1, Dim 8)**

<a id="r2-f13"></a>

**Owner:** technical-writer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** technical-writer

[Round 1 Finding 13](2026-05-20-documentation-reviewer.md#r1-f13) raised the implicit single-session assumption (Step 1's `BOOKMARK_CLI_DB` export + Step 5's relative `cd`). Verifying:

1. [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md):8 has a new **Session-state preamble** that names the single-uninterrupted-shell-session requirement AND offers the alternative ("set `BOOKMARK_CLI_DB` to a stable absolute path ... so each subsequent step is independent of the prior shell state"). This is the Round 1 "Better" proposed fix.
2. [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md):169 Step 5 now captures `PROJECT_DIR="$(pwd)"` before uninstall and `cd "$PROJECT_DIR"` for the reinstall — the Step 5 cwd dependency is now explicit rather than implicit.

Cold-reader clone-and-follow test passes: a reader who pauses mid-test or starts a new terminal mid-test is given the explicit setup discipline up-front. **Resolution:** fix held; finding closed (Dim 1, Dim 8).

---

### Deferred

**Finding 1 — `DESIGN.md` line 3 still uses broken `1ab-spec-development.md` primer reference (Round 1 F4 fix incomplete) (Dim 4, Dim 6, Dim 11)**

<a id="r2-f1-deferred"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** technical-writer

[Round 1 Finding 4](2026-05-20-documentation-reviewer.md#r1-f4) raised that four user-facing artifacts (PROCESS.md, DESIGN.md, SOLUTION-ARCHITECT-REVIEW.md, QUALITY-ENGINEER-REVIEW.md) link to a non-existent `1ab-spec-development.md` primer; the actual primer is at `1ab-spec-crystallization.md`. The Round 2 [CHANGELOG.md](../../CHANGELOG.md):36 entry claims the fix was applied to PROCESS.md only — the other three sites are not mentioned in the doc-batch entry. Verifying:

- [`PROCESS.md`](../../PROCESS.md):21 — fixed (now uses `1ab-spec-crystallization.md`).
- [`DESIGN.md`](../../DESIGN.md):3 — **still broken**: reads `[Phase 1a+1b](../../vsdd-suite/primers/1ab-spec-development.md) contract`. The same line ALSO retains the wrong-depth `../vsdd-suite/...` paths flagged in Round 1 Finding 3 below — both defects coexist on the same line.
- [`vsdd-suite/SOLUTION-ARCHITECT-REVIEW.md`](../SOLUTION-ARCHITECT-REVIEW.md):21 — **still broken**: reads `Routed via [Phase 4](...) to [Phase 1a+1b](../../../vsdd-suite/primers/1ab-spec-development.md)`.
- [`vsdd-suite/QUALITY-ENGINEER-REVIEW.md`](../QUALITY-ENGINEER-REVIEW.md):21 — **still broken**: reads `routed via [Phase 4](...) to [Phase 1a+1b](../../../vsdd-suite/primers/1ab-spec-development.md)`.

Clickthrough validation (Dim 11): three of the four broken links 404 against the current `vsdd-suite/primers/` directory (`ls vsdd-suite/primers/` returns `1ab-spec-crystallization.md`, not `1ab-spec-development.md`). Cross-reference resolution (Dim 4): the three remaining sites point at a path that does not resolve. Documentation rot (Dim 6): the CHANGELOG claims the fix landed, but the docs themselves contradict the claim — that internal contradiction is its own doc-rot signal beyond the path-resolution defect.

The Round 1 proposed fix prescribed a search-and-replace across all four files; the fix pass applied it to only one. A literal `grep -rn "1ab-spec-development" bookmark-cli-manual/` confirms three live sites remain.

Proposed fix: search-and-replace `1ab-spec-development.md` → `1ab-spec-crystallization.md` in the three remaining files (DESIGN.md:3, SOLUTION-ARCHITECT-REVIEW.md:21, QUALITY-ENGINEER-REVIEW.md:21). Update the [CHANGELOG.md](../../CHANGELOG.md):36 doc-batch entry to name the three additional sites — the current CHANGELOG entry reads "broken primer reference 1ab-spec-development.md → 1ab-spec-crystallization.md corrected" against PROCESS.md only, which is itself a doc-rot signal because it overstates the scope of the fix.

**Classification:** Deferred — Round 1 finding's fix is incomplete; Round 3 verification required once the remaining three sites are corrected (Dim 4, Dim 6, Dim 11).

---

**Finding 2 — `DESIGN.md` line 3 still uses wrong relative-depth `../vsdd-suite/...` paths (Round 1 F3 fix incomplete) (Dim 4, Dim 11)**

<a id="r2-f2-deferred"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** technical-writer

[Round 1 Finding 3](2026-05-20-documentation-reviewer.md#r1-f3) raised that README.md and DESIGN.md line 3 used `../vsdd-suite/...` single-dot relative paths that resolve to `vsdd-suite-reference-examples/vsdd-suite/...` (which does not exist) rather than the portfolio-root `../../vsdd-suite/...` form. The Round 2 [CHANGELOG.md](../../CHANGELOG.md):32 entry claims the fix was applied to README.md (`relative-depth fix ../vsdd-suite/README.md → ../../vsdd-suite/README.md`). DESIGN.md is not mentioned in the doc-batch entry. Verifying:

- [`README.md`](../../README.md):7 — fixed (uses `../../vsdd-suite/README.md`).
- [`README.md`](../../README.md):47 (current line; was line 47 in Round 1) — fixed (uses `../../vsdd-suite/README.md`).
- [`DESIGN.md`](../../DESIGN.md):3 — **still broken**: contains `[`../vsdd-suite/primers/1ab-spec-crystallization.md`](../vsdd-suite/primers/1ab-spec-crystallization.md)` AND `[`../vsdd-suite/README.md`](../vsdd-suite/README.md)`. Both single-`..` paths. From `vsdd-suite-reference-examples/bookmark-cli-manual/DESIGN.md`, both resolve to non-existent sibling paths.

The same line also retains the broken `1ab-spec-development.md` (R2 Finding 1 Deferred above) — the two Round 1 Findings 3 and 4 share a host line and the fix pass closed neither.

Clickthrough validation (Dim 11): both links on DESIGN.md:3 404 against the current portfolio layout. Cross-reference resolution (Dim 4): the cited paths don't resolve.

Proposed fix: in [`DESIGN.md`](../../DESIGN.md):3, change `../vsdd-suite/primers/1ab-spec-crystallization.md` → `../../vsdd-suite/primers/1ab-spec-crystallization.md` and `../vsdd-suite/README.md` → `../../vsdd-suite/README.md`. Combine with the R2-F1-Deferred edit on the same line. Also update [CHANGELOG.md](../../CHANGELOG.md):32 to reflect that DESIGN.md was also part of the relative-depth fix — the entry currently mentions README.md only, which is incomplete against what the fix pass should have done.

**Classification:** Deferred — Round 1 finding's fix is incomplete; Round 3 verification required (Dim 4, Dim 11).

---

**Finding 3 — Per-domain index lines still contain "Mutation Testing Mutation Testing" / "Purity Boundary Audit Purity Boundary Audit" stutters (Round 1 F7 fix incomplete) (Dim 6)**

<a id="r2-f3-deferred"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** technical-writer

[Round 1 Finding 7](2026-05-20-documentation-reviewer.md#r1-f7) raised mechanical search/replace stutter artifacts from the descriptive-naming migration. The Round 2 [CHANGELOG.md](../../CHANGELOG.md):33 entry mentions the lettering migration in TODO.md but does not name the stutter cleanup. Verifying:

- [`DESIGN.md`](../../DESIGN.md):15 — **still contains** `Mutation Testing (Mutation Testing via cargo-mutants)` and `property-based testing (property-based testing via proptest)` (two stutters on one line).
- [`DESIGN.md`](../../DESIGN.md):17 — **still contains** `Purity Boundary Audit Purity Boundary Audit + Mutation Testing Mutation Testing closure` (two stutters in one phrase).
- [`vsdd-suite/SOLUTION-ARCHITECT-REVIEW.md`](../SOLUTION-ARCHITECT-REVIEW.md):21 — **still contains** `Phase 5 Purity Boundary Audit Purity Boundary Audit` and `Companion QE round (Mutation Testing Mutation Testing)`.
- [`vsdd-suite/QUALITY-ENGINEER-REVIEW.md`](../QUALITY-ENGINEER-REVIEW.md):21 — **still contains** `**Phase 5 hardening:** Mutation Testing — Mutation Testing for Layer 1 via cargo-mutants` (Round 1 flagged this as borderline; given the consistent stutter pattern across the other forward-facing sites, treating it as a stutter is the consistent disposition — the [G-177](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-177) canonical preamble-tag shape is `**Phase 5 hardening:** <name> — <scope>` where `<scope>` is "for Layer 1 via cargo-mutants", NOT a re-statement of the surface name).
- [`PROCESS.md`](../../PROCESS.md):23 — **still contains** `the Phase 5 Purity Boundary Audit Purity Boundary Audit`.
- [`PROCESS.md`](../../PROCESS.md):39 — **still contains** `The Phase 5 Purity Boundary Audit Purity Boundary Audit produced the cross-source divergence finding`.

The CHANGELOG v0.11.4 § Changed — docs § TODO.md entry mentions "retired letter-coded 'Surface A.0 / B' verbiage replaced with descriptive 'Purity Boundary Audit + Mutation Testing' Title-Case names" — but does NOT mention the stutter cleanup across DESIGN.md, the per-domain index files, or PROCESS.md. The TODO.md migration completed; the broader stutter sweep did not.

Documentation rot (Dim 6) — the stutters survive across six sites that the Round 1 finding explicitly cited (DESIGN.md:15, DESIGN.md:17, SOLUTION-ARCHITECT-REVIEW.md:21, QUALITY-ENGINEER-REVIEW.md:21) plus two adjacent sites (PROCESS.md:23, PROCESS.md:39) that the fix sweep should have caught.

Proposed fix: a targeted search-and-replace pass on the stutter patterns:
- `Mutation Testing Mutation Testing` → `Mutation Testing`
- `Purity Boundary Audit Purity Boundary Audit` → `Purity Boundary Audit`
- `property-based testing (property-based testing via proptest)` → `property-based testing (via proptest)`
- `Mutation Testing (Mutation Testing via cargo-mutants)` → `Mutation Testing (via cargo-mutants)`

Apply across DESIGN.md, SOLUTION-ARCHITECT-REVIEW.md, QUALITY-ENGINEER-REVIEW.md, and PROCESS.md (forward-facing prose only; CHANGELOG historical narrative preserved per [G-89](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-89)).

**Classification:** Deferred — Round 1 finding's fix is incomplete; Round 3 verification required (Dim 6).

---

**Finding 4 — `SOLUTION-ARCHITECT-REVIEW.md` companion-round link still cites "QE Review 1" with broken anchor `#review-1--2026-05-20-0245z` (Round 1 F5 fix incomplete) (Dim 4, Dim 11)**

<a id="r2-f4-deferred"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** technical-writer

[Round 1 Finding 5](2026-05-20-documentation-reviewer.md#r1-f5) raised the two-defect citation in SOLUTION-ARCHITECT-REVIEW.md:21: link text says "QE Review 1" but the round is Review 2, and the anchor `#review-1--2026-05-20-0245z` does not resolve because the QE file's heading is `## Review 2 — 2026-05-20 02:45Z`. The Round 2 CHANGELOG does NOT mention this fix anywhere in the doc-batch entry. Verifying:

[`vsdd-suite/SOLUTION-ARCHITECT-REVIEW.md`](../SOLUTION-ARCHITECT-REVIEW.md):21 currently reads:

> Companion QE round (Mutation Testing Mutation Testing) at [QE Review 1](2026-05-20-quality-engineer.md#review-1--2026-05-20-0245z).

Both Round 1 defects still present:

1. Link text "QE Review 1" — the QE file at `vsdd-suite/review-log/2026-05-20-quality-engineer.md`:9 begins with `## Review 2 — 2026-05-20 02:45Z`; there is no Review 1 heading in that file. (Review 1 lives in `2026-05-17-quality-engineer.md`.) So the link text mis-cites the round.
2. Anchor `#review-1--2026-05-20-0245z` — the actual anchor on the target file is `#review-2--2026-05-20-0245z`. Clicking the link lands on the file but at no heading.

Clickthrough validation (Dim 11) fails: the anchor does not resolve. Cross-reference resolution (Dim 4) fails: the cited round + anchor do not match the target's actual structure. This Round 1 finding is also tracked in [`vsdd-suite/FINDINGS-INDEX.md`](../FINDINGS-INDEX.md):27 as F-027 (VDD-IAR-A R1 F5) classified Resolved — but the FINDINGS-INDEX classification does not match the docs' actual state, which is itself a Dim 6 doc-rot finding registered separately as R2-F6-Deferred below.

Proposed fix: change link text from `QE Review 1` to `QE Review 2`; change anchor from `#review-1--2026-05-20-0245z` to `#review-2--2026-05-20-0245z`. While editing the line, also fix the "Mutation Testing Mutation Testing" stutter per R2-F3-Deferred above.

**Classification:** Deferred — Round 1 finding's fix has not landed in the docs; Round 3 verification required (Dim 4, Dim 11).

---

**Finding 5 — Letter-coded "Surfaces A + C + D" / "/C/D" identifiers still present in DESIGN.md and per-domain index forward-facing prose (Round 1 F6 fix incomplete) (Dim 2, Dim 6)**

<a id="r2-f5-deferred"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** technical-writer

[Round 1 Finding 6](2026-05-20-documentation-reviewer.md#r1-f6) raised four sites using the retired letter-coded "Surface A.0 / B / C / D" identifiers in forward-facing prose (TODO.md:5, TODO.md:41, DESIGN.md:134 (i.e., the verification-architecture line numbered :138 in current), DESIGN.md:17). The Round 2 [CHANGELOG.md](../../CHANGELOG.md):33 entry claims the TODO.md fix landed. Verifying:

- [`TODO.md`](../../TODO.md):5 — fixed: reads `5 Purity Boundary Audit + Mutation Testing hardening`.
- [`TODO.md`](../../TODO.md):41 — fixed: reads `Phase 5 Purity Boundary Audit + Mutation Testing both at closure`.
- [`DESIGN.md`](../../DESIGN.md):17 (Phase 6 strategy declaration) — **still contains** `property-based testing/C/D declared not-applicable` (the slash-separated letter codes survive inside the parenthetical).
- [`DESIGN.md`](../../DESIGN.md):138 (Phase 5 hardening line in § Verification architecture) — **still contains** `per-layer Phase 5 rounds file in vsdd-suite/SOLUTION-ARCHITECT-REVIEW.md (Purity Boundary Audit / A / D) and vsdd-suite/QUALITY-ENGINEER-REVIEW.md (Mutation Testing / C)`. Letter codes `A`, `D`, `C` survive in the forward-facing § Verification architecture spec — Round 1 cited this exact line.
- [`vsdd-suite/QUALITY-ENGINEER-REVIEW.md`](../QUALITY-ENGINEER-REVIEW.md):21 — **still contains** `Surfaces A + C + D explicitly declared deferred / not applicable per the project's Phase 5 strategy in DESIGN.md § Project intent` — current forward-facing prose, not preserved historical narrative.

Implicit-knowledge audit (Dim 2): a cold reader of DESIGN.md sees "(Purity Boundary Audit / A / D)" at line 138 and "property-based testing/C/D" at line 17, and has no information in the project's own docs about what `A`, `C`, or `D` mean. Doc-rot (Dim 6): the migration the Round 1 finding cited was incomplete.

Proposed fix: spell out each letter code with its descriptive name. Suggested edits:
- DESIGN.md:17 — `property-based testing/C/D declared not-applicable` → `property-based testing, Fuzz Testing, and Proof Execution declared not-applicable`
- DESIGN.md:138 — `(Purity Boundary Audit / A / D)` → `(Purity Boundary Audit + property-based testing + Proof Execution)`; `(Mutation Testing / C)` → `(Mutation Testing + Fuzz Testing)`.
- QUALITY-ENGINEER-REVIEW.md:21 — `Surfaces A + C + D explicitly declared deferred / not applicable` → `property-based testing, Fuzz Testing, and Proof Execution explicitly declared deferred / not applicable`.

**Classification:** Deferred — Round 1 finding's fix is incomplete; Round 3 verification required (Dim 2, Dim 6).

---

**Finding 6 — `vsdd-suite/FINDINGS-INDEX.md` still contains both stale cross-reference claims (Round 1 F9 fix incomplete) (Dim 4, Dim 6)**

<a id="r2-f6-deferred"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** technical-writer

[Round 1 Finding 9](2026-05-20-documentation-reviewer.md#r1-f9) raised two stale claims in `vsdd-suite/FINDINGS-INDEX.md`:
- Line 38 (current line 60): "DESIGN.md — Phase 1a contract" — should be "Phase 1a+1b contract" per the v0.6.0 / G-160 rename.
- Line 40 (current line 62): "five others remain as scaffolded stubs" — stale since PR 6 and PR #36 customized all per-domain index files.

The Round 2 CHANGELOG does NOT mention FINDINGS-INDEX.md anywhere in the doc-batch entry. Verifying:

- [`vsdd-suite/FINDINGS-INDEX.md`](../FINDINGS-INDEX.md):60 — **still reads** `[`DESIGN.md`](../DESIGN.md) — Phase 1a contract`.
- [`vsdd-suite/FINDINGS-INDEX.md`](../FINDINGS-INDEX.md):62 — **still reads** `Per-domain index files in this directory — round-level rollup per domain (QUALITY-ENGINEER-REVIEW.md and SOLUTION-ARCHITECT-REVIEW.md customized; five others remain as scaffolded stubs)`.

Documentation rot (Dim 6) — both stale claims survive verbatim. Cross-reference resolution (Dim 4) — the labels mislead a cold reader (Phase 1a contract is now Phase 1a+1b contract; "five others remain as scaffolded stubs" misrepresents the current state where 12 of 12 per-domain index files are customized per Reviews 78 and 80 + PR 6 + PR #36).

Adjacent rot beyond the Round 1 finding: [`vsdd-suite/FINDINGS-INDEX.md`](../FINDINGS-INDEX.md):17 reads `**Open findings only:** \`grep "| Open |"\` (currently: 4 Open ...)` — but the rows in the registry table (lines 27–53) report 5 rows with the value `Open` in the Status column (F-009, F-008, F-018, F-019, F-020) plus several with `Open` in the Status column on PE rows (F-010 through F-017). A cold reader running `grep "| Open |" vsdd-suite/FINDINGS-INDEX.md` against the current state would get materially more than 4 hits. The "currently: 4 Open" note is from the SO Round 1 close state and is now stale against the 11 PE Open rows that landed afterward.

Adjacent rot beyond the Round 1 finding (continued): the registry table contains 27 finding rows (F-001 through F-027) but the CHANGELOG.md v0.11.4 entry's Scope line declares "all 80 findings filed across the 12-domain Round 1 cold-context IAR pass". The 27-row registry against the 80-finding-pass claim is its own internal-consistency defect — either the FINDINGS-INDEX is missing rows for the routed findings, or the CHANGELOG's "80 findings" framing overcounts. A cold reader cross-referencing the two cannot reconcile.

Proposed fix:
- Line 60 — change `Phase 1a contract` to `Phase 1a+1b contract`.
- Line 62 — restate the customization status: `Per-domain index files in this directory — round-level rollup per domain (12 of 12 per-domain index files customized as of PR 6 + PR #36 + Review 80 Documentation Reviewer activation; the only-scaffold-stub state no longer applies)`.
- Line 17 — recompute the "currently: N Open" tally against the actual registry-table contents and update.
- Reconcile the 27-row registry against the CHANGELOG's "80 findings" claim — either backfill the missing rows or correct the CHANGELOG framing. (Out of pure doc-rot scope but flagged here because the next Doc Reviewer round will surface it if untouched.)

**Classification:** Deferred — Round 1 finding's fix has not landed in the docs; Round 3 verification required (Dim 4, Dim 6).

---

**Finding 7 — `README.md` Layer 1 status "complete" contradicts `TODO.md` Layer 1 status "In progress" (new — adjacent defect not raised in Round 1) (Dim 1, Dim 6)**

<a id="r2-f7-deferred"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** technical-writer

Cold-reader pass picks up an internal-consistency defect across README.md and TODO.md that is independently registered in [`vsdd-suite/FINDINGS-INDEX.md`](../FINDINGS-INDEX.md):45 as F-009 (SO R1 F4) — Open — but is not in the [Round 1 Doc Reviewer log](2026-05-20-documentation-reviewer.md), so this is a new Doc Reviewer Round 2 finding rather than a regression-check carryforward.

[`README.md`](../../README.md):9 reads: `Current state: **Layer 1 complete** (add + list).`
[`TODO.md`](../../TODO.md):11 reads: `**Status:** In progress ([Phase 2a](../../vsdd-suite/primers/2a-red-gate.md) → 2b in the reference-implementation session).`

A cold reader landing first on README sees "Layer 1 complete." Following the README's pointer at line 9 to `DESIGN.md` does not resolve the question (DESIGN.md frames Layer 1 against the full-methodology context, not against a "complete vs. in-progress" status). Following the README's pointer at line 11 to `TODO.md` sees the explicit "In progress" — direct contradiction. The reader has no in-doc affordance to decide which source is current.

Cross-reference: [`vsdd-suite/FINDINGS-INDEX.md`](../FINDINGS-INDEX.md):45 F-009 raises this from the Solution Owner lens (deliverable-vs-promise misalignment); this finding registers the same divergence from the cold-reader-of-docs lens (Dim 1 clone-and-follow + Dim 6 doc rot). The SO finding is Open per FINDINGS-INDEX.md; Round 2's doc-batch fix did not close it.

Adjacent defect surfaced at the same time: [`README.md`](../../README.md):58 phase-progression-table Phase 3 row reads `Scaffolded; rounds-in-progress (this is reference-implementation work, not a real merge gate)` — but [`README.md`](../../README.md):59 Phase 4 row reads `Routed 80 findings through Phase 4 → fix cycle → Round 2 verification` — the Phase 4 row claims completion against 80 findings while the Phase 3 row claims rounds are still in progress. A reader cross-referencing the table sees an internal contradiction: how can Phase 4 routing complete 80 findings if Phase 3 rounds are still in progress?

Doc-rot (Dim 6): the README's status claims at line 9 (Layer 1 complete), line 58 (Phase 3 in-progress), and line 59 (Phase 4 completed against 80 findings) are mutually inconsistent and inconsistent with TODO.md's "In progress" status. Clone-and-follow fidelity (Dim 1): a reader following the README literally has no consistent picture of where the project actually is.

Proposed fix: pick a single status framing for Layer 1 and apply it consistently across README, TODO, and the README phase-progression table. The most defensible framing, given the actual state (Round 1 IAR completed and Round 2 fix-cycle landed but not all docs in sync; install-verification gate operator-pending): "Layer 1 implementation complete; Phase 3 IAR Round 1 + Round 2 fix-cycle complete; capstone closure pending operator-executed install-verification per G-155" — and reconcile TODO.md's "In progress" against the same framing (the "In progress" status is the pre-Phase-3-close state and should advance to "Phase 3 closed; capstone-gate pending Phase 6 + install-verification").

**Classification:** Deferred — new finding; Round 3 verification required after the README + TODO reconciliation pass lands (Dim 1, Dim 6).

---

### Dismissed

*(none.)*

---

### Hallucinated

*(none — every Finding above was verified against the current project state with file:line citations and direct cross-file comparison. The sycophancy-guard pre-classification was applied: each Round 1 finding was tested against the question "does the docs-as-the-contract test still fail, or has the fix actually landed?" — every Deferred finding kept passes the test by demonstrating an observable cold-reader divergence that the proposed Round 2 fix did not close. The CHANGELOG entry's *claim* of resolution was held adversarially against the docs' *actual* current state, not deferred to.)*

---

### Summary

**Round 1 disposition (regression check across F1–F13):**

| Round 1 finding | Round 2 status |
|---|---|
| [R1-F1](2026-05-20-documentation-reviewer.md#r1-f1) — README install dir | [Resolved](#r2-f1) |
| [R1-F2](2026-05-20-documentation-reviewer.md#r1-f2) — README test count | [Resolved](#r2-f2) |
| [R1-F3](2026-05-20-documentation-reviewer.md#r1-f3) — relative-depth `../vsdd-suite/...` | **Deferred** — fix incomplete (DESIGN.md:3 remains broken); see [R2-F2-Deferred](#r2-f2-deferred) |
| [R1-F4](2026-05-20-documentation-reviewer.md#r1-f4) — broken `1ab-spec-development.md` references | **Deferred** — fix incomplete (DESIGN.md, SOLUTION-ARCHITECT-REVIEW.md, QUALITY-ENGINEER-REVIEW.md still broken); see [R2-F1-Deferred](#r2-f1-deferred) |
| [R1-F5](2026-05-20-documentation-reviewer.md#r1-f5) — SA per-domain index "QE Review 1" + anchor drift | **Deferred** — fix not applied; see [R2-F4-Deferred](#r2-f4-deferred) |
| [R1-F6](2026-05-20-documentation-reviewer.md#r1-f6) — letter-coded Surface identifiers | **Deferred** — fix incomplete (TODO.md fixed; DESIGN.md and QE per-domain index forward-facing prose still letter-coded); see [R2-F5-Deferred](#r2-f5-deferred) |
| [R1-F7](2026-05-20-documentation-reviewer.md#r1-f7) — search/replace stutters | **Deferred** — fix not applied across six sites; see [R2-F3-Deferred](#r2-f3-deferred) |
| [R1-F8](2026-05-20-documentation-reviewer.md#r1-f8) — TODO.md active-domain count | [Resolved](#r2-f8) |
| [R1-F9](2026-05-20-documentation-reviewer.md#r1-f9) — FINDINGS-INDEX.md stale claims | **Deferred** — fix not applied; see [R2-F6-Deferred](#r2-f6-deferred) |
| [R1-F10](2026-05-20-documentation-reviewer.md#r1-f10) — install-verification.md doubled-path links | [Resolved](#r2-f10) |
| [R1-F11](2026-05-20-documentation-reviewer.md#r1-f11) — `which bm` literal-match expectation | [Resolved](#r2-f11) |
| [R1-F12](2026-05-20-documentation-reviewer.md#r1-f12) — VSDD / IAR / MVR first-use expansion | [Resolved](#r2-f12) |
| [R1-F13](2026-05-20-documentation-reviewer.md#r1-f13) — manual-test single-session assumption | [Resolved](#r2-f13) |

Resolved: 7 (F1, F2, F8, F10, F11, F12, F13). Deferred (fix-incomplete carryforwards): 6 (R2-F1-Deferred through R2-F6-Deferred map to R1-F4, R1-F3, R1-F7, R1-F5, R1-F6, R1-F9 respectively). New findings surfaced in Round 2: 1 (R2-F7-Deferred — README vs. TODO Layer 1 status contradiction; cold-reader-of-docs lens of the same divergence Solution Owner R1 F4 raises).

**Source:** `domain-raised` on every finding. Owner: 13 × `technical-writer`. Validator: 13 × `technical-writer` per the Doc Reviewer ↔ TW adversarial-pair convention ([Review 77](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-77--2026-05-20-1545z), [Review 80](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z)).

**Finding progression (against the [G-131](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-131) continue trigger):** 7 Resolved + 7 new real findings (6 fix-incomplete carryforwards + 1 new adjacent defect) → Round 3 is mandatory once the TW fix pass lands. The pattern in this round — half the Round 1 findings Resolved cleanly and half left as fix-incomplete-but-CHANGELOG-claims-resolved — is exactly the doc-rot-from-incomplete-search-and-replace pattern Round 1 Finding 7 named in a different form. The next fix pass should treat the descriptive-name migration AND the broken-primer-link replacement AND the per-domain index file customization claim as project-wide sweeps with verification (e.g., `grep -rn` searches for the offending strings should return zero forward-facing matches before claiming resolution).

**MVR signal:** **NOT reached.** Per [G-131](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-131) continue trigger discipline, Round 3 is mandatory once the next fix pass lands. The layer cannot reach MVR for the Documentation Reviewer domain until a cold-reader pass produces only Hallucinated findings or no findings — this round produced 7 real findings, six of which are demonstrations that the Round 2 fix-cycle's CHANGELOG-stated resolutions did not fully land in the docs themselves.

**Coordination:** Several findings overlap with [Technical Writer](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) round 2 concerns by adversarial-pair construction — R2-F3-Deferred (stutters) and R2-F5-Deferred (lettering) are cold-reader pairs to [TW Dim 12](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md). R2-F1-Deferred / R2-F2-Deferred / R2-F4-Deferred / R2-F6-Deferred (broken links, broken anchors, FINDINGS-INDEX rot) are cold-reader pairs to [TW Dim 13](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) inline-reference clickthrough validation. R2-F7-Deferred (README vs. TODO status contradiction) overlaps with the existing Solution Owner R1 F4 finding (F-009 in FINDINGS-INDEX) — the cold-reader lens registers it as a doc-rot defect (Dim 6) while SO registers it as a deliverable-vs-promise misalignment; both views call for the same reconciliation.

**Note on the CHANGELOG-vs-docs divergence pattern:** The CHANGELOG v0.11.4 entry repeatedly claims fixes have landed when grep against the docs shows they have not. This is its own doc-rot signal — a CHANGELOG that overstates the scope of changes is a forward-facing artifact whose own claims have to be verified against the codebase. The pattern recurs in:

- CHANGELOG line 32 (relative-depth fix) — claims README only; DESIGN.md:3 remains broken.
- CHANGELOG line 33 (lettering migration) — claims TODO.md only; DESIGN.md and per-domain index files retain letter codes.
- CHANGELOG line 33 (stutter cleanup) — not mentioned at all; six sites remain.
- CHANGELOG line 36 (primer-rename fix) — claims PROCESS.md only; three additional sites remain broken.

The next fix pass should either complete the work the CHANGELOG claims has happened, OR walk back the CHANGELOG claims to match the docs' actual state. Either is acceptable; the current divergence between CHANGELOG-claim and docs-state is not.

---

## Review 3 — 2026-05-20 22:00Z

**Layer:** 1
**Tested against:** post-Round-2-fix-cycle state of `bookmark-cli-manual` ([CHANGELOG.md](../../CHANGELOG.md) v0.11.4; current working tree as of 2026-05-20 23:00Z).
**Round:** 3
**Active domain set:** 11 role + 1 meta = 12 (per [DESIGN.md § Project intent](../../DESIGN.md)).
**Scope:** Phase 3 [Adversarial Refinement](../../../../vsdd-suite/primers/3-review-session.md) Round 3 verification of the six Documentation Reviewer Round 2 Deferred-fix-incomplete findings (R2-F1 through R2-F6) plus the one new Round 2 finding (R2-F7 README/TODO contradiction). Independent adversarial cold-reader pass also looks for further adjacent defects per the [G-131](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-131) continue-trigger framing ("the Round N+1 cold pass verifies the fix held and looks for adjacent defects the fix may have created"). Read in cold-reader order with DESIGN.md last per the [Documentation Reviewer](../../../../vsdd-suite/domains/role/DOCUMENTATION-REVIEWER-REVIEW.md) discipline. Per-domain index files in [`vsdd-suite/`](..) consulted only as cited targets of links in the user-facing artifacts.
**Lens:** Adversarial cold-reader pair to [Technical Writer](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md). The 11 Documentation Reviewer dimensions re-applied to the post-fix state with the Round 2 carryforwards as the primary regression-check target: clone-and-follow fidelity (Dim 1), implicit-knowledge audit (Dim 2 — reader-built glossary), forward-reference safety (Dim 3), cross-reference resolution (Dim 4 — every `[text](path)` re-opened), audience-fit calibration (Dim 5), documentation rot (Dim 6 — every claim re-verified against current code/spec/process), recovery-from-confusion (Dim 7), manual-test plan executability (Dim 8), onboarding sequencing (Dim 9), manual-test file structure consistency (Dim 10 — Review 74 convention), inline-reference clickthrough validation (Dim 11 — every markdown link re-opened).
**Source:** `domain-raised` — every finding below was elicited by applying the 11 Documentation Reviewer dimensions to the current user-facing artifacts in cold context. No director-raised observations interrupted this round.
**Regression check:** Each Round 2 carryforward (R2-F1-Deferred through R2-F6-Deferred) plus the new R2-F7-Deferred re-verified against the current state by re-locating the cited file:line and inspecting against the proposed fix in the Round 2 entry. The fix-cycle's [Review 82](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z) Round 2 closure landed the primary fixes for some sites and not others; this round verifies each cited site individually.
**Validator:** `technical-writer` on each Resolved/Deferred finding per the [Doc Reviewer ↔ Technical Writer](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) adversarial-pair convention ([Review 80](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z)).

**Session note:** Cold-context AI session — did not author the Round 2 fix-cycle, the Round 2 Documentation Reviewer entry, or any of the carryforwards' proposed fixes. The primer's sycophancy guard applied: each Round 2 carryforward was re-tested by re-reading the cited file:line in the current state. The Round 2 entry's *claim* of fix-incomplete was treated adversarially — if the fix has in fact landed since Round 2, the carryforward must Resolve; if it has not, the Deferred classification persists and Round 4 becomes mandatory.

**MVR signal:** **NOT reached.** Five of the six Round 2 Deferred carryforwards remain unresolved in the current state (Doc Reviewer R3 Findings 1–5 below). Per the [G-131](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-131) continue trigger, Round 4 is mandatory once the next fix pass lands. The carryforward pattern across two rounds is a structural signal: the fix-cycle has the search/replace surface mapped (the Round 2 log enumerates each cited site) but the sweep has not been executed; a `grep -rn` discipline before claiming closure is the methodology fix.

---

### Resolved

**Finding 1 — `DESIGN.md` line 3 broken `1ab-spec-development.md` primer link (R2 F1 Deferred site for DESIGN.md) (Dim 4, Dim 6, Dim 11)**

<a id="r3-f1"></a>

**Owner:** technical-writer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** technical-writer

[R2 Finding 1 Deferred](2026-05-20-documentation-reviewer.md#r2-f1-deferred) raised that [`../../DESIGN.md`](../../DESIGN.md):3 still linked at `[Phase 1a+1b](../../vsdd-suite/primers/1ab-spec-development.md)`. Verifying the current state: [`../../DESIGN.md`](../../DESIGN.md):3 now reads `[Phase 1a+1b](../../vsdd-suite/primers/1ab-spec-crystallization.md) contract (per v0.7.2 conventions; the file was originally authored under the prior single-step "Phase 1a" naming + the prior primer filename ``1ab-spec-development.md`` — both retired by the suite. The current canonical primer is [``../../vsdd-suite/primers/1ab-spec-crystallization.md``](../../vsdd-suite/primers/1ab-spec-crystallization.md); historical narrative preserved per [G-89](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-89) forward-only policy).`

The link target now points at the existing `1ab-spec-crystallization.md` primer; the prior `1ab-spec-development.md` filename appears only as backtick-quoted prose narrative inside a parenthetical that explicitly cites [G-89](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-89) forward-only narrative-preservation. The link is now resolvable; the prose narrative is preserved appropriately. Round 2's specific complaint against DESIGN.md:3 is closed.

The same line also had the wrong relative-depth `../vsdd-suite/...` paths from [R2 Finding 2 Deferred](#r3-f2-deferred-below) — those are also resolved here: [`../../DESIGN.md`](../../DESIGN.md):3 now uses `../../vsdd-suite/primers/1ab-spec-crystallization.md` and `../../vsdd-suite/README.md` (both correct two-dot depth).

**Resolution:** [`../../DESIGN.md`](../../DESIGN.md):3 corrected for both the primer-rename and the relative-depth defects. Cold-reader clickthrough test passes against the current portfolio layout. Round 2 Finding 1 Deferred (DESIGN.md:3 site) and Round 2 Finding 2 Deferred (DESIGN.md:3 site) both closed; the remaining sites for R2-F1 (SA-REVIEW.md:21, QE-REVIEW.md:21) are re-raised as [R3 Doc Reviewer Finding 1 Deferred](#r3-f1-deferred) below — the DESIGN.md fix landed but the per-domain index files retain the broken link (Dim 4, Dim 6, Dim 11).

---

### Deferred

**Finding 1 — `SOLUTION-ARCHITECT-REVIEW.md` and `QUALITY-ENGINEER-REVIEW.md` still link to broken `1ab-spec-development.md` (R2 F1 Deferred residue) (Dim 4, Dim 6, Dim 11)**

<a id="r3-f1-deferred"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** technical-writer

[R2 Finding 1 Deferred](2026-05-20-documentation-reviewer.md#r2-f1-deferred) enumerated three remaining broken-link sites after the Round 2 fix-cycle (DESIGN.md:3, SOLUTION-ARCHITECT-REVIEW.md:21, QUALITY-ENGINEER-REVIEW.md:21). DESIGN.md:3 is now Resolved (see [R3 DR Resolved Finding 1](#r3-f1)). Verifying the remaining two:

- [`../SOLUTION-ARCHITECT-REVIEW.md`](../SOLUTION-ARCHITECT-REVIEW.md):21 — **still broken**: reads `Routed via [Phase 4](../../../vsdd-suite/primers/4-feedback-integration.md) to [Phase 1a+1b](../../../vsdd-suite/primers/1ab-spec-development.md)`. The target `vsdd-suite/primers/1ab-spec-development.md` does not exist.
- [`../QUALITY-ENGINEER-REVIEW.md`](../QUALITY-ENGINEER-REVIEW.md):21 — **still broken**: reads `routed via [Phase 4](../../../vsdd-suite/primers/4-feedback-integration.md) to [Phase 1a+1b](../../../vsdd-suite/primers/1ab-spec-development.md)`. Same broken target.

Clickthrough validation (Dim 11): both links 404 against the current `vsdd-suite/primers/` directory. Cross-reference resolution (Dim 4): the cited paths do not resolve. Documentation rot (Dim 6): the fix-cycle has had two opportunities to close these sites (the Round 2 fix-cycle batch and the implicit post-Round-2-DR-log surface) and has closed neither. This is the same fix-incomplete pattern Round 2 flagged, persisted across one further round.

Proposed fix: search-and-replace `1ab-spec-development.md` → `1ab-spec-crystallization.md` in [`../SOLUTION-ARCHITECT-REVIEW.md`](../SOLUTION-ARCHITECT-REVIEW.md) and [`../QUALITY-ENGINEER-REVIEW.md`](../QUALITY-ENGINEER-REVIEW.md). Discipline: run `grep -rn "1ab-spec-development" vsdd-suite-reference-examples/bookmark-cli-manual/` and verify zero forward-facing matches (preserved prose narrative inside DESIGN.md:3's G-89 carve-out parenthetical is acceptable; markdown link targets in per-domain index Reviews-table summaries are not).

**Classification:** Deferred — Round 2 carryforward fix is incomplete at two remaining sites; Round 4 verification required (Dim 4, Dim 6, Dim 11).

---

**Finding 2 — Stutters survive across DESIGN.md, SA-REVIEW.md, QE-REVIEW.md, PROCESS.md (R2 F3 Deferred — fix still not applied) (Dim 6)**

<a id="r3-f2"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** technical-writer

[R2 Finding 3 Deferred](2026-05-20-documentation-reviewer.md#r2-f3-deferred) enumerated six sites with descriptive-name stutters left over from the Round 1 letter-coded-Surface migration. Round 2 prescribed a targeted search-and-replace pass. Verifying the current state:

- [`../../DESIGN.md`](../../DESIGN.md):15 — **still contains** `Mutation Testing (Mutation Testing via cargo-mutants)` and `property-based testing (property-based testing via proptest)` (two stutters on one line).
- [`../../DESIGN.md`](../../DESIGN.md):17 — **still contains** `Purity Boundary Audit Purity Boundary Audit + Mutation Testing Mutation Testing closure` (two stutters in one phrase).
- [`../SOLUTION-ARCHITECT-REVIEW.md`](../SOLUTION-ARCHITECT-REVIEW.md):21 — **still contains** `Phase 5 Purity Boundary Audit Purity Boundary Audit` and `Companion QE round (Mutation Testing Mutation Testing)`.
- [`../QUALITY-ENGINEER-REVIEW.md`](../QUALITY-ENGINEER-REVIEW.md):21 — **still contains** `Purity Boundary Audit purity-boundary audit` (a variant — lowercase-second-half stutter; the canonical-preamble-tag shape is unchanged but the in-prose summary mixes the descriptive name with a lower-case half).
- [`../../PROCESS.md`](../../PROCESS.md):23 — **still contains** `the Phase 5 Purity Boundary Audit Purity Boundary Audit`.
- [`../../PROCESS.md`](../../PROCESS.md):39 — **still contains** `The Phase 5 Purity Boundary Audit Purity Boundary Audit produced the cross-source divergence finding`.

Documentation rot (Dim 6) — six sites enumerated in Round 2, all still present in Round 3. The Round 2 [`../../CHANGELOG.md`](../../CHANGELOG.md):33 entry's "retired letter-coded 'Surface A.0 / B' verbiage replaced with descriptive 'Purity Boundary Audit + Mutation Testing' Title-Case names" claim continues to apply to TODO.md only; the broader sweep has not landed.

Proposed fix (re-asserted from Round 2):

- `Mutation Testing Mutation Testing` → `Mutation Testing`
- `Purity Boundary Audit Purity Boundary Audit` → `Purity Boundary Audit`
- `Purity Boundary Audit purity-boundary audit` → `Purity Boundary Audit`
- `property-based testing (property-based testing via proptest)` → `property-based testing (via proptest)`
- `Mutation Testing (Mutation Testing via cargo-mutants)` → `Mutation Testing (via cargo-mutants)`

Apply across DESIGN.md, SOLUTION-ARCHITECT-REVIEW.md, QUALITY-ENGINEER-REVIEW.md, and PROCESS.md (forward-facing prose only; CHANGELOG historical narrative preserved per [G-89](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-89)).

**Classification:** Deferred — Round 2 carryforward fix has not landed across the cited sites; Round 4 verification required (Dim 6).

---

**Finding 3 — `SOLUTION-ARCHITECT-REVIEW.md` Reviews-table companion-link still cites "QE Review 1" with broken anchor `#review-1--2026-05-20-0245z` (R2 F4 Deferred — fix still not applied) (Dim 4, Dim 11)**

<a id="r3-f3"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** technical-writer

[R2 Finding 4 Deferred](2026-05-20-documentation-reviewer.md#r2-f4-deferred) raised that [`../SOLUTION-ARCHITECT-REVIEW.md`](../SOLUTION-ARCHITECT-REVIEW.md):21's companion-QE-round link mis-cites the round number and the anchor does not resolve. Verifying the current state: the line reads `Companion QE round (Mutation Testing Mutation Testing) at [QE Review 1](2026-05-20-quality-engineer.md#review-1--2026-05-20-0245z).` — identical to the Round 1 cite. Both Round 1 Finding 5 defects persist into Round 3:

1. Link text "QE Review 1" — [`2026-05-20-quality-engineer.md`](2026-05-20-quality-engineer.md):9 begins `## Review 2 — 2026-05-20 02:45Z`; there is no Review 1 heading in that file. (Review 1 lives in [`2026-05-17-quality-engineer.md`](2026-05-17-quality-engineer.md).) The link text mis-cites the round.
2. Anchor `#review-1--2026-05-20-0245z` — the actual anchor on the target file is `#review-2--2026-05-20-0245z`. The link lands on the file but at no heading.

Clickthrough validation (Dim 11): the anchor does not resolve. Cross-reference resolution (Dim 4): the cited round + anchor do not match the target's actual structure. This finding now persists across three rounds — Round 1 (R1-F5), Round 2 (R2-F4-Deferred), Round 3 (this finding).

Cross-finding overlap: the same line also carries the `Mutation Testing Mutation Testing` stutter flagged in [R3 DR Finding 2](#r3-f2) above. A single edit on this line can address both.

Proposed fix: change link text from `QE Review 1` to `QE Review 2`; change anchor from `#review-1--2026-05-20-0245z` to `#review-2--2026-05-20-0245z`. While editing the line, also fix the `Mutation Testing Mutation Testing` stutter per [R3 DR Finding 2](#r3-f2).

**Classification:** Deferred — Round 2 carryforward fix has not landed; Round 4 verification required (Dim 4, Dim 11).

---

**Finding 4 — Letter-coded "Surfaces A + C + D" identifiers still present in DESIGN.md and QE-REVIEW.md forward-facing prose (R2 F5 Deferred — fix still not applied at DESIGN.md and QE-REVIEW.md sites) (Dim 2, Dim 6)**

<a id="r3-f4"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** technical-writer

[R2 Finding 5 Deferred](2026-05-20-documentation-reviewer.md#r2-f5-deferred) raised three remaining letter-coded forward-facing prose sites after the Round 2 fix-cycle (DESIGN.md:17 Phase 6 strategy `property-based testing/C/D`; DESIGN.md:138 Verification architecture `(Purity Boundary Audit / A / D)` and `(Mutation Testing / C)`; QUALITY-ENGINEER-REVIEW.md:21 `Surfaces A + C + D explicitly declared deferred / not applicable`). Verifying the current state:

- [`../../DESIGN.md`](../../DESIGN.md):17 — **still contains** `property-based testing/C/D declared not-applicable` (the slash-separated letter codes survive inside the Phase 6 strategy parenthetical).
- [`../../DESIGN.md`](../../DESIGN.md):138 — **still contains** `per-layer Phase 5 rounds file in vsdd-suite/SOLUTION-ARCHITECT-REVIEW.md (Purity Boundary Audit / A / D) and vsdd-suite/QUALITY-ENGINEER-REVIEW.md (Mutation Testing / C)`. Letter codes `A`, `D`, `C` survive in the forward-facing § Verification architecture spec.
- [`../QUALITY-ENGINEER-REVIEW.md`](../QUALITY-ENGINEER-REVIEW.md):21 — **still contains** `Surfaces A + C + D explicitly declared deferred / not applicable per the project's Phase 5 strategy in DESIGN.md § Project intent`.

Implicit-knowledge audit (Dim 2): a cold reader of DESIGN.md sees `(Purity Boundary Audit / A / D)` at line 138 and `property-based testing/C/D` at line 17, and has no information in the project's own docs about what `A`, `C`, or `D` mean. The descriptive name (`Purity Boundary Audit`, `Mutation Testing`, `property-based testing`) carries meaning at point of use; the letter does not. Doc-rot (Dim 6): the migration the Round 1 / Round 2 findings cited remains incomplete at these three sites.

Proposed fix (re-asserted from Round 2):

- DESIGN.md:17 — `property-based testing/C/D declared not-applicable` → `property-based testing, Fuzz Testing, and Proof Execution declared not-applicable`
- DESIGN.md:138 — `(Purity Boundary Audit / A / D)` → `(Purity Boundary Audit + property-based testing + Proof Execution)`; `(Mutation Testing / C)` → `(Mutation Testing + Fuzz Testing)`
- QUALITY-ENGINEER-REVIEW.md:21 — `Surfaces A + C + D explicitly declared deferred / not applicable` → `property-based testing, Fuzz Testing, and Proof Execution explicitly declared deferred / not applicable`

**Classification:** Deferred — Round 2 carryforward fix is incomplete across DESIGN.md and QE-REVIEW.md; Round 4 verification required (Dim 2, Dim 6).

---

**Finding 5 — `FINDINGS-INDEX.md` cross-reference + open-count + customization-status claims still stale (R2 F6 Deferred — fix still not applied) (Dim 4, Dim 6)**

<a id="r3-f5"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** technical-writer

[R2 Finding 6 Deferred](2026-05-20-documentation-reviewer.md#r2-f6-deferred) raised four stale claims in [`../FINDINGS-INDEX.md`](../FINDINGS-INDEX.md): the "Phase 1a contract" cross-reference label (line 60 in R2 enumeration), the "five others remain as scaffolded stubs" customization-status claim (line 62), the "currently: 4 Open" tally (line 17), and the 27-row registry vs. CHANGELOG's "80 findings" framing inconsistency. Verifying the current state:

- [`../FINDINGS-INDEX.md`](../FINDINGS-INDEX.md):60 — **still reads** `[``DESIGN.md``](../DESIGN.md) — Phase 1a contract`. The current DESIGN.md H1 is `Phase 1a+1b contract` per the v0.6.0 / G-160 rename documented in [`../../CHANGELOG.md`](../../CHANGELOG.md):126.
- [`../FINDINGS-INDEX.md`](../FINDINGS-INDEX.md):62 — **still reads** `Per-domain index files in this directory — round-level rollup per domain (QUALITY-ENGINEER-REVIEW.md and SOLUTION-ARCHITECT-REVIEW.md customized; five others remain as scaffolded stubs)`. Per [`../../CHANGELOG.md`](../../CHANGELOG.md):76-77 PR 6 customized 4 newly-capstone-activated extended domains plus 5 pre-existing scaffolded stubs; per [`../../CHANGELOG.md`](../../CHANGELOG.md):54 PR #36 added the customized Documentation Reviewer index. The current state is 12 of 12 per-domain index files customized; "five others remain as scaffolded stubs" is stale by two PRs.
- [`../FINDINGS-INDEX.md`](../FINDINGS-INDEX.md):17 — **still reads** `**Open findings only:** ``grep "| Open |"`` (currently: 4 Open — SO Round 1 (2026-05-20 19:30Z) filed F-006/F-007 Backlogged and F-008/F-009 Open per the deliverable-vs-promise + documentation-defect Findings; resolution paths tracked in the per-Finding ``Blocked by:`` lifecycle fields)`. The registry table rows at lines 27–53 now show many more `| Open |` rows than 4 (F-008, F-009, F-010 through F-020 — i.e., the 11 PE Open rows that landed after the SO Round 1 close, plus the 2 SO Open rows = ~13 Open). The "currently: 4 Open" tally is materially understated.
- The 27-row registry vs. CHANGELOG's "80 findings" framing — the registry table at lines 27–53 still contains 27 rows (F-001 through F-027); CHANGELOG v0.11.4 § Scope still declares "all 80 findings filed across the 12-domain Round 1 cold-context IAR pass". The internal-consistency defect persists; a cold reader cross-referencing the two cannot reconcile.

Documentation rot (Dim 6): every claim is wrong against the current state, two rounds after the defects were first surfaced. Cross-reference resolution (Dim 4): the labels mislead a cold reader about what they will find when they open the cited artifacts.

Proposed fix (re-asserted from Round 2):

- Line 60 — change `Phase 1a contract` to `Phase 1a+1b contract`.
- Line 62 — restate the customization status: `Per-domain index files in this directory — round-level rollup per domain (12 of 12 per-domain index files customized as of PR 6 + PR #36 + Review 80 Documentation Reviewer activation; the only-scaffold-stub state no longer applies)`.
- Line 17 — recompute the "currently: N Open" tally against the actual registry-table contents.
- Reconcile the 27-row registry against the CHANGELOG's "80 findings" claim — either backfill the missing rows (likely the FINDINGS-INDEX has not yet been updated against the 12-domain Round 1 sweep that the CHANGELOG claims as the "all 80 findings" scope) or correct the CHANGELOG framing.

**Classification:** Deferred — Round 2 carryforward fix has not landed; Round 4 verification required (Dim 4, Dim 6).

---

### Dismissed

*(none.)*

---

### Hallucinated

*(none — every Finding above was verified against the current project state with file:line citations. The sycophancy-guard pre-classification was applied: each Round 2 carryforward was re-tested by re-reading the cited file:line in the current state; every Deferred finding kept demonstrates an observable cold-reader divergence that the proposed Round 2 fix did not close. The DESIGN.md:3 / R2-F1 + R2-F2 closure was specifically attributable to fixes in the current state, not pre-existing; the cold reader confirmed both the primer-rename target and the relative-depth target now resolve correctly.)*

---

### Summary

**Round 2 disposition (regression check across R2-F1-Deferred through R2-F7-Deferred):**

| Round 2 finding | Round 3 status |
|---|---|
| [R2-F1-Deferred](2026-05-20-documentation-reviewer.md#r2-f1-deferred) — DESIGN.md:3 + SA-REVIEW.md:21 + QE-REVIEW.md:21 broken `1ab-spec-development.md` | **Partially resolved** — DESIGN.md:3 closed; SA + QE per-domain index sites Deferred forward as [R3 DR Finding 1 Deferred](#r3-f1-deferred) |
| [R2-F2-Deferred](2026-05-20-documentation-reviewer.md#r2-f2-deferred) — DESIGN.md:3 wrong relative-depth `../vsdd-suite/...` | [Resolved](#r3-f1) (closed alongside R2-F1 DESIGN.md:3 site) |
| [R2-F3-Deferred](2026-05-20-documentation-reviewer.md#r2-f3-deferred) — six-site stutter cleanup | **Deferred** — fix still not applied; see [R3 DR Finding 2 Deferred](#r3-f2) |
| [R2-F4-Deferred](2026-05-20-documentation-reviewer.md#r2-f4-deferred) — SA-REVIEW.md:21 `QE Review 1` + broken anchor | **Deferred** — fix still not applied; see [R3 DR Finding 3 Deferred](#r3-f3) |
| [R2-F5-Deferred](2026-05-20-documentation-reviewer.md#r2-f5-deferred) — DESIGN.md + QE-REVIEW.md letter-coded Surfaces | **Deferred** — fix still not applied; see [R3 DR Finding 4 Deferred](#r3-f4) |
| [R2-F6-Deferred](2026-05-20-documentation-reviewer.md#r2-f6-deferred) — FINDINGS-INDEX.md stale claims | **Deferred** — fix still not applied; see [R3 DR Finding 5 Deferred](#r3-f5) |
| [R2-F7-Deferred](2026-05-20-documentation-reviewer.md#r2-f7-deferred) — README/TODO Layer-1 status contradiction | Resolved (inline by the post-Round-2 doc-cleanup pass — TODO Status line now reads "Layer 1 code-complete; [Phase 3](../../../../vsdd-suite/primers/3-review-session.md) IAR Round 1 + Round 2 cold-session cycles closed in [Review 82](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z). [Phase 6](../../../../vsdd-suite/primers/6-convergence.md) four-dimensional convergence DEFERRED pending Round 3 fix cycles for the 8 non-MVR domains + operator-runs-install-verification (Platform Engineer Dim 38)." consistent with README:9's "Layer 1 complete" framing.) |

Resolved: 2 (R2-F2 fully, R2-F7 inline; R2-F1's DESIGN.md:3 site partially). Deferred (fix-incomplete carryforwards): 5 (R3 DR Findings 1–5). No new adjacent defects surfaced beyond the carryforwards in this round — the fix-cycle has not introduced new defects, it has simply not closed the previously-enumerated ones.

**Source:** `domain-raised` on every finding. Owner: 6 × `technical-writer`. Validator: 6 × `technical-writer` per the [Doc Reviewer ↔ Technical Writer](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) adversarial-pair convention ([Review 80](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z)).

**Finding progression (against the [G-131](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-131) continue trigger):** 1 Resolved (R2-F1 DESIGN.md:3 + R2-F2 collapsed into one resolution) + 5 carryforward Deferred + 0 new adjacent defects → Round 4 is mandatory once the next fix pass lands. The pattern across three rounds — Round 1 surfaces, Round 2 partial fix, Round 3 carryforward verification with most sites still unresolved — is itself a structural signal: the fix-cycle has the search/replace surface mapped (every Round 2 finding enumerates the exact file:line targets) but the sweep across all enumerated sites has not been executed. The methodology fix is to treat the descriptive-name migration, broken-link replacement, FINDINGS-INDEX cross-reference cleanup, and per-domain index repair as project-wide sweeps with `grep -rn` verification before claiming closure.

**MVR signal:** **NOT reached.** Per [G-131](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-131) continue trigger discipline, Round 4 is mandatory once the next fix pass lands. The layer cannot reach MVR for the Documentation Reviewer domain until a cold-reader pass produces only Hallucinated findings or no findings — this round produced 5 carryforward real findings + 1 resolution, none Hallucinated.

**Coordination:** Several findings overlap with [Technical Writer](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) Round 3 concerns by adversarial-pair construction — R3-DR-F2 (stutters) and R3-DR-F4 (lettering) are cold-reader pairs to [TW Dim 12](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md). R3-DR-F1 / R3-DR-F3 / R3-DR-F5 (broken links, broken anchors, FINDINGS-INDEX rot) are cold-reader pairs to [TW Dim 13](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) inline-reference clickthrough validation. The TW Round 3 in the parallel cluster should see substantially the same surface and is expected to reach a similar disposition.

---
