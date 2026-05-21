# Templates

Scaffolding artifacts for new projects adopting the VSDD suite. These templates are copied into a new project's tree to establish the conventions; they are not used at runtime.

## Contents

| Template | Purpose | Copied to |
|---|---|---|
| `DESIGN-template.md` | Skeleton for the project's `DESIGN.md` ([Phase 1a+1b](../primers/1ab-spec-development.md) output). Mirrors the driving-question structure from `primers/1ab-spec-crystallization.md` so the first session has a structured starting point. | `<your-project>/DESIGN.md` |
| `PROJECT-README-template.md` | Skeleton for the project's user-facing README — purpose, prerequisites, run/test, link to DESIGN.md. | `<your-project>/README.md` |
| `DOMAIN-REVIEW-template.md` | Generic per-domain index file — **OPTIONAL as of v0.13.0 ([Review 84](../suite-development/review-log/2026-05-21-suite-review.md#review-84--2026-05-21-1100z))**. Copy and customize per active domain (role, activation, language supplement, sycophancy check from the domain prompt file) only when the project wants a domain-organized navigation surface; the scaffold script no longer copies this by default. Pass `--with-per-domain-indexes` to `scaffold-project.sh` (or copy manually) to opt in. | `<your-project>/vsdd-suite/<DOMAIN>-REVIEW.md` (one per active domain, only if opted in) |
| `PROJECT-FINDINGS-INDEX-template.md` | Cross-cutting findings registry (manual-mode equivalent of [G-138](../suite-development/FINDINGS-INDEX.md#g-138)). One row per finding across every domain and layer, structured like the suite's FINDINGS-INDEX.md. Skip in [crosslink](https://github.com/forecast-bio/crosslink) mode — `crosslink issue` with the labeled-issue convention is the queryable equivalent (see `vsdd-suite/suite-development/suite-development.md` § Project-level finding index). | `<your-project>/vsdd-suite/FINDINGS-INDEX.md` (single file per project, manual mode only) |
| `scaffold-project.sh` | Helper script: creates `vsdd-suite/` + `review-log/` in a target project, copies the templates, prints next-step guidance. | run from project root |

## Usage

The scaffold step is identical for both operational modes — it lays down the per-project artifacts (DESIGN.md, project README, `vsdd-suite/review-log/` folder, `vsdd-suite/FINDINGS-INDEX.md`, and — only when `--with-per-domain-indexes` is passed — per-domain `<DOMAIN>-REVIEW.md` index files). The mode you operate in afterwards (`[crosslink]` recommended or `[manual]` first-class fallback) is determined by whether crosslink is installed and whether you exercise the crosslink-mode commands described in `vsdd-suite/README.md` § Worked example. The templates themselves are mode-independent.

**With the helper script (recommended for both modes):**

```sh
cd <your-project>
<path-to-vsdd-suite>/templates/scaffold-project.sh
# Then customize the placeholders in the copied files.
```

**Manual scaffold (if you prefer not to use the script):**

```sh
cd <your-project>
mkdir -p vsdd-suite/review-log
cp <path-to-vsdd-suite>/templates/DESIGN-template.md DESIGN.md
cp <path-to-vsdd-suite>/templates/PROJECT-README-template.md README.md
# In manual mode, copy the cross-cutting findings registry:
cp <path-to-vsdd-suite>/templates/PROJECT-FINDINGS-INDEX-template.md vsdd-suite/FINDINGS-INDEX.md
# OPTIONAL (per-domain index files — retired as default in v0.13.0, Review 84):
# Only do this if the project wants a domain-organized navigation surface
# in addition to the date-organized review-log/. For each active domain
# (see vsdd-suite/domains/DOMAIN-INDEX.md):
# cp <path-to-vsdd-suite>/templates/DOMAIN-REVIEW-template.md vsdd-suite/QUALITY-ENGINEER-REVIEW.md
# ... then customize the placeholders in each copied file.
```

Crosslink and the suite are separate tools that each scaffold their own state in a project; there is no shared scaffolding mechanism, so the suite scaffold step runs the same way regardless of whether crosslink is installed.

## Customization checklist

**Per-domain index files (optional as of v0.13.0 — only present if the project opted in via `scaffold-project.sh --with-per-domain-indexes` or by manual copy).** For each `<DOMAIN>-REVIEW.md` you copy:

1. Replace `{{ROLE_TITLE}}` with the role name from the domain prompt's `Reviewer role:` line (e.g., "[Quality Engineer](../domains/role/QUALITY-ENGINEER-REVIEW.md)").
2. Replace `{{ROLE_VARIANTS}}` with the variants from the same line (e.g., "Quality Engineer / QA Engineer / Test Engineer").
3. Replace `{{PURPOSE}}` with one paragraph stating what this domain evaluates for *this specific project*.
4. **For extended domains only:** fill in the `**Activation:**` line with the named conditions and your project-specific rationale. For core/meta domains, delete the Activation line.
5. Replace `{{LANGUAGE_SUPPLEMENT_LINE}}` with either the active supplement reference or the explicit opt-out for language-agnostic domains.
6. Replace `{{SYCOPHANCY_CHECK}}` with the domain prompt file's sycophancy-check paragraph (copy verbatim, do not paraphrase).

Projects on the v0.13.0+ default (no per-domain index files) skip this checklist entirely; the per-session review-log file (`review-log/YYYY-MM-DD-<slug>.md`) is self-describing via its filename and H1 date, and `FINDINGS-INDEX.md` is the cross-cutting view.

For `DESIGN.md`:

1. Work the driving questions in `vsdd-suite/primers/1ab-spec-crystallization.md` rather than filling in the skeleton structurally — the skeleton is a placeholder, not a fill-in-the-blanks form.
2. **Declare `§ Project intent` first** (`learning-exercise` / `portfolio` / `capstone` / `production`). The intent gates the active-domain set (per `vsdd-suite/domains/DOMAIN-INDEX.md` § Intent calibration), the stop-signal sensitivity, and at capstone+ intent the [Phase 5](../primers/5-formal-hardening.md) / [Phase 6](../primers/6-convergence.md) strategy declarations. The wrong intent over-invests or under-invests methodology effort and the over-investment variant is hard to catch in-project. Declare the intent before customizing any per-domain index files (if opted in) so the active-domain set you scaffold matches the intent you'll review against.

For project `README.md`: this is your user-facing entry point; expand and edit freely as the project takes shape. The skeleton just establishes existence so [Phase 3](../primers/3-review-session.md) [Technical Writer](../domains/role/TECHNICAL-WRITER-REVIEW.md) reviews have an artifact to evaluate.
