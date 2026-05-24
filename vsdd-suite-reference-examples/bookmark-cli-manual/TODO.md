# TODO.md — bookmark-cli-manual

[Phase 1c](../../vsdd-suite/primers/1c-decomposition.md) output. Authored with [`../../vsdd-suite/primers/1c-decomposition.md`](../../vsdd-suite/primers/1c-decomposition.md) loaded ([G-96](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-96) renamed Phase 1b → Phase 1c; this file predates the rename and is preserved per [G-89](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-89) forward-only narrative-preservation policy). Each layer below is independently testable and shippable; acceptance criteria are observable behaviors (not implementation steps); the Red Gate test plan names the literal tests that must fail before any implementation lands per layer.

This project is the manual-method reference implementation for the suite's worked example ([G-112](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112) in [`../../vsdd-suite/suite-development/FINDINGS-INDEX.md`](../../vsdd-suite/suite-development/FINDINGS-INDEX.md)). **As of PR 6 / Review 78, bookmark-cli-manual is at `capstone` intent** with all 6 VSDD (Verified Spec-Driven Development) phases demonstrated end-to-end (1a+1b spec → 1c decomposition → 2a Red Gate → 2b implementation → 2c refactor → 3 IAR (Iterative Adversarial Refinement) (13 active domains) → 4 routing → 5 Purity Boundary Audit + Mutation Testing hardening → 6 four-dimensional convergence). Layer 1 closed project-terminal at PR #42; Layer 2 (tag + filter) is the active layer per the post-PR-#43 cycle. Layer 3 (export + import) remains scoped only.

---

## Layer 1 — Add and List

**Status:** Layer 1 code-complete; [Phase 3](../../vsdd-suite/primers/3-review-session.md) IAR Round 1 + Round 2 cold-session cycles closed in [Review 82](../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z). [Phase 6](../../vsdd-suite/primers/6-convergence.md) four-dimensional convergence DEFERRED pending Round 3 fix cycles for the 8 non-MVR domains + operator-runs-install-verification (Platform Engineer Dim 38).

**Acceptance criteria** (observable behaviors from outside the binary):

- **AC 1:** `bm add <url>` creates a bookmark record with the current UTC timestamp; exits 0; stdout silent.
- **AC 2:** `bm add` (no positional argument, or empty-string argument) exits 1 with stderr `Error: URL cannot be empty.\n` and writes nothing to the store.
- **AC 3:** `bm list` prints all bookmarks newest-first to stdout, one per line in format `<RFC3339-timestamp> <url>`; exits 0.
- **AC 4:** `bm list` against an empty or absent store exits 0 with stderr `No bookmarks yet.\n` and stdout silent.

**Red Gate test plan** (per `primers/2a-red-gate.md` — these tests must fail before any implementation in `src/lib.rs` or `src/main.rs` is written):

- `tests_add_creates_bookmark` — invokes `bm add https://example.com` against a temp BOOKMARK_CLI_DB, asserts exit 0, asserts the temp file now contains a bookmark with that URL and a parseable RFC 3339 timestamp.
- `tests_add_rejects_empty_url` — invokes `bm add ""` against a temp DB, asserts exit 1, asserts stderr matches `Error: URL cannot be empty.\n` exactly, asserts the temp file is not created.
- `tests_list_orders_newest_first` — invokes two `bm add` calls with a small sleep between them, then `bm list`, asserts the second URL appears before the first in stdout.
- `tests_list_empty_state` — invokes `bm list` against an absent temp DB, asserts exit 0, asserts stderr matches `No bookmarks yet.\n`, asserts stdout is empty.

All four tests live in `tests/bookmarks.rs` and invoke the compiled binary via `assert_cmd` per CLI supplement § Quality Engineering ("integration tests invoke the binary"). Each uses `tempfile::tempdir()` for an isolated `BOOKMARK_CLI_DB` per test — no shared state between tests.

**Manual Testing Checklist:** [`manual-tests/layer-1.md`](manual-tests/layer-1.md)

(Per Review 74 manual-test split convention — the full per-step plan with runnable commands + literal expected-output blocks lives in the linked per-layer file. Migrated from the prior inline-in-TODO.md shape as part of PR 6's capstone-intent promotion.)

**[Phase 2c](../../vsdd-suite/primers/2c-refactor.md) (refactor):** `no refactor required` (per `../../vsdd-suite/primers/2c-refactor.md` § Completion criteria #5 explicit-skip annotation). Layer 1's implementation (`src/lib.rs` + `src/main.rs`) is small enough that the [Phase 2b](../../vsdd-suite/primers/2b-implementation.md) artifact already exhibits the idiomatic [Rust](https://www.rust-lang.org/) patterns the refactor primer's scope catalog targets (extract-and-name; collapse-and-inline; reshape-data-flow; surface-purity-boundary; idiomatic-alignment; language-supplement rules). The purity boundary was explicitly surfaced by SA Review 1 ([Phase 5](../../vsdd-suite/primers/5-formal-hardening.md) Purity Boundary Audit); no further refactor warranted. The explicit-skip annotation here satisfies [VDD-IAR Alignment](../../vsdd-suite/domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md) dim 12 (Phase 2c refactor discipline per [G-161](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-161)) — silent-skip would be a finding; this annotation is the alternative.

**Layer-gate criteria:**

1. All four Red Gate tests pass: `cargo test --test bookmarks`.
2. `cargo build --release` succeeds with no warnings.
3. The manual testing checklist at `manual-tests/layer-1.md` runs clean (every step produces the expected exit/stdout/stderr).
4. [Phase 3](../../vsdd-suite/primers/3-review-session.md) IAR (Iterative Adversarial Refinement) reviews complete for the **capstone-active domain set** per `../../vsdd-suite/domains/DOMAIN-INDEX.md` § Intent calibration: 7 cores (SE, QE, [UX](../../vsdd-suite/domains/role/UX-REVIEW.md), [Security](../../vsdd-suite/domains/role/SECURITY-REVIEW.md), SA, SO, [VDD-IAR Alignment](../../vsdd-suite/domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md)) + capstone-tier extended ([Performance Engineer](../../vsdd-suite/domains/role/PERFORMANCE-ENGINEER-REVIEW.md), [Red Team](../../vsdd-suite/domains/role/RED-TEAM-REVIEW.md), [Platform Engineer](../../vsdd-suite/domains/role/PLATFORM-ENGINEER-REVIEW.md), [Technical Writer](../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md), [Documentation Reviewer](../../vsdd-suite/domains/role/DOCUMENTATION-REVIEWER-REVIEW.md), [AI Engineer](../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md)); each domain reaches MVR (maximum viable refinement) or zero-findings. ([Data Engineer](../../vsdd-suite/domains/role/DATA-ENGINEER-REVIEW.md) evaluated and ruled out — bookmark-cli's flat JSON storage falls below the activation threshold per [G-178](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-178); the absence is documented as deliberate.)
5. Phase 5 Purity Boundary Audit + Mutation Testing both at closure with the per-domain log preambles per [G-177](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-177) — Purity Boundary Audit in SA Review 1; Mutation Testing in QE Review 2.
6. [Phase 6](../../vsdd-suite/primers/6-convergence.md) four-dimensional convergence record landed as the final VDD-IAR Alignment review round titled "Review N — Phase 6 four-dimensional convergence (project-terminal)" per primer 6 + [G-177](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-177).

---

## Layer 2 — Tag and filter

**Status:** Active per post-PR-#43 cycle. Promoted from "deferred — scoped only" to capstone-active per operator directive after Layer 1 reached project-terminal MVR at PR #42. The Layer 2 cycle closes three Layer-1 Deferred-to-Layer-2 items: [Performance Engineer Review 1 Finding 2](vsdd-suite/review-log/2026-05-20-performance-engineer.md) (benchmarking infrastructure → hyperfine sanity-check at `manual-tests/layer-2.md`), Finding 5 (data-scaling sentinel tests at 100/1,000/10,000-bookmark cliffs in `tests/scaling.rs`), and the operator-queued fsync benchmark item (parent-directory `fsync` after `rename(2)` for durability).

**Acceptance criteria** (observable behaviors from outside the binary):

- **AC 5:** `bm tag <url> <label>` against a store where one or more bookmarks have `url` exactly: appends `<label>` to each matching bookmark's `tags` field (idempotent — second invocation with the same arguments produces an identical post-state); exits 0; stdout silent.
- **AC 6:** `bm tag <url> <label>` against a store where no bookmark has `url`: exits 1 with stderr `Error: no bookmark found with URL <url>.\n`; the store file is not rewritten.
- **AC 7:** `bm tag` with empty URL (both `bm tag "" <label>` and `bm tag` missing the URL positional): exits 1 with stderr `Error: URL cannot be empty.\n`; no file write. (Mirrors the `bm add` empty-URL contract per DESIGN.md.)
- **AC 8:** `bm tag` with empty label (both `bm tag <url> ""` and `bm tag <url>` missing the label positional): exits 1 with stderr `Error: tag label cannot be empty.\n`; no file write.
- **AC 9:** `bm list --tag <label>` returns only the bookmarks whose `tags` field contains `<label>`, in newest-first ordering, exits 0. Empty filter result: stderr `No bookmarks match the supplied filter.\n`, exit 0 (filter empty-state is distinct from store empty-state).
- **AC 10:** `bm list --tag <label1> --tag <label2>` returns the OR-union of bookmarks matching either label (a bookmark with `["label1"]` matches; with `["label2"]` matches; with `["label1", "label2"]` matches; with `["label3"]` does not match). Newest-first ordering preserved; exit 0.
- **AC 11:** `bm list --tag ""`: exits 1 with stderr `Error: tag label cannot be empty.\n` (parallel to AC 8 input invariant).
- **AC 12:** `bm tag` against a Layer-1-format store (bookmarks without `tags` field): the missing field deserializes to empty `Vec<String>`; the post-tag save emits the field for every bookmark (touched and untouched alike) — forward-only migration shape.
- **AC 13:** Durability: every `bm add` and `bm tag` save fsyncs the destination file's parent directory after `rename(2)` (Unix; `#[cfg(unix)]`-gated). A `bm list` immediately after a successful `bm tag` reflects the change even after a synthesized power-fail (the rename has crossed the durability boundary).

**Red Gate test plan** (per `primers/2a-red-gate.md` — these tests must fail before any implementation lands; tests are added to `tests/bookmarks.rs` alongside the existing Layer 1 tests):

- `tests_tag_attaches_label_to_matching_bookmark` — adds a bookmark, invokes `bm tag <url> rust`, asserts exit 0 + stderr empty + store file contains the URL bookmark with `tags: ["rust"]`.
- `tests_tag_is_idempotent` — adds a bookmark, invokes `bm tag <url> rust` twice, asserts second invocation exits 0 + the bookmark's `tags` is exactly `["rust"]` (not `["rust", "rust"]`).
- `tests_tag_rejects_unknown_url` — adds bookmark A, invokes `bm tag B nonsense`, asserts exit 1 + stderr matches `Error: no bookmark found with URL B.\n` exactly + the store file is byte-identical to its pre-invocation state.
- `tests_tag_rejects_empty_url` — adds a bookmark, invokes `bm tag "" rust`, asserts exit 1 + stderr matches `Error: URL cannot be empty.\n`.
- `tests_tag_rejects_empty_label` — adds a bookmark, invokes `bm tag <url> ""`, asserts exit 1 + stderr matches `Error: tag label cannot be empty.\n`.
- `tests_tag_against_layer_1_format_file_migrates_forward` — writes a Layer-1-format JSON file directly (no `tags` field per bookmark), invokes `bm tag <url> rust`, asserts exit 0 + the post-write file contains explicit `tags: ["rust"]` on the tagged bookmark AND explicit `tags: []` on every other bookmark.
- `tests_tag_against_duplicate_url_tags_all_matches` — adds the same URL twice (append-only permits this), invokes `bm tag <url> rust`, asserts exit 0 + both bookmarks have `tags: ["rust"]` post-save.
- `tests_list_with_tag_filter_returns_matching_bookmarks` — adds three bookmarks A/B/C, tags A and C with `rust`, invokes `bm list --tag rust`, asserts exit 0 + stdout matches `<ts-C> C\n<ts-A> A\n` (newest-first) + stderr empty.
- `tests_list_with_tag_filter_empty_match_emits_filter_empty_state` — adds two bookmarks (untagged), invokes `bm list --tag rust`, asserts exit 0 + stdout silent + stderr matches `No bookmarks match the supplied filter.\n`.
- `tests_list_with_tag_filter_repeated_flag_is_or_semantics` — adds three bookmarks A/B/C, tags A with `rust` and B with `go`, invokes `bm list --tag rust --tag go`, asserts exit 0 + stdout contains both A and B (newest-first) but not C.
- `tests_list_with_tag_filter_against_empty_store_emits_store_empty_state` — invokes `bm list --tag rust` against absent store, asserts exit 0 + stderr matches `No bookmarks yet.\n` (store empty-state takes precedence over filter empty-state per DESIGN.md edge-case catalog).
- `tests_list_with_empty_tag_label_rejected` — adds a bookmark, invokes `bm list --tag ""`, asserts exit 1 + stderr matches `Error: tag label cannot be empty.\n`.
- `tests_list_rfc3339_scripted_check` (closes Layer-1-Deferred QE item) — adds three bookmarks with small delays, invokes `bm list`, asserts every emitted timestamp matches the RFC 3339 grammar at byte level via a `chrono::DateTime::parse_from_rfc3339` round-trip — not merely a regex eyeball. The Red Gate failure mode is intentional ambiguity in the Layer-1 implementation (any deviation from strict RFC 3339 — missing-`Z`, ambiguous-offset, sub-microsecond precision drift — is a finding).
- `tests_save_durable_path_succeeds_unix_weak_proxy_for_fsync` (closes operator-queued PE fsync item structurally; SE R1 F4 carry-forward rename applied at PR #46 from prior overclaiming name `tests_save_fsyncs_parent_directory`) — adds a bookmark, asserts the durable-`save` codepath (including parent-dir fsync on Unix) executes successfully against a real filesystem and that the saved file round-trips through `load`. The WEAK PROXY framing per the test's docstring + the renamed function name: no portable userspace test can directly assert the `fsync(2)` syscall was issued, so the test verifies the codepath executes without panic + the saved file is correct. Direct verification (`strace`/`dtruss` harness or injected-seam) is deferred per SE R1 F4 Carry-forward disposition.

**Layer 2 manual testing checklist:** [`manual-tests/layer-2.md`](manual-tests/layer-2.md) — parallel to `manual-tests/layer-1.md`. Includes the `hyperfine` sanity-check sub-section that closes Layer-1-Deferred [Performance Engineer Review 1 Finding 2](vsdd-suite/review-log/2026-05-20-performance-engineer.md).

**Layer 2 data-scaling tests:** `tests/scaling.rs` with `#[ignore]`-gated sentinels at 100/1,000/10,000 bookmark cliffs. Asserts the budget table in DESIGN.md § Performance budget holds against programmatically-generated stores. CI runs `cargo test -- --ignored` in a separate job so the `cargo test` default stays fast. Closes Layer-1-Deferred [Performance Engineer Review 1 Finding 5](vsdd-suite/review-log/2026-05-20-performance-engineer.md).

**[Phase 2c](../../vsdd-suite/primers/2c-refactor.md) (refactor):** **extract-and-name applied** at Phase 2b commit `326e25d` (per `vsdd-suite/primers/2c-refactor.md` § Scope catalog). The Phase 2b implementation extracted three per-subcommand helpers — `run_add`, `run_list`, `run_tag` — from `src/main.rs`'s single `match cli.command { ... }` block in `main()`. The trigger was clippy's `too_many_lines` lint at the `pedantic` floor (the consolidated `main()` reached ~103 lines vs. the 100-line limit), but the refactor is justified independent of the lint floor: each helper now reads as a complete top-to-bottom subcommand contract (load → validate → mutate → save with named error routing), which improves audit-trail readability for the Phase 3 IAR cluster reviewers and makes per-subcommand unit-test seams reachable from a future test layer. No further refactor warranted at Phase 2c — the three helpers share the same load-store-emit pattern in 3 lines apiece, and a `load_store_or_emit` helper would obscure the per-subcommand control flow without reducing line count materially (per the suite's "three similar lines is better than a premature abstraction" discipline). Phase 2c satisfies [VDD-IAR Alignment](../../vsdd-suite/domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md) dim 12 per [G-161](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-161) — the extract-and-name annotation here is the alternative to a silent-skip finding.

**Red Gate evidence-preservation annotation (Layer 2 Round 1 VDD-IAR Alignment R4 F1).** Layer 2's Phase 2a + Phase 2b landed in the SINGLE commit `326e25d`, which means the Red Gate failure evidence (12 of the 13 new tests failing correctly against the unmodified Layer 1 binary with `error: unrecognized subcommand 'tag'`) lives in the Phase 2b sub-agent's spawn-output report at commit time, NOT in git history as a separate Phase 2a-only commit. This is a methodology-audit-trail tradeoff — the convenience of a single commit at Phase 2b landing time was prioritized over the audit-trail discipline of a two-commit Phase 2a + Phase 2b shape. **For future Layer cycles** (Layer 3, future projects): the canonical shape is **two commits** — one for the Phase 2a Red Gate (failing tests committed alone; CI confirms RED) and a second for the Phase 2b implementation (the same tests pass; CI confirms GREEN). This Layer 2 annotation documents the precedent so the next cycle's operator + sub-agents know to apply the discipline; Round 1 VDD-IAR Alignment R4 F1 surfaced the gap + this paragraph is the closure.

**Layer-gate criteria:**

1. All Red Gate tests above pass: `cargo test --test bookmarks` + `cargo test -- --ignored` (scaling).
2. `cargo build --release` succeeds with no warnings.
3. The manual testing checklist at `manual-tests/layer-2.md` runs clean (every step produces the expected exit/stdout/stderr).
4. [Phase 3](../../vsdd-suite/primers/3-review-session.md) IAR reviews complete for the 13-domain capstone-active set per the post-PR-#39 DOMAIN-INDEX.md § Intent calibration; each domain reaches MVR or zero-findings. ([Data Engineer](../../vsdd-suite/domains/role/DATA-ENGINEER-REVIEW.md) re-evaluated for Layer 2: still ruled out — tags-as-`Vec<String>` is still flat JSON below the [G-178](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-178) activation threshold.)
5. Phase 5 Layer 2 rounds at closure: Purity Boundary Audit re-runs against the extended pure surface; Mutation Testing re-runs against the extended impl with 100% kill rate maintenance or named-rationale drop; property-based testing via [`proptest`](https://github.com/proptest-rs/proptest) activated at [`tests/properties.rs`](tests/properties.rs) against the tag-idempotence + filter-OR-monotonicity properties.
6. **[Phase 6](../../vsdd-suite/primers/6-convergence.md) not applicable** per [DESIGN.md § Project intent](DESIGN.md#project-intent) Phase 6 strategy declaration ([G-150](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-150) over-investment guard + [G-112](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112) reference-implementation-purpose-already-satisfied). Layer 1's Phase 6 attestation at [VDD-IAR Alignment Review 3](vsdd-suite/review-log/2026-05-20-vdd-iar-alignment.md) stands as the project's terminal four-dimensional convergence record; the reference-implementation purpose (exercise all six VSDD phases end-to-end as a worked example) is satisfied by Layer 1's project-terminal MVR. Re-running Phase 6 for Layer 2 would mis-teach the methodology by suggesting capstone artifacts require per-layer four-dimensional convergence — capstone gates at project-terminal MVR per primer 6, not per-layer. Closes Layer 2 Round 1 VDD-IAR Alignment R4 F5 + Solution Owner R4 F2 (the cluster's own SO recommended Option 1: mark not-applicable).

---

## Layer 3 — Export and import (deferred)

**Status:** Scoped only. Not in scope for this reference implementation.

**Acceptance criteria sketch:**
- `bm export` emits all bookmarks as JSON to stdout
- `bm import` reads bookmarks from stdin and merges them into the store

**Why deferred:** same as Layer 2.
