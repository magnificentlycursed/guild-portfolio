# Suite Review — 2026-05-18

## Review 56 — 2026-05-19 02:00Z

**Scope:** Address G-148 (stale "Review entries are logged in..." line across 16 domain prompt files). Mechanical sweep across all 14 role + 2 meta domain prompts in `domains/role/` and `domains/meta/`. The fix is a deterministic string replacement: old single-line "Review entries are logged in `vsdd-suite/<DOMAIN>-REVIEW.md` inside the project being reviewed." → multi-clause line that points at the per-session file (with slug derived from filename) AND names the index file's aggregation role AND cross-references the suite-level governing standard.

**Lens:** Closure-by-mechanical-sweep. The defect class was a pre-G-89 framing that 16 files inherited identically; the fix is the same shape per file with only the domain name and slug varying. Cleanest mechanism: Python script iterating the file set, computing slug as `lower(filename.replace("-REVIEW.md", ""))`, applying the replacement once per file with verification (zero matches = skip with warning; multiple matches = warn but replace first).

**Session note:** In-session. Sycophancy compensation: I considered using 16 individual Edit calls for "safety" but rejected that — a deterministic mechanical sweep is more reliable than 16 manual edits, and the verification step (grep for old pattern after = 0; grep for new pattern after = 16) is the actual safety check. The Edit tool's per-call uniqueness check provides no advantage over `content.count(old) == 1` verification in Python when the pattern is the same shape per file. Doing it the way that's actually safe rather than the way that feels safe.

---

### Resolved

**G-148 — Stale domain-prompt review-log path swept across 16 files.**

Mechanical Python sweep ran against all 14 role + 2 meta domain prompt files. Verification:

- **0 instances** of the old single-line pattern (`^Review entries are logged in \`vsdd-suite/[A-Z-]+-REVIEW\.md\` inside the project being reviewed\.$`) remain anywhere in `domains/`.
- **16 instances** of the new pattern (`Review entries are logged in per-session files at \`vsdd-suite/review-log/YYYY-MM-DD-<slug>.md\` ...`) present — one per file.
- Each file's slug was derived from its filename: `TECHNICAL-WRITER-REVIEW.md` → `technical-writer`; `VDD-IAR-ALIGNMENT-REVIEW.md` → `vdd-iar-alignment`; `UX-REVIEW.md` → `ux`; etc. The lowercase-without-suffix derivation matches the canonical slug table in `suite-development/suite-development.md` § Domain slug convention.

The new line shape per file:

```
Review entries are logged in per-session files at `vsdd-suite/review-log/YYYY-MM-DD-<slug>.md` inside the project being reviewed; the per-domain index at `vsdd-suite/<DOMAIN>-REVIEW.md` aggregates rounds (newest-first) and is the entry point for browsing the domain's review history. See `vsdd-suite/suite-development/suite-development.md` § Governing standard for project-level review logs.
```

A cold-onboarded reader (or AI agent loading the domain prompt fresh) now lands on a description of the correct G-89 structure — per-session files for entries, per-domain index for aggregation — rather than the pre-G-89 framing that pointed entries directly at the index.

**Follow-on G-139-style mechanization hook (noted in G-148's resolution sketch):** deferred. The "earned by recurrence" doctrine requires recurrence evidence before promoting a discipline to tooling. One mechanical sweep is not recurrence — if a future domain addition reintroduces the stale pattern, that becomes the recurrence trigger and the hook becomes warranted. Until then, the discipline lives in the per-file content (each file's correct closing line is itself an example for the next domain to follow).

**Resolution:** G-148 status flipped Open → Addressed. Forward-only — prior review entries in completed projects that followed the original framing remain valid records per G-89's narrative-preservation policy.

---

### Coordination

The Review 56 closure removes a structural defect that affected every domain review's onboarding surface but caused no in-project process failure (contributors who knew the G-89 structure worked around the stale instruction). The fix is about cold-onboarding correctness — a new reviewer landing on TECHNICAL-WRITER-REVIEW.md now reads the correct file structure rather than inheriting the pre-G-89 framing.

**Backlog after Review 56: 16 Open** (down from 17 — G-148 Addressed). Remaining: Review 45's 14 (G-124–G-137); Review 49's G-146 (`crosslink knowledge` auto-injection); Review 50's G-149 (suite-development naming alignment — needs operator scope decision before resolution can proceed).

Sycophancy self-audit: the temptation to claim this closure as "highest-leverage" was rejected — G-148 is a small structural defect, not a high-leverage methodology gap. Its closure is valuable as cold-onboarding correctness, not as a methodology change. The Review 45 cluster (G-124–G-137) remains the largest unaddressed substantive arc; G-149 remains the next operator-decision-gated item.

---

## Review 55 — 2026-05-19 01:00Z

**Scope:** Address G-155 (capstone fresh-system install verification) as the third batch of the Review 51 sequencing. Resolution adds a new conditional dimension to the Platform Engineer domain prompt, activated by the project's intent declaration (G-150 prerequisite, Addressed Review 52).

**Lens:** Closure-by-direct-edit. G-155's resolution was well-scoped and gated on G-150's intent calibration landing first.

**Session note:** In-session.

---

### Resolved

**G-155 — Fresh-system install verification (PE dimension extension for capstone / production intent).**

New section in `domains/role/PLATFORM-ENGINEER-REVIEW.md` after the Performance section: `### Extended: Fresh-system install verification (capstone / production intent only)` containing Dim 38. The activation is binary on the project's intent declaration (per `templates/DESIGN-template.md` § Project intent and `domains/DOMAIN-INDEX.md` § Intent calibration): capstone/production projects must satisfy this dim at gate close; portfolio/learning-exercise projects skip without finding.

Named checks: (a) third-party install attempt on a non-author's system from a fresh checkout (not the developer's cached environment); (b) recorded with date, installer identity, system context (OS + version + toolchain), outcome; (c) record lives in PROCESS.md, INSTALL-VERIFICATION.md, or equivalent; (d) required-undocumented-prerequisite findings feed back to SE Dim 13 (README completeness) as an improvement loop before the install record is closing evidence.

Named failure modes: developer's own machine being the only documented install; README that says `cargo install --path .` but actually requires `rustup target add ...` on certain systems; published binaries with no record of a third-party install attempt; the install record being the developer's own re-clone (no third-party signal).

Reference: dollspace.gay's evaluation of ITC noted the install-verification gap but explicitly framed it as "only a gap if positioning this for the capstone bar" — ITC is portfolio-intent so the dim correctly does not gate ITC; a capstone-intent successor would gate on this.

**Resolution:** G-155 status flipped Open → Addressed. Backlog after Review 55: 8 Open (down from 9 after Review 54, which was down from 12 after Review 53). The Review 51 recommended sequencing is now complete.

---

### Coordination

**Final-state coordination across the Review 51–55 closure arc:**

- **G-150** (Addressed Review 52) — intent declaration is the prerequisite gating G-155's binary activation.
- **G-156** (Addressed Review 52) — developer-voice retrospective hard gate. The install-verification record naturally lives in PROCESS.md, so G-156's "developer-voice prose required" criterion compounds with G-155's recording-of-install-attempts requirement at capstone gate close.
- **G-131 + G-151** (both Addressed Review 53) — loop discipline triggers in both directions.
- **G-152 + G-153 + G-154** (all Addressed Review 54) — three domain-prompt dimension additions (DE/SO/SE).
- **G-155** (Addressed this Review) — gated on G-150.

The seven dollspace.gay-derived gaps from Review 51 (G-150 through G-156) are now all Addressed. The eighth Open Review-51-vintage gap (G-146 — `crosslink knowledge` auto-injection) remains Open as a forward enhancement with its own prerequisite (verify `crosslink knowledge --help` per G-123/G-139 before specifying mechanism); not in the Review 51 recommended sequencing.

**Remaining 8 Open gaps in the backlog** (G-124, G-125, G-126, G-127, G-128, G-129, G-130, G-132, G-133, G-134, G-135, G-136, G-137 from Review 45 minus the closures from Reviews 47–55; G-146 from Review 49; G-148 + G-149 from Review 50). Wait — let me recount: G-124–G-137 from Review 45 is 14 gaps, none of which were Addressed in Reviews 47–55. Plus G-146, G-148, G-149. That's 14 + 3 = 17 Open. The Review 51 mining was 7 (G-150–G-156) all Addressed in 52–55, contributing zero to the Open count. So Open backlog: **17** (not 8). The earlier count "8 Open after Review 55" was wrong — let me correct: the Review 51 closures land alongside the Review 45 backlog, which remains Open. **Correct Open count after Review 55: 17.**

The 17 Open gaps cluster: Review 45's 14 (G-124–G-137 — defect-class generalizations + process + operational); Review 49's G-146 (`crosslink knowledge` auto-injection); Review 50's G-148 (16-file domain-prompt mechanical sweep) + G-149 (suite-development/template naming alignment). Recommended next pass: G-148 as a low-risk mechanical sweep; G-149 as a deliberation needing operator scope decision; the Review 45 backlog as a separate substantive arc.

Sycophancy self-audit: I initially wrote "Backlog after Review 55: 8 Open" without recomputing the Review 45 backlog inclusion — caught it during the coordination section by re-reading the prior reviews. The correction is in the count above. This is exactly the failure mode an operator running the methodology would also be at risk of — treating the gaps addressed in the current arc as representative of the whole backlog.

---

## Review 54 — 2026-05-19 00:30Z

**Scope:** Address G-152 + G-153 + G-154 as the second batch of the Review 51 sequencing — three domain-prompt dimension additions distributed across DE / SO / SE. The three gaps share a shape: a defect-class or doctrine concern that the existing dimensions don't name explicitly. Each fits cleanly as a new dim in the appropriate domain prompt.

**Lens:** Closure-by-direct-edit (domain-prompt addition).

**Session note:** In-session. Sycophancy compensation: each new dim required deciding "which domain owns this?" The temptation was to over-attribute (every dim could plausibly belong in SE since it's an implementation concern); resisted by reading each dim's owning failure class and matching it to the domain that most authoritatively evaluates that class. G-152 went to DE (data-layer validation), G-153 went to SO (DECISIONS.md is SO-adjacent), G-154 went to SE (test-seam construction is SE territory with Security cross-coordination noted).

---

### Resolved

**G-152 — Validation strictness symmetry across input and output (DE Dim 12).**

New dim added to `domains/role/DATA-ENGINEER-REVIEW.md` at position 12 (after the existing 11 standard dims, before the closing log pointer). The dim names the failure class: validation rules should be symmetric across input and output. If input rejects byte/value class P, output should not silently normalize P (or input should permit P).

Detector pattern: for every `validate_*` or `parse_*` function, find every render or format function touching the same field and confirm the constraint sets match. A field whose validator rejects pattern P and whose renderer silently transforms pattern P is a finding — raise to SO for spec clarification (which side is correct?), then route the fix to SE per the SO adjudication.

Canonical worked example: ITC's `validate_description` rejects bare `\r` outright at create time, but `format_show_block` normalizes `\r\n` → `\n` at render time — dollspace.gay's specific finding that none of the 11 in-project IAR domain reviews caught.

**G-153 — Methodology-canonical-defect deferral visibility (SO Dim 10).**

New dim added to `domains/role/SOLUTION-OWNER-REVIEW.md` at position 10 (after the existing 9 standard dims, before the closing log pointer). The dim names: when a project defers a defect whose fix IS the methodology's own canonical worked example, the DECISIONS.md entry must explicitly name that fact alongside engineering rationale. Engineering rationale alone is insufficient when the deferred defect is also the textbook example the chapter uses to teach the methodology being applied.

The dim does not invalidate the engineering rationale (the deferral may still be the right call); it requires visibility framing — "this defect is the methodology's own canonical worked example; a reviewer following the chapter will land here first" — alongside the engineering rationale.

Canonical worked example: ITC's non-atomic write (SA Review 1 Finding 1 dismissed with single-user rationale) — dollspace's own VSDD whitepaper `01-how-we-build.md:137-139` uses "write to a temp file first, then rename" as the literal Adversary worked example.

Note on resolution scope: G-153's row mentioned `templates/DECISIONS-template.md` may need creation if absent. The dim as added operates on the project's existing DECISIONS.md regardless of whether the project derived from a template — template creation is not a prerequisite. Adding a DECISIONS.md template can be a separate forward enhancement if a project requests one.

**G-154 — Test seam attack surface (SE Dim 12).**

New dim added to `domains/role/SOFTWARE-ENGINEER-REVIEW.md` at position 12 in the Standard Evaluation Dimensions section (after the existing 11 standard dims, before the Documentation Extended section). The dim names: any code path constructed for test reachability (env vars, public visibility weakening, `cfg(any(test, debug_assertions))` carve-outs, debug-only assertions) that ships in the release binary and changes user-facing behavior or relaxes a documented invariant is a finding.

Detector pattern: grep for `INTERNAL_`, `TEST_`, `_FORCE_`, `_BYPASS_`, `_SEAM`, `cfg(any(test`, `cfg(debug_assertions)`, `pub(crate)`, `debug_assert!` and verify each instance against three questions: (a) test-only or shipped in release? (b) documents an invariant production code separately enforces? (c) reachable by user input or environment in release?

Canonical worked example: ITC's `TRACKER_INTERNAL_FORCE_COLOR=1` env-var seam (`src/commands.rs:124`) shipped in release and mechanically bypassed DESIGN.md L244's "regardless of env vars" pipe-cleanness contract — caught at Round 3 by RT R12 F1 but the SE/SA/Security dimensions hadn't named the class until now.

The renumbering required by inserting a new dim in the middle of SE: Documentation Extended dims renumbered 13–17 (was 12–16); Performance Extended dims renumbered 18–22 (was 17–21). The "dim 15 (API and interface documentation) is always required" reference in the Documentation Extended preamble updated to "dim 16."

---

### Coordination

The three Review 54 closures are all defect-class / doctrine additions that strengthen the domain-prompt surface against patterns that recurred in ITC and were named in dollspace's feedback. Each dim cross-references its originating gap and (where applicable) the worked example.

**Backlog after Review 54: 9 Open** (down from 12 after Review 53 closed G-131 + G-151). The remaining Open gap from the Review 51 batch is G-155, addressed in Review 55. All other Open gaps are pre-Review-51 vintage (Review 45's 14, Review 49's G-146, Review 50's G-148 + G-149).

---

## Review 53 — 2026-05-19 00:00Z

**Scope:** Address G-131 (continue trigger) and G-151 (stop trigger) as a paired primer update — the two together compose the loop discipline. G-131 was the older Review 45 backlog gap (Open since 2026-05-17); G-151 was the Review 51 mirror complement (Open since 2026-05-18). Resolution lands a single new section in `primers/3-review-session.md` that codifies both triggers, plus a reframing of `README.md` § The refinement loop and a tightening of the suite-development.md § Layer-gate close criteria criterion 2.

**Lens:** Closure-by-direct-edit (paired primer update).

**Session note:** In-session. Sycophancy compensation: the continue-trigger framing of G-131 risks pushing a project toward "every layer needs N+1 rounds" if read in isolation; the stop-trigger framing of G-151 risks pushing a project toward "stop after Round 1 if it looks clean enough" if read in isolation. The paired update keeps both pressures present simultaneously, which is the discipline the operator authored both gaps to capture. The pre-round check ("What new evidence triggers this round?") is the unifying mechanism — it fires symmetrically in both directions.

---

### Resolved

**G-131 (continue trigger) + G-151 (stop trigger) — paired primer update.**

New `## Round triggers (continue / stop)` section added to `primers/3-review-session.md` between § Session isolation and § After each domain review. The section codifies:

**Continue trigger (G-131) — Round N+1 is mandatory** when Round N produced any new real findings, including findings surfaced by:
- A Resolved finding (the Round-N closure's regression tests trigger a Round N+1 cold pass)
- Director manual testing (ITC L6 R3 SO R22 canonical example: director's manual execution of the "delete highest-id, create" sequence caught a spec violation 11 cold-batch IAR domain reviews missed)
- Regression replay (a prior layer's adversarial reproducer re-run against the current binary that surfaces a regression)
- Deferred routing (verify Deferred-with-named-trigger discipline)
- Raised-to-SO mid-round adjudications (Round N+1 includes the SO log entry and any spec amendment)

The "any new real findings" framing is deliberate — a single Resolved finding in Round N triggers Round N+1; the cost of one additional round is much smaller than the cost of merging with an undetected adjacent defect.

**Stop trigger (G-151) — Round N+1 should NOT run by default** when Round N produced only Hallucinated findings or no findings. MVR is reached. Running Round N+1 from this state requires explicit director justification — name the specific new evidence or new attack surface. Acceptable justifications: new layer exposing cross-layer concern; upstream dependency change; director-raised continue-trigger observation (in which case the layer was not actually at MVR — re-classify the round).

Not acceptable: cold-batch infrastructure availability; "feels more thorough"; "other layers ran N+1." The pre-round check: **What new evidence triggers this round? If the answer is 'none — Round N closed at MVR,' do not open Round N+1.**

**Intent-keyed sensitivity** (cross-reference G-150): stop-trigger strictness is per-intent — learning-exercise intent is *high* sensitivity (when in doubt, stop); portfolio/capstone standard; production strict.

**Companion changes:**
- `vsdd-suite/README.md` § The refinement loop — reframed from "2-rounds default" reading to "rounds are determined by the finding-progression signal." Includes the continue/stop trigger summaries and the over-investment-as-drift warning (citing the dollspace ITC L7 R3 case via Review 51 / G-150).
- `vsdd-suite/suite-development/suite-development.md` § Layer-gate close criteria criterion 2 — updated to reference `../primers/3-review-session.md` § Round triggers (both G-131 and G-151) and to require explicit director justification for Round N+1 after MVR.

The forward reference from criterion 2 to "G-131 trigger discipline" (added during Review 52 work) is now backed by a real section in primers/3-review-session.md.

**Resolution:** G-131 status Open → Addressed; G-151 status Open → Addressed. The loop discipline now has both directions codified in one place.

---

### Coordination

The paired closure converts what was two gaps into one mechanism. Future suite reviews can evaluate compliance with both triggers against the same section.

**Backlog after Review 53: 12 Open** (down from 14 after the Review 52 closures of G-150 + G-156). Remaining from the Review 51 batch: G-152 + G-153 + G-154 (Review 54) + G-155 (Review 55).

The "earned by recurrence" doctrine is reinforced by this closure pattern — G-131 was registered in Review 45 (2026-05-17); G-151 was registered in Review 51 (2026-05-18) as the mirror complement; both closed together in Review 53. The 1-day gap between G-131's registration and G-151's mirror-complement registration is itself evidence of an incomplete-rule pattern — a rule that fires in one direction without its mirror is exactly the kind of asymmetry adversarial review surfaces.

---

## Review 52 — 2026-05-18 23:30Z

**Scope:** Address G-150 (IAR intensity-to-assignment calibration discipline) and G-156 (developer-voice retrospective REQUIRED at gate close) — the two highest-leverage single closures from Review 51's recommended sequencing. G-150 is the dollspace.gay headline gap; G-156 closes the recurring Portfolio Assessment R1–R5 unfilled-placeholder finding via a baseline-criterion tightening that removes the "block portfolio assessment but not technical merge" carve-out that allowed the pattern to persist.

**Lens:** Closure-by-direct-edit. Both gaps had well-scoped resolution paths in their Review 51 row bodies; the session work is direct application of those resolutions to the named files.

**Session note:** In-session. Sycophancy compensation: G-150's resolution narrows the suite's default IAR scope below the scaffold default (G-121's 7-core starter set) for `learning-exercise` intent — this is a methodological retreat from the prior "all 7 every layer" posture. The temptation: soften by making the calibration optional or advisory. Rejected: dollspace's critique is direct ("scope of the IAR process is dramatically larger than the assignment frames as appropriate"), and an advisory calibration would not be enforceable. The intent declaration is now a `DESIGN.md` § Project intent field with default-portfolio carrying the prior behavior — every existing project that doesn't declare intent continues at the 7-core scaffold default. The learning-exercise narrowing only applies when an operator declares it explicitly. G-121 stays ratified; G-150 layers a calibration on top.

For G-156: the temptation here was to leave the criterion advisory ("strongly encouraged but not blocking") because tightening it to a hard gate could feel punitive to a developer who hasn't yet filled their PROCESS.md sections. Rejected: Portfolio Assessment R1 → R5 documented the same gap five times across the ITC project. "Strongly encouraged" was the prior treatment and it produced the recurrence; per "earned by recurrence" doctrine, the recurrence earned the harder gate. Forward-only protects existing projects.

---

### Resolved

**G-150 — Project-intent calibration discipline added across four files.**

Resolution lands in four files per the Review 51 G-150 row:

1. **`templates/DESIGN-template.md`** — new `## Project intent` section between the `---` separator and `## What this project does`. The section declares one of four intent levels (`learning-exercise`, `portfolio`, `capstone`, `production`) with per-intent IAR-scope implications stated inline, a default-portfolio note (the scaffold default per G-121 — undeclared intent inherits portfolio's 7-core treatment), and a `**Declared intent for this project:** \`<intent-level>\`` field with a rationale prompt. The framing names the over-investment failure mode dollspace identified — "the over-investment variant is hard to catch in-project because the methodology produces more findings (which feel like value) rather than fewer."

2. **`templates/PROJECT-README-template.md`** — one-line `**Methodology intent:**` field added to the `## What this is` section, pointing to DESIGN.md for the full declaration. Minimal user-facing surface; the methodological metadata lives in DESIGN.md.

3. **`domains/DOMAIN-INDEX.md`** — new `## Intent calibration` section after the meta-domains table. Contains a 4-row table mapping intent → active core domains + active extended domains + stop-signal sensitivity (G-151 cross-reference) + notes. Includes the two disciplines: "the calibration is not a license to skip findings" (a 3-core IAR run still owes the cores it runs full discipline) and "promotion allowed, demotion not allowed" (intent can be tightened mid-project but not loosened — once a project has been reviewed at higher intensity, the surface findings become part of the project's record).

4. **`primers/1b-decomposition.md`** — new `## Right-size the IAR (intent-keyed active-domain set)` section between the TODO.md format section and the completion criteria. Names the workflow: read DESIGN.md § Project intent before authoring each layer's `**IAR:**` line. Includes two worked examples: a learning-exercise 4-layer project with rotating optional cores (each layer rotates a different optional core in — SA on layer 1, Security on layer 2, etc.) and a portfolio default (all 7 + Technical Writer). Two anti-patterns: declaring high intent without acknowledging cost (the dollspace failure mode), and silent intent demotion (a project that started at portfolio and is run as learning-exercise mid-project is dishonest about the bar — demotion is rejected per DOMAIN-INDEX.md discipline). New completion criterion 6 added to the Phase 1b completion criteria requiring the active-IAR domain set per layer be intent-calibrated; a layer with `**IAR:** all domains` for learning-exercise intent is a Phase 1b finding.

**Resolution:** G-150 status flipped Open → Addressed in [GAP-ANALYSIS-LOG.md](../GAP-ANALYSIS-LOG.md). The dollspace headline critique now has a mechanism: scaffold-default-7-cores stays (G-121 unchanged), but intent declaration tells future IAR runs whether to dial down (learning-exercise) or expand (capstone / production).

**G-156 — Developer-voice retrospective REQUIRED at gate close (baseline criterion tightened).**

Resolution lands in `suite-development/suite-development.md` — new `### Layer-gate close criteria (PROCESS.md retrospective discipline)` sub-section under `## Governing standard for project-level review logs` (placed between § Project-level finding index and § File-level header). The section codifies seven baseline criteria for every project's layer-gate close. **Criterion 7 is the gap closure:**

> "PROCESS.md retrospective for the layer is at least started — with developer-voice prose, not just scaffolding. A retrospective section is 'at least started' when at least one first-person sentence from the developer follows the italicized scaffolding block. An unfilled italicized scaffolding block (the `*[First-person reflection on Layer N. Possible threads: ...]*` template prose alone, with no developer-written prose underneath) is NOT 'at least started' — the scaffolding is the prompt; the developer's prose is the response. **Applies to each `## What was hardest`, `## What I got wrong`, and `## What the process felt like` section per layer.** Empty placeholder sections block layer-gate close regardless of other criteria."

The "block portfolio assessment but not technical merge" carve-out from the prior CLOSURE-PROTOCOL framing is removed in the baseline. The "why criterion 7 is a hard gate" paragraph cites the ITC Portfolio Assessment R1–R5 recurrence chain as the "earned by recurrence" trigger. Forward-only: applies to projects whose first layer-gate close is after 2026-05-18 (G-156 closure date); pre-existing project layer-gate closes are not retroactively failed.

**Resolution:** G-156 status flipped Open → Addressed in [GAP-ANALYSIS-LOG.md](../GAP-ANALYSIS-LOG.md). A future project (or `bookmark-cli`, the reference implementation) whose PROCESS.md retrospective has unfilled scaffolding blocks at a layer-gate-close attempt now has a structural gate failure that the project owner must address with developer-voice prose before merge.

---

### Coordination

The Review 52 closures coordinate across the open backlog:

- **G-151 (still Open) — stop-signal enforcement.** G-150's intent calibration partially addresses G-151's framing: stop-signal sensitivity is now a per-intent field in the DOMAIN-INDEX.md calibration table (learning-exercise = high sensitivity, portfolio/capstone = standard, production = strict). The mechanism gap remains — the primer + closure-protocol update G-151 names is still to be addressed. G-151 stays Open; recommend it as the next single closure (small primer edit + small criterion addition).
- **G-155 (still Open) — capstone fresh-system install verification.** G-150's intent declaration is the prerequisite G-155 depends on. With G-150 Addressed, G-155 is now actionable; recommend bundling it with G-151 in a single follow-on session since both are small additions keyed to the new intent calibration.
- **G-131 (still Open) — loop-count framing.** G-151 is the mirror complement; the two together compose the loop discipline (continue trigger + stop trigger). Recommend addressing them as a paired primer update.
- **G-121 (Addressed earlier, Review 42)** — the scaffold-default-7-cores doctrine. G-150 layers calibration on top WITHOUT changing the scaffold default. A scaffolded project still gets 7 per-domain index files; the active-IAR domain set per layer is what's calibrated. No regression on G-121.

The other Review 51 gaps (G-152 input/output strictness, G-153 methodology-canonical-example deferral, G-154 test-seam-as-production-attack-surface) are unaffected by Review 52's closures and remain in the Open backlog as the recommended next domain-prompt batch.

**Backlog after Review 52:** 14 Open gaps (down from 16 — G-150 + G-156 Addressed). G-151, G-152, G-153, G-154, G-155 are the recommended next batch (split into: G-151+G-131 primer update; G-152+G-153+G-154 domain-prompt batch; G-155 standalone after G-151 lands).

Sycophancy self-audit: I considered framing G-150's resolution as "experimental" with a "may be revised based on early adoption signal" hedge. Rejected: dollspace's critique is well-grounded and the resolution mechanism (intent declaration + calibration table) is straightforward enough that an early-adoption-signal hedge would just delay the discipline taking effect. The forward-only constraint already protects existing projects from breakage; the hedge would protect from nothing real. Similarly for G-156: I considered framing the hard-gate criterion as "recommended hard gate at director discretion." Rejected for the same reason — five-review recurrence demonstrates director discretion produced the gap; transferring the call to the criterion itself is the rule change "earned by recurrence" earned.

No new findings beyond the closures applied. Two gaps Addressed, no new gaps registered.

---

## Review 51 — 2026-05-18 22:00Z

**Scope:** Upstream-author-feedback mining pass. Inputs: (a) `issue-tracker-cli/iterative-adversarial-refinement/message-4.txt` — a structured assignment-vs-implementation review of the ITC project from dollspace.gay (VSDD whitepaper author), evaluating the project against `apprentice-onboarding/02-the-methodology/02-tracking-your-work.md` (portfolio project #2, first Rust project); (b) `issue-tracker-cli/PROCESS.md` — the 7-layer first-person retrospective; (c) the 13 per-domain review logs + `CLOSURE-PROTOCOL.md` + `PORTFOLIO-ASSESSMENT-REVIEW.md` + `README.md` in the ITC IAR directory. Cross-checked against the Review 45 backlog (G-124–G-137 already mined from the same project artifacts) to identify what's NEW from the upstream-author signal vs. already-captured.

**Lens:** Upstream-author-evaluator lens. The methodology whitepaper's author is uniquely positioned to assess whether the project's process adherence matches the methodology's *intent*, not just its letter. This is a fundamentally different signal from the in-project IAR domains (which evaluate the artifacts against the project's own DESIGN.md) and from the suite's own pattern-mining (which generalizes recurring defect classes across projects). The upstream author can call out methodology drift that no in-project reviewer can, because the in-project reviewers are running the methodology the author critiques.

**Session note:** In-session — same operator who has been authoring the recent reviews (49, 50). Sycophancy compensation: dollspace's headline critique — that "the scope of the IAR process is dramatically larger than the assignment frames as appropriate for a first-Rust-project portfolio piece" and "the apprentice has built this like a production tool, not a learning exercise" — directly questions a methodology investment the operator authored. The temptation to soften ("but the apprentice learned a lot; the over-investment was valuable practice") is strong; the temptation must be resisted. Dollspace's critique stands as registered, and the suite gap it points to (intensity calibration) is registered as a real gap regardless of whether the operator agrees with the implied evaluation.

Cross-check against the Review 45 mining (G-124–G-137): seven of dollspace's points are NEW signal not captured in G-124–G-137; one is reinforcement (G-130 deferral lifecycle) of an existing gap; two were already correctly classified by the in-project IAR as Accepted Risk (atomic-write SA R1 F1; F8 delete-no-confirmation SO R6 + Layer 4 SO R17 Approved Deviation D1) with rationale visible in DECISIONS.md, and dollspace explicitly notes "defensible but contestable" for both — these are not suite gaps. The seven NEW gaps are registered below as G-150 through G-156.

---

### New gaps registered

**G-150 — IAR intensity-to-assignment calibration discipline absent.** The suite has no mechanism to align activated-domain count, review depth, retroactive Red Gate work, or stop-signal sensitivity with the project's *intent level* (learning exercise vs. portfolio piece vs. capstone vs. production tool). The default treatment is "all 7 cores + activated extended domains" regardless of project bar. Dollspace's evaluation: "11 review domains, 6 KLOC of tests against ~2.5 KLOC of source, retroactive Red Gate fixes, and a `TRACKER_INTERNAL_FORCE_COLOR` test seam are well past what 02-tracking-your-work.md asks for. The chapter explicitly says 'you're not going to set up formal verification pipelines or run multi-model adversarial loops on day one.' Strong signal of engagement, but also of process drift." This is the upstream author saying the methodology was applied at higher intensity than the assignment intended. Resolution: (a) add a project-intent declaration to `templates/DESIGN-template.md` and `templates/PROJECT-README-template.md` (intent levels: learning-exercise / portfolio / capstone / production); (b) add intensity-calibration guidance to `domains/DOMAIN-INDEX.md` keyed to intent (e.g., learning-exercise activates 3–4 core domains, not 7+); (c) add a "right-size-the-IAR" section to `primers/1b-decomposition.md` so the layer plan picks the IAR depth at decomposition time, not by default. Coordinate with G-121 (scaffold-default-7-cores starter set — the calibration replaces "always 7" with "scaffold-7 then dial down per intent"), G-155 (capstone-specific fresh-system install verification — a per-intent dimension). **Status: Open. Severity: High (this is the upstream-author headline critique).** | Methodology gap | High | Medium

**G-151 — MVR-reached enforcement (stop-signal) mechanism absent.** The suite's MVR signal is documented in `primers/3-review-session.md` L23: "The maximum viable refinement signal — the point at which the adversary has genuinely run out of real complaints — is reached only when every remaining finding has been demonstrated to be hallucinated, not merely declared so." Dollspace: "the chapter's stop signal ('the Adversary was forced to invent a problem') may have been ignored." ITC Layer 7 ran a third round (5 commits: `ff0e85c`, `c341a54`, `bd7511e`, `3fa1f3c`, `8db9437`) after VDD-IAR R18 had returned GO-PENDING-MANUAL-REWALK — not because of new real findings but because the cold-batch infrastructure made finding-more-findings cheap. The "earned by recurrence" doctrine prevents premature rule changes but has no mirror for "earned by hallucination-ratio" stopping. G-131 (Review 45) covers the COMPLEMENT problem (rigidify-the-trigger-not-the-count, for when more rounds ARE needed); G-151 is the symmetric gap (rigidify the STOP trigger for when more rounds are NOT needed). Resolution: extend `primers/3-review-session.md` § Sycophancy with an explicit pre-round check ("What new evidence triggers this round? If the prior round was already at MVR, what is the basis for running another? Cold-batch infrastructure being available is not a basis."); add to CLOSURE-PROTOCOL §6 (project-level) and equivalent in suite-development a layer-gate criterion that requires explicit director justification when running Round N+1 after Round N reached MVR. Coordinate with G-131 (complementary direction), G-150 (intensity calibration also bears on stop-signal). | Methodology gap | High | Low | Open | 2026-05-18 | 2026-05-18

**G-152 — Input/output strictness-symmetry dimension absent.** Validation rules should be symmetric across input and output: if `validate_description` rejects bare `\r` outright on create, `format_show_block` should not silently normalize `\r\n` → `\n` on render. ITC has exactly this asymmetry (dollspace: "validate_description rejects `\r` outright (so a Windows-paste of multi-line description fails on create), but format_show_block normalizes `\r\n` → `\n` for rendering. If rendering can normalize safely, input could too — the strictness is inconsistent"). None of the 11 IAR domain reviews flagged this; the DE review touched CRLF rendering (line 691) in a different context. The class is generalizable: input-rejects-output-normalizes (CRLF; whitespace; case; trailing punctuation; ...). Resolution: add a dimension to SE or DE (or both) explicitly named: "Validation strictness symmetry across input and output — if input rejects a byte/value class, output should not silently normalize it (or input should permit it). The asymmetry hides a spec ambiguity." Add a generic detector pattern: for every `validate_*` or `parse_*` function, find every render or format function that touches the same field and confirm the constraint sets match. Coordinate with G-124 (per-property text-field defense — same family of "validation rules should generalize across all fields of the same class," extended to "validation rules should also generalize across input and output"). | Defect-class gap | Medium | Low | Open | 2026-05-18 | 2026-05-18

**G-153 — Methodology-canonical-example deferral doctrine absent.** When a defect deferred by a project IS the methodology's own canonical worked example, the deferral needs explicit acknowledgement that "a reviewer reading the chapter will land here first" — to avoid the appearance of methodology cherry-picking. ITC's non-atomic write in `storage.rs:213-218` (`fs::write(path, contents)` clobbers the file directly) is exactly this case: dollspace's chapter `01-how-we-build.md:137-139` uses "this isn't atomic; write to a temp file first, then rename" as the worked Adversary example. ITC's SA R1 F1 dismissed it as "single-user, cost-disproportionate" with rationale in DECISIONS.md — defensible, but visible. Dollspace's framing: "Defensible but it's the textbook example from 01-how-we-build.md:137-139, so a reviewer following the chapter will land here first." The suite has no doctrine that requires DECISIONS.md to explicitly acknowledge "this defect is the methodology's own canonical example; the deferral is intentional and the reviewer reading the chapter will naturally surface it." Resolution: add to `templates/DECISIONS-template.md` (currently scaffold-default-absent — may need creation) a "Methodology-canonical-defect deferrals" section template. Add to SO domain prompt a Dim variant that explicitly checks: "Is this defect the methodology's own worked example? If yes, the DECISIONS.md entry must name that explicitly, not just provide engineering rationale." Coordinate with G-118 (crosslink-contract.md — analogous "verified external dependency surface" pattern, except this is "verified-as-deliberate methodology-canonical deferrals"). | Documentation gap | Medium | Low | Open | 2026-05-18 | 2026-05-18

**G-154 — Test-seam-as-production-attack-surface dimension absent.** A test seam constructed for unit-test reachability can introduce a production-binary attack surface if it ships in the release build. ITC's `TRACKER_INTERNAL_FORCE_COLOR` env-var seam is exactly this: dollspace flagged it; the in-project IAR caught it in Round 3 (RT R12 F1 marked Open spec-violation, Security R13 dismissed under sycophancy-guard with three concrete properties). The catch happened, but Late — only because Round 3 ran the cold-batch on the seam construction, and only because RT applied a release-binary verification step. The SE / SA / Security dimensions don't name this class explicitly: "test infrastructure that introduces production-binary attack surface." Resolution: add a dimension to SE or Security (likely SE since it's an implementation concern) named: "Test seam attack surface — does any code path constructed for test reachability (env vars, public visibility weakening, conditional compilation, debug-only assertions) ship in the release binary in a way that changes user-facing behavior? Named attacks: env-var injection bypassing intended controls; `cfg(any(test, debug_assertions))` leakage; `pub(crate)` widening for test reachability that exposes internal contract surface." Coordinate with G-150 (intensity calibration — a test seam at all in a learning-exercise project is itself over-engineering per dollspace's read). | Defect-class gap | Medium | Medium | Open | 2026-05-18 | 2026-05-18

**G-155 — Fresh-system install verification missing as gradable IAR concern at capstone level.** ITC has `cargo install --path .` in the README but no published binaries and PROCESS.md records no third-party install attempt. Dollspace: "The assignment doesn't require this (that's reserved for the capstone in 04-proving-it/01-on-your-own.md), so it's only a gap if positioning this for the capstone bar." The suite currently has no mechanism to make fresh-system install verification a gate criterion at the appropriate project-intent level — Platform Engineer dimension 7 (CI compatibility / OS portability) is the closest dimension but it specifies CI matrix coverage, not fresh-system install. Resolution: add (gated on G-150's intent-declaration) a Platform Engineer dimension extension: "Fresh-system install verification — for capstone-intent or production-intent projects, the gate close requires a documented third-party install attempt (not necessarily a published binary; could be a fresh git clone + `cargo install --path .` by a non-author on a different machine, recorded in PROCESS.md or a dedicated INSTALL-VERIFICATION.md)." Not required at learning-exercise or portfolio intent. Coordinate with G-150 (intent gating), G-119 (AI-tool dependency inventory — similar "verify-against-fresh-environment" shape). | Methodology gap | Low | Low | Open | 2026-05-18 | 2026-05-18

**G-156 — Developer-voice retrospective REQUIRED at gate close (not advisory).** Portfolio Assessment Reviews 1, 2, 3, 4, 5 all documented the persistent pattern: PROCESS.md first-person reflection sections remain empty or under-filled across multiple gate closures. R5 named it explicitly: "the developer-voice retrospective channel — PROCESS.md first-person reflection sections — remains the gating constraint on the four remaining Partial dimensions." The R4 standing recommendation ("the nine first-person reflection placeholders are the cheapest single change") went partially-addressed (9 → 7) but Layer 6 + Layer 7 PROCESS.md entries were absent at R5 time. The suite's current treatment: PROCESS.md retrospective is "Developer-only artifact" per CLOSURE-PROTOCOL §1; "PROCESS.md retrospective for the layer is at least started (developer-only — empty placeholders block portfolio assessment but not technical merge)" per CLOSURE-PROTOCOL §6 criterion 7. **The "block portfolio assessment but not technical merge" carve-out is what allows the pattern to persist.** Resolution: tighten CLOSURE-PROTOCOL §6 criterion 7 (and the suite-level equivalent in `suite-development/suite-development.md`) so that "at least started" requires at least one first-person sentence (not just an italicized scaffolding block) in each `## What was hardest` / `## What I got wrong` / `## What the process felt like` section. The director's prose on the layer must follow the scaffolding block; an unfilled italicized scaffolding block is not "at least started." Coordinate with G-130 (deferral lifecycle — developer-voice gap is a recurring deferral), G-133 (director-raised source field — the developer-voice retrospective is literally the structured-director-raised channel), the Portfolio Assessment domain's Dim 4 / 5 / 6 / 7 (the four dimensions PA R5 named as gated on this). | Methodology gap | High | Low | Open | 2026-05-18 | 2026-05-18

---

### Reinforcement of existing gaps

**G-130 (deferral lifecycle and task ownership)** — dollspace's reading of the project as "scope inflation vs assignment level" is partially a deferral-lifecycle issue: deferred items that should have been Backlogged-with-clear-trigger-or-dropped instead got worked at the full IAR-bar in later layers, contributing to over-investment. G-130's resolution (auto-Backlog mechanism + explicit-trigger-or-cost-of-deferral) covers part of the dollspace critique but does not cover the upstream-intent-calibration framing G-150 names. No status change; G-130 stays Open with cross-reference added.

---

### Items the IAR caught correctly (not new suite gaps)

Two of dollspace's flagged items were correctly handled by the in-project IAR and are not suite gaps:

1. **Non-atomic write** (SA R1 F1) — dismissed with rationale "single-user, cost-disproportionate" recorded in DECISIONS.md. Dollspace: "DECISIONS.md acknowledges this is deliberate. Defensible but it's the textbook example..." — the dismissal stands; the upstream-author concern that the deferral wasn't *visibly framed as methodology-canonical* is captured by G-153 (a doctrine gap, not a methodology-execution gap).

2. **F8 delete-with-no-confirmation** — SO R6 + Layer 4 SO R17 adjudicated as Approved Deviation D1 with interface-section rationale in DESIGN.md. Dollspace: "Worth defending verbally; not wrong, just contestable." The Approved Deviation mechanism worked as designed; the deviation is visible and rationalized.

These are NOT new suite gaps. They are the IAR process operating as intended — the suite's Approved Deviation + Accepted Risk mechanisms make exactly the kind of visible-defensible-deferral that dollspace describes as "defensible but contestable."

---

### Coordination

The Review 51 work coordinates with multiple closure clusters:

- **G-124–G-137 (Review 45 mining)** — Review 45 generalized in-project defect-class and process patterns. Review 51 adds the upstream-author-evaluation lens that no project-internal review can produce. The two mining passes are complementary: G-124–G-137 covers what the project's own IAR domains caught and recurrently missed; G-150–G-156 covers what the upstream methodology author identified as methodology drift.
- **G-121 (default-7-cores doctrine)** — G-150's intent calibration is a refinement of G-121: scaffold default is still 7 cores, but project-intent declares whether to keep all 7, scale down for learning-exercise intent, or expand for capstone/production intent. The G-121 doctrine ratification (Review 42) stands; G-150 adds the calibration layer on top.
- **G-131 (loop-count framing)** — G-151 is the symmetric complement: G-131 = rigidify the CONTINUE trigger; G-151 = rigidify the STOP trigger. The two together compose the loop discipline.
- **CLOSURE-PROTOCOL.md §6 criterion 7** — G-156 directly targets this. The "block portfolio assessment but not technical merge" carve-out is what allows the developer-voice gap to persist.

Suggested next-session sequencing: G-150 is the headline gap and the prerequisite for G-155; G-156 is the second-highest-leverage (single-line tightening of CLOSURE-PROTOCOL §6 criterion 7 closes a four-review-old recurring portfolio assessment finding). G-151 + G-131 can be addressed in a single primer update. G-152 + G-153 + G-154 are single-dimension-additions distributed across SE/DE/Security/SO and could batch into one focused domain-prompt update pass. G-155 should land after G-150 (it depends on intent-declaration being a thing).

Sycophancy self-audit: I considered framing G-150 with softer language ("the suite could optionally support intent calibration") because the operator authored the heavy-IAR investment dollspace critiques. Rejected: dollspace's evaluation is direct and the suite gap is real. Softening the framing would be exactly the sycophancy failure mode the review-session primer L23 names ("describing a gap and then concluding it is acceptable without verification"). The operator's investment was valuable as practice; the methodology gap that allowed the investment to drift past the assignment's intended bar is a separate, real, registrable gap.

No new findings beyond the seven registered gaps. The 16 Open gaps now in the backlog (G-124–G-137 minus the previously-Addressed ones, G-146, G-148, G-149, plus G-150–G-156) form the largest backlog the suite has carried; recommend next session(s) prioritize G-150 + G-156 as the two highest-leverage single closures.

---

## Review 50 — 2026-05-18 19:30Z

**Scope:** Operator-raised observation pass surfacing two gaps off the back of the Review 49 polarity-sweep work. Triggered by the operator reading line 54 of `vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md` ("Review entries are logged in `vsdd-suite/TECHNICAL-WRITER-REVIEW.md` inside the project being reviewed") and asking whether the suite-development artifact names should align with the project template's names.

**Lens:** Operator-raised observation (source: director-raised per G-133). No adversarial sweep this session — just registration of two gaps the operator identified by reading. Both are structural defects of "stale instruction inherited from pre-G-89 framing" or "naming asymmetry inherited from suite-predates-template history" — well-suited to operator-direct-observation rather than to a cold review pass.

**Session note:** In-session, operator-driven. Sycophancy compensation: the second gap (rename) is a deliberation the operator framed as a question, not an assertion — I provided a recommendation in the gap body but did not flip the operator's question into a foregone conclusion. The first gap (16-file stale line) is a clear defect; flagged for fix in a follow-on session rather than addressing inline because the fix touches 16 files and crosses domain-prompt territory that should get its own commit for reviewability.

---

### New gaps registered

**G-148 — Domain prompt files cite outdated review-log path.** All 16 domain prompt files (14 role + 2 meta in `vsdd-suite/domains/role/` and `vsdd-suite/domains/meta/`) close with a line of shape "Review entries are logged in `vsdd-suite/<DOMAIN>-REVIEW.md` inside the project being reviewed." This predates the G-89 per-domain index + per-session-file structure (registered 2026-05-17). Under G-89, `<DOMAIN>-REVIEW.md` is now the INDEX file; actual review entries live in per-session files at `vsdd-suite/review-log/YYYY-MM-DD-<domain-slug>.md`. The stale line will lead a cold reader (human or AI) to append entries directly to the index file, violating G-89. Resolution: mechanical rewrite across all 16 files to the corrected wording (see G-148's row in GAP-ANALYSIS-LOG.md for the verbatim replacement). Forward-only — prior project review entries that may have followed the old framing remain valid records. **Status: Open.** Consider a follow-on G-139-style hook that asserts every `domains/{role,meta}/*-REVIEW.md` file references the per-session path correctly.

**G-149 — Suite-development artifact naming diverges from project-template naming.** The project template (per G-89 + G-138) uses `<DOMAIN>-REVIEW.md` / `review-log/...` / `FINDINGS-INDEX.md`; suite-development uses `SUITE-REVIEW-INDEX.md` / `review-log/...` / `GAP-ANALYSIS-LOG.md`. Same structural roles, divergent names. Historical reason: suite-development predates the project template; the template was derived and renamed for project-facing clarity, leaving the suite with its older naming. The asymmetry means the suite cannot serve as a worked example of its own template and contributors hold two parallel name systems. **Three resolution options** documented in G-149's row in GAP-ANALYSIS-LOG.md: (A) rename suite-development to match (recommended — dogfood-correct, mechanical rename, forward-only); (B) rename template to match (high-churn, not recommended); (C) document the divergence without renaming (lowest cost, preserves split mental model). **Status: Open** with recommendation (A) but pending operator decision. Rename, if approved, belongs in a dedicated PR (this PR is already substantive).

---

### Coordination

Both gaps are downstream of the structural decisions in G-89 (per-domain index + per-session-file) and G-138 (cross-cutting FINDINGS-INDEX.md):
- **G-148** is a stale-instruction defect: the 16 domain prompts were not swept when G-89 landed.
- **G-149** is a naming-alignment question: the project-template names that G-89/G-138 standardized never propagated back into the suite-development directory.

Both are forward-only fixes. Neither blocks any in-flight work; the suite is functional with the current names and stale instructions (contributors who know the structure work around them). The fixes are about cold-onboarding correctness (G-148) and worked-example coherence (G-149).

Sycophancy self-audit: I considered framing G-149 with a stronger recommendation than (A), since dogfooding is a long-standing suite principle. Rejected — the rename has cross-cutting cost (every internal reference to `GAP-ANALYSIS-LOG.md` and `SUITE-REVIEW-INDEX.md` in the suite must be updated; existing tools / scripts / docs that grep for the old names will break) and the operator's framing as a question signals deliberation, not a foregone conclusion. The body recommends (A) and explains the trade-off; the operator decides scope and timing.

No new findings beyond the two registered gaps. The 15 Open gaps now in the backlog (G-124–G-137 from Review 45 + G-146 from Review 49 + G-148 + G-149) remain as scoped; no re-prioritization triggered by this session.

---

## Review 49 — 2026-05-18 18:20Z

**Scope:** Adversarial review of the suite's value as a supplement to crosslink (driver-requested), followed by explicit articulation of the suite's two-mode operational design principle and a polarity sweep across user-facing docs to land the principle consistently. Files in scope of the sweep: `vsdd-suite/README.md` (Prerequisites, Quickstart, Worked Example Phases 1a/1b/2a/2b/3/4, Loop-until-MVR); `vsdd-suite/primers/4-feedback-integration.md` (§ With crosslink / § Without crosslink → `[crosslink]` / `[manual]`); `vsdd-suite/suite-development/suite-development.md` § Project-level finding index (two equivalent paths → two operational modes); `vsdd-suite/templates/README.md` (mode-independent scaffold + mode-specific usage); `vsdd-suite/suite-development/README.md` (added two-mode design principle statement for contributors).

**Lens:** Design-principle articulation + polarity-sweep. The adversarial review (this session's predecessor work, summarized in conversation) initially proposed a "crosslink-primary, manual fallback" framing that the operator corrected: "Crosslink is primary, manual is fallback is correct but manual must be a 1st class supported method." That correction is the design principle this Review crystallizes — manual mode is not a degraded path; it is a fully supported mode that the suite scaffolds, documents, and reviews with the same rigour as crosslink mode. The sweep re-frames every in-flight doc that previously read as "crosslink path / fallback" into parallel `[crosslink]` / `[manual]` blocks where both blocks carry the same VSDD discipline.

**Session note:** In-session — same operator that drafted the adversarial review and the original framings the sweep corrects. Sycophancy compensation: the predecessor adversarial-review session erred toward "crosslink should be primary, manual is the lesser fallback" and the operator pushed back; the inverse failure (validating crosslink-mode bias) is the one this session must avoid. I acknowledged the sycophancy inversion explicitly before authoring the sweep — the operator's "1st class supported method" constraint is binding on every edit. I tested the polarity by reading each touched section back and asking: does the manual block describe the same discipline with mechanical substitutions (grep instead of label filter, markdown rows instead of issue graph, inline narrative instead of `issue relate`), or does it describe a stripped-down lesser version? Where the answer was the latter, I rewrote until the discipline parity held. The five files listed in Scope all passed this check after the sweep.

---

### New gaps registered

**G-144 — Two-mode operational design principle implicit but never stated.** The suite's structure has always supported both crosslink-mode and manual-mode operation (every primer carries both paths; the templates scaffold both shapes; the finding-index pattern has both routes), but no user-facing doc stated this as a design principle. The README's earlier framing ("crosslink amplifier" / "without crosslink fallback") read as crosslink-first with manual as a degraded escape hatch; an AI authoring agent operating against that framing will continue to drift toward stripping the manual mode. Addressed by adding a "Two modes of operation (design principle)" section to `vsdd-suite/README.md` (user-facing) and `vsdd-suite/suite-development/README.md` (contributor-facing, binding on future contributions). See G-144 row in [GAP-ANALYSIS-LOG.md](../GAP-ANALYSIS-LOG.md) for the full Resolution.

**G-145 — Crosslink-mode additive operations under-used in Phase 4 routing.** The suite's `**Coordination:**` line on cross-domain findings was previously documented only as prose; in crosslink mode this is mechanizable as a structured issue-graph edge via `crosslink issue relate <a> <b>`, but no primer named the command. Addressed by adding `crosslink issue relate` to the `[crosslink]` block of Phase 4 in `vsdd-suite/README.md` and to the `[crosslink]` mode subsection in `vsdd-suite/primers/4-feedback-integration.md`. Manual mode retains the same discipline (coordination recorded inline in the routed finding's narrative) — G-145 is additive, not corrective.

**G-146 — Suite primer auto-injection via `crosslink knowledge` not documented or wired.** Crosslink's `knowledge` subcommand can register reference material to be auto-injected into agent sessions; in crosslink mode this would let the suite register primers once at scaffold time so `crosslink kickoff run` / `crosslink swarm review` invocations load them automatically. Currently primers are loaded by hand in both modes. **Open** — needs verification of `crosslink knowledge`'s actual surface (G-123/G-139 discipline applies — `crosslink knowledge --help` is the source of truth, not speculation), a decision on whether `scaffold-project.sh` or a separate hook should do the registration, and a policy on primer versioning. Manual mode is unaffected by any future implementation; G-144 binds the resolution to preserve the manual path.

**G-147 — Polarity sweep across in-flight suite docs to land G-144's design principle.** Five files re-keyed to parallel `[crosslink]` / `[manual]` blocks in a single sweep. Forward-only: prior review logs and CHANGELOG entries preserve the original framings as audit trail per G-89 narrative-preservation policy. Addressed in this Review by direct edit; the audit trail is git history + this entry.

---

### Resolved

**G-144, G-145, G-147 — Addressed via direct edit in this Review.** The five files listed in Scope now carry the two-mode framing consistently. Specifically:

- `vsdd-suite/README.md` — added "Two modes of operation (design principle)" section above the Prerequisites; updated Prerequisites to split "Baseline (required for both modes)" from "For crosslink-primary mode (recommended)"; restructured Quickstart as two parallel quickstarts; restructured Worked Example Overview table with `[crosslink]` and `[manual]` columns; flipped every phase block (Setup, 1a, 1b, 2a, 2b, 3, 4, Loop-until-MVR) to lead with `[crosslink]` (recommended) then `[manual]` (first-class fallback) and verified the manual block carries the same discipline; added `crosslink issue relate` in Phase 4 per G-145.
- `vsdd-suite/primers/4-feedback-integration.md` — re-framed § "With crosslink (Phase 2+ projects)" → § "[crosslink] — Recommended path"; re-framed § "Without crosslink (manual / Phase 1 projects)" → § "[manual] — First-class fallback path"; added a mode-framing paragraph above both subsections; added Step 4 (`crosslink issue relate`) for G-145; added a coordination-recording sentence to the manual mode's per-finding shape so cross-domain coordination is captured inline.
- `vsdd-suite/suite-development/suite-development.md` § Project-level finding index — re-framed "Two equivalent paths" → "Two operational modes"; re-framed "Crosslink path (preferred when crosslink is in use)" → "[crosslink] mode — recommended path"; re-framed "Manual path (when crosslink is not in use)" → "[manual] mode — first-class fallback path"; added a discipline-parity paragraph stating that every IAR discipline is fully exercisable in manual mode and the trade-off is mechanical, not methodological.
- `vsdd-suite/templates/README.md` — replaced "Manual (suite-only path)" / "With the helper script" / "Crosslink-enabled projects: templates are independent" with a single Usage section that states templates are mode-independent, leads with the recommended scaffold script, then provides a manual scaffold block as the first-class equivalent; added the `cp ... FINDINGS-INDEX.md` step to the manual scaffold block with the "manual mode only" callout per G-138.
- `vsdd-suite/suite-development/README.md` — added a "Two operational modes (design principle)" section between the structural-split paragraph and the "What lives here" section; the section names the principle, what it binds on future contributors (every crosslink-only mechanism MUST have a manual-mode equivalent), and what the trade-off is.

**Resolution:** Statuses flipped Open → Addressed in [GAP-ANALYSIS-LOG.md](../GAP-ANALYSIS-LOG.md) for G-144, G-145, G-147. G-146 remains Open as a forward enhancement.

---

### Coordination

The Review 49 work coordinates with the in-flight PR #20 cluster:
- **G-138** (Addressed, Review 46) — the finding-index pattern is what gives crosslink-mode's `issue relate` edges their queryable target population. G-145's `crosslink issue relate` example is only useful because findings are filed as issues per G-138.
- **G-139** (Addressed, Review 48) — the polarity sweep added new `crosslink <subcommand> --flag` citations in five files; the G-139 hook validates these automatically on commit. The sweep's correctness is gated on the hook passing clean. Specifically, the Phase 4 `crosslink issue relate <a> <b>` addition has no `--flag` portion and so isn't validated, but the surrounding context (`issue comment <id> ... --kind <kind>`, `issue close <id>`, `swarm fix --from-label`, `swarm fix --budget-aware`) is in scope.
- **G-123** (Addressed, Review 43) — the parent discipline. G-144's principle constrains future G-123-style mechanism additions: any auto-verification added to crosslink mode must also preserve the manual mode's parity (e.g., a hook that auto-files `crosslink issue create` must not become required infrastructure that breaks manual-mode users).

The 14 Open gaps from Review 45 (G-124–G-137) remain as scoped; G-144 / G-145 / G-147 do not affect that backlog. G-146 adds one Open gap to the backlog as a candidate enhancement. The recommended sequencing for follow-on closure is unchanged; G-146 would slot into the operational/tooling cluster once `crosslink knowledge`'s surface is verified.

Sycophancy self-audit: I considered framing the sweep as "minor wording adjustments" given that the underlying structure already supported both modes. Rejected: the operator's correction was substantive (the prior framing did read as crosslink-first with manual degraded; the operator was right to push back), and the sweep's effect on doc reading order changes user behavior — a new user landing on the Quickstart now sees the manual quickstart as a peer, not as a footnote. "Minor wording" would have undersold the principle.

---

## Review 48 — 2026-05-18 01:31Z

**Scope:** Address G-139 by implementing the `check-crosslink-references` pre-commit hook proposed in Review 47. The hook mechanizes the G-123 discipline ("verify external-dependency feature references against governing documentation") by automatically running `crosslink <subcommand> --help` for every cited command in user-facing suite docs and failing the commit if any cited long flag is missing from the help output.

**Lens:** Tooling-addresses-recurring-discipline-failure lens. The closure pattern is itself a precedent: when a manual discipline (G-123) fails twice in the same way, the rule change earned by recurrence is a tooling fix (G-139), not a stricter discipline. Future similar gaps should follow the same arc.

**Session note:** In-session — same operator that registered G-139 and authored the prior wrong references the hook was designed to catch. Sycophancy compensation: I tested the hook against the full suite *before* claiming it was correct, which surfaced (a) 23 historical-narrative false-positives in the review-log/gap-registry/index files, addressed by adding a self-skip list to the hook AND an `exclude:` filter in `.pre-commit-config.yaml`; and (b) one real catch in `suite-development.md:104` where the G-123 governing-standard text quoted `crosslink init --with-suite` as a worked example — rewritten to "a fictitious `--with-suite` flag attributed to crosslink's `init` subcommand" to convey the same information without the grep-trigger substring. The pre-test verification is what produced the (a)/(b) outcomes; without it the hook would have shipped broken or with the policy violation un-noticed.

---

### Resolved

**G-139 — `check-crosslink-references` pre-commit hook implemented, tested clean against the full suite.**

The hook is a Python script (with `.sh` filename for parity with the existing `check-review-log-anonymization.sh`; the shebang routes to `python3`). Behavior:

- Scans staged text files for `crosslink <subcommand> ... --<flag>` patterns. Subcommand is 1–3 words; longest-first match via `crosslink <tokens> --help`.
- For each `(subcommand, flag)` pair, validates that the flag appears in the help output's option lines.
- Fails the commit if any cited long-form flag is not in the help. Reports file:line, the cited subcommand+flag, and the set of valid flags for the subcommand.
- Skips gracefully when crosslink is not installed (`shutil.which("crosslink")` returns `None`; the hook prints a warning and exits 0 — CI-environment safe).
- Self-skips known historical-narrative files (CHANGELOG, COMPATIBILITY, GAP-ANALYSIS-LOG, SUITE-REVIEW-INDEX, review-log/*, FINDINGS-INDEX) where past wrong commands are preserved as audit trail. Defense in depth: `.pre-commit-config.yaml` also `exclude:`s the same paths for efficiency at the staged-files level.
- Scope (long flags only): short-form flags (`-l`, `-s`, etc.) are not validated in this version. Narrow scope catches both recorded G-123 recurrences (`--with-suite`, `--comment`) while keeping the regex tractable. Short-flag validation would extend the scope but would also surface false positives from incidental short-flag-like substrings in narrative prose; not warranted by the current recurrence evidence.

`.pre-commit-config.yaml` updated with the new `check-crosslink-references` hook entry, scoped via `files:` to `vsdd-suite/**/*.{md,sh}` and `<project>/vsdd-suite/*.md` (single-level — project per-domain index files), and `exclude:`d for the historical-narrative paths listed above.

**Tested clean against the full current suite:** all user-facing docs (README.md, primers/, supplements/, hooks/, templates/, crosslink-contract.md, suite-development/README.md, suite-development/suite-development.md, bookmark-cli/vsdd-suite/QUALITY-ENGINEER-REVIEW.md, etc.) — zero false positives. The historical-narrative files (review-log/, GAP, INDEX, CHANGELOG, COMPATIBILITY, FINDINGS-INDEX) are correctly skipped; their `--with-suite` and `--comment` citations remain as audit trail.

**One narrative correction applied during the test:** `suite-development/suite-development.md:104` (the G-123 governing-standard section) quoted `crosslink init --with-suite` directly in prose. The hook caught it; rewrite preserves the information ("a fictitious `--with-suite` flag attributed to crosslink's `init` subcommand") without the grep-trigger substring. This is a feature, not a bug — the contributor primer is a reference doc that should not itself cite non-existent commands even in failure-mode discussion; the historical-narrative files are the appropriate home for verbatim citations.

**Resolution:** Status flipped Open → Addressed in [GAP-ANALYSIS-LOG.md](../GAP-ANALYSIS-LOG.md). G-123 is now mechanism-backed; recurrence of the speculation pattern would fail the commit hook rather than ship to users.

---

### Coordination

The Review 47 → Review 48 arc closes the G-123 / G-139 / G-118 cluster cleanly:
- G-123 (Review 43) introduced the discipline.
- G-139 (Review 47) recognized the discipline alone was insufficient.
- G-139 closure (this Review 48) implemented the mechanism.

Future similar patterns should follow the arc: discipline → recurrence-recognition → tooling. The "earned by recurrence" doctrine (Review 37 / G-99 framing) names the trigger; G-123 → G-139 is the first end-to-end instance of the discipline-to-tooling promotion pattern in the suite's history.

No new gaps surfaced this session. Sycophancy self-audit: I considered pushing back against adding the `exclude:` filter on the basis that "if it's wrong text, fix the wrong text rather than excluding files." Rejected: the historical-narrative files DELIBERATELY preserve past wrong citations per Review 43's narrative-preservation policy. Excluding them from the hook is the policy-coherent answer; rewriting them would violate Review 43.

The Review 45 backlog (G-124–G-137) remains as scoped; G-139's closure does not affect that backlog directly, but the hook's mechanism is reusable infrastructure that G-129 (CHANGELOG-currency hook) could share — when G-129 is addressed, the hook can be modeled on `check-crosslink-references.sh`'s shape (Python with shebang, pre-commit `files:`/`exclude:` scoping, self-skip safety net).

---

## Review 47 — 2026-05-18 01:21Z

**Scope:** G-118 follow-on (driver-requested) to update `crosslink-contract.md` with the verified surface for the G-138 finding-index pattern. The verification step (per the G-123 external-dependency discipline) ran `crosslink <subcommand> --help` against installed crosslink v0.8.0 for every command the suite references — and surfaced a second-instance G-123 recurrence: `crosslink issue close --comment "<text>"` was referenced in 5 places across the suite (1 in the existing `crosslink-contract.md`, 3 in `README.md` § Worked example Phase 3 and Phase 4, 1 in `primers/4-feedback-integration.md`) but the actual `close` subcommand does not accept `--comment`. The rationale belongs in a prior `crosslink issue comment <id> "<text>" --kind <kind>` followed by `crosslink issue close <id>`. Both the G-138 surface addition and the `--comment` correction land in this session.

**Lens:** Verification-against-installed-CLI applied as a sub-lens of suite-internal review. Specifically anchors every claim about crosslink CLI surface to a `crosslink <cmd> --help` invocation; rejects any claim that does not match.

**Session note:** In-session. Sycophancy compensation: I authored the `--comment`-on-close references in earlier session work (Reviews 38–43 + bookmark-cli reference impl). The G-123 discipline was supposed to prevent this exact pattern; that this is the SECOND instance (the first was `crosslink init --with-suite` in Reviews 40–42, corrected in Review 43) is a recurrence signal — not just "I was careless" but "the discipline as currently authored is insufficient against AI-agent recurrence." Registering G-139 as a tooling gap separately rather than treating the recurrence as a one-off correction.

---

### Addressed

**G-118 follow-on — `crosslink-contract.md` extended with G-138 finding-index commands; existing `--comment` row corrected.**

The pre-existing dependency-surface table (Phase 3 row 3) claimed `crosslink issue close <id> --comment "<text>"` with `--comment` as a required flag. Verification: `crosslink issue close --help` shows only `<ID>` positional + `--no-changelog` flag; no `--comment` exists. Corrected the row to `crosslink issue close <id>` and added two new rows above documenting the correct comment-then-close pattern (`issue comment <id> "<text>" --kind <kind>` then `issue close <id>`). Added `crosslink issue unlabel` and refined `issue list` to use `-s` (short form for `--status`) per the verified `--help` output.

Added a new section to `crosslink-contract.md`:

- **`### G-138 finding-index commands (crosslink path)`** — table enumerating: `issue create` with structured labels (`-l` repeatable; verified); `issue list -l <axis>:<value> -s <status>` for single-axis filter; multi-axis composition note (single-label filter only — use `--json | jq` for multi-axis); `crosslink tui` for interactive browse; `issue label` / `issue unlabel` for label adjustment; the reclassify sequence (unlabel-then-label-then-close, comment-then-close); `export -f json -o <path>` / `import <INPUT>` for manual ↔ crosslink migration.
- **`### Crosslink commands the suite does not depend on`** — explicit out-of-scope list (kickoff, container, sentinel, knowledge, style, mc, serve, tui, trust, locks, sync, migrate, config, context, integrity, compact, prune, timer). For audit clarity: a future contributor knows the contract surface is intentionally scoped, not accidentally narrow.

The "Tested-against version" line at the top updated to: "every command and flag in this file was verified against `crosslink <subcommand> --help` output on 2026-05-17 (Review 46 + 47 verification pass)."

**Correction sweep** for the `--comment`-on-close error:
- `README.md` § Worked example Phase 3 — corrected the Hallucinated example to use `comment --kind decision` then `close`; corrected the Resolved example to use `comment --kind resolution` then `close`; updated `issue list --status` to `issue list -s` (matching verified short-form).
- `README.md` § Worked example Phase 4 — corrected the routed-finding closure example to use `comment --kind resolution` then `close`; added explanatory note about `issue close` not accepting `--comment`.
- `primers/4-feedback-integration.md` § Step 5 — corrected the routed-finding closure prose to use the `&&`-chained `comment --kind resolution && close` pattern with explanatory parenthetical.

**Verification of remaining grep matches:** the 2 remaining matches for `issue close.*--comment` in the suite are both inside the CORRECTION TEXT itself (the explanatory phrase "`issue close` does not accept `--comment`"). Those are the correction notes; not actual command examples.

**Resolution:** G-118 follow-on closed via the contract extension + correction sweep. The crosslink-contract.md now serves as the suite's canonical record of the verified crosslink dependency surface AT THE FLAG LEVEL — any future suite documentation referencing a crosslink command must match this file or update both.

---

### New gap registered

**G-139 — G-123 manual discipline insufficient against AI-agent recurrence; CLI-verification tooling needed.**

The pattern: Reviews 38–43 introduced `crosslink init --with-suite` references that don't exist (corrected in Review 43, G-123 registered). Reviews 38–46 introduced `crosslink issue close --comment` references that don't exist (corrected in this Review 47). Both were violations of the G-123 discipline ("before referencing an external tool's feature, verify against that tool's governing documentation") that landed despite the discipline being explicitly documented.

The recurrence shape: an AI authoring agent operating inside a long session naturally pattern-matches "this command probably accepts this flag" against precedent from other CLIs (`gh pr close --comment "..."`; `jira issue close --comment "..."`; etc.). The G-123 discipline asks the agent to verify before writing — but the discipline runs *in the same context* that produced the speculation, so the agent's confidence in the speculation overrides the verification step.

**The discipline as currently authored is insufficient against this failure mode.** Two instances across four sessions = recurrence trigger per the "earned by recurrence" doctrine (Review 37 / G-99 framing). The rule change earned by the recurrence: add an automated verification step to the suite's pre-commit hook surface.

**Resolution sketch:** Add a `vsdd-suite/hooks/check-crosslink-references.sh` script that:
1. Greps the staged suite documentation files for `crosslink \w+( \w+)?( --\w+)*` patterns.
2. For each unique pattern found, runs `crosslink <subcommand> --help` and checks that every cited flag appears in the help output.
3. Fails the commit if any cited flag is not in the help — with a clear message naming the file:line, the cited flag, and the actual help output.
4. Wired into `.pre-commit-config.yaml` scoped to `vsdd-suite/**/*.md` and `vsdd-suite/**/*.sh`.

The hook converts G-123 from "discipline the author must remember" to "mechanism that fires automatically." Operates only when crosslink is installed (the hook checks for `command -v crosslink` and skips with a warning otherwise — a CI environment without crosslink reports the skip rather than failing).

**Severity:** Mission-critical High / Speculative Medium. Mission-critical because every recurrence of the pattern ships docs that mislead users (and may waste their time when they try the wrong command); speculative is medium because the tooling cost is modest and the pattern is well-understood.

**Status:** Open. Cross-coordinate: G-118 (the contract file is the source of truth that the hook would validate against, so the hook need not also re-run all `--help` invocations every commit if the contract file is up-to-date); G-123 (the discipline this hook mechanizes); G-129 (CHANGELOG-currency hook, similar tooling shape — could share infrastructure).

---

### Coordination

The Review 47 work coordinates with multiple recently-closed gaps:

- **G-118** — this Review extends the crosslink-contract.md surface. The contract file is now the canonical record at the flag level; any future suite documentation referencing crosslink must match.
- **G-138** — the new crosslink-finding-index commands documented in this Review's contract update are the verified surface for G-138's crosslink path. The path is now backed by tested CLI evidence.
- **G-123** — second recurrence acknowledged. Triggers G-139 (tooling fix) per "earned by recurrence" doctrine.

The 14 Open gaps from Review 45 (G-124 through G-137) are unaffected. The recommended-sequencing plan for follow-on closure still stands; G-139 inserts at the top of the operational/tooling cluster as the cheapest single addition that prevents further G-123 recurrence.

No new findings from the verification pass beyond the `--comment` correction and G-139 registration. The other crosslink commands referenced across the suite (`crosslink design`, `quick`, `milestone *`, `session *`, `swarm *`, `issue label`, `issue block`, `issue list`, `issue create`) all match verified `--help` output.
