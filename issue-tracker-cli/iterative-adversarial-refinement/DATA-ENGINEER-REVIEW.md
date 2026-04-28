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

---

---

## Review 2 — 2026-04-27 22:00Z

**Scope:** Layer 1 test data access patterns — how `tracker.json` is read and asserted in `tests/layer1.rs`. Evaluating storage schema assumptions, field coverage, and schema correctness.

**Session note:** In-session with all other Layer 1 domain reviews. Acknowledged quality tradeoff.

---

### Raised to SO

**Finding 1 — Integration tests assumed a top-level array; DESIGN.md specifies a wrapped object (Dim 1 — Schema correctness)**

DESIGN.md specified `{"issues": [Issue]}` as the storage format. The integration tests access `tracker.json` using `v[0]["field"]` — correct only for a top-level array. A correct implementation following the wrapped format would cause these tests to silently compare against `null`.

This finding was identified jointly with QE Review 2 Finding 1. See QE Review 2 for the full test-level analysis.

From a data-layer perspective, a top-level array is the simpler and more idiomatic representation: deserialization becomes `serde_json::from_str::<Vec<Issue>>(&raw)` with no wrapper struct. The `"issues"` key adds no semantic content — there are no other top-level keys in the format and no schema evolution benefit to the envelope. Consistent with SA Review 1's complexity-budget principle.

**Classification: Raised to SO Review 7.** DESIGN.md is controlled by SO. DE proposes the top-level array as the preferred resolution. See SO Review 7 Finding 1 for the decision.

---

### Dismissed

**Finding 2 — `create_stores_issue_in_json` does not assert `created_at` or `updated_at` field presence (Dim 2 — Field completeness)**

The test asserts `title`, `id`, `status`, `priority` but not the timestamp fields.

**Classification:** Dismissed. `create_timestamps_equal_on_fresh_issue` is a dedicated test that explicitly asserts both timestamp fields are present and equal. Repeating timestamp assertions in `create_stores_issue_in_json` would be redundant. The field coverage across the full test suite is complete.

---

**Finding 3 — `create_stores_issue_in_json` does not assert `labels` field (Dim 2)**

The test asserts four fields but not the `labels` array.

**Classification:** Dismissed. Labels are Layer 4 scope. The `create_stores_issue_in_json` test is a Layer 1 smoke test for the core stored-field contract. Asserting `v[0]["labels"] == json!([])` would be accurate but cross-layer — it would require the implementation to serialize an empty `labels: Vec<String>` correctly, which is tested in Layer 4. Layer 1's contract is: the issue is stored with the correct non-optional fields. Labels can wait for their layer.

---

**Finding 4 — No test asserting `tracker.json` contains valid JSON after a create (Dim 3 — Data integrity)**

The test suite reads `tracker.json` via `serde_json::from_str::<serde_json::Value>` which would `unwrap()` panic if the JSON is malformed. This is an implicit validity assertion.

**Classification:** Dismissed. The `serde_json::from_str(&raw).unwrap()` in each test that reads `tracker.json` is an implicit "valid JSON" assertion — if the implementation writes malformed JSON, the test panics and fails. The intent is to test field values, not JSON validity specifically, but validity is verified as a side effect. No additional test is needed.

---

### Open

*(none)*

---

### Summary

One real finding raised to SO (storage schema mismatch — cross-reference with QE Review 2). Three dismissed findings. The data layer test coverage for Layer 1 is complete pending SO's resolution of the schema finding. Key Layer 6 requirement (`description` absent-not-null) is tracked in the existing Layer 6 Red Gate test plan.

---

---

## Review 3 — 2026-04-28 05:30Z

**Scope:** Layer 1 implementation — `src/lib.rs`, `Cargo.toml`. Evaluating data model correctness, post-deserialization validation, serialization behavior, and schema alignment.

**Session note:** In-session with Layer 1 IAR suite. Acknowledged quality tradeoff.

---

### Resolved

**Finding 1 — Post-deserialization validation absent (Dim 2 — Domain validation)**

Data Engineer Review 1 Finding 1 (cross-referenced with Security Review 1) required that the implementation validate each deserialized issue's domain values. The initial Layer 1 implementation did not implement this validation — `load_issues` returned `Ok(issues)` without checking field values against domain constraints.

A `tracker.json` with `"status": "flying"` would silently produce an `Issue` struct with an invalid `status` field, which would then be silently sorted to the bottom of the list (via `usize::MAX` in `priority_rank`) — invisible to the user. An `"id": 0` issue would violate the ID-is-positive-integer invariant.

**Resolution:** `issue_fields_are_valid()` added in `lib.rs`. Validates: `id > 0`, `!title.trim().is_empty()`, `status ∈ {"open", "in-progress", "done"}`, `priority ∈ {"low", "medium", "high"}`. Called in `load_issues` after successful deserialization; any failing issue triggers the corrupt-data error path. Constant `VALID_STATUSES` and `VALID_PRIORITIES` arrays are defined for readability and future extensibility. Cross-referenced: Security Review 3, Red Team Review 2.

---

### Dismissed

**Finding 2 — `description` field serialization is correct (Dim 7 — Review 1 requirement)**

Data Engineer Review 1 Finding 2 required `#[serde(skip_serializing_if = "Option::is_none")]` on the `description` field. Verified present in `lib.rs`:

```rust
#[serde(skip_serializing_if = "Option::is_none")]
pub description: Option<String>,
```

Layer 1 never writes a description (all creates use `description: None`), so this attribute does not affect Layer 1 output. Its presence ensures that when Layer 6 implements description storage, the correct serialization behavior is in place from the start.

**Classification:** Dismissed. Requirement satisfied.

---

**Finding 3 — Storage format is a top-level array as approved by SO Review 7 (Dim 1)**

`load_issues` uses `serde_json::from_str::<Vec<Issue>>()` — correct for the `[Issue]` top-level array format. SO Review 7 approved the format change. The implementation matches the spec.

**Classification:** Dismissed. No action required.

---

**Finding 4 — `labels` serializes as `[]` (empty array), not absent (Dim 7)**

`Issue` has `pub labels: Vec<String>` with no `skip_serializing_if`. A freshly-created issue will serialize as `"labels": []` rather than omitting the `labels` key. DESIGN.md Data Model shows `"labels": [String]` — an empty array is within spec (the type is `[String]`, which includes the empty case). DESIGN.md does not require omitting the key when labels are empty (unlike `description`, which explicitly requires key omission when absent).

**Classification:** Dismissed. `"labels": []` is the correct serialization. The `create_first_issue_unchanged_after_second_create` test asserts `v[0]["labels"] == serde_json::json!([])` — confirming this is the expected representation.

---

### Open

*(none)*

---

### Summary

One real finding resolved: post-deserialization domain validation now implemented. Three dismissed findings. The data layer is now specification-compliant: the top-level array format is correctly deserialized, domain values are validated after deserialization, `description` is absent-not-null, and `labels` serializes as an empty array. No open items.
