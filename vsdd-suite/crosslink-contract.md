# Crosslink Dependency Contract

The VSDD suite teaches projects to apply the Solution Architect External Interface Contracts dimensions (Dims 13–22 in [`domains/role/SOLUTION-ARCHITECT-REVIEW.md`](domains/role/SOLUTION-ARCHITECT-REVIEW.md)). This file is the suite's own application of those dimensions to its dependency on the [crosslink](https://github.com/forecast-bio/crosslink) CLI.

**Reason this file exists:** registered as G-118 in [`suite-development/FINDINGS-INDEX.md`](suite-development/FINDINGS-INDEX.md) by Review 41 (Solution Architect lens, dogfooding gap). The worked example in [`README.md`](README.md) § Worked example invokes 8+ crosslink commands with specific flags; this file makes the dependency surface explicit.

---

## Tested-against version

**crosslink v0.8.0** — every command and flag in this file was verified against `crosslink <subcommand> --help` output on 2026-05-17 (Review 46 + 47 verification pass). Updates to the crosslink CLI surface require re-validating the worked example against the new version and updating this contract file.

The crosslink-dependent portion of the suite is the **[+crosslink] enhancement path only**. The suite was designed for manual operation first; every step in the worked example has a manual-path version above the [+crosslink] block. Projects that do not use crosslink are unaffected by this contract.

## Dependency surface — commands the suite invokes

The worked example invokes these crosslink commands. Each is part of the contract; a change to any of them in crosslink may require an update to the suite's worked example.

| Phase | Command(s) | Used for | Required flags |
|---|---|---|---|
| Setup | `crosslink init` | Initialize `.crosslink/`, issues.db, embedded policy | (none) |
| Setup | `crosslink workflow diff` | Verify deployed policy matches embedded defaults | (none) |
| Setup | `crosslink agent --help` | Identity setup discovery | (none) |
| Setup | `crosslink knowledge import <DIRECTORY> --tag <tag>` | Register suite primers, activated domain prompts, and supplements as knowledge pages (G-146 / G-163 / G-164; invoked by `templates/scaffold-project.sh`) | `--tag`, positional `<DIRECTORY>`; `--overwrite` for re-import on suite version bump |
| Setup | `crosslink knowledge sync` | Initialize the knowledge cache when `knowledge import` reports `Sync cache not initialized` | (none) |
| 1a | `crosslink design "<desc>"` | Open Phase 1a+1b session container with `.design/<slug>.md` working draft | (none) |
| 1a | `crosslink design --continue <slug>` | Resume the Phase 1a+1b draft when a Phase 4 route brings work back | `--continue` |
| 1b | `crosslink quick "<title>" -p <pri> -l <label> [--parent <id>] [--quiet]` | Create epic, layer issue, acceptance criterion | `-p`, `-l`, `--parent`, `--quiet` |
| 1b | `crosslink milestone create "<name>"` | Create per-layer milestone container | (none) |
| 1b | `crosslink milestone add "<name>" "<issue-id>"` | Attach layer issue to its milestone | (none) |
| 1b | `crosslink milestone show "<name>"` | Verify layer container is populated | (none) |
| 1b | `crosslink issue comment "<id>" "<text>"` | Attach Red Gate test plan to layer issue | (none) |
| 2a | `crosslink session start` | Open implementation session | (none) |
| 2a | `crosslink session work "<id>"` | Mark active focus issue | (none) |
| 2b | `crosslink swarm gate <slug>` | Run project test suite as layer gate | (positional `<slug>`) |
| 3 | `crosslink swarm review --agents <N> --mandate adversarial --file-issues --doc <path>` | Launch N parallel cold-context adversaries | `--agents`, `--mandate`, `--file-issues`, `--doc` |
| 3 | `crosslink issue list -l <label> -s <status>` | Inspect filed findings; single label per call; `-s` accepts `open`/`closed`/`all` (default `open`) | `-l`, `-s` |
| 3 | `crosslink issue comment <id> "<text>" --kind <kind>` | Classification rationale before close; `--kind` accepts `note`/`plan`/`decision`/`observation`/`blocker`/`resolution`/`result`/`handoff`/`human` (default `note`) | `--kind` (optional but recommended) |
| 3 | `crosslink issue close <id>` | Close a classified finding (positional ID only; closure rationale lives in a prior `issue comment` per the close-after-comment pattern) | (positional `<id>`) |
| 4 | `crosslink issue label <id> <label>` | Apply route label (positional ID + positional label) | (both positional) |
| 4 | `crosslink issue unlabel <id> <label>` | Remove a route label | (both positional) |
| 4 | `crosslink issue block <id> <blocker>` | Block layer issue on a routed Phase-1a finding (`<id>` is the blocked issue; `<blocker>` is the blocking issue) | (both positional) |
| 4 | `crosslink swarm fix --from-label <label> --budget-aware` | Dispatch fix agents for routed cohort | `--from-label`, `--budget-aware` |
| Loop | `crosslink milestone close "<name>"` | Close layer milestone at MVR | (none) |
| Loop | `crosslink session end --notes "<text>"` | Record handoff for next session | `--notes` |
| Loop | `crosslink session last-handoff` | Read prior session's handoff at start of new session | (none) |

### G-138 finding-index commands (crosslink path)

The G-138 project-level finding index (cross-cutting registry) uses the same `issue` subcommand surface as Phase 3 above, with an explicit label-axis convention. All commands verified against installed crosslink v0.8.0 on 2026-05-17.

| Used for | Command | Verified flags |
|---|---|---|
| Create a finding-as-issue with structured labels | `crosslink issue create "<finding-title>" -l domain:<slug> -l layer:<N> -l round:<N> -l finding:<N> -l classification:<class> -l source:<source>` | `-l` (repeatable for multiple labels); `--description`; `--priority`; `--parent` for subissue relationships; `--label review-finding` auto-applied when `swarm review --file-issues` files the finding |
| Filter by single label axis | `crosslink issue list -l domain:quality-engineer -s open` | `-l` (single label per call); `-s` for status |
| Filter by multiple axes (AND) | Not directly supported by `-l` (single-label filter); use `crosslink issue list --json -s all \| jq` for multi-axis composition, OR pipe through `grep` on the table output | (jq composition is the suite's verified manual fallback; future crosslink versions may add multi-label filter) |
| Browse interactively | `crosslink tui` | (interactive — Issues tab supports tree view, detail view, filtering, sorting) |
| Add a label to an existing finding | `crosslink issue label <id> <label>` | (both positional) |
| Remove a label | `crosslink issue unlabel <id> <label>` | (both positional) |
| Reclassify (e.g., Open → Resolved) | `crosslink issue unlabel <id> classification:open && crosslink issue label <id> classification:resolved && crosslink issue close <id>` | (sequence — close after labeling) |
| Migrate manual ↔ crosslink | `crosslink export -f json -o vsdd-suite/FINDINGS-INDEX-export.json` (export); `crosslink import vsdd-suite/FINDINGS-INDEX-import.json` (import — positional `<INPUT>` file path; format is JSON only per `--help`) | `-f json` / `-o <path>` for export; positional `<INPUT>` for import |

### Crosslink commands the suite *does not* depend on

For audit clarity — these commands exist in crosslink but the suite does not reference them in any current artifact. Listed so a future contributor knows the suite's contract surface is intentionally scoped:

`crosslink kickoff` (suite uses `swarm` instead), `crosslink container` (not in scope), `crosslink sentinel` (not in scope), `crosslink style` (not in scope — the suite carries its own house-style discipline via per-domain reviews), `crosslink mc` / `crosslink serve` / `crosslink tui` (TUI is mentioned as a quick-lookup option but not as a workflow dependency), `crosslink trust` / `crosslink locks` / `crosslink migrate` (operational; not part of the suite's documented workflow), `crosslink config` (used by crosslink-using projects as needed; not suite-documented), `crosslink context` / `crosslink integrity` / `crosslink compact` / `crosslink prune` (housekeeping; not suite-documented), `crosslink timer` (time-tracking; not in scope).

**Note:** `crosslink knowledge` moved from the "does not depend on" list to the dependency surface table above as of v0.5.0 (G-146) — `scaffold-project.sh` registers primers, activated domain prompts, and supplements via `crosslink knowledge import`. `crosslink sync` likewise moved into the dependency surface (knowledge import requires the cache to be initialized; `crosslink knowledge sync` is the explicit invocation when an import surfaces a sync-cache-not-initialized error).

### Known limitations (suite-discovered against crosslink v0.8.0)

These are not breaking-change items — they are surface limitations the suite has worked around. Listed so a future contributor knows what does NOT work as documented in `crosslink --help` and what the suite's documented workaround is:

| Command surface | Observed behavior | Suite's workaround | Source |
|---|---|---|---|
| `crosslink milestone create --quiet` | The `--quiet` flag does not reduce output to just the milestone ID — `Created milestone #N: <title>` is still printed. The README's worked example previously assumed `--quiet` returned the ID (parallel to `crosslink quick --quiet`). | The worked example extracts the milestone ID via `awk '/^Created milestone/ {gsub(/[#:]/,"",$3); print $3}'`. Alternative: invoke `crosslink milestone list` after creation. | G-167 (registered Review 63) — discovered during G-106 sample-output capture. |
| `crosslink milestone add/show/close <ID>` | The `<ID>` argument is a numeric milestone ID, not a milestone name. The prior worked example used the milestone-name form which fails with `error: invalid value 'Layer 1: ...' for '<ID>': invalid digit found in string`. | The worked example captures the milestone ID into a shell variable at `milestone create` time and passes the numeric ID to subsequent commands. | G-106 closure (Review 62). |
| `crosslink swarm gate <phase-slug>` | Requires `crosslink swarm init --doc <design>` to have run first AND all planned agents to be resolved. The README previously treated `swarm gate` as a standalone "run the test suite" command. | Solo projects use clean `cargo test` (or equivalent) as the gate; multi-agent swarm builds use `crosslink swarm gate <phase>` after `swarm init` + agent resolution. | G-106 closure (Review 62). |
| `crosslink swarm review --doc <PATH>` | The `--doc` flag is the **output path** for the consolidated findings document, not the per-agent input prompt. The README's prior phrasing was ambiguous; a new reader could read `--doc vsdd-suite/SOFTWARE-ENGINEER-REVIEW.md` as "use this domain prompt for the review." | Per-agent input prompts are loaded from `vsdd-suite-domain`-tagged crosslink knowledge pages (G-146); `--doc` only specifies where the aggregated findings doc is written. | G-106 closure (Review 62). |

A discovered limitation is **not** a breaking change against this contract — it is a doctrine-clarification for the suite's documented workflow. If a limitation becomes blocking (a suite-documented command stops producing the workaround's expected output, e.g., `crosslink milestone create` stops printing the `#N` line we parse), that IS a breaking change against this contract and triggers the response in `## Breaking-change definition` above.

If the suite begins to depend on any of these, this section's row moves into the dependency surface tables above and the "Tested against" line at the top must be re-confirmed.

## Breaking-change definition

A change in crosslink's CLI that requires an update to the suite's worked example is a **breaking change for the suite's contract** with crosslink. Specifically:

- **Breaking:** removing any command in the table above; renaming any command; renaming or removing any required flag listed above; changing the semantic of a required flag; changing the exit-code contract such that the suite's documented commands return non-zero in cases where they previously returned zero (or vice versa).
- **Non-breaking:** adding new commands; adding new optional flags to existing commands; expanding the accepted-input domain of an existing flag; adding output that doesn't replace existing output (e.g., adding a new line above existing stdout); deprecating commands with a stable alias still working.

When a breaking change is observed, the suite must:

1. Pin the prior version explicitly in this file as the "last known-good" version.
2. Update the worked example for the new version.
3. Bump the "Tested against" line at the top of [`README.md`](README.md) and at the top of this file.
4. File a CHANGELOG entry under the suite's next Unreleased section.

## Error contract

When the suite's worked example invokes a crosslink command, the expected and unexpected error responses are:

| Command class | Expected error response | Unexpected error response |
|---|---|---|
| `crosslink init` | Refuses to overwrite an existing `.crosslink/` directory with clear message | Silent overwrite; partial-state init |
| `crosslink workflow diff` | Shows a diff between deployed and embedded policy | Crashes; produces no diff |
| `crosslink quick` / `milestone create` etc. (create operations) | Returns non-zero with clear error if duplicate / invalid input | Silently creates duplicate; returns zero on validation failure |
| `crosslink swarm gate <slug>` | Returns non-zero if the project's test suite fails; prints failing test names | Returns zero on test failure; returns non-zero on environment error indistinguishable from test failure |
| `crosslink swarm review --agents N` | Returns non-zero if any adversary worktree fails to start; per-agent results captured | Silent agent failure; missing findings without notice |
| `crosslink swarm fix --from-label` | Returns non-zero if any fix-agent worktree fails; per-agent results captured | Silent agent failure; un-fixed findings claimed-fixed |
| `crosslink session end --notes "<text>"` | Persists notes; returns zero | Drops notes silently; returns zero |

The unexpected-error column is the failure-mode catalog for Phase 3 reviewers — if a Phase 3 Platform Engineer review observes an unexpected-error case, that is a finding against either crosslink (file upstream) or against the suite's documented expectation (file a suite gap).

## Contract testing

There is currently no automated contract test that runs the worked example end-to-end against a pinned crosslink version. This is tracked as **G-112** in [`suite-development/FINDINGS-INDEX.md`](suite-development/FINDINGS-INDEX.md) — the reference implementations at [`../vsdd-suite-reference-examples/bookmark-cli-manual/`](../vsdd-suite-reference-examples/bookmark-cli-manual/) and [`../vsdd-suite-reference-examples/bookmark-cli-crosslink/`](../vsdd-suite-reference-examples/bookmark-cli-crosslink/) exercise the full worked example end-to-end in both operational modes; the crosslink-variant reference example serves as the canary for both contract-drift detection (this file) and documentation-accuracy regression (G-106).

Until the reference implementation lands, contract drift detection is manual: a contributor must re-run the worked example end-to-end after any reported change to crosslink's CLI surface.

## Cross-references

- [`README.md`](README.md) — the worked example that invokes the commands above.
- [`domains/role/SOLUTION-ARCHITECT-REVIEW.md`](domains/role/SOLUTION-ARCHITECT-REVIEW.md) — SA External Interface Contracts dimensions (Dims 13–22) the suite teaches.
- [`suite-development/FINDINGS-INDEX.md`](suite-development/FINDINGS-INDEX.md) — G-111 (version-pinning), G-112 (reference implementation), G-118 (this file's reason-for-being), G-120 (suite versioning that anchors the "Tested against" line).
- [`domains/role/PLATFORM-ENGINEER-REVIEW.md`](domains/role/PLATFORM-ENGINEER-REVIEW.md) — PE coordinates with SA on CLI dependency surfaces.
