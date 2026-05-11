# Solution Architect Review Log

This review is part of the [Iterative Adversarial Refinement (IAR)](README.md) suite. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

**Reviewer role: Solution Architect** (Solution Architect / Software Architect / Technical Lead)

The purpose of this review is to evaluate whether the architecture — its structure, boundaries, decisions, and tradeoffs — is sound, coherent, and appropriate for the project's stated purpose and constraints. Every review targets the whole application, not only the most recently changed code.

**Language supplement applied:** `lang/rust.md` (SA section).

**Sycophancy check:** An agent that designed the architecture will find it sound because it reflects its own training distribution and defaults, not because it is right for this project's constraints. Push hardest on dim 9 (complexity budget) and dim 8 (technology fitness): these are the dimensions where agent defaults most consistently diverge from what a single maintainer or small project actually needs. For each technology choice and architectural pattern, ask: "would this choice have been made by a human engineer working alone on a project of this scope, or is it a team-scale default?"

---

## Review 1 — 2026-04-27 15:00Z

**Scope:** DESIGN.md pre-build architecture review. No implementation exists yet. Primary lens: SA dim 9 (complexity budget) — does the spec encode architectural decisions whose implementation cost is proportionate to the project's maintenance model (a single Phase 1 apprentice building their first Rust project)?

**Session note:** Reviewed in-session with spec authorship. Acknowledged quality tradeoff.

---

### Resolved

**Finding 1 — Atomic writes are a production-grade constraint for a personal learning tool (Dim 9)**

`DESIGN.md` Constraints section: "Atomic writes. Every mutation writes `tracker.json.tmp` then renames it. This is a hard requirement, not a polish item."

The assignment's security guidance says: "Handle the case where the JSON file is missing or contains invalid data without crashing." It does not require crash-safe mutation. Atomic writes via temp-file-and-rename are correct engineering for a production service. For a single-user CLI running one command at a time, the failure scenario (Ctrl-C mid-write) is rare and recoverable by deleting `tracker.json`. The implementation cost — `fs::rename`, two-stage error propagation, temp file cleanup — is real overhead for an apprentice writing their first Rust file I/O.

The complexity is not proportionate to the maintenance model (single developer, personal tool, no external users).

**Resolution:** Removed atomic write constraint from Constraints section and all feature postconditions. Storage model now states writes go directly to `tracker.json`; on failure the file may be in an indeterminate state. Atomic writes noted in Out of Scope with rationale.

---

**Finding 2 — Exit code 2 is a scripted-caller contract; this tool has no scripted callers (Dim 4, Dim 9)**

`DESIGN.md` Interface section: exit code `2` for I/O errors, distinct from exit code `1` for user errors.

Exit code tiers are meaningful only to a process that checks `$?` and branches on the specific value. The assignment shows interactive terminal use by a person, not composition in a pipeline or CI script. No caller is identified. The two-tier exit code contract adds a testable interface obligation — integration tests must assert the specific exit code for each I/O failure — with no identified beneficiary.

**Resolution:** Collapsed to exit 0/1 throughout. All storage error states updated from exit 2 to exit 1. Added to Out of Scope with rationale.

---

**Finding 3 — `next_id` counter encodes a non-reuse guarantee the assignment does not require (Dim 3, Dim 9)**

`DESIGN.md` Data Model storage file: `"next_id": u64` with invariant `next_id > max(id)` always.

The simpler approach: `max(existing_ids) + 1` computed at create time. For a flat JSON file with tens of issues, this is instantaneous. The `next_id` field introduces a storage-level invariant that must be maintained across all writes — and introduces a new failure mode: if `next_id` falls out of sync with actual issue IDs (manual file edit, bug in write logic), the tracker silently assigns a duplicate or skips IDs. ID non-reuse after deletion is meaningful when IDs are referenced externally (foreign keys, logs, URLs). In this tool, IDs appear only in the tracker's own output.

**Resolution:** Removed `next_id` from the storage file shape. ID assignment now specified as `max(existing_ids) + 1`, or `1` if the issue list is empty. Delete invariant updated accordingly. Purity boundary note updated from "`next_id` arithmetic" to "`max(existing_ids) + 1` logic".

---

**Finding 4 — Dynamic column widths add two-pass table rendering to a display concern (Dim 9)**

`DESIGN.md` Interface section: "Column widths are determined by the widest value in each column (including the header)."

A dynamic-width table requires collecting all matching rows, computing per-column maxima, then rendering. It is two passes over the data. Fixed-width columns — ID padded to 4, Status to 11, Priority to 8, Labels truncated at 20, Title consuming the remainder — produce equivalent readability with a single pass and predictable behavior. The assignment specifies a tabular list; it does not require dynamic width calculation.

**Resolution:** Replaced dynamic-width specification with fixed column widths: ID 4, Status 11, Priority 8, Labels 20, Title 50. Both columns truncate with `…` at their limits. `show` always displays full values.

---

**Finding 5 — `\r\n` normalization is speculative cross-platform scope (Dim 9)**

`DESIGN.md` Edge Cases / Description: "`\r\n` line endings are normalized to `\n` on storage."

The target platform is macOS (Darwin). macOS terminal input does not produce `\r\n`. This normalization handles a case that cannot arise on the target platform, adds a transformation to user-supplied data that is not visible to the user, and is not motivated by any identified failure mode. If Windows support is added later, the normalization can be introduced then with a known need.

**Resolution:** Removed. No Windows target in scope; the normalization step had no identified failure mode on the target platform (macOS).

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

Five real findings, all resolved via DESIGN.md edits: removed atomic-write constraint, collapsed exit code tier from 0/1/2 to 0/1, removed `next_id` counter, replaced dynamic column widths with fixed widths, removed `\r\n` normalization. No dismissed items.

**Note — Purity boundary is methodology overhead, not assignment scope:** The purity boundary section (naming `validate_title`, `issue_matches_filters`, `format_issue_row`, etc. as formally pure) is required by the spec-crystallization primer, not by the assignment. The assignment's learning goals are Rust syntax, CLI design, state machines, and serialization — not VSDD Phase 5 formal verification preparation. This is not a finding. The purity boundary is a correct application of the VSDD methodology the project is following. It is noted here as a recognized tension: the methodology adds architectural structure that exceeds the assignment's learning objectives. The human director should decide whether this structure is appropriate for this stage of the program.

**Coordination:** Findings 1, 2, and 3 are spec-level decisions that, if changed, require corresponding updates to DESIGN.md before Layer 1 opens. Cross-reference with [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) for the spec compliance angle.

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

**Finding 2 — Layer 5 (compound filtering) as a standalone layer (Dim 9)**

Is a separate layer for compound filtering architecturally justified, or is it a test-only concern folded into Layer 4?

**Classification:** Dismissed. Compound filter AND-logic is a distinct behavioral requirement: it verifies that three independently-implemented filters compose correctly, and that the no-match message is correct for each combination. Testing this in Layer 4 would require all three filters to exist in Layer 4, which would pull priority filtering (Layer 3) and status filtering (Layer 2) forward out of their natural sequence. Layer 5 is the correct place to verify inter-filter composition once all filters exist. The layer is small but necessary.

---

**Finding 3 — Layer 6 combines description, show, and delete — possibly too broad (Dim 9)**

Layer 6 delivers three distinct capabilities: `--description` on create, `tracker show`, and `tracker delete`. Is this one layer or three?

**Classification:** Dismissed. All three are tightly coupled: `--description` is only meaningful once `tracker show` exists to display it fully; `tracker delete` is naturally paired with `tracker show` (the assignment's Layer 6 groups them explicitly). The acceptance criteria for all three fit in a single verifiable layer — a user can create with a description, show the full details, and delete an issue. None of the three creates a standalone capability without the others.

---

### Open

*(none)*

---

### Summary

One real finding resolved (Layer 1 sort algorithm specification). Two dismissed. The decomposition is architecturally sound: each layer delivers an independent, verifiable capability; layer ordering is correct (each layer depends only on previous layers); no structural debt is introduced between layers.

**Coordination:** *(none)*

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

**Coordination:** Finding 3 cross-referenced with [SOFTWARE-ENGINEER-REVIEW.md](SOFTWARE-ENGINEER-REVIEW.md) Review 2 Finding 2.

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

**Coordination:** *(none)*

---

---

## Review 5 — 2026-04-30 00:00Z

**Scope:** Layer 1 gate closure pass — no code changes since Review 4.

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

No SA findings. The Layer 1 architecture is unchanged and all prior findings remain resolved. MVR reached for Layer 1.

**Coordination:** *(none)*

---

---

## Review 6 — 2026-05-01 00:00Z

**Scope:** Layer 2 implementation — `src/lib.rs`, `src/main.rs`. Architectural evaluation of the Layer 2 additions: `parse_status`, `parse_id`, `cmd_status`, updated `cmd_list`.

**Session note:** In-session with full Layer 2 IAR suite. Acknowledged quality tradeoff. Review-session primer applied.

---

### Resolved

**Finding 1 — `parse_status` duplicates `VALID_STATUSES`: two independent sources of truth for valid status values (Dim 3 — Type safety / Dim 1 — Consistency)**

SA Review 4 (Layer 1) explicitly deferred enum introduction with the note: "The correct time to introduce enums is when the parsing layer is implemented." Layer 2 is the parsing layer. The implementation did not introduce enums, and now two sources of truth coexist:

- `VALID_STATUSES: &[&str] = &["open", "in-progress", "done"]` — used in `issue_fields_are_valid()` for post-deserialization validation
- `parse_status()` — uses a hardcoded `match` arm: `"open" | "in-progress" | "done" => Ok(...)`

These are maintained independently. A developer adding a fourth status value would need to update both locations. If only one is updated: `parse_status` accepts values that `issue_fields_are_valid` rejects (a write succeeds, next read fails) or vice versa (deserialization accepts values the parser would reject). Both failure modes produce user-visible errors on the read following a write.

The SA Review 4 deferred item is now due. The correct resolution is one of:
1. Derive `parse_status` from `VALID_STATUSES` (iterate and match)
2. Introduce a `Status` enum with `Display` and `FromStr` impls, replacing both

For a Phase 1 project, option 1 (use the constant in `parse_status`) is the minimum correct fix. Option 2 is the full architectural fix deferred from Layer 1.

**Resolution:** Replaced the `match` in `parse_status` with iteration over `VALID_STATUSES`:

```rust
pub fn parse_status(raw: &str) -> Result<String, String> {
    let lower = raw.to_lowercase();
    if VALID_STATUSES.contains(&lower.as_str()) {
        Ok(lower)
    } else {
        Err(format!(
            "Invalid status '{}'. Expected: open, in-progress, or done.",
            raw
        ))
    }
}
```

This also eliminates the double `.to_lowercase()` call (SE Review Finding 2, below). `VALID_STATUSES` is now the single source of truth for valid status values across both parsing and validation. All 37 tests pass.

---

### Dismissed

**Finding 2 — `cmd_list` `is_open_view` logic: string comparison to select empty-state message (Dim 1)**

`is_open_view = effective_status == "open"` is a string comparison that gates which empty-state message to show. Adding a second "special" status (e.g., a future `archived` status with its own message) would require adding another string comparison. This is a pattern that doesn't scale.

**Classification:** Dismissed. Two empty-state messages are all that DESIGN.md specifies, and the implementation correctly handles both. Extending to a third case would require refactoring at that point — not before. The current pattern is the minimum correct implementation for the current spec.

---

**Finding 3 — `Commands::Status` uses `id: String` rather than a typed ID (Dim 3 — Type safety)**

The CLI parser accepts the ID as a `String` and delegates validation to `parse_id()`. This means invalid IDs are caught in the command handler, not at the clap parsing layer.

**Classification:** Dismissed. This is the correct design: the spec requires the error message format `Error: '<id>' is not a valid issue ID. Expected a positive integer.` which must include the raw input string. A typed clap argument (e.g., `u64`) would produce clap's own error format, not the spec-required format. `String` + `parse_id()` is the correct implementation choice. Cross-referenced in SE review.

---

**Finding 4 — `tracker()` helper in `tests/layer2.rs` is a second duplicate of `tests/layer1.rs` helper (Dim 4 — Cohesion)**

SA Review 3 Finding 2 (Layer 1 stub) dismissed extraction as premature with: "Extract when Layer 3 or Layer 4 introduces a third file." Two files now exist.

**Classification:** Dismissed — one below the threshold stated in the prior dismissal. Three test files (Layer 3) is the correct extraction point.

---

### Open

*(none)*

---

### Summary

One real finding resolved: `parse_status` now derives from `VALID_STATUSES` rather than maintaining an independent match — single source of truth for valid status values restored. Three dismissed findings. The Layer 2 architecture is sound: `parse_id` and `parse_status` are pure validation functions with no I/O coupling, `cmd_status` is a thin command handler, and the library/binary split remains clean.

**Coordination:** Finding 1 discharges the deferred enum item from SA Review 4. Finding 3 cross-referenced with [SOFTWARE-ENGINEER-REVIEW.md](SOFTWARE-ENGINEER-REVIEW.md) Review 7. The `parse_status` deduplication also resolves [SOFTWARE-ENGINEER-REVIEW.md](SOFTWARE-ENGINEER-REVIEW.md) Review 7 Finding 2 (double `to_lowercase` call).

---

---

## Review 7 — 2026-05-04 05:45Z

**Scope:** Layer 3 implementation — `src/lib.rs`, `src/main.rs`, `tests/layer3.rs`. Architectural evaluation of the Layer 3 additions: `parse_priority`, `priority_rank`, `sort_issues`, extended `cmd_create` and `cmd_list` signatures.

**Session note:** Cold-session adversarial review using `iterative-adversarial-refinement/prompts/review-session.md` primer. Reviewer did not participate in Layer 3 build. Round 1 of Layer 3 SA review. Code change for Finding 1 was applied in this session by the human director.

---

### Resolved

**Finding 1 — `parse_priority` and `priority_rank` had two sources of truth for the priority value set (Dim 1 — Consistency / Dim 3 — Single source of truth)**

The Layer 3 implementation introduced two independent constants for the priority domain:

- `VALID_PRIORITIES: &[&str] = &["low", "medium", "high"]` — used in `issue_fields_are_valid()` and `parse_priority()` for membership testing
- `PRIORITY_ORDER: &[&str] = &["high", "medium", "low"]` — used in `priority_rank()` for sort ordering

Two slices of the same underlying domain (the three valid priority values) maintained independently. A developer adding a fourth priority value (e.g., `urgent`) would need to update both — and to keep the relative order coherent across both. Drift between them would corrupt either validation (accepting/rejecting wrong values) or sort order (an unknown priority routed to `usize::MAX`, sorting to the bottom silently). The same two-sources-of-truth pattern that SA Review 6 closed for status now exists for priority.

The minimum correct fix mirrors SA Review 6: collapse to a single ordered slice, use `.contains()` for membership and `.iter().position()` for rank. Membership and order are intrinsically tied for an enum-like priority domain — one slice expresses both.

**Resolution:** Removed `VALID_PRIORITIES`. `PRIORITY_ORDER = &["high", "medium", "low"]` is now the single source of truth: `issue_fields_are_valid` and `parse_priority` use `PRIORITY_ORDER.contains(...)` for membership; `priority_rank` uses `PRIORITY_ORDER.iter().position(...)` for sort rank. Doc comment added to the constant declaring both responsibilities. All 52 tests pass. `cargo clippy -- -D warnings` clean.

This discharges the priority-side equivalent of SA Review 6 Finding 1.

---

### Open

**Finding 2 — `tracker()` test helper now duplicated across three test files (Dim 4 — Cohesion)**

`tests/layer1.rs:6-10`, `tests/layer2.rs:6-10`, and `tests/layer3.rs:6-10` each define an identical four-line helper:

```rust
fn tracker(dir: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("tracker").unwrap();
    cmd.current_dir(dir.path());
    cmd
}
```

SA Review 3 Finding 2 (Layer 1) dismissed extraction as premature: "Extract when Layer 3 or Layer 4 introduces a third file." SA Review 6 Finding 4 (Layer 2) confirmed: "Three test files (Layer 3) is the correct extraction point." Layer 3 has now introduced `tests/layer3.rs`, the third file — the deferred extraction is now due per the prior dismissal's explicit threshold.

The standard Rust pattern for shared integration-test helpers is `tests/common/mod.rs` (each `.rs` directly under `tests/` is a separate test crate, but `tests/common/mod.rs` is treated as a module rather than a test target). Each test file then declares `mod common;` and uses `common::tracker(&dir)`.

**Classification:** Open — raised to the human director. Recommended resolution: extract to `tests/common/mod.rs` with `pub fn tracker(dir: &TempDir) -> Command`, add `mod common;` declaration to each of the three test files, replace local `tracker(...)` call sites with `common::tracker(...)`. Verify with `cargo test`. Not applied in this session — the prior dismissal threshold was met but the human director should approve the structural change before it lands.

**Update (2026-05-04 06:10Z): Resolved.** Director approved during gate-closure work. Applied the recommended resolution exactly: created `tests/common/mod.rs` with `pub fn tracker(dir: &TempDir) -> Command`; replaced the local `fn tracker(...)` definition in `tests/layer1.rs`, `tests/layer2.rs`, and `tests/layer3.rs` with `mod common;` + `use common::tracker;`. The unused `assert_cmd::Command` import was removed from each test file (the helper now owns it). All 53 tests pass; `cargo clippy --all-targets -- -D warnings` clean; `cargo fmt --check` clean.

---

### Dismissed

**Finding 3 — `priority_rank` returns `usize::MAX` for unknown values; silent failure mode (Dim 6 — Error handling visibility)**

`priority_rank` falls back to `usize::MAX` for any priority string not in `PRIORITY_ORDER`. An invalid stored priority would route to the bottom of sort order silently rather than producing an error.

**Classification:** Dismissed. Defensive backstop already evaluated and accepted in SE Review 3 (Layer 1). `issue_fields_are_valid` rejects any stored priority outside `PRIORITY_ORDER` at load time, so the `usize::MAX` branch is unreachable for legitimately persisted data. The only path to reach it is direct in-memory construction of an `Issue` with a malformed priority — internal-only, non-user-facing. A `panic!` or explicit error here would convert a defensive backstop into a crash on an internal bug; the silent-bottom-sort is the safer behavior.

---

**Finding 4 — `cmd_create` and `cmd_list` signatures growing with each layer (Dim 2 — Coupling)**

`cmd_create` now takes `(title_raw, priority_raw, issues_path)`. `cmd_list` takes `(status_filter, priority_filter, issues_path)`. Layer 4 will add `--label`, Layer 6 will add `--description`, etc. The signatures are accumulating positional parameters.

**Classification:** Dismissed. Phase 1 simplicity preferred over premature parameter-object abstraction. Argument count remains under 5 even after all layers are complete. A `CreateArgs` / `ListFilters` struct would be appropriate if the count exceeded 5, but the current shape is the minimum correct implementation. Revisit if Layer 6 pushes either signature past 5 parameters.

---

### Hallucinated

*(none)*

### Backlogged

*(SA does not own — defer to SO)*

---

### Summary

Two real findings resolved: priority constants unified — `PRIORITY_ORDER` is the single source of truth for both validity and sort rank, mirroring the `VALID_STATUSES` / `parse_status` unification from SA Review 6 (Finding 1); the `tracker()` test helper extracted to `tests/common/mod.rs` per the explicit extraction threshold from SA Review 3 / SA Review 6, with each test file now importing `common::tracker` (Finding 2, resolved at gate closure). Two findings dismissed (priority_rank defensive backstop — already accepted; signature growth — premature abstraction). The Layer 3 architecture is sound: `parse_priority` mirrors `parse_status`, `sort_issues` is a pure function with no I/O, the library/binary split is preserved, and the addition of priority filtering composes cleanly with the existing status filtering via sequential `retain` calls. The shared test helper closes the integration-test cohesion gap that has been carried forward since Layer 1.

**Coordination:** Finding 1 closes the priority-side equivalent of SA Review 6 Finding 1. Finding 2 closes the deferred test-helper extraction tracked since SA Review 3. No cross-domain coordination required.

---

---

## Review 8 — 2026-05-04 14:00Z

**Scope:** Cold-session adversarial SA review of the Layer 3 implementation post-gate-closure (gate work landed 2026-05-04 06:10Z). Reviewer did not participate in any Layer 1/2/3 build, prior IAR review, or spec authorship. Code under review: `src/lib.rs` (377 lines), `src/main.rs` (71 lines), `tests/common/mod.rs`, `tests/layer{1,2,3}.rs`, `Cargo.toml`. Primary lens: SA dim 1 (separation of concerns), dim 7 (extensibility for Layer 4 and Layer 7), and dim 12 (purity boundary inside `cmd_list`). Sycophancy guard: every prior dismissal was treated as a hypothesis to falsify, not a settled answer to defer to.

**Session note:** Cold session per `prompts/review-session.md` primer. Adversarial framing intact. Two real findings, two dismissed (with explicit re-raise conditions named), one hallucinated (with the control demonstrated).

---

### Open

**Finding 1 — `cmd_list` entangles pure filter+sort logic with effectful rendering; Layer 7 color introduction will collide with this same function (Dim 1 — Separation of concerns / Dim 12 — Purity boundary)**

`src/lib.rs:219-271`. The `cmd_list` function performs four distinct operations in a single ~50-line body:

1. **Effect:** load issues from disk (line 234)
2. **Pure:** filter by status, optionally by priority (lines 235-238, inline `retain` closures)
3. **Pure:** sort by priority then ID (line 249, calls extracted `sort_issues`)
4. **Effect:** render header + rows via inline `println!` calls with hardcoded format strings (lines 251-268)

Three concrete concerns:

- The pure logic (filter) is not extracted as a named function and is therefore not directly unit-testable in isolation. `sort_issues` is extracted (line 195) and has unit tests (`priority_sort_order_is_correct`, `priority_sort_tie_breaking_by_id`); the filter step is an inline `retain` closure with no analogous unit-level coverage. Filter behavior is testable today only via subprocess integration tests. The CLI supplement (`supplements/cli.md` § Software Engineering) is explicit on the principle: "Does the program compute a result as a typed value and then format it, rather than building output strings inline during computation?" The current answer is "no, it builds output strings inline." DESIGN.md "Purity guidance" (lines 372-374) names this as a recommended boundary; the implementation honors it for validation and parsing, partially honors it for sorting, and does not honor it for filtering or rendering.

- Column widths are encoded as inline format-string literals in two places: line 252 (header: `"{:<4}  {:<11}  {:<8}  {:<20}  Title"`) and line 265 (row: `"{:<4}  {:<11}  {:<8}  {:<20}  {}"`). The truncation widths appear separately as bare numeric literals at lines 262 (`truncate_with_ellipsis(&labels_raw, 20)`) and 263 (`truncate_with_ellipsis(&issue.title, 50)`). DESIGN.md commits to fixed column widths (4/11/8/20/50, lines 218); a change to any width requires synchronized edits across at least three locations. The DRY violation is implicit and silent — there is no compile-time check that the truncation cap matches the format-string padding.

- **Layer 7 (color output) compounding cost:** DESIGN.md (lines 227-238) specifies that priority and status values are colored per-cell when stdout is a TTY, with color suppression when piped. ANSI escape codes injected into a value before it reaches `{:<11}` will break column padding because Rust's `{:<width}` counts the escape bytes as visible chars. The Layer 7 implementation must therefore either (a) pad-then-color (compute the padded cell first, then wrap the value substring with escape codes), or (b) replace the inline format strings with explicit per-cell rendering that takes a `Style` parameter. Both paths require touching this same function, and both interact with the column-width literals scattered above. Doing the rewrite in Layer 7 alongside `IsTerminal` plumbing concentrates Layer 7's change volume into a function that is already carrying two unrelated responsibilities.

The minimum-correct restructure — appropriate for Layer 4, before Layer 7 lands:

```rust
// pure
fn filter_issues(issues: Vec<Issue>, status: &str, priority: Option<&str>) -> Vec<Issue> { ... }

// pure
fn format_issue_row(issue: &Issue) -> String { ... }
fn format_header_row() -> String { ... }

// effectful, calls pure helpers
pub fn cmd_list(...) -> Result<(), String> { ... }
```

Plus column-width constants at the module top:

```rust
const ID_WIDTH: usize = 4;
const STATUS_WIDTH: usize = 11;
const PRIORITY_WIDTH: usize = 8;
const LABELS_WIDTH: usize = 20;
const TITLE_WIDTH: usize = 50;
```

Layer 7 then injects color through `format_issue_row(issue, &Style::for_terminal())` without touching filter/sort and without re-deriving column widths.

**Classification:** Open — raised to human director. Recommended timing: extract during Layer 4 (which already touches `cmd_list` to add the `--label` filter), so Layer 7 can introduce color injection without simultaneously refactoring rendering. Performing the extraction during Layer 7 conflates two unrelated changes; doing it in Layer 4 keeps each layer's diff small and the architectural change reviewable in isolation.

**Coordination:** Cross-reference with [SOFTWARE-ENGINEER-REVIEW.md](SOFTWARE-ENGINEER-REVIEW.md) (CLI supplement § Software Engineering — output formatting separation, user-visible strings centralized, structured result types before formatting). When Layer 7 work is scoped, this finding is its architectural prerequisite.

---

**Finding 2 — `is_default_open_view` heuristic uses positive enumeration of filter states; structurally reproduces the SO Review 11 regression pattern at every new filter (Dim 7 — Extensibility)**

`src/lib.rs:232`:

```rust
let is_default_open_view = effective_status == "open" && effective_priority.is_none();
```

This variable selects between the two empty-state messages DESIGN.md (lines 307-309) specifies: `"No open issues. Nice work!"` (default-view empty) and `"No issues match the given filters."` (filter-view empty). The naming concern was addressed by SE Review 8 Finding 2 (`is_open_view` → `is_default_open_view`); the *structural* concern is independent and unresolved.

The concern is the shape of the expression. The "default view" is detected by enumerating every filter dimension and asserting it is at its default value. SO Review 11 already caught one regression caused by exactly this pattern: Layer 3 introduced `--priority` without updating the heuristic, so `tracker list --priority high` with no matches printed the default-view message in violation of DESIGN.md. The fix added `&& effective_priority.is_none()` to the conjunction. CHANGELOG.md credits this fix.

Layer 4 (`--label` filter) repeats the hazard exactly: the same conjunction must gain `&& effective_label.is_none()`. Layer 5 (compound filtering) does not add a new dimension but tests the existing ones together. Any future filter (a hypothetical `--created-since`, etc.) requires another conjunct each. Each addition is a fresh opportunity for the same omission. The Layer 3 regression was caught by SO in cold session, not by the Red Gate plan — and the Layer 4 Red Gate plan in `TODO.md` will need to explicitly include a `list_label_filter_no_match_shows_filter_message` test or risk repeating the omission.

The architectural fix inverts the polarity: track whether *any* filter beyond the default is active, then negate.

```rust
let any_extra_filter_active = status_filter.is_some() || priority_filter.is_some();
let is_default_open_view = !any_extra_filter_active;
```

Adding a new filter then requires changing one location (the disjunction), and the empty-state branch self-extends without requiring the developer to remember to update the conjunction. This is a one-line change today and removes the regression hazard from Layer 4 onward.

A subtlety worth noting: the disjunction operates on the raw `Option<&str>` flag inputs, not the post-validation `effective_*` values. This is intentional — `is_default_open_view` is a property of the *user's input shape*, not of the validated values. Using the raw flags also avoids the latent issue that `effective_status == "open"` is true both for the no-flag default and for an explicit `--status open` invocation (the spec treats these equivalently per `list_explicit_open_filter_matches_default` in `tests/layer2.rs:228`, and a future divergence would otherwise quietly require revisiting this code). The proposed form is robust against either interpretation.

**Classification:** Open — raised to human director. Recommended timing: land alongside Layer 4's `--label` addition, where the heuristic must be touched anyway. Inverting the polarity at Layer 4 prevents a third instance of the SO Review 11 regression pattern and is strictly cheaper than continuing to extend the conjunction.

**Coordination:** Closes the structural side of the SE Review 8 Finding 2 thread. The rename addressed naming clarity; this addresses composition fragility. No new domain referrals needed. Implies one Red Gate addition for Layer 4: a `list_label_filter_no_match_shows_filter_message` test analogous to `list_priority_filter_no_match_shows_filter_message` (QE Review 9) — but this is a QE concern, not an SA mandate.

---

### Dismissed

**Finding 3 — `lib.rs` is 377 lines and mixes data model, validation, parsing, sorting, formatting, storage I/O, command handlers, and unit tests in a single module (Dim 2 — Cohesion)**

A reviewer fresh to the codebase observes that `lib.rs` has accumulated nine distinct concerns in three layers. Layer 6 will add `cmd_show`, `cmd_delete`, description handling, and the `(none)`/`(empty)` rendering for `show` — pushing the file past 500 lines on current trajectory. The line count is approaching the upper end of what a single Rust module reads cleanly.

**Classification:** Dismissed. SA Review 4 Finding 2 explicitly deferred module decomposition to Layer 6: "When five commands exist (Layer 6 implementation complete), splitting into `lib/storage.rs`, `lib/validate.rs`, and `lib/commands.rs` would be appropriate." Three commands at Layer 3 is below the threshold; the file remains navigable; section boundaries are clear (data model → validation → constants → loaders → commands → tests). The deferred decomposition is on schedule. Re-raising it now would invert SA Review 4's explicit timing decision without new evidence — the failure mode the deferral was guarding against (refactoring before the shape is known) has not been overcome.

**Re-raise condition:** If Layer 4 pushes `lib.rs` past ~500 lines, or if Layer 4's label-handling code sits awkwardly alongside the existing flat structure (e.g., a new `validate_label` and a new `dedupe_labels` would both fit cleanly into a `validate.rs` module that does not yet exist), revisit then rather than at Layer 6. Otherwise hold to Layer 6 timing.

---

**Finding 4 — `parse_status` and `parse_priority` return `String` rather than typed enum values, leaving validated and unvalidated `String` indistinguishable to the type system (Dim 3 — Type precision)**

A reviewer fresh to the codebase observes that the validated post-parse value is still a `String` (lib.rs:128, 182). There is no `Status`/`Priority` enum to mark the value as validated. A caller could in principle pass any `String` into `Issue { status: ..., .. }` and the type system would not object. The runtime invariant exists (post-deserialization validation; parse-time validation), but the type system does not encode it.

**Classification:** Dismissed. This is the deferred enum item from SA Review 4 Finding 1. SA Review 6 Finding 1 chose option 1 (use the constant in `parse_status`) over option 2 (introduce enum) as the minimum-correct fix, and SA Review 7 Finding 1 applied the same decision to priority. Both decisions are documented in DECISIONS.md and were taken with the trade-off explicit (Phase 1 simplicity vs. type-system enforcement). The post-deserialization `issue_fields_are_valid` check provides the runtime invariant the type system does not enforce. Re-raising would re-litigate a settled architectural choice without new evidence; no defect has surfaced that an enum would have prevented.

**Re-raise condition:** If a defect is found that an enum would have caught at compile time — specifically, an internal construction site for `Issue` that places an unvalidated string into the `status` or `priority` field and survives review — revisit. Layer 6's `cmd_show` and Layer 7's color rendering are the next plausible places for such a defect to land; if either layer surfaces this kind of error, escalate to enum introduction at that point.

---

### Hallucinated

**Finding 5 — `Issue` fields are all `pub`; lack of encapsulation permits direct construction of `Issue` instances with invalid field values (Dim 4 — Interface contracts)**

Initial concern: `pub struct Issue { pub id, pub title, pub status, ... }` (lib.rs:13-23) exposes every field publicly. An external consumer of the `tracker` library (or a future internal caller) could construct `Issue { status: "garbage".to_string(), .. }` directly, bypassing `parse_status` and the post-deserialization validation. A constructor function (`Issue::new(...)`) returning `Result<Issue, String>` or non-`pub` fields with accessor methods would enforce construction-time invariants.

**Classification:** Hallucinated. Demonstration that the control holds:

1. **No external consumer exists.** The `tracker` library target (`Cargo.toml:10-12`) exists for the testing architecture (per SA Review 3 Finding 1) — to enable `lib.rs` unit tests against pure functions. The library is not published to crates.io, has no external callers, and is documented (DECISIONS.md, DESIGN.md) as a single-binary tool. There is no caller outside `main.rs` and the test suite that could construct an `Issue`.

2. **All internal construction sites are validated.** The construction sites are exhaustive: `cmd_create` (lib.rs:110-120, all fields validated upstream — title via `validate_title`, priority via `parse_priority`, status hardcoded `"open"`, ID via `next_id`, timestamps via `current_timestamp`); `serde::Deserialize` from disk (gated by `issue_fields_are_valid` post-parse, lib.rs:77-79); the test helper `issue` (lib.rs:323-334, test-only synthetic data). No construction path bypasses validation.

3. **Encapsulation would impose serde overhead with no observable benefit.** `serde::Deserialize` populates `pub` fields directly. A constructor-based design would require either `#[serde(from = "...")]` plumbing (a separate `RawIssue` deserialization struct and a fallible conversion) or runtime panics during deserialization. Both add complexity for a control that already exists at the validation boundary. The current shape is the minimum-correct implementation given the actual call graph.

The control holds: no realistic path exists for an internal caller to construct an invalid `Issue` and reach the storage or display layer. The "what if a future caller bypasses validation" concern is a hypothetical without an identified path of exploitation. Marking this finding hallucinated requires demonstrating the control specifically; the demonstration is the exhaustive enumeration above.

**If this changes:** if `lib.rs` is ever published as a separate crate or used by an external binary, re-raise immediately — the encapsulation argument shifts entirely at that point.

---

### Summary

Two real findings raised to director, both architectural-extensibility concerns whose cost compounds at Layer 4 and Layer 7:

1. **Finding 1** — `cmd_list` mixes pure filter+sort with effectful rendering; column widths and row formats are inline literals scattered across three locations. The architectural debt is paid in full at Layer 7 when ANSI-escape-coded values meet `{:<11}` padding. Recommended extraction during Layer 4 to keep Layer 7's diff focused on color/TTY logic.
2. **Finding 2** — `is_default_open_view` heuristic enumerates filter states positively, structurally reproducing the SO Review 11 regression pattern at every new filter dimension. Inverting to `!any_extra_filter_active` is a one-line fix that removes the hazard for Layer 4 and beyond.

Two findings dismissed with explicit re-raise conditions (lib.rs module decomposition deferred per SA Review 4 to Layer 6 unless Layer 4 forces the issue; enum types for status/priority deferred per SA Reviews 4/6/7 unless a defect surfaces that an enum would have caught). One finding hallucinated and demonstrated as such (Issue field-level `pub` exposure — no path exists by which an internal caller can reach storage or display with an invalid `Issue`).

**Regression check:** No regressions detected from prior architectural decisions. Library/binary split intact (`Cargo.toml:6-12`). Single-source-of-truth pattern intact for status (`VALID_STATUSES` consumed by both `parse_status` and `issue_fields_are_valid`, lib.rs:51, 60, 130) and priority (`PRIORITY_ORDER` consumed by `issue_fields_are_valid`, `parse_priority`, and `priority_rank`, lib.rs:55, 61, 173, 184). Post-deserialization validation in place (`load_issues` calls `issue_fields_are_valid` per loaded issue, lib.rs:77-79). Exit code 0/1 contract preserved (main.rs:49, 69). Fixed column widths preserved (4/11/8/20/50 per DESIGN.md, lib.rs:252, 262-265). `description` field's absent-not-null serialization preserved (`#[serde(skip_serializing_if = "Option::is_none")]` at lib.rs:16, ready for Layer 6 consumption). `#![deny(clippy::unwrap_used)]` enforced at crate level (lib.rs:1) with the single sanctioned `#[allow]` documented inline (lib.rs:85-86).

**Coordination:** Finding 1 is the architectural prerequisite for Layer 7 color introduction; cross-reference with [SOFTWARE-ENGINEER-REVIEW.md](SOFTWARE-ENGINEER-REVIEW.md) (CLI supplement § Software Engineering — output formatting separation, user-visible strings centralized, structured result types before formatting). Finding 2 closes the structural-fragility side of the SE Review 8 Finding 2 thread (which addressed naming via the `is_open_view` → `is_default_open_view` rename); the rename did not solve the underlying composition fragility and the two findings are complementary, not duplicative. No new domain referrals.

---

---

## Review 9 — 2026-05-05 00:00Z

**Scope:** Cold-session adversarial SA review of the Layer 4 implementation on branch `issue-tracker-cli-labels`. Layer 4 adds `--label` to `tracker create` (repeatable, deduplicated, case-preserved) and `--label` to `tracker list` (single value, case-sensitive AND-combined filter). Reviewer did not participate in any prior layer build, IAR review, or spec authorship. Code under review: `src/lib.rs` (703 lines), `src/main.rs` (96 lines), `tests/layer4.rs`, `tests/common/mod.rs`, `Cargo.toml`. Primary lens: dim 1 (separation of concerns), dim 2 (cohesion), dim 3 (data model integrity for labels), dim 7 (extensibility — Layer 6/7 prep), dim 9 (complexity budget for a single-maintainer Phase 1 portfolio project), dim 12 (purity boundary regression). Sycophancy guard: every prior Open finding from SA Review 8 was specifically checked for compliance against its own recommended Layer 4 timing, and every dismissed-with-re-raise-condition was checked against its named re-raise threshold.

**Session note:** Cold session per `prompts/review-session.md` primer. Adversarial framing intact. Two prior Open findings from SA Review 8 explicitly recommended landing during Layer 4 (Finding 1 — `cmd_list` extraction; Finding 2 — invert `is_default_open_view` polarity). Both deserve regression-style re-evaluation: was the Layer 4 work done in a way that honored or contradicted those recommendations?

---

### Open

**Finding 1 — SA Review 8 Finding 1 (cmd_list pure/effectful split + column-width constants) recommended for Layer 4 was NOT applied; Layer 4 instead extended the entangled `cmd_list` body with another inline `retain` for `--label`, compounding the very debt the prior finding warned about (Dim 1 — Separation of concerns / Dim 12 — Purity boundary / Regression of prior Open finding)**

`src/lib.rs:400-461`. SA Review 8 Finding 1 raised an Open architectural finding that `cmd_list` mixes pure filter+sort with effectful rendering, that column widths are scattered across three call sites as bare literals, and that the recommended fix was to extract `filter_issues`, `format_header_row`, `format_issue_row`, and module-level width constants (`ID_WIDTH`, `STATUS_WIDTH`, etc.) **during Layer 4** so that Layer 7 (color) does not conflate two changes. The recommendation was specific in both content and timing.

Layer 4 has now landed. The current `cmd_list` body adds the `--label` filter as a third inline `retain` (line 422-424):

```rust
issues.retain(|i| i.status == effective_status);
if let Some(p) = &effective_priority {
    issues.retain(|i| &i.priority == p);
}
if let Some(l) = label_filter {
    issues.retain(|i| label_matches(&i.labels, l));
}
```

The structure is unchanged from Layer 3: filter logic remains inline closures, the rendering remains inline `println!` calls (lines 441-457), and the column-width literals remain duplicated across the header format string (line 442: `"{:<4}  {:<11}  {:<8}  {:<20}  Title"`), the row format string (line 455: `"{:<4}  {:<11}  {:<8}  {:<20}  {}"`), the labels truncation (line 452: `truncate_with_ellipsis(&labels_raw, 20)`), and the title truncation (line 453: `truncate_with_ellipsis(&issue.title, 50)`). Four unsynchronized literal occurrences of the column widths now exist, up from three at Layer 3.

Layer 4 added a new pure helper (`label_matches`, lines 367-369) and a new pure helper (`dedupe_labels`, lines 351-360), both with unit tests — which is good. But the `cmd_list` body itself is now carrying **three** filter retain closures plus the rendering plus the load-from-disk effect, in a single function whose pure logic is still not testable in isolation at the unit level. Filter behavior for `--label` is exercised only by `tests/layer4.rs` integration tests via subprocess.

The Layer 7 collision argument from SA Review 8 stands unchanged and now has one additional layer of debt: ANSI-escape-coded priority/status values must still meet `{:<11}` and `{:<8}` padding, and the rewrite Layer 7 will need to do now spans more inline literals than it did at Layer 3.

**Classification:** Open — raised to SE. The recommended structure from SA Review 8 Finding 1 is unchanged. The case for landing it before Layer 7 is strengthened, not weakened, by Layer 4. Proposed text for the SE referral:

> Extract `filter_issues(issues, status, priority, label) -> Vec<Issue>` as a pure function with unit tests; extract `format_header_row()` and `format_issue_row(&Issue)` as pure formatters; introduce module-level `const ID_WIDTH: usize = 4; const STATUS_WIDTH: usize = 11; const PRIORITY_WIDTH: usize = 8; const LABELS_WIDTH: usize = 20; const TITLE_WIDTH: usize = 50;` and use them in both the format strings (via `format!` with `:<{width}`) and `truncate_with_ellipsis` calls. `cmd_list` becomes a thin effectful wrapper: load → filter → empty-state branch → sort → format → println.

**Coordination:** Cross-reference with [SOFTWARE-ENGINEER-REVIEW.md](SOFTWARE-ENGINEER-REVIEW.md) (CLI supplement § Software Engineering — output formatting separation, user-visible strings centralized, structured result types before formatting). Re-raise the prior coordination thread; this is the same finding with one more layer of evidence behind it.

---

**Finding 2 — SA Review 8 Finding 2 (invert `is_default_open_view` polarity) recommended for Layer 4 was applied PARTIALLY: the new conjunct was added correctly, but the polarity was NOT inverted, leaving the regression hazard intact for Layer 5+ (Dim 7 — Extensibility / Regression of prior Open finding)**

`src/lib.rs:414-415`:

```rust
let is_default_open_view =
    effective_status == "open" && effective_priority.is_none() && label_filter.is_none();
```

SA Review 8 Finding 2 explicitly raised the *structural* fragility of the positive-enumeration form — that every new filter dimension forces the developer to remember to extend the conjunction, and that the SO Review 11 regression came from exactly this pattern. The recommended fix was:

```rust
let any_extra_filter_active = status_filter.is_some() || priority_filter.is_some() || label_filter.is_some();
let is_default_open_view = !any_extra_filter_active;
```

Layer 4 added `&& label_filter.is_none()` to the existing conjunction (the Layer 4 author *did* remember to update it — credit where due), but did not invert the polarity as recommended. The regression hazard the prior finding warned about is therefore unchanged: a future filter (say, a hypothetical `--created-since` in a Layer 8 polish round, or a `--description-contains`) will once again require the developer to remember to extend the conjunction, with no compile-time check that they did. Each new filter is another opportunity for a third instance of the SO Review 11 pattern.

The SA Review 8 finding was specific that the rename (`is_open_view` → `is_default_open_view`) was not a substitute for the structural fix. Layer 4 honored the rename and even extended it correctly for `--label`, but the *structural* fragility remains. The defense in depth that comes from inverting the polarity (a developer adding a new filter only has to add it to the disjunction; the empty-state branch then "self-corrects") was the entire point of the prior finding.

A subtlety: the existing conjunction reads `effective_status == "open"` rather than `status_filter.is_none()`. The two are not equivalent — the former is true both for the no-flag default and for an explicit `--status open`, while the latter is true only for the no-flag default. The current code chooses the former intentionally (an explicit `--status open` with no other filters takes the default-view empty-state message, matching `tests/layer2.rs::list_explicit_open_filter_matches_default`). The proposed inversion must preserve this semantics — so the disjunction should be `status_filter.is_some() || priority_filter.is_some() || label_filter.is_some()` operating on raw flags, *plus* a check that if `status_filter.is_some()` we treat it as not-default unless the resolved status is also `open` and the other filters are absent. Re-reading the SA Review 8 proposed form, it operated on raw flag presence — which has a behavioral difference from the current code at `tracker list --status open`. The QE Review 9 test `list_explicit_open_filter_matches_default` would catch this regression.

The minimum-correct fix that preserves current semantics:

```rust
let any_filter_flag_set = priority_filter.is_some() || label_filter.is_some()
    || status_filter.is_some_and(|s| s != "open");
let is_default_open_view = !any_filter_flag_set;
```

Or, since `effective_status == "open"` already captures the "status is at default OR equivalent to default" check, retain that and only invert the *new-filter* portion:

```rust
let no_extra_filter = effective_priority.is_none() && label_filter.is_none();
let is_default_open_view = effective_status == "open" && no_extra_filter;
```

The second form is essentially what's in the code today, just named differently — so the actual structural improvement is to extract the disjunction into a named variable that future filters extend, even if the polarity is preserved. Either of these is acceptable; the current code is not.

**Classification:** Open — raised to SE. Proposed text for the SE referral:

> Refactor the `is_default_open_view` derivation in `cmd_list` to extract the "any non-default filter active" disjunction into a named helper or local variable, so adding a future filter requires touching one location (the disjunction) rather than adding a conjunct to `is_default_open_view`. Preserve current semantics: `tracker list --status open` continues to use the default-view empty-state message. Existing test `list_explicit_open_filter_matches_default` is the regression check.

**Coordination:** Re-raise of SA Review 8 Finding 2. Cross-reference with QE for the regression-check test that already exists (`list_explicit_open_filter_matches_default`).

---

### Dismissed

**Finding 3 — `lib.rs` is now 703 lines, exceeding the 500-line re-raise threshold from SA Review 8 Finding 3 (Dim 2 — Cohesion / Re-raise condition triggered)**

SA Review 8 Finding 3 dismissed module decomposition as deferred to Layer 6, with an explicit re-raise condition: "If Layer 4 pushes `lib.rs` past ~500 lines, or if Layer 4's label-handling code sits awkwardly alongside the existing flat structure (e.g., a new `validate_label` and a new `dedupe_labels` would both fit cleanly into a `validate.rs` module that does not yet exist), revisit then rather than at Layer 6."

`lib.rs` is 703 lines. The re-raise threshold has been exceeded. The Layer 4 additions (`parse_label`, `dedupe_labels`, `label_matches`, plus extended tests) do fit cleanly into a hypothetical `validate.rs` / `labels.rs`. By the prior finding's own re-raise rule, this should be re-evaluated.

**Classification:** Dismissed (with re-raise acknowledgment). Re-evaluating: 703 lines includes ~240 lines of `#[cfg(test)]` unit tests that compile out of the binary. The non-test code is ~460 lines, still under the 500 threshold the prior re-raise rule was concerned about (the prior finding measured "the file" without distinguishing test from non-test, but the cohesion concern is properly about non-test code — tests are inherently a separate concern that's already isolated by the cfg gate). With Layer 6 adding `cmd_show` and `cmd_delete` plus description rendering, the non-test code will likely cross 500 lines at that point. Holding to Layer 6 timing is consistent with the *intent* of the SA Review 8 re-raise rule, even though the literal line count has crossed it.

**Re-raise condition (revised):** If Layer 5 or Layer 6 adds non-test code (i.e., excluding the `#[cfg(test)]` block at `lib.rs:463-703`) past 500 lines, the decomposition into `lib/storage.rs`, `lib/validate.rs`, `lib/commands.rs` is due. The prior dismissal's intent stands; the trigger is on production code, not test code. SE / SO note this in their reviews.

---

**Finding 4 — `cmd_create` has grown to 4 parameters; SA Review 7 Finding 4's "5 parameter" re-raise threshold is approaching for Layer 6 (Dim 2 — Coupling / Re-raise watch)**

SA Review 7 Finding 4 dismissed signature growth as "Phase 1 simplicity preferred over premature parameter-object abstraction" with re-raise at >5 parameters. Current `cmd_create` (lib.rs:202-207) takes `(title_raw, priority_raw, labels_raw, issues_path)` = 4 parameters. Layer 6 adds `--description` → 5. Layer 7 adds nothing to create. The threshold is reached at Layer 6 exactly.

**Classification:** Dismissed (Layer 4 is below threshold). No structural change required at Layer 4. The dismissal from SA Review 7 holds.

**Re-raise condition:** Layer 6's `--description` addition pushes `cmd_create` to 5 parameters. At that point, introduce `CreateArgs { title, priority, labels, description }` and pass `(args, path)`. SE should preempt this when scoping Layer 6 to avoid a separate refactor pass.

---

### Hallucinated

**Finding 5 — `dedupe_labels` allocates a new `Vec` and `HashSet` on every `cmd_create`; for a CLI that processes a typical `--label`-list of length ≤5, this is over-engineering relative to a simple `Vec` with a linear `contains` check (Dim 9 — Complexity budget for single-maintainer Phase 1)**

Initial concern: `src/lib.rs:351-360` uses a `HashSet<&str>` to dedupe a slice of labels. For typical CLI input (1-5 labels), a `Vec::contains` would be simpler, has the same asymptotic order at this scale, and avoids the hash allocation. The `HashSet` is a team-scale-engineering default — a human engineer working alone on a Phase 1 portfolio project would likely write the `Vec::contains` form.

**Classification:** Hallucinated. Demonstration that the control holds:

1. **The `HashSet` form is also what a careful human engineer would write** — `Vec::contains` on `&String` requires either `.iter().any(|x| x == label)` (clear but verbose) or `Vec::contains(&label.clone())` (allocates per-comparison). The `HashSet` form is actually shorter and clearer at the call site than the equivalent linear-scan form once you account for the `&String` vs `&str` borrow gymnastics. Allocation count: one `HashSet` and one `Vec::with_capacity` per call, both pre-sized — this is a single allocation pair, not "over-engineering."

2. **The pattern is consistent with `issues_collection_invariants_hold`** (lib.rs:146-149), which uses `HashSet::with_capacity` for ID uniqueness checking — the same shape, established earlier in the codebase, reviewed and accepted by prior IAR rounds. Using a different shape for label dedup would be inconsistent; using the same shape is consistent.

3. **The "team-scale default" sycophancy guard does not apply here.** The complaint pattern says: AI agents reach for `HashSet`/`HashMap` where a `Vec` would do. But the use here is correct — dedup with O(n) expected time and clear intent. The alternative (`Vec::contains` with linear scan) is O(n²) and only better when n is small *and* the comparison is cheap. For string labels, hash comparison is faster per-iteration than full string equality once n exceeds ~3. The `HashSet` is the right primitive.

The control holds: this is not over-engineering. Marking the finding hallucinated requires demonstrating the alternative would be worse, and the demonstration is above. The `HashSet` form is the minimum-correct implementation, not a team-scale default.

---

### Summary

Two real findings, both regression checks on prior Open findings that were explicitly scoped to Layer 4:

1. **Finding 1 (re-raise of SA Review 8 Finding 1)** — `cmd_list` extraction recommended for Layer 4 was not applied; Layer 4 instead added a third inline `retain` and a fourth column-width literal occurrence, compounding the debt the prior finding warned about. Layer 7 (color) prep is now further behind, not at-pace.
2. **Finding 2 (partial re-raise of SA Review 8 Finding 2)** — the new `&& label_filter.is_none()` conjunct was added correctly (no functional regression), but the polarity inversion was not done. The regression hazard for Layer 5+ filter additions is unchanged.

Two findings dismissed with explicit re-raise conditions (lib.rs decomposition: re-raise rule revised to "non-test code past 500 lines" — currently ~460, due at Layer 5/6; cmd_create signature growth: 4 parameters now, threshold at 5, due at Layer 6's `--description`). One finding hallucinated and demonstrated as such (`HashSet` for label dedup is correct, not over-engineered).

**Regression check on prior architectural decisions:** Library/binary split intact (`Cargo.toml:11-18`). Single-source-of-truth pattern intact for status (`VALID_STATUSES`, lib.rs:102) and priority (`PRIORITY_ORDER`, lib.rs:106). Post-deserialization validation extended correctly to labels (`issue_fields_are_valid` checks `labels.iter().all(|l| !l.trim().is_empty())` at lib.rs:131 — invariant for stored data is enforced). Description-field absent-not-null serialization preserved (lib.rs:39). `description` field is reserved at `None` in `cmd_create` (lib.rs:225) — Layer 6 will populate it. Exit code 0/1 contract preserved (main.rs:65, 68, 94). Empty-state stderr routing preserved (lib.rs:432, 434). The `clippy::unwrap_used` deny set is preserved with the single sanctioned `#[allow]` documented inline (lib.rs:184-185). The label data model is sound: `Vec<String>` with case-preserved, dedup-at-creation, case-sensitive match semantics align with DESIGN.md Feature 1, Feature 2, and Edge Cases / Labels. The asymmetry in `--label` semantics (repeatable on `create`, single-value on `list`) is correctly enforced by clap's argument types: `Vec<String>` on create accepts repetition, `Option<String>` on list rejects it via clap's default behavior, with the resulting error transformed to the spec's `Error:` prefix in `main.rs:62-65`.

**Coordination:**
- **Raised to SE (Finding 1):** Apply the `cmd_list` pure/effectful split with module-level column-width constants. Recommended ahead of Layer 7. Same proposal text as SA Review 8 Finding 1 with one additional layer of evidence (now four width-literal occurrences, three filter retains). Cross-reference with [SOFTWARE-ENGINEER-REVIEW.md](SOFTWARE-ENGINEER-REVIEW.md) CLI supplement.
- **Raised to SE (Finding 2):** Refactor the `is_default_open_view` derivation to extract the new-filter disjunction into a named local variable that future filters extend at one site, preserving current semantics. Existing test `list_explicit_open_filter_matches_default` (`tests/layer2.rs`) is the regression check.
- **For QE (informational, not a mandate):** Layer 4 Red Gate adequately covers `--label` filtering (`tests/layer4.rs:180-235`). No specific test gap to flag from SA's lens; QE will have its own dimensions for label filter coverage.
- **For SO (informational):** No DESIGN.md changes proposed. The label data model and asymmetric `--label` semantics on create vs. list are coherent with the spec as written.

**Architectural concerns next-tier reviewers should know about:** SE will receive both Open findings; the `cmd_list` extraction is the higher-value fix (it's the architectural prerequisite for Layer 7 color and reduces inline literal sprawl now). QE may want to verify the `list --label X --label Y` rejection error text matches the spec's "usage error" intent — clap's default rejection produces clap's standard message format ("the argument '--label <label>' cannot be used multiple times"), which is then transformed by `main.rs` to begin with `Error:`. DESIGN.md does not specify the exact wording for the multiple-label-on-list rejection, so this is acceptable, but a QE round may flag the message quality. Software Engineer should also note that `truncate_with_ellipsis` is now used for both label and title columns — the function is fine as-is, but the magic number `20` for labels duplicates the format-string `:<20}` width and should land alongside the Finding 1 column-width constant extraction.

---

## Review 10 — 2026-05-06 03:00Z

**Round:** SA Review 10 (Round-2 architectural carry-forward for Layer 4)
**Scope:** Re-evaluate Round-1 F1 / F2 status after the Round-2 source changes in commit `67ef920`. Sycophancy guard: did Round-2 either resolve, worsen, or leave unchanged the architectural debt named in Round 1?
**Session context:** Warm-verification session.

### Status of Round-1 findings

#### Finding 1 (Round-1) — `cmd_list` extraction with column-width constants

Round-2 source changes added a `parse_label` call on the filter side (`cmd_list:454-461`) and a `display_safe` helper for error formatters. Neither change modified the cmd_list rendering pipeline (column widths, header row, retain calls). The function gained one more pre-load step (`effective_label` validation) but the rendering core is unchanged. The four scattered column-width literals are still scattered. The Layer 7 prep work is still in front of the project.

**Classification: Open (Deferred to a focused PR before Layer 7).** Same as Round 1; SO Review 17 records the deferral with the named target. The Round-2 commit did not regress the architectural quality (no new inline retains; no new format-string occurrences); it also did not advance it. Holding to the original SE/SA rationale: the extraction is a structural change that needs its own PR with its own test scaffolding, not bundled with the security/correctness fix.

#### Finding 2 (Round-1) — `is_default_open_view` polarity inversion

Resolved in SE Review 11 (commit `b4f2db1`). Round-2 source changes re-used the named `extra_filter_active` local introduced by SE Review 11 — adding the new `effective_label.is_some()` disjunct in the same single location, rather than appending a new conjunct to the empty-state predicate. This is the future-extension property the original finding was designed to ensure: future filters (Layer 6's `--description-contains` etc.) will continue to extend at one site rather than everywhere. The structural fragility is genuinely closed.

**Classification: Resolved (re-verified).** The SO Review 11 regression class is closed for Layer 4+ filter additions.

### Round-2 architectural observations (no new findings)

- The new `display_safe` helper (`src/lib.rs:149-166`) is a small, well-scoped, side-effect-free pure helper with its own unit tests — exactly the architectural shape the project's CLI supplement recommends. Its addition reduces architectural debt by centralizing the Cc-escape rule at the formatter sites.
- The new `label_is_valid` helper (`src/lib.rs:141-147`) is similarly well-scoped: it's the read-side dual of `parse_label`'s write-side validation, and it's called from `issue_fields_are_valid` to maintain the load-time hygiene invariant. This follows the same pattern as the title check on the line above. Symmetry with prior validation discipline is preserved.
- No new module-level state, no new public API beyond what the spec changes required, no new dependencies.

### Re-raise watches (unchanged from Round 1)

- `lib.rs` non-test line count: now ~485 (was ~460). Approaching the SA Review 8 re-raise threshold of 500 non-test lines. Layer 5/6 will likely cross it, at which point the `lib/storage.rs`, `lib/validate.rs`, `lib/commands.rs` decomposition is due. Tracking note for SE / Layer 5 scoping.
- `cmd_create` parameters: still 4. Threshold of 5 reached at Layer 6's `--description`. Tracking note unchanged.

### Summary

Round-1 F2 verified Resolved (and the resolution survived contact with new filter additions in Round-2 — the property held). F1 unchanged (Deferred to a focused PR before Layer 7). 0 new architectural findings from Round-2 source changes. The Round-2 source changes are architecturally clean.

**Files modified:** Only this log appended.

---

---

## Review 11 — 2026-05-07 00:23Z

**Round:** SA Review 11 (Layer 5 — Compound Filtering).
**Scope:** Cold-session adversarial SA review of the Layer 5 implementation (commits `7d1ca57` Phase 2a Red Gate, `bd15a9d` Phase 2b implementation, `da0fd8d` manual-testing checklist closure). Code under review: `src/lib.rs` (948 lines incl. tests), `src/main.rs`, `tests/layer5.rs`. Primary lens: dim 1 (separation of concerns), dim 2 (cohesion), dim 3 (coupling), dim 4 (interface contracts), dim 5 (complexity budget), dim 6 (decision documentation), and the SA Review 9 carry-forward findings (F1 cmd_list extraction, F2 extra_filter_active disjunction).
**Session note:** Cold session per the IAR primer. Adversarial framing intact. Parallel batch with SO 18 / QE 13 / SE 13 / VDD-IAR 13. Sycophancy guard: every prior Open / Resolved finding from SA 8/9/10 was specifically re-validated against the Layer 5 source — particularly the SA R10 "Resolved" verdict on F2 (extra_filter_active disjunction).

---

### Open

**Finding 1 — Layer 5's predicate extraction closed the *filter* half of SA Review 9 Finding 1; the *rendering* half (column-width constants + `format_header_row` / `format_issue_row` extraction) remains untouched. The Layer 7 collision argument is unchanged (Dim 1 — Separation of concerns / Dim 12 — Purity boundary / Carry-forward of SA R8 F1 → SA R9 F1)**

`src/lib.rs:425-434` — `issue_matches_filters` is now a named pure predicate with five unit tests (`filter_and_logic_all_present_returns_true`, `filter_and_logic_all_must_match`, `filter_status_only_matches_any_priority_and_labels`, `filter_status_mismatch_rejects_regardless_of_optional_filters`, `filter_label_match_is_case_sensitive`). `cmd_list:500-507` collapses three chained `retain` calls into a single `retain` over the predicate. This is a real architectural improvement and partially discharges SA R9 F1: the filter logic is now testable in isolation, and any future filter (e.g. a hypothetical Layer 8 `--created-since`) extends the predicate at one site rather than appending another `retain` to `cmd_list`.

The *rendering* half of SA R9 F1 is unchanged. The four scattered column-width literals are still scattered:

- `src/lib.rs:525` — header format string `"{:<4}  {:<11}  {:<8}  {:<20}  Title"`
- `src/lib.rs:535` — `truncate_with_ellipsis(&labels_raw, 20)`
- `src/lib.rs:536` — `truncate_with_ellipsis(&issue.title, 50)`
- `src/lib.rs:538` — row format string `"{:<4}  {:<11}  {:<8}  {:<20}  {}"`

No module-level `ID_WIDTH` / `STATUS_WIDTH` / `PRIORITY_WIDTH` / `LABELS_WIDTH` / `TITLE_WIDTH` constants exist. No `format_header_row()` / `format_issue_row(&Issue)` helpers exist. `cmd_list` still mixes load (effect) → filter (now pure via predicate) → empty-state branch (effect, two messages) → sort (pure) → header `println!` (effect) → row `println!` loop (effect) in a single function body of ~80 lines. The Layer 7 collision argument from SA Review 8 stands unchanged: ANSI escape bytes injected into priority/status values before they reach `{:<11}` / `{:<8}` will break column padding because Rust's `{:<width}` counts escape bytes as visible chars, and the rewrite Layer 7 will need to do still spans four width-literal occurrences plus two scattered format strings.

**Self-test (sycophancy guard):** could this be dismissed as "rendering extraction is genuinely a Layer 7 concern"? No — the original SA R8 F1 finding was specific that doing the extraction in Layer 7 *alongside* `IsTerminal` plumbing and color injection conflates two unrelated changes; doing it before Layer 7 is the architecturally cheaper path. Layer 5 touched `cmd_list` (the predicate refactor, the chained-retain collapse) and was the third natural opportunity to address the rendering half. The work was scoped to predicate extraction only. The Layer 7 prep is still in front of the project.

**Severity:** Low-to-medium. Half of SA R9 F1 is now genuinely closed; the architectural improvement is real (filter logic is unit-testable, predicate is the future-extension site). The remaining half is the same shape it has been since SA R8 — a known, tracked, deferred refactor.

**Classification: Open (Deferred to a focused PR before Layer 7).** Same disposition as SA R10's verdict on the original F1: deferred to a focused PR with its own test scaffolding, not bundled with another layer's behavioral work. SO Review 17 already records this deferral with the named target. This finding is the narrowed re-raise: the predicate extraction *did* close half the original concern, and that progress should be acknowledged.

**Proposed action:** Track as a pre-Layer-7 PR (existing deferral). Recommended scope at that time: introduce `ID_WIDTH`/`STATUS_WIDTH`/`PRIORITY_WIDTH`/`LABELS_WIDTH`/`TITLE_WIDTH` module-level `const` items; extract `format_header_row()` and `format_issue_row(&Issue)`; thread the format-width literals through `format!("{:<width$}", ..., width = STATUS_WIDTH)` so the `:<11}` / `:<20}` widths and the `truncate_with_ellipsis(_, 20)` / `truncate_with_ellipsis(_, 50)` caps share a single source of truth.

**Coordination:** Cross-reference with [SOFTWARE-ENGINEER-REVIEW.md](SOFTWARE-ENGINEER-REVIEW.md) (CLI supplement § Software Engineering — output formatting separation, structured result types before formatting). No new SE referral; the existing pre-Layer-7 deferral covers it.

---

### Resolved

**Finding 2 — Predicate-extraction half of SA Review 9 Finding 1 (Carry-forward / Dim 1)**

The predicate-extraction half of SA R9 F1 is now closed by Layer 5: `issue_matches_filters` is a named pure function (`src/lib.rs:425-434`) with five focused unit tests. `cmd_list` no longer carries inline filter closures; the AND-combination is testable at the unit level and visible at one site. Future filter additions extend the predicate's body or signature, not the call site.

**Classification: Resolved (predicate-extraction half).** Re-validated against the Layer 5 source. The remaining rendering-extraction half is tracked as Finding 1 above.

---

**Finding 3 — `extra_filter_active` disjunction property survives Layer 5 (Carry-forward / Dim 7 — Extensibility / SA R8 F2 → SA R9 F2 → SA R10)**

SA Review 10 verified F2 Resolved: `cmd_list` uses a named `extra_filter_active = effective_priority.is_some() || effective_label.is_some()` disjunction (`src/lib.rs:496`), and the empty-state predicate consumes it as `effective_status == "open" && !extra_filter_active` (`src/lib.rs:497`). Layer 5 did not add a new filter dimension (compound filtering is the AND of existing dimensions), so the disjunction was not re-tested by source change.

The Layer 6 risk to this property is bounded: per DESIGN.md (line 207-208), `--description` is on `tracker create` only, not `tracker list`. Layer 6 will not extend the `cmd_list` filter set. The property holds for the spec as written. If a future hypothetical filter (a `--description-contains` polish, etc.) is added to `list`, it will need to extend the disjunction at one site — which is exactly the property the original finding was designed to ensure.

`tests/layer5.rs::list_compound_three_filter_no_match_shows_filter_message` is a regression check on this property: the label filter alone is the odd-one-out, so a regression that dropped `--label` from `extra_filter_active` would route to "No open issues. Nice work!" instead of "No issues match the given filters." and the test would fail.

**Classification: Resolved (re-verified for Layer 5).** Property holds; regression check exists; spec-as-written has no Layer 6 filter additions to threaten it.

---

### Dismissed

**Finding 4 — Predicate signature asymmetry (`status: &str` required, `priority`/`label: Option<&str>` optional) is undocumented at the contract level beyond the doc comment; risk of misuse if a caller passes a non-normalized status (Dim 4 — Interface contracts)**

`src/lib.rs:425-434`. The predicate signature is asymmetric: `status` is required, `priority` and `label` are optional. The function uses `==` for status equality (`issue.status == status`) — case-sensitive byte comparison. A caller that passed `"OPEN"` rather than `"open"` would silently no-match every issue. The doc comment (`src/lib.rs:421-424`) explicitly notes "priority and status comparisons assume the caller has already normalized the filter values (lowercase) and that stored values are normalized at write/load time," which addresses the contract but does not enforce it.

**Classification: Dismissed.** Demonstration that the control holds:

1. **The function is private** (`fn`, not `pub fn`), so external misuse is impossible. The only call site is `cmd_list` (`src/lib.rs:500-507`).
2. **The single call site normalizes inputs**: `effective_status` is the result of `parse_status(s)?` (`src/lib.rs:471-474`) which lowercases unconditionally; `effective_priority` is the result of `parse_priority(p)?` (`src/lib.rs:475-478`) which also lowercases. `effective_label` flows through `parse_label` (`src/lib.rs:485-488`) which trims but does not case-normalize — but label comparison is *spec-required* to be case-sensitive (DESIGN.md "Labels filter matches case-sensitively"), so passing the raw trimmed value is correct.
3. **Stored values are normalized at write/load time**: `parse_status` lowercases on input to `cmd_status` and `cmd_create`; `parse_priority` lowercases on input to `cmd_create`; `issue_fields_are_valid` rejects non-canonical stored values at load (`src/lib.rs:129-130`).
4. **The doc comment is the contract for an internal helper.** Promoting the contract from doc-comment-on-private-fn to a type-level invariant (e.g., an enum-typed `Status` parameter) was already evaluated and Dismissed in SA R8 F4 / SA R6 F1 / SA R7 F1 with re-raise condition "if a defect is found that an enum would have caught at compile time." No such defect has surfaced.

The control holds: no realistic path exists for a misuse to land. Marking this finding Dismissed requires demonstrating the control specifically; the demonstration is above.

**Re-raise condition:** if `issue_matches_filters` is ever made `pub` (exported from the library), or if a future Layer's `cmd_*` handler grows a second call site that does not flow through `parse_status` / `parse_priority`, revisit immediately — the doc-comment contract is too weak for an exported API or a multi-call-site predicate.

---

**Finding 5 — DECISIONS.md has no Layer 5 entry; the predicate-extraction architectural choice is captured only in the commit messages of `7d1ca57` and `bd15a9d` (Dim 6 — Decision documentation)**

DECISIONS.md (107 lines, last updated for Layer 3 SO Review 13 spec amendments) has entries for atomic-write deferral, line-ending non-normalization, control-character-rejection-in-titles, empty-state-stderr-routing, and SE Review 9 ratification — but no Layer 4 or Layer 5 entries. The Layer 5 architectural decision (extract `issue_matches_filters` as a private predicate rather than a `filter_issues(...) -> Vec<Issue>` form, collapse three retains into one, defer the rendering split per SA R10) is documented in the commit messages of `7d1ca57` Phase 2a Red Gate and `bd15a9d` Phase 2b implementation. A future reviewer reading DECISIONS.md cold will not see why the predicate is private, why `is_none_or` was chosen over a match, or why the rendering split was deferred.

**Classification: Dismissed.** DECISIONS.md is a record of *durable* architectural choices and *spec amendments* — atomic-write deferrals, control-character rejection, stream-routing changes. The predicate extraction is an internal refactor of a private function with no spec-visible behavior change ("Behavior is unchanged from the prior chained-retain form" per the `bd15a9d` commit message). The commit-message rationale is the appropriate documentation venue for this class of change. DECISIONS.md should not become a refactor log; it would dilute the durable-decision signal.

**Re-raise condition:** if Layer 5 had introduced a *spec-visible* architectural choice (a new public API, a behavior change, a stream-routing change, etc.), DECISIONS.md would be the right venue. None did. If a future layer's refactor introduces a spec-visible architectural decision, DECISIONS.md is the right venue at that point.

---

### Hallucinated

**Finding 6 — `issue_matches_filters` couples to `Issue` struct shape (`issue.status`, `issue.priority`, `issue.labels` field reads); a future change to `Issue` would propagate to the predicate (Dim 3 — Coupling)**

Initial concern: the predicate reads three `Issue` fields directly. If `Issue.status` ever became `Issue.state`, the predicate would need updating. Decoupling via accessor methods (`issue.status()`, `issue.priority()`, `issue.labels()`) would isolate the predicate from struct-shape change.

**Classification: Hallucinated.** Demonstration that the control holds:

1. **`Issue` is the data model, not a service abstraction.** Coupling a filter predicate to the data model it filters is *the correct* coupling shape. Decoupling via accessors would add ceremony for no benefit — accessor methods that just return `&self.field` are noise that obscure the intent.
2. **The `Issue` struct shape is fixed by DESIGN.md.** DESIGN.md (lines 162-171) specifies the field names and types as part of the storage contract. A field rename would require both a DESIGN.md spec amendment AND a migration story for existing `tracker.json` files. The "what if a field gets renamed" hypothetical does not have a realistic path.
3. **Direct field reads are the Rust-idiomatic pattern for internal predicates.** A `pub fn matches(&self, ...)` on `Issue` would be the alternative, but moving the predicate to an `impl` block on `Issue` would couple `Issue` (the data model) to filter semantics (a `cmd_list` concern), which is a worse architectural shape than the current arrangement.

The control holds: the coupling is the correct coupling for an internal-predicate-over-data-model pattern. Marking this finding Hallucinated requires demonstrating the alternative would be worse, and the demonstration is above.

---

### Summary

One real finding — a narrowed carry-forward of SA R9 F1: the predicate-extraction half is now closed (filter logic is unit-testable; AND-combination is one named function), but the rendering-extraction half (column-width constants + header/row formatters) is unchanged. Layer 5 was the third natural opportunity to do that work; it remained scoped out. The Layer 7 collision argument is unchanged. Disposition: Open — Deferred to a focused PR before Layer 7 (same disposition as SA R10's verdict on the original F1).

Two findings Resolved (re-verified for Layer 5): predicate-extraction half of SA R9 F1, and the `extra_filter_active` disjunction property from SA R8/R9/R10 F2. The disjunction property was not re-tested by source change in Layer 5 (no new filter dimension was added), and the Layer 6 risk is bounded by the spec (`--description` is on `create` only, not `list`).

Two findings Dismissed (predicate signature asymmetry — function is private, single normalized call site, doc comment is the contract; DECISIONS.md absent Layer 5 entry — internal refactor with no spec-visible behavior change is the wrong venue for DECISIONS.md). One finding Hallucinated (predicate coupling to `Issue` struct shape — direct field coupling is the correct pattern for an internal predicate over a data model).

**Complexity budget (Dim 5):** `git show bd15a9d --stat` shows `+19/-24` for `src/lib.rs` — net **−5 lines** in the implementation commit. Phase 2a Red Gate added `+107` (predicate stub + 5 unit tests + helper); Phase 2b reduced `cmd_list` body. Net Layer 5 source change is +102 lines, dominated by tests and doc comments. The refactor genuinely shrunk the function body while adding testability — the right direction. `lib.rs` is now 948 lines total, with ~485 non-test lines (estimate: tests-and-helpers section starts at line 546). Approaching the 500-non-test-lines re-raise threshold (SA R9 Finding 3 revised condition), but not over it. Tracking note for SE Layer 6 scoping unchanged.

**Layer boundary (Dim 1):** Respected. No Layer 6 / show / delete / description bleed into Layer 5 source. The predicate has no description filter even though `Issue.description` exists; clean.

**Sycophancy check:** Two findings I tried to dismiss but could not:
- Finding 1 (rendering-extraction half remains): I tried to dismiss as "Layer 7 concern" — but SA R8 F1 explicitly argued doing it in Layer 7 conflates two changes, and Layer 5 was the third natural touch-point. Dismissal unconvincing → finding stands as Open (narrowed).
- The original F2 (extra_filter_active) — I tried to find a way it had regressed in Layer 5. It hasn't. Layer 5 didn't touch the disjunction at all because it didn't add a filter dimension. The "Resolved (re-verified)" verdict survives the dismissal attempt → confirmed Resolved.

Two findings I tried to elevate but couldn't:
- Predicate signature asymmetry (Finding 4) — I tried to argue the doc-comment contract is too weak. But the function is private and the single call site normalizes inputs. Elevation unconvincing → finding stands as Dismissed with explicit re-raise condition.
- DECISIONS.md absent Layer 5 entry (Finding 5) — I tried to argue future reviewers will lack context. But DECISIONS.md is for spec amendments and durable choices; an internal refactor is not the right venue. Elevation unconvincing → finding stands as Dismissed.

**Carry-forward status (explicit):**
- **SA R9 F1 (cmd_list extraction):** Predicate-extraction half **closed by Layer 5**. Rendering-extraction half **still Open** (Deferred to pre-Layer-7 focused PR). Net progress.
- **SA R9 F2 (extra_filter_active disjunction):** **Resolved**, re-verified for Layer 5. No new filter dimension added; property holds; regression check exists in `tests/layer5.rs:312-350`.
- **SA R8 F3 (lib.rs decomposition past 500 non-test lines):** ~485 non-test lines now (estimate). Below threshold. Tracking note for Layer 6.
- **SA R8 F4 (`cmd_create` parameter count):** still 4. Threshold at 5. Layer 6's `--description` reaches threshold. Tracking note unchanged.

**Coordination:**
- **No new SE referral.** Finding 1 (rendering-extraction half) is the existing pre-Layer-7 deferred PR, already tracked.
- **For QE (informational):** the Layer 5 unit tests (`filter_and_logic_*`, `filter_status_*`, `filter_label_match_is_case_sensitive`) genuinely close the "filter logic is testable only via subprocess integration" gap from SA R8 F1. The integration tests in `tests/layer5.rs` are correctly disclosed as Cat B Red Gate deviations (the AND-combination was emergent from prior layers).
- **For SO (informational):** no DESIGN.md changes proposed by SA. The compound-filter behavior is spec-faithful; the predicate-extraction is an internal refactor.
- **For VDD-IAR (informational):** the Phase 2a/2b split (Red Gate first, predicate body second) was executed as documented; the Phase-2a-only `#[allow(dead_code)]` was correctly removed at Phase 2b. Process compliance is intact from SA's lens.

---

## Review 12 — 2026-05-07 00:40Z

**Round:** SA Review 12 (Round-2 closure for Layer 5)
**Scope:** Verify SA R11 disposition holds after Round-1 inline fixes commit `7f9bae4`. Warm closure-verification, not a new adversarial pass.

### Round-1 finding status

- **F1 (rendering half of `cmd_list` extraction — column-width literals × 4 sites, no `format_*_row` helpers):** **Open / Deferred unchanged.** The Round-1 inline fixes did not touch the rendering half of `cmd_list` (correctly — the SA R11 disposition is "deferred to focused pre-Layer-7 PR"). SA confirms this finding is *not* claimed Resolved by `7f9bae4` and remains the only suite-wide Layer 5 Open finding pending Layer 7 prep.

### New findings

*(none this round. The other Round-1 inline fixes — SO F1/F2/F3, QE F1, SE F1 — are doc-precision-class changes that don't intersect SA's separation-of-concerns / coupling lens.)*

### Summary

SA R11 F1 carry-forward unchanged: deferred. 0 new findings this round. From SA's lens, Layer 5 is merge-ready; the rendering-half deferral is bookkept for the focused PR before Layer 7.

---

## Review 13 — 2026-05-11 01:07Z

**Round:** SA Review 13 (Layer 6 — Description + Show + Delete).
**Scope:** Cold-session adversarial SA review of the Layer 6 implementation (commits `4fb5e67` Phase 2a Red Gate, `c91676a` Phase 2b implementation). Code under review: `src/lib.rs` (1156 lines incl. tests; 665 non-test lines), `src/main.rs` (118 lines), `tests/layer6.rs` (465 lines). Primary lens: Dim 1 (layered decomposition), Dim 2 (separation of concerns), Dim 3 (coupling), Dim 4 (interface contracts), Dim 5 (complexity budget), Dim 6 (decision documentation), and the SA Review 7/8/9/11 carry-forward re-raise conditions that explicitly named Layer 6 as the trigger point.
**Session note:** Cold session per the IAR primer. Adversarial framing intact. Parallel batch with SO 20 / QE 15 / SE 15 / Security 9 / Data Engineer 9 / Platform Engineer 10 / Red Team 8 / Technical Writer 9 / UX 8 / VDD-IAR 15. Sycophancy guard: every prior re-raise watch from SA 7/8/9/10/11/12 was specifically re-evaluated against the Layer 6 source — particularly the two tripwires SA 7/8/9 set with explicit "trigger fires at Layer 6" language.

---

### Open

**Finding 1 — Both `lib.rs` decomposition (SA R8 F3 / SA R9 F3 revised) and `cmd_create` parameter-object (SA R7 F4 / SA R8 F4 / SA R10) re-raise triggers fired at Layer 6 and neither was acted on (Dim 2 — Cohesion / Dim 3 — Coupling / Carry-forward — explicit re-raise condition triggered)**

Two prior dismissals carried explicit, single-trigger re-raise conditions that named Layer 6 as the firing point. Both fired in this commit. Neither was acted on.

*Trigger 1 — non-test source past 500 lines.* SA R9 Finding 3 (revised re-raise condition, recorded in this log at `Review 9 — Dismissed F3` and re-affirmed in SA R10 / SA R11): "If Layer 5 or Layer 6 adds non-test code (i.e., excluding the `#[cfg(test)]` block) past 500 lines, the decomposition into `lib/storage.rs`, `lib/validate.rs`, `lib/commands.rs` is due."

Measurement: `src/lib.rs:666` is `#[cfg(test)]`. Non-test lines = `665`. The threshold is `500`. The threshold was crossed by `+165` (33% over). SA R11 estimated ~485 non-test lines at the end of Layer 5; Layer 6 added the four new bodies (`validate_description`, `format_show_block`, `cmd_show`, `cmd_delete`) plus doc comments and the `cmd_create` description-wiring change, totaling roughly +180 non-test lines. The Layer 6 source change is what crossed the threshold, exactly as the re-raise condition predicted.

*Trigger 2 — `cmd_create` parameter count reaches 5.* SA R7 F4 / SA R8 F4 (re-affirmed in SA R10): "Layer 6's `--description` addition pushes `cmd_create` to 5 parameters. At that point, introduce `CreateArgs { title, priority, labels, description }` and pass `(args, path)`. SE should preempt this when scoping Layer 6 to avoid a separate refactor pass."

Measurement (`src/lib.rs:229-234`):

```
pub fn cmd_create(
    title_raw: &str,
    description_raw: Option<&str>,
    priority_raw: Option<&str>,
    labels_raw: &[String],
    issues_path: &Path,
) -> Result<(), String> {
```

Five parameters. The threshold is `>5` if SA R7 F4's literal text is read strictly, or `≥5` if SA R8 F4's "5 parameter" language is read as the trigger point. Reading the *combined intent* across SA R7/R8/R10 — which all named Layer 6's `--description` as the action-now moment — the re-raise was scheduled for now, not for a later >5. The four-parameter signature was the dismissal-condition. The five-parameter signature is the re-raise-condition. The re-raise was scoped to *be done at* Layer 6, not *after* Layer 6.

Both re-raise conditions were prior commitments by SA. The Layer 6 implementer did not act on either (no module split; `cmd_create` still takes 5 positional parameters with no `CreateArgs` struct). Neither re-raise condition is mentioned in the commit message of `c91676a`.

**Self-test (sycophancy guard):** Could I dismiss this as "the refactor is scheduled for a later focused PR, same as the SA R11 F1 rendering deferral"? No — the rendering-half deferral was tied to a different layer's trigger (Layer 7's color injection). These two triggers were scheduled *for Layer 6* by their original dismissal text. Dismissing them again with the same "deferred to focused PR" framing would invalidate the prior re-raise commitment retroactively. The dismissal-rule contract was: "We accept the smaller fix now, and we commit to acting at Layer 6." Layer 6 came and went without action. Could I dismiss as "the re-raise rule was Phase-1-too-strict, revisit"? No — that would be moving the goalposts after the fact. The proper move is to raise the finding and let SO/SE decide the disposition (defer again with a *new* explicit trigger, or act now).

Could I dismiss the parameter-count half specifically as "5 parameters is still readable, the dismissal was over-cautious"? On its own merits, 5 parameters is acceptable Rust; it is not unreadable. But the SA prior commitments tied the param-object refactor to layer-stable signatures: Layer 6 is the last layer that touches `cmd_create`'s signature in any planned way; Layer 7 is polish-only. Doing the refactor *now*, in the same commit that adds the fifth param, is cheaper than doing it later (atomic test scaffolding, no separate review pass). Dismissing on "5 is still fine" undoes the cost-amortization argument the prior reviews made.

**Severity:** Medium for the lib.rs decomposition (the trigger is materially exceeded — 33% over, not borderline). Low-to-medium for the param-object (the literal count is exactly at the boundary; readability is not yet impaired). Both findings are about prior-commitment compliance more than about new-defect material.

**Classification: Open.** Disposition recommendation: SO / SE should decide whether to (a) act in a Round-2 inline fix during this IAR cycle, (b) defer to a single focused pre-Layer-7 PR (alongside the SA R11 F1 rendering-half), or (c) explicitly revise the re-raise condition with a new trigger. Whichever option is chosen, the SA carry-forward bookkeeping needs an explicit update — the current Open list is now three (this finding's two halves + SA R11 F1).

**Proposed action (if (b) is chosen):** Bundle into the pre-Layer-7 focused PR. Module split: `src/lib/mod.rs` (re-exports + `Issue`), `src/lib/storage.rs` (`load_issues`, `save_issues`, validation predicates), `src/lib/validate.rs` (`validate_title`, `validate_description`, `parse_label`, `parse_id`, `parse_status`, `parse_priority`, `display_safe`), `src/lib/commands.rs` (`cmd_create`, `cmd_list`, `cmd_status`, `cmd_show`, `cmd_delete`, `format_show_block`, `issue_matches_filters`, `truncate_with_ellipsis`, `priority_rank`, `sort_issues`). `CreateArgs` struct introduced alongside, with the call site in `main.rs` updated. Tests stay in `lib.rs` until module-level test colocation is decided.

**Coordination:** Raised to SE for disposition. Raised to SO for trigger-revision authority if (c) is preferred. Cross-reference with [SOFTWARE-ENGINEER-REVIEW.md](SOFTWARE-ENGINEER-REVIEW.md) — SE 15 is the right round to scope the action.

---

**Finding 2 — `format_show_block` encodes the 13-char label-column width as eight hand-spaced format-string literals plus a duplicated 13-space continuation indent; no `LABEL_WIDTH` / `LABEL_INDENT` constant exists; the duplication shape mirrors SA R8 F1's `cmd_list` rendering-half complaint applied to a new render site (Dim 1 — Separation of concerns / Dim 2 — Cohesion / "two of a kind makes a pattern")**

`src/lib.rs:369-377` — the show block is rendered via a single `format!` with the label column hand-spaced at every line:

```
"ID:          {}\n\
 Title:       {}\n\
 Status:      {}\n\
 Priority:    {}\n\
 Labels:      {}\n\
 Description: {}\n\
 Created:     {}\n\
 Updated:     {}\n",
```

Each prefix is hand-counted to 13 characters (`"ID:"` + 10 spaces, `"Title:"` + 7 spaces, etc.). The unit test at `src/lib.rs:1102-1126` re-counts the same 13-character widths in eight assert lines. The continuation-indent for multi-line descriptions at `src/lib.rs:366` is a separate `"\n             "` literal (1 `\n` + 13 spaces) — the 13 is duplicated from the column-width-via-spacing encoding.

There is no module-level `const LABEL_WIDTH: usize = 13;` and no `format!("{:<width$}", label, width = LABEL_WIDTH)` form. A future change to the label-column width would require updating: (a) eight format-string literals in `format_show_block`, (b) one `"\n             "` replace-target literal, (c) eight test prefix strings in `show_label_column_right_padded_to_13`, and (d) test prefix strings in `tests/layer6.rs:172-187` and `tests/layer6.rs:215-216`. DESIGN.md "Show output format" already specifies the 13-char column width, so the *value* of the constant is contractual — but the *single-source-of-truth* property is missing.

This is the same complaint shape as SA R8 F1 (rendering-half of `cmd_list` extraction): scattered width literals without a named constant. Applied to a new render site introduced in Layer 6. Two render functions (`cmd_list`, `format_show_block`) now both encode their column widths as scattered literals — two of a kind makes a pattern. The Layer 7 collision argument from SA R8 F1 applies here too in a narrower form: `format!("{:<width$}", value, width = …)` is the form Layer 7 needs if color injection is added to status/priority values inside the show block, because escape bytes break `{:<13}` padding the same way they would break `{:<11}` in the list row. The fact that Layer 7's color plan currently doesn't list show-block coloring (DESIGN.md line 234 says "color is applied only to the value text in its column cell, not to the entire row or header" — and DESIGN.md line 232 says color applies "in list and show output") makes this *more* relevant for Layer 7, not less.

**Self-test (sycophancy guard):** Could I dismiss this as "the format! is readable, the 13-char column is contractual, hand-spacing is the simplest form"? Partially — the 8-line `format!` literal *is* readable in the abstract, and a `format!("{:<13}{}", "ID:", id)` form has its own readability tax (more vertical, more boilerplate per row, the `:` placement inside the format-string vs. inside the format-argument decision). But the duplication-count argument is the actual finding: the 13 appears in 1 (format-call) + 1 (replace-target) + 8 (unit-test asserts) + at-least-2 (integration-test asserts) = 12+ source locations. That count is a real maintenance-cost number, not an aesthetic preference. Dismissing on "format! is readable" would skip the duplication argument.

Could I dismiss as "Layer 7 will add color to status/priority *value* cells, not to label cells, so the column-width concern doesn't apply"? Color is applied to *value text* per DESIGN.md line 243, and the value column starts at offset 13 — so injecting ANSI escape bytes into the status/priority values inside `format_show_block` would not break the *label* column (it's already past it), but it *would* corrupt the trailing-newline-and-13-space-continuation logic if a multi-line description ever contained color (it doesn't today, but the architectural shape is the same as the list-row collision). The pattern is the same. Dismissal-attempt fails.

Could I dismiss as "this is the same finding as SA R8 F1 rendering-half, just at a new site — log it as part of that carry-forward, not a new finding"? Reasonable accounting choice — but SA R8 F1 was explicitly about `cmd_list`. Pretending the new render function isn't a separate site is wishful-thinking accounting. The right move is: this is the *second* instance of the rendering-half pattern; the focused pre-Layer-7 PR (if that's the disposition for F1) should cover both render sites; record the new render site here so it's visible in the carry-forward bookkeeping.

**Severity:** Low. The function is short, the duplication is local, the eight-line format-string is itself readable. The finding is about pattern-recognition — Layer 6 added a *second* render site that encodes its widths as scattered literals rather than as a named constant. Worth flagging for the same focused pre-Layer-7 PR; not worth blocking Layer 6 closure for.

**Classification: Open (Deferred to the same focused pre-Layer-7 PR as SA R11 F1).** Same disposition as the existing SA R11 F1 rendering-half deferral, scoped to include `format_show_block` alongside `cmd_list` rendering.

**Proposed action:** In the pre-Layer-7 PR, introduce `const LABEL_WIDTH: usize = 13;` at module level. Refactor `format_show_block` to use `format!("{:<width$} {}", "ID:", id, width = LABEL_WIDTH)` (or a small `fn show_row(label: &str, value: &str) -> String` helper). Refactor the continuation-indent to `"\n".to_string() + &" ".repeat(LABEL_WIDTH)` or equivalent. Update the test prefix strings to derive from the constant.

**Coordination:** Cross-reference with [SOFTWARE-ENGINEER-REVIEW.md](SOFTWARE-ENGINEER-REVIEW.md) — SE 15 has the action.

---

### Resolved

*(no findings resolved this round. The Layer 6 source change did not touch the rendering-half of `cmd_list` (correctly per the SA R11 deferral), did not touch `extra_filter_active` (correctly — `cmd_list`'s filter set is unchanged), and added two new bodies that are independent of all prior Open findings.)*

---

### Dismissed

**Finding 3 — `format_show_block`'s `\r\n` → `\n` normalization at render time is an undocumented architectural choice that conflicts with DECISIONS.md "No Windows line-ending normalization" / SA Review 1 Finding 5 (Dim 6 — Decision documentation)**

`src/lib.rs:365` — `format_show_block` does `let normalized = d.replace("\r\n", "\n");` on the description value before splitting at `\n`. DECISIONS.md lines 81-83 record: "No Windows line-ending normalization — `\r\n` is not normalized to `\n` on storage. Target platform is macOS." The commit message of `c91676a` mentions the normalization ("`\r\n` separators are normalized to `\n` before splitting so a CRLF-stored description renders without a stray `\r` in the first line") but DESIGN.md does not, and DECISIONS.md does not.

A future reader of DECISIONS.md cold sees "we don't normalize line endings" and may not realize there's a render-time normalization in `format_show_block` (different concern: not storage, not list-rendering — just the multi-line description-indent in show output).

**Classification: Dismissed.** Demonstration that the control holds:

1. **DECISIONS.md "No Windows line-ending normalization" is about *storage* — the input/output boundary of `tracker.json`.** The original decision text and its SA Review 1 source are about whether to rewrite `\r\n` to `\n` on the way *into* the file. The Layer 6 normalization is at *render* time only — the stored value is still verbatim. These are different concerns; there is no contradiction.

2. **The normalization is a defensive render-side detail, not a spec amendment.** The DESIGN.md "Show output format" example shows a multi-line description with continuation lines indented 13 spaces. If a stored description contains `\r\n` (because the user piped a file or pasted from Windows), the spec is silent on whether the `\r` should render as a stray invisible character at the end of each line. The Layer 6 implementer chose: strip the `\r` so the indent works. The alternative (preserve the `\r`) would visibly corrupt the show-block alignment with no user-visible benefit. The choice is correct and defensible.

3. **DECISIONS.md is for *durable* architectural choices and *spec amendments* (SA R11 F5 dismissal text).** A render-side detail in a single private function is the wrong venue. The commit-message rationale is the appropriate documentation venue for this class of change, exactly per SA R11 F5's dismissal-rule.

4. **The defensive normalization is bounded:** it does not affect storage (stored description still contains `\r\n` per "stored verbatim"); it does not affect `list` (description is not rendered in list); it only affects the show-block rendering. The blast radius is one private function.

The control holds: the choice is correct, the venue (commit message) is appropriate, and DECISIONS.md "no line-ending normalization" is about a different concern (storage) so no contradiction exists. Dismissal-attempt to elevate: I tried to argue future readers will be confused. But the function-level comment at `src/lib.rs:362-364` explains the choice in-place; that's where a future reader of `format_show_block` will look first. The control holds.

**Re-raise condition:** if a future render decision introduces a *spec-visible* line-ending behavior change (e.g., a `--strict-storage` mode that round-trips `\r\n` through show output), DECISIONS.md is the right venue at that point.

---

**Finding 4 — `validate_description`'s contract is *describe* the un-trimmed input, in contrast to `validate_title`'s *trim* the input; the asymmetry is documented in the doc-comment but not enforced at the type level (Dim 4 — Interface contracts)**

`src/lib.rs:326-340`. The doc comment at lines 326-334 explicitly notes the contract: "DESIGN.md Feature 1: `--description` must be non-empty after trim, but the *stored* value is the input verbatim (not trimmed). This function returns the un-trimmed input on success so the caller can write it as-is." `validate_title` at lines 56-77 has the same `Result<String, String>` shape but the returned value is trimmed. A caller that confuses the two (passes the result of `validate_description` to a code path that expected a trimmed value, or vice versa) gets a silent semantic error.

**Classification: Dismissed.** Demonstration that the control holds:

1. **Both functions have a single call site each — `cmd_create`.** `cmd_create:236` for title; `cmd_create:237-240` for description. There is no third call site that would conflate them. (`tests` calls them in unit-test context where the caller controls the assertion.)

2. **The doc comment is the contract for a function whose only two callers are: `cmd_create` and a unit test.** Promoting the contract to a type-level distinction (e.g., separate `TrimmedTitle(String)` and `VerbatimDescription(String)` newtypes) would add ceremony for a single-call-site invariant. SA R11 F4 dismissed an analogous "doc-comment-as-contract" concern with the same reasoning.

3. **The asymmetry is the spec's:** DESIGN.md Feature 1 postconditions specify the title-trim-on-store and description-verbatim-on-store rules explicitly. The function shape mirrors the spec asymmetry rather than introducing it. If anything, the asymmetry is correct documentation discipline — the *function* asymmetry surfaces the *spec* asymmetry.

4. **The dismissal-attempt to elevate:** I tried to argue that future maintenance might add a second call site (e.g., a hypothetical `tracker edit` that re-validates the description). But (a) DESIGN.md "Out of Scope" line 396 explicitly excludes editing after creation, and (b) if such a feature were ever added, the spec amendment would surface the trim-vs-verbatim choice and the function's caller would consult the doc-comment-or-callsite-pattern at that point. The contract is sufficient for the current spec.

The control holds: doc-comment-on-private-call-site-pair is the right contract level for an internal validation helper. Marking this finding Dismissed requires demonstrating the alternative would be worse; the alternative (newtype wrapper) is heavier than the problem.

**Re-raise condition:** if `validate_description` or `validate_title` is ever exported beyond the lib crate, or if a second call site is added that does not flow through `cmd_create`, revisit immediately.

---

### Hallucinated

**Finding 5 — `cmd_show` and `cmd_delete` both call `parse_id` + `load_issues` + find-by-id, then diverge into "render" vs. "mutate"; the shared prelude is duplicated rather than extracted into a `fn find_issue_by_id(id_raw, path) -> Result<(Vec<Issue>, usize), String>` helper (Dim 2 — Cohesion / Dim 3 — Coupling)**

Initial concern: `src/lib.rs:398-409` (cmd_show) and `src/lib.rs:422-432` (cmd_delete) share the same opening lines — `parse_id` + `load_issues` + position-or-find by id. The shared prelude is duplicated. A `find_issue_by_id(id_raw, path) -> Result<(Vec<Issue>, usize), String>` (or `find_issue_index`) helper would centralize the id-not-found error message ("Issue #N not found.") and the parse-then-load sequence.

**Classification: Hallucinated.** Demonstration that the control holds:

1. **The two functions actually diverge on the *third* step, not the *prelude*.** `cmd_show` uses `.iter().find(…)` (returns `&Issue`); `cmd_delete` uses `.iter().position(…)` (returns `usize`). The shared *prelude* is two lines (`let id = parse_id(id_raw)?; let mut/let issues = load_issues(issues_path)?;`). Extracting a two-line shared prelude into a helper that has to return *either* a borrowed reference *or* an index — i.e., a `Result<(Vec<Issue>, Either<&Issue, usize>), String>` — would be heavier than the duplication. The Rust borrow checker would make the unified-helper form ugly: returning `(Vec<Issue>, &Issue)` from the same function requires lifetime annotations that propagate to the caller, and returning `(Vec<Issue>, usize)` for both call sites means `cmd_show` would have to re-index after the call, paying for ergonomics it didn't ask for.

2. **`cmd_status` (`src/lib.rs:311-324`) has the exact same shape:** `parse_id` + `load_issues` + `position(|i| i.id == id)` + diverge into "mutate this field". Three call sites with the same prelude *plus* divergent middle-and-tail is the right shape for a private-helper extraction only if all three call sites want the same return type. They don't — `cmd_show` wants a borrowed reference for read-only access (avoid a clone or an index into a freshly-loaded Vec), `cmd_delete` wants an index for `Vec::remove`, `cmd_status` wants an index for `Vec[idx].status = …`. Two of three want the index; one wants the reference. A helper that returns the index would impose a re-lookup on `cmd_show`. A helper that returns the reference would force `cmd_delete` and `cmd_status` to do a second pass to get the index.

3. **The duplication is the *correct* coupling shape for the Rust borrow-checker constraints.** The shared lines are mechanically simple (parse + load + find); their duplication is cheap. The non-shared lines (the mutation/render divergence) are where the complexity lives; extracting the shared part would push complexity *into* the helper signature (lifetime parameters, enum-of-results) at no readability gain.

4. **The `Issue #N not found.` error string is duplicated three times across `cmd_status` / `cmd_show` / `cmd_delete`.** This is the real candidate for centralization — `fn not_found_error(id: u64) -> String`. But the savings (three callsite literals → one constant) is small and the constant version is no clearer than the inline `format!`. Marginal at best.

The control holds: the duplication is the right cost-of-doing-business for the divergent-Result-type pattern. Marking this finding Hallucinated requires demonstrating the alternative would be worse; the demonstration is above. The two-line prelude is not a maintenance burden, and the alternative shapes are uglier.

---

### Summary

Two real findings, both prior-commitment re-raises that explicitly named Layer 6 as the trigger:

1. **Finding 1 — both `lib.rs` decomposition (non-test code past 500 lines) and `cmd_create` parameter-object (5 parameters reached) re-raise triggers fired in Layer 6 and neither was acted on.** Non-test lib.rs is 665 lines (target: ≤500; 33% over). `cmd_create` has 5 parameters (target: ≤4 without param-object). Both were scheduled-for-Layer-6 by prior dismissal text. Disposition: Open; SO/SE to choose Round-2 inline fix vs. focused pre-Layer-7 PR vs. trigger-revision.

2. **Finding 2 — `format_show_block` is a new render site (Layer 6 added it) and it encodes the 13-char label-column width as eight hand-spaced format-string literals plus a duplicated 13-space continuation-indent.** Same architectural shape as SA R8/R9/R11 F1's `cmd_list` rendering-half complaint, applied at a new site. Bundle into the same pre-Layer-7 focused PR.

Three findings reviewed and Dismissed/Hallucinated:

- **Finding 3 (Dismissed)** — `\r\n` → `\n` normalization at show-render time vs. DECISIONS.md "no line-ending normalization." Different concern (render vs. storage); no contradiction; commit-message-as-documentation venue is correct for a render-side detail.
- **Finding 4 (Dismissed)** — `validate_description` returns un-trimmed in contrast to `validate_title`'s trimmed return. Asymmetry is the spec's, not the function's. Doc-comment-on-single-call-site-pair is the right contract level for an internal validation helper.
- **Finding 5 (Hallucinated)** — shared `parse_id` + `load_issues` + find prelude across `cmd_show` / `cmd_delete` / `cmd_status`. Divergent Result types make the unified-helper form heavier than the duplication; the duplication is the correct coupling shape for the Rust borrow-checker constraints.

**Complexity budget (Dim 5):** `git show c91676a --stat` shows `src/lib.rs` +90/-43 = net +47 lines in the implementation commit. Phase 2a Red Gate added +145 lines (stubs + tests). Net Layer 6 source change is +192 lines, dominated by `format_show_block`'s body and three integration-test additions in `tests/layer6.rs`. Predicate-extraction pattern from Layer 5 was *not* applied to Layer 6 — there is no `show_matches_filter` or similar predicate; the Layer 5 pattern was specific to filter-AND-combination, not render. Layer 6's natural predicate-extraction analogue would be the rendering-half refactor (Finding 2) — same pattern, different site — but it was not applied. `lib.rs` is now 1156 lines total, 665 non-test (Finding 1's trigger).

**Layer boundary (Dim 1):** Respected. No Layer 7 surface leak — no color output in `format_show_block`, no `IsTerminal` plumbing, no `--help` polish. The implementation is scoped to Layer 6 features exactly.

**Separation of concerns (Dim 2):** `format_show_block` is a *mostly*-pure rendering function (it builds a `String`, no I/O). `cmd_show` is the thin I/O wrapper around it: parse, load, find, print. This split is clean and is the architectural shape DESIGN.md "Testing Methodology / Purity guidance" recommends. The corresponding split was *not* done for `cmd_list` (rendering still inline; SA R11 F1 carry-forward). The `cmd_show` split is the right pattern; the `cmd_list` rendering should follow it in the pre-Layer-7 PR.

**Coupling (Dim 3):** `format_show_block` couples to `Issue` field shape directly (`issue.id`, `issue.title`, `issue.status`, etc.) — this is the *correct* coupling for an internal renderer over a data model (SA R11 F6 dismissal logic applies symmetrically). `cmd_show` and `cmd_delete` couple to `parse_id` / `load_issues` / `save_issues` via direct function calls — correct internal coupling.

**Interface contracts (Dim 4):** `cmd_show` (lib.rs:389-397) and `cmd_delete` (lib.rs:411-421) have faithful doc comments — preconditions, postconditions, error states all named. `validate_description` (lib.rs:326-334) explicitly documents the un-trimmed-return contract (see dismissed Finding 4 for the contract-level analysis).

**Sycophancy check:** Two findings I tried to dismiss but could not:
- Finding 1 (both Layer-6-tripwire triggers fired without action) — I tried to dismiss as "defer to focused PR, same as SA R11 F1." But the original dismissals named Layer 6 as the *action* moment, not the *defer-again* moment. Dismissing the trigger after it fires would invalidate the prior re-raise contract retroactively. Dismissal unconvincing → finding stands as Open.
- Finding 2 (new render-site duplication in `format_show_block`) — I tried to dismiss as "the 8-line format! is readable in isolation." But the duplication-count argument (1 format + 1 replace-target + 8 unit-test asserts + 2 integration-test asserts = 12+ source locations) is a real maintenance-cost number. Dismissal unconvincing → finding stands as Open (deferred to pre-Layer-7 PR).

Three findings I tried to elevate but couldn't:
- Finding 3 (`\r\n` normalization undocumented) — I tried to argue future-reader confusion. But function-level comment exists and DECISIONS.md "no line-ending normalization" is about storage, not render. Elevation unconvincing → Dismissed.
- Finding 4 (`validate_description` un-trimmed return) — I tried to argue type-level enforcement. But single call site + doc-comment-as-contract + asymmetry-is-the-spec's. Elevation unconvincing → Dismissed.
- Finding 5 (prelude duplication across cmd_show/cmd_delete/cmd_status) — I tried to argue helper-extraction. But divergent Result types and Rust borrow-checker constraints make the helper-form heavier. Elevation unconvincing → Hallucinated.

**Carry-forward status (explicit):**
- **SA R7 F4 / SA R8 F4 (`cmd_create` parameter count, threshold at 5):** **Trigger fired in Layer 6.** Re-raised as Finding 1 half-A. Was the explicit Layer-6-action expectation.
- **SA R8 F3 / SA R9 F3 revised (`lib.rs` decomposition, non-test past 500 lines):** **Trigger fired in Layer 6.** Re-raised as Finding 1 half-B. Was the explicit Layer-5-or-Layer-6-action expectation; the threshold was 33% exceeded.
- **SA R8 F1 / SA R9 F1 / SA R11 F1 (`cmd_list` rendering-half extraction):** **Unchanged.** Layer 6 did not touch `cmd_list` rendering; correctly per the SA R11 deferral. The rendering-half deferral now also covers a new render site (`format_show_block`, Finding 2). Pre-Layer-7 PR scope expanded.
- **SA R9 F2 (`extra_filter_active` disjunction):** **Resolved, unchanged.** Layer 6 did not add a `list` filter (description is `create`-only per DESIGN.md); the property holds trivially.
- **SA R11 F4 / SA R11 F6 (doc-comment-as-contract; data-model coupling for predicates):** **Patterns applied symmetrically to Layer 6.** `validate_description` doc-contract dismissed under R11 F4 logic; `format_show_block` data-model coupling not raised under R11 F6 logic.

**DECISIONS.md (Dim 6):** No Layer 6 entry added. Following SA R11 F5 dismissal-rule precedent: internal refactors / render-side defensive details (the `\r\n` normalization) don't warrant a DECISIONS.md entry. The non-confirmation-on-delete decision (DECISIONS.md lines 37-39) was the only spec-visible Layer 6 choice, and that was added under the D1 deviation entry in DESIGN.md (lines 413-420) ahead of Layer 6 implementation — correctly. The Layer 6 commit-message rationale is the appropriate documentation venue for the render-side details.

**Coordination:**
- **Raised to SE (Finding 1):** decide Round-2 inline fix vs. focused pre-Layer-7 PR vs. trigger-revision. The two halves can be acted on independently if needed (param-object now, module-split later, or vice versa) but both have the same disposition decision. Cross-reference with [SOFTWARE-ENGINEER-REVIEW.md](SOFTWARE-ENGINEER-REVIEW.md) — SE 15 has the action.
- **Raised to SE (Finding 2):** bundle `format_show_block`'s `LABEL_WIDTH` constant extraction into the pre-Layer-7 PR alongside the `cmd_list` rendering-half. Same focused-PR scope; lower-priority within that scope.
- **Raised to SO (Finding 1):** trigger-revision authority. SO may choose to extend the dismissal contract (revised re-raise condition with a new explicit Layer 7 trigger) rather than acting now. Either path is acceptable from SA's lens; what is *not* acceptable is silently letting the trigger fire and continuing the prior dismissal as if it had not fired.
- **For QE (informational):** the Layer 6 unit tests in `lib.rs:1085-1138` are well-scoped (multiline format, label-column padding, max+1 ID assignment). Cat B Red Gate disclosure for the `max_id_plus_one_skips_deleted_ids` test is correct (Layer 1's `next_id` already returns max+1). No SA gap to flag.
- **For SO (informational):** DESIGN.md is faithful to the Layer 6 implementation; no spec amendments proposed. The non-confirmation-delete decision is correctly recorded in the D1 deviation. The `\r\n`-normalization choice is appropriately at commit-message-level (not DESIGN.md / not DECISIONS.md).
- **For VDD-IAR (informational):** Phase 2a Red Gate (4fb5e67) → Phase 2b implementation (c91676a) split was executed as documented. The Phase-2a `#[allow(dead_code)]` annotations on `validate_description` and `format_show_block` were correctly removed at Phase 2b (both now on the production path). Process compliance is intact from SA's lens.

**Architectural concerns next-tier reviewers should know about:** SE will receive Finding 1 (the carry-forward trigger fires) and needs to make the disposition call — Round-2 inline fix, pre-Layer-7 focused PR, or trigger-revision. The right answer is likely the focused PR (bundles cleanly with SA R11 F1's rendering-half + Finding 2's `LABEL_WIDTH` extraction), but the decision is SE/SO's. If the dispostion is the focused PR, the scope at that time is: (a) `cmd_list` rendering extraction with `ID_WIDTH` / `STATUS_WIDTH` / `PRIORITY_WIDTH` / `LABELS_WIDTH` / `TITLE_WIDTH` constants and `format_header_row` / `format_issue_row` helpers (SA R11 F1); (b) `format_show_block` `LABEL_WIDTH` constant + helper (Finding 2); (c) `lib.rs` module split into `storage`/`validate`/`commands` (Finding 1 half-A); (d) `CreateArgs` struct + `cmd_create` signature update (Finding 1 half-B). All four are architectural prep work that benefits from being done in a single focused PR with its own test scaffolding rather than scattered through Layer 7's color/help work.

**Coordination:** *(none — closure pass)*
