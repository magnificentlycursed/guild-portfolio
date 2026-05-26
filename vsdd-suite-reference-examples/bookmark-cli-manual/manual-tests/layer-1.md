<!-- hook-bypass[check-suite-internal-terminology]: pre-existing bare abbreviation use (IAR/VSDD/MVR) preserved per the forward-only narrative-preservation policy. These files predate the R95 F1 abbreviation-first-use-expansion check; flagging would require retroactive prose rewriting that crosses the forward-only carve-out. Future authoring SHOULD spell out abbreviations on first mention; the bypass-mechanism is itself a finding for the next registry-walk review. -->
# Manual Testing — Layer 1: Add and List

**Layer:** [`TODO.md` § Layer 1 — Add and List](../TODO.md#layer-1--add-and-list)
**Tested against:** Layer 1 close ([[Phase 2b](../../../vsdd-suite/primers/2b-implementation.md) implementation](../../../vsdd-suite/primers/2b-implementation.md) committed; [Phase 3](../../../vsdd-suite/primers/3-review-session.md) IAR Reviews QE 1 + 2 + SA 1 closed; [Phase 5](../../../vsdd-suite/primers/5-formal-hardening.md) Purity Boundary Audit + Mutation Testing closed at 2026-05-20).
**Convention:** Review 74 manual-test split — this file is the per-layer manual-test plan; the corresponding `TODO.md` Layer 1 block points here.
**Authoring note:** the test plan below was inline in `TODO.md` prior to PR 6 (under portfolio intent); split out per the Review 74 forward-only convention as part of bookmark-cli-manual's capstone-intent promotion. The runnable-step standard applies (per primer 1c § Manual testing checklist) — each step names the exact command, clean-state setup where required, binary lifecycle steps, and literal expected output where invariant.

**Session-state preamble ([Documentation Reviewer Review 1 Finding 13](../vsdd-suite/review-log/2026-05-20-documentation-reviewer.md) Round 2 fix):** Execute these steps in a single uninterrupted shell session — Step 1 exports `BOOKMARK_CLI_DB` via `mktemp`, and Steps 2-6 depend on that export plus the working directory established by Step 0. Alternative: set `BOOKMARK_CLI_DB` to a stable absolute path (e.g., `/tmp/bookmark-cli-manual-test.json`) before Step 1 so each subsequent step is independent of the prior shell state.

---

## Step 0 — Update the installed binary

```sh
cd vsdd-suite-reference-examples/bookmark-cli-manual
cargo install --locked --path . --force --quiet
which bm
echo "exit: $?"
```

Expected `which bm` behavior: the command MUST exit 0 AND print a path containing `/.cargo/bin/bm` (the absolute path under the user's cargo home — note `which` does NOT tilde-expand, so the literal output is something like `/Users/<you>/.cargo/bin/bm` on macOS or `/home/<you>/.cargo/bin/bm` on Linux). The textual prefix is shell-dependent; do not assert on the prefix, only on the `/.cargo/bin/bm` suffix substring. The trailing `echo "exit: $?"` line (mirroring the Step 1/3/4/5/6 convention) must show `exit: 0`.

(On macOS / Linux the cargo-home default is `~/.cargo/bin`. If `which bm` returns a path ending in `/.cargo/bin/bm` the install is current; any other path indicates the installed binary is shadowed by an older build elsewhere on PATH.)

Expected exit code:

```
0
```

---

## Step 1 — Happy path: `bm add <url>` captures a bookmark

```sh
export BOOKMARK_CLI_DB="$(mktemp -d)/bookmarks.json"
bm add https://example.com
echo "exit: $?"
cat "$BOOKMARK_CLI_DB"
```

Expected stdout for `bm add` — none (the command is silent on success; the fenced block below is intentionally empty):

```
```

Expected stderr for `bm add` — none (the command emits nothing to stderr on success; the fenced block below is intentionally empty):

```
```

Expected stdout for `echo` (literal):

```
exit: 0
```

Expected stdout for `cat "$BOOKMARK_CLI_DB"` — a pretty-printed JSON object with one bookmark in the `bookmarks` array per [`DESIGN.md`](../DESIGN.md) § Storage format. The URL is invariant; the timestamp is variable. Representative literal example (pretty-printed multi-line shape that `serde_json::to_string_pretty` emits):

```json
{
  "bookmarks": [
    {
      "url": "https://example.com",
      "timestamp": "2026-05-20T22:15:42.371Z"
    }
  ]
}
```

Invariant parts: the document is a JSON object with a `bookmarks` field whose value is an array; the array has exactly one object; the object has fields `url` (value `"https://example.com"`) and `timestamp` (value parseable as RFC 3339 / ISO 8601 UTC).

---

## Step 2 — Error state: empty URL rejected

```sh
bm add ""
echo "exit: $?"
ls "$BOOKMARK_CLI_DB" 2>&1
```

Expected stderr (literal):

```
Error: URL cannot be empty.
```

Expected stdout for the `echo` line (literal):

```
exit: 1
```

Expected on-disk state: the file from Step 1 still exists with the one bookmark; the empty-URL invocation did NOT modify the store. `ls "$BOOKMARK_CLI_DB"` should succeed with no error.

---

## Step 3 — `bm list` orders newest-first

```sh
# Add a second bookmark with a deliberate sleep so timestamps differ
sleep 1
bm add https://second.example
bm list
echo "exit: $?"
```

Expected stdout for `bm list` (literal up to the variable timestamps):

```
<RFC3339-timestamp-of-second> https://second.example
<RFC3339-timestamp-of-first>  https://example.com
```

Invariant parts: exactly two lines; `https://second.example` appears FIRST (newest); `https://example.com` appears SECOND; both lines are `<timestamp> <url>` format with the timestamp parseable as RFC 3339 UTC; the two timestamps differ by ≥ 1 second.

Expected `echo "exit: $?"` (literal):

```
exit: 0
```

---

## Step 4 — Empty-state: `bm list` against an absent store

```sh
rm "$BOOKMARK_CLI_DB"
bm list
echo "exit: $?"
```

Expected stderr (literal):

```
No bookmarks yet.
```

Expected stdout for `bm list` — none (the empty-state message routes to stderr so pipe-consumers like `bm list | grep ...` don't see placeholder text; the fenced block below is intentionally empty):

```
```

Expected `echo "exit: $?"` (literal):

```
exit: 0
```

The empty-state message routes to stderr (not stdout) so that pipe-consumers (e.g., `bm list | grep example.com`) don't see the placeholder text.

---

## Step 5 — Persistence: install → create → uninstall → reinstall → verify data survives

```sh
# Capture a third bookmark
bm add https://persistence-test.example

# Snapshot the DB content
DBSNAPSHOT=$(cat "$BOOKMARK_CLI_DB")

# Record the project directory as an absolute path before uninstall
# (so the reinstall step does not depend on session cwd).
PROJECT_DIR="$(pwd)"

# Uninstall the binary
cargo uninstall bookmark-cli --quiet

# Confirm `bm` is gone
which bm 2>&1
echo "uninstall-exit: $?"

# Reinstall — use the absolute path captured above
cd "$PROJECT_DIR"
cargo install --locked --path . --force --quiet

# Verify data survives
DBPOSTREINSTALL=$(cat "$BOOKMARK_CLI_DB")
test "$DBSNAPSHOT" = "$DBPOSTREINSTALL" && echo "PASS: persistence" || echo "FAIL: persistence"
```

Expected behavior for `which bm 2>&1` after uninstall: the `which bm` command MUST exit non-zero AND MUST NOT print a path. The exact textual output is shell-dependent (bash: typically empty stdout/stderr + exit 1; zsh: may print `bm not found` + exit 1; BSD `which` on macOS: typically empty + exit 1); do not assert on it.

Expected `echo "uninstall-exit: $?"` (literal):

```
uninstall-exit: 1
```

Expected output for the persistence-check `test` (literal):

```
PASS: persistence
```

The DB content before and after reinstall must be byte-identical — the binary swap does not touch user data.

---

## Step 6 — Verify file mode 0600 on Unix

Per [`DESIGN.md`](../DESIGN.md) § Storage data classification — the captured bookmarks are *confidential*-class data and the storage file is written with mode 0600 (owner read/write only) on Unix. Verify the on-disk file permissions.

```sh
# (macOS / Linux only — file-permission semantics differ on Windows and the project declares Windows untested):
# Pick the appropriate stat invocation for the platform.
if [ "$(uname)" = "Darwin" ]; then
  stat -f %A "$BOOKMARK_CLI_DB"
else
  stat -c %a "$BOOKMARK_CLI_DB"
fi
```

Expected stdout (literal):

```
600
```

Invariant: the storage file's mode is exactly `600` — readable and writable by the owning user and inaccessible to group or world. A divergence (e.g., `644`) is a security defect — the implementation MUST set mode 0600 at save time per the spec's confidential-class data classification.

---

## Cleanup

```sh
rm -rf "$(dirname "$BOOKMARK_CLI_DB")"
unset BOOKMARK_CLI_DB
cargo uninstall bookmark-cli --quiet
```

No expected output; cleanup is best-effort.

---

## Closure protocol per session

Per primer 3 § Manual testing is a second adversarial surface to IAR ([G-132](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-132)): every manual-test session closes with one of:

- **Insight-reached / no findings** — all 6 steps reached expected outputs; record the session timestamp + the `Tested against:` field above + a one-line "passed clean" note. No per-domain review-log entry needed.
- **Findings surfaced** — any step diverged from expected; record each divergence as a finding in the per-domain review log (the natural-pair domain — typically QE for test-discipline issues; [UX](../../../vsdd-suite/domains/role/UX-REVIEW.md) for output-quality concerns; SE for binary-behavior bugs) with `**Source:** director-raised` per the per-review-preamble standard ([G-133](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-133)), the divergence cited file:line + the expected vs. observed output, and the appropriate `**Owner:**` per Review 77 lifecycle.
