# Manual Testing — Install Verification

Third-party install verification record per G-155 Platform Engineer Dim 38 (Fresh-system install verification, capstone / production intent only).

**File location convention (Review 78 Finding 2):** install-verification IS a manual test (a human runs commands on a real fresh system and records what they observed), so this file lives in `manual-tests/` alongside the per-layer test plans. The file name is lowercase + hyphenated (`install-verification.md`) to match the per-layer pattern (`layer-N.md`).

---

## AI-co-authored reference-example disclosure

**This install-verification record is AI-co-authored.** G-155 dim 38 specifies that the install verification be performed by **a non-author on a fresh system** — that is the discipline's load-bearing requirement. The AI agent that built `bookmark-cli-manual` and authored this file is by definition the author; the AI's environment is not a fresh non-author system.

This file documents the install procedure the operator would follow on a fresh non-author machine to satisfy G-155 dim 38 in a real capstone project. The verification rows below are scaffolded — they describe what the verification should record. The actual fresh-system install attempt is **the operator's task** to execute and record; the AI cannot satisfy this gate on the project's behalf because the discipline's whole point is non-author verification.

The AI co-authorship is disclosed here per the operator's directive ("I run everything that can be automated; PROCESS.md authored as 'AI-co-authored reference example' with the operator-voice limitation explicitly disclosed"). The verification rows below describe what the operator's fresh-system install attempt would record; the **Outcome** column is left blank pending the operator's execution. Per G-155, leaving the **Outcome** blank means the gate is not yet satisfied; the project's capstone closure is pending operator-executed verification.

---

## Verification procedure

The operator executes the following on a system that has never built or installed `bookmark-cli-manual`. Recommended: a fresh container (`docker run --rm -it rust:1.81-bookworm bash`) or a colleague's machine that has Rust toolchain installed but no familiarity with this project.

### Step 1 — Clone the portfolio

```sh
git clone https://github.com/magnificentlycursed/guild-portfolio.git
cd guild-portfolio/vsdd-suite-reference-examples/bookmark-cli-manual
```

Expected: clone succeeds; directory exists; `ls` shows `Cargo.toml`, `DESIGN.md`, `TODO.md`, `src/`, `tests/`, `manual-tests/` (including `manual-tests/install-verification.md` — this file), `vsdd-suite/`, `PROCESS.md`.

### Step 2 — Install the binary from the project directory

```sh
cargo install --path . --force --quiet
which bm
```

Expected: `cargo install` succeeds; exit code 0; `which bm` resolves to `~/.cargo/bin/bm` (or the equivalent Rust cargo-home path on the verifier's system).

### Step 3 — Run the manual-test plan

Follow [`manual-tests/layer-1.md`](manual-tests/layer-1.md) end-to-end. Each step (0 through 5 + cleanup) should produce the expected output. Record any divergence as a Platform Engineer finding.

### Step 4 — Record outcomes below

Each row of the table below is one verification attempt. Per G-155, a single PASSING row from a non-author on a fresh system is sufficient to satisfy dim 38. Multiple attempts are encouraged (different OSes, different Rust toolchain versions) for stronger verification.

---

## Verification records

| Date (UTC) | Verifier | System (OS / Rust toolchain) | Manual-test steps that PASSED | Manual-test steps that FAILED / details | Outcome | Notes |
|---|---|---|---|---|---|---|
| *(pending)* | *(non-author operator)* | *(fresh-system context)* | *(per manual-tests/layer-1.md execution)* | *(divergences, if any)* | *(PASS / FAIL)* | *(any context)* |

**Per G-155 / G-156 capstone-gate discipline:** a Verification record with **Outcome: PASS** from a non-author on a fresh system is the gate signal. Until at least one PASS row exists, the project's capstone closure is pending Platform Engineer Dim 38.

---

## Coordination with other artifacts

- **[`manual-tests/layer-1.md`](manual-tests/layer-1.md)** — the test plan the verifier executes.
- **[`PROCESS.md`](PROCESS.md)** — first-person retrospective; the verifier's experience could be recorded as an addendum if the verifier is willing to author a brief reflection on the install experience (cold-reader signal that complements TW Dim 11 audit trail).
- **`vsdd-suite/PLATFORM-ENGINEER-REVIEW.md`** — Platform Engineer's per-domain index. Dim 38 (Fresh-system install verification) is evaluated against this file's contents.
- **[`DESIGN.md` § Project intent](DESIGN.md#project-intent)** — declared capstone intent; G-162 strategy declarations include the verification commitment.
