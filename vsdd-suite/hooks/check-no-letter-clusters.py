#!/usr/bin/env python3
# ruff: noqa: W605
r"""
Pre-commit hook: ban letter-only labels (Cluster A/B/C, Surface A/B, Path A/B, Option A/B)
in user-facing project + suite-side audit-trail prose.

**Why this hook exists.** Four recurrences across four PRs of the same anti-pattern: I
reach for letter labels as working-session shorthand (`Cluster A` to name a thematic
grouping of findings during a routing pass; `Surface A.0` to name a Phase 5 verification
activity; `Path B` to label an SO-decision option in an AskUserQuestion call) and the
letters leak into permanent artifacts — DESIGN.md spec amendments, the Phase 4 routing
record, CHANGELOG slim-form entries. A future reader (operator a month later; cold-session
sub-agent reading the audit trail) sees `Cluster B` and has no recovery path — the letter
carries no meaning at point of use. Memory-discipline + primer-level rules have failed to
catch the slip across 4 cycles (Review 78 Surface lettering; PR #38 Round 3 cluster
lettering; PR #44 Round 1 cluster lettering; PR #52 Phase 4 routing cluster lettering).
This hook is the next escalation: mechanical enforcement at the artifact boundary.

**The discipline:** every thematic grouping (clusters of findings, verification surfaces,
SO-decision options, multi-path choices) MUST carry a descriptive name as its primary
identifier. Letter labels (A/B/C/D, Roman numerals, Greek letters) are not acceptable as
section headers, finding-cluster identifiers, or option labels — they require a lookup
that descriptive names avoid.

**Forbidden patterns:**

1. `\bCluster [A-Z]\b` (with optional digit suffix like `A1`, `A2`) — the recurring
   offender across cluster-batching + Phase-4-routing-record workflows.
2. `\bSurface [A-Z](?:\.\d)?\b` — the Phase 5 hardening offender (caught at Review 78);
   retired forms `Surface A.0`, `Surface B`, etc. should not appear in new prose.
3. `\bPath [A-Z]\b` — the AskUserQuestion option-naming variant.
4. `\bOption [A-Z]\b` — variant of Path; same anti-pattern.

**Allowed:**

- Existing well-established abbreviations that DO add meaning at point of use:
  `Dim 2`, `Layer 1`, `Round 3`, `Phase 5`, `Finding 7`, `R8 F1`, domain slugs
  (`solution-architect`, `red-team`) — these carry meaning without a lookup.
- Descriptive names with optional ordering suffixes: `JSON-native escape design`,
  `sorted-tag-comparison dedup`, `imported-tag control-char rejection` — descriptive
  identifier first, no opaque letter.
- Per-file `<!-- hook-bypass: <rationale> -->` HTML comment in the first 5 lines of the
  file bypasses for that file only. Bypasses are themselves findings for the next
  registry-walk review.

**Scope:**

- User-facing project artifacts: `*/DESIGN.md`, `*/README.md`, `*/TODO.md`,
  `*/PROCESS.md`, `*/manual-tests/*.md`, `*/INSTALL-VERIFICATION.md`.
- Project audit-trail artifacts: `*/CHANGELOG.md`, `*/vsdd-suite/**/*.md`.
- Suite-side artifacts: `vsdd-suite/**/*.md` (primers, domain prompts, suite-development
  artifacts, review logs, supplements, README, COMPATIBILITY).

**Out-of-scope:**

- Generic code paths (`*.rs`, `*.py`, `*.toml`, `*.yml`) — where `Surface` or `Path`
  might be legit type / variable names.
- Verbatim quoted blocks (lines starting with `> ` or fenced code blocks) — preserve
  historical references + source artifacts + command transcripts; not scanned by the
  rule patterns (parallel to the check-suite-internal-terminology.py pattern).

**Forward-only:** enforcement applies at the commit-time check; pre-existing audit-trail
references to retired letter labels (e.g., `Surface A.0` in pre-Review-78 review logs)
are NOT migrated by this hook. The forward-only carve-out is implicit — the hook checks
new + changed files at commit time; historical files unchanged in a commit are not
scanned.

**Cross-domain ownership** (per Review 87 Finding 6 per-error-class owner table):
this is a process-enforcement + early-detection script — AI Engineer owns; the
substantive discipline (letter-labels-vs-descriptive-names) is informed by Technical
Writer Dim 12 (lookup-cost) + UX Dim 6 (message clarity).

Exit codes:
- 0 — pass
- 1 — discipline violation detected; specific lines + offending labels emitted to stderr
"""

import re
import sys
from pathlib import Path
from typing import List


# Patterns flagged. Each is (regex, descriptive-rationale-for-violation-message).
# The patterns use word boundaries on both sides to avoid false positives on
# substring matches (e.g., "Subcluster A" inside a larger identifier shouldn't trip).
FORBIDDEN_PATTERNS = [
    (
        re.compile(r"\bCluster [A-Z]\d*\b"),
        "Letter-only `Cluster X` label. Rename to a descriptive identifier that "
        "carries meaning at point of use (e.g., `JSON-native escape design cluster`, "
        "`sorted-tag-comparison dedup cluster`). The letter requires a lookup that "
        "the descriptive name avoids.",
    ),
    (
        re.compile(r"\bSurface [A-Z](?:\.\d)?\b"),
        "Letter-only `Surface X` label. Phase 5 verification surfaces use descriptive "
        "names (e.g., `Purity Boundary Audit`, `Mutation Testing`, `Fuzz Testing`, "
        "`Proof Execution`) per the Review 78 rename. Inline the descriptive name "
        "instead of the letter.",
    ),
    (
        re.compile(r"\bPath [A-Z]\b"),
        "Letter-only `Path X` label (typically from AskUserQuestion options). Rename to "
        "a descriptive identifier that names the substantive choice (e.g., `Path "
        "active-mitigation` → `active mitigation: reject control-char tags at import`).",
    ),
    (
        re.compile(r"\bOption [A-Z]\b"),
        "Letter-only `Option X` label. Rename to a descriptive identifier per the same "
        "rule as `Path X`.",
    ),
]


# Path filters. The hook scans markdown files in three scope classes:
# (1) user-facing project artifacts, (2) project audit-trail artifacts, (3) suite-side
# artifacts. All other files are skipped.
IN_SCOPE_PATTERNS = [
    re.compile(r"^[^/]+/README\.md$"),  # repo-root READMEs
    re.compile(r".*/README\.md$"),
    re.compile(r".*/DESIGN\.md$"),
    re.compile(r".*/TODO\.md$"),
    re.compile(r".*/PROCESS\.md$"),
    re.compile(r".*/CHANGELOG\.md$"),
    re.compile(r".*/COMPATIBILITY\.md$"),
    re.compile(r".*/INSTALL-VERIFICATION\.md$"),
    re.compile(r".*/manual-tests/[^/]+\.md$"),
    re.compile(r".*/vsdd-suite/.*\.md$"),
    re.compile(r"^vsdd-suite/.*\.md$"),
]


def is_in_scope(path: str) -> bool:
    """Check if path is in the scope this hook enforces."""
    for p in IN_SCOPE_PATTERNS:
        if p.search(path):
            return True
    return False


# Bypass marker: `<!-- hook-bypass: <rationale> -->` in the first 5 lines of the file.
BYPASS_RE = re.compile(r"<!--\s*hook-bypass:\s*.*?-->", re.IGNORECASE)


def has_bypass(content: str) -> bool:
    """Check if the file has a hook-bypass marker in the first 5 lines."""
    head = "\n".join(content.splitlines()[:5])
    return bool(BYPASS_RE.search(head))


def strip_verbatim_blocks(content: str) -> str:
    """Blank out fenced code blocks + blockquoted lines so verbatim content
    (commands, transcripts, historical citations) is not scanned. Line numbers
    are preserved.
    """
    lines = content.split("\n")
    in_fence = False
    out = []
    for line in lines:
        stripped = line.strip()
        if stripped.startswith("```"):
            in_fence = not in_fence
            out.append("")  # blank out the fence line itself
            continue
        if in_fence:
            out.append("")  # blank out the fenced content
            continue
        if line.lstrip().startswith(">"):
            out.append("")  # blank out the blockquote line
            continue
        out.append(line)
    return "\n".join(out)


def check_file(path: Path) -> List[str]:
    """Apply forbidden-letter-cluster patterns to one file."""
    violations: List[str] = []
    content = path.read_text(encoding="utf-8")

    if has_bypass(content):
        return []  # bypass marker — skip; the bypass is itself a finding for the next review

    scannable = strip_verbatim_blocks(content)
    lines = scannable.split("\n")

    for line_idx, line in enumerate(lines, start=1):
        for pattern, rationale in FORBIDDEN_PATTERNS:
            for m in pattern.finditer(line):
                violations.append(
                    f"{path}:{line_idx}: forbidden letter-label `{m.group(0)}` "
                    f"in audit-trail or user-facing artifact. {rationale}"
                )
                break  # one violation per line per pattern; don't double-report

    return violations


def main(argv: List[str]) -> int:
    files = argv[1:]
    if not files:
        return 0

    all_violations: List[str] = []
    for f in files:
        if not is_in_scope(f):
            continue
        path = Path(f)
        if not path.exists():
            continue
        all_violations.extend(check_file(path))

    if all_violations:
        print(
            "check-no-letter-clusters: letter-only labels detected in audit-trail / user-facing prose:",
            file=sys.stderr,
        )
        for v in all_violations:
            print(f"  {v}", file=sys.stderr)
        print(
            "\nPer the Avoid-lettering-and-abbreviation-standards discipline "
            "(originally Review 78; recurring across PR #38, PR #44, PR #52): every "
            "thematic grouping carries a descriptive name as its primary identifier. "
            "Letter labels (A/B/C, Roman numerals) are not acceptable as section headers, "
            "cluster identifiers, or option labels. To bypass for a deliberate "
            "out-of-pattern entry (e.g., a historical-narrative block citing retired "
            "letter labels), add `<!-- hook-bypass: <rationale> -->` within the first 5 "
            "lines of the file; bypasses are themselves findings for the next "
            "registry-walk review.",
            file=sys.stderr,
        )
        return 1

    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
