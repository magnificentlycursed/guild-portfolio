# Design Decisions

Key decisions made during the spec phase, with rationale. Source: IAR review logs.

---

## Data and Storage

**Non-atomic writes** — `tracker.json` is written directly on every mutation; no temp-file-and-rename.
- SA Review 1, Finding 1
- Why: single-user tool, one command at a time, no concurrent writers. The failure scenario (Ctrl-C mid-write) is rare and recoverable by deleting the file. Atomic write implementation cost is disproportionate for a personal learning tool. Revisit if concurrent access is ever required.

**ID assignment via `max(existing_ids) + 1`** — no `next_id` counter stored in `tracker.json`.
- SA Review 1, Finding 3
- Why: a stored `next_id` counter introduces a sync invariant that must be maintained across all writes and can fall out of sync with manual file edits. Computing the next ID from the existing maximum is simpler, has no failure mode, and is fast enough for any realistic issue count.
- **Reversed by SO Review 22 (Layer 6 spec amendments — SO Review 22).** Director manual-testing surfaced that `max(existing_ids) + 1` reuses the deleted id when the deleted issue was the highest, violating the "never reused, including after deletion" invariant. Persistent `next_id` counter restored.

**Top-level JSON array storage format** — `tracker.json` is a top-level `[Issue]` array, not a wrapped object `{"issues": [...]}`.
- SO Review 7 (approved), QE Review 2 / Data Engineer Review 2 (raised)
- Why: the original wrapper object contained two top-level keys (`"issues"` and `"next_id"`). SA Review 1 removed `"next_id"` as unnecessary complexity. After that removal, the wrapper contained a single key with no peers and added no information. A top-level array is simpler to deserialize (`serde_json::from_str::<Vec<Issue>>`) and is more idiomatic for a homogeneous collection.
- **Reversed by SO Review 22.** When the persistent `next_id` counter was restored, the wrapper object regained a meaningful peer — the rationale for collapsing to a bare array no longer applied.

**`description` field absent (not null) when not provided** — serialize `None` as a missing key, not `"description": null`.
- Data Engineer Review 1, Finding 2
- Why: absent and null carry different semantics in JSON. Omitting the key makes forward compatibility cleaner — a future schema that assigns meaning to `null` is unambiguous.

**Post-deserialization validation required** — semantically invalid field values in structurally-valid JSON (e.g., unknown status, non-positive ID, empty title) are treated as corrupt data.
- Security Review 1, Finding 1
- Why: `serde` deserialization validates structure, not domain. File data is untrusted. An implementation that passes deserialization then operates on invalid field values silently corrupts behavior. Invalid domain values trigger the same error path as malformed JSON.

---

## Interface and CLI

**Exit codes 0/1 only** — no exit code 2 for I/O errors; all failures exit 1.
- SA Review 1, Finding 2
- Why: distinct exit codes are only meaningful to a caller that checks `$?` and branches on the specific value. The tool is interactive. No scripted caller is identified. Two-tier exit codes add integration test obligation with no identified beneficiary.

**Non-interactive delete** — `tracker delete <id>` removes the issue immediately with no confirmation prompt.
- SO Review 6, Finding 1
- Why: the assignment's authoritative interface section lists `tracker delete <id>` with no confirmation signal. The build-layer guidance ("with confirmation") is explicitly framed as advisory. The tool is non-interactive by design — no command prompts for input. CLI convention for single-argument destructive commands in personal tools is immediate execution.

**Fixed column widths in list output** — not dynamic per-row maxima.
- SA Review 1, Finding 4
- Why: dynamic-width tables require two passes over the data (collect rows, compute maxima, render). Fixed widths produce equivalent readability with a single pass and predictable, testable output format.

**Library-agnostic CLI and JSON crates** — DESIGN.md names no specific Rust crates.
- SO Review 3, Finding 2
- Why: the observable interface contract (subcommand names, flag names, error messages, stdout/stderr/exit-code behavior) is independent of which library implements it. Naming crates in the spec locks the implementation unnecessarily and shifts the spec from behavioral to prescriptive.

**Color output included** — priority and status values are colored in TTY output; suppressed when piped.
- SO Review 3, Finding 1
- Why: the assignment's Layer 7 explicitly lists "colored output" alongside `--help` and empty-state messages. Previously excluded as a polish concern; inclusion is consistent with the assignment's explicit scope.

---

## Validation Scope

**No character limits on titles or labels** — non-empty validation only.
- SO Review 3, Finding 3
- Why: the assignment requires rejecting empty titles. It does not specify length limits. Limits add test obligations not required by the assignment and were removed to stay at 100% assignment scope.

**Empty description rejected** — `--description ""` (empty or whitespace-only after trim) → error.
- SO Review 5, Finding 6
- Why: consistent with empty-title and empty-label validation. The assignment's security guidance says "validate all input from the command line." Applying this to descriptions is a straightforward extension of the named principle. The alternative (silently ignoring empty description) was less consistent with the spec's overall input-validation posture.

**Description stored verbatim** — no trimming of leading/trailing whitespace.
- DESIGN.md (implicit in Feature 1 specification)
- Why: a user providing a description with intentional formatting or leading whitespace has expressed intent. Unlike titles (where leading whitespace is almost always accidental), descriptions are free-form. Title trimming is title-specific.

---

## Code Quality Enforcement

**`#![deny(clippy::unwrap_used)]` at crate level** — enforced in `lib.rs`; any `unwrap()` in future layers requires an inline `#[allow]` with a comment explaining why it is safe.
- SE Review 6 (general adversarial pass)
- Why: the single `unwrap()` in `save_issues` was verified safe in review logs but had no CI enforcement. A future developer adding a second `unwrap()` on a user-facing path would face no automated check. Enforcing at crate level forces explicit inline justification for every `unwrap()`, making safety analysis visible at the call site rather than only in the review history.

---

## Out of Scope (deliberate exclusions)

**No Windows line-ending normalization** — `\r\n` is not normalized to `\n` on storage.
- SA Review 1, Finding 5
- Why: target platform is macOS. macOS terminal input does not produce `\r\n`. No failure mode was identified. If Windows support is added, re-evaluate.

**No atomic writes** — deferred, not ruled out permanently.
- SA Review 1, Finding 1
- Why: implementation cost exceeds failure risk for a single-user local tool. If the tool is ever used with multiple concurrent writers or in a high-availability context, this decision should be revisited.

---

## Layer 3 spec amendments — SO Review 13

**Reject control characters in titles** — `validate_title` rejects any character with `is_control()` (Unicode category `Cc`). Same rejection applies at load time via `issue_fields_are_valid` (stored data with a control-character title is corrupt).
- UX Review 5 Findings 2 and 3 / Red Team Review 5 Findings 1 and 3 / SO Review 13 Finding 1
- Why: control characters in titles break the spec's one-issue-per-line `list` contract (newline / CR), corrupt column alignment (tab), and enable terminal-escape injection in any tool that displays the title (ESC, C1 controls). The assignment's "validate all input from the command line" principle covers this case; the rule closes both UX exploits and a Red Team attack surface in a single defect-fix-class spec amendment.

**Empty-state messages route to stderr, not stdout** — `No open issues. Nice work!` and `No issues match the given filters.` print to stderr. stdout is empty when no records match.
- UX Review 5 Finding 4 / SO Review 13 Finding 2
- Why: empty-state messages are informational, not data. The original spec routed them to stdout, which polluted piped consumers (`tracker list | wc -l` returned 1 in the empty case). The Unix convention separates data (stdout) from status / informational text (stderr) precisely so pipelines compose correctly. The change is a refinement of an originally underspecified detail; no caller depends on the prior stream choice.

**Unknown JSON fields are NOT preserved across writes** — documentation amendment to `Edge Cases / Storage`.
- DE Review 6 Finding 3 / SO Review 13 Finding 3
- Why: the spec already says unknown fields are ignored on read (forward-compatible deserialization). The non-obvious side effect — that any subsequent mutation rewrites `tracker.json` with only the documented schema, dropping unknown fields — was implicit. Users hand-editing `tracker.json` should know.

**SE Review 9 DESIGN.md content (lines 218 / 220-225) ratified** — the "exactly 2 spaces" column-separator rule and the example block stand as written.
- VDD-IAR Review 10 Finding 1 / SO Review 13 Finding 4
- Why: the content of the SE-9 edits is correct and useful — it specifies the format precisely and the example matches actual implementation output. Process integrity (the SE-9 edits were applied without prior SO approval) remains a separate VDD-IAR finding; SO ratification of the content does not retroactively legitimize the process. The split is intentional: SO owns spec content, VDD-IAR owns process compliance.

---

## Layer 6 spec amendments — SO Review 22

**Persistent `next_id` counter restored to `tracker.json`** — storage shape changes from a bare `[Issue]` array to `{"issues": [Issue], "next_id": u64}`. The counter is initialized to `1`, bumped via `checked_add(1)` on every successful create, and left unchanged by delete. Load-time invariants: `next_id >= 1`; if `issues` is non-empty, `next_id > max(issue.id)`.
- SO Review 22 Finding 1 (director-raised from Layer 6 manual testing); reverses SA Review 1 Finding 3 and SO Review 7's "bare top-level array" decision.
- Why: the pre-R22 `max(existing_ids) + 1` implementation did not preserve the DESIGN.md "deleted ID never reused, including after deletion" invariant (stated in three places: Feature 1 Invariants, Feature 5 Invariants, Data Model field invariants). In the high-edge case — delete the highest-id issue, then create — `max(remaining_ids) + 1` equals the just-deleted id, reusing it. Director manual-test reproduction (delete #2 from {#1,#2}, create → reassigned id=2) demonstrated the gap. SA Review 3 Finding 3's threat-model argument that ID non-reuse is unnecessary for an internal-only id space is preserved as accurate; the resolution simplified storage by removing a counter that was needed for a still-binding contract. Option A restores the counter; the spec contract is honored, the false sub-claim at DESIGN.md:152 (`max(remaining_ids) + 1` "will always be greater than the deleted ID") is removed, and the integration test `delete_id_not_reused_high_edge` regresses the prior failure.
- Trade-off accepted: a load-time invariant (`next_id > max(issue.id)`) replaces SA Review 3 Finding 3's simplicity argument. The cost is a single additional check in `tracker_is_valid` plus one field in the storage shape — proportionate to the contract it preserves. SA Review 3 Finding 3's "the counter can fall out of sync with manual edits" concern is mitigated by the load-time invariant: any out-of-sync state (`next_id <= max(id)`) is rejected as corrupt at load with the standard error path.
