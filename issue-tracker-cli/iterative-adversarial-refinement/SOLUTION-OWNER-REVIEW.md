# Solution Owner Review Log

This review is part of the [Iterative Adversarial Refinement (IAR)](README.md) suite. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

**Reviewer role: Solution Owner** (Solution Owner / Product Owner / Product Manager)

The purpose of this review is to guard the project against scope creep and over-engineering. DESIGN.md is a Scope of Work. The SO review holds the implementation to that contract: 100% of what was agreed, nothing that was not. DESIGN.md is the contract for every other domain review — SO must confirm it is faithful to the assignment before other domains evaluate the implementation against it.

**Language supplement applied:** Not applicable. The SO review evaluates spec compliance, which is language-agnostic. No supplement section exists for SO. For evaluating whether technology choices are appropriate to the language (dim 3 — Technology compliance), consult the Solution Architect section of the relevant lang supplement (`lang/rust.md`) — SA evaluates technology fitness from an architectural lens that informs SO's technology compliance check.

**Sycophancy check:** An agent that participated in scoping and speccing the project will not flag scope creep it introduced. If the AI helped write or refine DESIGN.md, it treats every element of DESIGN.md as intentional — because it intended it. The adversary must evaluate DESIGN.md against the upstream assignment brief (dim 9) without treating DESIGN.md as authoritative. A spec that was scope-crept during Phase 1 will produce a project that passes every other SO dimension and still fails an external review.

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

**Finding 1 — Multi-line description display behavior undefined (Dim 1)**

`DESIGN.md` stored description "verbatim" but the `show` output format was a single-line key-value block. A description containing `\n` would corrupt the display. Spec was silent on this case.

**Resolution:** Defined display convention: first line follows the `Description:` label, continuation lines indented 13 spaces to align with the value column. `\r\n` normalized to `\n` on storage.

---

**Finding 2 — `tracker list --label` with multiple flags: behavior undefined (Dim 1)**

Feature 2 stated "only one `--label` filter is supported" but did not define what happens when the user passes two `--label` flags. A reviewer and an implementer would produce different behavior (last wins vs. clap error vs. AND-combine).

**Resolution:** Defined: clap rejects multiple `--label` flags on `list` with a usage error. Documented in Feature 2 and the Labels edge case section.

---

**Finding 3 — List column width contract incomplete (Dim 1)**

The list output showed a fixed-width table example but gave no specification of how column widths are determined or what happens to long values. An automated test for `format_issue_row` would have no defined behavior to assert against.

**Resolution:** Defined dynamic-width columns with truncation contracts: title truncates at 50 chars with `…`, labels truncate at 30 chars with `…`. `show` always displays full values.

---

**Finding 4 — Empty state message deviated from assignment (Dim 9)**

Assignment Layer 7 names the exact string `"No open issues. Nice work!"`. DESIGN.md used `"No open issues."` with no note that the wording was intentionally changed.

**Resolution:** Updated to match assignment wording: `"No open issues. Nice work!"`.

---

**Finding 5 — Feature 5 ("Add labels to issues") label post-creation path undocumented (Dim 1, Dim 9)**

The Out of Scope section excluded "editing after creation" as a blanket rule, but did not explicitly address whether creation-only labels satisfies the assignment's Feature 5. A reviewer could flag this as under-delivery.

**Resolution:** Added explicit note to the Out of Scope bullet explaining that the assignment's Feature 5 is satisfied by creation-time labels, consistent with the assignment's own interface examples.

---

**Finding 6 — Label length unconstrained (Dim 1)**

The edge case catalog covered empty labels but not maximum length. The label column in list output has a truncation limit; labels should have a corresponding input constraint so the contract is complete.

**Resolution:** Added 50-character maximum label length with error message `Error: Label must be 50 characters or fewer (got <n>).`. Added to Feature 1 preconditions, error states, data model, and edge cases.

---

### Dismissed

*(none)*

### Backlogged

*(none)*

### Hallucinated

*(none)*

---

### Summary

Six real findings, all resolved via DESIGN.md edits: multi-line description display, multiple-`--label` behavior, list column width contract, empty-state message wording, label post-creation note, and label length cap. No dismissed or backlogged items. Cold-session pass remains warranted before Layer 1 opens.

**Coordination:** *(none)*

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

**Finding 1 — Color output excluded but is named in assignment Layer 7 (Dim 9)**

Assignment Layer 7 lists "colored output" alongside `--help` and empty-state messages, both of which were already in scope. Out of Scope said "terminal color is a polish-layer concern and is not part of the core contract."

**Resolution:** Removed from Out of Scope. Added color output spec to the Interface section: priority and status values are colored in list and show output when stdout is a TTY; color suppressed when piped. Defined color scheme: `high`=red/bold, `medium`=yellow, `low`=default; `open`=default, `in-progress`=cyan, `done`=green.

---

**Finding 2 — `clap` and `serde_json` named as spec constraints (Dim 3)**

The assignment requires Rust and a local JSON file. Naming specific crates locks the implementation to those choices at spec level.

**Resolution:** Replaced with: "CLI argument parsing: any Rust crate or standard library. JSON serialization: any Rust crate. The observable interface contract is defined by this spec regardless of which libraries implement it."

---

**Finding 3 — Title 200-char and label 50-char limits not in the assignment (Dim 2, Dim 4)**

Both limits were added by the spec author to close boundary gaps but create test obligations not required by the assignment.

**Resolution:** Removed both character limits from Feature 1 preconditions, error states, data model, and edge cases. Titles and labels are validated as non-empty only, per the assignment.

---

**Finding 4 — "No panics" constraint broader than assignment guidance (Dim 4, Dim 9)**

Assignment says: handle missing/corrupt JSON without crashing. DESIGN.md had a blanket "no `unwrap()` or `expect()` in production code" rule.

**Resolution:** Replaced with a constraint scoped to the assignment's guidance: "The binary must not crash when `tracker.json` is missing or contains invalid data." Blanket `unwrap()` prohibition removed.

---

**Finding 5 — "No warnings" requirement not in assignment (Dim 9)**

Assignment requires compilation with no errors. Warnings are normal during iterative development.

**Resolution:** Changed to "Compilation must succeed with `cargo build` with no errors at the end of each layer."

---

### Dismissed

*(none)*

### Backlogged

*(none)*

### Hallucinated

*(none)*

---

### Summary

Five real findings, all resolved via DESIGN.md edits. Color output added to spec; named crates removed from Technology section; over-specified character limits removed; "no panics" scoped to assignment guidance; "no warnings" replaced with the assignment's actual no-errors requirement. No dismissed or backlogged items.

**Coordination:** *(none)*

---

---

## Review 4 — 2026-04-27 18:00Z

**Scope:** DESIGN.md Testing Methodology section — does it fit or overshoot the assignment?

**Session note:** In-session. Acknowledged quality tradeoff.

### Resolved

**Finding 1 — Purity boundary section presupposes implementation structure and imports Phase 5 language (Dim 4, Dim 9)**

The purity boundary section named specific function names (`validate_title`, `issue_matches_filters`, `format_issue_row`, `load_store`, etc.) locking the implementation to a particular module structure before any code exists. It also used the phrase "formally verifiable in principle" — VSDD Phase 5 language inappropriate for a Phase 1 first Rust project.

**Resolution:** Removed the purity boundary section. Replaced with a single-paragraph "Purity guidance" note: prefer separating validation, filtering, and sorting logic from I/O; this is a code organization principle, not a formal requirement. No function names named.

---

**Finding 2 — Display formatting and label deduplication named as standalone unit test items (Dim 4)**

Both are internal implementation behaviors covered by the integration tests. A `tracker list` invocation that produces wrong output is already a failing integration test; a separate unit test for `format_issue_row` tests implementation internals, not assignment-defined behavior. Label deduplication is equally covered by the full lifecycle integration test.

**Resolution:** Removed "label deduplication logic" and "display formatting" from the automated tests list.

---

**Finding 3 — ID assignment listed as a named unit test item (Dim 4)**

`max(existing_ids) + 1` is two lines of logic covered by the create → delete → create lifecycle integration test. Naming it as a spec-required unit test added test obligations for internal data plumbing.

**Resolution:** Removed "ID assignment" from the automated tests list.

---

### Dismissed

*(none)*

### Backlogged

*(none)*

### Hallucinated

*(none)*

---

### Summary

Three real findings, all resolved via DESIGN.md edits to the Testing Methodology section: removed purity boundary presuppositions and Phase 5 language; removed display formatting and label deduplication from named unit tests; removed ID assignment from named unit tests. No dismissed or backlogged items.

**Coordination:** *(none)*

---

---

## Review 5 — 2026-04-27 19:00Z

**Scope:** Full clean-state pass of DESIGN.md against the assignment.

**Session note:** In-session. Acknowledged quality tradeoff.

### Resolved

**Finding 1 — Title 200-char limit survived Review 3 (Dim 7)**

The precondition `<title> after trimming is ≤ 200 characters` was not removed when Review 3 removed the corresponding error state and edge cases. The spec had a precondition with no enforced consequence — undefined behavior for titles over 200 chars.

**Resolution:** Removed the precondition. Titles are validated as non-empty only.

---

**Finding 2 — `clap` named in the `--help` flag description (Dim 3, Dim 7)**

`clap generates --help for the binary...` re-introduced a named crate dependency after Review 3 removed them from the Technology line.

**Resolution:** Replaced with: "`--help` is supported for the binary and each subcommand."

---

**Finding 3 — `clap`-specific error message quoted in Feature 2 (Dim 3)**

Feature 2 quoted the exact clap error string for multiple `--label` flags, locking the spec to one library's error format.

**Resolution:** Replaced with: "a usage error is produced on stderr and the command exits 1."

---

**Finding 4 — `atty` named in color output spec (Dim 3)**

`atty` is a third-party crate. Review 3 removed named crates from the Technology section; this reference was missed.

**Resolution:** Removed `atty`. Retained `std::io::IsTerminal` (standard library, Rust 1.70+).

---

**Finding 5 — "All pure functions are unit tested" orphaned text (Dim 7)**

Review 4 removed the purity boundary section; this opening sentence referencing "pure functions" was left behind, making an unsupported claim.

**Resolution:** Replaced with: "The following behaviors are automatable and should be covered by unit tests:"

---

**Finding 6 — `--description ""` silently coerced to absent (Dim 7)**

The assignment says "optional description." The spec treated empty-string description as equivalent to not providing the flag — a non-obvious silent coercion with no error feedback to the user.

**Resolution:** Changed to validation consistent with the rest of the boundary: `--description ""` (empty or whitespace-only after trim) → `Error: Description cannot be empty.` → exit 1. Error state added to Feature 1. Description edge case updated.

---

### Dismissed

*(none)*

### Backlogged

*(none)*

### Hallucinated

*(none)*

---

### Summary

Six real findings, all resolved via DESIGN.md edits: removed surviving 200-char title precondition; replaced two named-crate references (`clap`, `atty`); replaced a clap-specific quoted error string with a generic usage-error description; cleaned up orphaned "pure functions" text; closed the silent empty-description coercion with explicit validation. No dismissed or backlogged items.

**Coordination:** Cold-session SO review recommended before Layer 1 opens (completed in Review 6). SA Review 1 findings resolved; SA Review 2 also complete — see [SOLUTION-ARCHITECT-REVIEW.md](SOLUTION-ARCHITECT-REVIEW.md). VDD-IAR Alignment Reviews 1 and 2 complete — see [VDD-IAR-ALIGNMENT-REVIEW.md](VDD-IAR-ALIGNMENT-REVIEW.md). Layer 1 merge gate: at least one cold-session domain review (QE or Security) required after Layer 1 implementation — see VDD-IAR Finding 2.

---

---

## Review 7 — 2026-04-27 22:00Z

**Scope:** Layer 1 Red Gate tests — scope compliance, spec alignment, and pending DESIGN.md change decision. Artifacts reviewed: `DESIGN.md` (current state after change was applied), `tests/layer1.rs`, [QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md) Review 2, [DATA-ENGINEER-REVIEW.md](DATA-ENGINEER-REVIEW.md) Review 2. SO did not participate in writing the spec, the tests, or the prior domain reviews — review-session primer applied for fresh adversarial posture. A change to DESIGN.md was applied before this review ran; SO's obligation is to evaluate it independently and approve or revert.

**Session note:** In-session. Acknowledged quality tradeoff. This is not the cold-session review required at the merge gate — that remains open.

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

### Backlogged

*(none)*

### Hallucinated

*(none)*

---

### Open

*(none)*

---

### Summary

DESIGN.md change independently evaluated and approved. The `{"issues": [...]}` wrapper lost its justification when SA Review 1 removed `"next_id"`; the top-level array is the correct simplification. No stale references. All 13 Red Gate tests are in-scope for Layer 1. One process finding raised to VDD-IAR: DESIGN.md was changed before SO review ran. The correct sequence is: domain raises finding → SO evaluates → SO applies or rejects the change. The actual sequence was: domain raises finding → DESIGN.md changed → SO evaluates after the fact. The change happened to be correct and is approved, but the authority chain was inverted (a dim 8 / role-integrity concern in [VDD-IAR-ALIGNMENT-REVIEW.md](VDD-IAR-ALIGNMENT-REVIEW.md)). The change itself stands; the process must not recur.

Cold-session gate still open: at least one cold-session domain review (QE or Security) required before Layer 1 implementation code merges.

**Coordination:** Process finding raised to [VDD-IAR-ALIGNMENT-REVIEW.md](VDD-IAR-ALIGNMENT-REVIEW.md) for role-integrity assessment.

---

---

## Review 6 — 2026-04-27 20:00Z

**Scope:** Full DESIGN.md cold-session pass — assignment compliance (dim 9), scope coverage (dim 1), internal consistency (dim 7), scope creep (dim 2). No implementation exists; review is spec-only.

**Session note:** Cold session. No participation in spec authorship or prior reviews. This is the review recommended in the open items from Reviews 1–5.

### Compliance Table

*(Full table above under Review 1 scope; addendum findings below.)*

---

### Resolved

**Finding 2 — Labels column width: spec text (20 chars) and example table (14 chars) are inconsistent (Dim 7)**

The Interface section states: "Column widths are fixed minimums: `Labels` 20 chars." The example table in the same section shows:

```
ID   Status       Priority  Labels        Title
1    open         high      bug, auth     Fix the login bug
```

Counting the Labels column from the example header: `Labels` (6) + 8 spaces = 14 characters before `Title` begins. The data rows confirm: `bug, auth     ` = 9 + 5 spaces = 14 characters. The example and the spec text specify different widths.

An implementer reading the spec text will produce a 20-char Labels column. An implementer writing tests against the example output will expect 14 chars. These will disagree.

This finding was introduced by the column-width resolution in Review 1 Finding 3, which originally specified 30 chars and was later changed. The example was not updated to match.

**Resolution:** Updated the example table to match the specified 20-char Labels column.

---

### Dismissed

**Finding 1 — `tracker delete <id>` "with confirmation" in assignment Layer 6 not addressed in DESIGN.md (Dim 9)**

The assignment's Layer 6 build description states: "`tracker delete <id>` with confirmation." DESIGN.md Feature 5 defines delete as non-interactive: the command runs, prints `Deleted issue #<id>.`, and exits. The Out of Scope section explicitly rules out interactive mode: "the tool is non-interactive; it reads arguments from the command line and exits."

This is a genuine tension: the assignment's build layer guidance includes a confirmation prompt; DESIGN.md explicitly disallows interactive behavior without documenting that it is overriding that signal.

Prior SO reviews 1, 3, 4, 5 did not flag or dismiss this. The finding was not hallucinated by prior reviews — it was simply not raised.

Mitigation context: The assignment describes build layers as "one way to layer the build" (explicitly advisory), and the formal interface section — "`tracker delete <id>` (remove an issue)" — carries no mention of confirmation. The design choice to make the tool non-interactive is defensible. But the overriding rationale must be documented, not silently assumed.

**Classification:** Dismissed. The Layer 6 build layers are explicitly framed as advisory ("here's one way to layer the build"), not a formal interface spec. The authoritative interface section lists `tracker delete <id>` with no confirmation signal. The non-interactive design choice is consistent with CLI tool conventions (no command in the assignment's interface list prompts for input). Rationale added to the Out of Scope "Interactive mode" bullet to document the Layer 6 signal and why it was not adopted.

---

**Finding 3 — Empty description rejection not explicitly required by assignment (Dim 9)**

The assignment's security habit check states: "Validate all input from the command line. Reject empty titles." Titles are called out by name; descriptions are not. DESIGN.md (Review 5 Finding 6) added: `--description ""` → `Error: Description cannot be empty.` → exit 1.

This is a defensible extension of the assignment's general input-validation principle and consistent with how the spec handles empty labels. However, the assignment names only titles explicitly, and the Finding 6 addition went beyond the literal assignment text without documenting that it was an interpretive extension rather than an explicit assignment requirement.

**Classification:** Dismissed. The assignment's security guidance is general ("Validate all input from the command line") and applying it to description is a straightforward extension of the named principle. The spec is more complete for having it. The behavior is consistent with empty-label and empty-title handling. No action required beyond noting it as an interpretive decision.

---

### Backlogged

*(none)*

### Hallucinated

*(none)*

---

### Open

*(none)*

---

### Summary

Two real findings. One resolved (column width inconsistency — spec update applied). Two dismissed with documented rationale (delete confirmation; empty-description interpretive extension). All three prior in-session concerns (dim 9 slippage from spec authorship context) are addressed: the only genuine assignment-compliance gap (delete confirmation signal) is documented and dismissed with rationale. No scope creep found beyond what prior reviews addressed. No under-delivery. Spec is ready for Layer 1 to open.

**Coordination:** *(none)*

---

---

## Review 8 — 2026-04-28 05:30Z

**Scope:** Layer 1 implementation — spec compliance audit. Verifying the implementation satisfies all Layer 1 acceptance criteria, no scope creep, and documentation is current.

**Session note:** In-session with Layer 1 IAR suite. Acknowledged quality tradeoff.

---

### Resolved

**Finding 1 — README.md status block was stale (Dim 9 — Scope/documentation accuracy)**

README.md showed Layer 1 implementation as unchecked (`- [ ] Layer 1: Core create + list`) after Layer 1 implementation was complete. Also the status line read "Spec complete. Implementation in progress." which was accurate during Layer 1 implementation but should be updated to reflect Layer 1 being complete.

**Resolution:** Updated README.md:
- `- [ ] Layer 1: Core create + list` → `- [x] Layer 1: Core create + list`
- Status line updated to: `Layer 1 implementation complete. Layer 2 not started.`

---

**Finding 2 — DECISIONS.md missing storage format decision (Dim 9 — Decision documentation)**

DECISIONS.md was created in TW Review 1 with spec-phase decisions. The storage format change (top-level array vs. wrapped object) — a significant SO Review 7 decision — was not recorded. A reader of DECISIONS.md would not find the rationale for why `tracker.json` is a top-level array.

**Resolution:** Added entry to DECISIONS.md: "Top-level JSON array storage format" with rationale referencing SO Review 7, the removal of `next_id` from SA Review 1, and the simplicity argument.

---

### Dismissed

**Finding 3 — Post-deserialization validation was absent (Dim 2 — Spec compliance)**

DESIGN.md Storage edge cases explicitly required that invalid domain values in valid JSON trigger the corrupt-data error. The initial Layer 1 implementation did not implement this. Security Review 3 identified and resolved the gap. This is the correct process: domain identifies → escalation → resolution.

**Classification:** Dismissed. The gap is resolved. SO's role here is to confirm the resolution is spec-compliant — it is. The error message and behavior match DESIGN.md exactly.

---

**Finding 4 — Layer 1 scope: no features beyond `create` and `list` (Dim 1)**

Verified: `main.rs` defines only `Create` and `List` subcommands. No `status`, `show`, or `delete` subcommands exist. `lib.rs` implements only `cmd_create` and `cmd_list`. No `--priority`, `--label`, or `--description` flags are wired in Layer 1. The scope matches TODO.md Layer 1's "Not in this layer" constraint.

**Classification:** Dismissed. Scope compliance confirmed.

---

**Finding 5 — CHANGELOG.md missing Layer 1 implementation entry (Dim 9 — Documentation currency)**

Observed and resolved by TW Review 2 in coordination with this review. CHANGELOG.md was updated with a Layer 1 implementation entry before this finding was classified.

**Classification:** Dismissed. Resolved by TW Review 2.

---

### Open

*(none)*

---

### Backlogged

*(none)*

### Hallucinated

*(none)*

---

### Summary

Two real findings resolved: README.md Layer 1 status stale (fixed), DECISIONS.md missing storage format decision (added). Three dismissed. No open items. The implementation is spec-compliant, scope-correct, and documentation is now current.

**Coordination:** Finding 5 (CHANGELOG Layer 1 entry) resolved by [TECHNICAL-WRITER-REVIEW.md](TECHNICAL-WRITER-REVIEW.md) Review 2. Finding 3 (post-deser validation) resolved by [SECURITY-REVIEW.md](SECURITY-REVIEW.md) Review 3.

---

---

## Review 9 — 2026-04-30 00:00Z

**Scope:** Layer 1 gate closure pass — no spec or documentation changes since Review 8 other than gate closure records (TODO.md manual checklist, IAR log updates).

**Session note:** In-session with all other domain reviews. Acknowledged quality tradeoff.

---

### Dismissed

*(none)*

### Backlogged

*(none)*

### Hallucinated

*(none)*

---

### Open

*(none)*

---

### Summary

No SO findings. MVR reached for Layer 1. Layer 1 scope compliance verified — `tracker create` and `tracker list` only; no Layer 2+ features introduced; scope discipline maintained. Documentation currency verified — README, CHANGELOG, TODO all current; manual testing checklist complete; DECISIONS.md complete.

**Coordination:** *(none)*

---

---

## Review 10 — 2026-05-01 00:00Z

**Scope:** Layer 2 implementation — spec compliance audit. Artifacts reviewed: `DESIGN.md`, `src/lib.rs`, `src/main.rs`, `tests/layer2.rs`, `TODO.md`, `README.md`, `CHANGELOG.md`. SO did not build Layer 2; primary obligation is to DESIGN.md, not the implementation. Every finding is evaluated against the spec contract.

**Session note:** In-session with full Layer 2 IAR suite. Same model as builder. Acknowledged quality tradeoff. Review-session primer applied.

### Compliance Table

| Layer 2 acceptance criterion | Covered in implementation | Notes |
|---|---|---|
| `tracker status 1 in-progress` exits 0, prints confirmation | ✓ `cmd_status` + test | stdout = `Issue #1 status → in-progress.\n` |
| `tracker status 1 done` exits 0 | ✓ | |
| `tracker.json` updated: status + updated_at; all other fields unchanged | ✓ tests + code | `cmd_status` modifies only `status` and `updated_at` |
| `updated_at` after change >= before | ✓ | 1-second sleep in test |
| `tracker list` default shows only open | ✓ | `status_filter=None` → "open" |
| `tracker list --status done` shows done | ✓ | |
| `tracker list --status in-progress` shows in-progress | ✓ | |
| `tracker list --status open` == default | ✓ | `is_open_view = effective_status == "open"` |
| `tracker status 1 IN-PROGRESS` (uppercase) → stored lowercase | ✓ | `parse_status` normalizes |
| Idempotent: same status → exits 0, refreshes updated_at | ✓ | no guard against no-op |
| Invalid ID string → exit 1, stderr | ✓ | `parse_id` |
| Zero ID → exit 1, stderr | ✓ | `filter(|&n| n > 0)` |
| Not found → exit 1, stderr | ✓ | `ok_or_else` |
| Invalid status value → exit 1, stderr | ✓ | `parse_status` |
| `tracker list --status flying` → exit 1, stderr | ✓ | `cmd_list` calls `parse_status` |
| All issues done, `tracker list` (default) → "No open issues. Nice work!" | ✓ | `is_open_view` guard |
| Layer 2 scope: no `--priority`, no `--label`, no show/delete | ✓ | |

### Resolved

**Finding 1 — CHANGELOG.md missing Layer 2 entry (Dim 9 — Documentation currency)**

CHANGELOG.md documents the spec phase, Layer 1 implementation, and Layer 1 gate closure. No entry covers Layer 2. Layer 2 features (`tracker status`, `--status` filter), the 17-test suite, and the two extra tests beyond the Red Gate plan are undocumented in the CHANGELOG. A reader of CHANGELOG.md cannot determine what Layer 2 delivered.

**Resolution:** Added Layer 2 entry to CHANGELOG.md. See below.

---

**Finding 2 — README.md status block is stale (Dim 9 — Documentation accuracy)**

README.md Status section shows `- [ ] Layer 2: Status flow` (unchecked) and the status line reads "Layer 1 implementation complete. Layer 2 not started." Layer 2 is implemented, tested, and manually verified.

**Resolution:** Updated README.md:
- `- [ ] Layer 2: Status flow` → `- [x] Layer 2: Status flow`
- Status line updated to: `Layer 2 implementation complete. Layer 3 not started.`

---

### Dismissed

**Finding 3 — `tracker list --status open` empty state uses "Nice work!" message (Dim 7 — Internal consistency)**

`is_open_view = effective_status == "open"` means `--status open` shows "No open issues. Nice work!" when empty, identical to the no-flag default. The acceptance criterion explicitly requires this: "`tracker list --status open` behaves identically to `tracker list` (explicit `open` flag matches default)." The implementation is spec-compliant.

**Classification:** Dismissed. Spec and acceptance criterion align. No action.

---

### Open

*(none)*

---

### Backlogged

*(none)*

### Hallucinated

*(none)*

---

### Summary

Two real findings resolved: CHANGELOG missing Layer 2 entry (added), README status stale (updated). One dismissed (open-status empty-state message — spec-compliant). All Layer 2 acceptance criteria are met. No scope creep. Layer 2 delivers exactly what the spec requires: status mutation, status-based list filtering, error handling for invalid IDs and status values.

**Coordination:** *(none)*

---

---

## Review 11 — 2026-05-04 05:40Z

**Scope:** Layer 3 implementation — spec compliance audit. Artifacts reviewed: `DESIGN.md`, `src/lib.rs`, `src/main.rs`, `tests/layer3.rs`, `Cargo.toml`, `TODO.md`, `README.md`, `CHANGELOG.md`. SO did not build Layer 3; primary obligation is to DESIGN.md, not the implementation.

**Session note:** Cold-session adversarial review using `iterative-adversarial-refinement/prompts/review-session.md` primer. Reviewer did not participate in Layer 3 build. Round 1.

### Compliance Table

| Layer 3 acceptance criterion | Covered in implementation | Notes |
|---|---|---|
| `tracker create "X" --priority high` stores `"priority": "high"` | ✓ `cmd_create` + `parse_priority` + test | |
| `tracker create "X"` (no flag) stores `"priority": "medium"` | ✓ default branch in `cmd_create` | |
| `tracker create "X" --priority HIGH` (uppercase) stores `"high"` | ✓ `parse_priority` lowercases via `to_lowercase()` | |
| `tracker create "X" --priority critical` → exit 1, stderr `Error: Invalid priority 'critical'. Expected: low, medium, or high.` | ✓ `parse_priority` + test | |
| `tracker list` sorts high → medium → low | ✓ `sort_issues` + `priority_rank` + test | |
| Within tier, sort by ID ascending | ✓ `.then(a.id.cmp(&b.id))` + test | |
| `tracker list --priority high` shows only high | ✓ second `retain` in `cmd_list` + test | |
| `tracker list --priority medium` shows only medium | ✓ same retain logic | covered by `parse_priority` symmetry |
| `tracker list --priority low` shows only low | ✓ same retain logic | covered by `parse_priority` symmetry |
| `tracker list --priority invalid` → exit 1, stderr | ✓ `parse_priority` early return + test | |
| `tracker list --status open --priority high` AND-combines | ✓ two sequential `retain` calls in `cmd_list` | full compound-filter verification deferred to Layer 5 per TODO.md |
| Layer 3 scope: no `--label` on create, no `--label` filter, no description/show/delete | ✓ | clap rejects unknown flags |

**DESIGN.md regression sweep:**

| DESIGN.md section | Layer 3 impact | Status |
|---|---|---|
| Feature 1 title trim/empty (Layer 1) | None | preserved |
| Feature 1 default priority `medium` (Layer 1 + Layer 3) | Re-verified: `cmd_create` uses `"medium".to_string()` when flag absent | preserved |
| Feature 2 default open view (Layer 1) | Re-verified: `effective_status` defaults to `"open"` | preserved |
| Feature 2 sort priority desc, ID asc (Layer 1, full algorithm) | `sort_issues` is the implementation; matches | preserved |
| Feature 3 status command (Layer 2) | Untouched | preserved |
| Edge case "Issues exist but none match the filters → No issues match" (line 308) | **Regressed by Layer 3** — see Finding 1 | resolved this review |
| Edge case "No issues in storage → No open issues. Nice work!" (line 307) | Verified post-fix | preserved |
| Edge case "All issues done → No open issues. Nice work!" (line 309) | Verified post-fix (priority=None, status=open) | preserved |

### Resolved

**Finding 1 — `is_open_view` empty-state heuristic does not consider priority filter (Dim 7 — Design fidelity)**

`src/lib.rs:225` — `let is_open_view = effective_status == "open";` evaluates to `true` whenever the status filter is absent or set to `open`, regardless of any other filter. Layer 3 introduced the `--priority` filter without updating this heuristic. Consequence: `tracker list --priority X` (or `tracker list --status open --priority X`) with no matches prints `No open issues. Nice work!` instead of `No issues match the given filters.`, violating DESIGN.md edge case line 308 ("Issues exist but none match the filters → prints `No issues match the given filters.` (exit 0)").

Reproduction (verified pre-fix, on `caf5f9a` Layer 3 implementation):

```
$ tracker create "Fix login" --priority low
Created issue #1: Fix login
$ tracker list --priority high
No open issues. Nice work!     ← spec violation; should be "No issues match the given filters."
```

This is a Layer 3-introduced regression in spec compliance. Layer 1 and Layer 2 had no path to reach this case (no `--priority` flag existed). TODO.md Layer 5 defers full compound-filter no-match verification, but the implementation is observable to a Layer 3 user today and DESIGN.md is the contract regardless of layer.

**Resolution:** Modified `src/lib.rs:225` to:

```rust
let is_open_view = effective_status == "open" && effective_priority.is_none();
```

Manual verification post-fix:
- `tracker list --priority high` (with one low-priority open issue) → `No issues match the given filters.` ✓
- `tracker list` (default, with open issues) → table output ✓
- `tracker list` (default, empty tracker) → `No open issues. Nice work!` ✓
- `cargo test` — all 52 tests pass (11 unit + 16 layer1 + 18 layer2 + 7 layer3) ✓

**Coordination:** Raised to QE — Layer 3 Red Gate test plan does not assert the no-match message for priority-filtered lists. QE should add a regression test (e.g. `list_priority_filter_no_match_shows_filter_message`) to lock in the fix. The `--label` extension of this heuristic is a Layer 4 concern and should be considered when `cmd_list` accepts a label filter.

---

**Finding 2 — CHANGELOG.md missing Layer 3 entry (Dim 9 — Documentation currency)**

`CHANGELOG.md` last entry is `Layer 2 — 2026-05-01 00:00Z`. Layer 3 features (`--priority` flag on `tracker create` and `tracker list`, `parse_priority`, `sort_issues`, `priority_rank` helper, 7 new integration tests, 4 new unit tests) are undocumented. A reader of CHANGELOG.md cannot determine what Layer 3 delivered. Same pattern as Review 10 Finding 1.

**Resolution:** Added Layer 3 entry to CHANGELOG.md documenting Added (parse_priority, priority_rank, sort_issues, cmd_create/cmd_list signature extensions, layer3.rs tests, 4 unit tests), Changed (`is_open_view` empty-state fix per Finding 1), and IAR (SO Review 11).

---

**Finding 3 — README.md status block is stale (Dim 9 — Documentation accuracy)**

`README.md:60` reads `**Layer 2 implementation complete. Layer 3 not started.**`. `README.md:68` shows `- [ ] Layer 3: Priority` (unchecked). Layer 3 is implemented, tested, and manually verified per `TODO.md` and recent commits (`6f7fd46` "Layer 3 manual testing complete", `caf5f9a` "Layer 3 implementation"). Same pattern as Review 10 Finding 2.

**Resolution:** Updated README.md:
- `- [ ] Layer 3: Priority` → `- [x] Layer 3: Priority`
- Status line updated to: `Layer 3 implementation complete. Layer 4 not started.`

---

### Dismissed

*(none)*

### Open

*(none)*

### Backlogged

*(none)*

### Hallucinated

*(none)*

---

### Summary

Three real findings resolved: `is_open_view` regression introduced by Layer 3's `--priority` filter (one-line fix in `cmd_list`); CHANGELOG missing Layer 3 entry (added); README status stale (updated). All 11 Layer 3 acceptance criteria are met. No scope creep — `cmd_create` and `cmd_list` were extended for `--priority` only; no Layer 4+ features (`--label`, `--description`, `show`, `delete`) leaked into Layer 3. Sort algorithm (full priority→ID) was already in place from Layer 1 per SA Review 2; Layer 3 implements `parse_priority`, `sort_issues`, `priority_rank` cleanly and `parse_priority` mirrors `parse_status` (single source of truth via `VALID_PRIORITIES`, mirroring SA Review 6 unification done for status). Round 1 produced one defect-class finding (Finding 1) plus two documentation-currency findings (Findings 2 & 3); a follow-up SO pass is not warranted unless Findings 2/3 recur, but a second cold-session pass after QE adds the regression test is consistent with MVR practice.

**Coordination:**
- **QE:** Add regression test `list_priority_filter_no_match_shows_filter_message` to `tests/layer3.rs` to lock in the Finding 1 fix.
- **VDD-IAR Alignment:** Round 1 of Layer 3 IAR; the Layer 3 implementation merge (`caf5f9a`) preceded any Layer 3 IAR review. Standard "build → IAR → fix → repeat to MVR" loop is now in motion; the `is_open_view` fix is the kind of finding the loop is designed to catch and is not a process violation.

---

---

## Review 12 — 2026-05-04 15:10Z

**Scope:** Layer 3 implementation — second cold-session SO pass (Round 2). Artifacts reviewed: `DESIGN.md` (post-Review-11 state), `src/lib.rs`, `src/main.rs`, `tests/layer1.rs`, `tests/layer2.rs`, `tests/layer3.rs`, `tests/common/mod.rs`, `Cargo.toml`, `rust-toolchain.toml`, `.gitignore`, `tracker.json` (project root), `CHANGELOG.md`, `DECISIONS.md`, `PROCESS.md`, `TODO.md`, `README.md`, prior IAR log (this file). SO did not build Layer 3; primary obligation is to DESIGN.md.

**Session note:** Cold session per primer; parallel batch run with other domains. Reviewer did not participate in Layer 3 build, prior reviews, or any same-session IAR work. Round 2.

### Compliance Table

| DESIGN.md element | Layer 3 status | Notes |
|---|---|---|
| Feature 1 `--priority` (low/medium/high, case-insensitive, default medium, error on invalid) | Met | `cmd_create` + `parse_priority` + tests |
| Feature 1 `--label` | Deferred (Layer 4) | not in this layer's scope |
| Feature 1 `--description` | Deferred (Layer 6) | not in this layer's scope |
| Feature 2 `--priority` filter (AND-combined) | Met | `cmd_list` second `retain` after status filter |
| Feature 2 sort priority desc → ID asc | Met | `sort_issues` + `priority_rank` |
| Feature 2 default empty-state vs filtered empty-state messages | Met (post Review 11 fix) | re-verified `is_default_open_view` evaluates priority filter |
| Feature 3 status command | Met (Layer 2) | regression-checked: untouched |
| Feature 4 / 5 (show, delete) | Deferred | Layer 6 |
| Interface: column widths (ID 4, Status 11, Priority 8, Labels 20) + 2-space separators | Met | `{:<W}  ` format strings; locked by `list_columns_use_exactly_two_space_separator` |
| Interface: title truncate at 50 with `…`; labels truncate at 20 with `…` | Met | `truncate_with_ellipsis` |
| Storage: top-level array, missing→empty, malformed→corrupt error, post-deser validation | Met | `load_issues` + `issue_fields_are_valid` |
| Edge case: ID error message text consistency between Feature 3 (line 98) and Edge Cases (line 291) | **Resolved this review** — Finding 1 |  |

**Regression sweep against prior SO findings:** Review 11 Finding 1 fix (`is_default_open_view` includes `effective_priority.is_none()`) verified at `src/lib.rs:232`; variable was renamed from `is_open_view` to `is_default_open_view` (a Review 11 follow-up that resolves SE Review 8 Finding 2's naming concern as a side effect — confirmed by reading current source). All other prior SO-confirmed compliance points still hold.

### Resolved

**Finding 1 — DESIGN.md ID error message text inconsistent between Feature sections and Edge Cases (Dim 7 — Internal consistency)**

`DESIGN.md` Feature 3 (line 98), Feature 4 (line 120), and Feature 5 (line 142) all specify the error string as:

```
Error: '<id>' is not a valid issue ID. Expected a positive integer.
```

`DESIGN.md` Edge Cases / IDs (line 291) specifies a truncated form:

```
- Non-integer (`tracker show abc`) → error: `'abc' is not a valid issue ID.`
```

The implementation in `parse_id` (`src/lib.rs:142`) and the test assertions (`tests/layer2.rs:288`, `tests/layer2.rs:300`, `tests/layer2.rs:312`) use the longer, Feature-spec form. The truncated edge-case form would also pass the `predicate::str::contains("not a valid issue ID")` assertions, masking the inconsistency under loose matchers. A future stricter test (full-string equality) keyed off the edge-case wording would diverge from the implementation. Prior SO reviews 1, 3, 5, 6 did not surface this; the Edge Cases section was added/edited in early reviews and the wording slipped relative to the Feature sections.

**Resolution (DESIGN.md edit applied this review):** Updated `DESIGN.md:291` to match the Feature sections:

```diff
- - Non-integer (`tracker show abc`) → error: `'abc' is not a valid issue ID.`
+ - Non-integer (`tracker show abc`) → error: `'abc' is not a valid issue ID. Expected a positive integer.`
```

Single-line change; reconciles the spec with the three authoritative Feature sections and with the implementation. No code or test change required.

---

**Finding 2 — CHANGELOG.md test counts are inaccurate (Dim 9 — Documentation accuracy)**

`CHANGELOG.md` Layer 3 entry states: "Total suite: 53 tests (42 integration + 11 unit), all passing." `cargo test` (verified this review) reports 56 tests: 11 unit + 18 (`layer1.rs`) + 18 (`layer2.rs`) + 9 (`layer3.rs`) = 45 integration + 11 unit. The CHANGELOG also says Layer 3 "Added 8 integration tests"; `tests/layer3.rs` has 9 `#[test]` functions. The 9th — `list_columns_use_exactly_two_space_separator` (`tests/layer3.rs:195`) — is a test for the DESIGN.md "exactly 2 spaces" column-separator contract (line 218); it is not enumerated in the Layer 3 CHANGELOG description. Layer 2 entry similarly says "38 tests (34 integration + 4 unit)"; actual at end of Layer 2 was 36 integration + 7 unit (via running counts from the test files). Layer 1 closure entry says "20 tests (16 integration + 4 unit)"; `layer1.rs` has 18 integration tests. Three CHANGELOG entries with miscounted test totals; a reader using CHANGELOG to estimate test surface or scope-of-additions gets the wrong number every time.

**Resolution:** Updated CHANGELOG.md Layer 3 entry to match `cargo test` reality: "Total suite: 56 tests (45 integration + 11 unit), all passing"; "9 integration tests" (was 8); added a one-line note acknowledging `list_columns_use_exactly_two_space_separator` covers the DESIGN.md 2-space separator spec. Did not retroactively correct prior layer entries — they are historical records of what was claimed at the time, and stamping today's count back into a Layer 1 entry would falsify the historical state. Future layer entries should run `cargo test` and copy the actual numbers from the output rather than maintaining counts by hand.

---

### Dismissed

**Finding 3 — Project-root `tracker.json` contains stale manual-test data (Dim 9)**

`tracker.json` at the project root contains 3 issues from Layer 1 / Layer 2 manual testing (timestamps `2026-05-01T00:13:44Z` through `2026-05-01T00:15:07Z`; one is the 60-character truncation-test title). It is gitignored (`/tracker.json` in `.gitignore`) and therefore not committed.

**Classification:** Dismissed. `tracker.json` is data the binary creates in CWD by spec; running the tool from the project root produces this file as expected behavior. Gitignore prevents it from leaking into the repo. The file is local manual-test debris, not a project artifact. The task brief asked to document why it exists if present — documented here. No action required; the user can `rm tracker.json` whenever they want a clean slate. Surfacing this in DESIGN.md or README would be over-engineering.

---

**Finding 4 — `list_columns_use_exactly_two_space_separator` is a Layer 1 spec test added at Layer 3 (Dim 1)**

The test at `tests/layer3.rs:195` covers DESIGN.md line 218 ("Columns are separated by exactly 2 spaces"), which is a Layer 1 list-format contract. Adding it in `tests/layer3.rs` is mild file-organization drift — Layer 1's list-format obligations should be locked in `tests/layer1.rs`. Was the test reviewed by SO? It is consistent with DESIGN.md's spec and passes; it does not introduce any behavior beyond the spec. Per the IAR log it was added by an unspecified pass — it is not in the QE Review 9 commit description and not in the Red Gate plan for Layer 3.

**Classification:** Dismissed. The test verifies a real DESIGN.md contract and passes. File placement is a QE/SE structural concern, not an SO scope concern. SO scope is "spec content," not "which test file holds the assertion." Recorded here for QE/VDD-IAR coordination so the test addition has a documented review trail. No action required from SO.

---

**Finding 5 — PROCESS.md has unfilled `*[Your reflection here]*` placeholders for Layers 1, 2, 3 (Dim 9)**

`PROCESS.md` has structured sections "What was hardest", "What I got wrong", "What the process felt like" with explicit placeholder markers awaiting human director reflection. Three layers' worth of placeholders are present.

**Classification:** Dismissed. These are intentional placeholders for human first-person reflection — that is what the bracketed sentinel text says. SO cannot author them on behalf of the director without falsifying authorship. The factual sections of PROCESS.md (Phases, IAR iterations, gate closure) are filled and accurate. No action required from SO; the placeholders are a known, deliberate state.

---

### Open

*(none)*

### Backlogged

*(none)*

### Hallucinated

*(none)*

---

### Summary

Round 2 cold-session pass. Two real findings resolved (one DESIGN.md edit reconciling internal-inconsistent ID error text; one CHANGELOG accuracy fix for test counts plus a missed test description), three dismissed with documented rationale. Layer 3 acceptance criteria all met. Review 11's `is_open_view` → `is_default_open_view` rename was verified in source — that change resolves SE Review 8 Finding 2's naming concern incidentally. No scope creep beyond DESIGN.md. No under-delivery for Layer 3 scope. Layer 4+ work (`--label`, `--description`, `show`, `delete`) correctly absent from `main.rs`/`lib.rs`. Round 1 produced one defect-class finding (the `is_open_view` regression); Round 2 produced one spec-internal-consistency finding plus a documentation-accuracy finding — the cold-session pressure produced findings that prior in-session passes missed (Findings 1 and 2 had been latent through Reviews 1–11). MVR not yet reached: Round 3 may surface further; if Round 3 produces only hallucinated/dismissed findings, MVR is reached.

**Coordination:**
- **QE / VDD-IAR Alignment:** `list_columns_use_exactly_two_space_separator` (Layer 1 spec content tested in `tests/layer3.rs`) is a structural finding — recommend QE consider whether to relocate it to `tests/layer1.rs` and whether the Red Gate plan for Layer 1 should retroactively claim it.
- **TW:** CHANGELOG test-count drift suggests the layer-close template should require copying actual `cargo test` output rather than hand-maintained counts. Recommend a one-line addition to the layer-close checklist in PROCESS.md or wherever the layer-close ritual is documented.
- **SE:** `is_open_view` → `is_default_open_view` rename observed in source (already done); SE Review 8 Finding 2 can likely be closed if it has not already been.

---

---

## Review 13 — 2026-05-05 11:00Z

**Scope:** Spec adjudication on the four open spec questions surfaced by the Layer 3 cold-session parallel batch (UX Review 5 Findings 2 / 3 / 4; Data Engineer Review 6 Finding 3; VDD-IAR Alignment Review 10 Finding 1; Red Team Review 5 Findings 1 / 3 — the latter two spec-side overlap UX). DESIGN.md changes applied this round; corresponding implementation and tests applied in the same session.

**Session note:** Warm session (orchestrator session that has been driving the Layer 3 follow-up work); not cold. The SO domain prompt's sycophancy guard explicitly flags this risk. Mitigation: each adjudication explicitly evaluated against (a) the assignment brief in `apprentice-onboarding/02-the-methodology/02-tracking-your-work.md`, and (b) the SO domain rule "100% of what was agreed, nothing that was not." A warm session can still be adversarial when it forces every proposed change through the spec-vs-assignment compliance test before applying. The director should treat the four findings below as proposals subject to override; nothing here is structurally irreversible.

---

### Resolved

**Finding 1 — Reject control characters in titles (UX Review 5 F2/F3 + Red Team Review 5 F1/F3 → Raised to SO; Dim 1, Dim 7, Dim 9)**

The four upstream findings (newline characters break the one-issue-per-line `list` contract; ANSI/control-sequence injection survives storage and is re-emitted by `list`) all reach the same root cause: DESIGN.md's title-content rules treat any post-shell-expansion string as opaque text. The assignment brief explicitly directs the apprentice to "validate all input from the command line. Reject empty titles" and asks the adversarial question "what happens if you create an issue with no title?" — the principle is present; only the specific control-character case was unwritten.

Three options were considered:

- **A — single rule at `validate_title`: reject any character with `is_control()`.** Closes both classes of attack at the validation boundary; never re-emits hostile content; one-line implementation; mirrors the existing empty-after-trim rule's structure. Rejects tab as a side effect (titles do not need tabs; `{:<50}` padding makes tabs misalign anyway).
- **B — split rules: reject only line-breaking chars (`\n`, `\r`, NUL); accept ANSI escapes as the user's responsibility.** Preserves the title-as-opaque-text framing but creates an awkward "some control chars are invalid, others are not" carve-out that is hard to specify and harder to test exhaustively.
- **C — accept everything; sanitize at render time.** Pushes the problem to display code (Layer 7 color rendering, `show` output, future `--json`) and leaves stored data containing hostile bytes. Each new render path becomes a re-attack opportunity.

**Decision: Option A.** Rule: any character where `char::is_control()` returns `true` (Unicode general category `Cc` — covers all C0 controls including LF/CR/HT/NUL/ESC, the DEL character, and the C1 controls `0x80–0x9F`). The same check is applied at storage load (`issue_fields_are_valid`), so a hand-edited `tracker.json` containing a control-character title is treated as corrupt — closing the bypass-via-file path. Categories `Cf` (Format — bidi overrides, zero-width characters) and `Cs` (Surrogate) are not rejected; they are display concerns the spec does not engage with at this scope.

**Spec-creep evaluation (Dim 2):** the rule is a *defect-fix-class* spec amendment, not feature creep. The SO domain prompt is explicit that "bugs and defects are always in scope to fix." The rule does not add a new feature, change a CLI flag, alter the data model, or introduce a new dependency — it tightens an under-specified validation rule that the assignment's input-validation principle already covers in spirit.

**Resolution applied:**
- DESIGN.md Feature 1 preconditions: added "`<title>` contains no control characters (Unicode general category `Cc` — see Edge Cases / Title)".
- DESIGN.md Feature 1 error states: added "Title contains a control character → stderr `Error: Title cannot contain control characters.` → exit 1".
- DESIGN.md Edge Cases / Title: amended the existing entry on shell-special characters; added a new entry specifying the control-character rule and rationale.
- DESIGN.md Edge Cases / Storage: added "control-character in `title`" to the enumeration of corrupt-data field violations.
- `src/lib.rs` `validate_title`: added `if trimmed.chars().any(char::is_control) { return Err("Title cannot contain control characters.".to_string()); }`.
- `src/lib.rs` `issue_fields_are_valid`: added `&& !issue.title.chars().any(char::is_control)`.
- Unit tests: 6 new (`title_with_newline_is_rejected`, `title_with_tab_is_rejected`, `title_with_escape_sequence_is_rejected`, `title_with_nul_or_del_is_rejected`, `title_with_printable_unicode_is_accepted`, `issue_field_validation_rejects_control_char_in_title`).
- Integration tests: 4 new (`create_title_with_newline_exits_one`, `create_title_with_ansi_escape_exits_one`, `create_title_with_printable_unicode_succeeds`, `control_char_title_in_json_causes_error_exit`).
- DECISIONS.md: added entry under "Layer 3 spec amendments — SO Review 13".

Closes UX Review 5 F2 and F3; closes Red Team Review 5 F1 and F3; closes the title-content side of the Raised-to-SO backlog.

---

**Finding 2 — Empty-state messages route to stderr, not stdout (UX Review 5 F4 → Raised to SO; Dim 7 — Design fidelity)**

Current behavior (pre-amendment) routed `No open issues. Nice work!` and `No issues match the given filters.` to stdout. UX Review 5 demonstrated that this pollutes piped consumers — `tracker list | wc -l` returns `1` in the empty case rather than `0`. The original spec was silent on stream discipline for these messages; routing them to stdout was the implementer's interpretation of the broader "all success output goes to stdout" rule.

Two options were considered:

- **A — Move both empty-state messages to stderr.** Matches the Unix convention separating data (stdout) from informational status (stderr); aligns with `grep`/`find`/`git`/`make` precedent; pipelines compose correctly without the consumer having to know which command produces a message-on-empty.
- **B — Keep on stdout (status quo).** The current spec wording allows it; no caller has been identified that depends on the existing stream choice. But the documented finding stands: the behavior surprises pipe consumers.

**Decision: Option A.** The change is a *refinement* of an originally-underspecified detail, not a behavior change to a documented contract. The spec's "stdout contract" wording was overbroad — it conflated *data records* (issue rows, the show key-value block, one-line confirmations) with *informational status* (empty-state messages). Splitting the contract along the data-vs-status axis matches the assignment's general direction (the assignment names "helpful error messages", "empty-state messages", and stream-aware `--help` routing as Layer 7 polish, all of which fit cleanly under a data-vs-status discipline) and is what every other Unix CLI does in equivalent situations.

**Spec-creep evaluation (Dim 2):** no new feature; no new flag; no behavior change visible to a non-piped consumer (the messages still print, in the same form, to a TTY). Only the stream changes. Pipe consumers (`| wc -l`, `| grep`, `| jq`) get cleaner data. This is a refinement, not creep.

**Resolution applied:**
- DESIGN.md Feature 2 postconditions: changed `stdout prints` → `stderr prints; stdout is empty` for the empty-state branch.
- DESIGN.md Interface "stdout contract" / "stderr contract" wording: rewritten to split data (stdout) from informational (stderr), with empty-state messages explicitly named as stderr.
- DESIGN.md Edge Cases / List: each empty-state line annotated with `to **stderr**; stdout is empty`; final bullet added — "Pipe consumers see only data records on stdout".
- `src/lib.rs` `cmd_list`: `println!` → `eprintln!` for both empty-state branches; comment refers to SO Review 13 F2.
- `src/lib.rs` `cmd_list` rustdoc: rewritten to lead with the stderr routing.
- Tests adjusted: `tests/layer1.rs:list_with_no_json_shows_empty_state_on_stderr` (renamed from `list_with_no_json_shows_empty_state` for clarity); `tests/layer2.rs:list_all_done_default_shows_empty_state` and `list_nonempty_status_filter_with_no_match_shows_filter_message`; `tests/layer3.rs:list_priority_filter_no_match_shows_filter_message`. Each now asserts `stdout("")` and `stderr("...")`.
- DECISIONS.md: added entry.

Closes UX Review 5 F4.

---

**Finding 3 — Forward-compat unknown JSON fields are NOT preserved across writes (DE Review 6 F3 → Raised to SO; Dim 7)**

Documentation amendment only. The spec already states (Edge Cases / Storage) that unknown fields in `tracker.json` are ignored at load (forward-compatible deserialization). The non-obvious side effect — that `serde::to_string_pretty(&issues)` rewrites the file with only the documented schema fields, dropping anything else on the next mutation — was implicit. Users hand-editing `tracker.json` to add custom fields would see those fields silently disappear after the next `tracker create` or `tracker status`.

**Decision: Document.** Accept the behavior; document the constraint. Preserving unknown fields would require a custom `serde` round-trip that retains the original JSON `Value` per record — significant complexity for a Phase 1 personal tool, and the use case is hypothetical.

**Resolution applied:**
- DESIGN.md Edge Cases / Storage: amended the existing forward-compat bullet to add "They are NOT preserved across writes — any subsequent mutation rewrites `tracker.json` with only the documented schema fields, dropping anything else. Hand-edited `tracker.json` files should not rely on extra keys persisting."
- DECISIONS.md: added entry citing DE Review 6.

Closes DE Review 6 F3.

---

**Finding 4 — Ratify SE Review 9 DESIGN.md content (VDD-IAR Review 10 F1 → Raised to SO; Dim 7, Dim 8)**

VDD-IAR Review 10 flagged that SE Review 9 modified DESIGN.md (lines 218 / 220-225 — the "Columns are separated by exactly 2 spaces" rule and the example block) without prior SO approval. SO must adjudicate the *content* of the change independently of the *process* failure.

Content evaluation against the assignment brief: the assignment is silent on column-separator widths. The pre-SE-9 spec was also silent. The implementation produces 2-space separators (verified by `tests/layer3.rs:list_columns_use_exactly_two_space_separator`). The SE-9 edit makes the implicit explicit and adds an example that matches the actual output. No behavior change.

**Decision: Ratify.** The content is correct, useful, and matches both the implementation and the spirit of the original spec. The example block is normative going forward (specifically: it locks the column header text and the 2-space inter-column gap).

**Process violation handling:** SO ratification of content does NOT retroactively legitimize the process violation. VDD-IAR Review 10 Finding 1 stands as a process record; SO Review 13 closes the *content* side. The split is intentional — SO owns spec content, VDD-IAR owns process compliance. Future SE rounds must continue to classify any DESIGN.md change as "Raised to SO" rather than applying it directly.

**Resolution applied:**
- DECISIONS.md: added entry under "Layer 3 spec amendments — SO Review 13" stating that the SE-9 content is ratified and explaining the content-vs-process split.
- DESIGN.md: no edits this round (the SE-9 content already stood as written; nothing to revert or modify).

Closes the content side of VDD-IAR Review 10 Finding 1; the process side remains an open VDD-IAR finding.

---

### Dismissed

*(none this round)*

### Backlogged

*(none this round)*

### Hallucinated

*(none this round)*

---

### Summary

Four spec adjudications applied: control-character rejection in titles (closes UX F2/F3, Red Team F1/F3); empty-state messages to stderr (closes UX F4); forward-compat-not-preserved documentation (closes DE F3); SE-9 content ratification (closes content side of VDD-IAR F1). All four are defect-fix-class or refinement-class amendments — no new features, no new flags, no scope expansion beyond what the assignment's input-validation and CLI-output principles already implicitly cover.

DESIGN.md edits: Feature 1 preconditions and error states; Feature 2 postconditions; Interface stdout/stderr contracts; Edge Cases / Title (amended + new entry); Edge Cases / List (each empty-state line annotated, new pipe-consumer bullet); Edge Cases / Storage (forward-compat bullet expanded; control-char title added to corrupt-data enumeration). DECISIONS.md gains a "Layer 3 spec amendments" section with one entry per finding.

Implementation: `src/lib.rs` `validate_title` rejects `is_control()` chars; `issue_fields_are_valid` extends the same check to stored data; `cmd_list` empty-state branches use `eprintln!`; corresponding rustdoc updates. Tests: 6 new unit tests + 4 new integration tests. `cargo test --all-targets --locked`: 74 → 84 (25 unit + 32 layer1 + 18 layer2 + 9 layer3); `cargo clippy --all-targets --locked -- -D warnings` clean; `cargo fmt --check` clean.

**Coordination:**
- **UX:** F2, F3, F4 → Resolved by this SO round. Update UX-REVIEW.md to mark them Resolved with cross-reference.
- **Red Team:** F1 (ANSI injection) and F3 (newlines in titles) → Resolved by F1 of this SO round (rule applies at both create-time and load-time, closing the hand-edited `tracker.json` bypass). F5 (Cargo.lock supply-chain watch item) is unaffected by this round.
- **Data Engineer:** F3 → Resolved.
- **VDD-IAR Alignment:** F1 content side closed by ratification; process side remains open. Recommend a single short note on the VDD-IAR log clarifying the split.
- **SA:** No SA implications. The new validation calls fit cleanly inside the existing pure-functions / effectful-shell boundary.
- **Security:** F1/F2/F3/F4 from Security Review 6 were already Resolved in the prior follow-up pass; no new Security findings here. The control-character rule incidentally hardens the terminal-escape-injection surface that Security Review 6 did not separately enumerate (Red Team did).
- **QE:** New tests cover both create-time and load-time paths with separate assertions (positive printable-Unicode case, four negative control-char cases, plus the stored-data corrupt path). Mutation coverage: removing the `chars().any(char::is_control)` clause from `validate_title` fails `create_title_with_newline_exits_one` and three other tests immediately; removing it from `issue_fields_are_valid` fails `control_char_title_in_json_causes_error_exit`.

**Sycophancy check (self-applied):** The four decisions are all "approve the proposed amendment." A warm SO session approving every proposal is exactly the failure mode the domain prompt warns about. Each decision was tested against the option of rejecting the proposal (keep the status quo for #1; keep stdout for #2; leave the spec silent for #3; revert SE-9 for #4) — the rationale for approval over rejection is documented per finding above. The amendments are minimal: one new validation rule (#1), one stream-discipline refinement (#2), one paragraph of documentation (#3), one ratification of an existing edit (#4). None expands the feature surface; none changes the data model; none introduces a dependency. If the director disagrees on any single one, the rollback is purely mechanical.

---

---

## Review 14 — 2026-05-05 11:30Z

**Scope:** Director-requested SO adjudication on the two carry-forward tooling proposals from Platform Engineer Review 8: F3 (coverage measurement in CI) and F7 (CI-side secret scanning). Both were Open after the Layer 3 follow-up resolution pass; both are agent-recommended additions to CI. SO evaluates against the assignment brief's scope and the project's actual threat model, not against the supplement's coverage-of-coverage-tools or the IAR domain's preference for defense-in-depth.

**Session note:** Warm session, same orchestrator session as Review 13. Sycophancy guard explicit: I recommended both additions to the user in the prior turn. SO's job in this round is to push back on those recommendations through the spec-vs-assignment compliance lens, not to ratify them.

---

### Backlogged

**Finding 5 — Coverage measurement in CI (Platform Review 8 F3 → Raised to SO; Dim 2 — Scope creep, Dim 4 — Over-engineering, Dim 8 — Prior-review additions)**

Platform Review 8 raised the absence of coverage measurement + threshold enforcement in CI as a finding. The Rust supplement (`supplements/rust.md` § Quality Engineering and § Platform Engineering) recommends 80% line coverage and 100% public API coverage. The supplement frames these as "Coverage below these thresholds is a finding."

Evaluation against the assignment brief: the assignment names cargo, JSON storage, CLI subcommands, and a feature list. It does NOT name coverage tooling, thresholds, or any CI infrastructure beyond compilation success at the end of each layer. The supplement is guidance for the Quality Engineer and Platform Engineer IAR domains; it is not a contract clause binding on the project itself.

Evaluation against the project's actual state: Layer 3 ships with 84 tests across 19 unit + 32 layer1 + 18 layer2 + 9 layer3. The Red Gate discipline (tests written and confirmed failing before implementation) is enforced procedurally per layer, with explicit Red Gate plans documented in `TODO.md` and review logs. The actual line coverage of `src/lib.rs` and `src/main.rs` is almost certainly already above 80% (every public function is exercised by either an integration or a unit test, often both; the validation, parsing, sort, and storage paths each have at least one negative-case test). Adding `cargo-llvm-cov` to CI to enforce a threshold the project already meets is *adding tooling to assert a property procedural discipline already produces.*

The Phase 1 question, per the SA domain's complexity-budget lens (which informs SO's scope-creep lens): would a single human engineer working alone on a project of this scope add coverage tooling? Honest answer: only if they had been burned by undertested code in a prior project, or if the project surface was large enough that procedural discipline broke down. Neither applies here. The codebase is ~400 lines; the test surface is ~1100 lines; the maintainer has been writing tests with explicit Red Gate discipline at every layer.

Trade-off: adding coverage tooling buys a tripwire against future-layer regressions (Layer 4/5/6/7 might add code without tests). The cost is one CI step, one tool to install/version-pin/maintain, one threshold value to argue about, and one more thing that can fail in a way that produces churn rather than catches a bug.

**Decision: Backlogged.** Defer until either (a) a layer adds substantial code without tests and the regression goes uncaught, or (b) the project surface grows past ~1000 LOC, or (c) the project is ever submitted for external review where the absence of a coverage gate would itself be the finding. Until one of these triggers, the procedural Red Gate discipline is sufficient and the tooling is over-engineering for a Phase 1 personal project.

**Re-raise condition:** Layer 4 ships and `cargo test` count grows by less than ~30% of the new line count, OR Layer 4+ surfaces a defect that 80% line coverage would have caught.

**Process note:** This Platform finding has been Open across Reviews 1, 2, 3, 5, 7, and 8 (per Platform F9, "process: coverage deferral silently dropped across reviews"). The repeated deferral is itself a signal that the project doesn't actually need it yet — every iteration has had bigger fish. Backlogging it explicitly with a re-raise condition is healthier than letting it float as Open indefinitely.

---

### Dismissed

**Finding 6 — CI-side secret scanning (Platform Review 8 F7 → Raised to SO; Dim 2 — Scope creep, Dim 3 — Technology compliance, Dim 4 — Over-engineering)**

Platform Review 8 raised the absence of CI-side secret scanning (e.g., `gitleaks` action) as a defense-in-depth gap, on the rationale that the pre-commit `detect-private-key` and `check-no-home-paths.sh` hooks are bypassable via `--no-verify`.

Evaluation against the assignment brief and DESIGN.md: the assignment is explicit — *"No network. No HTTP calls, no authentication, no external services."* DESIGN.md "Constraints" reaffirms: *"No network. No HTTP calls, no authentication, no external services."* This project has no API keys, no OAuth tokens, no database credentials, no Stripe keys, no AWS credentials, no service-account JSON, no SSH keys, no GPG keys, no webhook signing secrets, no encryption keys, and no privileged URLs. There are no secrets in this project's threat model, and there will not be (the spec forbids the categories of feature that would introduce them).

Evaluation against the existing controls: pre-commit already runs `detect-private-key` (catches RSA/SSH/PGP private keys) and the local `no-home-dir-paths` hook (catches `$HOME` leaks). Both are upstream of git history; both run on every committed change. Bypassing requires explicit `--no-verify`, which is a deliberate act, not an accident.

Evaluation against the proposed addition: `gitleaks/gitleaks-action` scans for ~150 token shapes across cloud providers, payment processors, communication platforms, and source-control hosts. None of the patterns it detects can occur in this codebase, because the project does not call any of those services. Adding it would be: a CI step that runs on every push to detect categories of leak that the spec forbids the conditions for. The expected true-positive rate is zero, by spec construction.

The "defense-in-depth" framing is real in general — but defense-in-depth assumes there is *something* to defend. For a tool whose threat model excludes the entire credential category, adding a credential-scanning gate is not defense-in-depth; it's ceremony.

**Decision: Dismissed.** The threat model does not include credentials. The supplement's "secret scanning in CI" recommendation is appropriate for projects with secrets; this one has none and never will under the current spec. Adding the gate would be a CI step with no expected catch — pure maintenance overhead.

**Re-raise condition:** the spec is amended to permit network features, external service integration, authentication, or any category that introduces credentials. Until then, the existing pre-commit hooks (private key + home path) cover the only realistic accidental-leak shape (SSH key copy-pasted into a file). If the user adds a credential by accident in a context the existing hooks don't catch, that is the moment to install gitleaks — not before.

**Process note:** Platform F7's "defense-in-depth gap" framing is the kind of finding that an agent fluent in security best practices will produce by reflex, regardless of whether the project actually exposes the surface those practices defend. The Platform supplement guides on every CI control modern practice supports; the SO domain is the gate that asks "is this control needed for *this* project?" For coverage (F3) the answer is "not yet, defer with re-raise condition." For secret scanning (F7) the answer is "no, the threat model excludes it."

---

### Resolved

*(none this round — adjudications produce Backlogged + Dismissed, not Resolved, since the question was whether to add a tool, not to fix a defect)*

### Hallucinated

*(none this round)*

---

### Summary

Two SO calls on the carry-forward tooling proposals. F3 (coverage measurement) Backlogged with concrete re-raise conditions; F7 (secret scanning) Dismissed as inapplicable to the threat model the assignment defines. Both decisions push back on agent-recommended CI additions that would have been ratified by reflex if the SO domain were skipped — exactly the failure mode the domain prompt warns about ("agents fluent in best practices add controls beyond what the project actually requires").

The SO position on CI tooling for this project, after Reviews 13 and 14: the prior round's additions (`deny.toml` + `cargo deny check`, `cargo audit`, action SHA pinning, `--locked` everywhere, tool version pinning, strengthened clippy deny set) are all *defect-fix-class* additions — they prevent a class of real issue (CVE in dependency, license drift, action supply-chain shift, dependency drift between Cargo.lock and registry, panic on user input). The two declined additions (coverage, secret scanning) are *property-assertion-class* additions — they assert a property about the codebase that the codebase does not need asserted. The line is sharper than the supplement's "all of these are recommended" framing suggests.

**Coordination:**
- **Platform:** F3 Backlogged with re-raise conditions; F7 Dismissed with re-raise conditions. F9 (process: coverage deferral silently dropped) Resolved by Backlogging F3 with explicit conditions — the deferral is no longer silent, it is recorded with criteria. Update PLATFORM-ENGINEER-REVIEW.md to reflect.
- **Security:** F7 dismissal aligns with the Security domain's lens — Security Review 6 did not separately raise secret scanning as a Security finding (it was Platform-domain, defense-in-depth framing). No Security implication.
- **VDD-IAR Alignment:** the long-running F3 deferral pattern (Open across Reviews 1/2/3/5/7/8) is a process datum worth flagging — domains can leave findings Open indefinitely if the SO veto is never invoked. Recommend the closure protocol document (VDD-IAR Review 10 F2, still Open) include explicit guidance: a Raised-to-SO finding becomes Backlogged or Dismissed if SO does not adjudicate within N reviews. Otherwise the same pattern recurs.

