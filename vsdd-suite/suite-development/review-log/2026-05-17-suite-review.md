# Suite Review — 2026-05-17

## Review 46 — 2026-05-17 22:04Z

**Scope:** Driver-raised observation that "some files exceed 20K lines" (referring to ITC's domain review logs — the largest is `SOLUTION-OWNER-REVIEW.md` at 2495 lines / 252KB, and several others are in the 1500–2100 line range). Driver requested a TOC-and-split pattern matching the suite's own `SUITE-REVIEW-INDEX.md` + `review-log/` shape, with crosslink features where available. Initial AskUserQuestion clarified intent (TOC-only, respecting G-89's forward-only constraint); follow-on clarification: "This is for new projects only and is not applied retroactively." ITC stays untouched per G-89 / Review 38 ratification. The work this session: formalize the project-level finding registry pattern (G-138) — a cross-cutting index orthogonal to the per-domain round index — and apply it to bookmark-cli as the reference implementation.

**Lens:** Forward-only suite enhancement driven by accumulated-evidence observation. The driver's underlying need (quick-lookup across large finding sets) is real for any project that grows beyond ~5–10 active findings. The suite's per-domain index + per-session-file structure (G-89) indexes ROUNDS; a project still needs a FINDINGS-level index to answer cross-cutting queries ("show me all Open"; "show me everything on Layer 2"; "show me everything the director raised manually"). Adding the finding-index pattern is a small structural addition that compounds with the existing G-89 structure rather than replacing it.

**Session note:** In-session. Sycophancy compensation: the driver's instinct was to split the existing ITC files retroactively; my G-89-aware response surfaced the forward-only constraint and forced an explicit choice. The driver chose the constraint-respecting path, then clarified that the entire approach is forward-only. The work that landed (G-138) addresses the underlying need — quick lookup across findings — via a forward-only pattern that does not touch ITC. The driver's original instinct (split ITC) and my constraint-flagging (G-89) and the driver's clarification (forward-only) compose into a clean outcome.

---

### New gap registered

**G-138 — Project-level finding index (cross-cutting registry) not established.**

The G-89 per-domain index + per-session-file structure indexes rounds but not individual findings. Projects with substantial finding counts have no cross-cutting view. The driver's "quick lookup" instinct names the need; the manual-path (FINDINGS-INDEX.md) and crosslink-path (labeled crosslink issues) carry the same information shape so projects can adopt either. Forward-only per G-89's framing — applies to projects starting after 2026-05-17.

**Severity:** High mission-critical / Medium speculative. The recurrence evidence: ITC accumulated ~50+ findings across 13 domains over 7 layers with no cross-cutting index; PROCESS.md L6 operator quote "I need a mechanism to make sure deferred items are properly worked. Maybe like some sort of task manager lol lmao" names the gap implicitly.

---

### Addressed (G-138 — same session)

**1. `suite-development/suite-development.md` § Governing standard for project-level review logs gained a new `### Project-level finding index (cross-cutting registry)` subsection.**

Defines two equivalent paths (crosslink and manual), enumerates the label-axis convention for the crosslink path (`domain:<slug>`, `layer:N`, `round:N`, `finding:N`, `classification:<class>`, `source:<source>`), names the manual-path equivalent (`<project>/vsdd-suite/FINDINGS-INDEX.md` structured like the suite's GAP-ANALYSIS-LOG.md), and states the path-switch trade-off (supported but not free; choose at scaffold time).

**2. `templates/PROJECT-FINDINGS-INDEX-template.md` (new) — manual-path template.**

Header + Quick-lookup recipes + Findings registry table + Cross-references. Mirrors the GAP-ANALYSIS-LOG.md shape applied at project scale. New projects scaffold this; the template includes 2 example rows that the project deletes when populating.

**3. `templates/scaffold-project.sh` updated to copy `PROJECT-FINDINGS-INDEX-template.md` to `vsdd-suite/FINDINGS-INDEX.md` during scaffolding.**

The script always copies it; projects using the crosslink path can delete it. Output message names the conditional: "delete if using crosslink for finding tracking."

**4. `templates/README.md` updated to list the new template in the Contents table.**

One row added with the conditional note.

**5. `README.md` § Worked example Phase 3 updated to mention finding tracking.**

One sentence added after the per-domain round-filing instruction: "Also append a row to the project-level finding index (`vsdd-suite/FINDINGS-INDEX.md` for the manual path, or `crosslink issue create` with structured labels for the crosslink path)."

**6. `bookmark-cli/vsdd-suite/FINDINGS-INDEX.md` (new) — populated reference-impl demonstration.**

Three rows for the three QE Review 1 findings (F-001 Phase 2a → 2b commit-boundary discipline, Resolved; F-002 missing edge-case test coverage, Resolved; F-003 insufficient-test-count claim, Hallucinated). Demonstrates the manual-path structure with real finding data; future Phase 3 reviews on bookmark-cli would append rows for new findings.

**Resolution:** Status flipped Open → Addressed in [GAP-ANALYSIS-LOG.md](../GAP-ANALYSIS-LOG.md) in the same session as registration. The pattern is now part of the suite for new projects; the reference impl demonstrates the manual path; crosslink path is documented but not exercised in this session (no crosslink-using portfolio project yet).

---

### Coordination

G-138 coordinates with:
- **G-89** — the same forward-only framing applies; G-138 sits *on top of* G-89's per-domain structure, not as a replacement. A project gets both: per-domain index files (G-89) AND a cross-cutting findings index (G-138).
- **G-130** (deferral lifecycle) and **G-133** (Source: director-raised classification) — both reference structured-finding metadata that G-138's label axes (or manual columns) capture. G-138's adoption makes G-130 and G-133 trivially queryable when they're addressed.
- **G-118** (crosslink-contract.md) — G-138's crosslink path adds new commands to the verified surface: `crosslink issue list -l <label>`, `crosslink issue create --label <axis>:<value>`, etc. The contract file should be updated to enumerate these as part of G-138's full closure. (Noted as a follow-on; not blocking.)

No new gaps surfaced this session. Sycophancy self-audit: the temptation was to either (a) silently violate G-89 (the driver's instinct supported the override) or (b) implement only the TOC subset without the cross-cutting index. Both temptations rejected — the constraint was surfaced explicitly via AskUserQuestion (driver confirmed forward-only), and the addition was implemented at full scope (both paths documented + manual template + reference-impl population + scaffold-script update + README integration).

The Review 45 + Review 46 pairing is the closure of a coherent observation arc: Review 45 mined ITC for recurring patterns and registered 14 backlog gaps; Review 46 addresses one of them (G-138 — the project-level finding index — though not in the original 14, surfaced by the driver in the same session) by forward-only suite enhancement. The remaining 14 from Review 45 stay Open per the recommended-sequencing plan.

---

## Review 45 — 2026-05-17 21:46Z

**Scope:** Cross-project pattern-mining from `issue-tracker-cli/iterative-adversarial-refinement/` (~20K lines across 13 domain review logs + CLOSURE-PROTOCOL.md) plus `issue-tracker-cli/PROCESS.md` (547 lines, 7-layer first-person director retrospective). Read with two complementary methods: (a) Explore subagent scanned the 13 domain logs for recurring patterns (defect classes appearing in multiple layers or across multiple domains); (b) direct read of PROCESS.md for operator-experience friction the agent could not see (cold-session connectivity issues, cost concerns, loop-shape questions, "task manager lol lmao" deferral observation). The combination surfaces both finding-recurrence and operator-friction signals; either alone would have missed half the patterns.

**Lens:** Cross-project pattern-mining. The completed-project IAR corpus is treated as evidence for suite-level gaps: a defect class that recurred across multiple layers in one project is signal that the suite's upstream primers and dimensions are letting the class through. Distinct from suite-internal adversarial review (Review 38-44 arc) and from doctrine ratification (Review 42, 43). This lens has not been used in a prior suite review and is itself a worthwhile methodology innovation — register as a recurring suite-development discipline.

**Session note:** In-session — the same operator landed the Review 38-44 arc and the `bookmark-cli` reference implementation earlier this session. Sycophancy compensation: the Explore subagent ran in its own context (no exposure to the operator's framing), reporting 10 patterns; the operator's direct read of PROCESS.md added 4 more patterns the agent could not have seen (operational-cost, loop-rigidity, manual-test elevation, director-raised-finding classification gap). Cross-validation: 7 of the 10 agent-patterns are also explicitly named in PROCESS.md retrospectives, providing independent confirmation. The 3 agent-only patterns (rustdoc verification command; serde schema-evolution discipline; mutation-resistant assertions) are anchored to specific review-log evidence (TW R4 vs R6 commands; DE R1+R3+R6 serde questions; QE R3+R5+R8 mutation gaps).

---

### New gap registered

This review registers 14 new gaps (G-124 through G-137) from the ITC pattern-mining lens. Clustered into three groups in the registry: defect-class generalizations (G-124–G-128), process/discipline gaps (G-129–G-133), and operational/tooling gaps (G-134–G-137).

| G-ID | Title | Severity | Cluster |
|---|---|---|---|
| G-124 | Per-property defense pattern for free-form text fields not in primer | Critical | A (defect-class) |
| G-125 | Error-message escape interpolation not named in Security domain | High | A (defect-class) |
| G-126 | Asymmetric trust boundary (create vs load) not in DE primer | High | A (defect-class) |
| G-127 | Empty-state regression on every new filter dimension | Medium | A (defect-class) |
| G-128 | Mutation-resistant test assertions not in QE primer | Medium | A (defect-class) |
| G-129 | Documentation currency requires automation not review discipline | Critical | B (process) |
| G-130 | Deferral lifecycle and task ownership — promote ITC §3 to suite-default | High | B (process) |
| G-131 | Loop-count framing: rigidify trigger not default | High | B (process) |
| G-132 | Manual testing as peer surface to IAR (not a checkbox) | High | B (process) |
| G-133 | Director-raised finding classification | Medium | B (process) |
| G-134 | Cold-session dispatch tooling absent | High | C (operational) |
| G-135 | Cost/token discipline — new "AI Engineering" meta-domain candidate | Medium-High | C (operational) |
| G-136 | Suite-level phase-flow visualization missing from README | Medium | C (operational) |
| G-137 | Rustdoc verification command in Rust supplement insufficient | Low | C (operational) |

**Methodology gap also surfaced** (recorded here, not separately registered): the pattern-mining lens itself is novel. The suite's `suite-development.md` § Suite review entry format Lens-field examples include defect-class lens, registry-walk lens, role-based lens — but not "cross-project pattern-mining lens." This Review 45 entry establishes it as a precedent; future reviewers can cite it. If a second project (after `bookmark-cli` grows beyond Layer 1) provides a second corpus, the lens warrants its own primer treatment.

---

### Coordination

The 14 gaps cluster into three coordinated decisions a future closure session would face:

**Cluster A — Defect-class generalizations (G-124–G-128).** Five gaps share a common pattern: a class of defect that recurred multiple times in ITC because the suite did not name the generalization upstream. Resolution shape: each gets a primer-text addition (1b-decomposition.md for A1/A4; Security domain for A2; Data Engineer domain for A3; Quality Engineer domain for A5) + a Rust supplement update. Total: ~7 primer/domain edits. Cross-coordinate: G-124 + G-125 + G-126 are tightly linked (all about per-property defense); could land together. G-127 + G-128 are independent but cheap.

**Cluster B — Process/discipline gaps (G-129–G-133).** Five gaps require methodology changes, not just dimension additions. G-129 is the cheapest single change (one hook script + .pre-commit-config.yaml template); G-130 promotes the ITC-specific CLOSURE-PROTOCOL.md §3 to a suite-default standard in `suite-development.md`; G-131-G-133 are paragraph-level primer additions to `primers/3-review-session.md` (loop count, manual testing elevation) and `suite-development.md` (director-raised classification). Cross-coordinate: G-130 should land before G-133 — the auto-Backlog mechanism is the structural prerequisite for "Source: director-raised" being trackable.

**Cluster C — Operational/tooling gaps (G-134–G-137).** Four gaps where the suite needs to produce tooling or content beyond primer text. G-134 (cold-session dispatch script) is small (~50 lines bash) and unblocks the recurring "I don't have a good manual workflow" friction. G-135 (cost/token meta-domain) is a multi-session effort warranting its own arc — likely the largest single piece of follow-on suite work. G-136 (phase-flow diagram) and G-137 (rustdoc command fix) are small README/supplement edits. Cross-coordinate: G-135 is the only gap in this review that's *larger* than a single closure session; could be its own future review arc.

**Recommended sequencing for a follow-on closure session:** G-129 first (immediate value, smallest cost); G-124 + G-125 + G-126 together (the per-property defense cluster); G-130 (deferral lifecycle); G-131 + G-132 (loop and manual-test framing). That's 7 closures in one session, addressing the highest-leverage gaps. The remaining 7 (G-127, G-128, G-133, G-134, G-135, G-136, G-137) are individually small enough to bundle into a second session or absorb opportunistically.

**Cross-coordinate to future bookmark-cli work:** when `bookmark-cli` advances beyond Layer 1 (Layer 2 tag + filter is the natural next step), the same pattern-mining lens can be applied to its accumulating reviews — and any recurrence of the patterns named here would be confirming evidence that the gap is suite-level, not project-specific.

---

## Review 44 — 2026-05-17 04:43Z

**Scope:** Reference implementation landing event. The `bookmark-cli/` reference implementation was built at the portfolio root, scaffolded via the suite's `templates/scaffold-project.sh`, with Layer 1 complete through Phase 2b (8/8 tests pass against the Red Gate suite) and a demonstration Phase 3 QE Review 1 filed in the new per-domain index + per-session-file structure (G-89 forward-only convention). Closes G-112 as the suite's first end-to-end canary; refines G-106's status.

**Lens:** Reference-implementation lens — not an adversarial review of the suite per se, but a logged suite-development event that the suite's worked example now has a verifiable artifact behind it. The suite's documentation is no longer purely hypothetical for Layer-1-through-Phase-3 scope.

**Session note:** In-session — the same operator authored the suite documentation and the reference implementation. Sycophancy compensation: the reference implementation's QE Review 1 explicitly flagged the Phase 2a → 2b commit-boundary discipline failure (the operator wrote tests + implementation in one chat without an intervening commit). That self-flagging is the dogfooding signal: a real project under the suite would commit the Phase 2a Red Gate state separately; the reference impl combined them as a deliberate scope tradeoff and documented the tradeoff as a Finding 1 in `bookmark-cli/vsdd-suite/review-log/2026-05-17-quality-engineer.md`.

---

### Resolved

**G-112 — End-to-end reference implementation of the worked example.**

The `bookmark-cli/` project at portfolio root is the suite's first end-to-end canary. Artifacts:

- `bookmark-cli/DESIGN.md` — Phase 1a contract (scope, behavioral contracts, edge case catalog, interface definitions, verification architecture, technology choices with rationale, constraints, open questions).
- `bookmark-cli/TODO.md` — Phase 1b layer plan (Layer 1 fully detailed with ACs, Red Gate test plan, runnable manual testing checklist; Layers 2 + 3 scoped only).
- `bookmark-cli/Cargo.toml` — minimal Rust crate spec (clap, serde, serde_json, chrono, anyhow; assert_cmd + predicates + tempfile dev-deps).
- `bookmark-cli/tests/bookmarks.rs` — Phase 2a Red Gate (4 integration tests mapping 1:1 to the 4 ACs; invokes the compiled `bm` binary via `assert_cmd` per CLI supplement § QE; uses `tempfile` for per-test storage isolation).
- `bookmark-cli/src/lib.rs` — Phase 2b pure-core storage logic (Bookmark + BookmarkStore with load/save/add/newest_first; 4 unit tests for the pure primitives).
- `bookmark-cli/src/main.rs` — Phase 2b effectful shell (clap dispatch, env-var storage path resolution, exit-code contract per DESIGN.md).
- `bookmark-cli/vsdd-suite/` — scaffolded via the suite's `templates/scaffold-project.sh` (the script ran clean against the empty `bookmark-cli/` directory and produced the 7 default-active core domain index files + DESIGN/README skeletons). One per-domain index file customized (`QUALITY-ENGINEER-REVIEW.md`) with a realistic Review 1 entry filed in `vsdd-suite/review-log/2026-05-17-quality-engineer.md`. The other 6 indices remain as scaffolded template stubs — accurate for the project state (no other domain reviews have been filed; the reference impl's demonstration purpose is satisfied by one customized index).
- `bookmark-cli/README.md` — project README per the suite's `PROJECT-README-template.md` (purpose, prerequisites, install/run/test, methodology pointer to the suite, phase progression table).

**Verification:** `cd bookmark-cli && cargo test` against the working tree at this commit produces:

```
running 4 tests
test tests::newest_first_sorts_descending_by_timestamp ... ok
test tests::load_returns_empty_for_missing_file ... ok
test tests::load_returns_empty_for_empty_file ... ok
test tests::save_then_load_roundtrips ... ok

test result: ok. 4 passed; 0 failed

running 4 tests
test tests_list_empty_state ... ok
test tests_add_rejects_empty_url ... ok
test tests_add_creates_bookmark ... ok
test tests_list_orders_newest_first ... ok

test result: ok. 4 passed; 0 failed
```

The worked example is no longer hypothetical at the Layer-1 granularity. A new user reading `vsdd-suite/README.md` § Worked example can now follow `bookmark-cli/` as a side-by-side concrete artifact.

**What's NOT included** (scope honestly disclosed): Layers 2 + 3 are not built; only QE Phase 3 Review 1 is filed (the other 6 active-core-domain indices are scaffolded stubs awaiting their first round); no Phase 4 routing has occurred because no live cross-domain findings exist; no Layer 1 merge gate has run because this is reference work, not delivery work; the Phase 2a → 2b commit-boundary discipline was not strictly satisfied (acknowledged in the QE Review 1's Finding 1). These omissions are appropriate for the reference-implementation purpose — G-112 asked for end-to-end exercise of the worked example at toy-project scale, not for a complete production-quality run.

**Resolution:** Status flipped Open → Addressed in [GAP-ANALYSIS-LOG.md](../GAP-ANALYSIS-LOG.md). Portfolio README updated to include `bookmark-cli/` as the third portfolio project with the "Reference implementation" framing.

---

### Refined status

**G-106 — Sample crosslink command outputs remain Open.**

The reference implementation closes G-112 but does not close G-106, because the reference impl was built via the suite-only (crosslink-free) path per the G-117 ratification of manual copy as the canonical default. G-106 specifically asked for sample outputs of `crosslink workflow diff`, `crosslink swarm review`, etc. — those commands were not exercised in this session.

Refined status: G-106 stays Open with revised reason. The natural closure is a separate session that runs crosslink against a follow-on toy project (could be Layer 2 of bookmark-cli with crosslink integration, or a different toy project explicitly demonstrating the `[+crosslink]` amplifier path). Until then, the worked example's `[+crosslink]` blocks remain documented-but-unverified. The contract-testing canary (G-118's crosslink-contract.md) holds because `bookmark-cli` itself doesn't depend on crosslink; G-106's resolution awaits a project that does.

---

### Coordination

The Review 40 + 41 + 42 + 43 + 44 onboarding-experience arc now closes at 23 of 24 gaps addressed (the 21 from prior arcs + G-123 from Review 43 + G-112 from this Review 44 = 23 Addressed; 1 remains Open: G-106). The remaining gap is genuinely scoped to a follow-on session and does not block any current suite functionality.

The reference implementation also serves as a dogfooding test of the suite's own scaffolding work that landed in Reviews 40 + 41 + 42: `templates/scaffold-project.sh` ran cleanly against an empty target directory; `DESIGN-template.md` and `PROJECT-README-template.md` were sufficient starting points (replaced in the reference impl with project-specific content); `DOMAIN-REVIEW-template.md` produced usable index file stubs that one customization pass adapted into `QUALITY-ENGINEER-REVIEW.md`. The scaffolding work is verified by use.

No new gaps surfaced by Review 44. Self-sycophancy check: I was tempted to claim "the reference implementation is complete" — caught that and rejected it. Only Layer 1 is built; only QE Phase 3 is exercised; the commit-boundary discipline failed. These omissions are honestly disclosed in the bookmark-cli QE review and in the "What's NOT included" paragraph above. The honest framing is: G-112 satisfied at toy-project granularity; further validation is a future session's concern.

---

## Review 43 — 2026-05-17 03:10Z

**Scope:** Correction review. The driver flagged that Reviews 40, 41, and 42 introduced references to a non-existent `crosslink init --with-suite` feature, treating it as a "coordination ask against crosslink upstream." The driver does not control the crosslink repository, and the feature is not in crosslink's current documentation. The references implicitly committed to a PR against an out-of-scope repo. This review documents the scope-creep, corrects the user-facing references, and registers a class-level gap for the underlying suite-development discipline failure.

**Lens:** Self-correction lens — an in-suite finding raised by the driver after Reviews 40–42 closed but before the broader work was committed. SO discipline applied (the suite IS a project; its docs are its spec; references to features in external dependencies must be substantiated against those dependencies' governing documentation).

**Session note:** In-session. Sycophancy compensation: this finding is anchored to driver-supplied evidence ("I don't think it supports `crosslink init --with-suite`" + "this would require a feature PR to a repo I don't control"). My prior framing of `--with-suite` as "coordination ask" was speculative — I did not verify against crosslink's governing documentation before introducing the term, and the term implies a commitment I had no authority to make. The correction is unambiguous; no judgment call required.

---

### Resolved

**Finding 1 — Suite documentation referenced a non-existent crosslink feature (`crosslink init --with-suite`).**

Six user-facing occurrences across five files mentioned `crosslink init --with-suite` as a "coordination ask," "forthcoming feature," or accepted-variant option in the suite-to-project coupling discussion. The references were introduced by me in Reviews 40–42 without verifying against crosslink's governing documentation. Per the driver: the feature does not exist in crosslink and would require a PR against a repository the driver does not control. Treating it as a coordination ask implicitly committed someone (the driver or me) to filing that PR.

Corrections applied to user-facing docs (present-tense statements of capability):

- `vsdd-suite/README.md` — Quickstart Phase 1 step rewritten to drop the `--with-suite` reference and clarify that crosslink and the suite are independent (running both requires running each separately). The accepted-variant table dropped the `crosslink init --with-suite` row entirely and gained a one-line clarification that crosslink and the suite are independent tools with no shared scaffolding.
- `vsdd-suite/templates/README.md` — § Usage paragraph corrected to drop the "forthcoming `crosslink init --with-suite`" framing; replaced with a factual statement that crosslink and the suite are separate tools that each scaffold their own state.
- `vsdd-suite/templates/scaffold-project.sh` — header comment rewritten to remove the speculative future-feature mention.
- `vsdd-suite/suite-development/README.md` § Pure core / effectful shell — example of future-effectful-shell-expansion reworded to use a non-speculative example (additional hook scripts, an extended scaffold helper) rather than the upstream-feature reference.

**Historical narrative preserved as-is.** Reviews 40, 41, 42 entries in this same file; the Review 42 row in `SUITE-REVIEW-INDEX.md`; and the relevant rows in `GAP-ANALYSIS-LOG.md` (G-101, G-117, G-120) all contain the original `--with-suite` text. These are historical records of what was decided at the time. The corrections above are present-tense documentation fixes; the historical record is annotated by this Review 43 entry rather than rewritten.

**Resolution:** Six file edits applied. Confirmed clean via `grep -rn "with-suite" vsdd-suite/` after the edits (no remaining user-facing occurrences; only the historical-narrative occurrences in the gap registry and prior review-log entries, which are correctly preserved).

---

### New gap registered

**G-123 — Suite-development primer lacks a discipline check for external-dependency feature references.**

The Review 43 correction surfaces a class-level discipline gap, not just a one-off mistake. The suite-development primer at `suite-development/suite-development.md` § Governing standard for session primers does not name the discipline: "before referencing an external tool's feature (e.g., a crosslink command, a Claude Code subcommand, a language toolchain capability) in suite documentation, verify the reference against that tool's governing documentation; do not speculate about features that may be added later, and do not treat speculative features as 'coordination asks' unless the referencing party has the authority to file and own the request."

Without this check, future suite-development sessions are likely to recur the same pattern — an LLM-driven authoring session naturally extrapolates "the suite could integrate more deeply if X existed" into "X is coordination-asked," conflating speculative design with committed plan. The corrections to Reviews 40–42 are the specific instance; the primer addition is the class-level fix.

**Resolution sketch:** Add a `### External dependency references` subsection to `suite-development/suite-development.md` § Governing standard for session primers (or as a top-level discipline note under the prompt section). Brief — three or four sentences naming the check and the failure mode. Coordinate with G-118 (crosslink CLI contract) — that file is now the suite's canonical record of the verified crosslink dependency surface; any speculation about additional crosslink features must update that file with explicit verification or not be referenced at all.

**Classification:** Open. **Coordinate:** G-118 (crosslink contract — the dependency surface verification anchor).

---

### Coordination

The correction does not affect the doctrine ratifications from Review 42 (G-117 still ratified manual copy as canonical default; G-121 still ratified the scaffold-default-7-cores as the starter-set doctrine). G-117's rationale stands — submodule and sibling-symlink remain accepted variants for explicit reasons; the `--with-suite` row simply did not belong in the table because it referenced a non-existent feature.

The Review 40 + 41 + 42 + 43 onboarding-experience arc closes at 22 of 24 gaps addressed (the 21 from before plus G-123 just registered, with 2 still Open from the prior arc plus G-123 as Open this round = 3 Open; 21 Addressed). Remaining Open:

- **G-106** — sample crosslink command outputs (pending reference implementation)
- **G-112** — end-to-end reference implementation (in progress — `bookmark-cli/` scaffolding started before this correction interrupted)
- **G-123** — suite-development primer external-dependency check (resolution sketch above; one-paragraph addition to the primer)

G-123 is the next natural fix (small, primer-text-only). Reference implementation work resumes after.

---

## Review 42 — 2026-05-17 03:01Z

**Scope:** Solution Owner doctrine ratification of the two architectural-decision gaps left Open from Review 41 (G-117 suite-to-project coupling mechanism, G-121 complexity-budget / starter-set definition). Read `vsdd-suite/domains/role/SOLUTION-OWNER-REVIEW.md`; applied SO dimensions to each open question; ratified the doctrine in-session and applied the resulting documentation changes.

**Lens:** Solution Owner with explicit doctrine-ratification posture. Two angles: (a) SO Dim 4 (over-engineering) test — what is the minimum-viable mechanism that satisfies the requirement?; (b) SA Dim 9 sycophancy-check transferred — "would a human engineer working alone on a portfolio of 3 projects have authored this?" applied to each candidate option.

**Session note:** In-session. Sycophancy compensation: I am the same operator that proposed the candidates in Review 41 and authored the supporting documentation in Reviews 38–41. SO discipline required pushing back against my own framing — I had noted in Review 41's SA-1 writeup a lean toward "submodule is the cleanest architectural answer," which SO Dim 4 + SA sycophancy-check rejected as team-scale-default. Documented the temptation-and-rejection at each decision point so future reviewers can verify the framing wasn't softened.

---

### Resolved

**G-117 — Suite-to-project coupling mechanism ratified: manual copy via `scaffold-project.sh` is the canonical default.**

The other three mechanisms (git submodule, sibling symlink, `crosslink init --with-suite`) are preserved as accepted-variant options for projects with explicit reasons, but the suite's recommended path is the scaffold script. Rationale anchored to SO Dim 3 (technology compliance — manual copy matches the suite's existing markdown + bash tech surface; submodule introduces git submodule semantics; `--with-suite` requires upstream crosslink work; symlink is filesystem-only), SO Dim 4 (over-engineering — submodule and `--with-suite` are justifiable for team-scale portfolios but disproportionate for personal-portfolio scale of ~3 projects), and SA Dim 9 sycophancy-check (a human engineer working alone would not author submodule infrastructure when `cp` + a 50-line bash script suffices).

The applied documentation change at `vsdd-suite/README.md` § Bringing the suite into your project: the prior 4-equal-options table was restructured to lead with "The canonical default: manual copy via `scaffold-project.sh`" and demote the other three to a labeled "Accepted-variant options" table. The `crosslink init --with-suite` future option is preserved as a coordination ask against crosslink upstream, explicitly noted as not-yet-available and not a suite-side responsibility to implement.

**Resolution:** Doctrine landed at `README.md` § Bringing the suite into your project. Status flipped Open → Resolved in [GAP-ANALYSIS-LOG.md](../GAP-ANALYSIS-LOG.md).

---

**G-121 — Complexity-budget / starter-set ratified: the scaffold script's default-7-cores IS the starter-set doctrine; one-sentence affordance documents it.**

No separate "starter set" concept introduced — that would have been over-engineering on top of an existing answer. The scaffold script already defaults to the 7 core domains (SE, QE, UX, Security, SA, SO, VDD-IAR Alignment); DOMAIN-INDEX.md already names the activation criteria for extended domains. The closure was a documentation framing fix: explicitly state in the README's Quickstart Phase-3 step and in the Domains section opener that "default activation is 7 cores; extended domains activate per DOMAIN-INDEX.md conditions; for typical portfolio projects this is 7–9 active domains per layer." The 16-domain surface is *available*, not *required*.

Rationale anchored to SO Dim 1 (spec coverage — the implicit default needed to be made explicit), SO Dim 4 (over-engineering — proposing a separate two-mode doctrine on top of the existing scaffold default is itself team-scale-default machinery), and SO Dim 9 (assignment compliance — apprentices learning the suite benefit from a documented floor and growth path rather than a 16-domain overwhelm at first impression).

Self-sycophancy check applied at the decision point: I had initially leaned toward "(a) define starter set + (b) 'you don't have to use it all' affordance — together with elaborate framing" in my Review 41 SA-5 writeup. SO Dim 4 rejected the elaborate framing. The single-sentence affordance is sufficient because the structural mechanism (scaffold script default) already encodes the doctrine; the documentation just needed to make it visible.

**Resolution:** Two documentation additions: one sentence in `README.md` § Quickstart Phase-3 step; one paragraph in `README.md` § Domains section opener. Status flipped Open → Resolved in [GAP-ANALYSIS-LOG.md](../GAP-ANALYSIS-LOG.md).

---

### Coordination

The Architectural-doctrine cluster from Review 41 (G-117 + G-120 + G-121) is now fully closed: G-117 and G-121 ratified in Review 42; G-120 (versioning strategy) was addressed in Review 41 via `COMPATIBILITY.md` + retroactive version anchors. The Dogfooding cluster from Review 41 (G-118 + G-119 + G-122) was likewise fully closed in Review 41.

The full Review 40 + 41 onboarding-experience finding set has now reached the following terminal state:

- **Addressed (21):** G-100, G-101, G-102, G-103, G-104, G-105, G-107, G-108, G-109, G-110, G-111, G-113, G-114, G-115, G-116, G-117, G-118, G-119, G-120, G-121, G-122.
- **Open (2):** G-106 (sample crosslink command outputs) and G-112 (end-to-end reference implementation). Both deferred pending the reference-implementation work — G-106 needs real command outputs from a running reference; G-112 is multi-session work that produces those outputs and serves as the contract-testing canary for G-118.

The onboarding-experience review arc (Reviews 40, 41, 42) is at refinement-signal exhaustion for everything addressable within this session's scope. The 2 remaining Open gaps require either: (a) a separate session to build the reference implementation, or (b) registration of the work against a project that will naturally exercise the worked example end-to-end (the next portfolio project to start with the new vsdd-suite directory).

No new gaps surfaced by Review 42 — the SO doctrine ratification did not reveal additional scope-creep or compliance failures beyond the two gaps it was scoped to ratify. The sycophancy self-audit confirms the doctrine choices are minimum-viable rather than team-scale-defaulted: I caught and rejected the submodule-bias in G-117 and the elaborate-framing-bias in G-121.

---

## Review 41 — 2026-05-17 02:53Z

**Scope:** Solution Architect lens applied to the suite-as-system, with onboarding-relevance as the prioritizing filter. Read `vsdd-suite/domains/role/SOLUTION-ARCHITECT-REVIEW.md` (Dim 1–12 standard, Extended 13–22 External Interface Contracts, Extended 23–27 External Service Integration); held the suite's own architecture, dependency surface, and version policy against those dimensions. Cross-referenced with the Review 40 finding set (G-100 through G-116, plus the synthesis findings TW-1/TW-3) to identify SA-specific findings rather than re-articulating findings already filed under other lenses.

**Lens:** Solution Architect, with explicit attention to three angles: (a) architectural decisions about how the suite is consumed; (b) **dogfooding** — does the suite apply its own External Interface Contracts and External Service Integration discipline to its own crosslink and AI-tool dependencies; (c) complexity-budget proportionality for a single-maintainer portfolio per SA Dim 9's sycophancy-check focus.

**Session note:** In-session. Sycophancy compensation: the SA Dim 9 finding (SA-5 / G-121) was the one I had to push hardest against my own temptation to validate — agents default to "more discipline is better" framing, and "would a single developer working alone author 16 review domains for a portfolio of 3 CLI projects?" is the opposing question the primer's sycophancy check explicitly directs the reviewer to apply. Documented the temptation-and-rejection in the finding's resolution sketch so future reviewers can verify the framing wasn't softened.

---

### New gap registered

This review registers six new gaps (G-117 through G-122). Each cross-references the prior round's findings where relevant; SA's distinctive contribution is reframing architectural decisions (SA-1) and dogfooding gaps (SA-2, SA-3, SA-6) that the defect-class + TW + UX lenses partially missed.

| G-ID | Title | SA dim(s) | Severity |
|---|---|---|---|
| G-117 | Suite-to-project coupling mechanism: architectural decision unmade | Dim 1 + 2 + 8 | High |
| G-118 | Crosslink CLI contract undeclared and undefended (dogfooding gap on Ext. Interface Contracts dims 13–18) | Dim 4 + Ext 13–18 | High |
| G-119 | AI-tool dependency inventory absent; Privacy/Security dogfooding gap on Ext. Service Integration dims 23, 27 | Ext 23 + 27 | High |
| G-120 | Suite versioning strategy absent; G-94 sub-issue 4 promoted from spinoff-MVP to current-need | Ext 14 + 15 + 16 + 20 | High |
| G-121 | Complexity budget for personal-portfolio scale: 16-domain default may be AI-defaulted team-scale | Dim 9 + sycophancy-check focus | High |
| G-122 | Suite's own VSDD purity boundary undocumented (dogfooding gap on Dim 12) | Dim 12 | Low |

---

### Coordination

The six gaps cluster into two coordinated decisions:

**Cluster A — Architectural-doctrine gaps requiring SO ratification (G-117, G-120, G-121).** Three gaps share the property that SA can propose but SO is the spec-authority for the doctrine choice. G-117 names the consumption-mechanism choice; G-120 names the versioning strategy that makes G-117's submodule and `--with-suite` options operable; G-121 names the complexity-budget question (full surface vs. starter-set default). All three are filed Raised-to-SO-equivalent (the suite's analogue of the project-level Raised-to-SO classification — SA proposes, SO decides). Recommended sequence: G-117 first (the foundational choice), then G-120 (versioning falls out of the coupling choice), then G-121 (the starter-set definition is independent but its doctrine touches the others).

**Cluster B — Dogfooding gaps where the suite teaches a dimension it does not apply to itself (G-118, G-119, G-122).** The suite contains External Interface Contracts dims 13–22 (taught by SA Extended Section 1), External Service Integration dims 23–27 (taught by SA Extended Section 2), and the purity-boundary map (taught by SA Dim 12). The suite does not apply these dimensions to its own crosslink dependency (G-118), AI-tool dependency (G-119), or pure-vs-effectful structure (G-122). Closure shape: each gap gets a small addition (crosslink-contract.md / AI-tool data-flow paragraph in Prerequisites / purity-boundary paragraph in suite-development/README.md). Coordinate with Privacy (G-119 specifically activates Privacy dim 6) and Platform Engineer (G-118's CLI dependency surface).

**Cross-coordinate with Review 40 findings:**
- SA-1 (G-117) extends G-101 (mechanism-options documentation): G-101 listed options; SA-1 demands a choice. Both can be Addressed when the choice is documented.
- SA-2 (G-118) extends G-111 (version-pinning): G-111 added a Tested-Against line; SA-2 demands a full contract-management posture.
- SA-3 (G-119) extends G-100 (Prerequisites listing): G-100 named AI tools; SA-3 demands data-flow + privacy posture.
- SA-4 (G-120) extends G-94 sub-issue 4: G-94 deferred versioning to spinoff-MVP; SA-4 promotes it to current-need.

Recommended next review (when warranted): Solution Owner, to ratify the doctrine choices in Cluster A. Until then, the gaps remain Open as proposed-not-ratified.

---

## Review 40 — 2026-05-17 02:39Z

**Scope:** Onboarding experience for a new user using the suite on a crosslink-enabled project. Read `vsdd-suite/README.md` start-to-end as a new user would; read `vsdd-suite/primers/3-review-session.md` and `vsdd-suite/suite-development/suite-development.md` as the governing primers; read `vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md`, `vsdd-suite/domains/role/UX-REVIEW.md`, and `vsdd-suite/supplements/cli.md` as the additional adversarial lenses applied. Triggered by driver request following the post-restructure work in Reviews 38 + 39 — the suite's outward identity has just changed, and the onboarding surface deserves adversarial pressure before the next project lands on it.

**Lens:** Multi-domain adversarial review of onboarding documentation. Three coordinated lenses applied serially: (a) **defect-class lens** — trace the new-user journey from README to first-Phase-3-review-filed, name every friction point; (b) **Technical Writer domain** — apply TW Dims 1–10 against the suite's documentation surface as the primary deliverable, with explicit attention to the clone-and-follow test (Dim 1) and the knowledge-transfer test (Dim 9); (c) **UX + CLI-supplement domain** — apply UX standard dims and CLI supplement dims to the suite-as-interface, treating the documentation-plus-command-sequence as a hybrid interface that has discoverability, scannability, empty-state, and interruption-handling concerns.

**Session note:** In-session — this entry is authored by the operator that landed Reviews 38 and 39 in the same session. Sycophancy compensation: every finding is anchored to a specific file:line citation or an explicit prose-absence claim, not to narrative judgment. The TW and UX lens applications were deliberately structured to surface findings *distinct* from the defect-class lens, not duplicates — a coordination column is present in the round summary for each finding showing where it cross-references the prior findings. Two specific sycophancy temptations were caught and rejected mid-review: (1) "the worked example was just reframed in Review 39 — give it credit" (rejected: the suite-vs-crosslink positioning was the reframed issue; new-user executability is independent); (2) "UX is mostly N/A for a documentation interface" (rejected by forcing the dimension list through end-to-end with the CLI-supplement reframe; surfaced UX-1 through UX-4 as substantive).

---

### New gap registered

This review registers seventeen new gaps in `GAP-ANALYSIS-LOG.md` (G-100 through G-116), each addressing a distinct documentation, scaffolding, or framing defect surfaced by one of the three lenses. Per the suite-development convention, the table below is the gap-registry pointer; the prose body of each gap lives in the registry row itself. Defect-class-lens findings F1–F12 from the prior round of this conversation are consolidated into G-100–G-110; TW lens findings TW-2 and TW-4 register as G-111 and G-112; UX/CLI lens findings UX-1 through UX-4 register as G-113 through G-116. TW-1 and TW-3 are synthesis findings (compounded effects of other gaps) and are filed as coordination notes within G-100 and the registry's reactivation-triggers section rather than as standalone gaps. Findings that strictly coordinate with existing closures (e.g., the version-pinning question relates to G-94 sub-issue 4 on release tagging) are noted in the coordination block below.

| G-ID | Title | Primary lens | Severity |
|---|---|---|---|
| G-100 | Prerequisites and tool setup undocumented (crosslink, AI tool, language toolchain) | Defect-class + TW Dim 1 | Critical |
| G-101 | Mechanism for bringing `vsdd-suite/` into a new project tree is unspecified | Defect-class | Critical |
| G-102 | No scaffolding templates for per-domain index files or per-project DESIGN/README skeletons | Defect-class + TW Dim 9 | High |
| G-103 | Worked example is Rust-specific; supplement integration not surfaced for JS/TS/Python users | Defect-class | High |
| G-104 | README information architecture front-loads reference over walkthrough; no Quickstart | Defect-class + UX Dim 1 | High |
| G-105 | Domain activation deduction asserted not shown in worked example (PE/DE-not-active without reasoning) | Defect-class | High |
| G-106 | Crosslink command outputs not shown with sample output; new user has no mental model | Defect-class + CLI Dim 6 | Medium |
| G-107 | Project README authoring not in worked example (artifact convention by example only) | Defect-class + TW Dim 1 | Medium |
| G-108 | `suite-development.md` lead text still says "the IAR suite" after the G-88 rename to VSDD Suite | TW Dim 2 | Medium |
| G-109 | No "skip these for now" guidance for VSDD Phases 5 and 6 in the pipeline-context table | Defect-class | Medium |
| G-110 | Domain slug convention buried in `suite-development.md`; new user filing first Phase 3 review must cross-file-lookup | Defect-class + CLI Dim 1 | Low |
| G-111 | Crosslink CLI commands in worked example not version-pinned; documentation-accuracy regression risk | TW Dim 2 + TW sycophancy check | Medium |
| G-112 | No end-to-end reference implementation of the worked example; the walkthrough is hypothetical | TW Dim 2 + Dim 10 | High |
| G-113 | "Where each primer fits in the flow" summary table at the bottom of Worked Example, not the top | UX Dim 1 + CLI Dim 3 | Medium |
| G-114 | Zero-findings end-state for Phase 3 surfaced only in contributor primer, not in user-facing README | UX Dim 6 + CLI Dim 6 | Medium |
| G-115 | No interruption/resumption guidance for in-progress Phase 3 reviews (multi-day reality) | CLI Dim 9 | Low |
| G-116 | No "new user, start here" affordance on README entry point; readers default to expensive top-to-bottom path | UX Dim 1 + Dim 5 | High |

**Synthesis findings (not standalone gaps):**

- **TW-3 — Knowledge-transfer test failure (synthesis).** The TW domain's Dim 9 test ("could a developer who has never seen this project file their first Phase 3 QE review in one day using only the documentation?") fails because of compounded blockers from G-101 + G-102 + G-103 + G-110 + G-116. The test itself is the resolution gate; the suite cannot mark this Addressed without empirical evidence the test passes. Recorded as a coordination-check artifact, not a separately tracked gap.

- **TW-1 — Clone-and-follow test (synthesis).** The TW domain's Dim 1 bright-line test ("clone the repo into a fresh environment and follow the README; if any step fails, the README is incomplete") fails because of G-100 + G-101 + G-103 + G-111. Same synthesis structure as TW-3; recorded as a coordination check.

---

### Coordination

The seventeen new gaps cluster into three coordinated decisions:

**Cluster A — Entry-point repair (G-100, G-104, G-116).** Three gaps share a single resolution: a Prerequisites + Quickstart + "New here? Start at…" affordance block at the top of `README.md`, before the existing reference tables. One coordinated edit closes all three. **Recommended sequence:** address Cluster A first; it unblocks every other documentation finding by giving the new user a single signposted path.

**Cluster B — Suite-to-project scaffolding (G-101, G-102, G-107).** Three gaps share the question "how does a project go from `crosslink init` to a working `vsdd-suite/` directory with the right templates filled in?" Resolution shape: a "Bringing the suite into your project" section in README answering G-101's mechanism question; a `vsdd-suite/templates/` directory containing stub index files (one per domain) + DESIGN.md skeleton + project-README skeleton; the README's review-logs section points at the templates as the starting-point artifact. The longer-term move is the coordination ask to crosslink for `crosslink init --with-suite`, which would mechanize the entire cluster.

**Cluster C — Worked-example fidelity (G-103, G-105, G-106, G-111, G-112, G-113).** Six gaps share the concern "the worked example asserts behavior the suite cannot itself verify." Resolution shape: language-agnostic framing markers; inline activation-deduction reasoning; sample command outputs where opaque; a "Tested against crosslink v0.8.0 on 2026-05-17" line at the top of the section; the summary table moved to the top; **and**, the largest-leverage item, an end-to-end reference implementation (G-112) at e.g. `bookmark-cli/` that turns the worked example from hypothesis to canary.

**Standalone:** G-108 (prose IAR → VSDD update), G-109 (skip-Phase-5/6 note), G-110 (slug table in README), G-114 (zero-findings paragraph), G-115 (pausing/resuming paragraph). Each is a one-paragraph or one-line fix with no cross-coordination.

**Cross-coordinate:** Solution Architect domain review recommended next (per the prior round's refinement-signal-not-yet-exhausted note) — SA would surface findings around G-101's structural decision (suite-as-submodule vs. suite-as-clone vs. suite-as-installed) and G-111's external-interface-contract framing of the suite's crosslink dependency. Driver has confirmed SA is the next review to run.

No suite primer or domain prompt is structurally changed by Review 40 itself — every finding is documentation/scaffolding artifact debt rather than methodology gap. The TW Dim 1 / Dim 9 framing and the UX/CLI Dim 1/3/6/9 framing surface findings the defect-class lens partially missed; the multi-lens application is the methodological value of running TW + UX as a complement to a generalist defect-class lens.

---

## Review 39 — 2026-05-17 02:16Z

**Scope:** G-89 closure (project-level domain review-log structural standard) plus a worked-example reframing in `README.md` driven by driver feedback that the existing prose positioned crosslink as the operational shell, displacing the suite's own primary role. Both changes are documentation/standard updates; no project review logs or other domain artifacts are restructured by this session (forward-only constraint preserved).

Read: `vsdd-suite/suite-development/suite-development.md` (full Governing-standard-for-project-level-review-logs section, lines 98–192), `vsdd-suite/README.md` (full Worked-example section and Review-logs section), the bundled-trigger paragraph in `suite-development/GAP-ANALYSIS-LOG.md`, the existing suite-review precedent (`SUITE-REVIEW-INDEX.md` + `review-log/` shape) as the architectural template G-89 mirrors. Loaded `suite-development.md` as the governing session primer.

**Lens:** Structural-standard update + driver-feedback reframing. Two coordinated changes in one session because both touch the documentation surface that frames "how does a project use the suite": G-89 changes how project teams structure their per-domain review logs going forward; the worked-example reframe changes how project teams read the suite-vs-crosslink relationship in the README walkthrough.

**Session note:** In-session — this entry is authored by the operator that made the changes. Sycophancy compensation: G-89's closure claim is anchored to verifiable artifact changes (a new sub-section exists at the named location in `suite-development.md`; `README.md` § Review logs has the updated tree shape and forward-only paragraph; the gap-registry row references this session as the closure event with the structural-decision rationale embedded in the row). The worked-example reframe is anchored to a side-by-side narrative test: in the previous text, every phase opened with a `crosslink` command and the suite primer appeared as a parenthetical loading instruction; in the new text, every phase opens with the primer's prescription and the `[+crosslink]` block appears as an optional amplifier under it. Both narrative shapes are observable in the file; the reframe claim is verifiable by reading the section start-to-end.

---

### Resolved

**G-89 — Project-level domain review-log structure standardized on the per-domain index + per-session file pattern.**

Forward-only carve-out preserved: completed projects (`bookmark-manager/iterative-adversarial-refinement/`, `issue-tracker-cli/iterative-adversarial-refinement/`) retain their existing single-file-per-domain structure; the new index + session pattern applies only to projects whose first IAR run is filed on or after 2026-05-17.

Substantive changes:

1. **`vsdd-suite/suite-development/suite-development.md`** § Governing standard for project-level review logs gained a new `### Structure (per-domain index + per-session entries)` sub-section at the top. The sub-section names the forward-only constraint up front; defines the two file shapes (per-domain index file at `<project>/vsdd-suite/<DOMAIN>-REVIEW.md` holding file-level header + Reviews table; per-session entry file at `<project>/vsdd-suite/review-log/YYYY-MM-DD-<domain-slug>.md` holding round narratives); enumerates the domain-slug convention for all 16 active core/extended/meta domains (lowercase, hyphenated role name, no `-review` suffix because the parent dir conveys it); states the cross-domain reference convention (link directly to the session-file anchor, not through the index); names the rationale (faster cross-domain reading; cleaner scoped-search; multi-round closure trails visible at the index level; mirrors the suite's own SUITE-REVIEW-INDEX.md + review-log/ pattern which has been load-tested with 38+ sessions).

2. The existing `### File-level header` sub-section was reframed for the new shape — the header content now belongs at the top of the per-domain index file (was top of accumulating file). A new `### Per-session file header` sub-section was added describing the session-file H1 + optional index-back-link convention.

3. **`vsdd-suite/README.md`** § Review logs updated with the new tree shape showing the per-domain index files alongside a `review-log/` directory containing per-session files. The forward-only paragraph names the two completed projects whose existing structure is preserved. The reference to the governing standard now links to the relevant sub-sections in `suite-development.md` (Structure / File-level header / Per-session file header) rather than treating it as a single flat standard.

What was NOT changed: no existing project review log was restructured (forward-only); the per-review entry preamble, finding sections, finding body, closing block, and round numbering sub-sections of the governing standard were left unchanged (those describe the SHAPE of a single review entry, which is the same in either file structure — only the FILE shape changed); the suite-review log structure itself was untouched (it already follows the index + session pattern that G-89 borrows for projects).

**Resolution:** Status flipped Deferred → Addressed in [GAP-ANALYSIS-LOG.md](../GAP-ANALYSIS-LOG.md). Last Reviewed 2026-05-17. The "after `issue-tracker-cli` completes" bundled trigger that originally gated G-88, G-89, G-90, G-91, G-92, G-93, G-94, G-95 is now fully closed (G-89 was the final gap remaining against it).

---

**Finding 1 — Worked-example section in README.md framed crosslink as the operational shell rather than the suite as the spine.**

Driver feedback: the prior text of `vsdd-suite/README.md` § Worked example: A VSDD session with crosslink opened each phase with a crosslink command (e.g., "Open the spec authoring session in a fresh model context: `crosslink design "bookmark CLI..."`"), with the suite's primers appearing as a parenthetical loading instruction inside the crosslink-managed session. The `[manual fallback]` tag at the end of each phase reinforced the inverted framing — labeling the suite-primer-only path as a fallback rather than as the primary path the suite was designed for. The opening paragraph stated "using the suite primers and crosslink as the operational shell", which puts crosslink in the operational-shell role and the primers as content loaded into it.

Verifiable artifact-state evidence of the framing inversion: every "### Phase N" sub-section in the prior text led with a code block of `crosslink` commands; the primer file was named only after the code block, in prose like "Work through the driving questions in `primers/1a-spec-crystallization.md`."

**Resolution:** Reframed the section so each phase opens with the primer's prescription ("Per `primers/1a-spec-crystallization.md`: write `DESIGN.md` against the primer's driving questions..."), shows the manual path as the primary instruction, then adds an `[+crosslink]` block as an optional amplifier showing how the tracker mechanizes what the primer prescribes. Section header retained per driver instruction ("The ## Worked example: A VSDD session with crosslink header is good"). Lead paragraph rewritten: "running the suite's primers, with optional crosslink integration shown as an enhancement layer at each phase. The primers are the spine of the work — every step works manually, and the manual path is sufficient on its own. Crosslink commands appear under each phase as `[+crosslink]` blocks showing how a project tracker can mechanize what the primer prescribes; if you do not use crosslink, ignore those blocks and the rest is a complete walkthrough." The `[manual fallback]` tags were removed throughout because the manual path is no longer a fallback — it is the primary path. The Where-each-primer-fits table at the end gained a "What the primer prescribes" column, with the crosslink-command column renamed to "Optional crosslink amplifier" to match.

---

### Coordination

G-89's closure completes the post-ITC bundled trigger. No further gaps in the original bundle remain Deferred. Future suite reviews may register new gaps against new trigger shapes, but the specific "after `issue-tracker-cli` completes" trigger is retired (preserved in `GAP-ANALYSIS-LOG.md` § Reactivation triggers for reference in case a future bundle reuses the trigger pattern).

The worked-example reframing is a documentation change with no methodology consequences — it adjusts how the README narrates the suite-vs-crosslink relationship without changing what either does or what the primers prescribe. Filed as Finding 1 rather than as a registered gap because the inverted framing was not a tracked Open gap; the driver caught it in narrative review and the fix landed in the same session.

The forward-only constraint preserved by G-89 means the two existing portfolio projects' domain review logs remain navigable in their current shape; readers of those logs do not need to learn the new structure to follow them. The new structure becomes mandatory only for the next project that opens a vsdd-suite/ directory for the first time.

---

## Review 38 — 2026-05-17 02:00Z

**Scope:** Post-ITC-completion restructure pass closing the four-gap structural cluster (G-88, G-91, G-92, G-93) plus the in-cluster bundle of G-95 (split `prompts/implementation.md` into separate Phase 2a and Phase 2b primers). All five gaps had been Deferred against the bundled `"after issue-tracker-cli completes"` trigger; ITC reached the trigger conditions per the closure protocol (Layer 7 ✅ Complete at `eca5b25`; final-merge VDD-IAR Alignment ratification GO at `a7bdc64`; project archived per the portfolio convention).

Read: `vsdd-suite/README.md` (now), `vsdd-suite/CHANGELOG.md`, `vsdd-suite/suite-development/SUITE-REVIEW-INDEX.md`, `vsdd-suite/suite-development/GAP-ANALYSIS-LOG.md`, `vsdd-suite/suite-development/suite-development.md`, all primers in `vsdd-suite/primers/`, all role and meta domains, all supplements, `vsdd-suite/hooks/check-review-log-anonymization.sh`, root `.pre-commit-config.yaml`, `.github/PULL_REQUEST_TEMPLATE.md`. Loaded `prompts/suite-development.md` (now `vsdd-suite/suite-development/suite-development.md`) as the governing session primer.

**Lens:** Structural-restructure pass — five coordinated artifact moves to settle the suite's outward identity now that ITC is done. Not adversarial review; this is the suite-development equivalent of a refactor commit, where the lens is "does the new structure honor each gap's resolution sketch and the forward-only constraint."

**Session note:** In-session — this entry is authored by the same operator that executed the moves and the cross-reference rewrite pass. Sycophancy compensation: each closure is anchored to a verifiable artifact-state claim (directory exists at new path; cross-references resolve; hook regex covers both old-path and new-path forms; G-88's forward-only constraint demonstrably honored by leaving `issue-tracker-cli/iterative-adversarial-refinement/` and `bookmark-manager/iterative-adversarial-refinement/` untouched). The closure claims here are reproducible against the current working tree.

---

### Resolved

**G-88 — Suite directory rename from `iterative-adversarial-refinement/` to `vsdd-suite/`.**

Driver chose `vsdd-suite/` after candidate evaluation (`vsdd-suite/` anchors to the methodology whitepaper, is phase-neutral, is shorter than alternatives, and reads naturally as a future standalone-repo name). Executed via `git mv iterative-adversarial-refinement vsdd-suite` (single rename preserves git history for every tracked file under the tree). The H1 in `vsdd-suite/README.md` was retitled `# VSDD Suite` to match; the lead paragraph reframes the suite as a multi-phase prompt and process library (the prior framing as "the IAR adversarial review suite that grew" is no longer accurate now that the directory matches the actual scope).

Forward-only constraint honored: `issue-tracker-cli/iterative-adversarial-refinement/` and `bookmark-manager/iterative-adversarial-refinement/` are untouched. The cross-reference rewriter used a negative-lookbehind regex to skip any `iterative-adversarial-refinement/` occurrence preceded by `issue-tracker-cli/` or `bookmark-manager/`. The `.pre-commit-config.yaml` hook regex was updated to cover all three reachable shapes (`^(vsdd-suite/.*\.md|.*/iterative-adversarial-refinement/.*\.md|.*/vsdd-suite/.*\.md)$`) so review-log anonymization continues to fire on completed projects' IAR markdown without requiring those projects to rename.

**Resolution:** Status flipped Deferred → Addressed in [GAP-ANALYSIS-LOG.md](../GAP-ANALYSIS-LOG.md). Last Reviewed 2026-05-17.

---

**G-91 — `prompts/` folder renamed to `primers/` and primer files prefixed with VSDD phase.**

Executed: `git mv vsdd-suite/prompts vsdd-suite/primers`, then per-file renames:
- `spec-crystallization.md` → `1a-spec-crystallization.md`
- `decomposition.md` → `1b-decomposition.md`
- `implementation.md` → split into `2a-red-gate.md` + `2b-implementation.md` (see G-95 below)
- `review-session.md` → `3-review-session.md`
- `feedback-integration.md` → `4-feedback-integration.md`
- `suite-development.md` → moved into `vsdd-suite/suite-development/suite-development.md` (see G-92 below) — not phase-prefixed because it is not a VSDD-phase primer

Naming convention chosen: bare phase digit prefix (`1a-`, `2b-`, `3-`), no `phase-` prefix. Shorter; sorts cleanly under filesystem ordering; the suite-development primer (without phase prefix) sorts naturally after all phase primers (lexically `1a-` < `1b-` < `2a-` < `2b-` < `3-` < `4-`; the suite-dev primer lives in a different directory anyway, so collision is moot).

Cross-references throughout the suite were updated by a Python rewriter that scoped substitutions to suite-internal files plus the two repo-root files (`.pre-commit-config.yaml`, `.github/PULL_REQUEST_TEMPLATE.md`). One round of false-positive double-prefixing (`2b-2b-implementation.md` and similar in the two new 2a/2b primer files and in one historical Review 31 entry that pre-discussed the phase-prefix convention) was swept clean with a follow-up sed pass before any commit. A subsequent round of corruption to GAP-ANALYSIS-LOG, SUITE-REVIEW-INDEX, and the prior review-log session files (where the rewriter overwrote *narrative* mentions of historical paths, not just link forms) was caught and reverted via `git restore` — historical narrative now reads correctly, and link forms in those files were sibling-relative so no link breakage resulted.

**Resolution:** Status flipped Deferred → Addressed in [GAP-ANALYSIS-LOG.md](../GAP-ANALYSIS-LOG.md). Last Reviewed 2026-05-17.

---

**G-92 — Suite-meta-development materials moved into `suite-development/` subfolder.**

Created `vsdd-suite/suite-development/` and moved into it:
- `suite-development.md` (from `prompts/suite-development.md` — the contributor session primer)
- `SUITE-REVIEW-INDEX.md`
- `GAP-ANALYSIS-LOG.md`
- `review-log/` (entire directory, including all nine prior session files plus this Review 38 entry)

What stayed at suite root: `README.md`, `CHANGELOG.md`, `primers/`, `domains/`, `supplements/`, `hooks/`. CHANGELOG stayed top-level per convention (changelogs typically live at project root); `hooks/` stayed top-level because the suite hook is wired from the repo-root `.pre-commit-config.yaml` and structural relocation would force a config edit per change.

Relative paths from suite-development/ to suite-root content (README, CHANGELOG, primers, domains, supplements, hooks) all gained one extra `..` of depth; the rewriter handled this per-file based on source-file location class. Relative paths within suite-development/ (e.g., GAP-ANALYSIS-LOG.md referencing review-log/ siblings) remained unchanged — both sides moved together preserving the relative position.

**Resolution:** Status flipped Deferred → Addressed in [GAP-ANALYSIS-LOG.md](../GAP-ANALYSIS-LOG.md). Last Reviewed 2026-05-17.

---

**G-93 — README split into user-facing top-level + contributor-facing `suite-development/README.md`.**

Approach taken was the *minimal* variant of G-93 rather than the full content-migration variant (where governing-standards prose from `suite-development.md` would have moved into a dedicated handbook). The user-facing `vsdd-suite/README.md` retains its existing operational and pedagogical sections (Suite scope, VSDD pipeline context, Governing references, Domains, Session primers, Language and interface supplements, Worked example, Running IAR, Review logs, Merging gate) — adjusted only for: (1) H1 rename, (2) lead-paragraph reframing, (3) a new "For contributors evolving this suite" pointer near the top directing to `suite-development/README.md`, (4) primer table updates reflecting G-95's split, (5) the Worked example's Phase 2a code block pointing at the new `2a-red-gate.md` primer.

The new `vsdd-suite/suite-development/README.md` is a navigation document: introduces the four contributor artifacts (suite-development.md primer, SUITE-REVIEW-INDEX.md, GAP-ANALYSIS-LOG.md, review-log/), names the suite-development workflow at a step level, points at the reactivation-triggers mechanism, and documents the project-scoped → suite-level promotion doctrine. It is intentionally short — the long-form discipline lives in `suite-development.md` (the primer); the new README is the entry point that orients a contributor to the materials.

The deeper variant of G-93 — consolidating governing-standards prose currently inside `suite-development.md` into the contributor README or a dedicated handbook — is *not* taken in this pass. The current placement (primer carries the long-form governing standards, README is the navigation entry point) works structurally and avoids a content-migration pass whose value is unclear absent a contributor experience report. Trigger for revisiting: a future contributor reports that the primer-vs-README split is confusing or that they wanted the governing standards in a different shape.

**Resolution:** Status flipped Deferred → Addressed (minimal variant) in [GAP-ANALYSIS-LOG.md](../GAP-ANALYSIS-LOG.md). Last Reviewed 2026-05-17.

---

**G-95 — `prompts/implementation.md` split into `2a-red-gate.md` + `2b-implementation.md`.**

The original `implementation.md` had clear internal phase boundaries (the "## Phase 2a: Red Gate" and "## Phase 2b: Implementation" sections at lines 27 and 51 of the pre-split file). The split divided the file at that boundary with each new primer getting a phase-tailored Posture section, a phase-tailored Layer reference section, the phase-specific content, and a phase-tailored Completion criteria section. Key framing adjustments:

- `2a-red-gate.md`'s Completion criteria explicitly names the Phase 2a → Phase 2b handoff (the Red Gate commit hash as the verifiable boundary; no implementation logic written in this session).
- `2b-implementation.md`'s opening paragraph names the Red Gate commit as a session prerequisite (cannot start Phase 2b without it). The retroactive-Red-Gate label paragraph stays in `2b-implementation.md` — it describes a Phase 2b discovery condition. The Layer 7 R19 / G-99 warm-finding-closure discussion remains project-scoped at `issue-tracker-cli/iterative-adversarial-refinement/CLOSURE-PROTOCOL.md` §8 per the Review 37 deferred decision (no suite-level warm-closure section added here).
- The README's session-primers table gained two rows (one per primer) where it previously had one combined row; the VSDD-pipeline-context table at the top of the README now has the 2a row pointing at `2a-red-gate.md` (was pointing at `2b-implementation.md` because the combined primer covered both).

**Resolution:** Status flipped Deferred → Addressed in [GAP-ANALYSIS-LOG.md](../GAP-ANALYSIS-LOG.md). Last Reviewed 2026-05-17.

---

### Coordination

The five gaps were closed as one coordinated restructure pass per the driver's "cluster first" sequencing. The two remaining ITC-completion-gated deferred gaps — G-89 (standardize project-level domain review log structure) and G-90/G-94 (already addressed in Reviews 33/34) — sit outside this cluster. G-89 is independent and is the next gap to address per the agreed sequencing ("quick wins after the cluster"). G-99 (warm-finding-closure framework) remains Deferred against natural-recurrence trigger; the framework split into 2a/2b here does not affect its disposition (the warm-closure mode is orthogonal to the phase-boundary split — it concerns IAR Round 2+ closure shape, not the initial 2a→2b boundary).

Forward-only is honored in two senses: (1) completed projects' `iterative-adversarial-refinement/` subtrees are untouched, and (2) the historical narrative in this suite's review-log entries and gap registry descriptions preserves the path names that were current at the time of registration — only LINK paths were updated (and even those mostly didn't need updating because they were sibling-relative within suite-development/). A reader of G-88's row sees the original constraint text "completed projects retain their existing `iterative-adversarial-refinement/` review-log paths" intact, which is a true historical fact and accurately describes today's state of the two completed projects in the portfolio.

No suite primer or domain prompt was structurally changed beyond the file-rename and the 2a/2b split. The pedagogical content of the suite is unchanged; only its organization, naming, and entry-point shape evolved.
