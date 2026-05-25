# Solution Owner Review — 2026-05-24

---

## Review 1 — 2026-05-25 01:12Z

**Phase:** [Phase 3](../../../../vsdd-suite/primers/3-review-session.md) — Iterative Adversarial Refinement (Layer 3 Round 1).
**Scope:** Layer 3 spec-vs-implementation compliance across the three-commit sequence `878d3b6` (Phase 2a Red Gate — 15 failing tests for `bm export` + `bm import`) + `fd21900` (Phase 2b — implementation; 45/45 + 3/3 + 0 clippy warnings) + `78bd3cf` (Phase 2c — extract-and-name annotation; no code changes). Spec authoring context: AI-co-authored first-draft at `79a9a83` + operator-confirmation pass at `654cbbf` (6 confirmed-at-default + 2 revised + 1 deferred-to-Phase-2b-verification). Layer 1 + Layer 2 regression-check baselines preserved.
**Session note:** Cold session opened against the post-commit-`78bd3cf` state. Did not author `DESIGN.md` § Layer 3 contracts, `TODO.md` § Layer 3 ACs, `src/lib.rs` `export_json` / `import_json` / `MAX_STDIN_BYTES_DEFAULT` / `ImportError` additions, `src/main.rs` `Cmd::Export` / `Cmd::Import` / `run_export` / `run_import` additions, or any Layer 3 Red Gate test. Reading order: [SO domain prompt](../../../../vsdd-suite/domains/role/SOLUTION-OWNER-REVIEW.md) → [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) → [`suite-development.md` § Per-review entry preamble + § Finding-header form + § Source attribution](../../../../vsdd-suite/suite-development/suite-development.md#per-review-entry-preamble-under-each--review-n--yyyy-mm-dd-hhmmz) → prior SO Reviews ([2026-05-20-solution-owner.md](2026-05-20-solution-owner.md) + [2026-05-21-solution-owner.md](2026-05-21-solution-owner.md) — R4 F2 Phase-6-not-applicable precedent + R5 F1 spec-honest-resolution precedent) → [`README.md`](../../README.md) + [`PROCESS.md`](../../PROCESS.md) for reference-example purpose declaration → [`TODO.md`](../../TODO.md) § Layer 3 (AC 14..AC 28 + Layer-gate criteria) → [`tests/bookmarks.rs`](../../tests/bookmarks.rs) Layer 3 Red Gate block (lines 1065-1689) → [`src/lib.rs`](../../src/lib.rs) (`export_json` + `import_json` + `MAX_STDIN_BYTES_DEFAULT` + `ImportError`) → [`src/main.rs`](../../src/main.rs) (`Cmd::Export` + `Cmd::Import` + `run_export` + `run_import`) → [`DESIGN.md`](../../DESIGN.md) § Layer 3 blocks read LAST per the cold-reader-poisoning discipline.
**Source:** domain-raised — cold-session adversarial reviewer applying the SO domain's nine dimensions plus the operator-supplied per-domain prompt's specific questions about scope discipline, intent calibration, spec-coverage-of-operator-decisions, accepted-limitation framing, layer-gate readiness, reference-example purpose alignment, and Phase 5 + Phase 6 strategy declarations for Layer 3.
**Lens:** Scope discipline (Dim 2 + Dim 6 + Dim 8); intent calibration (Dim 4 — capstone proportionality for Layer 3 stdin-fed attacker surface); spec coverage (Dim 1) of the 8 operator-confirmed decisions from the AI-co-authored first-draft; under-delivery (Dim 5) against TODO.md-named artifacts; reference-example purpose alignment ([G-112](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112)) of the AI-co-authored-disclosure shape at the spec; Phase 5 + Phase 6 strategy declarations against capstone-intent + [G-150](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-150) + [G-112](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112); Phase 6 not-applicable disposition consistency with [R4 F2](2026-05-21-solution-owner.md#r4-f2) precedent.
**Reference:** DESIGN.md is the spec contract; the AI-co-authored disclosure at DESIGN.md:47 + operator-ownership declaration are the upstream-assignment-equivalent surface this review evaluates compliance against (Dim 9 assignment-compliance — the operator's "I author first-draft; you edit + own" directive is the assignment).
**Regression-check against:** [Solution Owner Review 4 (2026-05-21)](2026-05-21-solution-owner.md#review-4--2026-05-21-2200z) (Layer 2 Round 1 SO MVR — established the R4 F1 spec-honest-resolution precedent + R4 F2 Phase-6-not-applicable precedent) + [Solution Owner Review 5 (2026-05-21)](2026-05-21-solution-owner.md#review-5--2026-05-22-1630z) (Layer 2 Round 2 SO MVR — verified F1 + F2 closures held under Option-1 resolution paths) + [Solution Owner Review 3 (2026-05-20)](2026-05-20-solution-owner.md#review-3--2026-05-20-2200z) (Layer 1 project-terminal SO MVR; still the regression floor for Layer 1 ACs 1-4).

**Compliance table** (Layer 3 ACs 14-28 vs. observable implementation behavior — regression-check floor for Layer 1 ACs 1-4 + Layer 2 ACs 5-13 implied via the 45/45 `cargo test --test bookmarks` GREEN verification at commit `fd21900`):

| AC | Spec (DESIGN.md / TODO.md) | Implementation | Status |
|---|---|---|---|
| AC 14 (export emits all bookmarks as storage-format JSON) | stdout `{"bookmarks":[...]}` newest-first; exit 0; stderr silent | `src/main.rs:393-416` `run_export` → `BookmarkStore::export_json` at `src/lib.rs:453-485`; `tests_export_emits_all_bookmarks_as_storage_format_json` (`tests/bookmarks.rs:1077-1126`) | Met |
| AC 15 (export against absent store emits `{"bookmarks":[]}\n`) | stdout empty-array shape + trailing newline; exit 0; stderr silent (no `No bookmarks yet.` message) | `src/lib.rs:474` `serde_json::json!({ "bookmarks": bookmarks_array })` + `:483` `s.push('\n')`; `tests_export_against_empty_store_emits_empty_bookmarks_array` (`:1131-1151`) asserts `ends_with('\n')` + `stderr empty` | Met |
| AC 16 (export --tag OR-union) | OR-filtered subset; filter-empty emits same `{"bookmarks":[]}\n` shape | `src/main.rs:405-409` filter construction + `src/lib.rs:454-456` `filter_labels.map_or_else(...)`; `tests_export_with_tag_filter_emits_or_union` (`:1156-1204`) | Met |
| AC 17 (export --tag "" rejected) | exit 1; stderr `Error: tag label cannot be empty.\n` | `src/main.rs:394-397` empty-string-in-tags screen; `tests_export_with_empty_tag_label_rejected` (`:1209-1229`) | Met |
| AC 18 (export applies `display_safe` at serialization step + remains JSON-valid) | URLs + tag-label strings routed through `display_safe` BEFORE serialization; emitted JSON remains valid + parseable; round-trip via `bm import` recovers underlying bytes | `src/lib.rs:464` `display_safe(t)` for tag elements + `:467` `display_safe(bm.url())` for URLs; `tests_export_applies_display_safe_to_pathological_url` (`:1236-1284`) asserts no raw ESC + JSON-valid + URL escape-clean; round-trip closure via AC 28 | Met |
| AC 19 (import appends valid payload; stderr `Imported N bookmark(s).`) | singular/plural per Layer 2 R2 UX F4; exit 0; stdout silent | `src/main.rs:471-472` `let noun = if n == 1 { "bookmark" } else { "bookmarks" }; eprintln!("Imported {n} {noun}.")`; `tests_import_appends_valid_payload_to_existing_store` (`:1290-1324`) | Met |
| AC 20 (import idempotent on exact-tuple-match) | second invocation `Imported 0 bookmarks.\n`; dedup against destination state AND within payload | `src/lib.rs:558-564` `for new_bm in imported { if !self.bookmarks.contains(&new_bm) { ... } }` — each push joins destination state for subsequent contains-checks; `tests_import_is_idempotent_on_exact_tuple_match` (`:1331-1363`) | Met |
| AC 21 (empty-bookmarks-array payload is no-op success) | stderr `Imported 0 bookmarks.\n`; exit 0; store byte-identical | `src/main.rs:465-470` save-skip-when-n==0 preserves byte state; `tests_import_empty_payload_is_no_op_success` (`:1369-1397`) asserts `pre_state == post_state` | Met |
| AC 22 (empty stdin rejected) | exit 1; stderr `Error: stdin is empty; nothing to import.\n`; no file write | `src/main.rs:439-442`; `tests_import_empty_stdin_rejected` (`:1402-1418`) asserts `!db.exists()` | Met |
| AC 23 (invalid JSON rejected) | exit 1; stderr `Error: stdin is not valid JSON.\n` + serde detail on next line; no file write | `src/main.rs:475-479` (`InvalidJson` arm) + `:443-449` (non-UTF-8 path); `tests_import_invalid_json_rejected` (`:1423-1441`) | Met |
| AC 24 (schema-mismatch rejected) | exit 1; stderr `Error: stdin JSON does not match storage-format schema; expected {"bookmarks": [...]}.\n` + offending-field detail; no file write | `src/lib.rs:525-532` top-level schema validation + `src/main.rs:480-486` (`SchemaMismatch` arm); `tests_import_schema_mismatch_rejected` (`:1448-1466`) | Met |
| AC 25 (import to Layer-1-format destination migrates forward) | post-write store carries explicit `tags: []` for untouched bookmarks + preserves imported tags | `src/lib.rs:54-55` `#[serde(default)] tags`; `tests_import_against_layer_1_format_destination_migrates_forward` (`:1474-1521`) | Met |
| AC 26 (partial-failure atomicity) | any record fails → entire import fails; store preserved | `src/lib.rs:538-550` validation completes BEFORE any mutation to `self.bookmarks`; `tests_import_partial_failure_preserves_existing_store` (`:1527-1561`) asserts `pre_state == post_state` after partial-failure | Met |
| AC 27 (stdin size cap enforced) | exit 1; stderr `Error: stdin exceeded maximum byte limit of <N>.\n`; no file write; default cap 10 MB; operator override via `--max-stdin-bytes <N>` | `src/main.rs:419-438` `take(cap+1)` discipline; `src/lib.rs:576` `MAX_STDIN_BYTES_DEFAULT`; `src/main.rs:167` clap default-value-t for the flag; `tests_import_stdin_size_cap_enforced` (`:1566-1594`) | Met |
| AC 28 (round-trip reproduces source store exactly) | `bm export | bm import` against fresh destination reproduces source bookmarks modulo timestamps-preserved-as-emitted | `tests_export_import_round_trip` (`:1602-1689`) asserts sorted-tuple equality | Met |

_Layer 1 + Layer 2 regression-check (the floor from Reviews 3 + 4 + 5):_ all Layer 1 ACs 1-4 + Layer 2 ACs 5-13 continue to hold — Phase 2b commit `fd21900` message documents `cargo test --test bookmarks → 45/45 pass` + `cargo test --test properties → 3/3 pass` + `cargo clippy --all-targets --all-features → zero warnings`. SO seat ran `cargo test --test bookmarks` in this session and confirmed 45/45 pass (the same number; the 13 Layer 2 tests + 4 Layer 1 tests + 13 Layer 1+2 tests inherited unchanged + 15 new Layer 3 tests = 45). The exit-code contract (0/1/2/64), atomic-save discipline, mode-0600, symlink-rejection, display_safe sanitizer, parent-directory fsync all preserved per spot-check of `src/lib.rs:175-323`. **Layer 1 + Layer 2 spec compliance: NO REGRESSION.**

**MVR signal:** **Round 1 — NOT REACHED.** Two real findings surface (one under-delivery against TODO.md:138 spec commitment for `manual-tests/layer-3.md`; one stale-citation regression at README.md:9 that still says "Layer 3 ... scoped but not built" against the post-Phase-2b state where Layer 3 has been implemented). Three scope-discipline observations resolve cleanly (the 15 ACs map cleanly to the operator-confirmed-decisions block; capstone-intent calibration holds against the new stdin-fed attacker surface without triggering production-intent escalation; the Phase 6 not-applicable disposition for Layer 3 is consistent with the R4 F2 precedent + correctly framed against [G-150](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-150) + [G-112](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112)). One specific question (does the AI-co-authored-disclosure shape preserve reference-example clarity?) is answered against the SO seat. Per [G-131](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-131) continue-trigger discipline, Round 2 is mandatory if any of the substantive findings open.

---

### Backlogged

**Finding 1 — Under-delivery: `manual-tests/layer-3.md` named by TODO.md:138 as Phase-2a-Red-Gate-companion is absent (Dim 5)**

<a id="r1-f1"></a>

**Owner:** solution-owner
**Status:** raised
**Blocked by:** *(none — directly observable artifact gap)*
**Validator:** vdd-iar-alignment

TODO.md:138 names the artifact explicitly:

> "**Layer 3 manual testing checklist:** `manual-tests/layer-3.md` (to be authored alongside the Phase 2a Red Gate commit) — parallel to `manual-tests/layer-{1,2}.md`. Includes the `bm export | bm import` round-trip canonical workflow + the cross-machine sync workflow via file-transfer-pipe."

TODO.md:150 Layer-gate criterion #3 requires "The manual testing checklist at `manual-tests/layer-3.md` runs clean (every step produces the expected exit/stdout/stderr)." But `manual-tests/` contains only `install-verification.md`, `layer-1.md`, `layer-2.md` — Phase 2a commit `878d3b6` did not author it, Phase 2b commit `fd21900` did not author it, Phase 2c commit `78bd3cf` did not author it.

**Evidence of absence:** `ls vsdd-suite-reference-examples/bookmark-cli-manual/manual-tests/` returns `install-verification.md`, `layer-1.md`, `layer-2.md` — three files; `layer-3.md` is not present.

**Why this is SO scope-discipline, not "merely future work":** TODO.md:138 commits the artifact to "the Phase 2a Red Gate commit" — the timing is spec-bound, not Phase-5-trigger. The Layer 3 cycle is mid-cycle (Phase 2a + Phase 2b + Phase 2c landed; Phase 3 IAR underway; Phase 5 hardening + Layer-gate close still ahead). Per the Layer 2 R4 F1 precedent, when DESIGN.md / TODO.md names an artifact + file path + the timing trigger ("to be authored alongside the Phase 2a Red Gate commit"), the artifact is a spec-contract closure commitment; absence at the commit-named-as-trigger is a Dim 5 under-delivery, not a deferred-future-work item. The Layer 2 closure pattern at PR #44 was Option 1 (author the missing artifact); the same option exists here.

**Disposition:** the SO seat does not have authority to dismiss this as scope reduction. Two resolution paths are spec-honest, parallel to Layer 2 R4 F1:

1. **Author `manual-tests/layer-3.md`** per the TODO.md:138 commitment — parallel to `layer-2.md`'s 13-step shape, including the `bm export | bm import` round-trip canonical workflow + cross-machine sync workflow. Layer-gate criterion #3 then clears cleanly.

2. **Amend TODO.md:138 + TODO.md:150** to defer `manual-tests/layer-3.md` to a later closure surface (e.g., Phase 5 closure, post-Phase-3 close) with explicit named rationale. This is the SO-authority path: TODO.md is the spec contract; if the closure commitment is being unwound, the unwinding must be visible at the spec surface, not silently elided.

The operator's choice between paths is preserved as the resolution path; the finding documents the gap.

**Classification:** Backlogged — Layer 3 acceptance gate (criterion #3) cannot close without resolution; operator-decision-required between Option 1 (author `manual-tests/layer-3.md`) and Option 2 (amend TODO.md to defer manual-test artifact further). Carries forward to Round 2; if Round 2 finds Option 1 applied, the finding closes as Resolved per the R5 F1 verification precedent.

---

**Finding 2 — Stale-citation regression: README.md:9 still says "Layer 3 (export + import) is scoped in DESIGN.md but not built" against the post-Phase-2b state where Layer 3 is implemented (Dim 7 — design fidelity / stale narrative)**

<a id="r1-f2"></a>

**Owner:** solution-owner
**Status:** raised
**Blocked by:** *(none — directly observable artifact gap)*
**Validator:** technical-writer

README.md:9 currently reads:

> "Current state: **Layer 1 project-terminal at PR #42** (add + list) + **Layer 2 active in the post-PR-#43 cycle** (tag + filter). Layer 3 (export + import) is scoped in [`DESIGN.md`](DESIGN.md) but not built — the reference-implementation purpose is satisfied by Layer 1 reaching project-terminal end-to-end + Layer 2 extending the worked example through a second iteration of the full 6-phase cycle."

This narrative is stale against the post-`78bd3cf` state. Layer 3 IS built — `src/main.rs` declares `Cmd::Export` + `Cmd::Import`, `src/lib.rs` declares `export_json` + `import_json` + `ImportError` + `MAX_STDIN_BYTES_DEFAULT`, `tests/bookmarks.rs` carries 15 Layer 3 Red Gate tests that pass GREEN at Phase 2b close. The README narrative pre-dates the Layer 3 promotion ("Layer 3 ... not built" is a Layer-2-cycle-era statement preserved through to the post-Layer-3-Phase-2b state).

A cold reader landing on the README first (the canonical entry path for a public-portfolio repository) is told the project is at Layer 2; they then read `bm --help` or `DESIGN.md` and see `bm export` + `bm import` subcommands and Layer 3 active scope. The two surfaces contradict each other — the reader cannot tell which is authoritative. The DESIGN.md is the spec contract per the SO domain prompt; the README is a documentation surface; the contradiction is documentation drift against the spec.

**Why this is SO scope-discipline, not merely a TW documentation finding:** the README claim "the reference-implementation purpose is satisfied by Layer 1 reaching project-terminal end-to-end + Layer 2 extending the worked example through a second iteration of the full 6-phase cycle" is a [G-112](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112) reference-example-purpose statement. Now that Layer 3 has been promoted, the README's "purpose-already-satisfied at Layer 2" framing becomes inconsistent with the project's own continued investment (Layer 3 spec authoring + Phase 2a/2b/2c). A reviewer reading the README + then the Layer 3 commits asks: "if the purpose was already satisfied at Layer 2, why was Layer 3 built?" The answer is operator-discretion ("I author first-draft; you edit + own" directive at DESIGN.md:47) + the AI-co-authored-disclosure framing — but the README does not currently surface that framing.

**Disposition:** SO recommends Option 1 (update README.md to reflect Layer 3 active state). Three sub-options for the framing:

1a. **Brief update**: "Current state: Layer 1 project-terminal at PR #42 + Layer 2 layer-terminal at PR #47 + Layer 3 active in the post-PR-#52 cycle (AI-co-authored first-draft; operator-owned)." Honest about the AI-co-authoring + parallel to DESIGN.md:47 disclosure shape.

1b. **Reference-example-purpose reframe**: "...the reference-implementation purpose was satisfied at Layer 1's project-terminal Phase 6 attestation; Layer 2 + Layer 3 extend the worked example as additional spec-cycle demonstrations including AI-co-authored Layer 3 spec authoring as a methodology-discipline subject." This is the more verbose path; it preserves the original [G-112](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112)-satisfaction claim while naming what Layer 3 adds to the worked example.

1c. **Minimal-status-line update only**: change just "Layer 3 ... is scoped ... but not built" to "Layer 3 ... is active in the post-PR-#52 cycle" without re-touching the purpose framing. Lowest-touch; preserves the unresolved tension about purpose-already-satisfied vs continued-investment for a future review to surface.

Option 2 (do nothing; preserve stale narrative) is not spec-honest — the README is the public-facing surface and the contradiction is observable to any cold reader.

**Classification:** Backlogged — operator-decision-required between 1a / 1b / 1c README update shapes. The fix is small (one to three sentences in README.md:9); the finding documents the staleness for Round 2 verification.

---

### Resolved

**Finding 3 — Spec coverage of operator-confirmed decisions: all 8 decisions from the operator-confirmation pass (`654cbbf`) are preserved at DESIGN.md + TODO.md (Dim 1)**

<a id="r1-f3"></a>

**Owner:** solution-owner
**Status:** validated
**Blocked by:** *(none)*
**Validator:** vdd-iar-alignment

The operator-confirmation pass commit `654cbbf` documents 8 decisions adjudicated against the AI-co-authored first-draft (`79a9a83`): 6 confirmed at AI-author-default + 2 operator-revised + 1 deferred to Phase 2b implementation verification. SO seat walked each decision against DESIGN.md + TODO.md + the implementation:

| Decision | Operator disposition | Spec surface | Implementation evidence |
|---|---|---|---|
| Dedup granularity | Confirmed: exact-tuple-match on url+timestamp+tags | DESIGN.md:120, :132, :165, :223 | `src/lib.rs:558-564` `contains` check using derived `PartialEq`; AC 20 test passes |
| Stdin input-size cap | Confirmed: 10 MB default + `--max-stdin-bytes <N>` override | DESIGN.md:128; AC 27 | `src/lib.rs:576` `MAX_STDIN_BYTES_DEFAULT = 10 * 1024 * 1024`; `src/main.rs:167` clap `default_value_t`; AC 27 test passes |
| Empty-stdin handling | Confirmed: exit 1 user-error (loud failure) | DESIGN.md:124; AC 22 | `src/main.rs:439-442`; AC 22 test passes |
| Fuzz framework | Confirmed: cargo-fuzz + libFuzzer | DESIGN.md:15 § Phase 5 Layer 3 strategy | NOT YET REQUIRED at Phase 2c close; spec-named for Phase 5 (Layer-gate criterion #5) |
| Stdin input shape | Confirmed: strict-only on `{"bookmarks":[...]}`; bare-array rejected | DESIGN.md:118, :123 | `src/lib.rs:525-532` top-level schema validation; AC 24 test passes |
| Filter-empty-state output | Confirmed: same `{"bookmarks":[]}` shape as store-empty | DESIGN.md:108, :109 | `src/lib.rs:454-456` filter routing converges with empty-store path at the JSON output; AC 16 test passes (the `tests_export_with_tag_filter_emits_or_union` covers the non-empty case; the filter-empty edge is implicit in the same code path) |
| Within-payload duplicates (REVISED) | Revised from "insert-all" to "dedup BOTH against destination AND within payload" | DESIGN.md:120, :132, :165 | `src/lib.rs:558-564` each `push` joins destination for subsequent `contains` — implements dedup-within-payload automatically; AC 20 test confirms the behavior |
| Selective-copy via `--tag`-filtered export (REVISED) | Revised from "documented use case" to "emergent behavior" | DESIGN.md:47, :114 | DESIGN.md:114 explicit "The composition of `bm export --tag <label>` with subsequent `bm import` is emergent from the parts working independently; the spec does not commit to it as a discrete documented use case." — narrower contract surface as operator-decided |
| `display_safe` placement (DEFERRED) | Deferred to Phase 2b verification | DESIGN.md:106 explicit | `src/lib.rs:464` + `:467` route both tag elements + URLs through `display_safe` at the per-field serialization step; AC 18 test confirms emitted bytes are JSON-valid + raw ESC absent — the deferred verification is satisfied at Phase 2b |

All 8 decisions are visibly present at DESIGN.md + TODO.md + the implementation. No decision was silently revised; no decision was silently dropped. The audit trail is complete.

**Edge-case-spec specificity check:** DESIGN.md § Layer 3 additions catalog (lines 156-168) covers all 12 named Layer 3 edge cases (Layer-1-format-export, empty-store-filter-export, control-chars-in-URL-export, round-trip, double-import-idempotence, partial-overlap, duplicate-URL-different-timestamp within payload, byte-equal within payload, size-cap, post-filter-import composition, Layer-1-format destination). No edge case is named in TODO.md that is missing from DESIGN.md; no edge case is named in DESIGN.md that is missing from TODO.md ACs. The 12 edge cases cover the 15 ACs without surplus.

**Classification:** Resolved — all 8 operator-confirmed decisions are preserved in spec + implementation; the audit trail from `79a9a83` first-draft → `654cbbf` confirmation → `878d3b6`+`fd21900`+`78bd3cf` implementation is intact.

---

**Finding 4 — Scope discipline confirmed: Layer 3 ACs 14-28 cleanly match the DESIGN.md § Scope and non-goals Layer 3 in-scope list with no creep beyond export + import (Dim 2, Dim 6)**

<a id="r1-f4"></a>

**Owner:** solution-owner
**Status:** validated
**Blocked by:** *(none)*
**Validator:** vdd-iar-alignment

DESIGN.md:43-46 § Scope and non-goals declares Layer 3 in-scope as exactly:

> "- `bm export` — emit bookmarks as JSON to stdout in the storage-format object-wrapped shape (`{"bookmarks":[...]}`); pipeable to other tools; optional `--tag <label>` flag for filtered export (parallel to `bm list --tag` OR-semantics).
> - `bm import` — read bookmarks from stdin (storage-format JSON); append to existing store preserving append-only semantics; idempotent on URL+timestamp+tags exact-match (no duplicate-row creation for identical records). Storage format unchanged from Layer 2."

ACs 14-28 enumerate behaviors all within this scope: AC 14-18 are `bm export` surface (emit + filter + display_safe); AC 19-27 are `bm import` surface (append + idempotence + empty-stdin + invalid-JSON + schema-mismatch + Layer-1-migration + partial-failure atomicity + size-cap); AC 28 is the round-trip composition of the two. No AC introduces a third subcommand; no AC adds a non-tag flag to export or a non-stdin-input mechanism to import; no AC extends storage format ("Storage format unchanged from Layer 2" — confirmed; `Bookmark` struct at `src/lib.rs:50-56` is unchanged from Layer 2; only the `derive(PartialEq, Eq)` was already present from Layer 2 for `bookmark_tags_accessor_returns_constructor_supplied_slice`).

The implementation surfaces add exactly:

- `src/lib.rs`: `BookmarkStore::export_json` (pure) + `BookmarkStore::import_json` (pure transformation; mutation of `self.bookmarks` only after validation) + `MAX_STDIN_BYTES_DEFAULT` const + `ImportError` enum + `Display` + `std::error::Error` impls. No new dependencies; the import_json uses the existing `serde_json::Value` + `serde_json::from_value` path that was already a transitive dep at Layer 1.
- `src/main.rs`: `Cmd::Export { tags: Vec<String> }` + `Cmd::Import { max_stdin_bytes: usize }` clap variants + `run_export` + `run_import` per-subcommand helpers. The two new helpers parallel the Layer 2 R2 extract-and-name pattern (`run_add` / `run_list` / `run_tag`); the extract-and-name annotation at Phase 2c commit `78bd3cf` makes this discipline visible in TODO.md.
- `tests/bookmarks.rs`: 15 new Red Gate tests (1:1 mapping to AC 14-28); no shared helper module added; no new dev-dep introduced.

No new clap subcommand surface beyond the spec-named two; no new lib-public type beyond what the spec contract requires; no new dependency beyond what was already present at Layer 2; no new build artifact (no `fuzz/` dir yet — the cargo-fuzz infrastructure is Phase 5 Layer 3 work per DESIGN.md:15 + TODO.md:142, not Phase 2b implementation scope). Dim 2 + Dim 6 + Dim 7 all clean.

**The one borderline case:** the implementation's `ImportError` enum (`src/lib.rs:584-599`) is hand-rolled with `Display` + `std::error::Error` impls — parallel to the Layer 2 `AttachTagError` enum + the project's documented decision against `thiserror` (DESIGN.md:264 Technology choices: "anyhow for error types — Custom error enums per `thiserror` — Single-binary tool; `thiserror` would be over-engineering"). The Layer 3 implementation preserves this decision; `ImportError` is in the same hand-rolled shape as `AttachTagError`. Dim 3 (Technology compliance) clean.

**Classification:** Resolved — the Layer 3 in-scope surface matches DESIGN.md's Layer 3 in-scope declaration with no creep; the implementation adds only what AC 14-28 require; no scope-bleed into Layer 4-or-beyond territory (no AND-semantics filter operators per DESIGN.md:101's deferred-pending-feedback framing; no `bm delete` or `bm edit` per the non-goals; no shared-storage / multi-process consideration; no encryption-at-rest).

---

**Finding 5 — Intent calibration confirmed proportionate: capstone intent remains correct for Layer 3 despite the stdin-fed attacker surface introduction (Dim 4)**

<a id="r1-f5"></a>

**Owner:** solution-owner
**Status:** validated
**Blocked by:** *(none)*
**Validator:** vdd-iar-alignment

The operator-supplied per-domain prompt asks: "intent calibration (is capstone still the right intent for this reference example? does Layer 3 introduce production-intent triggers like adversary-controlled stdin?)".

The question is real — Layer 3 IS the project's first untrusted-input attack surface per DESIGN.md:130 § Threat model addition for stdin-fed attacker input. A production-intent project consuming untrusted stdin would warrant stricter hardening: fuzz-target activation BEFORE Phase 2b ships (not Phase 5); per-allocation memory caps beyond the 10 MB stdin cap; per-parse panic recovery beyond serde_json's default; rate-limit / repeated-invocation defense; structured-error reporting to a centralized logging system. The SO seat evaluates whether any of these are required by the project's reference-example purpose.

They are not. The reference-implementation purpose ([G-112](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112)) is "exercise all six VSDD phases end-to-end as a worked example". Layer 1 demonstrated the six phases against a no-untrusted-input surface; Layer 2 demonstrated the same six phases against a forward-only-migration surface; Layer 3 demonstrates the same six phases against a stdin-fed-attacker-input surface. The six-phase methodology applies uniformly; the worked-example purpose is to teach the methodology, not to ship a production-tier stdin parser. The capstone intent calibrates the rigor floor (full 6-phase exercise, 13-domain active set, MVR-strict gates) at a level proportionate to "reference example", not "production tool".

Three observable signals support the disposition:

1. **The accepted-limitation framing at DESIGN.md:128 for the 10 MB cap is calibrated to "reference-example operator-controlled stdin"**, not "production unbounded internet stdin". The cap is operator-overridable via `--max-stdin-bytes <N>`; the spec names this as the operator's responsibility, not a hard-limit budget. A production-intent fork would make the cap unconfigurable + lower the default + add a per-second-rate cap; the reference-example treats the cap as a sensible default + operator escape hatch.

2. **The threat model addition at DESIGN.md:130 explicitly inherits the Layer 1 disposition: "same disposition: store the bytes as-given; defer rendering safety to the `display_safe` discipline at output time."** The discipline is "no new defense pattern; reuse the existing pattern". A production-intent project would author a new defense pattern (allowlist URL schemes, validate URL syntax, normalize byte sequences to canonical Unicode); the reference example chose to teach "the existing defense is sufficient at this scope" — which IS a methodology lesson worth teaching at capstone (the proportionality call).

3. **The Phase 5 Layer 3 strategy at DESIGN.md:15 names cargo-fuzz + libFuzzer activation against the import_stdin path as the bug-class-targets gate** (parse-panic / parse-OOM / parse-stack-overflow / non-spec exit codes). A production-intent project would activate fuzz BEFORE Phase 2b shipped; capstone gates Phase 5 hardening AFTER Phase 2b (the canonical six-phase shape). The deferral-to-Phase-5 IS the capstone discipline — Phase 5 catches what Phase 2b ships; production would gate Phase 5 as a pre-ship requirement.

**Capstone intent holds.** The Layer 3 stdin-fed attacker surface is correctly framed as an extension of the existing threat-model discipline rather than a tier-up trigger. The operator-confirmed-decisions block at `654cbbf` (operator chose the 10 MB cap + `--max-stdin-bytes` override + cargo-fuzz framework + strict-object-wrapped-only) is consistent with capstone-tier intent applied to a reference-example surface — none of the operator's revisions narrowed the contract to production-tier strictness; they tightened it (within-payload dedup; emergent rather than contract for selective-copy) at the methodology-discipline level, not the production-tier level.

**No production-intent escalation trigger fires.** The Layer 3 stdin surface stays within capstone scope.

**Classification:** Resolved — capstone-tier intent calibration holds; Layer 3 does not introduce production-intent triggers despite the new stdin-fed attacker surface; the accepted-limitation framing at DESIGN.md:128 is correct.

---

**Finding 6 — Phase 6 not-applicable disposition for Layer 3 is consistent with the R4 F2 precedent + correctly framed against [G-150](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-150) + [G-112](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112) (Dim 8 — prior-review additions; Dim 4 — over-engineering)**

<a id="r1-f6"></a>

**Owner:** solution-owner
**Status:** validated
**Blocked by:** *(none)*
**Validator:** vdd-iar-alignment

DESIGN.md:17 § Project intent's Phase 6 strategy line declares for Layer 3:

> "Layer 3 four-dimensional convergence (AI-co-authored; operator-owned): **NOT APPLICABLE** per the same [G-150](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-150) + [G-112](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112) rationale as Layer 2 — capstone gates at project-terminal MVR per primer 6, not per-layer; running Phase 6 for Layer 3 would re-teach the same not-applicable disposition the Layer 2 declaration already documents. The Phase 5 hardening at Layer 3 still occurs (Purity Boundary Audit re-run + Mutation Testing re-run + proptest round-trip + cargo-fuzz on bm import); Phase 6 specifically (four-dimensional convergence attestation) is the not-applicable part."

TODO.md:153 Layer-gate criterion #6 echoes this:

> "**Phase 6 not applicable** per DESIGN.md § Project intent Phase 6 strategy declaration (G-150 + G-112 — same rationale as Layer 2: capstone gates at project-terminal MVR per primer 6, not per-layer; Layer 1's Phase 6 attestation stands as the project's terminal four-dimensional convergence record)."

The disposition is consistent with R4 F2 (Layer 2 Phase 6 not-applicable per the SO recommendation; R5 F2 verified the closure held). The Layer 3 application of the same precedent is correct:

- The reference-example purpose ([G-112](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112) — "exercise all six VSDD phases end-to-end as a worked example") was satisfied at Layer 1's Phase 6 attestation per [VDD-IAR Alignment Review 3](2026-05-20-vdd-iar-alignment.md). The worked example is the audit-trail artifact; running Phase 6 again does not extend the artifact's worked-example reach.
- The over-investment guard at [G-150](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-150) (capstone gates at project-terminal MVR per primer 6, not per-layer) is correctly applied: re-running Phase 6 for Layer 3 would teach methodology consumers that capstone artifacts require per-layer four-dimensional convergence, which is not the suite's intent. The Layer 2 closure already documented this; Layer 3 re-applies the same disposition rather than re-running the same Phase 6.
- The Phase 5 hardening at Layer 3 STILL occurs (Purity Boundary Audit + Mutation Testing + proptest round-trip + cargo-fuzz on `import_stdin`); Phase 5 is layer-scoped per primer 5 because hardening targets the layer's new surface, not the project's cumulative surface. Phase 6 is project-scoped per primer 6 because four-dimensional convergence attestation is the project-terminal sign-off. The distinction is preserved correctly in the spec.

**No over-investment.** The Layer 3 cycle's Phase 6 commitment is the explicit not-applicable declaration, which preserves the spec contract without forcing 13-domain re-attestation for a methodology already-attested at Layer 1.

**[G-162](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-162) strict-form requirement satisfied:** capstone-intent declarations require both Phase 5 + Phase 6 strategy lines named; DESIGN.md:15 names Phase 5 strategy for Layer 3 (Purity Boundary Audit + Mutation Testing + proptest round-trip + cargo-fuzz framework + named bug classes); DESIGN.md:17 names Phase 6 strategy for Layer 3 (explicit not-applicable with G-150 + G-112 rationale). Both lines are present + carry named scope + are visible in the project intent block. Cross-check against [Solution Owner Review 4 F2](2026-05-21-solution-owner.md#r4-f2) + [Review 5 F2](2026-05-21-solution-owner.md#r5-f2) precedent confirms the disposition is structurally identical to Layer 2's — same rationale, same closing form, same Layer 1 attestation citation.

**Classification:** Resolved — Phase 5 + Phase 6 strategy declarations for Layer 3 are present + correctly calibrated against capstone-intent + G-150 + G-112; the Phase 6 not-applicable disposition is consistent with the R4 F2 precedent + applies the over-investment guard correctly.

---

**Finding 7 — Reference-example purpose alignment: the AI-co-authored disclosure shape at DESIGN.md:47 + the operator-confirmed-decisions inline summary support clarity for a methodology reader without compromising the spec-contract authority (Dim 8; G-112 reference-example purpose)**

<a id="r1-f7"></a>

**Owner:** solution-owner
**Status:** validated
**Blocked by:** *(none)*
**Validator:** vdd-iar-alignment

The operator-supplied per-domain prompt asks: "reference-example purpose alignment (does Layer 3 teach the methodology cleanly? does the AI-co-authored disclosure add or subtract clarity for a reader?)".

The disclosure shape at DESIGN.md:47:

> "(Layer 3 promoted from 'deferred — scoped only' to active at AI-co-authored first-draft 2026-05-24 per operator's 'I author first-draft; you edit + own' directive. **This spec is AI-co-authored; operator owns the final contract.** The [G-156](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-156) developer-voice discipline applies to the PROCESS.md retrospective, not the spec authoring; the AI-co-authored-disclosure shape parallel to `PROCESS.md` § AI-co-authored reference-example disclosure applies. Operator-confirmed decisions inline: dedup-on-`url`+`timestamp`+`tags` exact-tuple-match (both against destination state AND within imported payload); 10 MB input-size cap default with `--max-stdin-bytes <N>` override; strict-object-wrapped stdin only (bare arrays rejected); empty-stdin treated as user-error exit 1; cargo-fuzz with libFuzzer as the Phase 5 fuzz harness; filter-empty-state shares the store-empty `{"bookmarks":[]}` shape; selective-copy via `--tag`-filtered export stays silent as emergent behavior. `display_safe` placement at the serialization step is deferred to Phase 2b implementation verification.)"

The shape adds three signals: (a) which decisions were operator-confirmed (a transparency claim — the reader sees the contract is the operator's, not the AI's silent default); (b) what the AI-author defaults were vs. what the operator revised (the disclosure names two revisions: within-payload dedup tightening + selective-copy emergent-rather-than-documented); (c) the deferred decision (display_safe placement) is named as Phase-2b-verification-bound, so the reader knows where to look for the closure evidence.

**Clarity gain:** the reader knows which decisions are operator-considered vs. AI-author-default vs. deferred. This is the methodology lesson the suite teaches at [G-156](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-156) (developer-voice discipline) applied to AI-co-authored spec authoring — a parallel discipline to the PROCESS.md AI-co-authored disclosure. A methodology consumer reading the bookmark-cli-manual reference example sees how to disclose AI-co-authored spec authoring honestly, which is itself a methodology contribution worth surfacing.

**Clarity cost:** the disclosure is verbose (one large paragraph). A reader scanning DESIGN.md § Scope and non-goals for the Layer 3 in-scope list (DESIGN.md:43-46) reads three short bullets + then a long disclosure paragraph that interrupts the rhythm of the scope-and-non-goals section. The disclosure could be moved out (e.g., to § Project intent or a dedicated § Layer 3 disclosure subsection) and a brief inline pointer left in § Scope and non-goals — but that is a documentation-discipline call, not a spec-contract correctness call.

**No spec-contract authority concern:** the disclosure explicitly says "operator owns the final contract" — the AI-co-authoring does not weaken the operator's spec authority. The 8 operator-confirmed decisions are visible at the disclosure paragraph + the per-AC TODO.md surfaces + the per-§ DESIGN.md behavioral-contract surfaces. The audit trail is triple-coded (operator's commit `654cbbf` message + DESIGN.md:47 disclosure + per-AC implementation evidence) — a future reviewer cannot misread which decisions were operator-considered.

**The reference-example purpose is teachable from this disclosure shape.** A consumer reading "DESIGN.md is AI-co-authored; operator owns the final contract" learns that AI-co-authoring is a valid pattern at capstone+ intent when disclosed honestly + the operator exercises spec authority on the load-bearing decisions. The lesson generalizes — the operator's "I author first-draft; you edit + own" directive could be inverted ("operator authors first-draft; AI reviews + revises") at a future project without invalidating the disclosure discipline. The disclosure shape is the methodology contribution.

**No SO finding against the disclosure.** A Documentation Reviewer / Technical Writer round may have separate concerns about the verbosity / placement of the disclosure paragraph; SO defers to those domains for the rendering-and-placement call.

**Classification:** Resolved — the AI-co-authored disclosure shape preserves clarity for a methodology reader + does not weaken the spec-contract authority; the reference-example purpose ([G-112](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112)) is honestly extended at Layer 3.

---

### Hallucinated

*(none — the seven findings above are concrete and citation-backed; no SO-dim concerns that turned out to be spec-misread emerged in this round)*

---

### Approved deviation

*(none — no pre-approved DESIGN.md deviations apply at this round)*

---

### Dismissed

*(none — every Layer 3 spec commitment was either Met, Resolved (Findings 3-7), or Open (Findings 1-2); no dismissable concerns)*

---

### Raised to SO

*(none — this IS the SO round; cross-domain findings that would route to SO are filed against their originating domain's log)*

---

### Summary

Seven findings in Round 1:

- **Backlogged (operator-decision-required, blocks Layer 3 layer-gate close):**
  - [Finding 1](#r1-f1) — `manual-tests/layer-3.md` absent (Dim 5 under-delivery against TODO.md:138 spec commitment + TODO.md:150 Layer-gate criterion #3); two resolution paths (author OR amend spec).
  - [Finding 2](#r1-f2) — README.md:9 still says "Layer 3 ... scoped but not built" against post-Phase-2b state where Layer 3 is implemented (Dim 7 design fidelity / stale narrative); three sub-options for the README update shape.
- **Resolved:**
  - [Finding 3](#r1-f3) — all 8 operator-confirmed decisions from `654cbbf` are preserved at DESIGN.md + TODO.md + the implementation; audit trail intact (Dim 1).
  - [Finding 4](#r1-f4) — Layer 3 ACs 14-28 cleanly match the DESIGN.md § Scope and non-goals Layer 3 in-scope list; no creep beyond export + import (Dim 2 + Dim 6 + Dim 7).
  - [Finding 5](#r1-f5) — capstone intent remains correct for Layer 3; the stdin-fed attacker surface does not trigger production-intent escalation; accepted-limitation framing for the 10 MB cap is calibrated correctly (Dim 4).
  - [Finding 6](#r1-f6) — Phase 5 + Phase 6 strategy declarations for Layer 3 are present + correctly calibrated against capstone-intent + G-150 + G-112; the Phase 6 not-applicable disposition is consistent with the R4 F2 precedent (Dim 8 + Dim 4).
  - [Finding 7](#r1-f7) — the AI-co-authored disclosure shape at DESIGN.md:47 preserves clarity for a methodology reader + does not weaken the spec-contract authority; the reference-example purpose is honestly extended at Layer 3 (Dim 8, [G-112](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112)).

**Operator-supplied per-domain prompt answers (summarized for the audit trail):**

1. _"Scope discipline (did Layer 3 stay in scope per the original 'deferred — scoped only' + active promotion? did the implementation creep beyond the spec contract?)"_ — Layer 3 stayed cleanly within scope per [Finding 4](#r1-f4). The implementation adds exactly what AC 14-28 require; no third subcommand, no new flag beyond `--tag` + `--max-stdin-bytes`, no new storage-format field, no new dependency beyond what was already transitively present at Layer 2.
2. _"Intent calibration (is capstone still the right intent for this reference example? does Layer 3 introduce production-intent triggers like adversary-controlled stdin?)"_ — Capstone holds per [Finding 5](#r1-f5). The stdin-fed attacker surface is correctly framed as an extension of the existing threat-model discipline rather than a tier-up trigger; no production-intent escalation fires.
3. _"Spec coverage of decisions (are all the operator-confirmed decisions from the AI-co-authored first-draft preserved in the final spec? are any decisions ambiguous?)"_ — All 8 decisions preserved per [Finding 3](#r1-f3); no decision ambiguous; the 6 confirmed-at-default + 2 revised + 1 deferred all visible at DESIGN.md + TODO.md + implementation.
4. _"Risk inventory + accepted-limitations (are the Layer 3 accepted risks documented? is the 10 MB stdin cap accepted-limitation framing correct?)"_ — Accepted-limitation framing is correct per [Finding 5](#r1-f5). The 10 MB cap with `--max-stdin-bytes <N>` override is the reference-example-proportionate framing; a production-intent fork would tighten it but capstone intent calibrates to "reference-example operator-controlled stdin" not "production unbounded internet stdin".
5. _"Layer-gate criteria readiness (will the project clear all 6 layer-gate criteria for Layer 3? what's missing?)"_ — Criterion #3 (`manual-tests/layer-3.md` runs clean) cannot clear without [Finding 1](#r1-f1) resolution; criterion #5 (Phase 5 Layer 3 rounds at closure) still ahead; criteria #1 + #2 + #4 + #6 clear cleanly at the current state. Two of six clear-blocking gaps; both have spec-honest resolution paths.
6. _"Reference-example purpose alignment (does Layer 3 teach the methodology cleanly? does the AI-co-authored disclosure add or subtract clarity for a reader?)"_ — Reference-example purpose is honestly extended per [Finding 7](#r1-f7). The AI-co-authored disclosure shape is a methodology contribution (parallel to PROCESS.md AI-co-authored disclosure); a methodology consumer learns "AI-co-authoring is a valid pattern at capstone+ intent when disclosed honestly + the operator exercises spec authority on load-bearing decisions."
7. _"Phase 5 + Phase 6 strategy declarations for Layer 3 (consistent with capstone intent + G-150 + G-112?)"_ — Yes per [Finding 6](#r1-f6). Phase 5 strategy named (Purity Boundary Audit + Mutation Testing + proptest round-trip + cargo-fuzz framework); Phase 6 strategy named as explicit not-applicable with G-150 + G-112 rationale; both lines satisfy G-162 strict-form requirement.

**Coordination:** [Finding 1](#r1-f1) (manual-tests/layer-3.md under-delivery) is the natural SO surfacing of what would be a Quality Engineer test-coverage finding from the manual-test-discipline seat — QE in any adjacent Layer 3 cluster should be expected to surface the same gap; cross-validation between SO + QE expected. [Finding 2](#r1-f2) (README.md stale citation) routes naturally to Technical Writer or Documentation Reviewer for the rendering-and-placement call. [Finding 3-7](#r1-f3) document the spec-clean + scope-clean + intent-clean + Phase 5/6-clean + disclosure-clean state for future-cycle regression-check.

**Phase 5 / Phase 6 closure-blocker check:** [Finding 1](#r1-f1) blocks Layer 3 layer-gate criterion #3; [Finding 2](#r1-f2) does not block layer-gate close but blocks publishable-state declaration on the README's accuracy claim. Both have spec-honest Option-1 fixes; if Round 2 finds them adopted, both close cleanly per the R5 F1 precedent.

**Validator:** vdd-iar-alignment — VDD-IAR Alignment confirms the spec changes (the Finding 1 + Finding 2 resolution paths each route through TODO.md / README.md amendments) went through proper routing and don't conflict with prior intent).

---

## Review 2 — 2026-05-25 04:30Z

**Round:** Layer 3 Phase 3 IAR Round 2.
**Phase:** [Phase 3](../../../../vsdd-suite/primers/3-review-session.md) — Iterative Adversarial Refinement (Layer 3 Round 2; cold-session continuation against the post-Round-1-fix state at commits `fdfa989` → `ba6a4a9` → `bfc0713` → `795bc25`).
**Scope:** Layer 3 spec-vs-implementation compliance verification that Round 1 fixes hold + surface NEW residuals introduced by the fix-work. Round 1 closure context: 2 Backlogged (manual-tests + README) + 5 Resolved + 5 SO-decidable findings adjudicated via main-session AskUserQuestion pass at Phase 4 (per per-domain Phase 4 routing appendices (per-domain Phase 4 appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md`)) + 1 architectural correction sub-decision at Phase 2b landing (`display_safe` removed from `export_json`) + in-cycle suite-hardening (Review 94 meta-finding cycle + `check-no-letter-clusters.py` hook + primer 4 amendment). Layer 1 + Layer 2 + Layer 3 Round 1 regression-check baselines preserved.
**Session note:** Cold session opened against post-`795bc25` state. Did not author any Round 1 fix-work commits, the Phase 4 routing record, the suite-side Review 94, the lettering hook, or the manual-tests/layer-3.md artifact. Reading order: [SO domain prompt](../../../../vsdd-suite/domains/role/SOLUTION-OWNER-REVIEW.md) → [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) → [Review 1 above](#review-1--2026-05-25-0112z) → Phase 4 routing record (per-domain Phase 4 appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md`) → [suite Review 94](../../../../vsdd-suite/suite-development/review-log/2026-05-24-suite-review.md#review-94--2026-05-25-0300z) → [README.md](../../README.md) + [PROCESS.md](../../PROCESS.md) + [CHANGELOG.md](../../CHANGELOG.md) post-Round-1 narrative → [TODO.md § Layer 3](../../TODO.md#layer-3--export-and-import-ai-co-authored-operator-owned) Layer-gate criteria + Phase 2c follow-up annotation → [DESIGN.md](../../DESIGN.md) Layer 3 § Behavioral contracts + § Threat model + § Storage data classification + § Verification architecture + § Performance budget read LAST per cold-reader-poisoning discipline → [`src/lib.rs`](../../src/lib.rs) (`export_json` architectural correction + `import_json` + `bookmark_set_eq` + `display_safe` JSON-native rewrite + `ImportError::TagContainsControlChars`) → [`src/main.rs`](../../src/main.rs) (`run_import` validation order + size-cap hint + `long_about`) → [`tests/bookmarks.rs`](../../tests/bookmarks.rs) (51 tests) → [`manual-tests/layer-3.md`](../../manual-tests/layer-3.md) (16 steps).
**Source:** domain-raised — cold-session adversarial reviewer applying the SO domain's nine dimensions to the Round 1 closure surface + verifying the in-cycle scope additions stayed proportionate. The operator-supplied per-domain prompt directed re-verification of: R1 Backlogged closure (manual-tests + README); 5 SO-decidable Round 1 findings implementation-alignment; architectural-correction sub-decision scope discipline; in-cycle suite-hardening scope discipline; layer-gate criteria readiness.
**Lens:** R1 regression-check (do the Round 1 fixes hold against the impl?); Dim 5 under-delivery (are any Round 1 routed items partially implemented?); Dim 2 + Dim 6 scope discipline (architectural correction + suite-hardening in-cycle scope); Dim 7 design fidelity (post-fix spec narrative + post-fix README narrative + CHANGELOG accuracy claim); Dim 8 prior-review-additions (the architectural correction sub-decision is a Round 1-routing-adjacent SO-decision; verify it stayed within the operator's intent); reference-example purpose alignment ([G-112](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112)) of the audit-trail-with-architectural-correction shape; layer-gate readiness across all 6 criteria.
**Reference:** DESIGN.md is the post-fix spec contract; the Phase 4 routing record is the operator-decided routing of Round 1 findings; the 4 Round 1 fix-work commits are the implementation-of-decisions artifact this round evaluates compliance against.
**Regression-check against:** [Review 1 above](#review-1--2026-05-25-0112z) (Layer 3 Round 1 SO MVR — 2 Backlogged + 5 Resolved) + [Solution Owner Review 5](2026-05-21-solution-owner.md#review-5--2026-05-22-1630z) (Layer 2 Round 2 SO MVR — the closure-verification precedent this round mirrors) + [Solution Owner Review 3](2026-05-20-solution-owner.md#review-3--2026-05-20-2200z) (Layer 1 project-terminal SO MVR — Layer 1 ACs 1-4 regression floor).

**Compliance table** (Round 1 routed findings + 5 SO-decidable items + architectural-correction sub-decision vs. observable post-fix implementation behavior):

| Routed item | Operator decision (Phase 4 record) | Post-fix evidence | Status |
|---|---|---|---|
| JSON-native escape design (SA+SE+RT+Sec 4-domain) | switch `display_safe` to JSON-native `\uHHHH` 6-char form; preserve byte-round-trip | `src/lib.rs:798-807` BMP-+-surrogate-pair encoder; `src/lib.rs:455-499` `export_json` delegates to serde-native (architectural correction); `tests_export_import_round_trip_preserves_pathological_bytes` GREEN | Met |
| sorted-tag-comparison dedup (SE+RT 2-domain) | dedup on sorted-tag-comparison; storage Vec preserves insertion order | `src/lib.rs:624-636` `bookmark_set_eq` helper; `src/lib.rs:601-611` import_json uses it; DESIGN.md:133 names L132→L223 set-frame resolution; tag-reorder regression test GREEN | Met |
| imported-tag control-char rejection (Sec F2) | active mitigation; new `ImportError` variant for control-char tag rejection | `src/lib.rs:583-589` pre-mutation predicate; `src/lib.rs:681` `TagContainsControlChars(usize, String)` variant; `src/main.rs:515-525` CLI arm with `display_safe` on tag; DESIGN.md:129 spec contract; control-char rejection test GREEN | Met |
| imported-tag classification extension (Sec F3) | inherit same classification as user-typed tags | DESIGN.md:332 § Storage data classification new paragraph; no impl change required (spec-only) | Met |
| SO F1 manual-tests/layer-3.md authoring | Author the file per the R1 F1 Option-1 path | `manual-tests/layer-3.md` exists; 16 steps; covers AC 14..AC 28 + Round 1 routed closures at Steps 8/9/10 + size-cap with override at Step 12 + hyperfine sanity-check at Step 15 | Met (file exists; operator-execution pending per criterion-3 hard gate) |
| SO F2 README post-L3-state | Sub-option 1a/1c — minimal status-line + Phase-progression accuracy | README.md:9 "Layer 3 active in PR #52" + Round 1 + Phase 4 + routing-record references; in-flight framing accurate to fix-work-mid-cycle state | Met |
| QE coverage gaps (F1+F2+F3) | Phase 2a new tests | `tests_import_dedup_within_payload_collapses_byte_equal_records`; `tests_export_applies_display_safe_to_pathological_tag` (revised assertion to byte-round-trip semantic); `tests_import_max_stdin_bytes_operator_override` — all GREEN | Met |
| UX help-and-error-remediation (UX F2 + TW F4 + SE F4) | Phase 2b impl changes | `src/main.rs:464-468` size-cap hint with MiB + remediation; `src/main.rs:428-431` lower-bound `--max-stdin-bytes` rejection; `src/main.rs:449-455` empty-stdin-before-size-cap ordering; clap `long_about` covers Layer 3 surface | Met |
| DESIGN.md verification-architecture refresh (SA F2) | Phase 1a+1b spec amendment | DESIGN.md:237-239 § Verification architecture pure-fn list extends to `export_json` + `import_json` + `display_safe` Layer 3 framing | Met |
| dedup-complexity accepted-limit annotation (SA F4 + PE convergence) | Phase 1a+1b accepted-limit | DESIGN.md:298 § Performance budget Layer 3 paragraph on O(M×N) dedup-complexity accepted-limit | Met |
| import_json doc-comment fix (SE F3) | Phase 2b — remove proptest claim until Phase 5 lands the property | `src/lib.rs:515-518` doc comment names "the proptest itself is not yet activated in `tests/properties.rs` at this Phase 2b landing" — claim removed | Met |
| ImportError variant detail (SA F5) | Phase 2b LOW-PRIORITY deferred-to-follow-up-PR | Not landed; explicitly deferred per Phase 4 routing record line 282 (`G-150 over-investment guard`); SchemaMismatch still carries `String` only | Deferred-by-spec — not a Round 2 finding |
| Phase 2c follow-up annotation for `bfc0713` | Phase 2c discipline | TODO.md:146 follow-up annotation paragraph documents additive-changes-preserve-helper-structure + names the architectural correction as structural-simplification | Met |
| `display_safe` architectural correction sub-decision at Phase 2b | Operator-authorized at Phase 2b landing per CHANGELOG L24-31 narrative | DESIGN.md:106 § `bm export` rewritten to name "JSON-native escape design ... architectural correction sub-decision at Phase 2b landing"; impl at `src/lib.rs:455-499` documents the trade-off + the why; CHANGELOG L24-31 names the change | Met — see Finding 2 below for scope-discipline disposition |

_Layer 1 + Layer 2 + Layer 3 Round 1 regression-check (the floor from Reviews 3 + 4 + 5 + Review 1 above):_ `cargo test --test bookmarks` runs **51/51 GREEN** (45 from Round-1-pre-fix-baseline + 6 new Phase 2a regression-or-coverage tests); `cargo test --test properties` runs **3/3 GREEN**; `cargo build --release` runs clean (0 warnings); `cargo clippy --all-targets --all-features` runs clean. Integration-test exit-code contract (0/1/2/64), atomic-save discipline, mode-0600, symlink-rejection, parent-directory fsync all preserved per spot-check of `src/lib.rs:175-323`. Layer 1 ACs 1-4 + Layer 2 ACs 5-13 + Layer 3 ACs 14-28 all hold at the integration surface. **HOWEVER:** `cargo test --lib` reveals 2 unit-test failures (see Finding 1) — Layer 1 + Layer 2 spec compliance HOLDS at the integration surface but the Round 1 `display_safe` rewrite introduced a regression at the unit-test surface that the fix-work missed.

**MVR signal:** **Round 2 — NOT REACHED.** One new real finding (under-delivery against the test sweep: the `display_safe` JSON-native rewrite at `bfc0713` left 2 pre-existing Layer 1/2 unit tests asserting the old Rust-syntax escape format — `cargo test --lib` fails 2/13, AND `cargo test` (the default invocation README.md:51 promises will pass) fails). One scope-discipline observation closes cleanly (the architectural correction sub-decision stayed within operator intent + is honestly disclosed at the spec + CHANGELOG + Phase 2c follow-up annotation; SO does not find scope-creep). One scope-discipline observation closes cleanly with a documented carry-forward (the in-cycle suite-hardening — Review 94 + lettering hook + primer 4 amendment — was operator-authorized at letter-label 4th-recurrence and the load-bearing hook prevents future commits violating the discipline; SO does not find unauthorized scope-creep but flags the PR-#52 scope-blast surface for the post-merge audit). Two layer-gate criteria gaps named (criterion 1 fails on the `cargo test --lib` regression; criterion 3 operator-execution pending). Per [G-131](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-131) continue-trigger discipline, Round 3 is mandatory if Finding 1 opens with non-trivial fix work; an Option-2 path (declare the 2 unit tests obsolete given the JSON-native escape format is the new contract; update assertions; close in Round 2) closes the finding cleanly without escalation.

---

### Backlogged

<a id="r2-f1"></a>

**Finding 1 — Under-delivery: Round 1 `display_safe` JSON-native rewrite at `bfc0713` left 2 pre-existing unit tests asserting the old `\u{HHHH}` Rust-syntax form; `cargo test --lib` fails 2/13 + `cargo test` (the README.md:51 promised-passing invocation) fails (Dim 5 + Dim 7 design fidelity)**

**Owner:** solution-owner
**Status:** raised
**Blocked by:** *(none — directly observable test failure)*
**Validator:** vdd-iar-alignment

The Round 1 Phase 2b fix-work at commit `bfc0713` rewrote `display_safe` from the Rust-syntax `\u{HHHH}` curly-brace form (used at Layer 1 + Layer 2) to the JSON-native `\uHHHH` 6-char form (per the JSON-native-escape-design operator decision routed at Phase 4). The fix-work updated:

- the integration-test `bm_list_sanitizes_terminal_escape_in_url` per CHANGELOG.md:56 ("updated assertion to expect the JSON-native `` 6-char form (was `\u{001b}` Rust-syntax)");
- the spec narrative at DESIGN.md:106 § `bm export` JSON-native-escape-design paragraph;
- the impl at `src/lib.rs:798-807`.

But the fix-work did NOT update the 2 pre-existing `display_safe` UNIT tests at `src/lib.rs:1042-1054` (`display_safe_escapes_ansi_escape`) + `src/lib.rs:1056-1064` (`display_safe_escapes_format_chars`), both of which still assert `out.contains("\\u{001b}")` / `out.contains("\\u{202e}")` (the old Rust-syntax form). The new JSON-native impl emits `` / `‮` (no curly braces), so the assertions fail:

```
test tests::display_safe_escapes_ansi_escape ... FAILED
  panicked at src/lib.rs:1046:9: ESC should be escaped; got 31mred

test tests::display_safe_escapes_format_chars ... FAILED
  panicked at src/lib.rs:1060:9: RLO should be escaped; got plain‮evil
```

**Why this is SO scope-discipline, not "merely a missed test sweep":** The README.md:51 promised invocation reads `cargo test  # expect: all tests pass`. The TODO.md:150 Layer-gate criterion #1 reads "All Red Gate tests above pass: `cargo test --test bookmarks` + `cargo test -- --ignored` (scaling) + `cargo test --test properties`" — which is satisfied (51/51 + 3/3 GREEN). The criterion #1 wording does NOT mention `cargo test --lib`, so a strict reading lets the layer-gate-criterion #1 close. BUT:

- The README's promise is the public-portfolio contract; a cold reader running the README's command sees 2 failing tests, not "all tests pass." This is design fidelity drift per Dim 7 + the same Dim-5 framing as Review 1 F1.
- The unit-test invocation is the canonical Rust developer's first-pass health check (`cargo test` is the canonical command; the user must opt into `--test bookmarks` to scope away from the lib failures). A future maintainer running `cargo test` after the PR merges sees a failing project.
- The grep-before-claim-closure discipline named in PROCESS.md:96-98 (Stumbling point 6 — "Site-specific fix declared closure") is the exact pattern the Round 1 fix-work tripped: the display_safe site update at the integration-test surface declared closure without grepping the lib unit tests for the same assertion pattern. The methodology fix landed in PR #40 + is repeated in PROCESS.md as a discipline reference; the Round 1 fix-work did not apply it.
- The CHANGELOG.md:51 claim "**51 passed; 0 failed**" is technically correct for `cargo test --test bookmarks` but elides the `cargo test --lib` failure. CHANGELOG drift from the actual test-surface state is the same Dim 7 drift pattern Review 1 F2 surfaced for the README.

**Disposition:** Two resolution paths are spec-honest, parallel to the Round 1 R1 F1 + F2 shape:

1. **Update the 2 unit-test assertions** to expect the JSON-native form (`out.contains("\\u001b")` + `out.contains("\\u202e")`) consistent with the Round 1 operator decision + the integration-test update at `bm_list_sanitizes_terminal_escape_in_url`. Add a sweep step to CHANGELOG.md naming the unit-test update alongside the integration-test update. Layer-gate criterion #1 then closes cleanly at `cargo test` (default invocation), not just the scoped `--test bookmarks` invocation.

2. **Update the TODO.md:150 Layer-gate criterion #1** to scope explicitly to `cargo test --test bookmarks + --test properties + --ignored` (the GREEN surfaces) AND explicitly name `cargo test --lib` as a known-failing surface pending Option-1 sweep + update README.md:51 to match. This is the spec-honest acknowledgment that the layer-gate criterion as currently written closes against a partial test surface — but it is a much weaker resolution than Option 1 because it bakes the unit-test-regression into the spec contract.

Option 1 is the SO recommendation. The fix is small (2 string literals); Round 3 verification closes it cleanly per the R5 F1 precedent.

**Classification:** Backlogged — Layer 3 layer-gate criterion #1 cannot close honestly without resolution (the README + CHANGELOG accuracy claim depends on `cargo test` passing, not just the scoped invocation); operator-decision-required between Option 1 (update assertions) and Option 2 (amend criterion + README). Carries forward to Round 3; if Round 3 finds Option 1 applied + `cargo test` passes, the finding closes as Resolved.

---

### Resolved

<a id="r2-f2"></a>

**Finding 2 — Scope discipline confirmed: `display_safe` architectural correction sub-decision at Phase 2b landing stayed within the JSON-native-escape-design Round 1 routing operator intent + is honestly disclosed across DESIGN.md + CHANGELOG.md + TODO.md Phase 2c annotation (Dim 2 + Dim 6 + Dim 8 prior-review-additions)**

**Owner:** solution-owner
**Status:** validated
**Blocked by:** *(none)*
**Validator:** vdd-iar-alignment

The operator-supplied per-domain prompt asks whether the architectural correction (removing `display_safe` from `export_json` and delegating to serde_json's native encoder) was within scope of the Round 1 routing operator-intent (byte-preservation) or scope-creep (a substantive spec change without dedicated AskUserQuestion).

SO seat walked the audit trail: the Phase 4 routing record for the JSON-native escape design (4-domain convergence: SA+SE+RT+Sec) at [per-domain Phase 4 routing appendices:26-44` (per-domain Phase 4 appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md`) names the operator decision as "switch `display_safe` from Rust-syntax `\u{HHHH}` to JSON-native `\uHHHH` to preserve byte-round-trip." The operator intent stated explicitly: "Preserves both terminal-safety AND byte-round-trip." The Phase 2b implementation discovered mid-implementation that pre-escaping inside the JSON encoding path double-escapes (the literal `` text becomes `\\u001b` in JSON output and parses back as the 6-char text, NOT the original byte). The architectural correction — removing `display_safe` from `export_json` entirely + relying on serde_json's native encoder — is the only impl-path that achieves the operator's stated byte-preservation intent. The original routing's JSON-native-escape decision presumed `display_safe` would be CALLED at the export serialization step; the impl discovered that calling it there breaks the operator's stated goal.

**Three audit-trail surfaces confirm the correction is in-scope, not scope-creep:**

1. DESIGN.md:106 § `bm export` Success-output paragraph names the architectural correction explicitly: "**JSON-native escape design (Round 1 Phase 4 routing; SA + SE + RT + Sec 4-domain convergence + architectural correction sub-decision at Phase 2b landing):** the export path serializes `Bookmark` records via serde's native encoder; `display_safe` is NOT applied at the per-field serialization step because pre-escaping inside the JSON encoding path double-escapes..." — the spec contract now matches the impl AND names the trade-off (curated format chars survive as raw UTF-8 bytes in JSON output).
2. CHANGELOG.md:24-31 names the architectural correction at the commit-level audit trail: "Phase-2b-surfaced architectural correction sub-decision for the JSON-native escape design ... `BookmarkStore::export_json` architectural correction: serializes `Bookmark` records via serde's native encoder; `display_safe` is NOT applied at the per-field serialization step."
3. TODO.md:146 Phase 2c follow-up annotation names the architectural correction as "structural simplification, not a new refactor — the function got shorter + cleaner. No new helper-extraction opportunities surfaced during the Round 1 fix-work."

The operator's intent (byte-preservation via JSON-native escape) is preserved; the impl-path-change (where the escape happens — at serde-native encoding boundary vs. `display_safe` pre-wrap) is a load-bearing detail that needed to be visible at the spec contract. The triple-coded audit trail (spec + changelog + Phase 2c annotation) makes the correction visible to any cold reader. The trade-off (format chars survive raw in JSON) is named alongside the correction at every surface; downstream-consumer responsibility is named explicitly.

**The borderline case:** a stricter SO interpretation could classify this as scope-creep because the operator's AskUserQuestion did NOT explicitly authorize "remove `display_safe` from the export serialization step." The Phase 4 routing record presumes the JSON-native-escape decision wraps `display_safe` at the per-field step. The Phase 2b discovery that this breaks byte-round-trip is mid-implementation engineering work, not a contract-renegotiation. SO judgment: the operator's stated goal (byte-preservation) is load-bearing; the impl-path is the engineer's domain to discover the correct realization of the goal. The discovery + spec-update + CHANGELOG-disclosure pattern is exactly the Phase 2b discipline the methodology teaches.

**Reference-example purpose alignment ([G-112](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112)):** the architectural-correction-mid-Phase-2b worked example IS a methodology lesson worth teaching. A capstone+ reference example that discovers a spec/impl tension mid-implementation, names it explicitly, updates the spec contract, and discloses the correction across all three audit surfaces is the methodology in action — not noise. A future cold reader sees: "the operator authorized the JSON-native-escape decision; the engineer discovered that decision's literal interpretation broke the goal; the spec was updated to name the engineer's realization-of-the-goal; the audit trail names the correction explicitly so the next engineer cycle doesn't unwind it." That's the methodology's [G-156](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-156) developer-voice discipline applied to spec/impl tensions at Phase 2b boundary.

**Classification:** Resolved — the architectural correction stayed within operator intent; the audit trail is triple-coded; the trade-off is named; the worked-example teaches Phase-2b-engineering-discovery discipline cleanly. SO finds no scope-creep.

---

<a id="r2-f3"></a>

**Finding 3 — In-cycle suite-hardening scope discipline: the Review 94 meta-finding cycle + `check-no-letter-clusters.py` hook + primer 4 amendment landed in-PR-#52-scope per operator authorization at the lettering-4th-recurrence trigger; the load-bearing hook is the proportionate immediate enforcement; the remaining deferred items are correctly carry-forward-scoped to the post-PR-#52-merge suite-hardening PR (Dim 2 + Dim 6 + Dim 8)**

**Owner:** solution-owner
**Status:** validated
**Blocked by:** *(none)*
**Validator:** vdd-iar-alignment

The operator-supplied per-domain prompt asks whether the in-cycle suite-hardening (Review 94 + lettering hook + primer 4 amendment) is acceptable scope-creep given the load-bearing nature of the hook (catches commit-time violations) or should it have been a separate PR.

SO seat walked the audit trail per the suite-side [Review 94](../../../../vsdd-suite/suite-development/review-log/2026-05-24-suite-review.md#review-94--2026-05-25-0300z): the 3 deferred findings (Phase 4 bypass + phase-frequency gap + letter-label 4th-recurrence) all surfaced director-raised during PR #52's Layer 3 IAR cycle. Finding 3 (letter-label 4th-recurrence) triggered immediate in-cycle action because: (a) the recurrence count had reached 4 (PR #38 + PR #44 + Review 78 + PR #52 itself), which is the [Review 91 Finding 1](../../../../vsdd-suite/suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z)-codified earned-by-recurrence threshold for mechanical enforcement; (b) the hook is load-bearing — it catches commit-time violations going forward across all suite contributors, not just the operator's manual catch; (c) Findings 1 + 2 are correctly deferred per operator authorization ("Discussion only; defer the PR until after the IAR cycle closes" per Review 94 line 423) because they require primer/standard/template edits that constitute a separate methodology PR scope.

**Scope-creep test (the SO discipline):** a scope addition is creep when (a) it expands the PR's stated mission silently OR (b) it requires methodology debate that the original PR scope did not budget for. Test against PR #52's Layer 3 mission:

- Layer 3 mission = implement `bm export` + `bm import` + close 13-domain IAR Round 1 + Phase 4 routing + close Layer-gate criteria.
- Suite-hardening addition = lettering hook + primer 4 amendment.

Is the suite-hardening within scope? **Borderline.** The original PR #52 scope was project-side Layer 3 work; suite-side primer/hook edits are normally a separate PR per the [one-PR-at-a-time](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md) discipline. BUT:

1. The lettering recurrence happened DURING PR #52's Phase 4 routing record authoring (literal letter-cluster labels appeared in the routing record before the corrective rename); the catch + correction + codification fired against PR #52's own artifact. This is the canonical "discovery-during-execution → in-cycle codification" pattern PROCESS.md:86-94 Stumbling point 5 names.
2. The hook landing without the primer 4 amendment would leave the audit trail incomplete (the hook + rationale must land together so a future reader sees the hook + the methodology codification together).
3. The hook is mechanical enforcement against commit-time letter-label patterns — it does NOT change methodology semantics, only enforces existing methodology semantics that 4 prior recurrences proved were under-enforced. Adding mechanical-enforcement-of-existing-discipline is structurally similar to fixing a defect (Dim 1 spec coverage of an under-enforced discipline), not adding a new feature (Dim 2 scope-creep).

The deferred items (Phase 4 bypass + phase-frequency gap + Finding 3's follow-up co-authoring + stale-document layered defense) are correctly carry-forward-scoped — they require primer/standard/template authoring that would substantively expand PR #52 scope.

**No PR-scope-blast surface:** the in-cycle suite-hardening is bounded to (a) one new hook file + (b) one primer 4 amendment + (c) `.pre-commit-config.yaml` wiring. The bulk of suite-side methodology work is correctly deferred. The PR #52 scope-shape remains "Layer 3 Round 1 closure" with one load-bearing meta-hook addition; the PR title + PROCESS.md retrospective should name the in-cycle codification explicitly so the audit trail is honest (TW + DR cluster naturally surfaces this if it isn't).

**Reference-example purpose alignment ([G-112](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112)):** the in-cycle codification IS a methodology lesson worth teaching — "when a methodology recurrence reaches the earned-by-recurrence threshold mid-cycle, the load-bearing mechanical enforcement lands in-cycle to prevent the 5th recurrence; the substantive methodology authoring defers to the next cycle." A future cold reader sees the methodology applying its own [Review 91 Finding 1](../../../../vsdd-suite/suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z) earned-by-recurrence discipline against itself, which is exactly the meta-discipline the suite teaches.

**Classification:** Resolved — the in-cycle suite-hardening stayed within the operator-authorized scope (lettering 4th-recurrence trigger + load-bearing mechanical enforcement + deferred substantive methodology authoring); the audit trail is correctly recorded at Review 94 + the deferred items are honestly carry-forward-scoped. SO does not find unauthorized scope-creep.

---

<a id="r2-f4"></a>

**Finding 4 — Layer-gate criteria readiness for Layer 3 close: 3-of-6 criteria MET; 1 criterion BLOCKED by Finding 1; 1 criterion has known-operator-execution-pending status; 1 criterion is declared-not-applicable (Dim 5 layer-gate readiness)**

**Owner:** solution-owner
**Status:** validated
**Blocked by:** *(none — synthesis of the cross-criterion state)*
**Validator:** vdd-iar-alignment

The operator-supplied per-domain prompt asks for the layer-gate-criteria-status readout to close Layer 3.

Per TODO.md:148-155 § Layer 3 Layer-gate criteria, applied against post-`795bc25` state + Round 2 work:

| # | Criterion | Status |
|---|---|---|
| 1 | All Red Gate tests pass: `cargo test --test bookmarks` + `--ignored` + `--test properties` | **PARTIALLY MET — see Finding 1.** 51/51 `--test bookmarks` GREEN; 3/3 `--test properties` GREEN; `--ignored` scaling tests not re-run in this session (Layer 2 baseline preserved); **BUT `cargo test --lib` fails 2/13** AND `cargo test` (the README.md:51 default-invocation contract) fails. The criterion as literally worded ("`cargo test --test bookmarks`") closes; the README + cold-reader contract does not. |
| 2 | `cargo build --release` succeeds with no warnings | **MET.** Verified in-session: clean build, 0 warnings, 23.75s. |
| 3 | `manual-tests/layer-3.md` runs clean | **PARTIALLY MET.** File authored (16 steps; covers all ACs + Round 1 routed closures). Operator-execution of the 16 steps is the closure trigger; SO cold-session cannot execute the manual tests (Step 0 requires `cargo install --force` which mutates the operator's installed binary). Operator-execution pending. |
| 4 | Phase 3 IAR 13-domain MVR | **PENDING this Round 2 closing cycle.** Round 2 SO seat (this entry) is one of 13 parallel cold-session reviews; MVR determination requires the cross-domain finding-progression read at cycle close. |
| 5 | Phase 5 dispositions | **NOT YET REACHED.** Phase 5 cycle queued after Round 2 MVR per the TODO.md routing. The cargo-fuzz harness at `fuzz/fuzz_targets/import_stdin.rs` is named at TODO.md:142; not yet authored. |
| 6 | Phase 6 NA | **MET.** Declared not-applicable per DESIGN.md:17 + TODO.md:155 + the R4 F2 precedent. The Layer 1 project-terminal Phase 6 attestation stands. |

**Layer 3 cannot close at this Round 2 closure** without:
- Finding 1 resolution (criterion 1 honesty);
- Operator-execution of `manual-tests/layer-3.md` (criterion 3);
- Round 2 reaching MVR + Round 3 if Finding 1 opens with non-trivial fix work (criterion 4);
- Phase 5 cycle (criterion 5).

**No criteria are unreachable** — all 5 gating criteria have spec-honest closure paths. The layer-gate is on a recoverable progression.

**Classification:** Resolved — the layer-gate-criteria status is documented honestly; the gap-to-close is named at each criterion; no criterion is silently failing or silently dispositioned.

---

<a id="r2-f5"></a>

**Finding 5 — Round 1 fixes for the 7 Resolved + Backlogged findings hold against the post-fix state (regression-check pass) (Dim 5 + Dim 7)**

**Owner:** solution-owner
**Status:** validated
**Blocked by:** *(none)*
**Validator:** vdd-iar-alignment

Per [Review 1 above](#review-1--2026-05-25-0112z), Round 1 produced 7 findings (2 Backlogged + 5 Resolved). Regression-check against the post-fix state:

| Round 1 finding | Status this Round | Evidence |
|---|---|---|
| [R1 F1](#r1-f1) — `manual-tests/layer-3.md` absent (Backlogged Option-1 choice) | Closed | `manual-tests/layer-3.md` exists (572 lines; 16 steps) — see compliance-table SO F1 row |
| [R1 F2](#r1-f2) — README.md:9 stale "Layer 3 not built" (Backlogged Option-1 choice) | Closed | README.md:9 reflects post-Layer-3 state; phase-progression-table includes Layer 3 cycle status — see compliance-table SO F2 row |
| [R1 F3](#r1-f3) — 8 operator-confirmed decisions preserved at DESIGN.md + TODO.md (Resolved) | Holds | All 8 decisions still visible at the spec contract; no silent revisions or drops in the Round 1 fix-work commits |
| [R1 F4](#r1-f4) — Layer 3 ACs 14-28 match scope-and-non-goals (Resolved) | Holds | ACs 14-28 unchanged; the architectural correction at Phase 2b does NOT add ACs; the 6 new Phase 2a regression-and-coverage tests close routed findings without expanding spec surface |
| [R1 F5](#r1-f5) — capstone intent calibration holds; no production-intent trigger fires (Resolved) | Holds | The active control-char rejection (Round 1 routed mitigation) does NOT escalate to production-intent strict hardening; the impl mirrors the Layer 2 tag-injection accepted-risk framing extended to the new stdin attack surface; capstone-tier proportionality maintained |
| [R1 F6](#r1-f6) — Phase 5 + Phase 6 strategy declarations correctly calibrated (Resolved) | Holds | DESIGN.md:15 + :17 lines unchanged; both Phase 5 + Phase 6 lines for Layer 3 still satisfy [G-162](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-162) strict-form requirement |
| [R1 F7](#r1-f7) — AI-co-authored disclosure shape preserves clarity + spec-contract authority (Resolved) | Holds | DESIGN.md:47 disclosure paragraph unchanged; operator-ownership declaration still in force; CHANGELOG + PROCESS.md narrative consistent with the disclosure shape |

**No regression** against Round 1's Resolved findings. The 2 Backlogged findings closed cleanly per the operator-authorized Option-1 paths. The audit trail from Round 1 → Phase 4 routing → Round 1 fix-work → Round 2 verification is intact.

**Classification:** Resolved — all 7 Round 1 findings hold against the post-fix state.

---

### Hallucinated

*(none — the 5 findings above are concrete and citation-backed; no SO-dim concerns that turned out to be spec-misread emerged in this round)*

---

### Approved deviation

*(none — no pre-approved DESIGN.md deviations apply at this round)*

---

### Dismissed

*(none — every Layer 3 Round 2 spec commitment was either Met, Resolved (Findings 2-5), or Backlogged (Finding 1); no dismissable concerns)*

---

### Raised to SO

*(none — this IS the SO round; cross-domain findings that would route to SO are filed against their originating domain's log)*

---

### Summary

Five findings in Round 2:

- **Backlogged (operator-decision-required, blocks Layer 3 layer-gate criterion 1 honesty):**
  - [Finding 1](#r2-f1) — Round 1 `display_safe` JSON-native rewrite at `bfc0713` left 2 pre-existing unit tests asserting the old `\u{HHHH}` Rust-syntax form; `cargo test --lib` fails 2/13 + `cargo test` (the README.md:51 default-invocation contract) fails. Two resolution paths (update assertions to JSON-native form OR amend criterion + README to scope the layer-gate explicitly to the GREEN test surfaces).
- **Resolved:**
  - [Finding 2](#r2-f2) — `display_safe` architectural correction sub-decision at Phase 2b landing stayed within operator intent (byte-preservation goal preserved; impl-path-change is engineer-discovery, not contract-renegotiation); audit trail is triple-coded across DESIGN.md + CHANGELOG.md + TODO.md Phase 2c annotation; reference-example purpose extends honestly (Dim 2 + Dim 6 + Dim 8).
  - [Finding 3](#r2-f3) — In-cycle suite-hardening (Review 94 + lettering hook + primer 4 amendment) stayed within operator-authorized scope (lettering 4th-recurrence trigger + load-bearing mechanical enforcement + deferred substantive methodology authoring); no unauthorized scope-creep (Dim 2 + Dim 6 + Dim 8).
  - [Finding 4](#r2-f4) — Layer-gate criteria readiness: 3-of-6 MET (criteria 2 + 3-partially + 6) + 1 BLOCKED by Finding 1 (criterion 1) + 1 PENDING-this-cycle-close (criterion 4) + 1 NOT-YET-REACHED (criterion 5); no criterion silently failing (Dim 5).
  - [Finding 5](#r2-f5) — All 7 Round 1 findings hold against the post-fix state; no regression introduced by the Round 1 fix-work commits at the spec or audit-trail surfaces (Dim 5 + Dim 7).

**Operator-supplied per-domain prompt answers (summarized for the audit trail):**

1. _"R1 Backlogged closure verification — does manual-tests/layer-3.md satisfy the criterion-3 gate? does the README update accurately reflect post-L3 state?"_ — File exists + 16 steps cover all ACs + Round 1 routed closures + hyperfine sanity-check per [Finding 4](#r2-f4) criterion 3 row; operator-execution pending. README is accurate to post-Layer-3-mid-cycle state per [Finding 5](#r2-f5) R1 F2 row.
2. _"5 SO-decidable Round 1 findings — implementation alignment"_ — All 5 implemented per operator decision; no misimplementations; no partial implementations. See compliance table rows for JSON-native escape design + sorted-tag-comparison dedup + manual-tests authoring + control-char rejection + classification extension.
3. _"Architectural correction sub-decision — scope discipline"_ — Within scope of the Round 1 routing decision per [Finding 2](#r2-f2); operator's stated goal (byte-preservation) is preserved; the impl-path-change is engineer-discovery + correctly disclosed at three surfaces.
4. _"In-cycle suite-hardening — scope discipline"_ — Acceptable scope per [Finding 3](#r2-f3); the load-bearing mechanical-enforcement-of-existing-discipline justifies in-cycle landing; deferred substantive methodology authoring is correctly carry-forward-scoped.
5. _"Layer-gate criteria readiness — readiness to close Layer 3"_ — Per [Finding 4](#r2-f4): criteria 2 + 6 MET cleanly; criterion 1 PARTIALLY MET (blocked by [Finding 1](#r2-f1)); criterion 3 PARTIALLY MET (file authored; operator-execution pending); criterion 4 PENDING this Round 2 cycle close; criterion 5 NOT YET REACHED (Phase 5 cycle queued).
6. _"Reference-example purpose alignment — does the Round 1 + Round 2 cycle teach the methodology cleanly to a future reader?"_ — Yes. The architectural-correction-at-Phase-2b worked example + the in-cycle-suite-hardening-at-recurrence-threshold worked example both teach load-bearing methodology disciplines; the audit-trail-noise concern resolves favorably because the triple-coded disclosure pattern makes the corrections honest, not chaotic. [G-112](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112) preserved.

**Coordination:** [Finding 1](#r2-f1) routes naturally to SE for the unit-test assertion update (2-line fix) + QE for the test-discipline-sweep-completeness validation. [Finding 2-5](#r2-f2) document the spec-clean + scope-clean + audit-trail-clean state for the Layer-3-close-readiness + future-cycle regression-check baseline.

**Phase 5 / Phase 6 closure-blocker check:** [Finding 1](#r2-f1) blocks Layer 3 layer-gate criterion #1 honesty + the README/CHANGELOG accuracy claim. The fix is small (2 string literals); if Round 3 finds Option 1 applied, the finding closes cleanly per the R5 F1 precedent. Phase 5 still ahead per criterion #5.

**Cost-tally** (per AIE F7 carry-forward; agent-self-verifiable tier only per [Review 91 F8](../../../../vsdd-suite/suite-development/review-log/2026-05-23-suite-review.md#review-91--2026-05-23-1900z)):

- **AI tool / Model / Execution method:** [claude-code CLI](https://claude.com/claude-code) / `claude-opus-4-7` / cold sub-agent spawn (cold-context for the SO Round 2 seat)
- **Wall-clock anchors (Bash `date -u`):** session-start observed at 2026-05-25T03:07Z; session-end at commit-prep step (main session commits)
- **Tool-call counts by tool name (agent-self-verifiable):** Bash invocations ~14 (git log + ls + wc + cargo test [3] + cargo build [1] + cargo clippy [1] + date + grep [3] + file listings); Read invocations ~13 (SO domain prompt; primer 3; SO Review 1; Phase 4 routing; DESIGN.md [2 ranges]; TODO.md; README.md; manual-tests/layer-3.md; src/lib.rs [3 ranges]; src/main.rs [2 ranges]; PROCESS.md; suite Review 94); Edit invocations 1 (this entry append)
- **Files read (with approximate line counts):** SOLUTION-OWNER-REVIEW.md (60); 3-review-session.md (241); 2026-05-24-solution-owner.md Review 1 (~318); per-domain Phase 4 routing appendices (398); DESIGN.md (336 across 2 ranges); TODO.md (155); README.md (97); manual-tests/layer-3.md (572); src/lib.rs (~950 across 3 ranges); src/main.rs (~370 across 2 ranges); PROCESS.md (~100); 2026-05-24-suite-review.md Review 94 (~90)
- **Files written/edited (with line counts from Edit):** this Review 2 entry (~270 lines appended to 2026-05-24-solution-owner.md)
- **Mechanical sweeps run:** `cargo test --test bookmarks` (51 passed); `cargo test --test properties` (3 passed); `cargo test --lib` (11 passed, 2 failed — surfaced [Finding 1](#r2-f1)); `cargo test` (default; 2 failures); `cargo build --release` (clean); `cargo clippy --all-targets --all-features` (clean); `git log --oneline -30` (commit-sequence verification); `wc -l` on 9 key files
- **Plan tier:** *pending operator confirmation per session (do NOT inherit silently from prior context)*
- **Raw tokens:** *pending operator `/cost` paste*
- **Would-be API cost:** *pending operator `/cost` paste*
- **Actual cost to operator:** *pending operator declaration*
- **Rate-limit-window utilization:** *pending operator `/cost` paste*
- **Findings/100k tokens:** *NOT COMPUTABLE — pending operator `/cost` paste*

**Operator-action queue:** if cost-tally precision is load-bearing for cross-cycle calibration, operator runs `/cost` in this session and pastes the output here as an append-only addendum, replacing the *pending operator …* placeholders with measured values.

**Validator:** vdd-iar-alignment — VDD-IAR Alignment confirms the [Finding 1](#r2-f1) resolution path (unit-test assertion update OR layer-gate criterion amendment) routes through the spec/test surface correctly + doesn't conflict with prior intent; [Findings 2 + 3 + 4 + 5](#r2-f2) document spec-clean + scope-clean + layer-gate-readiness state for the cross-cycle regression-check baseline).

---

---

## Phase 4 routing — Round 1 (2026-05-25 02:00Z)

Per [`vsdd-suite/primers/4-feedback-integration.md`](../../../../vsdd-suite/primers/4-feedback-integration.md) § [manual] First-class fallback path. SO-decisions captured via main-session AskUserQuestion pass on 2026-05-25 across the cross-domain finding clusters. This appendix lists this domain's routable findings in the primer-4-canonical per-finding shape; cross-domain coordination signals live in each Round 1 finding's `**Coordination:**` line. Cross-cluster sequencing matrix lives in the commit message + the CHANGELOG slim-form entry that recorded this Phase 4 pass (refactored from a prior consolidated routing record per operator directive 2026-05-25 — the consolidated file was an anti-pattern; primer-4-canonical is per-domain appendices).

#### Finding `r1-f1` — manual-tests/layer-3.md absent despite TODO.md:138 spec commitment — ROUTED

**Cluster:** manual-tests/layer-3.md authoring
**Route:** `Phase 2a-equivalent artifact authoring`
**Gate:** (see DR R1 F3 routing — same cluster; operator decided: author the file)
**Sequencing:** Blocks Layer 3 layer-gate close (criterion 3)

#### Finding `r1-f2` — README:9 still says Layer 3 scoped but not built against post-Phase-2b state — ROUTED

**Cluster:** README post-Layer-3 update
**Route:** `Phase 1a+1b`
**Gate:** (see DR R1 F1 + TW R1 F1 routings — same cluster)
**Sequencing:** Should land before Layer 3 gate close
