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

---

---

## Review 7 — 2026-05-05 21:45Z

**Scope:** Layer 4 cold-session pass — `labels: Vec<String>` field added to `Issue`. Audit of round-trip serialization for empty / single / multi / special-char labels, forward-compat behavior on label-bearing JSON, field invariants (dedup-at-creation, case-preserved, never-empty-after-trim), and stored-data corruption detection per DESIGN.md "Edge Cases / Storage". Files reviewed: `DESIGN.md`, `Cargo.toml`, `src/lib.rs`, `src/main.rs`, `tests/layer4.rs`, `tracker.json`.

**Session note:** Cold session per primer. Reviewer did not build the project. Adversarial posture: stored data and CLI input are both untrusted. Live experiments performed via the compiled `tracker` binary against scratch directories under `/tmp/de-test*`.

---

### Round-trip serialization — verified working

Empty labels (`labels: []`), single label, multi-label preserving order, leading/trailing whitespace trimmed at create, internal spaces preserved (`"with space"`), non-ASCII (`"ünicode-é"`), and emoji (`"emoji 🚀"`) all round-trip correctly through `cmd_create` → `save_issues` → `load_issues`. `dedupe_labels` correctly preserves first occurrence and is case-sensitive (`"bug"`, `"Bug"`, `"BUG"` all stored). Empty `[]` serializes as `[]`, not omitted — matches DE Review 3 Finding 4 dismissal.

Stored-data corruption detection at load time:
- `"labels": [""]` → corrupt-data error, exit 1 ✓
- `"labels": ["  "]` → corrupt-data error, exit 1 ✓ (validator uses `trim().is_empty()`)

These satisfy the DESIGN.md "Edge Cases / Storage" enumeration that includes "an empty `label`" as a corruption trigger.

---

### Open

**Finding 1 — Labels are not validated for control characters at create time or at load time (Dim 2 — Validation and normalization, Dim 4 — Data integrity invariants)**

`src/lib.rs:339-346` (`parse_label`) only trims and rejects empty-after-trim. `src/lib.rs:131` (`issue_fields_are_valid`) only checks `!l.trim().is_empty()`. Neither path applies the `char::is_control` rejection that title fields receive (`src/lib.rs:73`, `src/lib.rs:128`).

Empirical reproduction (`/tmp/de-test2`, `/tmp/de-test3`):

```
$ tracker create "Test" --label $'bug\nbreak'
Created issue #1: Test
$ tracker list
ID    Status       Priority  Labels                Title
1     open         medium    bug
break             Test
```

The newline embedded in the stored label splits a single issue's row across two physical lines on `list` output, breaking the spec's one-issue-per-line contract. The same input path with an ANSI escape sequence (`$'\x1b[31mEvil\x1b[0m'`) stores the literal `0x1B` byte and emits it on `list`, enabling terminal-escape injection — exactly the attack vector that DESIGN.md "Edge Cases / Title" enumerates as the rationale for control-char title rejection ("ESC, C1 controls", "terminal-escape injection in any tool that displays the title"). Labels are displayed in the same `list` output and the same `show` output (Layer 6 scope) and so share the threat model.

Stored-data variant (`/tmp/de-test5`): a hand-edited `tracker.json` with `"labels": ["bug\nbreak"]` loads successfully and breaks `list` output the same way. The validator does not enforce label hygiene that the spec implicitly requires by parallel reasoning to title hygiene.

This is the exact failure mode DE Review 6 Finding 1 / 2 fixed for timestamps and IDs and the SO Review 13 Finding 1 cross-cut fixed for stored titles. The labels field is the remaining `String`-bearing field in the schema where the same untrusted-input invariant has not been applied symmetrically.

**Recommendation:**
- **SE:** Extend `parse_label` to reject control characters at create-time with `Err("Label cannot contain control characters.")`, mirroring `validate_title`. Extend `issue_fields_are_valid` (`src/lib.rs:131`) with `&& issue.labels.iter().all(|l| !l.chars().any(char::is_control))` — same shape as the title check on line 128.
- **SO:** DESIGN.md "Edge Cases / Labels" currently enumerates only empty / whitespace-only / case-sensitivity. Extend to add a "Label containing a control character" bullet pointing to the same rationale as Title (display contract, escape-injection). Extend "Edge Cases / Storage" to include "an empty or control-character `label`" in the corruption-trigger list (the line currently reads `"an empty 'label'"`).
- **QE:** Add unit test `parse_label("bug\nbreak").is_err()` and integration test asserting create with `--label $'bug\nbreak'` exits 1 with `Error: Label cannot contain control characters.`. Add `tests/layer4.rs` integration test asserting that a stored `tracker.json` with a control-char-bearing label triggers the corrupt-data error path on load.

**Classification:** Raised to SE (validator extension), Raised to SO (DESIGN.md edge-case enumeration), Raised to QE (regression coverage). The scope is symmetric to DE Review 6 Finding 1 / SO Review 13 Finding 1 — the same untrusted-input boundary control, applied to the new field.

Cross-reference: [SECURITY-REVIEW.md](SECURITY-REVIEW.md) (escape-injection vector via labels), [RED-TEAM-REVIEW.md](RED-TEAM-REVIEW.md) (terminal-injection through stored data), [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) (DESIGN.md amendment), [QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md) (test coverage gap).

---

**Finding 2 — Label filter input is not normalized symmetric with create-time normalization (Dim 2 — Validation and normalization)**

`src/lib.rs:422-424` passes the raw `label_filter` straight to `label_matches` without trimming. `parse_label` trims at create time. This produces an asymmetry that surprises users:

Empirical reproduction (`/tmp/de-test9`): a stored label `"  bug  "` (untrimmed because hand-edited or written by an older version) cannot be matched by `--label "bug"` (filter not trimmed in storage) and CAN be matched by `--label "  bug  "`. Conversely, an issue created with `--label "bug"` (stored as `"bug"`) can be filtered by `tracker list --label "  bug  "` only by accident — actually that's a no-match because the filter side is also not trimmed.

DESIGN.md is silent on whether `list --label` should trim its input. The contract says "exact match, case-sensitive" — read literally, the current behavior is correct. But the create side trims, which creates asymmetric user expectations: "I created with `bug` (with spaces); why does `--label bug` not find it?" The minor user-facing harm is mitigated by `parse_label`'s trim at create time guaranteeing stored labels never have leading/trailing whitespace (under organic flow); the asymmetry is exposed only via hand-edited storage.

**Recommendation:**
- **SO:** Clarify in DESIGN.md "Feature 2 / List" or "Edge Cases / Labels" whether `--label` filter input is trimmed before comparison. Either choice is defensible; the spec needs to pick one. Recommended: trim filter input symmetrically with create input, so the filter and create normalize the same way and a hand-edited untrimmed stored label is unreachable by filter (which is a feature, not a bug — that label is corrupt by the create-time contract anyway).
- **SE:** Whatever SO decides, apply the same `trim()` policy on both sides of the boundary.

**Classification:** Raised to SO. Spec-clarity finding; implementation matches the literal text of "exact match, case-sensitive", so there is no implementation defect against the current spec.

Cross-reference: [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) (DESIGN.md amendment).

---

### Dismissed

**Finding 3 — `labels` field on `Issue` lacks `#[serde(default)]` for backward-compat with pre-Layer-4 stored data (Dim 3 — Schema evolution; Rust supplement — `#[serde(default)]` for schema evolution)**

`src/lib.rs:47` declares `pub labels: Vec<String>` with no `#[serde(default)]`. A `tracker.json` written by a Layer 1-3 binary would have no `labels` key. A Layer 4 binary loading that file would fail deserialization with "missing field `labels`".

**Classification:** Dismissed (with caveat). Empirically tested: a `tracker.json` from the existing project root has `"labels": []` already (Layer 1's `cmd_create` always writes `Vec::new()`, which serializes to `[]`). DESIGN.md DE Review 3 Finding 4 confirms the empty array has been written from Layer 1 onward; there is no real-world stored data without the key. The Rust supplement's `#[serde(default)]` guidance applies to *new* optional fields added after the field has been deployed without serialization support. Here, the field has been *present* in serialized form (as `[]`) since Layer 1; only the *write code path* and *display code path* have changed in Layer 4. No backward-compat hazard exists in this project's actual storage history.

Caveat: a future field addition that adds `Vec<X>` *for the first time* would need `#[serde(default)]` (or `Option<Vec<X>>`) for files written before that field existed. Current implementation does not establish a pattern for that case. Tracking note for whoever adds the next collection-typed field.

---

**Finding 4 — `dedupe_labels` allocates a new `String` per label via `.clone()` (Dim 6 — Access patterns, supplement: Lifetimes and cloning)**

`src/lib.rs:351-360` clones each kept label into the output vector even though the input slice could be consumed by value or returned with refs.

**Classification:** Dismissed. Performance is irrelevant at this scale (single-digit labels per issue, micro-cost). The cloning pattern matches the function's `&[String] -> Vec<String>` signature, which is the right ergonomic shape for the call site. No defect.

---

**Finding 5 — `Vec<String>` for labels does not enforce uniqueness at the type level (Dim 1 — Data model correctness)**

DESIGN.md states labels are "deduplicated at creation". A `BTreeSet<String>` or `IndexSet<String>` would encode this in the type. As `Vec<String>`, uniqueness is a runtime invariant enforced only by `dedupe_labels` at create time.

**Classification:** Dismissed. The field invariant requires *order preservation* ("order is preserved") in addition to dedup. `BTreeSet` violates insertion order. `IndexSet` would require a non-stdlib dep; `serde` would still serialize as a JSON array regardless. The current `Vec<String>` + dedup-at-creation is the correct trade-off given the spec requires both ordered and deduped. Stored-data uniqueness across reads is NOT validated, but a stored `["bug", "bug"]` is harmless to downstream operations (display shows `bug, bug` — cosmetically odd but not corrupt) — not a real defect worth raising as Open.

---

### Hallucinated

*(none)*

---

### Summary

Two real findings, both rooted in the same principle: the labels field is the new `String`-bearing data in Layer 4, and it has not received the same untrusted-input hygiene treatment that titles received in Layer 1 / SO Review 13. Finding 1 is the strongest — it exposes the same control-character / escape-injection failure mode that DESIGN.md explicitly enumerates as the rationale for title control-char rejection, and the implementation already has the validation pattern to apply (`!s.chars().any(char::is_control)` is on line 73 of lib.rs). The fix is symmetric and ~2 lines. Finding 2 is a minor spec/implementation symmetry gap.

Round-trip serialization is otherwise clean: empty / single / multi / unicode / emoji labels round-trip correctly. Stored empty/whitespace labels are correctly caught as corrupt data. Forward-compat behavior on unknown fields is correct per DESIGN.md (load tolerates, write drops — documented after SO Review 13 Finding 3).

The MVR signal for this domain is not yet reached for Layer 4: Finding 1 is a real defect with a clear spec parallel and a small fix. Finding 2 is a real ambiguity. Re-evaluate after Finding 1 is addressed (SE + SO + QE) and SO clarifies Finding 2.

**Concerns for the merge gate:**
1. **Block on Finding 1.** Layer 4 ships the labels feature; allowing newline / ESC in stored labels reproduces the exact threat model that the title control-char check exists to prevent. The fix is symmetric to existing code — the cost of doing it now is far below the cost of finding it later in production via a corrupted display.
2. Finding 2 should land before Layer 5 to avoid documenting two contradictory normalization policies (create vs. filter) at the schema-stability layer.

**Coordination:**
- Finding 1 → [SOFTWARE-ENGINEER-REVIEW.md](SOFTWARE-ENGINEER-REVIEW.md) (validator extension, ~2 lines), [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) (DESIGN.md edge-case enumeration), [QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md) (regression test), [SECURITY-REVIEW.md](SECURITY-REVIEW.md) / [RED-TEAM-REVIEW.md](RED-TEAM-REVIEW.md) (escape-injection vector).
- Finding 2 → [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) (spec-clarity), [SOFTWARE-ENGINEER-REVIEW.md](SOFTWARE-ENGINEER-REVIEW.md) (apply chosen policy on filter side).

**Files modified:** This review log only.

---

## Review 8 — 2026-05-06 02:45Z

**Round:** Data Engineer Review 8 (Round-2 verification for Layer 4)
**Scope:** Verify Round-1 F1 (label control-char) and F2 (filter normalization symmetry) are closed by the SO/SE/QE Round-2 work in commit `67ef920`.
**Session context:** Warm-verification session.

### Resolved

#### Finding 1 (Round-1) — Label control-character at create + load

DESIGN.md sanctioned per SO Review 17. SE applied the `parse_label` extension AND the load-time check via the new `label_is_valid` helper called from `issue_fields_are_valid` (`src/lib.rs:131`). QE added integration tests for both paths, plus unit tests on `label_is_valid` directly. Verified by re-running the Review 7 reproducers from `/tmp/de-test*`:

- `tracker create "Test" --label $'bug\nbreak'` → exit 1 with `Error: Label cannot contain control characters.` ✓
- Hand-edited `tracker.json` with `"labels": ["bug\nbreak"]` → `Could not read tracker data...` exit 1 ✓
- Same for tab, ESC, NUL, DEL ✓

The data-model invariant — every `String`-bearing field in `Issue` has the same untrusted-input hygiene at create-time AND load-time — is now uniform across `title`, `labels`, and (Layer 6 forward-prep) `description`. **Resolved.**

#### Finding 2 (Round-1) — Filter trim symmetry

DESIGN.md Feature 2 amended (per SO Review 17 Option A): the `--label` filter value is trimmed before comparison, and an empty/whitespace-only filter is rejected with the spec-literal `Error: Label cannot be empty.`. SE applied `parse_label` on the filter side in `cmd_list`. QE added `list_label_filter_is_trimmed_to_match_stored`, `list_empty_label_filter_exits_one`, `list_whitespace_label_filter_exits_one`, and `list_control_char_label_filter_exits_one`. Verified: a stored `bug` is now reachable via `tracker list --label "  bug  "`. The two normalization policies (create vs. filter) are now identical. **Resolved.**

### Round-trip serialization regression check

`Cargo.toml` and `Cargo.lock` unchanged. The new `parse_label` and `label_is_valid` rules don't affect `serde` round-trip semantics — they're invariant predicates over the data, not codec changes. Verified empirically: a clean `tracker.json` written by Round-2 binary loads correctly; the additional rejection cases (control-char, comma) are caught at load time as documented in DESIGN.md Edge Cases / Storage. The forward-compat behavior (unknown fields tolerated at load, dropped at write) is intact.

### Summary

Round-1 F1 + F2 → Round-2 0 Open. The data-model invariant is now uniform across all `String`-bearing fields. No new findings. **Coordination:** cross-references Security Review 8, Red Team Review 7, SE Review 12, QE Review 12, SO Review 17.

**Files modified:** Only this log appended.

---

