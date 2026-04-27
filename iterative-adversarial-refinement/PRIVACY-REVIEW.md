# Privacy Review

This review is part of the [Iterative Adversarial Refinement (IAR)](README.md) suite. It may be run independently or alongside other domains. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

The purpose of this review is to evaluate privacy as a distinct concern from security. Security asks whether data can be exfiltrated. Privacy asks whether the data should have been collected in the first place, how long it is retained, who can access it, whether the basis for processing is lawful, and whether users can exercise their rights. A system with no security vulnerabilities can still be a privacy failure.

This domain is most relevant to applications that collect, process, or store information about identifiable individuals. For single-user local tools that store only the user's own data and do not transmit it, this domain may apply lightly — evaluate using the "solo personal tool" notes in each dimension. For any application with users other than the developer, full evaluation is required.

## Current Review Prompt

**Scope:** Whole application. Read DESIGN.md for stated data handling policies. Then audit all data collection points, storage locations, transmission paths, and third-party integrations.

For each finding, cite the specific data element, file, and line number. Classify as **resolved** (fix applied this review), **deferred** (scheduled for a specific layer, reason given), **dismissed** (no action taken, rationale required — note: "we don't have users yet" is not rationale; privacy debts compound), **accepted risk** (deliberate decision with explicit documented rationale and owner), or **hallucinated** (the adversary invented a problem that does not exist — push back is warranted. Consistent hallucinated findings are the maximum viable refinement signal).

**Coordination:** Flag findings that overlap with Security (PII in logs or error output), DE (retention enforcement at the data layer), PE (pre-commit hooks for PII detection), and SA (architectural decisions that create unnecessary data collection).

**Sycophancy check:** Privacy is the dimension most often dismissed as "not applicable yet" or "we'll handle this when we have users." An agent will not proactively flag privacy concerns — it will implement what was asked without considering whether what was asked is appropriate to collect. The adversary must apply privacy pressure before data collection exists in the codebase, not after. A privacy debt incurred at implementation is far more expensive to resolve after data exists than before.

**Language and interface supplement:** No supplement required — privacy is language-agnostic. The data collected and its handling are the artifact; the language is incidental.

## Standard Evaluation Dimensions

1. **Data inventory** — What personally identifiable or potentially-identifying information does the application collect, store, process, or transmit? Build an explicit list: direct identifiers (name, email, phone, ID), indirect identifiers (IP address, device fingerprint, behavioral patterns, timestamps with fine granularity), sensitive categories (health, financial, location, biometric, political, religious). If the data inventory cannot be derived from the code and DESIGN.md, that is itself a finding — untracked data is unmanaged data.

2. **Necessity and data minimization** — For each item in the data inventory: is this data necessary to provide the stated functionality? Could the feature work with less granular data, anonymous data, or no data at all? The burden of justification is on collection, not on omission. An AI agent will collect whatever data is convenient to implement a feature; the adversary asks whether each piece is justified by a specific functional need.

3. **Legal basis for processing** — For applications with users: on what legal basis is each category of personal data processed? Named bases: explicit consent, legitimate interest, contract performance, legal obligation. If the basis is consent: is it obtained before collection, specific to each purpose, and freely withdrawable? If legitimate interest: has a balancing test been documented?

4. **Retention policy** — How long is each category of data retained? Is retention enforced in the code (automatic deletion or expiration) or only documented in policy? Data that is retained indefinitely because "deletion was never built" is a privacy failure. Named check: does the application accumulate data in storage that is never explicitly removed?

5. **User rights** — Can a user request: access to their data (what is stored), deletion of their data (the right to erasure), portability of their data (export in a usable format)? Are these implementable from the current data model? If the data is stored in a way that makes it impossible to retrieve or delete a specific user's data, the architecture is a privacy failure regardless of stated policy.

6. **Third-party data sharing** — Does the application transmit personal data to third parties (analytics services, CDNs, error monitoring, payment processors, AI APIs)? For each: what data is shared, under what terms, and does the user know? Third-party scripts that run in the browser (analytics, tag managers, chat widgets) may collect data independently — is this accounted for?

7. **Consent mechanism quality** — If consent is the legal basis for processing: is it obtained before data collection begins? Is it specific (one consent per purpose, not bundled)? Is it freely given (not required to access the service unless the processing is necessary for the service)? Is withdrawal as easy as giving consent? Are consent records stored and auditable?

8. **PII in non-primary storage** — Does personally identifiable information appear in: application logs, error messages, crash reports, analytics events, URL parameters, browser history, debug output? These are secondary storage paths that may not be subject to the same retention and access controls as the primary data store.

9. **Sensitive data categories** — Does the application handle health information, financial information, location data, biometric data, data about minors, or other sensitive categories? These categories carry heightened obligations in most jurisdictions and require additional safeguards beyond standard PII handling.

10. **Privacy by design** — Was privacy considered at design time, or is it being evaluated for the first time now? Named indicators of privacy-by-design: DESIGN.md contains a data handling section; the data model is minimal rather than maximally expressive; collection is opt-in rather than opt-out; the default state collects nothing rather than everything. An application where privacy was not designed in will require structural changes to achieve compliance — the earlier this is evaluated, the cheaper the fix.

---

Review entries are logged in `iterative-adversarial-refinement/PRIVACY-REVIEW.md` inside the project being reviewed.
