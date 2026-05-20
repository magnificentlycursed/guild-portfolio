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
Review 68 / 73 suite-review hook. Extended Review 77 (2026-05-20) with
the lifecycle-field checks: every non-Hallucinated finding has
`**Owner:** <domain-slug>` (or the `### Raised to SO` shorthand);
every Resolved finding has `**Validator:** <domain-slug | *self* —
<rationale>>`; `**Validator:** *self*` requires a substantive
rationale per the strict self-validation policy (Portfolio Assessment
blanket-allowlisted at the domain level); `**Status:**` values are in
the {raised, assigned, fix-landed, validated} set. Forward-only
threshold for the lifecycle fields is 2026-05-21 (day-after-Review-77-
adoption) — separate from the 2026-05-20 Review 74 threshold so
pre-Review-77 entries that comply with Review 74 don't fail under
Review 77's stricter rules.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path
from typing import Dict, List, Set

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

# Forward-only date threshold (Review 74 enforcement floor — applies to
# Summary / Coordination / classification-heading / dim-reference checks).
ENFORCEMENT_THRESHOLD = "2026-05-20"

# Forward-only date threshold for Review 77 fields (Owner / Status /
# Validator / strict self-validation). Separate threshold so pre-Review-77
# entries with the Review 74-only convention don't fail under Review 77's
# stricter rules.
LIFECYCLE_FIELDS_THRESHOLD = "2026-05-21"

# Known domain slugs (per `suite-development.md` § Domain slug convention)
# — the canonical set against which Owner / Validator field values are
# validated. Identical to DOMAIN_CLASSIFICATIONS.keys() but kept separate
# for readability + future divergence.
KNOWN_DOMAIN_SLUGS = frozenset(DOMAIN_CLASSIFICATIONS.keys()) | {"documentation-reviewer"}

# Domains blanket-allowlisted for `**Validator:** *self*` per
# `suite-development.md` § Validation loop discipline. Portfolio Assessment's
# entire classification universe is introspective (Demonstrated / Partial /
# Absent / Hallucinated — none are defects with cross-domain validators);
# the domain-level rationale is documented once in PORTFOLIO-ASSESSMENT-
# REVIEW.md and covers all findings under the domain.
SELF_VALIDATION_BLANKET_ALLOWLIST = {"portfolio-assessment"}

# Valid `**Status:**` values per Review 77 sub-state lifecycle.
VALID_STATUS_VALUES = ("raised", "assigned", "fix-landed", "validated")

# Substantive-rationale pattern for `**Validator:** *self* — <rationale>`
# (per the strict-self-validation policy). The rationale must be a
# non-empty string after the `—` (em-dash) AND not match any of the
# placeholder patterns below.
SELF_VALIDATION_PLACEHOLDER_PATTERNS = (
    re.compile(r"^\s*$"),
    re.compile(r"^\s*TBD\s*$", re.IGNORECASE),
    re.compile(r"^\s*N/?A\s*$", re.IGNORECASE),
    re.compile(r"^\s*(no pair available|none|todo)\s*\.?\s*$", re.IGNORECASE),
)


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

    # Checks 6+ — Review 77 lifecycle fields (Owner / Status / Validator /
    # Blocked by). Gated on a separate forward-only threshold so
    # pre-Review-77 entries that comply with Review 74 don't fail under
    # the stricter Review 77 rules.
    if review_date < LIFECYCLE_FIELDS_THRESHOLD:
        return failures

    failures.extend(_check_lifecycle_fields(entry_lines, header_idx, path, domain_slug, review_n))

    return failures


def _check_lifecycle_fields(
    entry_lines: List[str],
    header_idx: int,
    path: Path,
    domain_slug: str,
    review_n: str,
) -> List[str]:
    """Apply Review 77 lifecycle-field checks per finding within an entry.

    Walks the entry looking for finding-header lines; for each finding,
    determines its classification (from the most recent `### <heading>`
    above it) and the field values that follow (Owner / Status /
    Blocked by / Validator), and validates them against the
    `suite-development.md` § Validation loop discipline rules.
    """
    failures: List[str] = []
    current_classification = None

    for k, line in enumerate(entry_lines):
        m_h = CLASSIFICATION_HEADING.match(line)
        if m_h:
            heading_text = m_h.group(1).strip()
            current_classification = re.sub(r"\s*\(.+\)\s*$", "", heading_text)
            continue

        if not line.startswith("**Finding ") and not line.startswith("**G-"):
            continue
        m_f = (
            FINDING_HEADER.match(line)
            or FINDING_HEADER_LEGACY.match(line)
            or FINDING_HEADER_ERRATA.match(line)
        )
        if not m_f:
            continue  # Heading-form validity is the suite-review hook's job.

        # Hallucinated findings are exempt from the lifecycle field
        # requirements (the finding didn't apply, so the lifecycle
        # doesn't apply).
        if current_classification == "Hallucinated":
            continue

        # Collect the finding body — runs from this header line until
        # the next finding header, classification heading, horizontal
        # rule, or entry end.
        body_start = k + 1
        body_end = len(entry_lines)
        for j in range(body_start, len(entry_lines)):
            nxt = entry_lines[j]
            if (
                nxt.startswith("**Finding ")
                or nxt.startswith("**G-")
                or nxt.startswith("### ")
                or nxt.startswith("#### ")
                or nxt.startswith("---")
            ):
                body_end = j
                break
        body_lines = entry_lines[body_start:body_end]

        def find_field(field_name: str) -> "tuple[int, str] | None":
            """Find a `**Field:**` line in the finding body and return its
            (line-offset-within-body, value-after-field-marker) tuple,
            or None if absent. Match anywhere in the line so a body that
            embeds the field in prose (e.g., quoting another finding)
            doesn't false-positive — the marker must START a line."""
            marker = f"**{field_name}:**"
            for bj, bline in enumerate(body_lines):
                if bline.startswith(marker):
                    value = bline[len(marker):].strip()
                    return (bj, value)
            return None

        owner = find_field("Owner")
        status = find_field("Status")
        validator = find_field("Validator")
        # `**Blocked by:**` is optional and not required-presence-checked;
        # a future enhancement would resolve the cited anchor and refuse
        # to close this finding if the blocker is still Open. Lookup
        # deferred to that enhancement.

        # Detect `### Raised to SO` sub-heading covering this finding —
        # walk backward from the finding header to the most recent
        # sub-heading.
        raised_to_so = False
        for j in range(k - 1, -1, -1):
            prior = entry_lines[j]
            if prior.startswith("**Finding ") or prior.startswith("**G-"):
                break  # Hit a prior finding header; no Raised-to-SO covers us
            if prior.startswith("### "):
                if prior.startswith("### Raised to SO"):
                    raised_to_so = True
                break

        # Check 6: Owner field required for non-Hallucinated findings
        # (Raised-to-SO shorthand satisfies the requirement implicitly —
        # Raised-to-SO is equivalent to `**Owner:** solution-owner`).
        if owner is None and not raised_to_so:
            failures.append(
                f"{path}:{header_idx + k + 1}: Review {review_n} finding "
                f"missing required `**Owner:** <domain-slug>` field under "
                f"classification {current_classification!r} (Review 77 § "
                f"Validation loop discipline; the `### Raised to SO` "
                f"sub-heading is also accepted as Owner-equivalent)"
            )

        # Check 7: Owner value is a known domain slug.
        if owner is not None:
            owner_value = owner[1]
            if owner_value not in KNOWN_DOMAIN_SLUGS:
                failures.append(
                    f"{path}:{header_idx + body_start + owner[0] + 1}: "
                    f"Review {review_n} `**Owner:**` value "
                    f"{owner_value!r} not in known domain-slug set "
                    f"(known: {', '.join(sorted(KNOWN_DOMAIN_SLUGS))})"
                )

        # Check 8: Validator field required for Resolved findings.
        if current_classification == "Resolved" and validator is None:
            failures.append(
                f"{path}:{header_idx + k + 1}: Review {review_n} Resolved "
                f"finding missing required `**Validator:**` field "
                f"(Review 77 § Validation loop discipline; use a domain "
                f"slug for cross-domain validation OR `*self* — "
                f"<rationale>` for self-validation per the strict policy)"
            )

        # Check 9: Validator value — domain slug OR `*self* — rationale`.
        if validator is not None:
            v_value = validator[1]
            if v_value.startswith("*self*"):
                # Strict self-validation check. Portfolio Assessment is
                # blanket-allowlisted; for other domains, the rationale
                # following `*self*` must be substantive.
                if domain_slug in SELF_VALIDATION_BLANKET_ALLOWLIST:
                    pass  # Blanket allowlist; no rationale required.
                else:
                    # Extract the rationale (after `*self*` and an em-dash
                    # or hyphen separator).
                    rationale = re.sub(r"^\*self\*\s*[—-]?\s*", "", v_value)
                    is_placeholder = any(
                        p.match(rationale)
                        for p in SELF_VALIDATION_PLACEHOLDER_PATTERNS
                    )
                    if is_placeholder:
                        failures.append(
                            f"{path}:{header_idx + body_start + validator[0] + 1}: "
                            f"Review {review_n} `**Validator:** *self*` "
                            f"lacks substantive rationale (saw: "
                            f"{rationale.strip()!r}). Per Review 77 strict "
                            f"self-validation policy, name WHY no "
                            f"cross-domain validator applies."
                        )
            elif v_value not in KNOWN_DOMAIN_SLUGS:
                failures.append(
                    f"{path}:{header_idx + body_start + validator[0] + 1}: "
                    f"Review {review_n} `**Validator:**` value "
                    f"{v_value!r} not in known domain-slug set and not "
                    f"a `*self*` form (known: "
                    f"{', '.join(sorted(KNOWN_DOMAIN_SLUGS))})"
                )

        # Check 10: Status value (if present) is in the valid set.
        if status is not None:
            s_value = status[1]
            # Strip a trailing rationale clause (some Status values
            # legitimately carry context, e.g., `**Status:** assigned
            # (waiting on SO ratification)`).
            s_root = re.sub(r"\s*\(.+\)\s*$", "", s_value).strip()
            if s_root and s_root not in VALID_STATUS_VALUES:
                failures.append(
                    f"{path}:{header_idx + body_start + status[0] + 1}: "
                    f"Review {review_n} `**Status:**` value {s_value!r} "
                    f"not in valid set "
                    f"{{{', '.join(VALID_STATUS_VALUES)}}}"
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
