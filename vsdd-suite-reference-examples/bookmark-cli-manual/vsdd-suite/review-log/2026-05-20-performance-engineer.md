# Performance Engineer Review — 2026-05-20

[Index](../PERFORMANCE-ENGINEER-REVIEW.md)

---

## Review 1 — 2026-05-20 19:30Z

**Scope:** Layer 1 cold-context [Phase 3](../../../vsdd-suite/primers/3-review-session.md) IAR Round 1 for the Performance Engineer domain (capstone-tier activation per [`DESIGN.md`](../../DESIGN.md) § Project intent). Artifacts read in adversarial order: [`README.md`](../../README.md), [`Cargo.toml`](../../Cargo.toml), [`src/main.rs`](../../src/main.rs), [`src/lib.rs`](../../src/lib.rs), [`tests/bookmarks.rs`](../../tests/bookmarks.rs), [`DESIGN.md`](../../DESIGN.md) (last, per primer guidance — the spec is the contract this review evaluates the impl against). Existing-round schema reference: [QE Review 2 — 2026-05-20 02:45Z](2026-05-20-quality-engineer.md#review-2--2026-05-20-0245z) (lifecycle-field shape; classification heading discipline; preamble ordering).

**Session note:** Cold session — this PE round opened in a fresh context with no prior participation in the project's build or in any prior IAR domain round. The adversarial posture follows the [`3-review-session.md`](../../../vsdd-suite/primers/3-review-session.md) primer: primary obligation is to the spec; the implementation may be correct and still be a finding if its performance shape is undeclared in DESIGN.md. Per the PE domain prompt's sycophancy check, every "fast enough" intuition is treated as suspect until a measurement (`cargo bench`, `cargo flamegraph`, `hyperfine`) backs it — none exist in the project, which is itself the dominant finding for this round.

**Source:** `domain-raised` — the PE [Standard Evaluation Dimensions](../../../vsdd-suite/domains/role/PERFORMANCE-ENGINEER-REVIEW.md) (dims 1, 3, 4, 5, 7, 8, 9, 10) applied to the implementation plus the [`rust.md`](../../../vsdd-suite/supplements/rust.md) § Performance Engineer supplement (Criterion benchmarks; debug-vs-release profile; allocation patterns) produced every finding below.

**Regression check:** No prior PE rounds exist for bookmark-cli (the index in [`../PERFORMANCE-ENGINEER-REVIEW.md`](../PERFORMANCE-ENGINEER-REVIEW.md) lists this as Review 1). Cross-domain regression context: [SA Review 1](2026-05-20-solution-architect.md#review-1--2026-05-20-0245z) rewrote DESIGN.md § Verification architecture but did not introduce a Performance subsection; [QE Review 2](2026-05-20-quality-engineer.md#review-2--2026-05-20-0245z) executed cargo-mutants but no perf-adjacent benchmarks. Neither prior round addressed PE concerns; the perf surface is unmeasured.

**Scope carve-outs (not findings — dimensions inapplicable to this project shape):** Dim 2 (main thread / event loop blocking) is not applicable to a short-lived CLI process. Dim 6 (caching / memoization) is not applicable to a single-invocation CLI — there is no in-process cache surface to evaluate. Dim 7 (memory growth over long sessions) is not applicable — the process lifetime is one command invocation. These three dims are recorded here as deliberately N/A rather than as Hallucinated findings.

---

### Resolved

*(none — this round catalogs findings; resolution requires either DESIGN.md authority (Solution Owner) or code-fix authority (Software Engineer / Platform Engineer) per the validator-pair routing below. Cold adversary does not self-resolve mid-session.)*

---

### Raised to SO

**Finding 1 — DESIGN.md declares no performance budget despite capstone intent (Dim 8)**

**Owner:** solution-owner
**Status:** raised
**Blocked by:** *(none)*

[`DESIGN.md`](../../DESIGN.md) contains zero performance language. There is no startup-time target, no per-command latency budget, no declared maximum bookmark count, no I/O-throughput claim, no memory ceiling, and no explicit "accepted scale ceiling" subsection naming a known limitation. The § Edge case catalog (DESIGN.md:71-80) covers correctness edge cases (empty URL, whitespace, missing file, invalid JSON) but is silent on scale edges (1K / 10K / 100K bookmarks; multi-MB JSON file). § Constraints (DESIGN.md:152-157) names Rust toolchain, platform, dependency policy, and deployment — no performance constraint.

Per PE Dim 8: *"A project with no performance budget has no performance requirement."* This formulation makes every downstream PE dimension unfalsifiable — "is the implementation fast enough?" has no answer because "fast enough" was never defined. For a portfolio-intent project the absence might be acceptable; for the project's **declared `capstone` intent** (DESIGN.md:9 promotion record; capstone activates PE per [`DOMAIN-INDEX.md`](../../../vsdd-suite/domains/DOMAIN-INDEX.md) § Intent calibration), the omission is a Phase 1a+1b gap surfaced only at PE time.

The dependent findings below (Findings 2–6) are all conditional on this one: a benchmarking infrastructure with no declared SLA is benchmarking for its own sake; a "quadratic cumulative cost on add" finding (Finding 3) without a declared maximum N is unactionable; "no scaling tests at 10K/100K" (Finding 5) presumes a target N the spec never states. Resolving Finding 1 unblocks honest dispositions for the rest.

**Proposed change to DESIGN.md (for SO adjudication):** Add a `## Performance budget` section after § Constraints (DESIGN.md:152) containing, at minimum:

1. **Declared scale ceiling** — e.g., *"bookmark-cli is sized for ≤10,000 bookmarks under single-user manual-entry rates. Behavior at N > 10K is undefined; the storage architecture (flat JSON, full file rewrite on add) becomes superlinear past that bound."*
2. **Per-command latency target** — e.g., *"`bm add` and `bm list` must complete in <100 ms wall-clock at the declared ceiling on the declared reference platform (macOS / Linux on commodity 2020+ hardware)."*
3. **Measurement methodology declaration** — names whether perf is measured (criterion bench, hyperfine, manual stopwatch) or explicitly not measured (with rationale).
4. **Accepted limitations subsection** — explicitly names the trade-offs documented as Findings 3 + 6 below (flat-JSON O(n) per write; pretty-print serialization). Per the [PE classification universe](../../../vsdd-suite/domains/role/PERFORMANCE-ENGINEER-REVIEW.md), `accepted limitation` requires *"deliberate performance trade-off, explicitly documented with the trade-off rationale"* — the spec is the natural home for that documentation.

Rationale: the worked example exists to teach the methodology end-to-end. A capstone-tier reference that activates PE but has no perf budget teaches readers that PE rounds can run without a spec to evaluate against — which is the exact failure mode the primer's "primary obligation is to the spec" framing was written to defeat. The fix is one section in DESIGN.md, not a code change.

**Classification:** Raised to SO. DESIGN.md change authority belongs to the Solution Owner per the PE domain prompt's `DESIGN.md change authority` paragraph and the [`SOLUTION-OWNER-REVIEW.md`](../../../vsdd-suite/domains/role/SOLUTION-OWNER-REVIEW.md) charter. SO adjudication should result in an SO review entry under [`vsdd-suite/SOLUTION-OWNER-REVIEW.md`](../SOLUTION-OWNER-REVIEW.md) (index currently empty; would land as SO Review 1) recording the budget section addition or an explicit decision to declare PE not-applicable at the bookmark-cli scale (which would also be a spec change — silence is not adjudication).

---

### Deferred

**Finding 2 — No benchmarking infrastructure declared or wired (Dim 9)**

**Owner:** platform-engineer
**Status:** raised
**Blocked by:** [Finding 1](#raised-to-so)

[`Cargo.toml`](../../Cargo.toml) `[dev-dependencies]` lists `assert_cmd`, `predicates`, `tempfile` — zero benchmarking crates. There is no `benches/` directory, no `criterion` or `divan` crate referenced anywhere, no `hyperfine` invocation script under `manual-tests/` or `tests/`, and no documented `cargo bench` workflow. The README.md § Test section (README.md:38-44) lists `cargo test` as the entire verification methodology; § How this was built (README.md:46-59) makes no reference to a performance methodology.

The [`rust.md`](../../../vsdd-suite/supplements/rust.md) § Performance Engineer supplement names the failure mode directly: *"A function documented as 'fast' with no benchmark is an assertion without evidence."* While the project does not currently document any function as "fast," the capstone-intent reference-implementation purpose makes the absence of measurement infrastructure itself the finding — a reader learning the methodology from this example will see PE activated with no PE tooling demonstrated, and will conclude that PE-tier projects need no measurement substrate. That conclusion contradicts the supplement's own discipline.

**Concrete shift-left mechanization that would close this finding (for the Platform Engineer / Software Engineer fix-owner pair):**

- Add `criterion = "0.5"` to `[dev-dependencies]` in [`Cargo.toml`](../../Cargo.toml).
- Add `[[bench]]` entry pointing to `benches/store_ops.rs`.
- Author `benches/store_ops.rs` exercising at minimum three workloads: `BookmarkStore::load` over a 1K / 10K / 100K-entry pre-built JSON file; `BookmarkStore::save` for the same sizes; `BookmarkStore::newest_first` sort cost at the same sizes. Use `criterion::black_box` per supplement guidance to defeat optimizer elision.
- Optional shift-further-left: a `manual-tests/perf-baseline.md` invoking `hyperfine 'bm list' --warmup 3` against a fixture store of declared size, recording the baseline for cross-version regression comparison.
- README.md § Test acquires a third subsection naming `cargo bench` and the methodology.

**Trigger (G-130):** Layer 2 implementation start. Layer 2 adds `tag` + `--tag` filter — both operations iterate the bookmark collection per command, making the per-N-entries cost more visible and giving the criterion baseline a non-trivial second data point. Building the baseline at Layer 2 start lets the Layer 2 implementation evolve against a measurement floor rather than after-the-fact.

**Cost-of-deferral:** Each additional layer (tag at L2; export/import at L3) lands without a baseline against which to detect regression. A Layer 3 `bm export` that iterates the full store could ship a 10x slowdown over Layer 1 `bm list` and no review would catch it — there is no baseline to compare to. PE Dim 10 (regression risk) cannot fire without a baseline; PE Dim 9 (perf testing methodology) registers `nil` indefinitely. The capstone-intent reference-implementation purpose continues to teach "PE without measurement" for every reader between now and Layer 2 start.

**Auto-Backlog clause:** If Layer 2 closes without `benches/` populated, this finding auto-Backlogs at the start of Layer 2 R2 (per the [G-130](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-130) auto-Backlog mechanism — three consecutive PE rounds Open without adjudication promotes to Backlog), carrying forward to the project's [`FINDINGS-INDEX.md`](../FINDINGS-INDEX.md) Backlog section, and re-raises as a Platform Engineer dim 38 (fresh-system install verification) coordination item — the Platform Engineer round at capstone intent will independently flag the missing perf-tooling shift-left.

**Classification:** Deferred. The fix is shift-left mechanization (Cargo.toml + benches/ + README.md), which is Platform Engineer territory per the PE domain prompt's Coordination paragraph; the validator pair is platform-engineer (shift-left verification: the bench harness runs in CI / on demand and produces the artifact). The PE round re-measures against the same workload at Round 2 to confirm the harness produces the declared budget numbers from Finding 1.

---

**Finding 3 — `BookmarkStore::save` rewrites entire file on every `add` — cumulative O(n²) cost (Dim 4)**

**Owner:** software-engineer
**Status:** raised
**Blocked by:** [Finding 1](#raised-to-so)

[`src/main.rs:42-61`](../../src/main.rs) implements `Cmd::Add` as: `BookmarkStore::load(&path)` ([`src/lib.rs:45-56`](../../src/lib.rs)) → `store.add(url)` ([`src/lib.rs:76-81`](../../src/lib.rs)) → `store.save(&path)` ([`src/lib.rs:60-72`](../../src/lib.rs)). Each step on the load+save bookends operates on the *entire* bookmark collection:

- `load` calls `std::fs::read_to_string` (whole file → `String` allocation of file-size bytes) followed by `serde_json::from_str` (whole-document parse → `Vec<Bookmark>` of N entries). Both are unconditional regardless of whether the caller needs the existing entries (`bm add` only needs to append).
- `save` calls `serde_json::to_string_pretty` (entire `BookmarkStore` → `String` of file-size bytes including the just-pushed entry) followed by `std::fs::write` (whole-file overwrite, no `OpenOptions::append`, no incremental write).

Adding N bookmarks via N sequential `bm add` invocations therefore performs `1 + 2 + 3 + ... + N` cumulative bytes of read+parse+serialize+write — **O(N²) total I/O** to construct an N-entry store. At the supplement's named scale checkpoints:

- **N = 1,000**: ~500 KB cumulative I/O. Negligible. Each individual `add` is ~1 KB read + 1 KB write — sub-millisecond.
- **N = 10,000**: ~50 MB cumulative I/O. Each individual `add` is ~10 KB read + 10 KB write — still sub-100ms per add but the cumulative cost is real.
- **N = 100,000**: ~5 GB cumulative I/O. Each individual `add` is ~100 KB read + parse + serialize + 100 KB write — likely 100+ ms per add on commodity SSD; the parse and serialize cost dominates.

Per the supplement's *"`String::from` or `.to_string()` inside a loop that could use `&str`; `Vec::new()` inside a function called per-item in a collection"* failure mode: the per-`add` cost grows with N because the data structure rebuild is per-call, not per-delta. JSON Lines (append-only newline-delimited records, one `Bookmark` per line) would make `add` O(1) at the cost of breaking the spec's pretty-printed array storage format (DESIGN.md:101-110).

This finding is conditional on Finding 1. If the SO declares the bookmark-cli scale ceiling at N ≤ 1,000 (a plausible "single-user manual-entry rate over 10 years at 1/day" bound), the cumulative cost is negligible and this finding becomes Dismissed (with the SO declaration as rationale). If the ceiling is declared at N ≥ 100,000 or undeclared, the storage architecture is wrong for the declared scale and the finding promotes to a Layer 2+ refactor (storage format change is a spec-level concern).

**Trigger (G-130):** [Finding 1](#raised-to-so) adjudication. The SO's declared scale ceiling determines whether this finding is real (ceiling ≥ 10K → real, needs Layer 2 storage refactor) or hallucinated (ceiling ≤ 1K → impl is correct for declared scale).

**Cost-of-deferral:** If Finding 1 closes with ceiling ≥ 10K and this finding stays Deferred past Layer 2 start, Layer 2's `tag` operation inherits the same O(n²) cumulative shape (read-whole / write-whole on every tag command) — fixing the storage format at Layer 1 close is one refactor; fixing it at Layer 3 close after `tag` and `export`/`import` have been built against the array-of-records assumption is three refactors.

**Auto-Backlog clause:** If Layer 2 closes without an SO-declared scale ceiling AND without a storage-architecture decision, the finding auto-Backlogs at Layer 2 R2 closure per the [G-130](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-130) discipline, re-raises as a Solution Architect Dim concern (architectural decision creating performance constraint per the PE Coordination paragraph naming SA overlap).

**Classification:** Deferred. The PE measurement work (file the finding with concrete N → cost numbers per the dim 4 named failure mode "tested with realistic data volumes — a test with 5 items does not validate performance with 5,000") is complete in-session; the implementation choice (refactor or accept) requires the spec-side answer from Finding 1 first. Validator at Resolved time: software-engineer (per the PE domain prompt's validator-pair paragraph: PE measures, SE owns the code fix, PE re-measures).

---

**Finding 4 — `Cargo.toml` has no `[profile.release]` tuning (Dim 3)**

**Owner:** platform-engineer
**Status:** raised
**Blocked by:** *(none)*

[`Cargo.toml`](../../Cargo.toml) ends at the `[dev-dependencies]` block (Cargo.toml:24-27). There is no `[profile.release]` section. The `cargo install --path .` workflow named in [README.md:18-23](../../README.md) and [DESIGN.md:157](../../DESIGN.md) (`Deployment: cargo install --path . into ~/.cargo/bin/`) defaults to the unconfigured release profile, which means:

- `lto = false` (no link-time optimization). For a binary that fits in one crate plus six deps (`clap`, `serde`, `serde_json`, `chrono`, `anyhow` plus the transitive set), LTO would meaningfully shrink the binary and remove cross-crate function-call indirection. Skipping it is the default; for a release-installable CLI it is leaving cycles and bytes on the floor.
- `codegen-units = 16` (parallel codegen units; sacrifices some optimization for compile speed). For an installable binary the trade is reversed — installs are rare, runs are frequent.
- `panic = "unwind"` (default; embeds unwinding tables). For a CLI with no panic-recovery requirement (the binary exits on panic), `panic = "abort"` produces a smaller binary and slightly faster startup.
- `strip = false` (default; debug symbols ship in the release binary). `strip = "symbols"` or `strip = "debuginfo"` removes them from the installed binary.

Per the Rust supplement's debug-vs-release entry: *"Performance measurements taken against debug builds are not representative."* The supplement's intent is symmetric — release-build performance is what the user sees, so the release profile is the configuration that matters. Leaving it at defaults treats the release profile as if it were equivalent to a tuned one; it is not.

**Concrete shift-left mechanization for the Platform Engineer / Software Engineer fix pair:**

```toml
[profile.release]
lto = "thin"
codegen-units = 1
panic = "abort"
strip = "symbols"
```

`lto = "thin"` (rather than `lto = true` / fat-LTO) is the practical default — most of the wall-clock savings, a fraction of the compile-time cost.

**Trigger (G-130):** Layer 2 implementation start OR the next `cargo install --path .` workflow invocation for a release-tagged version (whichever comes first). Layer 2 is the natural moment because Layer 2 adds the second feature and the binary grows; the install-workflow trigger catches the case where the binary is shipped to a user before Layer 2.

**Cost-of-deferral:** Each user who runs `cargo install --path .` between now and the fix gets a binary that is larger and (marginally) slower than necessary. The marginal cost is small per-user — but it compounds the "capstone-intent reference doesn't demonstrate release-build tuning" pedagogical gap that Finding 2 already names from a different angle. Two findings on the same pedagogical seam is the sycophancy-resistance signal that the seam matters.

**Auto-Backlog clause:** If Layer 2 starts without `[profile.release]` configured, the finding auto-Backlogs at Layer 2 R1 closure and re-raises as a Platform Engineer Dim shift-left mechanization concern (the binary's install profile is unambiguously platform-engineering territory).

**Classification:** Deferred. The fix is four lines in `Cargo.toml` and could be applied in-session by the operator, but the PE adversarial role does not edit project source — code changes route through the validator-pair to Platform Engineer + Software Engineer. Validator at Resolved time: platform-engineer (verifies the profile block is present and the post-fix `cargo build --release` produces a stripped binary with the declared options applied) — this is a shift-left mechanization, so platform-engineer is the natural pair per the task framing.

---

**Finding 5 — Zero data-scaling tests (Dim 4)**

**Owner:** quality-engineer
**Status:** raised
**Blocked by:** [Finding 1](#raised-to-so)

[`tests/bookmarks.rs`](../../tests/bookmarks.rs) exercises the binary against at most **two** bookmarks (`tests_list_orders_newest_first`, tests/bookmarks.rs:67-111). The `src/lib.rs` `#[cfg(test)] mod tests` block ([src/lib.rs:92-169](../../src/lib.rs)) exercises at most **three** bookmarks (`newest_first_sorts_descending_by_timestamp`, src/lib.rs:101-113). There is no test at N = 1,000, N = 10,000, or N = 100,000 — the supplement's three named checkpoint sizes for data scaling are entirely absent.

Per PE Dim 4's named failure mode: *"rendering an unsorted list of 10,000 items without virtualization; `localStorage.setItem` with a multi-MB JSON blob; iterating the full dataset on every keystroke to compute a derived value."* The bookmark-cli analog of "multi-MB JSON blob" is *"the load+save path in Finding 3 at N = 10,000."* The test suite as it stands cannot distinguish "correct at small N" from "correct at all declared N" — Finding 3 had to be reasoned about from inspection, not from test evidence, because no test would have surfaced it.

The dim's sycophancy framing applies: *"A test with 5 items does not validate performance with 5,000."* All current tests cap at ≤3 items.

**Concrete shift-left mechanization for the Quality Engineer / Software Engineer fix pair:**

- Add one ignored-by-default test per workload size: `#[ignore = "scaling test; run with --ignored"] #[test] fn list_scales_to_10k_bookmarks() { ... }` covering N = 1K, 10K, and (optionally) 100K. The `#[ignore]` keeps the default `cargo test` run fast while making the scaling tests reachable via `cargo test -- --ignored`.
- Each scaling test pre-populates a temp store via fixture-generation (loop pushing N entries), then exercises `bm list` or `bm add` and asserts both correctness (output line count matches N) and wall-clock under the budget declared in [Finding 1](#raised-to-so)'s proposed Performance budget section.
- Coordinates with [Finding 2](#deferred) — the criterion benches measure cost; the scaling tests assert correctness at scale. Both surfaces are needed; neither substitutes for the other.

**Trigger (G-130):** [Finding 1](#raised-to-so) adjudication. The scaling tests need a declared budget to assert against — "completes under 100ms" requires a 100ms budget the spec doesn't currently name.

**Cost-of-deferral:** Each layer that lands without scaling tests means each PE round at that layer files Finding 3-shaped concerns from inspection alone — the round produces unverified hypotheses about scale behavior rather than measured findings. PE Round 2 at Layer 2 cannot promote the Finding 3 hypothesis to a confirmed regression-tracked finding without scaling tests to back it.

**Auto-Backlog clause:** If Layer 2 closes without scaling tests, the finding auto-Backlogs at Layer 2 R2 closure and re-raises as a Quality Engineer dim concern (test-coverage-at-realistic-scale is QE Dim 4 territory by the PE Coordination paragraph's QE overlap).

**Classification:** Deferred. The test work is QE territory mechanically (test authoring) but PE territory dimensionally (the workload sizes and budget assertions come from the PE budget). Validator at Resolved time: quality-engineer for the test-discipline shape; the PE Round 2 re-runs the scaling tests to confirm wall-clock budgets hold.

---

### Accepted limitation

**Finding 6 — `to_string_pretty` instead of `to_string` for storage serialization (Rust supplement § Performance Engineer — allocation patterns)**

**Owner:** software-engineer
**Status:** raised
**Blocked by:** *(none)*

[`src/lib.rs:67`](../../src/lib.rs) calls `serde_json::to_string_pretty(self)` rather than `serde_json::to_string(self)`. Pretty-printing produces a larger output (whitespace and newlines per record) and pays a serialization-time cost that is non-trivial at scale — roughly 1.5–2x the compact-form throughput per `serde_json` benchmark community data.

At the [Finding 3](#deferred) scale points: pretty-printing 100K entries produces a ~5–10 MB JSON file rather than a ~3 MB compact file, and the serialize step is the dominant cost in the `save` hot path at that scale. At 1K entries, the difference is negligible.

The cost is real but the trade is deliberate. [`DESIGN.md`](../../DESIGN.md) § Storage format (lines 101-110) shows the storage example *in pretty-printed form*:

```json
{
  "bookmarks": [
    {"url": "https://example.com", "timestamp": "2026-05-17T03:01:00Z"},
    {"url": "https://example.org", "timestamp": "2026-05-17T02:55:00Z"}
  ]
}
```

The spec's example is itself a partial commitment — readers internalize that the on-disk format is human-readable. DESIGN.md § Edge case catalog (DESIGN.md:71-80) further supports this via the *"manual JSON edit if needed"* affordance in § Non-goals (DESIGN.md:50) — the design explicitly contemplates the user opening the file in a text editor, which presumes a readable format. Switching to `to_string` (single-line, no whitespace) would make the file unreadable for the documented manual-edit affordance.

The trade-off rationale is therefore:
- **Cost accepted:** ~2x serialize cost at scale; ~1.7x on-disk size.
- **Benefit retained:** human-readable storage file consistent with DESIGN.md's manual-edit affordance.

**Per the PE classification universe — `accepted limitation` requires *"deliberate performance trade-off, explicitly documented with the trade-off rationale"*:** the trade-off rationale above is the documentation. For DESIGN.md to register the acceptance properly, the proposed § Performance budget section from [Finding 1](#raised-to-so) should include an Accepted limitations subsection citing this finding by anchor.

**Classification:** Accepted limitation. The finding is real (the cost is real at scale) but the trade is deliberate (human-readable storage is the documented design choice). Resolution does not require a code change; it requires the DESIGN.md documentation update from Finding 1 to formally absorb this trade-off into the spec.

---

### Dismissed

*(none)*

---

### Hallucinated

*(none — Round 1 produced 6 evidence-backed findings against PE dims 3, 4, 5, 8, 9, 10 + Rust supplement § Performance Engineer; no finding could be demonstrated as adversary-invented in the cold pass. The Round-N+1 trigger fires per G-131 — at least Finding 1 is a real new finding regardless of how the others are adjudicated, so PE Round 2 is mandatory once Finding 1 is adjudicated by SO.)*

---

### Deferred

*(none — every finding above is in a terminal-or-routed classification: Raised to SO routes to the solution-owner index; Deferred carries the G-130 trigger discipline; Accepted limitation is terminal for PE. No findings are "actively being worked across sessions" in the in-progress sense.)*

---

### Summary

6 findings filed against [`DESIGN.md`](../../DESIGN.md), [`Cargo.toml`](../../Cargo.toml), [`src/lib.rs`](../../src/lib.rs), and [`tests/bookmarks.rs`](../../tests/bookmarks.rs): **1 Raised to SO** (no perf budget declared despite capstone intent — the foundational gap that makes the other five dispositions provisional); **4 Deferred** with G-130 triggers (no benchmarking infrastructure; cumulative O(n²) cost on add; no `[profile.release]` tuning; zero data-scaling tests); **1 Accepted limitation** (`to_string_pretty` at scale — deliberate human-readable-storage trade-off). 0 Resolved (cold adversarial round does not self-resolve), 0 Dismissed, 0 Hallucinated. The finding progression — 6 real, 0 hallucinated — is the G-131 continue-trigger signal for a mandatory PE Round 2 after Finding 1 is adjudicated by the Solution Owner.

The dominant pattern across the findings: the implementation is *correct* for the project's likely manual-entry scale (≤1K bookmarks) but the *spec* never declared that scale, so every "good enough" disposition is currently unfalsifiable. The Round 1 contribution is to make the unfalsifiability itself the headline finding (Finding 1) and route every dependent finding through it.

**Coordination:** Finding 1 routes to the Solution Owner via the index at [`vsdd-suite/SOLUTION-OWNER-REVIEW.md`](../SOLUTION-OWNER-REVIEW.md) — adjudication would land as SO Review 1 (the project has no prior SO rounds filed). Findings 2 + 4 route to the Platform Engineer pair at [`vsdd-suite/PLATFORM-ENGINEER-REVIEW.md`](../PLATFORM-ENGINEER-REVIEW.md) for shift-left mechanization (benchmarking harness; release-profile tuning). Finding 3 routes to the Software Engineer pair at [`vsdd-suite/SOFTWARE-ENGINEER-REVIEW.md`](../SOFTWARE-ENGINEER-REVIEW.md) (storage refactor, conditional on Finding 1 outcome). Finding 5 routes to the Quality Engineer pair at [`vsdd-suite/QUALITY-ENGINEER-REVIEW.md`](../QUALITY-ENGINEER-REVIEW.md) (scaling-test discipline, conditional on Finding 1's budget declaration). Finding 6 closes terminally as Accepted limitation but its documentation lives in the Finding 1 DESIGN.md change. The PE Round 2 re-runs the cold pass against the post-Finding-1 spec and the post-mechanization code; the validator pair (per the PE domain prompt's Review 77 paragraph) is software-engineer for code-fix findings and platform-engineer for shift-left mechanization findings, with sanity-check as fallback for any finding without a natural cross-domain pair.

---

## Review 2 — 2026-05-20 21:00Z

**Scope:** Layer 1 cold-context [Phase 3](../../../vsdd-suite/primers/3-review-session.md) IAR Round 2 for the Performance Engineer domain (capstone-tier activation per [`DESIGN.md`](../../DESIGN.md) § Project intent). This round verifies the Round 1 fix-cycle resolutions against the current Layer 1 artifacts and re-pressures the implementation for new findings. Artifacts read (adversarial order): [`Cargo.toml`](../../Cargo.toml) (now declares `[profile.release]`), [`src/main.rs`](../../src/main.rs), [`src/lib.rs`](../../src/lib.rs) (now uses atomic tmp-file + rename in `save`, plus `sync_all`, `symlink_metadata`, mode 0600), [`tests/bookmarks.rs`](../../tests/bookmarks.rs), [`DESIGN.md`](../../DESIGN.md) (last, per primer guidance — now contains § Performance budget at lines 163-177), and the [Round 1 log](2026-05-20-performance-engineer.md#review-1--2026-05-20-1930z) for the per-finding verification matrix.

**Session note:** Cold session — Round 2 opened in a fresh context with no carryover from Round 1's authoring session. The primer's sycophancy guard applies twice in this round: once against the Round 1 reviewer's posture (do not soft-confirm Round 1's classifications without re-checking the artifacts), and once against the fix-cycle authors (do not soft-accept the Round 1 fixes without re-pressuring the *new* code paths they introduced).

**Source:** `domain-raised` — Round 1 finding-verification + fresh adversarial pressure on the Round 2 fix surface (atomic-save tmp-file + rename + `sync_all` + `symlink_metadata` — all introduced since Round 1) under PE [Standard Evaluation Dimensions](../../../vsdd-suite/domains/role/PERFORMANCE-ENGINEER-REVIEW.md) (Dims 4, 5, 8, 9, 10) + [`rust.md`](../../../vsdd-suite/supplements/rust.md) § Performance Engineer (allocation patterns; debug-vs-release).

**Regression check:** Round 1's findings F1–F6 are verified individually below. The atomic-save refactor that landed between Round 1 and Round 2 (per [SE Review 1 Finding 2](2026-05-20-software-engineer.md#r1-f2) and [Security Review 1 Finding 2](2026-05-20-security.md#r1-f2)) is the only structurally new code path in the perf surface; it is re-pressured under the new-finding section. The `newest_first` sort path is unchanged; the `load` path is unchanged; the `add` mutation path is unchanged. The serialize hot path (`serde_json::to_string_pretty`) is unchanged.

**Scope carve-outs (unchanged from Round 1):** Dim 1 (time-to-interactive — browser apps), Dim 2 (main-thread / event-loop blocking), Dim 3 (asset optimization — browser), Dim 6 (caching / memoization in long-lived process), Dim 7 (memory growth over long sessions) remain inapplicable to a short-lived CLI process. Logged here as deliberately N/A to keep the "every dim is either evaluated or N/A" surface explicit.

---

### Resolved

<a id="r2-f1"></a>
**Finding 1 — Round 1 F1 (no performance budget) closed via DESIGN.md amendment (Dim 8)**

**Owner:** solution-owner
**Status:** validated
**Blocked by:** *(none)*
**Validator:** performance-engineer

[`DESIGN.md`](../../DESIGN.md) now contains a **§ Performance budget** section at lines 163-177, added per Round 2 fix-cycle adjudication of [PE Round 1 Finding 1](2026-05-20-performance-engineer.md#raised-to-so). The section declares all four components Round 1 requested:

1. **Scale ceiling** named: 10,000 bookmarks ([`DESIGN.md:173`](../../DESIGN.md)). The cumulative O(n²) cost beyond that bound is explicitly named as an accepted limitation, with the storage-architecture rationale.
2. **Per-command latency targets** named: `bm --help` / `bm --version` < 50 ms p95; `bm add` / `bm list` < 100 ms p95 at ≤ 1,000 bookmarks ([`DESIGN.md:167-171`](../../DESIGN.md) table).
3. **Measurement methodology** declared: manual observation; [`hyperfine`](https://github.com/sharkdp/hyperfine) acceptable for sanity-check ([`DESIGN.md:169-171`](../../DESIGN.md) "Measurement" column).
4. **Accepted limitations subsection** present: lines 173 + 175 + 177 explicitly name PE R1 F3 (cumulative O(n²)) + PE R1 F6 (pretty-print) + the Layer 1 deferrals of PE R1 F2 + PE R1 F5 with citations back to Round 1.

The spec is no longer silent on performance; every dependent finding from Round 1 (F2 / F3 / F5 / F6) now has a budget to evaluate against, and the disposition shape Round 1 forecast (F2 + F5 deferred, F3 + F6 accepted-limitation) is the disposition the spec text adopted. Round 1's headline gap — "every 'fast enough' claim is unfalsifiable" — is closed: the budget *is* the falsifiable claim against which Round 2's new-finding work below pressures the implementation.

**Resolution:** [`DESIGN.md`](../../DESIGN.md) § Performance budget (lines 163-177) added by [Solution Owner](../../../vsdd-suite/domains/role/SOLUTION-OWNER-REVIEW.md)-authority Round 2 fix cycle. Validator is performance-engineer (this round) per the [Review 77](../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-77--2026-05-20-1545z) validator-pair convention naming PE as the validator for budget declarations (Dim 8)

<a id="r2-f4"></a>
**Finding 4 — Round 1 F4 (`[profile.release]` absent) closed via Cargo.toml additions (Dim 3)**

**Owner:** platform-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** performance-engineer

[`Cargo.toml:48-53`](../../Cargo.toml) now declares an explicit `[profile.release]` block:

```toml
[profile.release]
opt-level = 3          # cargo default — declared for clarity
lto = "fat"            # max cross-crate inlining; slower compile, smaller + faster binary
codegen-units = 1      # single codegen unit enables full-crate optimization
panic = "abort"        # smaller binary; no unwinding (no catch_unwind in this crate)
strip = "symbols"      # strip debug symbols from the release binary
```

Round 1 F4 recommended `lto = "thin"`; the fix cycle chose `lto = "fat"` instead. The choice is **acceptable** — fat LTO is the more aggressive of the two and the trade-off (slower compile / smaller + faster binary) is the right one for a `cargo install`-deployed CLI where installs are rare and runs are frequent. The supplement's named-failure-mode framing ("Performance measurements taken against debug builds are not representative") is satisfied by *any* declared block; the Round 1 finding's underlying complaint was the silent-defaults state, not the specific value of `lto`. Choosing fat-over-thin is a deliberate strengthening, not a deviation.

`panic = "abort"` is consistent with bookmark-cli's lack of `catch_unwind` (verified via `grep -rE 'catch_unwind|panic::set_hook' src/` returning no matches). `strip = "symbols"` removes debug symbols from the installed binary — appropriate for a release-installable CLI. `codegen-units = 1` enables full-crate optimization. All four tuning knobs are present with one-line rationale comments per the TOML supplement § Platform Engineering "declare the chosen value and a one-line rationale" discipline (cited in [Platform Engineer Review 1 Finding 6](2026-05-20-platform-engineer.md)).

**Resolution:** [`Cargo.toml:48-53`](../../Cargo.toml) added by Round 2 fix cycle (Platform Engineer-owned; cross-references [Platform Engineer Review 1 Finding 6](2026-05-20-platform-engineer.md)). The release profile is now explicit and tuned. Validator: performance-engineer this round confirms the chosen values align with the supplement's § Performance Engineer recommendation that long-lived release artifacts favor fat LTO + single codegen unit (Dim 3)

---

### Deferred

<a id="r2-f2"></a>
**Finding 2 — Round 1 F2 (no benchmarking infrastructure) — Deferred discipline intact (Dim 9)**

**Owner:** platform-engineer
**Status:** raised
**Blocked by:** *(none)*

[`Cargo.toml:35-38`](../../Cargo.toml) `[dev-dependencies]` still contains only `assert_cmd`, `predicates`, `tempfile` — no `criterion`, no `divan`. There is no `benches/` directory and no `[[bench]]` entry in the manifest. The fix-cycle did not add a benchmark harness in Round 2.

The deferral discipline is, however, formally absorbed into the spec. [`DESIGN.md:175`](../../DESIGN.md) explicitly declares:

> **Benchmarking infrastructure:** [Layer 2+](TODO.md) work — Layer 1's surface is too small to benchmark meaningfully ([`criterion`](https://github.com/bheisler/criterion.rs) adds dependency cost without commensurate value at this scale). [Performance Engineer Review 1 Finding 2](vsdd-suite/review-log/2026-05-20-performance-engineer.md) declared **Deferred** at the layer level; the budget above is the contract a future Layer-2 benchmarking infrastructure would assert against.

This satisfies all three components of the [G-130](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-130) deferral-trigger discipline:

1. **Trigger named:** Layer 2 implementation start. Layer 2 adds `tag` + `--tag` filter — non-trivial second data point per the Round 1 trigger framing.
2. **Cost-of-deferral named:** Round 1 named it ("each additional layer lands without a baseline against which to detect regression"); the spec endorses the layer-2-is-the-natural-moment framing rather than weakening it.
3. **Auto-Backlog clause inherited from Round 1:** the original Round 1 auto-Backlog clause (Layer 2 R2 closure without `benches/` populated → auto-Backlog) continues to apply.

The deferral is therefore **disciplined**, not procrastinated. The Round 2 cold pass confirms no new evidence has emerged that would invalidate the Layer-2 trigger.

**Cost-of-deferral (carrying forward from Round 1):** Each layer that lands without a `benches/` harness leaves PE Dim 10 (regression risk) unable to fire — there is no baseline against which to detect regression. The Layer-2 trigger keeps that cost bounded: the harness lands when the second feature lands.

**Auto-Backlog clause:** If Layer 2 closes without `benches/` populated, the finding auto-Backlogs at Layer 2 R2 per the [G-130](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-130) auto-Backlog mechanism and re-raises as a Platform Engineer dim 38 (fresh-system install verification) coordination item.

**Classification:** Deferred — Layer 2 implementation start. The Round 1 deferral discipline is intact and the spec has now formally absorbed it; no new evidence supports overriding the Layer 2 trigger. Validator at Resolved time: platform-engineer (Dim 9)

<a id="r2-f5"></a>
**Finding 5 — Round 1 F5 (zero data-scaling tests) — Deferred discipline intact (Dim 4)**

**Owner:** quality-engineer
**Status:** raised
**Blocked by:** *(none)*

[`tests/bookmarks.rs`](../../tests/bookmarks.rs) and the `#[cfg(test)] mod tests` block in [`src/lib.rs:310-495`](../../src/lib.rs) still cap at ≤ 3 bookmarks. No `#[ignore]`-flagged scaling test has been added at 1K / 10K / 100K. The fix-cycle did not add scaling tests in Round 2.

The deferral discipline is, again, formally absorbed into the spec. [`DESIGN.md:177`](../../DESIGN.md) explicitly declares:

> **Data-scaling tests:** sentinel tests at the 100 / 1,000 / 10,000-bookmark cliffs land at Layer 2+ ([Performance Engineer Review 1 Finding 5](vsdd-suite/review-log/2026-05-20-performance-engineer.md) **Deferred**). At Layer 1 the existing `save_then_load_roundtrips` test exercises the 1-bookmark case; the layer's correctness is observable from there.

The three deferral-discipline components are satisfied:

1. **Trigger named:** Layer 2+ — the scaling tests need the bench-harness (Finding 2) to land first so that the wall-clock assertions can use a measurement substrate that matches what `cargo bench` measures.
2. **Cost-of-deferral named:** Round 1 named it ("each layer that lands without scaling tests means each PE round files Finding 3-shaped concerns from inspection alone — unverified hypotheses about scale behavior rather than measured findings").
3. **Auto-Backlog clause inherited from Round 1:** Layer 2 R2 closure without scaling tests → auto-Backlog as a QE Dim 4 concern.

The deferral coheres with Finding 2's trigger — both findings unblock together at Layer 2 implementation start, because the bench-harness and the scaling-test substrate share the fixture-generation surface (loop pushing N entries into a temp store).

**Cost-of-deferral:** As Round 1; Layer 2's `tag` operation will inherit the same N-bound shape and Round 1 F3's hypothesis will remain unverified-by-test until scaling tests exist.

**Auto-Backlog clause:** If Layer 2 closes without scaling tests, the finding auto-Backlogs at Layer 2 R2 and re-raises as a Quality Engineer dim 4 (data-scaling) concern.

**Classification:** Deferred — Layer 2 implementation start. Discipline intact; spec now endorses the deferral. Validator at Resolved time: quality-engineer (Dim 4)

---

### Accepted limitation

<a id="r2-f3"></a>
**Finding 3 — Round 1 F3 (O(n²) cumulative `save` cost on every `add`) — accepted limitation formally absorbed into spec (Dim 4)**

**Owner:** solution-owner
**Status:** validated
**Blocked by:** *(none)*
**Validator:** performance-engineer

[`src/lib.rs:130-174`](../../src/lib.rs) `BookmarkStore::save` continues to serialize the entire store and write the whole file on every call. The cumulative O(n²) cost over N sequential `bm add` invocations that Round 1 measured is unchanged at the algorithmic level. (The Round 2 atomic-save refactor changed the *write* mechanism — tmp-file + rename — but did not change the *quantity* of bytes serialized per call. See [Round 2 Finding 7](#r2-f7) below for the new finding the atomic-save mechanism introduced.)

The acceptance is now formally absorbed into the spec. [`DESIGN.md:173`](../../DESIGN.md) declares:

> **Scale ceiling:** 10,000 bookmarks. Beyond this the user should consider a real bookmark manager — this project's non-goals (§ Scope and non-goals) declare unsuitability for primary-use scale. The flat-JSON-rewrite-on-every-add design has cumulative O(n²) cost which makes large stores impractical; declared as **accepted limitation** at Layer 1 intent + named in [Performance Engineer Review 1 Findings 3 + 6](vsdd-suite/review-log/2026-05-20-performance-engineer.md).

This is the textbook acceptance shape the PE classification universe defines (`accepted limitation`: "deliberate performance trade-off, explicitly documented with the trade-off rationale"). The trade-off rationale is named — the project is the worked example of a suite that exercises the methodology end-to-end, not a production bookmark manager; the architectural simplicity (flat JSON, full rewrite) buys teaching value at the cost of scale.

The Round 1 conditional framing ("if ceiling ≥ 10K → real, needs Layer 2 storage refactor") is the alternate-history path the spec did NOT take. The spec adopted the 10K ceiling exactly — which means at the ceiling, per-`add` cost reaches ~100 KB read + parse + serialize + 100 KB write (Round 1's measured envelope). At a sub-millisecond-per-syscall budget that is ~5-20 ms per `add` at the ceiling — comfortably under the declared 100 ms p95 budget (DESIGN.md:170). The algorithm is fast enough for the declared scale; the deferral of the storage-format refactor to a hypothetical Layer-4-or-beyond is the right call.

**Classification:** Accepted limitation. The cost is real (O(n²) cumulative; the per-call envelope grows linearly with N), the trade is deliberate (spec-declared 10K ceiling + simplicity rationale), and the documentation lives in [`DESIGN.md`](../../DESIGN.md) § Performance budget per the PE classification universe's documentation requirement. The Round 2 verification surface is the spec amendment itself; no code change applies (Dim 4)

<a id="r2-f6"></a>
**Finding 6 — Round 1 F6 (`to_string_pretty` at scale) — accepted limitation formally absorbed into spec (Rust supplement § Performance Engineer — allocation patterns)**

**Owner:** solution-owner
**Status:** validated
**Blocked by:** *(none)*
**Validator:** performance-engineer

[`src/lib.rs:149`](../../src/lib.rs) continues to call `serde_json::to_string_pretty(self)` rather than `serde_json::to_string(self)`. The ~2x serialize-time cost and ~1.7x on-disk size are unchanged from Round 1.

The acceptance is now bound to the spec via two anchors:

1. [`DESIGN.md:107-114`](../../DESIGN.md) § Storage format example *is* pretty-printed JSON — readers internalize that the on-disk format is human-readable.
2. [`DESIGN.md:50`](../../DESIGN.md) § Non-goals names "manual JSON edit if needed" as the editing affordance — explicitly contemplates the user opening the file in a text editor, which presupposes readability.
3. [`DESIGN.md:173`](../../DESIGN.md) § Performance budget explicitly cites PE R1 F6 as accepted alongside F3.

The trade-off rationale Round 1 documented — *cost accepted: ~2x serialize cost at scale, ~1.7x on-disk size; benefit retained: human-readable storage file consistent with DESIGN.md's manual-edit affordance* — is now spec-attested rather than reviewer-asserted.

Sanity check on the budget interaction: at the 10K scale ceiling, the pretty-print serialize cost adds roughly a factor of two to the per-save serialize work. Round 1 envelope: ~10K records × ~100 bytes/record = ~1 MB serialize ≈ ~1-5 ms on commodity hardware; pretty-print ~2-10 ms. Comfortably inside the 100 ms p95 budget. The trade-off is real and accepted.

**Classification:** Accepted limitation. The cost is real (~2x serialize / ~1.7x file-size); the trade is deliberate (human-readable storage is named in DESIGN.md § Storage format example + § Non-goals manual-edit affordance); the spec absorbs the acceptance via PE R1 F6 citation in § Performance budget. No code change applies (Rust supplement § Performance Engineer — allocation patterns)

---

### Deferred

<a id="r2-f7"></a>
**Finding 7 — Round 2 atomic-save adds `sync_all` + `symlink_metadata` + `rename` syscalls per `bm add`; budget-impact unmeasured (Dim 10)**

**Owner:** software-engineer
**Status:** raised
**Blocked by:** *(none)*

The Round 2 fix cycle replaced the Round 1 single-call `fs::write` path with a four-step atomic-save sequence in [`src/lib.rs:130-174`](../../src/lib.rs):

1. [`src/lib.rs:132`](../../src/lib.rs) — `std::fs::symlink_metadata(path)` (extra `lstat(2)` syscall per save).
2. [`src/lib.rs:238-242`](../../src/lib.rs) — `OpenOptions::create_new(true).mode(0o600).open(tmp_path)` (open syscall on a sibling temp file).
3. [`src/lib.rs:243-244`](../../src/lib.rs) — `write_all(bytes)` + `write_all(b"\n")` (two write syscalls; Round 1 was one).
4. [`src/lib.rs:245`](../../src/lib.rs) — **`f.sync_all()`** — `fsync(2)` syscall.
5. [`src/lib.rs:161`](../../src/lib.rs) — `std::fs::rename(&tmp_path, path)` (rename syscall).

Total syscall count per `bm add` save grew from roughly **1 open + 1 write + 1 close** (Round 1's `fs::write`) to **1 lstat + 1 open + 2 writes + 1 fsync + 1 close + 1 rename** — a ~3-4x syscall-count increase with the **`fsync` being qualitatively new**.

**Why the `fsync` is the load-bearing concern, not the syscall count:** `std::fs::write` does NOT call `fsync`; it relies on the kernel page cache and lazy writeback. The Round 2 `sync_all()` forces the kernel to flush the file's contents AND metadata to durable storage before returning. On commodity hardware fsync latencies are:

- **SSD with no concurrent writes:** 0.1-1 ms — negligible.
- **SSD on a busy host or in a sync-monitored directory:** 1-10 ms — meaningful.
- **Spinning disk (HDD):** 5-50 ms — a significant fraction of the 100 ms p95 budget.
- **Network-mounted filesystem (NFS, SMB, syncthing-monitored dirs, Dropbox-watched dirs):** 50-500 ms — **can exceed the 100 ms p95 budget by itself**.

[`DESIGN.md:170`](../../DESIGN.md) § Performance budget declares `bm add` < 100 ms p95 at ≤ 1,000 bookmarks. The Round 1 budget calibration was performed against an `fs::write`-only path that did not fsync. The Round 2 path fsyncs, and the budget has not been re-measured. The supplement's regression-risk dim (Dim 10) names exactly this pattern: *"adding a synchronous operation in a hot code path"* — `sync_all` is a synchronous operation added to the `bm add` hot path between Round 1 and Round 2.

The fsync is **not gratuitous** — it is the durability half of the atomic-save discipline [Software Engineer Review 1 Finding 2](2026-05-20-software-engineer.md#r1-f2) and [Security Review 1 Finding 2](2026-05-20-security.md#r1-f2) prescribed. Without fsync, a crash between the rename and the kernel's writeback can leave the rename's metadata committed but the file contents lost — defeating the atomic-save guarantee. The atomic-save discipline is the correct call; the unmeasured budget impact is the finding.

**Verifiability:** the supplement's named tools all apply — `hyperfine 'bm add https://example.com' --warmup 3` against a 1K-entry fixture store on the project's declared reference platform (macOS / Linux on commodity 2020+ hardware per [`DESIGN.md:159`](../../DESIGN.md)) would confirm or refute the budget claim. The DESIGN.md § Performance budget already names `hyperfine` as acceptable measurement methodology; the Round 1 + Round 2 fix-cycle did not run it.

**Concrete next-action shape (for the validator-pair when this finding moves to Open / Assigned):**

- Run `hyperfine 'bm add https://example.com' --warmup 3 --runs 20` on a fresh-fixture 1K-entry store and confirm the median + p95 are < 100 ms on both an SSD-backed temp dir AND a non-SSD or network-backed temp dir (or document the platform constraint as part of the budget).
- If p95 exceeds 100 ms on any common reference platform, the disposition options are: (a) loosen the budget with platform-specific notes ("< 100 ms p95 on local SSD; < 250 ms on network-mounted filesystems"); (b) tighten the implementation (skip `sync_all` if the spec re-declares durability as a lower-priority concern than the latency budget — likely the wrong call given the atomic-save rationale); (c) accept the cross-storage-class variability as an explicit budget caveat.
- Cross-reference: [SE Review 1 Finding 2](2026-05-20-software-engineer.md#r1-f2) — that finding owns the *correctness* discipline (atomic-save guarantee); this finding owns the *cost* of that correctness.

**Trigger ([G-130](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-130)):** Layer 2 implementation start OR the first manual `hyperfine` measurement against the declared budget (whichever comes first). Layer 2 is the natural moment because the bench-harness from [Round 2 Finding 2](#r2-f2) lands then and the budget assertion gains a measurement substrate; the manual-hyperfine trigger catches the case where the operator runs hyperfine pre-Layer-2 for budget-claim verification.

**Cost-of-deferral:** Each `bm add` between now and the measurement is operating against an unverified p95 budget claim. The Round 2 spec amendment claims < 100 ms p95 at ≤ 1,000 bookmarks; if real-world measurement reveals 250 ms p95 on (say) a network-mounted home directory, the spec's budget claim is wrong and the worked example teaches an undermeasured budget as if it were measured. The pedagogical cost is exactly the kind of "budget without measurement" failure mode the PE domain prompt sycophancy check warns against: *"Flag any dimension where 'works in tests' is the only evidence of performance adequacy."* The spec currently declares a budget the project cannot demonstrate it meets.

**Auto-Backlog clause:** If Layer 2 closes without a `hyperfine` measurement of `bm add` against the declared 100 ms p95 budget, the finding auto-Backlogs at Layer 2 R2 and re-raises as a Platform Engineer fresh-system install-verification concern (the install-verification surface is the natural place to record measured-vs-claimed budget evidence per [Platform Engineer Review 1](2026-05-20-platform-engineer.md)).

**Classification:** Deferred — Layer 2 implementation start (or first hyperfine measurement, whichever comes first). The finding is real (`fsync` is a measurably new cost on the `bm add` hot path; the spec budget pre-dates the measurement) and bounded (the supplement's regression-risk dim names exactly this pattern; the resolution is one hyperfine invocation). Validator at Resolved time: platform-engineer (the install-verification surface is the natural home for measured-budget evidence) or software-engineer if the resolution requires a code change (Dim 10)

---

### Dismissed

*(none)*

---

### Hallucinated

*(none — Round 2 produced two `Resolved` findings (R2-F1, R2-F4) validating the Round 1 fix-cycle outputs, two `Deferred` findings (R2-F2, R2-F5) verifying the Deferred-with-named-trigger discipline is intact, two `Accepted limitation` findings (R2-F3, R2-F6) validating the spec-side acceptance, and one new `Deferred` finding (R2-F7) pressuring the Round 2 atomic-save addition. None of the seven findings was demonstrated as adversary-invented; the Round 2 atomic-save fsync addition (R2-F7) is supported by direct reading of the `src/lib.rs:130-174` code path and named explicitly in the Rust supplement's Performance Engineer dim ("synchronous operation in a hot code path"). The MVR signal is therefore **not yet reached** — Round 2 produced a new real finding (R2-F7), which fires the [G-131](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-131) continue-trigger and makes Round 3 mandatory after R2-F7's measurement work lands.)*

---

### Summary

7 findings filed: **2 Resolved** (R2-F1 spec amendment for budget; R2-F4 `[profile.release]` block), **2 Deferred** (R2-F2 bench infrastructure to Layer 2; R2-F5 scaling tests to Layer 2 — both with G-130 discipline intact from Round 1), **2 Accepted limitation** (R2-F3 cumulative O(n²) cost; R2-F6 pretty-print serialization — both with spec-side acceptance now formal in DESIGN.md § Performance budget), **1 new Deferred** (R2-F7 atomic-save `sync_all` + lstat + rename syscall additions add measurable cost to the `bm add` hot path that the declared budget has not been re-measured against).

**MVR signal: NOT REACHED.** Round 2 surfaced one new real finding (R2-F7) under PE Dim 10 (regression risk — *"adding a synchronous operation in a hot code path"*). Per the [G-131](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-131) continue-trigger discipline a single new real finding mandates Round N+1; PE Round 3 is therefore mandatory after R2-F7 measurement work lands. The Round 2 fix-cycle resolved every Round 1 finding (F1 + F4 via direct artifact change; F2 + F5 via spec-absorbed Deferred discipline; F3 + F6 via spec-absorbed accepted-limitation citations), and Round 2 itself did not produce a Hallucinated finding — the finding progression is **6 real Round 1 findings → 6 verified Round 1 resolutions + 1 new real Round 2 finding**, which is the canonical "fix cycle worked + cold pass catches the new defect the fix introduced" shape the primer's continue-trigger framing was written to handle.

**Coordination:** R2-F1 routes to [`vsdd-suite/SOLUTION-OWNER-REVIEW.md`](../SOLUTION-OWNER-REVIEW.md) — the spec amendment landed via Round 2 fix cycle but the SO log has not yet recorded the adjudication as a discrete SO Round entry; the natural follow-up is a Solution Owner Round 2 entry noting the DESIGN.md § Performance budget addition. R2-F4 cross-references [Platform Engineer Review 1 Finding 6](2026-05-20-platform-engineer.md) (the Platform Engineer round owns the manifest-edit work; this round owns the PE-side validation of the chosen values). R2-F2 + R2-F5 carry the same Layer 2 trigger and same auto-Backlog clauses Round 1 established; the per-finding routing to [`vsdd-suite/PLATFORM-ENGINEER-REVIEW.md`](../PLATFORM-ENGINEER-REVIEW.md) (bench harness) and [`vsdd-suite/QUALITY-ENGINEER-REVIEW.md`](../QUALITY-ENGINEER-REVIEW.md) (scaling tests) is unchanged from Round 1. R2-F7 routes to [`vsdd-suite/SOFTWARE-ENGINEER-REVIEW.md`](../SOFTWARE-ENGINEER-REVIEW.md) (atomic-save owner; the durability/cost trade-off is the SE-owned interface) with [`vsdd-suite/PLATFORM-ENGINEER-REVIEW.md`](../PLATFORM-ENGINEER-REVIEW.md) as the validator pair (the install-verification surface is the natural home for measured-budget evidence). Round 1's anchor list (`<a id="r1-f1">` through `<a id="r1-f6">`) remains the cross-reference surface for the original findings; Round 2's anchors (`<a id="r2-f1">` through `<a id="r2-f7">`) are the cross-reference surface for the verifications + the new R2-F7 finding.

---

## Review 3 — 2026-05-20 22:00Z

**Layer:** 1
**Tested against:** commit `9b915b1` (current `main` as of 2026-05-20)
**Round:** 3
**Active domain set:** 11 role + 1 meta = 12 (per [DESIGN.md § Project intent](../../DESIGN.md))
**Scope:** Cold-context [Performance Engineer](../../../../vsdd-suite/domains/role/PERFORMANCE-ENGINEER-REVIEW.md) IAR Round 3 verification of [Round 2 PE Finding R2-F7](2026-05-20-performance-engineer.md#r2-f7) — the atomic-save `fsync`/`lstat`/`rename` syscall-additions Deferred-not-blocking status against the current [DESIGN.md § Performance budget](../../DESIGN.md). Round 2 closed at MVR-not-yet-reached on the strength of R2-F7 alone; this Round 3 cold pass verifies (a) the perf budget declarations in DESIGN.md haven't changed since Round 2, (b) R2-F7's Deferred-with-named-trigger discipline is intact, (c) no new perf defects in the now-stabilized Layer 1 surface.
**Lens:** PE Dim 4 (Data scaling), Dim 7 (Memory growth — N/A for short-lived CLI but re-checked), Dim 8 (Performance budget), Dim 9 (Performance testing methodology), Dim 10 (Regression risk). [Rust supplement](../../../../vsdd-suite/supplements/rust.md) § Performance Engineer (Criterion benchmarks; debug-vs-release profile; allocation patterns) applied to the post-R2-fix state — no new code paths since Round 2 close, so this round is primarily verification rather than fresh-surface adversarial pressure.
**Session note:** Cold cluster-batched session. Independent cold pass for PE — no reasoning leak from the SE sub-section above. The R2 round closed with R2-F7 as the new finding; the R2 finding was classified Deferred at the time with the Layer 2 implementation start trigger named. This Round 3 pass verifies the Deferred discipline is intact, not that R2-F7 has been resolved (resolution requires `hyperfine` measurement against the declared budget, which is operator-executable at Layer 2 start per the R2 trigger framing).
**Source:** `domain-raised` — applying PE Dims 8, 9, 10 + the Rust supplement § Performance Engineer to the current artifacts.
**Scope carve-outs (unchanged from R1/R2):** Dim 1 (time-to-interactive — browser apps), Dim 2 (main-thread / event-loop blocking), Dim 3 (asset optimization — browser), Dim 5 (N+1 access patterns — single-invocation CLI), Dim 6 (caching / memoization in long-lived process), Dim 7 (memory growth over long sessions) remain inapplicable to a short-lived CLI process per [Round 1 § Scope carve-outs](2026-05-20-performance-engineer.md#review-1--2026-05-20-1930z) and the Round 2 re-statement.
**Assumption surfacing:** [DESIGN.md:163-177](../../DESIGN.md) § Performance budget remains the contract this round evaluates against. The R2-F7 finding correctly cited the budget at the time (`< 100 ms p95 at ≤ 1,000 bookmarks` for `bm add` / `bm list`; `< 50 ms p95` for `bm --help` / `bm --version`). Verified the budget table at lines 167-171 of the current DESIGN.md still declares those exact values — the budget has not silently widened or tightened since Round 2, so R2-F7's measurement-gap framing applies unchanged.

---

### Deferred

**Finding 1 — R2-F7 (atomic-save `fsync` + `lstat` + `rename` syscall cost) — Deferred-not-blocking status intact (Dim 10)**

<a id="r3-f1"></a>

**Owner:** software-engineer (atomic-save owner per R2-F7 routing)
**Status:** raised (Deferred carrying R2-F7 trigger forward)
**Blocked by:** *(none — the deferral is to Layer 2 implementation start OR first `hyperfine` measurement, whichever comes first; the named trigger is operator-executable rather than block-on-prior-finding.)*

[`src/lib.rs:148-201`](../../src/lib.rs) `BookmarkStore::save` continues to use the atomic-save sequence Round 2 identified: `symlink_metadata` (line 150) → `OpenOptions::create_new + mode 0600` (lines 265-269) → `write_all(bytes)` + `write_all(b"\n")` (lines 270-271) → `sync_all` (line 272) → `rename` (line 188). The syscall count and the `fsync` cost contour are unchanged from Round 2 — no code-level revision to `save` has landed between Round 2 and Round 3.

[`DESIGN.md:163-177`](../../DESIGN.md) § Performance budget is unchanged since Round 2:

| Metric | Budget (p95) | Measurement |
|---|---|---|
| `bm --help` / `bm --version` startup | < 50 ms wall-clock on commodity laptop | Manual observation; `hyperfine` acceptable for sanity-check |
| `bm add <url>` end-to-end | < 100 ms wall-clock on a store with ≤ 1,000 bookmarks | Same |
| `bm list` end-to-end | < 100 ms wall-clock on a store with ≤ 1,000 bookmarks | Same |

The R2-F7 finding's central claim — the `fsync` introduces a measurable cost the declared budget hasn't been re-measured against — remains true at the cold-read level. The supplement's [Rust § Performance Engineer](../../../../vsdd-suite/supplements/rust.md) "synchronous operation in a hot code path" failure-mode framing applies to `sync_all` as it did at Round 2. No `hyperfine` measurement record has been added between Round 2 and Round 3 (verified: `grep -rn "hyperfine" vsdd-suite-reference-examples/bookmark-cli-manual/` returns only the DESIGN.md table reference at line 169 and prose mentions in [Round 2 R2-F7](2026-05-20-performance-engineer.md#r2-f7) — no measurement artifact, no `manual-tests/perf-baseline.md`, no CI workflow step).

The deferral discipline is intact per the three-component [G-130](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-130) check:

1. **Trigger named:** Layer 2 implementation start OR first manual `hyperfine` measurement against the declared budget (whichever comes first). The Round 2 framing established the dual-trigger shape; the Round 3 cold pass confirms no new evidence has emerged that would invalidate the Layer-2-or-first-measurement trigger.
2. **Cost-of-deferral named:** Round 2 named it ("Each `bm add` between now and the measurement is operating against an unverified p95 budget claim ... if real-world measurement reveals 250 ms p95 on (say) a network-mounted home directory, the spec's budget claim is wrong"). Cost is bounded and operator-resolvable; not a regression introduced this round.
3. **Auto-Backlog clause inherited from Round 2:** Layer 2 R2 closure without a `hyperfine` measurement → auto-Backlog and re-raise as a Platform Engineer fresh-system install-verification concern. Inherited cleanly.

The cold-pass independent re-application of PE Dim 10: the `save` hot path's syscall sequence is unchanged from Round 2; no new synchronous operations have been added; no widening of data access patterns has occurred. R2-F7 is the only outstanding regression-risk surface and it remains correctly Deferred.

**Cost-of-deferral (Round 3 update):** Carrying forward from Round 2 — the cost-of-deferral framing has not changed because the artifact state is identical to Round 2 close. The 100 ms p95 budget remains a spec-claimed-but-not-measured number for the post-atomic-save build.

**Auto-Backlog clause:** Unchanged from Round 2. If Layer 2 closes without a `hyperfine` measurement of `bm add` against the declared 100 ms p95 budget, the finding auto-Backlogs at Layer 2 R2 and re-raises as a Platform Engineer fresh-system install-verification concern.

**Classification:** Deferred — Layer 2 implementation start (or first hyperfine measurement, whichever comes first). The Round 2 deferral discipline is intact; no new evidence supports overriding the trigger. (Dim 10)

---

### Resolved

*(none — Round 3 carries no fix verifications. The R2 round closed all R1 findings as Resolved or Accepted limitation or Deferred-with-discipline-intact; the only outstanding finding at Round 2 close was R2-F7, which is operator-/Layer-2-blocked and not AI-resolvable in this PR cycle.)*

---

### Accepted limitation

*(none new — R1-F3 (cumulative O(n²) cost) and R1-F6 (pretty-print serialization) remain Accepted limitation per [Round 2 R2-F3](2026-05-20-performance-engineer.md#r2-f3) and [R2-F6](2026-05-20-performance-engineer.md#r2-f6); the DESIGN.md § Performance budget acceptance citations are unchanged.)*

---

### Dismissed

*(none)*

---

### Hallucinated

*(none — the single Deferred finding above is R2-F7 carried forward with the Round 2 deferral discipline intact; the cold pass surfaces no new defects. Per the [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) discipline, a finding that is Deferred-discipline-intact at Round N+1 is not Hallucinated — the verification IS the round's work product, and the Deferred status is the methodology-correct routing for a finding whose resolution requires operator-executable measurement.)*

---

### Summary

1 finding classified: 1 Deferred (R3-PE-F1 / R2-F7 carried forward with G-130 discipline intact); 0 Resolved; 0 Accepted limitation new; 0 Dismissed; 0 Hallucinated; 0 new findings.

The Round 2 → Round 3 cycle introduced no new perf-relevant code paths. The atomic-save sequence at [`src/lib.rs:148-201`](../../src/lib.rs) is unchanged; the perf budget declarations at [`DESIGN.md:163-177`](../../DESIGN.md) are unchanged; the R2-F7 measurement gap remains correctly Deferred to Layer 2 implementation start or first `hyperfine` measurement. The cold-pass independent re-application of PE Dims 4, 8, 9, 10 produced no new findings: no widening of data access patterns, no new synchronous operations in hot paths, no budget drift, no new dependency additions that would change allocation patterns.

**MVR signal: NOT YET REACHED for the Performance Engineer domain.** Per [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) § Round triggers [G-131](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-131) continue-trigger discipline, the layer is at MVR when "the round *after* the last new-finding round produces only Hallucinated findings or no findings." Round 2 raised R2-F7 as a real Deferred finding; Round 3 (this round) raised zero new findings and verified R2-F7's Deferred discipline is intact. On the strict interpretation of the stop trigger, this is the MVR-equivalent state: the round after the last new-finding round (Round 2) produced no new findings. **However**, R2-F7 itself is Deferred-not-Resolved, and the budget claim it cited remains unmeasured. The methodology-correct posture is: **MVR-blocked-by-deferred-measurement** — the AI-resolvable surface is at MVR, but the budget-verification gap that R2-F7 named is operator-executable rather than AI-executable, so the domain advances to "no further AI-executable findings" rather than to fully-Resolved MVR. The Round 4 trigger fires when the operator runs `hyperfine` against `bm add` and either confirms the < 100 ms p95 claim (R2-F7 closes as Resolved) or surfaces a budget violation (R2-F7 escalates to a new in-tree finding).

The R2-F7 Deferred status holds the right shape: trigger named, cost-of-deferral named, auto-Backlog clause inherited. The Round 3 work product is verifying the discipline, not producing fresh findings.

**Coordination:**

- [Finding 1](#r3-f1) (R2-F7 carried forward) — routes to [`vsdd-suite/SOFTWARE-ENGINEER-REVIEW.md`](../SOFTWARE-ENGINEER-REVIEW.md) as the atomic-save owner; [`vsdd-suite/PLATFORM-ENGINEER-REVIEW.md`](../PLATFORM-ENGINEER-REVIEW.md) remains the validator pair (the install-verification surface is the natural home for measured-budget evidence per Round 2's framing). The validator-pair routing is unchanged from Round 2.
- Cross-cluster: this PE round closes at MVR-blocked-by-deferred-measurement independently of the SE sub-section above (which closed at MVR-reached) and the Platform sub-section below. The cluster-batched session does not require all three domains to share a single MVR state — each domain advances independently per its own finding progression.

---
