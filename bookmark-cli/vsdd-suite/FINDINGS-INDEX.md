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
| F-003 | L1 | R1 | quality-engineer | F3 | Claim of insufficient test count (rejected — 8/8 pass against 4 ACs) | domain-raised | Hallucinated | Closed | [QE R1 F3](review-log/2026-05-17-quality-engineer.md#review-1--2026-05-17-0325z) |
| F-002 | L1 | R1 | quality-engineer | F2 | Missing test coverage for whitespace-only-URL and URL-with-newlines edge cases named in DESIGN.md | domain-raised | Resolved | Closed | [QE R1 F2](review-log/2026-05-17-quality-engineer.md#review-1--2026-05-17-0325z) |
| F-001 | L1 | R1 | quality-engineer | F1 | Phase 2a → 2b commit boundary not enforced (acknowledged scope tradeoff of reference-implementation context) | domain-raised | Resolved | Closed | [QE R1 F1](review-log/2026-05-17-quality-engineer.md#review-1--2026-05-17-0325z) |

---

## Cross-references

- [`README.md`](../README.md) — project README
- [`DESIGN.md`](../DESIGN.md) — Phase 1a contract
- [`TODO.md`](../TODO.md) — layer plans + manual testing checklists
- Per-domain index files in this directory — round-level rollup per domain (QUALITY-ENGINEER-REVIEW.md is the only one customized in the reference-implementation scope; six others remain as scaffolded stubs)
- Per-session files in [`review-log/`](review-log/) — finding-level narratives
- [`vsdd-suite/suite-development/suite-development.md`](../../vsdd-suite/suite-development/suite-development.md) § Project-level finding index — the governing standard for this file
