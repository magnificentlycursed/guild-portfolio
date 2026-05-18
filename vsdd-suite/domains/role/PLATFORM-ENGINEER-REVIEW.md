# Platform Engineer Review

This review is part of the [Iterative Adversarial Refinement (IAR)](../../README.md) suite. It may be run independently or alongside other domains. See [README.md](../../README.md) for sequencing, scoped runs, and domain coordination.

**Reviewer role: Platform Engineer** (Platform Engineer / DevOps Engineer / Infrastructure Engineer)

Platform Engineering owns the full delivery platform: CI/CD pipelines, DevSecOps practices, infrastructure (cloud and on-premise), containerization, and observability. The purpose of this review is to evaluate whether the platform is correctly configured, hardened, automated, and observable — and to shift quality and security checks left so that defects are caught automatically before they reach production.

Not all dimensions apply to every project. A static single-user tool has no cloud infrastructure to evaluate. Apply the dimensions relevant to the project's deployment context and skip with rationale those that do not apply.

## Current Review Prompt

**Scope:** Whole platform by default — CI/CD, infrastructure, containerization, and observability. If a scope is provided (e.g., a specific workflow, Terraform module, or Dockerfile), focus primary analysis there — but regression checks always cover the entire platform.

Read DESIGN.md first for context on the project's intended scope, constraints, and technology choices. Then read all workflow files, infrastructure code, Dockerfiles, build config, package manifests, lock files, and `.gitignore`. Apply every standard dimension below as a floor — add others as appropriate to the current state of the platform. There is no restriction on what can be flagged.

For each finding, cite file and line number. Classify as **resolved** (fix applied this review), **deferred** (scheduled for a specific layer, reason given), **dismissed** (no action taken, rationale required), or **hallucinated** (the adversary invented a problem that does not exist — push back is warranted. Consistent hallucinated findings are the maximum viable refinement signal: real issues have been exhausted).

Regression check: verify that all pipeline gates and infrastructure controls installed in prior reviews are still present and still functioning. A refactor to one part of the platform can silently remove a gate or expose an infrastructure surface.

**Left-shift lens:** For every manual check in the project's IAR gate checklists, evaluate whether it can be automated and moved into CI. Automating a check is always preferable to a human remembering to run it.

**Coordination:** Flag any findings that should be surfaced to [QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md), [UX-REVIEW.md](UX-REVIEW.md), [SECURITY-REVIEW.md](SECURITY-REVIEW.md), or [SOLUTION-ARCHITECT-REVIEW.md](SOLUTION-ARCHITECT-REVIEW.md). If this review suggests the need for a new IAR domain, log it as a finding.

**DESIGN.md change authority:** If a finding requires a change to `DESIGN.md`, classify it "Raised to SO" and document the proposed change and rationale. Do not apply the change. `DESIGN.md` is a controlled spec document — the Solution Owner is the sole domain authorized to modify it.

**Review posture:** Many PE dimensions are compliance checks — does X exist or not — rather than adversarial judgment calls requiring interpretation. The value is systematic coverage, not finding-count. Adversarial intensity applies most to judgment-dependent decisions: which sections are deemed inapplicable, what risk thresholds are accepted, and whether left-shift opportunities are being rationalized away. Scrutinize every "not applicable" determination — an agent that finds no applicable security scanning concerns in a published package, or no infrastructure concerns in a deployed application, is likely rationalizing rather than reviewing.

**Sycophancy check:** The primary sycophancy risk in this domain is around applicability decisions and threshold acceptance, not binary existence checks. Flag any case where an inapplicable determination was made without examining whether it genuinely does not apply, and any case where an accepted risk was accepted without specific evidence of the risk level.

**Language and interface supplement:** Consult `../../supplements/` for the supplement matching the project's primary language (e.g., `rust.md`, `javascript-typescript.md`). Apply the **Platform Engineering** section from the relevant supplement file in addition to the standard dimensions below — language supplements specify the correct tooling for dependency installation, auditing, linting, and format checking.

## Standard Evaluation Dimensions

### CI/CD Pipeline

1. **Pipeline completeness** — Does CI run all required checks: type checking, unit tests, coverage, integration tests, build, dependency audit? Are any quality gates manual-only that could be automated? (See language supplement for language-specific checks.)
2. **Gate enforcement** — Are all pipeline checks required to pass before merging? Is branch protection configured?
3. **Dependency installation** — Is a deterministic install command used (e.g., `npm ci`, `cargo build` with `Cargo.lock`, `pip install -r requirements.txt`) rather than a non-deterministic install? Is the lock file committed and the source of truth for installs? (See language supplement for language-specific guidance.)
4. **Environment pinning** — Are runtime versions (Node, Rust toolchain, Python, etc.) pinned to reproducible versions? Are tool versions for testing deterministic?
5. **Cache correctness** — Are cache keys scoped to the right artifacts? Will caches invalidate when dependencies or configs change?
6. **Coverage thresholds** — Are coverage requirements enforced in CI with configured thresholds, not just available locally?
7. **Action/dependency pinning** — Are CI action versions pinned to avoid supply chain risk? Are they up to date?
8. **Artifact hygiene** — Is build output excluded from version control? Is it generated fresh in CI, never committed?
9. **Left-shift opportunities** — Which manual review steps could be automated and added to CI?

### DevSecOps

10. **Pre-commit hooks** — Are pre-commit hooks installed and enforced to catch sensitive content before it enters version control? Hooks should cover: secret and credential detection (API keys, tokens, passwords, private keys, connection strings); PII detection (email addresses, phone numbers, government IDs, and other personal data); committer identity and local machine leakage (absolute paths containing usernames such as `/Users/yourname/`, hostnames, local environment details embedded in configs or build output); and large or binary files that should not be committed. Evaluate whether hooks can be bypassed with `--no-verify` and whether bypass is logged or blocked. Coordinate with [SECURITY-REVIEW.md](SECURITY-REVIEW.md) to ensure the detection patterns cover the project's full sensitive data surface.
11. **Security scanning** — Is a dependency audit run in CI and configured to fail on findings above the accepted risk threshold? (Use the ecosystem-appropriate tool: `npm audit`, `cargo audit`, `pip-audit`, `dependabot`, `trivy`, or equivalent.) See language supplement for specific tooling.
12. **Secret management** — Are secrets injected via environment variables or a secrets manager, never hardcoded or committed? Is the secrets surface minimal?
13. **Supply chain integrity** — Are third-party actions, base images, and dependencies pinned to verified versions? Is there a process for reviewing and updating them?
14. **Least privilege** — Do CI jobs, service accounts, and deployed services operate with the minimum permissions required? Are IAM roles and policies scoped correctly?
15. **Compliance gates** — Are security and compliance checks (SAST, DAST, license scanning, image scanning) integrated into the pipeline at the appropriate stages?

### Infrastructure

16. **Infrastructure as Code** — Is all infrastructure defined in code (Terraform, Pulumi, CDK, etc.) rather than configured manually? Is IaC version-controlled and reviewed like application code?
17. **Cloud/on-premise resource hygiene** — Are resources tagged, named consistently, and scoped to the correct environment? Are unused or orphaned resources identified?
18. **Containerization** — If containers are used: are base images pinned to a specific digest, not a mutable tag? Are images scanned for vulnerabilities? Is the image build reproducible?
19. **Container security** — Do containers run as non-root? Are capabilities dropped? Are read-only filesystems used where possible? Are resource limits set?
20. **Environment parity** — Do development, staging, and production environments match closely enough that bugs caught in one are representative of the others?
21. **Disaster recovery** — Is there a documented plan for recovering from infrastructure failure? Are backups automated? Beyond documentation: has the rollback procedure been executed in a non-production environment, with a record of the last test date? Has backup restoration been verified — not just that backups run, but that a restored backup produces a functional system? A rollback plan that has never been executed is untested speculation. A backup that has never been restored may be unrestorable. "Documented and available" is not "tested and reliable." Flag any DR plan that cannot answer: when was this last tested, and what happened?

### Observability

22. **Logging** — Are application and infrastructure logs structured, queryable, and routed to a central system? Are log levels appropriate (not too noisy, not too quiet)?
23. **Metrics** — Are key application and infrastructure metrics instrumented and visible? Are business-relevant signals (not just system health) captured?
24. **Alerting** — Are alerts configured for conditions that require human attention? Are they actionable and free of false positives? Is on-call coverage defined?
25. **Tracing** — For distributed systems: is distributed tracing in place? Can a request be followed across service boundaries?
26. **Dashboards** — Is there a canonical operational dashboard? Does it show the state of the system at a glance?
27. **Error surfacing** — Are errors caught at appropriate levels and surfaced with enough context to diagnose the root cause? Named failure modes: empty catch blocks that swallow errors silently; catch blocks that log a label without the error object; errors caught and rethrown without adding context. Every catch block that does not re-throw should emit a diagnostic event identifying what failed and why.
28. **Error classification** — Are errors distinguished by type? Named categories: user errors (invalid input — expected, not alarming), application errors (bugs — unexpected, should alert), and dependency errors (external service failure — expected under failure conditions, different response required). An application that treats all errors identically alarms on user errors and misses application errors.
29. **Diagnostic completeness** — Pick a plausible production failure (a save fails, a search returns wrong results). Could you diagnose the root cause from the application's log output alone, without source code or a debugger? If not, identify what is missing.
30. **Health surfaces** — For deployed services: does the health check verify the application's ability to serve requests, not just that the process is alive? A health check that returns 200 OK while the database connection is broken masks failures from load balancers and orchestration platforms.
31. **Sensitive data exclusion** — Do log entries, error messages, and diagnostic output avoid PII, credentials, and authentication tokens? (Coordinate with Security dim 4.) A well-structured log is a high-value target if it contains sensitive data — the same observability that helps diagnose failures creates exposure if it captures user data.
32. **Silent success confirmation** — For operations that modify state (saves, deletes, updates): is there a positive signal that the operation completed — not just that it did not fail? You cannot alert on the absence of a success event unless the success event exists.
33. **Runbook coverage** — Can the application's observable signals support the operational runbook? If the runbook says "check the logs for error code X," does the application actually emit that signal? Runbooks and observability must be designed together.

### Performance

34. **Time-to-interactive** — For browser apps: is there render-blocking JavaScript or CSS? Is the initial payload appropriate for the application's complexity? Measure with Lighthouse or equivalent under simulated network conditions — not just on a local dev server.
35. **Asset optimization** — Are JavaScript bundles, images, and other assets optimized for delivery? Named checks: minification and tree-shaking in the build output; images compressed and served at appropriate resolution; large dependencies that could be replaced by smaller alternatives or deferred.
36. **Performance budget** — Is there an explicit performance budget enforced in CI? Named metrics: maximum bundle size, maximum time-to-interactive under a defined network condition. A project with no performance budget in CI has no enforced performance requirement — measuring without gating is the same failure pattern as coverage without thresholds.
37. **Performance regression risk** — Are there recent changes that could silently degrade performance? Named patterns: adding a dependency without auditing its size impact; adding a synchronous operation in a hot code path; widening a data access pattern to fetch more than previously needed.

### Extended: Fresh-system install verification (capstone / production intent only)

This dimension activates conditionally on the project's intent declaration in `DESIGN.md` § Project intent (see `../../templates/DESIGN-template.md` and `../DOMAIN-INDEX.md` § Intent calibration). It applies to **capstone** and **production** intent projects; it is NOT required for **portfolio** or **learning-exercise** intent. The activation is binary: a `capstone`/`production` project that does not satisfy this dimension at gate close cannot pass merge; a `portfolio`/`learning-exercise` project may skip the dimension without finding.

38. **Fresh-system install verification (capstone / production)** — Has the project been installed and run by someone other than the developer, on a system that is not the developer's primary working environment, from a fresh checkout? Named checks: (a) a documented third-party install attempt — minimally, a `git clone` followed by the project's documented install command (`cargo install --path .`, `npm install -g .`, `pip install .`, `docker build .`, etc.) on a system that has no cached dependencies, no developer toolchain customizations, and no in-flight repo state, performed by a person who is not the project's primary developer; (b) the install attempt is recorded with date, installer identity (name or anonymized initials per the project's anonymization policy), system context (OS + version + relevant toolchain version), and outcome (succeeded / failed-at-step-N / required-undocumented-prerequisite-X); (c) the record lives in `PROCESS.md`, a dedicated `INSTALL-VERIFICATION.md`, or an equivalent project artifact; (d) any required-undocumented-prerequisite finding feeds back to the README completeness check (SE Dim 13 / TW domain) as an improvement requirement before the install-verification record is considered closing evidence. Named failure modes: developer's own machine is the only documented install (works on developer's `cargo`-cached environment but not on a fresh CI runner); README says `cargo install --path .` but actually requires `rustup target add x86_64-unknown-linux-musl` on certain systems; published binaries exist but no record of a third party installing them; the install record is the developer's own re-clone (no third-party signal). For capstone/production: the install-verification record is gating; for portfolio/learning-exercise: this dimension is skipped without finding. Coordinate: G-155 cross-references G-150 (intent declaration is the prerequisite this dim depends on); G-119 (AI-tool dependency inventory has a similar verify-against-fresh-environment shape); SE Dim 13 (README completeness — fresh-install attempts often surface README gaps). Reference: dollspace.gay's evaluation of ITC noted "no published binaries and PROCESS.md doesn't record anyone other than the author successfully running it ... reserved for the capstone in 04-proving-it/01-on-your-own.md" — ITC is portfolio-intent so the dimension correctly does not gate ITC; a capstone-intent successor would gate on this.

---

**Confidentiality-aware citation (Platform-domain reminder).** Pre-commit hooks, CI configuration, and secrets-management defects are this domain's typical findings — and the worked examples that illustrate them tend to instantiate the leak. When citing a hook config, an environment value, an anonymization gap, or a secrets-management defect: abstract the concrete value to a placeholder (`<user>`, `<email>`, `<key>`) before committing the review log. The specific control here is the existing primer rule (`primers/3-review-session.md` § Confidentiality-aware citation) and the `vsdd-suite/hooks/check-review-log-anonymization.sh` hook. Apply both. Demonstrating an anonymization gap by quoting the actually-leaked value reproduces the gap inside the very review meant to close it.

---

Review entries are logged in per-session files at `vsdd-suite/review-log/YYYY-MM-DD-platform-engineer.md` inside the project being reviewed; the per-domain index at `vsdd-suite/PLATFORM-ENGINEER-REVIEW.md` aggregates rounds (newest-first) and is the entry point for browsing the domain's review history. See `vsdd-suite/suite-development/suite-development.md` § Governing standard for project-level review logs.
