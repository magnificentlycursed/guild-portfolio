#!/usr/bin/env python3
"""
VSDD suite pre-commit hook — verify Phase 2a Red Gate test plan exists in
TODO.md before test files are committed.

Per [R95 F3 Design A](../suite-development/review-log/2026-05-24-suite-review.md#r95-f3)
(SO-decision 2026-05-25) + [§ Phase transition provability](../suite-development/suite-development.md#phase-transition-provability-r95-f3-closure)
in suite-development.md: the **1c → 2a transition** is the second
hook-enforced phase boundary. The antecedent state is "Layer's TODO.md
Red Gate test plan section populated"; the successor state is "first
Phase 2a Red Gate test commit per layer". This hook verifies the
antecedent state when a test-shaped file is staged.

Semantics:

  - Hook receives staged file paths from pre-commit.
  - If any test-shaped path (`tests/*.rs`, `tests/test_*.py`, etc.) is
    staged, walks up the directory tree from the test file to find a
    `TODO.md` in an ancestor directory.
  - If TODO.md found: requires it to contain a `**Red Gate test plan**`
    marker (canonical shape per primer 2a + the bookmark-cli-manual
    worked example). Marker absence = Phase 2a test-plan-backfilling
    is happening (the discipline gap R95 F3 closes).
  - If TODO.md not found in any ancestor (project doesn't use the
    per-layer TODO.md convention): the hook treats this as out-of-scope
    + passes. Suite-internal test directories are out-of-scope by this
    same logic (the suite root has no TODO.md).
  - Scoped bypass: `<!-- hook-bypass[check-red-gate-plan-precedence]: <rationale> -->`
    in the staged test file's first 5 lines OR in the project's
    TODO.md's first 5 lines.

Three-audience design (per
[§ Three-audience design principle](../suite-development/suite-development.md#three-audience-design-principle-review-80-finding-3)):

  - Suite developers — TEST_PATH_PATTERNS is extensible (add new
    language conventions); RED_GATE_MARKER_RE is the single source of
    truth for the marker shape.
  - Suite users — failure messages name the project's TODO.md location +
    the missing marker + the primer 2a reference + the scoped-bypass
    relief valve.
  - AI agents — canonical `path:line: <message>` output; the marker
    `**Red Gate test plan**` is part of the agent-API surface for
    Phase 2a attestation.

Forward-only: the hook enforces against artifacts dated 2026-05-26 or
later in spirit (no date logic in the hook itself since test files don't
carry dates; the forward-only convention is at the commit-level — pre-
existing test commits that landed before the hook's adoption are not
retroactively required to refactor).
"""

from __future__ import annotations

import re
import sys
from pathlib import Path


HOOK_ID = "check-red-gate-plan-precedence"

# Test-shaped path patterns (per-language conventions). Extensible.
TEST_PATH_PATTERNS = (
    re.compile(r"(^|/)tests/.*\.rs$"),                   # Rust integration tests
    re.compile(r"(^|/)tests/test_.*\.py$"),              # Python pytest
    re.compile(r"(^|/).*_test\.(py|ts|js|tsx|jsx)$"),    # Generic *_test.* convention
    re.compile(r"(^|/).*\.test\.(ts|js|tsx|jsx)$"),      # JS/TS .test.* convention
    re.compile(r"(^|/)__tests__/.*\.(ts|js|tsx|jsx)$"),  # JS/TS __tests__/ convention
)

# Canonical Red Gate test plan marker. Matches `**Red Gate test plan**`
# (bold-paragraph emphasis) per the bookmark-cli-manual TODO.md worked
# example + primer 2a § Output convention.
RED_GATE_MARKER_RE = re.compile(r"\*\*Red Gate test plan\*\*", re.IGNORECASE)

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


def is_test_file(path: str) -> bool:
    """Match the staged path against the test-shape patterns."""
    return any(pat.search(path) for pat in TEST_PATH_PATTERNS)


def find_project_todo(test_path: Path) -> Path | None:
    """Walk up from the test file's parent directory to find the project's
    TODO.md. Returns the first TODO.md found in an ancestor directory, or
    None if no TODO.md exists above the test path.
    """
    current = test_path.parent
    while True:
        candidate = current / "TODO.md"
        if candidate.is_file():
            return candidate
        if current.parent == current:
            return None
        current = current.parent


def check_file(test_path: str) -> list[str]:
    """Validate that the project's TODO.md (ancestor of the test path)
    contains a Red Gate test plan marker. Returns violation strings.
    """
    if not is_test_file(test_path):
        return []
    test_p = Path(test_path)
    # Check test file's own bypass marker first (covers e.g., a test file
    # that's deliberately added without a TODO.md plan for a non-VSDD
    # subdirectory).
    try:
        test_content = test_p.read_text(encoding="utf-8")
        if has_bypass(test_content):
            return []
    except (OSError, UnicodeDecodeError):
        pass
    todo_p = find_project_todo(test_p)
    if todo_p is None:
        # No TODO.md in any ancestor → project doesn't use per-layer TODO.md
        # convention; out-of-scope (e.g., the suite root's own test
        # directories, if any). Pass.
        return []
    try:
        todo_content = todo_p.read_text(encoding="utf-8")
    except (OSError, UnicodeDecodeError):
        return []
    # TODO.md's own bypass marker also satisfies (a project-wide bypass).
    if has_bypass(todo_content):
        return []
    if RED_GATE_MARKER_RE.search(todo_content):
        return []
    return [
        f"{test_path}:1: Phase 2a Red Gate test file staged but project's "
        f"TODO.md (at {todo_p}) does not contain the `**Red Gate test plan**` "
        f"marker. The R95 F3 1c → 2a transition discipline requires the Red "
        f"Gate test plan to be documented BEFORE test files are committed "
        f"(per primer 2a § Output). Add the `**Red Gate test plan**` section "
        f"to TODO.md for the layer being tested, OR bypass for a deliberate "
        f"out-of-pattern test (e.g., scaling sentinel, fuzz harness) via "
        f"`<!-- hook-bypass[check-red-gate-plan-precedence]: <rationale> -->` "
        f"in the test file's OR TODO.md's first 5 lines."
    ]


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
        "check-red-gate-plan-precedence: Phase 2a Red Gate test-plan-precedence violations:\n"
    )
    for v in violations:
        sys.stderr.write(f"  {v}\n")
    sys.stderr.write(
        "\nPer R95 F3 Design A SO-decision (suite-hardening PR): the 1c → 2a "
        "transition is hook-enforced at commit time. The antecedent state "
        "(TODO.md Red Gate test plan section populated) must precede the "
        "successor state (Phase 2a Red Gate test commits). The hook enforces "
        "the precedence at commit time; the scoped-bypass marker is the "
        "relief valve for legitimately out-of-pattern test additions.\n"
    )
    return 1


if __name__ == "__main__":
    sys.exit(main())
