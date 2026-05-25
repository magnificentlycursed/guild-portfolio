<!-- hook-bypass: this is a Phase 4 routing record per primers/4-feedback-integration.md § [manual] First-class fallback path, not a per-domain review log; the per-domain review-discipline hook's classification-section convention does not apply to routing records. -->

# Phase 4 — Feedback Integration Routing — Layer 3 Round 1

## Routing Pass 1 — 2026-05-25 02:00Z

**Source:** Phase 3 Round 1 aggregate finding set across 13 capstone-active domains (per `vsdd-suite/review-log/2026-05-24-<domain-slug>.md` files; commit `2acc418`).

**Scope:** Layer 3 cycle (commits `878d3b6` Phase 2a + `fd21900` Phase 2b + `78bd3cf` Phase 2c). All routable findings (Deferred / Backlogged / Raised-to-SO classifications) from Round 1.

**Mode:** manual — per `vsdd-suite/primers/4-feedback-integration.md` § [manual] First-class fallback path. The per-finding routing decisions land in this consolidated document; each per-domain Phase 3 review log file gains a one-line cross-reference to its routing block here.

**SO-decisions captured** via main-session AskUserQuestion pass on 2026-05-25 (5 substantive decisions):
1. Cluster B `display_safe` round-trip → **Path C** (switch to JSON-native `\uHHHH` escape syntax)
2. Cluster C tags dedup → **Path A** (sorted-tag-comparison)
3. SO F1 `manual-tests/layer-3.md` → **Author the file**
4. Cluster H tag-injection L3 → **Path B** (active mitigation; reject control-char tags at import)
5. Cluster I imported-tag class → **Path A** (same classification as user-typed tags)

Smaller items routed via conservative defaults (operator may flag any to revise).

---

## Routing summary by cluster

### Cluster B — `display_safe` round-trip (highest severity; 4-domain convergence)

**Operator decision:** Path C — switch `display_safe` from Rust-syntax `\u{HHHH}` (8-byte literal) to JSON-native `\uHHHH` (6-char escape). Preserves both terminal-safety AND byte-round-trip.

**Findings routed:** [SA R5 F3](2026-05-24-solution-architect.md), [SE R1 F1](2026-05-24-software-engineer.md#r1-f1), [RT R1 F3](2026-05-24-red-team.md#r1-f3), [Sec R1 F1](2026-05-24-security.md#r1-f1)

**Route:** `Phase 2a → Phase 2b → Phase 1a+1b` (multi-phase chain)
- **Phase 2a (regression test):** new test in `tests/bookmarks.rs` exercising true byte-preservation through `bm export | bm import` round-trip — payload includes pathological URL with raw ESC + control bytes; assert post-import store contains the SAME bytes as the source store (currently fails because Rust-syntax `\u{HHHH}` becomes 8-byte literal at import).
- **Phase 2b (impl fix):** `src/lib.rs` `display_safe` function — change the escape format from `format!("\\u{{{:04x}}}", c as u32)` to `format!("\\u{:04x}", c as u32)` for BMP codepoints (≤ U+FFFF); surrogate-pair encoding for higher codepoints. All existing display_safe call-sites benefit (export + list + tag rendering).
- **Phase 1a+1b (spec amendment):** DESIGN.md § `bm export` (Layer 3) update the byte-preservation paragraph to name the JSON-native escape design explicitly. Also update DESIGN.md § Edge case catalog § Layer 3 entry on display_safe.

**Owning artifact:** `src/lib.rs` (display_safe); `tests/bookmarks.rs` (round-trip regression test); `DESIGN.md` § Behavioral contracts § `bm export` (Layer 3) + § Edge case catalog.

**Gate:**
- Regression test commits in standalone Phase 2a commit (RED against current Rust-syntax impl);
- `display_safe` rewrite passes the regression test + all 45 prior tests + clippy clean;
- DESIGN.md amendment lands with the spec-vs-impl alignment in writing;
- Validator: SE (impl correctness) + Security (terminal-safety preserved) + RT (round-trip verified).

**Sequencing:** Blocks Layer 3 layer-gate close. Round 2 of Phase 3 IAR re-runs SA + SE + RT + Sec for validation.

---

### Cluster C — Tags dedup order-sensitivity (2-domain convergence)

**Operator decision:** Path A — dedup on sorted-tag-comparison. Resolves spec internal tension toward L223 set-frame; storage `tags` Vec preserves insertion order (Layer 2 semantic intact); only the dedup comparison is set-frame.

**Findings routed:** [SE R1 F2](2026-05-24-software-engineer.md#r1-f2), [RT R1 F1](2026-05-24-red-team.md#r1-f1)

**Route:** `Phase 2a → Phase 2b` (multi-phase chain)
- **Phase 2a (regression test):** new test in `tests/bookmarks.rs` — import a record with `tags: ["rust", "go"]`, then re-import same record with `tags: ["go", "rust"]`; assert second import is dedup'd to zero. Currently fails (impl uses Vec equality so reordered tags create duplicate row).
- **Phase 2b (impl fix):** `src/lib.rs` `import_json` dedup logic — change `self.bookmarks.contains(&new_bm)` to a custom `bookmark_set_eq` predicate that compares (`url`, `timestamp`, `sorted(tags)`). Storage `tags` Vec unchanged.

**Owning artifact:** `src/lib.rs` (import_json dedup); `tests/bookmarks.rs`.

**Gate:**
- Regression test RED against current impl + GREEN after fix;
- DESIGN.md edge-case entry updated to make the sorted-comparison-dedup explicit AND resolve the L132 (byte-equal frame) vs L223 (set-frame) tension toward L223;
- Validator: QE (test discipline) + Security (silent-amplification attack vector closed).

**Sequencing:** Blocks Layer 3 layer-gate close. Folded into Round 2 verification.

---

### Cluster H — Tag-injection escalation at Layer 3

**Operator decision:** Path B — active mitigation. `import_json` rejects records whose `tags` contain control characters / format characters at import time. New `ImportError` variant + new Phase 2a regression test + DESIGN.md amendment.

**Findings routed:** [Sec R1 F2](2026-05-24-security.md#r1-f2)

**Route:** `Phase 1a+1b → Phase 2a → Phase 2b` (multi-phase chain)
- **Phase 1a+1b (spec amendment):** DESIGN.md § `bm import` (Layer 3) — new bullet for "Failure (imported record contains control-char in `tags`)" with stderr `Error: imported bookmark tags contain disallowed control characters.` + exit 1. DESIGN.md § Threat model addition for stdin-fed attacker input — name tag-injection escalation explicitly; document the active-mitigation framing.
- **Phase 2a (regression test):** new test exercising rejection of `tags: ["rust"]` import payload — assert exit 1 + new error message + no file write.
- **Phase 2b (impl fix):** `src/lib.rs` `import_json` — after per-record schema validation, validate each tag against the same predicate `display_safe` uses (any `is_control()` or format-char in tags = reject); new `ImportError::TagContainsControlChars(record_index, tag)` variant + Display + Error impls. The check fires pre-mutation per the existing atomicity discipline.

**Owning artifact:** `DESIGN.md` § `bm import` (Layer 3) + § Threat model; `src/lib.rs` (ImportError + import_json); `tests/bookmarks.rs`.

**Gate:**
- Spec amendment + impl + regression test land together (multi-phase chain);
- Existing 45 tests still pass + new test passes;
- Validator: Security (mitigation effectiveness) + Red Team (residual attack surface verified).

**Sequencing:** Blocks Layer 3 layer-gate close. Triggers Round 2 Security + Red Team re-runs.

---

### Cluster I — Imported-tag confidential-data classification

**Operator decision:** Path A — amend DESIGN.md § Storage data classification so imported-tag provenance inherits the same confidentiality + integrity classification as user-typed tags. Pairs with Cluster H mitigation — once control-char rejection lands, imported tags can be trusted at the same level as typed tags.

**Findings routed:** [Sec R1 F3](2026-05-24-security.md#r1-f3)

**Route:** `Phase 1a+1b` (spec-only)
- **Phase 1a+1b:** DESIGN.md § Storage data classification — add paragraph: "Imported tags (via `bm import`) inherit the same confidentiality + integrity classification as user-typed tags (via `bm tag`). The active control-char rejection at import time (per § `bm import` § Failure (imported record contains control-char in tags)) ensures imported tags meet the same content-shape contract as typed tags."

**Owning artifact:** `DESIGN.md` § Storage data classification.

**Gate:** Spec paragraph lands; Validator: Security.

**Sequencing:** Coordinates with Cluster H landing; should land in the same commit batch.

---

### Cluster A — Layer 3 docs staleness (7-domain confirmation; layer-gate-blocking)

**Sub-cluster A1 — `manual-tests/layer-3.md` absent (operator decision: Author the file)**

**Findings routed:** [PE R7 F7](2026-05-24-performance-engineer.md), [DR R1 F3](2026-05-24-documentation-reviewer.md#r1-f3), [SO R1 F1 Backlogged](2026-05-24-solution-owner.md#r1-f1), [VDD-IAR R1 F4](2026-05-24-vdd-iar-alignment.md#r1-f4), [PFE R7 F1](2026-05-24-platform-engineer.md#r7-f1), [TW R1 F1 indirectly](2026-05-24-technical-writer.md#r1-f1), [DR R1 F4 indirectly](2026-05-24-documentation-reviewer.md#r1-f4)

**Route:** `Phase 2a-equivalent (artifact authoring)`
- Author `manual-tests/layer-3.md` parallel to `manual-tests/layer-{1,2}.md` per Review 74 convention.
- Includes `bm export | bm import` round-trip canonical workflow + cross-machine sync workflow (file-transfer-pipe) + `hyperfine` sanity-check sub-section (closes PE Round 1 F7).
- Each step is runnable top-to-bottom from a clean checkout with literal expected-output blocks.

**Owning artifact:** `manual-tests/layer-3.md` (new file).

**Gate:** File exists; all steps execute cleanly against the bm binary; hyperfine sanity-check produces budget-table values; Validator: PFE.

**Sequencing:** Blocks Layer 3 layer-gate close (criterion 3).

**Sub-cluster A2 — README.md stale ("Layer 3 not built")**

**Findings routed:** [DR R1 F1](2026-05-24-documentation-reviewer.md#r1-f1), [TW R1 F1](2026-05-24-technical-writer.md#r1-f1), [SO R1 F2 Backlogged](2026-05-24-solution-owner.md#r1-f2)

**Route:** `Phase 1a+1b` (narrative update)
- README.md § header phase-progression line, § Run section, test-count claim, and any Layer-3-deferred prose updated to reflect post-Phase-2b state. Add bm export + bm import to the command surface; add the round-trip workflow as an example.

**Owning artifact:** `README.md`.

**Gate:** README accurately reflects Layer 3 active state; test count updated to 58 (45 integration + 13 unit); Validator: TW + DR.

**Sequencing:** Should land before Layer 3 layer-gate close.

**Sub-cluster A3 — CHANGELOG.md missing Phase 2a entry + non-slim-form prose**

**Findings routed:** [DR R1 F2](2026-05-24-documentation-reviewer.md#r1-f2)

**Route:** `Phase 1a+1b` (narrative correction)
- Add a slim-form entry for Phase 2a Red Gate commit `878d3b6` (currently invisible in CHANGELOG narrative).
- Reformat the Phase 2b + Phase 2c entries to slim-form per [Review 93 Finding 1](../../../vsdd-suite/suite-development/review-log/2026-05-24-suite-review.md#r93-f1) (current entries carry full prose; slim-form is the codified shape).

**Owning artifact:** `CHANGELOG.md`.

**Gate:** Phase 2a entry exists; existing L3 entries in slim-form; Validator: DR.

**Sequencing:** Should land before Layer 3 layer-gate close.

**Sub-cluster A4 — PROCESS.md Layer 3 still "(deferred)" stub (G-156 hard gate)**

**Findings routed:** [TW R1 F2](2026-05-24-technical-writer.md#r1-f2)

**Route:** `Phase 1a+1b` (retrospective authoring)
- PROCESS.md § Layer 3 — substantive retrospective per [G-156](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-156) hard gate at capstone+ intent. Topics: spec activation via AI-co-authored first-draft; operator-confirmation pass; Phase 2a/2b/2c lifecycle; 13-domain IAR Round 1 + routing.

**Owning artifact:** `PROCESS.md`.

**Gate:** PROCESS.md § Layer 3 retrospective is non-stub + addresses the G-156 retrospective discipline; Validator: TW.

**Sequencing:** **BLOCKS layer-gate close per G-156.**

**Sub-cluster A5 — FINDINGS-INDEX.md has zero Layer 3 rows**

**Findings routed:** [DR R1 F4](2026-05-24-documentation-reviewer.md#r1-f4)

**Route:** `Phase 1a+1b` (audit-trail registry update)
- vsdd-suite/FINDINGS-INDEX.md — add rows for all Round 1 routable findings using the post-R91-F17 anchor-ID scheme (`<domain-slug>-r1-fN`); skip Hallucinated + Accepted-risk (already terminal-no-route per the registry convention).

**Owning artifact:** `vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/FINDINGS-INDEX.md`.

**Gate:** Round 1 routable findings all have registry rows; Validator: TW + AIE.

**Sequencing:** Should land before Layer 3 layer-gate close.

**Sub-cluster A6 — clap `long_about` omits `bm export` + `bm import`**

**Findings routed:** [UX R1 F1](2026-05-24-ux.md#r1-f1), [TW R1 F3](2026-05-24-technical-writer.md#r1-f3)

**Route:** `Phase 2b` (impl change)
- `src/main.rs` `Cli` `long_about` — add `bm export` + `bm import` to the examples block; add the `bm export | bm import` round-trip as a canonical workflow example; update exit codes summary to reflect Layer 3 paths.

**Owning artifact:** `src/main.rs`.

**Gate:** `bm --help` output enumerates Layer 3 subcommands + the round-trip; Validator: UX.

**Sequencing:** Should land before Layer 3 layer-gate close.

**Sub-cluster A7 — install-verification.md missing Layer 3 G-155 inheritance note**

**Findings routed:** [PFE R7 F3](2026-05-24-platform-engineer.md#r7-f3)

**Route:** `Phase 1a+1b` (docs update)
- Add Layer 3 G-155 inheritance note parallel to the Layer 2 precedent.

**Owning artifact:** `vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/install-verification.md`.

**Gate:** L3 inheritance note added; Validator: PFE.

**Sequencing:** Should land before Layer 3 layer-gate close.

---

### Cluster D — Test coverage gaps (QE R8)

**Findings routed:** [QE R8 F1](2026-05-24-quality-engineer.md#r8-f1) (within-payload byte-equal dedup unexercised), [QE R8 F2](2026-05-24-quality-engineer.md#r8-f2) (tag-element display_safe untested), [QE R8 F3](2026-05-24-quality-engineer.md#r8-f3) (`--max-stdin-bytes` override untested)

**Route:** `Phase 2a` (new tests)
- New test exercising within-payload byte-equal dedup (closes R8 F1).
- New test exercising tag-element `display_safe` with pathological tag content (closes R8 F2); converges with Cluster B regression test (Phase 2a) since the new display_safe shape applies to tags.
- New test exercising `--max-stdin-bytes` operator override (closes R8 F3).

**Owning artifact:** `tests/bookmarks.rs`.

**Gate:** 3 new tests pass; mutation-testing-equivalent coverage of the spec sub-cases; Validator: QE.

**Sequencing:** Should land before Layer 3 layer-gate close.

---

### Cluster E — UX/help/error remediation

**Findings routed:** [UX R1 F2](2026-05-24-ux.md#r1-f2) (size-cap remediation hint), [TW R1 F4](2026-05-24-technical-writer.md#r1-f4) (clap docs `bookmark(s)` notation), [SE R1 F4](2026-05-24-software-engineer.md#r1-f4) (size-cap check-before-empty-stdin ordering)

**Route:** `Phase 2b` (impl changes)
- **UX F2:** `src/main.rs` `run_import` size-cap error message — add `Hint: use --max-stdin-bytes <N> to override the default 10 MB cap.` line per the R1 F5 hint precedent. Use human-readable unit suffix for the byte count (e.g., `10485760 bytes (10 MB)`).
- **TW F4:** `src/main.rs` `Cli` doc comments — replace `bookmark(s)` notation with the spec singular/plural contract (singular at N=1; plural otherwise).
- **SE F4:** `src/main.rs` `run_import` validation order — move empty-stdin check BEFORE size-cap check; add lower-bound validation on `--max-stdin-bytes` (reject 0 with `Error: --max-stdin-bytes must be at least 1.`).

**Owning artifact:** `src/main.rs`.

**Gate:** Updated error message; clap docs use singular/plural; new validation order + bound check; Validator: UX + SE.

**Sequencing:** Should land before Layer 3 layer-gate close.

---

### Cluster F — DESIGN.md § Verification architecture stale (SA F2)

**Findings routed:** [SA R5 F2](2026-05-24-solution-architect.md)

**Route:** `Phase 1a+1b` (spec amendment)
- DESIGN.md § Verification architecture — extend the pure-fn enumeration to name `export_json` + `import_json` + the dedup-on-exact-tuple-match logic; cross-source consistency check against `src/lib.rs` module-doc.

**Owning artifact:** `DESIGN.md`.

**Gate:** Pure-fn list extends to Layer 3; module-doc + spec narrative aligned; Validator: SA.

**Sequencing:** Should land before Layer 3 layer-gate close.

---

### Cluster J — SA F4 dedup O(N×M) complexity

**Findings routed:** [SA R5 F4](2026-05-24-solution-architect.md) — converges with [PE R7 F1](2026-05-24-performance-engineer.md) Accepted-limitation

**Route:** `Phase 1a+1b` (accepted-limit annotation)
- DESIGN.md § Performance budget — add paragraph naming the dedup-via-contains O(N×M) complexity at the 10K scale ceiling; accepted-limit framing matching the Layer 1 cumulative-add-cost precedent.

**Owning artifact:** `DESIGN.md` § Performance budget.

**Gate:** Accepted-limit paragraph lands; Validator: PE + SA.

**Sequencing:** Should land before Layer 3 layer-gate close.

---

### Cluster K — SA F5 ImportError variant detail

**Findings routed:** [SA R5 F5](2026-05-24-solution-architect.md)

**Route:** `Phase 2b` (low priority; deferred)
- `src/lib.rs` `ImportError::SchemaMismatch` — extend variant to optionally carry `record_index: Option<usize>` for per-record schema-mismatch findings. Low priority; can defer to follow-up PR.

**Owning artifact:** `src/lib.rs`.

**Gate:** ImportError variant extended; Validator: SE.

**Sequencing:** Low priority. **Deferred-to-follow-up-PR** per the G-150 over-investment guard; not required for Layer 3 gate close.

---

### Cluster L — SE F3 import_json doc-comment misclaim

**Findings routed:** [SE R1 F3](2026-05-24-software-engineer.md#r1-f3)

**Route:** `Phase 2b` (small impl fix)
- `src/lib.rs` `import_json` doc comment — remove the `tests/properties.rs` round-trip property claim OR add the property at Phase 5 + retain claim. Conservative-default: remove the claim now; re-add when Phase 5 lands the proptest property.

**Owning artifact:** `src/lib.rs` import_json doc comment.

**Gate:** Doc comment matches reality; Validator: QE.

**Sequencing:** Should land before Layer 3 layer-gate close.

---

### Cluster M — AIE F6/F7 process discipline

**Findings routed:** [AIE R1 F6](2026-05-24-ai-engineer.md#r1-f6) (pre-cycle methodology declaration absent), [AIE R1 F7](2026-05-24-ai-engineer.md#r1-f7) (per-commit cost-tally gap carry-forward)

**Route:** `Phase 4 itself` (process discipline)
- F6: Round 2 launch includes the pre-cycle methodology declaration per primer 3 § Pre-cycle methodology check Path 2.
- F7: Carry-forward across PRs; document operator-time commitment in TODO.md.

**Owning artifact:** Round 2 IAR launch prompt + TODO.md tracking.

**Gate:** Round 2 declaration lands; Validator: AIE.

**Sequencing:** Folded into Round 2 work; does not block Layer 3 gate close (process improvement, not defect).

---

### Cluster N — PFE F2 cargo-fuzz harness not yet authored

**Findings routed:** [PFE R7 F2](2026-05-24-platform-engineer.md#r7-f2)

**Route:** `Phase 5` (already scheduled per DESIGN.md § Project intent § Phase 5 strategy Layer 3)

**Owning artifact:** `fuzz/fuzz_targets/import_stdin.rs` (new file at Phase 5).

**Gate:** Phase 5 harness lands per DESIGN.md Phase 5 strategy; not Round 1 fix work.

**Sequencing:** Layer 3 layer-gate close DOES require Phase 5 dispositions per criterion 5; this finding is tracking-only at Round 1 routing.

---

### Cluster O — Numbering inconsistency (operator-flagged at collection commit)

**Source:** Round 1 collection commit `2acc418` flagged QE used Review 8 + PFE used Review 7 (continuing per-project per-domain sequence) while the other 11 agents used "Review 1" (per-dated-file fresh sequence).

**Route:** `Suite-development` (not a project phase per primer 4 routing table line 38)
- Folds into the deferred suite-hardening discussion captured in the main session about Phase 4 bypass prevention + phase-frequency clarification + numbering convention disambiguation. Defers entirely until after PR #52 merges.

**Sequencing:** Does not block Layer 3 layer-gate close; suite-side work, not project-side.

---

## Cross-cluster sequencing matrix

| Phase | Work to land before Round 2 |
|---|---|
| **Phase 1a+1b** (spec amendments) | Cluster B spec amendment; Cluster H spec amendment (Threat model + bm import failure); Cluster I storage data classification; Cluster A2 README; Cluster A3 CHANGELOG; Cluster A4 PROCESS.md retrospective; Cluster A5 FINDINGS-INDEX; Cluster A7 install-verification; Cluster F Verification architecture; Cluster J Performance budget accepted-limit; numbering convention deferred to suite |
| **Phase 2a** (new failing tests; canonical two-commit pattern) | Cluster B regression test; Cluster C regression test; Cluster H regression test; Cluster D 3 new tests; manual-tests/layer-3.md authoring (artifact-equivalent) |
| **Phase 2b** (implementations after Phase 2a tests RED) | Cluster B display_safe rewrite; Cluster C sorted-tag-comparison; Cluster H control-char rejection + new ImportError variant; Cluster A6 long_about; Cluster E error message + clap docs + validation order; Cluster L doc-comment fix |
| **Phase 2c** (refactor if needed) | TBD at Phase 2b landing; explicit-skip annotation otherwise |
| **Phase 5** | Cluster N cargo-fuzz harness (already scheduled) |
| **Deferred-to-follow-up-PR** | Cluster K ImportError variant detail |

**Two-commit-canonical-pattern reminder** per Layer 2 Red Gate evidence-preservation annotation: Phase 2a (failing tests RED) lands as standalone commit; Phase 2b (implementation GREEN) lands as second commit; the canonical 3-commit shape (2a + 2b + 2c-annotation) per Round 1 SE F2's "tests fail for the right reason" discipline.

## Round 2 trigger

Per [`primers/3-review-session.md`](../../../vsdd-suite/primers/3-review-session.md) § Round triggers (G-131 continue + G-151 stop): Round 1 has substantive routable findings across 13 domains; Round 2 is mandatory after Phase 4 fix work lands. Round 2 re-runs the 13-domain capstone-active set against the post-fix state to verify Round 1 fixes hold + surface any residuals.

**Round 2 launch will include:**
- Pre-cycle methodology declaration per [AIE R1 F6](2026-05-24-ai-engineer.md#r1-f6) routing
- Updated DESIGN.md + TODO.md + tests + impl reflecting Round 1 fix work
- Each agent reads the Round 1 routing record (this file) as part of cold-session context so they know what was deliberately addressed

## Layer 3 layer-gate close criteria status post-Round 1 routing

| # | Criterion | Status post-Round 1 routing |
|---|---|---|
| 1 | All Red Gate tests pass | NOT MET — Cluster B + C + D + H regression tests pending |
| 2 | `cargo build --release` clean | NOT MET — Cluster B + C + H impl changes pending |
| 3 | `manual-tests/layer-3.md` runs clean | NOT MET — file absent; Cluster A1 authoring pending |
| 4 | Phase 3 IAR 13-domain MVR | NOT MET — Round 2 required (Round 1 has substantive routable findings) |
| 5 | Phase 5 dispositions (proptest + cargo-fuzz + mutation + purity boundary) | NOT MET — Phase 5 cycle queued after Round 2 MVR |
| 6 | Phase 6 NA | MET per DESIGN.md § Phase 6 strategy Layer 3 (capstone gates at project-terminal MVR per primer 6, not per-layer) |

**Phase 4 routing pass 1 complete; fix work follows.**

---

## Phase 4 reflection (per primer 4 § Anti-patterns)

The primer 4 primary failure mode is "Routing every finding to Phase 2b". Audit against this round's routing:

| Phase | Finding count routed | Rationale |
|---|---|---|
| Phase 1a+1b (spec/narrative) | ~13 findings (most of Cluster A + B + F + H + I + J spec sides) | Substantive spec gaps (display_safe semantic; tag-injection threat model; verification arch; storage data classification; accepted-limit annotation) + Layer 3 narrative-staleness across README/CHANGELOG/PROCESS/FINDINGS-INDEX |
| Phase 1c | 0 | No decomposition gaps surfaced; Layer 3 acceptance criteria covered the surface adequately |
| Phase 2a (new tests) | ~7 findings (Cluster B + C + D + H regression tests + 3 QE coverage tests) | Test discipline gaps surfaced by QE + the Cluster B/C/H regression tests required for the impl changes |
| Phase 2b (implementation) | ~8 findings (Cluster B + C + H impl + Cluster A6 + Cluster E + Cluster K + Cluster L) | True implementation defects (display_safe escape format; sorted-tag-dedup; control-char rejection; clap text; error remediation; ordering) |
| Phase 2c | 0 | No refactor regressions surfaced |
| Phase 5 | 1 finding (Cluster N tracking-only) | Already scheduled per DESIGN.md Phase 5 strategy |
| Phase 4 itself | 2 findings (AIE F6 + F7) | Process discipline carried forward to Round 2 launch |
| Suite-development | 1 finding (Cluster O numbering convention) | Suite-side discoverability gap; not a project phase |
| Terminal-no-route | ~5 findings (Accepted-risk + Accepted-limitation; RT F2 + Sec F4 + Sec F5 + PE F1 + SA F4 partial) | Already terminal; classification suffices |

Phase 2b count is moderate, not dominant. Cluster A (docs staleness) routed primarily to Phase 1a+1b not Phase 2b — confirming the "what artifact would have prevented this" trace per primer 4 § Driving questions (the right artifact was the README/CHANGELOG/PROCESS narrative, not the implementation). Cluster B + C + H multi-phase chains correctly route across 1a+1b + 2a + 2b — recording the chain shape per primer 4 § Multi-phase findings is the correct discipline.

**Routing pass complete.** Fix work begins.
