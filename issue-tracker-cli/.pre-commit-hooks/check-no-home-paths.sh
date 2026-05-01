#!/usr/bin/env bash
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
