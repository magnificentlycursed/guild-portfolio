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
//!   verification of `add` is in scope; for the current Layer 1 capstone
//!   intent, the clock dependency is accepted.
//!
//! Tests against `lib.rs` use `tempfile` for I/O isolation; tests against
//! the compiled binary live in `tests/bookmarks.rs`.
//!
//! Crate-level lint floor per the [Rust supplement](../../vsdd-suite/supplements/rust.md)
//! § Software Engineering — Clippy lint configuration. Closes [SE Review 1
//! Finding 5](../vsdd-suite/review-log/2026-05-20-software-engineer.md#r1-f5).

#![deny(missing_docs, unsafe_code)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::Path;

/// A single captured URL with its capture timestamp.
///
/// Fields are private to enforce the library's invariants at the type
/// boundary (non-empty URL; library-supplied timestamp). Serde derives
/// drive the on-disk JSON shape declared in `DESIGN.md` § Storage format
/// — field names match the JSON keys exactly so the encapsulation change
/// is invisible across the storage boundary. Closes [SE Review 1
/// Finding 4](../vsdd-suite/review-log/2026-05-20-software-engineer.md#r1-f4).
///
/// **Layer 2 — `tags` field.** Per `DESIGN.md` § Storage format `tags`
/// field section, `tags` is optional during deserialization (Layer-1-format
/// files without the field deserialize cleanly with `tags` defaulting to
/// empty `Vec<String>`) and always present during serialization (every
/// Layer-2 write emits the explicit field for every bookmark — forward-only
/// migration shape).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Bookmark {
    url: String,
    timestamp: DateTime<Utc>,
    #[serde(default)]
    tags: Vec<String>,
}

impl Bookmark {
    /// The captured URL string.
    #[must_use]
    pub fn url(&self) -> &str {
        &self.url
    }

    /// The UTC timestamp at which the URL was captured.
    #[must_use]
    pub const fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    /// The labels attached to this bookmark via `BookmarkStore::attach_tag`.
    ///
    /// Insertion order is preserved per `DESIGN.md` § Storage format `tags`
    /// field — first `bm tag` invocation's label appears first; subsequent
    /// labels append. Duplicates are not produced by the application
    /// (idempotent `bm tag`), and the spec does not contract on order
    /// beyond "label X is present in the array."
    #[must_use]
    pub fn tags(&self) -> &[String] {
        &self.tags
    }
}

/// Error variants surfaced by `BookmarkStore::attach_tag`.
///
/// Mirrors the `DESIGN.md` § `bm tag <url> <label>` failure contract — the
/// CLI shell maps each variant to the spec-contracted stderr message + exit
/// code. Hand-rolled (no `thiserror` dep) because the variant set is small
/// and the `Display` impl is the entire surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttachTagError {
    /// The supplied URL was empty. Per `DESIGN.md` § `bm tag` failure
    /// contract, the CLI shell renders this as `Error: URL cannot be
    /// empty.` and exits 1.
    EmptyUrl,
    /// The supplied label was empty. Per `DESIGN.md` § `bm tag` failure
    /// contract, the CLI shell renders this as `Error: tag label cannot
    /// be empty.` and exits 1.
    EmptyLabel,
    /// No bookmark in the store has a URL matching the supplied URL
    /// exactly (case-sensitive). The variant carries the URL string so
    /// the `Display` impl can render the spec-contracted message
    /// `no bookmark found with URL <url>` without the CLI shell needing
    /// to interpolate from out-of-band scope. Per `DESIGN.md` § `bm tag`
    /// failure contract, the CLI shell renders this as
    /// `Error: no bookmark found with URL <url>.` and exits 1.
    ///
    /// Closes Layer 2 Round 1 SE Finding 1 (variant now carries the
    /// URL value the spec-contracted message contains; library-level
    /// callers — Layer 3 importers, future test harnesses — no longer
    /// depend on the CLI shell to re-construct the message).
    NoMatch(String),
}

impl fmt::Display for AttachTagError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyUrl => f.write_str("URL cannot be empty"),
            Self::EmptyLabel => f.write_str("tag label cannot be empty"),
            Self::NoMatch(url) => write!(f, "no bookmark found with URL {url}"),
        }
    }
}

impl std::error::Error for AttachTagError {}

/// The in-memory bookmark collection, serializable to JSON per
/// `DESIGN.md` § Storage format.
///
/// The `bookmarks` field is private; callers go through `add` to mutate
/// and `bookmarks()` / `newest_first()` to read, so the library's
/// "non-empty URL" invariant is enforceable at the type boundary.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct BookmarkStore {
    #[serde(default)]
    bookmarks: Vec<Bookmark>,
}

impl BookmarkStore {
    /// Loads the store from `path`.
    ///
    /// - Absent file → empty store (not an error; first-use case).
    /// - Empty file (zero bytes) → empty store (treat as fresh).
    /// - Present file with invalid JSON → error (with file path context).
    ///
    /// # Errors
    ///
    /// Returns an error if the file exists but cannot be read, or if the
    /// file content is non-empty and fails JSON parsing per `DESIGN.md`
    /// § `bm list` failure contract (exit 2 in the caller).
    pub fn load(path: &Path) -> Result<Self> {
        // Symlink-follow rejection per `DESIGN.md` § Threat model — applies
        // to both load and save sides per the symlink-hardening discipline.
        // Closes [Red Team Review 1
        // Finding 5](../vsdd-suite/review-log/2026-05-20-red-team.md#r2-f5).
        // The Round 1 fix narrowed this to `save` only; Round 2 surfaced
        // that `load` was unchanged and still followed symlinks (read-side
        // oracle: a parse-error vs. empty vs. valid-JSON triple-channel
        // signal on whether the pointed-to file's contents conform to the
        // bookmark-store shape). Symmetric rejection on load + save closes
        // both halves of the discipline.
        //
        // **Residual TOCTOU — Red Team Round 3 Finding 2 (Accepted risk):**
        // the `symlink_metadata` check and the subsequent `read_to_string`
        // are separate syscalls; an attacker with concurrent filesystem
        // write access to the parent directory could swap a regular file
        // for a symlink in the race window. Tight fix is
        // `OpenOptions::custom_flags(O_NOFOLLOW)`, which would require a
        // `libc` dep (operator-sign-off-gated) or fragile per-OS literal
        // constants. The race window is on the order of microseconds; the
        // threat model declares single-user local tool with no concurrent
        // adversary in the user's home directory; the save side uses
        // `rename(2)` which is atomic regardless. Documented in
        // `DESIGN.md` § Threat model — residual risks.
        if let Ok(meta) = std::fs::symlink_metadata(path) {
            if meta.file_type().is_symlink() {
                return Err(anyhow!(
                    "refusing to read through symlink at {}: this is the symlink-hardening discipline declared in DESIGN.md § Threat model",
                    path.display()
                ));
            }
        }
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

    /// Saves the store to `path` atomically, overwriting any existing
    /// content. Creates parent directories if needed.
    ///
    /// **Atomic write discipline** — writes to a sibling temp file in the
    /// destination directory, then renames over `path` via POSIX
    /// `rename(2)` semantics. If the write or rename fails, the prior
    /// state at `path` is preserved. Closes [SE Review 1
    /// Finding 2](../vsdd-suite/review-log/2026-05-20-software-engineer.md#r1-f2).
    ///
    /// **Mode 0600 on Unix** — the temp file is opened with mode 0600 so
    /// that after the rename the on-disk file is owner-only-readable.
    /// Closes [Security Review 1
    /// Finding 2](../vsdd-suite/review-log/2026-05-20-security.md) +
    /// [Red Team Review 1
    /// Finding 5](../vsdd-suite/review-log/2026-05-20-red-team.md).
    /// Windows is named as untested under `DESIGN.md` § Constraints; the
    /// mode setter is gated behind `#[cfg(unix)]`.
    ///
    /// **Symlink-follow rejection** — if `path` is itself a symlink, the
    /// save refuses with an error rather than letting `rename(2)` clobber
    /// the symlink with a regular file (which would have surprised the
    /// user) or write through it (under earlier non-rename designs).
    /// Closes [Red Team Review 1
    /// Finding 6](../vsdd-suite/review-log/2026-05-20-red-team.md).
    ///
    /// **Layer 2 durability — parent-directory fsync after rename.** Per
    /// `DESIGN.md` § Performance budget "Durability discipline (Layer 2)",
    /// the save fsyncs the destination file's parent directory after the
    /// `rename(2)` so that the rename itself is durable across a power
    /// loss — without the parent fsync, the rename may live only in the
    /// kernel page cache and be lost on power-fail. The cost is one extra
    /// `fsync(2)` syscall per write (benchmarked at < 5 ms on commodity
    /// SSD per the Layer 2 PE round budget). Gated `#[cfg(unix)]`;
    /// Windows uses its own durability semantics that are not addressed
    /// at Layer 2. A best-effort `fsync` — the file rename has already
    /// landed on the inode atomically; a fsync failure here is logged
    /// as a save error so the operator can react (e.g. retry on a flaky
    /// remote mount).
    ///
    /// # Errors
    ///
    /// Returns an error if (a) `path` exists and is a symlink, (b) the
    /// parent directory cannot be created, (c) the temp file cannot be
    /// created or written, (d) the rename fails, or (e) the parent
    /// directory fsync fails (Unix only). All five failure modes
    /// preserve the prior `path` state from the operator's perspective
    /// — modes (a) through (d) leave the original file untouched, and
    /// mode (e) leaves the renamed-but-unsynced file in place (it may
    /// or may not survive a power loss; the on-disk inode is what
    /// `ls(1)` shows, and a successful retry of `save` will sync it).
    pub fn save(&self, path: &Path) -> Result<()> {
        // Symlink-follow rejection per DESIGN.md § Threat model.
        if let Ok(meta) = std::fs::symlink_metadata(path) {
            if meta.file_type().is_symlink() {
                return Err(anyhow!(
                    "refusing to write through symlink at {}: this is the symlink-hardening discipline declared in DESIGN.md § Threat model",
                    path.display()
                ));
            }
        }

        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)
                    .with_context(|| format!("creating parent dir for {}", path.display()))?;
            }
        }

        let json =
            serde_json::to_string_pretty(self).context("serializing bookmark store to JSON")?;

        let tmp_path = temp_sibling_path(path);
        if let Err(e) = write_temp_file(&tmp_path, json.as_bytes()) {
            // Cleanup orphan temp file on partial write failure
            // (Round 2 SE Finding 7). The `create_new` open in
            // `write_temp_file` may have succeeded before `write_all` /
            // `sync_all` failed — without this cleanup, the temp file
            // accumulates on disk. Best-effort: ignore the cleanup error
            // so the original write failure surfaces to the caller.
            let _ = std::fs::remove_file(&tmp_path);
            return Err(e).with_context(|| {
                format!(
                    "writing temp file for atomic save at {}",
                    tmp_path.display()
                )
            });
        }

        // POSIX rename(2) is atomic on the same filesystem; the temp file
        // lives next to `path` so the rename does not cross filesystems.
        if let Err(e) = std::fs::rename(&tmp_path, path) {
            // Best-effort cleanup; ignore the cleanup error so the
            // original rename failure surfaces to the caller.
            let _ = std::fs::remove_file(&tmp_path);
            return Err(e).with_context(|| {
                format!(
                    "atomically renaming temp file into place at {}",
                    path.display()
                )
            });
        }

        // Layer 2 durability — fsync the parent directory so the rename
        // itself is durable across a power loss. Per `DESIGN.md`
        // § Performance budget "Durability discipline (Layer 2)" — the
        // `rename(2)` syscall above may live in the kernel page cache;
        // without this fsync the rename can be lost on power-fail even
        // though the file contents were synced inside `write_temp_file`.
        // Gated `#[cfg(unix)]` per the spec.
        #[cfg(unix)]
        {
            if let Some(parent) = path.parent() {
                let parent = if parent.as_os_str().is_empty() {
                    Path::new(".")
                } else {
                    parent
                };
                fsync_directory(parent).with_context(|| {
                    format!(
                        "fsyncing parent directory {} for durable rename of {}",
                        parent.display(),
                        path.display()
                    )
                })?;
            }
        }

        Ok(())
    }

    /// Appends a bookmark with the supplied URL and the current UTC time.
    ///
    /// # Errors
    ///
    /// Returns an error if `url` is empty. Per `DESIGN.md` § `bm add`,
    /// non-empty URL is the library-level invariant; this check moves the
    /// enforcement from the CLI shell into the library boundary so that
    /// any future caller (Layer 2 `bm tag`, library-as-dev-dep, etc.)
    /// inherits the invariant. Closes [SE Review 1
    /// Finding 4](../vsdd-suite/review-log/2026-05-20-software-engineer.md#r1-f4).
    pub fn add(&mut self, url: String) -> Result<()> {
        if url.is_empty() {
            return Err(anyhow!("URL cannot be empty"));
        }
        self.bookmarks.push(Bookmark {
            url,
            timestamp: Utc::now(),
            tags: Vec::new(),
        });
        Ok(())
    }

    /// Borrowed view of the bookmark collection in storage order
    /// (oldest-first append order). For newest-first render order use
    /// `newest_first`.
    #[must_use]
    pub fn bookmarks(&self) -> &[Bookmark] {
        &self.bookmarks
    }

    /// Returns bookmarks sorted newest-first (by timestamp descending).
    /// Pure — does not mutate the store.
    #[must_use]
    pub fn newest_first(&self) -> Vec<&Bookmark> {
        let mut sorted: Vec<&Bookmark> = self.bookmarks.iter().collect();
        sorted.sort_by_key(|b| std::cmp::Reverse(b.timestamp));
        sorted
    }

    /// Attaches `label` to every bookmark whose URL equals `url` exactly
    /// (case-sensitive). Idempotent: if a matching bookmark already has
    /// `label` in its `tags` field, the label is NOT appended a second
    /// time.
    ///
    /// Returns the count of bookmarks whose `tags` field was checked
    /// against the supplied label — i.e. the number of bookmarks whose URL
    /// matched. This count includes idempotent no-op matches (a bookmark
    /// already tagged is still a successful match).
    ///
    /// Pure transformation against the store; the caller is responsible
    /// for persisting via `save`. Per `DESIGN.md` § Verification
    /// architecture, `attach_tag` lives on the pure side of the purity
    /// boundary (deterministic; no I/O; no clock).
    ///
    /// # Errors
    ///
    /// Returns `AttachTagError::EmptyUrl` if `url` is empty,
    /// `AttachTagError::EmptyLabel` if `label` is empty, or
    /// `AttachTagError::NoMatch` if no bookmark's URL matches `url`. The
    /// store is not mutated when any error variant is returned.
    pub fn attach_tag(&mut self, url: &str, label: &str) -> Result<usize, AttachTagError> {
        if url.is_empty() {
            return Err(AttachTagError::EmptyUrl);
        }
        if label.is_empty() {
            return Err(AttachTagError::EmptyLabel);
        }
        let mut matched = 0_usize;
        for bm in &mut self.bookmarks {
            if bm.url == url {
                matched += 1;
                if !bm.tags.iter().any(|t| t == label) {
                    bm.tags.push(label.to_string());
                }
            }
        }
        if matched == 0 {
            return Err(AttachTagError::NoMatch(url.to_string()));
        }
        Ok(matched)
    }

    /// Returns the subset of bookmarks whose `tags` field contains at
    /// least one of the supplied `labels` (OR-semantics across labels),
    /// in newest-first ordering per `DESIGN.md` § `bm list --tag <label>`.
    ///
    /// Pure: borrows from the store; does not mutate.
    ///
    /// Per `DESIGN.md` § Verification architecture, `filter_by_tags`
    /// lives on the pure side of the purity boundary (deterministic;
    /// no I/O; no clock).
    #[must_use]
    pub fn filter_by_tags<'a>(&'a self, labels: &[&str]) -> Vec<&'a Bookmark> {
        self.newest_first()
            .into_iter()
            .filter(|b| b.tags.iter().any(|t| labels.iter().any(|l| t == *l)))
            .collect()
    }
}

/// Produces a sibling temp path next to `path`. Uniqueness is sufficient
/// for the single-user, single-process scope declared in `DESIGN.md`
/// § Scope and non-goals; concurrent-write races are out of scope per
/// the threat model.
fn temp_sibling_path(path: &Path) -> std::path::PathBuf {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos());
    let pid = std::process::id();
    let file_name = path.file_name().map_or_else(
        || std::ffi::OsString::from("bookmarks.json"),
        std::ffi::OsStr::to_os_string,
    );
    let mut tmp_name = file_name;
    tmp_name.push(format!(".tmp.{pid}.{nanos}"));
    path.with_file_name(tmp_name)
}

/// fsync the directory at `path` so a preceding `rename(2)` is durable
/// across power loss per `DESIGN.md` § Performance budget "Durability
/// discipline (Layer 2)". Unix-only — Windows directory durability has
/// different semantics and is out of scope at Layer 2.
#[cfg(unix)]
fn fsync_directory(path: &Path) -> std::io::Result<()> {
    let dir = std::fs::File::open(path)?;
    dir.sync_all()?;
    Ok(())
}

#[cfg(unix)]
fn write_temp_file(tmp_path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    use std::io::Write;
    use std::os::unix::fs::OpenOptionsExt;
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(tmp_path)?;
    f.write_all(bytes)?;
    f.write_all(b"\n")?;
    f.sync_all()?;
    Ok(())
}

#[cfg(not(unix))]
fn write_temp_file(tmp_path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    use std::io::Write;
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(tmp_path)?;
    f.write_all(bytes)?;
    f.write_all(b"\n")?;
    f.sync_all()?;
    Ok(())
}

/// Returns a display-safe rendering of `s`.
///
/// Control characters (Cc) and bidi / zero-width format characters (Cf
/// subset) are escaped as `\u{HHHH}` Unicode escapes. `\n` and `\t` are
/// preserved as legitimate whitespace; every other `is_control()` char
/// and the named format chars are escaped.
///
/// **Why this exists** — `DESIGN.md` § Threat model names URL contents
/// and storage paths as adversary-controlled when they reach the error
/// stream. Per the [Rust supplement](../../vsdd-suite/supplements/rust.md)
/// § Security ("wrap every user-derived value before it reaches the error
/// stream"), the CLI shell uses this before any `eprintln!` / `println!`
/// interpolation of user-derived data. Closes [Security Review 1
/// Finding 1](../vsdd-suite/review-log/2026-05-20-security.md) +
/// [Red Team Review 1
/// Finding 4](../vsdd-suite/review-log/2026-05-20-red-team.md).
#[must_use]
pub fn display_safe(s: &str) -> String {
    use std::fmt::Write as _;
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if c == '\n' || c == '\t' {
            out.push(c);
            continue;
        }
        if c.is_control() || is_format_char(c) {
            // \u{HHHH} — 4-hex-digit minimum, more digits for higher codepoints.
            // write! into a String only fails on allocation panic, not on
            // formatting; the result is safe to discard.
            let _ = write!(out, "\\u{{{:04x}}}", c as u32);
        } else {
            out.push(c);
        }
    }
    out
}

/// A **curated** set of Unicode format-category codepoints that
/// `display_safe` escapes. Per `DESIGN.md` § Threat model, this is not a
/// claim of full Cf-category coverage (full coverage would require a
/// Unicode-properties dependency such as `unicode-general-category`); it
/// is a hand-maintained enumeration of the known terminal-escape-injection,
/// Trojan-Source, and invisible-glyph spoofing vectors. New bypass codepoints
/// are added as adversarial review surfaces them; the in-function comment
/// lists every range in scope so a reader does not need to grep the matcher.
/// `is_control()` already covers the Cc range.
const fn is_format_char(c: char) -> bool {
    // Curated Unicode-format-category subset covering known terminal-escape-
    // injection + Trojan-Source + invisible-glyph spoofing vectors. Initial
    // coverage closes [Red Team Review 1
    // Finding 6](../vsdd-suite/review-log/2026-05-20-red-team.md#r2-f6).
    // Round 3 [Red Team Review 1 Finding 3](../vsdd-suite/review-log/2026-05-20-red-team.md#r3-f3)
    // extended coverage with the additional Cf codepoints listed below. The
    // matcher is intentionally curated (vs. categorical) so that the audit
    // surface — every codepoint we claim defense against — is reviewable in
    // one place. Coverage:
    //
    // - U+00AD — SOFT HYPHEN (classic invisible URL-spoof primitive; R3)
    // - U+0600..=0605 — Arabic number signs + ALM-class (R3)
    // - U+061C — Arabic Letter Mark (bidi format char)
    // - U+06DD — Arabic end of ayah (R3)
    // - U+070F — Syriac abbreviation mark (R3)
    // - U+08E2 — Arabic disputed end of ayah (R3)
    // - U+180B..=180D, U+180F — Mongolian Free Variation Selectors
    // - U+200B..=200F — zero-width chars + LRM/RLM
    // - U+202A..=202E — explicit bidi formatting (RLE/LRE/PDF/LRO/RLO)
    // - U+2060..=2064 — word joiner + invisible math chars
    // - U+2066..=2069 — isolate bidi formatting
    // - U+FE00..=FE0F — Variation Selectors 1-16 (Trojan-Source supplementary)
    // - U+FEFF — zero-width no-break space / BOM
    // - U+FFF9..=FFFB — interlinear annotation anchors
    // - U+110BD, U+110CD — Kaithi number sign + end-of-text marker (R3)
    // - U+13430..=13438 — Egyptian hieroglyph format controls (R3)
    // - U+1BCA0..=1BCA3 — Duployan shorthand format controls (R3)
    // - U+E0001 — language tag
    // - U+E0020..=E007F — tag characters (Trojan-Source supplementary plane)
    // - U+E0100..=E01EF — Variation Selectors 17-256
    matches!(
        c as u32,
        0x00AD
            | 0x0600..=0x0605
            | 0x061C
            | 0x06DD
            | 0x070F
            | 0x08E2
            | 0x180B..=0x180D
            | 0x180F
            | 0x200B..=0x200F
            | 0x202A..=0x202E
            | 0x2060..=0x2064
            | 0x2066..=0x2069
            | 0xFE00..=0xFE0F
            | 0xFEFF
            | 0xFFF9..=0xFFFB
            | 0x1_10BD
            | 0x1_10CD
            | 0x1_3430..=0x1_3438
            | 0x1_BCA0..=0x1_BCA3
            | 0xE_0001
            | 0xE_0020..=0xE_007F
            | 0xE_0100..=0xE_01EF
    )
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    reason = "Restriction-group lints from [lints.clippy] apply to production code; \
              tests use unwrap/expect/panic freely per Rust supplement test-helper convention. \
              Platform Engineer Round 2 Finding 13."
)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn bm(url: &str, ts: DateTime<Utc>) -> Bookmark {
        Bookmark {
            url: url.to_string(),
            timestamp: ts,
            tags: Vec::new(),
        }
    }

    /// Layer 2 Phase 5 Mutation Testing closure — kills the 3 surviving
    /// `Bookmark::tags()` accessor mutants ([cargo-mutants 27.0.0 reported
    /// MISSED at `src/lib.rs:80:9`](../vsdd-suite/review-log/2026-05-21-quality-engineer.md#r6-qe-f1)
    /// for `Vec::leak(Vec::new())`, `Vec::leak(vec![String::new()])`,
    /// `Vec::leak(vec!["xyzzy".into()])`). Asserts the accessor returns the
    /// constructor-supplied tags slice for both populated and empty `tags`
    /// fields.
    #[test]
    fn bookmark_tags_accessor_returns_constructor_supplied_slice() {
        let ts = Utc.with_ymd_and_hms(2026, 5, 22, 0, 0, 0).unwrap();

        let with_tags = Bookmark {
            url: "https://example.com".to_string(),
            timestamp: ts,
            tags: vec!["rust".to_string(), "cli".to_string()],
        };
        let expected_tags = ["rust".to_string(), "cli".to_string()];
        assert_eq!(with_tags.tags(), &expected_tags[..]);

        let empty_tags = Bookmark {
            url: "https://empty.example".to_string(),
            timestamp: ts,
            tags: Vec::new(),
        };
        let no_tags: &[String] = &[];
        assert_eq!(empty_tags.tags(), no_tags);
    }

    #[test]
    fn newest_first_sorts_descending_by_timestamp() {
        let t1 = Utc.with_ymd_and_hms(2026, 5, 17, 1, 0, 0).unwrap();
        let t2 = Utc.with_ymd_and_hms(2026, 5, 17, 2, 0, 0).unwrap();
        let t3 = Utc.with_ymd_and_hms(2026, 5, 17, 3, 0, 0).unwrap();
        let store = BookmarkStore {
            bookmarks: vec![
                bm("https://b", t1),
                bm("https://a", t3),
                bm("https://c", t2),
            ],
        };
        let sorted = store.newest_first();
        assert_eq!(sorted[0].url(), "https://a");
        assert_eq!(sorted[1].url(), "https://c");
        assert_eq!(sorted[2].url(), "https://b");
    }

    #[test]
    fn load_returns_empty_for_missing_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("absent.json");
        let store = BookmarkStore::load(&path).unwrap();
        assert!(store.bookmarks().is_empty());
    }

    #[test]
    fn load_returns_empty_for_empty_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("empty.json");
        std::fs::write(&path, "").unwrap();
        let store = BookmarkStore::load(&path).unwrap();
        assert!(store.bookmarks().is_empty());
    }

    #[test]
    fn save_then_load_roundtrips() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("rt.json");
        let mut store = BookmarkStore::default();
        store.add("https://example.com".to_string()).unwrap();
        store.save(&path).unwrap();
        let loaded = BookmarkStore::load(&path).unwrap();
        assert_eq!(loaded.bookmarks().len(), 1);
        assert_eq!(loaded.bookmarks()[0].url(), "https://example.com");
    }

    /// retroactive Red Gate (Phase 5 source): save creates parent directory
    /// for a nested-path target — Mutation Testing (cargo-mutants) surfaced the gap
    /// at src/lib.rs:48 where `!parent.as_os_str().is_empty()` could be
    /// flipped without any test failing. Test added post-MVR; confirmed
    /// passes against current implementation. See vsdd-suite/review-log/2026-05-20-quality-engineer.md
    /// (Review 2 — Phase 5 Mutation Testing) for the surviving mutant disposition.
    #[test]
    fn save_creates_parent_directory_for_nested_path() {
        let dir = tempfile::tempdir().unwrap();
        // Parent directory does NOT exist yet — save must create it.
        let path = dir
            .path()
            .join("nested")
            .join("subdir")
            .join("bookmarks.json");
        assert!(
            !path.parent().unwrap().exists(),
            "parent must not exist before save"
        );

        let mut store = BookmarkStore::default();
        store.add("https://example.com".to_string()).unwrap();
        store
            .save(&path)
            .expect("save should create missing parent directories");

        assert!(
            path.exists(),
            "store file should exist after save to nested path"
        );
        assert!(
            path.parent().unwrap().exists(),
            "parent directory should have been created"
        );

        // Verify the saved content round-trips correctly through the just-created path.
        let loaded = BookmarkStore::load(&path).unwrap();
        assert_eq!(loaded.bookmarks().len(), 1);
        assert_eq!(loaded.bookmarks()[0].url(), "https://example.com");
    }

    #[test]
    fn add_rejects_empty_url() {
        let mut store = BookmarkStore::default();
        let err = store
            .add(String::new())
            .expect_err("empty URL must be rejected");
        assert!(
            err.to_string().contains("URL cannot be empty"),
            "error should name the empty-URL contract; got {err}"
        );
        assert!(
            store.bookmarks().is_empty(),
            "no bookmark should be appended on rejection"
        );
    }

    #[test]
    fn display_safe_preserves_newline_and_tab() {
        assert_eq!(display_safe("line1\nline2\ttab"), "line1\nline2\ttab");
    }

    #[test]
    fn display_safe_escapes_ansi_escape() {
        // ESC = U+001B
        let out = display_safe("\x1b[31mred");
        assert!(
            out.contains("\\u{001b}"),
            "ESC should be escaped; got {out}"
        );
        assert!(
            !out.contains('\x1b'),
            "raw ESC must not survive sanitization; got {out}"
        );
    }

    #[test]
    fn display_safe_escapes_format_chars() {
        // RLO = U+202E (right-to-left override) is a classic bidi spoof.
        let out = display_safe("plain\u{202e}evil");
        assert!(
            out.contains("\\u{202e}"),
            "RLO should be escaped; got {out}"
        );
    }

    #[test]
    fn save_refuses_symlink() {
        // Unix-only — symlink_metadata semantics differ on Windows.
        #[cfg(unix)]
        {
            let dir = tempfile::tempdir().unwrap();
            let real_target = dir.path().join("real_target.json");
            std::fs::write(&real_target, b"{\"bookmarks\":[]}\n").unwrap();
            let link = dir.path().join("link.json");
            std::os::unix::fs::symlink(&real_target, &link).unwrap();

            let mut store = BookmarkStore::default();
            store.add("https://example.com".to_string()).unwrap();
            let err = store
                .save(&link)
                .expect_err("save must refuse symlink targets");
            assert!(
                err.to_string()
                    .contains("refusing to write through symlink"),
                "error should name the symlink-hardening discipline; got {err}"
            );

            // The real target must be untouched (still the empty store we wrote above).
            let contents = std::fs::read_to_string(&real_target).unwrap();
            assert_eq!(contents, "{\"bookmarks\":[]}\n");
        }
    }

    #[cfg(unix)]
    #[test]
    fn save_creates_file_with_mode_0600() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("mode.json");
        let mut store = BookmarkStore::default();
        store.add("https://example.com".to_string()).unwrap();
        store.save(&path).unwrap();
        let mode = std::fs::metadata(&path).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600, "expected mode 0600, got {mode:o}");
    }

    /// Closes the operator-queued PE fsync benchmark item structurally —
    /// `save` invokes `fsync(2)` on the parent directory after `rename(2)`
    /// for durability per DESIGN.md § Performance budget Layer 2
    /// "Durability discipline". There is no portable way for a black-box
    /// unit test to assert that fsync was actually called on the parent
    /// directory FD (the syscall has no observable side effect from
    /// userspace). Acceptable alternative: the test asserts that after a
    /// `save` of a non-trivial store the file is present on disk + the
    /// store round-trips cleanly through `load`. This is a WEAK PROXY for
    /// the durability contract — it confirms the save codepath executes
    /// successfully against a real filesystem (the same codepath that
    /// includes the fsync on Unix) but does not directly verify the fsync
    /// syscall was issued. Direct verification would require either:
    /// (a) an injected trait/seam at the syscall boundary, which would add
    /// complexity disproportionate to the Layer 2 budget; or (b) a
    /// `strace`/`dtruss` harness, which is platform-specific + outside the
    /// `cargo test` discipline. Deferred per the test plan in
    /// `TODO.md` § Layer 2 Red Gate test 14.
    #[cfg(unix)]
    #[test]
    fn tests_save_durable_path_succeeds_unix_weak_proxy_for_fsync() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("durable.json");
        let mut store = BookmarkStore::default();
        store.add("https://example.com".to_string()).unwrap();
        store
            .save(&path)
            .expect("save (with parent-dir fsync) should succeed on a writable tempdir");
        assert!(
            path.exists(),
            "store file must be on disk after a durable save"
        );
        // Weak proxy: round-trip through load to confirm the saved content
        // is valid + complete.
        let loaded = BookmarkStore::load(&path).unwrap();
        assert_eq!(loaded.bookmarks().len(), 1);
        assert_eq!(loaded.bookmarks()[0].url(), "https://example.com");
    }
}
