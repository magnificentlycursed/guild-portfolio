#!/usr/bin/env python3
"""check-changelog-currency — when a project has both CHANGELOG.md and a
.changelog-required marker file at the same directory level, require that
staged source changes in the project tree are accompanied by a CHANGELOG.md
update in the same commit.

Mechanizes the G-129 discipline: ITC's TW R6, R7, R8, R9, R10, R11 all
caught CHANGELOG.md staleness at layer-close commits — six consecutive layer
closes where the layer-closure commit shipped without a CHANGELOG entry.
Operator (PROCESS.md L6): "Documentation staleness definitely needs some
sort of process check." TW R8 proposed the fix in-project; G-129 promotes
the pattern to a suite-shipped hook.

Behavior:

  Per staged source file (`.rs`, `.ts`, `.tsx`, `.js`, `.jsx`, `.py`,
  `.go`, `.md`, `.sh`, `.yaml`, `.yml`, `.toml`), walk up from the file's
  directory toward the repo root looking for the nearest CHANGELOG.md.
  If that directory ALSO contains a `.changelog-required` marker file
  (zero-byte flag), the CHANGELOG.md must be in the staged-files set
  for the commit. Fail the commit if any source change has a
  marker-required CHANGELOG.md that is missing from the staged set.

  Projects opt in to the discipline by `touch .changelog-required` in
  the directory that holds their CHANGELOG.md. Projects without the
  marker are not enforced — the hook is methodology-neutral about
  CHANGELOG discipline absent explicit opt-in.

Bypass:

  Operator can split the commit (some changes worth a CHANGELOG entry,
  some not), or add a brief "No-op refactor — no changelog entry needed
  per [reason]" line to CHANGELOG.md and stage it. The standard
  pre-commit SKIP env var bypass (`SKIP=check-changelog-currency git
  commit ...`) works for emergency cases but should not be the default
  posture — it bypasses the discipline G-129 was designed to enforce.

Filename note: kept `.sh` extension for parity with the other suite
hooks (`check-review-log-anonymization.sh`, `check-crosslink-references.py`),
even though the implementation is Python. The shebang routes to python3.
"""

import re
import subprocess
import sys
from pathlib import Path

# File extensions considered "source" for changelog-currency purposes.
# Includes documentation extensions (.md, .sh) so that the suite's own
# documentation-only changes also enforce CHANGELOG updates.
SOURCE_EXTENSIONS = frozenset({
    ".rs", ".ts", ".tsx", ".js", ".jsx", ".py", ".go",
    ".md", ".sh", ".yaml", ".yml", ".toml",
})

# CHANGELOG filename (case-sensitive — Markdown convention is uppercase).
CHANGELOG_NAMES = ("CHANGELOG.md", "Changelog.md", "changelog.md")

# Opt-in marker file.
MARKER_NAME = ".changelog-required"


def find_changelog_with_marker(start: Path, repo_root: Path) -> Path | None:
    """Walk up from `start` toward `repo_root` looking for the nearest directory
    that contains BOTH a CHANGELOG.md and a .changelog-required marker.
    Returns the path to that CHANGELOG.md, or None if not found."""
    cur = start.resolve() if start.exists() else start.parent.resolve()
    if start.is_file():
        cur = start.parent.resolve()
    repo_root = repo_root.resolve()
    while True:
        for name in CHANGELOG_NAMES:
            candidate_cl = cur / name
            candidate_marker = cur / MARKER_NAME
            if candidate_cl.is_file() and candidate_marker.is_file():
                return candidate_cl
        if cur == repo_root or cur == cur.parent:
            return None
        cur = cur.parent


def main(argv: list[str]) -> int:
    files = [Path(p) for p in argv[1:]]
    if not files:
        return 0

    try:
        repo_root = Path(
            subprocess.check_output(
                ["git", "rev-parse", "--show-toplevel"],
                stderr=subprocess.DEVNULL,
            ).decode().strip()
        )
    except (subprocess.CalledProcessError, FileNotFoundError):
        print("check-changelog-currency: not in a git repo; skipping (G-129 hook)")
        return 0

    # Query the FULL staged-files set from git rather than relying on argv.
    # Pre-commit chunks files across multiple hook invocations to avoid argv
    # length limits; argv only contains the current chunk, not the whole
    # commit's staged set. CHANGELOG.md may be in a different chunk than the
    # source file that triggered the expectation, so we must consult git
    # directly to know what the commit-as-a-whole stages.
    try:
        staged_list = subprocess.check_output(
            ["git", "diff", "--cached", "--name-only"],
            stderr=subprocess.DEVNULL,
        ).decode().splitlines()
    except (subprocess.CalledProcessError, FileNotFoundError):
        # If we can't query staged files, fall back to argv. This may
        # produce false positives under pre-commit chunking, but is safer
        # than skipping the check entirely.
        staged_list = [str(f) for f in files]
    staged_resolved = {(repo_root / p).resolve() for p in staged_list}

    # Group violations: map CHANGELOG.md path -> list of source files
    # that triggered the expectation.
    violations: dict[Path, list[Path]] = {}
    for f in files:
        if f.suffix.lower() not in SOURCE_EXTENSIONS:
            continue
        # The CHANGELOG.md file itself is never a "source change."
        if f.name in CHANGELOG_NAMES:
            continue
        # The marker file is itself never a "source change."
        if f.name == MARKER_NAME:
            continue
        changelog = find_changelog_with_marker(f, repo_root)
        if changelog is None:
            # No opt-in marker in this file's tree. Skip.
            continue
        if changelog.resolve() in staged_resolved:
            # CHANGELOG is staged. Satisfied.
            continue
        violations.setdefault(changelog, []).append(f)

    if not violations:
        return 0

    print("check-changelog-currency: source changes staged without corresponding CHANGELOG.md update\n")
    for changelog, source_files in violations.items():
        try:
            rel_cl = changelog.relative_to(repo_root)
        except ValueError:
            rel_cl = changelog
        print(f"  Expected to be staged: {rel_cl}")
        print(f"  Source changes that triggered this expectation (marker at {rel_cl.parent}/{MARKER_NAME}):")
        for sf in source_files:
            print(f"    - {sf}")
        print()
    print("Per G-129: source changes in a tree with both CHANGELOG.md and the")
    print(f"`{MARKER_NAME}` opt-in marker must include a CHANGELOG.md update")
    print("in the same commit. Either:")
    print(f"  (a) Add an entry for these changes to the CHANGELOG.md above and stage it, or")
    print("  (b) Split the commit if some changes belong in a separate logical unit, or")
    print("  (c) If the change genuinely needs no changelog entry (a no-op refactor, a")
    print(f"      comment fix, etc.), add a brief 'No-op — no changelog entry needed per")
    print("      [reason]' line to the CHANGELOG.md and stage it (preserves the audit trail).")
    print()
    print(f"To stop enforcing for this project, `rm {MARKER_NAME}` in the project's CHANGELOG directory.")
    return 1


if __name__ == "__main__":
    sys.exit(main(sys.argv))
