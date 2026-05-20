#!/usr/bin/env python3
r"""check-project-review-discipline — verify project-level domain-review
entries conform to the governing-standard structure beyond what
check-suite-review-preamble.py already validates.

Parallel to `check-suite-review-preamble.py` (which validates preamble
fields, Source-value enumeration, finding-header forms, closer-line
presence across both suite-review and project-review files). This hook
focuses on the additional discipline that applies to PROJECT-LEVEL
review logs only, per `suite-development.md` § Governing standard for
project-level review logs:

  1. Every Review entry contains a `### Summary` section before the
     entry boundary (next `## Review N` heading, `---\n\n---` separator,
     or end-of-file).

  2. Every Review entry contains a `**Coordination:**` line (with
     `*(none)*` placeholder allowed when no cross-domain findings exist).

  3. Classification-section headings within a Review entry match the
     domain's classification universe per
     `suite-development.md` § Finding classification schemas by domain
     type. Empty classification sections use `*(none)*` placeholder so
     the structure is visible.

  4. Finding-header dim references — for findings under classification
     headings other than Hallucinated, the heading should include a
     parenthetical dim reference per the standard
     `**Finding N — Title (Dim X)**`. Hallucinated findings may omit
     the dim reference (the dim that "would have caught it if real" is
     less load-bearing).

  5. Domain slug consistency — the per-session-file filename's domain
     slug matches one of the suite's domain-slug conventions
     (suite-development.md § Domain slug convention).

Scope (per `.pre-commit-config.yaml` files-regex): project-level
review-log files only, NOT suite-review files.

  - `<project>/vsdd-suite/review-log/YYYY-MM-DD-<slug>.md`

(The suite-review files at `vsdd-suite/suite-development/review-log/`
are validated by `check-suite-review-preamble.py`; this hook
intentionally does not double-validate them.)

Bypass:

  Per-entry `<!-- hook-bypass: <rationale> -->` HTML comment in the
  first 5 lines of the entry skips that entry only. Bypasses are
  themselves findings for the next registry-walk suite review.

Forward-only:

  Entries dated 2026-05-19 or earlier (predates this hook's adoption)
  are not enforced; entries dated 2026-05-20 or later are. The date
  is parsed from the `## Review N — YYYY-MM-DD` heading.

Authored Review 74 of the VSDD Suite (2026-05-20) parallel to the
Review 68 / 73 suite-review hook.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path
from typing import Dict, List, Set, Tuple

# Domain classification universes per suite-development.md § Finding
# classification schemas by domain type. Set membership is checked
# case-insensitive (the standard prescribes Title Case headings).
DOMAIN_CLASSIFICATIONS: Dict[str, Set[str]] = {
    "quality-engineer": {"Resolved", "Deferred", "Dismissed", "Hallucinated"},
    "software-engineer": {"Resolved", "Deferred", "Dismissed", "Hallucinated"},
    "ux": {"Resolved", "Deferred", "Dismissed", "Hallucinated"},
    "solution-architect": {"Resolved", "Deferred", "Dismissed", "Hallucinated"},
    "data-engineer": {"Resolved", "Deferred", "Dismissed", "Hallucinated"},
    "platform-engineer": {"Resolved", "Deferred", "Dismissed", "Hallucinated"},
    "technical-writer": {"Resolved", "Deferred", "Dismissed", "Hallucinated"},
    "localization": {"Resolved", "Deferred", "Dismissed", "Accepted scope", "Hallucinated"},
    "performance-engineer": {"Resolved", "Deferred", "Dismissed", "Accepted limitation", "Hallucinated"},
    "accessibility": {"Resolved", "Deferred", "Dismissed", "Accepted deviation", "Hallucinated"},
    "privacy": {"Resolved", "Deferred", "Dismissed", "Accepted risk", "Hallucinated"},
    "security": {"Resolved", "Accepted risk", "Dismissed", "Hallucinated"},
    "red-team": {"Resolved", "Accepted risk", "Dismissed", "Hallucinated"},
    "solution-owner": {"Resolved", "Backlogged", "Dismissed", "Hallucinated", "Approved deviation"},
    "vdd-iar-alignment": {"Resolved", "Dismissed", "Hallucinated"},
    "portfolio-assessment": {"Demonstrated", "Partial", "Absent", "Hallucinated"},
    "observability": {"Resolved", "Deferred", "Dismissed", "Hallucinated"},
}

# Some domains use dim-first organization rather than classification-first
# headings (Portfolio Assessment per suite-development.md § Finding
# sections "Exception — Portfolio Assessment"). For these, the
# classification-section-heading check is skipped.
DOMAINS_USING_DIM_FIRST_ORGANIZATION = {"portfolio-assessment"}

# Cross-cutting `### Raised to SO` sub-heading is valid for any non-meta
# role domain log (suite-development.md § Cross-cutting classification).
RAISED_TO_SO = "Raised to SO"

# Review-entry boundary heading.
REVIEW_HEADING = re.compile(r"^## Review (\d+) — (\d{4}-\d{2}-\d{2}) (\d{2}:\d{2}Z)\s*$")

# Classification heading (level-3) regex.
CLASSIFICATION_HEADING = re.compile(r"^### (.+?)\s*$")

# Finding-header regex with optional trailing-parenthetical discipline
# reference. The standard (suite-development.md § Finding body) allows
# `(Dim 2)`, `(Dim 1, Dim 10)`, `(Rust supplement — path traversal)`,
# `(Phase 5 Surface B)`, etc. — the discipline reference takes the
# shape of a trailing `(...)` group, and the *presence* of one is the
# load-bearing requirement (not the specific wording, which legitimately
# varies across Phase 3 / Phase 5 / supplement contexts).
TRAILING_PAREN_GROUP = r"(?:\s+\(([^)]+)\))"
FINDING_HEADER = re.compile(
    r"^\*\*Finding (\d+) — (.+?)" + TRAILING_PAREN_GROUP + r"?\*\*\s*$"
)
FINDING_HEADER_LEGACY = re.compile(
    r"^\*\*G-\d+ — (.+?)" + TRAILING_PAREN_GROUP + r"?\*\*\s*$"
)
FINDING_HEADER_ERRATA = re.compile(
    r"^\*\*Finding (\d+) — (.+?)" + TRAILING_PAREN_GROUP + r"? \(added \d{4}-\d{2}-\d{2}\)\*\*\s*$"
)

# Required closing fields per Review entry.
REQUIRED_CLOSING_SUMMARY = "### Summary"
REQUIRED_CLOSING_COORDINATION = "**Coordination:**"

# Forward-only date threshold.
ENFORCEMENT_THRESHOLD = "2026-05-20"


def domain_slug_from_path(path: Path) -> str:
    """Extract the domain slug from a path like `.../review-log/YYYY-MM-DD-<slug>.md`.

    Returns the slug after the date prefix, or empty string if no
    YYYY-MM-DD prefix is found. Handles slugs with multiple hyphens
    (e.g., `solution-architect`, `vdd-iar-alignment`).
    """
    name = path.stem  # YYYY-MM-DD-<slug>
    match = re.match(r"^\d{4}-\d{2}-\d{2}-(.+)$", name)
    return match.group(1) if match else ""


def is_project_review_log(path: Path) -> bool:
    """A project review-log file lives at `<project>/vsdd-suite/review-log/...`
    and is NOT under `<suite>/suite-development/review-log/...`. The
    `suite-development/` segment is the discriminator."""
    s = str(path)
    if "/suite-development/review-log/" in s:
        return False
    return "/vsdd-suite/review-log/" in s


def find_entry_bounds(lines: List[str], header_idx: int) -> int:
    """Return the (exclusive) end-line index for the review entry that
    opens at `header_idx`. The entry runs to the next `## Review N`
    heading, the `---\n\n---` double-rule separator, or end-of-file
    (whichever comes first)."""
    for j in range(header_idx + 1, len(lines)):
        if REVIEW_HEADING.match(lines[j]):
            return j
        # Double horizontal rule separator (--- followed by blank then ---)
        if (
            j + 2 < len(lines)
            and lines[j].rstrip() == "---"
            and lines[j + 1].strip() == ""
            and lines[j + 2].rstrip() == "---"
        ):
            return j
    return len(lines)


def check_entry(
    lines: List[str],
    header_idx: int,
    end_idx: int,
    path: Path,
    domain_slug: str,
) -> List[str]:
    """Apply all per-entry checks and return a list of human-readable
    failure messages."""
    failures: List[str] = []
    header = lines[header_idx]
    m = REVIEW_HEADING.match(header)
    review_n = m.group(1)
    review_date = m.group(2)

    # Forward-only: skip entries dated before the threshold.
    if review_date < ENFORCEMENT_THRESHOLD:
        return failures

    entry_lines = lines[header_idx:end_idx]

    # Hook-bypass shortcut.
    first5 = "\n".join(entry_lines[:5])
    if "<!-- hook-bypass:" in first5:
        return failures

    entry_text = "\n".join(entry_lines)

    # Check 1: ### Summary section presence.
    if REQUIRED_CLOSING_SUMMARY not in entry_text:
        failures.append(
            f"{path}:{header_idx + 1}: Review {review_n} missing required "
            f"closing section {REQUIRED_CLOSING_SUMMARY!r}"
        )

    # Check 2: **Coordination:** line presence.
    if REQUIRED_CLOSING_COORDINATION not in entry_text:
        failures.append(
            f"{path}:{header_idx + 1}: Review {review_n} missing required "
            f"closing line {REQUIRED_CLOSING_COORDINATION!r} "
            f"(use `*(none)*` placeholder when no cross-domain findings)"
        )

    # Check 3: Classification-section headings match the domain's
    # classification universe. Skipped for dim-first-organization domains
    # and for unknown domain slugs (we don't have a universe to check
    # against).
    if (
        domain_slug in DOMAIN_CLASSIFICATIONS
        and domain_slug not in DOMAINS_USING_DIM_FIRST_ORGANIZATION
    ):
        valid_universe = DOMAIN_CLASSIFICATIONS[domain_slug] | {RAISED_TO_SO}
        for k, line in enumerate(entry_lines):
            m_h = CLASSIFICATION_HEADING.match(line)
            if not m_h:
                continue
            heading_text = m_h.group(1).strip()
            # Skip non-classification level-3 headings (Threat Model,
            # Compliance Table, Summary, structural sections). The
            # required-pre-review sections per suite-development.md
            # § Required pre-review sections.
            if heading_text in {"Summary", "Threat Model", "Compliance Table"}:
                continue
            # Strip a trailing parenthetical (some entries write
            # `### Resolved (3 findings)`).
            heading_root = re.sub(r"\s*\(.+\)\s*$", "", heading_text)
            if heading_root not in valid_universe:
                failures.append(
                    f"{path}:{header_idx + k + 1}: Review {review_n} "
                    f"classification heading {heading_text!r} not in domain "
                    f"{domain_slug!r}'s valid universe "
                    f"{{{', '.join(sorted(valid_universe))}}}"
                )

    # Check 4: Finding-header dim reference. Walk classification sections
    # and verify findings under non-Hallucinated classifications include
    # `(Dim X)` parentheticals. Hallucinated findings may omit it.
    current_classification = None
    for k, line in enumerate(entry_lines):
        m_h = CLASSIFICATION_HEADING.match(line)
        if m_h:
            heading_text = m_h.group(1).strip()
            current_classification = re.sub(r"\s*\(.+\)\s*$", "", heading_text)
            continue
        if not line.startswith("**Finding ") and not line.startswith("**G-"):
            continue
        # Match any of the three authorized heading forms.
        m_f = FINDING_HEADER.match(line) or FINDING_HEADER_LEGACY.match(line) or FINDING_HEADER_ERRATA.match(line)
        if not m_f:
            continue  # Form-validity is the suite-review hook's job.
        # The discipline-reference parenthetical is captured at the
        # trailing group of each regex. The presence of any trailing
        # `(...)` group is the load-bearing requirement; the wording
        # legitimately varies — `(Dim 2)`, `(Phase 5 Surface B)`,
        # `(Rust supplement — path traversal)`, etc. are all valid.
        m_new = FINDING_HEADER.match(line)
        m_leg = FINDING_HEADER_LEGACY.match(line)
        m_err = FINDING_HEADER_ERRATA.match(line)
        has_ref_paren = False
        if m_new:
            has_ref_paren = m_new.group(3) is not None
        elif m_leg:
            has_ref_paren = m_leg.group(2) is not None
        elif m_err:
            has_ref_paren = m_err.group(3) is not None
        if current_classification not in ("Hallucinated", None) and not has_ref_paren:
            failures.append(
                f"{path}:{header_idx + k + 1}: Review {review_n} finding "
                f"header under classification {current_classification!r} "
                f"missing trailing discipline-reference parenthetical "
                f"(e.g., `(Dim 2)`, `(Phase 5 Surface B)`, `(Rust "
                f"supplement — path traversal)`) per suite-development.md "
                f"§ Finding body (line: {line.strip()!r})"
            )

    # Check 5: domain slug recognition. If the slug doesn't match any
    # known domain, flag it (the canonical set is in
    # suite-development.md § Domain slug convention).
    if domain_slug and domain_slug not in DOMAIN_CLASSIFICATIONS:
        failures.append(
            f"{path}:1: filename slug {domain_slug!r} not in the suite's "
            f"recognized domain-slug set per suite-development.md § "
            f"Domain slug convention "
            f"(known: {', '.join(sorted(DOMAIN_CLASSIFICATIONS.keys()))})"
        )

    return failures


def check_file(path: Path) -> List[str]:
    """Check all review entries in a single project-level review-log file."""
    if not is_project_review_log(path):
        return []
    text = path.read_text()
    lines = text.splitlines()
    domain_slug = domain_slug_from_path(path)
    failures: List[str] = []
    for i, line in enumerate(lines):
        if REVIEW_HEADING.match(line):
            end = find_entry_bounds(lines, i)
            failures.extend(check_entry(lines, i, end, path, domain_slug))
    return failures


def main(argv: List[str]) -> int:
    paths = [Path(a) for a in argv[1:]]
    if not paths:
        return 0
    all_failures: List[str] = []
    for p in paths:
        all_failures.extend(check_file(p))
    if all_failures:
        print(
            "check-project-review-discipline: project-level review-log "
            "discipline violations:",
            file=sys.stderr,
        )
        for f in all_failures:
            print(f"  {f}", file=sys.stderr)
        print(
            "\nPer `suite-development.md` § Governing standard for "
            "project-level review logs: every Review entry must contain "
            "a `### Summary` section, a `**Coordination:**` line "
            "(with `*(none)*` placeholder allowed), classification-"
            "section headings matching the domain's classification "
            "universe per § Finding classification schemas by domain "
            "type, and finding headers with `(Dim X)` parentheticals "
            "for non-Hallucinated findings. To bypass for a deliberate "
            "out-of-pattern entry, add `<!-- hook-bypass: <rationale> "
            "-->` within the first 5 lines of the entry; bypasses are "
            "themselves findings for the next registry-walk review.",
            file=sys.stderr,
        )
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
