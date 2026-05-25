# Manual Testing — Install Verification

Third-party install verification record per [G-155](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-155) [Platform Engineer](../../../vsdd-suite/domains/role/PLATFORM-ENGINEER-REVIEW.md) Dim 38 (Fresh-system install verification, capstone / production intent only).

**File location convention (Review 78 Finding 2):** install-verification IS a manual test (a human runs commands on a real fresh system and records what they observed), so this file lives in `manual-tests/` alongside the per-layer test plans. The file name is lowercase + hyphenated (`install-verification.md`) to match the per-layer pattern (`layer-N.md`).

---

## AI-co-authored reference-example disclosure

**This install-verification record is AI-co-authored. AI-author cannot satisfy this gate.** [G-155](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-155) dim 38 specifies that the install verification be performed by **a non-author on a fresh system** — that is the discipline's load-bearing requirement. The AI agent that built `bookmark-cli-manual` and authored this file is by definition the author; the AI's environment is not a fresh non-author system. **The Outcome row is satisfied by a non-author operator running the install verification on a fresh system — no AI session can mark this row PASS.**

This file documents the install procedure the operator would follow on a fresh non-author machine to satisfy [G-155](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-155) dim 38 in a real capstone project. The verification rows below are scaffolded — they describe what the verification should record. The actual fresh-system install attempt is **the operator's task** to execute and record; the AI cannot satisfy this gate on the project's behalf because the discipline's whole point is non-author verification.

The AI co-authorship is disclosed here per the operator's directive ("I run everything that can be automated; PROCESS.md authored as 'AI-co-authored reference example' with the operator-voice limitation explicitly disclosed"). The verification rows below describe what the operator's fresh-system install attempt would record; the **Outcome** column is left blank pending the operator's execution. Per [G-155](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-155), leaving the **Outcome** blank means the gate is not yet satisfied; the project's capstone closure is pending operator-executed verification.

---

## Verification procedure

The operator executes the following on a system that has never built or installed `bookmark-cli-manual`. Recommended: a fresh container (`docker run --rm -it rust:1.81-bookworm bash`) or a colleague's machine that has [Rust](https://www.rust-lang.org/) toolchain installed but no familiarity with this project.

### Step 1 — Clone the portfolio

```sh
git clone https://github.com/magnificentlycursed/guild-portfolio.git
cd guild-portfolio/vsdd-suite-reference-examples/bookmark-cli-manual
```

Expected: clone succeeds; directory exists. `ls` shows the project-config files (`Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `deny.toml`, `.gitignore`), the source + test directories (`src/`, `tests/`), the documentation surfaces (`README.md`, `DESIGN.md`, `TODO.md`, `PROCESS.md`, `CHANGELOG.md`), the manual-test plans (`manual-tests/layer-1.md`, `manual-tests/install-verification.md` — this file), and the VSDD audit trail (`vsdd-suite/` containing `FINDINGS-INDEX.md` + per-session files under `review-log/`). The `Cargo.lock`, `rust-toolchain.toml`, and `deny.toml` files exist per [`../DESIGN.md`](../DESIGN.md) § Project-level details (`Cargo.lock` committed for reproducible builds; `rust-toolchain.toml` pins the toolchain; `deny.toml` is the [`cargo deny`](https://github.com/EmbarkStudios/cargo-deny) supply-chain policy). A `target/` directory will appear after Step 2's `cargo install` runs — it is a build artifact, gitignored.

### Step 2 — Install the binary from the project directory

```sh
cargo install --locked --path . --force --quiet
which bm
```

Expected: `cargo install` succeeds; exit code 0; `which bm` resolves to `~/.cargo/bin/bm` (or the equivalent Rust cargo-home path on the verifier's system).

### Step 3 — Run the manual-test plan

Follow [`layer-1.md`](layer-1.md) (sibling file in this directory) end-to-end. Each step (0 through 6 + cleanup) should produce the expected output. Record any divergence as a Platform Engineer finding.

### Step 4 — Record outcomes below

Each row of the table below is one verification attempt. Per [G-155](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-155), a single PASSING row from a non-author on a fresh system is sufficient to satisfy dim 38. Multiple attempts are encouraged (different OSes, different Rust toolchain versions) for stronger verification.

---

## Verification records

| Date (UTC) | Verifier | System (OS / Rust toolchain) | Manual-test steps that PASSED | Manual-test steps that FAILED / details | Outcome | Notes |
|---|---|---|---|---|---|---|
| Thu May 21 07:40:36 PM UTC 2026 | nwhitehead | Ubuntu 24.04.4 LTS / rust 1.95.0 | 0-6 | NONE | PASS | |
| *(pending)* | *(non-author operator)* | *(fresh-system context)* | *(per manual-tests/layer-1.md execution)* | *(divergences, if any)* | *(PASS / FAIL)* | *(any context)* |

**Per [G-155](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-155) / [G-156](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-156) capstone-gate discipline:** a Verification record with **Outcome: PASS** from a non-author on a fresh system is the gate signal. Until at least one PASS row exists, the project's capstone closure is pending Platform Engineer Dim 38.

---

## Coordination with other artifacts

- **[`layer-1.md`](layer-1.md)** — the test plan the verifier executes (sibling file in this `manual-tests/` directory).
- **[`../PROCESS.md`](../PROCESS.md)** — first-person retrospective; the verifier's experience could be recorded as an addendum if the verifier is willing to author a brief reflection on the install experience (cold-reader signal that complements the project's documentation audit trail).
- **[`../vsdd-suite/review-log/`](../vsdd-suite/review-log/)** — Platform Engineer's per-session review-log files (matching `*-platform-engineer.md`) and the [`../vsdd-suite/FINDINGS-INDEX.md`](../vsdd-suite/FINDINGS-INDEX.md) project finding registry. Dim 38 (Fresh-system install verification) per the [Platform Engineer domain prompt](../../../vsdd-suite/domains/role/PLATFORM-ENGINEER-REVIEW.md) is evaluated against this file's contents.
- **[`../DESIGN.md` § Project intent](../DESIGN.md#project-intent)** — declared capstone intent; [G-162](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-162) strategy declarations include the verification commitment.

---

## Layer 2 inheritance note (Layer 2 Round 1 PE F3 disposition)

Per [G-155](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-155) strict reading — "the project has been installed by a third party once" — Layer 2 inherits Layer 1's install-verification PASS row from PR [#41](https://github.com/magnificentlycursed/guild-portfolio/pull/41) (Nathan's 2026-05-21 Ubuntu 24.04 / rust 1.95.0 PASS). The Layer 2 cycle's MVR does NOT require a new install-verification row to ship; the project-as-a-whole has cleared the dim 38 gate once and continues to satisfy the requirement.

**Operator action item (post-Layer-2 merge):** solicit a fresh-system install-verification PASS row for the Layer 2 binary in the post-merge feedback cycle — similar to the PR #41 → [Bluesky thread](https://bsky.app/profile/shimmermathlabs.com) shape that produced Nathan's verification. The Layer 2 cycle inherits the Layer 1 PASS row for shipping purposes, but a Layer-2-specific verification row strengthens the audit trail + catches any Layer 2 install-experience regressions (e.g. the `proptest` dev-dependency may add to the build time the verifier perceives; the new `manual-tests/layer-2.md` Step 12 has the `hyperfine` install prerequisite which is platform-dependent).

Layer 2 Round 1 Platform Engineer F3 framed this as "Operator's call" — the disposition above adopts the inheritance-by-strict-[G-155](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-155)-reading path + queues the operator-driven Layer 2 install verification as a separable post-merge feedback-loop item rather than a Layer 2 MVR blocker.

## Layer 3 inheritance note (Layer 3 Round 1 PFE F3 disposition)

Per [G-155](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-155) strict reading — "the project has been installed by a third party once" — Layer 3 inherits the Layer 1 install-verification PASS row above (Nathan's 2026-05-21 Ubuntu 24.04 / rust 1.95.0 PASS) under the same inheritance shape as the Layer 2 note. Layer 3 adds two new subcommands (`bm export` + `bm import`) that change the installed binary's behavior surface; the `cargo install --locked --path .` install mechanism is unchanged, so the existing PASS row continues to cover the install-procedure-itself dimension. The expanded behavioral surface is exercised by the new `manual-tests/layer-3.md` plan (authored alongside the Layer 3 Phase 1a+1b commit per Round 1 Phase 4 routing) rather than by re-running the install-verification gate.

**Operator action item (post-Layer-3 merge):** solicit a fresh-system install-verification PASS row for the Layer 3 binary in the post-merge feedback cycle — similar to the PR #41 → [Bluesky thread](https://bsky.app/profile/shimmermathlabs.com) shape that produced Nathan's verification. The Layer 3 cycle inherits the Layer 1 PASS row for shipping purposes, but a Layer-3-specific verification row strengthens the audit trail + catches any Layer 3 install-experience regressions (e.g. the new `manual-tests/layer-3.md` Step exercising `bm export | bm import` round-trip has no install-time prerequisites beyond Layer 1's `cargo install` step; verification primarily confirms the new subcommands are present + the round-trip workflow runs cleanly).

Layer 3 Round 1 Platform Engineer F3 framed this as "Operator's call" — the disposition adopts the inheritance-by-strict-[G-155](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-155)-reading path (same as Layer 2) + queues the operator-driven Layer 3 install verification as a separable post-merge feedback-loop item rather than a Layer 3 MVR blocker.
