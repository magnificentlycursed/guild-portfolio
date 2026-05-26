#!/usr/bin/env python3
r"""check-suite-review-preamble — verify per-review entries in suite-review
and project-level review-log files conform to the governing-standard
preamble and finding-header forms.

Mechanizes the discipline corrections from Review 67 + the drift findings
from Review 68, and the gap-analysis-verbiage deprecation from Review 73:

  - Review 67 Finding 1: only Deferred / Open findings get tracked-registry
    anchors; Resolved-in-session findings use `**Finding N — Title**` (no
    anchor in the heading — the anchor lives in the FINDINGS-INDEX row).
  - Review 67 Finding 2: artifact identifiers are ONLY `**Finding N — Title**`
    and `**G-XX — Title**` (the latter is the legacy-registry-anchor form,
    accepted as historical-anchor walk per the Review 73 forward-only
    convention shift). Chat-shorthand identifiers (`**F1 — `, `**R1 — `,
    `**Q1 — `, `**B1 — `, `**R1 / G-173 — `) are not valid artifact forms.
  - Review 68 Finding 1: per-review preamble field `**Source:**` is
    Required-for-all-domains per `suite-development.md:275` (G-133); the
    hook fails on its absence.
  - Review 68 Findings 4 + 7: finding bodies must end with a closer line
    (`**Resolution:**` for Resolved or `**Classification:**` for others).
  - Review 73 convention shift: the legacy `G-` series is closed; new
    findings are identified by their originating `Review N Finding M`
    anchor (the row in FINDINGS-INDEX forward-only section). The
    `### New gap registered` heading is RETIRED — new findings tracked for
    future work use `### Open` or `### Deferred` per the project-aligned
    classification universe. Existing session entries that used the
    retired heading remain valid as historical records per the
    forward-only narrative-preservation policy; the hook accepts both
    the retired heading and the project-aligned classification headings
    without enforcement, since pre-2026-05-20 entries are out of
    enforcement scope and post-2026-05-20 entries follow the new shape.

Scope (per `.pre-commit-config.yaml` files-regex):

  - `vsdd-suite/suite-development/review-log/YYYY-MM-DD-suite-review.md`
  - `<project>/vsdd-suite/review-log/YYYY-MM-DD-<slug>.md`

Per-review entry format checks (each `## Review N — YYYY-MM-DD HH:MMZ`
heading opens an entry; the entry runs until the next `## Review N` or
the file end):

  1. Required preamble fields within the entry's first ~60 lines:
       `**Scope:**`
       `**Session note:**`
       `**Source:**`     (per G-133)
     Suite-review entries additionally require `**Lens:**` (per the
     Suite review entry format at `suite-development.md:417`).

  2. `**Source:**` value is one of the enumerated set:
       `domain-raised`, `director-raised`, `regression-replay`,
       `external-feedback`, `mixed`  (the last per Review 68 Finding 9).

  3. Finding-level bold headings inside the entry conform to one of
     two regexes:
       ^\*\*Finding \d+ — .+\*\*\s*$
       ^\*\*G-\d+ — .+\*\*\s*$
     Errata-style addition headers also allowed:
       ^\*\*Finding \d+ — .+ \(added \d{4}-\d{2}-\d{2}\)\*\*\s*$
     Anything else opening with `**` and ending with `**` on its own line
     that looks like a finding header (heuristic: contains ` — ` em-dash
     separator and is not a Required-preamble field) is flagged.

  4. Each finding body ends with exactly one closer line:
       `**Resolution:**`  OR
       `**Classification:**`
     Within the finding's text run (from its bold header to the next
     bold header or `---` or `### `).
     Closer absence is flagged.

  5. The `### New gap registered` classification heading is RETIRED per
     the Review 73 convention shift (post-2026-05-20). New findings
     tracked for future work use `### Open` / `### Deferred` per the
     project-aligned classification universe. The hook does not enforce
     either form on its own — Check 5 is advisory-grade by design (the
     `### New gap registered` heading is preserved as valid for
     historical entries; the project-aligned classification headings are
     valid going forward; either is acceptable structurally and the
     governing-standard prose covers the substantive distinction).

Bypass:

  This hook is advisory-grade discipline. To bypass for a deliberate
  out-of-pattern entry, the entry's narrative must include a one-line
  `<!-- hook-bypass: <rationale> -->` HTML comment within the first 5
  lines of the entry. The bypass is itself a finding for the next
  registry-walk suite review.

Forward-only:

  This hook does not enforce against entries dated 2026-05-19 or earlier
  (predates the discipline closures). Entries dated 2026-05-20 or later
  are enforced. The date is parsed from the `## Review N — YYYY-MM-DD`
  heading.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path
from typing import List

# Required preamble fields for ALL review entries.
REQUIRED_PREAMBLE_ALL = ("**Scope:**", "**Session note:**", "**Source:**")
# Required ONLY for suite-review entries (additional Lens field).
REQUIRED_PREAMBLE_SUITE = REQUIRED_PREAMBLE_ALL + ("**Lens:**",)

# Valid Source field values (per suite-development.md:275 + Review 68 Finding 9).
VALID_SOURCE_VALUES = (
    "domain-raised",
    "director-raised",
    "regression-replay",
    "external-feedback",
    "mixed",
)

# Finding header regexes (the two authorized forms + the errata-addition form).
FINDING_HEADER_NEW = re.compile(r"^\*\*Finding \d+ — .+\*\*\s*$")
FINDING_HEADER_GAP = re.compile(r"^\*\*G-\d+ — .+\*\*\s*$")
FINDING_HEADER_ERRATA = re.compile(
    r"^\*\*Finding \d+ — .+ \(added \d{4}-\d{2}-\d{2}\)\*\*\s*$"
)

# Bold-line heuristic: any line that starts with `**` + word + ` — ` and
# ends with `**` is candidate for being a finding header.
FINDING_HEADER_CANDIDATE = re.compile(r"^\*\*\S.+ — .+\*\*\s*$")

# Review-entry boundary heading.
REVIEW_HEADING = re.compile(r"^## Review (\d+) — (\d{4}-\d{2}-\d{2}) (\d{2}:\d{2}Z)\s*$")

# Bypass marker (scoped form is canonical per AIE R2 F6 SO-decision):
#   `<!-- hook-bypass[hook-id1,hook-id2]: <rationale> -->` in the first 5 lines
#   of an entry. Each hook only bypasses if its own pre-commit id is in the
#   scope list. Legacy unscoped form is REJECTED by check-no-legacy-bypass-markers.
HOOK_ID = "check-suite-review-preamble"
SCOPED_BYPASS_RE = re.compile(
    r"<!--\s*hook-bypass\[([^\]]+)\]:\s*.+?-->",
    re.IGNORECASE | re.DOTALL,
)

# Forward-only date threshold — entries dated on or after this are enforced.
ENFORCEMENT_THRESHOLD = "2026-05-20"

# Forward-only threshold for Check 6: Phase 4 routing closing field (R94 F1
# closure). Project-level Phase 3 round entries dated 2026-05-26 or later
# require a `**Phase 4 routing:**` closing field per primer 3 § Round closing.
# The threshold lets the bookmark-cli-manual PR #52 cycle entries (2026-05-24
# and 2026-05-25) stand without retroactive amendment.
PHASE_4_ROUTING_THRESHOLD = "2026-05-26"


def is_suite_review(path: Path) -> bool:
    """Suite-review-log paths follow `.../suite-development/review-log/...`."""
    return "/suite-development/review-log/" in str(path)


def is_project_review(path: Path) -> bool:
    """Project review-log paths follow `.../vsdd-suite/review-log/...` but
    are not suite-development files."""
    s = str(path)
    return "/vsdd-suite/review-log/" in s and "/suite-development/" not in s


def is_review_log(path: Path) -> bool:
    return is_suite_review(path) or is_project_review(path)


def find_entry_bounds(lines: List[str], header_idx: int) -> int:
    """Return the (exclusive) end-line index for the review entry that
    opens at `header_idx`. The entry runs to the next `## Review N` or to
    end-of-file."""
    for j in range(header_idx + 1, len(lines)):
        if REVIEW_HEADING.match(lines[j]):
            return j
    return len(lines)


def check_entry(
    lines: List[str], header_idx: int, end_idx: int, is_suite: bool, path: Path
) -> List[str]:
    """Apply all per-entry checks and return a list of human-readable
    failure messages with file:line citations."""
    failures: List[str] = []
    header = lines[header_idx]
    m = REVIEW_HEADING.match(header)
    review_n, review_date, _ = m.group(1), m.group(2), m.group(3)

    # Forward-only: skip entries dated before the enforcement threshold.
    if review_date < ENFORCEMENT_THRESHOLD:
        return failures

    entry_lines = lines[header_idx:end_idx]
    entry_text = "\n".join(entry_lines)

    # Hook-bypass shortcut: an entry with the scoped bypass marker naming this
    # hook's id in its first 5 lines is skipped (the bypass itself is flagged as
    # a finding by the next registry-walk review). Scoped form is canonical per
    # AIE R2 F6 SO-decision; legacy unscoped form is REJECTED by the separate
    # check-no-legacy-bypass-markers hook.
    first5 = "\n".join(entry_lines[:5])
    for bypass_match in SCOPED_BYPASS_RE.finditer(first5):
        scoped_hooks = [h.strip() for h in bypass_match.group(1).split(",")]
        if HOOK_ID in scoped_hooks:
            return failures

    # Check 1: required preamble fields.
    required = REQUIRED_PREAMBLE_SUITE if is_suite else REQUIRED_PREAMBLE_ALL
    # Look in the first 80 lines of the entry to allow for multi-paragraph
    # preamble fields (some entries have wrapping prose).
    preamble_window = "\n".join(entry_lines[:80])
    for field in required:
        if field not in preamble_window:
            failures.append(
                f"{path}:{header_idx + 1}: Review {review_n} missing required "
                f"preamble field {field!r} in first 80 lines of entry"
            )

    # Check 2: Source value is enumerated.
    source_line_idx = None
    for k, line in enumerate(entry_lines[:80]):
        if line.startswith("**Source:**"):
            source_line_idx = k
            break
    if source_line_idx is not None:
        source_text = entry_lines[source_line_idx]
        # Strip the field marker and look for any of the enumerated values.
        # Composite source values are allowed in `mixed` form; the line
        # must contain `mixed` OR one of the simple values.
        body = source_text[len("**Source:**"):].lower()
        if not any(v in body for v in VALID_SOURCE_VALUES):
            failures.append(
                f"{path}:{header_idx + source_line_idx + 1}: Review {review_n} "
                f"**Source:** value not in enumerated set "
                f"{{{', '.join(VALID_SOURCE_VALUES)}}}"
            )

    # Check 3 + 4: finding headers + closers.
    # Walk the entry, identify finding-header lines, then verify each
    # finding has exactly one closer between its header and the next
    # finding-header / classification-heading / horizontal-rule / entry-end.
    finding_starts = []
    has_new_gap_section = False
    for k, line in enumerate(entry_lines):
        if line.startswith("### New gap registered"):
            has_new_gap_section = True
        if FINDING_HEADER_CANDIDATE.match(line):
            # Exclude preamble-field lines that happen to match (e.g., a
            # multi-line `**Source:**` line — though preamble fields don't
            # use em-dashes in our standard, the regex shouldn't catch
            # them anyway because they end with `:**` not just `**`).
            if line.startswith(("**Scope:**", "**Lens:**", "**Session note:**",
                                "**Source:**", "**Posture:**", "**Program phase:**",
                                "**Reference:**", "**Regression check:**",
                                "**Assumption surfacing:**", "**Resolution:**",
                                "**Classification:**", "**Coordination:**",
                                "**Backlog after Review", "**Sycophancy")):
                continue
            # Validate against the two authorized forms or the errata form.
            is_authorized = (
                FINDING_HEADER_NEW.match(line)
                or FINDING_HEADER_GAP.match(line)
                or FINDING_HEADER_ERRATA.match(line)
            )
            if not is_authorized:
                failures.append(
                    f"{path}:{header_idx + k + 1}: Review {review_n} finding "
                    f"header does not match authorized forms "
                    f"`**Finding N — Title**` or `**G-XX — Title**` "
                    f"(line: {line.strip()!r})"
                )
            finding_starts.append((k, line, is_authorized))

    # Check 4: each finding has a closer.
    for i, (k, line, is_authorized) in enumerate(finding_starts):
        # Skip closer-check for unauthorized headers (we already flagged
        # them; the closer check would compound the noise).
        if not is_authorized:
            continue
        # Find the next boundary: next finding header, next H3 section,
        # next horizontal rule, or entry end.
        next_k = end_idx - header_idx
        if i + 1 < len(finding_starts):
            next_k = finding_starts[i + 1][0]
        for j in range(k + 1, min(next_k, len(entry_lines))):
            if entry_lines[j].startswith(("### ", "---")):
                next_k = j
                break
        body_text = "\n".join(entry_lines[k + 1:next_k])
        has_resolution = "**Resolution:**" in body_text
        has_classification = "**Classification:**" in body_text
        if not (has_resolution or has_classification):
            failures.append(
                f"{path}:{header_idx + k + 1}: Review {review_n} finding "
                f"missing required closer (`**Resolution:**` for Resolved "
                f"OR `**Classification:**` for everything else); body "
                f"runs to line {header_idx + next_k}"
            )

    # Check 5: retired in v2 of the hook (Review 73 convention shift).
    # The `### New gap registered` heading is no longer the canonical form
    # for newly-tracked findings; the project-aligned classification
    # headings (`### Open` / `### Deferred`) are the forward-only form.
    # Both are accepted structurally; the substantive distinction is
    # covered by the governing-standard prose in suite-development.md
    # § Suite review entry format. The hook intentionally does not enforce
    # one form over the other.

    # Check 6: Phase 4 routing closing field (R94 F1 closure; bookmark-cli-
    # manual PR #52 carry-forward). Every project-level Phase 3 round entry
    # dated 2026-05-26 or later must include a `**Phase 4 routing:**` closing
    # field per primer 3 § Round closing. The field's value points to the
    # per-domain `## Phase 4 routing — Round N` appendix (canonical shape)
    # OR uses `*(no routable findings)*` placeholder for rounds that
    # produced only Hallucinated / Resolved-in-session findings.
    #
    # Forward-only threshold (2026-05-26) lets the bookmark-cli-manual PR #52
    # cycle entries (already-merged historical records dated 2026-05-24/-25)
    # stand without retroactive amendment. Suite-review entries are exempt
    # (this hook validates project-level review-log entries via this check).
    if not is_suite and review_date >= PHASE_4_ROUTING_THRESHOLD:
        entry_text = "\n".join(entry_lines)
        if "**Phase 4 routing:**" not in entry_text:
            failures.append(
                f"{path}:{header_idx + 1}: Review {review_n} missing required "
                f"`**Phase 4 routing:** <reference | *(no routable findings)*>` "
                f"closing field per primer 3 § Round closing (R94 F1 closure). "
                f"Phase 4 routing is per-round, not per-layer; every Phase 3 "
                f"round entry records its routing decision."
            )

    return failures


def check_file(path: Path) -> List[str]:
    """Check all review entries in a single file."""
    if not is_review_log(path):
        return []
    text = path.read_text()
    lines = text.splitlines()
    is_suite = is_suite_review(path)
    failures: List[str] = []
    for i, line in enumerate(lines):
        if REVIEW_HEADING.match(line):
            end = find_entry_bounds(lines, i)
            failures.extend(check_entry(lines, i, end, is_suite, path))
    return failures


def main(argv: List[str]) -> int:
    paths = [Path(a) for a in argv[1:]]
    if not paths:
        return 0
    all_failures: List[str] = []
    for p in paths:
        all_failures.extend(check_file(p))
    if all_failures:
        print("check-suite-review-preamble: review-log discipline violations:", file=sys.stderr)
        for f in all_failures:
            print(f"  {f}", file=sys.stderr)
        print(
            "\nPer suite-development.md § Per-review entry preamble (G-133) and "
            "§ Suite review entry format (G-89 forward-only); review-log artifacts "
            "use only `**Finding N — Title**` and `**G-XX — Title**` finding "
            "header forms (Review 67 Findings 1 + 2 corrections). To bypass for "
            "a deliberate out-of-pattern entry, add "
            "`<!-- hook-bypass: <rationale> -->` within the first 5 lines of "
            "the entry; bypasses are themselves findings for the next "
            "registry-walk review.",
            file=sys.stderr,
        )
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
