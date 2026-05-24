# Rust Language Supplement

These dimensions supplement the standard IAR domain reviews for Rust projects. During each domain review, apply the relevant section below in addition to the standard dimensions for that domain.

**Source note:** Rust-specific tooling requirements (coverage thresholds, clippy deny set, cargo-deny configuration, cargo-vet) were sourced from the guild's CLAUDE.md gist, which may be superseded — verify against current `apprentice-onboarding` content for authoritative thresholds before applying.

---

## Quality Engineering

- **Test structure** — Are unit tests colocated with the code they test (`#[cfg(test)]` modules), and integration tests in `tests/`? Are doc tests present for public API functions?
- **Integration tests invoke the binary** — For CLI projects, do integration tests invoke the compiled binary (not internal functions) and assert on stdout, stderr, and exit code?
- **Clippy compliance** — Does `cargo clippy` pass without warnings? Are any `#[allow(...)]` suppressions justified with a comment?
- **Error path coverage** — Are `Err` branches, `None` arms, and panic conditions exercised in tests, not just the happy path?
- **Doc tests compile and pass** — Are doc examples in `///` comments syntactically correct and tested via `cargo test`?
- **Coverage thresholds** — Line coverage should be at minimum 80%. Public API coverage should be 100% — every exported function, type, and trait impl must have at least one test exercising it. Coverage below these thresholds is a finding.
- **Mutation testing with `cargo-mutants`** — For pure functions, validation logic, and any path where an off-by-one or wrong-comparison would be invisible to users but catastrophic, run `cargo mutants` and confirm the surviving-mutations count is zero on the in-scope modules. Install: `cargo install cargo-mutants`. Common invocation: `cargo mutants --in-place --no-shuffle --file src/<module>.rs` to scope to a specific module; `cargo mutants` for the whole crate (slow on large codebases — scope at first, expand once the scoped surface is clean). A surviving mutation means a test passes against a deliberately-wrong implementation — that's a defect in the test surface, not the implementation. Coordinate with QE Dim 2 (Test falsifiability) — the dim names the recurring mutation classes (off-by-one in truncation, sort-direction reversal, `&&` ↔ `||` flips, loop-exit polarity, comparison-operator polarity); `cargo-mutants` is the measurement tool. A `cargo mutants` clean run is the only objective signal that tests resist the named mutation classes; reasoning about test-falsifiability without running the tool is speculation. Surviving mutations are findings whose fix is either (a) a new test asserting the boundary the mutation crossed, or (b) a tightened assertion (substring `contains` → exact-match `assert_eq!`) on an existing test.

## Security

- **File path validation** — If the application reads from or writes to user-supplied paths, are traversal attacks (`../`) and absolute path escapes validated before I/O?
- **`.unwrap()` discipline** — Is `.unwrap()` on `Result` or `Option` used on paths where panicking is acceptable (test code, programmer errors on provably-safe values)? Is it absent from user-facing paths where the input is untrusted?
- **Dependency audit** — Is `cargo audit` run against `Cargo.lock` to detect known CVEs in dependencies? Are any `RUSTSEC` advisories acknowledged?
- **cargo-deny** — Is `cargo deny check` configured with a `deny.toml`? A complete `deny.toml` configures four sections: `[advisories]` (CVE policy), `[licenses]` (allowed/denied SPDX identifiers), `[bans]` (duplicate version policy, denied crates), and `[sources]` (allowed registries and git sources). Missing or incomplete `deny.toml` is a finding.
- **cargo-vet** — For projects with supply-chain risk, is `cargo vet` configured to require audit records for dependencies? For personal portfolio projects this may be deferred; for any project with production users or sensitive data it is a finding if absent.
- **Unsafe usage** — Is `unsafe` code present? Is each block justified with a comment explaining why safety invariants hold? Is there a test or formal argument for each unsafe block?
- **Secrets in code** — Are API keys, tokens, or credentials hardcoded? Are they read from environment variables or a secrets file excluded from version control?
- **Error-message escape via `display_safe` (G-125 / Security Dim 9)** — For every error-emit site (`panic!`, `eprintln!`, `format!` into an `Err`, `?`-propagation paths whose `Error::Display` interpolates input), confirm a sanitizer wraps every user-derived value before it reaches the error stream. The sanitizer must escape `is_control()` chars (`Cc`) AND `Cf` format chars (Trojan-Source U+202E and zero-width characters) while preserving structurally-significant whitespace (`\n` for multi-line errors stays; `\r`/`\t`/`\x07` get escaped to printable forms). For clap-generated errors specifically, narrow the sanitizer to operate only inside `'...'` quoted regions so clap's structural newlines and formatting survive — applying `display_safe` to the whole clap-error string destroys clap's multi-line formatting (ITC L7 R2 hit this; the narrow-scope `sanitize_quoted_values` is the worked example). Detector: `grep -rE 'eprintln!\|format!\|panic!' src/` and verify each site that interpolates user input wraps with the sanitizer. Tests: include an integration test that feeds a control-character-bearing input through every error path and asserts the rendered stderr is escape-free.

## Software Engineering

- **`.unwrap()` on user-facing paths** — Any `.unwrap()` or `.expect()` on `Result`/`Option` values derived from user input, file I/O, or network I/O is a finding unless the panic is intentional and documented.
- **`?` operator propagation** — Are errors propagated with `?` rather than `.unwrap()`/`.expect()` where the calling function can recover or surface the error?
- **Error type hierarchy** — Is there a coherent error type strategy? (`thiserror` for libraries, `anyhow` for binaries, or a custom enum)? Are error messages user-readable at the top level?
- **Clippy lint configuration** — Is the clippy deny list configured at the crate level rather than relying on default warnings? The standard deny set is: `#![deny(clippy::all, clippy::pedantic, clippy::nursery, clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::missing_errors_doc, clippy::missing_panics_doc, missing_docs)]`. Note that `missing_docs` is a rustc lint (not a clippy lint) and pairs with the rustdoc-coverage check in the Technical Writer section (G-137) — including it in the crate-level deny set catches missing public-item documentation at clippy/cargo-check time rather than only at `cargo doc` time. Any deviation from this baseline requires documented rationale. Selective `#[allow(...)]` with a comment is acceptable; a weaker global deny set is a finding.
- **Clippy as idiom proxy** — Are Clippy lints treated as code review? Suppressed lints with no comment are a finding.
- **Lifetimes and cloning** — Is data cloned where a reference or borrow would work without lifetime complexity? Is excessive cloning masking a design issue?
- **`unwrap_or_else` vs. match** — Is `unwrap_or_else`, `map`, `and_then`, `or_else` used where a `match` would be clearer, or vice versa?

## Platform Engineering

- **`cargo audit`** — Is `cargo audit` run in CI? Does it fail the build on findings above the accepted severity threshold?
- **`cargo deny`** — Is `cargo deny check` run in CI with a `deny.toml`? This gates on CVEs, license violations, banned crates, and disallowed sources simultaneously. `cargo audit` alone is insufficient if `cargo deny` is not also present.
- **`cargo vet`** — For projects requiring supply-chain assurance: is `cargo vet` run in CI and configured to block unreviewed dependency additions?
- **`cargo clippy --deny warnings`** — Is `cargo clippy -- -D warnings` enforced in CI so new Clippy warnings fail the build?
- **`cargo fmt --check`** — Is `cargo fmt --check` run in CI to enforce consistent formatting without modifying files?
- **Coverage enforcement** — Is coverage measured in CI with thresholds enforced? Minimum 80% line coverage; 100% public API coverage. A CI run that measures coverage but does not fail below thresholds is not enforcement.
- **`Cargo.lock` commitment** — For binary crates and applications, is `Cargo.lock` committed to version control? (Libraries should exclude it; binaries should include it for reproducible builds.)
- **Toolchain pinning** — Is the Rust toolchain version pinned via `rust-toolchain.toml` to ensure reproducible builds across environments?

## Data Engineering

- **`serde` boundary validation** — Is data deserialized from external sources (files, stdin, APIs) validated after deserialization? `#[derive(Deserialize)]` succeeds on structurally valid JSON but does not enforce domain constraints (non-empty strings, valid ranges, etc.).
- **`#[serde(default)]` for schema evolution** — Are new optional fields marked `#[serde(default)]` so data written under old schemas can still be deserialized?
- **Deserialization error handling** — Are `serde_json::from_str` / `serde_json::from_reader` errors propagated to the caller or surfaced to the user with context, not silently discarded?
- **Sensitive data in serialized output** — If structs are serialized for storage or logging, are sensitive fields excluded with `#[serde(skip)]` or redacted?

## Red Team

- **Integer overflow in release builds** — Rust debug builds panic on integer overflow; release builds wrap silently (two's complement). Any arithmetic on values derived from user input (file sizes, counts, offsets, indices) is a potential overflow in release mode. Test by: providing inputs near `u32::MAX`, `usize::MAX`, or other relevant type boundaries and verifying the application rejects or saturates rather than wrapping. Use `checked_add`, `saturating_add`, or `wrapping_add` explicitly where overflow semantics matter.
- **Panic as a DoS vector** — Any `.unwrap()` or `.expect()` on `Result`/`Option` values derived from user input can be triggered by a crafted input to produce a panic and terminate the process. For a CLI this crashes the session; for a server this kills the thread or process. Enumerate every `.unwrap()` / `.expect()` on a user-influenced value and verify it is either protected by prior validation or that a panic in that context is acceptable.
- **Path traversal via user-supplied paths** — If the application reads from or writes to paths containing user-supplied components, verify that `../` sequences and absolute path escapes are rejected before I/O. Rust's `Path::join` does not sanitize: `base.join("../../etc/passwd")` resolves outside the base. Use `canonicalize` and verify the result is within the intended root, or reject non-component path segments explicitly.
- **`unsafe` block exploitation** — Each `unsafe` block is a trust boundary. For each block: what invariant is being asserted? Can a caller violate that invariant by providing crafted input? Named concerns: `slice::from_raw_parts` with attacker-controlled length; `transmute` between types where the target type's invariants can be violated; FFI calls where the foreign function's safety contract is not documented.
- **Crates.io supply chain** — Check for dependencies that shadow standard library names or use names close to popular crates (typosquatting). Use `cargo deny check` to detect banned or unreviewed sources. For any dependency added recently or at an unusual version: verify the crate author and publication history on crates.io.

## Performance Engineer

- **Criterion benchmarks** — Are performance-critical functions benchmarked with `criterion`? Named benchmarking discipline: benchmarks live in `benches/`, use `criterion::black_box` to prevent optimizer elision, and run under `cargo bench` in CI or on demand. A function documented as "fast" with no benchmark is an assertion without evidence.
- **Flamegraph profiling** — Use `cargo flamegraph` (which wraps `perf` on Linux or `dtrace` on macOS) to identify hot paths under realistic workloads. Named failure mode: a function that appears unexpectedly hot due to excessive cloning, allocation in a loop, or a hash map lookup inside a tight iteration.
- **Debug vs. release build performance** — Debug builds include overflow checks, no inlining, and no optimization — they can be 10–100x slower than release builds for CPU-bound code. Performance measurements taken against debug builds are not representative. Verify that performance claims and benchmarks are based on `--release` builds.
- **Allocation patterns** — Does the hot path allocate? Named failure modes: `String::from` or `.to_string()` inside a loop that could use `&str`; `Vec::new()` inside a function called per-item in a collection; repeated `.clone()` of heap data where a reference would work. Use `cargo-flamegraph` or `heaptrack` to identify allocation hot spots.
- **Async task overhead** — For `tokio`/`async-std` projects: are there blocking operations (file I/O, CPU-bound computation, `thread::sleep`) called inside async functions without `spawn_blocking`? A blocking call inside an async executor starves other tasks on the same thread.

## Solution Architect

- **CLI parsing separated from business logic** — Is `clap` (or equivalent) argument parsing in `main.rs` or a thin `cli.rs` module, with business logic in separate modules that take typed arguments, not raw `ArgMatches`?
- **Command enum dispatch** — For multi-command CLIs, is a `Command` enum used for exhaustive dispatch rather than a chain of `if`/`else if` on string comparisons?
- **Error type hierarchy as architecture** — Are error types defined at module boundaries and composed upward, so callers receive typed errors they can match on?
- **`lib.rs` / `main.rs` split** — For projects that could benefit from library use, is the core logic exposed in `lib.rs` and thin wiring in `main.rs`, enabling unit testing without subprocess overhead?

## Technical Writer

- **rustdoc coverage (G-137)** — Are all public items (`pub fn`, `pub struct`, `pub enum`, `pub trait`, `pub type`) documented with `///` doc comments? Named check: **`RUSTDOCFLAGS="-D missing_docs" cargo doc --no-deps`** — any output is a finding. Do NOT rely on `cargo doc --no-deps 2>&1 | grep "missing documentation"`: `cargo doc` does not warn on missing docs unless the `missing_docs` lint is explicitly enabled either in `Cargo.toml` (via `[lints.rust] missing_docs = "deny"`) or at invocation time via `RUSTDOCFLAGS`. The grep-clean default output is not evidence of documentation coverage — it's evidence the lint was off. (Recurrence: ITC TW R4 reported the grep-form check clean; TW R6 caught 9 missing-doc errors with the stricter `RUSTDOCFLAGS` form.) For library crates, missing documentation on public exports is a finding. For binary-only crates, exported functions in `lib.rs` must be documented; internal functions in `main.rs` may be omitted. Coordinate with PE supplement § Platform Engineering — the clippy/lint deny set should include `missing_docs` so the gap is caught at lint time, not just at `cargo doc` time.
- **Doc test quality** — Do `///` examples compile and produce the expected output? Run `cargo test --doc` to verify. An example that is syntactically valid but produces wrong output is a documentation failure — it misleads callers about the function's behavior. An example that does not compile causes `cargo test` to fail and must be fixed or wrapped in `# fn main() {}` / `# use ...` to provide context.
- **Module-level documentation** — Does each module root (`mod.rs` or `lib.rs`) have a `//!` inner doc comment explaining the module's purpose and its major types/functions? Module-level docs are the entry point for callers navigating the crate with `cargo doc`. A crate with no module-level documentation is navigable only by reading source, not docs.
- **`#[doc(hidden)]` discipline** — Are internal items that must be `pub` for technical reasons (e.g., macro internals, proc-macro helpers) marked `#[doc(hidden)]`? An item that appears in the public docs but is not intended for callers pollutes the API surface and confuses consumers. Conversely, public items that are part of the intended API must not be hidden.
- **`cargo doc --document-private-items`** — For internal documentation (when the project has contributors or reviewers who need to navigate private implementation): is `cargo doc --document-private-items` used or configured? A codebase with undocumented private internals is harder to onboard into and harder for the IAR process to evaluate.

## Localization

- **`fluent-rs` bundle configuration** — If Fluent is used: are `.ftl` files per locale stored in a consistent location (e.g., `i18n/` or `locales/`)? Is the `FluentBundle` constructed with an explicit `LanguageIdentifier` rather than a raw string? Is a fallback locale chain configured so missing messages gracefully degrade to a base locale rather than panicking or returning the message ID?
- **Fluent message completeness** — Are all user-visible strings defined in `.ftl` files rather than hardcoded as string literals in source? Named failure modes: a `println!("{}", msg)` with a hardcoded English string bypassing the bundle; a `format!()` call constructing user-visible output inline. A search for user-visible string literals outside `i18n/` or `locales/` is the practical check.
- **Missing message error handling** — Are missing Fluent messages handled explicitly? `FluentBundle::format_pattern` returns errors when a message or its attributes are missing. Silently using the message ID as the display string is an acceptable fallback only if explicitly documented; panicking on a missing message key is never acceptable in a deployed binary.
- **`rust-i18n` macro usage** — If `rust-i18n` is used: are all translation keys defined in the default locale YAML before being referenced with the `t!()` macro? Missing keys in the default locale cause the macro to return the key string at runtime. Run `cargo test` with the `rust-i18n` extraction feature enabled to verify key coverage. Are locale files generated from a canonical source (extraction) rather than manually maintained in parallel?

---

## Three-audience lens

The Rust supplement covers Rust-specific dimensions across 10+ domain perspectives. Per the [Three-audience design principle](../suite-development/suite-development.md#three-audience-design-principle-review-80-finding-3) ([Review 92 Finding 2](../suite-development/review-log/2026-05-24-suite-review.md#review-92--2026-05-24-0419z) cascade-applied for per-language supplements with broad domain-perspective coverage):

- **Suite developers** (contributors extending Rust-specific dimensions) read this supplement to understand which Rust idioms / failure modes the methodology teaches as canonical + how to extend per-domain sections when Rust ecosystem changes (new toolchain features; new cargo subcommands; new lint groups). Each per-domain section under `## <Domain>` is the extension surface; new dimensions land in the relevant per-domain section with the canonical Rust authority cited + the named failure mode named.
- **Suite users** (project teams applying VSDD to a Rust project) read this supplement alongside the domain prompt when running each domain's IAR cycle. The per-domain sections (`## Software Engineering`, `## Quality Engineering`, `## Security`, etc.) are the Rust-specific add-ons to the corresponding domain's standard dimensions. When authoring a per-domain review-log entry, declare via the `**Supplements applied:**` preamble field (per [Review 91 Finding 2](../suite-development/review-log/2026-05-23-suite-review.md#r91-f2)) which section(s) of this supplement informed the round.
- **AI agents** (parallel cold-session reviewers + main-session orchestrators) read this supplement as the Rust-specific failure-mode catalog. Per-domain sections are H2-anchored; agent grep idiom for SE Rust-specific failure modes: `awk '/^## Software Engineering/,/^## /' vsdd-suite/supplements/rust.md` returns the section's full content. Each named failure mode (bold-key-value form `- **<failure class>** — <description>`) is a substantive defect-class to assess; absent named-failure-mode coverage in a per-domain section is itself a methodology gap (signal: a Rust review surfaces a defect class not in the supplement → file as a supplement-extension finding).

The companion review dimensions per audience map: SO scopes which Rust idioms are spec-promised (suite-developer); Documentation Reviewer audits the supplement's clone-and-follow fidelity (suite-user); AI Engineer audits the supplement's per-domain coverage + cite-verify discipline at finding-authoring time (agent); TW audits the supplement's prose for cold-reader readability across all per-domain sections (cross-audience).
