# Red Team Review Log

This review is part of the [Iterative Adversarial Refinement (IAR)](README.md) suite. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

**Reviewer role: Red Team Hacker** (Penetration Tester / Offensive Security Engineer)

**Activation:** User-controlled CLI input; file I/O operations. Run after Security Engineer.

**Language supplement applied:** `lang/rust.md` (Red Team section) + `lang/cli.md` (Red Team section).

**Sycophancy check:** An agent that built the application will rationalize its defenses as adequate because it believes in the controls it generated. The Red Team does not evaluate intent — it evaluates outcome. For every control, ask: "can this be bypassed by a caller who does not follow the happy path?" An application where every attack is dismissed as "not applicable" has not been red-teamed — it has been reassured.

---

## Review 1 — 2026-04-27 21:00Z

**Scope:** `DESIGN.md` attack surface review. No source code. Post-Security review pass.

**Session note:** In-session, post-Security review. Acknowledged quality tradeoff.

**Posture:** I am looking for ways to make the tool behave incorrectly, panic, corrupt data, or reveal information it should not. The threat model (single-user local CLI, no network, no auth) significantly constrains the attack surface — but that assessment must be earned, not assumed.

---

### Resolved

**Finding 1 — Crafted `tracker.json` with invalid domain values (Dim 6, cross-reference Security Finding 1)**

A malicious or accidentally crafted `tracker.json` with valid JSON structure but invalid domain values (e.g., `"status": "flying"`, `"id": -1` encoded as a large u64 near `u64::MAX`, `"title": ""`) could produce undefined behavior in an implementation that trusts deserialized data.

For a single-user local tool, the scenario is: the user manually edits `tracker.json` and introduces an invalid value. The question is whether the tool crashes, silently uses the invalid data, or errors cleanly.

[SECURITY-REVIEW.md](SECURITY-REVIEW.md) Review 1 resolved this by adding the post-deserialization validation requirement to DESIGN.md. Invalid domain values in deserialized data now trigger the corrupt-data error path (exit 1, informative stderr message).

**Resolution:** Resolved by [SECURITY-REVIEW.md](SECURITY-REVIEW.md) Review 1 Finding 1. The spec now requires post-deserialization validation. The Red Team confirms this is the correct mitigation — the tool should treat all file-read data as untrusted.

---

### Accepted Risk

**Finding 2 — Integer overflow on ID counter at `u64::MAX` (Rust supplement — integer overflow)**

ID assignment: `max(existing_ids) + 1`. If `max(existing_ids)` equals `u64::MAX` (18,446,744,073,709,551,615), the `+ 1` wraps to 0 in Rust release builds (overflow is defined to wrap for release mode arithmetic, unlike debug mode which panics).

**Classification:** Accepted Risk. A personal issue tracker would need to create 18.4 quintillion issues to reach this condition. This is not a real threat. Accepted without mitigation. If the implementation uses `checked_add` this is a free mitigation; if it uses `+`, the overflow risk is categorically acceptable. Risk owner: the user/developer.

---

### Dismissed

**Finding 4 — Panic via crafted CLI input (Rust supplement — panic as DoS)**

`.unwrap()` on values derived from user input can panic and crash the binary. For a single-user CLI, a crash is inconvenient but not a denial-of-service concern — the user is attacking themselves. The Rust supplement notes this is a DoS vector for server applications; for a CLI the stakes are much lower.

**Classification:** Dismissed at Review 1. No implementation exists yet — re-evaluation tracked in Review 2 when code is available. The Rust SE supplement requires `.unwrap()` discipline on user-facing paths; SE Review will verify when code exists. For a CLI tool, a panic from user input is a quality defect (unhelpful error message) not a security vulnerability. The DESIGN.md Constraints section requires crash-safe I/O for file operations specifically. No spec change needed.

---

**Finding 5 — Supply chain attack on Cargo dependencies (Rust supplement — crates.io supply chain)**

No `Cargo.toml` exists. No dependencies declared.

**Classification:** Dismissed at Review 1; re-evaluated in Review 2 once dependencies exist. [PLATFORM-ENGINEER-REVIEW.md](PLATFORM-ENGINEER-REVIEW.md) findings (`cargo audit`, `cargo deny`) cover this.

---

### Hallucinated

**Finding 3 — Path traversal via `tracker.json` (Rust supplement — path traversal)**

The file path `tracker.json` is hardcoded in the implementation. No user-supplied path component is involved. There is no path traversal surface.

**Classification:** Hallucinated. Path traversal requires user-controlled path components. The spec contains none.

---

### Open

*(none)*

---

### Summary

The attack surface of this tool is extremely small: hardcoded file path, no network, no auth, single user, all input validated at the CLI boundary. The one real pre-implementation finding (crafted `tracker.json` with invalid domain values) was already resolved by Security Review 1. Two findings carried forward (panic discipline, supply chain) for re-evaluation in Review 2. One hallucinated (path traversal). One accepted risk (u64 overflow — not a real threat). Maximum viable refinement is close for the pre-implementation phase.

**Coordination:** Finding 1 cross-referenced with [SECURITY-REVIEW.md](SECURITY-REVIEW.md) Review 1. Findings 4 and 5 carry forward for evaluation when code exists.

---

---

## Review 2 — 2026-04-28 05:30Z

**Scope:** Layer 1 implementation — `src/lib.rs`, `src/main.rs`, `Cargo.lock`. Attempting to crash, corrupt, or cause undefined behavior through crafted inputs and files. Looking for ways to break this binary; small attack surface is a conclusion I must earn.

**Session note:** In-session with Layer 1 IAR suite. Acknowledged quality tradeoff.

**Posture:** Adversarial — break the binary; do not trust the prior reviewer's "small attack surface" framing.

**Regression check:** Review 1 carried forward Findings 4 (panic discipline) and 5 (supply chain) for evaluation when code exists. Both are re-evaluated below.

---

### Resolved

**Finding 1 — Panic via crafted CLI input (Rust supplement — panic as DoS) (regression check from Review 1 Finding 4)**

Audit of all `.unwrap()` calls in `lib.rs` and `main.rs`:

- `lib.rs:49` — `serde_json::to_string_pretty(issues).unwrap()`: serialization of `Vec<Issue>` with known-serializable field types. Cannot fail. Accepted.
- `lib.rs` — `?` propagation on all user-facing paths (file read, JSON parse, domain validation). No unwrap on any code path reachable from user input or file content.
- `main.rs` — `Cli::parse()`: clap handles all argument parsing failures with its own error output and exit 1. No unwrap.

No user-reachable panic surface found.

**Resolution:** No unsafe unwrap on user-facing paths. Review 1 Finding 4 discharged.

---

**Finding 2 — Supply chain attack on Cargo dependencies (Rust supplement — crates.io supply chain) (regression check from Review 1 Finding 5)**

`cargo audit` run against `Cargo.lock` (100 packages: serde, serde_json, clap, chrono and all transitive deps): **0 advisories**. No known vulnerabilities in the dependency tree.

**Resolution:** Review 1 Finding 5 discharged.

---

**Finding 3 — Crafted `tracker.json` with invalid domain values (regression check from Review 1 Finding 1) (Dim 6)**

Re-evaluated against the implementation. The initial implementation was vulnerable: `load_issues` deserialized without domain validation, allowing a crafted `tracker.json` with `"status": "flying"` to be silently processed. [SECURITY-REVIEW.md](SECURITY-REVIEW.md) Review 3 / [DATA-ENGINEER-REVIEW.md](DATA-ENGINEER-REVIEW.md) Review 3 identified and resolved this gap — `load_issues` now validates all field domain values and rejects any issue with an invalid value via the corrupt-data error path.

**Resolution:** Resolved via [SECURITY-REVIEW.md](SECURITY-REVIEW.md) Review 3. The mitigated path now exits 1 with an informative message for any crafted file with invalid domain values.

---

### Accepted Risk

**Finding 4 — Integer overflow on ID counter (regression check from Review 1 Finding 2) (Rust supplement — integer overflow)**

Unchanged assessment. `u64::MAX` + 1 overflow is unreachable in any realistic use.

**Classification:** Accepted Risk. Risk owner: the user/developer.

---

### Dismissed

**Finding 5 — Zero-id crafted file (Dim 6)**

A `tracker.json` with `"id": 0` is now caught by the `issue.id > 0` check in `issue_fields_are_valid()`. Exit 1, corrupt-data error. Verified by reading the validation logic. Test `invalid_domain_values_in_json_causes_error_exit` covers the adjacent case (`"status": "flying"`); zero-ID is structurally identical.

**Classification:** Dismissed. Handled by the post-deserialization validation. No additional test required at Layer 1; the validation path is the same for any failing field.

---

### Hallucinated

**Finding 6 — Path traversal (regression check from Review 1 Finding 3)**

File path is hardcoded. Unchanged assessment.

**Classification:** Hallucinated.

---

### Open

*(none)*

---

### Summary

Both carried-forward findings from Review 1 resolved: `.unwrap()` discipline verified (no panic surface on user-facing paths); supply chain audit clean (0 advisories). The crafted-file attack path is now mitigated by post-deserialization validation. The tool's attack surface is as small as its deployment context warrants. Maximum viable refinement reached for Layer 1.

**Coordination:** Finding 3 resolved jointly with [SECURITY-REVIEW.md](SECURITY-REVIEW.md) Review 3 and [DATA-ENGINEER-REVIEW.md](DATA-ENGINEER-REVIEW.md) Review 3.

---

---

## Review 3 — 2026-04-30 00:00Z

**Scope:** Layer 1 gate closure pass — no code changes since Review 2.

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

No new attack surface. All prior findings resolved. No Red Team findings. MVR reached for Layer 1.

**Coordination:** *(none)*

---

---

## Review 4 — 2026-05-01 00:00Z

**Scope:** Layer 2 implementation — adversarial evaluation of new entry points: `tracker status <id> <status>` and `tracker list --status <s>`.

**Session note:** In-session with full Layer 2 IAR suite. Acknowledged quality tradeoff.

**Posture:** Adversarial — attempting to crash, corrupt data, or produce undefined behavior through crafted inputs and sequences.

---

### Dismissed

**Finding 4 — Crafted `tracker.json` with a status field not in `VALID_STATUSES` (Dim 6) (regression check)**

Unchanged from Layer 1 analysis. `issue_fields_are_valid` catches this; `load_issues` returns `Err(CORRUPT_DATA_ERROR)` before any command executes. The mitigation is in place and unchanged.

**Classification:** Dismissed. Already mitigated; unchanged from Layer 1.

---

### Hallucinated

**Finding 1 — Status string injection (CLI input path) (Dim 6)**

`tracker status 1 "done; rm -rf /"` — `parse_status` rejects any value not in `{"open", "in-progress", "done"}`. The string is never passed to a shell, never written to disk, never interpreted. The boundary is `VALID_STATUSES.contains(&lower.as_str())`.

**Classification:** Hallucinated. No injection surface exists.

---

**Finding 2 — Very large ID string (Dim 5)**

`tracker status 9999999999999999999999 done` — `parse::<u64>()` returns `Err` for values exceeding `u64::MAX`. No panic, no storage access, clean error message.

**Classification:** Hallucinated. `u64` parsing handles overflow correctly.

---

**Finding 3 — Rapid status toggling to produce timestamp collision (Dim 5)**

Two rapid `tracker status` invocations within the same second would produce identical `updated_at` timestamps (second precision). No state corruption results — the second write just overwrites with the same timestamp. The spec notes this: "`updated_at` after a status change is `>=` `updated_at` before the change" (allowing equal).

**Classification:** Hallucinated. The timestamp equality case is spec-defined and non-corrupting.

---

### Open

*(none)*

---

### Summary

No new attack vectors. Layer 2 adds two entry points; both are fully validated. The attack surface remains the same bounded scope as Layer 1: no shell execution, no user-controlled file paths, all inputs validated before use. No Red Team findings. MVR reached for Layer 2.

**Coordination:** *(none)*

---

---

## Review 5 — 2026-05-04 22:50Z

**Scope:** Layer 3 implementation. Attempting to crash, corrupt data, brick the tool, or produce undefined behavior through crafted CLI inputs and crafted `tracker.json` files. Exercising `cargo build --release` binary directly against attack inputs rather than reading code in isolation.

**Session note:** Cold session per primer; parallel batch run with other domains; Security running concurrently in separate session.

**Posture:** Adversarial. Threat model includes hand-edited `tracker.json` passed between machines, malicious crates in dependency tree, command-line input from untrusted sources. The prior reviewers' "small attack surface" framing has been earning credit on each layer; this session re-tests rather than re-asserts.

**Regression check:** All Review 1–4 findings remain mitigated *with one exception*: Finding 2 (Review 1) / Finding 4 (Review 2) — the integer overflow at `u64::MAX` previously classified Accepted Risk on the rationale of "would need 18.4 quintillion issues" — is reopened below as a concrete, single-step data-destruction attack against the threat model the project actually adopted (hand-edited files passed between machines). The original rationale evaluated only one attack path (organic counting) and did not consider crafted input.

---

### Open

**Finding 1 — ANSI/control-sequence injection in titles via CLI (Dim 5, 7) — confirmed by direct exploit**

Title strings are stored as opaque text and re-emitted to stdout by `cmd_list` via `println!("{:<4}  {:<11}  {:<8}  {:<20}  {}", ...)` (`src/lib.rs:264-267`). No filter exists for ASCII control bytes (0x00–0x1F, 0x7F) or for Unicode control codepoints. A title supplied at the CLI containing `\x1b[2J\x1b[H` clears the user's screen on the next `tracker list`; OSC 8 hyperlinks (`\x1b]8;;https://evil.example/\x1b\\Click here\x1b]8;;\x1b\\`) render as a clickable hyperlink labelled "Click here" pointing to an attacker URL; backspace bytes (`\x08`) overwrite previously-printed text in the row.

**Reproduction:**

```sh
TMP=$(mktemp -d) && cd "$TMP"
tracker create "$(printf 'Innocuous')"
tracker create "$(printf 'Looks_safe\b\b\b\b\b\b\b\b\b\bDANGEROUS')"
tracker list   # row 2 displays as "DANGEROUSe" (or similar) due to BS overwrite

# OSC 8 hyperlink phishing:
rm tracker.json
tracker create "$(printf '\033]8;;https://evil.example/\033\\Click here\033]8;;\033\\')"
tracker list   # "Click here" is a clickable terminal hyperlink to evil.example
```

DESIGN.md Edge Cases / Title says only "Title containing quotes or special shell characters → shell responsibility; the binary receives the raw string after shell expansion and treats it as opaque text." That treats the binary as not responsible for what it then *prints back to the terminal*. For the threat model the project adopted (multi-machine sharing of `tracker.json`, untrusted CLI history), this is a real client-side attack. The same issue applies to `description` and `labels` (DESIGN.md spec for Layer 4 / 5) once those flags exist.

**Classification:** Open. Ask Solution Owner whether DESIGN.md should require sanitization of control bytes in stored fields (or escaping at output). At minimum, ASCII C0 controls (other than `\n` in description) and the OSC introducers should be either rejected on input or rendered with C-style escapes when printed. Cross-reference: Security review (controls present?), QE review (no test exercises control-byte input).

---

**Finding 2 — Integer overflow on `next_id` produces `id: 0`, which then bricks the tracker (Dim 5) — escalation of Review 1 Finding 2 / Review 2 Finding 4**

`next_id` (`src/lib.rs:39-41`) computes `existing_ids.iter().max().copied().unwrap_or(0) + 1`. In release builds, when `max(existing_ids) == u64::MAX`, `+ 1` wraps to `0`. The newly-created issue is then written to `tracker.json` with `id: 0`. On the next read, `issue_fields_are_valid` rejects `id == 0` and `load_issues` returns `CORRUPT_DATA_ERROR`. **The tool then refuses every subsequent read, instructing the user to "Delete tracker.json to start fresh" — destroying the entire tracker history including the user's legitimate issues.**

**Reproduction:**

```sh
TMP=$(mktemp -d) && cd "$TMP"
# Plant one crafted issue (simulating a hand-edited tracker.json delivered to the user):
printf '[{"id":18446744073709551615,"title":"plant","status":"open","priority":"low","labels":[],"created_at":"2026-01-01T00:00:00Z","updated_at":"2026-01-01T00:00:00Z"}]' > tracker.json
tracker create "user's real work"   # → "Created issue #0: user's real work"
tracker list                         # → "Error: Could not read tracker data. ... Delete tracker.json to start fresh."
```

The prior accept-risk rationale ("18.4 quintillion organic issues") is sound for the organic case but was the wrong threat model. The adopted threat model includes crafted input files. A single planted issue with `id: u64::MAX` is sufficient to brick the user's tracker the next time they run `tracker create`. This is a data-destruction attack in one CLI call.

Mitigation options for the SO to choose from: (a) `next_id` uses `checked_add` and surfaces a "tracker is full" error without writing; (b) `load_issues` rejects `id >= u64::MAX` (capped), defending in depth; (c) DESIGN.md adds an explicit upper bound on `id` and `load_issues` rejects above it. Any of these closes the brick path.

**Classification:** Open. Cross-reference with [SECURITY-REVIEW.md](SECURITY-REVIEW.md) — this is also an absent input-validation control. Cross-reference with [QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md) — no test exercises the `u64::MAX` planted-issue path.

---

**Finding 3 — Newlines and other layout-breaking bytes in `title` produce malformed `list` output (Dim 5)**

`cmd_list` formats one row per issue with `println!`. A title supplied at the CLI as `Line1\nLine2` (multi-line) produces a row that visually spans two lines and breaks the documented "one issue per line" tabular format. Reading any issue with `\n` or `\t` in its title from a tracker.json (hand-edited) reproduces the same effect.

**Reproduction:**

```sh
TMP=$(mktemp -d) && cd "$TMP"
tracker create "Line1
Line2"
tracker list   # row appears split across two output lines, breaking column alignment
```

DESIGN.md Edge Cases / Title does not address embedded newlines. The list format contract ("one issue per line, tabular") is silently violated. Description allows newlines (DESIGN.md says so), but title was not specified to. Classification depends on Solution Owner intent — if newlines in titles are out of scope, validation should reject them; if accepted, list output needs to render them as `\n` or replace with a printable substitute.

**Classification:** Open. Raised to SO for spec clarification. Less severe than Findings 1 and 2 but a contract violation either way.

---

**Finding 4 — `load_issues` does not enforce DESIGN.md's per-storage uniqueness invariant on `id` (Dim 4 — business logic abuse)**

`issue_fields_are_valid` validates each issue's fields in isolation. It does not check whether two issues share the same `id`. DESIGN.md "Field invariants" explicitly states: "`id` is unique across all issues and never reused." A hand-edited `tracker.json` with two issues at `id: 1` is loaded successfully; `tracker status 1 done` mutates only the first match (via `iter().position`) and silently leaves the duplicate untouched.

**Reproduction:**

```sh
TMP=$(mktemp -d) && cd "$TMP"
printf '[{"id":1,"title":"a","status":"open","priority":"high","labels":[],"created_at":"2026-01-01T00:00:00Z","updated_at":"2026-01-01T00:00:00Z"},{"id":1,"title":"b","status":"open","priority":"high","labels":[],"created_at":"2026-01-01T00:00:00Z","updated_at":"2026-01-01T00:00:00Z"}]' > tracker.json
tracker list           # both rows appear with id 1
tracker status 1 done  # only "a" updates; "b" is silently desynchronized
```

This is a load-time invariant gap, parallel in spirit to the post-deserialization domain validation Security Review 1 added. The fix shape is the same: extend `load_issues` (or `issue_fields_are_valid`'s caller) to verify cross-issue invariants before returning `Ok`. Trigger the existing `CORRUPT_DATA_ERROR` path on duplicate IDs.

**Classification:** Open. Cross-reference with [SECURITY-REVIEW.md](SECURITY-REVIEW.md) and [DATA-ENGINEER-REVIEW.md](DATA-ENGINEER-REVIEW.md) — same class of finding as the original post-deserialization gap, just on the cross-row dimension.

---

### Dismissed

**Finding 5 — Timestamps not validated as ISO 8601 (Dim 5)**

`created_at` and `updated_at` are typed `String` and `issue_fields_are_valid` does not parse them as dates. A crafted `tracker.json` can supply `"created_at": "NOT A DATE"` and the tool accepts it. However, the spec's only on-load behavior that depends on timestamp content is sort/display, and the display merely echoes the string (no parsing). No crash, no data loss, no unexpected behavior beyond "garbage in, garbage out."

**Classification:** Dismissed. Real but not exploitable beyond visual garbage. If DESIGN.md ever introduces an ordering by `created_at`, this becomes Open. Flagged here as a watch item.

---

**Finding 6 — Performance with 10 000 issues is fine (Dim 11 — DoS via large input)**

A 1.4 MB `tracker.json` containing 10 000 issues lists in ~50 ms wall-clock on the development machine. No quadratic blowup observed. Memory usage is bounded by the file size. Not a viable DoS vector for a personal tool.

**Classification:** Dismissed. Re-evaluate if the data model grows nested structures.

---

**Finding 7 — No `unsafe` blocks (Dim — Rust supplement, unsafe usage)**

`grep -rn unsafe src/` returns nothing. No FFI, no transmute, no raw pointer manipulation.

**Classification:** Dismissed. No unsafe surface to exploit.

---

### Hallucinated

**Finding 8 — Path traversal via `tracker.json` (regression check)**

Still hardcoded as `Path::new("tracker.json")` in `main.rs:55`. No user-controlled path component. Unchanged.

**Classification:** Hallucinated.

---

**Finding 9 — `serde_json::to_string_pretty(...).unwrap()` panic surface (regression)**

`save_issues` (`src/lib.rs:87`) keeps the documented `unwrap` on `Vec<Issue>` serialization. `Issue` contains only `String`/`u64`/`Vec<String>`/`Option<String>` — no NaN floats, no `Map` with non-string keys, no reference cycles. Cannot fail in practice; the `#[allow(clippy::unwrap_used)]` annotation is justified by the comment immediately above.

**Classification:** Hallucinated.

---

### Accepted Risk

**Finding 10 — Plaintext `tracker.json` in CWD (regression from Security Review 1)**

Unchanged. Risk owner: the user/developer.

**Classification:** Accepted Risk.

---

### Open — supply-chain watch item

**Finding 11 — `Cargo.lock` contains transitive dependencies inconsistent with the upstream crates' real dependency trees (Dim 12)**

Cargo.lock declares `serde_json 1.0.149` depends on `zmij 1.0.21`, and `chrono 0.4.44` depends on `wit-bindgen 0.51.0` / `wasm-bindgen 0.2.118` / `wasip3` / `wasm-encoder` / `wasm-metadata` / `wit-component`. Real-world `serde_json` does not depend on a crate named `zmij`; real-world `chrono 0.4.x` pulls `wasm-bindgen` only when targeting `wasm32-unknown-unknown` and has never pulled `wit-bindgen` as a transitive. This Cargo.lock either (a) reflects a synthetic/test environment and never represents what `cargo install` would resolve in practice, or (b) reflects a real registry that has been substituted with crates differing from the public crates.io trees. Either way, there is no `deny.toml` or `cargo vet` configuration to constrain which sources are accepted, and no CI workflow exists in the project (`.github/workflows/` absent) to gate `cargo audit` on every push.

The Rust supplement's Red Team section explicitly calls this out: "Use `cargo deny check` to detect banned or unreviewed sources. For any dependency added recently or at an unusual version: verify the crate author and publication history on crates.io." That control is absent here.

**Classification:** Open. Cross-reference with [SECURITY-REVIEW.md](SECURITY-REVIEW.md) (control: `deny.toml`) and [PLATFORM-ENGINEER-REVIEW.md](PLATFORM-ENGINEER-REVIEW.md) (CI enforcement). The Red Team flag is: until the lock file's transitive entries can be verified against an authoritative source list, treat this dependency tree as unverified.

---

### Summary

Five Open findings, three Dismissed, two Hallucinated, one Accepted Risk (carried forward), one Open supply-chain watch item. The most severe is Finding 2 — a data-destruction attack achievable in one CLI call against a hand-edited `tracker.json`, which the previous reviews accepted as risk under a too-narrow threat model. ANSI/control-sequence injection (Finding 1) is the second-most-severe and is reachable from organic CLI input, not just hand-edited files. Findings 3 and 4 are spec-contract violations on the load and display paths.

The pattern across Findings 1–4: validation at the trust boundary is per-field structural ("is this string in a known set?") and does not check cross-row invariants, content character classes, or display safety. The post-deserialization domain validation that Security Review 1 added is good but does not extend to either uniqueness across issues or printable-character constraints on stored text.

The "small attack surface" framing carried forward from prior reviews was correct for the prior layers and incorrect for this one. The prior frame measured surface as "number of CLI subcommands × number of file paths" — this review measures it as "what bytes can flow from the input boundary to the output terminal without transformation," which is a much larger surface.

**Coordination:**
- Findings 1, 2, 4 → [SECURITY-REVIEW.md](SECURITY-REVIEW.md): each reflects an absent input-validation or output-encoding control. Security review N+1 is running concurrently — these are independent confirmations.
- Findings 1, 2, 3, 4 → [QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md): no existing test exercises the reproduction recipes above. The crafted-file paths in particular are absent from `tests/layer*.rs`.
- Findings 2, 3 → [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) / DESIGN.md change request: the ID-overflow brick path and newline-in-title behavior need spec decisions before code can encode the right validation.
- Finding 11 → [PLATFORM-ENGINEER-REVIEW.md](PLATFORM-ENGINEER-REVIEW.md): no CI workflow, no `deny.toml`, no `cargo audit` gate.

---

### Update — 2026-05-04 16:00Z: Layer 3 follow-up resolution pass

Two of the five attack paths closed in code; two remain spec-pending (require SO adjudication of the title sanitization contract); one supply-chain watch item is partially mitigated.

- **F1 (ANSI/control-sequence injection in titles) → still Open / spec-pending.** Sanitization scope is a DESIGN.md decision (reject in `validate_title`, sanitize at render time, or accept and document as the user's responsibility — the spec currently treats title as opaque text after shell expansion). UX Review 5 Finding 3 has this Raised to SO. Validator and renderer remain as-is until SO adjudicates.
- **F2 (`u64::MAX` next_id brick attack) → Resolved.** `next_id` switched to `Result<u64, String>` with `checked_add(1)`. Reproduction recipe (plant `id: u64::MAX`, attempt `tracker create`) now produces `Error: Cannot assign new issue ID: maximum ID reached.` with exit 1; no silent wrap, no debug-build panic. The prior Accepted Risk rationale ("18.4 quintillion organic issues") was reframed correctly in this finding — the threat was hand-edited input, not organic exhaustion. Regression locked by `tests/layer1.rs:u64_max_id_in_json_blocks_next_create_with_clean_error` and unit test `id_assignment_at_u64_max_returns_error`.
- **F3 (newlines in titles break list-row contract) → still Open / spec-pending.** Same as F1 — sanitization scope is a spec decision. UX Review 5 Finding 2 has this Raised to SO.
- **F4 (`load_issues` does not enforce ID uniqueness) → Resolved.** `issues_collection_invariants_hold` (HashSet membership walk) added; `load_issues` now calls both per-record and cross-record validators. The `cmd_status` "first match only" silent desync path is no longer reachable from valid stored data. Regression locked by `tests/layer1.rs:duplicate_ids_in_json_causes_error_exit` and unit tests `collection_invariants_{reject_duplicate_ids, accept_unique_ids}`. Same fix lands the cross-domain Security F3 / SE F1 / DE F2.
- **F5 (Cargo.lock supply-chain watch item) → partially mitigated.** `deny.toml` added with `[sources]` allowlist restricted to `https://github.com/rust-lang/crates.io-index`; CI workflow now runs `cargo deny --locked check` after `cargo audit`. Future surprise sources (typosquatting, unknown-git pulls) will fail CI. Whether the original anomaly (the prior reviewer's observation about `serde_json` listing `zmij` as a dep, etc.) reflects a synthetic test environment or a real lockfile divergence is not adjudicated here — `cargo deny check` on the real CI will provide the authoritative answer; if it surfaces a genuine anomaly, Red Team N+1 should re-investigate against that signal.

**Net for Red Team posture:** the two crash/corruption paths (F2, F4) are closed; the two output-encoding paths (F1, F3) remain Open until SO acts on the spec questions; supply-chain dependency posture is improved by `cargo deny check` in CI. No new Red Team findings this round.

---

### Update — 2026-05-05 11:00Z: SO Review 13 spec adjudication

The two spec-pending output-encoding attacks closed by SO Review 13 Finding 1 (single rule: reject `is_control()` characters at both `validate_title` and `issue_fields_are_valid`). All five Round 5 findings now have a disposition.

- **F1 (ANSI/control-sequence injection) → Resolved.** ESC (`0x1B`) is in Unicode category `Cc`; the new rule rejects it at create time AND at load time, closing both the organic-CLI path and the hand-edited-`tracker.json` path. Reproduction recipe `tracker create $'\e[2J\e[H...'` now fails with `Error: Title cannot contain control characters.` and exit 1; `tracker list` against a planted `tracker.json` containing a control-character title returns the corrupt-data error instead of re-emitting the payload. Regression locked by `tests/layer1.rs:create_title_with_ansi_escape_exits_one` and `control_char_title_in_json_causes_error_exit`.
- **F3 (newlines in title break list contract) → Resolved.** Same rule covers `\n`, `\r`, `\r\n`. Regression locked by `tests/layer1.rs:create_title_with_newline_exits_one` and unit test `title_with_newline_is_rejected`.
- **F2, F4 → Resolved (already in prior round).** No change this round.
- **F5 (Cargo.lock supply-chain watch item) → still Open / partially mitigated.** Unaffected by SO Review 13. The `deny.toml` `[sources]` allowlist + `cargo deny check` in CI provide ongoing detection; an authoritative answer awaits the next CI run on a clean checkout.

**Net for Red Team posture (updated):** four of five Round 5 findings closed in code (F1, F2, F3, F4); F5 remains a watch item with a CI tripwire in place. The control-character rule closes a single class of attack at the validation boundary — better than per-renderer sanitization which would have left attacks alive at every output path.

**Coordination:** SO Review 13 cross-reference. No new Red Team findings.


---

---

## Review 6 — 2026-05-05 22:30Z

**Scope:** Layer 4 implementation (`tracker create --label`, `tracker list --label`). Verify Security Review 7 Finding 1 reproduces on the current binary; pressure-test the corollary attack vectors Security flagged for Red Team (load path, filter-rendering path, OSC 8, combined attacks); and apply independent adversarial creativity beyond Security's list (Trojan-Source bidi/zero-width characters, error-message reflection of raw bytes, resource exhaustion, JSON depth bomb, symlink, ID/argument-injection edge cases).

**Session note:** Cold session per primer. Parallel Tier-4 batch with VDD-IAR Alignment scheduled to follow. Security 7 ran in a parallel session and surfaced F1 the same day — this review confirms F1 against the actual binary and adds independent findings that Security 7 did not enumerate.

**Posture:** Adversarial. The Layer 3 / SO Review 13 fix (`char::is_control()` rejection at `validate_title` *and* `issue_fields_are_valid`) was scoped *to the title field only*. Layer 4 introduces labels — a second free-form text field that flows to the same `list` rendering surface. The sycophancy hazard the primer names is "describing a gap and concluding it is acceptable without verification": every claim of "the existing control covers this" gets a reproducer.

**Build:** `cargo build --release` against `issue-tracker-cli-labels` @ HEAD `f14c296`. Binary at `target/release/tracker`. Reproducers run from `mktemp -d` directories; output captured with `od -c` so terminal escapes are visible regardless of terminal interpretation.

---

### Threat Model (preamble — not a finding)

Aligned with Security 7's preamble. Crown jewel: the integrity of the tabular `list` output and (Layer 6) `show` rendering surface — terminal-escape injection here propagates to any tool that displays issue data without inspecting the underlying bytes (`grep`, `awk`, `cat` without `-v`, code review of `tracker.json` in an editor that interprets ANSI). Plausible attackers: third-party who hands the user a `tracker.json`; the user pasting clipboard content; a process that writes to the user's CWD `tracker.json`. No remote attackers. No multi-user privilege boundary.

What's *new* in the threat model for this review: **error messages reflect raw user input to stderr**. That surface was not in scope for prior reviews because prior interpolated values were either pre-validated (status, priority small enums echoed back via `format!("Invalid status '{}'", raw)`) or numeric (id strings). Security 7 evaluated label rendering on the *data path* but did not evaluate the *error-reflection path* — that's where Finding 2 below lives.

---

### Open

**Finding 1 — Security 7 F1 confirmed on the current binary; load-path is also vulnerable; OSC 8 hyperlink injection is also vulnerable (Dim 6 — Injection via output-encoding gap; Dim 7 — Client-side terminal injection; Rust supplement Red Team — terminal-escape injection)**

Confirms Security Review 7 Finding 1 against the release binary. The defense gap Security identified is real and exploitable on three distinct paths, two of which Security explicitly asked Red Team to pressure-test:

**(a) Create-time path — newline label fabricates a fake `list` row.**

```
$ tracker create "Real" --label $'bug\nFAKE'
Created issue #1: Real
$ tracker list 2>&1 | od -c | head -10
0000060                T   i   t   l   e  \n   1                       o
0000100    p   e   n                                       m   e   d   i
0000120    u   m                   b   u   g  \n   F   A   K   E        
0000140                                                    R   e   a   l
0000160   \n
```

The `\n` between `bug` and `FAKE` emerges as a literal newline (`od -c` confirms byte `\n` not `\\n`). `tracker list | wc -l` reports 4 lines (header + payload + injected + real-row continuation) instead of the expected 2. Any line-oriented consumer (`grep`, `awk`, `head`) treats the injected substring as a real record.

**(b) Load-time path — hand-edited `tracker.json` reproduces the same exploit and bypasses any create-time fix that doesn't extend to `issue_fields_are_valid`.**

```
$ cat > tracker.json <<'JSON'
[{"id":1,"title":"Real","status":"open","priority":"medium","labels":["bug\nFAKE"],"created_at":"2026-01-01T00:00:00Z","updated_at":"2026-01-01T00:00:00Z"}]
JSON
$ tracker list 2>&1 | od -c | head -10
0000120    u   m                   b   u   g  \n   F   A   K   E        
0000140                                                    R   e   a   l
```

`load_issues` returns success (exit 0). `issue_fields_are_valid` (`src/lib.rs:131`) only checks `!l.trim().is_empty()` for label hygiene, parallel to the same gap Security 7 named for `parse_label`. Any SE-side fix that closes only `parse_label` and leaves `issue_fields_are_valid` unchanged still fails this reproducer. The remediation Security 7 proposed (extend BOTH `parse_label` and `issue_fields_are_valid`) is necessary; either alone is insufficient.

**(c) OSC 8 hyperlink injection in labels — both create and load paths.**

```
$ tracker create "Real" --label $'\x1b]8;;https://evil/\x1b\\X\x1b]8;;\x1b\\'
$ tracker list 2>&1 | od -c | head -10
0000120    u   m                 033   ]   8   ;   ;   a  \a   X 033   ]
0000140    8   ;   ;  \a                                   R   e   a   l
```

Raw `\x1b]8;;...\x1b\\` (or BEL-terminated `\x1b]8;;...\x07X\x1b]8;;\x07` for shorter encodings that fit the 20-char `Labels` column) emerges intact. In any terminal supporting OSC 8 hyperlinks (iTerm2, modern xterm, GNOME Terminal, recent Windows Terminal), the rendered cell becomes a clickable hyperlink to the attacker URL. The DESIGN.md Layer 7 rationale for the title control-char rule (`src/lib.rs:60-66`) names "ESC, C1 controls" — OSC 8 uses ESC, so the same defense closes this attack class once extended to labels. ESC is in Unicode category `Cc` (`char::is_control() == true`), so the recommended Security 7 fix (`trimmed.chars().any(char::is_control)`) covers OSC 8 leaders for free.

**Combined-row attack:** an honest-looking label paired with a payload-bearing label in the same `--label --label` invocation hits the same row, producing rows that pass casual visual inspection but fabricate an entire fake issue:

```
$ tracker create "Login bug" --label "auth" --label $'bug\n2     done         high      backdoor              Backdoor merged'
$ tracker list 2>&1 | od -c | head
   ... auth, bug\n2     done         high      backdoor ... Backdoor merged ...
```

The `Labels` column truncation at 20 chars cuts mid-injection but only after the newline — the fabricated row is printed beyond the truncation boundary because `truncate_with_ellipsis` operates on the joined-and-rendered string only for the first cell, while the rest of the line is `println!`'d as-is.

**Recommended remediation (Raised to SE; Raised to QE; Raised to SO):** identical to Security 7 F1's recommendation. Extend `parse_label` AND `issue_fields_are_valid` to reject `char::is_control()`. Add unit + integration tests mirroring the title control-char tests. DESIGN.md amendment per Security 7 F1.

**Severity:** Medium-High. Confirms Security 7 F1 with no scope reduction; adds the OSC 8 vector that Security flagged for Red Team to pressure-test (and confirms it's covered by the same `is_control()` rule because ESC is `Cc`); confirms the load-path corollary Security flagged. The fix Security proposed is correct and complete *for the labels surface specifically*. Findings 2 and 3 below identify two surfaces that Security 7 did not enumerate and that the labels fix does not close.

**Classification:** **Open. Raised to SE / Raised to QE / Raised to SO.** Cannot be Hallucinated — reproducers above are concrete and fail to be defended. Cannot be Resolved — fix is not yet applied as of HEAD `f14c296`.

---

**Finding 2 — Error messages reflect raw user input including ESC bytes; terminal-escape injection from any invalid `--priority`, `--status`, or invalid `<id>` argument (Dim 6 — Injection via output-encoding gap; Dim 7 — Client-side terminal injection; Dim 8 — Information leakage via verbose error messages)**

Independent of Security 7 F1. The labels-rendering attack covers the *data* path; this finding covers the *error-reflection* path. Three error-formatter sites interpolate raw user-supplied bytes into stderr:

- `parse_priority` (`src/lib.rs:319`): `Err(format!("Invalid priority '{}'. Expected: low, medium, or high.", raw))`
- `parse_status` (`src/lib.rs:248-251`): `Err(format!("Invalid status '{}'. Expected: open, in-progress, or done.", raw))`
- `parse_id` (`src/lib.rs:260-265`): `Err(format!("'{}' is not a valid issue ID. Expected a positive integer.", raw))`

Reproducer (release binary):

```
$ tracker list --priority $'\x1b[31mPWN\x1b[0m' 2>&1 | od -c | head
0000020    r   i   o   r   i   t   y       ' 033   [   3   1   m   P   W
0000040    N 033   [   0   m   '   .       E   x   p   e   c   t   e   d
```

Raw ESC sequence is interpolated into the error message and emitted on stderr. In any ANSI-capable terminal, this renders the word "PWN" in red — from a single command-line argument typo. Same exploit on `--status`:

```
$ tracker list --status $'foo\nbar\x1b[31m' 2>&1 | od -c | head
0000020    t   a   t   u   s       '   f   o   o  \n   b   a   r 033   [
0000040    3   1   m   '   .       E   x   p   e   c   t   e   d
```

The `\n` between `foo` and `bar` emerges raw, breaking the one-line-per-error contract. Same exploit on `<id>`:

```
$ tracker status $'abc\x1b[31mEVIL\x1b[0m' done 2>&1 | od -c | head
0000000    E   r   r   o   r   :       '   a   b   c 033   [   3   1   m
0000020    E   V   I   L 033   [   0   m   '
```

Why this matters beyond labels:

1. **Reach is wider than labels.** Labels require the user to invoke `--label`. The error-reflection vector fires on *any* invalid `--priority` / `--status` / `<id>` argument across all subcommands. A single typo with terminal control bytes pasted from clipboard executes the attack.
2. **DESIGN.md "stderr contract" is silently violated.** `Error messages begin with Error: and are followed by a human-readable description; no stack traces or internal detail are exposed to the user.` Embedded raw control characters break "human-readable" the same way they break the title rendering surface.
3. **The Security 7 F1 remediation does NOT close this surface.** Even after `parse_label` + `issue_fields_are_valid` get the `is_control()` check, the three `format!` sites above continue to interpolate raw bytes. This is the same regression class Security 7 named: a control was scoped by-field rather than by-property.
4. **Same control-char rule covers it.** The fix shape is identical to F1: validate-then-format, or sanitize at the format site (replace `chars().filter(|c| c.is_control())` with `\\u{XX}` escapes before interpolation). Sanitize-at-formatter is the safer architectural choice because it closes *every* error-reflection surface in one place rather than per-validator.

**Recommended remediation (Raised to SE / Raised to QE / Raised to SO):**

- **SO:** Amend DESIGN.md "stderr contract" / "Error states" to specify that error messages echoing user input must escape control characters (Unicode category `Cc`). Suggested wording: `Error messages that interpolate user-supplied values must render control characters as \uXXXX escapes; the error stream is not a transparent pipe for arbitrary terminal sequences.`
- **SE:** Add a `display_safe(&str) -> String` helper that maps each `is_control()` char to `\uXXXX` (or a similar printable substitute) and use it at the three `format!` sites. The helper localizes the rule and is reusable for future error formatters.
- **QE:** Add unit tests `error_message_escapes_control_chars_in_*` for priority/status/id. Add an integration test asserting `tracker list --priority $'\x1b[31m' 2>&1 | grep -c $'\x1b'` returns 0.

**Severity:** Medium. Same vulnerability class as Security 7 F1 with a wider entry-point surface but a milder primary impact (stderr typically gets scrolled past faster than stdout). Combined with F1, the project has *two* control-char-output regressions both deriving from the same scoping decision. F1 is the louder of the two; F2 is the broader.

**Classification:** **Open. Raised to SE / Raised to QE / Raised to SO.** Independent of Security 7 — Security 7 evaluated the labels rendering path and did not evaluate error-message reflection. Cannot be Hallucinated — reproducers are concrete byte-for-byte. Cannot be Deferred — Red Team findings are not deferred per CLOSURE-PROTOCOL.md.

---

**Finding 3 — Trojan-Source bidi override and zero-width characters bypass the `char::is_control()` defense in titles AND labels (Dim 6 — Injection via display-class gap; Dim 7 — Client-side terminal injection; Rust supplement Red Team — Trojan-Source / CVE-2021-42574 class)**

The SO Review 13 fix uses `char::is_control()`, which returns `true` only for Unicode general category `Cc` (C0 + C1 + DEL). Two adjacent Unicode classes that affect display rendering are NOT category `Cc` and pass the check unmolested:

- **Category `Cf` (Format):** RIGHT-TO-LEFT OVERRIDE U+202E, LEFT-TO-RIGHT OVERRIDE U+202D, RIGHT-TO-LEFT EMBEDDING U+202B, etc. These reverse the visual rendering direction of subsequent text without changing the underlying byte order — the canonical Trojan-Source attack (CVE-2021-42574, "Some Things You Just Can't Trust").
- **Zero-width characters (Cf and others):** ZERO WIDTH SPACE U+200B, ZERO WIDTH JOINER U+200D, ZERO WIDTH NON-JOINER U+200C. These render as nothing but participate in string equality — `"auth"` vs `"au​th"` look identical in the terminal but compare unequal, confounding label exact-match.

Reproducer — bidi override in title (release binary, current HEAD):

```
$ tracker create $'attack‮suoicilam'
Created issue #1: attack‮suoicilam
$ tracker list 2>&1 | od -c | head -10
0000140    c   k 342 200 256   s   u   o   i   c   i   l   a   m  \n
```

The bytes `342 200 256` are UTF-8 for U+202E. The title passes `validate_title` (the `is_control()` check skips category `Cf`), is stored, and on `tracker list` the terminal renders the literal bytes `attack` + RLO + `suoicilam`, which appears as `attackmalicious` to the user. Same payload via hand-edited JSON (`"‮"`) loads cleanly. Same payload in a label produces the same render in the `Labels` cell.

Reproducer — zero-width characters bypass label exact-match dedup/filter:

```
$ tracker create "x" --label "auth" --label $'auth​'
$ tracker list --label "auth"  # → only the first issue's label matches
```

The two labels render visually identically but `dedupe_labels` (`src/lib.rs:351-360`) treats them as distinct and `label_matches` (`src/lib.rs:367-369`) matches only the first. A user filtering by `--label auth` may believe they've enumerated all auth-tagged issues when in fact a zero-width-poisoned label silently hides one.

Why this is *not* covered by the Security 7 F1 fix: extending `is_control()` to labels closes the `Cc` surface but leaves `Cf` and zero-width characters open — the SO Review 13 wording was specifically `is_control()`, which Rust documents as "category Cc." Closing this finding requires a *broader* validation rule than the title fix used, OR an explicit decision to accept the `Cf` / zero-width surface as out-of-scope.

**Recommended remediation (Raised to SO for spec adjudication; conditional Raised to SE):** SO must adjudicate whether the project's display-safety contract covers `Cf` / zero-width / general bidi/format characters. Three reasonable spec stances:

1. **Tighten:** prohibit any character where `c.is_control() || matches!(c.general_category(), Cf | Co | ...)`. Closes the surface; requires `unicode-general-category` crate or hand-coded ranges.
2. **Document the surface:** add a DESIGN.md "Edge Cases / Title" bullet explicitly noting that bidi/format/zero-width characters are accepted as valid printable Unicode and may produce visually-misleading output in `list` / `show`. Treats this as the user's responsibility (consistent with the existing "shell-special characters → shell responsibility" framing).
3. **Sanitize at render:** strip or substitute `Cf` chars at output time in `cmd_list` / `cmd_show` while accepting them at storage. Asymmetric but closes the visual-deception surface without restricting input.

The minimum action is decision (2) — explicitly document what the spec considers safe vs. what falls outside the threat model. Silent acceptance is the failure mode.

**Severity:** Medium. The bidi attack lets any title or label produce visually-misleading rows that pass `cat -v` inspection (RLO renders as `M-^^M-^@M-^^^^^[` — visible but obscure). The zero-width attack defeats the exact-match label filter contract (DESIGN.md Edge Cases / Labels: "exact match, case-sensitive"). Lower than F1/F2 because the user attacks themselves in the organic case, but a hand-edited `tracker.json` from a third party reaches the same outcome.

**Classification:** **Open. Raised to SO (primary) / Raised to SE (conditional on spec stance) / Raised to QE (regression test conditional on spec stance).** Cannot be Hallucinated — reproducers above produce raw UTF-8 bytes for U+202E / U+200B that pass `is_control()` and reach the rendering surface. Cannot be Deferred — Red Team findings are not deferred. The spec adjudication may legitimately accept the surface (option 2 above) — that is the SO's call, not Red Team's, and acceptance must be explicit per CLOSURE-PROTOCOL.md.

---

### Dismissed

**Finding 4 — `--label` filter value is not validated and could carry control bytes (Dim 6 — Injection)**

Concern: `cmd_list`'s `label_filter` parameter (`src/lib.rs:403`) accepts arbitrary strings without `parse_label` validation. A filter value `$'\x1b[31mPWN\x1b[0m'` is passed through unmodified.

Verified reproducer — empty match path:

```
$ tracker list --label $'\x1b[31mPWN\x1b[0m' 2>&1 | od -c | head
0000000    N   o       i   s   s   u   e   s       m   a   t   c   h    
0000020    t   h   e       g   i   v   e   n       f   i   l   t   e   r
0000040    s   .  \n
```

The filter value is consumed only by `label_matches` (`src/lib.rs:367-369`) which performs `String` equality. It's never reflected in stdout, never reflected in the `No issues match the given filters.` stderr message, and never reaches a terminal escape sink. Confirms Security 7's Hallucinated Finding 10 is correctly classified — the filter side has no rendering sink even though Security explicitly asked Red Team to pressure-test it.

**Classification:** Dismissed. Security 7 Hallucinated Finding 10 confirmed correct. No injection surface on the filter side regardless of whether the filter value carries control bytes.

---

**Finding 5 — Resource exhaustion via long titles or many `--label` flags (Dim 11 — DoS via expensive operations)**

Empirical timing on release binary, fresh CWD:

- `tracker create "$(python3 -c 'print("A"*200000)')"`: completes in <50ms; 200KB title written to `tracker.json`. List output truncates to 50 chars per spec, no quadratic blowup.
- `tracker create "x"` with 5000 `--label` flags: completes in 33ms wall-clock; `tracker.json` weighs 94KB; subsequent `tracker list` truncates the `Labels` cell at 20 chars and runs in <50ms.
- JSON depth bomb (`[[[[[[[[...]]]]]]`, 10000 levels): `tracker list` returns `Could not read tracker data. The file may be corrupt.` with exit 1 in <100ms (serde_json's recursion limit catches this — confirmed by `cargo deny` and `serde_json` defaults).

Single-user local CLI, no remote attacker, no shared state. The user cannot DoS themselves in any meaningful sense; even a worst-case title would just bloat their own `tracker.json`. Carries forward Review 5 Finding 6 reasoning.

**Classification:** Dismissed. Performance bounded; no remote vector; spec-consistent.

---

**Finding 6 — Symlinked `tracker.json` redirects writes outside CWD (Dim — sandbox escape)**

Reproducer:

```
$ mkdir realdir && ln -s realdir/tracker.json tracker.json
$ tracker create "via symlink"
Created issue #1: via symlink
$ ls realdir/
tracker.json
```

The binary follows the symlink and writes through to `realdir/tracker.json`. DESIGN.md "File location: `tracker.json` in the current working directory at the time the command runs" treats the path as opaque; symlink resolution is the OS's job and consistent with every other Unix tool. There's no privilege boundary being crossed (the user creates the symlink themselves; they can already write to `realdir/`). Not a sandbox escape; a documented behavior.

**Classification:** Dismissed. Spec-consistent; no security boundary crossed.

---

**Finding 7 — `tracker.json` argument injection via title that looks like a flag (`tracker create --label`) (Dim 6 — Injection)**

Reproducer:

```
$ tracker create "--label"
Error: a value is required for '--label <LABEL>' but none was supplied
$ tracker create -- "--label"
Created issue #1: --label
```

Clap correctly identifies `--label` as a flag without a value and produces a usage error (exit 1). Passing `--` first allows the literal title `--label` to be stored — standard POSIX argument-separator behavior. No injection surface; clap's parsing boundary holds.

**Classification:** Dismissed. Standard clap behavior; spec-consistent.

---

### Hallucinated

**Finding 8 — `tracker.json` could be redirected by env var or relative-path manipulation**

Concern: `Path::new("tracker.json")` in `main.rs:71` could be redirected by setting `TRACKER_JSON_PATH` or chrooting before the binary runs.

Reading `src/main.rs:71` confirms the path is a hardcoded literal; no env var is read. The "redirect by changing CWD" path is documented in DESIGN.md "File location: `tracker.json` in the current working directory at the time the command runs" — that's not an attack, that's the spec.

**Classification:** Hallucinated. No env-var path-redirection surface; CWD-relative behavior is spec-defined.

---

**Finding 9 — `parse_id` could panic on `u128`-sized input**

Concern: `raw.parse::<u64>()` could panic on extreme input.

`parse::<u64>()` returns `Result<u64, ParseIntError>`; `parse_id` (`src/lib.rs:259-266`) handles it with `.ok().filter(|&n| n > 0).ok_or_else(...)`. Verified: `tracker status 999999999999999999999999999 done` produces a clean error (exit 1, no panic). Carries forward Review 4 Finding 2.

**Classification:** Hallucinated. `u64::parse` overflow handled.

---

### Accepted Risk

**Finding 10 — Plaintext `tracker.json` (regression carried forward)**

Unchanged from prior reviews. **Risk owner:** the user/developer (named in DESIGN.md "Constraints").

**Classification:** Accepted Risk.

---

### Summary

Round **6** logged. Cold-session sweep produced **three Open findings (Raised to SE / Raised to QE / Raised to SO)**, **four Dismissed**, **two Hallucinated**, **one Accepted Risk** (carried forward).

**Confirmation of Security 7 F1:** Reproduced on the release binary at HEAD `f14c296`. The fix Security proposed is correct and complete *for the labels surface specifically*. The corollary attack vectors Security flagged for Red Team to pressure-test all reproduce: load-path is independently vulnerable (the fix must touch BOTH `parse_label` AND `issue_fields_are_valid`); OSC 8 hyperlink injection is covered by the same `is_control()` rule because ESC is `Cc`; combined-row attacks produce convincing fake rows.

**Beyond Security 7:** Two additional control-char-output regressions exist that the Security 7 F1 remediation does not close — both derive from the same scoping decision (defense scoped by-field rather than by-property):

- **F2 (error-message reflection)** — three `format!` sites in `parse_priority`, `parse_status`, `parse_id` interpolate raw user input including ESC sequences into stderr error messages. Fires on any invalid argument across all subcommands; wider entry-point surface than labels.
- **F3 (Trojan-Source bidi/zero-width)** — the SO Review 13 fix used `char::is_control()` (category `Cc` only). Categories `Cf` (RLO/LRO/embeds) and zero-width characters bypass the check at both create and load time, in both titles AND labels. Bypasses the existing title defense too — the title control-char-defense regression test passes because the test uses `\n` and `\x1b` (Cc); RLO U+202E was never tested.

**Pattern:** every layer that adds a free-form text field reopens the terminal-escape attack surface unless the defense is generalized. Security 7 named this exact pattern; this review confirms it AND identifies a parallel pattern: every formatter that interpolates user input reopens the same surface unless the formatter sanitizes control bytes. The strategic remediation is a `display_safe(&str) -> String` helper applied at every output sink, not per-validator rules at every input boundary.

**Coordination:**

- [SECURITY-REVIEW.md](SECURITY-REVIEW.md) — F1 confirms Security 7 F1 with no scope reduction. F2 and F3 should appear as new Security findings (absent controls that Security's review dimensions cover); flag for Security 8 if a follow-up runs. F2 is most clearly Security's surface (DESIGN.md "stderr contract" violation).
- [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) — F1 requires the DESIGN.md amendment Security 7 already proposed. F2 requires a new DESIGN.md "stderr contract" amendment. F3 requires SO adjudication of the spec's display-safety stance on `Cf` / zero-width characters (three options enumerated in F3).
- [SOFTWARE-ENGINEER-REVIEW.md](SOFTWARE-ENGINEER-REVIEW.md) — F1 fix per Security 7. F2 and F3 fixes pending SO adjudication; F2 specifically suggests a `display_safe` helper at format sites rather than per-input validation, because the helper closes every reflection surface in one place.
- [QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md) — Regression tests for all three findings. The existing title control-char tests should be extended to RLO/zero-width per F3, regardless of which spec stance SO chooses.
- [VDD-IAR-ALIGNMENT-REVIEW.md](VDD-IAR-ALIGNMENT-REVIEW.md) — see merge-gate concerns below.

**Merge-gate concern for VDD-IAR Alignment:** F1, F2, and F3 are all Open and Raised to SO/SE/QE. Per IAR domain authority, Red Team does not modify code. The Layer 4 gate cannot legitimately close with three Open Red Team findings unless the spec accepts the F3 surface (option 2) AND F1 + F2 land code fixes. Specifically: closing the gate with only the Security 7 F1 fix in place leaves F2 (error-reflection) and F3 (Trojan-Source) as known-exploitable surfaces with the same vulnerability class as the title defense the project already invested in. That asymmetry is the merge-gate signal.

**Files modified:** Only this review log appended. No source, tests, or DESIGN.md changes per IAR domain authority boundaries (CLOSURE-PROTOCOL.md).

---

## Review 7 — 2026-05-06 02:40Z

**Round:** Red Team Review 7 (Round-2 verification for Layer 4)
**Scope:** Re-run the three Round-1 attack reproducers against the release binary at commit `67ef920`. Verify F1 (label control-char), F2 (error reflection), F3 (Trojan Source) per the SO Review 17 adjudications.
**Session context:** Warm-verification session. Reproducers from Review 6 re-executed verbatim with `od -c` to capture raw bytes.

### Resolved

#### Finding 1 (Round-1) — Label control-character injection (all three paths)

Re-running the Review 6 reproducers:

```
$ tracker create "Real" --label $'bug\nFAKE'
Error: Label cannot contain control characters.
$ echo $?
1
```

Create-time path: closed.

```
$ cat > tracker.json <<'JSON'
[{"id":1,"title":"Real","status":"open","priority":"medium","labels":["bug\nFAKE"],"created_at":"2026-01-01T00:00:00Z","updated_at":"2026-01-01T00:00:00Z"}]
JSON
$ tracker list
Error: Could not read tracker data. The file may be corrupt. Delete tracker.json to start fresh.
$ echo $?
1
```

Load-time path: closed (the SE fix correctly extends the rule to `issue_fields_are_valid` per the new `label_is_valid` helper).

```
$ tracker create "Real" --label $'\x1b]8;;https://evil/\x1b\\X\x1b]8;;\x1b\\'
Error: Label cannot contain control characters.
```

OSC 8 hyperlink leader: closed (ESC is `Cc`; covered by the same rule). **Resolved.**

#### Finding 2 (Round-1) — Error-message escape interpolation

Re-running the reproducers:

```
$ tracker list --priority $'\x1b[31mPWN\x1b[0m' 2>&1 | od -c | head -3
0000000    E   r   r   o   r   :       I   n   v   a   l   i   d       p
0000020    r   i   o   r   i   t   y       '   \   u   {   1   B   }   [
0000040    3   1   m   P   W   N   \   u   {   1   B   }   [   0   m   '
```

The previously-vulnerable raw `033` (ESC, byte 0x1B) is now rendered as the literal six-character sequence `\u{1B}`. No bytes in the `Cc` range emerge in stderr. Same observed for `parse_status` (newline → `\u{A}`) and `parse_id` (ESC escaped). **Resolved.**

#### Finding 3 (Round-1) — Trojan-Source bidi / zero-width

SO Review 17 chose Option 2: document the surface as out-of-threat-model in DESIGN.md "Edge Cases / Labels". Per CLOSURE-PROTOCOL.md Section 2, Red Team findings cannot be Deferred but may be Accepted Risk with a named risk owner.

The director (the human user of this branch) is the named risk owner; the threat model basis is DESIGN.md "Constraints" (Single user. No network. No accounts.). Re-evaluation trigger: any future use case that widens the threat model (multi-user / network-distributed / shared `tracker.json`) re-opens this finding. **Reclassified as Accepted Risk.**

### Dismissed

#### Finding (new) — `display_safe` formatting could be exploited by very long input

Tested: `tracker list --priority $(printf '\x1b%.0s' {1..100000})` runs cleanly in <50ms and produces bounded stderr output (~700KB). No buffer stall, no infinite loop, no panic. `display_safe` is bounded by input length × 7 (max expansion per Cc char). **Dismissed.** Cross-reference Security Review 8.

#### Finding (regression) — Symlink, env-var path redirection, JSON depth bomb, `parse::<u64>` overflow

Re-checked the previously-Dismissed and previously-Hallucinated findings against the new binary; all behave identically to Review 6. **Dismissed (regression intact).**

### Accepted Risk

#### Finding 3 (this round) — Trojan-Source / `Cf` / zero-width

Per Round-2 adjudication above. Risk owner: director. Re-evaluation trigger named in DESIGN.md.

#### Finding 10 (carried) — Plaintext `tracker.json`

Unchanged.

### Summary

Round-2 verification: F1 closed at create-time, load-time, and OSC 8 paths; F2 closed at all three error-formatter sites; F3 reclassified as Accepted Risk per the SO-adjudicated spec stance. Three Round-1 Open findings → 0 Round-2 Open findings. No new attack surface introduced by the Round-2 source changes.

**Adversarial honest assessment:** I tried to hallucinate new findings from the new code (the `display_safe` helper, the broader `parse_label`, the filter-side `parse_label` call). None of the candidate hallucinated findings stood up to a reproducer attempt. Either the Round-2 fix is genuinely complete for the Layer 4 surface, or my adversarial creativity is exhausted within this warm-session. A round-2 cold-batch by a fresh reviewer would be the more confident verification — flagged for VDD-IAR Alignment.

**Coordination:** Cross-references Security Review 8 (independent confirmation of F1 closure); SE Review 12 (the source-level fix); SO Review 17 (the spec amendment for F3 Accepted Risk).

**Files modified:** Only this log appended.

---

## Review 8 — 2026-05-11 01:15Z

**Round:** Red Team Review 8 (cold-batch, Layer 6 — description + show + delete)
**Scope:** New attack surface introduced by commits `4fb5e67` (Red Gate) and `c91676a` (implementation): `--description` input, `format_show_block` rendering, `cmd_show`, `cmd_delete`. Probe whether the Layer 4 control-character defense lineage (RT R6 F1 labels + RT R6 F2 error-message reflection) was extended to description, and verify the existing defenses still hold under the new code paths.
**Session context:** Cold session. Built `cargo build --release --locked` against HEAD `c91676a`. Reproducers run from `/tmp/rt8` (fresh CWD per attack). All payloads abstracted as `<ESC>`, `<NUL>`, `<RLO>`, etc. in this log per confidentiality-aware citation.

### Open

#### Finding 1 — `validate_description` accepts control characters; `tracker show` renders them raw to stdout (Dim 6 — Injection via display-class gap; Dim 7 — Client-side terminal injection)

`validate_description` (`src/lib.rs:335-340`) only checks `raw.trim().is_empty()`. There is NO `is_control()` filter. Description is then written verbatim into storage and, on `tracker show <id>`, interpolated raw into `format_show_block` (`src/lib.rs:369-387`) which prints to stdout via `print!`. `issue_fields_are_valid` (`src/lib.rs:125-139`) also does NOT validate description content — only checks `!d.trim().is_empty()` — so the load-path corollary fires too.

Reproducer A — create-time path (release binary, HEAD `c91676a`, fresh CWD):

```
$ tracker create "Real title" --description <ESC>[31mPWN<ESC>[0m
Created issue #1: Real title
$ tracker show 1 | od -c | sed -n '7,8p'
0000140    i   p   t   i   o   n   :     033   [   3   1   m   P   W   N
0000160  033   [   0   m  \n   C   r   e   a   t   e   d   :
```

Raw `033` (ESC, byte 0x1B) reaches stdout. On a TTY the four-byte sequence `<ESC>[31m` switches the terminal to red text; `PWN` renders red until `<ESC>[0m` resets. Same defect class as RT R6 F1 for labels, on a different field.

Reproducer B — load-path path (hand-edited `tracker.json`):

```
$ python3 -c 'import json; open("tracker.json","w").write(json.dumps([{"id":1,"title":"Real","description":"[31mPWN[0m","status":"open","priority":"medium","labels":[],"created_at":"2026-01-01T00:00:00Z","updated_at":"2026-01-01T00:00:00Z"}]))'
$ tracker show 1 | od -c | sed -n '7,8p'
0000140    i   p   t   i   o   n   :     033   [   3   1   m   P   W   N
0000160  033   [   0   m  \n   C   r   e   a   t   e   d   :
```

The hand-edited record passes `issue_fields_are_valid` and the same bytes reach stdout. Confirms both halves of the Layer 4 F1 lineage are open on description: input boundary AND load boundary.

Reproducer C — OSC 8 hyperlink leader (terminal-rendered clickable link pointing wherever the attacker chose):

```
$ tracker create "Real" --description $'<ESC>]8;;https://evil/<ESC>\\X<ESC>]8;;<ESC>\\'
Created issue #1: Real
$ tracker show 1 | od -c | sed -n '7,9p'
0000140    i   p   t   i   o   n   :     033   ]   8   ;   ;   h   t   t
0000160    p   s   :   /   /   e   v   i   l   / 033   \   X 033   ]   8
0000200    ;   ; 033   \  \n   C   r   e   a   t   e   d   :
```

The OSC 8 sequence is preserved byte-for-byte. Same byte-class as RT R6 F1's OSC 8 reproducer for labels.

Reproducer D — bare CR (no LF) causes the description line to overwrite preceding terminal content:

```
$ tracker create "Real" --description $'before<CR>OVERWRITE'
$ tracker show 1 | od -c | sed -n '7,8p'
0000140    i   p   t   i   o   n   :       b   e   f   o   r   e  \r   O
0000160    V   E   R   W   R   I   T   E  \n   C   r   e   a   t   e   d
```

Note: `format_show_block` (`src/lib.rs:365`) normalizes `\r\n` → `\n` for the continuation-indent logic, but does NOT touch bare `\r`. A description with `before<CR>OVERWRITE` renders on a TTY as `OVERWRITE` (CR sends the cursor to column 0, `OVERWRITE` overwrites `before `).

Reproducer E — combined clear-screen attack via multi-line description:

```
$ tracker create "title" --description $'first\nsecond<ESC>[2J<ESC>[H'
$ tracker show 1
```

The `<ESC>[2J<ESC>[H` sequence clears the screen and homes the cursor mid-render. The 13-space continuation indent on the second line legitimately survives the `format_show_block` normalization, so the attacker can place the escape on any line of a multi-line description.

**Severity:** High. Same impact class as RT R6 F1 (labels) but on a wider field (description has no length limit per DESIGN.md "Description is not validated for length"). Combined with reproducers C (OSC 8) and E (clear-screen + cursor-home), an attacker can paint arbitrary content on the user's terminal when the user later runs `tracker show <id>`. The user is the attacker in the local-CLI threat model, but the attack vector reaches them through pasted clipboard content, a shared `tracker.json` (e.g. accidentally committed to a repo and viewed by a teammate running `tracker show`), or a hand-crafted record planted by another tool.

**Why this is a regression of the Layer 4 F1 fix specifically:** SO Review 17 adjudicated F1 by adding control-character rejection to both `parse_label` and `issue_fields_are_valid` (via `label_is_valid`). The Layer 6 spec amendment added description but DESIGN.md "Edge Cases / Description" (lines 339-345) is SILENT on control characters — it explicitly *allows* `\n` ("Description may contain newlines"). The implementation faithfully mirrors the silent spec: `validate_description` does only the empty-after-trim check, `issue_fields_are_valid` does only the empty-after-trim check for the description. This is the same scoping pattern Security 7 named: defense scoped by-field rather than by-property. The spec gap is real (DESIGN.md "stderr contract" / "Edge Cases / Title" both invoke `Cc` rejection for terminal-safety reasons that apply identically to description rendered through `tracker show`).

**A spec-aware nuance:** the description spec deliberately permits `\n` (multi-line rendering is a feature). So the defense cannot be a blanket `is_control()` rejection like title/label. The minimally-correct rule is: reject `Cc` *except* `\n` (and possibly `\t`). The `format_show_block` `\r\n` → `\n` normalization at line 365 suggests the implementor anticipated only `\n` and `\r\n` line endings, not arbitrary control bytes — but no validator enforces that anticipation.

**Recommended remediation (Raised to SO / Raised to SE / Raised to QE):**

- **SO:** Amend DESIGN.md "Edge Cases / Description" to specify the description's control-character contract. Three reasonable spec shapes: (a) reject all `Cc` except `\n` (tightest; consistent with title/label rationale); (b) reject all `Cc` except `\n` and `\t` (permits tabular content); (c) escape control characters at the `show` render site via `display_safe`-equivalent before printing (asymmetric: accept at storage, sanitize at output — mirrors the F2 fix shape). Option (c) closes both create-time and load-time paths with one change. Option (a) is the simplest and consistent with the title/label posture; the cost is rejecting tab-indented descriptions, which DESIGN.md does not currently endorse anyway.
- **SE:** Either (a)/(b): extend `validate_description` and `issue_fields_are_valid` with a `description_is_valid`-style helper analogous to `label_is_valid`; or (c): apply `display_safe`-but-keep-`\n` at the `format_show_block` description branch. Either fix should ride alongside a regression test exercising the load-path corollary (the trap RT R6 F1 originally fell into is closing only the input boundary).
- **QE:** Add regression tests `description_with_escape_sequence_is_rejected` (create path), `description_with_nul_or_del_is_rejected`, `issue_field_validation_rejects_control_char_in_description` (load path), `description_with_bare_cr_is_rejected_or_sanitized`, `description_with_osc8_hyperlink_is_rejected_or_sanitized`. Conditional on the SO stance, `description_with_newline_is_accepted` should remain green.

**Classification:** **Open. Raised to SO (primary, spec adjudication) / Raised to SE (conditional on spec stance) / Raised to QE (regression tests).** Cannot be Hallucinated — five reproducers above (A–E) all produce raw bytes on stdout. Cannot be Deferred — Red Team findings are not deferred per CLOSURE-PROTOCOL.md. Not Accepted Risk: this is a NEW field introduced in Layer 6, not a previously-adjudicated surface; the analogous title/label surfaces were adjudicated as code-fixes, not as accepted risks, so a consistency principle says description should follow the same route unless SO explicitly chooses otherwise.

**Self-dismissal test:** Can the defense be circumvented? There IS no defense currently — `validate_description` is empty-after-trim only and `issue_fields_are_valid` checks empty-after-trim only. Self-dismissal fails because there is no defense to dismiss. The finding stands.

### Dismissed

#### Finding 2 — Description with `\n` leaks into `list` output (A3)

Concern: a description containing `\n` might somehow reach `cmd_list` and break the one-issue-per-line `list` contract.

Verified: `cmd_list` (`src/lib.rs:583-664`) reads only `id`, `status`, `priority`, `labels`, `title` from each issue — never `description`. The list-rendering closure constructs a row from these five fields and `truncate_with_ellipsis` further bounds `labels` and `title` to fixed widths. Reproducer with a `\n`-containing description shows `list` output identical to a `\n`-free description (other than absent description column). **Dismissed.** The Layer 4 one-issue-per-line contract is preserved.

#### Finding 3 — Concurrent `tracker delete N1` / `tracker delete N2` race (A4)

Concern: two simultaneous deletes could corrupt `tracker.json` or crash the binary.

Reproducer: five rounds of four parallel `tracker delete N` invocations against a fresh 8-issue store. Result: no panics, no crashes, all four target IDs removed cleanly across rounds, `tracker.json` ended well-formed (JSON-parseable, 4 records remaining). Last-writer-wins on the file write is the observed behavior; this matches DESIGN.md "Constraints" (single user, no concurrency contract) and is consistent with how every other mutating subcommand behaves. **Dismissed (Accepted-Risk-adjacent).** The single-user threat model (DESIGN.md "Single user. No network. No accounts.") names the user as the named owner of any concurrency loss; the binary's behavior on contended writes is graceful, not crash-prone.

#### Finding 4 — `tracker show 99` error message escape interpolation (A5)

Concern: `Error: Issue #99 not found.` could echo a control byte if `99` was attacker-controlled.

`parse_id` (`src/lib.rs:291-298`) rejects non-`u64` input *first* — and its error path uses `display_safe(raw)` (verified by reproducer in A6 below). Only a validated `u64` ever reaches the `Issue #{} not found.` format site. A `u64` cannot contain a control character in its `Display` output. **Dismissed.** No injection surface.

#### Finding 5 — `tracker show abc` error message escape interpolation (A6)

Concern: `Error: 'abc' is not a valid issue ID.` could echo a control byte from the raw input.

Reproducer: `tracker show $'<ESC>[31mabc' 2>&1 | od -c` produced `Error: '\u{1B}[31mabc' is not a valid issue ID.` — the ESC byte rendered as the literal six-character escape `\u{1B}`. The `parse_id` error path (`src/lib.rs:294`) calls `display_safe(raw)`, inherited correctly from the Layer 4 R2 F2 fix. Same observed for `tracker delete $'<ESC>[31mabc'`. **Dismissed.** Inherits the F2 closure intact.

#### Finding 6 — Path traversal on `tracker.json` argument (A8)

Concern: `tracker show` / `tracker delete` might accept a user-controlled path argument.

`src/main.rs:84` hardcodes `Path::new("tracker.json")`; no `show` or `delete` subcommand accepts a path argument. CWD-relative behavior is spec-defined (DESIGN.md "File location"). **Dismissed.** Same posture as RT R6 F8/F6.

### Accepted Risk

#### Finding 7 — Bidi-override / `Cf` / zero-width characters in description (A7)

Concern: the same Trojan-Source / zero-width attack RT R6 F3 documented for title/label is also valid for description.

Reproducer: `tracker create "Real" --description $'attack<RLO>suoicilam'` accepts the UTF-8 bytes `342 200 256` (U+202E RIGHT-TO-LEFT OVERRIDE) and `tracker show 1` renders them on a TTY as the visually-misleading `attackmalicious`.

DESIGN.md "Edge Cases / Title" (line 314) documents bidi/`Cf`/zero-width as out-of-threat-model for the single-user local tool. SO Review 17 made this an explicit Accepted Risk for the title/label surfaces. The same threat model basis (DESIGN.md "Constraints": Single user. No network. No accounts.) and same risk owner (the director, the user of this branch) extend identically to description. Re-evaluation trigger: any future use case widening the threat model (multi-user / shared `tracker.json`) re-opens this finding for description simultaneously with title/label.

**Classification:** Accepted Risk. Risk owner: director. Re-evaluation trigger named in DESIGN.md "Edge Cases / Title" (line 314).

#### Finding 8 (carried) — Plaintext `tracker.json`

Unchanged from RT R6 F10 / RT R7 F10. **Risk owner:** the user/developer (DESIGN.md "Constraints").

### Hallucinated

None this round. Each candidate finding that initially looked exploitable was confirmed by a byte-level reproducer (Finding 1) or refuted by a byte-level reproducer (Findings 2, 3, 4, 5, 6).

### Summary

Round **8** logged. Cold-session cold-batch produced **one Open finding (F1, Raised to SO/SE/QE)**, **five Dismissed**, **two Accepted Risk** (F7 new for Layer 6 — by analogy to RT R6 F3 carried via SO Review 17; F8 carried). Zero Hallucinated.

**Top exploitable finding:** F1 — `validate_description` and `issue_fields_are_valid` both accept control characters in description; `tracker show` prints them raw to stdout. Five reproducers (A–E) covering the create-time path, the load-time path, OSC 8 hyperlink injection, bare-CR line-overwrite, and combined clear-screen-via-multi-line-description. This is the third instance of the same control-character-rendering vulnerability class on this project (title at Layer 1, labels at Layer 4, description at Layer 6). The pattern Security 7 named ("every layer that adds a free-form text field reopens the terminal-escape attack surface unless the defense is generalized") is now empirically validated across three consecutive layers.

**Carry-over status for the description-control-char-defense lineage:** Open. The Layer 4 F1 fix (SO Review 17 + SE Review 12) closed labels via `label_is_valid` and `parse_label` extensions. Description received NO equivalent treatment at Layer 6; the spec is silent on the property and the implementation is silent on the property. The strategic fix Security 7 already named — a `display_safe`-style helper applied at every output sink, not per-validator rules at every input boundary — remains the architecturally cheapest closure for the whole lineage going forward. F1's recommended remediation Option (c) (sanitize at `format_show_block`) is the direct instantiation of that strategy for this layer. Whichever option SO chooses, the design principle to preserve forward is: any new free-form text field added in a future layer (notes, comments, attachment names, anything that flows through a stdout render) is presumptively in-scope for the same control-character rule unless DESIGN.md explicitly says otherwise.

**Adversarial honest assessment:** I tried hard to find additional Open findings beyond F1 — multi-line list leakage, concurrent-delete corruption, error-message reflection on the new `show`/`delete` paths, hardcoded-path bypass, bidi escalation. None of these stood up to byte-level reproducers. F1 alone is the Layer 6 attack-surface signal, and it is a textbook instance of a known regression pattern. A round-2 verification by a fresh reviewer would be the higher-confidence closure of the dismissed list; this review log is the round-1 cold-batch product.

**Coordination:**

- [SOLUTION-OWNER-REVIEW.md](SOLUTION-OWNER-REVIEW.md) — F1 requires a DESIGN.md "Edge Cases / Description" amendment specifying the control-character contract (three options enumerated in F1 above). F7 should appear as an Accepted Risk extension consistent with the existing line-314 stance.
- [SOFTWARE-ENGINEER-REVIEW.md](SOFTWARE-ENGINEER-REVIEW.md) — F1 fix shape depends on the SO adjudication; the simplest (Option a/b) is a `description_is_valid` helper analogous to `label_is_valid`. Option c (sanitize at `format_show_block`) is architecturally cleaner because it closes the load-path with no additional validator.
- [SECURITY-REVIEW.md](SECURITY-REVIEW.md) — F1 is independently a Security Dim 6 / Dim 7 finding; cross-reference Security Review 9 for the same byte-level reproducers.
- [QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md) — Regression tests on both input-boundary and load-boundary paths for description, mirroring the Layer 4 R2 label tests.
- [VDD-IAR-ALIGNMENT-REVIEW.md](VDD-IAR-ALIGNMENT-REVIEW.md) — Layer 6 gate cannot legitimately close with F1 Open. This is the third repetition of the same regression class across three layers; flag for VDD-IAR Alignment as a systemic spec-coverage issue. The pattern "new free-form field added without explicit control-character contract" should become a Layer-N Red Gate criterion if it isn't already.

**Files modified:** Only this review log appended. No source, tests, or DESIGN.md changes per IAR domain authority boundaries (CLOSURE-PROTOCOL.md).

---
