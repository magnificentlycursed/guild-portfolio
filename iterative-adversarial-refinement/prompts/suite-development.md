# Session Primer: Suite Development

Use this prompt at the start of any session whose purpose is developing the IAR suite itself — adding or modifying domain files, updating dimensions, running gap analysis, or revising primers. Do not use this for reviewing projects under the suite; use `prompts/review-session.md` for that.

---

## Prompt

You are helping develop the Iterative Adversarial Refinement (IAR) suite. The suite is itself a software artifact: it has a specification (the VSDD and VDD methodology documents), a design (the domain structure, dimensions, supplement architecture, and session primers), and an implementation (the domain prompt files, README, lang/ supplements, gap analysis log, and DOMAIN-INDEX).

**Apply the same adversarial standard to the suite that the suite applies to projects.** A dimension that sounds rigorous but would not catch a real defect is a quality failure. A sycophancy check copied unchanged across all domains reduces to boilerplate — it is not designed for the domain's specific failure modes. A new domain that overlaps substantially with an existing one has not been scoped correctly.

The adversary's question for every proposed change: **what defect would ship to a user if this change were not made?**

---

## Suite structure

The suite currently contains these artifact types:

| Artifact | Location | Purpose |
|---|---|---|
| Domain files | `domains/role/` and `domains/meta/` | Review prompts and evaluation dimensions per role |
| Lang supplements | `lang/` | Language- and interface-type-specific dimensions applied alongside domain reviews |
| Session primers | `prompts/` | Posture-setting prompts loaded at the start of a phase or session type |
| Domain index | `domains/role/DOMAIN-INDEX.md` | Authoritative core/extended classification with activation criteria |
| README | `README.md` | Suite entry point: domain tables, primer table, running instructions, supplement table |
| Gap analysis log | `GAP-ANALYSIS-LOG.md` | Running registry of identified suite gaps and their status |
| Suite review log | `SUITE-REVIEW.md` | Adversarial review runs of the suite itself |
| Changelog | `CHANGELOG.md` | Record of all non-trivial changes to suite artifacts |

---

## Governing standard for domain files

A complete domain file contains these elements, in order:

1. **H1 title** — `# [Role] Review`
2. **Suite membership line** — standard text linking to `../../README.md` and noting the domain may run independently
3. **Reviewer role line** — exactly one `**Reviewer role: [Title]** ([Job title variants])` line. Not zero, not two. **Exception:** meta domains (`domains/meta/`) are exempt — they have no job role persona by design. The README explicitly notes this distinction. A meta domain with a reviewer role line is wrong; a meta domain without one is correct.
4. **Purpose statement** — one paragraph. Answers: what does this role evaluate? What failure mode does it own that no other domain owns?
5. **`## Current Review Prompt`** section containing, in order:
   - Scope statement
   - Instruction to read DESIGN.md first
   - Finding classification schema with all valid classifications for this domain
   - Regression check
   - Coordination links (named, linked, relative paths within the same `domains/role/` folder)
   - Sycophancy check (domain-specific failure modes named — not boilerplate)
   - Lang supplement reference (`../../lang/`) — **required**, or an explicit opt-out line with rationale (e.g., `**Language and interface supplement:** Not applicable. [Reason].`)
6. **Domain-specific structural sections** — some domains have additional sections between the prompt and the dimensions. These fall into two categories:
   - **Required sections** — must be completed before the dimensions apply; their output is prerequisite to the review, not a classified finding. Example: `SECURITY-REVIEW.md` `## Threat Model` — the reviewer must name threat actors, crown jewel, and entry points before reading source files. Required sections must state: what the reviewer must produce, and how that output is logged (as a preamble record, not a resolved/dismissed/hallucinated finding).
   - **Optional extended sections** — conditional sub-dimensions that apply to specific project types. Example: `SE` `### Extended: Documentation`, `SA` `### Extended: External Interface Contracts`. Optional sections state the conditions under which they apply.
7. **`## Standard Evaluation Dimensions`** — numbered list, each dimension:
   - Named in bold (the failure class, not a question)
   - Explains why it matters
   - Names specific failure modes or named attacks
   - Optional extended sub-sections for conditional concerns (e.g., `### Extended: Documentation` in SE, `### Extended: External Interface Contracts` in SA)
8. **Log pointer** — final line: `Review entries are logged in \`iterative-adversarial-refinement/[FILENAME].md\` inside the project being reviewed.`

**What a weak dimension looks like:** "Are tests present and meaningful?" — a question, not a dimension; names no failure class; gives the reviewer no technique to apply.

**What a strong dimension looks like:** `**Test falsifiability** — Would each test catch a broken implementation? Named attacks: mutation testing. Named tools: Stryker (JS/TS), mutmut (Python), cargo-mutants (Rust).` — failure class named, why it matters explained, specific technique given.

**Finding classification schemas by domain type:**

- Most role domains: `resolved`, `deferred`, `dismissed`, `hallucinated`
- Security and Red Team: `resolved`, `accepted risk`, `dismissed`, `hallucinated` (no `deferred` — security findings are not deferred)
- Solution Owner: `resolved`, `backlogged`, `dismissed`, `hallucinated` (plus `approved deviation`)
- Accessibility: `resolved`, `deferred`, `dismissed`, `accepted deviation`, `hallucinated`
- Performance Engineer: `resolved`, `deferred`, `dismissed`, `accepted limitation`, `hallucinated`
- Privacy: `resolved`, `deferred`, `dismissed`, `accepted risk`, `hallucinated` (accepted risk requires explicit rationale and owner — "we don't have users yet" is not rationale)
- Localization: `resolved`, `deferred`, `dismissed`, `accepted scope`, `hallucinated` (accepted scope requires explicit documentation in DESIGN.md; silence is not acceptance)
- VDD-IAR Alignment (meta): `resolved`, `dismissed`, `hallucinated` (no `deferred` — process findings are binary)
- Portfolio Assessment (meta): `demonstrated`, `partial`, `absent`, `hallucinated` (no `resolved` — portfolio evidence is assessed, not fixed during the review)

**Cross-cutting classification — `raised to SO`:** Any role domain that finds a defect requiring a `DESIGN.md` change must classify it `raised to SO` rather than applying the change directly (only Solution Owner has DESIGN.md change authority). The classification is a valid sub-heading (`### Raised to SO`) for any non-meta role domain log; the body must include the proposed change, rationale, and a Markdown link to the SO log entry where the resolution will be tracked.

Verify the classification schema matches the domain's nature before finalizing a new domain file.

---

## Governing standard for session primers

A complete session primer contains:

1. **H1 title** — `# Session Primer: [Phase or Session Type] (VSDD Phase N)` where applicable
2. **Usage instructions** — when to use this primer, what it produces, what must exist before using it
3. **`---` separator**
4. **`## Prompt`** section — the text pasted into a fresh AI session. Must establish: the AI's role, the governing constraint (what binds the session), and the primary failure mode to watch for (the equivalent of a sycophancy check)
5. **Phase-specific sections** — driving questions, structure templates, completion criteria, checklists as appropriate to the session type. Each section should answer: what does the AI do with this? What does a correct output look like?

Primers are not domain files. They do not have dimensions, coordination links, or log pointers. They have posture and process.

---

## Governing standard for project-level review logs

A project-level review log is the file produced by running a domain review on a project under review (e.g., `issue-tracker-cli/iterative-adversarial-refinement/QUALITY-ENGINEER-REVIEW.md`). The domain prompt file specifies *what* to evaluate; this standard specifies *what the resulting log must contain*. Drift in log structure makes cross-domain reading harder and hides governance gaps — apply this standard whenever a new project-level log is created or an existing one is updated.

### File-level header (above the first review)

A complete project-level review log opens with these elements, in order:

1. **H1 title** — `# [Role] Review Log`
2. **Suite link line** — standard text linking to the project's `README.md` and noting the review is part of the IAR suite
3. **Purpose statement** — one paragraph stating what this domain evaluates in this project
4. **Reviewer role line** — `**Reviewer role: [Title]** ([Job title variants])` — required for role domains, copied verbatim from the domain prompt file. **Exception:** meta domains are exempt — parallel to the prompt-file rule.
5. **Activation line** — `**Activation:** [conditions and rationale]` — required for **extended** domains active on this project; omitted for core domains and meta domains
6. **Language supplement applied** — required when the domain prompt file references a lang supplement; format: `**Language supplement applied:** \`lang/<file>.md\` ([Section name]).` Domains marked language-agnostic in the prompt file include an explicit opt-out: `**Language supplement applied:** Not applicable. [Reason].`
7. **Sycophancy check** — required; one paragraph restating the domain-specific failure mode the reviewer must resist. Drawn from the domain prompt file's sycophancy check, not paraphrased into generic warnings.

### Per-review entry preamble (under each `## Review N — YYYY-MM-DD HH:MMZ`)

**Required for all domains:**
- **Scope:** what artifacts were reviewed in this round
- **Session note:** session-isolation status (cold session vs. in-session, with explicit acknowledgement of the quality tradeoff when in-session)

**Optional, only when applicable to the domain:**
- **Posture:** adversarial framing (Red Team)
- **Program phase:** apprentice phase context (VDD-IAR Alignment)
- **Reference:** non-DESIGN.md authoritative source the review evaluates against (Solution Owner reviewing against the assignment brief)
- **Regression check:** prior-review verification (any domain when a prior review for the same scope exists)
- **Assumption surfacing:** dependency and library-API verification (Quality Engineer, per the QE prompt's G-20/G-21/G-23 obligations) — one short paragraph per review naming assumptions verified or flagged

A reviewer who finds they need a preamble field that is not in either list should propose adding it to this standard rather than introducing it ad-hoc. Examples of fields that are **not** valid additions: `Preamble`, `Governing methodology`, `Mutation analysis method`, free-form `Test count` lines — these duplicate `Scope` or `Session note`, or belong inside individual findings or the closing summary.

### Required pre-review sections

Some domains require an output that must be produced before findings can be classified. These match the "Required sections" carve-out in the domain-file governing standard. The output is a preamble record, not a classified finding.

| Domain | Required section | Source |
|---|---|---|
| Security | `### Threat Model` | `SECURITY-REVIEW.md` prompt |
| Solution Owner | `### Compliance Table` | `SOLUTION-OWNER-REVIEW.md` prompt (assignment compliance audit) |

When a domain prompt file specifies a required section, the project-level review log must contain that section above the finding sections, populated for the round being logged. An absent required section is itself a finding for VDD-IAR Alignment dim 1.

### Finding sections

Group findings by classification heading. Use only the classifications listed for this domain in the **Finding classification schemas by domain type** table elsewhere in this document — no ad-hoc variants (`### Dismissed with Rationale`, `### Observation`, `### Note`, `### Raised to SO`). The classification universe per domain is the source of truth; the log structure must mirror it.

Empty classification sections use a `*(none)*` placeholder so the structure is visible:

```
### Open

*(none)*
```

A round that closes with no `### Open` section is a structural error — the absence of open findings is itself a state worth recording. The set of section headings should equal the domain's full classification set, with `*(none)*` used wherever empty.

**Exception — Portfolio Assessment:** Portfolio Assessment groups by dimension (`### Dim N — Name`), not by classification, because each portfolio dimension produces a per-dim assessment (`Demonstrated`/`Partial`/`Absent`/`Hallucinated`) rather than a defect to fix. The classification appears at the end of each dim section and a summary table appears at the close. Portfolio also adds a file-level `**Developer participation note:**` directly under the sycophancy check, naming which dimensions require direct developer interrogation rather than artifact analysis. These exceptions are intentional and limited to Portfolio Assessment; no other domain may use dim-first organization or the participation-note field.

### Finding body

Each finding follows this structure:

```
**Finding N — Title (Dim X)**

[Prose body — what was observed, why it matters, evidence]

**Resolution:** [for Resolved findings — what was changed and where]

— or —

**Classification:** [Dismissed | Deferred | Hallucinated | Accepted Risk | …]
[rationale; for Accepted Risk and similar, include the named owner]
```

- Finding title always includes the dim reference parenthetically (`(Dim 2)`, `(Dim 1, Dim 10)`, `(Rust supplement — path traversal)`)
- Numbering is continuous within a Review (1, 2, 3, … across all classifications), not restarted per classification
- Cross-references to other domain logs use Markdown links: `[QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md) Finding 4` — not prose ("Logged in QE log")
- Closer is exactly one of `**Resolution:**` (Resolved only) or `**Classification:**` (everything else). Mixing the two within a single domain's log is drift.

### Closing block

Each review ends with:

1. **`### Summary`** — one short paragraph: tally of findings by class and what the round produced. Required.
2. **`**Coordination:**`** line — handoffs to other domain logs with linked relative paths to the receiving file. Required when any finding references another domain; `*(none)*` otherwise.

Reviews without a Summary are incomplete. Reviews with cross-domain implications and no Coordination line have an unrecorded handoff.

### Round numbering and ordering

- Reviews are numbered per domain (Review 1, Review 2, …) and ordered chronologically with most recent at the bottom
- Each `## Review N` is separated from the next by a double horizontal rule (`---\n\n---`)
- Round numbers are required in every entry (`review-session.md` reinforces this for the merge gate)

---

## Before adding a dimension

1. Name the failure class: what defect reaches users if this dimension is absent?
2. Check whether an existing dimension in any domain already owns this failure class. If it does, flag the gap there rather than adding cross-domain overlap.
3. Check the gap registry (`GAP-ANALYSIS-LOG.md`). If the gap is tracked, add the gap ID to the CHANGELOG entry for this change. If it is not tracked, add it and immediately mark it Addressed.
4. Write the dimension using the standard form: failure class in bold, explanation, named failure modes or attacks.

## Before adding a domain

1. Name the defect class the domain would catch that no existing domain catches.
2. Evaluate the role-based taxonomy: what job title does this reviewer hold? Is it a job a real person would have at a real company?
3. Decide: core domain (every project) or extended domain (conditional on project type)? Document the activation criteria. Add the domain to `domains/role/DOMAIN-INDEX.md`.
4. Create the domain file following the governing standard above. Verify the finding classification schema is appropriate for the domain's nature.
5. Add the domain to `README.md` in the appropriate table (Core or Extended), with Role, Job title, Prompt file, and Focus columns filled.
6. Add lang supplement sections where applicable. See **Lang supplement coverage** below.
7. Add a suite review session entry in `review-log/YYYY-MM-DD-suite-review.md` (creating the file if no entry exists for that date) documenting the addition, rationale, defect class addressed, and dimensions that were considered and rejected. Add a corresponding row to the **Suite Reviews** table in `SUITE-REVIEW.md` linking to the entry.
8. Add a `CHANGELOG.md` entry.
9. Add a `GAP-ANALYSIS-LOG.md` entry if the domain addresses an existing open gap.

## Before modifying a domain

1. State what defect the current version fails to catch — name a specific scenario where the current prompt produces a false pass.
2. Make the change.
3. Update the gap registry: if the gap was tracked, mark it Addressed with today's date. If it was not tracked, add it and immediately mark it Addressed.
4. Log the change in `CHANGELOG.md`.
5. If the change is structural (new section, new classification schema, changed prompt format): add a suite review session entry in `review-log/YYYY-MM-DD-suite-review.md` and a corresponding row in the **Suite Reviews** table in `SUITE-REVIEW.md`.

## Running gap analysis

Read `GAP-ANALYSIS-LOG.md` for the current open gaps, then read all domain files and evaluate whether each open gap has been addressed by recent changes. Follow the instructions at the top of that file.

**Gap registry discipline:** When a gap is resolved by a suite change, update the original row's status in place — change `Open` to `Addressed` and update the `Last Reviewed` date. Do not append a new row for an existing gap. New gaps get new rows; status changes update existing rows.

A gap analysis session is one mode of suite review (registry-walk lens). Like any suite review session, it ends with:
- All recently addressed gaps marked Addressed in `GAP-ANALYSIS-LOG.md` with the date
- Any new gaps discovered added to the registry with a new G-ID
- A `## Review N — date` entry in `review-log/YYYY-MM-DD-suite-review.md` (creating the file if no entry exists for that date) summarizing scope, findings, decisions, and suite changes made
- A corresponding row added to the **Suite Reviews** table in `SUITE-REVIEW.md` linking to the new entry

`GAP-ANALYSIS-LOG.md` contains only the registry table. Run narratives belong in `review-log/`. `SUITE-REVIEW.md` is the index of those narratives, not their home.

## Suite review and review-log discipline

The IAR suite has three parallel review-record artifacts. Their roles do not overlap:

- **`SUITE-REVIEW.md`** is an index. It contains one table — **Suite Reviews** — each row pointing to a session entry in `review-log/`.
- **`review-log/YYYY-MM-DD-suite-review.md`** holds the actual session entries. One file per date; multiple sessions on the same date append to the same file (newest at the top).
- **`GAP-ANALYSIS-LOG.md`** is the gap registry. Status only — no narrative. One row per gap; status changes update the row in place.

Every non-trivial suite change requires a session entry in `review-log/` and a corresponding index row in `SUITE-REVIEW.md`. Non-trivial means: any addition or removal of a domain or primer, any new evaluation dimension, any structural change to the prompt format, or any change to sequencing or activation guidance.

Mechanical fixes (typos, filename renames, path updates) do not require a session entry but should be logged in `CHANGELOG.md`.

**One artifact type, multiple modes.** A suite review may apply fresh adversarial pressure (defect-search lens), walk the gap registry top-down (registry-walk lens), or both. The mode lives in the **Lens** field; it is not a separate artifact type. Sessions previously called "meta-reviews" and "gap analysis runs" are now both `Review N` entries — the distinction is mode, not kind.

### Filename convention

The filename date is the **session start date in UTC**. When a session crosses midnight UTC, it remains in the file matching its start date — do not split it across two files. Same-date sessions append to the existing file (newest at the top).

### Suite review entry format

A `## Review N — date` entry in `review-log/YYYY-MM-DD-suite-review.md` must contain:

1. **Header** — `## Review N — YYYY-MM-DD HH:MMZ`. Review numbers are **sequence-wide across all suite-review files** (Review 30 follows Review 29 even if they live in different date-named files); the timestamp is the session start in UTC.
2. **Scope** — What artifacts were read this round (specific domain files, primers, supplements, README, gap registry rows, etc.) and what triggered the review (user request, follow-up to a prior finding, scheduled cadence, project type added). Cite specific files when narrow; "all 14 role domains, 2 meta domains, 5 primers" when broad.
3. **Lens** — The angle the reviewer applied. A lens is a named defect class ("coordination link format compliance", "classification schema coverage", "lang supplement symmetry") OR a registry-walk scope ("walk all open gaps", "review G-22 and G-30"). A diffuse lens produces a diffuse review. If a session has no specific lens, log it as a generalist pass and name the prior passes' specialization gaps it is filling.
4. **Findings** grouped by classification heading. Valid headings:
   - `### Resolved` — fix applied and verified during the session. Use both for newly-found defects fixed in-session and for previously-tracked gaps closed in-session (cite the G-ID).
   - `### Dismissed` — concern reviewed and rejected; rationale required. Use both for newly-raised defects rejected and for previously-tracked gaps dismissed.
   - `### Hallucinated` — adversary-invented concern that does not apply; rationale required.
   - `### New gap registered` — finding promoted to a tracked gap; G-ID stated; the registry row is added in `GAP-ANALYSIS-LOG.md`. This heading is **suite-review-specific** — it is not part of the project-level classification universe and is not valid in project-level review logs.
5. **Finding body** — same shape as project-level review logs: `**Finding N — Title**` for new findings, `**G-XX — Title**` for gap-registry walk entries; prose body; then `**Resolution:**` (Resolved) or `**Classification:**` (everything else). Cross-references to other suite artifacts use Markdown links.
6. **Closing** — no separate Summary required (the classification headings carry the tally). Follow-up findings introduced after the session has been logged must be marked `**Finding M — Title (added YYYY-MM-DD)**` and placed at the end of the original entry, not in a new entry. Do not silently amend prior findings.

### Common discipline

The session entry is the narrative record. The `GAP-ANALYSIS-LOG.md` row is the status indicator for gaps. The `SUITE-REVIEW.md` row is the index pointer for the session. Never put narrative in the registry; never omit the registry update; never omit the index row. An unindexed session is invisible to future reviewers.

## Lang supplement coverage

Supplements provide language-specific failure modes. Not every domain has language-specific concerns — process-compliance and portfolio domains (VDD-IAR Alignment, Portfolio Assessment, Solution Owner) are language-agnostic.

**Current supplement sections by domain:**

| Domain | `javascript-typescript.md` | `rust.md` | Notes |
|---|---|---|---|
| Quality Engineer | ✓ | ✓ | |
| Security | ✓ | ✓ | |
| Software Engineer | ✓ | ✓ | |
| Platform Engineer | ✓ | ✓ | |
| Data Engineer | ✓ | ✓ | |
| Solution Architect | ✓ | ✓ | |
| Red Team | ✓ | ✓ | |
| Performance Engineer | ✓ | ✓ | |
| UX | — | — | Covered by `lang/cli.md` (CLI) and `lang/browser-app.md` (browser) |
| Technical Writer | ✓ | ✓ | |
| Accessibility | — | — | Covered by `lang/browser-app.md` |
| Privacy | — | — | Language-agnostic |
| Localization | ✓ | ✓ | |
| Solution Owner | — | — | Language-agnostic |
| VDD-IAR Alignment | — | — | Language-agnostic (explicitly noted in domain file) |
| Portfolio Assessment | — | — | Language-agnostic |

Before closing any suite development session, verify this table is still accurate. If a domain is added or a supplement section is added, update this table and the gap registry.
