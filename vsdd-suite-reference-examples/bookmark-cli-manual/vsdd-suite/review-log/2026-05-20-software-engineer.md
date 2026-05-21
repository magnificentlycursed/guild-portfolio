# Software Engineer Review — bookmark-cli-manual — 2026-05-20

[Index](../SOFTWARE-ENGINEER-REVIEW.md)

---

## Review 1 — 2026-05-20 19:30Z

**Scope:** Cold-context [Software Engineer](../../../../vsdd-suite/domains/role/SOFTWARE-ENGINEER-REVIEW.md) IAR Round 1 against Layer 1 of [bookmark-cli-manual](../../README.md). Artifacts read in cold-reader order: [`README.md`](../../README.md), [`Cargo.toml`](../../Cargo.toml), [`src/main.rs`](../../src/main.rs), [`src/lib.rs`](../../src/lib.rs), [`tests/bookmarks.rs`](../../tests/bookmarks.rs), [`TODO.md`](../../TODO.md), [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md), then [`DESIGN.md`](../../DESIGN.md) last (author-natural cold read; spec read after implementation per the [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md)). No prior SE round exists for this project — this is the first SE-domain pass.

**Layer:** 1
**Tested against:** commit `5f326bc4` (current `main` as of 2026-05-20)
**Round:** 1
**Active domain set:** 11 role + 1 meta = 12 (per [DESIGN.md § Project intent](../../DESIGN.md))
**Lens:** Standard SE dimensions emphasized: Dim 1 (Correctness), Dim 2 (Error handling), Dim 3 (Naming and type precision — primitive obsession), Dim 4 (Function and method design), Dim 8 (Defensive coding). [Rust supplement](../../../../vsdd-suite/supplements/rust.md) § Software Engineering: `.unwrap()` discipline, error-type hierarchy, Clippy lint configuration. Dim 12 (Test seam attack surface) checked mechanically (`grep -E 'INTERNAL_|TEST_|_FORCE_|_BYPASS_|_SEAM|cfg\(any\(test|cfg\(debug_assertions|debug_assert!'` against `src/`) — no hits; the `BOOKMARK_CLI_DB` env var is a user-facing configuration documented in DESIGN.md, not a test seam. Documentation dimensions (13–17) and Performance dimensions (18–22) NOT applied here — [Technical Writer](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) and [Performance Engineer](../../../../vsdd-suite/domains/role/PERFORMANCE-ENGINEER-REVIEW.md) are both activated per [`DESIGN.md`](../../DESIGN.md) § Project intent so documentation/performance finding ownership defers to those domains per the SE prompt's deferral rule.

**Session note:** Cold session. The reviewer did not build, design, or previously read this project; reading order followed the [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md)'s cold-context discipline (primer → domain prompt → language supplement → governing standard → project artifacts in author-natural order → DESIGN.md last → existing rounds for shape only, not findings). No findings were softened to reconcile with the existing [QE](2026-05-20-quality-engineer.md) and [SA](2026-05-20-solution-architect.md) rounds; the existing rounds covered Phase-5 territory (Mutation Testing, Purity Boundary Audit), not the Phase-3 SE territory this round covers.

**Source:** `domain-raised` — every finding below was elicited by applying the SE dimensions from a cold seat.

**Assumption surfacing:** External dependencies declared in [`Cargo.toml`](../../Cargo.toml) — [clap](https://docs.rs/clap) 4 (derive), [serde](https://serde.rs/) 1 (derive), [serde_json](https://docs.rs/serde_json) 1, [chrono](https://docs.rs/chrono) 0.4 (clock, serde), [anyhow](https://docs.rs/anyhow) 1; dev: [assert_cmd](https://docs.rs/assert_cmd) 2, [predicates](https://docs.rs/predicates) 3, [tempfile](https://docs.rs/tempfile) 3. **Verified assumption:** clap 4 derive `Subcommand` enum with a required positional `url: String` argument causes clap to call `std::process::exit(2)` on missing-argument parse failure (clap default `error_exit_code = 2`), bypassing the `ExitCode` returned by `fn main`. This assumption is load-bearing for [Finding 1](#r1-f1) below — verified against clap's documented behavior, not just assumed. **Verified assumption:** `chrono::Utc::now()` returns a `DateTime<Utc>` whose `to_rfc3339()` rendering matches the contract in [`DESIGN.md`](../../DESIGN.md) § Interface definitions — confirmed against [chrono](https://docs.rs/chrono/0.4/chrono/) 0.4 docs. **Flagged assumption (unverified in-session, propose verification):** `std::fs::write(path, ...)` is not atomic against process death between truncate and complete write — this is the standard POSIX `O_WRONLY|O_CREAT|O_TRUNC` open-and-write sequence, NOT an atomic operation. [Finding 2](#r1-f2) below depends on this — propose verification by deliberately killing the process mid-`save` (e.g., via `SIGKILL` from another shell) and inspecting the file state, OR by reading the [`std::fs::write`](https://doc.rust-lang.org/std/fs/fn.write.html) source for the underlying syscall sequence.

---

### Deferred

**Finding 1 — `bm add` with no positional argument bypasses the contracted exit code and stderr (Dim 1, Dim 2)**

<a id="r1-f1"></a>

[`TODO.md`](../../TODO.md) AC 2 states explicitly: "`bm add` (no positional argument, or empty-string argument) exits 1 with stderr `Error: URL cannot be empty.\n` and writes nothing to the store." The implementation at [`src/main.rs:43-47`](../../src/main.rs) only handles the empty-string case:

```rust
Cmd::Add { url } => {
    if url.is_empty() {
        eprintln!("Error: URL cannot be empty.");
        return ExitCode::from(1);
    }
```

The missing-argument case (`bm add` with no positional) is delegated entirely to [clap](https://docs.rs/clap) at [`src/main.rs:39`](../../src/main.rs) (`let cli = Cli::parse();`). clap 4's default behavior on a missing required argument is `std::process::exit(2)` with its own error message rendered to stderr (something like `error: the following required arguments were not provided: <URL>`), NOT the contracted exit code 1 with the contracted message `Error: URL cannot be empty.`. The exit happens inside `Cli::parse()` so the `ExitCode` returned by `fn main` never gets a chance to override it.

[`DESIGN.md`](../../DESIGN.md) § Behavioral contracts `### PROT_37` is ambiguous on the missing-argument case — it names "Input shape: exactly one positional argument, a non-empty string" and lists the "Failure (empty URL)" branch but does not explicitly address the "missing argument" branch. So there is **simultaneously** an impl-vs-`TODO.md` divergence (which I flag here) AND a `DESIGN.md`-vs-`TODO.md` ambiguity (which the SO must reconcile).

The integration test suite has no test for the missing-argument case — the only `bm add` failure test is `tests_add_rejects_empty_url` at [`tests/bookmarks.rs:47-63`](../../tests/bookmarks.rs), which uses `.args(["add", ""])` (empty-string positional, not missing positional). So the divergence is invisible to `cargo test`.

**Why it matters:** A user who types `bm add` (forgetting to paste the URL) sees a clap usage error and exit code 2 (the same code reserved by [`DESIGN.md`](../../DESIGN.md) § Exit codes for "Storage error"). The user-facing experience contradicts the documented contract; any script that distinguishes "user forgot URL" (code 1) from "storage is broken" (code 2) by checking exit code is wrong.

**Owner:** software-engineer
**Status:** raised
**Blocked by:** [Finding 3](#r1-f3) — the resolution for this finding depends on the SO's disposition of the exit-code-2 overloading: if [`DESIGN.md`](../../DESIGN.md) is updated to assign clap usage errors a distinct code, this finding can be fixed by overriding `clap::Command::exit_status` to use code 1 for missing-required-positional errors; if [`DESIGN.md`](../../DESIGN.md) is updated to accept exit code 2 for clap usage errors, this finding becomes an [`TODO.md`](../../TODO.md) AC 2 amendment.
**Validator:** quality-engineer

**Classification:** Open — the resolution is structural (override clap's exit behavior OR amend the spec) and crosses into SO territory; flagging open here and routing the spec piece to SO via [Finding 3](#r1-f3).

---

**Finding 2 — `BookmarkStore::save` is not atomic — partial-write violates DESIGN.md's "No partial write" contract (Dim 1, Dim 8)**

<a id="r1-f2"></a>

[`DESIGN.md`](../../DESIGN.md) § Behavioral contracts `### PROT_37` includes: "**Failure (storage file unreadable / unwritable):** stderr `Error: <descriptive message>` followed by newline. Exit 2. **No partial write.**" (Emphasis added — the "no partial write" clause is a positive durability guarantee, not a fall-through.)

The implementation at [`src/lib.rs:60-72`](../../src/lib.rs):

```rust
pub fn save(&self, path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("creating parent dir for {}", path.display()))?;
        }
    }
    let json = serde_json::to_string_pretty(self)
        .context("serializing bookmark store to JSON")?;
    std::fs::write(path, json + "\n")
        .with_context(|| format!("writing bookmark store at {}", path.display()))?;
    Ok(())
}
```

`std::fs::write` is the standard convenience wrapper that opens the path with `O_WRONLY | O_CREAT | O_TRUNC`, writes the buffer, and closes. Between the `O_TRUNC` (which immediately empties the existing file) and the buffer write completing, the file is left in an indeterminate state. If the process is killed (`SIGKILL`, `kill -9`, OOM kill, power loss, panic outside this code path) anywhere in that window — including, on a slow disk under load, during the write itself if it spans multiple flushed blocks — the on-disk file is observably partial: either zero bytes or a prefix of the new content. The prior contents are gone in either case.

For a [bookmark-cli](../../README.md) user, this means: invoking `bm add` against an existing store with N bookmarks, suffering an unrelated process kill mid-`save`, then invoking `bm list` produces either "No bookmarks yet." (file truncated to zero) or [`Error: parsing bookmark store at ... — EOF while parsing a value at line N column M`](../../src/lib.rs) (file truncated mid-JSON). **In both cases the N pre-existing bookmarks are unrecoverable.**

The defensible-coding fix is the standard atomic-write pattern: serialize to bytes; write to `path.tmp` (same directory, same filesystem, so `rename` is atomic on Unix); `std::fs::rename(path.tmp, path)`. The `rename` syscall is atomic on POSIX — the old file's contents survive until the rename succeeds, and after the rename the file points at the new contents.

**Why it matters at Layer 1:** The reference-implementation purpose ([G-112](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112)) is to exercise the suite end-to-end and to teach what the suite documents. Shipping a documented "no partial write" contract that the implementation does not honor is the exact "implementation does what's generated, not what's specified" sycophancy failure the SE prompt names. A future reader using [bookmark-cli](../../README.md) as a template will internalize "Rust's `std::fs::write` satisfies a no-partial-write contract" — it does not.

**Verification proposed:** the assumption that `std::fs::write` is non-atomic is testable empirically by issuing `bm add` against a store on a tmpfs while running `kill -9` on the `bm` process from a sibling shell in a tight loop and inspecting `bookmarks.json` afterward. I did not run this in-session (cold-reader scope). The supplement-source check (Rust std docs for `std::fs::write`) is sufficient evidence to raise the finding.

**Owner:** software-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** quality-engineer

**Classification:** Open — the implementation change is local (write-to-tempfile + rename in `BookmarkStore::save`), and the validating test is a new test in [`src/lib.rs`](../../src/lib.rs)'s `#[cfg(test)] mod tests` block asserting that after an interrupted-save simulation (e.g., a manually-pre-written `bookmarks.json.tmp` left over from a prior crash), `BookmarkStore::load` still returns the pre-crash state. Coordination note: this is also a QE Dim 2 (test falsifiability) concern — the existing `save_then_load_roundtrips` test at [`src/lib.rs:133-142`](../../src/lib.rs) exercises the happy path only; a crash-during-save test would catch a regression if a future refactor reverts the atomic-write fix.

---

### Raised to SO

**Finding 3 — Exit code 2 has two distinct meanings: "Storage error" (Dim 1, Dim 2)**

<a id="r1-f3"></a>

[`DESIGN.md`](../../DESIGN.md) § Interface definitions § Exit codes assigns:

| Code | Meaning |
|---|---|
| 0 | Success (including empty `bm list`) |
| 1 | User error (empty URL) |
| 2 | Storage error (file unreadable, corrupt JSON, write failure) |

The implementation honors this in the storage-error branches at [`src/main.rs:52, 58, 67`](../../src/main.rs) (`return ExitCode::from(2);` for each `BookmarkStore::load`/`save` failure). However, clap's default exit code on argument-parse failure is also 2 (per clap 4's defaults — [clap docs](https://docs.rs/clap/4/clap/error/enum.ErrorKind.html) note `error::ErrorKind` rendering uses status 2). The implementation does not override this default. Concrete cases that exit with code 2 via the clap path:

- `bm` (no subcommand) — clap prints help-or-error to stderr, exits 2
- `bm add` (no positional URL) — clap prints "required arguments" error, exits 2
- `bm bogus` (unknown subcommand) — clap prints unknown-subcommand error, exits 2
- `bm add --unknown-flag https://foo` — clap prints unknown-flag error, exits 2

A consumer (a shell script, a test harness, a user reading the manpage) observing exit code 2 cannot tell whether the user mistyped the command or the on-disk store is corrupt. These are very different failure modes — one is the user's fault, one is a data-integrity event.

**Two specification paths to resolve:**

1. **Reassign codes in [`DESIGN.md`](../../DESIGN.md) so clap parse failures map to a distinct code** (e.g., 64 per `sysexits.h` `EX_USAGE`, or simply 3). This is the cleaner contract — usage errors and storage errors are kept distinct. Implementation change: override `clap::Command` to use a non-2 exit code for `ErrorKind` variants that represent usage errors. Updates required: [`DESIGN.md`](../../DESIGN.md) Exit codes table; [`TODO.md`](../../TODO.md) AC 2 (which currently says missing-URL exits 1); [`src/main.rs`](../../src/main.rs) clap configuration; new integration tests in [`tests/bookmarks.rs`](../../tests/bookmarks.rs) covering each usage-error case.

2. **Acknowledge the overlap in [`DESIGN.md`](../../DESIGN.md)** ("Exit code 2 covers any failure where the user input or the storage layer prevented the command from running; finer-grained distinction is out of scope at Layer 1"). This is the easier path but loses signal — a downstream script consuming the binary's exit code can no longer distinguish the two cases.

The SE-internal preference is path 1 (clearer contract), but the choice is a spec decision, not an implementation decision. Raising to SO per the SE prompt's `DESIGN.md change authority` paragraph.

**Owner:** solution-owner
**Status:** raised
**Blocked by:** *(none)*
**Validator:** solution-architect — SA Dim "CLI parsing separated from business logic" (Rust supplement § Solution Architect) is the natural cross-domain pair: the resolution may move clap configuration into a dedicated `cli.rs` module to keep the exit-code-mapping logic out of `main.rs`, which is an SA-shaped change. SA validates that the resulting structure honors the supplement's "thin `main.rs` + business logic in modules" guidance.

**Classification:** Raised to SO — proposed change above; rationale: exit-code semantics are part of the project's external interface contract and live in [`DESIGN.md`](../../DESIGN.md), which only the [Solution Owner](../../../../vsdd-suite/domains/role/SOLUTION-OWNER-REVIEW.md) can amend.

---

**Finding 4 — `BookmarkStore` and `Bookmark` expose mutable `pub` fields, bypassing the `add` method's invariants (Dim 3, Dim 4, Dim 8)**

<a id="r1-f4"></a>

[`src/lib.rs:27-37`](../../src/lib.rs):

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Bookmark {
    pub url: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct BookmarkStore {
    #[serde(default)]
    pub bookmarks: Vec<Bookmark>,
}
```

All three fields (`Bookmark::url`, `Bookmark::timestamp`, `BookmarkStore::bookmarks`) are `pub`. The library exposes `BookmarkStore::add(url)` ([`src/lib.rs:76-81`](../../src/lib.rs)) as the intended mutation path — its doc-comment names the contract: "Caller is responsible for non-empty URL validation." But because `bookmarks` is `pub`, any caller can sidestep `add` entirely:

```rust
let mut store = BookmarkStore::default();
store.bookmarks.push(Bookmark {
    url: String::new(),              // empty — violates the contract
    timestamp: DateTime::<Utc>::MIN_UTC,  // arbitrary, possibly bogus
});
```

This is a Dim 4 encapsulation gap (the `add` method's intent is not enforceable via the type system because the underlying field is `pub`) and a Dim 8 defensive-coding gap (the type's invariants — non-empty URL; sensible timestamp — are documented in prose but not enforced at the boundary). It is also a Dim 3 primitive-obsession concern: `url: String` and `bookmarks: Vec<Bookmark>` are the rawest possible types; a `Url(String)` newtype with a non-empty constructor, or a `BookmarkList(Vec<Bookmark>)` private-field wrapper, would close the hole.

**Why it matters even at Layer 1:** [`Cargo.toml`](../../Cargo.toml) configures both `[lib]` (`name = "bookmark_cli"`) and `[[bin]]` (`name = "bm"`) targets — so the library IS a callable API surface, not just an internal binary helper. Anyone who depends on `bookmark_cli` as a dev-dep, or who copies the library shape into another project (the reference-implementation purpose), inherits the encapsulation gap. The empty-URL check in [`src/main.rs:44`](../../src/main.rs) is correct as written but is not the only path into the library; a future Layer 2 (`bm tag`, `bm list --tag`) that adds another mutation path could easily forget to re-check.

**The defensible fix (small, idiomatic):**

```rust
pub struct Bookmark {
    url: String,
    timestamp: DateTime<Utc>,
}

impl Bookmark {
    pub fn url(&self) -> &str { &self.url }
    pub fn timestamp(&self) -> &DateTime<Utc> { &self.timestamp }
}

pub struct BookmarkStore {
    bookmarks: Vec<Bookmark>,
}

impl BookmarkStore {
    pub fn add(&mut self, url: String) -> Result<()> {
        if url.is_empty() { return Err(anyhow!("URL cannot be empty")); }
        self.bookmarks.push(Bookmark { url, timestamp: Utc::now() });
        Ok(())
    }
    pub fn bookmarks(&self) -> &[Bookmark] { &self.bookmarks }
    // ... existing newest_first / load / save
}
```

This shape (a) enforces the invariant at the type boundary, (b) keeps the binary's main.rs essentially unchanged (it already calls `store.add(url)` after the `is_empty` check; now the check is in the library where it belongs), and (c) preserves the serde round-trip via the same struct shape — serde's `#[derive(Serialize, Deserialize)]` does not require fields to be `pub`.

**Tradeoff:** The current `pub` fields likely exist to make the integration test at [`tests/bookmarks.rs:36-38`](../../tests/bookmarks.rs) ergonomic — the test reads `parsed["bookmarks"]` from the on-disk JSON, not via the type. The proposed fix does not affect that test (it operates on the JSON shape, not the struct). The `src/lib.rs` `#[cfg(test)] mod tests` block does use the `pub` fields directly (e.g., [`src/lib.rs:107`](../../src/lib.rs) constructs `Bookmark { url, timestamp: ts }` via positional field syntax) — these tests would need to be updated to use a private constructor or a test-only `pub(crate)` shortcut. The fix is local and small; the encapsulation gain is real.

**Owner:** software-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** quality-engineer

**Classification:** Open — the fix is a local refactor (un-`pub` the fields; add accessor methods; route `add`'s emptiness check into the library). Surface-level breaking change to the library API but the only declared caller is `src/main.rs` which already uses the `add` method.

---

**Finding 5 — Crate-level Clippy / `missing_docs` deny set is absent; no documented rationale (Rust supplement § Software Engineering — Clippy lint configuration)**

<a id="r1-f5"></a>

The [Rust supplement](../../../../vsdd-suite/supplements/rust.md) § Software Engineering names the standard deny set as a floor:

> `#![deny(clippy::all, clippy::pedantic, clippy::nursery, clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::missing_errors_doc, clippy::missing_panics_doc, missing_docs)]`

> Any deviation from this baseline requires documented rationale. Selective `#[allow(...)]` with a comment is acceptable; a weaker global deny set is a finding.

Neither [`src/lib.rs`](../../src/lib.rs) nor [`src/main.rs`](../../src/main.rs) carries a crate-level `#![deny(...)]` attribute; [`Cargo.toml`](../../Cargo.toml) has no `[lints]` table; there is no `clippy.toml` in the project root. The crate runs at the Cargo default lint level, which is roughly `clippy::correctness = deny, everything else = allow-or-warn`. The supplement's baseline is missing in full.

The consequence is twofold:

1. **The `.unwrap()`/`.expect()` discipline named in the Rust supplement § SE first bullet is not enforced at lint time.** All `.unwrap()` calls in the project happen to be in `#[cfg(test)] mod tests` blocks (verified by `grep -n 'unwrap\|expect\|panic' src/`) so the project would still pass `clippy::unwrap_used = deny` today — but a future Layer-2 contributor adding a `.unwrap()` on a user-facing path (e.g., parsing a tag string) would not see a clippy error.

2. **The `missing_docs` rustc lint is not enabled at the crate level.** The [Technical Writer](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) supplement explicitly cross-references this — `missing_docs` in the crate-level deny set catches missing public-item documentation at clippy/cargo-check time rather than only at `cargo doc` time, per the [G-137](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-137) recurrence. Today, `BookmarkStore::bookmarks` field, `Bookmark::url` field, `Bookmark::timestamp` field, and the `Cmd` enum and `Cli` struct in [`src/main.rs`](../../src/main.rs) all lack `///` doc comments; none of this is caught by `cargo clippy` or `cargo check` because the lint is off by default.

The fix is a one-line addition at the top of [`src/lib.rs`](../../src/lib.rs) (and a matching `#![deny(missing_docs)]` in [`src/main.rs`](../../src/main.rs) if the binary's items also fall under the rule, though the supplement says "internal functions in `main.rs` may be omitted" — so a narrower deny set is acceptable for the bin target). Adding the deny set may surface new findings from clippy::pedantic / clippy::nursery — those should be triaged into either fixes or scoped `#[allow(...)]` with a comment, per the supplement.

**Why it matters even at Layer 1:** [bookmark-cli-manual](../../README.md) is the reference implementation for the [VSDD suite](../../../../vsdd-suite/README.md)'s worked example. A worked example that does not exhibit the lint discipline the suite teaches in its own Rust supplement undercuts the teaching. Per the [G-177](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-177) reference-examples-migrate precedent, reference examples are expected to land at the bar they teach.

**Owner:** software-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** quality-engineer — Rust supplement § Quality Engineering "Clippy compliance" is the natural cross-domain pair (QE owns the test-side verification that `cargo clippy` passes; SE owns the per-crate lint declaration that makes the verification meaningful). A platform-engineering coordination note also applies — [Platform Engineer](../../../../vsdd-suite/domains/role/PLATFORM-ENGINEER-REVIEW.md) Rust supplement names `cargo clippy --deny warnings` in CI, which depends on the crate-level deny set being correct.

**Classification:** Open — the fix is a literal one-line attribute addition plus per-finding triage of any new lints surfaced. The Layer 1 surface is small enough that the lint pass should produce a tractable set of follow-ups.

---

### Dismissed

*(none)*

---

### Deferred

*(none)*

---

### Hallucinated

*(none — every finding above cites a specific file:line or a specific [`DESIGN.md`](../../DESIGN.md) clause and a specific implementation path that diverges from it. The cold-reader-against-spec discipline produced 5 substantive findings against a small Layer 1 surface (~80 LOC in [`src/lib.rs`](../../src/lib.rs) + [`src/main.rs`](../../src/main.rs)); no finding was raised on speculation.)*

---

### Summary

5 findings raised in-session: 4 SE-owned ([Finding 1](#r1-f1) clap usage-error contract; [Finding 2](#r1-f2) non-atomic save; [Finding 4](#r1-f4) `pub` field encapsulation; [Finding 5](#r1-f5) missing crate-level lint floor) + 1 Raised to SO ([Finding 3](#r1-f3) exit-code 2 overloading). The implementation is small and broadly idiomatic — `.unwrap()` discipline is clean on user-facing paths; clap-derive separates parsing from logic well; the [`src/lib.rs`](../../src/lib.rs) module doc accurately reflects the post-Phase-5 purity boundary documented by [SA Review 1](2026-05-20-solution-architect.md#review-1--2026-05-20-0245z). The findings cluster on the gap between the documented contract (DESIGN.md) and the implementation's actual surface: clap's default exit codes don't match DESIGN.md's exit-code table; `std::fs::write` doesn't satisfy DESIGN.md's "no partial write" guarantee; `pub` fields don't enforce the library's documented "callers must validate" contract; the suite's own Rust supplement lint floor is not configured. None of these are catastrophic at the single-user-Layer-1 surface, but all four shape what the reference implementation teaches downstream — and the reference-implementation purpose ([G-112](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112)) is to teach.

Per the [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) § Round triggers (G-131 continue trigger): this round produced 5 new real findings, so SE Round 2 against [bookmark-cli-manual](../../README.md) Layer 1 is mandatory after the findings above are resolved — the Round 2 cold pass verifies the fixes held and looks for adjacent defects the fixes may have created.

**Coordination:**

- [Finding 1](#r1-f1) and [Finding 3](#r1-f3) are jointly routed: [Finding 3](#r1-f3) is the SO-spec piece (exit-code reassignment in [`DESIGN.md`](../../DESIGN.md)); [Finding 1](#r1-f1) is the SE-impl piece (clap exit-code override) and blocks on [Finding 3](#r1-f3)'s resolution. The natural sequencing is: SO disposes [Finding 3](#r1-f3) → SE implements the result against [Finding 1](#r1-f1) → QE adds the integration tests for each usage-error case. Route to [SOLUTION-OWNER-REVIEW.md](../SOLUTION-OWNER-REVIEW.md) for [Finding 3](#r1-f3).
- [Finding 2](#r1-f2) (non-atomic save) is jointly an SE implementation fix and a QE test-falsifiability concern — a crash-during-save regression test belongs in [`src/lib.rs`](../../src/lib.rs)'s test module. Route the test-design piece to [QUALITY-ENGINEER-REVIEW.md](../QUALITY-ENGINEER-REVIEW.md). Also surface to [Red Team](../../../../vsdd-suite/domains/role/RED-TEAM-REVIEW.md) (capstone-active per [`DESIGN.md`](../../DESIGN.md) § Project intent) — a "no partial write" contract that doesn't hold under SIGKILL is a small adversarial surface (data-loss vector under interruption), worth at least one cold-reader pass from RT.
- [Finding 4](#r1-f4) (encapsulation) is jointly an SE fix and an [SA](../../../../vsdd-suite/domains/role/SOLUTION-ARCHITECT-REVIEW.md) architecture concern — the `pub`-field-vs-accessor-method choice is module-boundary design. Surface to [SOLUTION-ARCHITECT-REVIEW.md](../SOLUTION-ARCHITECT-REVIEW.md) for the design adjudication.
- [Finding 5](#r1-f5) (lint floor) cross-references the [PLATFORM-ENGINEER-REVIEW.md](../PLATFORM-ENGINEER-REVIEW.md) Rust supplement (`cargo clippy --deny warnings` in CI) and the [TECHNICAL-WRITER-REVIEW.md](../TECHNICAL-WRITER-REVIEW.md) Rust supplement (`missing_docs` cross-reference per [G-137](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-137)). The crate-level declaration is SE-owned; the CI enforcement is PE-owned; the doc-coverage downstream is TW-owned. Surface to PE and TW for their respective downstream pieces.
- **Non-SE-owned observation surfaced for [TW](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md):** [`DESIGN.md`](../../DESIGN.md) § Behavioral contracts uses placeholder-looking section identifiers `### PROT_37` and `### PROT_41` (with extra leading spaces), and [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md) Step 1/3/4 headings contain similar tokens `PROT_30`, `PROT_40`, `PROT_46`. These read as unresolved templating tokens rather than human-readable section names ("Add command contract" / "List command contract" would be the natural descriptive form). The [Naming and identifier discipline](../../../../vsdd-suite/suite-development/suite-development.md#naming-and-identifier-discipline-review-78-finding-4) governing standard explicitly retired letter-and-number labels in the suite (Review 78) in favor of descriptive names — the same discipline applied to a reference example would flag these. Documentation-finding ownership defers to [TW](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) per the SE prompt's deferral rule; flagging here as a Coordination handoff rather than an SE finding.
