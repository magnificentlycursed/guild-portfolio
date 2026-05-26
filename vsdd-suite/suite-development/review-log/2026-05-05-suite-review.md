<!-- hook-bypass[check-document-staleness]: pre-existing in-flight phrasing preserved per the forward-only narrative-preservation policy. This file's status claims predate the R95 F2 check-document-staleness hook; flagging would require retroactive rewriting that crosses the forward-only carve-out. Future status-claim edits SHOULD use current-state phrasing; the bypass-mechanism is itself a finding for the next registry-walk review. -->
# 2026-05-05 Suite Reviews

## Review 35 — 2026-05-05 20:30Z

**Scope:** `prompts/decomposition.md` — `### Manual testing checklist` and the `**Manual Testing Checklist:**` block in the `## TODO.md format` example. Triggered by user feedback after rendering the Layer 4 manual test plan for `issue-tracker-cli`: the produced format (numbered runnable steps with explicit commands, expected output, clean-state markers, and binary install/uninstall/reinstall lifecycle) sets a useful baseline that the existing primer guidance does not produce. The user requested this be captured as a standard so future test plans inherit the same shape.

**Lens:** Defect-class lens — **manual-testing-checklist authoring quality / tester-familiarity assumption.** The current primer produces shorthand bullets ("Happy path: create issue with `--label bug --label auth` → list shows `bug, auth`"). The bullet is meaningful to the developer who wrote the layer; it is not meaningful to a reviewer running the plan later, an AI agent in a new session, or an apprentice's reviewer running the test for the first time. The defect class: a checklist whose execution requires familiarity that the tester does not have produces test sessions that become debugging sessions, because the tester cannot tell whether they are verifying behavior or fighting environmental drift.

**Session note:** Same session as the user's Layer 4 manual-test-plan request and the rendered output that motivated the change. Not cold. Sycophancy compensation: the change reflects an artifact-level claim — the rendered Layer 4 plan exists in this conversation's transcript and is reproducible against the current source; the new primer text codifies properties of that rendered artifact (numbered steps, exact commands, expected outcomes, explicit clean-state, binary lifecycle), not subjective judgments about it. A cold session would evaluate the same artifact against the same properties and reach the same conclusion.

---

### Resolved

**Finding 1 — `prompts/decomposition.md` `### Manual testing checklist` produced shorthand bullets that assumed tester familiarity with the toolchain and the project's run procedure. Tightened to a runnable-step standard.**

The pre-change section listed five item types (happy path, error states, empty state, persistence check, edge cases) with parenthetical sub-prompts ("step by step", "what does the user see?"). The output was consistently terse single-line checklist entries. A tester running the plan needed to infer:

- where the binary lives (PATH, target dir, where to install from)
- whether the installed binary is the latest source (or is a stale `cargo install` from an earlier layer)
- which commands to actually run (the bullet describes intent, not invocation)
- what the expected output is (the bullet states the action, not the assertion)
- when to clean state between tests (the bullet does not say `rm -f tracker.json` between phases)
- what "persistence" means for this stack (just process restart? or uninstall + reinstall?)

These inferences are correct for the developer who wrote the layer. They are not available to anyone else. The Layer 4 manual-test-plan rendering for `issue-tracker-cli` exposed the gap concretely — the user's prior session had failed manual tests because their `~/.cargo/bin/tracker` was a stale install from before the layer's source changes, and the prior primer guidance did not require an "update the installed binary" step to be authored into the plan.

**Resolution:** Replaced `### Manual testing checklist` with a runnable-step standard. The new text requires four properties per step (exact command, expected outcome including stdout/stderr/exit code/on-disk state, explicit clean-state setup when required, binary lifecycle when relevant) and names the audience (a tester unfamiliar with the toolchain and the project). Required-items list retained, with the persistence check tightened to "install → create → uninstall → reinstall → verify data survives." A concrete "Example shape (one expanded step)" was inlined so the standard is anchored in a copyable form, not just abstract requirements.

The `**Manual Testing Checklist:**` block in the `## TODO.md format` example was updated to (a) name an explicit "Step 0 — Update the installed binary" item when the layer changes runtime behavior, (b) reframe the bullet list as placeholders that expand into runnable step blocks rather than as the final form, and (c) point at `### Manual testing checklist` for the standard.

Forward-only: existing `TODO.md` files in projects under review are not retroactively rewritten. New layer plans (and re-decomposed layers) inherit the new standard.

---

### New gap registered

**G-97 — Manual testing checklist format produced tester-familiarity-dependent items.**

Registered in [GAP-ANALYSIS-LOG.md](../FINDINGS-INDEX.md). Distinct from G-42 ("Manual testing checklists not owned by any domain", Addressed 2026-04-26) which addressed *which domain evaluates* manual testing checklist completion. G-97 addresses *the format the checklist itself takes when produced by decomposition*. Status: Addressed in-session by Finding 1.

**Classification:** New gap, immediately Addressed. The fix and the registry entry land in the same session, per the suite-development convention "If it was not tracked, add it and immediately mark it Addressed."

---

### Coordination

This change affects `prompts/decomposition.md` only. `prompts/implementation.md` references "the manual testing checklist from the layer plan" once and does not need an update — the implementation primer hands off to the human for verification using whatever checklist decomposition produced; richer checklists from the new standard flow through naturally. `prompts/spec-crystallization.md` references "manual testing requirements" at the verification-architecture level and is correct as-is — it specifies that the spec must classify which behaviors require manual testing, not the format of the resulting checklist.

No domain prompt files require updates. VDD-IAR Alignment dim 9 ("Manual testing checklists") evaluates whether checklists exist and were completed, not their format quality; the format-quality evaluation belongs implicitly to whichever reviewer reads the project's TODO.md (Solution Owner via dim 1 spec coverage; Technical Writer via documentation accuracy). Neither needs a new dimension — the standard now lives in the primer that produces the artifact.

---

**Finding 2 — Expected outcomes were specified as prose, not literal output blocks (added 2026-05-05).**

User executing the Layer 4 manual-test plan (rendered against the freshly-updated standard from Finding 1) reported `tracker --help` output that did not include the `--label` flag, asking why. Investigation: the rendered Step 0 read `tracker --help # expect: --label flag listed under create + list` — but `tracker --help` (top-level) only lists subcommand names, not their flags. The `--label` flag is on `tracker create --help` and `tracker list --help`. The plan named the wrong help command, and the prose hint ("expect: --label flag listed under create + list") was ambiguous enough that the error went undetected at authoring time.

The defect is a refinement of Finding 1: the new standard required "expected outcomes" but did not specify the form. Prose descriptions ("expect the flag to be listed") are easy to write and easy to misalign with the actual output — the author and the tester can reach different conclusions about whether the test passed. Literal output blocks force the author to obtain the actual bytes (eliminating "I expect this is what it shows" guesses) and let the tester perform a literal comparison.

Two specific failures of prose-form expected outcomes:

1. **Stream ambiguity.** A line that says "expect: usage error on stderr" leaves the tester unsure whether to consult `2>&1` redirection, look for an exit code, or both. A literal stderr block plus a tagged exit-code block resolves the ambiguity.
2. **Wrong-command silent passes.** "Expect: --label flag listed" passes against `tracker --help` (which does not list `--label`) if the tester reads "Personal issue tracker" + the commands list and concludes "looks fine, the flag is in there somewhere." A literal expected block of the help output forces the right command.

**Resolution:** Tightened `prompts/decomposition.md` requirement #2 to specify literal expected-output blocks for invariant output, with prose descriptions reserved for variable output (timestamps, IDs, OS-chosen paths) anchored to a representative literal example. Added a sub-clause to requirement #4 about help-command specificity (`<binary> <subcommand> --help`, not `<binary> --help`, when help text is part of verification — top-level help typically lists subcommands only, not their flags). Updated the example shape in the primer to show a literal expected-stderr block, a literal expected-stdout block, and a tagged on-disk-state assertion.

This refinement remains within G-97's scope (manual-testing-checklist authoring quality). G-97 stays Addressed; the registry row and entry text capture the original defect and this refinement together.

---

**Finding 3 — CLI projects had no standing help-output verification requirement (added 2026-05-05).**

Finding 2 introduced help-command specificity as a sub-clause on requirement #4 ("when help text is part of verification, name the specific subcommand"). That solves the rendering problem when the author has decided to include help verification — but it does not require help verification in the first place. The Layer 4 plan included help verification only because the author chose to. A Layer 5 or Layer 6 plan, against the standard as written after Finding 2, could omit help verification entirely. That would be a regression: any layer that adds a flag, subcommand, or argument is also adding to the binary's discoverability surface, and a tester running the manual plan should be able to confirm the new surface is actually exposed.

User direction: "Future test plans for cli apps should include manually verifying help output." The standard should require this for CLI projects rather than leave it to author judgment.

The defect this catches: a CLI layer ships with a flag whose runtime behavior works (covered by integration tests) but whose `--help` description is missing, stale, or contradicts the actual flag. Integration tests do not exercise help output unless explicitly written to do so; manual testers who only run the behavioral tests will not notice the discoverability gap. Multi-month-later users hitting `--help` to relearn the tool see drift accumulate.

**Resolution:** Added a sixth bullet to the **Required items per layer** list in `prompts/decomposition.md` — "Help-output verification (CLI projects)". The bullet specifies: per binary and per changed subcommand, run `<binary> <subcommand> --help` and include a literal expected-output block. Anchored to "Step 0 — Update the installed binary" so a stale-binary problem fails fast. Carves out an explicit exception for layers that don't change the CLI surface (internal-only refactor / polish). Forward-only.

Considered and rejected: placing this rule in `supplements/cli.md` instead of `decomposition.md`. The supplement architecture is currently consumed by domain prompt files (review-time), not by session primers (authoring-time); routing a manual-testing-plan rule through the supplement would introduce a new cross-reference pattern between primers and supplements that no other primer uses. Inline in `decomposition.md` with a CLI-conditional clause keeps the supplement focused on review dimensions. If multiple interface types accumulate similar primer-time conditional clauses (CLI, browser, mobile), consolidate later — premature pattern.

This refinement also remains within G-97's scope (manual-testing-checklist authoring quality, with CLI-specific authoring requirements as a sub-concern). No new gap registered.

---

**Finding 4 — Usage examples in `--help` for compound CLI flags were not captured anywhere (added 2026-05-05).**

User direction after Layer 4 manual testing on `issue-tracker-cli`: "I like usage examples for cli commands covering common scenarios like filtering. If that's already in layer 7 it can wait. If it wasn't previously captured then add it to the decomp primer as appropriate."

Triage of where this concern is currently captured:

- **`issue-tracker-cli/TODO.md` Layer 7 acceptance criteria** — describes flags and valid values (line 354–358); does **not** require usage examples in help output.
- **`issue-tracker-cli/DESIGN.md` `--help` flag section** (line 218) — "must accurately describe all flags and their valid values"; does **not** require usage examples.
- **`supplements/cli.md` UX dim 1** — *partially* captured: "Does the top-level help include a usage example?" This is review-time guidance, scoped to *top-level* help only, and asks a yes/no question rather than mandating it. It does not extend to subcommand-level usage examples for compound flags.

So the concern is unaddressed for subcommand-level help where compound flags exist. The defect this catches: a user who runs `<binary> list --help` against a layer with five filters sees five orthogonal flag descriptions and has to imagine which combinations make sense. They cannot tell from the flag list alone that `list --status open --priority high --label bug` is a sensible compound query. A short `Examples:` block answers the "how do I do the thing I came to do?" question that a flag list does not.

**Resolution:** Added a seventh bullet to the **Required items per layer** list in `prompts/decomposition.md` — "Usage examples in `--help` (CLI projects with compound flags or filters)". The bullet directs the decomposing agent to identify the polish/help-finalization layer and add two acceptance criteria there: (1) the subcommand's `--help` includes 1–3 usage examples covering common scenarios (compound filtering for `list`-style commands; the most-frequent creation form for `create`-style commands); (2) the manual test plan for that layer verifies the examples appear in the help-output expected block. The bullet cross-references the existing `supplements/cli.md` UX dim 1 (review-time, top-level) and explicitly extends the expectation to subcommand-level for compound-flag cases.

Considered and rejected: amending `supplements/cli.md` UX dim 1 to also assert subcommand-level coverage. The supplement is review-time guidance; primer-time authoring guidance is decomposition.md's job. Bumping the supplement would still leave the decomp output unimproved — a Layer 7 plan written without this primer guidance would still skip usage examples, and the supplement would only catch the gap during IAR review, after the implementation existed. Capturing at the primer level closes the gap before authoring, not just at review.

Considered and rejected: amending `issue-tracker-cli/DESIGN.md` and Layer 7 acceptance criteria directly. That would only fix `issue-tracker-cli`'s case; future CLI portfolio projects would re-discover the gap. Primer-level capture applies to all future CLI decomp output.

This refinement also remains within G-97's scope (manual-testing-checklist authoring quality, with CLI-specific authoring requirements as a sub-concern). No new gap registered. Note that this is a forward-only change for new CLI decomp output; `issue-tracker-cli`'s Layer 7 acceptance criteria are not amended retroactively, but the apprentice's reviewer can elect to add the usage-examples ACs to Layer 7 manually before that layer opens.
