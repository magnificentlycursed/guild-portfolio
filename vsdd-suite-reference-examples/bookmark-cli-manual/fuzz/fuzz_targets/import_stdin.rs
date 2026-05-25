#![no_main]

//! Phase 5 Layer 3 — cargo-fuzz harness for `bm import`'s stdin attack
//! surface (the project's first untrusted-input fuzz target per `DESIGN.md`
//! § Phase 5 strategy Layer 3). Bug-class targets: parse-panic,
//! parse-OOM-on-malicious-input, parse-stack-overflow, and any non-error-
//! result behavior outside the spec'd Exit 1 / Exit 2 paths.
//!
//! The harness drives `BookmarkStore::import_json` directly with arbitrary
//! byte sequences interpreted as UTF-8 (lossy where invalid). This exercises
//! the full parse-validate-dedup-mutate path in `import_json`; both the
//! Round 1 active control-char tag rejection AND the Round 2 Security F3
//! URL rejection (+ Round 2 RT F2 empty-tag rejection) are surfaces under
//! fuzz coverage.

use bookmark_cli::BookmarkStore;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Lossy UTF-8 conversion so the fuzzer's binary input space maps to the
    // String surface that `import_json` expects. Invalid UTF-8 sequences
    // become U+FFFD; the parser will reject them via the schema path, which
    // is itself a fuzz target (the parse-path's error handling).
    let payload = String::from_utf8_lossy(data);
    let mut store = BookmarkStore::default();
    let _ = store.import_json(&payload);
});
