# Red Team Review

This review is part of the [Iterative Adversarial Refinement (IAR)](../../README.md) suite. It may be run independently or alongside other domains. See [README.md](../../README.md) for sequencing, scoped runs, and domain coordination.

**Reviewer role: Red Team Hacker** (Penetration Tester / Offensive Security Engineer)

The Security Engineer review asks whether defenses exist. This review asks whether they can be bypassed. The reviewer is an attacker — actively trying to exploit the application, chain low-severity findings into high-impact compromises, and abuse features in ways the developer did not intend. Assume every control has a bypass until you demonstrate otherwise. A defense that has never been tested under adversarial conditions is a defense that has never been tested.

This domain is most relevant to any application with authentication, user-controlled input, network exposure, or data belonging to users other than the developer. For a purely local single-user tool with no network surface and no authentication, scope down significantly.

**Sequencing:** Run after Security Engineer. Security Engineer ensures controls exist; Red Team verifies they hold under attack. A Red Team finding that traces to an absent control is also a Security Engineer finding — flag both.

## Current Review Prompt

**Scope:** Whole application — all entry points, all trust boundaries, all data flows. If a scope is provided, concentrate attack simulation there, but do not ignore adjacent surfaces that could be used to pivot.

Read DESIGN.md first to understand what the application is supposed to do and what it considers out of scope. Then read all source files, configuration, and dependency manifests. The most valuable findings are often in the gap between what the developer intended and what the application actually does.

For each finding, cite file and line number. Classify as **resolved** (fix applied this review), **accepted risk** (no fix, explicit rationale and risk owner required), **dismissed** (no action taken, rationale required), or **hallucinated** (the adversary invented an attack that does not work — demonstrate why the control holds. Consistent hallucinated findings are the maximum viable refinement signal).

Regression check: verify that previously-confirmed attack mitigations remain intact. Implementation changes can silently reopen entry points, weaken input validation, or remove controls that a prior Red Team pass confirmed as adequately defended. Every change to input handling, rendering, storage, or authentication logic is in scope for this check regardless of stated scope.

**Coordination:** Flag findings to [SECURITY-REVIEW.md](SECURITY-REVIEW.md) when a Red Team finding reveals an absent control the Security review should have caught. Flag to [QUALITY-ENGINEER-REVIEW.md](QUALITY-ENGINEER-REVIEW.md) when an exploitable path has no test coverage. Flag to [SOLUTION-ARCHITECT-REVIEW.md](SOLUTION-ARCHITECT-REVIEW.md) when an attack succeeds because of an architectural decision.

**DESIGN.md change authority:** If a finding requires a change to `DESIGN.md`, classify it "Raised to SO" and document the proposed change and rationale. Do not apply the change. `DESIGN.md` is a controlled spec document — the Solution Owner is the sole domain authorized to modify it.

**Sycophancy check:** An agent that built the application will rationalize its defenses as adequate because it believes in the controls it generated. The Red Team does not evaluate intent — it evaluates outcome. For every control, ask: "can this be bypassed by a caller who does not follow the happy path?" An application where every attack is dismissed as "not applicable" has not been red-teamed — it has been reassured.

**Language and interface supplement:** Consult `../../supplements/` for the supplement matching the project's primary language and interface type. Apply the **Red Team** section — attack tooling, injection patterns, and client-side exploit vectors are language- and framework-specific.

## Standard Evaluation Dimensions

1. **Threat model** — Who are the plausible attackers? What are they after? What is the crown jewel — the data or capability that, if compromised, would be the worst outcome? Name the threat actors (anonymous internet user, authenticated user, disgruntled insider, automated scanner) and identify which attack surfaces are in scope for each. A review without a threat model is a checklist, not a red team.

2. **Attack surface enumeration** — List every entry point: HTTP endpoints, form inputs, URL parameters, file uploads, WebSocket messages, localStorage reads, postMessage handlers, CLI arguments, environment variables read at runtime, third-party script injection points. Every entry point is a potential attack surface. Flag any entry point that is not explicitly validated.

3. **Authentication bypass** — Can you access protected resources or actions without valid credentials? Named attacks: JWT algorithm confusion (accepting `alg: none`), JWT secret brute-force (weak secrets), session fixation (server accepting a session ID set by the client), session token in URL (leaks in logs and Referer headers), account enumeration via differential error messages or timing, password reset flow abuse (token reuse, predictable token, token not expiring), OAuth redirect URI manipulation.

4. **Authorization flaws and privilege escalation** — Can an authenticated user access resources or perform actions belonging to another user or a higher privilege level? Named attacks: insecure direct object reference (IDOR — changing an ID in a request to access another user's data), horizontal privilege escalation (accessing another user's data at the same privilege level), vertical privilege escalation (modifying a role parameter in a request or JWT to gain admin access), mass assignment (sending extra fields in a request that get applied to a sensitive model attribute).

5. **Business logic abuse** — Can the application's intended features be weaponized? Named attacks: race conditions (concurrent requests to exploit a check-then-act window), negative or boundary values (negative quantities, zero-price purchases, integer overflow), workflow sequence violations (skipping steps in a multi-step process, replaying a completed step), feature interaction abuse (using one feature to bypass a restriction in another), time-of-check to time-of-use (TOCTOU) gaps.

6. **Injection and server-side attack chains** — Can user input cause unintended execution or data access? Named attacks: SQL injection, NoSQL injection, command injection, LDAP injection, XPath injection, template injection (SSTI), path traversal (`../../etc/passwd`), open redirect, SSRF (server-side request forgery — tricking the server into making requests to internal services). For each injection class: does input ever reach an interpreter, filesystem, or network call without being parameterized or strictly validated?

7. **Client-side attack chains** — Can an attacker deliver a payload to another user via the application? Named attacks: stored XSS (payload persisted and served to other users), reflected XSS (payload in URL reflected without encoding), DOM XSS (payload processed by JavaScript without sanitization), CSRF (forging a state-changing request from another origin), clickjacking (framing the application to steal clicks), prototype pollution (injecting properties via `__proto__` that affect application behavior), postMessage origin confusion (accepting messages from untrusted origins).

8. **Information leakage for reconnaissance** — What does the application reveal that helps an attacker plan? Named leakage patterns: verbose error messages exposing stack traces, file paths, or database schemas; differential error messages that confirm valid usernames or record existence; HTTP response headers revealing server version, framework, or internal hostnames; comments or dead code in client-side bundles containing credentials, internal URLs, or implementation details; timing differences that reveal valid vs. invalid inputs.

9. **Chained vulnerabilities** — Do individually low-severity findings combine into a high-severity attack chain? Named patterns: an information-leakage finding that enables a targeted brute-force; a low-privilege IDOR that exposes a token needed for a higher-privilege action; a reflected XSS that, combined with CSRF, enables account takeover. Evaluate the combined risk of your finding set, not just each finding in isolation.

10. **Insider threat and legitimate user abuse** — What can an authenticated user do that they should not be able to? Named concerns: a standard user who can enumerate all other users' records; a user who can exhaust shared resources (storage, API rate limits) affecting other users; a user who can inject content visible to other users (stored XSS, content injection); a user who can trigger server-side behavior at scale (DoS via expensive operations with no rate limiting or resource cap).

11. **Automated attack resilience** — Does the application resist automated attacks? Named failure modes: no rate limiting on authentication endpoints (brute-force); no account lockout or progressive delay after repeated failures; no CAPTCHA or proof-of-work on high-value actions; no detection or blocking of credential stuffing patterns; API responses that return full record sets with no pagination (enables bulk data extraction in a single request).

12. **Supply chain and dependency exploitation** — Can a third-party dependency be used as an attack vector against this application or its users? Named attacks: a dependency with a known RCE or XSS CVE; a dependency that loads remote content at runtime (CDN scripts without SRI); prototype pollution via a utility library; a typosquatting or dependency confusion attack against internal package names. Cross-reference with Security dim 3 — the Red Team looks for how a vulnerable dependency would actually be exploited in this specific application's context.

---

Review entries are logged in `iterative-adversarial-refinement/RED-TEAM-REVIEW.md` inside the project being reviewed.
