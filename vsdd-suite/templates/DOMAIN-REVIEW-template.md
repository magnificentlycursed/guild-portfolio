<!--
OPTIONAL TEMPLATE — per-domain index files are NOT created by default as of
v0.13.0 ([Review 84](../suite-development/review-log/2026-05-21-suite-review.md#review-84--2026-05-21-1100z)).
New projects scaffolded at v0.13.0+ navigate via `review-log/` (per-session
files) + `FINDINGS-INDEX.md` (cross-cutting registry) without a per-domain
index. The `bookmark-cli-manual` reference example retired its 13 per-domain
index files in PR #40 and is the new canonical reference shape.

This template exists for projects that opt in to the per-domain index — either
via `templates/scaffold-project.sh --with-per-domain-indexes` or by manual
copy. See [`suite-development/suite-development.md`](../suite-development/suite-development.md)
§ Structure for the canonical default and the opt-in rationale.
-->

# {{ROLE_TITLE}} Review Log (Index)

This review log is part of the [VSDD Suite](../../README.md). The [Phase 3](../primers/3-review-session.md) adversarial review for this domain runs as one cold-context session per round; this file indexes the rounds. Round narratives live in [`review-log/`](review-log/) — see the Reviews table below.

**Reviewer role: {{ROLE_TITLE}}** ({{ROLE_VARIANTS}})

{{PURPOSE}}

**Activation:** {{ACTIVATION_CONDITIONS_AND_RATIONALE}}
<!-- Required for extended domains; delete this line for core and meta domains. -->

**Language supplement applied:** {{LANGUAGE_SUPPLEMENT_LINE}}
<!-- Examples:
  Language supplement applied: `../../supplements/rust.md` ([Quality Engineer](../domains/role/QUALITY-ENGINEER-REVIEW.md) section).
  Language supplement applied: `../../supplements/javascript-typescript.md` ([Security](../domains/role/SECURITY-REVIEW.md) section).
  Language supplement applied: Not applicable. This is a meta domain; no language-specific dimensions.
-->

**Sycophancy check:** {{SYCOPHANCY_CHECK}}
<!-- Copy verbatim from the domain prompt file's Sycophancy check section. Do not paraphrase. -->

Read individual rounds via the links in the Reviews table below. This file is the index only; round narratives live in `review-log/YYYY-MM-DD-<slug>.md` per the project-level review log governing standard in [`../../suite-development/suite-development.md`](../../suite-development/suite-development.md) § Structure.

---

## Reviews

| Review | Date | File | Scope summary |
|---|---|---|---|
| *(no rounds filed yet)* | | | |
