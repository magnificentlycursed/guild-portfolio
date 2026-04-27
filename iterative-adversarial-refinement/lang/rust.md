# Rust Language Supplement

These dimensions supplement the standard IAR domain reviews for Rust projects. During each domain review, apply the relevant section below in addition to the standard dimensions for that domain.

---

## Quality Engineering

- **Test structure** — Are unit tests colocated with the code they test (`#[cfg(test)]` modules), and integration tests in `tests/`? Are doc tests present for public API functions?
- **Integration tests invoke the binary** — For CLI projects, do integration tests invoke the compiled binary (not internal functions) and assert on stdout, stderr, and exit code?
- **Clippy compliance** — Does `cargo clippy` pass without warnings? Are any `#[allow(...)]` suppressions justified with a comment?
- **Error path coverage** — Are `Err` branches, `None` arms, and panic conditions exercised in tests, not just the happy path?
- **Doc tests compile and pass** — Are doc examples in `///` comments syntactically correct and tested via `cargo test`?
- **Coverage thresholds** — Line coverage should be at minimum 80%. Public API coverage should be 100% — every exported function, type, and trait impl must have at least one test exercising it. Coverage below these thresholds is a finding. (Source: claude.md; verify against current apprentice-onboarding content for authoritative thresholds.)

## Security

- **File path validation** — If the application reads from or writes to user-supplied paths, are traversal attacks (`../`) and absolute path escapes validated before I/O?
- **`.unwrap()` discipline** — Is `.unwrap()` on `Result` or `Option` used on paths where panicking is acceptable (test code, programmer errors on provably-safe values)? Is it absent from user-facing paths where the input is untrusted?
- **Dependency audit** — Is `cargo audit` run against `Cargo.lock` to detect known CVEs in dependencies? Are any `RUSTSEC` advisories acknowledged?
- **cargo-deny** — Is `cargo deny check` configured with a `deny.toml`? A complete `deny.toml` configures four sections: `[advisories]` (CVE policy), `[licenses]` (allowed/denied SPDX identifiers), `[bans]` (duplicate version policy, denied crates), and `[sources]` (allowed registries and git sources). Missing or incomplete `deny.toml` is a finding. (Source: claude.md; verify against current apprentice-onboarding content.)
- **cargo-vet** — For projects with supply-chain risk, is `cargo vet` configured to require audit records for dependencies? For personal portfolio projects this may be deferred; for any project with production users or sensitive data it is a finding if absent. (Source: claude.md; verify against current apprentice-onboarding content.)
- **Unsafe usage** — Is `unsafe` code present? Is each block justified with a comment explaining why safety invariants hold? Is there a test or formal argument for each unsafe block?
- **Secrets in code** — Are API keys, tokens, or credentials hardcoded? Are they read from environment variables or a secrets file excluded from version control?

## Software Engineering

- **`.unwrap()` on user-facing paths** — Any `.unwrap()` or `.expect()` on `Result`/`Option` values derived from user input, file I/O, or network I/O is a finding unless the panic is intentional and documented.
- **`?` operator propagation** — Are errors propagated with `?` rather than `.unwrap()`/`.expect()` where the calling function can recover or surface the error?
- **Error type hierarchy** — Is there a coherent error type strategy? (`thiserror` for libraries, `anyhow` for binaries, or a custom enum)? Are error messages user-readable at the top level?
- **Clippy lint configuration** — Is the clippy deny list configured at the crate level rather than relying on default warnings? The standard deny set is: `#![deny(clippy::all, clippy::pedantic, clippy::nursery, clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::missing_errors_doc, clippy::missing_panics_doc)]`. Any deviation from this baseline requires documented rationale. Selective `#[allow(...)]` with a comment is acceptable; a weaker global deny set is a finding. (Source: claude.md; verify against current apprentice-onboarding content.)
- **Clippy as idiom proxy** — Are Clippy lints treated as code review? Suppressed lints with no comment are a finding.
- **Lifetimes and cloning** — Is data cloned where a reference or borrow would work without lifetime complexity? Is excessive cloning masking a design issue?
- **`unwrap_or_else` vs. match** — Is `unwrap_or_else`, `map`, `and_then`, `or_else` used where a `match` would be clearer, or vice versa?

## Platform Engineering

- **`cargo audit`** — Is `cargo audit` run in CI? Does it fail the build on findings above the accepted severity threshold?
- **`cargo deny`** — Is `cargo deny check` run in CI with a `deny.toml`? This gates on CVEs, license violations, banned crates, and disallowed sources simultaneously. `cargo audit` alone is insufficient if `cargo deny` is not also present. (Source: claude.md; verify against current apprentice-onboarding content.)
- **`cargo vet`** — For projects requiring supply-chain assurance: is `cargo vet` run in CI and configured to block unreviewed dependency additions? (Source: claude.md; verify against current apprentice-onboarding content.)
- **`cargo clippy --deny warnings`** — Is `cargo clippy -- -D warnings` enforced in CI so new Clippy warnings fail the build?
- **`cargo fmt --check`** — Is `cargo fmt --check` run in CI to enforce consistent formatting without modifying files?
- **Coverage enforcement** — Is coverage measured in CI with thresholds enforced? Minimum 80% line coverage; 100% public API coverage. A CI run that measures coverage but does not fail below thresholds is not enforcement. (Source: claude.md; verify against current apprentice-onboarding content.)
- **`Cargo.lock` commitment** — For binary crates and applications, is `Cargo.lock` committed to version control? (Libraries should exclude it; binaries should include it for reproducible builds.)
- **Toolchain pinning** — Is the Rust toolchain version pinned via `rust-toolchain.toml` to ensure reproducible builds across environments?

## Data Engineering

- **`serde` boundary validation** — Is data deserialized from external sources (files, stdin, APIs) validated after deserialization? `#[derive(Deserialize)]` succeeds on structurally valid JSON but does not enforce domain constraints (non-empty strings, valid ranges, etc.).
- **`#[serde(default)]` for schema evolution** — Are new optional fields marked `#[serde(default)]` so data written under old schemas can still be deserialized?
- **Deserialization error handling** — Are `serde_json::from_str` / `serde_json::from_reader` errors propagated to the caller or surfaced to the user with context, not silently discarded?
- **Sensitive data in serialized output** — If structs are serialized for storage or logging, are sensitive fields excluded with `#[serde(skip)]` or redacted?

## Solution Architect

- **CLI parsing separated from business logic** — Is `clap` (or equivalent) argument parsing in `main.rs` or a thin `cli.rs` module, with business logic in separate modules that take typed arguments, not raw `ArgMatches`?
- **Command enum dispatch** — For multi-command CLIs, is a `Command` enum used for exhaustive dispatch rather than a chain of `if`/`else if` on string comparisons?
- **Error type hierarchy as architecture** — Are error types defined at module boundaries and composed upward, so callers receive typed errors they can match on?
- **`lib.rs` / `main.rs` split** — For projects that could benefit from library use, is the core logic exposed in `lib.rs` and thin wiring in `main.rs`, enabling unit testing without subprocess overhead?
