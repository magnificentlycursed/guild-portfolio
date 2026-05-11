# Platform Engineer Review Log

This review is part of the [Iterative Adversarial Refinement (IAR)](README.md) suite. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

**Reviewer role: Platform Engineer** (Platform Engineer / DevOps Engineer / Infrastructure Engineer)

The purpose of this review is to evaluate the delivery platform: CI/CD, build configuration, DevSecOps practices, and toolchain setup. Many standard PE dimensions (cloud infrastructure, containerization, observability dashboards, disaster recovery) do not apply to a local single-user CLI binary with no deployment infrastructure; the review focuses on CI/CD, build tooling, and DevSecOps — the dimensions relevant to a Rust binary project.

**Language supplement applied:** `lang/rust.md` (Platform Engineering section).

**Sycophancy check:** The primary sycophancy risk in this domain is around applicability decisions and threshold acceptance, not binary existence checks. Flag any case where an inapplicable determination was made without examining whether it genuinely does not apply, and any case where an accepted risk was accepted without specific evidence of the risk level.

---

## Review 1 — 2026-04-27 21:00Z

**Scope:** Build configuration requirements. No source code, `Cargo.toml`, or CI configuration exists yet. Pre-implementation pass: identifying platform requirements for Layer 1 setup. Many standard PE dimensions (cloud infrastructure, containerization, observability dashboards, disaster recovery, performance budgets) do not apply to a local single-user CLI binary with no deployment infrastructure and are noted as inapplicable below.

**Session note:** In-session with all other domain reviews. Acknowledged quality tradeoff.

---

### Resolved

**Finding 1 — No `rust-toolchain.toml` (Dim 4 — Environment pinning)**

No Rust toolchain version was pinned. Without `rust-toolchain.toml`, the build uses whatever version of Rust the developer has installed locally, producing non-reproducible builds.

**Resolution:** Created `issue-tracker-cli/rust-toolchain.toml` pinning toolchain to `1.94.1` (the installed stable version at time of writing) with `clippy` and `rustfmt` components.

---

**Finding 2 — No `.gitignore` for Rust (Dim 8 — Artifact hygiene)**

`/target` must be excluded from version control to prevent build artifacts from being committed.

**Resolution:** Created `issue-tracker-cli/.gitignore` excluding `/target`.

---

**Finding 3 — No CI pipeline (Dim 1 — Pipeline completeness)**

No CI configuration existed. The following checks are now automated: `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`, `cargo audit`.

**Resolution:** Created `.github/workflows/issue-tracker-cli.yml` running all required checks on push/PR to paths under `issue-tracker-cli/`. Uses `dtolnay/rust-toolchain@master` with explicit `1.94.1` version pin, `Swatinem/rust-cache@v2` for build caching, and `cargo-audit` installed via `cargo install --locked`. Coverage enforcement (`cargo tarpaulin`) to be added by Layer 2 once enough tests exist to make a threshold meaningful.

---

### Deferred

**Finding 4 — No `Cargo.lock` in version control (Dim 3 — Dependency installation)**

For binary crates (as opposed to libraries), `Cargo.lock` must be committed to ensure reproducible builds. Without it, `cargo build` resolves dependencies non-deterministically and two builds from the same source may produce different binaries.

**Classification:** Deferred to Layer 1. `Cargo.lock` must be committed alongside the initial `Cargo.toml`.

---

**Finding 5 — No pre-commit hooks (Dim 10 — Pre-commit hooks)**

No pre-commit hook configuration exists. For a Phase 1 portfolio project, hooks should cover at minimum: absolute path leakage (local usernames in committed files), and secret/credential detection if any keys are introduced.

**Classification:** Deferred to Layer 1. Pre-commit hooks are a requirement for portfolio projects intended for external review.

---

### Dismissed

**Finding 6 — Inapplicable PE dimensions for this deployment context (Dims 16–37)**

Infrastructure, containerization, observability, disaster recovery (dims 16–33) are not applicable: the project is a local CLI binary with no deployment infrastructure, no cloud resources, no container runtime, and no operational environment. Performance budget and time-to-interactive (dims 34–37) are not applicable: CLI binary with no browser, no asset pipeline, no network latency.

**Classification:** Dismissed. These dimensions do not apply to this deployment context.

---

### Hallucinated

*(none)*

---

### Open

*(none — all open work is captured in the Deferred section)*

---

### Summary

Three findings resolved: `rust-toolchain.toml`, `.gitignore`, and CI pipeline created. Two findings deferred to Layer 1: `Cargo.lock` (requires `Cargo.toml` first) and pre-commit hooks (requires a hook framework decision). One inapplicability finding dismissed. Coverage threshold enforcement deferred to Layer 2 when test volume makes a threshold meaningful.

**Coordination:** *(none)*

---

---

## Review 2 — 2026-04-27 22:00Z

**Scope:** Layer 1 build configuration — `Cargo.toml`, `Cargo.lock` status, CI configuration. Evaluating delivery of the two deferred findings from Review 1.

**Session note:** In-session with all other Layer 1 domain reviews. Acknowledged quality tradeoff.

---

### Resolved

**Finding 1 — `Cargo.lock` not in version control (regression check from Review 1 Finding 4) (Dim 3 — Dependency installation)**

`Cargo.toml` now exists. `cargo test` was run, generating `Cargo.lock` with 66 packages locked (all from dev-dependencies: assert_cmd, predicates, serde_json, tempfile and their transitive dependencies). The `Cargo.lock` file is not in `.gitignore` and must be committed alongside the Layer 1 Red Gate commit.

**Resolution:** `Cargo.lock` must be staged and committed with the Layer 1 Red Gate files. This is a gate requirement for the Red Gate commit, not the merge gate. Action item for the current commit.

---

### Deferred

**Finding 2 — No pre-commit hooks (regression check from Review 1 Finding 5) (Dim 10)**

Still open. No pre-commit hook framework has been configured. For a portfolio project intended for external review, hooks covering local username leakage and secret detection are required.

The framework decision (lefthook, pre-commit, husky, or a shell script in `.git/hooks`) is a human director decision. The minimum requirements:
- Reject commits that include absolute paths containing the developer's local home directory path in committed files. This prevents local machine-specific paths from appearing in public portfolio code.
- The CI pipeline already runs `cargo fmt --check` and `cargo clippy` — pre-commit hooks for these are optional if the developer is disciplined about running them locally.

**Classification:** Deferred. Carried forward as a Layer 1 merge gate requirement. `Cargo.lock` commit (Finding 1) unblocks the Red Gate commit; pre-commit hooks unblock the merge gate.

---

### Dismissed

**Finding 3 — `Cargo.toml` has no `[profile.release]` section (Dim 7)**

No release profile optimization settings are declared. The default release profile (`opt-level = 3`, no debug info) applies.

**Classification:** Dismissed. The default release profile is appropriate for a personal CLI tool. LTO, codegen-units, and strip settings are production optimization concerns. The assignment has no performance budget for the binary; the default profile is correct for this stage.

---

**Finding 4 — CI `cargo audit` step installs `cargo-audit` at CI runtime (Dim 1)**

The workflow runs `cargo install cargo-audit --locked` on every CI run. This adds ~30 seconds to the CI runtime if not cached.

**Classification:** Dismissed. `Swatinem/rust-cache@v2` is configured in the CI pipeline. `cargo install` artifacts are cached between runs on matching toolchain/dependency fingerprints. The first run pays the install cost; subsequent runs use the cache. Acceptable for a portfolio project. Using a pre-built action (e.g., `rustsec/audit-check`) would be an optimization but is not required.

---

### Hallucinated

*(none)*

---

### Open

*(none)*

---

### Summary

Finding 1 resolved: `Cargo.lock` must be committed with the current Red Gate commit. Finding 2 deferred — pre-commit hooks gate the Layer 1 merge. Two dismissed. One action item: commit `Cargo.lock` now.

**Coordination:** *(none)*

---

---

## Review 3 — 2026-04-28 05:30Z

**Scope:** Layer 1 implementation — build configuration with runtime dependencies added. Evaluating `Cargo.toml` runtime dependency declarations, `cargo audit` status, and the still-open pre-commit hooks finding.

**Session note:** In-session with Layer 1 IAR suite. Acknowledged quality tradeoff.

---

### Resolved

**Finding 1 — `Cargo.lock` status (regression check from Review 2 Finding 1) (Dim 3)**

`Cargo.lock` exists (generated on first `cargo test`). It was not in the git index at the time of Review 2 but the current session confirms it must be committed. Re-confirming: the `Cargo.lock` will be staged and committed as part of the Layer 1 implementation commit.

**Resolution:** `Cargo.lock` staged for commit.

---

**Finding 2 — Runtime dependencies introduced without audit (Dim 3 — Dependency audit)**

Runtime dependencies added: `serde` 1.x, `serde_json` 1.x, `clap` 4.x, `chrono` 0.4. These are widely-used, well-maintained crates with large community audit coverage.

`cargo audit` run against the full `Cargo.lock` (100 packages): **0 vulnerabilities found**. The audit database was loaded from the RustSec advisory database. CI pipeline already enforces `cargo audit` on every push.

**Resolution:** No advisories. Dependency audit clean.

---

### Deferred

**Finding 3 — No pre-commit hooks (regression check from Review 2 Finding 2) (Dim 10)**

Still open. No pre-commit hook framework has been configured. This is a Layer 1 merge gate requirement.

Minimum requirements (unchanged from Review 2):
- Reject commits containing absolute home directory paths in committed files.
- Secret/credential detection (no keys, tokens, or passwords in committed files).

Recommended approach: A shell script in `.git/hooks/pre-commit` — the simplest option requiring no external tooling for a single-developer project. Alternatively, `pre-commit` framework with a YAML config if the developer prefers a declarative approach.

**Classification:** Deferred. Carried forward — still gates the Layer 1 merge. Pre-commit hooks cannot be implemented automatically — the framework selection is a human director decision. The developer must configure at minimum the absolute-path check before merging Layer 1.

---

### Dismissed

**Finding 4 — `Cargo.toml` runtime dependencies at semver-compatible ranges (Dim 3)**

`serde = { version = "1", features = ["derive"] }` — semver-compatible range. Same for others. For a portfolio project, this is the correct level of version pinning: `Cargo.lock` provides exact reproducibility; `Cargo.toml` allows compatible updates. Pinning to exact versions (`= "1.0.228"`) in `Cargo.toml` is over-specified.

**Classification:** Dismissed. Semver ranges in `Cargo.toml` with a committed `Cargo.lock` is the idiomatic Rust approach for binary crates.

---

### Hallucinated

*(none)*

---

### Open

*(none)*

---

### Summary

`cargo audit` clean: 0 vulnerabilities across 100 dependencies. `Cargo.lock` to be committed with Layer 1 implementation. Pre-commit hooks remain the one deferred gate item requiring human director action. No new platform issues from the runtime dependency additions.

**Coordination:** *(none)*

---

---

## Review 4 — 2026-04-30 00:00Z

**Scope:** Layer 1 gate closure — pre-commit hook configuration delivered. Evaluating Finding 3 resolution.

**Session note:** In-session with Layer 1 IAR suite. Acknowledged quality tradeoff.

---

### Resolved

**Finding 1 — Pre-commit hooks (regression check from Review 3 Finding 3) (Dim 10)**

Pre-commit framework configured at git root (`guild-portfolio/.pre-commit-config.yaml`). Hooks installed:
- `detect-private-key` — rejects staged files containing private key material
- `no-commit-to-branch` — prevents direct commits to `main`
- `no-home-dir-paths` (local) — rejects staged files containing `$HOME` resolved at runtime; no username hardcoded in any committed file

Hook script at `issue-tracker-cli/.pre-commit-hooks/check-no-home-paths.sh` uses `$HOME` at runtime. Verified: `pre-commit run --all-files` passed; hook correctly caught and prompted removal of two legacy occurrences of the developer's home directory path in `PLATFORM-ENGINEER-REVIEW.md` (review documentation example text). Both occurrences replaced with generic descriptions before commit. Final `grep -rn` scan confirmed zero occurrences of the local username across all tracked files.

Git history was subsequently rewritten with `git filter-repo --force` to remove a username occurrence from commit `f874a60`. Force-push to remote confirmed clean.

**Resolution:** Pre-commit hooks installed and verified.

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

Finding 1 closed. All platform findings are now resolved. The Layer 1 merge gate platform requirements are fully satisfied.

**Coordination:** *(none)*

---

---

## Review 5 — 2026-04-30 00:00Z

**Scope:** Layer 1 gate closure pass — no CI/build changes since Review 4. Manual testing session created `tracker.json` artifact.

**Session note:** In-session with all other domain reviews. Acknowledged quality tradeoff.

---

### Resolved

**Finding 1 — `tracker.json` not in `.gitignore` (Dim 8 — Artifact hygiene)**

Manual testing of the Layer 1 binary created `tracker.json` in `issue-tracker-cli/`. The file appeared as `??` in `git status`. The `.gitignore` only excluded `/target`; `tracker.json` was not listed. Any developer running `tracker` from the project directory accumulates this file as untracked and risks accidentally committing test data to the repository.

**Resolution:** Added `/tracker.json` to `issue-tracker-cli/.gitignore`. File is now gitignored and absent from `git status`.

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

One finding resolved: `tracker.json` added to `.gitignore`. CI pipeline, toolchain pin, `Cargo.lock`, and pre-commit hooks all remain in place and verified. All platform requirements satisfied. MVR reached for Layer 1.

**Coordination:** *(none)*

---

---

## Review 6 — 2026-05-01 00:00Z

**Scope:** Post-merge gap — `cargo fmt --check` enforced in CI but not locally via pre-commit hooks. CI failed on first PR with formatting violations that a local hook would have caught before push.

**Session note:** In-session, post-merge follow-up. Acknowledged quality tradeoff.

---

### Resolved

**Finding 1 — `cargo fmt --check` not enforced pre-commit (Dim 10)**

CI pipeline runs `cargo fmt --check` on every push. The pre-commit hook configuration did not include a corresponding local check. A formatting violation (`#[allow]` attribute and its trailing comment on the same line; two `assert!` calls exceeding line width) passed the pre-commit hook suite and reached CI, where it failed.

The CI failure is a correct catch, but the feedback loop is slower than local: push → CI trigger → wait → read failure → fix → push again. A pre-commit hook provides the same feedback in under one second without a round-trip.

**Resolution:** Added `cargo-fmt-check` hook to `.pre-commit-config.yaml`:

```yaml
- id: cargo-fmt-check
  name: cargo fmt (issue-tracker-cli)
  language: system
  entry: bash -c 'cd issue-tracker-cli && cargo fmt --check'
  pass_filenames: false
  files: ^issue-tracker-cli/.*\.rs$
```

The hook runs only when `.rs` files under `issue-tracker-cli/` are staged. `pre-commit run cargo-fmt-check --all-files` verified passing on current codebase.

**Resolution:** Hook added.

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

Finding 1 resolved: `cargo fmt --check` now runs as a pre-commit hook, closing the gap between local and CI formatting enforcement.

**Coordination:** *(none)*

---

---

## Review 7 — 2026-05-01 00:00Z

**Scope:** Layer 2 implementation — platform impact assessment. No new CI changes, no new dependencies, no toolchain changes.

**Session note:** In-session with full Layer 2 IAR suite. Acknowledged quality tradeoff.

---

### Dismissed

**Finding 1 — Two Layer 2 tests sleep 1 second each (Dim 1 — CI runtime)**

`status_change_refreshes_updated_at` and `status_idempotent_same_value_succeeds` each call `std::thread::sleep(Duration::from_secs(1))` to guarantee a different timestamp at second precision. This adds ≥2 seconds wall-clock to CI test runs.

**Classification:** Dismissed. The 1-second sleep is the minimum required to test timestamp-refresh behavior at second precision (ISO 8601 per spec). The alternative — mocking `current_timestamp()` — would require making the timestamp function injectable, adding implementation complexity beyond Phase 1 scope. The CI overhead is bounded and documented; accepted as a known limitation rather than a defect.

---

**Finding 2 — No new Cargo dependencies (Dim 3)**

Layer 2 added no runtime or dev-dependencies. `cargo audit` unchanged. ✓

**Classification:** Dismissed.

---

**Finding 3 — `cargo fmt --check` pre-commit hook active (Dim 10)**

The hook added in Platform Review 6 correctly catches formatting violations before push. Layer 2 additions pass `cargo fmt --check`. ✓

**Classification:** Dismissed. Hook functioning as expected.

---

**Finding 4 — `cargo clippy -- -D warnings` passes on Layer 2 additions (Dim 7)**

Verified: `cargo clippy -- -D warnings` produces no warnings on the Layer 2 implementation. `#![deny(clippy::unwrap_used)]` continues to enforce the no-unwrap policy.

**Classification:** Dismissed.

---

### Hallucinated

*(none)*

---

### Open

*(none)*

---

### Summary

No platform findings requiring action. CI pipeline, toolchain pin, `Cargo.lock`, and pre-commit hooks all unchanged and verified. One known limitation (1-second sleeps in timestamp tests) accepted under Dismissed Finding 1. MVR reached for Layer 2.

**Coordination:** *(none)*

---

---

## Review 8 — 2026-05-04

**Scope:** Layer 3 complete (commands: `create`, `list`, `status` with `--priority`). Cold-session adversarial pass against the full delivery platform: CI workflow, build configuration, dependency policy, pre-commit hooks, toolchain pinning. Layer 4 (labels) not started.

**Session note:** Cold session per primer; parallel batch run with other domains.

**Regression check:** `rust-toolchain.toml` still pins 1.94.1 with `clippy` + `rustfmt` (✓). `.gitignore` excludes `/target` and `/tracker.json` (✓ — `git ls-files` confirms `tracker.json` is untracked despite being present in the working tree). `Cargo.lock` is committed (✓). Pre-commit hooks (`detect-private-key`, `no-commit-to-branch`, `no-home-dir-paths`, `cargo-fmt-check`) still wired in `guild-portfolio/.pre-commit-config.yaml` (✓). CI workflow `.github/workflows/issue-tracker-cli.yml` still runs build/test/clippy/fmt-check/audit (✓).

---

### Open

**Finding 1 — `dtolnay/rust-toolchain@master` pinned to a moving branch (Dim 7 — Action/dependency pinning, Dim 13 — Supply chain integrity)**

`.github/workflows/issue-tracker-cli.yml:28` references `dtolnay/rust-toolchain@master`. `master` is a branch that the action author moves with every new release; any commit pushed to it is executed in CI on the next run with no review. The `with: toolchain: 1.94.1` argument pins the *Rust toolchain* installed by the action — it does not pin the *action code itself*. A compromised or buggy push to `master` runs in CI with full repository token scope.

The standard hardening for this exact action is to pin to a tagged commit SHA: `dtolnay/rust-toolchain@<sha>` (e.g., `@b3b07ba8b418998c39fb20f53e8b695cdcc8de1b # 1.94.1`). The action's README explicitly recommends SHA pinning for security-conscious consumers.

**Recommendation:** Replace `@master` with the commit SHA of a recent stable release, with the human-readable tag in a trailing comment. Apply the same pattern to `actions/checkout@v4` and `Swatinem/rust-cache@v2` (mutable major-version aliases — lower severity but the same supply-chain class).

**Classification:** Open. Defer fix to a Layer 4 platform sweep or apply now; recommendation provided.

---

**Finding 2 — No `cargo deny` / `deny.toml` (Rust supplement Platform Engineering — `cargo deny`)**

The Rust supplement explicitly states: "`cargo audit` alone is insufficient if `cargo deny` is not also present." A complete `deny.toml` configures `[advisories]`, `[licenses]`, `[bans]`, and `[sources]` — gating CVEs, license compliance, banned/duplicate crates, and disallowed registries simultaneously. CI currently runs only `cargo audit`, which covers the `[advisories]` section.

Concrete gaps left uncovered by `cargo audit` alone: (a) license drift — a transitive dependency could introduce a GPL-incompatible license with no signal; (b) duplicate-version blowup — `Cargo.lock` already shows multiple `anstyle*` and `clap*` family crates, no policy gating multiple major versions; (c) source restriction — a future dependency added from a non-crates.io git source would not trigger any warning.

**Recommendation:** Add `issue-tracker-cli/deny.toml` with at least the four standard sections; add a `cargo deny check` step to CI alongside `cargo audit`. For a portfolio project, an opinionated minimal `deny.toml` (allow MIT/Apache-2.0/BSD-*/Unicode-*; deny copyleft; warn-on-duplicate; allow only `crates.io`) is appropriate.

**Classification:** Open. Recommendation provided. This was not raised in Reviews 1–7.

---

**Finding 3 — No coverage measurement or threshold enforcement in CI (Dim 6 — Coverage thresholds, Rust supplement — Coverage enforcement)**

Review 1 deferred coverage to Layer 2 ("once enough tests exist to make a threshold meaningful"). Layer 3 is complete; tests now total 30+ across `tests/layer1.rs`, `tests/layer2.rs`, `tests/layer3.rs`, plus unit tests in `src/lib.rs`. The volume justification has been satisfied for two layers, yet no coverage tool is wired into CI. The Rust supplement explicitly requires: "Minimum 80% line coverage; 100% public API coverage. A CI run that measures coverage but does not fail below thresholds is not enforcement." The current state is worse: coverage is not even measured.

The bookmark-manager workflow in the same `.github/workflows/` directory does run `npm run test:coverage`, demonstrating the project family's existing precedent for coverage enforcement — the Rust workflow falls behind that bar.

**Recommendation:** Add a `cargo tarpaulin --out Xml --fail-under 80` (or `cargo llvm-cov --fail-under-lines 80`) step to CI. The 100% public API coverage requirement should be tracked as a separate audit (`pub fn` enumeration vs. test references) until tooling for it stabilizes.

**Classification:** Open. The Layer 2 deferral has aged out — Layer 3 is the natural deadline. Recommendation provided; ownership belongs to the human director.

---

**Finding 4 — Crate-level clippy lint configuration is far weaker than the Rust supplement's standard deny set (Rust supplement Software Engineering — Clippy lint configuration, Platform Engineering — `cargo clippy --deny warnings`)**

`src/lib.rs:1` declares only `#![deny(clippy::unwrap_used)]`. The Rust supplement specifies a standard baseline: `#![deny(clippy::all, clippy::pedantic, clippy::nursery, clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::missing_errors_doc, clippy::missing_panics_doc)]`. Any deviation from this baseline requires documented rationale; six of the eight items are absent, and there is no `DECISIONS.md` or inline comment justifying the weaker configuration.

CI runs `cargo clippy -- -D warnings`, which only escalates lints already at warning level by default. `pedantic`, `nursery`, `expect_used`, `panic`, and `missing_errors_doc` / `missing_panics_doc` are *not* in clippy's default-warn group. They are silently inactive in CI today. A user-facing panic added to a public function would not be caught by the current clippy gate.

**Recommendation:** Either (a) adopt the supplement's standard deny set in `src/lib.rs` (and `src/main.rs` if symmetric), or (b) document a rationale in `DECISIONS.md` for the weaker baseline. Selective `#[allow(...)]` with comments — already used in `save_issues` — is the correct escape hatch.

**Classification:** Open. Recommendation provided. Cross-domain coordination: this overlaps with Software Engineer review (it's their dim too) — flag it there if it has not been raised.

---

**Finding 5 — CI build and test do not use `--locked`, so `Cargo.lock` is advisory rather than authoritative (Dim 3 — Dependency installation)**

`.github/workflows/issue-tracker-cli.yml:39,42` runs `cargo build --release` and `cargo test` without `--locked`. If `Cargo.lock` falls out of sync with `Cargo.toml` (e.g., a developer edits `Cargo.toml` but doesn't run `cargo update`/`cargo build` locally before pushing), cargo silently regenerates `Cargo.lock` during the CI build to satisfy the manifest, then runs against the regenerated graph. The committed lock file becomes a recommendation, not a contract.

The deterministic install discipline that the supplement and Dim 3 require is "the lock file committed and the source of truth for installs." Without `--locked`, the lock file is committed but is not the source of truth in CI.

**Recommendation:** Add `--locked` to all `cargo build`, `cargo test`, `cargo clippy`, and `cargo audit` invocations in CI. `cargo install cargo-audit --locked` already uses this pattern (line 51); extend it to the rest.

**Classification:** Open. One-line fix per step; recommendation provided.

---

**Finding 6 — `cargo install cargo-audit --locked` is not version-pinned (Dim 7 — Action/dependency pinning, Dim 13 — Supply chain integrity)**

`.github/workflows/issue-tracker-cli.yml:51` installs `cargo-audit` at whatever the current published version is. A compromised or buggy `cargo-audit` release executes in CI with full token scope on the next run after publication. `--locked` only pins *cargo-audit's own dependencies* once a version is selected; it does not pin which version of `cargo-audit` to install.

**Recommendation:** Either pin the version (`cargo install cargo-audit --version "0.21" --locked`) or — preferred — switch to a pre-built pinned action like `rustsec/audit-check@<sha>`, which avoids the ~30-second compile-from-source step entirely and gives a single SHA to audit.

**Classification:** Open. Review 2 Finding 4 dismissed the *runtime* concern (caching mitigates) but did not address the *supply-chain* concern; this is a different finding. Recommendation provided.

---

**Finding 7 — Pre-commit hooks are bypassable with `--no-verify`, with no enforcement layer (Dim 10 — Pre-commit hooks)**

The standard dimension explicitly asks: "Evaluate whether hooks can be bypassed with `--no-verify` and whether bypass is logged or blocked." All four pre-commit hooks (`detect-private-key`, `no-commit-to-branch`, `no-home-dir-paths`, `cargo-fmt-check`) are `git commit --no-verify`-bypassable with no audit trail. The CI pipeline catches `cargo-fmt-check` after the fact; it does **not** catch `detect-private-key` (no scanning step in CI) or `no-home-dir-paths` (no scanning step in CI). A bypassed pre-commit hook for the home-dir or private-key checks lands directly in the repository with no second line of defense.

**Recommendation:** Add CI-side scanning for the secret/PII checks that pre-commit covers locally. Concrete options: (a) add `detect-secrets` or `gitleaks` as a CI step running against the diff or the full tree; (b) add a CI step that re-runs the `no-home-dir-paths` script over staged or all tracked files. The defense-in-depth model is: pre-commit catches early (fast feedback), CI catches reliably (no bypass).

**Classification:** Open. Cross-domain coordination: surface to Security review for input on the secret-scanning tool selection. Recommendation provided.

---

**Finding 8 — `.pre-commit-config.yaml` `cargo-fmt-check` hook hard-codes a sibling project directory and depends on the orchestrator's working directory (Dim 10 — Pre-commit hooks, Dim 5 — Cache correctness adjacent)**

`.pre-commit-config.yaml:21` runs `bash -c 'cd issue-tracker-cli && cargo fmt --check'`. This works only when `pre-commit` is invoked from `guild-portfolio/`. If the developer runs `pre-commit run --all-files` from `issue-tracker-cli/` (a natural place for that command), the `cd issue-tracker-cli` step fails silently (or worse, succeeds against a sibling directory that happens to share the name). The `files: ^issue-tracker-cli/.*\.rs$` regex is also pinned to that root-relative path; from any other working directory, it matches nothing.

This is a hidden coupling between the multi-project portfolio root and the per-project hook. The bookmark-manager project does not appear in `.pre-commit-config.yaml`, suggesting this was set up for issue-tracker-cli specifically; future projects added to the portfolio will need parallel entries with the same coupling.

**Recommendation:** Either (a) move `.pre-commit-config.yaml` into `issue-tracker-cli/` so the hooks are co-located with the project they protect (and remove the `cd issue-tracker-cli` shim), or (b) document the constraint that `pre-commit` must be run from `guild-portfolio/`. Option (a) generalizes better as the portfolio grows.

**Classification:** Open. Recommendation provided.

---

**Finding 9 — Coverage was deferred in Review 1, but no PE review has gated the deferral (process, Dim 6)**

Review 1 stated: "Coverage enforcement (`cargo tarpaulin`) to be added by Layer 2 once enough tests exist to make a threshold meaningful." Review 2 did not pick this up. Review 3 did not. Review 5 closed Layer 1 with "MVR reached." Review 7 closed Layer 2 with "MVR reached for Layer 2," asserting "no platform findings requiring action" — but the Layer 1 deferral targeted Layer 2 and was silently dropped. The deferral has aged for two layers without classification or re-deferral.

This is a process finding adjacent to the coverage finding (#3): the deferral mechanism failed to surface the carry-over. A deferred finding without a re-check at the targeted layer is indistinguishable from a forgotten finding.

**Recommendation:** When deferring a finding to a future layer, add a regression-check item to the next platform review's scope. The supplement and dim 6 both treat coverage as a baseline floor, not an optional enhancement.

**Classification:** Open. Process finding; resolution is procedural, not code.

---

### Dismissed

**Finding 10 — `cargo vet` not configured (Rust supplement Security — `cargo vet`)**

The supplement explicitly notes: "For personal portfolio projects this may be deferred; for any project with production users or sensitive data it is a finding if absent." This is a single-user local CLI portfolio project with no production users, no sensitive data, and a 100-package dependency graph dominated by widely-audited crates (serde, clap, chrono, serde_json).

**Classification:** Dismissed. The supplement's stated deferral criterion applies. If the project moves to production deployment or accepts contributions from outside parties, re-classify.

---

**Finding 11 — `[profile.release]` still uses defaults (Dim 7-adjacent)**

Re-confirmed dismissal from Review 2 Finding 3. No new performance budget; default profile is correct for a personal CLI.

**Classification:** Dismissed. Carrying forward Review 2's rationale.

---

### Hallucinated

*(none — all open findings have specific evidence in workflow YAML or source files; the `Cargo.lock`-without-`--locked` claim was verified against the workflow file directly)*

---

### Resolved

*(none — this is an adversarial pass; no fixes applied this session)*

---

### Summary

Cold-session pass found **9 open findings** and **2 dismissed**. The platform has the basic floor (toolchain pin, `Cargo.lock`, gitignore, pre-commit hooks, CI workflow with audit) but is missing several supplement-mandated controls: `cargo deny` + `deny.toml`, coverage enforcement, the standard clippy deny set, `--locked` on CI cargo invocations, version-pinned `cargo-audit` install, and SHA-pinned GitHub Actions. The pre-commit / CI defense-in-depth gap (Finding 7) and the orchestrator-coupled hook config (Finding 8) are real but lower-severity.

Two prior dismissals (Reviews 2 and 7) treated `cargo install cargo-audit` and missing-coverage concerns as resolved by acceptance; the supply-chain dimension of the former and the deferral-aging of the latter are surfaced as new findings here, not re-raises of resolved items.

**Coordination:**
- Finding 4 (clippy deny set) — surface to **Software Engineer review** (their supplement covers the same item from the source-code lens).
- Finding 7 (pre-commit bypass / CI secret scanning) — surface to **Security review** for tool selection input.
- Finding 8 (hook config orchestrator coupling) — surface to **Solution Architect review** if portfolio-multi-project structure becomes a recurring pattern.

---

### Update — 2026-05-04 16:00Z: Layer 3 follow-up resolution pass

Six of nine Open findings closed; three remain Open. See `CHANGELOG.md` § "Layer 3 follow-up: Open finding resolution pass" for the consolidated diff.

- **F1 (action SHA pinning) → Resolved.** `.github/workflows/issue-tracker-cli.yml` now pins `actions/checkout@34e114876b0b11c390a56381ad16ebd13914f8d5  # v4`, `dtolnay/rust-toolchain@3c5f7ea28cd621ae0bf5283f0e981fb97b8a7af9  # master at 2026-05-04`, `Swatinem/rust-cache@e18b497796c12c097a38f9edb9d0641fb99eee32  # v2`. Trailing comments document the resolved tag and the refresh procedure (`gh api repos/<owner>/<repo>/commits/<tag> --jq '.sha'`).
- **F2 (no `cargo deny` / `deny.toml`) → Resolved.** `deny.toml` added at the project root with all four supplement-required sections; new CI step `cargo deny --locked check` runs after `cargo audit`.
- **F3 (no coverage in CI) → Open (carry-forward).** Tool selection (`cargo-llvm-cov` vs `cargo-tarpaulin`) and threshold ratification not addressed this round; the Layer 2 deferral noted in F9 is still pending. Recommend landing as a single follow-up once a tool is chosen.
- **F4 (clippy deny set) → Resolved (partial).** Crate-level `#![deny(...)]` extended with `clippy::expect_used`, `clippy::panic`, `clippy::missing_errors_doc`. All public `Result`-returning functions now carry `# Errors` rustdoc sections. Skipped: `clippy::all`, `clippy::pedantic`, `clippy::nursery`, `clippy::missing_panics_doc` — these produce significant noise disproportionate to a Phase 1 portfolio scope; the choice and rationale are documented in CHANGELOG.md and DECISIONS.md is the next-write-target. Re-raise if a Layer 4+ refactor surfaces a defect that the skipped lints would have caught.
- **F5 (CI no `--locked`) → Resolved.** `cargo build --release --locked`, `cargo test --all-targets --locked`, `cargo clippy --all-targets --locked -- -D warnings`, and `cargo deny --locked check` all use `--locked`.
- **F6 (`cargo install cargo-audit` unpinned) → Resolved.** `cargo install cargo-audit --locked --version 0.22.1` and `cargo install cargo-deny --locked --version 0.19.4` — both pinned to current versions (refresh via `cargo search`).
- **F7 (pre-commit bypass / no CI-side scanning) → Open (carry-forward).** Defense-in-depth gap. Selecting a CI secret-scanning tool (e.g., `gitleaks`, `trufflehog`) is a Security-Platform joint decision and not closed this round.
- **F8 (`cargo-fmt-check` hook hard-coded `cd issue-tracker-cli`) → Resolved.** `.pre-commit-config.yaml` now uses `cd "$(git rev-parse --show-toplevel)/issue-tracker-cli"`; robust to invocation from any subdirectory.
- **F9 (process: coverage deferral silently dropped) → Open (carry-forward).** This finding remains Open until F3 is closed by a coverage tool selection decision. Tracked here so it is not silently re-dropped.

**Suite verification:** `cargo build --locked --all-targets`, `cargo test --all-targets --locked` (74/74), `cargo clippy --all-targets --locked -- -D warnings`, `cargo fmt --check` all clean locally. `cargo deny check` not validated locally (`cargo-deny` not installed on the dev machine); next CI run is the validation point.

---

### Update — 2026-05-05 11:30Z: SO Review 14 dispositions on F3 and F7

The two carry-forward Open findings have been adjudicated by SO Review 14 (`iterative-adversarial-refinement/SOLUTION-OWNER-REVIEW.md` Findings 5 and 6). Recording the dispositions here so the Platform log reflects final state.

- **F3 (no coverage in CI) → Backlogged by SO Review 14 Finding 5.** Defer until either (a) a layer adds substantial code without tests and the regression goes uncaught, (b) the project surface grows past ~1000 LOC, or (c) the project is submitted for external review where the absence of a coverage gate would itself be the finding. Procedural Red Gate discipline (tests written and confirmed failing before implementation, per layer) is sufficient at the current scope (~400 LOC source, ~1100 LOC tests, 84 total tests for 3 commands). Adding coverage tooling now would assert a property the procedural discipline already produces. The repeated Open status of this finding across Reviews 1/2/3/5/7/8 was a signal that the project doesn't actually need it yet — explicit Backlog with re-raise conditions is healthier than indefinite Open.
- **F7 (pre-commit bypass / no CI-side secret scanning) → Dismissed by SO Review 14 Finding 6.** The threat model excludes credentials by spec construction (DESIGN.md Constraints: "No network. No HTTP calls, no authentication, no external services."). There are no API keys, OAuth tokens, database credentials, or service-account JSON in this codebase, and the spec forbids the categories of feature that would introduce them. The existing pre-commit hooks (`detect-private-key`, `check-no-home-paths.sh`) cover the only realistic accidental-leak shape (SSH key copy-paste). Adding `gitleaks` or equivalent would be a CI step with zero expected catch rate — pure maintenance overhead.
- **F9 (process: coverage deferral silently dropped) → Resolved by F3's explicit Backlog.** The deferral is no longer silent; SO Review 14 records the decision and the re-raise conditions. The pattern of an Open finding floating across many reviews without SO veto is itself flagged in SO 14's Coordination section as guidance for the eventual closure-protocol document (VDD-IAR Review 10 F2).

**Net for Platform posture after Reviews 8 + 14:** all six other Open findings closed in the prior follow-up resolution pass (F1 action SHA pinning, F2 cargo-deny, F4 clippy deny set, F5 --locked, F6 tool version pinning, F8 hook cd-hardcode). F3 explicitly Backlogged with re-raise criteria; F7 explicitly Dismissed with re-raise criteria. No carry-forward Platform findings remain.

**Cross-domain coordination (from SO Review 14 Coordination section):** the recurring pattern of long-running Open findings across many reviews (F3 was Open across Reviews 1/2/3/5/7/8 before SO adjudicated) is a process datum. Recommend the closure protocol document include explicit guidance: a Raised-to-SO finding becomes Backlogged or Dismissed if SO does not adjudicate within N reviews. Otherwise the same indefinite-Open pattern recurs for the next agent-recommended-but-out-of-scope addition.

---

### Update — 2026-05-05 18:30Z: CI hotfix for self-crate license

**Context:** The first CI run after the Layer 3 follow-up resolution pass (which added `cargo deny --locked check`, F2) failed at the licenses step:

```
error[unlicensed]: tracker = 0.1.0 is unlicensed
 ├ tracker v0.1.0
```

`cargo deny`'s `[licenses]` allowlist (lines 39–50 of `deny.toml`) gates all crates in the dependency graph including the workspace crate itself. `Cargo.toml` had `description`, `readme`, and `publish = false` per TW Review 6 Finding 6, but no `license` or `license-file` — TW had explicitly raised that sub-item to SO and left a `TODO(SO)` comment in the manifest. The TW finding then sat Raised-to-SO across SO Reviews 10–14 without adjudication, and the new `cargo deny check` step surfaced it the moment it ran.

**Diagnostic note:** This was not introduced by Platform F2's resolution — F2 was the right addition. The latent gap was the missing `license` field; F2 simply gave it a CI-visible enforcement point. From a Platform-domain lens, this is the intended behavior of `cargo deny`: previously-invisible metadata holes become CI-blocking. Working as designed.

**Resolution (cross-domain):** SO Review 15 adjudicated the license decision, applying `license = "MIT OR Apache-2.0"` to `Cargo.toml`'s `[package]` section. The choice matches `deny.toml`'s existing allowlist, the Rust ecosystem norm, and TW Review 6 Finding 6's own proposal text. CI is expected green on the next push; `cargo-deny` is not installed on the dev machine, so the next CI run is the validation point.

**Other deny.toml notes from the failing run:**

- `warning[license-not-encountered]` for several allowlisted licenses (`Apache-2.0 WITH LLVM-exception`, `BSD-2-Clause`, `BSD-3-Clause`, `CC0-1.0`, `ISC`, `MPL-2.0`, `Unicode-DFS-2016`, `Zlib`) — informational. The current dependency tree does not pull any crate under these licenses. Not a finding: `deny.toml`'s allowlist is intentionally broader than the current need so a future dependency under one of these licenses does not produce a CI failure that requires an unrelated `deny.toml` edit. Tightening the allowlist to only currently-encountered licenses is a hygiene action available later if the warnings become noisy; not pursued this round.
- `confidence-threshold = 0.93` (line 51) — unchanged. The license-not-encountered warnings do not interact with this setting.

**Distribution-readiness flag (carry-forward, not Platform-owned):** the SPDX field declares the offer; the matching `LICENSE-MIT` and `LICENSE-APACHE` text files (required by both licenses' attribution clauses at distribution time) are not present. Not blocking, not pursued, flagged in SO Review 15 for revisit if external distribution is ever planned. SO and TW jointly own that decision; Platform notes it for completeness.

**Net Platform posture after this update:** unchanged from the post-Review-14 state — six prior F-numbers Resolved (F1, F2, F4, F5, F6, F8), F3 Backlogged, F7 Dismissed, F9 Resolved. The license fix is not a new Platform finding; it is an SO adjudication that closes a TW Open item that was CI-relevant once F2 landed. No carry-forward Platform findings remain.

---

## Review 9 — 2026-05-05 22:45Z

**Scope:** Layer 4 (labels) full-suite IAR pass on branch `issue-tracker-cli-labels`. Primary lens: Layer 4 platform impact. Secondary: regression check on every gate landed in Reviews 1–8 (toolchain pin, `Cargo.lock`, `--locked` CI invocations, SHA-pinned actions, version-pinned tool installs, `deny.toml` four-section coverage, pre-commit hooks, `Cargo.toml` `license` field).

**Session note:** Cold session per primer. Prior reviewer hit a rate limit before completing; this session starts fresh.

**Layer 4 diff (platform-relevant):** `git diff origin/main...HEAD --name-only` returns exactly three files — `src/lib.rs`, `src/main.rs`, `tests/layer4.rs`. Zero changes to `Cargo.toml`, `Cargo.lock`, `deny.toml`, `rust-toolchain.toml`, `.github/workflows/issue-tracker-cli.yml`, `.pre-commit-config.yaml`, or any file under `.pre-commit-hooks/`. Layer 4 introduced **no new dependencies, no toolchain change, no CI step change, and no hook change**.

**Regression check (verified this session):**

- `rust-toolchain.toml` still pins `channel = "1.94.1"` with `clippy` + `rustfmt` components (✓).
- `.gitignore` excludes `/target` and `/tracker.json` (✓).
- `Cargo.lock` is committed; `cargo build --locked` succeeds with no resolver complaint (✓).
- `Cargo.toml` carries `license = "MIT OR Apache-2.0"` (Review 8 hotfix, line 7) — still present (✓).
- `deny.toml` retains all four supplement-required sections — `[advisories]`, `[licenses]`, `[bans]`, `[sources]` — plus `[graph]` and `[output]` (✓).
- `.github/workflows/issue-tracker-cli.yml` still SHA-pins `actions/checkout@34e114876b…  # v4`, `dtolnay/rust-toolchain@3c5f7ea28c…  # master at 2026-05-04`, `Swatinem/rust-cache@e18b497796…  # v2`; all four cargo invocations carry `--locked`; `cargo install cargo-audit --locked --version 0.22.1` and `cargo install cargo-deny --locked --version 0.19.4` are version-pinned (✓).
- `.pre-commit-config.yaml`'s `cargo-fmt-check` hook still uses `cd "$(git rev-parse --show-toplevel)/issue-tracker-cli"` (Review 8 F8 fix) (✓).
- `.pre-commit-hooks/check-no-home-paths.sh` reads `$HOME` at runtime, no hardcoded username (✓).
- Local verification: `cargo build --locked` clean; `cargo clippy --all-targets --locked -- -D warnings` clean; `cargo fmt --check` clean. `cargo deny check` not run locally (`cargo-deny` not installed on dev machine, consistent with Reviews 8.1 and 8.4) — next CI run is the validation point. Per session brief, prior reviewers ran `cargo audit` returning exit 0 with 100 crates, no advisories — Layer 4 added zero crates so no re-audit needed.

---

### Open

*(none)*

---

### Resolved

*(none — no fixes applied this session; nothing to fix)*

---

### Deferred

*(none)*

---

### Dismissed

*(none — no findings raised this session)*

---

### Hallucinated

*(none — no findings raised this session)*

---

### Summary

**Layer 4 is platform-clean.** Zero findings, zero open items, zero regressions. Layer 4 (labels) was implemented as a pure source/test change — no new crates, no toolchain bump, no CI/workflow modification, no hook modification, no `deny.toml` policy change. Every gate installed by Reviews 1–8 (and the Review 8.4 license hotfix) remains in place and passes locally for the checks reproducible without `cargo-deny` installed.

The "no findings" outcome is itself the meaningful signal here: a feature layer that touches only `src/**` and `tests/**` should produce a no-op Platform review, and it did. Surfacing manufactured findings to fill space would be the sycophancy failure mode this domain is most prone to (rationalizing-to-completion). Confirmed "no findings" with explicit regression-check evidence above per the standard adversarial posture: the dimension list was walked; each item was either (a) verified unchanged from Review 8, or (b) verified inapplicable because Layer 4 made no platform-facing change.

Carry-forward state from prior reviews is unchanged: Review 8 F3 (coverage) remains Backlogged by SO Review 14 with explicit re-raise criteria; Review 8 F7 (CI secret scanning) remains Dismissed by SO Review 14 with explicit re-raise criteria. Layer 4 did not change the threat model or the LOC count past the F3 Backlog re-raise threshold (~1000 LOC source); both adjudications still hold.

**Merge-gate verdict:** No platform concerns blocking the Layer 4 merge. Recommend the `issue-tracker-cli-labels` branch proceed through the remaining IAR domains and merge-gate VDD-IAR Alignment without Platform-side gating.

**Coordination:** None required. No cross-domain raises this session.

---

## Review 10 — 2026-05-11 01:09Z

**Scope:** Layer 6 (description + show + delete) full-suite IAR pass on branch `issue-tracker-cli-compound-filtering` at commits `4fb5e67` (Red Gate) + `c91676a` (implementation). Primary lens: Layer 6 platform impact. Secondary: regression check on every gate landed in Reviews 1–9.

**Session note:** Cold session per primer. Read PE Reviews 8 and 9 plus the post-Review-14 adjudication block; ran every Dim-listed verification.

**Layer 6 platform-touched-files inventory:**

`git diff origin/main...HEAD --name-only` returns exactly four paths — `issue-tracker-cli/TODO.md`, `issue-tracker-cli/src/lib.rs`, `issue-tracker-cli/src/main.rs`, `issue-tracker-cli/tests/layer6.rs`. **Zero** platform-owned files touched: `git diff origin/main...HEAD -- Cargo.toml Cargo.lock rust-toolchain.toml deny.toml .github/workflows/issue-tracker-cli.yml .pre-commit-config.yaml issue-tracker-cli/.pre-commit-hooks/check-no-home-paths.sh` produces zero lines of diff. Layer 6 introduced **no new dependencies, no toolchain change, no CI step change, no `deny.toml` policy change, no hook change**.

**Dimension-by-dimension audit:**

- **Dim 1 — Build system changes:** None. Verified via the seven-path `git diff` above.
- **Dim 2 — Dependency surface:** None. `Cargo.lock` byte-identical to `origin/main`. No new crates pulled in; `cargo audit`'s "100 crate dependencies" matches Review 9's count exactly.
- **Dim 3 — Toolchain pinning:** `rust-toolchain.toml` still pins `channel = "1.94.1"` with `clippy` + `rustfmt` components. Unchanged.
- **Dim 4 — `cargo audit`:** `cargo audit` exit code 0; "Loaded 1068 security advisories"; "Scanning Cargo.lock for vulnerabilities (100 crate dependencies)"; no advisory output emitted. 0 advisories.
- **Dim 5 — Build verification (all four reproduced locally):**
  - `cargo build --locked --all-targets`: `Finished dev profile [unoptimized + debuginfo] target(s) in 0.14s` — clean.
  - `cargo test --locked`: 159/159 pass (48 unit + 32 layer1 + 18 layer2 + 9 layer3 + 25 layer4 + 7 layer5 + 20 layer6 + 0 doc-tests). Matches the c91676a commit message claim exactly.
  - `cargo clippy --all-targets --locked -- -D warnings`: clean, no output.
  - `cargo fmt --check`: clean, no output.
- **Dim 6 — Hooks:** Pre-commit hooks active and unmodified. The 4fb5e67 and c91676a commit messages do not contain `--no-verify`, `[skipped]`, or any "hook bypass" pattern. The two Layer 6 source files modified are `.rs` and trigger the `cargo-fmt-check` hook on the project; the fact that `cargo fmt --check` is locally clean confirms the hook would pass on these commits. `detect-private-key`, `no-commit-to-branch`, `no-home-dir-paths`, and the IAR `review-log-anonymization` hook all unchanged at `.pre-commit-config.yaml` rev `v5.0.0` / local script paths.
- **Dim 7 — CI compatibility / OS portability:** CI matrix runs `ubuntu-latest` only — single OS, no Windows or macOS leg. The PE Review 10 brief's specific concern (multi-line description test stability on Windows under `\r\n` normalization) is addressed at the implementation level (`src/lib.rs:365` `let normalized = d.replace("\r\n", "\n");`) and is **not** a CI concern at the matrix's current shape. Even if a Windows leg were added later, the `show_multiline_description_indents_continuation` test (`tests/layer6.rs:189–219`) passes a literal `"line1\nline2"` via `assert_cmd` which delivers it verbatim to the binary's argv (no shell intermediation, no CRLF transformation), and `assert_cmd::Output.stdout` is `Vec<u8>` captured raw with no newline mangling — the assertion `out.contains("\n             line2")` would hold on Windows because Rust's `println!` macro emits `\n` on every platform unless the binary explicitly opts into CRLF (it does not). No portability finding.

**Self-test via dismissal (per primer):**

I considered raising the absence of a Windows CI leg as a finding given Dim 7's framing in the brief. Dismissed: the SO-adjudicated portfolio scope (PE Review 9 carry-forward state, SO Review 14 dispositions) sets `ubuntu-latest`-only as deliberate; adding a Windows leg would be a Layer 8+ distribution-readiness item adjacent to the `LICENSE-MIT` / `LICENSE-APACHE` text-file gap flagged in Review 8.4. Not a Layer 6 regression. Also considered: does the `replace("\r\n", "\n")` mid-string normalization in `format_show_block` mishandle a lone `\r` (CR-only, classic-Mac line ending)? Dismissed: that line ending is effectively extinct, and a lone `\r` would render in show as a control char in the first line only — a presentation issue, not a Platform-domain concern. Surfaced to UX/SE if they choose to widen normalization.

### Open

*(none)*

### Resolved

*(none — no fixes applied this session; nothing to fix)*

### Deferred

*(none)*

### Dismissed

The "no Windows CI leg" and "lone `\r` normalization" concerns described in the self-test paragraph above are dismissed with stated reasoning; recording here for traceability per the primer's "dismissing without verification" failure mode.

### Hallucinated

*(none — no findings raised this session)*

### Summary

**Layer 6 is platform-clean.** Zero findings, zero open items, zero regressions. Layer 6 (description + show + delete) was implemented as a pure `src/**` + `tests/**` + `TODO.md` change — no new crates, no toolchain bump, no CI/workflow modification, no hook modification, no `deny.toml` policy change. Every gate installed by Reviews 1–9 remains in place and passes locally for the checks reproducible without `cargo-deny` installed on the dev machine (`cargo build --locked --all-targets`, `cargo test --locked` 159/159, `cargo clippy --all-targets --locked -- -D warnings`, `cargo fmt --check`, `cargo audit` exit 0 on 100 crates).

Carry-forward state from prior reviews is unchanged: Review 8 F3 (coverage) remains Backlogged by SO Review 14; Review 8 F7 (CI secret scanning) remains Dismissed by SO Review 14. Layer 6 did not move LOC past the F3 ~1000-LOC source re-raise threshold (the diff is +634 across two implementation commits, of which 465 lines are tests; current source LOC well under 1000) and did not change the threat model (still no network, still no credentials).

**Top concern:** None blocking. The closest thing to a concern is that the Layer 6 CRLF-normalization decision lives inside `format_show_block` (`src/lib.rs:365`) rather than as a shared utility; this is an SE-domain hygiene point, not PE. Surfaced here only as the Dim 7 audit trail.

**Merge-gate verdict:** No platform concerns blocking the Layer 6 merge. Recommend the `issue-tracker-cli-compound-filtering` branch proceed through the remaining IAR domains and merge-gate VDD-IAR Alignment without Platform-side gating.

**Coordination:** None required. No cross-domain raises this session.

---

## Review 11 — 2026-05-11 02:00Z

**Round:** Platform Engineer Review 11 (Round-2 closure for Layer 6)
**Scope:** Re-verify platform-clean state after Round-2 inline fixes commit `9b775f0`. Warm closure-verification.

### Round-2 platform impact

R10 reported 0 platform-touched files. R2 commit `9b775f0` adds 493 insertions / 35 deletions across 6 files — none of which are platform-owned. Verified via `git diff origin/main...HEAD --name-only`:
- `issue-tracker-cli/CHANGELOG.md` (doc)
- `issue-tracker-cli/DESIGN.md` (spec)
- `issue-tracker-cli/TODO.md` (doc, prior)
- `issue-tracker-cli/src/lib.rs` (source)
- `issue-tracker-cli/src/main.rs` (source)
- `issue-tracker-cli/tests/layer6.rs` (test, prior)
- `README.md` (portfolio doc)

Cargo.toml, Cargo.lock, rust-toolchain.toml, deny.toml, .pre-commit-config.yaml, .github/workflows/* all byte-identical to `origin/main`.

### Verifications re-run

- `cargo build --locked --all-targets` clean
- `cargo test --locked --no-fail-fast` 180/180 pass
- `cargo clippy --all-targets --locked -- -D warnings` clean
- `cargo fmt --check` clean
- `cargo audit` clean (0 advisories)

### New findings

*(none this round.)*

### Summary

Layer 6 R2 is platform-clean. Cargo.lock crate count unchanged. No CI changes, no dep changes, no toolchain changes, no hook changes.

**Coordination:** None required. **Merge-gate verdict (PE):** No platform concerns blocking Layer 6 merge.

---

## Review 12 — 2026-05-11 22:30Z

**Round:** Platform Engineer Review 12 (Layer 7 Round 1)
**Scope:** Layer 7 (polish — `--help`, TTY color, error specificity) full-suite IAR pass on branch `issue-tracker-cli-polish` at commits `7b461aa` (Red Gate), `a2b8062` (implementation), `603c689` (manual closure). Primary lens: Layer 7 platform impact, with explicit pressure on MSRV declaration since `a2b8062` introduces `std::io::IsTerminal` (stable in Rust 1.70). Secondary: regression check on every gate landed in Reviews 1–11. Cold session.

### Regression check (verified this session)

- `rust-toolchain.toml` still pins `channel = "1.94.1"` with `clippy` + `rustfmt` components (line 2-3) — unchanged.
- `Cargo.lock` package count = 100 — byte-identical to `main`. Layer 7 added zero crates (verified by `git diff main..HEAD -- issue-tracker-cli/Cargo.toml issue-tracker-cli/Cargo.lock` returning empty).
- `Cargo.toml` still carries `license = "MIT OR Apache-2.0"` (line 7), `description`, `readme`, `repository`, `publish = false`.
- `deny.toml` still has all four required sections — `[advisories]`, `[licenses]`, `[bans]`, `[sources]` — plus `[graph]` and `[output]`. Unmodified.
- CI workflow (`.github/workflows/<project>.yml`) still SHA-pins `actions/checkout` (v4), `dtolnay/rust-toolchain`, `Swatinem/rust-cache` (v2); all four cargo invocations carry `--locked`; `cargo install cargo-audit --locked --version 0.22.1` and `cargo install cargo-deny --locked --version 0.19.4` remain version-pinned.
- `.pre-commit-config.yaml`'s `cargo-fmt-check` hook still uses `cd "$(git rev-parse --show-toplevel)/<project>"` (R8 F8 fix preserved).
- `.pre-commit-hooks/check-no-home-paths.sh` reads `$HOME` at runtime; no hardcoded username.

### Pipeline gate verification (Layer 7 commits, local)

- `cargo test --no-fail-fast --locked` — **195/195 pass** (62 unit + 32 layer1 + 18 layer2 + 9 layer3 + 25 layer4 + 7 layer5 + 33 layer6 + 9 layer7 + 0 doc-tests). Layer 7 adds 9 new tests in `tests/layer7.rs` (Red Gate Phase 2a).
- `cargo clippy --all-targets --locked -- -D warnings` — clean (no output beyond `Finished` line).
- `cargo fmt --check` — clean (exit 0).
- `cargo audit` — clean: 1069 advisories loaded; 100 crate dependencies scanned; **0 advisories**.

**Tests:** 195/195 pass; clippy clean; fmt clean; audit 0 advisories.

### Layer 7 platform-touched-files inventory

`git log main..HEAD --stat` shows three commits touching exactly four paths:
- `issue-tracker-cli/CHANGELOG.md` (doc; layer summary entry)
- `issue-tracker-cli/TODO.md` (doc; checklist ticking)
- `issue-tracker-cli/src/lib.rs` (+111 -8; color helpers, `IsTerminal` usage)
- `issue-tracker-cli/tests/layer7.rs` (+193 net new)

**Zero** platform-owned files (per CLOSURE-PROTOCOL §1: `deny.toml`, `.github/workflows/*`, `.pre-commit-config.yaml`, `.pre-commit-hooks/*`) touched. **Zero** dependency-graph mutations (`Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml` byte-identical to `main`). The "raw ANSI vs. `anstyle`" decision (documented at `src/lib.rs` color-helper preamble, lines 40-44) holds dependency surface flat — applauded as deliberate dep-minimization, not flagged.

### Dimension-by-dimension audit

- **Dim 1 (pipeline completeness):** CI runs build, test, clippy, fmt-check, cargo-audit, cargo-deny. No coverage gate (carry-forward F3 from R8, SO-Backlogged R14). Layer 7 does not move source LOC past the ~1000-LOC re-raise threshold; F3 stays Backlogged.
- **Dim 2 (gate enforcement):** CI is `on: pull_request` to `main` + `on: push: branches: ['**']` — every gate runs on every push to the polish branch. Branch protection enforcement (whether all six steps are *required*-to-merge on `main`) is a repository setting not visible from the working tree; the workflow file alone cannot confirm required-status-check configuration. **Flagging as F1 (open question).**
- **Dim 3 (deterministic install):** `--locked` on every cargo invocation. Unchanged.
- **Dim 4 (environment pinning):** Toolchain pinned via `rust-toolchain.toml` (1.94.1) + workflow `toolchain: 1.94.1` (explicit match). Tool versions pinned. Unchanged.
- **Dim 5 (cache correctness):** `Swatinem/rust-cache@<sha>` keys on `Cargo.lock` by default; Layer 7 did not touch `Cargo.lock`, so cache reuses cleanly. Unchanged.
- **Dim 6 (coverage):** Not enforced. Carry-forward.
- **Dim 7 (action pinning):** All three actions SHA-pinned with tag comment. Refresh date noted in workflow (master at 2026-05-04). Acceptable.
- **Dim 8 (artifact hygiene):** `/target` ignored; `/tracker.json` ignored. Confirmed.
- **Dim 9 (left-shift opportunities):** Layer 7's manual TTY-color verification step (TODO.md L368-374, "manual only") is a documented inability to automate TTY-positive rendering in a subprocess test. The piped-negative branch is automated by `list_piped_has_no_ansi_codes` and `show_piped_has_no_ansi_codes`. The remaining manual step (TTY-positive color rendering) could in principle be left-shifted via a pseudo-TTY harness (`ptyprocess`, `expect`-style); the cost is meaningful for a single-user portfolio CLI and not pursued. Noted, not flagged.
- **Dim 10 (pre-commit hooks):** Hook set unchanged. Layer 7 touched `*.rs` files; `cargo-fmt-check` hook ran during the `a2b8062` commit (per session brief — "fired during the Phase 2a commit"). Fire-event is the hook working as intended, not a regression. **However**, hook coverage of the new Layer 7 surface deserves a closer look — see F2 below.
- **Dim 11 (security scanning):** `cargo audit` + `cargo deny` both in CI. Confirmed.
- **Dim 12 (secrets):** No secrets in code; spec forbids the category (DESIGN.md Constraints "No network"). Unchanged.
- **Dim 22-33 (observability):** Single-user local CLI; no logging surface, no metrics, no alerting, no health checks. Inapplicable by deployment context per the standard PE review framing. Verified `src/lib.rs` and `src/main.rs` for accidental observability additions: error path emits `Error: <msg>` to stderr and exits 1 (Dim 27 error-surfacing); no `eprintln!` swallowed-error pattern; no log crates added. Inapplicability is genuine, not rationalized.

### Pressure point: MSRV declaration

The session brief explicitly directs pressure on `Cargo.toml`'s missing `rust-version` field. `a2b8062` introduces `std::io::IsTerminal` at `src/lib.rs:25` and uses `stdout().is_terminal()` for the TTY-detection branch. `IsTerminal` has been stable in `std::io` since **Rust 1.70.0** (released 2023-06-01). The project's `rust-toolchain.toml` pins `1.94.1`, so the local + CI build is fine; but `Cargo.toml` has **no `[package].rust-version` declaration**. The relevance:

1. `rust-toolchain.toml` controls the *toolchain* used when this checkout is built. `rust-version` declares the *minimum supported Rust version* in package metadata — it is checked by `cargo` (since 1.56) and is the canonical answer to "what Rust version does this crate require?" for any consumer (including downstream tooling that reads `Cargo.toml` directly, e.g. cargo-msrv, cargo-info, dependabot's compat hints, distro packagers).
2. The package is `publish = false`, which mitigates the crates.io-consumer risk (no published metadata to misalign). But the field is still the spec contract for "what is this crate's MSRV?" — and the repository ships a `Cargo.toml` that asserts no minimum at all, even after a commit that demonstrably raises the floor to 1.70.
3. Searching `iterative-adversarial-refinement/` for prior `rust-version` / MSRV discussion: SE Review log L1341 / L1461 and QE Review log L1180 reference "MSRV is satisfied" / "(stable since Rust 1.82, present in this MSRV per `Cargo.toml`)" — that QE phrasing is **factually wrong**: `Cargo.toml` declares no MSRV. The QE reviewer inferred MSRV from `rust-toolchain.toml`. The PE log itself has no prior raise of `rust-version` absence — this is the **first PE-domain raise**, despite the floor having moved across multiple layers.

This is a real Platform finding, not a stylistic preference. **F2 below.**

### Pressure point: pre-commit hook coverage of the new surface

`.pre-commit-config.yaml` runs four hooks: `detect-private-key`, `no-commit-to-branch` (blocks direct commits to `main`), `no-home-dir-paths` (custom), `review-log-anonymization` (custom), and `cargo-fmt-check` (custom; `files: ^<project>/.*\.rs$`).

Layer 7's new code in `src/lib.rs` includes raw ANSI escape literals (`\x1b[1;31m`, `\x1b[33m`, etc.) as `&'static str` constants. These are not credentials, not PII, not home-dir paths, not absolute paths — none of the hook patterns match them, correctly. The cargo-fmt-check hook ran and presumably passed (file is fmt-clean per local verification).

The `no-commit-to-branch` hook is configured `args: ['--branch', 'main']`; the polish branch is not `main`, so commits to the branch are permitted — working as designed. No hook bypass observed in the three commit messages (no `--no-verify`, no `[skipped]` markers).

One genuine question: is there a hook covering `cargo clippy` or `cargo test` so that CI is not the first stop where a clippy-warning regression in `*.rs` shows up? Currently only `cargo fmt --check` runs pre-commit. **F3 below — left-shift opportunity.**

### Pressure point: CI YAML coverage of the new layer7.rs

`cargo test --all-targets --locked` (workflow line 48) discovers test binaries via Cargo's `tests/*.rs` convention; `tests/layer7.rs` is picked up automatically with no workflow edit needed. Locally verified: `cargo test` ran 9 layer7 tests as a separate binary, alongside layer1-layer6. CI coverage of the new tests is **complete and automatic** — no finding.

### Pressure point: no new deps + cargo-deny posture

Layer 7 added zero deps (verified above). `cargo-deny` re-runs in CI against the unchanged `Cargo.lock`. Advisory DB is fetched on each run (`db-path = "~/.cargo/advisory-db"`; `db-urls = ["https://github.com/rustsec/advisory-db"]` in `deny.toml` L23-24). **Rolling baseline accepted by design** — fetching the latest advisory DB is the right posture for a tracker that sits on a developer machine and gets re-run sporadically; pinning a stale baseline would mask freshly-disclosed CVEs. Trade-off: a sudden advisory landing in `rustsec/advisory-db` between commits can fail an otherwise-clean CI run. Acceptable for a portfolio project; would be re-evaluated for any application that needs CI determinism guarantees across long windows. **Not a finding.**

### Open

#### F1 — Branch-protection / required-status-check verification is outside the working tree

`gate enforcement` (Dim 2) requires that all CI gates be required-to-merge to `main`. The workflow YAML lists six gates (build, test, clippy, fmt, audit, deny) but **the workflow file alone cannot confirm whether GitHub's branch-protection rules mark these as required checks**. If only some gates are required, a flaky run on a non-required gate can be merged around; if branch protection is absent entirely, a direct push to `main` could bypass all of them (mitigated by the `no-commit-to-branch` pre-commit hook on the local machine, but not by CI).

**Evidence:** workflow file shows the gates exist; no artifact in this checkout demonstrates required-status-check configuration on the `main` branch.

**Recommendation:** Confirm (via `gh api repos/<owner>/<repo>/branches/main/protection` or repository Settings → Branches) that all six CI steps are marked required-status-checks for merge to `main`. If not, configure them. If branch protection is not enabled at all, enable it with `Require status checks` + `Require pull request before merging` + `Restrict who can push to matching branches`.

**Classification:** Open. Requires action outside the working tree.

#### F2 — `Cargo.toml` declares no `rust-version` / MSRV; floor raised to 1.70 this layer

`a2b8062` adopts `std::io::IsTerminal` (stable since Rust 1.70.0), raising the package's effective minimum Rust version to 1.70. `Cargo.toml` (verified L1-32) has no `[package].rust-version` key. The de-facto MSRV is now ≥ 1.70, but the manifest does not record it.

**Why this matters even though `publish = false`:**

- The QE log already contains the incorrect inference "(stable since Rust 1.82, present in this MSRV per `Cargo.toml`)" — a reviewer reading `Cargo.toml` for MSRV would find nothing, and the misinference propagated.
- `cargo` itself uses `rust-version` to provide actionable errors when a consumer's toolchain is too old. Without it, a future contributor on Rust 1.69 would see an opaque "`is_terminal` not found in `std::io`" rather than "this crate requires Rust 1.70+".
- The field is metadata, not enforcement — adding it is one line. The cost is zero; the documentation value is real.

**Recommendation:** add `rust-version = "1.70"` (the actual floor introduced by this layer's `IsTerminal` usage) to `[package]` in `Cargo.toml`. Alternative: `rust-version = "1.94"` to match `rust-toolchain.toml` if the project's intent is "build with the pinned toolchain only" — but that conflates "toolchain pin for reproducibility" with "minimum supported Rust", which are different concepts. The 1.70 value is the correct semantic for "minimum required to compile".

**Prior raise check:** searched `iterative-adversarial-refinement/PLATFORM-ENGINEER-REVIEW.md` and `**/CHANGELOG.md` for `rust-version` / `msrv` / `IsTerminal`; no prior PE raise. This is genuinely a new floor introduced by Layer 7, not a re-raise.

**Classification:** Open. Single-line fix; PE-domain authority (`Cargo.toml` "Platform Engineer (CI metadata)" per CLOSURE-PROTOCOL §1, though `[package]` metadata is ambiguous between SE and SO authority for license-class fields). Defensible for PE to apply in a Round-2 closure pass per session-brief authority statement; will note coordination with SE/SO before applying if Round-2 fires.

#### F3 — Pre-commit gate covers fmt but not clippy or test; CI is the first stop for either

`.pre-commit-config.yaml` currently runs `cargo fmt --check` on `*.rs` changes (registered as the `cargo-fmt-check` hook) but **does not run `cargo clippy` or `cargo test`** locally. A developer can land a commit that fails clippy or breaks a test, and CI is the first signal — which round-trips through GitHub Actions queueing time.

**Left-shift framing:** the standard PE Dim 9 lens. The cargo-fmt-check hook fires per-commit and is reportedly fast (per session brief: "fired during the Phase 2a commit"); a `cargo clippy --all-targets --locked -- -D warnings` hook would catch the same regressions clippy catches in CI, locally, in a few seconds on cached state. `cargo test --locked` is more expensive (subprocess tests + 195 cases) and may not be worth a pre-commit gate; clippy is the higher-value addition.

**Counter-argument considered:** pre-commit overhead has UX cost; a 5-10s clippy run per commit may discourage small commits or get bypassed with `--no-verify`. The current four-hook config is intentionally minimal. Reasonable to leave as-is for a single-user portfolio project; would be reconsidered for any project with multi-contributor onboarding cost.

**Recommendation:** Consider adding a `cargo-clippy-check` hook mirroring the `cargo-fmt-check` shape (same `cd "$(git rev-parse --show-toplevel)/<project>"` form, `files: ^<project>/.*\.rs$`, `entry: bash -c '... && cargo clippy --all-targets --locked -- -D warnings'`). If declined, document the decline rationale in PROCESS.md or CLOSURE-PROTOCOL so a future PE review does not re-raise.

**Classification:** Open. Left-shift opportunity; defer-or-implement decision is PE-authority + director judgment.

### Resolved

*(none — no fixes applied this session)*

### Deferred

*(none — F1/F2/F3 are Open pending Round-2 action or director decision)*

### Dismissed

- **"Raw ANSI vs. `anstyle` dependency"** — considered as a finding (supply-chain hygiene: a vetted color crate is more robust than hand-rolled escapes). Dismissed: the six ANSI sequences in use (`\x1b[1;31m`, `\x1b[33m`, `\x1b[36m`, `\x1b[32m`, `\x1b[0m`) are VT100-universal; the target environment is "developer terminal on macOS / Linux" per DESIGN.md; `anstyle` would add ~3 transitive crates and zero functional benefit for this surface; the project's dependency-minimization posture is explicit (`src/lib.rs` color-helper preamble L40-44). Working as designed; dep-minimization is itself the platform-relevant decision.
- **"Rolling `cargo audit` advisory-db baseline"** — considered as a finding (CI determinism: a freshly-published advisory between two commits can fail a clean tree). Dismissed: for a portfolio CLI with sporadic re-runs, rolling-baseline is the correct posture (a stale baseline would mask new CVEs in transitive deps); the trade-off is documented above. Acceptable.
- **"No Windows / macOS CI leg"** — considered, dismissed for the same reason as R10's identical self-test: `ubuntu-latest`-only is the SO-adjudicated portfolio scope per R9 / R10 / R14 carry-forward. Layer 7's TTY-detection code uses cross-platform `std::io::IsTerminal` (returns `false` for piped on every platform); no Windows-specific code path added.

### Hallucinated

*(none — no findings retracted as adversary-invented)*

### Summary

Layer 7 is functionally platform-clean on the four-gate cold-batch: 195/195 tests pass, clippy clean, fmt clean, audit 0 advisories. **Three Open findings** surface real platform gaps that prior reviews did not raise:

- **F1** (branch-protection enforcement outside the working tree) is the strongest substantive finding — the workflow exists, but whether all six gates are required-status-checks for merge to `main` is a repository-setting question that cannot be answered from the checkout. If branch protection is misconfigured, every prior PE review's "merge-gate verdict: no platform concerns" is conditional on a control that has never been verified.
- **F2** (missing `rust-version` MSRV declaration) is a genuine new gap introduced by Layer 7's `IsTerminal` adoption. The QE log's incorrect "stable since Rust 1.82, present in this MSRV per `Cargo.toml`" inference demonstrates that the absence already misled one reviewer. One-line fix; PE authority over `Cargo.toml` CI metadata.
- **F3** (clippy not in pre-commit hooks) is a defensible-either-way left-shift opportunity; raising as Open rather than Dismissed because the session brief explicitly pressed on hook coverage and the answer "fmt is covered, clippy and test are CI-only" is a real-but-acceptable gap that warrants explicit adjudication rather than silent acceptance.

The "no new deps + raw ANSI" choice is applauded as deliberate dependency minimization, not flagged. CI YAML coverage of `tests/layer7.rs` is automatic via the `--all-targets` flag; no finding there. `cargo deny` rolling-baseline posture is correct for this project's scope; no finding.

**Sycophancy-check self-audit:** the session brief warned that "all four gates pass + no new deps" can soften the reviewer. I pressed on MSRV (F2), hook coverage (F3), and branch protection (F1) precisely because those are the dimensions that a green-pipeline-pass + clean-diff state most easily rationalizes away. F2 in particular is a finding that would be invisible to a reviewer who reads `rust-toolchain.toml` and stops there — the manifest is the canonical answer to "MSRV?" and the manifest is silent. That silence is the finding.

**Cross-domain coordination flags:**

- **QE** — the SE/QE log inference "stable since Rust 1.82, present in this MSRV per `Cargo.toml`" should be corrected after F2 lands; the inference was MSRV-from-`rust-toolchain.toml`, not from `Cargo.toml`. QE log L1180 cited specifically.
- **SO** — F2's `rust-version` value (1.70 vs. 1.94) is partially a scope question (what MSRV does this project commit to supporting?); raising to SO for the value, while PE applies the field. If the SO call is "no MSRV commitment, this is `publish = false`", F2 closes with that rationale documented. PE owns the file; SO owns the policy.
- **SE** — F3 (pre-commit clippy hook) overlaps `src/**` authority because the hook would block SE commits that fail clippy. SE concurrence appropriate before adding.
- **VDD-IAR Alignment** — F1's "verify branch protection" task lives outside the working tree, which is the same shape as VDD-IAR Review 10 F1 (authority-record violation invisible until process review). Recommend VDD-IAR's next pass include a "branch protection configuration matches CI workflow gate set" check.

**Carry-forward state:** R8 F3 (coverage in CI) remains Backlogged by SO R14 — Layer 7 added 111 net source lines (+103 if you discount the inline doc comments); source LOC remains under the ~1000-LOC re-raise threshold. R8 F7 (CI secret scanning) remains Dismissed by SO R14 — Layer 7 introduced no credential surface. Neither carry-forward Open is reactivated.

**Coordination:** F1 outside-tree verification → SO + VDD-IAR Alignment. F2 `rust-version` value choice → SO; field application → PE. F3 clippy pre-commit hook → SE concurrence before PE applies. No new IAR domain proposed.

**Merge-gate verdict (PE, Layer 7 R1):** Three Open findings; none block functional merge, but F1 (branch protection) and F2 (`rust-version`) warrant Round-2 closure action before the Layer 7 merge per CLOSURE-PROTOCOL §6 criterion 3 ("No finding remains in Open state"). F3 may close via Round-2 implementation or via SO/director rationale documenting decline.

