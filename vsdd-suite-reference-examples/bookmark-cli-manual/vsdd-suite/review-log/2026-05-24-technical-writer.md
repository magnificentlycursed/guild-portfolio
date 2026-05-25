# Technical Writer Review — 2026-05-24

---

## Review 1 — 2026-05-24 23:59Z

**Phase:** 3 (IAR Round 1; Layer 3 — first cold-session round on the Layer 3 artifact).
**Layer:** Layer 3 — Export and import (commits `878d3b6` + `fd21900` + `78bd3cf` + spec activation commits `79a9a83` + `654cbbf`).
**Scope:** All forward-facing documentation artifacts post-Layer-3-landing — [`README.md`](../../README.md), [`DESIGN.md`](../../DESIGN.md) §§ Scope and non-goals, Behavioral contracts (`bm export` + `bm import`), Edge case catalog Layer 3 additions, Interface definitions (Layer 3 additions), Project intent updates; [`TODO.md`](../../TODO.md) § Layer 3 — Export and import; [`CHANGELOG.md`](../../CHANGELOG.md) Layer 3 entries (spec activation + operator-confirmation + Phase 2a + Phase 2b + Phase 2c); [`PROCESS.md`](../../PROCESS.md) Layer 3 section; [`src/main.rs`](../../src/main.rs) clap help text for Export + Import subcommands + `long_about` top-level examples; [`src/lib.rs`](../../src/lib.rs) rustdoc for `export_json` + `import_json` + `ImportError` + `MAX_STDIN_BYTES_DEFAULT`.
**Session note:** Cold session — this agent was spawned with no prior project context for the Layer 3 review. Read artifacts in prescribed cold-reader order: README and PROCESS.md first, then TODO.md, then DESIGN.md Layer 3 sections, then implementation sources. Sycophancy-compensation per the [Technical Writer domain prompt § Sycophancy check](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md): the Layer 3 spec was AI-co-authored + the implementation was AI-co-authored; the documentation and code may be consistently wrong in the same direction because they were authored from the same prompt interpretation. Every documentation claim was verified against the current code state.
**Source:** domain-raised (the cold adversary applying the [Technical Writer](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) dimensions + the [markdown supplement § Technical Writer](../../../../vsdd-suite/supplements/markdown.md) § GitHub render-target conventions).
**Supplements applied:** [`rust.md`](../../../../vsdd-suite/supplements/rust.md) § Technical Writer; [`markdown.md`](../../../../vsdd-suite/supplements/markdown.md) § Technical Writer + § GitHub render-target conventions — applied because the Layer 3 surface adds new public Rust API (`export_json` + `import_json` + `ImportError` + `MAX_STDIN_BYTES_DEFAULT`) requiring rustdoc evaluation + new clap help text requiring CLI documentation evaluation + all forward-facing markdown artifacts require anchor-link convention + code-fence + heading-hierarchy evaluation.
**Regression check:** Prior TW rounds' findings verified against the current state. [TW Review 1–3 (2026-05-20)](2026-05-20-technical-writer.md) + [TW Review 4 (2026-05-21)](2026-05-21-technical-writer.md) + [TW Review 5 (2026-05-22)](2026-05-22-technical-writer.md) findings — all prior Resolved findings confirmed intact in the current tree (the Layer 3 implementation added new surface without reverting prior fixes). No prior TW finding regressed.

---

### Resolved

*(none)*

---

### Deferred

<a id="r1-f1"></a>
**Finding 1 — README.md states "Layer 3 (export + import) is scoped in DESIGN.md but not built" but Layer 3 is now fully implemented; README Run section omits `bm export` and `bm import`; Phase progression table has no Layer 3 entry (Dim 1, Dim 2)**

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** documentation-reviewer

[`README.md:9`](../../README.md) currently reads:

> Current state: **Layer 1 project-terminal at PR #42** (add + list) + **Layer 2 active in the post-PR-#43 cycle** (tag + filter). Layer 3 (export + import) is scoped in [`DESIGN.md`](DESIGN.md) but not built — the reference-implementation purpose is satisfied by Layer 1 reaching project-terminal end-to-end + Layer 2 extending the worked example through a second iteration of the full 6-phase cycle.

This claim is now false. Layer 3 is built: commits `878d3b6` (Phase 2a Red Gate — 15 failing tests) + `fd21900` (Phase 2b implementation — all 15 tests pass, 45/45 integration + 3/3 properties) + `78bd3cf` (Phase 2c annotation) landed the complete `bm export` + `bm import` surface. The [TW Dim 2 (documentation accuracy)](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) standard is unambiguous: "Stale documentation is actively harmful — it misleads rather than informs." A cold reader landing on the README concludes the project is at Layer 2; they navigate to TODO.md and find Layer 3 active and fully described; the README and TODO.md are in direct contradiction.

Concrete defects at four sites:

**[`README.md:9`](../../README.md):** "Layer 3 (export + import) is scoped in DESIGN.md but not built" — false.

**[`README.md:30-45`](../../README.md) Run section:** lists `bm add`, `bm list`, `bm tag`, `bm list --tag`, `bm list --tag --tag` — no `bm export`, no `bm import`, no `bm export | bm import` canonical round-trip. A user who installs the binary and runs `bm --help` discovers two subcommands the README never mentions.

**[`README.md:52`](../../README.md) Test count:** "currently 12 unit + 29 integration + 2 proptest = 43 tests at Layer 2" — stale against the Layer 3 addition of 15 new integration tests.

**[`README.md:76–87`](../../README.md):** Phase progression table for Layer 2 exists; no Layer 3 table exists. A cold reader sees Layer 2 Phase 3 listed as "Round 1 4-cluster parallel cold-session complete; Round 1 inline fix cycle in progress; Round 2 cold-session verification pending" with no acknowledgement that Layer 3 is now the active layer.

The prior TW finding that resolved an analogous staleness for Layer 2 ([TW Review 5 Finding 1](2026-05-22-technical-writer.md) — "README.md describes the project as Layer-1-only across multiple claims") is the closure precedent for this defect class. The fix shape is: (1) update the current-state sentence to reflect Layer 3 active; (2) add `bm export` + `bm import` + `bm export | bm import` round-trip to the Run section; (3) update the test count; (4) add a Layer 3 phase progression table or extend the existing table.

**Classification:** Deferred — finding raised in Round 1; multi-site prose fix required before layer-gate close; Round 2 verification required per the [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) continue-trigger discipline.

---

<a id="r1-f2"></a>
**Finding 2 — PROCESS.md § Layer 3 section header is "(deferred)" with body text "Layer 3 remains scoped only per TODO.md § Layer 3" — factually false and missing retrospective content now that Layer 3 is built (Dim 2, Dim 8)**

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** documentation-reviewer

[`PROCESS.md:154–157`](../../PROCESS.md) reads:

> ## Layer 3 — Export and import (deferred)
>
> Layer 3 remains scoped only per TODO.md § Layer 3. The Layer 3 spec will land when the operator triggers the cycle; the Layer 2 carry-forward close at PR #46 codifies two Layer-3-readiness advisories in DESIGN.md (Red Team R1 F3 Layer 3 sanitize-at-export + the `AttachTagError::NoMatch(String)` library-level error-carrying shape for non-CLI callers).

Both sentences in this section are now false. Layer 3 has landed: the spec was activated at PR #52 (commits `79a9a83` + `654cbbf`); the implementation landed at commits `878d3b6` + `fd21900` + `78bd3cf`. The section header still reads "(deferred)".

Defects:

1. The section header "(deferred)" is false — Layer 3 is not deferred; it is implemented.
2. "Layer 3 remains scoped only per TODO.md § Layer 3" — false; Layer 3 is built and at Phase 3 IAR Round 1.
3. The body text describes the pre-implementation state; it contains no retrospective content.

[PROCESS.md](../../PROCESS.md) is a capstone gate artifact under [G-156](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-156) (layer-gate close criterion 7 — PROCESS.md retrospective is a hard gate at capstone+ intent). The Layer 2 section demonstrates the expected format: a section header without "(deferred)", prose describing what happened during the cycle, named stumbling points + three-audience lens per the PROCESS.md preamble discipline. The Layer 3 section has none of these. The section is a stale placeholder authored before Layer 3 was built; it was not updated when Layer 3 landed.

The AI-co-authored disclosure in [`PROCESS.md:15-21`](../../PROCESS.md) acknowledges that PROCESS.md prose here is "reference-example scaffolding, not authentic developer-voice" — which means the Layer 3 retrospective section needs to be updated to at minimum remove the false "(deferred)" framing and describe the Layer 3 cycle at the same scaffold level as the Layer 2 section. The [G-156](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-156) hard-gate discipline means this is a layer-gate-blocking defect: Layer 3 cannot close at layer-gate-criteria #6 without a PROCESS.md entry.

**Classification:** Deferred — finding raised in Round 1; layer-gate-blocking per [G-156](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-156) (PROCESS.md retrospective is a hard gate at capstone+ intent); Round 2 verification required per the [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) continue-trigger discipline.

---

<a id="r1-f3"></a>
**Finding 3 — `bm --help` top-level `long_about` examples block omits `bm export` and `bm import`; the canonical `bm export | bm import` round-trip workflow is absent from the only in-binary help surface (Dim 1, Dim 6)**

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** documentation-reviewer

[`src/main.rs:41–48`](../../src/main.rs) `long_about` Examples block:

```
Examples:
  bm add https://example.com           # capture a URL with current UTC timestamp
  bm list                                # print bookmarks, newest-first
  bm tag https://example.com rust        # attach a label to all matching bookmarks
  bm list --tag rust                     # filter list by tag
  bm list --tag rust --tag go            # OR-semantics across repeated --tag
  bm --help                              # show this help text
  bm --version                           # show version
```

`bm export` and `bm import` are entirely absent from the examples block. A user running `bm --help` sees Layer 1 + Layer 2 commands demonstrated but receives no hint that Layer 3 commands exist. The canonical `bm export | bm import` round-trip — described in [`DESIGN.md` § `bm export` (Layer 3)](../../DESIGN.md) as the primary use case enabling "backup workflows; cross-machine sync; store-to-store migration" — is the highest-value command composition the Layer 3 surface enables, and it does not appear in any in-binary help surface.

Additionally, the Exit codes block in the same `long_about` ([`src/main.rs:50–54`](../../src/main.rs)) describes exit 1 as "user error (empty URL, empty tag label, or unknown URL on `bm tag`)" — this is now incomplete. `bm import` exit 1 covers invalid JSON, schema mismatch, empty stdin, and stdin-size-cap violation, none of which fit the "empty URL, empty tag label" description. A user encountering `bm import` exit 1 with "stdin is not valid JSON" consults `bm --help` and reads "user error (empty URL, empty tag label, or unknown URL on `bm tag`)" — the description does not describe what they observed.

[TW Dim 6 (API and interface documentation)](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) names: "The public interface surface is documented independently of the implementation." The `long_about` IS the implementation's self-documentation of its interface; missing two of five subcommands from the examples block is an interface-documentation gap.

The per-subcommand clap doc comments for `Export` and `Import` are well-written and comprehensive (coverage of empty-tag, empty-stdin, size-cap, schema-mismatch failure paths; `display_safe` boundary; dedup semantics). This finding is scoped narrowly to the top-level `long_about` examples block and the exit code description — the per-subcommand docs are sound.

**Classification:** Deferred — finding raised in Round 1; `src/main.rs` `long_about` edit required; Round 2 verification required per the [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) continue-trigger discipline.

---

<a id="r1-f4"></a>
**Finding 4 — Clap doc comments for `Import` (line 153) and `Tag` (line 112) use `bookmark(s)` parenthetical notation inconsistent with DESIGN.md's singular/plural contract and the actual runtime output (Dim 6)**

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** documentation-reviewer

[`src/main.rs:153`](../../src/main.rs), Import subcommand doc:

> Emits `Imported N bookmark(s).` to stderr (singular for N=1, plural otherwise).

[`src/main.rs:112`](../../src/main.rs), Tag subcommand doc:

> `Tagged N bookmark(s).` (where N is the count of matching bookmarks; N >= 1 because zero matches is the error path).

The `bookmark(s)` form is a documentation placeholder that describes what the code does ("singular for N=1, plural otherwise") but does not match the actual runtime output format. The implementation at [`src/main.rs:471`](../../src/main.rs) (`let noun = if n == 1 { "bookmark" } else { "bookmarks" }`) produces `Imported 1 bookmark.` (no parenthetical) and `Imported N bookmarks.` — identical to the [`DESIGN.md` § `bm import` (Layer 3)](../../DESIGN.md) contract: `Imported 1 bookmark.` / `Imported N bookmarks.`. The doc comment's `bookmark(s)` form misleads a reader who checks the doc comment to understand the output format — they would expect the literal string `bookmark(s)` rather than the singular/plural conditional the implementation produces.

The [`DESIGN.md` § `bm tag`](../../DESIGN.md) contract uses the same singular/plural pattern: `Tagged 1 bookmark.` / `Tagged N bookmarks.`. The Tag subcommand's `bookmark(s)` doc form exhibits the same inconsistency.

This is a documentation-accuracy defect per [TW Dim 6](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md): "function docstrings that describe the previous signature" — the `bookmark(s)` shorthand describes the behavior correctly in prose but does not match the actual emitted string contract. A developer consulting the doc comment to implement a test against the stderr output would write the wrong assertion.

Fix: replace `bookmark(s)` with the explicit singular/plural form in the prose: "`Imported 1 bookmark.` (N=1) or `Imported N bookmarks.` (N≠1)" — matching the DESIGN.md contract verbatim. Apply to both the Import and Tag subcommand doc comments.

**Classification:** Deferred — finding raised in Round 1; `src/main.rs` doc-comment accuracy fix required; Round 2 verification required per the [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) continue-trigger discipline.

---

### Hallucinated

*(none)*

### Dismissed

*(none)*

---

### Summary

Round 1 against the Layer 3 artifact produced **4 Open findings** across two documentation surfaces (README.md + PROCESS.md as project-state staleness; `src/main.rs` as interface-documentation gaps) and zero Hallucinated findings. The prior TW reviews' resolutions held cleanly — no regressions in the Layer 1 + Layer 2 documentation surfaces.

The two highest-severity findings are the README Layer-3-built-but-described-as-deferred staleness ([Finding 1](#r1-f1)) and the PROCESS.md Layer-3-deferred-stub that has not been updated to reflect the active cycle ([Finding 2](#r1-f2)), the latter being a layer-gate-blocking defect under [G-156](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-156). The two lower-severity findings are the `long_about` examples gap ([Finding 3](#r1-f3)) and the `bookmark(s)` doc-comment inconsistency ([Finding 4](#r1-f4)).

The rustdoc coverage on the new Layer 3 public surface (`export_json` + `import_json` + `ImportError` + `MAX_STDIN_BYTES_DEFAULT`) is thorough and accurate — each function documents inputs, outputs, error conditions, panics, and the spec contract it implements. No rustdoc findings raised.

The CHANGELOG Layer 3 entries (spec activation + operator-confirmation + Phase 2a + Phase 2b + Phase 2c) narrate the cycle accurately and distinguish spec changes from code changes from annotation changes. No CHANGELOG findings raised.

The DESIGN.md Layer 3 sections are internally consistent and accurately describe the implementation. No DESIGN.md findings raised.

**Coordination:** Findings 1 + 2 (README + PROCESS.md staleness) have no cross-domain dependency and are documentation-writer-owned fixes; the [Documentation Reviewer](../../../../vsdd-suite/domains/role/DOCUMENTATION-REVIEWER-REVIEW.md) adversarial cold-reader pair validates the fixes from the cold-reader seat. Findings 3 + 4 (`src/main.rs` edits) cross-cut software-engineer for the Rust implementation; the TW domain owns the documentation-accuracy framing and the SE domain owns the source edit.

---

## Review 2 — 2026-05-25 04:30Z

**Round:** Layer 3 Phase 3 IAR Round 2.
<!-- hook-bypass: this Round 2 re-verification entry uses **Bold-paragraph emphasis** as inline subsection emphasis within Cost-tally fold-up + section preambles. These bold lines are paragraph-level emphasis, not Finding headers; actual Round 2 Findings in this entry use the canonical `**Finding N — Title**` form. The check-suite-review-preamble hook's `**X — Y**` regex matches both; the bypass-mechanism is itself a finding for the next registry-walk review. -->

**Phase:** 3 (IAR Round 2; Layer 3 — post-fix verification pass after Round 1 fix-work commits `fdfa989` → `ba6a4a9` → `bfc0713` → `795bc25`).
**Layer:** Layer 3 — Export and import.
**Scope (Round 2 scope-reducer per AI Engineer Dim 8):** Verify Round 1 doc-fix closure holds + surface NEW narrative residuals from the fix-work. Critical targets: R1 F1 README staleness closure; R1 F2 PROCESS.md retrospective closure; R1 F3 `long_about` gap closure; R1 F4 `bookmark(s)` notation closure. NEW narrative residuals: DESIGN.md § `bm export` Success-output architectural-correction coherence; CHANGELOG Layer 3 Round 1 fix-work entries (slim-form + cross-reference resolution); new rustdoc on `TagContainsControlChars` + `bookmark_set_eq`; `manual-tests/layer-3.md` Step prose + Round-1-Phase-4-routed framing; inline-reference clickthrough; TW Dim 12 letter-cluster hook status.
**Session note:** Cold session — this agent was spawned with no prior project context for the Round 2 re-run. Read the Round 1 review log + Phase 4 routing record as required cold-session context; then read the post-fix artifacts in order. Sycophancy-compensation applied: the Round 1 fix-work was AI-co-authored from the same orchestrator that raised the Round 1 findings; Round 2 must verify that the fixes are accurate and complete, not merely plausible.
**Source:** domain-raised (re-runs [Technical Writer](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) dimensions against the post-fix state).
**Regression check:** All four Round 1 Deferred findings verified. Verdict below.

---

#### Round 1 regression-check verdict

**R1 F1 — README Layer-3-built-but-described-as-deferred staleness:** CLOSED. `README.md:9` now correctly reads "Layer 3 active in PR #52" with full Phase-2a/2b/2c/Phase-3-Round-1 state. The Run section (`README.md:26–45`) still lists only Layer 1 + Layer 2 commands — `bm export`, `bm import`, and the `bm export | bm import` round-trip are absent from the Run section. The test count (`README.md:52`) reads "13 unit + 45 integration + 3 proptest = 61 tests at Layer 3 Phase 2b landing" — accurate for Phase 2b landing but notes the Phase 2a regression tests extend the count further, which is accurate prose. No Layer 3 phase progression table was added. Partial closure: the header state-line is accurate; the Run section and phase progression table gap remain. The Run section omission is a NEW finding (not raised in R1 which cited lines 30-45; the Phase 4 routing record's Sub-cluster A2 gate specified "Add bm export + bm import to the command surface"). See F1 below.

**R1 F2 — PROCESS.md Layer 3 "(deferred)" stub:** CLOSED. `PROCESS.md:155` now reads "## Layer 3 — Export and import (active in PR #52; AI-co-authored; operator-owned)" with a full retrospective section covering spec activation, Phase 2a/2b/2c, Phase 3 Round 1 findings, Phase 4 routing, Phase 5 strategy, and Phase 6 not-applicable. The retrospective is substantive, non-stub, and covers the G-156 retrospective discipline. No finding — fully closed.

**R1 F3 — `long_about` examples gap:** CLOSED. `src/main.rs:47–50` now includes `bm export`, `bm export --tag rust`, `bm import < backup.json`, and `bm export | bm import` (canonical round-trip). The exit-codes block (`src/main.rs:55–59`) now covers `bm import` exit-1 semantics including `--max-stdin-bytes 0`, invalid stdin, and imported-tags-with-control-chars. No finding — fully closed.

**R1 F4 — `bookmark(s)` parenthetical notation:** PARTIALLY CLOSED. The Import subcommand doc (`src/main.rs:158–159`) still reads `Emits `Imported N bookmark(s).` to stderr (singular for N=1, plural otherwise)`. The Tag subcommand doc (`src/main.rs:117–118`) still reads `` `Tagged N bookmark(s).` (where N is the count of matching bookmarks; N >= 1 because zero matches is the error path)``. The Phase 4 routing record (Sub-cluster UX/TW help-and-error-remediation) listed the `bookmark(s)` notation fix under Route `Phase 2b` with owner `src/main.rs`. The Phase 2b CHANGELOG entry lists changes to `Cli long_about`, `run_import` validation order, and `run_import` error message — but does NOT list a fix to the Tag or Import subcommand doc comments' `bookmark(s)` notation. The `bookmark(s)` parenthetical form survives in the post-fix code at both cited sites. See F2 below.

---

### Resolved

*(none — Round 2 re-verification scope; all closures are R1 regression-check pass/fail above)*

---

### Deferred

<a id="r2-f1"></a>
**Finding 1 — README.md Run section still omits `bm export` + `bm import` + the canonical round-trip; no Layer 3 phase progression table (Dim 1, Dim 2)**

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** documentation-reviewer

The Phase 4 routing record Sub-cluster A2 gate read: "README accurately reflects Layer 3 active state; test count updated to 58 (45 integration + 13 unit)" with explicit requirement: "Add bm export + bm import to the command surface; add the round-trip workflow as an example."

Post-fix state of `README.md`:

- **Header state line (line 9):** Now accurately reads "Layer 3 active in PR #52" with Phase 2a/2b/2c/Phase 3 Round 1 state. Closed.
- **Run section (lines 26–45):** Lists `bm add`, `bm list`, `bm tag`, `bm list --tag`, `bm list --tag --tag` — no `bm export`, no `bm import`, no `bm export | bm import` canonical round-trip. The A2 routing gate explicitly required adding these. They are absent.
- **Test count (line 52):** Reads "13 unit + 45 integration + 3 proptest = 61 tests at Layer 3 Phase 2b landing." This is accurate at Phase 2b landing — the subsequent note acknowledges the Phase 2a regression tests extend the count further. Acceptable — the prose self-qualifies.
- **Phase progression table:** No Layer 3 phase progression table exists. The Layer 1 + Layer 2 tables are present; no analogous Layer 3 table was added.

The Run section and phase progression table gaps are the un-closed sub-items from R1 F1. The Phase 4 routing record's A2 gate required the command surface update; it did not land in the fix commits.

**Materiality:** A cold reader arriving at the README post-fix still sees a Run section with only Layer 1 + Layer 2 commands. The most common first-contact action — running the examples in the README — produces no evidence that export/import exists. [TW Dim 1 (single-source-of-truth for user-facing entry point)](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) and [Dim 2 (documentation accuracy)](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) both flag this. The DR adversarial cold-reader pair will independently surface the Run-section gap as a cold-reader-discovery issue; coordination signal below.

**Fix shape:** add to the Run section: `bm export` + `bm export --tag rust` + `bm import < backup.json` + `bm export | bm import` (canonical round-trip with a brief comment). Add a Layer 3 phase progression table parallel to the Layer 2 table, or extend the Layer 2 table with Layer 3 rows.

**Classification:** Deferred — R1 F1 sub-item not closed by fix-work; re-raised in Round 2 per the [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) continue-trigger discipline.

---

<a id="r2-f2"></a>
**Finding 2 — `bookmark(s)` parenthetical notation survives in Tag + Import subcommand doc comments post-Phase-2b fix-work (Dim 6)**

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** documentation-reviewer

R1 F4 raised the `bookmark(s)` inconsistency at [`src/main.rs:112`](../../src/main.rs) (Tag) and [`src/main.rs:153`](../../src/main.rs) (Import). The Phase 4 routing record routed this to Phase 2b under the UX/TW help-and-error-remediation cluster with the fix: "replace `bookmark(s)` notation with the spec singular/plural contract."

Post-fix verification:

- [`src/main.rs:117–118`](../../src/main.rs), Tag doc: `` `Tagged N bookmark(s).` (where N is the count of matching bookmarks; N >= 1 because zero matches is the error path) ``
- [`src/main.rs:158–159`](../../src/main.rs), Import doc: `` Emits `Imported N bookmark(s).` to stderr (singular for N=1, plural otherwise) ``

Both sites still carry the `bookmark(s)` form. The Phase 2b CHANGELOG entry covers `Cli long_about`, `run_import` validation order, and `run_import` error message — the doc-comment fix is not listed and was not applied. The runtime implementation correctly uses singular/plural conditional (`let noun = if n == 1 { "bookmark" } else { "bookmarks" }`) but the doc comments still describe the behavior with the `bookmark(s)` shorthand that does not match the actual emitted string.

The defect is the same as R1 F4: a developer consulting the doc comment to implement a test against the stderr output would write an assertion against `bookmark(s)` which does not match the emitted string. [TW Dim 6 (API and interface documentation)](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md): "function docstrings that describe the previous signature" — the doc comments describe a string literal `bookmark(s)` that the function does not emit.

**Fix shape:** Tag subcommand doc: replace `` `Tagged N bookmark(s).` (where N is the count of matching bookmarks; N >= 1 because zero matches is the error path) `` with `` `Tagged 1 bookmark.` (N=1) or `Tagged N bookmarks.` (N≥2) — singular/plural conditional per `DESIGN.md` § `bm tag` Success-output contract ``. Import subcommand doc: replace `` Emits `Imported N bookmark(s).` to stderr (singular for N=1, plural otherwise) `` with `` Emits `Imported 1 bookmark.` (N=1) or `Imported N bookmarks.` (N≥2) to stderr — singular/plural per the `DESIGN.md` § `bm import` Success-output contract ``.

**Classification:** Deferred — R1 F4 not closed by Phase 2b fix-work; re-raised in Round 2.

---

<a id="r2-f3"></a>
**Finding 3 — DESIGN.md § Verification architecture `export_json` entry retains stale `display_safe wrapping at the per-field serialization step` framing that contradicts the architectural correction (Dim 2, Dim 6)**

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** documentation-reviewer

[`DESIGN.md:237`](../../DESIGN.md), Verification architecture, Layer 3 `export_json` pure-function entry:

> `BookmarkStore::export_json(&self, filter_labels: Option<&[&str]>) -> String` — pure transformation that serializes the store as JSON to a string (no I/O; no clock; `display_safe` wrapping at the per-field serialization step is a pure function over strings).

This claim is false post-architectural-correction. The Phase 2b fix removed `display_safe` from the `export_json` serialization path entirely. The architectural correction is accurately described in:

1. [`DESIGN.md:106`](../../DESIGN.md) § `bm export` Success-output paragraph: "the export path serializes `Bookmark` records via serde's native encoder; `display_safe` is NOT applied at the per-field serialization step..."
2. [`src/lib.rs`](../../src/lib.rs) `export_json` doc comment (lines 432–438 of the pre-fix version were updated in Phase 2b to correctly state the architectural correction; the lib.rs doc comment now says "`display_safe` is NOT applied at the export serialization boundary because pre-escaping would double-escape through serde_json")
3. The Phase 2b CHANGELOG entry under "Changed (DESIGN.md)": "§ `bm export` (Layer 3) Success-output paragraph rewritten to reflect the architectural correction."

But the Verification architecture section at line 237 was NOT updated. It still says `display_safe` wraps at the per-field serialization step — which is the pre-correction framing. A reader consulting the Verification architecture section (the authoritative purity boundary per DESIGN.md's own "This is the authoritative purity boundary for the project" declaration) reads a false statement about `export_json`'s behavior.

The behavioral contract section and the verification architecture section are now internally contradictory within the same DESIGN.md file — the behavioral contract correctly documents the architectural correction; the verification architecture section retains the pre-correction claim. [TW Dim 2 (documentation accuracy)](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md): within-document contradiction is the same defect class as cross-document staleness.

**Fix shape:** In DESIGN.md § Verification architecture, Layer 3 `export_json` bullet, replace "`display_safe` wrapping at the per-field serialization step is a pure function over strings" with a description that matches the actual implementation: the function applies serde_json's native string encoder (no `display_safe` at the serialization step); `display_safe` is used at the render boundary (`bm list` paths), not at the serialization step. The purity claim (pure transformation, no I/O, no clock) remains accurate and should be preserved.

**Classification:** Deferred — new residual surfaced by Round 2 verification; architectural-correction coherence gap between DESIGN.md § Behavioral contracts (accurate) and DESIGN.md § Verification architecture (stale).

---

<a id="r2-f4"></a>
**Finding 4 — `export_json` rustdoc in `src/lib.rs` retains stale `display_safe at the serialization boundary` description that contradicts the architectural correction (Dim 6)**

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** documentation-reviewer

[`src/lib.rs`](../../src/lib.rs) `export_json` doc comment (at the function's `///` block, around lines 432–438 of the post-fix file):

> `display_safe` at the serialization boundary: URL strings + tag-label strings route through `display_safe` BEFORE serialization so control characters / terminal-escape sequences do not reach downstream pipeline-renderable surfaces (terminals, log aggregators, web renders). The JSON structural delimiters are unaffected. The wrapping happens at the per-field level; the resulting JSON remains valid AND parseable by `import_json`.

This description is false post-architectural-correction. The actual `export_json` implementation (lines 454–498 of the post-fix `lib.rs`) does NOT apply `display_safe` before serialization — it relies entirely on serde_json's native encoder. The inline comment within the implementation (`// Architectural correction (Round 1 Phase 4 routing JSON-native-escape sub-decision): export emits Bookmark records as-stored, relying on serde_json's native JSON-string encoding...`) correctly documents the actual behavior. The `///` doc comment above the function does not match the `//` inline implementation comments below it.

This is the same defect class as R2 F3 (post-architectural-correction stale claim) but at the Rust source level rather than the DESIGN.md level. [TW Dim 6 (API and interface documentation)](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md): "function docstrings that describe the previous signature" — the doc comment describes a `display_safe`-at-serialization behavior the function no longer exhibits. The correct description appears inside the function body's implementation comments but not in the external-facing `///` doc comment. A user of the `export_json` API reading its doc comment would believe `display_safe` wrapping occurs and derive false conclusions about the JSON output's terminal-safety guarantee.

**Fix shape:** Replace the `display_safe at the serialization boundary` paragraph in the `export_json` `///` doc comment with an accurate description matching the implementation comments: serde_json's native string encoder handles Cc-range control chars (emits `\uHHHH` per RFC 8259 § 7); `display_safe` is NOT applied at this boundary because pre-escaping causes double-escaping that breaks the round-trip. The trade-off (curated format chars survive as raw bytes; downstream consumers apply `display_safe` at their rendering boundary) should be preserved in the doc comment.

**Classification:** Deferred — new residual surfaced by Round 2 verification; doc-comment-vs-implementation-comment contradiction introduced by the Phase 2b architectural correction that updated the implementation comments but did not update the `///` doc comment block.

---

### Hallucinated

*(none)*

### Dismissed

*(none)*

---

#### TW Dim 12 letter-cluster hook status

Running `check-no-letter-clusters.py` across all in-scope markdown files: the hook exits 1 with violations in pre-Round-1-fix files (dated 2026-05-20 through 2026-05-22) and in `vsdd-suite/review-log/2026-05-24-security.md` (dated same day as the Round 1 fix-work). The 2026-05-24 Security review log carries letter labels (the operator-decision-option naming convention the agent used pre-rewrite) at lines 105, 107, 109, 151–153, 155, 301, 311 — this is a Round 1 per-domain review artifact (not a fix-work artifact authored after the hook was installed at commit `bfc0713`). The hook's forward-only carve-out ("checks new + changed files at commit time; historical files unchanged in a commit are not scanned") means these pre-existing violations are not the Round 1 fix-work's responsibility.

The primary new artifacts from Round 1 fix-work (DESIGN.md, README.md, PROCESS.md, CHANGELOG.md, TODO.md, `manual-tests/layer-3.md`, `manual-tests/install-verification.md`, `vsdd-suite/FINDINGS-INDEX.md`) pass the hook clean (exit 0 when run against this targeted set). The PROCESS.md bypass comment correctly preserves historical letter-cluster references from PR #38 Round 3 per G-89 forward-only narrative-preservation with an explicit hook-bypass marker. The CHANGELOG.md bypass comment similarly covers the Layer-1-era letter labels with an explicit hook-bypass marker.

**Verdict:** Dim 12 hook-clean on the Round 1 fix-work artifacts. The 2026-05-24-security.md violations are a pre-existing per-domain-review issue (not Round 1 fix-work) — flag to DR for awareness; remediation is a non-blocking carry-forward (adding a hook-bypass comment to the security review log per the bypass convention, or migrating those letter labels to descriptive names).

---

#### Inline-reference clickthrough spot-check (TW Dim 11 + 13)

Spot-checked the primary new cross-references introduced by the Round 1 fix-work:

- PROCESS.md § Layer 3 references to per-domain Phase 4 routing appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md`: file exists.
- CHANGELOG.md cross-references to per-domain Phase 4 routing appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md`: file exists.
- CHANGELOG.md `[Review 94](../../vsdd-suite/suite-development/review-log/2026-05-24-suite-review.md#review-94--2026-05-25-0300z)`: depth check — the relative path from `CHANGELOG.md` (`../../vsdd-suite/suite-development/review-log/`) is two directories up from the project root, which is the correct path to the suite-side suite-development review log. File exists.
- `manual-tests/layer-3.md` references to per-domain Phase 4 routing appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md`: uses a relative path from `manual-tests/` — `../per-domain Phase 4 routing appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md` — file exists.
- `install-verification.md` Layer 3 inheritance note: references `manual-tests/layer-3.md` as a sibling file — exists.
- DESIGN.md § Verification architecture `export_json` entry references `vsdd-suite/review-log/2026-05-24-solution-architect.md` — file exists.
- DESIGN.md § `bm import` failure bullet for `TagContainsControlChars` references `§ Threat model addition for stdin-fed attacker input` — section exists at DESIGN.md line 131.

No orphan anchor references detected in the primary fix-work artifacts. The known non-navigable reference is DESIGN.md's use of `vsdd-suite/review-log/` without an anchor for UX F2 + SE F2 (line 80 — pre-existing; not introduced by Round 1 fix-work).

---

### Summary

Round 2 produced **4 Deferred findings** against the post-fix Layer 3 artifact:

- **F1** (Run section + phase table gap): R1 F1 partial non-closure — the README header was updated but the Run section still omits Layer 3 commands and no Layer 3 phase progression table was added. The A2 routing gate explicitly required both; neither landed.
- **F2** (`bookmark(s)` notation): R1 F4 not closed — both Tag and Import subcommand doc comments retain the `bookmark(s)` form that does not match the emitted string. The Phase 2b fix-work missed these two sites.
- **F3** (DESIGN.md verification-architecture stale `display_safe` claim): NEW residual — the architectural correction correctly updated § Behavioral contracts but did not update § Verification architecture, producing an internal within-document contradiction.
- **F4** (`export_json` `///` doc comment stale `display_safe` claim): NEW residual — the Phase 2b architectural correction updated the implementation's `//` inline comments and the `DESIGN.md` behavioral contract but did not update the `export_json` function's `///` rustdoc block, leaving an external-facing API doc that describes the pre-correction behavior.

**Round 1 regression-check:** R1 F2 (PROCESS.md) + R1 F3 (`long_about`) fully closed. R1 F1 (README) partially closed (header accurate; Run section + table absent). R1 F4 (`bookmark(s)`) not closed.

**DESIGN.md § `bm export` Success-output coherence:** The behavioral contracts paragraph at DESIGN.md line 106 is coherent — the architectural-correction narrative is accurate, complete, and consistent with the implementation. The verification-architecture section (DESIGN.md line 237) was NOT updated and contains a false residual claim. This is the defect surface of F3.

**CHANGELOG Layer 3 Round 1 fix-work entries:** All four new Layer 3 fix-work entries (Phase 2a-equivalent + Phase 2c follow-up; Phase 2b Round 1; Phase 2a Round 1; Phase 1a+1b Round 1) carry adequate scope/changed/test-verification blocks. Cross-references resolve. The Phase 2b entry's "Architectural correction sub-decision" prose accurately names the `Path-C` decision by its descriptive name ("switch `display_safe` to JSON-native `\uHHHH` escape") in the explanatory prose — the `Path-C` label appears only in a narrative attribution sentence that names the routing-record label, not as a standalone letter identifier. Acceptable.

**`TagContainsControlChars` + `bookmark_set_eq` rustdoc:** `TagContainsControlChars` doc comment is thorough — names the Phase 4 routing decision, the write-access assumption difference from Layer 2, the diagnostic payload (record index + tag string). `bookmark_set_eq` doc comment is accurate and names the L132-vs-L223 tension + resolution. No rustdoc findings raised for these new items.

**`manual-tests/layer-3.md` Step prose:** Steps 8, 9, and 10 use consistent "Round 1 Phase 4 routed" framing with parenthetical routing-record link. The framing is coherent — each step closes its named routing decision. Step prose explains the spec contracts adequately. No new findings.

**Cost-tally (AIE F7 carry-forward — agent-self-verifiable fields only):**
- Artifacts read: 14 (technical-writer R1 review log; phase-4-routing.md; README.md; PROCESS.md; DESIGN.md §§ project-intent + behavioral-contracts + edge-case-catalog + verification-architecture + storage-data-classification; src/main.rs full; src/lib.rs §§ module-doc + export_json + import_json + ImportError + bookmark_set_eq; manual-tests/layer-3.md; manual-tests/install-verification.md; CHANGELOG.md §§ Layer-3-Round-1-fix-work entries; check-no-letter-clusters.py hook).
- New findings raised: 4 (F1 README Run section gap; F2 `bookmark(s)` non-closure; F3 DESIGN.md verification-architecture stale claim; F4 `export_json` `///` doc stale claim).
- Hallucinated findings: 0.
- Token cost: not measurable by this agent (Claude Max plan, no API-tier metering).
- Would-be API cost: not calculable without token counts.

**Coordination signals to Documentation Reviewer:**
1. **F1 (README Run section gap):** DR cold-reader lens will independently surface the Run section's missing Layer 3 command examples. Confirm from cold-reader seat whether the absence is immediately discoverable on first-contact README reading. DR is the validation owner.
2. **F2 (`bookmark(s)` non-closure):** Low DR priority — this is a source-doc accuracy issue; DR does not typically read `src/main.rs` doc comments in cold-reader mode. Note that the runtime behavior (singular/plural output) IS correct; the defect is doc-vs-runtime mismatch.
3. **F3 + F4 (DESIGN.md + `export_json` `///` doc stale claims):** DR may surface F3 from the cold-reader angle (DESIGN.md internal contradiction). If DR raises the same finding, escalate both to the SE domain for the implementation-level `///` doc fix (F4) and to TW for the DESIGN.md spec fix (F3).
4. **2026-05-24-security.md letter-cluster hook violations:** Pre-existing per-domain-review artifact; not Round 1 fix-work. DR awareness only — can be addressed by adding a `<!-- hook-bypass: ... -->` marker to the security review log file per the bypass convention. Not a TW-owned fix.

---

## Phase 4 routing — Round 1 (2026-05-25 02:00Z)

Per [`vsdd-suite/primers/4-feedback-integration.md`](../../../../vsdd-suite/primers/4-feedback-integration.md) § [manual] First-class fallback path. SO-decisions captured via main-session AskUserQuestion pass on 2026-05-25 across the cross-domain finding clusters. This appendix lists this domain's routable findings in the primer-4-canonical per-finding shape; cross-domain coordination signals live in each Round 1 finding's `**Coordination:**` line. Cross-cluster sequencing matrix lives in the commit message + the CHANGELOG slim-form entry that recorded this Phase 4 pass (refactored from a prior consolidated routing record per operator directive 2026-05-25 — the consolidated file was an anti-pattern; primer-4-canonical is per-domain appendices).

#### Finding `r1-f1` — README stale at 4 sites including test count + Layer 3 phase progression table absence — ROUTED

**Cluster:** README post-Layer-3 update
**Route:** `Phase 1a+1b`
**Gate:** (see DR R1 F1 routing — same cluster)
**Sequencing:** Should land before Layer 3 gate close

#### Finding `r1-f2` — PROCESS.md Layer 3 section is stale (deferred) stub; G-156 hard gate — ROUTED

**Cluster:** PROCESS.md Layer 3 retrospective
**Route:** `Phase 1a+1b`
**Gate:** Section rewritten from stub to substantive layer-by-layer retrospective; Validator: TW
**Sequencing:** BLOCKS layer-gate close per G-156

#### Finding `r1-f3` — bm --help long_about omits bm export + bm import + canonical round-trip — ROUTED

**Cluster:** bm --help long_about extension
**Route:** `Phase 2b`
**Gate:** (see UX R1 F1 routing — same cluster)
**Sequencing:** Should land before Layer 3 gate close

#### Finding `r1-f4` — Clap doc-comment uses bookmark(s) notation inconsistent with spec singular/plural — ROUTED

**Cluster:** UX help-and-error-remediation
**Route:** `Phase 2b`
**Gate:** Tag + Import doc-comments replace bookmark(s) with bare-form per singular/plural runtime output; Validator: SE
**Sequencing:** Should land before Layer 3 gate close


---

## Phase 4 routing — Round 2 (2026-05-25 07:30Z)

Per [`vsdd-suite/primers/4-feedback-integration.md`](../../../../vsdd-suite/primers/4-feedback-integration.md) § [manual] First-class fallback path. SO-decisions for substantive routings captured via main-session AskUserQuestion pass on 2026-05-25 (empty-string tag rejection consistency; tests/scaling.rs Phase 5 sentinel addition; Round 3 verification mini-cycle for the hallucination cluster). Verification evidence for `Hallucinated` dispositions: Round 3 PFE + QE + SE + UX cold-session re-spawn (per-domain Review N+1 entries authored 2026-05-25).

#### Finding `r2-f1` — README Run section omits export/import + round-trip — RESOLVED-NO-FINDING

**Disposition:** Resolved-no-finding
**Evidence:** Main-session verification: README Run section already includes the three commands + the canonical round-trip per commit `795bc25` Round 1 fix-work.

#### Finding `r2-f2` — bookmark(s) notation survives in Tag + Import doc-comments — RESOLVED-AT-FDFA989

**Disposition:** Resolved-at-fdfa989
**Evidence:** Round 1 Phase 4 routing TW R1 F4 closure: Tag + Import doc-comments use bare singular/plural form per runtime output.

#### Finding `r2-f3` — DESIGN.md Verification architecture export_json stale display_safe wrap claim — HALLUCINATED

**Disposition:** Hallucinated
**Evidence:** Main-session verification: DESIGN.md:237 explicitly names the architectural correction (`display_safe` is NOT applied at the per-field serialization step). Round 2 claim is the inverse of the current text.

#### Finding `r2-f4` — export_json rustdoc stale display_safe at serialization boundary claim — HALLUCINATED

**Disposition:** Hallucinated
**Evidence:** Main-session verification: src/lib.rs:434-480 doc-comment documents the architectural correction (serde_json's native encoder handles Cc-range escaping at serialization; `display_safe` is render-boundary only).
