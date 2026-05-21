#!/usr/bin/env bash
# DEPRECATED at PR #43 (2026-05-21) — consolidated into the suite-level
# anonymization hook at `vsdd-suite/hooks/check-anonymization.sh`, which is
# a superset (HOME + git user.name + git user.email + public-URL allowlist)
# applied repo-wide to every committed text file. This file is preserved in
# place per G-89 forward-only narrative-preservation; the
# `.pre-commit-config.yaml` no longer references it. Do not extend; extend
# `check-anonymization.sh` instead.
#
# Original behavior (still functional if invoked directly):
# Rejects staged files that contain the current user's home directory path.
# Uses $HOME at runtime — no username is hardcoded in this script.
status=0
for file in "$@"; do
    if grep -qF -- "$HOME" "$file" 2>/dev/null; then
        echo "Error: $file contains a local home directory path."
        echo "  Pattern matched: $HOME"
        echo "  Remove the absolute path before committing."
        status=1
    fi
done
exit $status
