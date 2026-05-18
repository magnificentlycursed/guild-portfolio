#!/usr/bin/env bash
# scaffold-project.sh — set up a new project's vsdd-suite/ directory + templates.
#
# Usage:
#   cd <your-project>
#   /path/to/vsdd-suite/templates/scaffold-project.sh [domain1 domain2 ...]
#
# With no arguments, scaffolds the seven always-active core domains
# (SOFTWARE-ENGINEER, QUALITY-ENGINEER, UX, SECURITY, SOLUTION-ARCHITECT,
# SOLUTION-OWNER, VDD-IAR-ALIGNMENT). DATA-ENGINEER and PLATFORM-ENGINEER
# are core but conditional — pass them explicitly if active for your project.
# Pass any extended domain (RED-TEAM, PERFORMANCE-ENGINEER, TECHNICAL-WRITER,
# ACCESSIBILITY, PRIVACY, LOCALIZATION) as additional arguments if active.
#
# The script is crosslink-agnostic — it scaffolds the same directory shape
# whether you use crosslink or run the suite manually. Crosslink and the
# suite are independent tools that each scaffold their own state in a
# project; no shared scaffolding is provided.

set -euo pipefail

# Resolve script location regardless of where the user invoked it from.
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEMPLATES_DIR="$SCRIPT_DIR"
SUITE_DIR="$(dirname "$SCRIPT_DIR")"

# Default core domains. DATA-ENGINEER and PLATFORM-ENGINEER are core-but-conditional.
DEFAULT_DOMAINS=(
  SOFTWARE-ENGINEER
  QUALITY-ENGINEER
  UX
  SECURITY
  SOLUTION-ARCHITECT
  SOLUTION-OWNER
  VDD-IAR-ALIGNMENT
)

DOMAINS=("$@")
if [ ${#DOMAINS[@]} -eq 0 ]; then
  DOMAINS=("${DEFAULT_DOMAINS[@]}")
  echo "No domains specified — scaffolding default seven core domains."
  echo "If your project activates DATA-ENGINEER, PLATFORM-ENGINEER, or any extended"
  echo "domain (RED-TEAM, PERFORMANCE-ENGINEER, TECHNICAL-WRITER, ACCESSIBILITY,"
  echo "PRIVACY, LOCALIZATION), re-run with the activated domains as arguments."
  echo ""
fi

mkdir -p vsdd-suite/review-log

# Copy DESIGN.md skeleton (if not present)
if [ ! -f DESIGN.md ]; then
  cp "$TEMPLATES_DIR/DESIGN-template.md" DESIGN.md
  echo "Created DESIGN.md (from template)"
else
  echo "DESIGN.md already exists — skipped"
fi

# Copy project README skeleton (if not present)
if [ ! -f README.md ]; then
  cp "$TEMPLATES_DIR/PROJECT-README-template.md" README.md
  echo "Created README.md (from template)"
else
  echo "README.md already exists — skipped"
fi

# Copy FINDINGS-INDEX.md skeleton (the manual path of G-138 — skip if using
# crosslink for finding tracking). The script always copies the template; the
# project can delete it if adopting the crosslink path instead.
if [ ! -f vsdd-suite/FINDINGS-INDEX.md ]; then
  cp "$TEMPLATES_DIR/PROJECT-FINDINGS-INDEX-template.md" vsdd-suite/FINDINGS-INDEX.md
  echo "Created vsdd-suite/FINDINGS-INDEX.md (manual path of G-138 — delete if using crosslink for finding tracking)"
else
  echo "vsdd-suite/FINDINGS-INDEX.md already exists — skipped"
fi

# Copy per-domain index templates
for domain in "${DOMAINS[@]}"; do
  target="vsdd-suite/${domain}-REVIEW.md"
  if [ ! -f "$target" ]; then
    cp "$TEMPLATES_DIR/DOMAIN-REVIEW-template.md" "$target"
    echo "Created $target (from template — customize per templates/README.md)"
  else
    echo "$target already exists — skipped"
  fi
done

# Record the suite version this scaffold was done against (if a VERSION file exists)
# This anchors the project's suite usage to a specific suite revision.
if [ -f "$SUITE_DIR/VERSION" ]; then
  cp "$SUITE_DIR/VERSION" vsdd-suite/.suite-version
  echo "Recorded suite version in vsdd-suite/.suite-version"
fi

cat <<'EOF'

Scaffold complete. Next steps:

1. Customize the placeholders in each vsdd-suite/<DOMAIN>-REVIEW.md per
   templates/README.md § Customization checklist. The role line, sycophancy
   check, and language supplement come verbatim from the domain prompt
   file in vsdd-suite/domains/role/<DOMAIN>-REVIEW.md.

2. Open Phase 1a — load vsdd-suite/primers/1a-spec-crystallization.md
   in a fresh AI chat session and work the driving questions to populate
   DESIGN.md. Commit when the self-adversary check passes.

3. Optional: also commit your project README in this phase (the skeleton
   was just copied) — it gives Phase 3 Technical Writer reviews an
   artifact to evaluate from Layer 1 onward.

4. See vsdd-suite/README.md § Worked example for the full walkthrough.
EOF
