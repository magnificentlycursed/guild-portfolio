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

---

## Review 2 — 2026-05-20 21:00Z

**Layer:** 1
**Tested against:** commit `1534198d` (current `main` as of 2026-05-20)
**Round:** 2
**Active domain set:** 11 role + 1 meta = 12 (per [DESIGN.md § Project intent](../../DESIGN.md))
**Scope:** Round 2 verification of [Round 1 SE findings](2026-05-20-software-engineer.md) (F1–F5) against the post-Round-2 fix cycle. Independent adversarial pass also looks for new defects in the stabilized state per the [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) § Round triggers G-131 framing — "the Round N+1 cold pass verifies the fix held and looks for adjacent defects the fix may have created."
**Lens:** Software Engineer dims applied to the post-fix state per [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) adversarial discipline. Same dim emphasis as Round 1 (Dim 1 Correctness; Dim 2 Error handling; Dim 4 Function and method design; Dim 8 Defensive coding) plus targeted scan for Round-2-introduced complexity (Dim 6) and brittle assumptions on external API behavior (Rust supplement § Software Engineering — clippy lint configuration; G-20 assumption surfacing). [Rust supplement](../../../../vsdd-suite/supplements/rust.md) § Software Engineering re-applied — verified the standard deny set against the post-fix `[lints]` table in [`Cargo.toml`](../../Cargo.toml) and the crate-level attributes in [`src/lib.rs`](../../src/lib.rs) / [`src/main.rs`](../../src/main.rs).
**Source:** `domain-raised` — the post-fix verification pass is cold-context against the fix artifacts; the adjacent-defect Findings 6 and 7 below were elicited by applying SE Dim 1 (Correctness — regression check) and the Rust supplement § Software Engineering clippy-deny-set floor to the new code paths Round 2 introduced.
**Regression check:** every Round 1 finding re-evaluated against current state per the [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) § Current Review Prompt regression-check clause. The new findings below explicitly cite which Round 1 fix introduced the regression (Finding 6) or which Round 1 finding's "Resolved" claim is incomplete (Finding 7); no new finding raises a defect that Round 1 already covered without naming the prior finding.

**Session note:** Cold session. The reviewer did not author the Round 2 fixes nor participate in Round 1; reading order: [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) → [SE domain prompt](../../../../vsdd-suite/domains/role/SOFTWARE-ENGINEER-REVIEW.md) → [Rust supplement](../../../../vsdd-suite/supplements/rust.md) § Software Engineering → [Round 1 SE log](2026-05-20-software-engineer.md) → [`DESIGN.md`](../../DESIGN.md) → [`src/main.rs`](../../src/main.rs) → [`src/lib.rs`](../../src/lib.rs) → [`Cargo.toml`](../../Cargo.toml) → [`tests/bookmarks.rs`](../../tests/bookmarks.rs) → governing standard. No tests/clippy were executed in-session (cold-reader scope); verification proceeds by source inspection against the Rust supplement's documented contracts and clap 4's documented `ErrorKind` semantics.

**Assumption surfacing:** Round 2 introduced a new external-API assumption that warrants explicit surfacing — the [`src/main.rs:57-77`](../../src/main.rs) `handle_parse_error` function relies on (a) clap 4's `Error::kind()` returning `ErrorKind::MissingRequiredArgument` for the bare-`bm add` case AND (b) clap 4's `ContextKind::InvalidArg` rendering the missing positional's metavar as `<URL>` (uppercase, from the field name `url` per clap's default value-name derivation) AND (c) `ErrorKind::DisplayHelp` / `ErrorKind::DisplayVersion` being distinct variants from the error kinds in scope. Assumptions (a) and (b) are verified against clap 4 docs and the integration test [`tests/bookmarks.rs:165-184`](../../tests/bookmarks.rs) (`bm_add_with_no_positional_exits_1_with_url_cannot_be_empty`). Assumption (c) is load-bearing but **not** verified by any in-tree test — [Finding 6](#r2-f6) below is the failure surface.

---

### Resolved

**Finding 1 — `bm add` with no positional argument bypasses the contracted exit code and stderr (Dim 1, Dim 2)**

<a id="r2-f1"></a>

**Owner:** software-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** quality-engineer

Round 1's [Finding 1](2026-05-20-software-engineer.md#r1-f1) raised the divergence: `bm add` (no positional) exited 2 via clap's default rather than the spec-contracted exit 1 with `Error: URL cannot be empty.`. Verifying the Round 2 fix:

1. **Implementation path.** [`src/main.rs:79-83`](../../src/main.rs) switches from `Cli::parse()` (which exits the process inside the call on parse failure) to `Cli::try_parse()` + explicit error routing through `handle_parse_error` at [`src/main.rs:57-77`](../../src/main.rs). The handler matches `ErrorKind::MissingRequiredArgument` and emits `Error: URL cannot be empty.\n` + `ExitCode::from(1)` when the missing argument's `InvalidArg` context name contains the `URL`/`<URL>`/`<url>` substring — which it does, because the `url: String` field at [`src/main.rs:43`](../../src/main.rs) has no explicit `value_name` so clap derives the metavar from the field name as `URL` (uppercase).

2. **Test coverage.** [`tests/bookmarks.rs:165-184`](../../tests/bookmarks.rs) `bm_add_with_no_positional_exits_1_with_url_cannot_be_empty` invokes `bm add` (no positional) and asserts exit 1 + stderr `Error: URL cannot be empty.\n` + stdout empty + no file write. Verified to match the spec contract at [`DESIGN.md` § `bm add`](../../DESIGN.md) line 60.

3. **Spec alignment.** [`DESIGN.md`](../../DESIGN.md) line 60 was amended to make the "no positional argument given" branch explicit and identical to the empty-string branch — closes the Round 1 ambiguity that had also been flagged.

**Resolution:** The Round 2 fix is structurally sound for the in-scope case (`bm add` with no positional argument). The substring heuristic at [`src/main.rs:66`](../../src/main.rs) is fragile (it depends on clap's default value-name rendering staying uppercase + angle-bracketed for the foreseeable future), but the current behavior is correct against clap 4 and is exercised by the integration test. A future-clap-rendering-change is a Layer-2 maintenance concern, not a current defect. [Finding 6](#r2-f6) below identifies an **adjacent** defect introduced by the same `try_parse` switch — the help/version path now exits 64 — but that defect does not invalidate this finding's resolution; the resolution stands.

---

**Finding 2 — `BookmarkStore::save` is not atomic — partial-write violates DESIGN.md's "No partial write" contract (Dim 1, Dim 8)**

<a id="r2-f2"></a>

**Owner:** software-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** quality-engineer

Round 1's [Finding 2](2026-05-20-software-engineer.md#r1-f2) raised the partial-write window in `std::fs::write`. Verifying the Round 2 fix:

1. **Implementation path.** [`src/lib.rs:130-174`](../../src/lib.rs) `BookmarkStore::save` now writes via the standard tmp-sibling + `rename(2)` pattern. The temp path is computed by `temp_sibling_path` at [`src/lib.rs:219-232`](../../src/lib.rs) (pid + nanos suffix, same directory as `path` to keep the rename intra-filesystem). The temp file is created with `OpenOptions::new().create_new(true)` at [`src/lib.rs:240`](../../src/lib.rs) so a pre-existing temp file is detected. `f.write_all` + `f.sync_all` write and flush before the rename. On rename failure, best-effort cleanup of the temp file at [`src/lib.rs:163-164`](../../src/lib.rs).

2. **Test coverage.** [`tests/bookmarks.rs:209-253`](../../tests/bookmarks.rs) `save_is_atomic_on_write_failure` pre-stages a known-good store, makes the parent directory read-only (`0o500`), invokes `bm add` (which must fail at temp-file creation because the parent is unwriteable), then verifies the prior file content is byte-identical via `assert_eq!(after, before, ...)`. The test asserts both the failure exit code (2) and the prior-state preservation — the central contract of the atomic-write fix.

3. **Spec alignment.** [`DESIGN.md`](../../DESIGN.md) line 61 was amended to make the atomic-write contract explicit: "The implementation uses a temporary file in the destination directory + atomic rename per POSIX `rename(2)` semantics. If write or rename fails, the storage file's prior state is preserved."

**Resolution:** The Round 2 fix correctly implements the standard atomic-write pattern. The pattern is verified by an integration test that catches the regression at the binary-invocation level. Cross-domain coordination with [Red Team Review 1 Finding 6](2026-05-20-red-team.md) (symlink-follow rejection) integrated cleanly — the `symlink_metadata` check at [`src/lib.rs:132-139`](../../src/lib.rs) runs before the atomic write path, so the two hardening disciplines compose correctly. [Finding 7](#r2-f7) below identifies a **partial-cleanup** edge case (temp file orphaning on `write_temp_file` error) but that is an adjacent defect, not a regression of this finding's fix.

---

**Finding 4 — `BookmarkStore` and `Bookmark` expose mutable `pub` fields, bypassing the `add` method's invariants (Dim 3, Dim 4, Dim 8)**

<a id="r2-f4"></a>

**Owner:** software-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** quality-engineer

Round 1's [Finding 4](2026-05-20-software-engineer.md#r1-f4) raised the encapsulation gap on `Bookmark::url`, `Bookmark::timestamp`, and `BookmarkStore::bookmarks`. Verifying the Round 2 fix:

1. **Implementation path.** [`src/lib.rs:42-46`](../../src/lib.rs) — `Bookmark` fields `url` and `timestamp` are now private (no `pub` qualifier). [`src/lib.rs:68-72`](../../src/lib.rs) — `BookmarkStore::bookmarks` is now private. Accessor methods added: [`src/lib.rs:48-60`](../../src/lib.rs) — `Bookmark::url(&self) -> &str` and `Bookmark::timestamp(&self) -> DateTime<Utc>` (returns by value; `DateTime<Utc>` is `Copy`). [`src/lib.rs:200-203`](../../src/lib.rs) — `BookmarkStore::bookmarks(&self) -> &[Bookmark]`.

2. **Invariant enforcement migration.** [`src/lib.rs:186-195`](../../src/lib.rs) `BookmarkStore::add` now returns `Result<()>` and checks `url.is_empty()` at the library boundary. The CLI shell at [`src/main.rs:88-91`](../../src/main.rs) still checks emptiness shell-side (defense in depth + spec-message preservation), and [`src/main.rs:99-105`](../../src/main.rs) handles `store.add` returning Err as the library-invariant branch.

3. **Test continuity.** [`tests/bookmarks.rs:36-66`](../../tests/bookmarks.rs) `tests_add_creates_bookmark` operates on the JSON shape (`parsed["bookmarks"]`), not on the struct — unaffected by the field-encapsulation change. The unit tests at [`src/lib.rs:310-494`](../../src/lib.rs) construct `Bookmark` via positional field syntax inside the `mod tests` block, which works because the test module has crate-private access to the private fields. The `tests/bookmarks.rs` integration tests go through the public API (`store.add`, `store.bookmarks()`) which is the post-fix shape.

4. **Serde round-trip.** Despite the fields being private, `#[derive(Serialize, Deserialize)]` still emits / accepts the JSON keys `url`, `timestamp`, `bookmarks` — serde derive does not require fields to be `pub`. The on-disk JSON shape at [`DESIGN.md` § Storage format](../../DESIGN.md) lines 105-114 is preserved.

**Resolution:** The Round 2 fix correctly encapsulates the invariants at the type boundary. The proposed-fix shape from the Round 1 finding body is faithfully landed; serde compatibility is preserved by serde's field-visibility-agnostic derive. The choice to make `Bookmark::timestamp` return by value rather than by reference (`-> DateTime<Utc>` not `-> &DateTime<Utc>`) is correct because `DateTime<Utc>` is `Copy` — no efficiency penalty, and the by-value return composes more naturally with downstream `.to_rfc3339()` calls.

---

### Hallucinated

*(none — every Round 1 finding above is structurally addressed by an in-tree code change with cited file:line + a corresponding test in [`tests/bookmarks.rs`](../../tests/bookmarks.rs) or [`src/lib.rs`](../../src/lib.rs) `mod tests`. The Round 1 findings classified as Open are now in fact Resolved; no Round 1 finding was invented and the cold pass against the resolutions does not generate a hallucinated reclassification.)*

---

### Resolved

**Finding 3 — Exit code 2 has two distinct meanings: "Storage error" — re-verification of Raised-to-SO disposition (Dim 1, Dim 2)**

<a id="r2-f3"></a>

**Owner:** solution-owner
**Status:** validated
**Blocked by:** *(none)*
**Validator:** solution-architect

Round 1's [Finding 3](2026-05-20-software-engineer.md#r1-f3) was Raised to SO — exit-code reassignment is a spec change, not an SE implementation decision. Verifying the SO's disposition + the SE implementation that follows it:

1. **Spec amendment.** [`DESIGN.md`](../../DESIGN.md) § Interface definitions § Exit codes (lines 94-103) now has four rows: 0 (Success), 1 (User error — empty URL OR no positional), 2 (Storage error), **64 (CLI usage error — `EX_USAGE` per `sysexits.h`)**. The SO took path 1 from the Round 1 finding's two paths — the cleaner-contract path. [`DESIGN.md`](../../DESIGN.md) line 62 spells out the case scope: "Failure (CLI usage error other than missing/empty URL — e.g., unknown subcommand, unknown flag): stderr clap-formatted usage message. Exit 64."

2. **Implementation path.** [`src/main.rs:75-76`](../../src/main.rs) emits `ExitCode::from(64)` for the default branch of `handle_parse_error` (anything that is not the missing-`<URL>` case). Cross-checked against the spec's listed cases: unknown subcommand (e.g., `bm bogus`) → clap returns `ErrorKind::InvalidSubcommand`, falls through to `err.print()` + exit 64. ✓. Unknown flag (e.g., `bm add --unknown-flag URL`) → clap returns `ErrorKind::UnknownArgument`, falls through to exit 64. ✓.

3. **Test coverage.** [`tests/bookmarks.rs:189-201`](../../tests/bookmarks.rs) `bm_unknown_subcommand_exits_64` invokes `bm frobnicate` and asserts exit 64. The unknown-flag case is not separately tested, but the same code path handles both, and the spec lists both — the SE-internal preference would be a second test for the unknown-flag case, surface to QE as a coordination note rather than a SE finding (the implementation correctness is what's in scope here).

**Resolution:** The SO has dispositioned the Raised-to-SO finding by spec amendment (exit 64 added); the SE implementation follows the spec amendment. The contract is now unambiguous. [Finding 6](#r2-f6) below identifies that the same `handle_parse_error` branch incorrectly handles the help/version path — that is a separate adjacent defect, not a regression of this finding's resolution.

---

**Finding 5 — Crate-level Clippy / `missing_docs` deny set — partial implementation; Rust-supplement floor not fully met (Rust supplement § Software Engineering — Clippy lint configuration)**

<a id="r2-f5"></a>

**Owner:** software-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** quality-engineer

Round 1's [Finding 5](2026-05-20-software-engineer.md#r1-f5) raised the missing crate-level lint floor. Verifying the Round 2 fix:

1. **Implementation path.** [`src/lib.rs:26-27`](../../src/lib.rs) — `#![deny(missing_docs, unsafe_code)]` + `#![warn(clippy::all, clippy::pedantic, clippy::nursery)]`. [`src/main.rs:18-19`](../../src/main.rs) — `#![deny(unsafe_code)]` + `#![warn(clippy::all, clippy::pedantic, clippy::nursery)]`. `missing_docs` deliberately not set on the binary per the Rust supplement § Technical Writer note that "internal functions in `main.rs` may be omitted." [`Cargo.toml:62-69`](../../Cargo.toml) — `[lints]` table with `unsafe_code = "deny"`, `missing_docs = "deny"`, `clippy::all = "deny"`, `clippy::pedantic = "warn"`.

2. **Doc-comment coverage.** All `pub` items in [`src/lib.rs`](../../src/lib.rs) carry `///` doc comments: `Bookmark` (line 34-41), `Bookmark::url` (line 49), `Bookmark::timestamp` (line 55), `BookmarkStore` (line 62-67), `BookmarkStore::load` / `save` / `add` / `bookmarks` / `newest_first` (each with `# Errors` sections where applicable), `display_safe` (line 262-277). The crate-level `//!` doc at [`src/lib.rs:1-24`](../../src/lib.rs) covers the module-level documentation per the Rust supplement § Technical Writer "Module-level documentation" check.

3. **Gap against the documented standard.** The Rust supplement § Software Engineering names the **standard deny set** as:
   > `#![deny(clippy::all, clippy::pedantic, clippy::nursery, clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::missing_errors_doc, clippy::missing_panics_doc, missing_docs)]`
   > Any deviation from this baseline requires documented rationale.

   The current configuration deviates as follows: `clippy::pedantic` is `warn`, not `deny` ([`Cargo.toml:68`](../../Cargo.toml)); `clippy::nursery` is at `warn` per the crate attribute but absent from the `[lints]` table; `clippy::unwrap_used`, `clippy::expect_used`, `clippy::panic`, `clippy::missing_errors_doc`, `clippy::missing_panics_doc` are entirely absent. [`Cargo.toml:57-61`](../../Cargo.toml) carries the comment "The deny set tracks the Rust supplement § Software Engineering 'standard deny set' with `pedantic` as warn to surface guidance without blocking." — i.e., a documented rationale for the `pedantic` weakening but not for the other absences. The supplement says "Any deviation from this baseline requires documented rationale"; the absences of `unwrap_used`, `expect_used`, `panic`, `missing_errors_doc`, `missing_panics_doc` lack any rationale comment.

**Resolution:** The Round 2 fix substantially addresses the Round 1 finding — the floor that *was* missing (missing_docs; some clippy deny lints; the `[lints]` table) is now present, and per the cold-reader test of the supplement's primary cited consequences (no `.unwrap()`/`.expect()` discipline at lint time; no `missing_docs` enforcement), `missing_docs` *is* now caught at lint time. The residual gap (the absent supplement-named lints) is a tractable Layer-2 follow-up rather than a regression of this finding — the Round 1 finding's named fix was "a one-line addition" + per-finding triage of new lints, and the Round 2 implementation went further (added the `[lints]` table; added the binary-side configuration; documented the `pedantic` weakening). The remaining absent lints can be added as a Layer-2 hardening pass per the [SE Validator pair](../../../../vsdd-suite/domains/role/SOFTWARE-ENGINEER-REVIEW.md) coordination note — they are not the substance of this finding.

**Validator handoff:** [QE](../QUALITY-ENGINEER-REVIEW.md) should run `cargo clippy --all-targets -- -D warnings` against the current state and confirm zero warnings; that's the cross-domain validation that the lint configuration is internally consistent. If `cargo clippy` surfaces new warnings under `clippy::pedantic` / `clippy::nursery` (currently at `warn`), they become Layer-2 follow-ups.

---

### Deferred

**Finding 6 — `handle_parse_error` returns exit 64 for `bm --help` and `bm --version` — Round-2-introduced regression on the help/version path (Dim 1, Dim 2)**

<a id="r2-f6"></a>

**Owner:** software-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** quality-engineer

The Round 2 fix for [Round 1 Finding 1](2026-05-20-software-engineer.md#r1-f1) + [Finding 3](2026-05-20-software-engineer.md#r1-f3) replaced `Cli::parse()` with `Cli::try_parse()` + the new `handle_parse_error` at [`src/main.rs:57-77`](../../src/main.rs). This switch correctly intercepts `ErrorKind::MissingRequiredArgument` (→ exit 1) and routes other usage errors to exit 64. However, **`try_parse` also returns `Err(clap::Error)` for the `DisplayHelp` and `DisplayVersion` kinds** — these are not errors in the user-error or storage-error sense; they are clap's idiomatic way of signaling "user invoked `--help` / `--version`; print the corresponding text and exit 0."

Source: [clap 4 `ErrorKind` documentation](https://docs.rs/clap/4/clap/error/enum.ErrorKind.html) — `DisplayHelp` and `DisplayVersion` are listed alongside `MissingRequiredArgument`, `InvalidSubcommand`, etc. The `Cli::parse()` wrapper in the Round 1 implementation handled these by calling `std::process::exit(0)` internally (clap's `parse` matches on `ErrorKind` and selects the correct exit code per the variant); the `Cli::try_parse` switch in Round 2 hands the error to `handle_parse_error`, which routes everything-not-MissingRequiredArgument through the `_ = err.print(); ExitCode::from(64)` default branch.

**Concrete consequence:**

- `bm --help` — clap returns `Err(Error { kind: ErrorKind::DisplayHelp, ... })`. `handle_parse_error` calls `err.print()` (which writes the help text to stdout — correct destination per clap's `print()` behavior for DisplayHelp) then exits 64 (incorrect — should be 0).
- `bm --version` — clap returns `Err(Error { kind: ErrorKind::DisplayVersion, ... })`. Same path: version text to stdout, exit 64.

**Why it matters at Layer 1:** [`DESIGN.md` § Interface definitions § Command surface](../../DESIGN.md) lines 87-92 explicitly lists `bm --help` and `bm --version` as supported invocations. The Exit codes table at lines 94-103 assigns exit 0 to "Success (including empty `bm list`)" and exit 64 to "CLI usage error (`EX_USAGE` per `sysexits.h` — unknown subcommand, unknown flag, malformed invocation other than missing/empty URL)". A help-text request is not a usage error — it is a successful information retrieval. A shell script wrapping `bm --version` to detect "is this binary installed and runnable" would see exit 64 and (correctly per the spec's contract) conclude the binary is malformed. A package manager test (`bm --help` as a smoke test) would fail.

**Test surface gap:** the integration test suite at [`tests/bookmarks.rs`](../../tests/bookmarks.rs) has no test for `bm --help` or `bm --version` exit codes. The Round 2 fix's verification tests covered the negative cases (missing arg → exit 1; unknown subcommand → exit 64) but not the positive informational-request cases. The regression is invisible to `cargo test`.

**Defensible fix:** add explicit branches to `handle_parse_error` for the two kinds:

```rust
fn handle_parse_error(err: &clap::Error) -> ExitCode {
    if matches!(err.kind(), ErrorKind::DisplayHelp | ErrorKind::DisplayVersion) {
        let _ = err.print();   // help/version text goes to stdout per clap
        return ExitCode::SUCCESS;
    }
    if err.kind() == ErrorKind::MissingRequiredArgument { /* ...as before... */ }
    let _ = err.print();
    ExitCode::from(64)
}
```

And add `tests/bookmarks.rs` cases asserting `bm --help` and `bm --version` both exit 0 with non-empty stdout.

**Cross-domain coordination:** Surface to [QE](../QUALITY-ENGINEER-REVIEW.md) for the missing-test concern (no `--help` / `--version` exit-code test; QE Dim 1 test-design); the implementation fix itself is SE-owned. Surface to [UX](../UX-REVIEW.md) as a [UX](../../../../vsdd-suite/domains/role/UX-REVIEW.md) — `bm --help` printing the help text then exiting nonzero is a documented-CLI-convention violation that affects discoverability ergonomics.

**Classification:** Open — adjacent defect introduced by the Round 1 fix cycle; the fix is local (two added match arms in `handle_parse_error` + two integration tests). This is exactly the "the fix may have created adjacent defects" pattern the [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) § Round triggers G-131 names as the reason Round N+1 is mandatory after Round N produces real findings.

---

**Finding 7 — `BookmarkStore::save` leaks orphan temp files on `write_temp_file` failure (Dim 8 — defensive coding; partial-resolution of Round 1 Finding 2)**

<a id="r2-f7"></a>

**Owner:** software-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** quality-engineer

The Round 2 atomic-write fix at [`src/lib.rs:130-174`](../../src/lib.rs) correctly addresses the partial-write contract for the rename-failure path: lines 161-171 explicitly call `std::fs::remove_file(&tmp_path)` on rename failure to clean up the temp file. However, the **temp-file-write-failure** path at [`src/lib.rs:152-157`](../../src/lib.rs) does not have the same cleanup:

```rust
let tmp_path = temp_sibling_path(path);
write_temp_file(&tmp_path, json.as_bytes()).with_context(|| {
    format!(
        "writing temp file for atomic save at {}",
        tmp_path.display()
    )
})?;
```

The `?` propagates any error from `write_temp_file` without removing the temp file. Inspecting `write_temp_file` at [`src/lib.rs:234-247`](../../src/lib.rs):

```rust
#[cfg(unix)]
fn write_temp_file(tmp_path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(tmp_path)?;          // (a)
    f.write_all(bytes)?;           // (b)
    f.write_all(b"\n")?;           // (c)
    f.sync_all()?;                 // (d)
    Ok(())
}
```

- Failure at (a) `open` — no temp file created; no orphan. ✓
- Failure at (b), (c), or (d) — the temp file **exists on disk** with whatever bytes (`0..n`) were flushed before the failure, and `?` propagates the error without removing it.

The destination file's prior state is preserved (the rename never happens), so the **PROT_37 "no partial write" contract is honored** — that part of Round 1 [Finding 2](#r2-f2) is correctly resolved. But the temp file leaks: the user sees a stale `bookmarks.json.tmp.<pid>.<nanos>` sibling file after a save failure, which (a) confuses the user inspecting the directory, (b) on subsequent saves under heavy concurrent-error conditions can leak many temp files, and (c) the next save's `temp_sibling_path` produces a *different* name (different nanos) so the orphans never get reused or overwritten.

**Why it matters even at single-user Layer 1:** The reference-implementation purpose ([G-112](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112)) is to teach the suite end-to-end, and the atomic-write fix is part of what it teaches. The current implementation teaches half the pattern — the success path is atomic, the rename-failure path cleans up, but the write-failure path leaks. A future reader copying this pattern into a longer-lived service would inherit the orphan-file accumulation.

**Defensible fix:** wrap the `write_temp_file` call in cleanup, matching the rename-failure-path pattern at [`src/lib.rs:161-171`](../../src/lib.rs):

```rust
if let Err(e) = write_temp_file(&tmp_path, json.as_bytes()) {
    let _ = std::fs::remove_file(&tmp_path);   // best-effort cleanup
    return Err(e).with_context(|| { ... });
}
```

Or — more idiomatically — introduce a `Drop` guard around the temp path that removes the file unless explicitly defused after a successful rename.

**Test surface gap:** the existing `save_is_atomic_on_write_failure` integration test at [`tests/bookmarks.rs:209-253`](../../tests/bookmarks.rs) covers the case where `create_new` itself fails (parent directory is read-only — failure at (a), no temp file ever exists). It does NOT cover (b), (c), or (d) — the case where the temp file is created but the write fails mid-flight. Simulating (b)/(c) failure deterministically in an integration test is difficult (it requires a filesystem-quota cap or a fault-injection layer); the unit test at [`src/lib.rs`](../../src/lib.rs)'s `mod tests` block could simulate it by pre-staging a regular file at the expected temp path (forcing `create_new` to fail — same as (a)), but to exercise (b)/(c) you'd need to either inject a `std::io::Write` fake or use a quota-limited tmpfs. Surface to QE as a fault-injection test-design concern.

**Cross-domain coordination:** Surface to [QE](../QUALITY-ENGINEER-REVIEW.md) for the partial-test-coverage gap; surface to [Red Team](../RED-TEAM-REVIEW.md) as a small adversarial surface (an attacker who can intermittently induce disk-full errors at the right moment can fill the parent directory with orphan temp files — DoS via inode exhaustion in extreme cases; low-severity at Layer 1's single-user threat model, but the surface is real).

**Classification:** Open — partial-resolution gap of Round 1 [Finding 2](2026-05-20-software-engineer.md#r1-f2); the fix is local (3 lines in `save`); the test is harder to land than the fix. Naming it as a separate finding (rather than reopening Finding 2) per the [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) discipline that adjacent defects discovered in Round N+1 are new findings — the original Finding 2 is genuinely Resolved against its named contract ("no partial write" / "prior state preserved"); the cleanup discipline is an adjacent concern that Round 1 did not raise.

---

### Summary

7 findings classified: 5 Round 1 findings re-verified ([Finding 1](#r2-f1), [Finding 2](#r2-f2), [Finding 3](#r2-f3), [Finding 4](#r2-f4), [Finding 5](#r2-f5)) all Resolved (lifecycle terminal `validated`); 2 new Round 2 findings raised ([Finding 6](#r2-f6) help/version exit-code regression introduced by the `try_parse` switch; [Finding 7](#r2-f7) temp-file orphaning on partial write failure — partial-resolution edge case of the Round 1 [Finding 2](#r2-f2) fix).

The Round 2 fix cycle achieved its primary objective — every Round 1 SE finding lands a structurally correct in-tree fix with cited test coverage. The two new findings are **adjacent defects** of the kind the [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) § Round triggers G-131 explicitly names as the reason Round N+1 cold passes are mandatory: "[Round N+1] verifies the fix held and looks for adjacent defects the fix may have created." Both new findings were created by Round-2-fix design choices ([Finding 6](#r2-f6) by the `try_parse` switch; [Finding 7](#r2-f7) by the tmp-file pattern's incomplete cleanup) — exactly the pattern the round-trigger guidance anticipates.

**MVR signal:** **substantive findings raised** ([Finding 6](#r2-f6) and [Finding 7](#r2-f7) are both non-Hallucinated, real adjacent defects with cited file:line + named consequence + named defensible fix). Per [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) § Round triggers G-131 continue trigger: **Round 3 is mandatory** for SE on bookmark-cli-manual Layer 1 once Findings 6 and 7 are resolved. The Round 3 pass should verify (a) `bm --help` / `bm --version` exit 0; (b) integration test coverage for the help/version path; (c) temp-file cleanup on `write_temp_file` partial-failure paths; (d) any further adjacent defects introduced by the Finding 6/7 fixes.

**Coordination:**

- [Finding 6](#r2-f6) — Surface to [QE](../QUALITY-ENGINEER-REVIEW.md) for the missing help/version test coverage (test-design gap); surface to [UX](../UX-REVIEW.md) for the CLI-convention violation (help-text invocation should exit 0).
- [Finding 7](#r2-f7) — Surface to [QE](../QUALITY-ENGINEER-REVIEW.md) for fault-injection test-design (how to cover the (b)/(c)/(d) write-mid-flight failure modes); surface to [Red Team](../RED-TEAM-REVIEW.md) for the orphan-temp-file inode-exhaustion adversarial surface (low-severity at single-user Layer 1 but real).
- [Finding 5](#r2-f5) residual-deny-set gap — Surface to [Platform Engineer](../PLATFORM-ENGINEER-REVIEW.md) for the CI side (`cargo clippy -- -D warnings` enforcement against the full supplement-standard deny set) and to [Solution Architect](../SOLUTION-ARCHITECT-REVIEW.md) for the single-source-of-truth tension between [`src/lib.rs`](../../src/lib.rs) `#![deny/warn]` attributes and [`Cargo.toml`](../../Cargo.toml) `[lints]` table (two sources of lint configuration; potential drift).
- **Non-SE-owned observation surfaced for [TW](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md):** Round 1's Coordination section flagged the `PROT_37` / `PROT_41` / `PROT_30` / `PROT_40` / `PROT_46` placeholder-looking tokens in [`DESIGN.md`](../../DESIGN.md) and [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md). The current [`DESIGN.md`](../../DESIGN.md) no longer uses `PROT_37` / `PROT_41` as section identifiers (the behavioral-contracts section is now under descriptive headings `### `bm add <url>`` and `### `bm list``); references to "PROT_37" remain in the prose (line 60-62) as historical-narrative anchors. The migration to descriptive headings is the post-Review-78 [naming and identifier discipline](../../../../vsdd-suite/suite-development/suite-development.md#naming-and-identifier-discipline-review-78-finding-4) outcome. Re-flag to TW only as confirmation that the migration is complete and the residual prose references are intentional historical-narrative anchors per the [G-89](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-89) forward-only discipline.
