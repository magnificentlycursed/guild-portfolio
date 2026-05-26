#!/usr/bin/env python3
"""
Pre-commit hook: suite-internal-terminology containment in user-facing project artifacts.

Codified in `vsdd-suite/suite-development/suite-development.md` § External-review-log
subfolder pattern (companion hook). Authored at Review 88 (PR #42) per the recurrence-
prevention candidate motivated by @shimmermathlabs.com's external-feedback Post 10:
"this is fun, i'm getting 'Sycophancy-compensation reminder's" — suite-internal AI-
agent-discipline language was leaking to user-facing manual-test prose where a non-
author tester encountered it as unexplained jargon.

The discipline: **suite-internal audit-trail terminology stays in the audit-trail
artifacts; user-facing project artifacts use plain language.** A reviewer, apprentice,
operator, or external developer reading the project's user-facing surfaces (manual
tests, README, TODO, DESIGN, PROCESS — modulo the explicit AI-authored disclosure
sections) should not encounter suite-internal AI-agent-discipline language as part of
the instructions they are following.

Scope:
- User-facing project artifacts: `*/manual-tests/*.md`, `*/README.md`, `*/TODO.md`,
  `*/DESIGN.md`, `*/INSTALL-VERIFICATION.md`
- NOT scoped: `*/PROCESS.md` (the AI-authored retrospective contains methodology
  language by design; the AI-co-authorship disclosure section explicitly names this);
  `*/CHANGELOG.md` (audit-trail artifact); anything under `vsdd-suite/` (the suite's
  own authoring surface); anything under `vsdd-suite/suite-development/` (the audit
  trail itself); anything under `vsdd-suite/domains/` (domain prompts ARE methodology
  authoring); anything under `vsdd-suite/primers/` (primers ARE methodology authoring);
  per-domain review-log files (audit-trail artifacts).

Patterns flagged as suite-internal terminology (FAIL when in scope):

1. **"Sycophancy-compensation reminder"** — the canonical example surfaced by Nathan's
   feedback. The phrase belongs in the audit-trail discipline (per primer 3 +
   suite-development.md), NOT in user-facing manual-test prose.
2. **Bare `TW Dim N` / `QE Dim N` / `SE Dim N` / `<Domain> Dim N`** — domain
   dimensional references are methodology-internal. User-facing artifacts that need to
   cross-reference a discipline should name the discipline by its substantive content,
   not by its Dim number.
3. **Bare `G-NNN` without anchor link** — the G-NNN registry IDs are internal
   identifiers; user-facing prose that references them must use the markdown link form
   `[G-NNN](path/to/FINDINGS-INDEX.md#g-nnn)` so the reader can follow the reference.
4. **"adversarial-cold-session" / "cold-session-discipline" / "cluster-batching"** —
   suite-internal AI-agent-cycle vocabulary that has no meaning to a non-author user
   following a manual-test plan.

Exclusions per file:
- `<!-- hook-bypass: <rationale> -->` HTML comment in the first 5 lines bypasses for
  that file only; bypasses are themselves findings for the next registry-walk review.
- Quoted blocks (lines starting with `> ` or fenced code blocks) preserve verbatim
  content — historical references, source artifacts, command transcripts — and are
  not scanned.

Cross-domain ownership (per Review 87 Finding 6 per-error-class owner table):
this is a process-enforcement + early-detection script — AI Engineer owns; the
substantive discipline (suite-internal terminology vs user-facing language) is
informed by Technical Writer Dim 12 (lookup-cost) + UX Dim 6 (message clarity) +
Documentation Reviewer Dim 2 (implicit-knowledge audit).

Exit codes:
- 0 — pass
- 1 — discipline violation detected; specific lines + reason emitted to stderr
"""

import re
import sys
from pathlib import Path
from typing import List


# Patterns flagged as suite-internal terminology. Each is a compiled regex; the
# pattern's `pattern` attribute is used in the violation message so the rule is
# self-documenting.
SUITE_INTERNAL_PATTERNS = [
    (
        re.compile(r"\bSycophancy[- ]compensation reminder\b", re.IGNORECASE),
        'Suite-internal audit-trail discipline language "Sycophancy-compensation reminder" '
        "belongs in the audit-trail artifacts (primer 3 + suite-development.md), not in "
        "user-facing project prose. Rewrite the user-facing context to name the substantive "
        "concern (e.g., 'verify each manual-test step's expected output was actually observed') "
        "without invoking the suite-internal terminology.",
    ),
    (
        re.compile(r"\b(TW|QE|SE|UX|SA|SO|PE|DE) Dim \d+\b"),
        "Bare domain-dimensional reference (e.g., 'TW Dim 12') is suite-internal methodology "
        "language. User-facing artifacts cross-reference disciplines by substantive content, "
        "not by Dim number. Rewrite to name what the dimension covers (e.g., 'narrative "
        "lookup-cost discipline') in plain language.",
    ),
    (
        re.compile(r"(?<!\[)(?<!\w)G-\d{2,3}(?!\])\b(?![^\[]*?\]\()"),
        "Bare G-NNN registry-ID reference without a markdown link. User-facing artifacts that "
        "need to cross-reference a registry entry must use the markdown link form "
        "`[G-NNN](path/to/FINDINGS-INDEX.md#g-nnn)` so the reader can follow the reference.",
    ),
    (
        re.compile(r"\badversarial[- ]cold[- ]session\b", re.IGNORECASE),
        'Suite-internal AI-agent-cycle vocabulary "adversarial-cold-session" has no meaning '
        "to a non-author user. Rewrite to name the substantive concern in plain language.",
    ),
    (
        re.compile(r"\bcluster[- ]batching\b", re.IGNORECASE),
        'Suite-internal AI-agent-cycle vocabulary "cluster-batching" has no meaning '
        "to a non-author user. Rewrite to name the substantive concern in plain language.",
    ),
    (
        re.compile(r"\bcold[- ]session[- ]discipline\b", re.IGNORECASE),
        'Suite-internal AI-agent-cycle vocabulary "cold-session-discipline" has no meaning '
        "to a non-author user. Rewrite to name the substantive concern in plain language.",
    ),
]


# Path filters. The hook only fires on files matching IN_SCOPE; OUT_OF_SCOPE wins
# when both match (subpath of OUT_OF_SCOPE inside IN_SCOPE → skip).
IN_SCOPE_PATTERNS = [
    re.compile(r".*/manual-tests/[^/]+\.md$"),
    re.compile(r"^[^/]+/README\.md$"),
    re.compile(r".*/README\.md$"),
    re.compile(r".*/TODO\.md$"),
    re.compile(r".*/DESIGN\.md$"),
    re.compile(r".*/INSTALL-VERIFICATION\.md$"),
]
OUT_OF_SCOPE_PATTERNS = [
    re.compile(r"^vsdd-suite/"),  # the suite's own authoring surface
    re.compile(r".*/vsdd-suite/"),  # any project's vsdd-suite/ folder (audit trail)
    re.compile(r".*/PROCESS\.md$"),  # AI-authored retrospective per G-156
    re.compile(r".*/CHANGELOG\.md$"),  # audit-trail
    re.compile(r".*/COMPATIBILITY\.md$"),  # audit-trail
]


def is_in_scope(path: str) -> bool:
    """Check if path is in the scope this hook enforces."""
    # Out-of-scope wins when both match.
    for p in OUT_OF_SCOPE_PATTERNS:
        if p.search(path):
            return False
    for p in IN_SCOPE_PATTERNS:
        if p.search(path):
            return True
    return False


# Bypass marker (scoped form is canonical per AIE R2 F6 SO-decision):
#   `<!-- hook-bypass[hook-id1,hook-id2]: <rationale> -->` in the first 5 lines of the file.
# Each hook only bypasses if its own pre-commit id is in the scope list. The legacy
# unscoped form is REJECTED by the separate check-no-legacy-bypass-markers hook.
HOOK_ID = "check-suite-internal-terminology"
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


def strip_verbatim_blocks(content: str) -> str:
    """Remove quoted lines (`> ` prefix) and fenced code blocks. Returns the
    content with those regions blanked out so the line numbers are preserved
    but the verbatim content isn't scanned by the rule patterns.

    NOTE: this hook intentionally does NOT scan inside fenced code blocks or
    blockquotes — those preserve verbatim source content (commands, error
    messages, prior review citations) per the source-archiving discipline.
    """
    lines = content.split("\n")
    in_fence = False
    out = []
    for line in lines:
        stripped = line.strip()
        if stripped.startswith("```"):
            in_fence = not in_fence
            out.append("")  # blank out the fence line itself
            continue
        if in_fence:
            out.append("")  # blank out the fenced content
            continue
        if line.lstrip().startswith(">"):
            out.append("")  # blank out the blockquote line
            continue
        out.append(line)
    return "\n".join(out)


def check_file(path: Path) -> List[str]:
    """Apply suite-internal-terminology rules to one file.

    Returns a list of violation messages (empty if clean).
    """
    violations: List[str] = []
    content = path.read_text(encoding="utf-8")

    if has_bypass(content):
        return []  # bypass marker — skip; the bypass is a finding for the next review

    scannable = strip_verbatim_blocks(content)
    lines = scannable.split("\n")

    for line_idx, line in enumerate(lines, start=1):
        for pattern, rationale in SUITE_INTERNAL_PATTERNS:
            for m in pattern.finditer(line):
                violations.append(
                    f"{path}:{line_idx}: suite-internal terminology `{m.group(0)}` "
                    f"in user-facing project artifact. {rationale}"
                )
                break  # one violation per line per pattern; don't double-report

    return violations


def main(argv: List[str]) -> int:
    files = argv[1:]
    if not files:
        return 0

    all_violations: List[str] = []
    for f in files:
        if not is_in_scope(f):
            continue
        path = Path(f)
        if not path.exists():
            continue
        all_violations.extend(check_file(path))

    if all_violations:
        print(
            "check-suite-internal-terminology: user-facing-prose discipline violations:",
            file=sys.stderr,
        )
        for v in all_violations:
            print(f"  {v}", file=sys.stderr)
        print(
            "\nPer `suite-development.md` § External-review-log subfolder pattern "
            "(companion hook) + Review 88 Finding 3 (suite-internal-terminology "
            "containment discipline, from @shimmermathlabs.com's external-feedback "
            "Post 10 observing 'Sycophancy-compensation reminder' leak to user-facing "
            "manual-test prose): suite-internal AI-agent-discipline language stays in "
            "the audit-trail artifacts; user-facing project artifacts use plain "
            "language. To bypass for a deliberate out-of-pattern entry, add "
            "`<!-- hook-bypass: <rationale> -->` within the first 5 lines of the file; "
            "bypasses are themselves findings for the next registry-walk review.",
            file=sys.stderr,
        )
        return 1

    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
