# External Review — @dollspace-gay — 2026-05-20

## Reviewer

**Handle:** [@dollspace.gay](https://bsky.app/profile/dollspace.gay) (Bluesky) / [@dollspace-gay](https://github.com/dollspace-gay) (GitHub) — consistent identity across both platforms (per the [external-review-log § Identity-correlation discipline](../../suite-development.md): naming both handles is acceptable because the handle-string is the same across platforms after slug-normalization; this is consistent-identity surfacing, not correlation between separate identities).
**Pronouns:** it/its
**Relationship to suite:** Primary author and maintainer of [crosslink](../../../crosslink-contract.md), chainlink, the [VSDD whitepaper](https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00), the [VDD-IAR whitepaper](https://gist.github.com/dollspace-gay/45c95ebfb5a3a3bae84d8bebd662cc25), and the apprentice onboarding course. **Comments from this reviewer carry extremely high signal** per the operator's framing — it is the upstream methodology author of every dependency-surface the suite is downstream of. Canonical external-feedback Source-value precedent named in [primer 3 § Source attribution](../../../primers/3-review-session.md) ("dollspace-gay's `message-4.txt` evaluation of ITC, mined in Review 51, is the canonical example").

## Source

**Type:** Prose value-add review (operator-shared text artifact)
**URL:** *(not publicly hosted; archived in this markdown file)*
**Captured:** 2026-05-20 ~18:32 UTC (Pacific time before 11:32 AM PDT), shared with the operator.
**Archive provenance:** This markdown file IS the canonical archive. The original `.txt` source artifact at `vsdd-suite/suite-development/review-log/2026-05-20-crosslink-value-add-review.txt` was promoted to this form per [Review 88](../2026-05-21-suite-review.md#review-88--2026-05-21-1330z) (external-review-log subfolder pattern + reviewer-named-file convention + markdown conversion); the `.txt` is retired in favor of this file.
**Verbatim?:** Yes — the full source text from the `.txt` file is preserved below verbatim.

## Scope of what the reviewer addressed

Value-add evaluation of `vsdd-suite-with-crosslink integration` vs `crosslink alone`. The reviewer is both the VSDD whitepaper author and the crosslink CLI author — uniquely positioned to evaluate (a) where vsdd-suite adds value over crosslink alone, (b) where crosslink could absorb suite concepts, (c) the reverse signal of what the suite explicitly disclaims as crosslink's strategic centerpiece. The review identifies 7 crosslink CLI bugs/UX papercuts discovered through vsdd-suite usage + 5 absorbability concepts crosslink could ship as optional add-ons + 2 validation findings + 1 reverse-signal observation.

## Verbatim source content

> Value-adds for crosslink
>
> 1. Direct bugs / CLI gaps it discovered (these are essentially free issues)
>
> From crosslink-contract.md § Known limitations:

| Item | Fix in crosslink |
|---|---|
| `crosslink milestone create --quiet` still prints `Created milestone #N: <title>` instead of just the ID | Make `--quiet` actually quiet — parity with `crosslink quick --quiet` |
| `milestone add/show/close` accept only numeric IDs, but error message is `invalid digit found in string` (bad UX) | Either accept names, or give a friendlier error pointing at `milestone list` |
| `swarm gate <slug>` silently requires prior `swarm init --doc` — not discoverable | Better error: "run `swarm init` first" |
| `swarm review --doc <PATH>` flag name is ambiguous (people read it as the input prompt, it's the output) | Rename to `--output` / `--report` (with `--doc` deprecated alias) |
| `issue list -l` only filters by single label | Allow repeated `-l` with AND semantics |
| `knowledge import` errors with `"Sync cache not initialized"` instead of auto-initializing | Auto-run `knowledge sync` on first import |
| `crosslink import` is JSON-only | Document it, or accept markdown/yaml |

> These are real, named, reproducible UX papercuts a downstream user already hit.
>
> 2. Pre-commit hook worth porting in
>
> `hooks/check-crosslink-references.sh` (Python) — parses staged files for `crosslink <subcommand> --<flag>` patterns and validates each against `crosslink <subcommand> --help`. Catches hallucinated commands/flags before commit. This is exactly the kind of self-validating hook crosslink should ship as part of `crosslink init` for any project that documents crosslink usage (docs sites, design docs, CLAUDE.md, etc.). The other three hooks are project-specific (anonymization, suite-review preamble, changelog currency) and less generalizable.
>
> 3. A structured-label convention that crosslink could promote to first-class
>
> The suite's "G-138 finding-index" uses a label-axis scheme: `domain:<slug>`, `layer:N`, `round:N`, `finding:<N>`, `classification:<class>`, `source:<source>`, `route:phase-<N>`. They acknowledge multi-axis filter (AND across labels) is missing and they fall back to `--json | jq`. crosslink could:
>
> - Add typed/namespaced labels as a real concept (validation, autocomplete, indexed query)
> - Add `issue list -l a -l b` with AND semantics
> - Ship a `--label-schema` config so a project declares its label axes once
>
> 4. Concepts crosslink doesn't have but could ship as optional add-ons

| Concept | Where in vsdd-suite | Worth absorbing? |
|---|---|---|
| Phase primers (red-gate, decomposition, refactor, adversarial-review, feedback-integration, formal-hardening, convergence) as skills | `primers/*.md` | crosslink already ships design/qa/review-pre-commit/feature — adding a TDD-style `/red-gate` and `/refactor` skill would close obvious gaps |
| 16-domain adversarial review prompt set (SE/QE/UX/Security/SA/SO/PE/DE/Red-Team/Perf/TechWriter/A11y/Privacy/L10n + 2 meta) with per-domain dimensions and finding classification schemas | `domains/role/*.md`, `domains/meta/*.md` | crosslink's qa skill is one generic reviewer. `swarm review` could ship with this 16-prompt library as `knowledge import`-able defaults, so `swarm review --agents 7 --mandate adversarial` automatically distributes the 7 core domains |
| Language × interface supplements that compose with domain prompts | `supplements/{rust,javascript-typescript,cli,browser-app}.md` | crosslink already has per-language `rules/*.md` — the composition with review domain is the new idea |
| "Forward-only compatibility" doctrine in COMPATIBILITY.md | doc | Worth borrowing for crosslink's own breaking-change discipline |
| Cold-context dispatch (every Phase 3 reviewer runs in a fresh session, primer pasted explicitly) | `primers/3-review-session.md` | crosslink's `swarm review --agents N` already does this via worktree isolation — the doctrine is matched, just not named that way |

> 5. The reverse signal: things vsdd-suite explicitly disclaims
>
> They say `crosslink kickoff` is out of scope because they use `swarm` instead. They also disclaim `container`, `sentinel`, `style`, `mc`, `tui`, `trust`, `locks`, `migrate`, `context`, `integrity`, `compact`, `prune`, `timer`. Useful confirmation that the `swarm` surface is the methodological centerpiece — if you have to choose where to invest CLI ergonomics work, that's it.

## Suite-side mining

This external review was mined into the suite-side audit trail at **[Review 85](../2026-05-21-suite-review.md#review-85--2026-05-21-1130z)** (PR [#40](https://github.com/magnificentlycursed/guild-portfolio/pull/40)). The mining was deferred ~24 hours because the artifact's original `.txt` filename + non-canonical structure didn't trigger the suite-review hook's per-date pattern; the operator surfaced the artifact's status during PR #40 and the mining followed.

Per-finding routing:

| Reviewer observation | Suite-side classification | Routing |
|---|---|---|
| 7 crosslink CLI bugs (`milestone --quiet` partial; `milestone` IDs-only; `swarm gate` discoverability; `swarm review --doc` naming; `issue list -l` single-label; `knowledge import` auto-init missing; `crosslink import` JSON-only) | Upstream crosslink issues — NOT suite-side fixes | Operator-action queue: file 7 upstream issues in crosslink's issue tracker |
| 5 absorbability concepts (Phase primers as skills; 16-domain prompt set; language × interface supplements; forward-only compatibility doctrine; cold-context dispatch naming) | Upstream crosslink coordination asks — validation of the suite's design | Operator-action queue: file 5 upstream coordination asks |
| `check-crosslink-references.sh` hook worth porting | Validation finding — suite hook design validated | No suite-side action; consider whether `crosslink init` should ship this hook as a default |
| Reverse-signal disclaim list validates `swarm` as crosslink's methodological centerpiece | Validation finding — suite disclaim-list design validated | No suite-side action |

Per the [`suite-development.md`](../../suite-development.md) § External dependency references discipline ("only file upstream coordination asks the suite's owner has authority to own"), all 13 upstream items route to the operator-action queue for filing in crosslink's issue tracker — outside the vsdd-suite audit trail. The reviewer is also the crosslink author, so it has authority over the filings.

## Notes

- The vsdd-suite design is **validated by the methodology author**. Where the reviewer names absorbability concepts, the suite's design is the canonical source the absorption would copy from. Where the reviewer names CLI bugs in crosslink, the suite IS the surface that discovered them — methodology-vindication.
- The mining-Review 85 explicitly notes the sycophancy compensation: "resisted treating 'the methodology author validated our design' as load-bearing-positive (validation is signal not exemption from future adversarial pressure)."
- Capture date 2026-05-20 ~18:32 UTC is co-temporal with [Review 80](../2026-05-20-suite-review.md#review-80--2026-05-20-1830z) (Documentation Reviewer registration + GitHub Docs Style Guide adoption + Three-audience design principle codification). The temporal coincidence is documented per [Review 85](../2026-05-21-suite-review.md#review-85--2026-05-21-1130z).
