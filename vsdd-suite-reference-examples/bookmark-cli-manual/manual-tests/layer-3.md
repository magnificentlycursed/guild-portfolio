# Manual Testing — Layer 3: Export and Import

**Layer:** [`TODO.md` § Layer 3 — Export and import](../TODO.md#layer-3--export-and-import-ai-co-authored-operator-owned)
**Tested against:** Layer 3 [Phase 2b](../../../vsdd-suite/primers/2b-implementation.md) implementation committed (extends Layer 2 with `bm export` + `bm import` + JSON-native escape design + sorted-tag-comparison dedup + active control-char tag rejection); [Phase 3](../../../vsdd-suite/primers/3-review-session.md) Round 1 closed + Phase 4 routed (`vsdd-suite/review-log/2026-05-24-phase-4-routing.md`); [Phase 5](../../../vsdd-suite/primers/5-formal-hardening.md) re-runs pending per `TODO.md` § Layer 3 Layer-gate criteria.
**Convention:** Review 74 manual-test split — this file is the per-layer manual-test plan; the corresponding `TODO.md` Layer 3 block points here. Parallel to [`manual-tests/layer-1.md`](layer-1.md) + [`manual-tests/layer-2.md`](layer-2.md).
**Authoring note:** The test plan below exercises every Layer 3 acceptance criterion (AC 14 through AC 28 per `TODO.md`) at the binary surface PLUS the Round 1 Phase 4 routed Path-of-implementation closures (byte-preservation round-trip; sorted-tag-comparison dedup; active control-char tag rejection). The runnable-step standard applies (per primer 1c § Manual testing checklist) — each step names the exact command, clean-state setup where required, and literal expected output where invariant.

**Prerequisite (cross-layer):** Layer 3 inherits the install-verification discipline + session-state setup established in Layer 1. If [`manual-tests/layer-1.md`](layer-1.md) has not been run in this shell session, run Steps 0–1 of [`layer-1.md`](layer-1.md) first — this Layer 3 plan does NOT re-cover Layer 1 manual tests; it builds on the same `cargo install --locked --path . --force --quiet` install + `$BOOKMARK_CLI_DB`-via-`mktemp` fixture pattern.

**Session-state preamble (inherits [Documentation Reviewer Review 1 Finding 13](../vsdd-suite/review-log/2026-05-20-documentation-reviewer.md) Round 2 fix from Layer 1):** Execute these steps in a single uninterrupted shell session — Step 1 exports `BOOKMARK_CLI_DB` via `mktemp` and Steps 2–17 depend on that export plus the working directory established by Step 0. Alternative: set `BOOKMARK_CLI_DB` to a stable absolute path before Step 1 so each subsequent step is independent of the prior shell state.

---

## Step 0 — Update the installed binary

Refreshes the installed `bm` to the current source tree per the Layer 1 + Layer 2 Step 0 pattern.

```sh
cd vsdd-suite-reference-examples/bookmark-cli-manual
cargo install --locked --path . --force --quiet
bm --version
```

Expected: `cargo install` succeeds (exit 0; no compiler output unless a warning surfaces); `bm --version` prints the version line from `Cargo.toml`.

---

## Step 1 — Initialize the Layer 3 fixture

```sh
export BOOKMARK_CLI_DB="$(mktemp -d)/bookmarks.json"
echo "Using BOOKMARK_CLI_DB=$BOOKMARK_CLI_DB"

# Seed a small store via the Layer 1 + Layer 2 binary surface.
bm add "https://layer-3-test.example/alpha"
bm add "https://layer-3-test.example/beta"
bm add "https://layer-3-test.example/gamma"
bm tag "https://layer-3-test.example/alpha" rust
bm tag "https://layer-3-test.example/beta" go
bm tag "https://layer-3-test.example/beta" cli
bm list
```

Expected: each `bm add` exits 0 silently; `bm tag` prints `Tagged 1 bookmark.` on stderr per the Layer 2 R2 UX F4 singular form. `bm list` prints three lines newest-first in the format `<RFC3339-timestamp> <url>` (newest-first ordering — gamma + beta + alpha in that vertical order; timestamps differ per the per-`bm-add` clock read).

---

## Step 2 — `bm export` happy path (AC 14)

```sh
bm export
echo "exit: $?"
```

Expected: stdout emits valid JSON matching the storage-format object-wrapped shape `{"bookmarks":[...]}` with all 3 bookmarks in newest-first order; the tagged bookmark records carry their `tags` arrays (alpha → `["rust"]`; beta → `["go","cli"]`; gamma → `[]`). Trailing newline after the JSON. Stderr silent. Exit 0.

Sanity-check the JSON is parseable + the bookmark count is correct:

```sh
bm export | python3 -c "import json, sys; d = json.load(sys.stdin); print(f'bookmarks: {len(d[\"bookmarks\"])}'); print(f'urls: {sorted(b[\"url\"] for b in d[\"bookmarks\"])}')"
```

Expected:

```
bookmarks: 3
urls: ['https://layer-3-test.example/alpha', 'https://layer-3-test.example/beta', 'https://layer-3-test.example/gamma']
```

---

## Step 3 — `bm export` empty-store case (AC 15)

```sh
EMPTY_DB="$(mktemp -d)/empty.json"
BOOKMARK_CLI_DB="$EMPTY_DB" bm export
echo "exit: $?"
```

Expected (literal):

```
{"bookmarks":[]}
exit: 0
```

Stderr silent (no `No bookmarks yet.` message — `bm export` is pipeline-rendering, not human-rendering; the empty-array shape is the legitimate empty-state signal).

```sh
rm -rf "$(dirname "$EMPTY_DB")"
unset EMPTY_DB
```

---

## Step 4 — `bm export --tag <label>` OR-filter (AC 16)

Restore the seeded fixture context (Step 1's `BOOKMARK_CLI_DB` is still exported). Filter to the `rust` + `go` OR-union:

```sh
bm export --tag rust --tag go | python3 -c "import json, sys; d = json.load(sys.stdin); print(f'bookmarks: {len(d[\"bookmarks\"])}'); print(f'urls: {sorted(b[\"url\"] for b in d[\"bookmarks\"])}')"
echo "exit: $?"
```

Expected:

```
bookmarks: 2
urls: ['https://layer-3-test.example/alpha', 'https://layer-3-test.example/beta']
exit: 0
```

Gamma (untagged) excluded; alpha (rust) + beta (go) included per OR-semantics.

---

## Step 5 — `bm export --tag ""` empty label rejected (AC 17)

```sh
bm export --tag "" 2>&1 >/dev/null
echo "exit: $?"
```

Expected (literal):

```
Error: tag label cannot be empty.
exit: 1
```

Same error string as `bm tag ""` + `bm list --tag ""` (cross-layer consistency per UX domain alignment).

---

## Step 6 — `bm import` happy path (AC 19)

Set up an isolated destination + import a one-record payload:

```sh
IMPORT_DB="$(mktemp -d)/import.json"
echo '{"bookmarks":[{"url":"https://imported.example","timestamp":"2026-05-25T03:00:00Z","tags":["rust"]}]}' | BOOKMARK_CLI_DB="$IMPORT_DB" bm import
echo "exit: $?"
BOOKMARK_CLI_DB="$IMPORT_DB" bm list
```

Expected stderr (literal):

```
Imported 1 bookmark.
exit: 0
```

Expected `bm list` stdout: one line matching `2026-05-25T03:00:00+00:00 https://imported.example`. The destination store was created on the import write (file did not exist before; parallel to `bm add`'s store-creation behavior).

```sh
rm -rf "$(dirname "$IMPORT_DB")"
unset IMPORT_DB
```

---

## Step 7 — `bm import` idempotent on repeat invocation (AC 20)

```sh
IDEMP_DB="$(mktemp -d)/idemp.json"
PAYLOAD='{"bookmarks":[{"url":"https://example.com","timestamp":"2026-05-25T03:00:00Z","tags":["rust"]}]}'

echo "$PAYLOAD" | BOOKMARK_CLI_DB="$IDEMP_DB" bm import
echo "$PAYLOAD" | BOOKMARK_CLI_DB="$IDEMP_DB" bm import
echo "exit: $?"
BOOKMARK_CLI_DB="$IDEMP_DB" bm export | python3 -c "import json, sys; d = json.load(sys.stdin); print(f'bookmarks: {len(d[\"bookmarks\"])}')"
```

Expected:

```
Imported 1 bookmark.
Imported 0 bookmarks.
exit: 0
bookmarks: 1
```

Second import is dedup'd on `(url, timestamp, sorted(tags))` exact-tuple-match per the Round 1 sorted-tag-comparison dedup decision; the destination store still contains exactly one copy.

```sh
rm -rf "$(dirname "$IDEMP_DB")"
unset IDEMP_DB
```

---

## Step 8 — `bm import` sorted-tag-comparison dedup (Round 1 Phase 4 routed)

Closes the [Software Engineer R1 F2 + Red Team R1 F1 sorted-tag-comparison dedup](../vsdd-suite/review-log/2026-05-24-phase-4-routing.md) routing decision. Same `(url, timestamp)` but tags in different orders MUST collapse to one record (treats `tags` as a set during dedup; storage `Vec<String>` still preserves insertion order at the record level).

```sh
TAGORDER_DB="$(mktemp -d)/tagorder.json"

# First import: tags in [rust, go] order.
echo '{"bookmarks":[{"url":"https://example.com","timestamp":"2026-05-25T03:00:00Z","tags":["rust","go"]}]}' \
    | BOOKMARK_CLI_DB="$TAGORDER_DB" bm import

# Second import: same record, tags reordered to [go, rust]. Sorted-tag-
# comparison dedup must collapse this to zero appended.
echo '{"bookmarks":[{"url":"https://example.com","timestamp":"2026-05-25T03:00:00Z","tags":["go","rust"]}]}' \
    | BOOKMARK_CLI_DB="$TAGORDER_DB" bm import

BOOKMARK_CLI_DB="$TAGORDER_DB" bm export | python3 -c "import json, sys; d = json.load(sys.stdin); print(f'bookmarks: {len(d[\"bookmarks\"])}')"

rm -rf "$(dirname "$TAGORDER_DB")"
unset TAGORDER_DB
```

Expected stderr (in order, literal):

```
Imported 1 bookmark.
Imported 0 bookmarks.
```

Expected final stdout: `bookmarks: 1`. The pre-Round-1 impl used `Vec<String>` element-wise equality which would have produced 2 bookmarks here; the Phase 2b sorted-tag-comparison fix collapses them.

---

## Step 9 — `bm import` active control-char tag rejection (Round 1 Phase 4 routed)

Closes the [Security R1 F2 imported-tag control-char rejection](../vsdd-suite/review-log/2026-05-24-phase-4-routing.md) routing decision (active mitigation). The Layer 2 tag-injection accepted-risk was conditioned on attacker write-access; Layer 3 stdin-attacker doesn't need write-access, so the active mitigation closes the gap.

```sh
CCHAR_DB="$(mktemp -d)/cchar.json"

# Payload with a control character (ESC, U+001B) inside a tag string.
printf '{"bookmarks":[{"url":"https://example.com","timestamp":"2026-05-25T03:00:00Z","tags":["rust\\u001binjection"]}]}' \
    | BOOKMARK_CLI_DB="$CCHAR_DB" bm import
echo "exit: $?"

# Verify the destination store was NOT created (rejection happens pre-mutation).
ls "$CCHAR_DB" 2>&1 || echo "(store correctly absent)"

rm -rf "$(dirname "$CCHAR_DB")"
unset CCHAR_DB
```

Expected (literal):

```
Error: imported bookmark tags contain disallowed control characters.
Offending record index: 0
Offending tag: rustinjection
exit: 1
(store correctly absent)
```

The offending tag is rendered through `display_safe` before reaching stderr so the raw ESC byte does NOT reach the operator's terminal.

---

## Step 10 — `bm export | bm import` byte-preservation round-trip (AC 28; Round 1 Phase 4 routed)

Closes the [4-domain convergence on JSON-native escape design](../vsdd-suite/review-log/2026-05-24-phase-4-routing.md) (SA + SE + RT + Sec Round 1). The round-trip MUST preserve original bytes — pathological control bytes survive as their original byte values, NOT as 8-character ASCII literals.

```sh
SRC_DB="$(mktemp -d)/src.json"
DST_DB="$(mktemp -d)/dst.json"

# Write a source store with a URL containing a raw ESC byte (control char).
python3 -c "
import json
store = {'bookmarks': [{'url': 'https://evil.example/\\u001b[31mfrobnicate', 'timestamp': '2026-05-25T03:00:00Z', 'tags': []}]}
print(json.dumps(store, indent=2))
" > "$SRC_DB"
chmod 600 "$SRC_DB"

# Round-trip: export from src; pipe to import on dst.
BOOKMARK_CLI_DB="$SRC_DB" bm export | BOOKMARK_CLI_DB="$DST_DB" bm import

# Compare the bytes of the URL field via Python — the parsed-back URL
# from the destination store must contain the original ESC byte (U+001B).
python3 -c "
import json
src_url = json.load(open('$SRC_DB'))['bookmarks'][0]['url']
dst_url = json.load(open('$DST_DB'))['bookmarks'][0]['url']
print(f'src has ESC: {chr(0x1b) in src_url}')
print(f'dst has ESC: {chr(0x1b) in dst_url}')
print(f'byte-equal: {src_url == dst_url}')
"

rm -rf "$(dirname "$SRC_DB")" "$(dirname "$DST_DB")"
unset SRC_DB DST_DB
```

Expected stderr from the import:

```
Imported 1 bookmark.
```

Expected final stdout (literal):

```
src has ESC: True
dst has ESC: True
byte-equal: True
```

This is the byte-preservation round-trip contract per the JSON-native escape design (serde_json's native string encoder handles Cc-range control chars; the round-trip recovers them losslessly).

---

## Step 11 — `bm import` failure paths (AC 22 + AC 23 + AC 24)

Exercises the 3 stdin-rejection paths in one consolidated step (compact testing pattern). Each sub-step asserts the spec-contracted error string + exit 1 + no file write.

### Step 11a — Empty stdin (AC 22)

```sh
FAIL_DB="$(mktemp -d)/fail.json"
echo -n "" | BOOKMARK_CLI_DB="$FAIL_DB" bm import 2>&1 >/dev/null
echo "exit: $?"
ls "$FAIL_DB" 2>&1 || echo "(store correctly absent)"
```

Expected (literal):

```
Error: stdin is empty; nothing to import.
exit: 1
(store correctly absent)
```

### Step 11b — Invalid JSON (AC 23)

```sh
echo "not json at all" | BOOKMARK_CLI_DB="$FAIL_DB" bm import 2>&1 >/dev/null
echo "exit: $?"
ls "$FAIL_DB" 2>&1 || echo "(store correctly absent)"
```

Expected stderr starts with: `Error: stdin is not valid JSON.` followed by the underlying serde_json parse error. Exit 1. Store absent.

### Step 11c — Schema mismatch (AC 24)

```sh
echo '{"wrong":"shape"}' | BOOKMARK_CLI_DB="$FAIL_DB" bm import 2>&1 >/dev/null
echo "exit: $?"
ls "$FAIL_DB" 2>&1 || echo "(store correctly absent)"
```

Expected stderr starts with: `Error: stdin JSON does not match storage-format schema; expected {"bookmarks": [...]}.` Exit 1. Store absent.

### Step 11d — Cleanup

```sh
rm -rf "$(dirname "$FAIL_DB")"
unset FAIL_DB
```

---

## Step 12 — `bm import` stdin size cap (AC 27)

```sh
CAP_DB="$(mktemp -d)/cap.json"

# Build a payload that exceeds a 100-byte cap.
PAYLOAD=$(python3 -c "import json; print(json.dumps({'bookmarks':[{'url':'https://example.com/' + 'x' * 1000, 'timestamp':'2026-05-25T03:00:00Z','tags':[]}]}))")
echo "$PAYLOAD" | BOOKMARK_CLI_DB="$CAP_DB" bm import --max-stdin-bytes 100 2>&1 >/dev/null
echo "exit: $?"
ls "$CAP_DB" 2>&1 || echo "(store correctly absent)"

# Same payload with override accepting larger cap — succeeds.
echo "$PAYLOAD" | BOOKMARK_CLI_DB="$CAP_DB" bm import --max-stdin-bytes 5000
echo "exit: $?"

rm -rf "$(dirname "$CAP_DB")"
unset CAP_DB PAYLOAD
```

Expected for the rejection case (literal first two lines; bytes count + MiB value may vary by exact payload size):

```
Error: stdin exceeded maximum byte limit of 100 bytes (0.0 MiB).
Hint: use --max-stdin-bytes <N> to override the default; pass a byte count larger than the input payload.
exit: 1
(store correctly absent)
```

Expected for the override-accept case (literal):

```
Imported 1 bookmark.
exit: 0
```

---

## Step 13 — `bm import` against a Layer-1-format destination store (AC 25)

Closes the forward-only migration semantic at the import write path (parallel to `bm tag`'s Layer-1-format migration).

```sh
MIG_DB="$(mktemp -d)/mig.json"

# Write a Layer-1-format destination store (no `tags` field per bookmark).
cat > "$MIG_DB" <<'JSON'
{
  "bookmarks": [
    {"url": "https://A.example", "timestamp": "2026-05-21T01:00:00Z"},
    {"url": "https://B.example", "timestamp": "2026-05-21T02:00:00Z"}
  ]
}
JSON

# Import a Layer-2-format payload (with tags).
echo '{"bookmarks":[{"url":"https://C.example","timestamp":"2026-05-25T03:00:00Z","tags":["rust"]}]}' \
    | BOOKMARK_CLI_DB="$MIG_DB" bm import

# Verify the post-write store has explicit `tags` on every bookmark
# (touched + untouched alike) — the forward-only migration shape.
python3 -c "
import json
store = json.load(open('$MIG_DB'))
for b in store['bookmarks']:
    print(f'{b[\"url\"]}: tags={b.get(\"tags\", \"MISSING\")}')
"

rm -rf "$(dirname "$MIG_DB")"
unset MIG_DB
```

Expected stderr:

```
Imported 1 bookmark.
```

Expected final stdout:

```
https://A.example: tags=[]
https://B.example: tags=[]
https://C.example: tags=['rust']
```

The Layer-1-origin bookmarks (A + B) gain explicit `tags: []` arrays on the import write per the forward-only migration discipline.

---

## Step 14 — `bm import` partial-failure atomicity (AC 26)

```sh
ATOMIC_DB="$(mktemp -d)/atomic.json"

# Seed via bm add.
BOOKMARK_CLI_DB="$ATOMIC_DB" bm add "https://existing.example"
PRE_HASH=$(shasum -a 256 "$ATOMIC_DB" | awk '{print $1}')
echo "pre-import store hash: $PRE_HASH"

# Payload with one valid record + one invalid record (missing required `url` field).
PAYLOAD='{"bookmarks":[
    {"url":"https://valid.example","timestamp":"2026-05-25T03:00:00Z","tags":[]},
    {"timestamp":"2026-05-25T03:00:00Z","tags":[]}
]}'
echo "$PAYLOAD" | BOOKMARK_CLI_DB="$ATOMIC_DB" bm import 2>&1 >/dev/null
echo "exit: $?"

POST_HASH=$(shasum -a 256 "$ATOMIC_DB" | awk '{print $1}')
echo "post-import store hash: $POST_HASH"
[ "$PRE_HASH" = "$POST_HASH" ] && echo "PASS — store byte-identical" || echo "FAIL — store mutated"

rm -rf "$(dirname "$ATOMIC_DB")"
unset ATOMIC_DB PRE_HASH POST_HASH PAYLOAD
```

Expected: stderr starts with the schema-mismatch error (the missing `url` field is a per-record validation failure routed through `ImportError::SchemaMismatch`). Exit 1. The `PASS — store byte-identical` line confirms the destination store was preserved unmodified despite the import attempting to land mixed valid + invalid records (atomicity discipline).

On Linux replace `shasum -a 256` with `sha256sum` if needed.

---

## Step 15 — Performance budget sanity-check for `bm export` + `bm import`

Parallel to [`layer-2.md`](layer-2.md) Step 12. Exercises `bm export` + `bm import` at the 1,000-bookmark cliff. The [`DESIGN.md`](../DESIGN.md) § Performance budget contract applies (mean < 100 ms on a 1,000-bookmark store on commodity hardware).

**Prerequisite (same as `layer-2.md` Step 12):** [`hyperfine`](https://github.com/sharkdp/hyperfine). Install via `brew install hyperfine` / `apt install hyperfine` / `cargo install hyperfine --locked`. The `time` builtin fallback is documented at the end of this step for environments where `hyperfine` install is impractical.

### Step 15a — Generate a 1,000-bookmark store

```sh
BENCH_DB="$(dirname "$BOOKMARK_CLI_DB")/bench-1000.json"
python3 -c "
import json, datetime
now = datetime.datetime(2026, 5, 25, 0, 0, 0, tzinfo=datetime.timezone.utc)
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

Expected: `Generated <line-count> lines in <path>/bench-1000.json` with `<line-count>` between 4,000 and 10,000.

### Step 15b — hyperfine against bm export + bm import

```sh
export BOOKMARK_CLI_DB="$BENCH_DB"

# Capture the export output to a tempfile for the import benchmark.
EXPORT_OUT="$(dirname "$BENCH_DB")/exported.json"
bm export > "$EXPORT_OUT"

# Benchmark export (with + without filter) and import (against a fresh
# destination store seeded with the same 1,000-record payload — exercises
# dedup against existing state at scale).
DST_FOR_IMPORT="$(dirname "$BENCH_DB")/import-dst.json"
cp "$BENCH_DB" "$DST_FOR_IMPORT"

hyperfine --warmup 3 --runs 10 \
    'bm export > /dev/null' \
    'bm export --tag rust > /dev/null' \
    "cat '$EXPORT_OUT' | BOOKMARK_CLI_DB='$DST_FOR_IMPORT' bm import"
echo "exit: $?"
```

Expected output: `hyperfine` emits its own formatted report. Pass/fail criterion is qualitative — assert that each operation's mean falls under the budget:

| Operation | Budget (p95 per `DESIGN.md`) | Pass criterion (mean at N=10) |
|---|---|---|
| `bm export` (1,000-bookmark store) | < 100 ms | mean < 100 ms |
| `bm export --tag rust` (1,000-bookmark store) | < 100 ms | mean < 100 ms |
| `bm import` (10K dedup-against-existing-state at 1,000 × 1,000) | < 200 ms (relaxed per dedup-complexity accepted-limit) | mean < 200 ms |

The `bm import` budget is intentionally looser per the [Layer 3 dedup-complexity accepted-limit annotation](../DESIGN.md#performance-budget-) — the O(M × N) sorted-tag-comparison dedup at 1,000 × 1,000 is ~10^6 comparisons + JSON re-parse + atomic write; 200 ms is the documented acceptable envelope.

### Step 15c — Cleanup the benchmark store

```sh
rm -f "$BENCH_DB" "$EXPORT_OUT" "$DST_FOR_IMPORT"
unset BENCH_DB EXPORT_OUT DST_FOR_IMPORT
```

### Fallback: `time` builtin (no-hyperfine environments)

Same fallback pattern as [`layer-2.md`](layer-2.md) Step 12 — POSIX `time` builtin provides a coarser single-run measurement adequate to detect catastrophic regression.

---

## Step 16 — Cleanup

```sh
rm -rf "$(dirname "$BOOKMARK_CLI_DB")"
unset BOOKMARK_CLI_DB
```

No expected output; cleanup is best-effort.

---

## Closure protocol per session

Per primer 3 § Manual testing is a second adversarial surface to IAR ([G-132](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-132)) — same convention as [`layer-1.md`](layer-1.md) + [`layer-2.md`](layer-2.md) § Closure protocol per session. Every Layer 3 manual-test session closes with one of:

- **Insight-reached / no findings** — all 16 steps reached expected outputs; record the session timestamp + the `Tested against:` field above + a one-line "passed clean" note. No per-domain review-log entry needed.
- **Findings surfaced** — any step diverged from expected; record each divergence as a finding in the per-domain review log (the natural-pair domain — typically QE for test-discipline issues; [UX](../../../vsdd-suite/domains/role/UX-REVIEW.md) for output-quality concerns; SE for binary-behavior bugs; [Security](../../../vsdd-suite/domains/role/SECURITY-REVIEW.md) for control-char rejection regressions; [Performance Engineer](../../../vsdd-suite/domains/role/PERFORMANCE-ENGINEER-REVIEW.md) for Step 15 budget violations) with `**Source:** director-raised` per the per-review-preamble standard ([G-133](../../../vsdd-suite/suite-development/FINDINGS-INDEX.md#g-133)), the divergence cited file:line + the expected vs. observed output, and the appropriate `**Owner:**` per Review 77 lifecycle.

## What comes next

After all 16 steps pass cleanly, Layer 3 Layer-gate criterion 3 (per [`TODO.md` § Layer 3](../TODO.md#layer-3--export-and-import-ai-co-authored-operator-owned)) is satisfied. The remaining Layer-gate criteria (1 + 2 + 4 + 5) close via Phase 3 IAR Round 2 (validation that Round 1 fixes hold) + Phase 5 hardening (Purity Boundary Audit re-run + Mutation Testing re-run + proptest round-trip + cargo-fuzz harness on `bm import`).
