# Security Review

This review is part of the [Adversarial Iterative Refinement (AIR)](README.md) suite. It is a required gate for merging. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

The purpose of this review is to apply iterative adversarial pressure to find, document, and resolve security vulnerabilities, unsafe patterns, validation gaps, and regressions. Every review targets the whole application — not only the most recently changed code.

## Current Review Prompt

**Scope:** Whole application by default. If a scope is provided (e.g., a specific feature or set of changed files), focus primary analysis there — but regression checks always cover the entire application.

Read all source files, test files, HTML, CSS, and config. Apply every standard dimension below as a floor — add others as appropriate to the current state of the app. There is no restriction on what can be flagged.

For each finding, cite file and line number. Classify as **resolved** (fix applied this review), **accepted risk** (no fix, explicit rationale required), or **dismissed** (no action taken, rationale required).

Regression check: verify that all previously-addressed security controls remain intact. Prior layers' security findings are always in scope. A change to validation, rendering, or storage handling can silently remove a control.

**Coordination:** Flag any findings that should be surfaced to [QA-REVIEW.md](QA-REVIEW.md), [UX-REVIEW.md](UX-REVIEW.md), [PLATFORM-ENGINEERING-REVIEW.md](PLATFORM-ENGINEERING-REVIEW.md), or [SOLUTION-ARCHITECT-REVIEW.md](SOLUTION-ARCHITECT-REVIEW.md). If this review suggests the need for a new AIR domain, log it as a finding.

## Standard Evaluation Dimensions

1. **Rendering safety** — Is user-supplied content set via `.textContent` (safe) or `.innerHTML` (potentially unsafe)? Any `innerHTML` usage with user-controlled data is a finding.
2. **URL injection** — Can a user save a URL that executes code when clicked? Verify `javascript:`, `data:`, `vbscript:`, and other non-http(s) protocols are rejected.
3. **Storage data validation** — Is data loaded from storage validated before use? TypeScript `as` casts are compile-time only; they provide no runtime guarantee against malformed or tampered data.
4. **Dependency security** — Are there known CVEs in direct or transitive dependencies? Check with `npm audit`.
5. **Content Security Policy** — Is a CSP present (meta tag or response header)? Evaluate risk surface without one.
6. **Information exposure** — Do error messages, comments, or headers reveal internal structure, stack traces, or sensitive paths?
7. **Input handling** — Are all user inputs treated as untrusted before being stored or rendered?

---

Review entries are logged in `adversarial-iterative-refinement/SECURITY-REVIEW.md` inside the project being reviewed.
