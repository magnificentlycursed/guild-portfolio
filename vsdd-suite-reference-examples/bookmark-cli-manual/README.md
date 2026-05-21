# bookmark-cli

A single-user command-line tool for capturing URLs at the terminal and recalling them later. `bm add https://example.com` saves a URL with a timestamp; `bm list` prints all saved URLs newest-first.

## What this is

`bookmark-cli` is the **reference implementation** for the [VSDD (Verified Spec-Driven Development) Suite](../../vsdd-suite/README.md)'s worked example — it exists to validate the suite's documented workflow end-to-end ([G-112](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112) in the suite's gap registry). It is small by design and intentionally limited in scope. A user who wants a real bookmark manager should use a browser or a dedicated tool; this is a portfolio demonstration artifact.

Current state: **Layer 1 complete** (add + list). Layers 2 (tag + filter) and 3 (export + import) are scoped in [`DESIGN.md`](DESIGN.md) but not built — the reference-implementation purpose is satisfied by one layer end-to-end.

## Prerequisites

- [Rust](https://www.rust-lang.org/) 1.78+ (`cargo --version` to check)
- macOS or Linux (Windows untested)

## Install

```sh
git clone <portfolio-url>
cd <portfolio>/vsdd-suite-reference-examples/bookmark-cli-manual
cargo install --locked --path . --force
which bm   # expect: ~/.cargo/bin/bm
```

## Run

```sh
# Set the storage path (defaults to ./bookmarks.json if unset)
export BOOKMARK_CLI_DB=~/.bookmarks.json

# Add a URL
bm add https://example.com

# List all bookmarks, newest first
bm list
```

## Test

```sh
cd <portfolio>/vsdd-suite-reference-examples/bookmark-cli-manual
cargo test
# expect: all tests pass — the test suite (currently ~19 lib + integration tests at Layer 1, post-Round-2 fix cycle) covers the behavioral contracts in DESIGN.md.
```

## How this was built

Built using the [VSDD (Verified Spec-Driven Development) Suite](../../vsdd-suite/README.md) — the per-phase primers and per-domain review prompts. The spec is in [`DESIGN.md`](DESIGN.md); the layer plan and manual testing checklist are in [`TODO.md`](TODO.md); the per-domain review-log indices are in [`vsdd-suite/`](vsdd-suite/) (scaffolded via the suite's `templates/scaffold-project.sh`). IAR (Iterative Adversarial Refinement) runs at Phase 3 per the active domain set declared in [`DESIGN.md`](DESIGN.md) § Project intent; MVR (maximum viable refinement) is the per-domain stop trigger; TDD (test-driven development) discipline applies at Phase 2a (Red Gate) → Phase 2b (implementation).

Phase progression for Layer 1:

| Phase | Artifact | Status |
|---|---|---|
| 1a | [`DESIGN.md`](DESIGN.md) | Complete |
| 1b | [`TODO.md`](TODO.md) | Complete |
| 2a | [`tests/bookmarks.rs`](tests/bookmarks.rs) Red Gate | Complete (4 failing tests committed before implementation) |
| 2b | [`src/lib.rs`](src/lib.rs), [`src/main.rs`](src/main.rs) | Complete (all tests pass) |
| 3 | [`vsdd-suite/<DOMAIN>-REVIEW.md`](vsdd-suite/) per-domain indices | Scaffolded; rounds-in-progress (this is reference-implementation work, not a real merge gate) |
| 4 | [Phase 4](../../vsdd-suite/primers/4-feedback-integration.md) routing | Routed 80 findings through Phase 4 → fix cycle → Round 2 verification ([Review 82](../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z)) |

## License

MIT OR Apache-2.0 (matches the portfolio's [`issue-tracker-cli`](../../issue-tracker-cli/Cargo.toml) precedent; aligned with [`Cargo.toml`](Cargo.toml) license field in the [Review 82](../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z) Round 2 fix cycle).
