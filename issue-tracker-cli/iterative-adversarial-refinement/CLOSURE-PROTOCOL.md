# IAR Closure Protocol

This document codifies how findings move through their lifecycle and who has authority to close them. It exists because two patterns recurred during Layer 3 IAR rounds and were flagged as process gaps:

- **VDD-IAR Review 10 Finding 1** (process side): a non-SO domain modified `DESIGN.md` directly. The change was content-correct (later ratified by SO Review 13), but the authority chain was inverted — only SO is authorized to edit the spec. Without an explicit authority record, the violation was invisible until VDD-IAR caught it after the fact.
- **VDD-IAR Review 10 Finding 2** (closure mechanics): findings were closing by director judgment at merge time without an explicit protocol. The Layer 2 carry-forward Open items pattern repeated at Layer 3. SO Review 14 surfaced a related second-order pattern: long-running Open findings can float across many reviews if no one invokes the SO veto explicitly.

This protocol is project-scoped (lives inside `issue-tracker-cli/iterative-adversarial-refinement/`). It is a candidate for promotion to the suite-level IAR documentation if other projects find it useful — see "Suite adoption" at the end.

---

## 1. Domain authority over project artifacts

Each project artifact has exactly one domain that may modify it. Other domains identifying a needed change must classify the finding as **Raised to SO** (or the appropriate authority-domain) and document the proposed change. They do not apply the change.

| Artifact | Modify authority | Other domains may |
|---|---|---|
| `DESIGN.md` | **Solution Owner only** | Raise findings as "Raised to SO" with proposed text |
| `DECISIONS.md` | Solution Owner (primary); any domain (with rationale citing a specific review finding) | Append a new entry citing the source review finding |
| `TODO.md` | Solution Owner (scope); director (sequencing) | Suggest re-ordering or new layers as Backlogged findings |
| `CHANGELOG.md` | Any domain that produced the change being recorded | Append a new entry under the current layer |
| `PROCESS.md` retrospective placeholders | **Developer only** (human-authored) | Flag empty placeholders as findings; never fill them |
| `README.md` | Technical Writer; any domain (for accuracy fixes) | Edit directly to correct stale claims |
| `Cargo.toml`, `Cargo.lock` | Software Engineer (deps); Platform Engineer (CI metadata); Solution Owner (`license`, `description`) | Raise as "Raised to" the appropriate domain |
| `src/**/*.rs` | Software Engineer (primary); Quality Engineer (tests); Security (CVE fixes) | Raise as findings; do not apply directly |
| `tests/**/*.rs` | Quality Engineer (primary); Software Engineer (parity with code changes) | Raise as findings |
| `deny.toml`, `.github/workflows/*.yml`, `.pre-commit-config.yaml`, `.pre-commit-hooks/*` | Platform Engineer | Raise as findings |
| `iterative-adversarial-refinement/<DOMAIN>-REVIEW.md` | The owning domain only (each domain owns its own log) | Cross-reference in their own log |
| `iterative-adversarial-refinement/CLOSURE-PROTOCOL.md` (this file) | VDD-IAR Alignment (process changes); Solution Owner (scope changes); director (final word) | Propose amendments via VDD-IAR Alignment finding |

**Enforcement.** No automated gate currently blocks an authority violation — the SE Review 9 incident demonstrates this. The protective controls are:

1. **The reviewing agent's prompt** — every domain prompt now includes (or must include) the "Raised to" classification rule for cross-authority changes. SO and SA prompts already do.
2. **VDD-IAR Alignment review** — catches violations after the fact. This is the current backstop.
3. **Future:** a pre-commit hook scanning the diff for `DESIGN.md` changes when the commit message does not include `SO Review` could provide a build-time gate. Not implemented as of Layer 3; raised here for future Platform Engineer consideration.

---

## 2. Finding lifecycle

Every finding occupies exactly one state. States are terminal except as noted. Transitions require evidence.

```
            ┌───────┐
       ┌──→ │ Open  │ ──┬── Resolved
       │    └───┬───┘   ├── Dismissed
       │        │       ├── Hallucinated
   carry-       │       ├── Backlogged       (Solution Owner only)
   forward      │       ├── Approved deviation (with stakeholder approval)
       │        ▼       ├── Accepted risk    (Security / Red Team / Privacy only)
       │   Raised to    ├── Accepted deviation (Accessibility only)
       │   <DOMAIN>     ├── Accepted limitation (Performance Engineer only)
       │        │       ├── Accepted scope    (Localization only)
       │        ▼       ├── Deferred         (must name a specific future layer; NOT valid for Security, Red Team, or VDD-IAR)
       │   <other       │
       │   domain       └── (other domain re-raises if not adjudicated)
       │   adjudicates)
       └────────┘
```

**Transition rules:**

- **Open → Resolved.** Requires (a) the change applied (with file:line if a code/spec change), (b) a regression test or equivalent verification (for code-class findings), and (c) the closure recorded in the log of the domain that raised the finding. Cross-domain duplicates close on a single resolution that addresses the shared root cause; the closing domain notes the cross-domain coverage in its Coordination section.
- **Open → Dismissed.** Requires explicit rationale. "Not applicable" is not sufficient — the rationale must explain *why* the concern does not apply to this project. Re-raise condition is documented if the dismissal is conditional.
- **Open → Hallucinated.** Requires the reviewer to demonstrate specifically why the control holds or the concern does not apply. Marking a finding hallucinated by reflex (without demonstration) is itself a sycophancy failure.
- **Open → Backlogged.** Solution Owner only. Records the finding for future consideration with explicit re-raise conditions.
- **Open → Deferred.** Requires naming a specific future layer. **Not valid for Security, Red Team, or VDD-IAR Alignment** (per IAR README sequencing rules — security findings are not deferred; process findings are binary).
- **Open → Raised to <DOMAIN>.** Used when the original domain identifies a change but lacks authority to apply it. The receiving domain adjudicates in its next review and converts to one of the terminal states. The finding remains tracked in BOTH logs until adjudicated; the originating log shows "Raised to X (pending)" and the receiving log shows the adjudicated state.

---

## 3. Long-running Open findings — auto-Backlog rule

A finding that has been Open across **three consecutive reviews of the originating domain** without adjudication by the receiving authority should be auto-Backlogged by the originating domain at the start of the third subsequent review, with the original finding text plus a "carry-forward" annotation. This prevents the indefinite-Open pattern observed in Platform Engineer Review 8 Finding 3 (coverage tooling), which floated across Reviews 1, 2, 3, 5, 7, and 8 before SO Review 14 finally adjudicated.

The auto-Backlog is reversible: if the receiving authority later adjudicates, the finding moves out of Backlogged into the appropriate terminal state. The point of the rule is to surface "this question has not been answered" as an explicit Backlog entry rather than as silent log noise.

Counter-rule: **Security**, **Red Team**, and **VDD-IAR Alignment** findings do not auto-Backlog. Process and security findings carry forward as Open until explicitly resolved; their visibility is the closure mechanism.

---

## 4. Cross-domain duplicates

When two or more domains raise findings with the same root cause (e.g., the SIGPIPE panic was raised independently as UX F1 and Security F1), the resolution applies once. Tracking convention:

1. Each domain that raised the finding closes it in its own log with a cross-reference to the closing change (`closed by <other-domain> Review N Finding M`, or `closed by Layer N follow-up resolution pass with file:line citation`).
2. The Coordination section of the closing entry lists all linked domains.
3. The CHANGELOG entry lists all linked domain findings together (e.g., `Closes UX F1 + Security F1`).

This avoids the alternative — opening a single "canonical" tracking entry that all domains link to — which adds bookkeeping overhead disproportionate to the benefit at this project's scale.

---

## 5. Parallel cold-batch + warm sequential resolution cadence

Observed effective IAR cadence for Layer 3, recommended as standard:

1. **Cold-session parallel review batch** — one fresh agent session per domain, all running in parallel, none sharing context with the others or with the build session. This is the gold standard per the `prompts/review-session.md` primer. Produces independent adversarial pressure across domains; surfaces real findings that same-session passes miss.
2. **Warm sequential resolution pass** — a single orchestrator session (typically the build session, after the cold batch completes) reads all the cold-batch findings, identifies cross-domain duplicates, applies fixes coherently, runs the test suite end-to-end, and writes the Update entries to each affected domain log. The asymmetry is real: adversarial pressure benefits from parallel cold sessions; resolution coherence benefits from a single session that can sequence dependent edits.
3. **SO adjudication round (when spec or scope decisions are pending)** — a separate Solution Owner session, ideally cold but optionally warm-with-self-applied-sycophancy-guard, processes any "Raised to SO" findings and produces explicit Backlogged / Dismissed / Resolved / Approved-deviation calls.
4. **VDD-IAR Alignment closure round** — the meta domain runs last to verify the process record is complete: every finding has a terminal state or an auto-Backlog, every cross-domain duplicate is linked, the CHANGELOG accurately reflects what changed.

A round that skips step 4 is incomplete. A round that conflates step 1 with step 2 (an agent that reviews a finding it then immediately resolves) loses adversarial pressure. A round that skips step 3 leaves Raised-to-SO findings indefinitely Open, triggering the auto-Backlog rule.

---

## 6. Merge gate

A layer may be merged to `main` when all of the following hold:

1. Every active IAR domain has completed at least one cold-session pass on this layer.
2. The cold-batch + warm-resolution + SO-adjudication + VDD-IAR-closure cadence has run at least once.
3. No finding remains in **Open** state. Every finding is in one of the terminal states (Resolved, Dismissed, Hallucinated, Backlogged, Approved deviation, Accepted risk, Accepted deviation, Accepted limitation, Accepted scope, Deferred, Demonstrated/Partial/Absent for Portfolio).
4. CHANGELOG accurately describes what changed.
5. Cargo build, test, clippy, and fmt are green with `--locked`.
6. Any DESIGN.md changes during the layer have explicit SO authorship or SO ratification recorded in the SO log.
7. PROCESS.md retrospective for the layer is at least started (developer-only — empty placeholders block portfolio assessment but not technical merge).

The director is the final adjudicator on (3) — the closure protocol does not eliminate director judgment, it makes it explicit. A finding that would otherwise block merge can be Backlogged or Deferred (per scope/timing rules above) by SO with explicit rationale, allowing merge to proceed.

---

## 7. Suite adoption

If this protocol proves useful at the project level, the natural next step is promotion to the suite-level IAR documentation:

- Move this file to `guild-portfolio/iterative-adversarial-refinement/CLOSURE-PROTOCOL.md`.
- Add a row to `guild-portfolio/iterative-adversarial-refinement/SUITE-REVIEW-INDEX.md` recording the adoption.
- Add a corresponding `review-log/YYYY-MM-DD-suite-review.md` entry per `prompts/suite-development.md`.
- Update each domain prompt in `domains/role/` to reference the protocol's authority table (Section 1) explicitly.

The project-scoped version stands on its own — it does not require suite adoption to be useful here.

---

## Change history

- **2026-05-05** — Initial draft. Closes VDD-IAR Review 10 Finding 1 (process side, authority chain documentation) and Finding 2 (closure mechanics). Records the cadence pattern observed during Layer 3 round-2 and the auto-Backlog rule from SO Review 14 Coordination notes.
