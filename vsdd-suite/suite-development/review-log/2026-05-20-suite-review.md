# Suite Review — 2026-05-20

---

## Review 77 — 2026-05-20 15:45Z

**Scope:** Operator-directed methodology change — introduce the **ownership / blocking / validation lifecycle** for project-level findings, in response to operator observation about cross-domain relationship patterns (Security ↔ Red Team as adversarial pair; TW ↔ Documentation Reviewer as proposed parallel; SA/SO leadership receiving expert advice; QE/Security raisers with other-domain fixers; Platform Engineering shift-left collaboration). Existing classification-centric model captured WHAT a finding is and WHERE it routes, but not WHO owns the fix, WHAT blocks closing, or WHO validates the fix landed clean. Review 77 introduces four lifecycle fields per finding plus sub-state lifecycle on Open findings. Strict self-validation policy per operator selection. Owner-only (no Layer qualifier) per operator selection. Artifacts touched: `suite-development.md` § Per-review entry preamble + § Finding body + new § Validation loop discipline sub-section; 16 domain prompts each gained a `**Validator pair (Review 77):**` paragraph; `check-project-review-discipline.py` extended with 5 new checks gated on separate 2026-05-21 threshold; `templates/PROJECT-FINDINGS-INDEX-template.md` schema extended with Owner + Validator columns; `suite-development/FINDINGS-INDEX.md` forward-only registry schema parallel-extended.

**Lens:** Cross-artifact methodology change + cross-domain relationship modeling. Sycophancy compensation: resisted multi-domain Owner, layer qualifier, multi-validator support; each was honestly evaluated and rejected for the simplest-form-that-works. Strict self-validation chosen over soft-warn per operator selection — the friction cost (one sentence per legitimate self-validation) is justified by the discipline gain.

**Session note:** In-session with the operator who articulated the relationship patterns + made four explicit methodology selections (single PR scope; migrate bookmark-cli rather than forward-only-preserve; owner-only no Layer qualifier; strict self-validation). Resisted bundling Review 77 with the Documentation Reviewer domain registration + reference-example apply per the operator's chosen PR-phasing — methodology change ships first; apply lands in subsequent PRs.

**Source:** director-raised — operator articulated the relationship patterns + made the four methodology selections via clarifying-question UI.

### Resolved

**Finding 1 — Ownership / blocking / validation lifecycle methodology introduced (Validation loop discipline)**

The suite's existing finding-classification model (Open / Resolved / Dismissed / Hallucinated + Phase 4 routing labels) captured what a finding IS and where the fix happens phase-wise, but didn't model: who is accountable for resolution (Owner); what other findings must close first (Blocked by); who validates the fix landed clean (Validator); and the sub-state progression within Open (raised → assigned → fix-landed → validated). The gap was most visible in adversarial pairs — Security and Red Team work today because they run as parallel cold sessions, not because the suite has a model for "Red Team validates Security's resolved finding by re-running its threat model against the post-fix code."

**Resolution scope:**

| Artifact | Change |
|---|---|
| `suite-development.md` § Per-review entry preamble | Added a note that ownership/validation lifecycle fields live in the per-finding body, NOT the entry preamble. |
| `suite-development.md` § Finding body | Structure block extended with 4 new fields (`**Owner:**` required for non-Hallucinated; `**Status:**` required for non-Hallucinated; `**Blocked by:**` optional; `**Validator:**` required for Resolved). Bullet list extended with field-order rule + Hallucinated exemption + forward-only constraint. |
| `suite-development.md` § Validation loop discipline (new sub-section, ~80 lines) | Names the four fields, lifecycle sub-states with transition table, strict self-validation policy (Portfolio Assessment domain-level allowlist), owner-field qualifier choice (single domain slug; no Layer qualifier), and forward-only constraint (2026-05-21 cutoff). |
| 15 role + 1 meta domain prompt | Each gained a `**Validator pair (Review 77):**` paragraph after the Language-and-interface-supplement line. Pair mapping: Security ↔ Red Team (adversarial pair); TW ↔ Doc Reviewer (forward-link); QE → SE or `*self*`; SE → QE; SA → SO or `*self*`; SO → VDD-IAR Alignment; VDD-IAR Alignment → SO; PE → SE or `*self*` (shift-left); DE → SE or PE; UX → SE or SO; Accessibility → SE or UX; Privacy → Security or SO; Localization → SE; Performance Engineer → SE; Portfolio Assessment → `*self*` (blanket allowlist). |
| `vsdd-suite/hooks/check-project-review-discipline.py` | New `_check_lifecycle_fields` function adds 5 checks gated on 2026-05-21 threshold: Owner-required (Raised-to-SO shorthand accepted); Owner is known domain slug; Validator-required-on-Resolved; Validator is known slug or `*self*` with substantive rationale (placeholder patterns `TBD`, `N/A`, `no pair available` rejected); Status value in `{raised, assigned, fix-landed, validated}`. Portfolio Assessment blanket-allowlisted for `*self*`. |
| `vsdd-suite/templates/PROJECT-FINDINGS-INDEX-template.md` | Schema extended with Owner + Validator columns. Quick-lookup section gained two new grep examples + a "Self-validated findings (audit-trail signal)" diagnostic grep. Inline HTML comment updated with Owner/Validator semantics + forward-only constraint. |
| `vsdd-suite/suite-development/FINDINGS-INDEX.md` § Findings registry (forward-only) | Schema parallel-extended with Owner + Validator columns. |

**Per-finding example (before vs. after Review 77):** before-form omits Owner / Status / Validator and doesn't tell a reader who fixed the finding or whether it was cross-domain-validated; after-form makes ownership and validation visible at finding-body level. The discipline gain compounds with project size — a 50-finding project with no Owner/Validator fields has an opaque workload graph; with the fields, `grep "| Owner: software-engineer | open |"` answers "what does SE owe right now?" in one shell command.

**Forward-only constraint:** Lifecycle fields apply to findings dated 2026-05-21 or later. Pre-cutoff findings in any project (including Reviews 73–76 in this suite-review log) are NOT migrated by the hook's enforcement. The reference examples MAY migrate as part of their capstone-intent promotion under the G-177 precedent — deliberate per-project decision in a subsequent PR.

**Most-uncertain choice noted:** Portfolio Assessment blanket-allowlist for `*self*`. The alternative was requiring per-finding rationales even for Portfolio's introspective dimensions. Chose blanket-allowlist because Portfolio's classification universe is structurally non-defect — there's nothing to validate cross-domain. If a future Portfolio-related review identifies a per-finding case where cross-domain validation WOULD apply, the rationale can be added per-finding; the blanket-allowlist is the default, not the only option.

**Resolution:** All 7 artifact changes applied (suite-development.md + 16 domain prompts + hook + 2 registry/template files). Hook tested clean against existing project review logs — pre-cutoff dates skip the lifecycle gates correctly.

**Finding 2 — Sanity Check meta domain introduced as validator-of-last-resort + rubber-ducking surface**

Surfaced mid-session by operator observation against Finding 1's strict-self-validation policy. The policy required `**Validator:** *self*` with substantive rationale for findings whose work has no cross-domain pair (PE shift-left mechanizations, SA architecture-doctrine, QE test-discipline meta, Portfolio Assessment introspective dimensions, Security findings with no Red Team validation surface, TW findings pre-Doc-Reviewer-domain-registration). Operator observed: the self-validation seam is the discipline's degradation seam; better to have a structured **meta-validator** for these cases than to rely on the per-finding rationale to be honest. The operator articulated a new meta domain (Sanity Check) with two purposes: (1) primary — validate findings without a natural cross-domain pair; (2) secondary — rubber-ducking surface for developers working through problems whose solution emerges in articulation. The domain takes DESIGN.md + architectural context as input.

**Resolution scope:**

| Artifact | Change |
|---|---|
| `vsdd-suite/domains/meta/SANITY-CHECK-REVIEW.md` (new file, ~120 lines) | Domain prompt authored. Meta domain (no reviewer-role persona). Purpose statement names the two-purpose design (validator-of-last-resort + rubber-ducking). Scope: DESIGN.md + architectural context + (for validator-of-last-resort) the originally-raising domain's finding + resolution; (for rubber-ducking) the developer's prose articulation. Read DESIGN.md FIRST — the meta-domain holds the spec contract against every other domain's local view. Classification universe: Resolved / Dismissed / Hallucinated (meta-domain pattern matching VDD-IAR Alignment). Sycophancy check names the failure mode: agreement with the developer's articulation. 8 standard dimensions: (1) Coherence with DESIGN.md; (2) Coherence with architecture; (3) Internal consistency of articulation; (4) Hidden assumption surfacing; (5) Validator-of-last-resort discipline (three questions: does the fix address the reported concern? does the fix introduce new defects? does the fix change covered behaviors the spec doesn't permit?); (6) Rubber-duck closure honesty (insight-reached or insight-not-reached with next-session purpose); (7) Spec-drift detection at meta level (across-session pattern); (8) Meta-discipline integrity (Sanity Check is itself subject to suite discipline). Validator pair: VDD-IAR Alignment (Sanity Check rarely raises its own findings; the typical case is Sanity Check producing a finding owned by a different domain — the originating domain validates the re-opened finding). |
| `vsdd-suite/domains/DOMAIN-INDEX.md` § Meta domains | New row for Sanity Check naming the two-purpose design + activation criteria (no gate-requirement at any intent tier — invoked by need, not schedule). |
| `vsdd-suite/suite-development/suite-development.md` § Validation loop discipline | Strict-self-validation-policy paragraph rewritten: the recommended path for findings without a cross-domain pair is `**Validator:** sanity-check`, not `*self*`. `*self*` remains valid for cases where the work has no spec/architecture interface at all. New paragraph introducing Sanity Check meta domain with the two-purpose design + classification universe + when-it-runs guidance. Validator-of-last-resort discipline expanded — names what Sanity Check does in that role (read finding + resolution + DESIGN.md, ask three questions, validate or re-open). Domain-level allowlist retired (Portfolio Assessment moves to using `sanity-check` instead of the prior blanket `*self*` allowlist; the hook's `SELF_VALIDATION_BLANKET_ALLOWLIST` set is now empty by default). |
| `vsdd-suite/hooks/check-project-review-discipline.py` | `DOMAIN_CLASSIFICATIONS` extended with `"sanity-check": {"Resolved", "Dismissed", "Hallucinated"}`. `KNOWN_DOMAIN_SLUGS` consequently includes `sanity-check` (computed as the dict's keys plus `documentation-reviewer`). `SELF_VALIDATION_BLANKET_ALLOWLIST` reduced to empty set (Portfolio Assessment is no longer blanket-allowlisted — Sanity Check supersedes). The strict-self-validation check still runs but the recommended path is `**Validator:** sanity-check`; `*self*` cases are now genuinely rare (cases where even Sanity Check can't validate). |
| 6 domain prompts | Updated to reference `sanity-check` as the validator-of-last-resort instead of `*self*`: `SECURITY-REVIEW.md` (findings with no Red Team validation surface), `QUALITY-ENGINEER-REVIEW.md` (test-discipline meta), `PLATFORM-ENGINEER-REVIEW.md` (shift-left mechanizations), `SOLUTION-ARCHITECT-REVIEW.md` (architecture-doctrine without Raised-to-SO), `PORTFOLIO-ASSESSMENT-REVIEW.md` (introspective dimensions — blanket-allowlist retired in favor of `sanity-check`), `TECHNICAL-WRITER-REVIEW.md` (pre-Doc-Reviewer-domain-registration interim path). The 5 PE-authored existing suite hooks preserve their original `*self*` framing per G-89 forward-only narrative-preservation. |

**Why a separate meta domain rather than extend Validation loop discipline's policy:** Sanity Check is a substantive domain with its own dimensions (coherence with DESIGN.md / architecture, hidden assumption surfacing, rubber-duck closure honesty, spec-drift detection). It's not a hook-level policy or a flag on `*self*` — it's a session-type the developer or originating-domain author can invoke, with its own sycophancy check and discipline. The meta-domain shape (parallel to VDD-IAR Alignment) is the correct structural home for it. Sanity Check's secondary purpose (rubber-ducking) is also a genuine session-type the suite hasn't had — articulating a problem to a structured listener and surfacing inconsistencies + hidden assumptions is real methodology work, not a slot to fill on the validation form.

**Forward-only constraint:** the `sanity-check` validator-pair is the recommended path for findings dated 2026-05-21 or later (Review 77 cutoff). Pre-cutoff findings that landed under `*self*` (e.g., the 5 existing suite hooks' PE shift-left framing; the existing 3 bookmark-cli-manual rounds' Portfolio Assessment blanket allowlist if any) are preserved per G-89.

**Resolution:** All 5 artifact changes applied (new domain prompt + DOMAIN-INDEX entry + suite-development.md rewrite + hook update + 6 domain prompts updated). Hook tested clean against existing project review logs — `sanity-check` is now a recognized domain slug.

### Summary

2 findings Resolved in-session (Finding 1 = ownership/validation lifecycle methodology + Finding 2 = Sanity Check meta domain). Methodology introduction is structurally complete. The validator-of-last-resort + rubber-ducking design closes the seam where the strict-self-validation policy was most fragile. Sub-tasks (Documentation Reviewer domain registration; apply Review 77 to reference examples via capstone-intent promotion) forward-linked to subsequent PRs per operator's phasing. Backlog after Review 77: 0 Open + 7 Deferred (G-159, G-168, G-169, G-170, G-171, G-172 unchanged + Review 76 Finding 4 bundled-Deferred — no new findings registered this Review).

**Coordination:** Documentation Reviewer ↔ TW pair is forward-linked from TW's new Validator-pair paragraph + Python/Bash supplements' Doc Reviewer sections. The forthcoming Doc Reviewer domain registration (next Review) activates the pair. The reference-example apply (capstone promotion + migrate existing rounds + activate new domains' cold sessions + Phase 6 convergence) is the largest forward-linked piece. Sanity Check itself is immediately operational — no additional registration needed; the next Resolved finding without a cross-domain pair declares `**Validator:** sanity-check`.

---

## Review 76 — 2026-05-20 14:30Z

**Scope:** Operator-directed via a human reviewer's question — why do hooks that are Python scripts end in `.sh`? Two coordinated outputs: (a) author the suite's first Python language supplement and its first Bash language supplement (the suite previously had only Rust + JS/TS); (b) review the 7 scripts the suite ships (4 Python hooks + 1 bash hook + 2 bash templates) against the new supplements and apply findings. Artifacts touched: `vsdd-suite/supplements/python.md` (new ~400 lines); `vsdd-suite/supplements/bash.md` (new ~350 lines); `git mv` × 4 (Python hooks `.sh` → `.py`); internal docstring self-references rewritten; `.pre-commit-config.yaml` 4 entry paths updated. Read this round: every script in `vsdd-suite/hooks/` and `vsdd-suite/templates/`; existing `vsdd-suite/supplements/rust.md` (as template); FINDINGS-INDEX.md legacy G-139 entry that named the `.sh` extension as "for parity" (the choice this Review retires).

**Lens:** Cross-artifact-consistency + multi-domain authoring + dogfood-validation (QE + Security + Red Team + SE + SA + PE + DE + TW perspectives applied to the suite's own scripts via the new supplements). Operator-raised observation (Source: director-raised) triggered by an external human reviewer's question.

**Session note:** In-session with the operator who relayed the human reviewer's question and directed the supplement-then-review sequence. Sycophancy compensation: the natural temptation was to do the rename alone (one-line fix) and skip the supplement work; resisted because the rename without the supplement would close the symptom without addressing the cause (no Python-domain guidance existed, so the Python hooks were authored without per-domain Python-specific discipline). The supplements are the load-bearing change; the rename is the worked example of one finding the supplement teaches (Bash supplement § Platform Engineering "Filename extension matches content"). Findings batched into this Review rather than per-script log entries because the scripts are suite-development artifacts, not project artifacts.

**Source:** director-raised — operator surfaced the human reviewer's question; the bash-supplement scope expansion (added after Python supplement landed) was a follow-up operator directive in the same session.

### Resolved

**Finding 1 — Python language supplement authored at `vsdd-suite/supplements/python.md` (multi-domain authorship)**

The suite shipped Rust + JS/TS supplements but no Python supplement, despite shipping 4 Python hooks AND being applicable to Python projects users might build. The absence meant the Python hooks were authored without per-domain Python-specific guidance, and projects using the suite for Python work had no language-specific dimensions. The omission compounds Finding 2 — if the suite had a Python supplement with the "filename extension matches content" dimension visible at authoring time, the hooks would never have been written as `.sh`.

**Resolution:** Authored `vsdd-suite/supplements/python.md` (~400 lines) with 11 per-domain sections following the canonical supplement structure (Quality Engineering, Security, Software Engineering, Platform Engineering, Data Engineering, Red Team, Performance Engineer, Solution Architect, Technical Writer, Documentation Reviewer, Localization). Multi-domain perspective applied: QE names `pytest` + `hypothesis` + `mutmut` + `coverage.py` + `mypy --strict` as the test-discipline floor; Security + Red Team enumerate Python-specific exploit surfaces (eval/exec/pickle/yaml.load/subprocess shell=True/SQL injection/path traversal/XXE/PyPI typosquatting); PE anchors against the 2026 ecosystem (uv replacing pip+venv; ruff replacing flake8+isort+pyupgrade+black; pyproject.toml as canonical config); DE names pydantic + msgspec with the asymmetric-trust-boundary (G-126) and strictness-symmetry (G-152) generalizations applied; SA addresses src/ vs flat layout, circular imports, sync/async boundary, purity-boundary explicit per Dim 12; TW + Documentation Reviewer (forward-linked to Review 77) cover docstring formats, Sphinx vs mkdocs, README→PyPI rendering, `help()` discoverability.

**Finding 2 — `.sh` extension on Python hooks retired; rename to `.py` (filename-content match)**

4 Python hooks shipped with `.sh` extensions per G-139 (Review 48, 2026-05-18) "for parity" with the sibling actually-bash hook. "For parity" aged poorly: editors apply bash syntax highlighting to the files (wrong); pre-commit configs scoped by extension would silently miss the Python-ness; readers expect bash conventions from `.sh` and find Python. Bash supplement § Platform Engineering names this directly. A human reviewer surfaced the misnomer in seconds; in-context contributors had lived with it for months.

**Resolution:** `git mv` rename (preserves history):

| Before | After |
|---|---|
| `vsdd-suite/hooks/check-changelog-currency.sh` | `vsdd-suite/hooks/check-changelog-currency.py` |
| `vsdd-suite/hooks/check-crosslink-references.sh` | `vsdd-suite/hooks/check-crosslink-references.py` |
| `vsdd-suite/hooks/check-suite-review-preamble.sh` | `vsdd-suite/hooks/check-suite-review-preamble.py` |
| `vsdd-suite/hooks/check-project-review-discipline.sh` | `vsdd-suite/hooks/check-project-review-discipline.py` |

Internal `.sh` self-references in docstrings rewritten to `.py`. `.pre-commit-config.yaml` 4 `entry:` lines updated. Renamed hooks tested clean against existing project-review-log + suite-review-log files. Preserved per G-89: actually-bash hook `check-review-log-anonymization.sh` and templates `cold-session-dispatch.sh` + `scaffold-project.sh` keep `.sh` (correctly-named). Historical references in CHANGELOG / COMPATIBILITY / review-log + G-139's row preserve original framing.

**Finding 3 — Bash language supplement authored at `vsdd-suite/supplements/bash.md` (multi-domain authorship)**

Same gap as Finding 1, mirrored for Bash: the suite shipped 3 actually-bash scripts but no Bash supplement. `check-review-log-anonymization.sh` shows symptoms (uses `set -u` only; `[ ]` test syntax instead of `[[ ]]`; IFS not set) — defensible but never made explicit-and-justified.

**Resolution:** Authored `vsdd-suite/supplements/bash.md` (~350 lines) with 11 per-domain sections. Multi-domain perspective applied: QE names `bats-core` + `shellcheck` + `kcov`; Security + Red Team enumerate bash-specific exploit surfaces (unquoted variable expansion → word splitting → command/glob injection; `eval` on user input; predictable temp-file names + symlink races; `tar`/`zip` extractall path traversal; PATH-shadowing); PE anchors `#!/usr/bin/env bash` shebang, bash version requirements (macOS 3.2 caveat), `shellcheck` + `shfmt` in CI; SE codifies `[[ ]]` over `[ ]`, array discipline, `local` for function vars, `readonly` for constants; SA addresses script structure at scale (main function pattern, sourceable wrapper); TW + Doc Reviewer cover `--help` as primary documentation, error-message executability.

**Finding 4 — Suite's own scripts reviewed against the new supplements (consolidated findings)**

Python and Bash supplements applied as a review pass against the 7 in-scope scripts. Findings batched here (rather than per-script log entries) because they're minor stylistic relative to the renamed-extension headline; per-script logs would over-process for finding severity.

#### Sub-findings (Python — `vsdd-suite/hooks/*.py`)

| Script | Finding | Severity |
|---|---|---|
| All 4 Python hooks | No `from __future__ import annotations` — modern Python practice for PEP 604 union syntax | Minor |
| All 4 Python hooks | No automated tests for the hooks themselves — meta-test gap | Medium |
| All 4 Python hooks | No `mypy --strict` configuration; type hints present but not enforced | Medium |
| `check-suite-review-preamble.py` + `check-project-review-discipline.py` | Use `typing.List` / `typing.Dict` form; modern Python (3.9+) supports `list[str]` directly | Minor |
| All 4 Python hooks | No `ruff format` / `black` enforcement configured | Minor |

#### Sub-findings (Bash — `vsdd-suite/hooks/check-review-log-anonymization.sh` + `vsdd-suite/templates/*.sh`)

| Script | Finding | Severity |
|---|---|---|
| `check-review-log-anonymization.sh` | `set -u` only; missing `set -e` and `set -o pipefail` per Bash supplement § Security baseline | Medium |
| `check-review-log-anonymization.sh` | `[ ]` test syntax instead of `[[ ]]` | Minor |
| `check-review-log-anonymization.sh` | IFS not explicitly set | Minor |
| All 3 bash scripts | `shellcheck` not run as a pre-commit hook (tool not installed in the suite's dev environment) | Medium |
| `cold-session-dispatch.sh` | `tr` + `sed` chained where bash 4+ `${var^^}` would suffice | Minor |
| `scaffold-project.sh` | Mixed `[ ]` and `[[ ]]` styles | Minor |

**Resolution:** All 11 sub-findings registered Deferred with a shared trigger — the next "suite-self-hardening pass" that adopts shellcheck + ruff + mypy + bats-core configuration for the suite's own scripts. Auto-Backlog clause per G-130: if no progress by 2026-09-01, auto-Backlog and re-raise as PE priority candidates. The forward-only FINDINGS-INDEX.md registry stays empty (the Deferreds bundle under this Review's narrative — they share a single trigger and are stylistic-not-correctness, so per-row registration is over-discipline for the severity).

**Meta-finding (sycophancy compensation):** the suite teaches tools (`shellcheck`, `ruff`, `mypy`, `bats-core`, `cargo-mutants`) but doesn't enforce them on its own scripts. The asymmetry is itself a finding — the suite eats its own cooking on conventions (per-domain index structure, finding classification, registry shape) but not on tooling. Parallel to G-122 (purity-boundary documented but not enforced); resolution is the future suite-self-hardening pass. Forward-link only; not actionable in this Review.

### Summary

4 findings Resolved in-session (Python supplement authored; `.sh` → `.py` rename × 4; Bash supplement authored; consolidated review with 11 sub-findings batched-Deferred under a shared trigger). Supplements are the load-bearing change; rename is the worked example one supplement teaches. Forward-only per G-89: historical `.sh` references in CHANGELOG / COMPATIBILITY / review-log preserved; new references use `.py`. Backlog after Review 76: 0 Open + 6 Deferred from prior reviews + 1 bundled-Deferred from this Review.

**Coordination:** Documentation Reviewer section in the Python supplement (and parallel in Bash supplement) is forward-linked to Review 77 + 78. No coordination required in this Review — supplements are structurally complete and the forward-reference is harmless.

---

## Review 75 — 2026-05-20 13:15Z

**Scope:** Operator-directed reference-example folder restructure. (1) Create new top-level folder `vsdd-suite-reference-examples/` to house portfolio reference implementations. (2) `git mv bookmark-cli vsdd-suite-reference-examples/bookmark-cli-manual` — rename the existing reference to signal it's the manual-method variant. (3) Establish forward-link for a parallel `bookmark-cli-crosslink/` to be built in a subsequent PR (crosslink-method variant). (4) Update suite-side forward-facing references to the new path. (5) Restructure top-level portfolio README so `vsdd-suite/` and `vsdd-suite-reference-examples/` are listed as portfolio projects in their own right (not subsidiary sections). Artifacts touched this round: `bookmark-cli/` (entire tree, git mv to new location); `vsdd-suite/README.md` Worked-example intro paragraph (added reference-impl pointer); `vsdd-suite/primers/1c-decomposition.md` § Manual testing checklist (reference-example pointer); `vsdd-suite/primers/5-formal-hardening.md` Surface A.0 worked example (path update for G-173 historical reference); `vsdd-suite/crosslink-contract.md` Contract testing section (reference-impl path update); `guild-portfolio/README.md` (project listing restructure, forward-only compatibility section); `vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/{FINDINGS-INDEX,QUALITY-ENGINEER-REVIEW,SOLUTION-ARCHITECT-REVIEW}.md` (relative-path correction `../../vsdd-suite/` → `../../../vsdd-suite/` for the deeper-nesting); `vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/FINDINGS-INDEX.md` H1 (`bookmark-cli` → `bookmark-cli-manual`); `vsdd-suite-reference-examples/bookmark-cli-manual/TODO.md` lead (link fix + rename).

**Lens:** Operator-directed reference-example architecture pass — name the reference for what it's a reference TO (the manual method specifically), make room for the parallel crosslink-method reference, treat vsdd-suite + reference examples as portfolio projects rather than as suite-internal infrastructure.

**Session note:** In-session with the operator who directed the restructure across three iterations of clarifying directives ("bookmark-cli is the reference example for the manual method"; "put it in a folder called vsdd-suite-reference-examples"; "create another bookmark-cli in that project that uses the crosslink workflow"; "vsdd-suite and the reference examples are portfolio projects"). Sycophancy compensation: the natural temptation was to author the crosslink-variant reference in this PR alongside the rename; resisted because the crosslink-variant build is substantial (cold-session IAR rounds, PROCESS.md authoring, full 6-phase walkthrough) and warrants its own PR — PR 3 of the Review 73 / 74 / 75 sequence. Per the operator's "one PR at a time — no stacked PRs" doctrine, this PR's scope is structural-only (rename + folder restructure + path updates); the crosslink-variant build and capstone-promotion + 6-phase completion for both variants land in PR 3.

**Source:** director-raised — operator named the folder restructure + rename + crosslink-variant requirement directly across multiple messages within this conversation.

### Resolved

**Finding 1 — Reference-example folder restructure + bookmark-cli rename to bookmark-cli-manual (Reference-example architecture)**

The portfolio's `bookmark-cli/` reference implementation served two implicit roles that the operator surfaced as a coherence concern: (a) it was the worked-example reference, AND (b) it was specifically the manual-method reference (no crosslink). Per the G-144 two-mode design principle, both operational modes (`[crosslink]` recommended; `[manual]` first-class fallback) deserve reference implementations of equal weight; having only one reference (the manual variant) under-represented the crosslink mode. The operator directed: rename the existing reference to signal its manual-method nature, restructure the portfolio to host both reference variants as sibling projects under a dedicated folder, and treat the VSDD suite + the reference examples as portfolio projects in their own right rather than as suite-internal infrastructure.

**Resolution scope:**

| Artifact | Change |
|---|---|
| `bookmark-cli/` (entire tree) | `git mv bookmark-cli vsdd-suite-reference-examples/bookmark-cli-manual` — preserves git history; the rename signals the manual-method variant identity. |
| `vsdd-suite-reference-examples/bookmark-cli-crosslink/` | Reserved for PR 3 — the crosslink-method variant reference. Top-level portfolio README and the suite docs both reference its forthcoming path forward-link so a reader can see what's coming. |
| `vsdd-suite/README.md` § Worked example intro paragraph | Added reference-impl pointer naming both variants (`bookmark-cli-manual/` and `bookmark-cli-crosslink/`) at their `vsdd-suite-reference-examples/` paths; framed as the two variants that realize the walkthrough end-to-end. Worked-example Phase 1c row in the overview table updated to mention `manual-tests/` folder produced. |
| `vsdd-suite/primers/1c-decomposition.md` § Manual testing checklist | The Review 74 reference-example pointer reframed to name both variants (manual + crosslink) as adopters of the new manual-test-split convention. |
| `vsdd-suite/primers/5-formal-hardening.md` Surface A.0 worked example | Historical G-173 reference to bookmark-cli's `src/lib.rs:1-7` purity claim updated to the new path (`vsdd-suite-reference-examples/bookmark-cli-manual/src/lib.rs:1-7`). G-173 the finding stays as historical anchor; the path-reference is updated forward-only. |
| `vsdd-suite/crosslink-contract.md` § Contract testing | Reference-implementation citation updated: the manual-method variant exercises the worked example in manual mode; the crosslink-method variant (forthcoming) exercises it in crosslink mode and serves as the canary for contract-drift detection + G-106 closure verification. |
| `guild-portfolio/README.md` (top-level portfolio README) | Project listing restructured — `### Bookmark Manager` and `### Issue Tracker CLI` remain as before; new `### VSDD Suite — Methodology project` entry naming the suite as its own portfolio project with a component-status table; new `### VSDD Suite reference examples — Worked-example projects` entry naming both variants with their per-variant role + forward-link for the crosslink variant. The standalone `## The suite` section was retired (collapsed into the new project entry). `## Forward-only compatibility` section restated to name both reference variants stay current with each convention shift as part of being the worked example. |
| `vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/{FINDINGS-INDEX,QUALITY-ENGINEER-REVIEW,SOLUTION-ARCHITECT-REVIEW}.md` | Relative-path correction `../../vsdd-suite/` → `../../../vsdd-suite/` for the deeper nesting (the move added one level). 3 files, 8 path-references rewritten. |
| `vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/FINDINGS-INDEX.md` H1 + opening paragraph | `bookmark-cli` → `bookmark-cli-manual`; the broken `GAP-ANALYSIS-LOG.md` reference at line 5 fixed (pointing at the renamed `FINDINGS-INDEX.md` instead, per G-149 closure that was applied to the suite but not propagated to bookmark-cli's reference). |
| `vsdd-suite-reference-examples/bookmark-cli-manual/TODO.md` lead | H1 `bookmark-cli` → `bookmark-cli-manual`; broken `GAP-ANALYSIS-LOG.md` link fixed; framing paragraph updated to name the variant explicitly + note the capstone-intent promotion and 6-phase completion land in PR 3 of the Review 73 / 74 / 75 sequence. |

**Forward-only constraint (G-89 precedent):** Historical CHANGELOG / COMPATIBILITY / review-log entries that reference `bookmark-cli/` (the old path) are preserved as audit-trail records throughout the suite. The legacy `G-117`, `G-138`, `G-177`, `G-178`, `G-181` registry rows in `FINDINGS-INDEX.md` that cite the old path remain valid as historical anchors. Suite-development review-log entries (Reviews 44, 47, 49, 51, 52, 56, 60, 62, 65, 66, 67, 72) likewise preserve original framings.

**Forward-link to PR 3:** the crosslink-method variant (`vsdd-suite-reference-examples/bookmark-cli-crosslink/`) is referenced as a forward-link throughout the new prose; its actual build lands in PR 3 (capstone intent + crosslink workflow throughout + 6-phase completion). PR 3 also brings the manual-method variant up to capstone intent + 6-phase completion in parallel.

**Resolution:** All 9 artifact changes applied. Reference-example architecture is now coherent: two parallel reference implementations under `vsdd-suite-reference-examples/`, one per operational mode, both equally weighted; the suite + reference examples are listed as portfolio projects in their own right.

### Summary

1 finding Resolved in-session. The folder restructure is forward-only with full historical-anchor preservation per G-89. No new findings registered for tracking (no Open or Deferred findings). The forward-only `FINDINGS-INDEX.md` registry stays empty (this finding was Resolved in-session and does not need ongoing tracking).

**Coordination:** none — the change is scoped to the manual-method reference's location + name + forward-facing path references. The crosslink-variant build coordinates with PR 3 (forward-linked but out of this PR's scope).

---

## Review 74 — 2026-05-20 12:30Z

**Scope:** Operator-directed convention shift — manual testing plans split out of inline `TODO.md` checklists into per-layer files in a `manual-tests/` folder; new pre-commit hook to enforce project-level domain-review discipline (parallel to the Review 68 suite-review hook). Both changes reinforce project-level review-log discipline: the manual-test split keeps `TODO.md` as a navigable decomposition map by separating test-plan content into its own per-layer files; the new hook mechanizes the structural-discipline checks (`### Summary` section, `**Coordination:**` line, classification-heading universe, finding-header dim-reference) that the existing suite-review hook leaves uncovered for project-level review logs. Forward-only with reference-example carve-out: applies to projects whose first layer-gate close lands on or after 2026-05-20; pre-cutoff projects retain inline `TODO.md` checklist sections per G-89. The reference examples (`bookmark-cli-manual/` and forthcoming `bookmark-cli-crosslink/`) adopt the convention as part of their capstone-intent promotion (PR 3 scope). Artifacts touched this round: `vsdd-suite/primers/1c-decomposition.md` (§ Manual testing checklist new "File location" sub-section + per-layer-file structure spec; § TODO.md format template updated; § Completion criteria 3 + 7 updated); `vsdd-suite/README.md` Quickstart Phase 1c step + Session-primers table Decomposition row + Worked-example overview table Phase 1c row; `vsdd-suite/domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md` dim 9 (file-location standard added); new `vsdd-suite/hooks/check-project-review-discipline.sh` + `.pre-commit-config.yaml` wiring.

**Lens:** Cross-artifact consistency + mechanization (operator-raised observation surfaced two coordinated discipline concerns — the inline manual-test-checklist authoring shape, and the absent project-review-discipline hook parallel to the suite-review hook). Both are project-level review-log discipline reinforcements; the manual-test split is the convention shift, the new hook is the mechanization that catches drift.

**Session note:** In-session with the operator who raised both convention shifts directly across two messages within this conversation. Sycophancy compensation: the natural temptation was to bundle the manual-test split with PR 1's findings-index reshape since both are project-level discipline conventions; resisted because PR 1 was already focused (findings-index reshape only) and the manual-test split has its own reference-example apply step (`vsdd-suite-reference-examples/bookmark-cli-manual/manual-tests/layer-1.md` migration, deferred to PR 3 alongside the capstone promotion). The hook's classification-heading-universe lookup table was authored from `suite-development.md` § Finding classification schemas by domain type — every domain's universe encoded once, single source of truth.

**Source:** director-raised — operator named both convention shifts (manual-test split + parallel domain-review hook) directly. The new pre-commit hook's existence is itself the audit-trail mechanism for the discipline going forward.

### Resolved

**Finding 1 — Manual testing plans split into per-layer files in a `manual-tests/` folder (Phase 1c decomposition output / TODO.md format)**

`primers/1c-decomposition.md` prescribed an inline `**Manual Testing Checklist:**` block per Layer in `TODO.md`. With the runnable-step standard (per-step literal expected-output blocks; per-step clean-state setup; per-step binary-lifecycle steps), per-layer manual-test plans run 50+ lines per step and 200+ lines per layer. Bundling them inline in `TODO.md` (a) inflates `TODO.md` past the size where it serves as a navigable decomposition map, (b) mixes decomposition-plan concerns with test-plan concerns, and (c) makes per-layer test plans hard to diff, review, or cite by anchor independently. The operator's direction: split manual-test plans into per-layer files in a folder. The decomposition `TODO.md` Layer N block's `**Manual Testing Checklist:**` field becomes a one-line pointer to `manual-tests/layer-N.md`.

**Resolution scope:**

| Artifact | Change |
|---|---|
| `primers/1c-decomposition.md` § Manual testing checklist | New **File location (Review 74 convention shift — forward-only)** sub-section naming the convention: `manual-tests/layer-N.md` at project root (siblings to `DESIGN.md` / `TODO.md` / `src/`), one file per layer; `TODO.md` Layer N's `**Manual Testing Checklist:**` field becomes a one-line pointer; structural rationale (file-size, diff-ability, citation by anchor); forward-only constraint with reference-example carve-out. Per-layer file structure spec added (H1, layer-reference field, tested-against field, step blocks). |
| `primers/1c-decomposition.md` § TODO.md format template | Per-Layer block's `**Manual Testing Checklist:**` rewritten from inline placeholder bullets to a one-line pointer at `manual-tests/layer-N.md` with note about the forward-only carve-out. |
| `primers/1c-decomposition.md` § Completion criteria | Criterion 3 updated to name the per-layer-file convention and the forward-only carve-out. Criterion 7 (Phase 2+ crosslink projects) updated to clarify that per-layer manual-test files live in `manual-tests/layer-N.md` in both modes; crosslink projects reference them from the layer issue's comment thread. |
| `vsdd-suite/README.md` Quickstart Phase 1c step | Added the per-layer-file convention requirement alongside `TODO.md` authoring. |
| `vsdd-suite/README.md` § Session primers Decomposition (Spec Review Gate) row | "manual testing checklists" → "per-layer `manual-tests/layer-N.md` files (Review 74 convention; pre-cutoff projects retain inline TODO.md checklists)". |
| `vsdd-suite/README.md` § Worked example overview table Phase 1c row | Output column now reads "crosslink layer hierarchy (or `TODO.md`) + `manual-tests/` folder"; manual-mode column adds "author one `manual-tests/layer-N.md` per layer". |
| `vsdd-suite/domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md` dim 9 | Extended with the **File location** sub-paragraph: projects subject to the Review 74 convention carry the per-layer checklist in `manual-tests/layer-N.md`; pre-cutoff projects carry inline `TODO.md` sections; a project whose `TODO.md` Layer N has a pointer but no actual `manual-tests/layer-N.md` file is a finding (pointer without target is a defect). |

**Forward-only constraint:** Applies to projects whose first layer-gate close lands on or after 2026-05-20. Pre-existing projects (`bookmark-manager/`, `issue-tracker-cli/`) retain their inline `TODO.md` checklist sections per G-89. The reference examples (`vsdd-suite-reference-examples/bookmark-cli-manual/` and forthcoming `bookmark-cli-crosslink/`) adopt the convention as part of their capstone-intent promotion in PR 3 — reference implementations are kept current with the conventions they teach.

**Resolution:** All 4 forward-facing artifacts updated as enumerated (primer + README + domain prompt). The reference-example application (bookmark-cli-manual's `TODO.md` Layer 1 inline block split into `manual-tests/layer-1.md`) lands in PR 3.

**Finding 2 — New pre-commit hook `check-project-review-discipline.sh` enforces project-level domain-review entry-structure discipline (parallel to Review 68 suite-review hook)**

The existing `check-suite-review-preamble.sh` (Review 68) validates per-review preamble fields, finding-header forms, closer-line presence, and Source-value enumeration across both suite-review and project-level review-log files. But the project-level review-log discipline at `suite-development.md` § Governing standard for project-level review logs has additional requirements that the suite-review hook intentionally does not enforce: (a) `### Summary` section presence per Review entry; (b) `**Coordination:**` line presence (with `*(none)*` placeholder allowed); (c) classification-section headings matching the domain's classification universe per `suite-development.md` § Finding classification schemas by domain type (15 domain-specific universes); (d) finding-header dim-reference parenthetical (`(Dim X)`, `(Phase 5 Surface B)`, `(Rust supplement — path traversal)`) for non-Hallucinated findings; (e) domain-slug recognition vs. the suite's canonical slug set. The operator surfaced the asymmetry: the suite-review discipline has its own hook; the project-review discipline should too.

**Resolution scope:**

| Artifact | Change |
|---|---|
| `vsdd-suite/hooks/check-project-review-discipline.sh` (new file, ~250 lines Python) | Implements the 5 project-level discipline checks above. Domain classification universes encoded as a `DOMAIN_CLASSIFICATIONS` dict per `suite-development.md` § Finding classification schemas by domain type. Discipline-reference parenthetical accepts any trailing `(...)` group (not just `(Dim X)` specifically) — Phase 5 surface references and supplement references are equally valid per the standard's worked examples. Forward-only enforcement threshold 2026-05-20. Per-entry `<!-- hook-bypass: <rationale> -->` HTML-comment escape valve in the first 5 lines (bypass is itself a finding for next registry walk). Portfolio Assessment skipped from classification-heading check (per `suite-development.md` § Finding sections "Exception — Portfolio Assessment" uses dim-first organization). |
| `.pre-commit-config.yaml` | New `check-project-review-discipline` entry wired after the suite-review-preamble hook. Files-regex scopes to per-project review-log markdown only (`^.*/vsdd-suite/review-log/.*\.md$`); explicitly does NOT match suite-review-log files (which the preceding hook owns). |

**Verification:** Tested clean against all 3 existing bookmark-cli-manual review logs (`2026-05-17-quality-engineer.md`, `2026-05-20-quality-engineer.md`, `2026-05-20-solution-architect.md`) after one iteration on the discipline-reference parenthetical regex (initial draft required `(Dim X)` specifically; revised to accept any trailing parenthetical per the standard's worked examples — the QE Review 2 Surface B finding uses `(Phase 5 Surface B / G-174 5-disposition universe)` which is the correct shape).

**Resolution:** Hook authored, tested clean against existing project review logs, wired into `.pre-commit-config.yaml`. Going forward, project-level review-log entries dated 2026-05-20 or later are enforced; pre-cutoff entries are skipped per G-89.

### Summary

2 findings Resolved in-session. Both convention shifts are forward-only with full historical-anchor preservation per G-89 — pre-cutoff projects retain inline `TODO.md` checklists and pre-cutoff review-log entries are not enforced by the new hook. No new findings registered for tracking. The forward-only `FINDINGS-INDEX.md` registry stays empty (both findings were Resolved in-session). Backlog after Review 74: 0 Open + 6 Deferred (G-159, G-168, G-169, G-170, G-171, G-172 — unchanged from Review 73).

**Coordination:** Review 75 (folder restructure + bookmark-cli rename) — the reference-example variants both adopt the manual-test split convention as part of their capstone-intent promotion in PR 3. The new project-review-discipline hook validates the reference examples' review-log files going forward.

---

## Review 73 — 2026-05-20 11:30Z

**Scope:** Operator-directed convention shift — deprecate "gap analysis" / `G-XX` verbiage in the suite-development review and findings index; align suite findings logging with the same standards a project domain finding index uses; deliver consistent and intuitive suite-contributor / suite-user experience across scopes. Gaps not renamed retroactively (forward-only constraint per G-89). Artifacts read this round: `suite-development/FINDINGS-INDEX.md`; `suite-development/suite-development.md`; `suite-development/README.md`; `vsdd-suite/README.md` (lines 158, 371); `primers/3-review-session.md` (line 150); `hooks/check-suite-review-preamble.sh`; `bookmark-cli/vsdd-suite/FINDINGS-INDEX.md` (reference shape).

**Lens:** Cross-artifact consistency + dogfooding (SA dogfooding lens applied to the suite's own registry conventions — the suite teaches the project-level FINDINGS-INDEX shape, so its own findings registry should follow the same shape it teaches). Operator-raised observation (source: `director-raised` per G-133).

**Session note:** In-session with the operator who raised the convention shift directly; the decision was made via clarifying-question selection (drop ID prefix entirely; reshape forward-only with the Lens column; gaps not renamed retroactively). Sycophancy compensation: the natural temptation was to do a deeper sweep including the historical G-XX heading regex enforcement in the validation hook; resisted because the operator explicitly said "Gaps do not need to be renamed retroactively" — historical anchors stay valid, and the hook's existing `**G-XX — Title**` heading-form acceptance covers legacy-anchor walks. Each prose edit anchored to a specific file path (grep-verified before applying).

**Source:** director-raised — operator named the convention shift in chat; clarifying-question selections set the schema (drop prefix; reshape forward-only with Lens column).

### Resolved

**Finding 1 — Suite-development findings registry reshaped forward-only to mirror the project FINDINGS-INDEX shape; "gap analysis" / `G-XX` verbiage retired going forward (gaps not renamed retroactively)**

The suite-development governance files framed findings via "gap analysis" terminology and the `G-XX` ID series. The verbiage diverged from how the suite teaches projects to track findings — a project-level [`FINDINGS-INDEX.md`](../../../bookmark-cli/vsdd-suite/FINDINGS-INDEX.md) (bookmark-cli reference) uses `| ID | Layer | Round | Domain | Finding | Title | Source | Classification | Status | Anchor |` columns and identifies findings by per-domain Review-N + Finding-M anchors. A suite contributor walking the suite-development registry encountered different conventions than a suite user walking a project registry — failing the "consistent and intuitive experience" goal the operator named.

**Resolution scope:**

| Artifact | Change |
|---|---|
| `suite-development/FINDINGS-INDEX.md` | Opening prose dropped "(gaps)" parenthetical and "indexed by gap ID" framing; new **§ Conventions** section names the two-section structure and the project-shape mirroring goal; **§ Adding and updating findings** rewritten to drop new-G-ID instruction (legacy series closed; new findings identified by `Review N Finding M` anchor); **§ Reactivation triggers** prose rewrote "gaps" → "findings" while preserving the G-ID anchors as historical references; new **§ Findings registry (forward-only)** section added with the project-aligned schema `\| Review \| Lens \| Finding \| Title \| Source \| Classification \| Status \| Anchor \|` and an empty body (next suite review surfacing a tracked finding adds the first row); legacy `## Gap Registry` renamed to **§ Legacy registry (G-01–G-182, closed to new entries)** preserved untouched per G-89; trailing **Status values** footer split into legacy (Open · Addressed · Deferred · Dismissed · Context-Dependent) + forward-only (Closed · Open · Deferred) with disposition vs. lifecycle split explained. |
| `suite-development/suite-development.md` | Line 3 ("running gap analysis") → "walking the findings registry"; line 9 ("gap analysis log") → "findings index"; line 30 Suite-structure-table row renamed to "Findings index" with cross-scope-consistency note; line 358 ("gap registry" / "gap ID") → "findings registry" / "anchor (legacy `G-XX` for pre-2026-05-20 entries; `Review N Finding M` for forward-only)"; line 371 ("existing open gap") → "existing open finding"; line 377 ("gap registry" / "gap was tracked") → "findings registry" / "finding was tracked" with the legacy-vs-forward Resolved/Addressed disposition named; **§ Running gap analysis** header renamed to **§ Walking the findings registry** with body updated to call out both registries and the "no new G-IDs — legacy series closed" rule; **Suite review and review-log discipline** (three-artifact paragraph) updated "IAR suite" → "VSDD Suite", "gap registry" → "findings registry" with the two-section structure named, retired the "gap analysis runs" framing in the One-artifact-type paragraph; **Suite review entry format** (the load-bearing change) — Lens valid forms updated ("walk all open gaps" → "walk all open findings" + new forward-only example); classification headings reshaped to mirror project-level set (`### Resolved` / `### Dismissed` / `### Hallucinated` / `### Open` / `### Deferred`) with `### New gap registered` retired (existing entries preserved as historical records per G-89); finding-body rule updated — `**Finding N — Title**` is the heading form for all findings going forward (whether resolved in-session or registered for tracking); `**G-XX — Title**` retained as the accepted form for legacy-registry walks (re-walking pre-2026-05-20 entries); supplement-coverage closer ("gap registry") → "findings registry". |
| `suite-development/README.md` | Line 21 ("Living gap registry. Status-only table of every identified suite gap.") → "Living findings registry. Status-only registry … structured to mirror the project-level FINDINGS-INDEX shape …" with both registry sections named; line 42 ("walk all Open gaps") → "walk all Open findings"; line 44 ("new gap registered" / "existing gap" / "registers a gap") → finding-style language with the no-new-ID-prefix rule called out. |
| `vsdd-suite/README.md` (top-level user-facing) | Line 158 (Suite-scope item) "gap registry" → "findings registry"; line 371 (Session-primers table row) "running gap analysis" → "walking the findings registry"; "IAR suite" → "VSDD suite". |
| `primers/3-review-session.md` | Line 150 ("New gap registrations also need a row …") → "New findings registered for tracking also need a row in `suite-development/FINDINGS-INDEX.md` (forward-only section, identified by their `Review N Finding M` anchor — no new ID prefix; the legacy `G-` series is closed) …". |
| `hooks/check-suite-review-preamble.sh` | Docstring updated to name the Review 73 convention shift: the legacy `G-` series is closed; new findings identified by `Review N Finding M` anchor; the `### New gap registered` heading is RETIRED going forward (project-aligned `### Open` / `### Deferred` headings replace it); historical entries using the retired heading remain valid per G-89. Validation logic unchanged — the existing `**Finding N — Title**` + `**G-XX — Title**` heading-form acceptance already covers both new findings (former) and legacy-anchor walks (latter); Check 5 (`### New gap registered` enforcement) was already advisory-grade and remains so. No behavioral regression for legacy entries. |

**Forward-only constraint (G-89 precedent):** All historical G-IDs (G-01..G-182) remain valid as anchors throughout the suite — every cross-reference in CHANGELOG, COMPATIBILITY, prior review-log entries, primer prose, and domain prompts that names a `G-XX` continues to resolve. The legacy registry section in `FINDINGS-INDEX.md` is preserved untouched: same column shape, same row contents, same status conventions. Status updates to legacy findings continue in place — a long-Open `G-XX` closing in a future review still updates its row in the legacy section, not in the forward-only section.

**Cross-scope consistency goal achieved:** A suite contributor walking the suite-development `FINDINGS-INDEX.md` now encounters the same column shape, classification universe, source field, and anchor pattern as a suite user walking a project's `FINDINGS-INDEX.md`. The two registries differ in scope (suite-development tracks findings against the suite as software artifact; project tracks findings against a project) but share registry conventions — the operator-named "consistent and intuitive experience" outcome.

**Most-uncertain choice noted:** Keeping the legacy registry's `Type` and severity columns vs. retroactively reshaping them. Chose preservation per the operator's "Gaps do not need to be renamed retroactively" directive. A future contributor browsing the legacy section sees `| Type | Mission-Critical Severity | Speculative Severity |` columns that the forward-only section does not have; the difference is visible but acceptable as historical structure. If a future review prefers a cleaner unified view, the legacy section can stay closed-to-new-entries while the existing data shape evolves — but the operator's directive scopes that change out of the current pass.

**Resolution:** All 6 artifacts updated as enumerated. Forward-only constraint preserves every historical G-ID anchor across the suite. The forward-only **§ Findings registry** section in `FINDINGS-INDEX.md` is empty at convention-shift time (no Open or Deferred findings registered today via the new shape); the next suite review surfacing a tracked finding will add the first row.

### Summary

1 finding Resolved in-session. The convention shift is forward-only with full historical-anchor preservation per G-89. Backlog after Review 73: 0 Open + 6 Deferred (G-159, G-168, G-169, G-170, G-171, G-172 — unchanged; no findings closed or newly tracked this round beyond the convention shift itself, which is its own audit-trail anchor).

**Coordination:** none — the change is scoped to suite-development governance files + the two user-facing touch points (top-level README + primer 3) that mention the registry. Project FINDINGS-INDEX shape was already the canonical reference (bookmark-cli is the worked example) and is unchanged.

---

## Review 72 — 2026-05-20 10:15Z

**Scope:** Multi-artifact suite-development pass driven by operator-directed review of (a) the vsdd-suite README's Phase 5 / Phase 6 coverage and (b) suite-development governance documentation currency. Mid-session the operator promoted G-177 (Deferred) to Addressed via the explicit G-130 preemption mechanism, broadening the scope to retire the `PHASE-5-LOG.md` + `PHASE-6-CONVERGENCE.md` per-project artifact prescription across the suite + the bookmark-cli reference example. Artifacts read this round: `suite-development/suite-development.md` (governing standard, as session primer); `vsdd-suite/README.md` (Quickstart, per-layer flow diagram, Worked example, project-tree example, Merging gate, Running IAR sections); `primers/5-formal-hardening.md`; `primers/6-convergence.md`; `domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md` dim 13 + dim 14; `suite-development/FINDINGS-INDEX.md`; `suite-development/SUITE-DEVELOPMENT-REVIEW.md`; `suite-development/README.md`; `bookmark-cli/vsdd-suite/PHASE-5-LOG.md` + `DESIGN.md` + `src/lib.rs` + the SA and QE review log files + project CHANGELOG.

**Lens:** Multi-lens cross-artifact consistency + currency audit — SO (spec / methodology scope), SA (architecture / convention coherence applied to the suite itself), TW (documentation currency and drift), VDD-IAR Alignment (process compliance for the suite as artifact). Three coordinated artifact-state checks: Phase 5/6 integration coverage; legacy IAR-suite verbiage cleanup; G-177 operator promotion (per-domain log pattern roll-out).

**Session note:** In-session with the suite's authorial context — the same operator who promoted G-177 mid-session, directed the README review, and made the "stacked PRs are wrong; one PR at a time" workflow correction. Sycophancy compensation: every finding anchored to a specific file path and line range (grep-verified); the operator's directive that bookmark-cli is the reference example (so it migrates rather than gets a forward-only carve-out) was applied to remove the forward-only paragraphs I had initially written. Two course corrections during the session were applied immediately (PHASE-5-LOG retirement + reference-example migration framing) rather than deferred. Findings derived from artifact-state analysis (grep over PHASE-5-LOG / PHASE-6-CONVERGENCE / IAR-Suite / gap-analysis-run references), the user's specific directive prompts, and the governing standard's currency check.

**Source:** mixed — `director-raised` for the session-opening Phase 5/6 README review prompt, the G-177 operator-promotion message, the bookmark-cli reference-example migration directive, and the workflow directives ("log suite-development sessions automatically" + "one PR at a time, no stacked PRs"); `domain-raised` for the legacy-verbiage findings (TW lens) and the cross-artifact consistency findings (SA lens) the operator-directed review surfaced.

### Resolved

**Finding 1 — `PHASE-5-LOG.md` + `PHASE-6-CONVERGENCE.md` per-project files retired (G-177 operator-promoted from Deferred to Addressed)**

G-177 (Deferred since Review 67 with trigger "second project enters Phase 5 OR operator preemption") was operator-promoted to Addressed mid-session. The operator's directive: "PHASE-5-LOG.md + PHASE-6-CONVERGENCE.md should not exist; they violate conventions and are an anti-pattern." Resolution candidate (a) from G-177's row applied across the suite: retire the per-project files; Phase 5 findings file under per-domain review logs with `**Phase 5 surface:**` preamble tag; Phase 6 convergence record IS the final VDD-IAR Alignment review round.

**Resolution scope:**

| Artifact | Change |
|---|---|
| `primers/5-formal-hardening.md` § Phase 5 log format | Rewrote section: per-domain log pattern with per-surface→domain mapping (A / A.0 / D → SA; B / C → QE) + `**Phase 5 surface:**` preamble tag format. Surface C JS/TS distinction reworded to cite per-domain logs not PHASE-5-LOG.md. |
| `primers/5-formal-hardening.md` § Manual mode + § Completion criteria #2 | Updated to cite per-domain rounds with preamble tag instead of PHASE-5-LOG.md. |
| `primers/6-convergence.md` § Phase 6 convergence record format | Substantial rewrite: the convergence record IS the final VDD-IAR Alignment review round titled "Review N — Phase 6 four-dimensional convergence (project-terminal)" with the four-dimension attestations + cross-dimension consistency check + signed closing per the round entry format. |
| `primers/6-convergence.md` § Crosslink mode + § Manual mode + § Completion criteria + § Anonymization-aware attestation + § Layer reference + Dimension 2 verification step / disposition record + Dimension 4 signal | All references to `vsdd-suite/PHASE-6-CONVERGENCE.md` and `vsdd-suite/PHASE-5-LOG.md` rewritten to cite the per-domain log rounds (with `**Phase 5 surface:**` preamble) and the final VDD-IAR Alignment round respectively. |
| `domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md` dim 13 | Surface-activation check updated: evaluate per-domain rounds with the `**Phase 5 surface:**` preamble tag instead of PHASE-5-LOG.md per-layer entries. |
| `domains/meta/VDD-IAR-ALIGNMENT-REVIEW.md` dim 14 | Phase 6 convergence evaluation updated: evaluate the final VDD-IAR Alignment review round (the round titled "Review N — Phase 6 four-dimensional convergence (project-terminal)") instead of `vsdd-suite/PHASE-6-CONVERGENCE.md`. |
| `vsdd-suite/README.md` Quickstart steps 7 + 8 (both modes) | Phase 5 step describes per-surface session filing in the per-domain log; Phase 6 step describes the final VDD-IAR Alignment round. |
| `vsdd-suite/README.md` Worked example § Phase 5 + § Phase 6 walkthroughs (added in this session) | Authored from scratch using the per-domain log + final round pattern. |
| `bookmark-cli/vsdd-suite/PHASE-5-LOG.md` (reference example) | Deleted via `git rm`. The substantive content (purity-boundary audit + cargo-mutants outputs + per-mutant disposition table) was already present in the per-domain logs (`review-log/2026-05-20-solution-architect.md` Review 1 + `review-log/2026-05-20-quality-engineer.md` Review 2); PHASE-5-LOG.md was an index/coordination file the per-domain rounds duplicated. |
| `bookmark-cli/vsdd-suite/review-log/2026-05-20-solution-architect.md#review-1` | Added `**Phase 5 surface:** A.0 — purity-boundary verification for Layer 1` preamble tag; removed cross-references to `../PHASE-5-LOG.md` from Scope and Coordination lines. |
| `bookmark-cli/vsdd-suite/review-log/2026-05-20-quality-engineer.md#review-2` | Added `**Phase 5 surface:** B — mutation testing for Layer 1 via cargo-mutants` preamble tag; removed cross-references to `../PHASE-5-LOG.md` from Scope, unviable-mutants paragraph, and Coordination lines. |
| `bookmark-cli/vsdd-suite/QUALITY-ENGINEER-REVIEW.md` Reviews table | Row for Review 2 reworded to name the surface preamble explicitly + removed `../PHASE-5-LOG.md` citation. |
| `bookmark-cli/DESIGN.md` § Project intent Phase 5 strategy line + § Verification architecture Phase 5 bullet | Reworded to cite the per-domain logs instead of `PHASE-5-LOG.md`. |
| `bookmark-cli/src/lib.rs:148` doc comment | Updated to cite `vsdd-suite/QUALITY-ENGINEER-REVIEW.md` (Review 2 — Phase 5 Surface B) instead of `vsdd-suite/PHASE-5-LOG.md`. |
| `bookmark-cli/CHANGELOG.md` | New top entry documenting the v0.7.8 migration with the file delete + cross-reference update list + historical-narrative preservation note. |

**Forward-only narrative:** historical CHANGELOG / COMPATIBILITY / review-log entries that reference `PHASE-5-LOG.md` and `PHASE-6-CONVERGENCE.md` are preserved as audit-trail records per G-89. The CHANGELOG.md `## Unreleased — 2026-05-20 04:30Z (Review 68: ...)` entry mentioning `**G-177 (new)**` and the v0.7.0 / v0.7.1 / v0.7.3 COMPATIBILITY.md rows that reference the files reflect the state at the time of writing; the current state is described in this Review 72's CHANGELOG entry. PHASE-6-CONVERGENCE.md never existed on any project; no Phase 6 file deletion was needed.

**Classification:** Resolved.

**Finding 2 — README missing Phase 5 + Phase 6 operational integration (G-181)**

`vsdd-suite/README.md` had Phase 5 + Phase 6 named in `## Suite scope`, the `## VSDD pipeline context` table, and the `## Session primers` table — but no operational integration. Specifically:

- Both Quickstart sections (crosslink-primary and manual) stopped at Phase 4 / "Loop until MVR" with no step 7 (Phase 5) or step 8 (Phase 6).
- The Per-layer flow ASCII diagram (G-136 closure) ended at "Merge layer" — no Phase 5 box (per-layer, between Phase 3 MVR and merge) and no Phase 6 box (project-terminal).
- The Worked example walkthrough had `### Phase 1a+1b` through `### Phase 4 — Feedback Integration` + `### Loop until MVR` but no `### Phase 5 — Formal Hardening` or `### Phase 6 — Four-Dimensional Convergence` sections.
- The project-tree example listed per-domain index files but did not name the Phase 5 / Phase 6 artifacts the methodology produces (which, post-G-177, are per-domain rounds with `**Phase 5 surface:**` preamble + the final VDD-IAR Alignment round; no separate per-project files).

A new operator reading the README to learn the methodology would see Phase 5/6 named but have no operational guidance for executing them.

**Resolution:** added Phase 5 + Phase 6 steps to both Quickstart sections; extended Per-layer flow diagram with Phase 5 (conditional, between layer-gate close and merge) and Phase 6 (project-terminal after every layer's Phase 5); added `### Phase 5 — Formal Hardening` + `### Phase 6 — Four-Dimensional Convergence` walkthrough sections to the Worked example with `[crosslink]` + `[manual]` blocks (the four-surfaces table for Phase 5; the four-dimensions table for Phase 6). All new content reflects the post-G-177 per-domain log + VDD-IAR Alignment final round pattern. The project-tree example was intentionally not updated to add new per-project files — per G-177, those files are retired.

**Classification:** Resolved.

**Finding 3 — Legacy "IAR Suite" / "gap analysis" verbiage in suite-development governance files (G-182)**

Three suite-development files retained legacy IAR-suite / gap-analysis verbiage inconsistent with current VSDD Suite / Findings conventions:

- `suite-development/FINDINGS-INDEX.md:1` — H1 "# IAR Suite Gap Analysis Log" (file was renamed from `GAP-ANALYSIS-LOG.md` to `FINDINGS-INDEX.md` in v0.4.0 per G-149, but the H1 was not updated).
- `suite-development/FINDINGS-INDEX.md:3` — opening paragraph "This log tracks gap analysis runs against the IAR suite itself".
- `suite-development/FINDINGS-INDEX.md:11` — section header "## How to run a gap analysis" — the "gap analysis run" framing was retired by `suite-development.md:407`.
- `suite-development/SUITE-DEVELOPMENT-REVIEW.md:1` — H1 "# IAR Suite Review".
- `suite-development/SUITE-DEVELOPMENT-REVIEW.md:3` — "The IAR suite is itself a software artifact" + "gap analysis log".
- `suite-development/README.md:3` — "running gap analysis".
- `suite-development/README.md:60` — "Project IAR sessions sometimes produce findings".

A new contributor opening these files first would form a mental model out of date with the rest of the suite (where "VSDD Suite" is the current name and "suite review" is the unified session type per v0.4.0's mode-unification).

**Resolution:**

- FINDINGS-INDEX.md H1 → "# VSDD Suite Findings Index"; opening paragraph rewritten to "findings registry against the VSDD Suite itself"; § header "How to run a gap analysis" → "Adding and updating findings"; body rewritten to point at `suite-development.md` § Running gap analysis and § Suite review and review-log discipline as the canonical workflow source (single source of truth).
- SUITE-DEVELOPMENT-REVIEW.md H1 → "# VSDD Suite Review Index"; opening paragraph reworded to "The VSDD Suite is itself a software artifact" + "expanded beyond its original VSDD Phase 3 (IAR) scope to own every VSDD phase 1a+1b through 6".
- suite-development/README.md line 3 reworded to "registering and walking findings, logging suite reviews"; line 60 reworded to "Project-level review sessions sometimes produce findings whose substance generalizes."

Per G-89 narrative-preservation policy: "gap" remains valid as a concept-level term (the registry IS the gap registry; G-IDs identify gaps); "gap analysis run" specifically — the retired session-type framing — is replaced by "suite review" with the `Lens` field distinguishing modes (defect-search / registry-walk / role-based). Historical narrative in older review-log entries that uses "gap analysis run" prose remains as committed records.

**Classification:** Resolved.

### Coordination

This Review 72 entry registers and resolves three findings in-session (G-177 promoted from Deferred + G-181 + G-182). Cross-domain consequences:

- **G-177 closure ripples** to every project that may adopt Phase 5 or Phase 6 in the future (the per-domain log + VDD-IAR Alignment final round pattern is the active prescription). bookmark-cli (the reference example) is migrated in this session; no other project has reached Phase 5 yet, so no other project migrations are needed.
- **G-181 closure** depends on G-177's resolution (the README's Phase 5 + Phase 6 walkthroughs cite the post-G-177 per-domain pattern; if G-177 had been resolved with candidate (b) instead, the README content would have differed).
- **G-182 closure** is independent of G-177 / G-181 but ships in the same Review 72 because it surfaces from the same TW currency-audit lens.

**Operator workflow directives captured this session** (process feedback applicable to future suite-development sessions, saved as feedback memory at session close):

1. **Suite-development sessions should be logged proactively.** When the operator is doing suite-development work, the agent should be logging suite-review entries and registering findings as the session progresses — not waiting to bundle work at session end. This Review 72 entry started mid-session in response to the directive.
2. **No stacked PR pattern.** Reviews 70 + 71 were stacked PRs (#27 + #28) because they were authored as separate logical sessions that touched the same governance-file rows. Going forward: one PR at a time. This Review 72 ships as a single PR even though it folds in three findings (G-177 + G-181 + G-182) and a reference-example migration.

**Coordination with `bookmark-cli`:** the reference example's migration (PHASE-5-LOG.md deletion + per-domain round preamble tags + DESIGN.md / src/lib.rs / per-domain index updates) is part of this Review 72's scope rather than a separate bookmark-cli session because the migration IS the operational consequence of G-177's resolution at the suite scope. The bookmark-cli CHANGELOG entry cross-references this Review 72.

---

## Review 71 — 2026-05-20 09:15Z

**Scope:** Multi-artifact transition-progress assessment of the IAR-to-VSDD library expansion. Artifacts re-read in this session: `suite-development/suite-development.md` (governing standard); `primers/3-review-session.md` (Phase 3 adversarial review primer); `README.md` (full text, with attention to § Domains, § Quickstart, § Worked example, project-tree example at ~line 905, § Merging gate at ~line 951); `domains/DOMAIN-INDEX.md` (core/extended classification, intent calibration); `COMPATIBILITY.md` (full version history v0.1.0 → v0.7.6); `templates/README.md` (customization checklist); `suite-development/FINDINGS-INDEX.md` (full registry walk, 178 rows). Trigger: operator request for a transition-progress analysis across SO / SA / TW / UX / QE lenses.

**Lens:** Multi-lens transition-progress audit — SO (spec scope coverage), SA (architecture / classification coherence), TW (documentation drift / staleness), UX (developer-experience entry path), QE (suite-effectiveness instrumentation), VDD-IAR (process-compliance applied to the suite as artifact). Five lenses applied serially against the same artifact set to produce a comprehensive transition-completion picture.

**Session note:** In-session with the suite's authorial context (the same session that authored Review 70). Sycophancy compensation: each lens-finding was anchored to a specific file path and line range (grep-verified before recording); the analysis report disclosed both addressed and unaddressed gaps and named the open gaps that pre-date this session by months without re-litigating them as new findings. Findings derived from artifact-state analysis (grep over PE/DE/core-count refs, grep for "Merging gate" / "IAR" usage, file-by-file enumeration of customization checklists) rather than narrative judgment.

**Source:** domain-raised — multi-lens audit (SO / SA / TW / UX / QE) applied to the suite as artifact.

### Resolved

**Finding 1 — README § Merging gate stale relative to suite-development.md § Layer-gate close criteria (Dim 7 — TW / cross-artifact consistency) (G-179)**

`README.md` § Merging gate (prior lines 951–962) enumerated **6 layer-gate criteria**: (1) all active IAR domains have completed a run; (2) refinement loop ran to MVR; (3) every finding terminal; (4) accepted risks documented; (5) VDD-IAR Alignment run; (6) results logged with round numbers. `suite-development/suite-development.md` § Layer-gate close criteria has **7 baseline criteria** (the same 6 plus criterion 7: PROCESS.md retrospective with developer-voice prose as a hard gate, landed 2026-05-18 per G-156). The README's 6-criterion version was older and missing G-156's hard gate; the README also lacked the G-131/G-151 trigger-discipline framing the canonical version carries. A reader landing on the README's Merging gate first (the natural reading path for new adopters) would get a 6-criterion mental model that the canonical source has since superseded.

**Resolution:** replace the README's 6-criterion enumeration with a one-line pointer to the canonical 7-criteria set in `suite-development/suite-development.md` § Layer-gate close criteria. The replacement names criterion 7 (G-156 PROCESS.md retrospective) and the G-131/G-151 trigger discipline explicitly so a reader skimming the README's pointer understands what the canonical set adds. A two-sentence follow-up mentions the project-level `CLOSURE-PROTOCOL.md` precedent (ITC) — the canonical set is the baseline, and projects may add criteria but not weaken. Net change: −12 lines / +3 lines in `README.md`; criterion content lives in one place (suite-development.md) instead of two.

**Why a pointer rather than re-stating all 7:** the criterion set has evolved (6 → 7 via G-156) and will evolve again. Two sources of truth invite drift; one source plus a pointer eliminates the staleness vector. The README's `## Per-layer flow (within a project)` ASCII diagram (G-136) already references the canonical criteria from the diagram itself; this fix completes the single-source-of-truth pattern.

**Classification:** Resolved.

**Finding 2 — templates/README.md Customization checklist does not name DESIGN.md § Project intent declaration (Dim 1 — TW / spec completeness) (G-180)**

`templates/README.md` § Customization checklist enumerates 6 per-domain field substitutions (`{{ROLE_TITLE}}`, `{{ROLE_VARIANTS}}`, `{{PURPOSE}}`, etc.) and a closing paragraph each for `DESIGN.md` and the project `README.md`. The `DESIGN.md` paragraph names the primer to load (`primers/1ab-spec-crystallization.md`) but does not call out the **`§ Project intent` declaration** — the intent line is what gates the active-domain set, the stop-signal sensitivity, and (at capstone+ intent) the Phase 5 / Phase 6 strategy declarations. A first-time scaffolder following the checklist literally would customize the per-domain index files first, then write `DESIGN.md` from the skeleton, possibly without realizing the active-domain set the scaffold script picked should match the intent declared in `DESIGN.md`. The discoverability path is implicit (in the DESIGN-template.md skeleton itself) but the customization checklist is the first artifact the scaffolder reads — it should name the intent declaration explicitly.

**Resolution:** expand the `For DESIGN.md` paragraph in `templates/README.md` § Customization checklist into a 2-step ordered list: (1) work the driving questions in the primer (unchanged); (2) declare `§ Project intent` first, with a one-sentence rationale naming what the intent gates (active-domain set, stop-signal sensitivity, Phase 5/6 strategy declarations at capstone+) and a warning that the over-investment variant is hard to catch in-project. The fix lands in 4 lines of new prose with the cross-reference to `domains/DOMAIN-INDEX.md` § Intent calibration where the gating mechanism is documented.

**Classification:** Resolved.

### Dismissed

**Finding 3 — "IAR" terminology preserved in README (40 occurrences) and suite-development.md (19 occurrences) (Dim 6 — SA / naming consistency)**

The multi-lens audit surfaced that "IAR" still appears with high density across the user-facing surface — 40 occurrences in `README.md`, 19 in `suite-development/suite-development.md`. A cold reader landing on the README without context might read "IAR" as the suite's name rather than the Phase-3 component name. The transition-progress analysis flagged this as a potential drift signal.

**Classification:** Dismissed — intentional per the IAR-name-preservation policy stated explicitly in `suite-development/suite-development.md:11`: "the directory was renamed to `vsdd-suite/` in Review 38 (G-88 closure) to match the expanded scope; 'IAR' remains the name for the Phase 3 portion specifically and is preserved in historical project review logs that pre-date the rename per the forward-only constraint." The 40+19 occurrences are almost all contextually correct (referring to Phase 3 component, the VDD-IAR Alignment meta domain, legacy project paths, or forward-only narrative records). Mass-renaming "IAR" → "Phase 3" or similar would conflict with the explicit policy and would also break legacy project review log cross-references. The name-preservation is doing what the policy says it does.

**A one-sentence inline gloss in the README lead paragraph** ("IAR = Iterative Adversarial Refinement, the Phase 3 component of VSDD") was considered as a less-invasive alternative but rejected as redundant — the README's first sentence already names "Phase 3 (Iterative Adversarial Refinement — IAR)" and the Suite scope section reinforces it.

### Coordination

This Review 71 entry catalogues findings derived from a multi-lens transition-progress audit. The audit re-confirmed the status of **15 long-Open or Deferred gaps** without re-litigating them as new findings — the registry-walk classification universe explicitly authorizes this carry-over reading:

- **Open speculative-project / consulting-scope gaps** (G-01 Compliance and Legal; G-04 Operational Readiness; G-05 Delivery Governance; G-11 SO budget tracking; G-13 PE DR with RTO/RPO; G-14 learning goals; G-15 kill criteria; G-16 intentional tech debt; G-17 SA pivot readiness; G-18 Requirements/BA; G-26 Change Management; G-28 Client/Stakeholder Alignment; G-29 Discovery research quality; G-31 Engagement liability) — these are open by deliberate scope; the suite is a portfolio/apprenticeship tool, not a consulting or production-ops platform. Status unchanged. Reactivation trigger: if the suite's scope expands to consulting or speculative R&D contexts, the bundle becomes eligible.
- **G-57** (no effectiveness test for domain prompts) — long-Open since 2026-04-27; the only foundational QE-lens gap. The audit flagged it as the most-tractable next arc; status unchanged this session but elevated visibility for future selection.
- **Deferred (substantive)** — G-99 (warm-finding-closure Red Gate carve-out); G-135 (AI Engineering / cost-engineering meta-domain); G-159 (knowledge-page versioning); G-168, G-169 (suite-side gaps from Review 63); G-170, G-171, G-172 (Phase 6 refinement gaps from Review 65); G-177 (PHASE-5-LOG.md duplication from Review 67). All have named triggers + auto-Backlog dates per G-130; the audit confirmed their trigger conditions remain unfired and the auto-Backlog dates are still future. Status unchanged.

The audit also confirmed **Review 70 resolved G-178** (core-domain count inconsistency) — that finding's narrative is in Review 70's entry below, not duplicated here.

**Coordination:** **G-179** and **G-180** registered as new gaps in [`../FINDINGS-INDEX.md`](../FINDINGS-INDEX.md) and resolved in-session this round. The fixes were intra-artifact (single section in `README.md`; single section in `templates/README.md`) with no cross-domain implications. No project-level review logs are affected by Review 71's edits. The audit-derived inventory of long-Open gaps is informational; no auto-Backlog triggers fired this round.

---

## Review 70 — 2026-05-20 08:30Z

**Scope:** `domains/DOMAIN-INDEX.md` (core/extended classification tables and intent calibration table); `README.md` (Domains section core/extended tables and project-tree example); `templates/scaffold-project.sh` (header comments and default-domain list); `suite-development/FINDINGS-INDEX.md` (gap row addition); `CHANGELOG.md` (release entry); `COMPATIBILITY.md` (version row); `SUITE-DEVELOPMENT-REVIEW.md` (index row). Trigger: operator-raised three-way inconsistency surfaced during a transition-progress review of the IAR→VSDD library expansion ("the analysis identified PE in capstone calibration as 'All 7 core + Performance Engineer' — but PE is already in the core 8 per the DOMAIN-INDEX table; that's mathematically incoherent if PE is in the core 8").

**Lens:** Cross-artifact consistency — applied specifically to the core-domain count and PE/DE classification across all suite artifacts where the count is named.

**Session note:** In-session with the suite's authorial context (operator-driven structural change session, not a cold review). Sycophancy compensation: the reclassification direction was selected by the operator via an explicit AskUserQuestion with three options (demote, promote, or third-tier), each with a preview showing the resulting taxonomy; the agent's framing of the recommendation was disclosed and the operator chose Option A independently. Findings derived from artifact-state analysis (grep over every PE/DE/core-count reference in README, DOMAIN-INDEX, scaffold script, and templates) rather than narrative judgment.

**Source:** domain-raised — Solution Architect lens on the suite (classification scheme coherence is an SA dim 4 concern: data model integrity applied to the domain taxonomy itself).

### Resolved

**Finding 1 — Core-domain count inconsistency between DOMAIN-INDEX.md and README.md (Dim 4 — applied to suite taxonomy) (G-178)**

`domains/DOMAIN-INDEX.md` § Core domains opened with "These eight domains apply to all projects regardless of type, deployment context, or scale" and listed eight role domains in the core table (SE, QE, UX, Security, PE, SA, SO, DE). The same file's § Intent calibration table treated the count as seven ("All 7 core" for portfolio; "All 7 core + Performance Engineer" for capstone — incoherent if PE was already inside the 7). The `templates/scaffold-project.sh` script defaulted to seven (six role + VDD-IAR-Alignment meta, excluding PE+DE). `README.md` § Domains and Quickstart consistently said "7 core domains" and the worked example said "(7 core domains, no PE/DE/extended)". Three different mental models existed in parallel:

- DOMAIN-INDEX table: 8 core role
- DOMAIN-INDEX intent calibration: 7 (ambiguous about which)
- README + scaffold + worked example: 7 = 6 role + 1 meta

A new contributor or AI agent loading any one of these as authoritative would produce drift in the other two.

**Resolution:** demote Platform Engineer and Data Engineer from core role to extended-with-strong-presumption (operator selection from a three-option AskUserQuestion: A demote, B promote scaffold to 9, C add a third tier). Edits applied:

1. **`domains/DOMAIN-INDEX.md` § Core domains** — intro rewritten from "These eight domains apply to all projects" to "Six core role domains plus the VDD-IAR Alignment meta domain (seven total) apply to all projects." PE and DE rows removed from the core role table; a paragraph naming the seventh-core-is-VDD-IAR-Alignment meta domain was added. New forward-only-constraint paragraph cites the v0.7.6 cutoff date and the G-178 row for the reclassification's authority.
2. **`domains/DOMAIN-INDEX.md` § Extended domains** — PE and DE rows added at the top of the extended table with named activation criteria (PE: managed pipeline / infrastructure / observability hooks / any operational deployment surface beyond local-toolchain install; DE: persistent data through DB / managed schema / structured-storage integrity / external data systems). A new paragraph above the table establishes the "extended-with-strong-presumption" framing — both domains typically activate beyond local-toolchain CLI scope and are strongly presumed at capstone and production intent.
3. **`domains/DOMAIN-INDEX.md` § Intent calibration** — learning-exercise row reframed: SE+QE+SO+VDD-IAR Alignment as the four fixed cores plus one rotating fourth role drawn from {SA, Security, UX} (PE+DE removed from the rotation pool since they're now extended). Portfolio / capstone / production rows clarified to name PE+DE per their activation criteria; capstone and production now make explicit that PE+DE are typically active at those intents.
4. **`README.md` § Domains** — PE row and DE row moved from the Core role table to the top of the Extended role table. The lead paragraph "Default activation for new projects is the 7 core role domains plus VDD-IAR Alignment" reworded to "the 7 core domains — six core role domains (SE, QE, UX, Security, SA, SO) plus the VDD-IAR Alignment meta domain" — eliminates the "(7 role) + (1 meta) = 7?" arithmetic ambiguity. A new sentence under the core table names VDD-IAR Alignment as the seventh core domain (listed in the meta table). The extended table opens with the "extended-with-strong-presumption" framing for PE+DE.
5. **`README.md` project-tree example (~line 905)** — comment block reorganized: PE and DE moved from "# Core domains (always active)" to "# Extended domains (include only those active on the project; PE + DE are extended-with-strong-presumption per G-178 and typically active beyond local-toolchain CLI scope)".
6. **`templates/scaffold-project.sh`** — header comment block, `DEFAULT_DOMAINS` array comment, and the no-args echo block reworded from "core but conditional" to "extended-with-strong-presumption (G-178)". Script behavior unchanged (already defaulted to 7 since v0.3.0).

**Forward-only constraint (G-89 precedent):** projects whose first IAR run predates v0.7.6 (today, 2026-05-20) retain PE/DE-as-core in their existing review logs, DESIGN.md notes, and per-domain review-log files. The reclassification does not invalidate prior records. New projects scaffolded at v0.7.6+ follow the new classification automatically.

**Why this is non-breaking against COMPATIBILITY.md:** the PE and DE prompt files are unchanged (same dimensions, same sycophancy check, same finding classification schema). The classification (core vs. extended) is a metadata field about the domain, not a content field. Existing review logs that reference PE/DE remain syntactically valid against the suite's governing standard. The only behavioral change is in the scaffold-default activation set — which is already what the scaffold script does in practice.

**Why a third tier was rejected (Sycophancy self-audit):** the agent's initial framing in the analysis recommended Option A and previewed the result; the operator selected Option A. The third-tier option (Option C: "core-but-conditional") was rejected for a substantive reason: it would preserve the "core" label for PE+DE but require a new taxonomic concept to explain the difference between "always-core" and "core-presumed-with-scope-down". The operating reality already maps cleanly to a two-tier taxonomy; the third tier would be defending the prior label rather than the prior practice. (The README and scaffold script were always operating Option A semantics; only the DOMAIN-INDEX header was operating "core" semantics.) Per the "earned by recurrence" doctrine, taxonomic weight is added when a defect class recurs that the existing taxonomy can't catch — not when an existing taxonomy can be reorganized to match practice.

**Classification:** Resolved.

### Coordination

Edits propagated mechanically across all artifacts where the prior counts appeared:

- `domains/DOMAIN-INDEX.md` — primary canonical edit (core + extended tables + intent calibration)
- `README.md` — Domains section + project-tree example
- `templates/scaffold-project.sh` — header + comment block (no behavior change)
- `suite-development/FINDINGS-INDEX.md` — G-178 row added with full resolution narrative
- `CHANGELOG.md` — v0.7.6 entry added (additive non-breaking reclassification per COMPATIBILITY.md § Breaking change definition)
- `COMPATIBILITY.md` — v0.7.6 version row added
- `suite-development/SUITE-DEVELOPMENT-REVIEW.md` — Review 70 row added at top of Suite Reviews table

Coordinate with **G-121** (scaffold-default ratification — Review 42's Solution Owner ratification of the 7-core scaffold default; that ratification was the operating-reality precedent the reclassification now matches). Coordinate with **G-150** (intent calibration — already operating with 7 core + extensions; this reclassification removes the count ambiguity in that table). Coordinate with **G-89** (forward-only narrative-preservation policy — the v0.7.6 cutoff applies the same forward-only mechanism the prior structural changes used).
