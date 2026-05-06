# tracker

A personal issue tracker for the terminal. Single user, no network, no accounts. Issues are stored in a local JSON file (`tracker.json`) in the current working directory.

This is portfolio project #2 from the Phase 1 apprentice program — the first Rust project. The primary use case is tracking work on small software projects between AI sessions, where the tracker serves as the agent's external memory: a persistent record of what is open, in progress, and done that survives conversation resets.

---

## Commands

Available now (Layer 4):

```
tracker create "Fix the login bug" [--priority low|medium|high] [--label <l>]...
tracker list [--status open|in-progress|done] [--priority low|medium|high] [--label <l>]
tracker status <id> open|in-progress|done
```

Planned (not yet implemented — see Status):

```
tracker create ... [--description "..."]   # Layer 6
tracker show <id>                          # Layer 6
tracker delete <id>                        # Layer 6
```

`tracker list` defaults to open issues, sorted by priority (high → medium → low) then ID ascending. The `--label` filter on `list` accepts a single value and matches case-sensitively; on `create` it is repeatable and labels are deduplicated (case-preserved). Run `tracker --help` or `tracker <subcommand> --help` for the full flag reference of currently-implemented commands.

---

## Install

Requires Rust (toolchain pinned to 1.94.1 — see `rust-toolchain.toml`).

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

Integration tests invoke the compiled binary as a subprocess and assert on stdout, stderr, and exit code. Unit tests cover validation (title, label), ID assignment, status/priority/ID parsing, sort ordering, label deduplication, and case-sensitive label matching.

---

## Storage

Issues are stored in `tracker.json` in the directory where you run the command. The file is created automatically on first `tracker create`. To use separate issue lists per project, run `tracker` from the project's root directory.

`tracker.json` is plain JSON — you can inspect it with any text editor. Do not manually set field values outside the valid enum sets (status: `open`, `in-progress`, `done`; priority: `low`, `medium`, `high`) — invalid values are treated as file corruption.

---

## Status

**Layer 4 implementation complete. Layer 5 not started.**

- [x] DESIGN.md — full behavioral specification
- [x] TODO.md — 7-layer development plan with Red Gate test plans
- [x] DECISIONS.md — key design decisions with rationale
- [x] IAR suite — 10 domains reviewed
- [x] Layer 1: Core create + list
- [x] Layer 2: Status flow
- [x] Layer 3: Priority
- [x] Layer 4: Labels
- [ ] Layer 5: Compound filtering
- [ ] Layer 6: Description, show, delete
- [ ] Layer 7: Polish (color, `--help`, error messages)

---

## Project files

| File | Purpose |
|---|---|
| `DESIGN.md` | Full behavioral specification — preconditions, postconditions, error states, edge cases |
| `TODO.md` | Layered development plan with acceptance criteria, manual testing checklists, Red Gate test plans |
| `DECISIONS.md` | Key design decisions with rationale |
| `PROCESS.md` | Layer-by-layer process retrospective — what was built, what was caught by IAR, what was learned |
| `iterative-adversarial-refinement/` | IAR review logs for all 10 active domains |
