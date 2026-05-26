#!/usr/bin/env python3
"""
VSDD suite pre-commit hook — flag mechanical staleness signals in markdown.

Per [R95 F2 Design A](../suite-development/review-log/2026-05-24-suite-review.md#r95-f2)
(SO-decision 2026-05-25; hook-enforced per operator directive) +
operator-directive 2026-05-25 ("stale documentation hooks should also
apply to the upstream suite"): the staleness hook covers all markdown
files in the repository, including the suite-internal docs (primers,
suite-development.md, README, COMPATIBILITY.md, hooks docstrings).

The staleness problem class: post-Round-1 artifact-state drifts from
pre-Round-1 artifact-state (impl + spec evolve); forward-facing docs
lag behind; Round 2 IAR catches the drift (minutes-to-days later);
operator-visible cost is the lag-window between drift-introduction +
drift-catch. The PR #52 cycle's Round 1 surfaced 6+ domains converging
on README/CHANGELOG/PROCESS/manual-tests/FINDINGS-INDEX/install-
verification staleness — same recurrence-shape as letter-label problem.

Conservative-scope implementation (per the R95 F2 recommendation):
flag obvious staleness markers — phrases that implicitly claim
in-flight state when the cited state has advanced. False-positive rate
kept low; missing detections covered by the layered DR + TW + AIE
domain-prompt amendments at Round 2 IAR review (Design B).

Detection patterns:

1. **In-flight phrases** — "in flight", "in progress", "active in PR #N"
   when PR #N is merged, "to be authored", "scoped but not built",
   "Round N in progress" when Round N+1+ exists, "deferred — scoped
   only" when scope is now active.
2. **Forward-facing state claims** that contain stale-trigger words
   like "currently", "as of", "pending" — flagged with a softer signal
   (advisory rather than failure) when the document also names a
   specific date / PR / Round / commit that the author should verify.

Three-audience design (per
[§ Three-audience design principle](../suite-development/suite-development.md#three-audience-design-principle-review-80-finding-3)):

  - Suite developers — STALENESS_PATTERNS is extensible; adding a new
    pattern is additive. Each pattern includes a `name` + `regex` +
    `rationale` triple so the pattern set itself documents what it
    catches.
  - Suite users — failure messages name the matched phrase + the
    rationale + the corrective action + the scoped-bypass relief valve.
    Users hitting the failure know whether to update the doc or bypass
    for a legitimate historical reference.
  - AI agents — structured `path:line: <message>` output; the pattern
    set is part of the agent-API surface for staleness detection;
    adding patterns extends the surface, removing patterns requires a
    methodology shift per G-89.

Scope: all markdown files in the repository (matches the union of
file-sets the bypass-parsing hooks cover, per operator-directive
2026-05-25). Scoped-bypass via
`<!-- hook-bypass[check-document-staleness]: <rationale> -->` in first
5 lines — primarily for files that intentionally quote historical
in-flight state per G-89 forward-only narrative-preservation.
"""

from __future__ import annotations

import re
import sys
from dataclasses import dataclass
from pathlib import Path


HOOK_ID = "check-document-staleness"


@dataclass(frozen=True)
class StalenessPattern:
    name: str
    regex: re.Pattern[str]
    rationale: str


# Conservative-scope staleness patterns. Each pattern is high-precision
# (low false-positive rate) and named so the failure message can cite
# the pattern's rationale.
STALENESS_PATTERNS: tuple[StalenessPattern, ...] = (
    StalenessPattern(
        name="in-flight-phrase",
        regex=re.compile(
            r"\b(in[- ]flight|in[- ]progress|to be authored|scoped but not built|"
            r"pending implementation|deferred[- —]+scoped only|currently active in PR)\b",
            re.IGNORECASE,
        ),
        rationale=(
            "in-flight-phrase: implies the document references state that's "
            "still in progress. After a layer-gate close / PR merge, these "
            "phrases become stale. Update the prose to reflect current state "
            "OR bypass for a deliberate historical quotation per G-89."
        ),
    ),
    StalenessPattern(
        name="round-n-in-progress",
        regex=re.compile(
            r"\bRound\s+\d+\s+(in[- ]flight|in[- ]progress|pending|active)\b",
            re.IGNORECASE,
        ),
        rationale=(
            "round-n-in-progress: implies a specific Round is currently "
            "active. After Round N+1 entries land, the prior round's status "
            "changes to closed. Verify the cited round is still in-progress "
            "OR update to the current round's state OR bypass for a "
            "deliberate historical quotation per G-89."
        ),
    ),
    StalenessPattern(
        name="layer-n-pending",
        regex=re.compile(
            r"\bLayer\s+\d+\s+(scoped but not built|pending|to be authored|deferred)\b",
            re.IGNORECASE,
        ),
        rationale=(
            "layer-n-pending: implies a specific Layer is pending. After "
            "the Layer's first work commit, this becomes stale. Verify the "
            "cited Layer is still pending OR update to the current Layer's "
            "state OR bypass for a deliberate historical quotation per G-89."
        ),
    ),
    StalenessPattern(
        name="phase-pending",
        regex=re.compile(
            r"\bPhase\s+\d+[a-z]?\s+(scoped but not built|pending|to be authored|deferred)\b",
            re.IGNORECASE,
        ),
        rationale=(
            "phase-pending: implies a specific Phase is pending. After the "
            "Phase's commit lands, this becomes stale. Verify the cited "
            "Phase is still pending OR update to the current Phase's state "
            "OR bypass for a deliberate historical quotation per G-89."
        ),
    ),
)

# Scoped bypass marker (per AIE R2 F6 closure).
SCOPED_BYPASS_RE = re.compile(
    r"<!--\s*hook-bypass\[([^\]]+)\]:\s*.+?-->",
    re.IGNORECASE | re.DOTALL,
)


def has_bypass(content: str) -> bool:
    """Return True if a scoped hook-bypass marker naming this hook's id is in the first 5 lines."""
    head = "\n".join(content.splitlines()[:5])
    for match in SCOPED_BYPASS_RE.finditer(head):
        scoped_hooks = [h.strip() for h in match.group(1).split(",")]
        if HOOK_ID in scoped_hooks:
            return True
    return False


def strip_verbatim_blocks(content: str) -> str:
    """Blank out fenced code blocks + blockquoted lines + commit messages
    so historical citations + literal command transcripts are not scanned.
    Line numbers preserved.
    """
    lines = content.split("\n")
    in_fence = False
    out = []
    for line in lines:
        stripped = line.strip()
        if stripped.startswith("```"):
            in_fence = not in_fence
            out.append("")
            continue
        if in_fence:
            out.append("")
            continue
        if line.lstrip().startswith(">"):
            out.append("")
            continue
        out.append(line)
    return "\n".join(out)


# Scope: only the first N lines of each file are scanned for staleness
# signals. Status-claim staleness (the operator-visible-cost class R95 F2
# targets) lives at the top of files — README "Current state:" lines,
# top-of-file headers, CHANGELOG Unreleased sections. Audit-trail prose
# deeper in files describes past state per G-89 forward-only narrative-
# preservation; flagging it would produce noise. The R95 F2 recommendation:
# Design A scoped conservatively. The first-30-lines scope is the
# conservative-precision lever.
HEAD_SCAN_LINES = 30


def find_violations(content: str, path: str) -> list[str]:
    """Return one violation string per matched staleness pattern in the
    file's HEAD (first HEAD_SCAN_LINES). Past-state quotations deeper in
    the file are NOT scanned (G-89 forward-only narrative-preservation).
    """
    findings = []
    scanned = strip_verbatim_blocks(content)
    head_lines = scanned.splitlines()[:HEAD_SCAN_LINES]
    for lineno, line in enumerate(head_lines, start=1):
        for pattern in STALENESS_PATTERNS:
            m = pattern.regex.search(line)
            if m:
                matched = m.group(0)
                findings.append(
                    f"{path}:{lineno}: staleness signal `{matched}` "
                    f"({pattern.name}). {pattern.rationale}"
                )
                break  # one pattern match per line is enough
    return findings


def check_file(path: str) -> list[str]:
    try:
        content = Path(path).read_text(encoding="utf-8")
    except (OSError, UnicodeDecodeError):
        return []
    if has_bypass(content):
        return []
    return find_violations(content, path)


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
        "check-document-staleness: mechanical staleness signals detected:\n"
    )
    for v in violations:
        sys.stderr.write(f"  {v}\n")
    sys.stderr.write(
        "\nPer R95 F2 Design A SO-decision (suite-hardening PR; operator-"
        "directive 2026-05-25 suite-wide scope): the staleness hook flags "
        "mechanical signals that imply in-flight state when the cited state "
        "has likely advanced. Conservative-scope: false-positive rate kept "
        "low; missing detections covered by the layered DR + TW + AIE "
        "domain-prompt amendments at Round 2 IAR review (Design B). To "
        "bypass for a deliberate G-89 historical quotation, add "
        "`<!-- hook-bypass[check-document-staleness]: <rationale> -->` "
        "within the first 5 lines of the file.\n"
    )
    return 1


if __name__ == "__main__":
    sys.exit(main())
