# 2026-05-06 Suite Reviews

## Review 36 — 2026-05-06 03:30Z

**Scope:** `prompts/review-session.md` (cold-session primer); `domains/role/PLATFORM-ENGINEER-REVIEW.md` and `domains/role/SECURITY-REVIEW.md` (the two role domains whose typical findings most often surface anonymization / disclosure hazards); a new `hooks/` directory and pre-commit script `hooks/check-review-log-anonymization.sh`; `.pre-commit-config.yaml` at the portfolio repo root (wires the new hook); the existing `issue-tracker-cli/.pre-commit-hooks/check-no-home-paths.sh` (read for parity reference, not modified). Triggered by user feedback after rendering Layer 1 PROCESS.md retrospective for `issue-tracker-cli`: the user described a meta-leak class — the Layer 1 Platform Engineer review log itself cited a concrete deanonymizing example as part of "what not to do," and that example was the very leak the project's pre-commit hooks were defending against. The leak required git history rewrite to scrub. The user asked for suite-level mitigations so the class does not recur for future projects adopting this suite.

**Lens:** Defect-class lens — **adversarial-review-log self-disclosure / meta-leak when worked examples instantiate the defect they document.** The class is broader than anonymization: any review log demonstrating a secret/identity/anonymization defect through a worked example tends to reproduce the defect. A Security review citing a leaked credential as the example reproduces the credential. A Privacy review citing a real personal datum reproduces the disclosure. A Platform Engineer review citing the unscrubbed username reproduces the username. The defect is structural in adversarial review writing — concrete examples are good engineering practice, but for disclosure-class defects the example *is* the disclosure unless abstracted.

**Session note:** Same session as the user's request — explicitly in-session, not cold. Sycophancy compensation: the artifact claim is verifiable independently of session context — the `issue-tracker-cli/iterative-adversarial-refinement/PLATFORM-ENGINEER-REVIEW.md` log incident the user described is reproducible against the project's git history (the `git surgery` the user mentioned is itself evidence the meta-leak occurred); the new `hooks/check-review-log-anonymization.sh` was tested against four cases (public-URL allow, bare-username block, home-path block, noreply-email allow) plus the live baseline of all existing review-log markdown — results are byte-level reproducible, not narrative. A cold-session reviewer applying the same lens would reach the same conclusion against the same artifacts.

---

### Resolved

**Finding 1 — `prompts/review-session.md` did not warn the reviewer that adversarial review logs are publishable artifacts that can reproduce the disclosures they document.**

The cold-session primer set adversarial posture (sycophancy guards, classification rigor, human-verification rule) but did not name confidentiality-aware citation. A reviewer instructed to find real defects, exhibit concrete evidence, and resist sycophancy is also being instructed — implicitly — to write reviews dense with citations. For most defect classes this is correct: file:line references, command transcripts, and observed bytes are exactly what an adversarial reviewer should produce. For disclosure-class defects (anonymization gaps, secrets-management gaps, identity exposure), the same citation discipline reproduces the defect. The primer did not flag this asymmetry.

**Resolution:** Inserted a `## Confidentiality-aware citation` section between the existing posture section and the `## Before starting a domain review` section. The new section: (1) names the class — review logs are publishable artifacts; (2) lists concrete signals that a project is opt-in anonymized (`block local home directory paths` hooks, noreply git config, scrubbed `Cargo.toml`/`package.json` author fields, anonymization CHANGELOG entries); (3) states the principle — "an example illustrating what-not-to-do should never instantiate what-not-to-do"; (4) prescribes abstract placeholders (`<user>`, `<repo>`, `<email>`, `<key>`, `<path>`); (5) preserves shape over content — the finding remains reproducible against project state via the abstracted citation; (6) names the suite-level controls (the hook in Finding 3) but explicitly states they do not substitute for reviewer judgement.

Forward-looking: applies to every domain review run against a project under review. Suite-level reviews (this file) are about artifacts that are themselves explicitly public, so the rule applies less directly here — but in-session continuity with project-under-review context means a careless suite review can still reproduce a project leak. The rule is unconditional.

---

**Finding 2 — Platform Engineer and Security domain prompts did not name the meta-leak class even though their typical findings most often surface it.**

Both `domains/role/PLATFORM-ENGINEER-REVIEW.md` (pre-commit hooks, secrets management, CI configuration) and `domains/role/SECURITY-REVIEW.md` (information exposure, identity disclosure, secrets-management) have evaluation dimensions that are exactly the dimensions whose worked examples tend to disclose. Reviewers in these domains are most likely to produce review logs that meta-leak. Finding 1's primer-level rule applies but is general; the domain-specific reminder anchors the rule in the reviewer's working context.

**Resolution:** Appended a `**Confidentiality-aware citation (Platform-domain reminder).**` paragraph to PLATFORM-ENGINEER-REVIEW.md after the dimensions section, naming the class (hook configs, environment values, anonymization gaps, secrets-management defects) and the abstraction discipline (`<user>`, `<email>`, `<key>` placeholders). Appended a parallel `**Confidentiality-aware citation (Security-domain reminder).**` paragraph to SECURITY-REVIEW.md, named for that domain's typical surface (information-exposure findings, identity-disclosure findings, secrets-management findings). Both reminders cross-reference the primer rule and the suite-level hook.

---

**Finding 3 — The existing source-code anonymization hook (`issue-tracker-cli/.pre-commit-hooks/check-no-home-paths.sh`) was scoped too narrowly: it catches `$HOME` paths but not git-config user.name or user.email, and is wired to scan all text — review-log markdown was caught only by the home-path subset of patterns.**

The original incident the user described occurred specifically because the leaked username appeared in a review log without being inside a `$HOME` path string — the home-path hook would not have caught it. A wider hook scoped to review-log markdown closes the gap.

The hook needed to handle one subtlety: a project may opt in to publishing the git handle as part of a public repository URL (the `Cargo.toml` `repository` field added in `issue-tracker-cli` IAR Round 2 is exactly this — `https://github.com/<user>/guild-portfolio`). The hook must allow public-URL contexts while blocking bare identity citations.

**Resolution:** Created `iterative-adversarial-refinement/hooks/check-review-log-anonymization.sh`. The script reads `git config user.name`, `git config user.email`, and `$HOME` at runtime — no identity values hardcoded. It scans each file argument for these patterns and reports any line-level match that is NOT in a public-URL context. The public-URL allowlist is `github.com/`, `gitlab.com/`, `bitbucket.org/`, `noreply.*` — covering the three major forge URL forms and the noreply email form. Lines with these tokens are exempted; the handle on those lines is intentionally public.

The hook was wired in the portfolio repo's `.pre-commit-config.yaml` as a new `id: review-log-anonymization` entry, scoped via `files:` regex to IAR review-log markdown only (`iterative-adversarial-refinement/.*\.md` and `.*/iterative-adversarial-refinement/.*\.md`). The existing `no-home-dir-paths` hook covers source code; the new hook covers review logs. Coverage is now layered: source code by the project hook, review logs by the suite hook.

Tested against four cases at hook authoring time: (a) public URL line passes (exit 0); (b) bare username citation fails with line number and remediation hint (exit 1); (c) home-path string fails (exit 1); (d) noreply email line passes (exit 0). Live baseline against all existing IAR review-log markdown across the portfolio is clean (one legacy bare-username citation in `issue-tracker-cli/iterative-adversarial-refinement/TECHNICAL-WRITER-REVIEW.md` Review 7 was scrubbed to clear the baseline — the line previously referenced `the `magnificentlycursed/guild-portfolio` GitHub URL`; rewritten to `https://github.com/<user>/guild-portfolio`).

The hook is suite-internal — it lives at `iterative-adversarial-refinement/hooks/` so that a future spinoff of the IAR suite into its own repository carries the hook with it. The portfolio's `.pre-commit-config.yaml` is the wiring point; a project adopting the IAR suite reproduces the wiring in its own `.pre-commit-config.yaml`.

---

### New gap registered

**G-98 — Adversarial review logs can themselves leak the values they document.**

Registered in [GAP-ANALYSIS-LOG.md](../FINDINGS-INDEX.md). Severity: High mission-critical, Medium speculative — review logs in regulated/mission-critical contexts can directly cause compliance failures (HIPAA disclosure logs reproducing PHI; SOC 2 evidence files leaking access keys); for speculative/portfolio contexts the cost is reputational and recovery (git surgery) rather than regulatory. Status: Addressed in-session by the three resolutions above. The fix and the registry entry land in the same session, per the suite-development convention.

**Classification:** New gap, immediately Addressed.

---

### Coordination

The three resolutions operate in defense-in-depth: Finding 1 (primer rule) is the first-pass instruction every reviewer reads; Finding 2 (domain reminders) re-anchors the rule in the contexts where the meta-leak is most likely; Finding 3 (hook) is the build-time backstop that catches what reviewer judgement missed. None substitutes for the others — a primer rule without enforcement leaks (the original incident); enforcement without instruction creates false-positive friction; instruction without domain anchoring drifts as the reviewer specializes. All three were authored as a coordinated bundle for this gap; future projects inheriting the suite inherit the bundle.

The hook's public-URL allowlist (`github.com/`, `gitlab.com/`, `bitbucket.org/`, `noreply.*`) is the deliberate seam between "identity is the leak" and "identity is the public URL." Projects using a different forge (Codeberg, Gitea, internal GitLab) will need to extend the allowlist; the script's `public_url_allowlist` variable is the single point. This is a scope-narrowing choice: the suite ships with the three majority forges supported, and project-specific tuning lives in the project's fork of the hook (or, preferably, in the suite if the project's forge gains broad use).

Cross-references with project-level reviews: the underlying incident is described in the user's `issue-tracker-cli/PROCESS.md` Layer 1 retrospective ("What was hardest"); the artifact resolution is recorded in this suite review. No project-level review log update is required — the original Platform Engineer review entry that contained the leak has already been scrubbed via the user's prior git surgery; the controls added in this session are forward-looking.
