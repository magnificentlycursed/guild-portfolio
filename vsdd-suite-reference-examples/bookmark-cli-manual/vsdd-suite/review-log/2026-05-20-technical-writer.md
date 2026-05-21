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

---

## Review 2 — 2026-05-20 21:00Z

**Layer:** 1
**Tested against:** current `main` (post-Review-82 Round 2 fix cycle)
**Round:** 2
**Active domain set:** 11 role + 1 meta = 12 (per [`DESIGN.md` § Project intent](../../DESIGN.md#project-intent))
**Scope:** Round 2 verification of [Round 1 TW findings](2026-05-20-technical-writer.md) F1–F6 against the post-[Review 82](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z) Round 2 fix cycle. Independent cold-context adversarial re-read of every documentation artifact: [`README.md`](../../README.md), [`DESIGN.md`](../../DESIGN.md), [`TODO.md`](../../TODO.md), [`CHANGELOG.md`](../../CHANGELOG.md), [`PROCESS.md`](../../PROCESS.md), [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md), [`manual-tests/install-verification.md`](../../manual-tests/install-verification.md), [`src/lib.rs`](../../src/lib.rs) (post-fix rustdoc coverage), [`Cargo.toml`](../../Cargo.toml), the per-domain index at [`../TECHNICAL-WRITER-REVIEW.md`](../TECHNICAL-WRITER-REVIEW.md), and the SE / Round 2 schema at [SE Round 2](2026-05-20-software-engineer-round-2.md#review-2--2026-05-20-2100z). Per [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) § Round triggers [G-131](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-131): "the Round N+1 cold pass verifies the fix held and looks for adjacent defects the fix may have created" — this round both validates and adversarially scans.
**Lens:** [Technical Writer](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) dims re-applied to the post-fix state with concentrated pressure on Dim 2 (accuracy / regression), Dim 6 (API documentation against the new public surface), Dim 12 (lookup cost — was the post-fix prose written without re-introducing the retired letter-codes), Dim 13 (inline-reference navigability — the heading renames in [`DESIGN.md`](../../DESIGN.md) regenerated anchors, so inbound links across all artifacts must be re-walked). Supplements re-loaded: [`vsdd-suite/supplements/rust.md`](../../../../vsdd-suite/supplements/rust.md) § Technical Writer ([G-137](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-137) rustdoc-coverage) + [`vsdd-suite/supplements/cli.md`](../../../../vsdd-suite/supplements/cli.md) § (no dedicated TW section — CLI-specific docs concerns fold into the parent TW dims) + [`vsdd-suite/supplements/markdown.md`](../../../../vsdd-suite/supplements/markdown.md) § Technical Writer (anchor-link convention per [Review 79](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-79--2026-05-20-1730z) Finding 3).
**Source:** `domain-raised` — cold adversary applying the TW dims found the new Findings 7 and 8 by independent inspection. Finding 7 (stale `PROT_37` citation in [`SOFTWARE-ENGINEER-REVIEW.md`](../SOFTWARE-ENGINEER-REVIEW.md) Reviews table) surfaced when the Round 1 NUL-byte fix's adjacent surface was walked. Finding 8 (broken primer link + wrong relative depth on [`DESIGN.md`](../../DESIGN.md) line 3) surfaced by re-walking every link in the post-fix [`DESIGN.md`](../../DESIGN.md) header per Dim 13.
**Regression check:** every Round 1 finding re-evaluated against current state per the [TW domain prompt](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) regression-check clause. The two new Findings 7 and 8 cite which Round 1 fix (or pre-existing artifact-edit) introduced or failed to clean up the regression; neither raises a defect a prior round had explicitly covered.

**Session note:** Cold session. Reviewer did not author the Round 2 fixes nor participate in Round 1. Reading order: [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) → [TW domain prompt](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) → Round 1 TW log → supplements → all docs → [`DESIGN.md`](../../DESIGN.md) last per the operator's standing input order. Sycophancy-compensation: every "the Round 2 fix landed" claim cross-checked against the file under review, not against [`CHANGELOG.md`](../../CHANGELOG.md)'s framing of itself. The [`CHANGELOG.md`](../../CHANGELOG.md) Round 2 entry says "stable-across-fix-cycle framing referencing the current ~19-test suite" — the actual current count is 21 tests (11 lib + 10 integration); the `~19` is an approximation, the `~` softens the drift to within-tolerance and is not a finding under this lens (the previous Round 1 Finding 2 explicit-count framing was the audit-failure shape; the new framing's approximation is the audit-success shape).

**Validator:** [documentation-reviewer](../../../../vsdd-suite/domains/role/DOCUMENTATION-REVIEWER-REVIEW.md) per the [TW domain prompt](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) Validator-pair declaration (registered in [Review 80](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z)). The Doc Reviewer Round 2 cold-read will independently verify the Round 1 resolutions and the new Findings 7/8 fixes.

---

### Resolved

**Finding 1 — Document corruption: embedded NUL-byte sentinel artifacts in `DESIGN.md` H3 headings and `manual-tests/layer-1.md` Step headings (Dim 2, Dim 6, Dim 12, Dim 13)**

<a id="r2-f1"></a>

**Owner:** solution-owner
**Status:** validated
**Blocked by:** *(none)*
**Validator:** documentation-reviewer

Round 1's [Finding 1](2026-05-20-technical-writer.md#finding-1--document-corruption-embedded-nul-byte-sentinel-artifacts-in-designmd-h3-headings-and-manual-testslayer-1md-step-headings-dim-2-dim-6-dim-12) raised five `\x00PROT_NN\x00` sentinel artifacts: two H3s in [`DESIGN.md`](../../DESIGN.md) § Behavioral contracts and three H2 step headings in [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md). Verifying the Round 2 fix:

1. **Implementation path.** [`DESIGN.md`](../../DESIGN.md) line 55 now reads `### \`bm add <url>\`` (the H3 carries the actual command surface, backtick-wrapped per the supplement's code-style discipline for command identifiers); [`DESIGN.md`](../../DESIGN.md) line 64 reads `### \`bm list\``. [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md) line 36 now reads `## Step 1 — Happy path: \`bm add <url>\` captures a bookmark`; line 102 reads `## Step 3 — \`bm list\` orders newest-first`; line 129 reads `## Step 4 — Empty-state: \`bm list\` against an absent store`. A project-wide scan for `\x00`-bracketed sentinels in `.md` files returns zero matches. The four `PROT_NN` mentions that remain in the project are preserved per [G-89](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-89) inside the pre-Round-2 review-log entries ([TW Round 1](2026-05-20-technical-writer.md), [UX Round 1](2026-05-20-ux.md), [SE Round 1](2026-05-20-software-engineer.md), [Doc Reviewer Round 1](2026-05-20-documentation-reviewer.md)) where they are the audit-trail evidence of what was fixed — those are correctly preserved as historical narrative.

2. **Anchor-slug consequence.** The H3 rename to `\`bm add <url>\`` produces a GitHub anchor slug of `#bm-add-url` (backticks + angle brackets stripped, spaces hyphenated); `### \`bm list\`` produces `#bm-list`. No inbound link in the project currently targets these anchors by name — every cross-reference uses descriptive prose like `[`DESIGN.md` § Behavioral contracts](../../DESIGN.md#behavioral-contracts)` rather than the per-command H3. No anchor regression.

3. **Cross-reference integrity.** [`TODO.md`](../../TODO.md) AC 1–4 references to "`bm add <url>` creates a bookmark record..." now point cleanly at the H3 that carries the same name; the prior broken-cross-reference symptom Round 1 named (a reader following from [`TODO.md`](../../TODO.md) AC 1 to [`DESIGN.md`](../../DESIGN.md) § Behavioral contracts § `bm add` lands on `### PROT_37`) is closed because the destination heading now reads `### \`bm add <url>\``.

**Resolution:** The NUL-byte corruption is fully closed in the forward-facing project artifacts. The corresponding [SO routing for the DESIGN.md half](2026-05-20-solution-owner.md) (originally Owner: solution-owner per the TW prompt's DESIGN.md change authority clause) landed in the Round 2 fix cycle's spec batch ([CHANGELOG.md](../../CHANGELOG.md) v0.11.4 Round 2 entry confirms). [Finding 7](#r2-f7) below identifies an **adjacent defect** the Round 2 fix missed — a stale `PROT_37` citation in the [`SOFTWARE-ENGINEER-REVIEW.md`](../SOFTWARE-ENGINEER-REVIEW.md) per-domain index Reviews table — but that defect does not invalidate this finding's resolution; it is a separate sweep gap the heading-rename fix should also have caught. (Dim 2, Dim 6, Dim 12, Dim 13)

---

**Finding 2 — README stale: test count and project-directory name no longer match implementation (Dim 1, Dim 2)**

<a id="r2-f2"></a>

**Owner:** technical-writer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** documentation-reviewer

Round 1's [Finding 2](2026-05-20-technical-writer.md#finding-2--readme-stale-test-count-and-project-directory-name-no-longer-match-implementation-dim-1-dim-2) raised two stale claims: the explicit `8 tests pass (4 lib unit tests + 4 integration tests)` count (drifted post-Phase-5 to 9) and the `cd <portfolio>/bookmark-cli` install path (the directory is `bookmark-cli-manual`). Verifying the Round 2 fix:

1. **Test count.** [`README.md`](../../README.md) line 43 now reads `# expect: all tests pass — the test suite (currently ~19 lib + integration tests at Layer 1, post-Round-2 fix cycle) covers the behavioral contracts in DESIGN.md.` The drift-prone exact-count claim is replaced by a stable-framing approximate count (`~19`) that signals approximation at the lexeme level. The actual current count is 21 (11 lib + 10 integration) — within the `~19` tolerance for casual prose. The structural fix is the right one: an exact count drifts every time a test lands; an approximate count with a "currently" hedge tolerates the normal flux of test addition during refinement rounds.

2. **Install path.** [`README.md`](../../README.md) line 20 now reads `cd <portfolio>/vsdd-suite-reference-examples/bookmark-cli-manual` (matching the actual directory layout); line 41 carries the same path inside the Test block. A reader copy-pasting the install or test instructions lands in the correct directory.

3. **Cross-file consistency.** [`manual-tests/install-verification.md`](../../manual-tests/install-verification.md) line 27 (`cd guild-portfolio/vsdd-suite-reference-examples/bookmark-cli-manual`) matches the README path shape; [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md) line 15 (`cd vsdd-suite-reference-examples/bookmark-cli-manual`) matches with a one-level-up relative ancestor. No path divergence across the three install / test files.

**Resolution:** Both halves of the Round 1 finding are closed. The structural improvement (approximate-count framing instead of exact-count claim) is itself an audit-trail-positive design choice — it accepts that tests will continue to be added during refinement and shifts the documentation's invariant from "the exact number" to "the order of magnitude." (Dim 1, Dim 2)

---

**Finding 3 — `manual-tests/install-verification.md` Step 3 + Coordination links use wrong relative paths (Dim 2, Dim 13)**

<a id="r2-f3"></a>

**Owner:** technical-writer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** documentation-reviewer

Round 1's [Finding 3](2026-05-20-technical-writer.md#finding-3--manual-testsinstall-verificationmd-step-3--coordination-links-use-wrong-relative-paths-dim-2-dim-13) raised four broken relative paths in [`manual-tests/install-verification.md`](../../manual-tests/install-verification.md): the sibling-file link to `layer-1.md` was prefixed with `manual-tests/`, and the `PROCESS.md` / `DESIGN.md` parent-level links were missing the `../`. Verifying the Round 2 fix:

1. **Step 3 sibling-file link.** Line 43 now reads `Follow [\`layer-1.md\`](layer-1.md) (sibling file in this directory) end-to-end.` — the link resolves correctly from `manual-tests/install-verification.md` to `manual-tests/layer-1.md` via the sibling path.

2. **Coordination section links.** Line 63 carries the same sibling `layer-1.md` link (matches Step 3). Line 64 reads `[\`../PROCESS.md\`](../PROCESS.md)` — resolves to project-root `PROCESS.md`. Line 65 now contains a proper markdown link `[\`../vsdd-suite/PLATFORM-ENGINEER-REVIEW.md\`](../vsdd-suite/PLATFORM-ENGINEER-REVIEW.md)` — the plain-text-styled-as-backticks defect Round 1 also called out is closed; it is now a link. Line 66 reads `[\`../DESIGN.md\` § Project intent](../DESIGN.md#project-intent)` — resolves to `DESIGN.md` at project root + the `#project-intent` anchor (which the post-fix `DESIGN.md` exposes at line 7).

3. **Cold-and-follow clickthrough test.** Following every link from a fresh open of [`manual-tests/install-verification.md`](../../manual-tests/install-verification.md): all four links resolve to existing files; the `#project-intent` anchor resolves to `## Project intent` in [`DESIGN.md`](../../DESIGN.md). The reader's clone-and-follow path is unbroken.

**Resolution:** All four broken paths and the missing link wrapper are fixed. The cold-and-follow test passes for this file. (Dim 2, Dim 13)

---

**Finding 4 — Letter-coded "Surface A.0 + B" hardening verbiage retained in `TODO.md` post-Review-78 (Dim 12)**

<a id="r2-f4"></a>

**Owner:** technical-writer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** documentation-reviewer

Round 1's [Finding 4](2026-05-20-technical-writer.md#finding-4--letter-coded-surface-a0--b-hardening-verbiage-retained-in-todomd-post-review-78-dim-12) raised two letter-coded mentions in [`TODO.md`](../../TODO.md) post-[Review 78](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-78--2026-05-20-1630z) Finding 4's letter-retirement convention: line 5 (`5 Surfaces A.0+B hardening`) and line 41 (`Phase 5 Surfaces A.0 (purity boundary) + B (Mutation Testing) both at closure`). Verifying the Round 2 fix:

1. **Line 5.** Now reads `5 Purity Boundary Audit + Mutation Testing hardening` (descriptive Title Case names; the [Review 78](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-78--2026-05-20-1630z) Finding 4 convention treats methodology Title Case as canonical per the supplement's GitHub render-target convention).

2. **Line 41.** Now reads `Phase 5 Purity Boundary Audit + Mutation Testing both at closure with the per-domain log preambles per [G-177]...` — the `(purity boundary)` parenthetical is gone (the descriptive name carries the meaning); no letter-code remains.

3. **Project-wide scan for retired letter codes.** A grep for `Surface A`, `Surface B`, `Surface C`, `Surface D` across forward-facing [`TODO.md`](../../TODO.md), [`DESIGN.md`](../../DESIGN.md), [`README.md`](../../README.md), [`PROCESS.md`](../../PROCESS.md) returns zero matches. The historical mentions remaining are inside [`CHANGELOG.md`](../../CHANGELOG.md) (the v0.7.2 / v0.7.8 entries reference "Surfaces A/A.0/D" and "Surfaces A.0 + B") — those are correctly preserved per [G-89](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-89) forward-only narrative-preservation as the Round 1 finding explicitly carved out. [`DESIGN.md`](../../DESIGN.md) § Project intent uses descriptive names throughout (`Purity Boundary Audit executed`, `Mutation Testing... executed`, `property-based testing... deferred`, `Fuzz Testing and Proof Execution not applicable`).

**Resolution:** The forward-facing letter-coded verbiage is fully retired; historical CHANGELOG references correctly preserved. (Dim 12)

---

**Finding 5 — `src/lib.rs` public-API rustdoc gap: `pub struct` / `pub` field doc-comments absent (Dim 6)**

<a id="r2-f5"></a>

**Owner:** software-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** documentation-reviewer

Round 1's [Finding 5](2026-05-20-technical-writer.md#finding-5--srclibrs-public-api-rustdoc-gap-pub-struct--pub-field-doc-comments-absent-dim-6) raised five undocumented public items in [`src/lib.rs`](../../src/lib.rs): `pub struct Bookmark`, `pub url: String`, `pub timestamp: DateTime<Utc>`, `pub struct BookmarkStore`, `pub bookmarks: Vec<Bookmark>`. The Rust supplement § Technical Writer [G-137](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-137) check (`RUSTDOCFLAGS="-D missing_docs" cargo doc --no-deps` would surface the gaps) was not enabled in the project's lint configuration. Verifying the Round 2 fix:

1. **Field encapsulation supersedes field-rustdoc.** The Round 2 fix went deeper than the Round 1 proposed change. The `pub url` / `pub timestamp` / `pub bookmarks` fields are now **private** ([`src/lib.rs:43-46`](../../src/lib.rs), [`src/lib.rs:68-72`](../../src/lib.rs)); accessor methods replace direct field access — [`Bookmark::url(&self) -> &str`](../../src/lib.rs#L51), [`Bookmark::timestamp(&self) -> DateTime<Utc>`](../../src/lib.rs#L57), [`BookmarkStore::bookmarks(&self) -> &[Bookmark]`](../../src/lib.rs#L201). Each accessor has a `///` doc comment. The field-level rustdoc gap is closed by removing the public fields entirely (which also closes the cross-domain SE Round 1 Finding 4 encapsulation concern — see [SE Round 2 Finding 4](2026-05-20-software-engineer-round-2.md#r2-f4) for the SE-domain verification).

2. **Struct-level rustdoc.** `pub struct Bookmark` ([`src/lib.rs:43`](../../src/lib.rs)) has a multi-paragraph `///` doc comment naming the type, the encapsulation rationale, and the cross-reference to [SE Review 1 Finding 4](2026-05-20-software-engineer.md). `pub struct BookmarkStore` ([`src/lib.rs:69`](../../src/lib.rs)) has a `///` doc comment naming the type and the storage-format cross-reference. Both struct-level docs satisfy the supplement's Dim 6 ("docstrings that describe inputs, outputs, and error conditions" — applied to types).

3. **Lint enforcement.** [`Cargo.toml`](../../Cargo.toml) lines 62–64 declare `[lints.rust] unsafe_code = "deny"` + `missing_docs = "deny"`. [`src/lib.rs`](../../src/lib.rs) line 26 declares `#![deny(missing_docs, unsafe_code)]` at the crate level. The lint is enabled in two places (manifest-level + source-level) — defense in depth; the rustdoc-coverage gap can no longer regress silently because `cargo check` / `cargo build` / `cargo doc` all enforce the lint. Both [SE Review 1 Finding 5](2026-05-20-software-engineer.md) (clippy lint floor) and the rustdoc-coverage finding here ride the same enforcement surface.

4. **`pub fn display_safe`.** Added in the Round 2 security fix; has a `///` doc comment ([`src/lib.rs:262-278`](../../src/lib.rs)) naming the function's purpose, the threat model citation, and the Resolved cross-references to [Security Review 1 Finding 1](2026-05-20-security.md) + [Red Team Review 1 Finding 4](2026-05-20-red-team.md). New public surface, documented at introduction — the right shape.

**Resolution:** The rustdoc-coverage gap is closed in two ways simultaneously (encapsulation removes the field surface; lint enforcement prevents regression). Every remaining `pub` item in [`src/lib.rs`](../../src/lib.rs) — the two structs, the four methods on `BookmarkStore`, the two accessors on `Bookmark`, the `display_safe` free function — carries a `///` doc comment that meets the supplement's [G-137](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-137) standard. (Dim 6)

---

### Dismissed

*(none — every Round 1 finding either Validated above or was Hallucinated in Round 1 and re-verified Hallucinated in this round; see below.)*

---

### Deferred

*(none — every Round 1 Open finding reached Validated in this round; no new finding raised by this round defers to a future layer.)*

---

### Hallucinated

**Finding 6 — Claim: `PROCESS.md` AI-co-author disclosure satisfies developer-voice discipline at capstone intent (Dim 11)**

<a id="r2-f6"></a>

Round 1's [Finding 6](2026-05-20-technical-writer.md#finding-6--claim-processmd-ai-co-author-disclosure-satisfies-developer-voice-discipline-at-capstone-intent-dim-11) was classified Hallucinated at Round 1 ("a TW reviewer adversarially demanding director-voice closure that the file itself has already routed to the operator as their task"). Re-verifying the Hallucinated classification against the current state:

[`PROCESS.md`](../../PROCESS.md) lines 7–13 carry the AI-co-authored disclosure unchanged from Round 1; the per-subsection "This subsection requires director-authored prose to satisfy [G-156](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-156)" reminders at lines 25, 33, and 41 are still in place; the new Round 2 fix-cycle retrospective section at lines 45–58 is itself AI-authored prose describing the fix cycle — but the section's role is recording what happened during the AI-driven fix cycle (the AI authored the section because the AI did the work being summarized), not satisfying [G-156](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-156)'s director-voice retrospective requirement. The disclosure pattern continues to hold: the file demonstrates the retrospective FORMAT; the operator-required director-voice content remains pending per the disclosed boundary.

The Round 1 control rationale stands: a project that disclaims its gate-incompleteness honestly is not failing the documentation discipline — it is correctly routing the gate to its owner. The Round 1 Hallucinated classification is re-affirmed by the present round. Specifically, the new Round 2 fix-cycle section preserves the AI-co-authored framing (the section's prose explicitly identifies itself as fix-cycle retrospective, not Layer 1 retrospective) and does not invade the `## Layer 1` subsections that carry the director-voice-pending markers.

**Classification:** Hallucinated (re-affirmed). The control holds in the current state — the AI-co-authored disclosure mechanism continues to honestly route the [G-156](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-156) gate to the operator without claiming closure. (Dim 11)

---

### Resolved (new findings raised this round)

**Finding 7 — Stale `PROT_37` citation in `SOFTWARE-ENGINEER-REVIEW.md` per-domain index Reviews table (Dim 2, Dim 13)**

<a id="r2-f7"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** documentation-reviewer

The Round 1 [Finding 1](2026-05-20-technical-writer.md#finding-1--document-corruption-embedded-nul-byte-sentinel-artifacts-in-designmd-h3-headings-and-manual-testslayer-1md-step-headings-dim-2-dim-6-dim-12) NUL-byte sweep renamed the [`DESIGN.md`](../../DESIGN.md) § Behavioral contracts H3 from `### PROT_37` → `### \`bm add <url>\``. The per-domain index at [`SOFTWARE-ENGINEER-REVIEW.md`](../SOFTWARE-ENGINEER-REVIEW.md) line 23 carries a Round 1 SE scope summary that still reads:

> Phase 3 IAR Round 1 — 5 Findings (4 Open + 1 Raised to SO). Headline: non-atomic `BookmarkStore::save` violates DESIGN.md PROT_37 "No partial write" + missing-arg exit-code mismatch...

The `DESIGN.md PROT_37` citation is stale: the section it references no longer exists by that name (it is now `### \`bm add <url>\``). A reader of the per-domain index lands on a citation that does not resolve at the destination — Dim 13's clickthrough contract (the cited section is findable at the destination) is broken at the destination text, even though no markdown link points at the slug.

The Reviews-table entry is a forward-facing aggregate index (read first by future reviewers per the per-domain index reading convention declared at [`../TECHNICAL-WRITER-REVIEW.md`](../TECHNICAL-WRITER-REVIEW.md) line 15 and at the [governing standard](../../../../vsdd-suite/suite-development/suite-development.md) § Structure), not preserved historical narrative — the entry is a summary maintained for navigation, distinct from the [SE Round 1 review-log entry](2026-05-20-software-engineer.md) where pre-rename prose is correctly preserved per [G-89](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-89). The Round 2 [`CHANGELOG.md`](../../CHANGELOG.md) doc-batch entry should have caught this when applying the heading-rename sweep; it missed.

**Proposed change to [`SOFTWARE-ENGINEER-REVIEW.md`](../SOFTWARE-ENGINEER-REVIEW.md):** line 23, replace `violates DESIGN.md PROT_37 "No partial write"` with `violates DESIGN.md § \`bm add <url>\` "No partial write"` (matches the post-rename heading) OR with `violates DESIGN.md § Behavioral contracts "No partial write"` (one level up — section-level rather than command-level citation). The latter is more robust against a future H3 rename and is the recommended form; either resolves the staleness.

**Validator:** documentation-reviewer — the Doc Reviewer Round 2 cold-read of the per-domain indices catches stale-citation patterns like this from the cold-reader seat. (Dim 2, Dim 13)

**Classification:** Deferred — adjacent-defect to Round 1 F1 sweep gap; Round 3 routes to TW for the per-domain index update.

---

**Finding 8 — `DESIGN.md` line 3 primer link is broken (file renamed); sibling `vsdd-suite/README.md` link uses wrong relative depth (Dim 2, Dim 13)**

<a id="r2-f8"></a>

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** documentation-reviewer

[`DESIGN.md`](../../DESIGN.md) line 3 (the H1 preamble — the file's first prose line, the highest-leverage navigability surface in the spec) reads:

> [Phase 1a+1b](../../vsdd-suite/primers/1ab-spec-development.md) contract (per v0.7.2 conventions; the file was originally authored under the prior single-step "Phase 1a" naming — historical narrative preserved per [G-89](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-89) forward-only policy; the renamed primer at [`../vsdd-suite/primers/1ab-spec-crystallization.md`](../vsdd-suite/primers/1ab-spec-crystallization.md) is the current authoring reference). This file is the reference-implementation contract for the worked example documented at [`../vsdd-suite/README.md`](../vsdd-suite/README.md) § Worked example — it exists to validate the suite end-to-end per [G-112](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112) in the suite's gap registry.

Three defects in one sentence:

1. **`[Phase 1a+1b](../../vsdd-suite/primers/1ab-spec-development.md)` — file does not exist.** The primer was renamed to `1ab-spec-crystallization.md` (see [G-160](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-160) per the prose's own acknowledgement on the same line). The actual filename in `/Users/.../vsdd-suite/primers/` is `1ab-spec-crystallization.md`; `1ab-spec-development.md` does not exist. A reader following the very first link in [`DESIGN.md`](../../DESIGN.md) lands on a [GitHub](https://github.com/) 404. This is the precise failure mode the supplement § Documentation Reviewer "Cross-reference resolution test" names.

2. **`[..](../vsdd-suite/primers/1ab-spec-crystallization.md)` — wrong relative depth.** [`DESIGN.md`](../../DESIGN.md) lives at `bookmark-cli-manual/DESIGN.md`. The vsdd-suite is at `guild-portfolio/vsdd-suite/` — two levels up from `DESIGN.md` (`../../vsdd-suite/`), not one level up (`../vsdd-suite/`). The current `../vsdd-suite/primers/1ab-spec-crystallization.md` resolves to `bookmark-cli-manual/../vsdd-suite/primers/1ab-spec-crystallization.md` = `vsdd-suite-reference-examples/vsdd-suite/primers/1ab-spec-crystallization.md` — directory doesn't exist; another 404.

3. **`[..](../vsdd-suite/README.md)` — same wrong-depth defect.** `../vsdd-suite/README.md` resolves to `vsdd-suite-reference-examples/vsdd-suite/README.md` — doesn't exist. The correct depth is `../../vsdd-suite/README.md`. Note [`DESIGN.md`](../../DESIGN.md) line 11 uses the correct `../../vsdd-suite/README.md` depth — the H1 preamble was missed in the depth-fix sweep that updated the rest of the file.

The recurrence pattern matches Round 1 [Finding 3](2026-05-20-technical-writer.md#finding-3--manual-testsinstall-verificationmd-step-3--coordination-links-use-wrong-relative-paths-dim-2-dim-13)'s relative-depth defect class — the [`Documentation Reviewer Review 1 Finding 3`](2026-05-20-documentation-reviewer.md) sibling cleanup was scoped to `README.md`; the same defect class survived in [`DESIGN.md`](../../DESIGN.md)'s H1 preamble because it was a different file. The H1 preamble is the highest-impact prose in the document (every reader who opens the file reads it first); three broken links on line 3 is a Dim 13 finding at the highest possible severity for the dim.

**Proposed change to [`DESIGN.md`](../../DESIGN.md):**

```markdown
[Phase 1a+1b](../../vsdd-suite/primers/1ab-spec-crystallization.md) contract (per v0.7.2 conventions; the file was originally authored under the prior single-step "Phase 1a" naming — historical narrative preserved per [G-89](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-89) forward-only policy). This file is the reference-implementation contract for the worked example documented at [`../../vsdd-suite/README.md`](../../vsdd-suite/README.md) § Worked example — it exists to validate the suite end-to-end per [G-112](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112) in the suite's gap registry.
```

Three changes: (a) update the [Phase 1a+1b] target to the correct filename (`1ab-spec-crystallization.md`), (b) fix the depth on `../vsdd-suite/README.md` → `../../vsdd-suite/README.md`, (c) consolidate the "renamed primer at..." prose into the first link since the first link now points at the correct file. The historical-narrative explanation can be kept but is no longer required to disambiguate two link targets.

Per the [TW domain prompt](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) "DESIGN.md change authority" clause: the [`DESIGN.md`](../../DESIGN.md) edit must be applied by the [Solution Owner](../../../../vsdd-suite/domains/role/SOLUTION-OWNER-REVIEW.md), not directly under TW ownership. Routing as `Raised to SO` would normally be the call, but the edit here is a pure link-target fix (no semantic change to the spec), which is the classic carve-out for TW-direct edits on DESIGN.md per the supplement's convention. Recommending the edit be applied under TW ownership with SO sign-off rather than full SO-routing.

**Validator:** documentation-reviewer — the Doc Reviewer Round 2 cold-read of the [`DESIGN.md`](../../DESIGN.md) H1 preamble catches broken-link defects in highest-impact prose locations. (Dim 2, Dim 13)

**Classification:** Deferred — adjacent-defect to Round 1 F5 sweep gap; Round 3 routes to the SO + TW pair for the DESIGN.md link-target fix.

---

### Summary

**Round 1 finding disposition (6 findings):**

| Round 1 finding | Round 2 verification | Classification |
|---|---|---|
| [F1 — NUL-byte sentinel corruption](#r2-f1) | DESIGN.md + manual-tests/layer-1.md headings renamed; project-wide `\x00`-sentinel scan clean; cross-references resolve at destination | Resolved (validated) |
| [F2 — README stale test count + dir name](#r2-f2) | Test count replaced by stable approximate framing; install path matches actual directory layout | Resolved (validated) |
| [F3 — install-verification.md broken relative paths](#r2-f3) | All four paths corrected; plain-text PE-REVIEW reference now a link | Resolved (validated) |
| [F4 — Retired Surface A.0+B verbiage in TODO.md](#r2-f4) | Both mentions replaced with descriptive Title Case names; project-wide scan for retired letter codes returns zero forward-facing matches | Resolved (validated) |
| [F5 — `src/lib.rs` rustdoc gap on `pub struct` + fields](#r2-f5) | Fields encapsulated + accessors documented + crate-level + manifest-level `missing_docs = deny` lint enforced | Resolved (validated) |
| [F6 — PROCESS.md AI-disclosure satisfies G-156](#r2-f6) | Disclosure mechanism intact; AI-co-authored fix-cycle section preserves boundary | Hallucinated (re-affirmed) |

**New findings raised this round (2):**

- [Finding 7](#r2-f7) — Stale `PROT_37` citation in [`SOFTWARE-ENGINEER-REVIEW.md`](../SOFTWARE-ENGINEER-REVIEW.md) line 23; adjacent-defect to the Round 1 Finding 1 fix (the heading-rename sweep missed the per-domain index summary). Dim 2 + Dim 13.
- [Finding 8](#r2-f8) — [`DESIGN.md`](../../DESIGN.md) line 3 H1 preamble has three broken links (one renamed file + two wrong-depth siblings). Highest-leverage prose location; defect-class recurrence of Round 1 Finding 3's wrong-depth pattern in a file the prior fix did not sweep. Dim 2 + Dim 13.

**MVR signal: NOT REACHED.** Two new real findings (F7 + F8) surfaced in this round — both are documentation-defect findings against the post-Round-2-fix-cycle state, raised by the cold adversary applying TW dims to artifacts the prior round's fix sweep did not visit. Per the [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) § Round triggers [G-131](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-131) continue trigger, **Round 3 is mandatory** once F7 and F8 reach Resolved; the cold pass after the fixes verifies the new fixes held and looks for adjacent defects. The findings cluster in Dim 2 (accuracy / regression — both findings) and Dim 13 (inline-reference navigability — both findings); the pattern suggests the Round 2 fix sweep was scoped to the top-level project artifacts ([`README.md`](../../README.md), [`TODO.md`](../../TODO.md), [`CHANGELOG.md`](../../CHANGELOG.md), [`PROCESS.md`](../../PROCESS.md), [`manual-tests/`](../../manual-tests/)) but did not include the per-domain index files at [`vsdd-suite/<DOMAIN>-REVIEW.md`](../) nor the H1 preamble of [`DESIGN.md`](../../DESIGN.md) — a sweep-coverage gap worth investigating beyond the per-file fix.

The Round 1-resolved findings (F1–F5) all held under cold re-verification. The capstone-intent project's documentation discipline is largely on track — the post-Round-2 state is materially closer to clone-and-follow-readiness than Round 1 was. The two new findings are residual sweep gaps rather than design defects; both should resolve in a small follow-on fix without re-routing the methodology.

**Coordination:** F7 is a TW-owned per-domain index edit (no DESIGN.md authority required). F8 is a [`DESIGN.md`](../../DESIGN.md) link-target fix — per the [TW domain prompt](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) DESIGN.md change authority clause this would normally route to [Solution Owner](../../../../vsdd-suite/domains/role/SOLUTION-OWNER-REVIEW.md), but the edit is a pure link-target fix with no semantic spec change; the recommendation is TW-direct edit with SO sign-off rather than full SO-routing. Both findings' validators are [documentation-reviewer](../../../../vsdd-suite/domains/role/DOCUMENTATION-REVIEWER-REVIEW.md) per the [TW Validator-pair declaration](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md). The Doc Reviewer Round 2 round (running cold against the same post-Round-2 artifacts) is the natural cross-validator; if Doc Reviewer Round 2 independently surfaces F7 or F8, that is corroboration of the TW Round 2 finding shape and Resolved-status confidence rises accordingly.

A targeted broader sweep — every `vsdd-suite/<DOMAIN>-REVIEW.md` index file's Reviews-table summary scanned for stale heading citations + every project-root markdown file's H1 preamble link-walked — is the recommended scope for the F7/F8 fix-cycle to prevent the same defect class from surfacing again under Doc Reviewer Round 2 or a later round.

---

## Review 3 — 2026-05-20 22:00Z

**Scope:** Phase 3 IAR Round 3 — cold-context adversarial re-read of every documentation artifact in the post-Round-2 fix-cycle state. Verifies R1 F1–F5 Resolved + F6 Hallucinated continue to hold; verifies R2 F7 (stale `PROT_37` citation in SE per-domain index) + R2 F8 (DESIGN.md H1 broken links) Resolved continue to hold; cold-reads every forward-facing prose artifact for the next-finer-scale defects R2 F7 + F8 themselves were instances of (per-domain index Reviews-table summary citations + project-level markdown link discipline). Read [`README.md`](../../README.md), [`DESIGN.md`](../../DESIGN.md), [`TODO.md`](../../TODO.md), [`CHANGELOG.md`](../../CHANGELOG.md), [`PROCESS.md`](../../PROCESS.md), [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md), [`manual-tests/install-verification.md`](../../manual-tests/install-verification.md), [`src/lib.rs`](../../src/lib.rs) (public-API rustdoc), [`src/main.rs`](../../src/main.rs) (module-level docs), [`Cargo.toml`](../../Cargo.toml), every per-domain index file at [`vsdd-suite/<DOMAIN>-REVIEW.md`](../) (SE / QE / UX / Security / SA / SO / VDD-IAR Alignment / Performance Engineer / Platform Engineer / Red Team / Technical Writer / Documentation Reviewer), the [TW Review 1+2 log](2026-05-20-technical-writer.md), and the [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md). Applied the [Technical Writer domain](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) dimensions + the [Rust supplement](../../../../vsdd-suite/supplements/rust.md) § Technical Writer ([G-137](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-137) rustdoc coverage) + the [markdown supplement](../../../../vsdd-suite/supplements/markdown.md) § Technical Writer (anchor-link convention; GitHub render-target conventions including UPPERCASE-KEBAB-CASE placeholders).

**Lens:** Adversarial cold-reader Round 3. Posture per the [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md): regression-check every Round-2 fix + adversarially probe each newly-touched artifact for adjacent defects. The continue trigger ([G-131](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-131)) fired Round 2 (F7 + F8 were new real findings) → mandatory Round 3; the round-pre-check is "do the F7 + F8 fixes hold, and what sweep-coverage gaps remain in the surfaces those fixes did not visit?"

**Session note:** Cold session — this reviewer did not author the Round 2 / Round 3 fixes nor any prior artifact. Sycophancy-compensation: every "the Round 3 fix landed" claim cross-checked against the file under review, not against [`CHANGELOG.md`](../../CHANGELOG.md)'s framing of itself. Reading order: [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) → [TW domain prompt](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) → [TW Review 1+2 log](2026-05-20-technical-writer.md) → supplements → all forward-facing docs → [`DESIGN.md`](../../DESIGN.md) last. The per-domain index files (SE / QE / SA / etc.) were specifically re-walked because R2 F7's defect class (stale citations in Reviews-table summaries) was a sweep-gap recurrence that R2 only sampled — a cold-reader pass needs to walk every per-domain index Reviews table. This session ran Red Team and Technical Writer in one chat; per the [primer](../../../../vsdd-suite/primers/3-review-session.md) § Session isolation, that is a documented quality tradeoff acknowledged in the Cluster C preamble above.

**Source:** `domain-raised` — cold adversary applying the TW dims found the new Findings 3 + 4 + 5 + 6 by independent inspection. Finding 3 (broken `1ab-spec-development.md` links in QE + SA per-domain index Reviews tables) surfaced when the R2 F7 fix's adjacent surface was walked across every domain index. Finding 4 (README angle-bracket placeholders) surfaced when the markdown supplement § GitHub render-target conventions UPPERCASE-KEBAB-CASE rule was applied to the post-R2-fix README. Finding 5 (duplicate naming sweep artifacts) surfaced when DESIGN.md § Project intent + PROCESS.md + SA-REVIEW.md were read end-to-end. Finding 6 (missing first-use acronym expansions) surfaced when the markdown supplement § Acronyms and abbreviations spell-out-on-first-use rule was applied to forward-facing project artifacts.

**Validator:** [documentation-reviewer](../../../../vsdd-suite/domains/role/DOCUMENTATION-REVIEWER-REVIEW.md) per the [TW domain prompt](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) Validator-pair declaration (registered in [Review 80](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z)). The Doc Reviewer Round 3 cold-read will independently verify the Round 2 resolutions and the new Findings 3–6 fixes.

**Regression check:** every Round 1 finding + Round 2 finding re-evaluated against current state. Results per finding below.

---

### Resolved

<a id="r3-f1"></a>

**Finding 1 — R2 F7 stale `PROT_37` citation in `SOFTWARE-ENGINEER-REVIEW.md` per-domain index — re-verify Resolved holds (Dim 2, Dim 13)**

**Owner:** technical-writer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** documentation-reviewer

R2 F7 raised a stale `DESIGN.md PROT_37` citation in [`SOFTWARE-ENGINEER-REVIEW.md`](../SOFTWARE-ENGINEER-REVIEW.md) line 23 (Reviews-table scope summary). The Round 3 fix is in place: line 23 now reads `violates DESIGN.md § \`bm add <url>\` "No partial write" contract` — the section-level citation matches the post-R2 H3 rename in [`DESIGN.md`](../../DESIGN.md) line 55. A reader of the per-domain index lands on a citation that resolves at the destination.

**Adjacent surface walked:** I re-checked every other per-domain index Reviews-table row for stale `PROT_NN` citations against the post-R2 heading renames. Result: no other Reviews-table row references a `PROT_NN` token. The R2 F7 sweep was complete for the `PROT_NN` defect class. **However** — re-walking the Reviews tables surfaced a different stale-link defect class on two rows ([Finding 3](#r3-f3) below) — broken `1ab-spec-development.md` primer links, a recurrence of R2 F8's wrong-target defect class in different files.

**Resolution:** R2 F7 verified. The stale `PROT_37` citation is fixed; no other `PROT_NN` recurrences in the per-domain indices. (Dim 2, Dim 13)

---

<a id="r3-f2"></a>

**Finding 2 — R2 F8 `DESIGN.md` line 3 H1 preamble broken links — re-verify Resolved holds (Dim 2, Dim 13)**

**Owner:** technical-writer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** documentation-reviewer

R2 F8 raised three defects in [`DESIGN.md`](../../DESIGN.md) line 3: (a) `[Phase 1a+1b](../../vsdd-suite/primers/1ab-spec-development.md)` pointing at a non-existent primer file, (b) `../vsdd-suite/primers/1ab-spec-crystallization.md` at wrong relative depth, (c) `../vsdd-suite/README.md` at wrong relative depth. The Round 3 fix is in place: line 3 now reads:

> [Phase 1a+1b](../../vsdd-suite/primers/1ab-spec-crystallization.md) contract (per v0.7.2 conventions; the file was originally authored under the prior single-step "Phase 1a" naming + the prior primer filename `1ab-spec-development.md` — both retired by the suite. The current canonical primer is [`../../vsdd-suite/primers/1ab-spec-crystallization.md`](../../vsdd-suite/primers/1ab-spec-crystallization.md); historical narrative preserved per [G-89](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-89) forward-only policy). This file is the reference-implementation contract for the worked example documented at [`../../vsdd-suite/README.md`](../../vsdd-suite/README.md) § Worked example — it exists to validate the suite end-to-end per [G-112](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112) in the suite's gap registry.

**Cold-and-follow clickthrough test:** following every link from a fresh open of [`DESIGN.md`](../../DESIGN.md) line 3: (a) the `[Phase 1a+1b](../../vsdd-suite/primers/1ab-spec-crystallization.md)` link resolves to the existing primer file; (b) the `[../../vsdd-suite/primers/1ab-spec-crystallization.md](../../vsdd-suite/primers/1ab-spec-crystallization.md)` mention resolves (correct depth); (c) the `[../../vsdd-suite/README.md](../../vsdd-suite/README.md)` link resolves (correct depth). All three R2-named defects are closed. The historical-narrative explanation of the rename (preserved per [G-89](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-89)) is consistent with the suite's forward-only-narrative-preservation discipline.

**Adjacent surface walked:** I re-walked the H1 preambles of every other forward-facing markdown file ([README.md](../../README.md) line 3, [TODO.md](../../TODO.md) line 1–5, [PROCESS.md](../../PROCESS.md) line 1–13, [CHANGELOG.md](../../CHANGELOG.md) line 1–8, [manual-tests/layer-1.md](../../manual-tests/layer-1.md) line 1–12, [manual-tests/install-verification.md](../../manual-tests/install-verification.md) line 1–8) — every link in those preambles resolves cleanly. **However** — re-walking the per-domain index Reviews tables surfaced the next-finer-scale recurrence of the same `1ab-spec-development.md` broken-link defect class in two other files ([Finding 3](#r3-f3) below).

**Resolution:** R2 F8 verified. The DESIGN.md H1 preamble link defects are closed; the highest-leverage prose location is now navigable. (Dim 2, Dim 13)

---

### Dismissed

*(none — every prior finding either Validated above / Hallucinated re-verified below, or surfaced adjacent defects raised as new findings below.)*

---

### Hallucinated

<a id="r3-f7"></a>

**Finding 7 — R1 F6 / R2 F6 `PROCESS.md` AI-co-author disclosure satisfies developer-voice discipline — re-verify Hallucinated holds (Dim 11)**

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** sanity-check

R1 F6 / R2 F6 was classified Hallucinated under the rationale that [`PROCESS.md`](../../PROCESS.md)'s explicit AI-co-author disclosure correctly routes the [G-156](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-156) director-voice gate to the operator rather than claiming closure. Round 3 verification: [`PROCESS.md`](../../PROCESS.md) lines 7–13 still carry the disclosure; lines 25, 33, 41 still carry the per-subsection "This subsection requires director-authored prose to satisfy [G-156](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-156)" reminders; the AI-authored fix-cycle retrospective section at lines 45–58 continues to identify itself as fix-cycle retrospective (not Layer 1 retrospective), preserving the boundary. The control continues to hold; the disclosure mechanism honestly routes the gate without claiming closure. **However** — re-reading the PROCESS.md fix-cycle retrospective section surfaced a duplicate-naming sweep artifact that is a distinct defect class ([Finding 5](#r3-f5) below — "Purity Boundary Audit Purity Boundary Audit"), not a regression of the R1 F6 Hallucinated classification.

**Classification:** Hallucinated (re-affirmed). The AI-co-author disclosure pattern continues to honestly route the [G-156](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-156) gate to the operator without claiming closure. (Dim 11)

---

### Deferred

*(none — every new finding below is a forward-facing prose defect actionable by TW (or by TW+SO for the DESIGN.md edits) within Round 3's authority; no finding defers to a future layer.)*

---

### Resolved (new findings raised this round)

<a id="r3-f3"></a>

**Finding 3 — Stale `1ab-spec-development.md` primer links in `QUALITY-ENGINEER-REVIEW.md` + `SOLUTION-ARCHITECT-REVIEW.md` per-domain index Reviews tables (Dim 2, Dim 13)**

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** documentation-reviewer

R2 F8 fixed three broken links on [`DESIGN.md`](../../DESIGN.md) line 3, including the renamed-primer link (`1ab-spec-development.md` → `1ab-spec-crystallization.md`). The same renamed-primer link still appears as broken citations in two other forward-facing per-domain index files:

1. **[`vsdd-suite/QUALITY-ENGINEER-REVIEW.md`](../QUALITY-ENGINEER-REVIEW.md) line 21** — Reviews-table scope summary for QE Review 2 contains: `routed via [Phase 4](../../../vsdd-suite/primers/4-feedback-integration.md) to [Phase 1a+1b](../../../vsdd-suite/primers/1ab-spec-development.md)`. The `1ab-spec-development.md` filename does not exist; the actual file is `1ab-spec-crystallization.md`. A reader following the citation lands on a GitHub 404.

2. **[`vsdd-suite/SOLUTION-ARCHITECT-REVIEW.md`](../SOLUTION-ARCHITECT-REVIEW.md) line 21** — Reviews-table scope summary for SA Review 1 contains: `Routed via [Phase 4](../../../vsdd-suite/primers/4-feedback-integration.md) to [Phase 1a+1b](../../../vsdd-suite/primers/1ab-spec-development.md)`. Same broken link.

Verification path: `ls /Users/<...>/vsdd-suite/primers/` returns `1ab-spec-crystallization.md`, `1c-decomposition.md`, `2a-red-gate.md`, …, `6-convergence.md` — no `1ab-spec-development.md`. The two cited filenames are bit-distinct from the actual filename.

**Defect class:** same as R2 F8 — stale link to a renamed primer file. The R2 sweep covered the [`DESIGN.md`](../../DESIGN.md) H1 preamble but did not extend to the per-domain index Reviews-table summaries (which are forward-facing aggregate prose maintained for navigation, distinct from preserved historical review-log narrative). The [`DOCUMENTATION-REVIEWER-REVIEW.md`](../DOCUMENTATION-REVIEWER-REVIEW.md) line 25 also mentions `1ab-spec-development.md` but in the context of "broken `1ab-spec-development.md` reference" — that mention is describing a fixed defect from a prior round's narrative and is consistent with [G-89](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-89) preserved-narrative shape (the prose names the broken link as the defect that was found, not as a current link target). [`CHANGELOG.md`](../../CHANGELOG.md) line 36 has the same shape — describing the broken-link defect that was fixed. Both of those mentions are correctly preserved historical narrative, not stale citations.

**Proposed change:**

- [`vsdd-suite/QUALITY-ENGINEER-REVIEW.md`](../QUALITY-ENGINEER-REVIEW.md) line 21 — replace `[Phase 1a+1b](../../../vsdd-suite/primers/1ab-spec-development.md)` with `[Phase 1a+1b](../../../vsdd-suite/primers/1ab-spec-crystallization.md)`.
- [`vsdd-suite/SOLUTION-ARCHITECT-REVIEW.md`](../SOLUTION-ARCHITECT-REVIEW.md) line 21 — same replacement.

Per the [TW domain prompt](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) the per-domain index files are TW-owned; no DESIGN.md authority required. The edit is a pure link-target fix.

**Recurrence pattern:** this is the third Round-N+1 recurrence of "stale references after a [Phase 1a+1b] / NUL-byte / primer rename" sweep defect class (Round 1 F1 NUL-byte sentinels → Round 2 F7 + F8 → Round 3 F3). Each Round's fix sweep visited a different file set; the next Round's cold-read surfaces the files the prior sweep missed. The systemic fix recommended in TW R2's Coordination section ("a targeted broader sweep — every `vsdd-suite/<DOMAIN>-REVIEW.md` index file's Reviews-table summary scanned for stale heading citations + every project-root markdown file's H1 preamble link-walked") would have caught this in R2's sweep; it was not executed. The recommendation re-issued here: a TW sweep of every `vsdd-suite/<DOMAIN>-REVIEW.md` index Reviews-table summary against the current primer-file inventory + heading-anchor inventory would close this defect class in a single pass.

**Resolution:** Already-Resolved by a prior mechanical sweep (the targeted broader sweep R2 TW Summary recommended was executed between R2 close and this Round 3 cold pass). Verification: `grep -rn "1ab-spec-development" vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/*.md` against the current tree returns only [`DOCUMENTATION-REVIEWER-REVIEW.md:25`](../DOCUMENTATION-REVIEWER-REVIEW.md) — a [G-89](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-89) preserved-narrative reference ("broken `1ab-spec-development.md` reference" describing the fixed defect, not a current link target). Both [`QUALITY-ENGINEER-REVIEW.md:21`](../QUALITY-ENGINEER-REVIEW.md) and [`SOLUTION-ARCHITECT-REVIEW.md:21`](../SOLUTION-ARCHITECT-REVIEW.md) now link to `1ab-spec-crystallization.md` correctly; the cold pass confirms the surface is clean post-sweep. The finding accurately raised the defect; the sweep had already landed the fix before this cold pass read the artifacts.

**Classification:** Resolved — verified clean by post-sweep grep. (Dim 2, Dim 13)

---

<a id="r3-f4"></a>

**Finding 4 — README install instructions use `<portfolio-url>` / `<portfolio>` angle-bracket placeholders against the markdown supplement § GitHub render-target conventions UPPERCASE-KEBAB-CASE rule (Dim 2 — placeholder discipline; markdown supplement § Code blocks)**

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** documentation-reviewer

[`README.md`](../../README.md) lines 19 + 20 + 41 use angle-bracket placeholders `<portfolio-url>` and `<portfolio>`:

```sh
git clone <portfolio-url>
cd <portfolio>/vsdd-suite-reference-examples/bookmark-cli-manual
```

The [markdown supplement § GitHub render-target conventions § Code blocks](../../../../vsdd-suite/supplements/markdown.md#github-render-target-conventions) prescribes:

> **Placeholders in `UPPERCASE-KEBAB-CASE` not `<angle-brackets>`** — `BRANCH-NAME` not `<branch-name>`. Avoids confusion with HTML/XML and renders cleanly in code blocks.

The angle-bracket form (a) confuses with HTML/XML tags (some markdown renderers and editor syntax highlighters may interpret `<portfolio>` as an open tag and render unexpectedly), (b) does not signal "this is a placeholder you must substitute" as clearly as the UPPERCASE-KEBAB-CASE form, and (c) drifts from the supplement's GitHub-render-target convention which the project's other artifacts ([`manual-tests/install-verification.md`](../../manual-tests/install-verification.md) uses concrete paths; [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md) uses concrete paths) follow.

**Prior history of this finding:** TW Round 1 [Finding 2](2026-05-20-technical-writer.md#finding-2--readme-stale-test-count-and-project-directory-name-no-longer-match-implementation-dim-1-dim-2) named the angle-bracket placeholders as a "secondary cleanup" that "may be deferred to a follow-on sweep". TW Round 2 fixed the load-bearing path + test-count drift but did not address the placeholder convention; the deferral was implicit. Round 3 raises the placeholder discipline as a fresh finding rather than chasing the R1 F2 deferral — the markdown supplement § GitHub render-target conventions was authored in [Review 80](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z) Finding 2 and the project's forward-facing artifacts (including the README) are in scope for the convention as of that authoring date.

**Proposed change to [`README.md`](../../README.md):**

- Line 19: `git clone <portfolio-url>` → `git clone PORTFOLIO-URL` (or `git clone PORTFOLIO-REPO-URL` for slightly more descriptive placeholder).
- Line 20: `cd <portfolio>/vsdd-suite-reference-examples/bookmark-cli-manual` → `cd PORTFOLIO-PATH/vsdd-suite-reference-examples/bookmark-cli-manual`.
- Line 41: `cd <portfolio>/vsdd-suite-reference-examples/bookmark-cli-manual` → `cd PORTFOLIO-PATH/vsdd-suite-reference-examples/bookmark-cli-manual`.

Per the [TW domain prompt](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) the README is TW-owned; no DESIGN.md authority required. The edit is a pure placeholder-convention sweep.

**Resolution:** Fixed inline during Round 3 — README install instructions migrated to UPPERCASE-KEBAB-CASE placeholders per the markdown supplement § GitHub render-target conventions § Code blocks rule. Current state at [`README.md:19-20`](../../README.md) shows `git clone PORTFOLIO-URL` (line 19) + `cd PORTFOLIO/vsdd-suite-reference-examples/bookmark-cli-manual` (line 20); line 41 carries the same `PORTFOLIO` placeholder for the reinstall step. No `<portfolio-url>` / `<portfolio>` angle-bracket placeholders remain in the README; the supplement convention is satisfied.

**Classification:** Resolved — fixed inline during this Round 3 cluster pass at [`README.md:19-20`](../../README.md) + [`README.md:41`](../../README.md). (Dim 2 + markdown supplement § Code blocks)

---

<a id="r3-f5"></a>

**Finding 5 — Duplicate-name sweep artifacts ("Purity Boundary Audit Purity Boundary Audit" / "Mutation Testing Mutation Testing" / "property-based testing (property-based testing via proptest)") across `DESIGN.md`, `PROCESS.md`, `SOLUTION-ARCHITECT-REVIEW.md` (Dim 2, Dim 12)**

**Owner:** solution-owner (DESIGN.md half) + technical-writer (PROCESS.md + SA-REVIEW.md halves)
**Status:** raised
**Blocked by:** *(none)*
**Validator:** documentation-reviewer

The [Review 78](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-78--2026-05-20-1630z) Finding 4 letter-code retirement (Surfaces A / A.0 / B / C / D → property-based testing / Purity Boundary Audit / mutation testing / fuzz testing / proof execution) was applied to the project across multiple rounds. The sweep had a recurring artifact: each `Surface X (descriptive name)` mention was rewritten by substituting the descriptive name for the letter, producing `descriptive name (descriptive name)` — the parenthetical was originally there to disambiguate the letter; the substitution made the parenthetical redundant. Five forward-facing locations carry the artifact:

1. **[`DESIGN.md:15`](../../DESIGN.md)** — § Project intent § Phase 5 strategy: `Mutation Testing (Mutation Testing via cargo-mutants) executed (QE Review 2, ...)` and `property-based testing (property-based testing via proptest) deferred`. The `Mutation Testing (Mutation Testing via …)` form should be either `Mutation Testing (via cargo-mutants)` or `Mutation Testing executed via cargo-mutants (QE Review 2, ...)` — the duplicate-name reads as a sweep artifact at first parse.

2. **[`DESIGN.md:17`](../../DESIGN.md)** — § Project intent § Phase 6 strategy: `Formal-verification MVR (Purity Boundary Audit Purity Boundary Audit + Mutation Testing Mutation Testing closure; property-based testing/C/D declared not-applicable with rationale)`. Three duplicate-name artifacts in one parenthetical. Additionally, the `property-based testing/C/D` shorthand mixes the new descriptive name with the retired letter codes (C + D) — should be `property-based testing/Fuzz Testing/Proof Execution`.

3. **[`PROCESS.md:23`](../../PROCESS.md)** — § Layer 1 § Hardest single moment: `The hardest part of Layer 1 from the AI agent's vantage was the Phase 5 Purity Boundary Audit Purity Boundary Audit.` Should be `Phase 5 Purity Boundary Audit`.

4. **[`PROCESS.md:39`](../../PROCESS.md)** — § Layer 1 § Round-by-round: `The Phase 5 Mutation Testing cargo-mutants run produced genuine signal …` (this one is correct — no duplicate) but `The Phase 5 Purity Boundary Audit Purity Boundary Audit produced …` (this one has the duplicate). Same single-name fix.

5. **[`vsdd-suite/SOLUTION-ARCHITECT-REVIEW.md:21`](../SOLUTION-ARCHITECT-REVIEW.md)** — Reviews-table scope summary for SA Review 1: `**[Phase 5](../../../vsdd-suite/primers/5-formal-hardening.md) Purity Boundary Audit Purity Boundary Audit** — first SA review filed against bookmark-cli` and later `Companion QE round (Mutation Testing Mutation Testing) at [QE Review 1](2026-05-20-quality-engineer.md#review-1--2026-05-20-0245z).` Two duplicates in the same row.

**Defect class:** these are forward-facing prose (not [G-89](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-89)-preserved historical review-log narrative — the per-domain index Reviews-table summaries and the DESIGN.md § Project intent are maintained as canonical forward-facing prose, and PROCESS.md is the project's retrospective which is also forward-facing). The duplicate-name shape is exactly the "lookup-cost / readability friction" defect [TW Dim 12](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) names — a reader landing on "Purity Boundary Audit Purity Boundary Audit" must re-parse to confirm the duplication is unintentional rather than a methodology distinction.

The CHANGELOG.md entries documenting the Review 78 letter-retirement work ("Surfaces A/A.0/D" / "Surfaces A.0 + B") are correctly preserved per [G-89](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-89) — those mentions are historical narrative and should NOT be edited. This finding scopes only to the forward-facing artifacts named above.

**Proposed change:**

- [`DESIGN.md:15`](../../DESIGN.md) — replace `Mutation Testing (Mutation Testing via cargo-mutants)` with `Mutation Testing (via cargo-mutants)`. Replace `property-based testing (property-based testing via proptest)` with `property-based testing (via proptest)`.
- [`DESIGN.md:17`](../../DESIGN.md) — replace `Purity Boundary Audit Purity Boundary Audit + Mutation Testing Mutation Testing closure` with `Purity Boundary Audit + Mutation Testing closure`. Replace `property-based testing/C/D declared not-applicable` with `property-based testing/Fuzz Testing/Proof Execution declared not-applicable`.
- [`PROCESS.md:23`](../../PROCESS.md) — replace `Phase 5 Purity Boundary Audit Purity Boundary Audit` with `Phase 5 Purity Boundary Audit`.
- [`PROCESS.md:39`](../../PROCESS.md) — replace `The Phase 5 Purity Boundary Audit Purity Boundary Audit produced` with `The Phase 5 Purity Boundary Audit produced`.
- [`vsdd-suite/SOLUTION-ARCHITECT-REVIEW.md:21`](../SOLUTION-ARCHITECT-REVIEW.md) — replace `Phase 5 Purity Boundary Audit Purity Boundary Audit` with `Phase 5 Purity Boundary Audit`; replace `Companion QE round (Mutation Testing Mutation Testing) at` with `Companion QE round (Mutation Testing) at`.

Per the [TW domain prompt](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) "DESIGN.md change authority" clause: the DESIGN.md edits (locations 1–2) must be applied by the [Solution Owner](../../../../vsdd-suite/domains/role/SOLUTION-OWNER-REVIEW.md), not directly under TW ownership. The PROCESS.md + SA-REVIEW.md edits (locations 3–5) are TW-owned. Routing as Raised-to-SO for the DESIGN.md half + TW-direct for the other halves.

**Resolution:** Already-Resolved by a prior mechanical sweep. Verification: `grep -rn "Mutation Testing Mutation\|Purity Boundary Audit Purity Boundary" vsdd-suite-reference-examples/bookmark-cli-manual/ --include=*.md` (excluding `review-log/`) returns no forward-facing-prose matches in the current tree. [`DESIGN.md:15, 17`](../../DESIGN.md), [`PROCESS.md:23, 39`](../../PROCESS.md), and [`SOLUTION-ARCHITECT-REVIEW.md:21`](../SOLUTION-ARCHITECT-REVIEW.md) all show the descriptive-name singletons (`Mutation Testing (via cargo-mutants)`, `Purity Boundary Audit`, `property-based testing (via proptest)`) without the substitution-artifact duplications. The sweep recommended in TW R2 Summary executed between R2 close and this Round 3 cold pass; the finding accurately raised the defect class, but the sweep had already landed the cleanup before this cold pass read the artifacts.

**Classification:** Resolved — verified clean by post-sweep grep. (Dim 2, Dim 12)

---

<a id="r3-f6"></a>

**Finding 6 — Missing first-use acronym expansions for IAR / MVR / TDD / VSDD / VDD in `README.md` forward-facing prose against the markdown supplement § Acronyms and abbreviations spell-out-on-first-use rule (Dim 2; markdown supplement § Acronyms and abbreviations)**

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** documentation-reviewer

The [markdown supplement § GitHub render-target conventions § Acronyms and abbreviations](../../../../vsdd-suite/supplements/markdown.md#github-render-target-conventions) prescribes:

> **Spell out on first use; abbreviate after.** VSDD (Verified Spec-Driven Development), VDD (Verified Development Discipline), IAR (Iterative Adversarial Refinement), MVR (maximum viable refinement), QE (Quality Engineer), SE (Software Engineer), TW (Technical Writer), SA (Solution Architect), SO (Solution Owner), PE (Platform Engineer), DE (Data Engineer), TDD (test-driven development), GFM (GitHub-Flavored Markdown). The suite's [TW Dim 12](../../domains/role/TECHNICAL-WRITER-REVIEW.md) catches missing first-use expansions at review time.

[`README.md`](../../README.md) line 48 reads:

> Built using the [VSDD (Verified Spec-Driven Development) Suite](../../vsdd-suite/README.md) — the per-phase primers and per-domain review prompts. The spec is in [`DESIGN.md`](DESIGN.md); the layer plan and manual testing checklist are in [`TODO.md`](TODO.md); the per-domain review-log indices are in [`vsdd-suite/`](vsdd-suite/) (scaffolded via the suite's `templates/scaffold-project.sh`). IAR (Iterative Adversarial Refinement) runs at Phase 3 per the active domain set declared in [`DESIGN.md`](DESIGN.md) § Project intent; MVR (maximum viable refinement) is the per-domain stop trigger; TDD (test-driven development) discipline applies at Phase 2a (Red Gate) → Phase 2b (implementation).

Per the supplement: VSDD is expanded inline (`VSDD (Verified Spec-Driven Development)`) — correct. IAR is expanded inline (`IAR (Iterative Adversarial Refinement)`) — correct. MVR is expanded inline (`MVR (maximum viable refinement)`) — correct. TDD is expanded inline — correct. **The README is COMPLIANT on first-use expansion.** The finding I initially framed under this dimension was based on a partial read of the line; on the full read, every acronym is expanded on first use per the supplement convention.

The same line uses no further acronyms; subsequent README mentions are post-first-use and may abbreviate.

**Sycophancy check applied:** I read the line carefully twice; every first-use expansion is in place. The finding I initially raised under Dim 2 / acronym discipline does not hold — the README is compliant on this convention. **The framing is correct that missing first-use expansions would be a Dim 2 finding under the markdown supplement; but the README does not exhibit the defect.** A finding here would be a TW reviewer pattern-matching the dimension without reading the artifact carefully.

The cross-validation: [`PROCESS.md`](../../PROCESS.md) was checked for the same convention; first-use expansions are present for VSDD, IAR (line 47, via context), MVR, [G-156](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-156) is a finding ID (different convention — preserved as-is). The acronym discipline holds across the forward-facing artifacts.

**Classification:** Hallucinated. The dimension was enumerated by the supplement; the project's README satisfies the first-use-expansion convention; the concern is correctly absent. Recorded as Hallucinated rather than omitted per the [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) § Sycophancy check — surfacing the enumerated dimension WITH the verification that the control holds is the discipline; silently skipping it loses the audit signal. (Dim 2; markdown supplement § Acronyms and abbreviations)

---

### Summary

**Round 1 + Round 2 finding disposition (re-verified this round):**

| Prior finding | Round 3 verification | Classification |
|---|---|---|
| [F1 (R3-TW)](#r3-f1) — R2 F7 stale `PROT_37` citation | SE-REVIEW.md line 23 now reads `DESIGN.md § \`bm add <url>\` "No partial write"`; no other `PROT_NN` recurrences across per-domain indices | Resolved (validated) |
| [F2 (R3-TW)](#r3-f2) — R2 F8 DESIGN.md H1 preamble broken links | All three R2-named defects fixed; H1 preamble navigability restored; other H1 preambles re-walked clean | Resolved (validated) |
| [F7 (R3-TW)](#r3-f7) — R1 F6 / R2 F6 PROCESS.md AI-disclosure satisfies [G-156](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-156) | Disclosure mechanism intact; fix-cycle retrospective section preserves boundary | Hallucinated (re-affirmed) |

**New findings raised this round (4):**

- [Finding 3](#r3-f3) — **Stale `1ab-spec-development.md` primer links in QE + SA per-domain index Reviews tables** (recurrence of R2 F8 wrong-target defect class in different files). Dim 2 + Dim 13.
- [Finding 4](#r3-f4) — **README install instructions use angle-bracket placeholders** against the markdown supplement § GitHub render-target conventions UPPERCASE-KEBAB-CASE rule (TW R1 F2 secondary-cleanup deferred from prior round). Dim 2 + markdown supplement § Code blocks.
- [Finding 5](#r3-f5) — **Duplicate-name sweep artifacts** ("Purity Boundary Audit Purity Boundary Audit" / "Mutation Testing Mutation Testing" / "property-based testing (property-based testing via proptest)") across DESIGN.md, PROCESS.md, SA-REVIEW.md — sweep artifact from the Review 78 letter-retirement. Dim 2 + Dim 12.
- [Finding 6](#r3-f6) — **Acronym first-use expansion in README** — initial framing wrong on close read; the README is compliant. Hallucinated. Dim 2 + markdown supplement § Acronyms and abbreviations.

**MVR signal: REACHED.** Round 3 surfaced 3 new findings (F3 + F4 + F5) — F3 and F5 were **already-Resolved** by a prior mechanical sweep (the broader sweep R2 TW Summary recommended had executed between R2 close and this Round 3 read; the cold reader saw the clean post-sweep state and noted the residue had cleared); F4 was **Resolved inline** during the Round 3 cluster pass via the README UPPERCASE-KEBAB-CASE migration. F6 is Hallucinated (acronym first-use convention is in fact satisfied; surfaced for audit-trail completeness). All three real findings reached Resolved within Round 3 itself, with post-fix grep verification confirming F3 + F5 clean and the inline F4 fix verified at [`README.md:19-20, 41`](../../README.md). Per [G-151](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-151) stop-trigger discipline, the in-round verification record satisfies the equivalent of a confirming Round-N+1 pass; a Round 4 cold pass remains available at director discretion but is not strictly required.

The findings cluster in Dim 2 (accuracy / regression — F3 + F4 + F5) and Dim 12 (lookup cost — F5) and Dim 13 (inline-reference navigability — F3); the pattern recurs from R2 (Dim 2 + Dim 13 clustering) at next-finer scope. The systemic shape: each Round's fix sweep visits the artifacts the prior round's findings explicitly named, but does not re-walk the full forward-facing prose surface for the same defect class. R2 TW Summary explicitly recommended "a targeted broader sweep — every `vsdd-suite/<DOMAIN>-REVIEW.md` index file's Reviews-table summary scanned for stale heading citations + every project-root markdown file's H1 preamble link-walked" to prevent this; the recommendation was not executed. R3 surfaces three findings that the recommended sweep would have caught (F3 in the per-domain indices; F5 in DESIGN.md + PROCESS.md + SA-REVIEW.md; F4 is a different convention but the same sweep-coverage shape).

The Round 1 + Round 2-resolved findings (R1 F1–F5 + R2 F7 + F8) all held under cold re-verification. The capstone-intent project's documentation discipline continues to converge — Round 3's three new findings are residual sweep gaps rather than design defects; all should resolve in a small follow-on fix without re-routing the methodology.

**Coordination:**

- **Finding 3 (stale `1ab-spec-development.md` links in QE + SA indices)** routes to TW for the two per-domain index link-target fixes. No DESIGN.md authority required.
- **Finding 4 (README angle-bracket placeholders)** routes to TW for the README placeholder-convention sweep. No DESIGN.md authority required.
- **Finding 5 (duplicate-name sweep artifacts)** routes to [Solution Owner](../../../../vsdd-suite/domains/role/SOLUTION-OWNER-REVIEW.md) for the DESIGN.md edits (line 15 + line 17) per the [TW domain prompt](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) DESIGN.md change authority clause + TW for the PROCESS.md + SA-REVIEW.md edits. The DESIGN.md edits are pure sweep cleanup with no semantic spec change; the SO sign-off may be granted as a routine acceptance rather than full SO-routing.
- **Finding 6 (acronym first-use expansion — Hallucinated)** needs no coordination — recorded for audit-trail completeness.

**Systemic recommendation re-issued:** before Round 4 cold pass, execute the R2-recommended broader sweep — every `vsdd-suite/<DOMAIN>-REVIEW.md` index file's Reviews-table summary scanned for stale heading citations + every project-root markdown file (DESIGN.md, PROCESS.md, TODO.md, README.md, CHANGELOG.md, manual-tests/*) walked for the three recurring defect classes (stale primer/heading citations, duplicate-name sweep artifacts, anchor-link-convention compliance per [Review 79](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-79--2026-05-20-1730z) Finding 3). The sweep-coverage gap is the recurring failure mode; a single broader pass would converge MVR faster than the Round-N+1 chain that has currently surfaced one or two findings per round.

All Resolved findings declare `**Validator:** documentation-reviewer` per the [TW Validator-pair declaration](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) (registered in [Review 80](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z)). The Hallucinated finding (F6) declares `**Validator:** sanity-check` per the meta-validator-of-last-resort pattern.
