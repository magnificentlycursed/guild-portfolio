# TODO.md — bookmark-cli-manual

Phase 1c output. Authored with [`../../vsdd-suite/primers/1c-decomposition.md`](../../vsdd-suite/primers/1c-decomposition.md) loaded (G-96 renamed Phase 1b → Phase 1c; this file predates the rename and is preserved per G-89 forward-only narrative-preservation policy). Each layer below is independently testable and shippable; acceptance criteria are observable behaviors (not implementation steps); the Red Gate test plan names the literal tests that must fail before any implementation lands per layer.

This project is the manual-method reference implementation for the suite's worked example (G-112 in [`../../vsdd-suite/suite-development/FINDINGS-INDEX.md`](../../vsdd-suite/suite-development/FINDINGS-INDEX.md)). Only Layer 1 is built out — Layers 2 and 3 are scoped but deferred to follow-on work. **As of PR 6 / Review 78, bookmark-cli-manual is at `capstone` intent** with all 6 VSDD phases demonstrated end-to-end (1a+1b spec → 1c decomposition → 2a Red Gate → 2b implementation → 2c refactor (no-refactor annotation) → 3 IAR (10 active domains) → 4 routing → 5 Surfaces A.0+B hardening → 6 four-dimensional convergence).

---

## Layer 1 — Add and List

**Status:** In progress (Phase 2a → 2b in the reference-implementation session).

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

**Phase 2c (refactor):** `no refactor required` (per `../../vsdd-suite/primers/2c-refactor.md` § Completion criteria #5 explicit-skip annotation). Layer 1's implementation (`src/lib.rs` + `src/main.rs`) is small enough that the Phase 2b artifact already exhibits the idiomatic Rust patterns the refactor primer's scope catalog targets (extract-and-name; collapse-and-inline; reshape-data-flow; surface-purity-boundary; idiomatic-alignment; language-supplement rules). The purity boundary was explicitly surfaced by SA Review 1 (Phase 5 purity-boundary verification); no further refactor warranted. The explicit-skip annotation here satisfies VDD-IAR Alignment dim 12 (Phase 2c refactor discipline per G-161) — silent-skip would be a finding; this annotation is the alternative.

**Layer-gate criteria:**

1. All four Red Gate tests pass: `cargo test --test bookmarks`.
2. `cargo build --release` succeeds with no warnings.
3. The manual testing checklist at `manual-tests/layer-1.md` runs clean (every step produces the expected exit/stdout/stderr).
4. Phase 3 IAR reviews complete for the **capstone-active domain set** per `../../vsdd-suite/domains/DOMAIN-INDEX.md` § Intent calibration: 7 cores (SE, QE, UX, Security, SA, SO, VDD-IAR Alignment) + capstone-tier extended (Performance Engineer, Red Team, Platform Engineer, Technical Writer); each domain reaches MVR or zero-findings. (Data Engineer evaluated and ruled out — bookmark-cli's flat JSON storage falls below the activation threshold per G-178; the absence is documented as deliberate.)
5. Phase 5 Surfaces A.0 (purity boundary) + B (mutation testing) both at closure with the per-domain log preambles per G-177 — purity-boundary verification in SA Review 1; mutation testing in QE Review 2.
6. Phase 6 four-dimensional convergence record landed as the final VDD-IAR Alignment review round titled "Review N — Phase 6 four-dimensional convergence (project-terminal)" per primer 6 + G-177.

---

## Layer 2 — Tag and filter (deferred)

**Status:** Scoped only. Not in scope for this reference implementation.

**Acceptance criteria sketch:**
- `bm tag <bookmark-index> <label>` attaches a label
- `bm list --tag <label>` filters by label
- Multiple labels per bookmark allowed; comma-separated input or repeated `--tag` flag

**Why deferred:** the reference-implementation purpose (G-112) is satisfied by Layer 1 alone — one layer end-to-end through the suite proves the worked example pattern.

---

## Layer 3 — Export and import (deferred)

**Status:** Scoped only. Not in scope for this reference implementation.

**Acceptance criteria sketch:**
- `bm export` emits all bookmarks as JSON to stdout
- `bm import` reads bookmarks from stdin and merges them into the store

**Why deferred:** same as Layer 2.
