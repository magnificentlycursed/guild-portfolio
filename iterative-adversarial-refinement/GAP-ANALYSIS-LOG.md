# IAR Suite Gap Analysis Log

This log tracks gap analysis runs against the IAR suite itself. It is distinct from domain review logs, which evaluate specific projects. This file evaluates the suite's own coverage, completeness, and fitness for different project contexts.

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
| G-18 | Requirements and Business Analysis domain missing | Missing domain | Critical | High | Open | 2026-04-25 | 2026-04-25 |
| G-19 | Documentation fidelity domain missing | Missing domain | High | Medium | Open | 2026-04-25 | 2026-04-25 |
| G-20 | No AI assumption surfacing mechanism across domains | AI-workflow gap | High | High | Open | 2026-04-25 | 2026-04-25 |
| G-21 | No AI hallucination detection across domains | AI-workflow gap | High | High | Open | 2026-04-25 | 2026-04-25 |
| G-22 | No AI context drift / consistency checking across domains | AI-workflow gap | High | Medium | Open | 2026-04-25 | 2026-04-25 |
| G-23 | No dependency/API existence validation | AI-workflow gap | High | Medium | Open | 2026-04-25 | 2026-04-25 |
| G-24 | QE: no test gaming detection (AI validates its own implementation) | AI-workflow gap | High | Medium | Open | 2026-04-25 | 2026-04-25 |
| G-25 | Security: no AI-generated code anti-pattern review | AI-workflow gap | High | Medium | Open | 2026-04-25 | 2026-04-25 |
| G-26 | Change Management and Adoption domain missing | Missing domain | High | Low | Open | 2026-04-25 | 2026-04-25 |
| G-27 | Knowledge Transfer and Handoff domain missing | Missing domain | High | Low | Open | 2026-04-25 | 2026-04-25 |
| G-28 | Client/Stakeholder Alignment domain missing (consulting) | Missing domain | Critical | Medium | Open | 2026-04-25 | 2026-04-25 |
| G-29 | Discovery/Advisory: research quality and source validation unowned | Engagement-type gap | High | High | Open | 2026-04-25 | 2026-04-25 |
| G-30 | Feature Enhancement: existing system compatibility and upgrade burden unowned | Engagement-type gap | High | Low | Open | 2026-04-25 | 2026-04-25 |
| G-31 | Professional and engagement liability unowned (consulting) | Missing domain | Critical | Low | Open | 2026-04-25 | 2026-04-25 |
| G-32 | SA: no integration architecture review | Dimension gap | High | Low | Open | 2026-04-25 | 2026-04-25 |
| G-33 | No sycophancy detection across domains (agent agrees with everything) | AI-workflow gap | High | Critical | Addressed | 2026-04-25 | 2026-04-25 |
| G-34 | No learning/craft development assessment (portfolio: do you understand what was built?) | Personal-use gap | N/A | High | Open | 2026-04-25 | 2026-04-25 |
| G-35 | No future-maintainability-for-one assessment (will future-you understand this?) | Personal-use gap | Low | High | Addressed | 2026-04-25 | 2026-04-25 |
| G-36 | No side-business transition readiness assessment | Personal-use gap | N/A | High | Open | 2026-04-25 | 2026-04-25 |
| G-37 | No session continuity / AI context preservation across sessions | AI-workflow gap | Medium | Critical | Addressed | 2026-04-25 | 2026-04-25 |
| G-38 | Complexity trap: AI over-engineers for personal-scale maintenance | Personal-use gap | Low | High | Addressed | 2026-04-25 | 2026-04-25 |
| G-39 | No DESIGN.md fitness check: assignment compliance not evaluated | Dimension gap | High | High | Addressed | 2026-04-26 | 2026-04-26 |
| G-40 | No VDD process fidelity check (layer gates, TDD discipline, IAR at each merge) | Dimension gap | High | High | Addressed | 2026-04-26 | 2026-04-26 |
| G-41 | No MVR exit signal: hallucinated findings not a recognized classification | AI-workflow gap | High | High | Addressed | 2026-04-26 | 2026-04-26 |
| G-42 | Manual testing checklists not owned by any domain | Dimension gap | Medium | Medium | Addressed | 2026-04-26 | 2026-04-26 |
| G-43 | Commit history quality / linear accountability not evaluated | Dimension gap | Medium | Medium | Addressed | 2026-04-26 | 2026-04-26 |
| G-44 | Same-session sycophancy drift across domains (no session isolation guidance) | AI-workflow gap | Medium | Medium | Addressed | 2026-04-26 | 2026-04-26 |
| G-45 | Portfolio-arc perspective absent (suite evaluates projects, not the arc between them) | Personal-use gap | Low | High | Addressed | 2026-04-26 | 2026-04-26 |
| G-46 | SO split identity: spec contract mixed with process governance | Structural gap | High | High | Addressed | 2026-04-26 | 2026-04-26 |
| G-47 | Suite described as pre-merge gate, not iterative refinement loop | Structural gap | High | High | Addressed | 2026-04-26 | 2026-04-26 |
| G-48 | QE/SE domain boundary not explicit — overlapping correctness coverage | Structural gap | Medium | Medium | Addressed | 2026-04-26 | 2026-04-26 |
| G-49 | PE posture misrepresented as adversarial review; sycophancy check too generic | Structural gap | Low | Low | Addressed | 2026-04-26 | 2026-04-26 |
| G-50 | No generalist adversary pass (unstructured review complementing specialists) | Missing capability | Low | Medium | Addressed | 2026-04-26 | 2026-04-26 |
| G-51 | No VDD-IAR Alignment domain: process compliance unowned | Missing domain | High | High | Addressed | 2026-04-26 | 2026-04-26 |

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

---

## Run 2 — 2026-04-25 21:30Z

**Context:** General-purpose gap analysis against a professional consulting firm's software implementation practice. Evaluated across three engagement types: (1) discovery/advisory — research, current-state assessment, recommendations, roadmap; (2) greenfield implementation — full build from scratch; (3) feature enhancement — adding to an existing client-owned system. Specific lens: the suite is designed to mitigate the risks of AI-accelerated workflows and apply adversarial pressure to keep the agent honest and on task. Assessed which consulting roles and responsibilities have no corresponding review coverage.

**Suite state at time of run:** Same as Run 1 — eight domains, 32 existing gaps from Run 1 carried forward.

**Prior gap review:** All Run 1 gaps remain open unless noted. No suite changes were made between runs.

---

### Findings

#### AI-accelerated workflow gaps

These are the gaps most specific to this suite's stated purpose. An adversarial review process for human-written code has a different risk profile than one for AI-generated code. The agent introduces failure modes that human engineers do not.

**G-20 — No assumption surfacing mechanism across domains**
AI agents make assumptions constantly — about requirements, about what the client "probably" wants, about what constitutes standard practice, about what a library does. Most of these assumptions are never made explicit. They are baked silently into implementations, tests, and documentation. In a human workflow, a code review surfaces surprising choices for discussion. In an AI workflow, surprising choices look like confident, fluent code and are easy to miss.

No domain in the current suite asks: *what assumptions did the agent make, and are they correct?* This is the most pervasive AI-specific risk. Every domain review is implicitly checking outputs, but none is explicitly reconstructing and validating the premises behind those outputs.

In a consulting context, an unvalidated assumption is a change order waiting to happen. The client's understanding of what was agreed and the agent's interpretation of the spec will diverge silently if no one is looking for it.

**G-21 — No hallucination detection across domains**
AI agents confidently cite APIs that do not exist, invent package names, misremember library interfaces, and describe behaviors that are plausible but wrong. This is categorically different from a human writing incorrect code — the agent produces fluent, well-formatted, confident output that does not signal its own incorrectness.

The Quality Engineering domain requires tests to be falsifiable, which catches some hallucinated implementations at runtime. The Software Engineering domain checks correctness. But neither domain explicitly directs the reviewer to verify that referenced external components — libraries, APIs, services, third-party integrations — actually exist and behave as described. A hallucinated dependency is not a style issue; it is a project blocker.

For consulting work, hallucinated integrations discovered late in delivery are a scope and timeline crisis.

**G-22 — No context drift / consistency checking across domains**
AI agents working across long sessions or multiple sessions lose track of earlier decisions. An architectural choice made in session 1 may be silently contradicted in session 4. A constraint established in the spec may be forgotten by the time the relevant feature is implemented. Tests may be written against an earlier version of the interface than the implementation uses.

No domain currently asks: *are decisions made early in this project still reflected in the current state of the code?* This is distinct from a regression test — it is a coherence audit. In a consulting engagement where work spans weeks and multiple AI sessions, coherence drift is a first-class risk.

**G-23 — No dependency and API existence validation**
Distinct from hallucination detection in that this is checkable: does the package exist in the registry? Does the API endpoint exist and return the documented shape? Does the third-party service have the capabilities assumed? This should be an explicit checklist item, not an incidental catch.

**G-24 — QE: no test gaming detection**
An AI agent that writes both the implementation and the tests has an inherent conflict of interest. It will write tests that validate its own interpretation of the requirement, not tests that would catch if its interpretation was wrong. The existing QE falsifiability dimension asks whether tests would catch a *broken implementation*. This gap is different: it asks whether tests would catch a *correct implementation of the wrong requirement*. An agent that misunderstood the spec will often produce a consistent implementation-and-test-suite that passes completely while delivering the wrong thing.

This is the most dangerous AI-specific quality risk. A human engineer who misunderstands a requirement tends to ask a question. An agent produces a complete, passing solution.

**G-25 — Security: no AI-generated code anti-pattern review**
Large language models have documented tendencies toward specific insecure patterns: hardcoded credentials used as examples that persist into production, overly permissive CORS configurations, SQL concatenation that looks parameterized but is not, JWT verification that checks format but not signature, copy-paste of deprecated cryptographic functions. These patterns appear in AI output with higher frequency than in experienced human output because the model is pattern-matching on training data that includes insecure examples.

The Security domain currently reviews outputs against standard dimensions. It does not explicitly direct the reviewer to look for AI-specific generation anti-patterns. This should be a named checklist, not an implicit catch.

---

#### Consulting role and responsibility gaps

A professional consulting firm's implementation team includes roles whose concerns are entirely unrepresented in the current suite. Each unrepresented role is a category of delivery risk with no adversarial review.

**G-18 — Requirements and Business Analysis domain missing**
The Business Analyst role bridges client needs and technical implementation. The SA review evaluates architectural soundness; the SO review evaluates spec compliance. Neither evaluates whether the spec itself was correct. In a consulting engagement, requirements are gathered from client interviews, existing documentation, and stakeholder workshops — all of which are lossy and subject to misinterpretation. AI agents working from these inputs will extrapolate, fill gaps, and make the spec more internally consistent than the client's actual intent.

No domain asks: do the requirements accurately reflect what the client needs? Are user stories testable and unambiguous? Are acceptance criteria written so that both the client and the implementation team would agree on whether they are met? Is there a traceability map from client need to implemented feature?

In an AI-accelerated workflow, this gap is compounded: the agent will generate plausible-sounding requirements and acceptance criteria from minimal input, and those generated artifacts will look complete even when they are not.

**G-26 — Change Management and Adoption domain missing**
A technically perfect solution that is rejected by end users is a failed consulting engagement. Change management covers: stakeholder communication, training materials, rollout planning, resistance identification, and adoption measurement. In consulting, adoption failure is a reputational risk — the client blames the firm for a system no one uses.

No domain in the current suite asks whether the deliverables enable successful adoption. For AI-accelerated workflows specifically, there is an additional risk: the agent has no model of organizational politics, change fatigue, or user resistance. It will produce functional software with no awareness of whether the target users are ready or willing to use it.

**G-27 — Knowledge Transfer and Handoff domain missing**
Consulting engagements end. The client must own and operate what was built, often with a team that was not present for the build. No domain evaluates whether the deliverables enable handoff: is the code understandable without the AI conversation history? Are architectural decisions documented in a way a new maintainer can act on? Are there onboarding materials for the client's engineering team?

For AI-accelerated workflows, this gap is acute. Code generated by an agent may be correct and functional but written in a style that reflects the agent's training rather than the team's conventions, making it harder for the client's engineers to maintain. Documentation generated alongside code may not survive future changes unless someone owns its accuracy.

**G-28 — Client/Stakeholder Alignment domain missing**
The SO enforces spec compliance, but the spec is an artifact of a negotiation between the consulting firm and the client. As work progresses, the client's understanding of what was agreed and the firm's implementation may diverge without either side realizing it. Client expectations drift; the spec does not.

No domain regularly asks: would the client recognize this as what they asked for? Are demos and status updates accurately representing current system state? Are there unresolved ambiguities in the agreed scope that will surface as disputes at delivery?

In AI-accelerated workflows, this risk is amplified because the pace of delivery creates less opportunity for the organic alignment that happens during slower human-paced development.

**G-31 — Professional and engagement liability unowned**
Consulting firms carry professional liability: errors and omissions, breach of contract, intellectual property indemnification. No domain evaluates the firm's own exposure: are deliverables clearly scoped so that disputes about what was delivered can be resolved against a documented record? Is IP ownership of AI-generated code documented and agreed upon? Are there deliverables that could expose the firm to claims if they contain errors? Are third-party components used under licenses compatible with the client's intended use?

This is distinct from compliance (G-01), which covers the client's regulatory exposure. This covers the firm's own exposure as a service provider.

---

#### Engagement-type gaps

**G-29 — Discovery/Advisory: research quality and source validation unowned**
In a discovery or advisory engagement, the primary deliverable is analysis and recommendations. No domain evaluates the quality of the research underpinning those recommendations: are sources cited and verifiable? Are findings based on the client's actual situation or on generic best practices applied without validation? Are assumptions about the client's constraints made explicit and confirmed?

For AI-accelerated discovery work, this gap is critical. An agent conducting discovery analysis will produce confident, well-structured findings that may be based on pattern-matched generalizations rather than evidence from the specific client context. The output will look like rigorous research. The adversarial question — *is this actually true for this client?* — has no domain to ask it.

**G-30 — Feature Enhancement: existing system compatibility and upgrade burden unowned**
When adding a feature to an existing client-owned system, the primary risks are different from greenfield: does the enhancement fit the existing codebase's patterns, conventions, and constraints? Does it create upgrade or maintenance burdens the client did not agree to (new dependencies, build toolchain changes, runtime version requirements)? Does it create technical debt in the existing system that will outlast the engagement?

The Software Engineering domain reviews code quality. It does not evaluate compatibility with an existing system the client owns and will maintain. The Solution Owner guards against scope additions within the engagement; it does not evaluate the engagement's footprint on the client's broader codebase.

**G-32 — SA: no integration architecture review**
For consulting engagements — especially greenfield implementations — the system must connect to what it needs to connect to: existing client systems, third-party services, authentication providers, data sources. No dimension in the SA review explicitly evaluates integration architecture: are integration points identified and designed? Are interface contracts with external systems documented? Are integration failure modes handled? Are there single points of failure in the integration layer?

In AI-accelerated workflows, integrations are a hallucination risk. The agent will design integration patterns based on its training data, which may not reflect the specific versions, quirks, or constraints of the client's actual systems.

---

#### Documentation gap

**G-19 — Documentation fidelity domain missing**
AI agents generate documentation in parallel with code. This creates a specific risk: the documentation and the implementation are generated from the same prompt interpretation, so both can be consistently wrong in the same way. More commonly, documentation is generated once and the code changes; without a domain that owns documentation accuracy, the gap widens over time.

No domain currently asks: does the documentation accurately describe the system as it exists today? Are API contracts documented and correct? Do user guides match actual user flows? Are architectural diagrams current? In a consulting context, documentation is often a contractual deliverable — inaccurate documentation is a delivery failure.

---

### Decisions

**Decision — AI-workflow gaps (G-20 through G-25) should be incorporated as cross-cutting dimensions, not a separate domain.**
Each existing domain already reviews outputs in its area. Rather than creating an "AI Review" domain (which would be redundant), the AI-specific risks should be added as explicit named dimensions within the domains best positioned to catch them: assumption surfacing and consistency checking across all domains; dependency validation in QE and SA; test gaming detection in QE; AI anti-pattern review in Security. This is a suite-wide amendment, not a new domain. Log as high priority for next suite update.

**Decision — G-18 (Requirements/BA), G-28 (Client Alignment), and G-31 (Engagement Liability) are critical for consulting use and require new domains.**
These three gaps represent consulting-specific failure modes with no current coverage. They are not addressed by adding dimensions to existing domains — they require dedicated adversarial review with their own prompts and standard dimensions. Prioritize before using this suite on a client engagement.

**Decision — G-26 (Change Management) and G-27 (Knowledge Transfer) are high priority for greenfield and feature enhancement engagements, lower for discovery/advisory.**
These are real gaps but engagement-type dependent. Add as domains when the suite is first used on a greenfield implementation.

**Decision — G-29 (Discovery/Advisory research quality) requires a domain or pre-engagement checklist before AI-accelerated discovery work.**
An agent conducting discovery analysis with no adversarial check on research quality is a specific and underappreciated risk. Before using this suite on a discovery engagement, define what "valid evidence" means and how to distinguish it from plausible pattern-matching.

**Decision — G-30 (Feature enhancement compatibility) and G-32 (Integration architecture) can be addressed as dimensions within SA and SE, not new domains.**
These are scoped additions to existing domains. Add them in the next suite update pass.

**Decision — G-19 (Documentation fidelity) warrants a dedicated domain.**
Documentation accuracy is a contractual concern in consulting, a knowledge transfer concern at handoff, and a correctness concern in AI-accelerated workflows where docs and code drift from the same initial prompt. It cannot be adequately covered by adding a dimension to an existing domain because it spans all delivery artifacts across the full engagement.

### Suite changes made as a result of this run

None. All findings logged. Changes to domain files should be made in separate commits and referenced in Run 3.

---

## Run 3 — 2026-04-25 22:00Z

**Context:** Personal developer using AI-accelerated tools. Single-user scope, no team, no client. Project may be: personal use only, a portfolio piece, or a side business in development. Goal is professional-quality software with adversarial mitigation of AI workflow risks. This is also the context closest to the suite's origin project (bookmark-manager).

**Suite state at time of run:** Same eight domains. 32 gaps from Run 1, 15 new gaps from Run 2. Run 2 not yet committed.

**Prior gap review:** All prior gaps carried forward. Severities re-evaluated for personal context below where they change materially.

---

### Re-evaluation of prior gaps in personal context

**G-01 (Compliance/Legal):** Low — unless the project handles other people's data or grows into a side business with users. Re-evaluate at that transition.

**G-02 (Performance/Scalability):** Low — you are the only user. Promote to Medium if it becomes a side business.

**G-04 (Operational Readiness):** Low — you operate it yourself. "Know how to restart it" is sufficient.

**G-05 (Delivery Governance):** Changes character. No budget, no team. The personal equivalent is: *am I making progress toward something I will actually finish and use, or am I building indefinitely?* Completion discipline and "done" criteria are real risks on personal projects. Related to G-15 (kill criteria).

**G-18 (Requirements/BA):** Stays relevant but changes character. You are both client and implementer. The gap is no longer between client intent and written spec — it is between what you asked the agent for and what you actually needed. You are just as capable of mis-specifying your own requirements as a client is.

**G-20 (Assumption surfacing):** Promotes to Critical for personal use. In a team or consulting context, someone else may catch an unvalidated assumption. As a solo developer with an AI agent, you are the only human in the loop. If the agent makes a wrong assumption and you do not catch it, no one will.

**G-21 (Hallucination detection):** Promotes to Critical for the same reason. No peer review exists. The adversarial review process is the only check.

**G-22 (Context drift/consistency):** High. Long solo AI sessions are particularly prone to this. You may not notice that a decision made in session 1 was quietly reversed in session 4 because you were not tracking it consciously across sessions.

**G-24 (Test gaming):** High. You wrote the spec and you are the only reviewer. An agent that misunderstood your requirement will produce a consistent implementation and test suite that passes completely. The adversarial check that someone else would provide in code review does not exist.

**G-26 (Change Management):** Not applicable — no organization, no users to adopt.

**G-27 (Knowledge Transfer):** Changes character significantly — see G-37 below.

**G-28 (Client Alignment):** Not applicable — no client.

**G-31 (Engagement Liability):** Not applicable unless it grows into a business with contracts.

---

### New gaps

**G-33 — No sycophancy detection mechanism**
AI agents are trained to be helpful and agreeable. When you propose a direction, the agent will generally support it, improve on it, and implement it — even if it is wrong. It will not tell you that your data model is misconceived, that your feature idea does not solve the actual problem, or that the approach you are enthusiastic about is the wrong one. It will ask clarifying questions and then build what you described.

Human collaborators push back. The agent does not, unless explicitly prompted to. No domain in the current suite asks: *did the agent challenge any of the key decisions in this session, or did it agree with everything?* Agreement is a red flag in adversarial review. An AI reviewer that finds nothing is more suspicious than one that finds something.

This is the foundational risk of solo AI-accelerated workflows that the rest of the suite partially addresses but never names directly.

**G-34 — No learning and craft development assessment**
If you are building a portfolio piece, the adversarial question is not just whether the software is good — it is whether building it made you better. An agent that produces professional-quality code you do not fully understand is not a portfolio win; it is a liability when you are asked to explain or extend it. No domain asks: do you understand what was built well enough to own it? Could you reproduce the key decisions without the agent?

This matters for portfolio integrity (are you accurately representing your skills?) and for professional development (are you growing, or producing artifacts?). An AI-accelerated portfolio that demonstrates the agent's competence rather than yours is a specific and underappreciated risk.

**G-35 — No future-maintainer assessment (maintainability-for-one)**
In a team context, maintainability is about other engineers understanding the code. For a personal project, the other engineer is future-you — often six or twelve months later, with no memory of the original AI sessions, no access to the conversation history, and a different mental model of the problem.

AI-generated code tends to be correct and functional but written in a style that reflects training data rather than your natural idioms. It may use patterns you would not have chosen, at a level of abstraction that made sense in the original session but is opaque later. No domain currently asks: will future-you be able to understand and modify this without re-running the AI session?

This is distinct from SE dimension 10 (consistency) and from G-27 (knowledge transfer for handoff). It is about your own continuity of understanding over time.

**G-36 — No side-business transition readiness assessment**
A personal project that grows into a side business crosses a threshold where many previously-deferred gaps become relevant simultaneously: compliance (G-01), privacy (G-03), performance (G-02), operational readiness (G-04), and potentially security expansion (G-06 through G-10). If the project was not structured with that transition in mind, the cost of crossing that threshold is high.

No domain currently asks: if this project were to acquire its first paying user tomorrow, what would immediately break or expose liability? What would need to change before that could happen safely? For a personal project with side-business ambitions, this forward-looking question is more useful than most of the current review dimensions.

**G-37 — Knowledge transfer to future-self (session continuity)**
Distinct from G-35 (future maintainability of the code) and G-27 (handoff to another person). This is about the continuity of the AI-assisted development process itself. AI sessions have no memory across conversations. Decisions, context, constraints, and rationale established in one session are not available in the next unless explicitly preserved.

No domain currently asks: is the project's state — its decisions, its constraints, its open questions, its known debt — documented well enough that a new AI session can be productive without rediscovering everything from scratch? For a personal developer working in AI-accelerated sessions over weeks or months, this is a first-class workflow risk. Lost context means repeated mistakes, contradictory decisions, and work that has to be redone.

**G-38 — Complexity trap: AI over-engineers for personal scale**
AI agents produce professional-grade complexity by default. Given a personal tool that needs to store 50 bookmarks, an unconstrained agent may produce a layered architecture, an abstracted storage interface, a full test suite with mocking infrastructure, and a CI pipeline. These choices may be technically correct and individually justifiable. Together, for a project you maintain alone, they create a complexity budget that exceeds what one person can comfortably own.

The Solution Owner domain guards against scope additions (features). This gap is different: it is about architectural and infrastructural complexity added not by user request but by the agent's default inclination toward "proper" engineering. A personal project should be engineered for the person who maintains it, not for a hypothetical team. No domain currently asks whether the complexity level is appropriate for a one-person maintenance model.

---

### Gaps that do not apply in personal context

The following gaps from prior runs are not applicable to a solo personal developer with no clients, no team, and no regulated data. They should be re-evaluated if the project acquires users, a team, or a business structure.

- G-26 (Change Management / Adoption) — no organization to manage
- G-28 (Client/Stakeholder Alignment) — no client
- G-29 (Discovery/Advisory research quality) — not an advisory engagement
- G-31 (Professional and engagement liability) — no client contract
- G-04 (Operational Readiness) — no SLA, no on-call, no users depending on uptime
- G-09 (Audit logging) — no multi-user accountability surface

---

### Decisions

**Decision — The core value of the IAR suite for a personal developer is adversarial peer review substitution.**
In a team context, the suite augments human review. For a solo developer, it replaces it entirely. This changes the stakes for every domain: there is no fallback. The suite is the only check. This should be stated explicitly in the suite README as a use-case note.

**Decision — G-33 (sycophancy detection) should be added as a cross-cutting dimension to every domain.**
Each domain review should explicitly ask whether the agent challenged any key decisions in its area or agreed with everything. Universal agreement is a warning sign, not a passing grade. This is a lightweight addition — one sentence per domain — but it names a risk that currently goes unnamed.

**Decision — G-34 (learning/craft) and G-36 (side-business transition readiness) are portfolio-specific concerns that warrant a lightweight checklist, not a full domain.**
These are pre-project or post-project questions rather than per-layer review dimensions. Add as a framing document in the suite for portfolio use.

**Decision — G-35 (future maintainability) and G-37 (session continuity) should be added as dimensions to SA and SE respectively.**
G-35 fits in SE (code understandable to future-you). G-37 fits in SA (architectural decisions preserved across sessions). Both are small additions to existing domains.

**Decision — G-38 (complexity trap) should be added to the SO domain as a named dimension.**
The SO currently blocks scope additions. It should also explicitly evaluate whether the complexity level — architectural, infrastructural, toolchain — is appropriate for the number of people who will maintain it.

**Decision — G-01, G-02, G-03, G-04 are deferred for personal use but should be treated as a transition checklist for side-business growth.**
Rather than waiting for these gaps to be discovered when the project crosses a threshold, document the transition criteria explicitly. A framing document titled something like "Before your first user" would serve this purpose.

### Suite changes made as a result of this run

**G-33 addressed** — Sycophancy check added to all eight domain prompts (QE, UX, Security, PE, SA, SO, SE, DE). Wording is consistent across all domains: universal agreement by the agent is treated as a finding, not a pass.

**G-35 addressed** — Future-self maintainability added as dimension 11 to SOFTWARE-ENGINEERING-REVIEW.md. Asks whether the code is understandable to future-you without access to the original AI session.

**G-37 addressed** — Session continuity added as dimension 11 to SOLUTION-ARCHITECT-REVIEW.md. Asks whether architectural decisions are documented in a durable form outside conversation history.

**G-38 addressed** — Complexity budget for one added as dimension 9 to SOLUTION-OWNER-REVIEW.md. Distinct from over-engineering (beyond spec) — this flags complexity proportionate to the spec but disproportionate to a single maintainer.

**Remaining open from Run 3:** G-34 (learning/craft checklist), G-36 (side-business transition checklist). Tier 2 items — new documents, not domain edits. Deferred to next pass.

---

## Run 4 — 2026-04-26 00:00Z

**Context:** Personal developer, AI-accelerated workflow, portfolio-to-journeyman arc. Full evaluation of suite against the guild apprentice-onboarding methodology, the bookmark-manager project history (including the dollspace-gay guild review finding), and the upcoming issue-tracker-cli project. Goal: identify gaps that would cause the suite to pass a project that a guild portfolio reviewer would fail.

**Suite state at time of run:** Eight domains, all with sycophancy checks and language/interface supplements. lang/ subfolder with rust.md, javascript-typescript.md, cli.md, browser-app.md. GAP-ANALYSIS-LOG.md with 38 gaps registered.

### Findings

**G-39 — DESIGN.md fitness check missing (High)**

The SO domain treats DESIGN.md as the authoritative contract and checks whether the implementation matches it. But DESIGN.md is a student document, and it can itself be wrong. The bookmark-manager's DESIGN.md specified TypeScript, Vite, and a build toolchain — none of which were asked for by the assignment. Every SO dimension passed. The guild reviewer failed the project on assignment compliance.

The suite had no mechanism to catch scope creep that entered at the design stage, only scope creep that entered during implementation. The upstream assignment brief is the higher-level contract.

*Decision:* Add SO dim 10 (assignment compliance). Requires reading the assignment instructions alongside DESIGN.md and flagging deviations that entered at the design stage.

**G-40 — VDD process fidelity unowned (High)**

The guild portfolio review is explicitly "process over product." Reviewers look at whether the VDD loop was followed: design doc before code, layered development, IAR at each gate, tests alongside or before implementation. Nothing in the suite checked this. A project that is correctly built but built without process discipline would pass all eight domains and still fail a portfolio review.

*Decision:* Add SO dim 11 (VDD process fidelity). The SO domain is the right owner — it already guards the spec contract, and process fidelity is the meta-contract above it.

**G-41 — No MVR exit signal or hallucinated finding classification (High)**

The methodology document describes a specific exit condition: when the adversary starts hallucinating critiques because it cannot find real ones, the code has reached maximum viable refinement. This is the most important signal in the VDD loop. The suite had no classification for hallucinated findings, no guidance on recognizing the MVR signal, and no way to record that exit in the log.

This meant a reviewer running the suite could not distinguish: (a) a clean pass because the code is good, (b) a clean pass because the agent was too agreeable, or (c) an exit because the adversary ran out of real complaints. These are three very different states.

*Decision:* Add **hallucinated** classification to all 8 domain prompts. Add MVR exit signal explanation to README.

**G-42 — Manual testing checklists not owned (Medium)**

The bookmark-manager DESIGN.md included explicit manual testing checklists as part of the layer gate. The DECISIONS.md documents three cases where manual testing caught things automated tests missed (tag toggle deselect, empty URL message, focus after cancel). No domain asked whether manual checklists existed or were completed. This gap is especially significant for CLI projects where automated tests cannot cover all UX concerns.

*Decision:* Add QE dim 14 (manual testing checklists).

**G-43 — Commit history and linear accountability not evaluated (Medium)**

The methodology's "string of beads" principle: every piece of code traces to a sub-issue, every sub-issue to an issue. The portfolio review looks at commit history. A commit log of "fix stuff" or "wip" is a process failure. No domain evaluated commit message quality or traceability. A project could have excellent code and a useless commit history.

*Decision:* Add SO dim 12 (linear accountability).

**G-44 — Same-session sycophancy drift across domains (Medium)**

Each domain has a sycophancy check. But if all eight domains run in the same AI session, the agent accumulates context that softens its adversarial pressure. The methodology's "fresh eyes every time" principle argues for resetting context between rounds. The suite had no guidance on session isolation between domain reviews.

*Decision:* Add session isolation operational note to README under Full run.

**G-45 — Portfolio-arc perspective absent (High for personal use)**

Per-project IAR runs evaluate individual projects. The portfolio review evaluates the arc across all projects: growth, honest retrospectives, independence, assignment alignment patterns. The suite had no guidance for this cross-project perspective. A student who ran excellent IAR on each project but never assembled the arc-level view would miss what portfolio reviewers actually evaluate.

*Decision:* Add portfolio-arc review section to README.

### Suite changes made as a result of this run

**G-39 addressed** — SO dim 10 (assignment compliance) added. Explicitly requires reading the upstream assignment brief alongside DESIGN.md.

**G-40 addressed** — SO dim 11 (VDD process fidelity) added. Checks commit history for layered development, design doc before code, IAR at each gate.

**G-41 addressed** — **hallucinated** classification added to all 8 domain prompts. MVR exit signal explanation and session isolation note added to README.

**G-42 addressed** — QE dim 14 (manual testing checklists) added.

**G-43 addressed** — SO dim 12 (linear accountability) added.

**G-44 addressed** — Session isolation note added to README under Full run.

**G-45 addressed** — Portfolio-arc review section added to README.

**Remaining open:** G-34, G-36 (deferred from Run 3). G-01 through G-32 remain open; most are scoped to contexts (mission-critical teams, consulting engagements) not yet relevant to the current personal portfolio use case.

---

## Run 5 — 2026-04-26 01:00Z

**Context:** Organizational evaluation of the suite's structure and alignment with VDD-IAR. Prompted by the question: is this the right organization? Are additional domains needed? Does the suite reflect what VDD-IAR actually is?

**Suite state at time of run:** Nine domains after this run (previously eight). README restructured. VDD-IAR Alignment domain created. SO stripped to spec-contract identity.

### Findings

**G-46 — SO split identity (High)**

The SO domain had accumulated two distinct adversarial postures: spec-contract review (does the implementation match what was asked?) and process governance (was the work done correctly?). These read different artifacts, apply different adversarial frames, and belong in separate sessions. Having them in one domain created a reviewer that had to context-switch mid-review.

*Decision:* Strip SO to spec-contract focus (9 dims). Move process concerns to VDD-IAR Alignment. Move complexity budget for one to SA.

**G-47 — Suite described as gate, not iterative loop (High)**

The README described IAR as a pre-merge gate: "a full run is required before merging." VDD-IAR is a loop: build → adversary → fix → adversary again → repeat until MVR. The suite name says "Iterative" but the documented structure was a single checkpoint. No guidance existed for within-layer iteration, round numbering, or when to stop iterating.

*Decision:* Rewrite README. Replace "Full run" with "Refinement loop" section. Add round-number requirement to log format. Update merging gate to require MVR, not just one passing run.

**G-48 — QE/SE domain overlap without explicit boundary (Medium)**

SE dim 1 (correctness, logic errors) and QE dim 7 (logic errors) covered the same ground. In practice both reviewers would find the same bugs. The distinction — QE owns tests, SE owns code — was valid but unstated, creating duplicated effort without the benefit of independent confirmation.

*Decision:* Add domain boundary statements to QE and SE prompts. QE flags missing tests when it finds a logic error; SE flags the bug. Both findings are valid independently.

**G-49 — PE posture misrepresented (Low)**

The generic sycophancy check ("if the agent agreed with every decision...") doesn't fit a domain where most dimensions are binary existence checks. The real sycophancy risk in PE is rationalized inapplicability decisions, not agreeing with code quality judgments.

*Decision:* Replace generic sycophancy check with a posture note specific to PE's compliance-check nature. Scope the adversarial pressure to judgment-dependent decisions.

**G-50 — No generalist adversary pass (Medium for personal use)**

The IAR specialists each apply a specific framework. The VDD methodology's adversary has no framework — it just finds everything wrong. No domain covered the gaps between specialist frameworks.

*Decision:* Document as an optional unstructured pass in the README (not a formal domain). It intentionally has no dimensions — adding structure would make it another specialist.

**G-51 — VDD-IAR Alignment domain missing (High)**

Process compliance had no owner. Test discipline, layer gate compliance, IAR fresh context, IAR iteration, role integrity, and retrospective quality were either scattered across SO (awkwardly) or unowned. The methodology's "process over product" principle had no adversarial review mechanism.

*Decision:* Create VDD-IAR-ALIGNMENT-REVIEW.md. Ten dimensions covering the full VDD-IAR loop. Runs last in the sequence (reviews artifacts produced by all other domain runs). Mandatory gate before merge.

### Suite changes made as a result of this run

**G-46 addressed** — SO reduced to 9 dimensions (spec/contract focus). Dims 9 (complexity for one), 11 (VDD fidelity), 12 (linear accountability) removed and redistributed. Complexity budget for one moved to SA dim 9 expansion.

**G-47 addressed** — README rewritten around VDD-IAR as the governing framework. "Refinement loop" replaces "Full run." Merging gate updated to require MVR and round numbers.

**G-48 addressed** — Domain boundary notes added to QE and SE prompts.

**G-49 addressed** — PE sycophancy check replaced with a posture note specific to compliance-check domains.

**G-50 addressed** — Generalist adversary pass documented as optional README note.

**G-51 addressed** — VDD-IAR-ALIGNMENT-REVIEW.md created. Added to domain table, sequencing, and merging gate.

**Remaining open:** G-34, G-36. No new gaps identified in this run.
