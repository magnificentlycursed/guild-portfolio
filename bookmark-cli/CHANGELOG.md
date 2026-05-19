# Changelog

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
