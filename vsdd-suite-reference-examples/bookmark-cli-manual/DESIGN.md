# DESIGN.md — bookmark-cli

Phase 1a+1b contract (per v0.7.2 conventions; the file was originally authored under the prior single-step "Phase 1a" naming — historical narrative preserved per G-89 forward-only policy; the renamed primer at [`../vsdd-suite/primers/1ab-spec-crystallization.md`](../vsdd-suite/primers/1ab-spec-crystallization.md) is the current authoring reference). This file is the reference-implementation contract for the worked example documented at [`../vsdd-suite/README.md`](../vsdd-suite/README.md) § Worked example — it exists to validate the suite end-to-end per G-112 in the suite's gap registry.

---

## Project intent

(Added Review 67 per the v0.7.2 adoption; the declarations below conform to the suite's G-150 intent calibration and G-162 strategy declaration requirements as of suite v0.6.0+. The project's first-layer-gate-close predates 2026-05-20, so the v0.7.2 conventions apply forward-only — this declaration is the explicit adoption marker.)

**Declared intent for this project:** `portfolio`. Rationale: `bookmark-cli` is the suite's reference implementation, intended for external reading and handoff as the worked-example artifact (not capstone-graduation work, not production deployment). The 7-core default activation per G-121 scaffold-default applies; Technical Writer activation has not been formally added (the project predates the G-101–G-105 closure cycle's TW push) but the project README is maintained at a reader-can-follow standard.

**Phase 5 strategy:** `planned — Surface A.0 (purity-boundary verification) + Surface B (mutation testing via cargo-mutants) per the v0.7.2 primer. Surface A (property-based testing via proptest) deferred to a future layer if the purity boundary deepens; Surface C (fuzzing) and Surface D (formal proof) are not applicable — bookmark-cli has no safety-critical or cryptographic surface and the JSON-parsing surface is small enough that fuzzing's marginal value is low.` Per-layer Phase 5 rounds file under the per-domain review logs per G-177 closure (v0.7.8): Surface A.0 in [`vsdd-suite/SOLUTION-ARCHITECT-REVIEW.md`](vsdd-suite/SOLUTION-ARCHITECT-REVIEW.md); Surface B in [`vsdd-suite/QUALITY-ENGINEER-REVIEW.md`](vsdd-suite/QUALITY-ENGINEER-REVIEW.md).

**Phase 6 strategy:** `not applicable — bookmark-cli is portfolio-intent, not capstone-intent. The project closes at end of Phase 4 by design per the suite's intent-calibration discipline; four-dimensional convergence is not a methodology promise the project makes.` Per G-162: portfolio-intent declarations of `not applicable` are valid; the Phase 6 primer is not opened.

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

**Purity boundary (revised Review 67 / B2 reconciliation against actual `src/lib.rs` implementation; supersedes the prior implicit "pure-core" framing in the module doc).** This is the authoritative purity boundary for the project. The module doc at `src/lib.rs:1-?` cites this section as the single source.

- **Pure functions** (deterministic, no I/O, formally verifiable in principle):
  - `Bookmark` and `BookmarkStore` data types (serde derivations are pure functions of input).
  - `BookmarkStore::newest_first` (pure sort by reference; no I/O, no clock).
- **Effectful (deliberate I/O wrappers around pure ser/de):**
  - `BookmarkStore::load(path)` — filesystem read + `serde_json` parse. The parse step is pure; the file read makes the function effectful.
  - `BookmarkStore::save(path)` — `serde_json` serialize + filesystem write + directory creation. Same shape: serialize pure, write effectful.
- **Boundary refinement (morally pure w.r.t. its inputs; effectful w.r.t. external clock):**
  - `BookmarkStore::add(url)` — appends a new `Bookmark` whose timestamp is `Utc::now()` at call time. Deterministic given the clock; non-deterministic against absolute wall time. Acceptable at Layer 1 portfolio intent; could be refined to `add(url, ts)` at a future layer if formal verification of `add` enters scope.

**Verification surfaces:**

- **Unit tests** for the pure functions and the I/O-wrapper functions in `src/lib.rs`'s `#[cfg(test)] mod tests` block; the I/O-wrapper tests use `tempfile` for filesystem isolation.
- **Integration tests** in `tests/bookmarks.rs` that invoke the compiled binary via `assert_cmd` against per-test temp directories — full stdout/stderr/exit-code contract per CLI supplement § Quality Engineering.
- **No mocks for the storage layer** — tests use real temp files via `tempfile`.
- **Manual testing checklist** in [`TODO.md`](TODO.md) § Layer 1, expanded per the runnable-step standard.
- **IAR Phase 3** runs the 7 default-active core domains per Review 42 doctrine (SE, QE, UX, Security, SA, SO, VDD-IAR Alignment). Each domain's index lives at `vsdd-suite/<DOMAIN>-REVIEW.md`; rounds file as session entries in `vsdd-suite/review-log/YYYY-MM-DD-<slug>.md` per the G-89 structural standard.
- **Phase 5 hardening** (added Review 67 — Phase 5 adoption per v0.7.2 conventions; migrated to per-domain log shape per G-177 / v0.7.8): per-layer Phase 5 rounds file in `vsdd-suite/SOLUTION-ARCHITECT-REVIEW.md` (Surface A.0 / A / D) and `vsdd-suite/QUALITY-ENGINEER-REVIEW.md` (Surface B / C) with the `**Phase 5 surface:**` preamble tag per round. The Phase 5 strategy is declared in § Project intent below.

**Formal-proof candidates (Phase 5 Surface D):** none. `bookmark-cli` is not safety-critical or cryptographic; no function on the purity boundary above warrants formal proof. Surface D declared `not applicable` in the § Project intent Phase 5 strategy line.

**Automatable-vs-manual split:** every behavioral contract above is automatable via unit + integration tests. Manual testing (per TODO.md § Layer 1) verifies UX-coherence concerns (error message specificity; the empty-state stderr line as the user would read it) that automated tests can also assert syntactically but cannot evaluate as "reads naturally."

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
