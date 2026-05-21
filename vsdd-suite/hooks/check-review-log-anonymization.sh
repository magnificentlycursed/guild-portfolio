#!/usr/bin/env bash
# Scans ANY committed text file for identity-revealing patterns the developer
# may be opt-in anonymizing. Broadened from review-log-only scope at PR #43
# (per operator directive: "The anonymization check should be broadened to
# include logs, commits, etc. Basically it should apply to all committed
# files"). The previous narrow scope rested on the assumption that source
# code is identity-free by construction; that assumption doesn't hold once
# the suite carries markdown audit trails, supplementary docs, hook source
# itself, etc. — every committed text file may quote a path, a name, an email,
# or a transcript that leaks identity. Apply the same discipline everywhere.
#
# Patterns are read from `git config` and the runtime environment — no
# identity values are hardcoded in this script. Configure with
# `git config user.name "<value>"` and `git config user.email "<value>"`
# (typical noreply settings) before committing.
#
# Public-URL contexts are allowed: the project may opt in to publishing the
# git handle as the public repository URL (`Cargo.toml` `repository` field,
# `package.json` `repository`, etc.) or as the reviewer's authored Bluesky
# handle on a profile URL (per the external-review-log subfolder pattern at
# `vsdd-suite/suite-development/suite-development.md` § Identity-correlation
# discipline). Lines that contain `github.com/`, `gitlab.com/`,
# `bitbucket.org/`, `bsky.app/profile/`, or `*.noreply.<host>` are skipped —
# the handle is deliberately public there. Bare identity citations on other
# lines (`/Users/<handle>/`, free-text mention of the developer's name) are
# rejected.
#
# Designed to be invoked by pre-commit with `pass_filenames: true` and
# `types: [text]`. The caller may narrow `files:` further for specific
# overrides, but the default at PR #43+ is suite-wide application. This
# script does not re-filter — it checks every file argument it receives.

set -u

git_user_name=$(git config user.name 2>/dev/null || true)
git_user_email=$(git config user.email 2>/dev/null || true)
home_dir=${HOME:-}

# Lines containing any of these tokens are exempt: the identity value on
# such lines is being used as a public URL component, not a leak. Adjust the
# allowlist if a project uses a different forge.
public_url_allowlist='github\.com/|gitlab\.com/|bitbucket\.org/|bsky\.app/profile/|noreply\.'

status=0

# Reports a finding for each line in $file that contains the literal $pattern
# AND is NOT in a public-URL context. Empty $pattern is a no-op (e.g., when
# git config user.name is unset).
check_pattern() {
    local file=$1
    local pattern=$2
    local label=$3
    [ -n "$pattern" ] || return 0

    # `grep -nF` for line numbers + literal match; pipe through a grep -v
    # against the public-URL allowlist to drop deliberately-public mentions.
    local hits
    hits=$(grep -nF -- "$pattern" "$file" 2>/dev/null | grep -Ev "$public_url_allowlist" || true)
    if [ -n "$hits" ]; then
        echo "Error: $file contains a $label pattern outside a public-URL context."
        echo "$hits" | sed 's/^/  /'
        echo "  Replace with <user>, <email>, or <path> placeholder, or"
        echo "  rewrite the line into a github.com/gitlab.com/bitbucket.org/"
        echo "  noreply.* URL form if the value is intentionally public."
        echo "  See vsdd-suite/primers/3-review-session.md"
        echo "  § Confidentiality-aware citation."
        status=1
    fi
}

for file in "$@"; do
    [ -f "$file" ] || continue
    check_pattern "$file" "$git_user_name" "git config user.name"
    check_pattern "$file" "$git_user_email" "git config user.email"
    check_pattern "$file" "$home_dir" "local home directory"
done

exit $status
