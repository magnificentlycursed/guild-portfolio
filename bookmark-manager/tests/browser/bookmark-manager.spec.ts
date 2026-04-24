import { test, expect } from '@playwright/test';

test.beforeEach(async ({ page }) => {
  await page.goto('/');
  await page.evaluate(() => localStorage.clear());
  await page.reload();
});

// ---------------------------------------------------------------------------
// Layer 1: Core
// ---------------------------------------------------------------------------

test('page loads without console errors', async ({ page }) => {
  const errors: string[] = [];
  page.on('console', msg => { if (msg.type() === 'error') errors.push(msg.text()); });
  await page.goto('/');
  expect(errors).toHaveLength(0);
});

test('add form is visible with title and URL fields', async ({ page }) => {
  await expect(page.locator('#add-form')).toBeVisible();
  await expect(page.locator('input[name="title"]')).toBeVisible();
  await expect(page.locator('input[name="url"]')).toBeVisible();
});

test('adds a bookmark with the correct title and URL', async ({ page }) => {
  await page.fill('input[name="title"]', 'Example Site');
  await page.fill('input[name="url"]', 'https://example.com');
  await page.click('button[type="submit"]');

  await expect(page.locator('.bookmark-item')).toHaveCount(1);
  await expect(page.locator('.bookmark-title')).toHaveText('Example Site');
  await expect(page.locator('.bookmark-title')).toHaveAttribute('href', 'https://example.com');
});

test('form clears all fields after successful submission', async ({ page }) => {
  await page.fill('input[name="title"]', 'Example Site');
  await page.fill('input[name="url"]', 'https://example.com');
  await page.fill('textarea[name="note"]', 'Some note');
  await page.fill('input[name="tags"]', 'tag1, tag2');
  await page.click('button[type="submit"]');

  await expect(page.locator('input[name="title"]')).toHaveValue('');
  await expect(page.locator('input[name="url"]')).toHaveValue('');
  await expect(page.locator('textarea[name="note"]')).toHaveValue('');
  await expect(page.locator('input[name="tags"]')).toHaveValue('');
});

test('new bookmark appears at the top and older bookmark remains below', async ({ page }) => {
  await page.fill('input[name="title"]', 'First');
  await page.fill('input[name="url"]', 'https://first.com');
  await page.click('button[type="submit"]');

  await page.fill('input[name="title"]', 'Second');
  await page.fill('input[name="url"]', 'https://second.com');
  await page.click('button[type="submit"]');

  await expect(page.locator('.bookmark-item')).toHaveCount(2);
  await expect(page.locator('.bookmark-title').nth(0)).toHaveText('Second');
  await expect(page.locator('.bookmark-title').nth(1)).toHaveText('First');
});

test('bookmark link has correct href and opens in a new tab', async ({ page }) => {
  await page.fill('input[name="title"]', 'Example');
  await page.fill('input[name="url"]', 'https://example.com');
  await page.click('button[type="submit"]');

  const link = page.locator('.bookmark-title');
  await expect(link).toHaveAttribute('target', '_blank');
  await expect(link).toHaveAttribute('rel', 'noopener noreferrer');
  await expect(link).toHaveAttribute('href', 'https://example.com');
});

test('persists bookmarks after page refresh', async ({ page }) => {
  await page.fill('input[name="title"]', 'Persisted Bookmark');
  await page.fill('input[name="url"]', 'https://example.com');
  await page.click('button[type="submit"]');

  await page.reload();

  await expect(page.locator('.bookmark-title')).toHaveText('Persisted Bookmark');
  await expect(page.locator('.bookmark-title')).toHaveAttribute('href', 'https://example.com');
});

test('localStorage contains bookmark data serialized as JSON', async ({ page }) => {
  await page.fill('input[name="title"]', 'Example');
  await page.fill('input[name="url"]', 'https://example.com');
  await page.fill('textarea[name="note"]', 'A useful site');
  await page.fill('input[name="tags"]', 'work, reading');
  await page.click('button[type="submit"]');

  const raw = await page.evaluate(() => localStorage.getItem('bookmarks'));
  expect(raw).not.toBeNull();
  const stored = JSON.parse(raw!);
  expect(stored).toHaveLength(1);
  expect(stored[0].title).toBe('Example');
  expect(stored[0].url).toBe('https://example.com');
  expect(stored[0].note).toBe('A useful site');
  expect(stored[0].tags).toEqual(['work', 'reading']);
  expect(stored[0].id).toBeTruthy();
  expect(stored[0].createdAt).toBeGreaterThan(0);
});

test('shows error and does not add bookmark when title is empty', async ({ page }) => {
  await page.fill('input[name="url"]', 'https://example.com');
  await page.click('button[type="submit"]');

  await expect(page.locator('#form-error')).toHaveText('Title cannot be empty');
  await expect(page.locator('.bookmark-item')).toHaveCount(0);
});

test('form data is preserved when title validation fails', async ({ page }) => {
  await page.fill('input[name="url"]', 'https://example.com');
  await page.fill('textarea[name="note"]', 'Some note');
  await page.click('button[type="submit"]');

  await expect(page.locator('input[name="url"]')).toHaveValue('https://example.com');
  await expect(page.locator('textarea[name="note"]')).toHaveValue('Some note');
});

test('shows error and does not add bookmark when URL is invalid', async ({ page }) => {
  await page.fill('input[name="title"]', 'Bad URL');
  await page.fill('input[name="url"]', 'not-a-url');
  await page.click('button[type="submit"]');

  await expect(page.locator('#form-error')).toHaveText('URL must start with http:// or https://');
  await expect(page.locator('.bookmark-item')).toHaveCount(0);
});

test('form data is preserved when URL validation fails', async ({ page }) => {
  await page.fill('input[name="title"]', 'Test');
  await page.fill('input[name="url"]', 'not-a-url');
  await page.click('button[type="submit"]');

  await expect(page.locator('input[name="title"]')).toHaveValue('Test');
  await expect(page.locator('input[name="url"]')).toHaveValue('not-a-url');
});

test('accepts URLs with an uppercase protocol', async ({ page }) => {
  await page.fill('input[name="title"]', 'Example');
  await page.fill('input[name="url"]', 'HTTPS://example.com');
  await page.click('button[type="submit"]');

  await expect(page.locator('#form-error')).toHaveText('');
  await expect(page.locator('.bookmark-item')).toHaveCount(1);
});

test('rejects a URL that is only a protocol with no domain', async ({ page }) => {
  await page.fill('input[name="title"]', 'Test');
  await page.fill('input[name="url"]', 'https://');
  await page.click('button[type="submit"]');

  await expect(page.locator('#form-error')).not.toHaveText('');
  await expect(page.locator('.bookmark-item')).toHaveCount(0);
});

// ---------------------------------------------------------------------------
// Layer 4: Tag Filtering
// ---------------------------------------------------------------------------

test('"All" filter button is present even when no bookmarks exist', async ({ page }) => {
  await expect(page.locator('.filter-btn')).toHaveCount(1);
  await expect(page.locator('.filter-btn')).toHaveText('All');
});

test('"All" is highlighted as the active filter on load', async ({ page }) => {
  await expect(page.locator('.filter-btn--active')).toHaveCount(1);
  await expect(page.locator('.filter-btn--active')).toHaveText('All');
});

test('adding a bookmark with tags shows a filter button for each unique tag', async ({ page }) => {
  await page.fill('input[name="title"]', 'Example');
  await page.fill('input[name="url"]', 'https://example.com');
  await page.fill('input[name="tags"]', 'work, reading');
  await page.click('button[type="submit"]');

  await expect(page.locator('.filter-btn')).toHaveCount(3); // All + work + reading
  await expect(page.locator('.filter-btn').nth(1)).toHaveText('reading');
  await expect(page.locator('.filter-btn').nth(2)).toHaveText('work');
});

test('the same tag on multiple bookmarks produces only one filter button', async ({ page }) => {
  await page.fill('input[name="title"]', 'First');
  await page.fill('input[name="url"]', 'https://first.com');
  await page.fill('input[name="tags"]', 'work');
  await page.click('button[type="submit"]');
  await page.fill('input[name="title"]', 'Second');
  await page.fill('input[name="url"]', 'https://second.com');
  await page.fill('input[name="tags"]', 'work');
  await page.click('button[type="submit"]');

  await expect(page.locator('.filter-btn')).toHaveCount(2); // All + work (no duplicate)
});

test('clicking a tag filter shows only matching bookmarks', async ({ page }) => {
  await page.fill('input[name="title"]', 'Work Bookmark');
  await page.fill('input[name="url"]', 'https://work.com');
  await page.fill('input[name="tags"]', 'work');
  await page.click('button[type="submit"]');
  await page.fill('input[name="title"]', 'Reading Bookmark');
  await page.fill('input[name="url"]', 'https://reading.com');
  await page.fill('input[name="tags"]', 'reading');
  await page.click('button[type="submit"]');

  await page.locator('.filter-btn', { hasText: 'work' }).click();

  await expect(page.locator('.bookmark-item')).toHaveCount(1);
  await expect(page.locator('.bookmark-title')).toHaveText('Work Bookmark');
});

test('bookmarks without the active tag are not shown', async ({ page }) => {
  await page.fill('input[name="title"]', 'Work Bookmark');
  await page.fill('input[name="url"]', 'https://work.com');
  await page.fill('input[name="tags"]', 'work');
  await page.click('button[type="submit"]');
  await page.fill('input[name="title"]', 'No Tags');
  await page.fill('input[name="url"]', 'https://notags.com');
  await page.click('button[type="submit"]');

  await page.locator('.filter-btn', { hasText: 'work' }).click();

  await expect(page.locator('.bookmark-item')).toHaveCount(1);
  await expect(page.locator('.bookmark-title')).toHaveText('Work Bookmark');
});

test('when a tag filter is active and no bookmarks match the list is empty', async ({ page }) => {
  await page.fill('input[name="title"]', 'Work Bookmark');
  await page.fill('input[name="url"]', 'https://work.com');
  await page.fill('input[name="tags"]', 'work');
  await page.click('button[type="submit"]');

  await page.locator('.filter-btn', { hasText: 'work' }).click();
  await expect(page.locator('.bookmark-item')).toHaveCount(1);

  // Delete the only matching bookmark while the filter is active
  page.on('dialog', dialog => dialog.accept());
  await page.locator('.delete-btn').click();

  await expect(page.locator('.bookmark-item')).toHaveCount(0);
  // Active tag should be reset — "All" button is highlighted and is the only filter button
  await expect(page.locator('.filter-btn')).toHaveCount(1);
  await expect(page.locator('.filter-btn--active')).toHaveText('All');
});

test('clicking "All" after a tag filter shows all bookmarks', async ({ page }) => {
  await page.fill('input[name="title"]', 'Work Bookmark');
  await page.fill('input[name="url"]', 'https://work.com');
  await page.fill('input[name="tags"]', 'work');
  await page.click('button[type="submit"]');
  await page.fill('input[name="title"]', 'No Tags');
  await page.fill('input[name="url"]', 'https://notags.com');
  await page.click('button[type="submit"]');

  await page.locator('.filter-btn', { hasText: 'work' }).click();
  await page.locator('.filter-btn', { hasText: 'All' }).click();

  await expect(page.locator('.bookmark-item')).toHaveCount(2);
});

test('the active tag filter button is highlighted and "All" loses its highlight', async ({ page }) => {
  await page.fill('input[name="title"]', 'Example');
  await page.fill('input[name="url"]', 'https://example.com');
  await page.fill('input[name="tags"]', 'work');
  await page.click('button[type="submit"]');

  await page.locator('.filter-btn', { hasText: 'work' }).click();

  await expect(page.locator('.filter-btn--active')).toHaveCount(1);
  await expect(page.locator('.filter-btn--active')).toHaveText('work');
  await expect(page.locator('.filter-btn', { hasText: 'All' })).not.toHaveClass(/filter-btn--active/);
});

test('switching tag filters updates the active highlight and the list', async ({ page }) => {
  await page.fill('input[name="title"]', 'Work');
  await page.fill('input[name="url"]', 'https://work.com');
  await page.fill('input[name="tags"]', 'work');
  await page.click('button[type="submit"]');
  await page.fill('input[name="title"]', 'Reading');
  await page.fill('input[name="url"]', 'https://reading.com');
  await page.fill('input[name="tags"]', 'reading');
  await page.click('button[type="submit"]');

  await page.locator('.filter-btn', { hasText: 'work' }).click();
  await page.locator('.filter-btn', { hasText: 'reading' }).click();

  await expect(page.locator('.filter-btn--active')).toHaveText('reading');
  await expect(page.locator('.bookmark-item')).toHaveCount(1);
  await expect(page.locator('.bookmark-title')).toHaveText('Reading');
});

test('deleting all bookmarks with a tag removes that tag filter button', async ({ page }) => {
  await page.fill('input[name="title"]', 'Work Only');
  await page.fill('input[name="url"]', 'https://work.com');
  await page.fill('input[name="tags"]', 'work');
  await page.click('button[type="submit"]');

  await expect(page.locator('.filter-btn')).toHaveCount(2);

  page.on('dialog', dialog => dialog.accept());
  await page.locator('.delete-btn').click();

  await expect(page.locator('.filter-btn')).toHaveCount(1);
  await expect(page.locator('.filter-btn')).toHaveText('All');
});

test('adding a bookmark while a tag filter is active shows it only if it matches', async ({ page }) => {
  await page.fill('input[name="title"]', 'Work');
  await page.fill('input[name="url"]', 'https://work.com');
  await page.fill('input[name="tags"]', 'work');
  await page.click('button[type="submit"]');

  await page.locator('.filter-btn', { hasText: 'work' }).click();

  // Add a bookmark that matches the active filter
  await page.fill('input[name="title"]', 'Also Work');
  await page.fill('input[name="url"]', 'https://alsowork.com');
  await page.fill('input[name="tags"]', 'work');
  await page.click('button[type="submit"]');
  await expect(page.locator('.bookmark-item')).toHaveCount(2);

  // Add a bookmark that does not match
  await page.fill('input[name="title"]', 'Reading');
  await page.fill('input[name="url"]', 'https://reading.com');
  await page.fill('input[name="tags"]', 'reading');
  await page.click('button[type="submit"]');
  await expect(page.locator('.bookmark-item')).toHaveCount(2);
});

// ---------------------------------------------------------------------------
// Layer 3: Edit and Delete
// ---------------------------------------------------------------------------

test('each bookmark has a visible edit button', async ({ page }) => {
  await page.fill('input[name="title"]', 'First');
  await page.fill('input[name="url"]', 'https://first.com');
  await page.click('button[type="submit"]');
  await page.fill('input[name="title"]', 'Second');
  await page.fill('input[name="url"]', 'https://second.com');
  await page.click('button[type="submit"]');

  await expect(page.locator('.edit-btn')).toHaveCount(2);
});

test('each bookmark has a visible delete button', async ({ page }) => {
  await page.fill('input[name="title"]', 'First');
  await page.fill('input[name="url"]', 'https://first.com');
  await page.click('button[type="submit"]');
  await page.fill('input[name="title"]', 'Second');
  await page.fill('input[name="url"]', 'https://second.com');
  await page.click('button[type="submit"]');

  await expect(page.locator('.delete-btn')).toHaveCount(2);
});

test('clicking edit shows an inline form pre-populated with current values', async ({ page }) => {
  await page.fill('input[name="title"]', 'My Bookmark');
  await page.fill('input[name="url"]', 'https://example.com');
  await page.fill('textarea[name="note"]', 'A note');
  await page.fill('input[name="tags"]', 'work, reading');
  await page.click('button[type="submit"]');

  await page.click('.edit-btn');

  await expect(page.locator('.edit-form input[name="title"]')).toHaveValue('My Bookmark');
  await expect(page.locator('.edit-form input[name="url"]')).toHaveValue('https://example.com');
  await expect(page.locator('.edit-form textarea[name="note"]')).toHaveValue('A note');
  await expect(page.locator('.edit-form input[name="tags"]')).toHaveValue('work, reading');
});

test('saving an edit updates all displayed values immediately', async ({ page }) => {
  await page.fill('input[name="title"]', 'Original');
  await page.fill('input[name="url"]', 'https://original.com');
  await page.fill('textarea[name="note"]', 'Old note');
  await page.fill('input[name="tags"]', 'old');
  await page.click('button[type="submit"]');

  await page.click('.edit-btn');
  await page.fill('.edit-form input[name="title"]', 'Updated');
  await page.fill('.edit-form input[name="url"]', 'https://updated.com');
  await page.fill('.edit-form textarea[name="note"]', 'New note');
  await page.fill('.edit-form input[name="tags"]', 'new, tags');
  await page.click('.edit-form button[type="submit"]');

  await expect(page.locator('.bookmark-title')).toHaveText('Updated');
  await expect(page.locator('.bookmark-title')).toHaveAttribute('href', 'https://updated.com');
  await expect(page.locator('.bookmark-note')).toHaveText('New note');
  const badges = page.locator('.tag-badge');
  await expect(badges).toHaveCount(2);
  await expect(badges.nth(0)).toHaveText('new');
  await expect(badges.nth(1)).toHaveText('tags');
});

test('editing a bookmark to remove its note hides the note element', async ({ page }) => {
  await page.fill('input[name="title"]', 'With Note');
  await page.fill('input[name="url"]', 'https://example.com');
  await page.fill('textarea[name="note"]', 'A note');
  await page.click('button[type="submit"]');

  await page.click('.edit-btn');
  await page.fill('.edit-form textarea[name="note"]', '');
  await page.click('.edit-form button[type="submit"]');

  await expect(page.locator('.bookmark-note')).toHaveCount(0);
});

test('editing a bookmark to remove its tags hides the tag badges', async ({ page }) => {
  await page.fill('input[name="title"]', 'With Tags');
  await page.fill('input[name="url"]', 'https://example.com');
  await page.fill('input[name="tags"]', 'work, reading');
  await page.click('button[type="submit"]');

  await page.click('.edit-btn');
  await page.fill('.edit-form input[name="tags"]', '');
  await page.click('.edit-form button[type="submit"]');

  await expect(page.locator('.tag-badge')).toHaveCount(0);
});

test('saving an edit does not change the bookmark count', async ({ page }) => {
  await page.fill('input[name="title"]', 'First');
  await page.fill('input[name="url"]', 'https://first.com');
  await page.click('button[type="submit"]');
  await page.fill('input[name="title"]', 'Second');
  await page.fill('input[name="url"]', 'https://second.com');
  await page.click('button[type="submit"]');

  await page.locator('.edit-btn').first().click();
  await page.fill('.edit-form input[name="title"]', 'Updated');
  await page.click('.edit-form button[type="submit"]');

  await expect(page.locator('.bookmark-item')).toHaveCount(2);
});

test('canceling an edit leaves the bookmark unchanged', async ({ page }) => {
  await page.fill('input[name="title"]', 'Original');
  await page.fill('input[name="url"]', 'https://original.com');
  await page.click('button[type="submit"]');

  await page.click('.edit-btn');
  await page.fill('.edit-form input[name="title"]', 'Changed');
  await page.click('.cancel-edit');

  await expect(page.locator('.bookmark-title')).toHaveText('Original');
  await expect(page.locator('.bookmark-title')).toHaveAttribute('href', 'https://original.com');
});

test('saving an edit with an empty title shows an error and does not save', async ({ page }) => {
  await page.fill('input[name="title"]', 'Original');
  await page.fill('input[name="url"]', 'https://original.com');
  await page.click('button[type="submit"]');

  await page.click('.edit-btn');
  await page.fill('.edit-form input[name="title"]', '');
  await page.click('.edit-form button[type="submit"]');

  await expect(page.locator('.edit-error')).toHaveText('Title cannot be empty');
  await expect(page.locator('.edit-form')).toBeVisible();
});

test('saving an edit with an invalid URL shows an error and does not save', async ({ page }) => {
  await page.fill('input[name="title"]', 'Original');
  await page.fill('input[name="url"]', 'https://original.com');
  await page.click('button[type="submit"]');

  await page.click('.edit-btn');
  await page.fill('.edit-form input[name="url"]', 'not-a-url');
  await page.click('.edit-form button[type="submit"]');

  await expect(page.locator('.edit-error')).toHaveText('URL must start with http:// or https://');
  await expect(page.locator('.edit-form')).toBeVisible();
});

test('localStorage reflects edited values after save', async ({ page }) => {
  await page.fill('input[name="title"]', 'Original');
  await page.fill('input[name="url"]', 'https://original.com');
  await page.click('button[type="submit"]');

  await page.click('.edit-btn');
  await page.fill('.edit-form input[name="title"]', 'Updated');
  await page.fill('.edit-form input[name="url"]', 'https://updated.com');
  await page.fill('.edit-form textarea[name="note"]', 'New note');
  await page.fill('.edit-form input[name="tags"]', 'new, tags');
  await page.click('.edit-form button[type="submit"]');

  const raw = await page.evaluate(() => localStorage.getItem('bookmarks'));
  const stored = JSON.parse(raw!);
  expect(stored).toHaveLength(1);
  expect(stored[0].title).toBe('Updated');
  expect(stored[0].url).toBe('https://updated.com');
  expect(stored[0].note).toBe('New note');
  expect(stored[0].tags).toEqual(['new', 'tags']);
});

test('edited values persist after page refresh', async ({ page }) => {
  await page.fill('input[name="title"]', 'Original');
  await page.fill('input[name="url"]', 'https://original.com');
  await page.click('button[type="submit"]');

  await page.click('.edit-btn');
  await page.fill('.edit-form input[name="title"]', 'Updated');
  await page.fill('.edit-form input[name="url"]', 'https://updated.com');
  await page.click('.edit-form button[type="submit"]');

  await page.reload();

  await expect(page.locator('.bookmark-title')).toHaveText('Updated');
  await expect(page.locator('.bookmark-title')).toHaveAttribute('href', 'https://updated.com');
});

test('confirming delete removes exactly the targeted bookmark', async ({ page }) => {
  await page.fill('input[name="title"]', 'Keep');
  await page.fill('input[name="url"]', 'https://keep.com');
  await page.click('button[type="submit"]');
  await page.fill('input[name="title"]', 'Delete Me');
  await page.fill('input[name="url"]', 'https://deleteme.com');
  await page.click('button[type="submit"]');

  page.on('dialog', dialog => dialog.accept());
  await page.locator('.delete-btn').nth(0).click();

  await expect(page.locator('.bookmark-item')).toHaveCount(1);
  await expect(page.locator('.bookmark-title')).toHaveText('Keep');
});

test('dismissing the delete confirmation leaves the list unchanged', async ({ page }) => {
  await page.fill('input[name="title"]', 'My Bookmark');
  await page.fill('input[name="url"]', 'https://example.com');
  await page.click('button[type="submit"]');

  page.on('dialog', dialog => dialog.dismiss());
  await page.click('.delete-btn');

  await expect(page.locator('.bookmark-item')).toHaveCount(1);
});

test('localStorage no longer contains the deleted bookmark after deletion', async ({ page }) => {
  await page.fill('input[name="title"]', 'Keep');
  await page.fill('input[name="url"]', 'https://keep.com');
  await page.click('button[type="submit"]');
  await page.fill('input[name="title"]', 'Delete Me');
  await page.fill('input[name="url"]', 'https://deleteme.com');
  await page.click('button[type="submit"]');

  const rawBefore = await page.evaluate(() => localStorage.getItem('bookmarks'));
  const idToDelete = JSON.parse(rawBefore!).find((b: { title: string; id: string }) => b.title === 'Delete Me').id;

  page.on('dialog', dialog => dialog.accept());
  await page.locator('.delete-btn').nth(0).click();

  const rawAfter = await page.evaluate(() => localStorage.getItem('bookmarks'));
  const stored = JSON.parse(rawAfter!);
  expect(stored.find((b: { id: string }) => b.id === idToDelete)).toBeUndefined();
  expect(stored).toHaveLength(1);
  expect(stored[0].title).toBe('Keep');
});

test('deleted bookmark is gone after page refresh', async ({ page }) => {
  await page.fill('input[name="title"]', 'My Bookmark');
  await page.fill('input[name="url"]', 'https://example.com');
  await page.click('button[type="submit"]');

  page.on('dialog', dialog => dialog.accept());
  await page.click('.delete-btn');

  await page.reload();

  await expect(page.locator('.bookmark-item')).toHaveCount(0);
});

// ---------------------------------------------------------------------------
// Layer 2: Notes and Tags
// ---------------------------------------------------------------------------

test('note textarea is present in the add form', async ({ page }) => {
  await expect(page.locator('textarea[name="note"]')).toBeVisible();
});

test('tags input is present in the add form', async ({ page }) => {
  await expect(page.locator('input[name="tags"]')).toBeVisible();
});

test('submitting without a note does not produce an error', async ({ page }) => {
  await page.fill('input[name="title"]', 'No Note');
  await page.fill('input[name="url"]', 'https://example.com');
  await page.click('button[type="submit"]');

  await expect(page.locator('#form-error')).toHaveText('');
  await expect(page.locator('.bookmark-item')).toHaveCount(1);
});

test('submitting without tags does not produce an error', async ({ page }) => {
  await page.fill('input[name="title"]', 'No Tags');
  await page.fill('input[name="url"]', 'https://example.com');
  await page.click('button[type="submit"]');

  await expect(page.locator('#form-error')).toHaveText('');
  await expect(page.locator('.bookmark-item')).toHaveCount(1);
});

test('displays note below bookmark title when note is provided', async ({ page }) => {
  await page.fill('input[name="title"]', 'Example');
  await page.fill('input[name="url"]', 'https://example.com');
  await page.fill('textarea[name="note"]', 'This is a useful site');
  await page.click('button[type="submit"]');

  await expect(page.locator('.bookmark-note')).toHaveText('This is a useful site');
});

test('does not display a note element when no note is provided', async ({ page }) => {
  await page.fill('input[name="title"]', 'Example');
  await page.fill('input[name="url"]', 'https://example.com');
  await page.click('button[type="submit"]');

  await expect(page.locator('.bookmark-note')).toHaveCount(0);
});

test('displays tags as individual badges', async ({ page }) => {
  await page.fill('input[name="title"]', 'Example');
  await page.fill('input[name="url"]', 'https://example.com');
  await page.fill('input[name="tags"]', 'work, reading, tools');
  await page.click('button[type="submit"]');

  const badges = page.locator('.tag-badge');
  await expect(badges).toHaveCount(3);
  await expect(badges.nth(0)).toHaveText('work');
  await expect(badges.nth(1)).toHaveText('reading');
  await expect(badges.nth(2)).toHaveText('tools');
});

test('does not display tag badges when no tags are provided', async ({ page }) => {
  await page.fill('input[name="title"]', 'Example');
  await page.fill('input[name="url"]', 'https://example.com');
  await page.click('button[type="submit"]');

  await expect(page.locator('.tag-badge')).toHaveCount(0);
});
