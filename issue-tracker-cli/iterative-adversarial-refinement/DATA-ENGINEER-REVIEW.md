# Data Engineer Review Log

This review is part of the [Iterative Adversarial Refinement (IAR)](README.md) suite. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

The purpose of this review is to evaluate the data layer: modeling, validation, storage, schema evolution, and serialization. Pre-implementation: reviewing the data model and storage specification in DESIGN.md.

**Language supplement applied:** `lang/rust.md` (Data Engineering section).

---

## Review 1 — 2026-04-27 21:00Z

**Scope:** `DESIGN.md` Data Model and Storage sections. No implementation exists.

**Session note:** In-session with all other domain reviews. Acknowledged quality tradeoff.

---

### Resolved

**Finding 1 — Post-deserialization validation not specified (Dim 2)**

`#[derive(Deserialize)]` successfully parses structurally valid JSON into the Issue struct regardless of domain validity. A `tracker.json` with `"status": "flying"`, `"id": 0`, or `"title": ""` would deserialize without error and silently produce invalid domain state. The spec defined behavior for malformed JSON but not for semantically invalid JSON.

Cross-referenced: Security Finding 1, Red Team Finding 1.

**Resolution:** Added to DESIGN.md Storage edge cases: invalid domain values in otherwise-valid JSON → corrupt-data error → exit 1. The implementation must validate each deserialized issue's field values against domain constraints (non-empty title, valid status enum, valid priority enum, positive ID) after deserialization succeeds. Any violation triggers the same error path as malformed JSON.

---

**Finding 2 — `description: Option<String>` absent vs. null behavior not specified for serialization (Dim 7)**

DESIGN.md Data Model: `"description": Option<String>, // absent if not provided.` The spec says "absent" but `#[derive(Serialize)]` on an `Option<String>` field serializes `None` as `null` by default, not as a missing key. A `tracker.json` written by a naive implementation would contain `"description": null`, which differs from "absent." This has schema evolution implications: a future reader expecting the field to be absent would need to handle both `null` and missing.

**Resolution:** Added note to the DESIGN.md Data Model: "Absent means the JSON key is omitted, not serialized as null. Implementations must omit the key when the value is None." The Rust implementation must use `#[serde(skip_serializing_if = "Option::is_none")]` on the description field to produce the correct output.

---

### Dismissed

**Finding 3 — No schema migration strategy for future field additions (Dim 3)**

The spec handles forward-compatibility (new fields added in future): unknown fields are ignored on deserialization. But backward-compatibility (old data missing a field that is now required) is not addressed. If a future version adds a required field, old `tracker.json` data won't have it.

**Classification:** Dismissed. This is a Phase 1 portfolio project. No version migration is planned. The forward-compatibility story (unknown fields ignored) is correctly specified. For backward-compatibility, Rust's `#[serde(default)]` handles missing optional fields by filling the field-type's default. When new fields are added in a future version of this tool, the implementation should use `#[serde(default)]` to provide a sensible default for old data. This is an implementation guidance note, not a current spec gap — adding a schema migration requirement to Phase 1 would be over-engineering (see SA Review 1). **Backlogged** as a future consideration.

---

**Finding 4 — No explicit data volume limit (Dim 11)**

The spec does not define a maximum number of issues. Full JSON read-and-write on every mutation is O(n) in both time and memory. For a personal tool with tens to hundreds of issues, this is inconsequential — JSON parsing of hundreds of short objects is well under 1ms on modern hardware. At tens of thousands of issues the pattern would still work but becomes slower.

**Classification:** Dismissed. The tool is for a single developer tracking their own project issues. Real-world use is bounded well below any problematic threshold. No explicit limit is needed for Phase 1. If the tool is ever used at scale, this is the first architectural concern to revisit.

---

**Finding 5 — Timestamps stored as strings, not as typed date values (Dim 7)**

Timestamps (`created_at`, `updated_at`) are stored as ISO 8601 UTC strings rather than Unix epoch integers. String timestamps have slightly higher storage cost and require parsing for comparison, but are human-readable in the JSON file.

**Classification:** Dismissed. Human-readability of `tracker.json` is a feature — the spec says "verify `tracker.json` is valid JSON after each mutation (open it in a text editor)" in the manual testing checklist. A human reading the file should be able to understand the timestamps without a conversion tool. The performance cost of string timestamp parsing is negligible for this use case. The spec is correct.

---

### Open

*(none)*

---

### Summary

Two real findings, both resolved via DESIGN.md updates: post-deserialization validation gap (cross-referenced with Security and Red Team), and absent-vs-null serialization behavior for `description`. Three findings dismissed. The data model is well-specified for the project's scope. The key implementation requirements are now explicit: (1) validate domain values after deserialization, (2) use `skip_serializing_if = "Option::is_none"` for the description field.

**Coordination:** Finding 1 cross-referenced in Security log (dim 2) and Red Team log. Finding 2 should be noted in the Layer 1 Red Gate — a test that reads `tracker.json` and asserts the `description` key is absent (not null) when no description was provided. Added to TODO.md Layer 6: `create_without_description_has_no_field_in_json` already covers this.
