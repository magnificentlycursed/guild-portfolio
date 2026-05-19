# Suite Review — 2026-05-19

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

## Review 65 — 2026-05-20 01:30Z

<!-- hook-bypass: chat-shorthand F-prefix finding headers committed pre-Review-67 discipline correction; preserved per G-89 forward-only narrative-preservation policy. See § Errata at the end of this entry for the F-prefix → Finding-N cross-reference map. Discipline correction in Review 67 Findings 1+2 prevents recurrence. -->

**Scope:** Cold-context adversarial review of the two Review 64 primers (`primers/5-formal-hardening.md` and `primers/6-convergence.md`) via a subagent spawned in isolation from this session's authorial context. The subagent loaded the two new primers + `primers/3-review-session.md` (posture) + `suite-development/suite-development.md` (governing standard for primers) + role prompts for Quality Engineer (Phase 5 lens) + Solution Architect (Phase 6 lens). The subagent surfaced 12 findings (F1–F12), all classified Deferred by the subagent (because a reviewer does not edit; it surfaces). The director (this session) addressed 9 of 12 in-session and registered the remaining 3 as Deferred gaps with G-130 discipline.

**Lens:** Cold-context primer-validation pass. The subagent is the closest available approximation to a true cold reviewer — it begins without memory of the Reviews 62/63/64 work. The six lenses applied per the subagent's brief: primer-contract compliance; internal consistency; defect-catch coverage (QE on Phase 5); architectural coherence (SA on Phase 6); cold-reader UX; external-dependency references. SA lens produced the most findings (F4, F5, F6, F11 — four), consistent with the brief's framing that the convergence check is the load-bearing addition.

**Session note:** **Subagent-driven cold review.** The subagent isolation is partial — it shares the codebase and the Claude model with this session, but begins with no in-context memory of the primers being authored in Review 64. The compensation for the residual context is the subagent's structural compliance with the cold-session posture (loaded `3-review-session.md` first; classified findings per its schema; flagged its own sycophancy self-audit at the end). The findings the subagent surfaced are evidence-based with file:line citations; none are narrative-only. The director's in-session disposition of these findings is itself subject to sycophancy risk — I am classifying findings against work I wrote one session ago. Compensation: I addressed 9 of 12 in-session rather than dismissing them; the 3 I deferred each have a real design question that warrants more thought than a single Edit (and the deferral discipline G-130 names trigger + cost + auto-Backlog for each).

**Source:** `director-raised` (operator instruction: "Do option 2 then commit") triggered a cold-context review session. The findings within the review are `domain-raised` (the subagent surfaced them by applying domain lenses to the new primers).

---

### Resolved

**F10 — Phase 5 Surface B "default threshold" undefined / contradictory.** Subagent identified that `primers/5-formal-hardening.md:64` named a "threshold" but its definition ("default: every surviving mutant in spec-asserted code") was a scope, not a threshold; completion criterion 3 referenced "under the threshold" with no numeric anchor. Resolution: renamed "threshold" to "evaluation scope" throughout Surface B; defined scope explicitly as "every surviving mutant in spec-asserted code paths"; named the out-of-scope-omissions-must-be-listed-in-the-log-preamble discipline. Completion criterion 3 updated. Cold reader can now determine when Surface B is gate-complete.

**F12 — Forward-only anchored to "v0.7.0 ship date" rather than a date.** Subagent identified the convention drift against `suite-development.md` (which date-anchors forward-only constraints, not version-anchors). Resolution: replaced "v0.7.0 ship date" with the explicit date `2026-05-20` (Review 64 / v0.7.0) in both `primers/5-formal-hardening.md` § Forward-only and `primers/6-convergence.md` § Completion criteria. Audit risk closed: a reader doesn't need to map version to date.

**F8 — Phase 5 sycophancy check covered only Surface B.** Subagent identified that `suite-development.md:91` requires every primer's prompt section to name "the primary failure mode to watch for" — the prior Phase 5 sycophancy check named Surface B's failure mode (rationalizing surviving mutants as equivalent) but Surfaces A, C, D had only artifact-level anti-patterns, not cognitive-failure-mode sycophancy checks. Resolution: expanded the prompt-level sycophancy check into a per-surface block — Surface A (writing properties whose only assertion is doesn't-panic / liveness), Surface B (rationalizing equivalent mutants), Surface C (declaring "no crashes found" after short fuzz runs as evidence; closure now requires both budget exhaustion AND non-trivial coverage/corpus signal), Surface D (tautological proof harnesses; each harness's property must map to a DESIGN.md sentence).

**F2 — `fast-check` conflated between Surface A and Surface C on JS/TS.** Subagent identified that `fast-check` appeared in both Surface A (line 42) and Surface C (line 76) without distinguishing the property shape per surface — risking the cold reader collapsing two surfaces into one. Resolution: Surface C's JS/TS line now explicitly distinguishes the property shape (Surface A asserts spec invariants over generated inputs; Surface C asserts "doesn't crash / matches a fuzzing oracle" over `fc.string()` / `fc.uint8Array()` parser-input generators) and prescribes two distinct Surface entries in `PHASE-5-LOG.md` rather than one combined entry.

**F1 — Phase 6 `crosslink milestone close` cited without ID-capture pattern.** Subagent identified that `primers/6-convergence.md:184` showed `crosslink milestone close <project-terminal-milestone-id>` without breadcrumb for obtaining the numeric ID — a cold reader following verbatim would hit the same `invalid digit found in string` failure that G-167 / `crosslink-contract.md` § Known limitations documents. Resolution: added a `crosslink milestone list` recovery step inline above the close invocation, with explicit cross-reference to G-167 / crosslink-contract.md § Known limitations.

**F5 — Phase 6 Dimension 2 anti-signal non-falsifiable from convergence record alone.** Subagent identified that the prior Dimension 2 disposition record asked for "kill rate + disposition class counts" but had no explicit verification step that each cited per-layer Surface B section actually contains the per-mutant disposition table the Surface B mandate requires — a Phase 5 log that violated Surface B's mandate but reported a clean rate would still satisfy Phase 6 Dimension 2 as written. Resolution: added explicit "Verification step (per F5 — required, not aspirational)" paragraph in Dimension 2 instructing the convergence-record author to open each cited per-layer Surface B section, confirm the disposition table exists, and route the layer back to Phase 5 if absent. Closes the cross-primer verification gap the subagent surfaced.

**F3 — Phase 5 completion-criterion 1 ("purity boundary verified") had no surface owning it.** Subagent identified that the gate criterion was load-bearing but unassigned — none of Surfaces A–D actually performed the boundary audit. Resolution: new `### Surface A.0: Purity-boundary verification (preamble — required for every Phase 5 layer entry)` as a Phase 5 layer-entry prerequisite. The audit is explicit: open the implementation, verify pure-function signatures and bodies don't perform I/O (file / network / process / env / random / time), route violations one of three ways (Phase 2b to restore purity; Phase 1a+1b to revise the boundary; named boundary refinement in the log preamble). Cold reader now knows where completion criterion 1 is satisfied. VDD-IAR Alignment dim 13 already references Surface activation — the omission of Surface A.0 becomes a finding for dim 13 mechanically.

**F9 — "Signed attestation" conflicted with anonymization posture.** Subagent identified that `primers/6-convergence.md:165` required "Signed: <name> <date>." but anonymized portfolio projects (per `primers/3-review-session.md:30-35`'s Confidentiality-aware citation discipline) cannot use a real name without violating the anonymization posture. Resolution: new `**Anonymization-aware attestation (F9 — applies when the project's identity posture is opt-in anonymized)**` paragraph specifying that anonymized projects use the closing-commit's git hash as the signature (non-repudiable; respects anonymization). Non-anonymized projects retain real-name signing. Cross-references `primers/3-review-session.md` § Confidentiality-aware citation for the anonymization signals the operator looks for.

**F7 — Phase 5 "retroactive-Red-Gate label" cross-reference was to a forward-flow construct.** Subagent identified that `primers/2b-implementation.md:13/30` defines the label semantic for "discovered during Phase 2b" (in-flight implementation work), but Phase 5 surfaces missing tests against already-shipped implementation-MVR code (post-MVR retroactive discovery) — different semantic. Resolution: extended the 2b primer's label definition to explicitly cover post-MVR retroactive discovery via a `(Phase 5 source)` qualifier — same label discipline, expanded scope. Phase 5 Surface B sycophancy check now cites "retroactive-Red-Gate (Phase 5 source) label" explicitly. No new label introduced — minimal scope creep.

---

### Deferred

**G-170 (new — addresses F4) — Cross-dimension consistency check has no disposition class for spec behaviors that are intentionally test-less.**

Subagent identified that `primers/6-convergence.md:79-91` requires every spec-named behavior to have a test row in the consistency-check table, with any missing test routing to Phase 2a. But some spec behaviors are legitimately untestable-by-design: build-toolchain selection, license text, intent declaration in DESIGN.md itself, README rendering. The check as written would either generate spurious Phase 2a routings or silently skip rows.

**Trigger:** when a capstone-intent project's first Phase 6 record surfaces a spec behavior that warrants an `untestable-by-design` disposition. Observable on the first Phase 6 author noting that a row has no defensible Phase 2a destination.

**Cost-of-deferral:** the first Phase 6 author hits the gap and either improvises a disposition (which then becomes the precedent) or routes spuriously to Phase 2a. The improvised precedent risks becoming the convention without director review.

**Auto-Backlog clause:** if 2026-09-30 passes without a real Phase 6 author hitting this gap, the gap auto-Backlogs and re-raises in the next suite review for fresh evaluation (it may be the gap is theoretical rather than load-bearing for the project types that hit capstone intent).

**Proposed resolution at trigger:** introduce an `untestable-by-design` disposition class for the consistency-check table, parallel to `not applicable` in the strategy declarations. Required: a one-sentence rationale per row (matching deferral-trigger discipline G-130's "real rationale, not TBD" rule). The row stays visible in the table; the absence is the disposition.

**Coordinate with:** G-130 (deferral-trigger discipline — the rationale shape borrows from it), G-162 (explicit-skip pattern — same intellectual root), G-54 (parent gap; the convergence check is its closure).

---

**G-171 (new — addresses F6) — Phase 6 strategy declaration grammar lives in DOMAIN-INDEX but the Phase 6 primer doesn't link it; portfolio-intent verification path ambiguous.**

Subagent identified two sub-issues: (a) `primers/6-convergence.md:5/117-119` references `**Phase 6 strategy:** not applicable` and `planned` declarations but doesn't link to the strategy grammar in `domains/DOMAIN-INDEX.md` § Phase 5 / Phase 6 strategy declaration (G-162); a cold reader running Phase 6 doesn't know what valid declarations look like without chasing the cross-reference manually. (b) The primer simultaneously says portfolio-intent closes at end of Phase 4 with a `Phase 6 strategy: not applicable` AND that closures predating v0.7.0 retain prior implementation-MVR-only closure — the audit-ownership for the not-applicable declaration's verification is ambiguous when the project never opens this primer.

**Trigger:** when a portfolio-intent project's first DESIGN.md author asks "where does my `Phase 6 strategy: not applicable` declaration get verified if I never open the Phase 6 primer?" OR when a capstone-intent author asks "what does a valid `Phase 6 strategy: planned` declaration look like without me cross-referencing DOMAIN-INDEX?"

**Cost-of-deferral:** strategy declarations land without authors knowing the grammar; the audit-trail signal G-162 was designed to produce degrades.

**Auto-Backlog clause:** 2026-09-30. (Paired with G-170; same trigger window.)

**Proposed resolution at trigger:** (a) add an inline grammar block in `primers/6-convergence.md` § Project reference area citing the valid declaration shapes (mirroring DOMAIN-INDEX) with a forward-link to DOMAIN-INDEX as the canonical source; (b) clarify the ownership boundary — portfolio-intent `not applicable` declarations are verified at DESIGN.md authoring time (by VDD-IAR Alignment dim 1) rather than at Phase 6 entry, because the project never opens Phase 6. The two verification paths are explicit per intent.

**Coordinate with:** G-162 (parent — strategy declaration discipline), G-150 (intent calibration — the per-intent verification path differs by intent), VDD-IAR Alignment dim 1 (the actual enforcement surface for portfolio-intent declarations).

---

**G-172 (new — addresses F11) — Phase 6 Dimension 3 (Implementation MVR) layer-to-project rollup rule is missing.**

Subagent identified that `primers/6-convergence.md:57` cites Phase 3 G-131/G-151 round triggers — but those triggers are layer-scoped (`primers/3-review-session.md:79-109`). A four-layer project that closed each layer at MVR has four layer-scoped Implementation MVR signals; Phase 6 needs a project-scoped conjunction rule the primer does not state. The disposition record at line 61 cites "the final Phase 3 round per active domain" — singular round / singular layer — but a project with N layers needs N×|active-domains| citations and a rollup rule.

**Trigger:** when a four-or-more-layer capstone-intent project reaches Phase 6 entry and the author asks "do I cite all 4 layers' final rounds for SE, or just the last layer's SE round?"

**Cost-of-deferral:** the first multi-layer Phase 6 author improvises a rollup (likely under-specifying — citing only the last layer's domain rounds) without realizing earlier layers' MVR signals could be stale if subsequent layers' refactors affected the earlier layers' code.

**Auto-Backlog clause:** 2026-12-31 (later than G-170/G-171 because multi-layer capstone projects are rarer in the near-term portfolio).

**Proposed resolution at trigger:** explicit rule in `primers/6-convergence.md` Dimension 3: project Implementation MVR = ∀ layer ∈ project, ∀ active-domain ∈ layer.active-domains, layer.final-round-for-domain MVR-signal AND no subsequent layer's commits touched files in this layer's scope without a re-verification round on the affected layer. The "no subsequent layer touched my files" clause is the load-bearing piece — protects against stale MVR signals.

**Coordinate with:** G-54 (parent — convergence check), G-131 (continue trigger — the per-layer signal Phase 6 rolls up), G-151 (stop trigger — same), VDD-IAR Alignment dim 14 (the enforcement surface).

---

### New gap registered

- **G-170 (F4)** — Untestable-by-design disposition class for the cross-dimension consistency check — Deferred
- **G-171 (F6)** — Phase 6 strategy declaration grammar inline + portfolio-intent verification path explicit — Deferred
- **G-172 (F11)** — Project-level Implementation MVR rollup rule — Deferred

---

### Coordination

The Review 65 dispositions coordinate with:

- **G-54 / G-55** (Addressed Review 64) — the parent gaps. Review 65's findings are second-pass refinements; none invalidates the closure. The cold reviewer explicitly noted all 12 findings were "real but the primers are coherent at the surface level" — the closures hold, the refinements deepen them.
- **G-130** (Addressed prior) — the three Deferred new gaps (G-170, G-171, G-172) each name trigger + cost + auto-Backlog per the discipline.
- **G-162** (Addressed Review 63 / refreshed Review 64) — F6 / G-171 surfaces a sub-issue in the strategy-declaration grammar's reachability from the Phase 6 primer; the trigger-on-first-portfolio-author is honest about when this needs to land.
- **G-167** (Addressed Review 63) — F1's resolution cross-references G-167's known-limitations entry rather than re-stating the workaround.
- **G-89** (Addressed prior) — Forward-only narrative preservation. The F12 date-anchor refinement was a convention drift the subagent caught; the fix aligns the new primers with the suite's established forward-only convention. Historical narratives (review-log + COMPATIBILITY) keep their prior prose.
- **VDD-IAR Alignment dim 13 + dim 14** (Added Review 64) — F3 resolution (Surface A.0 preamble) becomes mechanically checkable via dim 13; F5 resolution (Dimension 2 verification step) becomes mechanically checkable via dim 14.

---

### Summary

12 findings classified: 9 Resolved in-session, 3 Deferred with G-130 discipline. Cold review served as quality gate (not audit-only) — the new primers ship with the refinements rather than known defects. Backlog after Review 65: **0 Open + 6 Deferred** (G-159, G-168, G-169, G-170, G-171, G-172). The 3 new deferrals each have triggers tied to real project usage events (first multi-layer capstone Phase 6 author; first portfolio-intent strategy-declaration author) rather than calendar dates alone.

Sycophancy self-audit: the cold reviewer's findings were largely-impossible-to-dismiss-after-citation — every finding had a file:line reference and a specific evidence trail. The temptation I felt was to defer more of them as "judgment calls" to avoid the work; I forced myself to address the cheap ones in-session because the cost was a single Edit per finding. The three I genuinely deferred (F4, F6, F11) each pose a real design question (a new disposition class; cross-primer reachability of strategy grammar; a multi-layer rollup rule) that warrants more thought than a single Edit. The cold reviewer's own sycophancy self-audit named three near-passes (Surface D carve-out; per-prompt sycophancy quality from one strong example; v0.7.0 version anchoring read as harmless) — the same shape my Review 64 self-audit could have caught had I been more rigorous. The cold pass produced what my in-session pass didn't catch.

### Errata

**Finding 13 — Cross-reference map: chat-shorthand F-prefix labels in `### Resolved` (added 2026-05-20)**

Per Review 67 Finding 2 (chat-shorthand identifiers stay out of artifacts), Review 65's `### Resolved` section uses non-standard `**F1 — `, `**F2 — `, etc. headers — the chat-shorthand labels the cold subagent surfaced. Forward-only G-89 preserves the original committed text; this errata provides the F-prefix → Finding-N cross-reference map so future readers can resolve `F1`/`F2`/etc. citations against the per-review Finding-N convention. The Deferred findings retain their G-IDs separately.

| Chat-shorthand label (committed) | Finding-N (the standard form this entry should have used) | Title |
|---|---|---|
| F10 | Finding 1 | Phase 5 Surface B "default threshold" undefined / contradictory |
| F12 | Finding 2 | Forward-only anchored to "v0.7.0 ship date" rather than a date |
| F8 | Finding 3 | Phase 5 sycophancy check covered only Surface B |
| F2 | Finding 4 | `fast-check` conflated between Surface A and Surface C on JS/TS |
| F1 | Finding 5 | Phase 6 `crosslink milestone close` cited without ID-capture pattern |
| F5 | Finding 6 | Phase 6 Dimension 2 anti-signal non-falsifiable from convergence record alone |
| F3 | Finding 7 | Phase 5 completion-criterion 1 ("purity boundary verified") had no surface owning it |
| F9 | Finding 8 | "Signed attestation" conflicted with anonymization posture |
| F7 | Finding 9 | Phase 5 "retroactive-Red-Gate label" cross-reference was to a forward-flow construct |

Deferred findings (already have G-IDs; F-prefix maps to G-ID):

| Chat-shorthand label | G-ID | Title |
|---|---|---|
| F4 | G-170 | Untestable-by-design disposition class for cross-dimension consistency check |
| F6 | G-171 | Phase 6 strategy declaration grammar inline + portfolio-intent verification path |
| F11 | G-172 | Project-level Implementation MVR rollup rule |

**Classification:** Errata — cross-reference map for non-standard chat-shorthand labels; original entries unchanged per G-89.

---

## Review 66 — 2026-05-20 02:30Z

<!-- hook-bypass: chat-shorthand R-prefix finding headers committed pre-Review-67 discipline correction; G-173–G-176 G-IDs also over-promoted to Resolved-in-session findings per Review 67 Finding 1. Both drifts preserved per G-89 forward-only narrative-preservation policy. See § Errata at the end of this entry for the R-prefix cross-reference map. Discipline correction in Review 67 Findings 1+2 prevents recurrence. -->

**Scope:** Refinements to `primers/5-formal-hardening.md` surfaced by running Phase 5 against `bookmark-cli` in a sandbox (the option-3 validation exercise that closed the Review 64 work cycle). Real Phase 5 execution surfaced 4 primer-refinement candidates (R1–R4) plus 2 bookmark-cli-side findings (B1, B2). This review addresses R1–R4; B1+B2 remain unaddressed pending operator authorization (bookmark-cli is the reference impl per G-117/G-122).

**Lens:** Methodology-tested-against-reference-impl lens. The Phase 5 primer was validated by running its prescriptions against bookmark-cli's actual source: cargo-mutants Surface B produced 11 mutants (7 caught, 1 missed, 3 unviable → 87.5% kill rate on viable); Surface A.0 purity-boundary audit caught a divergence between bookmark-cli's `src/lib.rs:1-7` module-doc purity claim and the actual implementation (3 of 4 `BookmarkStore` methods are effectful). Both findings are usable; the surfacing exercise produced 4 friction points the primer should address before it ships to a second project.

**Session note:** In-session, post-commit (the Review 62–65 work is committed at `57d5341`; Review 66 lands as additional refinements on the same branch). Partial-isolation tradeoff acknowledged — the same author who wrote the Phase 5 primer (Reviews 64+65) is now refining it based on its own run output. The compensation: refinements derived from concrete execution outcomes (real mutant disposition output; real Surface A.0 findings) rather than narrative reflection. The cold equivalent would be a second subagent re-running Phase 5 with the refined primer against a different project; deferred (no obvious second target).

**Source:** `domain-raised` — Phase 5's own surfaces (A.0 audit; cargo-mutants execution) raised the findings against the primer-as-applied; the operator's "Address R1–R4" instruction `director-raised` the closure decision.

---

### Resolved

**R1 / G-173 — Surface A.0 multi-source purity check.**

bookmark-cli's `src/lib.rs:1-7` (module doc) said "Pure-core storage logic ... contains only pure functions" while its DESIGN.md § Verification architecture was silent on per-function purity. The prior Surface A.0 audit instructions in `primers/5-formal-hardening.md` (Review 65 / F3 closure) only covered DESIGN.md as the purity-claim source — module docs were not in scope.

Resolution: extended Surface A.0 prose to list "every authoritative purity claim the project makes" with explicit enumeration: DESIGN.md § Verification architecture AND module/package documentation (Rust `//!`; Python module docstrings; TypeScript `/** @module */` JSDoc; "equivalent constructs in other languages"). Added a third check beyond (a) impl-vs-DESIGN.md and (b) impl-vs-module-doc: (c) DESIGN.md vs. module-doc cross-source consistency. The bookmark-cli case ("`src/lib.rs:1-7` claimed pure; DESIGN.md silent; both diverged from actual") cited as the canonical worked example. New routing option in step 3: if the two sources make divergent purity claims, the divergence is a finding → route to Phase 1a+1b to reconcile (single source of truth at DESIGN.md OR module doc cites DESIGN.md). Step 4 (audit-outcome log preamble) updated to record cross-source consistency status.

**R2 / G-174 — Surface B "unviable" disposition class.**

cargo-mutants reports 3 outcome classes (caught / missed / unviable). The prior Surface B sycophancy check enumerated 3 dispositions for surviving mutants ((a) equivalent / (b) missing-test / (c) spec-gap) but had no class for the unviable case. Phase 5's run against bookmark-cli produced 3 unviable mutants (e.g., `+` → `-` in `String + &str` string concatenation — type-system-rejected) that fit none of the 3 dispositions but did need to be listed in the log for completeness.

Resolution: added 5th disposition class (d) "unviable — mutation does not compile; not a behavioral signal" to the Surface B sycophancy check. The Surface B run-the-tool sub-section's step 2 ("Examine each surviving mutant...") updated from "four outcomes" to "five outcomes" and added explicit guidance: unviable mutants get a one-line note in the log (with the example "`src/lib.rs:55:35` `+` → `-` in string concatenation: unviable — compile failure on `String - &str`"), but they are not test-suite gaps. The disposition table now matches cargo-mutants' actual output shape; the Phase 5 log preamble has a categorical home for every outcome class.

**R3 / G-175 — Tool-install upfront cost note.**

cargo-mutants compiles from source on install (timed at 1m 39s on the bookmark-cli test machine). The prior Phase 5 primer mentioned cargo-mutants / cargo-fuzz / proptest etc. by name but did not flag that the first-Phase-5-session-per-project bundles tool installs with the run. A new operator reading the primer expected to start hardening within minutes; the realistic first-run wall-clock includes the install.

Resolution: new "**Tool-install upfront cost (G-175)**" paragraph immediately under § Phase 5 surface and above Surface A.0. Names the install costs explicitly: cargo-mutants compiles from source (1–2 minutes); cargo-fuzz requires Rust nightly toolchain; proptest/fast-check/hypothesis are dev-dependency adds. Prescribes that first-session log preamble names installs performed and time spent on installs separately from time spent on actual hardening. Subsequent sessions inherit and skip installs.

**R4 / G-176 — Surface A Cargo.toml dep-add prerequisite.**

Surface A's prior prose said "Express each invariant as a property-based test using the language's standard tool: Rust: proptest..." but didn't mention that adding proptest requires editing Cargo.toml's `[dev-dependencies]` — the dep-add is a separate commit from the property-test commits, and a cold reader running Surface A without prior Rust dep-management experience would not know to do this first.

Resolution: new "**Prerequisite (G-176)**" paragraph immediately under Surface A's intro. Names the dev-dependency target per language (Rust: `proptest` or `quickcheck` in `[dev-dependencies]` in `Cargo.toml`; JS/TS: `fast-check` in `devDependencies` in `package.json`; Python: `hypothesis` in `[tool.poetry.group.dev.dependencies]` or `requirements-dev.txt`; Go: `gopter` via `go.mod`). Prescribes that the dep-add lands as a separate commit before the property-test commits — keeps dep-introduction reviewable independently. Same shape as the Phase 2a Red Gate commit boundary discipline: separate commits for separable concerns.

---

### Coordination

The Review 66 refinements coordinate with:

- **G-55** (Addressed Review 64) — parent gap. G-173 / G-174 / G-175 / G-176 each refine Surface A.0 / Surface B / the primer intro / Surface A specifically; the parent's closure holds — these are refinements within the existing primer surfaces, not gap re-opens.
- **G-89** (Addressed prior) — forward-only narrative preservation. The Surface A.0 refinement extends the prior audit instruction; previous Phase 5 log entries (none yet exist in any project) remain valid records under the prior single-source audit instruction.
- **G-130** (Addressed prior) — deferral-trigger discipline. No new deferrals from Review 66; the 6 prior Deferred gaps (G-159, G-168, G-169, G-170, G-171, G-172) unchanged.
- **G-167** (Addressed Review 63) — crosslink-contract.md § Known limitations. The unviable-mutant disposition pattern (R2 / G-174) mirrors the same shape as known-limitations entries: name the observed behavior; provide a workaround; classify as not-a-bug. The shared structural shape isn't named in either primer but is the same intellectual pattern.
- **VDD-IAR Alignment dim 13** (Added Review 64) — dim 13's Surface-activation-matches-strategy check now covers the 5-disposition Surface B universe (G-174) and the multi-source Surface A.0 audit (G-173). The dim's evaluation procedure becomes slightly stricter; no rewrite needed because dim 13 references the primer rather than restating its contents.

The refinements do NOT regress any prior work. R1's multi-source Surface A.0 audit is additive (the prior DESIGN.md-only check still applies; the module-doc check is layered on top). R2's unviable disposition is additive (5 classes where there were 4; existing 4 unchanged). R3's tool-install note is metadata. R4's dep-add prerequisite is a sequencing clarification, not a methodology change.

**The two bookmark-cli-side findings (B1, B2) from the Phase 5 test against bookmark-cli remain UNADDRESSED:**

- **B1** — `tests/bookmarks.rs` is missing a save-to-nested-path test; the cargo-mutants surviving mutant at `src/lib.rs:48` would be killed by adding it. Disposition per Surface B: (b) test gap; route via Phase 4 to Phase 2a with the `retroactive-Red-Gate (Phase 5 source)` label per the primer.
- **B2** — DESIGN.md § Verification architecture is silent on per-function purity while `src/lib.rs:1-7` claims "Pure-core storage logic"; both diverge from the implementation (3 of 4 `BookmarkStore` methods are effectful). Disposition per Surface A.0: cross-source divergence + impl drift; route via Phase 4 to Phase 1a+1b (reconcile DESIGN.md and module doc to a single authoritative source; revise the boundary to match the implementation OR refactor the implementation to honor the boundary).

These are project-side findings against the reference implementation. bookmark-cli is the suite's worked example (G-112 closure; reference impl per G-117 / G-122). Modifying it warrants explicit operator authorization rather than routine work — the project's review-log structure would need to absorb Phase 5 findings under the v0.7.1 conventions, and bookmark-cli's first-layer-gate-close predates 2026-05-20 (forward-only G-89 carve-out applies). The findings are surfaced here for visibility; the operator decides whether to bring bookmark-cli to v0.7.1+ conventions in a future cycle.

---

### Summary

4 primer refinements Addressed in-session (R1–R4 → G-173–G-176). 2 project-side findings (B1, B2) surfaced and held pending operator authorization. Backlog after Review 66: **0 Open + 6 Deferred** (unchanged from Review 65: G-159, G-168, G-169, G-170, G-171, G-172). The Phase 5 primer's first real-project execution validated the methodology (concrete findings produced; disposition shapes worked) and surfaced 4 friction points the cold review hadn't anticipated — the methodology-tested-against-reference-impl lens caught what the cold-context primer-validation lens (Review 65) missed.

Sycophancy self-audit: the Phase 5 primer's first run produced findings; I closed 4 of the 4 friction points it surfaced rather than dismissing any as "edge cases the primer doesn't need to cover." The temptation was to dismiss R3 (tool-install cost) as "obviously a per-operator concern" — rejected because the primer is what a new operator reads BEFORE running, so an upfront note saves the operator the discovery cost. The temptation was also to defer R1 as "judgment call about what counts as a 'purity claim source'" — rejected because the bookmark-cli case is concrete (`src/lib.rs:1-7` made a claim that diverged from DESIGN.md silence) and any first-Phase-5 operator on any Rust project with module docs will hit the same shape. Both temptations represent the same failure mode the Phase 5 primer's own Surface B sycophancy check warns against: rationalizing surviving findings as "not really gaps." The discipline applied to the primer as it does to project mutants.

Most uncertain choice: R4 prescribes "dep-add lands as a separate commit before the property-test commits" — borrowing from the Phase 2a Red Gate boundary discipline. This may be over-prescriptive for small projects where dep-add and first-property could share a commit without harming reviewability. Forward-only carve-out preserves the prescription for v0.7.2+ starters; if a project's first Phase 5 session genuinely warrants a combined commit, the divergence is itself a Phase 5 log entry note rather than a discipline violation.

### Errata

**Finding 5 — Cross-reference map: chat-shorthand R-prefix labels in `### Resolved` + G-ID over-promotion (added 2026-05-20)**

Per Review 67 Finding 1 (G-IDs only for Deferred / Open findings) and Finding 2 (chat-shorthand identifiers stay out of artifacts), Review 66's `### Resolved` section drifted on both axes: the 4 Resolved-in-session findings were promoted to G-IDs (G-173–G-176) when they should have remained `**Finding N — Title**` entries, AND the headers used a compound `**Rn / G-XXX — Title**` form that combined chat shorthand with the G-ID. Forward-only G-89 preserves the original committed text; this errata provides the R-prefix and Finding-N cross-reference map.

| Chat-shorthand label (committed) | G-ID assigned (committed; over-promoted) | Finding-N (the standard form this entry should have used) | Title |
|---|---|---|---|
| R1 | G-173 | Finding 1 | Surface A.0 multi-source purity check |
| R2 | G-174 | Finding 2 | Surface B "unviable" disposition class |
| R3 | G-175 | Finding 3 | Tool-install upfront cost note |
| R4 | G-176 | Finding 4 | Surface A Cargo.toml dep-add prerequisite |

The G-IDs (G-173–G-176) stay registered in FINDINGS-INDEX.md as committed records per G-89; future readers can resolve `R1`/`R2`/`R3`/`R4` citations via either the Finding-N column above or the G-ID column.

**Classification:** Errata — cross-reference map for non-standard chat-shorthand labels + G-ID over-promotion; original entries unchanged per G-89.

---

## Review 67 — 2026-05-20 03:15Z

**Scope:** Operator-driven adversarial questions against the Review 64–66 Phase 5 / Phase 6 work, surfaced when reviewing bookmark-cli's first Phase 5 application. Two questions: (a) why does `PHASE-5-LOG.md` exist as a separate per-project file instead of folding Phase 5 findings into the existing per-domain review logs; (b) why did bookmark-cli's first Phase 5 session file only QE-domain findings when the Surface A.0 finding is SA territory. Question (b) is project-scope (closed in `bookmark-cli-phase5-adoption` branch by splitting the session into QE Review 2 + SA Review 1). Question (a) is suite-scope and this entry files it as Finding 1 / G-177 below.

A third adversarial question from the same operator turn — chat-shorthand finding identifiers (`Q1`, `Q2`, `B1`, `B2`, `R1`–`R4`, `F1`–`F12`) have been leaking from chat into review-log artifacts, commit messages, and PR descriptions across Reviews 65 / 66 / the bookmark-cli adoption commit. The governing standard in `suite-development.md` § Suite review entry format prescribes only `**Finding N — Title**` (new findings) and `**G-XX — Title**` (gap-registry walks); chat shorthand is not a valid identifier in artifacts. Filed as Finding 2 below (Resolved-in-session — discipline correction; forward-only per G-89 leaves the historical references in place).

The operator also raised a discipline question: prior reviews (specifically Reviews 63 and 66) over-promoted Resolved-in-session findings to suite-side G-IDs when the governing standard in `suite-development.md` § Suite review entry format reserves `### New gap registered` (and its G-ID assignment) for findings that need ongoing tracking — i.e., Deferred / Open findings, not Resolved-in-session. Review 65 was correct (G-170/G-171/G-172 for Deferred only); Reviews 63 (G-160–G-167 for 8 Resolved findings) and 66 (G-173–G-176 for 4 Resolved findings) over-promoted. Forward-only correction per G-89: historical G-IDs stay as committed records; **going forward, only Deferred / Open findings get G-IDs**. This review's Resolved finding (the discipline observation itself) does NOT get a G-ID.

**Lens:** Adversarial-question-against-recent-work lens (closely related to a cold review but driven by operator question rather than fresh-session pass). Single defect-class: structural coherence of the Phase 5 record artifacts; whether they earn their duplication of the existing per-domain review log shape.

**Session note:** In-session. Partial-isolation tradeoff: the same author who wrote the Phase 5 primer (Reviews 64–66) is now filing a structural finding against that primer. Compensation: the finding is grounded in concrete evidence from a real project application (bookmark-cli's PHASE-5-LOG.md Layer 1 entry contained ~50 lines of disposition narrative that also appears in QE Review 2 + SA Review 1; the duplication is observable, not narrative). The operator's question is the cold-reader signal — a reader unfamiliar with the Phase 5 primer's prescription asked the question my primer-internal logic did not.

**Source:** `director-raised` — operator's adversarial question against the Phase 5 record structure in bookmark-cli; the underlying finding is structural and was implicit in the primer's design but not surfaced as a defect until the operator named it.

---

### Resolved

**Finding 1 — Discipline drift on G-ID assignment in suite reviews 63 and 66 (corrected forward-only)**

`suite-development.md` § Suite review entry format already specifies that `### New gap registered` (and G-ID assignment) is the format for findings "promoted to a tracked gap" — implying a deliberate promotion act. Resolved-in-session findings live under `### Resolved` with `**Finding N — Title**` shape per the same standard; no G-ID required. Review 65 followed this discipline correctly (G-170/G-171/G-172 for the 3 Deferred findings; 9 Resolved findings as `**Finding N**` without G-IDs). Reviews 63 and 66 drifted — Review 63 assigned G-160–G-169 to all 10 findings (8 Resolved + 2 Deferred); Review 66 assigned G-173–G-176 to 4 Resolved findings. The historical G-IDs are committed; forward-only G-89 applies — they stay as records of how the registry was populated at the time, even though the discipline was loose.

**Resolution:** Discipline correction applied going forward. **From Review 67 onward, only Deferred / Open findings get suite-side G-IDs**; Resolved-in-session findings are `**Finding N — Title**` entries in the per-review narrative without G-IDs. The `FINDINGS-INDEX.md` registry rows G-160–G-167 and G-173–G-176 remain in place (forward-only — rewriting would break the audit trail and the cross-references in committed commit messages, COMPATIBILITY rows, and PR descriptions). The `suite-development.md` text already names the discipline; no governing-standard text change is needed. This finding does NOT itself receive a G-ID (it's Resolved-in-session per the corrected discipline).

**Finding 2 — Chat-shorthand finding identifiers leaking into artifacts (corrected forward-only)**

Across the recent work cycle, chat-shorthand identifiers (`Q1`/`Q2` for operator questions, `B1`/`B2` for bookmark-cli findings surfaced in Review 66's Phase 5 test, `R1`–`R4` for primer-refinement candidates, `F1`–`F12` for cold-subagent findings) have been propagating from chat conversation into review-log artifact text, commit messages, PR descriptions, and gap-registry row narratives. The governing standard in `suite-development.md` § Suite review entry format § Finding body prescribes exactly two valid identifier forms in artifacts: `**Finding N — Title**` for new findings within a review, and `**G-XX — Title**` for gap-registry-walk entries. Chat shorthand is not a valid artifact identifier.

Most visible drift sites (committed; forward-only — not rewritten): Review 65 § Resolved used `**F10 — Title**` / `**F12 — Title**` etc. headers (should have been `Finding 1 / Finding 2`); Review 66 § Resolved referred to findings as "G-173 (R1)" through "G-176 (R4)" with the `R`-prefix carried into the SUITE-DEVELOPMENT-REVIEW row text and CHANGELOG entries; the bookmark-cli adoption commit (`ca663ac`) and PR #23 description reference "B1" and "B2" extensively. None of these can be rewritten without breaking commit-message cross-references in published commit history.

**Resolution:** Going forward, artifacts (review-log entries, FINDINGS-INDEX rows, commit messages, PR descriptions, CHANGELOG entries, COMPATIBILITY rows) use **only** the two standard identifier forms. Chat-shorthand identifiers stay in the chat conversation that produced them; their job is to be ergonomic during synchronous discussion, not to be artifact identifiers. The discipline-drift correction itself does NOT receive a G-ID per the corrected Finding 1 discipline. Cleanup applied in this Review 67 entry before commit: the earlier chat references `(Q1)`, `(Q2)`, `(F1)`, `(B1)`, `(B2)` within Review 67's own narrative have been replaced with descriptive prose anchored to the finding's content rather than the chat-shorthand label.

---

### Deferred

**G-177 — `vsdd-suite/PHASE-5-LOG.md` per-project file duplicates per-domain review logs without earning the duplication.**

The Phase 5 primer (`primers/5-formal-hardening.md` § Phase 5 log format) prescribes a per-project `vsdd-suite/PHASE-5-LOG.md` file as the Phase 5 record. Surfaces A/A.0/B/C/D are written there. But the suite's existing project-level review log structure (per `suite-development.md` § Governing standard for project-level review logs) handles Phase 3 rounds via per-domain `<DOMAIN>-REVIEW.md` + `review-log/YYYY-MM-DD-<domain>.md` files; cross-domain coordination via the `**Coordination:**` line. Phase 5 work fits the same shape — a Surface B (mutation testing) round IS a QE round; a Surface A.0 (purity boundary verification) round IS an SA round.

The bookmark-cli adoption (committed at `a33a289` on the `bookmark-cli-phase5-adoption` branch — see PR #23) demonstrates the duplication concretely: the Surface B disposition table appears in both `PHASE-5-LOG.md` Layer 1 entry and in QE Review 2 (Finding 1 body); the Surface A.0 finding similarly appears in both `PHASE-5-LOG.md` and SA Review 1 (Finding 1 body). The duplication is ~50 lines per per-session pair and grows linearly with project + layer count.

**Two resolution candidates** (which one the project director chooses determines the primer revision):

- **(a) Retire `PHASE-5-LOG.md`.** Phase 5 findings file under per-domain review logs with a `**Phase 5 surface:**` preamble tag on each round (e.g., `**Phase 5 surface:** B — mutation testing`). Phase 6's convergence record cites per-domain rounds directly via Markdown links. Cleaner; matches the existing structure; no separate file to maintain.
- **(b) Make `PHASE-5-LOG.md` an index file.** Parallel to how `SUITE-DEVELOPMENT-REVIEW.md` indexes `review-log/` files at the suite level. `PHASE-5-LOG.md` becomes a one-table-per-layer index pointing at the per-domain rounds that did the actual Phase 5 work. Per-domain rounds carry the substantive content. Slight overhead (one extra file) but easier for Phase 6's convergence-check to cite a single document.

**Trigger** (per G-130 deferral discipline): when a **second project** enters Phase 5 hardening (real evidence that the duplication compounds across projects), OR when the operator preemptively chooses one of the two resolution candidates. Observable on the second project's Phase 5 session opening.

**Cost-of-deferral:** each Phase 5 project replicates the duplication shape; per-project Phase 5 records grow to duplicate per-domain records. A future Phase 6 author has to read both `PHASE-5-LOG.md` AND the per-domain rounds to confirm consistency. The audit trail is split across two artifacts that record the same dispositions. The cost grows linearly with project + layer count; at 1 project / 1 layer (the current bookmark-cli state), the cost is small but observable; at 5 projects / 3 layers, the duplication is structural debt.

**Auto-Backlog clause:** if 2026-10-31 passes without a second project entering Phase 5, the gap auto-Backlogs and re-raises in the next suite review (the duplication may be load-bearing in ways the bookmark-cli case did not surface; revisit with new evidence at that point).

**Coordinate with:** G-54 (parent — Phase 6 convergence check cites Phase 5 records), G-55 (parent — Phase 5 primer ownership), G-89 (forward-only — current Phase 5 records under bookmark-cli's first adoption stay as committed records under the prior primer shape), G-173 (Surface A.0 multi-source check), G-174 (Surface B 5-disposition class). Bookmark-cli's PHASE-5-LOG.md (committed at `a33a289`) includes a forward reference to this gap as documentation that the structural concern is acknowledged.

**Classification:** Deferred — gap registered with named G-130 trigger (second project enters Phase 5), cost-of-deferral (per-project duplication compounds linearly), and auto-Backlog clause (2026-10-31). No in-session resolution attempted; this entry promotes the structural concern to the registry for future closure.

---

### New gap registered

- **G-177** — `PHASE-5-LOG.md` per-project file duplicates per-domain review logs — Deferred with G-130 discipline (trigger: second project enters Phase 5; auto-Backlog 2026-10-31).

---

### Coordination

The Review 67 disposition coordinates with:

- **G-55 / G-54** (Addressed Review 64) — parent gaps. G-177 surfaces a structural defect introduced by Review 64's Phase 5 primer authoring; the parent closures hold (Phase 5 + Phase 6 ownership is unchanged), this is a refinement candidate within the existing surface.
- **G-130** (Addressed prior) — deferral-trigger discipline applied (trigger + cost + auto-Backlog all named).
- **G-89** (Addressed prior) — forward-only narrative-preservation. The discipline-drift acknowledgement under § Resolved does NOT retroactively renumber the historical G-IDs from Reviews 63 and 66; the prior IDs stay as committed records.
- **PR #23 (`bookmark-cli-phase5-adoption`)** — forward references G-177 in `bookmark-cli/vsdd-suite/PHASE-5-LOG.md`. The forward reference resolves once both PRs merge (the suite-side PR registers G-177 first; the bookmark-cli PR's PHASE-5-LOG.md then cites a registered gap rather than a forward-pointing TODO).

The closures do NOT regress any prior work. The discipline-drift correction is forward-only per G-89; the G-177 deferral is purely additive (new gap; existing primer unchanged).

---

### Summary

1 Resolved finding (the discipline-drift correction; no G-ID per the corrected discipline). 1 Deferred new gap (G-177; PHASE-5-LOG.md duplication). Backlog after Review 67: **0 Open + 7 Deferred** (G-159, G-168, G-169, G-170, G-171, G-172, G-177 — all with G-130 trigger + cost + auto-Backlog). The Phase 5 / Phase 6 owned-by-suite state from Review 64 is unchanged; this review surfaces a structural refinement candidate without re-opening the parent gaps.

Sycophancy self-audit: the operator's three adversarial points (PHASE-5-LOG duplication; G-ID over-assignment; chat-shorthand identifier leakage) were all real defects, and I addressed them honestly. G-177 is against my own Review 64 primer authoring — I authored PHASE-5-LOG.md as a separate file because the primer's first-draft prescription called for it; the cold review (Review 65 by subagent) and the first-real-project test (Review 66 by Phase 5 against bookmark-cli) both failed to surface the duplication concern. The operator's adversarial question was the first lens that caught it. Finding 1 (G-ID discipline) and Finding 2 (chat-shorthand identifiers) are process drifts in how I've been populating the registry and the review-log artifacts — the governing standard in `suite-development.md` already specified the correct disciplines; I drifted from them without noticing. This is the same shape as the suite's overall sycophancy discipline at the project level (the director's manual testing catches what cold-batch IAR misses, per G-132); applied here at the suite level via three director-raised observations rather than a cold review pass. All three are honest near-misses, none comfortable to flag against my own recent work.

---

## Review 68 — 2026-05-20 04:30Z

**Scope:** Operator-directed adversarial review of the vsdd-suite focused on **drift** — places where the suite's own implementation has departed from its own governing standards, conventions, or methodology promises. Conducted via cold subagent (spawned in isolation from the in-session author who produced Reviews 62–67) applying 5 drift lenses: discipline-vs-practice; convention drift; cross-artifact consistency; methodology fidelity; forward-only narrative preservation. Subagent surfaced 11 findings, all real (none Hallucinated); 10 Resolved in-session (1 by Dismissal as intentional distinction; 4 by standard / primer / README text updates; 5 by the new `check-suite-review-preamble.sh` hook + errata blocks under the G-89 forward-only narrative-preservation mechanism); 0 Deferred (the prevention mechanism is the disposition for the going-forward cases; the historical drift instances are preserved per G-89 with errata).

**Lens:** Cold-subagent drift-detection pass. Composes 5 drift lenses serially; the discipline-vs-practice lens produced the most findings (6 of 11), consistent with the recent operator pattern of catching the same author drifting from standards the author wrote.

**Session note:** In-session director triage of subagent findings, partial-isolation acknowledged. The subagent's findings are evidence-based with file:line citations; the in-session director (same author who produced Reviews 62–67) classifies and resolves. Compensation: every Resolved finding's resolution is a concrete artifact change (the new hook; text updates to `suite-development.md` / primer 3 / README; errata blocks per the sanctioned G-89 mechanism), not a narrative dismissal. The sycophancy risk is highest for Finding 5 (Coordination heading collision — Dismissed as intentional distinction); the Dismissal rationale is concrete (the standard text DOES distinguish the two contexts; no observed mis-use) but a cold reviewer might still flag it.

**Source:** mixed — subagent findings are `domain-raised` (the cold adversary applying drift lenses to artifacts surfaced the findings); the operator's "Conduct an adversarial suite review focused on finding and preventing drift" instruction is `director-raised` at the session-opening level.

---

### Resolved

**Finding 1 — `**Source:**` field missing from Reviews 52–61 despite being Required-for-all-domains per G-133 (suite-development.md:275)**

Subagent identified that the G-133 closure (Review 59, 2026-05-19 06:00Z) made `**Source:**` a Required-for-all-domains preamble field, but Reviews 52, 53, 54, 55, 56, 57, 58, 59, 60, 61 (10 reviews dated after G-133's closure window) omit the line. Reviews 62–67 do include it. Compliance: 6/16 post-G-133 reviews. The omission degrades the audit-trail-granularity signal G-133 was registered to produce.

**Resolution:** The historical reviews stay non-compliant per G-89 forward-only narrative-preservation (rewriting 10 review-log entries to insert Source lines would break the audit trail). Going-forward enforcement is the new `vsdd-suite/hooks/check-suite-review-preamble.sh` hook (committed this review): every entry dated 2026-05-20 or later must include `**Source:**`. The hook's `ENFORCEMENT_THRESHOLD` constant carries the forward-only date. Future drift cannot recur silently.

**Finding 2 — Review 65 § Resolved finding headers use `**F10 — Title**`, `**F12 — Title**`, etc. (chat-shorthand instead of `**Finding N — Title**`)**

Subagent identified that Review 65's 9 Resolved finding headers use the cold-subagent's `F`-prefix labels rather than the standard `**Finding N — Title**` form prescribed by `suite-development.md:317` / `:428`. Review 67 Finding 2 corrected the discipline forward-only but left the committed instances in place.

**Resolution:** Per the sanctioned errata mechanism (`suite-development.md:429`: "Follow-up findings introduced after the session has been logged must be marked `**Finding M — Title (added YYYY-MM-DD)**` and placed at the end of the original entry"), added `**Finding 13 — Cross-reference map: chat-shorthand F-prefix labels in `### Resolved` (added 2026-05-20)**` at the end of Review 65 with a F-prefix → Finding-N cross-reference table covering all 9 Resolved findings plus the 3 Deferred (which map to G-170/G-171/G-172). Future readers encountering `F1`/`F2`/etc. citations now have an artifact-level resolution path. Original entry text unchanged per G-89. `<!-- hook-bypass: ... -->` HTML comment added in the first 5 lines of Review 65 so the new hook does not flag the original non-standard headers (the bypass itself documents the rationale + cross-references the errata block).

**Finding 3 — Review 66 § Resolved finding headers use `**R1 / G-173 — Title**`, etc. (compound chat-shorthand + over-promoted G-IDs)**

Subagent identified Review 66's 4 Resolved finding headers as triple drift: (a) `R`-prefix chat shorthand; (b) Resolved-in-session over-promotion to G-IDs (corrected per Review 67 Finding 1); (c) compound `Rn / G-XXX` form combining both into a non-standard header shape.

**Resolution:** Same errata + bypass mechanism as Finding 2. Added `**Finding 5 — Cross-reference map: chat-shorthand R-prefix labels in `### Resolved` + G-ID over-promotion (added 2026-05-20)**` at the end of Review 66 with a R-prefix → Finding-N + G-ID cross-reference table covering all 4 Resolved findings. The G-IDs (G-173–G-176) stay registered in FINDINGS-INDEX.md as committed records per G-89. `<!-- hook-bypass: ... -->` HTML comment added in the first 5 lines of Review 66.

**Finding 4 — Review 65 + Review 66 findings lack the required closer line (`**Resolution:**` or `**Classification:**`) per suite-development.md:333 + :428**

Subagent identified that the governing standard mandates every finding body end with exactly one of `**Resolution:**` (Resolved) or `**Classification:**` (everything else), and that Reviews 65 + 66 entirely omit these closers — they describe what changed in prose under the header but never tag the closer with the standard's bold-line form. Review 67's findings DO include the closers.

**Resolution:** Reviews 65 + 66 stay non-compliant per G-89 forward-only (10+ findings to retroactively patch; the errata blocks added for Findings 2 + 3 above implicitly cite the corrected discipline). Review 67's G-177 was missing a `**Classification:**` line; added `**Classification:** Deferred — gap registered with named G-130 trigger ...` in this review session. The new `check-suite-review-preamble.sh` hook enforces closer presence going forward for any entry dated 2026-05-20 or later that isn't bypassed.

**Finding 5 — `### Coordination` H3 (suite-review entries) vs `**Coordination:**` bold-line (project-level closing block) naming collision**

Subagent identified that the same word "Coordination" points at two different structural elements depending on artifact type: `suite-development.md:340` prescribes `**Coordination:**` as a bold-line for project-level review log closing blocks; `:429` separately authorizes `### Coordination` H3 for suite-review entries. Both conventions are coherent if a reader finds both passages but the naming collision can confuse a contributor importing one pattern into the other context.

**Classification:** Dismissed — the standard text DOES distinguish the two contexts (project-level § Closing block at :340 vs. suite-review § Suite review entry format at :429); no observed mis-use of one convention in the wrong artifact-type. The distinction is intentional: suite-review entries are richer (warrant H3) while project-level closing blocks are concise (warrant a bold line). Renaming either to `### Coordination (suite-review)` etc. would add ceremony without solving a real-world ambiguity. The Dismissal rationale is the standard's textual distinction; if a future contributor mis-applies the convention, that's the trigger to revisit.

**Finding 6 — Reviews 53 + 55 carry bare `**Session note:** In-session.` with no compensation named, violating the G-133-era discipline at suite-development.md:435**

Subagent identified that `suite-development.md:435` makes session-note-compensation a structural-error grade requirement ("itself a finding for VDD-IAR Alignment dim 7"), but Reviews 53 and 55 (both post-G-133) have bare `**Session note:** In-session.` lines.

**Resolution:** Same shape as Finding 1 — historical instances stay per G-89; going-forward enforcement is the new hook. The hook's preamble check verifies `**Session note:**` exists but does not currently parse the prose for compensation language (that would require natural-language analysis). Documented as a hook-enhancement candidate; for now the hook + the in-session director's review-time scrutiny are the prevention mechanisms.

**Finding 7 — Reviews 62 + 66 register new gaps (G-159 / G-173–G-176) without the `### New gap registered` classification section per suite-development.md:427**

Subagent identified the discipline gap; the hook's check 5 (verify `### New gap registered` section exists when a `**G-XX — Title**` header is present) was drafted but dropped from the v1 hook implementation because distinguishing new-gap registrations from gap-registry-walks (G-ID headers that address already-tracked gaps, e.g., Review 64's G-54 / G-55 closures) requires reading the registry — out of scope for a v1 hook.

**Resolution:** Section-presence discipline is advisory-grade for the hook's v1; the standard text at :427 remains the authority. Going-forward hook enhancement: load `FINDINGS-INDEX.md` and verify per-G-ID that any G-XX header is either (a) addressing a registered gap whose status row exists OR (b) registering a new gap with both a row in FINDINGS-INDEX and a `### New gap registered` section in the review log. Deferred to future hook iteration; not blocking Review 68's closure.

**Finding 8 — Per-review preamble field ordering not standardized**

Subagent identified that the standard at `:272-284` lists required fields without explicit ordering; de-facto ordering in Reviews 62–67 is Scope → Lens → Session note → Source, but the standard's text order is Scope → Session note → Source (with Lens in a different sub-section applicable only to suite reviews).

**Resolution:** Added explicit "**Canonical ordering** (Review 68 Finding 8 clarification)" paragraph to `suite-development.md` § Per-review entry preamble naming the canonical order for project-level vs. suite-review entries. The hook does not enforce ordering — it checks field presence — but the standard now names the convention; future reviewers have a textual reference.

**Finding 9 — Reviews 63 + 66 use unauthorized `**Source:** mixed` value not enumerated in suite-development.md:275**

Subagent identified that G-133 enumerates 4 valid Source values; Reviews 63 + 66 use a `mixed` composite that's not in the list. The composite is honest about reality (a director-raised session that produced domain-raised findings is mixed-source), but the value is non-standard.

**Resolution:** Extended the standard at `:275` to include `mixed` as the 5th valid Source value, with explicit sub-disposition guidance ("The `mixed` value requires the Source line to name the sub-disposition explicitly"). The hook's `VALID_SOURCE_VALUES` constant includes `mixed`. Reviews 63 + 66 are now retroactively-compliant against the extended standard; the standard's text now authorizes the value they used.

**Finding 10 — Phase label says `Phase 1a+1b` but route label string is `route:phase-1a`; no inline rationale on the user-facing surface**

Subagent identified the cross-artifact consistency drift: README + primers reference `Phase 1a+1b` (renamed per G-160) but the route-label string `route:phase-1a` was deliberately kept stable (per G-160 closure: "labels are identifiers, not scope-descriptors"). The rationale is documented in FINDINGS-INDEX:G-160 but invisible at the user-facing call sites in README.md and primers/4-feedback-integration.md.

**Resolution:** Added inline "**Label-vs-phase naming note (Review 68 Finding 10)**" paragraph at the route-label introduction in `vsdd-suite/README.md` (between the `**[crosslink]** Use route labels` lead-in and the first code block) naming the convention: label string stays stable per G-160; phase name evolves in docs. Future readers hit the explanation in-page rather than chasing the gap registry.

**Finding 11 — `**Source:**` field is mandatory for per-review entries but `primers/3-review-session.md` (the adversarial-review primer) does not mention G-133 or the Source field**

Subagent identified the methodology-fidelity drift: a reviewer following primer 3 verbatim produces a per-review entry that violates the G-133 mandate, because primer 3 (the cold session's loaded primer) never names the Source field.

**Resolution:** Added new `## Source attribution (G-133 / Review 68 Finding 11)` section to `primers/3-review-session.md` after `## After each domain review` (where the classification schema is discussed) and before `## If reviewing the IAR suite itself`. The new section names the 5 valid Source values (including the `mixed` extension from Finding 9) and prescribes the per-session classification path. A cold reviewer loading only primer 3 now encounters the Source-attribution discipline without needing to read suite-development.md.

---

### New gap registered

*(none — every finding Resolved or Dismissed in-session; the prevention-mechanism hook is the disposition for the going-forward cases of Findings 1, 2, 3, 4, 6.)*

---

### Coordination

The Review 68 closures coordinate with:

- **G-89** (Addressed prior) — forward-only narrative-preservation policy. Reviews 52–61 (Finding 1) and Reviews 53 + 55 (Finding 6) and Reviews 65 + 66 (Findings 2, 3, 4) historical drift stays as committed records; the new hook enforces forward-only.
- **G-129** (Addressed prior, `check-changelog-currency.sh`) and **G-139** (Addressed prior, `check-crosslink-references.sh`) — precedent for shipping a discipline-mechanizing hook as a suite artifact rather than as advisory text. `check-suite-review-preamble.sh` follows the same pattern: discipline-then-recurrence-then-hook.
- **G-133** (Addressed Review 59) — Source field discipline. Findings 1 + 9 + 11 are all G-133-related: Finding 1 catches retroactive non-compliance forward-only via the hook; Finding 9 extends the value set; Finding 11 adds the field to primer 3 where the discipline is operationally needed.
- **G-160 / G-162 / G-167** (Addressed Reviews 63 / 63 / 63 respectively) — Finding 10 cross-references G-160's "labels are identifiers" framing; Finding 5 dismissal cross-references G-162's strategy-declaration discipline (which uses similar context-sensitive naming patterns); the hook's overall approach mirrors G-167's known-limitations cataloging.
- **Review 67 Findings 1 + 2** (Resolved prior in this same file) — Findings 2 + 3 of this review are the historical-instance cleanup for Review 67's forward-only discipline corrections; the errata + bypass mechanism is the sanctioned-by-G-89 path.

The Review 68 closures do NOT regress prior work. The new hook is additive (no existing hook modified). The standard text updates extend rather than narrow (canonical ordering paragraph adds clarification; `mixed` Source value adds a 5th option without invalidating the prior 4). The errata blocks in Reviews 65 + 66 add cross-reference content without modifying original entries. The bypass HTML comments in Reviews 65 + 66 are metadata that activates only when the hook runs.

---

### Summary

11 findings classified: 10 Resolved in-session (1 by Dismissal; 9 by concrete artifact changes); 0 Deferred (the prevention-mechanism hook is the disposition for going-forward cases). Backlog after Review 68: **0 Open + 7 Deferred** (G-159, G-168, G-169, G-170, G-171, G-172, G-177 — unchanged from Review 67). No new gaps registered.

**Prevention mechanism shipped:** `vsdd-suite/hooks/check-suite-review-preamble.sh` (a 200-line Python pre-commit hook) is the suite's fourth discipline-mechanizing hook (joining `check-crosslink-references.sh` G-139, `check-changelog-currency.sh` G-129, `check-review-log-anonymization.sh` G-98). Forward-only `ENFORCEMENT_THRESHOLD` of `2026-05-20`; per-entry `<!-- hook-bypass: <rationale> -->` HTML comment escape valve (used for Reviews 65 + 66 in this review per G-89 compliance). The hook composes with the existing three via `.pre-commit-config.yaml`.

Sycophancy self-audit: the subagent's drift findings were largely impossible to dismiss after file:line citation (the standard text vs. practice mismatch is observable mechanically). The temptation I caught myself nearly succumbing to: classifying more findings as Deferred than Resolved-in-session, on the theory that "the hook resolves them going forward." That framing is right for new artifact-level enforcement but it isn't right for the standard-text gaps Findings 8/9/10/11 surfaced — those needed text changes, not hook coverage. I forced myself to make the text changes inline rather than punt to a future Review. The hook + errata + bypass + standard-text-extensions combination is the right disposition for this class of drift; punting would have been the same failure mode the discipline was registered against. The Dismissal of Finding 5 is the only finding I'm uncertain about; if the operator disagrees with the Dismissal rationale, the appropriate next move is to extend the standard text at :340 + :429 with the explicit "Coordination (project-level)" / "Coordination (suite-review)" naming distinction.
