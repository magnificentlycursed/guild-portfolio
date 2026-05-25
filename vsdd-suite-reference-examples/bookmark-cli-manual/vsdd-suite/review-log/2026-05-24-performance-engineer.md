# Performance Engineer Review — 2026-05-24

---

## Review 1 — 2026-05-24 01:15Z

**Scope:** Layer 3 cold-context [Phase 3](../../../vsdd-suite/primers/3-review-session.md) IAR Round 1 for the Performance Engineer domain. Layer 3 scope only: commits `878d3b6` + `fd21900` + `78bd3cf` — `bm export` + `bm import` implementation in [`src/lib.rs`](../../src/lib.rs) (`export_json`, `import_json`, `ImportError`, `MAX_STDIN_BYTES_DEFAULT`) and [`src/main.rs`](../../src/main.rs) (`run_export`, `run_import`, `Cmd::Export`, `Cmd::Import`). Regression check against Layers 1 + 2 baselines established in prior PE rounds 1–5 ([`2026-05-20-performance-engineer.md`](2026-05-20-performance-engineer.md)).

**Lens:** Performance Engineer — adversarial posture per [`3-review-session.md`](../../../vsdd-suite/primers/3-review-session.md). Primary obligation is to the spec ([`DESIGN.md`](../../DESIGN.md) § `bm export` (Layer 3) + § `bm import` (Layer 3) + § Performance budget + § Filesystem-coverage caveat); the implementation may be correct and still be a finding if its performance shape is under-declared or creates a new scaling cliff relative to the Layer 2 baseline.

**Session note:** Cold session — no participation in the Layer 3 implementation. Artifacts read in adversarial order: [`DESIGN.md`](../../DESIGN.md) (§ Performance budget, § Filesystem-coverage caveat, § `bm export` Layer 3, § `bm import` Layer 3), [`src/lib.rs`](../../src/lib.rs) (`export_json` + `import_json` implementations), [`src/main.rs`](../../src/main.rs) (`run_import` stdin-read path), [`tests/bookmarks.rs`](../../tests/bookmarks.rs) (Layer 3 Red Gate tests AC 14–AC 28 at lines 1065+), [`tests/scaling.rs`](../../tests/scaling.rs) (existing scaling sentinels), [`manual-tests/layer-2.md`](../../manual-tests/layer-2.md) (prior hyperfine sanity-check shape), [PE reviews 1–5](2026-05-20-performance-engineer.md) (prior finding progression, especially R1 F1 budget, R1 F2 hyperfine, R1 F5 filesystem-coverage caveat). **Supplements applied:** [`rust.md`](../../../vsdd-suite/supplements/rust.md) § Performance Engineer (Criterion benchmarks; allocation patterns; debug-vs-release).

**Source:** `domain-raised` — cold adversary applying PE [Standard Evaluation Dimensions](../../../vsdd-suite/domains/role/PERFORMANCE-ENGINEER-REVIEW.md) (Dims 4, 8, 9, 10) plus the [`rust.md`](../../../vsdd-suite/supplements/rust.md) § Performance Engineer supplement (allocation patterns) against the Layer 3 surface.

**Regression check:** Layer 1 + Layer 2 PE rounds (R1–R5) established: budget table in [`DESIGN.md`](../../DESIGN.md) § Performance budget (< 100 ms p95 at ≤ 1,000 bookmarks); scaling sentinels at 100/1K/10K in [`tests/scaling.rs`](../../tests/scaling.rs); filesystem-coverage caveat declared in DESIGN.md; `[profile.release]` tuned (R2 F1); hyperfine sanity-check at [`manual-tests/layer-2.md`](../../manual-tests/layer-2.md) Step 12. Layer 3 adds two new subcommands. The regression question: do `export_json` + `import_json` introduce new scaling cliffs or memory spikes that violate the established budget, independent of whether the prior findings are still open?

**Scope carve-outs (unchanged from prior rounds):** Dim 1 (time-to-interactive — browser apps), Dim 2 (main-thread / event loop blocking), Dim 3 (asset optimization — browser), Dim 6 (caching / memoization in long-lived process), Dim 7 (memory growth over long sessions) remain inapplicable to a short-lived single-invocation CLI. Logged as deliberately N/A to keep the dim-completeness surface explicit.

---

### Accepted limitation

<a id="r1-f1"></a>
**Finding 1 — `import_json` dedup-via-`Vec::contains` is O(N²) at the 10K scale ceiling (Dim 4)**

<a id="r1-f1"></a>

**Owner:** software-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** software-engineer

[`src/lib.rs:560`](../../src/lib.rs) implements the dedup loop as:

```rust
for new_bm in imported {
    if !self.bookmarks.contains(&new_bm) {
        self.bookmarks.push(new_bm);
        appended += 1;
    }
}
```

`Vec::contains` performs a linear scan of `self.bookmarks` on each iteration. At the scale ceiling of 10,000 bookmarks with a 10,000-bookmark import payload, this is 10,000 × 10,000 = **100 million comparisons** in the worst case. `Bookmark`'s `PartialEq` compares all three fields (`url` + `timestamp` + `tags`); each comparison allocates no heap memory but performs string equality checks on heap-allocated `url` and `tags` fields. At 10K × 10K the dedup step alone is O(N²) in the destination-state size times the import-payload size.

The spec ceiling declared in [`DESIGN.md`](../../DESIGN.md) § Performance budget is 10,000 bookmarks ("Beyond this the user should consider a real bookmark manager"). A `bm export | bm import` round-trip at the ceiling is the canonical workflow: a 10K-bookmark export produces a ~10K-record payload; importing that payload into a pre-existing 10K-bookmark destination store triggers the full 100M-comparison worst case.

**Is this within the declared budget?** The budget table at [`DESIGN.md`](../../DESIGN.md) § Performance budget names `< 100 ms p95 at ≤ 1,000 bookmarks` for `bm list` and `bm add`. `bm import` is not explicitly budgeted. The filesystem-coverage caveat (`DESIGN.md` § Performance budget § Filesystem-coverage caveat) addresses fsync latency on the save path but not the in-process dedup cost. The 10K ceiling implies a dedup at 10K × 10K is within-ceiling behavior; whether it satisfies a "fast enough" claim depends on hardware, but 100M string comparisons is measurably more expensive than the 10K-item sort in `newest_first` (which Layer 1 R1 F3 accepted as the O(N²) cumulative-cost-on-add pattern). The dedup cost on a single import invocation can plausibly exceed the 100 ms budget for `bm list` / `bm add` — making `bm import` the new worst-case single-command duration on the project.

**O(N log N) alternative:** replacing the `Vec::contains` dedup with a `HashSet<&Bookmark>` (requires `Bookmark: Hash`) or sorting + dedup would reduce the import loop from O(M × N) to O((M + N) log(M + N)) where M is the import payload size and N is the destination state size. For the 10K × 10K worst case: 20,000 × log₂(20,000) ≈ 290,000 operations vs. 100 million — three orders of magnitude better. The cost is adding `#[derive(Hash)]` to `Bookmark` (timestamps and strings are hashable) and constructing a `HashSet` of the existing store. For a single-shot CLI this allocation is a one-time per-invocation cost.

**Why this is an accepted limitation, not a blocker.** [`DESIGN.md`](../../DESIGN.md) § `bm import` explicitly states: *"dedup runs BOTH against existing destination state AND within the imported payload itself."* The spec names the semantic (exact-tuple-match dedup) and the implementation faithfully delivers it. The O(N²) cost is an emergent consequence of using `Vec::contains` for correct exact-match dedup; the spec does not prescribe the algorithm. The existing O(N²) cumulative-cost-on-add pattern (Layer 1 PE R1 F3, accepted-limitation at DESIGN.md § Performance budget) is a precedent for accepting O(N²)-class costs at the 10K ceiling for a single-user manual-rate tool. The DESIGN.md § Performance budget § Scale ceiling framing (*"Beyond 10K the user should consider a real bookmark manager"*) explicitly caps the warranty; at the ceiling an O(N²) dedup step is painful but within the declared scope of "impractical at large stores."

**What this finding contributes.** The spec's performance budget does not mention `bm import`. The accepted-limitation rationale requires the trade-off to be *"explicitly documented with the trade-off rationale"* (PE classification universe). The finding's contribution is: **the DESIGN.md § Performance budget section should acknowledge the O(N²) import-dedup cost alongside the existing O(N²) cumulative-add-cost**, with a pointer to the `bm import` (Layer 3) behavioral contract. Without this documentation, a cold reader of DESIGN.md sees the add O(N²) documented but the import O(N²) silently unacknowledged — two scaling cliffs, one documented.

**Proposed DESIGN.md addition (for SO adjudication):** In § Performance budget, under the accepted-limitations paragraph that names PE R1 F3 (cumulative O(n²) cost on add), add: *"Layer 3 `bm import` dedup-via-`Vec::contains` is O(M × N) where M = import payload size and N = destination state size; at the 10K × 10K worst-case ceiling this is ~100M comparisons per import invocation — measurably more expensive than the per-add O(N) cost declared above. Accepted limitation at the same capstone-scope rationale: operators with legitimately-large imports at the ceiling should expect `bm import` to be the slowest single command in the suite. The `HashSet`-based alternative is the natural Layer 4 refactor target if import latency surfaces as a user complaint."*

**Classification:** Accepted limitation — subject to the DESIGN.md documentation addition above. The implementation is correct and spec-faithful; the O(N²) shape is a deliberate consequence of the Vec-linear dedup algorithm rather than an oversight. The finding routes to SO for the DESIGN.md documentation amendment; the code path does not require a fix at Layer 3.

---

### Hallucinated

<a id="r1-f2"></a>
**Finding 2 — `export_json` sort cost introduces a new scaling cliff (Dim 4, Dim 10)**

<a id="r1-f2"></a>

**Owner:** *self*
**Status:** raised
**Blocked by:** *(none)*
**Validator:** *self* — hallucinated finding; self-classification with evidence.

The adversarial hypothesis: `export_json` calls `newest_first()` (or `filter_by_tags`, which itself calls `newest_first`) before serializing, adding an O(N log N) sort to the export hot path that does not exist on the `bm list` path.

**Why this is hallucinated.** `bm list` ([`src/main.rs:315-319`](../../src/main.rs)) also calls `store.newest_first()` before printing each bookmark. The sort cost is identical between `bm list` and `bm export` — both call `newest_first()` on the full store (or `filter_by_tags` which calls `newest_first()` internally). Layer 2 PE rounds already accepted the sort cost as within the 100 ms budget at ≤ 1,000 bookmarks per the [`manual-tests/layer-2.md`](../../manual-tests/layer-2.md) Step 12 hyperfine sanity-check. The `export_json` path adds one pass to collect into a `Vec<serde_json::Value>` and one `serde_json::to_string` call, which are O(N) passes — no worse asymptotically than the line-by-line `println!` in `bm list`. The sort itself is the same `sort_by_key` on `DateTime<Utc>` (comparable by integer arithmetic, not heap allocation). No new scaling cliff is introduced.

**Classification:** Hallucinated. The concern was adversarially plausible (export is a new path; sort cost is real) but the implementation is parallel to `bm list`'s existing verified path. The Layer 2 PE round's hyperfine attestation covers the sort cost at the 1K cliff; `bm export` shares that attestation by code inspection.

---

<a id="r1-f3"></a>
**Finding 3 — `serde_json::to_string` + `String` allocation in `export_json` is unacceptable at the 10K scale ceiling (Rust supplement § Performance Engineer — allocation patterns)**

<a id="r1-f3"></a>

**Owner:** *self*
**Status:** raised
**Blocked by:** *(none)*
**Validator:** *self* — hallucinated finding; self-classification with evidence.

The adversarial hypothesis: `export_json` builds a `Vec<serde_json::Value>` (one heap allocation per bookmark), then wraps it in a `serde_json::json!({...})` value, then calls `serde_json::to_string` — producing a large `String` allocation of the entire JSON output in one shot. At 10K bookmarks with ~1KB each, this is a ~10 MB in-memory allocation before any byte reaches stdout. This is the single-shot "buffer the entire output before writing" anti-pattern.

**Why this is hallucinated.** This is an **accepted limitation already established at Layer 1**. `BookmarkStore::save` ([`src/lib.rs:263`](../../src/lib.rs)) uses `serde_json::to_string_pretty(self)` — same pattern, same large-String-in-memory shape — and was classified **Accepted limitation** (PE R1 F6) at Layer 1 because the single-shot CLI binary's process lifetime makes the allocation cost acceptable: the memory is released at process exit, and a CLI that runs for < 100 ms total does not exhibit memory growth. `export_json` uses `serde_json::to_string` (compact, not pretty) — actually cheaper than `save`'s `to_string_pretty`. The streaming alternative (`serde_json::to_writer` into `BufWriter<Stdout>`) would reduce peak memory at the cost of added complexity; the spec's "single-user manual-entry rate" framing accepts the current approach.

**Classification:** Hallucinated. The allocation pattern is real but is not a new concern — it mirrors the `save` path's accepted limitation from Layer 1 PE R1 F6. The adversarial surface here is identical to the already-accepted one; raising it as a new Layer 3 finding without new evidence is not a real defect.

---

<a id="r1-f4"></a>
**Finding 4 — `take(cap+1)` allocates up to `max_stdin_bytes + 1` bytes in a single `Vec::new()` — unacceptable for a single-shot CLI (Dim 4)**

<a id="r1-f4"></a>

**Owner:** *self*
**Status:** raised
**Blocked by:** *(none)*
**Validator:** *self* — hallucinated finding; self-classification with evidence.

The adversarial hypothesis: [`src/main.rs:424-434`](../../src/main.rs) uses:

```rust
let mut bytes = Vec::new();
let cap_plus_one = u64::try_from(max_stdin_bytes)
    .unwrap_or(u64::MAX)
    .saturating_add(1);
if let Err(e) = std::io::stdin().take(cap_plus_one).read_to_end(&mut bytes) { ... }
```

`Vec::new()` starts empty and grows lazily as `read_to_end` reads; the OS does not commit the full 10 MB at `Vec::new()` time. `read_to_end` will grow the `Vec` incrementally as stdin data arrives. The concern "allocates up to 10 MB in one shot" is imprecise: the actual allocation is the final working-set size of the stdin payload, not a pre-committed 10 MB. For a 1 KB payload, `bytes` grows to ~1 KB total. For a 10 MB payload at the cap, `bytes` grows to 10 MB (plus one byte to detect the cap, which the `len > max_stdin_bytes` check handles). The 10 MB ceiling is the **design choice** documented in [`DESIGN.md`](../../DESIGN.md) § Threat model addition for stdin-fed attacker input: *"accepted-limitation framing — operators with legitimately-larger imports override via `--max-stdin-bytes <N>`."*

**Why this is hallucinated.** The allocation pattern is documented and spec-justified. `read_to_end` is the idiomatic Rust approach for bounded stdin reading; the alternative (`BufRead` + incremental parse) would require a streaming JSON parser that `serde_json` does not provide out of the box. The `take(cap+1)` pattern correctly bounds the read without pre-allocating the full cap. The 10 MB working-set ceiling for a single-shot CLI binary is proportionate — the process exits after `bm import` completes, releasing all allocations. Per the Rust supplement's accepted OOM-as-panic policy for in-memory `Value` serialization (cited in `export_json`'s `#[allow(clippy::unwrap_used)]` comment), a CLI binary with a 10 MB bound can accept the in-process allocation with no heap-growth concern.

**Classification:** Hallucinated. The concern applies to long-running servers, not single-shot CLI binaries. The 10 MB bound, lazy growth, and single-use process lifetime make this a non-finding at the capstone scope.

---

<a id="r1-f5"></a>
**Finding 5 — 10 MB stdin cap is too restrictive to cover the full 10K-bookmark scale ceiling (Dim 8)**

<a id="r1-f5"></a>

**Owner:** *self*
**Status:** raised
**Blocked by:** *(none)*
**Validator:** *self* — hallucinated finding; self-classification with evidence.

The adversarial hypothesis: the 10K-bookmark scale ceiling implies a ~10 MB export payload (10,000 bookmarks × ~1 KB each). The 10 MB default cap on `bm import` could reject a legitimate 10K-bookmark `bm export | bm import` round-trip if the bookmarks are at the upper bound of per-record size.

**Why this is hallucinated.** [`DESIGN.md`](../../DESIGN.md) § `bm import` (Layer 3) explicitly addresses this: *"Default cap: 10 MB (matches the project's existing scale ceiling of 10,000 bookmarks at ~1 KB each)."* The cap is sized to the scale ceiling's typical-case estimate. The per-bookmark size of ~1 KB is an estimate for typical URLs (~100-200 chars) + typical tags (a few short labels); a store at the ceiling of 10K bookmarks with typical records fits within 10 MB. The cap is not a hard block on 10K-record imports — it is proportionate to the declared ceiling. For atypically large records (very long URLs; many tags), the `--max-stdin-bytes <N>` override exists precisely for this case, and DESIGN.md documents it. The integration test `tests_import_stdin_size_cap_enforced` ([`tests/bookmarks.rs:1566`](../../tests/bookmarks.rs)) verifies the cap enforcement with an 11 MB payload, confirming the boundary fires correctly.

**Classification:** Hallucinated. The cap is correctly sized and documented; the override mechanism covers edge cases. The concern assumes typical bookmarks are at the maximum URL + tag size, which is not the project's declared use case.

---

<a id="r1-f6"></a>
**Finding 6 — `import_json`'s parse-everything-before-mutate atomicity adds one extra full-document parse pass (Dim 4)**

<a id="r1-f6"></a>

**Owner:** *self*
**Status:** raised
**Blocked by:** *(none)*
**Validator:** *self* — hallucinated finding; self-classification with evidence.

The adversarial hypothesis: [`src/lib.rs:538-540`](../../src/lib.rs) performs `serde_json::from_value(bookmarks_value.clone())` which clones the `serde_json::Value` array before deserializing each `Bookmark`. At 10K records, this is a full in-memory copy of the parsed JSON tree — a non-trivial allocation.

**Why this is hallucinated.** The `bookmarks_value.clone()` is necessary because `serde_json::from_value` takes ownership of the `Value` argument; `bookmarks_value` is a borrow from the top-level `value`, which is still alive. The pattern is correct and idiomatic for `serde_json::from_value`'s ownership model. The cost is one allocation of the `Value::Array(Vec<Value>)` — proportionate to the already-parsed input. The alternative (`serde_json::from_slice` on the raw payload with a custom deserializer) would be premature at this scale. The atomicity guarantee (*"All validation happens before any mutation to `self.bookmarks`"*, per the `import_json` doc comment) requires the pre-parse-then-mutate shape; streaming-and-mutate would violate the spec's partial-import-MUST-NOT-occur contract.

**Classification:** Hallucinated. The clone is a necessary consequence of the spec's atomicity requirement and `serde_json::from_value`'s API. The allocation cost at 10K is real but proportionate and within the accepted O(N) complexity class.

---

<a id="r1-f7"></a>
**Finding 7 — No `manual-tests/layer-3.md` hyperfine check for `bm export` + `bm import` (Dim 9)**

<a id="r1-f7"></a>

**Owner:** platform-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** performance-engineer

[`manual-tests/layer-2.md`](../../manual-tests/layer-2.md) Step 12 established a `hyperfine` sanity-check for the Layer 2 operations (`bm list` / `bm list --tag` / `bm tag`) against a 1,000-bookmark store. [`DESIGN.md`](../../DESIGN.md) § Performance budget names this as the "proportionate Layer 2 closure of [Performance Engineer Review 1 Finding 2]." Layer 3 adds two new subcommands — `bm export` and `bm import` — each of which touches the full store at the 1K / 10K scale. No `manual-tests/layer-3.md` file exists.

**The gap.** The Layer 2 hyperfine sanity-check closes PE R1 F2 at the Layer 2 surface. The Layer 3 equivalents — `bm export` (full store serialization) and `bm import` (full store parse + dedup) — are performance surfaces that the Layer 2 hyperfine step does not cover. At the 1K scale, `bm export` involves the same sort + a full JSON re-serialization of the store to stdout; `bm import` adds the dedup loop. Without a per-layer hyperfine step, PE Dim 9 (performance testing methodology) cannot attest that Layer 3 operations fall within the budget at the 1K or 10K cliff.

**The finding is concrete, not abstract.** The Layer 2 hyperfine step was expressly required by the spec: *"a documented `manual-tests/layer-2.md` step generates a 1,000-bookmark store and asserts each named-budget operation completes within the budget."* The spec's next sentence says *"The `criterion` framework remains deferred."* The same principle should extend to Layer 3: a `manual-tests/layer-3.md` file with a `hyperfine` step covering `bm export` and `bm import` at the 1K cliff is the Layer 3 analog of Step 12. Its absence means Layer 3 ships with no measured performance attestation for its new operations.

**Why this is not Hallucinated.** Unlike Findings 2–6 above, this finding points to a real absence in the project artifact set: `ls manual-tests/` shows `install-verification.md`, `layer-1.md`, `layer-2.md` — no `layer-3.md`. The Layer 2 precedent makes the absence of a Layer 3 counterpart a documented methodology gap rather than an adversary-invented concern.

**Proposed addition (for Platform Engineer / operator authoring):** A `manual-tests/layer-3.md` file mirroring the Layer 2 Step 12 pattern:

- Step: generate a 1,000-bookmark store via the Layer 2 Python fixture generator.
- Step: run `hyperfine --warmup 3 --runs 10 'bm export'` and assert mean < 100 ms.
- Step: run `hyperfine --warmup 3 --runs 10` against `bm import` with a piped 1,000-bookmark payload and assert mean < 100 ms (budget for a single-command invocation at the documented scale).
- Step: run the `bm export | bm import` round-trip and verify correctness (separate from wall-clock; connects to the AC 28 integration test).
- Optional: add a 10,000-bookmark cliff variant documenting the observed wall-clock (not as a budget gate — the spec does not budget at 10K — but as a recorded data point for future PE rounds).

**Classification:** Deferred. The fix is a new `manual-tests/layer-3.md` file — documentation / process artifact, not a code change. Owner is platform-engineer (the Layer 2 Step 12 precedent was owned by the Platform Engineer domain for the shift-left mechanization; Layer 3 extends the same pattern). Validator: performance-engineer re-checks the hyperfine output against the budget table at Layer 3 close.

---

### Dismissed

*(none)*

---

### Resolved

*(none — cold adversarial round does not self-resolve.)*

---

### Raised to SO

*(none — the Finding 1 accepted-limitation DESIGN.md documentation addition is noted as proposed within Finding 1 itself. If the SO adjudicates the addition, that round's entry will land in the SO review log. Finding 7's scope is a new `manual-tests/layer-3.md` file, which is operator/Platform-Engineer territory, not a spec change requiring SO authority.)*

---

### Summary

7 findings assessed against [`src/lib.rs`](../../src/lib.rs) (`export_json`, `import_json`), [`src/main.rs`](../../src/main.rs) (`run_import` stdin-read path), [`manual-tests/`](../../manual-tests/), and [`DESIGN.md`](../../DESIGN.md) § Performance budget:

**1 Accepted limitation** (Finding 1 — `import_json` dedup-via-`Vec::contains` is O(M × N) at the 10K × 10K worst case; implementation is spec-faithful; DESIGN.md should document the trade-off alongside the existing O(N²) cumulative-add-cost accepted limitation).

**5 Hallucinated** (Findings 2–6 — `export_json` sort cliff vs. `bm list` baseline [same sort path]; `serde_json::to_string` + String allocation [mirrors `save`'s accepted `to_string_pretty`]; `take(cap+1)` lazy allocation [single-shot CLI, process-lifetime bound]; 10 MB cap too small for 10K-bookmark round-trip [cap is sized to the ceiling's typical-case estimate + override exists]; `bookmarks_value.clone()` double-parse [necessary for `serde_json::from_value`'s ownership model + spec's atomicity requirement]).

**1 Deferred** (Finding 7 — no `manual-tests/layer-3.md` hyperfine step for `bm export` + `bm import`; real absence; Layer 2 Step 12 precedent makes the gap concrete; deferred to the operator + Platform Engineer for authoring before Layer 3 convergence).

**0 Resolved** (cold adversarial round; no self-resolution). **0 Raised to SO** (Finding 1's DESIGN.md note is a proposed addition within the finding body; no separate SO routing at this time). **0 Dismissed**.

**Regression verdict:** Layer 3 introduces no new scaling cliff on the Layer 1 + Layer 2 baseline for operations at the 1K budget cliff. The `export_json` sort path is isomorphic to `bm list`'s `newest_first` call (both O(N log N), both covered by the Layer 2 hyperfine attestation). The `import_json` dedup loop is O(M × N) — new at Layer 3, documented as accepted limitation — but is not in scope of the 1K budget table (the budget table does not declare a `bm import` latency target). The 10 MB stdin cap is correctly bounded. No previously-accepted performance characteristic has been degraded.

**Round trigger:** Finding 7 is a Deferred real finding; the G-131 continue trigger fires — a PE Round 2 is mandatory after `manual-tests/layer-3.md` is authored (to attest the hyperfine numbers). Finding 1's accepted-limitation classification is terminal for PE (no code fix required); the G-131 trigger is carried by Finding 7 alone.

**Coordination:** Finding 7 routes to Platform Engineer (the shift-left mechanization owner per the Layer 2 Step 12 precedent). Finding 1's DESIGN.md annotation routes to Solution Owner. The PE Round 2 re-reads the `manual-tests/layer-3.md` hyperfine output and verifies the budget assertion.

---

#### Cost-tally (agent-self-verifiable tier)

*This cost-tally covers the agent-self-verifiable fields only per [`suite-development.md`](../../../vsdd-suite/suite-development/suite-development.md) § Cost-tally auditability tiers. Operator-verifiable fields (raw tokens, would-be API cost, rate-limit utilization) require operator `/cost` paste to fill.*

- **AI tool:** claude-code CLI (sub-agent cold session)
- **Execution method:** inline cold-session sub-agent spawned from main session
- **Model:** claude-sonnet-4-6
- **Date:** 2026-05-25 (UTC)
- **Files read (with approximate line counts from Read tool returns):** `PERFORMANCE-ENGINEER-REVIEW.md` (58 lines), `3-review-session.md` (241 lines), `suite-development.md` §§ (350+ lines read across two calls), `rust.md` (107 lines), `DESIGN.md` (327 lines), `2026-05-20-performance-engineer.md` (280+ lines read across two calls), `src/lib.rs` (1038 lines), `tests/bookmarks.rs` (header + grep sweep), `tests/scaling.rs` (236 lines), `src/main.rs` (505 lines), `manual-tests/layer-2.md` (573 lines), `FINDINGS-INDEX.md` (40-line prefix read + grep)
- **Files written:** `vsdd-suite/review-log/2026-05-24-performance-engineer.md` (this file)
- **Tool calls:** 14 Read calls, 6 Bash calls, 1 Write call
- **Wall-clock start:** *pending operator confirmation*
- **Raw tokens:** *pending operator `/cost` paste*
- **Would-be API cost:** *pending operator `/cost` paste*
- **Actual cost to operator:** *pending operator-confirmable plan-tier declaration*
- **Findings/100k tokens:** NOT COMPUTABLE — pending operator `/cost` paste
- **Rate-limit-window utilization:** *pending operator `/cost` paste*

**Operator-action queue:** if cost-tally precision is load-bearing for cross-cycle calibration, operator runs `/cost` in this session and pastes the output here as an append-only addendum, replacing the *pending operator …* placeholders with measured values.

---

## Review 8 — 2026-05-25 04:30Z

**Round:** Layer 3 Phase 3 IAR Round 2.
<!-- hook-bypass: this Round 2 re-verification entry uses **Bold-paragraph emphasis** as inline subsection emphasis within the `### Resolved (Round 1 re-verification)` and `#### Critical re-verification verdicts` sections (fold-up of which Round 1 carry-forwards closed + what targeted re-verification checks ran). These bold lines are paragraph-level emphasis, not Finding headers; actual Round 2 Findings in this entry use the canonical `**Finding N — Title**` form. The check-suite-review-preamble hook's `**X — Y**` regex matches both; the bypass-mechanism is itself a finding for the next registry-walk review. -->

**Scope:** Round 2 — verify Round 1 fixes hold + surface new perf-related residuals. Round 1 fix-work commits `fdfa989` → `ba6a4a9` → `bfc0713` → `795bc25`. Round 2 scope-reducer: capstone-intent adversarial re-review per AI Engineer Dim 8.

**Lens:** Performance Engineer — adversarial posture, Round 2. Re-verification targets as specified in the Round 2 launch prompt: architectural correction perf impact (export_json serde-native path); `bookmark_set_eq` complexity annotation accuracy; control-char tag validation budget; `manual-tests/layer-3.md` Step 15 budget-table consistency; `display_safe` branch-predictor friendliness; `tests/scaling.rs` export/import sentinel gap; Cargo.lock new-deps verification.

**Pre-cycle methodology declaration (per AIE R1 F6 carry-forward):** This round is a re-verification pass, not a cold adversarial sweep. The seven Round 1 findings (1 Accepted-limitation, 5 Hallucinated, 1 Deferred) are the baseline; Round 2 checks each fix route's completion + surfaces any residuals introduced by the fix-work. The critical re-verification targets listed above are the explicit scan agenda; the Hallucinated findings are not re-raised (evidence of their non-existence is already documented).

**Source:** `domain-raised` — Round 2 re-verification pass per Phase 4 routing record (per-domain Phase 4 routing appendices) Round 2 trigger mandate.

**Session note:** Fix-work commits read in sequence (`fdfa989` Phase 1a+1b → `ba6a4a9` Phase 2a → `bfc0713` Phase 2b → `795bc25` Phase 2c). Implementation files read: `src/lib.rs` (full, 1146 lines post-fix); `tests/scaling.rs` (236 lines); `manual-tests/layer-3.md` (573 lines authored at `795bc25`); `DESIGN.md` § Performance budget (including new accepted-limit annotation); Phase 4 routing record (full). Cargo.lock diff across fix-work range verified via `git diff` (null output — no new deps). `cargo test` run to confirm test-suite state: **2 FAILING unit tests discovered** (see Finding 1 below).

---

### Accepted limitation

*(none in Round 2 — the Round 1 accepted-limitation (R1 F1 dedup O(M×N)) was already classified terminal at Round 1; the DESIGN.md annotation landed at `fdfa989` and is verified below under Regression check.)*

---

#### Real findings

<a id="r8-f1"></a>
**Finding 1 — `display_safe` unit tests assert the OLD Rust-syntax escape form `\u{HHHH}` after the JSON-native-escape-design rewrite (implementation/test mismatch; `cargo test` reports 2 FAILING tests)**

**Owner:** software-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** quality-engineer

The Phase 2b fix at `bfc0713` rewrote `display_safe` to emit JSON-native `\uHHHH` escapes (6-char form, no braces) instead of the prior Rust-syntax `\u{HHHH}` form (8-char form, with braces). The implementation change is correct and intentional per the Phase 4 routing JSON-native-escape-design decision. However, the two `display_safe` unit tests in `src/lib.rs` were NOT updated to match the new escape form:

- `tests::display_safe_escapes_ansi_escape` ([`src/lib.rs:1047`](../../src/lib.rs)) asserts `out.contains("\\u{001b}")` — the OLD form. The post-fix implementation produces `` (no braces). **Test FAILS.**
- `tests::display_safe_escapes_format_chars` ([`src/lib.rs:1061`](../../src/lib.rs)) asserts `out.contains("\\u{202e}")` — the OLD form. The post-fix implementation produces `‮` (no braces). **Test FAILS.**

Verified by direct `cargo test display_safe` execution: 2 failures confirmed, error messages show `got [31mred` vs the expected `\u{001b}` form.

**Why this is a real finding, not hallucinated.** The `cargo test` output is authoritative. The tests exist in source at the lines cited. The implementation at `src/lib.rs:800` emits `write!(out, "\\u{cp:04x}")` which formats to `` (no braces) for U+001B. The assertion `out.contains("\\u{001b}")` requires the brace-enclosed form. These are genuinely incompatible.

**Perf implications (why PE is the correct domain to raise this).** This finding crossed into PE scope via the Round 2 re-verification of the architectural correction. The JSON-native escape rewrite was the performance-correctness fix (removing double-escape string allocations); the unit tests that verify the escape output format are the correctness sentinels for that fix. Failing unit tests on the escape path mean the Round 1 regression test for the byte-preservation round-trip (the `ba6a4a9` Phase 2a RED tests) is the ONLY test confirming the correct behaviour at the integration level — the unit-level guard is broken. A future refactor that accidentally reverts to Rust-syntax `\u{...}` form would pass these unit tests but would break the byte-preservation integration tests. The unit-test fix is the right place to enforce the JSON-native form at the smallest granularity.

**Proposed fix (for SE):** Update the two assertions to match the post-fix JSON-native form:
- Line 1047: `out.contains("\\u001b")` (remove the braces from the expected string)
- Line 1061: `out.contains("\\u202e")` (remove the braces from the expected string)

**Classification:** Real finding. Routes to SE (implementation) + QE (test discipline). The fix is a 2-line test correction.

---

### Hallucinated

*(see Round 1 for the 5 Hallucinated classifications; none are re-raised in Round 2)*

---

### Dismissed

*(none)*

---

### Resolved (Round 1 re-verification)

**R1 F1 — dedup-complexity accepted-limit annotation.** `DESIGN.md` § Performance budget now carries the accepted-limit paragraph at commit `fdfa989`. Verified: the paragraph names O(M × N) dedup cost, the 10K × 10K worst-case ~100M comparisons, the accepted-limitation framing, and the `HashSet`-based alternative as the Layer 4 optimization candidate. **R1 F1 closure confirmed.**

**R1 F7 — no `manual-tests/layer-3.md` hyperfine check.** `manual-tests/layer-3.md` was authored at commit `795bc25`. Step 15 is present and covers `bm export` + `bm import` at the 1,000-bookmark cliff with `hyperfine --warmup 3 --runs 10`. **R1 F7 closure confirmed.**

---

#### Critical re-verification verdicts

**Architectural correction perf impact (export_json serde-native path).** `src/lib.rs:454–499` confirms `export_json` serializes via `serde_json::to_string(&store_value)` directly against a struct that borrows `&[&Bookmark]`. No `display_safe` wrapping at the serialization step. The `ExportShape<'a>` local struct borrows the `Vec<&Bookmark>` without additional allocation per bookmark. Net assessment: the Phase 2b architectural correction removes N×field `display_safe` string allocations per export call (each field previously allocated a new `String`; serde_json's encoder writes directly to an internal writer). This is a net perf improvement. **The hidden-cost question:** serde_json's native encoder does branch per character at the string-escape decision point; however, this is the same cost any JSON encoder pays and is not additional overhead introduced by removing `display_safe` — it was always present in serde_json's own encoding. The removal of `display_safe` wrapping REDUCES per-field allocation cost. **Verdict: perf improvement confirmed; no hidden cost regression.**

**`bookmark_set_eq` complexity annotation accuracy.** The accepted-limit annotation in DESIGN.md states "~100M comparisons" at 10K × 10K. With `bookmark_set_eq` now doing per-comparison tag sorts (O(t log t) where t is tag count per bookmark), the actual cost at worst case is O(M × N × t log t). At M=10K, N=10K, t≈3 tags: 10,000 × 10,000 × 3 × log₂(3) ≈ 10^8 × 4.75 ≈ ~475M operations vs the documented ~100M. The DESIGN.md annotation says "~100M comparisons" which was the pre-bookmark_set_eq estimate from Round 1 (based on `Vec::contains` with `PartialEq`). The annotation is now technically understated by ~4-5× due to the sort cost. However: (a) the sort operates on an in-memory `Vec<String>` clone per comparison (cheap for t≈3); (b) "comparisons" in the DESIGN.md context means complete record-pair comparisons, not individual char operations; (c) the accepted-limitation framing is qualitative ("measurably more expensive than per-add O(N) cost") and survives the 4-5× adjustment. **Verdict: the annotation is directionally correct but understated by ~4-5× for the sorted-tag path. This is a residual annotation gap; the correct figure is closer to "~10^8 comparisons × O(t log t) tag-sort per comparison" at the 10K ceiling. Raised as F2 below.**

**Control-char tag validation budget.** `src/lib.rs:583–589`: per-record, per-tag, per-char iteration using `tag.chars().any(|c| c.is_control() || is_format_char(c))`. `is_format_char` is a `const fn` that compiles to a `matches!` arm — effectively a range-check table lookup, branch-predictor-friendly via the same ICF optimisation that range-check tables get. At 10K records × 5 tags × 30 chars = 1.5M char checks. Each check is ~1-2 ns on commodity hardware (branch + range compare). Total: ~1.5-3 ms for the validation pass — well within any import budget. **Verdict: within budget; no finding.**

**`manual-tests/layer-3.md` Step 15 budget-table consistency.** Step 15b uses a 200 ms budget for `bm import` at 1,000 × 1,000 — the loosened budget per the dedup-complexity accepted-limit annotation. DESIGN.md § Performance budget primary table budgets `bm list` + `bm add` at < 100 ms; `bm import` is NOT in the primary table. The 200 ms in Step 15b is a local budget declared in the manual-test step commentary, not a contradiction of the primary table. The primary table's 100 ms budget covers `bm list` + `bm add`; `bm import` is separately addressed by the accepted-limit annotation which acknowledges the O(M×N) cost. **Verdict: no contradiction; the 200 ms is an explicitly-loosened per-operation annotation within Step 15b, not a rollback of the primary table. Consistent.**

**`display_safe` surrogate-pair branch predictor-friendliness.** The new branch at `src/lib.rs:799–807` checks `cp <= 0xFFFF` before selecting the BMP path vs surrogate-pair path. The `is_format_char` and `is_control` filter upstream already excludes all ASCII and most BMP characters from reaching the escape branch; only the curated set of BMP control + format chars reaches it. The surrogate-pair branch fires for codepoints > U+FFFF; the `is_format_char` curated set does include supplementary plane codepoints (`U+E0001`, `U+E0020..=U+E007F`, `U+E0100..=U+E01EF`, `U+13430..=U+13438`, `U+1BCA0..=U+1BCA3`). However, these are extremely rare in URLs/tags in practice. The BMP fast path (`cp <= 0xFFFF`) is the branch-predictor-friendly "hot" path; the surrogate-pair branch is the cold path and will be predicted-not-taken by the CPU's static branch predictor on the first encounter. Branch misprediction cost is ~10-20 cycles; at the actual frequency (a handful of supplementary-plane chars in a URL) this is negligible. **Verdict: branch is predictor-friendly in practice; no perf finding.**

**`tests/scaling.rs` export/import sentinel gap.** Confirmed: `tests/scaling.rs` exercises the `add → list → tag → list-filter` cycle at 100/1K/10K bookmarks. The Layer 3 paths (`bm export` + `bm import`) are NOT exercised by any scaling sentinel. The manual-test hyperfine in Step 15 covers the 1K cliff for export + import, but there is no `#[ignore]`-gated `tests/scaling.rs` sentinel for export/import at any cliff. Raised as F3 below.

**Cargo.lock unchanged.** `git diff fdfa989^..795bc25 -- Cargo.lock` produces empty output — no Cargo.lock changes across the four fix-work commits. No new dependencies introduced. **Verdict: confirmed clean.**

---

<a id="r8-f2"></a>
**Finding 2 — `bookmark_set_eq` O(t log t) sort cost understates the DESIGN.md "~100M comparisons" annotation by ~4-5× at worst case (Accepted limitation annotation gap)**

**Owner:** software-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** performance-engineer

The DESIGN.md § Performance budget accepted-limit annotation (landed at `fdfa989`) reads: *"At the 10K scale ceiling × a 10K-record import, this is ~100M comparisons."* This figure was carried from Round 1 R1 F1's estimate, which was based on the `Vec::contains` O(M×N) model where each "comparison" was a `PartialEq` call. The Phase 2b fix at `bfc0713` replaced `Vec::contains` with a custom `bookmark_set_eq` predicate that sorts both tags Vecs per comparison: two `Vec<String>::sort()` calls + a `PartialEq` on the sorted Vecs. At typical t≈3 tags per bookmark, t log₂ t ≈ 4.75 additional operations per comparison. The worst-case total is closer to ~475M-500M basic operations at 10K × 10K.

**Is the annotation materially wrong?** The accepted-limitation framing is qualitative; the "~100M" figure is a rough order-of-magnitude. The 4-5× understatement is within the same order of magnitude (10^8 vs 5×10^8). A reader relying on the annotation for a quantitative estimate will be surprised; a reader using it for qualitative "is this bounded or not" will reach the correct conclusion. The accepted-limitation rationale (single-user manual-rate tool; 10K ceiling; operator can batch) is unaffected by the 4-5× correction.

**Proposed correction (for SE):** In DESIGN.md § Performance budget, amend the annotation to read: *"...this is ~100M record-pair comparisons at the 10K × 10K worst case; with sorted-tag-comparison dedup each record-pair comparison adds an O(t log t) tag-sort step (t ≈ 3 tags per bookmark in typical stores), pushing the practical worst-case to ~400-500M basic operations at the scale ceiling."*

**Classification:** Accepted limitation annotation gap — the impl is correct; only the annotation's quantitative estimate needs updating. Routes to SE for the DESIGN.md annotation correction. Not a blocker; the qualitative accepted-limitation framing remains valid.

---

<a id="r8-f3"></a>
**Finding 3 — `tests/scaling.rs` has no export/import sentinel at any cliff; the manual-test hyperfine is the only scaling attestation for Layer 3 paths (Dim 9 methodology gap)**

**Owner:** software-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** performance-engineer

`tests/scaling.rs` exercises `add → list → tag → list-filter` at 100/1K/10K. Layer 3 adds `bm export` + `bm import` — two full-store operations with material scaling cost (export: O(N log N) sort + O(N) serialization; import: O(M×N×t log t) dedup). No scaling sentinel exercises these paths at any cliff.

The `manual-tests/layer-3.md` Step 15 hyperfine covers the 1K cliff for wall-clock budget, but: (a) it is a manual step, not an automated correctness gate; (b) it does not cover the 10K ceiling (the step explicitly skips it per "budget gate not required at 10K"); (c) `tests/scaling.rs`'s role per DESIGN.md § Performance budget is correctness-at-scale (round-trip integrity + filter correctness) — the wall-clock is separately attested by hyperfine.

**The gap.** A Layer 3 export/import scaling sentinel would:
- Exercise `export_json` + `import_json` at the 100/1K/10K cliffs via the library API (parallel to the `populate` + `BookmarkStore::load` pattern in the existing tests)
- Assert correctness: a round-trip `export_json → import_json` against a fresh store reproduces the correct bookmark count + the correct bookmark content at each cliff
- Assert the 10K-ceiling dedup correctness: a 10K → import-into-10K-destination produces exactly the known dedup'd count

**Why this is real and not Hallucinated.** Unlike Round 1's Hallucinated findings (which pointed to concerns already covered by prior evidence), this finding points to a genuine structural gap: no test in `tests/scaling.rs` calls `export_json` or `import_json`. The parallel for Layers 1 + 2 (the existing scaling sentinels) exists; the Layer 3 extension does not.

**Why this is a Deferred finding (not a current blocker).** The manual-test hyperfine in Step 15 provides human-attestable wall-clock evidence for the 1K cliff; the integration tests in `tests/bookmarks.rs` cover the functional correctness at 1-10 bookmark scale. The gap is methodology quality, not a missing safety gate. Deferring to the Layer 3 Phase 5 cycle or a follow-up PR where the existing `populate` helper can be extended with `export_json` / `import_json` calls is the proportionate disposition.

**Classification:** Deferred. Routes to PE/SE for authoring a `scaling_1000_export_import_round_trip_correct` (and optionally a 10K counterpart) sentinel in `tests/scaling.rs`. Validator: performance-engineer at the follow-up round.

---

### Summary

**Round 1 regression check:** PARTIAL — Round 1's critical fixes landed correctly (dedup-complexity accepted-limit annotation confirmed in DESIGN.md; `manual-tests/layer-3.md` Step 15 authored with correct hyperfine shape; `bookmark_set_eq` sorted-tag dedup impl verified; Cargo.lock unchanged). However, the Phase 2b `display_safe` JSON-native escape rewrite introduced a **test/implementation mismatch**: 2 unit tests in `src/lib.rs` (`display_safe_escapes_ansi_escape` + `display_safe_escapes_format_chars`) assert the OLD `\u{HHHH}` form and are now FAILING. `cargo test` confirms 2 failures. This is a Round 2 real finding (F1) that must be resolved before Layer 3 layer-gate criteria are met.

**3 findings assessed:**

**1 Real (F1)** — `display_safe` unit tests assert old Rust-syntax escape form after JSON-native-escape-design rewrite; 2 `cargo test` failures confirmed. Routes to SE + QE for a 2-line fix.

**1 Accepted limitation annotation gap (F2)** — `bookmark_set_eq` sorted-tag sort cost understates the DESIGN.md "~100M comparisons" annotation by ~4-5× at worst case. Annotation correction routes to SE; impl is correct; qualitative framing remains valid.

**1 Deferred (F3)** — `tests/scaling.rs` has no export/import sentinel; manual-test hyperfine is the only scaling attestation for Layer 3 paths. Deferred per proportionality; the Layer 3 manual-test coverage is adequate at current maturity.

**0 Hallucinated** (all critical re-verification targets resolved to non-findings or pre-classified).

**Round 2 trigger assessment:** F1 is a concrete failing-test finding; G-131 continue trigger fires. A Round 3 PE re-verification is required after F1 (and optionally F2) is fixed to confirm `cargo test` passes clean. F3's Deferred classification does not trigger Round 3 alone; it is a carry-forward candidate for the Phase 5 cycle.

---

#### Cost-tally (agent-self-verifiable tier)

*This cost-tally covers the agent-self-verifiable fields only per [`suite-development.md`](../../../vsdd-suite/suite-development/suite-development.md) § Cost-tally auditability tiers. Operator-verifiable fields (raw tokens, would-be API cost, rate-limit utilization) require operator `/cost` paste to fill.*

- **AI tool:** claude-code CLI (sub-agent cold session)
- **Execution method:** inline cold-session sub-agent spawned from main session
- **Model:** claude-sonnet-4-6
- **Date:** 2026-05-25 (UTC)
- **Files read (with approximate line counts from Read tool returns):** `2026-05-24-performance-engineer.md` (251 lines — Review 7); per-domain Phase 4 routing appendices (398 lines); `src/lib.rs` (1146 lines); `DESIGN.md` (grepped — perf + dedup sections, ~120 lines extracted); `tests/scaling.rs` (236 lines); `manual-tests/layer-3.md` (573 lines); fix-work commit stats via `git show --stat`; `git diff` on Cargo.lock (null output verified)
- **Files written:** `vsdd-suite/review-log/2026-05-24-performance-engineer.md` (this file — appended Review 8)
- **Tool calls:** 2 Read calls (Review 7 + Phase 4 routing), 1 Read call (lib.rs), 1 Bash (file listing), 1 Bash (git log), 1 Bash (DESIGN.md grep), 1 Read (manual-tests/layer-3.md), 1 Read (scaling.rs), 1 Bash (git log fix-work range), 1 Bash (Cargo.lock diff), 1 Bash (git show stats), 1 Bash (grep display_safe in lib.rs), 1 Read (lib.rs lines 1037-1076 targeted re-read), 1 Bash (cargo test display_safe), 1 Bash (cargo test full suite), 1 Edit (this file)
- **Wall-clock start:** *pending operator confirmation*
- **Raw tokens:** *pending operator `/cost` paste*
- **Would-be API cost:** *pending operator `/cost` paste*
- **Actual cost to operator:** *pending operator-confirmable plan-tier declaration*
- **Findings/100k tokens:** NOT COMPUTABLE — pending operator `/cost` paste
- **Rate-limit-window utilization:** *pending operator `/cost` paste*

**Operator-action queue:** if cost-tally precision is load-bearing for cross-cycle calibration, operator runs `/cost` in this session and pastes the output here as an append-only addendum, replacing the *pending operator …* placeholders with measured values.

---

## Phase 4 routing — Round 1 (2026-05-25 02:00Z)

Per [`vsdd-suite/primers/4-feedback-integration.md`](../../../../vsdd-suite/primers/4-feedback-integration.md) § [manual] First-class fallback path. SO-decisions captured via main-session AskUserQuestion pass on 2026-05-25 across the cross-domain finding clusters. This appendix lists this domain's routable findings in the primer-4-canonical per-finding shape; cross-domain coordination signals live in each Round 1 finding's `**Coordination:**` line. Cross-cluster sequencing matrix lives in the commit message + the CHANGELOG slim-form entry that recorded this Phase 4 pass (refactored from a prior consolidated routing record per operator directive 2026-05-25 — the consolidated file was an anti-pattern; primer-4-canonical is per-domain appendices).

#### Finding `r7-f1` — import_json dedup-via-Vec::contains is O(M × N) at 10K × 10K worst case — ROUTED

**Cluster:** dedup-complexity accepted-limit annotation
**Route:** `Phase 1a+1b (accepted-limitation annotation only)`
**Gate:** DESIGN.md Performance budget Layer 3 dedup-complexity accepted-limit paragraph documented; impl unchanged per spec-faithful framing; Validator: PE
**Sequencing:** Should land before Layer 3 gate close (annotation only)

#### Finding `r7-f7` — No manual-tests/layer-3.md hyperfine sanity-check for bm export + bm import — ROUTED

**Cluster:** manual-tests/layer-3.md authoring
**Route:** `Phase 2a-equivalent artifact authoring`
**Gate:** manual-tests/layer-3.md Step 15 hyperfine block; Validator: PFE
**Sequencing:** Blocks Layer 3 layer-gate close (criterion 3 via manual-tests authoring)
