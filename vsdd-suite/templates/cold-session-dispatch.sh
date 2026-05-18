#!/usr/bin/env bash
# cold-session-dispatch — assemble the canonical paste-into-fresh-chat prompt
# for a Phase 3 cold-session domain review.
#
# Mechanizes the G-134 friction: the operator's stated pain in ITC PROCESS.md
# L2 + L3 was the manual assembly cost ("I don't have a good manual workflow
# for running [cold sessions] which suggests some helper scripts or project
# level claude.md might reduce friction"). This script removes that cost —
# operator provides a domain + scope, the script emits primer + domain prompt
# + applicable supplement(s) + scope framing to stdout. Operator pipes to
# clipboard (pbcopy / xclip / wl-copy) and pastes into a fresh chat session
# in the AI tool of choice.
#
# Usage:
#   cold-session-dispatch.sh <domain-slug> [--layer <N>] [--lang <slug>] [--scope <text>]
#
# Examples:
#   cold-session-dispatch.sh quality-engineer --layer 3 --lang rust --scope "Layer 3 priority filter — src/lib.rs cmd_list and sort_issues"
#   cold-session-dispatch.sh security --lang rust --scope "Whole application"
#   cold-session-dispatch.sh ux --scope "Layer 7 polish — color output and --help"
#
# Pipe to clipboard:
#   cold-session-dispatch.sh quality-engineer --lang rust --scope "Layer 3" | pbcopy   # macOS
#   cold-session-dispatch.sh quality-engineer --lang rust --scope "Layer 3" | xclip -selection clipboard  # Linux X11
#   cold-session-dispatch.sh quality-engineer --lang rust --scope "Layer 3" | wl-copy  # Linux Wayland
#
# Domain slugs match the canonical table in suite-development.md § Domain slug
# convention (lowercase, hyphenated, derived from the role title — e.g.,
# `quality-engineer`, `security`, `ux`, `vdd-iar-alignment`). Run without args
# to see the full list.

set -euo pipefail

# Resolve the suite root from this script's location so the script works
# from any project's working directory.
SUITE_ROOT=$(cd "$(dirname "$0")/.." && pwd)
PRIMERS_DIR="$SUITE_ROOT/primers"
DOMAINS_ROLE_DIR="$SUITE_ROOT/domains/role"
DOMAINS_META_DIR="$SUITE_ROOT/domains/meta"
SUPPLEMENTS_DIR="$SUITE_ROOT/supplements"

print_usage() {
    cat >&2 <<EOF
Usage: cold-session-dispatch.sh <domain-slug> [--layer <N>] [--lang <slug>] [--scope <text>]

Domain slugs (canonical per suite-development.md § Domain slug convention):

  Core role domains:
    software-engineer       quality-engineer        ux
    security                platform-engineer       solution-architect
    solution-owner          data-engineer

  Extended role domains:
    red-team                performance-engineer    technical-writer
    accessibility           privacy                 localization

  Meta domains:
    vdd-iar-alignment       portfolio-assessment

Language supplement slugs (under supplements/):
  rust                    javascript-typescript   (others: see supplements/ directory)

Interface supplements: cli, browser-app

Examples:
  cold-session-dispatch.sh quality-engineer --layer 3 --lang rust --scope "Layer 3 priority filter"
  cold-session-dispatch.sh security --lang rust --scope "Whole application"

Pipe to clipboard:
  cold-session-dispatch.sh quality-engineer --lang rust --scope "Layer 3" | pbcopy
EOF
    exit 1
}

slug_to_filename() {
    # Convert lowercase slug to UPPERCASE-HYPHENATED-REVIEW.md
    echo "$1" | tr '[:lower:]' '[:upper:]' | sed 's/$/-REVIEW.md/'
}

find_domain_file() {
    local filename="$1"
    if [ -f "$DOMAINS_ROLE_DIR/$filename" ]; then
        echo "$DOMAINS_ROLE_DIR/$filename"
    elif [ -f "$DOMAINS_META_DIR/$filename" ]; then
        echo "$DOMAINS_META_DIR/$filename"
    else
        return 1
    fi
}

# Parse args.
if [ $# -eq 0 ]; then
    print_usage
fi

DOMAIN_SLUG="$1"
shift

LAYER=""
LANG=""
SCOPE=""

while [ $# -gt 0 ]; do
    case "$1" in
        --layer)
            LAYER="$2"
            shift 2
            ;;
        --lang)
            LANG="$2"
            shift 2
            ;;
        --scope)
            SCOPE="$2"
            shift 2
            ;;
        -h|--help)
            print_usage
            ;;
        *)
            echo "Unknown option: $1" >&2
            print_usage
            ;;
    esac
done

DOMAIN_FILENAME=$(slug_to_filename "$DOMAIN_SLUG")
DOMAIN_FILE=$(find_domain_file "$DOMAIN_FILENAME" || true)
if [ -z "$DOMAIN_FILE" ]; then
    echo "Error: no domain file found for slug '$DOMAIN_SLUG' (expected $DOMAINS_ROLE_DIR/$DOMAIN_FILENAME or $DOMAINS_META_DIR/$DOMAIN_FILENAME)" >&2
    print_usage
fi

PRIMER_FILE="$PRIMERS_DIR/3-review-session.md"
if [ ! -f "$PRIMER_FILE" ]; then
    echo "Error: primer not found at $PRIMER_FILE" >&2
    exit 2
fi

# Assemble the prompt to stdout.
cat <<EOF
# Cold-Session Adversarial Review — Dispatch Prompt
#
# Assembled by vsdd-suite/templates/cold-session-dispatch.sh
# Domain: ${DOMAIN_SLUG}
# Layer: ${LAYER:-(not specified)}
# Language supplement: ${LANG:-(none)}
# Scope: ${SCOPE:-(not specified — review will default to whole-application scope per domain prompt)}
#
# Paste this entire prompt into a fresh chat session (Claude Code, claude.ai,
# Cursor, GitHub Copilot Chat, etc.). Do NOT load any prior conversation
# context; the cold-context discipline is load-bearing for Phase 3 review
# integrity (see vsdd-suite/README.md § Same-model review limitation). After
# pasting this prompt, share the project's DESIGN.md and the relevant source
# files via your tool's file-attachment or context mechanism, then ask the
# agent to begin the review.

---

## SECTION 1 — Session Primer (vsdd-suite/primers/3-review-session.md)

$(cat "$PRIMER_FILE")

---

## SECTION 2 — Domain Prompt (${DOMAIN_FILE#"$SUITE_ROOT/"})

$(cat "$DOMAIN_FILE")

EOF

# Optional language/interface supplement.
if [ -n "$LANG" ]; then
    SUPPLEMENT_FILE="$SUPPLEMENTS_DIR/${LANG}.md"
    if [ ! -f "$SUPPLEMENT_FILE" ]; then
        echo "" >&2
        echo "Warning: language supplement '$LANG' not found at $SUPPLEMENT_FILE — proceeding without supplement" >&2
    else
        cat <<EOF
---

## SECTION 3 — Language / Interface Supplement (supplements/${LANG}.md)

Apply the section matching the domain (\`## ${DOMAIN_SLUG^}\` or the canonical role-title section) in addition to the standard dimensions in Section 2.

$(cat "$SUPPLEMENT_FILE")

EOF
    fi
fi

cat <<EOF
---

## SECTION 4 — Scope and instructions for this review

**Scope:** ${SCOPE:-Whole application (default per domain prompt; the agent should narrow only if the domain prompt directs scoping).}

EOF

if [ -n "$LAYER" ]; then
    echo "**Layer:** Layer ${LAYER}. This review is scoped to Layer ${LAYER}'s changes; the regression check still covers the whole application per the domain prompt."
    echo ""
fi

cat <<EOF
**Instructions for the agent:**

1. Read DESIGN.md in full before reading any source files.
2. Read all source files in scope. Do not skim.
3. Read the prior IAR log for this layer if one exists, per the primer's "Before starting a domain review" section. Do not re-raise findings already resolved and verified. Do raise findings dismissed without adequate rationale.
4. Apply every standard dimension from Section 2 (and Section 3 if a language supplement was included) as a floor. Add other findings as appropriate.
5. For each finding, cite file and line number. Classify per the domain's classification schema (see Section 2).
6. End with the round number and finding progression: a round that moves from real findings to only Hallucinated findings is the MVR signal.

**Output format:** structured Markdown with one section per finding, following the per-review entry preamble standard documented in vsdd-suite/suite-development/suite-development.md § Per-review entry preamble. Include a \`**Source:**\` field — for cold-session dispatch use \`**Source:** domain-raised\`.

End of dispatch prompt.
EOF
