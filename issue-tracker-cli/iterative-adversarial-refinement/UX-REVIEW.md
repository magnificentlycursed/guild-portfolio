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
