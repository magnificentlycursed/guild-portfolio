# Session Primer: Project Decomposition (VSDD Phase 1b)

Use this prompt after `DESIGN.md` is complete and has passed adversarial pressure. The output of this session is a `TODO.md` with a layered development plan and (for Phase 2+ projects) a crosslink issue hierarchy. This is the plan the Red Gate (Phase 2) will execute against.

Do not start decomposition until the spec is complete. Decomposing an incomplete spec produces a plan that will require structural revision mid-build.

---

## Prompt

You are helping decompose a completed software specification into a layered development plan under the Verified Spec-Driven Development (VSDD) methodology. This is Phase 1b: Decomposition.

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

For each layer, enumerate:
- The happy path (step by step)
- The error states (what does the user see?)
- The empty state (what does the user see before any data exists?)
- The persistence check (what survives a reload?)
- The edge cases identified in DESIGN.md for this layer's features

---

## Work accountability

The accountability principle is the same at every phase: every piece of work is explicitly planned before it begins, and no undocumented work exists. The tool that holds the plan changes by phase:

- **Phase 1:** `TODO.md` is the source of truth. Layers, acceptance criteria, and manual testing checklists live there. Work that is not in a layer is not approved work.
- **Phase 2+:** Crosslink replaces `TODO.md` as the source of truth. The issue hierarchy (epics → issues → subissues) is the plan. `TODO.md` is not maintained separately once crosslink is introduced.

---

## Crosslink issue hierarchy (Phase 2+ projects)

If this is a Phase 2+ project, the crosslink issue hierarchy is the plan — build it before writing any code. TODO.md is not used.

```
# Create an epic for the project
crosslink quick "[Project Name]" -p high -l epic

# Create an issue per layer
crosslink quick "Layer 1: [Capability]" -p high -l feature
crosslink subissue <epic_id> <layer_issue_id>

# Create sub-issues for acceptance criteria that need tracking
crosslink subissue <layer_id> "AC: [specific criterion]"

# Record the Red Gate test plan as a comment
crosslink issue comment <layer_id> "Red Gate: tests to write before implementation — [list]"

# Open a session when work on a layer begins
crosslink session start
# ... do the work ...
crosslink session end --notes "Layer N complete. [What was learned. What was harder than expected.]"
```

Rules:
- Issues are created before work begins, not after
- A crosslink issue exists for every piece of work. No undocumented work.
- Session handoff notes are required. A session that ends without notes cannot be resumed cleanly.
- Decisions made during a layer go in issue comments, not just in commit messages

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
- [ ] Happy path: [step by step]
- [ ] Error state: [what to observe]
- [ ] Empty state: [what to observe]
- [ ] Persistence: [reload and verify]
- [ ] [Edge case from DESIGN.md]
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

## Completion criteria

The decomposition is ready to move to Phase 2 (Red Gate / implementation) when:

1. Every feature in DESIGN.md is covered by at least one layer
2. Every layer has specific, independently verifiable acceptance criteria
3. Every layer has a manual testing checklist
4. Every layer has a Red Gate test plan (tests to write before implementation)
5. The layers are ordered so each builds directly on the previous — no layer requires an unbuilt dependency
6. For Phase 2+ projects: the crosslink issue hierarchy is created before the first session opens

This plan will be evaluated against VDD-IAR Alignment dims 2 (layered decomposition), 3 (layer gate compliance), and 4 (test discipline). A TODO.md that lists features without acceptance criteria, or layers without manual testing checklists, will not pass the layer gate.
