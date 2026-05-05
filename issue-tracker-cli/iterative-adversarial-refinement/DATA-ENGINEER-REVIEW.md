# Data Engineer Review Log

This review is part of the [Iterative Adversarial Refinement (IAR)](README.md) suite. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

**Reviewer role: Data Engineer** (Data Engineer / Database Engineer / Data Platform Engineer)

The purpose of this review is to evaluate the data layer: modeling, validation, storage, schema evolution, and serialization.

**Language supplement applied:** `lang/rust.md` (Data Engineering section).

**Sycophancy check:** An agent that designed the data model will not question schema decisions — it evaluates whether the implementation matches the schema it chose, not whether the schema was the right choice. The adversary must ask whether each structural decision (field types, normalization approach, storage mechanism, validation boundaries) serves the domain correctly. The most dangerous data bug is not a validation gap — it is a schema that encodes the wrong model of the domain and silently corrupts every downstream operation.

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

**Classification:** Dismissed. This is a Phase 1 portfolio project. No version migration is planned. The forward-compatibility story (unknown fields ignored) is correctly specified. For backward-compatibility, Rust's `#[serde(default)]` handles missing optional fields by filling the field-type's default. When new fields are added in a future version of this tool, the implementation should use `#[serde(default)]` to provide a sensible default for old data. This is an implementation guidance note, not a current spec gap — adding a schema migration requirement to Phase 1 would be over-engineering (see [SOLUTION-ARCHITECT-REVIEW.md](SOLUTION-ARCHITECT-REVIEW.md) Review 1). Tracked informally as a future consideration; no SO backlog item raised at Layer 1.

---

**Finding 4 — No explicit data volume limit (Dim 11)**

The spec does not define a maximum number of issues. Full JSON read-and-write on every mutation is O(n) in both time and memory. For a personal tool with tens to hundreds of issues, this is inconsequential — JSON parsing of hundreds of short objects is well under 1ms on modern hardware. At tens of thousands of issues the pattern would still work but becomes slower.

**Classification:** Dismissed. The tool is for a single developer tracking their own project issues. Real-world use is bounded well below any problematic threshold. No explicit limit is needed for Phase 1. If the tool is ever used at scale, this is the first architectural concern to revisit.

---

**Finding 5 — Timestamps stored as strings, not as typed date values (Dim 7)**

Timestamps (`created_at`, `updated_at`) are stored as ISO 8601 UTC strings rather than Unix epoch integers. String timestamps have slightly higher storage cost and require parsing for comparison, but are human-readable in the JSON file.

**Classification:** Dismissed. Human-readability of `tracker.json` is a feature — the spec says "verify `tracker.json` is valid JSON after each mutation (open it in a text editor)" in the manual testing checklist. A human reading the file should be able to understand the timestamps without a conversion tool. The performance cost of string timestamp parsing is negligible for this use case. The spec is correct.

---

### Hallucinated

*(none)*

---

### Open

*(none)*

---

### Summary

Two real findings, both resolved via DESIGN.md updates: post-deserialization validation gap (cross-referenced with Security and Red Team), and absent-vs-null serialization behavior for `description`. Three findings dismissed. The data model is well-specified for the project's scope. The key implementation requirements are now explicit: (1) validate domain values after deserialization, (2) use `skip_serializing_if = "Option::is_none"` for the description field.

**Coordination:** Finding 1 cross-referenced in [SECURITY-REVIEW.md](SECURITY-REVIEW.md) (dim 2) and [RED-TEAM-REVIEW.md](RED-TEAM-REVIEW.md). Finding 2 should be noted in the Layer 1 Red Gate — a test that reads `tracker.json` and asserts the `description` key is absent (not null) when no description was provided. Added to TODO.md Layer 6: `create_without_description_has_no_field_in_json` already covers this.

---

---

## Review 2 — 2026-04-27 22:00Z

**Scope:** Layer 1 test data access patterns — how `tracker.json` is read and asserted in `tests/layer1.rs`. Evaluating storage schema assumptions, field coverage, and schema correctness.

**Session note:** In-session with all other Layer 1 domain reviews. Acknowledged quality tradeoff.

---

### Raised to SO

**Finding 1 — Integration tests assumed a top-level array; DESIGN.md specifies a wrapped object (Dim 1 — Schema correctness)**

DESIGN.md specified `{"issues": [Issue]}` as the storage format. The integration tests access `tracker.json` using `v[0]["field"]` — correct only for a top-level array. A correct implementation following the wrapped format would cause these tests to silently compare against `null`.

This finding was identified jointly with [QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md) Review 2 Finding 1. See QE Review 2 for the full test-level analysis.

From a data-layer perspective, a top-level array is the simpler and more idiomatic representation: deserialization becomes `serde_json::from_str::<Vec<Issue>>(&raw)` with no wrapper struct. The `"issues"` key adds no semantic content — there are no other top-level keys in the format and no schema evolution benefit to the envelope. Consistent with [SOLUTION-ARCHITECT-REVIEW.md](SOLUTION-ARCHITECT-REVIEW.md) Review 1's complexity-budget principle.

**Classification:** Raised to SO. DESIGN.md is controlled by SO. DE proposes the top-level array as the preferred resolution. See [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) Review 7 Finding 1 for the decision.

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

### Hallucinated

*(none)*

---

### Summary

One real finding raised to SO (storage schema mismatch — cross-reference with [QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md) Review 2). Three dismissed findings. The data layer test coverage for Layer 1 is complete pending SO's resolution of the schema finding. Key Layer 6 requirement (`description` absent-not-null) is tracked in the existing Layer 6 Red Gate test plan.

**Coordination:** Finding 1 raised to [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) Review 7 Finding 1.

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

**Resolution:** `issue_fields_are_valid()` added in `lib.rs`. Validates: `id > 0`, `!title.trim().is_empty()`, `status ∈ {"open", "in-progress", "done"}`, `priority ∈ {"low", "medium", "high"}`. Called in `load_issues` after successful deserialization; any failing issue triggers the corrupt-data error path. Constant `VALID_STATUSES` and `VALID_PRIORITIES` arrays are defined for readability and future extensibility. Cross-referenced: [SECURITY-REVIEW.md](SECURITY-REVIEW.md) Review 3, [RED-TEAM-REVIEW.md](RED-TEAM-REVIEW.md) Review 2.

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

**Finding 3 — Storage format is a top-level array as approved by [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) Review 7 (Dim 1)**

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

### Hallucinated

*(none)*

---

### Summary

One real finding resolved: post-deserialization domain validation now implemented. Three dismissed findings. The data layer is now specification-compliant: the top-level array format is correctly deserialized, domain values are validated after deserialization, `description` is absent-not-null, and `labels` serializes as an empty array. No open items.

**Coordination:** Finding 1 resolved jointly with [SECURITY-REVIEW.md](SECURITY-REVIEW.md) Review 3 and [RED-TEAM-REVIEW.md](RED-TEAM-REVIEW.md) Review 2.

---

---

## Review 4 — 2026-04-30 00:00Z

**Scope:** Layer 1 gate closure pass — no code or schema changes since Review 3.

**Session note:** In-session with all other domain reviews. Acknowledged quality tradeoff.

---

### Dismissed

*(none)*

### Hallucinated

*(none)*

---

### Open

*(none)*

---

### Summary

No Data Engineer findings. Schema correct, validation in place, serialization spec-compliant. MVR reached for Layer 1.

**Coordination:** *(none)*

---

---

## Review 5 — 2026-05-01 00:00Z

**Scope:** Layer 2 implementation — data model mutations: `status` field updated by `cmd_status`, `updated_at` refreshed, `created_at` unchanged. Schema evolution: no new fields.

**Session note:** In-session with full Layer 2 IAR suite. Acknowledged quality tradeoff.

---

### Dismissed

**Finding 1 — Status stored as lowercase string; consistent with `issue_fields_are_valid` (Dim 2 — Domain validation)**

`parse_status` now uses `VALID_STATUSES` (after SA Review 6 fix) to normalize and validate the input. The stored value is always one of `{"open", "in-progress", "done"}`, which are exactly the values in `VALID_STATUSES`. `issue_fields_are_valid` uses the same constant. Single source of truth restored. ✓

**Classification:** Dismissed. [SOLUTION-ARCHITECT-REVIEW.md](SOLUTION-ARCHITECT-REVIEW.md) Review 6 resolved the two-source-of-truth concern from the data layer's perspective as well.

---

**Finding 2 — `updated_at` format matches `created_at` (Dim 7 — Serialization consistency)**

`cmd_status` calls `current_timestamp()` for `updated_at`, identical to how `cmd_create` sets both timestamps. The ISO 8601 UTC format at second precision is consistent across all timestamp writes. ✓

**Classification:** Dismissed.

---

**Finding 3 — `created_at` not mutated by `cmd_status` (Dim 2 — Field invariant)**

`cmd_status` modifies only `issue.status` and `issue.updated_at`. `issue.created_at` is not referenced. The `created_at` field invariant (never changes after creation) is structurally enforced. ✓

**Classification:** Dismissed.

---

**Finding 4 — No schema changes; unknown fields still ignored on deserialization (Dim 3 — Forward compatibility)**

`Issue` struct is unchanged. `serde_json`'s default deserialization ignores unknown fields. Forward-compatibility behavior unchanged. ✓

**Classification:** Dismissed.

---

### Hallucinated

*(none)*

---

### Open

*(none)*

---

### Summary

No data engineering findings. Schema unchanged. Status mutation is correctly validated and stored. Timestamp consistency maintained. Single source of truth for valid status values restored by [SOLUTION-ARCHITECT-REVIEW.md](SOLUTION-ARCHITECT-REVIEW.md) Review 6. MVR reached for Layer 2.

**Coordination:** *(none)*

---

---

## Review 6 — 2026-05-04

**Scope:** Layer 3 implementation cold-session pass — `src/lib.rs`, `src/main.rs`, `tests/layer1.rs`, `tests/layer2.rs`, `tests/layer3.rs`, `tracker.json`. Evaluating data model invariants enforced at load time, schema evolution behavior across read-modify-write cycles, and timestamp data integrity.

**Session note:** Cold session per primer; parallel batch run with other domains. Reviewer did not build the project. Adversarial posture: stored data is untrusted; `tracker.json` could have been hand-edited.

---

### Open

**Finding 1 — `issue_fields_are_valid` does not validate timestamp format or the `updated_at >= created_at` invariant (Dim 4 — Data integrity invariants)**

`src/lib.rs:57-62` validates `id > 0`, non-empty title, status membership, priority membership. It does not validate the `created_at` / `updated_at` fields at all. They are typed `String` and accepted as-is on load.

DESIGN.md Data Model field invariants explicitly state:
- `created_at` ISO 8601 UTC, second precision, e.g. `"2026-04-27T14:00:00Z"`
- `updated_at` ISO 8601 UTC, second precision; **always >= `created_at`**

A hand-edited `tracker.json` with `"created_at": "yesterday"`, `"updated_at": ""`, or `"updated_at": "2025-01-01T00:00:00Z"` against `"created_at": "2026-05-01T00:00:00Z"` deserializes successfully and passes `issue_fields_are_valid`. The data layer silently accepts data that violates a documented invariant. Downstream code (`show` in Layer 6) will print whatever string is there; sort-by-timestamp functionality (not currently implemented but plausibly future) would behave incorrectly.

DESIGN.md Edge Cases / Storage lists `"id": 0`, `"title": ""`, `"status": "flying"`, `"priority": ""` as triggering the corrupt-data error. By parallel reasoning, a non-ISO-8601 timestamp string or a `updated_at < created_at` violation should trigger the same path — but the spec does not list timestamps in the corrupt-data examples. This is partially a spec gap and partially an implementation gap: even under the conservative reading of DESIGN.md, a `String` value documented as "ISO 8601 UTC, second precision" needs format validation to honor the contract.

Recommendation: extend `issue_fields_are_valid` to (a) parse both timestamps with `chrono` (e.g., `DateTime::parse_from_rfc3339` with a UTC `Z` suffix check) and (b) verify `updated_at >= created_at`. If timestamp validation is judged out-of-scope for the implementation, raise the question to SO to either (1) add timestamp-format and ordering examples to the corrupt-data Edge Cases list, or (2) explicitly weaken the field invariant to "best effort, format not enforced on load." Either resolution makes the contract explicit. The current state is a silent invariant violation.

**Classification:** Open. Cross-reference [SECURITY-REVIEW.md](SECURITY-REVIEW.md) (untrusted-input boundary) and [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) (DESIGN.md amendment to clarify corrupt-data scope for timestamps).

---

**Finding 2 — `issue_fields_are_valid` does not enforce ID uniqueness on load (Dim 4 — Data integrity invariants)**

DESIGN.md Field invariants: "`id` is unique across all issues and never reused." `load_issues` (`src/lib.rs:69-81`) calls `issue_fields_are_valid` per-issue but performs no cross-issue checks.

A hand-edited `tracker.json`:

```json
[
  {"id":1,"title":"A","status":"open","priority":"medium","labels":[],"created_at":"2026-05-01T00:00:00Z","updated_at":"2026-05-01T00:00:00Z"},
  {"id":1,"title":"B","status":"open","priority":"medium","labels":[],"created_at":"2026-05-01T00:00:00Z","updated_at":"2026-05-01T00:00:00Z"}
]
```

loads without error. Then `tracker status 1 done` calls `issues.iter().position(|i| i.id == id)` (`src/lib.rs:155-158`) — finds only the first match and silently leaves the duplicate untouched. The user sees one issue change status; the second copy retains the old status invisibly. `tracker create "C"` then computes `next_id = max(1,1)+1 = 2`, assigning ID 2 — the duplicate is now permanent and invisible to the user.

This is the exact failure mode the IAR primer warns about: "the most dangerous data bug is not a validation gap — it is a schema that encodes the wrong model of the domain and silently corrupts every downstream operation." A duplicate-ID violation downstream-corrupts every command that locates an issue by ID.

Recommendation: in `load_issues`, after the per-issue validation pass, verify `issues.iter().map(|i| i.id).collect::<HashSet<_>>().len() == issues.len()`. Treat duplicates as corrupt data (same error path). Cost: ~3 lines.

**Classification:** Open. Cross-reference [SECURITY-REVIEW.md](SECURITY-REVIEW.md) and [SOLUTION-ARCHITECT-REVIEW.md](SOLUTION-ARCHITECT-REVIEW.md) (the uniqueness invariant is structural, not just a runtime concern).

---

**Finding 3 — Forward-compat unknown fields are silently dropped on rewrite (Dim 3 — Schema evolution)**

`Issue` struct (`src/lib.rs:12-23`) has no `#[serde(other)]` catch-all and no flatten-extra mechanism. DESIGN.md Edge Cases / Storage states: "valid JSON but unknown fields → unknown fields are ignored (forward-compatible deserialization)." This is honored on read.

But the read-modify-write cycle is destructive: if a future version of the tool adds an `assignee` field, then a user runs an older binary, the older binary loads the JSON (silently dropping `assignee`), serializes back without it, and overwrites `tracker.json`. The forward-compat data is permanently lost on the next mutation.

For a single-user tool with a single binary version installed at a time, this risk is low. But the user is encouraged by DESIGN.md to hand-edit `tracker.json` ("verify it in a text editor"), and the tool is part of a portfolio meant to demonstrate engineering practice. A hand-added comment-style field (`"_note": "remember to refactor"`) would silently disappear on the next `tracker status` command.

Recommendation: either (a) document this as an explicit non-goal (in DECISIONS.md or DESIGN.md Out of Scope: "Hand-added unknown fields are not preserved across mutations"), or (b) implement field preservation via `#[serde(flatten)] extra: HashMap<String, Value>`. Option (a) is the right call for portfolio scope; it's the absence of the documentation that makes this a finding rather than a deliberate trade-off.

**Classification:** Raised to SO. The Solution Owner controls DESIGN.md; the proposed amendment is to add to "Out of Scope" or DECISIONS.md a documented statement that the read-modify-write cycle does not preserve unknown fields, despite their being ignored on read. No implementation change required if SO accepts the documentation-only resolution.

---

### Dismissed

**Finding 4 — `description: Option<String>` deserializes missing key as `None` without `#[serde(default)]` (Dim 3 — `serde` schema evolution)**

The Rust supplement Data Engineering section flags `#[serde(default)]` on new optional fields as a forward-compat concern. `description` lacks the attribute.

**Classification:** Dismissed. `Option<T>` has implicit serde behavior: a missing key deserializes to `None` regardless of `#[serde(default)]`. Verified by Layer 1 tests (`create_first_issue_unchanged_after_second_create` reads `tracker.json` containing no `description` key and never fails deserialization). The attribute is redundant for `Option<T>`. No defect; no change required.

**Finding 5 — `next_id` overflow at `u64::MAX` not handled (Rust supplement — integer overflow)**

`src/lib.rs:39-41` uses `+ 1` without `checked_add`.

**Classification:** Dismissed. Already raised and accepted as risk in [RED-TEAM-REVIEW.md](RED-TEAM-REVIEW.md) Review 1 Finding 2. Re-raising would not add signal.

**Finding 6 — Description data not validated on load — empty/whitespace `description` accepted (Dim 4)**

`issue_fields_are_valid` does not check description content. A hand-edited `"description": ""` or `"description": "   "` would load without error, contradicting the create-time validation that rejects these.

**Classification:** Deferred. Description is Layer 6 scope (TODO.md confirms). Raise concretely under DE Review 7+ when Layer 6 lands. Adding the check now would test code that does not exist yet. Tracking note: when Layer 6 implements `--description`, `issue_fields_are_valid` must extend to: `description.as_ref().is_none_or(|d| !d.trim().is_empty())`.

---

### Hallucinated

*(none)*

---

### Summary

Three real findings on data integrity invariants enforced at the data-layer boundary. The implementation correctly resolved Reviews 1-5 findings (post-deserialization domain validation, top-level array, `description` skip-if-none, status canonicalization). What remains untested at the load boundary is the **trans-issue and intra-record invariant set**: timestamp format and ordering (Finding 1), ID uniqueness across the array (Finding 2), and the documentation gap around unknown-field preservation across read-modify-write (Finding 3).

The first two findings are the classic adversarial failure mode the primer warns about — the data layer trusts the file enough to accept duplicate IDs and malformed timestamps. The third is a forward-compat documentation gap, not an implementation gap. Two findings dismissed (one on serde semantics, one already-handled by Red Team), one deferred to Layer 6.

The MVR signal for this domain is not yet reached: real findings remain. Re-evaluate after Findings 1-2 are addressed.

**Coordination:**
- Finding 1 → [SECURITY-REVIEW.md](SECURITY-REVIEW.md) (untrusted-input boundary), [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) (DESIGN.md amendment for corrupt-data scope of timestamps)
- Finding 2 → [SECURITY-REVIEW.md](SECURITY-REVIEW.md), [SOLUTION-ARCHITECT-REVIEW.md](SOLUTION-ARCHITECT-REVIEW.md)
- Finding 3 → [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) (Raised to SO; documentation amendment)

**Files modified:** This review log only.

---

### Update — 2026-05-04 16:00Z: Layer 3 follow-up resolution pass

Both implementation Open findings closed; the Raised-to-SO finding remains pending SO action.

- **F1 (timestamp ISO 8601 format + `updated_at >= created_at`) → Resolved.** New helper `parse_timestamp` (`src/lib.rs`) wraps `chrono::DateTime::parse_from_rfc3339`. `issue_fields_are_valid` extended with `parse_timestamp(&issue.created_at).is_some() && parse_timestamp(&issue.updated_at).is_some() && issue.updated_at >= issue.created_at`. ISO 8601 second-precision UTC strings are lex-comparable, so the `>=` comparison is correct without parsing into `DateTime` for the relational check (parsing handles only the format check). Regression locked by `tests/layer1.rs:{malformed_timestamp_in_json_causes_error_exit, updated_before_created_in_json_causes_error_exit}` and unit tests `issue_field_validation_rejects_{malformed_timestamp, updated_before_created}` / `issue_field_validation_accepts_equal_created_and_updated`.
- **F2 (ID uniqueness across collection) → Resolved.** `issues_collection_invariants_hold` (HashSet membership walk) added; `load_issues` now calls both per-record and cross-record validators. Stored data with duplicate IDs triggers `CORRUPT_DATA_ERROR`. Regression locked by `tests/layer1.rs:duplicate_ids_in_json_causes_error_exit` and unit tests `collection_invariants_{reject_duplicate_ids, accept_unique_ids}`.
- **F3 (forward-compat unknown fields silently dropped on rewrite) → still Raised to SO.** Documentation amendment to DESIGN.md / DECISIONS.md not applied (SO authority). Recommend SO add an explicit note in DESIGN.md "Storage / Edge Cases" — unknown fields in stored JSON are accepted at load (`serde` default behavior, intentional for forward compatibility) but are NOT preserved across writes (any subsequent mutation drops them). This is the implicit current behavior; users editing `tracker.json` directly should know.

**Forward-compat side-benefit (not a separately raised finding):** the new validator also rejects empty/whitespace `description` content in stored data, paving the way for Layer 6 (`--description`) without a follow-up validator pass.

---

### Update — 2026-05-05 11:00Z: SO Review 13 spec adjudication

- **F3 (forward-compat unknown fields silently dropped on rewrite) → Resolved by SO Review 13 Finding 3.** DESIGN.md Edge Cases / Storage now states explicitly: "Unknown fields in stored JSON load successfully (forward-compatible deserialization). They are NOT preserved across writes — any subsequent mutation rewrites `tracker.json` with only the documented schema fields, dropping anything else. Hand-edited `tracker.json` files should not rely on extra keys persisting." DECISIONS.md gains a corresponding entry citing this DE finding.

The behavior was unchanged — the spec gap was only that the side-effect was undocumented. Users hand-editing `tracker.json` to add custom keys will now find an explicit warning in DESIGN.md before discovering the loss empirically.

**Cross-cut from SO Review 13 Finding 1 (control-char title rejection):** `issue_fields_are_valid` was extended in the same round to add `&& !issue.title.chars().any(char::is_control)` — closes a related stored-data integrity gap that DE round 6 had not separately raised but is consistent with the same "stored data is untrusted" posture. The DE-domain logic for the new check follows the existing per-record validation pattern.

**No new DE findings this round.**

