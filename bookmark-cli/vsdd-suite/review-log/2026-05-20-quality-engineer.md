# Quality Engineer Review — 2026-05-20

[Index](../QUALITY-ENGINEER-REVIEW.md)

---

## Review 2 — 2026-05-20 02:45Z

**Scope:** Phase 5 (Formal Hardening) Layer 1 hardening round — **QE-scope only**. This entry covers Surface B (mutation testing via cargo-mutants) which is QE territory (QE Dim 2 — test falsifiability; suite Review 66 G-174 5-disposition universe). The Surface A.0 (purity-boundary verification) work landed during the same Phase 5 session belongs to **SA Dim 12** (VSDD purity boundary map) and is filed under [SA Review 1](2026-05-20-solution-architect.md#review-1--2026-05-20-0245z) — see Coordination below. First Phase 5 session against bookmark-cli per the v0.7.2 adoption (`bookmark-cli-phase5-adoption` branch). Surfaces A.0 + B activated this session; A deferred; C + D not applicable per the project's Phase 5 strategy in [`../DESIGN.md`](../DESIGN.md) § Project intent. Cross-domain hardening record at [`../PHASE-5-LOG.md`](../PHASE-5-LOG.md) Layer 1 entry.

**Session note:** In-session (the same operator authored the Phase 5 primer revisions in Review 64–66 and now applies them to bookmark-cli). Sycophancy-compensation: the Phase 5 Surface B sycophancy check (G-174 5-disposition discipline) applied to my own dispositions — every surviving mutant got a per-mutant outcome (no aggregate-only reporting); the missing-test mutant routed via disposition (b) with the `retroactive Red Gate (Phase 5 source)` label per `primers/2b-implementation.md` (Review 65 / F7 label extension) rather than being silently rationalized as equivalent. Cold-context equivalent would be stronger; running the Phase 5 primer's prescriptions against a real project (concrete kill-rate output; concrete divergence finding) provided enough artifact-level evidence to compensate for the in-session lens.

**Source:** `domain-raised` — the Phase 5 surfaces (A.0 audit, cargo-mutants execution) raised both findings against the implementation by applying the suite-level Phase 5 primer's dimensions; the operator-raised v0.7.2 adoption decision is `director-raised` at the session-opening level.

**Assumption surfacing:** Verified `cargo-mutants v27.0.0` is the installed version (matches the Review 66 G-167 / crosslink-contract.md § Known limitations test environment); installed via `cargo install cargo-mutants --locked` — 1m 39s wall time (the G-175 tool-install-cost discipline applied: install time recorded separately from hardening time). No other tool installs required this session.

---

### Resolved

**Finding 1 — Surface B surviving non-equivalent mutant (Phase 5 Surface B / G-174 5-disposition universe)**

`cargo-mutants v27.0.0` Surface B against the Phase 3-MVR codebase produced 11 mutants. Pre-B1 outcome: 7 caught / 1 missed / 3 unviable — kill rate on viable mutants 7/8 = **87.5%**.

The surviving mutant at `src/lib.rs:48:16` mutated `if !parent.as_os_str().is_empty()` (in `BookmarkStore::save`) to `if parent.as_os_str().is_empty()` — flipping when `create_dir_all` runs. The original guards `create_dir_all` only when there IS a parent dir component; the mutated version runs `create_dir_all` when there ISN'T one (and would fail-and-skip on paths with a non-existent parent).

**Behavior analysis (per Surface B sycophancy check (a) equivalence check):**
- Path `foo.json` (no parent dir): original skips; mutant tries `create_dir_all("")` (no-op-ish).
- Path `nested/foo.json` (parent doesn't exist): original creates `nested/`; mutant skips, then `std::fs::write` fails because `nested/` doesn't exist.

The mutant is NOT behavior-equivalent — it diverges on the nested-path case. The pre-B1 test suite (`tests/bookmarks.rs` integration tests + `src/lib.rs` `#[cfg(test)] mod tests` block) exercised only paths whose parent directory already existed (via `tempfile::tempdir()`), missing this case.

**Disposition (per Phase 5 Surface B sycophancy check option (b)):** test gap. The implementation is correct; the test suite was missing the falsifying case.

**Resolution:** Added `src/lib.rs::tests::save_creates_parent_directory_for_nested_path` with the doc-comment label `retroactive Red Gate (Phase 5 source): save creates parent directory for a nested-path target — Surface B (cargo-mutants) surfaced the gap at src/lib.rs:48 where ...` per the Review 65 F7 label extension. The test asserts: (a) parent doesn't exist pre-save; (b) save succeeds; (c) parent exists post-save; (d) the saved file round-trips correctly via `BookmarkStore::load`.

Post-B1 cargo-mutants re-run: **0 missed / 8 caught / 3 unviable** — kill rate on viable mutants 8/8 = **100%**. The previously-surviving mutant is now in `mutants.out/caught.txt`; verified end-to-end in-session. The unviable mutants (line 55 `+` → `-`/`*` on `String + &str`; line 72 leaked-Box default) are classified disposition (d) per G-174 — compile failures, not behavioral signals; listed in PHASE-5-LOG.md for completeness but not test-suite gaps.

---

### Dismissed

*(none)*

---

### Deferred

*(none — both findings Resolved in-session.)*

---

### Hallucinated

*(none — Phase 5 produced 2 evidence-backed findings, neither rationalizable as not-applicable.)*

---

### Summary

1 finding filed, Resolved in-session. Phase 5 Surface B missing-test gap (`src/lib.rs:48` surviving non-equivalent mutant) closed via the new `save_creates_parent_directory_for_nested_path` test; post-B1 kill rate on viable mutants is 8/8 = 100% (was 7/8 = 87.5% pre-B1). Layer 1 is Phase-5-MVR per the project's `**Phase 5 strategy:** planned — Surface A.0 + Surface B` declaration (the Surface A.0 portion is filed under [SA Review 1](2026-05-20-solution-architect.md#review-1--2026-05-20-0245z); both surfaces are required for Phase-5-MVR per the strategy).

**Coordination:** The Surface A.0 portion of this Phase 5 session is filed under [SA Review 1 — 2026-05-20 02:45Z](2026-05-20-solution-architect.md#review-1--2026-05-20-0245z) per SA Dim 12 (VSDD purity boundary map) ownership of the purity-boundary surface; the SA round handles the DESIGN.md § Verification architecture rewrite + `src/lib.rs:1-?` module-doc reconciliation. The cross-domain hardening record at [`../PHASE-5-LOG.md`](../PHASE-5-LOG.md) Layer 1 entry cites both this QE round and SA Review 1. Per G-151 stop trigger discipline: the prior Phase 3 reached MVR before this Phase 5 session; running Phase 3 Round 2 requires explicit director justification (the revised purity boundary IS new evidence — director may choose to open Round 2 for SO on the basis of the DESIGN.md change). Routing applied in-session per the Phase 5 primer's "intentional refactor that surfaced a spec gap is fine if the gap was raised to Phase 1a+1b routing via Phase 4" framing (per VDD-IAR Alignment dim 12 G-161).
