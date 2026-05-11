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

---

## Layer 7 Red Gate methodological deviation — VDD-IAR Review 17 Finding 1 (Option A resolution)

**Layer 7's Phase 2a Red Gate landed with 0 failing primary signals** — all 9 integration tests in `tests/layer7.rs` passed against pre-implementation code. Resolution per VDD-IAR R17 F1 Option A: 12 retroactive unit tests on `priority_ansi` / `status_ansi` / `wrap_color` / `pad_after_color` were added in a Phase 2b-companion commit, labelled as a Red Gate deviation per `iterative-adversarial-refinement/prompts/implementation.md` L56.
- VDD-IAR Review 17 Finding 1 (CRITICAL, Dim 4 — Red Gate compliance).
- Why this is a methodological compromise, not a clean Red Gate: the implementation existed before the unit tests would have failed. Per implementation.md L56 ("retroactive Red Gate"), tests added post-implementation cannot satisfy the Red Gate — they mitigate the dim-4 severity by exposing the testable primitive surface that should have been the Phase 2a focus, and they document the gap explicitly in the source file as `// retroactive Red Gate: ... — discovered during Phase 3 IAR Round 1, test added post-implementation, confirmed passes against current implementation.` lines. The clean-room equivalent for a future polish-style layer would be to write the pure-function helper unit tests (asserting literal ANSI sequences) in Phase 2a *against unimplemented `todo!()` helpers*, producing the failing primary signal that Layer 7's `tests/layer7.rs` integration-only Red Gate did not.
- **Do not repeat for non-polish layers.** This deviation is acceptable here because the new behavior is end-to-end TTY-detected presentation (untestable in subprocess + non-TTY assert_cmd harness) and the architecture kept color logic in private helpers (hiding the testable primitive surface from the integration-test layer). Future layers introducing new public functions or CLI surface must satisfy the literal Red Gate — failing tests on `todo!()` stubs before any Phase 2b work.
- Trade-off accepted: the retrofit gives the dim-4 finding evidence to close in a Round-2 VDD-IAR Alignment review (the unit tests cover 6 of the previously-mutation-survivable color-mapping permutations per QE Review 17 Finding 2), but it does not retroactively replay the Phase 2a → Phase 2b boundary in the commit history. The honest disclosure in `7b461aa`'s commit message and `tests/layer7.rs`'s top comment, plus this DECISIONS.md entry, are the durable record. Option B (CLOSURE-PROTOCOL.md polish-layer-exception amendment) was deliberately not taken — a permanent rule change should be earned by recurrence, not pre-empted by a single instance.

---

## Layer 7 IAR Round 2 spec amendments

**`NO_COLOR` and `CLICOLOR=0` honored; `CLICOLOR_FORCE` not honored** — when stdout is a TTY, color is also suppressed if `NO_COLOR` is set to any non-empty value OR if `CLICOLOR=0` is set. `CLICOLOR_FORCE=1` is deliberately ignored — the pipe-cleanness contract (no ANSI to a non-TTY stdout) takes precedence over forcing color into a redirected stream.
- UX Review 10 Finding 1 / Security Review 11 Finding 2.
- Why: `NO_COLOR` is the de-facto cross-tool standard (https://no-color.org/) honored by `git`, `cargo`, `ripgrep`, `bat`, `fd`, `eza`, `delta`. A user who cannot disambiguate the spec's red/green palette (deuteranopia ≈5% of men — the canonical CVD case for these two colors) has no in-band escape hatch otherwise. Implementation is a single env-var check in the color-mode helper; cost is one extra branch per `cmd_list` / `cmd_show` invocation. `CLICOLOR_FORCE=1` was considered and declined: a Layer-7 commitment that piped stdout NEVER carries ANSI escapes is a downstream-parser contract worth keeping absolute. A user who wants colored output into a pager should pipe through `less -R` (which interprets escapes regardless of who emits them), not request the producer to lie about TTY-state.

**Color bold-redundancy: every highlighted value carries `bold`** — `medium` priority and `in-progress` / `done` status all gain the `bold` SGR attribute (`\x1b[1;...m`), matching the bold attribute that `high` priority has carried since the initial Layer 7 spec. The default-color values (`low`, `open`) intentionally remain plain so the highlighted/unhighlighted dichotomy reads at a glance for both CVD and non-CVD users.
- UX Review 10 Finding 2.
- Why: the original spec table at DESIGN.md L243 declared "Red / bold" for `high` and pure-color for the other four highlighted values — an unintentional asymmetry. The red/green pair (`high` priority + `done` status) is the canonical deuteranopia/protanopia miss-case; without a bold cue on `done`, a CVD user reading the status column sees no non-color signal to distinguish `done` from `open`. WCAG 1.4.1 *Use of Color* requires a non-color cue alongside any color cue; the bold attribute satisfies this for every highlighted value. Implementation cost is two extra `1;` byte-pair edits in `status_ansi` and one in `priority_ansi`.

**Raw ANSI escapes rather than `anstyle` / `termcolor` dependency** — the six SGR sequences (`\x1b[1;31m`, `\x1b[1;33m`, `\x1b[1;36m`, `\x1b[1;32m`, `\x1b[0m`, plus the TTY-detection check) are emitted as hardcoded string constants in `src/lib.rs` rather than threading a typed `anstyle::Style` or `termcolor::ColorSpec` through the rendering layer.
- TW Review 11 Finding 4 (raised); SA Review 15 Finding 5 (Dismissed, verification: "VT100 universality" claim is factually safe within scope).
- Why: the six sequences are universally supported by VT100-compatible terminals — the only environment a single-user local portfolio CLI targets. The `anstyle` typed alternative would add a dependency (and transitively `anstream` if cross-platform Windows-legacy support were wanted) without functional gain for the spec-scoped target environment. The decision is scoped to: no Windows-legacy cmd.exe support, no 256-color or true-color palette, no per-attribute composition beyond bold-plus-foreground. Revisit if (a) the target environment widens to legacy Windows terminals or (b) the palette grows beyond the five colors specified.

**stderr Cc-escape rule extended to clap's argument-parsing pipeline** — every stderr write site that interpolates user-supplied values MUST apply the Cc-escape transform, including the `unrecognized subcommand '<name>'` and similar errors generated by clap before any application code runs.
- RT Review 10 Finding 1 (the 4th instance of the surface-class drift pattern: Title L1 / Labels L4 / Description L6 / clap pipeline L7).
- Why: the prior three instances of this defect class were caught by extending the Cc-rejection rule at each new free-form text field's validate boundary. RT R9 / Security R10 proposed promoting this from "per-field validate rule" to "per-property rule applied at every interpolation site." Layer 7's `unknown_subcommand` test pinned clap's behavior but did not test Cc-bearing payloads; the contract gap surfaced when RT R10 actively planted CR / LF / NEL bytes in `tracker $'pre\rpost'` and observed clap's verbatim reflection to stderr. The rule extension closes the byte-level reflection at the highest stderr write site (clap's pipeline) and lets a single `display_safe(...)` transform in `main.rs` cover the gap without per-error wrapping.

**`Error: Could not read/save tracker data: <os-error-description>.` errno tag is in-spec** — the previous DESIGN.md "permission denied" wording was a placeholder; the implementation emits the platform's full `std::io::Error` Display (e.g. `Permission denied (os error 13)`). The errno tag is preserved as a low-severity diagnostic aid (helps users distinguish EACCES vs. EISDIR vs. EROFS); the fixed prefix `Error: Could not read tracker data: ` carries the spec-stable shape.
- RT Review 10 Finding 2 (spec-wording drift + low-severity errno reconnaissance).
- Why: the errno value (13/21/30/etc.) is portable diagnostic signal that aids the user. No path, username, or filesystem-layout data leaks. Single-user local CLI threat model: the user is the only party reading their own stderr; reconnaissance value is zero. Broadening the spec to ratify the actual emit is cheaper than rewording the implementation, and preserves a debugging aid users would otherwise lose.

**SA R11 F1 + SA R13 F1 Trigger B + SA R13 F2 auto-Backlog per CLOSURE-PROTOCOL.md §3** — the "pre-Layer-7 focused PR" deferral set by SO Review 21 expired without action when Layer 7 opened. Per CLOSURE-PROTOCOL.md §3 (auto-Backlog after 3 consecutive reviews of the originating domain without adjudication), the three findings transition from `Deferred` to `Backlogged` (Solution Owner scope). They remain out-of-Layer-7-scope and may be picked up as a focused PR at any future layer's discretion or at portfolio-closeout polish.
- SO Review 23 Finding 1 / SA Review 15 Finding 1 / VDD-IAR Review 17 Finding 4 (process pattern).
- Why: the deferral pattern works only when the deadline is enforced. The three findings persisted across Layer 4 close (SA R11), Layer 5 close (SA R12), Layer 6 close (SA R13-14), with the named "pre-Layer-7" deadline missed. CLOSURE-PROTOCOL.md §3's auto-Backlog rule prevents silent indefinite deferral by promoting unadjudicated multi-round Opens to Backlog at the originating domain's authority. The architectural concern is real (SA's analysis stands: `cmd_list` rendering should be its own function; `src/lib.rs` is past the 500-LOC threshold; `format_show_block` column widths are magic numbers) but the cost-benefit calculus for a focused refactor PR has not shifted enough to schedule it in any specific upcoming layer. Backlogging captures it without commitment to a specific layer.
