#!/usr/bin/env python3
"""check-crosslink-references — validate crosslink command/flag citations in
suite documentation against the installed crosslink CLI's --help output.

Mechanizes the G-123 discipline ("before referencing an external tool's
feature, verify against that tool's governing documentation"). Registered
as G-139 in vsdd-suite/suite-development/FINDINGS-INDEX.md after two
recurrences across four sessions of the same speculation-then-late-
correction failure mode (a fictitious `--with-suite` flag attributed to
crosslink's `init` subcommand, corrected in Review 43; a fictitious
`--comment` flag attributed to the `issue close` subcommand, corrected
in Review 47) demonstrated that manual discipline alone is insufficient.

Behavior:
- Scans each staged text file for `crosslink <subcommand> ... --<flag>`
  patterns. Subcommand can be 1, 2, or 3 words; tried longest-first
  via `crosslink <tokens> --help`.
- For each (subcommand, flag) pair, validates that the flag appears in
  the help output's options.
- Fails the commit if any cited long-form flag (`--<word>`) is not in
  the help. Reports file:line, the cited subcommand+flag, and the set
  of valid flags for the subcommand.
- Skips gracefully if crosslink is not installed (CI-environment safe).
- Short-form flags (-l, -s, -p, etc.) are NOT validated in this version
  — narrow scope to long flags catches both recorded G-123 recurrences
  while keeping the regex tractable.

Filename note: kept the `.sh` extension for parity with other suite
hooks (`check-review-log-anonymization.sh`), even though the
implementation is Python. The shebang routes to python3; the .pre-commit
`language: script` integration treats it as an executable regardless of
implementation language.

Invocation: receives staged file paths as arguments via pre-commit's
`pass_filenames: true`.

Scope: validates user-facing documentation only. Historical narrative
files (CHANGELOG, COMPATIBILITY, FINDINGS-INDEX, SUITE-DEVELOPMENT-REVIEW,
review-log entries) deliberately preserve citations of past wrong commands
as audit trail per the suite's forward-only narrative-preservation policy
(Review 43 framing); the hook skips them so the audit trail is not flagged
as a defect. The scope is enforced both in `.pre-commit-config.yaml`
(efficient staged-file filtering) and in this script (correctness if
invoked manually).
"""

import re
import shutil
import subprocess
import sys

# Path substrings that mark a file as historical-narrative — the hook
# skips these because they intentionally cite past wrong commands as
# audit trail. Matched as substrings against the filepath, not regex,
# so they work whether the file is invoked by relative or absolute path.
HISTORICAL_NARRATIVE_MARKERS = (
    "/CHANGELOG.md",
    "/COMPATIBILITY.md",
    "/FINDINGS-INDEX.md",
    "/SUITE-DEVELOPMENT-REVIEW.md",
    "/review-log/",
)


def is_historical_narrative(filepath: str) -> bool:
    """Return True if the filepath matches any historical-narrative marker."""
    # Normalize: ensure leading "/" for substring matching when path is relative.
    test_path = filepath if filepath.startswith("/") else "/" + filepath
    return any(marker in test_path for marker in HISTORICAL_NARRATIVE_MARKERS)

# Subcommand → set of valid long flags (cache)
_HELP_CACHE: dict[tuple[str, ...], frozenset[str] | None] = {}


def get_valid_flags(crosslink_bin: str, tokens: tuple[str, ...]) -> frozenset[str] | None:
    """Return the set of valid long flags for `crosslink <tokens>`, or None
    if the subcommand does not exist (non-zero --help exit)."""
    if tokens in _HELP_CACHE:
        return _HELP_CACHE[tokens]
    try:
        result = subprocess.run(
            [crosslink_bin, *tokens, "--help"],
            capture_output=True,
            text=True,
            timeout=10,
            check=False,
        )
    except (subprocess.TimeoutExpired, OSError):
        _HELP_CACHE[tokens] = None
        return None
    if result.returncode != 0:
        _HELP_CACHE[tokens] = None
        return None
    flags: set[str] = set()
    for line in result.stdout.splitlines():
        # Option lines look like:
        #   "  -l, --label <LABEL>           Filter by label"
        #   "      --json                    Output as JSON"
        # The flag tokens we want are --xxx (long form). Short flags are
        # narrower scope; not validated in this version.
        stripped = line.strip()
        if not stripped.startswith("-"):
            continue
        # Split into the flag column (before any 2-space gap) and parse out --xxx
        flag_col = stripped.split("  ", 1)[0]
        for m in re.finditer(r"(?<![\w-])--[\w-]+", flag_col):
            flags.add(m.group(0))
    frozen = frozenset(flags)
    _HELP_CACHE[tokens] = frozen
    return frozen


def find_command_invocations(line: str):
    """Yield (subcommand_token_list, remainder) for each `crosslink ...`
    invocation on the line that has at least one long flag in scope."""
    # Match "crosslink" + 1-3 word tokens + remainder up to the next backtick or end.
    pattern = re.compile(
        r"crosslink((?:\s+[\w-]+){1,3})([^`\n]*)"
    )
    for match in pattern.finditer(line):
        token_part = match.group(1).strip()
        rest = match.group(2) or ""
        tokens = token_part.split()
        # Filter: only yield if there's at least one --flag in tokens or rest.
        combined = " ".join(tokens) + " " + rest
        if not re.search(r"(?<![\w-])--[\w-]+", combined):
            continue
        yield tokens, rest


def validate_file(crosslink_bin: str, filepath: str) -> list[tuple[str, int, str, str, list[str]]]:
    """Return a list of (filepath, lineno, subcommand_str, cited_flag, valid_flags_sorted)
    for each invalid flag citation in the file."""
    errors: list[tuple[str, int, str, str, list[str]]] = []
    try:
        with open(filepath, encoding="utf-8") as f:
            content = f.read()
    except (OSError, UnicodeDecodeError):
        return errors
    for lineno, line in enumerate(content.split("\n"), start=1):
        for cmd_tokens, rest in find_command_invocations(line):
            # Try longest-first to find the actual subcommand prefix.
            subcommand: tuple[str, ...] | None = None
            consumed = 0
            for n in range(min(3, len(cmd_tokens)), 0, -1):
                candidate = tuple(cmd_tokens[:n])
                if get_valid_flags(crosslink_bin, candidate) is not None:
                    subcommand = candidate
                    consumed = n
                    break
            if subcommand is None:
                # No valid subcommand prefix. Skip silently — likely a fragment.
                continue
            valid = get_valid_flags(crosslink_bin, subcommand)
            assert valid is not None
            # Scan the remaining tokens + rest for cited long flags.
            remaining = " ".join(cmd_tokens[consumed:]) + " " + rest
            for flag_match in re.finditer(r"(?<![\w-])--[\w-]+", remaining):
                cited = flag_match.group(0)
                if cited not in valid:
                    errors.append(
                        (
                            filepath,
                            lineno,
                            " ".join(["crosslink", *subcommand]),
                            cited,
                            sorted(valid),
                        )
                    )
    return errors


def main(argv: list[str]) -> int:
    crosslink_bin = shutil.which("crosslink")
    if not crosslink_bin:
        print("check-crosslink-references: crosslink not installed; skipping (G-139 hook)")
        return 0

    files = argv[1:]
    if not files:
        return 0

    all_errors: list[tuple[str, int, str, str, list[str]]] = []
    for filepath in files:
        if not filepath.endswith((".md", ".sh", ".yaml", ".yml")):
            continue
        if is_historical_narrative(filepath):
            continue
        all_errors.extend(validate_file(crosslink_bin, filepath))

    if not all_errors:
        return 0

    print("check-crosslink-references: cited crosslink flag(s) not found in --help output\n")
    for filepath, lineno, subcommand, cited, valid in all_errors:
        print(f"  {filepath}:{lineno}")
        print(f"    Cited:   `{subcommand} ... {cited}`")
        if valid:
            print(f"    Valid flags for `{subcommand}`: {', '.join(valid)}")
        else:
            print(f"    Valid flags for `{subcommand}`: (none — subcommand has no documented flags)")
        print()
    print("Per G-139: every cited crosslink long flag must match the installed CLI's --help output.")
    print("Fix the citation or update vsdd-suite/crosslink-contract.md if the surface has changed.")
    print("To bypass for an intentional speculative reference (rare), wrap the flag in a phrase")
    print("the hook will not match (e.g., quote the flag in narrative prose without preceding it")
    print("with `crosslink <subcommand>` in the same line).")
    return 1


if __name__ == "__main__":
    sys.exit(main(sys.argv))
