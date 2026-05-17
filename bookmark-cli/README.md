# bookmark-cli

A single-user command-line tool for capturing URLs at the terminal and recalling them later. `bm add https://example.com` saves a URL with a timestamp; `bm list` prints all saved URLs newest-first.

## What this is

`bookmark-cli` is the **reference implementation** for the [VSDD Suite](../vsdd-suite/README.md)'s worked example — it exists to validate the suite's documented workflow end-to-end (G-112 in the suite's gap registry). It is small by design and intentionally limited in scope. A user who wants a real bookmark manager should use a browser or a dedicated tool; this is a portfolio demonstration artifact.

Current state: **Layer 1 complete** (add + list). Layers 2 (tag + filter) and 3 (export + import) are scoped in [`DESIGN.md`](DESIGN.md) but not built — the reference-implementation purpose is satisfied by one layer end-to-end.

## Prerequisites

- Rust 1.78+ (`cargo --version` to check)
- macOS or Linux (Windows untested)

## Install

```sh
git clone <portfolio-url>
cd <portfolio>/bookmark-cli
cargo install --path . --force
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
cd <portfolio>/bookmark-cli
cargo test
# expect: 8 tests pass (4 lib unit tests + 4 integration tests)
```

## How this was built

Built using the [VSDD Suite](../vsdd-suite/README.md) — the per-phase primers and per-domain review prompts. The spec is in [`DESIGN.md`](DESIGN.md); the layer plan and manual testing checklist are in [`TODO.md`](TODO.md); the per-domain review-log indices are in [`vsdd-suite/`](vsdd-suite/) (scaffolded via the suite's `templates/scaffold-project.sh`).

Phase progression for Layer 1:

| Phase | Artifact | Status |
|---|---|---|
| 1a | [`DESIGN.md`](DESIGN.md) | ✓ Complete |
| 1b | [`TODO.md`](TODO.md) | ✓ Complete |
| 2a | [`tests/bookmarks.rs`](tests/bookmarks.rs) Red Gate | ✓ Complete (4 failing tests committed before implementation) |
| 2b | [`src/lib.rs`](src/lib.rs), [`src/main.rs`](src/main.rs) | ✓ Complete (8/8 tests pass) |
| 3 | [`vsdd-suite/<DOMAIN>-REVIEW.md`](vsdd-suite/) per-domain indices | Scaffolded; rounds-in-progress (this is reference-implementation work, not a real merge gate) |
| 4 | Phase 4 routing | N/A — no live findings to route |

## License

MIT
