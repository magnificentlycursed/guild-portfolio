#!/usr/bin/env python3
"""
VSDD suite pre-commit hook — verify all Phase 4 routing records exist
before Phase 5 surface entries land per layer.

Per [R95 F3 Design A](../suite-development/review-log/2026-05-24-suite-review.md#r95-f3)
(SO-decision 2026-05-25) + [§ Phase transition provability](../suite-development/suite-development.md#phase-transition-provability-r95-f3-closure)
in suite-development.md: the **4 → 5 transition** is the fifth
hook-enforced phase boundary. The antecedent state is "every Round N has
a Phase 4 routing record per layer"; the successor state is "first Phase
5 review entry per layer with `**Phase 5 surface:**` preamble tag".
This hook verifies the antecedent state when a Phase 5 entry is staged.

Semantics:

  - Hook receives staged file paths from pre-commit.
  - For each staged path that's a project review log (matches
    `<project>/vsdd-suite/review-log/YYYY-MM-DD-<slug>.md`), scans the
    file for newly-added entries marked with `**Phase 5 surface:**`
    preamble tag (per [§ Phase transition provability matrix](../suite-development/suite-development.md#phase-transition-provability-r95-f3-closure)).
  - For each such Phase 5 entry, infers the layer from the surrounding
    review-log context (the file's path + the entry's nearest layer
    reference in preamble prose).
  - For the inferred layer, walks the sibling review-log files looking
    for ALL Round N entries' `**Phase 4 routing:**` closing fields. If
    any Round N entry is missing the closing field (or the per-domain
    `## Phase 4 routing — Round N` appendix), fail: Phase 5 cannot land
    before Phase 4 routing is complete.
  - Scoped bypass on the Phase 5 entry's first 5 lines OR on the file.

The hook is the most complex of the R95 F3 hook set because it requires
cross-file walking (Phase 5 entry's file + sibling Phase 3 round files).
The current implementation is conservative: it validates the Phase 5
entry's own preamble + flags the absence of a Phase-4-routing-evidence
reference; full cross-file walking is deferred to a future hook revision
once empirical patterns from the next reference-example cycle inform
the design.

Three-audience design (per
[§ Three-audience design principle](../suite-development/suite-development.md#three-audience-design-principle-review-80-finding-3)):

  - Suite developers — PHASE_5_TAG_RE + PHASE_4_ROUTING_FIELD_RE are
    the single sources of truth for the markers; extending to new
    surface tags is additive.
  - Suite users — failure messages name the Phase 5 entry + the missing
    Phase 4 routing references + the scoped-bypass relief valve.
  - AI agents — structured `path:line: <message>` output; the
    `**Phase 5 surface:**` tag + the `**Phase 4 routing:**` field are
    part of the agent-API surface per [§ Agent-API surface](../suite-development/suite-development.md#agent-api-surface-review-80-finding-3).
"""

from __future__ import annotations

import re
import sys
from pathlib import Path


HOOK_ID = "check-phase-5-routing-precedence"

# Phase 5 surface preamble tag (per § Phase transition provability matrix).
# Example: `**Phase 5 surface:** Purity Boundary Audit`
PHASE_5_TAG_RE = re.compile(r"^\*\*Phase 5 surface:\*\*", re.MULTILINE)

# Phase 4 routing closing field (R94 F1 / per-domain appendix canonical shape).
PHASE_4_ROUTING_FIELD_RE = re.compile(r"\*\*Phase 4 routing:\*\*")

# Project-level review-log path pattern.
PROJECT_REVIEW_LOG_RE = re.compile(
    r"(?P<project_root>.+?)/vsdd-suite/review-log/\d{4}-\d{2}-\d{2}-(?P<slug>[a-z-]+)\.md$"
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


def find_phase_5_entries(content: str) -> list[int]:
    """Return line numbers (1-indexed) where a `**Phase 5 surface:**` tag appears."""
    return [
        idx + 1 for idx, line in enumerate(content.splitlines())
        if PHASE_5_TAG_RE.match(line)
    ]


def find_phase_4_routing_references(content: str) -> int:
    """Count `**Phase 4 routing:**` references in the file (closing fields +
    per-domain appendix references). Used to detect whether the project's
    Phase 4 routing record artifacts exist when a Phase 5 entry is added.
    """
    return len(PHASE_4_ROUTING_FIELD_RE.findall(content))


def check_file(path: str) -> list[str]:
    """Validate Phase 5 entries cite Phase 4 routing precedence."""
    m = PROJECT_REVIEW_LOG_RE.match(path)
    if not m:
        return []
    p = Path(path)
    try:
        content = p.read_text(encoding="utf-8")
    except (OSError, UnicodeDecodeError):
        return []
    if has_bypass(content):
        return []
    phase_5_lines = find_phase_5_entries(content)
    if not phase_5_lines:
        # No Phase 5 entries in this file; out-of-scope.
        return []
    # Phase 5 entries exist. Validate: the file (or a sibling review-log
    # file in the same project) contains Phase 4 routing references for
    # the layer's prior rounds. Conservative implementation: count
    # **Phase 4 routing:** references in this file + sibling project
    # review-log files. If zero references exist → likely Phase-4-routing
    # not done before Phase 5 → fail.
    project_review_log_dir = p.parent
    phase_4_count = 0
    for sibling in project_review_log_dir.glob("*.md"):
        try:
            sibling_content = sibling.read_text(encoding="utf-8")
        except (OSError, UnicodeDecodeError):
            continue
        phase_4_count += find_phase_4_routing_references(sibling_content)
    failures: list[str] = []
    if phase_4_count == 0:
        for phase_5_lineno in phase_5_lines:
            failures.append(
                f"{path}:{phase_5_lineno}: Phase 5 surface entry "
                f"(`**Phase 5 surface:**` tag at line {phase_5_lineno}) added "
                f"but NO `**Phase 4 routing:**` references found in this file "
                f"OR any sibling project review-log file at {project_review_log_dir}. "
                f"The R95 F3 4 → 5 transition discipline requires all Round N "
                f"Phase 4 routing records to exist BEFORE Phase 5 entries land. "
                f"Add the per-domain `## Phase 4 routing — Round N` appendices "
                f"per primer 4-feedback-integration, OR bypass for "
                f"legitimately-out-of-pattern Phase 5 entry via "
                f"`<!-- hook-bypass[check-phase-5-routing-precedence]: <rationale> -->` "
                f"in the file's first 5 lines."
            )
    return failures


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
        "check-phase-5-routing-precedence: Phase 5 cannot land before Phase 4 routing complete:\n"
    )
    for v in violations:
        sys.stderr.write(f"  {v}\n")
    sys.stderr.write(
        "\nPer R95 F3 Design A SO-decision (suite-hardening PR): the 4 → 5 "
        "transition is hook-enforced at commit time. The antecedent state "
        "(every Round N has a Phase 4 routing record per layer) must precede "
        "the successor state (first Phase 5 review entry per layer with "
        "**Phase 5 surface:** preamble tag). The hook's conservative "
        "implementation counts Phase 4 routing references in the project's "
        "review-log directory; a future revision will walk per-layer per-round.\n"
    )
    return 1


if __name__ == "__main__":
    sys.exit(main())
