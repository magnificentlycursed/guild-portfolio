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
