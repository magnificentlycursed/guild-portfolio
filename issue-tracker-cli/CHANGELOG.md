# Changelog

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
