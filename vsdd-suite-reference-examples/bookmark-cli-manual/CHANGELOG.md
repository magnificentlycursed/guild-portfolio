# Changelog

## vsdd-suite v0.11.0 capstone-intent promotion + 6-phase preparation — 2026-05-20 16:30Z (PR 6 / Review 78)

**Scope:** Promotes `bookmark-cli-manual` from `portfolio` intent to `capstone` intent. Reference examples track current conventions per G-177 precedent — bookmark-cli-manual exists to teach the worked example end-to-end through all 6 VSDD phases, which requires capstone-intent bar. This PR lands the STRUCTURAL preparation; the cold-session IAR rounds for the newly-activated capstone domains + the Phase 6 four-dimensional convergence record + FINDINGS-INDEX repopulation land in **PR 7**.

### Changed

- **`DESIGN.md`** § Project intent — promoted to `capstone` with intent-transition note. Active domain set expanded from 7 (portfolio default) to **10 role + 1 meta = 11 domains** for capstone: 6 core role (SE, QE, UX, Security, SA, SO) + VDD-IAR Alignment meta + 4 extended (Performance Engineer for capstone-required activation; Platform Engineer for G-178 strong-presumption + G-155 dim 38 fresh-system install verification; Red Team for capstone-tier adversarial intensity; Technical Writer for portfolio+ external-reading activation given bookmark-cli-manual's reference-example role). Data Engineer evaluated and ruled out — bookmark-cli's flat JSON storage falls below the G-178 activation threshold; the absence is documented as deliberate. Sanity Check meta domain (Review 77 Finding 2) available on-demand for findings without natural cross-domain pair. Historical portfolio-intent declaration (Review 67) preserved as the historical-narrative anchor below the current declaration per G-89 forward-only.
- **`DESIGN.md`** § Project intent Phase 6 strategy — promoted from `not applicable` to `planned` with concrete scope naming the four dimensions + the cross-dimension consistency check + the signed closing attestation. Per G-162: capstone-intent declarations require both Phase 5 + Phase 6 strategy lines; both now declared.
- **`TODO.md`** Layer 1 — `**Manual Testing Checklist:**` block split out per Review 74 convention (the inline runnable-step shell-script block became a one-line pointer to the new per-layer file). Layer-gate criteria expanded from 4 to 6 reflecting capstone-active domain set + Phase 5 + Phase 6.
- **`TODO.md`** Layer 1 — new **Phase 2c (refactor):** annotation declaring `no refactor required` per `primers/2c-refactor.md` § Completion criteria #5 explicit-skip pattern. Layer 1's implementation already exhibits the refactor primer's scope-catalog idioms; explicit-skip annotation satisfies VDD-IAR Alignment dim 12 (Phase 2c refactor discipline per G-161).
- **`vsdd-suite/review-log/2026-05-17-quality-engineer.md`** Reviews 1's Findings 1+2 + **`vsdd-suite/review-log/2026-05-20-quality-engineer.md`** Review 2 Finding 1 + **`vsdd-suite/review-log/2026-05-20-solution-architect.md`** Review 1 Finding 1 — migrated with Review 77 lifecycle fields (`**Owner:**` / `**Status:**` / `**Blocked by:**` / `**Validator:**`) per G-177 reference-example-migrates precedent. Each file gained a migration-note paragraph documenting the retroactive field addition. Hallucinated findings (QE Review 1 Finding 3) exempt from lifecycle fields per Review 77. The fields are aspirational on these pre-2026-05-21 dates (the hook's lifecycle-field enforcement gates on 2026-05-21+); the next-day Review-77-enforced rounds in PR 7 will carry the fields under the enforced standard.
- **`vsdd-suite/FINDINGS-INDEX.md`** — schema extended with `Owner` + `Validator` columns per Review 77; existing 5 rows updated with migrated Owner/Validator values per the per-round migration above. Quick-lookup section retained.

### Added

- **`manual-tests/layer-1.md`** (new ~140 lines) — Layer 1 manual-test plan split out from `TODO.md` per Review 74 convention. 6 step blocks (Step 0 binary install / Step 1 happy path / Step 2 error state / Step 3 list ordering / Step 4 empty-state / Step 5 persistence with uninstall+reinstall lifecycle) with literal expected-output blocks per the runnable-step standard. Closure protocol section at end naming the two outcome shapes (insight-reached/no findings vs findings-surfaced) per primer 3 § Manual testing is a second adversarial surface to IAR (G-132).
- **`PROCESS.md`** (new ~80 lines) — first-person retrospective skeleton per G-156 layer-gate close criterion 7. **AI-co-authored reference-example disclosure** at the top makes explicit that the discipline G-156 specifies (director-authored prose) is NOT satisfied by AI-authored scaffold prose; the file demonstrates the FORMAT for an actual capstone project, with section structure (What was hardest / What I got wrong / What the process felt like per layer) but with AI-authored placeholder prose that must be overwritten by the director to satisfy the gate. The disclosure complies with the operator's earlier directive on AI-co-authored artifacts.
- **`manual-tests/install-verification.md`** (new ~70 lines) — Platform Engineer Dim 38 third-party install verification record per G-155. **AI-co-authored disclosure** at top makes explicit that the AI cannot satisfy this gate (the discipline's load-bearing requirement is non-author verification on a fresh system); the file documents the verification procedure the operator would execute. Verification table is scaffolded with pending row. **File location convention (Review 78 Finding 2):** lives in `manual-tests/` because install-verification IS a manual test (operator runs commands on a fresh system and records observations); lowercased + hyphenated filename (`install-verification.md`) parallels the per-layer pattern (`layer-N.md`).
- **`vsdd-suite/PERFORMANCE-ENGINEER-REVIEW.md`** + **`vsdd-suite/PLATFORM-ENGINEER-REVIEW.md`** + **`vsdd-suite/RED-TEAM-REVIEW.md`** + **`vsdd-suite/TECHNICAL-WRITER-REVIEW.md`** (4 new files) — per-domain index files for the 4 newly-capstone-activated extended domains. Each file is customized for bookmark-cli-manual with activation rationale + supplement references + sycophancy-check excerpt + empty Reviews table (rounds populate in PR 7).
- **`vsdd-suite/SOFTWARE-ENGINEER-REVIEW.md`** + **`vsdd-suite/UX-REVIEW.md`** + **`vsdd-suite/SECURITY-REVIEW.md`** + **`vsdd-suite/SOLUTION-OWNER-REVIEW.md`** + **`vsdd-suite/VDD-IAR-ALIGNMENT-REVIEW.md`** — the 5 pre-existing scaffolded stubs were customized for bookmark-cli-manual (template placeholders filled with domain-specific values; reading convention + Reviews table sections completed).

### Note

**Backlog after PR 6: 0 Open findings.** The cold-session rounds for the 9 not-yet-reviewed-at-capstone domains (SE, UX, Security, SO, VDD-IAR Alignment + Performance Engineer, Platform Engineer, Red Team, Technical Writer) + Phase 6 four-dimensional convergence record + FINDINGS-INDEX row repopulation land in **PR 7**. The structural preparation here (DESIGN.md intent declaration; manual-test split; existing-rounds migration; per-domain index scaffolds; PROCESS.md + `manual-tests/install-verification.md` skeletons; FINDINGS-INDEX schema migration) is reviewable in isolation. PR 7 will execute the IAR rounds against the prepared structure. Forward-only per G-89: pre-2026-05-21 reviews (QE R1, QE R2, SA R1) retain their original framings under portfolio intent + are augmented with Review 77 lifecycle fields per the operator's migration directive.

---

## vsdd-suite v0.7.8 migration — 2026-05-20

**Scope:** Migrates `bookmark-cli` from the prior `vsdd-suite/PHASE-5-LOG.md` per-project file shape to the per-domain review log shape that vsdd-suite v0.7.8 prescribes (G-177 closure — operator-promoted from Deferred to Addressed). bookmark-cli is the suite's reference example, so it tracks the current convention rather than the forward-only carve-out.

### Removed

- **`vsdd-suite/PHASE-5-LOG.md`** (deleted) — the per-project Phase 5 record. All substantive content (purity-boundary audit table; cargo-mutants pre-B1 / post-B1 outputs; per-mutant disposition table; strategy declaration) was already mirrored in the per-domain rounds at `vsdd-suite/review-log/2026-05-20-solution-architect.md#review-1` (Surface A.0) and `vsdd-suite/review-log/2026-05-20-quality-engineer.md#review-2` (Surface B). The deletion eliminates the duplication G-177 named.

### Changed

- **`vsdd-suite/review-log/2026-05-20-solution-architect.md#review-1`** — added `**Phase 5 surface:** A.0 — purity-boundary verification for Layer 1` preamble tag per G-177 v0.7.8 convention. Removed the `../PHASE-5-LOG.md` cross-references from the Scope line and Coordination line.
- **`vsdd-suite/review-log/2026-05-20-quality-engineer.md#review-2`** — added `**Phase 5 surface:** B — mutation testing for Layer 1 via cargo-mutants` preamble tag. Removed the `../PHASE-5-LOG.md` cross-references from the Scope line, the unviable-mutants paragraph, and the Coordination line.
- **`vsdd-suite/QUALITY-ENGINEER-REVIEW.md`** — Reviews-table row for Review 2 reworded to name the `**Phase 5 surface:** B` preamble explicitly and remove the `../PHASE-5-LOG.md` final-sentence citation.
- **`DESIGN.md`** — § Project intent Phase 5 strategy line + § Verification architecture Phase 5 hardening bullet reworded to cite the per-domain logs (SOLUTION-ARCHITECT-REVIEW.md for Surfaces A/A.0/D; QUALITY-ENGINEER-REVIEW.md for Surfaces B/C) instead of `vsdd-suite/PHASE-5-LOG.md`.
- **`src/lib.rs::tests::save_creates_parent_directory_for_nested_path`** — doc comment updated to cite `vsdd-suite/QUALITY-ENGINEER-REVIEW.md` (Review 2 — Phase 5 Surface B) instead of `vsdd-suite/PHASE-5-LOG.md`.

### Note
Prior CHANGELOG entries that reference `vsdd-suite/PHASE-5-LOG.md` (the v0.7.2 adoption entry below) are preserved as historical-narrative records per G-89 forward-only narrative-preservation policy. The references in those entries reflect the state at the time of writing; the current state is described in this entry.

No `PHASE-6-CONVERGENCE.md` ever existed on this project (bookmark-cli is portfolio-intent with `Phase 6 strategy: not applicable`); no Phase 6 migration is required.

---

## v0.7.2 adoption + Phase 5 Layer 1 hardening — 2026-05-20 02:45Z

**Scope:** Adopts the vsdd-suite v0.7.2 conventions (G-150 intent calibration, G-162 Phase 5/6 strategy declarations, G-55 Phase 5 ownership, G-54 Phase 6 ownership). Closes the 2 held findings (B1, B2) from the Review 66 Phase 5 test that had been blocked pending operator authorization on the reference impl. Lands on dedicated branch `bookmark-cli-phase5-adoption`.

### Added

- **`DESIGN.md`** — new `## Project intent` section declaring `portfolio` intent + `**Phase 5 strategy:**` (planned — Surfaces A.0 + B; A deferred; C + D not applicable) + `**Phase 6 strategy:**` (not applicable — portfolio-intent closes at end of Phase 4 by design).
- **`DESIGN.md` § Verification architecture** — new explicit Purity boundary subsection enumerating each function's purity status (pure: data types + `newest_first`; effectful: `load`/`save` I/O wrappers; boundary refinement: `add` clock-read). New Phase 5 hardening reference pointing at `vsdd-suite/PHASE-5-LOG.md`. New formal-proof-candidates declaration (none). New automatable-vs-manual split. Resolves B2 (cross-source purity divergence).
- **`vsdd-suite/PHASE-5-LOG.md`** (new file) — per-layer Phase 5 hardening record per the v0.7.2 primer format. Documents the Layer 1 Surface A.0 + B run: tool-install cost; full pre-B1 and post-B1 cargo-mutants output; per-mutant disposition table (5-disposition universe per G-174); Surfaces A + C + D dispositions per the project's strategy.
- **`vsdd-suite/review-log/2026-05-20-quality-engineer.md`** (new file) — QE Review 2 entry filing the Surface B (mutation testing) portion of the Phase 5 round per the suite's project-level review log governing standard. One Resolved finding (Surface B missing-test); coordination line points at SA Review 1 for the Surface A.0 portion.
- **`vsdd-suite/review-log/2026-05-20-solution-architect.md`** (new file) — SA Review 1 entry filing the Surface A.0 (purity-boundary verification) portion of the Phase 5 round. One Resolved finding (Dim 12 — VSDD purity boundary map cross-source divergence). First SA review filed against bookmark-cli; coordination line points at QE Review 2 for the Surface B portion.
- **`vsdd-suite/SOLUTION-ARCHITECT-REVIEW.md`** — index customized from the scaffolded stub (Reviewer role line, sycophancy check, language supplement reference filled in; first row added for SA Review 1). Prior state was placeholders.
- **`vsdd-suite/FINDINGS-INDEX.md`** — new rows F-004 (SA Dim 12 purity-boundary divergence, Resolved) and F-005 (QE Surface B missing-test, Resolved); Cross-references list updated to note SA-REVIEW.md is now customized.
- **`src/lib.rs::tests::save_creates_parent_directory_for_nested_path`** — falsifying test for the previously-surviving cargo-mutants mutant at `src/lib.rs:48`. Labeled `retroactive Red Gate (Phase 5 source)` per the Review 65 / F7 label extension in `primers/2b-implementation.md`. Verified end-to-end: post-B1 cargo-mutants kill rate on viable mutants is 8/8 = 100% (was 7/8 = 87.5% pre-B1). Resolves B1 (test gap).

### Changed

- **`src/lib.rs:1-7`** module doc — retired the prior "Pure-core storage logic" claim; cites `DESIGN.md` § Verification architecture as the single authoritative source. Module-doc summary names the impl's actual purity boundary (pure / effectful / boundary-refinement categories) matching DESIGN.md. Resolves the cross-source divergence half of B2 (G-173 multi-source check).
- **`DESIGN.md`** H1 / preamble — updated from "Phase 1a contract" to "Phase 1a+1b contract" per the G-96 / G-160 v0.6.0 rename; cites the renamed primer at `primers/1ab-spec-crystallization.md`. Forward-only narrative-preservation per G-89: the file was originally authored under the prior "Phase 1a" single-step naming and that historical narrative is preserved.
- **`vsdd-suite/QUALITY-ENGINEER-REVIEW.md`** Reviews table — new row for Review 2 (the Phase 5 round).
- **`.gitignore`** — added `mutants.out` and `mutants.out.old` (cargo-mutants output directories).

### Note

`bookmark-cli` is the suite's reference implementation; the v0.7.2 adoption above is the first project-side application of the Phase 5 + Phase 6 methodology since G-54 + G-55 closed in suite Review 64. The Phase 5 primer's prescriptions worked as written for both surfaces activated (A.0 audit produced a real cross-source finding; B disposition table mapped cleanly to cargo-mutants' actual output shape including the 5th `unviable` class added in suite Review 66 / G-174). The forward-only G-89 carve-out applies: bookmark-cli's Layer 1 first-gate-close predates 2026-05-20; this adoption is an explicit opt-in, not retroactive enforcement.
