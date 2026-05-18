# TODO.md — bookmark-cli

Phase 1b output. Authored with [`../vsdd-suite/primers/1b-decomposition.md`](../vsdd-suite/primers/1b-decomposition.md) loaded. Each layer below is independently testable and shippable; acceptance criteria are observable behaviors (not implementation steps); the Red Gate test plan names the literal tests that must fail before any implementation lands per layer.

This project is the reference implementation for the suite's worked example (G-112 in `vsdd-suite/suite-development/GAP-ANALYSIS-LOG.md`). Only Layer 1 is built out — Layers 2 and 3 are scoped but deferred to follow-on work.

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

**Manual testing checklist** (per `primers/1b-decomposition.md` § Manual testing checklist, runnable-step standard):

```sh
# Setup: fresh install of the binary
cd <portfolio>/bookmark-cli
cargo install --path . --force --quiet
# Verify binary is on PATH:
which bm   # expect: ~/.cargo/bin/bm

# Test 1 — happy path
export BOOKMARK_CLI_DB="$(mktemp -d)/bookmarks.json"
bm add https://example.com
# expect exit 0, stdout empty, stderr empty
cat "$BOOKMARK_CLI_DB"
# expect a JSON document with one bookmark object, url=https://example.com, timestamp=current UTC

# Test 2 — empty URL rejection
bm add ""
# expect exit 1, stderr "Error: URL cannot be empty.", stdout empty
echo "exit=$?"   # expect: exit=1

# Test 3 — list ordering
bm add https://first.example
sleep 1
bm add https://second.example
bm list
# expect: two lines, second.example first (newest), first.example second

# Test 4 — empty list
rm "$BOOKMARK_CLI_DB"
bm list
# expect exit 0, stdout empty, stderr "No bookmarks yet."

# Cleanup
rm -rf "$(dirname "$BOOKMARK_CLI_DB")"
unset BOOKMARK_CLI_DB
cargo uninstall bookmark-cli --quiet
```

**Layer-gate criteria:**

1. All four Red Gate tests pass: `cargo test --test bookmarks`.
2. `cargo build --release` succeeds with no warnings.
3. The manual testing checklist above runs clean (every step produces the expected exit/stdout/stderr).
4. Phase 3 IAR reviews complete for the 7 default-active core domains (per G-121 doctrine: SE, QE, UX, Security, SA, SO, VDD-IAR Alignment); each domain reaches MVR or no findings.

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
