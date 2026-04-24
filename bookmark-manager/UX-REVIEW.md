# UX Review Log

---

## Review 2 — 2026-04-24 20:24Z

### Prompt

> You are a UX expert reviewing a TypeScript bookmark manager web app built with HTML, CSS, and vanilla TypeScript. No frameworks. Layers 1–4 are complete (add, display, persist, notes, tags, edit, delete, tag filtering). Read every relevant file — source, styles, HTML.
>
> Evaluate:
> 1. Are there missing empty states? What does the user see when content is absent?
> 2. Are error messages clear, correctly placed, and do they clear at the right time?
> 3. Does keyboard and focus behavior feel natural? Do interactive elements receive focus at the right moment?
> 4. Are there visual inconsistencies between equivalent UI surfaces (e.g., add form vs. edit form)?
> 5. Are interactive affordances clear? Do users know what they can interact with?
> 6. Are there missing or confusing feedback patterns (success, loading, empty, error)?
> 7. Are there any WCAG AA accessibility issues in the current implementation?
>
> Be specific. Cite file names and element selectors. Prioritize by user impact. Separate findings into: fix now, log for future layer, dismiss.

---

### Findings — Fix Now

#### Finding 1 — No empty state when bookmark list is empty

**Impact:** High. On first load with no bookmarks, and after deleting all bookmarks, the list area is blank. No message, no prompt, no explanation of what happened. Users are left in an ambiguous state: is something loading? Did a delete fail? Standard pattern for all lists is an empty state message.

**Resolution:** Added `<li class="list-empty">No bookmarks yet. Add one above.</li>` to the list when `sorted.length === 0`. Styled with muted color and generous padding. Note: with the Layer 4 `activeTag` auto-reset fix, a filtered-but-empty state (filter active, no matches) is structurally impossible — the `activeTag` is always reset before rendering. The empty state message therefore always means "no bookmarks in storage."

#### Finding 2 — Add form error message persists while user is typing to fix it

**Impact:** Medium. Submit with an invalid title, see "Title cannot be empty." Start typing in the title field — the red error stays visible. Standard UX pattern: clear the error as soon as the user starts correcting input, not on the next submit. Leaving it visible while typing creates false urgency and makes the interaction feel sticky.

**Resolution:** Added `form.addEventListener('input', () => { errorEl.textContent = ''; })` in `DOMContentLoaded`. Error clears as soon as any field in the add form receives input.

#### Finding 3 — Edit form does not focus the first field on open

**Impact:** Medium. Clicking Edit opens an inline form. Keyboard focus stays on the now-replaced Edit button, outside the new form. Users navigating by keyboard must tab through the page to reach the first editable field. Web accessibility guideline: when a dialog or inline form is injected, move focus to the first actionable element.

**Resolution:** After `li.appendChild(form)`, added `form.querySelector('input, textarea')?.focus()`.

Also added `form.addEventListener('input', () => { errorEl.textContent = ''; })` to the edit form for the same reason as Finding 2.

#### Finding 4 — Edit form missing "(optional)" hints on Note and Tags fields

**Impact:** Low. The add form labels these fields as "Note (optional)" and "Tags (optional, comma-separated)". The dynamically-constructed edit form labels them as "Note" and "Tags". Users editing a bookmark have less context than users adding one — a minor inconsistency.

**Resolution:** Added `hint` field to the edit form field descriptor array. Label construction now appends a `<span class="optional">` when a hint is present, matching the add form's structure.

---

### Findings — Deferred (log in TODO.md)

#### Tag badges on bookmark items are not interactive

The tag badges displayed on each saved bookmark are `<span>` elements. Clicking a tag badge on a bookmark is a natural shortcut to activate that tag filter — users familiar with similar apps (Notion, Pinboard) expect this. Deferred to Layer 5 or Layer 6 as a polish item.

#### `window.confirm` for delete confirmation

The native `window.confirm` dialog is jarring — it blocks the page, is styled by the browser, and is visually disconnected from the app. Deferred to Layer 6 (Polish) as a custom confirmation UI.

#### Error message not cleared between failed submissions on different fields

If a user submits with an empty title (error: "Title cannot be empty"), then fixes the title but leaves the URL empty, the error updates on submit. This is correct. However, the error persists between form field interactions until the input listener fires. The current implementation addresses the common case. Deferred.

---

### Dismissed Findings

#### `<input type="url">` vs `<input type="text">` for URL field

Using `type="url"` would give mobile users the URL keyboard with `.com` shortcut. However, `type="url"` also triggers browser-native validation that produces inconsistent messages across browsers and fires before our custom validation. Keeping `type="text"` maintains control over the validation message and behavior. Dismissed.

#### Color contrast

`#0066cc` on white (bookmark links): 4.54:1 — passes WCAG AA. `#cc0000` on white (errors): 5.92:1 — passes. `#fff` on `#0066cc` (active filter button): 4.54:1 — passes. Dismissed.

---

## Review 1 — 2026-04-24

### Question

> When testing I expected clicking a tag to deselect it. Clicking a tag again should deselect it. What if I click multiple tags? That should either show things that match both tags or things that match either. Should it be AND or OR? Review this as a user experience expert that values intuitive and clear user interfaces. You know how users expect UI to behave based on familiar design patterns and human interface guidelines.

---

### Finding 1 — Toggle deselect: implement it

Clicking an active filter to deselect it is expected behavior in virtually every filter UI: iOS, Android, e-commerce facets, music apps. Not supporting it means the only way to clear a filter is to click "All," which adds friction and feels like a bug to most users.

**Decision:** Implemented. Clicking an active tag button now toggles it off and returns to the full "All" view.

---

### Finding 2 — Multi-select AND vs OR: OR is correct, but deferred

**OR (show bookmarks matching any selected tag)**
- Matches how most users think about filter pill buttons — "I want to see stuff from any of these buckets"
- Returns more results; less likely to leave users with an empty list
- Standard in content apps: e-commerce filter bars, Pinterest, social media tag feeds
- Lower cognitive load — users don't need to think about tag overlap in their data

**AND (show bookmarks matching all selected tags)**
- More powerful for a well-organized tag system: "find things tagged both `work` and `reading`"
- Standard in knowledge management tools: Notion, Bear, DEVONthink
- Higher risk of zero results in a casual bookmark manager
- Requires users to have deliberately applied overlapping tags, which casual users often haven't

**Recommendation:** OR is the right model for a bookmark manager with pill-button UI. AND is a power-user feature that belongs in a more structured tool.

**Decision:** Multi-select deferred. The single-select model is clean and unambiguous. Before adding multi-select, the value depends on whether bookmarks are tagged in a way that makes selecting multiple tags useful. This is a candidate for Layer 6 polish or a future layer.
