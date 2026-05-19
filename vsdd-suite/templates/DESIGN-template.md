# DESIGN.md — {{PROJECT_NAME}}

Phase 1a+1b contract. Authored with [`vsdd-suite/primers/1ab-spec-crystallization.md`](../vsdd-suite/primers/1ab-spec-crystallization.md) loaded. Work the driving questions in the primer to populate this skeleton — the headings below are starting structure, not a fill-in-the-blanks form. Iterate until the self-adversary check passes (named in the primer's completion criteria), then commit. After commit, this file is the contract every Phase 2+ artifact is evaluated against; spec defects are routed back to Phase 1a+1b per `primers/4-feedback-integration.md`.

---

## Project intent

Declare one of the four intent levels below. The intent gates the IAR intensity calibration (active-domain set, stop-signal sensitivity, fresh-system install discipline) per `vsdd-suite/domains/DOMAIN-INDEX.md` § Intent calibration. An undeclared intent defaults to `portfolio` (the suite's scaffold-default; the same as G-121's 7-core starter set). Declare deliberately — the wrong intent over-invests or under-invests methodology effort and the over-investment variant is hard to catch in-project because the methodology produces more findings (which feel like value) rather than fewer.

- **learning-exercise** — first attempts in a new technology; goal is learning, not shipping. Apprentice's first project in a language or framework. The IAR intensity is intentionally narrower than the scaffold default: 3 cores (SE, QE, SO) plus one rotating optional core (SA / Security / UX / Platform Engineer / Data Engineer, rotated across layers to expose the apprentice to different lenses). Stop-signal sensitivity high (G-151) — stop early when findings get hallucinated; the cost of one missed defect is low relative to the cost of process-drift fatigue.
- **portfolio** — apprenticeship/portfolio demonstrations meant for handoff and external review. Standard IAR (the 7-core default per G-121, plus Technical Writer if intended for external reading). Standard stop signal.
- **capstone** — apprentice-graduation-level work. Full 7-core + Performance Engineer + fresh-system install verification discipline (G-155). Standard stop signal.
- **production** — software meant for ongoing operational use by people other than the developer. Full 7-core + all extended domains meeting their activation criteria (Red Team if user input or network exposure, Privacy if user data, Accessibility if UI, Localization if multi-locale). Strict MVR enforcement.

**Declared intent for this project:** `<intent-level>`. Rationale: `<one sentence stating why this intent fits the project — e.g., "first Rust project; goal is learning the language, not shipping a tool" or "portfolio submission for the apprentice graduation review">`.

**Phase 5 strategy:** `<one-sentence Phase 5 declaration — required at capstone and production intents; optional at learning-exercise and portfolio. Choose: "not applicable — <rationale>" OR "planned — <named tooling and scope>." See DOMAIN-INDEX § Phase 5 / Phase 6 strategy declaration (G-162).>`

**Phase 6 strategy:** `<one-sentence Phase 6 declaration — required at capstone and production intents; optional at lower tiers. Choose: "not applicable — <rationale>" OR "planned — <which of the four axes converge and how>." See DOMAIN-INDEX § Phase 5 / Phase 6 strategy declaration (G-162).>`

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

How the project is proven correct. Test layers (unit, integration, end-to-end); the manual testing checklist authored alongside the layer plan in Phase 1c; the IAR cadence; any property-based or fuzz testing.

## Technology choices and rationale

The decisions that bound the implementation. Each choice with: what was chosen, what alternatives were considered, why this option fits the constraints. A choice without a rationale is an outcome record, not a decision — Phase 3 Technical Writer will catch it.

## Constraints

External constraints the implementation must respect: dependencies, platforms, performance budgets, accessibility minima, security posture, regulatory requirements, deployment target.

## Open questions

Items that need decision before Phase 1c can complete. An empty Open Questions section after a single Phase 1a+1b session is suspicious — the self-adversary check usually surfaces at least one ambiguity worth recording.
