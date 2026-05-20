# Manual Testing — Layer 1: Add and List

**Layer:** [`TODO.md` § Layer 1 — Add and List](../TODO.md#layer-1--add-and-list)
**Tested against:** Layer 1 close (Phase 2b implementation committed; Phase 3 IAR Reviews QE 1 + 2 + SA 1 closed; Phase 5 purity-boundary verification + mutation testing closed at 2026-05-20).
**Convention:** Review 74 manual-test split — this file is the per-layer manual-test plan; the corresponding `TODO.md` Layer 1 block points here.
**Authoring note:** the test plan below was inline in `TODO.md` prior to PR 6 (under portfolio intent); split out per the Review 74 forward-only convention as part of bookmark-cli-manual's capstone-intent promotion. The runnable-step standard applies (per primer 1c § Manual testing checklist) — each step names the exact command, clean-state setup where required, binary lifecycle steps, and literal expected output where invariant.

---

## Step 0 — Update the installed binary

```sh
cd vsdd-suite-reference-examples/bookmark-cli-manual
cargo install --path . --force --quiet
which bm
```

Expected `which bm` output (literal — path varies by user; the invariant is that the binary resolves under the user's cargo home):

```
~/.cargo/bin/bm
```

(On macOS / Linux the cargo-home default is `~/.cargo/bin`. If `which bm` returns a path under `~/.cargo/bin/` the install is current; any other path indicates the installed binary is shadowed by an older build elsewhere on PATH.)

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

Expected stdout for `bm add` (literal — empty):

```
```

Expected stderr for `bm add` (literal — empty):

```
```

Expected stdout for `echo` (literal):

```
exit: 0
```

Expected stdout for `cat "$BOOKMARK_CLI_DB"` — a JSON document with one bookmark object. The URL is invariant; the timestamp is variable. Representative literal example:

```json
[{"url":"https://example.com","timestamp":"2026-05-20T22:15:42.371Z"}]
```

Invariant parts: the document is a JSON array; the array has exactly one object; the object has fields `url` (value `"https://example.com"`) and `timestamp` (value parseable as RFC 3339 / ISO 8601 UTC).

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

Expected stdout for `bm list` (literal — empty):

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

# Uninstall the binary
cargo uninstall bookmark-cli --quiet

# Confirm `bm` is gone
which bm 2>&1
echo "uninstall-exit: $?"

# Reinstall
cd vsdd-suite-reference-examples/bookmark-cli-manual
cargo install --path . --force --quiet

# Verify data survives
DBPOSTREINSTALL=$(cat "$BOOKMARK_CLI_DB")
test "$DBSNAPSHOT" = "$DBPOSTREINSTALL" && echo "PASS: persistence" || echo "FAIL: persistence"
```

Expected output for `which bm 2>&1` after uninstall (literal — path may differ slightly by shell):

```
bm not found
```

(Exit code 1 from `which`; the binary is genuinely gone after uninstall.)

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

## Cleanup

```sh
rm -rf "$(dirname "$BOOKMARK_CLI_DB")"
unset BOOKMARK_CLI_DB
cargo uninstall bookmark-cli --quiet
```

No expected output; cleanup is best-effort.

---

## Closure protocol per session

Per primer 3 § Manual testing is a second adversarial surface to IAR (G-132): every manual-test session closes with one of:

- **Insight-reached / no findings** — all 5 steps reached expected outputs; record the session timestamp + the `Tested against:` field above + a one-line "passed clean" note. No per-domain review-log entry needed.
- **Findings surfaced** — any step diverged from expected; record each divergence as a finding in the per-domain review log (the natural-pair domain — typically QE for test-discipline issues; UX for output-quality concerns; SE for binary-behavior bugs) with `**Source:** director-raised` per the per-review-preamble standard (G-133), the divergence cited file:line + the expected vs. observed output, and the appropriate `**Owner:**` per Review 77 lifecycle.

Sycophancy-compensation reminder: a 16-minute closure window with per-item specificity is the discipline working; a 16-minute closure with no per-item observed-vs-expected notes is the kind of finding a manager would flag in an audit (per TW Dim 11 G-132).
