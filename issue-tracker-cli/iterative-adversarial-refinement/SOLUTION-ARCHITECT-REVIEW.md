# Solution Architect Review Log

This review is part of the [Iterative Adversarial Refinement (IAR)](README.md) suite. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

The purpose of this review is to evaluate whether the architecture — its structure, boundaries, decisions, and tradeoffs — is sound, coherent, and appropriate for the project's stated purpose and constraints. Every review targets the whole application, not only the most recently changed code.

---

## Review 1 — 2026-04-27 15:00Z

**Scope:** DESIGN.md pre-build architecture review. No implementation exists yet. Primary lens: SA dim 9 (complexity budget) — does the spec encode architectural decisions whose implementation cost is proportionate to the project's maintenance model (a single Phase 1 apprentice building their first Rust project)?

**Session note:** Reviewed in-session with spec authorship. Acknowledged quality tradeoff.

**Coordination:** Finding 1 (atomic writes), Finding 2 (exit codes), and Finding 3 (`next_id`) are spec-level decisions that, if changed, require corresponding updates to DESIGN.md before Layer 1 opens. Cross-reference with [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) for the spec compliance angle.

---

### Resolved

**Finding 1 — Atomic writes are a production-grade constraint for a personal learning tool**

`DESIGN.md` Constraints section: "Atomic writes. Every mutation writes `tracker.json.tmp` then renames it. This is a hard requirement, not a polish item."

The assignment's security guidance says: "Handle the case where the JSON file is missing or contains invalid data without crashing." It does not require crash-safe mutation. Atomic writes via temp-file-and-rename are correct engineering for a production service. For a single-user CLI running one command at a time, the failure scenario (Ctrl-C mid-write) is rare and recoverable by deleting `tracker.json`. The implementation cost — `fs::rename`, two-stage error propagation, temp file cleanup — is real overhead for an apprentice writing their first Rust file I/O.

The complexity is not proportionate to the maintenance model (single developer, personal tool, no external users).

**Resolution:** Removed atomic write constraint from Constraints section and all feature postconditions. Storage model now states writes go directly to `tracker.json`; on failure the file may be in an indeterminate state. Atomic writes noted in Out of Scope with rationale.

---

**Finding 2 — Exit code 2 is a scripted-caller contract; this tool has no scripted callers**

`DESIGN.md` Interface section: exit code `2` for I/O errors, distinct from exit code `1` for user errors.

Exit code tiers are meaningful only to a process that checks `$?` and branches on the specific value. The assignment shows interactive terminal use by a person, not composition in a pipeline or CI script. No caller is identified. The two-tier exit code contract adds a testable interface obligation — integration tests must assert the specific exit code for each I/O failure — with no identified beneficiary.

**Resolution:** Collapsed to exit 0/1 throughout. All storage error states updated from exit 2 to exit 1. Added to Out of Scope with rationale.

---

**Finding 3 — `next_id` counter encodes a non-reuse guarantee the assignment does not require**

`DESIGN.md` Data Model storage file: `"next_id": u64` with invariant `next_id > max(id)` always.

The simpler approach: `max(existing_ids) + 1` computed at create time. For a flat JSON file with tens of issues, this is instantaneous. The `next_id` field introduces a storage-level invariant that must be maintained across all writes — and introduces a new failure mode: if `next_id` falls out of sync with actual issue IDs (manual file edit, bug in write logic), the tracker silently assigns a duplicate or skips IDs. ID non-reuse after deletion is meaningful when IDs are referenced externally (foreign keys, logs, URLs). In this tool, IDs appear only in the tracker's own output.

**Resolution:** Removed `next_id` from the storage file shape. ID assignment now specified as `max(existing_ids) + 1`, or `1` if the issue list is empty. Delete invariant updated accordingly. Purity boundary note updated from "`next_id` arithmetic" to "`max(existing_ids) + 1` logic".

---

**Finding 4 — Dynamic column widths add two-pass table rendering to a display concern**

`DESIGN.md` Interface section: "Column widths are determined by the widest value in each column (including the header)."

A dynamic-width table requires collecting all matching rows, computing per-column maxima, then rendering. It is two passes over the data. Fixed-width columns — ID padded to 4, Status to 11, Priority to 8, Labels truncated at 20, Title consuming the remainder — produce equivalent readability with a single pass and predictable behavior. The assignment specifies a tabular list; it does not require dynamic width calculation.

**Resolution:** Replaced dynamic-width specification with fixed column widths: ID 4, Status 11, Priority 8, Labels 20, Title 50. Both columns truncate with `…` at their limits. `show` always displays full values.

---

**Finding 5 — `\r\n` normalization is speculative cross-platform scope**

`DESIGN.md` Edge Cases / Description: "`\r\n` line endings are normalized to `\n` on storage."

The target platform is macOS (Darwin). macOS terminal input does not produce `\r\n`. This normalization handles a case that cannot arise on the target platform, adds a transformation to user-supplied data that is not visible to the user, and is not motivated by any identified failure mode. If Windows support is added later, the normalization can be introduced then with a known need.

**Resolution:** Removed. No Windows target in scope; the normalization step had no identified failure mode on the target platform (macOS).

---

### Observation — Purity boundary is methodology overhead, not assignment scope

The purity boundary section (naming `validate_title`, `issue_matches_filters`, `format_issue_row`, etc. as formally pure) is required by the spec-crystallization primer, not by the assignment. The assignment's learning goals are Rust syntax, CLI design, state machines, and serialization — not VSDD Phase 5 formal verification preparation.

This is not a finding. The purity boundary is a correct application of the VSDD methodology the project is following. It is noted here as a recognized tension: the methodology adds architectural structure that exceeds the assignment's learning objectives. The human director should decide whether this structure is appropriate for this stage of the program.

---

### Dismissed

*(none)*

---

---

## Review 2 — 2026-04-27 21:00Z

**Scope:** `TODO.md` layered decomposition. Evaluating whether the layer structure has sound architectural boundaries, correct ordering, and does not introduce refactoring debt between layers.

**Session note:** In-session with all other domain reviews. Acknowledged quality tradeoff.

---

### Resolved

**Finding 1 — Layer 1 sort description implied a simplified sort algorithm (Dim 9 — complexity budget)**

`TODO.md` Layer 1 acceptance criteria stated: "List output is sorted by ID ascending (ties in priority — all medium at this layer)." The parenthetical explains the effective behavior (ID ascending) without requiring the full sort algorithm. An implementer reading only this could implement a simple ID-ascending sort and then need to refactor the sort algorithm in Layer 3 when priorities are introduced. This is structural debt baked into the decomposition.

The correct approach: implement the full priority→ID sort algorithm from Layer 1. Since all issues are medium priority at Layer 1, the effective output is ID ascending — but the code uses the full algorithm. Layer 3 then adds the `--priority` filter and the sort is already correct.

**Resolution:** Updated `TODO.md` Layer 1 acceptance criteria to explicitly require the full sort algorithm: "uses the full sort algorithm (priority descending then ID ascending within tier); since all issues default to `medium`, the effective order is ID ascending. The sort algorithm must be the full algorithm from the start — not a simplified ID-only sort."

---

### Dismissed

**Finding 2 — Layer 5 (compound filtering) as a standalone layer**

Is a separate layer for compound filtering architecturally justified, or is it a test-only concern folded into Layer 4?

**Classification:** Dismissed. Compound filter AND-logic is a distinct behavioral requirement: it verifies that three independently-implemented filters compose correctly, and that the no-match message is correct for each combination. Testing this in Layer 4 would require all three filters to exist in Layer 4, which would pull priority filtering (Layer 3) and status filtering (Layer 2) forward out of their natural sequence. Layer 5 is the correct place to verify inter-filter composition once all filters exist. The layer is small but necessary.

---

**Finding 3 — Layer 6 combines description, show, and delete — possibly too broad**

Layer 6 delivers three distinct capabilities: `--description` on create, `tracker show`, and `tracker delete`. Is this one layer or three?

**Classification:** Dismissed. All three are tightly coupled: `--description` is only meaningful once `tracker show` exists to display it fully; `tracker delete` is naturally paired with `tracker show` (the assignment's Layer 6 groups them explicitly). The acceptance criteria for all three fit in a single verifiable layer — a user can create with a description, show the full details, and delete an issue. None of the three creates a standalone capability without the others.

---

### Open

*(none)*

---

### Summary

One real finding resolved (Layer 1 sort algorithm specification). Two dismissed. The decomposition is architecturally sound: each layer delivers an independent, verifiable capability; layer ordering is correct (each layer depends only on previous layers); no structural debt is introduced between layers.

---

---

## Review 3 — 2026-04-27 22:00Z

**Scope:** Layer 1 stub architecture — `Cargo.toml`, `src/main.rs`, `src/lib.rs`, `tests/layer1.rs`. Evaluating structural decisions, API boundary, and whether the stub introduces any architectural debt before implementation begins.

**Session note:** In-session with all other Layer 1 domain reviews. Acknowledged quality tradeoff.

---

### Dismissed

**Finding 1 — Library/binary split (`lib.rs` / `main.rs`) adds a crate boundary for a small binary (Dim 9 — complexity budget)**

The project declares both `[[bin]]` and `[lib]` targets in `Cargo.toml`, creating a `lib.rs` library and a `main.rs` binary that uses it. For a small CLI tool, this is more structure than a `main.rs`-only approach.

**Classification:** Dismissed. The split is architecturally correct and not premature. The unit tests in `lib.rs` require the library target — integration tests in `tests/` test the binary via subprocess, but unit tests for `validate_title` and `next_id` must compile the library directly. Without a `[lib]` target, unit tests would have to be in `main.rs` as `#[test]` functions, which is a known antipattern for CLI binaries (it pollutes the binary entry point and makes it harder to test in isolation). The split is justified by the testing architecture.

---

**Finding 2 — The `tracker()` helper in `tests/layer1.rs` is not extracted to a shared test helper module (Dim 4)**

The `tracker()` function will be duplicated in every `tests/layerN.rs` file. It could be in a `tests/common/mod.rs` shared module.

**Classification:** Dismissed. Premature abstraction. Only one test file exists. The helper is three lines. When `tests/layer2.rs` exists, the duplication is two instances — below the threshold for extraction. Extract when Layer 3 or Layer 4 introduces a third file, or when the helper grows beyond argument-building (e.g., if it gains environment variable setup).

---

**Finding 3 — `validate_title` returns `Result<String, String>` rather than a richer error type (Dim 3)**

See SE Review 2 Finding 2.

**Classification:** Dismissed. Cross-reference with SE Review 2 Finding 2 — the dismissal rationale applies architecturally as well. `String` errors are appropriate for a tool whose only error-handling action is printing to stderr. No error variant routing is needed.

---

### Open

*(none)*

---

### Summary

Three dismissed findings — all hallucinated or premature concerns. The stub architecture is minimal, correct, and non-debt-generating. Library/binary split is justified by the testing architecture. No new structural findings.

---

---

## Review 4 — 2026-04-28 05:30Z

**Scope:** Layer 1 full implementation — `src/lib.rs`, `src/main.rs`, `Cargo.toml`. Evaluating the implemented architecture: module structure, data flow, separation of concerns, and whether the implementation introduces any architectural debt for future layers.

**Session note:** In-session with Layer 1 IAR suite. Acknowledged quality tradeoff.

---

### Dismissed

**Finding 1 — `VALID_STATUSES` and `VALID_PRIORITIES` are hardcoded string slice constants rather than enums (Dim 3 — Type safety)**

The implementation uses `const VALID_STATUSES: &[&str] = &["open", "in-progress", "done"]` for validation rather than a `Status` enum. `Issue.status` is typed as `String`. An enum would make invalid status values unrepresentable after deserialization.

**Classification:** Dismissed. Layer 1 does not implement status change or status parsing from user input — all new issues receive the hardcoded `"open"`. The `VALID_STATUSES`/`VALID_PRIORITIES` constants are used only in the post-deserialization corruption check. Introducing enums now would require custom serde implementations and add complexity before the feature that needs the enum exists (Layer 2 status change, Layer 3 priority parsing). The correct time to introduce enums is when the parsing layer is implemented. At that point, the type system can enforce validity rather than runtime checks. No architectural debt at Layer 1 — the string-based model is explicitly temporary and the validation constants are already in place.

---

**Finding 2 — `cmd_create` and `cmd_list` are in `lib.rs`, not a commands module (Dim 4 — Cohesion)**

The library module contains both data model types, pure functions (`validate_title`, `next_id`), I/O functions (`load_issues`, `save_issues`), and command handlers (`cmd_create`, `cmd_list`). As more commands are added, this module will grow.

**Classification:** Dismissed at Layer 1. Two commands do not justify module decomposition. When five commands exist (Layer 6 implementation complete), splitting into `lib/storage.rs`, `lib/validate.rs`, and `lib/commands.rs` would be appropriate. The SA Review 3 principle (don't extract until three instances exist) applies here. Layer 6 is the right point to revisit module structure.

---

**Finding 3 — `priority_rank` returns `usize::MAX` for unknown priorities (Dim 1 — defensive fallback)**

Reviewed in SE Review 3 / SA Review 4 context. With post-deserialization validation now in place (`VALID_PRIORITIES.contains(&issue.priority.as_str())`), any issue loaded from storage has a valid priority. The `usize::MAX` fallback in `priority_rank` is now structurally unreachable for issues from storage — it would only trigger for issues constructed incorrectly in code, which doesn't happen. The fallback remains a safe defensive pattern.

**Classification:** Dismissed. The `usize::MAX` fallback is now doubly safe: the validation gate prevents invalid priorities from reaching `priority_rank`, and the fallback handles the hypothetically-impossible case. No action needed.

---

### Open

*(none)*

---

### Summary

Three dismissed findings. The Layer 1 architecture is clean and appropriate. The `lib.rs` structure will require module decomposition at Layer 6 — this is the correct time to do it, not now. Enum-based status/priority types are deferred to Layer 2/3. No architectural debt introduced at Layer 1.

---

---

## Review 5 — 2026-04-30 00:00Z

**Scope:** Layer 1 gate closure pass — no code changes since Review 4.

**Session note:** In-session with all other domain reviews. Acknowledged quality tradeoff.

---

### Dismissed

No architectural concerns. The Layer 1 architecture is unchanged and all prior findings remain resolved. **No SA findings.** MVR reached for Layer 1.

