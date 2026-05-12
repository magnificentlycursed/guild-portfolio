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

## Review 8 — 2026-05-11 01:09Z

**Round:** UX Review 8 (Layer 6 — `--description` + `tracker show` + `tracker delete`)
**Scope:** Commits `4fb5e67` + `c91676a`. CLI supplement (replacement) dimensions 1, 2, 3, 4, 5, 6, 7, 8, 9. Release binary built clean; exercised in `/tmp/ux-review-l6` with scenarios for each new command; `--help` captured for all five subcommands and the binary.
**Session note:** Cold session per primer. Standalone domain run; quality tradeoff acknowledged.

---

### Open

**Finding 1 — `tracker show --help` and `tracker delete --help` are one-line stubs that omit basic Layer 6 facts the user needs (CLI Dim 1 — discoverability; CLI Dim 2 — help text quality)**

Captured (release binary, this commit):

```
$ tracker show --help
Show full details for an issue

Usage: tracker show <ID>

Arguments:
  <ID>  Issue ID

Options:
  -h, --help  Print help
```

```
$ tracker delete --help
Delete an issue (no confirmation; deleted IDs are never reused)

Usage: tracker delete <ID>

Arguments:
  <ID>  Issue ID

Options:
  -h, --help  Print help
```

Compare to the level of detail the project authored for Layer 1–3 commands:

- `create`: top-level doc names every optional flag ("with optional description, priority, and labels"); each flag has a one-line description with its valid values inline (`Priority: low, medium, high (default: medium)`, `Label (repeatable; deduplicated; case-preserved)`, `Free-form description (stored verbatim; not trimmed)`).
- `list`: every filter argument names its valid values inline ("Filter by status: open, in-progress, done").
- `status`: positional argument doc names the valid values (`New status: open, in-progress, done`).

`show` and `delete` are below that bar. Concretely:

1. `show` does not state the value semantics for `<ID>`. The project's existing pattern (`status`) uses `/// Issue ID` as the argument doc — but `status` is paired with an argument that *does* name its valid values inline, so a user reading the two doc strings together gets "positive integer plus a status string" without needing the longer-form spec. `show <ID>` has no such partner doc; the only signal that `<ID>` must be a positive integer comes from running the command with bad input. The error message is clear (`Error: '0' is not a valid issue ID. Expected a positive integer.`), but discovery-via-error is worse UX than discovery-via-help.

2. `delete` has the same `<ID>` gap, *plus* the top-level command doc strings `"Delete an issue (no confirmation; deleted IDs are never reused)"` — which is excellent (it both states the destructive-by-design choice and the ID-reuse rule) — but does not survive being read alongside the rest of the binary. The "no confirmation" hint to the user reading `tracker --help` is the only place in the entire interface that signals the destructive-without-prompt behavior. A user who reads `tracker delete --help` looking for a `--yes` / `-f` flag (because no-confirmation destruction is unusual for software they didn't write) sees the same line they already read and no further help. There is no acknowledgement that the operation is permanent.

3. No usage example for either command. Suite-level commitment `5b95911` only requires examples for compound-flag commands (which `show` / `delete` are not). However: the Layer 4 R6 F3 dismissal-then-deferral pattern was specifically for *compound flag* commands; `show` and `delete` are single-positional and the example value is just a number. Not a violation of the suite commitment; raising for symmetry only.

**Minimum recommended copy** (no clap structure change needed — only doc-comment edits):

```rust
/// Show full details for an issue.
///
/// Renders all fields as a labelled key-value block to stdout. Full title,
/// labels, and description are shown untruncated (unlike `tracker list`).
Show {
    /// Issue ID (positive integer, ≥ 1)
    id: String,
},
/// Delete an issue.
///
/// Removes the issue from storage and prints `Deleted issue #<id>.` to stdout.
/// Destructive without confirmation by design (see DESIGN.md "Approved
/// Deviations"). Deleted IDs are never reused — the next created issue
/// receives `max(remaining_ids) + 1`.
Delete {
    /// Issue ID (positive integer, ≥ 1)
    id: String,
},
```

Both are doc-comment-only edits in `src/main.rs`. No clap argument structure change. Brings `show` / `delete` help text up to the level the project established for `create` / `list` / `status` in Layer 1–4.

**Classification:** Open. Real defect against CLI Dim 1 (discoverability) and Dim 2 (help text quality). The implementation is correct; the help-text effort is uneven across layers. DESIGN.md `--help` flag contract ("must accurately describe all flags and their valid values") is technically satisfied — there are no flags on `show` / `delete` — but the same contract for the `<id>` argument is the thinnest possible implementation. Recommend the doc-comment uplift as part of Layer 6 polish or roll it into Layer 7. Cross-reference SE.

---

### Dismissed

**Finding 2 — Top-level `tracker --help` lists `show` and `delete` correctly with helpful one-line summaries (CLI Dim 1)**

Verified:

```
Commands:
  create  Create a new issue (with optional description, priority, and labels)
  list    List issues (default: open) with optional status / priority / label filters
  status  Change an issue's status
  show    Show full details for an issue
  delete  Delete an issue (no confirmation; deleted IDs are never reused)
  help    Print this message or the help of the given subcommand(s)
```

Both new commands appear in the top-level listing. `create`'s top-line was correctly updated to add `description` to the list of optional flags (was `"Create a new issue (with optional priority and labels)"` at Layer 4; now includes `description`). `delete`'s one-liner is the strongest line in the whole listing — it tells the user two important facts in passing.

**Classification:** Dismissed. Top-level discoverability is correct. The detail-level help (Finding 1) is the gap.

---

**Finding 3 — `show` output exactly matches DESIGN.md example for both single- and multi-line descriptions (CLI Dim 3)**

Reproduction (single-line, no description, no labels — match against DESIGN.md:247-255):

```
$ tracker show 1
ID:          1
Title:       Update README
Status:      open
Priority:    low
Labels:      (none)
Description: (none)
Created:     2026-05-11T01:12:44Z
Updated:     2026-05-11T01:12:44Z
exit=0
```

Reproduction (multi-line description — match against DESIGN.md:260-269):

```
$ tracker show 2
ID:          2
Title:       Fix auth flow
Status:      open
Priority:    high
Labels:      bug
Description: Token refresh fails after 1 hour.
             Reproduces reliably on Safari.
Created:     2026-05-11T01:12:49Z
Updated:     2026-05-11T01:12:49Z
exit=0
```

Label column is right-padded to 13 characters. Continuation-line indent of the multi-line description is exactly 13 spaces. `(none)` placeholders for absent labels / description are present. Trailing newline emitted by `print!` of the block (no double-newline because `format_show_block` already adds the final `\n`). Visual alignment matches the spec example character-for-character. `format_show_block` also normalizes `\r\n` → `\n` so a description hand-edited with Windows line endings still renders without a stray `\r` on the first line — defensive beyond what the spec requires.

**Classification:** Dismissed. The most important user-visible new surface (the show block) is exactly to spec.

---

**Finding 4 — `delete` confirmation text is informative on success and follows the spec literal (CLI Dim 5; CLI Dim 7)**

Reproduction:

```
$ tracker delete 1
Deleted issue #1.
exit=0
```

Names the issue ID, uses past tense (the action is complete), and writes to stdout per the spec's data-output contract. Same family as `Created issue #<id>: <title>.` and `Issue #<id> status → <new_status>.` — the user is told "what just happened" in a single short line. The D1 approved deviation (no confirmation prompt) is recorded in DESIGN.md "Approved Deviations" with rationale; UX position consistent — the `rm` / `git rm` convention does mean a single-user CLI tool should not prompt by default, and the confirmation-on-success text is the right post-action signal.

**Classification:** Dismissed. The success message is informative. The deviation is documented. The user's safety net is the `tracker show <id>` pre-check workflow named in SO Review 6.

---

**Finding 5 — Error messages on the new commands are actionable for the cases where actionability is possible (CLI Dim 8)**

Verified each new error path:

- `tracker create "X" --description ""` → `Error: Description cannot be empty.` (states the rule)
- `tracker create "X" --description "  "` → same (whitespace-only after trim, same message)
- `tracker show 99` → `Error: Issue #99 not found.` (names the missing ID)
- `tracker delete 99` → `Error: Issue #99 not found.` (same; consistent with `tracker status 99 open` from Layer 2)
- `tracker show abc` → `Error: 'abc' is not a valid issue ID. Expected a positive integer.` (names the bad value and the expected format)
- `tracker show 0` → `Error: '0' is not a valid issue ID. Expected a positive integer.` (zero rejected at the parser boundary)

The "not-found" messages do not tell the user "what to do next" because there is no remedial action the binary can suggest — the user knows their own ID space better than the tool does. (A `did you mean #1?` hint based on nearest-ID is an SO-level call about ergonomics scope, not a defect against the contract.) The "invalid ID" messages tell the user exactly what the rule is. The "description cannot be empty" message is parallel to `Title cannot be empty.` and `Label cannot be empty.` — same voice, same level of specificity.

**Classification:** Dismissed. Error messages are at the bar the project established in Layer 1–4. No new actionability gap.

---

**Finding 6 — stdout/stderr discipline preserved on all new code paths (CLI Dim 4 / Dim 7)**

Verified by redirecting stdout and stderr separately on each path:

- `tracker show 1` → block on stdout, stderr empty
- `tracker show 99` → stdout empty, `Error: Issue #99 not found.` on stderr
- `tracker show abc` → stdout empty, `Error: 'abc' is not a valid issue ID. Expected a positive integer.` on stderr
- `tracker delete 1` → `Deleted issue #1.` on stdout, stderr empty
- `tracker delete 99` → stdout empty, `Error: Issue #99 not found.` on stderr
- `tracker create "X" --description ""` → stdout empty, `Error: Description cannot be empty.` on stderr

Matches the DESIGN.md stdout/stderr contract. Data → stdout, errors → stderr, prefix `Error:` consistent throughout. The `cmd_show` use of `print!` (not `println!`) is correct because `format_show_block` already terminates with `\n` — verified the block ends with one newline, not two, in the captured stdout.

**Classification:** Dismissed. Routing is correct on all new paths.

---

**Finding 7 — Exit codes 0/1 contract preserved on all new code paths (CLI Dim 9)**

Verified: all success paths exit 0; all error paths (invalid ID, not-found, empty description) exit 1. No new exit-101 / clap-exit-2 leakage in the Layer 6 surface. SIGPIPE handler from Layer 3 remains installed (release binary still has the `signal(SIGPIPE, SIG_DFL)` call at process start), so `tracker show <id> | head -1` does not panic.

**Classification:** Dismissed. Exit-code contract holds.

---

### Hallucinated

**Finding 8 — `tracker show` of an issue with a description containing only whitespace should distinguish `(none)` from `(blank)`**

Initial concern: `Description: ` followed by nothing might be visually identical to `Description: (none)` when the stored description is a literal whitespace string. But — the create-side validation rejects empty/whitespace-only descriptions (`validate_description` errors on `raw.trim().is_empty()`). A whitespace-only description cannot enter storage via the binary. (Hand-edited `tracker.json` is a separate concern but would render the actual whitespace, which is the user's responsibility for hand-editing.) The defect path I was reasoning about is unreachable from the CLI.

**Classification:** Hallucinated. The validation boundary prevents the input that would produce the ambiguity.

---

**Finding 9 — `tracker delete` is missing a `--yes` / `--force` flag and should warn the user about deletion**

Initial impulse: destructive commands should require confirmation or a force flag.

**Classification:** Hallucinated *as a Layer 6 defect*. The D1 approved deviation in DESIGN.md is explicit, the SO has signed off, the `rm` / `git rm` family analog is named, and prior UX rounds (Review 1 F2) already dismissed this with cross-reference. Re-raising it without new evidence would be a meta-leak ("I would not have done it this way") rather than a defect against the spec. The user-side mitigation (`tracker show <id>` first) is documented.

---

### Summary

Round 8 finds 1 open, 5 dismissed, 2 hallucinated. The Layer 6 surface (`--description`, `tracker show`, `tracker delete`) is high-quality almost everywhere:

- The most consequential new output (`show` labelled-block) is character-for-character to the DESIGN.md example, including multi-line description indentation.
- Error messages on the new paths preserve the project's voice and specificity.
- stdout/stderr discipline and exit codes are preserved.
- The delete confirmation text is informative without being verbose.

The single open finding is **#1**: the `show` and `delete` `--help` doc-comments are below the bar the project established for `create` / `list` / `status` in earlier layers. The implementation is correct; the help-text effort is uneven. Concretely: `<ID>` is documented as `Issue ID` without naming the "positive integer ≥ 1" rule; the destructive-without-confirmation behavior of `delete` is signalled in the top-level listing but not reinforced in `delete --help`. The fix is doc-comment-only — no clap argument structure change — and brings the two new commands to parity with the existing ones.

**Discoverability assessment:** Top-level `tracker --help` correctly lists both new commands with helpful one-line summaries (`show` and `delete` are present; `create` was correctly updated to mention `--description`). `tracker create --help` correctly lists `--description` with its valid-values gloss ("Free-form description (stored verbatim; not trimmed)"). The gap is in the per-command help for `show` / `delete`, not in the top-level listing.

**Top UX concern:** Finding 1 — `show` / `delete` `--help` text is the thinnest possible implementation rather than the project-standard level of detail established in Layer 1–4. Cosmetic but visible; recommend doc-comment uplift as Layer 6 polish or rolled into Layer 7.

**Coordination:**
- **Finding 1** → cross-reference [SOFTWARE-ENGINEER-REVIEW.md](SOFTWARE-ENGINEER-REVIEW.md) for the doc-comment edits in `src/main.rs`. No spec change required; DESIGN.md `--help` contract is already satisfied at the contract level — this is symmetry-with-existing-commands polish.

**Files modified:** Only this log appended.

---

## Review 9 — 2026-05-11 02:00Z

**Round:** UX Review 9 (Round-2 closure for Layer 6)
**Scope:** Verify Round-1 Open finding is resolved by commit `9b775f0`. Warm closure-verification.

### Round-1 finding closure

- **F1 (`show` / `delete` `--help` one-line stubs vs. Layer 1-4 standard):** **Resolved by commit `9b775f0`.** Doc-comments in `src/main.rs` expanded:
  - `Show`: now reads "Show full details for an issue: ID, Title, Status, Priority, Labels, Description, Created, Updated"; `<id>` positional documented as "Issue ID (positive integer, >= 1)".
  - `Delete`: now reads "Delete an issue. No confirmation prompt (see DESIGN.md D1); deleted IDs are never reused."; `<id>` positional same as Show.

Verified via `cargo run --quiet -- show --help` and `cargo run --quiet -- delete --help` against the release binary. Help text now matches the depth of `create --help` / `list --help` / `status --help` (each documents the valid-values surface and the destructive/non-mutating posture of the operation).

### Carry-forward verification

- Trim-asymmetry round-trip (UX R6 F1): no regression at Layer 6.
- stdout/stderr discipline: confirmed unchanged — `show` data → stdout, `delete` confirmation → stdout, all errors → stderr.

### New findings

*(none this round.)*

### Summary

1/1 Round-1 UX finding Resolved. Layer 6 UX-domain is at MVR. `--help` depth is now uniform across all five subcommands.

**Coordination:** *(none — closure pass)*

---

## Review 10 — 2026-05-11 22:30Z

**Round:** UX Review 10 (Layer 7 — Polish: `--help`, color, error specificity). Cold session per `prompts/review-session.md`.
**Scope:** Layer 7 surface as committed at `8ed7db3` and prior. CLI supplement (replacement) dimensions 1, 2, 3, 4, 5, 6, 7, 8, 10, 11. Whole-application regression check across all five subcommands and every error path from Layers 1–6. Release binary built clean (`cargo build --release`, `Finished` only); exercised in `/tmp/uxr10` against a fresh `tracker.json` for the happy paths, and with redirected stdout / `script -q /dev/null` TTY simulation / `cat -v` escape-rendering for the color and pipe paths.
**Sycophancy disclosure:** Reviewer cannot perceive color directly — verifications below rely on `cat -v` rendering of the ANSI sequences emitted to a `script(1)`-allocated PTY. Color values, contrast, and accessibility for users with color-vision deficiency are reasoned about from the spec's color choices, not from observation. Flagged where applicable.

### Regression check

- Trim-asymmetry round-trip (UX R6 F1, R7 F1): re-verified — `tracker list --label "  bug  "` matches stored `bug`; `tracker list --label ""` errors. No regression.
- stdout/stderr discipline (UX R1 F1 / Dismissed; R8 F6 / Dismissed): re-verified on every new-and-prior code path. Empty-state messages route to stderr (`tracker list | wc -l` returns `0`). Color output is suppressed on piped stdout for both `list` and `show`. No regression.
- Show block format (UX R8 F3): re-verified byte-for-byte against DESIGN.md examples. No regression.
- Delete confirmation deviation (D1) (UX R1 F2, R8 F4 / R8 F9): re-verified that the SO-approved deviation is documented and the success-message convention is preserved. Not re-raised on its own; see Finding 1 below for an adjacent disclosure concern.

### Open

**Finding 1 — `NO_COLOR` / `CLICOLOR` / `CLICOLOR_FORCE` environment variables are not honored; users have no way to suppress color on a TTY (CLI Dim 11 — verbose/quiet; CLI Dim 8 — error/control surface; cross-cuts Accessibility)**

Reproduction (release binary, `script -q /dev/null` to allocate a PTY so `IsTerminal::is_terminal()` returns true):

```
$ NO_COLOR=1 script -q /dev/null tracker list --status open | cat -v
... 1     open         ^[[1;31mhigh^[[0m      bug                   High thing ...

$ CLICOLOR=0 script -q /dev/null tracker list --status open | cat -v
... 1     open         ^[[1;31mhigh^[[0m      bug                   High thing ...
```

`NO_COLOR=1` and `CLICOLOR=0` both leave the ANSI sequences in place. The implementation in `src/lib.rs:591` and `src/lib.rs:835` gates color exclusively on `std::io::stdout().is_terminal()` — no env-var override exists.

`NO_COLOR` is the de-facto cross-tool standard documented at <https://no-color.org/> and honored by `git`, `ls --color=auto` (via `LS_COLORS`), `ripgrep`, `bat`, `fd`, `eza`, `delta`, `cargo` itself (since 1.70), and most modern Rust CLIs that emit color. It is a single-line check at the call sites (`use_color &= std::env::var_os("NO_COLOR").is_none()`). `CLICOLOR_FORCE=1` (BSD convention) and `CLICOLOR=0` are the secondary convention pair; `NO_COLOR` alone covers the most important opt-out case.

Why this matters for a portfolio CLI:

1. **Accessibility opt-out is the user's bargaining position with a color-blind palette.** The spec ratifies `red high` + `green done` — the canonical deuteranopia/protanopia pitfall (≈5% of men cannot reliably distinguish these). The asymmetry below (Finding 2) is the in-band mitigation; `NO_COLOR` is the out-of-band mitigation. A user who cannot disambiguate the colors cannot turn them off.
2. **Low-color terminals (`TERM=dumb`, `screen-256color` over flaky SSH, basic `vt100`) emit literal escape bytes as garbage.** The user has no recourse.
3. **Test/CI environments where `is_terminal()` returns true** (some terminal multiplexers, `expect`, `script`) cannot opt out of color even when a downstream consumer needs clean text.
4. **The implementation cost is one line each at the two call sites**, plus a small unit test that the helper returns `None` when `NO_COLOR` is set. This is well inside Layer 7 polish scope.

DESIGN.md does not currently mention `NO_COLOR` or any color-suppression env var. The spec is silent — neither requires nor forbids honoring it. A literal reading therefore says "the implementation matches the spec." A user-experience reading says "the spec's silence is itself a defect against an accessibility convention every comparable tool follows."

**Classification:** Open — Raised to SO (proposal: amend DESIGN.md "Interface / Color output" to add a paragraph: "When stdout is a TTY, color is also suppressed if the environment variable `NO_COLOR` is set to any non-empty value, per <https://no-color.org/>. `CLICOLOR=0` is honored equivalently. `CLICOLOR_FORCE=1` is not honored — color is never emitted to a non-TTY stdout regardless of env vars, to preserve the pipe-cleanness contract."). If SO ratifies, the implementation is a one-line edit to each of the two `is_terminal()` call sites and a unit test. If SO declines, the rationale should be documented in DESIGN.md to make the deliberate omission visible.

Cross-reference: this affects the same color path that Finding 2 raises; resolving Finding 1 (env-var opt-out) also gives users with color-vision deficiency a clean escape hatch even before Finding 2 (in-band redundancy) is addressed.

---

**Finding 2 — Color asymmetry between `priority` (partial bold redundancy) and `status` (no redundancy): `high` is bold-red but `done` is plain green, leaving deuteranopia/protanopia users with no non-color signal to distinguish `done` from `open` (CLI Dim 3 — output scannability; cross-cuts Accessibility)**

Reproduction (release binary, `script -q /dev/null`):

```
priority=high     → ^[[1;31m...^[[0m   (bold + red)
priority=medium   → ^[[33m...^[[0m    (plain yellow)
priority=low      → (no escape)        (default)
status=in-progress → ^[[36m...^[[0m   (plain cyan)
status=done       → ^[[32m...^[[0m    (plain green)
status=open       → (no escape)        (default)
```

Implementation: `priority_ansi` at `src/lib.rs:51` returns `\x1b[1;31m` for `high` (bold+red). `status_ansi` at `src/lib.rs:65` returns `\x1b[32m` (plain green) for `done` and `\x1b[36m` (plain cyan) for `in-progress`. The asymmetry is unintentional from the spec — DESIGN.md "Color output" table reads:

| Value | Color |
|---|---|
| `high` priority | Red / bold |
| `medium` priority | Yellow |
| ... |
| `in-progress` status | Cyan |
| `done` status | Green |

The "Red / bold" cell is the only cell with a non-color attribute. The other "highlight a special state" cell (`done`) does not have a parallel non-color attribute. This is a spec gap that the implementation faithfully reproduces.

Why this is a real UX defect, not a cosmetic one:

1. **Deuteranopia (red-green color blindness) is the most common form of CVD**, affecting roughly 5% of men. The pair `red high` + `green done` is the textbook miss case — both render as near-identical muddy yellow-brown to a deuteranope.
2. **`high` already has the boldness fallback** because the spec spelled "Red / bold" — a deuteranope cannot tell the priority is red, but the bold weight is unambiguous. This is the *correct* design pattern: never rely on color alone (WCAG 1.4.1 "Use of Color", the SC most directly applicable to terminal output).
3. **`done` does NOT have this fallback.** `status=done` is `\x1b[32m` only. A deuteranope reading `tracker list --status done` vs. `tracker list --status open` sees identical-weight uncolored-looking values in the same column position. The user cannot rely on the status column to disambiguate at a glance.
4. **`in-progress` is partially saved** by being a six-syllable word in the column, but only because the text content differs from `open`/`done`. Color adds no information for a CVD user; it adds visual clutter without payload.

The fix has two natural shapes:

- **A. Symmetric boldness.** Apply `bold` to every non-default value (`done`, `in-progress`, `medium`), so the bold attribute consistently means "this is not the default state." This makes the implementation read uniformly and gives every value a non-color cue. ANSI: `\x1b[1;32m`, `\x1b[1;36m`, `\x1b[1;33m`. Spec update: change the four "highlight" rows to "<color> / bold".
- **B. Symmetric plain.** Drop the `bold` from `high` so the spec describes pure color throughout. Easier to implement (one-byte edit). Weaker accessibility. Not recommended.

Option A is the accessibility-correct choice and matches the established pattern of `high`. The implementation cost is two extra `1;` prefixes in the ANSI strings.

**Classification:** Open — Raised to SO (proposal: amend DESIGN.md "Color output" table to read "Cyan / bold" for `in-progress`, "Green / bold" for `done`, and "Yellow / bold" for `medium`. The single source of truth becomes "every highlighted value is bold-plus-color"). SE follow-up: update `status_ansi` and `priority_ansi` accordingly; the existing piped-no-ANSI tests (`list_piped_has_no_ansi_codes`, `show_piped_has_no_ansi_codes`) still pass; the manual TTY-rendering checklist item should be re-walked.

Cross-reference: Finding 1 (NO_COLOR) is the out-of-band mitigation for the same accessibility surface. Both can land in the same SO round.

---

### Dismissed

**Finding 3 — `tracker` (no args) routes the help block to stderr and exits 1 (CLI Dim 4 — stdout/stderr discipline)**

Reproduction:

```
$ tracker 1>/tmp/out 2>/tmp/err ; echo exit=$?
exit=1
$ cat /tmp/out   # empty
$ cat /tmp/err   # the Usage block
Personal issue tracker
...
```

`tracker` with no subcommand emits the help text on stderr (not stdout) and exits 1. The contrast with `tracker --help` (which exits 0 and routes to stdout) is by design in clap and the `try_parse` transform in `src/main.rs:72-83`: a missing-subcommand error is a usage error, which routes to stderr with `Error:` prefix-rewriting and exits 1. An explicit `--help` request is a help action, which routes to stdout with exit 0. The user who runs `tracker` with no args gets the help content they need to recover, in the channel that conventionally carries error/diagnostic output. This matches the convention of `git` (exit 1, help on stderr) and is well-behaved.

**Classification:** Dismissed. The dual behavior is the correct CLI convention. The help content is the same. The user who pipes `tracker | cat` correctly sees nothing on stdout and the usage block on the terminal.

---

**Finding 4 — `tracker help` and `tracker help <subcommand>` work and route to stdout, providing a third discoverability path (CLI Dim 1)**

Verified:

- `tracker help` → exit 0, full top-level help on stdout (matches `tracker --help`).
- `tracker help create` → exit 0, `create`'s help on stdout (matches `tracker create --help`).

Clap provides this for free. The spec only requires `--help`; the additional `help` subcommand is a bonus discoverability path. The top-level help also lists `help` as a command, so a user who types `tracker` and reads the output learns about both routes.

**Classification:** Dismissed. The help surface is over-delivered, not under-delivered.

---

**Finding 5 — Error messages are at or above the bar across all six subcommands and all error categories (CLI Dim 8)**

Manual walk of every L1–L7 error path (matches the Layer 7 AC "Review each error message from all prior layers manually"):

| Path | Reproduction | Message | Verdict |
|---|---|---|---|
| empty title | `create ""` | `Error: Title cannot be empty.` | states the rule |
| title control | `create $'a\nb'` (would be) | `Error: Title cannot contain control characters.` | states the rule |
| invalid priority | `create X --priority urgent` | `Error: Invalid priority 'urgent'. Expected: low, medium, or high.` | names bad value + valid set |
| empty label | `create X --label ""` | `Error: Label cannot be empty.` | states the rule |
| comma in label | `create X --label "a,b"` | `Error: Label cannot contain a comma.` | states the rule |
| empty description | `create X --description ""` | `Error: Description cannot be empty.` | states the rule |
| desc ESC | `create X --description $'a\x1bb'` | `Error: Description cannot contain control characters other than newline.` | states the rule |
| invalid status | `status 1 closed` | `Error: Invalid status 'closed'. Expected: open, in-progress, or done.` | names bad value + valid set |
| non-int ID | `show abc` | `Error: 'abc' is not a valid issue ID. Expected a positive integer.` | names bad value + rule |
| zero ID | `show 0` | `Error: '0' is not a valid issue ID. Expected a positive integer.` | names bad value + rule |
| negative ID | `delete -1` | `Error: unexpected argument '-1' found` + tip + usage | clap-level; states the tip (`use '-- -1'`) |
| not found | `show 99` | `Error: Issue #99 not found.` | names the missing ID |
| multi-label-list | `list --label a --label b` | `Error: the argument '--label <LABEL>' cannot be used multiple times` + usage | clap-level |
| unknown subcmd | `frobnicate` | `Error: unrecognized subcommand 'frobnicate'` + usage | clap-level; names bad value |
| empty label filter | `list --label ""` | `Error: Label cannot be empty.` | round-trip symmetric |
| corrupt JSON | hand-edit | `Error: Could not read tracker data. The file may be corrupt. Delete tracker.json to start fresh.` | states the recovery action |
| permission | (not exercised this round) | `Error: Could not read tracker data: <reason>.` | per spec |
| write fail | (not exercised this round) | `Error: Could not save tracker data: <reason>.` | per spec |

All errors begin with `Error:` per the spec. All errors are on stderr. All errors exit 1. All errors that interpolate user input go through `display_safe`, which escapes Cc characters as `\u{XX}` so a pasted ANSI sequence cannot cross from stderr to the terminal as raw bytes — verified by code review at `src/lib.rs:268-278`. The clap-level errors (negative ID, multiple `--label`, unknown subcommand) include the standard usage block and a recovery tip when applicable. Two of those messages (`unrecognized subcommand 'frobnicate'` and `unexpected argument '-1' found`) do not include `Expected: ...` enumerations because clap does not know the value space; the project's hand-rolled validators do enumerate.

**Classification:** Dismissed. The Layer 7 manual review AC is genuinely satisfied. The message bar is consistent across all six subcommands. No specificity gaps.

---

**Finding 6 — `--help` text is complete and accurate across all six surfaces; valid-value enumerations are inline for every flag that has them (CLI Dim 1, Dim 2)**

Verified:

- `tracker --help` lists every subcommand with a one-line summary; `delete`'s line signals the no-confirmation behavior and the no-id-reuse rule.
- `tracker create --help` lists `--description`, `--priority` (with `low, medium, high (default: medium)` inline), `--label` (with the `repeatable; deduplicated; case-preserved` semantics inline).
- `tracker list --help` lists `--status` (with `open, in-progress, done`), `--priority` (with `low, medium, high`), `--label` (with `case-sensitive exact match; single value only` — the asymmetry with create's repeatable `--label` is correctly signposted, closing UX R1 F4).
- `tracker status --help` lists both positionals with valid values inline.
- `tracker show --help` and `tracker delete --help` both document `<ID>` as `Issue ID (positive integer, >= 1)` — at the bar set by `create` / `list` / `status` after the R8 F1 → R9 closure.

**Classification:** Dismissed. UX R1 F4 (label-asymmetry visibility) is also definitively closed by the inline single-value-only docstring on `list --label`. No new gaps.

---

**Finding 7 — Empty-state messages route to stderr; piped consumers see clean stdout (CLI Dim 4, Dim 6)**

Reproduction:

```
$ tracker list | wc -l    # default-open view, empty tracker
No open issues. Nice work!
       0
```

The `0` from `wc -l` confirms stdout is empty; the message goes to stderr (visible to the user but not piped). Same for `No issues match the given filters.` Verified for both the default-open empty case (no `tracker.json`) and the filter-no-match case (all-done tracker, default-open view).

**Classification:** Dismissed. The stderr routing of empty-state messages is correct and was the right call relative to UX R1 F1 (which dismissed the alternative). Pipe-cleanness is preserved.

---

**Finding 8 — Color is applied only to the value text in its cell, not to the row or header (CLI Dim 3)**

`script -q /dev/null` rendering shows the header line `ID    Status       Priority  Labels                Title` with no ANSI escapes; only the value cells in subsequent rows are wrapped. Both the `\x1b[1;31m` prefix and `\x1b[0m` reset hug the value text — surrounding padding (the `pad_after_color` helper at `src/lib.rs:91`) is uncolored. Matches the spec contract "Color is applied only to the value text in its column cell, not to the entire row or header."

**Classification:** Dismissed. The value-only color rule holds for both `list` and `show`.

---

### Hallucinated

**Finding 9 — Show output's 13-char label column plus value can exceed 80 cols on narrow terminals (responsive-design concern)**

Initial concern: a 60-col terminal would wrap the table output. Verified by visual measurement: `tracker list` of a long title plus long labels produces a ~99-col line; on a 60-col terminal this wraps to a second line and breaks tabular alignment.

But: the spec's column-width contract explicitly fixes the widths (`ID` 4, `Status` 11, `Priority` 8, `Labels` 20, `Title` up to 50, with two-space separators) and the truncation rules (`Labels` at 20 with `…`, `Title` at 50 with `…`). The minimum line is 4 + 2 + 11 + 2 + 8 + 2 + 20 + 2 + 1 = 52 cols for a title with one character, and 99 cols at maximum (50-char title). 99 cols exceeds 80 — the conventional terminal default — but the design choice is explicit: the spec ratifies the column widths and would have had to specify a narrower variant or a `--narrow` mode to address this. DESIGN.md "Out of Scope" does not name this but the spirit ("interactive interactive macOS CLI") makes a 80+ terminal a fair assumption.

`tracker show` has a similar concern — long titles or descriptions can exceed a narrow terminal — but the per-line break is on the description's own newlines, which the user controls, and the show block is by design for full-detail viewing where wrapping is acceptable.

**Classification:** Hallucinated. The column widths and truncation rules are deliberate spec choices. The portfolio-CLI target (a developer's wide terminal) is reasonable. Re-raising as a real defect would require either evidence that a target user runs in a sub-80-col terminal habitually or a spec amendment to add a narrow variant — neither is present.

---

**Finding 10 — `tracker delete <id>` should reintroduce a confirmation prompt at Layer 7 polish**

Initial concern: Layer 7 is the polish layer; this is the moment to revisit "no confirmation" since it is the most user-visible irreversibility.

But: D1 in DESIGN.md "Approved Deviations" is the documented SO ratification with rationale (consistency with the rest of the binary, `git rm` / `rm` family precedent, recoverable via JSON edit, single-user threat model). UX R1 F2, R8 F4, and R8 F9 have all reached the same dismissal. The Layer 7 AC list does not mention confirmation, by design. Re-raising it without new evidence would be a meta-leak ("I would have built it with a prompt") rather than a defect against the spec.

**Classification:** Hallucinated. The deviation is documented, the SO has ratified, prior UX rounds agree. The user-side workflow (`tracker show <id>` before `tracker delete <id>`) is the documented mitigation. No new evidence to re-open.

---

### Summary

Review 10 finds **2 Open (both Raised to SO), 6 Dismissed, 2 Hallucinated**. Layer 7 polish is largely successful at the contract level: `--help` is uniformly informative across all six subcommands, color rendering on a TTY is correct and is correctly suppressed on a pipe, error messages are consistent in voice and specificity across every L1–L7 path, stdout/stderr discipline is preserved, and `display_safe` defends the error stream against ANSI-injection via interpolated user input. The Layer 7 AC list is technically satisfied.

The two real Open findings both surface the same root cause: **color is the user's only signal in two places where it should not be the only signal.** Finding 1 (no `NO_COLOR` opt-out) is the out-of-band mitigation that every comparable Rust CLI provides; Finding 2 (asymmetric bold redundancy between `priority=high` and `status=done`/`in-progress`) is the in-band mitigation half the spec already provides for `high` but does not extend to the other highlighted values. Both are spec-level findings raised to SO with concrete proposals; both have minimal implementation cost; both improve accessibility for a real user population (deuteranopia ≈5% of men, plus `TERM=dumb` / SSH / multiplexer cases).

**Top UX concerns:**
1. **Finding 1** — `NO_COLOR` / `CLICOLOR` env-var opt-out. The spec is silent; every comparable tool honors `NO_COLOR`. One-line fix per call site after SO ratifies.
2. **Finding 2** — `done` and `in-progress` lack the bold-redundancy `high` already has, so red-green color-blind users cannot disambiguate `done` from `open` in the status column without reading the text. Spec table should be amended to "Cyan / bold" + "Green / bold" + "Yellow / bold" for the three highlighted values.

**Coordination:**
- **Finding 1** → Raised to SO (DESIGN.md "Interface / Color output" amendment). On SO ratification, cross-reference [SOFTWARE-ENGINEER-REVIEW.md](SOFTWARE-ENGINEER-REVIEW.md) for the call-site edits and [QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md) for a regression test (a `script(1)`-allocated PTY plus `NO_COLOR=1` env var, asserting no `\x1b[` in stdout).
- **Finding 2** → Raised to SO (DESIGN.md "Color output" table amendment). On SO ratification, cross-reference [SOFTWARE-ENGINEER-REVIEW.md](SOFTWARE-ENGINEER-REVIEW.md) for the `status_ansi` / `priority_ansi` edits and [QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md) for the manual TTY-rendering checklist re-walk.
- **No findings raised to QE, Security, Platform, or SA for code-only action this round.**

**Files modified:** Only this log appended.

---

## Review 11 — 2026-05-12 00:00Z

**Round:** UX Review 11 (Layer 7 IAR Round 2 closure pass). Warm verification per CLOSURE-PROTOCOL.md §5; not a new adversarial round.

**Scope:** Verify R10 Open findings closed by commit `09b1905`. Inputs: DESIGN.md "Interface / Color output" amendments (NO_COLOR / CLICOLOR honoring + bold-redundancy spec); `src/lib.rs` `color_mode_from_env` + `priority_ansi` / `status_ansi` (now bold-on-highlighted); integration test `no_color_env_does_not_break_piped_invocation`; manual TTY re-walk pending.

### Round-1 finding closures

- **F1 — `NO_COLOR` / `CLICOLOR` / `CLICOLOR_FORCE` not honored:** **Resolved by `09b1905`.** DESIGN.md amendment ratifies the de-facto cross-tool standard (https://no-color.org/) — NO_COLOR (any non-empty value) and CLICOLOR=0 both force ColorMode::Off; CLICOLOR_FORCE deliberately not honored to preserve pipe-cleanness. `color_mode_from_env()` implements the precedence: TTY check → NO_COLOR → CLICOLOR=0 → On. Integration test `no_color_env_does_not_break_piped_invocation` verifies env-var passthrough on the piped path (TTY-positive verification deferred to manual checklist re-walk). The cross-tool consistency principle is now honored: user has the same opt-out lever in `tracker` that they have in `git`, `cargo`, `ripgrep`, `bat`, `fd`, `eza`, `delta`.
- **F2 — Color asymmetry: `high` is bold-red but `done` / `in-progress` / `medium` are plain:** **Resolved by `09b1905`.** DESIGN.md amendment changes the color table to "Red / bold" / "Yellow / bold" / "Cyan / bold" / "Green / bold" for the four highlighted values, leaving `low` and `open` plain so the highlighted-vs-unhighlighted dichotomy reads at a glance. `priority_ansi("medium", On)` returns `\x1b[1;33m`; `status_ansi("in-progress", On)` returns `\x1b[1;36m`; `status_ansi("done", On)` returns `\x1b[1;32m`. WCAG 1.4.1 *Use of Color* is now honored: every color cue carries a non-color cue (bold weight) so deuteranopia/protanopia users have a non-color signal to distinguish highlighted states.

### Manual checklist re-walk needed (carry-forward)

The Round-2 spec and implementation changes introduce new behaviors that the existing 7-item manual checklist (TODO.md L368-374) does not cover. Director should add and execute:

- **`NO_COLOR=1 tracker list` in terminal → no ANSI rendered** (CVD/accessibility opt-out).
- **`CLICOLOR=0 tracker list` in terminal → no ANSI rendered.**
- **`CLICOLOR_FORCE=1 tracker list | cat -v` → still no ANSI** (pipe-cleanness contract preserved).
- **`tracker list` in terminal with done / in-progress / medium values → bold weight visible** (CVD redundancy).
- **`tracker list | cat -v` with no issues matching → stderr empty-state has no ANSI** (R2 symmetric-stderr commitment).

### New findings

*(none — closure pass.)*

### Summary

Both R1 UX findings Resolved by R2 spec amendments + implementation: F1 NO_COLOR / CLICOLOR honoring is now in-spec and tested at the env-var level; F2 bold-redundancy gives every highlighted value a non-color cue per WCAG 1.4.1. The CVD-correctness gap that the original Layer-7 spec carried is closed. Manual TTY re-walk for the new behaviors is the standing carry-forward to merge gate.

**Coordination:** SO R24 — spec amendments ratified; Security R12 — NO_COLOR cross-domain coordination verified; QE R18 — integration coverage for env-var path acknowledged as partial (TTY-positive deferred to manual / future force_color seam).

**Files modified:** This log appended only. The DESIGN.md amendments and `src/lib.rs` implementation landed in `09b1905` under SO + SE authority per CLOSURE-PROTOCOL.md §1.

---

## Review 12 — 2026-05-12 12:00Z

**Round:** UX Review 12 (Layer 7 IAR Round 3 — five-commit refactor cluster: `ff0e85c` clippy pre-commit hook, `c341a54` render_cell ASCII debug_assert, `bd7511e` TRACKER_INTERNAL_FORCE_COLOR test seam, `3fa1f3c` cmd_list rendering extraction + column constants, `8db9437` three-module split). Cold session per `prompts/review-session.md`.

**Scope:** Whole-application UX surface re-verification. The R3 change set is documented as "pure refactor + test seam + ASCII guard + clippy hook"; the adversarial premise is that "no user-visible change" must be verified at the binary, not assumed from commit titles. Release binary built fresh from HEAD (`cargo build --release` → `Finished release [optimized]`); exercised in `/tmp/uxr12` against a fresh `tracker.json` for golden path, all six `--help` surfaces, all major error paths, and the four color-suppression / TTY / forced-color permutations. CLI supplement (replacement) dimensions 1, 2, 3, 4, 5, 6, 7, 8, 11.

**Sycophancy disclosure:** Reviewer cannot directly perceive color; ANSI bytes were verified by `cat -v` rendering of output captured through `script -q /dev/null` PTY allocation. Color appearance, contrast, and accessibility for users with CVD are reasoned from emitted byte sequences, not from observation. The R3 set claims "no user-visible change" — this review applies binary-level diff pressure to that claim rather than accepting it.

### Regression check (full L1-L7 surface against R3 HEAD)

- **Golden path lifecycle.** `create` x3 → `list` → `status 2 in-progress` → `show 2` → `delete 3` → `list`: stdout output byte-identical in shape to the Layer 7 manual checklist outputs (TODO.md L368-374 ticks). `Created issue #N: <title>`, the labelled-key `show` block (13-char label column, label-aligned multi-line description indent), `Issue #2 status → in-progress.` (with the `\u{2192}` rightwards-arrow), `Deleted issue #3.` — all exact-match to spec. No regression.
- **Persistent next_id counter.** Created issue #4 after deleting #3: `next_id: 5` in `tracker.json`, the SO R22 Option A invariant ("monotonically increasing, deleted IDs never reused") preserved by R3. No regression.
- **All six `--help` surfaces.** `tracker --help`, `tracker create --help`, `tracker list --help`, `tracker status --help`, `tracker show --help`, `tracker delete --help` all match the R10 F6 verified content byte-for-byte. `Issue ID (positive integer, >= 1)` still on `show` / `delete`; `repeatable; deduplicated; case-preserved` still on `create --label`; `case-sensitive exact match; single value only` still on `list --label`; valid-value enumerations still inline for `--priority` / `--status` / `<STATUS>` positionals. No clap-derived doc-comment regression from the module split. No regression.
- **Error paths.** `create ""` → `Error: Title cannot be empty.`; `show abc` → `Error: '<v>' is not a valid issue ID. Expected a positive integer.`; `show 99` → `Error: Issue #99 not found.`; `frobnicate` → `Error: unrecognized subcommand 'frobnicate'` + clap usage block + `try '--help'` tip. All begin with `Error:`, all on stderr, all exit 1. Cc-escape rule still active (the R10 F5 sweep stands). No regression.
- **Color env-var matrix.** All six relevant permutations re-verified against R3 HEAD via `script -q /dev/null` PTY allocation:
  - TTY, no env: `^[[1;31mhigh^[[0m`, `^[[1;33mmedium^[[0m`, `^[[1;36min-progress^[[0m`, `^[[1;32mdone^[[0m` — bold-redundancy preserved (R11 closure verification reaffirmed).
  - TTY + `NO_COLOR=1`: no ANSI on stdout. Preserved.
  - TTY + `CLICOLOR=0`: no ANSI on stdout. Preserved.
  - Piped stdout (no env): no ANSI. Preserved.
  - Piped + `CLICOLOR_FORCE=1`: still no ANSI (pipe-cleanness contract preserved; DESIGN.md "Interface / Color output" deliberate non-honor). Preserved.
  - Piped + `TRACKER_INTERNAL_FORCE_COLOR=1`: ANSI emitted (the test seam works). Documented behavior.
- **Color is applied only to value text in its cell.** R3 extraction into `render_cell(value, ansi, total_width)` (`commands.rs:218`) preserves the value-only color rule. Header row in `format_list_header` (`commands.rs:533`) carries no ANSI; padding bytes after `wrap_color` carry no ANSI. R10 F8 verification stands. No regression.

### Open

**Finding 1 — `TRACKER_INTERNAL_FORCE_COLOR` env-var, while documented as "test seam only, not part of the public CLI contract," is discoverable to a user who runs `strings target/release/tracker | grep -i color` and creates an undocumented "force color on pipe" surface with no help text, no graceful failure mode, and no rejection of the value `=1` if accidentally exported in a CI environment (CLI Dim 1 — discoverability; CLI Dim 11 — verbose/quiet modes; cross-cuts QE / Security)**

Reproduction (release binary):

```
$ strings /Users/<user>/.../target/release/tracker | grep -i 'force_color\|tracker_internal'
TRACKER_INTERNAL_FORCE_COLOR
... (binary string table entry)

$ TRACKER_INTERNAL_FORCE_COLOR=1 tracker list | cat -v
ID    Status       ...
1     open         ^[[1;31mhigh^[[0m      ...
```

The env var is bona-fide hidden from `tracker --help`, `tracker <sub> --help`, README.md, DESIGN.md, and CHANGELOG.md — confirmed by exhaustive grep. The doc-comment in `commands.rs:107-113` explicitly says "deliberately ugly, namespaced env var" and "not documented in `--help`, README.md, or DESIGN.md." Good. The R3 introduction (`bd7511e`) is mechanically correct against the QE R17 F1 deferral — TTY-positive color now has integration coverage.

But: the name `TRACKER_INTERNAL_FORCE_COLOR` appears in the binary string table (any `strings` invocation surfaces it), in source comments (`grep -r TRACKER_INTERNAL_FORCE_COLOR src/` returns 17 hits), and in the Round-2 R3 closure commit message (`bd7511e`). A user who discovers it has no documented escape hatch ("this is not a public flag") *inside the binary* — `--help` won't mention it, but `--help` also won't explain why setting it produces unexpected color on piped output. A future polish-layer reader will see the env var, recognize the namespace as "internal," but have no in-binary signal that the value `=1` is the *only* trigger or that the var is test-only. The R10 F3 finding (`tracker` with no args routes help to stderr and exits 1) was dismissed because clap's "missing subcommand → stderr help, exit 1" is the standard convention. The R12 analogue here — "binary surface includes a test seam discoverable via `strings`" — has no comparable standard convention. The closest analogue is `CARGO_INCREMENTAL_TEST_STUB` / `RUSTC_BOOTSTRAP` style internal-only env vars in `cargo` / `rustc`: there the convention is the variable is undocumented BUT setting it to a wrong value produces a clear "this is unsupported" message. Here, setting `TRACKER_INTERNAL_FORCE_COLOR=0` or `=true` silently falls through to default behavior, which is consistent with the doc-comment but offers no user-visible "you found an internal seam" signal.

Three orthogonal mitigations are possible; any one (or none, if SO ratifies the current posture) addresses the UX surface:

- **A. Add a stderr warning when the env var is set on a non-test invocation.** When `TRACKER_INTERNAL_FORCE_COLOR=1` is detected, emit `Warning: TRACKER_INTERNAL_FORCE_COLOR is an internal test seam and is not a supported public flag. Color is forced on for this invocation.` on stderr. Cost: 3 lines in `color_mode_from_env`. Side effect: contaminates the stderr stream that `assert_cmd` tests assert against. Probably not worth the test-side disruption.
- **B. Add a one-line stanza to README.md "Color output" section.** "An internal `TRACKER_INTERNAL_FORCE_COLOR=1` env var exists for integration test purposes only; it is not part of the supported CLI contract and may change without notice." Cost: one paragraph. Trade-off: violates the explicit doc-comment claim ("not documented in `--help`, README.md, or DESIGN.md"). The doc-comment's claim is itself a UX choice — if the variable is genuinely discoverable via `strings` and is in the commit history and source, the "no documentation anywhere" posture is more of a fig leaf than a hidden-by-default property.
- **C. Rename to a more aggressively-hostile-to-discovery name.** E.g., a SHA-prefixed `TRACKER_FORCE_COLOR_TEST_ONLY_d8f3e1b2=1` so the value is non-guessable. Cost: every test updates. Marginal benefit: a user running `strings` still sees it. The actual security/safety property is "this is an internal seam," and a strings-grep can always find it; the question is what the discoverer should be told. Probably overengineered.

DESIGN.md's "Interface / Color output" section currently does not name the test seam. The cleanest pattern is probably (B): add the README disclosure and update the `commands.rs:108-110` doc-comment to reflect the disclosure. The doc-comment's "not documented in README.md" claim is currently true at HEAD; the R12 raise is that this is the *wrong* posture given the variable is discoverable.

**Classification:** Open — Raised to SO. Decision request: ratify current "hide-via-naming-only" posture (and update commands.rs doc-comment to remove the "not documented in README.md" claim, which is true but UX-defective), OR ratify a one-paragraph README disclosure (mitigation B). The technical implementation is unaffected; only the disclosure surface changes.

---

**Finding 2 — TODO.md manual testing checklist (L368-374) has NOT been updated with the 5 carry-forward items introduced by R11 (Round-2 spec/behavior changes); the director's Round-2 manual TTY re-walk for `NO_COLOR` / `CLICOLOR` / `CLICOLOR_FORCE` / bold-redundancy / stderr-empty-state remains unverified at the checklist level (CLI Dim 1 — discoverability of behavior to a future maintainer; cross-cuts QE)**

Reproduction:

```
$ grep -n "NO_COLOR\|CLICOLOR\|TRACKER_INTERNAL" issue-tracker-cli/TODO.md
(no output)

$ sed -n '368,375p' issue-tracker-cli/TODO.md
**Manual Testing Checklist:**
- [x] Run `tracker --help` and each subcommand `--help` ...
- [x] Run `tracker list` in terminal → verify `high` priority is red/bold, `in-progress` is cyan, `done` is green
- [x] Run `tracker list | cat` → verify output contains no `\033[` escape sequences
- [x] Run `tracker show <id>` in terminal with an `in-progress`/`high` issue → verify coloring in show output
- [x] Run `tracker show <id>` piped → no ANSI codes
- [x] Review each error message from all prior layers manually: does it say what went wrong and what the valid alternatives are?
- [x] `tracker frobnicate` → exit 1, stderr usage error
```

The checklist still reflects the pre-R2 spec wording — `in-progress is cyan, done is green` (no mention of bold for these values, which R2's bold-redundancy amendment introduced). The 5 items the R11 entry explicitly told the director to add:

1. `NO_COLOR=1 tracker list` in terminal → no ANSI rendered
2. `CLICOLOR=0 tracker list` in terminal → no ANSI rendered
3. `CLICOLOR_FORCE=1 tracker list | cat -v` → still no ANSI (pipe-cleanness)
4. `tracker list` in terminal with `done` / `in-progress` / `medium` values → bold weight visible
5. `tracker list | cat -v` with no matching issues → stderr empty-state has no ANSI

...are missing from TODO.md. R3 (`ff0e85c` clippy hook + `c341a54` ASCII assert + `bd7511e` test seam + `3fa1f3c` extraction + `8db9437` split) did not address this — every R3 commit is code/test/hook; none touched TODO.md.

The CHANGELOG R2-closure entry says explicitly: "Director to add the new manual items to TODO.md and re-tick." This is an explicit director action item that has not landed. The R3 binary-level re-verification this session performs (Regression check above) does confirm the behaviors, so the *behavioral* property is fine — but the *artifact* (TODO.md as the project's manual-testing source of truth) has drifted from the spec at HEAD. A future reader walking TODO.md will not exercise the env-var matrix, the bold-redundancy, or the stderr-empty-state cleanness, because those items don't appear.

This is a UX defect against the maintainer-as-user surface (TODO.md is the canonical "how do I sanity-test this layer" document). The portfolio-CLI target audience explicitly includes "agents resuming work between sessions" (per README.md L5), so a drifted checklist will mis-teach the next maintainer about what L7 covers.

**Classification:** Open. Suggested fix: append items 1-5 above to TODO.md L368-374 under a new "Layer 7 Round 2 / Round 3 carry-forward" subhead with checkboxes for the director to tick after a manual re-walk. Round 3 itself doesn't add new manual items beyond what Round 2 introduced, but Round 2's items still need to land. The 6 items the R12 prompt names ("manual checklist re-walk note. R2 created 6 new manual checklist items that the director needs to add to TODO.md and execute") aligns with R11's 5 items plus arguably the show-block-bold-redundancy item (currently bundled into item 4); a 6-item slate is reasonable.

### Dismissed

**Finding 3 — `c341a54` introduced a `debug_assert!(value.is_ascii(), ...)` in `render_cell` that would panic in `cargo test` on a non-ASCII colored value, but cannot panic on any spec-legal input (CLI Dim 2)**

The `render_cell` debug-assert (`commands.rs:219-226`) panics if `value` is not ASCII. Both current call sites pass `issue.status` or `issue.priority`, which are validated against closed enums (`STATUS_ORDER` / `PRIORITY_ORDER`) at parse and load time — every legal value is ASCII (`open`, `in-progress`, `done`, `low`, `medium`, `high`). The debug-assert exists to surface a spec amendment that would relax the ASCII constraint (e.g., non-ASCII status labels), at which point the panic prompts replacing `chars().count()` with `UnicodeWidthStr::width(value)`. In release builds the assert is compiled out, so user-visible behavior is unchanged. No regression from R2.

**Classification:** Dismissed. The assert is defensive coding against a future spec change; user impact is zero in release builds. R3 stance is correct.

---

**Finding 4 — `8db9437` three-module split (`commands.rs` / `storage.rs` / `validate.rs`) might have changed `cargo doc` rendering for public surfaces (`cmd_*`, `Issue`, `Tracker`, `CreateArgs`, `ColorMode`, `color_mode_from_env`, `display_safe`, `sanitize_quoted_values`, etc.)**

Initial concern: a pure-API user of the crate (running `cargo doc`) would see different module paths after the split (e.g., `tracker::commands::cmd_list` vs. the prior `tracker::cmd_list`). However, `src/lib.rs:43` re-exports the public surface from the submodules: `cmd_create, cmd_delete, cmd_list, cmd_show, cmd_status, color_mode_from_env, label_matches, ...` are all surfaced at the crate root, preserving the previous flat-namespace contract. The `tracker::cmd_list` path that `main.rs` uses works exactly as before.

For a `cargo doc` reader, the rendered surface is unchanged at the crate root; the submodules show up as additional documentation surfaces but the canonical paths are preserved. No clap-derived `--help` text touches submodule paths.

**Classification:** Dismissed. The re-exports in `src/lib.rs` preserve the API path contract. R3 module split is a refactor that maintains the public surface.

---

**Finding 5 — The `ff0e85c` clippy pre-commit hook landing is internal-only and has no user-visible UX surface (CLI Dim 11 — verbose/quiet)**

The clippy pre-commit hook runs at `git commit` time on the developer's machine, not at end-user invocation. End-user `tracker` behavior is unaffected. The hook is documented as PE R12 F3 closure.

**Classification:** Dismissed. The hook is a developer-tooling change with zero end-user UX surface.

---

**Finding 6 — `3fa1f3c` `cmd_list` extraction into `filter_issues` / `sort_issues` / `format_list_header` / `format_list_row` pure helpers might have changed the output of `tracker list`**

Initial concern: a pure-function extraction risks subtle off-by-one or reordering. Verified via golden-path lifecycle: header row `ID    Status       Priority  Labels                Title` (4 + 2 + 11 + 2 + 8 + 2 + 20 + 2 + 5 = 56 byte-positions through the start of Title) matches the DESIGN.md "Interface / Color output / List output format" example byte-for-byte. Row content for #1 (high/bug,auth/Fix login bug) and #2 (open/medium/feature/Add search bar) matches the column-width contract. Truncate-with-ellipsis behavior at `LABELS_WIDTH=20` and `TITLE_WIDTH=50` preserved.

**Classification:** Dismissed. The extraction is byte-preserving against the spec. R3 stance is correct.

### Hallucinated

**Finding 7 — The R3 module split should have introduced module-level `--help` text describing the module organization to users**

Initial concern: a multi-module crate ought to surface its organization somehow. But: `tracker --help` is the user-facing surface; the module organization is an implementation detail of the crate, not of the binary. A user reading `tracker --help` does not need to know whether the subcommand handlers live in `commands.rs` or `lib.rs`. Surfacing module structure in `--help` would be category-misuse of the user-facing documentation surface.

**Classification:** Hallucinated. Module structure is implementation detail; the user-facing `--help` correctly omits it.

---

**Finding 8 — `tracker.json` at `/tmp/uxr12/tracker.json` appears in the working directory after this session ran, contaminating subsequent reviewer sessions in the same directory**

Initial concern: the session left a `tracker.json` behind. But: the spec explicitly says `tracker.json` lives "in the current working directory at the time the command runs," and `/tmp/uxr12` is a per-session temporary directory created by this review. Subsequent reviewer sessions should `rm -f tracker.json` (or `mkdir /tmp/uxr<N>` with the next session number) per the suite's standard review-directory hygiene. Not a UX defect — a session-hygiene practice.

**Classification:** Hallucinated. Per-session directory creation is the documented convention.

### Summary

Review 12 finds **2 Open, 4 Dismissed, 2 Hallucinated**. The R3 "pure refactor + test seam + ASCII guard + clippy hook" claim holds at the binary level: golden-path output, all six `--help` surfaces, error messages, color env-var matrix, bold-redundancy, pipe-cleanness, stderr-empty-state cleanness — every Layer 1-7 behavior verified against the release binary byte-for-byte. No user-visible regression from the five R3 commits.

The two Open findings are both about the *artifact surface around* the R3 work, not the behavior itself:

1. **Finding 1** — `TRACKER_INTERNAL_FORCE_COLOR` is discoverable via `strings` and the source tree but lacks any in-binary signal that it is a test-only seam. The "deliberately ugly, namespaced, undocumented" posture works against a casual reader but does not address the `strings`-discovery vector. Raised to SO with three mitigation options (warning on stderr / README disclosure / SHA-prefixed name); SO chooses.
2. **Finding 2** — TODO.md's manual testing checklist has not been updated with the 5 carry-forward items the R11 closure entry explicitly told the director to add. R3 binary-level verification confirms the behaviors, but the TODO.md drift means the next maintainer walking the manual checklist will not exercise the env-var matrix, bold-redundancy, or stderr-empty-state cleanness. Cross-cuts QE (manual-checklist-as-test-surface).

**Coordination:**
- **Finding 1** → Raised to SO (decide between "ratify current hide-via-naming posture and remove the inaccurate doc-comment claim" vs. "add README disclosure"). SE follow-up minimal regardless.
- **Finding 2** → Raised to SO / director (TODO.md is a planning artifact; the R11 closure entry explicitly tagged this as a director action). Cross-reference [QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md) — manual-checklist-as-test-surface aligns with QE R17 F1's "TTY-positive verification deferred to manual" stance; QE may want to acknowledge this carry-forward in its own R3 round.
- **No findings raised to Security, Platform, or SA for code-only action this round.**

**Files modified:** This log appended only.


**Round-3 finding closure — see [SO Review 26 ledger](SOLUTION-OWNER-REVIEW.md#review-26--2026-05-12-1500z--closure-ledger-closure-protocol-2c-reconciliation).** F1: Resolved (`ecec07f` SO R25 F2 DESIGN.md amendment naming `TRACKER_INTERNAL_FORCE_COLOR` as test-only / unstable). F2: Resolved (`ecec07f` SO R25 F3 TODO.md inline edit + `e28bef4` director-execution closure 6/6 ticked).