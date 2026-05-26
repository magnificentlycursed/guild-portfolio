<!-- hook-bypass[check-document-staleness]: pre-existing in-flight phrasing preserved per the forward-only narrative-preservation policy. This file's status claims predate the R95 F2 check-document-staleness hook; flagging would require retroactive rewriting that crosses the forward-only carve-out. Future status-claim edits SHOULD use current-state phrasing; the bypass-mechanism is itself a finding for the next registry-walk review. -->
# {{PROJECT_NAME}}

One-sentence description of what this project does, in user-visible terms.

## What this is

Two or three paragraphs expanding the one-sentence description. Name the target user, the problem the project solves, and the project's current state ([Phase 2b](../primers/2b-implementation.md) in-progress on Layer N; layer-N-complete; etc.). Link to [`DESIGN.md`](DESIGN.md) for the full behavioral contract.

**Methodology intent:** `<learning-exercise | portfolio | capstone | production>` — see [`DESIGN.md`](DESIGN.md) § Project intent for what this intent declares about the IAR scope the project runs.

## Prerequisites

What a user needs to run this project from a clean checkout.

- Language toolchain and version (e.g., [Rust](https://www.rust-lang.org/) 1.78+, Node 20+, [Python](https://www.python.org/) 3.11+)
- System dependencies (if any)
- Optional: any AI tool or tracker (e.g., [crosslink](https://github.com/forecast-bio/crosslink)) used in development — only required for contributors

## Install

Step-by-step commands from a clean checkout. The [Technical Writer](../domains/role/TECHNICAL-WRITER-REVIEW.md) review's clone-and-follow test applies here: if any step fails when run from a fresh environment, this section is incomplete.

```sh
git clone <repo-url>
cd {{PROJECT_NAME}}
# build / install commands here
```

## Run

How to invoke the project for its primary use case.

```sh
# example invocation
```

## Test

How to run the test suite.

```sh
# example: cargo test / npm test / pytest
```

## How this was built

Brief paragraph noting the methodology (VSDD via the [VSDD suite](https://github.com/<suite-repo-url>)), the layer structure, and where to find the spec ([`DESIGN.md`](DESIGN.md)) and the per-domain review history ([`vsdd-suite/`](vsdd-suite/)). Useful for future-you and any reviewer evaluating process artifacts.

The project's audit-trail artifacts ([`vsdd-suite/FINDINGS-INDEX.md`](vsdd-suite/FINDINGS-INDEX.md) + per-session files in [`vsdd-suite/review-log/`](vsdd-suite/review-log/)) are authored for the three audiences of the methodology's [three-audience design principle](../suite-development/suite-development.md#three-audience-design-principle-review-80-finding-3): the project's contributors + future maintainers / cold readers (both human audiences) + AI agents performing structured lookups. The column shape + classification vocabulary + per-Finding anchor IDs are stable agent-API surface — see [`vsdd-suite/suite-development/suite-development.md`](../suite-development/suite-development.md) [§ Agent-API surface](../suite-development/suite-development.md#agent-api-surface-review-80-finding-3) for the full contract.

## License

(your license here)
