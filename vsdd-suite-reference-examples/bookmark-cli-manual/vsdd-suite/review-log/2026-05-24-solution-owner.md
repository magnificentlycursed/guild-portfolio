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

**Validator:** vdd-iar-alignment (the SO ↔ VDD-IAR Alignment validator pair per [Review 77](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-77--2026-05-20-1545z) — VDD-IAR Alignment confirms the spec changes (the Finding 1 + Finding 2 resolution paths each route through TODO.md / README.md amendments) went through proper routing and don't conflict with prior intent).

---
