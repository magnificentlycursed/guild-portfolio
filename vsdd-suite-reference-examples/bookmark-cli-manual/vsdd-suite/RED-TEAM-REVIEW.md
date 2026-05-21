# Red Team Review Log (Index)

This review log is part of the [VSDD Suite](../../../vsdd-suite/README.md). The [Phase 3](../../../vsdd-suite/primers/3-review-session.md) adversarial review for this domain runs as one cold-context session per round; this file indexes the rounds. Round narratives live in [`review-log/`](review-log/) — see the Reviews table below.

**Reviewer role: [Red Team](../../../vsdd-suite/domains/role/RED-TEAM-REVIEW.md)** (Red Team / Offensive [Security](../../../vsdd-suite/domains/role/SECURITY-REVIEW.md) / Penetration Tester)

Adversarial pair to Security. Challenges Security's threat model from the outside; enumerates attack surfaces that the inside-the-model perspective normalized as acceptable.

**Activation:** Capstone intent activates Red Team per the extended-pool activation criteria — capstone-tier adversarial intensity for any shipped software warrants Red Team's involvement.

**Language supplement applied:** `../../../vsdd-suite/supplements/rust.md` § Red Team + `../../../vsdd-suite/supplements/cli.md` § Red Team.

**Sycophancy check:** Red Team is the most-easily-domesticated domain. The adversary must explicitly attempt the named attacks (input injection, path traversal, panic-as-DoS, supply-chain typosquatting) rather than reasoning about whether they apply.

Read individual rounds via the links in the Reviews table below. This file is the index only; round narratives live in `review-log/YYYY-MM-DD-<slug>.md` per the project-level review log governing standard in [`../../../vsdd-suite/suite-development/suite-development.md`](../../../vsdd-suite/suite-development/suite-development.md) § Structure.

---

## Reviews

| Review | Date | File | Scope summary |
|---|---|---|---|
| [Review 1](review-log/2026-05-20-red-team.md#review-1--2026-05-20-1930z) | 2026-05-20 19:30Z | `review-log/2026-05-20-red-team.md` | Phase 3 IAR Round 1 — 11 Findings (3 Accepted risk + 3 Raised to SO + 1 Dismissed + 4 Hallucinated). Raised-to-SO cluster: terminal-escape injection via stored URLs + file mode 0644 ambient-readable + symlink-follow on `fs::write`/`fs::read_to_string`. Independent of Security pass (the parallel Security round wasn't available at session-start); cross-validation in Round 2. |
