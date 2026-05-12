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

## 8. Warm-finding-closure Red Gate carve-out

VDD-IAR Alignment Review 19 Finding 1 (Layer 7 IAR Round 3) raised that
the methodology's Red Gate framework — as written in
`iterative-adversarial-refinement/prompts/implementation.md` L11/L32/L56
— defines two states for a new test:

1. **Phase 2a Red Gate**: test fails first, implementation makes it pass.
2. **Retroactive Red Gate (L56 carve-out)**: test discovered during
   Phase 2b, added post-implementation, labelled with the literal
   `// retroactive Red Gate: <behavior> — discovered during Phase 2b,
   test added post-implementation, confirmed passes against current
   implementation.` source comment.

Both modes assume the work-in-progress context of a fresh layer. Neither
fits cleanly for the **warm-finding-closure** pattern that recurs during
IAR Round 2+ cadence: a previously-documented Open / Deferred /
Backlogged IAR finding is closed via a single commit that bundles new
tests with the implementation change because the resolution requires
both (a refactor + its regression tests; a defense-in-depth assertion +
its test; a new helper extraction + the test that pins its contract).

Layer 7 IAR Round 1 closure (`fbbb8a3`) applied the literal retroactive
Red Gate label for VDD-IAR R17 F1 Option A. Layer 7 IAR Round 3 commits
(`c341a54`, `bd7511e`, `3fa1f3c`) did NOT — the resolution-bundled tests
were added in the same commit as their target implementation without
the label, on the framing that "warm closure of a documented finding"
is a different mode than "test discovered during Phase 2b" (L56's
framing). VDD-IAR R19 F1 flagged the inconsistency; the closure
options it offered were:

- **Option A** — apply the retroactive label retrofit across the 17
  affected R3 test bodies (matches L56 literally).
- **Option B** — codify the warm-finding-closure mode as a distinct
  carve-out (a permanent CLOSURE-PROTOCOL.md amendment), earned by the
  recurrence between R17 and R19.

VDD-IAR R17 F1 had declined Option B with "a permanent rule change
should be earned by recurrence, not pre-empted by a single instance."
R19's recurrence on the polish-layer / warm-closure pattern is the
earning event. **This section is the Option B amendment.**

**Carve-out:** A commit that resolves a previously-documented IAR
finding (one already logged as Open, Deferred, Backlogged, or
Raised-to-{domain} in an IAR review file under
`{project}/iterative-adversarial-refinement/<DOMAIN>-REVIEW.md`) MAY
bundle new tests with their target implementation in a single commit
without the literal `// retroactive Red Gate:` source comment, when
ALL of the following hold:

1. The commit message explicitly cites the originating finding by
   domain + review number + finding number (e.g.,
   "QE R17 F5 closure"). This serves the disclosure purpose the L56
   label served.
2. The test additions are scoped to **regression coverage of the
   closure** — pinning the closure's contract so the finding cannot
   silently reopen. Tests that exercise new behavior unrelated to the
   finding remain subject to the standard Red Gate (Phase 2a fails
   first).
3. The finding's resolution genuinely requires bundling. Indicators:
   the resolution introduces a new symbol whose existence is what the
   test asserts (extraction tests); the resolution adds a `debug_assert!`
   whose firing condition is what the test triggers (assertion tests);
   the resolution adds a parameter / enum whose values the test
   enumerates (refactor tests).

**Scope limits.** This carve-out does NOT apply to:

- New features or new ACs (Phase 2a Red Gate still required).
- Layer-introductory work (the layer's primary Red Gate commit at
  Phase 2a → Phase 2b boundary must satisfy L11/L32 literally).
- Findings not previously logged in an IAR review (a developer noticing
  a gap and writing a test + fix in one commit without a logged
  finding still owes the retroactive Red Gate label per L56 — the
  prior-logging requirement is the difference between "warm closure"
  and "discovered during Phase 2b").
- Findings logged as Open within the SAME commit's prior history (the
  finding must have been logged in a separate, earlier commit;
  same-commit "log a finding then close it" collapses the two phases
  and forfeits the carve-out).

**Author obligation.** The originating-finding citation is the
auditable disclosure. The VDD-IAR Alignment closure round (Section 5
step 4) verifies the citation is real by reading the cited finding and
confirming the commit's diff is plausibly within the finding's stated
scope. A citation that does not match the cited finding's scope is a
Dim 4 process violation regardless of the carve-out's other conditions.

**Suite-level gap.** The methodology gap this carve-out addresses
(implementation.md's L56 retroactive Red Gate framing does not cleanly
describe warm-finding-closure) is registered as
[G-99](../../../iterative-adversarial-refinement/GAP-ANALYSIS-LOG.md)
for possible suite-level promotion. This project-scoped carve-out
stands on its own (per Section 7's "the project-scoped version stands
on its own — it does not require suite adoption to be useful here"
posture).

---

## Change history

- **2026-05-05** — Initial draft. Closes VDD-IAR Review 10 Finding 1 (process side, authority chain documentation) and Finding 2 (closure mechanics). Records the cadence pattern observed during Layer 3 round-2 and the auto-Backlog rule from SO Review 14 Coordination notes.
- **2026-05-12** — Section 8 (Warm-finding-closure Red Gate carve-out) added per VDD-IAR Review 19 Finding 1 closure (Option B). The earning event was the recurrence of the pattern between Layer 7 R17 F1 (single instance, Option B declined) and Layer 7 R19 F1 (second instance: the R3 commits `c341a54`, `bd7511e`, `3fa1f3c` bundled tests with implementation for warm-finding closures without the L56 retroactive Red Gate label). Suite-level methodology gap registered as G-99 in `iterative-adversarial-refinement/GAP-ANALYSIS-LOG.md`.
