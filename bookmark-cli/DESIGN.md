# DESIGN.md — bookmark-cli

Phase 1a contract. Authored with [`../vsdd-suite/primers/1a-spec-crystallization.md`](../vsdd-suite/primers/1a-spec-crystallization.md) loaded as the session primer. This file is the reference-implementation contract for the worked example documented at [`../vsdd-suite/README.md`](../vsdd-suite/README.md) § Worked example — it exists to validate the suite end-to-end per G-112 in the suite's gap registry.

---

## What this project does

`bookmark-cli` is a single-user command-line tool for capturing URLs the user encounters at the terminal and recalling them later. The user runs `bm add <url>` to save a URL with a timestamp; the user runs `bm list` to print all saved URLs newest-first. Storage is a flat JSON file in the current working directory (or at the path named by `$BOOKMARK_CLI_DB`).

The project exists as the reference implementation for the VSDD suite's worked example. It is small by design — its purpose is to exercise the suite end-to-end, not to be a useful bookmark manager. A user who wants a real bookmark tool should use the browser, not this.

## Scope and non-goals

**In scope (Layer 1):**
- `bm add <url>` — capture a URL with the current timestamp
- `bm add` (no URL) — reject with a specific error message
- `bm list` — print all bookmarks newest-first
- `bm list` (no bookmarks) — print an explicit empty-state message
- Storage in a flat JSON file at `$BOOKMARK_CLI_DB` or `./bookmarks.json`

**In scope (Layer 2, deferred):**
- `bm tag <id> <label>` — attach a label to a bookmark
- `bm list --tag <label>` — filter by label

**In scope (Layer 3, deferred):**
- `bm export` — emit bookmarks as JSON to stdout
- `bm import` — read bookmarks from stdin

**Non-goals (out of scope at every layer):**
- Network synchronization — local file only
- User accounts or multi-user — single-user local tool
- Browser integration — terminal only
- Search beyond tag filtering — `grep` is the search tool
- URL validation beyond non-empty — accept any string; the user is responsible
- Editing or deleting bookmarks — append-only semantics; manual JSON edit if needed
- Configuration file — environment variable + sensible default is the entire config surface

## Behavioral contracts

### `bm add <url>`

- **Input shape:** exactly one positional argument, a non-empty string.
- **Success output (stdout):** silent. Exit 0.
- **Success side effect:** appends a `Bookmark { url, timestamp }` record to the storage file. Creates the file if absent. Timestamp is the current UTC time in RFC 3339 format.
- **Failure (empty URL):** stderr `Error: URL cannot be empty.` followed by newline. Exit 1. No file write.
- **Failure (storage file unreadable / unwritable):** stderr `Error: <descriptive message>` followed by newline. Exit 2. No partial write.

### `bm list`

- **Input shape:** no positional arguments, no flags.
- **Success output (stdout):** zero or more lines, one per bookmark, newest-first. Format per line: `<timestamp> <url>` (timestamp in RFC 3339; single space separator). Trailing newline after the last bookmark.
- **Success exit:** 0.
- **Empty-state output:** stdout silent. Stderr: `No bookmarks yet.` followed by newline. Exit 0 (empty is success, not failure).
- **Failure (storage file unreadable / corrupt JSON):** stderr `Error: <descriptive message>` followed by newline. Exit 2. Stdout silent.

## Edge case catalog

- **Empty URL argument:** `bm add ""` → rejected per failure contract above.
- **Whitespace-only URL:** `bm add "   "` → currently accepted; the user is responsible. This is a deliberate non-goal of input validation.
- **Storage file absent:** `bm list` on a fresh project → empty-state message; `bm add` → creates the file.
- **Storage file empty (zero bytes):** treat as empty bookmark list, not as corrupt.
- **Storage file contains invalid JSON:** error to stderr, exit 2. Do not attempt recovery.
- **Concurrent writes:** out of scope; not a multi-process tool. Single user, single shell session.
- **Very long URL (10K+ chars):** accepted. No length cap.
- **URL containing newlines:** accepted. May visually break the `bm list` output, which is acceptable for this scope.

## Interface definitions

### Command surface (Layer 1)

```
bm add <url>
bm list
bm --help
bm --version
```

### Exit codes

| Code | Meaning |
|---|---|
| 0 | Success (including empty `bm list`) |
| 1 | User error (empty URL) |
| 2 | Storage error (file unreadable, corrupt JSON, write failure) |

### Storage format (JSON file)

```json
{
  "bookmarks": [
    {"url": "https://example.com", "timestamp": "2026-05-17T03:01:00Z"},
    {"url": "https://example.org", "timestamp": "2026-05-17T02:55:00Z"}
  ]
}
```

Newest-first ordering is a render concern (sort on read), not a storage concern (append on write).

## Verification architecture

- **Unit tests** for the pure-core storage logic in `src/lib.rs`: load/save/add operations.
- **Integration tests** in `tests/bookmarks.rs` that invoke the compiled binary via `assert_cmd` against per-test temp directories — full stdout/stderr/exit-code contract per CLI supplement § Quality Engineering.
- **No mocks for the storage layer** — tests use real temp files via `tempfile`.
- **Manual testing checklist** in [`TODO.md`](TODO.md) § Layer 1, expanded per the runnable-step standard.
- **IAR Phase 3** runs the 7 default-active core domains per Review 42 doctrine (SE, QE, UX, Security, SA, SO, VDD-IAR Alignment). Each domain's index lives at `vsdd-suite/<DOMAIN>-REVIEW.md`; rounds file as session entries in `vsdd-suite/review-log/YYYY-MM-DD-<slug>.md` per the G-89 structural standard.

## Technology choices and rationale

| Choice | Alternatives considered | Why this |
|---|---|---|
| Rust | TypeScript/Node, Python, Go | Matches the worked example's language; portfolio precedent (`issue-tracker-cli`); strong test/CLI ergonomics |
| Cargo workspace = single crate | Workspace with separate `lib` and `bin` crates | Over-engineering for one binary |
| `clap` (derive) | Hand-rolled arg parsing | Standard Rust CLI parser |
| `serde_json` | Custom JSON / TOML / sqlite | Spec calls for JSON |
| `chrono` (UTC) | `time` crate / system epoch ints | RFC 3339 formatting is well-supported |
| `anyhow` for error types | Custom error enums per `thiserror` | Single-binary tool; `thiserror` would be over-engineering |
| `assert_cmd` + `tempfile` for tests | Direct std::process invocation | CLI supplement § QE prescribes binary-invocation tests |

## Constraints

- **Rust toolchain:** 1.78+ (modern stable Rust; no unstable features).
- **Platform:** macOS, Linux. Windows untested.
- **Dependencies:** all from crates.io, no git deps. `Cargo.lock` committed.
- **Deployment:** `cargo install --path .` into `~/.cargo/bin/`. No release pipeline.

## Open questions

*(none at the close of Phase 1a — the self-adversary check completed cleanly because the project's scope is small and the contracts are observable from outside the implementation. Any ambiguities surfaced during Phase 2 or Phase 3 will be routed back to this section per Phase 4 routing discipline.)*
