# VSDD Suite — Suite Development

This directory holds the materials for **evolving the suite itself** — adding domains, updating dimensions, running gap analysis, logging suite reviews. It is contributor-facing, not user-facing. Project teams using the suite to ship software should read [`../README.md`](../README.md) instead.

The split between `vsdd-suite/` (user-facing) and `vsdd-suite/suite-development/` (contributor-facing) is structural per G-93. Promotion of contributor content into the user-facing top level (or vice versa) is itself a suite-development decision, recorded here.

---

## What lives here

| Artifact | Purpose |
|---|---|
| [`suite-development.md`](suite-development.md) | The suite-development session primer. Load this at the start of any session that intends to modify suite artifacts — adding a domain, updating dimensions, registering a gap, running a suite review, restructuring the suite. Not for reviewing projects. |
| [`SUITE-REVIEW-INDEX.md`](SUITE-REVIEW-INDEX.md) | Index of all suite review sessions. The suite is itself a software artifact and gets reviewed adversarially with the same discipline projects receive. Each session is logged in `review-log/YYYY-MM-DD-suite-review.md`; this file is the index. |
| [`GAP-ANALYSIS-LOG.md`](GAP-ANALYSIS-LOG.md) | Living gap registry. Status-only table of every identified suite gap. Narratives live in the per-session review-log files; this file tracks status and links back to the session that registered the gap. |
| [`review-log/`](review-log/) | Per-session suite-review entries, one file per session date (`YYYY-MM-DD-suite-review.md`). The mode (defect-search vs. registry-walk) is recorded per entry in the `Lens` field — they share a single artifact type. |

### Pure core / effectful shell

The suite applies SA Dim 12 (VSDD purity boundary map) to itself per G-122. The suite's artifacts fall into two categories:

| Category | Examples | Why this matters |
|---|---|---|
| **Pure core** — markdown content with no side effects when read | All primer files (`../primers/`), all domain prompts (`../domains/`), all language and interface supplements (`../supplements/`), `../README.md`, `../CHANGELOG.md`, `../COMPATIBILITY.md`, `../crosslink-contract.md`, the contributor primer in this directory, this README, `SUITE-REVIEW-INDEX.md`, `GAP-ANALYSIS-LOG.md`, all session files in `review-log/` | Trivially copy-able into project trees and AI sessions without bootstrap concerns. The pure core is the suite's deliverable; the effectful shell is the suite's tooling. |
| **Effectful shell** — scripts that touch the filesystem, run subprocesses, or interact with external tools | `../hooks/check-review-log-anonymization.sh` (filesystem reads, grep, exit codes), `../templates/scaffold-project.sh` (filesystem writes, copies templates into target project) | Each effectful artifact has a documented purpose, an explicit no-op-on-empty behavior, and uses `set -euo pipefail` (or equivalent) to fail-fast on unexpected state. |

The boundary is small by design — the suite is primarily content, not code. Any future suite-side tooling (additional hook scripts, an extended scaffold helper, etc.) would expand the effectful shell; if that happens, this section is the place to document it.

---

## Suite-development workflow

A typical pass:

1. **Open a suite-development session.** Load [`suite-development.md`](suite-development.md) as the session primer. This is a deliberate posture shift — you are modifying suite artifacts, not reviewing a project.
2. **Pick a lens.** Either a *named defect class* ("primer naming consistency"), a *registry walk* ("walk all Open gaps registered before 2026-05-01"), or a *role-based lens* ("Solution Owner + Technical Writer pass on the README").
3. **Run the review.** Apply the lens to the relevant artifacts. Record findings in a new entry in [`review-log/`](review-log/) per the format in [`suite-development.md`](suite-development.md) § "Suite review entry format".
4. **Update [`GAP-ANALYSIS-LOG.md`](GAP-ANALYSIS-LOG.md).** Any new gap registered, any status change to an existing gap, any closure — all get recorded in the registry. A session that registers a gap without updating the registry is an incomplete session.
5. **Add a row to [`SUITE-REVIEW-INDEX.md`](SUITE-REVIEW-INDEX.md).** An unindexed session is invisible to future reviewers. The index is read first; the session file is read second.
6. **Update [`../CHANGELOG.md`](../CHANGELOG.md)** if the session resulted in artifact changes (added/changed/removed). Suite-only narrative findings without artifact changes do not need a CHANGELOG entry — they live in the review-log only.

The session-isolation tradeoff is documented in [`suite-development.md`](suite-development.md) § "Session isolation" — suite reviews are typically run in-session with the suite's authorial context, with a `**Session note:**` line per entry naming the isolation status and sycophancy compensation.

---

## Reactivation triggers

Some gaps are deferred against named reactivation triggers (e.g., "after `issue-tracker-cli` completes"). The trigger conditions and the gaps they gate are documented in [`GAP-ANALYSIS-LOG.md`](GAP-ANALYSIS-LOG.md) § "Reactivation triggers". When a trigger fires, all gaps sharing it become eligible — they do not all need to be addressed in the same pass.

---

## Promoting project-scoped material to suite-level

Project IAR sessions sometimes produce findings whose substance generalizes — a closure protocol, a primer addition, a missing dimension. Per the "earned by recurrence" doctrine, suite-level promotion requires evidence the pattern recurs across projects. One project's recurrence is project-scope evidence; suite-level addition should wait until a second project independently encounters the pattern, OR until the project-scoped resolution is explicitly promoted under its own §promotion mechanism.

When a promotion happens, the project's CLOSURE-PROTOCOL.md (or equivalent) is moved to `../` or another suite-level location; the project's local copy becomes a stub pointing at the suite-level canonical version.

---

## Related references

- [`../README.md`](../README.md) — User-facing README. The "Suite scope" section there lists what the suite is for; this contributor README is the companion for *how the suite itself is maintained*.
- [`../CHANGELOG.md`](../CHANGELOG.md) — Suite changelog. Suite-development sessions that change artifacts add entries here.
- [`../primers/`](../primers/) — User-facing session primers (Phases 1a, 1b, 2a, 2b, 3, 4). The suite-development primer is intentionally located here in `suite-development/` rather than in `primers/`, because it is meta to the user-facing primer set.
