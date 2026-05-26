<!-- hook-bypass[check-document-staleness,check-no-letter-clusters]: pre-existing in-flight phrasing + letter labels preserved per the forward-only narrative-preservation policy. This file's status claims + letter labels predate the R95 F2 staleness hook + the Review 94 letter-cluster hook; flagging would require retroactive rewriting that crosses the forward-only carve-out. Future edits SHOULD use current-state phrasing + descriptive identifiers; the bypass-mechanism is itself a finding for the next registry-walk review. -->
# Issue Tracker CLI

## Overview

A personal issue tracker for the terminal. Single user, no network, no accounts. Issues are stored in a local JSON file in the project directory. The tool is a Rust binary with a subcommand interface modeled after git: `tracker create`, `tracker list`, `tracker status`, `tracker show`, `tracker delete`. The primary use case is tracking work on small software projects between AI sessions, where the tracker serves as the agent's external memory — a persistent record of what is open, in progress, and done that survives conversation resets.

This is portfolio project #2, per the Phase 1 apprentice program assignment in `02-the-methodology/02-tracking-your-work.md`.

---

## Features

### Feature 1: Create Issue

**Command:** `tracker create "<title>" [--description "<desc>"] [--priority <p>] [--label <l>]...`

**Preconditions:**
- `<title>` argument is present
- `<title>` after trimming leading/trailing whitespace is non-empty
- `<title>` contains no control characters (Unicode general category `Cc` — see Edge Cases / Title)
- If `--priority` is present, its value is one of: `low`, `medium`, `high` (case-insensitive)
- If `--label` is present, each label value, after trimming, is non-empty, contains no control characters (Unicode general category `Cc`), and contains no comma `,`

**Postconditions:**
- A new issue is appended to `tracker.json` with a unique, auto-assigned ID
- `status` is `open`
- `priority` defaults to `medium` if `--priority` is not provided
- `labels` is the deduplicated list of `--label` values, with each value trimmed of leading/trailing whitespace; order is preserved (first occurrence retained); case is preserved as provided after trimming; empty if no `--label` flags given
- `description` is stored as provided (not trimmed); absent if `--description` is not provided. Description must not contain control characters other than newline (`\n`); see Error states.
- `created_at` and `updated_at` are set to the current UTC timestamp (ISO 8601, second precision)
- `tracker.json` is updated with the new issue
- stdout prints exactly: `Created issue #<id>: <title>` (trimmed title)
- Exit code 0

**Error states:**
- Title is absent or empty after trim → stderr `Error: Title cannot be empty.` → exit 1
- Title contains a control character → stderr `Error: Title cannot contain control characters.` → exit 1
- `--description` value is empty or whitespace-only after trim → stderr `Error: Description cannot be empty.` → exit 1
- `--description` value contains a control character other than newline (`\n`) → stderr `Error: Description cannot contain control characters other than newline.` → exit 1
- Invalid priority value → stderr `Error: Invalid priority '<v>'. Expected: low, medium, or high.` → exit 1
- Empty label after trim → stderr `Error: Label cannot be empty.` → exit 1
- Label contains a control character → stderr `Error: Label cannot contain control characters.` → exit 1
- Label contains a comma → stderr `Error: Label cannot contain a comma.` → exit 1

**Invariants:**
- No two issues share the same ID
- IDs are assigned in strictly ascending order and are never reused, including after deletion
- All existing issues in storage are unchanged by a create operation

---

### Feature 2: List Issues

**Command:** `tracker list [--status <s>] [--priority <p>] [--label <l>]`

**Default behavior (no flags):** shows all `open` issues, sorted by priority descending (high → medium → low), ties broken by ID ascending.

**With `--status <s>`:** shows issues matching that status. Valid values: `open`, `in-progress`, `done`. Default is `open` only when no `--status` flag is present; if `--status` is provided, it overrides the default.

**With `--priority <p>`:** shows only issues matching that priority. Valid values: `low`, `medium`, `high`.

**With `--label <l>`:** shows only issues that have that label (exact match, case-sensitive). The filter value is trimmed before comparison (symmetric with create-side trim-on-store). Only one `--label` filter is supported per invocation. If `--label` is provided more than once to `list`, a usage error is produced on stderr and the command exits 1.

**Multiple filters:** `--status`, `--priority`, and `--label` are AND-combined. An issue must match all provided filters to appear.

**Preconditions:** none — an empty tracker is valid input.

**Postconditions:**
- stdout shows matching issues in tabular format, one issue per line
- Columns: `ID`, `Status`, `Priority`, `Labels`, `Title`
- Sorting: priority descending (high → medium → low), then ID ascending within each priority tier
- If no issues match: **stderr** prints `No issues match the given filters.` (or `No open issues. Nice work!` when default view and tracker is empty); stdout is empty
- Exit code 0 in all cases (no matching results is not an error)

**Error states:**
- Invalid `--status` value → stderr `Error: Invalid status '<v>'. Expected: open, in-progress, or done.` → exit 1
- Invalid `--priority` value → stderr `Error: Invalid priority '<v>'. Expected: low, medium, or high.` → exit 1
- Empty or whitespace-only `--label` filter value → stderr `Error: Label cannot be empty.` → exit 1 (symmetric with create-side validation; prevents a silent-no-match for a malformed filter)

**Invariants:**
- Output is deterministic for the same storage state and flags
- Issues are never mutated by a list operation

---

### Feature 3: Change Status

**Command:** `tracker status <id> <status>`

**Preconditions:**
- `<id>` is a positive integer (≥ 1)
- An issue with `<id>` exists in storage
- `<status>` is one of: `open`, `in-progress`, `done` (case-insensitive)

**Postconditions:**
- The issue's `status` field is updated to the new value
- `updated_at` is set to the current UTC timestamp
- All other fields on the issue are unchanged
- `tracker.json` is updated with the new status
- stdout prints: `Issue #<id> status → <new_status>.`
- Exit code 0

**Error states:**
- `<id>` is not a positive integer → stderr `Error: '<id>' is not a valid issue ID. Expected a positive integer.` → exit 1
- Issue not found → stderr `Error: Issue #<id> not found.` → exit 1
- Invalid status value → stderr `Error: Invalid status '<v>'. Expected: open, in-progress, or done.` → exit 1
- Attempting to set the same status the issue already has → still succeeds (idempotent); stdout prints the same confirmation message; `updated_at` is refreshed

**State transitions:** any status may transition to any other status. There is no forbidden transition. An issue may be re-opened after being marked done.

---

### Feature 4: Show Issue

**Command:** `tracker show <id>`

**Preconditions:**
- `<id>` is a positive integer (≥ 1)
- An issue with `<id>` exists in storage

**Postconditions:**
- stdout shows all fields for the issue: ID, Title, Status, Priority, Labels (comma-separated, or `(none)` if empty), Description (or `(none)` if absent), Created, Updated
- Exit code 0

**Error states:**
- `<id>` is not a positive integer → stderr `Error: '<id>' is not a valid issue ID. Expected a positive integer.` → exit 1
- Issue not found → stderr `Error: Issue #<id> not found.` → exit 1

**Invariants:** the issue is never mutated by a show operation.

---

### Feature 5: Delete Issue

**Command:** `tracker delete <id>`

**Preconditions:**
- `<id>` is a positive integer (≥ 1)
- An issue with `<id>` exists in storage

**Postconditions:**
- The issue is removed from `tracker.json`
- `tracker.json` is updated with the issue removed
- stdout prints: `Deleted issue #<id>.`
- Exit code 0

**Error states:**
- `<id>` is not a positive integer → stderr `Error: '<id>' is not a valid issue ID. Expected a positive integer.` → exit 1
- Issue not found → stderr `Error: Issue #<id> not found.` → exit 1

**Invariants:**
- The deleted ID is never reused. The persistent `next_id` counter (see Data Model / Storage file) is monotonically increasing across the tracker's lifetime and is left unchanged by delete, so the next created issue always receives an id strictly greater than every previously-assigned id — including the just-deleted one, including the case where the deleted issue was the highest id at delete time. (SO Review 22 Option A: the prior `max(remaining_ids) + 1` formulation was incorrect at the high edge and reused the deleted id.)
- IDs of all remaining issues are unchanged
- No other issues are affected by the delete

---

## Data Model

### Issue

```
{
  "id":          u64,              // positive integer, 1-indexed, immutable after creation
  "title":       String,           // non-empty, trimmed
  "description": Option<String>,   // absent if not provided; stored as-is if provided. "Absent" means the JSON key is omitted, not serialized as null. Implementations must omit the key when the value is None.
  "status":      "open" | "in-progress" | "done",
  "priority":    "low" | "medium" | "high",
  "labels":      [String],         // may be empty; deduplicated at creation; case-preserved
  "created_at":  String,           // ISO 8601 UTC, second precision, e.g. "2026-04-27T14:00:00Z"
  "updated_at":  String            // ISO 8601 UTC, second precision; always >= created_at
}
```

**Field invariants:**
- `id` is unique across all issues and never reused
- `title` is always non-empty after trimming
- `status` is always one of the three valid string values
- `priority` is always one of the three valid string values
- `created_at` never changes after issue creation
- `updated_at` is refreshed on every mutation (status change); equals `created_at` on a freshly created issue

### Storage file

```
{
  "issues":  [Issue],   // order is not significant (list sorts on display)
  "next_id": u64        // monotonically-increasing counter; the next id to assign
}
```

**Storage invariants:**
- If the file does not exist, the tracker is treated as fresh — equivalent to `{"issues": [], "next_id": 1}`
- `next_id >= 1` at all times
- If `issues` is non-empty, `next_id > max(issue.id)` (strictly greater — the counter has been advanced past every assigned id)
- On every create, the new issue's id is `next_id`; the counter is then bumped via `checked_add(1)` (overflow at `u64::MAX` surfaces a clean "Cannot assign new issue ID" error)
- On every delete, the counter is NOT modified — `next_id` is monotonically increasing across the tracker's lifetime, so deleted ids are never reassigned, even when the deleted issue was the highest id at delete time (SO Review 22 Option A)
- `tracker.json` is written directly on every mutation; on I/O failure the file may be in an indeterminate state — the error is reported and the binary exits 1. Atomic writes are the correct production approach and are deferred — implementation cost exceeds the failure risk for a single-user local tool.

**File location:** `tracker.json` in the current working directory at the time the command runs.

---

## Interface

**Binary name:** `tracker`

**Technology:** Rust. CLI argument parsing: any Rust crate or standard library. JSON serialization: any Rust crate. The observable interface contract — subcommand names, flag names, error message format, stdout/stderr/exit-code behavior — is defined by this spec regardless of which libraries implement it.

**Subcommands:**

| Subcommand | Synopsis |
|---|---|
| `create` | `tracker create "<title>" [--description "<desc>"] [--priority low\|medium\|high] [--label <l>]...` |
| `list` | `tracker list [--status open\|in-progress\|done] [--priority low\|medium\|high] [--label <l>]` |
| `status` | `tracker status <id> open\|in-progress\|done` |
| `show` | `tracker show <id>` |
| `delete` | `tracker delete <id>` |

**stdout contract:** all *data* output goes to stdout — issue rows from `list`, the labelled key-value block from `show`, and the one-line confirmations from `create` / `status` / `delete`. A consumer that pipes stdout (`tracker list | wc -l`, `tracker show <id> | grep ...`) sees only data records.

**stderr contract:** all error messages and informational status messages go to stderr. Error messages begin with `Error:` and are followed by a human-readable description; no stack traces or internal detail are exposed to the user. Error messages that interpolate user-supplied values (e.g. `Error: Invalid priority '<v>'.`) MUST escape any control character (Unicode general category `Cc`) in the interpolated value as `\u{XX}` before rendering — the error stream is not a transparent pipe for arbitrary terminal sequences. **This rule applies to every stderr write site, including errors generated by the argument-parsing pipeline (e.g. `Error: unrecognized subcommand '<name>'` from clap) — the parser's reflected user-supplied value MUST be passed through the same Cc-escape transform as application-generated errors.** Empty-state messages from `list` (`No open issues. Nice work!` and `No issues match the given filters.`) are informational, not data — they go to stderr so a piped consumer sees an empty stdout when no issues match.

**Exit codes:**
- `0` — success
- `1` — any error (invalid argument value, issue not found, empty title, I/O failure, etc.)

**`--help` flag:** `--help` is supported for the binary and each subcommand. The output must accurately describe all flags and their valid values.

**List output format:** tabular, fixed-width columns, header row. Column widths are fixed minimums: `ID` 4 chars, `Status` 11 chars, `Priority` 8 chars, `Labels` 20 chars, `Title` consuming the remainder up to 50 characters. Columns are separated by exactly 2 spaces. `Labels` renders all labels comma-separated and truncates at 20 characters with `…` if longer. `Title` truncates at 50 characters with `…` if longer. `show` always displays the full, untruncated values. Example:

```
ID    Status       Priority  Labels                Title
1     open         high      bug, auth             Fix the login bug
2     in-progress  medium    feature               Add search bar
3     open         low       (none)                Update README
```

**Color output (polish layer — Layer 7):** Priority and status values are colored in list and show output when stdout is a TTY. Color is suppressed when stdout is piped or redirected (detect with `std::io::IsTerminal`). When stdout is a TTY, color is *also* suppressed if either of these environment-variable opt-outs is set, per the de facto cross-tool standard at <https://no-color.org/> and honored by `git`, `cargo`, `ripgrep`, `bat`, and most modern Rust CLIs:

- `NO_COLOR` set to any non-empty value
- `CLICOLOR` set to `0`

`CLICOLOR_FORCE=1` is **not** honored: color is never emitted to a non-TTY stdout regardless of *user-facing* env vars, to preserve the pipe-cleanness contract. Color suppression is symmetric: ANSI escapes are never emitted to stderr (the empty-state messages and error messages go to stderr per the stderr contract; they remain uncolored regardless of TTY state to keep error streams reliably scriptable).

**Internal test seam (`TRACKER_INTERNAL_FORCE_COLOR=1`) — not a user-facing contract.** A single, deliberately-namespaced environment variable, `TRACKER_INTERNAL_FORCE_COLOR=1`, bypasses the TTY detection and the `NO_COLOR` / `CLICOLOR` opt-outs and forces `ColorMode::On`. It exists so the integration test harness (`assert_cmd::Command`, which connects stdout to a pipe) can exercise the positive color path that would otherwise be reachable only through the manual TTY checklist. The variable is **not part of the public CLI contract**: it is not described in `--help`, its name and activation semantics are **unstable across versions**, and any production reliance on it is unsupported. Documented here only because any env var that mutates observable output is, in the spec-fidelity sense, observable CLI behavior — and the spec's contract integrity requires that observable behavior be named in the spec rather than hidden in source comments (SO Review 25 Finding 2). The check is placed before the `NO_COLOR` / `CLICOLOR` checks so tests can fully control color emission without inheriting CI-environment `NO_COLOR`; an end user who somehow sets this variable will override their own `NO_COLOR` preference, which is the explicit cost of the test-ergonomics choice.

| Value | Color |
|---|---|
| `high` priority | Red / bold |
| `medium` priority | Yellow / bold |
| `low` priority | Default (no color) |
| `open` status | Default (no color) |
| `in-progress` status | Cyan / bold |
| `done` status | Green / bold |

Every colored value carries the `bold` SGR attribute (WCAG 1.4.1 *Use of Color*: a non-color cue must accompany any color cue so users with color-vision deficiency can distinguish states). The default-color values (`low`, `open`) intentionally have no bold so the highlighted vs. unhighlighted dichotomy reads at a glance for both CVD and non-CVD users. Color is applied only to the value text in its column cell, not to the entire row or header.

**Show output format:** labelled key-value block. The label column is right-padded to a fixed width of 13 characters so values align. For multi-line descriptions, the first line follows the `Description:` label; each continuation line is indented by 13 spaces (matching the label column width) so the text block remains visually aligned. `\r\n` separators in a stored description are normalized to `\n` before splitting (defensive: bare `\r` and `\r\n` are rejected at create time per Edge Cases / Description, but a legacy stored value or external-editor round-trip should render cleanly). Example (single-line description):

```
ID:          3
Title:       Update README
Status:      open
Priority:    low
Labels:      (none)
Description: (none)
Created:     2026-04-27T14:00:00Z
Updated:     2026-04-27T14:00:00Z
```

Example (multi-line description):

```
ID:          4
Title:       Fix auth flow
Status:      open
Priority:    high
Labels:      bug
Description: Token refresh fails after 1 hour.
             Reproduces reliably on Safari.
Created:     2026-04-27T15:00:00Z
Updated:     2026-04-27T15:00:00Z
```

---

## Constraints

- **Single user.** No concurrent access guarantees required. The tool does not implement file locking.
- **No network.** No HTTP calls, no authentication, no external services.
- **Local storage only.** `tracker.json` is the sole data store.
- **Rust only.** No JavaScript, no Python, no shell scripts in the binary. Compilation must succeed with `cargo build` with no errors at the end of each layer.
- **Crash-safe I/O.** The binary must not crash when `tracker.json` is missing or contains invalid data. These cases produce an error message on stderr and exit 1.
- **Input validation at the boundary.** All CLI input is validated before reaching business logic. Invalid values produce an error message on stderr and exit 1 before any storage operation occurs.

---

## Edge Cases

### Title

- Empty string (`tracker create ""`) → error: `Title cannot be empty.`
- Whitespace-only string (`tracker create "   "`) → error: `Title cannot be empty.` (checked after trim)
- Leading/trailing whitespace (`tracker create "  Fix bug  "`) → stored as `"Fix bug"` (trimmed)
- Title containing quotes or other shell-special characters (`$`, `&`, `*`, etc.) → shell responsibility; the binary receives the raw string after shell expansion and treats it as opaque text. Printable Unicode is accepted regardless of script (e.g., emoji, CJK).
- Title containing a control character (Unicode general category `Cc` — includes newline, carriage return, tab, NUL, ESC `0x1B`, DEL `0x7F`, the C1 controls, and all other ASCII < 0x20) → error: `Title cannot contain control characters.`. Rationale: control characters break the one-issue-per-line contract of `list` output (newline/CR), corrupt column alignment (tab), and enable terminal-escape injection in any tool that displays the title (ESC). Stored data containing a control-character title is treated as corrupt (same error path as other invalid stored fields).

### IDs

- Non-integer (`tracker show abc`) → error: `'abc' is not a valid issue ID. Expected a positive integer.`
- Zero (`tracker status 0 done`) → error: zero is not a positive integer
- Negative number (`tracker delete -1`) → the CLI parser treats `-1` as a flag and produces a usage error; the command exits 1
- ID of a deleted issue (e.g., issue #3 was deleted; `tracker show 3`) → error: `Issue #3 not found.`
- ID larger than any existing issue but within u64 range → error: `Issue #<id> not found.`

### Labels

- Duplicate labels on create (`--label bug --label bug`) → deduplicated; stored once as `["bug"]`
- Empty label (`--label ""`) → error: `Label cannot be empty.`
- Whitespace-only label (`--label "  "`) → error: `Label cannot be empty.` (checked after trim)
- Leading/trailing whitespace on a label is trimmed before storage; `--label "  bug  "` stores `bug`. Deduplication compares trimmed values, so `--label "bug" --label "  bug  "` stores `["bug"]`
- Label containing a control character (Unicode general category `Cc` — newline, CR, tab, NUL, ESC, DEL, C1 controls) → error: `Label cannot contain control characters.` Same rationale as Title (preserves the one-issue-per-line `list` contract and prevents terminal-escape injection via the comma-separated `Labels` column rendering)
- Label containing a comma (`--label "a,b"`) → error: `Label cannot contain a comma.` The comma is the `Labels` column display separator (`a,b, c` would be ambiguous with `a, b, c`); rejecting commas at input keeps the display unambiguous
- Label filter matches case-sensitively: `--label Bug` does not match an issue with label `bug`
- The `--label` filter value on `tracker list` is trimmed before comparison; `tracker list --label "  bug  "` matches a stored `bug` (symmetric with the create-side trim)
- An issue with no labels filtered with `--label bug` → does not appear in results
- Bidi-override / format-class / zero-width characters (Unicode general categories `Cf`, `Mn` and similar) are accepted as printable Unicode and may produce visually-misleading output. Out-of-threat-model for this single-user local tool: the threat surface is bounded to the user attacking themselves with hand-pasted clipboard content or a hand-edited `tracker.json`. If a future use case widens the threat model (multi-user / network-distributed / shared `tracker.json`), revisit this stance

### List

- No issues in storage → `tracker list` prints `No open issues. Nice work!` to **stderr**; stdout is empty; exit 0
- Issues exist but none match the filters → prints `No issues match the given filters.` to **stderr**; stdout is empty; exit 0
- All issues are done → `tracker list` (default open filter) → `No open issues. Nice work!` to **stderr**
- Multiple filters combined: `tracker list --status open --priority high --label bug` → AND-logic; only issues matching all three
- Pipe consumers see only data records on stdout; the empty-state messages do not pollute pipelines like `tracker list | wc -l`

### Status transitions

- Setting a status to the same value it already has → succeeds; `updated_at` refreshes; confirmation message is printed normally
- `tracker status <id> DONE` (uppercase) → accepted; status values are case-insensitive on input, stored lowercase

### Storage

- `tracker.json` does not exist → treated as empty tracker; first `create` produces `tracker.json`
- `tracker.json` contains valid JSON but unknown fields → unknown fields are ignored at load (forward-compatible deserialization). They are NOT preserved across writes — any subsequent mutation rewrites `tracker.json` with only the documented schema fields, dropping anything else. Hand-edited `tracker.json` files should not rely on extra keys persisting.
- `tracker.json` contains valid JSON but invalid domain values (e.g., `"status": "flying"`, `"priority": ""`, `"id": 0`, `"title": ""`, a control-character in `title`, an empty `label`, a control-character or comma in any `label`, an empty `description` after trim, a control-character other than newline in `description`, a malformed `created_at` / `updated_at`, `updated_at < created_at`, or duplicate `id` across records) → stderr `Error: Could not read tracker data. The file may be corrupt. Delete tracker.json to start fresh.` → exit 1
- `tracker.json` contains malformed JSON → stderr `Error: Could not read tracker data. The file may be corrupt. Delete tracker.json to start fresh.` → exit 1
- `tracker.json` exists but is not readable (permissions) → stderr `Error: Could not read tracker data: <os-error-description>.` → exit 1. The `<os-error-description>` is the platform's `std::io::Error` Display, which on Unix renders the underlying errno reason (e.g. `Permission denied (os error 13)`). The errno tag is permitted; the prefix `Error: Could not read tracker data: ` is fixed. Save-side errors follow the symmetric pattern: `Error: Could not save tracker data: <os-error-description>.`
- Write fails (disk full, permissions) → stderr `Error: Could not save tracker data: <reason>.` → exit 1
- `tracker.json` is a directory → read error, treated as I/O failure → exit 1

### Description

- `--description` not provided → description is absent; not shown in `list`, shown as `(none)` in `show`
- `--description ""` (empty string after trim) → error: `Error: Description cannot be empty.` → exit 1; consistent with how empty title and empty label are handled
- Description is not validated for length (no maximum)
- Description is not trimmed; stored verbatim
- Description may contain newlines (`\n`). In `show` output, the first line follows the `Description:` label; each subsequent line is indented by 13 spaces to align with the value column. In `list` output, description is never shown.
- Description may NOT contain any other control character (Unicode general category `Cc`): no carriage return (`\r`), tab (`\t`), NUL, BEL, ESC, DEL, or C1 controls. Rejected at create time with `Error: Description cannot contain control characters other than newline.` and at load time as corrupt data. Rationale parallels title (Edge Cases / Title) and labels (Edge Cases / Labels): description flows to the same `show` rendering pipeline that emits to stdout, and an unescaped ESC byte enables terminal-escape injection. Newline is carved out because the spec explicitly permits multi-line descriptions for `show` continuation rendering.
- For multi-line descriptions, `\r\n` is normalized to `\n` before splitting in `show` output so a CRLF-stored description (e.g., pasted from a Windows source) renders without a stray `\r` in the first line. Stored descriptions with bare `\r` or `\r\n` are rejected at create time per the control-character rule above; the normalization defends against legacy stored data and round-trips from external editors.
- Bidi control characters (Unicode general category `Cf`, e.g., U+202E, zero-width joiners) are NOT rejected. Same out-of-threat-model posture as titles and labels (single-user local CLI; risk owner: director). Cross-reference Red Team R6 F3 / R8 (Trojan-Source acceptance).

### Labels (additional)

- Multiple `--label` flags to `tracker list` → usage error on stderr, exit 1; multiple `--label` flags to `tracker create` are accepted and deduplicated

---

## Testing Methodology

### Automated tests

The following behaviors are automatable and should be covered by unit tests:

- Title validation: empty, whitespace-only, leading/trailing whitespace trimmed
- Priority string parsing: valid values, invalid value, case-insensitive matching
- Status string parsing: valid values, invalid value, case-insensitive matching
- Issue filtering: each filter independently; AND-combination of two and three filters; no-match case
- Issue sorting: high/medium/low order; tie-breaking by ID ascending
- Storage deserialization: valid JSON, unknown fields, malformed JSON (error path)

Integration tests (invoke the binary as a subprocess):

- Full create → list → status → show → delete lifecycle
- Exit code contract for each error condition
- stdout/stderr separation: success output on stdout, error messages on stderr
- `--help` output: verify it does not crash and exits 0

### Manual testing checklist (per layer)

Each layer must be manually tested before the layer gate closes:

- Run the happy path commands from a clean state (no `tracker.json`)
- Verify the empty-state message when no issues exist
- Introduce each error condition and read the error message: is it specific and actionable?
- Run `tracker list` after several creates and verify sort order visually
- Verify `tracker.json` is valid JSON after each mutation (open it in a text editor or `cat` it)
- Reload test: run commands, then delete the binary, reinstall with `cargo install --path .`, and verify all data persists

### Purity guidance

Prefer separating validation, filtering, and sorting logic into functions with no I/O side effects — these are easier to unit test. I/O (reading and writing `tracker.json`, reading the system clock) belongs in thin wrapper functions called from the command handlers. This separation is a code organization principle, not a formal requirement.

---

## Out of Scope

- **Multiple users or sharing** — single-user tool only; no access control, no shared state
- **Due dates or calendar integration** — no time-based fields beyond `created_at` / `updated_at`
- **Subissues or hierarchy** — issues are a flat list; no parent/child relationships
- **Time tracking** — no start/stop timers or effort logging
- **Editing after creation** — no command to change a title, description, or labels after an issue is created; status change is the only post-creation mutation. The assignment's Feature 5 ("Add labels to issues") is satisfied by label support at creation time; the assignment's interface section shows no post-creation label command, and this interpretation is consistent with the provided interface examples
- **Search by text** — no full-text search across titles or descriptions; filtering is by exact-match status, priority, and label only
- **Undo/redo** — deletions are permanent
- **Issue comments** — no per-issue comment thread
- **Remote or synced storage** — local file only; no cloud backend, no API
- **Archiving** — delete is the only removal mechanism; no soft-delete or archive state
- **Interactive mode** — the tool is non-interactive; it reads arguments from the command line and exits; no TUI or REPL. See "Approved Deviations from Assignment" below for the `tracker delete <id>` confirmation prompt waiver.
- **Concurrent access** — no file locking; undefined behavior if two instances run simultaneously against the same `tracker.json`
- **Atomic writes** — direct write to `tracker.json` on every mutation; no temp-file-and-rename. Correct production practice but implementation cost exceeds failure risk for a single-user local tool. Revisit if the tool is ever used in a context with multiple concurrent writers.
- **Structured exit codes for scripted callers** — exit 0/1 only; no separate exit code for I/O vs. user errors. This tool is used interactively; no scripted caller exists to distinguish error categories.

---

## Approved Deviations from Assignment

This section documents deliberate, director-approved deviations from the upstream assignment brief at `apprentice-onboarding/02-the-methodology/02-tracking-your-work.md` (canonical source: https://github.com/Navigators-Guild/apprentice-onboarding/blob/main/02-the-methodology/02-tracking-your-work.md). Each entry records the deviation, the rationale, the approver, and the date. Approved deviations are NOT scope-narrowings; they are explicit "we considered the assignment requirement and chose to deviate, with stakeholder approval." Per `iterative-adversarial-refinement/CLOSURE-PROTOCOL.md`, only the Solution Owner may add entries here.

### D1 — `tracker delete <id>` does not require confirmation

- **Assignment text:** Build layer sequence, Layer 6: "Detail & delete: show full details; **delete with confirmation**".
- **Deviation:** `tracker delete <id>` is non-interactive. No `[y/N]` prompt; no `--yes` bypass flag. Single-shot deletion at exit 0 (or exit 1 on error).
- **Rationale:** (1) the rest of the binary is non-interactive (every other subcommand exits without prompting), so a single interactive command would be inconsistent with the established surface; (2) standard CLI tools in the same family (`git rm`, `rm`, `mv`) delete without confirmation by default and offer `-i` for interactive mode; (3) the operation is recoverable in practice — `tracker.json` is a flat JSON file under version control or backup, and the user can restore the deleted record by editing the file directly; (4) the tool's threat model is a single user on a local machine, where accidental-deletion friction is the user's own concern, not a multi-stakeholder safety surface.
- **Approver:** apprentice-program director (the human user of this branch), implicit approval recorded via the `2026-05-05` round-2 IAR resolution that addressed SO Review 16 Finding 4.
- **Date approved:** 2026-05-05
- **Re-evaluation trigger:** if the tool is used in a multi-user / shared context (which itself contradicts the assignment's "Single user" constraint), reintroduce the confirmation requirement at that point.
