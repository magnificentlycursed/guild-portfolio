# Observability Review

This review is part of the [Iterative Adversarial Refinement (IAR)](../../README.md) suite. It may be run independently or alongside other domains. See [README.md](../../README.md) for sequencing, scoped runs, and domain coordination.

The purpose of this review is to evaluate whether the application is *diagnosable* — whether a production failure can be understood from the signals the application emits. Platform Engineering owns the infrastructure that collects and routes logs, metrics, and traces. This domain owns the application-layer instrumentation: whether the code emits useful signals, whether errors surface with enough context to be actionable, and whether the system's state is observable from the outside without access to source code or a debugger.

A functionally correct application that swallows errors, logs only happy-path events, and provides no health signal is unobservable. When it fails in production, it will fail silently.

This domain is most relevant to deployed services, server-rendered applications, and long-running browser applications. It may apply lightly to simple local tools with no deployment context.

## Current Review Prompt

**Scope:** All error handling paths, all log emission points, all external interfaces, and any health or status surfaces.

Read DESIGN.md for stated operational requirements. Then read all source files with focus on error handling, logging calls, and any diagnostic or health endpoints.

For each finding, cite file and line number. Classify as **resolved** (fix applied this review), **deferred** (scheduled for a specific layer, reason given), **dismissed** (no action taken, rationale required), or **hallucinated** (the adversary invented a problem that does not exist — push back is warranted. Consistent hallucinated findings are the maximum viable refinement signal).

**Coordination:** Flag findings that overlap with Security (sensitive data in logs — see Security dim 4), PE (log routing, metrics infrastructure, alerting), SA (architectural decisions that make state unobservable), and SE (error handling quality).

**Sycophancy check:** An agent writes code to make features work; it does not write code to make failures diagnosable. Observability is almost never in the spec and almost always omitted from AI-generated code. The adversary should assume that every error path has insufficient logging until proven otherwise. A codebase where all `catch` blocks are `console.error(e)` is not instrumented — it is logged.

**Language and interface supplement:** Consult `../../lang/` for language-specific logging libraries, structured logging patterns, and error type conventions.

## Standard Evaluation Dimensions

1. **Error surfacing** — Are errors caught at appropriate levels and surfaced with enough context to diagnose the root cause? Named failure modes: empty `catch` blocks that swallow errors silently; `catch (e) { console.log("error") }` that logs a label without the error; errors caught and rethrown without adding context; errors that manifest as incorrect behavior rather than an observable failure signal. Every `catch` block that does not re-throw should emit a diagnostic event that identifies what failed and why.

2. **Error classification** — Are errors distinguished by type? Named categories: user errors (invalid input — expected, not alarming), application errors (bugs — unexpected, should alert), and dependency errors (external service unavailable — expected under failure conditions, different response required). An application that treats all errors the same will alarm on user errors (noise) and fail to alarm on application errors (missed signal).

3. **Structured log emission** — Are log entries structured enough to be queryable and filterable? A `console.log("Bookmark saved")` tells you something happened; a structured entry with `{ event: "bookmark_saved", id: bookmark.id, timestamp: ... }` tells you what, when, and to what. For applications with a meaningful operational context, human-readable string logs are insufficient — they cannot be aggregated, filtered, or alerted on.

4. **Diagnostic completeness** — Pick a plausible production bug (a save fails, a search returns wrong results, a form submits but nothing happens). Could you diagnose the root cause from the application's log output alone, without access to source code or a debugger? If the answer is no, identify what is missing.

5. **Health and status surfaces** — For deployed services: is there a health check endpoint or status signal that infrastructure can poll to determine whether the application is functional? A health check that returns `200 OK` regardless of application state (e.g., database connection broken, external dependency unavailable) is worse than no health check — it masks failures from load balancers and orchestration platforms. Health checks must verify the application's ability to serve requests, not just that the process is alive.

6. **Correlation and request tracing** — For applications that handle multiple concurrent requests or have multi-step operations: can a single user action or request be traced through the application's log output? Named pattern: a correlation ID or request ID that is attached to every log entry for a given operation, allowing all events from a single user interaction to be filtered together.

7. **Sensitive data exclusion** — Do log entries, error messages, and diagnostic output avoid including personal data, credentials, authentication tokens, or other sensitive information? (See also Security dim 4.) A well-structured log is a high-value target if it contains PII or secrets — the same observability that helps diagnose failures creates exposure if it captures sensitive data.

8. **Local diagnostic parity** — Can a developer reproduce the same observability locally that exists in production? If production logs are structured JSON and local development logs are unformatted console output, a developer debugging a locally reproduced issue cannot work in the same mental model as a production post-mortem. Development and production observability should be structurally identical, even if the destination differs.

9. **Silent success confirmation** — For operations that modify state (saves, deletes, updates), is there a positive confirmation that the operation completed — not just that it did not fail? An operation that succeeds without emitting any signal is invisible to monitoring. You cannot alert on the absence of a success event unless the success event exists.

10. **Operational runbook coverage** — Can the application's observable signals support the operational runbook? If the runbook says "check the logs for error code X," does the application actually emit that signal? Runbooks and observability must be designed together — a runbook that describes signals that don't exist is fiction.

---

Review entries are logged in `iterative-adversarial-refinement/OBSERVABILITY-REVIEW.md` inside the project being reviewed.
