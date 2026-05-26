#!/usr/bin/env python3
"""
VSDD suite pre-commit hook — verify DESIGN.md is spec-frozen before
Phase 1c decomposition begins.

Per [R95 F3 Design A](../suite-development/review-log/2026-05-24-suite-review.md#r95-f3)
(operator-directed hook-enforced phase-transition provability; SO-decision 2026-05-25)
+ [§ Phase transition provability](../suite-development/suite-development.md#phase-transition-provability-r95-f3-closure)
in suite-development.md: the **1a+1b → 1c transition** is the first
hook-enforced phase boundary. The antecedent state is "DESIGN.md complete
per primer 1ab template; no TBD markers in spec sections"; the successor
state is "TODO.md decomposition committed". This hook validates the
antecedent state at commit time.

Continuous-discipline semantics:

  - The hook validates ANY DESIGN.md when it's staged.
  - Pre-spec-frozen (authoring in progress): the project commits include
    a scoped bypass marker `<!-- hook-bypass[check-spec-frozen]: <rationale> -->`
    in the DESIGN.md's first 5 lines. Hook is bypassed; DESIGN.md may be
    incomplete during authoring.
  - Spec-frozen (transition committed): the bypass marker is REMOVED in
    the spec-frozen commit. Hook runs; DESIGN.md must be complete (all
    required sections + no `**TBD**` markers).
  - Post-spec-frozen: bypass marker absent; hook continues to enforce on
    every subsequent commit to DESIGN.md.

The transition is observable in git: the bypass-removal commit IS the
spec-frozen attestation. `git log --diff-filter=M -p -- DESIGN.md | grep
"hook-bypass\\[check-spec-frozen\\]"` shows the commit pair: removal at
spec-frozen attestation; add-back if the project re-opens spec authoring.

Three-audience design (per
[§ Three-audience design principle](../suite-development/suite-development.md#three-audience-design-principle-review-80-finding-3)):

  - Suite developers (this hook's authors) — template matches the
    check-no-legacy-bypass-markers.py pattern: scoped-bypass parsing
    (HOOK_ID + SCOPED_BYPASS_RE), self-bypass support, structured
    violation output. Extensible: adding new required sections updates
    REQUIRED_SECTIONS in one place.
  - Suite users (project teams running pre-commit) — failure messages
    name the missing section by name + cite the primer 1ab template.
    Users hitting the failure know which section to add without reading
    the hook source. The scoped-bypass mechanism is named in the failure
    message for explicit pre-spec-frozen authoring.
  - AI agents (cold-session reviewers reading audit trails + main-session
    orchestrators authoring DESIGN.md) — violation output uses the
    canonical `path:line: <message>` format the agent API commits to.
    Adding a new required section extends the API surface; renames or
    removals are forbidden under G-89 except as a documented methodology
    shift.

Scope: per-project `DESIGN.md` files. The suite's own root has no
DESIGN.md; project DESIGN.md files live at `<project>/DESIGN.md` and
`<project>/<subproject>/DESIGN.md` (e.g., bookmark-cli-manual's
DESIGN.md).
"""

from __future__ import annotations

import re
import sys
from pathlib import Path


HOOK_ID = "check-spec-frozen"

# Per primer 1ab § Output: DESIGN.md structure. These sections constitute
# the minimum complete spec at the spec-frozen attestation. Sections may
# carry richer content (e.g., per-feature subsections under Features); the
# hook validates presence only, not content depth (depth is VDD-IAR
# Alignment Dim 1's surface).
#
# Each required section maps to a set of canonical aliases — projects use
# richer / domain-appropriate section names (e.g., bookmark-cli-manual's
# `Behavioral contracts` covers Features; `Verification architecture`
# covers Testing Methodology; `Storage data classification` covers Data
# Model). The hook accepts ANY of the listed aliases as satisfying the
# required-section presence check. Extending the alias set is an additive
# methodology-shift; documented per § Phase transition provability.
REQUIRED_SECTIONS: dict[str, tuple[str, ...]] = {
    "Overview": ("Overview", "Project intent", "What this project does", "Purpose"),
    "Features": ("Features", "Behavioral contracts"),
    "Data Model": ("Data Model", "Storage format", "Storage data classification", "Data shapes"),
    "Interface": ("Interface", "Interface definitions", "Command surface"),
    "Constraints": ("Constraints", "Performance budget"),
    "Edge Cases": ("Edge Cases", "Edge case catalog"),
    "Testing Methodology": ("Testing Methodology", "Verification architecture"),
    "Out of Scope": ("Out of Scope", "Scope and non-goals", "Non-goals"),
}

# A required section is present when at least one of its aliases appears
# as an H2 heading (token-normalized match).
SECTION_HEADING_RE = re.compile(r"^##\s+(.+?)\s*$")
TBD_MARKER_RE = re.compile(r"\*\*TBD\*\*", re.IGNORECASE)

# Scoped bypass marker (per AIE R2 F6 closure):
#   `<!-- hook-bypass[check-spec-frozen]: <rationale> -->` in first 5 lines.
# Legacy unscoped form rejected by check-no-legacy-bypass-markers.
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


def find_section_headings(content: str) -> set[str]:
    """Return the set of token-normalized H2 headings present in the doc.

    Token normalization: lowercase + collapse whitespace. Lets a project
    use `## Edge cases` (lowercase) or `## Edge Case Catalog` (richer
    name) and still match against `Edge Cases` (canonical name).
    """
    headings = set()
    for line in content.splitlines():
        m = SECTION_HEADING_RE.match(line)
        if m:
            headings.add(_normalize(m.group(1)))
    return headings


def _normalize(name: str) -> str:
    return re.sub(r"\s+", " ", name.lower()).strip()


def is_required_present(required: str, aliases: tuple[str, ...], headings: set[str]) -> bool:
    """Required section is present when at least one of its aliases
    appears as an H2 heading (token-normalized substring match).
    """
    for alias in aliases:
        alias_tokens = _normalize(alias).split()
        for heading in headings:
            if all(tok in heading for tok in alias_tokens):
                return True
    return False


def find_tbd_markers(content: str) -> list[tuple[int, str]]:
    """Return list of (lineno, matched-text) for any `**TBD**` markers."""
    findings = []
    for lineno, line in enumerate(content.splitlines(), start=1):
        m = TBD_MARKER_RE.search(line)
        if m:
            findings.append((lineno, line.strip()))
    return findings


def check_file(path: str) -> list[str]:
    p = Path(path)
    if p.name != "DESIGN.md":
        return []
    try:
        content = p.read_text(encoding="utf-8")
    except (OSError, UnicodeDecodeError):
        return []
    if has_bypass(content):
        return []
    failures: list[str] = []
    headings = find_section_headings(content)
    for required, aliases in REQUIRED_SECTIONS.items():
        if not is_required_present(required, aliases, headings):
            alias_list = " | ".join(f"`## {a}`" for a in aliases)
            failures.append(
                f"{path}:1: DESIGN.md missing required section concept `{required}` "
                f"per primer 1ab § Output: DESIGN.md structure (accepted aliases: "
                f"{alias_list}). The spec-frozen attestation requires all 8 "
                f"section concepts present + no `**TBD**` markers in spec content. "
                f"Add the section OR bypass for in-flight authoring via "
                f"`<!-- hook-bypass[check-spec-frozen]: <rationale> -->` in the "
                f"file's first 5 lines (the bypass-removal commit IS the "
                f"spec-frozen attestation per § Phase transition provability)."
            )
    for lineno, line in find_tbd_markers(content):
        failures.append(
            f"{path}:{lineno}: DESIGN.md contains `**TBD**` marker at "
            f"spec-frozen attestation. Resolve the TBD OR bypass for in-flight "
            f"authoring via "
            f"`<!-- hook-bypass[check-spec-frozen]: <rationale> -->`. Line: {line!r}"
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
        "check-spec-frozen: DESIGN.md spec-frozen attestation violations:\n"
    )
    for v in violations:
        sys.stderr.write(f"  {v}\n")
    sys.stderr.write(
        "\nPer R95 F3 Design A SO-decision (suite-hardening PR; bookmark-cli-"
        "manual PR #52 follow-up): the 1a+1b → 1c transition is hook-enforced "
        "at commit time. The antecedent state (DESIGN.md complete per primer "
        "1ab template + no TBD markers) is verified here; the successor state "
        "(TODO.md decomposition committed) is the natural next commit. The "
        "scoped-bypass marker is the relief valve for in-flight authoring; "
        "its removal IS the spec-frozen attestation.\n"
    )
    return 1


if __name__ == "__main__":
    sys.exit(main())
