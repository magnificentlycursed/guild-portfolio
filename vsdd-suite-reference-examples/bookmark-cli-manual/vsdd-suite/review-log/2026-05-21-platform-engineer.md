# Platform Engineer Review — bookmark-cli-manual

[Index](../PLATFORM-ENGINEER-REVIEW.md)

---

## Review 4 — 2026-05-22 00:30Z

**Source:** domain-raised — Phase 3 IAR Round 1 cold-session pass against the Layer 2 (tag + filter) implementation; cluster C of the cluster-batched Layer 2 Round 1 (cluster manifest: SA + Red Team + PE, with adversarial pairs Security and VDD-IAR Alignment carved out to clusters B and D per [AI Engineer R1 F1](2026-05-21-ai-engineer.md) cluster-batching discipline).

**Scope:** First Platform Engineer round against the Layer 2 implementation (4 commits on `bookmark-cli-manual-layer-2`: [`5ba62d5`](https://github.com/magnificentlycursed/guild-portfolio/commit/5ba62d5) Phase 1 → [`326e25d`](https://github.com/magnificentlycursed/guild-portfolio/commit/326e25d) Phase 2a/2b → [`16ee420`](https://github.com/magnificentlycursed/guild-portfolio/commit/16ee420) manual-tests → [`98b5886`](https://github.com/magnificentlycursed/guild-portfolio/commit/98b5886) Phase 2c). Read [`.github/workflows/bookmark-cli-manual.yml`](../../../../.github/workflows/bookmark-cli-manual.yml) (existing 5-job CI workflow, post-Layer-1-R3 SHA-pinned + lint-set-completed); [`Cargo.toml`](../../Cargo.toml) (post-Layer-1-R3 lints + profile.release + dual-license); [`rust-toolchain.toml`](../../rust-toolchain.toml) (`channel = "1.95"`); [`deny.toml`](../../deny.toml) (4-section policy); [`manual-tests/install-verification.md`](../../manual-tests/install-verification.md) (Layer 1 PR #41 nwhitehead PASS row); [`manual-tests/layer-2.md`](../../manual-tests/layer-2.md) (Layer 2 manual-test plan); the portfolio-root [`.pre-commit-config.yaml`](../../../../.pre-commit-config.yaml) (per-project detection logic post-Layer-1-R2 F7 closure).

**Lens:** PE Dim 1/2 (CI/CD coverage of the Layer 2 test surface — does the existing `.github/workflows/bookmark-cli-manual.yml` exercise the 13 new integration tests on every push?); Dim 3 (`--locked` enforcement at Layer 2 install sites in `manual-tests/layer-2.md`); Dim 4 (`rust-toolchain.toml` constraint vs. Layer 2's `std::os::unix::fs::OpenOptionsExt` use + `fsync_directory`); Dim 10 (pre-commit hooks coverage of the new Layer 2 surface); Dim 11 (`cargo deny check` Layer 2 — any new dependencies?); Dim 38 (capstone-required fresh-system install verification at Layer 2 — does the existing PASS row carry forward?).

**Session note:** Cold-context session — this reviewer did not author the Layer 2 artifact or any preceding PE round. Sycophancy-compensation: the PE domain prompt warns that the dominant failure mode is rationalizing inapplicability — each Layer 2 dimension was tested against the actual artifacts (CI workflow, Cargo.toml, manual-tests/layer-2.md, install-verification.md, .pre-commit-config.yaml) rather than reasoned-about in the abstract. The Layer 1 PE R3-close MVR-blocked-by-operator-gate disposition closed when PR #41 landed nwhitehead's Ubuntu 24.04.4 PASS row; the regression-check focus is what changed at Layer 2 and what carried forward unchanged. Cluster-batched session per the [primer 3](../../../../vsdd-suite/primers/3-review-session.md) § Session isolation framing.

**Reviewer:** platform-engineer (cold session, no in-conversation context from Layer 2 authoring).

**Model:** Sonnet 4.6 (conceptually, per the cost-discipline routing for PE per [`DESIGN.md`](../../DESIGN.md) line 19; executed at Opus 4.7 in this session per the cluster-batched cold-session shape).

**Cold-session shape:** Cluster C (SA + Red Team + PE in one cluster pass per Review 88-era cluster-batching with adversarial-pair separation — Security to cluster B, VDD-IAR Alignment to cluster D).

**Regression-check against:** [PE Review 1 — 2026-05-20 19:30Z](2026-05-20-platform-engineer.md#review-1--2026-05-20-1930z) (Layer 1 R1 — 11 Open + 2 Dismissed), [PE Review 2 — 2026-05-20 21:00Z](2026-05-20-platform-engineer.md#review-2--2026-05-20-2100z) (Layer 1 R2 — 8 Resolved + 5 Deferred including F9 install-verification operator-pending), [PE Review 3 — 2026-05-20 22:00Z](2026-05-20-platform-engineer.md#review-3--2026-05-20-2200z) (Layer 1 R3 — 2 Resolved-via-fix + 1 carried-Deferred-operator-gate; closed at MVR-blocked-by-operator-gate). The operator gate closed via PR #41 (nwhitehead's Ubuntu 24.04.4 PASS row at [`manual-tests/install-verification.md:55`](../../manual-tests/install-verification.md)) — Layer 1 PE Dim 38 satisfied.

**Cost-tally placeholder:** see Summary.

---

### Resolved

<a id="r4-f1"></a>

**Finding 1 — Layer 2's 13 new integration tests are exercised by the existing CI `test` job on every push; CI coverage holds at Layer 2 without modification (Dim 1)**

**Owner:** platform-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** software-engineer

[`.github/workflows/bookmark-cli-manual.yml:87-111`](../../../../.github/workflows/bookmark-cli-manual.yml) — the `test` job runs `cargo test --locked` against the project. The Layer 2 commits add 13 new integration tests to [`tests/bookmarks.rs`](../../tests/bookmarks.rs) (lines 504–982, AC 5 through AC 13 + Layer-1-Deferred RFC3339 closure). All Layer 2 tests live in `tests/bookmarks.rs` (the same file the Layer 1 tests live in); `cargo test --locked` exercises all integration tests in the `tests/` directory by default, so the Layer 2 additions are covered automatically.

The `clippy` job at [`.github/workflows/bookmark-cli-manual.yml:63-85`](../../../../.github/workflows/bookmark-cli-manual.yml) runs `cargo clippy --all-targets --locked -- -D warnings` — `--all-targets` covers the test files in addition to the binary + library. With the [`Cargo.toml`](../../Cargo.toml) lines 62–81 lint set (`[lints.clippy]` deny-all + restriction-group), any clippy warning on the new test code would fail the build.

The `fmt` job at lines 39–61 covers all `.rs` files via `cargo fmt --check`.

The `deny` and `audit` jobs (lines 113–162) run unchanged — the Layer 2 commits added NO new dependencies (verified by checking the Cargo.toml diff at lines 24–28 in [`Cargo.toml`](../../Cargo.toml): `clap`, `serde`, `serde_json`, `chrono`, `anyhow` — same five direct deps as Layer 1). The Cargo.lock would only change if the Cargo.toml semver constraints resolved to a different point — verified Cargo.lock is unchanged by the Layer 2 diff (`git diff e9b6d37..98b5886 -- vsdd-suite-reference-examples/bookmark-cli-manual/Cargo.lock` returns no output). The `cargo audit` + `cargo deny check` jobs gate the same supply-chain surface at Layer 2.

**Resolution:** The Layer 1 R2 PE F1 (no CI workflow) closure + R3 lint-set completion fully covers the Layer 2 surface without modification. No new CI artifact required.

**Classification:** Resolved (Dim 1)

---

### Deferred

<a id="r4-f2"></a>

**Finding 2 — `tests/scaling.rs` referenced by DESIGN.md + TODO.md does NOT exist; the data-scaling sentinel tests at 100/1,000/10,000-bookmark cliffs are not landed; the CI `--ignored` job for them is absent (Dim 1)**

**Owner:** software-engineer
**Status:** raised
**Blocked by:** *(none — the file should land in the Layer 2 fix cycle)*
**Validator:** quality-engineer

[`DESIGN.md`](../../DESIGN.md) line 230 declares the Layer 2 commitment: "Layer 2 ships sentinel integration tests at the 100 / 1,000 / 10,000-bookmark cliffs that exercise the full add → list → tag → list-filter cycle. Each cliff asserts: (a) operations complete within the budget table above; (b) the storage file round-trips without corruption; (c) the filter result set is correct against a programmatically-generated reference. The tests live in `tests/scaling.rs` and use `#[ignore]` by default so `cargo test` stays fast; CI runs them via `cargo test -- --ignored` in a separate job."

[`TODO.md`](../../TODO.md) line 81 echoes: "**Layer 2 data-scaling tests:** `tests/scaling.rs` with `#[ignore]`-gated sentinels at 100/1,000/10,000 bookmark cliffs."

Verified absent: `ls vsdd-suite-reference-examples/bookmark-cli-manual/tests/` returns only `bookmarks.rs`. The `scaling.rs` file is not present in the Layer 2 commits (`git diff e9b6d37..98b5886 -- vsdd-suite-reference-examples/bookmark-cli-manual/tests/` lists only `bookmarks.rs`). The CI workflow has no `--ignored` job (verified by grep against [`.github/workflows/bookmark-cli-manual.yml`](../../../../.github/workflows/bookmark-cli-manual.yml) — no `cargo test -- --ignored` invocation).

This is a spec-vs-implementation divergence at the PE-relevant layer. The DESIGN.md commitment is the spec contract; the implementation does not match. Three angles of significance:

1. **Layer 2 closure dependency.** Per [`TODO.md`](../../TODO.md) line 91 Layer-gate criterion 1: "All Red Gate tests above pass: `cargo test --test bookmarks` + `cargo test -- --ignored` (scaling)." The second clause cannot be exercised — there is no `--ignored` test suite to run.
2. **Performance Engineer Layer-1-Deferred item.** The PE R1 F5 Performance Engineer review (referenced via [`DESIGN.md`](../../DESIGN.md) line 230) is the deferral source. The Layer 2 cycle was supposed to close it via this artifact; the artifact does not exist.
3. **CI infrastructure missing.** Even if `scaling.rs` lands tomorrow, the CI workflow does not yet have the `--ignored` job. The Rust supplement § PE coverage discipline applies — measuring without gating is the same failure as having no measurement.

Recommendation: Two-part fix:

(a) Land `tests/scaling.rs` with `#[ignore]`-gated tests per DESIGN.md spec. Implementation guidance: the test pattern can mirror the [`manual-tests/layer-2.md:454-481`](../../manual-tests/layer-2.md) Step 12a programmatic-generation pattern (Python emit for 1,000 bookmarks; Rust-side equivalent via `serde_json` direct construction). Each cliff (100, 1000, 10000) runs the add → list → tag → list-filter cycle; assert wall-clock budgets from DESIGN.md § Performance budget hold.

(b) Add a CI `scaling` job to [`.github/workflows/bookmark-cli-manual.yml`](../../../../.github/workflows/bookmark-cli-manual.yml) running `cargo test --locked -- --ignored`. Per the existing per-job structure, the `scaling` job is a separate job alongside `fmt` / `clippy` / `test` / `deny` / `audit`. Pin the same toolchain + actions as the existing jobs. The cliff tests are slow by design (10,000-bookmark cycle is the worst case); the 10-minute timeout-minutes (per the existing `timeout-minutes: 10` in each job) is likely sufficient but the `scaling` job should be explicitly cost-monitored.

Why Deferred rather than Open: the artifact is a small implementation deliverable (a Rust test file + 10 YAML lines for the CI job); the fix should land in the Layer 2 fix cycle. Routed to [SE](../SOFTWARE-ENGINEER-REVIEW.md) for the test file authoring and [Quality Engineer](../QUALITY-ENGINEER-REVIEW.md) for the cliff-threshold ratification; PE owns the CI-side wiring once the test file exists. Auto-Backlog: at Layer 2 final closure if no `scaling.rs` lands, the Layer 2 closure attestation must note "Layer-1-Deferred PE R1 F5 carried forward to Layer 3 or backlogged."

**Classification:** Deferred. Trigger to close: `tests/scaling.rs` lands with `#[ignore]`-gated tests AND CI workflow gains the `scaling` job invoking `cargo test --locked -- --ignored`. (Dim 1)

---

<a id="r4-f3"></a>

**Finding 3 — `manual-tests/install-verification.md` Verification records table needs a Layer 2 row to attest Layer 2 install-verification holds; the Layer 1 nwhitehead PASS row attests to a Layer 1 binary, not the Layer 2 binary (Dim 38)**

**Owner:** platform-engineer
**Status:** raised (operator-pending; routing to non-author for execution)
**Blocked by:** operator execution
**Validator:** *self* — Dim 38 is binary against [G-155](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-155): the validation surface is the file content itself (a PASS row from a non-author on a fresh system); no second reviewer adds adversarial signal that file-presence-checking doesn't already produce. The strict-self-validation rationale matches Layer 1 PE R1 F9 / R2 F9 / R3 F3 disposition shape.

[`manual-tests/install-verification.md:53-56`](../../manual-tests/install-verification.md) Verification records table contains two rows:

```
| Thu May 21 07:40:36 PM UTC 2026 | nwhitehead | Ubuntu 24.04.4 LTS / rust 1.95.0 | 0-6 | NONE | PASS | |
| *(pending)* | *(non-author operator)* | *(fresh-system context)* | *(per manual-tests/layer-1.md execution)* | *(divergences, if any)* | *(PASS / FAIL)* | *(any context)* |
```

The nwhitehead row attests to `manual-tests/layer-1.md` Steps 0–6 against the Layer 1 binary (per the row's "Manual-test steps that PASSED: 0-6" — these refer to Layer 1's 7-step plan). The Layer 2 manual-test plan at [`manual-tests/layer-2.md`](../../manual-tests/layer-2.md) introduces 13 NEW steps (Step 0 through Step 13) against the Layer 2 binary, including a new prerequisite ([`hyperfine`](https://github.com/sharkdp/hyperfine) at Step 12 — see [`manual-tests/layer-2.md:446-452`](../../manual-tests/layer-2.md)) NOT named in Layer 1's plan.

Three angles on whether Layer 2 needs a NEW install-verification row:

1. **Strict reading of G-155 dim 38** ("a single PASSING row from a non-author on a fresh system is sufficient to satisfy dim 38"): the Layer 1 PASS row IS a passing row; the discipline does not require per-layer rows. The Layer 1 nwhitehead row satisfies Dim 38 in the absolute sense.

2. **Layer-aware reading:** the Layer 2 binary has a different install profile — same `cargo install --locked --path .` command, but the binary is now the Layer 2 binary (different code paths, new `bm tag` subcommand). A fresh-system attempt to follow [`manual-tests/layer-2.md`](../../manual-tests/layer-2.md) end-to-end may hit divergences that Layer 1's plan did not exercise (e.g., the Step 12 hyperfine prerequisite is new and may surface a fresh-environment install-doc gap). The Layer 1 PASS does not attest that Layer 2's plan runs clean.

3. **The capstone-tier methodology bar:** for the project to claim "Layer 2 reaches capstone-tier MVR per the methodology," the project needs evidence that a non-author can install + verify Layer 2 end-to-end. The Layer 1 row is necessary but not sufficient evidence for Layer 2 capstone closure.

The PE R1+R2+R3 Layer 1 install-verification disposition (operator-blocked-pending-execution) closed when PR #41 landed. The same disposition shape applies to Layer 2: the AI author cannot satisfy the gate; the operator must execute. The fresh-system attempt could be the same operator (nwhitehead) or a different non-author.

Recommendation: Either (a) the operator coordinates a Layer 2 fresh-system install attempt + PASS row addition to the Verification records table (preferred for capstone-tier methodology bar); OR (b) the project explicitly documents in [`DESIGN.md`](../../DESIGN.md) § Phase 6 strategy that the Layer 1 install-verification PASS row covers Layer 2 because the install command is identical and the divergences are accepted (looser interpretation; defensible per the strict G-155 reading but weakens the per-layer Phase 6 attestation discipline).

The Red Team / Security analog cross-reference: the [`manual-tests/install-verification.md`](../../manual-tests/install-verification.md) line 30 explicitly names what the fresh-system verifier should check: "the `Cargo.lock`, `rust-toolchain.toml`, and `deny.toml` files exist." Layer 2 adds NO new project-level configuration files (verified by `ls`); the install profile is unchanged. The Layer 2-specific verification step is Step 12 (hyperfine), which is NOT part of the install-verification's scope — it's a per-layer manual-test step.

Procedural fix orientation: the cleanest disposition is (a) — add a Layer 2 row to the Verification records table. Update the row template at line 56 to read "manual-tests/layer-2.md" rather than "manual-tests/layer-1.md" — OR add a NEW pending row for Layer 2 alongside the Layer 1 row. The operator's call.

Why Deferred-operator-blocked rather than Open: the implementation gate is operator-executable; the AI author cannot satisfy by construction. Same disposition shape as Layer 1 R2 F9 / R3 F3 (which closed when PR #41 landed). Trigger to close: a PASS row from a non-author on a fresh system citing `manual-tests/layer-2.md` Steps 0–13. Auto-Backlog: at Layer 2 final closure if no row lands.

**Classification:** Deferred — operator-blocked. Cross-references Layer 1 PE R1 F9 / R2 F9 / R3 F3. (Dim 38)

---

<a id="r4-f4"></a>

**Finding 4 — `Cargo.toml` `rust-version = "1.78"` may be incorrect now that Layer 1 R3-fix used `reason = "..."` attribute on `#[allow(...)]` — the actual MSRV is 1.81+; the manifest underclaims the minimum (Dim 4)**

**Owner:** software-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** platform-engineer

[`Cargo.toml`](../../Cargo.toml) line 10 declares `rust-version = "1.78"`. [`rust-toolchain.toml`](../../rust-toolchain.toml) line 16 pins `channel = "1.95"`. The MSRV check applies to consumers using the crate via `cargo install` (cargo refuses to build with a toolchain < `rust-version`); the `rust-toolchain.toml` channel is what every contributor and CI runner uses.

The Layer 1 PE R1 F2 closure verified that `1.78` covered the Layer 1 implementation. The Layer 2 implementation adds:

1. **`std::os::unix::fs::OpenOptionsExt`** at [`src/lib.rs:450`](../../src/lib.rs) — stable since Rust 1.0. ✓ Compatible with 1.78.
2. **`std::fs::File::sync_all`** at [`src/lib.rs:443`](../../src/lib.rs) and elsewhere — stable since 1.0. ✓
3. **`std::process::id()`** at [`src/lib.rs:426`](../../src/lib.rs) — stable since 1.26. ✓
4. **`SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, ...)`** — `map_or` on `Result` is stable since 1.41. ✓
5. **`#[allow(..., reason = "...")]`** at [`src/lib.rs:580-589`](../../src/lib.rs) AND [`tests/bookmarks.rs:1-10`](../../tests/bookmarks.rs) — the `reason = "..."` attribute on lint suppressions was stabilized in **Rust 1.81** (per the [Rust 1.81 release notes](https://blog.rust-lang.org/2024/09/05/Rust-1.81.0.html) — "Stabilized `lint_reasons`"). The PE R3 F2 finding body cites: "the `reason = '...'` attribute is the modern Rust convention (stable since Rust 1.81)."

The Layer 1 PE R3 F2 closure noted the 1.81 requirement and DID NOT update the `Cargo.toml` `rust-version`. This is a real MSRV-manifest divergence: the codebase actually requires Rust 1.81+ (because of the `reason = "..."` attribute), but the manifest claims 1.78+. A consumer on Rust 1.78–1.80 attempting to build this crate will get a compilation error on the `reason = "..."` syntax, not a `rust-version`-mismatch error.

The MSRV-manifest check is the **first line of defense** against MSRV drift: `cargo install` refuses to build if the toolchain is below `rust-version`. With `rust-version = "1.78"`, a user on Rust 1.79 would `cargo install` successfully (per the resolver) and then hit a compilation error mid-build. The current state is a worst-of-both-worlds shape: the manifest's MSRV declaration is not the actual MSRV.

Verification: `grep -rn 'reason = ' vsdd-suite-reference-examples/bookmark-cli-manual/src/ vsdd-suite-reference-examples/bookmark-cli-manual/tests/` returns 2 matches (the `#[allow(..., reason = "...")]` blocks at `src/lib.rs:586` and `tests/bookmarks.rs:7`). Both are in `#[cfg(test)]` paths but the `tests/bookmarks.rs` one is in the file's outer `#![allow(...)]` which compiles unconditionally for the integration-test build. The actual MSRV including this attribute is 1.81.

Recommendation: Bump [`Cargo.toml`](../../Cargo.toml) line 10 from `rust-version = "1.78"` to `rust-version = "1.81"`. Update [`DESIGN.md`](../../DESIGN.md) § Constraints line 211 to match. Also update [`README.md`](../../README.md) § Prerequisites if it cites 1.78. The `rust-toolchain.toml` `channel = "1.95"` already covers this comfortably; the change is purely MSRV-manifest correctness.

Adjacent concern: the Layer 1 PE R3 F2 closure adopted the `reason = "..."` attribute without flagging the MSRV implication. This is a finding-cycle adjacency — the PE R3 round closed by adding a feature that required a toolchain bump but didn't propagate the bump to the manifest. The cluster-batched session-isolation may have masked the cross-finding implication.

**Classification:** Deferred — small mechanical fix (one-line edit to `Cargo.toml`, optional doc updates). Routes to [Software Engineer](../SOFTWARE-ENGINEER-REVIEW.md) for the change. Trigger to close: `Cargo.toml` `rust-version` matches actual minimum AND DESIGN.md/README updated. (Dim 4)

---

<a id="r4-f5"></a>

**Finding 5 — Parent-directory `fsync` works on commodity SSD per the DESIGN.md spec but the Layer 2 implementation is NOT tested against the realistic deployment-surface filesystems (tmpfs, btrfs, CIFS-mounted network drive); the test coverage exercises only what tempfile's default `/tmp` filesystem provides (Dim 20)**

**Owner:** performance-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** quality-engineer

[`DESIGN.md`](../../DESIGN.md) line 228 declares the durability commitment: "the save path uses `tempfile + rename(2)` for atomic replacement (preserves the prior file's contents on partial failure). Layer 2 adds an explicit `fsync` of the destination file's parent directory after the rename ... benchmarked at the Layer 2 Performance Engineer Round against the budget table above (expected < 5 ms on commodity SSD). The fsync is gated `#[cfg(unix)]`."

The `fsync(2)` semantics are filesystem-specific:

- **ext4/xfs (typical commodity SSD on Linux):** `fsync(parent_dir)` syncs the directory entry to disk; the spec's "< 5 ms" budget is the typical commodity SSD shape.
- **tmpfs (RAM-backed):** `fsync` is a no-op (or near-no-op); no durability guarantee across power loss (the data is in RAM). The test environment likely uses `tempfile::tempdir()` which on macOS and most Linux setups defaults to a real filesystem (`/tmp` on Linux is typically a real filesystem; `/var/folders/...` on macOS is a real filesystem). But `/tmp` on Linux IS tmpfs on systemd-default + RAM-tmpfs configurations.
- **btrfs (copy-on-write):** `fsync(parent_dir)` semantics include the COW snapshot — durability guarantees hold but with different performance characteristics than ext4.
- **CIFS-mounted network drive:** `fsync` semantics are weakened (some CIFS clients don't honor parent-dir-fsync); the durability guarantee may not hold across power loss on the SMB server.
- **NFSv3/v4:** sync semantics are mount-option-dependent; the user's `$BOOKMARK_CLI_DB` may be on an NFS-mounted home directory where the spec's "< 5 ms commodity SSD" budget is wildly violated.

The implementation correctness is robust — it issues the `fsync` syscall; the kernel/filesystem handles whatever durability semantic that fs provides. The PE-relevant concern is:

1. **The CI test environment.** GitHub-hosted Ubuntu runners typically use `/tmp` on a real filesystem (ext4 on the runner's local SSD). The Layer 2 `tests_save_fsyncs_parent_directory` test at [`src/lib.rs:794-813`](../../src/lib.rs) uses `tempfile::tempdir()` which inherits the test environment's `/tmp` filesystem. The test exercises the syscall; the durability semantic depends on whatever filesystem CI provides.

2. **Per the test's own self-disclosure** at [`src/lib.rs:776-793`](../../src/lib.rs): "This is a WEAK PROXY for the durability contract — it confirms the save codepath executes successfully against a real filesystem (the same codepath that includes the fsync on Unix) but does not directly verify the fsync syscall was issued. Direct verification would require either: (a) an injected trait/seam at the syscall boundary, which would add complexity disproportionate to the Layer 2 budget; or (b) a `strace`/`dtruss` harness, which is platform-specific + outside the `cargo test` discipline. Deferred per the test plan in `TODO.md` § Layer 2 Red Gate test 14."

3. **`TODO.md` § Layer 2 Red Gate test 14** at [`TODO.md`](../../TODO.md) line 77 declares: "`tests_save_fsyncs_parent_directory` (closes operator-queued PE fsync item) — adds a bookmark, asserts the `save` codepath invoked `fsync(2)` on the parent directory FD after the `rename(2)`. Implementation strategy: extract the durable-save into a function whose effect is observable from a unit test (an injected counter or trace-line on the unix path); the integration test asserts the observable." The actual implementation in `src/lib.rs:794-813` does NOT exercise the planned "injected counter or trace-line" — it's the weak-proxy roundtrip test.

The architectural gap: the Layer 2 spec promised a fsync benchmark + a direct fsync assertion; the implementation has neither. The hyperfine sanity-check at [`manual-tests/layer-2.md:482-507`](../../manual-tests/layer-2.md) Step 12b will exercise the wall-clock budget (< 100 ms per operation including the fsync), but a "< 5 ms commodity SSD fsync overhead" attestation requires either (a) a per-syscall benchmark distinguishing fsync from the rest of save, OR (b) accepting the manual-test step as the closing-evidence record.

Recommendation (two-part):

(a) **Filesystem-coverage caveat.** Add to [`DESIGN.md`](../../DESIGN.md) § Performance budget "Durability discipline (Layer 2)" line 232: "The 'commodity SSD' assumption is the test-coverage envelope. Filesystems with weaker fsync semantics (tmpfs — fsync is a no-op; CIFS/SMB — fsync may not propagate to the server; NFSv3 — sync semantics are mount-option-dependent) provide weaker durability than the spec claims; users on those filesystems should not rely on the parent-dir-fsync for power-fail durability."

(b) **Direct fsync assertion.** Per the TODO.md § Layer 2 Red Gate test 14 plan, the proper implementation is a seam test. The Layer 1 R3 close adopted the curated approach; Layer 2 should at minimum document the divergence between plan and impl. Recommendation: either land the seam-test implementation, OR update [`TODO.md`](../../TODO.md) line 77 to match the actual weak-proxy implementation (and explain why the seam test was not pursued).

Why Deferred rather than Open: the test coverage gap is real but the implementation correctness is not in question; the durability semantic depends on the filesystem at runtime, which is outside the project's control. The recommendation is a documentation update + a low-priority test improvement.

**Classification:** Deferred — Coordination to [Performance Engineer](../PERFORMANCE-ENGINEER-REVIEW.md) for the deployment-surface filesystem caveat (PerfE owns the budget contract); routing to [Quality Engineer](../QUALITY-ENGINEER-REVIEW.md) if the seam-test implementation is pursued. (Dim 20)

---

### Dismissed

<a id="r4-f6"></a>

**Finding 6 — `deny.toml` does not cover the Layer 2 `Vec<String>` storage extension's adversarial-cargo-crate pivot path (a malicious crate could pivot from tag-storage into a different exploit path) (Dim 12)**

**Owner:** platform-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** sanity-check

The user-prompt names this as "a stretch; if no finding, document the reasoning explicitly." Reasoning:

The `deny.toml` policy at [`deny.toml`](../../deny.toml) gates four supply-chain concerns: known CVEs (`[advisories]`), license violations (`[licenses]`), banned/duplicate crates (`[bans]`), disallowed sources (`[sources]`). These gate the dependency graph that ships in the binary; they do NOT gate the data flow inside the binary.

The Layer 2 `Vec<String>` storage extension does not introduce any new dependency, and the existing dependencies (`serde`, `serde_json`, `clap`, `chrono`, `anyhow`) handle string storage identically to their Layer 1 use. A "malicious crate could pivot from tag-storage into a different exploit path" would require: (a) the malicious crate being a dependency of bookmark-cli; (b) the malicious crate having a CVE or pre-existing exploit path; (c) the Layer 2 storage extension exposing a new data-flow into that exploit. None of (a), (b), or (c) is materially different between Layer 1 and Layer 2 — the dependency surface is unchanged.

The `deny.toml` ALREADY covers the relevant policy surface:

- `[advisories]` would catch any RUSTSEC advisory against `serde` / `serde_json` / `clap` / `chrono` / `anyhow`.
- `[sources]` constrains every dependency to come from `crates.io` (no git URLs).
- `[bans]` denies wildcards (no `*`-version specs).
- `[licenses]` constrains the license shape.

If a future Layer adds a new dependency (e.g., `unicode-general-category` for Red Team R3 F3's broader Cf coverage), the policy would re-gate at that point.

**Classification:** Dismissed. The reasoning is the supply-chain surface is unchanged at Layer 2 (no new dependencies); the existing `deny.toml` policy applies symmetrically to the Layer 2 code. The "pivot from tag-storage" framing fails the specificity test — what concrete crate, what concrete pivot, what concrete exploit? No specific instance can be named because the surface is unchanged. (Dim 12)

---

### Hallucinated

*(none — every Resolved / Deferred / Dismissed finding above is grounded in a specific file:line citation against the Layer 2 implementation. The cold adversary applied the [PE domain prompt](../../../../vsdd-suite/domains/role/PLATFORM-ENGINEER-REVIEW.md) sycophancy-check rigorously: each dimension that produced a finding was tested against the project's actual Layer 2 artifacts; the Dismissed finding (F6) was named and justified rather than silently dropped.)*

---

### Summary

5 findings filed in this Platform Engineer Layer 2 Round 1 cold-session pass + 1 Dismissed: **1 Resolved** (Finding 1 — Layer 2's 13 new integration tests are exercised by the existing CI workflow without modification); **4 Deferred** (Finding 2 — `tests/scaling.rs` referenced but absent + CI `--ignored` job missing; Finding 3 — Layer 2 install-verification row needed at `manual-tests/install-verification.md` operator-pending; Finding 4 — `Cargo.toml` `rust-version = "1.78"` underclaims the actual 1.81 minimum from the `reason = "..."` attribute; Finding 5 — fsync filesystem-coverage caveat needs documentation + planned seam-test diverges from impl); **1 Dismissed** (Finding 6 — `deny.toml` Vec<String> storage-extension pivot path fails the specificity test).

The Layer 1 PE R3-close MVR-blocked-by-operator-gate disposition closed when PR #41 landed nwhitehead's Ubuntu 24.04.4 PASS row. The Layer 2 R1 cold pass produces a clear shape: the Layer 1 platform-control transferred holds (CI workflow, pre-commit hooks, `deny.toml`, `rust-toolchain.toml`, `--locked` enforcement) — none of these need modification for Layer 2. The Layer 2 R1 surface is 3 spec-vs-implementation gaps (Findings 2 + 4 + 5) + 1 operator-gate-row (Finding 3) + 1 Resolved + 1 Dismissed. Per [G-131](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-131) continue-trigger discipline, this round's 4 substantive Deferred findings mandate a Round 2 cold pass after the fix cycle lands.

MVR signal: Platform Engineer reaches **MVR-blocked-by-deferred-implementation-and-operator-gate** for Layer 2. The implementation-gap findings (F2 + F4 + F5) are SE/QE-resolvable in the Layer 2 fix cycle; the operator-gate finding (F3) is the same shape as Layer 1 R2 F9 / R3 F3 and resolves when a fresh-system PASS row for Layer 2 lands. The methodology-correct posture: PE cannot reach standard MVR in this round without operator action on Dim 38 AND the SE/QE fix cycle landing the scaling tests + MSRV bump + fsync caveat.

**Coordination:**

- **Finding 1** (Resolved CI Layer 2 coverage) — no coordination needed; documented for the audit trail.
- **Finding 2** (`tests/scaling.rs` absent + CI `--ignored` job missing) — routes to [Software Engineer](../SOFTWARE-ENGINEER-REVIEW.md) for the test file authoring; [Quality Engineer](../QUALITY-ENGINEER-REVIEW.md) for the cliff-threshold ratification; [Performance Engineer](../PERFORMANCE-ENGINEER-REVIEW.md) for the budget-table alignment.
- **Finding 3** (Layer 2 install-verification row) — operator-routing per the G-155 fresh-system-non-author discipline; surface to [VDD-IAR Alignment review](../VDD-IAR-ALIGNMENT-REVIEW.md) as the meta-process check that the per-layer capstone gate is tracked.
- **Finding 4** (`rust-version` MSRV underclaim) — routes to [Software Engineer](../SOFTWARE-ENGINEER-REVIEW.md) for the manifest edit; mechanical fix.
- **Finding 5** (fsync filesystem-coverage caveat + impl-vs-plan divergence) — Coordination to [Performance Engineer](../PERFORMANCE-ENGINEER-REVIEW.md) for the budget-table caveat documentation; [Quality Engineer](../QUALITY-ENGINEER-REVIEW.md) for the seam-test pursuit decision.
- **Finding 6** (Dismissed) — no coordination; documented reasoning.

**Cost-tally:** Cluster C session (SA + Red Team + PE in one cluster pass) — PE sub-section consumed an estimated ~30k–40k tokens for the cold context-load (PE R1+R2+R3 review-log ~977 lines, Layer 2 spec sections, CI workflow file, Cargo.toml, deny.toml, rust-toolchain.toml, install-verification.md, manual-tests/layer-2.md, .pre-commit-config.yaml), and per-finding evidence-gathering. Per-finding cost ≈ 6k–8k tokens; below the capstone band's 100k–300k/finding range, consistent with cluster-batching efficiency.
