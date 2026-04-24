# Bookmark Manager — TODO

Tasks are marked complete only after all acceptance criteria pass automated tests.
Unit tests cover pure logic in isolation. Browser tests verify end-to-end behavior: form interaction, rendered output, localStorage state, and link attributes.
Code inspection and successful compilation are not sufficient — the tests must exist and pass.

Before moving to the next layer, complete the manual testing checklist for that layer and run an adversarial QA review using the prompt in ADVERSARIAL.md. The review must confirm:
- All acceptance criteria for the completed layer are verified by tests
- `src/bookmarks.ts` maintains 100% statement, branch, and function coverage (`npm run test:coverage`)
- No dead exports or unreachable code introduced in this layer
- No unused dependencies added
- All findings logged in ADVERSARIAL.md with resolution or dismissal rationale

---

## Layer 1: Core

- [x] Set up project structure (`index.html`, `styles.css`, `main.ts`, `tsconfig.json`)
  - `index.html`, `styles.css`, `main.ts`, and `tsconfig.json` all exist in the project root
  - `tsc` compiles without errors
  - Opening `index.html` in a browser renders without console errors

- [x] Define bookmark data type in TypeScript
  - A `Bookmark` interface exists with at minimum `id`, `url`, `title`, and `createdAt` fields
  - TypeScript compiles with strict mode enabled

- [x] Add a bookmark with URL and title
  - Filling in a valid URL and title and submitting adds exactly one bookmark to the list
  - The new bookmark's title text and `href` attribute match the submitted values
  - The new bookmark appears at the top of the list immediately after submission
  - The form clears all fields (title, URL, note, tags) after a successful submission
  - When validation fails, the form is not cleared — all user input is preserved

- [x] Display bookmarks in a list, newest first
  - Adding multiple bookmarks shows them in reverse chronological order
  - The most recently added bookmark is always at the top
  - When two bookmarks share an identical timestamp, their relative order is deterministic (sorted by ID as a tiebreaker)
  - Adding a second bookmark shows exactly two items; neither is duplicated or removed

- [x] Click a bookmark to open it in a new tab
  - The bookmark title is an `<a>` element with `href` set to the saved URL, `target="_blank"`, and `rel="noopener noreferrer"`
  - The current tab does not navigate away

- [x] Persist bookmarks in localStorage
  - Adding a bookmark, then refreshing the page, still shows the bookmark with the correct title and URL
  - A browser test reads `localStorage.getItem('bookmarks')` directly, parses it as JSON, and verifies `id`, `title`, `url`, and `createdAt` fields are present

- [x] Validate: reject empty titles
  - Submitting the form with an empty title field does not add a bookmark
  - An error message is displayed when the title is empty

- [x] Validate: reject invalid URLs
  - Submitting a URL that does not start with `http://` or `https://` does not add a bookmark
  - An error message is displayed for invalid URLs
  - Submitting a URL starting with `http://` or `https://` succeeds
  - Uppercase protocols (`HTTP://`, `HTTPS://`) are accepted as valid
  - A protocol-only URL with no domain (e.g. `https://`) is rejected
  - Unit tests cover all of: no protocol, `ftp://`, empty string, protocol-only, `http://`, `https://`, `HTTP://`, `HTTPS://`

**Layer 1 manual testing checklist:**
- [ ] Open the app — page loads with no visible errors and the add form is present
- [ ] Submit the form with a valid title and URL — bookmark appears at the top of the list
- [ ] Verify the bookmark title is a clickable link — clicking it opens the correct URL in a new tab without navigating away
- [ ] Add a second bookmark — both are visible, newest is at the top, count is exactly 2
- [ ] Refresh the page — both bookmarks are still present with correct titles and URLs
- [ ] Submit the form with the title field empty — error message appears, no bookmark is added, the URL you typed is still in the field
- [ ] Submit the form with a URL that has no protocol (e.g. `example.com`) — error message appears, no bookmark is added, the title you typed is still in the field
- [ ] Submit the form with an uppercase protocol (e.g. `HTTPS://example.com`) — bookmark is accepted and added successfully
- [ ] Submit the form with `https://` as the URL (no domain) — error message appears, no bookmark is added
- [ ] After a successful submission, verify all form fields are empty (title, URL, note, tags)
- [ ] After a failed submission, verify the error message disappears after a subsequent successful submission

**Layer 1 QA review:** Completed 2026-04-23. See ADVERSARIAL.md Review 1.

---

## Layer 2: Notes and Tags

- [x] Add optional note field to the add form
  - A note textarea is present in the add form
  - Submitting without a note is allowed and does not produce an error

- [x] Add optional tags field to the add form (comma-separated input)
  - A tags input is present in the add form
  - Entering `work, reading, tools` stores tags as `["work", "reading", "tools"]` (trimmed, no empty entries)
  - Submitting without tags is allowed and does not produce an error

- [x] Display note under each bookmark's title
  - A bookmark saved with a note shows the note text below the title
  - A bookmark saved without a note shows nothing in the note area

- [x] Display tags as badges on each bookmark
  - A bookmark with tags shows each tag as a distinct badge
  - A bookmark without tags shows no badges

**Layer 2 manual testing checklist:**
- [ ] Add a bookmark with a note — note text appears below the title in the list
- [ ] Add a bookmark without a note — no note area is visible for that bookmark
- [ ] Add a bookmark with tags (e.g. `work, reading, tools`) — three separate tag badges appear on the bookmark
- [ ] Add a bookmark without tags — no tag badges are visible for that bookmark
- [ ] Add a bookmark with extra whitespace in tags (e.g. `  work  ,  reading  `) — tags are trimmed and displayed correctly
- [ ] Verify that a bookmark with a note and tags and one without are both displayed correctly in the same list
- [ ] Refresh the page — note and tags are still present on the correct bookmarks

**Layer 2 QA review:** Completed 2026-04-23. See ADVERSARIAL.md Reviews 1 and 2.

---

## Layer 3: Edit and Delete

- [ ] Add edit button to each bookmark
  - Each bookmark has a visible edit button
  - The edit button is present for every bookmark in the list, not just the first

- [ ] Inline editing of title, URL, note, and tags
  - Clicking edit makes title, URL, note, and tags fields editable in place, pre-populated with current values
  - Saving the edit updates the bookmark in the list immediately; the displayed title, URL, note, and tags all reflect the new values
  - Canceling the edit leaves the bookmark unchanged; the original values are still displayed
  - Saving an edit with an empty title shows an error and does not save
  - Saving an edit with an invalid URL shows an error and does not save
  - The bookmark count does not change after a successful edit

- [ ] Persist edits to localStorage
  - Editing a bookmark and refreshing the page shows the updated title, URL, note, and tags
  - A browser test reads `localStorage.getItem('bookmarks')` directly after an edit and verifies the stored values match the edited values
  - The total number of bookmarks in localStorage does not change after an edit

- [ ] Add delete button to each bookmark
  - Each bookmark has a visible delete button
  - The delete button is present for every bookmark in the list, not just the first

- [ ] Confirm before deleting
  - Clicking delete shows a confirmation prompt before removing the bookmark
  - Confirming removes exactly the targeted bookmark from the list; all other bookmarks remain
  - Canceling leaves the bookmark list unchanged

- [ ] Persist deletions to localStorage
  - Deleting a bookmark and refreshing the page shows it is gone
  - A browser test reads `localStorage.getItem('bookmarks')` directly after deletion and verifies the deleted bookmark's `id` is no longer present
  - Remaining bookmarks are still present in localStorage after deletion

**Layer 3 manual testing checklist:**
- [ ] Each bookmark has a visible edit button
- [ ] Clicking edit on a bookmark reveals editable fields pre-populated with the current title, URL, note, and tags
- [ ] Edit the title and save — the updated title appears in the list immediately
- [ ] Edit the URL, note, and tags and save — all updated values appear correctly
- [ ] Refresh after saving an edit — the updated values persist
- [ ] Click edit, change a value, then cancel — the original values are unchanged
- [ ] While editing, clear the title and try to save — an error appears and the edit is not saved
- [ ] While editing, enter an invalid URL and try to save — an error appears and the edit is not saved
- [ ] Verify the total number of bookmarks does not change after a successful edit
- [ ] Each bookmark has a visible delete button
- [ ] Click delete on a bookmark — a confirmation prompt appears before anything is removed
- [ ] Confirm the deletion — only the targeted bookmark is removed; others remain
- [ ] Cancel the deletion — the bookmark is still present
- [ ] Refresh after deleting — the deleted bookmark is gone; remaining bookmarks are still present

**Layer 3 QA review:** Pending.

---

## Layer 4: Tag Filtering

- [ ] Display all unique tags as clickable filter buttons above the list
  - All tags from all bookmarks appear as individual filter buttons — one button per unique tag, no duplicates
  - Tags that appear across multiple bookmarks produce only one filter button
  - Adding a bookmark with a new tag causes that tag's button to appear immediately
  - When all bookmarks are deleted, the tag filter area is empty (no stale buttons remain)
  - Unit tests cover the tag deduplication and extraction logic in isolation

- [ ] Filter bookmark list when a tag is clicked
  - Clicking a tag button shows only bookmarks that include that tag
  - The exact count of matching bookmarks is shown — no extras, no omissions
  - Bookmarks that do not have that tag are not shown
  - If no bookmarks match the active tag, the list is empty (not showing all bookmarks)

- [ ] Add "All" button to clear the active tag filter
  - An "All" button is always present, including when no bookmarks exist
  - Clicking "All" removes the active filter and shows all bookmarks
  - The total count of bookmarks shown after clicking "All" equals the total count of bookmarks

- [ ] Visually highlight the active tag filter
  - The currently active filter button has a distinct CSS class or attribute compared to inactive buttons
  - "All" has the active style when no tag filter is active
  - The previously active button loses the active style when a different filter is clicked

**Layer 4 manual testing checklist:**
- [ ] Add bookmarks with different tags — a filter button appears for each unique tag above the list
- [ ] Add two bookmarks with the same tag — only one filter button appears for that tag (no duplicates)
- [ ] An "All" button is always present and highlighted when no filter is active
- [ ] Click a tag filter — only bookmarks with that tag are shown; the filter button is visually highlighted
- [ ] Click "All" — all bookmarks are shown again and "All" is highlighted
- [ ] Click one tag filter then another — the list updates correctly and only the newly clicked filter is highlighted
- [ ] Delete all bookmarks with a given tag — that tag's filter button disappears
- [ ] Add a new bookmark while a tag filter is active — the list behaves correctly (new bookmark shown if it matches the filter, hidden if not)

**Layer 4 QA review:** Pending.

---

## Layer 5: Search

- [ ] Add search bar above the bookmark list
  - A search input is visible above the bookmark list at all times, including when the bookmark list is empty

- [ ] Filter bookmarks in real time as the user types
  - The bookmark list updates on each keystroke without requiring a submit action
  - Clearing the search input restores the full unfiltered list (or tag-filtered list if one is active)
  - Unit tests cover the search/filter logic in isolation against known bookmark fixtures

- [ ] Match against title and note content
  - Searching for a word present in a bookmark's title shows that bookmark
  - Searching for a word present in a bookmark's note shows that bookmark
  - A bookmark matches if the term appears in the title OR the note — both are checked
  - Searching for a word not present in any title or note produces an empty list, not all bookmarks
  - Search is case-insensitive: searching `example` matches a bookmark titled `Example`
  - Unit tests cover: title match, note match, no match, case-insensitive match, empty query returning all results

- [ ] Search and tag filter work together
  - With a tag filter active, search results are limited to bookmarks that match both the tag and the search term
  - A bookmark must satisfy both conditions to be shown — matching one is not sufficient
  - Clearing the search input while a tag filter is active returns to the tag-filtered view
  - Clearing the tag filter while a search is active returns to the search-filtered view
  - Unit tests cover the combined filter logic: tag only, search only, both active, neither active

**Layer 5 manual testing checklist:**
- [ ] The search bar is visible above the bookmark list at all times, including when the list is empty
- [ ] Type a word that appears in a bookmark's title — that bookmark is shown; non-matching bookmarks are hidden
- [ ] Type a word that appears in a bookmark's note — that bookmark is shown
- [ ] Type a word that appears in neither title nor note of any bookmark — the list is empty (not showing all bookmarks)
- [ ] Search is case-insensitive — searching `example` matches a bookmark titled `Example`
- [ ] Clear the search input — all bookmarks are shown again
- [ ] With a tag filter active, type a search term — only bookmarks matching both the tag and the search term are shown
- [ ] With a tag filter and search active, clear the search — bookmarks matching the tag filter are shown
- [ ] With a tag filter and search active, click "All" — bookmarks matching the search term are shown

**Layer 5 QA review:** Pending.

---

## Layer 6: Polish

- [ ] Dark color scheme
  - Background is dark, text is light
  - Text and background color combinations meet WCAG AA contrast ratio (4.5:1 for normal text)
  - Browser test verifies the page loads without console errors after style changes

- [ ] Collapsible add form
  - The add form is hidden and a "+" button is visible on page load
  - Clicking "+" makes the add form visible
  - The form collapses automatically after a bookmark is successfully added
  - The form does not collapse on validation failure — the user's input remains visible
  - Clicking "+" while the form is already open does not produce a broken state

- [ ] Extract and display domain name from URL on each bookmark
  - A bookmark with URL `https://example.com/some/path` shows `example.com` as a domain label
  - A bookmark with URL `http://sub.domain.co.uk/path` shows `sub.domain.co.uk`
  - The domain label is present for every bookmark in the list, not just the first
  - Unit tests cover domain extraction: standard URLs, subdomains, paths, query strings

- [ ] Smooth transitions when filtering and searching
  - Filtering by tag and searching animate the list change rather than snapping instantly
  - The transition does not break the visible count of bookmarks before and after

- [ ] Responsive layout (360px minimum width)
  - At 360px viewport width all UI elements are visible and usable
  - No horizontal scrolling is required at 360px
  - The add form, bookmark list, tag filters, and search bar are all usable at 360px

**Layer 6 manual testing checklist:**
- [ ] The color scheme is dark — background is dark, text is clearly readable against it
- [ ] The page loads showing a "+" button, not the full add form
- [ ] Click "+" — the add form expands and is ready to use
- [ ] Add a bookmark — the form collapses automatically after successful submission
- [ ] Open the form, enter values, submit with the title empty — the form stays open with the input preserved
- [ ] Each bookmark shows a domain label (e.g. `example.com`) that is visually smaller or secondary to the title
- [ ] Verify the domain label is correct for a URL with a path (e.g. `https://example.com/some/path` shows `example.com`)
- [ ] Verify the domain label is correct for a subdomain URL (e.g. `https://sub.example.com` shows `sub.example.com`)
- [ ] Resize the browser to 360px wide — all UI elements (form, list, search bar, tag filters) are visible and usable with no horizontal scrollbar
- [ ] Filter or search with bookmarks present — the list change is animated rather than instant

**Layer 6 QA review:** Pending.
