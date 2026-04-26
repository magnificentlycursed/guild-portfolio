# Bookmark Manager — Process Log

## What I Built and Why

A personal bookmark manager: save links with titles, notes, and tags, filter by tag, search by keyword, edit and delete. It runs in the browser and stores everything in localStorage with no backend. I actually use tools like this, so it felt like a real problem rather than a toy exercise.

The scope ended up larger than I expected going in. The project grew from plain JavaScript to TypeScript, added Vite as a build tool, Vitest for unit tests, Playwright for browser tests, ESLint, and a GitHub Actions CI pipeline. It also went through five rounds of formal adversarial review across QA, UX, Security, Platform Engineering, and Solution Architecture domains. The final design is meaningfully better than the first version for all of that pressure.

---

## Build Process

### Layer 1: Core

Added a bookmark with URL and title, displayed the list newest first, persisted to localStorage.

This went smoothly at a code level, but the first adversarial QA review found several real bugs once I looked hard:

- **URL validation was case-sensitive.** `HTTPS://example.com` was rejected because the check used `.startsWith('http://')` directly. Fixed by switching to `new URL(url)`, which normalizes the protocol to lowercase automatically.
- **Sort was unstable for identical timestamps.** Two bookmarks added in the same millisecond would appear in undefined order on every render. Fixed by adding a secondary sort key: `|| a.id.localeCompare(b.id)`.
- **Tests weren't verifying what they claimed.** The "adds a bookmark" test only checked that an item appeared — it didn't verify the `href` attribute. The "persists" test didn't actually inspect localStorage. I rewrote these to check the things they were supposed to check.

I also refactored storage access early: the initial version called `localStorage` directly inside `loadBookmarks` and `saveBookmarks`, which made unit testing impossible without a browser environment. Adding a `BookmarkStorage` interface that both `localStorage` and a simple test mock could satisfy let me run unit tests in pure Node.js. That change was worth doing at Layer 1 because it would have been painful to retrofit later.

### Layer 2: Notes and Tags

Added the optional note textarea and tags field to the add form, displayed note text and tag badges on each bookmark.

Worked on the first try. The only thing I had to be deliberate about was the data model — I made `note` a non-optional empty string rather than `note?: string`, which eliminated null checks throughout the rest of the codebase. Every place that reads `bookmark.note` can just use the value directly.

### Layer 3: Edit and Delete

Added inline editing (click Edit, get a form pre-populated with current values, save or cancel) and delete with a confirmation step. Changes and deletions persist to localStorage.

The first version used `window.confirm` for delete confirmation, which works but is jarring — the browser dialog doesn't identify which bookmark you're about to delete and the styling is completely inconsistent with the rest of the app. I kept it for Layer 3 and replaced it in Layer 6.

A QA review found that my edit test wasn't verifying enough: it checked that the title updated but didn't check the URL, note, or tags. It also hadn't tested clearing a note or tags during an edit (the conditional rendering for those fields was untested). Fixed both.

One thing that bit me: the inline edit form I constructed in JavaScript had `<label>` and `<input>` as adjacent siblings with no `for`/`id` association. Visually fine. Programmatically, screen readers couldn't link them — axe caught this as a critical violation later. I should have thought about label association while building the form.

### Layer 4: Tag Filtering

Added a filter bar above the list: an "All" button, plus one button per unique tag across all bookmarks. Clicking a tag filters the list. Active filter is highlighted. Filter and search compose.

An adversarial review found a state management bug: if you had a tag filter active and deleted all bookmarks with that tag, the `activeTag` variable still held the deleted tag name, but no filter button existed for it. The result was no button highlighted and an invisible filter still active — the user was stranded. Fixed by checking at render time whether `activeTag` still exists in the tag list and resetting to `null` if not.

UX review caught that clicking an active tag did nothing — you had to click "All" to deselect. That's not the expected behavior in any filter UI I've ever used. Fixed with a one-line change: `activeTag = activeTag === tag ? null : tag`.

### Layer 5: Search

Added a search bar that filters in real time against title and note content. Search and tag filter compose: both conditions must be satisfied.

A few things I wouldn't have caught without review:

- **Safari search input appearance.** `input[type="search"]` gets a pill shape in Safari by default, overriding border and border-radius. Fixed with `appearance: none`.
- **No screen reader announcement when results change.** An `aria-live` attribute on the list itself would announce every item on every keystroke — unusable. The correct pattern is a separate visually-hidden status region that announces only the result count. Added `<p id="search-status" class="sr-only" aria-live="polite" aria-atomic="true">` and updated it in `renderBookmarks()`.
- **No search landmark.** Added `role="search"` to the search bar wrapper so keyboard users navigating by landmark can jump directly to it.

### Layer 6: Polish

Dark color scheme, collapsible add form, domain label below each bookmark title, smooth transitions wrapped in `prefers-reduced-motion`, touch targets ≥44px, inline delete confirmation, tag badges as clickable filter shortcuts.

This was the most design-intensive layer. A few things:

The dark color scheme required verifying each color pair against the WCAG AA contrast formula (4.5:1 for normal text). I did this numerically rather than relying on visual judgment. I'm glad I did — the first pass at `.list-empty` text was `#888` on white: 3.54:1, fails. An earlier UX review had incorrectly dismissed that as acceptable under the large-text exception; it wasn't, because 14.4px normal weight doesn't qualify. axe confirmed the violation.

Replacing `window.confirm` with inline confirmation was more involved than it sounds. The confirmation div replaces the bookmark's action buttons in place (no full list re-render), which meant the cancel path had a focus management problem: clicking Cancel called `renderBookmarks()`, which destroyed and rebuilt the list DOM, and focus fell on `document.body`. A keyboard user was stranded. Fixed by having the cancel handler focus the newly rendered Delete button for the same bookmark after the re-render. The same bug existed in the edit cancel path since Layer 3 — caught in the Layer 6 IAR run and fixed at the same time.

The `extractDomain` function that pulls the hostname from a URL lives in `bookmarks.ts` alongside the other pure logic. It uses `new URL(url).hostname`, which strips the path, query, port, and protocol in one call. The try/catch returns `''` for malformed URLs so the caller can safely skip rendering the domain label when there's nothing to show.

---

## What I Learned

**The design doc is worth arguing with.** The first draft would have produced a workable app. The constraints I added during refinement — deterministic sort, form state preservation on failure, case-insensitive URL validation — caught real bugs before any code existed. Writing "what does this actually mean?" before building is faster than debugging it afterward.

**Testing infrastructure is load-bearing.** Adding Vitest and Playwright early made every subsequent layer faster to verify and every bug easier to isolate. Pure functions in `bookmarks.ts` that are fully unit-tested are a stable foundation — when something broke in Layer 5, the unit tests told me the logic was correct and the bug was in the DOM wiring.

**Adversarial review finds things you genuinely cannot find yourself.** Not because you're incompetent, but because you're inside the work. The ghost `activeTag` bug in Layer 4, the label association on the edit form, the `.list-empty` contrast failure, the screen reader gaps in Layer 5, the cancel focus management in Layer 6 — I wouldn't have found most of those without structured adversarial pressure. The axe integration was particularly high signal: it flagged the unlabeled form inputs as a critical violation and the contrast failure precisely, where manual review had gotten both wrong.

**Icons vs text is a real decision.** I used text buttons ("Edit", "Delete") throughout rather than icon buttons. Icons would be more compact; text is unambiguous without a tooltip or aria-label. I don't regret the choice, but I made it by default rather than intentionally. Next time I'd think about it at design doc time.

---

## Known Issues

**Single-browser automated testing.** Playwright tests run against Chromium only. Firefox and Safari compatibility was verified manually, and the known Safari quirk (`input[type="search"]` appearance) was fixed, but cross-browser regressions could go undetected in CI.

**Tag badge touch targets.** Tag badges (`.tag-badge`) are smaller than the 44px minimum touch target. They're a secondary shortcut — the primary filter path is the dedicated filter bar, which does meet the minimum. Accepted for now.

**No keyboard shortcut for adding a bookmark.** Getting to the add form requires tabbing through the filter bar and search input. Not a barrier, but not efficient.

**No export or import.** Out of scope per the design doc. If localStorage is cleared (browser settings, private browsing session ending, storage quota), bookmarks are gone with no recovery path. For a single-user personal tool this is an accepted risk; worth noting.

---

## Review History

See `iterative-adversarial-refinement/` for the full IAR suite logs across eight review domains (Quality Engineering, Software Engineering, Solution Owner, Solution Architect, Platform Engineering, Security, UX, VDD-IAR Alignment). Nine QA/QE reviews, six UX reviews, four Security reviews, four PE reviews, three SA reviews, and one each of SE, SO, and VDD-IAR Alignment were completed. Every finding is logged with resolution or dismissal rationale.
