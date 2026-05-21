# Platform Engineer Review — 2026-05-20

[Index](../PLATFORM-ENGINEER-REVIEW.md)


**Migration note (PR 6 / Review 78):** This entry is the first Platform Engineer round filed against `bookmark-cli-manual`. The project promoted from portfolio intent to capstone intent at PR 6; PE activates by [G-178](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-178) strong-presumption + [G-155](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-155) dim 38 (fresh-system install verification, capstone-required). Per the [G-177](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-177) reference-example-migration precedent, this entry is authored under the Review 74 hook standard (classification headings, discipline-reference parenthetical on Finding titles, `### Summary` + `**Coordination:**`); Review 77 lifecycle fields (`Owner` / `Status` / `Blocked by` / `Validator`) are included aspirationally on each non-Hallucinated finding so the reference example demonstrates current conventions. The hook's lifecycle-field enforcement (`check-project-review-discipline.py`) does NOT enforce on this date (pre-2026-05-21 cutoff); the next-day Review-77-enforced rounds (2026-05-21+) carry the same fields under the enforced standard.

---

## Review 1 — 2026-05-20 19:30Z

**Scope:** First Platform Engineer cold-context adversarial pass against `bookmark-cli-manual` at Layer 1 close (post-Phase 5 hardening). Adversarial inputs read in declared order: [primer 3](../../../vsdd-suite/primers/3-review-session.md), [PE domain prompt](../../../vsdd-suite/domains/role/PLATFORM-ENGINEER-REVIEW.md), [Rust supplement § Platform Engineering](../../../vsdd-suite/supplements/rust.md), [TOML supplement § Platform Engineering](../../../vsdd-suite/supplements/toml.md), [suite-development.md § Governing standard + Agent-API surface](../../../vsdd-suite/suite-development/suite-development.md). Project artifacts evaluated: [`README.md`](../../README.md), [`Cargo.toml`](../../Cargo.toml), [`Cargo.lock`](../../Cargo.lock) (presence only — line scan), [`.gitignore`](../../.gitignore), [`manual-tests/install-verification.md`](../../manual-tests/install-verification.md), [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md), [`DESIGN.md`](../../DESIGN.md), the prior-domain [QE Review 2](2026-05-20-quality-engineer.md#review-2--2026-05-20-0245z) and [SA Review 1](2026-05-20-solution-architect.md#review-1--2026-05-20-0245z) for round-history context, and the portfolio-root CI surface [`.github/workflows/`](../../../../.github/workflows/) + [`.pre-commit-config.yaml`](../../../../.pre-commit-config.yaml). Capstone intent activates PE Dim 38 (fresh-system install verification) — primary judgement-dependent dimension this round.

**Session note:** Cold session per [primer 3](../../../vsdd-suite/primers/3-review-session.md) — this reviewer did not author any project artifact and reads the project for the first time. The single in-context dependency is the suite domain prompt + supplements; project state is observed via file reads only. Sycophancy-compensation: the PE domain prompt warns that the dominant failure mode is rationalizing inapplicability ("no CI needed — single-user tool"); I tested every "not applicable" candidate against the supplement's stated activation criteria rather than dismissing on local-tool grounds. The bookmark-manager (browser-extension) and issue-tracker-cli workflows under [`.github/workflows/`](../../../../.github/workflows/) demonstrate the portfolio's existing precedent that CLI projects ship a CI workflow — applying that precedent forecloses the easy dismissal.

**Source:** `domain-raised` — cold adversary applying the [PE domain prompt](../../../vsdd-suite/domains/role/PLATFORM-ENGINEER-REVIEW.md) dimensions + [Rust](../../../vsdd-suite/supplements/rust.md) / [TOML](../../../vsdd-suite/supplements/toml.md) supplements to the project state surfaced every finding below. No director-raised observations were folded in mid-round.

**Regression check:** No prior PE rounds exist against `bookmark-cli-manual` (the [`PLATFORM-ENGINEER-REVIEW.md`](../PLATFORM-ENGINEER-REVIEW.md) index Reviews table is empty at session open). Cross-project regression context: the parallel Rust CLI [`issue-tracker-cli`](../../../../issue-tracker-cli/) has a fully-hardened platform surface (CI workflow, [`rust-toolchain.toml`](../../../../issue-tracker-cli/rust-toolchain.toml), [`deny.toml`](../../../../issue-tracker-cli/deny.toml), version-pinned cargo-audit/cargo-deny, SHA-pinned GitHub Actions) per its PE Reviews 1–14; `bookmark-cli-manual` does not inherit any of those controls and the portfolio's [`.pre-commit-config.yaml`](../../../../.pre-commit-config.yaml) hooks `cargo-fmt-check` + `cargo-clippy-check` are explicitly scoped `files: ^issue-tracker-cli/.*\.rs$` (lines 135, 149) — they do not cover `bookmark-cli-manual/`. The platform-control floor that ITC reached at PE R8 closure has not been transferred to `bookmark-cli-manual`; that is the dominant surface this review interrogates.

**Assumption surfacing:** Verified `Cargo.lock` is committed at [`Cargo.lock`](../../Cargo.lock) (24574 bytes; tracked) — the lockfile-commitment dim 3 / Rust supplement § PE `Cargo.lock` commitment passes. Verified no `[profile.*]` section exists in [`Cargo.toml`](../../Cargo.toml) (file is 28 lines; ends after `[dev-dependencies]`). Verified absence of `rust-toolchain.toml`, `deny.toml`, `.cargo/`, `build.rs`, and `.github/workflows/bookmark-cli-manual*` via filesystem scan. The `bookmark-manager.yml` workflow under [`.github/workflows/`](../../../../.github/workflows/) is the browser-extension project, not this CLI; the only Rust CI in the repository is `issue-tracker-cli.yml`.

---

### Deferred

Note on classification choice: each finding below is raised by the cold adversary with a fix recommendation. The classification universe valid for the platform-engineer domain per [`check-project-review-discipline.py`](../../../vsdd-suite/hooks/check-project-review-discipline.py) is `{Resolved, Deferred, Dismissed, Hallucinated}` — there is no `Open` classification on the project side (the suite-side `### Open` heading is suite-internal only). Findings raised but not fixed in-session are `Deferred` with the [G-130](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-130) deferral-trigger discipline applied per finding (named trigger, cost-of-deferral, auto-Backlog clause). The natural deferral target for every finding below is **PE Round 2** — the next cold pass after the operator lands the fixes per the [G-131](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-131) continue-trigger discipline that fires by construction on this round's evidence-backed findings. The auto-Backlog clause names a layer-or-condition that releases the deferral; for a reference-example project at Layer 1 close with no further layers planned, the auto-Backlog fires at Layer 1 final IAR closure (the project's [`PROCESS.md`](../../PROCESS.md) retrospective for Layer 1) if the fix has not landed.

**Finding 1 — No CI workflow exists for `bookmark-cli-manual` (Dim 1 — Pipeline completeness, Dim 2 — Gate enforcement)**

**Owner:** platform-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** software-engineer — the fix is a new `.github/workflows/bookmark-cli-manual.yml` artifact in the project tree; SE owns CI / build-gate config changes per [Review 77](../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-77--2026-05-20-1545z) Finding 2 validator-pair pattern.

[`.github/workflows/`](../../../../.github/workflows/) contains two workflow files at session open: `bookmark-manager.yml` (the browser-extension project, line 1: `name: CI — bookmark-manager`) and `issue-tracker-cli.yml` (the sibling Rust CLI). Neither references `bookmark-cli-manual`. `grep -rn "bookmark-cli" .github/workflows/` returns zero matches. The project has **no automated build / test / lint / fmt-check / audit gate at all**: every quality check is operator-runs-cargo-locally.

This is the single largest platform gap in the project. The dimensions failing simultaneously: pipeline completeness (no checks run on any push or PR); gate enforcement (no required-pass merge gate); action/dependency pinning (no actions to pin); coverage thresholds (no measurement infra to gate on); supply-chain integrity (no `cargo audit` / `cargo deny` ever runs). The portfolio precedent for the equivalent Rust CLI — [`issue-tracker-cli.yml`](../../../../.github/workflows/issue-tracker-cli.yml) — demonstrates the expected shape: build/test/clippy/fmt-check with `--locked`, then `cargo audit`, then `cargo deny --locked check`, with SHA-pinned actions and version-pinned tool installs.

The capstone intent declared at [`DESIGN.md`](../../DESIGN.md) § Project intent (line 11) names the activation rationale: "reference implementations must exercise the full 6-phase methodology to teach what they document." A capstone reference implementation that ships without CI teaches the wrong lesson — namely, that capstone-tier delivery discipline lets a project ship with zero automated gates. The asymmetry between `issue-tracker-cli` (capstone-equivalent hardening at PE R8) and `bookmark-cli-manual` (zero CI) reflects the methodology backwards.

**Recommendation:** Add `.github/workflows/bookmark-cli-manual.yml` modeled on `issue-tracker-cli.yml` — `working-directory: vsdd-suite-reference-examples/bookmark-cli-manual`, path-filtered to `vsdd-suite-reference-examples/bookmark-cli-manual/**`, with build/test/clippy/fmt-check (all `--locked`) at minimum; cargo-audit + cargo-deny once Finding 4 lands; SHA-pinned actions per `issue-tracker-cli.yml` lines 23–31; tool installs version-pinned per its lines 51, 57. The current `branches: ['**']` push trigger on the existing Rust workflow is the right pattern to reuse.

**Classification:** Open. Recommendation provided; ownership belongs to platform-engineer for implementation; SE validates the workflow artifact.

---

**Finding 2 — No `rust-toolchain.toml` despite a declared MSRV in `DESIGN.md` (Rust supplement § PE — Toolchain pinning, TOML supplement § PE — `rust-toolchain.toml` for toolchain pinning)**

**Owner:** platform-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** software-engineer — the fix is a new `rust-toolchain.toml` file at the project root; SE owns toolchain-pin config per the [Rust supplement](../../../vsdd-suite/supplements/rust.md) § Software Engineering MSRV discipline.

[`DESIGN.md`](../../DESIGN.md) § Constraints (line 154) declares: "**Rust toolchain:** 1.78+ (modern stable Rust; no unstable features)." [`README.md`](../../README.md) § Prerequisites (line 13) echoes: "[Rust](https://www.rust-lang.org/) 1.78+ (`cargo --version` to check)." Neither claim is mechanically enforced — `Cargo.toml` has no `rust-version` key (verified: [`Cargo.toml`](../../Cargo.toml) lines 1–8 list `name` / `version` / `edition` / `description` / `license` / `publish` only) and no `rust-toolchain.toml` exists at the project root. A contributor on Rust 1.77 or earlier can build, test, and `cargo install` the project without any signal that they are below the declared MSRV; a future API that requires a newer minimum (e.g., a `std::io::IsTerminal` use that requires 1.70 — the exact precedent ITC hit at PE R12 Finding 2) would silently break MSRV without anyone noticing until a downstream consumer reported the failure.

The sibling [`issue-tracker-cli`](../../../../issue-tracker-cli/) pins its toolchain via [`rust-toolchain.toml`](../../../../issue-tracker-cli/rust-toolchain.toml) (`channel = "1.94.1"`, `components = ["clippy", "rustfmt"]`) AND declares `rust-version = "1.82"` in [`Cargo.toml`](../../../../issue-tracker-cli/Cargo.toml) line 5 — the two are complementary: `rust-toolchain.toml` pins what every contributor's `cargo build` uses, `rust-version` is the manifest-level MSRV that crates.io/cargo enforce when consumers depend on the crate.

The Rust supplement is explicit: "Is the Rust toolchain version pinned via `rust-toolchain.toml` to ensure reproducible builds across environments?" Absent without rationale = finding.

**Recommendation:** Add `rust-toolchain.toml` at the project root with `channel = "1.78"` (or a current stable that satisfies the DESIGN.md declared minimum) + `components = ["clippy", "rustfmt"]`. Independently, add `rust-version = "1.78"` to [`Cargo.toml`](../../Cargo.toml) `[package]` — the manifest MSRV is the cargo-resolver-visible signal.

**Classification:** Open. Recommendation provided.

---

**Finding 3 — `Cargo.toml` `[package]` is missing canonical fields (`repository`, `readme`, `rust-version`) (TOML supplement § Software Engineering — `[package]` metadata completeness)**

**Owner:** platform-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** software-engineer — Cargo.toml is owned at the project-tree level; SE pairs naturally with package-manifest fixes.

[`Cargo.toml`](../../Cargo.toml) lines 1–7 declare:

```
[package]
name = "bookmark-cli"
version = "0.1.0"
edition = "2021"
description = "Reference implementation for the VSDD suite's worked example. Captures URLs at the terminal and recalls them later."
license = "MIT"
publish = false
```

The TOML supplement § SE explicitly enumerates the canonical fields: "`name`, `version`, `edition`, `license` (SPDX identifier), `description`, `repository`, `readme`, `keywords`, `categories`, `authors`." Three load-bearing fields are absent:

1. **`rust-version`** — missing; the MSRV stated in [`DESIGN.md`](../../DESIGN.md) § Constraints line 154 is not mechanized. See Finding 2.
2. **`repository`** — missing; the [`README.md`](../../README.md) install instructions (line 19: `git clone <portfolio-url>`) leave the repository URL as a placeholder. The sibling [`issue-tracker-cli`](../../../../issue-tracker-cli/Cargo.toml) line 9 declares `repository = "https://github.com/magnificentlycursed/guild-portfolio"` — the same value applies here. Without `repository`, a `cargo install --path .` consumer who runs `cargo info bookmark-cli` cannot follow back to source.
3. **`readme`** — missing; while `cargo` will default-discover `README.md`, declaring it explicitly is the supplement-prescribed shape and makes the manifest self-describing.

The `license = "MIT"` declaration is also weaker than the portfolio precedent: [`issue-tracker-cli`](../../../../issue-tracker-cli/Cargo.toml) line 8 declares `license = "MIT OR Apache-2.0"`, the conventional Rust dual-license shape. The [`README.md`](../../README.md) License section (line 61–63) only states "MIT", so the single-license declaration is internally consistent — but the inconsistency across the portfolio's two Rust CLIs is itself worth a Solution Owner ratification (raise during SO Review).

**Recommendation:** Extend [`Cargo.toml`](../../Cargo.toml) `[package]` with `rust-version = "1.78"`, `repository = "https://github.com/magnificentlycursed/guild-portfolio"`, and `readme = "README.md"`. Surface license-uniformity to SO Review as cross-project ratification.

**Classification:** Open. Recommendation provided.

---

**Finding 4 — No `deny.toml` / no `cargo deny` configuration (Rust supplement § PE — `cargo deny`, TOML supplement § Security — cargo-deny configured)**

**Owner:** platform-engineer
**Status:** raised
**Blocked by:** Finding 1 — until CI exists, the `cargo deny check` invocation has no gate to attach to; the `deny.toml` artifact can ship independently, but the enforcement requires the CI from Finding 1.
**Validator:** security — the four-section `deny.toml` ([advisories] / [licenses] / [bans] / [sources]) is the supply-chain policy surface; Security pairs naturally on policy choice; PE owns CI-side wiring.

The Rust supplement § PE is explicit: "Is `cargo deny check` configured with a `deny.toml`? This gates on CVEs, license violations, banned crates, and disallowed sources simultaneously. `cargo audit` alone is insufficient if `cargo deny` is not also present." `bookmark-cli-manual` has neither `deny.toml` (filesystem scan: file absent) nor any `cargo deny` invocation (no CI at all per Finding 1). The TOML supplement § Security adds: "Missing or incomplete `deny.toml` is a finding."

Concrete gaps the absence of `deny.toml` leaves uncovered: (a) the [`Cargo.lock`](../../Cargo.lock) at 24574 bytes resolves a non-trivial transitive dependency graph (`clap`, `serde`, `serde_json`, `chrono`, `anyhow`, plus their transitives) — without `[advisories]` policy, a RUSTSEC-class CVE landing in any of these crates produces no signal; (b) the [`DESIGN.md`](../../DESIGN.md) § Constraints line 156 commitment "all from crates.io, no git deps" is operator-discipline rather than mechanism — `[sources]` enforcement is what makes that commitment durable; (c) no license-policy gate — `license = "MIT"` declared in [`Cargo.toml`](../../Cargo.toml) line 6 is the project's outward claim but no `[licenses]` policy enforces what the transitive graph is allowed to be (a GPL-only transitive would silently land and break the MIT distribution claim downstream).

The sibling [`issue-tracker-cli/deny.toml`](../../../../issue-tracker-cli/deny.toml) is the natural template — it's the worked example PE R8 Finding 2 closure produced.

**Recommendation:** Add `bookmark-cli-manual/deny.toml` with the four standard sections (copy + adapt from `issue-tracker-cli/deny.toml`); add `cargo deny --locked check` to the CI workflow from Finding 1 as a post-`cargo audit` step.

**Classification:** Open. Recommendation provided.

---

**Finding 5 — No `cargo audit` invocation anywhere — known-CVE detection is absent (Dim 11 — Security scanning, Rust supplement § PE — `cargo audit`)**

**Owner:** platform-engineer
**Status:** raised
**Blocked by:** Finding 1 — `cargo audit` needs the CI workflow to land on; absent CI there is no place to wire it.
**Validator:** security — supply-chain CVE policy is Security's domain; PE owns the CI wiring.

[`Cargo.lock`](../../Cargo.lock) (24574 bytes) records the resolved dependency graph for `clap = "4"`, `serde = "1"`, `serde_json = "1"`, `chrono = "0.4"`, `anyhow = "1"`, plus their transitive closure. No `cargo audit` ever runs against it — there is no CI (Finding 1), no pre-commit hook invocation, and no manual instruction in [`README.md`](../../README.md) / [`TODO.md`](../../TODO.md) / `manual-tests/` directing the operator to run it. A new RUSTSEC advisory published against any direct or transitive dependency between PR-merge and the next `cargo install --path .` run produces zero signal to the project.

The Rust supplement § PE: "Is `cargo audit` run in CI? Does it fail the build on findings above the accepted severity threshold?" — the answer is no on both counts. The TOML supplement § Security § cargo-audit in CI restates this: "Is `cargo audit` run against `Cargo.lock` in CI?"

`cargo audit` can be subsumed by Finding 4's `cargo deny --locked check` (the `[advisories]` section), but the Rust supplement names them as parallel controls and the portfolio precedent ([`issue-tracker-cli.yml`](../../../../.github/workflows/issue-tracker-cli.yml) lines 49–63) runs both — `cargo audit` first as a fast CVE-only check, then `cargo deny check` for the broader policy set.

**Recommendation:** Add `cargo install cargo-audit --locked --version <pinned>` + `cargo audit` to the Finding 1 CI workflow. Pin the tool version per the supply-chain discipline ITC PE R8 Finding 6 established. May be consolidated with Finding 4's `cargo deny` invocation if the operator prefers the single-tool path; the supplement permits both.

**Classification:** Open. Recommendation provided.

---

**Finding 6 — `Cargo.toml` has no `[profile.release]` declarations — release-build tuning is silently default (Rust supplement § PE — Toolchain / build config; TOML supplement § PE — `[profile.release]` optimization settings + Performance Engineer — `lto` configuration tradeoff)**

**Owner:** platform-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** performance-engineer — `[profile.release]` settings are the Performance Engineer's primary platform surface (LTO / codegen-units / panic-handling tradeoffs); PE owns the manifest edit, PerfE validates the chosen values.

[`Cargo.toml`](../../Cargo.toml) ends at line 28 after `[dev-dependencies]`. There is no `[profile.release]` section, so the project inherits cargo's defaults silently. The TOML supplement § PE — `[profile.release]` optimization settings is explicit:

> Are release-build profile settings declared explicitly rather than inheriting cargo defaults? ... Each setting carries a tradeoff — declare the chosen value and a one-line rationale.

The TOML supplement § TW — "Section grouping that mirrors logical organization" lists `[profile.*]` as one of the conventional sections expected in a `Cargo.toml`. The TOML supplement § PerfE goes further: "For CLI tools and binaries where users run the release artifact thousands of times, `'thin'` or `'fat'` LTO is the right tradeoff." `bookmark-cli-manual` is a CLI binary distributed via `cargo install --path .` to `~/.cargo/bin/bm` ([`README.md`](../../README.md) line 22) — the artifact is the long-lived release-build, exactly the workload where LTO is the supplement-recommended default.

The dim is a judgement call (the supplement permits inheriting defaults if declared); the *finding* is the silent inheritance — a reviewer reading [`Cargo.toml`](../../Cargo.toml) cannot tell whether the absence is deliberate (defaults were considered and accepted) or accidental (no one thought about it). The TOML supplement § TW guidance — "comments name the why; the value names the what" — applies: even a one-line `# Inheriting cargo's release defaults — opt-level=3, lto=false; sufficient for a single-binary tool` would satisfy the discipline. The absence of the declaration *and* the absence of any rationale is the finding.

**Recommendation:** Either (a) add a minimal `[profile.release]` block with explicit `opt-level = 3` + `lto = "thin"` + `strip = "symbols"` and a one-line rationale comment, or (b) add a brief comment block at the end of [`Cargo.toml`](../../Cargo.toml) documenting that the cargo defaults are deliberately accepted for a single-binary CLI. Option (a) is the TOML supplement § PerfE preferred path for CLI tools; option (b) is the minimum to close the silent-inheritance finding.

**Classification:** Open. Recommendation provided.

---

**Finding 7 — Pre-commit `cargo-fmt-check` / `cargo-clippy-check` hooks do not cover `bookmark-cli-manual/` (Dim 9 — Left-shift opportunities, Dim 10 — Pre-commit hooks)**

**Owner:** platform-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** sanity-check — the fix is a new pre-commit hook entry in [`.pre-commit-config.yaml`](../../../../.pre-commit-config.yaml) at the portfolio root, mirroring the `issue-tracker-cli`-scoped hooks; this is a PE shift-left mechanization (a new pre-commit hook the suite-adjacent surface authors to catch a recurring defect class — formatting / clippy drift) per the [meta-validator-of-last-resort pattern](../../../vsdd-suite/domains/role/PLATFORM-ENGINEER-REVIEW.md) ([Review 77](../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-77--2026-05-20-1545z) Finding 2 — for project-tree hooks owned by Platform Engineer, [Sanity Check](../../../vsdd-suite/domains/meta/SANITY-CHECK-REVIEW.md) is the cohesion-validator).

[`.pre-commit-config.yaml`](../../../../.pre-commit-config.yaml) line 126–149 declares two Rust-specific hooks:

- `cargo-fmt-check` (lines 126–135): `files: ^issue-tracker-cli/.*\.rs$`
- `cargo-clippy-check` (lines 137–149): `files: ^issue-tracker-cli/.*\.rs$`

Both are explicitly scoped to `issue-tracker-cli/` only. `bookmark-cli-manual/src/*.rs` modifications do not trigger either hook — `cargo fmt --check` and `cargo clippy -- -D warnings` are not enforced locally on bookmark-cli-manual source files. The PE Dim 9 — Left-shift opportunities prompt — "Which manual review steps could be automated and added to CI?" — applies inverted here: the *existing* automation is selectively un-applied to this project.

The Dim 10 framing — "Are pre-commit hooks installed and enforced to catch [defects] before it enters version control?" — fails for the project's own source: a contributor editing `bookmark-cli-manual/src/lib.rs` can commit a `cargo fmt`-violating or `cargo clippy`-warning-emitting change with no local signal. Without the CI from Finding 1, there is no second line of defense either; defects land directly in the repository.

The hook config in [`.pre-commit-config.yaml`](../../../../.pre-commit-config.yaml) line 133 — `bash -c 'cd "$(git rev-parse --show-toplevel)/issue-tracker-cli" && cargo fmt --check'` — is the worked pattern; the natural fix is to add parallel entries `cargo-fmt-check-bookmark` + `cargo-clippy-check-bookmark` with `cd "$(git rev-parse --show-toplevel)/vsdd-suite-reference-examples/bookmark-cli-manual"` and `files: ^vsdd-suite-reference-examples/bookmark-cli-manual/.*\.rs$`.

**Recommendation:** Add the two parallel hooks. Independently, evaluate whether the rust-CLI hook shape should be generalized — the portfolio now has two Rust CLI projects (and the suite's reference-examples directory will likely accumulate more); a per-project hook entry grows linearly. A `language: system` hook with a path-derived working directory (e.g., scan `git diff --staged --name-only` for `.rs` files, group by their crate root, run `cargo fmt --check` / `cargo clippy` per group) generalizes — but that is a Solution Architect-shape design rather than a PE-shape fix; raise to SA review if the recurrence pattern materializes.

**Classification:** Open. Recommendation provided.

---

**Finding 8 — `cargo install --path .` invocations across README + manual-tests + install-verification do not use `--locked` — committed `Cargo.lock` is advisory rather than authoritative at install time (Dim 3 — Dependency installation, TOML supplement § PE — Lockfile commitment, Rust supplement § PE — `Cargo.lock` commitment)**

**Owner:** platform-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** software-engineer — the fix is editing user-facing install commands in [`README.md`](../../README.md), [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md), and [`manual-tests/install-verification.md`](../../manual-tests/install-verification.md); SE pairs on user-facing install / documentation correctness.

`Cargo.lock` is committed at the project root (verified — file present, 24574 bytes — Rust supplement § PE `Cargo.lock` commitment passes for the **existence** check). But the install invocations across user-facing docs do not enforce the lockfile:

- [`README.md`](../../README.md) line 21: `cargo install --path . --force`
- [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md) line 14: `cargo install --path . --force --quiet` (Step 0)
- [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md) line 167: `cargo install --path . --force --quiet` (Step 5 reinstall)
- [`manual-tests/install-verification.md`](../../manual-tests/install-verification.md) line 35: `cargo install --path . --force --quiet` (the operator-instruction for the [G-155](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-155) gating step)

None use `--locked`. Without `--locked`, if `Cargo.lock` falls out of sync with `Cargo.toml` (e.g., a contributor edits `Cargo.toml` and forgets to refresh the lockfile, or — more subtly — if the lockfile gets out of date relative to a tightened semver constraint), `cargo install` silently regenerates the lock to satisfy the manifest and installs against the regenerated graph. The Dim 3 — Dependency installation prompt — "Is a deterministic install command used ... Is the lock file committed and the source of truth for installs?" — fails on the second clause.

The PE R8 Finding 5 closure for `issue-tracker-cli` is the worked precedent: every `cargo build / test / clippy / audit` in CI is `--locked`. The capstone-tier reproducibility discipline ([`DESIGN.md`](../../DESIGN.md) § Constraints line 156: "`Cargo.lock` committed") relies on `--locked` to make the commitment binding.

The Dim 38 implication is sharpest: [`manual-tests/install-verification.md`](../../manual-tests/install-verification.md) is the file the fresh-system non-author verifier executes. If their `cargo install --path .` regenerates the lockfile and installs against a different dependency graph than the developer tested, the verification record loses its reproducibility property — the non-author's PASS doesn't attest to the same build the developer shipped.

**Recommendation:** Replace every `cargo install --path . --force [--quiet]` with `cargo install --path . --locked --force [--quiet]` across the three files. The flag composes cleanly with the existing `--force` and `--quiet`; no new infrastructure needed. Same change should apply prospectively to any new install instruction the project adds.

**Classification:** Open. Recommendation provided.

---

**Finding 9 — Capstone Dim 38 install-verification record has zero PASSING rows; the gate is declared but not satisfied (Dim 38 — Fresh-system install verification at capstone intent)**

**Owner:** platform-engineer
**Status:** raised
**Blocked by:** *(none — the gate is operator-executable; no fix-landed prerequisite blocks recording a PASS row.)*
**Validator:** *self* — Dim 38 is binary against [G-155](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-155) ("a single PASSING row from a non-author on a fresh system is sufficient to satisfy dim 38"); the validation surface is the file content itself, not a cross-domain judgement call. The strict-self-validation rationale ([Review 77](../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-77--2026-05-20-1545z) Finding 1): the verification record is a compliance-check artifact where presence/absence of a row IS the validation — no second reviewer adds adversarial signal that file-presence-checking doesn't already produce. (Sanity Check would be the alternative; for a binary file-state check Sanity Check is over-allocated.)

[`manual-tests/install-verification.md`](../../manual-tests/install-verification.md) Verification records table (line 53–55) contains exactly one row, and it is the scaffolding template:

```
| *(pending)* | *(non-author operator)* | *(fresh-system context)* | *(per manual-tests/layer-1.md execution)* | *(divergences, if any)* | *(PASS / FAIL)* | *(any context)* |
```

The **Outcome** column is `*(pending)*`. The file's own self-disclosure (lines 9–16) is explicit:

> The AI co-authorship is disclosed here per the operator's directive ... the verification rows below describe what the operator's fresh-system install attempt would record; the **Outcome** column is left blank pending the operator's execution. Per [G-155](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-155), leaving the **Outcome** blank means the gate is not yet satisfied; the project's capstone closure is pending operator-executed verification.

This is a self-acknowledged Open finding. The PE Dim 38 prompt is explicit: "For capstone/production: the install-verification record is gating." The capstone intent is declared in [`DESIGN.md`](../../DESIGN.md) line 11 ("Promoted to `capstone` in PR 6 / Review 78"). [`DESIGN.md`](../../DESIGN.md) line 17 (Phase 6 strategy) names the convergence dependency: "every active-domain Phase 3 round at MVR per the post-PR-6 capstone IAR coverage" — Platform Engineer is an active domain; Dim 38 is its capstone-required dim; the dim is at "not yet satisfied" state.

The sycophancy-guard from the PE prompt applies here: the project is documented honestly (the file is *not* hiding the gap; it discloses it forward-facing). The dim, however, doesn't pass on disclosure alone — it passes on a PASSING row from a non-author fresh-system execution.

Important scope note for the cold adversary: this finding is NOT raised to require the AI agent to execute the verification (which the file correctly establishes is impossible — the AI is by definition the author). The finding is raised so that Layer 1 closure cannot silently skip the dim. The router for resolution is the human operator's manual execution; the cold-session PE round can only flag that the gate is open.

Adjacent concern about the procedure itself: [`manual-tests/install-verification.md`](../../manual-tests/install-verification.md) Step 2 (line 35) instructs `cargo install --path . --force --quiet` without `--locked` — see Finding 8. The verification, when executed, will not exercise the reproducible-build property the committed `Cargo.lock` exists to attest; the operator's fresh-system PASS will validate a possibly-different dependency graph than the developer shipped. The recommendation is to land Finding 8's `--locked` fix *before* the operator executes verification, so the PASS row attests to the right build.

**Recommendation:** No code/config change resolves this finding; the recommendation is procedural. Order of operations: (1) land Finding 8 (`--locked` in install commands) — without this the verification PASS attests to the wrong graph; (2) operator executes [`manual-tests/install-verification.md`](../../manual-tests/install-verification.md) Steps 1–4 on a non-author fresh system; (3) operator fills in a PASS row in the Verification records table with date, verifier, system, and outcome; (4) the PE round following the row addition verifies the row's completeness and closes this Finding. Until step (3) lands, the capstone-tier merge gate is open.

**Classification:** Open. Procedural-resolution path documented.

---

**Finding 10 — No coverage measurement or threshold enforcement (Dim 6 — Coverage thresholds, Rust supplement § PE — Coverage enforcement, Rust supplement § QE — Coverage thresholds)**

**Owner:** platform-engineer
**Status:** raised
**Blocked by:** Finding 1 — coverage tooling lands in CI; absent CI there is no place to wire it.
**Validator:** quality-engineer — coverage thresholds are equally a QE-supplement concern (Rust § QE: "Line coverage should be at minimum 80%. Public API coverage should be 100%"); PE owns the CI-side mechanization, QE validates the thresholds.

The Rust supplement § PE states unambiguously: "Is coverage measured in CI with thresholds enforced? Minimum 80% line coverage; 100% public API coverage. A CI run that measures coverage but does not fail below thresholds is not enforcement." Neither half is present for `bookmark-cli-manual`: no CI exists at all (Finding 1), no `cargo tarpaulin` / `cargo llvm-cov` invocation anywhere in the repo, no coverage report artifact, no threshold gate.

The Rust supplement § QE adds the public-API requirement: "every exported function, type, and trait impl must have at least one test exercising it." [`src/lib.rs`](../../src/lib.rs) lines 27–90 export `Bookmark`, `BookmarkStore`, and four `impl BookmarkStore` methods (`load`, `save`, `add`, `newest_first`); [`src/lib.rs`](../../src/lib.rs) lines 92–169 include unit tests for `newest_first`, `load`, `save`, and the QE-R2-introduced `save_creates_parent_directory_for_nested_path` — `add` is exercised indirectly through `save_then_load_roundtrips` but has no direct unit test, and the data-type derives (`Serialize`/`Deserialize` for `Bookmark`/`BookmarkStore`) are exercised through the integration tests in `tests/bookmarks.rs` but not asserted against the round-trip invariant via a dedicated unit test. Without measurement, the actual coverage is unknown; without enforcement, drift is unobservable.

The portfolio precedent at [`issue-tracker-cli`](../../../../issue-tracker-cli/) closed PE R8 Finding 3 (coverage) with an SO R14 Backlog disposition (defer until a layer adds substantial code without tests OR project exceeds ~1000 LOC OR external review). The same Backlog disposition may apply here — `bookmark-cli-manual` is even smaller (Layer 1 only; 169 lines in `src/lib.rs`, ~50 lines in `src/main.rs`, 100% of public API arguably exercised through the 8 tests). The PE round, however, cannot Backlog (only SO can); the cold-session classification is Open with a recommendation routed to SO for Backlog ratification.

**Recommendation:** Raise to SO Review for Backlog ratification with re-raise criteria mirroring SO R14's `issue-tracker-cli` Backlog (re-raise on substantial code addition, ~1000 LOC threshold, or external review). If SO declines Backlog, add `cargo llvm-cov --fail-under-lines 80` to the Finding 1 CI workflow.

**Classification:** Open. Recommendation provided + routing to SO.

---

**Finding 11 — Clippy lint configuration relies on cargo defaults; no crate-level `#![deny(...)]` deny set (Rust supplement § SE — Clippy lint configuration, § PE — `cargo clippy --deny warnings`)**

**Owner:** platform-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** software-engineer — crate-level lint configuration is SE-owned per the [Rust supplement](../../../vsdd-suite/supplements/rust.md) § SE; PE pairs on the CI-side `-D warnings` enforcement.

[`src/lib.rs`](../../src/lib.rs) line 1–22 contains the module-level documentation only; no `#![deny(...)]` or `#![warn(...)]` crate attributes. [`src/main.rs`](../../src/main.rs) is similarly unconfigured (not read in full this round; the absence is observable in the file's first 5 lines if not present). The Rust supplement § SE — Clippy lint configuration is explicit:

> The standard deny set is: `#![deny(clippy::all, clippy::pedantic, clippy::nursery, clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::missing_errors_doc, clippy::missing_panics_doc, missing_docs)]`. Note that `missing_docs` is a rustc lint (not a clippy lint) and pairs with the rustdoc-coverage check in the Technical Writer section (G-137) — including it in the crate-level deny set catches missing public-item documentation at clippy/cargo-check time rather than only at `cargo doc` time.

The supplement permits selective `#[allow(...)]` with comments for deviation; it does not permit silent absence of the deny set. The PE-side mechanization (`cargo clippy -- -D warnings` in CI) is missing too — see Finding 1. Both halves of the discipline fail simultaneously.

The portfolio precedent at [`issue-tracker-cli`](../../../../issue-tracker-cli/) closed PE R8 Finding 4 with a partial deny set (Resolved partial — `clippy::expect_used`, `clippy::panic`, `clippy::missing_errors_doc` added; pedantic/nursery skipped with rationale in DECISIONS.md). A similar partial-with-documented-rationale closure is acceptable here; the silent-absent state is not.

**Recommendation:** Add the supplement's standard deny set to [`src/lib.rs`](../../src/lib.rs) line 1 (and [`src/main.rs`](../../src/main.rs) for symmetry). Document any selective skips in a project-level DECISIONS.md or inline comments. Wire `cargo clippy --all-targets --locked -- -D warnings` into the Finding 1 CI workflow.

**Classification:** Open. Recommendation provided.

---

### Dismissed

**Finding 12 — No containerization, no observability stack, no IaC, no IAM, no disaster-recovery plan (Dims 16–33)**

**Owner:** *(N/A — dismissed)*
**Status:** *(N/A — dismissed)*

The cold adversary considered each in turn:

- **Infrastructure as Code (Dim 16):** no infrastructure to encode — single-user CLI installed via `cargo install --path .` to the user's `~/.cargo/bin/`. No cloud account, no on-prem hardware, no Terraform analog warranted.
- **Containerization (Dim 18–19):** no container ships — the deliverable is a Rust binary. [`manual-tests/install-verification.md`](../../manual-tests/install-verification.md) line 21 suggests `docker run --rm -it rust:1.81-bookworm` as a fresh-environment proxy for the verifier, but this is a verification-environment proxy, not a distribution mechanism.
- **Observability (Dims 22–33):** no service runs — there is no log/metric/trace surface beyond stderr error messages. The runbook-coverage and silent-success-confirmation framings of Dims 32–33 don't apply to a CLI whose entire success signal is "command exits 0 + the file changed on disk."
- **Disaster recovery (Dim 21):** the project's data layer is a single JSON file at `$BOOKMARK_CLI_DB` whose backup-and-restore is the user's filesystem (no automated DR sensible at this scope).

The PE prompt's sycophancy-check ("Scrutinize every 'not applicable' determination — an agent that finds no applicable security scanning concerns in a published package, or no infrastructure concerns in a deployed application, is likely rationalizing rather than reviewing") was tested against each. The asymmetric posture here is that *some* dimensions of the prompt ARE rationalized to inapplicability (the infrastructure / containerization / observability / DR cluster above) while the supply-chain + CI + capstone-install-verification + toolchain-pin dimensions are NOT — Findings 1–11 above name the load-bearing applicable surface. The cluster dismissed here is not in tension with the active findings; the project is genuinely a small local CLI with no deployment surface.

**Classification:** Dismissed. The single-binary single-user CLI scope from [`DESIGN.md`](../../DESIGN.md) § Scope and non-goals (lines 27–52) is genuinely outside the infrastructure / observability / DR surface; no rationalization required. If a future layer adds e.g. a multi-user server mode (which DESIGN.md explicitly excludes as a non-goal), re-raise.

---

**Finding 13 — Web-shaped performance dimensions not applicable to a CLI binary (Dims 34–37 — Performance budget, time-to-interactive, asset optimization, performance regression risk)**

**Owner:** *(N/A — dismissed)*
**Status:** *(N/A — dismissed)*

Dims 34 (time-to-interactive — browser apps), 35 (asset optimization — JS bundles, images), 36 (performance budget — bundle size, TTI), 37 (performance regression risk — JS-shaped failure modes) target browser/web application surfaces. `bookmark-cli-manual` is a Rust CLI binary; the relevant Performance Engineer surface is `[profile.release]` (raised separately at Finding 6) and any Criterion benchmarks (none active at Layer 1; Layer 1 has no hot-path performance requirement per [`DESIGN.md`](../../DESIGN.md)).

**Classification:** Dismissed. Wrong-domain dimensions; the CLI-shaped performance concerns are routed through Finding 6 (`[profile.release]`) and the Performance Engineer domain's separate review.

---

### Deferred

*(none — every applicable finding above is Open with a procedural, code, or config recommendation; deferral would push capstone-tier discipline past the [G-130](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-130) deferral-trigger discipline without a specific triggering layer to defer to. Layer 2 and Layer 3 are scoped in DESIGN.md but explicitly out-of-scope per the reference-implementation-satisfied-by-one-layer framing — they are not natural deferral targets.)*

---

### Hallucinated

*(none — every Open finding above is grounded in a specific file:line citation in the project state. The cold adversary applied the [PE domain prompt](../../../vsdd-suite/domains/role/PLATFORM-ENGINEER-REVIEW.md) sycophancy-check rigorously: each dim that produced a finding was re-tested against the project's actual artifacts, and the inapplicable-cluster (Findings 12 + 13) was named and justified rather than silently dropped.)*

---

### Summary

11 Open findings + 2 Dismissed across the [PE domain prompt](../../../vsdd-suite/domains/role/PLATFORM-ENGINEER-REVIEW.md)'s standard dimensions + [Rust](../../../vsdd-suite/supplements/rust.md) + [TOML](../../../vsdd-suite/supplements/toml.md) supplements. The project ships with a load-bearing platform-control gap: **no CI workflow exists** (Finding 1) and therefore no automated gate enforces any of the Rust supplement's CI-side controls — `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo audit`, `cargo deny --locked check`, coverage measurement, `--locked` enforcement. The shift-left mechanizations partially fill the gap for `issue-tracker-cli/` (the pre-commit `cargo-fmt-check` + `cargo-clippy-check` hooks) but explicitly do not apply to `bookmark-cli-manual/` (Finding 7). The result is a capstone-tier reference example whose platform-control surface is materially below the sibling Rust CLI [`issue-tracker-cli`](../../../../issue-tracker-cli/), which closed its equivalent PE R8 with the same dimensions covered.

Additionally, **the capstone-required Dim 38 install-verification gate is open** (Finding 9) — the [`manual-tests/install-verification.md`](../../manual-tests/install-verification.md) Verification records table has zero PASSING rows from a non-author fresh-system execution. The project's own disclosure (the file's lines 9–16 self-acknowledgement) is honest about the gap; the disclosure is not a substitute for the gate signal per [G-155](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-155). Until the operator executes the verification on a fresh non-author system and records a PASS row, the capstone closure is pending Platform Engineer Dim 38.

Per [G-131](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-131) continue-trigger discipline, this Round 1 producing 11 Open findings mandates a Round 2 after the fixes land — the cold pass verifies the fix held and looks for adjacent defects. The Layer 1 MVR claim in [`DESIGN.md`](../../DESIGN.md) § Phase 5/6 strategy (lines 15–17) is dependent on every active-domain Phase 3 round at MVR; the PE round is not at MVR until Findings 1–11 resolve and the subsequent Round produces only Hallucinated or no findings.

**Coordination:**

- **Finding 1** (no CI) — surface to [Software Engineer review](../SOFTWARE-ENGINEER-REVIEW.md) for the workflow YAML content review (build/test/clippy/fmt-check invocations + path-filter + working-directory scoping).
- **Finding 3** (Cargo.toml missing `repository` / `readme` / `rust-version`; license-uniformity across portfolio Rust CLIs) — license-uniformity routed to [Solution Owner review](../SOLUTION-OWNER-REVIEW.md) for cross-project ratification (MIT vs MIT OR Apache-2.0 — the same operator owns both projects; the choice should be deliberate, not accidental).
- **Finding 4** (`deny.toml` / cargo-deny) — surface to [Security review](../SECURITY-REVIEW.md) for `[advisories]` + `[licenses]` + `[bans]` + `[sources]` policy content (Security owns supply-chain policy; PE owns CI wiring).
- **Finding 5** (`cargo audit`) — surface to [Security review](../SECURITY-REVIEW.md) — CVE-policy ownership.
- **Finding 6** (`[profile.release]`) — surface to [Performance Engineer review](../PERFORMANCE-ENGINEER-REVIEW.md) for LTO / opt-level / panic-handling choices (PerfE owns the tradeoff judgement; PE owns the manifest edit).
- **Finding 7** (pre-commit hook coverage gap) — if the recurring-multi-project-hook-shape pattern materializes (third Rust CLI added to the portfolio), surface to [Solution Architect review](../SOLUTION-ARCHITECT-REVIEW.md) for generalized-hook-shape design.
- **Finding 8** (`--locked` on install commands) — surface to [Technical Writer review](../TECHNICAL-WRITER-REVIEW.md) for [`README.md`](../../README.md) install-instruction correctness (TW owns user-facing doc accuracy); SE pairs on the technical correctness.
- **Finding 9** (Dim 38 install-verification) — surface to [VDD-IAR Alignment review](../VDD-IAR-ALIGNMENT-REVIEW.md) as the meta-process check that the capstone-required gate is tracked; SO routing applies if the operator opts to Backlog-with-trigger rather than execute verification.
- **Finding 10** (coverage) — recommendation routes to [Solution Owner review](../SOLUTION-OWNER-REVIEW.md) for Backlog ratification (parallel to ITC SO R14 Finding 5); [Quality Engineer review](../QUALITY-ENGINEER-REVIEW.md) pairs on the public-API coverage requirement.
- **Finding 11** (clippy deny set) — surface to [Software Engineer review](../SOFTWARE-ENGINEER-REVIEW.md) — the crate-level `#![deny(...)]` is SE-owned per Rust supplement § SE.

**Round trigger:** Per [G-131](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-131), this round's 11 Open findings (each grounded in evidence; none classified Hallucinated) mandate Round 2 after the fixes land — the continue trigger fires by construction. The MVR signal for the Platform Engineer domain will be the post-fix Round that produces only Hallucinated findings or no findings.
