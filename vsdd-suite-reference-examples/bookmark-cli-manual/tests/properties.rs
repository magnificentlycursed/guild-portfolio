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

//! Phase 5 Layer 2 — property-based testing via `proptest`.
//!
//! Per [`DESIGN.md`](../DESIGN.md) § Project intent's Phase 5 strategy for
//! Layer 2: "property-based testing via `proptest` now warranted... activated
//! against the tag-idempotence + filter-OR-monotonicity properties." Both
//! properties operate on the pure `BookmarkStore` API (`attach_tag` +
//! `filter_by_tags`) per `DESIGN.md` § Verification architecture's purity
//! boundary — no `BOOKMARK_CLI_DB` / filesystem / binary involved.
//!
//! - **Tag idempotence:** for any `(url, label)` pair, `attach_tag(url, label)`
//!   followed by `attach_tag(url, label)` produces the same final store
//!   state as a single `attach_tag` invocation. The library-level idempotence
//!   contract per `DESIGN.md` § `bm tag` § Idempotence under repeat invocation.
//! - **Filter OR-monotonicity:** for any disjoint label sets `A` and `B`,
//!   `filter_by_tags(A ∪ B)` returns the set-union of `filter_by_tags(A)` and
//!   `filter_by_tags(B)`. The OR-semantics contract per `DESIGN.md`
//!   § `bm list --tag <label>` "OR-semantics across repeated flags."
//!
//! Closes Layer 2 Round 1 VDD-IAR Alignment R4 F5 + Solution Owner R4 F2
//! load-bearing: DESIGN.md's Phase 5 strategy for Layer 2 declared `proptest`
//! activation; the cluster surfaced that no proptest dev-dep / tests existed
//! on disk. This file is the activation.

use bookmark_cli::BookmarkStore;
use proptest::prelude::*;
use std::collections::HashSet;

/// A small alphabet for URLs + labels so the search space stays dense enough
/// that the two properties get genuine cross-bookmark overlap. Wide enough
/// to admit duplicates (so tag idempotence has multi-match cases to
/// exercise) but narrow enough that filter overlap occurs without an
/// exponential blow-up in the number of cases proptest must enumerate.
fn small_url_strategy() -> impl Strategy<Value = String> {
    "https://example-[0-3]\\.com".prop_map(String::from)
}

fn small_label_strategy() -> impl Strategy<Value = String> {
    "[a-d]{1,3}".prop_map(String::from)
}

/// Generates a `BookmarkStore` populated with 0..=8 random URL entries.
/// The store is built via the public `add` API so the resulting state is
/// the same shape `bm add` would have produced.
fn small_store_strategy() -> impl Strategy<Value = BookmarkStore> {
    prop::collection::vec(small_url_strategy(), 0..=8).prop_map(|urls| {
        let mut store = BookmarkStore::default();
        for url in urls {
            // `add` only rejects the empty URL; the strategy never emits an
            // empty string so unwrap is safe.
            store.add(url).unwrap();
        }
        store
    })
}

proptest! {
    #![proptest_config(ProptestConfig {
        // 64 cases is small enough that `cargo test` stays fast (< 1s for the
        // two properties combined) but large enough to surface non-trivial
        // counterexamples — proptest's default of 256 is overkill for a
        // pure-side property on a 0..=8-bookmark store with a 4-URL alphabet.
        cases: 64,
        .. ProptestConfig::default()
    })]

    /// Tag idempotence — attach_tag twice produces the same state as attach_tag once.
    ///
    /// Acceptance criterion (per DESIGN.md § `bm tag` § Idempotence): the
    /// second `attach_tag` invocation against the same `(url, label)` pair
    /// does NOT duplicate the label in the matching bookmark's `tags` field;
    /// the store state after the second call is byte-equal to the state
    /// after the first.
    #[test]
    fn tag_idempotence_property(
        store in small_store_strategy(),
        url in small_url_strategy(),
        label in small_label_strategy(),
    ) {
        let mut once = store.clone();
        // If the URL doesn't match any bookmark, attach_tag returns
        // NoMatch — property holds trivially (no state change in either
        // run), so skip the cases where there's no match to assert against.
        let single_result = once.attach_tag(&url, &label);
        prop_assume!(single_result.is_ok());

        let mut twice = store.clone();
        twice.attach_tag(&url, &label).unwrap();
        twice.attach_tag(&url, &label).unwrap();

        prop_assert_eq!(
            once.bookmarks(),
            twice.bookmarks(),
            "attach_tag twice should produce the same state as attach_tag once for (url, label) = ({:?}, {:?})",
            url,
            label
        );
    }

    /// Filter OR-monotonicity — filter_by_tags(A ∪ B) = filter_by_tags(A) ∪ filter_by_tags(B).
    ///
    /// Acceptance criterion (per DESIGN.md § `bm list --tag <label>`):
    /// repeated `--tag` flags compose with OR-semantics, so filtering by
    /// the union of two disjoint label sets should yield the union of the
    /// individual filter results. Compared at the URL-set level (not the
    /// list level) so that newest-first ordering — which depends on
    /// nondeterministic `Utc::now()` timestamps — does not affect the
    /// assertion.
    #[test]
    fn filter_or_monotonicity_property(
        urls in prop::collection::vec(small_url_strategy(), 0..=8),
        tag_a in small_label_strategy(),
        tag_b in small_label_strategy(),
        a_assignments in prop::collection::vec(any::<bool>(), 0..=8),
        b_assignments in prop::collection::vec(any::<bool>(), 0..=8),
    ) {
        // Pre-filter to disjoint labels so the union test is meaningful.
        prop_assume!(tag_a != tag_b);

        let mut store = BookmarkStore::default();
        for url in &urls {
            store.add(url.clone()).unwrap();
        }

        // For each unique URL, optionally assign tag_a / tag_b based on the
        // generated bit-vectors. Use `attach_tag` (the public API) so the
        // property exercises the same path the CLI would.
        let unique_urls: Vec<String> = {
            let mut seen = HashSet::new();
            urls.iter().filter(|u| seen.insert((*u).clone())).cloned().collect()
        };

        for (i, url) in unique_urls.iter().enumerate() {
            if a_assignments.get(i).copied().unwrap_or(false) {
                // attach_tag returns NoMatch only if the URL isn't in the
                // store — every `unique_urls` entry is in the store by
                // construction, so this unwrap is safe.
                store.attach_tag(url, &tag_a).unwrap();
            }
            if b_assignments.get(i).copied().unwrap_or(false) {
                store.attach_tag(url, &tag_b).unwrap();
            }
        }

        // Compare URL-sets — the OR-monotonicity property is about set
        // membership, not list ordering.
        let url_set = |bookmarks: Vec<&bookmark_cli::Bookmark>| -> HashSet<String> {
            bookmarks.into_iter().map(|b| b.url().to_string()).collect()
        };

        let union_labels = [tag_a.as_str(), tag_b.as_str()];
        let lhs = url_set(store.filter_by_tags(&union_labels));

        let a_only = url_set(store.filter_by_tags(&[tag_a.as_str()]));
        let b_only = url_set(store.filter_by_tags(&[tag_b.as_str()]));
        let rhs: HashSet<String> = a_only.union(&b_only).cloned().collect();

        prop_assert_eq!(
            lhs,
            rhs,
            "filter_by_tags(A ∪ B) should equal filter_by_tags(A) ∪ filter_by_tags(B); A = {:?}, B = {:?}",
            tag_a,
            tag_b
        );
    }
}
