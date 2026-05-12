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

## Review 9 — 2026-05-11 01:09Z

**Round:** Data Engineer Review 9 (cold-session pass, Layer 6 — `--description` + show + delete)
**Scope:** Commits `4fb5e67` (Red Gate stubs/tests) + `c91676a` (implementation). DE-domain audit of the new `description: Option<String>` schema member, load-time validation, `next_id` after delete, save atomicity, post-delete storage shape, schema evolution from pre-Layer-6 data, and storage edge cases (CRLF / multi-line / very long / whitespace-only descriptions). Files reviewed: `DESIGN.md`, `src/lib.rs`, `tests/layer6.rs`, `TODO.md` Layer 6.

**Session context:** Cold session per primer. Adversarial posture: stored data is untrusted (DESIGN.md Storage Edge Cases enumerates "control-character in `title`" and "control-character or comma in any `label`" as corruption triggers; description is the new `String`-bearing field on the same boundary). Live experiments performed against `/tmp/de-r9` with the Layer 6 binary.

---

### Open

**Finding 1 — Stored `description` is not validated for control characters at load time (Dim 2 — Load-time validation; Dim 7 — Edge cases / Description)**

`src/lib.rs:125-139` (`issue_fields_are_valid`) currently validates description only as `description.as_ref().is_none_or(|d| !d.trim().is_empty())` — empty-after-trim is rejected, nothing else. By contrast, `title` (line 128) is validated with `!issue.title.chars().any(char::is_control)`, and `labels` (line 131) are validated through `label_is_valid` (line 145-147), which rejects control characters and commas. The description field is the only `String`-bearing schema member without control-char hygiene at the load boundary.

Empirical reproduction (`/tmp/de-r9`, hand-edited `tracker.json` written via Python so the raw bytes are unambiguous):

```python
data = [{"id": 1, "title": "Test",
         "description": "Line1\nline2-injected\x1b[31mRED\x1b[0m",
         "status": "open", "priority": "medium", "labels": [],
         "created_at": "2025-01-01T00:00:00Z",
         "updated_at": "2025-01-01T00:00:00Z"}]
```

`tracker show 1` loads this issue successfully (exit 0) and emits the raw `0x1B` byte to stdout. Verified via `od -c`:

```
0000200    -   i   n   j   e   c   t   e   d 033   [   3   1   m   R   E
0000220    D 033   [   0   m  \n
```

This is the exact failure mode DE Review 7 F1 / SO Review 17 fixed for stored labels and SO Review 13 F1 fixed for stored titles. The threat model from DESIGN.md "Edge Cases / Title" (control characters "break the one-issue-per-line contract", "corrupt column alignment", "enable terminal-escape injection in any tool that displays the title") applies word-for-word to description, since `tracker show` renders description as one of the labelled key-value rows on stdout. The Layer 4 round-2 commit message explicitly noted: "the data-model invariant — every `String`-bearing field in `Issue` has the same untrusted-input hygiene at create-time AND load-time — is now uniform across `title`, `labels`, and (Layer 6 forward-prep) `description`." Layer 6 landed without finishing that forward-prep on the load side.

A defect-revealing hand-edited `tracker.json` for this dimension:

```json
[{"id": 1, "title": "Innocent",
  "description": "ok]8;;file:///etc/passwdClick me]8;;",
  "status": "open", "priority": "medium", "labels": [],
  "created_at": "2025-01-01T00:00:00Z",
  "updated_at": "2025-01-01T00:00:00Z"}]
```

OSC 8 hyperlink injection via stored description; `tracker show 1` renders it as a clickable terminal hyperlink to a local file path. The user opened the tracker; the data file (which the user may have copied from another machine, or restored from backup, or hand-edited under another agent's direction) attacks the terminal. Same threat surface as labels and titles.

**Cross-cut (input side, noted for SE/Security context but the DE-domain framing is the load gap):** `validate_description` at `src/lib.rs:335-340` also has no `char::is_control` rejection. `tracker create "Bad" --description $'l\x1bm'` succeeds; the resulting `tracker.json` stores `"description": "lm"`. So today, the input boundary AND the load boundary both fail to reject control chars in description — meaning even an organic create flow (not just hand-edited storage) can plant the injection vector. This is consistent with Security/RT framings, but DE's specific contribution is: even after SE fixes the input side, `issue_fields_are_valid` must reject control-char descriptions at load time so that a `tracker.json` containing such a description from any source (older binary, hand-edit, restored backup, cross-host transfer) is treated as corrupt rather than silently rendered.

**Recommendation:**
- **SO:** DESIGN.md "Edge Cases / Description" currently has six bullets covering absent / empty / length / not-trimmed / multi-line behaviors. Add a bullet: "Description containing a control character (Unicode general category `Cc` — newline, CR, tab, NUL, ESC, DEL, C1 controls) other than the spec-sanctioned `\n` line separator → error: `Description cannot contain control characters.` Same rationale as Title and Labels (preserves rendering contract in `show`, prevents terminal-escape injection)." **Note the carve-out:** description is the only field the spec actively permits to contain `\n` (DESIGN.md "Edge Cases / Description" final bullet — "Description may contain newlines (`\n`)"). The validator must reject every `Cc` *except* `\n`; bare `\r` is rejected (see Finding 2). Extend DESIGN.md "Edge Cases / Storage" enumeration: add "a control-character (other than `\n`) in `description`" to the corruption-trigger list (alongside the existing "control-character in `title`" and "control-character or comma in any `label`" items).
- **SE:** Extend `validate_description` (line 335) and `issue_fields_are_valid` (line 132-135) with the carved-out predicate (e.g. a `description_is_valid(&str)` helper paralleling `label_is_valid` that returns `false` if `d.trim().is_empty() || d.chars().any(|c| c.is_control() && c != '\n')`).
- **QE:** Add (1) unit test asserting `validate_description("l\u{1B}m").is_err()` and `validate_description("line1\nline2").is_ok()` — the latter pins the carve-out; (2) integration test asserting `tracker create "Bad" --description $'l\x1bm'` exits 1 with the new error message; (3) integration test asserting a hand-edited `tracker.json` with `"description": "lm"` triggers the corrupt-data error path on load.

**Classification:** Raised to SO (spec amendment — including the `\n` carve-out), Raised to SE (validator extension at both input and load boundaries), Raised to QE (regression coverage at both boundaries + the `\n` carve-out). The scope is symmetric to DE Review 7 F1 / SO Review 17.

Cross-reference: [SECURITY-REVIEW.md](SECURITY-REVIEW.md) (escape-injection vector via descriptions in `show`), [RED-TEAM-REVIEW.md](RED-TEAM-REVIEW.md) (terminal-injection through stored data), [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) (DESIGN.md amendment), [SOFTWARE-ENGINEER-REVIEW.md](SOFTWARE-ENGINEER-REVIEW.md), [QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md).

---

**Finding 2 — `format_show_block` normalizes only `\r\n` → `\n`; a bare `\r` in a stored description is emitted raw and overprints the alignment column (Dim 7 — Edge cases / Description)**

`src/lib.rs:365` reads `let normalized = d.replace("\r\n", "\n"); normalized.replace('\n', "\n             ")`. This handles CRLF correctly but a description containing a bare `\r` (no following `\n`) is left untouched in the replace, then printed verbatim. A bare `\r` is a "go to column 0" cursor instruction in essentially every terminal — the bytes printed after it overwrite the same physical line, destroying the 13-space alignment that `show` is contractually obligated to maintain ("first line follows the `Description:` label; each continuation line is indented by 13 spaces").

Empirical reproduction (`/tmp/de-r9`):

```python
data = [{"id": 1, ..., "description": "line1\r\nline2\rline3", ...}]
```

`tracker show 1` produces (od -c on the stdout):

```
Description: line1\n             line2\rline3\n
```

The `\r\n` was correctly normalized into `\n` + 13-space indent. The bare `\r` after `line2` was preserved. When rendered to a terminal, `line2` is overwritten by `line3` starting at column 0, breaking the show alignment contract. Even ignoring the visual artifact, the byte stream contains a control character that DESIGN.md "Edge Cases / Title" explicitly enumerates as a rejection trigger for titles ("newline / CR ... break the one-issue-per-line contract") — and description is rendered in the same display surface.

This is in scope for Finding 1's recommended carve-out predicate: rejecting all `Cc` other than `\n` *would* reject this case (`\r` is `Cc`). The two findings overlap; this is a finer-grained pointer to one specific concrete failure mode of the more general gap. If SO chooses a different remediation (e.g. "allow `\r\n` and `\n`, reject other control chars"), the rule needs to explicitly enumerate bare-`\r` rejection — `\r\n` is the only `\r`-bearing sequence the spec should accept; lone `\r` has no defensible purpose in stored description.

**Recommendation:** subsume into Finding 1's predicate. Specifically: the predicate must reject `\r` even though the spec permits "newlines (`\n`)" — i.e. lone `\r` is not a "newline" in the spec's sense; only `\r\n` and `\n` are. Document this in DESIGN.md alongside the Finding 1 amendment.

**Classification:** Raised to SO + SE + QE as a sub-case of Finding 1. Tracked separately because the manifestation (alignment-column overprinting in `show`) differs from Finding 1's manifestation (escape-injection via ESC) and the test case is distinct.

Cross-reference: same as Finding 1.

---

### Dismissed

**Finding 3 — `save_issues` is non-atomic; a write failure mid-process could leave `tracker.json` truncated (Dim 4 — Save atomicity)**

`src/lib.rs:210-215` calls `fs::write(path, contents)` directly with no temp-file-and-rename. A disk-full / power-loss / kill-9 between `fs::write` opening the file (truncating) and finishing the write would leave `tracker.json` in a half-written state, likely failing JSON parse on next load → all data appears corrupt to the user.

A defect-revealing input for this dimension: not a `tracker.json` shape but a runtime condition — e.g. `dd` into the data directory to fill the disk just before invoking `tracker create`, or `kill -9` the binary during the `fs::write` call.

**Classification:** Dismissed (spec-sanctioned trade-off). DESIGN.md "Out of Scope" explicitly enumerates: "Atomic writes — direct write to `tracker.json` on every mutation; no temp-file-and-rename. Correct production practice but implementation cost exceeds failure risk for a single-user local tool. Revisit if the tool is ever used in a context with multiple concurrent writers." DESIGN.md "Storage invariants" similarly says: "`tracker.json` is written directly on every mutation; on I/O failure the file may be in an indeterminate state — the error is reported and the binary exits 1. Atomic writes are the correct production approach and are deferred — implementation cost exceeds the failure risk for a single-user local tool." The implementation matches the spec. Self-test by dismissal passes: there is no defect against the spec, only a difference between this spec and what a production multi-user data store would require.

No raise. Tracking note: if the project's threat model widens (multi-host sync, shared `tracker.json`, agent-driven concurrent edits), the atomic-write deferral is the first storage decision to revisit.

---

**Finding 4 — Schema evolution: a `tracker.json` written by a pre-Layer-6 binary has no `description` key; a Layer 6 binary must read it without error (Dim 6 — Schema evolution)**

`src/lib.rs:39-40` declares `#[serde(skip_serializing_if = "Option::is_none")] pub description: Option<String>`. `Option<T>` in serde defaults to `None` when the key is absent — this is the documented serde behavior for `Option` fields (no `#[serde(default)]` needed because the codec already treats absent-key as `None` for `Option`).

Empirical verification (`/tmp/de-r9`): a hand-edited `tracker.json` matching the exact shape of a Layer 1-5 binary's output (no `description` key) loads successfully under the Layer 6 binary. `tracker show 1` renders `Description: (none)`. No corrupt-data error. Round-trip on a subsequent mutation re-writes the file without a `description` key (because `None` skips via `skip_serializing_if`), so pre-Layer-6 data round-trips through Layer 6 with no schema churn.

**Classification:** Dismissed. The schema-evolution contract from DE Review 1 Finding 2 / Layer 1 (`skip_serializing_if = "Option::is_none"`) and from DE Review 3 Finding 2 (presence of the attribute since Layer 1) is satisfied by construction. Layer 6 added the field as a real schema member exactly the way `Option<T>` fields are supposed to be added: optional, absent-when-None, default-None-on-missing. No defect.

A defect-revealing input for this dimension would be a `tracker.json` containing `"description": null` (a hand-edit, or written by a tool that doesn't use `skip_serializing_if`). Empirical test: `python3 -c "import json; json.dump([{...,'description': None,...}], open('tracker.json','w'))"` → load. Result: loads successfully (`null` deserializes to `None` for `Option<String>` in serde, then `is_none_or(...)` is vacuously true; no error). Forward-compatible AND backward-compatible. The DESIGN.md Data Model line 165 contract ("Implementations must omit the key when the value is None") is about *writes*; reads tolerate `null` as a courtesy. No defect.

---

**Finding 5 — `Issue` does not derive `serde(deny_unknown_fields)`, so a hand-edited `tracker.json` with a typo'd field (e.g., `descrption`) silently drops the value on round-trip (Dim 3 — Schema evolution, Dim 6)**

A defect-revealing input: `{"id": 1, ..., "descrption": "the real description"}` (note typo). Load succeeds, the typo field is dropped, any subsequent mutation rewrites the file without it.

**Classification:** Dismissed by prior round (DE Review 6 F3 → SO Review 13 Finding 3). DESIGN.md "Edge Cases / Storage" now states: "Unknown fields in stored JSON load successfully (forward-compatible deserialization). They are NOT preserved across writes — any subsequent mutation rewrites `tracker.json` with only the documented schema fields, dropping anything else. Hand-edited `tracker.json` files should not rely on extra keys persisting." This is the spec's deliberate forward-compat posture. The typo-field hazard is a sub-case the user is now warned about in writing. No new raise.

---

**Finding 6 — `cmd_delete` leaves `tracker.json` as `[]` after deleting the last issue, but `next_id` correctly returns 1 again (Dim 3 — `next_id` and delete, Dim 5 — Storage shape after delete)**

After `tracker delete 1` on a single-issue tracker, `tracker.json` is `[]` (empty array). The spec says "the deleted ID is never reused" — but if the tracker is empty, `next_id(&[]) = 1`, and the next create gets id=1. Is this a contract violation against the "never reused" invariant?

**Classification:** Dismissed. Re-reading DESIGN.md Feature 5 invariants: "the next created issue receives `max(remaining_ids) + 1`, which will always be greater than the deleted ID". When the remaining set is empty, there is no `max` — the invariant text technically does not cover this case. But the *spirit* of "never reused" is about not reassigning a still-meaningful ID; once every issue is gone, the user has reset the tracker to its initial state, and getting id=1 on the next create is the principle-of-least-surprise outcome (matches the first-ever create's behavior). Walking `next_id`'s implementation (line 88-92): `existing_ids.iter().max().copied().unwrap_or(0)` then `+1` — empty slice yields 0+1=1. Verified empirically (`/tmp/de-r9`): create #1, delete #1, create "Next" → gets id=1. The behavior is what a careful spec author would have intended; the spec's phrasing is the minor ambiguity, not the implementation.

**Coordination note:** SO may wish to amend the Feature 5 invariant text to say `max(remaining_ids) + 1`, or `1` if no issues remain, to remove the edge ambiguity. Tracked as a minor wording cleanup, not a raise. The behavior is correct.

---

**Finding 7 — Post-delete storage shape: `tracker.json` after `delete` is still a valid array and still passes `load_issues` (Dim 5)**

Verified empirically (`/tmp/de-r9`): after `tracker delete 1` on a two-issue tracker, the file is a valid JSON array `[{...id:2...}]`. Subsequent `tracker list`, `tracker show 2`, `tracker status 2 done`, and `tracker create "x"` all succeed against the post-delete file. The serialized output preserves the pretty-printed shape (`serde_json::to_string_pretty`) and remains human-readable per DESIGN.md "Manual testing checklist" ("verify `tracker.json` is valid JSON after each mutation"). No defect.

**Classification:** Dismissed (working as specified).

---

**Finding 8 — Very long description (200KB), multi-line description, and whitespace-only description handled correctly at storage boundary (Dim 7 — Edge cases)**

Verified empirically:
- 200KB description: stored, loaded, rendered (truncated by terminal width but no error in the pipeline).
- Multi-line description (`"line1\nline2"`): stored verbatim, rendered with 13-space continuation indent via `format_show_block`.
- Whitespace-only stored description (`"   "`): load-time check `!d.trim().is_empty()` rejects → corrupt-data error → exit 1.

All match DESIGN.md "Edge Cases / Description": "Description is not validated for length (no maximum)"; "Description may contain newlines (`\n`)"; "`--description ""` (empty string after trim) → error" (and the load side mirrors this rejection).

**Classification:** Dismissed (working as specified). The single edge case that *fails* — bare `\r` and other control chars — is Finding 1 / Finding 2.

---

### Hallucinated

*(none)*

---

### Schema evolution assessment

Layer 6 is the first layer to add an `Option<String>` field to the schema. The schema evolution outcome is clean:

1. **Pre-Layer-6 reads under Layer 6 binary:** absent `description` key → deserializes to `None` (default serde behavior for `Option<T>`) → renders as `(none)` in show. ✓
2. **Layer 6 reads under pre-Layer-6 binary:** if `--description` was never used, no `description` key is written, so a pre-Layer-6 binary sees its own schema and loads fine. If `--description` was used, a pre-Layer-6 binary either ignores the `description` field (its `Issue` struct has no such field, serde drops unknown keys by default) or — if a stricter deserializer was used — could fail. Spec-sanctioned forward-compat (DE R6 F3 → SO R13 F3): unknown fields tolerated.
3. **`"description": null` in stored data:** deserializes to `None` (serde courtesy), then `is_none_or(...)` passes, rendered as `(none)`. ✓
4. **Round-trip preservation:** an issue with no description, written by Layer 6, contains no `description` key thanks to `#[serde(skip_serializing_if = "Option::is_none")]`. An issue with a description writes the field verbatim. No drift across reads/writes. ✓

The forward-compat invariant established at Layer 1 (DE R1 F2, DE R3 F2) holds through Layer 6 without modification. **No schema-evolution defect.**

---

### Summary

**Open: 2 findings.** Both root-cause the same gap: the `description` field is the new `String`-bearing schema member added in Layer 6 and it has not received the per-field control-character hygiene that title (SO R13 F1, Layer 1 cross-cut) and labels (DE R7 F1 / SO R17, Layer 4 round 2) received. Finding 1 is the general case (any control char at the load boundary); Finding 2 is one specific concrete manifestation (bare `\r` in `show` rendering breaks the 13-space alignment contract). The fix is symmetric to the existing `label_is_valid` pattern — about 4 lines of code in `src/lib.rs`, plus the spec amendment with an explicit `\n` carve-out, plus the test triad.

**Dismissed: 6 findings.** Save atomicity is spec-sanctioned out-of-scope. Schema evolution is clean — `Option<String>` with `skip_serializing_if = "Option::is_none"` is exactly the textbook serde idiom for this addition; pre-Layer-6 data loads correctly and round-trips without drift. `next_id` after delete is correct (empirically and by walking the function). Post-delete storage shape is valid. The other storage edge cases (long, multi-line, whitespace-only) are handled.

**Top DE concern:** Finding 1 — the load-time control-char gap for description. The fix pattern is exactly the one applied to labels in Layer 4 R2; the cost of doing it now (≤ 1 hour SE + DESIGN.md amendment + test triad) is far below the cost of finding it later via a real terminal-injection attack through a synced or backup-restored `tracker.json`. The Layer 4 R2 commit message claimed this was already done as "(Layer 6 forward-prep)"; it was not — only the empty-after-trim check landed.

**Schema evolution assessment:** clean. The new `Option<String>` field is added correctly per DE R1 F2 / DE R3 F2; forward-compat (unknown fields tolerated, dropped at write) is intact and documented per DE R6 F3 / SO R13 F3; backward-compat (pre-Layer-6 data loads under Layer 6) verified empirically and matches the documented `Option<T>` serde contract. The single load-time invariant that was *not* extended to description in Layer 6 is the control-char check (Finding 1) — that is a hygiene gap, not a schema-evolution gap.

**MVR signal:** not yet reached for Layer 6 in the DE domain. Finding 1 + Finding 2 are real defects with a clear spec precedent and a small fix. Re-evaluate after SE + SO + QE close them (Round 2).

**Coordination:**
- Finding 1 → [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) (DESIGN.md amendment + `\n` carve-out), [SOFTWARE-ENGINEER-REVIEW.md](SOFTWARE-ENGINEER-REVIEW.md) (`validate_description` + `issue_fields_are_valid` extension, paralleling `label_is_valid`), [QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md) (test triad: unit + create-side integration + load-side integration), [SECURITY-REVIEW.md](SECURITY-REVIEW.md) / [RED-TEAM-REVIEW.md](RED-TEAM-REVIEW.md) (escape-injection vector via description in `show`).
- Finding 2 → folds into Finding 1's predicate; called out separately for the `\r`-overprinting test case.

**Files modified:** Only this log appended.

---

## Review 10 — 2026-05-11 02:00Z

**Round:** Data Engineer Review 10 (Round-2 closure for Layer 6)
**Scope:** Verify Round-1 Open findings (description Cc load-time gap + `\r`-overprint subcase) are resolved by commit `9b775f0`. Warm closure-verification.

### Round-1 finding closures

- **F1 (description Cc load-time gap):** **Resolved by commit `9b775f0`.** `description_is_valid` helper added (`src/lib.rs`) mirroring `label_is_valid` from Layer 4 R2; called from `issue_fields_are_valid`. Hand-edited tracker.json with `is_control()` characters other than `\n` in description is now rejected at load with `Error: Could not read tracker data. The file may be corrupt.`. DESIGN.md Edge Cases / Storage updated to enumerate "a control-character other than newline in `description`" in the corruption triggers list.
- **F2 (bare `\r` overprint in show):** **Resolved by commit `9b775f0`.** Subsumed by F1's broader Cc-except-`\n` rule — `\r` is Cc and is not `\n`, so it is rejected at both create-time (`validate_description`) and load-time (`description_is_valid`). Additionally, `format_show_block`'s `\r\n` → `\n` normalization (now ratified in DESIGN.md "Show output format") provides defense-in-depth for any legacy stored data.

### Schema evolution re-verification

- `Option<String>` with `#[serde(skip_serializing_if = "Option::is_none")]` unchanged — pre-Layer-6 data still loads correctly under Layer 6 + R2.
- A pre-R2 tracker.json with a valid (non-Cc) description still loads under post-R2 binary.
- A pre-R2 tracker.json with a Cc-other-than-`\n` description in description **now** fails to load — this is the intended new invariant per DE F1 resolution, not a backward-compatibility regression. The defect was that such files should never have been writable in the first place (validate_description didn't check Cc at create time).

### New findings

*(none this round.)*

### Summary

2/2 Round-1 DE findings Resolved. Schema evolution is still clean. The load-time hygiene invariant for description now matches the title and label parallels (Layer 1 / Layer 4 R2). Layer 6 DE-domain is at MVR.

**Coordination:** *(none — closure pass)*

---

## Review 11 — 2026-05-11 22:30Z

**Round:** Data Engineer Review 11 (Round 1 for Layer 7)
**Scope:** Layer 7 polish — `--help` content, TTY-detected color emission in `cmd_list` / `cmd_show` / `format_show_block`, error specificity. **Layer 7 is presentation-only; data layer untouched.** Cold session.

### Diff evidence that the data layer is untouched

Range: `8962b7f..603c689` (Layer 7 inclusive of Red Gate, implementation, and manual closure).

- `git diff 8962b7f..603c689 --stat`: 4 files touched — `CHANGELOG.md`, `TODO.md`, `src/lib.rs`, `tests/layer7.rs`. **`src/main.rs` diff: 0 lines.**
- `git diff 8962b7f..603c689 -- src/lib.rs | grep -E '(struct Tracker|struct Issue|fn load_tracker|fn save_tracker|fn issue_fields_are_valid|fn tracker_is_valid|fn parse_|fn validate_|next_id|PRIORITY_ORDER|STATUS_ORDER|serde|Serialize|Deserialize)'`: **0 matches.** No diff line touches any data-layer identifier.
- The `src/lib.rs` net additions are: `use std::io::IsTerminal;`, four pure presentation helpers (`priority_ansi`, `status_ansi`, `wrap_color`, `pad_after_color`), one new constant (`ANSI_RESET`), a `use_color` parameter threaded into `format_show_block`, and two `is_terminal()` decisions in `cmd_list` / `cmd_show`. None of these read, write, validate, serialize, or migrate `tracker.json`.
- The `\r\n` → `\n` description normalization (`src/lib.rs` line 541) is unchanged. The hardcoded path (`src/main.rs:84`), `Tracker::next_id` field, `bump_next_id`, `PRIORITY_ORDER`, `STATUS_ORDER`, `issue_fields_are_valid`, `tracker_is_valid`, `load_tracker`, `save_tracker`, and the `#[serde(skip_serializing_if = "Option::is_none")]` attribute on `description` are byte-identical to the post-Layer-6 R3 state.

### Regression check (whole-suite verification)

`cargo test` against `603c689`: **9/9 layer7 tests pass; 195 tests total across the full suite pass; 0 failures.** Test counts by binary (`cargo test 2>&1 | grep "test result"`): 62 + 32 + 18 + 9 + 25 + 7 + 33 + 9 + doc 0.

Empirical round-trip (cold, fresh `tracker.json` in `/tmp/de-r11`):
- `create "First" --priority high` → id=1, `next_id: 2`.
- `create "Second" --priority medium` → id=2, `next_id: 3`.
- `create "Third" --priority low` → id=3, `next_id: 4`.
- `delete 2` → file contains `{issues: [{id:1},{id:3}], next_id: 4}`. **`next_id` not regressed — persistent counter from SO R22 closure (Layer 6 R3) intact.**
- `create "Fourth"` → id=4 (not 3 — deleted ID not reused), `next_id: 5`. SO R22 invariant holds across Layer 7.

### Sycophancy probe — color emission as a derivative of data

**Probe:** can a corrupted `tracker.json` planting an ANSI sequence inside `status` or `priority` cause `cmd_show` / `cmd_list` to emit unescaped ANSI through the new color helpers?

**Test (`/tmp/de-r11/tracker.json` hand-edited to `"status": "high[31m"`):**
```
$ ./tracker list
Error: Could not read tracker data. The file may be corrupt. Delete tracker.json to start fresh.
EXIT=1
```

`issue_fields_are_valid` (`src/lib.rs:226`) checks `STATUS_ORDER.contains(&issue.status.as_str())` and `PRIORITY_ORDER.contains(&issue.priority.as_str())`. `"high\x1b[31m"` is not in either enum slice, so `tracker_is_valid` rejects at load time before `priority_ansi` / `status_ansi` ever see the value. The two functions in `src/lib.rs:48-76` then receive only the validated `&issue.status` and `&issue.priority`, both of which are guaranteed members of `STATUS_ORDER` / `PRIORITY_ORDER`. The data-to-presentation mapping is **airtight by load-time enum validation**, not by anything the new color helpers themselves do. Defense holds.

The `description` field is rendered with the existing `\r\n` → `\n` normalization and 13-space indent (unchanged from Layer 6 R2). `description_is_valid` continues to reject Cc-other-than-`\n` at load (DE R9 F1 / R10 closure), so a description cannot smuggle ANSI either.

### Findings by classification

**Resolved:** *(none — no fix needed)*
**Deferred:** *(none)*
**Dismissed:** *(none)*
**Hallucinated:** *(none)*
**Raised to SO:** *(none)*

Total findings: **0 substantive.** Per IAR README, a domain with zero findings is a valid outcome — the data layer was not modified by Layer 7, the existing validation invariants continue to protect the new presentation surface, and the persistent-counter regression check passes.

### Cross-domain flags

- No PII / sensitive-data surface change. `title` / `description` are still free-form user text; TTY-only color emission writes nothing back to storage. No escalation to [PRIVACY-REVIEW.md](PRIVACY-REVIEW.md).
- The escape-injection-via-storage defense rests on `STATUS_ORDER` / `PRIORITY_ORDER` enum membership at load (`tracker_is_valid`) plus the description Cc check (DE R9 F1). [SECURITY-REVIEW.md](SECURITY-REVIEW.md) / [RED-TEAM-REVIEW.md](RED-TEAM-REVIEW.md) may wish to independently verify the same probe with their own adversarial framing — flagging only because the new color path makes the "storage-to-TTY" pipeline newly load-bearing for output-channel safety, not because a defect was found here.
- No new domain proposal.

### Summary

Layer 7 is presentation-only. Diff evidence (0 lines in `src/main.rs`; 0 grep matches for data-layer identifiers in the `src/lib.rs` diff) confirms `Tracker`, `Issue`, `load_tracker`, `save_tracker`, `issue_fields_are_valid`, `tracker_is_valid`, `next_id`, and the `description` Cc validator are byte-identical to post-Layer-6-R3. Whole-suite tests pass (195/195). The empirical create/delete/create round-trip preserves the persistent `next_id` counter (SO R22 invariant intact). The corrupt-status ANSI-injection probe is rejected at `load_tracker` by enum-membership validation, so the new color helpers never receive untrusted values. Schema evolution: no change to evaluate. **0 substantive findings; MVR reached for Layer 7 DE-domain on Round 1.**

**Coordination:** *(none required; informational cross-reference to SECURITY-REVIEW.md / RED-TEAM-REVIEW.md noted above.)*

**Files modified:** Only this log appended.

---

## Review 12 — 2026-05-12 00:00Z

**Round:** DE Review 12 (Layer 7 IAR Round 2 closure pass). Warm verification per CLOSURE-PROTOCOL.md §5; not a new adversarial round.

**Scope:** Verify that Round-2 substantive commit `09b1905` and the R1 retrofit `fbbb8a3` preserve the R11 closure: Layer 7 is presentation-only; data layer untouched.

### Round-1 finding closures

- **R11 had 0 substantive findings.** Closure-pass scope is a regression check only.

### Data-layer regression check (R2 commits)

`git diff 01208f1 HEAD -- issue-tracker-cli/src/lib.rs | grep -E '(Tracker|Issue|load_tracker|save_tracker|parse_status|parse_priority|parse_label|parse_id|validate_title|validate_description|issue_fields_are_valid|tracker_is_valid|bump_next_id|next_id|PRIORITY_ORDER|STATUS_ORDER|VALID_STATUSES|serde|Serialize|Deserialize)'` — verified no non-doc edits in the data-layer code paths. The R2 commit `09b1905` added:

- `ColorMode` enum, `color_mode_from_env`, `render_cell`, `sanitize_quoted_values`, `wrap_color` debug_assert! — all presentation / safety helpers.
- `display_safe` exposed `pub` — accessibility change only, no behavioral.
- `cmd_show` / `cmd_list` signatures gain `color: ColorMode` — control-flow plumbing, no data-layer effect.
- Bold-redundancy color values in `priority_ansi` / `status_ansi` — output-byte change only; the underlying `status` / `priority` field values are unchanged.
- DESIGN.md amendments confined to "Interface / Color output", "stderr contract", and Permission-denied error wording — none touch storage shape or validation rules.

### Defense-in-depth coordination (informational)

The new `wrap_color` debug_assert! pattern (Security R12 closure) is the right shape for data-layer-to-presentation boundary verification: it pins the contract that a colored value must have already been validated by the data-layer's closed-enum check. `issue_fields_are_valid` is the upstream defense; `wrap_color`'s debug_assert! is the downstream sanity check. Defense-in-depth is cleanly layered.

### Storage round-trip verification

Spot-test via integration suite: `cargo test --test layer6 -- create_with_description_stores_verbatim show_displays_all_fields delete_id_not_reused_high_edge` — all pass at HEAD. The persistent `next_id` counter invariant (SO R22 closure) is regression-checked by the existing tests; no R2 edits could have affected it.

### New findings

*(none — closure pass, presentation-only confirmed.)*

### Summary

R11's 0-findings MVR is maintained at HEAD. R2 changes are confirmed presentation-only by diff inspection. Data layer (storage shape, validation predicates, parse_* helpers, next_id counter invariants) is byte-for-byte unchanged. No DE-domain action required.

**Coordination:** Security R12 — informational note on `wrap_color` debug_assert! defense-in-depth layering.

**Files modified:** Only this log appended.

---

## Review 13 — 2026-05-12 12:00Z

**Round:** DE Review 13 (Layer 7 IAR Round 3 — module-split review). Cold session.

**Scope:** Layer 7 R3 change set, primary commit `8db9437` — three-module split of `src/lib.rs` into `storage.rs` + `validate.rs` + `commands.rs`. **The data layer is now a dedicated module (`src/storage.rs`); this is the first review where storage has a module of its own**, so the DE review focuses on whether the extraction is clean (sycophancy posture: "data layer cleanly extracted" is a positive-sounding claim — verify, don't confirm). Other R3 commits (`ff0e85c` clippy hook, `c341a54` CJK debug_assert, `bd7511e` force_color seam, `3fa1f3c` cmd_list extraction) are non-data-layer.

### Whole-suite regression check

`cargo test` at HEAD (`8db9437`): **all bins pass.** Test count by binary: 93 unit + 32 + 18 + 9 + 25 + 7 + 33 + 20 + doc 0 = **237 tests, 0 failures.** Build (`cargo build --release`) is clean.

### Data-layer extraction audit (R3 primary pressure point)

Compared `git show b853a81:issue-tracker-cli/src/lib.rs` (pre-split) against `src/storage.rs` (post-split):

- **serde derives present.** `#[derive(Debug, Serialize, Deserialize)]` on both `Tracker` and `Issue`, with `#[serde(skip_serializing_if = "Option::is_none")]` on `Issue::description`. Identical to pre-split.
- **I/O entry points isolated.** `load_tracker` and `save_tracker` are the only fs-touching functions in the crate; both live in `storage.rs`. `grep "fs::" src/{commands,validate}.rs` → zero matches. The data-I/O boundary is the module boundary.
- **Load-time invariants co-located with types.** `tracker_is_valid`, `issue_fields_are_valid`, `description_is_valid`, `label_is_valid`, `parse_timestamp`, `CORRUPT_DATA_ERROR` all live in `storage.rs` alongside `Tracker`/`Issue` — the "validate what we deserialize" concern is now in the same module as the deserialization target. The split rationale in the module-doc comment ("data model + load-time invariants" vs "user-input validation" vs "commands") is consistent with what was actually moved.
- **Domain enums as single source of truth.** `VALID_STATUSES` and `PRIORITY_ORDER` live in `storage.rs`; `validate.rs` imports them via `use crate::storage::{PRIORITY_ORDER, VALID_STATUSES}` (validate.rs line 26); `commands.rs` imports `PRIORITY_ORDER` for sort ranking. No second copy exists. `grep -rn '\"open\", \"in-progress\", \"done\"\|\"high\", \"medium\", \"low\"' src/` → only the two definitions in `storage.rs`.

### Semantic-change probe (no-behavior-change claim verification)

Per the commit message, the split is "pure code reorganization." Verified by comparing pre/post-split:

- `tracker_is_valid` body byte-for-byte identical to pre-split (lines 280-289 of `/tmp/lib_pre_split.rs` vs lines 154-171 of `storage.rs`): `next_id < 1` check first, then per-record validity, then HashSet uniqueness, then the `next_id <= max_id` check inside `if let Some(max_id)`. Identical control flow. **SO R22 invariant byte-stable across the split.**
- `issue_fields_are_valid` body identical: same conjunct order, same `is_none_or` description check, same `updated_at >= created_at`.
- `description_is_valid`: `!trim().is_empty() && !chars.any(|c| c.is_control() && c != '\n')` — Cc-other-than-`\n` rule from DE R9 F1 / Layer 6 R2 is byte-stable.
- `label_is_valid`: identical predicate (empty, control, comma).
- `load_tracker` / `save_tracker`: identical bodies (error formatting, fresh-tracker default `{issues: [], next_id: 1}`, `serde_json::from_str` → `tracker_is_valid` chain).
- Diff of declared items reduces to: items split across three files, and crate-internal visibility upgraded from bare-`fn` to `pub(crate) fn` so the test module in `lib.rs` and sibling modules can reach them. No body changes.

### Module-boundary acyclicity probe

`grep -n 'use crate::' src/storage.rs` → 0 matches. `src/validate.rs` imports only from `storage`. `src/commands.rs` imports from `storage` + `validate`. The dependency graph is: `storage` (leaf) ← `validate` ← `commands` (root). Acyclic; the data layer has no upward dependencies — correct posture for a data-layer module.

### Empirical round-trip (cold, fresh `tracker.json` in `/tmp/der13`)

Release binary at HEAD, no prior data:

1. `tracker create "Test" --description $'Multi\nline'` (real LF planted via `printf`) → `Created issue #1: Test`.
2. `tracker show 1` renders:
   ```
   Description: Multi
                line
   ```
   First line on the same row as the label; continuation indented exactly 13 spaces. Round-trip of `\n` through serde → file → load → render is intact.
3. `cat tracker.json` shows `{"issues": [{"id": 1, "title": "Test", "description": "Multi\nline", ...}], "next_id": 2}`. **The `next_id: 2` field is present** — the SO R22 storage shape persisted, not the pre-R22 bare array.
4. `tracker delete 1` → `Deleted issue #1.` File becomes `{"issues": [], "next_id": 2}` (counter unchanged by delete).
5. `tracker create "Next"` → assigned id=**2** (not 1), `next_id: 3`. The deleted high-edge id is not reused — SO R22 invariant holds across the new module structure.

### Description Cc-defense regression probe (Layer 6 R2 lineage)

Two-layer defense intact:

- **Parse-time:** `validate::validate_description` (validate.rs lines 72-82) rejects Cc-other-than-`\n`. Unit tests `description_with_control_char_other_than_newline_is_rejected` (`a\u{1B}b`, `a\u{07}b`, `a\u{00}b`, `a\u{7F}b`, `a\tb`, `a\rb`, `line1\r\nline2`) all pass at HEAD.
- **Load-time:** `storage::description_is_valid` (storage.rs lines 133-135) wired via `issue_fields_are_valid` line 120. Unit tests `issue_field_validation_rejects_control_char_in_description` and `issue_field_validation_rejects_carriage_return_in_description` pass. Newline carve-out preserved (`issue_field_validation_accepts_newline_in_description`).

### Per-DE-dimension audit

1. **Data model correctness** — `Tracker` (`issues: Vec<Issue>`, `next_id: u64`) and `Issue` (id, title, description Option, status, priority, labels, created_at, updated_at) match DESIGN.md Data Model section unchanged. No field type or optionality drift across the split.
2. **Validation and normalization** — Two-tier defense unchanged: input boundary (`validate.rs`) and load boundary (`storage.rs`). The split moved code but did not weaken either tier.
3. **Schema evolution** — Pre-SO-R22 bare-array shape still rejected at load (commit message references this; the `let tracker: Tracker = serde_json::from_str(...)` line is unchanged from `b853a81`). No forward-compat regression.
4. **Data integrity invariants** — `next_id > max(issue.id)` check byte-stable in `tracker_is_valid`; HashSet uniqueness byte-stable; per-issue invariants unchanged. Round-trip probe confirms behavior end-to-end.
5. **Storage fitness** — JSON file, direct write, single-user CLI. Out-of-scope items (atomic writes, file locking) explicitly noted in DESIGN.md, unchanged by R3.
6. **Access patterns** — `cmd_create` / `cmd_status` / `cmd_delete` each call `load_tracker` then `save_tracker`. Read-then-write pattern unchanged from pre-split (verified by grep — `load_tracker` callers and `save_tracker` callers identical).
7. **Serialization and deserialization** — `serde_json::to_string_pretty` for write; `serde_json::from_str` for read; `#[serde(skip_serializing_if = "Option::is_none")]` on description verified by round-trip (`tracker.json` for id 2 has no description field — `None` correctly omitted, not serialized as `null`).
8. **Data consistency** — Single-process, single-writer model. R3 introduces no new write site.
9. **Sensitive data handling** — Title/description are free-form user text; same posture as prior reviews. No PII surface change.
10. **Test coverage of data paths** — All pre-split DE tests survive the split (visible in `lib.rs` `#[cfg(test)] mod tests` block — 93 unit tests, including `tracker_validation_*` family, `issue_field_validation_*` family, `description_*` family). The split did not break test reachability of `pub(crate)` items because the test module is at the lib.rs hub level with `use crate::storage::*; use crate::commands::*;` glob imports.
11. **Data volume limits** — Unchanged; single-user CLI, no documented hard cap. Out-of-scope per prior reviews.

### Sycophancy probe — "data layer cleanly extracted" claim verification

The claim sounds positive. Concrete checks against the claim, each independently:

- **Q: Is everything data-related actually in storage.rs?** A: Yes — `Tracker`, `Issue`, both I/O entry points, all load-time predicates, the parse_timestamp helper, and the domain enums. The one ambiguity is `bump_next_id` (in validate.rs, not storage.rs) — but `bump_next_id` is pure arithmetic on a `u64` that happens to be the counter type; it does not touch the storage struct or any field. Co-locating it with the rest of the input-side helpers in `validate.rs` is defensible. **Not a finding** — but I considered it, and the placement is justified rather than reflexively approved.
- **Q: Did anything data-related leak into the other modules?** A: `commands.rs` imports `PRIORITY_ORDER` for sort ranking and `Issue` for type signatures. Neither is a data-layer concern crossing the boundary — sort rank is a presentation concern that happens to share the priority enum, and `Issue` is the data type the commands operate on. No leakage.
- **Q: Are there two copies of any storage constant?** A: No (grep verified above).
- **Q: Could the split have silently changed a predicate?** A: Pre/post-split byte comparison of all five load-time predicates confirms identity.

The extraction is, on the evidence, clean. The claim is verified, not merely accepted.

### Carry-forward findings

- DE R11 had 0 substantive findings; DE R12 had 0 substantive findings (closure pass); DE R13 produces 0 substantive findings. **MVR maintained across three consecutive cold/warm rounds at the data layer.**

### Findings by classification

**Resolved:** *(none — no fix needed.)*
**Deferred:** *(none.)*
**Dismissed:** *(none.)*
**Hallucinated:** *(none — no finding was raised and then retracted; I considered the `bump_next_id` placement as a "should this be in storage?" prompt and concluded the current placement is justified before promoting it to a finding.)*
**Raised to SO:** *(none.)*

Total findings: **0 substantive.**

### Cross-domain flags

- **SOLUTION-ARCHITECT-REVIEW.md** — module split is the SA R13 F1 Trigger B closure; from the data-layer perspective the extraction is clean. SA's own dimensions (cognitive load, module-boundary clarity, public API stability) are the primary review surface for this commit and live in their domain.
- **QUALITY-ENGINEER-REVIEW.md** — full suite green at HEAD (237/237); the test reachability of `pub(crate)` items from the lib.rs test module is intact. No QE escalation from the data side.
- **SECURITY-REVIEW.md** / **RED-TEAM-REVIEW.md** — load-time enum validation (the "ANSI-in-status field" defense from DE R11 probe) is preserved byte-for-byte. No new attack surface introduced by the module split.
- No PII / privacy concern raised by the reorganization.
- No new IAR domain proposal.

### Summary

The R3 module split moves data-layer code into `src/storage.rs` as a dedicated module. The extraction is verified clean on the evidence: serde derives present, I/O isolated, load-time predicates co-located with types, domain enums single-sourced from storage, dependency graph acyclic with storage as the leaf. Pre/post-split byte comparison of `tracker_is_valid`, `issue_fields_are_valid`, `description_is_valid`, `label_is_valid`, `parse_timestamp`, `load_tracker`, and `save_tracker` confirms no semantic change. The empirical round-trip (create with `\n` description → show → delete-high-edge → create) preserves the SO R22 persistent-counter invariant and the description `\n` carve-out end-to-end. Description Cc-defense (parse-time in validate.rs, load-time in storage.rs) is intact. **0 substantive findings; MVR maintained at the data layer across R11/R12/R13.**

**Coordination:** SA R13 F1 Trigger B — module split is SA's primary concern at R3; data layer signs off cleanly. QE — full test suite green (237/237) at HEAD; no DE-side test gap surfaced by the split. Security / RT — informational: the load-time enum-validation defense for storage-to-TTY safety (DE R11 sycophancy probe) is byte-stable across the split.

**Files modified:** Only this log appended.


