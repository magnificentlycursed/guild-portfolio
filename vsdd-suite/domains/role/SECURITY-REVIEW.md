# Security Review

This review is part of the [Iterative Adversarial Refinement (IAR)](../../README.md) suite. It may be run independently or alongside other domains. See [README.md](../../README.md) for sequencing, scoped runs, and domain coordination.

**Reviewer role: Security Engineer** (Security Engineer / Application Security Engineer)

The purpose of this review is to apply iterative adversarial pressure to find, document, and resolve security vulnerabilities, unsafe patterns, validation gaps, and regressions. Every review targets the whole application — not only the most recently changed code.

## Current Review Prompt

**Scope:** Whole application by default. If a scope is provided (e.g., a specific feature or set of changed files), focus primary analysis there — but regression checks always cover the entire application.

Read DESIGN.md first for context on the project's intended scope, constraints, and feature set. Then read all source files, test files, HTML, CSS, and config. Apply every standard dimension below as a floor — add others as appropriate to the current state of the app. There is no restriction on what can be flagged.

For each finding, cite file and line number. Classify as **resolved** (fix applied this review), **accepted risk** (no fix, explicit rationale required), **dismissed** (no action taken, rationale required), or **hallucinated** (the adversary invented a vulnerability that does not exist — push back is warranted. Consistent hallucinated findings are the maximum viable refinement signal: real issues have been exhausted).

Regression check: verify that all previously-addressed security controls remain intact. Prior layers' security findings are always in scope. A change to validation, rendering, or storage handling can silently remove a control.

**Coordination:** Flag any findings that should be surfaced to [QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md), [UX-REVIEW.md](UX-REVIEW.md), [PLATFORM-ENGINEER-REVIEW.md](PLATFORM-ENGINEER-REVIEW.md), [SOLUTION-ARCHITECT-REVIEW.md](SOLUTION-ARCHITECT-REVIEW.md), or [PRIVACY-REVIEW.md](PRIVACY-REVIEW.md) (dim 8 — data classification cross-references Privacy dim 1; when Privacy is active, coordinate data classification findings there). For any sensitive data patterns identified in this review (secrets, PII, identity-leaking paths), flag them to [PLATFORM-ENGINEER-REVIEW.md](PLATFORM-ENGINEER-REVIEW.md) so they can be added to pre-commit hook detection rules.

**DESIGN.md change authority:** If a finding requires a change to `DESIGN.md`, classify it "Raised to SO" and document the proposed change and rationale. Do not apply the change. `DESIGN.md` is a controlled spec document — the Solution Owner is the sole domain authorized to modify it.

**Sycophancy check:** An agent reviewing its own security implementation will rationalize the risks it did not consider during generation as out of scope or not applicable. The most dangerous finding is not a missed CVE — it is a vulnerability class that was never considered at all. Treat every "not applicable" determination with extra scrutiny: verify it genuinely does not apply, not that the reviewer did not think to check. Flag any dimension where the answer is "this project doesn't have X" without verifying that the project cannot be made to have X by an attacker. If this review suggests the need for a new IAR domain, log it as a finding.

**Language and interface supplement:** Consult `../../supplements/` for the supplement matching the project's primary language (e.g., `rust.md`, `javascript-typescript.md`) and interface type (e.g., `cli.md`, `browser-app.md`). Apply the **Security** section from the relevant supplement files in addition to the standard dimensions below.


**Validator pair (Review 77):** `red-team` is the natural validator for findings owned by this domain — the Security ↔ Red Team adversarial pair is the suite's canonical worked example of the validator-pair pattern. Security defines posture from inside the threat model; Red Team challenges from outside the same fix. Resolved findings declare `**Validator:** red-team` per the lifecycle convention in `../../suite-development/suite-development.md` § Validation loop discipline. When a finding genuinely has no Red Team validation surface (e.g., a documentation-only secret-handling clarification with no exploit consequence), declare `**Validator:** sanity-check` per the meta-validator-of-last-resort pattern (Review 77 Finding 2) — Sanity Check applies DESIGN.md + architecture context to confirm the resolution is coherent with the spec.
## Threat Model

Before applying the standard dimensions, establish the threat model for this project. A security review without a threat model is a checklist. The threat model tells you which checklist entries matter most and which attack patterns are plausible for this specific application.

Answer these questions before reading the source files:

- **Who are the plausible attackers?** Name them: anonymous internet user, authenticated user, disgruntled insider, automated scanner. For a single-user local tool, the attacker surface is much smaller than for a multi-user web application — but name that explicitly rather than assuming it away.
- **What is the crown jewel?** Name the data or capability whose compromise would be the worst outcome. Is it user credentials? User data? The ability to execute arbitrary code on the host? The ability to exfiltrate data in bulk? A system with no identified crown jewel has no ranked threat surface.
- **What are the entry points?** List every surface where an attacker can interact with the system: HTTP endpoints, form inputs, URL parameters, file uploads, localStorage/sessionStorage reads, CLI arguments, environment variables read at runtime, third-party scripts. Every entry point is a potential attack surface — an unlisted entry point is an unreviewed attack surface.
- **Which threat actors are relevant to this project's deployment context?** A tool deployed only on a developer's local machine has a different threat model than one deployed to a public URL. If DESIGN.md specifies the deployment context, use it. If it does not, flag the omission.

Record the threat model as a **preamble in the review log**, before any numbered findings. It is not a classified finding (resolved/accepted risk/dismissed/hallucinated) — it is a prerequisite record. A reviewer who cannot state the threat model has not completed this review.

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

   Note: for applications with complex multi-user access control (role hierarchies, delegated permissions, cross-tenant isolation), this dimension is a floor. A dedicated auth review domain may be warranted — log it as a finding if the scope exceeds what these bullets cover.

7. **Audit logging** — Are security-relevant events logged in a way that supports incident response and forensic reconstruction?
   - Named audit events: authentication (login, logout, failure), privilege escalation, sensitive data access, data modification (create/update/delete), administrative actions, failed authorization attempts.
   - Are audit logs tamper-evident? A log that can be modified or deleted by an attacker provides false assurance, not a real control. Tamper evidence requires append-only storage or a separate, access-controlled log sink.
   - Are audit logs retained for an appropriate period and stored separately from application logs to prevent accidental purging?
   - Can audit logs reconstruct a timeline of events after an incident? If the answer is "we'd have to dig through application logs," the audit trail is not fit for purpose.
   - For single-user local tools: verify the deployment context genuinely has no accountability requirement. A tool deployed in an enterprise environment or on shared infrastructure carries an audit obligation even if it was designed as a personal tool.

8. **Data classification and control requirements** — Is data categorized by sensitivity, and are security controls applied proportionate to that classification?
   - Named classification tiers (as a floor): public (no restrictions), internal (access controls appropriate), confidential (encryption at rest and in transit, access logging), restricted (highest controls, limited access, audit required).
   - For each data type in the application: what classification does it carry? Are the applied controls — encryption, access restrictions, logging, retention limits — proportionate to the classification?
   - Named failure modes: storing confidential data in localStorage or a flat file without encryption; transmitting classified data over HTTP; applying identical controls to all data regardless of sensitivity; no defined classification scheme at all.
   - Cross-reference with [PRIVACY-REVIEW.md](PRIVACY-REVIEW.md) dim 1 (data inventory) when Privacy is active — Privacy identifies what data exists and whether it is personal; this dimension determines what controls the classification mandates.

9. **Error-message interpolation escape** — Every error message that interpolates user-supplied values must escape `is_control()` characters (`Cc`) and format characters (`Cf` — Trojan-Source bidi U+202E, zero-width characters) before writing the bytes to stdout or stderr. **Validators that reject input at parse time must not undo the rejection by echoing the rejected bytes raw to the error stream.** A `parse_*` validator that rejects `\x1b` in input but emits `Error: bad value: \x1b[31mred\x1b[0m` to stderr has reproduced the attack the validator was supposed to defend against — the operator's terminal renders the escape sequence regardless of whether the input was stored. Named failure modes: `format!("Error: invalid label: {label}")` where `label` contains an ANSI escape sequence the terminal will render; clap-generated error messages that quote the offending argument literally (the framework typically does not Cc-escape); JSON parser errors that include the malformed input verbatim in the error string; `eprintln!("{:?}", err)` where `err`'s `Display` formats user input without sanitization. Detector pattern: grep the codebase for every error site — `panic!`, `eprintln!`, `format!` into an `Err`, `?`-propagation paths whose `Error::Display` interpolates input — and confirm a `display_safe` (or equivalent) sanitizer wraps every user-derived value before it reaches the error stream. The escape must preserve structurally-significant whitespace (`\n` for multi-line error messages stays; `\r`/`\t`/`\x07` get escaped to printable forms like `\\r`, `\\t`, `\\x07`). Coordinate: G-125 (this dim), G-124 (per-property free-form text defense Red Gate checklist references this dim), G-152 (input/output strictness symmetry — error-message echoing is one specific asymmetry pattern). Canonical worked example: ITC's `sanitize_quoted_values` narrow-scope clap-error sanitizer (Layer 7 R2, RT R10 F1) — it preserves clap's structural `\n` while escaping inside `'...'` quoted regions, applying the rule that this dim names. Cross-reference Rust supplement § Security for `display_safe` patterns.

---

**Confidentiality-aware citation (Security-domain reminder).** Information-exposure findings, identity-disclosure findings, and secrets-management findings are this domain's typical surface — and the worked examples that illustrate them tend to instantiate the disclosure. When citing leaked credentials, hardcoded paths revealing developer identity, or environment values: abstract the concrete value to a placeholder (`<token>`, `<user>`, `<path>`) before committing the review log. The specific control here is the primer rule (`primers/3-review-session.md` § Confidentiality-aware citation) and the `vsdd-suite/hooks/check-review-log-anonymization.sh` hook. Apply both. Demonstrating a disclosure by quoting the actually-disclosed value reproduces the disclosure inside the very review meant to close it.

---

Review entries are logged in per-session files at `vsdd-suite/review-log/YYYY-MM-DD-security.md` inside the project being reviewed; the per-domain index at `vsdd-suite/SECURITY-REVIEW.md` aggregates rounds (newest-first) and is the entry point for browsing the domain's review history. See `vsdd-suite/suite-development/suite-development.md` § Governing standard for project-level review logs.
