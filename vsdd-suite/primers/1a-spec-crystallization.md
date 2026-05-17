# Session Primer: Spec Crystallization (VSDD Phase 1a)

Use this prompt at the start of a new project before writing any code. Paste it into a fresh AI session, then fill in the project description section. The output of this session is a `DESIGN.md` that will pass VDD-IAR Alignment dim 1 scrutiny.

This is not a review prompt. It is a construction prompt — it drives the creation of a complete specification. The adversarial pressure is applied *during* spec writing, not after.

**Session medium:** The default is a fresh chat session into which you paste this primer. Projects using crosslink may instead launch the session via `crosslink design [description] [--issue <id>] [--gh-issue <id>]`, which opens a foreground design session pre-loaded with the named issue's context and writes the draft to `.design/<slug>.md`. Iterating later uses `crosslink design --continue <slug>`. The primer text below is the same in either medium — `crosslink design` provides the session container, not the spec discipline. See `README.md` § Worked example: A VSDD session with crosslink.

---

## Prompt

You are helping create a software specification under the Verified Spec-Driven Development (VSDD) methodology. This is Phase 1a: Spec Crystallization. Your role is to drive toward a specification that is complete enough to be verified — not just described.

**Your posture:** Assume the spec is incomplete until proven otherwise. Your job is to find every place where the behavior is undefined, ambiguous, or only described for the happy path. Do not confirm what the developer has written. Find what is missing.

**Governing standard:** A complete VSDD Phase 1a spec contains:
- **Behavioral contracts**: for each feature, explicit preconditions (what must be true for the operation to be valid), postconditions (what the system guarantees after the operation), and invariants (what is always true regardless of operation sequence)
- **Exhaustive edge case catalog**: boundary values, empty inputs, malformed inputs, and failure modes enumerated before implementation, not discovered during debugging
- **Interface definitions**: explicit data shapes, validation rules, and error responses at every system boundary — not implied by the implementation
- **Verification architecture**: a map of which behaviors are automatable, which require manual testing, and (for Phase 5+) which properties are candidates for formal proof. The spec should identify the purity boundary: which functions are pure (deterministic, no I/O, formally verifiable) and which are effectful (I/O, storage, rendering)
- **Scope boundary**: what is explicitly out of scope is as important as what is in scope. An unconstrained spec grows

**What a bad spec looks like:** A feature list. A list of "the app will..." statements. A description of the happy path with no failure modes. A data model without invariants. A UI description without empty states or error states. An acceptance criterion that the developer and reviewer would interpret differently.

---

## Project type

Before starting, characterize the project type below and establish it with the AI before the first driving question. Different project types require different framing — the purity boundary and verification architecture look different for each.

| Type | Key spec concerns | Notes |
|---|---|---|
| **User-facing app** (browser, desktop, mobile) | Empty states, error states, form behavior, navigation, persistence | Standard driving questions below apply |
| **CLI tool** | Command interface, argument validation, stdout/stderr/exit code contracts, piped input, interactive vs. scripted use | Replace "UI" questions with interface contract questions |
| **Library / module** | Public API contracts, caller invariants, versioning behavior, error propagation to callers | No "user" — the caller is the user; define the API surface exhaustively |
| **Infrastructure / service** | Deployment context, failure modes, latency contracts, integration surfaces, operational requirements | No UI; spec the interface, the failure envelope, and the observability surface |
| **Research / speculative** | Learning goals, validation criteria, what a negative result looks like, kill criteria | The "acceptance criteria" are epistemic: what would you have learned? |

---

## Project description

*(Fill this in before starting the session.)*

**Assignment or goal:**

**Primary technology constraints:**

**Known constraints or non-negotiables:**

**Out of scope (initial thoughts):**

---

## Driving questions

Work through these questions with the AI. Do not proceed to the next section of the spec until the current one is complete.

### Features and behaviors

For each feature in the spec, ask:
- What must be true *before* this operation can succeed? (preconditions)
- What does the system guarantee *after* this operation? (postconditions)
- What does the system guarantee is *always* true, regardless of what sequence of operations has occurred? (invariants)
- What happens when the preconditions are not met? Is the error message specific? Does the system recover cleanly?
- What is the empty state for this feature? What does the user see when there is no data yet?

### Edge cases

For each input the system accepts, ask:
- What is the minimum valid input? The maximum?
- What happens at the boundary?
- What happens with whitespace-only input?
- What happens with duplicate input?
- What happens if the user submits the same operation twice rapidly?
- What is malformed input for this field? How is it rejected?

### Data model

For each piece of stored data, ask:
- What invariants must always hold? (e.g., "no two bookmarks can have the same URL")
- What happens when data is read back after a browser restart / app restart?
- What is the migration strategy if the data shape changes?
- Are there fields that must be unique? Ordered? Non-empty?

### Verification architecture

Before finalizing the spec, define:
- Which features can be verified with automated tests alone?
- Which require a human to evaluate (judgment-dependent UX, visual output, platform behavior)?
- Which functions will be pure (no I/O, deterministic output, formally verifiable in principle)?
- What is the purity boundary: where does deterministic business logic end and effectful code begin?

### Scope

Before finalizing, explicitly enumerate what is *not* being built. For every feature that came up in discussion but was excluded, write one line explaining why. An unwritten scope boundary will be violated.

---

## Self-adversary check

Before this spec is considered done, argue with it:

1. Read the spec as if you are an adversarial reviewer who has never seen it before. What behavior is undefined? What would a fresh implementation miss?
2. For each feature, ask: "if I implemented only what is written here, would the result be what I actually want?" If no — the spec is incomplete.
3. For each acceptance criterion, ask: "would I and a reviewer agree on whether this criterion is met?" If there is room for disagreement — the criterion is not specific enough.
4. Read the edge case catalog. Is every input that could be entered in production represented? Assume the first user will enter something unexpected.
5. Read the out-of-scope section. Is each item *actually* excluded, or could a careless implementation accidentally include it?

---

## Completion criteria

The spec is ready to move to Phase 1b (decomposition) when:

1. Every feature has explicit preconditions, postconditions, and invariants — not just happy-path descriptions
2. The edge case catalog covers boundary values, empty inputs, malformed inputs, and failure modes enumerated before implementation
3. The interface definitions specify data shapes, validation rules, and error responses at every system boundary
4. The verification architecture names which behaviors are automatable, which require manual testing, and where the purity boundary is
5. The out-of-scope section explicitly names excluded features with rationale
6. You cannot find an undefined behavior after genuine adversarial pressure using the self-adversary check above

**Promotion (crosslink users):** When the draft passes the completion criteria, promote it from `.design/<slug>.md` to the project's `DESIGN.md`. The `.design/` drafts are working files; `DESIGN.md` is the contract that Phase 3 evaluates against. Projects without crosslink write `DESIGN.md` directly — there is no separate draft file.

---

## Output: DESIGN.md structure

The spec should produce a DESIGN.md with at minimum these sections:

```
# [Project Name]

## Overview
One paragraph. What is this, for whom, and why.

## Features
Per feature: description, behavioral contract (preconditions, postconditions, invariants), and error states.

## Data Model
Data shapes with field-level validation rules and invariants.

## Interface
Technology choices. UI layout description (if applicable). Input/output formats.

## Constraints
Non-negotiables. Performance requirements. Accessibility requirements. Security requirements.

## Edge Cases
Exhaustive enumeration of non-obvious inputs and their expected behavior.

## Testing Methodology
Automated test scope. Manual testing requirements. Purity boundary map.

## Out of Scope
Explicit exclusions with brief rationale.
```

This structure will be evaluated against VDD-IAR Alignment dim 1. A design doc that enumerates only features without behavioral contracts, invariants, or edge cases is an incomplete spec.
