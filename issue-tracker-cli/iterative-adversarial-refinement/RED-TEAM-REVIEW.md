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

