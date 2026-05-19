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
# The script scaffolds the same directory shape whether you use crosslink
# or run the suite manually. When the project has already been initialized
# with `crosslink init` (i.e., `.crosslink/` exists) AND the `crosslink`
# binary is in PATH, the script additionally registers the suite primers
# and domain prompts as crosslink knowledge pages so future
# `crosslink kickoff run` / `crosslink swarm review` sessions can load them
# without per-session copy/paste (G-146). Manual mode is unaffected — when
# `.crosslink/` is absent or `crosslink` is not installed, the knowledge-
# registration step is skipped silently and the operator loads primers by
# hand into chat sessions per the manual quickstart.

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

# G-146 / G-163 / G-164: register suite primers, ACTIVATED domain prompts,
# and supplements as crosslink knowledge pages when the project is crosslink-
# enabled. The conditions are both:
#   (a) `.crosslink/` exists (`crosslink init` has been run in this project)
#   (b) the `crosslink` binary is in PATH
# When either is absent, the registration is skipped silently — manual mode
# loads primers by hand per the manual quickstart.
#
# G-163: only the activated domain prompts (from the DOMAINS array) are
# registered — registering all 16 prompts when a learning-exercise project
# uses 3 is over-investment per the intent-calibration discipline.
# G-164: language and interface supplements/ are also registered so swarm-
# review agents have the supplement loaded alongside the domain prompt.
KNOWLEDGE_REGISTERED=0
if [ -d .crosslink ] && command -v crosslink >/dev/null 2>&1; then
  echo ""
  echo "Crosslink project detected — registering suite primers, activated domain prompts, and supplements as knowledge pages..."
  if crosslink knowledge import "$SUITE_DIR/primers" --tag vsdd-suite-primer --quiet 2>/dev/null; then
    echo "  Registered primers (tagged vsdd-suite-primer)."
    KNOWLEDGE_REGISTERED=1
  else
    echo "  Primer registration failed — re-run manually after resolving:"
    echo "    crosslink knowledge import \"$SUITE_DIR/primers\" --tag vsdd-suite-primer"
  fi

  # G-163: register only the activated domain prompts. Stage them into a temp
  # directory so `crosslink knowledge import <dir>` operates on the subset.
  STAGE_DIR="$(mktemp -d)"
  trap 'rm -rf "$STAGE_DIR"' EXIT
  STAGED=0
  for domain in "${DOMAINS[@]}"; do
    # Map DOMAIN slug → role-or-meta path. VDD-IAR-ALIGNMENT and PORTFOLIO-ASSESSMENT live under meta/; everything else under role/.
    if [ "$domain" = "VDD-IAR-ALIGNMENT" ] || [ "$domain" = "PORTFOLIO-ASSESSMENT" ]; then
      src="$SUITE_DIR/domains/meta/${domain}-REVIEW.md"
    else
      src="$SUITE_DIR/domains/role/${domain}-REVIEW.md"
    fi
    if [ -f "$src" ]; then
      cp "$src" "$STAGE_DIR/${domain}-REVIEW.md"
      STAGED=$((STAGED + 1))
    fi
  done
  if [ "$STAGED" -gt 0 ]; then
    if crosslink knowledge import "$STAGE_DIR" --tag vsdd-suite-domain --quiet 2>/dev/null; then
      echo "  Registered $STAGED activated domain prompts (tagged vsdd-suite-domain)."
    else
      echo "  Activated-domain registration failed — re-run manually with the activated set."
    fi
  fi

  # G-164: register language and interface supplements.
  if crosslink knowledge import "$SUITE_DIR/supplements" --tag vsdd-suite-supplement --quiet 2>/dev/null; then
    echo "  Registered language and interface supplements (tagged vsdd-suite-supplement)."
  fi

  echo ""
  echo "  Re-import with --overwrite when the suite version bumps."
  echo "  Manual primer load (paste-into-chat) remains supported in both modes."
fi

cat <<'EOF'

Scaffold complete. Next steps:

1. Customize the placeholders in each vsdd-suite/<DOMAIN>-REVIEW.md per
   templates/README.md § Customization checklist. The role line, sycophancy
   check, and language supplement come verbatim from the domain prompt
   file in vsdd-suite/domains/role/<DOMAIN>-REVIEW.md.

2. Open Phase 1a+1b — load vsdd-suite/primers/1ab-spec-crystallization.md
   in a fresh AI chat session and work the driving questions to populate
   DESIGN.md. Commit when the self-adversary check passes.

3. Optional: also commit your project README in this phase (the skeleton
   was just copied) — it gives Phase 3 Technical Writer reviews an
   artifact to evaluate from Layer 1 onward.

4. See vsdd-suite/README.md § Worked example for the full walkthrough.
EOF

if [ "$KNOWLEDGE_REGISTERED" -eq 0 ] && command -v crosslink >/dev/null 2>&1 && [ ! -d .crosslink ]; then
  cat <<'EOF'

Crosslink binary detected but project not yet initialized.
After running `crosslink init`, re-run this scaffold script to register
the suite primers, activated domain prompts, and supplements as crosslink
knowledge pages, or run the registration manually with your activated set:

   crosslink knowledge import <path-to-vsdd-suite>/primers --tag vsdd-suite-primer
   # For each activated domain (substitute <DOMAIN> with the slug):
   crosslink knowledge add <DOMAIN>-review --from-doc <path-to-vsdd-suite>/domains/<role|meta>/<DOMAIN>-REVIEW.md --tag vsdd-suite-domain
   crosslink knowledge import <path-to-vsdd-suite>/supplements --tag vsdd-suite-supplement
EOF
fi
