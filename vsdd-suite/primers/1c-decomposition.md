# Session Primer: Project Decomposition (VSDD Phase 1c)

Use this prompt after `DESIGN.md` is complete and has passed adversarial pressure. The output of this session is a `TODO.md` with a layered development plan and (for Phase 2+ projects) a crosslink issue hierarchy. This is the plan the Red Gate (Phase 2) will execute against.

Do not start decomposition until the spec is complete. Decomposing an incomplete spec produces a plan that will require structural revision mid-build.

**Whitepaper alignment (G-96).** This step tracks the VSDD whitepaper's **Phase 1c (Spec Review Gate)** semantics: the spec is examined against the question "can each behavior named here be independently verified?" before any Phase 2 work begins. Decomposition is the operational form of that review — a spec that cannot be layered into independently-verifiable units is a spec gap, not a decomposition gap. The whitepaper's Phase 1b (Verification Architecture — the purity-boundary map, automatable-vs-manual split, and Phase-5 formal-proof candidates) is folded into the suite's Phase 1a+1b primer rather than carried as a separate step. The forward-only policy applies: project review logs that reference "Phase 1b" decomposition under the prior suite convention remain valid records.

---

## Prompt

You are helping decompose a completed software specification into a layered development plan under the Verified Spec-Driven Development (VSDD) methodology. This is Phase 1c: Decomposition (Spec Review Gate).

**Your posture:** Every layer you propose must be independently verifiable before the next layer opens. If you cannot write specific acceptance criteria for a layer that a human could check manually, the layer is not well-defined. Push back on layers that are too large, too vague, or that mix concerns.

**The Red Gate applies to every layer:** For each layer, the test plan must be written before implementation begins. A layer whose tests would pass against an empty implementation was not designed for test-first development. When writing acceptance criteria, ask: "what test would fail right now, before any code exists?"

**Linear accountability:** Every piece of work traces to a layer, every layer has explicit acceptance criteria, every acceptance criterion has a corresponding test or manual check. There is no undocumented work.

**Primary failure mode:** Accepting the developer's proposed layer structure without verifying each layer is independently gatable. A decomposing agent that approves all proposed layers without challenge is transcribing, not decomposing. Watch for layers that cannot be tested in isolation, acceptance criteria that require judgment rather than specific observations, and test plans whose tests would not fail against an empty implementation.

---

## DESIGN.md reference

*(Paste the full DESIGN.md here, or reference it in the session.)*

---

## Layer structure rules

A well-formed layer:

1. **Delivers one atomic capability.** A user can do something real and complete with the app after this layer that they could not do before. Not "the data model exists" but "the user can add an item and it persists."
2. **Has defined acceptance criteria.** Each criterion is specific: it names what to observe, not just what to click. "Verify the error message reads 'URL is required'" passes; "verify validation works" does not.
3. **Has a manual testing checklist.** Automated tests verify correctness. Manual testing verifies that the experience is coherent — interactions that are technically correct but wrong in context, empty states, error recovery. Every layer needs both.
4. **Has a test plan that satisfies the Red Gate.** Before the layer opens, enumerate which tests will be written first. Each test should be describable now (before implementation) because it is specifying behavior, not verifying existing code.
5. **Does not depend on unbuilt layers.** A layer that requires future layers to be meaningful is not atomic.

Layer anti-patterns:
- "Foundation layer" (setup with nothing verifiable) — fold into Layer 1 or make explicit what is verifiable
- "Polish layer" that touches everything — scope it to specific behaviors
- Layer acceptance criteria that require judgment ("looks good") rather than specific observations
- Tests that could only have been written after seeing the implementation

---

## Driving questions

### Layer boundaries

For each proposed layer, ask:
- After this layer, what can the user do that they could not do before?
- What is the most minimal version of this capability that is independently verifiable?
- What is explicitly not in this layer that might seem like it belongs here?
- What does the app look like in a broken state if this layer's implementation is wrong?

### Acceptance criteria

For each criterion, ask:
- Is this specific enough that two reviewers would agree on whether it passes?
- Does it name what to observe (not just what action to take)?
- Is there a corresponding automated test, or is manual testing the appropriate check?
- What would this criterion look like if it were failing?

### Red Gate setup

For each layer, before writing any code, ask:
- What tests need to exist and be failing before the first line of implementation?
- For each test: what would this test assert against an empty function? Would it fail? (If not — the test was written for existing code, not for specified behavior.)
- Are behavioral test names established now? ("returns an error message for empty URL" — not "tests validateUrl")

### Manual testing checklist

For each layer, enumerate the test items as **runnable steps** — not one-line checklist entries. A checklist item that says "verify it works" cannot be executed by a tester who is not the author. The plan must be executable top-to-bottom from a clean checkout without consulting the developer or an AI.

**Manual testing is a second adversarial surface to IAR, not a checkbox (G-132).** The manual testing checklist is the surface where the director surfaces what artifact-reading does not catch. IAR domains read source code, tests, DESIGN.md, and prior review logs; they do not exercise the built binary against the spec's invariants. The director running manual tests does exactly that — and catches what cold-batch adversarial review misses. The canonical example is ITC L6 R3 SO R22: 11 cold-batch IAR domain reviews (including the dedicated Feature-5 review surface for the delete command) all missed the spec violation that "delete the highest-id issue, then create" reuses the deleted ID; the director executing manual checklist item 8 caught it on the first run. Treat **manual-test findings as Round N findings of equal weight to cold-session findings** — log them in the per-domain review log with `**Source:** director-raised` per the per-review entry preamble standard (`../suite-development/suite-development.md` § Per-review entry preamble § Source). Manual testing checklist deferred to a later round is itself a finding for VDD-IAR Alignment dim 3 — the merge gate requires the checklist closed (per `../suite-development/suite-development.md` § Layer-gate close criteria). **Quick-closure framing of manual testing is its own audit signal:** a 16-minute window between implementation and manual checklist closure with no commit-body specificity is the kind of finding a manager doing an investigation would flag in an incident report or audit; quick closure with commit-body specificity demonstrating actual execution is the discipline working. The Technical Writer review dimension on manual-test note quality (see TW domain prompt) evaluates the audit-trail signal alongside the closure.

**Each step must include:**

1. **The exact command.** Not `cargo install`, but `cargo install --path . --locked --force` from the project directory. Spell out working directory, paths, and shell idioms.
2. **The expected outcome — shown as a literal block when the output is invariant.** For deterministic output (help text, error messages, fixed-shape success messages, exit codes captured by `echo "exit: $?"`), include the exact bytes the tester should see in a fenced code block. Tag the stream where ambiguous (`Expected stdout`, `Expected stderr`). For variable output (timestamps, generated IDs, OS-chosen paths), pin the invariant parts in prose with a representative literal example. Prose like "expect the help text to list `--label`" is not testable — the tester cannot tell whether the right help (e.g., subcommand-level help) was consulted. A literal block can be diff-compared; a prose description cannot.
3. **Explicit clean-state setup when required.** If the test depends on a fresh state, the step starts with the cleanup command (`rm -f tracker.json`, `rm -rf .cache`, etc.). Do not assume the tester remembers state from earlier tests.
4. **The binary lifecycle, where relevant.** Source changes do not automatically reach the installed binary (`~/.cargo/bin/<name>`, `/usr/local/bin/<name>`, etc.). A test plan that exercises a layer with new runtime behavior opens with an explicit "update the installed binary" step. The persistence test exercises uninstall + reinstall (not just process restart) to verify data survives a binary swap. When help text is part of the verification, the test names the specific help command (`<binary> <subcommand> --help`, not `<binary> --help`) — top-level help typically lists subcommands only, not their flags.

**Required items per layer:**
- The happy path — full command sequence with expected output
- Each error state — the input that triggers it + the exact stderr line + the exit code
- The empty state — what the user sees before any data exists, including pipe-consumer behavior if the layer affects stdout/stderr discipline
- The persistence check — install → create data → uninstall → reinstall → verify data survives
- Each edge case in DESIGN.md — phrased as a runnable step with expected output
- **Per-property free-form text field defense (G-124).** For every new free-form text field the layer introduces (title, label, description, name, note, message, etc. — anything that accepts arbitrary string input), the Red Gate plan must include all four of: (a) create-time `parse_*`/`validate_*` rejection test for control characters (`Cc`) and format characters (`Cf` for Trojan-Source defense, e.g., U+202E and zero-width characters); (b) load-time `*_is_valid` symmetric rejection test for the same character class (per G-126); (c) error-message escape test for any code path that interpolates the rejected input back to stdout/stderr (per G-125); (d) DESIGN.md control-character policy paragraph stating the field-by-field rule. The class generalizes — ITC's Title (L1) → Labels (L4) → Description (L6) was three consecutive instances of the same defense gap at three layers. The Red Gate plan item is the place to catch the *fourth* instance in advance rather than at IAR Round 3 of the new layer.
- **Empty-state coverage on every new filter / sort / selection dimension (G-127).** For every new filter, sort, or selection dimension the layer introduces (`--status`, `--priority`, `--label`, `--since`, `--sort-by`, etc.), the Red Gate plan must include an empty-state assertion test asserting both (a) "filtered down to zero results, the no-matches-for-this-filter message renders correctly" and (b) "the empty-state branches correctly between the no-data-at-all message and the no-matches-for-this-filter message." A test that exists for one filter dimension but not its symmetric peer at the same layer is itself a Red Gate finding — the symmetric absence is the defect, not the implementation gap that the absence eventually exposes (ITC L2 added the empty-state test for `--status`; L3 missed the symmetric test for `--priority`, and SO R11 caught the regression). The empty-state-per-filter check belongs at Red Gate authoring time, not at SO review time.
- **Help-output verification (CLI projects).** For the binary and each subcommand whose surface changed at this layer, run `<binary> <subcommand> --help` and verify the literal output (a fenced expected-output block per subcommand). Top-level `<binary> --help` is not sufficient — most CLI frameworks list subcommand names there but not their flags, so a stale install or a missing flag is invisible at the top level. The verification typically belongs in the layer's "Step 0 — Update the installed binary" so a stale-binary problem fails fast before any behavior tests run. New layers that don't change the CLI surface (e.g., a polish layer that only adjusts internals) can omit this; layers that add, remove, or change a flag, subcommand, argument, or `--help` description must include it.
- **Usage examples in `--help` (CLI projects with compound flags or filters).** For projects whose subcommands accept multiple optional flags, repeated flags, or compound filter combinations (e.g., `list --status open --priority high --label bug`), the polish or help-finalization layer's acceptance criteria must require **usage examples in the relevant subcommand's `--help` output covering common scenarios** — most-frequent invocations, compound filtering, the workflow that motivated the feature. A flag list answers "what can I do?"; usage examples answer "how do I do the thing I came to do?" Without examples, a user who reads `--help` and sees five orthogonal flags has to imagine which combinations make sense. The decomposing agent identifies the layer that owns help polish (typically the final polish layer) and adds two acceptance criteria there: (1) the subcommand's `--help` includes 1–3 usage examples covering common scenarios (compound filtering for `list`-style commands; the most-frequent creation form for `create`-style commands); (2) the manual test plan for that layer verifies the examples appear by including them in the help-output expected-block. The CLI supplement (`supplements/cli.md` UX dim 1) already asks at review time whether top-level help includes a usage example; this primer-time guidance extends the same expectation to subcommand-level help where compound flags exist.

**Audience.** Assume the tester is unfamiliar with the toolchain and with the project. The shorthand "verify it persists" is meaningful to the developer who wrote the layer; it is not meaningful to a reviewer running the plan two months later, or to an AI agent picking up the work in a new session, or to the apprentice's reviewer running it for the first time. The cost of an over-explicit plan is words on a page; the cost of a too-terse plan is a test session that becomes a debugging session because the tester cannot tell whether they are verifying behavior or fighting environmental drift.

**Example shape (one expanded step with a literal expected-output block):**

````markdown
### Step 3 — Empty title rejected

```sh
rm -f tracker.json
tracker create ""
echo "exit: $?"
```

Expected stderr (literal):

```
Error: Title cannot be empty.
```

Expected stdout (literal — `echo` line only; the error went to stderr):

```
exit: 1
```

Expected on-disk state: `tracker.json` is not created.
````

A flat one-line bullet ("Happy path: create issue with `--label bug --label auth` → list shows `bug, auth`") is a draft, not a test plan. The decomposition is not complete until each bullet is expanded into a runnable step with literal expected output where the output is invariant.

---

## Work accountability

The accountability principle is the same at every phase: every piece of work is explicitly planned before it begins, and no undocumented work exists. The tool that holds the plan changes by phase:

- **Phase 1:** `TODO.md` is the source of truth. Layers, acceptance criteria, and manual testing checklists live there. Work that is not in a layer is not approved work.
- **Phase 2+:** Crosslink replaces `TODO.md` as the source of truth. The issue hierarchy (epics → issues → subissues) is the plan. `TODO.md` is not maintained separately once crosslink is introduced.

---

## Crosslink issue hierarchy (Phase 2+ projects)

If this is a Phase 2+ project, the crosslink issue hierarchy is the plan — build it before writing any code. TODO.md is not used.

Crosslink uses `--parent <id>` on `quick` / `issue create` to form the hierarchy. There is no separate `subissue` command; the parent relationship is a flag on creation.

```sh
# Create the epic (parent) for the project
EPIC=$(crosslink quick "[Project Name]" -p high -l epic --quiet)

# Create one milestone per layer (gives layers a first-class container, separate from labels)
crosslink milestone create "Layer 1: [Capability]"
crosslink milestone create "Layer 2: [Capability]"

# Create an issue per layer, hung off the epic
LAYER1=$(crosslink quick "Layer 1: [Capability]" -p high -l feature -l layer --parent "$EPIC" --quiet)

# Attach the layer issue to its milestone
crosslink milestone add "Layer 1: [Capability]" "$LAYER1"

# Create sub-issues for acceptance criteria that need their own tracking
crosslink quick "AC: [specific criterion]" -p medium -l acceptance-criterion --parent "$LAYER1"

# Record the Red Gate test plan as a comment on the layer issue
crosslink issue comment "$LAYER1" "Red Gate: tests to write before implementation — [list]"

# Open a session when work on a layer begins
crosslink session start
crosslink session work "$LAYER1"
# ... do the work ...
crosslink session end --notes "Layer 1 complete. [What was learned. What was harder than expected.]"

# Run the project test suite as the layer gate (Phase 2b → 3 boundary)
crosslink swarm gate layer-1
```

Rules:
- Issues are created before work begins, not after
- A crosslink issue exists for every piece of work. No undocumented work.
- Session handoff notes are required. A session that ends without notes cannot be resumed cleanly.
- Decisions made during a layer go in issue comments, not just in commit messages
- One milestone per layer. The milestone is the layer container; the layer issue and its acceptance-criterion sub-issues all attach to it. This makes layer-scoped views (`crosslink milestone show "Layer 1: ..."`) a first-class operation instead of a label filter.
- `crosslink swarm gate <phase-slug>` runs the project test suite as the formal Phase 2b → 3 boundary. A layer that doesn't pass its gate doesn't open for IAR. Use the slug form of the milestone name (`layer-1`, `layer-2`).

**Verifying deployed policy:** `crosslink workflow diff` compares the policy files crosslink has deployed in the project against its embedded defaults. Run it after `crosslink init` to confirm the deployed rules match what the suite expects, and after any local policy edits to surface drift. A green diff is part of the Phase 1c completion check.

---

## TODO.md format

The decomposition produces a `TODO.md` with this structure:

```markdown
# [Project Name] — Development Plan

## Layer 1: [Capability Name]

**Goal:** One sentence. What the user can do after this layer.

**Acceptance Criteria:**
- [ ] [Specific, observable criterion]
- [ ] [Specific, observable criterion]
...

**Manual Testing Checklist:**

(Each item below is a placeholder for a runnable step block — full command + expected output + explicit clean-state and binary-lifecycle steps. See `### Manual testing checklist` above for the standard.)

- [ ] Step 0 — Update the installed binary (when the layer changes runtime behavior)
- [ ] Happy path
- [ ] Each error state
- [ ] Empty state (with pipe-consumer behavior if relevant)
- [ ] Persistence: install → create → uninstall → reinstall → verify
- [ ] Each edge case from DESIGN.md
...

**Red Gate — tests to write first:**
- `[behavioral test name]` — asserts [what], fails against [stub behavior]
- `[behavioral test name]` — asserts [what], fails against [stub behavior]
...

**IAR:** [domain list for this layer's review]

---

## Layer 2: [Capability Name]
...
```

---

## Right-size the IAR (intent-keyed active-domain set)

The decomposition picks the active-domain set for each layer's IAR, not the IAR runtime. The default "all 7 cores plus warranted extended domains" is the right starting point for **portfolio**-intent projects (per G-121's scaffold default). Other intents calibrate the set per `../domains/DOMAIN-INDEX.md` § Intent calibration.

Read the project's `DESIGN.md` § Project intent before authoring `TODO.md` Layer N's IAR line. Then populate the layer's `**IAR:**` field with the active-domain set this layer will run, derived from the intent. The set may vary by layer for `learning-exercise` intent (the rotating fourth optional core); other intents typically use the same set every layer unless a specific layer warrants narrowing (e.g., a polish layer with no new attack surface may skip Security + Red Team).

**Worked example — learning-exercise intent, 4-layer project, rotating optional core:**

```markdown
## Layer 1: Core data model
...
**IAR:** SE, QE, SO + SA (rotating: structural-decision layer, SA rotated in)

## Layer 2: Persistence + load-time validation
...
**IAR:** SE, QE, SO + Security (rotating: validation surface, Security rotated in)

## Layer 3: Filtering + sorting
...
**IAR:** SE, QE, SO + DE (rotating: data-query semantics, DE rotated in)

## Layer 4: Polish
...
**IAR:** SE, QE, SO + UX (rotating: user-facing layer, UX rotated in)
```

**Worked example — portfolio intent (default):**

```markdown
## Layer 1: Core create + list
...
**IAR:** SE, QE, SO, SA, Security, UX, Platform Engineer + Technical Writer (active for portfolio handoff per DOMAIN-INDEX.md activation criteria)
```

**Anti-pattern: declaring high intent without acknowledging the cost.** A learning-exercise project that runs the full 11-domain treatment because "the methodology is there, why not use it" is over-investing methodology effort relative to the assignment bar. The dollspace.gay critique of `issue-tracker-cli` (Review 51 / G-150) named this as the headline drift mode — the project ran like a production tool when the assignment asked for a learning exercise. The check at decomposition time: does the layer's `**IAR:**` line match what the project's intent calls for, not just what feels comprehensive?

**Anti-pattern: silent intent demotion.** A project that started at `portfolio` intent and is implicitly run as `learning-exercise` by skipping domain reviews mid-project is dishonest about the bar — demotion is not allowed per DOMAIN-INDEX.md § Intent calibration. If the project genuinely needs to narrow scope, the intent change is itself a DESIGN.md amendment with Solution Owner authority; the demotion is rejected (history is preserved) and the project either remains at portfolio bar or splits into a learning-exercise sub-project with a clear scope boundary.

---

## Completion criteria

The decomposition is ready to move to Phase 2 (Red Gate / implementation) when:

1. Every feature in DESIGN.md is covered by at least one layer
2. Every layer has specific, independently verifiable acceptance criteria
3. Every layer has a manual testing checklist
4. Every layer has a Red Gate test plan (tests to write before implementation)
5. The layers are ordered so each builds directly on the previous — no layer requires an unbuilt dependency
6. **The active-IAR domain set per layer is intent-calibrated.** Every layer's `**IAR:**` line names the active-domain set, derived from `DESIGN.md` § Project intent per the table in `../domains/DOMAIN-INDEX.md` § Intent calibration. A layer with `**IAR:** all domains` for a learning-exercise project is over-investment; a layer with `**IAR:** SE only` for a production project is under-investment. Either pattern is a Phase 1c finding.
7. For Phase 2+ projects: the crosslink issue hierarchy is created, one milestone per layer is created and populated, and `crosslink workflow diff` runs clean before the first session opens

This plan will be evaluated against VDD-IAR Alignment dims 2 (layered decomposition), 3 (layer gate compliance), and 4 (test discipline). A TODO.md that lists features without acceptance criteria, or layers without manual testing checklists, will not pass the layer gate.
