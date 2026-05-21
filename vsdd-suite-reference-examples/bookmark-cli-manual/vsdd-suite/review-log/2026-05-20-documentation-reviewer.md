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
