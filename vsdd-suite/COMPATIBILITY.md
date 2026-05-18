# VSDD Suite Compatibility Policy

The suite's compatibility commitment to projects that have adopted it.

**Reason this file exists:** registered as G-120 in [`suite-development/GAP-ANALYSIS-LOG.md`](suite-development/FINDINGS-INDEX.md) by Review 41 (Solution Architect lens). The forward-only constraint applied across G-88, G-89, G-91, G-92, G-93, G-95 IS the suite's compatibility policy — but it had been distributed across individual gap rows rather than documented as a coherent commitment. This file makes the policy explicit so a new user can verify the suite's backwards-compatibility posture without reading the gap registry.

---

## The policy in one sentence

**Forward-only compatibility:** suite changes apply to projects starting after the change date; projects whose first IAR run predates the change retain the pre-change shape and are not retroactively migrated.

## What this means in practice

- **Projects that adopted the suite before a given change** keep using the pre-change conventions. Their `iterative-adversarial-refinement/` subdirectories (legacy naming), single-file-per-domain review logs (legacy structure), or prior primer file names continue to be valid records. The suite does not require those projects to migrate.
- **Projects that adopt the suite after a given change** use the post-change conventions. New `vsdd-suite/` subdirectories, per-domain index + per-session-file review logs, and the current primer file names.
- **Both shapes coexist in the same portfolio.** The portfolio's pre-commit hook regex covers both shapes; primer/domain prompts are version-independent in their content (the conventions they reference adapt to the project's adopted-version state).

## Version anchors

| Version | Date | What changed |
|---|---|---|
| v0.4.0 | 2026-05-19 | Review 61 (G-149 closure): suite-development artifact rename to align with project-template naming. `SUITE-REVIEW-INDEX.md` → `SUITE-DEVELOPMENT-REVIEW.md`; `GAP-ANALYSIS-LOG.md` → `FINDINGS-INDEX.md`. Mechanical sed sweep updated all forward-facing internal cross-references; historical narrative files (review-log entries, CHANGELOG entries, prior version anchors here) preserve the old names in prose with markdown link targets pointing to new file paths. Suite can now serve as a worked example of its own template-defined naming (dogfood-correct). No breaking changes against this policy's project-facing surface — projects scaffolded from `templates/` already used the new names; the rename is internal to suite-development/. **Migration for projects with their own suite-side cross-references:** if a project's vsdd-suite/ tree links to `suite-development/SUITE-REVIEW-INDEX.md` or `suite-development/GAP-ANALYSIS-LOG.md` (rare — these are contributor-facing artifacts), update those links to the new names. |
| v0.3.0 | 2026-05-17 | Review 40 + 41 onboarding overhaul: README Prerequisites / Quickstart / "Bringing the suite into your project" sections; `templates/` directory; crosslink-contract.md; this COMPATIBILITY.md; AI-tool data-flow posture in Prerequisites. Forward-only — completed projects retain prior README and prior absence of templates. |
| v0.2.0 | 2026-05-17 | Review 38 + 39 post-ITC restructure: `iterative-adversarial-refinement/` → `vsdd-suite/` directory rename; `prompts/` → `primers/` with phase-prefixed filenames; suite-meta into `suite-development/`; README user/contributor split; `prompts/implementation.md` → `2a-red-gate.md` + `2b-implementation.md`; per-domain index + per-session file review-log structure (G-89). Forward-only — `bookmark-manager/` and `issue-tracker-cli/` retain their inner `iterative-adversarial-refinement/` paths and single-file-per-domain logs. |
| v0.1.0 | 2026-05-06 | Baseline at the close of Review 36 — IAR suite with full domain set, contributor governance (gap log, suite-review index, review-log/, suite-development.md primer), language and interface supplements, crosslink integration discussed but not yet first-class. Treated as the earliest tagged version because Review 36's anonymization-hook landing made the suite stable for portfolio use. |

Version tags v0.1.0, v0.2.0, v0.3.0, v0.4.0 are stub tags applied retroactively to anchor the forward-only policy to specific suite states. Future versions follow semantic versioning: PATCH for fix-only changes; MINOR for additive non-breaking changes; MAJOR for breaking changes against this compatibility policy.

## Breaking change definition for the suite

A change is **breaking against this compatibility policy** when:

- Existing project review logs become syntactically invalid against the suite's governing standard (e.g., a required field renamed or removed from the per-review entry preamble).
- Existing project artifact paths break — the renames in v0.2.0 are an example: `iterative-adversarial-refinement/` projects could not be expected to migrate; the suite kept the legacy path supported via the forward-only carve-out, so the change was NOT breaking against this policy.
- A primer's `## Prompt` text changes in a way that would invalidate review logs filed under the prior version (e.g., a finding classification removed from a domain's allowed schema while existing logs contain that classification).
- The contract file [`crosslink-contract.md`](crosslink-contract.md) experiences a documented crosslink breaking change that the suite cannot accommodate without breaking the worked example.

A change is **non-breaking** when:

- New domains, dimensions, supplements, or primers are added (additive).
- An existing prompt is reworded for clarity without changing the schema or classification universe.
- A documentation gap is closed (e.g., the Review 40 onboarding overhaul — adding sections to README without changing what projects under prior versions had to do).
- The renames in v0.2.0 — paired with a forward-only carve-out that preserves the prior shape.

The forward-only convention is the suite's primary mechanism for shipping structural changes without breaking compatibility: changes apply to future adopters; existing adopters keep working in the prior shape; the suite documentation acknowledges both shapes.

## Deprecation process

When a suite artifact must be deprecated (renamed, removed, restructured):

1. **Announce in CHANGELOG.md** under the relevant Unreleased section, naming the deprecated artifact and the replacement (or "no replacement; remove from new projects").
2. **Document the forward-only carve-out** in the relevant gap registry row, naming what existing projects retain.
3. **Update the suite's own artifacts** for the new shape; pre-existing project artifacts are not modified.
4. **Run a registry-walk suite review** when the deprecation completes, confirming the policy was applied consistently across all suite-side updates.
5. The deprecated artifact remains conceptually valid for completed projects indefinitely — there is no "remove old artifact" step. Forward-only compatibility means old conventions accumulate but never break.

## Cross-references

- [`suite-development/GAP-ANALYSIS-LOG.md`](suite-development/FINDINGS-INDEX.md) § Reactivation triggers — the bundled "after `issue-tracker-cli` completes" trigger was a coordinated deprecation event; G-88/G-89/G-91/G-92/G-93/G-95 closures all applied this policy.
- [`crosslink-contract.md`](crosslink-contract.md) — separate contract for the suite's dependency on crosslink.
- [`CHANGELOG.md`](CHANGELOG.md) — the per-release record that anchors version tags to specific change sets.
- [`domains/role/SOLUTION-ARCHITECT-REVIEW.md`](domains/role/SOLUTION-ARCHITECT-REVIEW.md) — SA Extended dims 14, 15, 16, 20 (breaking-change definition, versioning strategy, backward compatibility, deprecation process) are the dimensions this file applies to the suite.
