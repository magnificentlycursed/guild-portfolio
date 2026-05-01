# Platform Engineer Review Log

This review is part of the [Iterative Adversarial Refinement (IAR)](README.md) suite. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

The purpose of this review is to evaluate the delivery platform: CI/CD, build configuration, DevSecOps practices, and toolchain setup. At pre-implementation stage, the review establishes what must be in place before Layer 1 begins.

**Language supplement applied:** `lang/rust.md` (Platform Engineering section).

---

## Review 1 — 2026-04-27 21:00Z

**Scope:** Build configuration requirements. No source code, Cargo.toml, or CI configuration exists yet. Pre-implementation pass: identifying platform requirements for Layer 1 setup.

**Session note:** In-session with all other domain reviews. Acknowledged quality tradeoff.

**Applicability note:** Many standard PE dimensions (cloud infrastructure, containerization, observability dashboards, disaster recovery) do not apply to a local single-user CLI binary with no deployment infrastructure. These are dismissed below without individual findings. The review focuses on CI/CD, build tooling, and DevSecOps — the dimensions relevant to a Rust binary project.

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

### Deferred

---

**Finding 4 — No `Cargo.lock` in version control (Dim 3 — Dependency installation)**

For binary crates (as opposed to libraries), `Cargo.lock` must be committed to ensure reproducible builds. Without it, `cargo build` resolves dependencies non-deterministically and two builds from the same source may produce different binaries.

**Classification:** Deferred to Layer 1. `Cargo.lock` must be committed alongside the initial `Cargo.toml`.

---

**Finding 5 — No pre-commit hooks (Dim 10 — Pre-commit hooks)**

No pre-commit hook configuration exists. For a Phase 1 portfolio project, hooks should cover at minimum: absolute path leakage (local usernames in committed files), and secret/credential detection if any keys are introduced.

**Classification:** Deferred to Layer 1. Pre-commit hooks are a requirement for portfolio projects intended for external review.

---

### Dismissed

**Infrastructure, containerization, observability, disaster recovery (Dims 16–33)** — Not applicable. The project is a local CLI binary with no deployment infrastructure, no cloud resources, no container runtime, and no operational environment. These dimensions do not apply to this deployment context.

**Performance budget, time-to-interactive (Dims 34–37)** — Not applicable. CLI binary with no browser, no asset pipeline, no network latency.

---

### Open

*(none — all findings deferred)*

---

### Summary

Three findings resolved: `rust-toolchain.toml`, `.gitignore`, and CI pipeline created. Two findings remain deferred to Layer 1: `Cargo.lock` (requires Cargo.toml first) and pre-commit hooks (requires a hook framework decision). Coverage threshold enforcement deferred to Layer 2 when test volume makes a threshold meaningful.

---

---

## Review 2 — 2026-04-27 22:00Z

**Scope:** Layer 1 build configuration — `Cargo.toml`, `Cargo.lock` status, CI configuration. Evaluating delivery of the two deferred findings from Review 1.

**Session note:** In-session with all other Layer 1 domain reviews. Acknowledged quality tradeoff.

---

### Resolved

**Finding 4 (from Review 1) — `Cargo.lock` not in version control**

`Cargo.toml` now exists. `cargo test` was run, generating `Cargo.lock` with 66 packages locked (all from dev-dependencies: assert_cmd, predicates, serde_json, tempfile and their transitive dependencies). The `Cargo.lock` file is not in `.gitignore` and must be committed alongside the Layer 1 Red Gate commit.

**Resolution:** `Cargo.lock` must be staged and committed with the Layer 1 Red Gate files. This is a gate requirement for the Red Gate commit, not the merge gate. **Action item for the current commit.**

---

### Open

**Finding 5 (from Review 1) — No pre-commit hooks (Dim 10)**

Status: Still open. No pre-commit hook framework has been configured. For a portfolio project intended for external review, hooks covering local username leakage and secret detection are required.

**Classification: Open.** The framework decision (lefthook, pre-commit, husky, or a shell script in `.git/hooks`) is a human director decision. The minimum requirements:
- Reject commits that include absolute paths containing the developer's local home directory path in committed files. This prevents local machine-specific paths from appearing in public portfolio code.
- The CI pipeline already runs `cargo fmt --check` and `cargo clippy` — pre-commit hooks for these are optional if the developer is disciplined about running them locally.

**Gate:** This remains a Layer 1 merge gate requirement. Cargo.lock commit (Finding 4) unblocks the Red Gate commit; pre-commit hooks unblock the merge gate.

---

### Dismissed

**Finding 6 — `Cargo.toml` has no `[profile.release]` section (Dim 7)**

No release profile optimization settings are declared. The default release profile (`opt-level = 3`, no debug info) applies.

**Classification:** Dismissed. The default release profile is appropriate for a personal CLI tool. LTO, codegen-units, and strip settings are production optimization concerns. The assignment has no performance budget for the binary; the default profile is correct for this stage.

---

**Finding 7 — CI `cargo audit` step installs `cargo-audit` at CI runtime (Dim 1)**

The workflow runs `cargo install cargo-audit --locked` on every CI run. This adds ~30 seconds to the CI runtime if not cached.

**Classification:** Dismissed. `Swatinem/rust-cache@v2` is configured in the CI pipeline. `cargo install` artifacts are cached between runs on matching toolchain/dependency fingerprints. The first run pays the install cost; subsequent runs use the cache. Acceptable for a portfolio project. Using a pre-built action (e.g., `rustsec/audit-check`) would be an optimization but is not required.

---

### Summary

Finding 4 resolved: `Cargo.lock` must be committed with the current Red Gate commit. Finding 5 remains open and gates the Layer 1 merge. One action item: commit `Cargo.lock` now.

---

---

## Review 3 — 2026-04-28 05:30Z

**Scope:** Layer 1 implementation — build configuration with runtime dependencies added. Evaluating `Cargo.toml` runtime dependency declarations, `cargo audit` status, and the still-open pre-commit hooks finding.

**Session note:** In-session with Layer 1 IAR suite. Acknowledged quality tradeoff.

---

### Resolved

**Finding 4 (from Reviews 1–2) — `Cargo.lock` status**

`Cargo.lock` exists (generated on first `cargo test`). It was not in the git index at the time of Review 2 but the current session confirms it must be committed. Re-confirming: the `Cargo.lock` will be staged and committed as part of the Layer 1 implementation commit.

**Classification:** Resolved. `Cargo.lock` staged for commit.

---

**Finding 8 — Runtime dependencies introduced without audit (Dim 3 — Dependency audit)**

Runtime dependencies added: `serde` 1.x, `serde_json` 1.x, `clap` 4.x, `chrono` 0.4. These are widely-used, well-maintained crates with large community audit coverage.

`cargo audit` run against the full `Cargo.lock` (100 packages): **0 vulnerabilities found**. The audit database was loaded from the RustSec advisory database. CI pipeline already enforces `cargo audit` on every push.

**Classification:** Resolved. No advisories. Dependency audit clean.

---

### Open

**Finding 5 (from Reviews 1–2) — No pre-commit hooks (Dim 10)**

Status: Still open. No pre-commit hook framework has been configured. This is a Layer 1 merge gate requirement.

**Minimum requirements (unchanged from Review 2):**
- Reject commits containing absolute home directory paths in committed files.
- Secret/credential detection (no keys, tokens, or passwords in committed files).

**Recommended approach:** A shell script in `.git/hooks/pre-commit` — the simplest option requiring no external tooling for a single-developer project. Alternatively, `pre-commit` framework with a YAML config if the developer prefers a declarative approach.

**Gate status:** This still gates the Layer 1 merge. Pre-commit hooks cannot be implemented automatically — the framework selection is a human director decision. The developer must configure at minimum the absolute-path check before merging Layer 1.

---

### Dismissed

**Finding 9 — `Cargo.toml` runtime dependencies at semver-compatible ranges (Dim 3)**

`serde = { version = "1", features = ["derive"] }` — semver-compatible range. Same for others. For a portfolio project, this is the correct level of version pinning: `Cargo.lock` provides exact reproducibility; `Cargo.toml` allows compatible updates. Pinning to exact versions (`= "1.0.228"`) in `Cargo.toml` is over-specified.

**Classification:** Dismissed. Semver ranges in `Cargo.toml` with a committed `Cargo.lock` is the idiomatic Rust approach for binary crates.

---

### Summary

`cargo audit` clean: 0 vulnerabilities across 100 dependencies. `Cargo.lock` to be committed with Layer 1 implementation. Pre-commit hooks remain the one open gate item requiring human director action. No new platform issues from the runtime dependency additions.

---

---

## Review 4 — 2026-04-30 00:00Z

**Scope:** Layer 1 gate closure — pre-commit hook configuration delivered. Evaluating Finding 5 resolution.

---

### Resolved

**Finding 5 (from Reviews 1–3) — Pre-commit hooks (Dim 10)**

Pre-commit framework configured at git root (`guild-portfolio/.pre-commit-config.yaml`). Hooks installed:
- `detect-private-key` — rejects staged files containing private key material
- `no-commit-to-branch` — prevents direct commits to `main`
- `no-home-dir-paths` (local) — rejects staged files containing `$HOME` resolved at runtime; no username hardcoded in any committed file

Hook script at `issue-tracker-cli/.pre-commit-hooks/check-no-home-paths.sh` uses `$HOME` at runtime. Verified: `pre-commit run --all-files` passed; hook correctly caught and prompted removal of two legacy occurrences of the developer's home directory path in `PLATFORM-ENGINEER-REVIEW.md` (review documentation example text). Both occurrences replaced with generic descriptions before commit. Final `grep -rn` scan confirmed zero occurrences of the local username across all tracked files.

Git history was subsequently rewritten with `git filter-repo --force` to remove a username occurrence from commit `f874a60`. Force-push to remote confirmed clean.

**Classification:** Resolved.

---

### Summary

Finding 5 closed. All platform findings are now resolved. The Layer 1 merge gate platform requirements are fully satisfied.

---

---

## Review 5 — 2026-04-30 00:00Z

**Scope:** Layer 1 gate closure pass — no CI/build changes since Review 4. Manual testing session created `tracker.json` artifact.

**Session note:** In-session with all other domain reviews. Acknowledged quality tradeoff.

---

### Resolved

**Finding 10 — `tracker.json` not in `.gitignore` (Dim 8 — Artifact hygiene)**

Manual testing of the Layer 1 binary created `tracker.json` in `issue-tracker-cli/`. The file appeared as `??` in `git status`. The `.gitignore` only excluded `/target`; `tracker.json` was not listed. Any developer running `tracker` from the project directory accumulates this file as untracked and risks accidentally committing test data to the repository.

**Resolution:** Added `/tracker.json` to `issue-tracker-cli/.gitignore`. File is now gitignored and absent from `git status`.

---

### Dismissed

No additional platform findings. CI pipeline, toolchain pin, `Cargo.lock`, and pre-commit hooks all remain in place and verified.

---

### Summary

One finding resolved: `tracker.json` added to `.gitignore`. All platform requirements satisfied. MVR reached for Layer 1.

---

---

## Review 6 — 2026-05-01 00:00Z

**Scope:** Post-merge gap — `cargo fmt --check` enforced in CI but not locally via pre-commit hooks. CI failed on first PR with formatting violations that a local hook would have caught before push.

---

### Resolved

**Finding 11 — `cargo fmt --check` not enforced pre-commit (Dim 10)**

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

**Classification:** Resolved.

---

### Dismissed

No additional platform findings.

---

### Summary

Finding 11 resolved: `cargo fmt --check` now runs as a pre-commit hook, closing the gap between local and CI formatting enforcement.

