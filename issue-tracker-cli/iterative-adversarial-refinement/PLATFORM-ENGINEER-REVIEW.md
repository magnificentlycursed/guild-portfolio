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
- Reject commits that include absolute paths containing the developer's local username (e.g., `DEVELOPER_HOME/`) in committed files. This prevents local machine-specific paths from appearing in public portfolio code.
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
