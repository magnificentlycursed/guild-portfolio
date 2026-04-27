# Platform Engineering Review

This review is part of the [Iterative Adversarial Refinement (IAR)](README.md) suite. It may be run independently or alongside other domains. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

Platform Engineering owns the full delivery platform: CI/CD pipelines, DevSecOps practices, infrastructure (cloud and on-premise), containerization, and observability. The purpose of this review is to evaluate whether the platform is correctly configured, hardened, automated, and observable — and to shift quality and security checks left so that defects are caught automatically before they reach production.

Not all dimensions apply to every project. A static single-user tool has no cloud infrastructure to evaluate. Apply the dimensions relevant to the project's deployment context and skip with rationale those that do not apply.

## Current Review Prompt

**Scope:** Whole platform by default — CI/CD, infrastructure, containerization, and observability. If a scope is provided (e.g., a specific workflow, Terraform module, or Dockerfile), focus primary analysis there — but regression checks always cover the entire platform.

Read DESIGN.md first for context on the project's intended scope, constraints, and technology choices. Then read all workflow files, infrastructure code, Dockerfiles, build config, package manifests, lock files, and `.gitignore`. Apply every standard dimension below as a floor — add others as appropriate to the current state of the platform. There is no restriction on what can be flagged.

For each finding, cite file and line number. Classify as **resolved** (fix applied this review), **deferred** (scheduled for a specific layer, reason given), **dismissed** (no action taken, rationale required), or **hallucinated** (the adversary invented a problem that does not exist — push back is warranted. Consistent hallucinated findings are the maximum viable refinement signal: real issues have been exhausted).

Regression check: verify that all pipeline gates and infrastructure controls installed in prior reviews are still present and still functioning. A refactor to one part of the platform can silently remove a gate or expose an infrastructure surface.

**Left-shift lens:** For every manual check in the project's IAR gate checklists, evaluate whether it can be automated and moved into CI. Automating a check is always preferable to a human remembering to run it.

**Coordination:** Flag any findings that should be surfaced to [QUALITY-ENGINEERING-REVIEW.md](QUALITY-ENGINEERING-REVIEW.md), [UX-REVIEW.md](UX-REVIEW.md), [SECURITY-REVIEW.md](SECURITY-REVIEW.md), or [SOLUTION-ARCHITECT-REVIEW.md](SOLUTION-ARCHITECT-REVIEW.md). If this review suggests the need for a new IAR domain, log it as a finding.

**Review posture:** Many PE dimensions are compliance checks — does X exist or not — rather than adversarial judgment calls requiring interpretation. The value is systematic coverage, not finding-count. Adversarial intensity applies most to judgment-dependent decisions: which sections are deemed inapplicable, what risk thresholds are accepted, and whether left-shift opportunities are being rationalized away. Scrutinize every "not applicable" determination — an agent that finds no applicable security scanning concerns in a published package, or no infrastructure concerns in a deployed application, is likely rationalizing rather than reviewing.

**Sycophancy check:** The primary sycophancy risk in this domain is around applicability decisions and threshold acceptance, not binary existence checks. Flag any case where an inapplicable determination was made without examining whether it genuinely does not apply, and any case where an accepted risk was accepted without specific evidence of the risk level.

**Language and interface supplement:** Consult `lang/` for the supplement matching the project's primary language (e.g., `rust.md`, `javascript-typescript.md`). Apply the **Platform Engineering** section from the relevant supplement file in addition to the standard dimensions below — language supplements specify the correct tooling for dependency installation, auditing, linting, and format checking.

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

---

Review entries are logged in `iterative-adversarial-refinement/PLATFORM-ENGINEERING-REVIEW.md` inside the project being reviewed.
