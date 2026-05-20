# DESIGN.md — bookmark-cli

Phase 1a+1b contract (per v0.7.2 conventions; the file was originally authored under the prior single-step "Phase 1a" naming — historical narrative preserved per G-89 forward-only policy; the renamed primer at [`../vsdd-suite/primers/1ab-spec-crystallization.md`](../vsdd-suite/primers/1ab-spec-crystallization.md) is the current authoring reference). This file is the reference-implementation contract for the worked example documented at [`../vsdd-suite/README.md`](../vsdd-suite/README.md) § Worked example — it exists to validate the suite end-to-end per G-112 in the suite's gap registry.

---

## Project intent

(Initially declared `portfolio` in Review 67 per v0.7.2 adoption. **Promoted to `capstone` in PR 6 / Review 78** — bookmark-cli is the reference implementation for the VSDD Suite's worked example; reference implementations must exercise the full 6-phase methodology to teach what they document. Per the G-177 precedent — reference examples migrate when the methodology evolves — the prior portfolio-intent declaration is preserved as the historical-narrative anchor below the current declaration.)

**Declared intent for this project (current):** `capstone`. Rationale: bookmark-cli is the suite's reference implementation for the worked example documented at `../../vsdd-suite/README.md` § Worked example. The walkthrough exercises **all six VSDD phases** (1a+1b spec / 1c decomposition / 2a Red Gate / 2b implementation / 2c refactor / 3 IAR / 4 routing / 5 hardening / 6 convergence). For the reference to teach what it documents, it must itself run at the bar that walks the full methodology — capstone intent is the natural fit. The 7 core role+meta activate plus the capstone-tier extended domains. Active domain set: 6 core role (SE, QE, UX, Security, SA, SO) + VDD-IAR Alignment meta + 4 extended (Performance Engineer — capstone activation per G-150 intent calibration; Platform Engineer — G-178 strong-presumption + G-155 dim 38 fresh-system install verification at capstone; Red Team — capstone-tier adversarial intensity per the extended-pool activation criteria; Technical Writer — portfolio+ activation for the worked example's clone-and-follow audit trail) = **10 role + 1 meta = 11 active domains**. Data Engineer evaluated and ruled out — bookmark-cli's flat JSON storage falls below the G-178 activation threshold; the absence is documented as deliberate. Sanity Check meta domain (Review 77 Finding 2) activates on-demand for findings without natural cross-domain pair; not part of the 11-domain scheduled set.

**Declared intent (historical):** `portfolio` (Review 67 → PR 6 / Review 78). Preserved per G-89 forward-only narrative-preservation. The existing 3 portfolio-intent reviews (QE Review 1 dated 2026-05-17; QE Review 2 + SA Review 1 dated 2026-05-20) remain valid records of how the project operated under the prior intent; PR 6's migration adds Review 77 lifecycle fields to those entries without invalidating their portfolio-era findings.

**Phase 5 strategy:** `planned — Surface A.0 (purity-boundary verification) executed (SA Review 1, 2026-05-20) + Surface B (mutation testing via cargo-mutants) executed (QE Review 2, 2026-05-20, 100% kill rate on 8 viable mutants). Surface A (property-based testing via proptest) deferred — the purity boundary at Layer 1 is shallow (one pure function); property-based testing's marginal value is low. Surface C (fuzzing) and Surface D (formal proof) not applicable — bookmark-cli has no safety-critical, cryptographic, or input-boundary attack surface that warrants the tooling.` Per-layer Phase 5 rounds file under the per-domain review logs per G-177 closure: Surface A.0 in [`vsdd-suite/SOLUTION-ARCHITECT-REVIEW.md`](vsdd-suite/SOLUTION-ARCHITECT-REVIEW.md) Review 1; Surface B in [`vsdd-suite/QUALITY-ENGINEER-REVIEW.md`](vsdd-suite/QUALITY-ENGINEER-REVIEW.md) Review 2.

**Phase 6 strategy:** `planned — four-dimensional convergence record landed as the final VDD-IAR Alignment review round titled "Review N — Phase 6 four-dimensional convergence (project-terminal)" per primer 6 + G-177. Attests: Spec MVR (DESIGN.md round closure); Test MVR (QE Reviews 1+2 closure including the Phase 5 Surface B 100% mutation-kill); Implementation MVR (every active-domain Phase 3 round at MVR per the post-PR-6 capstone IAR coverage); Formal-verification MVR (Surface A.0 purity-boundary verification + Surface B mutation testing closure; Surface A/C/D declared not-applicable with rationale). Cross-dimension consistency check applied at convergence time; signed closing attestation.` Per G-162: capstone-intent declarations require both Phase 5 + Phase 6 strategy lines; both declared above.

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
