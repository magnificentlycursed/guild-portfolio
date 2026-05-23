# Technical Writer Review — 2026-05-22

---

## Review 5 — 2026-05-22 00:35Z

**Phase:** 3 (IAR Round 1; Layer 2 — first cold-session round on the Layer 2 artifact).
**Source:** domain-raised (the standard TW dimensions + the Rust supplement § Technical Writer floor raised every finding below; Layer 1 prior reviews referenced for regression-check only).
**Lens:** clone-and-follow audit-trail (Dim 1 README completeness; Dim 9 Knowledge transfer test) + documentation drift (Dim 2 documentation accuracy; regression-check on Layer 2 spec promotion) + cross-file consistency (DESIGN.md ↔ TODO.md ↔ src/* ↔ manual-tests/* ↔ tests/*) + `bm --help` text accuracy (Dim 6 API and interface documentation).
**Scope:** all forward-facing documentation artifacts post-Layer-2-landing — [`README.md`](../../README.md), [`DESIGN.md`](../../DESIGN.md), [`TODO.md`](../../TODO.md), [`PROCESS.md`](../../PROCESS.md), [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md), [`manual-tests/layer-2.md`](../../manual-tests/layer-2.md), [`manual-tests/install-verification.md`](../../manual-tests/install-verification.md), the Layer 2 source docstrings in [`src/lib.rs`](../../src/lib.rs) + [`src/main.rs`](../../src/main.rs), the Layer 2 test file docstring + per-test docstrings in [`tests/bookmarks.rs`](../../tests/bookmarks.rs), and `bm --help` output (mentally rendered from the clap derive attributes in [`src/main.rs:28-52`](../../src/main.rs)).
**Reviewer:** Technical Writer.
**Model:** Sonnet 4.6 (per [`DESIGN.md` § Cold-session budget](../../DESIGN.md) model assignment).
**Cold-session shape:** QE/Security/Technical-Writer cluster (shared with Quality Engineer + Security; the natural validator pair — [Documentation Reviewer](../../../../vsdd-suite/domains/role/DOCUMENTATION-REVIEWER-REVIEW.md) — is in Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster, structurally separated per the adversarial-pair separation discipline).
**Session note:** Cold session — this QE/Security/Technical-Writer cluster agent was spawned with no prior project context; read artifacts in the prescribed cold-reader order (README + manual-tests + TODO.md first; DESIGN.md last per cold-reader discipline). Sycophancy-compensation per the [TW domain prompt § Sycophancy check](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md): the "Layer 2 promotion was a clean operator-directive event; the narrative surfaces will follow uniformly" framing was kept as a hypothesis to verify. The verification found that the narrative surfaces DID propagate at the per-domain review-log level and at the per-test-file level, but the cross-file narrative coherence in the README + the install-verification gate + the spec citations was NOT propagated uniformly — the propagation gap is the cold-pass surface. A warm-context reviewer would likely have rationalized "Layer 2 PR landed cleanly" into "all surfaces coherent" and missed the four Raised findings + the one Deferred.
**Regression-check against:** [TW Review 1+2+3](2026-05-20-technical-writer.md) (Layer 1 first-pass + Round 2 + Round 3 — all findings either Resolved or routed; Layer 1 closure at PR #42); [TW Review 4](2026-05-21-technical-writer.md#review-4--2026-05-21-2030z) (PR #42 external-feedback mining cluster — 3 findings all Resolved inline; file-inventory + Sycophancy-compensation-leak + README Phase-3-row staleness).
**Cost-tally:** QE/Security/Technical-Writer cluster agent budget ~50-80k tokens per [AI Engineer R1](2026-05-21-ai-engineer.md#review-1--2026-05-21-1000z) cluster-batching discipline; 5 findings filed in this round yields ~10-16k tokens/finding — within the capstone-intent expected band.

**Assumption surfacing.** Verified [`README.md`](../../README.md) line 9 still says "**Layer 1 complete**" (current state at time of cold pass) — see [Finding 1](#r5-tw-f1) below. Verified [`tests/scaling.rs`](../../tests/) does NOT exist on the filesystem — see [Finding 2](#r5-tw-f2) below (cross-cuts with QE Review 4 Finding 1). Verified [`manual-tests/install-verification.md:43`](../../manual-tests/install-verification.md) Step 3 still says "Follow [`layer-1.md`](layer-1.md)" with no reference to `layer-2.md` — see [Finding 3](#r5-tw-f3) below. Verified [`README.md`](../../README.md) Prerequisites section does NOT mention `hyperfine` despite [`manual-tests/layer-2.md:444-452`](../../manual-tests/layer-2.md) Step 12 installing it via `brew`/`apt`/`cargo install` — see [Finding 4](#r5-tw-f4) below. Verified [`TODO.md:92`](../../TODO.md) Layer-gate criterion #6 cites the Phase 6 four-dimensional convergence record by name but does NOT link the planned location — see [Finding 5](#r5-tw-f5) below.

**Regression check.** Prior TW rounds' findings re-verified against the post-Layer-2 state:
- TW R1 F2 (README angle-bracket placeholders) — fixed in PR #38 R3 + PR #40; still fixed.
- TW R1 F3 (stale primer link in DESIGN.md) — fixed; still fixed.
- TW R2 F7+F8 (PROT_37 + DESIGN.md H1 links) — fixed; still fixed.
- TW R3 F3+F4+F5 (per-domain-index retirement + UPPERCASE placeholders + duplicate-name sweep) — Resolved post-PR-#40; still fixed.
- TW R4 F1 (file-inventory in install-verification.md) — fixed in PR #42; still fixed (the install-verification.md Step 1 file inventory is current as of Layer 1, though the Layer 2 promotion did not add new files at the repo root — `manual-tests/layer-2.md` IS a new file, see [Finding 3](#r5-tw-f3) below for the Layer-2-related variant).
- TW R4 F2 (`Sycophancy-compensation reminder` line in `manual-tests/layer-1.md`) — line deleted; still fixed.
- TW R4 F3 (README Phase 3 row stale post-PR-#40) — fixed; but the Phase progression table itself is now stale against Layer 2 promotion — see [Finding 1](#r5-tw-f1) below for the Layer-2 regression-of-fix variant.

**No prior TW finding regressed in its original disposition;** the Layer 2 promotion introduced new staleness defects in artifacts the prior rounds had cleaned (the README Phase progression table is a recurrence-class instance of the Layer 1 R4 F3 fix's same defect class, on a different document drift).

---

### Resolved

<a id="r5-tw-f1"></a>
**Finding 1 — README.md describes the project as Layer-1-only across multiple claims (line 9 "Layer 1 complete", line 43 "~19 lib + integration tests at Layer 1", line 50 "Phase progression for Layer 1", Phase 2a row "4 failing tests committed before implementation"); all four are stale after the Layer 2 promotion (Dim 2 — documentation accuracy; regression-check; clone-and-follow audit-trail)**

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none — inline prose edit required)*
**Validator:** documentation-reviewer

**Domain-raised** during the cold TW pass against [`README.md`](../../README.md). Concrete evidence at four distinct sites:

**[`README.md:9`](../../README.md):**
> "Current state: **Layer 1 complete** (add + list). Layers 2 (tag + filter) and 3 (export + import) are scoped in [`DESIGN.md`](DESIGN.md) but not built — the reference-implementation purpose is satisfied by one layer end-to-end."

**Reality:** Layer 2 is now built. Per [`TODO.md:48`](../../TODO.md): *"**Status:** Active per post-PR-#43 cycle. Promoted from 'deferred — scoped only' to capstone-active per operator directive after Layer 1 reached project-terminal MVR at PR #42."* The Layer 2 source ([`src/lib.rs`](../../src/lib.rs) `BookmarkStore::attach_tag` + `filter_by_tags`; [`src/main.rs`](../../src/main.rs) `Cmd::Tag` + `Cmd::List { tags }`), the Layer 2 manual-test plan ([`manual-tests/layer-2.md`](../../manual-tests/layer-2.md)), and the 13 new Layer 2 integration tests in [`tests/bookmarks.rs`](../../tests/bookmarks.rs) all exist. The "Layer 1 complete; Layer 2 ... not built" claim is concretely false.

**[`README.md:43`](../../README.md):**
> "`cargo test` # expect: all tests pass — the test suite (currently ~19 lib + integration tests at Layer 1, post-Round-2 fix cycle) covers the behavioral contracts in DESIGN.md."

**Reality:** Per the prompt's stated test count: "41 pass / 0 fail" against the Layer 2 source. The "~19 lib + integration tests at Layer 1" annotation is roughly half the current count and frames the test surface as Layer-1-scoped when 13 of the new tests are Layer-2 Red Gate tests + 1 new unit test in `src/lib.rs` is Layer-2-related (forward-only migration shape).

**[`README.md:50`](../../README.md):**
> "Phase progression for Layer 1:"

**Reality:** The table that follows describes Phase progression for Layer 1 only. No Layer 2 Phase progression table exists — a cold reader landing on README has no orientation signal that Layer 2 is the active layer; the README still presents Layer 1 as the current frame.

**[`README.md:56`](../../README.md) Phase 2a row:**
> "| 2a | [`tests/bookmarks.rs`](tests/bookmarks.rs) Red Gate | Complete (4 failing tests committed before implementation) |"

**Reality:** Per the prompt + the file's content, Layer 2's Phase 2a added 13 more Red Gate tests (`tests_tag_*` + `tests_list_with_*` + `tests_tag_against_layer_1_format_file_migrates_forward` + the RFC 3339 closure + the fsync proxy). The "4 failing tests" count is the Layer 1 Phase 2a count + does not reflect the Layer 2 Phase 2a addition.

**Cold-reader experience.** A stranger reading the repo from scratch per the [TW Dim 1 README completeness](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) check ("Can someone new to the project understand what it does, how to install dependencies, how to run it, how to run the tests, ... from the README alone?") lands at line 9 — "Layer 1 complete" — and concludes the project is at Layer 1. They navigate to TODO.md and see Layer 2 actively in progress with 9 acceptance criteria (AC 5-13). The README and the TODO.md are now telling them different things; the audit-trail fidelity breaks at the entry-point document.

**Why this is a TW Dim 2 finding** (not Dim 1). The README is *complete* in the Dim 1 sense (purpose paragraph + prerequisites + install + run + test commands + how-this-was-built explanation) — it just isn't *accurate* against the current state. Per [TW Dim 2 docstring](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md): *"DESIGN.md features that were not implemented; function docstrings that describe the previous signature. Stale documentation is actively harmful — it misleads rather than informs."* — this finding is the Layer 2 manifestation of the same defect class TW R4 F3 fixed at PR #42 (the Phase 3 row staleness against the post-PR-#40 state); the recurrence is the result of Layer 2 promotion landing the code + tests + manual-test plan but not amending the README's Layer-1-only narrative.

**Proposed change.** Multi-site edit to [`README.md`](../../README.md):

1. **Line 9** — replace `**Layer 1 complete**` with `**Layer 1 closed project-terminal at PR #42; Layer 2 (tag + filter) active** per [`TODO.md` § Layer 2](TODO.md#layer-2--tag-and-filter)`; remove the "but not built" subordinate clause.
2. **Line 43** — replace `~19 lib + integration tests at Layer 1, post-Round-2 fix cycle` with `41 lib + integration tests total at Layer 2, covering Layer 1 + Layer 2 behavioral contracts in DESIGN.md`.
3. **Line 50** — replace `Phase progression for Layer 1:` with `Phase progression (per-layer):` and either (a) add a second table for Layer 2 progression OR (b) extend the existing table with a `Layer` column and Layer-2 rows below the Layer-1 rows.
4. **Line 56 Phase 2a row** — update the status annotation to reflect both Layer 1 (4 tests) and Layer 2 (13 additional Red Gate tests + 1 unit test).

**Why this is Raised — Open, not Resolved-inline.** The fix is multi-site prose with non-trivial editorial decisions (does the Phase-progression table get duplicated per-layer or extended with a Layer column? Does the README's reference-implementation framing at line 7 — "It is small by design and intentionally limited in scope. ... this is a portfolio demonstration artifact." — need amendment to reflect the now-Layer-2-active capstone scope?). The fix is best handled in a single follow-up commit that also re-evaluates the per-layer narrative shape against Doc Reviewer's cold-reader pair (the validator pair for TW per the prompt) rather than a one-line patch.

**Resolution path.** Raised-Open. Owner: technical-writer. Validator: documentation-reviewer. The fix lands as a multi-site README amendment; Doc Reviewer's parallel Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster cold pass validates the post-fix narrative coherence.

**Classification:** Resolved (raised; multi-site fix path Open).

---

<a id="r5-tw-f2"></a>
**Finding 2 — DESIGN.md § Performance budget Layer 2 + TODO.md § Layer 2 + manual-tests/layer-2.md Step 12 reference `tests/scaling.rs` as a shipped artifact, but the file does NOT exist; the spec-vs-impl drift breaks the cross-file consistency contract that documentation describes the current implementation (Dim 2 — documentation accuracy; cross-file consistency)**

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none — either author `tests/scaling.rs` OR amend the documentation)*
**Validator:** documentation-reviewer

**Domain-raised** during the cold TW pass against [`DESIGN.md:230`](../../DESIGN.md) + [`TODO.md:81`](../../TODO.md) + [`manual-tests/layer-2.md:444`](../../manual-tests/layer-2.md). Concrete spec-vs-impl drift evidence:

**[`DESIGN.md:230`](../../DESIGN.md):**
> "**Data-scaling tests:** Layer 2 ships sentinel integration tests at the 100 / 1,000 / 10,000-bookmark cliffs ... The tests live in `tests/scaling.rs` and use `#[ignore]` by default ..."

**[`TODO.md:81`](../../TODO.md):**
> "**Layer 2 data-scaling tests:** `tests/scaling.rs` with `#[ignore]`-gated sentinels at 100/1,000/10,000 bookmark cliffs."

**[`manual-tests/layer-2.md:444`](../../manual-tests/layer-2.md):**
> "the in-CI [`tests/scaling.rs`](../tests/scaling.rs) `#[ignore]`-gated sentinels at 100/1,000/10,000 cliffs close [Finding 5](../vsdd-suite/review-log/2026-05-20-performance-engineer.md) separately."

**Filesystem state.** `find vsdd-suite-reference-examples/bookmark-cli-manual -name 'scaling*'` returns nothing; `ls tests/` returns only `bookmarks.rs`. **The file does not exist.**

**Cold-reader experience.** A cold reader following the documentation trail (DESIGN.md → TODO.md → manual-tests/layer-2.md) lands at three separate references to `tests/scaling.rs`. If they open the file path expecting to find the sentinel tests (per the [TW Dim 9 Knowledge transfer test](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md): *"Could a developer who has never seen this project make a meaningful, correct change in one day using only the documentation?"*), they encounter a missing file. The audit-trail fidelity breaks: the documentation makes a verifiable claim that the file exists; the claim is false.

**Why this is a TW finding** (not just QE / SE). Per [TW Dim 2 docstring](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md): *"DESIGN.md features that were not implemented; function docstrings that describe the previous signature."* — the dim explicitly names DESIGN.md features-not-implemented as a TW concern. The QE Review 4 Finding 1 raises the same artifact-absence from the test-surface-completeness angle (Dim 1 + Dim 13 — layer-gate vacuous-pass); this finding raises it from the documentation-accuracy angle. The two findings are non-duplicative — they cite different dim numbers, name different impact surfaces (QE: layer-gate-cannot-fail; TW: cold-reader-encounters-broken-citation), and route to different next-step disposition shapes (QE: SE authors the file OR SO defers; TW: SO/SE amends the documentation OR ships the file).

**The clone-and-follow audit-trail fidelity (TW Dim 1+9 lens).** A future maintainer reading the repo from clone discovers three different artifacts citing `tests/scaling.rs`. The natural next action is `find . -name 'scaling*'`; the result is empty. The maintainer must now determine whether (a) the file was deleted; (b) the file was never written but the spec was authored against it; (c) the file is in a feature branch not yet merged. The audit trail's clone-and-follow value drops sharply at this point — the documentation no longer reliably guides the reader through the artifact graph.

**Proposed change.** Two acceptable fix paths:

(a) **Resolved-by-implementation** — `tests/scaling.rs` is authored per the DESIGN.md spec with the three sentinels (100/1,000/10,000-bookmark cliffs). This closes the spec-vs-impl drift at the implementation level. The documentation does not need amendment.

(b) **Raised-to-SO + documentation amendment** — amend DESIGN.md + TODO.md + `manual-tests/layer-2.md` to defer the scaling tests to a future round (e.g., gate behind the Phase 5 Layer 2 Performance Engineer round). This closes the spec-vs-impl drift at the documentation level. Acceptable per the Phase 4 routing discipline if the operator decides the scaling test surface is not feasible at Layer 2 scope.

Path (a) preserves the spec's discoverability promise; path (b) re-aligns the spec with what's actually been built. Either resolves the cold-reader audit-trail break.

**Resolution path.** Raised-Open per the Phase 3 IAR Round 1 classification universe. Owner: technical-writer (consultative on the prose amendments if path (b) is chosen); Validator: documentation-reviewer. The QE Review 4 Finding 1 is the cross-domain pair; both findings close together.

**Classification:** Resolved (raised; fix path Open).

---

<a id="r5-tw-f3"></a>
**Finding 3 — manual-tests/install-verification.md Step 3 instructs the verifier to "Follow `layer-1.md` end-to-end" but does NOT reference `manual-tests/layer-2.md`; a non-author verifier running the install-verification gate post-Layer-2 has no signal that a Layer 2 manual-test plan exists or that it should be run (Dim 2 — documentation accuracy; Dim 7 — operational documentation; Layer-2 promotion regression of audit-trail completeness)**

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none — inline prose edit required)*
**Validator:** documentation-reviewer

**Domain-raised** during the cold TW pass against [`manual-tests/install-verification.md:43`](../../manual-tests/install-verification.md).

**[`manual-tests/install-verification.md:43`](../../manual-tests/install-verification.md):**
> "### Step 3 — Run the manual-test plan
>
> Follow [`layer-1.md`](layer-1.md) (sibling file in this directory) end-to-end. Each step (0 through 6 + cleanup) should produce the expected output. Record any divergence as a Platform Engineer finding."

**Reality:** `manual-tests/` now contains three files: `install-verification.md`, `layer-1.md`, AND `layer-2.md`. The Layer 2 manual-test plan exists at [`manual-tests/layer-2.md`](../../manual-tests/layer-2.md) with 13 steps including a Step 12 hyperfine sanity-check (closes the operator-queued PE benchmark item). The install-verification doc's Step 3 was authored at Layer 1 (when only `layer-1.md` existed) and was not amended when Layer 2 promoted.

**Cold-reader experience (Nathan-shape verifier).** A non-author verifier running install-verification on a fresh system post-Layer-2 promotion follows Step 3, runs `manual-tests/layer-1.md` Steps 0-6, records PASS, and stops. The Layer 2 manual-test plan — which includes the perf budget sanity-check at the 1,000-bookmark cliff + the forward-only migration verification + the OR-semantics filter behavior — is never exercised by the install-verification gate. The Platform Engineer Dim 38 install-verification surface no longer covers the Layer 2 contracts; the install-verification gate at the spec level has gone stale against the artifact's current scope.

**Why this is a TW Dim 7 finding** (operational documentation). Per the [TW Dim 7 docstring](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md): *"Can the application be set up, deployed, and operated from documentation alone? Named content: ... known failure modes and recovery steps."* — the install-verification gate's purpose (per [G-155](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-155)) is to confirm the project can be installed + run by a non-author on a fresh system. The "+ run" half of that contract now spans two layers; the doc's Step 3 instruction covers only one.

**Proposed change.** Amend [`manual-tests/install-verification.md:43`](../../manual-tests/install-verification.md) Step 3:

> "### Step 3 — Run the manual-test plan
>
> Follow [`layer-1.md`](layer-1.md) Steps 0-6 end-to-end first — each step should produce the expected output. Then follow [`layer-2.md`](layer-2.md) Steps 0-13 end-to-end against the same shell session (or per Step 0's `cargo install` refresh; `layer-2.md` inherits the install-verification context). Record any divergence as a Platform Engineer finding for `layer-1.md` or as the natural domain-pair per `layer-2.md` § Closure protocol (typically Performance Engineer for Step 12 budget violations; QE / UX / SE for behavioral / message / binary defects respectively)."

**Adjacent surface check.** Other docs that reference the manual-test plan:
- [`README.md`](../../README.md) — does not currently reference the manual-test plans by name; cross-cuts with [Finding 1](#r5-tw-f1)'s scope.
- [`TODO.md:79`](../../TODO.md) — links `manual-tests/layer-2.md` correctly + names the hyperfine sanity-check sub-section.
- [`PROCESS.md`](../../PROCESS.md) — does not currently reference Layer 2 manual-tests (acceptable; PROCESS.md is a Layer-1 retrospective).
- `vsdd-suite/review-log/` — multiple files reference both layer-1.md and layer-2.md; the audit-trail is correctly bidirectional at the per-domain log level. Only the install-verification doc has the staleness.

**Resolution path.** Raised-Open. Owner: technical-writer. Validator: documentation-reviewer. The fix is a 4-line prose edit; routed for inline-fix in the next documentation pass.

**Classification:** Resolved (raised; inline fix path Open).

---

<a id="r5-tw-f4"></a>
**Finding 4 — README.md Prerequisites section does NOT mention `hyperfine` despite manual-tests/layer-2.md Step 12 making it a required tool for the Layer 2 performance budget sanity-check; the discoverability surface for the Layer 2 development prerequisite is mis-placed (the Step itself documents the install commands but the README is the natural first-encounter surface) (Dim 1 — README completeness; Dim 7 — operational documentation prerequisite enumeration)**

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none — inline prose edit required)*
**Validator:** documentation-reviewer

**Domain-raised** during the cold TW pass against [`README.md:11-14`](../../README.md) + [`manual-tests/layer-2.md:444-452`](../../manual-tests/layer-2.md).

**[`README.md:11-14`](../../README.md):**
> "## Prerequisites
>
> - [Rust](https://www.rust-lang.org/) 1.78+ (`cargo --version` to check)
> - macOS or Linux (Windows untested)"

**[`manual-tests/layer-2.md:444-452`](../../manual-tests/layer-2.md):**
> "**NEW prerequisite for this step (does NOT exist in [`layer-1.md`](layer-1.md)):** [`hyperfine`](https://github.com/sharkdp/hyperfine). Install via:
>
> - macOS: `brew install hyperfine`
> - Debian-derived Linux: `apt install hyperfine`
> - Cargo fallback (any platform): `cargo install hyperfine --locked`
>
> If `hyperfine` is not available and the operator wants to skip the benchmark sub-section, that is acceptable ..."

**The discoverability defect.** Per the prompt's specific Layer 2 question: *"`manual-tests/layer-2.md` Step 12's hyperfine sub-section documents the install commands for hyperfine (brew/apt/cargo) — is this the right discoverability surface, or should the README also flag it as a development prerequisite?"* — the cold-pass answer: the install commands belong in `layer-2.md` Step 12 (the runnable-step standard requires literal install commands at the point of use), AND the README Prerequisites section should flag hyperfine as a Layer-2-conditional development prerequisite so a verifier who reads top-down gets the signal before they're 12 steps deep.

The current shape requires the verifier to: (a) open the README — no hyperfine signal; (b) install `bm` per the README — no hyperfine; (c) navigate to `manual-tests/install-verification.md` for the install-verification gate; (d) navigate to `manual-tests/layer-1.md` per Step 3 — no hyperfine; (e) navigate to `manual-tests/layer-2.md` per [the proposed Finding 3 amendment](#r5-tw-f3) — and at Step 12 encounter the hyperfine requirement.

For a verifier on a fresh system in a locked-down environment (e.g., a CI sandbox where `brew`/`apt`/`cargo install` may be unavailable), discovering the Layer-2-conditional dep at Step 12 — after they've already invested time on Steps 0-11 — is friction the README could have prevented. The fallback path (`time` builtin per [`manual-tests/layer-2.md:518-528`](../../manual-tests/layer-2.md)) is correctly documented at Step 12 too, but a verifier reading top-down would benefit from knowing the choice exists earlier.

**Why this is a TW Dim 1 finding** (README completeness). Per the [TW Dim 1 docstring](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md): *"prerequisites listed explicitly (runtime version, system dependencies)"*. The current README Prerequisites list is complete for the *runtime* dependencies (just Rust + a Unix-family OS); it's incomplete for the *development / verification* dependencies that the manual-test plans require.

**Proposed change.** Amend [`README.md:11-14`](../../README.md) Prerequisites section:

> "## Prerequisites
>
> - [Rust](https://www.rust-lang.org/) 1.78+ (`cargo --version` to check)
> - macOS or Linux (Windows untested)
>
> **Optional (Layer 2 manual-test plan only):** [`hyperfine`](https://github.com/sharkdp/hyperfine) for the [`manual-tests/layer-2.md`](manual-tests/layer-2.md) Step 12 performance budget sanity-check. Install via `brew install hyperfine` (macOS), `apt install hyperfine` (Debian-derived Linux), or `cargo install hyperfine --locked` (any platform). The manual-test plan documents a `time`-builtin fallback for environments where hyperfine cannot be installed."

**Why this is Raised — Open, not Resolved-inline.** The fix is a small README amendment but the editorial decision about how to introduce the "Optional (Layer 2 manual-test plan only)" tier in the README's Prerequisites list interacts with [Finding 1](#r5-tw-f1)'s broader Layer-2-narrative amendment. Best to land both edits in the same documentation pass for narrative coherence.

**Resolution path.** Raised-Open. Owner: technical-writer. Validator: documentation-reviewer.

**Classification:** Resolved (raised; inline fix path Open).

---

### Deferred

<a id="r5-tw-f5"></a>
**Finding 5 — TODO.md § Layer 2 Layer-gate criterion #6 cites "the next VDD-IAR Alignment review round titled 'Review N — Phase 6 four-dimensional convergence (project-terminal Layer 2)'" by descriptive title but does NOT link the planned location of the file the review will land in; the Phase 6 convergence record's discoverability is implicit-only against the per-domain review-log convention (Dim 4 — Decision rationale; Dim 9 — Knowledge transfer test)**

**Owner:** technical-writer
**Status:** raised
**Blocked by:** Methodology — the convergence record does not yet exist; the link would point to a future file that has not been authored.
**Validator:** sanity-check

**Domain-raised** during the cold TW pass against [`TODO.md:92`](../../TODO.md) Layer-gate criterion #6 + [`DESIGN.md:17`](../../DESIGN.md) § Phase 6 strategy Layer 2 declaration.

**[`TODO.md:92`](../../TODO.md):**
> "6. [Phase 6](../../vsdd-suite/primers/6-convergence.md) four-dimensional convergence record landed as the next VDD-IAR Alignment review round titled 'Review N — Phase 6 four-dimensional convergence (project-terminal Layer 2)' per primer 6 + [G-177](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-177)."

**[`DESIGN.md:17`](../../DESIGN.md):**
> "Layer 2 four-dimensional convergence record will land as a later VDD-IAR Alignment review round titled 'Review N — Phase 6 four-dimensional convergence (project-terminal Layer 2)' — attests: ..."

Per the prompt's specific Layer 2 question: *"The Phase 6 four-dimensional convergence record for Layer 1 lives at `vsdd-suite/review-log/2026-05-20-vdd-iar-alignment.md` Review 3. Is the Layer 2 convergence record's planned location (per primer 6) discoverable from the TODO.md § Layer 2 Layer-gate criteria #6? Or is it implicit?"*

**The discoverability defect.** Both TODO.md and DESIGN.md cite the convergence record by descriptive title ("Review N — Phase 6 four-dimensional convergence (project-terminal Layer 2)") but do NOT link to:
- The future file path where it will land (e.g., a Layer-2-dated `vsdd-suite/review-log/YYYY-MM-DD-vdd-iar-alignment.md`).
- The Layer 1 precedent file ([`vsdd-suite/review-log/2026-05-20-vdd-iar-alignment.md`](../../vsdd-suite/review-log/2026-05-20-vdd-iar-alignment.md) Review 3) so a reader can navigate to "what the Layer 1 version looked like" as a template signal.

A cold reader following the layer-gate criterion #6 has the descriptive title but no clickable path to either the future location or the past precedent. The per-domain review-log convention ([G-89](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-89) structural standard) makes the location *inferable* — "VDD-IAR Alignment is the meta domain; per-session files are named `YYYY-MM-DD-vdd-iar-alignment.md`; the Layer 1 instance lives at the 2026-05-20 file" — but the inference requires the reader to navigate the convention rather than follow a link.

**Why this is Deferred, not Resolved-inline.** Per the [TW Dim 13 inline-reference navigability](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md), forward-facing prose should link inline references where possible — but linking to a not-yet-existing file is itself a discipline concern (broken links are a worse audit-trail signal than implicit references). The acceptable forms are:

(a) **Link to the Layer 1 precedent** as a "see also" signal — "per primer 6 + [G-177](.../FINDINGS-INDEX.md#g-177); for the Layer 1 precedent see [VDD-IAR Alignment Review 3](../vsdd-suite/review-log/2026-05-20-vdd-iar-alignment.md#review-3--phase-6-four-dimensional-convergence-project-terminal--2026-05-21-1330z)". This is a TW Dim 13 inline-reference navigability improvement that does not introduce a broken link.

(b) **Wait for the convergence record to land** before adding the forward link — at that point the TODO.md layer-gate criterion can be amended to link the actual file.

Path (a) is the floor-compliant fix; path (b) is the wait-for-the-artifact disposition. The cold-pass finding here surfaces the dim but defers the inline-fix decision to either an immediate (a) amendment OR the natural Phase 6 closure landing.

**Resolution path.** Deferred to Layer 2 Phase 6 closure round. The defect is real (the audit-trail navigability is implicit-only) but the fix lands cleanly only at the closure round when the file path becomes known. The interim option (link to the Layer 1 precedent) is the right TW Dim 13 hygiene move; flag for the next documentation pass.

**Why this is a Deferred finding, not a Hallucinated one.** The dim's value is real — a cold reader navigating to the Layer 2 convergence record from the TODO.md criterion has more friction than they would if the precedent or the future location were linked. The deferral acknowledges that the optimal fix shape depends on when the convergence record actually lands; the audit-trail entry preserves the dim for follow-up.

**Classification:** Deferred — to Layer 2 Phase 6 closure round per the artifact's natural authoring sequence.

---

### Hallucinated

*(none — Findings 1+2+3+4 are evidence-backed by exact line citations; Finding 5 is evidence-backed by the implicit-reference-only state of the cited TODO.md / DESIGN.md prose. The cold pass found no candidate hallucinated dimension.)*

---

### Dismissed

*(none — every finding routed to a real authoring or deferral outcome.)*

---

### Summary

5 findings filed in Round 1 QE/Security/Technical-Writer cluster: **4 Raised — Open** ([Finding 1](#r5-tw-f1) README Layer-1-only narrative stale at 4 sites; [Finding 2](#r5-tw-f2) `tests/scaling.rs` cited-but-absent at 3 sites; [Finding 3](#r5-tw-f3) install-verification.md Step 3 missing `layer-2.md` reference; [Finding 4](#r5-tw-f4) README Prerequisites missing hyperfine Layer-2-conditional flag), **1 Deferred** ([Finding 5](#r5-tw-f5) TODO.md § Layer 2 Layer-gate criterion #6 Phase 6 convergence-record location implicit-only — deferred to Layer 2 Phase 6 closure).

**MVR signal: NOT REACHED for this round.** Round 1 produced 4 new substantive findings + 1 Deferred. Per the [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) § Round triggers (G-131 continue trigger), Round 2 is mandatory after the fix cycle lands — Doc Reviewer's Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster parallel cold pass will validate the post-fix narrative coherence + look for adjacent documentation defects the fix may have created.

**Highest-severity finding:** [Finding 1](#r5-tw-f1) (README Layer-1-only narrative stale at 4 sites). The defect is the cold-reader's entry-point document going stale against the active layer — the maximum-impact documentation drift defect class. Cross-cuts with [Finding 2](#r5-tw-f2) (the README's Phase 2a row's "4 failing tests" count is the same staleness defect as the `tests/scaling.rs` cited-but-absent issue at the spec level — both are Layer 2 promotion artifacts that landed code/test additions but didn't propagate the narrative updates uniformly).

**Coordination:**

- **[Finding 1](#r5-tw-f1) → [Documentation Reviewer](../DOCUMENTATION-REVIEWER-REVIEW.md):** Doc Reviewer's Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster parallel cold pass validates the README amendment's narrative coherence from the cold-reader seat. The per-layer table shape decision (duplicate vs. extend-with-Layer-column) is best evaluated against the cold-reader's clone-and-follow experience.
- **[Finding 2](#r5-tw-f2) ↔ [QE Review 4 Finding 1](2026-05-22-quality-engineer.md#r4-qe-f1):** same artifact-absence defect, two domain lenses. The fix path is shared; both close together. QE owns the test-surface-completeness + layer-gate-vacuity framing; TW owns the cross-file-consistency + cold-reader-citation-resolution framing.
- **[Finding 3](#r5-tw-f3) → [Platform Engineer](../PLATFORM-ENGINEER-REVIEW.md):** PE's Dim 38 install-verification surface is the natural validator for the install-verification.md Step 3 fix — the next install-verification PASS row should confirm Step 3 instructs the verifier through both layer-1.md and layer-2.md.
- **[Finding 4](#r5-tw-f4) → [Documentation Reviewer](../DOCUMENTATION-REVIEWER-REVIEW.md):** the README Prerequisites amendment is best validated against the cold-reader's first-encounter experience.
- **[Finding 1](#r5-tw-f1) ↔ [Security Review 4 Finding 1](2026-05-22-security.md#r4-sec-f1):** Security raises the DESIGN.md § Storage data classification gap (tags field unclassified) from the Dim 8 lens; both findings are spec-completeness drifts caused by the Layer 2 promotion landing impl-changes without uniformly amending the spec/narrative surfaces. Non-duplicative; both close in a single SO documentation-amendment pass.
- **[Finding 5](#r5-tw-f5) (Deferred):** no immediate routing; flag for the Layer 2 Phase 6 closure pass to land the inline link.

**Upstream-suite-recurrence-prevention candidates.**

1. **Layer promotion → README narrative sync checklist** ([Finding 1](#r5-tw-f1) + recurrence of TW R4 F3 defect class) — the defect class will recur on any project that promotes a deferred layer to active without amending the README's per-layer narrative. Recommendation: extend the [Phase 1c primer](../../../../vsdd-suite/primers/1c-decomposition.md) layer-promotion checklist with a "README narrative update" line — every layer promotion must amend the README's (a) project-state claim ("Layer N complete" → "Layer N closed; Layer N+1 active"); (b) Phase progression table per-layer narrative; (c) test-count + test-surface scope annotation. Cross-references the [primer 4 routing discipline](../../../../vsdd-suite/primers/4-feedback-integration.md).

2. **Layer promotion → install-verification gate sync** ([Finding 3](#r5-tw-f3)) — the install-verification gate's manual-test plan reference should auto-extend across layers. Recommendation: add a TW Dim 14 or extend Dim 7 with "install-verification.md MUST instruct the verifier through every active-layer manual-test plan, not only the most-recent or first-authored one." Alternative: a hook check parallel to [`check-anonymization.sh`](../../../../vsdd-suite/hooks/check-anonymization.sh) that scans `manual-tests/install-verification.md` against the existence of `manual-tests/layer-*.md` files + flags missing references.

3. **Spec-vs-impl citation hook** ([Finding 2](#r5-tw-f2)) — same as the QE Review 4 recurrence-prevention candidate. A `check-spec-vs-impl-citations.sh` hook that greps DESIGN.md + TODO.md for backtick-quoted file-path citations + verifies the citations resolve to existing files would mechanically prevent the drift. The TW + QE findings of this round are both manifestations of the same underlying hook-absent state.

4. **Phase 6 convergence-record forward-link discipline** ([Finding 5](#r5-tw-f5)) — the discipline gap is at the [primer 6 convergence](../../../../vsdd-suite/primers/6-convergence.md) authoring time. Recommendation: extend primer 6 with a clause naming "convergence-record discoverability — the layer-gate criterion citing the planned convergence record SHOULD link the Layer-N-minus-1 precedent (if one exists) as a 'see also' signal until the Layer-N convergence record lands; on landing, the link is updated to point to the new file." This codifies the TW Dim 13 hygiene for the convergence-authoring path specifically.

**The sycophancy-compensation discipline applied this round:** the cold-context framing kept "Layer 2 promotion was a clean operator-directive event; the narrative surfaces will follow uniformly" specifically as a hypothesis to verify rather than a default. The verification found that the narrative surfaces DID propagate at the per-domain review-log level (every cited finding-log file was updated) and at the per-test-file level (the new Layer 2 tests are correctly named and grouped), but the cross-file narrative coherence in the README + the install-verification gate + the spec citations was NOT propagated uniformly — the propagation gap is the cold-pass surface. A warm-context reviewer would likely have rationalized "the Layer 2 PR landed cleanly" into "all surfaces are coherent" and missed the four Raised findings + the one Deferred. Recorded explicitly so future reviewers see the reasoning chain.

**The cold-reader audit-trail fidelity (TW Dim 9 lens).** A non-author landing in the repo at HEAD and following the documentation top-down (README → DESIGN.md → TODO.md → manual-tests/) encounters: (a) the README presents Layer-1-only narrative; (b) DESIGN.md correctly carries Layer 2 contracts; (c) TODO.md correctly promotes Layer 2 to active; (d) `manual-tests/layer-2.md` exists but is not referenced from `install-verification.md`. The audit trail has four discoverable gaps; the Dim 9 Knowledge transfer test rating is **degraded but not broken** — a determined reader can still piece together the Layer 2 active-state, but the friction is real and the [Finding 1](#r5-tw-f1)+[Finding 3](#r5-tw-f3)+[Finding 4](#r5-tw-f4) fixes restore the Dim 9 rating to **clean**.

---

## Review 6 — 2026-05-22 16:55Z

**Phase:** 3 (IAR Round 2; Layer 2 Round 2 verification of the Round 1 fix cycle).
**Source:** domain-raised (Round 2 regression-verification + cross-file consistency audit + 1 new Round 2 finding on CHANGELOG.md numeric precision drift; cold-session).
**Lens:** Round 1 finding regression-verification (Dim 2 documentation-accuracy + Dim 1 README completeness + Dim 7 operational documentation + Dim 9 Knowledge transfer test) + cross-file consistency verification (DESIGN.md ↔ TODO.md ↔ README.md ↔ CHANGELOG.md ↔ src/lib.rs docstrings ↔ `bm --help`) + post-fix CHANGELOG.md accuracy audit.
**Scope:** Layer 2 post-fix artifact tip (`9d56c3f`). Read post-fix [`README.md`](../../README.md), [`CHANGELOG.md`](../../CHANGELOG.md) (Unreleased section + v0.12.3 section for cross-version consistency), [`manual-tests/install-verification.md`](../../manual-tests/install-verification.md) (Step 3 + Layer 2 inheritance note appendix), [`manual-tests/layer-2.md`](../../manual-tests/layer-2.md) Step 12 (hyperfine prerequisite), [`DESIGN.md`](../../DESIGN.md) § Storage data classification + § Threat model (the Round 1 spec amendments), [`TODO.md`](../../TODO.md) § Layer 2 (Phase 6 not-applicable annotation + layer-gate criteria), [`src/main.rs:60-119`](../../src/main.rs) clap docstrings (the `bm --help` source), and the relevant per-test docstrings in [`tests/scaling.rs`](../../tests/scaling.rs) + [`tests/properties.rs`](../../tests/properties.rs).
**Reviewer:** Technical Writer.
**Model:** Sonnet 4.6 (per the round-prompt-stated assignment).
**Cold-session shape:** QE/Security/Technical-Writer cluster (Round 2; same composition as Round 1; the natural validator pair — Documentation Reviewer — remains in Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster, structurally separated per adversarial-pair-separation discipline).
**Regression-check against:** [Review 5](#review-5--2026-05-22-0035z) (Round 1; this same per-domain log file). All 5 Round 1 findings re-evaluated against the post-fix state.
**Session note:** Cold session — this QE/Security/Technical-Writer cluster agent was spawned with no prior project context for the Round 2 verification; read post-fix artifacts in the prescribed cold-reader order (README + manual-tests + TODO.md first; DESIGN.md last per cold-reader discipline). Sycophancy-compensation per the [TW domain prompt § Sycophancy check](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md): the "the fix cycle landed cleanly; the README + DESIGN.md + TODO.md are all coherent" framing was kept as a hypothesis to verify rather than a default conclusion. The verification found that the load-bearing F1 + F2 fixes did land cleanly, but the non-load-bearing F3 + F4 fixes did not — and the Round 2 pass surfaced the disposition-change shape (operator chose a different resolution path than Round 1 proposed) explicitly rather than rationalizing it into Resolved. The CHANGELOG.md "12 fixes" precision-drift was also surfaced as a new Round 2 finding rather than dismissed.
**Cost-tally:** QE/Security/Technical-Writer cluster Round 2 verification budget ~25-40k tokens per [AI Engineer R1](2026-05-21-ai-engineer.md#review-1--2026-05-21-1000z) cluster-batching discipline at half the new-finding round budget; 5 regression-verifications (2 Resolved + 1 Carry-forward + 1 Carry-forward + 1 Resolved-via-disposition-change) + 1 Round 2 new finding (CHANGELOG.md "12 fixes" cross-file consistency hairline) → ~5-8k tokens / verification — within the capstone-intent expected band.

**Assumption surfacing.** Verified [`README.md:9`](../../README.md) post-fix reads "Layer 1 project-terminal at PR #42 + Layer 2 active in the post-PR-#43 cycle" (Round 1 F1 site 1 — fixed). Verified [`README.md:52`](../../README.md) post-fix reads "43 tests at Layer 2, post-Round-1 fix cycle" (Round 1 F1 site 2 — fixed). Verified [`README.md:63-87`](../../README.md) post-fix carries two Phase progression tables (Layer 1 + Layer 2) — the proposed Finding 1 fix-shape question (duplicate vs extend-with-Layer-column) was resolved as duplicate-per-layer. Verified the four-table-row staleness sites from Round 1 F1 are all updated. Verified [`DESIGN.md:230`](../../DESIGN.md) still cites `tests/scaling.rs` AND that the file now exists (Round 1 F2 — fixed). Verified [`manual-tests/install-verification.md:43`](../../manual-tests/install-verification.md) Step 3 still says "Follow `layer-1.md` end-to-end" with no `layer-2.md` reference — see [Finding 3](#r6-tw-f3) below. Verified [`README.md:11-14`](../../README.md) Prerequisites section still does NOT mention `hyperfine` — see [Finding 4](#r6-tw-f4) below. Verified [`TODO.md:94`](../../TODO.md) Layer-gate criterion #6 now annotated "Phase 6 not applicable" with rationale (Round 1 F5 — disposition-changed by the fix cycle to declare Phase 6 NOT APPLICABLE rather than wait for the convergence record).

---

### Resolved

<a id="r6-tw-f1"></a>
**Finding 1 — Round 1 F1 verification: README.md Layer-1-only narrative stale at 4 sites → all 4 sites updated; per-layer Phase progression tables added; 43-test count accurate (Dim 2 — documentation accuracy; regression-check; clone-and-follow audit-trail)**

**Owner:** technical-writer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** documentation-reviewer

**Round 1 Status:** Raised — Open (multi-site prose with editorial decisions about Phase-progression table shape).
**Round 2 Status:** Resolved.
**Evidence:** Fix landed at `d62bb1a` (README + CHANGELOG Layer-2-promotion). All four Round 1 stale sites updated:

- **[`README.md:9`](../../README.md)** (Round 1 site 1) post-fix reads: "Current state: **Layer 1 project-terminal at PR #42** (add + list) + **Layer 2 active in the post-PR-#43 cycle** (tag + filter). Layer 3 (export + import) is scoped in `DESIGN.md` but not built — the reference-implementation purpose is satisfied by Layer 1 reaching project-terminal end-to-end + Layer 2 extending the worked example through a second iteration of the full 6-phase cycle." The "but not built" subordinate clause moved off Layer 2 onto Layer 3, which is the correct shape.

- **[`README.md:52`](../../README.md)** (Round 1 site 2) post-fix reads: "the default test suite (currently 12 unit + 29 integration + 2 proptest = 43 tests at Layer 2, post-Round-1 fix cycle) covers the behavioral contracts in DESIGN.md. Three additional `#[ignore]`-gated data-scaling sentinels at the 100 / 1,000 / 10,000-bookmark cliffs live at `tests/scaling.rs`. Run them explicitly via `cargo test --release -- --ignored` (the CI workflow does this in a separate Linux-only job to keep macOS-runner cost down)." The test-count is accurate against the verified `cargo test` output (12 + 29 + 2 = 43 default + 3 ignored — matches).

- **[`README.md:63-74`](../../README.md)** (Round 1 site 3) post-fix carries a Layer 1 Phase progression table headed "Phase progression for Layer 1 (project-terminal at PR #42)" — the existing table preserved.

- **[`README.md:76-87`](../../README.md)** (Round 1 site 4 — new Layer 2 table) post-fix carries a second table headed "Phase progression for Layer 2 (active per the post-PR-#43 cycle — adds `bm tag` + `bm list --tag` + the per-bookmark `tags` field)" with rows for Phase 1a / 1b / 2a / 2b / 2c / 3 / 5 / 6. The Phase 2a row correctly names "13 + 1 = 14 new failing tests committed before implementation" (vs. the Round 1 stale "4 failing tests" claim). The Phase 6 row reads "**NOT APPLICABLE** at Layer 2 per `DESIGN.md` § Project intent" — matches the Phase 6 strategy declaration in DESIGN.md.

- **[`README.md:89-93`](../../README.md)** (bonus) — new "For the Layer 2 narrative in depth, see" pointers to `TODO.md` § Layer 2, `manual-tests/layer-2.md`, and `DESIGN.md` § Behavioral contracts. This addresses the cold-reader navigability concern.

**Round 2 commentary.** The fix-shape decision (duplicate-per-layer table) is the right editorial call — each layer's Phase progression is self-contained narrative, and a single combined table would have required a "Layer" column that adds visual clutter for limited gain at the current 2-layer scope. The fix resolves Round 1 F1 cleanly and restores the Dim 9 Knowledge transfer test rating.

**Classification:** Resolved (Round 1 Raised-Open → Round 2 Resolved). (Dim 2 — documentation accuracy; regression-check; clone-and-follow audit-trail)

---

<a id="r6-tw-f2"></a>
**Finding 2 — Round 1 F2 verification: `tests/scaling.rs` cited-but-absent → file created at `156ec53`; cross-file citation drift closed across DESIGN.md + TODO.md + manual-tests/layer-2.md (Dim 2 — documentation accuracy; cross-file consistency)**

**Owner:** technical-writer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** documentation-reviewer

**Round 1 Status:** Raised — Open (cross-file consistency break; spec-vs-impl drift at 3 citation sites).
**Round 2 Status:** Resolved.
**Evidence:** Fix landed at `156ec53` (create promised `tests/scaling.rs` + `tests/properties.rs`). The three Round 1 citation sites now resolve cleanly:

- [`DESIGN.md:230`](../../DESIGN.md) `Data-scaling tests:` paragraph still cites `tests/scaling.rs`; the file now exists at [`tests/scaling.rs:1-222`](../../tests/scaling.rs) with the three contracted sentinels.
- [`TODO.md:81`](../../TODO.md) `tests/scaling.rs` citation still present; resolves cleanly.
- [`manual-tests/layer-2.md:444`](../../manual-tests/layer-2.md) `[tests/scaling.rs]` link still present; resolves cleanly.

A cold reader following the documentation trail (DESIGN.md → TODO.md → `manual-tests/layer-2.md`) now lands on three coherent references that resolve to an actual file with the three named sentinels. The Round 1 F2 cold-reader-encounters-broken-citation concern is closed.

**Round 2 commentary.** Cross-validates with [QE Review 5 Finding 1 verification](2026-05-21-quality-engineer.md#review-5--2026-05-22-1645z) — same artifact, two domain lenses; both verifications agree.

**Classification:** Resolved (Round 1 Raised-Open → Round 2 Resolved). (Dim 2 — documentation accuracy; cross-file consistency)

---

### Deferred

<a id="r6-tw-f3"></a>
**Finding 3 — Round 1 F3 verification: install-verification.md Step 3 missing `layer-2.md` reference → operator disposition adopted Layer-2-inheritance path (Step 3 unchanged; inheritance note appendix added at lines 71-77 with operator-action-item for post-merge Layer 2 verification solicitation); Carry-forward to next Layer 2 install-verification cycle (Dim 2 — documentation accuracy; Dim 7 — operational documentation)**

**Owner:** technical-writer
**Status:** raised
**Blocked by:** Operator-driven disposition — the install-verification gate is satisfied via the inheritance disposition per [G-155](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-155) strict reading; the Step 3 amendment is deferred to the cycle that activates the queued operator-action-item to solicit a Layer 2 install-verification.
**Validator:** documentation-reviewer

**Round 1 Status:** Raised — Open (4-line prose edit; Step 3 should instruct verifier through both layer-1.md and layer-2.md).
**Round 2 Status:** Partially-Resolved-with-disposition-change (Carry-forward at low priority).
**Evidence:** Fix commit `9d56c3f` ("Layer 2 inheritance note (PE F3)") added a **Layer 2 inheritance note appendix** at [`manual-tests/install-verification.md:71-77`](../../manual-tests/install-verification.md) declaring that Layer 2 inherits Layer 1's PR #41 install-verification PASS row per [G-155](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-155) strict reading — the project-as-a-whole has cleared dim 38 once and continues to satisfy the requirement.

**Step 3 itself remains unchanged** — [`manual-tests/install-verification.md:43`](../../manual-tests/install-verification.md) still reads "Follow `layer-1.md` (sibling file in this directory) end-to-end" with no `layer-2.md` reference.

**Disposition analysis.** The fix-cycle adopted a different resolution path than the Round 1 TW F3 proposed amendment. The operator/PE F3 disposition is that Layer 2's MVR does NOT require a new install-verification PASS row to ship (the inheritance note replaces the proposed Step 3 amendment for the shipping-blocker question). The operator-action-item flags soliciting a fresh-system Layer 2 install-verification in the post-merge feedback cycle — the queued action is not a Layer 2 MVR blocker but a discipline-improvement item.

This is a **legitimate disposition choice** (the operator's call on the install-verification scope) BUT it leaves the underlying TW F3 cold-reader-discoverability concern partially open: a future verifier who DOES want to run a Layer 2 verification (per the operator-action-item or independently) lands on Step 3 and finds no signal that `layer-2.md` exists or should be run. The verifier has to discover `layer-2.md` via the [Layer 2 inheritance note appendix](../../manual-tests/install-verification.md) at lines 71-77 or via the README's Layer 2 pointers.

**Carry-forward classification.** The fix-cycle's choice to add the inheritance note appendix + operator-action-item satisfies the install-verification gate's shipping-blocker concern, but the cold-reader Dim 7 operational-documentation concern from TW F3 persists at the Step 3 site itself. The Step 3 wording is now consistent with the inheritance note ("Layer 2 inherits the Layer 1 PASS row; Step 3 runs `layer-1.md`") but a future Layer-2-specific verification cycle (when the operator solicits one) will need the Step 3 amendment to instruct the verifier through `layer-2.md`. **Flagged Carry-forward to: next Layer 2 install-verification cycle** (the natural trigger to amend Step 3 is when the operator activates the queued operator-action-item to solicit a Layer 2 install-verification).

**Round 2 commentary.** This is a non-load-bearing Carry-forward — the install-verification gate is satisfied per the inheritance disposition; the Step 3 amendment is deferred to the cycle that activates the Layer 2 verification. The audit-trail is intact (the inheritance note documents the disposition; the operator-action-item documents the deferred amendment). Not raising as a new Round 2 finding because the disposition is operator-driven and the carryforward routing is documented.

**Classification:** Deferred — to next Layer 2 install-verification cycle (operator-action-item activation; pair-routed with Finding 4). (Dim 2 — documentation accuracy; Dim 7 — operational documentation)

---

<a id="r6-tw-f4"></a>
**Finding 4 — Round 1 F4 verification: README.md Prerequisites missing hyperfine → not addressed by the fix cycle; pair-routed with Finding 3 to next Layer 2 install-verification cycle (Dim 1 — README completeness; Dim 7 — operational documentation prerequisite enumeration)**

**Owner:** technical-writer
**Status:** raised
**Blocked by:** Coordinates with Finding 3 (both address Layer 2 install-verification cold-reader discoverability; land cleanly together at the next install-verification cycle).
**Validator:** documentation-reviewer

**Round 1 Status:** Raised — Open (inline prose edit; flag hyperfine as Layer-2-conditional development prerequisite).
**Round 2 Status:** Carry-forward (not addressed by the fix cycle; non-load-bearing).
**Evidence:** [`README.md:11-14`](../../README.md) Prerequisites section post-fix still reads:

> "## Prerequisites
>
> - [Rust](https://www.rust-lang.org/) 1.78+ (`cargo --version` to check)
> - macOS or Linux (Windows untested)"

No hyperfine entry was added. The fix cycle's `d62bb1a` (README + CHANGELOG) focused on the load-bearing Layer-2-narrative promotion (Round 1 F1) but did not amend the Prerequisites section. [`manual-tests/layer-2.md:462-466`](../../manual-tests/layer-2.md) Step 12 still documents the hyperfine install commands at point-of-use. The cold-reader friction concern (a Layer 2 verifier reaches Step 12 before discovering they need hyperfine; the `time` builtin fallback is documented at Step 12 too) persists at the README level.

**Carry-forward classification.** The fix-cycle did NOT address this finding. The non-load-bearing classification holds — the install commands at the point-of-use are correct + complete + include a no-hyperfine fallback path; the discoverability friction is real but not a blocker for the install-verification gate (the inheritance disposition + the operator-action-item shape means a Layer 2 verifier is operator-solicited rather than a cold-Nathan-shape verifier who lands on the README first). **Flagged Carry-forward to: next README amendment pass** (the natural trigger is the operator-action-item activation that solicits a Layer 2 install-verification — at that point both the install-verification.md Step 3 amendment (Round 1 F3) and the README Prerequisites hyperfine entry (Round 1 F4) land together as a coherent Layer 2 install-verification cycle preparation).

**Round 2 commentary.** Non-load-bearing Carry-forward. Pair-routed with Round 1 F3 (both address Layer 2 install-verification cold-reader discoverability and land cleanly together at the next install-verification cycle).

**Classification:** Deferred — to next README amendment pass (pair-routed with Finding 3 to next Layer 2 install-verification cycle). (Dim 1 — README completeness; Dim 7 — operational documentation prerequisite enumeration)

---

<a id="r6-tw-f5"></a>
**Finding 5 — Round 1 F5 verification: TODO.md § Layer 2 Layer-gate criterion #6 Phase 6 convergence-record location → resolved via Phase 6 NOT APPLICABLE declaration at `002d747` (subsumes the Round 1 Dim 13 navigability concern; SO/VDD-IAR-Alignment cluster-led disposition closure) (Dim 2 — documentation accuracy; Dim 13 — inline-reference navigability)**

**Owner:** technical-writer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** documentation-reviewer

**Round 1 Status:** Deferred (to Layer 2 Phase 6 closure round; the inline-fix decision depended on when the convergence record actually lands).
**Round 2 Status:** Resolved-via-disposition-change (the fix-cycle resolved the deferred dimension by declaring Phase 6 NOT APPLICABLE rather than waiting for the convergence record).
**Evidence:** Fix landed at `002d747` ([`TODO.md:94`](../../TODO.md) Layer-gate criterion #6 + [`DESIGN.md:17`](../../DESIGN.md) § Phase 6 strategy for Layer 2). [`TODO.md:94`](../../TODO.md) post-fix reads:

> "6. **[Phase 6](../../vsdd-suite/primers/6-convergence.md) not applicable** per [DESIGN.md § Project intent](DESIGN.md#project-intent) Phase 6 strategy declaration ([G-150](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-150) over-investment guard + [G-112](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112) reference-implementation-purpose-already-satisfied). Layer 1's Phase 6 attestation at [VDD-IAR Alignment Review 3](vsdd-suite/review-log/2026-05-20-vdd-iar-alignment.md) stands as the project's terminal four-dimensional convergence record; the reference-implementation purpose (exercise all six VSDD phases end-to-end as a worked example) is satisfied by Layer 1's project-terminal MVR..."

The TW Dim 13 inline-reference-navigability concern from Round 1 F5 is closed by this disposition: the layer-gate criterion #6 now links the Layer 1 precedent at `vsdd-suite/review-log/2026-05-20-vdd-iar-alignment.md` (the "see also" path from Round 1 F5 option (a)) AND declares Phase 6 NOT APPLICABLE for Layer 2 (which obviates the need to forward-link a future convergence record that won't exist).

**Round 2 commentary.** The fix-cycle adopted a more substantive resolution than Round 1 F5 proposed — rather than wait for the convergence record (Round 1 option b) or link the Layer 1 precedent (Round 1 option a), the cluster declared Phase 6 NOT APPLICABLE for Layer 2 per [G-150](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-150) over-investment guard + [G-112](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112) reference-implementation-purpose-already-satisfied. This is a Solution-Owner/VDD-IAR-Alignment-cluster-led disposition closure that subsumes the Round 1 F5 navigability concern. Round 2 verifies the disposition lands coherently across DESIGN.md + TODO.md + README.md (Phase progression table Phase 6 row also reads "NOT APPLICABLE" — cross-file consistency verified).

**Classification:** Resolved (Round 1 Deferred → Round 2 Resolved-via-disposition-change). (Dim 2 — documentation accuracy; Dim 13 — inline-reference navigability)

---

<a id="r6-tw-f6"></a>
**Finding 6 — Round 2 new finding: CHANGELOG.md § Round 1 cluster cold-session review (line 46) declares "(12 fixes)" but the actual fix-commit count on `bookmark-cli-manual-layer-2` since Round 1 close (`02e6eb3`) is 5 commits / ~17 finding-closures; minor numeric drift at the audit-trail surface (Dim 2 — documentation accuracy)**

**Owner:** technical-writer
**Status:** raised
**Blocked by:** *(none — single-line CHANGELOG edit OR re-counting of "fixes" granularity)*
**Validator:** documentation-reviewer

**Domain-raised** during the cold TW Round 2 pass against [`CHANGELOG.md:46`](../../CHANGELOG.md) (Unreleased section, "Round 1 cluster cold-session review" paragraph) cross-checked against `git log 02e6eb3..9d56c3f --oneline` (5 commits: `156ec53` + `d62bb1a` + `002d747` + `cdb46bc` + `9d56c3f`).

**Defect class.** [`CHANGELOG.md:46`](../../CHANGELOG.md):

> "The 4 cold-session clusters (SE/UX/Performance-Engineer; QE/Security/Technical-Writer; Solution-Architect/Red-Team/Platform-Engineer; Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment) surfaced ~30 findings across the 13 capstone-active domains. The Round 1 inline fix cycle on this branch resolved the load-bearing subset (12 fixes); the residual subset routes to Round 2 verification or carryforward."

The "12 fixes" number does not match the 5 fix-commits visible at `git log 02e6eb3..9d56c3f`. Possible reading: the "12 fixes" refers to individual finding-closures rather than commits (each commit closes multiple findings per the commit-subject lines — `156ec53` closes QE F1 + PE F2 + SO F1 + Phase 5 proptest activation = 4 finding-closures in one commit). Re-counting at finding-closure granularity: `156ec53` (4 finding closures) + `d62bb1a` (TW F1 + Doc Reviewer F1 = 2) + `002d747` (Security F1 + SA F5 + Red Team F6 + PE F4 + VDD-IAR R4 F1 + F5 = 6) + `cdb46bc` (UX F1 + F2 + F3 + SE F2 = 4) + `9d56c3f` (PE F3 = 1) = 17 finding-closures total.

Neither "12 fixes" nor "5 commits" nor "17 finding-closures" matches cleanly. The number is at minimum imprecise; at maximum it's the residue of an earlier fix-cycle plan that intended 12 finding-fixes and was either expanded or re-grouped during execution.

**Cold-reader experience.** A future reader (or a CHANGELOG-driven post-PR-#-merge release-notes author) lands on this line and gets an inaccurate count of the Layer 2 Round 1 fix-cycle's scope. The discipline-cost is small (1 line out of 47 in the Unreleased section is imprecise) but real — CHANGELOG.md is a load-bearing audit-trail artifact + a release-notes source-of-truth; numeric drift at this surface is the same defect class TW Dim 2 catches at other documentation sites.

**Proposed change.** [`CHANGELOG.md:46`](../../CHANGELOG.md) — replace "(12 fixes)" with either:
- "(5 fix commits closing ~17 finding-closures across the 4 clusters)" — the verifiable-against-`git log` form; OR
- Soften to "(the load-bearing subset of fixes)" without a number — avoids the precision claim entirely; OR
- If the original "12 fixes" had a specific intended granularity (perhaps the 12 load-bearing findings that fix-cycle commits explicitly close), restate as "(12 load-bearing finding-closures across 5 commits)" and verify the count matches.

**Why this is a Round 2 new finding** (not a Round 1 regression). The CHANGELOG.md Unreleased section is a Round-1-fix-cycle artifact — it didn't exist at Round 1 cold-session time. This is a new audit-trail surface introduced by the fix cycle itself; the Round 2 cold pass is the natural surface to verify it for accuracy.

**Resolution path.** Raised — Open. Owner: technical-writer (or solution-owner per CHANGELOG.md change authority — coordinate). Validator: documentation-reviewer.

**Classification:** Resolved (Round 2 new finding; raised with single-line fix path Open). (Dim 2 — documentation accuracy; minor numeric drift)

---

### Dismissed

*(none — every finding routed to a real authoring or deferral outcome.)*

---

### Hallucinated

*(none — Round 1 F1+F2+F5 are evidence-backed by post-fix file inspection; F3+F4 Carry-forward dispositions are evidence-backed by the unchanged Step 3 + Prerequisites prose; new Round 2 Finding 6 is evidence-backed by the CHANGELOG.md line 46 vs `git log` count discrepancy.)*

---

### Summary

5 Round 1 findings verified at Round 2 + 1 new Round 2 finding: **2 Resolved** ([F1](#r6-tw-f1) README Layer-1-only narrative → updated at all 4 sites at `d62bb1a`; [F2](#r6-tw-f2) `tests/scaling.rs` → file created at `156ec53`), **1 Resolved-via-disposition-change** ([F5](#r6-tw-f5) Layer-gate criterion #6 → resolved via Phase 6 NOT APPLICABLE declaration at `002d747` rather than the Round 1 deferral path), **2 Deferred-Carry-forward** ([F3](#r6-tw-f3) install-verification Step 3 → operator-disposition path with operator-action-item queued; [F4](#r6-tw-f4) README Prerequisites hyperfine → pair-routed with F3 to next install-verification cycle), **1 new Round 2 finding** ([F6](#r6-tw-f6) CHANGELOG.md "12 fixes" number doesn't reconcile against fix-commit count; minor audit-trail hairline at line 46).

**Cross-file consistency audit (Round 2 prompt request).** Verified DESIGN.md ↔ TODO.md ↔ README.md ↔ CHANGELOG.md ↔ src/lib.rs docstrings ↔ `bm --help`:

- **Test-count consistency:** `README.md:52` says "43 tests at Layer 2" + "Three additional `#[ignore]`-gated scaling sentinels". `cargo test` verified output: 43 default + 3 ignored. **Consistent.**
- **`bm --help` accuracy (clap doc strings at [`src/main.rs:60-119`](../../src/main.rs)):**
  - `Cmd::Add` docstring (lines 60-72) — names the URL-non-empty requirement + exit codes 0/1/2/64 + atomic-save + parent-dir-fsync on Unix. **Accurate against DESIGN.md § Behavioral contracts `bm add` + source.**
  - `Cmd::List` docstring (lines 77-92) — names the optional `--tag` filter + OR-semantics across repeated `--tag` + empty-store-precedence-over-filter-empty-state + empty-label rejection. **Accurate against DESIGN.md § Behavioral contracts `bm list --tag <label>` + source.** The "OR-semantics" naming matches the post-fix DESIGN.md + TODO.md + properties.rs property name.
  - `Cmd::Tag` docstring (lines 100-115) — names URL-as-identifier + idempotence + multi-match-all-tagged + `Tagged N bookmark(s).` stderr on success + the four failure modes (NoMatch / EmptyUrl / EmptyLabel / storage-error) with their exit codes. **Accurate against the post-fix Round 1 UX F2 + F3 + SE F2 disposition** (the `Tagged N bookmark(s).` affordance is the load-bearing UX fix). The success-line wording matches `src/main.rs:312` exactly.
- **CHANGELOG.md vs README.md cross-file consistency on the Phase 6 disposition:** CHANGELOG.md line 27 cites "Option 1 (mark as not-applicable)"; README.md line 87 Phase 6 row reads "NOT APPLICABLE"; DESIGN.md line 17 + TODO.md line 94 say the same. **Consistent across 4 surfaces.**
- **CHANGELOG.md vs README.md cross-file consistency on the test-count:** CHANGELOG.md does NOT cite a specific test-count number in the Unreleased section; README.md cites "43 tests at Layer 2"; the two are consistent (CHANGELOG.md is silent at this surface; README.md is the canonical citation). **Consistent.**

**MVR signal: NOT REACHED for this round.** The 2 Carry-forward findings (F3 + F4) + the 1 new Round 2 finding (F6 CHANGELOG.md "12 fixes" number) keep MVR open for the Technical Writer domain at the Layer 2 surface. The carryforwards are non-load-bearing (the install-verification gate is satisfied via the inheritance disposition; the README Prerequisites omission is a cold-reader friction issue, not a shipping blocker). The new Round 2 finding is a minor numeric-precision audit-trail issue that should land in a follow-up CHANGELOG edit. None of the residual findings block Layer 2 Phase 5 closure or other downstream phases.

**Per the operator's [feedback memory MVR signal threshold](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md)** ("if all Round 1 findings Resolved/Verified-Deferred AND zero new Round 2 findings, declare MVR"): this round does NOT meet the threshold because 2 Round 1 findings are Carry-forward + 1 new Round 2 finding was filed. The next TW cold-session round (Round 3) is the natural surface to close MVR after the operator-action-item that activates the Layer 2 install-verification cycle lands the F3 + F4 fixes AND the CHANGELOG.md hairline is addressed.

**Highest-severity Round 2 finding:** F6 (CHANGELOG.md "12 fixes" number) — but this is a low-severity audit-trail hairline, not a load-bearing defect. The Carry-forward F3 + F4 are non-load-bearing per the operator-disposition logic above.

**Phase 5/6 blockers (forward-look from TW seat).**

- **Phase 5 Layer 2:** the `tests/properties.rs` proptest activation lands cleanly per Round 1 F2 fix; the Phase 5 Layer 2 Mutation Testing + Purity Boundary Audit rounds are the natural QE/SA-domain surfaces (not TW). The TW Dim 9 Knowledge transfer test rating at Phase 5 is unaffected.
- **Phase 6 Layer 2:** NOT APPLICABLE per [`DESIGN.md` § Project intent Phase 6 strategy for Layer 2](../../DESIGN.md). TW-side Phase 6 surface for Layer 2 = none.

**Cross-file consistency audit conclusion.** Per the Round 2 prompt's specific request, the cross-file consistency between DESIGN.md ↔ TODO.md ↔ README.md ↔ CHANGELOG.md ↔ src/lib.rs docstrings ↔ `bm --help` is now **substantively coherent**. The Phase 6 NOT APPLICABLE declaration propagates correctly across 4 surfaces; the test-count is consistent; the `bm --help` text accurately reflects the post-fix UX/SE F2 disposition; only the CHANGELOG.md "12 fixes" number is imprecise. The TW Dim 9 Knowledge transfer test rating is now **clean** for the Layer 2 narrative surface (the Round 1 F1 + F2 fixes restored it from "degraded" to "clean"); the residual carryforwards (F3 + F4) are cold-reader-friction at non-entry-point surfaces (install-verification.md Step 3 + README.md Prerequisites) rather than entry-point narrative drift.

**Cost-tally suffix:** ~7-9k tokens for this Round 2 verification + 1 new Round 2 finding + 1 cross-file consistency audit = within the projected ~25-40k Round-2 cluster budget at half-rate.

**Coordination:** Round 1 F1 routes to Documentation Reviewer for cold-reader-validation of the per-layer-table shape decision in the Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster Round 2. Round 1 F2 cross-validates with [QE Review 5 Finding 1 verification](2026-05-21-quality-engineer.md#review-5--2026-05-22-1645z) — same file, two domain lenses; both verifications agree. Round 1 F3 + F4 pair-route to the next Layer 2 install-verification cycle when the operator activates the queued action-item. Round 1 F5 cross-closes with the SO/VDD-IAR Alignment cluster's Phase-6-NOT-APPLICABLE disposition at `002d747`. Round 2 new Finding 6 routes to next CHANGELOG amendment pass (SO change authority + TW consultative). All Resolved findings declare `**Validator:**` per [TW Validator-pair declaration](../../../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md).
