#!/usr/bin/env python3
"""Mechanical anchor-link sweep — Phase 2 of Review 79 Finding 3.

Applies the anchor-link convention authored in
`suite-development.md § Anchor-link convention for cross-references`
across forward-facing markdown content.

Conservative substitution rules:
- Protect code fences, inline code, existing markdown links, HTML anchors,
  and heading lines from substitution.
- Substitute G-N (unlinked) on every occurrence — internal links are cheap.
- Substitute external software / people / governing-document mentions on
  the first per-file occurrence (per the first-mention-per-file rule).
- Substitute domain names on the first per-file occurrence.
- Substitute Phase / primer names on the first per-file occurrence.
- Review N references are NOT substituted by this script — they require
  per-Review date+time anchors that depend on the originating review-log
  file; hand-sweep those in the curated entry points if needed.

Per the G-89 forward-only constraint, this script targets only
forward-facing content. Historical files (CHANGELOG, COMPATIBILITY,
pre-Review-79 review-log entries) are excluded.

Usage:
    python3 vsdd-suite/suite-development/scripts/sweep-anchor-links.py \\
        --dry-run vsdd-suite/README.md
    python3 vsdd-suite/suite-development/scripts/sweep-anchor-links.py \\
        --apply vsdd-suite/README.md vsdd-suite/domains/**/*.md
"""

from __future__ import annotations

import argparse
import os
import re
import sys
from pathlib import Path
from typing import Dict, List, Tuple

# ---------- substitution maps ----------

EXTERNAL_LINKS: Dict[str, str] = {
    # governing documents (first occurrence per file)
    "VSDD whitepaper": "https://gist.github.com/dollspace-gay/d8d3bc3ecf4188df049d7a4726bb2a00",
    "VDD whitepaper":  "https://gist.github.com/dollspace-gay/45c95ebfb5a3a3bae84d8bebd662cc25",
    # people (first occurrence per file)
    "dollspace.gay":   "https://github.com/dollspace-gay",
    # first-party dependency
    "crosslink":       "https://github.com/forecast-bio/crosslink",
    # well-known OSS tooling
    "Python":          "https://www.python.org/",
    "Rust":            "https://www.rust-lang.org/",
    "TypeScript":      "https://www.typescriptlang.org/",
    "pytest":          "https://docs.pytest.org/",
    "ruff":            "https://github.com/astral-sh/ruff",
    "mypy":            "https://mypy-lang.org/",
    "shellcheck":      "https://www.shellcheck.net/",
    "bats-core":       "https://github.com/bats-core/bats-core",
    "Pre-commit":      "https://pre-commit.com/",
    "Claude Code":     "https://github.com/anthropics/claude-code",
}

# Domain name → (subdir, filename) under vsdd-suite/domains/
DOMAIN_SLUGS: Dict[str, Tuple[str, str]] = {
    "Software Engineer":      ("role", "SOFTWARE-ENGINEER-REVIEW.md"),
    "Solution Architect":     ("role", "SOLUTION-ARCHITECT-REVIEW.md"),
    "Solution Owner":         ("role", "SOLUTION-OWNER-REVIEW.md"),
    "Quality Engineer":       ("role", "QUALITY-ENGINEER-REVIEW.md"),
    "UX":                     ("role", "UX-REVIEW.md"),
    "Security":               ("role", "SECURITY-REVIEW.md"),
    "Accessibility":          ("role", "ACCESSIBILITY-REVIEW.md"),
    "Privacy":                ("role", "PRIVACY-REVIEW.md"),
    "Performance Engineer":   ("role", "PERFORMANCE-ENGINEER-REVIEW.md"),
    "Platform Engineer":      ("role", "PLATFORM-ENGINEER-REVIEW.md"),
    "Data Engineer":          ("role", "DATA-ENGINEER-REVIEW.md"),
    "Technical Writer":       ("role", "TECHNICAL-WRITER-REVIEW.md"),
    "Localization":           ("role", "LOCALIZATION-REVIEW.md"),
    "Red Team":               ("role", "RED-TEAM-REVIEW.md"),
    "Documentation Reviewer": ("role", "DOCUMENTATION-REVIEWER-REVIEW.md"),
    "VDD-IAR Alignment":      ("meta", "VDD-IAR-ALIGNMENT-REVIEW.md"),
    "Portfolio Assessment":   ("meta", "PORTFOLIO-ASSESSMENT-REVIEW.md"),
    "Sanity Check":           ("meta", "SANITY-CHECK-REVIEW.md"),
}

# Phase / primer name → primer filename under vsdd-suite/primers/
# Longest-key-first ordering matters at substitution time.
PRIMERS: Dict[str, str] = {
    "Phase 2a Red Gate":            "2a-red-gate.md",
    "Phase 2b Minimal Implementation": "2b-implementation.md",
    "Phase 2b implementation":      "2b-implementation.md",
    "Phase 3 Adversarial Refinement": "3-review-session.md",
    "Phase 4 Feedback Integration Loop": "4-feedback-integration.md",
    "Phase 5 Formal Hardening":     "5-formal-hardening.md",
    "Phase 5 hardening":            "5-formal-hardening.md",
    "Phase 6 Convergence":          "6-convergence.md",
    "Phase 1a+1b":                  "1ab-spec-crystallization.md",
    "Phase 1c Decomposition":       "1c-decomposition.md",
    "Phase 1c":                     "1c-decomposition.md",
    "Phase 2a":                     "2a-red-gate.md",
    "Phase 2b":                     "2b-implementation.md",
    "Phase 2c":                     "2c-refactor.md",
    "Phase 3":                      "3-review-session.md",
    "Phase 4":                      "4-feedback-integration.md",
    "Phase 5":                      "5-formal-hardening.md",
    "Phase 6":                      "6-convergence.md",
}

# ---------- exclusion patterns ----------

# Repo-relative paths to skip entirely (historical / preserved per G-89)
SKIP_EXACT = {
    "vsdd-suite/CHANGELOG.md",
    "vsdd-suite/COMPATIBILITY.md",
    "vsdd-suite/suite-development/FINDINGS-INDEX.md",
}

# Pre-Review-79 review-log entries are historical
SKIP_RE = [
    re.compile(r"vsdd-suite/suite-development/review-log/2026-04"),
    re.compile(r"vsdd-suite/suite-development/review-log/2026-05-(0[1-9]|1[0-9])-"),
    # The 2026-05-20 file has Reviews 70-80; pre-Review-79 Findings inside are
    # historical, but Reviews 79 + 80 are forward-facing. Skip the whole file
    # at the sweep level (the high-leverage Review 79/80 sweeps were done
    # by hand in PRs #35 and #36).
    re.compile(r"vsdd-suite/suite-development/review-log/2026-05-20-suite-review\.md$"),
    # Historical reference-example reviews
    re.compile(r"vsdd-suite-reference-examples/bookmark-cli-manual/vsdd-suite/review-log/2026-05-1"),
    # Pre-restructure projects
    re.compile(r"^bookmark-cli/"),
    re.compile(r"^bookmark-manager/"),
    re.compile(r"^issue-tracker-cli/"),
    # Supplements already authored under the convention
    re.compile(r"vsdd-suite/supplements/(markdown|html|css|json|yaml|toml)\.md$"),
    # SUITE-DEVELOPMENT-REVIEW.md is mostly historical Review rows; the new
    # Reviews 78/79/80 already use links per their authoring. Skip at sweep
    # level to avoid retroactive changes to historical rows.
    re.compile(r"vsdd-suite/suite-development/SUITE-DEVELOPMENT-REVIEW\.md$"),
]

# ---------- protection (mask code blocks, links, headings) ----------

PLACEHOLDER_MARKER = "\x00PROT_"
PLACEHOLDER_END = "\x00"

def protect(text: str) -> Tuple[str, List[str]]:
    """Replace protected regions with placeholders.
    Returns (masked_text, list_of_originals).
    """
    placeholders: List[str] = []

    def stash(m: re.Match) -> str:
        placeholders.append(m.group(0))
        return f"{PLACEHOLDER_MARKER}{len(placeholders) - 1}{PLACEHOLDER_END}"

    # Fenced code blocks ```...``` (multi-line)
    text = re.sub(r"```.*?```", stash, text, flags=re.DOTALL)
    # Existing markdown links [text](url)
    text = re.sub(r"\[[^\]\n]*\]\([^\)\n]*\)", stash, text)
    # Image links ![alt](src)
    text = re.sub(r"!\[[^\]\n]*\]\([^\)\n]*\)", stash, text)
    # Inline code `...`
    text = re.sub(r"`[^`\n]+`", stash, text)
    # HTML anchor tags <a id="..."></a>
    text = re.sub(r"<a [^>]*>.*?</a>", stash, text, flags=re.DOTALL)
    # Heading lines (entire line)
    text = re.sub(r"^#{1,6} .*$", stash, text, flags=re.MULTILINE)

    return text, placeholders

def restore(text: str, placeholders: List[str]) -> str:
    for i, original in enumerate(placeholders):
        text = text.replace(f"{PLACEHOLDER_MARKER}{i}{PLACEHOLDER_END}", original)
    return text

# ---------- per-file relative-path resolver ----------

def relpath_from(file_path: Path, target: str) -> str:
    """Compute relative path from file_path's directory to repo-root/target."""
    repo_root = Path(".").resolve()
    file_dir = file_path.resolve().parent
    target_abs = (repo_root / target).resolve()
    return os.path.relpath(target_abs, start=file_dir).replace("\\", "/")

# ---------- substitution functions ----------

def link_g_ids(text: str) -> str:
    """Link unlinked G-N references on every occurrence."""
    pattern = re.compile(r"\bG-(\d{2,3})\b")
    def replace(m: re.Match) -> str:
        return f"[G-{m.group(1)}](VSDD_FINDINGS_INDEX#g-{m.group(1)})"
    return pattern.sub(replace, text)

def link_external_first(text: str) -> str:
    """Link external mentions on first occurrence per file."""
    for name, url in EXTERNAL_LINKS.items():
        pattern = re.compile(r"\b" + re.escape(name) + r"\b")
        m = pattern.search(text)
        if m:
            text = text[:m.start()] + f"[{name}]({url})" + text[m.end():]
    return text

def link_domains_first(text: str) -> str:
    """Link domain names on first per-file occurrence."""
    for name, (subdir, fname) in DOMAIN_SLUGS.items():
        pattern = re.compile(r"\b" + re.escape(name) + r"\b")
        m = pattern.search(text)
        if m:
            token = f"VSDD_DOMAIN_{subdir}_{fname.replace('.md', '')}"
            link = f"[{name}]({token})"
            text = text[:m.start()] + link + text[m.end():]
    return text

def link_primers_first(text: str) -> str:
    """Link Phase / primer names on first per-file occurrence. Longest-first."""
    sorted_keys = sorted(PRIMERS.keys(), key=len, reverse=True)
    for name in sorted_keys:
        fname = PRIMERS[name]
        pattern = re.compile(r"\b" + re.escape(name) + r"\b")
        m = pattern.search(text)
        if m:
            token = f"VSDD_PRIMER_{fname.replace('.md', '')}"
            link = f"[{name}]({token})"
            text = text[:m.start()] + link + text[m.end():]
    return text

def resolve_tokens(text: str, file_path: Path) -> str:
    """Replace TOKEN placeholders with file-relative paths."""
    text = text.replace(
        "VSDD_FINDINGS_INDEX",
        relpath_from(file_path, "vsdd-suite/suite-development/FINDINGS-INDEX.md"),
    )
    for _, (subdir, fname) in DOMAIN_SLUGS.items():
        token = f"VSDD_DOMAIN_{subdir}_{fname.replace('.md', '')}"
        target = relpath_from(file_path, f"vsdd-suite/domains/{subdir}/{fname}")
        text = text.replace(token, target)
    for _, fname in PRIMERS.items():
        token = f"VSDD_PRIMER_{fname.replace('.md', '')}"
        target = relpath_from(file_path, f"vsdd-suite/primers/{fname}")
        text = text.replace(token, target)
    return text

# ---------- main ----------

def should_skip(file_path: Path) -> bool:
    rel = str(file_path).lstrip("./")
    if rel in SKIP_EXACT:
        return True
    for pat in SKIP_RE:
        if pat.search(rel):
            return True
    return False

def sweep_file(file_path: Path, *, dry_run: bool) -> Tuple[str, int]:
    """Returns (action, char-delta).
    action ∈ {"skip", "nochange", "sweep"}.
    """
    if should_skip(file_path):
        return ("skip", 0)
    if not file_path.exists() or file_path.suffix != ".md":
        return ("skip", 0)
    original = file_path.read_text()
    text, placeholders = protect(original)
    text = link_g_ids(text)
    text = link_external_first(text)
    text = link_domains_first(text)
    text = link_primers_first(text)
    text = restore(text, placeholders)
    text = resolve_tokens(text, file_path)
    if text == original:
        return ("nochange", 0)
    delta = abs(len(text) - len(original))
    if not dry_run:
        file_path.write_text(text)
    return ("sweep", delta)

def main() -> int:
    p = argparse.ArgumentParser()
    g = p.add_mutually_exclusive_group(required=True)
    g.add_argument("--dry-run", action="store_true")
    g.add_argument("--apply", action="store_true")
    p.add_argument("paths", nargs="+")
    args = p.parse_args()

    swept: List[Tuple[str, int]] = []
    skipped: List[str] = []
    unchanged: List[str] = []

    for spec in args.paths:
        for f in Path(".").glob(spec):
            if not f.is_file():
                continue
            action, delta = sweep_file(f, dry_run=args.dry_run)
            rel = str(f)
            if action == "sweep":
                swept.append((rel, delta))
            elif action == "skip":
                skipped.append(rel)
            else:
                unchanged.append(rel)

    print(f"\nSwept: {len(swept)}")
    for rel, delta in swept:
        print(f"  +{delta:>5} chars  {rel}")
    print(f"Unchanged: {len(unchanged)}")
    print(f"Skipped (excluded): {len(skipped)}")
    if args.dry_run:
        print("\n[dry-run — no files written]")
    return 0

if __name__ == "__main__":
    sys.exit(main())
