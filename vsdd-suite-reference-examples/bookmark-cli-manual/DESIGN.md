# DESIGN.md — bookmark-cli

[Phase 1a+1b](../../vsdd-suite/primers/1ab-spec-crystallization.md) contract (per v0.7.2 conventions; the file was originally authored under the prior single-step "Phase 1a" naming + the prior primer filename `1ab-spec-development.md` — both retired by the suite. The current canonical primer is [`../../vsdd-suite/primers/1ab-spec-crystallization.md`](../../vsdd-suite/primers/1ab-spec-crystallization.md); historical narrative preserved per [G-89](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-89) forward-only policy). This file is the reference-implementation contract for the worked example documented at [`../../vsdd-suite/README.md`](../../vsdd-suite/README.md) § Worked example — it exists to validate the suite end-to-end per [G-112](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112) in the suite's gap registry.

---

## Project intent

(Initially declared `portfolio` in Review 67 per v0.7.2 adoption. **Promoted to `capstone` in PR 6 / Review 78** — bookmark-cli is the reference implementation for the VSDD Suite's worked example; reference implementations must exercise the full 6-phase methodology to teach what they document. Per the [G-177](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-177) precedent — reference examples migrate when the methodology evolves — the prior portfolio-intent declaration is preserved as the historical-narrative anchor below the current declaration.)

**Declared intent for this project (current):** `capstone`. Rationale: bookmark-cli is the suite's reference implementation for the worked example documented at [`../../vsdd-suite/README.md`](../../vsdd-suite/README.md) § Worked example. The walkthrough exercises **all six VSDD phases** (1a+1b spec / 1c decomposition / 2a Red Gate / 2b implementation / 2c refactor / 3 IAR / 4 routing / 5 hardening / 6 convergence). For the reference to teach what it documents, it must itself run at the bar that walks the full methodology — capstone intent is the natural fit. The 7 core role+meta activate plus the capstone-tier extended domains. Active domain set: 6 core role (SE, QE, [UX](../../vsdd-suite/domains/role/UX-REVIEW.md), [Security](../../vsdd-suite/domains/role/SECURITY-REVIEW.md), SA, SO) + [VDD-IAR Alignment](../../vsdd-suite/domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md) meta + 6 extended ([Performance Engineer](../../vsdd-suite/domains/role/PERFORMANCE-ENGINEER-REVIEW.md) — capstone activation per [G-150](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-150) intent calibration; [Platform Engineer](../../vsdd-suite/domains/role/PLATFORM-ENGINEER-REVIEW.md) — [G-178](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-178) strong-presumption + [G-155](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-155) dim 38 fresh-system install verification at capstone; [Red Team](../../vsdd-suite/domains/role/RED-TEAM-REVIEW.md) — capstone-tier adversarial intensity per the extended-pool activation criteria; [Technical Writer](../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md) — portfolio+ activation for the worked example's clone-and-follow audit trail; [Documentation Reviewer](../../vsdd-suite/domains/role/DOCUMENTATION-REVIEWER-REVIEW.md) — TW adversarial cold-reader pair, registered in [Review 80](../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z), activates together with TW at capstone intent; [AI Engineer](../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md) — cost-and-quality discipline for parallel cold-session AI-agent usage, registered in [Review 83](../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-83--2026-05-21-1000z) after PR [#38](https://github.com/magnificentlycursed/guild-portfolio/pull/38)'s 3-round cycle burned ~3-4M tokens + hit a daily rate-limit mid-cycle, activates by default at capstone intent given sustained multi-round IAR cycles) = **12 role + 1 meta = 13 active domains**. [Data Engineer](../../vsdd-suite/domains/role/DATA-ENGINEER-REVIEW.md) evaluated and ruled out — bookmark-cli's flat JSON storage falls below the [G-178](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-178) activation threshold; the absence is documented as deliberate. [Sanity Check](../../vsdd-suite/domains/meta/SANITY-CHECK-REVIEW.md) meta domain ([Review 77](../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-77--2026-05-20-1545z) Finding 2) activates on-demand for findings without natural cross-domain pair; not part of the 13-domain scheduled set.

**Declared intent (historical):** `portfolio` (Review 67 → PR 6 / Review 78). Preserved per [G-89](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-89) forward-only narrative-preservation. The existing 3 portfolio-intent reviews (QE Review 1 dated 2026-05-17; QE Review 2 + SA Review 1 dated 2026-05-20) remain valid records of how the project operated under the prior intent; PR 6's migration adds Review 77 lifecycle fields to those entries without invalidating their portfolio-era findings.

**[Phase 5](../../vsdd-suite/primers/5-formal-hardening.md) strategy:** `planned — Layer 1: Purity Boundary Audit executed (SA Review 1, 2026-05-20) + Mutation Testing via cargo-mutants executed (QE Review 2, 2026-05-20, 100% kill rate on 8 viable mutants); property-based testing via proptest deferred (Layer-1 purity boundary shallow); Fuzz Testing and Proof Execution not applicable (no safety-critical / cryptographic / input-boundary attack surface). Layer 2: Purity Boundary Audit re-runs against the extended pure surface (filter_by_tags + attach_tag); Mutation Testing re-runs against the extended impl with the budget that the 100% kill rate is maintained or any drop has a named rationale; property-based testing via proptest now warranted — the tag idempotence + filter OR-monotonicity properties have natural algebraic shape and proptest's marginal cost is low at Layer 2 scope. Fuzz Testing + Proof Execution remain not applicable. Layer 3 (AI-co-authored; operator-owned): Purity Boundary Audit re-runs against the extended pure surface (export-serialize + import-deserialize + dedup-on-exact-tuple-match — all pure functions of the input JSON + existing store state); Mutation Testing re-runs against the extended impl with the same 100%-kill-rate budget; proptest extends with a round-trip property — for any valid storage-state X, parse(serialize(X)) == X (export + re-import round-trip invariant) AND with an `import(import(X)) == import(X)` idempotence property exercising the dedup rule; Fuzz Testing now warranted — bm import is the project's first untrusted-input surface (stdin-fed JSON from an attacker-controlled pipe), making it the natural first fuzz target. The fuzz harness uses cargo-fuzz with libFuzzer to feed arbitrary byte sequences as stdin to the import deserialize path; the bug class targets are parse-panic / parse-OOM / parse-stack-overflow / any non-error-result behavior outside the spec'd Exit 1 / Exit 2 paths. Proof Execution remains not applicable (no safety-critical / cryptographic primitives even with Layer 3).` Per-layer Phase 5 rounds file under the per-domain review logs per [G-177](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-177) closure: Layer 1 Purity Boundary Audit in [`vsdd-suite/review-log/2026-05-20-solution-architect.md`](vsdd-suite/review-log/2026-05-20-solution-architect.md) Review 1; Layer 1 Mutation Testing in [`vsdd-suite/review-log/2026-05-20-quality-engineer.md`](vsdd-suite/review-log/2026-05-20-quality-engineer.md) Review 2; Layer 2 Phase 5 rounds will land at the same per-domain files with later dated session entries.

**[Phase 6](../../vsdd-suite/primers/6-convergence.md) strategy:** `planned — Layer 1 four-dimensional convergence record landed as the VDD-IAR Alignment Review 3 (project-terminal Layer 1) per primer 6 + [G-177](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-177); attestation lives at [vsdd-suite/review-log/2026-05-20-vdd-iar-alignment.md](vsdd-suite/review-log/2026-05-20-vdd-iar-alignment.md) Review 3 and was signed at PR #42 once Platform Engineer Dim 38 / [G-155](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-155) install-verification cleared via PR #41. Layer 2 four-dimensional convergence: **NOT APPLICABLE** per [G-150](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-150) (over-investment guard) + [G-112](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112) (reference-implementation-purpose-already-satisfied) — bookmark-cli's reference-implementation purpose is "exercise all six VSDD phases end-to-end as a worked example", which Layer 1's project-terminal MVR + Phase 6 attestation already demonstrate. Re-running Phase 6 for Layer 2 would teach methodology consumers that capstone artifacts require per-layer four-dimensional convergence, which is not the suite's intent — capstone gates at project-terminal MVR per primer 6, not per-layer. This disposition closes Layer 2 Round 1 VDD-IAR Alignment R4 F5 + Solution Owner R4 F2 (the cluster's own SO recommended Option 1: mark not-applicable; this declaration adopts that recommendation). Layer 2's Phase 5 strategy stands (Purity Boundary Audit re-run + Mutation Testing re-run + proptest activation); Layer 2's Phase 6 strategy is this explicit "not applicable" declaration. Layer 3 four-dimensional convergence (AI-co-authored; operator-owned): **NOT APPLICABLE** per the same [G-150](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-150) + [G-112](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112) rationale as Layer 2 — capstone gates at project-terminal MVR per primer 6, not per-layer; running Phase 6 for Layer 3 would re-teach the same not-applicable disposition the Layer 2 declaration already documents. The Phase 5 hardening at Layer 3 still occurs (Purity Boundary Audit re-run + Mutation Testing re-run + proptest round-trip + cargo-fuzz on bm import); Phase 6 specifically (four-dimensional convergence attestation) is the not-applicable part.` Per [G-162](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-162): capstone-intent declarations require both Phase 5 + Phase 6 strategy lines; both declared above for both layers (Layer 2's Phase 6 declared as the explicit not-applicable disposition per the [G-150](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-150) + [G-112](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112) rationale above).

**Cold-session budget:** capstone default per [`../../vsdd-suite/domains/DOMAIN-INDEX.md`](../../vsdd-suite/domains/DOMAIN-INDEX.md) § Cold-session budget per intent — max 4 rounds before stop-trigger consultation; max 10 parallel agents per round (or 4-cluster batched with adversarial-pair separation per the PR [#38](https://github.com/magnificentlycursed/guild-portfolio/pull/38) Round 3 precedent); 100k–300k tokens per substantive finding expected band; [Opus 4.7](../../vsdd-suite/README.md) for Software Engineer / Security / Red Team / Solution Architect / Solution Owner / VDD-IAR Alignment / AI Engineer; [Sonnet 4.6](../../vsdd-suite/README.md) for UX / Performance Engineer / Platform Engineer / Technical Writer / Documentation Reviewer / Quality Engineer; [Haiku 4.5](../../vsdd-suite/README.md) for mechanical-sweep delegated sub-agents (anchor-link sweeps, reference rewrites, per-domain-index retirement cascades). Actual cost evidence: PR #38 Round 3 cycle ~$5/cluster at the 4-cluster shape; AI Engineer Review 1 cycle (PR [#39](https://github.com/magnificentlycursed/guild-portfolio/pull/39)) registered ~21k tokens/finding — well below the band's floor, read as parallel adversarial review running efficiently per [AI Engineer R1 F6+F7+F8](vsdd-suite/review-log/2026-05-21-ai-engineer.md). Pre-cycle declaration discipline applied at every future multi-agent cycle per [`../../vsdd-suite/primers/3-review-session.md`](../../vsdd-suite/primers/3-review-session.md) § Pre-cycle methodology check; after-action cost-tally per [`../../vsdd-suite/suite-development/suite-development.md`](../../vsdd-suite/suite-development/suite-development.md) § Per-review entry preamble § Cost-tally. Per [Review 84](../../vsdd-suite/suite-development/review-log/2026-05-21-suite-review.md#review-84--2026-05-21-1100z) (PR [#40](https://github.com/magnificentlycursed/guild-portfolio/pull/40)): cold-session-budget declarations are required at capstone + production intent.

---

## What this project does

`bookmark-cli` is a single-user command-line tool for capturing URLs the user encounters at the terminal and recalling them later. The user runs `bm add <url>` to save a URL with a timestamp; the user runs `bm list` to print all saved URLs newest-first. Storage is a flat JSON file in the current working directory (or at the path named by `$BOOKMARK_CLI_DB`).

The project exists as the reference implementation for the VSDD suite's worked example. It is small by design — its purpose is to exercise the suite end-to-end, not to be a useful bookmark manager. A user who wants a real bookmark tool should use the browser, not this.

## Scope and non-goals

**In scope (Layer 1):**
- `bm add <url>` — capture a URL with the current timestamp
- `bm add` (no URL) — reject with a specific error message
- `bm list` — print all bookmarks newest-first
- `bm list` (no bookmarks) — print an explicit empty-state message
- Storage in a flat JSON file at `$BOOKMARK_CLI_DB` or `./bookmarks.json`

**In scope (Layer 2):**
- `bm tag <url> <label>` — attach a label to all bookmarks matching `<url>` exactly; idempotent
- `bm list --tag <label>` — filter by label; repeated flag is OR-semantics
- Storage format extends with a per-bookmark `tags: Vec<String>` field that defaults to empty when absent (Layer-1-format files remain readable)

**In scope (Layer 3, active):**
- `bm export` — emit bookmarks as JSON to stdout in the storage-format object-wrapped shape (`{"bookmarks":[...]}`); pipeable to other tools; optional `--tag <label>` flag for filtered export (parallel to `bm list --tag` OR-semantics).
- `bm import` — read bookmarks from stdin (storage-format JSON); append to existing store preserving append-only semantics; idempotent on URL+timestamp+tags exact-match (no duplicate-row creation for identical records). Storage format unchanged from Layer 2.

(Layer 3 promoted from "deferred — scoped only" to active at AI-co-authored first-draft 2026-05-24 per operator's "I author first-draft; you edit + own" directive. **This spec is AI-co-authored; operator owns the final contract.** The [G-156](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-156) developer-voice discipline applies to the PROCESS.md retrospective, not the spec authoring; the AI-co-authored-disclosure shape parallel to [`PROCESS.md` § AI-co-authored reference-example disclosure](PROCESS.md) applies. Operator-confirmed decisions inline: dedup-on-`url`+`timestamp`+`tags` exact-tuple-match (both against destination state AND within imported payload); 10 MB input-size cap default with `--max-stdin-bytes <N>` override; strict-object-wrapped stdin only (bare arrays rejected); empty-stdin treated as user-error exit 1; cargo-fuzz with libFuzzer as the Phase 5 fuzz harness; filter-empty-state shares the store-empty `{"bookmarks":[]}` shape; selective-copy via `--tag`-filtered export stays silent as emergent behavior. `display_safe` placement at the serialization step is deferred to Phase 2b implementation verification.)

**Non-goals (out of scope at every layer):**
- Network synchronization — local file only
- User accounts or multi-user — single-user local tool
- Browser integration — terminal only
- Search beyond tag filtering — `grep` is the search tool
- URL validation beyond non-empty — accept any string; the user is responsible
- Editing or deleting bookmarks — append-only semantics; manual JSON edit if needed
- Configuration file — environment variable + sensible default is the entire config surface

## Behavioral contracts

### `bm add <url>`

- **Input shape:** exactly one positional argument, a non-empty string.
- **Success output (stdout):** silent. Exit 0.
- **Success side effect:** appends a `Bookmark { url, timestamp }` record to the storage file. Creates the file if absent. Timestamp is the current UTC time in RFC 3339 format.
- **Failure (empty URL — both "empty string" and "no positional argument given"):** stderr `Error: URL cannot be empty.` followed by newline. Exit 1. No file write. Per [SE Review 1 Finding 1](vsdd-suite/review-log/2026-05-20-software-engineer.md#review-1--2026-05-20-1930z) (Review 82 fix-cycle): `bm add` (no positional) is treated identically to `bm add ""` — the parser intercepts clap's usage-error path and emits the spec-contracted exit code 1.
- **Failure (storage file unreadable / unwritable):** stderr `Error: <descriptive message>` followed by newline. Exit 2. **Atomic write** — partial writes MUST NOT occur. The implementation uses a temporary file in the destination directory + atomic rename per POSIX `rename(2)` semantics. If write or rename fails, the storage file's prior state is preserved. Per [SE Review 1 Finding 2](vsdd-suite/review-log/2026-05-20-software-engineer.md#review-1--2026-05-20-1930z) (Review 82 fix-cycle).
- **Failure (CLI usage error other than missing/empty URL — e.g., unknown subcommand, unknown flag):** stderr clap-formatted usage message. Exit 64 (per `sysexits.h` `EX_USAGE`). Per [SE Review 1 Finding 3](vsdd-suite/review-log/2026-05-20-software-engineer.md#review-1--2026-05-20-1930z) (Review 82 fix-cycle) — disambiguates from exit 2 storage errors.

### `bm list`

- **Input shape:** no positional arguments. Layer 2 adds an optional `--tag <label>` flag (may be repeated; see § `bm list --tag` below).
- **Success output (stdout):** zero or more lines, one per bookmark, newest-first. Format per line: `<timestamp> <url>` (timestamp in RFC 3339; single space separator). Trailing newline after the last bookmark. Per-bookmark tags are NOT printed at Layer 2 — the list format remains the Layer 1 contract; tags are a filtering surface only. (A future `--show-tags` flag is out of scope for Layer 2.)
- **Success exit:** 0.
- **Empty-state output:** stdout silent. Stderr: `No bookmarks yet.` followed by newline. Exit 0 (empty is success, not failure).
- **Failure (storage file unreadable / corrupt JSON):** stderr `Error: <descriptive message>` followed by newline. Exit 2. Stdout silent.

### `bm tag <url> <label>` (Layer 2)

- **Input shape:** exactly two positional arguments — `<url>` (non-empty string; matched against bookmark URLs exactly; case-sensitive) and `<label>` (non-empty string; whitespace permitted, parallel to the URL rule).
- **Success output (stdout):** silent. **Stderr:** `Tagged 1 bookmark.` for a single match; `Tagged N bookmarks.` for N ≥ 2 matches (singular/plural conditional per Layer 2 Round 2 UX F4). Exit 0. The stderr affordance was added at Layer 2 Round 1 per [UX F2 + SE F2](vsdd-suite/review-log/) — silent-on-success would leave the multi-match semantic (a `bm tag` that touched 2 bookmarks because two share the same URL) undiscoverable from user behavior; routing the count to stderr (not stdout) preserves pipeline-script-ability of `bm tag` (`stdout` stays silent so `bm tag X Y | downstream` does not see a placeholder line).
- **Success side effect:** for every bookmark in the store whose `url` field equals `<url>` exactly, appends `<label>` to that bookmark's `tags` field if not already present (idempotent). The atomic-write discipline from `bm add` applies — the destination file is written via temp file + `rename(2)`; partial writes MUST NOT occur. If the store file is absent (no bookmarks have been added yet), the command is a no-match case and behaves per the failure contract below.
- **Failure (empty URL — both `bm tag "" <label>` and `bm tag` with the URL positional missing):** stderr `Error: URL cannot be empty.` followed by newline. Exit 1. No file write. (Same error string as `bm add` — the same input invariant.)
- **Failure (empty label — both `bm tag <url> ""` and `bm tag <url>` with the label positional missing):** stderr `Error: tag label cannot be empty.` followed by newline. Exit 1. No file write.
- **Failure (no bookmark matches `<url>`):** stderr `Error: no bookmark found with URL <url>.` followed by newline. Exit 1. No file write. (Typos surface as user-errors; silent no-op would mask them.)
- **Failure (storage file unreadable / corrupt JSON):** stderr `Error: <descriptive message>` followed by newline. Exit 2.
- **Failure (CLI usage error other than empty/missing positional — e.g., third positional, unknown flag):** stderr clap-formatted usage message. Exit 64. (Same exit-64 discipline as `bm add`.)

**Multi-match semantics.** If two or more bookmarks have the same URL (append-only semantics permits duplicates), `bm tag <url> <label>` tags ALL matches in one atomic save. This is the deliberate semantic — the URL is the user's identifier, and any rendering of "tag this bookmark" against duplicate URLs is ambiguous; tagging all matches preserves invariants over `bm list --tag <label>` (a filter on `<label>` will surface every record whose URL was tagged).

**Idempotence under repeat invocation.** `bm tag <url> <label>` followed by `bm tag <url> <label>` (same args, same store state) is a no-op on the second invocation — the second save still writes the file atomically but the file contents are identical. Tests assert exit 0 for both invocations and asserts the tag appears exactly once in the bookmark's `tags` field.

### `bm list --tag <label>` (Layer 2)

- **Input shape:** zero positional arguments + one or more `--tag <label>` flags.
- **Success output (stdout):** the subset of bookmarks (in `bm list` newest-first ordering) whose `tags` field contains AT LEAST ONE of the supplied `<label>` values (OR-semantics across repeated flags). Format per line is identical to plain `bm list`: `<timestamp> <url>`.
- **Success exit:** 0.
- **Empty-state output (no bookmark matches the filter):** stdout silent. Stderr: `No bookmarks match the supplied filter.` followed by newline. Exit 0. (Distinct from plain `bm list`'s `No bookmarks yet.` — the user filtered explicitly, so the empty-state message names the filter.)
- **Failure (empty label — `bm list --tag ""`):** stderr `Error: tag label cannot be empty.` followed by newline. Exit 1. (Same error string as `bm tag`.)
- **Failure (storage file unreadable / corrupt JSON):** stderr `Error: <descriptive message>` followed by newline. Exit 2.

**Why OR-semantics for repeated `--tag`:** AND-semantics would require boolean composition syntax that is out of scope at Layer 2 (e.g., `--tag rust AND --tag cli`). OR-semantics matches the natural shell idiom (`--tag rust --tag go` reads as "anything tagged rust OR go"). A future Layer (or Layer 3) may add AND-semantics with an explicit operator if user feedback warrants.

### `bm export` (Layer 3)

- **Input shape:** zero positional arguments + optional `--tag <label>` flag (repeated; OR-semantics filter parallel to `bm list --tag`).
- **Success output (stdout):** the storage-format object-wrapped JSON shape per § Storage format (`{"bookmarks":[...]}`) emitting every bookmark in the store, OR (when `--tag` is supplied) the OR-filtered subset. **Newest-first ordering preserved** in the emitted JSON array (matches `bm list` ordering invariant). Per [Red Team Review 1 Finding 3 carry-forward](vsdd-suite/review-log/2026-05-20-red-team.md) (advisory closed at PR #46 as Layer-3-trigger): tag labels + URLs route through `display_safe` at the export boundary BEFORE being emitted to stdout — the rendered JSON is sanitization-clean for downstream pipeline-renderable surfaces (terminal pipes, log aggregators, web renders). `display_safe` wraps the string fields (`url`, individual `tags` elements) at the per-bookmark serialization step; the JSON structure itself is unsanitized (JSON's structural delimiters are not terminal-escape-bearing). **Phase 2b implementation verification:** the implementation must confirm `display_safe`-wrapped strings remain JSON-valid (terminal escape sequences serialize as JSON-valid `\u001b` sequences so the round-trip `bm export | bm import` preserves the underlying bytes).
- **Success exit:** 0.
- **Empty-state output (store is absent or empty):** stdout emits `{"bookmarks":[]}` followed by newline — the empty-array case is a valid export; downstream tools consume it as zero-bookmarks-state without ambiguity. Exit 0. **No stderr message** in the empty-state case (unlike `bm list` which emits `No bookmarks yet.` to stderr). The distinction: `bm list` is human-rendering; `bm export` is pipeline-rendering; stderr noise on the empty-state would surface in pipeline logs without adding signal.
- **Empty-state output (filter produces zero matches):** stdout emits `{"bookmarks":[]}` followed by newline. Stderr silent. Exit 0. Same shape as the store-empty case — pipeline consumers see one structural shape for both zero-bookmarks cases; the consumer that needs to distinguish the two states already knows whether it supplied a filter.
- **Failure (empty label — `bm export --tag ""`):** stderr `Error: tag label cannot be empty.` followed by newline. Exit 1. (Same error string as `bm tag` + `bm list --tag`.) No stdout output.
- **Failure (storage file unreadable / corrupt JSON):** stderr `Error: <descriptive message>` followed by newline. Exit 2. No stdout output.
- **Failure (CLI usage error other than empty `--tag`):** stderr clap-formatted usage message. Exit 64.

**Pipeline-script-ability.** `bm export | bm import` is the canonical round-trip: export emits the store as JSON; import consumes the JSON and merges into the destination store (which may be the same or different `$BOOKMARK_CLI_DB`). The round-trip invariant: `bm export | bm import` against a fresh destination store reproduces the source store's bookmarks (modulo timestamps which are preserved as-emitted, not re-stamped). The pair (`bm export` + `bm import`) enables: backup workflows; cross-machine sync (via manual pipe through SSH or file transfer); store-to-store migration. (The composition of `bm export --tag <label>` with subsequent `bm import` is emergent from the parts working independently; the spec does not commit to it as a discrete documented use case.)

### `bm import` (Layer 3)

- **Input shape:** zero positional arguments + stdin pipe of JSON content matching the storage-format object-wrapped shape (`{"bookmarks":[...]}`). Strict-only on the object-wrapped form — bare-array stdin (`[{...}]`) is rejected with the schema-mismatch error; operators with bare-array JSON from other sources can wrap via `jq '{bookmarks: .}'` before piping in.
- **Success output (stdout):** silent. **Stderr:** `Imported 1 bookmark.` for single-import; `Imported N bookmarks.` for N ≥ 2 (singular/plural per Layer 2 Round 2 UX F4 precedent). Exit 0. Stderr (not stdout) preserves pipeline-script-ability of `bm import` in case future composition pipes its stdout somewhere.
- **Success side effect:** appends imported bookmark records to the existing store, preserving append-only semantics. **Idempotence under repeat invocation:** `bm import` consuming the same JSON twice (against the same destination store state) is a no-op on the second invocation — dedup runs on `url`+`timestamp`+`tags` exact-tuple-match. **Dedup scope:** dedup runs BOTH against existing destination state AND within the imported payload itself — if the imported JSON contains two records with identical `url`+`timestamp`+`tags`, only one is appended. The `Imported N bookmarks.` count reflects only the appended-records count (zero counts the records dropped via either dedup path).
- **Storage-file write semantics:** atomic-write per `bm add` (`tempfile + rename(2) + fsync(parent_dir)`). If the store file is absent, create it (parallel to `bm add`'s store-creation behavior). All imported bookmarks land in one atomic save — partial imports MUST NOT occur (if any record fails validation, the entire import fails + the existing store is preserved).
- **Failure (invalid JSON in stdin):** stderr `Error: stdin is not valid JSON.` followed by newline + the underlying `serde_json` parse error on the next line for diagnostic context. Exit 1. No file write.
- **Failure (JSON parses but does not match storage-format schema):** stderr `Error: stdin JSON does not match storage-format schema; expected {"bookmarks": [...]}.` followed by newline + the offending field-mismatch detail. Exit 1. No file write. (Bare-array stdin lands on this path.)
- **Failure (empty stdin — no input received):** stderr `Error: stdin is empty; nothing to import.` followed by newline. Exit 1. No file write. Empty-stdin is treated as a user-error (likely a missing pipe) rather than a no-op success; the loud failure makes the mistake visible.
- **Empty-payload success (empty bookmarks array — stdin is `{"bookmarks":[]}`):** stdout silent. Stderr `Imported 0 bookmarks.` followed by newline. Exit 0. (Distinct from empty-stdin: the structural input is valid + naming-zero-imports is the legitimate empty-import semantic.)
- **Failure (storage file unreadable / corrupt JSON on the destination side):** stderr `Error: <descriptive message>` followed by newline. Exit 2.
- **Failure (CLI usage error — positional arguments supplied):** stderr clap-formatted usage message. Exit 64.
- **Failure (stdin exceeds size cap):** stderr `Error: stdin exceeded maximum byte limit of <N>.` followed by newline. Exit 1. No file write. Default cap: 10 MB (matches the project's existing scale ceiling of 10,000 bookmarks at ~1 KB each). Operator override via `--max-stdin-bytes <N>` flag.

**Threat model addition for stdin-fed attacker input.** `bm import` is the project's first surface to consume untrusted JSON. Existing defenses apply: serde_json default 128-level recursion limit defends against JSON depth-bomb (per [Security Review 1 Finding 6](vsdd-suite/review-log/2026-05-20-security.md) Hallucinated disposition); display_safe applies at the rendering boundary if imported bookmarks are subsequently rendered via `bm list`. New defenses for the stdin attack surface: **(a)** the 10 MB input-size cap above (accepted-limitation framing — operators with legitimately-larger imports override via `--max-stdin-bytes <N>`); **(b)** import-from-attacker-controlled-stdin per the Layer 1 threat-model framing on adversary-supplied URL contents — same disposition: store the bytes as-given; defer rendering safety to the `display_safe` discipline at output time.

**Why dedup-on-exact-tuple-match.** The dedup-on-`url`+`timestamp`+`tags` rule prevents the common-mistake double-import (operator pipes `bm export | bm import` twice; without dedup the second import duplicates the records) AND preserves the Layer 2 multi-record semantic for duplicate URLs with different timestamps (two truly-distinct bookmarks happening to share a URL stay distinct because their timestamps differ — URL-only dedup would collapse them). The trade-off accepted: a re-tag-then-re-pipe workflow (re-tag a record at source; re-pipe to destination) produces a duplicate-row at destination because the new tags-tuple does not match the existing destination tuple — but the duplicate is operator-detectable via `bm list`, not silent data-loss. Dedup applies BOTH against existing destination state AND within the imported payload itself; the "byte-equal records collapse" rule has algebraic shape suitable for a proptest property (`import(import(X)) == import(X)`).

## Edge case catalog

- **Empty URL argument:** `bm add ""` → rejected per failure contract above.
- **Whitespace-only URL:** `bm add "   "` → currently accepted; the user is responsible. This is a deliberate non-goal of input validation.
- **Storage file absent:** `bm list` on a fresh project → empty-state message; `bm add` → creates the file.
- **Storage file empty (zero bytes):** treat as empty bookmark list, not as corrupt.
- **Storage file contains invalid JSON:** error to stderr, exit 2. Do not attempt recovery.
- **Concurrent writes:** out of scope; not a multi-process tool. Single user, single shell session.
- **Very long URL (10K+ chars):** accepted. No length cap.
- **URL containing newlines:** accepted. May visually break the `bm list` output, which is acceptable for this scope.

**Layer 2 additions:**

- **Tag against a Layer-1-format file (no `tags` field on existing bookmarks):** the missing field deserializes to an empty `Vec<String>`; `bm tag` appends; on save the file is rewritten with the field present for every bookmark (touched and untouched alike). This is the deliberate forward-only migration shape — Layer-1-format files become Layer-2-format files on first Layer-2 write.
- **Whitespace-only label:** `bm tag <url> "   "` → currently accepted; the user is responsible. Mirrors the whitespace-only-URL rule.
- **Duplicate-URL bookmark store (two bookmarks with identical URL):** `bm tag <url> <label>` tags both; `bm list --tag <label>` surfaces both lines (one per record). The append-only semantic permits this.
- **Tag a bookmark twice with the same label:** idempotent — the second `bm tag` invocation is a no-op against the bookmark's `tags` field but still writes the file (the atomic-write discipline does not optimize for byte-equality).
- **`bm list --tag <label>` against an empty store:** the empty-store empty-state (`No bookmarks yet.`) takes precedence over the no-filter-match empty-state (`No bookmarks match the supplied filter.`) — the user has no bookmarks at all, which is the more informative signal.
- **`bm list --tag <label1> --tag <label2>` where one label has no matches:** OR-semantics means the bookmarks tagged with the OTHER label are still surfaced. No partial-match warning is emitted.
- **Very long tag label (10K+ chars):** accepted. No length cap (parallel to URL rule).
- **Tag label containing newlines or control characters:** stored as-is; rendered through the same `display_safe` sanitizer as URLs at the rendering boundary (Layer 2 has no tag-rendering path at the user surface — `bm list` does not print tags — but the sanitizer still applies if a future flag prints them).

**Layer 3 additions:**

- **`bm export` on a Layer-1-format store (no `tags` field on existing bookmarks):** the missing field deserializes to an empty `Vec<String>`; emitted JSON always carries the `tags` array (possibly empty) per the serialization-side invariant from Layer 2. Forward-only migration semantic preserved.
- **`bm export --tag <label>` against an empty store:** emits `{"bookmarks":[]}` to stdout + exit 0 (no special-cased stderr message; pipeline-rendering audience does not benefit from human-rendering noise).
- **`bm export` against a store with bookmarks containing control characters or newlines in URLs / tags:** `display_safe` wraps the offending string fields at serialization; emitted JSON remains valid + escape-clean + parseable by `bm import` (round-trip invariant — Phase 2b implementation verifies).
- **`bm export | bm import` round-trip:** importing the export output into a fresh destination store reproduces the source store's bookmarks (modulo timestamps preserved as-emitted, not re-stamped). This is the canonical regression target for the proptest round-trip.
- **`bm import` consuming stdin twice against the same destination store:** second invocation is a no-op IF the records' `url` + `timestamp` + `tags` tuple matches existing records exactly. Idempotence guard against operator double-pipe accidents.
- **`bm import` consuming stdin with bookmarks that partially overlap an existing store:** the overlapping records are deduplicated per the exact-tuple-match rule; the non-overlapping records are appended. Mixed-import is permitted and the `Imported N bookmarks.` count reflects only the appended-records count (zero counts the dedup'd records).
- **`bm import` consuming a JSON file with bookmarks that have duplicate URLs (different timestamps) within the imported payload itself:** all records are inserted as-given because their `url`+`timestamp`+`tags` tuples differ; this preserves the Layer 2 duplicate-URL multi-record semantic.
- **`bm import` consuming a JSON file with bookmarks that are byte-equal (identical `url`+`timestamp`+`tags`) within the imported payload itself:** only one record is appended; the duplicates are dropped at import time. Dedup runs against both the existing destination state AND within the imported payload itself — the `Imported N bookmarks.` count reflects only the appended-records count.
- **`bm import` with stdin exceeding the input-size cap (10 MB default; operator may override via `--max-stdin-bytes <N>`):** error `Error: stdin exceeded maximum byte limit of <N>.` to stderr + exit 1. No partial import.
- **`bm import` after `bm export --tag <label>`:** only the tag-filtered subset is imported into the destination. This composition is emergent from the parts working independently; the spec does not commit to it as a discrete documented use case (a future Layer 4 may change import semantics in a way that affects this composition without breaking the documented contract of either subcommand in isolation).
- **`bm import` against a Layer-1-format destination store:** the missing-`tags`-field forward-only migration semantic applies at the destination on the import write — the rewritten store carries the `tags` field for every bookmark (touched and untouched alike). Same migration discipline as `bm tag`.

## Interface definitions

### Command surface (Layer 1)

```
bm add <url>
bm list
bm --help
bm --version
```

### Command surface (Layer 2 additions)

```
bm tag <url> <label>
bm list --tag <label> [--tag <label>...]
```

### Command surface (Layer 3 additions)

```
bm export [--tag <label> [--tag <label>...]]
bm import [--max-stdin-bytes <N>]
```

`bm export` emits the storage-format JSON to stdout; `bm import` reads the same shape from stdin. The pair enables backup / cross-machine sync / store-to-store migration via shell-pipe composition. `--max-stdin-bytes` is the operator-override for the default 10 MB input-size cap (see § Behavioral contracts § `bm import` (Layer 3)).

### Exit codes

| Code | Meaning | Source |
|---|---|---|
| 0 | Success (including empty `bm list`) | Application |
| 1 | User error (empty URL — both `bm add ""` and `bm add` with no positional argument) | Application |
| 2 | Storage error (file unreadable, corrupt JSON, write failure, parent-dir creation failure) | Application |
| 64 | CLI usage error (`EX_USAGE` per `sysexits.h` — unknown subcommand, unknown flag, malformed invocation other than missing/empty URL) | Application (intercepts clap's default exit) |

[SE Review 1 Finding 3](vsdd-suite/review-log/2026-05-20-software-engineer.md#review-1--2026-05-20-1930z) Round 2 disposition: spec extended to disambiguate "user error in URL" (exit 1) from "user error in CLI invocation shape" (exit 64). Storage error (exit 2) is unambiguous.

### Storage format (JSON file)

```json
{
  "bookmarks": [
    {"url": "https://example.com", "timestamp": "2026-05-17T03:01:00Z", "tags": ["rust", "cli"]},
    {"url": "https://example.org", "timestamp": "2026-05-17T02:55:00Z", "tags": []}
  ]
}
```

Newest-first ordering is a render concern (sort on read), not a storage concern (append on write).

**`tags` field (Layer 2).** Optional during deserialization — Layer-1-format files (no `tags` field per bookmark) deserialize cleanly with `tags` defaulting to an empty `Vec<String>`. Always present during serialization — once any Layer 2 operation rewrites the file, every bookmark has an explicit `tags` array (possibly empty). This is the forward-only migration shape: Layer 1 files become Layer 2 files on first Layer 2 write; the reverse is not guaranteed (a Layer 1 binary reading a Layer 2 file is acceptable because `serde_json` tolerates extra fields by default, but this is not contract).

Within a single bookmark, `tags` is treated as a set: duplicates are not produced by the application (idempotent `bm tag`), but the JSON shape is an array. Ordering of the array is insertion order — first `bm tag` invocation's label appears first; subsequent labels append. The spec does NOT contract on tag ordering, and tests should not assert order beyond "label X is present in the array."

## Verification architecture

**Purity boundary (revised Review 67 / B2 reconciliation against actual `src/lib.rs` implementation; supersedes the prior implicit "pure-core" framing in the module doc).** This is the authoritative purity boundary for the project. The module doc at `src/lib.rs:1-?` cites this section as the single source.

- **Pure functions** (deterministic, no I/O, formally verifiable in principle):
  - `Bookmark` and `BookmarkStore` data types (serde derivations are pure functions of input).
  - `BookmarkStore::newest_first` (pure sort by reference; no I/O, no clock).
  - **Layer 2:** `BookmarkStore::filter_by_tags(&[&str])` — pure OR-filter against the store's bookmarks; returns a `Vec<&Bookmark>` in newest-first order.
  - **Layer 2:** `BookmarkStore::attach_tag(url, label)` — pure transformation when given the store, URL, and label; appends `label` to every matching bookmark's `tags` field if not already present. Returns `Result<usize, AttachTagError>` (count of bookmarks affected; error variants for empty-URL / empty-label / no-match-with-URL-carried per the `NoMatch(String)` shape post-PR-#46-Layer-2-carry-forward-close).

**attach_tag / save separation rationale (Layer 2 Round 1 SA F2 carry-forward close at PR #46).** `BookmarkStore::attach_tag` and `BookmarkStore::save` are deliberately separate calls rather than a combined `tag_and_save` helper. Reasoning: callers that perform batched tag operations (canonical example: Layer 3 `bm import` reading a JSON blob with N (url, label) pairs) call `attach_tag` once per pair and `save` once at the end, paying O(1) save cost rather than O(N). A combined `tag_and_save` would force per-pair save overhead — acceptable at the CLI shell (one tag operation per `bm tag` invocation) but wasteful at the library level. The CLI shell's `run_tag` always calls `save` immediately after `attach_tag` because the CLI's contract is one tag per invocation; the library's separation preserves the option for non-CLI callers to batch.
- **Effectful (deliberate I/O wrappers around pure ser/de):**
  - `BookmarkStore::load(path)` — filesystem read + `serde_json` parse. The parse step is pure; the file read makes the function effectful.
  - `BookmarkStore::save(path)` — `serde_json` serialize + filesystem write + directory creation. Same shape: serialize pure, write effectful.
- **Boundary refinement (morally pure w.r.t. its inputs; effectful w.r.t. external clock):**
  - `BookmarkStore::add(url)` — appends a new `Bookmark` whose timestamp is `Utc::now()` at call time. Deterministic given the clock; non-deterministic against absolute wall time. Acceptable at Layer 1 portfolio intent; could be refined to `add(url, ts)` at a future layer if formal verification of `add` enters scope.

**Verification surfaces:**

- **Unit tests** for the pure functions and the I/O-wrapper functions in `src/lib.rs`'s `#[cfg(test)] mod tests` block; the I/O-wrapper tests use `tempfile` for filesystem isolation.
- **Integration tests** in `tests/bookmarks.rs` that invoke the compiled binary via `assert_cmd` against per-test temp directories — full stdout/stderr/exit-code contract per CLI supplement § Quality Engineering.
- **No mocks for the storage layer** — tests use real temp files via `tempfile`.
- **Manual testing checklist** in [`TODO.md`](TODO.md) § Layer 1, expanded per the runnable-step standard.
- **IAR [Phase 3](../../vsdd-suite/primers/3-review-session.md)** runs the 7 default-active core domains per Review 42 doctrine (SE, QE, UX, Security, SA, SO, VDD-IAR Alignment). Rounds file as session entries in `vsdd-suite/review-log/YYYY-MM-DD-<slug>.md` per the [G-89](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-89) structural standard; project finding navigation is via [`vsdd-suite/FINDINGS-INDEX.md`](vsdd-suite/FINDINGS-INDEX.md).
- **[Phase 5 hardening](../../vsdd-suite/primers/5-formal-hardening.md)** (added Review 67 — Phase 5 adoption per v0.7.2 conventions; migrated to per-domain log shape per [G-177](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-177) / v0.7.8): per-layer Phase 5 rounds file in `vsdd-suite/review-log/2026-05-20-solution-architect.md` (Purity Boundary Audit) and `vsdd-suite/review-log/2026-05-20-quality-engineer.md` (Mutation Testing) with the `**Phase 5 surface:**` preamble tag per round. The Phase 5 strategy is declared in § Project intent below.

**Formal-proof candidates (Phase 5 Proof Execution):** none. `bookmark-cli` is not safety-critical or cryptographic; no function on the purity boundary above warrants Proof Execution. Proof Execution declared `not applicable` in the § Project intent Phase 5 strategy line.

**Automatable-vs-manual split:** every behavioral contract above is automatable via unit + integration tests. Manual testing (per TODO.md § Layer 1) verifies UX-coherence concerns (error message specificity; the empty-state stderr line as the user would read it) that automated tests can also assert syntactically but cannot evaluate as "reads naturally."

## Technology choices and rationale

| Choice | Alternatives considered | Why this |
|---|---|---|
| [Rust](https://www.rust-lang.org/) | [TypeScript](https://www.typescriptlang.org/)/Node, [Python](https://www.python.org/), Go | Matches the worked example's language; portfolio precedent (`issue-tracker-cli`); strong test/CLI ergonomics |
| Cargo workspace = single crate | Workspace with separate `lib` and `bin` crates | Over-engineering for one binary |
| `clap` (derive) | Hand-rolled arg parsing | Standard Rust CLI parser |
| `serde_json` | Custom JSON / TOML / sqlite | Spec calls for JSON |
| `chrono` (UTC) | `time` crate / system epoch ints | RFC 3339 formatting is well-supported |
| `anyhow` for error types | Custom error enums per `thiserror` | Single-binary tool; `thiserror` would be over-engineering |
| `assert_cmd` + `tempfile` for tests | Direct std::process invocation | CLI supplement § QE prescribes binary-invocation tests |

## Constraints

- **Rust toolchain:** 1.81+ (modern stable Rust; no unstable features). Pinned via [`rust-toolchain.toml`](rust-toolchain.toml) — Round 2 fix per [Platform Engineer Review 1 Finding 2](vsdd-suite/review-log/2026-05-20-platform-engineer.md); MSRV bumped 1.78 → 1.81 at Layer 2 Round 1 per [Platform Engineer Review 4 Finding 4](vsdd-suite/review-log/2026-05-21-platform-engineer.md) because Layer 1 R3's `reason = "..."` attribute syntax stabilized in 1.81.
- **Platform:** macOS, Linux. Windows untested.
- **Dependencies:** all from [crates.io](https://crates.io/), no git deps. `Cargo.lock` committed. Supply-chain policy enforced via [`deny.toml`](deny.toml) + `cargo deny check` in CI — Round 2 fix per [Security Review 1 Finding 3](vsdd-suite/review-log/2026-05-20-security.md) + [Platform Engineer Review 1 Finding 4](vsdd-suite/review-log/2026-05-20-platform-engineer.md).
- **Deployment:** `cargo install --locked --path .` into `~/.cargo/bin/`. No release pipeline. `--locked` flag enforces `Cargo.lock` at install time — Round 2 fix per [Platform Engineer Review 1 Finding 8](vsdd-suite/review-log/2026-05-20-platform-engineer.md).

## Performance budget ([Review 82](../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z) Round 2 fix for [Performance Engineer Review 1 Finding 1](vsdd-suite/review-log/2026-05-20-performance-engineer.md))

Layer 1 performance commitments:

| Metric | Budget (p95) | Measurement |
|---|---|---|
| `bm --help` / `bm --version` startup | < 50 ms wall-clock on commodity laptop | Manual observation; [`hyperfine`](https://github.com/sharkdp/hyperfine) acceptable for sanity-check |
| `bm add <url>` end-to-end | < 100 ms wall-clock on a store with ≤ 1,000 bookmarks | Same |
| `bm list` end-to-end | < 100 ms wall-clock on a store with ≤ 1,000 bookmarks | Same |

**Scale ceiling:** 10,000 bookmarks. Beyond this the user should consider a real bookmark manager — this project's non-goals (§ Scope and non-goals) declare unsuitability for primary-use scale. The flat-JSON-rewrite-on-every-add design has cumulative O(n²) cost which makes large stores impractical; declared as **accepted limitation** at Layer 1 intent + named in [Performance Engineer Review 1 Findings 3 + 6](vsdd-suite/review-log/2026-05-20-performance-engineer.md).

**Benchmarking infrastructure:** at Layer 2 the surface is still small but tag/filter operations enable meaningful contract assertions. The [`hyperfine`](https://github.com/sharkdp/hyperfine) sanity-check pattern is the Layer 2 contract: a documented `manual-tests/layer-2.md` step generates a 1,000-bookmark store and asserts each named-budget operation completes within the budget. The [`criterion`](https://github.com/bheisler/criterion.rs) framework remains deferred — its development-cycle cost (longer test iteration; benchmark-comparison artifacts in CI) is not justified by the project's scale at Layer 2. The hyperfine sanity-check at the layer-2 manual-test surface is the proportionate Layer 2 closure of [Performance Engineer Review 1 Finding 2](vsdd-suite/review-log/2026-05-20-performance-engineer.md) (declared **Deferred-to-Layer-2** at Round 2; Layer 2 Phase 3 Performance Engineer Round will land the closure attestation).

**Data-scaling tests:** Layer 2 ships sentinel integration tests at the 100 / 1,000 / 10,000-bookmark cliffs that exercise the full add → list → tag → list-filter cycle. Each cliff asserts: (a) operations complete within the budget table above; (b) the storage file round-trips without corruption; (c) the filter result set is correct against a programmatically-generated reference. The tests live in `tests/scaling.rs` and use `#[ignore]` by default so `cargo test` stays fast; CI runs them via `cargo test -- --ignored` in a separate job. This closes [Performance Engineer Review 1 Finding 5](vsdd-suite/review-log/2026-05-20-performance-engineer.md) (**Deferred-to-Layer-2**).

**Durability discipline (Layer 2):** the save path uses `tempfile + rename(2)` for atomic replacement (preserves the prior file's contents on partial failure). Layer 2 adds an explicit `fsync` of the destination file's parent directory after the rename, ensuring the rename itself is durable across a power loss — without the parent `fsync`, the rename may be in the kernel page cache and lost on a power-fail. The cost is one extra `fsync(2)` syscall per write; benchmarked at the Layer 2 Performance Engineer Round against the budget table above (expected < 5 ms on commodity SSD). The fsync is gated `#[cfg(unix)]`; Windows uses its own durability semantics that are not addressed at Layer 2. This closes the operator-queued Performance Engineer fsync benchmark item (deferred from Layer 1 Round 2 via the cold-session-budget gate).

**Filesystem-coverage caveat (PE R1 F5 carry-forward close at PR #47).** The "< 5 ms on commodity SSD" estimate is measured against the reference-example operator's local APFS (macOS) + ext4 (Linux CI runner) filesystems. The cost may differ materially on: (a) NFS / CIFS network-mounted directories — `fsync` round-trip latency to a remote server can be 10-100x local; (b) FUSE-based filesystems (sshfs, encfs) — fsync semantics depend on the FUSE driver's implementation; (c) tmpfs — fsync is a no-op but the durability guarantee is vacuous (tmpfs is RAM-backed); (d) cross-filesystem `rename(2)` — POSIX prohibits cross-filesystem renames so the rename itself would EXDEV-fail before fsync becomes relevant; the `tempfile + rename` discipline guards against this by placing the temp file in the destination directory. **Accepted limitation for the reference-example scope:** the budget table targets the typical local-disk operator scenario. Operators running `bm` against NFS-mounted home directories should expect higher save latency; the spec does not contract on NFS performance. A future production-intent fork that targets shared-filesystem deployments would extend the budget table per measured filesystem.

## Threat model ([Review 82](../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z) Round 2 fix for [Security Review 1](vsdd-suite/review-log/2026-05-20-security.md) + [Red Team Review 1](vsdd-suite/review-log/2026-05-20-red-team.md))

**In-scope adversaries:**

- **Co-tenant on a shared Unix host** — read access to the user's home directory hierarchy. **Mitigation:** storage file mode 0600 (read/write owner only) per the *confidential* data classification below.
- **Adversary-controlled `$BOOKMARK_CLI_DB`** — the env var points at a writable path the user does not control (e.g., a shared `/tmp/...`, a directory with a pre-staged symlink). **Mitigations:** symlink-follow-rejection on **both** load and save (symmetric `symlink_metadata` check + rejection) per the symlink-hardening discipline; the env var is the user's own shell + the user is responsible for what they set. **Residual TOCTOU** — the load-side `symlink_metadata` check and the subsequent `read_to_string` are separate syscalls; an attacker with concurrent filesystem write access to the parent directory could swap a regular file for a symlink in the microsecond race window ([Red Team Review 1 Round 3 Finding 2](vsdd-suite/review-log/2026-05-20-red-team.md#r3-f2) **Accepted risk**). Tight fix is `OpenOptions::custom_flags(O_NOFOLLOW)` (single-syscall atomic check), which is deferred pending a `libc` dependency addition and Platform Engineer / Security re-review. The save side uses `rename(2)` which is atomic regardless.
- **Adversary-supplied URL contents** — a URL captured at one terminal session is later rendered at `bm list` in another terminal session. URLs can carry terminal-escape sequences (ANSI `\x1b[...`, OSC 0/8/1337, bidi format chars U+202E + zero-width chars). **Mitigation:** `display_safe` sanitizer wraps every user-derived value before any `eprintln!` / `println!` / `Display` interpolation — escapes `is_control()` (Cc) chars + `Cf` format chars while preserving `\n` `\t` for legitimate whitespace.
- **Tag-injection-as-trust-signal (Layer 2)** — an adversary with write access to the storage file (the same vector as Adversary-controlled `$BOOKMARK_CLI_DB` above) can fabricate tags like `["verified", "approved"]` on user-trusted bookmarks, creating a misleading trust-signal that the user might interpret as their own past-tagging. **Mitigation:** the file is mode-0600-restricted (only the owner can write) and the symlink-rejection discipline applies; an attacker with write access to the user's `$BOOKMARK_CLI_DB` already has more leverage than tag injection — they can also rewrite URLs, fabricate entire bookmark records, or replace the whole file. Tag-injection is documented as a **deliberate accepted risk** per the same threat-model frame that accepts URL-injection (the attacker has primary write access; downstream tag forgery is a secondary consequence, not a separable threat). Layer 2 Round 1 Red Team F6 surfaced this as a load-bearing gap; this paragraph names the attack class so future cold-readers see the risk is documented + dispositioned.

- **Layer 3 sanitize-at-export readiness (Layer 2 Round 1 Red Team F3 carry-forward advisory closed at PR #46).** Layer 3 (export + import) will introduce `bm export` (emit bookmarks as JSON to stdout) and `bm import` (read bookmarks from stdin). The Layer 3 export path will render tag labels + URLs to stdout for downstream pipeline consumption; the existing `display_safe` sanitizer (used at `bm list`'s URL rendering) must apply at every Layer 3 export boundary that emits to a terminal-renderable surface. Currently no tag-rendering path exists at the user surface, so tag labels do not require the `display_safe` wrap; Layer 3's spec authoring inherits this discipline by reference — the Layer 3 `DESIGN.md` § Behavioral contracts for `bm export` must explicitly route tag labels (and URLs) through `display_safe` before any terminal-renderable emission. This advisory is documented now so Layer 3 spec authoring does not re-discover the discipline; the threat model itself does not change for Layer 2.

- **Chained-vulnerability class — downgrade-compatibility hazard composes with binary-flip vectors (Layer 2 Round 2 Red Team F10 carry-forward close at PR #46).** The downgrade-compatibility hazard documented in § Storage data classification below (Layer 2 binaries write `tags`; Layer 1 binaries silently discard `tags` on next save) composes with attacker primitives that have separate write access to the operator's `bm` binary. An attacker who can flip the operator's `bm` binary to a Layer 1 version (vectors: package-manager tampering; supply-chain compromise of the `cargo` registry serving the `bookmark-cli` crate; `$PATH` manipulation putting an attacker-controlled `bm` ahead of `~/.cargo/bin/bm`) AND who has read access to the user's pre-flip Layer 2 store can silently observe tag-data loss on the next `bm add`. This is the same threat-model frame as the URL-injection class above (the attacker already has substantial leverage via the binary-flip primitive; the asymmetric `serde` discipline does not expand the attacker's surface but does compose with their existing primitives). Documented as **accepted risk** under the same disposition as the URL-injection + tag-injection classes (the attacker has primary leverage; downstream silent-data-loss is a secondary consequence). Named for cold-reader visibility so the chained scenario is documented + dispositioned rather than missed.

**Out-of-scope adversaries:**

- **Same-user concurrent process** writing the storage file at the same time — the project is a single-user single-process tool per § Scope and non-goals; concurrent-write race is **accepted risk** ([Red Team Review 1 Finding 3](vsdd-suite/review-log/2026-05-20-red-team.md)).
- **Unbounded URL length** — the spec accepts arbitrarily long URLs per the original § Edge case catalog. DoS-via-memory is acknowledged but accepted at Layer 1 ([Red Team Review 1 Finding 2](vsdd-suite/review-log/2026-05-20-red-team.md) **Accepted risk**).
- **TOCTOU between `path.exists()` and `read_to_string()` in `BookmarkStore::load`** — single-process foreclosure makes the race window non-exploitable ([Security Review 1 Finding 5](vsdd-suite/review-log/2026-05-20-security.md) **Accepted risk**).
- **JSON-parser depth-bomb (deeply-nested user-controlled JSON)** — `serde_json` enforces a 128-level recursion limit by default; the attacker model does not grant write access to the store file ([Security Review 1 Finding 6](vsdd-suite/review-log/2026-05-20-security.md) **Hallucinated** — verified protection holds).

## Storage data classification ([Review 82](../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z) Round 2 fix for [Security Review 1 Finding 2](vsdd-suite/review-log/2026-05-20-security.md))

The captured bookmarks are **confidential**-class data — "what someone is reading is sensitive" per the [Security domain prompt](../../vsdd-suite/domains/role/SECURITY-REVIEW.md) Dim 8 information-leakage classification. The storage file is written with **mode 0600** (Unix; read/write owner only) using `std::fs::OpenOptions::new().mode(0o600)...` behind a `#[cfg(unix)]` gate. Windows is named as untested under § Constraints; Windows file-permission semantics differ from Unix and are deferred to a Windows-port layer.

Encryption at rest is **not** in scope at Layer 1 — mode 0600 is the spec's floor for confidential-class data on Unix, per the Security domain prompt's proportionality discipline. A future layer (or production-intent fork) may add at-rest encryption if the spec's data-classification rises.

**Layer 2 — `tags` field classification.** The per-bookmark `tags: Vec<String>` field is also **confidential**-class — what someone tags their reading with is at least as sensitive as the URLs themselves ("interests + intent" carries at least the same disclosure risk as "what URLs were captured"). The mode-0600 + symlink-rejection mitigations established for URLs apply uniformly to the `tags` field — the on-disk file is one confidential-class artifact; there is no per-field permission split. Layer 2 Round 1 Security F1 + Solution Architect F5 surfaced this as a load-bearing gap (the prior § Storage data classification text named URLs as confidential but was silent on `tags`); this paragraph closes the gap by stating the classification explicitly.

**Downgrade-compatibility hazard.** The `serde` shape is asymmetric: Layer 2 binaries read Layer 1 files via the `#[serde(default)]` attribute on the `tags` field (Layer-1-format files deserialize cleanly with `tags` defaulting to empty `Vec<String>`), but a Layer 1 binary reading a Layer 2 file will silently discard the `tags` field on the next save (the Layer 1 `Bookmark` struct does not have the `tags` field; `serde_json`'s default behavior ignores unknown fields on deserialize, so the parse succeeds with no error — and on the next `bm add`'s save the file is re-serialized from the in-memory Layer 1 shape, dropping `tags` from disk). This is a **deliberate forward-only migration choice** (matches the [G-89](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-89) forward-only-narrative-preservation discipline applied at the methodology layer). **Mitigation:** the operator should not downgrade their `bm` binary; if they do, they accept the loss of tag data on next write. Layer 2 Round 1 Security F1 + Solution Architect F5 surfaced this as a load-bearing gap; this paragraph documents the deliberate-choice + accepted-risk shape so a future cold-reader knows the asymmetric `serde` behavior is intended, not a defect.

## Open questions

*(none at the close of Phase 1a — the self-adversary check completed cleanly because the project's scope is small and the contracts are observable from outside the implementation. Any ambiguities surfaced during Phase 2 or Phase 3 will be routed back to this section per [Phase 4](../../vsdd-suite/primers/4-feedback-integration.md) routing discipline.)*
