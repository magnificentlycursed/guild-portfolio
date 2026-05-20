# Solution Architect Review — 2026-05-20

[Index](../SOLUTION-ARCHITECT-REVIEW.md)


**Migration note (PR 6 / Review 78):** This pre-2026-05-21 review entry was authored under portfolio intent and the pre-Review-77 classification-centric finding model. Per PR 6's capstone-intent promotion + the G-177 reference-example-migrates precedent, the Review 77 lifecycle fields (`Owner` / `Status` / `Blocked by` / `Validator`) have been added retroactively to each non-Hallucinated finding so the reference example demonstrates current conventions. The hook's lifecycle-field enforcement (`check-project-review-discipline.py`) does NOT enforce on this date (pre-2026-05-21 cutoff), so the fields are aspirational here; the next-day Review-77-enforced rounds (Reviews dated 2026-05-21+) carry the same fields under the enforced standard.
---

## Review 1 — 2026-05-20 02:45Z

**Phase 5 hardening:** purity-boundary verification — purity-boundary verification for Layer 1 (per G-177 closure convention; preamble tag added at v0.7.8 migration).

**Scope:** Phase 5 (Formal Hardening) Layer 1 hardening round — **SA-scope only**. This entry covers purity-boundary verification which is SA territory (SA Dim 12 — VSDD purity boundary map). The mutation testing portion of the same Phase 5 session is filed under [QE Review 2 — 2026-05-20 02:45Z](2026-05-20-quality-engineer.md#review-2--2026-05-20-0245z) per QE Dim 2 ownership of test falsifiability — see Coordination below. First SA review filed against bookmark-cli per the v0.7.2 adoption (`bookmark-cli-phase5-adoption` branch).

**Session note:** In-session (the same operator authored the Phase 5 primer revisions in suite Reviews 64–66 and now applies them to bookmark-cli). Sycophancy-compensation: the SA Dim 12 (VSDD purity boundary map) check applied to my own architectural framing — the previously-implicit "Pure-core" claim in `src/lib.rs:1-7` was not interrogated by any prior SA review (none had been filed against this project until this session); the cold equivalent would have been an SA-domain Phase 3 round at Layer 1 close that asked "what does DESIGN.md actually say is pure, and does the module doc agree?". That cold round never ran in the project's history. This Phase-5-Surface-A.0 round is a partial substitute — it caught the divergence at Phase 5 timing rather than Phase 3 timing, which is later than the discipline ideally lands but is the discipline working through a different mechanism.

**Source:** `domain-raised` — the Phase 5 purity-boundary verification audit (G-173 multi-source check per suite Review 66) raised the finding against the implementation by applying the suite-level Phase 5 primer's prescriptions to bookmark-cli; the operator-raised v0.7.2 adoption decision is `director-raised` at the session-opening level.

**Regression check:** No prior SA reviews exist for bookmark-cli to verify against. The DESIGN.md § Verification architecture section was authored at Phase 1a (pre-Review-44 reference-impl creation) without an SA pass — this Phase-5-Surface-A.0 round is the first SA scrutiny of the purity-boundary claim.

---

### Resolved

**Finding 1 — Cross-source purity-boundary divergence (Dim 12 — VSDD purity boundary map)**

**Owner:** solution-owner
**Status:** validated
**Blocked by:** *(none)*
**Validator:** solution-architect

The Phase 5 purity-boundary verification multi-source audit (suite Review 66 / G-173 multi-source check) found a 3-way divergence between bookmark-cli's purity claims:

- **`src/lib.rs:1-7`** (module documentation) claimed: *"Pure-core storage logic for bookmark-cli. Per SA Dim 12 (VSDD purity boundary map) — this module contains only pure functions over `Bookmark` and `BookmarkStore`. All I/O is in the effectful shell in `main.rs`."*
- **`DESIGN.md` § Verification architecture** was silent on per-function purity. It named "unit tests for the pure-core storage logic" without enumerating which functions were pure.
- **The implementation** has 3 of 4 `BookmarkStore` methods that are effectful: `load(path)` performs filesystem read (`std::fs::read_to_string` + `path.exists()`); `save(path)` performs filesystem write + directory creation (`std::fs::write` + `std::fs::create_dir_all`); `add(url)` reads system clock via `Utc::now()`. Only `newest_first()` is genuinely pure.

The module-doc claim was wrong relative to the implementation; DESIGN.md was silent on the question the module doc answered (cross-source consistency check failed per G-173 check (c)). The "Per SA Dim 12 (VSDD purity boundary map)" citation in the module doc made the wrongness more load-bearing — it appealed to the suite's own SA discipline as justification for a claim the discipline would have flagged on inspection.

**Behavior consequence:** a future maintainer reading `lib.rs` would internalize a stronger purity claim than the implementation honored. Tests written against the "pure" assumption (e.g., assuming `add` is deterministic) would surface non-determinism unexpectedly. A Phase 5 property-based testing property-based test designed to assert spec invariants over a "pure" `add` would be testing the wrong thing.

**Resolution:** Routed via Phase 4 to Phase 1a+1b per the Phase 5 purity-boundary verification disposition options (option b — revise the boundary). Applied in-session:

- **`DESIGN.md` § Verification architecture rewritten** with an explicit Purity boundary subsection enumerating each function's status:
  - **Pure** (deterministic, no I/O, formally verifiable in principle): `Bookmark` / `BookmarkStore` data types; `BookmarkStore::newest_first`.
  - **Effectful (deliberate I/O wrappers around pure ser/de):** `BookmarkStore::load` (filesystem read + `serde_json` parse — parse step is pure; file read makes the function effectful); `BookmarkStore::save` (serialize pure; filesystem write + directory creation effectful).
  - **Boundary refinement (morally pure w.r.t. inputs; effectful w.r.t. external clock):** `BookmarkStore::add(url)` — deterministic given the clock; non-deterministic against absolute wall time. Acceptable at Layer 1 portfolio intent; could be refined to `add(url, ts)` at a future layer if formal verification of `add` enters scope.
- **`src/lib.rs:1-?`** module doc rewritten to retire the prior "Pure-core" claim and to cite DESIGN.md § Verification architecture as the single authoritative source. The module-doc summary now names the impl's actual purity boundary matching DESIGN.md.

Per the Phase 5 purity-boundary verification outcome format: `Boundary violations found and routed: load/save/add (impl effectful, claimed pure); cross-source divergence between src/lib.rs:1-7 and DESIGN.md § Verification architecture; reconciliation routed to Phase 1a+1b and applied in-session.`

**Cross-source consistency post-fix:** ✓ — `DESIGN.md` § Verification architecture is now the single authoritative source; `src/lib.rs:1-?` module doc cites it without making divergent claims. A future purity-boundary verification audit will find the two sources consistent.

---

### Dismissed

*(none)*

---

### Deferred

*(none — Resolved in-session.)*

---

### Hallucinated

*(none.)*

---

### Summary

1 finding filed, Resolved in-session. SA Dim 12 (VSDD purity boundary map) is now correctly documented and consistent across DESIGN.md and the `lib.rs` module doc; the implementation's actual purity boundary matches the documented one. Layer 1 SA-scope is Phase-5-MVR alongside the QE-scope mutation testing completion (see QE Review 2).

**Coordination:** The mutation testing portion of this same Phase 5 session is filed under [QE Review 2 — 2026-05-20 02:45Z](2026-05-20-quality-engineer.md#review-2--2026-05-20-0245z) per QE Dim 2 ownership of test falsifiability. The revised DESIGN.md § Verification architecture is new evidence under the G-151 stop trigger discipline — director may choose to open a Phase 3 Round 2 for SO on the basis of the DESIGN.md change (the SA-domain change is the routing artifact of this Phase 5 finding; an SO review would evaluate whether the revised verification-architecture statement now satisfies SO Dim 9 / assignment compliance and SO Dim 1 / spec coverage).
