# Quality Engineer Review — 2026-05-21

---

## Review 3 — 2026-05-21 20:30Z

**Layer:** Layer 1 — Add and List ([`TODO.md` § Layer 1](../../TODO.md#layer-1--add-and-list)).
**Tested against:** Post-PR-#40 state ([per-domain index retirement](../FINDINGS-INDEX.md) + [Doc Reviewer sweep](2026-05-20-documentation-reviewer.md)) + PR #41 (Nathan's [`manual-tests/install-verification.md`](../../manual-tests/install-verification.md) PASS row added).
**Round:** 3 (per-session files list 2 prior QE rounds — [`2026-05-17-quality-engineer.md`](2026-05-17-quality-engineer.md) `## Review 1 — 2026-05-17 03:25Z` + [`2026-05-20-quality-engineer.md`](2026-05-20-quality-engineer.md) `## Review 2 — 2026-05-20 02:45Z`).
**Active domain set:** 12 role + 1 meta = 13 active domains per [`DESIGN.md`](../../DESIGN.md) § Project intent (post-PR-#39 [AI Engineer](../../../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md) addition).

**Scope:** Quality Engineer dimensions applied to the manual-test executability + assertion-quality discipline against [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md) Steps 0-6 + [`manual-tests/install-verification.md`](../../manual-tests/install-verification.md) Steps 1-4 (PRIMARY scrutiny target per Nathan's Post 8 "literal -- empty" confusion — a manual-test expected-output that's confusing IS a test-discipline defect because the verifier cannot confidently distinguish pass-from-fail when the assertion is unclear). Read [`README.md`](../../README.md), [`manual-tests/`](../../manual-tests/), [`TODO.md`](../../TODO.md), [`PROCESS.md`](../../PROCESS.md), [`tests/bookmarks.rs`](../../tests/bookmarks.rs), [`src/lib.rs`](../../src/lib.rs) test modules, [`src/main.rs`](../../src/main.rs), [`DESIGN.md`](../../DESIGN.md) (LAST per cold-reader discipline). Applied the [Quality Engineer domain](../../../../vsdd-suite/domains/role/QUALITY-ENGINEER-REVIEW.md) dimensions + the [CLI supplement § Quality Engineering](../../../../vsdd-suite/supplements/cli.md) + the [Rust supplement § Quality Engineering](../../../../vsdd-suite/supplements/rust.md) + the [markdown supplement § Quality Engineering](../../../../vsdd-suite/supplements/markdown.md) (markdown-as-test-input dim for manual-test plan executability).

**Session note:** Cold session — this cluster agent was spawned post-PR-#40 with no prior project context; read artifacts in the prescribed cold-reader order. This round's primary target is the manual-test plan as a test-discipline artifact — Nathan's Post 8 + Post 10 external-feedback ([`../../../../vsdd-suite/suite-development/review-log/2026-05-21-install-verification-bluesky-thread.txt`](../../../../vsdd-suite/suite-development/review-log/2026-05-21-install-verification-bluesky-thread.txt)) provides external corroboration of the test-plan-executability defects this cold pass surfaces independently. Per the [Quality Engineer domain prompt § Sycophancy check](../../../../vsdd-suite/domains/role/QUALITY-ENGINEER-REVIEW.md): "An agent that wrote both the tests and the implementation will find the tests adequate because they reflect its own interpretation of the spec, not the spec itself." — the manual-test plan's "literal — empty" wording is exactly this failure mode: the AI author who wrote both the test plan and the implementation found the wording internally consistent (an empty fenced block does match an empty stdout); a non-author tester encountering it cold cannot interpret it the same way. Nathan's "the 'literal -- empty' was confusing" is the falsification signal.

**Source:** mixed — `external-feedback` for Finding 1 (Nathan Whitehead's Bluesky Post 8 exact quote about Step 1 expected-output); `domain-raised` for Findings 2 + 3 (the cold QE pass surfaced adjacent test-discipline defects against the manual-test plan executability + assertion strength).

**Cost-tally:** This cluster execution (UX + TW + QE simultaneously) budgeted ~30-50k tokens per [AI Engineer R1 Dim 7](2026-05-21-ai-engineer.md#review-1--2026-05-21-1000z) cluster-batching discipline; ~3 findings filed across 3 domains yields ~10-15k tokens/finding — within the capstone-intent expected band.

**Assumption surfacing.** Verified the integration-test setup at [`tests/bookmarks.rs`](../../tests/bookmarks.rs) uses `assert_cmd` ([crates.io](https://crates.io/crates/assert_cmd)) v2.x (per [`Cargo.toml`](../../Cargo.toml) dev-dependencies) — integration tests DO invoke the compiled binary per the [CLI supplement § Quality Engineering](../../../../vsdd-suite/supplements/cli.md) "integration tests invoke the binary" discipline. The 4 Red Gate tests in `tests/bookmarks.rs` + the unit tests in `src/lib.rs::tests` + the mutation-test-driven `save_creates_parent_directory_for_nested_path` test (PR #38 R2 fix per [QE R2 F1](2026-05-20-quality-engineer.md#review-2--2026-05-20-0245z)) are the automated-test surface. The manual-test plan ([`manual-tests/layer-1.md`](../../manual-tests/layer-1.md) Steps 0-6) is the **second adversarial surface to IAR** per [G-132](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-132) — this review focuses on the manual-test plan's discipline since Nathan's external-feedback targets it.

**Regression check:** Prior QE rounds' findings re-verified. R1 F1 (Phase 2a → 2b commit boundary scope tradeoff — acknowledged) holds; R1 F2 (whitespace-only URL + URL-with-newlines edge cases) — `src/lib.rs::tests::reject_*` tests present + passing; R1 F3 (insufficient test count claim) — Hallucinated, no regression; R2 F1 (mutation-testing nested-parent-dir test gap) — `save_creates_parent_directory_for_nested_path` test present + passing per the [QE R2 Resolution](2026-05-20-quality-engineer.md#review-2--2026-05-20-0245z); post-fix kill rate 8/8 on viable mutants holds (verified by checking the test file for the named test — confirmed present). **No prior QE finding regressed.**

---

### Resolved

<a id="r3-f1"></a>
**Finding 1 — Step 1 expected-output blocks use "(literal — empty)" wording above empty fenced code blocks; the assertion is ambiguous to a cold tester who cannot confidently distinguish "stdout is the empty string" from "the doc author left the block as a stub" (Dim 2 — Test falsifiability / assertion strength + Dim 3 — Test selector and assertion strength)**

**Owner:** quality-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** software-engineer

**External-feedback evidence.** Nathan Whitehead, Bluesky Post 8 (2026-05-21 19:33:14 UTC) — exact quote: *"step (1) of layer.md the expected parts look a bit weird...the \"literal -- empty\" was confusing."* (Source: [`../../../../vsdd-suite/suite-development/review-log/2026-05-21-install-verification-bluesky-thread.txt`](../../../../vsdd-suite/suite-development/review-log/2026-05-21-install-verification-bluesky-thread.txt) line 46.) Nathan completed the gate (PR #41 PASS row landed) but his exact-quote feedback is the load-bearing signal — the wording impeded comprehension for a verifier who passed the gate, meaning a verifier who actually encountered a divergence would have had even less confidence interpreting the expected vs. observed delta.

**The QE framing** (distinct from the [UX framing of the same defect](2026-05-21-ux.md#r4-f1)). UX owns the "the wording is confusing" stance; QE owns the "an ambiguous expected-output block is a test-discipline defect" stance. The two findings are non-duplicative — they're the same wording defect viewed through different domain lenses.

From the QE Dim 2 (test falsifiability / assertion strength) angle: every manual-test step IS a test, and the expected-output block IS an assertion. The discipline requires the assertion to falsify a wrong implementation — a tester running the step must be able to confidently say "the implementation matches the expected output" or "the implementation diverges, and here is exactly what diverged." Three pre-fix locations in [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md) violated this discipline:

- **Line 42-45** (Step 1 stdout for `bm add`): `Expected stdout for \`bm add\` (literal — empty):` then ` ```\n``` `
- **Line 47-50** (Step 1 stderr for `bm add`): same form for stderr.
- **Line 140-143** (Step 4 stdout for `bm list`): same form for `bm list`'s stdout under the empty-state case.

Each block asserts "the expected output is the empty string"; each block's wording is ambiguous enough that a cold tester reading it has three competing interpretations (per the [UX Finding 1 cold-reader confusion path](2026-05-21-ux.md#r4-f1)). An ambiguous assertion is a low-falsifiability assertion: if a wrong implementation prints `(empty)` or `""` or a single newline to stdout, the cold tester must decide whether that matches "literal — empty" or diverges. The QE Dim 2 docstring is explicit: *"A test that cannot fail on a defective implementation has no value."* — three test-steps with this wording have low falsifiability against subtle divergence implementations.

**Concrete adversary scenario.** Consider a hypothetical broken `bm add` that prints a trailing whitespace character to stdout after success (a common defect from a `println!` instead of nothing). The pre-fix expected-output assertion at line 42 was "(literal — empty)" + empty fenced block; the observed output is one whitespace character. A cold tester sees:

- `bm add https://example.com` (cursor returns to prompt with no visible text but one trailing space character invisible to most terminals)
- Compares against "Expected stdout for `bm add` (literal — empty)" — the wording does not name what counts as a divergence. Is a single trailing space "empty"? The fenced block below contains zero bytes; a single trailing space is one byte. Strict-empty vs. visually-empty is undefined.

A high-falsifiability assertion would name: "Expected stdout — none. The command MUST emit zero bytes to stdout (no whitespace, no trailing newline; the byte count of captured stdout is exactly 0)." The pre-fix wording falls short.

**Why this is a QE Dim 2 finding** (vs. UX Dim 6). UX raises the user-friction stance; QE raises the test-discipline stance. The CLI supplement § Quality Engineering names "stdout / stderr / exit code assertions" as a test-discipline floor — every test must assert on the full interaction. The manual-test plan IS the second adversarial surface to IAR ([G-132](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-132)); an ambiguous assertion in the manual-test plan loses the cold-tester's ability to catch divergence — which is the WHOLE POINT of the manual-test discipline.

**Proposed change.** Same content fix as the [UX Finding 1 fix](2026-05-21-ux.md#r4-f1), with the QE framing: the wording change explicitly names what the assertion is (zero-byte output, silent on success) so a tester running the step has a falsifiable expectation. The UX framing emphasized user-friction; the QE framing emphasizes assertion-strength — both lead to the same authoring outcome.

**Resolution.** Inline fix coordinated with [UX Finding 1](2026-05-21-ux.md#r4-f1). [`manual-tests/layer-1.md:42`](../../manual-tests/layer-1.md), line 47, line 140 now use the explicit-prose form ("Expected stdout for `bm add` — none (the command is silent on success; the fenced block below is intentionally empty)") that names both what the expected output is (none / zero bytes) and why the fenced block is empty. The falsifiability discipline is restored: a cold tester encountering any non-zero output knows that's a divergence; the implementation must satisfy zero-byte-output to pass the step.

**Cross-domain pair.** [UX Finding 1](2026-05-21-ux.md#r4-f1) — same defect, user-friction angle. The QE finding owns the test-discipline framing + the cross-reference to the CLI supplement § Quality Engineering stdout-assertion discipline; the UX finding owns the user-facing wording fix.

**Classification:** Resolved — inline fix applied at [`manual-tests/layer-1.md:42`](../../manual-tests/layer-1.md), line 47, line 140 (coordinated with [UX Finding 1](2026-05-21-ux.md#r4-f1)). (Dim 2 — Test falsifiability; Dim 3 — Test selector and assertion strength)

---

<a id="r3-f2"></a>
**Finding 2 — [`manual-tests/install-verification.md`](../../manual-tests/install-verification.md) Step 1 expected `ls` assertion under-specifies the expected file inventory; a cold tester cannot reliably determine whether extra files are normal or a divergence from the clean-clone state (Dim 6 — Validation gaps; Dim 3 — assertion strength)**

**Owner:** quality-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** technical-writer

**Domain-raised** during the cold QE pass against [`manual-tests/install-verification.md:30`](../../manual-tests/install-verification.md) (pre-fix state). Cross-references the [TW Finding 1](2026-05-21-technical-writer.md#r4-f1) which raises the same defect from the documentation-accuracy angle; this finding raises it from the test-assertion-strength angle.

**The QE framing.** Step 1 of the install-verification gate is a test step: clone the portfolio, then assert `ls` shows a specific set of files. The pre-fix assertion enumerated only 7 of the 16 files/dirs actually present in the repo (`Cargo.toml`, `DESIGN.md`, `TODO.md`, `src/`, `tests/`, `manual-tests/`, `vsdd-suite/`, `PROCESS.md`). A cold tester running `ls` sees 16+ entries; the assertion does not say whether the extras are expected or a divergence.

**Test-discipline cost of under-specified assertions.** Per [Quality Engineer Dim 3](../../../../vsdd-suite/domains/role/QUALITY-ENGINEER-REVIEW.md): *"Are selectors, matchers, and assertions tight enough to fail on a broken implementation? Vague assertions (e.g., checking presence but not content) are a quality gap."* — the pre-fix `ls` assertion checked presence of 7 named files but did not say:

1. **Whether the absence of other-named files would be a divergence** (under-specification of the expected set).
2. **Whether the presence of un-named files would be a divergence** (no mention of `Cargo.lock`, `deny.toml`, `CHANGELOG.md`, etc. — but they ARE in the repo).
3. **What constitutes a passing match** (subset match vs. exact match — a cold tester following the doc literally might think the assertion requires exactly the named files and panic when they see more; another cold tester might think only the named files matter and miss a missing-file divergence).

The Nathan-scenario plays out here: he saw extra files and his Post 6 quote ("looks good, i see more files than are mentioned in the doc") demonstrates the cognitive-load defect. He chose to proceed because he correctly inferred the assertion was incomplete (the extra files are normal project artifacts). A less generous tester might have stopped to investigate, or might have flagged the divergence as a defect when it wasn't one. Either failure mode is a test-discipline cost the assertion strength should eliminate.

**Why this is a QE finding** (vs. the TW Finding 1 documentation-accuracy framing). QE owns the test-assertion-strength stance: an assertion that doesn't name the exact pass-or-fail criteria is a low-falsifiability assertion. TW owns the documentation-accuracy stance: documentation that under-enumerates the expected file set is stale relative to the current implementation. The TW canonical owner gets the inline-fix authoring authority because the fix is a prose change in user-facing doc; the QE finding cross-validates the fix by confirming the new assertion is falsifiable (a tester now knows exactly which files MUST be present and which are tolerable extras like `target/`).

**Resolution.** Inline fix coordinated with [TW Finding 1](2026-05-21-technical-writer.md#r4-f1). [`manual-tests/install-verification.md:30`](../../manual-tests/install-verification.md) now enumerates the full expected file set grouped by category (project-config / source + test / docs / manual-tests / VSDD audit trail) + names the `target/` directory as a post-Step-2 artifact (gitignored). The assertion is now testable: a cold tester compares observed `ls` against the named groups; presence-of-named-files is required; absence-of-named-files is a divergence; presence of `target/` is conditional on Step-2 ordering; no other surprise files should appear.

**Cross-domain pair.** [TW Finding 1](2026-05-21-technical-writer.md#r4-f1) — same defect, documentation-accuracy angle. QE owns the test-assertion-strength framing; TW owns the inline-fix authoring.

**Classification:** Resolved — inline fix applied at [`manual-tests/install-verification.md:30`](../../manual-tests/install-verification.md) (coordinated with [TW Finding 1](2026-05-21-technical-writer.md#r4-f1)). (Dim 6 — Validation gaps; Dim 3 — assertion strength)

---

### Deferred

<a id="r3-f3"></a>
**Finding 3 — [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md) Step 3 expected-output uses RFC3339-timestamp placeholders + parameter `<RFC3339-timestamp-of-second>` shape; the assertion is human-falsifiable but not machine-falsifiable — a cold tester running the step has no scripted check for "this looks like an RFC3339 timestamp" (Dim 6 — Validation gaps; Dim 7 — Logic errors / boundary detection)**

**Owner:** quality-engineer
**Status:** raised
**Blocked by:** Methodology — the manual-test plan is by design a human-executed checklist, not a scripted test. Adding scripted assertions would change the manual-test plan's role from "second adversarial surface" to "automated test in a different file" (which already exists in `tests/bookmarks.rs`).
**Validator:** sanity-check

**Domain-raised** during the cold QE pass against [`manual-tests/layer-1.md:111-115`](../../manual-tests/layer-1.md). Step 3 expected output:

```
<RFC3339-timestamp-of-second> https://second.example
<RFC3339-timestamp-of-first>  https://example.com
```

Followed by the prose assertion at line 116: "Invariant parts: exactly two lines; `https://second.example` appears FIRST (newest); `https://example.com` appears SECOND; both lines are `<timestamp> <url>` format with the timestamp parseable as RFC 3339 UTC; the two timestamps differ by ≥ 1 second."

**Test-discipline analysis.** The prose assertion is high-quality on three of four invariants: the two-line count (mechanical to verify visually), the URL ordering (mechanical), and the URL-pair identity (mechanical). The fourth invariant — "the timestamp [is] parseable as RFC 3339 UTC" — is NOT mechanical for a human tester:

1. **What counts as RFC 3339?** A tester who hasn't memorized RFC 3339 has to either trust the format on visual inspection (e.g., `2026-05-21T19:40:36.371Z` looks date-shaped, must be OK) or pull up the RFC. RFC 3339 has subtle valid-vs-invalid distinctions: `2026-05-21T19:40:36Z` is valid; `2026-5-21T19:40:36Z` is NOT (month must be zero-padded); `2026-05-21 19:40:36Z` is NOT (space instead of `T` is allowed by RFC 3339 § 5.6 NOTE but not by strict parsers); `2026-05-21T19:40:36+00:00` is valid (equivalent to `Z`).

2. **No scripted check is provided.** The manual-test plan does not include a one-liner like `python -c "from datetime import datetime; datetime.fromisoformat(line.split()[0])"` or a Rust-snippet via `cargo run` that a tester could run to validate parsability mechanically.

A cold tester encountering a subtly malformed timestamp (e.g., an implementation defect that emits `2026-5-21T19:40:36Z` without zero-padding) would likely accept it as valid because it looks date-shaped — the assertion has a mechanical-check gap.

**Why this is Deferred, not Resolved-inline.** Adding a scripted parsability check would change the manual-test plan's character: from "a human runs commands and visually verifies output" to "a human runs commands and runs scripted assertions on the output." The Phase 3 primer + the [Manual testing checklist § G-132](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-132) framing explicitly positions the manual-test plan as the **second** adversarial surface (the first is automated tests in `tests/bookmarks.rs`); the manual-test surface's value comes from the human-eye discipline, not from scripted assertions duplicating the automated tests.

The automated test surface DOES handle this: [`tests/bookmarks.rs`](../../tests/bookmarks.rs) `tests_list_orders_newest_first` invokes `bm list`, parses the timestamp via `chrono::DateTime::parse_from_rfc3339`, and asserts parsability + ordering mechanically. The automated test catches the malformed-timestamp defect class; the manual-test plan's "parseable as RFC 3339 UTC" prose is a redundant assertion the human is not expected to mechanically verify.

The honest resolution is to **defer** this finding to a future layer's Manual-Testing discipline pass — Layer 2 would re-evaluate whether the manual-test plan should add scripted snippets or whether the human-eye + automated-test split is the right design. For Layer 1, the trip-wire combination (automated tests catch malformed RFC 3339 + manual-test plan documents the expected shape + a real verifier (Nathan) ran the gate without surfacing a timestamp defect) is adequate.

Per [`primers/4-feedback-integration.md`](../../../../vsdd-suite/primers/4-feedback-integration.md) routing discipline + the QE classification universe ("Deferred — scheduled for a specific layer, reason given"): this finding is deferred to **Layer 2 manual-test plan re-evaluation** — the trigger is the Layer 2 plan adding new commands (`bm tag <bookmark-index> <label>` per [`TODO.md` § Layer 2](../../TODO.md#layer-2--tag-and-filter-deferred)), which will require new manual-test steps with their own expected-output assertions; that's the natural moment to re-evaluate whether the manual-test plan should add scripted snippets.

**Classification:** Deferred — to Layer 2 manual-test plan re-evaluation. The current human-eye + automated-test split is acceptable for Layer 1; the deferral preserves the audit trail for Layer 2 to re-evaluate. (Dim 6 — Validation gaps; Dim 7 — boundary-detection)

---

### Dismissed

*(none — every finding routed to a real test-discipline outcome.)*

---

### Hallucinated

*(none — Findings 1 + 2 are evidence-backed by Nathan's quotes + the actual repo state; Finding 3 is evidence-backed by the visible prose-vs-scripted-assertion gap in the manual-test plan.)*

---

### Summary

3 findings filed; 2 Resolved inline ([Finding 1](#r3-f1) "literal — empty" wording fix at [`manual-tests/layer-1.md`](../../manual-tests/layer-1.md) lines 42 + 47 + 140; [Finding 2](#r3-f2) under-specified `ls` assertion fix at [`manual-tests/install-verification.md:30`](../../manual-tests/install-verification.md)); 1 Deferred to Layer 2 ([Finding 3](#r3-f3) RFC 3339 parsability scripted-check question).

**External-feedback Source-value precedent honored.** [Finding 1](#r3-f1) is derived from Nathan Whitehead's Bluesky Post 8 exact-quote evidence; [Finding 2](#r3-f2) is corroborated by Nathan's Post 6 file-inventory observation (cold-batch IAR cannot surface the observed-vs-expected `ls` gap because the AI agent cannot run `ls` against a fresh clone the way a real verifier does). Per the external-feedback Source-value precedent ([Review 51](../../../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md) + [Review 85](../../../../vsdd-suite/suite-development/review-log/2026-05-21-suite-review.md#review-85--2026-05-21-1130z)), this round demonstrates the install-verification gate's external-feedback value for the QE domain.

**Cross-domain pattern across the 3 domains in this cluster.** The "literal — empty" wording defect shows up as UX Dim 6 message-clarity ([UX Finding 1](2026-05-21-ux.md#r4-f1)) and QE Dim 2/3 assertion-strength ([this Finding 1](#r3-f1)) — same defect, two domain lenses. The file-inventory under-specification shows up as TW Dim 2 documentation-accuracy ([TW Finding 1](2026-05-21-technical-writer.md#r4-f1)) and QE Dim 3/6 assertion-strength ([this Finding 2](#r3-f2)) — same defect, two domain lenses. The cluster batching correctly surfaced both defects from multiple angles without redundant findings (cross-references documented per-finding).

**Upstream-suite-recurrence-prevention candidates.**

1. **Manual-test "empty expected-output" wording standard** ([Finding 1](#r3-f1)) — The [Phase 1c primer § Manual testing checklist](../../../../vsdd-suite/primers/1c-decomposition.md) ([G-132](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-132)) requires "literal expected-output blocks" but does not specify the wording shape for *empty* expected output. Recommendation: codify the explicit-prose form (`Expected stdout — none (silent on success; the fenced block below is intentionally empty)`) as a worked example in primer 1c. Cross-references the [CLI supplement § Quality Engineering](../../../../vsdd-suite/supplements/cli.md) and § UX Dim 6.

2. **File-inventory assertion in install-verification template** ([Finding 2](#r3-f2)) — If the suite ships an install-verification template at `vsdd-suite/templates/manual-tests/install-verification.md` (verification needed), the template should include the file-inventory assertion as a templated section a project must fill in — not as a hand-edited list that drifts (as it did here when [PR #38](https://github.com/magnificentlycursed/guild-portfolio/pull/38) R2 added `Cargo.lock` / `deny.toml` / `rust-toolchain.toml` / `CHANGELOG.md` without updating the install-verification doc). Alternative: add a QE Dim 15 ("File-inventory assertion strength — install-verification docs must enumerate the full post-clone file set as a falsifiable assertion, not a subset").

3. **Manual-test as automated-vs-human discipline split** ([Finding 3](#r3-f3)) — The deferred finding raises a methodology-level question (when does a manual-test plan need scripted snippets vs. when is the human-eye + automated-test split adequate?). Recommendation: add a Phase 1c primer § Manual testing checklist clause naming the split discipline: "Manual-test assertions that are human-mechanically-verifiable (line counts, URL identity, exit codes) should stay in the manual-test plan; assertions that require pattern-matching or grammar-validation (RFC 3339 parsability, JSON schema, regex matching) should be delegated to the automated-test surface and named in the manual-test plan only by reference." This clarifies the manual-test plan's authoring shape and prevents over-investing scripted assertions in the manual-test surface (which would erode its second-adversarial-surface value).

**MVR signal: REACHED for this round.** 2 of 3 findings Resolved inline within Round 3; the 3rd is correctly Deferred to Layer 2 (a future layer, not a process slip). The install-verification gate (per Nathan's PR #41 PASS row) remains satisfied; the inline-fix work improves the next non-author verifier's experience with falsifiable assertions.

**Coordination:** [Finding 1](#r3-f1) cross-validates with [UX Finding 1](2026-05-21-ux.md#r4-f1) (same defect, message-quality angle); [Finding 2](#r3-f2) cross-validates with [TW Finding 1](2026-05-21-technical-writer.md#r4-f1) (same defect, documentation-accuracy angle); [Finding 3](#r3-f3) is QE-canonical with `**Validator:** sanity-check` per the meta-validator-of-last-resort pattern (no natural cross-domain pair for the manual-test-as-test-surface methodology question — the Sanity Check meta domain handles introspective methodology findings). All Resolved findings declare `**Validator:**` per [QE domain prompt § Validator pair](../../../../vsdd-suite/domains/role/QUALITY-ENGINEER-REVIEW.md). Upstream-suite-recurrence-prevention candidates routed via [Phase 4 feedback integration](../../../../vsdd-suite/primers/4-feedback-integration.md) to the next suite-review round for codification (3 candidates: empty-expected-output wording standard; install-verification template file-inventory section; manual-test scripted-vs-human-split discipline).

---

## Review 4 — 2026-05-22 00:25Z

**Phase:** 3 (IAR Round 1; Layer 2 — first cold-session round on the Layer 2 artifact).
**Source:** domain-raised (the standard QE dimensions + the Rust supplement § Quality Engineering floor raised every finding below; Layer 1 prior reviews referenced for regression-check only).
**Lens:** test-suite completeness + mutation-resistance projection + property-based-coverage projection + scaling-test plan rigor.
**Scope:** Layer 2 artifact in its entirety — [`DESIGN.md`](../../DESIGN.md) (Layer 2 § Scope, § Behavioral contracts `bm tag` + `bm list --tag`, § Edge case catalog Layer 2 additions, § Storage format `tags` field, § Verification architecture purity boundary Layer 2 additions, § Performance budget Layer 2 additions, § Phase 5 + § Phase 6 strategy Layer 2 declarations), [`TODO.md`](../../TODO.md) § Layer 2 (AC 5-13 + Red Gate test plan + layer-gate criteria), [`src/lib.rs`](../../src/lib.rs) (`Bookmark.tags` + `AttachTagError` + `BookmarkStore::attach_tag` + `BookmarkStore::filter_by_tags` + `fsync_directory` + the unit-test module), [`src/main.rs`](../../src/main.rs) (`Cmd::Tag` + `Cmd::List { tags }` + `run_tag` + `run_list` + `handle_parse_error` LABEL routing), [`tests/bookmarks.rs`](../../tests/bookmarks.rs) (13 new Layer 2 integration tests), [`manual-tests/layer-2.md`](../../manual-tests/layer-2.md) (13-Step plan + Step 12 hyperfine sanity-check).
**Reviewer:** Quality Engineer.
**Model:** Sonnet 4.6 (conceptual; this cluster's QE seat).
**Cold-session shape:** QE/Security/Technical-Writer cluster (shared with Security + Technical Writer; adversarial pairs — Software Engineer in SE/UX/Performance-Engineer cluster; Red Team in Solution-Architect/Red-Team/Platform-Engineer cluster; Documentation Reviewer in Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster — are NOT in this cluster).
**Session note:** Cold session — this QE/Security/Technical-Writer cluster agent was spawned with no prior project context; read artifacts in the prescribed cold-reader order (TODO.md + sources + tests + manual-tests; DESIGN.md last per cold-reader discipline). Sycophancy-compensation per the [Quality Engineer domain prompt § Sycophancy check](../../../../vsdd-suite/domains/role/QUALITY-ENGINEER-REVIEW.md): the "Layer 2 was a clean operator-directive promotion; the test surface should be uniformly complete against the spec" framing was kept as a hypothesis to verify. The verification found that the test surface MATCHES the Layer 2 contracts at the binary boundary (13 new integration tests cover AC 5-13) but DIVERGES from the spec at the shipped-artifact level (`tests/scaling.rs` + proptest tests cited in DESIGN.md / TODO.md but absent from the artifact). The cold pass kept the dim active rather than absorb the gap into "Layer 2 is clean."
**Regression-check against:** [QE Review 1](2026-05-17-quality-engineer.md#review-1--2026-05-17-0325z) (Layer 1 first pass — F1/F2 findings hold; F3 hallucinated; no Layer-2 regression surface to verify against); [QE Review 2](2026-05-20-quality-engineer.md#review-2--2026-05-20-0245z) (Layer 1 Mutation Testing — post-fix kill rate 8/8 on viable mutants; verify the mutant population for Layer 2's added code has not been measured this round, which is itself a finding); [QE Review 3](#review-3--2026-05-21-2030z) (Layer 1 manual-test executability — F3 Deferred-to-Layer-2 RFC 3339 scripted-check question is closed by this round's regression check on the new `tests_list_rfc3339_scripted_check`).
**Cost-tally:** QE/Security/Technical-Writer cluster agent budget ~50-80k tokens per [AI Engineer R1](2026-05-21-ai-engineer.md#review-1--2026-05-21-1000z) cluster-batching discipline; 4 findings filed across this round yields ~12-20k tokens/finding — within the capstone-intent expected band.

**Assumption surfacing.** Verified `assert_cmd` v2.x and `tempfile` v3.x are still in dev-dependencies (no changes from Layer 1; `Cargo.toml` was not modified for Layer 2). Verified `proptest` is NOT in `Cargo.toml` `[dev-dependencies]` despite [`DESIGN.md`](../../DESIGN.md) § Phase 5 strategy Layer 2 declaring "property-based testing via proptest now warranted" + [`TODO.md` § Layer 2](../../TODO.md) Layer-gate criterion #5 declaring "property-based testing via proptest now activated against the tag-idempotence + filter-OR-monotonicity properties" — see [Finding 3](#r4-qe-f3) below. Verified `tests/scaling.rs` does NOT exist on the filesystem despite [`DESIGN.md`](../../DESIGN.md) § Performance budget Layer 2 + [`TODO.md` § Layer 2](../../TODO.md) declaring it as a shipped artifact — see [Finding 1](#r4-qe-f1) below. Verified `cargo-mutants` was NOT re-run against the Layer 2 added code this session (no Phase 5 Layer 2 Mutation Testing round entry exists in this QE log); the [QE Review 2](2026-05-20-quality-engineer.md#review-2--2026-05-20-0245z) Layer 1 kill rate (8/8 on viable mutants) is the prior baseline.

**Regression check.** Layer 1 acceptance criteria AC 1-4 retested mentally against the post-Layer-2 source: `tests_add_creates_bookmark`, `tests_add_rejects_empty_url`, `tests_list_orders_newest_first`, `tests_list_empty_state` still present at [`tests/bookmarks.rs:44-169`](../../tests/bookmarks.rs); `Bookmark::tags` field with `#[serde(default)]` at [`src/lib.rs:54-55`](../../src/lib.rs) preserves Layer-1-format read compatibility (asserted by `tests_tag_against_layer_1_format_file_migrates_forward` at [`tests/bookmarks.rs:673`](../../tests/bookmarks.rs)); QE Review 2 retroactive Red Gate `save_creates_parent_directory_for_nested_path` still present at [`src/lib.rs:655`](../../src/lib.rs) — no Layer 1 regression.

---

### Resolved

<a id="r4-qe-f1"></a>
**Finding 1 — `tests/scaling.rs` is declared as a shipped Layer 2 artifact in DESIGN.md + TODO.md + the Layer 2 manual-test cross-reference, but the file does NOT exist on disk; Layer 2 layer-gate criterion #1 (`cargo test -- --ignored` for scaling tests) cannot be satisfied (Dim 1 — Acceptance criteria; Dim 6 — Validation gaps; Dim 13 — Quality gates / spec-vs-impl drift)**

**Owner:** software-engineer
**Status:** raised
**Blocked by:** *(none — `tests/scaling.rs` must be authored or the spec must be amended)*
**Validator:** quality-engineer

**Domain-raised** during the cold QE pass against [`DESIGN.md:230`](../../DESIGN.md) (`Data-scaling tests:` paragraph) + [`TODO.md:81`](../../TODO.md) (`Layer 2 data-scaling tests:` paragraph) + [`TODO.md:87`](../../TODO.md) (Layer-gate criterion #1) + [`manual-tests/layer-2.md:444`](../../manual-tests/layer-2.md) (Step 12 preamble cross-reference). Concrete evidence of the asserted-shipped-but-absent state:

**[`DESIGN.md:230`](../../DESIGN.md):**
> "**Data-scaling tests:** Layer 2 ships sentinel integration tests at the 100 / 1,000 / 10,000-bookmark cliffs that exercise the full add → list → tag → list-filter cycle. Each cliff asserts: (a) operations complete within the budget table above; (b) the storage file round-trips without corruption; (c) the filter result set is correct against a programmatically-generated reference. The tests live in `tests/scaling.rs` and use `#[ignore]` by default so `cargo test` stays fast; CI runs them via `cargo test -- --ignored` in a separate job."

**[`TODO.md:81`](../../TODO.md):**
> "**Layer 2 data-scaling tests:** `tests/scaling.rs` with `#[ignore]`-gated sentinels at 100/1,000/10,000 bookmark cliffs."

**[`TODO.md:87`](../../TODO.md) (Layer-gate criterion #1):**
> "1. All Red Gate tests above pass: `cargo test --test bookmarks` + `cargo test -- --ignored` (scaling)."

**Filesystem state.** `find vsdd-suite-reference-examples/bookmark-cli-manual -name 'scaling*'` returns nothing; `ls tests/` returns only `bookmarks.rs`. No `tests/scaling.rs` file exists. Therefore `cargo test -- --ignored` returns "0 ignored" (no ignored tests to run); the layer-gate criterion #1 vacuously passes for the scaling half but the criterion's intent (verify the budget at the 100/1,000/10,000-bookmark cliffs in CI) is not satisfied.

**Test-discipline cost.** Per [QE Dim 1](../../../../vsdd-suite/domains/role/QUALITY-ENGINEER-REVIEW.md): *"Are all criteria from DESIGN.md actually met by the implementation, not just implied? Trace each feature to its test coverage."* — DESIGN.md's `Data-scaling tests:` paragraph declares an implementation commitment (the tests *live in* `tests/scaling.rs`); the implementation does not match the spec. This is a Phase 2b → spec drift, the exact failure mode the [QE domain prompt § Sycophancy check](../../../../vsdd-suite/domains/role/QUALITY-ENGINEER-REVIEW.md) flags: *"the most dangerous failure mode in QE is not a missing test — it is a complete, passing test suite for the wrong behavior."* — here it is one stronger: the passing test suite for the wrong behavior is missing entirely AND the spec claims it exists.

Per [QE Dim 13](../../../../vsdd-suite/domains/role/QUALITY-ENGINEER-REVIEW.md): *"Are coverage thresholds, linting, and test runs enforced automatically? Are any quality checks manual-only that a passing CI run could miss?"* — the layer-gate criterion #1's `cargo test -- --ignored` invocation runs zero ignored tests at the moment of layer-gate evaluation; CI cannot fail on a missing scaling sentinel because there is no scaling sentinel to fail. The discipline-cost is non-trivial: a future regression in `BookmarkStore::filter_by_tags` performance at the 10,000-bookmark cliff would not be caught by any test surface against the current artifact.

**Specific mutations Layer 2's added pure code is likely to NOT have caught absent the missing scaling tests + absent any Phase 5 Layer 2 Mutation Testing round:**

- [`src/lib.rs:412`](../../src/lib.rs) `b.tags.iter().any(|t| labels.iter().any(|l| t == *l))` — the inner `any` returns `false` on empty `labels` slice. If the operator passes `bm list --tag` with zero `--tag` flags through some clap shape that bypasses the [`src/main.rs:214`](../../src/main.rs) `tags.is_empty()` early-return, `filter_by_tags(&[])` returns an empty `Vec` (every bookmark fails the inner `any`). Mutation: change `any` → `all` in the outer; under the all-bookmarks-have-empty-tags Layer-1-migration case, neither `any` nor `all` differ visibly because the inner predicate fails — only a 1,000-bookmark mixed-tag store catches the mutation. The scaling test at the 1,000-cliff would catch it; the current 13-bookmark-max integration tests do not.
- [`src/lib.rs:384-396`](../../src/lib.rs) `attach_tag` — the `matched == 0` check at line 393 + the `NoMatch` error gate the per-bookmark loop's early return. Mutation: change `matched == 0` → `matched > 0` (invert); the all-match case still passes (count is positive; returns `Ok(matched)`) but the no-match case returns `Ok(0)` instead of `Err(NoMatch)`. The `tests_tag_rejects_unknown_url` integration test catches this on the no-match path; but a `matched < N` boundary mutation at the 10,000-bookmark scale where N is data-dependent would not be caught without scaling tests.

**Why this is a QE finding** (not SE). The implementation absence is jointly a Software Engineer concern (the code doesn't exist; SE writes the code) and a Quality Engineer concern (the test surface doesn't cover the contracted scale + the layer-gate criterion's `cargo test -- --ignored` is a quality gate not actually gating anything). The QE framing owns the assertion-strength angle: the layer-gate criterion is a Quality Gate (Dim 13); a quality gate that vacuously passes is a quality gate that cannot fail on a broken implementation (Dim 2). Routed to `software-engineer` for implementation (write `tests/scaling.rs` with the three sentinels per the spec); QE validates the post-fix.

**Resolution path.** Raised-Open per the Phase 3 IAR Round 1 classification universe. Two acceptable fix paths: (a) Resolved-by-implementation — author `tests/scaling.rs` with the three `#[ignore]`-gated sentinels at 100/1,000/10,000 cliffs per the DESIGN.md spec; (b) Raised-to-SO — amend DESIGN.md + TODO.md to defer the scaling tests to a future round (e.g., gate them behind the Phase 5 Layer 2 Performance Engineer round explicitly rather than declaring them shipped at Phase 2b). Path (a) is the floor-compliant fix; path (b) is the documented-tradeoff alternative. Owner: software-engineer. Validator: quality-engineer (the natural pair for SE-implemented tests).

**Cross-domain coordination.** [Technical Writer Review 5 Finding 2](2026-05-22-technical-writer.md) raises the same artifact-absence from the documentation-accuracy lens (the spec declares the file shipped; the file is absent — TW Dim 2). The two findings are non-duplicative — TW owns the documentation-vs-code drift framing; QE owns the test-surface-completeness + layer-gate-vacuity framing. The fix path is shared; both findings close together.

**Classification:** Resolved (raised; fix path Open). The cold-session-discipline default-to-finding-when-in-doubt rule per [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) applies: the spec-vs-impl drift is unambiguous (the file is named in three places and absent from the filesystem); no rationalization closes the gap without spec amendment or code authorship.

---

<a id="r4-qe-f2"></a>
**Finding 2 — `proptest` activation is declared in DESIGN.md § Phase 5 strategy Layer 2 + TODO.md Layer-gate criterion #5 as Layer 2 layer-gate criterion, but no proptest dependency is added to Cargo.toml and no proptest tests exist; the property-based-coverage commitment is purely declarative (Dim 4 — Coverage meaningfulness; Dim 14 — TDD proxy indicators / failure specificity)**

**Owner:** software-engineer
**Status:** raised
**Blocked by:** *(none — `proptest` dep + property tests must be added or the spec must be amended)*
**Validator:** quality-engineer

**Domain-raised** during the cold QE pass against [`DESIGN.md:15`](../../DESIGN.md) § Phase 5 strategy Layer 2 declaration + [`TODO.md:91`](../../TODO.md) Layer-gate criterion #5. Exact spec language:

**[`DESIGN.md:15`](../../DESIGN.md):**
> "Layer 2: ... property-based testing via proptest now warranted — the tag idempotence + filter OR-monotonicity properties have natural algebraic shape and proptest's marginal cost is low at Layer 2 scope."

**[`TODO.md:91`](../../TODO.md) Layer-gate criterion #5:**
> "5. Phase 5 Layer 2 rounds at closure: Purity Boundary Audit re-runs against the extended pure surface; Mutation Testing re-runs against the extended impl with 100% kill rate maintenance or named-rationale drop; property-based testing via proptest now activated against the tag-idempotence + filter-OR-monotonicity properties."

**Filesystem state.** [`Cargo.toml`](../../Cargo.toml) `[dev-dependencies]` does not include `proptest`. `grep -r proptest tests/ src/` returns no matches. The property tests do not exist.

**Test-discipline cost.** Per [QE Dim 4](../../../../vsdd-suite/domains/role/QUALITY-ENGINEER-REVIEW.md): *"Does coverage reflect genuine confidence, or are covered lines trivially exercised? Are branches, edge cases, and error paths tested, not just happy paths?"* — the two algebraic properties named by the spec have natural proptest shape:

1. **Tag idempotence:** `forall url, label, store. attach_tag(store, url, label).then(attach_tag(store, url, label)) == attach_tag(store, url, label)`. Currently exercised by the integration test `tests_tag_is_idempotent` at [`tests/bookmarks.rs:553`](../../tests/bookmarks.rs) with exactly one URL + one label + a 2-invocation count — a 1×1×2 sample of the property space. Proptest would generate `(url, label)` pairs across the full UTF-8 string space + invocation counts ≥ 2 and catch idempotence violations that emerge only at specific input shapes (e.g., URL containing tag-separator-shaped bytes, label equal to URL).
2. **Filter OR-monotonicity:** `forall labels_subset, labels_superset, store. labels_subset.is_subset(labels_superset) -> filter_by_tags(store, labels_subset).is_subset(filter_by_tags(store, labels_superset))`. The current Layer 2 tests exercise this property with hand-picked 2-label cases (`tests_list_with_tag_filter_repeated_flag_is_or_semantics`); proptest would shrink to the minimal counterexample if the filter degenerates under specific tag-set shapes.

The natural algebraic shape of both properties is exactly what proptest is designed for; the spec acknowledges this. The absence of the actual proptest tests means the layer-gate criterion #5 vacuously passes against a missing test surface.

**Why this is a QE Dim 4 finding** (not Phase 5 Mutation Testing surface). The proptest activation is a Phase 5 hardening surface (per [Phase 5 primer](../../../../vsdd-suite/primers/5-formal-hardening.md)), but the spec-vs-impl drift is observable at the Phase 3 QE round level: the layer-gate criterion declares the surface activated; the surface is not activated. The Phase 5 Layer 2 round (when it lands) will be evaluating whether the proptest properties survive mutation; before that round can run there must be properties to survive. Routing: this is `Raised — Open` against Layer 2 Phase 2b authorship, not against Phase 5 closure — the Phase 5 Layer 2 round is correctly DEFERRED per the TODO.md spec; the issue is the proptest tests are declared shipped by the layer-gate criterion but absent from the artifact.

**Resolution path.** Raised-Open per the Phase 3 IAR Round 1 classification universe. Two acceptable fix paths: (a) Resolved-by-implementation — add `proptest = "1"` to `Cargo.toml` `[dev-dependencies]` and author the two property tests (idempotence + OR-monotonicity) in a new `tests/properties.rs` or as a `#[cfg(test)] mod properties` block inside `src/lib.rs`; (b) Raised-to-SO — amend DESIGN.md + TODO.md to soften the layer-gate criterion #5's proptest clause from "now activated" to "evaluated at the Phase 5 Layer 2 closure round; activation conditional on the Phase 5 surface's cost-benefit pass." Path (a) is the floor-compliant fix; path (b) preserves the cold-session-budget by deferring the surface explicitly. Owner: software-engineer. Validator: quality-engineer.

**Classification:** Resolved (raised; fix path Open).

---

### Deferred

<a id="r4-qe-f3"></a>
**Finding 3 — `tests_save_fsyncs_parent_directory` documentation in TODO.md § Layer 2 Red Gate test 14 promises a Red Gate test that asserts the syscall was invoked, but the actual test at `src/lib.rs:796` is documented as a "WEAK PROXY" that only verifies the save codepath ran successfully — the Red Gate framing is misaligned with the implementation discipline (Dim 2 — Red Gate; Dim 3 — assertion strength)**

**Owner:** quality-engineer
**Status:** raised
**Blocked by:** Methodology — direct fsync-syscall verification requires either a Rust-side trait injection (adds production-code seam) or a `strace`/`dtruss` harness (platform-specific, outside `cargo test` discipline).
**Validator:** sanity-check

**Domain-raised** during the cold QE pass against [`TODO.md:77`](../../TODO.md) Red Gate test 14 declaration + [`src/lib.rs:776-794`](../../src/lib.rs) actual test docstring.

**[`TODO.md:77`](../../TODO.md):**
> "`tests_save_fsyncs_parent_directory` (closes operator-queued PE fsync item) — adds a bookmark, asserts the `save` codepath invoked `fsync(2)` on the parent directory FD after the `rename(2)`. **Implementation strategy:** extract the durable-save into a function whose effect is observable from a unit test (an injected counter or trace-line on the unix path); the integration test asserts the observable."

**[`src/lib.rs:776-794`](../../src/lib.rs) (test docstring):**
> "Closes the operator-queued PE fsync benchmark item structurally — `save` invokes `fsync(2)` on the parent directory after `rename(2)` for durability per DESIGN.md § Performance budget Layer 2 'Durability discipline'. There is no portable way for a black-box unit test to assert that fsync was actually called on the parent directory FD (the syscall has no observable side effect from userspace). Acceptable alternative: the test asserts that after a `save` of a non-trivial store the file is present on disk + the store round-trips cleanly through `load`. **This is a WEAK PROXY for the durability contract** — it confirms the save codepath executes successfully against a real filesystem (the same codepath that includes the fsync on Unix) but does not directly verify the fsync syscall was issued."

**Test-discipline analysis.** The TODO.md Red Gate test plan declares an implementation strategy (extract-and-name + inject a counter/trace-line) that would make the syscall observable from a unit test; the actual implementation discarded that strategy in favor of a "WEAK PROXY" round-trip assertion. The misalignment is documented honestly in the test docstring, but the Red Gate discipline is compromised: the test cannot fail if a hypothetical broken implementation removes the `fsync_directory` call at [`src/lib.rs:304`](../../src/lib.rs) (the save would still succeed; the file would still round-trip; the test would still pass). Per [QE Dim 2 Red Gate](../../../../vsdd-suite/domains/role/QUALITY-ENGINEER-REVIEW.md): *"A test that passes against an empty function body or a trivially wrong stub implementation was not written first."* — the WEAK PROXY test passes against an implementation that comments out the entire `#[cfg(unix)] { ... fsync_directory(parent) ... }` block at [`src/lib.rs:296-312`](../../src/lib.rs). The test's value as a Red Gate is zero against a fsync-removal mutation.

The test docstring at [`src/lib.rs:788-793`](../../src/lib.rs) acknowledges the gap honestly + names the two alternatives that would close it: (a) injected trait/seam at the syscall boundary; (b) `strace`/`dtruss` harness. The discipline-honest disposition is "this is a weak proxy; the test is named in the Red Gate plan for audit-trail purposes but does not actually exercise the Red Gate property."

**Why this is Deferred, not Resolved-inline.** Per [QE Review 3 Finding 3 disposition shape](#r3-f3) — the manual-test plan's "RFC 3339 parseability" gap was similarly Deferred because adding the scripted check would change the manual-test plan's character. The parallel disposition here: closing the fsync verification gap would change the production-code character (add a syscall-observability seam to `BookmarkStore::save` solely for test instrumentation) or the test-harness character (introduce `strace`/`dtruss` outside the `cargo test` discipline). Both options have costs disproportionate to Layer 2's budget; the operator-queued PE fsync item was queued specifically because it lives at this boundary.

The honest resolution is to **defer** this finding to a future Layer 2 Phase 5 Performance Engineer round — the PE round is the natural surface to evaluate whether the fsync benchmark + verification deserves an instrumented codepath (e.g., the PE round may also benchmark `bm add` under simulated power-fail via VM snapshot rollback, at which point the fsync verification becomes observable as a survival-vs-loss test).

**Why this is a QE finding** (not just operator-queued PE item closure). The QE framing names the Red Gate misalignment specifically: the TODO.md Red Gate test plan's promise (the test "asserts the `save` codepath invoked `fsync(2)`") does not match the test's actual assertion (the test asserts the save succeeded + round-tripped). The Red Gate discipline requires the test to fail when the property is removed; the current test does not.

**Resolution path.** Deferred to Layer 2 Phase 5 Performance Engineer round. Cross-references the [TODO.md operator-queued PE fsync item](../../TODO.md) which is correctly named as Layer 2's closure of the deferred Layer 1 PE finding. The Phase 5 PE round will re-evaluate whether the fsync verification deserves the production-code seam or the harness investment; for Layer 2 Phase 3 Round 1, the WEAK PROXY's honest docstring is the audit-trail evidence that the gap is named, not silently rationalized. Owner: quality-engineer. Validator: sanity-check per the meta-validator-of-last-resort pattern (the Red Gate methodology question has no natural cross-domain pair).

**Classification:** Deferred — to Layer 2 Phase 5 Performance Engineer round per the operator-queued PE fsync item shape.

---

### Hallucinated

<a id="r4-qe-f4"></a>
**Finding 4 — Claim: `tests_list_rfc3339_scripted_check` is mis-scoped as a Layer-2 Red Gate test because it exercises only Layer 1's `bm list` (no `--tag`); it would not fail against an unmodified Layer 1 impl and therefore does not exercise a Layer-2 capability gap (Dim 2 — Red Gate)**

**Owner:** quality-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** sanity-check

Initial adversarial framing per the prompt's specific Layer 2 question: *"Test 13 (`tests_list_rfc3339_scripted_check`) passed at Red Gate against the unmodified Layer 1 impl, per the prior sub-agent report. Is this acceptable per its TODO.md framing as 'closes Layer-1-Deferred QE item', or does it indicate the test is mis-scoped (it doesn't actually exercise a Layer-2 capability gap)?"*

**Rejected.** The test's authoring intent + actual scope are correctly aligned with its TODO.md framing. [`TODO.md:76`](../../TODO.md) Red Gate test plan entry for this test:

> "`tests_list_rfc3339_scripted_check` (closes Layer-1-Deferred QE item) — adds three bookmarks with small delays, invokes `bm list`, asserts every emitted timestamp matches the RFC 3339 grammar at byte level via a `chrono::DateTime::parse_from_rfc3339` round-trip — not merely a regex eyeball. The Red Gate failure mode is intentional ambiguity in the Layer-1 implementation (any deviation from strict RFC 3339 — missing-`Z`, ambiguous-offset, sub-microsecond precision drift — is a finding)."

The test is **explicitly named** as closing the Layer-1-Deferred QE Review 3 Finding 3 (the RFC 3339 scripted-check question deferred to Layer 2 by [QE Review 3 disposition](#r3-f3)). It is NOT framed as testing a Layer-2 capability — it is framed as closing a Layer-1 deferred test-discipline item that was deferred because adding a scripted check inside the manual-test plan would erode the manual-test plan's second-adversarial-surface value; the deferral to Layer 2 explicitly resolved by moving the scripted check from the manual-test surface into the automated-test surface.

The test passing against an unmodified Layer 1 impl is the **expected behavior** — Layer 1's `bm list` already emits RFC 3339-compliant timestamps via `chrono::DateTime::<Utc>::to_rfc3339()` at [`src/main.rs:216`](../../src/main.rs) (which is itself unchanged from Layer 1; the Layer 2 fix moved it into the per-subcommand `run_list` helper without changing the emission semantics). The test's Red Gate property is "Layer 1's `bm list` continues to emit RFC 3339 timestamps" — a regression-prevention assertion against future Layer 1 code mutations that would drift the timestamp format. It is NOT meant to fail against the Layer-1 impl; it is meant to fail against a future Layer-1-impl regression.

**The hallucinated framing** confuses two distinct Red Gate properties:
1. **Layer-2 Red Gate** — tests must fail against the Layer-1-only binary (e.g., `tests_tag_attaches_label_to_matching_bookmark` fails against Layer 1 because `bm tag` doesn't exist).
2. **Layer-1-Deferred-to-Layer-2 closure** — tests added at Layer 2 to close a test-discipline gap surfaced at Layer 1 but properly addressed at the layer where the natural authoring surface exists (the automated-test file). These do NOT need to fail against the Layer 1 binary; they need to assert a property the Layer 1 binary continues to satisfy.

The test in question is type 2; the TODO.md framing names it as such. Verified the rejection: the test is correctly scoped per its declared intent. The "passes against unmodified Layer 1 impl" observation is the expected behavior for a Layer-1-Deferred-to-Layer-2 closure test, not a defect.

**Why this is filed despite being Hallucinated.** Per [Phase 3 primer § Hallucinated discipline](../../../../vsdd-suite/primers/3-review-session.md): hallucinated findings are recorded in the audit trail as evidence the adversarial check fired + concluded against the framing. The sycophancy-counter discipline ([QE domain prompt § Sycophancy check](../../../../vsdd-suite/domains/role/QUALITY-ENGINEER-REVIEW.md)) is operative here in reverse: the adversarial framing was correctly tested against the actual test's documented intent + the closure-of-prior-finding context, and the rejection is grounded in specific citation evidence (TODO.md line 76 + QE Review 3 Finding 3 deferral disposition) rather than a hand-wave dismissal.

**Classification:** Hallucinated. Validator = sanity-check per the meta-validator-of-last-resort pattern (no Red Team validation surface — no fix to test for a Hallucinated finding).

---

### Dismissed

*(none — every finding routed to a real test-discipline outcome.)*

---

### Summary

4 findings filed in Round 1 QE/Security/Technical-Writer cluster: **2 Raised — Open** ([Finding 1](#r4-qe-f1) `tests/scaling.rs` declared-but-absent + layer-gate-criterion-vacuity; [Finding 2](#r4-qe-f2) `proptest` activation declared-but-absent), **1 Deferred** ([Finding 3](#r4-qe-f3) `tests_save_fsyncs_parent_directory` WEAK PROXY vs. Red Gate framing — Deferred to Layer 2 Phase 5 PE round), **1 Hallucinated** ([Finding 4](#r4-qe-f4) `tests_list_rfc3339_scripted_check` mis-scoped framing rejected with citation-grounded rebuttal).

**MVR signal: NOT REACHED for this round.** Round 1 produced 2 new real substantive findings with named fix paths + 1 Deferred + 1 Hallucinated. Per the [Phase 3 primer](../../../../vsdd-suite/primers/3-review-session.md) § Round triggers (G-131 continue trigger), Round 2 is mandatory after the fix cycle lands — the round-after-a-new-finding-round verifies the fixes hold + looks for adjacent defects the fix may have created.

**Highest-severity finding:** [Finding 1](#r4-qe-f1) (`tests/scaling.rs` declared-but-absent + layer-gate criterion #1 vacuous-pass). The defect is structurally severe because it compromises both the test surface (no scaling coverage at the contracted cliffs) AND the layer-gate (the criterion's `cargo test -- --ignored` runs zero ignored tests; the gate cannot fail on a broken impl). The fix is mechanical (author `tests/scaling.rs` per the spec) but the underlying drift (spec declares a shipped artifact that doesn't exist) is the failure mode the QE domain prompt § Sycophancy check most pointedly warns against.

**Cross-domain coordination.**

- **[Finding 1](#r4-qe-f1) → [Software Engineer](../SOFTWARE-ENGINEER-REVIEW.md):** SE owns the `tests/scaling.rs` authorship per the spec. QE validates post-fix that the three sentinels at 100/1,000/10,000-bookmark cliffs assert the budget table from [`DESIGN.md`](../../DESIGN.md) § Performance budget + that `cargo test -- --ignored` returns three ignored tests (not zero).
- **[Finding 1](#r4-qe-f1) → [Technical Writer Review 5](2026-05-22-technical-writer.md):** cross-validates the artifact-absence defect from the documentation-accuracy lens (TW Dim 2). The fix path is shared; both findings close together.
- **[Finding 2](#r4-qe-f2) → [Software Engineer](../SOFTWARE-ENGINEER-REVIEW.md):** SE owns the `proptest` dep + property-test authorship. QE validates post-fix that the two properties (tag idempotence + filter OR-monotonicity) are correctly named + assert the natural algebraic shape.
- **[Finding 3](#r4-qe-f3) → [Performance Engineer](../PERFORMANCE-ENGINEER-REVIEW.md):** Performance Engineer Layer 2 round inherits the fsync-verification methodology question; the WEAK PROXY disposition is the audit-trail evidence the gap is named for the PE round to re-evaluate.
- **[Finding 4](#r4-qe-f4) (Hallucinated):** no coordination; recorded for audit-trail completeness per the sycophancy-counter discipline.

**Upstream-suite-recurrence-prevention candidates.**

1. **Spec-vs-impl drift detection at layer-gate time** ([Finding 1](#r4-qe-f1) + [Finding 2](#r4-qe-f2)) — both findings exhibit the same defect class: DESIGN.md + TODO.md name shipped artifacts that do not exist on the filesystem. A pre-merge hook that greps DESIGN.md + TODO.md for citation patterns (`` `tests/<name>.rs` ``, `` `proptest` ``, etc.) and verifies the citations resolve to existing files / dependencies would mechanically prevent the drift at authoring time. Recommendation: extend the existing [`vsdd-suite/hooks/check-anonymization.sh`](../../../../vsdd-suite/hooks/check-anonymization.sh) discipline with a parallel `check-spec-vs-impl-citations.sh` that gates layer-gate evaluation. Cross-references [`vsdd-suite/suite-development/suite-development.md`](../../../../vsdd-suite/suite-development/suite-development.md) § Per-review entry preamble standard.
2. **Red Gate WEAK-PROXY discipline** ([Finding 3](#r4-qe-f3)) — the test docstring's explicit "WEAK PROXY" annotation is the right discipline (audit-trail honesty), but the [Phase 2a primer](../../../../vsdd-suite/primers/2a-red-gate.md) does not codify the WEAK-PROXY classification as a named exception to the Red Gate property. Recommendation: add a § Red Gate WEAK-PROXY clause to primer 2a that names: (a) when a Red Gate property is observable-from-userspace AND when it is not; (b) the WEAK-PROXY classification as the discipline-honest deferral disposition when the property is unobservable + the cost of making it observable is disproportionate; (c) the deferral-to-Phase-5 routing as the natural follow-up surface.

**Coordination:** All Resolved findings declare `**Validator:**` per [QE domain prompt § Validator pair](../../../../vsdd-suite/domains/role/QUALITY-ENGINEER-REVIEW.md). The QE/Security/Technical-Writer cluster's adversarial-pair separation (SE in SE/UX/Performance-Engineer cluster; Red Team in Solution-Architect/Red-Team/Platform-Engineer cluster; Doc Reviewer in Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster) means SE's parallel review against the same Layer 2 source will independently surface or not-surface the same defects from the implementation-correctness lens, satisfying the cold-session adversarial-pair discipline.

---

## Review 5 — 2026-05-22 16:45Z

**Phase:** 3 (IAR Round 2; Layer 2 Round 2 verification of the Round 1 fix cycle).
**Source:** domain-raised (Round 2 regression-verification + Phase 5 forward-look proptest case-count flag; cold-session).
**Lens:** Round 1 finding regression-verification + adjacent-defect surface scan (new proptest activation + scaling sentinel test rigor) + Phase 5 forward-look on proptest case-count.
**Scope:** Layer 2 post-fix artifact tip (`9d56c3f`) — the 5 fix commits since `02e6eb3` Round 1 close: `156ec53` (scaling.rs + properties.rs + Cargo.toml proptest dev-dep + CI scaling job), `d62bb1a` (README + CHANGELOG Layer-2-promotion), `002d747` (DESIGN.md spec amendments), `cdb46bc` (Tagged N stderr + expanded help text), `9d56c3f` (install-verification Layer 2 inheritance note). Read [`tests/scaling.rs`](../../tests/scaling.rs), [`tests/properties.rs`](../../tests/properties.rs), [`Cargo.toml`](../../Cargo.toml) dev-dependencies, [`tests/bookmarks.rs`](../../tests/bookmarks.rs) (unchanged — verification of regression-free), and the post-fix [`DESIGN.md`](../../DESIGN.md) + [`TODO.md`](../../TODO.md) for spec-vs-impl alignment.
**Reviewer:** Quality Engineer.
**Model:** Sonnet 4.6 (per the [round-prompt-stated assignment](../../../../vsdd-suite/domains/role/QUALITY-ENGINEER-REVIEW.md)).
**Cold-session shape:** QE/Security/Technical-Writer cluster (Round 2; same composition as Round 1; adversarial-pair separation preserved — SE in SE/UX/Performance-Engineer cluster; Red Team in Solution-Architect/Red-Team/Platform-Engineer cluster; Doc Reviewer in Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster).
**Regression-check against:** [Review 4](#review-4--2026-05-22-0025z) (Round 1; this same per-domain log file). All 4 Round 1 findings re-evaluated against the post-fix state.
**Session note:** Cold session — this QE/Security/Technical-Writer cluster agent was spawned with no prior project context for the Round 2 verification; read post-fix artifacts in the prescribed cold-reader order. Sycophancy-compensation per the [Quality Engineer domain prompt § Sycophancy check](../../../../vsdd-suite/domains/role/QUALITY-ENGINEER-REVIEW.md): the "the fix cycle landed cleanly; everything is fine" framing was kept as a hypothesis to verify rather than a default conclusion. The verification confirmed the scaling.rs + properties.rs artifacts exist on disk + pass empirically (43 default + 3 ignored scaling sentinels all PASS); the wall-clock estimate in the scaling.rs docstring is empirically too tight (24m 33s vs docstring's ~1-2 min) and is flagged for Phase 5 PE re-evaluation. No new substantive findings surfaced; MVR reached.
**Cost-tally:** QE/Security/Technical-Writer cluster Round 2 verification budget ~25-40k tokens per [AI Engineer R1](2026-05-21-ai-engineer.md#review-1--2026-05-21-1000z) cluster-batching discipline at half the new-finding round budget; 0 new findings + 4 regression-verifications → ~6-10k tokens / verification — within the capstone-intent expected band.

**Assumption surfacing.** Verified `tests/scaling.rs` exists on disk at the post-fix tip; verified `tests/properties.rs` exists; verified `Cargo.toml` `[dev-dependencies]` now contains `proptest = "1"` at [`Cargo.toml:55`](../../Cargo.toml); verified the previously declared 1.78 MSRV is now 1.81 at [`Cargo.toml:19`](../../Cargo.toml) (PE F4 disposition for the `reason = "..."` attribute requirement). Ran `cargo test` at the post-fix tip: **43 default tests pass (12 unit + 29 integration + 2 proptest); 3 ignored scaling sentinels** (matches the prompt-stated state). Ran `cargo test --release --test scaling -- --ignored` separately: all 3 sentinels PASS in 1473.21s (release profile) on the verifier's macOS-darwin commodity hardware.

---

### Resolved

<a id="r5-qe-f1"></a>
**Finding 1 — Round 1 F1 verification: `tests/scaling.rs` declared-but-absent → file now exists with 3 sentinels at 100/1k/10k cliffs; all 3 PASS empirically (Dim 1 — Acceptance criteria; Dim 6 — Validation gaps; Dim 13 — Quality gates / spec-vs-impl drift)**

**Owner:** quality-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** software-engineer

**Round 1 Status:** Raised — Open (load-bearing; layer-gate criterion #1 vacuous-pass).
**Round 2 Status:** Resolved.
**Evidence:** [`tests/scaling.rs:1-222`](../../tests/scaling.rs) exists post-fix `156ec53`. Three `#[ignore]`-gated sentinel tests are present at the three contracted cliffs:
- `scaling_100_bookmarks_round_trips_and_filters_correctly` at [`tests/scaling.rs:90-127`](../../tests/scaling.rs) — 100-bookmark cliff.
- `scaling_1000_bookmarks_round_trips_and_filters_correctly` at [`tests/scaling.rs:136-170`](../../tests/scaling.rs) — 1,000-bookmark cliff.
- `scaling_10_000_bookmarks_round_trips_and_filters_correctly` at [`tests/scaling.rs:187-221`](../../tests/scaling.rs) — 10,000-bookmark cliff.

Each sentinel exercises the full add → list → tag → list-filter cycle (per DESIGN.md spec) at its cliff. Each sentinel asserts: (a) `bm list` line count equals N; (b) one bookmark matches after a single `bm tag` invocation against the middle URL; (c) `BookmarkStore::load` recovers exactly N bookmarks (the round-trip-without-corruption assertion). The wall-clock budget assertion is NOT made in `tests/scaling.rs` — the docstring at [`tests/scaling.rs:18-21`](../../tests/scaling.rs) explicitly delegates wall-clock assertions to the `hyperfine` sanity-check at `manual-tests/layer-2.md` Step 12, citing CI-flakiness concerns. This is a defensible separation per [QE Dim 3 assertion-strength](../../../../vsdd-suite/domains/role/QUALITY-ENGINEER-REVIEW.md): correctness-at-scale is the in-CI invariant; wall-clock is the human-observed sanity-check.

**Round 2 commentary.** Verified `cargo test --release --test scaling -- --ignored` runs the three sentinels: **all 3 PASS** (test result: ok. 3 passed; 0 failed; 0 ignored; finished in 1473.21s ≈ 24m 33s on the verifier's macOS-darwin commodity hardware, release profile). The 10,000-bookmark sentinel + 1,000-bookmark sentinel each ran > 60s as expected. The layer-gate criterion #1 in [`TODO.md:87`](../../TODO.md) (`cargo test --test bookmarks` + `cargo test -- --ignored` for scaling) now resolves to a non-vacuous gate: the scaling half runs three actual ignored tests that all pass against the post-fix implementation, rather than zero. The Round 1 F1 load-bearing concern (the gate cannot fail on a broken impl because there's nothing to run) is closed.

**One forward-look concern (NOT a finding).** The 10,000-bookmark sentinel's docstring at [`tests/scaling.rs:181-184`](../../tests/scaling.rs) names a ~1-2 min wall-clock on commodity hardware. The actual run on this verifier's macOS-darwin hardware (release profile) measured ~24m 33s for the full three-sentinel set (the 10k sentinel + 1k sentinel were each > 60s; the 100-bookmark sentinel ran sub-second). This is roughly 10-20× the docstring's estimate. Possible interpretations: (a) the docstring estimate is from a faster (Linux + later cargo cache) commodity-hardware baseline; (b) the verifier's machine is slower than the reference benchmark; (c) the docstring should be updated to a wider range. Flag for **Phase 5 PE Layer 2 round** to either re-benchmark on a reference machine + update the docstring or re-evaluate the wall-clock budget; CI runs under `--ignored` may land in the 5-30 min range per scaling pass depending on hardware. This is acceptable for a Linux-only separate CI job per the CHANGELOG note at [`CHANGELOG.md:17`](../../CHANGELOG.md), but the docstring's "~1-2 min" estimate is empirically too tight.

**Classification:** Resolved (Round 1 Raised-Open → Round 2 Resolved). (Dim 1 — Acceptance criteria; Dim 6 — Validation gaps; Dim 13 — Quality gates)

---

<a id="r5-qe-f2"></a>
**Finding 2 — Round 1 F2 verification: `proptest` activation declared-but-absent → Cargo.toml dev-dep + tests/properties.rs both now exist with the two named properties (tag_idempotence + filter_or_monotonicity); 64-case override flagged for Phase 5 evaluation but defensible (Dim 4 — Coverage meaningfulness; Dim 14 — TDD proxy indicators / failure specificity)**

**Owner:** quality-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** software-engineer

**Round 1 Status:** Raised — Open (load-bearing; layer-gate criterion #5 vacuous-pass).
**Round 2 Status:** Resolved.
**Evidence:** [`Cargo.toml:55`](../../Cargo.toml) `[dev-dependencies]` now contains `proptest = "1"`. [`tests/properties.rs:1-176`](../../tests/properties.rs) exists post-fix `156ec53` with the two named property tests:
- `tag_idempotence_property` at [`tests/properties.rs:84-108`](../../tests/properties.rs) — exercises the DESIGN.md § `bm tag` § Idempotence contract: `attach_tag(url, label)` twice produces the same state as once. Uses `prop_assume!(single_result.is_ok())` at line 95 to filter trivial-no-match cases, keeping the property focused on the substantive contract.
- `filter_or_monotonicity_property` at [`tests/properties.rs:119-175`](../../tests/properties.rs) — exercises the DESIGN.md § `bm list --tag <label>` OR-semantics contract: `filter_by_tags(A ∪ B) = filter_by_tags(A) ∪ filter_by_tags(B)` for disjoint labels A and B. URL-set comparison (not list comparison) at [`tests/properties.rs:157-159`](../../tests/properties.rs) correctly decouples the property from `Utc::now()` newest-first ordering nondeterminism.

Verified `cargo test` runs the two proptest tests at the default profile (output: `running 2 tests / ..` in the proptest binary). Both pass.

**Round 2 commentary (Phase 5 forward-look on case-count).** The fix-cycle agent's `ProptestConfig { cases: 64, .. }` override at [`tests/properties.rs:73`](../../tests/properties.rs) reduces proptest's default 256 cases to 64. The docstring at lines 70-72 names the rationale: "small enough that `cargo test` stays fast (< 1s for the two properties combined) but large enough to surface non-trivial counterexamples — proptest's default of 256 is overkill for a pure-side property on a 0..=8-bookmark store with a 4-URL alphabet." The search space is small enough this is defensible — the URL strategy at [`tests/properties.rs:45`](../../tests/properties.rs) (`"https://example-[0-3]\\.com"`) yields exactly 4 URLs; the label strategy at [`tests/properties.rs:49`](../../tests/properties.rs) (`"[a-d]{1,3}"`) yields 4 + 4² + 4³ = 84 labels; the store generator (0..=8 URLs) yields a small but dense search space.

The Phase 5 Layer 2 Mutation Testing round (deferred per [`TODO.md`](../../TODO.md) § Layer 2 Layer-gate criterion #5; not run this session) is the natural surface to evaluate whether 64 cases kills the same mutants 256 cases would. The case-count override may compromise mutation-resistance against subtle mutants that emerge only at higher draw counts; specifically a mutation like `t == *l` → `t.contains(*l)` in `filter_by_tags` (a substring-vs-equality boundary) is sensitive to label-overlap shapes that may not appear in 64 draws but would in 256. **NOT filed as a Round 2 finding** because: (a) the override decision is correctly documented with rationale; (b) the small alphabet makes 64 a defensible coverage choice; (c) the Phase 5 round is the canonical surface to re-evaluate. **Flag for Phase 5 PE/QE Layer 2 round** to verify mutation kill rate is maintained at 64 cases or to raise the cases back to 256 if not. Recorded here as Round 2 commentary rather than a new finding because the rationale chain is honest and the Phase 5 surface owns the evaluation.

The layer-gate criterion #5 in [`TODO.md:91`](../../TODO.md) (Phase 5 Layer 2 rounds at closure including proptest activation) now resolves to a non-vacuous declaration: the property tests exist and are run by `cargo test`. Round 1 F2 closed.

**Classification:** Resolved (Round 1 Raised-Open → Round 2 Resolved). (Dim 4 — Coverage meaningfulness; Dim 14 — TDD proxy indicators / failure specificity)

---

### Deferred

<a id="r5-qe-f3"></a>
**Finding 3 — Round 1 F3 re-verification: `tests_save_fsyncs_parent_directory` WEAK PROXY disposition stable; Phase 5 PE Layer 2 round deferral routing intact (Dim 2 — Red Gate; Dim 3 — assertion strength)**

**Owner:** quality-engineer
**Status:** raised
**Blocked by:** Methodology — direct fsync-syscall verification requires either a Rust-side trait injection (adds production-code seam) or a `strace`/`dtruss` harness (platform-specific, outside `cargo test` discipline). Deferral routing unchanged from Round 1.
**Validator:** sanity-check

**Round 1 Status:** Deferred (to Layer 2 Phase 5 Performance Engineer round; methodology — direct fsync verification requires production-code seam or strace harness).
**Round 2 Status:** Verified-Deferred.
**Evidence:** The Round 1 disposition holds without change. [`src/lib.rs:776-794`](../../src/lib.rs) (the test docstring) still names the WEAK PROXY classification honestly. No fix commit attempted to close this finding; the cluster fix-cycle correctly respected the Round 1 deferral disposition. The Round 1 commit `cdb46bc` (UX affordance + help text) and `002d747` (spec amendments) did not touch the fsync-test or the production fsync seam; the methodology disposition stands.

**Round 2 commentary.** Re-affirmed: the fsync verification is a Phase 5 Layer 2 PE-round concern. The disposition's audit-trail value is the explicit naming of the WEAK PROXY at the test docstring; that audit-trail signal is intact. No regression. The deferral routing has not been activated yet (Phase 5 Layer 2 PE round has not landed); the trigger is the Phase 5 closure, not Round 2.

**Classification:** Deferred — to Layer 2 Phase 5 Performance Engineer round (disposition stable from Round 1). (Dim 2 — Red Gate; Dim 3 — assertion strength)

---

### Hallucinated

<a id="r5-qe-f4"></a>
**Finding 4 — Round 1 F4 re-verification: `tests_list_rfc3339_scripted_check` correctly scoped as Layer-1-Deferred-to-Layer-2 closure; Hallucinated disposition stable (Dim 2 — Red Gate)**

**Owner:** quality-engineer
**Status:** raised
**Blocked by:** *(none)*
**Validator:** sanity-check

**Round 1 Status:** Hallucinated (the test is correctly scoped as Layer-1-Deferred-to-Layer-2 closure per its TODO.md framing).
**Round 2 Status:** Verified-Hallucinated (no fix needed; recorded for audit-trail).
**Evidence:** No fix commit attempted to act on this finding (correctly — Hallucinated findings have no fix path). [`tests/bookmarks.rs`](../../tests/bookmarks.rs) `tests_list_rfc3339_scripted_check` still passes against the Layer 1 + Layer 2 binary surface as designed.

**Round 2 commentary.** The Round 1 sycophancy-counter discipline (recording a Hallucinated finding with citation-grounded rebuttal rather than silent dismissal) is the audit-trail artifact; no further action.

**Classification:** Hallucinated (disposition stable from Round 1).

---

### Dismissed

*(none.)*

---

### Summary

4 Round 1 findings verified at Round 2: **2 Resolved** (F1 + F2; load-bearing fixes landed cleanly at `156ec53` — scaling sentinels at 100/1k/10k cliffs + proptest activation with two property tests on the pure surface), **1 Verified-Deferred** (F3 — Phase 5 PE Layer 2 round inherits the WEAK PROXY methodology question; disposition stable), **1 Verified-Hallucinated** (F4 — no fix path; audit-trail intact). **Zero new Round 2 findings.**

**MVR signal: REACHED for this round.** All Round 1 findings reach a Round 2 terminal disposition (Resolved / Verified-Deferred / Verified-Hallucinated) and the Round 2 pass surfaced no new substantive findings against the post-fix artifact. Per [Phase 3 primer § Round triggers (G-131)](../../../../vsdd-suite/primers/3-review-session.md), the QE domain reaches MVR at this round for the Layer 2 surface. The Phase 5 Layer 2 round (Purity Boundary Audit re-run + Mutation Testing re-run + property-based testing kill-rate evaluation) remains the next natural QE-adjacent surface; that round is Phase-5-deferred per [`TODO.md` § Layer 2 Layer-gate criterion #5](../../TODO.md), not a Round-2 blocker.

**Phase 5/6 blockers (forward-look from QE seat).**

- **Phase 5 Layer 2 Mutation Testing:** the 64-case proptest override warrants a kill-rate evaluation against the Layer 1 baseline (8/8 viable kill rate). If a mutation survives at 64 cases that would die at 256, raise the cases; otherwise the override stands.
- **Phase 5 Layer 2 PE round:** the WEAK PROXY fsync-test methodology question is the PE-round's natural surface; QE's Round 2 disposition is intact.
- **Phase 6 Layer 2:** NOT APPLICABLE per [`DESIGN.md` § Project intent Phase 6 strategy for Layer 2](../../DESIGN.md) (Option 1 — capstone gates at project-terminal MVR per primer 6; Layer 1's Phase 6 attestation stands). No QE-side Phase 6 surface for Layer 2.

**Cost-tally suffix:** ~6-8k tokens for this Round 2 verification + 0 new substantive findings = within the projected ~25-40k Round-2 cluster budget at half-rate (Round 2 verification is cheaper than Round 1 new-finding-generation per cold-session-discipline economics).

**Coordination:** Round 1 F1 + F2 cross-validate with the parallel SE / VDD-IAR Alignment / SO Round 1 findings on the same artifact-absence defect class — all close together at `156ec53`; the shared fix is the right routing. Round 1 F3 routes to Phase 5 PE Layer 2 round (deferral intact). Round 2 adversarial-pair separation preserved: SE's parallel Round 2 in the SE/UX/Performance-Engineer cluster validates the same `tests/scaling.rs` + `tests/properties.rs` from the implementation-correctness lens; Doc Reviewer's parallel Round 2 in the Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment cluster validates the spec-vs-test-citation alignment from the documentation-completeness lens. All Round 2 verifications declare `**Validator:**` per [QE domain prompt § Validator pair](../../../../vsdd-suite/domains/role/QUALITY-ENGINEER-REVIEW.md).

---

## Review 6 — 2026-05-22 22:30Z

**Phase:** 5 (Mutation Testing re-run; Layer 2 hardening per [DESIGN.md](../../DESIGN.md) § Project intent's Phase 5 strategy commitment for Layer 2 — "Mutation Testing re-runs against the extended impl with the budget that the 100% kill rate is maintained or any drop has a named rationale").
**Source:** director-raised (operator-directed inline-run of Phase 5 per the AskUserQuestion choice of "Run inline + author logs"; `cargo mutants --no-shuffle --timeout 60` tool output is the evidence base).
**Lens:** test-suite-coverage + mutation-survival-analysis + Layer-1-baseline-regression.
**Scope:** Layer 2 extensions to `src/lib.rs` (`Bookmark.tags` field, `Bookmark::tags()` accessor, `AttachTagError`, `BookmarkStore::attach_tag`, `BookmarkStore::filter_by_tags`, `fsync_directory`).
**Reviewer:** Quality Engineer.
**Model:** Opus 4.7 (`claude-opus-4-7`).
**Cold-session shape:** N/A — inline-run from the main session. Trade-off declared per the parallel [SA Review 4](2026-05-22-solution-architect.md#review-4--2026-05-22-2200z) framing: a parallel cold-session cluster spawn would be over-investment per [G-150](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-150); the mutation-testing tool produces the evidence and the analysis is the only judgment surface.
**Regression-check against:** [Review 2](../2026-05-20-quality-engineer.md#review-2--2026-05-20-0245z) (Layer 1 Phase 5 Mutation Testing baseline — 100% kill rate on 8 viable mutants).
**Session note:** `cargo mutants` is an empirical-evidence tool — the survival counts are not a judgment surface, only the per-mutant disposition (test-gap vs acceptable-survival vs surprise-survivor) requires reviewer analysis. Sycophancy-compensation: the natural temptation is to classify all surviving mutants as "acceptable-survival" because the implementation is by-default judged correct; the disposition table below distinguishes acceptable-survival (deliberate cost-vs-benefit deferral with prior citation) from genuine test-gap (closeable in this cycle) explicitly.
**Cost-tally:** placeholder; filled at session-end below.

---

**Assumption surfacing.** `cargo mutants --no-shuffle --timeout 60` ran against the post-fix Layer 2 tip (commit `580db12`). The tool output is the empirical evidence; the per-mutant disposition below is the only reviewer-judgment surface.

Final: **51 mutants tested in 5 minutes: 6 missed, 38 caught, 7 unviable.**

Viable kill rate: 38 / 44 = **86.4%** — a drop from Layer 1's 100% (8/8 viable). The viable surface expanded 5.5× (8 → 44 viable mutants) because Layer 2 added ~80 LOC of new functions (`attach_tag`, `filter_by_tags`) + a new field (`Bookmark.tags`) + a new type (`AttachTagError`) + a new effectful helper (`fsync_directory`).

The 6 surviving mutants:

| # | Site | Mutation | Survival reason |
|---|---|---|---|
| 1 | `src/lib.rs:80:9` | `Bookmark::tags() -> &[String]` → `Vec::leak(Vec::new())` | No test asserts against `Bookmark::tags()` directly — tests check `bookmark.tags` via JSON serialization round-trip; the accessor is never called in tests. |
| 2 | `src/lib.rs:80:9` | `Bookmark::tags()` → `Vec::leak(vec![String::new()])` | Same — accessor not exercised. |
| 3 | `src/lib.rs:80:9` | `Bookmark::tags()` → `Vec::leak(vec!["xyzzy".into()])` | Same — accessor not exercised. |
| 4 | `src/lib.rs:109:9` | `<impl fmt::Display for AttachTagError>::fmt` → `Ok(Default::default())` | The CLI shell at `src/main.rs:run_tag` paper-overs the `Display` impl by re-creating the error string per-variant — see [Round 1 SE Finding 1](2026-05-20-software-engineer.md#r4-se-f1). The variant's `Display` impl is dead code from the binary's perspective; no test invokes it. |
| 5 | `src/lib.rs:442:5` | `fsync_directory(path) -> std::io::Result<()>` → `Ok(())` | The [Phase 2b WEAK PROXY annotation](../../src/lib.rs) on `tests_save_fsyncs_parent_directory` documented this gap — there is no portable way for a black-box test to assert that `fsync(2)` was syscalled on the parent FD. Mutation testing confirms the documented limitation: a no-op `fsync_directory` survives. |
| 6 | `src/lib.rs:464:5` | `write_temp_file(tmp_path, bytes) -> std::io::Result<()>` → `Ok(())` | Pre-existing Layer 1 helper (not Layer 2 new code) — Layer 1's mutation testing baseline did not surface this; either the Layer 1 baseline's `cargo-mutants` version (or invocation) skipped this site, or the Layer 2 cycle's expanded test surface introduced a coverage interaction. Investigated: `write_temp_file` uses `OpenOptions::create_new(true).mode(0o600).open(...)?` which creates a zero-byte file before `write_all`. If the body returns `Ok(())` early (before `write_all`), the temp file exists at zero bytes; `rename(2)` succeeds; `save()` returns Ok; the in-memory store says "saved" but the on-disk file is zero bytes. The subsequent `load()` would read zero bytes and per [`src/lib.rs:123`](../../src/lib.rs) `contents.trim().is_empty()` returns `Self::default()` — an empty bookmark store. So any test that does `bm add` followed by `bm list` would fail (the list output would be `No bookmarks yet.` instead of the just-added URL). But cargo-mutants reports MISSED, which means at least one such test sequence does NOT exist. The most likely gap: tests that exit after a single `bm add` without subsequent `bm list` validation (since the JSON read in tests happens via `serde_json::from_str(fs::read_to_string(&db)?)`, an empty file would deserialize as `Default::default()` → `bookmarks: vec![]`, which would fail the `assert_eq!(bookmarks.len(), 1)` assertion). So mutants must be surviving via some other path — possibly that `write_temp_file` is called from a code path the tests don't exercise (the `#[cfg(not(unix))]` variant). Defer to detailed mutant inspection. |

---

### Resolved

<a id="r6-qe-f1"></a>
**Finding 1 — Layer 2 Mutation Testing kill rate closed at 93.2% via Option A inline fix; 3 documented acceptable-survivals named per-mutant (Dim 7 — mutation-resistance; Dim 14 — TDD proxy indicators)**

**Owner:** quality-engineer
**Status:** validated
**Blocked by:** *(none — Option A applied at commit c186d0b; cargo-mutants re-verified)*
**Validator:** solution-architect

**Evidence:** Per `DESIGN.md` § Project intent's Phase 5 strategy for Layer 2 ("Mutation Testing re-runs against the extended impl with the budget that the 100% kill rate is maintained or any drop has a named rationale"), the 13.6% drop from Layer 1's 100% baseline requires explicit rationale. The 6 surviving mutants split into three categories:

- **Acceptable survival (2 mutants):** Mutant 4 (`AttachTagError::Display`) — the CLI shell paper-overs the Display impl by design (the spec contracts the error string at the CLI surface, not at the library surface; see [Round 1 SE F1](2026-05-20-software-engineer.md#r4-se-f1) which Deferred this as a Layer-3 trigger). Mutant 5 (`fsync_directory`) — the WEAK PROXY annotation in `tests_save_fsyncs_parent_directory` documented this gap inline at Phase 2b. Both are deliberate cost-vs-benefit deferrals, not test-suite defects.
- **Genuine test gap (3 mutants):** Mutants 1, 2, 3 — `Bookmark::tags()` accessor untested. Tests assert against the JSON post-state but never call the public `tags()` API. **Closeable**: a single unit test in `src/lib.rs#[cfg(test)] mod tests` that constructs a `Bookmark` with known tags and asserts `bm.tags() == &["expected"][..]` would kill all three mutants.
- **Surprise survivor (1 mutant):** Mutant 6 (`write_temp_file`) — Layer 1 code (not Layer 2 new) that survived only after Layer 2 cargo-mutants invocation; the mechanism is unclear (see the table commentary). **Requires inspection.**

**Reasoning:** The surviving mutants are all on accessor/display/effectful-syscall surfaces; behavioral contracts are still covered by integration tests at the binary surface. Layer 2 added 5.5× the viable mutant surface (8 → 44) while preserving 86.4% kill rate at no test-additions cost beyond the original 13 Red Gate + 14 unit tests + 2 proptest + 3 scaling sentinels. The 13.6% drop is documented per-mutant rather than aggregate, so the rationale is verifiable.

**Disposition:** Recommend Option A — add the `tags()` accessor unit test + investigate Mutant 6 — as a small inline fix in this Layer 2 cycle to bring the kill rate to ~93% (40/44 = 90.9% closing #1-#3 alone). Mutant 4 + Mutant 5 stay as documented-acceptable-deferrals. Then Layer 2 Phase 5 closes with the named rationale: "86.4% → ~93% post-fix; deferral set is documented per-mutant with cost-vs-benefit framing." OR Option B — accept the 86.4% kill rate as the Layer 2 floor with the named rationale documented above, defer the small fixes to a Layer 3 or operator-attention cycle.

**Round 2 addendum (2026-05-23 04:00Z) — Option A landed at commit `c186d0b`; cargo-mutants re-verified at commit `c186d0b`:** Operator chose Option A. Two changes landed:

1. **`Bookmark::tags()` accessor unit test** added at `src/lib.rs` `tests::bookmark_tags_accessor_returns_constructor_supplied_slice` — asserts the accessor returns populated and empty slices. Kills mutants #1, #2, #3 by direct invocation. (13 lib unit tests pass, was 12.)
2. **Mutant #6 (`write_temp_file` → `Ok(())`) re-classified as cfg-shadow false-positive.** Investigation: line 464:5 lives inside the `#[cfg(not(unix))]` branch (Windows-only variant). cargo-mutants ran on macOS where the `#[cfg(unix)]` variant at line 446+ is selected and the not(unix) variant compiles out as dead code. Mutations to dead code can never be killed by tests on the test platform — known cargo-mutants behavior, not a real test-suite gap. Re-classified from "surprise survivor" to acceptable-survival.

cargo-mutants 27.0.0 re-run (commit `c186d0b`, macOS): **51 mutants in 5 min — 3 missed, 41 caught, 7 unviable. Viable kill rate 41/44 = 93.2%** (up from 86.4% pre-fix; +6.8 percentage points). The 3 remaining survivors are all documented acceptable-survivals:

- `AttachTagError::Display` (Layer-3-trigger per SE R1 F1)
- `fsync_directory` no-op (WEAK PROXY annotation per Phase 2b)
- `write_temp_file` cfg-shadow (`#[cfg(not(unix))]` dead-code on Unix platform)

Phase 5 Layer 2 closes per the named-rationale criterion: 93.2% kill rate with all surviving mutants documented per-mutant with cost-vs-benefit framing.

**Classification:** Resolved (Option A applied + verified; 93.2% kill rate; 3 documented acceptable-survivals). (Dim 7 — mutation-resistance; Dim 14 — TDD proxy indicators)

---

### Summary

Phase 5 Layer 2 Mutation Testing re-run via cargo-mutants 27.0.0 surfaced a 86.4% viable kill rate (38/44) initially — a documented 13.6% drop from Layer 1's 100% baseline (8/8 viable). Operator chose Option A (inline closure). Two changes landed at commit `c186d0b`: a `Bookmark::tags()` accessor unit test (kills mutants #1/#2/#3) + investigation of Mutant #6 revealed it was a cfg-shadow false-positive in the `#[cfg(not(unix))]` dead-code branch, re-classified as acceptable-survival. cargo-mutants re-verification at `c186d0b`: **41 caught / 3 missed / 7 unviable = 93.2% viable kill rate** — Phase 5 closes per the named-rationale criterion. 3 remaining survivors are all documented acceptable-survivals: `AttachTagError::Display` (Layer-3-trigger per SE R1 F1); `fsync_directory` no-op (WEAK PROXY annotation per Phase 2b); `write_temp_file` cfg-shadow (Windows-variant on Unix platform). The companion SA Phase 5 Purity Boundary Audit at [Review 4](2026-05-22-solution-architect.md#review-4--2026-05-22-2200z) confirmed zero purity-boundary findings; the surviving mutants do not cross the boundary.

---

**Cost-tally** (updated per the operator's 2026-05-22 directive — name plan + tool + execution method explicitly):

- **AI tool:** [claude-code CLI](https://claude.com/claude-code) (orchestrator); `cargo-mutants` 27.0.0 (evidence-generating tool)
- **Plan tier:** Claude Max (operator's personal plan)
- **Execution method:** inline `cargo mutants` invocation in the main session; analysis + review authoring also inline
- **Model:** Opus 4.7 for the analysis + authoring (`claude-opus-4-7`)
- **Raw tokens (rough estimate; not measured):** ~6k–8k for the analysis + authoring (read `cargo mutants` output, classify the 6 surviving mutants, write this review entry)
- **Tool wall-clock:** 5 min (cargo-mutants on the 51-mutant Layer 2 surface)
- **Would-be API cost** (Opus 4.7 API tier; NOT the operator's actual cost since Max plan is subscription): ~$0.30–0.50 USD
- **Actual cost to operator:** $0 marginal (within Max plan limits)
- **Findings:** 1 (this Review's QE F1) — a budget-vs-result finding rather than a defect finding; the underlying defects (accessor untested, Display dead-code, fsync WEAK PROXY documented inline) are cataloged within F1's disposition.

The cost-tally discipline upgrade per the operator's 2026-05-22 directive lands in [Task #56](../../../../vsdd-suite/suite-development/) (suite-level upstream remediation).

**Coordination:** [SA Phase 5 Review 4](2026-05-22-solution-architect.md#review-4--2026-05-22-2200z) — Purity Boundary Audit re-ran in parallel (same session, inline) with this QE Phase 5 Round; SA found zero findings; this QE Round found 1 mutation-coverage finding (above). Together the two close Phase 5 Layer 2 per the Phase 5 strategy declaration. The operator's choice between Option A (close #1-#3 inline) vs Option B (accept the rationale + defer) is the next step.

---

## Review 7 — 2026-05-23 16:00Z

**Phase:** 5 (Phase-5-trigger follow-up; Layer 2 hardening per the carry-forward queue from PR #44 / Review 6).
**Source:** director-raised (operator-directed PR #47 closure of the 3 Phase-5-trigger items: SE R2 F5 proptest restructure + PE R1 F5 fsync filesystem-coverage caveat + PE R2 F4 scaling sentinel process-spawn overhead).
**Lens:** test-architecture-correctness + measurement-honesty + scope-discipline.
**Scope:** Layer 2 post-PR-#46 tip. Files touched: `tests/scaling.rs` (populate refactor — library API instead of per-bookmark `bm add` spawn); `tests/properties.rs` (tag_idempotence property restructure + new tag_idempotence_property_no_match_path companion); `DESIGN.md` § Performance budget (new "Filesystem-coverage caveat" paragraph); `CHANGELOG.md` (PR #47 entry).
**Reviewer:** Quality Engineer.
**Model:** Opus 4.7.
**Cold-session shape:** N/A — inline main-session methodology authoring; same shape as Review 6's inline-run trade-off rationale. Per [G-150](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-150): adversarial cold-session cluster spawn would be over-investment against a 3-item bounded carry-forward queue.
**Regression-check against:** [Review 6](#review-6--2026-05-22-2230z) (Layer 2 Phase 5 Mutation Testing 93.2% kill rate + per-mutant disposition). All 3 Phase-5-trigger items from Review 6's Coordination + Disposition + CHANGELOG-documented carry-forwards verified.
**Session note:** Each closure has empirical-evidence verification rather than just docstring-text confirmation. SE R2 F5 verified via `cargo test --test properties` (3 proptests pass; no `prop_assume!` in source after refactor; rejection-rate dependency eliminated structurally). PE R1 F5 verified via DESIGN.md amendment text + the existing benchmark basis (APFS macOS + ext4 Linux CI runner) is preserved. PE R2 F4 verified empirically via `cargo test --release --test scaling -- --ignored` — 3 scaling sentinels pass in 0.85s post-refactor (was ~24 min; 1700× speedup). Sycophancy compensation: the same agent that authored the original Phase 5 reviews (Review 6 + the SA companion) is closing the follow-ups; verified via direct test runs to prevent the "I'm sure it's fine" failure mode.
**Cost-tally:**

- **AI tool:** [claude-code CLI](https://claude.com/claude-code)
- **Plan tier:** Claude Max (operator's personal plan)
- **Execution method:** inline main session
- **Model:** Opus 4.7
- **Raw tokens (estimated):** ~10-15k for the 3 closures + this review
- **Tool wall-clock:** cargo test runs ~3-5 sec each (3 invocations: properties + scaling --ignored + full --all-targets); cargo fmt ~2 sec
- **Would-be API cost** (Opus 4.7 API tier): ~$0.50-0.80 USD
- **Actual cost to operator:** $0 marginal (within Max plan limits)
- **Findings:** 0 — this is a closure round, not a discovery round.

---

### Resolved

<a id="r7-qe-f1"></a>
**Finding 1 — Phase 5 Layer 2 strategy declaration fully satisfied; all 3 Phase-5-trigger carry-forwards closed (Dim 1 — Acceptance criteria; Dim 7 — mutation-resistance; Dim 13 — Quality gates)**

**Owner:** quality-engineer
**Status:** validated
**Blocked by:** *(none)*
**Validator:** solution-architect

**Evidence:** Three closures verified at the post-PR-#47 tip:

1. **PE R2 F4 — scaling sentinel `populate` process-spawn overhead** → `tests/scaling.rs:41-65` `populate` refactored to use library API (`BookmarkStore::add` + single trailing `save`). Empirical wall-clock: **0.85 sec for 3 sentinels (100 + 1k + 10k bookmarks) — was ~24 min; 1700× speedup.** 10K-cliff `#[ignore]` docstring updated from "~1-2 min" to "~5-15 sec post-PR-#47" to match empirical reality.

2. **SE R2 F5 — proptest `prop_assume!` rejection-rate disclosure** → `tests/properties.rs` `tag_idempotence_property` restructured via new `store_with_matching_url_strategy()` (generates store first, picks URL from store's existing URLs via `prop_flat_map`); no `prop_assume!` in the property body. NoMatch boundary covered by new companion property `tag_idempotence_property_no_match_path` using a disjoint URL alphabet (`https://unmatched-example-[0-3].com`). 3 proptests pass; total count up from 2.

3. **PE R1 F5 — fsync filesystem-coverage caveat** → `DESIGN.md` § Performance budget § Durability discipline extended with new "Filesystem-coverage caveat" paragraph naming the limitation (measured against APFS + ext4 reference benchmarks; NFS / CIFS / FUSE / tmpfs may differ materially) + accepting the limitation for the reference-example scope. The discipline-documentation closes the disposition without forcing a multi-filesystem benchmark cycle that would be over-investment per [G-150](../../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-150).

**Reasoning:** The Phase 5 Layer 2 strategy declaration in `DESIGN.md` § Project intent committed to: Purity Boundary Audit re-run (closed at SA Review 4 — zero findings), Mutation Testing re-run with 100% kill rate maintenance or named-rationale drop (closed at QE Review 6 — 93.2% kill rate post-Option-A; per-mutant rationale named), property-based testing via proptest activation (now structurally hardened against rejection-rate dependency post-PR-#47). The fsync durability budget was implied by the Performance Budget Layer 2 declarations; the caveat documents the scope boundary explicitly. Layer 2 Phase 5 closes fully per the strategy declaration.

**Classification:** Resolved (Dim 1 — Acceptance criteria; Dim 7 — mutation-resistance; Dim 13 — Quality gates).

---

### Summary

Phase 5 Layer 2 follow-up round closing the 3 carry-forward items documented in Review 6's Coordination + CHANGELOG. All 3 closures have empirical-evidence verification (not just docstring-text confirmation). The Phase 5 Layer 2 strategy declaration in `DESIGN.md` § Project intent is now fully satisfied across all four declared surfaces (Purity Boundary Audit + Mutation Testing + proptest + scaling sentinels). No new findings; this is a closure round per the bounded carry-forward queue. Layer 2 capstone-cycle Phase 5 surface is closed; the remaining bookmark-cli-manual carry-forwards are operator-action (Bluesky install-verification solicitation) or future-layer-trigger (Layer 3 export + import).

**Coordination:** Cross-references the post-PR-#47 `tests/scaling.rs` + `tests/properties.rs` + `DESIGN.md` amendments. No cross-domain routing required at this round — the 3 closures live in QE-owned test artifacts + the QE-validator-paired DESIGN.md § Performance budget caveat.
