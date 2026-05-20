# VSDD Suite — Suite Development

This directory holds the materials for **evolving the suite itself** — adding domains, updating dimensions, registering and walking findings, logging suite reviews. It is contributor-facing, not user-facing. Project teams using the suite to ship software should read [`../README.md`](../README.md) instead.

The split between `vsdd-suite/` (user-facing) and `vsdd-suite/suite-development/` (contributor-facing) is structural per G-93. Promotion of contributor content into the user-facing top level (or vice versa) is itself a suite-development decision, recorded here.

## Two operational modes (design principle)

The suite supports two operational modes for users: `[crosslink]` (recommended, when crosslink is installed and adopted by the project) and `[manual]` (first-class fallback, when crosslink is not in use). Both modes carry the same VSDD discipline — same primers, same domains, same Red Gate, same routing table, same MVR signal. The trade-off between modes is mechanical (issue graph + label filter + swarm dispatch vs. markdown index + grep + human routing), not methodological. See `../README.md` § "Two modes of operation (design principle)" for the user-facing statement.

This design principle is binding on suite contributors: when adding a primer, domain, dimension, hook, or template, every crosslink-only mechanism MUST have a manual-mode equivalent that preserves the same discipline. A primer that says "if you don't have crosslink, you can't do this" is a defect. The manual mode is not a degraded path — it is a fully supported mode that the suite scaffolds, documents, and tests with the same rigour. Suite reviews evaluate both modes; a review that only validates the crosslink mode is incomplete.

---

## What lives here

| Artifact | Purpose |
|---|---|
| [`suite-development.md`](suite-development.md) | The suite-development session primer. Load this at the start of any session that intends to modify suite artifacts — adding a domain, updating dimensions, registering a gap, running a suite review, restructuring the suite. Not for reviewing projects. |
| [`SUITE-DEVELOPMENT-REVIEW.md`](SUITE-DEVELOPMENT-REVIEW.md) | Index of all suite review sessions. The suite is itself a software artifact and gets reviewed adversarially with the same discipline projects receive. Each session is logged in `review-log/YYYY-MM-DD-suite-review.md`; this file is the index. |
| [`FINDINGS-INDEX.md`](FINDINGS-INDEX.md) | Living findings registry. Status-only registry of every identified finding against the suite, structured to mirror the project-level FINDINGS-INDEX shape so suite contributors and suite users encounter consistent conventions across scopes. Two sections — a forward-only registry (no ID prefix; findings identified by `Review N Finding M` anchor) and a legacy registry (`G-01–G-182`, closed to new entries, preserved as historical anchors per the forward-only narrative-preservation policy). Narratives live in the per-session review-log files; this file tracks status and links back to the session that registered the finding. |
| [`review-log/`](review-log/) | Per-session suite-review entries, one file per session date (`YYYY-MM-DD-suite-review.md`). The mode (defect-search vs. registry-walk) is recorded per entry in the `Lens` field — they share a single artifact type. |

### Pure core / effectful shell

The suite applies SA Dim 12 (VSDD purity boundary map) to itself per G-122. The suite's artifacts fall into two categories:

| Category | Examples | Why this matters |
|---|---|---|
| **Pure core** — markdown content with no side effects when read | All primer files (`../primers/`), all domain prompts (`../domains/`), all language and interface supplements (`../supplements/`), `../README.md`, `../CHANGELOG.md`, `../COMPATIBILITY.md`, `../crosslink-contract.md`, the contributor primer in this directory, this README, `SUITE-DEVELOPMENT-REVIEW.md`, `FINDINGS-INDEX.md`, all session files in `review-log/` | Trivially copy-able into project trees and AI sessions without bootstrap concerns. The pure core is the suite's deliverable; the effectful shell is the suite's tooling. |
| **Effectful shell** — scripts that touch the filesystem, run subprocesses, or interact with external tools | `../hooks/check-review-log-anonymization.sh` (filesystem reads, grep, exit codes), `../templates/scaffold-project.sh` (filesystem writes, copies templates into target project) | Each effectful artifact has a documented purpose, an explicit no-op-on-empty behavior, and uses `set -euo pipefail` (or equivalent) to fail-fast on unexpected state. |

The boundary is small by design — the suite is primarily content, not code. Any future suite-side tooling (additional hook scripts, an extended scaffold helper, etc.) would expand the effectful shell; if that happens, this section is the place to document it.

---

## Suite-development workflow

A typical pass:

1. **Open a suite-development session.** Load [`suite-development.md`](suite-development.md) as the session primer. This is a deliberate posture shift — you are modifying suite artifacts, not reviewing a project.
2. **Pick a lens.** Either a *named defect class* ("primer naming consistency"), a *registry walk* ("walk all Open findings registered before 2026-05-01"), or a *role-based lens* ("Solution Owner + Technical Writer pass on the README").
3. **Run the review.** Apply the lens to the relevant artifacts. Record findings in a new entry in [`review-log/`](review-log/) per the format in [`suite-development.md`](suite-development.md) § "Suite review entry format".
4. **Update [`FINDINGS-INDEX.md`](FINDINGS-INDEX.md).** Any new finding registered (in the forward-only section, identified by its `Review N Finding M` anchor — no new ID prefix), any status change to an existing finding (in either section), any closure — all get recorded in the registry. A session that registers a finding without updating the registry is an incomplete session.
5. **Add a row to [`SUITE-DEVELOPMENT-REVIEW.md`](SUITE-DEVELOPMENT-REVIEW.md).** An unindexed session is invisible to future reviewers. The index is read first; the session file is read second.
6. **Update [`../CHANGELOG.md`](../CHANGELOG.md)** if the session resulted in artifact changes (added/changed/removed). Suite-only narrative findings without artifact changes do not need a CHANGELOG entry — they live in the review-log only.

The session-isolation tradeoff is documented in [`suite-development.md`](suite-development.md) § "Session isolation" — suite reviews are typically run in-session with the suite's authorial context, with a `**Session note:**` line per entry naming the isolation status and sycophancy compensation.

---

## Reactivation triggers

Some gaps are deferred against named reactivation triggers (e.g., "after `issue-tracker-cli` completes"). The trigger conditions and the gaps they gate are documented in [`FINDINGS-INDEX.md`](FINDINGS-INDEX.md) § "Reactivation triggers". When a trigger fires, all gaps sharing it become eligible — they do not all need to be addressed in the same pass.

---

## Promoting project-scoped material to suite-level

Project-level review sessions sometimes produce findings whose substance generalizes — a closure protocol, a primer addition, a missing dimension. Per the "earned by recurrence" doctrine, suite-level promotion requires evidence the pattern recurs across projects. One project's recurrence is project-scope evidence; suite-level addition should wait until a second project independently encounters the pattern, OR until the project-scoped resolution is explicitly promoted under its own §promotion mechanism.

When a promotion happens, the project's CLOSURE-PROTOCOL.md (or equivalent) is moved to `../` or another suite-level location; the project's local copy becomes a stub pointing at the suite-level canonical version.

---

## Related references

- [`../README.md`](../README.md) — User-facing README. The "Suite scope" section there lists what the suite is for; this contributor README is the companion for *how the suite itself is maintained*.
- [`../CHANGELOG.md`](../CHANGELOG.md) — Suite changelog. Suite-development sessions that change artifacts add entries here.
- [`../primers/`](../primers/) — User-facing session primers (Phases 1a, 1b, 2a, 2b, 3, 4). The suite-development primer is intentionally located here in `suite-development/` rather than in `primers/`, because it is meta to the user-facing primer set.
