# {{PROJECT_NAME}} — Findings Index

Cross-cutting registry of every classified finding across every domain and layer. The narrative for each finding lives in the per-session file linked from the row; this file is the index only.

Structured like [`vsdd-suite/suite-development/FINDINGS-INDEX.md`](../../vsdd-suite/suite-development/FINDINGS-INDEX.md) — same column shape, same close-don't-delete discipline. New findings get new rows; status changes update existing rows. Do not delete rows.

**When to use this file:** the manual path of G-138 (project-level finding index). Projects using crosslink for finding tracking populate `crosslink issue` instead; the markdown rows and crosslink labels carry the same information shape so a project can migrate between them with `crosslink import` / `crosslink export`.

**Reading convention:** the canonical narrative for each finding is in its per-session file, linked in the **Anchor** column. This file's rows are summaries for fast cross-cutting filtering (by domain, layer, classification, source). Use a markdown viewer's table filter (or `grep`) for quick lookup.

---

## Quick lookup

- **By domain:** `grep "| quality-engineer |"` (or equivalent for any domain slug)
- **By layer:** `grep "| L2 |"`
- **By classification:** `grep "| Resolved |"` / `| Hallucinated |` / `| Open |` / `| Deferred |` / `| Accepted Risk |` / `| Backlogged |` / `| Dismissed |`
- **By source:** `grep "| director-raised |"` / `| domain-raised |` / `| regression-replay |`
- **By owner (Review 77):** `grep "| Owner: software-engineer |"` (or any domain slug — names the domain accountable for the resolution; distinct from Domain which is the raising domain)
- **By validator (Review 77):** `grep "| Validator: red-team |"` (validator of Resolved findings — typically the natural adversarial pair per the domain prompts' validator-pair paragraphs)
- **Open findings only:** `grep "| Open |"`
- **Self-validated findings (audit-trail signal):** `grep "| Validator: \*self\* |"` — surfaces every Resolved finding that did not get cross-domain validation; useful for assessing the discipline's health (a project whose Resolved findings cluster heavily in self-validation has not exercised the validator-pair pattern)

---

## Findings registry

| ID | Layer | Round | Domain | Finding | Title | Source | Classification | Owner | Validator | Status | Anchor |
|---|---|---|---|---|---|---|---|---|---|---|---|
| F-001 | L1 | R1 | software-engineer | F1 | (example title) | domain-raised | Resolved | software-engineer | quality-engineer | Closed | [SE R1 F1](review-log/2026-06-15-software-engineer.md#review-1--2026-06-15-1400z) |
| F-002 | L1 | R1 | quality-engineer | F2 | (example title) | domain-raised | Hallucinated | — | — | Closed | [QE R1 F2](review-log/2026-06-15-quality-engineer.md#review-1--2026-06-15-1430z) |

<!-- Delete the example rows above when this file is first populated for your project.
     Maintain newest-first ordering (latest finding at top of the table).
     Status values: Open | Closed
     Classification values per domain — see the domain prompt file's Current Review Prompt section.
     Source values: domain-raised | director-raised | regression-replay | external-feedback | mixed (per G-133).
     Owner values (Review 77): a single domain slug; the domain accountable for the resolution. Distinct from Domain (the raising domain). For Raised-to-SO findings, Owner = solution-owner. Hallucinated findings have no Owner (use —).
     Validator values (Review 77): a domain slug for cross-domain validation OR `*self*` for self-validation (with rationale in the per-finding body). For Resolved findings only — Hallucinated/Dismissed have no Validator (use —).
     The Review 77 lifecycle fields (Owner / Status / Validator + the Status sub-state on Open findings: raised / assigned / fix-landed / validated) apply forward-only to findings dated 2026-05-21 or later. -->

---

## Cross-references

- [`README.md`](../README.md) — project README
- [`DESIGN.md`](../DESIGN.md) — Phase 1a+1b contract
- [`TODO.md`](../TODO.md) — layer plans + manual testing checklists
- Per-domain index files in this directory — round-level rollup per domain
- Per-session files in [`review-log/`](review-log/) — finding-level narratives
- [`vsdd-suite/suite-development/suite-development.md`](../../vsdd-suite/suite-development/suite-development.md) § Project-level finding index — the governing standard for this file
