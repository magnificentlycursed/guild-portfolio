# CLI Interface Type Supplement

These dimensions supplement the standard IAR domain reviews for command-line interface projects. The standard UX review dimensions assume a browser-rendered interface — for CLI projects, use the dimensions below in place of the standard UX dimensions. Additional CLI-relevant dimensions appear under Quality Engineering and Software Engineering.

---

## UX (CLI replacement dimensions)

The following replace the browser-centric UX standard dimensions for CLI projects. Apply all that are relevant.

1. **Command discoverability** — Does `--help` produce output that accurately describes all available subcommands, flags, and arguments? Is the help text complete, current, and not misleading? Does the top-level help include a usage example?
2. **Argument and flag design** — Are arguments named consistently with conventions of the host platform (`--kebab-case` on Unix, etc.)? Are positional arguments used only where the semantic is unambiguous? Are required vs. optional arguments clearly communicated in `--help`?
3. **Output scannability** — Is output structured for human reading — one item per line for lists, consistent columns for tabular data? Is there unnecessary noise (debug lines, progress artifacts) in default output that should be behind a verbose flag?
4. **`stdout` / `stderr` discipline** — Does usable output (data the user pipes or redirects) go to `stdout`? Do status messages, progress, warnings, and errors go to `stderr`? Are these correctly separated so piped output is clean?
5. **Exit codes** — Does the command exit `0` on success and non-zero on failure? Are distinct error conditions represented by distinct non-zero exit codes where callers need to distinguish them?
6. **Empty state messages** — When a command succeeds but produces no results (e.g., list with no items), is there a clear message indicating the empty result rather than silent zero output? Is the empty message on `stderr` so it does not pollute piped output?
7. **Destructive operation confirmation** — Do commands that delete, overwrite, or irreversibly modify data require explicit confirmation (e.g., `--force` flag or interactive prompt) before proceeding? Is the confirmation UX clear about what will be affected?
8. **Error message quality** — Do error messages include: what failed, why it failed (if knowable), and what the user should do next? Are errors prefixed with a label (`error:`, `warning:`) to distinguish severity? Are error messages on `stderr`?
9. **Interruption handling** — Does the command handle `SIGINT` (Ctrl+C) gracefully without leaving partial state? If interrupted mid-write, is the output file truncated or rolled back rather than left corrupt?
10. **Machine-readable output** — If the output is intended to be piped or parsed by other programs, is a `--json` (or equivalent) flag available? Is the JSON schema consistent across versions?
11. **Verbose / quiet modes** — Is there a `--verbose` or `-v` flag for diagnostic output? A `--quiet` or `-q` flag to suppress informational messages for scripting use?

## Quality Engineering

- **Integration tests invoke the binary** — Do integration tests invoke the compiled or packaged binary (not internal functions) via subprocess? Testing internal functions directly misses argument parsing errors, output formatting bugs, and exit code regressions.
- **`stdout` / `stderr` / exit code assertions** — Do tests assert on the full interaction: stdout content, stderr content, and exit code? A test that only checks exit code will miss output regressions; a test that only checks stdout will miss error-path regressions.
- **Piped input and edge-case args** — Are tests present for piped stdin, empty input, `--`, special characters in arguments, and paths with spaces?
- **Interruption test** — Is there a test or documented manual check for `SIGINT` behavior if the command modifies state?

## Software Engineering

- **Output formatting as a code concern** — Is output formatting code separated from business logic? A function that computes a result and also formats it for display is harder to test and maintain.
- **User-visible strings centralized** — Are error messages and output labels defined in one place (constants or a dedicated module), or scattered throughout the codebase? Scattered strings make it difficult to audit the user-facing language consistently.
- **Structured result types before formatting** — Does the program compute a result as a typed value and then format it, rather than building output strings inline during computation? This separation enables both human-readable and `--json` output from the same logic.
