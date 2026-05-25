# Software Engineer Review — bookmark-cli-manual — 2026-05-24

[Index](../SOFTWARE-ENGINEER-REVIEW.md)

---

## Review 1 — 2026-05-24 21:00Z

**Scope:** Cold-context [Software Engineer](../../../../vsdd-suite/domains/role/SOFTWARE-ENGINEER-REVIEW.md) Phase 3 IAR Round 1 against [bookmark-cli-manual](../../README.md) Layer 3 (`bm export` + `bm import`). Artifacts read in cold-reader order: [`README.md`](../../README.md), [`Cargo.toml`](../../Cargo.toml), [`src/lib.rs`](../../src/lib.rs) (Layer 3 additions `export_json` + `import_json` + `ImportError` + `MAX_STDIN_BYTES_DEFAULT`), [`src/main.rs`](../../src/main.rs) (Layer 3 additions `Cmd::Export` + `Cmd::Import` + `run_export` + `run_import` + `handle_parse_error` updates), [`tests/bookmarks.rs`](../../tests/bookmarks.rs) Layer 3 Red Gate block (`tests_export_*` + `tests_import_*`), [`tests/properties.rs`](../../tests/properties.rs), [`TODO.md`](../../TODO.md) § Layer 3, [`DESIGN.md`](../../DESIGN.md) § Behavioral contracts § `bm export` (Layer 3) + § `bm import` (Layer 3) + § Edge case catalog Layer 3 additions + § Interface definitions § Command surface (Layer 3 additions). Prior Layer 1 + Layer 2 SE rounds ([Review 1–5](2026-05-20-software-engineer.md)) read for shape, not finding re-litigation — Layer 3 is a fresh surface and no Layer 3 SE round has previously been logged. **Tested against:** commits `878d3b6` (Phase 2a Red Gate, 15 tests), `fd21900` (Phase 2b implementation), `78bd3cf` (Phase 2c refactor annotation — no code change) — the Layer 3 cycle.

**Layer:** 3
**Round:** 1
**Active domain set:** 11 role + 2 meta = 13 (capstone intent per [DESIGN.md § Project intent](../../DESIGN.md))
**Lens:** Standard SE dimensions emphasized: Dim 1 (Correctness — spec-vs-impl conformance under the sycophancy-check directive), Dim 2 (Error handling), Dim 3 (Naming and type precision — primitive obsession at the `&[String]` filter boundary), Dim 4 (Function and method design), Dim 8 (Defensive coding — invariants of the new untrusted-input surface), Dim 11 (Future-self maintainability). [Rust supplement](../../../../vsdd-suite/supplements/rust.md) § Software Engineering: `.unwrap()` discipline on user-facing paths, `?` propagation, error-type hierarchy. Dim 12 (Test seam attack surface) checked mechanically (`grep -E 'INTERNAL_|TEST_|_FORCE_|_BYPASS_|_SEAM|cfg\(any\(test|cfg\(debug_assertions|debug_assert!'` against `src/`) — no Layer-3-introduced seams beyond the existing `#[cfg(test)] mod tests` block. Documentation dimensions (13–17) and Performance dimensions (18–22) NOT applied here — [Technical Writer](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) + [Performance Engineer](../../../../vsdd-suite/domains/role/PERFORMANCE-ENGINEER-REVIEW.md) are both activated per [`DESIGN.md`](../../DESIGN.md) § Project intent, so documentation/performance finding ownership defers per the SE prompt's deferral rule.

**Session note:** Cold session. The reviewer did not author, design, or previously read the Layer 3 implementation; reading order followed the Phase 3 primer's cold-context discipline (primer → domain prompt → language supplement → governing standard → implementation files in author-natural order → DESIGN.md last → existing rounds for shape only, not finding re-litigation). The Layer 3 Phase 2a + Phase 2b + Phase 2c commits landed within the past hours via the two-commit canonical shape — the cycle is fresh and no fix-cycle findings are in scope for verification.

**Source:** `domain-raised` — every finding below was elicited by applying the SE dimensions from a cold seat against the Layer 3 spec text + Layer 3 implementation.

**Supplements applied:** [`rust.md`](../../../../vsdd-suite/supplements/rust.md) § Software Engineering — `.unwrap()` discipline, `?` propagation, error-type hierarchy, Clippy lint configuration verified against the post-Layer-3 `[lints]` table in [`Cargo.toml`](../../Cargo.toml).

**Assumption surfacing:** New Layer-3-load-bearing external-API assumptions: **(a)** `serde_json`'s default JSON-string serialization escapes control characters (U+0000–U+001F) as `\uHHHH` (lower-case hex, no braces) per [RFC 8259 § 7](https://datatracker.ietf.org/doc/html/rfc8259#section-7) — load-bearing for [Finding 1](#r1-f1) below; verified against the [serde_json 1.0 source](https://docs.rs/serde_json/1/serde_json/) `format_escaped_str_contents` implementation. **(b)** `serde_json::Value::String("\\u{HHHH}")` — when the in-memory String contains the literal 8-byte sequence `\u{HHHH}` (backslash + `u` + brace + 4 hex digits + brace) — serializes to the JSON text `"\\u{HHHH}"` (the backslash is JSON-escaped to `\\`; braces pass through as literal `{` `}`) and round-trips to the same 8-byte literal on parse, NOT to the codepoint `\u{HHHH}`. Load-bearing for [Finding 1](#r1-f1) below. **(c)** Derived `PartialEq` for `Bookmark { url, timestamp, tags }` performs order-sensitive equality on the `Vec<String>` tags field — `["rust", "go"]` and `["go", "rust"]` compare as **not equal**. Load-bearing for [Finding 2](#r1-f2) below; verified against [Rust reference § Derive](https://doc.rust-lang.org/reference/attributes/derive.html). **(d)** `std::io::Read::take(n).read_to_end(&mut buf)` reads AT MOST n bytes from the underlying stream; the cap-plus-one trick distinguishes "exactly at cap" from "exceeded" without uncapped buffering — verified against [`std::io::Take` docs](https://doc.rust-lang.org/std/io/struct.Take.html).

---

### Deferred

**Finding 1 — `export_json`'s `display_safe` pre-escape produces JSON text that round-trips as the 8-byte literal `\u{HHHH}`, NOT the original control byte; DESIGN.md round-trip preservation claim broken (Dim 1, Dim 8)**

<a id="r1-f1"></a>

**Owner:** software-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** quality-engineer

[`DESIGN.md`](../../DESIGN.md) § Behavioral contracts § `bm export` (Layer 3) names the round-trip guarantee explicitly:

> **Phase 2b implementation verification:** the implementation must confirm `display_safe`-wrapped strings remain JSON-valid (terminal escape sequences serialize as JSON-valid `` sequences so the round-trip `bm export | bm import` preserves the underlying bytes).

The spec's example escape form is `` — six bytes (`\`, `u`, `0`, `0`, `1`, `b`) — which is the **JSON native Unicode escape** per [RFC 8259 § 7](https://datatracker.ietf.org/doc/html/rfc8259#section-7). When a JSON parser reads `""`, it produces the single ESC byte (U+001B). When serde_json serializes a String containing the raw ESC byte back out, it emits `""` by default. Round-trip byte-preservation is the natural behavior of serde_json on raw control bytes.

The Layer 3 implementation diverges. [`src/lib.rs:687-705`](../../src/lib.rs) `display_safe`:

```rust
if c.is_control() || is_format_char(c) {
    let _ = write!(out, "\\u{{{:04x}}}", c as u32);
}
```

This produces the 8-byte ASCII literal `\u{001b}` (backslash + `u` + brace + `0` + `0` + `1` + `b` + brace) — the **Rust source-code escape syntax**, not the JSON Unicode escape. When [`src/lib.rs:454-485`](../../src/lib.rs) `export_json` wraps the URL through `display_safe` before constructing the `serde_json::Value::String`, the in-memory String holds those 8 literal bytes. `serde_json::to_string` then JSON-escapes the leading backslash to `\\`, emitting the on-disk JSON text `"\\u{001b}"` (10 bytes between the quotes). A JSON parser reading that text produces the 8-byte literal `\u{001b}` — NOT the ESC byte the original record contained.

**Concrete round-trip failure.** Store a bookmark whose URL contains ESC (`https://x.example/\x1bfoo`). Run `bm export | bm import` into a fresh destination. The destination store now contains a bookmark whose URL is `https://x.example/\u{001b}foo` — 8 characters longer than the source URL, with the ESC byte replaced by 8 printable bytes. The spec-named invariant — "the round-trip `bm export | bm import` preserves the underlying bytes" — does not hold for any record that triggers `display_safe`'s escape path (every URL/tag containing a control character per `is_control()` OR a format character per `is_format_char()`).

**Why this is the canonical SE-prompt sycophancy-check failure.** The implementation pre-mangles strings at the export boundary because the Security/Red-Team carry-forward (PR #46) said "route URLs + tags through `display_safe` before terminal-renderable emission." But the spec contracts a *round-trip* invariant that requires bytes survive a parse + re-serialize cycle. serde_json's default JSON-string serialization ALREADY satisfies the threat-model concern (control bytes get `\uHHHH`-escaped on the wire; a downstream terminal-renderer that JSON-parses the field gets back the raw ESC, which is the downstream renderer's responsibility — and a downstream renderer that prints the raw JSON text never sees the ESC at all because JSON's own escape covered it). The `display_safe` call at the export boundary does NOT add safety serde_json doesn't already provide; it actively breaks the round-trip claim.

**Test coverage gap that hides this.** [`tests/bookmarks.rs:1236-1284`](../../tests/bookmarks.rs) `tests_export_applies_display_safe_to_pathological_url` only asserts: (1) the on-disk bytes contain no raw ESC; (2) the output parses as JSON; (3) the URL field after JSON-parse does not contain raw ESC. It does **not** assert the spec-contracted round-trip: that `import_json(export_json(store))` reproduces `store`. [`tests/bookmarks.rs:1602-1689`](../../tests/bookmarks.rs) `tests_export_import_round_trip` uses URLs `https://item-{i}.example` (no control chars), so it exercises the happy-path where `display_safe` is a no-op. The Phase 5 round-trip proptest named in [`TODO.md`](../../TODO.md) § Layer 3 ("`import(export(X))` against a fresh destination store produces a store byte-equivalent to X") is not yet activated — would have caught this with high probability on the first generated control-char URL.

**The defensible fix.** Two viable shapes:

1. **Remove `display_safe` from `export_json`'s serialization path.** Trust serde_json's native JSON-string escaping for control bytes; the round-trip preserves bytes by RFC-8259 construction. The DESIGN.md spec text already names this shape ("terminal escape sequences serialize as JSON-valid `` sequences"). The Security/Red-Team carry-forward concern was about *human-rendered* terminal output, not JSON-pipeline output — a downstream JSON parser does the right thing.
2. **Change `display_safe`'s escape format from `\u{HHHH}` (Rust-syntax) to `\uHHHH` (JSON-syntax) at the export call site only.** Keeps the pre-escape but emits in JSON-native form. After JSON serialization the on-disk bytes become `"\\u001b"`; after parse the value is the literal `` (6-byte ASCII). On *re-export* the value re-escapes to `"\\u001b"` — stable. But the original ESC byte is still gone from the imported record — the round-trip preserves the *post-first-export* shape, not the original byte. This is a weaker contract than (1) and arguably still violates the spec's "preserves the underlying bytes" framing.

Path (1) is the spec-faithful answer and matches the existing pattern at [`src/main.rs:317`](../../src/main.rs) `run_list` where `display_safe(bm.url())` wraps the URL before `println!` — that's the **human-rendering** boundary where `display_safe` belongs. `export_json` is the **JSON-pipeline** boundary where serde_json's escape is the right tool.

**Classification:** Deferred — carried to the Layer 3 Round 1 fix cycle. The spec contract is unambiguous; the implementation does not honor it. Path (1) is a 3-line removal in `export_json` (drop the `display_safe(bm.url())` and `display_safe(t)` wraps; emit `bm.url()` and `t` directly via `serde_json::json!`); the regression test is the round-trip proptest that DESIGN.md § Phase 5 strategy already names as Layer 3 activation. Trigger: Layer 3 Round 1 fix cycle; auto-Backlog if Layer 3 closes without resolution.

---

**Finding 2 — `import_json` dedup uses order-sensitive `Bookmark::PartialEq` on the `tags: Vec<String>` field; spec says tag ordering is uncontracted — implementation creates duplicate-record bug when tag order differs across imports (Dim 1, Dim 3, Dim 8)**

<a id="r1-f2"></a>

**Owner:** software-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** quality-engineer

[`DESIGN.md`](../../DESIGN.md) § Storage format `tags` field section names the discipline:

> Within a single bookmark, `tags` is treated as a set: duplicates are not produced by the application (idempotent `bm tag`), but the JSON shape is an array. Ordering of the array is insertion order — first `bm tag` invocation's label appears first; subsequent labels append. **The spec does NOT contract on tag ordering, and tests should not assert order beyond "label X is present in the array."**

The `bm import` dedup contract per [`DESIGN.md`](../../DESIGN.md) § `bm import` (Layer 3):

> dedup runs on `url`+`timestamp`+`tags` exact-tuple-match

The Layer 3 implementation interprets "exact-tuple-match" via derived `PartialEq`. [`src/lib.rs:50-56`](../../src/lib.rs):

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Bookmark {
    url: String,
    timestamp: DateTime<Utc>,
    #[serde(default)]
    tags: Vec<String>,
}
```

`PartialEq` on `Vec<String>` is element-wise + **order-sensitive**: `["rust", "go"] != ["go", "rust"]`. The dedup loop at [`src/lib.rs:558-564`](../../src/lib.rs):

```rust
for new_bm in imported {
    if !self.bookmarks.contains(&new_bm) {
        self.bookmarks.push(new_bm);
        appended += 1;
    }
}
```

uses `contains(&new_bm)` which delegates to `PartialEq`. The conflict: the spec says tag order is uncontracted; the dedup says tag order must match byte-for-byte for dedup to fire.

**Concrete duplicate-row bug.** Source store S1 has a bookmark B with `tags: ["rust", "go"]` (tagged in that order at the CLI). Operator runs `bm export` on S1, gets JSON `{"bookmarks":[{"url":"...","timestamp":"...","tags":["rust","go"]}]}`. Pipes to a fresh destination D. D now contains B with `tags: ["rust", "go"]`. Operator then manually edits the export JSON (or uses `jq` to reorder for readability) so `tags` becomes `["go", "rust"]` (semantically the SAME bookmark per the spec's "treated as a set" framing). Operator pipes the edited JSON to D again. **Dedup does NOT fire** — `["rust", "go"] != ["go", "rust"]` — and D now contains TWO copies of B with reordered tag arrays. The spec promised "idempotence guard against operator double-pipe accidents"; the implementation only delivers idempotence against byte-equal stdin.

A second scenario where this fires without manual editing: a future Layer that adds `bm untag` (or any operation that produces tag-arrays in a different order from the original `bm tag` insertion order). The round-trip `bm export | bm import` against a store where tags were re-derived from a different insertion order produces duplicate records.

**Why this is also a Dim 3 primitive-obsession concern.** The dedup logic semantically operates on a `(Url, Timestamp, TagSet)` tuple, but the type system carries it as `(String, DateTime<Utc>, Vec<String>)`. The mismatch between the spec's "set" semantics and the type's "ordered sequence" semantics is invisible at the call site — `self.bookmarks.contains(&new_bm)` reads as obviously-correct dedup. A `TagSet(BTreeSet<String>)` newtype or a hand-rolled `Bookmark::matches(&Bookmark) -> bool` that compared tags via `iter().collect::<BTreeSet<_>>() == other.tags.iter().collect::<BTreeSet<_>>()` would close the hole at the type level. The bug surfaces only when the test surface exercises a tag-order divergence — neither [`tests_import_is_idempotent_on_exact_tuple_match`](../../tests/bookmarks.rs) (uses the same byte-equal payload twice) nor [`tests_export_import_round_trip`](../../tests/bookmarks.rs) (uses single-tag records `tags: ["rust"]` and `tags: ["go"]` separately) exercises it.

**The defensible fix.**

1. **Replace derived `PartialEq` with a hand-rolled `impl PartialEq` that compares tags set-equally.** Roughly: `self.url == other.url && self.timestamp == other.timestamp && tags_eq(&self.tags, &other.tags)` where `tags_eq` is a length-then-set comparison. Add a dedicated unit test that asserts `["rust", "go"] == ["go", "rust"]` for the PartialEq surface.
2. **Replace the dedup `contains` call with a tuple-key extractor.** Build a `HashSet<(String, DateTime<Utc>, BTreeSet<String>)>` of existing keys before the loop; check via `set.contains(&key_for(new_bm))`; insert as you push. O(K log T) per record instead of O(N) — also closes the linear-scan-quadratic concern that's a Performance Engineer-domain neighbor of this finding.

Path (1) is the simpler shape that closes the correctness gap without restructuring the dedup loop; (2) closes correctness + the implicit O(K·N) concern in one shape but is a larger change. Either fix is a small, contained edit to `import_json` + one new unit test asserting the set-equality contract.

**Classification:** Deferred — carried to the Layer 3 Round 1 fix cycle. The spec text is unambiguous ("tags is treated as a set") and the implementation contradicts it. The fix is local and self-contained; the regression test is a 5-line unit test. Trigger: Layer 3 Round 1 fix cycle; auto-Backlog if Layer 3 closes without resolution.

---

**Finding 3 — `import_json` doc comment claims the function "implements the `import(import(X)) == import(X)` idempotence property in `tests/properties.rs`", but no such property test exists in [`tests/properties.rs`](../../tests/properties.rs) — doc reference points at vapor (Dim 9 / Dim 11)**

<a id="r1-f3"></a>

**Owner:** software-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** quality-engineer

[`src/lib.rs:496-497`](../../src/lib.rs) doc comment on `import_json`:

> This implements the `import(import(X)) == import(X)` idempotence property in `tests/properties.rs`.

Grepped [`tests/properties.rs`](../../tests/properties.rs) for `import` / `export` / `round` — zero matches. The file contains exactly three properties (`tag_idempotence_property`, `tag_idempotence_property_no_match_path`, `filter_or_monotonicity_property`); none of them touches `import_json` or the round-trip. The proptest activation for the round-trip property is named in [`TODO.md`](../../TODO.md) § Layer 3 property-based testing as a Phase 5 closure item — currently not landed.

The doc comment makes a forward-looking claim as if it were already true. A future reader (human or AI) navigating the codebase to verify the dedup-idempotence contract follows the doc-comment pointer, lands at [`tests/properties.rs`](../../tests/properties.rs), greps for `import`, finds nothing, and is left with three options: (a) believe the doc and miss that the verification gap is real; (b) read the file end-to-end to confirm the absence (cost: ~200 lines); (c) git-log archaeology to find where the property used to live or was supposed to land. None of these are desirable.

**Why this is SE-territory (not TW-deferred).** The SE prompt defers `documentation finding ownership` to TW when TW is active — for **user-facing documentation** (README, DESIGN.md prose, doc-test quality). Inline source-code doc comments that make false claims about the test surface are different: they're a Dim 9 (self-documentation accuracy) + Dim 11 (future-self maintainability) concern internal to the implementation. The misclaim hides a verification gap the SE-domain dim set is built to surface. The deferral rule's intent is to avoid duplicate ownership of README/CHANGELOG/manual-test prose between SE and TW, not to immunize source-code lies from SE review.

**The defensible fix.** Two viable shapes:

1. **Land the missing proptest property** (`import(import(X)) == import(X)` idempotence + the export-import round-trip per `TODO.md` § Layer 3) and close this finding by making the doc-comment claim true. This is the spec-faithful answer — the proptest is named in the Layer 3 strategy and is a Phase 5 closure deliverable anyway.
2. **Amend the doc comment** to "This will implement … (deferred to Phase 5 Layer 3 proptest activation per `TODO.md` § Layer 3 property-based testing)." — preserves the audit trail without making a false claim about current state.

Path (1) is the better long-term answer; path (2) is the immediate-fix-without-blocking-on-Phase-5 shape. Either closes the misclaim.

**Classification:** Deferred — carried to the Layer 3 Round 1 fix cycle. The misclaim is observable today and survives `cargo test`. The fix is small (delete or amend two lines of doc comment; or land the proptest property when Phase 5 activates). Trigger: Layer 3 Round 1 fix cycle OR Layer 3 Phase 5 proptest activation (whichever lands first); auto-Backlog if Layer 3 closes without resolution.

---

**Finding 4 — `run_import` empty-stdin error path predates the size-cap check ordering invariant: if operator sets `--max-stdin-bytes 0` with empty stdin, the size-cap-of-zero is silently shadowed by the empty-stdin error message (Dim 1, Dim 2)**

<a id="r1-f4"></a>

**Owner:** software-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** quality-engineer

[`src/main.rs:418-442`](../../src/main.rs) `run_import` performs (in order): read stdin up to `cap+1` bytes; check `bytes.len() > max_stdin_bytes` → exit 1 "exceeded"; check `bytes.is_empty()` → exit 1 "empty"; UTF-8 decode → load → import. Consider the call `bm import --max-stdin-bytes 0` with empty stdin (`echo -n | bm import --max-stdin-bytes 0`):

- `bytes.len()` is `0`; `0 > 0` is `false` — the size-cap branch does not fire.
- `bytes.is_empty()` is `true` — the empty-stdin branch fires with exit 1 + `Error: stdin is empty; nothing to import.`

The user sees "stdin is empty" when they explicitly asked for "no bytes accepted." The error message attributes the failure to the wrong cause; the user could reasonably believe their pipe was broken when in fact their cap was the rejection.

A second related case: `bm import --max-stdin-bytes 0` with **any** stdin content (`echo x | bm import --max-stdin-bytes 0`) emits `Error: stdin exceeded maximum byte limit of 0.` — semantically correct but odd (the cap-of-zero degenerate case is a configuration that rejects all input). The spec is silent on whether `--max-stdin-bytes 0` is even a permitted value — there's no documented lower bound on the flag in [`DESIGN.md`](../../DESIGN.md) § Command surface (Layer 3 additions) or AC 27.

**Why this is correctness-class (Dim 1) and not just UX-polish.** The spec contracts a specific error message for each of the four `bm import` failure modes (empty stdin / invalid JSON / schema mismatch / size-cap). When two failure modes are simultaneously satisfied and the implementation routes through one branch silently, the operator-visible behavior diverges from what either contract alone names. A test harness asserting "user supplied `--max-stdin-bytes 0` should see `Error: stdin exceeded maximum byte limit of 0.`" would fail; a test asserting "empty stdin should see `Error: stdin is empty.`" would pass for the same invocation. The behavior is ambiguous.

**Defense gap as well (Dim 8).** The `--max-stdin-bytes` flag has no validation against the lower bound. clap accepts `0` (the type is `usize`; clap-derive does not infer a `> 0` constraint). A negative value is rejected by `usize` parsing. There's no spec-level statement that a zero cap is meaningful — `--max-stdin-bytes 0` is effectively "reject all imports" which the user could express more clearly by simply not running `bm import`. The flag's documented purpose (per DESIGN.md § `bm import`) is the operator-override for legitimately-larger imports; zero is outside that purpose.

**The defensible fix.** Two viable shapes:

1. **Reorder the checks**: empty-stdin first, then size-cap. Closes the message-attribution ambiguity; preserves the existing `--max-stdin-bytes 0` degenerate-but-accepted behavior. Two-line swap.
2. **Validate the flag at parse time** to reject `--max-stdin-bytes 0` (and possibly `--max-stdin-bytes < some_minimum` like `< 16` for "the smallest possible valid storage-format JSON `{"bookmarks":[]}` is 16 bytes"). Surface as `error: invalid value '0' for '--max-stdin-bytes <N>'` (clap exit 64). Closes the degenerate-case ambiguity at the spec layer.

Path (1) alone is sufficient for the immediate ambiguity. Path (2) + the order-check is the spec-tightening shape; raising to SO would be appropriate for adding a lower-bound clause to AC 27 / DESIGN.md § Command surface.

**Classification:** Deferred — carried to the Layer 3 Round 1 fix cycle. Minor but real; the ordering swap is a two-line fix and closes the message-attribution ambiguity. The spec-tightening half (lower-bound validation) is a candidate the SO can decide whether to absorb at Layer 3 or defer. Trigger: Layer 3 Round 1 fix cycle; auto-Backlog if Layer 3 closes without resolution.

---

### Raised to SO

*(none)*

---

### Dismissed

*(none)*

---

### Resolved

*(none)*

---

### Hallucinated

*(none — every finding above cites a specific [`src/lib.rs`](../../src/lib.rs) / [`src/main.rs`](../../src/main.rs) / [`tests/bookmarks.rs`](../../tests/bookmarks.rs) line + a specific [`DESIGN.md`](../../DESIGN.md) clause the implementation diverges from. The cold-reader-against-spec discipline produced 4 substantive findings against the Layer 3 surface, all reproducible against the current state at commits `878d3b6` / `fd21900` / `78bd3cf`.)*

---

### Summary

4 findings raised in-session — all SE-owned, all classified Deferred at session-close (carried to the Layer 3 Round 1 fix cycle with named triggers), all small-and-local fixes. The findings cluster on the gap between the documented spec contract and the implementation's actual behavior at the round-trip boundary ([Finding 1](#r1-f1) `display_safe` pre-mangle breaks the spec-named round-trip preservation), the dedup boundary ([Finding 2](#r1-f2) order-sensitive Vec equality contradicts the spec's "tags is treated as a set" framing), the doc-comment audit trail ([Finding 3](#r1-f3) reference to a non-existent proptest), and the input-validation ordering ([Finding 4](#r1-f4) empty-stdin vs size-cap shadowing). The implementation is broadly idiomatic — `?` propagation is clean; the two-phase validate-then-mutate shape in `import_json` honors the atomicity contract at AC 26; the cap-plus-one stdin-read trick correctly distinguishes "at cap" from "over cap" without unbounded buffering; the `[lints.clippy]` floor at [`Cargo.toml:83-98`](../../Cargo.toml) carries the Rust supplement's deny set forward to Layer 3 with no relaxation. The four findings above are the substantive divergences from the spec contract.

Per the [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) § Round triggers (G-131 continue trigger): this round produced 4 new real findings, so SE Round 2 against Layer 3 is mandatory after these findings are dispositioned — the Round 2 cold pass verifies the fixes held and looks for adjacent defects the fixes may have created.

**Coordination:**

- [Finding 1](#r1-f1) (broken round-trip) coordinates with [Security](../../../../vsdd-suite/domains/role/SECURITY-REVIEW.md) Dim 1 / [Red Team](../../../../vsdd-suite/domains/role/RED-TEAM-REVIEW.md) (the carry-forward closure at PR #46 that originated the `display_safe`-at-export discipline) — the spec-faithful fix removes `display_safe` from `export_json` because serde_json's native JSON escape already covers the threat-model concern at the JSON-pipeline boundary. Security and Red Team should validate that the fix does not regress the carry-forward intent. Also coordinates with [QUALITY-ENGINEER-REVIEW.md](../QUALITY-ENGINEER-REVIEW.md) for the regression-test gap — the Phase 5 round-trip proptest named in [`TODO.md`](../../TODO.md) § Layer 3 property-based testing would catch this with high probability on the first generated control-char URL; QE should land it as the regression pin.
- [Finding 2](#r1-f2) (order-sensitive dedup) coordinates with [QUALITY-ENGINEER-REVIEW.md](../QUALITY-ENGINEER-REVIEW.md) — the regression test is a 5-line unit test on PartialEq + the import-with-reordered-tags integration test. Also coordinates with [Performance Engineer](../../../../vsdd-suite/domains/role/PERFORMANCE-ENGINEER-REVIEW.md) — the recommended fix Path (2) (HashSet-based dedup) closes correctness + the implicit O(K·N) linear-scan concern that PE may surface independently against the 10K scale-ceiling.
- [Finding 3](#r1-f3) (doc-comment misclaim) coordinates with [QUALITY-ENGINEER-REVIEW.md](../QUALITY-ENGINEER-REVIEW.md) — the long-term answer is to land the missing proptest property the doc references, which is QE-owned activation territory per [`TODO.md`](../../TODO.md) § Layer 3 property-based testing. Also routes a non-SE-owned observation to [Technical Writer](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md): the inline doc-comment misclaim pattern at `import_json` is the SE-internal mirror of the user-facing doc-accuracy concern TW Dim 14 surfaces — TW may wish to apply a Layer-3-wide sweep for similar forward-looking doc-comment claims that reference unmaintained test files.
- [Finding 4](#r1-f4) (empty-stdin vs size-cap ordering) coordinates with [UX](../../../../vsdd-suite/domains/role/UX-REVIEW.md) — the error-message-attribution ambiguity is a UX-adjacent concern; UX may wish to validate the fix against the "error message reads accurately" floor. The spec-tightening half (Path 2 — `--max-stdin-bytes` lower-bound validation) would route to [Solution Owner](../../../../vsdd-suite/domains/role/SOLUTION-OWNER-REVIEW.md) for the AC 27 amendment; the inline-fix half (Path 1 — reorder the checks) is purely SE-owned.

---
