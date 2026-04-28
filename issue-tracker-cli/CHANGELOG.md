# Changelog

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
