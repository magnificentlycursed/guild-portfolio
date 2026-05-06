# Technical Writer Review Log

This review is part of the [Iterative Adversarial Refinement (IAR)](README.md) suite. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

**Reviewer role: Technical Writer** (Technical Writer / Developer Experience Engineer)

**Activation:** Portfolio project intended for handoff and external review.

**Language supplement applied:** `lang/rust.md` (Technical Writer section).

**Sycophancy check:** An agent generating documentation in the same session as code will produce documentation that is accurate at the moment of generation and stale after the next change. The adversary must verify that documentation describes the current implementation, not the implementation at the time it was written. Every claim in the documentation should be verifiable against the current code. Treat every "this function does X" statement as a claim that requires verification.

---

## Review 1 — 2026-04-27 21:00Z

**Scope:** All documentation artifacts: `DESIGN.md`, `TODO.md`, IAR review logs, project structure. No source code exists.

**Session note:** In-session with all other domain reviews. Acknowledged quality tradeoff. The documentation and spec were authored in the same sessions; the adversary must evaluate them against what a new reader — without session history — would understand.

---

### Resolved

**Finding 3 — No DECISIONS.md (Dim 4 — Decision rationale)**

Significant design decisions were documented in IAR review logs only — not in a dedicated decisions record. A reader understanding "why is atomic write in Out of Scope?" had to find SA Review 1, Finding 1 and read through the review log.

**Resolution:** Created `issue-tracker-cli/DECISIONS.md` with entries for all key decisions from the spec phase: non-atomic writes, ID assignment, description absent-vs-null, post-deserialization validation, exit codes, non-interactive delete, fixed column widths, library-agnostic spec, color output, validation scope, and deliberate exclusions. Each entry includes the source IAR review and the rationale.

---

### Deferred

**Finding 1 — No README.md exists (Dim 1 — README completeness)**

A portfolio project with no README has no entry point for a new reader. There is no way to understand what the project does, how to build it, how to run it, or how to run the tests from the project directory alone. The knowledge transfer test fails immediately: a developer who clones this repository cannot make a meaningful start.

**Classification:** Deferred to Layer 1 gate. A README cannot be written before the project has an implementation — the build and run instructions do not exist yet. Required content when written: project purpose (one paragraph); prerequisites (`cargo`, Rust toolchain version from `rust-toolchain.toml`); how to build (`cargo build --release`); how to install locally (`cargo install --path .`); how to run tests (`cargo test`); a brief command reference (or reference to `--help`). Must be present and accurate before Layer 1 merges.

---

**Finding 2 — No CHANGELOG.md (Dim 8 — CHANGELOG quality)**

No CHANGELOG exists. For a layered portfolio project, each layer gate close is a natural CHANGELOG entry.

**Classification:** Deferred to Layer 1 gate. CHANGELOG should be started when Layer 1 closes, with an entry that describes what was delivered and which IAR findings drove changes. Format: date, layer, features delivered, significant findings resolved.

---

**Finding 6 — rustdoc coverage not yet applicable (Rust supplement)**

No source code exists. No `pub` items to document.

**Classification:** Deferred. Rustdoc review will occur in Review 2 when source code exists. The `cargo doc --no-deps 2>&1 | grep "missing documentation"` check will be part of the Layer 1 gate for any public items in `lib.rs`.

---

### Dismissed

**Finding 4 — DESIGN.md accuracy after multiple review passes (Dim 2)**

DESIGN.md has been through 6 SO reviews, 1 SA review, and 5 other domain reviews. Could stale content have survived?

**Classification:** Dismissed. The current review session identified and resolved two remaining stale references (clap reference in edge cases, column width example mismatch). After those fixes, DESIGN.md reflects the current spec state. The spec is the authoritative source of truth and has been verified to be internally consistent.

---

**Finding 5 — IAR review logs document decisions but are not structured as decision records (Dim 4)**

IAR logs serve double duty: recording adversarial findings AND documenting design rationale (through the Dismissed and Resolved entries). A new reviewer reading SO Review 3 Finding 2 learns why clap was removed from the spec — but only by reading through a review log, not a decisions index.

**Classification:** Dismissed. This is the nature of the VDD-IAR process: the review log is the authoritative process record. DECISIONS.md would be supplementary. The issue is that decisions are findable in the logs — they are not lost. Finding 3 (DECISIONS.md) addresses the discoverability concern. The dual-purpose of IAR logs is not a documentation failure; it is a structural feature.

---

### Hallucinated

*(none)*

---

### Open

*(none)*

---

### Summary

One real finding resolved (DECISIONS.md created). Three deferred findings (README, CHANGELOG, rustdoc coverage) — all expected for a pre-implementation pass. Two dismissed. The documentation artifacts that exist (DESIGN.md, TODO.md, IAR logs) are thorough and accurate. The gap is the user-facing onboarding documentation, which must follow the implementation. README and CHANGELOG are required before the Layer 1 merge gate closes.

**Coordination:** Add README and CHANGELOG to the Layer 1 IAR checklist.

---

---

## Review 2 — 2026-04-28 05:30Z

**Scope:** Layer 1 implementation — all documentation artifacts: `README.md`, `CHANGELOG.md`, `DECISIONS.md`, `TODO.md`, IAR review logs, `src/lib.rs` public API. Evaluating completeness, currency, and handoff quality.

**Session note:** In-session with Layer 1 IAR suite. Acknowledged quality tradeoff.

---

**Regression check:** Review 1 deferred Findings 1 (README), 2 (CHANGELOG), and 6 (rustdoc) to Layer 1. README and CHANGELOG are evaluated below; rustdoc deferred again to Review 4 below.

---

### Resolved

**Finding 1 — README.md exists but was stale (Dim 1 — README completeness) (regression check from Review 1 Finding 1)**

Review 1 deferred README creation to Layer 1 gate. README.md exists and contains: project purpose, command reference, install/build/test instructions, storage explanation, and a status tracker.

Finding: the Layer 1 status was showing as unchecked despite implementation being complete. Status line said "Spec complete. Implementation in progress."

**Resolution:** Updated README.md: Layer 1 status checked; status line updated to "Layer 1 implementation complete. Layer 2 not started." Coordinated with [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) Review 8 Finding 1.

---

**Finding 2 — No CHANGELOG.md (Dim 8) (regression check from Review 1 Finding 2)**

Review 1 deferred CHANGELOG creation to Layer 1 gate. CHANGELOG.md now exists with a spec-phase entry. No Layer 1 implementation entry was present.

**Resolution:** Added a Layer 1 implementation entry to CHANGELOG.md: scope, added files, IAR findings and resolutions, test count.

---

**Finding 3 — DECISIONS.md storage format gap (Dim 4 — Decision rationale)**

DECISIONS.md exists and covers 12 key decisions, but the storage format decision (top-level array) was missing. A reader cannot find why `tracker.json` is a top-level array.

**Resolution:** [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) Review 8 Finding 2 added the storage format entry. DECISIONS.md is now complete for spec-phase and Layer 1 decisions.

---

### Dismissed

**Finding 4 — rustdoc on public `lib.rs` items (Rust supplement — Dim 6)**

`lib.rs` exports: `Issue`, `validate_title`, `next_id`, `current_timestamp`, `load_issues`, `save_issues`, `cmd_create`, `cmd_list`. None have `///` doc comments.

**Classification:** Dismissed. Reaffirming [QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md) Review 3 Finding 4 dismissal: this is a binary's internal library crate exposed for integration testing, not a library API for external consumers. The `pub` visibility is an implementation detail of the testing architecture, not a publication commitment. `cargo doc` would generate empty documentation for these items, which is appropriate — a consumer of this crate is the binary, not an external user. Rustdoc is relevant at Layer 7 if the public API surface is intended for external use; it is not at Layer 1.

---

**Finding 5 — TODO.md Layer 1 checklist items for invalid domain values test (Dim 2)**

The `invalid_domain_values_in_json_causes_error_exit` test was added in [QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md) Review 4 but is not listed in TODO.md's Layer 1 Red Gate section.

**Classification:** Dismissed. The test was added as a consequence of IAR (not a pre-planned Red Gate test) and is correctly classified as an IAR-driven addition rather than a pre-implementation Red Gate test. The Red Gate section in TODO.md documents the tests planned before implementation; additional tests discovered during IAR are recorded in the IAR log. No update to TODO.md is required.

---

### Hallucinated

*(none)*

---

### Open

*(none)*

---

### Summary

Three findings resolved: README.md updated (Layer 1 status correct); CHANGELOG.md Layer 1 entry added; DECISIONS.md storage format entry added. Two dismissed. Documentation is now current for Layer 1. The project is handoff-ready at this layer: a new reader can clone, build, run, and understand why key decisions were made.

**Coordination:** Findings 1 and 3 coordinated with [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) Review 8.

---

---

## Review 3 — 2026-04-30 00:00Z

**Scope:** Layer 1 gate closure pass — TODO.md manual checklist complete; gate closure records added to IAR logs.

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

No TW findings. TODO.md manual testing section all checked. Gate status note updated. IAR review logs current. CHANGELOG accurate. No documentation gaps. MVR reached for Layer 1.

**Coordination:** *(none)*

---

---

## Review 4 — 2026-04-30 00:00Z

**Scope:** General adversarial review, pre-merge gate. Review-session primer loaded. Applying Rust TW supplement (rustdoc coverage).

**Session note:** In-session review. Acknowledged quality tradeoff.

---

### Resolved

**Finding 1 — No rustdoc coverage on public `lib.rs` items (Rust supplement — rustdoc coverage)**

All seven public functions and the `Issue` struct had zero `///` doc comments. This re-evaluates Review 1 Finding 6 (deferred to Layer 1).

**Resolution:** Added `///` doc comments to all public items in `lib.rs`: `Issue` struct (struct-level doc + field semantics noted inline), `validate_title`, `next_id`, `current_timestamp`, `load_issues`, `save_issues`, `cmd_create`, `cmd_list`. `cargo doc --no-deps` produces no warnings. Comments are concise — they describe the contract, not the implementation.

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

One finding resolved: rustdoc coverage added to all public `lib.rs` items. `cargo doc --no-deps` clean. MVR reached for Layer 1.

**Coordination:** *(none)*

---

---

## Review 5 — 2026-05-01 00:00Z

**Scope:** Layer 2 implementation — documentation completeness, currency, and handoff quality. Artifacts reviewed: `README.md`, `CHANGELOG.md`, `PROCESS.md`, `DECISIONS.md`, `src/lib.rs` public API, `tests/layer2.rs`.

**Session note:** In-session with full Layer 2 IAR suite. Acknowledged quality tradeoff. Review-session primer applied.

---

### Resolved

**Finding 1 — CHANGELOG.md missing Layer 2 entry (Dim 8 — CHANGELOG quality)**

Resolved by [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) Review 10 Finding 1. CHANGELOG.md now has a Layer 2 entry. TW confirms the entry is present and documents the scope, features, tests, and IAR findings.

**Resolution:** Resolved via [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) Review 10.

---

**Finding 2 — README.md status stale (Dim 1 — README accuracy)**

Resolved by [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) Review 10 Finding 2. README status block updated, Layer 2 checked, status line current.

**Resolution:** Resolved via [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) Review 10.

---

### Dismissed

**Finding 3 — New public functions `parse_status` and `parse_id` rustdoc coverage check (Rust supplement — rustdoc coverage)**

`lib.rs` now exports `parse_status` and `parse_id`. Both have `///` doc comments:
- `parse_status`: "Parses and normalizes a status string (case-insensitive)..."
- `parse_id`: "Parses an issue ID from a string. Must be a positive integer (>= 1)."

`cargo doc --no-deps` produces no warnings.

**Classification:** Dismissed. Rustdoc coverage maintained.

---

**Finding 4 — PROCESS.md developer reflection sections remain as placeholders (Dim 10 — Retrospective quality)**

PROCESS.md Layer 1 has:
- "What was hardest" — `*[Your reflection here...]*` placeholder
- "What the process felt like" — `*[First-person reflection...]*` placeholder
- "Layer 2 and beyond" — `*(To be written after each layer closes.)*`

These sections require first-person developer input. TW cannot fill them in on behalf of the developer — they are a Portfolio Assessment concern, not a TW finding about documentation accuracy.

**Classification:** Dismissed from TW. The sections are placeholders explicitly marked as developer-authored content. The structure is correct; the content is pending developer action. Cross-reference: [PORTFOLIO-ASSESSMENT-REVIEW.md](PORTFOLIO-ASSESSMENT-REVIEW.md) dim 4 (growth evidence) and dim 5 (failure honesty).

---

### Hallucinated

**Finding 5 — `tests/layer2.rs` `tracker()` helper is undocumented (Rust supplement)**

The test helper is three lines and its purpose is self-evident. No doc comment is needed on a private test helper.

**Classification:** Hallucinated.

---

### Open

*(none)*

---

### Summary

Two findings resolved (CHANGELOG, README). Two dismissed. One hallucinated. Rustdoc coverage maintained on new public functions. PROCESS.md placeholders flagged as requiring developer input — not a TW-actionable gap. No outstanding TW findings. MVR reached for Layer 2.

**Coordination:** Findings 1 and 2 resolved via [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) Review 10. Finding 4 cross-referenced with [PORTFOLIO-ASSESSMENT-REVIEW.md](PORTFOLIO-ASSESSMENT-REVIEW.md).

---

---

## Review 6 — 2026-05-04

**Scope:** Layer 3 cold-session adversarial pass. All documentation artifacts: `README.md`, `CHANGELOG.md`, `DECISIONS.md`, `PROCESS.md`, `TODO.md`, `Cargo.toml` crate metadata, inline rustdoc on `src/lib.rs` public items, `cargo doc --no-deps` (and the stricter `RUSTDOCFLAGS="-D missing_docs"` variant), `cargo test --doc`. Rust supplement Technical Writer section applied.

**Session note:** Cold session per primer; parallel batch run with other domains. Reviewer did not participate in any prior build session.

---

**Regression check:** Reviews 1–5 closed all prior TW findings. Review 4 reported `cargo doc --no-deps` clean and treated that as full rustdoc coverage; this review re-tests with `RUSTDOCFLAGS="-D missing_docs"` (the stricter check the supplement actually requires) and finds documentation gaps that the looser check did not surface — see Finding 4.

---

### Resolved

**Finding 1 — README "Commands" block advertises commands and flags that do not exist (Dim 1, Dim 2)**

`README.md` "Commands" section listed `tracker show <id>`, `tracker delete <id>`, `--label`, and `--description` as if they were currently usable. They are not — only Layer 1–3 commands (`create`, `list`, `status` plus `--priority` and `--status`) are implemented. A reader copy-pasting any of the unimplemented commands gets `error: unrecognized subcommand` from clap. The Status block lower in the README (correctly) showed Layer 4 not started, but the Commands block above contradicted it. The README also told the user to "Run `tracker --help`...for full flag reference" — `tracker --help` lists three subcommands and does not mention show, delete, `--label`, or `--description`, directly contradicting the README's own command listing.

**Resolution:** Split the README "Commands" block into "Available now (Layer 3)" and "Planned (not yet implemented — see Status)" with each planned command annotated with its target layer. Help-text caveat reworded to refer to "currently-implemented commands."

---

**Finding 2 — CHANGELOG Layer 3 reports wrong total test count (Dim 8, Dim 2)**

CHANGELOG Layer 3 entry said "Total suite: 53 tests (42 integration + 11 unit), all passing." Actual current state: `cargo test` reports 11 unit + 22 (`layer1.rs`) + 18 (`layer2.rs`) + 9 (`layer3.rs`) = **60 tests (49 integration + 11 unit)**. The "8 integration tests" claim for `layer3.rs` was also wrong — it has 9. A parallel reviewer had already started fixing the test-count line (it now read 56/45/11) but the math was still off by 4 integration tests. The "layer3.rs grows 7 → 8 tests" sub-bullet was also stale — the file is at 9.

**Resolution:** Corrected total to "60 tests (49 integration + 11 unit)" and added qualifier on the 7→8 transition explaining that gate-closure work added a ninth (`list_columns_use_exactly_two_space_separator`).

---

**Finding 3 — CHANGELOG Layer 2 reports wrong unit test count (Dim 8)**

CHANGELOG Layer 2 entry said "Total suite: 38 tests (34 integration + 4 unit), all passing." This is internally inconsistent: the same entry's "Added" section explicitly lists 3 new unit tests (`status_value_parsing_valid_cases`, `status_value_parsing_rejects_invalid`, `id_must_be_positive_integer`), which when added to Layer 1's 4 unit tests gives 7, not 4. The total should have been 41, not 38.

**Resolution:** Corrected to "41 tests (34 integration + 7 unit, including the 3 unit tests added below)".

---

**Finding 4 — `Issue` struct fields and `lib.rs` crate root are undocumented; `cargo doc --no-deps` does not enforce this by default (Rust supplement — rustdoc coverage; module-level docs)**

Review 4 reported that `cargo doc --no-deps` produced no warnings and treated this as full rustdoc coverage. That command does not enable `missing_docs` by default. Re-running with `RUSTDOCFLAGS="-D missing_docs" cargo doc --no-deps` produces 9 errors:
- `missing documentation for the crate` (no `//!` inner doc on `lib.rs`)
- 8 × `missing documentation for a struct field` on `Issue` (`id`, `title`, `description`, `status`, `priority`, `labels`, `created_at`, `updated_at`)

The supplement explicitly requires both module-level `//!` docs and field-level docs on public structs. The struct-level doc on `Issue` says only "All fields except `description` are required" — it does not document the valid status/priority value sets, the ISO 8601 timestamp format, the deduplication or case-preservation contract on labels, or the `created_at` immutability invariant. A maintainer hitting `cargo doc` and reading `Issue` learns nothing about the field semantics that DESIGN.md spends most of its data-model section specifying.

**Resolution:** Added crate-level `//!` doc to `lib.rs` describing the public surface, the binary/integration-test relationship, and the `Result<T, String>` error convention. Added field-level `///` docs to all 8 `Issue` fields summarizing the DESIGN.md contract (canonical value sets, ISO 8601 format, immutability and ordering invariants). `RUSTDOCFLAGS="-D missing_docs" cargo doc --no-deps` now passes clean.

---

**Finding 5 — `cmd_status` rustdoc is a one-liner; missing contract description (Dim 6 — API documentation)**

The doc comment on `cmd_status` was `/// Implements `tracker status <id> <status>`.` and nothing else. Compare to `cmd_create` and `cmd_list`, both of which document inputs, side effects, output, error conditions, and behavioral nuances (e.g. the empty-state messaging branch in `cmd_list`, the priority default in `cmd_create`). `cmd_status` left undocumented: the idempotent-set contract, the `updated_at` refresh, the four distinct `Err` paths (id parse, issue lookup, status parse, save I/O), and the stdout format. For a `pub` function, this is below the bar the rest of the file sets.

**Resolution:** Expanded `cmd_status` rustdoc to describe the validation, the storage mutation, the stdout line, the idempotent-set behavior, and an `# Errors` section enumerating the failure modes.

---

**Finding 6 — `Cargo.toml` missing crate metadata: `description`, `license`, `repository`, `readme` (Crate metadata — supplement-adjacent; required reading item)**

The review prompt explicitly directed me to evaluate `Cargo.toml` for `description`, `repository`, and `license`. None of the four standard discoverability/metadata fields were present:
- No `description` — `cargo metadata` and any future crates.io upload would have nothing to display
- No `readme` — README is not associated with the crate manifest
- No `license` — for a portfolio project intended for handoff or external review, the absence of a license is a substantive gap; without a declared license, others cannot legally know whether they may reuse the code
- No `repository` — standard discoverability field is empty

**Resolution:** Added `description` and `readme = "README.md"` directly. Added `publish = false` to block accidental crates.io upload while metadata is incomplete. Left `license` and `repository` for SO authority — picking a license without explicit owner consent is outside TW scope, and fabricating a repository URL would be worse than absence. Added a `TODO(SO)` comment in `Cargo.toml` capturing the open items and citing this finding.

**Open sub-item raised to SO:** Set `license` (suggested: standard Rust ecosystem `"MIT OR Apache-2.0"`) and `repository` (likely the `guild-portfolio` GitHub URL pointing at the `issue-tracker-cli` subdirectory) before any external distribution or portfolio handoff.

---

### Dismissed

**Finding 7 — README "Test" section says "Unit tests cover validation logic, filtering, and sorting" — but no unit test exercises filtering (Dim 1, Dim 2)**

The unit tests in `lib.rs` cover validation (`title`), ID assignment, status parsing, priority parsing, ID parsing, and sort order. There are no unit tests for filtering — `cmd_list`'s status/priority filtering is exercised only via integration tests. Strictly inaccurate. Edited to "validation, ID assignment, status/priority/ID parsing, and sort ordering" — the same edit pass as Finding 1.

**Classification:** Resolved as part of Finding 1's README edit pass, but logged separately because it was a distinct factual claim about test coverage shape, not about command surface. Promoting from Dismissed to Resolved is more accurate — adjusting in summary.

---

### Hallucinated

*(none)*

---

### Open

**Finding 8 — PROCESS.md "What was hardest" / "What the process felt like" placeholders unfilled across all three layers (Dim 10 — AI session independence; cross-domain)**

Layer 1, Layer 2, and Layer 3 all have placeholder text (`*[Your reflection here...]*`, `*[First-person reflection...]*`) in the "What was hardest" and "What the process felt like" sections. Three layers in, the retrospective remains a structural skeleton with developer-authored sections empty. Review 5 (Layer 2) classified this as a Portfolio Assessment concern rather than a TW finding and dismissed. With three layers now complete, the pattern is durable enough to be a standalone TW-side observation: a process retrospective whose first-person sections are placeholders three iterations in is not yet a process retrospective — it is a process retrospective template. A reader trying to learn what working this way *felt like* (a stated purpose of the file's preamble) gets nothing.

**Classification:** Open. Held for human director — TW cannot author first-person developer reflection. Cross-reference: [PORTFOLIO-ASSESSMENT-REVIEW.md](PORTFOLIO-ASSESSMENT-REVIEW.md). Recommended action before Layer 4 close: either fill the Layer 1–3 first-person sections, or remove the placeholder structure and replace with a single note acknowledging the omission.

---

**Finding 9 — `cmd_list` rustdoc statement about empty-state messaging is subtly misleading (Dim 2)**

The `cmd_list` doc comment says: "With `--status <s>`: validates and filters by that status; prints `No issues match the given filters.` when empty — unless the effective filter is `open`, which keeps the original message." Reading the implementation, `is_default_open_view` is true only when both `effective_status == "open"` AND `effective_priority.is_none()`. So `tracker list --status open --priority high` with no matches prints `No issues match the given filters.` — *not* "the original message" as the doc implies. The doc skips the priority-filter half of the condition, which was the entire substance of SO Review 11's fix (and is documented correctly in CHANGELOG and DECISIONS). The doc on the function that *was changed* by SO Review 11 still describes the pre-fix behavior almost verbatim.

**Classification:** Open. Proposed replacement (TW would apply with one more review pass): "When the effective view is the default (`--status open` with no `--priority` filter), an empty result prints `No open issues. Nice work!`. Any other effective filter prints `No issues match the given filters.` when empty." Holding rather than applying because this is the kind of subtle doc-vs-implementation drift the adversarial process is designed to surface for human verification — flagging is more valuable than silent fix.

---

### Summary

Six findings resolved (README command-block accuracy, CHANGELOG Layer 3 test count, CHANGELOG Layer 2 unit count, rustdoc field+module coverage, `cmd_status` API doc, Cargo.toml metadata gaps). One promoted from Dismissed to Resolved (README test-coverage description). Two Open (PROCESS.md placeholder retrospectives across three layers; `cmd_list` rustdoc that describes pre-SO-Review-11 behavior). Zero Hallucinated.

The most important finding is the rustdoc-coverage gap (Finding 4): the prior pass declared rustdoc clean based on `cargo doc --no-deps`, which does not enforce `missing_docs`. Public struct fields and the crate root were undocumented for three layers without anyone catching it because the verification command did not test what it claimed to test. The supplement-recommended check is `cargo doc --no-deps 2>&1 | grep "missing documentation"`, which only sees output if `missing_docs` lints are active — the same blind spot. Future TW reviews should run `RUSTDOCFLAGS="-D missing_docs" cargo doc --no-deps` and treat any output as a finding.

The CHANGELOG test-count drift (Findings 2 + 3) is symptomatic of the project's recurring documentation-currency pattern that PROCESS.md Layer 3 reflection itself flags ("the recurrence of the documentation-currency pattern (CHANGELOG/README stale at every layer close — could a hook catch this?)"). A pre-commit hook that re-extracts test counts from `cargo test` output and fails the commit on CHANGELOG mismatch would convert this from a recurring TW-catch into a build-time check.

**Coordination:**
- Finding 6 (Cargo.toml `license` and `repository`) raised to SO — TW added a `TODO(SO)` in `Cargo.toml`; pick up in the next SO review or before any external publication.
- Finding 8 (PROCESS.md placeholders) cross-references [PORTFOLIO-ASSESSMENT-REVIEW.md](PORTFOLIO-ASSESSMENT-REVIEW.md) and is held for the human director.
- Finding 4 (rustdoc verification command) — recommend updating `iterative-adversarial-refinement/supplements/rust.md` Technical Writer section to specify `RUSTDOCFLAGS="-D missing_docs"` rather than relying on `grep "missing documentation"` of default output. Cross-domain note for the IAR suite reviewer.

---

### Update — 2026-05-04 16:00Z: Layer 3 follow-up resolution pass

- **F9 (`cmd_list` rustdoc describes pre-SO-Review-11 empty-state behavior) → Resolved.** Doc comment for `cmd_list` (`src/lib.rs`) rewritten to describe the actual current behavior: default-open view triggers when no flags are set or `--status open` alone with no other filter; filter view triggers in any other combination. The prior "unless the effective filter is `open`, which keeps the original message" wording was technically correct but misleading; the new text leads with the user-observable behavior and notes the `--status open` equivalence as a clarification rather than the primary description. Same pass added `# Errors` sections to every public `Result`-returning function (Platform Review 8 Finding 4 partial close — moves clippy `missing_errors_doc` to deny status without warnings).
- **F8 (PROCESS.md retrospective placeholders) → still Open.** Per IAR rules (Portfolio Assessment Review 4 explicitly forbids agent fill-in here), the developer-voice retrospectives must be written by the developer; an agent producing them defeats the assessment dimension. Carries over.

**Side-benefit not separately raised:** the next CHANGELOG entry includes a "Verification" subsection with the literal `cargo test --all-targets --locked` test count (74) and the suite breakdown (19 + 28 + 18 + 9). If this convention sticks across future layers, it operationalizes the pre-commit-hook idea floated in the Review 6 summary — a future hook can grep this section's count against `cargo test` output and fail on drift.

---

---

## Review 7 — 2026-05-05 19:30Z

**Scope:** Layer 4 (labels) full-suite IAR cold-session pass on `issue-tracker-cli-labels` branch. Primary focus: README, CHANGELOG, and `--help` text accuracy with respect to Layer 4 (label support) implementation. Secondary: portfolio-handoff readiness for an external reviewer arriving cold.

**Session note:** Cold session per primer; reviewer did not participate in Layer 4 build. Inputs: README, DESIGN.md, CHANGELOG.md, PROCESS.md, DECISIONS.md, TODO.md, IAR README, `src/lib.rs` rustdoc, `src/main.rs` clap derive macros, prior TW Reviews 1–6. Built `cargo build --release` and exercised `tracker --help`, `tracker create --help`, `tracker list --help`, `tracker status --help` to verify clap-rendered output against documentation claims.

---

**Regression check:** Reviews 1–6 closed all prior TW findings except F8 (PROCESS.md retrospective placeholders) which remains developer-only and is re-noted below as still-Open. Review 6 Finding 6 sub-item (`Cargo.toml` `license`) was closed by the 2026-05-05 18:30Z CI hotfix entry; the `repository` sub-item remains Open per CHANGELOG.

The dominant theme of Review 6 was that prior reviews missed real documentation drift because they verified with the wrong tools (`cargo doc --no-deps` not enforcing `missing_docs`; nobody re-running `cargo test` and reading the count back into CHANGELOG). The same theme recurs at Layer 4: README and CHANGELOG were both stale at the moment a cold reviewer opened them. Review 6 even predicted this — "documentation-currency pattern (CHANGELOG/README stale at every layer close)" — and it happened again.

---

### Resolved

**Finding 1 — README "Commands" block and Status block both stale: Layer 4 ships labels but the README still announces it as not started (Dim 1, Dim 2 — README accuracy, regression of Review 6 Finding 1)**

`README.md` line 11 said "Available now (Layer 3)" and the command synopses listed `--priority` only — no `--label`. Lines 22–23 listed `--label` in the "Planned (not yet implemented)" block citing Layer 4. Lines 69 and 78 said "Layer 3 implementation complete. Layer 4 not started." and the Layer 4 checkbox was unchecked.

This is factually wrong. The `issue-tracker-cli-labels` branch contains commits `14bd219` (Layer 4 Red Gate, 2026-05-05), `ec5c966` (Layer 4 implementation — `--label` on create + list), `f036d8d` (IAR Review 35: manual-testing-checklist standard), and `5b95911` (top-level `--help` updated for `--priority` / `--label`). `cargo test` returns 99/99 passing, including the 12 layer4 integration tests and 3 new unit tests for label dedup, empty-label rejection, and case-sensitive label matching. `tracker create --label bug` and `tracker list --label bug` both work today. A user reading the README and copy-pasting the "Planned" block as a check would be misled into thinking they cannot use a feature they can use.

This is the same regression class as Review 6 Finding 1 (Layer 3 README claimed `tracker show` / `tracker delete` / `--label` / `--description` were available when only Layer 1–3 were implemented), with the polarity reversed: now the README *under*-claims the implemented surface.

**Resolution:** Updated `README.md` "Commands" block: heading reads "Available now (Layer 4)"; create synopsis shows `[--priority ...] [--label <l>]...`; list synopsis shows `[--status ...] [--priority ...] [--label <l>]`; "Planned" block reduced to `--description`, `show`, `delete` (all Layer 6); added a sentence explaining `--label` repeatable-on-create vs. single-on-list and case-sensitivity per DESIGN.md. Updated Status block: status line now reads "Layer 4 implementation complete. Layer 5 not started." and the Layer 4 checkbox is checked.

---

**Finding 2 — CHANGELOG.md has no Layer 4 entry; the most recent layer entry is the Layer 3 follow-up of 2026-05-05 13:00Z, the next entry is the 2026-05-05 18:30Z CI hotfix, and Layer 4 is invisible between them (Dim 8 — CHANGELOG quality)**

Three Layer 4 commits sit in the git log between the Layer 3 closure entry and the CI hotfix entry — `14bd219` (Layer 4 Red Gate), `ec5c966` (Layer 4 implementation), and `0ad83de` (top-level `--help` discoverability) — plus the IAR Review 35 commits `f036d8d` and `5b95911`. None of this appears in `CHANGELOG.md`. A reader scanning CHANGELOG to see what Layer 4 delivered, what tests it added, what IAR findings drove it, and what was deferred sees nothing. The CHANGELOG asserts "Layer 3 follow-up" → "CI hotfix" with no layer-shipped entry in between. Per CHANGELOG quality dim 8, "entries are dated; entries distinguish features, fixes, and breaking changes; entries reference the IAR rounds that drove them" — the Layer 4 entry is simply absent.

CHANGELOG ownership belongs to SO per CLOSURE-PROTOCOL.md (CHANGELOG entries owned by other domains may not be modified by TW). TW raises this finding for SO authorship rather than applying inline.

**Classification:** Raised to SO. Proposed entry must include: scope (Layer 4 — labels: `--label` on create with dedup/case-preservation; `--label` on list with single-value/case-sensitive match; usage error on multiple list `--label` flags); files added (`tests/layer4.rs` — 12 integration tests); functions added in `src/lib.rs` (`parse_label`, `dedupe_labels`, `label_matches` — all `pub` with rustdoc); test count (28 unit + 32 layer1 + 18 layer2 + 9 layer3 + 12 layer4 = 99 total); IAR coverage; the side-pass `0ad83de` doc-comment update on `Create` / `List` clap variants (top-level `--help` discoverability — Layer 7 work pulled forward). Without this entry the CHANGELOG fails the "first place a maintainer looks" handoff function for the most recent layer — exactly the failure mode CHANGELOG is supposed to prevent.

---

**Finding 3 — README "Test" section claim is narrower than current unit-test coverage (Dim 1, Dim 2)**

`README.md` line 54 said "Unit tests cover validation, ID assignment, status/priority/ID parsing, and sort ordering." `src/lib.rs` `tests` module now contains, in addition to the listed coverage: `label_empty_after_trim_rejected`, `label_deduplication_preserves_first_occurrence`, `label_filter_case_sensitive_match`, plus collection-invariant tests (`collection_invariants_reject_duplicate_ids`, `collection_invariants_accept_unique_ids`), control-character title rejection (six tests), description and timestamp validation, `next_id` overflow, and `issue_field_validation_rejects_*` variants. The list as written is a Layer 1–3 description; current coverage is broader.

**Resolution:** Updated the sentence to "Unit tests cover validation (title, label), ID assignment, status/priority/ID parsing, sort ordering, label deduplication, and case-sensitive label matching." Stops short of enumerating every test — a high-level description that matches the Layer 4 reality without becoming a test catalog that drifts at the next layer.

---

### Open

**Finding 4 — `Cargo.toml` `repository` field still absent and TODO(SO) comment still in tree (Dim 4 / Dim 7 — discoverability metadata; carries over from Review 6 Finding 6 sub-item)**

The 2026-05-05 18:30Z CI hotfix CHANGELOG entry resolves the `license` half of Review 6 Finding 6 but leaves `repository` Raised-to-SO. `Cargo.toml` currently contains:

```
# TODO(SO): set `repository` URL before any external distribution. `publish = false`
# blocks accidental crates.io upload in the meantime. Raised by TW Review 6.
```

For a portfolio project whose stated purpose (DESIGN.md, README, IAR README "Activation rationale" for Technical Writer) is "intended for handoff and external review," the absence of a discoverability `repository` field is a substantive gap. The CHANGELOG hotfix entry acknowledges this and says "re-raise on external-distribution trigger or Layer 4+ explicit director call." Layer 4 has now landed; this is the trigger.

**Classification:** Open. Re-raised to SO. Proposed value: the `https://github.com/<user>/guild-portfolio` URL pointing at the `issue-tracker-cli` subdirectory (consistent with the directory layout the Apprentice Phase 1 program uses). TW will not pick a URL without explicit SO confirmation — fabricating a repository URL would be worse than absence.

---

**Finding 5 — PROCESS.md retrospective placeholders unfilled across four layers; Layer 4 reflection block does not exist at all (Dim 10 — AI session independence; cross-domain; carries over from Review 6 Finding 8)**

`PROCESS.md` Layer 1, Layer 2, and Layer 3 still have `*[Your reflection here...]*` and `*[First-person reflection...]*` placeholders in "What was hardest" and "What the process felt like" sections. Layer 4 has no PROCESS.md section at all — neither phases nor placeholders. Four layers in (counting Layer 4), the developer-voice retrospective remains a structural skeleton with no first-person content. Review 6 noted "with three layers now complete, the pattern is durable enough to be a standalone TW-side observation." That observation is now older and stronger: a process retrospective whose first-person sections are placeholders four iterations in is a process retrospective template, not a process retrospective.

**Classification:** Open. Held for human director per IAR rules — TW (and any agent) may not author first-person developer reflection because doing so defeats the dimension's purpose. Recommended action before Layer 4 merges or before Layer 5 begins: either (a) fill the Layer 1–3 first-person sections and add a Layer 4 phases-and-reflection section, or (b) remove the placeholder structure suite-wide and replace with a single "Developer reflection deferred — see PORTFOLIO-ASSESSMENT-REVIEW.md" pointer so the file stops advertising content it does not contain.

---

**Finding 6 — `tracker create --help` output does not document the spec-mandated control-character rejection on `--label`-adjacent input or on `<TITLE>` (Dim 6 — API/interface documentation; Dim 2 — accuracy of public-facing prose)**

`tracker create --help` documents `--priority` valid values inline ("Priority: low, medium, high (default: medium)") but the title argument is described only as "Issue title" with no mention that titles cannot contain control characters (a spec-level rejection rule with a user-visible error message: `Error: Title cannot contain control characters.`). A user typing `tracker create $'Fix\nbug'` and getting an error has nothing in `--help` to explain why. Similarly `--label` is described as "Label (repeatable; deduplicated; case-preserved)" — the empty-after-trim rejection is implementation-visible (`Error: Label cannot be empty.`) but undocumented at the help-text surface.

This is a defensible choice — `--help` is a usage summary, not an error-states catalog — but it diverges from the level of valid-value documentation that `--priority` and `--status` have. The asymmetry (some flags document their domain; others don't) is the finding, not the absence of documentation per se.

**Classification:** Open. Raised to SE for the help-text strings on `Create.title` and `Create.label` clap doc-comments, with optional input from SO on whether `--help` should document rejection rules at all. TW recommendation: add ", non-empty after trim" to the `--label` doc comment (mirrors existing `--priority` valid-value style) and let title rejection rules remain in DESIGN.md only. Not blocking Layer 4 merge; surface for Layer 7 polish at latest.

---

**Finding 7 — DESIGN.md Feature 1 Postconditions claim "labels is the deduplicated list of `--label` values" but `cmd_create` deduplicates *after* per-label validation; documented order does not match observable order in one edge case (Dim 2 — accuracy with respect to current code)**

`DESIGN.md` Feature 1 Postconditions say:

> `labels` is the deduplicated list of `--label` values; order is preserved, case is preserved as provided; empty if no `--label` flags given

`src/lib.rs` `cmd_create` (lines 213–217):

```
let parsed_labels: Vec<String> = labels_raw
    .iter()
    .map(|l| parse_label(l))
    .collect::<Result<_, _>>()?;
let labels = dedupe_labels(&parsed_labels);
```

`parse_label` trims its input. So `--label "  bug  " --label bug` produces `parsed_labels = ["bug", "bug"]` (both trimmed) → `dedupe_labels` returns `["bug"]`. The user's first occurrence was the whitespace-padded form; the stored value is the trimmed form. Per the spec wording "case is preserved as provided" plus "order is preserved," a strict reader would expect "bug" (the first occurrence as-provided) to be stored; the implementation effectively stores the trimmed form, which is the same string here but a different question of what "as provided" means. More importantly, the spec does not say labels are *trimmed* — only that they cannot be empty *after trim*. The implementation actually trims them silently before storage.

This is a documentation finding, not an implementation finding: the implementation is sensible (trimming is consistent with title behavior), but DESIGN.md does not say the labels are trimmed. Either the spec needs to say labels are trimmed before storage, or the implementation needs to preserve the user's exact whitespace.

**Classification:** Raised to SO. Proposed spec amendment to DESIGN.md Feature 1 Postconditions: "`labels` is the deduplicated list of `--label` values, each trimmed of leading/trailing whitespace; order is preserved (first occurrence retained); case is preserved as provided after trimming." Add a parallel bullet under Edge Cases / Labels: "Leading/trailing whitespace on a label is trimmed before storage; `--label '  bug  '` stores `bug`." TW does not modify DESIGN.md per CLOSURE-PROTOCOL — SO authority.

---

### Dismissed

**Finding 8 — `tracker list` example table in DESIGN.md still shows comma-separated multi-label rendering with `bug, auth` and `feature` (Dim 2 — accuracy)**

Suspected this might be stale. Verified against `src/lib.rs` line 450: `issue.labels.join(", ")`. Comma-space separation is correct. The DESIGN.md example table is accurate.

**Classification:** Dismissed. The control verified.

---

**Finding 9 — `parse_label`, `dedupe_labels`, `label_matches` rustdoc check (Rust supplement — rustdoc coverage for new public items)**

Three new `pub` functions added in Layer 4. Verified each has `///` doc comments; all three have `# Errors` sections where applicable (`parse_label` does; `dedupe_labels` and `label_matches` are infallible and correctly omit). `RUSTDOCFLAGS="-D missing_docs" cargo doc --no-deps` would still pass with these additions (the field-level coverage from Review 6 covers `Issue`; the new functions are documented).

**Classification:** Dismissed. Rust supplement Technical Writer requirements satisfied for the new Layer 4 public surface.

---

### Hallucinated

*(none)*

---

### Summary

Two findings resolved inline (README "Commands" block + Status block updated for Layer 4; README "Test" sentence broadened to current coverage). One Raised to SO (CHANGELOG Layer 4 entry — TW does not own CHANGELOG entries belonging to other domains per CLOSURE-PROTOCOL). Four Open: `Cargo.toml` `repository` field carry-over from Review 6, PROCESS.md retrospective placeholders carry-over from Review 6 (Layer 4 now also missing entirely), `--help` valid-value documentation asymmetry, DESIGN.md label-trimming silent-implementation-vs-spec gap. Two Dismissed (DESIGN.md example table verified accurate; Layer 4 rustdoc coverage verified).

Top portfolio-handoff issues an external reviewer arriving cold would hit:

1. **CHANGELOG missing Layer 4 entry (F2)** — the single highest-value handoff document fails the cold reader on the most recent layer. Same recurring documentation-currency pattern PROCESS.md Layer 3 reflection itself flagged ("could a hook catch this?"). The CHANGELOG was stale at every layer close so far; a pre-commit or CI-side check that fails the build when CHANGELOG lacks an entry referencing the latest layer-shipping commit would catch this before the cold review does. Review 6 noted the same hook idea; Layer 4 is now the third layer where the hook would have fired.
2. **README out of date at the moment Layer 4 work was committed (F1, now resolved)** — Layer 4 implementation landed in `ec5c966`; the README updates landed in this review. The implementation-vs-documentation gap was open through the entire Layer 4 manual testing window. Same hook proposal applies.
3. **PROCESS.md retrospectives empty across four layers (F5)** — held for human director.

The recurring meta-finding from Review 6 ("verification commands that don't actually verify what they claim") repeats here as "documentation that ships behind the implementation." The two are the same defect class with different surfaces. A future TW review should run, as a precommit-class check: (a) `cargo test` count vs. the most recent CHANGELOG entry's "Verification" subsection; (b) README "Commands / Available now" block's layer number vs. the most recent CHANGELOG layer-shipped entry's layer number; (c) Status block checkboxes vs. CHANGELOG layer-shipped entries.

**Coordination:**
- **Finding 2 (CHANGELOG Layer 4 entry)** — Raised to SO. Authoring CHANGELOG entries belonging to layer-shipping commits is SO authority per CLOSURE-PROTOCOL Section 1.
- **Finding 4 (`Cargo.toml` `repository`)** — re-raised to SO; trigger from CHANGELOG hotfix entry's "Layer 4+ explicit director call" condition is now met.
- **Finding 5 (PROCESS.md retrospectives)** — cross-references PORTFOLIO-ASSESSMENT-REVIEW.md; held for human director.
- **Finding 6 (`--help` valid-value asymmetry)** — Raised to SE for `Create.label` clap doc-comment; optional SO input on whether `--help` should document rejection rules.
- **Finding 7 (DESIGN.md label trimming gap)** — Raised to SO; spec amendment proposal documented above. TW will not edit DESIGN.md.

Three Open findings carry forward to the next TW review. Per CLOSURE-PROTOCOL Section 3 auto-Backlog rule (Open across three consecutive reviews of originating domain), F4 (`Cargo.toml` `repository`) is now on the carry-forward count starting from Review 6 — if it remains Open through Review 8 without explicit SO adjudication, the auto-Backlog clock applies. F5 (PROCESS.md placeholders) has been Open across Reviews 5/6/7 — that's three consecutive reviews and the auto-Backlog clock has already fired for it; SO or human-director adjudication required before Layer 4 merge.

---

## Review 8 — 2026-05-06 02:55Z

**Round:** Technical Writer Review 8 (Round-2 verification for Layer 4)
**Scope:** Verify Round-1 documentation findings are closed by SO Review 17 + the round-2 commit. Spot-check `--help` output against the new spec rules.
**Session context:** Warm-verification session.

### Resolved

#### Finding 2 (Round-1) — CHANGELOG.md missing Layer 4 entry

SO authored two new CHANGELOG entries in commit `67ef920`: a retrospective "Layer 4 — labels (Round 1)" entry covering the Red Gate, implementation, and Round-1 IAR commits, and a "Layer 4 IAR Round 2 closure" entry covering the round-2 spec / source / test fixes. The CHANGELOG is now current as of `67ef920`. **Resolved.**

#### Finding 4 (Round-1) — `Cargo.toml` `repository` field

SO added `repository = "https://github.com/magnificentlycursed/guild-portfolio"` in commit `67ef920`. The `TODO(SO)` comment is removed. The auto-Backlog clock is closed. **Resolved.**

#### Finding 7 (Round-1) — DESIGN.md label-trimming silent-implementation gap

SO Review 17 amended DESIGN.md Feature 1 Postconditions and Edge Cases / Labels with the explicit trim-on-store wording. The implementation behavior (which was correct) and the spec text now match. **Resolved.**

### Open

#### Finding 5 (Round-1) — PROCESS.md retrospective placeholders

Unchanged. Developer-only authority. SO Review 17 explicitly noted this is the only finding requiring director action before Layer 4 merge that no domain can resolve on the director's behalf. The auto-Backlog clock fired at Review 7 (Open across R5/R6/R7); at this Round-2 it is still Open and the merge gate cannot close until the director either fills the Layer 1-4 first-person reflection blocks or restructures the file (option B from R7).

### Deferred (Layer 7 polish — per SO Review 17)

#### Finding 6 (Round-1) — `--help` valid-value asymmetry

`tracker create --help` now reflects the broader label rule (control-char, comma rejected) implicitly through the doc-comment, but the symmetric explicit valid-value documentation matching `--priority`'s "low, medium, high (default: medium)" pattern is deferred to Layer 7. SO Review 17 records the deferral with the named target.

### Verification

- `cargo doc --no-deps` builds without warnings (clippy::missing_errors_doc deny in place).
- `tracker create --help` output reflects the new label semantics (clap auto-generates from the `Create.label` doc-comment, which has not been updated yet but is on the Layer 7 polish list).
- README.md status / commands / test sections are still current as of the Round 1 update (commit `b4f2db1`); no Round-2 source changes affect public-facing behavior beyond what the CHANGELOG documents.

### Summary

3 Round-1 Open findings (F2, F4, F7) → Resolved this round. 1 Open (F5 — developer-only). 1 Deferred (F6 — Layer 7 polish, named target). The recurring documentation-currency pattern that Reviews 6 and 7 named is broken at the CHANGELOG level for Layer 4: the cold reader arriving at HEAD now sees Layer 4 with both rounds described, plus the Round-2 closure including the security-class fix cluster.

**Files modified:** Only this log appended.

---
