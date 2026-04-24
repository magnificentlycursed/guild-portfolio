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
