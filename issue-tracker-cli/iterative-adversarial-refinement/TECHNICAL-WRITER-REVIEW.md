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

## Review 9 — 2026-05-11 01:10Z

**Round:** Technical Writer Review 9 — Layer 6 cold-batch (description + show + delete).
**Scope:** All public-facing documentation against Layer 6 implementation (commits `4fb5e67` Red Gate + `c91676a` implementation). Inputs: `README.md`, `CHANGELOG.md`, `DESIGN.md`, `DECISIONS.md`, `TODO.md`, `PROCESS.md`, `src/main.rs` clap doc-comments, `src/lib.rs` rustdoc on the new `pub` surface (`validate_description`, `cmd_show`, `cmd_delete`) and the private `format_show_block`. Bash captures of `tracker --help`, `tracker show --help`, `tracker delete --help`, `tracker create --help`; end-to-end `tracker show` output against the two DESIGN.md example blocks; `cargo test --quiet` test count; `RUSTDOCFLAGS="-D missing_docs" cargo doc --no-deps`.

**Session note:** Cold session per primer; reviewer did not participate in Layer 6 build.

---

**Regression check:** Reviews 1–8 closed all prior TW findings except Review 7 Finding 5 (PROCESS.md retrospective placeholders) which remains developer-only Open. The recurring documentation-currency pattern named in Reviews 6/7/8 ("CHANGELOG/README stale at every layer close") recurs at Layer 6 — same shape as TW R7 F2.

---

### Resolved

*(none — TW did not modify any source-of-truth artifact this round.)*

### Open

**Finding 1 — CHANGELOG.md has no Layer 6 entry (Dim 2 — CHANGELOG quality; same shape as TW R7 F2)**

`CHANGELOG.md` head is `## Layer 5 — compound filtering — 2026-05-07 00:43Z`. Both Layer 6 layer-shipping commits — `4fb5e67` (Red Gate: description + show + delete tests + stubs) and `c91676a` (implementation) — sit in `git log` with no corresponding CHANGELOG entry. The only `Layer 6` token in CHANGELOG.md is a Layer 3 follow-up forward-compat reference (line 491). A cold maintainer arriving at HEAD reads CHANGELOG, sees Layer 5 as the most recent layer-shipped entry, and concludes Layer 6 has not landed — directly contradicting the implementation, the test suite (20 layer6 integration tests now pass), and the TODO.md acceptance-criteria checkmarks. Same recurring documentation-currency defect class TW Reviews 6/7/8 named. The Layer 5 entry's "Verification" subsection set the precedent of including a literal test count (136 at gate close); the absent Layer 6 entry must include the new count.

Verified count via `cargo test --quiet`: **159/159 pass** (48 unit + 32 layer1 + 18 layer2 + 9 layer3 + 25 layer4 + 7 layer5 + 20 layer6). Delta from Layer 5 close: +3 unit, +20 layer6 = +23 total.

**Classification:** Open. Raised to SO. CHANGELOG ownership belongs to SO per CLOSURE-PROTOCOL.md — TW does not author layer-shipping entries. Proposed entry must include: scope (Feature 1 `--description`, Feature 4 `show`, Feature 5 `delete`); files added (`tests/layer6.rs` — 20 integration tests; `src/lib.rs` — `validate_description`, `cmd_show`, `cmd_delete` `pub` + private `format_show_block`; `src/main.rs` — `Commands::Show`, `Commands::Delete`, `Create.description` flag); test count (159 — see breakdown above); IAR coverage (this round); explicit reference to D1 (`tracker delete <id>` confirmation waiver, codified in DESIGN.md "Approved Deviations" at Layer 4 R2, now realized in Layer 6 code).

---

**Finding 2 — `tracker show --help` and `tracker delete --help` doc-comments are below the parity bar set by `tracker create --help` and `tracker list --help` (Dim 4 — `--help` quality)**

Captured via Bash:

- `tracker show --help` body: `Show full details for an issue` / `Usage: tracker show <ID>` / `<ID>  Issue ID`.
- `tracker delete --help` body: `Delete an issue (no confirmation; deleted IDs are never reused)` / `Usage: tracker delete <ID>` / `<ID>  Issue ID`.

Compare to `tracker create --help`, which documents the valid `--priority` value set inline (`Priority: low, medium, high (default: medium)`), and to `tracker list --help`, which documents the case-sensitive single-value `--label` filter rule. The `Show` and `Delete` doc-comments in `src/main.rs` (lines 46–55) carry zero spec contract:

- `Show` doc-comment does not name the eight-field labelled key-value block (`ID`, `Title`, `Status`, `Priority`, `Labels`, `Description`, `Created`, `Updated`) — i.e. what the user sees when the command succeeds.
- `Delete` doc-comment does say "no confirmation; deleted IDs are never reused" — this is good — but does not state the post-condition message format (`Deleted issue #<id>.`) or the not-found error path.

Symmetry-wise: `Create` documents its valid-value sets; `List` documents its filter semantics; `Status` documents the valid status values via the `New status: open, in-progress, done` arg doc-comment. `Show` and `Delete` are the only subcommands whose `--help` body is a bare one-liner with no contract details. The asymmetry is the finding.

This overlaps with TW R7 F6 (`--help` valid-value asymmetry) which was deferred to Layer 7 polish in SO R17. The same Layer 7 deferral disposition applies here — the underlying defect class is identical (`--help` doc-comment depth varies by subcommand). Flagging in case Layer 7 polish is the moment the suite addresses it suite-wide rather than per-subcommand.

**Classification:** Open. Deferred to Layer 7 polish, mirroring the TW R7 F6 disposition. Raised to SE for the `Show` / `Delete` clap doc-comments. Not blocking Layer 6 merge — the help text is accurate as far as it goes, just thinner than the parity expectation.

---

**Finding 3 — README.md Status block and Commands block both stale: Layer 6 ships description + show + delete but the README still describes them as Planned (Dim 1 — README accuracy; same shape as TW R7 F1)**

`README.md` line 11 reads `Available now (Layer 4):`. Lines 19–25 list `--description`, `show`, `delete` under `Planned (not yet implemented — see Status):` with the `# Layer 6` annotation. Line 68 reads `**Layer 4 implementation complete. Layer 5 not started.**`. Line 78 has `[ ] Layer 5: Compound filtering` and line 79 has `[ ] Layer 6: Description, show, delete`, both unchecked.

Factually wrong on three counts as of HEAD:

1. Layer 5 (compound filtering) shipped at `bd15a9d` and its CHANGELOG entry is at the top of `CHANGELOG.md` (2026-05-07 00:43Z). The README never caught up to Layer 5 closure.
2. Layer 6 (description + show + delete) shipped at `c91676a` (per the commit log). `tracker create --description`, `tracker show <id>`, `tracker delete <id>` all work today, but the README presents them as Planned.
3. The README "Commands" synopses for `create` and `list` do not show `--description`; the synopsis section omits `tracker show` and `tracker delete` entirely.

Same regression class as TW R7 F1 (under-claiming the implemented surface). The two-layer README staleness (Layer 5 + Layer 6) compounds the cold-reader handoff failure: a user who clones the repo and reads README sees a tool that supports only labels, when in fact every documented command in DESIGN.md is now implemented.

**Classification:** Open. Raised to SO. README is SO authority per CLOSURE-PROTOCOL Section 1. Proposed shape: bump heading to "Available now (Layer 6)"; add `[--description "<desc>"]` to the create synopsis and add `tracker show <id>` and `tracker delete <id>` to the synopsis block; remove the Planned block entirely (Layer 7 is polish, not new commands); flip Layer 5 and Layer 6 status checkboxes; status line to "Layer 6 implementation complete. Layer 7 not started." Also: line 27's `--label` paragraph is fine but should grow a sibling sentence about description (verbatim storage, multi-line) and the non-truncating nature of `show` per DESIGN.md.

---

**Finding 4 — TODO.md Layer 6 manual-testing checklist all unchecked (Dim 8 — process artifact accuracy; not strictly TW-owned but visible to cold reader)**

`TODO.md` Layer 6 acceptance-criteria block (lines 283–301) is fully `[x]`. The manual-testing checklist immediately below it (lines 303–316) is fully `[ ]`. Prior layers' pattern: manual-testing checklist flips to `[x]` in its own commit after director-run smoke tests (see Layer 5: commit `da0fd8d`). For Layer 6 the manual-testing window has not yet executed, or the checkbox flip has not yet been committed. This is not a TW-actionable defect (manual testing is a director gate, not a doc-currency claim), but it is visible to the cold reader as a Layer-6-not-fully-closed signal — flagging for situational awareness rather than as a finding to fix in this round. Same disposition as the Open carry-over for TW R7 F5 (PROCESS.md retrospectives) — held for human director.

**Classification:** Open. Informational. Held for human director per the Layer 5 precedent (manual testing commits its own checklist flip).

---

### Dismissed

**Finding 5 — DESIGN.md Show output examples diverge from binary output (Dim 7 — examples consistency)**

Suspected the two example blocks in DESIGN.md "Show output format" (lines 247–256 single-line and 261–270 multi-line) might be stale. Verified by reproducing both via Bash:

- Single-line: `tracker create "Update README" --priority low` → `tracker show 1` produces the exact 8-line block DESIGN.md shows (modulo the specific `Created`/`Updated` timestamps), including the 13-char label-column padding and the `(none)` rendering for absent labels + description.
- Multi-line: `tracker create "Fix auth flow" --priority high --label bug --description $'Token refresh fails after 1 hour.\nReproduces reliably on Safari.'` → `tracker show 2` produces the exact 9-line block including the 13-space continuation-line indent on `Reproduces reliably on Safari.`.

The two example blocks match the implementation byte-for-byte (excluding the timestamps, which are wall-clock).

**Classification:** Dismissed. DESIGN.md "Show output format" examples are accurate.

---

**Finding 6 — `validate_description` rustdoc does not note that the stored value is un-trimmed (Dim 5 — rustdoc fidelity)**

Suspected the rustdoc on `validate_description` (lines 326–334 of `src/lib.rs`) might fail to surface the spec-mandated verbatim-storage rule — a subtle defect because the trim/store distinction is the one thing a careless reader would miss. Re-read:

> Per DESIGN.md Feature 1: `--description` must be non-empty after trim, but the *stored* value is the input verbatim (not trimmed). This function returns the un-trimmed input on success so the caller can write it as-is.

The rustdoc names the trim-vs-store distinction explicitly and cites DESIGN.md. The reading-order prompt asked specifically whether the un-trimmed return is documented; it is.

**Classification:** Dismissed. Verified accurate.

---

**Finding 7 — DECISIONS.md does not have a Layer 6 entry; D1 deviation lives only in DESIGN.md (Dim 6)**

DECISIONS.md does record the non-interactive-delete decision in two places: the original "Interface and CLI" / "Non-interactive delete" entry (line 37, citing SO R6 F1) AND DESIGN.md's "Approved Deviations from Assignment" section D1 added at Layer 4 R2. DECISIONS.md does not have a Layer-6-specific section, but Layer 6 did not introduce new decisions distinct from D1 — `cmd_delete` simply realizes D1 in code without confirmation-prompt logic. The decision record is discoverable by a reader who looks under "Interface and CLI" → "Non-interactive delete" or under DESIGN.md "Approved Deviations". A Layer-6 retrospective grouping would be a nicety, not a defect.

**Classification:** Dismissed. The D1 decision is documented in both DESIGN.md (the "Approved Deviations" canonical record) and DECISIONS.md (the "Non-interactive delete" entry). No Layer-6-specific addition is required.

---

**Finding 8 — `cmd_show` / `cmd_delete` rustdoc thin compared to `cmd_status` (Rust supplement — rustdoc coverage)**

Suspected the new `pub` functions might have skimpy rustdoc. Re-read (lines 389–433 of `src/lib.rs`): both have `///` doc comments, both have `# Errors` sections enumerating the failure modes, both cite DESIGN.md, and `cmd_delete` explicitly states the ID-reuse invariant ("Deleted IDs are never reused: the next `create` assigns `max(remaining_ids) + 1`, which is strictly greater than any deleted ID. Other issues are not affected.") which is the substance of DESIGN.md Feature 5 invariants. `RUSTDOCFLAGS="-D missing_docs" cargo doc --no-deps` is clean. Rustdoc parity with `cmd_status` (post-TW R6 F5 expansion) is met.

**Classification:** Dismissed. Rustdoc on Layer 6 public surface is accurate and at parity with the rest of the file.

---

### Hallucinated

*(none)*

---

### Summary

4 Open findings: (1) CHANGELOG missing Layer 6 entry — high-impact recurring documentation-currency defect, same shape as TW R7 F2, raised to SO; (2) `Show` / `Delete` `--help` doc-comment depth asymmetry, deferred to Layer 7 polish mirroring TW R7 F6; (3) README Status + Commands blocks stale across both Layer 5 and Layer 6 closure, raised to SO; (4) TODO.md Layer 6 manual-testing checklist all unchecked, informational, held for human director. 4 Dismissed: DESIGN.md show-output examples verified byte-for-byte against the binary; `validate_description` rustdoc verified to name the trim-vs-store distinction; D1 decision is documented twice (DESIGN.md + DECISIONS.md "Non-interactive delete"); new `pub` rustdoc parity with `cmd_status` confirmed. 0 Hallucinated.

**Doc-currency assessment:** The two highest-impact handoff documents (CHANGELOG, README) are stale at the moment Layer 6 lands — repeating the Layer 4 pattern that was the dominant theme of TW Reviews 7/8. CHANGELOG misses Layer 5 nothing (already entered) but is missing the entire Layer 6 entry; README is stale at both Layer 5 closure (status checkbox never flipped) and Layer 6 implementation (commands still listed as Planned). The pre-commit hook idea floated in TW R6 ("a future hook can grep this section's count against `cargo test` output and fail on drift") would have fired here. Rustdoc, DESIGN.md show-output examples, and the `validate_description` un-trimmed-storage contract are all accurate.

**Top concern:** Finding 1 (CHANGELOG no Layer 6 entry) — the cold reader's primary handoff document fails on the most recently shipped layer for the third time in a row across Reviews 7/8/9. Finding 3 (README stale across two layers) is functionally part of the same defect class. SO authority required for both before Layer 6 merge.

**Coordination:**
- F1 (CHANGELOG Layer 6 entry) → Raised to SO. Proposed shape documented above.
- F2 (`--help` doc-comment asymmetry) → Deferred to Layer 7 polish per TW R7 F6 precedent. Raised to SE for the `src/main.rs` `Show` / `Delete` clap doc-comments.
- F3 (README staleness across Layer 5 + Layer 6) → Raised to SO. Proposed shape documented above.
- F4 (TODO.md manual-testing checklist) → Informational; held for human director per the Layer 5 precedent that manual-test commits its own checklist flip.

**Files modified:** Only this log appended.

---

## Review 10 — 2026-05-11 02:00Z

**Round:** Technical Writer Review 10 (Round-2 closure for Layer 6)
**Scope:** Verify Round-1 Open findings (CHANGELOG, --help depth, README, manual checklist) are resolved by commit `9b775f0`. Warm closure-verification.

### Round-1 finding closures

- **F1 (CHANGELOG missing Layer 6 entry):** **Resolved by commit `9b775f0`.** New Layer 6 retrospective + Round-2 closure entry added at the head of `CHANGELOG.md`. Entry follows the Layer 4 R2 format: Scope / Changed / IAR / Deferred / Open / Verification. Test count documented as 180/180 at Round 2 close. The cold-reader handoff document is current.
- **F2 (`Show` / `Delete` `--help` doc-comment depth asymmetry — cross-cut with UX R8 F1):** **Resolved by commit `9b775f0`.** `Show` and `Delete` doc-comments expanded in `src/main.rs` to enumerate fields (Show) / reference D1 + never-reused-ID rule (Delete) / document `<id>` as positive integer >= 1. Verified via `cargo run --quiet -- show --help` and `cargo run --quiet -- delete --help` — now match the Layer 1-4 depth standard.
- **F3 (portfolio README stale across Layer 5 + Layer 6):** **Resolved by commit `9b775f0`.** `guild-portfolio/README.md` updated: Layer 5 → ✅ Complete (was 🟡 In review PR #17); Layer 6 → 🟡 In IAR Round 2 (was 🔲 Not started). Synopsis-block / Commands-block deferred — those are project-README concerns (issue-tracker-cli/README.md), which TW will re-check at Layer 7 polish per the established cadence.
- **F4 (TODO.md Layer 6 manual-testing checklist unchecked):** **Open / Pending Director.** Same disposition as Layer 4 R11 F2 / Layer 5 final closure: director executes the 13 items + commits per `b0a3789` / `da0fd8d` precedent. SO R21 + VDD-IAR R16 both track this as the merge gate.

### Carry-forward verification

- TW R7 F2 / R7 F4 / R7 F7 (Layer 4 doc closures): No regression at Layer 6.

### New findings

*(none this round.)*

### Summary

3/4 Round-1 TW findings Resolved by commit `9b775f0`. 1 Open / Pending Director (F4 manual checklist). Cold-reader handoff documents (CHANGELOG + portfolio README) are current.

**Coordination:** *(none — closure pass)*
---

## Review 11 — 2026-05-11 22:30Z

**Round:** Technical Writer Review 11 — Layer 7 Round 1 cold-batch (polish layer: `--help`, TTY color, error specificity).
**Scope:** All public-facing documentation against Layer 7 implementation (Phase 2a Red Gate `7b461aa`; Phase 2b implementation `a2b8062`; manual-checklist closure `603c689`). Inputs: `issue-tracker-cli/README.md`, `guild-portfolio/README.md`, `DESIGN.md`, `DECISIONS.md`, `TODO.md`, `CHANGELOG.md` head entry, `iterative-adversarial-refinement/CLOSURE-PROTOCOL.md` §1, `tests/layer7.rs` top-comment block, `src/lib.rs` Layer 7 helper section + `format_show_block` / `cmd_show` / `cmd_list` doc-comments + module-level `//!`, `src/main.rs` clap doc-comments, and TW Reviews 9–10 for carry-forward.

**Session note:** Cold session per primer; reviewer did not participate in Layer 7 build.

---

**Regression check (whole-suite doc state):** Reviews 1–10 closed all prior TW findings except R7 F5 (PROCESS.md retrospective placeholders, developer-only) and R9 F4 / R10 F4 (TODO.md manual-checklist gate, director-owned — `603c689` flips Layer 7's 7/7). The recurring "CHANGELOG missing layer entry" defect class (TW R7 F2 / R9 F1) **does not recur** at Layer 7: `CHANGELOG.md` head is the Layer 7 entry at `2026-05-11 22:00Z`, written by the implementation commit. This is the first layer since at least Layer 4 where the CHANGELOG is current at landing — a real break in the documentation-currency pattern. The other recurring defect class (issue-tracker-cli/README.md staleness at every layer close — TW R7 F1 / R9 F3) **does recur** and is now spanning four layers (5/6/7 still claim `Available now (Layer 4)`).

---

### Resolved

*(none this round — Round 1 is the surfacing pass; fixes land Round 2.)*

### Open

**Finding 1 — `issue-tracker-cli/README.md` "Available now (Layer 4)" block stale across four layers (Dim 1 — README completeness; recurring TW R7 F1 / R9 F3 defect class)**

README.md L11 still reads `Available now (Layer 4):`. L19–25 list `--description`, `show`, `delete` under `Planned (not yet implemented — see Status):` annotated `# Layer 6`. L66–80 Status block: L68 reads `**Layer 4 implementation complete. Layer 5 not started.**`; L78–80 list Layers 5, 6, 7 all unchecked. As of HEAD: Layer 5 shipped at `bd15a9d` (2026-05-07), Layer 6 shipped at `c91676a` (2026-05-11 02:00Z), Layer 7 shipped at `a2b8062` (2026-05-11 22:00Z). All three commands previously gated as "Planned (Layer 6)" — `--description`, `show`, `delete` — are implemented and tested (180+ tests pass, including the layer6 and layer7 integration suites).

Additionally, the README does **not** describe the new Layer 7 surface at all: no mention that priority/status values render in color when stdout is a TTY, no mention that color is suppressed when stdout is piped/redirected (the `IsTerminal` rule). A cold reader cloning the repo today reads a project that is described as Layer-4-complete with three years of unmentioned feature work.

TW R9 F3 raised this for Layer 5 + Layer 6 staleness and was *partially* resolved in R10: the **portfolio** README (guild-portfolio/README.md) was updated, but the **project** README (issue-tracker-cli/README.md) was explicitly deferred ("Synopsis-block / Commands-block deferred — those are project-README concerns... which TW will re-check at Layer 7 polish per the established cadence"). Layer 7 is the named target; this is the moment.

**Classification:** Open. TW direct-edit authority per CLOSURE-PROTOCOL §1 (`README.md: Technical Writer; any domain (for accuracy fixes) — Edit directly to correct stale claims`). To land in Round 2: bump heading to `Available now (Layer 7):`; add `[--description "<desc>"]` to the create synopsis; add `tracker show <id>` and `tracker delete <id>` synopsis lines; remove the entire `Planned (not yet implemented)` block; flip Status block to `**Layer 7 implementation complete.**`; tick all six layer checkboxes; add a short "Color output" paragraph that documents the TTY-only rule (and cross-references DESIGN.md "Interface / color output"). NO_COLOR env-var support is out-of-scope (not implemented; not in DESIGN.md) — explicitly do not document it unless UX/SO raise it.

---

**Finding 2 — `guild-portfolio/README.md` Layer 7 row mis-marked 🔲 Not started (Dim 2 — documentation accuracy)**

`guild-portfolio/README.md` L36 reads `| 7 | Polish (color, `--help`) | 🔲 Not started |`. As of HEAD: the Layer 7 Red Gate (`7b461aa`), implementation (`a2b8062`), and manual-checklist closure (`603c689`) are all committed. CHANGELOG.md has a Layer 7 entry. The current state matches the convention used at the Layer 6 row (`🟡 In IAR Round 2`) at the moment Layer 6 cold-batch was active — Layer 7 should read `🟡 In IAR Round 1` while this review and the parallel domain reviews are in flight.

TW R10 F3 cited the established cadence — portfolio README is updated by the layer-shipping commit when a layer enters IAR. Layer 7's shipping commit didn't update it. Same defect class as R9 F3 portfolio half (which was resolved), now regressing one round later.

**Classification:** Open. TW direct-edit authority. Fix in Round 2: flip Layer 7 to `🟡 In IAR Round 1`. (Layer 6 row currently reads `🟡 In IAR Round 2` — but Layer 6 shipped via merged PR #18 per the git log, so Layer 6 is arguably ✅ Complete; flag for SO to adjudicate the Layer 6 row update too.)

---

**Finding 3 — `src/lib.rs` module-level `//!` doc-comment names functions that no longer exist (Dim 2 — documentation accuracy; Dim 5 — inline comment quality; Rust supplement — rustdoc fidelity)**

`src/lib.rs` L11–13 reads:

> ...the command implementations (`cmd_create`, `cmd_list`, `cmd_status`), the parsing/validation helpers (`validate_title`, `parse_status`, `parse_priority`, `parse_id`), and the storage primitives (`load_issues`, `save_issues`).

Three accuracies fail:

1. `load_issues` and `save_issues` were renamed to `load_tracker` and `save_tracker` in the SO R22 Option A commit (`8ed7db3`). The names `load_issues`/`save_issues` do not exist in the current source (`grep -n` confirms zero matches). The module-level rustdoc — the very first thing `cargo doc` shows a caller browsing the crate — names a nonexistent function.
2. `cmd_show` and `cmd_delete` (Layer 6 — landed `c91676a`) and the new Layer 7 helpers (`priority_ansi`, `status_ansi`, `wrap_color`, `pad_after_color`, `ANSI_RESET`) are missing entirely from the module-level enumeration. A cold reader using rustdoc as the crate's table of contents finds an out-of-date map.
3. `validate_description`, `parse_label`, `Tracker`, `Issue`, `CreateArgs`, `bump_next_id` — all part of the public surface — are likewise missing.

The pattern is the same TW R7/R9 documentation-currency class but at the rustdoc layer rather than CHANGELOG/README. Notably this is a *Layer 6* regression that survived TW R9 (which dismissed F8 declaring "rustdoc parity met") — the module-level doc was not part of R9's per-function check. The Layer 7 commit gave a clean reason to refresh the enumeration and didn't.

**Classification:** Open. Raised to SE (src/lib.rs is SE authority per CLOSURE-PROTOCOL §1). Proposed text: rewrite the L8–19 module comment to enumerate the current public surface (`Tracker`, `Issue`, `CreateArgs`; `cmd_create`, `cmd_list`, `cmd_status`, `cmd_show`, `cmd_delete`; `validate_title`, `validate_description`, `parse_status`, `parse_priority`, `parse_id`, `parse_label`, `bump_next_id`, `current_timestamp`; `load_tracker`, `save_tracker`). Add a one-line Layer 7 note: "Color helpers (`priority_ansi`, `status_ansi`, `wrap_color`, `pad_after_color`) are private — see the Layer 7 block above `Tracker` for the TTY-detected color contract."

---

**Finding 4 — DECISIONS.md missing the "raw ANSI escapes, no `anstyle` dependency" decision (Dim 4 — decision rationale)**

`CHANGELOG.md` Layer 7 entry L9 captures the rationale (`Raw ANSI escapes (no anstyle / termcolor dependency) — the six sequences ... are universally supported by VT100-compatible terminals, the only environment this single-user portfolio CLI targets.`) and `src/lib.rs` L41–44 carries the same justification inline. But DECISIONS.md has no entry. A future developer asking "why did we hand-roll ANSI escapes instead of using `anstyle`?" must reverse-engineer the answer from the CHANGELOG or the source comment — neither is the canonical artifact for design-rationale lookup.

DECISIONS.md already includes the *prior* color decision (the "Color output included" entry at line 50, citing SO Review 3 Finding 1), but that entry is scoped to **whether** to color, not **how**. The how-decision — minimal-dependency raw ANSI vs. `anstyle`/`termcolor` — is a real choice with a real trade-off (no parsing safety net, no Windows-conpty compatibility, no graceful degradation on legacy terminals), and the rationale (single-user macOS portfolio CLI, VT100-compatible target) is exactly the kind of context DECISIONS.md is meant to preserve. The Layer 7 IAR brief flagged this anticipatorily: "DECISIONS.md: Layer 7 included a deliberate 'raw ANSI escapes, no anstyle dependency' decision. Was it recorded? If not, raise to SO Review 23 for a DECISIONS.md entry." It was not.

**Classification:** Open. Raised to SO (DECISIONS.md authority is SO primary per CLOSURE-PROTOCOL §1, with rationale-citing append allowed by any domain — but a SO-owned new entry is the cleaner closure path given this is a new section "Layer 7 — implementation decisions"). Proposed entry: title "Raw ANSI escapes, no `anstyle` / `termcolor` dependency"; cite the CHANGELOG Layer 7 entry and `src/lib.rs` L41–44; rationale: "VT100-compatible terminals universally support the six sequences used (`\x1b[1;31m`, `\x1b[33m`, `\x1b[36m`, `\x1b[32m`, `\x1b[0m`); the six-string surface is small enough that a parsing/portability dependency adds more risk (CVE surface, version churn) than it removes for a single-user macOS portfolio CLI; revisit if Windows or legacy-terminal support is added."

---

**Finding 5 — `CHANGELOG.md` Layer 7 entry "Open (process)" half-claims a contradicted state (Dim 8 — CHANGELOG quality)**

CHANGELOG.md L37 reads:

> **Layer 7 manual testing checklist** (TODO.md L368-374) — 7 unchecked items. Director must execute and commit per CLOSURE-PROTOCOL.md merge-gate criterion 3 (same standing process Open as Layers 4 / 6). Carry-forward for the IAR session.

This was written by commit `a2b8062`. The very next commit, `603c689` ("Layer 7 manual testing complete — 7/7 ticked"), closes the items. As of HEAD the manual-checklist is fully ticked — TODO.md L370–376 all show `[x]`. The CHANGELOG entry has not been updated to reflect that the Open-process item is now Closed. A cold reader reading the CHANGELOG head sees an Open process finding that no longer applies.

This is mild: a reader who reads `603c689`'s commit message resolves the contradiction quickly, and the manual-checklist gate is correctly closed at the TODO.md / commit-history level. But the CHANGELOG entry's own "Open (process)" subsection is now misleading.

**Classification:** Open. Any domain may edit CHANGELOG to record a change per CLOSURE-PROTOCOL §1. Fix in Round 2: amend the L37 bullet to reflect closure — e.g. "**Layer 7 manual testing checklist** — Closed by `603c689` (2026-05-11 22:30Z). All 7 items ticked per TODO.md L370–376." Or restructure to a Closed (process) sub-bullet to match the precedent of Layer 6 entries.

---

**Finding 6 — `tests/layer7.rs` top-comment is excellent for AI-session-independence (Dim 10) — verified, no action**

(Recording the verification, not a finding.) The 26-line top-comment block at L1–25 explains the unusual "tests pass against pre-Phase-2b code" framing in three named-failure-mode terms: (a) Layer 7 is polish; clap and the Layer 1 `try_parse` already satisfy most help / unknown-subcommand acceptance criteria *against current code*; (b) the tests pin the *contract* (valid-value enumerations, exit codes, stderr routing) that prior layers established only by convention — a future refactor would now fail named tests; (c) TTY-positive rendering is intentionally manual-only (subprocess `assert_cmd` produces a non-TTY stdout by construction). A cold reader (or a future AI session) reading the test file in isolation can reconstruct the Red Gate framing without git history or prior IAR logs. Cross-references TODO.md ("Manual only (TTY-detection cannot be automated in subprocess tests)") which is reachable from the comment.

This is the right level of context preservation for AI-session independence. The comment is durable, scoped to the file, and explains both *what* (the Red Gate contract test) and *why* (polish-layer redundancy with clap defaults) without inviting the reader to read elsewhere.

**Classification:** Verified — recorded for the durable record, not a finding.

---

### Dismissed

**Finding 7 — `format_show_block` doc-comment fails to document the `use_color` parameter (Dim 6 — API documentation)**

Suspected: the function signature gained a `use_color: bool` parameter in `a2b8062` (CHANGELOG L14 confirms). If the doc-comment was updated for Layer 6 but not Layer 7, the cold reader sees a stale `# Parameters`-or-equivalent description.

Read `src/lib.rs` L518–525:

> Renders a single issue as the `tracker show` labelled key-value block.
>
> Per DESIGN.md "Show output format": each label is right-padded to a fixed width of 13 characters so values align. For multi-line descriptions, the first line follows the `Description:` label; each continuation line is indented by 13 spaces (matching the label-column width).
>
> Returns the formatted block including a trailing newline.

The doc-comment does **not** explicitly document `use_color`. But on closer inspection: (a) `format_show_block` is a **private** function (`fn`, not `pub fn`); (b) the inline comment block at L545–547 immediately above the call to `wrap_color` documents the parameter's behavior in context (`Layer 7: color the status and priority values when use_color is true. The label column ... is uncolored — color applies to value text only per DESIGN.md "Interface / color output".`); (c) rustdoc convention does not require parameter-level docs for private functions, and the IAR Rust supplement Technical Writer dim 1 explicitly carves out private items from the public-surface coverage requirement ("For binary-only crates, exported functions in `lib.rs` must be documented; internal functions in `main.rs` may be omitted" — `format_show_block` is closer to the latter category as a `pub`-crate helper). The behavior is documented at the call site, not the function definition, but it is documented and accurate.

**Classification:** Dismissed. Function-level doc-comment is at parity with the rest of the file's private-function convention; the use_color contract is documented at the call site where it is non-obvious, not at the signature where it would duplicate the bool semantic.

---

**Finding 8 — `cmd_list` / `cmd_show` doc-comments fail to mention TTY-detection / color (Dim 2 — documentation accuracy)**

Suspected: the Layer 7 color rule should appear in the `pub fn cmd_list` and `pub fn cmd_show` rustdoc per the "describe the current implementation" rule.

Read `src/lib.rs` L752–770 (`cmd_list`) and L570–578 (`cmd_show`). Neither doc-comment mentions color or TTY. But (a) the color rule is documented in DESIGN.md "Interface / color output" (canonical spec); (b) it is documented in the Layer 7 helper block at L28–44 (where the implementation lives); (c) it is documented at the call sites (L832–835 cmd_list, L589–591 cmd_show) with explicit inline comments tying back to DESIGN.md; (d) coloring is a presentation-layer concern that does not change the function's *behavioral contract* (`Errors`, postconditions, valid arguments) — those are all unchanged by Layer 7. Adding color-rule prose to the `pub fn` rustdoc would duplicate DESIGN.md without changing the caller-relevant contract.

**Classification:** Dismissed. The function rustdoc documents behavior the caller must understand; color rendering is implementation detail of the rendering path, covered in DESIGN.md and at the call sites.

---

**Finding 9 — clap `--help` text "valid values" coverage incomplete for Layer 7 (Dim 1 — README/help completeness; CLI supplement UX dim 1)**

Suspected: Layer 7's "all error messages reviewed for specificity" and "help is accurate" criteria might miss a value-enumeration in `src/main.rs` clap doc-comments. Re-read L13–55 against `tests/layer7.rs` content assertions:

- `Create.priority` (L20–22): doc-comment `Priority: low, medium, high (default: medium)` — matches `tests/layer7.rs::help_flag_create_exits_zero` assertion `"low, medium, high"`. ✓
- `List.status` (L29–31): doc-comment `Filter by status: open, in-progress, done` — matches `tests/layer7.rs::help_flag_list_exits_zero` assertion `"open, in-progress, done"`. ✓
- `List.priority` (L32–34): doc-comment `Filter by priority: low, medium, high` — matches. ✓
- `List.label` (L35–37): doc-comment `Filter by label (case-sensitive exact match; single value only)` — DESIGN.md-current; documents the multi-`--label` rejection rule.
- `Status.status` (L43–45): doc-comment `New status: open, in-progress, done` — matches `tests/layer7.rs::help_flag_status_exits_zero` assertion. ✓
- `Show` / `Delete` (L46–55): doc-comments expanded in `9b775f0` (Layer 6 R2) per TW R9 F2 closure — confirmed at current parity.

All help-text valid-value enumerations match the tests and DESIGN.md. The `Create.description` doc-comment `Free-form description (stored verbatim; not trimmed)` is the only one without an explicit "valid values" enumeration, but this is correct — the description is free-form (no enum), and the doc-comment names the surprising property (verbatim, not trimmed). Layer 7 help is current.

**Classification:** Dismissed. `tracker [subcommand] --help` text is accurate and at parity across all six subcommands.

---

### Hallucinated

*(none)*

---

### Deferred

*(none — Round 1 surfaces; Round 2 dispositions.)*

---

### Summary

**6 findings (5 actionable Open + 1 Verified-recorded):**
- F1 — issue-tracker-cli/README.md "Available now (Layer 4)" stale across four layers (TW direct-edit) — *recurring TW R7 F1 / R9 F3 defect, now four layers deep and the named "Layer 7 polish moment" promised in R10 F3*
- F2 — guild-portfolio/README.md Layer 7 row mis-marked 🔲 Not started (TW direct-edit)
- F3 — src/lib.rs module-level `//!` doc names nonexistent `load_issues` / `save_issues`; misses entire Layer 6+7 public surface (Raised to SE)
- F4 — DECISIONS.md missing the "raw ANSI escapes, no anstyle" rationale (Raised to SO; flagged in Layer 7 IAR brief)
- F5 — CHANGELOG Layer 7 entry's "Open (process)" half-claims a contradicted state — manual checklist closed by `603c689` but the entry was not updated (Any-domain edit)
- F6 — `tests/layer7.rs` top-comment AI-session-independence: **Verified excellent**, no action

**3 Dismissed (with specific verification):**
- F7 — `format_show_block` use_color doc — private function, documented at call site, follows file convention
- F8 — `cmd_list` / `cmd_show` color rule — implementation detail, covered in DESIGN.md and inline at call sites; behavioral contract unchanged
- F9 — clap `--help` valid-value enumerations — verified against tests and DESIGN.md across all six subcommands

**0 Hallucinated.**

**Doc-currency assessment:** Layer 7 is the first layer since at least Layer 4 where CHANGELOG.md is current at landing (TW R7 F2 / R9 F1 pattern broken — credit where due). The remaining doc-currency defects cluster around the *project README* (four-layer-stale) and the *module-level rustdoc* (one-layer-stale after the SO R22 rename); these are the same defect class as before, just in different artifacts. F4 (DECISIONS.md ANSI rationale) is a fresh decision-rationale gap, not a recurrence — Layer 7 introduced the first new architectural choice since Layer 4 (label rules) that warrants a DECISIONS.md entry, and the Layer 7 CHANGELOG entry captured the rationale but the canonical decisions log did not.

**Top concern:** Finding 1 (project README stale four layers deep). The cold reader's primary handoff document fails at Layer 7 in the exact same shape it failed at Layers 5 / 6 — TW R10's "deferred to Layer 7 polish per the established cadence" disposition makes this the named moment. Sycophancy check: I considered marking this a soft "informational" finding given the portfolio README is current, but the project README is the artifact a cold cloner reads first (per Dim 1's "clone the repo into a fresh environment and follow the README" test), and four layers of staleness in the synopsis block actively misleads.

**Coordination:**
- F1 (project README) → TW direct-edit in Round 2.
- F2 (portfolio README) → TW direct-edit in Round 2.
- F3 (src/lib.rs module doc) → **Raised to SE** for the `//!` rewrite. Coordinates with SE Review (Layer 7 cold-batch).
- F4 (DECISIONS.md raw-ANSI entry) → **Raised to SO** for the new Layer 7 decisions section. Coordinates with SO Review 23.
- F5 (CHANGELOG Open-process line) → Any-domain edit; bundle with TW direct edits in Round 2.
- F6 (tests/layer7.rs verified) → No coordination; recorded for the durable record.
- Cross-domain: F3 overlaps with SE Review (stale inline doc); F4 overlaps with SO Review 23 (DECISIONS.md authority) and SA Review (architectural-choice rationale not documented).

**Files modified:** Only this log appended.

---

## Review 12 — 2026-05-12 00:00Z

**Round:** TW Review 12 (Layer 7 IAR Round 2 closure pass). Warm verification per CLOSURE-PROTOCOL.md §5; not a new adversarial round.

**Scope:** Verify R11 Open findings closed by commit `09b1905`. Inputs: rewritten `issue-tracker-cli/README.md`; updated `guild-portfolio/README.md` Layer 7 row; refreshed `src/lib.rs` `//!` module-level doc-comment; new DECISIONS.md entries; refreshed CHANGELOG.md Round-2 entry.

### Round-1 finding closures

- **F1 — `issue-tracker-cli/README.md` "Available now (Layer 4)" block stale across four layers:** **Resolved by `09b1905`.** Full rewrite: all 5 subcommands documented with current flags including `--description`; new "Color output" section explaining TTY rule + NO_COLOR / CLICOLOR honoring + WCAG bold-redundancy rationale + CLICOLOR_FORCE exclusion; install section bumped to `rust-version = "1.82"` reference; Status block now reads "Layer 7 implementation complete; Layer 7 IAR Round 2 closure in progress" with all 7 layer checkboxes; IAR domain count corrected to 11 active. The recurring "README stale" pattern (TW R7 F1 / R9 F3 lineage) is closed at the Layer 7 timing the prior reviews explicitly committed to.
- **F2 — `guild-portfolio/README.md` Layer 7 row mis-marked 🔲 Not started:** **Resolved by `09b1905`.** Layer 6 row corrected to ✅ Complete; Layer 7 row corrected to 🟡 In IAR Round 2. The portfolio-level reader now sees accurate current-state.
- **F3 — `src/lib.rs` module-level `//!` doc-comment names functions that no longer exist:** **Resolved by `09b1905`.** Full refresh: `load_issues` / `save_issues` (renamed at Layer 6 R3 SO-R22 closure) corrected to `load_tracker` / `save_tracker`; missing surface added (`cmd_show`, `cmd_delete`, `validate_description`, `CreateArgs`, `Tracker`, `display_safe`); `cargo doc` now shows an accurate map for cold readers.
- **F4 — DECISIONS.md missing the "raw ANSI escapes, no `anstyle` dependency" decision:** **Resolved by `09b1905`.** New entry "Raw ANSI escapes rather than `anstyle` / `termcolor` dependency" under "Layer 7 IAR Round 2 spec amendments" captures the SE-domain rationale, the spec-scoped target environment (VT100-compatible terminals), and the re-evaluation triggers.
- **F5 — `CHANGELOG.md` Layer 7 entry "Open (process)" half-claims a contradicted state:** **Resolved by `09b1905`.** The prior Layer 7 implementation entry's "Open (process)" item was the manual-checklist closure, which landed in `603c689` before Round 1. The new Round-2 CHANGELOG entry has its own "Open (process)" section listing current carry-forward items: VDD-IAR R18 ratification, manual re-walk, deferred force_color seam, deferred clippy hook. The CHANGELOG narrative is now consistent with closure state at HEAD.

### Doc-currency verification (R2 sweep)

- **README.md (issue-tracker-cli):** every claim cross-checked against current code — Color output section against `priority_ansi` / `status_ansi` bold values; NO_COLOR / CLICOLOR section against `color_mode_from_env`; storage shape claim against `Tracker` struct.
- **README.md (guild-portfolio):** Layer 7 row consistent with Status block in issue-tracker-cli README.
- **CHANGELOG.md:** the three Round-2 commit message claims (substantive R2, R1 review log batch, retroactive Red Gate retrofit) reflected in the CHANGELOG entry.
- **DECISIONS.md:** six R2 entries, each with citation chain to originating R1 finding(s).
- **src/lib.rs `//!`:** public surface listing matches `pub fn` / `pub enum` / `pub struct` declarations at HEAD.

No claim-vs-code drift detected in the R2 doc sweep.

### New findings

*(none — closure pass.)*

### Summary

All 5 R1 TW findings Resolved. The recurring "doc-currency drift" pattern (TW R7 F1 / R9 F3 — README stale at end-of-layer) was the highest-priority recurrent finding across the project's TW history; its resolution at Layer 7 R2 closure breaks the pattern at the timing prior reviews committed to. Doc artifacts are at MVR for Layer 7.

**Coordination:** SO R24 — DECISIONS.md and CHANGELOG entries authored under SO authority verified for correctness; SE R18 — `//!` doc-comment refresh verified against `pub fn` surface.

**Files modified:** Only this log appended. The README.md / CHANGELOG.md / DECISIONS.md / `//!` doc-comment edits landed in `09b1905` under TW + SO + SE authority per CLOSURE-PROTOCOL.md §1.

---

## Review 13 — 2026-05-12 12:00Z

**Round:** Technical Writer Review 13 — Layer 7 IAR Round 3 cold-batch (R3 surfacing pass over the five R3 commits: clippy hook `ff0e85c`, CJK debug_assert `c341a54`, force-color test seam `bd7511e`, cmd_list rendering extraction + column constants `3fa1f3c`, three-module split `8db9437`).

**Scope:** Cold-session review of documentation against R3 changes. Inputs: `issue-tracker-cli/README.md`, parent `guild-portfolio/README.md`, `DESIGN.md`, `DECISIONS.md`, `CHANGELOG.md`, `PROCESS.md`, `TODO.md`, the four new module-level `//!` doc-comments (`src/lib.rs`, `src/storage.rs`, `src/validate.rs`, `src/commands.rs`), inline doc-comments on the new extracted helpers / constants / debug_assert sections, and TW R11/R12 carry-forward. `cargo doc --no-deps` run for rustdoc-fidelity check.

**Session note:** Cold session per primer; reviewer did not participate in any R3 commit.

---

**Regression check:** R11's five Open findings (project README staleness, portfolio README mismark, lib.rs `//!` drift, DECISIONS.md missing raw-ANSI entry, CHANGELOG contradicted-state) all confirmed still closed by R12 against HEAD — none has regressed. The recurring `CHANGELOG missing layer entry` defect class (TW R7 F2 / R9 F1 lineage), which R11 noted was broken at Layer 7 R1, now **recurs** at R3 (see F1 below). The recurring `README staleness across layers` class is *not* recurring at the project-README level (Status block is updated), though the project README's text content predates the module split and the parent portfolio README's Layer-7 row marker is also slightly stale (see F4).

---

### Resolved

*(none this round — Round 3 is the surfacing pass; fixes land in a follow-up if needed per the IAR brief.)*

### Open

**Finding 1 — `CHANGELOG.md` has no entry for the five Layer 7 IAR Round 3 commits (Dim 8 — CHANGELOG quality; recurring TW R7 F2 / R9 F1 defect class)**

`CHANGELOG.md` head is still the `Layer 7 IAR Round 2 closure — 2026-05-11 23:30Z` entry. Between that entry and HEAD, five substantive commits landed:

1. `ff0e85c` — `cargo clippy` pre-commit hook (Platform R12 F3 closure, previously *Deferred* in the R2 entry's PE bullet).
2. `c341a54` — `render_cell` ASCII `debug_assert` (QE R17 F5 closure, previously *Deferred* in the R2 entry's QE bullet).
3. `bd7511e` — `TRACKER_INTERNAL_FORCE_COLOR` test seam (QE R17 F1 closure, previously *Deferred* in the R2 entry's QE bullet).
4. `3fa1f3c` — `cmd_list` rendering extraction + column-width constants (SA R11 F1 + SA R13 F2 closure, both previously *Backlogged* per CLOSURE-PROTOCOL §3 in the R2 closure entry).
5. `8db9437` — three-module split of `src/lib.rs` into `storage.rs` / `validate.rs` / `commands.rs` (SA R13 F1 Trigger B closure, previously *Backlogged*).

Each of these is a substantive change. Three close findings the R2 CHANGELOG entry explicitly listed as `Open / Deferred`; two close findings auto-Backlogged under CLOSURE-PROTOCOL §3. A cold reader reading `CHANGELOG.md` at HEAD sees:

- The R2 closure's "Open (process)" section still listing "VDD-IAR Round 2 (next round)" and "Layer 7 manual testing checklist" as forward-looking items, with no acknowledgement that R3 has happened and that three of the named *Deferred* items above have shipped.
- No "Layer 7 IAR Round 3" or equivalent entry recording the module split, the column-width constants, the `cmd_list` extraction, the new test seam, the new clippy hook, or the new `debug_assert!` in `render_cell`.

This is the same documentation-currency defect class TW R7 F2 / R9 F1 raised and R11 noted was broken at Layer 7 R1 ("first layer since at least Layer 4 where the CHANGELOG is current at landing"). The pattern resumes at R3: five commits, zero CHANGELOG entries. Per CLOSURE-PROTOCOL.md §1, the CHANGELOG is editable by any domain.

**Classification:** Open. Raised to SO (CHANGELOG curation is SO-primary). Proposed remedy: a single bundled `## Layer 7 IAR Round 3 — <date>` entry listing each of the five commits as a sub-bullet under Changed / Added / Tests as appropriate, with the originating IAR-finding lineage cited (`Platform R12 F3`, `QE R17 F5`, `QE R17 F1`, `SA R11 F1`, `SA R13 F2`, `SA R13 F1 Trigger B`), the SA carry-forward cluster closure called out (`Every Round-1 deferred finding has now landed terminal closure`, mirroring the commit message of `8db9437`), and the Verification block updated with the post-R3 test count (237/237 per `8db9437`'s commit message). The R2 entry's "Open (process)" subsection should also be amended to mark the three Deferred items as Resolved with their closing commit references.

---

**Finding 2 — `src/lib.rs` module-level `//!` enumerates `pub(crate)` items as part of the module's "exports" without distinguishing them from the public surface (Dim 6 — API documentation; Rust supplement — rustdoc fidelity)**

`src/lib.rs` L8–32 is the new hub `//!` written by `8db9437`. It enumerates the public surface per submodule. The wording is "data types (...), persistence (...), and load-time invariants (`tracker_is_valid`, `issue_fields_are_valid`)" for `storage`, and "rendering / color layer (`ColorMode`, `color_mode_from_env`, `format_show_block`, `format_list_row`, etc.)" for `commands`. Verified against the actual `pub use` re-exports at L42–50:

- `tracker_is_valid` and `issue_fields_are_valid` are **`pub(crate)`** in `src/storage.rs` (L154, L110) — not part of `pub use` and not in the public API surface. A `cargo doc --no-deps`-browsing caller will not find them.
- `format_show_block` and `format_list_row` are **`pub(crate)`** in `src/commands.rs` (L347, L554) — same status.

The `//!` does not flag these as crate-internal. A cold reader reading the hub map sees four function names presented as part of the module's documented surface, then opens the rustdoc HTML and finds two of them missing entirely. This is a milder version of the R11 F3 "names functions that no longer exist" failure — here the functions exist but are inaccessible from the public API surface the `//!` purports to map.

Additionally, three items that ARE in `pub use` are NOT named in the `//!`: `dedupe_labels` (validate), `label_matches` (commands), `sort_issues` (commands). The `etc.` in the commands bullet covers this informally but `dedupe_labels` falls into the validate bullet which has no `etc.`. A reader navigating `tracker::dedupe_labels` from main.rs / integration tests finds no module-map mention.

**Classification:** Open. Raised to SE (src/lib.rs is SE authority per CLOSURE-PROTOCOL §1). Proposed remedy: either (a) reframe the `//!` enumeration as "primary responsibility" rather than "exports", explicitly noting "(plus crate-internal helpers — see the module's `cargo doc` page)" and adding the three omitted public re-exports; or (b) restrict each bullet to the actual `pub` surface only and move the `pub(crate)` mentions to a follow-on paragraph or to each submodule's `//!` (where they already appear in context).

---

**Finding 3 — `cargo doc --no-deps` emits a `rustdoc::bare_urls` warning on `color_mode_from_env`'s doc-comment (Rust supplement — rustdoc fidelity)**

`src/commands.rs` L90 reads:

> 3. `NO_COLOR` set to any non-empty value — `Off` (per https://no-color.org/).

`cargo doc --no-deps` (run as part of this review at HEAD) emits:

```
warning: this URL is not a hyperlink
  --> src/commands.rs:90:59
   |
90 | /// 3. `NO_COLOR` set to any non-empty value — `Off` (per https://no-color.org/).
   |                                                           ^^^^^^^^^^^^^^^^^^^^^
   = note: bare URLs are not automatically turned into clickable links
   = note: `#[warn(rustdoc::bare_urls)]` on by default
help: use an automatic link instead
   |
90 | /// 3. `NO_COLOR` set to any non-empty value — `Off` (per <https://no-color.org/>).
```

The fix is a single character pair: `<` + `>` around the URL on commands.rs L90. The same URL appears correctly bracketed in `issue-tracker-cli/README.md` L30 and in `DECISIONS.md` (Layer 7 IAR Round 2 spec amendments entry) — the rustdoc warning is purely local to this one site. Without the fix, `cargo doc` produces non-zero warnings, which (a) clutter CI output and (b) means the auto-generated rustdoc shows the URL as plain text rather than a clickable link for any developer who runs `cargo doc --open`.

This is a one-character fidelity defect, but it is now the ONLY `cargo doc --no-deps` warning at HEAD — fixing it gets the docs build to warning-clean.

**Classification:** Open. Raised to SE (src/commands.rs is SE authority). Proposed remedy: change `https://no-color.org/` to `<https://no-color.org/>` on commands.rs L90.

---

**Finding 4 — `guild-portfolio/README.md` Layer 7 row still reads `🟡 In IAR Round 2` (Dim 2 — documentation accuracy; recurring TW R7 F1 / R9 F3 portfolio-half defect class)**

`guild-portfolio/README.md` L36 reads `| 7 | Polish (color, `--help`) | 🟡 In IAR Round 2 |`. As of HEAD, Layer 7 has moved through R2 closure (`09b1905` 2026-05-11 23:30Z), Portfolio Assessment R5 (`6b03dee`), PROCESS.md retrospective additions (`8f87f3a`, `2a245f9`), and now the five R3 commits. The portfolio-level reader sees the project as still in R2, two rounds behind HEAD.

Additionally, `issue-tracker-cli/README.md` L74 reads `**Layer 7 implementation complete; Layer 7 IAR Round 2 closure in progress.**`. R2 closure landed in `09b1905`; we are now in R3 cold-batch. The Status block is one round behind. This is a project-README staleness echo of the same class.

These are both mild — R2 closure has actually happened, the project is in R3 (not "R2 closure in progress"), and the portfolio README will need an update anyway when Layer 7 closes terminally. But the pattern (portfolio README + project Status block stale one round at every IAR boundary) is the recurring R7 F1 / R9 F3 class.

**Classification:** Open. TW direct-edit authority for both README files per CLOSURE-PROTOCOL §1. Proposed remedy: amend the project README Status block to `**Layer 7 implementation complete; Layer 7 IAR Round 3 closure in progress.**` (or, if R3 is approaching final-closure, the closure marker); flip the portfolio README Layer 7 row to `🟡 In IAR Round 3`. Both edits can land bundled with the F1 CHANGELOG R3 entry.

---

**Finding 5 — `TODO.md` Layer 7 manual-checklist still shows only the original 7 ticked items; the 6 new R2 manual items the R2 CHANGELOG entry committed to adding never landed (Dim 7 — operational documentation; carry-forward from R2)**

The Layer 7 R2 CHANGELOG entry's "Open (process)" subsection at L47 reads:

> **Layer 7 manual testing checklist** — Re-walk: NO_COLOR / CLICOLOR / CLICOLOR_FORCE behaviors, bold-redundancy rendering in terminal, no ANSI on stderr empty-state. Director to add the new manual items to TODO.md and re-tick.

As of HEAD, `TODO.md` L368–376 (Layer 7 Manual Testing Checklist) still shows only the original 7 items, all `[x]` ticked. No new items have been added for the R2 surfaces: NO_COLOR honoring, CLICOLOR=0 honoring, CLICOLOR_FORCE non-honoring, bold-on-medium rendering, bold-on-in-progress rendering, bold-on-done rendering, no-ANSI-on-stderr-empty-state. The R2 closure entry explicitly committed to this manual re-walk; the closure has not happened in two rounds.

This is also a CLOSURE-PROTOCOL.md merge-gate concern (criterion 3 — manual checklist closure) for the Layer 7 terminal close. If Layer 7 closes terminally without these items added and re-ticked, the same R2-equivalent behaviors will have been ratified into DESIGN.md without a manual-test record.

**Classification:** Open. Raised to Director (TODO.md manual-checklist closure is director-owned per the established cadence at Layers 4 / 6 / 7-R1). Carry-forward from the R2 closure entry's "Open (process)" promise. Proposed action: SO or director adds 6 explicit checkbox items to TODO.md L376 covering the R2 NO_COLOR / CLICOLOR / CLICOLOR_FORCE / bold-redundancy / stderr-empty-state behaviors; director walks them and ticks before Layer 7 terminal close.

---

**Finding 6 — Module-level `//!` doc-comment quality verification on `storage.rs` / `validate.rs` / `commands.rs` — three of three pass with caveats (Dim 6 — API documentation; Rust supplement)**

(Recording the verification, not a finding except via F2's enumeration-vs-export coupling.)

- **`storage.rs` L1–15 //!:** Names the module's responsibility ("data types persisted to `tracker.json` and the load-time invariant checking"), names the public surface (`Tracker`, `Issue`), explains the load-time-invariant treatment of untrusted data, cross-references the other two submodules with the SA R13 F1 Trigger B closure lineage. Voice and depth are consistent with the lib.rs hub. **Verified accurate.**
- **`validate.rs` L1–21 //!:** Names the module's responsibility ("User-input validation and safety transforms"), lists the validators by name, names the safety transforms (`display_safe`, `sanitize_quoted_values`), names the arithmetic / time helpers, cites the SA R13 F1 Trigger B closure, notes that `VALID_STATUSES` / `PRIORITY_ORDER` live in storage as the single source of truth. **Verified accurate.**
- **`commands.rs` L1–26 //!:** Names the module's responsibility (command implementations + rendering layer), names the helpers, documents the single-decision-point color-injection pattern with the SE R17 F1 / SA R15 F2 closure lineage, restates the DESIGN.md color contract, names the new R3 features (`TRACKER_INTERNAL_FORCE_COLOR` seam, `wrap_color` + `render_cell` debug_asserts) with their QE / Security R-numbers. **Verified accurate** and arguably the highest-information-density `//!` of the four.

All four `//!`s (including lib.rs) cite SA R13 F1 Trigger B closure lineage as the IAR finding that drove the split. Voice is consistent (declarative, second-person-implicit, IAR-citation-tagged). The only quality concern is F2's pub-vs-pub(crate) accuracy gap in the lib.rs hub.

**Classification:** Verified — recorded for the durable record, not a finding. The new `//!`s are substantial and consistent in voice and depth; the only drift is the lib.rs hub's pub-vs-pub(crate) elision (F2).

---

### Dismissed

**Finding 7 — `DECISIONS.md` should have a new entry for the three-module split (Dim 4 — decision rationale)**

Suspected: the three-module split is the largest architectural decision since Layer 7's initial color choice, yet DECISIONS.md has no entry for it.

Re-read DECISIONS.md L153–156 ("SA R11 F1 + SA R13 F1 Trigger B + SA R13 F2 auto-Backlog per CLOSURE-PROTOCOL.md §3"). The entry already documents:

- The three findings, with originating R-numbers cited.
- The architectural concern ("`cmd_list` rendering should be its own function; `src/lib.rs` is past the 500-LOC threshold; `format_show_block` column widths are magic numbers").
- The decision-making process: auto-Backlog under CLOSURE-PROTOCOL §3 when the deferral deadline expired.
- The trade-off: the cost-benefit "has not shifted enough to schedule it in any specific upcoming layer".

The R3 commits *resolve* this entry by landing the refactor that the Backlog entry preserved. Whether a new DECISIONS.md entry is needed depends on whether the resolution itself is decision-rationale-bearing. Verified: the resolution did **not** introduce a new design choice — the module-split boundary (storage / validate / commands) was already named in the Backlog entry's prose ("`cmd_list` rendering should be its own function; `src/lib.rs` is past the 500-LOC threshold") and in the originating SA R3 F2 dismissal that named "storage / validate / commands triad". The R3 implementation executed a pre-existing recommendation; it did not make a new decision.

Conversely, the existing entry's annotation could be amended to note "**Resolved by `8db9437` + `3fa1f3c` — module split landed, column constants extracted**" (mirroring the `Reversed by SO Review 22` annotation pattern on SA R1 F3). That edit is bundled-into-F1's CHANGELOG R3 entry as DECISIONS.md curation, not a new entry.

**Classification:** Dismissed. The existing DECISIONS.md entry covers the rationale; the only follow-on is an inline "Resolved by ..." annotation that lives more naturally in the CHANGELOG R3 entry's DECISIONS sub-bullet. No new entry needed.

---

**Finding 8 — Inline doc-comments on the new extracted helpers (`filter_issues`, `format_list_header`, `format_list_row`, `show_label`, column constants, `TRACKER_INTERNAL_FORCE_COLOR` doc-section, `render_cell` ASCII constraint) fail to explain *why* (Dim 5 — inline comment quality)**

Suspected: R3 added a substantial volume of new doc-comments; given the recurring TW concern about doc-currency, a cold reader should spot-check that the new doc-comments explain non-obvious decisions rather than restating the function name.

Read each in turn:

- `filter_issues` (commands.rs L516–522): "Pure function: no I/O, no allocations beyond the resulting `Vec`. Extracted from `cmd_list`'s inline `retain` per SA R11 F1 closure so the filter logic is unit-testable in isolation and a future filter dimension lands as a parameter addition rather than a fourth inline `retain` call." — *Why* is explicit: future-extensibility + unit-testability. ✓
- `format_list_header` (commands.rs L533–545): "Pure function — uses the module-level column-width constants so a future spec amendment that changes column widths touches one site (the constants) rather than the format string. The header row is never colored per DESIGN.md ..." — *Why* explicit (single-site-of-change + DESIGN.md cross-reference). ✓
- `format_list_row` (commands.rs L548–554): explains the visible-width-against-bare-value contract (`Padding for status / priority is done against *visible* character count (via `render_cell`) so ANSI bytes do not consume column budget`). ✓
- `show_label` (commands.rs L385–390): cross-references the SA R13 F2 closure single-source-of-truth rationale. ✓
- Column constants (`ID_WIDTH` etc., commands.rs L42–61): each doc-comment names the widest legal value driving the width (`STATUS_WIDTH` sized for "in-progress"; `LABEL_COLUMN_WIDTH` sized for "Description:"). *Why* explicit. ✓
- `TRACKER_INTERNAL_FORCE_COLOR` section in `color_mode_from_env` (commands.rs L99–118): a 20-line dedicated subsection explaining QE Review 17 Finding 1 lineage, the test seam's necessity (assert_cmd non-TTY pipe), the naming rationale, and the "do not document in --help" stance. ✓ — exceptionally well-documented.
- `render_cell` ASCII constraint (commands.rs L204–217): a 14-line dedicated `# ASCII-only constraint (QE Review 17 Finding 5)` subsection explaining the chars().count() vs. display-width tradeoff, the closed-enum guarantee at call sites, the debug_assert's surfacing role, and the production remediation path (unicode-width crate). ✓ — exceptional.

All new R3 doc-comments explain *why*. No drift between the doc-comment claims and the implementation behavior (every claim spot-checked against the surrounding code).

**Classification:** Dismissed. R3's inline doc-comment quality is uniformly strong; the `TRACKER_INTERNAL_FORCE_COLOR` and `render_cell` ASCII-constraint blocks in particular are the most thoroughly-explained inline doc-comments in the codebase.

---

**Finding 9 — `issue-tracker-cli/README.md` should be updated to reflect the three-module split (Dim 1 — README completeness)**

Suspected: a developer reading the README to understand the codebase architecture would benefit from a "Code structure" note pointing at the three-module split.

Re-read the README. The README is end-user-oriented: install / build / test / commands / color rules / storage shape. The closest thing to a contributor-facing section is the `Project files` table at L92–99, which lists the project-level artifacts (DESIGN.md, TODO.md, DECISIONS.md, PROCESS.md, IAR/). There is no `src/` walkthrough — which is correct for a portfolio CLI README. The IAR Rust supplement's TW dim 1 explicitly carves out internal code structure: "For binary-only crates, exported functions in `lib.rs` must be documented; internal module structure is not part of the README contract."

A contributor reading the source instead gets a thorough module map from `cargo doc` (subject to F2's hub-`//!` accuracy fix). The README does not need a `src/` section.

**Classification:** Dismissed. README is end-user-facing; the module map is correctly delegated to `cargo doc` (with F2's accuracy fix landing in a SE follow-up).

---

### Hallucinated

*(none)*

---

### Deferred

*(none — Round 3 is surfacing; Round 4 / follow-up dispositions per the IAR brief.)*

---

### Summary

**9 findings (5 actionable Open + 1 Verified-recorded + 3 Dismissed):**

- F1 — CHANGELOG.md has no R3 entry; five substantive commits unrecorded; three R2-Deferred items closed but the R2 entry's "Open (process)" not amended (Raised to SO)
- F2 — `src/lib.rs` hub `//!` enumerates pub(crate) items as if exports; omits three actual pub re-exports (Raised to SE)
- F3 — `cargo doc --no-deps` emits one `rustdoc::bare_urls` warning at `src/commands.rs` L90 (Raised to SE; one-character fix)
- F4 — `guild-portfolio/README.md` Layer 7 row + `issue-tracker-cli/README.md` Status block both one round behind HEAD (TW direct-edit)
- F5 — `TODO.md` Layer 7 manual checklist has not added the 6 R2-committed new items (Raised to Director; carry-forward from R2 closure)
- F6 — Three new module-level `//!`s (`storage.rs`, `validate.rs`, `commands.rs`): **Verified strong**, no action
- F7 — DECISIONS.md three-module split entry — Dismissed (existing entry covers rationale; resolution annotation belongs in CHANGELOG R3 entry)
- F8 — Inline doc-comments on R3 new helpers / constants / debug_assert sections — Dismissed; all explain *why*; `TRACKER_INTERNAL_FORCE_COLOR` and `render_cell` ASCII-constraint blocks are exceptional
- F9 — README module-structure update — Dismissed (README is end-user-facing; module map correctly delegated to cargo doc)

**0 Hallucinated.**

**Doc-currency assessment:** R3's *inline* doc-currency is strong — the four module-level `//!`s and the new helper doc-comments are substantial, accurate (modulo F2's pub/pub(crate) accuracy), and explain *why*. R3's *project-level* doc-currency is weak — the CHANGELOG missed five commits (F1, recurring R7 F2 / R9 F1 class), the portfolio + project READMEs are one round stale (F4, recurring R7 F1 / R9 F3 class), and the R2-committed manual-checklist re-walk has not materialized in TODO.md (F5, carry-forward from R2 closure). The R3 implementation work is excellently documented at the source level; the project-level artifacts that index it have not kept pace.

**Top concern:** Finding 1 (CHANGELOG R3 entry missing). Three of the five R3 commits close findings the R2 CHANGELOG entry explicitly listed as `Deferred` — leaving those items uncrossed in the CHANGELOG is the same defect class as TW R7 F2 / R9 F1, which R11 noted was broken at Layer 7 R1. The pattern has resumed. Sycophancy check: I considered classifying the absence as expected ("R3 is mid-flight; CHANGELOG entries land at closure"), but R2 closure shipped a CHANGELOG entry the same day as the R2 work, and the five R3 commits all carry full commit-message rationale that maps cleanly to a CHANGELOG bullet — there is no information gap, only a curation gap.

**Coordination:**

- F1 (CHANGELOG R3 entry) → **Raised to SO** (CHANGELOG curation is SO-primary per CLOSURE-PROTOCOL §1).
- F2 (`lib.rs` hub `//!` pub-vs-pub(crate) drift) → **Raised to SE** (src/lib.rs is SE authority).
- F3 (rustdoc bare-URL warning) → **Raised to SE** (one-character fix in commands.rs L90).
- F4 (portfolio + project README round-number staleness) → TW direct-edit; bundle with F1 CHANGELOG edit.
- F5 (TODO.md manual checklist) → **Raised to Director** (carry-forward from R2 closure).
- F6 / F7 / F8 / F9 — no coordination; recorded for the durable record.
- Cross-domain: F1 also flags SA (R3 closed three SA-originating findings — SA may want to verify the CHANGELOG entry's SA-finding-closure citations); F2 overlaps with SA R16-equivalent (module-split documentation accuracy).

**Files modified:** Only this log appended. Per the IAR brief, R3 is the surfacing pass; the CHANGELOG R3 entry / `lib.rs` `//!` refinement / commands.rs rustdoc-URL fix / portfolio + project README round-number flips / TODO.md manual-checklist additions all land in a follow-up (or at R4 / Layer 7 terminal close) if at all.
