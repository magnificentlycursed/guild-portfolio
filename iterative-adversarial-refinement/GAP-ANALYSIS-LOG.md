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
6. Record findings in a new run entry in `SUITE-REVIEW.md` under a `## Gap Analysis Run N — date` section.
7. Update the gap registry: change statuses, add new entries, close addressed ones.
8. If a gap is addressed by changing a domain file, commit those changes separately and reference the commit here.

## Gap Registry

Living table of all identified gaps. Update statuses here as gaps are addressed or dismissed. Do not delete rows — mark them Addressed or Dismissed with rationale. Each ID links to the gap analysis run where it was first identified.

| ID | Gap | Type | Mission-Critical Severity | Speculative Severity | Status | Opened | Last Reviewed |
|---|---|---|---|---|---|---|---|
| [G-01](SUITE-REVIEW.md#gap-analysis-run-1--2026-04-25-2000z) | Compliance and Legal domain missing | Missing domain | Critical | Low–Medium | Open | 2026-04-25 | 2026-04-25 |
| [G-02](SUITE-REVIEW.md#gap-analysis-run-1--2026-04-25-2000z) | Performance and Scalability domain missing | Missing domain | Critical | Defer | Addressed | 2026-04-25 | 2026-04-27 |
| [G-03](SUITE-REVIEW.md#gap-analysis-run-1--2026-04-25-2000z) | Privacy domain missing (listed as candidate) | Missing domain | Critical | Medium | Addressed | 2026-04-25 | 2026-04-27 |
| [G-04](SUITE-REVIEW.md#gap-analysis-run-1--2026-04-25-2000z) | Operational Readiness domain missing | Missing domain | Critical | Low | Open | 2026-04-25 | 2026-04-25 |
| [G-05](SUITE-REVIEW.md#gap-analysis-run-1--2026-04-25-2000z) | Delivery Governance missing (timeline, budget, milestones) | Missing domain | Critical | Medium | Open | 2026-04-25 | 2026-04-25 |
| [G-06](SUITE-REVIEW.md#gap-analysis-run-1--2026-04-25-2000z) | Security: no threat modeling | Dimension gap | High | Low | Addressed | 2026-04-25 | 2026-04-27 |
| [G-07](SUITE-REVIEW.md#gap-analysis-run-1--2026-04-25-2000z) | Security: no authentication/authorization review | Dimension gap | High | Low–Medium | Addressed (partial) | 2026-04-25 | 2026-04-28 |
| [G-08](SUITE-REVIEW.md#gap-analysis-run-1--2026-04-25-2000z) | Security: no session management review | Dimension gap | High | Low | Addressed (partial) | 2026-04-25 | 2026-04-28 |
| [G-09](SUITE-REVIEW.md#gap-analysis-run-1--2026-04-25-2000z) | Security: no audit logging requirement | Dimension gap | High | Low | Addressed | 2026-04-25 | 2026-04-28 |
| [G-10](SUITE-REVIEW.md#gap-analysis-run-1--2026-04-25-2000z) | Security: no data classification requirement | Dimension gap | Medium | Low | Addressed | 2026-04-25 | 2026-04-28 |
| [G-11](SUITE-REVIEW.md#gap-analysis-run-1--2026-04-25-2000z) | Solution Owner: no budget tracking dimension | Dimension gap | High | Medium | Open | 2026-04-25 | 2026-04-25 |
| [G-12](SUITE-REVIEW.md#gap-analysis-run-1--2026-04-25-2000z) | Quality Engineering: no integration/contract testing mandate | Dimension gap | High | Low | Addressed (SA Extended: External Interface Contracts) | 2026-04-25 | 2026-04-27 |
| [G-13](SUITE-REVIEW.md#gap-analysis-run-1--2026-04-25-2000z) | Platform Engineering: DR dimension lacks RTO/RPO targets | Dimension gap | Medium | Low | Open | 2026-04-25 | 2026-04-25 |
| [G-14](SUITE-REVIEW.md#gap-analysis-run-1--2026-04-25-2000z) | No domain for learning goals / validation structure (speculative projects) | Missing domain | N/A | Critical | Open | 2026-04-25 | 2026-04-25 |
| [G-15](SUITE-REVIEW.md#gap-analysis-run-1--2026-04-25-2000z) | No kill criteria mechanism (speculative projects) | Missing domain | N/A | High | Open | 2026-04-25 | 2026-04-25 |
| [G-16](SUITE-REVIEW.md#gap-analysis-run-1--2026-04-25-2000z) | No intentional technical debt tracking (speculative projects) | Dimension gap | Low | High | Open | 2026-04-25 | 2026-04-25 |
| [G-17](SUITE-REVIEW.md#gap-analysis-run-1--2026-04-25-2000z) | Solution Architect: no pivot readiness dimension (speculative projects) | Dimension gap | Low | High | Open | 2026-04-25 | 2026-04-25 |
| [G-18](SUITE-REVIEW.md#gap-analysis-run-2--2026-04-25-2130z) | Requirements and Business Analysis domain missing | Missing domain | Critical | High | Open | 2026-04-25 | 2026-04-25 |
| [G-19](SUITE-REVIEW.md#gap-analysis-run-2--2026-04-25-2130z) | Documentation fidelity domain missing | Missing domain | High | Medium | Addressed | 2026-04-25 | 2026-04-27 |
| [G-20](SUITE-REVIEW.md#gap-analysis-run-2--2026-04-25-2130z) | No AI assumption surfacing mechanism across domains | AI-workflow gap | High | High | Addressed (partial) | 2026-04-25 | 2026-04-27 |
| [G-21](SUITE-REVIEW.md#gap-analysis-run-2--2026-04-25-2130z) | No AI hallucination detection across domains | AI-workflow gap | High | High | Addressed (partial) | 2026-04-25 | 2026-04-27 |
| [G-22](SUITE-REVIEW.md#gap-analysis-run-2--2026-04-25-2130z) | No AI context drift / consistency checking across domains | AI-workflow gap | High | Medium | Addressed | 2026-04-25 | 2026-04-28 |
| [G-23](SUITE-REVIEW.md#gap-analysis-run-2--2026-04-25-2130z) | No dependency/API existence validation | AI-workflow gap | High | Medium | Addressed (partial) | 2026-04-25 | 2026-04-27 |
| [G-24](SUITE-REVIEW.md#gap-analysis-run-2--2026-04-25-2130z) | QE: no test gaming detection (AI validates its own implementation) | AI-workflow gap | High | Medium | Addressed (partial) | 2026-04-25 | 2026-04-28 |
| [G-25](SUITE-REVIEW.md#gap-analysis-run-2--2026-04-25-2130z) | Security: no AI-generated code anti-pattern review | AI-workflow gap | High | Medium | Addressed (partial) | 2026-04-25 | 2026-04-28 |
| [G-26](SUITE-REVIEW.md#gap-analysis-run-2--2026-04-25-2130z) | Change Management and Adoption domain missing | Missing domain | High | Low | Open | 2026-04-25 | 2026-04-25 |
| [G-27](SUITE-REVIEW.md#gap-analysis-run-2--2026-04-25-2130z) | Knowledge Transfer and Handoff domain missing | Missing domain | High | Low | Addressed | 2026-04-25 | 2026-04-27 |
| [G-28](SUITE-REVIEW.md#gap-analysis-run-2--2026-04-25-2130z) | Client/Stakeholder Alignment domain missing (consulting) | Missing domain | Critical | Medium | Open | 2026-04-25 | 2026-04-25 |
| [G-29](SUITE-REVIEW.md#gap-analysis-run-2--2026-04-25-2130z) | Discovery/Advisory: research quality and source validation unowned | Engagement-type gap | High | High | Open | 2026-04-25 | 2026-04-25 |
| [G-30](SUITE-REVIEW.md#gap-analysis-run-2--2026-04-25-2130z) | Feature Enhancement: existing system compatibility and upgrade burden unowned | Engagement-type gap | High | Low | Addressed | 2026-04-25 | 2026-04-28 |
| [G-31](SUITE-REVIEW.md#gap-analysis-run-2--2026-04-25-2130z) | Professional and engagement liability unowned (consulting) | Missing domain | Critical | Low | Open | 2026-04-25 | 2026-04-25 |
| [G-32](SUITE-REVIEW.md#gap-analysis-run-2--2026-04-25-2130z) | SA: no integration architecture review | Dimension gap | High | Low | Addressed | 2026-04-25 | 2026-04-28 |
| [G-33](SUITE-REVIEW.md#gap-analysis-run-3--2026-04-25-2200z) | No sycophancy detection across domains (agent agrees with everything) | AI-workflow gap | High | Critical | Addressed | 2026-04-25 | 2026-04-25 |
| [G-34](SUITE-REVIEW.md#gap-analysis-run-3--2026-04-25-2200z) | No learning/craft development assessment (portfolio: do you understand what was built?) | Personal-use gap | N/A | High | Addressed | 2026-04-25 | 2026-04-27 |
| [G-35](SUITE-REVIEW.md#gap-analysis-run-3--2026-04-25-2200z) | No future-maintainability-for-one assessment (will future-you understand this?) | Personal-use gap | Low | High | Addressed | 2026-04-25 | 2026-04-25 |
| [G-36](SUITE-REVIEW.md#gap-analysis-run-3--2026-04-25-2200z) | No side-business transition readiness assessment | Personal-use gap | N/A | High | Dismissed | 2026-04-25 | 2026-04-28 |
| [G-37](SUITE-REVIEW.md#gap-analysis-run-3--2026-04-25-2200z) | No session continuity / AI context preservation across sessions | AI-workflow gap | Medium | Critical | Addressed | 2026-04-25 | 2026-04-25 |
| [G-38](SUITE-REVIEW.md#gap-analysis-run-3--2026-04-25-2200z) | Complexity trap: AI over-engineers for personal-scale maintenance | Personal-use gap | Low | High | Addressed | 2026-04-25 | 2026-04-25 |
| [G-39](SUITE-REVIEW.md#gap-analysis-run-4--2026-04-26-0000z) | No DESIGN.md fitness check: assignment compliance not evaluated | Dimension gap | High | High | Addressed | 2026-04-26 | 2026-04-26 |
| [G-40](SUITE-REVIEW.md#gap-analysis-run-4--2026-04-26-0000z) | No VDD process fidelity check (layer gates, TDD discipline, IAR at each merge) | Dimension gap | High | High | Addressed | 2026-04-26 | 2026-04-26 |
| [G-41](SUITE-REVIEW.md#gap-analysis-run-4--2026-04-26-0000z) | No MVR exit signal: hallucinated findings not a recognized classification | AI-workflow gap | High | High | Addressed | 2026-04-26 | 2026-04-26 |
| [G-42](SUITE-REVIEW.md#gap-analysis-run-4--2026-04-26-0000z) | Manual testing checklists not owned by any domain | Dimension gap | Medium | Medium | Addressed | 2026-04-26 | 2026-04-26 |
| [G-43](SUITE-REVIEW.md#gap-analysis-run-4--2026-04-26-0000z) | Commit history quality / linear accountability not evaluated | Dimension gap | Medium | Medium | Addressed | 2026-04-26 | 2026-04-26 |
| [G-44](SUITE-REVIEW.md#gap-analysis-run-4--2026-04-26-0000z) | Same-session sycophancy drift across domains (no session isolation guidance) | AI-workflow gap | Medium | Medium | Addressed | 2026-04-26 | 2026-04-26 |
| [G-45](SUITE-REVIEW.md#gap-analysis-run-4--2026-04-26-0000z) | Portfolio-arc perspective absent (suite evaluates projects, not the arc between them) | Personal-use gap | Low | High | Addressed | 2026-04-26 | 2026-04-26 |
| [G-46](SUITE-REVIEW.md#gap-analysis-run-5--2026-04-26-0100z) | SO split identity: spec contract mixed with process governance | Structural gap | High | High | Addressed | 2026-04-26 | 2026-04-26 |
| [G-47](SUITE-REVIEW.md#gap-analysis-run-5--2026-04-26-0100z) | Suite described as pre-merge gate, not iterative refinement loop | Structural gap | High | High | Addressed | 2026-04-26 | 2026-04-26 |
| [G-48](SUITE-REVIEW.md#gap-analysis-run-5--2026-04-26-0100z) | QE/SE domain boundary not explicit — overlapping correctness coverage | Structural gap | Medium | Medium | Addressed | 2026-04-26 | 2026-04-26 |
| [G-49](SUITE-REVIEW.md#gap-analysis-run-5--2026-04-26-0100z) | PE posture misrepresented as adversarial review; sycophancy check too generic | Structural gap | Low | Low | Addressed | 2026-04-26 | 2026-04-26 |
| [G-50](SUITE-REVIEW.md#gap-analysis-run-5--2026-04-26-0100z) | No generalist adversary pass (unstructured review complementing specialists) | Missing capability | Low | Medium | Addressed | 2026-04-26 | 2026-04-26 |
| [G-51](SUITE-REVIEW.md#gap-analysis-run-5--2026-04-26-0100z) | No VDD-IAR Alignment domain: process compliance unowned | Missing domain | High | High | Addressed | 2026-04-26 | 2026-04-26 |
| [G-52](SUITE-REVIEW.md#gap-analysis-run-6--2026-04-27) | Test discipline: VDD-IAR Alignment dim 4 treated test-after as yellow flag, not finding; no TDD proxy indicators in QE | Dimension gap | High | High | Addressed | 2026-04-27 | 2026-04-27 |
| [G-53](SUITE-REVIEW.md#gap-analysis-run-7--2026-04-27) | Spec crystallization quality unowned: VSDD Phase 1 behavioral contracts, edge case catalog, verification architecture not evaluated by any domain | Dimension gap | High | High | Addressed | 2026-04-27 | 2026-04-27 |
| [G-54](SUITE-REVIEW.md#gap-analysis-run-7--2026-04-27) | Four-dimensional convergence one-dimensional: exit signal tracks implementation MVR only; spec MVR, test MVR, and verification MVR are untracked | Structural gap | High | Medium | Open | 2026-04-27 | 2026-04-27 |
| [G-55](SUITE-REVIEW.md#gap-analysis-run-7--2026-04-27) | Formal hardening completely unowned: VSDD Phase 5 (proof harnesses, fuzzing, mutation testing, purity boundary audit) has no domain; not even listed as a gap for applicable projects | Missing domain | High | Low | Open | 2026-04-27 | 2026-04-27 |
| [G-56](SUITE-REVIEW.md#gap-analysis-run-8--2026-04-27) | VSDD purity boundary map unowned: no domain enforced pure-core/effectful-shell separation required by VSDD verification architecture | Dimension gap | High | Medium | Addressed | 2026-04-27 | 2026-04-27 |
| [G-57](SUITE-REVIEW.md#gap-analysis-run-8--2026-04-27) | No effectiveness test for domain prompts: suite correctness verified only through application on real projects; no benchmark project with known defects to validate prompts catch what they claim | AI-workflow gap | Medium | Medium | Open | 2026-04-27 | 2026-04-27 |
| [G-58](SUITE-REVIEW.md#gap-analysis-run-9--2026-04-27) | QE: no coverage threshold in base domain — language supplements had thresholds but base domain had none; a JS/TS project with 10% coverage passed QE review | Dimension gap | High | High | Addressed | 2026-04-27 | 2026-04-27 |
| [G-59](SUITE-REVIEW.md#gap-analysis-run-9--2026-04-27) | QE: no mutation testing guidance — 100% line coverage with wrong assertions passes all QE dims | Dimension gap | High | High | Addressed | 2026-04-27 | 2026-04-27 |
| [G-60](SUITE-REVIEW.md#gap-analysis-run-9--2026-04-27) | QE: no flaky test detection — flaky tests named as failure class but not specific failure modes | Dimension gap | Medium | Medium | Addressed | 2026-04-27 | 2026-04-27 |
| [G-61](SUITE-REVIEW.md#gap-analysis-run-9--2026-04-27) | Security: secrets-in-logs not covered — dim 4 only asked about source control; logs/error messages/crash reports unaddressed | Dimension gap | High | High | Addressed | 2026-04-27 | 2026-04-27 |
| [G-62](SUITE-REVIEW.md#gap-analysis-run-9--2026-04-27) | Security: dim 6 (auth/authz) was a single-line placeholder; inadequate for any project with access control | Dimension gap | Critical | Low | Addressed | 2026-04-27 | 2026-04-27 |
| [G-63](SUITE-REVIEW.md#gap-analysis-run-9--2026-04-27) | Security (JS/TS): prototype pollution not covered | Dimension gap | High | Medium | Addressed | 2026-04-27 | 2026-04-27 |
| [G-64](SUITE-REVIEW.md#gap-analysis-run-9--2026-04-27) | Security (JS/TS + PE): dependency confusion attack not named as a supply-chain failure mode | Dimension gap | Medium | Low | Addressed | 2026-04-27 | 2026-04-27 |
| [G-65](SUITE-REVIEW.md#gap-analysis-run-9--2026-04-27) | UX: loading states and async failure recovery entirely absent | Dimension gap | High | High | Addressed | 2026-04-27 | 2026-04-27 |
| [G-66](SUITE-REVIEW.md#gap-analysis-run-9--2026-04-27) | UX: keyboard focus trap not named; dim 3 did not call out this WCAG 2.1 Level A failure | Dimension gap | Medium | Medium | Addressed | 2026-04-27 | 2026-04-27 |
| [G-67](SUITE-REVIEW.md#gap-analysis-run-9--2026-04-27) | UX: destructive action confirmation dim only evaluated quality of existing gates, not absence of required gates | Dimension gap | Medium | Medium | Addressed | 2026-04-27 | 2026-04-27 |
| [G-68](SUITE-REVIEW.md#gap-analysis-run-9--2026-04-27) | SE: flag argument (boolean trap) anti-pattern not named | Dimension gap | High | Medium | Addressed | 2026-04-27 | 2026-04-27 |
| [G-69](SUITE-REVIEW.md#gap-analysis-run-9--2026-04-27) | SE: primitive obsession not named as a type safety failure mode | Dimension gap | Medium | Medium | Addressed | 2026-04-27 | 2026-04-27 |
| [G-70](SUITE-REVIEW.md#gap-analysis-run-9--2026-04-27) | SA: memory leaks and event listener lifecycle absent; production failure not caught by tests | Dimension gap | High | High | Addressed | 2026-04-27 | 2026-04-27 |
| [G-71](SUITE-REVIEW.md#gap-analysis-run-9--2026-04-27) | SA (JS/TS): circular dependency detection absent | Dimension gap | Medium | Medium | Addressed | 2026-04-27 | 2026-04-27 |
| [G-72](SUITE-REVIEW.md#gap-analysis-run-9--2026-04-27) | DE: dim 3 (schema evolution) too thin — one question, no migration testing, no rollback, no forward-compat | Dimension gap | High | Medium | Addressed | 2026-04-27 | 2026-04-27 |
| [G-73](SUITE-REVIEW.md#gap-analysis-run-9--2026-04-27) | DE: data volume limits entirely absent — localStorage quotas, rendering performance, main thread blocking | Dimension gap | Medium | High | Addressed | 2026-04-27 | 2026-04-27 |
| [G-74](SUITE-REVIEW.md#gap-analysis-run-9--2026-04-27) | PE: dim 21 (DR) accepted "documented" as equivalent to "tested"; rollback and backup restoration require test records | Dimension gap | High | Low | Addressed | 2026-04-27 | 2026-04-27 |
| [G-75](SUITE-REVIEW.md#gap-analysis-run-9--2026-04-27) | VDD-IAR Alignment sequencing: run last only; should also gate each layer close, not only final merge | Structural gap | Medium | Medium | Addressed | 2026-04-27 | 2026-04-27 |
| [G-76](SUITE-REVIEW.md#gap-analysis-run-9--2026-04-27) | G-20/21/23 (assumption surfacing, hallucination detection, dependency validation) unaddressed since Run 2; critical for AI-accelerated workflow | AI-workflow gap | High | High | Addressed (partial) | 2026-04-27 | 2026-04-27 |
| [G-77](SUITE-REVIEW.md#gap-analysis-run-9--2026-04-27) | Sycophancy check boilerplate: identical text across all 9 domains reduces salience; domain-specific failure modes not named | Structural gap | Medium | Medium | Addressed | 2026-04-27 | 2026-04-27 |
| [G-78](SUITE-REVIEW.md#gap-analysis-run-9--2026-04-27) | prompts/spec-crystallization.md: driving questions UI-centric; no project type framing for library/CLI/infra/research | Prompt gap | Medium | High | Addressed | 2026-04-27 | 2026-04-27 |
| [G-79](SUITE-REVIEW.md#gap-analysis-run-9--2026-04-27) | prompts/decomposition.md: crosslink conflated with all projects; principle/tool separation absent; crosslink replaces TODO.md not stated | Prompt gap | Medium | Medium | Addressed | 2026-04-27 | 2026-04-27 |
| [G-80](SUITE-REVIEW.md#gap-analysis-run-10--2026-04-27) | Accessibility evaluated only as UX dimensions; no full domain for WCAG depth, screen reader testing, cognitive accessibility, dynamic announcements | Missing domain | High | High | Addressed | 2026-04-27 | 2026-04-27 |
| [G-81](SUITE-REVIEW.md#gap-analysis-run-10--2026-04-27) | No Observability domain for application-layer diagnostics (error classification, structured logging, diagnostic completeness, health surfaces) | Missing domain | High | Medium | Addressed | 2026-04-27 | 2026-04-27 |
| [G-82](SUITE-REVIEW.md#gap-analysis-run-10--2026-04-27) | No Documentation domain: documentation accuracy, knowledge transfer, AI session independence evaluated only as brief SE/SA dimensions | Missing domain | High | High | Addressed | 2026-04-27 | 2026-04-27 |
| [G-83](SUITE-REVIEW.md#gap-analysis-run-10--2026-04-27) | No Localization domain: i18n readiness entirely unowned; locale assumptions baked in by AI agent without surfacing | Missing domain | Medium | High | Addressed | 2026-04-27 | 2026-04-27 |
| [G-84](SUITE-REVIEW.md#review-3--2026-04-27) | Technical Writer lang supplement absent — rustdoc, TypeDoc, JSDoc tooling guidance not in javascript-typescript.md or rust.md | Dimension gap | Medium | Medium | Addressed | 2026-04-27 | 2026-04-28 |
| [G-85](SUITE-REVIEW.md#review-4--2026-04-27-2000z) | Localization lang supplement absent — JS/TS section needed (Intl.* APIs, i18next, react-i18next); Rust section needed (fluent-rs) | Dimension gap | Medium | High | Addressed | 2026-04-27 | 2026-04-28 |
| [G-86](SUITE-REVIEW.md#review-16--2026-05-01-0000z) | No VSDD Phase 4 (Feedback Routing) session primer — developer has no guide for routing IAR findings to correct earlier phase | Missing primer | Medium | Medium | Open | 2026-05-01 | 2026-05-01 |

**Status values:** Open · Addressed · Deferred · Dismissed · Context-Dependent

Run narratives are recorded in `SUITE-REVIEW.md` under `## Gap Analysis Run N` sections.
