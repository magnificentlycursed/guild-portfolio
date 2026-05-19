//! Storage logic for bookmark-cli — mixed purity surface.
//!
//! See `DESIGN.md` § Verification architecture for the authoritative
//! purity boundary. In summary (per the Review 67 / B2 reconciliation):
//!
//! - **Pure:** `Bookmark` + `BookmarkStore` data types; `BookmarkStore::newest_first`.
//! - **Effectful (deliberate I/O wrappers):** `BookmarkStore::load`
//!   (filesystem read + parse), `BookmarkStore::save` (filesystem write +
//!   serialize). These wrap pure JSON ser/de with file I/O — the wrapping
//!   is the convenience boundary the project chose at Layer 1, not impl
//!   drift from a stricter pure-core claim.
//! - **Boundary refinement:** `BookmarkStore::add` reads `Utc::now()` at
//!   call time. Morally pure with respect to its inputs (URL string) but
//!   non-deterministic w.r.t. the clock. Could be refined to take a
//!   `timestamp: DateTime<Utc>` parameter at a future layer if formal
//!   verification of `add` is in scope; for the current Layer 1 portfolio
//!   intent, the clock dependency is accepted.
//!
//! Tests against `lib.rs` use `tempfile` for I/O isolation; tests against
//! the compiled binary live in `tests/bookmarks.rs`.

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Bookmark {
    pub url: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct BookmarkStore {
    #[serde(default)]
    pub bookmarks: Vec<Bookmark>,
}

impl BookmarkStore {
    /// Loads the store from `path`.
    ///
    /// - Absent file → empty store (not an error; first-use case).
    /// - Empty file (zero bytes) → empty store (treat as fresh).
    /// - Present file with invalid JSON → error (with file path context).
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("reading bookmark store at {}", path.display()))?;
        if contents.trim().is_empty() {
            return Ok(Self::default());
        }
        serde_json::from_str(&contents)
            .with_context(|| format!("parsing bookmark store at {}", path.display()))
    }

    /// Saves the store to `path`, overwriting any existing content.
    /// Creates parent directories if needed.
    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)
                    .with_context(|| format!("creating parent dir for {}", path.display()))?;
            }
        }
        let json = serde_json::to_string_pretty(self)
            .context("serializing bookmark store to JSON")?;
        std::fs::write(path, json + "\n")
            .with_context(|| format!("writing bookmark store at {}", path.display()))?;
        Ok(())
    }

    /// Appends a bookmark with the supplied URL and the current UTC time.
    /// Caller is responsible for non-empty URL validation.
    pub fn add(&mut self, url: String) {
        self.bookmarks.push(Bookmark {
            url,
            timestamp: Utc::now(),
        });
    }

    /// Returns bookmarks sorted newest-first (by timestamp descending).
    /// Pure — does not mutate the store.
    pub fn newest_first(&self) -> Vec<&Bookmark> {
        let mut sorted: Vec<&Bookmark> = self.bookmarks.iter().collect();
        sorted.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        sorted
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn bm(url: &str, ts: DateTime<Utc>) -> Bookmark {
        Bookmark { url: url.to_string(), timestamp: ts }
    }

    #[test]
    fn newest_first_sorts_descending_by_timestamp() {
        let t1 = Utc.with_ymd_and_hms(2026, 5, 17, 1, 0, 0).unwrap();
        let t2 = Utc.with_ymd_and_hms(2026, 5, 17, 2, 0, 0).unwrap();
        let t3 = Utc.with_ymd_and_hms(2026, 5, 17, 3, 0, 0).unwrap();
        let store = BookmarkStore {
            bookmarks: vec![bm("https://b", t1), bm("https://a", t3), bm("https://c", t2)],
        };
        let sorted = store.newest_first();
        assert_eq!(sorted[0].url, "https://a");
        assert_eq!(sorted[1].url, "https://c");
        assert_eq!(sorted[2].url, "https://b");
    }

    #[test]
    fn load_returns_empty_for_missing_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("absent.json");
        let store = BookmarkStore::load(&path).unwrap();
        assert!(store.bookmarks.is_empty());
    }

    #[test]
    fn load_returns_empty_for_empty_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("empty.json");
        std::fs::write(&path, "").unwrap();
        let store = BookmarkStore::load(&path).unwrap();
        assert!(store.bookmarks.is_empty());
    }

    #[test]
    fn save_then_load_roundtrips() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("rt.json");
        let mut store = BookmarkStore::default();
        store.add("https://example.com".to_string());
        store.save(&path).unwrap();
        let loaded = BookmarkStore::load(&path).unwrap();
        assert_eq!(loaded.bookmarks.len(), 1);
        assert_eq!(loaded.bookmarks[0].url, "https://example.com");
    }

    /// retroactive Red Gate (Phase 5 source): save creates parent directory
    /// for a nested-path target — Surface B (cargo-mutants) surfaced the gap
    /// at src/lib.rs:48 where `!parent.as_os_str().is_empty()` could be
    /// flipped without any test failing. Test added post-MVR; confirmed
    /// passes against current implementation. See vsdd-suite/PHASE-5-LOG.md
    /// Layer 1 Surface B disposition for the surviving mutant.
    #[test]
    fn save_creates_parent_directory_for_nested_path() {
        let dir = tempfile::tempdir().unwrap();
        // Parent directory does NOT exist yet — save must create it.
        let path = dir.path().join("nested").join("subdir").join("bookmarks.json");
        assert!(!path.parent().unwrap().exists(), "parent must not exist before save");

        let mut store = BookmarkStore::default();
        store.add("https://example.com".to_string());
        store.save(&path).expect("save should create missing parent directories");

        assert!(path.exists(), "store file should exist after save to nested path");
        assert!(path.parent().unwrap().exists(), "parent directory should have been created");

        // Verify the saved content round-trips correctly through the just-created path.
        let loaded = BookmarkStore::load(&path).unwrap();
        assert_eq!(loaded.bookmarks.len(), 1);
        assert_eq!(loaded.bookmarks[0].url, "https://example.com");
    }
}
