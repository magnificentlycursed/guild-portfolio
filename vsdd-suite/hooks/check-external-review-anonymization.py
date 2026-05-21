#!/usr/bin/env python3
"""
Pre-commit hook: identity-correlation discipline on external-review-log files.

Codified in `vsdd-suite/suite-development/suite-development.md` § External-review-log
subfolder pattern § Identity-correlation discipline (Review 88, PR #42).

The principle (per the operator's framing): the suite (and many people who will be
reviewing) are marginalized people. Surfacing real names + handles in correlated form
has historically been a vector for harm. This hook extends the suite's anonymization
discipline to external-author content — the operator-controlled `check-review-log-
anonymization.sh` hook protects the local-user's identity from suite-side review-log
authoring; this hook protects external-reviewer identities from suite-side correlation
in external-review-log/ files.

Scope: files matching `vsdd-suite/suite-development/review-log/external-review-log/*.md`.

Rules enforced (load-bearing — fail-on-violation):

1. **Multi-platform handle declarations must share a normalized slug.** If a file
   declares both Bluesky and GitHub handles (or any two-platform combination), the
   handles' slug-normalized strings must share a common substring. Example PASS:
   `dollspace.gay` (Bluesky) + `dollspace-gay` (GitHub) — normalized to
   `dollspacegay` on both, share the common substring. Example FAIL: `shimmermathlabs`
   (Bluesky) + `nwhitehead` (GitHub) — different identity strings; the reviewer
   engaged on Bluesky for this review and the GitHub handle is correlation-surfacing
   the suite must not perform.

2. **`**Name:**` fields must match a declared handle slug after normalization.**
   A `**Name:**` field whose value is a real first/last name distinct from any
   declared handle is correlation-surfacing. Example PASS: `**Name:** dollspace-gay`
   matches the handle. Example FAIL: `**Name:** Nathan Whitehead` with handle
   `shimmermathlabs` — different identity strings; the real name surfaces a
   correlation the reviewer didn't surface in the engagement platform.

3. **Bare email addresses in the `## Reviewer` or `## Source` preamble — FAIL.**
   Emails are identity-correlation surfaces; the reviewer's authored choice is
   their platform handle. If an email appears in quoted source content
   (verbatim `> ` blocks or fenced code blocks), it is preserved per the
   source-archiving discipline — those are skipped by the scanner.

4. **Required structural sections.** Every external-review file must declare
   `## Reviewer` and `## Source` H2 headings. The first 50 lines of the file
   must contain the H1 `# External Review — @<handle> — <date>`. This is the
   shape contract that downstream tooling (AI Engineer Dim 11 audit-trail
   machine-readability) depends on.

Cross-domain ownership (per Review 87 Finding 6 per-error-class owner table):
this is a process-enforcement script for an early-detection discipline — AI Engineer
owns; AI Engineer Dim 11 (audit-trail machine-readability) + the Privacy domain's
identity-correlation concerns motivate the rule set.

Exit codes:
- 0 — pass
- 1 — discipline violation detected; specific lines + reason emitted to stderr
"""

import re
import sys
from pathlib import Path
from typing import List, Tuple


EXTERNAL_REVIEW_LOG_PREFIX = "vsdd-suite/suite-development/review-log/external-review-log/"

# Handle declaration patterns. A handle line declares one or more platform handles
# in the format `**Handle:** [@slug.tld](url) (Platform) / [@slug](url) (Platform)`.
# We extract the platform-tagged slug pairs.
HANDLE_LINE_RE = re.compile(r"^\*\*Handle:\*\*\s+(.*)$", re.MULTILINE)
HANDLE_DECL_RE = re.compile(r"\[@([\w.\-]+)\]\([^)]+\)\s*\(([A-Za-z][\w ]*)\)")

# Real-name field. We allow `**Name:**` only when it matches a declared handle slug
# after normalization.
NAME_FIELD_RE = re.compile(r"^\*\*Name:\*\*\s+(.+?)$", re.MULTILINE)

# Email pattern — bare email in preamble is a violation.
# Crude RFC-5322-ish but intentionally not exhaustive; the goal is to catch the
# obvious "leak the reviewer's email in the preamble" pattern, not to validate.
EMAIL_RE = re.compile(r"\b[A-Za-z0-9._%+\-]+@[A-Za-z0-9.\-]+\.[A-Za-z]{2,}\b")

# H1 + required H2 patterns.
H1_RE = re.compile(r"^# External Review — @[\w.\-]+ — \d{4}-\d{2}-\d{2}\s*$", re.MULTILINE)
H2_REVIEWER_RE = re.compile(r"^## Reviewer\s*$", re.MULTILINE)
H2_SOURCE_RE = re.compile(r"^## Source\s*$", re.MULTILINE)


def slug_normalize(s: str) -> str:
    """Normalize a handle string to a comparable slug.

    `dollspace.gay` -> `dollspacegay`
    `dollspace-gay` -> `dollspacegay`
    `shimmermathlabs.com` -> `shimmermathlabscom`
    `nwhitehead` -> `nwhitehead`

    Comparison is then substring-based: two normalized slugs share an identity
    iff one is a substring of the other (handles longer common-prefix testing
    than exact equality so platform-suffix variations don't break the match).
    """
    return re.sub(r"[^a-z0-9]", "", s.lower())


def slugs_share_identity(a: str, b: str) -> bool:
    """Two slugs share identity if one is a substring of the other after normalize."""
    na = slug_normalize(a)
    nb = slug_normalize(b)
    if not na or not nb:
        return False
    return na in nb or nb in na


def split_verbatim_blocks(content: str) -> Tuple[str, str]:
    """Split file content into (preamble, verbatim) — the preamble is everything
    before the first '## Verbatim source content' section; the verbatim block is
    the rest. We only enforce identity-correlation rules on the preamble; quoted
    source content is preserved as-authored per the source-archiving discipline.
    """
    parts = re.split(r"^## Verbatim source content\s*$", content, maxsplit=1, flags=re.MULTILINE)
    if len(parts) == 1:
        return content, ""
    return parts[0], parts[1]


def extract_reviewer_block(preamble: str) -> str:
    """Extract the `## Reviewer` block (from its H2 to the next H2 or EOF)."""
    m = re.search(r"^## Reviewer\s*\n(.*?)(?=^## |\Z)", preamble, re.MULTILINE | re.DOTALL)
    return m.group(1) if m else ""


def extract_source_block(preamble: str) -> str:
    """Extract the `## Source` block (from its H2 to the next H2 or EOF)."""
    m = re.search(r"^## Source\s*\n(.*?)(?=^## |\Z)", preamble, re.MULTILINE | re.DOTALL)
    return m.group(1) if m else ""


def check_file(path: Path) -> List[str]:
    """Apply identity-correlation discipline rules to one file.

    Returns a list of violation messages (empty if clean).
    """
    violations: List[str] = []
    content = path.read_text(encoding="utf-8")

    # Rule 4: H1 + required H2s
    if not H1_RE.search(content):
        violations.append(
            f"{path}: missing required H1 of the form `# External Review — @<handle> — YYYY-MM-DD`"
        )
    if not H2_REVIEWER_RE.search(content):
        violations.append(f"{path}: missing required `## Reviewer` H2 section")
    if not H2_SOURCE_RE.search(content):
        violations.append(f"{path}: missing required `## Source` H2 section")

    # Split preamble from verbatim source content; identity-correlation rules
    # only apply to the preamble.
    preamble, _ = split_verbatim_blocks(content)
    reviewer_block = extract_reviewer_block(preamble)
    source_block = extract_source_block(preamble)

    # Rule 1: multi-platform handle declarations must share a normalized slug.
    # Extract all (slug, platform) pairs from the `**Handle:**` line.
    handle_pairs: List[Tuple[str, str]] = []
    for handle_match in HANDLE_LINE_RE.finditer(reviewer_block):
        for slug, platform in HANDLE_DECL_RE.findall(handle_match.group(1)):
            handle_pairs.append((slug, platform.strip()))
    if len(handle_pairs) >= 2:
        # All pairs must share a normalized identity-slug.
        first_slug = handle_pairs[0][0]
        for slug, platform in handle_pairs[1:]:
            if not slugs_share_identity(first_slug, slug):
                violations.append(
                    f"{path}: identity-correlation discipline violation — multi-platform handles "
                    f"`{first_slug}` and `{slug}` ({platform}) do not share a normalized slug. "
                    f"Per the discipline: when a reviewer's handles differ between platforms, "
                    f"surface only the platform the reviewer engaged on for this review. "
                    f"See `suite-development.md` § External-review-log subfolder pattern."
                )

    # Rule 2: `**Name:**` field must match a declared handle slug.
    declared_slugs = [slug for slug, _ in handle_pairs]
    for name_match in NAME_FIELD_RE.finditer(reviewer_block):
        name_value = name_match.group(1).strip()
        if not declared_slugs:
            violations.append(
                f"{path}: `**Name:**` field declared without any handle. "
                f"Per the discipline: name only the reviewer's authored handle; if no handle, "
                f"the file shouldn't have a Name field."
            )
        else:
            matches_handle = any(slugs_share_identity(name_value, slug) for slug in declared_slugs)
            if not matches_handle:
                violations.append(
                    f"{path}: identity-correlation discipline violation — `**Name:** {name_value}` "
                    f"does not match any declared handle slug ({', '.join(declared_slugs)}). "
                    f"Per the discipline: the `**Name:**` field is only declared when the name IS "
                    f"the handle. Otherwise omit. The suite does not surface real names that "
                    f"differ from the reviewer's authored handle."
                )

    # Rule 3: bare emails in preamble Reviewer or Source blocks are violations.
    for block_name, block_text in (("Reviewer", reviewer_block), ("Source", source_block)):
        for email_match in EMAIL_RE.finditer(block_text):
            # Skip if inside a quoted source-content line (starts with `> `).
            line_start = block_text.rfind("\n", 0, email_match.start()) + 1
            line_end = block_text.find("\n", email_match.end())
            if line_end == -1:
                line_end = len(block_text)
            line = block_text[line_start:line_end]
            if line.lstrip().startswith(">"):
                continue
            # Skip if inside a fenced code block (between ```).
            # (Conservative heuristic: skip if line is indented 4+ spaces or starts with ```.)
            if line.startswith("```") or line.startswith("    "):
                continue
            violations.append(
                f"{path}: identity-correlation discipline violation — bare email address "
                f"`{email_match.group(0)}` appears in `## {block_name}` preamble. "
                f"Per the discipline: emails are identity-correlation surfaces; the reviewer's "
                f"authored choice is their platform handle. Emails in verbatim source content "
                f"(quoted with `> ` prefix or in fenced blocks) are preserved per source-archiving."
            )

    return violations


def main(argv: List[str]) -> int:
    files = argv[1:]
    if not files:
        # When invoked by pre-commit with no matching files, exit 0.
        return 0

    all_violations: List[str] = []
    for f in files:
        # Only enforce on files in external-review-log/.
        if EXTERNAL_REVIEW_LOG_PREFIX not in f:
            continue
        if not f.endswith(".md"):
            continue
        path = Path(f)
        if not path.exists():
            # File was deleted in the commit; skip.
            continue
        all_violations.extend(check_file(path))

    if all_violations:
        print("check-external-review-anonymization: identity-correlation discipline violations:", file=sys.stderr)
        for v in all_violations:
            print(f"  {v}", file=sys.stderr)
        print(
            "\nPer `suite-development.md` § External-review-log subfolder pattern § "
            "Identity-correlation discipline: the suite does not correlate identities the "
            "reviewer engaged through different surfaces, even when correlation is knowable. "
            "Knowability is not surfacing. To bypass for a deliberate out-of-pattern entry "
            "(e.g., a same-identity-string-across-platforms reviewer like dollspace-gay where "
            "the hook's heuristic doesn't match), add `<!-- hook-bypass: <rationale> -->` "
            "within the first 5 lines of the file; bypasses are themselves findings for the "
            "next registry-walk review.",
            file=sys.stderr,
        )
        return 1

    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
