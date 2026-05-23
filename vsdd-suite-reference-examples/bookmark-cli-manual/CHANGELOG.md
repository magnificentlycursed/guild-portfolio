# Changelog

## [Unreleased] Layer 2 tag + filter — Phase 2 + Phase 3 Round 1 fix cycle — 2026-05-22

**Scope:** Layer 2 (`bm tag` + `bm list --tag`) Phase 2 implementation + Phase 3 IAR Round 1 4-cluster parallel cold-session review + Round 1 inline fix cycle on the `bookmark-cli-manual-layer-2` branch.

### Added — Layer 2 surface

- **`bm tag <url> <label>`** command: attaches a label to every bookmark whose URL matches exactly (case-sensitive); idempotent — a label already present is not re-appended. Multi-match deliberate (URL-as-identifier per [DESIGN.md § `bm tag` § Multi-match semantics](DESIGN.md#bm-tag-url-label-layer-2)); tags-all-matches in one atomic save.
- **`bm list --tag <label>`** filter: returns the subset of bookmarks whose `tags` field contains the supplied label, in newest-first order. Repeated `--tag` composes with OR-semantics across labels per [DESIGN.md § `bm list --tag <label>` § Why OR-semantics for repeated `--tag`](DESIGN.md#bm-list---tag-label-layer-2).
- **`Bookmark.tags: Vec<String>`** field — optional during deserialization (Layer-1-format files default to empty), always present during serialization (forward-only migration shape per [DESIGN.md § Storage format `tags` field](DESIGN.md#storage-format-json-file)).
- **`BookmarkStore::attach_tag(&str, &str) -> Result<usize, AttachTagError>`** — pure transformation against the store; returns the count of matching bookmarks; idempotent per the spec.
- **`BookmarkStore::filter_by_tags(&[&str]) -> Vec<&Bookmark>`** — pure OR-filter against the store; returns newest-first.
- **`AttachTagError`** enum with `EmptyUrl` / `EmptyLabel` / `NoMatch` variants — mirrors the [DESIGN.md § `bm tag` failure contract](DESIGN.md#bm-tag-url-label-layer-2); the CLI shell maps each variant to its spec-contracted stderr.
- **Parent-directory `fsync(2)` on Unix** after the atomic-save `rename(2)` — closes the operator-queued Performance Engineer fsync benchmark item from Layer 1 Round 2. Gated `#[cfg(unix)]`; documented in [DESIGN.md § Performance budget § Durability discipline (Layer 2)](DESIGN.md#performance-budget-review-82-round-2-fix-for-performance-engineer-review-1-finding-1).
- **`manual-tests/layer-2.md`** — 13-step per-layer manual-test plan parallel to `manual-tests/layer-1.md`. Includes the `hyperfine` performance sanity-check at Step 12 (closes Layer-1-Deferred [Performance Engineer Review 1 Finding 2](vsdd-suite/review-log/2026-05-20-performance-engineer.md)).
- **`tests/scaling.rs`** — three `#[ignore]`-gated sentinel tests at the 100 / 1,000 / 10,000-bookmark cliffs; closes Layer-1-Deferred [Performance Engineer Review 1 Finding 5](vsdd-suite/review-log/2026-05-20-performance-engineer.md) at the in-CI surface. Runs via `cargo test -- --ignored` in a separate Linux-only CI job.
- **`tests/properties.rs`** — `proptest` activation against the tag-idempotence + filter-OR-monotonicity properties on the pure `BookmarkStore` API (closes Layer 2 Round 1 VDD-IAR Alignment R4 F5 + Solution Owner R4 F2 — the DESIGN.md claim of `proptest` activation was load-bearing).
- **Layer 2 acceptance criteria AC 5–13** at [TODO.md § Layer 2](TODO.md#layer-2--tag-and-filter) — 9 new ACs covering tag + filter happy paths, error paths, idempotence, multi-match, OR-semantics, store-empty-vs-filter-empty precedence, forward-only migration, and durability.

### Changed

- **`Cargo.toml`** `rust-version` 1.78 → 1.81 (Layer 2 Round 1 PE F4 — Layer 1 R3's `reason = "..."` attribute syntax in the `#[allow(...)]` blocks at `src/lib.rs` + `tests/bookmarks.rs` requires Rust 1.81+; the declared 1.78 MSRV was inaccurate).
- **`Cargo.toml`** `[dev-dependencies]` — added `proptest = "1"` per Phase 5 Layer 2 strategy + Round 1 inline fix Fix 7.
- **`DESIGN.md` § Storage data classification** — added paragraph explicitly classifying the `tags` field as confidential-class data alongside URLs; added "Downgrade-compatibility hazard." paragraph naming the asymmetric `serde` shape (Layer 2 binaries read Layer 1 files via `#[serde(default)]` but Layer 1 binaries reading Layer 2 files discard `tags` on next save — documented as a deliberate forward-only-migration choice). Closes Layer 2 Round 1 Security F1 + Solution Architect F5.
- **`DESIGN.md` § Threat model** — added "Tag-injection-as-trust-signal" paragraph (Layer 2 Round 1 Red Team F6) naming the attack class where an adversary with write access to the store file fabricates tags like `["verified", "approved"]` to mislead the user; documented as accepted risk under the same mode-0600 + symlink-rejection mitigations that apply to URL-injection.
- **`DESIGN.md` § Project intent's Phase 6 strategy for Layer 2** — adopted Option 1 (mark as not-applicable) per [G-150](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-150) over-investment guard + [G-112](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112) reference-implementation-purpose-already-satisfied. Layer 1's Phase 6 attestation at VDD-IAR Alignment Review 3 stands as the project's terminal four-dimensional convergence record; re-running Phase 6 for Layer 2 would teach methodology consumers that capstone artifacts require per-layer Phase 6, which is not the suite's intent (capstone gates at project-terminal MVR per primer 6). Closes Layer 2 Round 1 VDD-IAR Alignment R4 F5 + Solution Owner R4 F2 (Option 1 the cluster's own SO recommended).
- **`DESIGN.md` § Phase 5 strategy for Layer 2** — kept the `proptest` activation declaration (Fix 7a chosen over Fix 7b).
- **`TODO.md` § Layer 2 Layer-gate criterion #6** — annotated as not-applicable cross-linking the DESIGN.md Phase 6 strategy declaration.
- **`TODO.md` § Layer 2 Phase 2c** — added evidence-preservation annotation naming the single-commit Phase 2a + Phase 2b shape as a deliberate trade-off (Layer 2 Round 1 VDD-IAR Alignment R4 F1); future Layer cycles should use the canonical two-commit shape so Red Gate failure evidence lives in git history rather than only in sub-agent spawn output.
- **`README.md`** — Layer-1-only narrative promoted to Layer 2; phase progression table extended to cover Layer 2 (Phases 1a / 1b / 2a / 2b / 2c / 3 / 5 / 6); test counts updated (43 default tests post-Round-1 fix cycle); added pointers to TODO.md § Layer 2 + manual-tests/layer-2.md + DESIGN.md § Behavioral contracts Layer 2 surface. Closes Layer 2 Round 1 Technical Writer F1 + Documentation Reviewer F1.
- **`src/main.rs`** — `Cmd::Tag` + `Cmd::List { tags }` clap surface added; `run_tag` emits `Tagged N bookmark(s).` to stderr on success per Layer 2 Round 1 UX F2 + SE F2 (silent-on-success leaves the multi-match semantic undiscoverable). Help-text doc comments for `Cmd::Tag` + `Cmd::List` expanded with the semantic rules (idempotence, OR-semantics, empty-state precedence) per Layer 2 Round 1 UX F1 + F3.
- **`src/lib.rs`** — `Bookmark.tags` field; `AttachTagError` enum; `BookmarkStore::attach_tag` + `BookmarkStore::filter_by_tags`; `fsync_directory` helper; updated module docstring naming the Layer 2 purity boundary extensions.
- **`manual-tests/install-verification.md`** — appended Layer 2 inheritance note (Layer 2 inherits Layer 1's Nathan PR #41 install-verification PASS row per [G-155](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-155) strict reading; operator should solicit a new Layer 2 install verification in the post-merge feedback cycle).
- **`manual-tests/layer-2.md` Steps 2 / 3 / 7** — updated expected stderr from silent-on-success to the new `Tagged N bookmark(s).` line; companion fix to the `src/main.rs` change above.

### Closes Layer-1-Deferred items at Layer 2

- **[Performance Engineer Review 1 Finding 2](vsdd-suite/review-log/2026-05-20-performance-engineer.md)** (benchmarking infrastructure) → `hyperfine` sanity-check at [`manual-tests/layer-2.md` Step 12](manual-tests/layer-2.md).
- **[Performance Engineer Review 1 Finding 5](vsdd-suite/review-log/2026-05-20-performance-engineer.md)** (data-scaling tests) → [`tests/scaling.rs`](tests/scaling.rs) sentinels.
- **Quality Engineer Layer-1-Deferred RFC 3339 scripted check** → test 13 `tests_list_rfc3339_scripted_check` in [`tests/bookmarks.rs`](tests/bookmarks.rs).
- **Operator-queued fsync benchmark item** → parent-directory `fsync(2)` in `BookmarkStore::save` on Unix; documented in [DESIGN.md § Performance budget § Durability discipline (Layer 2)](DESIGN.md#performance-budget-review-82-round-2-fix-for-performance-engineer-review-1-finding-1).

### Round 1 cluster cold-session review

The 4 cold-session clusters (SE/UX/Performance-Engineer; QE/Security/Technical-Writer; Solution-Architect/Red-Team/Platform-Engineer; Solution-Owner/Documentation-Reviewer/AI-Engineer/VDD-IAR-Alignment) surfaced ~30 findings across the 13 capstone-active domains. The Round 1 inline fix cycle on this branch applied 12 numbered fixes (Fix 1 through Fix 13 in the fix-cycle prompt with Fix 11 deferred to Round 2), resolving ~17 cross-domain finding-closures; the residual subset routes to Round 2 verification or carryforward.

---

## v0.12.3 Phase 6 four-dimensional convergence ATTESTED + UX/TW/QE cluster fix-cycle from @shimmermathlabs.com install-verification thread — 2026-05-21 13:30Z ([Review 88](../../vsdd-suite/suite-development/review-log/2026-05-21-suite-review.md#review-88--2026-05-21-1330z))

**Scope:** PR [#42](https://github.com/magnificentlycursed/guild-portfolio/pull/42) — Phase 6 four-dimensional convergence attestation (project-terminal at Layer 1) following [PR #41](https://github.com/magnificentlycursed/guild-portfolio/pull/41)'s closure of the Platform Engineer Dim 38 / [G-155](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-155) install-verification gate. The reference-example purpose ([G-112](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-112)) is satisfied: all 6 VSDD phases demonstrated end-to-end.

### Attested — Phase 6 four-dimensional convergence (project-terminal)

[VDD-IAR Alignment Review 3 (project-terminal)](vsdd-suite/review-log/2026-05-20-vdd-iar-alignment.md) — all 4 dimensions attest:

| Dimension | Status | Evidence |
|---|---|---|
| Spec MVR | ATTESTED | DESIGN.md round closure across PR #38 R2 + PR #40 + PR #42 cycles; no Open spec findings |
| Test MVR | ATTESTED at Layer-1 scope | QE Reviews 1/2/3; Phase 5 Mutation Testing 8/8 viable kill rate; 1 Deferred-to-Layer-2 RFC 3339 check does not block Layer-1 |
| Implementation MVR | ATTESTED | 10 of 10 active capstone-tier role-domains at MVR at Layer-1 scope |
| Formal-verification MVR | ATTESTED | Purity Boundary Audit + Mutation Testing closure; property-based testing / Fuzz Testing / Proof Execution declared deferred-or-not-applicable with rationale |
| Cross-dimension consistency check | ATTESTED | No contradictions across DESIGN.md / src/lib.rs / tests/ / per-domain review-logs |

### Changed — UX + Technical Writer + Quality Engineer cluster cold-session fix-cycle

Per operator's Bluesky Post-9 commitment in [@shimmermathlabs.com](https://bsky.app/profile/shimmermathlabs.com)'s install-verification thread, a 3-domain cluster cold-session was spawned against the post-PR-#40 state. 9 findings filed (8 Resolved inline + 1 Deferred-to-Layer-2). Full narrative in the per-session review-log files:

- [UX Review 4](vsdd-suite/review-log/2026-05-21-ux.md) — F1 "literal — empty" wording rewrite at `manual-tests/layer-1.md`; F2 "Sycophancy-compensation reminder" leak deletion; F3 silent-on-success affordance routed to SO.
- [Technical Writer Review 4](vsdd-suite/review-log/2026-05-21-technical-writer.md) — F1 `manual-tests/install-verification.md` file-inventory completion (closes Nathan Post-6 "more files than are mentioned in the doc"); F2 Sycophancy-leak deletion (companion fix); F3 README Phase 3 row rewrite to reference the post-PR-#40 canonical artifacts.
- [Quality Engineer Review 3](vsdd-suite/review-log/2026-05-21-quality-engineer.md) — F1 expected-output wording cross-fix; F2 file-inventory closure; F3 RFC 3339 scripted-check Deferred-to-Layer-2 per primer 1c § scripted-vs-human-split discipline.

### Project files changed

- **[`manual-tests/layer-1.md`](manual-tests/layer-1.md)** — Step 1 + adjacent steps with the same wording shape rewritten: `(literal — empty)` parenthetical → explicit `Expected stdout — none (silent on success; the fenced block below is intentionally empty):` form, naming both the silent-on-success affordance AND the intentionally-empty fenced block as a unit. "Sycophancy-compensation reminder" line at the document's tail deleted.
- **[`manual-tests/install-verification.md`](manual-tests/install-verification.md)** — file-inventory section rewritten with full repo file enumeration + explanatory annotations; `TW Dim 11` bare reference rewritten in plain language per new `check-suite-internal-terminology.py` hook discipline.
- **[`README.md`](README.md)** — Phase 3 row in the Phase progression table rewritten to reference per-session review-log files canonical post-PR-#40 (per-domain index files retired in PR #40); 7-of-12 → 10-of-10-at-MVR scorecard update.
- **[`DESIGN.md`](DESIGN.md)** — 2 bare `G-NNN` references rewritten as markdown links per new hook; 1 "cold-session discipline" suite-internal-vocabulary instance rewritten in plain language.

### Upstream-suite recurrence-prevention applications (per [Review 88 Finding 3](../../vsdd-suite/suite-development/review-log/2026-05-21-suite-review.md#review-88--2026-05-21-1330z))

The 3 in-thread findings Nathan surfaced were each routed to upstream-suite recurrence-prevention so future install-verification cycles on other projects don't repeat the same defects:

1. **"literal — empty" wording confusion** → primer 1c § Manual testing checklist § Empty-output wording discipline (new sub-section with worked-example pattern).
2. **File-inventory under-enumeration** → install-verification template file-inventory section (partial; template work routed forward).
3. **"Sycophancy-compensation reminder" suite-internal-vocabulary leak** → new `vsdd-suite/hooks/check-suite-internal-terminology.py` hook wired in pre-commit; scans user-facing project artifacts for suite-internal AI-agent-discipline vocabulary.

Additionally:
4. **Scripted-vs-human-split discipline** → primer 1c § Manual testing checklist § Scripted-vs-human-split discipline (new sub-section codifying the split between manual-test-plan assertions vs automated-test-surface assertions).

### MVR scorecard post-PR-#42 (project-terminal at Layer 1)

| Domain | Status |
|---|---|
| Software Engineer | MVR (R3) |
| Quality Engineer | MVR (R3 + Phase 5 Mutation Testing closure) |
| UX | MVR (R4 — NEW closure via Nathan-thread fix-cycle) |
| Security | MVR (R3) |
| Solution Architect | MVR (R1 + Phase 5 Purity Boundary Audit closure) |
| Solution Owner | MVR (R3) |
| Performance Engineer | MVR at Layer-1 scope (Layer-2 fsync benchmark deferred) |
| Platform Engineer | **MVR — NEW** (Dim 38 gate closed by PR #41) |
| Red Team | MVR (R3) |
| Technical Writer | MVR (R4 — NEW closure via Nathan-thread fix-cycle) |
| Documentation Reviewer | MVR (R4 — closed in PR #40) |
| AI Engineer | MVR (R1 + R2 F6/F7/F8 closure via PR #40) |
| VDD-IAR Alignment | MVR (R3 — project-terminal Phase 6 attestation) |

**10 of 10 active capstone-tier role-domains at MVR + the meta + 2 Phase-5-active SA/QE rounds = 13 active per [DESIGN.md § Project intent](DESIGN.md).** The reference-example IS the worked example of all 6 VSDD phases end-to-end.

### Acknowledgements

- [@shimmermathlabs.com](https://bsky.app/profile/shimmermathlabs.com) for the install-verification PASS row in [PR #41](https://github.com/magnificentlycursed/guild-portfolio/pull/41) + the 3 in-thread findings that motivated the fix-cycle + upstream-suite recurrence-prevention applications. The Bluesky-thread archive lives at [`../../vsdd-suite/suite-development/review-log/external-review-log/2026-05-21-shimmermathlabs.md`](../../vsdd-suite/suite-development/review-log/external-review-log/2026-05-21-shimmermathlabs.md).

---

## v0.12.2 PROCESS.md three-audience-lens optimization — 2026-05-21 12:00Z ([Review 86](../../vsdd-suite/suite-development/review-log/2026-05-21-suite-review.md#review-86--2026-05-21-1200z) Finding 4)

**Scope:** PR [#40](https://github.com/magnificentlycursed/guild-portfolio/pull/40) — operator-directed evaluation of the AI-authored PROCESS.md against review-log evidence + three-audience-lens optimization. The 3 AI-author "what was hardest" claims verified against review-log evidence (all 3 PROVEN OUT); existing stumbling points gained three-audience treatment; 3 new post-PR-#38/#39/#40 stumbling points added.

### Changed — PROCESS.md

- **Top-of-file three-audience-lens preamble** added explaining the three-audience optimization (suite developers / suite users / AI agents) per [Review 86](../../vsdd-suite/suite-development/review-log/2026-05-21-suite-review.md#review-86--2026-05-21-1200z) Finding 4 + cross-referencing the [Three-audience design principle](../../vsdd-suite/suite-development/suite-development.md#three-audience-design-principle-review-80-finding-3). The format applies to capstone+ intent PROCESS.md content as a discipline.
- **Existing 3 Layer-1 stumbling points** each gained a `**Three-audience lens:**` paragraph naming what each audience should take from the stumbling point + a `**Review-log evidence:**` line citing the specific Finding F + Review N that proves the claim.
- **3 NEW post-PR-#38/#39/#40 stumbling points added:**
  - **Stumbling point 4** — 80 Round-1 findings as spec/test under-investment signal disguised as IAR thoroughness (Review 82 Finding 2 evidence).
  - **Stumbling point 5** — Operator-directive correction cost: 3 mid-cycle slips in PR #38 (Round 2 filename violation; wrong adversarial-pair clustering; cluster-letter naming) named in the feedback memory + AI Engineer R1 F4 evidence.
  - **Stumbling point 6** — Site-specific fix declared closure: Doc Reviewer R3 carryforward pattern that motivated the [`grep -rn before claiming closure`](../../vsdd-suite/primers/4-feedback-integration.md) discipline authored in Review 84 Finding 1.

### Verified — AI-author claims against review-log evidence (all 3 PROVEN OUT)

| Claim | Review-log evidence | Verdict |
|---|---|---|
| "Phase 5 Purity Boundary Audit was hardest" | F-004 — [SA Review 1 Finding 1](vsdd-suite/review-log/2026-05-20-solution-architect.md) (3-way divergence between src/lib.rs:1-7 module doc + DESIGN.md silence + 3-of-4 effectful methods; Resolved by DESIGN.md rewrite) | PROVEN OUT |
| "Phase 2a Red Gate framing was wrong" | F-001 — [QE Review 1 Finding 1](vsdd-suite/review-log/2026-05-17-quality-engineer.md) ("Phase 2a → 2b commit boundary not enforced"; Resolved by post-hoc documentation) | PROVEN OUT |
| "Mutation Testing + Purity Boundary Audit produced genuine signal vs ceremony" | F-005 — [QE Review 2 Finding 1](vsdd-suite/review-log/2026-05-20-quality-engineer.md) (7/8 pre-fix kill rate; missing falsifying test for save-to-nested-path; Resolved); F-004 again; AI Engineer R1 ~21k tokens/finding (below capstone band floor of 100k — efficient discipline zone) | PROVEN OUT |

---

## v0.12.1 Per-domain index retirement + Cold-session budget declaration + Doc Reviewer Round 4 closure — 2026-05-21 11:00Z ([Review 84](../../vsdd-suite/suite-development/review-log/2026-05-21-suite-review.md#review-84--2026-05-21-1100z))

**Scope:** PR [#40](https://github.com/magnificentlycursed/guild-portfolio/pull/40) — upstream-suite remediation cycle applied at the reference example. Per-domain index files retired (the reference-example demonstrates the new methodology shape after the operator's redundancy evaluation); new Cold-session budget declaration in DESIGN.md (per the new methodology requirement); Doc Reviewer Round 4 verification closes the 5 R3 carryforwards (Doc Reviewer reaches MVR).

### Removed — per-domain index navigation surface

- **13 per-domain index files deleted** at [`vsdd-suite/<DOMAIN>-REVIEW.md`](vsdd-suite/) (12 role + 1 meta). The reference example now navigates exclusively via `review-log/` + `FINDINGS-INDEX.md`.

### Changed — spec ([`DESIGN.md`](DESIGN.md))

- **§ Project intent — new Cold-session budget declaration line** below the Phase 6 strategy declaration. Required at capstone + production intent per the new methodology fix in [`../../vsdd-suite/domains/DOMAIN-INDEX.md`](../../vsdd-suite/domains/DOMAIN-INDEX.md) § Cold-session budget per intent. Names: max 4 rounds; max 10 parallel agents or 4-cluster batched; 100k-300k tokens per substantive finding band; Opus 4.7 for SE/Security/Red Team/SA/SO/VDD-IAR Alignment/AI Engineer; Sonnet 4.6 for UX/PE/Platform/TW/Doc Reviewer/QE; Haiku 4.5 for mechanical-sweep delegated sub-agents.

### Changed — reference rewrites for per-domain index retirement

- **[`README.md`](README.md)** + **[`src/lib.rs`](src/lib.rs)** + **[`manual-tests/install-verification.md`](manual-tests/install-verification.md)** + **[`vsdd-suite/FINDINGS-INDEX.md`](vsdd-suite/FINDINGS-INDEX.md)** + **[`DESIGN.md`](DESIGN.md)** — ~7 reference rewrites total replacing per-domain index citations with the canonical replacement target (specific per-session review-log file; FINDINGS-INDEX for cross-finding queries; suite-side domain prompt for methodology cites).

### Changed — review-log discipline

- **[`vsdd-suite/review-log/2026-05-20-documentation-reviewer.md`](vsdd-suite/review-log/2026-05-20-documentation-reviewer.md)** — appended `## Review 4 — 2026-05-21 11:00Z` section. Phase-4-routed verification of the 5 Round 3 Deferred carryforwards: 3 Resolved by per-domain-index retirement's elimination; 2 Resolved by grep-clean evidence applying the newly-codified `grep -rn before claiming closure` discipline. **Doc Reviewer MVR reached.**

### Per-domain MVR scorecard promotes from 7 of 10 → 9 of 10 at MVR

| Domain | Status after PR #40 |
|---|---|
| Software Engineer | MVR reached (PR #38 Round 3) |
| Performance Engineer | MVR-blocked-by-Deferred (R2-F7 fsync benchmark — Layer 2) |
| Platform Engineer | MVR-blocked-by-operator-gate (R2-F9 install-verification) |
| Security | MVR reached (PR #38 Round 3) |
| UX | MVR reached (PR #38 Round 3 inline-fix) |
| Red Team | MVR reached (PR #38 Round 3 inline-fix + Accepted risk) |
| Technical Writer | MVR reached (PR #38 Round 3 inline-fix) |
| **Documentation Reviewer** | **MVR reached (PR #40 Round 4 verification — NEW)** |
| Solution Owner | MVR reached (PR #38 Round 3) |
| VDD-IAR Alignment | MVR reached (PR #38 Round 2) |
| **AI Engineer** | **MVR reached (PR #39 R1 + PR #40 closes F6/F7/F8 — NEW)** |

### Phase 6 status

**Phase 6 four-dimensional convergence record continues DEFERRED.** Platform Engineer install-verification operator-gate is the AI-unsatisfiable hard ceiling per [G-155](../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-155).

---

## v0.12.0 AI Engineer domain registration + first cold-session round — 2026-05-21 10:00Z ([Review 83](../../vsdd-suite/suite-development/review-log/2026-05-21-suite-review.md#review-83--2026-05-21-1000z))

**Scope:** Reference example's capstone-active-domain set extended with the newly-registered [AI Engineer](../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md) role-domain (cost-and-quality discipline for AI-agent usage in the IAR cycle). First cold-session Round 1 filed against PR [#38](https://github.com/magnificentlycursed/guild-portfolio/pull/38)'s 3-round cycle as the new domain's first audit.

### Added — review-log discipline

- **AI Engineer domain activation** — Active domain promotes from 11 role + 1 meta = 12 → **12 role + 1 meta = 13** at capstone intent. Round 1 review filed at [`vsdd-suite/review-log/2026-05-21-ai-engineer.md`](vsdd-suite/review-log/2026-05-21-ai-engineer.md).
- **[`vsdd-suite/review-log/2026-05-21-ai-engineer.md`](vsdd-suite/review-log/2026-05-21-ai-engineer.md)** — Round 1 cold-session review against PR #38's R1 + R2 + R3 cycle. **10 findings filed: 5 Resolved + 3 Deferred + 1 Dismissed + 1 Hallucinated**.

### Changed — spec ([`DESIGN.md`](DESIGN.md))

- **§ Project intent — Active domain set** — `5 extended` → `6 extended`; `11 role + 1 meta = 12 active domains` → `12 role + 1 meta = 13 active domains` (AI Engineer added with the rationale that PR [#38](https://github.com/magnificentlycursed/guild-portfolio/pull/38)'s 3-round cycle burned ~3-4M tokens + hit a daily rate-limit mid-cycle; capstone intent + sustained multi-round cycles compound cost; AI Engineer is the discipline that keeps the gold-standard achievable at scale).

### Changed — docs

- **[`TODO.md`](TODO.md)** — `12 active domains` → `13 active domains`; capstone-tier extended-set list extended with [AI Engineer](../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md).
- **[`vsdd-suite/FINDINGS-INDEX.md`](vsdd-suite/FINDINGS-INDEX.md)** — Cross-references preamble updated (`11 active role-domain indexes` → `12 active role-domain indexes (including AI Engineer)`); 10 new rows (F-028 through F-037) for the AI Engineer Round 1 findings.

### Per-domain Round 1 outcomes ([Review 1](vsdd-suite/review-log/2026-05-21-ai-engineer.md#review-1--2026-05-21-1000z))

| Finding | Dim | Classification | Summary |
|---|---|---|---|
| F1 | 7 | Resolved | Cluster-batching with adversarial-pair separation as Round 3 spawn shape is operative discipline; codified for Round 3+ at capstone scale |
| F2 | 8 | Resolved | Phase 4 routing as Round-2+ scope-reducer is operative (R2 + R3 prompts routed prior-round findings, didn't re-scan) |
| F3 | 11 | Resolved | Audit-trail machine-readability passes [Review 80](../../vsdd-suite/suite-development/review-log/2026-05-20-suite-review.md#review-80--2026-05-20-1830z) Finding 3 Agent-API contract spot-check |
| F4 | 12 | Resolved | Operator-directive correction cost: 3 mid-cycle slips codified back into the methodology so future cycles don't repeat |
| F5 | 5 | Resolved | Rate-limit graceful-degradation discipline vindicated by Review 82 Finding 4 R2-Performance-Engineer rate-limit-hit + clean recovery |
| F6 | 2 | Deferred | Token economy per finding NOT knowable from audit trail; methodology should add token-tally discipline → PR [#40](https://github.com/magnificentlycursed/guild-portfolio/pull/40) |
| F7 | 9 | Deferred | Cold-session-budget declaration absent from DESIGN.md § Project intent; intent-tier table should add budget column → PR [#40](https://github.com/magnificentlycursed/guild-portfolio/pull/40) |
| F8 | 13 | Deferred | Pre-cycle methodology check missing from primer 3; suite-side review-log doesn't open each cycle with pre-spawn declaration → PR [#40](https://github.com/magnificentlycursed/guild-portfolio/pull/40) |
| F9 | 1 | Dismissed | Candidate Dim-1 session-isolation concern — dismissed on close read; Review 82 spawn evidence shows prior-round review-log loaded as adversary's claim per regression-check discipline |
| F10 | 3 | Hallucinated | Candidate Dim-3 prompt-cache-divergence concern — failed evidence check; spawn prompts were templated against a common base |

**MVR signal:** NOT REACHED at Round 1 per the [Phase 3 primer](../../vsdd-suite/primers/3-review-session.md) G-131 continue-trigger; 8 substantive findings (5 Resolved + 3 Deferred) mandate an AI Engineer Round 2 in a future cycle. The 3 Deferred findings target methodology-authoring work (suite-side, not project-side) and route to PR [#40](https://github.com/magnificentlycursed/guild-portfolio/pull/40) upstream-suite-remediation per the operator-queued sequencing.

**Per-finding token cost (estimated for this round):** ~21k tokens/finding — well below the capstone-intent expected band floor of 100k/finding per the [AI Engineer domain prompt](../../vsdd-suite/domains/role/AI-ENGINEER-REVIEW.md) Dim 2. Read as cold-session discipline working well (efficient context-load AND substantive findings) rather than under-investment.

---

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
