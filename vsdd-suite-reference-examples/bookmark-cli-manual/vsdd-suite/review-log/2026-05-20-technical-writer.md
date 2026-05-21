# Technical Writer Review — 2026-05-20

[Index](../TECHNICAL-WRITER-REVIEW.md)

---

## Review 1 — 2026-05-20 19:30Z

**Scope:** Layer 1 first-pass [Technical Writer](../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) review (cold-context). Read [`DESIGN.md`](../../DESIGN.md), [`README.md`](../../README.md), [`TODO.md`](../../TODO.md), [`CHANGELOG.md`](../../CHANGELOG.md), [`PROCESS.md`](../../PROCESS.md), [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md), [`manual-tests/install-verification.md`](../../manual-tests/install-verification.md), [`src/main.rs`](../../src/main.rs) (doc comments), [`src/lib.rs`](../../src/lib.rs) (doc comments + public-API rustdoc), [`Cargo.toml`](../../Cargo.toml), and the existing-round schema at [QE Review 2](2026-05-20-quality-engineer.md#review-2--2026-05-20-0245z) + [SA Review 1](2026-05-20-solution-architect.md#review-1--2026-05-20-0245z). Capstone-intent activation per [`DESIGN.md` § Project intent](../../DESIGN.md#project-intent); supplements loaded per the language + interface declaration: [`vsdd-suite/supplements/rust.md`](../../../vsdd-suite/supplements/rust.md) § Technical Writer + [`vsdd-suite/supplements/cli.md`](../../../vsdd-suite/supplements/cli.md) § Technical Writer + [`vsdd-suite/supplements/markdown.md`](../../../vsdd-suite/supplements/markdown.md) § Technical Writer (every project artifact reviewed here is markdown — the markdown supplement is load-bearing).

**Session note:** Cold context. No prior involvement in authoring `bookmark-cli-manual`. The Phase 3 IAR primer at [`vsdd-suite/primers/3-review-session.md`](../../../vsdd-suite/primers/3-review-session.md) was loaded before the domain prompt; adversarial posture established before reading any project artifact; [`DESIGN.md`](../../DESIGN.md) read last in the input order per the operator's explicit directive. Sycophancy-compensation: every "documentation X describes Y" claim verified against the current implementation/state, not against the documentation's framing of itself. The five `PROT_NN` byte-corruption artifacts (Finding 1 below) surfaced because `grep` without `-a` silently dropped lines containing embedded NUL bytes — that null-result-as-evidence is itself an audit-trail data-integrity concern documented in the finding.

**Source:** `domain-raised` — the cold adversary applying the [TW](../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) dimensions (plus the [Rust supplement § TW rustdoc-coverage](../../../vsdd-suite/supplements/rust.md) check per [G-137](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-137) + the [markdown supplement § TW](../../../vsdd-suite/supplements/markdown.md) anchor-link-convention check per [Review 79](../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-79--2026-05-20-1730z) Finding 3 + the [CLI supplement](../../../vsdd-suite/supplements/cli.md) `--help` clone-and-follow lens) surfaced every finding below by direct artifact inspection.

**Regression check:** [QE Review 1 — 2026-05-17 03:25Z](2026-05-17-quality-engineer.md#review-1--2026-05-17-0325z) summary line claimed "8 tests pass" against the 4 ACs + 4 lib unit tests at that point in time. Phase 5 added a fifth lib test (`save_creates_parent_directory_for_nested_path`) per [QE Review 2 Finding 1](2026-05-20-quality-engineer.md#review-2--2026-05-20-0245z). The README still asserts the pre-Phase-5 count — this is a stale-documentation regression (Finding 2 below), and is exactly the failure mode TW Dim 2 names.

---

### Resolved

*(none — this is Round 1; findings open for owner acceptance.)*

---

### Deferred

**Finding 1 — Document corruption: embedded NUL-byte sentinel artifacts in `DESIGN.md` H3 headings and `manual-tests/layer-1.md` Step headings (Dim 2, Dim 6, Dim 12)**

**Owner:** solution-owner
**Status:** raised
**Blocked by:** *(none)*

[`DESIGN.md`](../../DESIGN.md) § Behavioral contracts contains two H3 headings whose section title is rendered as opaque token `PROT_37` (line 55) and `PROT_41` (line 63). These are the section titles for the `bm add` and `bm list` behavioral contracts — the load-bearing section names for the project's verifiable behavior catalog. [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md) contains three H2 step headings with the same shape: `## Step 1 — Happy path:  PROT_30  captures a bookmark` (line 34), `## Step 3 —  PROT_40  orders newest-first` (line 93), `## Step 4 — Empty-state:  PROT_46  against an absent store` (line 120).

The token shape is `\x00PROT_NN\x00` — each `PROT_NN` string is wrapped in literal NUL bytes (`0x00`). Evidence shape (per the primer's confidentiality-aware citation discipline — I cite the byte structure, not whatever the sentinels were meant to redact-or-replace):

- `DESIGN.md:55` raw bytes (13 total): `### \x00PROT_37\x00`
- `DESIGN.md:63` raw bytes (13 total): `### \x00PROT_41\x00`
- `manual-tests/layer-1.md:34` raw bytes: `## Step 1 — Happy path: \x00PROT_30\x00 captures a bookmark`
- `manual-tests/layer-1.md:93` raw bytes: `## Step 3 — \x00PROT_40\x00 orders newest-first`
- `manual-tests/layer-1.md:120` raw bytes: `## Step 4 — Empty-state: \x00PROT_46\x00 against an absent store`

Four failure modes simultaneously:

1. **Audit-trail integrity ([Dim 2](../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md))** — NUL bytes inside `.md` text content is a corruption signal. `grep` treats files with NUL bytes as binary by default and silently drops matching lines from search results unless `-a` is passed (verified during this review — initial `grep "PROT_"` against the project's `.md` tree returned zero results; only the `-a`-flag retry surfaced the matches). Audit-trail integrity is compromised when the project's own search tooling can't see the affected lines. Editors, [GitHub](https://github.com/)'s renderer, and downstream markdown linters will render the NUL bytes inconsistently (some swallow silently, some render as `^@`, some as the replacement character ).
2. **Headings carry no meaning to the reader ([Dim 12](../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) — lookup cost; [Review 78](../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-78--2026-05-20-1630z) Finding 4 retired exactly this pattern)** — `PROT_37` and `PROT_41` are opaque letter+number codes for what the prose makes obvious should be `bm add` and `bm list`. A capstone-intent reference example whose H3 headings in the behavioral-contract catalog name themselves with placeholder codes is failing the same anti-pattern the suite-side Review 78 closed (Surface A / Surface B → "property-based testing" / "mutation testing"). The fix here is not to define what `PROT_37` means in a lookup table; it is to replace the codes with descriptive names (`### bm add` / `### bm list` / `## Step 1 — Happy path: bm add captures a bookmark` / `## Step 3 — bm list orders newest-first` / `## Step 4 — Empty-state: bm list against an absent store`).
3. **Anchor links are unusable ([Dim 13](../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) — inline-reference navigability)** — [GitHub](https://github.com/)'s anchor-slug derivation lowercases + hyphenates the heading text. A heading containing NUL bytes either produces a degraded slug (e.g., `#prot_37`) or no usable slug at all depending on renderer behavior. No internal anchor link can target these sections. The suite's own [§ Anchor-link convention](../../../vsdd-suite/suite-development/suite-development.md#anchor-link-convention-for-cross-references-review-79-finding-3) relies on slug stability — these headings break the convention's contract at the heading-slug layer.
4. **Behavioral-contract identity is load-bearing for cross-doc citation** — [`TODO.md`](../../TODO.md) AC 1–4, [`tests/bookmarks.rs`](../../tests/bookmarks.rs) test names, and [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md) steps all reference the `bm add` and `bm list` contracts by command name. The DESIGN.md H3 headings should carry the same names. A reader following a cross-reference from [TODO.md AC 1](../../TODO.md) ("`bm add <url>` creates a bookmark record...") to DESIGN.md § Behavioral contracts § `bm add` lands on `### PROT_37` instead — the cross-reference is broken at the destination.

**Suspected origin:** this looks like an in-progress anonymization or templating pass that injected sentinel-bracketed placeholders meant to be replaced by a later substitution step, but committed before the substitution completed. The NUL-byte wrapping is consistent with tooling that scans for `\x00...\x00` sentinel pairs as substitution markers. Whatever the origin, the artifacts are visible to every reader of the rendered file.

**Proposed change to [`DESIGN.md`](../../DESIGN.md):** rename `### PROT_37` (line 55) → `### bm add` and `### PROT_41` (line 63) → `### bm list`, removing the NUL-byte wrappers. The H3 names should match the command surface enumerated in DESIGN.md line 87–90 (`bm add <url>` / `bm list` / `bm --help` / `bm --version`).

**Proposed change to [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md):** rename Step headings to remove NUL bytes and replace each `PROT_NN` token with the command name it stands in for — `PROT_30` → `bm add`, `PROT_40` → `bm list`, `PROT_46` → `bm list`.

Per [TW domain prompt](../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) "DESIGN.md change authority" clause: the [`DESIGN.md`](../../DESIGN.md) edit must be applied by the [Solution Owner](../../../vsdd-suite/domains/role/SOLUTION-OWNER-REVIEW.md), not by TW. The `manual-tests/layer-1.md` edit does not touch DESIGN.md and can be applied directly under TW ownership once the SO leg is approved (or independently — the two files share a defect class but the SO routing applies only to the DESIGN.md half). Filing as Open here with the SO routing implied via Owner; a [Sanity Check](../../../vsdd-suite/domains/meta/SANITY-CHECK-REVIEW.md) pass is also appropriate before the SO accepts because the corruption may be a tooling-pipeline regression worth diagnosing at the project-pipeline level rather than fixed by hand only.

**Classification:** Deferred — finding raised in Round 1; Round 2 verification required per the Phase 3 primer's continue-trigger discipline.

---

**Finding 2 — README stale: test count and project-directory name no longer match implementation (Dim 1, Dim 2)**

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*

[`README.md`](../../README.md) contains two stale claims relative to the current working tree:

1. **Test count mismatch.** [`README.md`](../../README.md) line 43 asserts: *"expect: 8 tests pass (4 lib unit tests + 4 integration tests)"*. The current implementation has **9** tests: 5 lib unit tests in [`src/lib.rs`](../../src/lib.rs) (`newest_first_sorts_descending_by_timestamp`, `load_returns_empty_for_missing_file`, `load_returns_empty_for_empty_file`, `save_then_load_roundtrips`, `save_creates_parent_directory_for_nested_path` — the last one was added during the Phase 5 Mutation Testing round per [QE Review 2 Finding 1](2026-05-20-quality-engineer.md#review-2--2026-05-20-0245z) Resolution) + 4 integration tests in [`tests/bookmarks.rs`](../../tests/bookmarks.rs). A user following the README's `cargo test` instruction and counting the 8-test expectation will see 9 pass and assume their checkout is wrong (or that a test got added without documentation update — the latter is the actual case).

   This is the precise failure mode [TW Dim 2](../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) names: a documentation claim correct at the time it was written, made stale by a subsequent implementation change. The Phase 5 round at [QE Review 2](2026-05-20-quality-engineer.md#review-2--2026-05-20-0245z) added the fifth lib test on 2026-05-20 02:45Z; the README was last touched in the post-Review-79 anchor-link sweep but the test count was not updated to match.

2. **Project-directory name mismatch.** [`README.md`](../../README.md) lines 20 and 41 both contain `cd <portfolio>/bookmark-cli` — the directory name does not exist. The actual directory is `bookmark-cli-manual` (this is the manual-method reference example; the crosslink-method companion is `bookmark-cli-crosslink` in the same parent directory). A user copy-pasting the install instructions will get a `cd: no such file or directory` error. The project-directory rename predates the README's anchor-link sweep — the file-name reference was missed in the sweep.

   [`manual-tests/install-verification.md`](../../manual-tests/install-verification.md) line 27 correctly uses `cd guild-portfolio/vsdd-suite-reference-examples/bookmark-cli-manual`; the README's instructions are inconsistent with the install-verification doc that's supposed to be derived from them.

**Proposed change to [`README.md`](../../README.md):** update line 43 to `expect: 9 tests pass (5 lib unit tests + 4 integration tests)`; update lines 20 and 41 to `cd <portfolio>/vsdd-suite-reference-examples/bookmark-cli-manual` (matching `install-verification.md`'s path). Consider also: replacing the `<portfolio-url>` / `<portfolio>` angle-bracket placeholders with the kebab-case style the [markdown supplement § GitHub render-target conventions](../../../vsdd-suite/supplements/markdown.md) prescribes (`PORTFOLIO-URL` / `PORTFOLIO-PATH`) — this is a secondary cleanup and may be deferred to a follow-on sweep; the load-bearing fix is the stale path and test count.

**Classification:** Deferred — finding raised in Round 1; Round 2 verification required per the Phase 3 primer's continue-trigger discipline.

---

**Finding 3 — `manual-tests/install-verification.md` Step 3 + Coordination links use wrong relative paths (Dim 2, Dim 13)**

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*

[`manual-tests/install-verification.md`](../../manual-tests/install-verification.md) lives inside the `manual-tests/` subdirectory but four of its outbound links treat the link-source path as if it lived at the project root:

- Line 43: `[\`manual-tests/layer-1.md\`](manual-tests/layer-1.md)` — from `manual-tests/install-verification.md`, the relative target `manual-tests/layer-1.md` resolves to `manual-tests/manual-tests/layer-1.md` (broken). The correct relative target is `layer-1.md` (or `./layer-1.md`).
- Line 63 (in § Coordination with other artifacts): same `[\`manual-tests/layer-1.md\`](manual-tests/layer-1.md)` — same defect.
- Line 64: `[\`PROCESS.md\`](PROCESS.md)` resolves to `manual-tests/PROCESS.md` (broken). Correct: `../PROCESS.md`.
- Line 66: `[\`DESIGN.md\` § Project intent](DESIGN.md#project-intent)` resolves to `manual-tests/DESIGN.md` (broken). Correct: `../DESIGN.md#project-intent`. The file `vsdd-suite/PLATFORM-ENGINEER-REVIEW.md` reference on line 65 is plain-text-styled-as-backticks (not a markdown link), so it is not link-broken — but it lacks the link a TW Dim 13 pass would prefer; the correct link target from this file would be `../vsdd-suite/PLATFORM-ENGINEER-REVIEW.md`.

A reader following the cold-and-follow path the worked example teaches (clone the repo → open `install-verification.md` → click through to `layer-1.md` for the test plan) lands on a 404 on [GitHub](https://github.com/) and a "file not found" in a local editor. The whole point of [Dim 13](../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) is that the clickthrough works; here it doesn't. The [Documentation Reviewer](../../../vsdd-suite/domains/role/DOCUMENTATION-REVIEWER-REVIEW.md) validator pair (registered in [Review 80](../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z)) Cross-reference resolution test catches this class as a cold-reader signal — exactly the surface this finding occupies.

**Proposed change to [`manual-tests/install-verification.md`](../../manual-tests/install-verification.md):** update the four link targets per the corrections above. Also add a link wrapper around the `vsdd-suite/PLATFORM-ENGINEER-REVIEW.md` mention on line 65 per the markdown-supplement anchor-link convention's file-path rule.

**Classification:** Deferred — finding raised in Round 1; Round 2 verification required per the Phase 3 primer's continue-trigger discipline.

---

**Finding 4 — Letter-coded "Surface A.0 + B" hardening verbiage retained in `TODO.md` post-Review-78 (Dim 12)**

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*

[`TODO.md`](../../TODO.md) line 5 contains: *"5 Surfaces A.0+B hardening"*. [`TODO.md`](../../TODO.md) line 41 contains: *"Phase 5 Surfaces A.0 (purity boundary) + B (Mutation Testing) both at closure ..."*. [Review 78](../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-78--2026-05-20-1630z) Finding 4 retired the Surface-letter codes (`A` / `A.0` / `B` / `C` / `D`) in favor of descriptive names (`property-based testing` / `Purity Boundary Audit` / `mutation testing` / `fuzz testing` / `proof execution`). [`TODO.md`](../../TODO.md) is forward-facing prose updated as part of the same PR 6 capstone-intent promotion that adopted the rest of the [Review 78](../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-78--2026-05-20-1630z) conventions — the letter codes are forward-only post-Review-78 prose and should follow the descriptive-name convention.

[`PROCESS.md`](../../PROCESS.md) (newer, post-Review-78) consistently uses descriptive names (`Phase 5 Mutation Testing`, `Phase 5 Purity Boundary Audit`); [`DESIGN.md`](../../DESIGN.md) § Project intent uses descriptive names ("`Purity Boundary Audit executed`", "`property-based testing ... deferred`", "`Fuzz Testing and Proof Execution not applicable`"). [`TODO.md`](../../TODO.md) is the outlier. The [CHANGELOG.md](../../CHANGELOG.md) historical entries using "Surfaces A/A.0/D" and "Surfaces A.0 + B" are preserved per [G-89](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-89) forward-only narrative-preservation; this finding does NOT apply to CHANGELOG.

The defect class is the same one [TW Dim 12](../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) names: a reader landing on [`TODO.md`](../../TODO.md) line 5 (the file's intro narrative) sees "Phase 5 Surfaces A.0+B" and must know what those letters mean before the line is interpretable. The descriptive equivalent — "Phase 5 Purity Boundary Audit + Mutation Testing" — carries the meaning at point of use without lookup.

**Proposed change to [`TODO.md`](../../TODO.md):** line 5 — replace `5 Surfaces A.0+B hardening` with `5 Purity Boundary Audit + Mutation Testing hardening`. Line 41 — replace `Phase 5 Surfaces A.0 (purity boundary) + B (Mutation Testing) both at closure` with `Phase 5 Purity Boundary Audit + Mutation Testing both at closure`. The "(purity boundary)" parenthetical can be removed since the audit's full name carries the meaning.

**Classification:** Deferred — finding raised in Round 1; Round 2 verification required per the Phase 3 primer's continue-trigger discipline.

---

**Finding 5 — `src/lib.rs` public-API rustdoc gap: `pub struct` / `pub` field doc-comments absent (Dim 6)**

**Owner:** software-engineer
**Status:** raised
**Blocked by:** *(none)*

[`src/lib.rs`](../../src/lib.rs) exports a public library API ([`Cargo.toml`](../../Cargo.toml) declares `[lib] name = "bookmark_cli"`). The [Rust supplement § Technical Writer](../../../vsdd-suite/supplements/rust.md) rustdoc-coverage check ([G-137](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-137)) requires: *"all public items (`pub fn`, `pub struct`, `pub enum`, `pub trait`, `pub type`) documented with `///` doc comments"* — verified via `RUSTDOCFLAGS="-D missing_docs" cargo doc --no-deps`. The following public items in [`src/lib.rs`](../../src/lib.rs) lack `///` doc comments:

- `pub struct Bookmark` (line 28) — no `///`
- `pub url: String` (line 29) — no `///`
- `pub timestamp: DateTime<Utc>` (line 30) — no `///`
- `pub struct BookmarkStore` (line 34) — no `///`
- `pub bookmarks: Vec<Bookmark>` (line 36) — no `///`

The four `pub fn` methods on `BookmarkStore` (`load`, `save`, `add`, `newest_first`) DO have `///` doc comments (lines 40–44, 58–59, 74–75, 83–84) — that part of the API surface is documented. The defect is the data-type and field surface.

The `missing_docs` lint is not enabled in [`Cargo.toml`](../../Cargo.toml) (no `[lints.rust] missing_docs = "deny"` block; no `#![deny(missing_docs)]` in [`src/lib.rs`](../../src/lib.rs)). Per the supplement's explicit warning: *"`cargo doc` does not warn on missing docs unless the `missing_docs` lint is explicitly enabled"* — the gap is currently invisible to the project's tooling. The supplement's recommended check `RUSTDOCFLAGS="-D missing_docs" cargo doc --no-deps` would surface the gaps listed above; the project does not run this check in CI (no GitHub Actions workflow file present in the project; capstone-intent normally activates [Platform Engineer](../../../vsdd-suite/domains/role/PLATFORM-ENGINEER-REVIEW.md), so CI shape may be addressed in PR 7's queued PE round — but the project-side `missing_docs` lint enablement is a TW/SE concern independent of CI presence).

[`src/main.rs`](../../src/main.rs) is the binary's entry point and has no `pub` items in scope for the supplement's rule ("internal functions in `main.rs` may be omitted") — main.rs is not in scope for this finding.

**Behavior consequence:** a caller using `bookmark_cli` as a library (the `[lib]` target is enabled in `Cargo.toml` and the methods are reachable from outside the crate) sees `Bookmark` and `BookmarkStore` types in their `cargo doc` output with no documentation — they have to read the source to understand whether they should construct these directly, what invariants the fields satisfy, and whether the fields are stable across versions. Given that this project is a **reference implementation** for the suite's worked example ([G-112](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112)), the library API surface is part of what the worked example teaches — undocumented public types are an anti-pattern the suite itself flags.

**Proposed change to [`src/lib.rs`](../../src/lib.rs):**

```rust
/// A single bookmark — a URL captured at a specific UTC instant.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Bookmark {
    /// The bookmarked URL. No length cap; non-empty validation is the caller's responsibility.
    pub url: String,
    /// The instant the bookmark was captured, in UTC.
    pub timestamp: DateTime<Utc>,
}

/// The persisted collection of bookmarks. Serializes to a flat JSON file
/// per `DESIGN.md` § Interface definitions § Storage format.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct BookmarkStore {
    /// Append-only list of bookmarks in insertion order. Use `newest_first()`
    /// for the user-facing newest-first rendering.
    #[serde(default)]
    pub bookmarks: Vec<Bookmark>,
}
```

Plus add `#![deny(missing_docs)]` at the top of [`src/lib.rs`](../../src/lib.rs) (after the existing `//!` module doc) so subsequent additions to the public API surface trigger a lint failure at `cargo check` time rather than at the next TW review. (This pairs with the [SE supplement § Clippy lint configuration](../../../vsdd-suite/supplements/rust.md) recommendation to include `missing_docs` in the crate-level deny set.)

**Classification:** Deferred — finding raised in Round 1; Round 2 verification required per the Phase 3 primer's continue-trigger discipline.

---

### Hallucinated

**Finding 6 — Claim: `PROCESS.md` AI-co-author disclosure satisfies developer-voice discipline at capstone intent (Dim 11)**

Initial adversarial framing: *"[`PROCESS.md`](../../PROCESS.md) is AI-authored scaffold prose with an explicit disclosure that the discipline is not yet satisfied. At capstone intent, that's a [G-156](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-156) hard-gate failure — file a finding."*

Rejected. The framing is correct that AI-authored PROCESS.md prose does not satisfy [G-156](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-156)'s director-voice requirement; but [`PROCESS.md`](../../PROCESS.md) lines 7–13 + the per-subsection "This subsection requires director-authored prose to satisfy [G-156](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-156)" reminders make this explicit, repeatedly, and route the gate-failure to its owner (the operator) without claiming closure. The file's purpose is to demonstrate the retrospective FORMAT, not to assert the retrospective is closed. The [VDD-IAR Alignment](../../../vsdd-suite/domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md) Dim 7 ("Retrospective honesty") boundary holds: a project that disclaims its gate-incompleteness is honest about the gap; a project that pretends the AI prose satisfies the discipline is dishonest. The current shape is the former.

A finding here would be a TW reviewer adversarially demanding director-voice closure that the file itself has already routed to the operator as their task. The disclosure pattern is exactly what the suite teaches for AI-co-authored artifacts at capstone-intent reference examples; verifying the control holds: the disclosure language is unambiguous (`"This file exists to demonstrate the retrospective FORMAT the suite teaches ... A real project at capstone intent must replace this file's content with director-authored retrospective prose before the layer-gate close criterion 7 is satisfied"` — [`PROCESS.md`](../../PROCESS.md) lines 11–13). The hallucinated framing is the boilerplate-Dim-11-applied-without-reading lens.

The cross-validation: [`manual-tests/install-verification.md`](../../manual-tests/install-verification.md) carries the same shape — AI-co-authored disclosure at the top, the Outcome column explicitly left blank to record "gate not yet satisfied". The pair of disclaimer-headed files demonstrates the same discipline; flagging the pattern would generalize to a [G-156](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-156) misread that would chill future AI-co-authored reference scaffolds (which the suite explicitly endorses for reference examples per the same operator directive cited in the disclosures).

**Classification:** Deferred — finding raised in Round 1; Round 2 verification required per the Phase 3 primer's continue-trigger discipline.

---

### Summary

5 Open findings (4 documentation-defect findings + 1 rustdoc-coverage finding) + 1 Hallucinated finding. The findings cluster in [Dim 2](../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) (accuracy — Findings 1+2+3), [Dim 6](../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) (API docs — Finding 5), [Dim 12](../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) (lookup cost — Findings 1+4), and [Dim 13](../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) (inline-reference navigability — Findings 1+3). Finding 1 is the highest-severity item — the document corruption affects [`DESIGN.md`](../../DESIGN.md) (the project's spec) and a load-bearing manual-test artifact; the NUL-byte pattern suggests a tooling-pipeline issue worth investigating beyond the per-file fix.

Per the [G-131 continue trigger](../../../vsdd-suite/primers/3-review-session.md), Round 2 is mandatory once these findings reach Resolved — the cold pass after the fixes verifies they hold and looks for adjacent defects the fixes may have created (especially the DESIGN.md heading rename — anchor-link inbound from any other file would need to be checked).

**Validator:** [documentation-reviewer](../../../vsdd-suite/domains/role/DOCUMENTATION-REVIEWER-REVIEW.md) is the natural cold-reader pair per the [TW domain prompt](../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) Validator-pair declaration (registered in [Review 80](../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z)). Each Resolved finding above will declare `**Validator:** documentation-reviewer` when the resolution lands; the Doc Reviewer round will read the fixed artifacts cold and confirm the documentation reads coherently to a reader who did not author the fixes. For Finding 1's DESIGN.md half (Raised-to-SO via Owner: solution-owner), the Doc Reviewer pass also crosses with the [Solution Owner](../../../vsdd-suite/domains/role/SOLUTION-OWNER-REVIEW.md)'s next round — the heading rename is a spec-edit that SO approves, then Doc Reviewer validates the result reads cleanly. The session-isolation discipline per [primer 3 § Session isolation](../../../vsdd-suite/primers/3-review-session.md) applies: the Doc Reviewer round runs in a fresh cold session, not as a continuation of this one.

**Coordination:** Finding 1's manual-tests half + Finding 3 + Finding 4 are pure TW-owned documentation edits. Finding 1's DESIGN.md half routes to [Solution Owner](../../../vsdd-suite/domains/role/SOLUTION-OWNER-REVIEW.md) per the [TW domain prompt](../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) "DESIGN.md change authority" clause. Finding 5 routes to [Software Engineer](../../../vsdd-suite/domains/role/SOFTWARE-ENGINEER-REVIEW.md) for the rustdoc additions + the `#![deny(missing_docs)]` lint enablement (the lint-enablement also coordinates with [Platform Engineer](../../../vsdd-suite/domains/role/PLATFORM-ENGINEER-REVIEW.md) if CI is added in PR 7 — the deny set should be enforced in the cargo check step). Finding 2's `<portfolio-url>` / `<portfolio>` angle-bracket cleanup is a [markdown supplement § GitHub render-target conventions](../../../vsdd-suite/supplements/markdown.md) carve-out and can route to a follow-on sweep with the other markdown-supplement-driven cleanups.

A sanity-check pass on Finding 1's NUL-byte corruption pattern by [Sanity Check](../../../vsdd-suite/domains/meta/SANITY-CHECK-REVIEW.md) (validator-of-last-resort) before the SO accepts the DESIGN.md rename would help confirm the corruption is not a regression from any current hook or build step — the artifacts looking like tooling sentinels left in place is the kind of pattern Sanity Check is designed to surface.
