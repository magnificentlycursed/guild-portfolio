#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    reason = "Restriction-group lints from [lints.clippy] apply to production code; \
              integration tests use unwrap/expect/panic freely per Rust supplement \
              test-helper convention. Platform Engineer Round 2 Finding 13."
)]

//! Phase 2a-equivalent Layer 2 data-scaling sentinel tests.
//!
//! Per [`DESIGN.md`](../DESIGN.md) § Performance budget "Data-scaling tests":
//! Layer 2 ships sentinel integration tests at the 100 / 1,000 / 10,000-bookmark
//! cliffs that exercise the full add → list → tag → list-filter cycle. Each
//! cliff asserts: (a) the storage file round-trips without corruption, and
//! (b) the filter result set is correct against a programmatically-generated
//! reference. **Wall-clock budget assertions are NOT made here** — those are
//! flaky in CI and are exercised by the `hyperfine` sanity-check at
//! `manual-tests/layer-2.md` Step 12. This file's purpose is correctness-at-scale.
//!
//! All three tests are `#[ignore]`-gated by default so `cargo test` stays fast.
//! Run them via `cargo test -- --ignored` (the CI workflow does this in a
//! separate Linux-only job to keep macOS-runner cost down).
//!
//! Closes:
//! - [Performance Engineer Review 1 Finding 5](../vsdd-suite/review-log/2026-05-20-performance-engineer.md) (Layer-1-Deferred to Layer 2).
//! - Layer 2 Round 1 Quality Engineer F1 + Performance Engineer F2 + Solution Owner F1 — the spec/decomposition/manual-test plan all named `tests/scaling.rs` as shipped, but the file did not exist on disk; this file is the fulfillment.

use assert_cmd::Command;
use bookmark_cli::BookmarkStore;
use std::path::Path;
use tempfile::tempdir;

/// Generates `n` bookmarks via the library API (`BookmarkStore::add` +
/// `BookmarkStore::save` once at the end). Returns the recorded URLs so the
/// test can pick a known URL (e.g. `n/2`) to tag.
///
/// **PR #47 refactor (Layer 2 Phase-5-trigger PE R2 F4 close):** prior
/// shape spawned `bm add` once per bookmark via `assert_cmd`. At N=10,000
/// the 10,000 process spawns dominated the wall-clock (~24 min measured
/// vs. ~1-2 min docstring estimate) — the test was measuring process-spawn
/// overhead, not the `add` codepath at scale. The library-API path tests
/// the actual storage-layer scale; the binary-surface integration aspect
/// is already covered by `tests/bookmarks.rs` per-bookmark tests. With
/// the in-process loop + single trailing `save`, 10K bookmarks complete
/// in seconds rather than minutes.
///
/// The fsync codepath is still exercised — `save` calls
/// `fsync_directory` on the parent dir on Unix per `DESIGN.md`
/// § Performance budget § Durability discipline (Layer 2) — but it's
/// called once at population-end, not once per bookmark. This matches the
/// production-shape semantic: a real operator running 10K `bm add`
/// invocations pays the per-add save + fsync cost; the SCALING SENTINEL
/// tests the correctness-at-scale, not the per-add wall-clock-at-scale
/// (which is dominated by process-spawn overhead anyway).
fn populate(db: &Path, n: usize) -> Vec<String> {
    let mut store = BookmarkStore::default();
    let mut urls = Vec::with_capacity(n);
    for i in 0..n {
        let url = format!("https://example-{i}.com");
        store.add(url.clone()).unwrap();
        urls.push(url);
    }
    store.save(db).unwrap();
    urls
}

/// Counts the lines of `bm list`'s stdout against the store at `db`.
fn list_line_count(db: &Path) -> usize {
    let output = Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", db)
        .args(["list"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    String::from_utf8(output).unwrap().lines().count()
}

/// Counts the lines of `bm list --tag <label>`'s stdout against the store at `db`.
fn list_with_tag_line_count(db: &Path, label: &str) -> usize {
    let output = Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", db)
        .args(["list", "--tag", label])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    String::from_utf8(output).unwrap().lines().count()
}

/// 100-bookmark sentinel — the smallest cliff. Runs in well under a second
/// on commodity hardware and is the closest to the unit-test scale; useful
/// for catching regressions that only manifest above the per-test fixtures'
/// 2-3-bookmark scale.
#[test]
#[ignore = "scaling sentinel; run via `cargo test -- --ignored`"]
fn scaling_100_bookmarks_round_trips_and_filters_correctly() {
    let dir = tempdir().unwrap();
    let db = dir.path().join("bookmarks.json");
    let n = 100_usize;

    let urls = populate(&db, n);

    assert_eq!(
        list_line_count(&db),
        n,
        "bm list against {n}-bookmark store should emit exactly {n} lines"
    );

    // Tag the middle bookmark — `n/2` is a deterministic pick.
    let target = &urls[n / 2];
    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["tag", target, "testtag"])
        .assert()
        .success()
        .code(0);

    assert_eq!(
        list_with_tag_line_count(&db, "testtag"),
        1,
        "exactly one bookmark matches `testtag` after a single tag invocation"
    );

    // Library-side round-trip: confirm the on-disk file deserializes to the
    // expected bookmark count via the pure `BookmarkStore::load` path.
    let loaded = BookmarkStore::load(&db).expect("store should round-trip through load");
    assert_eq!(
        loaded.bookmarks().len(),
        n,
        "BookmarkStore::load should recover exactly {n} bookmarks"
    );
}

/// 1,000-bookmark sentinel — the budget table's named cliff. Per
/// `DESIGN.md` § Performance budget the operations at this scale must
/// complete within the 100 ms p95 budget; the wall-clock assertion is the
/// `manual-tests/layer-2.md` Step 12 hyperfine sanity-check's responsibility.
/// This test asserts correctness only.
#[test]
#[ignore = "scaling sentinel; run via `cargo test -- --ignored`"]
fn scaling_1000_bookmarks_round_trips_and_filters_correctly() {
    let dir = tempdir().unwrap();
    let db = dir.path().join("bookmarks.json");
    let n = 1_000_usize;

    let urls = populate(&db, n);

    assert_eq!(
        list_line_count(&db),
        n,
        "bm list against {n}-bookmark store should emit exactly {n} lines"
    );

    let target = &urls[n / 2];
    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["tag", target, "testtag"])
        .assert()
        .success()
        .code(0);

    assert_eq!(
        list_with_tag_line_count(&db, "testtag"),
        1,
        "exactly one bookmark matches `testtag` after a single tag invocation"
    );

    let loaded = BookmarkStore::load(&db).expect("store should round-trip through load");
    assert_eq!(
        loaded.bookmarks().len(),
        n,
        "BookmarkStore::load should recover exactly {n} bookmarks"
    );
}

/// 10,000-bookmark sentinel — the project's scale ceiling per
/// `DESIGN.md` § Performance budget. Beyond this the operator should
/// consider a real bookmark manager; the flat-JSON-rewrite-on-every-add
/// design has cumulative O(n²) cost which makes very large stores
/// impractical. This test verifies correctness still holds at the
/// declared ceiling — meaningful wall-clock budget violations are
/// expected and are surfaced by the manual-test hyperfine sanity-check,
/// not asserted here.
///
/// Wall-clock note: 10,000 sequential `bm add` invocations × ~5-15 ms
/// each (including the atomic-save + fsync per add) takes on the order
/// of a minute or two on commodity hardware. This is why the sentinel
/// is `#[ignore]`-gated by default.
#[test]
#[ignore = "scaling sentinel (~5-15 sec wall-clock post-PR-#47 library-API populate); run via `cargo test -- --ignored`"]
fn scaling_10_000_bookmarks_round_trips_and_filters_correctly() {
    let dir = tempdir().unwrap();
    let db = dir.path().join("bookmarks.json");
    let n = 10_000_usize;

    let urls = populate(&db, n);

    assert_eq!(
        list_line_count(&db),
        n,
        "bm list against {n}-bookmark store should emit exactly {n} lines"
    );

    let target = &urls[n / 2];
    Command::cargo_bin("bm")
        .unwrap()
        .env("BOOKMARK_CLI_DB", &db)
        .args(["tag", target, "testtag"])
        .assert()
        .success()
        .code(0);

    assert_eq!(
        list_with_tag_line_count(&db, "testtag"),
        1,
        "exactly one bookmark matches `testtag` after a single tag invocation"
    );

    let loaded = BookmarkStore::load(&db).expect("store should round-trip through load");
    assert_eq!(
        loaded.bookmarks().len(),
        n,
        "BookmarkStore::load should recover exactly {n} bookmarks"
    );
}
