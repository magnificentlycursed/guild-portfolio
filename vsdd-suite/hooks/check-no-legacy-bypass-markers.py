#!/usr/bin/env python3
"""
VSDD suite pre-commit hook — rejects legacy unscoped hook-bypass markers.

Per AIE R2 F6 SO-decision (bookmark-cli-manual PR #52 Round 2 carry-forward):
the legacy `<!-- hook-bypass: <rationale> -->` marker is hook-agnostic —
when present, it silently bypasses every hook that parses bypass markers
(check-no-letter-clusters, check-suite-internal-terminology,
check-suite-review-preamble, check-project-review-discipline). An author
intending to bypass ONE hook for a deliberate out-of-pattern entry
inadvertently disables four other discipline checks; the bypass mechanism
is therefore wider than the bypass intent.

The closure: a scoped marker form names the hooks it bypasses explicitly:

    <!-- hook-bypass[check-no-letter-clusters,check-project-review-discipline]: <rationale> -->

Each parsing hook only treats the marker as a bypass if its own pre-commit
id appears in the scope list. This hook flags the legacy unscoped form as
a violation — preventing the AIE R2 F6 risk from recurring + forcing every
new bypass marker to declare its scope explicitly.

Scope: all markdown files in the repository (matches the union of file
sets the bypass-parsing hooks cover). Bypass for this hook itself uses
the scoped form `<!-- hook-bypass[check-no-legacy-bypass-markers]: ... -->`
(a deliberate self-reference: this hook can be bypassed for a legitimate
documentation-of-legacy-form citation, e.g., a primer paragraph that
shows the rejected legacy form as a counterexample).
"""

from __future__ import annotations

import re
import sys
from pathlib import Path


HOOK_ID = "check-no-legacy-bypass-markers"

# Scoped form: `<!-- hook-bypass[hook-id1,hook-id2]: <rationale> -->`. Canonical.
SCOPED_BYPASS_RE = re.compile(
    r"<!--\s*hook-bypass\[([^\]]+)\]:\s*.+?-->",
    re.IGNORECASE | re.DOTALL,
)
# Legacy form: `<!-- hook-bypass: <rationale> -->` (no brackets after hook-bypass). REJECTED.
LEGACY_BYPASS_RE = re.compile(
    r"<!--\s*hook-bypass:\s*(?!\[).+?-->",
    re.IGNORECASE | re.DOTALL,
)


def has_self_bypass(content: str) -> bool:
    """Return True if a scoped marker naming this hook's id is in the first 5 lines."""
    head = "\n".join(content.splitlines()[:5])
    for match in SCOPED_BYPASS_RE.finditer(head):
        scoped_hooks = [h.strip() for h in match.group(1).split(",")]
        if HOOK_ID in scoped_hooks:
            return True
    return False


def find_legacy_markers(content: str, path: str) -> list[str]:
    """Return one violation string per legacy-form marker in the file's first 5 lines.

    Only first-5-lines positions are flagged because that's the position where the
    bypass-parsing hooks actually consume bypass markers. A legacy-form citation
    deeper in the file (CHANGELOG entry; primer prose example; supplement-doc
    discussion) is prose, not an active bypass attempt, so it doesn't trip the
    AIE R2 F6 risk this hook is closing. Per-entry first-5-lines positions inside
    `## Review N` blocks are flagged by check-suite-review-preamble + check-
    project-review-discipline (which already scope to those entries).
    """
    findings = []
    for lineno, line in enumerate(content.splitlines()[:5], start=1):
        if LEGACY_BYPASS_RE.search(line):
            findings.append(
                f"{path}:{lineno}: legacy unscoped hook-bypass marker REJECTED "
                f"per AIE R2 F6 SO-decision. Rewrite as scoped form: "
                f"`<!-- hook-bypass[<hook-id1>,<hook-id2>]: <rationale> -->` "
                f"(comma-separate hook ids to bypass multiple hooks; valid ids: "
                f"check-no-letter-clusters, check-suite-internal-terminology, "
                f"check-suite-review-preamble, check-project-review-discipline, "
                f"check-no-legacy-bypass-markers). The bypass mechanism is itself "
                f"a finding for the next registry-walk review."
            )
    return findings


def check_file(path: str) -> list[str]:
    try:
        content = Path(path).read_text(encoding="utf-8")
    except (OSError, UnicodeDecodeError):
        return []
    if has_self_bypass(content):
        return []
    return find_legacy_markers(content, path)


def main() -> int:
    paths = sys.argv[1:]
    if not paths:
        return 0
    violations = []
    for path in paths:
        violations.extend(check_file(path))
    if not violations:
        return 0
    sys.stderr.write(
        "check-no-legacy-bypass-markers: legacy unscoped hook-bypass markers detected:\n"
    )
    for v in violations:
        sys.stderr.write(f"  {v}\n")
    sys.stderr.write(
        "\nPer AIE R2 F6 SO-decision (bookmark-cli-manual PR #52 carry-forward): "
        "the legacy unscoped form silently bypassed every hook that parses bypass "
        "markers. The scoped form names the hooks explicitly so the bypass scope "
        "matches the bypass intent. To bypass this hook itself for a legitimate "
        "documentation-of-legacy-form citation (e.g., a primer counterexample), "
        "add `<!-- hook-bypass[check-no-legacy-bypass-markers]: <rationale> -->` "
        "within the first 5 lines of the file.\n"
    )
    return 1


if __name__ == "__main__":
    sys.exit(main())
