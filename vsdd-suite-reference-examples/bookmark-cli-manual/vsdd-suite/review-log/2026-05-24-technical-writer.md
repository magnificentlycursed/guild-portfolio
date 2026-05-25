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
