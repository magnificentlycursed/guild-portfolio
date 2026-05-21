# DESIGN.md — bookmark-cli

[Phase 1a+1b](../../vsdd-suite/primers/1ab-spec-crystallization.md) contract (per v0.7.2 conventions; the file was originally authored under the prior single-step "Phase 1a" naming + the prior primer filename `1ab-spec-development.md` — both retired by the suite. The current canonical primer is [`../../vsdd-suite/primers/1ab-spec-crystallization.md`](../../vsdd-suite/primers/1ab-spec-crystallization.md); historical narrative preserved per [G-89](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-89) forward-only policy). This file is the reference-implementation contract for the worked example documented at [`../../vsdd-suite/README.md`](../../vsdd-suite/README.md) § Worked example — it exists to validate the suite end-to-end per [G-112](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112) in the suite's gap registry.

---

## Project intent

(Initially declared `portfolio` in Review 67 per v0.7.2 adoption. **Promoted to `capstone` in PR 6 / Review 78** — bookmark-cli is the reference implementation for the VSDD Suite's worked example; reference implementations must exercise the full 6-phase methodology to teach what they document. Per the [G-177](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-177) precedent — reference examples migrate when the methodology evolves — the prior portfolio-intent declaration is preserved as the historical-narrative anchor below the current declaration.)

**Declared intent for this project (current):** `capstone`. Rationale: bookmark-cli is the suite's reference implementation for the worked example documented at [`../../vsdd-suite/README.md`](../../vsdd-suite/README.md) § Worked example. The walkthrough exercises **all six VSDD phases** (1a+1b spec / 1c decomposition / 2a Red Gate / 2b implementation / 2c refactor / 3 IAR / 4 routing / 5 hardening / 6 convergence). For the reference to teach what it documents, it must itself run at the bar that walks the full methodology — capstone intent is the natural fit. The 7 core role+meta activate plus the capstone-tier extended domains. Active domain set: 6 core role (SE, QE, [UX](../../vsdd-suite/domains/role/UX-REVIEW.md), [Security](../../vsdd-suite/domains/role/SECURITY-REVIEW.md), SA, SO) + [VDD-IAR Alignment](../../vsdd-suite/domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md) meta + 6 extended ([Performance Engineer](../../vsdd-suite/domains/role/PERFORMANCE-ENGINEER-REVIEW.md) — capstone activation per [G-150](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-150) intent calibration; [Platform Engineer](../../vsdd-suite/domains/role/PLATFORM-ENGINEER-REVIEW.md) — [G-178](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-178) strong-presumption + [G-155](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-155) dim 38 fresh-system install verification at capstone; [Red Team](../../vsdd-suite/domains/role/RED-TEAM-REVIEW.md) — capstone-tier adversarial intensity per the extended-pool activation criteria; [Technical Writer](../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) — portfolio+ activation for the worked example's clone-and-follow audit trail; [Documentation Reviewer](../../vsdd-suite/domains/role/DOCUMENTATION-REVIEWER-REVIEW.md) — TW adversarial cold-reader pair, registered in [Review 80](../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z), activates together with TW at capstone intent; [AI Engineer](../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md) — cost-and-quality discipline for parallel cold-session AI-agent usage, registered in [Review 83](../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-83--2026-05-21-1000z) after PR [#38](https://github.com/magnificentlycursed/guild-portfolio/pull/38)'s 3-round cycle burned ~3-4M tokens + hit a daily rate-limit mid-cycle, activates by default at capstone intent given sustained multi-round IAR cycles) = **12 role + 1 meta = 13 active domains**. [Data Engineer](../../vsdd-suite/domains/role/DATA-ENGINEER-REVIEW.md) evaluated and ruled out — bookmark-cli's flat JSON storage falls below the [G-178](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-178) activation threshold; the absence is documented as deliberate. [Sanity Check](../../vsdd-suite/domains/meta/SANITY-CHECK-REVIEW.md) meta domain ([Review 77](../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-77--2026-05-20-1545z) Finding 2) activates on-demand for findings without natural cross-domain pair; not part of the 13-domain scheduled set.

**Declared intent (historical):** `portfolio` (Review 67 → PR 6 / Review 78). Preserved per [G-89](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-89) forward-only narrative-preservation. The existing 3 portfolio-intent reviews (QE Review 1 dated 2026-05-17; QE Review 2 + SA Review 1 dated 2026-05-20) remain valid records of how the project operated under the prior intent; PR 6's migration adds Review 77 lifecycle fields to those entries without invalidating their portfolio-era findings.

**[Phase 5](../../vsdd-suite/primers/5-formal-hardening.md) strategy:** `planned — Purity Boundary Audit executed (SA Review 1, 2026-05-20) + Mutation Testing via cargo-mutants executed (QE Review 2, 2026-05-20, 100% kill rate on 8 viable mutants). property-based testing via proptest deferred — the purity boundary at Layer 1 is shallow (one pure function); property-based testing's marginal value is low. Fuzz Testing and Proof Execution not applicable — bookmark-cli has no safety-critical, cryptographic, or input-boundary attack surface that warrants the tooling.` Per-layer Phase 5 rounds file under the per-domain review logs per [G-177](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-177) closure: Purity Boundary Audit in [`vsdd-suite/review-log/2026-05-20-solution-architect.md`](vsdd-suite/review-log/2026-05-20-solution-architect.md) Review 1; Mutation Testing in [`vsdd-suite/review-log/2026-05-20-quality-engineer.md`](vsdd-suite/review-log/2026-05-20-quality-engineer.md) Review 2.

**[Phase 6](../../vsdd-suite/primers/6-convergence.md) strategy:** `planned — four-dimensional convergence record landed as the final VDD-IAR Alignment review round titled "Review N — Phase 6 four-dimensional convergence (project-terminal)" per primer 6 + G-177. Attests: Spec MVR (DESIGN.md round closure); Test MVR (QE Reviews 1+2 closure including the Phase 5 Mutation Testing 100% mutation-kill); Implementation MVR (every active-domain Phase 3 round at MVR per the post-PR-6 capstone IAR coverage); Formal-verification MVR (Purity Boundary Audit + Mutation Testing closure; property-based testing / Fuzz Testing / Proof Execution declared deferred or not-applicable with rationale). Cross-dimension consistency check applied at convergence time; signed closing attestation.` Per [G-162](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-162): capstone-intent declarations require both Phase 5 + Phase 6 strategy lines; both declared above.

**Cold-session budget:** capstone default per [`../../vsdd-suite/domains/DOMAIN-INDEX.md`](../../vsdd-suite/domains/DOMAIN-INDEX.md) § Cold-session budget per intent — max 4 rounds before stop-trigger consultation; max 10 parallel agents per round (or 4-cluster batched with adversarial-pair separation per the PR [#38](https://github.com/magnificentlycursed/guild-portfolio/pull/38) Round 3 precedent); 100k–300k tokens per substantive finding expected band; [Opus 4.7](../../vsdd-suite/README.md) for Software Engineer / Security / Red Team / Solution Architect / Solution Owner / VDD-IAR Alignment / AI Engineer; [Sonnet 4.6](../../vsdd-suite/README.md) for UX / Performance Engineer / Platform Engineer / Technical Writer / Documentation Reviewer / Quality Engineer; [Haiku 4.5](../../vsdd-suite/README.md) for mechanical-sweep delegated sub-agents (anchor-link sweeps, reference rewrites, per-domain-index retirement cascades). Actual cost evidence: PR #38 Round 3 cycle ~$5/cluster at the 4-cluster shape; AI Engineer Review 1 cycle (PR [#39](https://github.com/magnificentlycursed/guild-portfolio/pull/39)) registered ~21k tokens/finding — well below the band's floor, read as cold-session discipline working efficiently per [AI Engineer R1 F6+F7+F8](vsdd-suite/review-log/2026-05-21-ai-engineer.md). Pre-cycle declaration discipline applied at every future multi-agent cycle per [`../../vsdd-suite/primers/3-review-session.md`](../../vsdd-suite/primers/3-review-session.md) § Pre-cycle methodology check; after-action cost-tally per [`../../vsdd-suite/suite-development/suite-development.md`](../../vsdd-suite/suite-development/suite-development.md) § Per-review entry preamble § Cost-tally. Per [Review 84](../../vsdd-suite/suite-development/review-log/2026-05-21-suite-review.md#review-84--2026-05-21-1100z) (PR [#40](https://github.com/magnificentlycursed/guild-portfolio/pull/40)): cold-session-budget declarations are required at capstone + production intent.

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
- **Failure (empty URL — both "empty string" and "no positional argument given"):** stderr `Error: URL cannot be empty.` followed by newline. Exit 1. No file write. Per [SE Review 1 Finding 1](vsdd-suite/review-log/2026-05-20-software-engineer.md#review-1--2026-05-20-1930z) (Review 82 fix-cycle): `bm add` (no positional) is treated identically to `bm add ""` — the parser intercepts clap's usage-error path and emits the spec-contracted exit code 1.
- **Failure (storage file unreadable / unwritable):** stderr `Error: <descriptive message>` followed by newline. Exit 2. **Atomic write** — partial writes MUST NOT occur. The implementation uses a temporary file in the destination directory + atomic rename per POSIX `rename(2)` semantics. If write or rename fails, the storage file's prior state is preserved. Per [SE Review 1 Finding 2](vsdd-suite/review-log/2026-05-20-software-engineer.md#review-1--2026-05-20-1930z) (Review 82 fix-cycle).
- **Failure (CLI usage error other than missing/empty URL — e.g., unknown subcommand, unknown flag):** stderr clap-formatted usage message. Exit 64 (per `sysexits.h` `EX_USAGE`). Per [SE Review 1 Finding 3](vsdd-suite/review-log/2026-05-20-software-engineer.md#review-1--2026-05-20-1930z) (Review 82 fix-cycle) — disambiguates from exit 2 storage errors.

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

| Code | Meaning | Source |
|---|---|---|
| 0 | Success (including empty `bm list`) | Application |
| 1 | User error (empty URL — both `bm add ""` and `bm add` with no positional argument) | Application |
| 2 | Storage error (file unreadable, corrupt JSON, write failure, parent-dir creation failure) | Application |
| 64 | CLI usage error (`EX_USAGE` per `sysexits.h` — unknown subcommand, unknown flag, malformed invocation other than missing/empty URL) | Application (intercepts clap's default exit) |

[SE Review 1 Finding 3](vsdd-suite/review-log/2026-05-20-software-engineer.md#review-1--2026-05-20-1930z) Round 2 disposition: spec extended to disambiguate "user error in URL" (exit 1) from "user error in CLI invocation shape" (exit 64). Storage error (exit 2) is unambiguous.

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
- **IAR [Phase 3](../../vsdd-suite/primers/3-review-session.md)** runs the 7 default-active core domains per Review 42 doctrine (SE, QE, UX, Security, SA, SO, VDD-IAR Alignment). Rounds file as session entries in `vsdd-suite/review-log/YYYY-MM-DD-<slug>.md` per the [G-89](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-89) structural standard; project finding navigation is via [`vsdd-suite/FINDINGS-INDEX.md`](vsdd-suite/FINDINGS-INDEX.md).
- **[Phase 5 hardening](../../vsdd-suite/primers/5-formal-hardening.md)** (added Review 67 — Phase 5 adoption per v0.7.2 conventions; migrated to per-domain log shape per [G-177](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-177) / v0.7.8): per-layer Phase 5 rounds file in `vsdd-suite/review-log/2026-05-20-solution-architect.md` (Purity Boundary Audit) and `vsdd-suite/review-log/2026-05-20-quality-engineer.md` (Mutation Testing) with the `**Phase 5 surface:**` preamble tag per round. The Phase 5 strategy is declared in § Project intent below.

**Formal-proof candidates (Phase 5 Proof Execution):** none. `bookmark-cli` is not safety-critical or cryptographic; no function on the purity boundary above warrants Proof Execution. Proof Execution declared `not applicable` in the § Project intent Phase 5 strategy line.

**Automatable-vs-manual split:** every behavioral contract above is automatable via unit + integration tests. Manual testing (per TODO.md § Layer 1) verifies UX-coherence concerns (error message specificity; the empty-state stderr line as the user would read it) that automated tests can also assert syntactically but cannot evaluate as "reads naturally."

## Technology choices and rationale

| Choice | Alternatives considered | Why this |
|---|---|---|
| [Rust](https://www.rust-lang.org/) | [TypeScript](https://www.typescriptlang.org/)/Node, [Python](https://www.python.org/), Go | Matches the worked example's language; portfolio precedent (`issue-tracker-cli`); strong test/CLI ergonomics |
| Cargo workspace = single crate | Workspace with separate `lib` and `bin` crates | Over-engineering for one binary |
| `clap` (derive) | Hand-rolled arg parsing | Standard Rust CLI parser |
| `serde_json` | Custom JSON / TOML / sqlite | Spec calls for JSON |
| `chrono` (UTC) | `time` crate / system epoch ints | RFC 3339 formatting is well-supported |
| `anyhow` for error types | Custom error enums per `thiserror` | Single-binary tool; `thiserror` would be over-engineering |
| `assert_cmd` + `tempfile` for tests | Direct std::process invocation | CLI supplement § QE prescribes binary-invocation tests |

## Constraints

- **Rust toolchain:** 1.78+ (modern stable Rust; no unstable features). Pinned via [`rust-toolchain.toml`](rust-toolchain.toml) — Round 2 fix per [Platform Engineer Review 1 Finding 2](vsdd-suite/review-log/2026-05-20-platform-engineer.md).
- **Platform:** macOS, Linux. Windows untested.
- **Dependencies:** all from [crates.io](https://crates.io/), no git deps. `Cargo.lock` committed. Supply-chain policy enforced via [`deny.toml`](deny.toml) + `cargo deny check` in CI — Round 2 fix per [Security Review 1 Finding 3](vsdd-suite/review-log/2026-05-20-security.md) + [Platform Engineer Review 1 Finding 4](vsdd-suite/review-log/2026-05-20-platform-engineer.md).
- **Deployment:** `cargo install --locked --path .` into `~/.cargo/bin/`. No release pipeline. `--locked` flag enforces `Cargo.lock` at install time — Round 2 fix per [Platform Engineer Review 1 Finding 8](vsdd-suite/review-log/2026-05-20-platform-engineer.md).

## Performance budget ([Review 82](../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z) Round 2 fix for [Performance Engineer Review 1 Finding 1](vsdd-suite/review-log/2026-05-20-performance-engineer.md))

Layer 1 performance commitments:

| Metric | Budget (p95) | Measurement |
|---|---|---|
| `bm --help` / `bm --version` startup | < 50 ms wall-clock on commodity laptop | Manual observation; [`hyperfine`](https://github.com/sharkdp/hyperfine) acceptable for sanity-check |
| `bm add <url>` end-to-end | < 100 ms wall-clock on a store with ≤ 1,000 bookmarks | Same |
| `bm list` end-to-end | < 100 ms wall-clock on a store with ≤ 1,000 bookmarks | Same |

**Scale ceiling:** 10,000 bookmarks. Beyond this the user should consider a real bookmark manager — this project's non-goals (§ Scope and non-goals) declare unsuitability for primary-use scale. The flat-JSON-rewrite-on-every-add design has cumulative O(n²) cost which makes large stores impractical; declared as **accepted limitation** at Layer 1 intent + named in [Performance Engineer Review 1 Findings 3 + 6](vsdd-suite/review-log/2026-05-20-performance-engineer.md).

**Benchmarking infrastructure:** [Layer 2+](TODO.md) work — Layer 1's surface is too small to benchmark meaningfully ([`criterion`](https://github.com/bheisler/criterion.rs) adds dependency cost without commensurate value at this scale). [Performance Engineer Review 1 Finding 2](vsdd-suite/review-log/2026-05-20-performance-engineer.md) declared **Deferred** at the layer level; the budget above is the contract a future Layer-2 benchmarking infrastructure would assert against.

**Data-scaling tests:** sentinel tests at the 100 / 1,000 / 10,000-bookmark cliffs land at Layer 2+ ([Performance Engineer Review 1 Finding 5](vsdd-suite/review-log/2026-05-20-performance-engineer.md) **Deferred**). At Layer 1 the existing `save_then_load_roundtrips` test exercises the 1-bookmark case; the layer's correctness is observable from there.

## Threat model ([Review 82](../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z) Round 2 fix for [Security Review 1](vsdd-suite/review-log/2026-05-20-security.md) + [Red Team Review 1](vsdd-suite/review-log/2026-05-20-red-team.md))

**In-scope adversaries:**

- **Co-tenant on a shared Unix host** — read access to the user's home directory hierarchy. **Mitigation:** storage file mode 0600 (read/write owner only) per the *confidential* data classification below.
- **Adversary-controlled `$BOOKMARK_CLI_DB`** — the env var points at a writable path the user does not control (e.g., a shared `/tmp/...`, a directory with a pre-staged symlink). **Mitigations:** symlink-follow-rejection on **both** load and save (symmetric `symlink_metadata` check + rejection) per the symlink-hardening discipline; the env var is the user's own shell + the user is responsible for what they set. **Residual TOCTOU** — the load-side `symlink_metadata` check and the subsequent `read_to_string` are separate syscalls; an attacker with concurrent filesystem write access to the parent directory could swap a regular file for a symlink in the microsecond race window ([Red Team Review 1 Round 3 Finding 2](vsdd-suite/review-log/2026-05-20-red-team.md#r3-f2) **Accepted risk**). Tight fix is `OpenOptions::custom_flags(O_NOFOLLOW)` (single-syscall atomic check), which is deferred pending a `libc` dependency addition and Platform Engineer / Security re-review. The save side uses `rename(2)` which is atomic regardless.
- **Adversary-supplied URL contents** — a URL captured at one terminal session is later rendered at `bm list` in another terminal session. URLs can carry terminal-escape sequences (ANSI `\x1b[...`, OSC 0/8/1337, bidi format chars U+202E + zero-width chars). **Mitigation:** `display_safe` sanitizer wraps every user-derived value before any `eprintln!` / `println!` / `Display` interpolation — escapes `is_control()` (Cc) chars + `Cf` format chars while preserving `\n` `\t` for legitimate whitespace.

**Out-of-scope adversaries:**

- **Same-user concurrent process** writing the storage file at the same time — the project is a single-user single-process tool per § Scope and non-goals; concurrent-write race is **accepted risk** ([Red Team Review 1 Finding 3](vsdd-suite/review-log/2026-05-20-red-team.md)).
- **Unbounded URL length** — the spec accepts arbitrarily long URLs per the original § Edge case catalog. DoS-via-memory is acknowledged but accepted at Layer 1 ([Red Team Review 1 Finding 2](vsdd-suite/review-log/2026-05-20-red-team.md) **Accepted risk**).
- **TOCTOU between `path.exists()` and `read_to_string()` in `BookmarkStore::load`** — single-process foreclosure makes the race window non-exploitable ([Security Review 1 Finding 5](vsdd-suite/review-log/2026-05-20-security.md) **Accepted risk**).
- **JSON-parser depth-bomb (deeply-nested user-controlled JSON)** — `serde_json` enforces a 128-level recursion limit by default; the attacker model does not grant write access to the store file ([Security Review 1 Finding 6](vsdd-suite/review-log/2026-05-20-security.md) **Hallucinated** — verified protection holds).

## Storage data classification ([Review 82](../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z) Round 2 fix for [Security Review 1 Finding 2](vsdd-suite/review-log/2026-05-20-security.md))

The captured bookmarks are **confidential**-class data — "what someone is reading is sensitive" per the [Security domain prompt](../../vsdd-suite/domains/role/SECURITY-REVIEW.md) Dim 8 information-leakage classification. The storage file is written with **mode 0600** (Unix; read/write owner only) using `std::fs::OpenOptions::new().mode(0o600)...` behind a `#[cfg(unix)]` gate. Windows is named as untested under § Constraints; Windows file-permission semantics differ from Unix and are deferred to a Windows-port layer.

Encryption at rest is **not** in scope at Layer 1 — mode 0600 is the spec's floor for confidential-class data on Unix, per the Security domain prompt's proportionality discipline. A future layer (or production-intent fork) may add at-rest encryption if the spec's data-classification rises.

## Open questions

*(none at the close of Phase 1a — the self-adversary check completed cleanly because the project's scope is small and the contracts are observable from outside the implementation. Any ambiguities surfaced during Phase 2 or Phase 3 will be routed back to this section per [Phase 4](../../vsdd-suite/primers/4-feedback-integration.md) routing discipline.)*
