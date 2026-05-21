# Changelog

## v0.11.5 Round 3 inline fix-cycle + cluster verification — 2026-05-20 22:00Z ([Review 82](../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z) Finding 5)

**Scope:** Round 3 of the Phase 3 IAR cycle. Inline fix batch (Round 2 carryforward fixes) + 4-cluster cold-session verification with adversarial-pair separation (engineering / security+ux / red-team+technical-writer / documentation-reviewer+solution-owner) + mid-round inline-fix sub-cycle for 7 new findings.

### Changed — code

- **[`src/lib.rs:330-380`](src/lib.rs)** `is_format_char` — curated Unicode-format-category matcher extended with named bypass codepoints: U+00AD (SOFT HYPHEN; classic invisible URL-spoof primitive); U+0600–0605 + U+06DD + U+070F + U+08E2 (Arabic / Syriac number-sign + abbreviation-mark format chars); U+110BD + U+110CD (Kaithi number sign + end-of-text marker); U+13430–13438 (Egyptian hieroglyph format controls); U+1BCA0–1BCA3 (Duployan shorthand format controls). Doc comment narrowed from claiming full Cf-category coverage to "curated set covering known terminal-escape-injection + Trojan-Source + invisible-glyph spoofing vectors" — full Cf coverage would require a `unicode-general-category` dep + Platform Engineer / Security re-review. Per [Red Team R3 F3](vsdd-suite/review-log/2026-05-20-red-team.md).
- **[`src/lib.rs:86-104`](src/lib.rs)** `BookmarkStore::load` — residual TOCTOU race window documented inline as Accepted Risk per Red Team R3 F2; the symmetric `symlink_metadata` check still catches the synchronous case; tight `O_NOFOLLOW` single-syscall fix deferred pending `libc` dep addition.
- **[`src/main.rs:32-48`](src/main.rs)** CLI `long_about` — audit-trail-trivia footer removed (was leaking `"Closes UX Review 1 Finding 4 (help-text usage example gap)."` into user-visible `bm --help` output). Per [UX R3 F6](vsdd-suite/review-log/2026-05-20-ux.md).
- **[`src/main.rs:79-98`](src/main.rs)** `emit_storage_error` load Hint — expanded to cover corrupt-JSON case (most common failure after first successful use). Per [UX R3 F7](vsdd-suite/review-log/2026-05-20-ux.md).

### Changed — spec ([`DESIGN.md`](DESIGN.md))

- **§ Threat model — `$BOOKMARK_CLI_DB` Mitigations row** — documented residual load-side TOCTOU race window per [Red Team R3 F2](vsdd-suite/review-log/2026-05-20-red-team.md) (Accepted Risk classification); tight fix path (`OpenOptions::custom_flags(O_NOFOLLOW)`) declared with `libc` dep + Security re-review as the gate.

### Changed — docs

- **[`README.md:19-20,41`](README.md)** — angle-bracket placeholders `<portfolio-url>` / `<portfolio>` rewritten as UPPERCASE-KEBAB-CASE (`PORTFOLIO-URL` / `PORTFOLIO`) per the markdown supplement § Code blocks placeholder convention. Per [TW R3 F4](vsdd-suite/review-log/2026-05-20-technical-writer.md).
- **[`manual-tests/layer-1.md:14-21`](manual-tests/layer-1.md)** Step 0 — added missing `echo "exit: $?"` line + literal-expected-output discipline match with Steps 1/3/4/5/6 per [UX R3 F8](vsdd-suite/review-log/2026-05-20-ux.md).
- **16-substitution mechanical sweep** across forward-facing markdown files (DESIGN.md, PROCESS.md, vsdd-suite/SOLUTION-ARCHITECT-REVIEW.md, vsdd-suite/QUALITY-ENGINEER-REVIEW.md, vsdd-suite/FINDINGS-INDEX.md, TODO.md) — retired letter-coded "Surface A/B/C/D" duplicate-name sweep artifacts from Review 78 letter retirement; `1ab-spec-development.md` → `1ab-spec-crystallization.md` per the Review 81 anchor-link sweep. Per [Documentation Reviewer R2 carryforwards](vsdd-suite/review-log/2026-05-20-documentation-reviewer.md) + [TW R3 F3 + F5](vsdd-suite/review-log/2026-05-20-technical-writer.md).

### Changed — review-log discipline

- **9 per-domain Round 3 review-log sections** appended to existing per-session files at [`vsdd-suite/review-log/2026-05-20-{domain-slug}.md`](vsdd-suite/review-log/) under `## Review 3 — 2026-05-20 22:00Z` headings (per-session-file convention; one file per date+domain; multiple Reviews share the file).
- **4 intermediate cluster files deleted** at consolidation (`engineering-cluster-round-3.md`, `cluster-b-round-3.md`, `cluster-c-round-3.md`, `cluster-d-round-3.md`). The letter-named cluster files were the operator-flagged TW Dim 12 naming-discipline slip; the canonical audit trail never sees the lettering.
- **[`vsdd-suite/FINDINGS-INDEX.md`](vsdd-suite/FINDINGS-INDEX.md)** — Quick lookup preamble updated with Post-Round-3 status (7 of 10 at MVR; 2 operator-gated; 1 Deferred-carryforward); Round 2 + Round 3 finding-rows policy declared (per-session anchors are the canonical lookup, not registry-row duplication).

### Per-domain Round 3 outcomes

| Domain | Outcome |
|---|---|
| Software Engineer | MVR reached |
| Performance Engineer | MVR-blocked-by-Deferred (R2-F7 fsync benchmark — Layer 2 operator-executable) |
| Platform Engineer | MVR-blocked-by-operator-gate (R2-F9 install-verification non-author fresh-system requirement; AI-unsatisfiable per [G-155](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-155)) |
| Security | MVR reached |
| UX | MVR reached (F6 + F7 + F8 inline-fixed mid-Round-3) |
| Red Team | MVR reached (F2 Accepted Risk; F3 inline-fixed) |
| Technical Writer | MVR reached (F3 + F5 already-Resolved by sweep; F4 inline-fixed; F6 Hallucinated) |
| Documentation Reviewer | NOT at MVR — 5 Deferred R2 carryforwards remain (sweep-discipline gap; routed to PR [#40](https://github.com/magnificentlycursed/guild-portfolio/pull/40) upstream-suite-remediation) |
| Solution Owner | MVR reached |
| VDD-IAR Alignment | MVR reached (from Round 2; no R3 per G-131 — continue trigger requires new real findings) |

### Verified

- `cargo fmt --check` clean.
- `cargo clippy --all-targets -- -D warnings` clean.
- `cargo test` — 11 unit + 16 integration tests pass.

### Phase 6 status

**Phase 6 four-dimensional convergence record continues DEFERRED.** Platform Engineer install-verification operator-gate is the hard ceiling (Dim 38 fresh-system requirement is AI-unsatisfiable per [G-155](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-155)). The four-dimensional attestation (Spec MVR + Test MVR + Implementation MVR + Formal-verification MVR + cross-dimension consistency check) cannot honestly claim Implementation MVR while the Platform Engineer gate is open. Phase 6 will be authored as the FINAL VDD-IAR Alignment review round once: (a) operator runs install-verification on a fresh system, and (b) Documentation Reviewer sweep-discipline carryforwards close via PR [#40](https://github.com/magnificentlycursed/guild-portfolio/pull/40) upstream-suite-remediation.

---

## v0.11.4 Round 2 fix-cycle — 2026-05-20 20:00Z ([Review 82](../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-82--2026-05-20-2000z))

**Scope:** Round 2 fix-cycle covering all 80 findings filed across the 12-domain Round 1 cold-context IAR pass. The fix-cycle is split across four parallel batches — spec / code / config / CI / docs — coordinated against the updated [`DESIGN.md`](DESIGN.md) as the contract. This entry covers the doc batch; the other batches land in their own commits.

### Changed — spec ([`DESIGN.md`](DESIGN.md))

- **§ Behavioral contracts** — `bm add` (no positional argument) now treated identically to `bm add ""` per [SE Review 1 Finding 1](vsdd-suite/review-log/2026-05-20-software-engineer.md); atomic-write semantics declared for `bm add` storage write (temp file + atomic rename per POSIX `rename(2)`) per [SE Review 1 Finding 2](vsdd-suite/review-log/2026-05-20-software-engineer.md); CLI usage error (unknown subcommand / unknown flag) now exits 64 per `sysexits.h` `EX_USAGE` to disambiguate from exit 2 storage errors per [SE Review 1 Finding 3](vsdd-suite/review-log/2026-05-20-software-engineer.md).
- **§ Exit codes table** — new row for exit 64 (CLI usage error) per `sysexits.h` `EX_USAGE`.
- **§ Performance budget** (new section) — Layer 1 commitments: `bm --help` / `bm --version` startup < 50 ms p95; `bm add` / `bm list` end-to-end < 100 ms p95 at ≤ 1,000 bookmarks; scale ceiling 10,000 bookmarks; benchmarking infrastructure deferred to Layer 2+ per [Performance Engineer Review 1](vsdd-suite/review-log/2026-05-20-performance-engineer.md).
- **§ Threat model** (new section) — in-scope adversaries (co-tenant on shared Unix host; adversary-controlled `$BOOKMARK_CLI_DB`; adversary-supplied URL contents → terminal-escape / bidi / zero-width chars); mitigations (mode 0600; symlink-follow rejection; `display_safe` sanitizer); out-of-scope adversaries with acceptance rationale per [Security Review 1](vsdd-suite/review-log/2026-05-20-security.md) + [Red Team Review 1](vsdd-suite/review-log/2026-05-20-red-team.md).
- **§ Storage data classification** (new section) — captured bookmarks classified *confidential*; storage file written with mode 0600 (Unix; `#[cfg(unix)]` gated); Windows file-permission semantics deferred per [Security Review 1 Finding 2](vsdd-suite/review-log/2026-05-20-security.md).

### Changed — code (owned by the code-fix agent; lands in parallel commits)

- **`src/lib.rs`** `BookmarkStore::save` — atomic-write semantics (temp file in destination dir + atomic rename); symlink rejection on save target; file mode 0600 set via `OpenOptions::mode()` behind `#[cfg(unix)]` gate.
- **`src/lib.rs`** — `display_safe` sanitizer added; wraps every user-derived value before any `eprintln!` / `println!` / `Display` interpolation; escapes `is_control()` (Cc) chars + `Cf` format chars while preserving `\n` `\t`.
- **`src/lib.rs`** `BookmarkStore` — field-level encapsulation; rustdoc on every `pub` item; `#![deny(missing_docs)]` lint enabled at crate level.
- **`src/main.rs`** — missing-positional-argument path intercepts clap's default exit 2 usage-error and routes through the spec-contracted exit 1 + `Error: URL cannot be empty.\n` shape; unknown-subcommand / unknown-flag path intercepts clap's default and emits exit 64.
- **`tests/bookmarks.rs`** — new integration tests covering atomic save, symlink rejection, file mode 0600 on Unix, sanitizer, missing-arg parity with empty-string, unknown-subcommand exit 64.

### Changed — config + CI (owned by config-fix + CI-fix agents)

- **`Cargo.toml`** — lint floor (clippy + missing_docs); `[lints.rust]` block enabling `missing_docs = "deny"`.
- **`rust-toolchain.toml`** (new) — pinned per [Platform Engineer Review 1 Finding 2](vsdd-suite/review-log/2026-05-20-platform-engineer.md).
- **`deny.toml`** (new) — `cargo deny check` policy per [Security Review 1 Finding 3](vsdd-suite/review-log/2026-05-20-security.md) + [Platform Engineer Review 1 Finding 4](vsdd-suite/review-log/2026-05-20-platform-engineer.md).
- **`.github/workflows/`** (new) — GitHub Actions workflow: build + test + clippy + cargo-deny on push / PR per [Platform Engineer Review 1](vsdd-suite/review-log/2026-05-20-platform-engineer.md).

### Changed — docs (this batch)

- **[`README.md`](README.md)** — test-count claim updated from "8 tests pass (4 lib + 4 integration)" to a stable-across-fix-cycle framing referencing the current ~19-test suite ([Technical Writer Review 1 Finding 2](vsdd-suite/review-log/2026-05-20-technical-writer.md) + [Documentation Reviewer Review 1 Finding 2](vsdd-suite/review-log/2026-05-20-documentation-reviewer.md) + [UX Review 1 Finding 2](vsdd-suite/review-log/2026-05-20-ux.md)); install-path `cd <portfolio>/bookmark-cli` → `cd <portfolio>/vsdd-suite-reference-examples/bookmark-cli-manual` ([Documentation Reviewer Review 1 Finding 1](vsdd-suite/review-log/2026-05-20-documentation-reviewer.md)); relative-depth fix `../vsdd-suite/README.md` → `../../vsdd-suite/README.md` ([Documentation Reviewer Review 1 Finding 3](vsdd-suite/review-log/2026-05-20-documentation-reviewer.md)); VSDD / IAR / MVR / TDD acronyms expanded on first use ([Documentation Reviewer Review 1 Finding 12](vsdd-suite/review-log/2026-05-20-documentation-reviewer.md)); Phase 4 row updated to reflect Round 2 fix-cycle routing ([Solution Owner Review 1 Finding 1](vsdd-suite/review-log/2026-05-20-solution-owner.md)); `--locked` flag added to every `cargo install` invocation.
- **[`TODO.md`](TODO.md)** — "10 active domains" → "12 active domains" per [Documentation Reviewer Review 1 Finding 8](vsdd-suite/review-log/2026-05-20-documentation-reviewer.md); Documentation Reviewer added to the layer-gate criterion 4 capstone-extended list; retired letter-coded "Surface A.0 / B" verbiage replaced with descriptive "Purity Boundary Audit + Mutation Testing" Title-Case names ([Technical Writer Review 1 Finding 4](vsdd-suite/review-log/2026-05-20-technical-writer.md) + [Documentation Reviewer Review 1 Finding 6](vsdd-suite/review-log/2026-05-20-documentation-reviewer.md)).
- **[`manual-tests/layer-1.md`](manual-tests/layer-1.md)** — Step 1 expected on-disk JSON shape corrected from bare array to object-wrapped `{"bookmarks": [...]}` form matching the [DESIGN.md § Storage format](DESIGN.md#storage-format-json-file) spec ([Solution Owner Review 1 Finding 2](vsdd-suite/review-log/2026-05-20-solution-owner.md) + [UX Review 1 Finding 3](vsdd-suite/review-log/2026-05-20-ux.md)); Step 5 `cd` made absolute-path-safe by capturing `$PROJECT_DIR` before uninstall ([UX Review 1 Finding 6](vsdd-suite/review-log/2026-05-20-ux.md)); Step 5 `which bm` post-uninstall expectation relaxed from literal-match "bm not found" to a behavioral assertion (non-zero exit + no path printed; exact textual output is shell-dependent) ([UX Review 1 Finding 7](vsdd-suite/review-log/2026-05-20-ux.md) + [Documentation Reviewer Review 1 Finding 11](vsdd-suite/review-log/2026-05-20-documentation-reviewer.md)); session-state preamble added naming the single-uninterrupted-shell-session-OR-absolute-`BOOKMARK_CLI_DB` requirement ([Documentation Reviewer Review 1 Finding 13](vsdd-suite/review-log/2026-05-20-documentation-reviewer.md)); new Step 6 verifies file mode 0600 on Unix via `stat -f %A` (macOS) / `stat -c %a` (Linux); `cargo install` invocations updated to use `--locked`.
- **[`manual-tests/install-verification.md`](manual-tests/install-verification.md)** — sibling-link path corrections (`manual-tests/layer-1.md` → `layer-1.md`; `PROCESS.md` → `../PROCESS.md`; `DESIGN.md` → `../DESIGN.md`) ([Technical Writer Review 1 Finding 3](vsdd-suite/review-log/2026-05-20-technical-writer.md) + [Documentation Reviewer Review 1 Finding 10](vsdd-suite/review-log/2026-05-20-documentation-reviewer.md)); AI-co-authored disclosure framing made explicit with "AI-author cannot satisfy this gate" reminder and operator-required `Outcome` row reminder.
- **[`PROCESS.md`](PROCESS.md)** — broken primer reference `1ab-spec-development.md` → `1ab-spec-crystallization.md` corrected ([Documentation Reviewer Review 1 Finding 4](vsdd-suite/review-log/2026-05-20-documentation-reviewer.md)); new "Round 1 IAR + Round 2 fix-cycle retrospective" section summarizing the 80 findings + the four-batch fix shape + the operator-gated install-verification remainder (covers the spec / code / docs / config / CI fix highlights and the [Technical Writer Review 1 Finding 1](vsdd-suite/review-log/2026-05-20-technical-writer.md) NUL-byte sentinel closure).

### Note

**Install-verification gate remains operator-pending.** [G-155](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-155) Platform Engineer Dim 38 fresh-system non-author install verification cannot be satisfied by any AI session — by construction. The Phase 6 four-dimensional convergence is therefore still deferred until the operator executes the fresh-system install attempt and records a PASS row in [`manual-tests/install-verification.md`](manual-tests/install-verification.md). Every other Round 1 domain finding reaches MVR or zero-findings under this Round 2 fix-cycle.

---

## vsdd-suite v0.11.2 Documentation Reviewer activated at capstone — 2026-05-20 18:30Z (PR [#36](https://github.com/magnificentlycursed/guild-portfolio/pull/36) / [Review 80](../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z))

**Scope:** Activates [Documentation Reviewer](../../vsdd-suite/domains/role/DOCUMENTATION-REVIEWER-REVIEW.md) on this reference example. Doc Reviewer is the adversarial cold-reader pair to [Technical Writer](../../vsdd-suite/domains/role/TECHNICAL-WRITER-REVIEW.md), registered in [Review 80](../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z); both activate together at capstone intent.

### Changed

- **`DESIGN.md`** § Project intent — Active domain set expanded from **10 role + 1 meta = 11** → **11 role + 1 meta = 12** (Documentation Reviewer added to the capstone-tier extended domains). Anchor-link convention applied to the active-domain-set declaration (each domain name now links to its prompt file).

### Added

- **`vsdd-suite/DOCUMENTATION-REVIEWER-REVIEW.md`** (new project per-domain index stub) — Activation rationale + validator-pair declaration + language-supplement-load reference + sycophancy-check excerpt. Reviews-table empty; rounds populate when the cold-session Documentation Reviewer round runs as part of the queued 6-phase IAR execution.

---

## vsdd-suite v0.11.0 capstone-intent promotion + 6-phase preparation — 2026-05-20 16:30Z (PR 6 / Review 78)

**Scope:** Promotes `bookmark-cli-manual` from `portfolio` intent to `capstone` intent. Reference examples track current conventions per G-177 precedent — bookmark-cli-manual exists to teach the worked example end-to-end through all 6 VSDD phases, which requires capstone-intent bar. This PR lands the STRUCTURAL preparation; the cold-session IAR rounds for the newly-activated capstone domains + the Phase 6 four-dimensional convergence record + FINDINGS-INDEX repopulation land in **PR 7**.

### Changed

- **`DESIGN.md`** § Project intent — promoted to `capstone` with intent-transition note. Active domain set expanded from 7 (portfolio default) to **10 role + 1 meta = 11 domains** for capstone: 6 core role (SE, QE, UX, Security, SA, SO) + VDD-IAR Alignment meta + 4 extended (Performance Engineer for capstone-required activation; Platform Engineer for G-178 strong-presumption + G-155 dim 38 fresh-system install verification; Red Team for capstone-tier adversarial intensity; Technical Writer for portfolio+ external-reading activation given bookmark-cli-manual's reference-example role). Data Engineer evaluated and ruled out — bookmark-cli's flat JSON storage falls below the G-178 activation threshold; the absence is documented as deliberate. Sanity Check meta domain (Review 77 Finding 2) available on-demand for findings without natural cross-domain pair. Historical portfolio-intent declaration (Review 67) preserved as the historical-narrative anchor below the current declaration per G-89 forward-only.
- **`DESIGN.md`** § Project intent Phase 6 strategy — promoted from `not applicable` to `planned` with concrete scope naming the four dimensions + the cross-dimension consistency check + the signed closing attestation. Per G-162: capstone-intent declarations require both Phase 5 + Phase 6 strategy lines; both now declared.
- **`TODO.md`** Layer 1 — `**Manual Testing Checklist:**` block split out per Review 74 convention (the inline runnable-step shell-script block became a one-line pointer to the new per-layer file). Layer-gate criteria expanded from 4 to 6 reflecting capstone-active domain set + Phase 5 + Phase 6.
- **`TODO.md`** Layer 1 — new **Phase 2c (refactor):** annotation declaring `no refactor required` per `primers/2c-refactor.md` § Completion criteria #5 explicit-skip pattern. Layer 1's implementation already exhibits the refactor primer's scope-catalog idioms; explicit-skip annotation satisfies VDD-IAR Alignment dim 12 (Phase 2c refactor discipline per G-161).
- **`vsdd-suite/review-log/2026-05-17-quality-engineer.md`** Reviews 1's Findings 1+2 + **`vsdd-suite/review-log/2026-05-20-quality-engineer.md`** Review 2 Finding 1 + **`vsdd-suite/review-log/2026-05-20-solution-architect.md`** Review 1 Finding 1 — migrated with Review 77 lifecycle fields (`**Owner:**` / `**Status:**` / `**Blocked by:**` / `**Validator:**`) per G-177 reference-example-migrates precedent. Each file gained a migration-note paragraph documenting the retroactive field addition. Hallucinated findings (QE Review 1 Finding 3) exempt from lifecycle fields per Review 77. The fields are aspirational on these pre-2026-05-21 dates (the hook's lifecycle-field enforcement gates on 2026-05-21+); the next-day Review-77-enforced rounds in PR 7 will carry the fields under the enforced standard.
- **`vsdd-suite/FINDINGS-INDEX.md`** — schema extended with `Owner` + `Validator` columns per Review 77; existing 5 rows updated with migrated Owner/Validator values per the per-round migration above. Quick-lookup section retained.

### Added

- **`manual-tests/layer-1.md`** (new ~140 lines) — Layer 1 manual-test plan split out from `TODO.md` per Review 74 convention. 6 step blocks (Step 0 binary install / Step 1 happy path / Step 2 error state / Step 3 list ordering / Step 4 empty-state / Step 5 persistence with uninstall+reinstall lifecycle) with literal expected-output blocks per the runnable-step standard. Closure protocol section at end naming the two outcome shapes (insight-reached/no findings vs findings-surfaced) per primer 3 § Manual testing is a second adversarial surface to IAR (G-132).
- **`PROCESS.md`** (new ~80 lines) — first-person retrospective skeleton per G-156 layer-gate close criterion 7. **AI-co-authored reference-example disclosure** at the top makes explicit that the discipline G-156 specifies (director-authored prose) is NOT satisfied by AI-authored scaffold prose; the file demonstrates the FORMAT for an actual capstone project, with section structure (What was hardest / What I got wrong / What the process felt like per layer) but with AI-authored placeholder prose that must be overwritten by the director to satisfy the gate. The disclosure complies with the operator's earlier directive on AI-co-authored artifacts.
- **`manual-tests/install-verification.md`** (new ~70 lines) — Platform Engineer Dim 38 third-party install verification record per G-155. **AI-co-authored disclosure** at top makes explicit that the AI cannot satisfy this gate (the discipline's load-bearing requirement is non-author verification on a fresh system); the file documents the verification procedure the operator would execute. Verification table is scaffolded with pending row. **File location convention (Review 78 Finding 2):** lives in `manual-tests/` because install-verification IS a manual test (operator runs commands on a fresh system and records observations); lowercased + hyphenated filename (`install-verification.md`) parallels the per-layer pattern (`layer-N.md`).
- **`vsdd-suite/PERFORMANCE-ENGINEER-REVIEW.md`** + **`vsdd-suite/PLATFORM-ENGINEER-REVIEW.md`** + **`vsdd-suite/RED-TEAM-REVIEW.md`** + **`vsdd-suite/TECHNICAL-WRITER-REVIEW.md`** (4 new files) — per-domain index files for the 4 newly-capstone-activated extended domains. Each file is customized for bookmark-cli-manual with activation rationale + supplement references + sycophancy-check excerpt + empty Reviews table (rounds populate in PR 7).
- **`vsdd-suite/SOFTWARE-ENGINEER-REVIEW.md`** + **`vsdd-suite/UX-REVIEW.md`** + **`vsdd-suite/SECURITY-REVIEW.md`** + **`vsdd-suite/SOLUTION-OWNER-REVIEW.md`** + **`vsdd-suite/VDD-IAR-ALIGNMENT-REVIEW.md`** — the 5 pre-existing scaffolded stubs were customized for bookmark-cli-manual (template placeholders filled with domain-specific values; reading convention + Reviews table sections completed).

### Note

**Backlog after PR 6: 0 Open findings.** The cold-session rounds for the 9 not-yet-reviewed-at-capstone domains (SE, UX, Security, SO, VDD-IAR Alignment + Performance Engineer, Platform Engineer, Red Team, Technical Writer) + Phase 6 four-dimensional convergence record + FINDINGS-INDEX row repopulation land in **PR 7**. The structural preparation here (DESIGN.md intent declaration; manual-test split; existing-rounds migration; per-domain index scaffolds; PROCESS.md + `manual-tests/install-verification.md` skeletons; FINDINGS-INDEX schema migration) is reviewable in isolation. PR 7 will execute the IAR rounds against the prepared structure. Forward-only per G-89: pre-2026-05-21 reviews (QE R1, QE R2, SA R1) retain their original framings under portfolio intent + are augmented with Review 77 lifecycle fields per the operator's migration directive.

---

## vsdd-suite v0.7.8 migration — 2026-05-20

**Scope:** Migrates `bookmark-cli` from the prior `vsdd-suite/PHASE-5-LOG.md` per-project file shape to the per-domain review log shape that vsdd-suite v0.7.8 prescribes (G-177 closure — operator-promoted from Deferred to Addressed). bookmark-cli is the suite's reference example, so it tracks the current convention rather than the forward-only carve-out.

### Removed

- **`vsdd-suite/PHASE-5-LOG.md`** (deleted) — the per-project Phase 5 record. All substantive content (purity-boundary audit table; cargo-mutants pre-B1 / post-B1 outputs; per-mutant disposition table; strategy declaration) was already mirrored in the per-domain rounds at `vsdd-suite/review-log/2026-05-20-solution-architect.md#review-1` (Purity Boundary Audit) and `vsdd-suite/review-log/2026-05-20-quality-engineer.md#review-2` (Mutation Testing). The deletion eliminates the duplication G-177 named.

### Changed

- **`vsdd-suite/review-log/2026-05-20-solution-architect.md#review-1`** — added `**Phase 5 hardening:** Purity Boundary Audit — Purity Boundary Audit for Layer 1` preamble tag per G-177 v0.7.8 convention. Removed the `../PHASE-5-LOG.md` cross-references from the Scope line and Coordination line.
- **`vsdd-suite/review-log/2026-05-20-quality-engineer.md#review-2`** — added `**Phase 5 hardening:** Mutation Testing — Mutation Testing for Layer 1 via cargo-mutants` preamble tag. Removed the `../PHASE-5-LOG.md` cross-references from the Scope line, the unviable-mutants paragraph, and the Coordination line.
- **`vsdd-suite/QUALITY-ENGINEER-REVIEW.md`** — Reviews-table row for Review 2 reworded to name the `**Phase 5 hardening:** Mutation Testing` preamble explicitly and remove the `../PHASE-5-LOG.md` final-sentence citation.
- **`DESIGN.md`** — § Project intent Phase 5 strategy line + § Verification architecture Phase 5 hardening bullet reworded to cite the per-domain logs (SOLUTION-ARCHITECT-REVIEW.md for Surfaces A/A.0/D; QUALITY-ENGINEER-REVIEW.md for Surfaces B/C) instead of `vsdd-suite/PHASE-5-LOG.md`.
- **`src/lib.rs::tests::save_creates_parent_directory_for_nested_path`** — doc comment updated to cite `vsdd-suite/QUALITY-ENGINEER-REVIEW.md` (Review 2 — Phase 5 Mutation Testing) instead of `vsdd-suite/PHASE-5-LOG.md`.

### Note
Prior CHANGELOG entries that reference `vsdd-suite/PHASE-5-LOG.md` (the v0.7.2 adoption entry below) are preserved as historical-narrative records per G-89 forward-only narrative-preservation policy. The references in those entries reflect the state at the time of writing; the current state is described in this entry.

No `PHASE-6-CONVERGENCE.md` ever existed on this project (bookmark-cli is portfolio-intent with `Phase 6 strategy: not applicable`); no Phase 6 migration is required.

---

## v0.7.2 adoption + Phase 5 Layer 1 hardening — 2026-05-20 02:45Z

**Scope:** Adopts the vsdd-suite v0.7.2 conventions (G-150 intent calibration, G-162 Phase 5/6 strategy declarations, G-55 Phase 5 ownership, G-54 Phase 6 ownership). Closes the 2 held findings (B1, B2) from the Review 66 Phase 5 test that had been blocked pending operator authorization on the reference impl. Lands on dedicated branch `bookmark-cli-phase5-adoption`.

### Added

- **`DESIGN.md`** — new `## Project intent` section declaring `portfolio` intent + `**Phase 5 strategy:**` (planned — Surfaces A.0 + B; A deferred; C + D not applicable) + `**Phase 6 strategy:**` (not applicable — portfolio-intent closes at end of Phase 4 by design).
- **`DESIGN.md` § Verification architecture** — new explicit Purity boundary subsection enumerating each function's purity status (pure: data types + `newest_first`; effectful: `load`/`save` I/O wrappers; boundary refinement: `add` clock-read). New Phase 5 hardening reference pointing at `vsdd-suite/PHASE-5-LOG.md`. New formal-proof-candidates declaration (none). New automatable-vs-manual split. Resolves B2 (cross-source purity divergence).
- **`vsdd-suite/PHASE-5-LOG.md`** (new file) — per-layer Phase 5 hardening record per the v0.7.2 primer format. Documents the Layer 1 Purity Boundary Audit + B run: tool-install cost; full pre-B1 and post-B1 cargo-mutants output; per-mutant disposition table (5-disposition universe per G-174); Surfaces A + C + D dispositions per the project's strategy.
- **`vsdd-suite/review-log/2026-05-20-quality-engineer.md`** (new file) — QE Review 2 entry filing the Mutation Testing portion of the Phase 5 round per the suite's project-level review log governing standard. One Resolved finding (Mutation Testing missing-test); coordination line points at SA Review 1 for the Purity Boundary Audit portion.
- **`vsdd-suite/review-log/2026-05-20-solution-architect.md`** (new file) — SA Review 1 entry filing the Purity Boundary Audit portion of the Phase 5 round. One Resolved finding (Dim 12 — VSDD purity boundary map cross-source divergence). First SA review filed against bookmark-cli; coordination line points at QE Review 2 for the Mutation Testing portion.
- **`vsdd-suite/SOLUTION-ARCHITECT-REVIEW.md`** — index customized from the scaffolded stub (Reviewer role line, sycophancy check, language supplement reference filled in; first row added for SA Review 1). Prior state was placeholders.
- **`vsdd-suite/FINDINGS-INDEX.md`** — new rows F-004 (SA Dim 12 purity-boundary divergence, Resolved) and F-005 (QE Mutation Testing missing-test, Resolved); Cross-references list updated to note SA-REVIEW.md is now customized.
- **`src/lib.rs::tests::save_creates_parent_directory_for_nested_path`** — falsifying test for the previously-surviving cargo-mutants mutant at `src/lib.rs:48`. Labeled `retroactive Red Gate (Phase 5 source)` per the Review 65 / F7 label extension in `primers/2b-implementation.md`. Verified end-to-end: post-B1 cargo-mutants kill rate on viable mutants is 8/8 = 100% (was 7/8 = 87.5% pre-B1). Resolves B1 (test gap).

### Changed

- **`src/lib.rs:1-7`** module doc — retired the prior "Pure-core storage logic" claim; cites `DESIGN.md` § Verification architecture as the single authoritative source. Module-doc summary names the impl's actual purity boundary (pure / effectful / boundary-refinement categories) matching DESIGN.md. Resolves the cross-source divergence half of B2 (G-173 multi-source check).
- **`DESIGN.md`** H1 / preamble — updated from "Phase 1a contract" to "Phase 1a+1b contract" per the G-96 / G-160 v0.6.0 rename; cites the renamed primer at `primers/1ab-spec-crystallization.md`. Forward-only narrative-preservation per G-89: the file was originally authored under the prior "Phase 1a" single-step naming and that historical narrative is preserved.
- **`vsdd-suite/QUALITY-ENGINEER-REVIEW.md`** Reviews table — new row for Review 2 (the Phase 5 round).
- **`.gitignore`** — added `mutants.out` and `mutants.out.old` (cargo-mutants output directories).

### Note

`bookmark-cli` is the suite's reference implementation; the v0.7.2 adoption above is the first project-side application of the Phase 5 + Phase 6 methodology since G-54 + G-55 closed in suite Review 64. The Phase 5 primer's prescriptions worked as written for both surfaces activated (A.0 audit produced a real cross-source finding; B disposition table mapped cleanly to cargo-mutants' actual output shape including the 5th `unviable` class added in suite Review 66 / G-174). The forward-only G-89 carve-out applies: bookmark-cli's Layer 1 first-gate-close predates 2026-05-20; this adoption is an explicit opt-in, not retroactive enforcement.
