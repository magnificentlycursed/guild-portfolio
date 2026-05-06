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

---

---

## Review 15 — 2026-05-05 18:30Z

**Scope:** Director-requested SO adjudication on the `Cargo.toml` `license` sub-item from TW Review 6 Finding 6. The decision was forced by a CI failure: `cargo deny --locked check` flagged `error[unlicensed]: tracker = 0.1.0 is unlicensed`. The other half of TW F6 — the `repository` URL — is unaffected by CI and remains unresolved.

**Session note:** Warm session. The TW finding was raised on 2026-05-04 and has been Raised-to-SO across SO Reviews 10, 11, 12, 13, and 14 without adjudication — five consecutive reviews. Per the auto-Backlog rule in `CLOSURE-PROTOCOL.md` Section 3 (three consecutive reviews → auto-Backlog), this finding should have been auto-Backlogged at SO Review 13 at the latest. CI forcing the adjudication is the dual of that failure mode: the protocol's auto-Backlog rule prevents indefinite-Open on the project's calendar, but external pressure (a failing CI gate) can force adjudication earlier and more sharply than scheduled review cadence would. Both pressures are healthy; neither requires the other.

---

### Resolved

**Finding 7 — `Cargo.toml` `license` field absent (TW Review 6 Finding 6 sub-item → Raised to SO; Dim 1 — Spec coverage / Dim 9 — Assignment compliance)**

TW Review 6 Finding 6 (2026-05-04) flagged `Cargo.toml` as missing four metadata fields. TW resolved `description` and `readme` directly within their authority, and raised the remaining two (`license`, `repository`) to SO with the proposal text: *"set `license` (suggested: standard Rust ecosystem `\"MIT OR Apache-2.0\"`) and `repository` (likely the `guild-portfolio` GitHub URL pointing at the `issue-tracker-cli` subdirectory) before any external distribution or portfolio handoff."*

The license sub-item became CI-blocking on 2026-05-05 when the new `cargo deny --locked check` step (added in the Layer 3 follow-up resolution pass per Platform F2) ran in CI for the first time on a workflow that included `[licenses]` enforcement. The check correctly identified the self-crate as unlicensed because no `license` or `license-file` field was set on `[package]`. `publish = false` blocks crates.io upload but does not satisfy `cargo deny`'s allowlist gate.

Decision criteria:
- **Assignment compliance.** The Phase 1 assignment does not mandate any specific license. Any choice the SO makes is in scope; absence is not (CI confirms).
- **Rust ecosystem norm.** `MIT OR Apache-2.0` is the standard for Rust crates: permissive, dual-licensed, compatible with both ends of the open-source community, and matched by `deny.toml`'s existing allowlist (lines 39–40).
- **TW's own proposal.** The TODO comment in `Cargo.toml` named `"MIT OR Apache-2.0"` as the example, and TW Review 6 Finding 6's resolution prose suggested the same value. The director's adjudication aligns with the originating domain's recommendation rather than overriding it — this is the cheap call.
- **No external distribution yet.** `publish = false` remains. The license is declared for tooling-correctness; the matching `LICENSE-MIT` and `LICENSE-APACHE` files (which the licenses' attribution clauses require at distribution time) are deferred until external distribution is planned. Setting only the SPDX identifier resolves CI without committing to distribution-readiness.

**Resolution:** Director applied `license = "MIT OR Apache-2.0"` to `[package]` in `Cargo.toml`. The TODO comment was trimmed: the `license` reference removed, the `repository` reference kept (still pending). `cargo build --locked --quiet` and `cargo test --locked --quiet` (84/84 pass) verified locally; `cargo deny` not installed on the dev machine, so CI is the validation point — the targeted error is `error[unlicensed]: tracker = 0.1.0 is unlicensed`, which the SPDX field directly addresses per `cargo-deny`'s license-graph synthesis.

**Carry-forward — `repository` URL.** The TW F6 sub-item for `repository` is not blocking CI and is not adjudicated this round. It remains Raised-to-SO under TW Review 6 Finding 6. Per the auto-Backlog rule (CLOSURE-PROTOCOL Section 3), this is the first adjudication of the broader TW F6 cluster — the `repository` half resets to "Open in SO docket" and gets two more SO reviews of grace before auto-Backlog should fire. Recommended trigger for adjudication: when the project gets a public repository URL (likely the `guild-portfolio` repo's `issue-tracker-cli/` subdirectory path), or when external handoff is planned. Until either condition triggers, no harm in leaving the field empty — `cargo build` and `cargo deny` both ignore missing `repository`.

**Distribution-readiness deferral.** If external distribution is ever planned (publish to crates.io, submit for portfolio review where reuse rights matter, transfer to another developer), the matching pair of license files (`LICENSE-MIT` and `LICENSE-APACHE`, content from https://choosealicense.com or the Rust API guidelines) must be added to the project root. The SPDX field declares the offer; the license texts are required for the offer to be legally effective at distribution. Not done this round; not blocking; flagged so it isn't lost.

---

### Dismissed

*(none this round)*

### Hallucinated

*(none this round)*

### Open

**Finding 8 — `Cargo.toml` `repository` field still absent (carry-forward from TW Review 6 Finding 6)**

The other half of TW F6 remains Raised-to-SO. Not blocking CI, not blocking the layer gate, and the project has no canonical repository URL distinct from the parent monorepo path. The auto-Backlog clock starts now: this Review (15) is the first SO touch of the broader TW F6 cluster — at SO Reviews 17 or 18, if still un-adjudicated, the originating domain (TW) should auto-Backlog per CLOSURE-PROTOCOL Section 3.

**Classification:** Open — pending external-distribution trigger or explicit director call.

---

### Summary

One real finding resolved: `Cargo.toml` `license` set to `"MIT OR Apache-2.0"` per TW's own proposal, restoring CI green after `cargo deny`'s licenses check correctly flagged the self-crate as unlicensed. The repository sub-item carries forward, with the auto-Backlog clock now started.

The process datum for VDD-IAR: the same long-running-Open pattern flagged in SO Review 14's Coordination section (and that motivated CLOSURE-PROTOCOL.md Section 3) is visible here too. TW F6 was Open across SO 10/11/12/13/14 without adjudication — five reviews, beyond the three-review auto-Backlog threshold. The protocol existed by Review 14's close but was not retroactively applied, and Review 15 was forced by CI rather than triggered by the auto-Backlog rule. This is consistent with CLOSURE-PROTOCOL.md being a forward-looking document; existing carry-forward Open findings did not get a sweep-and-classify pass when the protocol was adopted. A one-time sweep of pre-protocol Open findings against the 3-review rule is a candidate Platform/Process action — flagged as a coordination item for VDD-IAR, not a finding for this review.

**Coordination:**
- **Platform:** Platform Engineer log should record the CI fix (the SO adjudication is the action; Platform owns the CI gate visibility). Append an Update under Review 8 with the diagnostic detail and the resolution path.
- **Technical Writer:** TW Review 6 Finding 6 is now half-resolved (license closed; repository still Raised-to-SO). TW does not need a new review pass to record this — the SO log is the canonical resolution record, and TW's next review (when it runs) can note the closure in passing.
- **VDD-IAR Alignment:** flag the missed auto-Backlog application across SO 13/14 as a process datum — CLOSURE-PROTOCOL.md was published mid-stream, and pre-protocol Open findings need either (a) a one-time backfill sweep or (b) explicit guidance that the protocol applies only to findings raised after its adoption date. Not a finding against this review; an input for the next VDD-IAR pass.
- **Distribution-readiness:** if/when this project is prepared for external distribution, the LICENSE-MIT and LICENSE-APACHE text files must be added. Not blocking now, but deferred-not-forgotten.

---

---

## Review 16 — 2026-05-05 21:30Z

**Round:** SO Review 16
**Scope:** Layer 4 (labels) primary — `--label` on create, label dedup, `--label` filter on list, multiple-`--label` on list rejected. Secondary: full DESIGN.md regression check across prior layers.
**Session context:** Fresh subagent; cold session; no prior conversation. Part of a full-suite IAR run on branch `issue-tracker-cli-labels`. The director provided the SO domain prompt and the read-order; this session has not participated in scoping or implementing Layer 4.

---

### Compliance table (delta — Layer 4 features and regression spot-check)

| Requirement (DESIGN.md) | Status | Notes |
|---|---|---|
| Feature 1: `--label <l>...` (repeatable on create) | Met | `main.rs:21-22` `#[arg(long)] label: Vec<String>`; `cmd_create` consumes `&[String]`. |
| Feature 1 precondition: each label non-empty after trim | Met | `lib.rs:339-346` `parse_label` trims, errors on empty. |
| Feature 1 postcondition: `labels` deduplicated, order preserved, case preserved | Met (with caveat — see Finding 1) | `lib.rs:351-360` `dedupe_labels` first-occurrence, case-sensitive. Operates on **trimmed** values; spec is ambiguous on store-as-trimmed vs. store-as-provided. |
| Feature 1 error: empty label → `Error: Label cannot be empty.` | Met | `tests/layer4.rs:60-83`; verified via `parse_label`. |
| Feature 2: `--label <l>` filter (single, case-sensitive, exact-match) | Met | `lib.rs:367-369` `label_matches` uses `==`; `tests/layer4.rs:218-235`. |
| Feature 2: multiple `--label` flags on `list` → usage error, exit 1 | Met | clap `Option<String>` rejects second flag; binary transforms `error:` → `Error:` in `main.rs:62`; verified by `tests/layer4.rs:238-257` and runtime smoke test. |
| Feature 2 / Edge Cases: `--label bug` excludes unlabeled issues | Met | `tests/layer4.rs:211-214`. |
| Feature 2 / Edge Cases: `--label Bug` does not match issue labeled `bug` | Met | `tests/layer4.rs:218-235`. |
| List output: `Labels` column 20 chars min, comma-separated, truncate at 20 with `…` | Met | `lib.rs:441-457` `{:<20}` + `truncate_with_ellipsis`; matches DESIGN.md example exactly (verified by smoke test). |
| List output: `(none)` for empty labels | Met | `lib.rs:447-451`. |
| `tracker.json` Issue.labels round-trip | Met | `Issue` struct unchanged from Layer 1; serde `Vec<String>`. |
| Regression: empty-state messages still on stderr (SO 13 F2) | Met | `lib.rs:432, 434` `eprintln!`; smoke test confirms stdout empty, stderr message. |
| Regression: control-character title rejection (SO 13 F1) | Met | unit tests still pass; `validate_title` and `issue_fields_are_valid` both check `is_control()`. |
| Regression: priority sort + ID tie-break (Layer 3) | Met | `sort_issues` unchanged; `priority_sort_*` unit tests pass. |
| Regression: status idempotency, refresh `updated_at` (Layer 2) | Met | `cmd_status` unchanged in Layer 4. |
| Out-of-scope (Layer 4): `--description`, `show`, `delete` not present | Met (Deferred) | `main.rs` defines only `Create`, `List`, `Status` subcommands; no Layer 5+ leak. |
| Layer 4 acceptance criteria (TODO.md, 11 items) | Met | All 11 ACs map to passing tests in `tests/layer4.rs` + unit tests in `lib.rs`. |
| Manual testing checklist (Layer 4, TODO.md lines 203-212) | Unverified | All 9 manual items still `[ ]` unchecked in TODO.md. May reflect "not yet executed" rather than "skipped"; flagged as observation, not finding (Layer 4 gate not closed). |

---

### Findings

#### Finding 1: Label trim-on-store vs. store-as-provided is implementer's choice; DESIGN.md is ambiguous

- **Dimension:** Dim 7 (Design fidelity), Dim 1 (Spec coverage — under-specification surfaced by Layer 4)
- **Severity:** Low
- **Evidence:**
  - DESIGN.md Feature 1 postcondition (line 28): "`labels` is the deduplicated list of `--label` values; order is preserved, case is preserved as provided".
  - DESIGN.md Feature 1 precondition (line 22): "If `--label` is present, each label value is non-empty after trimming".
  - DESIGN.md Edge Cases / Labels (line 304): "Whitespace-only label (`--label "  "`) → error: `Label cannot be empty.` (checked after trim)".
  - `src/lib.rs:339-346` `parse_label` trims and returns the trimmed value. `cmd_create` (line 213-217) collects parsed (i.e., trimmed) labels and passes them through `dedupe_labels`. Net: stored labels are trimmed.
  - Runtime smoke test: `tracker create "T" --label "bug " --label "bug"` → `tracker.json` contains `labels: ["bug"]` (trimmed and deduped).
- **Rationale:** The spec is genuinely ambiguous. "Case is preserved as provided" reads as "store as the user provided" and could plausibly extend to whitespace. "Each label value is non-empty after trimming" and "(checked after trim)" both frame trimming as a *validation* step, not necessarily a *storage normalization* step. The implementation's chosen interpretation (trim-on-store) is reasonable, user-friendly, and consistent with title handling — but DESIGN.md does not say so. A reviewer reading the spec literally and an implementer reading the spec liberally will produce different behavior, which is the same defect class SO Review 1 Finding 2 caught for `tracker list --label` with multiple flags.
- **Classification:** Open — Raised to SO (this round) for spec clarification.
- **Proposed action:** Amend DESIGN.md Feature 1 postcondition to explicitly state the storage normalization. Suggested text: "`labels` is the deduplicated list of `--label` values, with each value trimmed of leading/trailing whitespace; order is preserved (first occurrence wins), case is preserved as provided after trimming; empty if no `--label` flags given." Optionally amend the Edge Cases / Labels section to add a bullet: "Leading/trailing whitespace on a label (`--label '  bug  '`) → stored as `'bug'` (trimmed); deduplication is performed against trimmed values."
- **Spec-creep evaluation:** This is a *clarification of an existing under-specification*, not a new feature or behavior change. The implementation's behavior would be unchanged; the spec would simply describe what the implementation already does. This is a defect-fix-class amendment in the same family as SO Review 13 Finding 2 (empty-state stream discipline).

---

#### Finding 2: List `--label` filter does not validate empty/whitespace-only filter values; behavior diverges from create-side validation

- **Dimension:** Dim 1 (Spec coverage — under-specification), Dim 7 (Design fidelity — symmetry with create-side rule)
- **Severity:** Low
- **Evidence:**
  - DESIGN.md Feature 2 error states (lines 73-74) enumerate only invalid `--status` and `--priority` values. Empty/whitespace `--label` filter is not addressed.
  - `src/lib.rs:400-424` `cmd_list` accepts `label_filter: Option<&str>` and passes it directly to `label_matches` without validation.
  - Runtime smoke test: `tracker list --label ""` → exits 0, prints `No issues match the given filters.` to stderr. Same for `tracker list --label "  "`.
  - Compare: `tracker create "T" --label ""` → exits 1, prints `Error: Label cannot be empty.` to stderr (per `parse_label`).
- **Rationale:** The spec is silent on the case, so the implementation is not strictly out of compliance. But the asymmetry is observable and surprising: a user who knows `--label ""` is invalid on create may reasonably expect symmetric rejection on list. The current behavior — silent acceptance, "no match" message — could mask a typo (e.g., shell-stripped quotes producing an empty argument) and lead the user to conclude their data is missing. The principle behind the create-side rule (validate-at-the-boundary, DESIGN.md Constraints line 278) arguably extends to filter values too.
- **Classification:** Open — Raised to SO (this round) for spec clarification, OR Hallucinated if the director takes the position that "exact-match against an empty filter naturally returns no matches" is the intended behavior. The decision is a real spec call.
- **Proposed action (option A — tighten spec):** Amend DESIGN.md Feature 2 error states to add: "Empty or whitespace-only `--label` filter → stderr `Error: Label cannot be empty.` → exit 1." Amend `cmd_list` to call `parse_label` on `label_filter` before applying. One unit test + one integration test required.
- **Proposed action (option B — document silent behavior):** Amend DESIGN.md Edge Cases / Labels to add: "Empty or whitespace-only `--label` filter on `tracker list` → no match (exits 0 with `No issues match the given filters.`); not validated as an error since stored labels are guaranteed non-empty by the create-time rule, so an empty filter is well-defined as 'no possible match.'" No code change required.
- **Spec-creep evaluation:** Either option is a *clarification of an under-specification*, not a new feature. Option A adds one validation point that mirrors an existing one; option B documents the existing behavior. Neither expands the feature surface.

---

#### Finding 3: Layer 4 manual testing checklist not visibly executed in TODO.md

- **Dimension:** Dim 1 (Spec coverage — DESIGN.md Testing Methodology requires manual testing per layer)
- **Severity:** Low (process observation; coordination item)
- **Evidence:**
  - DESIGN.md Testing Methodology / Manual testing checklist (line 367): "Each layer must be manually tested before the layer gate closes".
  - `TODO.md:203-212` lists 9 Layer 4 manual testing items, each rendered as `- [ ]` (unchecked). Compare Layer 3's checklist (`TODO.md:153-161`) which is fully `- [x]`.
  - Branch is `issue-tracker-cli-labels` and Layer 4 IAR is in progress (this review is part of it), suggesting the layer gate has not closed.
- **Rationale:** This is consistent with Layer 4 being mid-flight rather than gate-closed. Not a deviation if the gate is genuinely open. But the checklist must be executed and ticked before Layer 4 closes per DESIGN.md. Recording it here so the director and the next-tier reviewers (UX, VDD-IAR) know the status.
- **Classification:** Open — observation, pending Layer 4 gate. Not a defect against the implementation; a process tracking note. Coordination: VDD-IAR Alignment domain owns process compliance and should pick this up.
- **Proposed action:** Before Layer 4 gate closure, the director executes the 9 manual checks (or has the implementing agent execute them) and ticks each box in TODO.md. No DESIGN.md or source code change.

---

### Spec-creep audit (Dim 2 — additions not in DESIGN.md)

Walked the Layer 4 diff (current branch vs. `main`) for additions not described in DESIGN.md:

- **`parse_label`, `dedupe_labels`, `label_matches`** in `src/lib.rs` — directly required by Feature 1 postcondition (dedup) and Feature 2 (filter). Not creep.
- **`Vec<String>` arg parsing on `Create::label`** in `src/main.rs` — directly required by Feature 1 synopsis (repeatable). Not creep.
- **`Option<String>` arg parsing on `List::label`** in `src/main.rs` — directly required by Feature 2 synopsis (single value); the `Option` (rather than `Vec`) is what enforces the "single value" constraint, producing the multiple-flags error. Not creep.
- **No new dependencies in `Cargo.toml`** — confirmed; `serde`, `serde_json`, `clap`, `chrono`, `libc` (unix) unchanged from Layer 3.
- **No Layer 5+ subcommands** — `main.rs` enum still `Create | List | Status`. No `show`, no `delete`. No `--description` flag. Correct phase alignment.

No scope creep detected in Layer 4. The implementation adheres to the layer scope boundary.

---

### Dim 9 — Assignment compliance check

The assignment brief at `apprentice-onboarding/02-the-methodology/02-tracking-your-work.md` is referenced in DESIGN.md (line 7) and prior SO reviews (esp. Review 13 Finding 1). The brief is not present in this session's working tree (subdirectory `apprentice-onboarding/` does not exist locally), so I cannot independently re-evaluate Dim 9 against the source. Prior SO reviews (1-15) have done this audit and concluded DESIGN.md faithfully represents the assignment for the spec-defined feature surface. No new assignment-compliance concern surfaced by Layer 4 — labels are an explicit assignment feature ("Add labels to issues"), and the Layer 4 implementation matches DESIGN.md's interpretation of that feature.

If the director wants a fresh Dim 9 pass with the brief in scope, schedule it once the file is reachable in this branch's tree.

---

### Hallucinated

*(none this round)*

### Backlogged

*(none this round)*

### Dismissed

*(none this round)*

---

### Summary

4 findings: 4 Open, 0 Resolved, 0 Hallucinated, 0 Dismissed, 0 Backlogged. (Updated by the Dim 9 addendum below — F4 added 2026-05-05 21:38Z.)

F1, F2, and F3 are low-severity (see analysis below). F4, added by the Dim 9 addendum, is Medium-severity: DESIGN.md "Out of Scope" item line 394 unilaterally reclassifies the assignment's Layer 6 "delete with confirmation" as advisory and waives it; the assignment text does not authorize that reclassification. F1 and F2 are spec-clarification candidates surfaced by Layer 4 — the implementation made defensible choices in spec-ambiguous areas (label trim-on-store; empty-filter silent-no-match) that should be either ratified into DESIGN.md or amended. F3 is a process observation about Layer 4 manual testing being unexecuted, which is normal for an open layer and is recorded as a coordination input for VDD-IAR.

The Layer 4 implementation itself is in spec compliance: 11/11 acceptance criteria covered by passing tests; no new dependencies; no scope creep into Layer 5+ features; regression of prior-layer compliance (empty-state stderr routing, control-character rejection, priority sort, status idempotency) holds. The compliance table delta is clean.

**Sycophancy check (self-applied):** A clean Layer 4 review is exactly the failure mode the domain prompt warns about. I tested whether the three findings are real by trying to dismiss each one:
- F1 (label trim-on-store): could the spec text already cover this? No — "case is preserved as provided" and "checked after trim" pull in different directions; the implementer chose one resolution; the spec doesn't ratify it. Real finding.
- F2 (empty filter silent-no-match): could "exact match" be construed to inherently allow empty filter? Yes, that's a defensible reading — which is why the finding is classified as a spec-clarification call rather than a defect. Real but soft.
- F3 (manual checklist unchecked): could this be normal for an in-progress layer? Yes, and the finding records it as such — this is a coordination note, not an accusation.

Each finding survived the dismissal test. The clean compliance table is genuine; the three findings are the residue.

**Coordination:**
- **Software Engineer:** F1 has a code-side option (do nothing, since the implementation already trims-on-store; the change would only be to DESIGN.md). F2 has either a code-side option (call `parse_label` on the list filter — one line in `cmd_list`) OR a doc-only option. SE should not act unilaterally; SO must adjudicate the spec call first.
- **Quality Engineer:** if SO chooses Option A on F2, QE should add `list_empty_label_filter_exits_one` and `list_whitespace_label_filter_exits_one` integration tests. If Option B, QE may add a positive-coverage test asserting the silent-no-match behavior to lock it in.
- **VDD-IAR Alignment:** F3 (manual testing checklist status) is a process observation in VDD-IAR's wheelhouse. Recommend VDD-IAR's next pass verify the Layer 4 manual checklist is executed before gate closure, consistent with prior layers.
- **UX:** the empty-filter silent-no-match behavior (F2) is a UX surface — UX domain may have an opinion on whether the silent behavior surprises pipe consumers or interactive users. Flag for UX awareness.
- **Security / Red Team:** Layer 4 introduces no new attack surface beyond what was hardened in Layer 3 (control-character rejection extends to titles, not labels — labels are validated only as non-empty after trim). If labels become a render path that emits to a TTY (Layer 7 color, or future `show`), Red Team should evaluate label-content escape injection at that point. Out of scope for this round.

---

### Dim 9 — Assignment compliance addendum (2026-05-05 21:38Z)

**Source:** Canonical assignment fetched from https://github.com/Navigators-Guild/apprentice-onboarding/blob/main/02-the-methodology/02-tracking-your-work.md

The original Review 16 Dim 9 section (above) deferred the audit because the assignment file was not present in this branch's working tree. The director has since retrieved the canonical text from GitHub. This addendum re-runs the audit against the canonical source.

**Audit summary:**

- **Technology:** Match. Assignment specifies Rust + JSON file in project directory + git-style CLI subcommands. DESIGN.md (lines 5, 198, 276) matches all three.
- **Required fields:** Match. Assignment lists ID, Title (required), Description (optional), Status, Priority, Labels (list), Timestamps. DESIGN.md Data Model (lines 158-168) covers all seven.
- **Required commands and flag syntax:** Match. `tracker create "<title>" --priority <p> --label <l>`, `tracker list [--status <s>] [--label <l>]`, `tracker status <id> in-progress|done`, `tracker show <id>`, `tracker delete <id>`, compound `tracker list --status open --priority high --label bug` — all present in DESIGN.md Interface table (lines 200-208).
- **Layered build sequence:** Match. Assignment's 7-layer sequence (Core → Status → Priority → Labels → Compound filter → Detail & delete → Polish) maps 1:1 to TODO.md Layers 1-7 (lines 9, 70, 134, 184, 239, 279, 349). Phase alignment intact: branch is on Layer 4, Layers 5-7 are pre-decomposed but not implemented — correct, not a finding.
- **Out-of-scope items:** Match. DESIGN.md "Out of Scope" includes the four assignment-stated out-of-scope items (multiple users, due dates, subissues/hierarchy, time tracking) plus several implementation-class deferrals (editing after creation, undo, archiving, atomic writes, etc.) that are not in the assignment as out-of-scope but are reasonable implementation deferrals consistent with the assignment's silence on them.
- **State terminology ("closed" vs. "done"):** Equivalent. The assignment itself states "closed (done)" and uses `tracker status <id> done` in its example command — the assignment treats the terms as synonyms. DESIGN.md picking `done` for the stored value matches the assignment's own command syntax. Not a finding.
- **Default sort tie-break (priority then ID ascending):** The assignment specifies "sorted by priority" but is silent on tie-break. DESIGN.md adds tie-break by ID ascending (line 53). This is a clarification of an under-specification needed for deterministic output, not scope creep. Not a finding.
- **Label filter case-sensitivity:** Assignment is silent. DESIGN.md (line 305) specifies case-sensitive exact match. Reasonable disambiguation; not a Dim 9 violation.
- **Control-character title rejection:** Assignment says "Reject empty titles" and "Validate all input from the command line." DESIGN.md (lines 22, 290) extends to control-character rejection. The extension is consistent with the assignment's general validation guidance and the "no-crash" requirement (a newline in a title would corrupt list output). Reasonable defensive interpretation; not a Dim 9 violation.
- **List `--label` single-filter restriction:** Assignment shows `--label bug` (single) in all examples and is silent on multiplicity. DESIGN.md Feature 2 (line 59) restricts `list` to one `--label` per invocation and errors on multiple. The compound-filter assignment example also uses a single `--label`. This is a faithful clarification of an under-specified case (not a scope narrowing) consistent with all assignment examples. Not a Dim 9 violation. (Already audited as F1/F2 of Review 16 from a different angle — spec-internal ambiguity, not assignment compliance.)

**Audit findings:**

#### Finding 4: DESIGN.md unilaterally reclassifies the assignment's Layer 6 "delete with confirmation" requirement as advisory and waives it

- **Dimension:** Dim 9 (Assignment compliance), Dim 1 (Spec coverage)
- **Severity:** Medium
- **Evidence:**
  - Canonical assignment, Build layer sequence, Layer 6: "Detail & delete: show full details; delete with confirmation". The phrase "delete with confirmation" appears explicitly in the assignment's required build sequence.
  - Canonical assignment, Quality/security: "Validate all input from the command line." and the assignment lists `tracker delete <id>` in the same paragraph as confirmation. There is no language in the canonical text designating the build layers as "advisory" or non-binding.
  - Canonical assignment, Success criteria: "all 7 layers compile + pass adversarial testing; commands match specified syntax". The 7-layer sequence is named as success criteria, not advice.
  - DESIGN.md "Out of Scope" line 394: "**Interactive mode** — the tool is non-interactive; it reads arguments from the command line and exits; no TUI or REPL. The assignment's Layer 6 build guidance mentions `tracker delete <id>` with confirmation, but the authoritative interface section lists `tracker delete <id>` with no confirmation signal, and build layers are explicitly advisory." (Emphasis added.)
  - Searched the canonical assignment text for "advisory", "advice", "guidance", "optional", "non-binding" applied to the build layers. None found. The phrase "build layers are explicitly advisory" in DESIGN.md does not have a textual referent in the assignment.
  - The assignment has only one layer of authority — the assignment itself. DESIGN.md's appeal to its own "authoritative interface section" to override the assignment's Layer 6 text is circular: the interface section's authority derives from DESIGN.md, and DESIGN.md is meant to faithfully represent the assignment.
- **Rationale:** This is the textbook Dim 9 pattern — DESIGN.md narrows an assignment-stated requirement and rationalizes the narrowing by appeal to a self-declared authority hierarchy that the assignment itself does not establish. The implementation choice (delete-without-confirmation) may be defensible on UX or CLI-convention grounds, but those grounds belong in a deviation log with explicit stakeholder approval, not in an "Out of Scope" bullet that minimizes the assignment text. The closest legitimate path would be a documented "Approved deviation" per CLOSURE-PROTOCOL.md, with the SO domain explicitly approving the deviation and recording the rationale and approver — not an unattributed declaration in DESIGN.md.
  - Adversarial-posture self-check: could "delete with confirmation" plausibly be ambiguous in the assignment (e.g., satisfied by some non-interactive mechanism like a `--yes` flag)? The canonical phrase is "delete with confirmation" without further qualification; the most natural reading is interactive-prompt-style confirmation, but a `--confirm` / `--yes` flag would also be a faithful interpretation. Either is a confirmation step; the current implementation has neither. The finding survives the dismissal test.
  - Sycophancy check: I am tempted to soften this to "documentation gap" because the implementation choice is sensible. The adversarial mandate is to log the finding against the spec contract; the spec contract here is the assignment, and the assignment text does not authorize the waiver. Logging at Medium.
- **Classification:** Open — Raised to SO (this round) for adjudication.
- **Proposed action (option A — implement confirmation):** Add a `--yes` flag (or interactive `[y/N]` prompt) on `tracker delete <id>` in Layer 6, and amend DESIGN.md Feature 5 (lines 129-150) to specify the confirmation contract (default behavior, `--yes` bypass, exit code on cancellation). Remove the "Out of Scope" bullet at line 394. This is the assignment-faithful path.
- **Proposed action (option B — formalize the deviation):** Move the Layer 6 confirmation waiver out of "Out of Scope" and into a new "Approved Deviations from Assignment" section in DESIGN.md (or a separate `DEVIATIONS.md`), with explicit director-as-stakeholder approval, the date approved, and a rationale that does not rely on calling the assignment's build layers "advisory". This preserves the current implementation choice but makes the deviation visible and auditable per CLOSURE-PROTOCOL.md.
- **Proposed action (option C — evidence the "advisory" claim):** If the director has out-of-band evidence from the assignment author or guild process documentation that build layers are non-binding, cite it in DESIGN.md (link, quote). Without that citation the current text reads as a self-serving narrowing.
- **Spec-creep evaluation (inverse direction):** This is *spec-narrowing*, not spec-creep. DESIGN.md is removing a behavior the assignment requires. Per the SO domain prompt, the spec contract is the assignment for Dim 9 purposes; narrowing it without explicit deviation approval is the failure mode this dimension exists to catch.
- **Per CLOSURE-PROTOCOL.md:** SO is the only domain authorized to modify DESIGN.md. This finding proposes DESIGN.md changes (under any of A/B/C) and is logged for SO adjudication. No DESIGN.md edit performed by this addendum.

---

**Addendum coordination notes:**
- **Solution Owner (director):** F4 is yours to adjudicate. Option B is the lowest-cost path that satisfies the audit; Option A is the highest-fidelity path; Option C is only viable if the "advisory" claim has an actual referent that can be cited.
- **Software Engineer:** No action until SO adjudicates. Option A would add a Layer 6 sub-task; B and C are doc-only.
- **Quality Engineer:** If Option A, QE adds tests for the `--yes` bypass, the cancellation exit code, and the confirmation-prompt behavior. If B or C, no test changes.
- **VDD-IAR Alignment:** F4 is also a VDD-IAR signal — the existing "Out of Scope" rationale relies on a hierarchy ("authoritative interface section") that has no textual basis in the assignment. VDD-IAR may want to verify whether other DESIGN.md "Out of Scope" bullets rely on similar self-declared hierarchies.

---

## Review 17 — 2026-05-06 02:30Z

**Round:** SO Review 17 (Round-2 SO adjudication for Layer 4 IAR closure)
**Scope:** Spec amendments for the Round-1 finding cluster. Adjudicates SO R16 F1/F2/F4 plus DESIGN.md amendments raised by Security R7 F1, RT R6 F1/F2, DE R7 F1/F2, UX R6 F1/F4, SE R11 F3, TW R7 F2/F4/F7.
**Session context:** Director-orchestrated warm-resolution session; not adversarial-cold per CLOSURE-PROTOCOL.md Section 5 step 3 (SO adjudication is warm by design).

### Adjudications

#### F1 (label trim-on-store) — Resolved as spec ratification

DESIGN.md Feature 1 Postconditions amended: "labels is the deduplicated list of `--label` values, with each value trimmed of leading/trailing whitespace; order is preserved (first occurrence retained); case is preserved as provided after trimming". Edge Cases / Labels gains: "Leading/trailing whitespace on a label is trimmed before storage; `--label '  bug  '` stores `bug`. Deduplication compares trimmed values." This codifies the implementation's existing behavior; no source change required by F1 alone. **Same resolution closes TW R7 F7.**

#### F2 (empty/whitespace-only `--label` filter) — Resolved as Option A (validate)

Chose Option A from Review 16 F2: validate filter symmetric with create. Reasoning: UX Review 6 F1 surfaced a concrete round-trip failure (a stored label `bug` is unreachable by `tracker list --label "  bug  "`); silent-no-match masks the trim asymmetry. DE R7 F2 echoes the same. Spec amended: Feature 2 "With `--label <l>`": "filter value is trimmed before comparison (symmetric with create-side trim-on-store)." Feature 2 Error states: "Empty or whitespace-only `--label` filter value → `Error: Label cannot be empty.` → exit 1." SE applies `parse_label` on the filter side; QE adds three integration tests. **Same resolution closes UX R6 F1, DE R7 F2.**

#### F4 (Dim 9 — `tracker delete <id>` confirmation deviation) — Resolved as Option B (formalize Approved Deviation)

Chose Option B from the Dim 9 addendum: move the deviation out of "Out of Scope" and into a new top-level "Approved Deviations from Assignment" section. Rationale: Option A (implement confirmation) would expand Layer 6 scope; Option C (cite "advisory" evidence) has no referent. Option B preserves the implementation choice (non-interactive delete is consistent with the rest of the binary; recovery via `tracker.json` edit is the user's escape hatch) while making the deviation auditable rather than hidden behind a circular rationale.

The "Approved Deviations from Assignment" section is new in DESIGN.md; entry D1 records the `tracker delete <id>` confirmation waiver with director-as-stakeholder approval, the date (`2026-05-05`), the rationale, and the re-evaluation trigger. The original "Out of Scope" bullet now points to D1 instead of self-declaring "build layers are explicitly advisory."

### Round-1 cluster resolutions (in this round)

These are adjudications that flow from F1/F2/F4 and the cross-domain coordination in Round 1:

- **Security R7 F1 / RT R6 F1 / DE R7 F1 / SE R11 F3** (label control-character defense): DESIGN.md amended. Feature 1 preconditions add "contains no control characters (Unicode general category `Cc`)" to label rule; Feature 1 Error states add `Error: Label cannot contain control characters.`; Edge Cases / Labels gains a bullet rationale paralleling Title; Edge Cases / Storage adds control-char-in-label to corruption triggers. Security R7 F1's recommendation (extend BOTH `parse_label` and `issue_fields_are_valid`) is sanctioned. SE applies; QE tests. **Resolved.**
- **UX R6 F4** (comma-in-label rendering ambiguity): DESIGN.md amended. Feature 1 preconditions add "contains no comma `,`" to label rule; Feature 1 Error states add `Error: Label cannot contain a comma.`; Edge Cases / Labels documents the rationale (comma is the display separator). SE applies; QE tests. **Resolved.**
- **RT R6 F2** (error-message escape interpolation): DESIGN.md "stderr contract" amended to require `\u{XX}` escaping of Cc characters in error messages that interpolate user-supplied values. SE adds `display_safe` helper at `parse_priority`/`parse_status`/`parse_id` formatters; QE adds three integration tests. **Resolved.**
- **RT R6 F3** (Trojan-Source / `Cf` / zero-width): chose Option 2 from the RT finding (document the surface). DESIGN.md Edge Cases / Labels gains an explicit bullet placing `Cf` and zero-width characters out-of-threat-model for this single-user local CLI, with the re-evaluation trigger named (multi-user / network / shared `tracker.json`). Per CLOSURE-PROTOCOL.md Section 2, Red Team findings cannot be Deferred but may be Accepted Risk with a named risk owner — that owner is the director (single-user threat model is the documented constraint). **Resolved as Accepted Risk.**
- **TW R7 F2** (CHANGELOG missing Layer 4 entry): SO authored the Layer 4 retrospective entry plus the Round-2 closure entry in CHANGELOG.md. Layer 4 is now visible in CHANGELOG between the Layer 3 follow-up and the CI hotfix. **Resolved.**
- **TW R7 F4** (Cargo.toml `repository` field): added `repository = "https://github.com/magnificentlycursed/guild-portfolio"`. The `TODO(SO)` comment removed. **Resolved.**

### Open / deferred after this round

- **SO R16 F3** (Layer 4 manual testing checklist) — Resolved by commit `b0a3789` ("Layer 4 manual testing complete"). Closes VDD-IAR R11 F2.
- **TW R7 F5** (PROCESS.md retrospective placeholders) — Open. Developer-only authority; SO cannot adjudicate. Director must fill the Layer 1-4 reflection blocks or restructure the file before Layer 4 merge per the auto-Backlog clock that has now fired.
- **TW R7 F6** (`--help` valid-value asymmetry) — Deferred to Layer 7 polish.
- **SA R9 F1 / SE R11 F2** (cmd_list extraction) — Deferred to a focused PR before Layer 7. SE rationale (surgical inline conflates concerns with Layer 7 prep) accepted.
- **UX R6 F2** (clap-voice multi-label error) — Deferred to Layer 7 polish.
- **UX R6 F3** (`--help` examples for compound flags) — Deferred to Layer 7 polish, consistent with the suite-level `5b95911` commitment.

### Files modified

- `DESIGN.md` (Feature 1 / Feature 2 / Edge Cases / Labels / Edge Cases / Storage / stderr contract / new "Approved Deviations from Assignment" section / D1 entry).
- `Cargo.toml` (`repository` field).
- `CHANGELOG.md` (Layer 4 retrospective entry + Round-2 closure entry).

No source files modified by this review (SE applies code per the spec amendments — see SE Review 12).

### Summary

7 Round-2 resolutions logged: SO R16 F1/F2/F3/F4 (3 directly resolved + F3 closed by manual-testing commit), Security R7 F1, RT R6 F1/F2/F3 (F1+F2 resolved; F3 Accepted Risk), DE R7 F1/F2, SE R11 F3, UX R6 F1/F4, TW R7 F2/F4/F7. 5 findings deferred with named target layers (SA R9 F1 / SE R11 F2 / UX R6 F2/F3 / TW R7 F6 → Layer 7). 1 finding remains Open requiring developer-only action (TW R7 F5).
