# Solution Owner Review Log

This review is part of the [Iterative Adversarial Refinement (IAR)](README.md) suite. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

The purpose of this review is to guard the project against scope creep and over-engineering. DESIGN.md is a Scope of Work. The SO review holds the implementation to that contract: 100% of what was agreed, nothing that was not. DESIGN.md is the contract for every other domain review — SO must confirm it is faithful to the assignment before other domains evaluate the implementation against it.

---

## Review 1 — 2026-04-27 14:00Z

**Scope:** DESIGN.md pre-build spec review. No implementation exists yet. This pass evaluates whether DESIGN.md is a faithful, complete representation of the assignment brief before any code is written.

**Reference:** `apprentice-onboarding/02-the-methodology/02-tracking-your-work.md`

**Session note:** Review conducted in-session with spec authorship. Adversarial pressure applied post-authorship; acknowledged as a quality tradeoff. A second pass with a cold session is warranted before Layer 1 opens.

### Compliance Table

| Assignment requirement | Status | Notes |
|---|---|---|
| Create issue: title + optional description | Met | Feature 1 with preconditions, postconditions, error states |
| List open issues | Met | Feature 2, default `--status open` |
| Mark in-progress or done | Met | Feature 3, any-to-any state transitions |
| Set priority (low/medium/high) | Met | Feature 1 `--priority`, Feature 2 `--priority` filter |
| Add labels to issues | Met | Feature 1 `--label`; creation-only — see dismissed finding below |
| Filter by status, priority, or label | Met | Feature 2, AND-combined filters |
| List closed issues separately | Met | `tracker list --status done` |
| Delete an issue | Met | Feature 5 |
| Rust | Met | Binary name `tracker`, technology section |
| Local JSON file storage | Met | `tracker.json` in working directory |
| CLI subcommand interface | Met | `create`, `list`, `status`, `show`, `delete` |
| `tracker create "title" --priority high --label bug` | Met | Feature 1 synopsis |
| `tracker list` (open, sorted by priority) | Met | Feature 2 default behavior |
| `tracker list --status done` | Met | Feature 2 `--status` filter |
| `tracker list --label bug` | Met | Feature 2 `--label` filter |
| `tracker status <id> in-progress` | Met | Feature 3 |
| `tracker status <id> done` | Met | Feature 3 |
| `tracker show <id>` (with timestamps) | Met | Feature 4; `created_at` / `updated_at` per Layer 6 |
| `tracker delete <id>` | Met | Feature 5 |
| Out of scope: multiple users | Met | Out of Scope section |
| Out of scope: due dates | Met | Out of Scope section |
| Out of scope: subissues/hierarchy | Met | Out of Scope section |
| Out of scope: time tracking | Met | Out of Scope section |

### Resolved

**Finding 1 — Multi-line description display behavior undefined**

`DESIGN.md` stored description "verbatim" but the `show` output format was a single-line key-value block. A description containing `\n` would corrupt the display. Spec was silent on this case.

**Resolution:** Defined display convention: first line follows the `Description:` label, continuation lines indented 13 spaces to align with the value column. `\r\n` normalized to `\n` on storage.

---

**Finding 2 — `tracker list --label` with multiple flags: behavior undefined**

Feature 2 stated "only one `--label` filter is supported" but did not define what happens when the user passes two `--label` flags. A reviewer and an implementer would produce different behavior (last wins vs. clap error vs. AND-combine).

**Resolution:** Defined: clap rejects multiple `--label` flags on `list` with a usage error. Documented in Feature 2 and the Labels edge case section.

---

**Finding 3 — List column width contract incomplete**

The list output showed a fixed-width table example but gave no specification of how column widths are determined or what happens to long values. An automated test for `format_issue_row` would have no defined behavior to assert against.

**Resolution:** Defined dynamic-width columns with truncation contracts: title truncates at 50 chars with `…`, labels truncate at 30 chars with `…`. `show` always displays full values.

---

**Finding 4 — Empty state message deviated from assignment**

Assignment Layer 7 names the exact string `"No open issues. Nice work!"`. DESIGN.md used `"No open issues."` with no note that the wording was intentionally changed.

**Resolution:** Updated to match assignment wording: `"No open issues. Nice work!"`.

---

**Finding 5 — Feature 5 ("Add labels to issues") label post-creation path undocumented**

The Out of Scope section excluded "editing after creation" as a blanket rule, but did not explicitly address whether creation-only labels satisfies the assignment's Feature 5. A reviewer could flag this as under-delivery.

**Resolution:** Added explicit note to the Out of Scope bullet explaining that the assignment's Feature 5 is satisfied by creation-time labels, consistent with the assignment's own interface examples.

---

**Finding 6 — Label length unconstrained**

The edge case catalog covered empty labels but not maximum length. The label column in list output has a truncation limit; labels should have a corresponding input constraint so the contract is complete.

**Resolution:** Added 50-character maximum label length with error message `Error: Label must be 50 characters or fewer (got <n>).`. Added to Feature 1 preconditions, error states, data model, and edge cases.

---

### Dismissed

*(none)*

### Backlogged

*(none)*

---

---

## Review 3 — 2026-04-27 16:00Z

**Scope:** DESIGN.md assignment coverage audit — 100% coverage, no more, no less. Technology choices scoped to assignment.

**Session note:** In-session with prior spec work. Acknowledged quality tradeoff.

### Compliance Table

*(Addendum to Review 1 table — same features, reviewing scope boundary only)*

| Spec element | In assignment? | Status |
|---|---|---|
| Color output (Layer 7) | Yes — named in Layer 7 polish | Was excluded → resolved |
| `clap` / `serde_json` named in spec | No — implementation detail | Was over-specified → resolved |
| Title 200-char limit | No | Was over-specified → resolved |
| Label 50-char limit | No | Was over-specified → resolved |
| "No panics" blanket constraint | No (assignment scopes to crash-safe I/O only) | Was over-specified → resolved |
| "No warnings" in `cargo build` | No (assignment requires no errors only) | Was over-specified → resolved |

### Resolved

**Finding 1 — Color output excluded but is named in assignment Layer 7**

Assignment Layer 7 lists "colored output" alongside `--help` and empty-state messages, both of which were already in scope. Out of Scope said "terminal color is a polish-layer concern and is not part of the core contract."

**Resolution:** Removed from Out of Scope. Added color output spec to the Interface section: priority and status values are colored in list and show output when stdout is a TTY; color suppressed when piped. Defined color scheme: `high`=red/bold, `medium`=yellow, `low`=default; `open`=default, `in-progress`=cyan, `done`=green.

---

**Finding 2 — `clap` and `serde_json` named as spec constraints**

The assignment requires Rust and a local JSON file. Naming specific crates locks the implementation to those choices at spec level.

**Resolution:** Replaced with: "CLI argument parsing: any Rust crate or standard library. JSON serialization: any Rust crate. The observable interface contract is defined by this spec regardless of which libraries implement it."

---

**Finding 3 — Title 200-char and label 50-char limits not in the assignment**

Both limits were added by the spec author to close boundary gaps but create test obligations not required by the assignment.

**Resolution:** Removed both character limits from Feature 1 preconditions, error states, data model, and edge cases. Titles and labels are validated as non-empty only, per the assignment.

---

**Finding 4 — "No panics" constraint broader than assignment guidance**

Assignment says: handle missing/corrupt JSON without crashing. DESIGN.md had a blanket "no `unwrap()` or `expect()` in production code" rule.

**Resolution:** Replaced with a constraint scoped to the assignment's guidance: "The binary must not crash when `tracker.json` is missing or contains invalid data." Blanket `unwrap()` prohibition removed.

---

**Finding 5 — "No warnings" requirement not in assignment**

Assignment requires compilation with no errors. Warnings are normal during iterative development.

**Resolution:** Changed to "Compilation must succeed with `cargo build` with no errors at the end of each layer."

---

### Dismissed

*(none)*

### Backlogged

*(none)*

---

---

## Review 4 — 2026-04-27 18:00Z

**Scope:** DESIGN.md Testing Methodology section — does it fit or overshoot the assignment?

**Session note:** In-session. Acknowledged quality tradeoff.

### Resolved

**Finding 1 — Purity boundary section presupposes implementation structure and imports Phase 5 language**

The purity boundary section named specific function names (`validate_title`, `issue_matches_filters`, `format_issue_row`, `load_store`, etc.) locking the implementation to a particular module structure before any code exists. It also used the phrase "formally verifiable in principle" — VSDD Phase 5 language inappropriate for a Phase 1 first Rust project.

**Resolution:** Removed the purity boundary section. Replaced with a single-paragraph "Purity guidance" note: prefer separating validation, filtering, and sorting logic from I/O; this is a code organization principle, not a formal requirement. No function names named.

---

**Finding 2 — Display formatting and label deduplication named as standalone unit test items**

Both are internal implementation behaviors covered by the integration tests. A `tracker list` invocation that produces wrong output is already a failing integration test; a separate unit test for `format_issue_row` tests implementation internals, not assignment-defined behavior. Label deduplication is equally covered by the full lifecycle integration test.

**Resolution:** Removed "label deduplication logic" and "display formatting" from the automated tests list.

---

**Finding 3 — ID assignment listed as a named unit test item**

`max(existing_ids) + 1` is two lines of logic covered by the create → delete → create lifecycle integration test. Naming it as a spec-required unit test added test obligations for internal data plumbing.

**Resolution:** Removed "ID assignment" from the automated tests list.

---

### Dismissed

*(none)*

---

---

## Review 5 — 2026-04-27 19:00Z

**Scope:** Full clean-state pass of DESIGN.md against the assignment.

**Session note:** In-session. Acknowledged quality tradeoff.

### Resolved

**Finding 1 — Title 200-char limit survived Review 3**

The precondition `<title> after trimming is ≤ 200 characters` was not removed when Review 3 removed the corresponding error state and edge cases. The spec had a precondition with no enforced consequence — undefined behavior for titles over 200 chars.

**Resolution:** Removed the precondition. Titles are validated as non-empty only.

---

**Finding 2 — `clap` named in the `--help` flag description**

`clap generates --help for the binary...` re-introduced a named crate dependency after Review 3 removed them from the Technology line.

**Resolution:** Replaced with: "`--help` is supported for the binary and each subcommand."

---

**Finding 3 — `clap`-specific error message quoted in Feature 2**

Feature 2 quoted the exact clap error string for multiple `--label` flags, locking the spec to one library's error format.

**Resolution:** Replaced with: "a usage error is produced on stderr and the command exits 1."

---

**Finding 4 — `atty` named in color output spec**

`atty` is a third-party crate. Review 3 removed named crates from the Technology section; this reference was missed.

**Resolution:** Removed `atty`. Retained `std::io::IsTerminal` (standard library, Rust 1.70+).

---

**Finding 5 — "All pure functions are unit tested" orphaned text**

Review 4 removed the purity boundary section; this opening sentence referencing "pure functions" was left behind, making an unsupported claim.

**Resolution:** Replaced with: "The following behaviors are automatable and should be covered by unit tests:"

---

**Finding 6 — `--description ""` silently coerced to absent**

The assignment says "optional description." The spec treated empty-string description as equivalent to not providing the flag — a non-obvious silent coercion with no error feedback to the user.

**Resolution:** Changed to validation consistent with the rest of the boundary: `--description ""` (empty or whitespace-only after trim) → `Error: Description cannot be empty.` → exit 1. Error state added to Feature 1. Description edge case updated.

---

### Dismissed

*(none)*

---

## Open items entering Layer 1

- ~~Cold-session SO review recommended before Layer 1 opens~~ — completed (Review 6)
- ~~SA Review 1 findings resolved~~ — SA Review 2 also complete; see [SOLUTION-ARCHITECT-REVIEW.md](SOLUTION-ARCHITECT-REVIEW.md)
- ~~VDD-IAR Alignment review pending~~ — Reviews 1 and 2 complete; see [VDD-IAR-ALIGNMENT-REVIEW.md](VDD-IAR-ALIGNMENT-REVIEW.md)
- **Layer 1 merge gate:** at least one cold-session domain review (QE or Security) required after Layer 1 implementation — see VDD-IAR Finding 2

---

---

## Review 7 — 2026-04-27 22:00Z

**Scope:** Layer 1 Red Gate tests — scope compliance, spec alignment, and pending DESIGN.md change decision. Artifacts reviewed: `DESIGN.md` (current state after change was applied), `tests/layer1.rs`, QE Review 2, Data Engineer Review 2.

**Session note:** In-session. Acknowledged quality tradeoff. This is not the cold-session review required at the merge gate — that remains open.

**Adversarial posture (review-session primer applied):** SO did not participate in writing the spec, the tests, or the prior domain reviews. DESIGN.md and the proposed change are read fresh. A change to DESIGN.md was applied before this review ran — SO's obligation is to evaluate it independently and approve or revert.

### Compliance Table

| Layer 1 acceptance criterion | Covered by Red Gate test | Notes |
|---|---|---|
| `tracker create "Fix bug"` exits 0, prints confirmation | `create_valid_title_exits_zero_and_prints_confirmation` | ✓ |
| Empty title exits 1, stderr | `create_empty_title_exits_one_with_error_on_stderr` | ✓ |
| Whitespace-only title exits 1 | `create_whitespace_title_exits_one` | ✓ |
| Title is trimmed before storage | `create_trims_title` | ✓ |
| Issue stored in tracker.json | `create_stores_issue_in_json` | ✓ |
| Second create gets id=2 | `create_second_issue_gets_id_2` | ✓ |
| First issue unchanged after second create | `create_first_issue_unchanged_after_second_create` | ✓ |
| `created_at == updated_at` on fresh issue | `create_timestamps_equal_on_fresh_issue` | ✓ |
| `tracker list` with no file shows empty state | `list_with_no_json_shows_empty_state` | ✓ |
| List shows header and issues | `list_shows_header_and_issues` | ✓ |
| List shows created issue | `list_after_create_shows_issue` | ✓ |
| Title truncated at 50 chars with `…` | `list_truncates_title_at_50_chars_with_ellipsis` | ✓ |
| Malformed JSON causes error exit | `malformed_json_causes_error_exit` | ✓ |

---

### Escalations received

- **QE Review 2, Finding 1** — Integration tests assume a top-level JSON array; DESIGN.md specifies `{"issues": [...]}`. Two resolution options proposed: fix tests or simplify spec. DESIGN.md change authority deferred to SO.
- **Data Engineer Review 2, Finding 1** — Same finding from the schema layer; DE recommends the top-level array as the simpler and more idiomatic Rust representation.

---

### Adversarial evaluation of the applied DESIGN.md change

The change was applied before this review ran. SO reads it cold and evaluates whether it stands.

**Question 1 — Does the assignment constrain the storage file format?**

The assignment requires a local JSON file named `tracker.json`. It does not specify the internal JSON structure. Both a wrapped object and a top-level array satisfy the assignment. The format is within SO authority to specify.

**Question 2 — Was the wrapped format `{"issues": [...]}` justified by something that still holds?**

The original storage spec had two top-level keys: `"issues": [...]` and `"next_id": n`. A wrapper object made sense as the container for two peers. SA Review 1 (Finding 3) removed `"next_id"` as unnecessary complexity for a single-user tool computing `max(ids) + 1` at create time. After that removal, the wrapper object contains a single key. It adds a JSON nesting level and a string key that carry no information. The original justification does not hold.

**Question 3 — Does DESIGN.md contain stale `"issues"` JSON key references?**

Searched DESIGN.md for all occurrences of the word `issues`. Every occurrence is generic English prose referencing the tracked work items ("no two issues share the same ID", "no issues match the given filters"). No occurrence references the removed JSON key. The change is complete.

**Question 4 — Does the top-level array introduce any risks not present in the wrapped format?**

One concern: a top-level array cannot accommodate future top-level metadata keys (e.g., a schema version field) without a breaking format change; a wrapped object can. Assessed: (a) no metadata key is in scope; (b) no version migration is planned; (c) the assignment is a single-user personal tool with no versioning requirement; (d) speculating a future metadata need to justify present complexity is the same reasoning SA Review 1 rejected when it removed `"next_id"`. The concern does not justify retaining the wrapper.

**Verdict: APPROVE.** The change is assignment-compliant, correctly motivated by the removal of `"next_id"` in SA Review 1, complete (no stale references), and produces a simpler spec and a simpler implementation.

---

### Resolved

**Finding 1 — Storage format: wrapped object vs. top-level array (Dim 9 / SO change authority)**

**Received from:** QE Review 2 Finding 1, Data Engineer Review 2 Finding 1.

Following independent adversarial evaluation above: the `{"issues": [...]}` wrapper lost its justification when SA Review 1 removed `"next_id"`. The top-level array is correct, assignment-compliant, and simpler.

**Resolution approved:** DESIGN.md storage format stands as `[Issue]` (top-level array). Empty-tracker invariant reads "empty array." No stale references remain. The integration tests in `tests/layer1.rs` are correct as written. No test changes required.

---

### Dismissed

**Finding 2 — No test for unknown subcommand (Dim 1)**

Layer 7 scope. `unknown_subcommand_exits_one` is documented in TODO.md Layer 7.

**Classification:** Dismissed. Correctly deferred.

---

**Finding 3 — No test for `--help` in Layer 1 (Dim 1)**

Layer 7 scope.

**Classification:** Dismissed. Correctly deferred.

---

### Process finding — raised to VDD-IAR

**Process violation: DESIGN.md was changed before this SO review ran.**

The correct sequence is: domain raises finding → SO evaluates → SO applies or rejects the change. The actual sequence was: domain raises finding → DESIGN.md changed → SO evaluates after the fact. The change happened to be correct and is approved, but the authority chain was inverted. This is a dim 8 (role integrity) concern. Raised to VDD-IAR Alignment for assessment. The change itself stands; the process must not recur.

---

### Open

*(none)*

---

### Summary

DESIGN.md change independently evaluated and approved. The `{"issues": [...]}` wrapper lost its justification when SA Review 1 removed `"next_id"`; the top-level array is the correct simplification. No stale references. All 13 Red Gate tests are in-scope for Layer 1. One process finding raised to VDD-IAR: DESIGN.md was changed before SO review ran.

**Cold-session gate still open:** At least one cold-session domain review (QE or Security) required before Layer 1 implementation code merges.

---

---

## Review 6 — 2026-04-27 20:00Z

**Scope:** Full DESIGN.md cold-session pass — assignment compliance (dim 9), scope coverage (dim 1), internal consistency (dim 7), scope creep (dim 2). No implementation exists; review is spec-only.

**Session note:** Cold session. No participation in spec authorship or prior reviews. This is the review recommended in the open items from Reviews 1–5.

### Compliance Table

*(Full table above under scope; addendum findings below)*

### Findings

---

**Finding 1 — `tracker delete <id>` "with confirmation" in assignment Layer 6 not addressed in DESIGN.md (Dim 9)**

The assignment's Layer 6 build description states: "`tracker delete <id>` with confirmation." DESIGN.md Feature 5 defines delete as non-interactive: the command runs, prints `Deleted issue #<id>.`, and exits. The Out of Scope section explicitly rules out interactive mode: "the tool is non-interactive; it reads arguments from the command line and exits."

This is a genuine tension: the assignment's build layer guidance includes a confirmation prompt; DESIGN.md explicitly disallows interactive behavior without documenting that it is overriding that signal.

Prior SO reviews 1, 3, 4, 5 did not flag or dismiss this. The finding was not hallucinated by prior reviews — it was simply not raised.

Mitigation context: The assignment describes build layers as "one way to layer the build" (explicitly advisory), and the formal interface section — "`tracker delete <id>` (remove an issue)" — carries no mention of confirmation. The design choice to make the tool non-interactive is defensible. But the overriding rationale must be documented, not silently assumed.

**Classification:** Dismissed — the Layer 6 build layers are explicitly framed as advisory ("here's one way to layer the build"), not a formal interface spec. The authoritative interface section lists `tracker delete <id>` with no confirmation signal. The non-interactive design choice is consistent with CLI tool conventions (no command in the assignment's interface list prompts for input). **Rationale added to the Out of Scope "Interactive mode" bullet to document the Layer 6 signal and why it was not adopted.**

---

**Finding 2 — Labels column width: spec text (20 chars) and example table (14 chars) are inconsistent (Dim 7)**

The Interface section states: "Column widths are fixed minimums: `Labels` 20 chars." The example table in the same section shows:

```
ID   Status       Priority  Labels        Title
1    open         high      bug, auth     Fix the login bug
```

Counting the Labels column from the example header: `Labels` (6) + 8 spaces = 14 characters before `Title` begins. The data rows confirm: `bug, auth     ` = 9 + 5 spaces = 14 characters. The example and the spec text specify different widths.

An implementer reading the spec text will produce a 20-char Labels column. An implementer writing tests against the example output will expect 14 chars. These will disagree.

This finding was introduced by the column-width resolution in Review 1 Finding 3, which originally specified 30 chars and was later changed. The example was not updated to match.

**Classification:** Resolved — updated the example table to match the specified 20-char Labels column. See spec update below.

---

**Finding 3 — Empty description rejection not explicitly required by assignment (Dim 9 — marginal)**

The assignment's security habit check states: "Validate all input from the command line. Reject empty titles." Titles are called out by name; descriptions are not. DESIGN.md (Review 5 Finding 6) added: `--description ""` → `Error: Description cannot be empty.` → exit 1.

This is a defensible extension of the assignment's general input-validation principle and consistent with how the spec handles empty labels. However, the assignment names only titles explicitly, and the Finding 6 addition went beyond the literal assignment text without documenting that it was an interpretive extension rather than an explicit assignment requirement.

**Classification:** Dismissed — the assignment's security guidance is general ("Validate all input from the command line") and applying it to description is a straightforward extension of the named principle. The spec is more complete for having it. The behavior is consistent with empty-label and empty-title handling. No action required beyond noting it as an interpretive decision.

---

### Summary

Two real findings. One resolved (column width inconsistency — spec update required). One dismissed with documented rationale (delete confirmation). One dismissed as within reasonable interpretive scope (empty description validation).

All three prior in-session concerns (dim 9 slippage from spec authorship context) are addressed: the only genuine assignment-compliance gap (delete confirmation signal) is documented and dismissed with rationale. No scope creep found beyond what prior reviews addressed. No under-delivery.

**Spec is ready for Layer 1 to open.**
