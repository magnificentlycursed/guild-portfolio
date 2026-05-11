# Changelog

## Layer 7 implementation — TTY color output — 2026-05-11 22:00Z

**Scope:** Phase 2b implementation of the polish layer per TODO.md L350-396. Adds TTY-detected color output for `priority` and `status` value cells in `tracker list` and `tracker show`. Per DESIGN.md "Interface / color output": color applies only to value text (header rows and label columns are uncolored), only when stdout is a TTY; piped stdout (`tracker list | cat`) suppresses ANSI codes entirely.

### Added

- **src/lib.rs** — Color helper section above the `Tracker` definition: `ANSI_RESET` constant; `priority_ansi(priority, use_color)` and `status_ansi(status, use_color)` return `Option<&'static str>` ANSI start sequences (None for default-color values `low` / `open` and when `use_color` is false); `wrap_color(value, ansi)` centralizes the value-only wrapping contract; `pad_after_color(colored, visible_chars, total_width)` sidesteps Rust's byte-count padding bug for ANSI-wrapped strings. Raw ANSI escapes (no `anstyle` / `termcolor` dependency) — the six sequences (`\x1b[1;31m` bold red, `\x1b[33m` yellow, `\x1b[36m` cyan, `\x1b[32m` green, `\x1b[0m` reset) are universally supported by VT100-compatible terminals, the only environment this single-user portfolio CLI targets.

### Changed

- **src/lib.rs / `cmd_show`** — TTY detection via `std::io::stdout().is_terminal()` (stable since Rust 1.70). Result threaded through to `format_show_block(issue, use_color)`.
- **src/lib.rs / `format_show_block`** — Signature gains `use_color: bool`. Value text for `status` and `priority` rows is wrapped via `wrap_color`; the 13-char label column ("Status:      ", "Priority:    ") remains uncolored.
- **src/lib.rs / `cmd_list`** — TTY detection at the top of the function. Header row (`ID Status Priority Labels Title`) is never colored. Per-issue row formatting now emits status and priority as pre-padded cells (`pad_after_color`) instead of relying on Rust's `{:<width}` formatter — the latter pads by byte length and would over-count ANSI escape bytes, mis-aligning the Labels column when color is active.
- **src/lib.rs#tests** — `multiline_description_show_format` and `show_label_column_right_padded_to_13` updated to pass `false` for the new `use_color` parameter of `format_show_block` (both tests verify uncolored layout; coloring is verified by the integration `*_piped_has_no_ansi_codes` tests and the manual TTY checklist).

### Tests

- **tests/layer7.rs** — Already added in the Phase 2a Red Gate commit (7b461aa). All 9 tests continue to pass against this implementation. The two `*_piped_has_no_ansi_codes` tests now serve their intended regression-guard role: a `println!("\x1b[...")` without TTY detection would break them. `assert_cmd::Command` invokes the binary with stdout connected to a pipe (non-TTY), so the piped branch of `is_terminal()` is exercised by every existing integration test across layers 1-7 — none regressed.

### Verification

- `cargo test --no-fail-fast --locked` — **195/195 pass** (62 unit + 32 layer1 + 18 layer2 + 9 layer3 + 25 layer4 + 7 layer5 + 33 layer6 + 9 layer7).
- `cargo clippy --all-targets --locked -- -D warnings` — clean.
- `cargo fmt --check` — clean.
- Manual TTY verification via `script -q /dev/null tracker list ...`:
  - `list --status open` (issue with priority=low): no escape sequences ✓
  - `list --status in-progress` (priority=high): `\x1b[36min-progress\x1b[0m` cyan, `\x1b[1;31mhigh\x1b[0m` bold red ✓
  - `list --status done` (priority=medium): `\x1b[32mdone\x1b[0m` green, `\x1b[33mmedium\x1b[0m` yellow ✓
  - `show 1` piped: no escape sequences; `show 1` via `script`: status / priority value cells colored, label column uncolored ✓
  - Column alignment preserved across all colored / uncolored combinations (visible-width padding correct in all 9 status × priority combinations).
- `cargo audit` — pending pre-IAR check.

### Open (process)

- **Layer 7 manual testing checklist** (TODO.md L368-374) — 7 unchecked items. Director must execute and commit per CLOSURE-PROTOCOL.md merge-gate criterion 3 (same standing process Open as Layers 4 / 6). Carry-forward for the IAR session.
- **IAR** — Layer 7 active domains per TODO.md: SO, SA, QE, SE, UX, Platform, VDD-IAR Alignment.

---

## Layer 6 IAR Round 3 — `next_id` persistent counter (SO Review 22 Option A) — 2026-05-11 04:30Z

**Scope:** Resolves SO Review 22 Finding 1 — a director-raised spec violation surfaced by Layer 6 manual testing (TODO.md:311 "ID not reused"). The pre-R22 `max(existing_ids) + 1` id-assignment did not honor the "deleted ID never reused, including after deletion" invariant in the high-edge case (delete the highest-id issue, then create — reassigned the deleted id). Option A restores the persistent `next_id` counter that SA Review 3 Finding 3 had removed, reversing two prior decisions (SA R3 F3 "no stored counter" and SO R7 "bare top-level array") in favor of honoring the spec contract.

### Changed

- **DESIGN.md** — Data Model / Storage file: shape changes from `[Issue]` to `{"issues": [Issue], "next_id": u64}`. Storage invariants: `next_id >= 1`; if `issues` is non-empty, `next_id > max(issue.id)`; on create the new issue's id is `next_id` and the counter bumps via `checked_add(1)`; on delete the counter is unchanged. Feature 5 Invariants: rewrote the "never reused" sub-claim that previously asserted (falsely) "`max(remaining_ids) + 1` will always be greater than the deleted ID" — now references the persistent counter explicitly with SO R22 lineage citation.
- **src/lib.rs** — New `pub struct Tracker { issues: Vec<Issue>, next_id: u64 }`. `load_issues` / `save_issues` replaced by `load_tracker` / `save_tracker` (the bare-array shape is rejected at load with the standard corrupt-data message — serde deserializes the wrong shape as a parse error). `next_id(&[u64])` pure helper removed and replaced by `bump_next_id(u64)` (overflow defense via `checked_add(1)` preserved — Security R4 F2 lineage unbroken). `issues_collection_invariants_hold` replaced by `tracker_is_valid` (whole-tracker validation including unique-IDs, per-issue field validity, and counter invariants). `cmd_create` reads `tracker.next_id`, assigns, bumps. `cmd_delete` removes from `tracker.issues`, leaves `next_id` untouched. `cmd_status` / `cmd_show` / `cmd_list` load through the new `Tracker` shape. Doc-comments throughout updated to reflect the new contract (notably `cmd_delete` no longer self-justifies with the false "max + 1 strictly greater than deleted id" claim).
- **src/lib.rs#tests** — Removed three obsolete unit tests pinning the old `max+1` contract (`id_assignment_first_issue_is_1`, `id_assignment_increments_from_max`, `id_assignment_at_u64_max_returns_error`, `max_id_plus_one_skips_deleted_ids`, `collection_invariants_*`). Added: `bump_next_id_increments_by_one`, `bump_next_id_at_u64_max_returns_error`, `tracker_validation_rejects_duplicate_ids`, `tracker_validation_accepts_unique_ids`, `tracker_validation_rejects_next_id_zero`, `tracker_validation_rejects_next_id_not_greater_than_max_id`, `tracker_validation_accepts_next_id_strictly_greater_than_max`, `tracker_validation_accepts_empty_with_next_id_1`, `tracker_validation_accepts_empty_after_all_deleted_with_retained_counter`, `high_edge_delete_does_not_reuse_id` (the SO R22 regression at the unit level), `middle_gap_delete_does_not_reuse_id` (companion case). Net: +5 unit tests.
- **tests/layer6.rs** — `delete_id_not_reused` split into two named tests: `delete_id_not_reused_middle_gap` (the prior coverage) and `delete_id_not_reused_high_edge` (the SO R22 director-reproduction case — pins `Created issue #3: Third` after delete of #2 from {#1,#2}, and asserts `next_id == 4` in the persisted JSON). The high-edge test would have failed pre-R22 and pins the regression. Net: +1 integration test.
- **tests/layer{1,2,3,4,6}.rs** — All `tracker.json` reads updated: `v[0]["x"]` → `v["issues"][0]["x"]`; `v.as_array()` → `v["issues"].as_array()`. All hand-crafted corrupt-data JSON literals re-wrapped from `[{...}]` to `{"issues":[{...}],"next_id":N}` with a valid counter (the per-issue corruption being tested still triggers rejection through the `issue_fields_are_valid` / `tracker_is_valid` path). The `u64_max_id_in_json_blocks_next_create_with_clean_error` test renamed to `u64_max_next_id_in_json_blocks_next_create_with_clean_error` and now plants `next_id: u64::MAX` (with a valid issue) — overflow surfaces one layer earlier (on the counter, not on a derived value) but the error message is unchanged.
- **DECISIONS.md** — SA Review 1 Finding 3 ("ID assignment via `max(existing_ids) + 1`") and SO Review 7's "bare top-level array" entries annotated as **Reversed by SO Review 22**. New section "Layer 6 spec amendments — SO Review 22" added documenting Option A with rationale, trade-off, and lineage back to SA R3 F3.
- **TODO.md** — Layer 6 manual-testing checklist tick for "ID not reused" (line 311) — closed by the persistent counter, verified end-to-end. The bonus row's shell-quoting note (`\r` in double-quoted shell string is a literal, not CR) recorded as a non-finding.

### IAR

- **SO Review 22 Finding 1** (director-raised from manual testing) — Resolved via Option A (this commit). The pre-R22 implementation was correct against SA R3 F3's threat-model simplification rationale but incorrect against the spec invariants DESIGN.md asserts in three places. Option A makes the implementation match the spec; the alternative (Option B — weaken the spec to match `max+1`) was considered and rejected per the SO R22 sycophancy-guard dismissal tests.

### Open (process)

- **VDD-IAR (next round)** — Verify the Round-3 closure: spec/code/test/doc consistency around the new counter, regression coverage at the high edge, prior `max+1` mutation surfaces now closed.
- **Layer 6 manual checklist** — line 311 ticked this round; remaining items (12 of 13 pre-R22) still pending director closure per the standing Round-2 process Open finding (carry-forward).

### Verification

- `cargo test --no-fail-fast --locked` — **186/186 pass** at Round 3 close (62 unit + 32 layer1 + 18 layer2 + 9 layer3 + 25 layer4 + 7 layer5 + 33 layer6).
- `cargo clippy --all-targets --locked -- -D warnings` — clean.
- `cargo fmt --check` — clean.
- Director's manual-test reproduction (delete #2 from {#1,#2}, create "Third"): new id = 3 ✓, `tracker.json` shows `next_id: 4` ✓.
- Bonus `--description $'line1\rOVER'` (ANSI-C quoting for real CR): rejected with `Error: Description cannot contain control characters other than newline.` exit 1 ✓.

---

## Layer 6 IAR Round 2 closure — 2026-05-11 02:00Z

**Scope:** Resolves the substantive Open finding cluster surfaced by Layer 6 Round 1 cold-batch IAR (Security R9 / RT R8 / DE R9 / SE R15 / QE R15 / SO R20 / SA R13 / UX R8 / TW R9). Lands DESIGN.md spec amendments (SO authority), `src/lib.rs` defenses + the `CreateArgs` struct extraction (SE authority), tests (QE authority), `show` / `delete` `--help` parity (UX authority), and doc updates (TW authority).

### Changed

- **DESIGN.md** — Feature 1: description now must reject control characters other than `\n`; new error state `Error: Description cannot contain control characters other than newline.`. Edge Cases / Description: enumerated the rule, the `\n` carve-out rationale, the `\r\n` → `\n` normalization defense, and the bidi (Cf) accepted-risk stance (same posture as title and labels — single-user CLI, risk owner: director). Edge Cases / Storage: control-char-in-description added to corruption triggers. "Show output format": ratified the `\r\n` → `\n` normalization that the implementation does for legacy stored data / external-editor round-trips.
- **src/lib.rs** — `validate_description` extended to reject `char::is_control()` except `\n`; new `description_is_valid` helper enforces the same rule at load time via `issue_fields_are_valid`. Same lineage as `parse_label` + `label_is_valid` from Layer 4 R2. New public `CreateArgs<'a>` struct bundles the four `cmd_create` inputs (title / description / priority / labels); `cmd_create`'s signature collapses from 5 parameters to 2 (`args: &CreateArgs`, `issues_path: &Path`), discharging SA R13 F1 Trigger A (CreateArgs refactor scheduled at SA R7 F4 / R8 F4 / R10).
- **src/main.rs** — `Commands::Create` arm constructs `CreateArgs` and passes it through to `tracker::cmd_create`. `Show` and `Delete` doc-comments expanded to match the Layer 1-4 `--help` depth standard (UX R8 F1) — `show` now documents the full set of fields rendered; `delete` references the D1 deviation and the no-confirmation rule.
- **tests/layer6.rs** — +12 integration tests covering description Cc rejection (ESC, CR, CRLF, tab, DEL, OSC 8 hyperlink), `\n` carve-out acceptance, verbatim-with-whitespace storage (kills the `Ok(raw.trim().to_string())` mutation per QE R15 F3), load-time corruption rejection for control-char and CR in description, `\n` accepted at load, and exact-full-block `show` rendering (kills the over-padding mutation per QE R15 F1).
- **src/lib.rs#tests** — +10 unit tests covering `validate_description` empty/whitespace/Cc rejection, `\n` carve-out, verbatim-stored-with-whitespace, printable Unicode, and `issue_fields_are_valid` description-Cc rejection + `\n` acceptance + None acceptance.
- **CHANGELOG.md** — Layer 6 entry retrospective per TW R9 F1 (the cold reader's primary handoff document was stale at Layer 6 landing — repeating the Layer 4 pattern).

### IAR

Round 1 cold-batch — 11 domain reviews (SO 20, SA 13, QE 15, SE 15, Security 9, Platform Engineer 10, UX 8, Data Engineer 9, Red Team 8, Technical Writer 9, VDD-IAR Alignment 15). Verdict: NO-GO-PENDING-MANUAL + Round 2 required. The substantive Open findings resolved in this Round 2 commit:

- **Security R9 F1 / RT R8 F1 / DE R9 F1 / SE R15 F1 / QE R15 F2 / SO R20 F3** (description control-char defense — Open Medium-High; the third consecutive layer to surface the same generalization-failure pattern, the prior two being Title L1 and Labels L4 R7 F1) — resolved by the DESIGN.md amendment + `validate_description` + `description_is_valid` + 12 new tests.
- **SO R20 F2** (`format_show_block` `\r\n` normalization undeclared in DESIGN.md) — resolved by ratifying the normalization in the "Show output format" spec section.
- **QE R15 F1** (over-padding mutation survives substring assertions on 6 of 8 show rows) — resolved by the new `show_renders_exact_full_block_for_single_line_issue` test using full-line equality on all 8 rendered rows.
- **QE R15 F3** (verbatim-storage half of description postcondition untested) — resolved by `create_preserves_description_verbatim_with_surrounding_whitespace` plus the unit test `description_stored_verbatim_not_trimmed`.
- **SE R15 F2 / DE R9 F2** (bare `\r` overprints `show` alignment) — subsumed by the broader Cc-except-`\n` rejection rule; `\r` is now rejected at create time and at load time.
- **UX R8 F1 / TW R9 F2** (`show` / `delete` `--help` one-line stubs vs. Layer 1-4 standard) — resolved by expanded doc-comments in `src/main.rs`.
- **SA R13 F1 Trigger A** (CreateArgs refactor scheduled for Layer 6) — resolved by the new `CreateArgs<'a>` struct.
- **TW R9 F1** (CHANGELOG missing Layer 6 entry) — resolved by this entry.
- **RT R8 F2** (Trojan-Source / Cf in description) — Accepted Risk per the DESIGN.md amendment carve-out. Same posture as RT R6 F3 / R8 for title and labels (single-user local CLI threat model; risk owner: director).

### Deferred (named future layer)

- **SA R11 F1 / SA R13 F2** (rendering-half of `cmd_list` extraction; `format_show_block` column-width literals as second instance) — focused pre-Layer-7 PR.
- **SA R13 F1 Trigger B** (`src/lib.rs` storage/validate/commands module split — 665+ LOC over the 500-line threshold) — bundled into the same pre-Layer-7 PR per SO adjudication (SO Review 21).

### Open (process)

- **VDD-IAR R15 F1 / SO R20 F1 / TW R9 F4** (Layer 6 manual testing checklist 13/13 unchecked) — director must execute the checklist and commit before merge per CLOSURE-PROTOCOL.md merge-gate criterion 3. Same standard as Layer 4 R11 F2.

### Verification

- `cargo test --no-fail-fast --locked` — **180/180 pass** at Round 2 close (57 unit + 32 layer1 + 18 layer2 + 9 layer3 + 25 layer4 + 7 layer5 + 32 layer6).
- `cargo clippy --all-targets --locked -- -D warnings` — clean.
- `cargo fmt --check` — clean.
- Manual testing checklist (TODO.md Layer 6) — pending (process Open).

---

## Layer 5 — compound filtering — 2026-05-07 00:43Z

**Scope:** Closes the layer-shipping commits for Layer 5 of the assignment
build sequence ("Compound filter — `--status`, `--priority`, `--label` AND-
combined; correct empty-state messaging"). Layer 5's externally observable
behavior was already emergent from the chained `retain()` calls added in
Layer 3 (`--priority`) and Layer 4 (`--label`); this layer extracts the
AND-logic into a named pure predicate (`issue_matches_filters`) so it is
testable in isolation, adds explicit acceptance-criterion coverage of the
compound paths, and ratifies the manual-testing checklist.

### Changed

- **`src/lib.rs`** — added private `issue_matches_filters(&Issue, &str,
  Option<&str>, Option<&str>) -> bool` predicate AND-combining the required
  status filter and the optional priority/label filters; refactored
  `cmd_list`'s three chained `retain()` calls into a single `retain()` over
  the predicate. Behavior unchanged; unit-testability gained. Rustdoc on the
  predicate documents the caller obligation: status/priority comparisons
  assume lowercased filter values; label comparison is case-sensitive
  exact-match and the caller is responsible for trim normalization (per
  DESIGN.md Edge Cases / Labels trim-on-store / trim-on-filter symmetry).
- **`src/lib.rs#tests`** — Red Gate (commit `7d1ca57`): 5 unit tests against
  the `todo!()` stub covering all-three-match, three single-mismatch
  subcases, status-only wildcard, status-mismatch-with-optionals-absent, and
  case-sensitive label match. Round 2 (commit `7f9bae4`):
  `filter_and_logic_is_not_or_between_optional_conjuncts` defense-in-depth
  added per QE Review 13 Finding 1 — kills the inter-conjunct `&&`→`||`
  mutation that all five Round-1 unit tests survive.
- **`tests/layer5.rs`** — Red Gate (commit `7d1ca57`): 7 integration tests
  covering the three two-filter AND combinations (status+priority,
  status+label, priority+label), the three-way AND, the two-filter no-match
  filter-message branch, the three-filter no-match filter-message branch,
  and the default-view-non-empty no-filter-message branch. All seven are
  Cat B Red Gate deviations (the AND-combination was emergent from prior
  layers' chained retains; the integration tests are regression coverage
  of acceptance criteria, not Red Gate gating for new behavior — same
  disposition as Layer 3's `create_without_priority_defaults_to_medium`
  and Layer 4's two Cat B deviations).
- **`TODO.md`** — Layer 5 acceptance-criteria checkboxes flipped to `[x]`
  (commit `bd15a9d`); manual-testing checklist flipped after human
  verification (commit `da0fd8d`); manual-testing setup wording amended in
  Round 2 (commit `7f9bae4`) per SO Review 18 Finding 3 to enumerate the
  `tracker create` invocations and the `tracker status 3 done` step
  required to produce the `(done, high, bug)` issue.
- **`src/lib.rs`** (`cmd_list` comment) Round 2 — replaced the anticipatory
  `--description-contains` example with neutral "any new filter the spec is
  amended to add" framing plus a DESIGN.md "Out of Scope" citation, per SO
  Review 18 Finding 1 (DESIGN.md excludes text-search filtering).
- **`tests/layer5.rs`** (`list_priority_and_label_filter_and_combination`
  docstring) Round 2 — trimmed false claim that one of the setup issues is
  in-progress (per SO Review 18 Finding 2). Test assertions unchanged.

### IAR

Layer 5 Round 1 cold-batch — 5 domain reviews (SO 18; SA 11; QE 13; SE 13;
VDD-IAR 13). Verdict: 5 substantive Low Open findings + 1 Medium-severity
carry-forward (SA R11 F1, rendering half of `cmd_list` extraction —
deferred to focused pre-Layer-7 PR per prior SA R10 disposition). Round 2
warm-resolution closure pass — 4 domain reviews (SO 19; SA 12; QE 14;
SE 14) + VDD-IAR 14 merge-gate verdict: GO. The 5 substantive findings
closed inline in commit `7f9bae4`; the 1 carry-forward holds named-future-
layer disposition. MVR reached.

### Verification

- `cargo test --locked --no-fail-fast` — **136/136 pass** at gate close
  (45 unit + 32 layer1 + 18 layer2 + 9 layer3 + 25 layer4 + 7 layer5).
- `cargo clippy --all-targets --locked -- -D warnings` — clean.
- `cargo fmt --check` — clean.
- Manual testing checklist (TODO.md Layer 5) — six items executed by the
  director; all six expected outputs reproduced.

---

## Layer 4 IAR Round 2 closure — 2026-05-06 02:30Z

**Scope:** Resolves the Open finding cluster surfaced by Layer 4 Round 1
cold-batch IAR (Security R7 / RT R6 / DE R7 / SE R11 / QE R11 / UX R6 /
SO R16 / TW R7). Lands DESIGN.md spec amendments (SO authority), src/lib.rs
defenses (SE authority), tests (QE authority), and metadata fixes (Cargo.toml
`repository`). Closes the label control-character / comma defense cluster,
the error-message escape-interpolation defense, and the create/list filter
trim asymmetry. Manual testing for Layer 4 was completed in the prior commit
`b0a3789`.

### Changed

- **DESIGN.md** — Feature 1: extended `--label` preconditions to add
  control-character and comma rejection; clarified that labels are trimmed at
  storage. Feature 2: filter value is trimmed before comparison; empty/
  whitespace filter is rejected with `Error: Label cannot be empty.`. Edge
  Cases / Labels: enumerated the new rejection rules and the (out-of-threat-
  model) bidi/format/zero-width acceptance stance. Edge Cases / Storage: added
  control-char and comma in stored labels to the corruption triggers. stderr
  contract: error messages interpolating user input MUST escape Cc as
  `\u{XX}`. New "Approved Deviations from Assignment" section codifies the
  `tracker delete` confirmation waiver as director-approved (replaces the
  prior unattributed "advisory" rationale).
- **src/lib.rs** — `parse_label` extended to reject `char::is_control()` and
  the comma character; new `label_is_valid` helper enforces the same rules at
  load time via `issue_fields_are_valid`. New `display_safe` helper escapes
  Cc characters as `\u{XX}`; applied at the three error-formatter sites in
  `parse_priority`, `parse_status`, `parse_id`. `cmd_list` now runs
  `parse_label` on the filter value, closing the trim-asymmetry round-trip
  bug and rejecting empty filters symmetric with create.
- **tests/layer4.rs** — +12 integration tests covering label control-char
  rejection (newline, ESC), comma rejection, load-time corruption rejection
  for both, filter trimming/empty/control-char rejection, and error-message
  escape interpolation (priority, status, id).
- **src/lib.rs#tests** — +11 unit tests covering the new `parse_label`
  rejection rules, `label_is_valid` for stored data, and `display_safe` round
  trips.
- **Cargo.toml** — added `repository = "https://github.com/magnificentlycursed/guild-portfolio"`.
  Closes TW Review 6 Finding 6 sub-item (carried since Layer 3) and TW Review
  7 Finding 4. The `TODO(SO)` comment is removed.

### IAR

Round 2 cold-batch is recommended next per VDD-IAR R11 verdict. The
substantive Open findings from Round 1 that this commit resolves:

- Security R7 F1 (label control-char injection — Open / Raised to SE/QE/SO) —
  resolved by the DESIGN.md amendment + `parse_label` / `issue_fields_are_valid`
  extension + 7 new tests.
- Red Team R6 F1 (Security R7 F1 confirmed + load-path + OSC 8) — resolved by
  the same fix; OSC 8 covered by `is_control()` since ESC is Cc.
- Red Team R6 F2 (error-message escape interpolation) — resolved by
  `display_safe` helper + 3 new integration tests.
- Red Team R6 F3 (Trojan-Source bidi / zero-width) — Accepted Risk per the
  new DESIGN.md "Edge Cases / Labels" out-of-threat-model bullet; risk owner
  is the director (single-user local-CLI threat model).
- Data Engineer R7 F1 (label control-chars at create + load) — resolved by
  the same fix.
- Data Engineer R7 F2 (filter trim symmetry) — resolved by `cmd_list` running
  `parse_label` on the filter value.
- SE R11 F3 (label control-char defense, gated on SO) — resolved.
- QE R11 F4 (no test for label control-char rejection) — resolved by the new
  tests.
- UX R6 F1 (trim-asymmetry round-trip + empty-filter silent-no-match) —
  resolved.
- UX R6 F4 (comma-in-label display ambiguity) — resolved by the comma
  rejection rule.
- SO R16 F1 (label trim-on-store wording) — resolved by the postcondition
  amendment.
- SO R16 F2 (empty filter validation) — resolved (chose option A: validate +
  reject, symmetric with create).
- SO R16 F4 / TW R7 F7 (Dim 9 — delete-with-confirmation deviation) —
  resolved as Approved Deviation D1 in DESIGN.md.
- TW R7 F4 (Cargo.toml `repository`) — resolved.

Open / deferred after this commit:

- SA R9 F1 / SE R11 F2 (cmd_list extraction) — Deferred to a focused PR
  before Layer 7 (color), per SE's rationale: surgical inline conflates
  concerns with Layer 7 prep.
- UX R6 F2 (clap-voice multi-label error) / UX R6 F3 (no `--help` examples) /
  TW R7 F6 (`--help` valid-value asymmetry) — Deferred to Layer 7 polish.
- TW R7 F5 (PROCESS.md retrospective placeholders) — Open; developer-only;
  director must fill or restructure before Layer 4 merge.
- TW R7 F2 (CHANGELOG missing Layer 4 entry) — see "Layer 4 — labels (Round 1)"
  entry below.

### Verification

- `cargo build --locked` — clean.
- `cargo test --locked` — **123/123 pass** (39 unit + 32 layer1 + 18 layer2
  + 9 layer3 + 25 layer4). Up from 100 by +11 unit + +12 layer4.
- `cargo clippy --all-targets --locked -- -D warnings` — clean.
- `cargo fmt --check` — clean.
- Adversarial smoke tests against the release binary confirm: newline label
  rejected; ESC label rejected; comma label rejected; ESC `--priority` value
  renders as `\u{1B}[31mPWN\u{1B}[0m` in stderr (escaped — no raw ESC byte);
  `tracker list --label ""` rejected with `Error: Label cannot be empty.`;
  `tracker list --label "  bug  "` matches a stored `bug`.

---

## Layer 4 — labels (Round 1) — 2026-05-05 11:30Z

**Scope:** Closes the layer-shipping commits for Layer 4 of the assignment
build sequence ("Add label support; display + filter by labels"). Pulled into
CHANGELOG retroactively per TW Review 7 Finding 2 — the layer's commits
shipped in `14bd219` (Red Gate), `ec5c966` (implementation), `0ad83de`
(top-level `--help` discoverability — pulled forward from Layer 7), `f036d8d`
+ `5b95911` (suite-level IAR commits affecting this project's manual-testing
standard and `--help` examples). The CHANGELOG was stale through the entire
Layer 4 manual testing window and Round 1 IAR pass; this entry restores
"first place a maintainer looks" parity with the implementation state.

### Changed

- **`src/main.rs`** — added `--label <l>...` (repeatable) to `Create` clap
  variant; added `--label <l>` (single-value `Option<String>`) to `List` clap
  variant. Top-level `--help` doc-comments updated to mention `--priority` and
  `--label` for discoverability (commit `0ad83de`).
- **`src/lib.rs`** — added `parse_label` (trim + non-empty validation),
  `dedupe_labels` (first-occurrence preservation, case-sensitive),
  `label_matches` (exact-match filter); added `labels: Vec<String>` to
  `Issue`; extended `cmd_create` to consume `--label` arg, parse / dedup, and
  store; extended `cmd_list` to render the `Labels` column (comma-separated,
  20-char truncate with `…`, `(none)` for empty) and AND-combine the new
  `--label` filter; extended `issue_fields_are_valid` with empty-label
  rejection.
- **`tests/layer4.rs`** — Red Gate (commit `14bd219`): 12 integration tests
  + 3 unit tests in `src/lib.rs` covering label storage, ordering, dedup,
  empty rejection, comma-separated rendering, `(none)` empty state, 20-char
  truncation, exact-match filter, case-sensitive filter, and multiple-flag
  rejection on `list`.
- **`SE Review 11`** inline fix (commit b4f2db1) — refactored
  `is_default_open_view` derivation in `cmd_list` to extract the new-filter
  disjunction into `extra_filter_active`, discharging SA Review 9 Finding 2.
- **`tests/layer4.rs`** Round 1 strengthening (commit b4f2db1) — added
  `create_preserves_label_case_at_storage`; tightened
  `list_multiple_label_flags_exits_one` from `contains("Error:")` to the
  full clap-message text; added negative `Nice work!` assertion to
  `list_label_filter_is_case_sensitive`.
- **`README.md`** Round 1 (commit b4f2db1) — Layer 4 status block updated;
  Commands block synopses now show `--label`; Test section sentence
  broadened to current coverage.

### IAR

Layer 4 Round 1 cold-batch — 11 domain reviews (SO 16 + Dim 9 addendum;
SA 9; Security 7; SE 11; QE 11; UX 6; PE 9; DE 7; TW 7; RT 6; VDD-IAR 11).
Verdict: NO-GO-PENDING-ROUND-2 (23 Open findings across 9 domains). Round 2
resolution lands in commit b0a3789 (manual testing) + this commit's parent
(Round 2 closure).

### Verification

- `cargo test --locked` — **100/100 pass** at Round 1 close (28 unit + 32
  layer1 + 18 layer2 + 9 layer3 + 13 layer4).
- `cargo clippy --all-targets --locked -- -D warnings` — clean.
- `cargo fmt --check` — clean.

---

## CI hotfix: self-crate license — 2026-05-05 18:30Z

**Scope:** Restores green CI after `cargo deny --locked check` (added in the
Layer 3 follow-up resolution pass) flagged `error[unlicensed]: tracker = 0.1.0
is unlicensed`. The `Cargo.toml` `license` field was the unresolved half of TW
Review 6 Finding 6, which had been Raised-to-SO without adjudication across
SO Reviews 10–14. The new `cargo deny` step surfaced the latent gap on its
first run.

### Changed

- **`Cargo.toml`** — added `license = "MIT OR Apache-2.0"` to `[package]`. The
  choice matches the Rust ecosystem norm, `deny.toml`'s existing `[licenses]`
  allowlist, and TW Review 6 Finding 6's own proposal text. The
  `TODO(SO)` comment was trimmed: the `license` reference removed, the
  `repository` reference retained (still pending — not CI-blocking).

### IAR

- **SO Review 15** — adjudicates the license sub-item of TW Review 6 Finding
  6 as Resolved. The `repository` sub-item carries forward as Open with the
  auto-Backlog clock now started (CLOSURE-PROTOCOL Section 3). Notes the
  process datum that pre-protocol Open findings need either a one-time
  backfill sweep against the 3-review rule or explicit guidance that the
  protocol applies only to findings raised after its adoption date.
- **Platform Engineer Review 8 Update** — records the diagnostic detail
  (cargo-deny licenses gate fires on the workspace crate itself, not just
  the dependency graph; F2's enforcement made the previously-latent gap
  CI-visible — working as designed). Notes the `license-not-encountered`
  warnings on broader allowlist entries are informational and intentionally
  not narrowed.

### Open after this commit

- **`Cargo.toml` `repository` field** — still Raised-to-SO; not CI-blocking;
  re-raise on external-distribution trigger or Layer 4+ explicit director
  call.
- **Distribution-readiness:** if external distribution is ever planned, the
  matching `LICENSE-MIT` and `LICENSE-APACHE` text files must be added — the
  SPDX field declares the offer, but the licenses' attribution clauses
  require the texts to be present at distribution time. Not blocking; flagged
  in SO Review 15 so it isn't lost.

### Verification

- `cargo build --locked --quiet` — clean.
- `cargo test --locked --quiet` — **84/84 pass** (unchanged from prior
  commit; license metadata does not affect codegen).
- `cargo deny check` — not installed on the dev machine; next CI run is the
  validation point. The targeted error
  (`error[unlicensed]: tracker = 0.1.0 is unlicensed`) is directly addressed
  by the SPDX field.

---

## Layer 3 IAR closure: VDD-IAR protocol — 2026-05-05 13:00Z

**Scope:** Closes the two remaining VDD-IAR Review 10 findings (F1 process side
and F2) by drafting `iterative-adversarial-refinement/CLOSURE-PROTOCOL.md`,
a project-scoped document that codifies finding lifecycle, domain authority
over project artifacts, the auto-Backlog rule for long-running findings, the
cold-batch + warm-resolution cadence, and the explicit merge gate. Also
references the protocol from the project IAR README's "Merging gate" section.

### Added

- **`iterative-adversarial-refinement/CLOSURE-PROTOCOL.md`** — new file. Seven
  sections: (1) authority table mapping each project artifact to its modify-
  authority domain (closes VDD-IAR F1 process side via documentation;
  motivating case is the SE Review 9 incident); (2) finding lifecycle with
  explicit transition rules; (3) auto-Backlog rule for findings Open across
  three consecutive reviews of the originating domain (derived from SO Review
  14 Coordination notes; closes the long-running-Open pattern Platform F3
  exhibited across Reviews 1/2/3/5/7/8); (4) cross-domain duplicate handling
  convention; (5) the cold-batch + warm-sequential-resolution cadence observed
  effective during Layer 3; (6) explicit merge gate; (7) suite-adoption path
  if the protocol proves useful beyond this project.
- **`iterative-adversarial-refinement/README.md`** — appended one paragraph to
  the "Merging gate" section pointing at CLOSURE-PROTOCOL.md.

### Changed

- **`iterative-adversarial-refinement/VDD-IAR-ALIGNMENT-REVIEW.md`** — Update
  entry marking Review 10 Findings 1 (process side), 2, 3, and 4 all
  Resolved. F3 + F4 closed by commit `87e41c6` (the Layer 3 IAR round-2
  commit). F1 process side + F2 closed by the new closure protocol document.
  No carry-forward Open findings; Review 10 reaches MVR with this update.

### Open after this commit

- **Developer-only:** PROCESS.md retrospective placeholders (TW F8 /
  Portfolio Dim 4) — the empty `*[Your reflection here]*` blocks across
  Layer 1/2/3 remain. Not blocking technical merge per CLOSURE-PROTOCOL
  Section 6 item 7.
- **Suite-level (optional):** the closure protocol is currently project-
  scoped. Promotion to suite-level (per Section 7 of the protocol) would
  require a separate suite-development pass and is not included in this
  commit.

### Verification

- No code changes; no test changes. Build, test, clippy, fmt unchanged
  from the prior commit (`87e41c6`).

---

## Layer 3 spec amendments (SO Review 13) — 2026-05-05 11:00Z

**Scope:** Closes the four open spec questions surfaced by the cold-session
parallel batch and carried forward through the prior resolution pass:
title content sanitization (UX F2/F3, Red Team F1/F3); empty-state stream
discipline (UX F4); forward-compat unknown-fields documentation (DE F3); SE
Review 9 content ratification (VDD-IAR F1 content side). All four amendments
are defect-fix-class or refinement-class — no new features, flags, or
dependencies.

### Changed

- **`DESIGN.md` Feature 1 (preconditions + error states)** — added "`<title>`
  contains no control characters (Unicode general category `Cc`)" precondition
  and the corresponding `Error: Title cannot contain control characters.` error
  state. Closes UX F2 / UX F3 / Red Team F1 / Red Team F3.
- **`DESIGN.md` Feature 2 postconditions** — empty-state branch now states
  "**stderr** prints ...; stdout is empty". Closes UX F4.
- **`DESIGN.md` Interface stdout/stderr contracts** — rewritten to split *data*
  (stdout: issue rows, show key-value blocks, one-line confirmations) from
  *informational status* (stderr: error messages and empty-state messages).
- **`DESIGN.md` Edge Cases / Title** — amended the existing shell-special
  bullet; added a new bullet specifying the control-character rule and
  rationale (newline/CR break the line-per-row contract; tab corrupts column
  alignment; ESC enables terminal-escape injection).
- **`DESIGN.md` Edge Cases / List** — each empty-state line now annotates
  `to **stderr**; stdout is empty`; new closing bullet "Pipe consumers see
  only data records on stdout".
- **`DESIGN.md` Edge Cases / Storage** — forward-compat bullet expanded with
  "They are NOT preserved across writes — any subsequent mutation rewrites
  `tracker.json` with only the documented schema fields, dropping anything
  else." (Closes DE F3.) Same bullet's invalid-domain enumeration extended
  to include control-character titles, empty labels, malformed timestamps,
  `updated_at < created_at`, and duplicate IDs (factually documents the
  prior round's validator extensions).
- **`DECISIONS.md`** — new section "Layer 3 spec amendments — SO Review 13"
  with one entry per finding (control-char rejection, stderr empty state,
  unknown-field non-preservation, SE-9 content ratification + process
  violation split).
- **`src/lib.rs` `validate_title`** — rejects any character with
  `is_control()` after the empty-after-trim check; new error
  `"Title cannot contain control characters."`.
- **`src/lib.rs` `issue_fields_are_valid`** — extended with
  `&& !issue.title.chars().any(char::is_control)` so stored data with a
  control-character title is treated as corrupt (closes the
  hand-edited-`tracker.json` bypass path).
- **`src/lib.rs` `cmd_list`** — empty-state `println!` calls switched to
  `eprintln!`; rustdoc updated to lead with the stderr routing and the
  data-vs-status rationale.

### Added (tests)

- **`src/lib.rs` unit tests** — 6 new: `title_with_newline_is_rejected`,
  `title_with_tab_is_rejected`, `title_with_escape_sequence_is_rejected`,
  `title_with_nul_or_del_is_rejected`,
  `title_with_printable_unicode_is_accepted`,
  `issue_field_validation_rejects_control_char_in_title`. Unit total: 19 → 25.
- **`tests/layer1.rs`** — 4 new integration tests:
  `create_title_with_newline_exits_one`,
  `create_title_with_ansi_escape_exits_one`,
  `create_title_with_printable_unicode_succeeds`,
  `control_char_title_in_json_causes_error_exit`.
  `list_with_no_json_shows_empty_state` renamed to
  `list_with_no_json_shows_empty_state_on_stderr` and switched to assert
  `stdout("")` + `stderr(...)`. Layer1 total: 28 → 32.
- **`tests/layer2.rs`** — `list_all_done_default_shows_empty_state` and
  `list_nonempty_status_filter_with_no_match_shows_filter_message` switched
  to `stdout("")` + `stderr(...)` assertions.
- **`tests/layer3.rs`** — `list_priority_filter_no_match_shows_filter_message`
  switched to `stdout("")` + `stderr(...)` assertions.

### Verification

- `cargo build --locked --all-targets` — clean.
- `cargo test --all-targets --locked` — **84/84 pass** (25 unit + 32 layer1 +
  18 layer2 + 9 layer3). Suite delta: 74 → 84.
- `cargo clippy --all-targets --locked -- -D warnings` — clean.
- `cargo fmt --check` — clean.

### Open after this round

- **Process / housekeeping unchanged from prior round:**
  `tests/common/mod.rs` and `deny.toml` still untracked; full Layer 3 round-2
  + this resolution pass remain uncommitted (VDD-IAR F3 + F4); SE Review 9
  process violation remains an Open VDD-IAR finding (content side closed by
  SO Review 13 Finding 4; process side stands).
- **Platform deferred:** coverage measurement in CI (F3); CI-side secret
  scanning (F7). SO has not adjudicated either yet.
- **Developer-only:** PROCESS.md retrospective placeholders (TW F8 /
  Portfolio Dim 4) remain.

---

## Layer 3 follow-up: Open finding resolution pass — 2026-05-04 16:00Z

**Scope:** Closes Open findings from the cold-session parallel review batch that
ran earlier in the day (SE-10, UX-5, Security-6, Platform-8, DE-6, Red-Team-5,
TW-6 round). Eight domains had real implementation/CI findings; four were
spec-pending (Raised to SO). This pass addresses the implementation/CI side and
documents what remains.

### Added

- **`src/lib.rs`** — `parse_timestamp` helper (RFC 3339 / ISO 8601 parsing via
  `chrono::DateTime::parse_from_rfc3339`); `issues_collection_invariants_hold`
  (cross-record ID-uniqueness check separate from per-record validation).
- **`src/lib.rs` unit tests** — 8 new tests: `id_assignment_at_u64_max_returns_error`,
  `collection_invariants_reject_duplicate_ids`, `collection_invariants_accept_unique_ids`,
  `issue_field_validation_rejects_empty_label`,
  `issue_field_validation_rejects_empty_description`,
  `issue_field_validation_rejects_malformed_timestamp`,
  `issue_field_validation_rejects_updated_before_created`,
  `issue_field_validation_accepts_equal_created_and_updated`. Unit total: 11 → 19.
- **`tests/layer1.rs`** — 6 new integration tests covering the new validation
  paths and the SIGPIPE fix: `duplicate_ids_in_json_causes_error_exit`,
  `empty_label_in_json_causes_error_exit`,
  `malformed_timestamp_in_json_causes_error_exit`,
  `updated_before_created_in_json_causes_error_exit`,
  `list_does_not_panic_on_broken_pipe` (cfg(unix); ~600-row tracker.json,
  reader-end dropped before writer finishes; asserts no `panicked` on stderr
  and exit code != 101), `u64_max_id_in_json_blocks_next_create_with_clean_error`.
  Layer 1 total: 22 → 28. Suite total: 60 → 74.
- **`deny.toml`** — All four supplement-required sections (`[advisories]`,
  `[licenses]` with explicit allowlist, `[bans]`, `[sources]` restricted to
  crates.io). Closes Security Review 6 Finding 4 / Platform Review 8 Finding 2.
- **`Cargo.toml`** — `libc = "0.2"` under `[target.'cfg(unix)'.dependencies]`
  for the SIGPIPE fix.

### Changed

- **`src/main.rs`** — Restore default SIGPIPE handler at process start
  (`libc::signal(libc::SIGPIPE, libc::SIG_DFL)`, `cfg(unix)`, single unsafe
  block with safety rationale). Without this, Rust's runtime ignores SIGPIPE
  and `println!` panics with EPIPE when the reader closes the pipe;
  `tracker list | head` exited 101 with a backtrace on stderr, violating
  DESIGN.md `Error:` stderr contract and the exit-{0,1} set. Closes UX
  Review 5 Finding 1 / Security Review 6 Finding 1.
- **`src/lib.rs`** — `next_id` now returns `Result<u64, String>` and uses
  `checked_add(1)`; defends against hand-edited `tracker.json` planting
  `id: u64::MAX` (debug: panic; release: silent wrap to 0 → schema corruption,
  bricks tracker). `cmd_create` propagates the error. Closes Security Review 6
  Finding 2 / Red Team Review 5 Finding 2.
- **`src/lib.rs` `issue_fields_are_valid`** — Extended with: non-empty labels
  (after trim), non-empty description (after trim, when present;
  forward-compat for Layer 6), parseable `created_at` and `updated_at`
  timestamps, and `updated_at >= created_at`. Closes SE Review 10 Findings
  2/3 and DE Review 6 Finding 1.
- **`src/lib.rs` `load_issues`** — Now also calls
  `issues_collection_invariants_hold` to enforce DESIGN.md "no two issues
  share the same ID" at the collection level (per-record validation cannot
  catch duplicates). Closes SE Review 10 Finding 1 / Security Review 6
  Finding 3 / DE Review 6 Finding 2 / Red Team Review 5 Finding 4.
- **`src/lib.rs`** — Crate-level `#![deny(...)]` extended with
  `clippy::expect_used`, `clippy::panic`, `clippy::missing_errors_doc`. Every
  public `Result`-returning function now has an explicit `# Errors` section in
  its rustdoc. Closes Platform Review 8 Finding 4. (Skipped: `clippy::all`,
  `clippy::pedantic`, `clippy::nursery`, `clippy::missing_panics_doc` — these
  produce significant noise that isn't proportional to a Phase 1 portfolio
  scope; revisit if a Layer 4+ refactor surfaces a relevant defect they
  would have caught.)
- **`src/lib.rs` `cmd_list` rustdoc** — Now describes the post-SO-Review-11
  empty-state semantics (default-open view = no flags or `--status open`
  alone with no other filter; filter view = any other combination). Closes
  TW Review 6 Finding 9.
- **`.github/workflows/issue-tracker-cli.yml`** — All third-party actions
  pinned to commit SHA (`actions/checkout@34e1148...`,
  `dtolnay/rust-toolchain@3c5f7ea...`, `Swatinem/rust-cache@e18b497...`),
  trailing comments document the resolved tag and refresh procedure
  (Platform F1). All cargo invocations gain `--locked` (Platform F5).
  `cargo install` calls pin the tool version (`cargo-audit --version 0.22.1`,
  new `cargo-deny --version 0.19.4`); Platform F6. New step
  `cargo deny --locked check` runs after `cargo audit` (Platform F2).
- **`.pre-commit-config.yaml`** (git root) — `cargo-fmt-check` hook
  `cd "$(git rev-parse --show-toplevel)/issue-tracker-cli"` instead of
  bare `cd issue-tracker-cli` — robust to invocation from any subdirectory.
  Closes Platform Review 8 Finding 8.

### Open after this round

- **Spec-pending (Raised to SO; cannot proceed without DESIGN.md decisions):**
  newline characters in titles breaking the one-issue-per-line list contract
  (UX F2 / Red Team F3); ANSI/control-sequence injection in titles surviving
  storage and being re-emitted by `list` (UX F3 / Red Team F1); empty-state
  message currently on stdout polluting pipe consumers like `wc -l` (UX F4);
  forward-compat unknown-fields silently dropped on rewrite — document the
  constraint (DE F3).
- **Platform deferred:** coverage measurement + threshold in CI (F3); CI-side
  secret scanning to backstop bypassable pre-commit hooks (F7).
- **Process / housekeeping:** `tests/common/mod.rs` and `deny.toml` are
  untracked in git (VDD-IAR F3 expanded); the entire Layer 3 round-2 work
  remains uncommitted (VDD-IAR F4); SE Review 9's DESIGN.md edits at
  lines 218/220-225 still lack explicit SO approval (VDD-IAR F1);
  `gates merge` closure protocol (VDD-IAR F2). PROCESS.md retrospective
  placeholders (TW F8 / Portfolio Dim 4) remain developer-only — IAR rules
  forbid an agent filling these on the developer's behalf.

### Verification

- `cargo build --locked --all-targets` — clean.
- `cargo test --all-targets --locked` — 74/74 pass (19 unit + 28 layer1 +
  18 layer2 + 9 layer3).
- `cargo clippy --all-targets --locked -- -D warnings` — clean with the
  strengthened deny set.
- `cargo fmt --check` — clean.
- `cargo deny check` — not validated locally (`cargo-deny` not installed
  on the dev machine); will be validated on next CI run.

---

## Layer 3 — 2026-05-04 05:40Z

**Scope:** VSDD Phase 2a (Red Gate) + Phase 2b (Implementation). `--priority` flag on `tracker create` and `tracker list`. All 11 Layer 3 acceptance criteria met; manual testing complete.

### Added

- **`src/lib.rs`** — `parse_priority`: parses and normalizes priority strings (case-insensitive; derives from `VALID_PRIORITIES` constant — single source of truth, mirrors `parse_status`). `priority_rank`: maps priority strings to sort order (`high`=0, `medium`=1, `low`=2; unknown→`usize::MAX` as defensive backstop, unreachable for stored data given post-deserialization validation). `sort_issues`: sorts by priority rank then ID ascending. `cmd_create` extended to accept `Option<&str>` priority; defaults to `"medium"`. `cmd_list` extended to accept `Option<&str>` priority filter; AND-combined with status filter.

- **`src/main.rs`** — `Create` subcommand gains `--priority` flag (`Option<String>`). `List` subcommand gains `--priority` flag wired to `cmd_list`.

- **`tests/layer3.rs`** — 9 integration tests covering all Layer 3 acceptance criteria: `--priority` on create (happy path, default to medium, invalid value), priority sort (high→medium→low, ID tie-breaking), `--priority` filter (matching only, invalid value, no-match regression for SO Review 11 fix), and `list_columns_use_exactly_two_space_separator` (locks DESIGN.md line 218 "exactly 2 spaces" column-separator contract). Total suite: 60 tests (49 integration + 11 unit), all passing.

- **`src/lib.rs` unit tests** — 4 unit tests: `priority_parsing_valid_cases`, `priority_parsing_rejects_invalid`, `priority_sort_order_is_correct`, `priority_sort_tie_breaking_by_id`.

### Changed

- **`src/lib.rs` `cmd_list`** — `is_open_view` heuristic now requires no priority filter to be set (`effective_status == "open" && effective_priority.is_none()`). Previously, `tracker list --priority X` with no matches printed "No open issues. Nice work!" instead of "No issues match the given filters.", violating DESIGN.md edge case (line 308). SO Review 11 finding.

- **`src/lib.rs` priority constants** — `VALID_PRIORITIES` removed; `PRIORITY_ORDER` is now the single source of truth for both priority validity (membership check in `issue_fields_are_valid`, `parse_priority`) and sort rank (index in `priority_rank`). Mirrors the `parse_status` / `VALID_STATUSES` unification from SA Review 6. SA Review 7 finding.

### IAR — Layer 3 Reviews

- **SO Review 11:** `is_open_view` empty-state heuristic fixed to consider priority filter (real bug introduced by Layer 3); CHANGELOG entry added; README status updated.
- **SA Review 7:** Priority constants unified — `VALID_PRIORITIES` removed; `PRIORITY_ORDER` is the single source of truth for membership and sort rank.
- **QE Review 9:** Regression test `list_priority_filter_no_match_shows_filter_message` added to lock in SO Review 11 fix; layer3.rs grew 7 → 8 tests at the time of that review (subsequent gate-closure work added the column-separator test, bringing layer3.rs to 9).
- **SE Review 8:** `priority_rank` doc comment added explaining `usize::MAX` defensive fallback and unreachability invariant; `is_open_view` naming clarity raised as Open for human director (Layer 4 helper extraction may resolve).
- **VDD-IAR Review 9:** Layer 3 round 1 process compliance audit. Three Open items gate Layer 3 merge: cold-session pass (only SO was cold-session; SA/QE/SE were same-session as orchestrator), MVR via second IAR pass (round 1 only; SA F2 + SE F2 still Open), and PROCESS.md retrospective backlog (Layer 2 overdue, Layer 3 pending).
- **Gate-closure work (2026-05-04 06:10Z):** SA Review 7 Finding 2 Resolved — `tracker()` test helper extracted to `tests/common/mod.rs`; `tests/layer1.rs`, `tests/layer2.rs`, `tests/layer3.rs` now use `mod common; use common::tracker;`. PROCESS.md Layer 2 + Layer 3 retrospectives added (VDD-IAR Review 9 Finding 9 closed). SE Review 8 Finding 2 (`is_open_view` rename) and remaining MVR confirmation deferred to SE round 2 cold-session pass running in a separate session.

---

## Layer 2 — 2026-05-01 00:00Z

**Scope:** VSDD Phase 2a (Red Gate) + Phase 2b (Implementation). `tracker status` command and `--status` filter for `tracker list`. All 16 Layer 2 acceptance criteria met; manual testing complete.

### Added

- **`src/lib.rs`** — `parse_status`: parses and normalizes status strings (case-insensitive; derives from `VALID_STATUSES` constant — single source of truth). `parse_id`: validates issue IDs as positive integers. `cmd_status`: implements `tracker status <id> <status>` — validates ID and status, finds issue by ID, updates `status` and `updated_at`, writes to storage.

- **`src/main.rs`** — `Status` subcommand added with `id: String` and `status: String` positional arguments. `List` subcommand wired to pass `--status` flag value to `cmd_list`.

- **`tests/layer2.rs`** — 18 integration tests covering all Layer 2 acceptance criteria: status change (happy path, JSON validation, timestamp refresh, field immutability, case-insensitive input, idempotent set), list status filtering (default excludes non-open, explicit status filter, open explicit == default, all-done empty state, no-match filter message), and all error paths (invalid ID string, zero ID, not found, invalid status value, invalid list filter). Total suite at Layer 2 gate close: 41 tests (34 integration + 7 unit, including the 3 unit tests added below), all passing.

- **`src/lib.rs` unit tests** — 3 unit tests: `status_value_parsing_valid_cases`, `status_value_parsing_rejects_invalid`, `id_must_be_positive_integer`.

### IAR — Layer 2 Reviews

- **SA Review 6:** `parse_status` unified with `VALID_STATUSES` — single source of truth for valid status values. Previously `parse_status` had an independent match arm; now it iterates `VALID_STATUSES`. Eliminates the two-source-of-truth gap identified as a deferred item in SA Review 4.
- **SE Review 7:** `cmd_status` refactored from `iter_mut().find()` to `iter().position()` — eliminates unnecessary `new_status.clone()` and the resulting borrow conflict. `new_status` moved into `issues[idx].status`; `println!` reads `issues[idx].status`. Zero clones.
- **QE Review 7:** `list_nonempty_status_filter_with_no_match_shows_filter_message` added — verifies `--status done` with no matching issues prints "No issues match the given filters." Catches the `is_open_view` mutation that survived all 37 prior tests.
- **SO Review 10:** CHANGELOG entry added; README status updated.
- **All other domains:** No findings requiring code changes. Security, Platform, UX, Data Engineer, Red Team all reached MVR for Layer 2.
- **VDD-IAR Review 8:** Dim 4 violation (Category B) logged and closed — `list_explicit_open_filter_matches_default` and `list_all_done_default_shows_empty_state` were written post-implementation without a named finding; both cover documented acceptance criteria (same disposition as `invalid_domain_values_in_json_causes_error_exit`, Layer 1). Two open gate items remain: cold-session review requirement (dim 6), second IAR pass to confirm MVR (dim 7).

---

## Layer 1 gate closure — 2026-04-30 00:00Z

**Scope:** Post-implementation IAR iterations, gate closure work. No new features. No changes to the Layer 1 behavioral contract.

### Added

- **`tests/layer1.rs`** — Two new integration tests:
  - `list_shows_multiple_issues_in_id_order` — creates two issues, runs `tracker list`, asserts both titles appear and issue #1 precedes issue #2 (guards sort direction and two-issue list coverage; general adversarial review finding).
  - `zero_id_in_json_causes_error_exit` — writes `"id": 0` to `tracker.json`, asserts corrupt-data error exit (independently tests the `id > 0` validation branch in `issue_fields_are_valid`; general adversarial review finding).
  - Total: 20 tests (16 integration + 4 unit), all passing.

- **`issue-tracker-cli/.pre-commit-hooks/check-no-home-paths.sh`** — Shell hook rejecting staged files that contain `$HOME` (resolved at runtime; no username hardcoded). Platform Engineer Review 4 finding.

- **`.pre-commit-config.yaml`** (git root) — Pre-commit framework configuration: `detect-private-key`, `no-commit-to-branch` (main), `no-home-dir-paths` (local hook). Platform Engineer Review 1–3 deferred item closed.

- **`PROCESS.md`** — Layer 1 process retrospective: phases, findings caught by IAR, and session notes.

### Changed

- **`src/lib.rs`** — `#![deny(clippy::unwrap_used)]` added at crate level; `#[allow(clippy::unwrap_used)]` with inline safety rationale added at the `serde_json::to_string_pretty` call in `save_issues`; unit test `title_trimmed_before_storage` converted from `.unwrap()` to `assert_eq!` form (SE general adversarial review finding).

- **`src/lib.rs`** — `///` doc comments added to all public items: `Issue` struct, `validate_title`, `next_id`, `current_timestamp`, `load_issues`, `save_issues`, `cmd_create`, `cmd_list`. `cargo doc --no-deps` clean (TW Review 4 finding).

- **`issue-tracker-cli/.gitignore`** — Added `/tracker.json` to prevent accidental commits of local test data (Platform Engineer Review 5 finding).

- **`TODO.md`** — Layer 1 manual testing checklist all checked; status note updated.

### IAR — Gate closure reviews

- **General adversarial review (review-session primer):** QE Review 6 (sort direction + zero-id mutation); SE Review 6 (clippy::unwrap_used); TW Review 4 (rustdoc). Three findings resolved.
- **IAR suite (pre-gate closure pass):** QE Review 5 (`(none)` label assertion), Platform Review 5 (tracker.json gitignore). Two findings resolved.
- **Platform Review 4:** Pre-commit hooks configured; username leakage in prior review log text caught and removed; git history rewritten with `git filter-repo` to remove one historical username occurrence.
- **VDD-IAR Alignment Review 7:** Gate confirmed ready to merge. Premature MVR signals from Reviews 5–6 corrected by adversarial pass.

---

## Layer 1 — 2026-04-28 05:30Z

**Scope:** VSDD Phase 2a (Red Gate) + Phase 2b (Implementation). `tracker create` and `tracker list` commands, core data model, storage layer.

### Added

- **`src/lib.rs`** — Core library: `Issue` struct (serde Serialize/Deserialize), `validate_title`, `next_id`, `current_timestamp`, `load_issues`, `save_issues`, `cmd_create`, `cmd_list`. Post-deserialization domain validation in `load_issues` rejects issues with invalid status, priority, ID, or empty title as corrupt data (Security Review 3 / Data Engineer Review 3 finding).

- **`src/main.rs`** — CLI entrypoint using clap derive: `tracker create "<title>"` and `tracker list` subcommands wired to library command handlers.

- **`Cargo.toml`** — Runtime dependencies: `serde` 1.x (derive), `serde_json` 1.x, `clap` 4.x (derive), `chrono` 0.4.

- **`tests/layer1.rs`** — 14 integration tests covering the full Layer 1 acceptance criteria (create, list, error states, storage correctness, timestamp invariants, truncation). Includes `invalid_domain_values_in_json_causes_error_exit` added in IAR Review 4 (QE Review 4 finding).

- **`src/lib.rs` unit tests** — 4 unit tests covering `validate_title` and `next_id`.

### IAR — Layer 1 Reviews

- **QE Review 3 (cold-session):** Truncation test sharpened (asserts exact 49-char prefix); `create_first_issue_unchanged_after_second_create` extended to cover `labels`, `created_at`, `updated_at`; `malformed_json` test updated to assert distinguishing suffix of the parse-failure message.
- **SE Review 3 (cold-session):** 5 dismissed findings. No defects. `created_at` immutability confirmed.
- **QE Review 4 / Security Review 3 / Data Engineer Review 3:** Post-deserialization domain validation gap identified and resolved — `load_issues` now validates all field domain values after deserialization. New test `invalid_domain_values_in_json_causes_error_exit` added. 18 tests total, all passing.
- **All other domains:** No additional findings. See domain review logs for full detail.

---

## Spec phase — 2026-04-27 21:00Z

**Scope:** VSDD Phase 1 (Spec Crystallization) and Phase 1b (Decomposition). No implementation code. All changes are specification, planning, and process artifacts.

### Added

- **DESIGN.md** — Full behavioral specification for all five commands (`create`, `list`, `status`, `show`, `delete`). Covers preconditions, postconditions, invariants, error states, edge cases, data model, storage contract, interface contract, testing methodology, and out-of-scope exclusions.

- **TODO.md** — 7-layer development plan. Each layer has: a goal statement, specific acceptance criteria, a manual testing checklist, and a Red Gate test plan (behavioral test names established before implementation begins). Covers all DESIGN.md requirements mapped to layers.

- **DECISIONS.md** — Index of key design decisions with rationale: non-atomic writes, ID assignment via max+1, exit codes 0/1 only, non-interactive delete, fixed column widths, library-agnostic spec, post-deserialization validation, description absent-vs-null serialization, and others.

- **IAR suite** — 10 adversarial review domains run against the spec and decomposition:
  - SO Reviews 1–6 (including one cold-session pass): spec coverage, scope compliance, assignment compliance
  - SA Reviews 1–2: architectural decisions, complexity budget, decomposition soundness
  - QE Review 1: Red Gate test plan quality, coverage gaps
  - SE Review 1: spec-level implementation concerns
  - Security Review 1: threat model, input validation design, post-deserialization validation gap
  - Platform Review 1: CI/CD and build requirements
  - UX Review 1: CLI interface design (CLI supplement)
  - Data Engineer Review 1: data model, schema evolution, serialization
  - Technical Writer Review 1: documentation completeness
  - Red Team Review 1: attack surface (user-controlled input, crafted file)
  - VDD-IAR Alignment Reviews 1–2: process compliance, design-before-code, decomposition quality

- **CI pipeline** (`.github/workflows/issue-tracker-cli.yml`) — GitHub Actions workflow running on all pushes and PRs to `issue-tracker-cli/**`: `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`, `cargo audit`.

- **`rust-toolchain.toml`** — Rust 1.94.1 pinned with clippy and rustfmt components.

- **`.gitignore`** — Excludes `/target` from version control.

- **`README.md`** — Project overview, command reference, install/build/test instructions, status tracker, project file index.

### Changed (DESIGN.md — IAR-driven spec refinements)

- **Color output restored** (SO Review 3): layer 7 colored output was incorrectly excluded; restored per assignment Layer 7 scope.
- **Library-agnostic crate references** (SO Review 3): named crates (`clap`, `serde_json`, `atty`) removed from spec; observable interface contract is implementation-agnostic.
- **Character limits removed** (SO Review 3): title and label length limits removed; assignment requires non-empty validation only.
- **Non-interactive delete documented** (SO Review 6): rationale for omitting confirmation prompt recorded in Out of Scope.
- **Labels column width corrected** (SO Review 6): example table updated to match 20-char specified column width.
- **Post-deserialization validation specified** (Security Review 1 / Data Engineer Review 1): semantically invalid field values in structurally-valid JSON now defined as corrupt-data error.
- **`description` absent-vs-null clarified** (Data Engineer Review 1): spec explicitly requires omitting the JSON key when no description is provided, not serializing as null.
- **Stale library reference removed** (SE Review 1): "clap treats `-1` as a flag" replaced with implementation-agnostic language.
- **Sort algorithm clarified** (SA Review 2): Layer 1 spec requires full priority→ID sort algorithm from the start, not a simplified ID-only sort.
- **Red Gate test plan expanded** (QE Review 1): 5 tests added covering `created_at == updated_at` at creation, title and label truncation in list output, `--status in-progress` filter, and `created_at` immutability after status mutation.
