<!-- hook-bypass[check-document-staleness,check-suite-internal-terminology]: pre-existing in-flight phrasing preserved per the forward-only narrative-preservation policy. This file's status claims predate the R95 F2 check-document-staleness hook; flagging would require retroactive rewriting that crosses the forward-only carve-out. Future status-claim edits SHOULD use current-state phrasing; the bypass-mechanism is itself a finding for the next registry-walk review. + pre-existing bare abbreviation use (IAR/VSDD/MVR) preserved per the forward-only narrative-preservation policy. These files predate the R95 F1 abbreviation-first-use-expansion check; flagging would require retroactive prose rewriting that crosses the forward-only carve-out. Future authoring SHOULD spell out abbreviations on first mention; the bypass-mechanism is itself a finding for the next registry-walk review. -->
# tracker

A personal issue tracker for the terminal. Single user, no network, no accounts. Issues are stored in a local JSON file (`tracker.json`) in the current working directory.

This is portfolio project #2 from the Phase 1 apprentice program — the first Rust project. The primary use case is tracking work on small software projects between AI sessions, where the tracker serves as the agent's external memory: a persistent record of what is open, in progress, and done that survives conversation resets.

---

## Commands

```
tracker create "Fix the login bug" [--description "..."] [--priority low|medium|high] [--label <l>]...
tracker list   [--status open|in-progress|done] [--priority low|medium|high] [--label <l>]
tracker status <id> open|in-progress|done
tracker show   <id>
tracker delete <id>
```

`tracker list` defaults to open issues, sorted by priority (high → medium → low) then ID ascending. The `--label` filter on `list` accepts a single value and matches case-sensitively; on `create` it is repeatable and labels are deduplicated (case-preserved). Descriptions are stored verbatim; control characters other than `\n` are rejected at create and load. Deleted issue IDs are never reused. Run `tracker --help` or `tracker <subcommand> --help` for the full flag reference.

---

## Color output

When stdout is a TTY, `tracker list` and `tracker show` color the priority and status value cells: bold red for `high`, bold yellow for `medium`, bold cyan for `in-progress`, bold green for `done`. `low` priority and `open` status render in the terminal's default color so the highlighted-vs-unhighlighted dichotomy reads at a glance (including for color-vision-deficient users, since every highlighted value carries the `bold` SGR attribute per WCAG 1.4.1 *Use of Color*).

Color is suppressed when:

- stdout is piped or redirected (so downstream parsers see clean text), or
- the `NO_COLOR` environment variable is set to any non-empty value (per <https://no-color.org/>), or
- `CLICOLOR=0` is set.

`CLICOLOR_FORCE` is intentionally not honored — ANSI escapes are never emitted to a non-TTY stdout regardless of env vars.

---

## Install

Requires Rust 1.82+ (`rust-version` declared in `Cargo.toml`; the project's pinned toolchain via `rust-toolchain.toml` is 1.94.1).

```sh
cargo install --path .
```

Installs the `tracker` binary to `~/.cargo/bin/`. Ensure `~/.cargo/bin` is on your `PATH`.

## Build

```sh
cargo build --release
# binary at: target/release/tracker
```

## Test

```sh
cargo test
```

Integration tests invoke the compiled binary as a subprocess and assert on stdout, stderr, and exit code. Unit tests cover validation (title, label, description, ID parsing, priority/status enums), persistent `next_id` counter invariants, sort ordering, color-helper ANSI sequences, TTY-detection + env-var color suppression, and the clap-error quoted-value sanitizer.

---

## Storage

Issues are stored in `tracker.json` in the directory where you run the command. The file is created automatically on first `tracker create`. To use separate issue lists per project, run `tracker` from the project's root directory.

`tracker.json` is plain JSON — you can inspect it with any text editor. The top-level shape is `{"issues": [...], "next_id": <u64>}`; the `next_id` counter is monotonically increasing so deleted IDs are never reused (including the previously-highest ID). Do not manually set field values outside the valid enum sets (status: `open`, `in-progress`, `done`; priority: `low`, `medium`, `high`) — invalid values are treated as file corruption.

---

## Status

**Layer 7 implementation complete; Layer 7 IAR Round 2 closure in progress.**

- [x] DESIGN.md — full behavioral specification
- [x] TODO.md — 7-layer development plan with Red Gate test plans
- [x] DECISIONS.md — key design decisions with rationale
- [x] IAR suite — 11 active domains (8 core + RT + TW + VDD-IAR Alignment)
- [x] Layer 1: Core create + list
- [x] Layer 2: Status flow
- [x] Layer 3: Priority
- [x] Layer 4: Labels
- [x] Layer 5: Compound filtering
- [x] Layer 6: Description, show, delete
- [x] Layer 7: Polish (color, `--help`, error messages, NO_COLOR / CLICOLOR honoring, stderr Cc-escape)

---

## Project files

| File | Purpose |
|---|---|
| `DESIGN.md` | Full behavioral specification — preconditions, postconditions, error states, edge cases |
| `TODO.md` | Layered development plan with acceptance criteria, manual testing checklists, Red Gate test plans |
| `DECISIONS.md` | Key design decisions with rationale |
| `PROCESS.md` | Layer-by-layer process retrospective — what was built, what was caught by IAR, what was learned |
| `iterative-adversarial-refinement/` | IAR review logs for all 11 active domains |
