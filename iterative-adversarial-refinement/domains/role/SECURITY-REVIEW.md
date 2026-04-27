# Security Review

This review is part of the [Iterative Adversarial Refinement (IAR)](../../README.md) suite. It may be run independently or alongside other domains. See [README.md](../../README.md) for sequencing, scoped runs, and domain coordination.

The purpose of this review is to apply iterative adversarial pressure to find, document, and resolve security vulnerabilities, unsafe patterns, validation gaps, and regressions. Every review targets the whole application — not only the most recently changed code.

## Current Review Prompt

**Scope:** Whole application by default. If a scope is provided (e.g., a specific feature or set of changed files), focus primary analysis there — but regression checks always cover the entire application.

Read DESIGN.md first for context on the project's intended scope, constraints, and feature set. Then read all source files, test files, HTML, CSS, and config. Apply every standard dimension below as a floor — add others as appropriate to the current state of the app. There is no restriction on what can be flagged.

For each finding, cite file and line number. Classify as **resolved** (fix applied this review), **accepted risk** (no fix, explicit rationale required), **dismissed** (no action taken, rationale required), or **hallucinated** (the adversary invented a vulnerability that does not exist — push back is warranted. Consistent hallucinated findings are the maximum viable refinement signal: real issues have been exhausted).

Regression check: verify that all previously-addressed security controls remain intact. Prior layers' security findings are always in scope. A change to validation, rendering, or storage handling can silently remove a control.

**Coordination:** Flag any findings that should be surfaced to [QUALITY-ENGINEERING-REVIEW.md](QUALITY-ENGINEERING-REVIEW.md), [UX-REVIEW.md](UX-REVIEW.md), [PLATFORM-ENGINEERING-REVIEW.md](PLATFORM-ENGINEERING-REVIEW.md), or [SOLUTION-ARCHITECT-REVIEW.md](SOLUTION-ARCHITECT-REVIEW.md). For any sensitive data patterns identified in this review (secrets, PII, identity-leaking paths), flag them to [PLATFORM-ENGINEERING-REVIEW.md](PLATFORM-ENGINEERING-REVIEW.md) so they can be added to pre-commit hook detection rules.

**Sycophancy check:** An agent reviewing its own security implementation will rationalize the risks it did not consider during generation as out of scope or not applicable. The most dangerous finding is not a missed CVE — it is a vulnerability class that was never considered at all. Treat every "not applicable" determination with extra scrutiny: verify it genuinely does not apply, not that the reviewer did not think to check. Flag any dimension where the answer is "this project doesn't have X" without verifying that the project cannot be made to have X by an attacker. If this review suggests the need for a new IAR domain, log it as a finding.

**Language and interface supplement:** Consult `../../lang/` for the supplement matching the project's primary language (e.g., `rust.md`, `javascript-typescript.md`) and interface type (e.g., `cli.md`, `browser-app.md`). Apply the **Security** section from the relevant supplement files in addition to the standard dimensions below.

## Standard Evaluation Dimensions

1. **Input handling** — Are all user inputs treated as untrusted before being stored, processed, or rendered? Are inputs validated at the boundary where they enter the system?
2. **Persistence data validation** — Is data loaded from any persistent storage (files, databases, local storage, configuration) validated before use? Type assertions without runtime validation provide no actual safety guarantee against malformed or tampered data.
3. **Dependency security** — Are there known CVEs in direct or transitive dependencies? Use the appropriate audit tool for the project's language and ecosystem. (See language supplement for specific tooling.)
4. **Secret handling** — Are credentials, API keys, tokens, and private keys excluded from source code and version control? Are they injected via environment variables or a secrets manager? Verify also that secrets do not appear in: application logs, error messages, stack traces, monitoring and crash reporting payloads, or HTTP response bodies. A secret injected via environment variable that is included in a caught exception's message, a debug log, or a JSON error response is a production vulnerability regardless of how it was injected. Environment variable names themselves can be informative — flag any error output that reveals the names of secret variables.
5. **Information exposure** — Do error messages, logs, comments, or output reveal internal structure, stack traces, file system paths, or sensitive system information?
6. **Authentication and authorization** — If the application controls access to actions or data, evaluate:
   - Are authentication checks enforced at every entry point, or only at the UI layer? UI-only auth is bypassed by any direct API or storage call.
   - Are authorization checks applied on reads as well as writes? A user who cannot modify a record must also not be able to read it if it is confidential.
   - Can a user escalate privileges by modifying a request parameter, URL path, JWT claim, or stored role value?
   - Are session tokens bound to user identity (signed, non-guessable, with expiry)? Can a stolen token be used after logout?
   - Is the logout path complete — are sessions invalidated server-side, or only cleared client-side? A client-side-only logout means a stolen prior token is valid until expiry.
   - For projects without explicit authentication: could the project be deployed in a context where unauthenticated access is a security failure? Flag if the out-of-scope decision in DESIGN.md does not match the project's likely deployment environment.

   Note: G-07 (authentication/authorization review) is still open. For any project with multi-user access control, the single dimension above is insufficient — a dedicated auth review is required.

---

Review entries are logged in `iterative-adversarial-refinement/SECURITY-REVIEW.md` inside the project being reviewed.
