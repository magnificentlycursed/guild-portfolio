# Phase 5 Hardening Log — bookmark-cli

Per [`../vsdd-suite/primers/5-formal-hardening.md`](../vsdd-suite/primers/5-formal-hardening.md). Per-layer Phase 5 records appended below; newest at the top.

Project intent: `portfolio` (per [`DESIGN.md`](../DESIGN.md) § Project intent).
Phase 5 strategy: `planned — Surface A.0 + Surface B; Surface A deferred; Surfaces C + D not applicable.`

The strategy is a one-time-per-project declaration; per-layer entries below record the actual Phase 5 work performed against each layer.

---

## Layer 1 — 2026-05-20 02:45Z

**Layer reference:** acceptance criteria per `TODO.md` § Layer 1 (4 ACs); Phase 2b implementation commit (current `main` head before Review 67 adoption); Phase 3 final round per active domain reached MVR before this Phase 5 session opened.

**Phase 5 strategy (verbatim from DESIGN.md § Project intent):** `planned — Surface A.0 (purity-boundary verification) + Surface B (mutation testing via cargo-mutants) per the v0.7.2 primer. Surface A (property-based testing via proptest) deferred to a future layer if the purity boundary deepens; Surface C (fuzzing) and Surface D (formal proof) are not applicable — bookmark-cli has no safety-critical or cryptographic surface and the JSON-parsing surface is small enough that fuzzing's marginal value is low.`

**Surfaces activated this layer:** A.0 + B (per strategy).

**Tool-install upfront cost (G-175):** `cargo-mutants v27.0.0` installed via `cargo install cargo-mutants --locked` — 1m 39s wall time on the test machine (compile from source). No other tool installs required this session (no Surface A property-based dep-add; no Surface C / D tooling). Subsequent Phase 5 sessions on this project inherit the install and skip this step.

---

### Surface A.0 — Purity-boundary verification (preamble — G-173 multi-source audit)

**Authoritative purity-claim sources audited:**

1. `DESIGN.md` § Verification architecture — at the time of the audit, this section was silent on per-function purity (it named "unit tests for the pure-core storage logic" without enumerating which functions were pure).
2. Module documentation at `src/lib.rs:1-7` — claimed "Pure-core storage logic for bookmark-cli ... contains only pure functions over `Bookmark` and `BookmarkStore`. All I/O is in the effectful shell in `main.rs`."

**Cross-source consistency (G-173 check c):** divergent. The module doc made a stronger claim than DESIGN.md; DESIGN.md was silent on the per-function purity question the module doc answered.

**Impl-vs-claims audit:**

| Function (`src/lib.rs`) | Module-doc claim | DESIGN.md claim (pre-audit) | Implementation | Verdict |
|---|---|---|---|---|
| `Bookmark` / `BookmarkStore` types | pure | silent | serde derivations; pure | Consistent ✓ |
| `BookmarkStore::load` (line 31) | pure ("contains only pure functions") | silent | `path.exists()` + `std::fs::read_to_string(path)` | **Effectful** — module-doc claim wrong |
| `BookmarkStore::save` (line 46) | pure | silent | `std::fs::create_dir_all` + `std::fs::write` | **Effectful** — module-doc claim wrong |
| `BookmarkStore::add` (line 62) | pure | silent | `Utc::now()` reads system clock | **Effectful** w.r.t. clock — module-doc claim wrong |
| `BookmarkStore::newest_first` (line 71) | pure | silent | sort by reference; no I/O | Consistent ✓ |

**Disposition routing:** option (b) per the Phase 5 primer Surface A.0 — revise DESIGN.md to enumerate the actual purity boundary; revise module doc to cite DESIGN.md as the single authoritative source. Route via Phase 4 to Phase 1a+1b. Resolved in-session (Review 67):

- `DESIGN.md § Verification architecture` rewritten with explicit Purity boundary subsection enumerating pure functions (data types + `newest_first`), effectful functions (deliberate I/O wrappers around pure ser/de: `load`, `save`), and the boundary refinement (`add` reads `Utc::now()`).
- `src/lib.rs:1-?` module doc revised to cite DESIGN.md as authoritative; the prior "Pure-core" claim retired.

**Surface A.0 outcome (per primer prescription):** `Boundary violations found and routed: load/save/add (impl effectful, claimed pure); cross-source divergence between src/lib.rs:1-7 (claimed Pure-core) and DESIGN.md § Verification architecture (silent); reconciliation routed to Phase 1a+1b and applied in-session.`

---

### Surface B — Mutation testing (`cargo-mutants v27.0.0`)

**Pre-B1 baseline (before adding the missing save-to-nested-path test):**

```
Found 11 mutants to test
ok       Unmutated baseline in 28s build + 3s test
11 mutants tested in 59s: 1 missed, 7 caught, 3 unviable
```

Kill rate on viable mutants: 7 / 8 = **87.5%**.

**Post-B1 result (after adding `save_creates_parent_directory_for_nested_path` test):**

```
Found 11 mutants to test
ok       Unmutated baseline in 21s build + 3s test
11 mutants tested in 45s: 8 caught, 3 unviable
```

Kill rate on viable mutants: 8 / 8 = **100%**.

**Disposition table (per primer Surface B mandate, 5-disposition universe per G-174):**

| Mutant location | Mutation | Pre-B1 outcome | Disposition |
|---|---|---|---|
| `src/lib.rs:32:9` | `load` → `Ok(Default::default())` | caught | (a) — no action (test killed it) |
| `src/lib.rs:32:12` | delete `!` in `load` | caught | (a) — no action |
| `src/lib.rs:47:9` | `save` → `Ok(())` | caught | (a) — no action |
| `src/lib.rs:48:16` | delete `!` in `save` (`!parent.as_os_str().is_empty()` → `parent.as_os_str().is_empty()`) | **MISSED** (pre-B1) | (b) **missing-test** — added falsifying test `save_creates_parent_directory_for_nested_path` with `retroactive Red Gate (Phase 5 source)` label per `vsdd-suite/primers/2b-implementation.md` § Phase 2b step 2 (G-176 / Review 65 label extension); now caught |
| `src/lib.rs:63:9` | `add` → `()` | caught | (a) — no action |
| `src/lib.rs:72:9` | `newest_first` → `vec![]` | caught | (a) — no action |
| `src/main.rs:33:5` | `store_path` → default `PathBuf` | caught | (a) — no action |
| `src/main.rs:39:5` | `main` → default `ExitCode` | caught | (a) — no action |
| `src/lib.rs:55:35` | `+` → `-` in `save` | unviable | (d) **unviable** — compile failure on `String - &str`; not a behavioral signal |
| `src/lib.rs:55:35` | `+` → `*` in `save` | unviable | (d) **unviable** — same |
| `src/lib.rs:72:9` | `newest_first` → leaked-Box default | unviable | (d) **unviable** — type-system rejected |

**Evaluation scope (per G-174 / R2):** every surviving mutant in spec-asserted code paths (the default scope from the Phase 5 primer Surface B). No mutants omitted as out-of-scope; the project's code is small enough that the entire `src/` tree is in scope.

**Surviving non-equivalent mutants after B1:** 0. Surface B Phase-5-gate-complete: yes.

**Surface B outcome (per primer prescription):** Kill rate post-B1 100% on viable mutants (8 / 8); 3 unviable mutants listed per G-174 disposition (d); 0 surviving non-equivalent mutants requiring further action.

---

### Surface A — Property-based testing (DEFERRED)

Per `**Phase 5 strategy:** ... Surface A (property-based testing via proptest) deferred to a future layer if the purity boundary deepens`. No action this session.

Rationale for the deferral: the current pure functions on the purity boundary are `Bookmark` / `BookmarkStore` (data types — serde derivations exercise the roundtrip property already via the existing `save_then_load_roundtrips` unit test) and `BookmarkStore::newest_first` (a one-line sort whose invariants — length preservation, descending-timestamp ordering — are exercised by the existing `newest_first_sorts_descending_by_timestamp` test). Adding proptest properties at Layer 1 would duplicate the existing tests with a generated input space; the marginal value is low at this layer scale. If Layer 2 (tag filtering) or Layer 3 (import/export) deepens the purity boundary with non-trivial pure logic, Surface A activates per the per-layer strategy refinement.

---

### Surface C — Fuzzing (NOT APPLICABLE)

Per `**Phase 5 strategy:** ... Surface C (fuzzing) ... not applicable — ... the JSON-parsing surface is small enough that fuzzing's marginal value is low.` Declaration recorded; no action this session.

---

### Surface D — Formal proof (NOT APPLICABLE)

Per `**Phase 5 strategy:** ... Surface D (formal proof) ... not applicable — bookmark-cli has no safety-critical or cryptographic surface.` Declaration recorded; no action this session.

---

### Phase 5 completion criteria check (per primer § Completion criteria)

1. Purity boundary verified for the layer: ✓ — Surface A.0 audit complete; divergences resolved by revising DESIGN.md + module doc.
2. Surfaces activated per the project's `**Phase 5 strategy:** planned — <scope>` declaration each have a recorded outcome: ✓ — Surfaces A.0 + B activated and recorded; Surfaces A + C + D have explicit dispositions (deferred / not applicable / not applicable).
3. Every surviving mutant within the evaluation scope has a per-mutant disposition: ✓ — 0 surviving mutants post-B1; the 1 originally-surviving mutant was disposed via (b) missing-test.
4. Every Phase 5 finding routed to Phase 4 has either been Resolved or Deferred-with-named-trigger: ✓ — the Surface A.0 boundary divergence routed to Phase 1a+1b was Resolved in-session via DESIGN.md and module-doc revision; the Surface B missing-test was Resolved in-session via the new test addition.
5. The project's `**Phase 5 strategy:**` declaration's named scope is complete: ✓ — Surfaces A.0 + B both ran; the deferred / not-applicable Surfaces are explicitly declared.

**Phase 5 status for Layer 1: COMPLETE.** Layer 1 is Phase-5-MVR. No outstanding Phase 5 findings against Layer 1 implementation as of 2026-05-20 02:45Z.

---
