# bookmark-cli-manual — Findings Index

Cross-cutting registry of every classified finding across every domain and layer. The narrative for each finding lives in the per-session file linked from the row; this file is the index only.

Structured like [`vsdd-suite/suite-development/FINDINGS-INDEX.md`](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md) — same column shape, same close-don't-delete discipline. Manual-method reference implementation for G-138 (project-level finding index); `bookmark-cli-manual` is built via the suite-only path per G-117 ratification, so the manual path applies.

**Reading convention:** the canonical narrative for each finding is in its per-session file, linked in the **Anchor** column. This file's rows are summaries for fast cross-cutting filtering.

---

## Quick lookup

- **By domain:** `grep "| quality-engineer |"` (or any other active domain slug)
- **By layer:** `grep "| L1 |"`
- **By classification:** `grep "| Resolved |"` / `| Hallucinated |` / `| Open |` / `| Deferred |`
- **By source:** `grep "| director-raised |"` / `| domain-raised |` / `| regression-replay |`
- **Open findings only:** `grep "| Open |"` (currently: 4 Open — SO Round 1 (2026-05-20 19:30Z) filed F-006/F-007 Backlogged and F-008/F-009 Open per the deliverable-vs-promise + documentation-defect Findings; resolution paths tracked in the per-Finding `Blocked by:` lifecycle fields)

---

## Findings registry

Schema extended in PR 6 / Review 78 with `Owner` + `Validator` columns per Review 77 lifecycle convention (the suite's project FINDINGS-INDEX template was extended in PR 5; this reference example migrates to the extended schema as part of its capstone-intent promotion per G-177).

| ID | Layer | Round | Domain | Finding | Title | Source | Classification | Owner | Validator | Status | Anchor |
|---|---|---|---|---|---|---|---|---|---|---|---|
| F-027 | L1 | R1 | vdd-iar-alignment | F5 | SA per-domain index anchor link points at QE Review 1 instead of QE Review 2 (Dim 7 cross-link defect — Mutation Testing companion round mis-cited) | domain-raised | Resolved | solution-architect | sanity-check | Raised | [VDD-IAR-A R1 F5](review-log/2026-05-20-vdd-iar-alignment.md#review-1--2026-05-20-1930z) |
| F-026 | L1 | R1 | vdd-iar-alignment | F4 | Platform Engineer Dim 38 fresh-system install verification gate unsatisfied — manual-tests/install-verification.md Outcome `*(pending)*`; capstone-intent project with unsatisfied human-verification gate (Dim 5); coordinates with PE R1 F9 (F-018) | domain-raised | Resolved | platform-engineer | sanity-check | Raised | [VDD-IAR-A R1 F4](review-log/2026-05-20-vdd-iar-alignment.md#review-1--2026-05-20-1930z) |
| F-025 | L1 | R1 | vdd-iar-alignment | F3 | Capstone-active Phase 3 IAR coverage incomplete at intent-promotion gate (Dim 3 layer-gate); coordinates with SO R1 F3 (F-008) which raises the same state from the SO lens | domain-raised | Resolved | vdd-iar-alignment | sanity-check | Raised | [VDD-IAR-A R1 F3](review-log/2026-05-20-vdd-iar-alignment.md#review-1--2026-05-20-1930z) |
| F-024 | L1 | R1 | vdd-iar-alignment | F2 | SA Review 1 Finding 1 applied DESIGN.md change in-session without an SO ratification round; classified Resolved instead of Raised-to-SO (Dim 10 — Raised-to-SO routing) | domain-raised | Resolved | solution-architect | sanity-check | Raised | [VDD-IAR-A R1 F2](review-log/2026-05-20-vdd-iar-alignment.md#review-1--2026-05-20-1930z) |
| F-023 | L1 | R1 | vdd-iar-alignment | F1 | QE Review 1 Finding 2 classified as Resolved when no fix was applied — finding was routed to future Layer 1.5 (should be Deferred per QE classification universe; Dim 9 — Classification universe correctness) | domain-raised | Resolved | quality-engineer | sanity-check | Raised | [VDD-IAR-A R1 F1](review-log/2026-05-20-vdd-iar-alignment.md#review-1--2026-05-20-1930z) |
| F-022 | L1 | R1 | vdd-iar-alignment | F7 | In-session IAR rounds for QE R1 + QE R2 + SA R1 — already-documented defect with per-round sycophancy-compensation declared; not re-raised per regression-check discipline (Dim 6 — IAR fresh context) | domain-raised | Dismissed | — | — | Closed | [VDD-IAR-A R1 F7](review-log/2026-05-20-vdd-iar-alignment.md#review-1--2026-05-20-1930z) |
| F-021 | L1 | R1 | vdd-iar-alignment | F6 | Phase 2a Red Gate single-commit (DESIGN + TODO + tests + impl all introduced in commit a371469) — already-documented defect at QE R1 F1; not re-raised per regression-check discipline (Dim 4 — Red Gate commit precedence) | domain-raised | Dismissed | — | — | Closed | [VDD-IAR-A R1 F6](review-log/2026-05-20-vdd-iar-alignment.md#review-1--2026-05-20-1930z) |
| F-020 | L1 | R1 | platform-engineer | F11 | Clippy lint configuration relies on cargo defaults; no crate-level `#![deny(...)]` deny set per Rust supplement § SE Clippy lint configuration | domain-raised | Deferred | platform-engineer | software-engineer | Open | [PE R1 F11](review-log/2026-05-20-platform-engineer.md#review-1--2026-05-20-1930z) |
| F-019 | L1 | R1 | platform-engineer | F10 | No coverage measurement or threshold enforcement; routed to SO for Backlog ratification mirroring ITC SO R14 F5 disposition | domain-raised | Deferred | platform-engineer | quality-engineer | Open | [PE R1 F10](review-log/2026-05-20-platform-engineer.md#review-1--2026-05-20-1930z) |
| F-018 | L1 | R1 | platform-engineer | F9 | Capstone Dim 38 install-verification record has zero PASSING rows; the gate is declared but not satisfied — capstone closure pending non-author fresh-system execution | domain-raised | Deferred | platform-engineer | *self* | Open | [PE R1 F9](review-log/2026-05-20-platform-engineer.md#review-1--2026-05-20-1930z) |
| F-017 | L1 | R1 | platform-engineer | F8 | `cargo install --path .` invocations across README + manual-tests + install-verification do not use `--locked`; committed Cargo.lock advisory at install time | domain-raised | Deferred | platform-engineer | software-engineer | Open | [PE R1 F8](review-log/2026-05-20-platform-engineer.md#review-1--2026-05-20-1930z) |
| F-016 | L1 | R1 | platform-engineer | F7 | Pre-commit cargo-fmt-check / cargo-clippy-check hooks scoped to `issue-tracker-cli/` only; bookmark-cli-manual sources have no shift-left fmt/clippy enforcement | domain-raised | Deferred | platform-engineer | sanity-check | Open | [PE R1 F7](review-log/2026-05-20-platform-engineer.md#review-1--2026-05-20-1930z) |
| F-015 | L1 | R1 | platform-engineer | F6 | Cargo.toml has no `[profile.release]` declarations; release-build tuning is silently default | domain-raised | Deferred | platform-engineer | performance-engineer | Open | [PE R1 F6](review-log/2026-05-20-platform-engineer.md#review-1--2026-05-20-1930z) |
| F-014 | L1 | R1 | platform-engineer | F5 | No `cargo audit` invocation anywhere; known-CVE detection is absent | domain-raised | Deferred | platform-engineer | security | Open | [PE R1 F5](review-log/2026-05-20-platform-engineer.md#review-1--2026-05-20-1930z) |
| F-013 | L1 | R1 | platform-engineer | F4 | No `deny.toml` / no `cargo deny` configuration; supply-chain policy surface missing all four sections (advisories / licenses / bans / sources) | domain-raised | Deferred | platform-engineer | security | Open | [PE R1 F4](review-log/2026-05-20-platform-engineer.md#review-1--2026-05-20-1930z) |
| F-012 | L1 | R1 | platform-engineer | F3 | Cargo.toml `[package]` missing canonical fields (`repository`, `readme`, `rust-version`); MSRV declared in DESIGN.md not mechanized | domain-raised | Deferred | platform-engineer | software-engineer | Open | [PE R1 F3](review-log/2026-05-20-platform-engineer.md#review-1--2026-05-20-1930z) |
| F-011 | L1 | R1 | platform-engineer | F2 | No `rust-toolchain.toml` despite a declared MSRV in DESIGN.md; toolchain pin is operator-discipline rather than mechanism | domain-raised | Deferred | platform-engineer | software-engineer | Open | [PE R1 F2](review-log/2026-05-20-platform-engineer.md#review-1--2026-05-20-1930z) |
| F-010 | L1 | R1 | platform-engineer | F1 | No CI workflow exists for `bookmark-cli-manual`; pipeline completeness + gate enforcement absent (no fmt/clippy/test/audit/deny gates) | domain-raised | Deferred | platform-engineer | software-engineer | Open | [PE R1 F1](review-log/2026-05-20-platform-engineer.md#review-1--2026-05-20-1930z) |
| F-009 | L1 | R1 | solution-owner | F4 | Deliverable-vs-promise misalignment: Layer 1 status — README.md:9 says "complete" while TODO.md:11 says "In progress"; TODO.md:5 "(10 active domains)" stale against DESIGN.md:11 "12 active domains" | domain-raised | Open | solution-owner | sanity-check | Open | [SO R1 F4](review-log/2026-05-20-solution-owner.md#r1-f4) |
| F-008 | L1 | R1 | solution-owner | F3 | Capstone-intent gate criteria not satisfied — only 3 of 12 active domains have IAR rounds filed; Phase 6 four-dimensional convergence record not landed; CHANGELOG explicitly defers gate-close to PR 7 while README claims Layer 1 complete | domain-raised | Open | solution-owner | sanity-check | Open | [SO R1 F3](review-log/2026-05-20-solution-owner.md#r1-f3) |
| F-007 | L1 | R1 | solution-owner | F2 | Storage format divergence — manual-tests/layer-1.md:62 expects bare-array shape `[{...}]` but DESIGN.md:103-110 specifies object-wrapped shape `{"bookmarks":[...]}` (which is what the impl correctly produces); test-plan's expected output contradicts the spec and would falsely flag the correct implementation as divergent | domain-raised | Backlogged | solution-owner | sanity-check | Open | [SO R1 F2](review-log/2026-05-20-solution-owner.md#r1-f2) |
| F-006 | L1 | R1 | solution-owner | F1 | README.md:59 phase progression table Phase 4 row claims "N/A — no live findings to route" but FINDINGS-INDEX lists 5 findings (4 Resolved + 1 Hallucinated); routing was exercised in-session per the existing QE / SA review closures | domain-raised | Backlogged | solution-owner | sanity-check | Open | [SO R1 F1](review-log/2026-05-20-solution-owner.md#r1-f1) |
| F-005 | L1 | R2 | quality-engineer | F1 | Mutation Testing surviving non-equivalent mutant at src/lib.rs:48 (delete ! in BookmarkStore::save) — missing falsifying test for save-to-nested-path case; resolved by adding src/lib.rs::tests::save_creates_parent_directory_for_nested_path with retroactive-Red-Gate (Phase 5 source) label; post-fix kill rate on viable mutants 8/8 = 100% | domain-raised | Resolved | software-engineer | quality-engineer | Closed | [QE R2 F1](review-log/2026-05-20-quality-engineer.md#review-2--2026-05-20-0245z) |
| F-004 | L1 | R1 | solution-architect | F1 | Phase 5 Purity Boundary Audit cross-source purity-boundary divergence (Dim 12) — src/lib.rs:1-7 module doc claimed "Pure-core storage logic", DESIGN.md § Verification architecture was silent, impl had 3 of 4 BookmarkStore methods effectful; resolved by rewriting DESIGN.md § Verification architecture with explicit Purity boundary subsection and retiring the prior "Pure-core" module-doc claim | domain-raised | Resolved | solution-owner | solution-architect | Closed | [SA R1 F1](review-log/2026-05-20-solution-architect.md#review-1--2026-05-20-0245z) |
| F-003 | L1 | R1 | quality-engineer | F3 | Claim of insufficient test count (rejected — 8/8 pass against 4 ACs) | domain-raised | Hallucinated | — | — | Closed | [QE R1 F3](review-log/2026-05-17-quality-engineer.md#review-1--2026-05-17-0325z) |
| F-002 | L1 | R1 | quality-engineer | F2 | Missing test coverage for whitespace-only-URL and URL-with-newlines edge cases named in DESIGN.md | domain-raised | Resolved | software-engineer | quality-engineer | Closed | [QE R1 F2](review-log/2026-05-17-quality-engineer.md#review-1--2026-05-17-0325z) |
| F-001 | L1 | R1 | quality-engineer | F1 | Phase 2a → 2b commit boundary not enforced (acknowledged scope tradeoff of reference-implementation context) | domain-raised | Resolved | quality-engineer | sanity-check | Closed | [QE R1 F1](review-log/2026-05-17-quality-engineer.md#review-1--2026-05-17-0325z) |

---

## Cross-references

- [`README.md`](../README.md) — project README
- [`DESIGN.md`](../DESIGN.md) — Phase 1a contract
- [`TODO.md`](../TODO.md) — layer plans + manual testing checklists
- Per-domain index files in this directory — round-level rollup per domain (QUALITY-ENGINEER-REVIEW.md and SOLUTION-ARCHITECT-REVIEW.md customized; five others remain as scaffolded stubs)
- Per-session files in [`review-log/`](review-log/) — finding-level narratives
- [`vsdd-suite/suite-development/suite-development.md`](../../../vsdd-suite/suite-development/suite-development.md) § Project-level finding index — the governing standard for this file
