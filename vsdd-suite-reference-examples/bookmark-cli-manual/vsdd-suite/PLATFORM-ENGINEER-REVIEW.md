# Platform Engineer Review Log (Index)

This review log is part of the [VSDD Suite](../../../vsdd-suite/README.md). The Phase 3 adversarial review for this domain runs as one cold-context session per round; this file indexes the rounds. Round narratives live in [`review-log/`](review-log/) — see the Reviews table below.

**Reviewer role: Platform Engineer** (Platform Engineer / DevOps Engineer / Release Engineer)

Evaluates build/install/distribution discipline: `Cargo.lock` commitment, toolchain pinning, CI compatibility, dependency audit, fresh-system install verification (G-155 dim 38 — capstone-required).

**Activation:** G-178 strong-presumption activation; capstone-required for the fresh-system install verification per G-155 dim 38.

**Language supplement applied:** `../../../vsdd-suite/supplements/rust.md` § Platform Engineering + `../../../vsdd-suite/supplements/cli.md` § Platform Engineering.

**Sycophancy check:** An agent reviewing its own build/install setup will validate decisions made at scaffolding time. The adversary must test against the actual install lifecycle (uninstall → reinstall → verify data survives) rather than reasoning about it.

Read individual rounds via the links in the Reviews table below. This file is the index only; round narratives live in `review-log/YYYY-MM-DD-<slug>.md` per the project-level review log governing standard in [`../../../vsdd-suite/suite-development/suite-development.md`](../../../vsdd-suite/suite-development/suite-development.md) § Structure.

---

## Reviews

| Review | Date | File | Scope summary |
|---|---|---|---|
| *(no rounds filed yet — populated when the corresponding cold-session review lands in PR 7)* | | | |
