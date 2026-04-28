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
7. Add a `SUITE-REVIEW.md` entry documenting the addition, rationale, defect class addressed, and dimensions that were considered and rejected.
8. Add a `CHANGELOG.md` entry.
9. Add a `GAP-ANALYSIS-LOG.md` entry if the domain addresses an existing open gap.

## Before modifying a domain

1. State what defect the current version fails to catch — name a specific scenario where the current prompt produces a false pass.
2. Make the change.
3. Update the gap registry: if the gap was tracked, mark it Addressed with today's date. If it was not tracked, add it and immediately mark it Addressed.
4. Log the change in `CHANGELOG.md`.
5. If the change is structural (new section, new classification schema, changed prompt format): add a `SUITE-REVIEW.md` entry.

## Running gap analysis

Read `GAP-ANALYSIS-LOG.md` for the current open gaps, then read all domain files and evaluate whether each open gap has been addressed by recent changes. Follow the instructions at the top of that file.

**Gap registry discipline:** When a gap is resolved by a suite change, update the original row's status in place — change `Open` to `Addressed` and update the `Last Reviewed` date. Do not append a new row for an existing gap. New gaps get new rows; status changes update existing rows.

A gap analysis session ends with:
- All recently addressed gaps marked Addressed with the date
- Any new gaps discovered added to the registry with a new G-ID
- A new `## Gap Analysis Run N — date` entry in `SUITE-REVIEW.md` (not in `GAP-ANALYSIS-LOG.md`) summarizing context, findings, decisions, and suite changes made

`GAP-ANALYSIS-LOG.md` contains only the registry table. Run narratives belong in `SUITE-REVIEW.md`.

## SUITE-REVIEW.md discipline

Every non-trivial suite change requires a `SUITE-REVIEW.md` entry. Non-trivial means: any addition or removal of a domain or primer, any new evaluation dimension, any structural change to the prompt format, or any change to sequencing or activation guidance.

Mechanical fixes (typos, filename renames, path updates) do not require a `SUITE-REVIEW.md` entry but should be logged in `CHANGELOG.md`.

**Gap analysis run entry format** — A `## Gap Analysis Run N — date` entry in `SUITE-REVIEW.md` must contain:

1. **Header** — `## Gap Analysis Run N — YYYY-MM-DD HH:MMZ`
2. **Context** — What prompted this run? New project type, post-mortem, suite change, or scheduled maintenance?
3. **Scope** — Which domains, supplements, and suite artifacts were evaluated?
4. **New gaps** — Each newly identified gap listed with the proposed G-ID, description, and severity assessment. New gaps are added to the registry; the run entry is where the rationale lives.
5. **Addressed gaps** — Each gap closed by changes made during this run, with the G-ID and a one-sentence description of the change.
6. **Dismissed gaps** — Each gap reviewed and rejected, with G-ID and rationale.
7. **Suite changes made** — List of files modified, with what changed and why.

The entry is the narrative record. The registry row is the status indicator. Never put narrative in the registry; never omit the registry update.

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
