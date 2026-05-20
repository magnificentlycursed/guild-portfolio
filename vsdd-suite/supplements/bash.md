# Bash Language Supplement

These dimensions supplement the standard IAR domain reviews for Bash scripts. During each domain review, apply the relevant section below in addition to the standard dimensions for that domain.

**Authored:** Review 76 (2026-05-20) — alongside the Python supplement, when a human reviewer surfaced the suite's hook-extension misnomer. The Python supplement covers the 4 Python hooks (post-rename to `.py`); this Bash supplement covers the 3 actually-bash scripts (`check-review-log-anonymization.sh`, `templates/cold-session-dispatch.sh`, `templates/scaffold-project.sh`) plus any future bash scripts in the suite.

**Multi-domain authoring note:** the sections below were drafted with the relevant role-domain perspectives — QE (bats-core; shell-test patterns), Security + Red Team (Bash's word-splitting, quoting, and injection failure modes; `set -e` gotchas), SE + SA (idioms — `[[ ]]` over `[ ]`; arrays; function structure; trap-based cleanup), PE (shellcheck in CI; portability; bash version requirements), DE (jq for JSON; awk for CSV), TW + Doc Reviewer (`--help` discipline; the man-page-equivalent inline-doc-block pattern). The supplement is forward-looking against the 2026 bash ecosystem (bash 5.x; shellcheck 0.10+; bats-core as the de facto test framework).

**Scope of this supplement:** any script with a `#!/usr/bin/env bash` (or `#!/bin/bash`) shebang. POSIX `/bin/sh` scripts are out of scope — POSIX shell has its own failure modes (no `[[ ]]`, no arrays, different parameter expansion) that warrant a separate supplement if the suite ever ships POSIX-shell artifacts. The recommendation: don't write POSIX-shell scripts in new projects unless interop with non-bash systems demands it.

---

## Quality Engineering

- **`bats-core` for shell testing** — Is `bats-core` (Bash Automated Testing System) used to test bash scripts? `bats` provides `@test "description" { ... }` blocks, `assert` helpers (via `bats-assert`), and parallel execution. Install: `git submodule` or `apt install bats` / `brew install bats-core`. Invocation: `bats test/*.bats`. A bash script of more than 50 lines without `bats` tests is a finding for any project with reviewer attention.
- **Test isolation via `setup`/`teardown`** — Are tests isolated from each other? `bats-core` supports `setup_file` (once per file), `setup` (per-test), `teardown` (per-test), `teardown_file` (once per file). Named failure modes: a test that depends on prior test state (`test_a` leaves a file behind that `test_b` reads); a test that mutates `$PWD` without restoring it; tests that share a temp directory without per-test isolation. Use `mktemp -d` per-test for filesystem isolation.
- **Exit-code coverage** — Are non-zero exit codes tested as primary behaviors, not just side-effects? Named pattern: `run script.sh invalid-arg; [ "$status" -eq 1 ]; [[ "$output" =~ "expected error" ]]`. Tests that only assert exit 0 on the happy path miss the entire error-handling surface.
- **Shellcheck as a quality gate** — Is `shellcheck script.sh` run as a pre-commit hook AND in CI? Shellcheck is the canonical Bash linter; its warnings are the closest thing the Bash ecosystem has to a type checker. Treat shellcheck warnings as build failures (`shellcheck --severity=warning` is the recommended default; `error` is too lax). Suppressions (`# shellcheck disable=SCxxxx`) require a comment explaining why the flagged pattern is intentional.
- **Coverage with `kcov`** — For projects where bash-script test coverage matters, `kcov` instruments shell scripts and produces line coverage reports. Install: `apt install kcov`. Invocation: `kcov coverage/ bats test/*.bats`. Coverage thresholds at 70%+ are reasonable for shell (lower than Python/Rust because shell error paths are often unreachable defensively).
- **Test the actual command, not internals** — A bash script's "API" is its CLI: argv, environment, stdin, exit code, stdout, stderr, side effects. Tests should invoke the script as a user would and assert on those observable channels. Tests that source the script and call internal functions test the wrong thing — the wrong-thing-passing risk is real.

---

## Security

- **`set -euo pipefail` baseline** — Does every bash script start with `set -euo pipefail`? `set -e` exits on first error; `set -u` treats unset variables as an error; `set -o pipefail` propagates errors through pipes. Missing any of the three is a finding. Named failure modes without these: a script that silently continues past a failed command and produces a corrupt output; a typo in a variable name silently treated as the empty string and used in a destructive `rm -rf "$tmpdir/$prefix*"`; a pipe whose first stage failed but whose last stage succeeded.
- **`IFS=$'\n\t'` default for safer word splitting** — Is `IFS` (Internal Field Separator) explicitly set to a safer default than the bash default (space + tab + newline)? Common safe choice: `IFS=$'\n\t'` (drop space — so paths-with-spaces don't word-split). The default IFS is the source of many quoting bugs. Combined with `set -euo pipefail`, this is the standard prelude.
- **Quote every variable expansion** — Every `$var` and `${var}` should be `"$var"` and `"${var}"`. Unquoted expansion is subject to word splitting + glob expansion + IFS interpretation, all of which are silent injection vectors. Shellcheck SC2086 catches this. Named worked attack: `rm -rf $tmpdir` where `$tmpdir` contains `/ tmp` expands to `rm -rf / tmp` — recursive deletion of root. The quoted form `rm -rf "$tmpdir"` is the safe shape.
- **`printf %q` for shell-quoting user-supplied values** — When constructing a command string from user input (avoid this when possible; pass argv instead), use `printf %q` to shell-quote the value: `safe=$(printf %q "$untrusted"); eval "command $safe"`. Without `%q`, shell metacharacters in the input become injection vectors. The stronger pattern: structure the script so user input is argv, not shell-string-component, and `eval` is not used.
- **`eval` discipline** — Is `eval` used? Each `eval` call site needs justification — if any user input reaches `eval`, command injection. Named worked attack: `eval "$user_string"` where `user_string="ls; rm -rf ~"` — the semicolon ends the `ls` command and starts an arbitrary one. The rule: `eval` ONLY on internally-constructed strings, NEVER on user-supplied data.
- **Temp file race conditions** — Are temp files created via `mktemp` (which uses a secure unique name) rather than predictable patterns (`/tmp/myscript.$$.tmp`)? Predictable names enable a symlink race attack: an attacker creates `/tmp/myscript.<pid>.tmp` as a symlink to a sensitive file before the script writes to it; the script's write follows the symlink. `mktemp -d` for temp directories; `mktemp -t myscript.XXXXXX` for temp files. Always trap the cleanup: `tmp=$(mktemp -d); trap 'rm -rf -- "$tmp"' EXIT`.
- **Path traversal in archive extraction** — `tar -xf user-supplied.tar` extracts paths as-named, including `../../../etc/passwd`. Modern tar versions support `--no-same-owner`, `--no-overwrite-dir`, and `--strip-components=N`; for fully-untrusted archives, validate every entry's path before extraction or use a Python/Rust archive library with safe defaults instead of `tar`.
- **PATH manipulation** — Does the script rely on `$PATH` to find commands, or does it use absolute paths or `command -v` to locate them? A script that calls `rm` is calling whatever `rm` is in `$PATH`; a hostile `$PATH` (`PATH=/tmp:/usr/bin`) lets an attacker shadow `rm` with a malicious binary. Defense: at script start, set `PATH=/usr/local/bin:/usr/bin:/bin` to a known-good value, or use absolute paths for security-critical commands.
- **Don't trust filenames from `find` / `ls` without `-print0` / `--null`** — `find . -name "*.txt" | xargs rm` breaks on filenames with spaces, quotes, or newlines. Use `find . -name "*.txt" -print0 | xargs -0 rm` (NUL-terminated) for safety. Better: `find . -name "*.txt" -delete` (no pipe).

---

## Software Engineering

- **`[[ ]]` over `[ ]`** — Use `[[ condition ]]` (bash conditional expression) rather than `[ condition ]` (the POSIX `test` command). `[[ ]]` doesn't word-split its operands and supports `=~` (regex), `<` / `>` (string comparison), `&&` / `||` (logical operators). `[ ]` is the legacy POSIX form and has more footguns.
- **Array discipline** — When a script needs a list of values, use bash arrays (`arr=(a b c); echo "${arr[@]}"`) not space-separated strings (`vars="a b c"; for v in $vars; do ...`). Arrays preserve element boundaries; strings re-split per IFS and break on values containing spaces.
- **`local` for function variables** — Inside functions, declare variables with `local`: `f() { local x="$1"; ...; }`. Without `local`, the variable is global and leaks out of the function. Named failure mode: a recursive function that uses an unscoped iteration variable, corrupting outer-frame state.
- **`readonly` for immutables** — Constants and configuration values that shouldn't be reassigned should be `readonly`: `readonly TIMEOUT=30`. This catches accidental reassignment.
- **`declare`/`typeset` for typed variables** — `declare -i count=0` (integer); `declare -a arr=()` (indexed array); `declare -A map=()` (associative array — bash 4+ only). Typed declaration documents intent and catches some misuse at runtime.
- **Function structure** — Long scripts (200+ lines) should be organized into functions with a `main()` that dispatches. The trailing `main "$@"` invocation is the standard pattern. A 500-line script with no functions is a finding for any project past prototype scope.
- **Avoid sourcing scripts that have side effects** — A script that runs commands at top-level when sourced (rather than executed) breaks the source-as-library pattern. Wrap top-level execution in `if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then main "$@"; fi`.
- **Use `command -v` to check for tool presence** — Before calling an external tool, verify it's installed: `if ! command -v jq >/dev/null 2>&1; then echo "jq required"; exit 1; fi`. The legacy `which jq` form has portability problems; `command -v` is POSIX-standard and reliable.
- **`shfmt` for consistent formatting** — Is `shfmt -d *.sh` (diff form) run as a pre-commit hook? `shfmt` is the de facto bash formatter (analogous to `gofmt` / `ruff format`). Inconsistent indentation, brace placement, and quoting styles mask substantive diffs.

---

## Platform Engineering

- **Shebang discipline** — Is the shebang `#!/usr/bin/env bash` (PATH-resolved, portable across systems where bash isn't at `/bin/bash`) rather than `#!/bin/bash` (hardcoded path, breaks on systems where bash is elsewhere)? `#!/usr/bin/env bash` is the modern recommendation.
- **Bash version requirement explicit** — If the script uses bash 4+ features (associative arrays `declare -A`, `mapfile`, `${var^^}` case conversion, `&>>` redirection), is the bash-version requirement documented? Named failure mode: a script that uses associative arrays runs on macOS's default `/bin/bash` 3.2 (Apple ships bash 3.2 by default for license reasons) and silently misbehaves. The check: at script start, `[[ "${BASH_VERSINFO[0]}" -ge 4 ]] || { echo "bash 4+ required"; exit 1; }`.
- **`shellcheck` in CI** — Is `shellcheck *.sh` run as a CI gate? Treat shellcheck warnings as build failures.
- **`shfmt --diff` in CI** — Same for formatting. A CI run that lints but doesn't format-check accumulates formatting drift.
- **Filename extension matches content** — `.sh` for bash scripts; `.py` for Python; `.rb` for Ruby; etc. A Python script with a `.sh` extension is a maintenance defect: editors apply bash syntax highlighting; pre-commit hooks scoped by extension miss it; readers expect bash conventions. This is the Review 76 surfacing — the suite's own 4 Python hooks were `.sh` "for parity" and have been renamed to `.py`.
- **No hardcoded paths** — Are user-home or system-specific paths hardcoded (`/Users/...`, `/home/me/...`, `/opt/specific-tool/`)? The portfolio's existing `check-no-home-paths.sh` hook catches this for source code; the same discipline applies to bash scripts. Use `$HOME`, `$XDG_*`, `$PWD` instead.
- **Trap-based cleanup** — For scripts that create temp files, lock files, or other transient state, is cleanup handled via `trap`? Pattern: `tmp=$(mktemp -d); trap 'rm -rf -- "$tmp"' EXIT INT TERM`. Without trap, ctrl-C leaves temp state behind.
- **Exit code discipline** — Does the script exit with meaningful codes? `0` for success; `1` for general error; `2` for misuse (invalid args, missing prereqs); `>2` for specific named errors. Document the exit-code contract in the script's `--help` output. Scripts that always exit 0 (even on error) break shell-pipeline expectations.

---

## Data Engineering

- **`jq` for JSON manipulation** — When a bash script needs to read or transform JSON, is `jq` used? Named failure modes: `grep '"key"' file.json | cut -d : -f2` (parses JSON as a regular file — breaks on whitespace variation, escaped strings, etc.); `python -c "import json; print(json.load(open('file.json'))['key'])"` (works but adds a Python dependency where `jq -r '.key' file.json` would do it natively).
- **`awk` over hand-rolled field splitting** — For tabular data (CSV, TSV, fixed-width), use `awk` rather than chained `cut`/`sed`/`grep`. `awk` handles field boundaries correctly; chained Unix tools accumulate edge-case failures.
- **CSV via `csvkit` / `mlr` (Miller)** — For non-trivial CSV manipulation in bash, install `csvkit` (Python-backed, `csvcut`/`csvjson`/`csvgrep`) or `mlr` (Miller — high-performance, handles CSV/TSV/JSON/Parquet). Hand-rolled CSV parsing in bash is wrong in subtle ways (quoting, embedded newlines, locale-dependent separators).
- **Resist YAML in bash** — YAML's whitespace-significant + multi-document + reference syntax is too rich to parse correctly in bash. Use `yq` (the jq-equivalent for YAML) or a Python/Rust tool. Hand-rolled YAML parsing in bash is always wrong.
- **Atomic file writes** — When updating a file, write to a temp file first and `mv` it into place: `tmp=$(mktemp); cmd > "$tmp" && mv -- "$tmp" target`. Without this, a process crash mid-write leaves a half-written file. The `mv` is atomic within a single filesystem.

---

## Red Team

- **Command injection via unquoted expansion** — Find every unquoted `$var` reference in the script. For each, ask: can a caller (CLI user, env var, file content the script reads) control the value? If yes, that's command injection. Named worked attack: `if [ $user_input = "foo" ]; then ...` where `user_input="" -o 1"` makes the test `[ = "foo" -o 1 ]` which is always true. The quoted form `if [[ "$user_input" = "foo" ]]; then` is immune.
- **Glob injection** — Unquoted `$var` is subject to globbing. Named worked attack: `cd $userdir` where `userdir="*"` expands to the first directory in CWD. The quoted form `cd "$userdir"` is safe.
- **PATH-shadowing attack** — A user controlling `$PATH` can shadow any command the script calls. Defense: set `PATH=/usr/local/bin:/usr/bin:/bin` at script start, or use absolute paths for security-critical commands.
- **Environment-variable injection** — Variables the script reads from the environment (`$HOME`, `$USER`, custom config vars) are attacker-controllable in many contexts. Don't trust them more than user input.
- **Shellshock-style function injection** — Bash 4.2 and earlier had the Shellshock vulnerability where function definitions in environment variables were executed at shell startup. Modern bash patched this, but the lesson — environment variables are an attack surface — persists. Don't blindly trust env-var-supplied data.
- **`source` / `.` from untrusted paths** — `source "$userpath"` and `. "$userpath"` execute the contents of the path in the current shell. Same shape as Python's `eval()` — never on untrusted data.
- **Race conditions on shared state** — Multiple invocations of the same script writing to the same lock file or temp directory race. Use `flock` for advisory locking or `mkdir` (atomic) for lock acquisition.
- **TOCTOU (Time-Of-Check-Time-Of-Use)** — `if [ -f "$path" ]; then cat "$path"; fi` — between the check and the use, the file can be replaced (e.g., with a symlink to a sensitive file). For security-critical paths, use `open` + `fstat` patterns (which bash doesn't have natively — drop to Python/C for this case).
- **`rm -rf` in a script** — Every `rm -rf` is a potential disaster vector. Defense: validate the target path is non-empty AND contains the project root prefix; use `mktemp -d` for temp dirs so the path is unpredictable to the attacker.

---

## Performance Engineer

- **Avoid subshells in tight loops** — Each `$(command)` and `` `command` `` forks a subshell. In a loop iterating 10,000 times, the fork overhead dominates. Named alternative patterns: use bash built-ins (`${var//pattern/replacement}` instead of `echo "$var" | sed 's/pattern/replacement/'`); use process substitution (`< <(command)`); aggregate the work into one external invocation.
- **`echo`/`printf` over `cat` for single-string output** — `cat <<EOF\ntext\nEOF` forks `cat`; `printf '%s\n' "text"` is a built-in. For one-line output, prefer `echo` or `printf`.
- **Streaming over batching** — When processing large inputs, stream through pipes rather than reading the whole input into a variable. `while read line; do ...; done < file` streams; `for line in $(cat file); do ...` reads the whole file AND word-splits each line.
- **Avoid `cat | grep` (UUOC)** — Useless Use Of Cat: `cat file | grep pattern` forks `cat` for no reason. Use `grep pattern file` directly. Similarly `cat file | wc -l` → `wc -l < file`.
- **Bash built-ins over external commands** — `${#var}` (string length, built-in) over `echo -n "$var" | wc -c` (forks `wc`); `${var^^}` (uppercase, bash 4+) over `echo "$var" | tr '[:lower:]' '[:upper:]'`; `[[ "$var" =~ pattern ]]` (regex, built-in) over `echo "$var" | grep -q pattern`.

---

## Solution Architect

- **`main` function structure** — Long scripts should be organized as `main() { ... }` invoked at the end as `main "$@"`. This makes the entry point explicit and allows the script to be sourced for testing without side effects.
- **Option parsing — `getopts` for short flags, manual for long** — For scripts taking flags, use `getopts` for short options (`-h`, `-v`, `-o file`). For scripts needing long options (`--help`, `--verbose`, `--output file`), hand-roll a `case` loop. The third option (`getopt`, GNU-specific) is non-portable.
- **`--help` parsing first** — Before any work, parse `--help` / `-h` and exit with usage. A script that does work before checking `--help` wastes the user's time when they typo a flag.
- **Function naming convention** — Functions in a script should be named verb-noun (`build_image`, `parse_args`, `validate_path`). Helper functions invoked only by other functions should be prefixed `_` (`_internal_helper`) — a convention, not enforced by bash.
- **Trap handlers organized at script start** — `trap 'cleanup' EXIT` handlers belong near the top of `main()`, immediately after temp-resource acquisition. A trap declared at the bottom of the script doesn't fire if an early-error exits before declaration. Pattern: `tmp=$(mktemp -d); trap 'rm -rf -- "$tmp"' EXIT INT TERM` immediately after `mktemp`.
- **Standard config-file conventions** — Scripts that take configuration should read from `$XDG_CONFIG_HOME/<scriptname>/config` (Linux) or `$HOME/.<scriptname>rc` (legacy). Hardcoded `/etc/scriptname.conf` is acceptable for system-level scripts.

---

## Technical Writer

- **`--help` output is the documentation** — A bash script's `--help` output is its primary documentation. It should: list every flag with a one-line description; show 1–3 usage examples; name the exit-code contract; cite the script's purpose in one sentence. A script whose `--help` says "see README.md" is a documentation failure — the script's user shouldn't have to context-switch to read the README.
- **Inline doc block at script top** — Below the shebang, a comment block describing the script's purpose, prerequisites, and usage. This is the maintainer's documentation (vs. `--help` which is the user's). Pattern:

  ```bash
  #!/usr/bin/env bash
  #
  # script-name — one-line summary
  #
  # Purpose: longer description of what the script does.
  # Prerequisites: <named tools> must be installed.
  # Usage: script-name [OPTIONS] [ARGS]
  # Exit codes: 0 = success; 1 = general error; 2 = misuse.
  #
  # Authored against bash 5.x; tested on macOS + Linux.
  ```

- **Function docstrings** — Each non-trivial function should have a comment immediately above it describing inputs, outputs, side effects, and exit-code contract.
- **README integration** — For scripts that ship as part of a project (not as a one-off), is there a README section describing how to install/run/test them? Bash scripts are easy to under-document because they look simple — the consequence is users running them with wrong arguments and getting cryptic errors.
- **No mojibake in inline strings** — Ensure the file is saved as UTF-8 and that locale-dependent characters render correctly. Verify by reading the script back: `cat script.sh | od -c | head` and look for unexpected escape sequences in user-facing strings.

---

## Documentation Reviewer

(Active when Technical Writer is active — paired per the forthcoming Documentation Reviewer domain registration. This section is forward-link only; the domain prompt at `vsdd-suite/domains/role/DOCUMENTATION-REVIEWER-REVIEW.md` lands in a subsequent Review. Until that domain registers, the section here documents the cold-reader bash-script dimensions for whoever runs the pair-validation in advance of formal domain registration.)

- **`--help` cold-reader test** — Given only `<script> --help`, can a cold reader run the script for its intended use case? Named failure modes: `--help` lists flags without describing what each does; usage examples reference paths/state the reader doesn't have; exit codes undocumented.
- **Script-name self-consistency** — Does the script's `--help` output use the same name as the file? Named failure: a script named `check-foo.sh` whose `--help` says "USAGE: foo [OPTIONS]" — the user calls `foo` from the docs but the file is `check-foo.sh`.
- **Error message executability** — When the script errors, does the error message tell the user what to do next? Named failure: `Error: invalid arg` (with no indication which arg or what would be valid). The standard: error messages name the problem AND the next action.
- **Tutorial-followability** — If the project's docs reference the script ("run `scripts/build.sh` to build"), can a cold reader follow the reference and reach the intended outcome? Named failure: the docs reference a script that requires environment-variable setup the docs don't mention.
- **Shebang portability awareness** — Does the script's documentation acknowledge bash-specific features? A project's CONTRIBUTING.md that says "any POSIX shell will work" while the scripts use `[[ ]]` and arrays is documentation rot.
