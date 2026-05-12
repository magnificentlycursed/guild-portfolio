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

---

## Review 18 — 2026-05-07 00:23Z

**Round:** SO Review 18 (Layer 5 — Compound Filtering)
**Scope:** Layer 5 (status × priority × label AND-combination + filter-empty-state messaging) primary. Three commits to evaluate: `7d1ca57` (Phase 2a Red Gate — `issue_matches_filters` `todo!()` stub + 5 unit tests + 7 integration tests), `bd15a9d` (Phase 2b — predicate body + `cmd_list` retain refactor), `da0fd8d` (manual testing checklist closure). Secondary: spot-check that no Layer 6/7 surface leaked in.
**Reference:** `DESIGN.md` Feature 2 (lines 51-82, 316-322), `TODO.md` Layer 5 (lines 239-275), assignment brief Layer 5 ("Compound filter").
**Session note:** Cold session, fresh subagent. Parallel-batch IAR run on branch `issue-tracker-cli-compound-filtering` with SA Review 11, QE Review 13, SE Review 13, VDD-IAR Review 13. SO did not participate in Layer 5 scoping or implementation.

---

### Compliance table — Layer 5 acceptance criteria

| AC (TODO.md lines 244-251) | Status | Evidence |
|---|---|---|
| `--status open --priority high` shows only open AND high-priority | Met | `tests/layer5.rs:28-73` `list_status_and_priority_filter_and_combination`; predicate `src/lib.rs:425-434`. |
| `--status open --label bug` shows only open with label `bug` | Met | `tests/layer5.rs:75-117` `list_status_and_label_filter_and_combination`. |
| `--priority high --label bug` shows only high + bug | Met | `tests/layer5.rs:119-181` `list_priority_and_label_filter_and_combination`. |
| `--status open --priority high --label bug` shows only triple-match | Met | `tests/layer5.rs:185-279` `list_three_filter_and_combination`. |
| 2/3-match issue does NOT appear | Met | Same test — three negative assertions, one per single-filter mismatch. |
| `--status done --priority low` no-match → `No issues match the given filters.` | Met | `tests/layer5.rs:283-310` `list_compound_two_filter_no_match_shows_filter_message`; asserts stdout empty + stderr message + absence of `Nice work!`. |
| `--status open --priority high --label nonexistent` no-match → filter message | Met | `tests/layer5.rs:312-350` `list_compound_three_filter_no_match_shows_filter_message`. |
| `tracker list` (default, opens exist) → no `No issues match` message | Met | `tests/layer5.rs:352-384` `list_default_view_with_open_issues_does_not_show_filter_message`; asserts `Nice work!` and `No issues match` both absent. |
| Predicate `issue_matches_filters` AND-combines status/priority/label | Met | `src/lib.rs:431-433` — `issue.status == status && priority.is_none_or(|p| issue.priority == p) && label.is_none_or(|l| label_matches(...))`; 5 unit tests `src/lib.rs:867-930`. |
| Manual testing checklist (TODO.md lines 255-261) | Verified | All six items flipped to `[x]` in commit `da0fd8d`; expected outputs match the implementation under spot-trace (see Finding 2). |
| `Cargo.toml` unchanged (no new deps) | Met | `git diff 921525d..HEAD -- issue-tracker-cli/Cargo.toml issue-tracker-cli/Cargo.lock` returns empty. |
| Test count regression check | Met | `cargo test --no-fail-fast --locked` → 32+18+9+25+7 integration + 44 unit = 135 passing, 0 failing. |
| No Layer 6+ surface (no `show`, no `delete`, no `--description`) | Met | `src/main.rs:11-43` enum still `Create | List | Status`. |

All eight Layer 5 ACs are Met. Manual checklist closed. No new dependencies. No Layer 6/7 leakage. Test count is 135/135.

---

### Findings

#### Finding 1: `cmd_list` comment cites `--description-contains` as a future Layer 6 filter, contradicting DESIGN.md "Out of Scope" exclusion of text search

- **Dimension:** Dim 1 (Spec coverage), Dim 2 (Scope creep — anticipatory), Dim 7 (Design fidelity)
- **Severity:** Low
- **Evidence:**
  - `src/lib.rs:489-490`: "Disjunction over non-default filters: any future filter (e.g. Layer 6's `--description-contains`) must extend `extra_filter_active` here…"
  - DESIGN.md "Out of Scope" line 397: "**Search by text** — no full-text search across titles or descriptions; filtering is by exact-match status, priority, and label only".
  - DESIGN.md TODO.md Layer 6 (lines 279-345): no `--description-contains` flag is named in any AC; Layer 6 adds `tracker show`, `tracker delete`, and `--description` on `create` only — no description filter.
  - The bd15a9d commit message reinforces the same aspirational direction: "future filters (Layer 6's optional --description-* flag and beyond) extend one place rather than appending another retain".
  - The comment was added in an earlier layer (preexisting at the start of the Layer 5 diff), so this is not Layer 5's new creep — but Layer 5's commit message ratifies the same anticipated extension.
- **Rationale:** A code comment that names a feature DESIGN.md excludes is anticipatory scope creep at the documentation level. Future-proofing for a feature the spec forbids ("filtering is by exact-match status, priority, and label only") signals to the next reader that the author considers the exclusion soft. This is the same defect class SO has caught before (e.g., the `tracker delete <id>` confirmation rationalization, SO R16 F4): a self-declared eventuality not anchored in the spec. The code is correct; the comment misrepresents what the design space allows.
- **Classification:** Open — Raised to SO (this round) for code-comment cleanup. Carry-forward candidate, not a Layer 5 blocker.
- **Proposed action:** Edit `src/lib.rs:489-490` to remove the `--description-contains` example, or replace it with a Layer-correct example (e.g., a hypothetical future `--created-after` filter framed as "if the spec is ever amended to add a new filter"). Edit the bd15a9d commit narrative if a follow-up amendment lands; otherwise note the carry-forward in CHANGELOG. No DESIGN.md change required — DESIGN.md is correct; the code comment is the deviation.
- **Spec-creep evaluation:** Anticipatory creep at the comment level, which is the lowest-cost form to clear. The implementation itself is in scope.

---

#### Finding 2: Integration test docstring claims an in-progress setup issue exists, but the setup creates only open issues

- **Dimension:** Dim 4 (Test obligations — test/comment fidelity)
- **Severity:** Low
- **Evidence:**
  - `tests/layer5.rs:124-125`: "Note this exercises the non-default-status path: with no --status flag, effective_status is 'open' and only open issues participate; one of the setup issues is in-progress to confirm it is filtered out by the implicit status default."
  - `tests/layer5.rs:127-159`: setup creates three issues (`Match all`, `Wrong priority`, `Wrong label`) via `tracker create` only — no `tracker status` invocation. All three start at default status `open`. None are in-progress.
  - The test still passes because the AND-logic correctly excludes `Wrong priority` (low) and `Wrong label` (feature) from the `--priority high --label bug` filter result. The defect is in the docstring, not the assertion.
- **Rationale:** A future maintainer reading the comment will look for an in-progress setup step that does not exist, then either add one (changing test semantics) or be confused. This is a small but real test-doc fidelity defect that was missed during code review of the Red Gate commit. It does not affect AC coverage — the test still establishes the AND-combination — but the stated intent ("confirm an in-progress issue is filtered out by the implicit status default") is not actually tested anywhere in `tests/layer5.rs`.
- **Classification:** Open — coordination item for QE Review 13 (test-side ownership). SO surfaces it as a Dim 4 fidelity gap; QE may resolve directly without DESIGN.md or `TODO.md` changes.
- **Proposed action:** Either (a) add a fourth setup issue and a `tracker status N in-progress` call so the comment matches the setup, or (b) trim the comment to drop the "one of the setup issues is in-progress" clause. Option (b) is the cheaper resolution; option (a) modestly strengthens the test by exercising the implicit-status-default path explicitly.

---

#### Finding 3: Manual testing checklist setup wording does not specify the `tracker status` step required to produce the `(done, high, bug)` issue

- **Dimension:** Dim 1 (Spec coverage — checklist precision), Dim 9 (Assignment compliance — manual testing methodology)
- **Severity:** Low
- **Evidence:**
  - `TODO.md:256`: "Setup: create four issues — `(open, high, bug)`, `(open, medium, bug)`, `(done, high, bug)`, `(open, high, feature)` — then run each filter combination and verify only the correct issue(s) appear".
  - `tracker create` always produces `status: "open"` per DESIGN.md Feature 1 postcondition (line 26). To produce `(done, high, bug)` the user must also run `tracker status 3 done` after creating the third issue. The checklist instruction does not name this step.
  - The commit `da0fd8d` ticks the box without elaboration. A literal reading of the wording — "create four issues" — matches a four-`tracker create` workflow, which would produce four open issues, not the `(done, ...)` mix the subsequent items assume.
  - Compare prior layers' manual checklists (TODO.md Layer 2 / Layer 3 / Layer 4): each is explicit about each command needed. This Layer 5 entry is more terse and assumes the reader infers the status-change step.
- **Rationale:** A manual testing checklist is the contract between developer and reviewer for the human-verification gate (DESIGN.md Testing Methodology line 373: "Each layer must be manually tested before the layer gate closes"). When the wording elides a required step, the gate becomes ambiguous: did the developer execute the implied step, or just the literal one? The actual filter-output expectations downstream of the setup (e.g., `--status open --priority high → issues #1 and #4 only` at TODO line 257) are only correct if issue #3 is `done`, so the elision is benign in practice — but the checklist as written is under-specified for a future reviewer reproducing the steps from scratch.
- **Classification:** Open — Raised to SO for adjudication. The defect is in the checklist text, not in the implementation or the commit's ticking decision (which is consistent with the developer having executed the obvious additional `tracker status 3 done` step).
- **Proposed action:** Amend `TODO.md:256` to spell out the setup explicitly. Suggested wording: "Setup: `tracker create "..." --priority high --label bug` (×1), `tracker create "..." --priority medium --label bug` (×1), `tracker create "..." --priority high --label bug` then `tracker status 3 done` (×1), `tracker create "..." --priority high --label feature` (×1)". Lowest-cost resolution; preserves the developer's existing tick and just records the reproducible command sequence.
- **Spec-creep evaluation:** Documentation precision, not creep. Faithful elaboration of the existing checklist intent.

---

### Spec-creep audit (Dim 2 — additions not in DESIGN.md)

Walked the Layer 5 diff (`921525d..HEAD`) for additions not described in DESIGN.md or TODO.md Layer 5:

- **`issue_matches_filters` predicate** (`src/lib.rs:425-434`): directly required by DESIGN.md Feature 2 lines 63 + 321 (AND-combination) and the Layer 5 Red Gate plan (TODO.md lines 271-273). Not creep.
- **`cmd_list` retain refactor** (single retain over the predicate, replacing three chained retains): structural refactor with unchanged behavior; the bd15a9d commit message documents the equivalence and the regression check confirms 135/135 passing. Per SO Review 16/17 the prior chained-retain form was already in place; consolidating is not new feature surface. Not creep.
- **5 new unit tests** (`src/lib.rs:867-930`): all five exercise `issue_matches_filters` directly. Each maps to the Red Gate plan's two named unit tests (`filter_and_logic_all_must_match`, `filter_and_logic_all_present_returns_true`) plus three additional sub-cases that surface predicate-level corollaries (status-only matches anything, status mismatch rejects regardless of optional filters, label-match is case-sensitive). The three extras lean toward over-test by the strict TODO.md Red Gate plan, but each kills a distinct mutation (drop status conjunct, drop label conjunct case-sensitivity) and the marginal cost is trivial (~60 LOC across all five). Within tolerance for Dim 4; not creep.
- **7 new integration tests** (`tests/layer5.rs`): 4 are named in the Red Gate plan (`list_two_filter_and_combination`, `list_three_filter_and_combination`, `list_no_match_shows_filter_message`, `list_default_does_not_show_filter_message`); 3 are sub-decompositions that split the AC pairs and the no-match cases. The Red Gate plan calls for 4; the actual count is 7. The extra three each exercise a distinct AC-pair (status×priority, status×label, priority×label, three-way) so the over-coverage is shaped to the AC count (8 ACs, 7 tests + 1 manual setup), not gold-plating. Cat B Red Gate disposition (see audit below) is honest. Slight over-test; well within tolerance.
- **No new dependencies** in `Cargo.toml` or `Cargo.lock`. Confirmed unchanged by `git diff`.
- **No Layer 6/7 surface leak.** `src/main.rs` enum is still `Create | List | Status`. No `show`, no `delete`, no `--description`, no `--help` polish, no color output.

No spec creep detected at the implementation level. The single anticipatory comment in `cmd_list` referencing `--description-contains` is logged separately as Finding 1.

---

### Cat B Red Gate disposition audit

The 7 integration tests in `tests/layer5.rs` are classified as "Cat B Red Gate deviations" because the AND-combination already worked at Phase 2a (the chained retains in `cmd_list` from Layers 3+4 produced the AND emergent behavior). The 5 unit tests on `issue_matches_filters` panic at Phase 2a (the `todo!()` stub) and pass at Phase 2b (the real predicate body), so they are the genuine Cat A Red Gate for Layer 5.

Auditing for honesty:

- **Is the Cat B claim factually correct?** Yes. The Phase 2a commit `7d1ca57` did not modify `cmd_list`; the integration tests at Phase 2a invoked the binary which still went through chained retains. The integration tests passed at Phase 2a — confirmed by the commit message's explicit statement and reproducible via `git checkout 7d1ca57 && cargo test --test layer5`.
- **Is this consistent with prior layers' dispositions?** Yes. Layer 3's `create_without_priority_defaults_to_medium` and Layer 4's two Cat B deviations (commit `14bd219`) follow the same pattern: a layer's AC is genuinely emergent from a prior layer's implementation, the integration test is regression coverage rather than Red Gate gating, and the layer-specific genuine Red Gate is a unit-test-level abstraction that didn't exist before. Same shape here.
- **Does it paper over a Phase 2a violation?** No. A Phase 2a violation would be: a behavior named in Layer 5's ACs that exists at Phase 2a but does not have a unit test failing against the Phase 2a state. Here, every AC pair has a Phase-2a-failing unit test against `issue_matches_filters`, which is the abstraction Layer 5 introduces. The integration tests are *additional* regression coverage, not the only assertion of the AC.
- **Could the Cat A unit tests have been written without introducing the predicate?** Only by testing through `cmd_list` (an integration shape) or by inlining the AND-logic as a free function the tests could call. Either alternative would either degrade testability or duplicate the chained-retain logic in a function-body extracted-from-`cmd_list` form — which is exactly what Phase 2b does. So the predicate extraction is the natural Cat A scaffold, and committing it as a `todo!()` stub at Phase 2a is the correct Red Gate shape.

The Cat B disposition is honest. No Phase 2a violation.

---

### Hallucinated

*(none this round)*

### Backlogged

*(none this round)*

### Dismissed

*(none this round — F1 / F2 / F3 are real low-severity findings, not hallucinations)*

### Open

- **F1** (anticipatory `--description-contains` comment) — code-comment cleanup; coordination with SE.
- **F2** (test-comment claims in-progress setup that doesn't exist) — coordination with QE.
- **F3** (manual testing checklist setup elides `tracker status` step) — SO doc-only fix to `TODO.md`.

### Carry-forward (from prior rounds)

- **F8 (Review 15)** — `Cargo.toml` `repository` field — Resolved post-R17 (the file now has `repository = "https://github.com/magnificentlycursed/guild-portfolio"` per R17 closure entry). Status: **Resolved**, retroactively. No action this round.

---

### Summary

3 Open findings, all Low severity. 0 Hallucinated. 0 Dismissed. 0 Backlogged. 0 Approved deviations.

The Layer 5 implementation matches all 8 acceptance criteria with passing tests; the Cat B Red Gate disposition is honest and consistent with prior-layer practice; no new dependencies; no Layer 6/7 surface leakage; manual testing checklist closed. The compliance table is clean at the implementation level.

The three findings are documentation-class:
- F1 is an anticipatory code comment that names an out-of-scope feature.
- F2 is a test docstring that describes a setup step that doesn't exist.
- F3 is a manual testing checklist that elides a required `tracker status` step.

None block Layer 5 closure. All are within reach of a single-PR cleanup. F1 + F2 are SE/QE coordination items; F3 is SO-authority (TODO.md).

**Sycophancy check (self-applied):** A clean Layer 5 review is the failure mode the cold-session prompt warns about. I tested whether the three findings are real by trying to dismiss each:

- **F1 dismissal attempt:** "The comment is harmless aspirational future-proofing." Counter: the spec explicitly excludes the feature ("filtering is by exact-match status, priority, and label only"). A comment that names an excluded feature as a Layer 6 expectation contradicts the spec's binding text. The bd15a9d commit message ratifies the same direction, doubling the signal. Real finding.
- **F2 dismissal attempt:** "Test docstrings are low-stakes; the test still asserts the right behavior." Counter: the test is correct; the docstring is wrong. A future maintainer reading the docstring will look for the in-progress setup step that doesn't exist and either change the test (regressing coverage) or be confused. The cost to fix is one line; the cost to leave is one future code-review distraction. Real finding, low severity.
- **F3 dismissal attempt:** "Manual testing was executed correctly; the developer obviously knew the `tracker status` step was implied." Counter: the manual checklist is the contract between developer and external reviewer for the gate-close. The developer's tacit knowledge does not document the gate. Compare prior layers' checklists, which spell out each step explicitly — the Layer 5 wording is genuinely terser. Real finding, low severity.

Each finding survived the dismissal test. The clean compliance table is genuine; the three findings are the residue.

**Coordination:**
- **Software Engineer (SE Review 13):** F1 is the code-side change (one comment edit at `src/lib.rs:489-490`). SE may resolve directly; no DESIGN.md change required.
- **Quality Engineer (QE Review 13):** F2 is the test-side change. QE may resolve directly via either (a) trim the comment or (b) add the missing setup step. SO has no preference; QE owns the call.
- **Solution Architect (SA Review 11):** SA may have an architectural opinion on whether the predicate extraction is a valuable layer 7 prep step (it co-locates the AND-logic for the Layer 7 color refactor). Not a finding for this round; flagged for SA awareness.
- **VDD-IAR Alignment (VDD-IAR Review 13):** F3 is a manual-testing-process datum — the checklist precision norm is in VDD-IAR's wheelhouse. Recommend VDD-IAR's pass note whether the Layer 5 checklist precision (terser than Layers 2-4) is an isolated deviation or a drift the project should correct.
- **Director (SO authority):** F3 requires a `TODO.md` edit. SO is the only domain that may edit `TODO.md` directly per CLOSURE-PROTOCOL.md authority. Recommended action: in the Round-2 closure pass, edit the Layer 5 manual checklist setup line to enumerate the four-step command sequence including the `tracker status 3 done` step.

The Layer 5 implementation is in spec compliance and ready to merge after the documentation cleanup above. No DESIGN.md amendments required this round.

---

## Review 19 — 2026-05-07 00:39Z

**Round:** SO Review 19 (Round-2 closure for Layer 5 — Compound Filtering)
**Scope:** Verify Round-1 inline findings (F1, F2, F3) are resolved by commit `7f9bae4`. No new substantive review pass; this is a warm closure-verification per CLOSURE-PROTOCOL Section 5 step 3.
**Session context:** Director-orchestrated warm-resolution session; not adversarial-cold by design.

### Round-1 finding closures

- **F1 (anticipatory `--description-contains` comment):** **Resolved.** `src/lib.rs:489-497` now reads "any new filter the spec is amended to add" with an explicit DESIGN.md "Out of Scope" citation. The prior anticipated-Layer-6-feature framing is gone; no out-of-scope feature is named. Verified by reading the post-`7f9bae4` diff.
- **F2 (test-comment claims in-progress setup that does not exist):** **Resolved.** `tests/layer5.rs:121-125` docstring trimmed to drop the false in-progress claim; replaced with an accurate description of the AND-combination across the priority and label filters under the implicit `--status open` default. Test assertions unchanged; 7/7 Layer-5 integration tests still pass.
- **F3 (manual checklist setup elides `tracker status` step):** **Resolved.** `TODO.md:256` rewritten to enumerate each `tracker create` invocation and the `tracker status 3 done` step required to produce the `(done, high, bug)` issue. Matches the explicitness of Layers 2-4 manual checklists.

### Hallucinated / Backlogged / Dismissed

*(none this round)*

### Open

*(none from SO domain. SA R11 F1 — rendering half of `cmd_list` extraction — remains the only Layer 5 Open finding across the suite, deferred to a focused pre-Layer-7 PR per SA R10 / SA R11 disposition. SO has no objection to that deferral.)*

### Summary

3/3 Round-1 SO findings Resolved by commit `7f9bae4`. 0 new findings this round. Layer 5 SO compliance is closed. Layer 5 ready to merge from the SO lens once VDD-IAR R14 confirms the suite-level merge gate.

**Coordination:** *(none — closure pass)*

---

## Review 20 — 2026-05-11 01:07Z

**Round:** SO Review 20 (Layer 6 — Description + Show + Delete)
**Scope:** Layer 6 spec compliance, scope creep, over-engineering, under-delivery, technology choices, assignment compliance. Two commits to evaluate: `4fb5e67` (Phase 2a Red Gate — 20 integration tests + 3 unit tests + 4 `todo!()` stubs: `validate_description`, `format_show_block`, `cmd_show`, `cmd_delete`) and `c91676a` (Phase 2b — all four stubs replaced; `cmd_create` wires `--description` through; TODO.md ACs flipped).
**Reference:** `DESIGN.md` Feature 1 / Feature 4 / Feature 5 / Data Model / Interface / "Show output format" / Edge Cases / Description / D1 ("Approved Deviations from Assignment"); `TODO.md` Layer 6 (lines 279-345); assignment Layer 6 ("Detail & delete: show full details; delete with confirmation").
**Session note:** Cold session, fresh subagent. SO did not participate in Layer 6 scoping or implementation. `cargo test --no-fail-fast --locked` → 159 passing, 0 failing.

---

### Compliance table — Layer 6 acceptance criteria

| AC (TODO.md lines 283-301) | Status | Evidence |
|---|---|---|
| `--description "..."` stores verbatim | Met | `tests/layer6.rs:23-41` `create_with_description_stores_verbatim`; `src/lib.rs:335-340` `validate_description` returns input un-trimmed; `cmd_create` writes the value at `src/lib.rs:237-240`. |
| `--description ""` → empty error exit 1 | Met | `tests/layer6.rs:43-57` `create_with_empty_description_exits_one`. |
| `--description "  "` → empty error exit 1 | Met | `tests/layer6.rs:59-73` `create_with_whitespace_description_exits_one`. |
| no flag → description absent in JSON (not null/empty) | Met | `tests/layer6.rs:75-93` `create_without_description_has_no_field_in_json` asserts `obj.contains_key("description") == false`; `src/lib.rs:39` `#[serde(skip_serializing_if = "Option::is_none")]`. |
| `tracker show 1` displays all fields | Met | `tests/layer6.rs:97-155` `show_displays_all_fields` asserts each of the eight labels; `src/lib.rs:350-387` `format_show_block`. |
| 13-char right-padded label column | Met | Unit test `src/lib.rs:1101-1126` `show_label_column_right_padded_to_13`; `format_show_block` literal format string. |
| multi-line description: continuation indented 13 spaces | Met | Unit test `src/lib.rs:1084-1099` `multiline_description_show_format`; integration `tests/layer6.rs:189-219` `show_multiline_description_indents_continuation`; `src/lib.rs:365-366` performs `\n` → `\n` + 13 spaces. |
| `show` full untruncated title and labels | Met | `tests/layer6.rs:221-254` `show_does_not_truncate_title_or_labels`. |
| `show abc` → invalid-ID error exit 1 | Met | `tests/layer6.rs:258-272` `show_invalid_id_string_exits_one`. |
| `show 0` → invalid-ID error exit 1 | Met | `tests/layer6.rs:274-286` `show_zero_id_exits_one`. |
| `show 99` → not-found error exit 1 | Met | `tests/layer6.rs:288-301` `show_not_found_exits_one`. |
| `delete 1` → exit 0, `Deleted issue #1.`, JSON updated | Met | `tests/layer6.rs:305-316` `delete_exits_zero_and_prints_confirmation`; `tests/layer6.rs:318-332` `delete_removes_issue`; `src/lib.rs:422-433` `cmd_delete`. |
| after `delete 1`, `show 1` → not found | Met | `tests/layer6.rs:334-349` `delete_then_show_returns_not_found`. |
| deleted ID never reused (next = `max(remaining)+1`) | Met | `tests/layer6.rs:351-375` `delete_id_not_reused`; `src/lib.rs:88-92` `next_id` returns `max+1`; unit test `src/lib.rs:1128-1138` `max_id_plus_one_skips_deleted_ids`. |
| `delete abc` → invalid-ID error exit 1 | Met | `tests/layer6.rs:412-424` `delete_invalid_id_exits_one`. |
| `delete 99` → not-found error exit 1 | Met | `tests/layer6.rs:426-438` `delete_not_found_exits_one`. |
| other issues unchanged after delete | Met | `tests/layer6.rs:377-408` `delete_other_issues_unchanged` — byte-identity assertion across pre/post snapshots. |
| description never shown in `list` | Met | `tests/layer6.rs:442-465` `description_not_in_list_output`. |
| Manual Testing Checklist (TODO.md lines 303-316) | **Unchecked** | 13/13 boxes still `[ ]`. Process state — see "Manual testing closure" below. |
| `Cargo.toml` unchanged (no new deps) | Met | `git diff 727aef9..c91676a -- issue-tracker-cli/Cargo.toml issue-tracker-cli/Cargo.lock` returns empty. |
| Test count: Layer 6 Red Gate plan = 18 int + 3 unit | Over by 2 | Actual: 20 int + 3 unit. See Cat B audit. Both extras (`create_with_whitespace_description_exits_one`, `delete_then_show_returns_not_found`) are AC-faithful coverage of ACs at TODO.md lines 286 and 296 that the Red Gate plan elided. |
| No Layer 7 surface (color, `--help` polish) | Met | `src/main.rs` adds only `Show` / `Delete` variants and `--description` on `Create`; no `IsTerminal`, no color crate, no `--help` examples beyond clap's default. |

All 18 implementation ACs are Met. The 13-item manual checklist remains unchecked — flagged as a process state for director closure before merge, consistent with the Layer 6 commit message's explicit deferral.

---

### Findings

#### Finding 1: Manual Testing Checklist (TODO.md lines 303-316) is fully unchecked at the layer gate

- **Dimension:** Dim 9 (Assignment compliance — manual testing methodology); "Manual testing closure" per the cold primer.
- **Severity:** Low (process state, not a code defect).
- **Evidence:**
  - `TODO.md:303-316`: 13 manual testing items, all `[ ]`. None are ticked by `c91676a`.
  - `c91676a` commit message explicitly states: "Manual Testing Checklist is intentionally left unchecked — VSDD Phase 2 completion requires human verification, not satisfied by automated tests."
  - `DESIGN.md:373` Testing Methodology: "Each layer must be manually tested before the layer gate closes."
  - Prior layers' closure: Layer 1-5 checklists are all `[x]` at merge time (e.g., `TODO.md:31-39` for Layer 1, `TODO.md:255-261` for Layer 5).
- **Rationale:** Consistent with the pattern from prior layers (manual checklist is ticked just before merge, not at Phase 2b commit). Surfacing the state for the director's awareness; not a Phase 2 implementation defect. The commit author is honest about leaving it pending.
- **Classification:** Open — process item for director closure before Layer 6 merge.
- **Proposed action:** Director executes the 13 manual checklist items against the merged binary, then ticks the boxes in a single TODO.md commit (matching the Layer 5 `da0fd8d` shape).
- **Spec-creep evaluation:** None.

---

#### Finding 2: `format_show_block` silently normalizes stored `\r\n` to `\n` before rendering — not specified by DESIGN.md

- **Dimension:** Dim 2 (Scope creep — silent data-display transformation), Dim 7 (Design fidelity).
- **Severity:** Low.
- **Evidence:**
  - `src/lib.rs:365`: `let normalized = d.replace("\r\n", "\n");` — every `\r\n` pair in a stored description is replaced with `\n` before the continuation-indent substitution.
  - `DESIGN.md:345` Edge Cases / Description: "Description may contain newlines (`\n`). In `show` output, the first line follows the `Description:` label; each subsequent line is indented by 13 spaces to align with the value column." The spec names `\n` only; `\r\n` is not mentioned.
  - `DESIGN.md:344` Edge Cases / Description: "Description is not trimmed; stored verbatim." This binds storage but the show *rendering* path is bound by line 345's "may contain newlines (`\n`)" phrasing, which is consistent with either (a) `\n`-only and `\r` treated as a literal byte, or (b) `\r\n` normalized. The spec does not pick.
  - The implementation comment (`src/lib.rs:361-364`) self-justifies: "`\r\n` sequences are normalized to `\n` for splitting so a CRLF-stored description renders without a stray `\r` in the first line."
  - The transformation is display-only — `tracker.json` is unchanged; subsequent `show` runs re-normalize on each render. So this is a display contract, not a storage contract.
  - Tests do not cover the `\r\n` path. `tests/layer6.rs:189-219` only exercises `"line1\nline2"`.
- **Rationale:** The spec is silent on `\r\n`. The implementation picks a reasonable behavior (defend against a CRLF-stored description from a hand-edited Windows-line-ending `tracker.json`), but the choice is not anchored in DESIGN.md. Two options exist for closure: (a) ratify the behavior with a DESIGN.md sentence ("`\r\n` is normalized to `\n` for display"), or (b) remove the normalization and let `\r` pass through as a literal character (consistent with "stored verbatim" rendering). Option (a) is the cheaper resolution and matches the implementation. Option (b) preserves the "verbatim" principle but introduces a surprise to a CRLF user.
- **Classification:** Open — Raised to SO for adjudication. The defect is a small spec gap, not an implementation bug.
- **Proposed action:** Amend `DESIGN.md` Edge Cases / Description (line 345) to add: "Stored `\r\n` sequences are normalized to `\n` for `show` display so a CRLF-stored description does not render a stray `\r` on the first line; storage is unchanged." Alternatively, remove the `\r\n` → `\n` replace and let `\r` pass through (lower-cost code change, but a worse user experience for hand-edited tracker.json on Windows).
- **Spec-creep evaluation:** Low-grade scope creep at the rendering layer — the implementation does something the spec does not specify. Categorically distinct from the SO R18 F1 anticipatory-comment creep, which named an out-of-scope *future feature*; this one silently transforms display output. Both are documentation-class clears.

---

#### Finding 3: `validate_description` does not constrain control characters; raw ESC bytes in a stored description reach the terminal verbatim through `tracker show`

- **Dimension:** Dim 1 (Spec coverage — interaction with the stderr-contract escape rationale), Dim 9 (Assignment compliance — security stance).
- **Severity:** Low (spec-conformant under-delivery, by SO read).
- **Evidence:**
  - `DESIGN.md:38` Feature 1 error states: "`--description` value is empty or whitespace-only after trim → stderr `Error: Description cannot be empty.` → exit 1." Description has no control-character preclusion in the spec.
  - `DESIGN.md:343-345` Edge Cases / Description: empty/trim rules + length unspecified + "Description is not trimmed; stored verbatim" + multi-line support. Control characters are not mentioned for description (in contrast to title at `DESIGN.md:293` and label at `DESIGN.md:309`, which explicitly reject Cc characters with terminal-escape rationale).
  - `src/lib.rs:335-340` `validate_description` enforces only empty-after-trim. No control-char or ESC check.
  - `src/lib.rs:125-139` `issue_fields_are_valid` does not check description for control characters either: `issue.description.as_ref().is_none_or(|d| !d.trim().is_empty())`.
  - `cmd_show` prints `format_show_block` output to stdout via `print!` — a stored description containing ESC `0x1B` will emit a literal escape sequence to the terminal.
  - Rationale gap: the stderr-contract escape rule (`DESIGN.md:215`) names "the error stream" specifically and excludes data output. Description is data, not error text — so the spec does not bind it. Title/label control-char prohibition is bound to the `list` one-issue-per-line contract — description is *not* shown in `list` (verified by `description_not_in_list_output`), so the `list`-line-break rationale also does not apply. The threat is therefore confined to `tracker show <id>`: a tracker.json containing a description with an ANSI escape sequence will render through the user's terminal as the user's terminal interprets it.
- **Rationale:** This is a **spec-conformant under-delivery**: the spec deliberately permits newlines in description, so a broader control-character prohibition would contradict line 345. The threat model (DESIGN.md `Cf`-class rationale at line 314: "single-user local tool: the threat surface is bounded to the user attacking themselves with hand-pasted clipboard content or a hand-edited `tracker.json`") arguably covers description too — but Cc characters are categorically more dangerous than Cf characters (terminal escape vs. visual confusion), and the spec's silence on description-Cc is a gap that other domain reviewers may flag (Security R9, Red Team R8). SO surfaces it as a spec-coverage observation, not a Layer 6 implementation defect.
- **Classification:** Open — Raised to SO/Security/Red Team for cross-domain adjudication. Two resolutions exist: (a) accept-and-document (add an Edge Cases / Description bullet stating description-Cc is out-of-threat-model parallel to the `Cf` stance at line 314), or (b) tighten the spec to reject ESC / Cc-minus-newline in description (analogous to title/label) and add a corresponding `validate_description` check.
- **Proposed action:** Defer to Security R9 / Red Team R8 for threat-model adjudication. If accepted-as-risk, amend `DESIGN.md` Edge Cases / Description to add a bullet parallel to line 314 explicitly placing Cc-minus-newline-in-description out-of-threat-model with the same single-user / multi-user re-evaluation trigger. If tightened, add Cc-minus-`\n` check to `validate_description` and `issue_fields_are_valid`, plus tests.
- **Spec-creep evaluation:** Neither creep nor under-delivery against the literal spec — but a coordination item for the Security / Red Team cold-batch in this round.

---

### Spec-creep audit (Dim 2 — additions not in DESIGN.md or TODO.md Layer 6)

Walked the Layer 6 diff (`727aef9..c91676a`) for additions outside Layer 6's named scope:

- **`validate_description`** (`src/lib.rs:335-340`): directly required by DESIGN.md Feature 1 error state line 38. Not creep.
- **`format_show_block`** (`src/lib.rs:350-387`): directly required by DESIGN.md "Show output format" lines 245-270. The `\r\n` normalization is Finding 2.
- **`cmd_show`** (`src/lib.rs:398-409`): directly required by DESIGN.md Feature 4. Not creep.
- **`cmd_delete`** (`src/lib.rs:422-433`): directly required by DESIGN.md Feature 5. Non-interactive shape matches D1 — see Dim 9 audit below. Not creep.
- **`Issue.description` field unchanged** — already present from Layer 1 with `skip_serializing_if`; Layer 6 only starts writing to it. Not creep.
- **`Commands::Show` / `Commands::Delete` enum variants and `description: Option<String>` on `Create`** (`src/main.rs:14-55`): directly required by DESIGN.md Interface table line 207-211. Not creep.
- **No new dependencies** in `Cargo.toml` or `Cargo.lock`. Confirmed unchanged by `git diff`.
- **No Layer 7 surface leak:** no `IsTerminal`, no `colored` / `anstream` / `ansi_term` crate, no color application to status/priority, no manual `--help` examples added (clap defaults only). The `Delete` doc-comment in `src/main.rs:51` ("no confirmation; deleted IDs are never reused") is helpful contextual text that surfaces only through clap's auto-generated `--help`, which is Layer 7's polish; the wording is a single-line doc-comment, not a Layer 7 `--help` examples block. Border-acceptable; not creep.

One creep-candidate detected: the `\r\n` normalization (Finding 2). No anticipatory code or comment in the Layer 6 diff names a Layer 7 / future feature.

---

### Assignment-compliance audit (Dim 9)

**D1 — `tracker delete <id>` non-interactive (per DESIGN.md "Approved Deviations from Assignment")**

- The implementation matches D1 exactly: `cmd_delete` (`src/lib.rs:422-433`) takes only the id, performs one-shot deletion, prints `Deleted issue #<id>.`, exits 0. No `[y/N]` prompt; no `--yes` flag (which would itself contradict D1's "no bypass flag" clause).
- The `Delete` clap variant doc-comment (`src/main.rs:51`) reads: "Delete an issue (no confirmation; deleted IDs are never reused)" — this surfaces the deviation to the user via `--help`, anchoring the non-interactive contract in the user-facing surface.
- **Re-evaluation trigger** (DESIGN.md line 420): "if the tool is used in a multi-user / shared context, reintroduce the confirmation requirement". Layer 6 does not change the threat model — still single-user / local. D1's rationale holds. No new argument for or against.
- **Layer 6-specific question:** Does the implementation introduce any auxiliary safety net that compensates for the absent confirmation? Examined: no soft-delete, no archive, no `--dry-run`, no undo. The recovery path is exactly as D1 names it — "the user can restore the deleted record by editing the file directly." Consistent with the deviation rationale.

D1 compliance: clean. No Dim 9 finding.

**Assignment Layer 6 text vs. DESIGN.md:** Layer 6 assignment specifies "show full details; delete with confirmation". The `show` half is implemented in full (Feature 4). The `delete with confirmation` half is waived via D1 with director approval. The deviation is documented, dated, and has a re-evaluation trigger — proper assignment-compliance recordkeeping.

---

### Cat B Red Gate disposition audit

The Phase 2a commit (`4fb5e67`) classifies:

- **2 unit tests as Cat A** (fail against `todo!()`): `multiline_description_show_format`, `show_label_column_right_padded_to_13`.
- **1 unit test as Cat B** (passes at Phase 2a): `max_id_plus_one_skips_deleted_ids`.
- **18 integration tests as Cat A** (fail at runtime against `todo!()` stubs and the description-discarding `cmd_create`).
- **2 integration tests as Cat B** (pass at Phase 2a): `create_without_description_has_no_field_in_json`, `description_not_in_list_output`.

Auditing for honesty:

1. **`max_id_plus_one_skips_deleted_ids` Cat B claim:** `next_id(&[1, 3])` returning `4` is the Layer 1 implementation behavior — `next_id` was written in Layer 1 to return `max(existing_ids) + 1` (`src/lib.rs:88-92`). At Phase 2a, the test compiles and passes against the unchanged `next_id`. **Honest.** Classification matches the SO R18 audit pattern (a Layer's AC is structurally emergent from a prior Layer's implementation; the test pins the contract for the new Layer's domain).
2. **`create_without_description_has_no_field_in_json` Cat B claim:** The `Issue.description: Option<String>` field with `#[serde(skip_serializing_if = "Option::is_none")]` was present from Layer 1 (`git show 727aef9:issue-tracker-cli/src/lib.rs | grep description` confirms). At Phase 2a, `cmd_create` discards `description_raw` but still writes `description: None`, which serializes to no key. The test passes at Phase 2a. **Honest.**
3. **`description_not_in_list_output` Cat B claim:** `cmd_list` has never rendered description (Layer 1's tabular columns are ID/Status/Priority/Labels/Title only). At Phase 2a, even with `description: None` discarding, the test still passes — no description text could reach stdout regardless. **Honest.**
4. **No Phase 2a violation:** A Phase 2a violation would be a Layer 6 AC that exists at Phase 2a state without a unit test failing against the Phase 2a state. Every Layer 6 AC for description-storage, show-rendering, and delete-removal has a failing Cat A test at Phase 2a (the four `todo!()` stubs panic; `cmd_create` discards description). The Cat B emergent-from-prior-layer tests are regression coverage, not the sole assertion.
5. **Consistency with prior layers:** Same shape as Layer 3 (`create_without_priority_defaults_to_medium`), Layer 4 (two Cat B), Layer 5 (multiple Cat B per SO R18 audit). Pattern is established and documented.

**Cat B disposition: honest.** No mis-classification.

---

### Test obligation audit (Dim 4)

- **Red Gate plan:** TODO.md lines 320-343 enumerates 18 integration + 3 unit tests for Layer 6.
- **Actual:** 20 integration + 3 unit. Over by 2 integration tests.
- **The two extras:**
  - `create_with_whitespace_description_exits_one` (`tests/layer6.rs:59-73`) — covers TODO.md AC line 286 (`tracker create "Fix bug" --description "  "` exits 1). The Red Gate plan named only the empty-string variant (`create_with_empty_description_exits_one`); the whitespace variant is an AC-faithful twin test, identical to the Layer 1 / Layer 4 pattern of testing empty + whitespace separately.
  - `delete_then_show_returns_not_found` (`tests/layer6.rs:334-349`) — covers TODO.md AC line 296 (after `delete 1`, `show 1` returns not-found). The Red Gate plan named `delete_removes_issue` and `delete_id_not_reused` but elided the delete-then-show composition AC. The extra test pins the cross-feature interaction.
- **Verdict:** Both extras are AC-faithful coverage of ACs the Red Gate plan elided, not over-test or gold-plate. Marginal expansion (~30 LOC across both) for two real AC-mapped assertions. Within tolerance; same shape as SO R18 audit of Layer 5's "7 instead of 4" pattern.

---

### Hallucinated

*(none this round — Findings 1-3 each survive the dismissal test below)*

### Backlogged

*(none this round)*

### Dismissed

*(none this round)*

### Open

- **F1** (manual checklist 13/13 unchecked) — process item for director closure before merge.
- **F2** (`\r\n` → `\n` normalization in `format_show_block` not in DESIGN.md) — Raised to SO for adjudication (DESIGN.md amendment OR code rollback).
- **F3** (description has no Cc-character constraint; ESC bytes reach terminal via `show`) — Raised to SO / Security R9 / Red Team R8 for threat-model adjudication.

### Carry-forward (from prior rounds)

- **SA R11 F1** (rendering half of `cmd_list` extraction) — Open per SO R19; deferred to focused pre-Layer-7 PR. Layer 6 does not touch the affected code; carry-forward unchanged.
- **TW R7 F5** (PROCESS.md Layer 1-4 reflection blocks) — Resolved post-R17 by commit `a226d88` ("PROCESS.md Layer 1-4 developer reflections"). Closes the carry-forward.

---

### Summary

3 Open findings, all Low severity. 0 Hallucinated. 0 Dismissed. 0 Backlogged. 0 Approved deviations.

The Layer 6 implementation matches all 18 implementation ACs with passing tests (159/159 cargo test pass; clean clippy/fmt per commit message). The Cat B Red Gate disposition is honest and consistent with the SO R18 / prior-layer pattern. The two test count extras (20 int vs. 18 planned) are AC-faithful coverage of two TODO.md ACs the Red Gate plan elided. No new dependencies. No Layer 7 surface leak. D1 (non-interactive delete) compliance is clean; assignment-compliance documentation is in place.

The three findings:
- F1 is a *process state* (manual checklist), not a Phase 2 defect; consistent with the developer's explicit deferral in the Phase 2b commit message.
- F2 is a *low-grade rendering-layer creep* (`\r\n` → `\n` not in DESIGN.md), resolvable by either ratification or removal.
- F3 is a *spec-coverage gap* (description-Cc threat-surface), a coordination item for Security R9 / Red Team R8 in this round's cold-batch.

None block Layer 6 closure from the SO lens. F2 is a single sentence in DESIGN.md or a 1-line code revert. F3 will likely be settled by Security R9's stance and a small Edge Cases / Description amendment in Round 2.

**Sycophancy check (self-applied):** I tested whether each finding survives a dismissal attempt.

- **F1 dismissal attempt:** "Manual checklists always close just before merge; flagging it is process pedantry." Counter: the SO domain's job is to track the spec-and-process contract. DESIGN.md line 373 names manual testing as a layer gate. The prior layers close the checklist *as part of* the layer's IAR round, not "always after". Flagging the state at the gate boundary is exactly the SO duty. Real finding (Low severity, process state).
- **F2 dismissal attempt:** "`\r\n` normalization is obviously sensible; the spec just didn't enumerate every line-ending edge case." Counter: the spec mentions `\n` specifically (`DESIGN.md:345`). The implementation comment self-justifies the normalization, which is the tell — the author knew the spec doesn't bind it. Two layers of evidence (spec text + author's self-justification) confirm the gap. Same defect class as Layer 5 SO R18 F1 (code does something the spec doesn't name). Real finding.
- **F3 dismissal attempt:** "Description-Cc is out-of-threat-model by parallel with the `Cf` rationale at DESIGN.md:314." Counter: the `Cf` rationale at line 314 is *labels*, not description, and Cc characters are categorically more dangerous than Cf (terminal-escape injection vs. visual confusion). The spec is silent on description-Cc; the silence is the gap. The parallel is plausible but not made explicit, which is the SO's job to flag. Real finding (coordination, not implementation).

Each finding survived. The clean compliance table is real; the three findings are the residue.

**Coordination:**
- **Security Engineer (Security R9):** F3 (description-Cc threat surface) is in Security's wheelhouse. Recommend Security's pass either accept-as-risk with explicit DESIGN.md bullet, or tighten `validate_description` with a Cc-minus-`\n` check.
- **Red Team (Red Team R8):** F3 is a candidate Red Team finding — terminal-escape injection through `show` for a hand-edited `tracker.json`. SO defers to RT's threat model.
- **Data Engineer (DE R9):** F2 (`\r\n` normalization) intersects with stored-form-vs-rendered-form fidelity. DE may have an opinion on whether storage-faithful rendering is a stronger principle than display-clean rendering.
- **Software Engineer (SE R15):** Owns the implementation choice on F2 if option (b) (remove normalization) is adjudicated.
- **Technical Writer (TW R9):** F2 may be resolvable by a DESIGN.md amendment (TW co-owns).
- **Director (SO authority):** F1 is a TODO.md tick after manual execution. F2 / F3 may need DESIGN.md amendments — SO authority. Recommended action: settle F2/F3 in Round 1 cross-domain adjudication; close F1 by manual checklist execution before merge.

The Layer 6 implementation is in spec compliance and merge-ready from the SO lens after F1 is closed by manual testing and F2/F3 are adjudicated in Round 2.

---

## Review 21 — 2026-05-11 02:00Z

**Round:** SO Review 21 (Round-2 SO adjudication for Layer 6 IAR closure)
**Scope:** Spec amendments + cross-domain finding adjudications for the Round-1 Open cluster.
**Session context:** Director-orchestrated warm-resolution session; not adversarial-cold per CLOSURE-PROTOCOL.md Section 5 step 3.

### Adjudications

- **R20 F1 (manual testing 13/13 unchecked):** **Open / Pending Director.** Same standard as Layer 4 R11 F2 / Layer 5 final closure — execute the 13 items + commit before merge per `b0a3789` / `da0fd8d` precedent.
- **R20 F2 (`\r\n` → `\n` normalization undeclared in DESIGN.md):** **Resolved as spec ratification.** DESIGN.md "Show output format" now declares the normalization with rationale.
- **R20 F3 / Security R9 F1 / RT R8 F1 / DE R9 F1 / SE R15 F1 / QE R15 F2 (description Cc defense — convergent across 7 domains):** **Resolved as Option-A (Layer 4 R2 lineage replay).** Defend at create + load + spec, mirroring the Layer 4 label hardening. DESIGN.md Feature 1 + Edge Cases / Description amended; `validate_description` + `description_is_valid` extended; 12 integration + 10 unit tests added. Same resolution closes the cluster across 7 domains.
- **RT R8 F2 (Trojan-Source / Cf in description):** **Accepted Risk** per spec ratification. Same posture as RT R6 F3 for title/labels (single-user local-CLI threat model; risk owner: director). DESIGN.md Edge Cases / Description explicitly enumerates the Cf accepted-risk stance.
- **SA R13 F1 (CreateArgs + module-split tripwires fired):** **Split decision** — Trigger A (CreateArgs refactor) **Resolved inline**; Trigger B (`src/lib.rs` storage/validate/commands split) **Deferred to pre-Layer-7 focused PR** bundled with SA R11 F1 + SA R13 F2. The CreateArgs refactor was specifically scheduled "at the layer that adds the next create flag" by SA R7 F4 / R8 F4 / R10 — Layer 6 is that moment, so it lands now. The module split is real architectural work that benefits from its own focused PR with test scaffolding.
- **QE R15 F1 (over-padding mutation):** **Resolved** by `show_renders_exact_full_block_for_single_line_issue` (full-line equality across all 8 rendered rows).
- **QE R15 F3 (verbatim-storage half untested):** **Resolved** by `create_preserves_description_verbatim_with_surrounding_whitespace` + unit equivalent.
- **SE R15 F2 / DE R9 F2 (bare `\r` overprint):** **Resolved** — subsumed by the broader Cc-except-`\n` rejection rule.
- **UX R8 F1 / TW R9 F2 (`show` / `delete` `--help` depth):** **Resolved** by expanded doc-comments in `src/main.rs`.
- **TW R9 F1 (CHANGELOG missing Layer 6):** **Resolved** by Layer 6 retrospective + Round-2 closure entry in CHANGELOG.md.
- **TW R9 F3 (portfolio README stale across L5+L6):** **Resolved** — Layer 5 → ✅ Complete, Layer 6 → 🟡 In IAR Round 2.
- **TW R9 F4 (manual checklist unchecked):** Same as R20 F1. **Open / Pending Director.**

### Open

- **R20 F1 / VDD-IAR R15 F1 / TW R9 F4** — Layer 6 manual testing checklist. Director action.

### Backlogged / Dismissed / Hallucinated

*(none this round)*

### Summary

10 Round-1 cross-domain Open findings closed inline in commit `9b775f0`. 1 Accepted Risk (RT R8 F2). 2 architectural concerns Deferred to pre-Layer-7 focused PR (SA R11 F1 + SA R13 F2 rendering; SA R13 F1 Trigger B module split). 1 Open process finding pending director action. DESIGN.md amendments are non-creep — they ratify the description-Cc defense that the title and label hardenings already established at Layers 1 and 4.

**Coordination:**
- **VDD-IAR R16:** Verify Round-2 closure cadence + Open F1 disposition. Merge-gate verdict pending F1 closure.
- **Director:** Execute 13 manual checklist items; commit per `b0a3789` / `da0fd8d` precedent.

---

## Review 22 — 2026-05-11 03:36Z

**Round:** SO Review 22 (Round-3 director-raised finding from Layer 6 manual testing)
**Scope:** A single director-raised finding from manual-testing execution against the Layer 6 binary (TODO.md:303-316 checklist). The finding is a spec-vs-implementation defect surfaced by manual test item line 311 ("ID not reused: delete issue #2, create new issue → new ID is #3 (or higher, never #2)"). Scope is bounded to this defect; the rest of the checklist is not in scope this round.
**Reference:** `DESIGN.md` Feature 1 invariants (line 47), Feature 5 invariants (line 152), Data Model field invariants (line 176); `TODO.md` Layer 6 manual test line 311; `tests/layer6.rs:351-375` `delete_id_not_reused`; `src/lib.rs:88-92` `next_id`; `src/lib.rs:456-478` `cmd_delete`; SA Review 3 Finding 3 (`SOLUTION-ARCHITECT-REVIEW.md:47-53`) — the architectural decision under whose lineage this defect sits.
**Session context:** Warm session — same orchestrator session that diagnosed the failure from the director's manual-test transcript. **Sycophancy guard explicit:** I diagnosed and proposed this finding to the director before being asked to raise it. SO's job in this round is to verify the finding survives a dismissal attempt without softening, not to ratify my own diagnosis. The dismissal-test pass is recorded at the end of this entry.

---

### Finding 1: `next_id` reuses the deleted ID when the deleted issue was the highest — violates DESIGN.md "never reused" invariant in three places

- **Dimension:** Dim 1 (Spec coverage — invariant not enforced), Dim 5 (Under-delivery — Layer 6 AC for non-reuse partially fails), Dim 7 (Design fidelity — implementation contradicts spec text).
- **Severity:** Medium. The defect is a real spec violation, not a doc-only gap; but the impact in this tool's single-user threat model is low (IDs are referenced only in the tracker's own output, per SA R3 F3 rationale).

**Reproduction (director's manual-test transcript, condensed):**

```
$ rm tracker.json
$ tracker create "First"   → Created issue #1: First
$ tracker create "Second"  → Created issue #2: Second
$ tracker delete 2         → Deleted issue #2.
$ tracker create "Third"   → Created issue #2: Third    ← BUG: id=2 reused
$ cat tracker.json | grep '"id"'
    "id": 1,
    "id": 2,
```

The reused id=2 was the id of the just-deleted issue. TODO.md:311 explicitly requires "(or higher, never #2)".

**Spec text the implementation violates (three citations):**

- `DESIGN.md:47` Feature 1 Invariants: "IDs are assigned in strictly ascending order and are never reused, **including after deletion**."
- `DESIGN.md:152` Feature 5 Invariants: "The deleted ID is never reused; the next created issue receives `max(remaining_ids) + 1`, **which will always be greater than the deleted ID**."
- `DESIGN.md:176` Data Model field invariants: "`id` is unique across all issues **and never reused**."

The Feature 5 sub-claim that "`max(remaining_ids) + 1` … will always be greater than the deleted ID" is provably false. Counter-example: state `[#1, #2]`; delete `#2`; remaining = `[#1]`; `max(remaining) + 1 = 2`; the new issue receives `id=2` — equal to the deleted id, not greater.

**Implementation evidence:**

- `src/lib.rs:88-92` `next_id` computes `existing_ids.iter().max().copied().unwrap_or(0).checked_add(1)`. The function returns `max+1` against *currently-stored* IDs only. It has no knowledge of historically-assigned-then-deleted IDs.
- `src/lib.rs:79-81` doc-comment: "IDs are never reused: the next ID is always strictly greater than all existing IDs, including those of deleted issues." This claim is wrong in the same direction as DESIGN.md:152 — `next_id` only sees *existing* (currently-stored) IDs; deleted IDs are not in the input.
- `src/lib.rs:461-462` `cmd_delete` doc-comment self-justifies: "Deleted IDs are never reused: the next `create` assigns `max(remaining_ids) + 1`, which is strictly greater than any deleted ID." Same false claim — falls in the high-edge case.

**Test-coverage hole (why automated tests did not catch this):**

- `tests/layer6.rs:351-375` `delete_id_not_reused` exercises *only* the middle-gap case: create `#1`, create `#2`, delete `#1` (the lowest), create — asserts new id is `3`. After delete, remaining is `[#2]`; `max+1 = 3` ≠ deleted `1`. The test passes because the deleted id (1) is strictly less than `max+1` of the remaining set.
- The high-edge case (delete the highest id, then create) is *never* exercised. Mutation analysis: a `next_id` implementation that returns `max(existing_ids) + 1` and a hypothetical implementation that returns "max id ever assigned + 1" produce identical results for the middle-gap reproduction — the test cannot distinguish them. The director's manual test is the first execution that fixes the deleted id at the high edge.
- `src/lib.rs:1260-1269` unit test `max_id_plus_one_skips_deleted_ids` has the same hole — it asserts `next_id(&[1, 3]) == 4`, again middle-gap. It does not test `next_id(&[1])` after `[1, 2]` produced `2`.

**Historical context (SA Review 3 Finding 3, `SOLUTION-ARCHITECT-REVIEW.md:47-53`):**

SA R3 F3 simplified away a persistent `next_id: u64` storage field, reasoning: "ID non-reuse after deletion is meaningful when IDs are referenced externally (foreign keys, logs, URLs). In this tool, IDs appear only in the tracker's own output." SA's resolution: "Removed `next_id` from the storage file shape. ID assignment now specified as `max(existing_ids) + 1`, or `1` if the issue list is empty. **Delete invariant updated accordingly**."

The "Delete invariant updated accordingly" step kept the "never reused" claim and added the `max(remaining_ids) + 1` formula — but did not notice that the formula does not preserve the invariant for high-edge deletion. The simplification was sound (the persistent counter was over-engineered for this threat model) but the spec text was not reconciled with the simpler implementation's actual behavior. SA R3 produced both DESIGN.md:152's false sub-claim and the current implementation; both ship in the merge candidate.

**Why this is a real finding (sycophancy guard — dismissal attempts):**

1. *"It's a documentation gap. The implementation does what most users will expect — IDs are unique among existing issues."* Counter: DESIGN.md states "never reused" in **three** independent places (lines 47, 152, 176). All three are invariant declarations, not casual prose. TODO.md:311 codifies the contract as a manual test. The Layer 6 acceptance criterion at TODO.md line 296 names "ID not reused" as one of the gate-closing checks. Three spec invariants + one acceptance criterion + one manual test is not a documentation gap — it is a contract.

2. *"SA R3 F3 was approved; the implementation matches SA R3 F3's resolution."* Counter: SA R3 F3 said two things — (a) remove the persistent storage counter, and (b) update the delete invariant accordingly. (a) is correctly implemented. (b) is *not*: the delete invariant text still says "never reused" AND adds a false sub-claim. The implementation matches (a) and contradicts (b). SA's approval covers (a); it does not authorize the contradiction in (b).

3. *"The single-user threat model SA R3 F3 named makes ID-reuse harmless."* Counter: the threat-model argument supports *removing the persistent counter*; it does not support *keeping the "never reused" promise while quietly reusing IDs*. If the spec said "IDs are unique among existing issues at any point in time," the current implementation would be compliant. The spec does not say that. The spec says "never reused, including after deletion." The director executing TODO.md:311 expects #3, not #2 — that expectation is what the spec text creates.

4. *"The defect surfaces only at the high edge — practically rare."* Counter: rarity is not a defense for invariant violation. The director's first manual-test run hit it on a 2-issue tracker — not adversarial input, not a corner case 18.4 quintillion creates deep. The high-edge reproduction is two creates + one delete + one create.

The finding survives all four dismissal attempts.

**Classification:** **Open — Raised to SO for adjudication.** Two resolution paths, each internally coherent:

- **Option A — Honor the contract (implementation change).** Re-introduce a persistent counter that is monotonically increasing across the lifetime of the tracker. Storage shape changes from `[issue, …]` to either `{"issues": [issue, …], "next_id": N}` or an out-of-band sidecar. `next_id` reads from storage instead of computing from existing ids. `load_issues` validates `next_id > max(stored_ids)`. Cost: storage schema break (no existing user data is at risk in this branch since `tracker.json` is gitignored, but the format change is real); reverses half of SA R3 F3's simplification (the half that is breaking the invariant); adds one persistent invariant that must be maintained on every write. Strengthens spec fidelity at the cost of SA R3 F3's simplicity argument.

- **Option B — Weaken the contract (spec amendment).** Amend `DESIGN.md:47`, `DESIGN.md:152`, `DESIGN.md:176`, the `cmd_delete` doc-comment at `src/lib.rs:461-462`, the `next_id` doc-comment at `src/lib.rs:79-81`, and TODO.md:311 to state the *actual* behavior: "IDs assigned to currently-existing issues are unique; an ID equal to the id of the most-recently-deleted issue may be reassigned when that issue was the highest-id at delete time." Update the `delete_id_not_reused` test to remove the high-edge assertion and add a regression test pinning the high-edge *reuse* behavior. Cost: contradicts three invariant declarations the spec made in good faith; contradicts a Layer 6 acceptance criterion the director was about to tick; the assignment brief itself does not require either contract, but Option B is a spec retreat. Cheap in code (~5 spec edits, 1 test update, 0 implementation change) but expensive in contract integrity.

**SO recommendation (subject to director override):** Option A. Rationale: the "never reused" invariant is stated in three places with explicit cross-references between Feature 5 and the Data Model field invariants — the spec was authored deliberately to lock this behavior. SA R3 F3's threat-model argument *supports the simplification* but does not *justify the spec retreat*; the simpler `max+1` implementation is a wrong answer to the question "how do you preserve non-reuse in a flat-file store," not a different answer to "do we need non-reuse at all." Option B is the spec capitulating to a buggy implementation, which is exactly the pattern the SO domain prompt warns against ("Quality does not justify scope … 'Better than asked for' is not a defense" — the inverse also holds: "less than promised, because the implementation found a shortcut" is not a defense). The storage-schema change in Option A is mechanical, scoped, and well-bounded; the test that already exists (`delete_id_not_reused`) extends to cover the high-edge case with one additional `tracker delete <highest> + tracker create` block.

**Proposed action (if Option A approved):**
- DESIGN.md: amend Data Model storage shape to include `next_id: u64`; specify load-time invariant `next_id > max(id)`; specify cmd_create / cmd_delete write semantics (`cmd_create` reads `next_id`, assigns it, writes `next_id + 1`; `cmd_delete` does not modify `next_id`).
- `src/lib.rs`: introduce a `Tracker` struct (or equivalent) wrapping `{ issues: Vec<Issue>, next_id: u64 }`; refactor `load_issues` / `save_issues` to round-trip the new shape; refactor `next_id(&[u64])` either to take the stored counter or replace it with a method on the wrapper; add `checked_add` overflow defense (Security R4 F2 lineage); update `cmd_create` and `cmd_delete` call sites.
- `tests/layer6.rs:351-375` `delete_id_not_reused`: extend to add a high-edge subcase — create #1, create #2, delete #2, create "Third"; assert new id is 3 (not 2).
- `src/lib.rs:1260-1269` `max_id_plus_one_skips_deleted_ids`: rename and rewrite to match the new contract (either delete it or convert to test the persisted counter's monotonicity).
- TODO.md:311: unchanged (the manual test was already correct; this is the test the director executed).
- `cmd_delete` and `next_id` doc-comments: rewrite to match the persistent-counter contract.
- CHANGELOG.md: Round-3 entry crediting the director's manual-testing finding; SA R3 F3 lineage acknowledged with the spec-vs-implementation reconciliation correction.

**Proposed action (if Option B approved):**
- DESIGN.md:47, :152, :176: rewrite the "never reused" claims to the weaker contract (uniqueness among existing issues; possible high-edge reuse).
- `src/lib.rs:79-81`, `src/lib.rs:461-462` doc-comments: rewrite to match.
- TODO.md:311: amend to remove the high-edge expectation OR delete the manual test outright.
- `tests/layer6.rs:351-375`: rename `delete_id_not_reused` → `delete_id_not_reused_in_middle_gap`; add a paired test `delete_high_edge_id_may_be_reused` that pins the (now-spec-permitted) high-edge reuse.
- CHANGELOG.md: Round-3 entry documenting the spec retreat with explicit acknowledgement of which invariants were weakened and why.
- DECISIONS.md: new entry under "Layer 6 spec amendments — SO Review 22" recording the deliberate retreat from the original contract.

**Coordination:**
- **Solution Architect (SA — next round):** SA R3 F3's resolution authored the false sub-claim at DESIGN.md:152. If Option A is adjudicated, SA should record that R3 F3's threat-model argument supports the simplification but does not eliminate the non-reuse contract — the two are separable. If Option B is adjudicated, SA should record the retreat with the threat-model rationale promoted from "doesn't need external protection" to "the invariant itself is dropped."
- **Software Engineer (SE — next round):** Owns the Option A implementation if approved. The change is small but touches the storage schema, which means `load_issues` / `save_issues` shape changes, plus a load-time invariant check that complements the existing `issue_fields_are_valid` and `issues_collection_invariants_hold` checks.
- **Quality Engineer (QE — next round):** Owns the test extension for both options. The mutation-analysis observation (middle-gap-only coverage cannot distinguish `max+1` from "max-ever+1") is a Cat B Red Gate audit pattern that QE can record as a lessons-learned alongside the test extension.
- **Data Engineer (DE — next round):** If Option A, the storage schema change is a DE concern. The shape `{issues: [...], next_id: N}` introduces a new load-time invariant (`next_id > max(issues.id)`) that intersects with the existing duplicate-id rejection in `issues_collection_invariants_hold`.
- **Technical Writer (TW — next round):** If Option B, TW co-owns the spec-retreat documentation in CHANGELOG.md / DECISIONS.md; the retreat must be loud, not quiet.
- **Director:** Adjudicates A vs. B. Closure expected in Round-4 (or as a same-round inline resolution if Option B and the spec edits are mechanical).

---

### Non-finding note: bonus verification (`--description "line1\rOVER"`)

The director's session also reported a "bonus verification failed" for `tracker create "X" --description "line1\rOVER"` succeeding. This is **not a code defect.** zsh's double-quote rules do not interpret `\r` as carriage return — `"line1\rOVER"` is the literal 11-char string `line1\rOVER` (backslash + 'r'), which is not a control character. The `validate_description` Cc defense (R20 F3 / R21 cluster) correctly rejects an actual `\r` byte; the unit test `description_with_control_char_other_than_newline_is_rejected` (`src/lib.rs:1183-1193`) pins this with `validate_description("a\rb").is_err()` (Rust string-literal escape, which *does* produce a CR). The correct shell incantation to verify the Cc defense end-to-end is `tracker create "X" --description $'line1\rOVER'` (ANSI-C quoting), matching the pattern TODO.md:307 already uses for the multi-line newline check.

The bonus item should be re-run with `$'...'` quoting to complete the manual checklist row. No code, spec, or test change.

---

### Open

- **F1** (`next_id` reuses deleted high-edge id; violates "never reused" invariant) — **Raised to SO for adjudication; A vs. B decision pending director.**

### Hallucinated / Backlogged / Dismissed

*(none this round)*

### Carry-forward (from prior rounds)

- **SO R20 F1 / VDD-IAR R15 F1 / TW R9 F4** (Layer 6 manual testing checklist closure) — **partial progress this round.** The director executed at least items aligned with TODO.md:311 (the trigger for this finding) and the bonus row (resolved as a shell-quoting non-finding above). Full 13/13 closure remains pending.
- **SA R11 F1 + SA R13 F1 Trigger B + SA R13 F2** (architectural deferrals to pre-Layer-7 focused PR) — unchanged.

### Summary

1 Open finding raised. 0 Hallucinated. 0 Dismissed. 0 Backlogged.

The defect is a spec-vs-implementation contradiction that was latent through Layers 1-6 because the test coverage for non-reuse was scoped narrowly enough to miss the high-edge case. The director's manual execution of TODO.md:311 is the first execution that fixed the deleted id at the high edge and surfaced the gap. The finding has architectural lineage (SA R3 F3 simplified the persistent counter), test-coverage lineage (`delete_id_not_reused` only exercises middle-gap), and spec lineage (DESIGN.md:152's sub-claim is provably false). Two coherent resolutions exist; SO recommends Option A (honor the contract) over Option B (weaken the spec).

**Layer 6 merge gate impact:** F1 blocks merge from the SO lens. The "never reused" promise is a Layer 6 acceptance criterion (TODO.md:296), not a Phase 2+ nice-to-have. Closure requires the director's A/B adjudication and the implementing change. If Option A: one SE round + one QE test extension. If Option B: one round of spec/test/doc edits with explicit retreat documentation.

**Coordination:**
- **Director (immediate):** Adjudicate Option A vs. Option B. Recommend Option A.
- **VDD-IAR (next round):** This Round-3 finding extends the Layer 6 IAR cycle beyond Round-2 closure. VDD-IAR should record that the closure was incomplete — director-side manual testing surfaced a real defect that the four-domain Round-1 + adjudication Round-2 missed. The mutation-analysis blind spot in `delete_id_not_reused` is a QE-domain process datum; the spec-vs-implementation contradiction surviving Round 2 is a VDD-IAR-domain process datum. Both worth recording.

---

## Review 23 — 2026-05-11 22:30Z

**Round:** SO Review 23 (Layer 7 IAR Round 1, cold session)
**Scope:** Layer 7 polish layer — branch `issue-tracker-cli-polish`, 3 commits since main (`7b461aa` Red Gate, `a2b8062` Phase 2b implementation, `603c689` manual checklist closure). DESIGN.md "Interface / color output" is the binding spec section; TODO.md L353-364 enumerates the layer AC; TODO.md L368-374 enumerates the manual checklist.
**Session context:** Cold session — did not build this layer. SO posture per `prompts/review-session.md` + `domains/role/SOLUTION-OWNER-REVIEW.md`. Carry-forward awareness: SA R13 F1 Trigger B (`src/lib.rs` module split) + SA R11 F1 + SA R13 F2 were Deferred at SO Review 21 to a "pre-Layer-7 focused PR." Layer 7 has shipped without that PR landing. RT R8 F2 (Cf in description) is in Accepted Risk standing per SO R21.

### Layer 7 AC compliance table (TODO.md L353-364)

| # | Acceptance criterion | Status | Evidence |
|---|---|---|---|
| 1 | `tracker --help` exits 0 and describes all subcommands | Met | `tests/layer7.rs:36-48` pins exit 0 + presence of all 6 subcommand names in stdout. |
| 2 | `tracker create --help` describes all flags with valid values | Met | `tests/layer7.rs:51-61` pins `--description` / `--priority` / `--label` + `low, medium, high` enumeration. |
| 3 | `tracker list --help` describes all flags with valid values | Met | `tests/layer7.rs:64-75` pins `--status` / `--priority` / `--label` + `open, in-progress, done` + `low, medium, high`. |
| 4 | `tracker status --help` describes positional args and valid status values | Met | `tests/layer7.rs:78-87` pins `<ID>` / `<STATUS>` + `open, in-progress, done`. |
| 5 | `tracker show --help` describes the `<id>` argument | Met | `tests/layer7.rs:90-97` pins `<ID>`; release-binary stdout shows the field list and arg description. |
| 6 | `tracker delete --help` describes the `<id>` argument | Met | `tests/layer7.rs:100-107` pins `<ID>`; release-binary stdout includes the D1-deviation cross-reference. |
| 7 | TTY: `high`/`medium`/`low` priority rendered red-bold / yellow / default | Met (manual) | `src/lib.rs:51-60` `priority_ansi` returns `\x1b[1;31m` / `\x1b[33m` / `None`; manual checklist `[x]` at TODO.md:370; release-binary `script` transcript in CHANGELOG L29-30. |
| 8 | TTY: `in-progress`/`done`/`open` rendered cyan / green / default | Met (manual) | `src/lib.rs:65-74` `status_ansi` returns `\x1b[36m` / `\x1b[32m` / `None`; manual checklist `[x]` at TODO.md:370; release-binary transcript in CHANGELOG L29-30. |
| 9 | Piped stdout contains no ANSI codes | Met | `tests/layer7.rs:133-193` `list_piped_has_no_ansi_codes` + `show_piped_has_no_ansi_codes`; `cmd_list` `src/lib.rs:835` and `cmd_show` `src/lib.rs:591` gate on `stdout().is_terminal()`. |
| 10 | Color applied only to value text, not row or header | Met | `format_show_block` `src/lib.rs:548-549` wraps only `status_display` / `priority_display`; the 13-char label column is literal text. `cmd_list` `src/lib.rs:837-840` emits the header row with no color and uncolored `ID` / `Labels` / `Title` cells. `wrap_color` `src/lib.rs:79-84` only ever wraps the bare value text. |
| 11 | Color appears in both `tracker list` and `tracker show` output when TTY | Met | Both `cmd_list` (L835) and `cmd_show` (L591) thread `use_color` through; manual checklist `[x]` at TODO.md:370 and TODO.md:372. |
| 12 | All error messages begin with `Error:` followed by a human-readable description | Met | All `Err(...)` returns in `src/lib.rs` carry the bare message; `src/main.rs:117` prefixes `Error: ` for app errors; `src/main.rs:75` rewrites clap's `error:` to `Error:` for parser errors. Verified across the 13 distinct error strings in `src/lib.rs` (Title × 2, Description × 2, Label × 3, ID × 1, Status × 1, Priority × 1, NotFound × 3 sites, Storage R/W × 2, Counter-overflow × 1). |
| 13 | Unknown subcommand exits 1 with a usage error on stderr | Met | `tests/layer7.rs:112-122` `unknown_subcommand_exits_one` pins exit 1 + `Error:` + `unrecognized subcommand` + `frobnicate` in stderr. Verified end-to-end against release binary: `Error: unrecognized subcommand 'frobnicate'` followed by usage hint, exit 1. |

**Regression check on prior layers' compliance:** I re-verified that Layer 7 did not narrow any earlier behavior. Color-suppression-when-piped means every Layer 1-6 integration test (which by `assert_cmd::Command` construction connects stdout to a pipe) exercises the no-color branch; the implementation note in CHANGELOG L20-21 is correct that none regressed. `cmd_list` row format string changed from `{:<11}` / `{:<8}` to a pre-padded cell substitution — verified the visible column widths still match DESIGN.md "List output format" (ID 4, Status 11, Priority 8, Labels 20, Title up to 50, 2-space separator). `format_show_block` gained a parameter; all internal call sites (cmd_show, two unit tests) updated correctly per CHANGELOG L16. The Layer 6 `\r\n` normalization (R20 F2 → R21 ratified) survived intact at `src/lib.rs:541`.

All 13 ACs Met. Layer 7 ships the polish that DESIGN.md "Interface / color output" specifies. No spec deviation in the implementation itself.

### Findings

#### Finding 1: SA carry-forward "pre-Layer-7 focused PR" never landed — `src/lib.rs` shipped Layer 7 at 1506 LOC

- **Dimension:** Dim 8 (Prior-review additions — a deferred architectural item became a real Open by deferral failure), Dim 4 (Over-engineering — by negation: the absence of a scheduled refactor at the agreed gate).
- **Severity:** Medium. Not a spec violation; SO does not own architectural decomposition. But the deferral itself was an SO adjudication (R21), so its non-execution lands back in SO's lap as an open carry-forward to acknowledge — not silently ignore.

**Facts:**
- SO R21 adjudicated SA R13 F1 Trigger B + SA R11 F1 + SA R13 F2 as **Deferred to pre-Layer-7 focused PR** (`SOLUTION-OWNER-REVIEW.md:1928`). The deferral text: "real architectural work that benefits from its own focused PR with test scaffolding."
- SA R13 (final entry, `SOLUTION-ARCHITECT-REVIEW.md:1185`): "SA may re-raise at Layer 7 opening if the PR has not landed."
- The branch `issue-tracker-cli-polish` contains 3 commits since `main`: the Red Gate, the implementation, and the manual checklist tick. No focused architectural PR landed before any of them. `src/lib.rs` is now 1506 LOC (1411+ at carry-forward time; the Layer 7 change added ~95 more LOC of color helpers + threading the `use_color` parameter through). The non-test portion has grown well past the SA R9 F3 / SA R11 / SA R13 trigger threshold of 500 LOC.
- The pre-Layer-7 PR scope was explicitly bookkept at SA R13 last entry: (a) `cmd_list` rendering extraction, (b) `format_show_block` constants, (c) `lib.rs` module split into `storage` / `validate` / `commands`. None of (a) / (b) / (c) were applied. The Layer 7 implementation in fact added new rendering surface (`pad_after_color`, `wrap_color`, `status_ansi`, `priority_ansi`) into the same monolithic `src/lib.rs` — moving in the opposite direction from the deferred decomposition.

**Why this is an SO finding, not "just SA":** The R21 deferral was an SO adjudication binding the timeline. SO owns the "we agreed at R21 that this would happen before Layer 7; it did not." The architectural content remains SA's; the broken-commitment process datum is SO's. Re-affirming the deferral without an explicit new gate (and noting Layer 7 went forward anyway) is itself a sycophancy failure — the kind R20 F2 / R22 F1 dismissal tests warn against ("don't re-defer when the prior deferral's trigger has fired without action").

**Classification:** **Raised to SA / SE.** SA owns the re-raise decision (re-defer with a new explicit trigger, or escalate to "block merge"). SE owns the implementation if the disposition is "act now." SO records the broken-deferral fact and adjudicates the new disposition in the next SO round after SA + SE weigh in.

**Proposed dispositions for SA / SE:**
- *(a) Re-defer to "pre-Layer-7-IAR-close" — i.e., before this Layer 7 IAR round completes.* Mechanically scoped (no new spec, no new tests in scope); ships the architectural prep before merge. Most consistent with the R21 commitment's spirit.
- *(b) Re-defer to "before Layer 7 merges to main."* Same code-change scope; slightly weaker timeline. Allows the Layer 7 IAR to close on the polish content and bundles the refactor with the merge gate.
- *(c) Drop the deferral altogether, revising the SA R9 F3 / R11 / R13 trigger conditions explicitly with a new rationale.* Honest but expensive: requires SA to author a new dismissal text that does not retroactively legitimize letting the trigger fire silently. This is the "move the goalposts after the fact" path SA R13 already named (`SOLUTION-ARCHITECT-REVIEW.md:1000`).

**SO recommendation:** (a) or (b). The architectural prep was scheduled; it should land. Layer 7 itself is clean and merge-ready content-wise.

#### Finding 2: Red Gate pass-against-current framing is honestly documented but materially weakens the layer's TDD discipline

- **Dimension:** Dim 7 (Design fidelity — between TODO.md's "Red Gate — tests to write first" contract and what the tests actually pin), with a sycophancy lens.
- **Severity:** Low–Medium. The framing is *honestly* documented in `tests/layer7.rs:1-26` and the `7b461aa` commit message; this is not a hidden defect. But the SO sycophancy-check requirement (prompt-stated) compels me to evaluate whether the framing should be accepted at face value or named as a gap.

**Facts:**
- `tests/layer7.rs:5-10` (top comment): "clap's default `--help` plumbing and the Layer 1 `try_parse` transform in `src/main.rs` already satisfy most of the `--help` / unknown-subcommand / `Error:` acceptance criteria *against current code* — the tests below pass against `main` before any Phase 2b work."
- TODO.md L377-388 names the Red Gate test set with the conventional "fails against stub" annotation on each test (e.g. `help_flag_binary_exits_zero` — "fails against stub that exits 1"). The TODO.md commitment is the standard TDD-Red-Gate contract: write tests that fail, then implement to pass.
- 7 of 9 Layer 7 tests (the 6 `--help` tests + the unknown-subcommand test) passed at `main` before `a2b8062`. Only 2 tests (`list_piped_has_no_ansi_codes` + `show_piped_has_no_ansi_codes`) had any potential regression-guard value, and even those pass trivially against pre-color code (CHANGELOG L20 acknowledges this).
- The `7b461aa` commit message and the test file top-comment explicitly *acknowledge* the framing departure: "valid Layer 7 Red Gate tests because they pin the help/error *contract* (valid-value enumerations, exit codes, stderr routing) that prior layers established only by convention; a future refactor that drops the `--priority <low|medium|high>` enumeration … would now fail a named test rather than silently regress." This is a coherent argument — pinning conventions as regression guards has real value — but it is not the Red-Gate-as-driver-of-implementation framing TODO.md describes.

**Dismissal attempt:** *"The tests are honestly documented; if the developer says they pin conventions, that's a legitimate test purpose."* Counter: the question is not whether the tests have value (they do, as regression guards) but whether they discharge the Red Gate criterion as TODO.md states it. TODO.md L377: "Red Gate — tests to write first". L380 et seq.: each test annotated "fails against stub that exits 1" / "fails against stub that exits 0" — i.e., the tests are described as failing against the prior state. They did not. The honest acknowledgement in the commit message does not change the contract-vs-actual gap; it documents it.

**Counter-dismissal attempt:** *"Layer 7 is a polish layer; most of the AC was already structurally satisfied by Layer 1's clap plumbing. The Red Gate convention doesn't fit polish layers cleanly."* This is a reasonable architectural observation but it should have surfaced as a TODO.md amendment ("Layer 7 Red Gate is a regression-guard pin rather than a fails-first set") authored before the implementation, not a self-justifying test-file comment after. The pattern matches SO R20 F2: the implementation does something the spec doesn't sanction, and the implementation self-justifies. Same defect class.

**Classification:** **Raised to QE / Director.** The framing is honestly disclosed but should be either (a) ratified in TODO.md with an explicit "polish-layer Red Gate" clause that future polish layers can reuse, or (b) backfilled by adding a Phase-2a Red Gate test that actually fails against pre-Layer-7 main (e.g., a unit test asserting `priority_ansi("high", true)` returns `Some("\x1b[1;31m")` — which doesn't compile against main because the function doesn't exist yet). Option (b) is mechanically small if Layer 7 IAR is still mid-flight; (a) is appropriate if Layer 7 closes as-is.

#### Finding 3: Layer 7 manual checklist closed 16 minutes after implementation — verification depth claim warrants the standing skeptical posture

- **Dimension:** Dim 1 (Spec coverage — manual checklist is named in DESIGN.md L385-393 as a layer gate), Dim 5 (Under-delivery, by skepticism), Dim 9 by lens (process compliance to TODO.md L368-374 in detail).
- **Severity:** Low. SO does not own manual testing execution depth; that is director-and-VDD-IAR territory. But TODO.md L374 reads "Review each error message from all prior layers manually: does it say what went wrong and what the valid alternatives are?" — this is the most substantive item in the checklist and its tick warrants the standing sycophancy posture against fast checklist closures.

**Facts:**
- Layer 7 implementation commit `a2b8062` is timestamped `2026-05-11 15:04:36 -0700`.
- Layer 7 manual checklist closure commit `603c689` is timestamped `2026-05-11 15:20:31 -0700` — **16 minutes after implementation**.
- The 7 items include: building the release binary, exercising 6 `--help` invocations and visually verifying flag content, running `tracker list` in a TTY and visually confirming 3 color values, running `tracker list | cat` and verifying no escape sequences, running `tracker show` in TTY and piped, the substantive "review each error message from all prior layers" task, and the unknown-subcommand exit-1 check. The `603c689` commit message documents all 7 with concrete details — release binary built from `a2b8062`, scratch `/tmp` directory, `cat -v` pipe rendering, the `script -q /dev/null` TTY harness implied by CHANGELOG L27.
- The commit message specifically calls out: "Error-message review across all prior layers — every error path begins with `Error:`, names the offending input, and (where the domain is closed) enumerates the valid alternatives." This is a specific claim across 13 distinct error strings spanning 6 layers.

**Verification I can perform from the cold-session lens:**
- AC #12 ("all error messages begin with `Error:`") — I verified this against all 13 error strings in `src/lib.rs` (see compliance table). The `main.rs:117` and `main.rs:75` prefixing covers them all.
- AC #13 ("unknown subcommand exits 1") — verified end-to-end via release binary.
- The TTY-positive items I cannot verify (no TTY in the agent session). The piped no-color items are pinned by `tests/layer7.rs:133-193`.
- The "review each error message" item is the only one where the depth of the manual claim materially exceeds what the automated tests cover. The 16-minute window allows for it (the checklist items are small individually; the error-message review is the longest, and even at 30s per error message across 13 strings that's well under 16 minutes including the other items).

**Dismissal attempt:** *"The commit message documents the verification with adequate specifics; the timing window is sufficient; the standing process accepts a quickly-ticked checklist when the underlying tests are clean."* This held under scrutiny. The TODO.md L374 item is the one that could be ticked superficially, but the commit message records a concrete observation (Layer 4 duplicate-label dedup acceptance: "Duplicate-label on create succeeds silently with one stored 'bug' — spec-correct dedup behavior per DESIGN.md Feature 1 / Layer 4, not an error case") — that's a substantive judgement-call note that a superficial tick would not produce.

**Classification:** **Dismissed.** The fast-closure pattern triggered the standing sycophancy posture, but the commit message produces enough specifics to demonstrate the review happened. Recorded as a non-finding so the dismissal is on the record (rather than silent); VDD-IAR may revisit if process auditing surfaces a different pattern across layers.

#### Finding 4: Color helpers ship in `src/lib.rs` as private but spec-bearing — `wrap_color` / `pad_after_color` / `priority_ansi` / `status_ansi` are not part of the public library API surface

- **Dimension:** Dim 7 (Design fidelity, weak read — DESIGN.md does not specify whether color rendering is internal or library-API; either is consistent with the spec). Dim 4 (Over-engineering, by negation — minimal helpers; no abstraction over the 6 escape sequences; no anstyle/termcolor dep).
- **Severity:** Hallucinated.

**Why I considered it:** I checked whether the new helpers should be `pub` (consistent with `cmd_*` and validation helpers exposed for testing) or private (consistent with `format_show_block`, `priority_rank`, `truncate_with_ellipsis` precedents). The lib's `//!` module doc enumerates the public API as data model + cmd_* + parsing/validation + storage primitives — rendering helpers are private precedent. The choice matches the precedent.

**Why this is hallucinated:** SO does not own this — it's SA territory ("library API surface as architecture") and the choice is internally consistent with three prior precedents in the same file (`format_show_block` private since Layer 6, `priority_rank` private since Layer 3, `truncate_with_ellipsis` private since Layer 4). No spec deviation. No scope creep. No real finding.

**Classification:** **Hallucinated.** Recorded so the consideration is visible; not a real finding.

### Carry-forward (from prior rounds, status as of Layer 7 close)

- **SO R20 F1 / VDD-IAR R15 F1 / TW R9 F4** (Layer 6 manual testing 13-item checklist) — outside this layer's scope; Layer 6 R22 resolved by manual-test reproduction at item 311, the rest pending. Not adjudicated this round.
- **SA R11 F1 + SA R13 F1 Trigger B + SA R13 F2** — see Finding 1 above. Re-raised as a real Open finding because the deferral's gate ("pre-Layer-7 focused PR") has been crossed without action.
- **RT R8 F2** (Cf in description, Accepted Risk) — unchanged. Stable.

### Open / Raised / Dismissed / Hallucinated summary

- **Open: 0.** No Open SO findings as of this round close. F1 is Raised; F2 is Raised; F3 is Dismissed; F4 is Hallucinated.
- **Raised to SA / SE: 1** (F1 — broken pre-Layer-7 deferral).
- **Raised to QE / Director: 1** (F2 — Red Gate pass-against-current framing).
- **Dismissed: 1** (F3 — manual checklist fast closure).
- **Hallucinated: 1** (F4 — color helper visibility).

### Summary

Layer 7 polish content is in spec compliance. All 13 acceptance criteria Met; regression check on Layer 1-6 compliance clean. The implementation matches DESIGN.md "Interface / color output" exactly (6 ANSI sequences for the 6 colored value × default-value combinations; TTY-detection via stable `std::io::IsTerminal`; color suppressed when piped; color applied to value text only, not headers / labels / rows). No scope creep: no new dependencies added (raw ANSI escapes, no `anstyle` / `termcolor`); no spec-extending behavior introduced; no additional flags or subcommands.

The two substantive concerns are process-class, not implementation-class: (F1) the architectural prep PR that R21 deferred to "pre-Layer-7" was skipped — Layer 7 shipped with `src/lib.rs` at 1506 LOC and the 3 R21-bookkept refactor items unaddressed; (F2) the Red Gate tests were honestly framed as pass-against-current pins-of-conventions rather than fails-first drivers, which is a polish-layer-shaped departure from TODO.md's Red Gate contract.

**Verdict: GO-PENDING-{SA/SE Finding 1 disposition; QE/Director Finding 2 disposition}.** Layer 7 content is merge-ready from the SO lens. The two raised process findings require non-SO domain adjudication before the merge gate closes; neither requires changes to Layer 7's implementation itself. If SA disposes F1 as re-defer (a/b) and QE/Director disposes F2 as either ratify-TODO.md or backfill-one-failing-test, the layer closes cleanly.

**Coordination:**
- **Solution Architect (SA — Layer 7 round):** F1 is your re-raise opportunity. The R21 deferral has had its gate cross without action. Recommend re-deferring with a tighter gate (pre-merge-to-main) or escalating to "Layer 7 merge-gate dependency." Pick the disposition; SO will adjudicate inline in the next SO round.
- **Software Engineer (SE — Layer 7 round):** If SA's F1 disposition is "act now," the scope is the same 3-item bundle SA R13 documented: `cmd_list` rendering extraction (SA R11 F1), `format_show_block` constants (SA R13 F2), `lib.rs` module split into `storage` / `validate` / `commands` (SA R13 F1 Trigger B).
- **Quality Engineer (QE — Layer 7 round):** F2 is in QE's wheelhouse. Either (a) ratify in TODO.md with a "polish-layer Red Gate" clause that names pinning-of-conventions as a valid Red Gate purpose for polish layers, or (b) backfill a Phase-2a test that fails against pre-Layer-7 main (a unit test on `priority_ansi` / `status_ansi` would not compile against main).
- **Director:** Adjudicate F2 with QE. Manual checklist for Layer 7 already closed at `603c689`; the standing skeptical posture against fast closure was dismissed under scrutiny (F3) but recorded for future cross-layer auditing.
- **VDD-IAR Alignment (next round):** Two process-class findings this round (F1 broken-deferral, F2 Red-Gate-framing). Both are within the IAR process surface VDD-IAR audits. Worth recording the pattern: a deferral with a named gate that closes silently when the gate is crossed is the same defect class as SO R22's "spec invariant violated quietly by simplification" — both are "the system did the easier thing without acknowledging the trade-off."

---

## Review 24 — 2026-05-12 00:00Z

**Round:** SO Review 24 (Layer 7 IAR Round 2 closure pass). Warm closure-verification per CLOSURE-PROTOCOL.md §5; not a new adversarial round.

**Scope:** Verify Round-1 finding closures landed in commits `fbbb8a3` (VDD-IAR R17 F1 Option A retrofit) and `09b1905` (substantive cross-domain Round-2 closure). Inputs: DESIGN.md amendments at "Interface / Color output" + "stderr contract" + Edge Cases storage; six new DECISIONS.md entries under "Layer 7 IAR Round 2 spec amendments"; CHANGELOG.md Layer 7 Round-2 entry.

### Round-1 finding closures

- **F1 — Pre-Layer-7 focused PR deferral expired (SA carry-forward cluster):** **Backlogged per CLOSURE-PROTOCOL.md §3.** DECISIONS.md entry "SA R11 F1 + SA R13 F1 Trigger B + SA R13 F2 auto-Backlog per CLOSURE-PROTOCOL.md §3" promotes the three findings from Deferred to Backlogged. The architectural concerns remain real but the cost-benefit calculus for a focused refactor PR has not shifted; Backlogging captures the work without binding it to a specific upcoming layer.
- **F2 — Red Gate pass-against-current framing:** **Resolved by `fbbb8a3` (VDD-IAR R17 F1 Option A).** 12 retroactive unit tests added with the `// retroactive Red Gate:` label per `prompts/implementation.md` L56; DECISIONS.md entry documents the rationale and "Do not repeat for non-polish layers" annotation. Option B (CLOSURE-PROTOCOL.md polish-layer-exception amendment) deliberately not taken — rule changes earned by recurrence.
- **F3 — Manual checklist 16-minute closure window:** **Dismissed under scrutiny.** `603c689` commit body enumerates per-checklist-item observed behaviors with specificity exceeding checkbox restatement; verification depth claim stands.
- **F4 — Private color helpers as spec-bearing API:** **Resolved by `09b1905`.** `ColorMode` enum + `color_mode_from_env()` now `pub`; `display_safe` also exposed `pub` (RT R10 F1 lineage). The spec-bearing surface that previously hid behind `fn` is now part of the documented public surface.

### Layer 7 AC compliance (post-R2)

All 13 original Layer 7 ACs (TODO.md L353-364) Met. Four new R2-amended commitments Met:
- NO_COLOR / CLICOLOR=0 honored (DESIGN.md L239 amendment; `color_mode_from_env`; integration test `no_color_env_does_not_break_piped_invocation`).
- Bold redundancy on every highlighted value (DESIGN.md L243-248 amendment; `priority_ansi("medium", On)` → `\x1b[1;33m`; `status_ansi("in-progress", On)` → `\x1b[1;36m`; `status_ansi("done", On)` → `\x1b[1;32m`).
- stderr Cc-escape extended to clap pipeline (DESIGN.md L222 amendment; `sanitize_quoted_values`; integration test `unknown_subcommand_with_cc_payload_escapes_in_stderr`).
- Errno tag in OS-error stderr ratified (DESIGN.md L343 amendment).

### New findings

*(none — closure pass.)*

### Summary

All four R1 SO-domain findings transitioned cleanly: F1 Backlogged §3, F2 Resolved (fbbb8a3), F3 Dismissed, F4 Resolved (09b1905). Six R2 spec amendments landed in DECISIONS.md with citation chain to originating R1 findings. The "private helpers as spec-bearing API" observation prompted the right architectural move (public `ColorMode` + `color_mode_from_env`) — a small Round-2 sweetener beyond the Open-finding contract.

**Verdict:** **GO-PENDING-MANUAL-REWALK.** The Round-2 manual testing checklist re-walk for the new behaviors (NO_COLOR / CLICOLOR / CLICOLOR_FORCE / bold rendering / no-ANSI-on-stderr-empty-state) is the standing CLOSURE-PROTOCOL §6 criterion-3 requirement; director must add to TODO.md and execute before merge.

**Coordination:** VDD-IAR R18 — ratify R17 F1 closure (evidence chain: `fbbb8a3` retrofit + R2 test updates + DECISIONS.md entry). SA R16 — F1 Backlog state ratified.

**Files modified:** This log appended; DESIGN.md and DECISIONS.md edits (SO authority) landed in `09b1905`.

---

## Review 25 — 2026-05-12 12:00Z

**Round:** SO Review 25 (Layer 7 IAR Round 3 review, cold session).
**Scope:** 5 commits since `b853a81` — `ff0e85c` (PE R12 F3 clippy pre-commit hook), `c341a54` (QE R17 F5 render_cell ASCII debug_assert), `bd7511e` (QE R17 F1 `TRACKER_INTERNAL_FORCE_COLOR` test seam + 8 integration tests), `3fa1f3c` (SA R11 F1 + SA R13 F2 cmd_list extraction + column constants), `8db9437` (SA R13 F1 Trigger B `src/lib.rs` three-module split). All 5 are domain-other closures of items the SO log has touched directly (the SA cluster was R23 F1, the QE seam was raised in QE R17, etc.). Cold session: I did not build any of these commits.

**Session context:** Cold per `prompts/review-session.md`. Standing skeptical posture against the "closes all 5 deferred items — clean test suite" framing per the prompt's sycophancy guard. The Layer 7 IAR Round 2 SO close (R24, 2026-05-12 00:00Z) ended in **GO-PENDING-MANUAL-REWALK**; this round inherits that pending gate.

### Layer 7 AC compliance table (post-R3)

The 13 original Layer 7 ACs (TODO.md L353-364) plus the 4 R2-amended commitments (R24 § "Layer 7 AC compliance (post-R2)") are the binding set for this round. I re-walked each against the post-R3 source tree to verify R3's refactor + test-seam introduced no behavioral drift.

| # | Acceptance criterion | Status | Evidence (post-R3) |
|---|---|---|---|
| 1 | `tracker --help` exits 0 + lists all subcommands | Met | `tests/layer7.rs:36-48`; unchanged by R3. |
| 2 | `tracker create --help` describes flags + valid values | Met | `tests/layer7.rs:51-61`; unchanged. |
| 3 | `tracker list --help` describes flags + valid values | Met | `tests/layer7.rs:64-75`; unchanged. |
| 4 | `tracker status --help` describes positional args + valid statuses | Met | `tests/layer7.rs:78-87`; unchanged. |
| 5 | `tracker show --help` describes `<id>` arg | Met | `tests/layer7.rs:90-97`; unchanged. |
| 6 | `tracker delete --help` describes `<id>` arg | Met | `tests/layer7.rs:100-107`; unchanged. |
| 7 | TTY: `high`/`medium`/`low` priority colored red-bold / yellow-bold / default | Met (now automated) | `commands.rs:144-153` `priority_ansi`; integration tests `force_color_emits_bold_red_for_high_priority` / `_bold_yellow_for_medium_priority` / `_does_not_color_low_priority` (`tests/layer7.rs:280-333`). |
| 8 | TTY: `in-progress`/`done`/`open` colored cyan-bold / green-bold / default | Met (now automated) | `commands.rs:159-168` `status_ansi`; `force_color_emits_bold_cyan_for_in_progress_status` / `_bold_green_for_done_status` (`tests/layer7.rs:336-375`). |
| 9 | Piped stdout has no ANSI codes | Met | `tests/layer7.rs:163-200` + `:218-236`; `color_mode_from_env` at `commands.rs:127-129`. |
| 10 | Color applied only to value text, not row or header | Met | `force_color_does_not_color_header_row` (`tests/layer7.rs:378-414`) + `force_color_show_renders_colored_status_and_priority_value_cells` (`:417-471`). |
| 11 | Color present in both `list` and `show` when TTY | Met | `commands.rs` threads `ColorMode` through both `cmd_list` (L608) and `cmd_show` (L404); manual checklist ticked at TODO.md L370. |
| 12 | All error messages begin with `Error:` | Met | `main.rs:84` clap prefix transform; `main.rs:134` app-error prefix; 13 distinct error strings re-audited across `validate.rs` / `storage.rs` / `commands.rs` — all bare messages, all prefixed at the boundary. |
| 13 | Unknown subcommand exits 1 with usage error on stderr | Met | `tests/layer7.rs:112-125`; `main.rs:83-88`. |
| R2-a | `NO_COLOR` honored | Met | `commands.rs:130-134`; `tests/layer7.rs:239-264`. |
| R2-b | `CLICOLOR=0` honored | Met | `commands.rs:135-137`; `tests/layer7.rs:239-264`. |
| R2-c | `CLICOLOR_FORCE` NOT honored (pipe-clean) | Met | `commands.rs` lacks any CLICOLOR_FORCE branch; doc-comment L95-97 documents the deliberate non-honoring; integration test in `no_color_env_does_not_break_piped_invocation` includes the `CLICOLOR_FORCE=1` row. |
| R2-d | Bold redundancy on every highlighted value | Met | `priority_ansi` / `status_ansi` all return `\x1b[1;...m` for highlighted values; force-color integration tests pin the exact sequences. |
| R2-e | stderr Cc-escape extended to clap pipeline | Met | `main.rs:84-87`; `tests/layer7.rs:128-152`. |
| R2-f | Errno tag in OS-error stderr ratified | Met | `storage.rs:199` / `:217`; matches DESIGN.md L343. |

All 13 original + all 4 R2-amended commitments remain Met. **R3 introduced one new commitment that DESIGN.md does NOT currently document — see Finding F2.**

### Regression check (no-spec-drift verification per R3 prompt pressure point #2)

Re-read the diff at module-boundary level. Substantive behavior:
- **`8db9437` lib.rs split:** mechanical — `pub use` re-exports preserve every public name (`tracker::cmd_create`, `tracker::Tracker`, `tracker::ColorMode`, `tracker::CreateArgs`, etc.). Integration tests and `main.rs` import paths unchanged. No error string, output format, or behavioral commitment touched.
- **`3fa1f3c` cmd_list extraction + column constants:** `format_list_header` / `format_list_row` / `filter_issues` / `render_cell` / `show_label` carved out as pure functions. Header literal in unit test (`format_list_header_uses_width_constants`) pins exact string `"ID    Status       Priority  Labels                Title"` — matches DESIGN.md L233 example exactly. Column widths preserved (ID=4, Status=11, Priority=8, Labels=20, Title-truncate=50). Show label-column width LABEL_COLUMN_WIDTH=13 matches DESIGN.md "Show output format" exactly.
- **`bd7511e` test seam:** adds **NEW observable behavior** — when `TRACKER_INTERNAL_FORCE_COLOR=1`, ANSI emits to non-TTY stdout. The `color_mode_from_env` doc-comment (commands.rs:99-118) frames this as "test seam only, not part of the public CLI contract." This is the pressure point #4 substantive finding — flagged separately as **F2** below.
- **`c341a54` render_cell debug_assert:** debug-only panic on non-ASCII passed to `render_cell`. Compiled out in release; production behavior unchanged. No spec-bearing surface.
- **`ff0e85c` clippy pre-commit hook:** repository config; no source/binary behavior change.

**Verdict on no-drift:** Four of five commits are clean refactors / non-runtime config. **The fifth (`bd7511e`) demonstrably adds a new env-var-gated color-emission path that DESIGN.md does not document.**

### Findings

#### Finding 1 — Auto-Backlog → Resolved transition (process pattern, pressure point #1)

**Dimension:** Dim 8 (Prior-review additions, process side); the auto-Backlog `§3` mechanism's intent vs. its rapid reversal.

**Severity:** Low–Medium. Not a spec violation; a process-pattern question SO must adjudicate explicitly per CLOSURE-PROTOCOL §3.

**Facts:**
- SA cluster (SA R11 F1 + SA R13 F1 Trigger B + SA R13 F2) was Deferred for 4 layers (L4 → L5 → L6 close → L7 R1).
- At Layer 7 R2 (2026-05-12 00:00Z, ~36h before this round), per R24, the cluster auto-Backlogged per CLOSURE-PROTOCOL §3. DECISIONS.md entry (lines 154-156) reads: "Backlogging captures it without commitment to a specific layer."
- At Layer 7 R3 (this round's commit window, ~12h after R2), the three findings shipped as Resolved via commits `3fa1f3c` + `8db9437`. The DECISIONS.md entry stating "without commitment to a specific layer" is now factually superseded by an actual closing layer (Layer 7 R3).

**Why this is a real (Low–Medium) process finding, not a hallucinated concern:**
1. **Speed alone is not the defect.** A Backlogged item that gets picked up the next round is consistent with CLOSURE-PROTOCOL §3 ("may be picked up at any future layer's discretion"). The protocol does not impose a minimum dwell time.
2. **The defect is that the DECISIONS.md justification text — the durable record — is now stale.** The R2 entry's rationale ("cost-benefit calculus for a focused refactor PR has not shifted enough to schedule it") rationalized inaction by appealing to scheduling difficulty; R3 then scheduled and shipped the same work 12h later. Either the cost-benefit calculus changed materially in 12h (in which case the change-of-calculus is the new fact to record), or the R2 calculus was incorrect and the §3 invocation was used as cover. Either way, the durable rationale on record is no longer supported by R3's evidence.
3. **CLOSURE-PROTOCOL §3 second-paragraph clause is the relief valve.** "The auto-Backlog is reversible: if the receiving authority later adjudicates, the finding moves out of Backlogged into the appropriate terminal state. The point of the rule is to surface 'this question has not been answered' as an explicit Backlog entry rather than as silent log noise." This *exactly* describes what R3 did — the Backlog → Resolved transition is the protocol working as designed. **The process pattern is not the defect.**
4. **What IS the defect is the missing DECISIONS.md amendment annotating the R2 entry as superseded.** The SA R3 F3 / SO R7 reversal annotations (DECISIONS.md L16-17 and L21-22) set the precedent: "Reversed by SO Review 22 …". The same pattern should annotate the R2 auto-Backlog entry now that R3 has closed it.

**Classification:** **Resolved** (the process pattern is §3 functioning as designed) WITH a **bookkeeping correction (Resolved this round by SO authority):** I will append a "Reversed/Superseded by Layer 7 IAR Round 3 closure (`3fa1f3c` + `8db9437`)" annotation to the R2 auto-Backlog DECISIONS.md entry. SO has authority on DECISIONS.md per CLOSURE-PROTOCOL §1. The amendment lands in this same commit.

**Pressure-point #1 specifically pressed:** "Was Layer 7 R3 the right venue, or should this have been a focused post-Layer-7 PR?" SO answer: Layer 7 R3 was a reasonable venue *given* that the substantive refactor + 8 new tests landed, the test count grew, no spec drift was introduced (per Finding 2 caveat), and CLOSURE-PROTOCOL §3 explicitly permits this trajectory. A focused post-Layer-7 PR would have been *cleaner* (smaller blast radius per commit, clearer rollback story) but is not *more correct*. I decline to second-guess the venue choice.

#### Finding 2 — `TRACKER_INTERNAL_FORCE_COLOR` env var is observable CLI behavior; DESIGN.md must document it (pressure points #4 and #7)

**Dimension:** Dim 1 (Spec coverage — observable behavior must be in DESIGN.md); Dim 2 (Scope creep, weak read — the implementation added an env-var-gated color path the spec does not name).

**Severity:** **Medium.** A spec gap on a real, observable, env-var-gated production code path.

**Facts:**
- `commands.rs:124-126` checks `std::env::var_os("TRACKER_INTERNAL_FORCE_COLOR")` and short-circuits to `ColorMode::On` when the value equals `"1"`. The check is placed **before** the TTY detection, so a non-TTY stdout receives ANSI escapes when this var is set.
- The variable is read in **every** invocation of every subcommand that calls `color_mode_from_env()` (`cmd_list` and `cmd_show` via `main.rs:100`).
- The doc-comment at `commands.rs:107-118` asserts: "not a user-facing feature; it does not equal `CLICOLOR_FORCE` (which the spec deliberately declines to honor); it exists solely to make the positive color contract automatable." Naming uses `TRACKER_` + `INTERNAL_` prefix as a "do not use" signal.
- **DESIGN.md silence audit (verified by `grep -r TRACKER_INTERNAL_FORCE_COLOR` against all `.md` files):** zero hits. README.md, DESIGN.md, CHANGELOG.md, DECISIONS.md, and every IAR review log are silent. The variable's behavioral footprint exists only in `commands.rs` and `tests/layer7.rs`.
- The variable's observable effect *contradicts* DESIGN.md L243-244's binding statement: "**CLICOLOR_FORCE=1 is not honored: color is never emitted to a non-TTY stdout regardless of env vars, to preserve the pipe-cleanness contract.**" The seam violates "regardless of env vars." This is the literal-spec-text contradiction.

**Why pressure-point #7's "test-only by undocumentation" framing fails the sycophancy check:**

The doc-comment defense rests on three claims:
1. *"It is not a user-facing feature."* — Counter: the spec defines user-facing as observable CLI behavior, not as named-in-`--help`. ANY env var that mutates ANY observable output IS user-facing in the spec-contract sense. The user-or-not distinction is a UX framing, not a spec framing.
2. *"It does not equal CLICOLOR_FORCE."* — Correct on the surface (the name is different; the activation value is `1` literal not any-non-empty) but irrelevant. The objection is not "this is a re-implementation of CLICOLOR_FORCE." The objection is "DESIGN.md L243-244 promises **regardless of env vars**, and an env var demonstrably overrides that promise." The mechanism's specific name doesn't matter; the contract violation does.
3. *"It exists solely to make the positive color contract automatable."* — A legitimate engineering justification (and one I find persuasive on the merits) — but a justification for an observable spec-relevant behavior must land in the spec, not in a source-code doc-comment. The portfolio submission's audience reads DESIGN.md as the authoritative contract; a reader auditing whether the implementation honors "regardless of env vars" by reading the spec alone would conclude yes — and be wrong.

**The integration test `force_color_with_no_color_env_set_does_not_force` (tests/layer7.rs:474-502) is itself evidence the contract is now non-trivial:** the test documents (in its comments at L488-491) that `TRACKER_INTERNAL_FORCE_COLOR=1` **wins over** `NO_COLOR=1`. This is a precedence ordering between two env vars — the kind of detail DESIGN.md normally specifies (cf. the explicit NO_COLOR / CLICOLOR / CLICOLOR_FORCE precedence list at L239-244). The current spec has no entry that would predict the seam's behavior.

**Classification:** **Open — Raised to SO for adjudication; Resolved this round by SO authority via DESIGN.md amendment.** SO has the authority to amend DESIGN.md (CLOSURE-PROTOCOL §1). The amendment will:

- Add a new bullet under "Color output" naming `TRACKER_INTERNAL_FORCE_COLOR=1` as a **test-only seam** that bypasses TTY detection and overrides `NO_COLOR` / `CLICOLOR`.
- Explicitly mark the seam as **unstable across versions** (no compatibility guarantee).
- Cross-reference QE R17 F1 as the originating finding.

This preserves the engineering decision (the seam stays; QE R17 F1's coverage gap remains closed) while making the observable behavior contract-visible. The amendment lands in this same commit.

#### Finding 3 — TODO.md missing the GO-PENDING-MANUAL-REWALK item from R24 (pressure point #6)

**Dimension:** Dim 1 (Spec coverage — manual checklist is a DESIGN.md "Testing Methodology" gate). Dim 5 (Under-delivery, by carry-forward).

**Severity:** Low. A bookkeeping miss in TODO.md, not a code or spec gap.

**Facts:**
- R24 closed with **GO-PENDING-MANUAL-REWALK**: "Round-2 manual testing checklist re-walk for the new behaviors (NO_COLOR / CLICOLOR / CLICOLOR_FORCE / bold rendering / no-ANSI-on-stderr-empty-state) is the standing CLOSURE-PROTOCOL §6 criterion-3 requirement; director must add to TODO.md and execute before merge."
- TODO.md L368-374 contains the original 7-item Layer 7 manual checklist, all `[x]` from `603c689`. No new items have been added for the R2 behaviors. `grep -nE "(rewalk|Round.2|NO_COLOR|CLICOLOR)" TODO.md` returns no matches.
- R3 commits did not touch TODO.md.

**Classification:** **Open — Raised to Director / SO.** SO owns TODO.md scope (CLOSURE-PROTOCOL §1: "Solution Owner (scope); director (sequencing)"). I will append the Round-2 manual rewalk items to the Layer 7 manual testing checklist in TODO.md in this same commit (5 items: NO_COLOR=1 suppresses color in TTY; CLICOLOR=0 suppresses; CLICOLOR_FORCE=1 does NOT enable color when piped; medium / in-progress / done all render with bold attribute; `tracker list` empty-state on stderr has no ANSI). Director executes; closure unblocks the R24 standing GO-PENDING gate.

#### Finding 4 — `TRACKER_INTERNAL_FORCE_COLOR` precedence over `NO_COLOR` is a deliberate test-ergonomics choice but a non-trivial spec implication (sub-finding of F2)

**Dimension:** Dim 7 (Design fidelity, narrow read of L239-244's precedence semantics).

**Severity:** Hallucinated, subsumed by F2.

**Why I considered it:** The seam-before-NO_COLOR ordering (test `force_color_with_no_color_env_set_does_not_force` at L474-502 explicitly pins that force-color wins over NO_COLOR) raises a "user safety" question: a user who sets NO_COLOR for vision-accessibility reasons should not have it overridden by an env var they don't know exists. In practice, the user must set `TRACKER_INTERNAL_FORCE_COLOR=1` *themselves* for the override to occur — and a user who knows the variable's name well enough to set it has opted into the override. The accessibility concern is theoretical only.

**Classification:** **Hallucinated.** Subsumed by F2's DESIGN.md amendment, which names the seam as test-only and unstable; an end user setting it for production purposes is out of contract.

### Open / Resolved / Dismissed / Hallucinated / Backlogged / Raised-to summary

- **Open: 0** at round close. F2 and F3 opened and resolved inline this round by SO authority (DESIGN.md amendment for F2; TODO.md amendment for F3). F1 closed as Resolved (process pattern is §3-compliant; the bookkeeping correction is applied inline by SO authority on DECISIONS.md).
- **Resolved: 3** (F1 process closure with DECISIONS.md annotation; F2 with DESIGN.md amendment; F3 with TODO.md amendment).
- **Dismissed: 0**.
- **Hallucinated: 1** (F4 — NO_COLOR override precedence, subsumed by F2).
- **Backlogged: 0**.
- **Approved deviation: 0**.
- **Raised to: 0** (F2's "Raised to SO for adjudication" was self-adjudicated by SO this round per CLOSURE-PROTOCOL §1 authority).

### Carry-forward / Cross-domain coordination

- **GO-PENDING-MANUAL-REWALK from R24** — still standing until director executes the now-amended TODO.md L368-374 items. The TODO.md amendment in this commit lands the items; the *execution* is a director action that may follow this review.
- **SA R16 / SA Round 3 (if a cold SA round runs after this)** — the cluster is Resolved; SA may ratify this round's closure or, if the cold review surfaces post-refactor architectural concerns, raise new findings.
- **VDD-IAR R19** — should ratify the process pattern (Backlog → Resolved 12h apart is §3-permitted but with the DECISIONS.md supersedure annotation now in place).

### Summary

5 commits since `b853a81`. Two refactor commits (`3fa1f3c` + `8db9437`) close 3 long-deferred architectural items by closing the SA cluster that R24 had auto-Backlogged 12h earlier — a fast §3 Backlog → Resolved transition that the protocol permits and the DECISIONS.md supersedure annotation now documents. One commit (`bd7511e`) introduces a `TRACKER_INTERNAL_FORCE_COLOR` env-var test seam that **demonstrably violates DESIGN.md L243-244's "regardless of env vars" promise** — addressed this round by an SO-authority amendment to DESIGN.md naming the seam explicitly as test-only-and-unstable. One commit (`c341a54`) adds a debug-only assertion (no spec surface). One commit (`ff0e85c`) installs a clippy pre-commit hook (PE-domain artifact, no code or spec surface).

The "237/237 pass" framing did soften my first read; the sycophancy guard surfaced F2 (the env-var DESIGN.md silence) and the F1 DECISIONS.md staleness on second pass. Neither is a code bug — both are documentation contract gaps the implementation accumulated as it raced to close deferred items. Both are remediated this round by SO-authority edits.

**Verdict:** **GO-PENDING-MANUAL-REWALK.** Inherits R24's standing pending gate. The R3 substantive content is in spec compliance after this round's amendments. Once director executes the now-itemized Round-2 manual rewalk (TODO.md L368-374 extension landing in this commit), the layer closes cleanly.

**Coordination:** VDD-IAR R19 — ratify the §3 Backlog→Resolved transition with DECISIONS.md supersedure annotation; record the spec-amendment-driven closure of F2 as a process datum (a test seam slipped in without DESIGN.md update — a defect class worth naming alongside the SO R20 / SO R22 "implementation does something the spec does not sanction" pattern). SA Round 3 (if run) — re-walk `commands.rs` / `validate.rs` / `storage.rs` boundaries against the post-split structure; the public re-export surface in `lib.rs:42-50` preserves the pre-split API but introduces a new question (do the now-split modules have internal coupling SA wants to assess?). Director — execute the TODO.md L368-374 manual rewalk extension to close the R24 standing gate.

**Files modified this round (SO authority per CLOSURE-PROTOCOL §1):**
- `iterative-adversarial-refinement/SOLUTION-OWNER-REVIEW.md` (this log appended)
- `DESIGN.md` (F2 amendment — `TRACKER_INTERNAL_FORCE_COLOR` documented as test-only seam)
- `DECISIONS.md` (F1 bookkeeping — R2 auto-Backlog entry annotated "Superseded by Layer 7 IAR Round 3 closure"; F2 amendment recorded as new entry under "Layer 7 IAR Round 3 spec amendments")
- `TODO.md` (F3 amendment — Round-2 manual rewalk items appended to the Layer 7 manual testing checklist)
