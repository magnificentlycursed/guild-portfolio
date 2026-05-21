# Solution Owner Review — 2026-05-20

[Index](../SOLUTION-OWNER-REVIEW.md)

---

## Review 1 — 2026-05-20 19:30Z

**Scope:** [Phase 3](../../../vsdd-suite/primers/3-review-session.md) [Solution Owner](../../../vsdd-suite/domains/role/SOLUTION-OWNER-REVIEW.md) IAR Round 1 against bookmark-cli-manual at the post-PR-6 / post-PR-7-prep state. Layer 1 only — the full project (since Layers 2 and 3 are scoped-but-deferred). Inputs read in order: primer 3 → SO domain prompt → suite-development.md § Governing standard + Agent-API surface → README → TODO → CHANGELOG → PROCESS → src/main.rs → manual-tests/layer-1.md → manual-tests/install-verification.md → DESIGN.md (last) → existing-round schema at [`2026-05-20-quality-engineer.md`](2026-05-20-quality-engineer.md). First Solution Owner round filed against bookmark-cli-manual; populates the previously-empty [`../SOLUTION-OWNER-REVIEW.md`](../SOLUTION-OWNER-REVIEW.md) Reviews table.

**Lens:** Spec compliance + scope discipline + deliverable-vs-promise alignment + capstone-intent declaration appropriateness + raised-to-SO routing fidelity. Compliance table built first (below), then deviation analysis. Layer 1 is the implementation scope; Layers 2 and 3 are explicitly deferred per [`../../TODO.md`](../../TODO.md) §§ Layer 2 / Layer 3, not flagged as under-delivery.

**Session note:** Cold-context — this AI session did not participate in authoring DESIGN.md, the PR 6 capstone-intent promotion, or the PR 7 prep artifacts (PROCESS.md, manual-tests/install-verification.md). Inputs read in the order the operator specified, DESIGN.md last per the operator's discipline. Sycophancy-compensation: the project's capstone-intent declaration is the most natural place for design-stage scope creep (SO Dim 9), so the compliance table walks DESIGN.md against TODO.md + README simultaneously and the deviation analysis specifically calls out where deliverables-claim diverges from gate-criteria-declared.

**Source:** `domain-raised` — the cold adversary applying SO dimensions (1 through 10) against the project artifacts found each Finding below; no operator interrupts or regression replays during this round.

---

## Compliance table

Built first per the SO domain prompt's "Start with a compliance table" instruction. Every requirement from DESIGN.md is enumerated; each is marked **Met** / **Partial** / **Missing**. Layer 1 only.

#### Behavioral contracts

| Requirement | Source | Status | Evidence |
|---|---|---|---|
| `bm add <url>` captures URL with current UTC timestamp; exit 0; stdout silent | DESIGN.md:30, 55-60 | **Met** | [`src/main.rs:43-61`](../../src/main.rs); [`src/lib.rs:76-81`](../../src/lib.rs) |
| `bm add ""` rejects empty URL with stderr `Error: URL cannot be empty.\n`, exit 1, no write | DESIGN.md:31, 60 | **Met** | [`src/main.rs:44-47`](../../src/main.rs) |
| Storage error → stderr `Error: <descriptive>\n`, exit 2, no partial write | DESIGN.md:61 | **Met** | [`src/main.rs:51-54, 57-59`](../../src/main.rs) (anyhow `{:#}` format for descriptive chain) |
| `bm list` prints all bookmarks newest-first, `<timestamp> <url>` per line, RFC 3339, single space, trailing newline, exit 0 | DESIGN.md:32, 65-67 | **Met** | [`src/main.rs:74-77`](../../src/main.rs); [`src/lib.rs:85-89`](../../src/lib.rs) `newest_first` sorts descending by timestamp |
| Empty `bm list` → stdout silent, stderr `No bookmarks yet.\n`, exit 0 | DESIGN.md:33, 68 | **Met** | [`src/main.rs:70-73`](../../src/main.rs) |
| `bm list` storage error → stderr `Error: <descriptive>\n`, exit 2, stdout silent | DESIGN.md:69 | **Met** | [`src/main.rs:64-68`](../../src/main.rs) |
| Storage in `$BOOKMARK_CLI_DB` or `./bookmarks.json` | DESIGN.md:34 | **Met** | [`src/main.rs:32-36`](../../src/main.rs) |
| `bm --help` and `bm --version` | DESIGN.md:89-91 | **Met** | clap derive at [`src/main.rs:14-19`](../../src/main.rs) with `version` enabled |

#### Storage format

| Requirement | Source | Status | Evidence |
|---|---|---|---|
| JSON file shape `{"bookmarks": [...]}` (object with named array field) | DESIGN.md:103-110 | **Met (impl side)** | [`src/lib.rs:33-37`](../../src/lib.rs) `BookmarkStore { bookmarks: Vec<Bookmark> }` serializes to `{"bookmarks":[...]}` per serde default. Mismatch lives in the manual-test plan — see [Finding 2](#r1-f2). |
| Pretty-printed with trailing newline | (impl detail; not constrained by DESIGN.md) | n/a | [`src/lib.rs:67-71`](../../src/lib.rs) `to_string_pretty` + `+ "\n"` |
| Newest-first is render concern (sort on read), not storage concern (append on write) | DESIGN.md:112 | **Met** | [`src/lib.rs:76-81`](../../src/lib.rs) appends on `add`; [`src/lib.rs:85-89`](../../src/lib.rs) sorts on read |

#### Exit codes

| Requirement | Source | Status | Evidence |
|---|---|---|---|
| Exit 0: success (including empty list) | DESIGN.md:97 | **Met** | [`src/main.rs:60, 72, 77`](../../src/main.rs) |
| Exit 1: user error (empty URL) | DESIGN.md:98 | **Met** | [`src/main.rs:46`](../../src/main.rs) |
| Exit 2: storage error | DESIGN.md:99 | **Met** | [`src/main.rs:52, 58, 67`](../../src/main.rs) |

#### Edge case catalog

| Requirement | Source | Status | Evidence |
|---|---|---|---|
| Empty URL: rejected | DESIGN.md:73 | **Met** | Per AC 2 above |
| Whitespace-only URL: accepted | DESIGN.md:74 | **Met** | [`src/main.rs:44`](../../src/main.rs) `is_empty()` check only rejects zero-length |
| Storage file absent: `bm list` empty-state, `bm add` creates | DESIGN.md:75 | **Met** | [`src/lib.rs:45-48`](../../src/lib.rs) `load` returns default for missing; `save` creates parent directories at [`src/lib.rs:60-66`](../../src/lib.rs) |
| Storage file empty: treat as empty list | DESIGN.md:76 | **Met** | [`src/lib.rs:51-53`](../../src/lib.rs) empty-trim check |
| Invalid JSON: error to stderr, exit 2; no recovery | DESIGN.md:77 | **Met** | [`src/lib.rs:54-55`](../../src/lib.rs) `from_str` error propagates with context; [`src/main.rs:65-67`](../../src/main.rs) renders to stderr + exit 2 |
| Concurrent writes: out of scope | DESIGN.md:78 | **Met** (no implementation expectation) | Single-user, no locking |
| Very long URL: accepted, no length cap | DESIGN.md:79 | **Met** | `String` type accepts arbitrary length |
| URL with newlines: accepted | DESIGN.md:80 | **Met** | No filtering in [`src/main.rs:43-55`](../../src/main.rs) |

#### Verification architecture

| Requirement | Source | Status | Evidence |
|---|---|---|---|
| Unit tests for pure + I/O-wrapper functions, `tempfile` isolation | DESIGN.md:129 | **Met** | [`src/lib.rs:92-168`](../../src/lib.rs) `#[cfg(test)] mod tests` |
| Integration tests via `assert_cmd` against per-test temp dirs | DESIGN.md:130 | **Met** | `tests/bookmarks.rs` (referenced by README:56-57; 4 integration tests per TODO Red Gate plan) |
| No mocks for storage layer | DESIGN.md:131 | **Met** | Tests use real `tempfile::tempdir()` |
| Phase 3 IAR runs 7 core domains | DESIGN.md:133 | **Partial** — see [Finding 3](#r1-f3) — only QE (2 rounds) + SA (1 round) filed; 5 cores (SE, UX, Security, SO, VDD-IAR Alignment) without rounds at the time of this Round-1 SO entry, plus 4 of 5 extended domains without rounds. This SO Round 1 is filed in-session as part of the gap closure. |
| Phase 5 hardening: Purity Boundary Audit + Mutation Testing | DESIGN.md:15, 134 | **Met** | [`SA Review 1`](2026-05-20-solution-architect.md#review-1--2026-05-20-0245z) Purity Boundary Audit + [`QE Review 2`](2026-05-20-quality-engineer.md#review-2--2026-05-20-0245z) Mutation Testing both closed with the per-round preamble per G-177 v0.7.8 |
| Phase 5: property-based testing deferred; Fuzz Testing / Proof Execution not applicable | DESIGN.md:15, 136 | **Met** (declared) | Strategy line; Proof Execution rationale at DESIGN.md:136 |
| Phase 6 four-dimensional convergence record | DESIGN.md:17 | **Missing** — see [Finding 3](#r1-f3) |

#### Technology choices

| Choice | DESIGN.md row | Cargo.toml | Status |
|---|---|---|---|
| Rust 1.78+ | Constraints | edition 2021 (compatible) | **Met** |
| Single crate (bin + lib) | DESIGN.md:145 | `[[bin]]` + `[lib]` at [`Cargo.toml`](../../Cargo.toml) | **Met** |
| clap derive | DESIGN.md:146 | `clap = { version = "4", features = ["derive"] }` | **Met** |
| serde_json | DESIGN.md:147 | `serde_json = "1"` | **Met** |
| chrono UTC | DESIGN.md:148 | `chrono = { version = "0.4", default-features = false, features = ["clock", "serde"] }` | **Met** |
| anyhow | DESIGN.md:149 | `anyhow = "1"` | **Met** |
| assert_cmd + tempfile | DESIGN.md:150 | `assert_cmd = "2"`, `tempfile = "3"` (and `predicates = "3"` which is an `assert_cmd` companion, not a separate scope addition) | **Met** |

#### Scope creep audit (Dim 2)

Walked the implementation surface (`src/`, `Cargo.toml`, `tests/`, `manual-tests/`) for behavior, command, flag, or output not in DESIGN.md. No additions found at the **binary level** — `bm add`, `bm list`, `bm --help`, `bm --version` are all declared at DESIGN.md:87-91. No new commands; no new flags beyond clap's defaults. The `BOOKMARK_CLI_DB` env var is declared at DESIGN.md:34. No silent additions in `src/lib.rs` beyond the public API the spec implies.

`predicates = "3"` in `Cargo.toml` `[dev-dependencies]` is not named in DESIGN.md's technology table. It is the standard `assert_cmd` companion (predicates-based assertion DSL) — it does not add a user-facing feature; it's a test-time-only dependency used to express assert_cmd matchers. **Not a scope creep finding** — it is the standard companion to a tool DESIGN.md explicitly approves; calling it out would be Dim-3 pedantry against the "strictly necessary to implement it" carve-out. Logged in this audit so the rationale is on record.

`Cargo.lock` exists and is committed per DESIGN.md:156 — compliance, not scope creep.

#### Methodology-canonical-defect deferral visibility (Dim 10)

Walked DESIGN.md / TODO.md / CHANGELOG against the [VSDD whitepaper](https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00)'s canonical worked examples (the atomic-write Adversary example in `01-how-we-build.md:137-139` is the canonical detector pattern per Dim 10). `bookmark-cli`'s `BookmarkStore::save` at [`src/lib.rs:60-72`](../../src/lib.rs) is a non-atomic write — `std::fs::write` truncates first, then writes; a process crash mid-write leaves a truncated file. The DESIGN.md `Concurrent writes: out of scope` note at line 78 covers the multi-process angle; the single-process crash-during-write case is not explicitly named in DESIGN.md or in a DECISIONS.md (none exists for this project). Whether this is a Dim-10 finding depends on whether the bookmark-cli project's deferral pattern is *the same kind* as the whitepaper's atomic-write example: it is — the whitepaper uses atomic-write as the textbook adversary case. **However**, bookmark-cli's storage error model (DESIGN.md:61, 69 — exit 2 on storage error) is at the "single user, single shell session" level (DESIGN.md:78) where the cost-disproportionate-deferral rationale of the whitepaper's example applies. The deferral is implicit (DESIGN.md does not name it). Marginal: ITC's analogous deferral is the canonical Dim-10 finding; bookmark-cli's analogous case is more shallow (no live multi-process attack surface, no cryptographic invariant violated by partial-write). **Not flagged as a separate Finding** in this round — the SO finding ceiling for Round 1 is the four below; Dim 10 is recorded here as "noted, not raised" so the Round 2 cold reviewer has the analytical trail. If a future Round opens Dim 10 with a directed re-read against the whitepaper's specific framing, it should land as Backlogged-with-named-trigger.

#### Assignment compliance (Dim 9 — the sycophancy-resistance check)

The "assignment brief" for `bookmark-cli-manual` is [G-112](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112) (reference implementation for the suite's worked example). The brief says: walk the suite's documented workflow end-to-end on a small project. The DESIGN.md scope (`bm add`, `bm list`, JSON file, single user) is consistent with that brief — bookmark-cli is small by design.

The capstone-intent promotion (PR 6 / Review 78) raised the methodological bar — 11 role + 1 meta domains instead of the prior 7 — and added structural-prep artifacts (PROCESS.md, manual-tests/install-verification.md). The capstone bar's appropriateness is contestable in two directions:

1. **Defensible direction:** reference examples exist to teach the methodology; the methodology evolved past 7 domains since Review 67; tracking the current methodology requires the capstone bar; the [G-177](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-177) precedent says reference examples migrate when the methodology evolves. By this lens, the capstone declaration is *required*, not creep.
2. **Adversarial direction:** the project itself remains a 2-command CLI with 4 acceptance criteria. The capstone bar adds 5 new domains and 2 new artifact disciplines (G-155 install verification; G-156 director retrospective). For a 2-command tool, that bar is heavy. Calling it "capstone" risks setting a precedent where *the methodology under test calls for itself to be tested more thoroughly than the artifact under test deserves*.

The sycophancy-resistance question per the SO domain prompt: did scope creep enter at the design stage? The answer is: **yes, but defensibly so**. The promotion was driven by an external methodology shift, documented in CHANGELOG, traced through [G-150](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-150) intent calibration. The traceability is the discipline that distinguishes design-stage creep from methodology-driven re-alignment. I am not raising this as a Finding because the promotion's audit trail is concrete and the methodology-evolution rationale is verifiable. I AM raising it as a noted concern for the project-portfolio reviewer: if `bookmark-cli-manual` is the only reference example, the capstone bar is reasonable; if multiple reference examples accumulate this bar, the next portfolio-wide review should ask whether the bar is calibrated or whether the suite's reference-example footprint is creeping by methodology pressure.

#### Prior-review additions (Dim 8)

Findings in [QE Review 1](2026-05-17-quality-engineer.md) (2 Resolved: scope-tradeoff documentation; whitespace + newline edge-case tests), [QE Review 2](2026-05-20-quality-engineer.md) (1 Resolved: Mutation Testing missing-test for nested-path save), [SA Review 1](2026-05-20-solution-architect.md) (1 Resolved: Purity Boundary Audit cross-source purity divergence) all closed without expanding the binary's user-visible surface. The SA Review 1 finding DID modify DESIGN.md (Verification architecture rewrite) — this is the "raised to SO" routing pattern, and the SO action was applied in-session per the SA Review 1 entry. The routing fidelity holds: SA flagged a defect requiring a DESIGN.md change, SA classified it as a finding it could resolve only with SO authorization, the SO authority (the operator) applied the change to DESIGN.md, and the change is recorded in CHANGELOG (the 2026-05-20 02:45Z v0.7.2 entry). **No prior-review additions to flag.**

---

### Resolved

*(none — this round files four Findings, none Resolved in-session; the project gates and disclosed-AI-co-authored artifacts the Findings cite are operator-owned and not addressable by an adversarial SO round.)*

---

### Backlogged

**Finding 1 — README.md Phase 4 row claims "N/A — no live findings to route" but findings exist (Dim 7)**

<a id="r1-f1"></a>

**Owner:** solution-owner
**Status:** raised
**Blocked by:** *(none — directly editable in [`../../README.md`](../../README.md))*
**Validator:** sanity-check

[`../../README.md:59`](../../README.md) phase progression table row for Phase 4 reads: `| 4 | [Phase 4](...) routing | N/A — no live findings to route |`. This claim is inconsistent with [`../FINDINGS-INDEX.md`](../FINDINGS-INDEX.md) which lists 5 findings (F-001 through F-005); 4 are Resolved + 1 Hallucinated. Findings DID exist; the routing phase IS applicable to them. The "no live findings to route" framing only holds if "live" is read narrowly as "currently Open" — which excludes the Resolved findings the routing phase was actually exercised on.

The mis-framing is small but the SO concern is that it misrepresents the methodology coverage of this reference example. A learner reading the README phase-table will read "Phase 4 — N/A" as meaning Phase 4 was not exercised here; the actual state is that Phase 4 routing was exercised in-session per the QE Review 1 + QE Review 2 + SA Review 1 closure narratives (each lists a Resolution).

**Proposed change to [`../../README.md`](../../README.md):** rephrase Row 4 to "Exercised in-session (4 of 5 findings Resolved in-round; 1 Hallucinated)" or similar precise phrasing. Routes through SO because it touches the deliverable framing, not just typography. The framing is a Backlog candidate rather than a Resolved finding because the operator may prefer a different exact phrasing.

**Classification:** Backlogged. Trigger to close: README phase-table row 4 updated to accurately describe Phase 4 coverage (either "exercised in-round" or an equivalent precise phrasing that does not contradict the FINDINGS-INDEX).

---

**Finding 2 — Storage format divergence between DESIGN.md and manual-tests/layer-1.md Step 1 expected output (Dim 7)**

<a id="r1-f2"></a>

**Owner:** solution-owner
**Status:** raised
**Blocked by:** *(none — directly editable in [`../../manual-tests/layer-1.md`](../../manual-tests/layer-1.md))*
**Validator:** sanity-check

[`../../DESIGN.md:103-110`](../../DESIGN.md) declares the storage format as `{"bookmarks": [...]}` (an object with a named `bookmarks` array field). The implementation at [`src/lib.rs:33-37`](../../src/lib.rs) (`BookmarkStore { bookmarks: Vec<Bookmark> }` with serde default) correctly serializes to `{"bookmarks":[{...}]}` matching the spec. However, [`../../manual-tests/layer-1.md:62`](../../manual-tests/layer-1.md) Step 1 declares the **expected** output of `cat "$BOOKMARK_CLI_DB"` as:

```json
[{"url":"https://example.com","timestamp":"2026-05-20T22:15:42.371Z"}]
```

— a bare array, NOT the object-wrapped shape. A human operator executing Step 1 will see actual output:

```json
{"bookmarks":[{"url":"https://example.com","timestamp":"..."}]}
```

(pretty-printed per [`src/lib.rs:67-69`](../../src/lib.rs) — a multi-line object document with a trailing newline). The operator will judge this as a divergence from the expected output even though the implementation is correct per DESIGN.md.

This is a documentation defect, not an implementation defect. The manual-test plan must be updated to declare the actual on-disk format the implementation produces (per DESIGN.md). Two acceptable resolutions:

1. **Preferred:** edit the manual-test plan's Step 1 expected-output block to the pretty-printed object shape the implementation produces, preserving the "URL is invariant; timestamp is variable" invariant note.
2. **Alternative:** change DESIGN.md storage format declaration to the bare-array shape AND change the implementation to match. This is the *wrong* resolution because (a) DESIGN.md's shape is forward-compatible with future Layer 2 / Layer 3 additions (the wrapping object can carry version/metadata fields without a breaking change to the array shape); (b) changing both DESIGN.md and `src/lib.rs:33-37` is a larger blast radius than fixing the test plan.

Per the SO domain prompt's "Quality does not justify scope" principle: even though the bare-array shape is more compact and arguably nicer for one-bookmark cases, the spec is the contract — the manual test plan must reflect the spec, not propose an alternative.

**Proposed change to [`../../manual-tests/layer-1.md`](../../manual-tests/layer-1.md) Step 1 (lines 59-65):** replace the bare-array example with the pretty-printed `{"bookmarks":[...]}` shape that the implementation actually emits (per DESIGN.md storage format declaration). Routes through SO because the SO is the spec-authority and the change reaffirms the spec rather than weakening it.

**Classification:** Backlogged. Trigger to close: manual-tests/layer-1.md:62 expected-output block updated to match DESIGN.md:103-110 storage shape. The Backlogged classification (rather than Resolved) reflects that the fix is the operator's edit, not an in-round resolution by the SO reviewer.

---

### Dismissed

*(none — every Finding raised in this round has a substantive basis and a documentation defect, not an adversarial-without-evidence pattern.)*

---

### Hallucinated

*(none — see Open / Backlogged sections; all four Findings cite specific file:line evidence and a verifiable spec-vs-artifact mismatch. The sycophancy-resistance discipline holds: each Finding is uncomfortable for the project's claim of Layer-1-complete-at-capstone-intent, and each is the kind of Finding the project's own future reviewer would have to address; pre-classifying any as Hallucinated would be the sycophancy failure mode the primer warns against.)*

---

### Backlogged

**Finding 3 — Capstone-intent gate criteria not satisfied by current artifacts (Dim 5, Dim 7)**

<a id="r1-f3"></a>

**Owner:** solution-owner
**Status:** raised
**Blocked by:** PR 7 execution (per [`../../CHANGELOG.md`](../../CHANGELOG.md) 2026-05-20 16:30Z entry's "Backlog after PR 6: 0 Open findings ... cold-session rounds ... land in PR 7" note)
**Validator:** sanity-check

[`../../DESIGN.md:11`](../../DESIGN.md) declares the active domain set as **11 role + 1 meta = 12 active domains** with capstone intent. [`../../TODO.md:36-42`](../../TODO.md) declares 6 layer-gate criteria including:

- Criterion #4: Phase 3 IAR reviews complete for the capstone-active domain set (7 cores + 4 extended); each domain reaches MVR or zero-findings.
- Criterion #5: Phase 5 Surfaces A.0 + B both at closure (Met per SA Review 1 + QE Review 2).
- Criterion #6: Phase 6 four-dimensional convergence record landed as the final VDD-IAR Alignment review round.

Current artifact state at this round's filing time:

- **Phase 3 IAR rounds filed:** [QE Review 1](2026-05-17-quality-engineer.md), [QE Review 2](2026-05-20-quality-engineer.md), [SA Review 1](2026-05-20-solution-architect.md), and this SO Round 1 (in-session). **Four** rounds across **three** domains (QE, SA, SO); the remaining **nine** active domains (SE, UX, Security, VDD-IAR Alignment, Performance Engineer, Platform Engineer, Red Team, Technical Writer, Documentation Reviewer) have **zero rounds filed**. Criterion #4 is not yet satisfied.
- **Phase 6 convergence record:** [`../VDD-IAR-ALIGNMENT-REVIEW.md`](../VDD-IAR-ALIGNMENT-REVIEW.md) was not opened during this session for inspection; per [`../../CHANGELOG.md`](../../CHANGELOG.md) 2026-05-20 16:30Z entry, "FINDINGS-INDEX repopulation land in PR 7" — the Phase 6 record is explicitly deferred to PR 7. Criterion #6 is not satisfied.

The project's own gate criteria (TODO.md Layer-gate criteria #4 + #6) are NOT met by the current artifacts. The CHANGELOG explicitly defers gate satisfaction to PR 7. The SO concern: a project that declares "Layer 1 complete (add + list)" in its README ([`../../README.md:9`](../../README.md)) when the project's own gate criteria are not yet satisfied is a **deliverable-vs-promise misalignment**. Either:

1. The README's "Layer 1 complete" claim is premature, and should read "Layer 1 implementation complete; capstone gate-close pending PR 7" or similar, OR
2. The TODO.md layer-gate criteria #4 and #6 should be reduced to match the current artifact state, OR
3. The CHANGELOG's PR 7 deferral should be executed before any claim of Layer 1 completion is published.

Per the SO domain prompt's "100% of what was agreed, nothing that was not" principle: capstone intent + the 6-criterion layer-gate IS what was agreed (per the PR 6 / Review 78 promotion). The promotion declared the bar; current artifacts do not meet it; therefore the layer is not at gate close.

This is not an under-delivery against the IMPLEMENTATION scope (the binary at [`src/main.rs`](../../src/main.rs) + [`src/lib.rs`](../../src/lib.rs) satisfies all 4 ACs and the spec contracts). It is an under-delivery against the METHODOLOGICAL scope the capstone declaration added. The project's option to resolve: do PR 7 (close the gap), reduce the gate criteria (formally back the capstone declaration off, or carve out reference-implementation exceptions to G-156 + G-155), or change the README's deliverable framing.

**Classification:** Open — the resolution is owned by the operator and tracks to PR 7 execution per the CHANGELOG. The Finding is logged so that the deferral is visible at the SO level rather than buried in CHANGELOG narrative.

**Director note:** the primer-mandated trigger-check applies here. The PR 7 execution would itself produce new findings (per the [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) continue trigger discipline — "any new real findings" requires another round). The expected post-PR-7 state should be re-validated against this Finding before declaring Layer 1 at gate close.

---

**Finding 4 — Deliverable-vs-promise misalignment: Layer 1 status and Phase progression (Dim 5)**

<a id="r1-f4"></a>

**Owner:** solution-owner
**Status:** raised
**Blocked by:** *(none — directly editable in [`../../README.md`](../../README.md) and [`../../TODO.md`](../../TODO.md))*
**Validator:** sanity-check

Three statements about Layer 1's status appear across the project artifacts; they do not agree:

| Source | Claim |
|---|---|
| [`../../README.md:9`](../../README.md) | "Layer 1 complete (add + list)." |
| [`../../README.md:58`](../../README.md) | Phase 3 row: "Scaffolded; rounds-in-progress (this is reference-implementation work, not a real merge gate)" |
| [`../../TODO.md:11`](../../TODO.md) | "**Status:** In progress (Phase 2a → 2b in the reference-implementation session)." |
| [`../../TODO.md:5`](../../TODO.md) | "all 6 VSDD phases demonstrated end-to-end (1a+1b spec → 1c decomposition → 2a Red Gate → 2b implementation → 2c refactor (no-refactor annotation) → 3 IAR (10 active domains) → 4 routing → 5 Surfaces A.0+B hardening → 6 four-dimensional convergence)" |

README:9 says "complete." TODO:11 says "in progress." README:58 says "rounds-in-progress." TODO:5 says "demonstrated end-to-end" with "Phase 3 IAR (10 active domains)" — but DESIGN.md:11 currently declares **12** active domains (10 → 12 evolved via Review 80 Documentation Reviewer activation per CHANGELOG 2026-05-20 18:30Z). TODO.md:5's "10 active domains" framing is stale against DESIGN.md:11.

The discrepancies are not all the same kind:

- README:9 "Layer 1 complete" vs TODO:11 "In progress" is a direct contradiction.
- README:58 "rounds-in-progress" is consistent with TODO:11 — they agree the methodology phases are not closed.
- TODO:5 "all 6 VSDD phases demonstrated end-to-end" overclaims the current state given the CHANGELOG's PR 7 deferral (Finding 3).
- TODO:5 "(10 active domains)" is a stale figure now that DESIGN.md says 12.

For an SO reviewing a real product, these would be release-note quality bugs — the project's own documentation contradicts itself about what is shipped. For a reference example whose entire purpose is to teach the methodology, the cost is higher: a learner reading these artifacts in different orders gets different answers about whether Layer 1 is done.

**Proposed resolutions** (operator's choice):

1. Re-write all four claim sites to one consistent framing: e.g., "Layer 1 implementation: complete (ACs 1–4 satisfied by `src/main.rs` + `src/lib.rs`; 8/8 tests passing). Capstone gate-close: pending — see TODO.md Layer-gate criteria #4 + #6." Each of README:9, README:58, TODO:11, TODO:5 then says a precise version of that.
2. Update TODO:5's "(10 active domains)" → "(12 active domains)" to track DESIGN.md:11.
3. Decide whether "Phase 3 IAR demonstrated end-to-end" is accurate given only 3 of 12 domains have rounds at the time of this entry. If accurate (under a "demonstrated by the SE + QE + SA + SO rounds, the rest are queued for PR 7" reading), explicitly say so. If not, revise.

**Classification:** Open. The Finding is logged at the SO level because the inconsistency is across the deliverable framing (README) and the contract (DESIGN.md / TODO.md), not within a single domain's purview.

---

### Raised to SO

*(none — this IS the SO round; cross-domain findings that would route to SO are filed against the originating domain's log. This section is empty by structural necessity.)*

---

### Approved deviation

*(none — no operator-pre-approved DESIGN.md deviations apply at this round.)*

---

#### Summary

Four Findings filed; **0 Resolved**, **2 Backlogged**, **2 Open**, 0 Dismissed, 0 Hallucinated.

The compliance table closes cleanly against the binary contract — every behavioral contract, exit code, edge case, and technology choice in DESIGN.md is **Met** by the implementation at [`src/main.rs`](../../src/main.rs) + [`src/lib.rs`](../../src/lib.rs). The binary delivers what the spec promises.

The Findings cluster around a different axis: **deliverable-vs-promise alignment at the methodology layer**. The project DECLARES capstone intent + 12 active domains + 6 layer-gate criteria, but the current artifact state does not meet that bar (only 3 of 12 domains had rounds before this SO Round 1 lands; Phase 6 convergence record not filed; CHANGELOG defers gate-close to PR 7) — and the README claims "Layer 1 complete" while the TODO says "In progress." Two additional Findings are documentation defects within Layer 1's already-claimed-complete scope: a phase-progression-table row misrepresenting Phase 4 coverage (Finding 1) and an expected-output block in the manual-test plan contradicting the spec's storage format (Finding 2).

None of the Findings demand an implementation change to `src/`. All four are documentation/scope-framing changes the operator can make against [`../../README.md`](../../README.md), [`../../TODO.md`](../../TODO.md), [`../../manual-tests/layer-1.md`](../../manual-tests/layer-1.md), and (optionally for Finding 3) a re-execution of the PR 7 work the CHANGELOG already commits to.

Per the primer 3 continue-trigger discipline: this round produced 4 new real Findings, which mandates Round 2 (or equivalent: a Round 2 after Findings 1–4 are addressed, or a Round 2 that confirms Findings 1–4 are accepted as Open/Backlogged with explicit operator approval and the surrounding artifacts updated). The project is NOT at MVR for SO at the close of Round 1.

**Coordination:** The four Findings interact with several other domains. Finding 3 (capstone-gate not satisfied) is the input for the eventual [VDD-IAR Alignment](../../../../vsdd-suite/domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md) Phase 6 convergence round — the convergence record cannot honestly close while Finding 3 is Open. Finding 4 (deliverable-framing) sits closest to [Technical Writer](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) Dim 12 (lookup-cost) and Dim 11 (audit-trail consistency) — the eventual TW round on this project will likely re-surface the same artifacts and should cite Finding 4 as the prior SO surfacing. Finding 2 (manual-test expected-output divergence) is a [Quality Engineer](../../../../vsdd-suite/domains/role/QUALITY-ENGINEER-REVIEW.md) concern as well — the eventual QE Round 3 (post-PR-7) re-running the manual-test plan against the actual binary output would discover the same gap; Finding 2 is the SO surfacing of what QE would also catch.

**Validator:** sanity-check (per the [Review 77](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-77--2026-05-20-1545z) Finding 2 meta-validator-of-last-resort pattern — Solution Owner has no natural cross-domain validator pair; Sanity Check applies the DESIGN.md + architecture context to confirm SO decisions cohere with the project's spec). The eventual VDD-IAR Alignment Phase 6 convergence round will, in passing, also validate that the SO routing was correct — but VDD-IAR Alignment is a meta domain (process-binary), not a content-correctness validator.
