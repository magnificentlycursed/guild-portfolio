# bookmark-cli — Findings Index

Cross-cutting registry of every classified finding across every domain and layer. The narrative for each finding lives in the per-session file linked from the row; this file is the index only.

Structured like [`vsdd-suite/suite-development/GAP-ANALYSIS-LOG.md`](../../vsdd-suite/suite-development/GAP-ANALYSIS-LOG.md) — same column shape, same close-don't-delete discipline. Manual path of G-138 (project-level finding index); `bookmark-cli` is built via the suite-only path per G-117 ratification, so the manual path applies.

**Reading convention:** the canonical narrative for each finding is in its per-session file, linked in the **Anchor** column. This file's rows are summaries for fast cross-cutting filtering.

---

## Quick lookup

- **By domain:** `grep "| quality-engineer |"` (or any other active domain slug)
- **By layer:** `grep "| L1 |"`
- **By classification:** `grep "| Resolved |"` / `| Hallucinated |` / `| Open |` / `| Deferred |`
- **By source:** `grep "| director-raised |"` / `| domain-raised |` / `| regression-replay |`
- **Open findings only:** `grep "| Open |"` (currently: none — bookmark-cli has no Open findings)

---

## Findings registry

| ID | Layer | Round | Domain | Finding | Title | Source | Classification | Status | Anchor |
|---|---|---|---|---|---|---|---|---|---|
| F-005 | L1 | R2 | quality-engineer | F1 | Surface B surviving non-equivalent mutant at src/lib.rs:48 (delete ! in BookmarkStore::save) — missing falsifying test for save-to-nested-path case; resolved by adding src/lib.rs::tests::save_creates_parent_directory_for_nested_path with retroactive-Red-Gate (Phase 5 source) label; post-fix kill rate on viable mutants 8/8 = 100% | domain-raised | Resolved | Closed | [QE R2 F1](review-log/2026-05-20-quality-engineer.md#review-2--2026-05-20-0245z) |
| F-004 | L1 | R1 | solution-architect | F1 | Phase 5 Surface A.0 cross-source purity-boundary divergence (Dim 12) — src/lib.rs:1-7 module doc claimed "Pure-core storage logic", DESIGN.md § Verification architecture was silent, impl had 3 of 4 BookmarkStore methods effectful; resolved by rewriting DESIGN.md § Verification architecture with explicit Purity boundary subsection and retiring the prior "Pure-core" module-doc claim | domain-raised | Resolved | Closed | [SA R1 F1](review-log/2026-05-20-solution-architect.md#review-1--2026-05-20-0245z) |
| F-003 | L1 | R1 | quality-engineer | F3 | Claim of insufficient test count (rejected — 8/8 pass against 4 ACs) | domain-raised | Hallucinated | Closed | [QE R1 F3](review-log/2026-05-17-quality-engineer.md#review-1--2026-05-17-0325z) |
| F-002 | L1 | R1 | quality-engineer | F2 | Missing test coverage for whitespace-only-URL and URL-with-newlines edge cases named in DESIGN.md | domain-raised | Resolved | Closed | [QE R1 F2](review-log/2026-05-17-quality-engineer.md#review-1--2026-05-17-0325z) |
| F-001 | L1 | R1 | quality-engineer | F1 | Phase 2a → 2b commit boundary not enforced (acknowledged scope tradeoff of reference-implementation context) | domain-raised | Resolved | Closed | [QE R1 F1](review-log/2026-05-17-quality-engineer.md#review-1--2026-05-17-0325z) |

---

## Cross-references

- [`README.md`](../README.md) — project README
- [`DESIGN.md`](../DESIGN.md) — Phase 1a contract
- [`TODO.md`](../TODO.md) — layer plans + manual testing checklists
- Per-domain index files in this directory — round-level rollup per domain (QUALITY-ENGINEER-REVIEW.md and SOLUTION-ARCHITECT-REVIEW.md customized; five others remain as scaffolded stubs)
- Per-session files in [`review-log/`](review-log/) — finding-level narratives
- [`vsdd-suite/suite-development/suite-development.md`](../../vsdd-suite/suite-development/suite-development.md) § Project-level finding index — the governing standard for this file
