# Templates

Scaffolding artifacts for new projects adopting the VSDD suite. These templates are copied into a new project's tree to establish the conventions; they are not used at runtime.

## Contents

| Template | Purpose | Copied to |
|---|---|---|
| `DESIGN-template.md` | Skeleton for the project's `DESIGN.md` (Phase 1a output). Mirrors the driving-question structure from `primers/1a-spec-crystallization.md` so the first session has a structured starting point. | `<your-project>/DESIGN.md` |
| `PROJECT-README-template.md` | Skeleton for the project's user-facing README — purpose, prerequisites, run/test, link to DESIGN.md. | `<your-project>/README.md` |
| `DOMAIN-REVIEW-template.md` | Generic per-domain index file. Copy and customize per active domain (role, activation, language supplement, sycophancy check from the domain prompt file). | `<your-project>/vsdd-suite/<DOMAIN>-REVIEW.md` (one per active domain) |
| `PROJECT-FINDINGS-INDEX-template.md` | Cross-cutting findings registry (manual path of G-138). One row per finding across every domain and layer, structured like the suite's GAP-ANALYSIS-LOG.md. Skip if using crosslink for finding tracking — `crosslink issue` with the labeled-issue convention is the equivalent (see `vsdd-suite/suite-development/suite-development.md` § Project-level finding index). | `<your-project>/vsdd-suite/FINDINGS-INDEX.md` (single file per project) |
| `scaffold-project.sh` | Helper script: creates `vsdd-suite/` + `review-log/` in a target project, copies the templates, prints next-step guidance. | run from project root |

## Usage

**Manual (suite-only path):**

```sh
cd <your-project>
mkdir -p vsdd-suite/review-log
cp <path-to-vsdd-suite>/templates/DESIGN-template.md DESIGN.md
cp <path-to-vsdd-suite>/templates/PROJECT-README-template.md README.md
# For each active domain (see vsdd-suite/domains/DOMAIN-INDEX.md):
cp <path-to-vsdd-suite>/templates/DOMAIN-REVIEW-template.md vsdd-suite/QUALITY-ENGINEER-REVIEW.md
# ... then customize the placeholders in each copied file
```

**With the helper script:**

```sh
cd <your-project>
<path-to-vsdd-suite>/templates/scaffold-project.sh
# Then customize the placeholders in the copied files.
```

**Crosslink-enabled projects:** the templates are independent of crosslink — run the same copy/script. Crosslink and the suite are separate tools that each scaffold their own state in a project; no shared scaffolding mechanism is provided.

## Customization checklist

For each per-domain `<DOMAIN>-REVIEW.md` you copy:

1. Replace `{{ROLE_TITLE}}` with the role name from the domain prompt's `Reviewer role:` line (e.g., "Quality Engineer").
2. Replace `{{ROLE_VARIANTS}}` with the variants from the same line (e.g., "Quality Engineer / QA Engineer / Test Engineer").
3. Replace `{{PURPOSE}}` with one paragraph stating what this domain evaluates for *this specific project*.
4. **For extended domains only:** fill in the `**Activation:**` line with the named conditions and your project-specific rationale. For core/meta domains, delete the Activation line.
5. Replace `{{LANGUAGE_SUPPLEMENT_LINE}}` with either the active supplement reference or the explicit opt-out for language-agnostic domains.
6. Replace `{{SYCOPHANCY_CHECK}}` with the domain prompt file's sycophancy-check paragraph (copy verbatim, do not paraphrase).

For `DESIGN.md`: work the driving questions in `vsdd-suite/primers/1a-spec-crystallization.md` rather than filling in the skeleton structurally — the skeleton is a placeholder, not a fill-in-the-blanks form.

For project `README.md`: this is your user-facing entry point; expand and edit freely as the project takes shape. The skeleton just establishes existence so Phase 3 Technical Writer reviews have an artifact to evaluate.
