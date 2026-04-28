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

- Cold-session SO review recommended before Layer 1 opens (all three reviews conducted in-session with spec authorship)
- SA Review 1 findings resolved — see [SOLUTION-ARCHITECT-REVIEW.md](SOLUTION-ARCHITECT-REVIEW.md)
- VDD-IAR Alignment review pending — see [VDD-IAR-ALIGNMENT-REVIEW.md](VDD-IAR-ALIGNMENT-REVIEW.md)
