# Platform Engineering Review

This review is part of the [Adversarial Iterative Refinement (AIR)](README.md) suite. It may be run independently or alongside other domains. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

Platform Engineering owns the full delivery platform: CI/CD pipelines, DevSecOps practices, infrastructure (cloud and on-premise), containerization, and observability. The purpose of this review is to evaluate whether the platform is correctly configured, hardened, automated, and observable — and to shift quality and security checks left so that defects are caught automatically before they reach production.

Not all dimensions apply to every project. A static single-user tool has no cloud infrastructure to evaluate. Apply the dimensions relevant to the project's deployment context and skip with rationale those that do not apply.

## Current Review Prompt

**Scope:** Whole platform by default — CI/CD, infrastructure, containerization, and observability. If a scope is provided (e.g., a specific workflow, Terraform module, or Dockerfile), focus primary analysis there — but regression checks always cover the entire platform.

Read DESIGN.md first for context on the project's intended scope, constraints, and technology choices. Then read all workflow files, infrastructure code, Dockerfiles, build config, package manifests, lock files, and `.gitignore`. Apply every standard dimension below as a floor — add others as appropriate to the current state of the platform. There is no restriction on what can be flagged.

For each finding, cite file and line number. Classify as **resolved** (fix applied this review), **deferred** (scheduled for a specific layer, reason given), or **dismissed** (no action taken, rationale required).

Regression check: verify that all pipeline gates and infrastructure controls installed in prior reviews are still present and still functioning. A refactor to one part of the platform can silently remove a gate or expose an infrastructure surface.

**Left-shift lens:** For every manual check in the project's AIR gate checklists, evaluate whether it can be automated and moved into CI. Automating a check is always preferable to a human remembering to run it.

**Coordination:** Flag any findings that should be surfaced to [QUALITY-ENGINEERING-REVIEW.md](QUALITY-ENGINEERING-REVIEW.md), [UX-REVIEW.md](UX-REVIEW.md), [SECURITY-REVIEW.md](SECURITY-REVIEW.md), or [SOLUTION-ARCHITECT-REVIEW.md](SOLUTION-ARCHITECT-REVIEW.md). If this review suggests the need for a new AIR domain, log it as a finding.

## Standard Evaluation Dimensions

### CI/CD Pipeline

1. **Pipeline completeness** — Does CI run all required checks: typecheck, unit tests, coverage, browser tests, build, audit? Are any quality gates manual-only that could be automated?
2. **Gate enforcement** — Are all pipeline checks required to pass before merging? Is branch protection configured?
3. **Dependency installation** — Is `npm ci` (or equivalent) used rather than a non-deterministic install command? Is the lock file committed and the source of truth for installs?
4. **Environment pinning** — Are runtime versions (Node, Python, etc.) pinned? Are browser and tool versions for testing deterministic?
5. **Cache correctness** — Are cache keys scoped to the right artifacts? Will caches invalidate when dependencies or configs change?
6. **Coverage thresholds** — Are coverage requirements enforced in CI with configured thresholds, not just available locally?
7. **Action/dependency pinning** — Are CI action versions pinned to avoid supply chain risk? Are they up to date?
8. **Artifact hygiene** — Is build output excluded from version control? Is it generated fresh in CI, never committed?
9. **Left-shift opportunities** — Which manual review steps could be automated and added to CI?

### DevSecOps

10. **Security scanning** — Is a dependency audit (`npm audit`, `dependabot`, `trivy`, or equivalent) run in CI and configured to fail on findings above the accepted risk threshold?
11. **Secret management** — Are secrets injected via environment variables or a secrets manager, never hardcoded or committed? Is the secrets surface minimal?
12. **Supply chain integrity** — Are third-party actions, base images, and dependencies pinned to verified versions? Is there a process for reviewing and updating them?
13. **Least privilege** — Do CI jobs, service accounts, and deployed services operate with the minimum permissions required? Are IAM roles and policies scoped correctly?
14. **Compliance gates** — Are security and compliance checks (SAST, DAST, license scanning, image scanning) integrated into the pipeline at the appropriate stages?

### Infrastructure

15. **Infrastructure as Code** — Is all infrastructure defined in code (Terraform, Pulumi, CDK, etc.) rather than configured manually? Is IaC version-controlled and reviewed like application code?
16. **Cloud/on-premise resource hygiene** — Are resources tagged, named consistently, and scoped to the correct environment? Are unused or orphaned resources identified?
17. **Containerization** — If containers are used: are base images pinned to a specific digest, not a mutable tag? Are images scanned for vulnerabilities? Is the image build reproducible?
18. **Container security** — Do containers run as non-root? Are capabilities dropped? Are read-only filesystems used where possible? Are resource limits set?
19. **Environment parity** — Do development, staging, and production environments match closely enough that bugs caught in one are representative of the others?
20. **Disaster recovery** — Is there a documented and tested plan for recovering from infrastructure failure? Are backups automated and verified?

### Observability

21. **Logging** — Are application and infrastructure logs structured, queryable, and routed to a central system? Are log levels appropriate (not too noisy, not too quiet)?
22. **Metrics** — Are key application and infrastructure metrics instrumented and visible? Are business-relevant signals (not just system health) captured?
23. **Alerting** — Are alerts configured for conditions that require human attention? Are they actionable and free of false positives? Is on-call coverage defined?
24. **Tracing** — For distributed systems: is distributed tracing in place? Can a request be followed across service boundaries?
25. **Dashboards** — Is there a canonical operational dashboard? Does it show the state of the system at a glance?

---

Review entries are logged in `adversarial-iterative-refinement/PLATFORM-ENGINEERING-REVIEW.md` inside the project being reviewed.
