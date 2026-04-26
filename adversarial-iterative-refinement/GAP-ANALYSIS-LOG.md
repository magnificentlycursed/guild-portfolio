# AIR Suite Gap Analysis Log

This log tracks gap analysis runs against the AIR suite itself. It is distinct from domain review logs, which evaluate specific projects. This file evaluates the suite's own coverage, completeness, and fitness for different project contexts.

Gap analysis should be re-run when:
- The suite gains or loses domains
- A new project type is being evaluated (mission-critical, speculative, regulated industry, etc.)
- A post-mortem reveals a class of defect the suite did not catch
- Significant time has passed and the technology or regulatory landscape has shifted

## How to run a gap analysis

1. Read all domain prompt files in this folder to understand the current suite state.
2. Read this log in full. Note which gaps are open, deferred, or dismissed — do not re-litigate dismissed gaps without new evidence.
3. Define the evaluation context: project type, deployment environment, regulatory exposure, team size, and timeline pressure. The same suite gap can be critical in one context and irrelevant in another.
4. For each open gap: has it been addressed by a recent suite change? Has the context changed? Is it still a gap?
5. Look for new gaps not in the registry. Consider: what class of defect would this suite fail to catch? What failure mode is not represented?
6. Record findings in a new run entry below.
7. Update the gap registry: change statuses, add new entries, close addressed ones.
8. If a gap is addressed by changing a domain file, commit those changes separately and reference the commit here.

## Gap Registry

Living table of all identified gaps. Update statuses here as gaps are addressed or dismissed. Do not delete rows — mark them Addressed or Dismissed with rationale.

| ID | Gap | Type | Mission-Critical Severity | Speculative Severity | Status | Opened | Last Reviewed |
|---|---|---|---|---|---|---|---|
| G-01 | Compliance and Legal domain missing | Missing domain | Critical | Low–Medium | Open | 2026-04-25 | 2026-04-25 |
| G-02 | Performance and Scalability domain missing | Missing domain | Critical | Defer | Open | 2026-04-25 | 2026-04-25 |
| G-03 | Privacy domain missing (listed as candidate) | Missing domain | Critical | Medium | Open | 2026-04-25 | 2026-04-25 |
| G-04 | Operational Readiness domain missing | Missing domain | Critical | Low | Open | 2026-04-25 | 2026-04-25 |
| G-05 | Delivery Governance missing (timeline, budget, milestones) | Missing domain | Critical | Medium | Open | 2026-04-25 | 2026-04-25 |
| G-06 | Security: no threat modeling | Dimension gap | High | Low | Open | 2026-04-25 | 2026-04-25 |
| G-07 | Security: no authentication/authorization review | Dimension gap | High | Low–Medium | Open | 2026-04-25 | 2026-04-25 |
| G-08 | Security: no session management review | Dimension gap | High | Low | Open | 2026-04-25 | 2026-04-25 |
| G-09 | Security: no audit logging requirement | Dimension gap | High | Low | Open | 2026-04-25 | 2026-04-25 |
| G-10 | Security: no data classification requirement | Dimension gap | Medium | Low | Open | 2026-04-25 | 2026-04-25 |
| G-11 | Solution Owner: no budget tracking dimension | Dimension gap | High | Medium | Open | 2026-04-25 | 2026-04-25 |
| G-12 | Quality Engineering: no integration/contract testing mandate | Dimension gap | High | Low | Open | 2026-04-25 | 2026-04-25 |
| G-13 | Platform Engineering: DR dimension lacks RTO/RPO targets | Dimension gap | Medium | Low | Open | 2026-04-25 | 2026-04-25 |
| G-14 | No domain for learning goals / validation structure (speculative projects) | Missing domain | N/A | Critical | Open | 2026-04-25 | 2026-04-25 |
| G-15 | No kill criteria mechanism (speculative projects) | Missing domain | N/A | High | Open | 2026-04-25 | 2026-04-25 |
| G-16 | No intentional technical debt tracking (speculative projects) | Dimension gap | Low | High | Open | 2026-04-25 | 2026-04-25 |
| G-17 | Solution Architect: no pivot readiness dimension (speculative projects) | Dimension gap | Low | High | Open | 2026-04-25 | 2026-04-25 |

**Status values:** Open · Addressed · Deferred · Dismissed · Context-Dependent

---

## Run 1 — 2026-04-25 20:00Z

**Context:** Initial gap analysis. Evaluated against two project types: (1) mission-critical software project with reputational, legal, and business-continuity stakes; (2) speculative/exploratory project that may become a product or business. Prompted by question: how thorough is this suite for a board-level presentation?

**Suite state at time of run:** Eight domains — Quality Engineering, UX, Security, Platform Engineering, Solution Architect, Solution Owner, Software Engineering, Data Engineering. Platform Engineering expanded to cover DevSecOps, infrastructure, and observability. Pre-commit hooks added as PE dimension 10.

### Findings

#### Missing domains

**G-01 — Compliance and Legal (mission-critical: Critical)**
No domain evaluates regulatory exposure. For any project handling personal data, financial transactions, healthcare information, or operating in a regulated industry, compliance is a distinct failure mode from security. GDPR applies to personal data collection from EU residents even in a prototype. PCI-DSS, HIPAA, SOX, ADA/WCAG legal mandates, and OSS license compliance all represent categories of legal and financial liability that none of the existing domains own. Security asks whether data can be exfiltrated; Compliance asks whether you had the right to collect it, whether you stored it correctly, and whether you can prove it to a regulator.

*For speculative projects:* reduced but not zero. Establish the minimum floor (what data are you collecting, under what legal basis) and defer the full apparatus.

**G-02 — Performance and Scalability (mission-critical: Critical)**
No domain evaluates whether the system performs under real load. Load testing, stress testing, performance budgets, latency SLAs, scalability projections, and capacity planning are unowned. A system that is functionally correct but unusable at production scale has failed.

*For speculative projects:* defer entirely. You do not yet know your load profile or whether the thing is worth scaling.

**G-03 — Privacy (mission-critical: Critical)**
Listed as a candidate domain in the suite README but not implemented. Privacy is distinct from Security. Security asks whether data can be exfiltrated; Privacy asks whether it should be collected in the first place, how long it is retained, who can access it, whether consent was properly obtained, and whether subjects can exercise rights (access, erasure, portability). These are separate failure modes with separate legal exposure.

*For speculative projects:* medium priority. Know what you are collecting and why before you collect it.

**G-04 — Operational Readiness (mission-critical: Critical)**
No domain asks whether the team can operate the system in production. Runbooks, incident response procedures, on-call coverage, escalation paths, rollback plans, and deployment checklists are unowned. A system can pass every technical review and fail in production because no one documented how to restart the service.

*For speculative projects:* low priority. "How do we restart this" is sufficient.

**G-05 — Delivery Governance (mission-critical: Critical)**
The suite is a quality process, not a delivery process. No domain tracks whether the project is on time and on budget, flags milestone slippage, or forces tradeoff decisions when timeline pressure appears. The Solution Owner prevents scope creep at the feature level but has no mechanism for tracking engineering cost or surfacing delivery risk early.

*For speculative projects:* medium priority, different character. Replace milestone tracking with kill criteria and learning goals (see G-14, G-15).

#### Gaps within existing domains

**G-06 — Security: no threat modeling**
The current Security domain covers seven dimensions appropriate for a single-user web application. A mission-critical system requires formal threat modeling: enumerating assets, attack surfaces, threat actors, and mitigations systematically before implementation begins. Threat modeling finds architectural security flaws that a code review cannot catch.

**G-07 — Security: no authentication/authorization review**
The current domain does not evaluate whether users can access only what they are permitted to access. An authorization bypass that lets user A read user B's data would not be caught. For any multi-user system, auth/authz is the highest-impact security surface.

**G-08 — Security: no session management review**
Session lifecycle (creation, expiry, invalidation, fixation resistance) is unowned. Relevant for any system with authentication.

**G-09 — Security: no audit logging requirement**
No domain asks whether the system records who did what and when. For mission-critical systems in regulated industries, an audit trail is both a legal requirement and an incident response necessity.

**G-10 — Security: no data classification requirement**
No domain asks what data the system handles and whether it is handled appropriately for its sensitivity level. Classification (public, internal, confidential, regulated) is the prerequisite for proportionate controls.

**G-11 — Solution Owner: no budget dimension**
The SO enforces scope and prevents feature additions, but a feature that is in scope and takes ten times longer than estimated is a budget failure the SO would not flag. Effort estimation, burn rate, and budget variance are unowned.

**G-12 — Quality Engineering: no integration/contract testing mandate**
Unit tests and browser/end-to-end tests are covered. For systems with multiple components or third-party integrations, contract testing (Pact or equivalent) and integration testing are distinct concerns. A change to an upstream API contract that breaks the integration would not necessarily be caught.

**G-13 — Platform Engineering: DR dimension lacks RTO/RPO targets**
The existing DR dimension (PE-21) asks whether a plan exists and whether backups are verified. For mission-critical systems, this is insufficient. Recovery Time Objective and Recovery Point Objective should be defined, tested, and enforced — not just described.

#### Speculative project-specific gaps

**G-14 — No learning goals / validation structure domain**
A speculative project that is technically excellent but answers the wrong question has failed. No domain owns: what are we trying to learn, at what fidelity, and is the project structured to produce a valid answer? This is the most critical gap for speculative work — without it, the suite can tell you the software is well-built but not whether the exploration succeeded.

**G-15 — No kill criteria mechanism**
No domain defines stopping conditions. A speculative project without defined kill criteria can drift indefinitely, consuming runway without producing a decision. This is a governance failure unique to exploratory work.

**G-16 — No intentional technical debt tracking**
In speculative projects, taking on technical debt is sometimes the right call. The risk is debt accumulated unconsciously that compounds and makes pivoting harder. No domain distinguishes debt we chose from debt we accumulated, or tracks the former as a known liability.

**G-17 — Solution Architect: no pivot readiness dimension**
For speculative projects, the SA review should evaluate whether the architecture allows a pivot when the project learns something that changes direction. Current SA dimensions ask whether boundaries are clean; they do not ask whether the system is designed to change.

### Decisions

**Decision — Suite is strong for most portfolio projects as-is.**
For individual portfolio projects without regulatory exposure or production SLAs, the current eight domains provide coverage well above industry average. Gaps G-01 through G-13 are appropriate to defer until a project reaches production or handles real user data.

**Decision — G-14 and G-15 are high priority for the next speculative project.**
Before using this suite on an exploratory project, add lightweight mechanisms for learning goals and kill criteria. These do not require full domains — they may be handled as a framing document or a pre-project checklist. Revisit at next run.

**Decision — Security domain expansion (G-06 through G-10) is high priority if the suite is used for any multi-user or regulated project.**
The current Security domain is calibrated for a single-user local tool. It should be clearly marked as such and expanded before being applied to a system with authentication, multiple users, or regulated data.

**Decision — G-02 (Performance) and G-04 (Operational Readiness) are deferred indefinitely for portfolio projects.**
These gaps are real for production systems. They are not relevant to the current project context. Re-evaluate if a portfolio project is deployed for real users.

### Suite changes made as a result of this run

None. All findings logged as open gaps. Changes to domain files should be made in separate commits and referenced in the next run entry.
