# Manual Testing — Layer 2: Tag and Filter

**Layer:** [`TODO.md` § Layer 2 — Tag and filter](../TODO.md#layer-2--tag-and-filter)
**Tested against:** Layer 2 [Phase 2b](../../../vsdd-suite/primers/2b-implementation.md) implementation committed (extends Layer 1 with `bm tag <url> <label>` + `bm list --tag <label>` + forward-only `tags` migration); [Phase 3](../../../vsdd-suite/primers/3-review-session.md) IAR rounds + [Phase 5](../../../vsdd-suite/primers/5-formal-hardening.md) re-runs pending per `TODO.md` § Layer 2 Layer-gate criteria.
**Convention:** Review 74 manual-test split — this file is the per-layer manual-test plan; the corresponding `TODO.md` Layer 2 block points here. Parallel to [`manual-tests/layer-1.md`](layer-1.md).
**Authoring note:** the test plan below exercises every Layer 2 acceptance criterion (AC 5 through AC 13 per `TODO.md`) at the binary surface. The runnable-step standard applies (per primer 1c § Manual testing checklist) — each step names the exact command, clean-state setup where required, and literal expected output where invariant.

**Prerequisite (cross-layer):** Layer 2 inherits the install-verification discipline established in Layer 1. If [`manual-tests/layer-1.md`](layer-1.md) has not been run in this shell session, run Steps 0–1 of [`layer-1.md`](layer-1.md) first — this Layer 2 plan does NOT re-cover Layer 1 manual tests; it builds on the same `cargo install --locked --path . --force --quiet` install + `$BOOKMARK_CLI_DB`-via-`mktemp` fixture pattern.

**Session-state preamble (inherits [Documentation Reviewer Review 1 Finding 13](../vsdd-suite/review-log/2026-05-20-documentation-reviewer.md) Round 2 fix from Layer 1):** Execute these steps in a single uninterrupted shell session — Step 1 exports `BOOKMARK_CLI_DB` via `mktemp` and Steps 2–13 depend on that export plus the working directory established by Step 0. Alternative: set `BOOKMARK_CLI_DB` to a stable absolute path (e.g., `/tmp/bookmark-cli-manual-layer-2-test.json`) before Step 1 so each subsequent step is independent of the prior shell state.

---

## Step 0 — Update the installed binary

Refreshes the installed `bm` to the current source tree per the Layer 1 Step 0 pattern.

```sh
cd vsdd-suite-reference-examples/bookmark-cli-manual
cargo install --locked --path . --force --quiet
which bm
echo "exit: $?"
```

Expected `which bm` behavior: the command MUST exit 0 AND print a path containing `/.cargo/bin/bm`. The textual prefix is shell-dependent; do not assert on the prefix, only on the `/.cargo/bin/bm` suffix substring. The trailing `echo "exit: $?"` line must show `exit: 0`.

Expected exit code:

```
0
```

---

## Step 1 — Initialize the Layer 2 fixture

Sets up an isolated store with three bookmarks added with deliberate `sleep` between each so the newest-first ordering is observable downstream. This fixture is shared by Steps 2–9; Steps 10 / 11 use their own throwaway stores.

```sh
export BOOKMARK_CLI_DB="$(mktemp -d)/bookmarks.json"
bm add https://example.com/rust
sleep 1
bm add https://example.com/go
sleep 1
bm add https://example.com/none
bm list
echo "exit: $?"
```

Expected stdout for `bm list` (literal up to the variable timestamps; exactly three lines, newest-first):

```
<RFC3339-timestamp-of-none>  https://example.com/none
<RFC3339-timestamp-of-go>    https://example.com/go
<RFC3339-timestamp-of-rust>  https://example.com/rust
```

Invariant parts: exactly three lines; URLs appear in the order `https://example.com/none` (newest, added last) → `https://example.com/go` → `https://example.com/rust` (oldest, added first); each line is `<timestamp> <url>` with the timestamp parseable as RFC 3339 UTC; consecutive timestamps differ by ≥ 1 second.

Expected `echo "exit: $?"` (literal):

```
exit: 0
```

---

## Step 2 — `bm tag` happy path against a single matching URL (AC 5)

Exercises [AC 5](../TODO.md#layer-2--tag-and-filter): attaching a label to a matching bookmark produces an idempotent post-state with the label present in that bookmark's `tags` field; exits 0; stdout silent.

```sh
bm tag https://example.com/rust rust
echo "exit: $?"
cat "$BOOKMARK_CLI_DB"
```

Expected stdout for `bm tag` — none (the command is silent on stdout for pipeline-script-ability; the fenced block below is intentionally empty):

```
```

Expected stderr for `bm tag` (literal — Layer 2 Round 1 UX F2 + SE F2 affordance):

```
Tagged 1 bookmark(s).
```

The `Tagged N bookmark(s).` line on stderr names the match count so the multi-match semantic (covered at Step 7-equivalent multi-match scenario; tested in the integration suite at `tests_tag_against_duplicate_url_tags_all_matches`) is discoverable from user behavior. Stdout stays silent so `bm tag <url> <label> | downstream-script` sees no placeholder text.

Expected stdout for `echo` (literal):

```
exit: 0
```

Expected stdout for `cat "$BOOKMARK_CLI_DB"` — a pretty-printed JSON object with three bookmarks; the matching bookmark has `tags: ["rust"]`; the other two have `tags: []`. URLs invariant; timestamps variable. Representative shape per [`DESIGN.md`](../DESIGN.md) § Storage format:

```json
{
  "bookmarks": [
    {
      "url": "https://example.com/rust",
      "timestamp": "<RFC3339-timestamp-of-rust>",
      "tags": [
        "rust"
      ]
    },
    {
      "url": "https://example.com/go",
      "timestamp": "<RFC3339-timestamp-of-go>",
      "tags": []
    },
    {
      "url": "https://example.com/none",
      "timestamp": "<RFC3339-timestamp-of-none>",
      "tags": []
    }
  ]
}
```

Invariant parts: the `https://example.com/rust` record has `tags: ["rust"]` (exactly one entry, the label `rust`); the other two records have `tags: []`. The storage-order (oldest-first) of the `bookmarks` array reflects insertion order per [`DESIGN.md`](../DESIGN.md) § Storage format (note: this is the on-disk order; the `bm list` rendering order is newest-first, opposite direction).

---

## Step 3 — `bm tag` idempotent re-tag (AC 5 idempotence)

Exercises [AC 5](../TODO.md#layer-2--tag-and-filter) idempotence: a second `bm tag` invocation with identical arguments produces an identical post-state — the `tags` array does NOT contain `["rust", "rust"]`.

```sh
bm tag https://example.com/rust rust
echo "exit: $?"
cat "$BOOKMARK_CLI_DB"
```

Expected stdout for `bm tag` — none (silent on stdout for pipeline-script-ability; the fenced block below is intentionally empty):

```
```

Expected stderr for `bm tag` (literal — same `Tagged N bookmark(s).` affordance as Step 2; the second invocation's idempotent no-op does NOT reduce the match count because the URL still matches the same one bookmark):

```
Tagged 1 bookmark(s).
```

Expected stdout for `echo` (literal):

```
exit: 0
```

Expected stdout for `cat "$BOOKMARK_CLI_DB"` — byte-identical to the Step 2 post-state. The matching bookmark's `tags` array is still exactly `["rust"]` (not `["rust", "rust"]`); the file's byte content is identical to the prior step's content (per [`DESIGN.md`](../DESIGN.md) § `bm tag` § Idempotence under repeat invocation: "the second save still writes the file atomically but the file contents are identical").

---

## Step 4 — `bm tag` against an unknown URL (AC 6)

Exercises [AC 6](../TODO.md#layer-2--tag-and-filter): tagging a URL that no bookmark matches exits 1 with the spec-contracted error and does NOT rewrite the store.

```sh
SNAPSHOT_BEFORE=$(cat "$BOOKMARK_CLI_DB")
bm tag https://nonexistent.example anything
echo "exit: $?"
SNAPSHOT_AFTER=$(cat "$BOOKMARK_CLI_DB")
test "$SNAPSHOT_BEFORE" = "$SNAPSHOT_AFTER" && echo "PASS: store unchanged" || echo "FAIL: store mutated"
```

Expected stderr (literal):

```
Error: no bookmark found with URL https://nonexistent.example.
```

Expected stdout for `echo "exit: $?"` (literal):

```
exit: 1
```

Expected stdout for the snapshot-equality `test` (literal):

```
PASS: store unchanged
```

Per [`DESIGN.md`](../DESIGN.md) § `bm tag` failure contract — exit 1 + spec stderr message + no file write.

---

## Step 5 — `bm tag` with empty URL and empty label (AC 7 + AC 8)

Exercises [AC 7](../TODO.md#layer-2--tag-and-filter) (empty URL rejection) and [AC 8](../TODO.md#layer-2--tag-and-filter) (empty label rejection). Both routes share the exit-1 spec-contracted error pattern.

```sh
bm tag "" rust
echo "exit-empty-url: $?"
bm tag https://example.com/rust ""
echo "exit-empty-label: $?"
```

Expected stderr for the empty-URL `bm tag` (literal):

```
Error: URL cannot be empty.
```

Expected stdout for `echo "exit-empty-url: $?"` (literal):

```
exit-empty-url: 1
```

Expected stderr for the empty-label `bm tag` (literal):

```
Error: tag label cannot be empty.
```

Expected stdout for `echo "exit-empty-label: $?"` (literal):

```
exit-empty-label: 1
```

The empty-URL error string matches the `bm add` empty-URL message exactly (same input invariant per [`DESIGN.md`](../DESIGN.md) § `bm tag` failure contract).

---

## Step 6 — `bm list --tag` happy path (AC 9)

Sets up the second matching tag (`go`), then exercises [AC 9](../TODO.md#layer-2--tag-and-filter): `bm list --tag <label>` filters to bookmarks whose `tags` field contains `<label>`, in newest-first ordering; exit 0.

```sh
bm tag https://example.com/go go
bm list --tag rust
echo "exit: $?"
```

Expected stderr for `bm tag https://example.com/go go` (literal — Layer 2 Round 1 UX F2 + SE F2 affordance):

```
Tagged 1 bookmark(s).
```

Expected stdout for `bm list --tag rust` (literal up to the variable timestamp; exactly one line — only the `rust`-tagged bookmark matches):

```
<RFC3339-timestamp-of-rust> https://example.com/rust
```

Expected stderr for `bm list --tag rust` — none (the fenced block below is intentionally empty; the `bm tag` stderr line above is from the preceding command, not from `bm list`):

```
```

Expected `echo "exit: $?"` (literal):

```
exit: 0
```

---

## Step 7 — `bm list --tag` with no matches (AC 9 filter-empty-state)

Exercises [AC 9](../TODO.md#layer-2--tag-and-filter) filter-empty-state: when the filter matches no bookmarks but the store is non-empty, stderr emits the filter-empty message + exit 0 + stdout silent. (Distinct from the store-empty-state `No bookmarks yet.` per [`DESIGN.md`](../DESIGN.md) § Edge case catalog Layer 2.)

```sh
bm list --tag nonexistent
echo "exit: $?"
```

Expected stderr (literal):

```
No bookmarks match the supplied filter.
```

Expected stdout — none (the fenced block below is intentionally empty; the filter-empty-state routes to stderr so pipe-consumers don't see placeholder text — same routing convention as the Layer 1 store-empty-state):

```
```

Expected `echo "exit: $?"` (literal):

```
exit: 0
```

---

## Step 8 — `bm list --tag <a> --tag <b>` OR-semantics (AC 10)

Exercises [AC 10](../TODO.md#layer-2--tag-and-filter): repeated `--tag` flags compose as OR (union of bookmarks matching either label). Newest-first ordering preserved.

```sh
bm list --tag rust --tag go
echo "exit: $?"
```

Expected stdout (literal up to the variable timestamps; exactly two lines, newest-first — `https://example.com/go` was tagged second so it has the later tag-write but its `add` timestamp is earlier than the `none` bookmark; relative ordering of `go` vs. `rust` matches their `add` timestamps):

```
<RFC3339-timestamp-of-go>   https://example.com/go
<RFC3339-timestamp-of-rust> https://example.com/rust
```

Expected stderr — none (the fenced block below is intentionally empty):

```
```

Expected `echo "exit: $?"` (literal):

```
exit: 0
```

Invariant: exactly two lines; `https://example.com/go` appears FIRST (newest of the two tagged bookmarks); `https://example.com/rust` appears SECOND; the un-tagged `https://example.com/none` bookmark does NOT appear in the output.

---

## Step 9 — `bm list --tag ""` empty label rejected (AC 11)

Exercises [AC 11](../TODO.md#layer-2--tag-and-filter): the empty-label invariant on `bm list --tag` parallels the empty-label rule on `bm tag` (same spec-contracted message + exit code).

```sh
bm list --tag ""
echo "exit: $?"
```

Expected stderr (literal):

```
Error: tag label cannot be empty.
```

Expected stdout — none (the fenced block below is intentionally empty):

```
```

Expected `echo "exit: $?"` (literal):

```
exit: 1
```

---

## Step 10 — Forward-only migration: Layer-1-format store → Layer-2-format (AC 12)

Exercises [AC 12](../TODO.md#layer-2--tag-and-filter): writing a Layer-1-format JSON file directly (no `tags` field per bookmark), invoking `bm tag`, observing that the post-save file has explicit `tags` on every bookmark (touched and untouched alike). Per [`DESIGN.md`](../DESIGN.md) § Edge case catalog Layer 2 + § Storage format `tags` field — "Layer 1 files become Layer 2 files on first Layer 2 write."

```sh
LAYER1_DB="$(dirname "$BOOKMARK_CLI_DB")/layer1.json"
cat > "$LAYER1_DB" <<'EOF'
{
  "bookmarks": [
    {
      "url": "https://layer1.example",
      "timestamp": "2026-05-19T03:00:00Z"
    }
  ]
}
EOF
echo "--- PRE-MIGRATION ---"
cat "$LAYER1_DB"
BOOKMARK_CLI_DB="$LAYER1_DB" bm tag https://layer1.example migrated
echo "tag-exit: $?"
echo "--- POST-MIGRATION ---"
cat "$LAYER1_DB"
rm -f "$LAYER1_DB"
```

Expected stdout for `--- PRE-MIGRATION ---` block (literal):

```
--- PRE-MIGRATION ---
{
  "bookmarks": [
    {
      "url": "https://layer1.example",
      "timestamp": "2026-05-19T03:00:00Z"
    }
  ]
}
```

Expected stderr for `BOOKMARK_CLI_DB="$LAYER1_DB" bm tag https://layer1.example migrated` (literal — Layer 2 Round 1 UX F2 + SE F2 affordance; the Layer-1-format file had one bookmark matching the URL):

```
Tagged 1 bookmark(s).
```

Expected stdout for `echo "tag-exit: $?"` (literal):

```
tag-exit: 0
```

Expected stdout for `--- POST-MIGRATION ---` block (literal — the timestamp is preserved byte-for-byte from the input; `tags: ["migrated"]` is appended):

```
--- POST-MIGRATION ---
{
  "bookmarks": [
    {
      "url": "https://layer1.example",
      "timestamp": "2026-05-19T03:00:00Z",
      "tags": [
        "migrated"
      ]
    }
  ]
}
```

Invariant parts: the post-migration file contains an explicit `tags` field on the migrated bookmark with exactly one entry (`migrated`); per [`DESIGN.md`](../DESIGN.md) § Storage format `tags` field, every bookmark in a Layer-2 write has the explicit field — if the file had contained additional untagged bookmarks, they would all emit explicit `tags: []`.

---

## Step 11 — Store-empty-state takes precedence over filter-empty-state

Exercises [`DESIGN.md`](../DESIGN.md) § Edge case catalog Layer 2 — "`bm list --tag <label>` against an empty store: the empty-store empty-state (`No bookmarks yet.`) takes precedence over the no-filter-match empty-state (`No bookmarks match the supplied filter.`)." The user with zero bookmarks gets the more informative empty-store signal even with `--tag` supplied.

```sh
EMPTY_DB="$(dirname "$BOOKMARK_CLI_DB")/empty.json"
rm -f "$EMPTY_DB"
BOOKMARK_CLI_DB="$EMPTY_DB" bm list --tag rust
echo "exit: $?"
rm -f "$EMPTY_DB"
```

Expected stderr (literal — the empty-store message, NOT the filter-empty message):

```
No bookmarks yet.
```

Expected stdout — none (the fenced block below is intentionally empty):

```
```

Expected `echo "exit: $?"` (literal):

```
exit: 0
```

The precedence rule is the deliberate UX choice per [`DESIGN.md`](../DESIGN.md) § Edge case catalog Layer 2: a user with no bookmarks at all should not be told "no bookmarks match the supplied filter" — the more informative signal is "you haven't added any bookmarks yet."

---

## Step 12 — Performance budget sanity-check (closes [Performance Engineer Review 1 Finding 2](../vsdd-suite/review-log/2026-05-20-performance-engineer.md) at the per-layer manual-test surface)

Exercises the [`DESIGN.md`](../DESIGN.md) § Performance budget contract at the 1,000-bookmark cliff. This is the proportionate Layer 2 closure of [Performance Engineer Review 1 Finding 2](../vsdd-suite/review-log/2026-05-20-performance-engineer.md) (declared **Deferred-to-Layer-2** at Layer 1 Round 2); the in-CI [`tests/scaling.rs`](../tests/scaling.rs) `#[ignore]`-gated sentinels at 100/1,000/10,000 cliffs close [Finding 5](../vsdd-suite/review-log/2026-05-20-performance-engineer.md) separately.

**NEW prerequisite for this step (does NOT exist in [`layer-1.md`](layer-1.md)):** [`hyperfine`](https://github.com/sharkdp/hyperfine). Install via:

- macOS: `brew install hyperfine`
- Debian-derived Linux: `apt install hyperfine`
- Cargo fallback (any platform): `cargo install hyperfine --locked`

If `hyperfine` is not available and the operator wants to skip the benchmark sub-section, that is acceptable (parallel to [`layer-1.md`](layer-1.md) Step 6's "macOS / Linux only" platform-gated skip pattern) — but the budget table from [`DESIGN.md`](../DESIGN.md) § Performance budget remains the contract; running the sanity-check is the layer-2 closure of the deferred PE finding. A coarser fallback (`time bm list` etc. via the shell builtin) is documented at the end of this Step for sandbox / restricted environments where `hyperfine` install is impractical.

### Step 12a — Generate a 1,000-bookmark store

The fastest path is a one-shot Python emit (avoids 1,000 round-trip `bm add` invocations, each of which would itself be ~10 ms × 1,000 = ~10 s and dominated by atomic-save overhead irrelevant to the read-side benchmark):

```sh
BENCH_DB="$(dirname "$BOOKMARK_CLI_DB")/bench-1000.json"
python3 -c "
import json, datetime
now = datetime.datetime(2026, 5, 21, 0, 0, 0, tzinfo=datetime.timezone.utc)
bookmarks = []
for i in range(1000):
    ts = (now - datetime.timedelta(seconds=1000 - i)).isoformat().replace('+00:00', 'Z')
    tags = ['rust'] if i % 3 == 0 else (['go'] if i % 3 == 1 else [])
    bookmarks.append({'url': f'https://example-{i}.com', 'timestamp': ts, 'tags': tags})
print(json.dumps({'bookmarks': bookmarks}, indent=2))
" > "$BENCH_DB"
chmod 600 "$BENCH_DB"
echo "Generated $(wc -l < "$BENCH_DB") lines in $BENCH_DB"
```

Expected stdout (shape, modulo the variable `<line-count>` and `<path>` placeholders):

```
Generated <line-count> lines in <path>/bench-1000.json
```

The exact line count depends on the pretty-printed JSON shape (single-element tag arrays span three lines when emitted via `json.dumps(indent=2)`, while empty tag arrays span one line). On the generator above, where every third bookmark has one tag (`rust` or `go`) and one third are un-tagged, the observed count is around 6,338 lines. The invariant is that the file was created with one bookmark per `https://example-N.com` for `N` in `[0, 1000)` and `chmod 600` succeeded; any line count between 4,000 and 10,000 is acceptable evidence the 1,000-bookmark fixture is populated.

### Step 12b — Run hyperfine against the three named operations

```sh
export BOOKMARK_CLI_DB="$BENCH_DB"
hyperfine --warmup 3 --runs 10 \
    'bm list' \
    'bm list --tag rust' \
    'bm tag https://example-500.com benchmarked'
echo "exit: $?"
```

Expected output: `hyperfine` emits its own formatted report with mean / std-dev / min / max per command. The pass/fail criterion is qualitative — assert that each command's mean (hyperfine does not emit p95 in its default output; mean is the proportionate proxy at small N) falls under the budget from [`DESIGN.md`](../DESIGN.md) § Performance budget:

| Operation | Budget (p95 per [`DESIGN.md`](../DESIGN.md)) | Pass criterion (mean at N=10) |
|---|---|---|
| `bm list` (1,000-bookmark store) | < 100 ms | mean < 100 ms |
| `bm list --tag rust` (1,000-bookmark store) | < 100 ms | mean < 100 ms |
| `bm tag <url> <label>` (1,000-bookmark store) | < 100 ms | mean < 100 ms |

The `bm --help` / `bm --version` < 50 ms startup budget is parallel and may be sanity-checked with `hyperfine --warmup 3 --runs 10 'bm --help' 'bm --version'` if desired but is NOT required at the Layer 2 surface.

Expected `echo "exit: $?"` (literal):

```
exit: 0
```

### Step 12c — Cleanup the benchmark store

```sh
rm -f "$BENCH_DB"
unset BENCH_DB
```

No expected output.

### Fallback: `time` builtin (no-hyperfine environments)

If `hyperfine` cannot be installed (e.g., locked-down CI sandbox), the POSIX `time` builtin provides a coarser single-run measurement that is still adequate to detect catastrophic regression (anything an order of magnitude over budget). Run each operation under `time`; assert each `real` wall-clock figure falls under the budget:

```sh
time bm list > /dev/null
time bm list --tag rust > /dev/null
time bm tag https://example-500.com benchmarked
```

Expected behavior: each `time` invocation prints a `real <Ns>` line on stderr. The pass criterion is the same: `real` < 0.100 s for each of the three operations on the 1,000-bookmark store. This is a single-sample measurement (no warmup, no statistical averaging) — if a single run is close to budget, prefer the `hyperfine` path. Document the chosen path (`hyperfine` vs. `time`) in the Phase 3 Performance Engineer Layer 2 review log when closing the deferred finding.

---

## Step 13 — Cleanup

```sh
rm -rf "$(dirname "$BOOKMARK_CLI_DB")"
unset BOOKMARK_CLI_DB
```

No expected output; cleanup is best-effort.

---

## Closure protocol per session

Per primer 3 § Manual testing is a second adversarial surface to IAR ([G-132](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-132)) — same convention as [`layer-1.md`](layer-1.md) § Closure protocol per session. Every Layer 2 manual-test session closes with one of:

- **Insight-reached / no findings** — all 13 steps reached expected outputs; record the session timestamp + the `Tested against:` field above + a one-line "passed clean" note. No per-domain review-log entry needed.
- **Findings surfaced** — any step diverged from expected; record each divergence as a finding in the per-domain review log (the natural-pair domain — typically QE for test-discipline issues; [UX](../../../vsdd-suite/domains/role/UX-REVIEW.md) for output-quality concerns; SE for binary-behavior bugs; [Performance Engineer](../../../vsdd-suite/domains/role/PERFORMANCE-ENGINEER-REVIEW.md) for Step 12 budget violations) with `**Source:** director-raised` per the per-review-preamble standard ([G-133](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-133)), the divergence cited file:line + the expected vs. observed output, and the appropriate `**Owner:**` per Review 77 lifecycle.

## What comes next

After this manual-test plan runs clean, the Layer 2 layer-gate criteria sequence (per [`TODO.md` § Layer 2 — Tag and filter](../TODO.md#layer-2--tag-and-filter) § Layer-gate criteria) proceeds to:

1. [Phase 3](../../../vsdd-suite/primers/3-review-session.md) IAR rounds for the 13-domain capstone-active set — per-domain review logs file under [`../vsdd-suite/review-log/`](../vsdd-suite/review-log/).
2. [Phase 5](../../../vsdd-suite/primers/5-formal-hardening.md) Layer 2 re-runs — Purity Boundary Audit against the extended pure surface (`filter_by_tags` + `attach_tag`); Mutation Testing against the extended impl; proptest activation against the tag-idempotence + filter-OR-monotonicity properties.
3. [Phase 6](../../../vsdd-suite/primers/6-convergence.md) four-dimensional convergence attestation as the final VDD-IAR Alignment review round titled "Review N — Phase 6 four-dimensional convergence (project-terminal Layer 2)" per primer 6 + [G-177](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-177).
