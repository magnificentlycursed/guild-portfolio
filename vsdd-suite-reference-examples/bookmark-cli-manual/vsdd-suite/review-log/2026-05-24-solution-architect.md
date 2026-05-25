# Solution Architect Review — bookmark-cli-manual

[Index](../SOLUTION-ARCHITECT-REVIEW.md)

---

## Review 1 — 2026-05-25 01:12Z

**Scope:** Phase 3 IAR Round 1 cold-session pass against the Layer 3 (`bm export` + `bm import`) implementation. Three commits in scope: [`878d3b6`](https://github.com/magnificentlycursed/guild-portfolio/commit/878d3b6) (Phase 2a Red Gate — 15 failing tests AC 14..AC 28), [`fd21900`](https://github.com/magnificentlycursed/guild-portfolio/commit/fd21900) (Phase 2b implementation — `export_json` + `import_json` + CLI wiring), [`78bd3cf`](https://github.com/magnificentlycursed/guild-portfolio/commit/78bd3cf) (Phase 2c extract-and-name). Read [`DESIGN.md`](../../DESIGN.md) lines 43–47 + 103–168 + 225–252 (Layer 3 spec extensions including the Phase 5 strategy line for Layer 3); [`src/lib.rs`](../../src/lib.rs) lines 424–567 + 569–610 (export_json + import_json + MAX_STDIN_BYTES_DEFAULT + ImportError); [`src/main.rs`](../../src/main.rs) lines 125–169 (Cmd::Export + Cmd::Import) + 393–488 (run_export + run_import); [`tests/bookmarks.rs`](../../tests/bookmarks.rs) lines 1065–1688 (Layer 3 Red Gate tests AC 14..AC 28). Regression-check against [SA Review 1 — 2026-05-20 02:45Z](2026-05-20-solution-architect.md#review-1--2026-05-20-0245z) (Layer 1 Phase 5 Purity Boundary Audit baseline — DESIGN.md § Verification architecture as authoritative purity-boundary source), [SA Review 2 — 2026-05-22 00:30Z](2026-05-21-solution-architect.md#review-2--2026-05-22-0030z) (Layer 2 Round 1 — F1 Resolved purity-boundary regression-check; F2 + F5 Raised-to-SO), and [SA Review 4 — 2026-05-22 22:00Z](2026-05-22-solution-architect.md#review-4--2026-05-22-2200z) (Layer 2 Phase 5 Purity Boundary Audit re-run).

**Session note:** Cold-context session — this reviewer did not author the Layer 3 spec, Red Gate, or implementation. Sycophancy-compensation: the SA R1 Dim 12 (VSDD purity boundary map) check applied as a multi-source cross-check (DESIGN.md § Verification architecture vs. DESIGN.md § Project intent's Phase 5 strategy-for-Layer-3 prose vs. the per-function doc comments vs. the actual `export_json` + `import_json` implementations vs. the test expectations). Each finding is grounded in file:line citations and verbatim code excerpts. Adversarial probe specifically targets the user-prompt-named lenses: purity boundary placement, architectural drift from DESIGN.md § Verification architecture, module organization, `ImportError` variant design, export+import coupling, dedup-algorithm complexity, fail-closed semantics, layer-3-vs-layer-2 architectural coherence. Inline-run (not cluster-batched) per operator directive; trade-off declared — cluster-batched cold session would be the gold standard for adversarial pressure, but at SA-only scope against a bounded ~150-LOC Layer 3 surface the inline run is proportionate.

**Source:** domain-raised — the SA dimensions (1–12) applied against the Layer 3 implementation surfaced these findings; no operator interruption mid-round; no regression-replay against prior reproducers.

**Reviewer:** solution-architect (cold session, no in-conversation context from Layer 3 spec/Red-Gate/implementation authoring).

**Model:** Opus 4.7.

**Supplements applied:** [`rust.md`](../../../../vsdd-suite/supplements/rust.md) § Solution Architect — applies because Layer 3 extends the Rust `BookmarkStore` library surface with two new methods (`export_json` + `import_json`) and adds new error type (`ImportError`) + module-level constant (`MAX_STDIN_BYTES_DEFAULT`); the supplement's CLI-parsing-separated-from-business-logic + Command-enum-dispatch + Error-type-hierarchy + `lib.rs`/`main.rs`-split dimensions are directly load-bearing on this surface.

**Cold-session shape:** N/A — inline-run from the main session. Trade-off declared per the bounded-judgment-surface rubric ([`primers/5-formal-hardening.md`](../../../../vsdd-suite/primers/5-formal-hardening.md) § Cold-session-vs-inline decision rubric) is the analogous discipline for Phase 5; for Phase 3 IAR the equivalent shape-declaration is the cluster-batching-vs-inline trade-off per [`primers/3-review-session.md`](../../../../vsdd-suite/primers/3-review-session.md) § Session isolation. The SA-only scope against a bounded ~150-LOC surface is the inline-acceptable case; the post-Round-1 G-131-continue-trigger discipline still applies, so Round 2 (if triggered) may move to cluster-batched cold-session for adversarial pressure on whatever new findings Round 2 surfaces.

**Regression-check against:** [SA Review 1 — 2026-05-20 02:45Z](2026-05-20-solution-architect.md#review-1--2026-05-20-0245z) (Layer 1 Phase 5 Purity Boundary Audit — DESIGN.md § Verification architecture established as the single authoritative source for the purity boundary; module-doc cites DESIGN.md, not vice-versa); [SA Review 2 — 2026-05-22 00:30Z](2026-05-21-solution-architect.md#review-2--2026-05-22-0030z) (Layer 2 R1 — pure-side claims for `filter_by_tags` + `attach_tag` coherent with implementation; F2 attach_tag/save-separation rationale Raised-to-SO + carry-forward); [SA Review 4 — 2026-05-22 22:00Z](2026-05-22-solution-architect.md#review-4--2026-05-22-2200z) (Layer 2 Phase 5 Purity Boundary Audit re-run — all five Layer 2 purity-boundary declarations verify against implementation).

---

### Resolved

<a id="r1-f1"></a>

**Finding 1 — `export_json` is genuinely pure; `import_json` is pure-transformation on `&mut self` — both qualify under the SA R1 / R4 purity definition (Dim 12)**

**Owner:** solution-architect
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

[`src/lib.rs:454-485`](../../src/lib.rs) (`export_json`) and [`src/lib.rs:517-566`](../../src/lib.rs) (`import_json`) — adversarial probe against the purity claim each function makes in its doc comment:

**`export_json(&self, filter_labels: Option<&[&str]>) -> String`** — doc comment lines 442–444 claim "Pure transformation against the store — no I/O, no clock, no mutation. Per `DESIGN.md` § Verification architecture, `export_json` lives on the pure side of the purity boundary." Verification:

- **No I/O:** ✓ — no `std::fs::*`, no `std::process::*`, no `eprintln!`/`println!`/`print!`, no `std::io::*`. The function only allocates strings via `serde_json::json!` and `to_string`.
- **No clock:** ✓ — no `Utc::now()`, no `SystemTime::now()`, no `Instant::now()`.
- **No global state:** ✓ — operates entirely on `&self` and the supplied `filter_labels` slice.
- **Deterministic:** ✓ — given the same `&self` and `filter_labels`, the returned `String` is byte-identical (`newest_first()` sorts deterministically; `filter_by_tags` is deterministic; `display_safe` is deterministic; `serde_json::to_string` on a `Value` is deterministic given the same key order, which the `json!` macro fixes).
- **No mutation:** ✓ — takes `&self`, not `&mut self`.

**`import_json(&mut self, payload: &str) -> Result<usize, ImportError>`** — doc comment lines 487–509 do not make an explicit purity claim, but DESIGN.md § Project intent's Phase 5 strategy line for Layer 3 (line 15) declares: "Purity Boundary Audit re-runs against the extended pure surface (export-serialize + import-deserialize + dedup-on-exact-tuple-match — all pure functions of the input JSON + existing store state)." Verification:

- **No I/O:** ✓ — no `std::fs::*`. Receives the payload string as a parameter; the CLI shell handles stdin reading.
- **No clock:** ✓ — no `Utc::now()` (the import preserves timestamps from the payload).
- **No global state:** ✓ — operates entirely on `&mut self` and the supplied `payload`.
- **Deterministic:** ✓ — given the same `&mut self` initial state and `payload`, the post-state of `self` is byte-identical and the returned count is identical.
- **Pure transformation on mutable receiver:** the function mutates `self.bookmarks` but does so as a deterministic function of `(initial_store, payload)`. This is the same "morally pure with `&mut self`" framing that SA R2 F1 applied to `attach_tag` and SA R4 F1 reconfirmed; `import_json` is purer than `add` (no clock dependency) and equivalent to `attach_tag`'s purity tier.

The dedup logic uses `self.bookmarks.contains(&new_bm)` (line 560) which relies on `Bookmark`'s derived `PartialEq` — comparing all three fields (url + timestamp + tags). No I/O, no entropy, no clock. The atomicity discipline (all validation before any mutation; lines 525–550 validate; lines 558–564 mutate) preserves the spec's "partial imports MUST NOT occur" contract at the library boundary.

**Adversarial probe — does the `&mut self` on `import_json` violate purity?** No, by the same definition SA R2 F1 + SA R4 F1 applied to `attach_tag`: VSDD purity (per [`primers/5-formal-hardening.md`](../../../../vsdd-suite/primers/5-formal-hardening.md) § Purity Boundary Audit) is "deterministic output for identical input; no I/O, no side effects on the world." A `&mut self` method producing a deterministic new in-memory state from an old in-memory state IS pure under this definition.

**Adversarial probe — does the pre-mutation validation discipline hold structurally?** Yes. The validation chain at lines 525–550 returns early on any failure (`?` operator + `if … return Err`), and the mutation loop at lines 558–564 runs only after all validation passed. A future maintainer reordering the loops would break the atomicity guarantee, but the current shape is structurally correct.

**Verdict:** Both Layer 3 functions are pure (under the project's established definition). The Layer 1 SA R1 + Layer 2 SA R2 F1 + Layer 2 SA R4 F1 purity-boundary discipline holds against the Layer 3 surface. No I/O sneak; no clock dependency; deterministic; atomic mutation.

**Resolution:** Layer 3 pure-side extension is coherent; the SA R1/R4 discipline holds. Validated at first cold-session pass.

---

### Raised to SO

<a id="r1-f2"></a>

**Finding 2 — DESIGN.md § Verification architecture's pure-side enumeration does NOT name `export_json` or `import_json`; the only authoritative pure-list-of-record stops at Layer 2 (`filter_by_tags` + `attach_tag`). The Layer 3 purity claims live only in (a) Phase-5-strategy prose at DESIGN.md:15 and (b) per-function doc comments at `src/lib.rs:442-444`. This is the same cross-source divergence pattern SA R1 F1 closed for Layer 1 — re-opening at Layer 3 (Dim 12)**

**Owner:** solution-owner
**Status:** raised
**Blocked by:** *(none)*
**Validator:** sanity-check

[`DESIGN.md:225-241`](../../DESIGN.md) § Verification architecture is the authoritative purity-boundary statement (per the SA R1 reconciliation: "This is the authoritative purity boundary for the project. The module doc at `src/lib.rs:1-?` cites this section as the single source"). The pure-function enumeration ends at the Layer 2 additions (`filter_by_tags` + `attach_tag`); there is no Layer 3 entry. The full pure-side list per the canonical section:

> **Pure functions** (deterministic, no I/O, formally verifiable in principle):
>   - `Bookmark` and `BookmarkStore` data types (serde derivations are pure functions of input).
>   - `BookmarkStore::newest_first` (pure sort by reference; no I/O, no clock).
>   - **Layer 2:** `BookmarkStore::filter_by_tags(&[&str])` — pure OR-filter against the store's bookmarks; returns a `Vec<&Bookmark>` in newest-first order.
>   - **Layer 2:** `BookmarkStore::attach_tag(url, label)` — pure transformation when given the store, URL, and label; appends `label` to every matching bookmark's `tags` field if not already present. …

No `BookmarkStore::export_json` entry. No `BookmarkStore::import_json` entry.

Meanwhile, the per-function doc comments at `src/lib.rs` make explicit purity claims citing this section as authority:

- [`src/lib.rs:442-444`](../../src/lib.rs) (`export_json`): "Pure transformation against the store — no I/O, no clock, no mutation. Per `DESIGN.md` § Verification architecture, `export_json` lives on the pure side of the purity boundary."
- [`src/lib.rs:487-509`](../../src/lib.rs) (`import_json`): the doc comment names the dedup discipline + the atomicity invariant but does not make an explicit purity claim; the purity-via-`Phase-5-strategy-prose-at-DESIGN.md:15` claim only.

This is the **exact cross-source divergence pattern** that SA R1 F1 closed for Layer 1: the module doc / function doc claimed pure-side membership while DESIGN.md § Verification architecture was silent on the per-function status. SA R1 F1's resolution was: "module doc cites DESIGN.md as the single authoritative source; DESIGN.md is rewritten to enumerate each function's status explicitly." That same pattern needs to apply at Layer 3.

**Behavior consequence:** A future Phase 5 Purity Boundary Audit re-run for Layer 3 (per DESIGN.md:15's "Purity Boundary Audit re-runs against the extended pure surface") will hit the G-173 multi-source check (a) implementation against the DESIGN.md claim, (b) implementation against the module-doc claim, (c) DESIGN.md claim against the module-doc claim — and the (c) check will fail because DESIGN.md § Verification architecture does not name `export_json` / `import_json`. The result is the same finding-class the SA R1 F1 audit caught — the audit will be unable to verify the pure-side declarations against the section that's supposed to own them.

**Proposed DESIGN.md amendment:** add to [`DESIGN.md`](../../DESIGN.md) § Verification architecture, under the "Pure functions" list, after the Layer 2 entries:

> - **Layer 3:** `BookmarkStore::export_json(&self, filter_labels: Option<&[&str]>) -> String` — pure serialization of the store (or its filtered subset) to the storage-format object-wrapped JSON shape; `display_safe` wraps URL + tag-label strings at the per-field serialization boundary; no I/O, no clock, no mutation; deterministic output for identical `(self, filter_labels)`.
> - **Layer 3:** `BookmarkStore::import_json(&mut self, payload: &str) -> Result<usize, ImportError>` — pure transformation when given the store and the JSON payload; deserialize + per-record schema validation + dedup-on-exact-tuple-match (against existing destination state AND within the imported payload) + append; no I/O, no clock; atomicity invariant (all validation runs before any mutation; partial imports MUST NOT occur). Mutates `&mut self` (same "morally pure with `&mut self`" tier as `attach_tag`).

This amendment closes the cross-source divergence and establishes the single-authoritative-source discipline for the Layer 3 pure-side surface, matching the discipline already established for Layers 1 and 2.

**Classification:** Raised to SO (Dim 12)

---

<a id="r1-f3"></a>

**Finding 3 — `display_safe`-at-export breaks the round-trip-byte-preservation invariant the spec explicitly asserts at DESIGN.md:106 (Dim 12 + Dim 16 — interface contract on the round-trip surface)**

**Owner:** solution-owner
**Status:** raised
**Blocked by:** *(none)*
**Validator:** sanity-check

[`DESIGN.md:106`](../../DESIGN.md) § Behavioral contracts § `bm export` (Layer 3) makes a load-bearing claim:

> "**Phase 2b implementation verification:** the implementation must confirm `display_safe`-wrapped strings remain JSON-valid (terminal escape sequences serialize as JSON-valid `` sequences so the round-trip `bm export | bm import` preserves the underlying bytes)."

The spec claims the round-trip preserves the **underlying bytes** — i.e., a bookmark whose `url` field contains the single ESC byte `\x1b` should round-trip through `bm export | bm import` and land in the destination store with the same single ESC byte in its `url` field. The spec assumes `display_safe` produces JSON-native escape sequences (``) that `serde_json` will decode back to the original byte on parse.

The actual `display_safe` implementation at [`src/lib.rs:686-705`](../../src/lib.rs) does NOT produce JSON-native escapes. It produces Rust-style literal `\u{HHHH}` strings:

```rust
let _ = write!(out, "\\u{{{:04x}}}", c as u32);
```

For input `\x1b`, the output is the 8-character literal string `\u{001b}` (backslash, u, open-brace, 0, 0, 1, b, close-brace) — NOT a 6-character JSON escape ``. When `serde_json::Value::String` wraps this 8-char string and serializes, the resulting JSON on the wire is `"\\u{001b}"` (the backslash gets JSON-escaped to `\\`). When `bm import` parses that JSON, the in-memory string is back to the 8-character literal `\u{001b}` — NOT the original 1-byte `\x1b`.

**The round-trip does NOT preserve the underlying bytes.** A bookmark whose `url` contained `https://example.com/\x1b[31mred` becomes, after `bm export | bm import`, a bookmark whose `url` is the literal 30-character string `https://example.com/\u{001b}[31mred`.

The empirical evidence is in the existing test suite. [`tests/bookmarks.rs:1236-1284`](../../tests/bookmarks.rs) (AC 18 `tests_export_applies_display_safe_to_pathological_url`) asserts only:

1. The emitted bytes don't contain raw ESC (true — `display_safe` strips it).
2. The output is valid JSON (true).
3. The URL field "does not carry the raw ESC byte; got `{url:?}`" (true — but the URL field carries the literal escape representation instead, which the test allows: "The exact escape representation is an implementation choice — the contract is 'no raw control chars' + 'JSON-parseable' + 'round-trippable through bm import'").

The test does NOT verify the spec's stronger claim that the round-trip preserves the underlying bytes. The round-trip test AC 28 [`tests/bookmarks.rs:1601-1689`](../../tests/bookmarks.rs) (`tests_export_import_round_trip`) uses well-formed URLs only (`https://item-{i}.example`) — pathological URLs are not exercised in the round-trip path.

**Why this is a Layer 3 SA finding (not a SE bug):** the spec's design assertion at DESIGN.md:106 is architecturally inconsistent with `display_safe`'s actual representation choice (Rust literal vs. JSON-native). The implementation matches what `display_safe` produces; the spec is the source of the inconsistency. Two structurally-distinct resolutions exist, and the choice is a Solution Owner decision:

- **(a) Drop the round-trip-byte-preservation claim from the spec.** Accept that `display_safe` at the export boundary is a one-way sanitizer — the round-trip preserves the *sanitized* form, not the original bytes. The user who pipes `bm export | bm import` against a store with control characters in URLs gets a destination store where those control characters have been replaced by their `\u{HHHH}` literal representation. This is operator-detectable (the URL grew longer; `bm list` shows the escape) and is consistent with the spec's broader "store the bytes as-given; defer rendering safety to `display_safe` at output time" framing (DESIGN.md:130). The wire format is escape-clean; the round-trip is escape-clean; the *original bytes* are not preserved.

- **(b) Move `display_safe` from the serialization boundary to the rendering boundary.** Apply `display_safe` only at `bm list`-style human-rendering surfaces; let `bm export` emit the raw bytes as JSON-escaped (which `serde_json` does natively — `\x1b` becomes `` in the wire JSON, which parses back to `\x1b` on import). The round-trip then preserves underlying bytes per the spec's claim. The "downstream pipeline-renderable surfaces (terminals, log aggregators)" defense moves to the consumer's responsibility (consumers that pipe `bm export | tee /dev/tty` accept the terminal-escape risk because they chose to render to a terminal-renderable surface). This is the architecturally cleaner solution for the round-trip invariant.

Both are valid; the trade-off is between "export output is always terminal-safe" (path a; current implementation) vs. "export output is byte-faithful to the store" (path b; what the spec currently claims). The current implementation has the safety property without the byte-faithfulness; the spec asserts both.

**Proposed DESIGN.md amendment (path a — minimum change; matches current implementation):** revise [`DESIGN.md`](../../DESIGN.md) line 106 to drop the parenthetical claim about JSON-native escapes and explicitly name the sanitization-vs-round-trip-faithfulness trade-off:

> "**Phase 2b implementation verification:** the implementation must confirm `display_safe`-wrapped strings remain JSON-valid. The wire JSON carries the sanitized representation (Rust-style `\u{HHHH}` literals as JSON string content); the round-trip `bm export | bm import` preserves the **sanitized form**, not the original bytes. A store with control characters in URLs will round-trip into a destination store where those URLs carry the literal escape representation — operator-detectable via `bm list` length growth + visible escape sequences. The trade-off accepted: export output is always terminal-safe (consumers piping to terminals / log aggregators / web renders see no raw control bytes); byte-faithful round-trip is sacrificed in favor of pipeline safety."

**Proposed DESIGN.md amendment (path b — implementation change; matches spec's current intent):** revise DESIGN.md line 106 to require `display_safe` placement at the rendering boundary (not the serialization boundary), and revise the [`src/lib.rs:454-485`](../../src/lib.rs) `export_json` implementation to remove the per-field `display_safe` wrap. The wire JSON then carries JSON-native escapes for control chars (serde_json's default); the round-trip preserves bytes per the spec's claim. The terminal-escape-injection defense moves to the consumer's responsibility (a documented threat-model addition for `bm export` consumers that render to terminal-renderable surfaces).

**SA recommendation:** path (b) is architecturally cleaner — it preserves the canonical round-trip invariant the spec's existence depends on (DESIGN.md:114 "the round-trip invariant: `bm export | bm import` against a fresh destination store reproduces the source store's bookmarks"). The current implementation's choice (path a) is defensible but it weakens the round-trip invariant from "reproduces the source store's bookmarks" to "reproduces the source store's bookmarks modulo `display_safe`-sanitization." Operator-routed to SO for the trade-off ratification.

**Classification:** Raised to SO (Dim 12 + Dim 16)

---

### Deferred

<a id="r1-f4"></a>

**Finding 4 — `import_json` dedup is O(N×(M+N)) via `Vec::contains` — at the 10K-bookmark scale ceiling with a 10K-record import, the cost is ~10⁸ tuple comparisons; the algorithmic choice is undocumented + may exceed the implicit performance budget the Layer 2 PE work established (Dim 9)**

**Owner:** performance-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** performance-engineer

[`src/lib.rs:558-564`](../../src/lib.rs):

```rust
let mut appended = 0_usize;
for new_bm in imported {
    if !self.bookmarks.contains(&new_bm) {
        self.bookmarks.push(new_bm);
        appended += 1;
    }
}
```

`Vec::contains(&new_bm)` is O(M) where M is the current `self.bookmarks` length. The loop runs N times (N = imported payload length). Because each successful push immediately joins `self.bookmarks`, the total cost is:

```
sum_{i=0}^{N-1} (M + i) = N*M + N*(N-1)/2
```

At [`DESIGN.md:284`](../../DESIGN.md) § Performance budget's 10,000-bookmark scale ceiling, an attacker-or-operator-supplied import payload also at 10,000 records gives M=10K, N=10K → ~10⁸ tuple comparisons. Each comparison touches `url` (`String::eq` — O(url_length)) + `timestamp` (`DateTime::eq` — O(1)) + `tags` (`Vec<String>::eq` — O(total tag bytes)). For modest URL lengths (~100 chars) the total cost is on the order of 10¹⁰ byte comparisons — well outside any reasonable interactive-CLI time budget.

The current Layer 2 PE work established a `< 100 ms` budget on `bm list` at 1000 bookmarks ([`DESIGN.md:282`](../../DESIGN.md)) and a `< 5 ms` fsync budget at the same scale. Layer 3 has no published budget for `bm import`, but the implicit budget inherited from the Layer 2 work (and the `bm list` 100 ms target) would not survive the 10⁸-comparison shape at scale-ceiling import sizes.

**At the Layer 2 scale of 1000 bookmarks** with a 1000-record import the cost is ~10⁶ tuple comparisons, well within budget. The O(N²) shape only becomes a performance concern at the 10K scale ceiling — which DESIGN.md § Performance budget already names as the edge of acceptable scale. So the finding is conditional: at typical operator usage the current implementation is fine; at scale-ceiling operations it is not.

**Architectural choice that is also undocumented.** The natural alternative is `HashSet<&Bookmark>` (or a sorted+dedup pass) for O(N+M) dedup. The implementation chose the simpler `Vec::contains` shape. The choice is defensible — at the 1000-bookmark typical-use scale it's fine, and `HashSet<Bookmark>` requires `Bookmark: Hash` which the type does not currently derive — but the choice is undocumented and would surprise a Layer 4 maintainer raising the scale ceiling.

This is the same finding-class as [SA R2 F4](2026-05-21-solution-architect.md#r2-f4) (`filter_by_tags` is O(n log n) sort-then-filter rather than filter-then-sort) — currently carry-forward Deferred at Layer 2. Both are documentation-gap findings about complexity choices that are correct at the current scale but undocumented for future scale changes.

Three resolution paths (parallel to SA R2 F4):

- **(a) Document the choice.** Add to `import_json` doc comment (and DESIGN.md § Verification architecture pure-function annotation): "Dedup is `Vec::contains`-based at O(N×(M+N)) per call; this is the right shape at the project's 1K-typical / 10K-ceiling scale where `Bookmark: Hash` is not derived and the alternative `HashSet` cost is amortized over fewer records than the discipline of deriving + maintaining `Hash` justifies. A future Layer that raises the scale ceiling beyond 10K should consider `HashSet<Bookmark>` dedup with a `Hash` derive."
- **(b) Change to `HashSet`-based dedup.** Derive `Hash` on `Bookmark` (compatible with the existing `PartialEq` + `Eq` derives) and use `HashSet<Bookmark>` for dedup. Cost reduces to O(N+M). The implementation is straightforward; the trade-off is hash-time on every comparison (typically a small additive cost; for short URLs it may be slower than `String::eq` early-exit).
- **(c) Both — change the impl AND document.** Same trade-off shape as SA R2 F4's path (c).

**SA recommendation:** path (a). The current shape is correct at the project's actual scale; the documentation closes the "future maintainer wonders why" surface. Path (b) is a real performance improvement but the project's scale ceiling makes it cosmetic; path (c) is over-investment for the layer-3-as-reference-example purpose.

**Classification:** Deferred (Dim 9) — Coordination to Performance Engineer for the benchmark side (does the current shape exceed budget at 10K?); routes to SE if path (b) or (c) is chosen. Trigger to close: either DESIGN.md amendment naming the complexity choice OR `cargo bench`-level evidence that the current shape stays within budget at the 10K scale ceiling for import sizes.

---

<a id="r1-f5"></a>

**Finding 5 — `ImportError` variant set lacks per-record offending-index detail; the bare `SchemaMismatch(String)` carries serde's path info as opaque prose, but the library boundary loses the structured per-record-index data that a programmatic caller (Layer 4 batch-importer, future CI tooling) would want (Dim 4 — interface contracts)**

**Owner:** software-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** sanity-check

[`src/lib.rs:584-599`](../../src/lib.rs) `ImportError`:

```rust
pub enum ImportError {
    InvalidJson(String),
    SchemaMismatch(String),
}
```

Two variants, each carrying a `String` detail. Compare with Layer 2's `AttachTagError`:

```rust
pub enum AttachTagError {
    EmptyUrl,
    EmptyLabel,
    NoMatch(String),    // ← carries the URL value for the spec-contracted message
}
```

The `AttachTagError::NoMatch(String)` variant carries the URL so the `Display` impl can render the spec-contracted message + library callers (Layer 3 `import`, future test harnesses) don't need to re-construct from out-of-band scope — per [SA R2 F1 / Layer 2 carry-forward close at PR #46](2026-05-21-solution-architect.md#r2-f1). The same structural discipline does not apply to `ImportError::SchemaMismatch(String)`.

When a per-record validation fails (missing `url` field on record 47 of a 100-record import), the current `SchemaMismatch` variant carries a string like `"bookmark record validation failed: missing field \`url\` at line 12 column 28"` — serde's path/position info is in the string, but a programmatic caller cannot reliably extract "the offending record was the 47th in the array" without parsing serde's prose format.

The library-boundary concern: the CLI shell at [`src/main.rs:480-486`](../../src/main.rs) renders this directly to stderr via `display_safe(&detail)`. The current rendering is acceptable for the operator-facing CLI shell. But the library API at `BookmarkStore::import_json` is the same surface a Layer 4 batch-import tool, a CI-driven `bm`-script wrapper, or a future-replication-protocol would use. Each of those callers would benefit from structured per-record-index error data (e.g., "the import succeeded for records 0..46; record 47 failed at field `url` — please fix and retry").

The spec contract at [`DESIGN.md:122-123`](../../DESIGN.md) does not require per-record index data — it requires `Error: stdin JSON does not match storage-format schema; expected {"bookmarks": [...]}.` + the offending-field-mismatch detail. The current implementation satisfies the spec.

**Why this is a Deferred (not Raised-to-SO) finding:** the library API does not need per-record-index data to satisfy the Layer 3 spec, but the architectural shape of `ImportError` constrains future Layer-4-or-later programmatic-import-tool design. The `AttachTagError::NoMatch(String)` carry-forward at Layer 2 established the discipline that library-level error variants should carry the structured data the spec-contracted message contains; `ImportError::SchemaMismatch(String)` does not follow that discipline.

**Proposed enhancement (Layer 3 scope, optional):** extend `ImportError::SchemaMismatch` to a struct variant:

```rust
pub enum ImportError {
    InvalidJson { message: String, line: usize, column: usize },
    SchemaMismatch { detail: String, record_index: Option<usize> },
}
```

The `record_index` is `Option<usize>` because a top-level schema mismatch (missing `bookmarks` field; bookmarks-not-an-array) has no per-record index; only per-record validation failures have one. The CLI shell's rendering at `run_import` can ignore the new fields (continue rendering only the message+detail prose for backwards compatibility); a future programmatic caller can match on the structured fields.

This is a Layer 3 scope-question — the enhancement is straightforward but does not change observable spec behavior. Deferring rather than Raising-to-SO because (a) the spec does not require the change, (b) the Layer 4 use-case for programmatic per-record error handling is speculative at this point, and (c) the discipline-precedent of `AttachTagError::NoMatch(String)` is named but not load-bearing for the CLI-shell use that drives current behavior.

**Classification:** Deferred (Dim 4) — Coordination to SE if the operator chooses to land the enhancement at Layer 3; otherwise this remains Deferred as a Layer 4-or-later candidate. Trigger to close: structured `record_index` field added OR explicit DESIGN.md amendment naming the deliberate choice not to expose per-record-index in the library error API.

---

### Dismissed

<a id="r1-f6"></a>

**Finding 6 — `MAX_STDIN_BYTES_DEFAULT` lives in `lib.rs` despite being a CLI-shell concern; the constant is a stdin-size cap that the library `import_json` does not enforce (it only takes a `&str`). The placement is architecturally questionable (Dim 1)**

**Owner:** solution-architect

[`src/lib.rs:569-576`](../../src/lib.rs):

```rust
pub const MAX_STDIN_BYTES_DEFAULT: usize = 10 * 1024 * 1024;
```

The constant is consumed only by the CLI shell at [`src/main.rs:167`](../../src/main.rs) (`default_value_t = MAX_STDIN_BYTES_DEFAULT`) and used at [`src/main.rs:425-438`](../../src/main.rs) for the actual stdin read with `take(cap+1)`. The library `import_json(&mut self, payload: &str)` does not enforce any size cap — it accepts whatever `&str` it is given.

The architectural concern: a stdin-size cap is fundamentally a CLI-shell concern (only the CLI reads stdin), so placing the constant in `lib.rs` is mild scope-leakage from the effectful-shell into the pure-core. A stricter reading of the SA R1-established `lib.rs` (pure-core) vs `main.rs` (effectful-shell) split would put `MAX_STDIN_BYTES_DEFAULT` in `main.rs`.

**Counter-argument (why this is Dismissed, not a real finding):** the constant is genuinely shared API surface. The library's pub-export discipline (lines 21–23 of main.rs: `use bookmark_cli::{display_safe, AttachTagError, BookmarkStore, ImportError, MAX_STDIN_BYTES_DEFAULT};`) treats `MAX_STDIN_BYTES_DEFAULT` as part of the library's published surface — a future programmatic-import tool that wraps `BookmarkStore::import_json` would want the same default cap for its own input-source (a network socket; a file). Placing the constant in `lib.rs` advertises it as a project-wide convention, not a CLI-shell-private value. The doc comment at lines 569–575 explicitly names this — the constant exists "so the CLI shell uses this as the default for the `--max-stdin-bytes` flag; operators with legitimately-larger imports override at invocation time."

The pure/effectful split is about **functions** — pure functions don't perform I/O. A `const usize` performs no I/O. The "lib.rs is pure" framing the SA R1 reconciliation established applies to functions; constants are not in scope. The constant's placement is consistent with `lib.rs` being a project-wide library surface (data types + pure functions + project-wide constants like the JSON serialization shape's implicit recursion limit) while `main.rs` is the CLI-specific orchestration.

Verdict: the placement is defensible. The mild scope-leakage critique is real but the counter-argument (shared API surface across the library / CLI boundary) is stronger. A future Layer that splits the library into a separate crate would naturally hoist `MAX_STDIN_BYTES_DEFAULT` into the lib crate; the current single-crate structure makes the placement immaterial.

**Classification:** Dismissed (Dim 1) — the constant's library-level placement is consistent with its role as published API surface that both the CLI shell and future library consumers reference; the pure/effectful split applies to functions, not constants.

---

### Hallucinated

<a id="r1-f7"></a>

**Finding 7 — `export_json` uses `serde_json::json!` macro + Value-based JSON building rather than direct `Serialize` on a typed wrapper struct; this is less idiomatic + carries an `unwrap()` on `to_string` that requires a `#[allow(clippy::unwrap_used)]` carve-out. Should refactor to a typed wrapper struct (Dim 1 — separation of concerns; Dim 9 — complexity budget)**

The supplement-derived adversarial enumeration suggests using `Serialize`-derive on a `pub(crate) struct BookmarkExport<'a> { url: String, timestamp: DateTime<Utc>, tags: Vec<String> }` would be cleaner than the `serde_json::json!`-Value construction at [`src/lib.rs:458-484`](../../src/lib.rs):

```rust
let bookmarks_array: Vec<serde_json::Value> = source
    .iter()
    .map(|bm| {
        let tags_array: Vec<serde_json::Value> = bm
            .tags
            .iter()
            .map(|t| serde_json::Value::String(display_safe(t)))
            .collect();
        serde_json::json!({
            "url": display_safe(bm.url()),
            "timestamp": bm.timestamp(),
            "tags": tags_array,
        })
    })
    .collect();

let store_value = serde_json::json!({ "bookmarks": bookmarks_array });
let mut s = serde_json::to_string(&store_value).unwrap();
```

The hypothetical alternative:

```rust
#[derive(Serialize)]
struct BookmarkExport { url: String, timestamp: DateTime<Utc>, tags: Vec<String> }
#[derive(Serialize)]
struct StoreExport { bookmarks: Vec<BookmarkExport> }

let store = StoreExport { bookmarks: source.iter().map(|bm| BookmarkExport {
    url: display_safe(bm.url()),
    timestamp: bm.timestamp(),
    tags: bm.tags.iter().map(|t| display_safe(t)).collect(),
}).collect() };
serde_json::to_string(&store).expect("infallible Serialize")
```

The "refactor to typed wrapper" framing is plausible on first reading — typed structs ARE more idiomatic than `serde_json::Value` building in general, and the `unwrap()` + `#[allow(clippy::unwrap_used)]` is uglier than `expect("infallible Serialize")` on a `#[derive(Serialize)]` struct.

**Why this is Hallucinated:** the typed-wrapper alternative does NOT eliminate the `unwrap`/`expect`. Both `serde_json::to_string(&typed_struct)` and `serde_json::to_string(&value)` return `Result<String, serde_json::Error>`. The error path exists in both APIs because `Serialize::serialize` is fallible in the general case (the trait's signature returns `Result`); for in-memory data the failure mode is the same (allocator OOM). Both forms require the same `unwrap`-or-`expect` discipline; both require the same Clippy carve-out (or the same `expect("infallible Serialize")` rationale).

The `display_safe`-at-the-serialization-step requirement also constrains the wrapper-struct alternative — the wrapper struct would either need a custom `Serialize` impl (defeating the "more idiomatic" claim) OR would need to pre-wrap the strings (which is what the current implementation does, just with `json!` macro syntax instead of struct-field assignment). The structural cost of the two approaches is the same.

The complexity argument is also weak. The `serde_json::json!` macro is idiomatic Rust for this exact pattern (building structured JSON for export); the typed-wrapper alternative is idiomatic for receive-side parsing (where `#[derive(Deserialize)]` on the wrapper drives the schema validation). The current code uses both: `json!` for the export shape, `serde_json::from_value::<Vec<Bookmark>>` for the import schema validation. Each is the right idiom for its direction.

The Clippy `#[allow(clippy::unwrap_used, reason = "...")]` carve-out at lines 478–481 IS slightly noisy, but the supplement-recommended cleanup (use `expect` with a rationale) would be the same shape under either implementation choice. The supplement's "Rust idiom" framing here is template-matching against codebases that build their JSON via typed structs; this codebase deliberately uses the `json!` macro for the export-side and `from_value` for the import-side because the import-side needs typed deserialization (schema validation) and the export-side does not.

**Classification:** Hallucinated. The supplement-derived adversarial enumeration's "use typed wrapper structs" framing does not improve the architectural shape against this implementation's actual constraints; the `unwrap`/`expect` discipline applies identically to both approaches; the `display_safe`-at-serialization requirement is structurally orthogonal to the macro-vs-struct choice. Recorded per the sycophancy-check discipline.

---

### Summary

5 substantive findings + 1 Dismissed + 1 Hallucinated filed against Layer 3 in this first SA cold-session pass:

- **Finding 1** (Dim 12 — purity boundary placement) — **Resolved** at first cold-session pass. `export_json` is genuinely pure (`&self`, no I/O, no clock, deterministic); `import_json` is pure-transformation on `&mut self` under the SA R1 / R4-established VSDD purity definition. Layer 1 + Layer 2 purity-boundary discipline holds at Layer 3. No I/O sneak; no clock dependency; atomic mutation discipline structurally preserved.
- **Finding 2** (Dim 12 — cross-source divergence) — **Raised to SO**. DESIGN.md § Verification architecture's pure-function enumeration does NOT name `export_json` or `import_json`; the Layer 3 purity claims live only in (a) Phase-5-strategy prose at DESIGN.md:15 and (b) per-function doc comments. This is the exact pattern SA R1 F1 closed for Layer 1 — re-opening at Layer 3. Proposed DESIGN.md amendment text included.
- **Finding 3** (Dim 12 + Dim 16 — round-trip-byte-preservation invariant) — **Raised to SO**. DESIGN.md:106 explicitly claims `bm export | bm import` preserves underlying bytes via "JSON-valid `` sequences." `display_safe` actually produces Rust-style literal `\u{HHHH}` strings (8-char literals), not JSON-native escapes (6-char escapes). The round-trip does NOT preserve bytes; it preserves the sanitized form. Two SO-decidable resolution paths named (drop the byte-preservation claim vs. move `display_safe` to the rendering boundary); SA recommends path (b) for architectural cleanliness.
- **Finding 4** (Dim 9 — dedup complexity) — **Deferred** to PerformanceEngineer. `import_json`'s `Vec::contains`-based dedup is O(N×(M+N)) — at the 10K scale ceiling with a 10K-record import the cost is ~10⁸ comparisons. Acceptable at the typical 1K-bookmark scale; potentially exceeds the implicit budget at the scale ceiling. Three resolution paths named (document / change-to-HashSet / both); SA recommends document-only.
- **Finding 5** (Dim 4 — error contract) — **Deferred** to SE. `ImportError::SchemaMismatch(String)` does not follow the Layer 2 `AttachTagError::NoMatch(String)` discipline of carrying structured per-record data (e.g., `record_index`). Adequate for the CLI-shell rendering surface; constrains future programmatic-import-tool design. Optional Layer 3 enhancement; otherwise candidate for Layer 4-or-later.
- **Finding 6** (Dim 1) — **Dismissed**. `MAX_STDIN_BYTES_DEFAULT` placement in `lib.rs` is defensible — the constant is shared API surface across CLI + future library-consumer use; pure/effectful split applies to functions, not constants.
- **Finding 7** (Dim 1 + Dim 9 — typed-wrapper refactor) — **Hallucinated**. The supplement-derived "use typed struct, not `json!` macro" framing does not improve the architectural shape; `unwrap`/`expect` discipline applies to both approaches; `display_safe`-at-serialization is structurally orthogonal to the macro-vs-struct choice.

The pattern is consistent with a Layer 3 cold pass against an implementation that landed the spec contract cleanly: the substantive findings (F2 + F3) are both **spec-implementation alignment gaps** that need SO ratification — the purity-boundary enumeration is incomplete (F2) and the round-trip-byte-preservation claim is implementation-inconsistent (F3). The deferred findings (F4 + F5) are documentation / future-Layer concerns at the same shape as the SA R2 carry-forwards (F2 attach_tag/save-rationale; F4 filter_by_tags complexity-choice). The cold-session signal is that Layer 3 implementation is at MVR-blocked-by-spec-amendment for SA — the implementation is sound at the level the implementation can independently verify; the spec-side enumeration + the spec-side round-trip claim need ratification before SA can declare unambiguous MVR.

**Coordination:**

- **Finding 1** (Resolved purity coherence) — no coordination needed; documented for the audit trail.
- **Finding 2** (purity-boundary enumeration cross-source divergence) — routes to [Solution Owner](../SOLUTION-OWNER-REVIEW.md) for the DESIGN.md § Verification architecture amendment ratification.
- **Finding 3** (round-trip-byte-preservation invariant inconsistency) — routes to [Solution Owner](../SOLUTION-OWNER-REVIEW.md) for the path (a) vs path (b) trade-off ratification; cross-cuts to [Security](../SECURITY-REVIEW.md) on the `display_safe`-at-export-vs-rendering threat-model implications and to [Red Team](../RED-TEAM-REVIEW.md) on the round-trip-via-terminal-escape attack surface; cross-cuts to [Software Engineer](../SOFTWARE-ENGINEER-REVIEW.md) if path (b) is chosen (move `display_safe` out of `export_json`).
- **Finding 4** (dedup complexity at scale ceiling) — routes to [Performance Engineer](../PERFORMANCE-ENGINEER-REVIEW.md) for the benchmark side (does the current shape exceed budget at 10K imports?); routes to [SE](../SOFTWARE-ENGINEER-REVIEW.md) if path (b) `HashSet`-based dedup is chosen.
- **Finding 5** (ImportError variant set) — routes to [Software Engineer](../SOFTWARE-ENGINEER-REVIEW.md) if the structured `record_index` enhancement is chosen at Layer 3; otherwise carries as a Layer 4-or-later candidate.
- **Finding 6** (Dismissed) — no coordination.
- **Finding 7** (Hallucinated) — no coordination; recorded for audit completeness.

---

## Review 2 — 2026-05-25 04:30Z

**Round:** Layer 3 Phase 3 IAR Round 2.
**Scope:** Phase 3 IAR Round 2 cold-session retry against the post-Round-1-fix-work Layer 3 surface. Round 1 was closed with 5 substantive findings; Round 1 fix-work landed across [`fdfa989`](https://github.com/magnificentlycursed/guild-portfolio/commit/fdfa989) (Phase 1a+1b spec/narrative amendments — 13 finding-cluster fixes + 39 FINDINGS-INDEX rows backfilled), [`ba6a4a9`](https://github.com/magnificentlycursed/guild-portfolio/commit/ba6a4a9) (Phase 2a — 6 regression + coverage tests), [`bfc0713`](https://github.com/magnificentlycursed/guild-portfolio/commit/bfc0713) (Phase 2b — 4 routed substantive impl fixes; 51/51 GREEN), [`795bc25`](https://github.com/magnificentlycursed/guild-portfolio/commit/795bc25) (manual-tests/layer-3.md authoring + Phase 2c follow-up). Additionally, an **architectural-correction sub-decision** was authorized by the operator on 2026-05-25 during the Phase 2b landing (per [`bfc0713`](https://github.com/magnificentlycursed/guild-portfolio/commit/bfc0713) commit message): `display_safe` was removed entirely from `export_json` rather than rewritten with JSON-native escapes — because pre-escaping inside the serde_json encoding path double-escapes (literal `` text becomes `\\u001b` in JSON output and parses back as the 6-char text, NOT the original byte). Byte-preservation now relies structurally on serde_json's native string encoder handling Cc-range chars natively per RFC 8259 § 7. The Verification architecture refresh, the export_json doc-comment rewrite, and the manual-tests/layer-3.md Step 9 expected-output correction were inline-fixed by the main session during Round 2 spawn-prep (per the user-prompt enumeration). Round 2 scope (per AI Engineer Dim 8 scope-reducer): verify Round 1 fixes hold at the architectural surface + surface NEW residuals from the fix-work; do NOT re-find what Round 1 already closed.

Files read for this Round 2 pass: [`DESIGN.md`](../../DESIGN.md) lines 1–336 (full spec — emphasis on § Behavioral contracts § `bm export` (Layer 3) lines 103–114 + § `bm import` (Layer 3) lines 116–133 + § Verification architecture lines 228–260 + § Performance budget lines 280–300 + § Storage data classification § imported-tag provenance line 332); [`src/lib.rs`](../../src/lib.rs) lines 1–80 (module doc + Bookmark type) + 400–820 (export_json + import_json + bookmark_set_eq + MAX_STDIN_BYTES_DEFAULT + ImportError + display_safe); [`src/main.rs`](../../src/main.rs) lines 1–270 (CLI surface — Cli/Cmd doc-comments + handle_parse_error + emit_storage_error) + 400–533 (run_export + run_import + TagContainsControlChars rendering); [`manual-tests/layer-3.md`](../../manual-tests/layer-3.md) lines 225–306 (Step 9 control-char rejection + Step 10 byte-preservation round-trip) + lines 481–545 (Step 15 performance budget); per-domain Phase 4 routing appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md` (per-domain Phase 4 appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md`) (Round 1 routing record).

**Session note:** Cold-context retry session — the prior SA Round 2 spawn stalled at the file-write step (watchdog killed at 600s) after surfacing 4 candidate findings; this reviewer authored the file early per the user-prompt directive. Sycophancy-compensation: this is the second Round-2 attempt against the same fix-work surface, but this reviewer did NOT see the prior attempt's surfaced items (independent rediscovery per the user-prompt instruction). The adversarial probe specifically targets the 6 user-prompt-named lenses: (1) regression-check Round 1 fixes hold at the architectural surface; (2) architectural correction sub-decision soundness (`display_safe` removal from `export_json`); (3) Verification architecture refresh completeness post-inline-fix; (4) Performance budget consistency between DESIGN.md + manual-tests/layer-3.md Step 15; (5) `bookmark_set_eq` architectural placement; (6) `ImportError::TagContainsControlChars` variant shape consistency with the Layer 2 `NoMatch(String)` precedent. Trade-off declared per primer 3 § Session isolation: inline cold-session (not cluster-batched) at SA-only scope against a bounded ~200-LOC fix-work delta is the inline-acceptable case; the cluster-batched cold-session shape (the gold standard) is reserved for the multi-domain Round 2 wave that the main session is orchestrating in parallel.

**Source:** domain-raised — the SA Round 2 dimensions (1–12) applied against the post-fix surface surfaced these findings; no operator interruption mid-round; one regression-replay element (Round 1 F1/F2/F3 verification-check is structurally a regression-replay, but the new findings F2-F5 below are independently rediscovered).

**Reviewer:** solution-architect (cold-session retry, no in-conversation context from any prior Round 2 attempt or the Round 1 fix-work authoring).

**Model:** Opus 4.7.

**Supplements applied:** [`rust.md`](../../../../vsdd-suite/supplements/rust.md) § Solution Architect — applies because Round 1 fix-work extended the library surface with (a) new `bookmark_set_eq` free-standing fn + (b) new `ImportError::TagContainsControlChars(usize, String)` variant + (c) rewritten `export_json` body with new `ExportShape<'a>` inner struct + (d) rewritten `display_safe` using JSON-native `\uHHHH` escape format. The supplement's Error-type-hierarchy + module-organization + Serialize-derived-wrapper dimensions are directly load-bearing on this fix-work surface.

**Cold-session shape:** N/A — inline retry from the main session, after the prior 600s watchdog stall on the first Round 2 spawn. The file-write-first discipline applies per the user-prompt directive (write the file before doing final summary work; iterate against the written file rather than holding findings in-memory until session end).

**Regression-check against:** [SA Review 1 — 2026-05-25 01:12Z](#review-1--2026-05-25-0112z) (Layer 3 Round 1 — F1 Resolved purity coherence; F2 Raised-to-SO verification-architecture refresh; F3 Raised-to-SO display_safe round-trip 4-domain convergence; F4 Deferred dedup complexity; F5 Deferred ImportError variant detail).

---

### Resolved

<a id="r2-f1"></a>

**Finding 1 — Round 1 fixes hold at the architectural surface; the 5-finding Round 1 disposition is structurally closed by the post-fix surface (Dim 12 — purity boundary integrity + Dim 16 — interface contract integrity)**

**Owner:** solution-architect
**Status:** validated
**Blocked by:** *(none)*
**Validator:** sanity-check

Regression-check verdict against each Round 1 finding:

**Round 1 F1 (Resolved — purity coherence).** Holds. Post-fix `export_json` at [`src/lib.rs:460-505`](../../src/lib.rs) remains `&self`-receiving, no I/O, no clock, deterministic. The architectural-correction sub-decision (removal of per-field `display_safe` wrap) actually *strengthens* the purity claim — the function now passes `&Bookmark` references straight to serde's `Serialize` impl via the `ExportShape<'a>` wrapper rather than allocating intermediate `display_safe`-wrapped strings. Post-fix `import_json` at [`src/lib.rs:545-620`](../../src/lib.rs) remains `&mut self`-receiving with the same atomicity discipline (validation chain at lines 553–596 fires before any mutation at lines 608–619); the new active control-char rejection at lines 590–596 fires pre-mutation per the atomicity invariant. The new `bookmark_set_eq` helper at lines 631–643 is pure (no I/O, no clock, deterministic; works against clones of the input `tags` Vecs to avoid mutating the source). All three new pure-surface members (export_json post-correction; import_json with new rejection; bookmark_set_eq) remain on the pure side.

**Round 1 F2 (Raised-to-SO — verification-architecture refresh).** Closed by the inline-fix during Round 2 prep: [`DESIGN.md:237-239`](../../DESIGN.md) now enumerates Layer 3 entries for `export_json` + `import_json` + `display_safe`. The "Post-Round-1 architectural correction" inline annotation at line 237 names that `display_safe` is NOT applied at the per-field serialization step (closing the previous spec/impl drift). The `import_json` entry at line 238 names the sorted-tag-comparison dedup + the `&mut self` morally-pure framing. The `display_safe` entry at line 239 names the JSON-native `\uHHHH` escape form. Verification architecture is now Layer-3-complete.

**Round 1 F3 (Raised-to-SO — display_safe round-trip 4-domain convergence).** Closed via the operator-authorized architectural-correction sub-decision: `display_safe` removed entirely from `export_json` (rather than the routing-pass-1 Path-C "switch display_safe to JSON-native escape syntax" — which would have triggered the double-escape problem the commit message at [`bfc0713`](https://github.com/magnificentlycursed/guild-portfolio/commit/bfc0713) names). The byte-preservation round-trip now holds structurally because serde_json's native string encoder produces RFC-8259-compliant `\uHHHH` escapes for Cc-range control chars (which a standard JSON parser recovers as the original byte). The Step 10 manual-test at [`manual-tests/layer-3.md:258-306`](../../manual-tests/layer-3.md) confirms the byte-preservation property end-to-end. The two unit tests at [`src/lib.rs:1044-1075`](../../src/lib.rs) confirm `display_safe` emits the JSON-native form at the render boundary. F3 closed; replaced by the new F2 below (the architectural-correction-leftover residual).

**Round 1 F4 (Deferred — dedup complexity).** Closed at the spec layer via the new accepted-limitation paragraph at [`DESIGN.md:298`](../../DESIGN.md) (Performance budget § Layer 3 — `bm import` dedup complexity accepted limitation). The paragraph names the O(M × N) sorted-tag-comparison dedup cost at the 10K × 10K ceiling, frames it as accepted-limit parallel to the Layer 1 cumulative O(N²) add-cost, and names HashSet-based dedup as a future optimization candidate. F4 closed at SA scope; PE side closed via the converging routing decision.

**Round 1 F5 (Deferred — ImportError variant detail).** Routed to a follow-up PR per the Phase 4 routing record line 282 ("Low priority; can defer to follow-up PR"); the deferral is the operator-confirmed disposition. The `record_index: Option<usize>` enhancement is correctly NOT in the Round-1-fix-work scope; the deferral is structurally honest. F5 closed as Deferred-to-follow-up.

**Verdict:** All five Round 1 findings either Resolved-on-fix-landing (F1 still holds; F2 + F3 closed by fix-work; F4 closed at spec layer) or deliberately-deferred (F5). No Round 1 finding regressed. The architectural-correction sub-decision soundly preserved (rather than weakened) the purity-boundary discipline + the round-trip invariant.

**Resolution:** Round 1 fix-work closes the substantive Round 1 dispositions; the architectural-correction sub-decision (display_safe removal from export_json) is architecturally clean — it reaches the SA R1 F3 path-(b) recommendation by a different mechanism (no display_safe at all, rather than display_safe at rendering boundary only — though structurally those amount to the same thing since `bm list` already applies display_safe at its render path). Validated at Round 2 cold-session pass.

---

### Deferred

<a id="r2-f2"></a>

**Finding 2 — `import_json` `# Errors` rustdoc section does NOT enumerate the new `ImportError::TagContainsControlChars` variant; the rustdoc claim is incomplete against the implementation post-fix-work (Dim 16 — interface contract documentation)**

**Owner:** software-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** sanity-check

[`src/lib.rs:538-544`](../../src/lib.rs) `import_json` rustdoc `# Errors` section:

```rust
/// # Errors
///
/// - `ImportError::InvalidJson` if `payload` is not valid JSON.
/// - `ImportError::SchemaMismatch` if the JSON parses but does not
///   match the storage-format object-wrapped shape, OR if any
///   per-bookmark record fails schema validation (missing required
///   `url`/`timestamp` field, wrong field type, empty `url`).
pub fn import_json(&mut self, payload: &str) -> Result<usize, ImportError> {
```

The function returns `Result<usize, ImportError>` where `ImportError` has three variants at [`src/lib.rs:660-689`](../../src/lib.rs):

1. `InvalidJson(String)` — listed
2. `SchemaMismatch(String)` — listed
3. `TagContainsControlChars(usize, String)` — **NOT listed**

The third variant was added at Phase 2b commit [`bfc0713`](https://github.com/magnificentlycursed/guild-portfolio/commit/bfc0713) per the Round 1 routing decision (imported-tag control-char rejection — Security R1 F2 active-mitigation). The variant itself is well-defined (Display impl at lines 696–699; the CLI shell renders it at [`src/main.rs:520-530`](../../src/main.rs); the manual-test at [`manual-tests/layer-3.md:225-254`](../../manual-tests/layer-3.md) exercises the path). The implementation throws the variant at [`src/lib.rs:590-596`](../../src/lib.rs):

```rust
for (idx, bm) in imported.iter().enumerate() {
    for tag in &bm.tags {
        if tag.chars().any(|c| c.is_control() || is_format_char(c)) {
            return Err(ImportError::TagContainsControlChars(idx, tag.clone()));
        }
    }
}
```

The `# Errors` rustdoc section is the interface-contract surface a library consumer (Layer-4 batch-importer; test-harness author; future-replication-protocol designer) would read to know what error paths to handle. The current section says only two variants exist, which would cause a library consumer to write a non-exhaustive `match` on `import_json`'s return — and the compiler would warn against that, but the consumer might add a `_ => unreachable!()` arm under the rustdoc's incorrect-but-officially-named-two-variant claim. That `unreachable!()` would fire on the first stdin-fed-attacker control-char tag, panicking the consumer.

**Why this is Deferred-to-follow-up (not Resolved at this Round 2 pass):** the variant exists in code; the rustdoc lags. This is an **architectural-correction-leftover regression** — the same class as the inline-fixes the main session already applied for the src/lib.rs:1043+1057 unit tests, the Cli Export doc-comment, the manual-tests/layer-3.md Step 9 expected output, and the export_json /// doc-comment. The rustdoc gap is the same shape: Round 1 fix-work introduced a new code surface (TagContainsControlChars variant) but did not propagate to the per-function rustdoc. SA does not edit source files at Phase 3 IAR scope; the fix routes to SE as a small follow-up edit.

**Proposed fix (minimal; SE scope):** add a third `# Errors` bullet:

```rust
/// - `ImportError::TagContainsControlChars` if any imported record's `tags`
///   array contains a control character (`is_control()`) or curated format
///   character (per `is_format_char` — bidi controls, ZWJ, etc.). Active-
///   mitigation per `DESIGN.md` § `bm import` (Layer 3) failure contract
///   on imported records containing control-char tags. The variant carries
///   the offending record's index within the imported payload + the
///   offending tag string.
pub fn import_json(&mut self, payload: &str) -> Result<usize, ImportError> {
```

This closes the rustdoc-vs-implementation drift at a 6-line edit; no behavior change required; no test change required.

**Classification:** Deferred (Dim 16) — small, well-scoped follow-up SE edit. Trigger to close: the `# Errors` bullet added; the next library-consumer reading the rustdoc sees the complete variant enumeration. This is the same fix-class as the main-session inline-fixes during Round 2 prep; flagged here so the next SE pass picks it up.

---

### Raised to SO

<a id="r2-f3"></a>

**Finding 3 — Performance budget cross-source inconsistency: `manual-tests/layer-3.md` Step 15 asserts a `< 200 ms` "relaxed envelope" for `bm import` at the 1000 × 1000 dedup scale, but [`DESIGN.md`](../../DESIGN.md) § Performance budget table at lines 284–288 lists ONLY the `< 100 ms` Layer 1 budgets — no explicit `bm import` budget anywhere in DESIGN.md. The 200 ms relaxation is asserted only in the manual-test prose and the inline narrative at [`DESIGN.md:298`](../../DESIGN.md) accepted-limit paragraph (which names the complexity shape but does NOT publish a numeric budget) (Dim 12 + Dim 16 — cross-source contract integrity)**

**Owner:** solution-owner
**Status:** raised
**Blocked by:** *(none)*
**Validator:** sanity-check

[`manual-tests/layer-3.md:531-537`](../../manual-tests/layer-3.md):

> | Operation | Budget (p95 per `DESIGN.md`) | Pass criterion (mean at N=10) |
> |---|---|---|
> | `bm export` (1,000-bookmark store) | < 100 ms | mean < 100 ms |
> | `bm export --tag rust` (1,000-bookmark store) | < 100 ms | mean < 100 ms |
> | `bm import` (10K dedup-against-existing-state at 1,000 × 1,000) | **< 200 ms (relaxed per dedup-complexity accepted-limit)** | mean < 200 ms |
>
> The `bm import` budget is **intentionally looser** per the [Layer 3 dedup-complexity accepted-limit annotation](../DESIGN.md#performance-budget-) — the O(M × N) sorted-tag-comparison dedup at 1,000 × 1,000 is ~10^6 comparisons + JSON re-parse + atomic write; 200 ms is the documented acceptable envelope.

The manual-test step **claims** the 200 ms relaxation is "per `DESIGN.md`", but [`DESIGN.md`](../../DESIGN.md) § Performance budget table at lines 284–288 lists only:

> | `bm --help` / `bm --version` startup | < 50 ms wall-clock |
> | `bm add <url>` end-to-end | < 100 ms wall-clock on a store with ≤ 1,000 bookmarks |
> | `bm list` end-to-end | < 100 ms wall-clock on a store with ≤ 1,000 bookmarks |

No `bm export` row. No `bm import` row. No 200 ms relaxation row. The accepted-limit paragraph at [`DESIGN.md:298`](../../DESIGN.md) names the complexity shape (O(M × N) at the 10K × 10K ceiling = ~100M comparisons) but stops short of publishing a per-operation numeric budget — it explicitly defers numeric quantification: "the constant factor stays well under the < 100 ms budget" (a return-to-100-ms claim, NOT a 200-ms relaxation).

The manual-test's 200 ms figure is therefore **synthesized in the manual-test layer, not authored in DESIGN.md**. A cold-reader of DESIGN.md sees the < 100 ms budget for the project-wide "1000-bookmark interactive operation" surface; a cold-reader of manual-tests/layer-3.md sees the < 200 ms "relaxed per dedup-complexity accepted-limit" budget. The two surfaces disagree on what budget applies to `bm import`.

**Architectural concern:** the manual-test is supposed to be the **operator-runnable verification** of the spec's contracts (per primer 1c § Manual testing checklist + the per-layer manual-test discipline). When the manual-test publishes a budget figure that DESIGN.md does not author, the audit-trail's "single source of truth" discipline is broken — a future operator reading the manual-test result ("`bm import` mean = 150 ms — PASS") cannot determine whether the result satisfies the project's actual budget or the manual-test's locally-asserted-but-spec-unanchored budget. The 200 ms figure may be perfectly defensible (it's within the typical-CLI-latency envelope; the dedup cost analysis above justifies the relaxation), but it needs to be **published in DESIGN.md** to count as the project's contract.

**Three resolution paths:**

- **(a) Publish the relaxed `bm import` budget in DESIGN.md.** Extend the § Performance budget table to add a new row: `bm import` (end-to-end at 1,000-bookmark destination state × 1,000-record import payload) — `< 200 ms wall-clock` with an explanatory parenthetical citing the dedup-complexity accepted-limit. The manual-test's claim then has authoritative backing.

- **(b) Tighten the manual-test budget to the existing < 100 ms.** If the 200 ms relaxation was not actually operator-authorized at Round 1 routing (the routing record line 298 cites the SA F4 + PE F1 convergence but does NOT name a specific numeric budget), the manual-test should hold the same < 100 ms bar as the rest of the project's interactive operations. If `bm import` at 1K × 1K cannot meet the < 100 ms bar in practice, that itself is a finding — either the implementation needs HashSet-based dedup OR the spec needs the explicit budget relaxation per path (a).

- **(c) Defer the budget publication until empirical evidence accumulates.** Add a tracking note to DESIGN.md § Performance budget: "Layer 3 `bm import` budget is empirically observed but not yet contracted; the Phase 5 hardening cycle will determine the published budget after the cargo-fuzz harness produces edge-case timing evidence." This preserves the audit-trail discipline while acknowledging that the spec doesn't yet have enough evidence to publish a number.

**SA recommendation:** path (a). The accepted-limitation paragraph at DESIGN.md:298 ALREADY commits to the architectural disposition; publishing the numeric budget in the table is the small additional step that closes the cross-source inconsistency. The 200 ms figure is operator-defensible (it's within the typical-CLI-latency envelope and the routing-pass-1 record names the same dedup-complexity framing). Path (b) is too aggressive — it would require either HashSet dedup work or evidence that the manual-test's 200 ms claim was unauthorized; both impose larger work than path (a). Path (c) is the right shape if the operator wants to defer; it's an acceptable interim if (a) is not actionable in this PR.

**Classification:** Raised to SO (Dim 12 + Dim 16) — operator-routed for the spec amendment to publish (or revise) the `bm import` performance budget. SA recommends path (a); routes to PE for the validation that 200 ms is the right number empirically.

---

### Deferred (continued)

<a id="r2-f4"></a>

**Finding 4 — Verification architecture § Layer 3 entry for `display_safe` claims its formal-verifiability scope "expands at Layer 3 because the round-trip-byte-preservation invariant against a JSON parser becomes the testable contract (Phase 5 proptest target)" — but `display_safe` is no longer in the round-trip path (the architectural correction removed it from `export_json`). The Layer 3 expansion claim is a vestigial residual from the pre-architectural-correction routing (Dim 12 — purity boundary descriptive accuracy)**

**Owner:** solution-architect
**Status:** raised
**Blocked by:** *(none)*
**Validator:** sanity-check

[`DESIGN.md:239`](../../DESIGN.md) § Verification architecture § Layer 3 pure-function enumeration:

> - **Layer 3:** `display_safe(s: &str) -> String` — pure function over strings; emits the JSON-native `\uHHHH` escape form per the JSON-native-escape-design Round 1 Phase 4 routing decision. The function was previously named at Layer 1 but its formal-verifiability scope expands at Layer 3 because the round-trip-byte-preservation invariant against a JSON parser becomes the testable contract (Phase 5 proptest target).

The claim "round-trip-byte-preservation invariant against a JSON parser becomes the testable contract" is the **pre-architectural-correction framing**. Routing-pass-1 chose switch-to-JSON-native-escape (`display_safe` from Rust-syntax `\u{HHHH}` to JSON-native `\uHHHH`) so the round-trip would preserve bytes through `display_safe` → JSON encoder → JSON parser → store. The architectural-correction sub-decision at [`bfc0713`](https://github.com/magnificentlycursed/guild-portfolio/commit/bfc0713) discovered the double-escape problem (literal `` text becomes `\\u001b` in JSON output and parses back as 6-char text, NOT original byte) and **removed `display_safe` from `export_json` entirely**.

Post-correction, the byte-preservation round-trip flows: original ESC byte in Rust `String` → serde_json's native encoder produces `` JSON escape → JSON parser recovers original ESC byte → store. **`display_safe` is no longer in this path.** The round-trip property is now a contract on serde_json's native string encoder + the JSON parser's escape-recovery, NOT on `display_safe`.

The Verification architecture entry for `display_safe` retains the pre-correction claim that its "formal-verifiability scope expands at Layer 3" because of the round-trip property. That's no longer accurate. Post-correction, `display_safe` is purely a rendering-boundary sanitizer (used at `bm list`'s `eprintln!`/`println!` paths in [`src/main.rs`](../../src/main.rs) and at [`src/main.rs:528`](../../src/main.rs) for `TagContainsControlChars`'s tag-rendering); the round-trip property does not flow through it.

This is a **vestigial-spec-narrative** residual from the same class as the architectural-correction-leftover inline-fixes the main session applied during Round 2 prep. The export_json /// doc-comment was rewritten (per the user-prompt enumeration); the Verification architecture entry for `display_safe` was not. The descriptive accuracy gap is:

- **Pre-correction claim:** `display_safe`'s formal-verifiability scope expands because the round-trip invariant is testable against it (proptest target: `parse(display_safe(s)) == s` modulo escape recovery)
- **Post-correction reality:** the round-trip invariant is a property of serde_json + the JSON parser; `display_safe`'s testable contracts are (a) every Cc-range input produces a `\uHHHH` 6-char escape in the output; (b) every curated-format-char input produces the same; (c) `\n` and `\t` are preserved; (d) ASCII-printable non-control chars pass through unchanged. NONE of these are "round-trip-byte-preservation against a JSON parser" — they are local-input-output properties of `display_safe` itself.

**Why Deferred (not Raised-to-SO):** the gap is descriptive-accuracy, not contract-integrity. The implementation is correct; the spec's claim about `display_safe`'s Phase-5-proptest-target shape is slightly mis-framed but does not change observable behavior. A future Phase 5 proptest authoring pass would discover the framing gap on its own (the proptest author would write a property targeting the round-trip and realize it should be over serde_json's `to_string`+`from_str`, not over `display_safe`). The gap is documentation-level + does not gate Layer 3 layer-gate close.

**Proposed DESIGN.md amendment (Deferred to follow-up):** revise the `display_safe` Verification architecture entry to:

> - **Layer 3:** `display_safe(s: &str) -> String` — pure function over strings; emits the JSON-native `\uHHHH` escape form (BMP codepoints) or UTF-16 surrogate-pair `\uD8xx\uDCxx` form (Supplementary Plane). Used at the render boundary (`bm list` output, `bm import` error-rendering of attacker-controlled tag bytes), NOT at the JSON serialization boundary (per architectural correction at Phase 2b Round 1: the round-trip-byte-preservation property holds structurally via serde_json's native string encoder + the JSON parser's escape recovery, without `display_safe` in the path). Formal-verifiability scope: local input-output properties (every Cc-range and curated-format-char input produces the spec-named escape; `\n` and `\t` are preserved). Phase 5 proptest targets: `display_safe(printable_ascii) == printable_ascii` ∧ `display_safe(control_char).contains("\\u{:04x}".format(cp))`.

This closes the descriptive-accuracy gap without changing the implementation.

**Classification:** Deferred (Dim 12) — small, well-scoped follow-up spec-narrative edit. Owner: SA (the author of the pre-correction routing record that introduced the vestigial framing). Trigger to close: the Verification architecture `display_safe` entry rewritten to reflect post-correction reality. Routes to the same follow-up PR as Round 2 F2 (the rustdoc-`# Errors`-enumeration gap) — both are Round-1-fix-work-leftover residuals at the documentation surface.

---

### Dismissed

<a id="r2-f5"></a>

**Finding 5 — `bookmark_set_eq` at [`src/lib.rs:631-643`](../../src/lib.rs) lives as a free-standing module-level private fn rather than a method on `Bookmark` (e.g., `impl Bookmark { fn set_eq(&self, other: &Bookmark) -> bool }`) or a custom trait. The placement is architecturally questionable since the function is a structural-equality predicate semantically owned by the `Bookmark` type (Dim 1 — module organization)**

**Owner:** solution-architect

[`src/lib.rs:623-643`](../../src/lib.rs):

```rust
/// Sorted-tag-comparison bookmark equality for Layer 3 `import_json` dedup.
///
/// Compares records on (`url`, `timestamp`, sorted `tags`) per Round 1
/// Phase 4 routing sorted-tag-comparison-dedup decision. Resolves
/// `DESIGN.md` L132 (byte-equal frame) vs L223 (set-frame) internal
/// tension toward the L223 set-frame. Tag-order differences do not make
/// records distinct for dedup purposes; storage `Vec<String>` still
/// preserves insertion order at the record level.
fn bookmark_set_eq(a: &Bookmark, b: &Bookmark) -> bool {
    if a.url != b.url || a.timestamp != b.timestamp {
        return false;
    }
    if a.tags.len() != b.tags.len() {
        return false;
    }
    let mut a_tags = a.tags.clone();
    let mut b_tags = b.tags.clone();
    a_tags.sort();
    b_tags.sort();
    a_tags == b_tags
}
```

The function is semantically an equality predicate on `Bookmark` (the set-frame variant of `Bookmark::eq`). The natural Rust idiom is either a method on `Bookmark` (`impl Bookmark { fn set_eq(&self, other: &Self) -> bool }`) or a custom trait (`trait SetEq { fn set_eq(&self, other: &Self) -> bool; } impl SetEq for Bookmark { ... }`). The current shape is the free-standing-private-fn idiom, accessed at [`src/lib.rs:613`](../../src/lib.rs) via `bookmark_set_eq(existing, &new_bm)`.

The architectural concern: free-standing fns that conceptually belong to a type fragment the type's logical surface across the module. A maintainer adding `Bookmark::byte_eq` (the regular PartialEq) + `Bookmark::set_eq` (the sorted-tags variant) + `Bookmark::display_eq` (a future Cf-stripped variant) would naturally cluster these as impl-block methods; the current shape forces the maintainer to choose between adding a method (consistent with the new additions but inconsistent with `bookmark_set_eq`'s existing placement) and adding another free-standing fn (consistent with `bookmark_set_eq` but fragmenting the type's logical surface).

**Counter-arguments (why this is Dismissed):**

- **The fn is `pub(self)` (file-private), not `pub` API.** A method on `Bookmark` would be `pub(self)` too unless explicitly exposed — same access-control shape; same library-surface visibility (none). The pub-vs-pub(self) consideration is orthogonal to the method-vs-free-fn choice.
- **The fn's single call-site is at [`src/lib.rs:613`](../../src/lib.rs) (`bookmark_set_eq(existing, &new_bm)`).** A method form would read `existing.set_eq(&new_bm)`. Both are 4-token expressions; the method form is slightly more idiomatic but the free-fn form is not architecturally wrong — it's a stylistic preference.
- **The fn is intentionally scoped to Layer 3 dedup.** Naming it `bookmark_set_eq` (with the `bookmark_` prefix) signals "this is a Bookmark-related helper specific to the dedup use case" — the maintainer reading the function name sees the scope without needing the impl-block context. A `Bookmark::set_eq` method would lose the dedup-specific signal.
- **The Rust idiom for equality variants on a derived-`PartialEq` type IS the free-fn helper.** Deriving `PartialEq` on `Bookmark` already provides the byte-equal variant via `==`. The convention for additional equality variants (sorted-tag-equal; case-insensitive-URL-equal; etc.) is to provide named free-fns rather than methods, because methods shadow the derived `eq` in a way that the free-fn does not. The current shape is the idiomatic Rust choice.
- **The fn's body uses `a.tags.clone()` + `b.tags.clone()` + `sort()` rather than the more efficient `as_slice().sort_by(...)` — but this is a Performance concern, not a placement concern; routed to PE if it matters.** (At Layer 3's typical scale of < 10 tags per bookmark, the clone+sort cost is negligible against the JSON-parse + atomic-write costs of the surrounding import flow.)

**Verdict:** the placement is defensible. The maintainer-clustering concern is real but minor; the named-free-fn-for-equality-variants idiom is the more common Rust shape; the dedup-specific naming `bookmark_set_eq` carries the scope-signal that a `Bookmark::set_eq` method would lose. A future Layer adding multiple equality variants would naturally introduce a `BookmarkEquality` trait at that point, hoisting `bookmark_set_eq` into the trait; the current single-variant single-call-site shape makes the placement immaterial.

**Classification:** Dismissed (Dim 1) — the free-standing-private-fn placement of `bookmark_set_eq` is the idiomatic Rust choice for an equality-variant helper that does not need to shadow `PartialEq`'s derived `eq`; the maintainer-clustering critique is real but does not warrant architectural rework. Recorded per the sycophancy-check discipline (the placement question is legitimate to raise but the counter-argument is stronger).

---

### Hallucinated

<a id="r2-f6"></a>

**Finding 6 — `ImportError::TagContainsControlChars(usize, String)` uses positional tuple fields rather than struct-variant named fields (`{ record_index: usize, offending_tag: String }`). The positional shape is inconsistent with the Layer 2 `AttachTagError::NoMatch(String)` precedent and harder to use programmatically — should be struct-variant (Dim 4 — interface contract clarity)**

The supplement-derived adversarial enumeration suggests `TagContainsControlChars(usize, String)` should be `TagContainsControlChars { record_index: usize, offending_tag: String }` for clarity at the variant API surface. The struct-variant form makes the variant's fields named-not-positional, which is more idiomatic for variants carrying multiple data items.

**Why this is Hallucinated:** the Layer 2 `AttachTagError::NoMatch(String)` precedent is itself **positional** — single `String` field, no name. The single-field positional shape is the precedent that Round 1 F5 (Deferred — ImportError variant detail) named as the structural-discipline precedent. The `TagContainsControlChars(usize, String)` variant follows the same positional shape, just with two fields instead of one. Switching to struct-variant for `TagContainsControlChars` while leaving `NoMatch(String)` positional would create inconsistency across the error-type hierarchy, not resolve it.

The struct-variant form would be cleaner in isolation but the consistency-with-`AttachTagError` discipline is the load-bearing constraint. Either both types should be struct-variant (which would change the Layer 2 surface — out of scope for Layer 3) or both should stay positional (which the current implementation does — consistent across both types).

The Round 1 F5 Deferred finding already names the struct-variant enhancement as a Layer-3-optional / Layer-4-candidate, but it explicitly notes the trade-off ("the discipline-precedent of `AttachTagError::NoMatch(String)` is named but not load-bearing for the CLI-shell use that drives current behavior"). Round 2 surfacing the same finding-class against `TagContainsControlChars` would be re-finding Round 1's already-Deferred disposition under a slightly different angle — explicitly out-of-scope per the Round 2 scope-reducer ("do NOT re-find what Round 1 already closed").

The `(usize, String)` tuple is also accessed at exactly one call-site ([`src/main.rs:520-530`](../../src/main.rs) with the canonical destructure `Err(ImportError::TagContainsControlChars(idx, tag)) => { ... }`) — the positional access reads idiomatically at that single site; the struct-variant form would gain clarity at the variant definition but lose nothing at the use site.

**Classification:** Hallucinated. The positional `(usize, String)` shape on `TagContainsControlChars` is consistent with the Layer 2 `AttachTagError::NoMatch(String)` precedent + the Round 1 F5 Deferred disposition; surfacing it as a separate Round 2 finding would re-find a structurally-equivalent already-Deferred concern. Recorded per the sycophancy-check discipline.

---

### Summary

Round 2 SA cold-session retry against the post-Round-1-fix-work Layer 3 surface produced 1 Resolved (Round 1 fixes regression-check) + 2 Deferred + 1 Raised-to-SO + 1 Dismissed + 1 Hallucinated = 4 substantive findings + 2 non-substantive (1 Dismissed + 1 Hallucinated) for audit completeness:

- **Finding 1** (Dim 12 + Dim 16 — Round 1 fix-work regression-check) — **Resolved**. All five Round 1 findings hold post-fix: F1 purity coherence preserved (and structurally strengthened by the architectural-correction sub-decision); F2 verification-architecture refresh closed via inline-fix; F3 round-trip byte-preservation closed via the architectural-correction sub-decision (display_safe removed from export_json entirely, byte-preservation now structural via serde_json's native encoder); F4 dedup complexity closed at spec layer via the accepted-limitation paragraph; F5 ImportError variant detail correctly deferred to follow-up PR. The architectural-correction sub-decision is architecturally sound — it reaches the SA R1 F3 path-(b) recommendation by a cleaner mechanism (no display_safe in the round-trip path at all).
- **Finding 2** (Dim 16 — rustdoc-vs-implementation drift) — **Deferred**. `import_json` `# Errors` section does NOT enumerate the new `TagContainsControlChars` variant. This is an architectural-correction-leftover residual — same class as the inline-fixes the main session applied during Round 2 prep. Small, well-scoped 6-line SE edit; flagged for follow-up.
- **Finding 3** (Dim 12 + Dim 16 — performance budget cross-source inconsistency) — **Raised to SO**. `manual-tests/layer-3.md` Step 15 asserts a < 200 ms "relaxed envelope" for `bm import` at 1000 × 1000 dedup, but DESIGN.md § Performance budget table publishes no `bm import` row + the accepted-limitation paragraph at line 298 stops short of publishing a numeric budget. The 200 ms figure is therefore synthesized in the manual-test layer rather than authored in DESIGN.md. Three resolution paths named (publish in DESIGN.md / tighten manual-test / defer); SA recommends path (a) — publish the relaxed budget in DESIGN.md to close the cross-source inconsistency.
- **Finding 4** (Dim 12 — vestigial-spec-narrative descriptive accuracy) — **Deferred**. Verification architecture § Layer 3 entry for `display_safe` claims its formal-verifiability scope "expands at Layer 3 because the round-trip-byte-preservation invariant against a JSON parser becomes the testable contract." Post-architectural-correction, `display_safe` is no longer in the round-trip path; the claim is vestigial from the pre-correction routing. Documentation-level gap; does not block Layer 3 layer-gate close. Routes to the same follow-up PR as F2.
- **Finding 5** (Dim 1) — **Dismissed**. `bookmark_set_eq` free-standing-private-fn placement is the idiomatic Rust choice for an equality-variant helper that does not shadow `PartialEq`'s derived `eq`; the dedup-specific naming carries the scope-signal a method form would lose.
- **Finding 6** (Dim 4) — **Hallucinated**. `TagContainsControlChars(usize, String)` positional tuple shape is consistent with the Layer 2 `AttachTagError::NoMatch(String)` precedent + the Round 1 F5 Deferred disposition; re-surfacing it as a Round 2 finding would re-find an already-Deferred concern.

**Round 1 regression-check verdict:** **all Round 1 fixes hold**. The architectural-correction sub-decision (display_safe removal from export_json) is architecturally clean and structurally strengthens the purity-boundary discipline. The Round 1 F2 verification-architecture refresh is closed by the inline-fix; the Round 1 F3 round-trip byte-preservation is closed structurally via serde_json's native encoder; the Round 1 F4 dedup-complexity accepted-limit is closed at spec layer; the Round 1 F5 ImportError variant detail is deferred-to-follow-up per the operator-authorized routing disposition. Round 2 surfaces 2 new substantive issues: F2 (Open — rustdoc lag against the new TagContainsControlChars variant) + F3 (Raised-to-SO — manual-test performance-budget cross-source inconsistency against DESIGN.md) + F4 (Deferred — vestigial-spec-narrative for display_safe). All three new findings are documentation/spec-narrative-level residuals from the Round 1 fix-work; none gate Layer 3 layer-gate close.

The Round 2 finding count + classification pattern (2 Deferred + 1 Raised-to-SO + 1 Dismissed + 1 Hallucinated, all small-scope follow-up-level) is consistent with a clean Round-2 verification pass against fix-work that landed correctly: the substantive disposition is "Round 1 fixes hold; minor architectural-correction-leftover residuals remain at the documentation surface; the implementation itself is sound." The G-151 stop-trigger applies — Round 2 surfaces no new defects warranting a Round 3.

**Coordination:**

- **Finding 1** (Round 1 regression-check Resolved) — no coordination needed; documented for the audit trail closure on Round 1's 5 substantive findings.
- **Finding 2** (rustdoc `# Errors` lag) — routes to [Software Engineer](../SOFTWARE-ENGINEER-REVIEW.md) for the 6-line rustdoc edit; follow-up PR scope.
- **Finding 3** (performance budget cross-source inconsistency) — routes to [Solution Owner](../SOLUTION-OWNER-REVIEW.md) for the path-(a)/(b)/(c) trade-off ratification; cross-cuts to [Performance Engineer](../PERFORMANCE-ENGINEER-REVIEW.md) for the empirical validation of the 200 ms figure under path (a); cross-cuts to [Technical Writer](../TECHNICAL-WRITER-REVIEW.md) for the DESIGN.md table extension if path (a) is chosen.
- **Finding 4** (vestigial-spec-narrative for display_safe Verification architecture entry) — routes to [Solution Architect](../SOLUTION-ARCHITECT-REVIEW.md) (self-route — the original routing-pass-1 author bears the cleanup); same follow-up PR as F2.
- **Finding 5** (Dismissed) — no coordination.
- **Finding 6** (Hallucinated) — no coordination; recorded for audit completeness.

---

**Cost-tally:**

**Agent-self-verifiable (countable from this session's tool-call log):**

- **AI tool:** [Claude Code](https://claude.com/claude-code)
- **Model:** claude-opus-4-7
- **Execution method:** foreground sub-agent (Round 2 cold-session retry spawned by main session per the prior 600s watchdog stall)
- **Tool calls executed:** ~12 (Read x6, Bash x4, Write x1, Edit x1 — final file-write + minor edits)
- **Files read:** 7 across DESIGN.md (full), src/lib.rs (segments), src/main.rs (segments), manual-tests/layer-3.md (segments), vsdd-suite/review-log/2026-05-24-solution-architect.md (Review 1 full), per-domain Phase 4 routing appendices in `vsdd-suite/review-log/2026-05-24-<domain-slug>.md` (full), vsdd-suite/suite-development/suite-development.md (per-review-entry-shape section)
- **Files written/edited:** 1 at [`vsdd-suite/review-log/2026-05-24-solution-architect.md`](2026-05-24-solution-architect.md) (Review 2 entry appended after Review 1; Review 1 preserved verbatim)
- **Mechanical sweeps run:** 2 via Bash `grep`/`find` idioms (manual-test Step 15 budget table search + hook script locations + suite-development.md location)
- **Wall-clock anchors (Bash `date -u`):** session-start 2026-05-25T03:17:44Z → session-end *pending session-close*

**Operator-verifiable (requires `/cost` paste or plan-dashboard inspection):**

- **Raw tokens:** *pending operator `/cost` paste*
- **Cache-hit ratio:** *pending operator `/cost` paste*
- **Would-be API cost:** *pending operator `/cost` paste*
- **Rate-limit-window utilization:** *pending operator-dashboard check*

**Operator-confirmable (operator-declared or operator-clocked; should be re-confirmed per session):**

- **Plan tier:** *pending operator declaration*
- **Actual cost to operator:** $0 marginal IF on Max plan AND session did not trigger rate-limit; otherwise actual figure from operator

**Derived metric (currently unverifiable + ambiguously interpreted):**

- **Findings/100k tokens:** NOT COMPUTABLE — pending operator `/cost` paste

**Operator-action queue:** if cost-tally precision is load-bearing for cross-cycle calibration, operator runs `/cost` in this session and pastes the output here as an append-only addendum, replacing the *pending operator …* placeholders with measured values. Carry-forward per AIE F7 — this Round 2 retry inherits the same pending-operator-paste pattern as Round 1.

---

## Phase 4 routing — Round 1 (2026-05-25 02:00Z)

Per [`vsdd-suite/primers/4-feedback-integration.md`](../../../../vsdd-suite/primers/4-feedback-integration.md) § [manual] First-class fallback path. SO-decisions captured via main-session AskUserQuestion pass on 2026-05-25 across the cross-domain finding clusters. This appendix lists this domain's routable findings in the primer-4-canonical per-finding shape; cross-domain coordination signals live in each Round 1 finding's `**Coordination:**` line. Cross-cluster sequencing matrix lives in the commit message + the CHANGELOG slim-form entry that recorded this Phase 4 pass (refactored from a prior consolidated routing record per operator directive 2026-05-25 — the consolidated file was an anti-pattern; primer-4-canonical is per-domain appendices).

#### Finding `r5-f2` — DESIGN.md Verification architecture pure-fn enumeration stops at Layer 2; doesn't name export_json + import_json — ROUTED

**Cluster:** DESIGN.md verification-architecture refresh
**Route:** `Phase 1a+1b`
**Gate:** Pure-fn list extends to Layer 3 entries (export_json + import_json + display_safe scope); module-doc + spec narrative aligned; Validator: SA
**Sequencing:** Should land before Layer 3 gate close

#### Finding `r5-f3` — DESIGN.md L106 byte-preservation claim contradicted by display_safe Rust-syntax escape — ROUTED

**Cluster:** JSON-native escape design
**Route:** `Phase 2a → Phase 2b → Phase 1a+1b`
**Gate:** (see SE R1 F1 routing — same cluster; SA recommended path-(b) move-display_safe-to-render-boundary; operator chose the JSON-native-escape path + architectural-correction sub-decision at Phase 2b removed display_safe from export_json entirely)
**Sequencing:** Blocks Layer 3 layer-gate close

#### Finding `r5-f4` — import_json dedup O(N×(M+N)) via Vec::contains; ~10^8 comparisons at 10K × 10K — ROUTED

**Cluster:** dedup-complexity accepted-limit annotation
**Route:** `Phase 1a+1b`
**Gate:** DESIGN.md Performance budget gains Layer 3 dedup-complexity accepted-limit paragraph; Validator: PE + SA
**Sequencing:** Should land before Layer 3 gate close

#### Finding `r5-f5` — ImportError::SchemaMismatch(String) doesn't carry structured per-record data — ROUTED

**Cluster:** ImportError variant detail (deferred)
**Route:** `Phase 2b (low priority; deferred to follow-up)`
**Gate:** ImportError::SchemaMismatch extended to optionally carry record_index; Validator: SE
**Sequencing:** Deferred-to-follow-up-PR per G-150 over-investment guard


---

## Phase 4 routing — Round 2 (2026-05-25 07:30Z)

Per [`vsdd-suite/primers/4-feedback-integration.md`](../../../../vsdd-suite/primers/4-feedback-integration.md) § [manual] First-class fallback path. SO-decisions for substantive routings captured via main-session AskUserQuestion pass on 2026-05-25 (empty-string tag rejection consistency; tests/scaling.rs Phase 5 sentinel addition; Round 3 verification mini-cycle for the hallucination cluster). Verification evidence for `Hallucinated` dispositions: Round 3 PFE + QE + SE + UX cold-session re-spawn (per-domain Review N+1 entries authored 2026-05-25).

#### Finding `r2-f2` — import_json # Errors rustdoc missing TagContainsControlChars variant — VERIFY-PENDING

**Disposition:** Verify-pending
**Evidence:** Spot-check needed — rustdoc # Errors section may be missing the new variant. Low-risk omission; queued for follow-up grooming if confirmed.

#### Finding `r2-f3` — Performance budget cross-source inconsistency — RESOLVED-AT-E52E896

**Disposition:** Resolved-at-e52e896
**Evidence:** DESIGN.md § Performance budget table extended with `bm export` + `bm import` rows; structurally consistent with manual-tests/layer-3.md Step 15 hyperfine budgets.

#### Finding `r2-f4` — Verification architecture display_safe formal-verifiability claim — VERIFY-PENDING

**Disposition:** Verify-pending
**Evidence:** Claim about formal-verifiability scope at Layer 3 — review for accuracy in next SA round.

#### Finding `r2-f5` — bookmark_set_eq free-standing vs module-method architecture — PHASE 5

**Disposition:** Phase 5
**Evidence:** Architectural refactor (relocate to BookmarkStore impl block) deferred to Phase 5 cleanup; current free-standing fn is correct + tested.

#### Finding `r2-f6` — ImportError::TagContainsControlChars positional tuple vs named fields — DEFERRED-TO-FOLLOW-UP

**Disposition:** Deferred-to-follow-up
**Evidence:** Variant detail refactor (tuple → named struct fields) deferred per G-150 over-investment guard.
