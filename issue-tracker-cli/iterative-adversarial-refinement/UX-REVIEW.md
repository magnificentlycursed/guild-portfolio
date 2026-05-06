# UX Review Log

This review is part of the [Iterative Adversarial Refinement (IAR)](README.md) suite. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

**Reviewer role: UX Designer** (UX Designer / UX Researcher / Product Designer)

The purpose of this review is to evaluate the user experience of the interface. For CLI projects, the CLI supplement dimensions (`lang/cli.md`) replace the standard browser-centric UX dimensions.

**Language supplement applied:** `lang/cli.md` (UX replacement dimensions). Standard browser UX dimensions are not applicable.

**Sycophancy check:** An AI agent cannot experience a user interface — it cannot perceive latency, notice visual imbalance, or discover that a flow is confusing by trying to use it. An agent reviewing its own UI implementation will validate the decisions it made at generation time rather than evaluate the lived experience those decisions create. The adversary must flag any dimension where the review relies on reading code rather than observing the interface. If the project cannot be tested directly in a browser, state that explicitly — do not simulate user experience from source code and report it as a UX evaluation.

---

## Review 1 — 2026-04-27 21:00Z

**Scope:** `DESIGN.md` Interface section and all command specifications. No binary exists.

**Session note:** In-session with all other domain reviews. Acknowledged quality tradeoff.

---

### Dismissed

**Finding 1 — Empty-state messages route to stdout; may pollute piped output (CLI supplement dim 6)**

CLI supplement dim 6: "Is the empty message on `stderr` so it does not pollute piped output?"

DESIGN.md specifies that `"No open issues. Nice work!"` and `"No issues match the given filters."` print to stdout on success. If the user pipes `tracker list | wc -l`, the empty-state message will be counted. CLI convention (e.g., `grep`, `find`) emits nothing on stdout for zero-result success.

**Classification:** Dismissed. DESIGN.md's stdout contract explicitly routes all success output to stdout. The tool is designed for interactive use — the assignment's Layer 7 specifically names "No open issues. Nice work!" as a user-facing polish feature. Routing it to stderr when empty would mean `tracker list` produces silent output in the common zero-results case, which is a worse interactive experience. The SO review confirmed that no scripted caller is in scope (no structured exit codes; no `--json` flag). The piping concern is real but the trade-off is correctly weighted toward the interactive use case. Accepted design choice.

---

**Finding 2 — `tracker delete` has no confirmation gate (CLI supplement dim 7)**

CLI supplement dim 7: "Do commands that delete, overwrite, or irreversibly modify data require explicit confirmation?"

`tracker delete <id>` immediately removes the issue with no confirmation prompt or `--force` flag. Deletion is permanent and the ID is never reused.

**Classification:** Dismissed with cross-reference. SO Review 6 dismissed this finding with documented rationale: the assignment's authoritative interface section lists `tracker delete <id>` with no confirmation signal; the build-layer guidance ("with confirmation") is explicitly advisory. The tool is non-interactive by design (Out of Scope). The UX concern is real — a typo in the ID is irrecoverable — but the design choice is documented and has SO approval. The `tracker show <id>` workflow before `tracker delete <id>` is the user-side mitigation.

---

**Finding 3 — `→` Unicode arrow in status confirmation (CLI supplement dim 3 — output scannability)**

`Issue #<id> status → <new_status>.` uses the right arrow character U+2192. On non-UTF-8 terminals or systems with legacy encoding, this character may render incorrectly. On the target platform (macOS), this is not an issue.

**Classification:** Dismissed. Target platform (macOS, modern terminal) fully supports UTF-8. The character is a deliberate design choice in the spec. Cross-referenced in SE log.

---

**Finding 4 — No `--label` flag asymmetry documented as user-facing behavior (CLI supplement dim 2)**

Multiple `--label` flags are accepted on `tracker create` (deduplicated) but rejected on `tracker list` (usage error). This asymmetry might surprise users. The `--help` output must make this distinction clear.

**Classification:** Dismissed. The distinction is well-motivated: create accepts multiple labels because each is an addition to the list; list's `--label` filter is a single-value exact match. The spec requires that `--help` accurately describe all flags and their behavior (DESIGN.md Interface section). If the help text is clear, the asymmetry is a feature, not a defect. This will be verified at Layer 7's `--help` acceptance criteria.

---

### Hallucinated

**Finding 5 — No machine-readable output mode (CLI supplement dim 10)**

CLI supplement dim 10: "If the output is intended to be piped or parsed by other programs, is a `--json` flag available?"

**Classification:** Hallucinated. DESIGN.md Out of Scope explicitly excludes scripted callers. The tool is interactive. No piping or programmatic use case is identified. The absence of `--json` is correct for this scope.

---

### Open

*(none)*

---

### Summary

No blocking findings. Four findings dismissed and one hallucinated. The spec's interface design is clean for a CLI-first interactive tool: consistent flag naming, explicit stdout/stderr contract, tabular output with truncation, complete error message formats. The two design tradeoffs (no delete confirmation, empty state on stdout) are documented with rationale.

**Items deferred to Layer 7 manual verification at the gate** (cannot be evaluated from the spec alone — adversary cannot run the binary):
- **CLI dim 1 (help accuracy):** Does `--help` accurately describe all flags, valid values, and include a usage example? Is it complete for all five subcommands?
- **CLI dim 3 (output scannability):** Does the table render clearly with correct column alignment? Is the fixed-width format readable in practice?
- **CLI dim 8 (error message quality):** Do error messages include what failed, why (where knowable), and what to do? Test all error paths from the manual checklist.
- **CLI dim 9 (interruption):** Does `Ctrl+C` during a write leave `tracker.json` in a partially-written state? Manual test — the spec does not address SIGINT handling.

**Coordination:** Finding 2 cross-referenced with [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) Review 6 (no delete confirmation — SO-approved design choice). Finding 3 cross-referenced with [SOFTWARE-ENGINEER-REVIEW.md](SOFTWARE-ENGINEER-REVIEW.md) Review 1 Finding 4.

---

---

## Review 2 — 2026-04-28 05:30Z

**Scope:** Layer 1 implementation — `src/main.rs`, `src/lib.rs`. Evaluating actual CLI behavior as delivered: subcommand structure, help output, stdout/stderr routing, empty state, error message format, and list output readability.

**Session note:** In-session with Layer 1 IAR suite. Acknowledged quality tradeoff. Binary behavior is inferred from test output and code; direct binary execution is the developer's responsibility per the manual testing checklist.

---

### Dismissed

**Finding 1 — `tracker --help` output is clap-generated and has not been reviewed for accuracy (CLI Dim 1)**

clap generates `--help` output from the struct/variant doc comments in `main.rs`. The current doc comments are:
- Binary: `about = "Personal issue tracker"`
- `Create` variant: `/// Create a new issue`
- `List` variant: `/// List open issues`
- `title` field: `/// Issue title`

These are accurate for Layer 1 scope. Layer 7 will add flags (`--priority`, `--label`, `--description`, `--status`) with their valid values documented in the derive attributes, and the help text will be verified against the Layer 7 acceptance criteria.

**Classification:** Dismissed. Accurate for Layer 1 scope. Layer 7 acceptance criteria include help text review for all subcommands and flags.

---

**Finding 2 — Error message format: `Error: <message>` on stderr, nothing on stdout (CLI Dim 8)**

Verified from `tests/layer1.rs`: all error tests assert `.stderr(contains("Error: ...")).stdout("")`. The format is consistent with the spec. `main.rs` routes errors through `eprintln!("Error: {}", e)`.

**Classification:** Dismissed. Error message routing is correct and consistent.

---

**Finding 3 — List output column alignment (CLI Dim 3 — Output scannability)**

The list format uses `{:<4} {:<11} {:<8} {:<20}` left-aligned padding. The test `list_shows_header_and_issues` verifies all column names are present but does not verify exact alignment. Visual alignment is a manual testing concern.

**Classification:** Dismissed. Alignment is structurally correct by the fixed-width format strings. Manual verification (TODO.md Layer 1 checklist item: "Run `tracker list` → verify table shows both issues with correct header") is the appropriate gate for visual quality. No automated test can substitute for the developer reading the output.

---

**Finding 4 — Empty state "No open issues. Nice work!" routes to stdout (CLI Dim 6)**

Review 1 Finding 1 dismissed this; confirming the implementation matches the spec (`println!("No open issues. Nice work!")`) and the test `list_with_no_json_shows_empty_state` asserts it on stdout. No regression.

**Classification:** Dismissed. Behavior matches spec and prior dismissal.

---

### Hallucinated

*(none)*

---

### Open

*(none — items deferred to Layer 7 manual testing remain deferred from Review 1)*

---

### Summary

Four dismissed findings. No new UX concerns from the implementation. The Layer 1 implementation correctly routes success to stdout, errors to stderr, and error messages follow the `Error: <message>` format. Layer 7 deferred items (help accuracy for all flags, color rendering in TTY, piped-output ANSI suppression) remain open. Manual testing checklist must be completed by the developer before the Layer 1 gate closes.

**Coordination:** *(none)*

---

---

## Review 3 — 2026-04-30 00:00Z

**Scope:** Layer 1 gate closure pass — no code changes since Review 2. Manual testing now complete.

**Session note:** In-session with all other domain reviews. Acknowledged quality tradeoff.

---

### Dismissed

*(none)*

### Hallucinated

*(none)*

---

### Open

*(none)*

---

### Summary

No UX findings. Manual testing confirmed expected UX behavior: empty state message correct, table header and columns align, error messages specific and actionable. Layer 7 deferred items remain deferred. MVR reached for Layer 1.

**Coordination:** *(none)*

---

---

## Review 4 — 2026-05-01 00:00Z

**Scope:** Layer 2 implementation — CLI UX evaluation of `tracker status` and `--status` filter. Evaluating error message quality, output format, help text, and discoverability.

**Session note:** In-session with full Layer 2 IAR suite. Acknowledged quality tradeoff. CLI supplement applied.

---

### Dismissed

**Finding 1 — Error messages are specific and actionable (CLI Dim 8)**

- Invalid ID: `Error: 'abc' is not a valid issue ID. Expected a positive integer.` — names the bad value and the expected format. ✓
- Not found: `Error: Issue #99 not found.` — names the missing ID. ✓
- Invalid status value: `Error: Invalid status 'flying'. Expected: open, in-progress, or done.` — names the bad value and lists valid alternatives. ✓

**Classification:** Dismissed. All error messages are specific, actionable, and follow the `Error: <message>` format on stderr.

---

**Finding 2 — `tracker status --help` doc comments (CLI Dim 1)**

`main.rs` `Status` variant docs:
```rust
/// Change an issue's status
Status {
    /// Issue ID
    id: String,
    /// New status: open, in-progress, done
    status: String,
}
```

Accurate for Layer 2 scope. Help text names the valid status values inline. ✓

**Classification:** Dismissed. Layer 7 will verify the complete help text for all subcommands with all flags in place.

---

**Finding 3 — `tracker list --status` help text (CLI Dim 1)**

`List` variant:
```rust
/// Filter by status: open, in-progress, done
#[arg(long)]
status: Option<String>,
```

Accurate. The subcommand's top-level doc ("List issues (default: open only)") correctly documents the default behavior.

**Classification:** Dismissed. Accurate for Layer 2 scope.

---

**Finding 4 — `tracker status` positional argument ordering (CLI Dim 2)**

`tracker status <id> <status>` uses two positional arguments. Passing them in the wrong order (e.g., `tracker status done 1`) produces either a parse error (if `done` is not a valid u64) or a not-found error (if interpreted as ID). The error messages for these cases are not confused — `parse_id("done")` produces an ID error, not a confusing status error. The argument order matches the command name: "set this issue's status to this value."

**Classification:** Dismissed. The positional order is natural and consistent with the spec's interface definition. Accidental transposition produces a clear error.

---

### Hallucinated

*(none)*

---

### Open

*(none — Layer 7 manual TTY verification items remain deferred)*

---

### Summary

No UX findings. Layer 2 error messages are specific and actionable. Help text is accurate for current scope. Deferred Layer 7 items (color output, full help accuracy for all flags) remain deferred. MVR reached for Layer 2.

**Coordination:** *(none)*

---

---

## Review 5 — 2026-05-04 22:00Z

**Scope:** Layer 3 implementation — actual binary execution against the spec. CLI supplement (replacement) dimensions 1–11 applied. Build verified (`cargo build` clean) and binary exercised in a scratch directory under `/tmp` with a fresh `tracker.json` for each scenario. Reviewer ran every subcommand, every error path that exists at this layer, and pipe / redirect scenarios.

**Session note:** Cold session per primer; parallel batch run with other domains.

---

### Open

**Finding 1 — Writing to a closed pipe panics with backtrace and exit 101 (CLI Dim 4 — stdout/stderr discipline; CLI Dim 5 — Exit codes)**

Reproduction: with ≥ a few open issues in storage, run `tracker list 2>&1 | head -2`. Observed:

```
ID    Status       Priority  Labels                Title
2     open         medium    (none)                Add feature
thread 'main' (16977353) panicked at .../io/stdio.rs:1165:9:
failed printing to stdout: Broken pipe (os error 32)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Verified via `bash -c '... | head -2; echo ${PIPESTATUS[0]}'` → tracker exit code = **101** (Rust panic), not 0 or 1. Three independent UX problems collide here: (a) the `Error:` stderr contract is violated — the user sees an internal panic message and a backtrace hint instead of `Error: ...`; (b) the spec says exit 0 on success and 1 on failure (DESIGN.md Interface > Exit codes), but a SIGPIPE produces 101; (c) the contract leaks Rust internals (`thread 'main'`, source path, `RUST_BACKTRACE`) to a user who simply piped output to `head`, `less`, or `grep -m1`. This is the most common interactive shell pattern for browsing a long list and the tool fails it loudly.

Root cause: `main.rs` uses `println!` everywhere; the Rust standard library converts `EPIPE` into a panic in the default print macros. Fix is conventional (install a SIGPIPE handler with `signal(SIGPIPE, SIG_DFL)`, or write through a `Write` trait and treat `ErrorKind::BrokenPipe` as silent exit 0).

**Classification:** Open. Real defect against CLI Dims 4 and 5 and against DESIGN.md's exit-code contract. Spec does not need to change — exit codes 0/1 already cover the intended behavior; the implementation is producing a third undocumented code via panic. Recommend Layer 4 fix.

---

**Finding 2 — Title containing a literal newline corrupts the list table (CLI Dim 3 — output scannability)**

Reproduction: `tracker create $'Title with\nactual newline'` → succeeds, exit 0. Subsequent `tracker list` renders:

```
4     open         medium    (none)                Title with
actual newline
```

The "one issue per line" tabular contract (DESIGN.md `**List output format:**`) is silently broken: the issue spans two lines, the second line has no leading columns, and any downstream tool that splits on `\n` sees a phantom row. Tab characters in a title produce the same class of corruption (`\t` fed through `{:<50}` defeats the column alignment).

DESIGN.md Edge Cases > Title says "the binary receives the raw string after shell expansion and treats it as opaque text" — true at the storage layer, but the output formatter has no defense for control characters. Either the formatter should escape/replace control characters when rendering rows (typical: render `\n` as literal `\n` or `␤`, drop ANSI escapes, etc.), or `validate_title` should reject newlines/tabs at creation time. The spec is silent on which.

**Classification:** Raised to SO. The user-visible behavior — a list view that can be silently broken by a title that the spec accepts — is a real UX defect, but the resolution touches the spec (validation rules vs. display sanitization) and DESIGN.md is the SO's authority. Proposed change: add to DESIGN.md Edge Cases > Title either (a) "Titles containing ASCII control characters (newline, tab, carriage return) are rejected at creation: `Error: Title cannot contain control characters.`", or (b) "The list formatter renders control characters as escape sequences (`\n`, `\t`) so each issue remains on a single line."

---

**Finding 3 — ANSI escape sequences in titles are echoed verbatim into terminal output (CLI Dim 3 — output scannability; cross-ref Security)**

Reproduction: `tracker create $'\x1b[31mRed\x1b[0m'` → stored and round-tripped to `tracker list`. The terminal interprets the escapes; the title appears in red and the surrounding columns inherit the SGR state. A title with a longer escape sequence (e.g. cursor manipulation, screen clear, hyperlink OSC 8) can move the cursor, hide subsequent rows, or overwrite the header.

Output sanitization for terminal escapes is a well-known CLI defense (`git log --format`, `gh issue list`, `ls --color=auto`, etc., all sanitize or use `LC_CTYPE`-aware escape handling). The spec is silent on this.

**Classification:** Raised to SO with Security cross-ref. Coordinate with [SECURITY-REVIEW.md](SECURITY-REVIEW.md) — a single-user local tool has limited threat surface, but the same write-then-display path exists if a user ever copy-pastes a title from external content (a GitHub issue title, a chat message). Proposed DESIGN.md addition: "List and (Layer 4+) show output escape or strip ASCII control sequences and ANSI escape sequences from stored values before printing to a terminal."

---

**Finding 4 — Empty-state message routes to stdout even when stdout is piped, while the prior dismissal rationale rested on stdout being a TTY (CLI Dim 6)**

Verified: `tracker list 2>/dev/null > /dev/null; echo $?` → exit 0 with the message swallowed; but `tracker list | wc -l` returns `1` for a tracker with no open issues — the empty-state line is counted. Review 1 Finding 1 dismissed this with the rationale "no scripted caller is in scope," but the spec ships a list command whose output is exactly the kind of thing a user pipes into `wc`, `grep -c`, `awk`, or `xargs` even in interactive use. The dismissal also did not anticipate Finding 1 above (panic on broken pipe) — together, the two make `tracker list | head` actively hostile.

The previous reasoning is internally consistent for the interactive-only stance, but the consequence is concrete: `tracker list | wc -l` is wrong by exactly the empty-state line in the empty case. Re-raising for explicit re-confirmation now that a binary exists to demonstrate the behavior.

**Classification:** Raised to SO. Defer to SO whether the prior dismissal still holds given live behavior. Proposed minimal fix that preserves interactive UX: detect `IsTerminal` on stdout and route the empty-state message to stderr only when stdout is not a TTY (parallel to the Layer 7 color suppression rule already in DESIGN.md).

---

### Dismissed

**Finding 5 — `--help` text omits a usage example for the binary (CLI Dim 1)**

Verified: `tracker --help` lists subcommands but contains no inline usage example (e.g., a sample `tracker create "Fix bug" --priority high`). CLI Dim 1 explicitly asks "Does the top-level help include a usage example?" Modern clap convention is to add `#[command(after_help = "...")]` with one or two example invocations.

**Classification:** Dismissed. DESIGN.md does not require a usage example in `--help` output — only that `--help` "accurately describe all flags and their valid values," which is satisfied. Adding examples is a quality-of-life improvement and may be worth a backlog ticket but is not a defect against the contract.

---

**Finding 6 — `--version` is rejected with `Error: unexpected argument '--version' found` (CLI Dim 1)**

Verified: `tracker --version` exits 1 with a clap "unexpected argument" error. Most CLI tools expose `--version`. The spec does not require it.

**Classification:** Dismissed. Out of contract — DESIGN.md does not mention `--version`. The current behavior (reject with the project's `Error:`-prefixed format and exit 1) is consistent with the unknown-flag contract. No spec violation.

---

**Finding 7 — No short-flag aliases (`-p`, `-s`) for `--priority` / `--status` (CLI Dim 2)**

Verified: `tracker create "Test" -p high` produces `Error: unexpected argument '-p' found`. Short flags are common but the spec only specifies long forms.

**Classification:** Dismissed. The spec lists only long forms in the subcommand table; long-only is consistent and unambiguous. A future "ergonomics" backlog item, not a current defect.

---

**Finding 8 — Negative IDs hit a clap "unexpected argument" error rather than the spec's "not a valid issue ID" message (CLI Dim 8 — error message quality)**

Verified: `tracker status -1 done` → `Error: unexpected argument '-1' found / tip: to pass '-1' as a value, use '-- -1'`. DESIGN.md Edge Cases > IDs explicitly anticipates this: "Negative number (`tracker delete -1`) → the CLI parser treats `-1` as a flag and produces a usage error; the command exits 1." Behavior matches the spec.

**Classification:** Dismissed. Spec-anticipated. The clap "tip" line ("to pass '-1' as a value, use '-- -1'`") is actually helpful in the rare case the user really meant a literal dash-prefixed value.

---

### Hallucinated

**Finding 9 — Title in tabular row should be quoted to disambiguate trailing whitespace (CLI Dim 3)**

Initial concern: a title like `"Fix bug   "` (trailing spaces) would be visually indistinguishable in the rightmost column from one without trailing spaces.

**Classification:** Hallucinated. `validate_title` trims at creation; trailing whitespace in stored titles is impossible by the spec invariant. The code path that would produce this defect is unreachable.

---

### Summary

Round 5 finds 4 open / Raised-to-SO findings, 4 dismissed, 1 hallucinated. The signal moved up sharply versus prior rounds because Round 5 is the first review to actually execute the binary instead of inferring behavior from tests and source.

The most consequential finding is **#1 (SIGPIPE → panic with exit 101)**: violates DESIGN.md's `Error:` stderr contract, the documented exit-code set {0,1}, and the most common interactive shell pattern (`| head`, `| less`). It is a pure implementation fix — no spec change.

Findings 2 and 3 (newlines, ANSI escapes in titles) expose a contract gap in DESIGN.md: the storage layer treats titles as opaque, but the display layer has no corresponding guidance. The SO needs to choose between input validation and output sanitization.

Finding 4 re-raises an earlier dismissal now that it can be observed concretely; the SO may re-dismiss with the same rationale, but the dismissal should be re-affirmed against live behavior, not against the spec alone.

**Coordination:**
- **Finding 1** → cross-reference [QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md): no test in `tests/layer1.rs|layer2.rs|layer3.rs` exercises a piped-stdout scenario; an integration test that pipes `tracker list | head -1` would have caught this. Suggested QE addition: "broken pipe / SIGPIPE behavior" to the test plan.
- **Finding 1** → cross-reference [SOFTWARE-ENGINEER-REVIEW.md](SOFTWARE-ENGINEER-REVIEW.md): `println!` is used directly in command handlers (`lib.rs`); routing all output through a `Write` handle (passed in by `main.rs`) would localize SIGPIPE handling and incidentally make output testable without subprocess invocation.
- **Finding 3** → cross-reference [SECURITY-REVIEW.md](SECURITY-REVIEW.md): terminal escape injection is a well-known CLI security primitive even in single-user tools.
- **Findings 2, 3, 4** → all Raised to SO; cross-reference [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) for spec adjudication.

---

### Update — 2026-05-04 16:00Z: Layer 3 follow-up resolution pass

- **F1 (SIGPIPE panic on `tracker list | head`) → Resolved.** `src/main.rs` now restores the default SIGPIPE handler at process start (`#[cfg(unix)] libc::signal(libc::SIGPIPE, libc::SIG_DFL)`). `tracker list` piped to a reader that closes early now exits cleanly via signal termination instead of panicking with a backtrace and exit 101. Regression locked by `tests/layer1.rs:list_does_not_panic_on_broken_pipe` (cfg(unix); ~600-row stored data, reader-end dropped before writer finishes; asserts no `panicked` on stderr and exit code != 101). Closes the corresponding Security Review 6 Finding 1 in lockstep.
- **F2 (newlines in titles), F3 (ANSI/control-sequence injection), F4 (empty-state on stdout pollutes pipes) → still Raised to SO.** Spec-level decisions; no implementation change in this round. SO has not adjudicated yet.

---

### Update — 2026-05-05 11:00Z: SO Review 13 spec adjudication

All three Raised-to-SO findings closed by SO Review 13 (`iterative-adversarial-refinement/SOLUTION-OWNER-REVIEW.md`).

- **F2 (newline characters break list contract) → Resolved by SO Review 13 Finding 1.** A single rule — "reject any character with `is_control()` at `validate_title`" — closes both F2 and F3 at the validation boundary. The same check applies at storage load (`issue_fields_are_valid`) so a hand-edited `tracker.json` with a control-character title triggers the corrupt-data path. Regression locked by `tests/layer1.rs:create_title_with_newline_exits_one` and unit test `title_with_newline_is_rejected`.
- **F3 (ANSI/control-sequence injection) → Resolved by SO Review 13 Finding 1.** Same rule. ESC (`0x1B`) is a control character; `\u{1B}[2J` and OSC 8 sequences are rejected. Regression locked by `tests/layer1.rs:create_title_with_ansi_escape_exits_one` and unit tests `title_with_escape_sequence_is_rejected`, `title_with_nul_or_del_is_rejected`. Hostile content can never be re-emitted because it cannot be stored.
- **F4 (empty-state messages on stdout pollute pipes) → Resolved by SO Review 13 Finding 2.** Both empty-state messages (`No open issues. Nice work!` and `No issues match the given filters.`) now route to stderr; stdout is empty when no records match. `tracker list | wc -l` now returns `0` in the empty case, not `1`. Regression locked by `tests/layer1.rs:list_with_no_json_shows_empty_state_on_stderr`, `tests/layer2.rs:list_all_done_default_shows_empty_state` + `list_nonempty_status_filter_with_no_match_shows_filter_message`, and `tests/layer3.rs:list_priority_filter_no_match_shows_filter_message`.

The companion fact (printable Unicode including emoji and CJK is still accepted): regression-locked by `tests/layer1.rs:create_title_with_printable_unicode_succeeds` and unit test `title_with_printable_unicode_is_accepted` — guards against a future over-tightening of the rule that might accidentally ban legitimate non-ASCII content.

**No new UX findings this round.** Suite: 74 → 84 tests; `cargo test --all-targets --locked`, `cargo clippy --all-targets --locked -- -D warnings`, `cargo fmt --check` all clean.

---

---

## Review 6 — 2026-05-05 21:50Z

**Scope:** Layer 4 (`issue-tracker-cli-labels` branch) — `--label` on `create` and `list`. CLI supplement (replacement) dimensions 1–11 applied. `cargo build --release` ran clean. Binary exercised in `/tmp/ux-review-l4` from a fresh state per scenario; stdout and stderr separated with `1>/dev/null` / `2>/dev/null`; exit codes captured. Regression spot-check on Review 5 SIGPIPE / empty-state-routing fixes preserved through Layer 4.

**Session note:** Cold session per primer. Parallel batch with other domain reviews; quality tradeoff acknowledged.

**Cross-domain input considered:** SO Review 16 Open finding on `tracker list --label ""` / `--label "  "` exiting 0 with the "no match" message while the create-side rejects the same input as `Error: Label cannot be empty.`.

---

### Open

**Finding 1 — `tracker list --label "<padded>"` does not trim, but `tracker create --label "<padded>"` does, so the same string round-trips into a "no match" (CLI Dim 8 — error message quality; CLI Dim 4 — stdout/stderr discipline)**

Reproduction (fresh `/tmp/ux-review-l4`):

```
$ tracker create "X" --label "  spaced  "
Created issue #1: X

$ cat tracker.json | jq '.[0].labels'
[ "spaced" ]                                 # CREATE TRIMMED to "spaced"

$ tracker list --label "  spaced  "
No issues match the given filters.           # FILTER NOT TRIMMED → silent "no match"
exit=0

$ tracker list --label "spaced"
ID    Status       Priority  Labels                Title
1     open         medium    spaced                X
```

This is a strictly stronger version of the SO Review 16 asymmetry. It is not just an aesthetic mismatch between create-rejection and list-silence: the user typed *the same literal string* for both invocations, and the second one — with stored data that obviously matches the user's intent — silently returned zero rows with exit 0 and a generic "no match" message that masks the real cause (filter input had whitespace the storage path quietly stripped). A user investigating "why didn't my label match?" gets no signal pointing at the trimming asymmetry; they see a successful command with no results.

The library (`src/lib.rs`) already exposes `parse_label`, which is exactly the right validator for filter input too — it trims and rejects empty/whitespace-only — but `cmd_list` calls `label_matches` directly on the raw `--label` value (`src/lib.rs:422`). Two principled fixes exist: (a) apply `parse_label` to the filter as well so `list --label "  bug  "` matches a stored `"bug"` and `list --label ""` produces the same `Error: Label cannot be empty.` as create; or (b) leave equality strict but reject empty/whitespace-only filters explicitly with an error message so the user is told they typed nothing.

**Classification:** Raised to SO. The DESIGN.md spec is silent on whether `--label` filter input is trimmed and on whether empty filter values are an error or a silent no-match. The spec ambiguity is upstream of the implementation, so the SO needs to adjudicate. Proposed DESIGN.md addition under Edge Cases > Labels: "On `tracker list --label <l>`, the `<l>` argument is trimmed before comparison, and an empty or whitespace-only `<l>` produces `Error: Label cannot be empty.` exit 1 — symmetric with `tracker create --label`." Independent assessment of SO Review 16 below confirms the silent-no-match is not acceptable UX.

---

**Finding 2 — Multi-`--label` rejection on `list` uses clap's lowercase generic message, breaking the spec's `Error:` prefix and bypassing the project's user-facing voice (CLI Dim 8 — error message quality)**

Reproduction:

```
$ tracker list --label bug --label auth
Error: the argument '--label <LABEL>' cannot be used multiple times

Usage: tracker list [OPTIONS]

For more information, try '--help'.
exit=1
```

DESIGN.md Feature 2 says: "If `--label` is provided more than once to `list`, a usage error is produced on stderr and the command exits 1." The exit code is correct, but the message is clap's stock string, not a project-style message. Compare the project's hand-authored phrasing for the same class of mistake — `Error: Invalid status 'closed'. Expected: open, in-progress, or done.` — which (a) is one line, (b) names the offending argument, (c) tells the user what the valid alternative is. The list-multi-label path leaks clap's three-line block (including a `Usage: ...` reprint and `For more information, try '--help'.`), and it does not name *which* label values were rejected. A user with `--label bug --label auth` gets no hint that "list takes only one label" — they have to infer that from "cannot be used multiple times."

The repaint to `Error:` (uppercase) is in place from `main.rs:62` (`replacen("error:", "Error:", 1)`), so the prefix is correct, but everything after the prefix is generic. Recommend a list-specific override (e.g., clap's `ArgAction::Set` with a custom error, or a manual count check after `try_parse`) producing something like `Error: --label may be specified only once on 'list'. Multiple labels per filter are not supported.` This is the only remaining error path in the binary that uses clap's stock voice for a spec-anticipated condition.

**Classification:** Raised to SE. DESIGN.md does not prescribe the exact text — the contract says only "usage error on stderr, exit 1" — so this is implementation polish, not a spec change. Symmetry with the project's other error messages is the case for changing it.

---

**Finding 3 — Top-level `tracker --help` still has no usage example for compound flag invocations, despite the project newly committing to that standard (CLI Dim 1 — command discoverability)**

Reproduction:

```
$ tracker --help
Personal issue tracker

Usage: tracker <COMMAND>

Commands:
  create  Create a new issue (with optional priority and labels)
  list    List issues (default: open) with optional status / priority / label filters
  status  Change an issue's status
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

Commit `5b95911` ("IAR Review 35 Finding 4: usage examples in --help for compound CLI flags") added a *suite-wide* prompt rule that decomposing agents must include usage-example acceptance criteria in the polish/help-finalization layer for CLI projects with compound flags. Layer 4 is a CLI project with compound flags (`--label` is repeatable on `create`, `--label` + `--status` + `--priority` AND-combine on `list`). The user-facing help still does not show a single example. `tracker create --help` likewise has no `EXAMPLES:` block — a new user reading `--label <LABEL>  Label (repeatable; deduplicated; case-preserved)` cannot tell from the description alone that you say `--label bug --label auth` (two flags) rather than `--label "bug,auth"` or `--label bug auth`.

This was raised in Review 5 Finding 5 and dismissed on the rationale that the spec only requires `--help` to "accurately describe all flags and their valid values." That dismissal pre-dates the suite-level commitment in `5b95911`. The new posture is that compound-flag CLI projects *should* carry usage examples by the polish/help-finalization layer. The polish layer for this project is Layer 7; the spec does not yet name this requirement; raising for SO consideration so the Layer 7 acceptance criteria pick it up.

**Classification:** Raised to SO. Proposed DESIGN.md addition under "**`--help` flag:**": "Subcommands with compound flag interactions (multiple repeatable flags, AND-combined filters) include at least one usage example in their `--help` output." Defers actual implementation to Layer 7 polish.

---

**Finding 4 — `list` rendering of labels containing a comma is ambiguous (CLI Dim 3 — output scannability)**

Reproduction:

```
$ tracker create "X" --label "a,b" --label "c"
Created issue #1: X

$ tracker list
ID    Status       Priority  Labels                Title
1     open         medium    a,b, c                X
```

The Labels column renders `["a,b", "c"]` as `a,b, c` — a human reading the row cannot recover whether this issue has two labels (`a,b`, `c`) or three (`a`, `b`, `c`). DESIGN.md Edge Cases > Labels does not forbid commas in stored labels; the validation is only "non-empty after trim." DESIGN.md "List output format" says "`Labels` renders all labels comma-separated" — which is the source of the ambiguity, since labels are themselves allowed to contain the comma separator.

Two sane resolutions: (a) reject commas in labels at `parse_label` (and at `issue_fields_are_valid` for stored data), narrowing the storage spec to match the display contract; or (b) change the display delimiter to one that cannot appear in a label (e.g., `|` or a Unicode separator), or quote each label with a single character that can't appear inside (e.g., `"a,b" "c"`). Option (a) is the cheaper fix; the comma-in-label use case is a cosmetic edge that no plausible user workflow needs.

**Classification:** Raised to SO. The fix touches a validation rule (option a) or a display-format spec line (option b); both are DESIGN.md territory. Proposed minimum DESIGN.md addition under Edge Cases > Labels: "Labels may not contain `,` (the list-display separator); `--label "a,b"` produces `Error: Label cannot contain a comma.` exit 1."

---

### Dismissed

**Finding 5 — `--label` filter is case-sensitive and silently returns no match for case-mismatched input (CLI Dim 8)**

Verified: `tracker list --label Bug` against a stored `bug` label produces `No issues match the given filters.` on stderr, exit 0. The user has no hint that case is the reason. *However*, DESIGN.md Edge Cases > Labels explicitly anchors this behavior: `--label Bug` does not match an issue with label `bug`. The spec is intentional, the test (`tests/layer4.rs:list_label_filter_is_case_sensitive`) locks it, and changing it would be an SO-driven spec decision, not a defect against the current contract.

**Classification:** Dismissed. Spec-anticipated and test-locked. A "case-sensitive — try `--label bug`" hint would be a UX win, but it requires inspecting stored labels for a near-match and is out of contract. Note for SO: if you ever revisit this, a "did you mean?" hint is a documented CLI ergonomics pattern.

---

**Finding 6 — `tracker list --label bug` shows the labels column truncated mid-label without indicating the row matched the filter (CLI Dim 3)**

Initial concern: when a label list is wider than 20 chars and gets truncated to `…`, a user filtering by `--label foo` sees `bug, auth, perf, …` but cannot tell whether `foo` is in the truncated tail.

**Classification:** Dismissed. The issue *did* match the filter — that's why it appears in the list at all. The user knows from the act of filtering that the row contains the label. Truncation hides which other labels exist alongside the match, but `tracker show <id>` (Layer 5) is the documented escape hatch for full values per DESIGN.md (`show always displays the full, untruncated values`). Not a defect at Layer 4.

---

### Hallucinated

**Finding 7 — `tracker list --label ""` should exit 1 because empty filters are conceptually wrong**

Initial impulse: the empty-string filter is meaningless, so the binary should refuse it.

**Classification:** Hallucinated *as stated*. Whether `--label ""` should be an error vs. a silent-no-match is a real UX question, but framing it as "empty filters are conceptually wrong" begs the question — the SO has not committed to that position. The substantive concern is captured in Finding 1 (asymmetry with create-side rejection plus the trimming round-trip bug). This standalone framing without the round-trip evidence would have been hand-waving.

---

### Summary

Round 6 finds 4 open / Raised findings, 2 dismissed, 1 hallucinated. The Layer 4 surface area (`--label` on create + list, label rendering in the table) is *mostly* clean — error messages are specific where the project authored them, empty-state routing to stderr is preserved, multi-label rejection on list happens at exit 1 — but four edges remain.

The most consequential finding is **#1**: the create-trim / list-no-trim asymmetry is not a curiosity; it makes a literal round-trip of the same user input fail silently with an unhelpful "no match" message. This subsumes and strengthens the SO Review 16 finding (see UX position below) — the trimming behavior gives the bug a concrete failure case beyond the conceptual asymmetry.

Findings #2 (clap-voice multi-label error) and #3 (no usage examples in `--help`) are polish items that the project has effectively committed to fixing by Layer 7. Finding #4 (comma-in-label rendering) is a real defect against scannability that DESIGN.md does not yet anticipate.

**UX position on SO Review 16 silent-no-match asymmetry (independent assessment):** The silent-no-match is *not* acceptable UX, and the asymmetry should be closed in favor of the create-side behavior (reject the empty/whitespace filter with `Error: Label cannot be empty.`, exit 1). Two reasons:

1. The "no match" message is technically true but communicatively misleading: the user provided input that *cannot* match anything in storage by spec invariant, regardless of what's stored. Telling them "no issues match the given filters" implies "your filter is well-formed but happens to match nothing yet" — which suggests creating a matching issue would help. Erroring out tells them "your filter is malformed" — which is the actual situation.

2. Finding #1 above demonstrates the concrete harm: combined with the trimming asymmetry, the silent-no-match path masks a real input-vs-storage mismatch. Even if the SO decides to *not* trim filter input (option (b) in Finding #1), explicitly erroring on empty filters means the user types `--label ""` once, gets `Error: Label cannot be empty.`, and immediately realizes their shell expansion went wrong — instead of seeing an empty filter result and concluding their tracker is empty.

Recommend the SO accept the asymmetry as a genuine defect (not "by design") and adopt the symmetric-rejection rule.

**Coordination:**
- **Finding 1** → cross-reference [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md): spec adjudication required for trimming + empty-filter rule. Independent re-raise of SO Review 16.
- **Finding 1** → cross-reference [QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md): once SO decides, regression test for `list --label "  bug  "` matching a stored `"bug"` (or rejecting the padded filter).
- **Finding 2** → cross-reference [SOFTWARE-ENGINEER-REVIEW.md](SOFTWARE-ENGINEER-REVIEW.md): replace clap's stock multi-occurrence error with a project-voice message for `list --label`.
- **Finding 3** → cross-reference [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md): pick up the `5b95911` suite-level commitment in DESIGN.md's `--help` contract; route to Layer 7 polish.
- **Finding 4** → cross-reference [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md): comma-in-label storage rule vs. display-delimiter change.

---

## Review 7 — 2026-05-06 02:50Z

**Round:** UX Review 7 (Round-2 verification for Layer 4)
**Scope:** Verify Round-1 F1 (trim-asymmetry round-trip + empty-filter silent-no-match) and F4 (comma-in-label rendering ambiguity) are closed. F2 / F3 deferred to Layer 7 polish per SO Review 17.
**Session context:** Warm-verification session. Re-ran the Review 6 reproducers against the release binary at commit `67ef920`.

### Resolved

#### Finding 1 (Round-1) — Trim-asymmetry round-trip + empty filter

SO Review 17 chose Option A (validate symmetric with create). DESIGN.md Feature 2 amended; SE applied `parse_label` to the filter side; QE added the regression tests. Re-verified:

```
$ tracker create "X" --label "  spaced  "
Created issue #1: X
$ tracker list --label "  spaced  "
ID    Status       Priority  Labels                Title
1     open         medium    spaced                X
$ tracker list --label ""
Error: Label cannot be empty.
exit=1
$ tracker list --label "  "
Error: Label cannot be empty.
exit=1
```

The round-trip now holds: a stored value is reachable by the same literal filter string. Empty/whitespace-only filters are rejected with the spec-literal error (symmetric with `tracker create`). **Resolved.**

#### Finding 4 (Round-1) — Comma-in-label rendering ambiguity

SO Review 17 chose Option (a) from F4 (reject commas at parse). DESIGN.md Edge Cases / Labels documents the rule; SE added the comma check to `parse_label` and `label_is_valid`; QE added `create_with_comma_label_exits_one` and `corrupt_data_with_comma_label_is_rejected`. Re-verified:

```
$ tracker create "X" --label "a,b" --label "c"
Error: Label cannot contain a comma.
exit=1
```

The previously-ambiguous `bug, auth` vs. `a,b, c` rendering can no longer arise; the display delimiter is unambiguous because the alphabet of allowed label characters excludes the delimiter itself. **Resolved.**

### Deferred (Layer 7 polish — per SO Review 17)

#### Finding 2 (Round-1) — Clap-voice multi-label error

The clap default message `the argument '--label <LABEL>' cannot be used multiple times` still appears (project-voice rewrite not applied this round). DESIGN.md does not require a specific phrasing — only "usage error on stderr, exit 1" — so this is implementation polish. Deferred to Layer 7.

#### Finding 3 (Round-1) — Top-level `tracker --help` lacks usage examples

Suite-level commitment `5b95911` calls for usage examples by the polish/help-finalization layer. Layer 7 is the polish layer for this project. Deferred to Layer 7. SO Review 17 records the deferral with the named target.

### Summary

Round-1 F1 + F4 → Round-2 closed. F2 + F3 → Layer 7 polish (deferred with named target). The two consequential UX defects (silent no-match for trim-asymmetric input, comma rendering ambiguity) are both resolved. The two polish items are correctly scoped to a future layer rather than being silently dropped.

**Files modified:** Only this log appended.

---
