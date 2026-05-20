# Performance Engineer Review Log (Index)

This review log is part of the [VSDD Suite](../../../vsdd-suite/README.md). The Phase 3 adversarial review for this domain runs as one cold-context session per round; this file indexes the rounds. Round narratives live in [`review-log/`](review-log/) — see the Reviews table below.

**Reviewer role: Performance Engineer** (Performance Engineer / SRE / Site Reliability Engineer)

Evaluates the performance characteristics of the implementation: hotspots, allocation patterns, async/sync boundary, benchmarking discipline. Capstone-tier evaluation of whether performance claims have evidence.

**Activation:** Capstone intent activates Performance Engineer per `../../../vsdd-suite/domains/DOMAIN-INDEX.md` § Intent calibration.

**Language supplement applied:** `../../../vsdd-suite/supplements/rust.md` § Performance Engineer + `../../../vsdd-suite/supplements/cli.md` § Performance Engineer.

**Sycophancy check:** An agent reviewing its own implementation will rationalize performance choices as 'good enough' for the project's scale. The adversary must apply named measurement discipline (`cargo bench`, `cargo flamegraph`) rather than reasoning about performance from inspection.

Read individual rounds via the links in the Reviews table below. This file is the index only; round narratives live in `review-log/YYYY-MM-DD-<slug>.md` per the project-level review log governing standard in [`../../../vsdd-suite/suite-development/suite-development.md`](../../../vsdd-suite/suite-development/suite-development.md) § Structure.

---

## Reviews

| Review | Date | File | Scope summary |
|---|---|---|---|
| *(no rounds filed yet — populated when the corresponding cold-session review lands in PR 7)* | | | |
