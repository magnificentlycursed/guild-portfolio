# Suite Review — 2026-05-12

## Review 37 — 2026-05-12 12:30Z

**Scope:** `prompts/implementation.md` Red Gate framework (L11 "Tests before code" / L32 "Every new test must fail" / L34 "Commit the Red Gate state before Phase 2b begins" / L56 retroactive-Red-Gate carve-out). Triggered by `issue-tracker-cli` Layer 7 IAR Round 3 VDD-IAR Alignment Review 19 Finding 1, which surfaced a methodology shape the suite's existing framework does not name cleanly: the **warm-finding-closure commit** — a commit that resolves a previously-documented Open/Deferred/Backlogged IAR finding by bundling new tests with the implementation change, where the bundling is structurally required (the resolution is the test target). Project-scoped resolution landed at `issue-tracker-cli/iterative-adversarial-refinement/CLOSURE-PROTOCOL.md` §8; this entry registers the suite-level gap for possible suite promotion.

**Lens:** Defect-class lens — **methodology framing covers fresh-layer Red Gate (Phase 2a fails-first; Phase 2b makes-pass) and one-off Phase-2b discovery (the L56 retroactive carve-out) but does not name the IAR-cadence warm-closure mode that recurs during Round 2+ resolution passes.** Recurrence is the qualifying signal: Layer 7 R17 F1 (single instance, Option B declined per "earned by recurrence" doctrine) → Layer 7 R19 F1 (second instance with three R3 commits bundling tests + implementation without the literal L56 label). The pattern is not specific to polish layers, only first observed there; any IAR-driven refactor or defense-in-depth closure has the same shape.

**Session note:** In-session — this entry is authored by the orchestrator that landed the project-scoped CLOSURE-PROTOCOL.md §8 amendment, not a cold-session reviewer. Sycophancy compensation: the artifact claim is verifiable independently of session context — the four cited commits (`fbbb8a3` L7 R17 F1 Option A retrofit with literal labels; `c341a54`, `bd7511e`, `3fa1f3c` L7 R3 closures without labels) are reproducible against git history; the VDD-IAR R17 F1 Option-B-declined record and the VDD-IAR R19 F1 Option-B-accepted record are in `issue-tracker-cli/iterative-adversarial-refinement/VDD-IAR-ALIGNMENT-REVIEW.md`. The project-scoped amendment at CLOSURE-PROTOCOL.md §8 is the verifiable closure mechanism for R19 F1; the gap-registry entry below is the parallel suite-level disclosure, not a substitute closure.

---

### New gap registered

**G-99 — `prompts/implementation.md` Red Gate framework does not name the warm-finding-closure mode that recurs during IAR Round 2+ cadence.**

The existing framework supports two test states:

1. **Phase 2a Red Gate** (L11/L32) — test fails first, implementation makes it pass. Applies to fresh-layer work where the layer plan named the test before the developer wrote it.
2. **Retroactive Red Gate** (L56 carve-out) — test discovered during Phase 2b, added post-implementation, labelled with the literal `// retroactive Red Gate: <behavior> — discovered during Phase 2b, test added post-implementation, confirmed passes against current implementation.` source comment. Applies to a single test discovered while building the feature.

Neither mode fits the **warm-finding-closure commit** shape that recurs during IAR Round 2+ cadence: a previously-documented Open / Deferred / Backlogged IAR finding is closed via a commit that bundles new tests with the implementation change because the resolution requires both (a refactor + its regression tests; a defense-in-depth assertion + its test; a new helper extraction + the test that pins its contract).

L56's carve-out fits the warm-closure shape formally (a test added post-implementation, confirmed passes) but the FRAMING in L56 ("discovered during Phase 2b") doesn't match — the test wasn't discovered during Phase 2b; it was prescribed by an IAR review in an earlier round. The reviewer in the warm-closure context isn't a developer who noticed something while building; they're an orchestrator applying a fix the IAR process already prescribed.

The methodological consequence at `issue-tracker-cli`: Layer 7 R17 F1 Option A retrofit (`fbbb8a3`) applied the literal label to 12 unit tests retrofitted on color helpers — fits the L56 framing as well as it can. Layer 7 R3 commits `c341a54` / `bd7511e` / `3fa1f3c` did NOT apply the label across their 17 test bodies, on the operator's framing that "warm closure of a documented finding" was a different mode than "discovered during Phase 2b." VDD-IAR R19 F1 flagged the inconsistency and offered Option A (retroactive label retrofit) or Option B (codify the warm-closure mode as a distinct carve-out, earned by the R17→R19 recurrence). Option B was taken, scoped at the project level (CLOSURE-PROTOCOL.md §8).

This gap registers the suite-level shape of the same question: should `prompts/implementation.md` itself add a warm-finding-closure section, parallel to its L56 retroactive carve-out? Doing so would lift the closure from project-scoped CLOSURE-PROTOCOL.md to the suite primer that every project inherits.

**Resolution options for the suite-level question (Open):**

- **(a) Promote project-scoped §8 to a suite-level addition in `prompts/implementation.md`** as a new section after the current L56 paragraph. Cost: one primer edit. Benefit: future projects adopting the IAR suite inherit the warm-closure carve-out by default rather than having to author a project-scoped CLOSURE-PROTOCOL.md §8 of their own.
- **(b) Defer to the suite-level CLOSURE-PROTOCOL.md adoption flow already named at the project CLOSURE-PROTOCOL.md §7.** If `issue-tracker-cli`'s CLOSURE-PROTOCOL.md gets promoted to suite-level (per §7's "Move this file to `guild-portfolio/iterative-adversarial-refinement/CLOSURE-PROTOCOL.md`"), §8 comes with it as part of the promotion bundle. Cost: zero now; the §7 promotion event becomes the trigger.
- **(c) Maintain status quo (project-scoped only) and re-evaluate when a second project encounters the pattern.** The "earned by recurrence" doctrine applied within `issue-tracker-cli` (R17 → R19) but the suite-level recurrence is one project (issue-tracker-cli alone has demonstrated the pattern). A future project's first encounter with the warm-closure shape would either independently author the carve-out (validating it as needed across projects) or import it from `issue-tracker-cli`'s CLOSURE-PROTOCOL.md (validating it via reuse). Either outcome informs the suite-level decision better than a pre-emptive primer edit.

**Recommended:** (c) — defer to natural recurrence. The "rule changes earned by recurrence" doctrine that produced the R19 F1 Option B closure applies symmetrically at the suite level. One project's recurrence pattern is project-scope evidence; suite-level addition should require evidence the pattern recurs across projects. Reactivation trigger: a second project in this portfolio (or any project adopting the IAR suite) encountering the warm-finding-closure shape without an obvious framework fit.

**Type:** Methodology gap.
**Severity:** Mission-critical Medium / Speculative Medium. The gap does not block delivery — projects can author their own project-scoped CLOSURE-PROTOCOL.md §8 as `issue-tracker-cli` did. The gap is in framework completeness, not in operational capability.
**Status:** Deferred — natural-recurrence trigger; coordinated with `issue-tracker-cli` CLOSURE-PROTOCOL.md §8 (which carries the project-scoped resolution).

---

### Coordination

Project-scoped resolution landed at `issue-tracker-cli/iterative-adversarial-refinement/CLOSURE-PROTOCOL.md` §8 (Warm-finding-closure Red Gate carve-out), authored as part of `issue-tracker-cli` Layer 7 IAR Round 3 closure (VDD-IAR R19 F1 Option B). The project-scoped carve-out cites this gap-registry entry for the suite-level question.

Suite-level recurrence trigger names the conditions for promoting the project-scoped §8 to suite-level `prompts/implementation.md`. No suite-level primer edit applied in this session — the gap is registered as Deferred pending recurrence evidence per the recommended option above.

If `issue-tracker-cli`'s CLOSURE-PROTOCOL.md is later suite-promoted under its §7 mechanism, §8 travels with it as part of the bundle and this gap closes via option (b) by default. The current Deferred status is compatible with that flow.
