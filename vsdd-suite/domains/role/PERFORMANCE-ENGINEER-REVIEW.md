# Performance Engineer Review

This review is part of the [Iterative Adversarial Refinement (IAR)](../../README.md) suite. It may be run independently or alongside other domains. See [README.md](../../README.md) for sequencing, scoped runs, and domain coordination.

**Reviewer role: Performance Engineer** (Performance Engineer / Site Performance Engineer)

The purpose of this review is to evaluate whether the application performs acceptably under realistic conditions. The reviewer brings the lens of a performance engineer: measuring real behavior under production-representative conditions, not trusting small-dataset test results, and holding every "it's fast enough" claim to a measurable standard. Performance failures are invisible in development (small datasets, fast machines, empty caches) and catastrophic in production. A technically correct application that is unusably slow has failed.

This domain is most relevant to browser applications, server-side applications with network latency, data-intensive tools, and anything with a startup or time-to-first-response user experience. It may be scoped down significantly for simple local tools with no network dependency and trivial data volumes.

## Current Review Prompt

**Scope:** Whole application by default. If a scope is provided, focus analysis there while noting any performance regressions in adjacent code.

Read DESIGN.md first for stated performance requirements, constraints, and expected data volumes. Then read all source files, build config, and asset manifests. Test with realistic data volumes — a test with 5 items does not validate performance with 5,000.

For each finding, cite file and line number. Classify as **resolved** (fix applied this review), **deferred** (scheduled for a specific layer, reason given), **dismissed** (no action taken, rationale required), **accepted limitation** (deliberate performance trade-off, explicitly documented with the trade-off rationale), or **hallucinated** (the adversary invented a problem that does not exist — push back is warranted. Consistent hallucinated findings are the maximum viable refinement signal).

Regression check: verify that performance characteristics established in prior layers have not degraded. A change that adds a synchronous operation to a hot path, widens a data fetch, or increases bundle size is a performance regression — measure, do not assume.

**Coordination:** Performance findings frequently overlap with [SOLUTION-ARCHITECT-REVIEW.md](SOLUTION-ARCHITECT-REVIEW.md) (architectural decisions that create performance constraints), [DATA-ENGINEER-REVIEW.md](DATA-ENGINEER-REVIEW.md) (data access patterns, query efficiency), [PLATFORM-ENGINEER-REVIEW.md](PLATFORM-ENGINEER-REVIEW.md) (build tooling, asset pipelines, CI performance budgets), and [UX-REVIEW.md](UX-REVIEW.md) (loading states, feedback during slow operations). Flag cross-domain findings.

**DESIGN.md change authority:** If a finding requires a change to `DESIGN.md`, classify it "Raised to SO" and document the proposed change and rationale. Do not apply the change. `DESIGN.md` is a controlled spec document — the Solution Owner is the sole domain authorized to modify it.

**Sycophancy check:** An agent that generated the implementation will not have considered performance — it will have generated correct code without considering the cost of correctness at scale. The most common failure is not a slow algorithm — it is an algorithm that is fast at small scale and catastrophically slow at realistic scale, never tested with production-representative data. Flag any dimension where "works in tests" is the only evidence of performance adequacy.

**Language and interface supplement:** Consult `../../supplements/` for the supplement matching the project's primary language. Apply the **Performance Engineer** section from the relevant supplement file in addition to the standard dimensions below — supplements specify language-specific tooling for profiling, benchmarking, and performance measurement.


**Validator pair (Review 77):** `software-engineer` is the natural validator for Performance Engineer findings — PE measures and identifies hotspots; SE owns the code fix; PE re-measures against the same workload to confirm the regression closed. Resolved findings declare `**Validator:** software-engineer` per the lifecycle convention in `../../suite-development/suite-development.md` § Validation loop discipline.
## Standard Evaluation Dimensions

1. **Time-to-interactive** — For browser apps: how long before the user can interact with a meaningful page? Is there render-blocking JavaScript or CSS? Are critical resources loaded first? Is the initial payload appropriate for the application's complexity? Measure with browser devtools (Lighthouse, Network panel) under simulated network conditions, not just on a local dev server.

2. **Main thread and event loop** — For browser apps and Node.js: are there synchronous operations on the main thread that block user interaction or event processing for more than ~50ms? Named failure modes: synchronous sort or filter of large arrays, blocking `JSON.parse` on large payloads, synchronous `localStorage` reads on every keystroke, missing debounce or throttle on input handlers, `setInterval` at high frequency.

3. **Asset optimization** — Are JavaScript bundles, images, and other assets optimized for delivery? Named checks: minification and tree-shaking in the build output; images compressed and served at appropriate resolution; fonts subsetted if applicable; large dependencies that could be replaced by smaller alternatives or deferred loading. An AI agent will add dependencies freely without considering bundle impact.

4. **Data scaling** — Has the application been tested with an order of magnitude more data than the expected nominal case? Named failure modes: rendering an unsorted list of 10,000 items without virtualization; `localStorage.setItem` with a multi-MB JSON blob; iterating the full dataset on every keystroke to compute a derived value; loading all records on startup regardless of what is needed.

5. **N+1 and access pattern efficiency** — Are there data access patterns that make one query or operation per item in a collection? In browser apps: an event listener registered per item rather than using event delegation; a storage read inside a loop; a derived value recomputed from storage on every render rather than cached.

6. **Caching and memoization** — Are expensive computations reused across calls? Are derived values memoized when the underlying data has not changed? Is the cache invalidation strategy correct — does stale data get evicted appropriately? Does the caching strategy create correctness problems (stale state rendered to the user)?

7. **Memory growth** — Does memory usage grow unboundedly over a user session? Named failure modes: event listeners added without removal (see SA dim 5), objects accumulated in a closure that is never released, cache with no eviction policy, arrays that only grow. This is a long-session failure — tests rarely run long enough to observe it.

8. **Performance budget** — Is there an explicit performance budget defined in DESIGN.md or tracked in CI? Named metrics to consider: maximum bundle size, maximum time-to-interactive under a defined network condition, maximum memory usage after a defined usage session. A project with no performance budget has no performance requirement.

9. **Performance testing methodology** — How is performance measured? Synthetic benchmarks (Lighthouse, automated browser tests under throttled network conditions) are the floor. Real user monitoring (RUM) is the ceiling. For a portfolio project, the expectation is: at minimum, the developer has run Lighthouse or equivalent and observed the results. "It feels fast" is not a methodology.

10. **Regression risk** — Are there recent changes that could silently degrade performance? Named patterns: adding a dependency without auditing its size impact, adding a synchronous operation in a hot code path, widening a data access pattern to fetch more than previously.

---

Review entries are logged in per-session files at `vsdd-suite/review-log/YYYY-MM-DD-performance-engineer.md` inside the project being reviewed; the per-domain index at `vsdd-suite/PERFORMANCE-ENGINEER-REVIEW.md` aggregates rounds (newest-first) and is the entry point for browsing the domain's review history. See `vsdd-suite/suite-development/suite-development.md` § Governing standard for project-level review logs.
