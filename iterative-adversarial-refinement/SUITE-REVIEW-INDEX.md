# IAR Suite Review

The IAR suite is itself a software artifact. Like any artifact it has a specification (the VSDD and VDD methodology documents), a design (the domain structure, dimensions, supplement architecture, and session primer set), and an implementation (the domain prompt files, session primers, README, supplements/ supplements, and gap analysis log). The adversary should apply to the suite the same pressure it applies to projects under review — to both adversarial review prompts (does this dimension catch a real defect?) and constructive primers (does this primer prevent a real spec or process gap?). The suite has expanded beyond its original VSDD Phase 3 scope; see `README.md` `## Suite scope` for the artifact map.

This file is the index of suite review sessions. Individual session logs live in `review-log/`, organized as `YYYY-MM-DD-suite-review.md`. A suite review is a single artifact type — sessions vary in mode (defect-search lens vs. registry-walk lens), and the mode lives in each entry's Lens field rather than in a separate artifact type. See `prompts/suite-development.md` for the entry format.

Governing references:
- VSDD whitepaper: https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00
- VDD whitepaper: https://gist.github.com/dollspace-gay/45c95ebfb5a3a3bae84d8bebd662cc25
- Apprentice-onboarding: https://github.com/Navigators-Guild/apprentice-onboarding
- Crosslink: https://github.com/forecast-bio/crosslink

---

## Reading convention

Read individual sessions via the links below. **Do not read this file as a substitute for reading the linked session logs** — this file is an index only. Use `offset` and `limit` when a session file exceeds ~400 lines.

---

## Suite Reviews

| Review | Date | File | Lens / Scope summary |
|--------|------|------|----------------------|
| Review 37 | 2026-05-12 12:30Z | [2026-05-12-suite-review.md](review-log/2026-05-12-suite-review.md#review-37--2026-05-12-1230z) | Warm-finding-closure Red Gate framing (methodology gap lens). Surfaced by `issue-tracker-cli` Layer 7 IAR Round 3 VDD-IAR R19 F1: `prompts/implementation.md`'s L11/L32 (fails-first) + L56 (retroactive carve-out) framework does not name the IAR-Round-2+ warm-closure mode in which a previously-documented finding is closed by a commit bundling tests + implementation. Earned by R17 (single instance, Option B declined) → R19 (recurrence, Option B accepted). Project-scoped resolution at `issue-tracker-cli/iterative-adversarial-refinement/CLOSURE-PROTOCOL.md` §8. G-99 registered Deferred (natural-recurrence trigger for suite-level promotion). |
| Review 36 | 2026-05-06 03:30Z | [2026-05-06-suite-review.md](review-log/2026-05-06-suite-review.md#review-36--2026-05-06-0330z) | Adversarial-review-log self-disclosure / meta-leak (defect-class lens). Surfaced by `issue-tracker-cli` Layer 1 PROCESS.md retrospective. Three coordinated mitigations: confidentiality-aware citation rule in `prompts/review-session.md`; domain-specific reminders in PE + Security prompts; new suite-level hook `hooks/check-review-log-anonymization.sh` with public-URL allowlist, wired in `.pre-commit-config.yaml`. G-98 registered and Addressed. |
| Review 35 | 2026-05-05 20:30Z | [2026-05-05-suite-review.md](review-log/2026-05-05-suite-review.md#review-35--2026-05-05-2030z) | Manual-testing-checklist authoring quality (`prompts/decomposition.md`): runnable-step standard replaces shorthand bullets; explicit binary install/uninstall/reinstall lifecycle, clean-state markers, and expected-output assertions required. G-97 registered and Addressed. |
| Review 34 | 2026-05-03 23:30Z | [2026-05-03-suite-review.md](review-log/2026-05-03-suite-review.md#review-34--2026-05-03-2330z) | Apply G-90 (Phase 1 → Phase 1a after upstream-whitepaper check) and G-94 sub-issues 1, 2, 3, 5 (`lang/`→`supplements/`, DOMAIN-INDEX move, SUITE-REVIEW rename, primer H1 convention); sub-issue 4 deferred to spinoff-MVP; G-96 registered (whitepaper sub-phase semantic divergence) |
| Review 33 | 2026-05-03 23:00Z | [2026-05-03-suite-review.md](review-log/2026-05-03-suite-review.md#review-33--2026-05-03-2300z) | Bundled-deferral dependency analysis: G-90 and G-94 promoted from Deferred to Open (substance independent of `issue-tracker-cli` feedback); G-88, G-89, G-91, G-92, G-93, G-95 reviewed and remain Deferred |
| Review 32 | 2026-05-03 22:00Z | [2026-05-03-suite-review.md](review-log/2026-05-03-suite-review.md#review-32--2026-05-03-2200z) | Suite-review entry-format and deferral-trigger consistency: F1 (`### Coordination` heading authorized), F2 (`issue-tracker-cli` trigger defined), F3 (role-based `Lens` form added), F4 (session-isolation policy documented); 4 resolved, 2 hallucinated |
| Review 31 | 2026-05-03 18:00Z | [2026-05-03-suite-review.md](review-log/2026-05-03-suite-review.md#review-31--2026-05-03-1800z) | Five-lens adversarial pass (clarity, naming, ambiguity, consistency, transitional-state alignment) framed for standalone-repo spinoff; G-90–G-95 registered (Deferred); 2 hallucinated |
| Review 30 | 2026-05-03 12:00Z | [2026-05-03-suite-review.md](review-log/2026-05-03-suite-review.md#review-30--2026-05-03-1200z) | Suite scope and identity (SO + TW + VDD-IAR lenses); transitional state documented; G-87 registered and addressed |
| Review 29 | 2026-05-01 00:00Z | [2026-05-01-suite-review.md](review-log/2026-05-01-suite-review.md#review-29--2026-05-01-0000z) | VDD-IAR + VSDD alignment of all session primers; Red Gate commit step added to implementation.md |
| Review 28 | 2026-04-27 09:00Z* | [2026-05-01-suite-review.md](review-log/2026-05-01-suite-review.md#review-28--2026-04-27-0900z) | README Focus column accuracy (Security dims 7–8, SA extended); Phase 2a commit gap follow-up (Finding 6) |
| Review 27 | 2026-04-28 08:00Z | [2026-04-28-suite-review.md](review-log/2026-04-28-suite-review.md#review-27--2026-04-28-0800z) | Registry walk: G-22 (AI context drift) and G-30 (feature enhancement) addressed |
| Review 26 | 2026-04-28 07:00Z | [2026-04-28-suite-review.md](review-log/2026-04-28-suite-review.md#review-26--2026-04-28-0700z) | Registry walk: G-09, G-10, G-32 addressed; ownership decisions recorded for all remaining open gaps |
| Review 25 | 2026-04-28 06:00Z | [2026-04-28-suite-review.md](review-log/2026-04-28-suite-review.md#review-25--2026-04-28-0600z) | Registry walk: all 29 open gaps reviewed; G-84, G-85 (lang supplements) addressed; 4 gaps dismissed |
| Review 24 | 2026-04-28 05:00Z | [2026-04-28-suite-review.md](review-log/2026-04-28-suite-review.md#review-24--2026-04-28-0500z) | Governing standard format conformance; DE reviewer role variants |
| Review 23 | 2026-04-28 04:00Z | [2026-04-28-suite-review.md](review-log/2026-04-28-suite-review.md#review-23--2026-04-28-0400z) | review-session.md classification schema; VDD-IAR Alignment Deferred exclusion |
| Review 22 | 2026-04-28 03:00Z | [2026-04-28-suite-review.md](review-log/2026-04-28-suite-review.md#review-22--2026-04-28-0300z) | Format consistency; CHANGELOG.md "AIR→IAR"; lang supplement note accuracy |
| Review 21 | 2026-04-28 02:00Z | [2026-04-28-suite-review.md](review-log/2026-04-28-suite-review.md#review-21--2026-04-28-0200z) | Inter-domain coordination; lang supplement symmetry; PE coverage tooling (JS/TS) |
| Review 20 | 2026-04-28 01:00Z | [2026-04-28-suite-review.md](review-log/2026-04-28-suite-review.md#review-20--2026-04-28-0100z) | QE↔SE coordination gap; Portfolio Assessment dim 8 ownership framing |
| Review 19 | 2026-04-28 00:00Z | [2026-04-28-suite-review.md](review-log/2026-04-28-suite-review.md#review-19--2026-04-28-0000z) | lang supplement source citations; GAP-ANALYSIS-LOG consistency; cli.md contradiction |
| Review 18 | 2026-04-27 23:30Z | [2026-04-27-suite-review.md](review-log/2026-04-27-suite-review.md#review-18--2026-04-27-2330z) | Generalist pass; stale phase labels; coordination link format compliance |
| Review 17 | 2026-04-27 23:00Z | [2026-04-27-suite-review.md](review-log/2026-04-27-suite-review.md#review-17--2026-04-27-2300z) | VDD-IAR philosophy alignment; dim 7 feedback routing fidelity added |
| Review 16 | 2026-04-27 22:00Z | [2026-04-27-suite-review.md](review-log/2026-04-27-suite-review.md#review-16--2026-04-27-2200z) | Phase numbering correction (4→3); same-model review limitation documented |
| Review 15 | 2026-04-27 21:00Z | [2026-04-27-suite-review.md](review-log/2026-04-27-suite-review.md#review-15--2026-04-27-2100z) | Full suite pass; regression check ordering across 5 domains; governing standard carve-outs |
| Review 14 | 2026-04-27 20:00Z | [2026-04-27-suite-review.md](review-log/2026-04-27-suite-review.md#review-14--2026-04-27-2000z) | Full adversarial pass; sycophancy checks; Localization/Privacy/Portfolio Assessment classification |
| Review 13 | 2026-04-27 | [2026-04-27-suite-review.md](review-log/2026-04-27-suite-review.md#review-13--2026-04-27) | Full suite pass against governing standard; spec-crystallization + decomposition primers |
| Review 12 | 2026-04-27 | [2026-04-27-suite-review.md](review-log/2026-04-27-suite-review.md#review-12--2026-04-27) | Production slop pass; 22 dimensions added across QE, Security, UX, SE, SA, DE, PE |
| Review 11 | 2026-04-27 | [2026-04-27-suite-review.md](review-log/2026-04-27-suite-review.md#review-11--2026-04-27) | Initial review; session primers created; VSDD purity boundary map; VSDD pipeline in README |
| Review 10 | 2026-04-27 | [2026-04-27-suite-review.md](review-log/2026-04-27-suite-review.md#review-10--2026-04-27) | Registry walk: 8 new extended domains drafted (Performance, Accessibility, Privacy, Observability, API Contract, Documentation, Portfolio Assessment, Localization) |
| Review 9 | 2026-04-27 | [2026-04-27-suite-review.md](review-log/2026-04-27-suite-review.md#review-9--2026-04-27) | Full adversarial roast; G-58–79 addressed across all domains and primers |
| Review 8 | 2026-04-27 | [2026-04-27-suite-review.md](review-log/2026-04-27-suite-review.md#review-8--2026-04-27) | Meta-adversarial review; G-56 (purity boundary map); SUITE-REVIEW.md + prompts/ created |
| Review 7 | 2026-04-27 | [2026-04-27-suite-review.md](review-log/2026-04-27-suite-review.md#review-7--2026-04-27) | VSDD alignment; G-53 (spec completeness), G-54, G-55 registered; dim 11 + SO/QE updates |
| Review 6 | 2026-04-27 | [2026-04-27-suite-review.md](review-log/2026-04-27-suite-review.md#review-6--2026-04-27) | TDD enforcement; G-52 addressed; VDD-IAR dim 4 hardened; QE dim 14 added |
| Review 5 | 2026-04-26 01:00Z | [2026-04-26-suite-review.md](review-log/2026-04-26-suite-review.md#review-5--2026-04-26-0100z) | Suite organization; G-46–51 addressed; VDD-IAR Alignment domain created |
| Review 4 | 2026-04-26 00:00Z | [2026-04-26-suite-review.md](review-log/2026-04-26-suite-review.md#review-4--2026-04-26-0000z) | Portfolio/apprentice alignment; G-39–45 addressed; hallucinated classification added |
| Review 3 | 2026-04-25 22:00Z | [2026-04-25-suite-review.md](review-log/2026-04-25-suite-review.md#review-3--2026-04-25-2200z) | Personal developer context; G-33–38 registered; sycophancy checks added to all domains |
| Review 2 | 2026-04-25 21:30Z | [2026-04-25-suite-review.md](review-log/2026-04-25-suite-review.md#review-2--2026-04-25-2130z) | Consulting firm context; AI workflow gaps G-20–25; consulting role gaps G-18, 26–32 |
| Review 1 | 2026-04-25 20:00Z | [2026-04-25-suite-review.md](review-log/2026-04-25-suite-review.md#review-1--2026-04-25-2000z) | Initial gap analysis; mission-critical and speculative project contexts; G-01–17 registered |

*Review 28's header date stamp is incorrect — Finding 6 was added in the 2026-05-01 session and the entry is filed alongside Review 29.

---

## Migration footnote — 2026-05-02

This file was reorganized on 2026-05-02. Prior to this date, suite reviews were tracked in two parallel sequences: **Suite Meta-Reviews** (Review 1–16) and **Gap Analysis Runs** (Run 1–13). Sessions with each format were filed in `review-log/YYYY-MM-DD-{meta-review|gap-analysis}.md`. The collapse to a single artifact type (per the standard in `prompts/suite-development.md`) renumbered all 29 sessions chronologically as Review 1–29 and renamed the underlying files to `YYYY-MM-DD-suite-review.md`. The mapping from old identifier to new identifier:

| Old | New | | Old | New |
|-----|-----|-|-----|-----|
| Run 1 | Review 1 | | Review 1 | Review 11 |
| Run 2 | Review 2 | | Review 2 | Review 12 |
| Run 3 | Review 3 | | Review 3 | Review 13 |
| Run 4 | Review 4 | | Review 4 | Review 14 |
| Run 5 | Review 5 | | Review 5 | Review 15 |
| Run 6 | Review 6 | | Review 6 | Review 16 |
| Run 7 | Review 7 | | Review 7 | Review 17 |
| Run 8 | Review 8 | | Review 8 | Review 18 |
| Run 9 | Review 9 | | Review 9 | Review 19 |
| Run 10 | Review 10 | | Review 10 | Review 20 |
| Run 11 | Review 25 | | Review 11 | Review 21 |
| Run 12 | Review 26 | | Review 12 | Review 22 |
| Run 13 | Review 27 | | Review 13 | Review 23 |
| | | | Review 14 | Review 24 |
| | | | Review 15 | Review 28 |
| | | | Review 16 | Review 29 |

Cross-references in `GAP-ANALYSIS-LOG.md`, `CHANGELOG.md` (only newest entries; historical entries preserved as-is), and within session entries themselves were updated to the new numbering. Historical references in commit messages and external documents are out of scope.
