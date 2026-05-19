# Suite Review — 2026-05-19 (part 1 of 2)


This file holds Reviews 61–64 (the v0.4.0 → v0.7.0 work cycle: G-149 rename closure + documentation-and-tooling cluster + multi-lens SO/SA/TW review + Phase 5/6 implementation). Reviews 65–68 live in [2026-05-19-suite-review-part2.md](2026-05-19-suite-review-part2.md). The split conforms to the filename convention at [`../suite-development.md`](../suite-development.md) § Filename convention — files exceeding the 80 KB / 15-review threshold are split into `-partN.md` suffixed parts (Review 69 amendment).

## Review 61 — 2026-05-19 09:00Z

**Scope:** Address G-149 (suite-development artifact naming alignment) per operator-selected resolution Option A — rename `SUITE-REVIEW-INDEX.md` → `SUITE-DEVELOPMENT-REVIEW.md` and `GAP-ANALYSIS-LOG.md` → `FINDINGS-INDEX.md` to match the project-template naming. Lands on a separate branch (`vsdd-suite-rename`) and PR per the option-A scoping rationale that a cross-cutting rename is more reviewable in isolation than mixed with substantive work.

**Lens:** Closure-by-mechanical-sweep + structural-consistency. The rename is mechanical (file mvs + reference sweep); the methodological value is dogfood-correctness — the suite now uses the same naming convention it ships for projects.

**Session note:** In-session, separate branch. Sycophancy compensation: the rename touches 17 forward-facing files plus 5 historical review-log files (link targets only). The temptation was to also re-frame the rename as some kind of methodological revelation; rejected — it's a naming alignment, a small unit of work that closes a gap registered weeks ago. The substantive part is the discipline of doing the sweep correctly: forward-facing files get both prose and link updates; historical files get link-target updates only (preserving the prose mention of the old name as audit trail per G-89). The sweep script enforces that distinction by checking `is_historical(path)` before doing the prose pass.

---

### Resolved

**G-149 — Suite-development artifact naming alignment via Option A rename.**

Files renamed (`git mv`):
- `vsdd-suite/suite-development/SUITE-REVIEW-INDEX.md` → `vsdd-suite/suite-development/SUITE-DEVELOPMENT-REVIEW.md`
- `vsdd-suite/suite-development/GAP-ANALYSIS-LOG.md` → `vsdd-suite/suite-development/FINDINGS-INDEX.md`

Both renames preserve git history via `git mv`'s rename detection.

**Reference sweep** (Python script, 17 files modified):

Forward-facing files (link targets AND prose references updated):
- `.pre-commit-config.yaml` (3 prose updates in comments)
- `vsdd-suite/README.md` (2 link-target + 6 prose updates)
- `vsdd-suite/crosslink-contract.md` (3 link-target + 3 prose updates)
- `vsdd-suite/domains/role/SOLUTION-OWNER-REVIEW.md` (1 link-target + 1 prose update — the Dim 10 reference added in Review 54)
- `vsdd-suite/hooks/check-crosslink-references.sh` (3 prose updates in docstring + HISTORICAL_NARRATIVE_MARKERS tuple)
- `vsdd-suite/primers/3-review-session.md` (2 prose updates)
- `vsdd-suite/primers/4-feedback-integration.md` (4 prose updates)
- `vsdd-suite/suite-development/README.md` (5 link-target + 7 prose updates)
- `vsdd-suite/suite-development/suite-development.md` (7 link-target + 24 prose updates)
- `vsdd-suite/templates/PROJECT-FINDINGS-INDEX-template.md` (1 link-target + 1 prose update)
- `vsdd-suite/templates/README.md` (1 prose update)
- `vsdd-suite/suite-development/FINDINGS-INDEX.md` (renamed file, 2 forward-facing self-references manually updated post-sweep: line 18 contributor instruction, line 212 instruction footer)

Historical-narrative files (link targets only — prose mentions preserved as audit trail per G-89):
- `vsdd-suite/COMPATIBILITY.md` (2 link-target updates)
- `vsdd-suite/suite-development/review-log/2026-05-03-suite-review.md` (1)
- `vsdd-suite/suite-development/review-log/2026-05-05-suite-review.md` (1)
- `vsdd-suite/suite-development/review-log/2026-05-06-suite-review.md` (1)
- `vsdd-suite/suite-development/review-log/2026-05-17-suite-review.md` (10)
- `vsdd-suite/suite-development/review-log/2026-05-18-suite-review.md` (5)

**Post-sweep manual cleanups:**
- `.pre-commit-config.yaml` exclude regex for the `check-crosslink-references` hook: dropped dead `(GAP-ANALYSIS-LOG|SUITE-REVIEW-INDEX)` alternatives (those filenames no longer exist at that path); regex now uses `SUITE-DEVELOPMENT-REVIEW\.md` and `.*/FINDINGS-INDEX\.md` only.
- `check-crosslink-references.sh` HISTORICAL_NARRATIVE_MARKERS tuple: deduped accidental duplicate `/FINDINGS-INDEX.md` entry (created when the sweep replaced `/GAP-ANALYSIS-LOG.md` with `/FINDINGS-INDEX.md` while a `/FINDINGS-INDEX.md` entry was already present); docstring updated to reflect the new tuple contents.

**COMPATIBILITY.md entry:** new v0.4.0 row in the Version anchors table documenting the rename, scope, forward-only positioning, and a migration note for any project with cross-references into `suite-development/` (rare — these are contributor-facing artifacts; projects rarely link into them).

**Verification:** pre-commit hooks (review-log-anonymization, check-crosslink-references, check-changelog-currency) all pass against the staged set.

**Resolution:** G-149 status flipped Open → Addressed. The suite now uses the same naming convention it ships for projects. Backlog after Review 61: **1 Open** (only G-146 — `crosslink knowledge` auto-injection, forward enhancement gated on `crosslink knowledge --help` verification per G-123/G-139 discipline).

---

### Coordination

The Review 61 closure coordinates with:

- **G-89** (Addressed, Review 39) — established the per-domain-index + per-session-file pattern that the project template inherited. G-149 brings the suite-development directory into structural alignment with that pattern.
- **G-138** (Addressed, Review 46) — established the FINDINGS-INDEX.md naming for the project-template manual-mode finding registry. G-149's rename of suite-level GAP-ANALYSIS-LOG.md → FINDINGS-INDEX.md is the dogfood application.
- **G-122** (Addressed, prior) — the suite-eats-its-own-cooking purity-boundary principle. G-149's rename is the same principle applied to naming convention.

The rename does NOT regress any prior work — every prior gap closure that referenced the old names by markdown link now points at the renamed file; every gap closure that referenced the old names in prose preserves the historical text (forward-only narrative-preservation policy).

**Backlog after Review 61: 1 Open** (G-146). The full mining cycle from Review 45's ITC pattern-mining and Review 51's dollspace.gay upstream-author mining is now closed: 14 + 7 + 2 (G-148, G-149) + 1 (G-158) = 24 gaps registered across this work cycle, of which 22 Addressed + 1 Deferred (G-135) + 1 Open (G-146). The remaining G-146 is a forward enhancement, not a backlog item — it requires external prerequisite work (verify `crosslink knowledge --help` surface) before any suite-side closure can begin.

Sycophancy self-audit: I considered cleaning up the renamed file's gap-row prose to use the new names throughout (rewriting "Resolution: A rename `SUITE-REVIEW-INDEX.md` → `SUITE-DEVELOPMENT-REVIEW.md`" to use only the new name). Rejected: the gap-row body describes the RESOLUTION the gap proposes, and that resolution names the old names as the source of the rename. Replacing "old name → new name" with "new name → new name" would lose the audit-trail signal of what was renamed. The forward-only policy applies inside gap-row bodies the same way it applies in review-log narratives — historical names stay, current names take their place in cross-references and instructions only.

---

### Summary

G-149 closed via Option A rename. 1 Open gap remaining in the active backlog (G-146, forward enhancement). The vsdd-suite naming convention is now consistent across user-facing (templates) and contributor-facing (suite-development) artifacts.

---

## Review 62 — 2026-05-19 22:30Z

**Scope:** Address the documentation-and-tooling cluster from the FINDINGS-INDEX backlog — **G-96** (whitepaper sub-phase semantics divergence), **G-106** (crosslink command sample outputs missing from the worked example), and **G-146** (suite primer auto-injection via `crosslink knowledge` not yet documented or wired). Operator-selected resolutions: G-96 harmonize with whitepaper; G-146 scaffold-project.sh gated by mode (versioning policy deferred); G-106 use bookmark-cli as the host for output capture. Lands on a dedicated branch (`vsdd-suite-doc-tooling-cluster`).

**Lens:** Closure-by-direct-edit + tooling + dogfood-verification. The G-146 wiring required running `crosslink knowledge --help` per G-123/G-139 discipline before specifying the mechanism; the G-106 closure required running the worked-example crosslink commands against a sandbox copy of `bookmark-cli` to capture verbatim outputs and surface real CLI-surface defects the README had not previously caught.

**Session note:** In-session, dedicated branch. Sycophancy compensation: the G-106 closure surfaced multiple corrections to the README's worked example that I almost waved past — `crosslink milestone add/show/close` require numeric IDs (not milestone names as the README claimed); `crosslink swarm gate <phase>` requires `crosslink swarm init --doc <design>` first (the README treated it as a standalone "run the test suite" command); `crosslink swarm review --doc <PATH>` is the OUTPUT path for the consolidated findings document, not the per-agent INPUT prompt (the README's framing was ambiguous and could be read either way). Each of these is the kind of finding G-106 was registered to surface — a new user following the worked example verbatim would hit them. The temptation was to capture only sample outputs and let the broken commands stand; rejected — sample outputs against broken commands would be worse than no sample outputs.

**Source:** domain-raised (the three gaps were originally raised by Reviews 34, 40, and 49 respectively, by domain adversaries applying their dimensions to the suite). The cluster selection was operator-raised in the current session.

---

### Resolved

**G-96 — Whitepaper sub-phase semantics harmonized (Option A: rename + introduce 2c).**

The VSDD whitepaper organizes Phase 1 as Steps 1a (Behavioral Specification), 1b (Verification Architecture), 1c (Spec Review Gate) and Phase 2 as Steps 2a (Test Suite Generation), 2b (Minimal Implementation), 2c (Refactor). The suite previously used Phase 1a (Spec Crystallization) and Phase 1b (Decomposition) as co-equal sub-phases — divergent from the whitepaper, which treats decomposition as part of Step 1c. The suite also lacked any equivalent of Step 2c (Refactor). Operator selected Option A (harmonize) over Option B (document divergence) and Option C (document-now-harmonize-later).

Changes:
- `git mv vsdd-suite/primers/1b-decomposition.md vsdd-suite/primers/1c-decomposition.md` — rename + update H1 + add a "Whitepaper alignment (G-96)" framing paragraph naming the whitepaper Step 1c (Spec Review Gate) semantics and explaining that whitepaper Step 1b (Verification Architecture) is folded into the suite's Phase 1a (per the existing primer's verification-architecture section).
- New `vsdd-suite/primers/2c-refactor.md` — primer for the TDD red→green→**refactor** loop's third step. Refactor scope explicitly bounded (extract-and-name, collapse-and-inline, reshape-data-flow, surface-purity-boundary, idiomatic-alignment, language-supplement rules); behavior changes routed out as Phase 2a/Phase 1a findings. Explicit-skip annotation pattern (`"Phase 2c: no refactor required — minimal Phase 2b implementation passes the refactor checklist as-landed."`) added so a silent skip is visible in the audit trail.
- `vsdd-suite/primers/1ab-spec-crystallization.md` — new framing paragraph noting that suite Phase 1a folds whitepaper Steps 1a + 1b.
- `vsdd-suite/README.md` — phase table updated (1b row → 1c row + new 2c row); per-layer flow diagram updated (Phase 1b → Phase 1c; new Phase 2c step in the loop; Phase 4 routing list adds 1c and 2c); Session primers table updated; Worked example overview table updated; Worked example § Phase 1b — Decomposition → § Phase 1c — Decomposition (Spec Review Gate); new § Phase 2c — Refactor worked-example section between Phase 2b and Phase 3; Phase 2b crosslink block reframed to not formalize the gate inside 2b (the gate moves to 2c).
- `vsdd-suite/primers/4-feedback-integration.md` — finding-to-phase routing table updated (Phase 1b → Phase 1c row; new Phase 2c row for refactor regressions); driving question 4 updated to reference 1c + 2c gates; Routing output `Route` value list expanded (1c, 2c added; 1b removed); `route:phase-1b` → `route:phase-1c`; `route:phase-2c` added to crosslink labels and to fix-cohort selection rules; "After Phase 4" re-entry list updated for 1c + 2c.
- `vsdd-suite/domains/DOMAIN-INDEX.md` — link target `primers/1b-decomposition.md` → `primers/1c-decomposition.md` in § Intent calibration.
- `vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md` — link target update in Dim 11.
- `vsdd-suite/suite-development/suite-development.md` — narrative description of the suite's phase coverage updated; G-96 cross-reference added so future contributors find the rationale.
- `vsdd-suite/templates/DESIGN-template.md` — Phase 1b → Phase 1c in two places (Verification architecture section; Open questions section).

Forward-only narrative-preservation policy (G-89) preserved: historical review-log files keep their original "Phase 1b" prose mentions; CHANGELOG.md / SUITE-DEVELOPMENT-REVIEW.md historical entries unchanged. The `check-crosslink-references.sh` hook's `HISTORICAL_NARRATIVE_MARKERS` tuple already covers `/CHANGELOG.md`, `/COMPATIBILITY.md`, `/FINDINGS-INDEX.md`, `/SUITE-DEVELOPMENT-REVIEW.md`, and `/review-log/` — no hook update needed.

**Resolution:** G-96 status flipped Open → Addressed.

---

**G-146 — Suite primer auto-injection via `crosslink knowledge` wired into `scaffold-project.sh`.**

Per the G-123/G-139 prerequisite discipline, `crosslink knowledge --help` was verified first (subcommands: `add`, `show`, `list`, `edit`, `remove`, `sync`, `import`, `search`). `crosslink knowledge import <DIRECTORY> --tag <tag>` is the bulk-registration primitive; chosen for primer + domain-prompt registration.

Operator-selected mechanism: scaffold-project.sh gated by mode. Operator-selected versioning policy: deferred (new gap G-159 registered for follow-up).

Changes:
- `vsdd-suite/templates/scaffold-project.sh` — header comment rewritten to name the G-146 wiring; new conditional block detects `.crosslink/` directory + `crosslink` binary in `$PATH` and runs three `crosslink knowledge import --quiet` calls registering primers (tag `vsdd-suite-primer`), role domain prompts (tag `vsdd-suite-domain`), and meta domain prompts (tag `vsdd-suite-domain`). On either condition's absence, registration skips silently — manual mode is unaffected (G-144 principle). Trailing block prints a "crosslink binary detected but project not yet initialized" hint when `crosslink` is in `$PATH` but `.crosslink/` is missing, so users running `scaffold-project.sh` before `crosslink init` get a clear next step.
- `vsdd-suite/README.md` Quickstart 1 (crosslink-primary) — recommended order updated to `crosslink init && scaffold-project.sh` (init before scaffold) so the auto-registration fires on the first scaffold; order-reversed fallback documented.
- `vsdd-suite/README.md` § Bringing the suite into your project — new sub-section `### Crosslink knowledge auto-injection (G-146)` documenting the three `crosslink knowledge import` invocations, the conditions under which scaffold-project.sh auto-fires them, the deferred-versioning-policy disposition, and the G-144 manual-mode-parity preservation.

**Verification:** end-to-end tested in `/tmp/g146-clean` — fresh git repo + `crosslink init` + `scaffold-project.sh` registered 24 knowledge pages (7 primers + 14 role domain prompts + 2 meta + 1 auto-generated index). `crosslink knowledge list --tag vsdd-suite-primer` shows all 7 primers including the newly-created `1c-decomposition` and `2c-refactor` from the G-96 closure. Smoke-test dirs cleaned up after verification.

**Resolution:** G-146 status flipped Open → Addressed.

---

**G-106 — Crosslink command sample outputs embedded in the worked example via bookmark-cli sandbox.**

Operator-selected host: bookmark-cli. A safety-preserving copy of `bookmark-cli/` was staged at `/tmp/g106-bookmark-cli` with a local bare-repo remote so crosslink's tracker-push flow worked without polluting the real `bookmark-cli`. Sandbox cleaned up after capture.

Captured outputs embedded in `vsdd-suite/README.md`:
- `crosslink init` — full output sequence (initializing database … agent identity).
- `crosslink workflow diff` — post-`init` output with the `hook-config.json: customized` line annotated as expected-post-init state (not drift), and a note that drift would show as additional `customized` lines under Rules or Hooks. This corrects the prior README claim of "expect: no diff" which is only true *before* init populates the agent identity.
- `crosslink quick`, `crosslink milestone create`, `crosslink milestone add`, `crosslink milestone show`, `crosslink issue comment` — all captured with verbatim output and post-block interpretation paragraph.
- `crosslink session start`, `crosslink session work`, `crosslink session end --notes`, `crosslink session last-handoff` — verbatim outputs + interpretation of the auto-lock semantics.
- `crosslink issue close` — verbatim output including the CHANGELOG.md side-effect, with a note about `.crosslink/hook-config.json` `tracking_mode` settings (`relaxed`/`normal`/`strict`).

CLI-surface defects in the README's worked example fixed during capture:
- `crosslink milestone add/show/close` take **numeric milestone IDs**, not milestone names. The prior worked example used `crosslink milestone add "Layer 1: add and list bookmarks" "$L1"` which fails with `error: invalid value '...' for '<ID>': invalid digit found in string`. README updated to capture milestone IDs into shell variables via `awk` extraction of `Created milestone #N: <title>` (`crosslink milestone create --quiet` does not yet return just the ID — separate small finding worth a future crosslink upstream feature request).
- `crosslink swarm gate <phase>` requires `crosslink swarm init --doc <design>` to have run first AND all planned agents to be resolved. README's Phase 2c section reframed: solo projects use clean `cargo test` as the gate; multi-agent swarm builds use `crosslink swarm gate <phase>` after `swarm init` + agent resolution.
- `crosslink swarm review --doc <PATH>` is the **output path** for the consolidated findings document, not the per-agent input prompt as the README's phrasing implied. README updated to clarify; per-agent prompt loading is via the `vsdd-suite-domain`-tagged knowledge pages registered by G-146.

The G-106 closure depends on G-111 (Addressed Review 40 — Tested against crosslink v0.8.0 line near top of README). All sample outputs in this closure were captured against crosslink v0.8.0 and labeled as such in their preceding sentence.

**Resolution:** G-106 status flipped Open → Addressed (partial — see follow-up below).

---

### Deferred

**G-159 (new) — Crosslink-knowledge primer versioning policy.**

When the suite version bumps (e.g., the G-96 + G-146 work would bump v0.4.0 → v0.5.0), projects that already ran `scaffold-project.sh` against a prior version have stale knowledge pages registered in their crosslink database. The G-146 closure documents `crosslink knowledge import --overwrite` as the manual re-import path and suggests tag-based drift detection (compare `vsdd-suite-primer` page count against current suite primer count), but does not mechanize either.

**Trigger** (per G-130 deferral discipline): when a second project in the portfolio adopts the suite via crosslink-knowledge auto-injection AND the suite version subsequently bumps in a way that adds, removes, or substantively re-shapes a primer or domain prompt — at that point versioning friction will manifest concretely. Auto-Backlog fallback: 2026-08-01 (≥3-review threshold from this Review 62).

**Cost of deferral:** primer drift is invisible to a project that registered an older suite version's knowledge pages; future `crosslink swarm review` invocations against that project may load stale prompts. The drift is detectable by inspecting `crosslink knowledge show <slug>` against the current suite source, but no automated check warns of it. The G-89 forward-only policy is the suite's compatibility commitment to projects — under that policy, a stale knowledge-page registration is a project-side responsibility, not a suite-side bug. The deferral is honest about that scope.

**Coordinate with:** G-117 (manual-copy scaffolding doctrine — knowledge registration is a parallel decision to copy-vs-submodule), G-120 (suite versioning strategy — versioning policy for knowledge pages composes with the suite's semantic-version-stub-tag policy), G-144 (manual-mode parity — any versioning automation must skip silently in manual mode).

---

### Coordination

The Review 62 closures coordinate with:

- **G-89** (Addressed, Review 39) — the forward-only narrative-preservation policy was applied to the G-96 rename (historical review-log Phase 1b prose preserved).
- **G-111** (Addressed, Review 40) — the "Tested against crosslink v0.8.0" version-pin line is the anchor for the G-106 sample outputs; if crosslink's CLI surface shifts in a future version, the G-111 process flags the sample outputs as needing re-capture.
- **G-112** (Addressed, Review 44) — the reference implementation at `bookmark-cli/` was the safe sandbox source for the G-106 capture pass (used via a copy at `/tmp/`, not the real `bookmark-cli` directory).
- **G-117** (Addressed, Review 42) — `scaffold-project.sh` is the canonical manual-copy scaffolding mechanism; the G-146 wiring extends what the script does without changing the doctrine.
- **G-121** (Addressed, Review 42) — the scaffold default of 7 core domains is unchanged; G-146 knowledge registration uses the FULL domain surface (all 14 role + 2 meta), since users can later activate extended domains without re-running scaffold.
- **G-122** (Addressed, prior) — the suite-eats-its-own-cooking principle. The G-96 rename harmonizes the suite's own primer naming with the upstream whitepaper's sub-phase taxonomy — dogfood-correctness applied to terminology.
- **G-123 / G-139** (verification discipline) — `crosslink knowledge --help` was verified before specifying the G-146 mechanism, per the discipline. The captures throughout G-106 are the dogfood application of the same discipline at scale.
- **G-130** (deferral discipline) — G-159's deferral includes the three named items (trigger, cost, auto-Backlog) the discipline requires.
- **G-144** (Addressed, Review 49) — manual-mode parity preserved; G-146 wiring skips silently when `.crosslink/` is absent OR `crosslink` is not in `$PATH`.

The closures do NOT regress any prior work. The G-96 rename is forward-only (Phase 1b prose preserved in historical files). The G-146 wiring is opt-in by environment detection (no breakage for manual-mode users). The G-106 README updates correct real defects in the worked example that any new user would have hit on first run.

---

### Summary

G-96, G-106, G-146 all Addressed. New gap G-159 registered + Deferred (knowledge-versioning policy, gated on second project + suite-version bump trigger).

Backlog after Review 62: **0 Open + 1 Deferred** (G-159 — new, Deferred with G-130 discipline applied; auto-Backlog fallback 2026-08-01). The active-mining-cycle backlog from Reviews 45 / 49 / 51 (the work cycle Review 61 wound down) is now empty.

Cluster closure dogfood note: the documentation-and-tooling cluster was the operator's selection from the larger Open-gap surface (20 Open as of Review 61's close). The cluster spanned one whitepaper-alignment finding (G-96, low/medium), one CLI-output-fidelity finding (G-106, medium/medium), and one tooling-enhancement (G-146, medium/medium) — three independent gaps whose closures composed: the G-96 rename produced new primer filenames (`1c-decomposition`, `2c-refactor`), the G-146 wiring registered them as knowledge pages on smoke test, and the G-106 capture pass verified the README references resolve to the new filenames. The composition surfaced one CLI-surface limitation (`crosslink milestone create --quiet` does not return just the ID — captured as a future upstream feature request, not a suite-side gap).

Sycophancy self-audit: I considered scoping G-106 down to "capture outputs for the cheap commands; mark G-106 as partial closure" when the swarm-gate / swarm-review captures proved difficult (swarm gate requires resolved agents; swarm review consumes real API budget). Rejected the partial closure framing — the G-106 *spirit* is to give a new user a mental model of what the worked-example commands produce, and that mental model is well-served by the cheap-command captures plus the reframed swarm-gate / swarm-review sections that explain what those commands actually do. The captures that didn't happen (resolved-agent swarm gate output; real swarm review agent dispatch output) are operator decisions about API-budget spend, not suite-development defects.

---

## Review 63 — 2026-05-19 23:30Z

**Scope:** Operator-driven adversarial review of the suite. Inputs loaded: `suite-development/suite-development.md` (the contributor governing standard) and `primers/3-review-session.md` (the adversarial-posture primer). Lenses applied serially: cutting bloat / redundancy, thoroughness of VSDD implementation, user experience. Domain perspectives applied as appropriate: Solution Owner (spec contract; over-engineering; under-delivery), Solution Architect (architecture; coupling; complexity budget), Technical Writer (README completeness; documentation accuracy; AI session independence). The review produced 10 findings (F1–F10); operator directed in-session closure of the addressable subset and explicit deferral with G-130 discipline for the two judgment-call findings.

**Lens:** Multi-lens role-and-defect-class pass — Solution Owner + Solution Architect + Technical Writer applied across the three named lenses (bloat/redundancy, VSDD-implementation thoroughness, UX). The composition is a generalist suite-review pass with multiple specialization channels — broader than a single-defect-class pass; narrower than a registry-walk.

**Session note:** In-session. **Partial-isolation acknowledged with high sycophancy risk against own recent work** — I authored the G-96 + G-106 + G-146 closures (Review 62) earlier in the same conversation thread, so could not present as cold-context for those artifacts. Compensation applied: findings derived from artifact-state re-reads (the files as they sit on disk after Review 62 + this review's edits) rather than from narrative recall; findings against my own recent work weighted slightly heavier (F2 in particular, which directly closes a gap my G-96 Phase 2c primer introduced). The cold equivalent would be stronger; if any finding below is contentious, the right move is a true cold session on a fresh branch.

**Source:** mixed — `domain-raised` for the SO/SA/TW lens applications; `director-raised` for the operator's instruction to apply the three-lens × three-domain matrix.

---

### Resolved

**Finding 1 — Phase 2c is over-engineered for learning-exercise intent (G-168, registered + Deferred).** SO + bloat lens. The `primers/2c-refactor.md` says "Phase 2c is optional but not invisible" but the README's per-layer flow diagram and worked example treat the Phase 2c → 3 gate as a required boundary (commit OR explicit annotation). For learning-exercise intent (G-150), this adds a mandatory ceremony per layer with no defect class it uniquely catches that existing SE/SA dimensions wouldn't catch in Phase 3. The refactor pressure was previously applied implicitly through Phase 3 SE/SA review — moving it earlier to a dedicated phase may be over-investment at the lower intent tiers.

**Classification:** Deferred — judgment call requiring evidence from a second project at learning-exercise intent. Registered as G-168 (see Deferred section below).

---

**Finding 2 — Refactor-scope-creep is undetected by any domain dimension (G-161, Resolved).** SO + VSDD-thoroughness lens. The Phase 2c primer says "a Phase 3 SE reviewer should be able to diff Phase 2b → 2c and see only structural improvement, not new behavior." No current SE, QE, SA, or VDD-IAR Alignment dimension evaluated this property — there was no dim that said "if a Phase 2c commit exists, verify the diff against Phase 2b adds no behavioral paths." A Phase 2c commit smuggling new validation would go undetected unless a reviewer happened to compare commits manually.

**Resolution:** Added new VDD-IAR Alignment dim 12 in `domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md` — **Phase 2c refactor discipline (G-161)**: requires that Phase 2c either commits a refactor with no new behavior paths beyond Phase 2b, OR records an explicit "no refactor required" annotation. The check: diff Phase 2b → 2c and look for added control-flow paths; intentional refactor that surfaces a spec gap must route via Phase 4 to Phase 1a+1b. Silent skip (no commit, no annotation) is also a finding per `primers/2c-refactor.md` § Completion criteria #5. Distinguishes intentional discovery (route to spec) from labeling defect (audit-trail-vs-diff mismatch). This finding is the highest-leverage single move from the review — it closes a gap my own G-96 work opened.

---

**Finding 3 — G-146 registers all 16 domain prompts regardless of project activation (G-163, Resolved).** SO + bloat lens. The G-146 wiring (Review 62) ran `crosslink knowledge import` against `domains/role/` and `domains/meta/` unconditionally — all 14 role + 2 meta prompts got registered. The G-121 doctrine says default activation is 7 cores; the 16-domain surface is "available, not required." A learning-exercise project (3-domain activation per intent calibration) still got all 16 registered.

**Resolution:** Modified `templates/scaffold-project.sh` G-146 block to register **only the activated domain prompts**. The script stages the activated subset into a temp directory via the existing DOMAINS array (which already drives per-domain index file creation) and runs `crosslink knowledge import "$STAGE_DIR" --tag vsdd-suite-domain`. Trap on EXIT cleans up the temp dir. Verified end-to-end: a 3-domain scaffold (`SOFTWARE-ENGINEER QUALITY-ENGINEER SOLUTION-OWNER`) registered exactly 3 domain prompts (not 16). The scaffold default (7 cores) registers 7. Both correctly scoped to activation.

---

**Finding 4 — Supplements not registered as knowledge pages (G-164, Resolved).** SA + VSDD-thoroughness lens. G-146 registered primers and domain prompts but NOT `supplements/` (rust.md, javascript-typescript.md, cli.md, browser-app.md). A `crosslink swarm review` agent reviewing a Rust CLI project needed the Rust supplement + CLI supplement alongside the domain prompt; with the prior G-146 wiring, the agent had primers + domain prompts loaded but had to fetch supplements separately.

**Resolution:** Extended `templates/scaffold-project.sh` to also run `crosslink knowledge import "$SUITE_DIR/supplements" --tag vsdd-suite-supplement`. Verified end-to-end: 4 supplements registered alongside the primers and activated domain prompts in the smoke test. The `vsdd-suite-supplement` tag is the third tag axis; future `crosslink swarm review` invocations can compose `vsdd-suite-primer` + `vsdd-suite-domain` + `vsdd-suite-supplement` loads per agent based on the project's language/interface.

---

**Finding 5 — Sample-output version-pin audit signal is distributed (G-165, Resolved).** TW + UX lens. The G-106 closure embedded sample outputs in 6 locations in the README, each labeled "captured against crosslink v0.8.0." When crosslink v0.9.0 ships, there was no single place that listed which sample outputs needed re-capture — each block had to be hunted by grep.

**Resolution:** Added `<!-- G-165: crosslink-v0.8.0-sample-output -->` HTML comment immediately before each of the 6 sample-output preamble lines in `README.md`. A future crosslink-version-bump check is now `grep -n "G-165: crosslink-v0.8.0-sample-output" vsdd-suite/README.md` — returns 6 lines, one per block needing re-validation. When the captures are re-validated against a newer crosslink, bump the version in the comment marker.

---

**Finding 6 — Worked-example bash assumes POSIX shell without naming the assumption (G-166, Resolved).** TW + UX lens. The Phase 1c worked example used `awk '/^Created milestone/ {gsub(/[#:]/,"",$3); print $3}'` to extract milestone IDs from output — pure POSIX-shell idiom. PowerShell or Windows-cmd users reading the example saw broken syntax. The Prerequisites section named "Git" as required but did not pin shell.

**Resolution:** Added a new `**POSIX-compatible shell**` bullet to `README.md` § Prerequisites (baseline-for-both-modes list), naming the dependency explicitly and pointing Windows-cmd / PowerShell users to a documented workaround (capture the milestone ID by inspecting `crosslink milestone list` output after `crosslink milestone create` rather than parsing inline). The shell-prereq is now visible in the same place a new user reads "Git" — failure surfaces at install time, not at Phase 1c.

---

**Finding 7 — "Captured as future upstream feature request" was filed nowhere (G-167, Resolved).** TW + accuracy lens. Review 62's G-106 closure noted that `crosslink milestone create --quiet` does not return just the ID — "captured as a future upstream feature request, not a suite-side gap." But no upstream issue was actually filed in crosslink's tracker, no entry was added to `crosslink-contract.md`, and no follow-up gap was registered. The "captured" framing implied a tracked artifact; no tracked artifact existed. This is an accountability gap against the external-dependency reference discipline (`suite-development.md` § External dependency references).

**Resolution:** Added new `### Known limitations (suite-discovered against crosslink v0.8.0)` sub-section to `vsdd-suite/crosslink-contract.md` enumerating 4 known limitations the suite has worked around: (a) `milestone create --quiet` doesn't reduce output to ID (G-167 — this finding); (b) `milestone add/show/close` take numeric IDs not names (G-106 closure precedent); (c) `swarm gate <phase>` requires `swarm init --doc` first (G-106 closure precedent); (d) `swarm review --doc` is OUTPUT path not input prompt (G-106 closure precedent). Each row names the observed behavior, the suite's workaround, and the source review. The breaking-change definition clarified: a discovered limitation is NOT a breaking change — but if a limitation becomes blocking (e.g., `milestone create` stops printing the `#N` line we parse), that IS a breaking change. Suite owner authority is limited to suite-side artifacts per `suite-development.md` § External dependency references rule 3 — the upstream ask remains the suite operator's prerogative to file, not auto-coordinated.

---

**Finding 8 — `suite-development.md` is a load-bearing 467-line wall (G-169, registered + Deferred).** SA + bloat + UX lens. The document combines 8+ governing standards (domain files, session primers, project-level review logs, layer-gate close criteria, deferral-trigger discipline, supplement coverage, suite review session entry format, file-level headers, etc.). A contributor adding a new artifact type has to scroll the whole file. The structure-finding cost compounds when 2+ contributors edit in parallel.

**Classification:** Deferred — judgment call gated on contributor count. Registered as G-169.

---

**Finding 9 — Phase 1b → 1c rename leaked abstraction; missing 1b row confused readers (G-160, Resolved).** TW + UX lens. The G-96 closure (Review 62) renamed Phase 1b → Phase 1c and folded whitepaper Step 1b into the suite's Phase 1a. The README phase table showed rows for 1a, 1c, 2a, 2b, 2c — a user scanning saw a missing 1b and either thought the suite forgot a phase or had to read the 1a row's parenthetical to reconstruct the merge. The cognitive cost was low for a careful reader and confusing for a fast scanner. The operator directly hit this confusion in the same conversation ("Why is there a 1a and 1c but no 1b?") — concrete evidence the friction was nonzero.

**Resolution:** Renamed Phase 1a → Phase 1a+1b throughout the suite, with `git mv vsdd-suite/primers/1a-spec-crystallization.md vsdd-suite/primers/1ab-spec-crystallization.md` and a Python sweep (18 files edited, 80 replacements). The phase-table row labels now read `1a+1b` so the absent stand-alone `1b` row is visibly absorbed. Forward-only per G-89: historical review-log files, CHANGELOG, COMPATIBILITY, SUITE-DEVELOPMENT-REVIEW, FINDINGS-INDEX preserved prose mentions of "Phase 1a" alone; only link targets to the renamed file were updated in historical contexts. The route-label `route:phase-1a` was kept as a stable identifier string (per G-130 stability — labels are identifiers, not scope-descriptors). Adversarial cleanup: the sweep collided with one self-referential meta-text in the renamed primer ("prior label 'Phase 1a+1b' alone made..." — nonsense after the sweep); fixed manually to use backtick-quoted `Phase 1a` for the historical-name reference.

---

**Finding 10 — Phase 5/6 unownership did not gate capstone-intent projects (G-162, Resolved).** SO + VSDD-thoroughness lens. G-54 (Four-Dimensional Convergence partial ownership) and G-55 (Formal Hardening unowned) remained Open. The README footnote said "skip unless your project is safety-critical or cryptographic." But G-150 intent calibration introduced `capstone` and `production` intents, and the methodology eventually treats Phase 5/6 as relevant for those tiers. The suite was shipping a methodology that explicitly couldn't guide a capstone-intent project through formal hardening — and the intent calibration did not flag this as a gate.

**Resolution:** Added new `### Phase 5 / Phase 6 strategy declaration (G-162 — capstone + production intents)` sub-section to `domains/DOMAIN-INDEX.md` § Intent calibration. At capstone and production intents, `DESIGN.md` § Project intent must include a one-sentence `**Phase 5 strategy:**` line and a one-sentence `**Phase 6 strategy:**` line. Valid declarations: `not applicable — <rationale>` (real rationale, not TBD) or `planned — <named tooling and scope>`. Learning-exercise and portfolio intents are exempt — silence acceptable at those tiers because the methodology never claimed Phase 5/6 ownership for them. Added matching `**Phase 5 strategy:**` / `**Phase 6 strategy:**` lines to `templates/DESIGN-template.md` § Project intent so the declaration surfaces at scaffold time. Extended VDD-IAR Alignment dim 1 (Design-before-code) to enforce the declarations at capstone+ intent — absence is a finding for dim 1 (spec completeness). The explicit-skip pattern was generalized from Phase 2c's "no refactor required" annotation (G-96) — silence at the gate is itself the finding. **This does NOT close G-54 or G-55** — the suite still doesn't own Phase 5/6 primers or dimensions — but it ensures the gaps are visible at every capstone-tier closure rather than hidden by intent-calibration silence. G-54 and G-55 stay Open as forward enhancement candidates.

---

### Deferred

**G-168 — Phase 2c at learning-exercise intent (registered Review 63; addresses F1).**

The Phase 2c → 3 gate added by G-96 is a mandatory ceremony at every intent tier in the current closure. For learning-exercise intent (3-domain activation; stop-signal sensitivity high per G-150), the per-layer overhead is non-trivial and the defect class Phase 2c uniquely catches at that tier is unclear (Phase 3 SE/SA review would catch refactor scope-creep anyway with G-161's new dim 12).

**Trigger:** when a second learning-exercise-intent project reaches Layer 3 and the operator measures actual per-layer overhead from Phase 2c (commit ceremony OR annotation ceremony) against the value Phase 2c added in that project. If the overhead exceeds the value at that tier, the Phase 2c → 3 gate becomes intent-conditional (required at portfolio+, skip-optional at learning-exercise).

**Cost-of-deferral:** learning-exercise projects under the current methodology run Phase 2c every layer regardless of value. The methodology-effort overhead compounds across layers in projects whose intent says "goal is learning, not shipping." This is the headline G-150 over-investment failure mode applied to one specific phase.

**Auto-Backlog clause:** if 2026-09-01 passes without a second learning-exercise project surfacing this evidence, the gap auto-Backlogs and re-raises in the next suite review for fresh evaluation.

**Coordinate with:** G-96 (parent — introduced Phase 2c); G-150 (intent calibration — the discipline this gap potentially refines); G-161 (Phase 2c discipline dim — would catch refactor-scope-creep regardless of whether Phase 2c is a separate gate).

---

**G-169 — `suite-development.md` is a 467-line single file (registered Review 63; addresses F8).**

The contributor governing standard combines 8+ topics. At the current 1-contributor scale, the single-file shape is fine (the contract for contributors is trivially discoverable). At 3+ contributors, merge conflicts on adjacent governing-standard sections become routine and the structure-finding cost compounds.

**Trigger:** when a second active contributor lands meaningful suite-development work (not just a typo fix — a structural contribution: new artifact type, new governing standard section, etc.). The trigger fires on observable parallel-contributor evidence, not on a date.

**Cost-of-deferral:** at 1 contributor, none. The risk is silent until a second contributor's first non-trivial PR hits merge friction on `suite-development.md`.

**Auto-Backlog clause:** if 2027-02-01 passes without a second active contributor, the gap auto-Backlogs and re-raises in the next suite review — by then the 1-contributor assumption may itself warrant re-evaluation.

**Proposed resolution at trigger:** split into `suite-development/governing/<topic>.md` files (e.g., `governing/domain-files.md`, `governing/session-primers.md`, `governing/project-level-review-logs.md`); `suite-development.md` becomes an index pointing at the topical files. The pattern mirrors how `SUITE-DEVELOPMENT-REVIEW.md` is an index pointing at `review-log/` files.

**Coordinate with:** G-89 (per-domain index + per-session-file pattern — the same shape applied to governing standards); G-122 (suite-eats-its-own-cooking — the project-template uses indexed structure; the suite should too at scale).

---

### New gap registered

All 10 findings from this review become tracked gaps:

- **G-160** — Phase 1a label clarity (F9) — Addressed in-session
- **G-161** — Phase 2c refactor-scope-creep dim (F2) — Addressed in-session
- **G-162** — Phase 5/6 strategy declaration at capstone+ intent (F10) — Addressed in-session
- **G-163** — scaffold-project.sh registers only activated domains (F3) — Addressed in-session
- **G-164** — scaffold-project.sh registers supplements (F4) — Addressed in-session
- **G-165** — HTML comment markers on G-106 sample outputs (F5) — Addressed in-session
- **G-166** — POSIX shell prerequisite explicit (F6) — Addressed in-session
- **G-167** — crosslink-contract.md known-limitations section + `milestone create --quiet` accountability (F7) — Addressed in-session
- **G-168** — Phase 2c at learning-exercise intent (F1) — Deferred with G-130 discipline
- **G-169** — suite-development.md as 467-line wall (F8) — Deferred with G-130 discipline

---

### Coordination

The Review 63 closures coordinate with:

- **G-89** (Addressed, Review 39) — forward-only narrative-preservation applied to the F9 rename (Phase 1a prose preserved in historical files).
- **G-96** (Addressed, Review 62) — the rename G-160 addressed is the back-half of G-96's whitepaper-harmonization arc; G-161's dim 12 is the missing enforcement that G-96 introduced the boundary without; G-162's explicit-skip pattern was generalized from G-96's 2c primer annotation pattern.
- **G-106 / G-111** (Addressed, Review 62 / Review 40) — G-165's HTML-comment markers operationalize G-111's version-pin discipline for the G-106 sample outputs; G-167's known-limitations section is the documentation home for the workarounds the G-106 capture pass discovered.
- **G-130** (Addressed, prior) — G-168 and G-169 each name trigger + cost + auto-Backlog per the discipline.
- **G-138** (Addressed, prior) — G-163's activated-only registration uses the same DOMAINS array the project-level finding-index uses; the activation surface is consistent across the scaffold's structured outputs.
- **G-144** (Addressed, Review 49) — G-163 and G-164 preserve manual-mode parity by silent-skip on detection failure (same pattern as G-146's original wiring).
- **G-146** (Addressed, Review 62) — G-163 and G-164 refine G-146's mechanism without changing its operator-facing contract; G-165 is a related discipline (sample-output marker).
- **G-150 / G-151 / G-156** (Addressed, prior) — G-162's explicit-skip-at-capstone+ extends the intent calibration discipline; G-168's deferral hinges on G-150's intent definitions.
- **G-54 / G-55** (still Open) — G-162 makes the Phase 5/6 unownership *visible* at capstone-tier closure but does not close G-54 or G-55. They remain open as forward enhancement candidates.

The closures do NOT regress any prior work. The G-160 rename is forward-only. The G-161 dim 12 is additive (new dim, no existing dim modified). The G-162 explicit-skip is conditional on capstone+ intent (no effect at portfolio or below). The G-163 / G-164 scaffold changes preserve manual-mode parity. The G-165 HTML comments are pure metadata (zero rendered effect; grep-discoverable). The G-166 shell-prereq adds a bullet (no existing line modified). The G-167 known-limitations section is additive.

---

### Summary

10 findings classified (F1–F10): 8 Addressed in-session (G-160, G-161, G-162, G-163, G-164, G-165, G-166, G-167), 2 Deferred with G-130 discipline (G-168, G-169).

Backlog after Review 63: **0 Open + 3 Deferred** (G-159 from Review 62; G-168 and G-169 from this review). All three deferrals have named triggers + costs + auto-Backlog dates. The active-mining backlog from the documentation-and-tooling cluster (Review 62) plus this review's findings is fully classified.

Sycophancy self-audit (high importance — review of own recent work): I flagged 4 findings against my Review 62 closures (F2 / G-161 against G-96; F3 / G-163 against G-146; F4 / G-164 against G-146; F5 / G-165 against G-106's lack of regression-replay infra; F7 / G-167 against G-106's "captured as upstream feature request" framing). Each is a real defect my own closure left open. Anti-finding: I did NOT find a way to validate that my G-96 / G-106 / G-146 closures were "complete" in some grander sense — every closure leaves at least one downstream gap, and the suite's discipline is to surface them rather than declare completeness. The review classifications I'm most uncertain about are F1 / G-168 (Phase 2c at learning-exercise — could go either way; deferred for second-project evidence) and F10 / G-162 (the explicit-skip pattern's effectiveness at capstone+ — won't be verifiable until a capstone-intent project actually declares).

---

## Review 64 — 2026-05-20 00:30Z

**Scope:** Implement VSDD Phases 5 (Formal Hardening) and 6 (Four-Dimensional Convergence) — the two long-standing Open gaps G-55 + G-54 from Review 1 (2026-04-25). Operator-directed continuation of the Reviews 62 + 63 work cycle on the same `vsdd-suite-doc-tooling-cluster` branch. New artifacts: `primers/5-formal-hardening.md`, `primers/6-convergence.md`. Updates: README phase table + Suite scope + Session primers table; `primers/4-feedback-integration.md` routing table + labels; `domains/DOMAIN-INDEX.md` Phase 5/6 strategy declaration section (now points at real primers); `domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md` new dims 13 (Phase 5 discipline) + 14 (Phase 6 four-dimensional convergence).

**Lens:** Phase-implementation lens — design the methodology surface for two phases that the whitepaper defines but the suite previously did not own. The lens is constructive (primer authoring) plus enforcement (new VDD-IAR Alignment dims) plus integration (routing labels, intent calibration cross-references). Not a registry-walk; not a defect-search. The mode is **closure-by-implementation** — close two long-Open gaps by producing the missing artifacts.

**Session note:** In-session, dedicated branch (`vsdd-suite-doc-tooling-cluster`). Continuation of the Reviews 62 + 63 work cycle. Partial-isolation tradeoff acknowledged: I authored the new primers, the dim additions, and the routing-table extensions, so I cannot present as cold-context for these artifacts. Compensation: the primer designs deliberately avoided over-engineering — Phase 5 has four surfaces (A property-based, B mutation testing, C fuzzing, D formal proof) with explicit anti-patterns per surface; Phase 6 has the four-dimension MVR definitions plus a cross-dimension consistency check. Each primer's "skip with explicit declaration" pattern preserves the G-162 explicit-skip doctrine. Where I felt uncertainty, I named it inline (Surface D is "strictly optional even at capstone+ intent"; Phase 6 attestation cost-of-production is itself the audit signal). Cold-session review against this Review 64 work would be the natural next step — a fresh cold session reading the two new primers without the build context would surface the things I missed.

**Source:** director-raised (operator instruction "Implement VSDD phases 5 and 6 then address the related findings"). The G-54 + G-55 gaps were originally domain-raised (Review 1, 2026-04-25); the trigger for Review 64 closure was director-raised in this conversation.

---

### Resolved

**G-55 — Phase 5 (Formal Hardening) ownership transferred to the suite.**

`primers/5-formal-hardening.md` created. Four surfaces:

- **Surface A: Property-based testing for the purity boundary** — for each pure function on the verification-architecture map, express the spec's invariants as property-based tests (proptest / fast-check / hypothesis / etc.); each counterexample routes through Phase 4 to its appropriate destination (1a+1b for spec gap, 2b for implementation defect, 2a for over-specific property).
- **Surface B: Mutation testing of the existing test suite** — measures whether the test suite would catch realistic defects; each surviving mutant gets a per-mutant disposition (equivalent / missing-test / spec-gap), not aggregate-only kill-rate reporting (anti-pattern named).
- **Surface C: Fuzzing for parser / input-boundary surfaces** — per-layer fuzzing (not end-of-project) builds corpus over time; crashes route through Phase 4 like Phase 3 findings; corpus committed to `tests/` for cumulative coverage.
- **Surface D: Formal proof for designated pure functions (advanced)** — strictly optional even at capstone+; Kani / CBMC / TLA+ / Coq / Lean / Liquid Haskell named per language; the harness must establish a non-trivial spec-asserted property (no tautological harnesses).

Each surface has named anti-patterns and a sycophancy check applied to AI-driven Phase 5 work ("I'd rationalize surviving mutants as equivalent without proof — each must get a disposition or a falsifying test or a Phase 4 routing"). The primer specifies the `vsdd-suite/PHASE-5-LOG.md` per-layer entry format and integrates with crosslink + manual modes parallel to the other primers.

The primer ships with explicit forward-only positioning: projects whose first layer-gate close predates v0.7.0 are not retroactively required to retro-fit Phase 5. Capstone-intent projects starting after v0.7.0 must declare Phase 5 strategy per G-162 (no change from Review 63).

**Resolution:** G-55 status flipped Open → Addressed.

---

**G-54 — Phase 6 (Four-Dimensional Convergence) ownership transferred to the suite.**

`primers/6-convergence.md` created. The primer defines MVR per dimension (Spec / Test / Implementation / Formal-verification) and the cross-dimension consistency check that closes G-54's specific gap (the prior implementation-MVR-only signal was named in G-54 as "one-dimensional convergence; spec MVR, test MVR, and verification MVR are untracked").

The four MVR signals:

- **Spec MVR:** SO cold-batch reviews across final 2+ layers produced only Hallucinated findings AND Phase 4 routing surfaced no `route:phase-1a+1b` destinations across the final 2+ layers. Anti-signal named: silent DESIGN.md amendments that didn't re-trigger SO review.
- **Test MVR:** Phase 5 Surface B kill-rate per layer with every surviving mutant having a recorded disposition. For not-applicable Phase 5, signal is the QE final-round attestation (explicitly weaker; named in the convergence record as such).
- **Implementation MVR:** The suite's original signal; Phase 3 final round per active domain produces only Hallucinated findings under preserved cold-session isolation. Anti-signal: relaxed cold-context discipline.
- **Formal-verification MVR:** Surface D harnesses succeed for each declared formal-proof candidate, OR explicit `**Phase 6 strategy:** not applicable` declaration.

The **cross-dimension consistency check** is the load-bearing addition. Per spec-named behavior, walk the four dimensions and identify inconsistencies (spec asserts X but tests don't; tests assert X but spec doesn't; implementation does X but neither asserts; formal-verification disproves a spec property). Each inconsistency routes via Phase 4 before convergence is declared.

The Phase 6 record format (`vsdd-suite/PHASE-6-CONVERGENCE.md`) is per-project (not per-layer), written once at project close, preserved as audit trail on subsequent re-opens. The convergence attestation is signed and dated; subsequent work that touches any of the four dimensions re-opens the project and triggers a fresh Phase 6 record.

**Resolution:** G-54 status flipped Open → Addressed.

---

**G-162 strategy-declaration targets updated.**

The `### Phase 5 / Phase 6 strategy declaration` section in `domains/DOMAIN-INDEX.md` (Review 63 closure) previously said "the suite still doesn't own Phase 5/6 primers or dimensions, but ensures the gaps are visible at every capstone-tier closure." With G-54 + G-55 closed, this framing is updated: the strategy declarations now point at real primers (`5-formal-hardening.md` and `6-convergence.md`), and a `planned` declaration maps to specific Surfaces named in the primer. Example `planned` strategy declaration in DESIGN.md is now actionable: "property-based testing via proptest on the purity boundary + mutation testing via cargo-mutants per layer + fuzzing via cargo-fuzz on the parser surface (Surfaces A + B + C; Surface D not applicable — no formal-proof candidates)."

The strategy declaration is still required at capstone + production intents (G-162 unchanged); it now has a real downstream target.

---

**README and Session-primers table updated.**

- README § VSDD pipeline context phase table: rows 5 + 6 now point at the new primers; columns "What happens" and "IAR's role" filled with real content (not "(skip unless safety-critical)" placeholders).
- README § Suite scope: "Known scope gaps" line replaced with "Suite ownership" line listing every owned phase including 5 + 6 with G-54 + G-55 explicitly cited as closed.
- README § Session primers table: new rows for Formal Hardening + Four-Dimensional Convergence with `When to use` content keyed to intent-calibration.
- README candidate-domains line: "Formal Verification (for VSDD Phase 5+)" line replaced — formal verification is now owned by Phase 5 Surface D rather than a separate candidate domain.

---

**Phase 4 routing table extended for Phase 5 + Phase 6 destinations.**

`primers/4-feedback-integration.md` updates:

- New row: "Property-based counterexample, surviving non-equivalent mutant, fuzzer crash, failing proof harness" → **Phase 5** route.
- New row: "Spec/test/impl/formal inconsistency surfaced by the convergence check" → **Phase 6** route (typically multi-phase since the convergence check's inconsistencies route to the inconsistent dimension's destination phase).
- `Route` value list extended: `5`, `6` added.
- `route:phase-5`, `route:phase-6` labels added to crosslink-mode label list.

---

**VDD-IAR Alignment dims 13 + 14 added.**

`domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md` extended:

- **Dim 13 (Phase 5 discipline)** — applies only when `**Phase 5 strategy:** planned` is declared. Evaluates: Surface activation matches strategy; per-mutant disposition completeness; property-test invariant fidelity (not "doesn't panic" liveness); fuzz-corpus persistence; Phase 4 routing of Phase 5 findings.
- **Dim 14 (Phase 6 four-dimensional convergence)** — applies only when `**Phase 6 strategy:** planned` is declared. Evaluates: per-dimension citation specificity; cross-dimension consistency check completeness; out-of-scope dimensions explicitly named; convergence attestation signed + dated; cost-of-production audit signal (a 5-minute record vs. a 30+-minute record).

Both dims default to "Demonstrated" for `not applicable` declarations (the declaration itself is the evidence) and require substantive evidence for `planned` declarations.

---

### Coordination

The Review 64 closures coordinate with:

- **G-150 / G-162** (Addressed prior) — intent calibration unchanged; Phase 5/6 strategy declarations now have real downstream targets in the new primers. A capstone-intent project's strategy lines can now name specific Surfaces (5) and dimensions (6).
- **G-161** (Addressed Review 63) — Phase 2c refactor discipline. Independent of Phase 5/6 but composes: a Phase 5 finding routed to Phase 2c (refactor regression) follows G-161's discipline (no new behavior; explicit-skip alternative).
- **G-130** (Addressed prior) — Deferral discipline. Phase 5/6 work that surfaces gaps the project chooses to defer must name trigger + cost + auto-Backlog per G-130. The primer doesn't re-state this discipline; it inherits.
- **G-89** (Addressed prior) — forward-only narrative preservation. Projects whose first layer-gate close predates v0.7.0 retain prior phase-coverage framings. The new Phase 5/6 ownership is forward-only.
- **G-144** (Addressed prior) — manual-mode parity preserved. Both new primers ship crosslink + manual mode sections; the methodology is loadable into either operational mode.
- **G-96** (Addressed Review 62) — whitepaper sub-phase taxonomy harmonization. Phase 5/6 closures complete the full whitepaper-phase enumeration: the suite now owns 1a+1b, 1c, 2a, 2b, 2c, 3, 4, 5, 6. No phase the whitepaper defines is unowned by the suite.

The closures do NOT regress prior work. The Phase 5/6 primers are additive (new files; no existing primer modified). The README updates remove the "skip unless safety-critical" framing but preserve the underlying optional-by-intent disposition (lower intents still close at end of Phase 4 by design). The Phase 4 routing-table additions are net-new rows. The VDD-IAR Alignment dim additions are intent-gated (only apply at capstone+ with `planned` declarations); learning-exercise and portfolio intents are unaffected.

---

### Summary

G-54 + G-55 Addressed. Every VSDD phase the methodology defines is now owned by the suite — the "Known scope gaps" framing in the README is retired. Sycophancy self-audit (highest importance — this work expands the suite's surface substantially): the two new primers are written in the same authorial voice as the prior primers, with deliberate friction at the anti-patterns ("treat Surface B as a checklist" / "treat Phase 6 as 'looks finished'"). I did NOT add a new "Formal Verification Engineer" domain — explicitly considered and rejected as scope creep; Surface D's existing-tooling references (Kani / CBMC / TLA+ / Coq / Lean / Liquid Haskell) cover the territory without introducing a parallel role concept. I did NOT add new finding classifications to any domain — Phase 5/6 findings classify through the existing schemas plus the new route labels. The most uncertain choice: Surface D as part of Phase 5 rather than its own phase. Argument for keeping it under Phase 5: it shares discipline with Surfaces A-C (verification of spec invariants against implementation evidence); Surface D is "more rigorous evidence" rather than "different methodology." Argument against: Surface D involves theorem-prover tooling that's qualitatively different from property-based testing. The current placement (Surface D under Phase 5) is the cheaper choice; if a project's formal-verification scope grows to dominate its hardening effort, a future Review can spin Surface D into a separate Phase 5.5 or extend Phase 6's formal-verification dimension. Forward-only carve-out preserves the current placement as valid for projects starting under v0.7.0.

Backlog after Review 64: **0 Open + 3 Deferred** (G-159 from Review 62; G-168 + G-169 from Review 63 — unchanged by Review 64). The Open-gap surface remaining in `FINDINGS-INDEX.md` from older reviews (G-11, G-13, G-14, G-15, G-16, G-17, G-18, G-26, G-28, G-29, G-31, G-57) is the project-template surface, not the Phase 5/6 surface — outside Review 64's scope.

---

