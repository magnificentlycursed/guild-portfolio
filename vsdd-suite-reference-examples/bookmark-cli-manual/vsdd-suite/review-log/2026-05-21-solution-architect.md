# Solution Architect Review — bookmark-cli-manual

[Index](../SOLUTION-ARCHITECT-REVIEW.md)

---

## Review 2 — 2026-05-22 00:30Z

**Source:** domain-raised — Phase 3 IAR Round 1 cold-session pass against the Layer 2 (tag + filter) implementation; cluster C of the cluster-batched Layer 2 Round 1 (cluster manifest: SA + Red Team + PE, with adversarial pairs Security and VDD-IAR Alignment carved out to clusters B and D per [AI Engineer R1 F1](2026-05-21-ai-engineer.md) cluster-batching discipline).

**Scope:** First SA round against the Layer 2 implementation (4 commits on `bookmark-cli-manual-layer-2`: [`5ba62d5`](https://github.com/magnificentlycursed/guild-portfolio/commit/5ba62d5) Phase 1 → [`326e25d`](https://github.com/magnificentlycursed/guild-portfolio/commit/326e25d) Phase 2a/2b → [`16ee420`](https://github.com/magnificentlycursed/guild-portfolio/commit/16ee420) manual-tests → [`98b5886`](https://github.com/magnificentlycursed/guild-portfolio/commit/98b5886) Phase 2c). Read [`DESIGN.md`](../../DESIGN.md) lines 38–142 + 169–195 + 211–232 (Layer 2 spec extensions including the revised purity-boundary statement); [`src/lib.rs`](../../src/lib.rs) lines 50–82, 90–117, 125–129, 327–397, 399–414, 436–445, 447–460 (Layer 2 pure surface + effectful additions); [`src/main.rs`](../../src/main.rs) lines 58–80, 171–282 (subcommand surface + per-subcommand helpers); [`tests/bookmarks.rs`](../../tests/bookmarks.rs) Layer 2 block lines 504–982; [`TODO.md`](../../TODO.md) § Layer 2 lines 46–93. Regression-check against [SA Review 1 — 2026-05-20 02:45Z](2026-05-20-solution-architect.md#review-1--2026-05-20-0245z) (Phase 5 Purity Boundary Audit for Layer 1).

**Lens:** SA Dim 12 (VSDD purity boundary map — the critical regression-check basis is the Layer 1 SA R1 Phase 5 Purity Boundary Audit — three new pure-side claims to interrogate); SA Dim 2 (Coupling and cohesion — the `Cmd::List` shape change + the `run_*` per-subcommand extraction); SA Dim 3 (Data model integrity — the `tags: Vec<String>` extension); SA Dim 7 (Extensibility — Layer 3 readiness); SA Dim 11 (Session continuity — what gets recorded for future-AI-sessions and future-maintainers); SA Dim 16 (Backward compatibility — storage-format downgrade hazard).

**Session note:** Cold-context session — this reviewer did not author the Layer 2 artifact or any preceding SA round. Sycophancy-compensation: applied the SA R1 Dim 12 (VSDD purity boundary map) check against the Layer 2 pure-side claims as a multi-source cross-check (DESIGN.md § Verification architecture vs. the per-function doc comments vs. the actual `attach_tag` and `filter_by_tags` implementations); each finding is grounded in a file:line citation rather than reasoned about in the abstract. Cluster-batched session per the [primer 3](../../../../vsdd-suite/primers/3-review-session.md) § Session isolation framing — SA + Red Team + PE in one cluster pass with adversarial-pair separation to clusters B and D.

**Reviewer:** solution-architect (cold session, no in-conversation context from Layer 2 authoring).

**Model:** Opus 4.7.

**Cold-session shape:** Solution-Architect/Red-Team/Platform-Engineer cluster (SA + Red Team + PE in one cluster pass per Review 88-era cluster-batching with adversarial-pair separation — Security to cluster B, VDD-IAR Alignment to cluster D).

**Regression-check against:** [SA Review 1 — 2026-05-20 02:45Z](2026-05-20-solution-architect.md#review-1--2026-05-20-0245z) (Phase 5 Purity Boundary Audit for Layer 1 — established DESIGN.md § Verification architecture as the single authoritative source for the purity boundary).

**Cost-tally placeholder:** see Summary.

---

### Resolved

<a id="r2-f1"></a>

**Finding 1 — Layer 2 purity-boundary claims (`filter_by_tags` + `attach_tag`) cohere with implementation; regression-check of SA R1 holds (Dim 12)**

**Owner:** solution-architect
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

[`DESIGN.md`](../../DESIGN.md) § Verification architecture (lines 176–177) extends the Layer 1 purity boundary with two new pure-side claims:

> **Layer 2:** `BookmarkStore::filter_by_tags(&[&str])` — pure OR-filter against the store's bookmarks; returns a `Vec<&Bookmark>` in newest-first order.
> **Layer 2:** `BookmarkStore::attach_tag(url, label)` — pure transformation when given the store, URL, and label; appends `label` to every matching bookmark's `tags` field if not already present. Returns `Result<usize, AttachTagError>` (count of bookmarks affected; error variants for empty-URL / empty-label / no-match).

Cross-source check (per [G-173](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-173) multi-source audit) against [`src/lib.rs:409-414`](../../src/lib.rs) (`filter_by_tags`) and [`src/lib.rs:377-397`](../../src/lib.rs) (`attach_tag`):

- **`filter_by_tags`** — no I/O (only `&self` borrow + `Vec` allocation), no clock read, deterministic output for identical input. The doc comment at [`src/lib.rs:405-407`](../../src/lib.rs) explicitly cites DESIGN.md as authority: "Per `DESIGN.md` § Verification architecture, `filter_by_tags` lives on the pure side of the purity boundary (deterministic; no I/O; no clock)." Implementation is pure. **Coherent with spec claim.**
- **`attach_tag`** — operates on `&mut self` but the mutation is a deterministic function of `(store_state, url, label)`. No I/O, no clock read. The doc comment at [`src/lib.rs:366-369`](../../src/lib.rs) makes the equivalent citation. The "morally pure with `&mut self`" framing matches the SA R1 framing for `add` (DESIGN.md line 181–182's "Boundary refinement" tier acknowledges `add` as morally pure modulo the clock); `attach_tag` does not even reach the clock, so it is on the stricter pure tier.

Adversarial probe — does `&mut self` on `attach_tag` violate purity? No. Purity in the VSDD sense (per [`primers/5-formal-hardening.md`](../../../../vsdd-suite/primers/5-formal-hardening.md) § Purity Boundary Audit definition the SA R1 finding cited) is "deterministic output for identical input; no I/O, no side effects on the world." A `&mut self` method that produces a deterministic new in-memory state from an old in-memory state IS pure under this definition — the relevant test is "could this be formally verified as a function from `(store, url, label) → (store', count_or_error)`?" Yes — there is no clock dependency, no filesystem access, no entropy source. The mutation is local to the parameter. The R1 framing categorized `add` as boundary-refinement only because of the clock; `attach_tag` does not have that defect.

Cross-source consistency post-Layer-2: ✓ — DESIGN.md and the per-function doc comments name the same boundary; the implementation honors the boundary. No regression on SA R1's reconciliation.

**Resolution:** Layer 2 pure-side extension is coherent; SA R1 discipline holds. Validated at first cold-session pass.

**Classification:** Resolved (Dim 12)

---

### Raised to SO

<a id="r2-f2"></a>

**Finding 2 — `attach_tag` + `save` separation forces a load-store-emit boilerplate pattern across all subcommands; the architectural rationale for NOT offering a combined `tag_and_save` helper is undocumented (Dim 12)**

**Owner:** solution-owner
**Status:** raised
**Blocked by:** *(none)*
**Validator:** sanity-check

[`src/main.rs:239-282`](../../src/main.rs) (`run_tag`) shows the explicit two-call pattern: `store.attach_tag(url, label)` followed (on Ok) by `store.save(path)`. The same pattern appears in [`run_add`](../../src/main.rs#L171) (`store.add(url)` then `store.save(path)`). A naive reader might propose collapsing to a `BookmarkStore::tag_and_save(path, url, label)` method that closes the load → mutate → save cycle in one library call.

Architectural rationale for the current separation (which the implementation got right by accident or by design — DESIGN.md does not state this explicitly):

1. **Purity boundary preservation.** `attach_tag` is on the pure side (Finding 1 above); `save` is effectful. A `tag_and_save` method would put a deliberately-impure operation in the library surface, which the SA R1 reconciliation explicitly worked to avoid for the `add`/`save` shape. The library's pure-vs-effectful tier-listing in [`DESIGN.md`](../../DESIGN.md) lines 174–182 stays clean by virtue of the separation.
2. **Composability for Layer 3.** When Layer 3 `bm import` lands (per [`DESIGN.md`](../../DESIGN.md) line 44), it will need to call `attach_tag` zero, one, or many times before a single `save`. A pre-merged `tag_and_save` would force per-call saves, defeating the batch-import discipline. The current separation keeps the library API forward-compatible.
3. **Test surface.** Unit tests for `attach_tag` (none currently exist — see Finding 3 below) would have to set up real filesystem state if the method also saved. Keeping `attach_tag` pure lets the eventual unit tests operate against an in-memory `BookmarkStore` without `tempfile`.

The cost of the current separation: every caller writes the load → mutate → save boilerplate by hand. At three subcommands × five lines × two layers (Layer 1 `add`; Layer 2 `tag`), the duplication is bounded. At Layer 3 with `import` + `export` added, the duplication grows but the "save once after N mutations" shape diverges further — `import` does NOT compose as "per-call mutate-and-save," so the helper would not generalize.

Is this a finding worth raising? Yes, per the SA R1 Dim 12 framing: "Is the boundary documented in DESIGN.md as a verification architecture decision, or only implicit in the code structure?" The boundary IS documented (the purity-tier listing names the per-function status), but the **rationale** for the separation — why the library does NOT offer a combined `tag_and_save` — is implicit in the code only. A future maintainer or AI session reading `main.rs` may propose the helper and would find no DESIGN.md guidance against it.

**Proposed DESIGN.md amendment:** add to [`DESIGN.md`](../../DESIGN.md) § Verification architecture, after the purity-tier listing: "The library API offers `add` / `attach_tag` as pure mutations and `load` / `save` as effectful boundary calls — deliberately separated to keep the pure surface formally-verifiable and to permit Layer 3 batch imports that mutate many times per save. A combined `add_and_save` / `tag_and_save` helper is **explicitly out of scope** — orchestration (load → mutate → save) is the CLI shell's job, not the library's."

**Classification:** Raised to SO (Dim 12)

---

<a id="r2-f5"></a>

**Finding 5 — `tags: Vec<String>` storage extension with `#[serde(default)]` opens a downgrade-corruption hazard not named in DESIGN.md (Dim 16)**

**Owner:** solution-owner
**Status:** raised
**Blocked by:** *(none)*
**Validator:** sanity-check

[`src/lib.rs:50-56`](../../src/lib.rs):

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Bookmark {
    url: String,
    timestamp: DateTime<Utc>,
    #[serde(default)]
    tags: Vec<String>,
}
```

The `#[serde(default)]` makes `tags` optional on deserialization — Layer-1-format files (no `tags` field) deserialize cleanly. This is the spec'd forward-only migration (DESIGN.md lines 165–166). The hazard: the inverse direction (Layer 2 file read by Layer 1 binary) is not contracted as safe. Per DESIGN.md line 166: "the reverse is not guaranteed (a Layer 1 binary reading a Layer 2 file is acceptable because `serde_json` tolerates extra fields by default, but this is not contract)."

The spec correctly names the asymmetry. The architectural concern is downstream: a user who downgrades from a Layer-2 `bm` to a Layer-1 `bm` (e.g., reverts to an older release; uninstalls + reinstalls from a stale cache; runs in a CI environment with a pinned older binary) and tries to `bm list` against a Layer-2 store will get... what? The Layer-1 `serde_json::from_str` will tolerate the extra `tags` field. The bookmarks render. The user is unaware that the downgrade has discarded the tags-aware code path. If they then `bm add` on the Layer-1 binary, the next `save()` will write the file WITHOUT the `tags` field — silently destroying tag data. This is a real downgrade-corruption hazard. The spec acknowledges it ("not contract") but does not name it as a hazard or recommend a mitigation.

Adjacent concern at the `Cmd::List` shape change: in Layer 1, `Cmd::List` was a unit variant. The Layer 2 version is now a struct variant `Cmd::List { tags: Vec<String> }`. At the CLI surface, `bm list` (no `--tag` flag) is still backward-compatible because clap's `ArgAction::Append` defaults to an empty `Vec<String>`. ✓ The internal `Cli` type ([`src/main.rs:53-56`](../../src/main.rs)) is private to `main.rs` — no library consumer sees `Cmd::List` directly. The architectural note is that **the deliberate choice was made to introduce a field rather than a separate `Cmd::ListFiltered { tags }` variant** — and that choice is correct (the no-arg list and the filtered list are the same UX operation, not separate operations). No recommended change for the `Cmd::List` sub-concern.

**Proposed DESIGN.md amendment:** Add to [`DESIGN.md`](../../DESIGN.md) § Storage format § `tags` field (Layer 2): "**Downgrade hazard.** A Layer 1 binary reading a Layer 2 file will silently discard the `tags` data on the next `save()` because the Layer 1 `Bookmark` struct does not have a `tags` field to round-trip. Users on Layer 2 should not downgrade to Layer 1 against the same store file; if downgrade is necessary, the user should back up the store file first. A future Layer may add a `format_version` field to the top-level `BookmarkStore` to make version-detection explicit; at Layer 2 the version is implicit in the field presence."

**Classification:** Raised to SO (Dim 16)

---

### Deferred

<a id="r2-f3"></a>

**Finding 3 — `attach_tag` and `filter_by_tags` have NO unit tests at the library level; the only coverage is the integration tests in `tests/bookmarks.rs` (Dim 12)**

**Owner:** quality-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** quality-engineer

[`src/lib.rs:579-814`](../../src/lib.rs) (`#[cfg(test)] mod tests`) — the library-level test module — contains tests for `newest_first`, `load`, `save` (roundtrip + parent-directory + mode 0600 + symlink + orphan), `add`, and `display_safe` (newline/tab preservation, ANSI escape, format chars). It does NOT contain any test for `attach_tag` or `filter_by_tags` — the new pure-side library functions. The only coverage is in [`tests/bookmarks.rs`](../../tests/bookmarks.rs) Layer 2 block (lines 504–982), which exercises the functions via the compiled `bm` binary.

Why this is an SA finding (not just a QE finding): the SA R1 reconciliation made DESIGN.md the authoritative purity-boundary source and named "Unit tests for the pure functions ... in `src/lib.rs`'s `#[cfg(test)] mod tests` block" as the verification surface (DESIGN.md line 186). The Layer 2 spec extends the pure surface with `attach_tag` + `filter_by_tags`; if the verification-architecture commitment "unit tests for the pure functions" is to be honored, both new pure functions need lib-level unit tests. The integration tests in `tests/bookmarks.rs` are necessary but not sufficient — they exercise the binary, not the library API. A future maintainer using `bookmark_cli::BookmarkStore::attach_tag` as a library dependency (Layer 3 `import` will do this) gets no unit-test signal.

The architectural impact is also concrete:

- **No coverage for `AttachTagError::EmptyUrl` / `EmptyLabel` / `NoMatch` at the library boundary.** The integration tests exercise the CLI shell's mapping of these errors to stderr messages, but the library invariant (the enum variants themselves; the precondition order; that the store is NOT mutated on the error path) is not directly tested.
- **No coverage for the idempotence invariant at the library boundary.** `tests::tests_tag_is_idempotent` in `tests/bookmarks.rs` (lines 553–584) exercises it via two CLI invocations; a library-level test could assert it via a single in-memory `store.attach_tag(url, label)` × 2 sequence without filesystem round-tripping.
- **No coverage for `filter_by_tags` OR-semantics, empty-label-list edge case, or newest-first ordering at the library boundary.** The integration tests cover OR-semantics (`tests_list_with_tag_filter_repeated_flag_is_or_semantics` lines 870–938) but again only through the CLI.

Proposed unit tests (suggested for QE to land): `attach_tag_appends_label_to_matching_bookmark` (single match, idempotent on second call); `attach_tag_returns_no_match_when_url_absent_without_mutating_store`; `attach_tag_returns_empty_url_for_empty_string`; `attach_tag_returns_empty_label_for_empty_string`; `attach_tag_tags_all_duplicate_url_bookmarks`; `filter_by_tags_returns_or_union_in_newest_first_order`; `filter_by_tags_empty_labels_returns_empty` (currently this slips through — `labels: &[]` would return empty without explicit assertion).

Adjacent concern surfaced during this finding: `filter_by_tags(&[])` (empty labels slice) returns an empty `Vec<&Bookmark>` — every `b.tags.iter().any(|t| labels.iter().any(...))` evaluates to false when `labels` is empty. Is this the intended semantic? At the CLI shell, the empty-labels case is intercepted upstream by `run_list`'s `if tags.is_empty()` branch (line 214) — the bare `bm list` path. So the library function's behavior on `&[]` is reachable only via direct library use (Layer 3 or external consumers). The contract is "filter to bookmarks whose tags contain at least one of the labels" — under set-theory the empty-labels filter is the empty set, which matches the implementation. But this is implicit; a unit test would make it explicit and prevent future maintainers from "fixing" it to return all bookmarks.

**Classification:** Deferred — Coordination-to-QE; the Layer 2 Phase 5 Mutation Testing re-run (per [`DESIGN.md`](../../DESIGN.md) line 15) will likely surface the same gap from a mutation-survival angle. Trigger to close: 5–7 library-level unit tests added to `src/lib.rs::tests` covering `attach_tag` + `filter_by_tags`. (Dim 12)

---

<a id="r2-f4"></a>

**Finding 4 — `filter_by_tags` is O(n log n) regardless of filter selectivity because it sorts first then filters; the architectural choice is undocumented and may surprise a Layer 3 caller (Dim 9)**

**Owner:** software-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** performance-engineer

[`src/lib.rs:409-414`](../../src/lib.rs):

```rust
pub fn filter_by_tags<'a>(&'a self, labels: &[&str]) -> Vec<&'a Bookmark> {
    self.newest_first()
        .into_iter()
        .filter(|b| b.tags.iter().any(|t| labels.iter().any(|l| t == *l)))
        .collect()
}
```

The implementation calls `newest_first` (which sorts ALL bookmarks O(n log n)) then filters. The alternative — filter first, then sort the smaller result — is O(n + k log k) where k is the selectivity (matches). For a 10,000-bookmark store with a high-selectivity filter (10 matches), filter-then-sort is ~1000× cheaper.

At the [`DESIGN.md`](../../DESIGN.md) § Performance budget scale (10,000 bookmark ceiling, 100 ms wall-clock budget on `bm list` operations), both approaches likely fit. The hyperfine sanity-check at [`manual-tests/layer-2.md`](../../manual-tests/layer-2.md) Step 12 will determine whether the O(n log n) sort-then-filter actually exceeds budget at the 1000-bookmark cliff, but the architectural concern is independent of whether the current shape fits the budget:

The choice is undocumented. A future maintainer (or Layer 3 consumer) reading `filter_by_tags` sees the impl and may assume "this was deliberate — perhaps `newest_first` is cached internally and the filter-then-sort path would be slower." That assumption is wrong (`newest_first` allocates a fresh `Vec<&Bookmark>` and sorts it on every call), but the spec gives no guidance.

Three resolution paths:

(a) **Document the choice.** Add to `filter_by_tags` doc comment (and DESIGN.md § Verification architecture pure-function annotation): "Implementation is sort-then-filter rather than filter-then-sort for simplicity at the 10,000-bookmark scale ceiling; the cost is O(n log n) per call, dominated by `newest_first`'s sort. A filter-then-sort variant would be O(n + k log k) for k matches and is the right shape if the scale ceiling is raised."

(b) **Change to filter-then-sort.** One-line edit: collect matches first, then sort by timestamp descending. The Layer 2 spec is silent on the operation order, and the observable behavior (returned `Vec<&Bookmark>` in newest-first order) is identical. The unit tests proposed in Finding 3 would catch any regression.

(c) **Both — change the impl AND document.** The complexity reduction is essentially free at Layer 2 (no API change; the unit tests proposed in Finding 3 cover the regression surface); the documentation is the durable record of the choice.

SA recommendation: (c). The current shape has no defenders — it is the natural composition of the two existing primitives (`newest_first` + `Iterator::filter`), which is what a Phase 2b implementer would write. The improvement is straightforward; the documentation closes the "future maintainer wonders why" surface. **However**, the cost-asymmetry at the project's actual scale is small enough that (a) alone is acceptable if the operator prefers minimum-change disposition.

**Classification:** Deferred — Coordination to [Performance Engineer](../PERFORMANCE-ENGINEER-REVIEW.md) for the benchmark side (does the current shape actually exceed budget at 1000?), and routing to [SE](../SOFTWARE-ENGINEER-REVIEW.md) for the implementation side if path (b) or (c) is chosen. Per the SA R1 framing, this is a Dim 9 (complexity vs. spec) finding — not over-engineered (the current shape is the minimum implementation), but the choice is undocumented. (Dim 9)

---

### Hallucinated

<a id="r2-f6"></a>

**Finding 6 — `Bookmark` struct accretion will become unwieldy by Layer 3 if `export`/`import` add more fields; the refactor to `BookmarkMetadata` should land now (Dim 7)**

The supplement-derived adversarial enumeration suggests that growing structs are a code smell warranting preemptive refactor (extract metadata; or use `serde_json::Value` for forward-compatibility). Applied to `Bookmark`:

```rust
pub struct Bookmark {
    url: String,
    timestamp: DateTime<Utc>,
    #[serde(default)]
    tags: Vec<String>,
}
```

The struct grew from 2 fields (Layer 1) to 3 fields (Layer 2). At Layer 3 (`export`/`import` per DESIGN.md line 44), the field count is unchanged — `export` and `import` operate on existing `Bookmark` records; they don't add fields. The DESIGN.md scope for Layer 3 lists "emit bookmarks as JSON to stdout" and "read bookmarks from stdin and merges them into the store" — both are operations on the existing schema.

Verified absent. A future hypothetical layer that adds (e.g.) a `description: Option<String>` field would grow the struct to 4 fields, and at 4 fields the YAGNI principle would still favor leaving the struct shape rather than extracting a metadata sub-struct. The portfolio precedent ([`issue-tracker-cli`](../../../../issue-tracker-cli/)) has multi-field record structs without sub-struct extraction. The hypothetical concern is the adversarial enumeration's invention; the project's actual trajectory does not motivate the refactor.

**Classification:** Hallucinated. The struct accretion concern is supplement-enumerated; the project's actual Layer 3 scope does not exercise it; the YAGNI baseline holds. Recorded per the sycophancy-check discipline.

---

### Summary

5 findings + 1 Hallucinated filed against Layer 2 in this first SA cold-session pass: **1 Resolved** (Finding 1 — Layer 2 purity-boundary claims cohere with implementation; SA R1 discipline holds); **2 Raised to SO** (Finding 2 — `attach_tag`/`save` separation rationale should be documented in DESIGN.md; Finding 5 — `tags` field `#[serde(default)]` creates a downgrade-corruption hazard not named in DESIGN.md); **2 Deferred** (Finding 3 — `attach_tag` + `filter_by_tags` lack library-level unit tests, Coordination-to-QE; Finding 4 — `filter_by_tags` sort-then-filter complexity choice is undocumented, Coordination-to-PerformanceEngineer + SE); **1 Hallucinated** (Finding 6 — `Bookmark` struct accretion concern does not apply at Layer 3's actual scope).

The pattern is consistent with a Layer 2 cold pass against an implementation that landed the spec contract cleanly: zero structural defects in the purity boundary or the type-shape choices; the live findings are all spec-documentation gaps (Findings 2, 4, 5 each propose a DESIGN.md amendment to make implicit choices explicit) or test-surface gaps (Finding 3). The cold-session signal is that Layer 2 is at MVR-blocked-by-spec-documentation for SA — the implementation is sound; the spec is silent on three architectural decisions that future maintainers (or Layer 3) will run into.

**Coordination:**

- **Finding 1** (Resolved purity-boundary regression-check) — no coordination needed; documented for the audit trail.
- **Finding 2** (`attach_tag` + `save` separation rationale) — routes to [Solution Owner](../SOLUTION-OWNER-REVIEW.md) for the spec-amendment ratification.
- **Finding 3** (no library-level tests for `attach_tag` + `filter_by_tags`) — routes to [Quality Engineer](../QUALITY-ENGINEER-REVIEW.md) for the unit-test implementation; also flagged to the Layer 2 Phase 5 Mutation Testing re-run (per DESIGN.md line 15) since the testing-gap surface is exactly what Mutation Testing surfaces from the opposite direction.
- **Finding 4** (`filter_by_tags` sort-then-filter is undocumented) — routes to [Performance Engineer](../PERFORMANCE-ENGINEER-REVIEW.md) for the benchmark side (the [`manual-tests/layer-2.md`](../../manual-tests/layer-2.md) Step 12 hyperfine run will close the "does this actually exceed budget" question); routes to [SE](../SOFTWARE-ENGINEER-REVIEW.md) if the implementation change (path b or c) is chosen.
- **Finding 5** (storage `#[serde(default)]` downgrade hazard) — routes to [Solution Owner](../SOLUTION-OWNER-REVIEW.md) for the spec-amendment ratification.
- **Finding 6** (Hallucinated) — no coordination; recorded for audit completeness.

**Cost-tally:** Solution-Architect/Red-Team/Platform-Engineer cluster session (SA + Red Team + PE in one cluster pass) — SA sub-section consumed an estimated ~25k–30k tokens for the cold context-load + per-finding evidence-gathering (the SA R1 Layer 1 review, the DESIGN.md Layer 2 sections, the `lib.rs`/`main.rs` Layer 2 deltas, the tests/bookmarks.rs Layer 2 block, and the FINDINGS-INDEX baseline). Per-finding cost ≈ 5k–6k tokens; below the capstone band's 100k–300k/finding range, consistent with the cluster-batching efficiency [AI Engineer R1 F6+F7+F8](2026-05-21-ai-engineer.md) observed on the prior cycle.
