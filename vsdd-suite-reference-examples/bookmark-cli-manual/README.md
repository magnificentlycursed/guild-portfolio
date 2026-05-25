# bookmark-cli

A single-user command-line tool for capturing URLs at the terminal and recalling them later. `bm add https://example.com` saves a URL with a timestamp; `bm list` prints all saved URLs newest-first.

## What this is

`bookmark-cli` is the **reference implementation** for the [VSDD (Verified Spec-Driven Development) Suite](../../vsdd-suite/README.md)'s worked example — it exists to validate the suite's documented workflow end-to-end ([G-112](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112) in the suite's gap registry). It is small by design and intentionally limited in scope. A user who wants a real bookmark manager should use a browser or a dedicated tool; this is a portfolio demonstration artifact.

Current state: **Layer 1 project-terminal at PR #42** (add + list) + **Layer 2 layer-terminal at PR #47** (tag + filter) + **Layer 3 active in PR #52** (export + import). Layer 3 is Phase-2-complete (Phase 2a Red Gate `878d3b6` + Phase 2b implementation `fd21900` + Phase 2c annotation `78bd3cf`); Phase 3 IAR Round 1 closed at commit `2acc418` with 76 findings across 13 capstone-active domains; Phase 4 routing pass landed at commit `e233ad8`; Round 1 fix work is in flight per the [`vsdd-suite/review-log/2026-05-24-phase-4-routing.md`](vsdd-suite/review-log/2026-05-24-phase-4-routing.md) routing record. Layer 3 cycle iterates Round 2 IAR after fix work lands.

## Prerequisites

- [Rust](https://www.rust-lang.org/) 1.78+ (`cargo --version` to check)
- macOS or Linux (Windows untested)

## Install

```sh
git clone PORTFOLIO-URL
cd PORTFOLIO/vsdd-suite-reference-examples/bookmark-cli-manual
cargo install --locked --path . --force
which bm   # expect: ~/.cargo/bin/bm
```

## Run

```sh
# Set the storage path (defaults to ./bookmarks.json if unset)
export BOOKMARK_CLI_DB=~/.bookmarks.json

# Add a URL (Layer 1)
bm add https://example.com

# List all bookmarks, newest first (Layer 1)
bm list

# Tag a bookmark by URL (Layer 2; idempotent)
bm tag https://example.com rust

# Filter the list by tag (Layer 2)
bm list --tag rust

# Repeated --tag composes as OR (a bookmark matches if it has ANY listed tag)
bm list --tag rust --tag go
```

## Test

```sh
cd PORTFOLIO/vsdd-suite-reference-examples/bookmark-cli-manual
cargo test
# expect: all tests pass — the default test suite (currently 13 unit + 45 integration + 3 proptest = 61 tests at Layer 3 Phase 2b landing) covers the behavioral contracts in DESIGN.md. Layer 3 cycle's Phase 2a regression tests + Phase 2b impl fixes for the 4 substantive Round 1 findings (JSON-native escape design + sorted-tag-comparison dedup + control-char tag rejection + QE coverage gaps) extend the test count further at their respective commits.
# Three additional #[ignore]-gated data-scaling sentinels at the
# 100 / 1,000 / 10,000-bookmark cliffs live at `tests/scaling.rs`. Run them
# explicitly via `cargo test --release -- --ignored` (the CI workflow does
# this in a separate Linux-only job to keep macOS-runner cost down).
```

## How this was built

Built using the [VSDD (Verified Spec-Driven Development) Suite](../../vsdd-suite/README.md) — the per-phase primers and per-domain review prompts. The spec is in [`DESIGN.md`](DESIGN.md); the layer plan and manual testing checklist are in [`TODO.md`](TODO.md); the per-session review-log files are in [`vsdd-suite/review-log/`](vsdd-suite/review-log/) and the project finding registry is at [`vsdd-suite/FINDINGS-INDEX.md`](vsdd-suite/FINDINGS-INDEX.md) (scaffolded via the suite's `templates/scaffold-project.sh`). IAR (Iterative Adversarial Refinement) runs at Phase 3 per the active domain set declared in [`DESIGN.md`](DESIGN.md) § Project intent; MVR (maximum viable refinement) is the per-domain stop trigger; TDD (test-driven development) discipline applies at Phase 2a (Red Gate) → Phase 2b (implementation).

Phase progression for Layer 1 (project-terminal at PR #42):

| Phase | Artifact | Status |
|---|---|---|
| 1a | [`DESIGN.md`](DESIGN.md) | Complete |
| 1b | [`TODO.md`](TODO.md) | Complete |
| 2a | [`tests/bookmarks.rs`](tests/bookmarks.rs) Red Gate | Complete (4 failing tests committed before implementation) |
| 2b | [`src/lib.rs`](src/lib.rs), [`src/main.rs`](src/main.rs) | Complete (all tests pass) |
| 3 | per-session [`vsdd-suite/review-log/`](vsdd-suite/review-log/) files + [`vsdd-suite/FINDINGS-INDEX.md`](vsdd-suite/FINDINGS-INDEX.md) registry | Complete (10 of 10 active capstone-tier role-domains at MVR + the VDD-IAR Alignment meta + Phase 5 SA Purity Boundary Audit + Phase 5 QE Mutation Testing rounds = 13 active per [`DESIGN.md` § Project intent](DESIGN.md#project-intent); Round 4 UX/TW/QE cluster closed in PR #42 post-Nathan-Bluesky-thread feedback) |
| 4 | [Phase 4](../../vsdd-suite/primers/4-feedback-integration.md) routing | Routed 80 findings through Phase 4 → fix cycle → Round 2 verification ([Review 82](../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z)) |
| 5 | Phase 5 hardening | Purity Boundary Audit ([SA Review 1](vsdd-suite/review-log/2026-05-20-solution-architect.md)) + Mutation Testing ([QE Review 2](vsdd-suite/review-log/2026-05-20-quality-engineer.md), 8/8 viable kill rate). Property-based testing deferred at Layer 1 (Layer-1 purity boundary shallow); Fuzz Testing + Proof Execution not applicable. |
| 6 | Phase 6 four-dimensional convergence attestation | [VDD-IAR Alignment Review 3 (project-terminal Layer 1)](vsdd-suite/review-log/2026-05-20-vdd-iar-alignment.md) — Spec MVR + Test MVR + Implementation MVR + Formal-verification MVR all ATTESTED at PR #42 |

Phase progression for Layer 2 (active per the post-PR-#43 cycle — adds `bm tag` + `bm list --tag` + the per-bookmark `tags` field):

| Phase | Artifact | Status |
|---|---|---|
| 1a | [`DESIGN.md`](DESIGN.md) Layer 2 extensions | Complete (AC 5–13 + § `bm tag` + § `bm list --tag` + § Storage format `tags` field + § Performance budget Layer 2 extensions) |
| 1b | [`TODO.md` § Layer 2 — Tag and filter](TODO.md#layer-2--tag-and-filter) | Complete |
| 2a | [`tests/bookmarks.rs`](tests/bookmarks.rs) Layer 2 Red Gate | Complete (13 + 1 = 14 new failing tests committed before implementation per the per-AC + RFC 3339 Layer-1-Deferred-closure plan; the Phase 2b sub-agent's pre-implementation spawn output recorded 12 of 13 failing with `error: unrecognized subcommand 'tag'`) |
| 2b | [`src/lib.rs`](src/lib.rs) + [`src/main.rs`](src/main.rs) Layer 2 surface | Complete (`Bookmark.tags` field + `AttachTagError` + `attach_tag` + `filter_by_tags` + parent-dir `fsync` on Unix + `Cmd::Tag` + `Cmd::List { tags }` clap surface) |
| 2c | [refactor (extract-and-name)](TODO.md#layer-2--tag-and-filter) | Complete — `run_add` / `run_list` / `run_tag` extracted from `main()` per [primer 2c § Scope catalog](../../vsdd-suite/primers/2c-refactor.md) |
| 3 | per-session [`vsdd-suite/review-log/`](vsdd-suite/review-log/) Layer 2 rounds | Round 1 4-cluster parallel cold-session complete; Round 1 inline fix cycle in progress; Round 2 cold-session verification pending |
| 5 | Phase 5 Layer 2 re-runs | Pending — Purity Boundary Audit against the extended pure surface (`filter_by_tags` + `attach_tag`); Mutation Testing against the extended impl; property-based testing via [`proptest`](https://github.com/proptest-rs/proptest) activated at [`tests/properties.rs`](tests/properties.rs) (tag-idempotence + filter-OR-monotonicity properties) |
| 6 | Phase 6 four-dimensional convergence | **NOT APPLICABLE** at Layer 2 per [`DESIGN.md` § Project intent](DESIGN.md#project-intent) Phase 6 strategy for Layer 2 — Layer 1's project-terminal Phase 6 attestation at VDD-IAR Alignment Review 3 satisfies the reference-implementation purpose ([G-112](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112)) + capstone gates at project-terminal MVR per primer 6, not per-layer ([G-150](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-150) over-investment guard) |

For the Layer 2 narrative in depth, see:

- [`TODO.md` § Layer 2 — Tag and filter](TODO.md#layer-2--tag-and-filter) — acceptance criteria + Red Gate test plan + Layer-gate criteria.
- [`manual-tests/layer-2.md`](manual-tests/layer-2.md) — per-layer manual-test plan (13 steps including the `hyperfine` performance sanity-check).
- [`DESIGN.md` § Behavioral contracts](DESIGN.md#behavioral-contracts) — the `bm tag` / `bm list --tag` contracts + the `tags` field forward-only-migration discipline.

## License

MIT OR Apache-2.0 (matches the portfolio's [`issue-tracker-cli`](../../issue-tracker-cli/Cargo.toml) precedent; aligned with [`Cargo.toml`](Cargo.toml) license field in the [Review 82](../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z) Round 2 fix cycle).
