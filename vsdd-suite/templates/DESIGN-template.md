# DESIGN.md — {{PROJECT_NAME}}

Phase 1a contract. Authored with [`vsdd-suite/primers/1a-spec-crystallization.md`](../vsdd-suite/primers/1a-spec-crystallization.md) loaded. Work the driving questions in the primer to populate this skeleton — the headings below are starting structure, not a fill-in-the-blanks form. Iterate until the self-adversary check passes (named in the primer's completion criteria), then commit. After commit, this file is the contract every Phase 2+ artifact is evaluated against; spec defects are routed back to Phase 1a per `primers/4-feedback-integration.md`.

---

## What this project does

One paragraph. State the user-visible purpose in concrete behavioral terms. A reader who knows nothing about the project must finish this paragraph able to describe what it does.

## Scope and non-goals

What is explicitly in scope and what is explicitly out of scope. Out-of-scope items prevent scope creep during later phases — they are as load-bearing as the in-scope items. A non-goal list of `(none)` is itself a finding for VDD-IAR Alignment dim 1; absence of non-goals usually means scope was not interrogated.

## Behavioral contracts

The observable behaviors the project guarantees. Each contract is testable from outside the implementation. List per primary capability:

- **Capability A:**
  - Input shape: …
  - Output shape (success): …
  - Output shape (failure): …
  - Error conditions: …
  - Side effects (if any): …

## Edge case catalog

Inputs at the boundaries — empty, oversized, malformed, concurrent, adversarial. The catalog is the source of truth for what Phase 2a writes Red Gate tests for. Each entry: the edge case, the expected behavior, the failure mode if unhandled.

## Interface definitions

The external surfaces other systems (or users) interact with. CLI: argument/flag list with semantics. Library: public API with signatures. Service: HTTP routes / RPC methods. File format: schema. Be precise — every promise here is a contract Phase 3 enforces.

## Verification architecture

How the project is proven correct. Test layers (unit, integration, end-to-end); the manual testing checklist authored alongside the layer plan in Phase 1b; the IAR cadence; any property-based or fuzz testing.

## Technology choices and rationale

The decisions that bound the implementation. Each choice with: what was chosen, what alternatives were considered, why this option fits the constraints. A choice without a rationale is an outcome record, not a decision — Phase 3 Technical Writer will catch it.

## Constraints

External constraints the implementation must respect: dependencies, platforms, performance budgets, accessibility minima, security posture, regulatory requirements, deployment target.

## Open questions

Items that need decision before Phase 1b can complete. An empty Open Questions section after a single Phase 1a session is suspicious — the self-adversary check usually surfaces at least one ambiguity worth recording.
