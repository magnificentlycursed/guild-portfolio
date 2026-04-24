import { describe, it, expect } from 'vitest';
import {
  type Bookmark,
  type BookmarkStorage,
  STORAGE_KEY,
  validateTitle,
  validateUrl,
  parseTags,
  loadBookmarks,
  saveBookmarks,
  generateId,
  sortBookmarks,
  updateBookmark,
  deleteBookmark,
  getUniqueTags,
  filterByTag,
} from '../../src/bookmarks';

// ---------------------------------------------------------------------------
// Mock storage — no DOM or browser APIs required
// ---------------------------------------------------------------------------

function createMockStorage(): { storage: BookmarkStorage; store: Map<string, string> } {
  const store = new Map<string, string>();
  const storage: BookmarkStorage = {
    getItem: (key: string) => store.get(key) ?? null,
    setItem: (key: string, value: string) => { store.set(key, value); },
  };
  return { storage, store };
}

// ---------------------------------------------------------------------------
// validateTitle
// ---------------------------------------------------------------------------

describe('validateTitle', () => {
  it('returns an error message for an empty string', () => {
    expect(validateTitle('')).toBe('Title cannot be empty');
  });

  it('returns an error message for a whitespace-only string', () => {
    expect(validateTitle('   ')).toBe('Title cannot be empty');
  });

  it('returns null for a valid title', () => {
    expect(validateTitle('My Bookmark')).toBeNull();
  });

  it('returns null for a title with surrounding whitespace', () => {
    expect(validateTitle('  My Bookmark  ')).toBeNull();
  });
});

// ---------------------------------------------------------------------------
// validateUrl
// ---------------------------------------------------------------------------

describe('validateUrl', () => {
  it('returns an error message for a URL without a protocol', () => {
    expect(validateUrl('example.com')).toBe('URL must start with http:// or https://');
  });

  it('returns an error message for an ftp:// URL', () => {
    expect(validateUrl('ftp://example.com')).toBe('URL must start with http:// or https://');
  });

  it('returns an error message for an empty string', () => {
    expect(validateUrl('')).toBe('URL must start with http:// or https://');
  });

  it('returns an error message for a protocol-only URL with no domain', () => {
    expect(validateUrl('https://')).toBe('URL must start with http:// or https://');
  });

  it('returns null for an http:// URL', () => {
    expect(validateUrl('http://example.com')).toBeNull();
  });

  it('returns null for an https:// URL', () => {
    expect(validateUrl('https://example.com')).toBeNull();
  });

  it('returns null for an uppercase HTTP:// URL', () => {
    expect(validateUrl('HTTP://example.com')).toBeNull();
  });

  it('returns null for an uppercase HTTPS:// URL', () => {
    expect(validateUrl('HTTPS://example.com')).toBeNull();
  });
});

// ---------------------------------------------------------------------------
// parseTags
// ---------------------------------------------------------------------------

describe('parseTags', () => {
  it('splits comma-separated tags', () => {
    expect(parseTags('work,reading,tools')).toEqual(['work', 'reading', 'tools']);
  });

  it('trims whitespace from each tag', () => {
    expect(parseTags('work, reading, tools')).toEqual(['work', 'reading', 'tools']);
  });

  it('trims leading and trailing whitespace', () => {
    expect(parseTags('  work  ,  reading  ')).toEqual(['work', 'reading']);
  });

  it('filters out empty entries from consecutive commas', () => {
    expect(parseTags('work,,tools')).toEqual(['work', 'tools']);
  });

  it('returns an empty array for an empty string', () => {
    expect(parseTags('')).toEqual([]);
  });

  it('returns an empty array for a whitespace-only string', () => {
    expect(parseTags('   ')).toEqual([]);
  });

  it('returns a single-element array for input with no commas', () => {
    expect(parseTags('work')).toEqual(['work']);
  });
});

// ---------------------------------------------------------------------------
// loadBookmarks / saveBookmarks
// ---------------------------------------------------------------------------

describe('loadBookmarks', () => {
  it('returns an empty array when storage is empty', () => {
    const { storage } = createMockStorage();
    expect(loadBookmarks(storage)).toEqual([]);
  });

  it('returns the bookmarks stored in storage', () => {
    const { storage } = createMockStorage();
    const bookmarks: Bookmark[] = [{
      id: '1', url: 'https://example.com', title: 'Example',
      note: '', tags: [], createdAt: 1000,
    }];
    storage.setItem(STORAGE_KEY, JSON.stringify(bookmarks));
    expect(loadBookmarks(storage)).toEqual(bookmarks);
  });

  it('returns an empty array when storage contains invalid JSON', () => {
    const { storage } = createMockStorage();
    storage.setItem(STORAGE_KEY, 'not valid json {{');
    expect(loadBookmarks(storage)).toEqual([]);
  });
});

describe('saveBookmarks', () => {
  it('serializes bookmarks to storage under the correct key', () => {
    const { storage, store } = createMockStorage();
    const bookmarks: Bookmark[] = [{
      id: '1', url: 'https://example.com', title: 'Example',
      note: '', tags: [], createdAt: 1000,
    }];
    saveBookmarks(storage, bookmarks);
    expect(store.get(STORAGE_KEY)).toBe(JSON.stringify(bookmarks));
  });

  it('overwrites previously saved bookmarks', () => {
    const { storage, store } = createMockStorage();
    const first: Bookmark[] = [{ id: '1', url: 'https://a.com', title: 'A', note: '', tags: [], createdAt: 1 }];
    const second: Bookmark[] = [{ id: '2', url: 'https://b.com', title: 'B', note: '', tags: [], createdAt: 2 }];
    saveBookmarks(storage, first);
    saveBookmarks(storage, second);
    expect(store.get(STORAGE_KEY)).toBe(JSON.stringify(second));
  });
});

// ---------------------------------------------------------------------------
// generateId
// ---------------------------------------------------------------------------

describe('generateId', () => {
  it('returns a non-empty string', () => {
    expect(generateId().length).toBeGreaterThan(0);
  });

  it('contains a hyphen separator', () => {
    expect(generateId()).toContain('-');
  });

  it('returns a unique value on each call', () => {
    const ids = new Set(Array.from({ length: 100 }, generateId));
    expect(ids.size).toBe(100);
  });
});

// ---------------------------------------------------------------------------
// sortBookmarks
// ---------------------------------------------------------------------------

describe('sortBookmarks', () => {
  it('sorts bookmarks newest first by createdAt', () => {
    const bookmarks: Bookmark[] = [
      { id: 'a', url: 'https://a.com', title: 'A', note: '', tags: [], createdAt: 100 },
      { id: 'b', url: 'https://b.com', title: 'B', note: '', tags: [], createdAt: 200 },
      { id: 'c', url: 'https://c.com', title: 'C', note: '', tags: [], createdAt: 150 },
    ];
    const sorted = sortBookmarks(bookmarks);
    expect(sorted.map(b => b.id)).toEqual(['b', 'c', 'a']);
  });

  it('does not mutate the original array', () => {
    const bookmarks: Bookmark[] = [
      { id: 'a', url: 'https://a.com', title: 'A', note: '', tags: [], createdAt: 100 },
      { id: 'b', url: 'https://b.com', title: 'B', note: '', tags: [], createdAt: 200 },
    ];
    sortBookmarks(bookmarks);
    expect(bookmarks[0].id).toBe('a');
  });

  it('produces a stable, deterministic order for bookmarks with identical timestamps', () => {
    const bookmarks: Bookmark[] = [
      { id: 'z', url: 'https://z.com', title: 'Z', note: '', tags: [], createdAt: 100 },
      { id: 'a', url: 'https://a.com', title: 'A', note: '', tags: [], createdAt: 100 },
      { id: 'm', url: 'https://m.com', title: 'M', note: '', tags: [], createdAt: 100 },
    ];
    const sorted1 = sortBookmarks(bookmarks);
    const sorted2 = sortBookmarks([...bookmarks].reverse());
    expect(sorted1.map(b => b.id)).toEqual(sorted2.map(b => b.id));
  });

  it('returns an empty array for empty input', () => {
    expect(sortBookmarks([])).toEqual([]);
  });
});

// ---------------------------------------------------------------------------
// updateBookmark
// ---------------------------------------------------------------------------

describe('updateBookmark', () => {
  const bookmarks: Bookmark[] = [
    { id: 'a', url: 'https://a.com', title: 'A', note: '', tags: [], createdAt: 100 },
    { id: 'b', url: 'https://b.com', title: 'B', note: 'note', tags: ['x'], createdAt: 200 },
  ];

  it('updates the matching bookmark with the provided fields', () => {
    const result = updateBookmark(bookmarks, 'a', { title: 'Updated', url: 'https://updated.com' });
    expect(result[0].title).toBe('Updated');
    expect(result[0].url).toBe('https://updated.com');
  });

  it('does not mutate the original array', () => {
    updateBookmark(bookmarks, 'a', { title: 'Updated' });
    expect(bookmarks[0].title).toBe('A');
  });

  it('does not change the array length', () => {
    const result = updateBookmark(bookmarks, 'a', { title: 'Updated' });
    expect(result).toHaveLength(2);
  });

  it('does not modify other bookmarks', () => {
    const result = updateBookmark(bookmarks, 'a', { title: 'Updated' });
    expect(result[1].title).toBe('B');
    expect(result[1].note).toBe('note');
    expect(result[1].tags).toEqual(['x']);
  });

  it('preserves fields not included in the update', () => {
    const result = updateBookmark(bookmarks, 'a', { title: 'Updated' });
    expect(result[0].id).toBe('a');
    expect(result[0].createdAt).toBe(100);
  });

  it('returns the array unchanged when the id is not found', () => {
    const result = updateBookmark(bookmarks, 'z', { title: 'Updated' });
    expect(result[0].title).toBe('A');
    expect(result[1].title).toBe('B');
  });
});

// ---------------------------------------------------------------------------
// deleteBookmark
// ---------------------------------------------------------------------------

describe('deleteBookmark', () => {
  const bookmarks: Bookmark[] = [
    { id: 'a', url: 'https://a.com', title: 'A', note: '', tags: [], createdAt: 100 },
    { id: 'b', url: 'https://b.com', title: 'B', note: '', tags: [], createdAt: 200 },
    { id: 'c', url: 'https://c.com', title: 'C', note: '', tags: [], createdAt: 300 },
  ];

  it('removes the bookmark with the matching id', () => {
    const result = deleteBookmark(bookmarks, 'b');
    expect(result.find(b => b.id === 'b')).toBeUndefined();
  });

  it('does not mutate the original array', () => {
    deleteBookmark(bookmarks, 'a');
    expect(bookmarks).toHaveLength(3);
  });

  it('reduces the array length by one', () => {
    const result = deleteBookmark(bookmarks, 'a');
    expect(result).toHaveLength(2);
  });

  it('does not remove other bookmarks', () => {
    const result = deleteBookmark(bookmarks, 'b');
    expect(result.find(b => b.id === 'a')).toBeDefined();
    expect(result.find(b => b.id === 'c')).toBeDefined();
  });

  it('returns the array unchanged when the id is not found', () => {
    const result = deleteBookmark(bookmarks, 'z');
    expect(result).toHaveLength(3);
  });
});

// ---------------------------------------------------------------------------
// getUniqueTags
// ---------------------------------------------------------------------------

describe('getUniqueTags', () => {
  it('returns an empty array for an empty bookmark list', () => {
    expect(getUniqueTags([])).toEqual([]);
  });

  it('returns an empty array when no bookmarks have tags', () => {
    const bookmarks: Bookmark[] = [
      { id: 'a', url: 'https://a.com', title: 'A', note: '', tags: [], createdAt: 1 },
    ];
    expect(getUniqueTags(bookmarks)).toEqual([]);
  });

  it('returns each unique tag once even if it appears on multiple bookmarks', () => {
    const bookmarks: Bookmark[] = [
      { id: 'a', url: 'https://a.com', title: 'A', note: '', tags: ['work', 'reading'], createdAt: 1 },
      { id: 'b', url: 'https://b.com', title: 'B', note: '', tags: ['work', 'tools'], createdAt: 2 },
    ];
    expect(getUniqueTags(bookmarks)).toEqual(['reading', 'tools', 'work']);
  });

  it('returns tags sorted alphabetically', () => {
    const bookmarks: Bookmark[] = [
      { id: 'a', url: 'https://a.com', title: 'A', note: '', tags: ['zebra', 'apple', 'mango'], createdAt: 1 },
    ];
    expect(getUniqueTags(bookmarks)).toEqual(['apple', 'mango', 'zebra']);
  });

  it('does not mutate the input array', () => {
    const bookmarks: Bookmark[] = [
      { id: 'a', url: 'https://a.com', title: 'A', note: '', tags: ['work'], createdAt: 1 },
    ];
    getUniqueTags(bookmarks);
    expect(bookmarks[0].tags).toEqual(['work']);
  });
});

// ---------------------------------------------------------------------------
// filterByTag
// ---------------------------------------------------------------------------

describe('filterByTag', () => {
  const bookmarks: Bookmark[] = [
    { id: 'a', url: 'https://a.com', title: 'A', note: '', tags: ['work', 'reading'], createdAt: 1 },
    { id: 'b', url: 'https://b.com', title: 'B', note: '', tags: ['work'], createdAt: 2 },
    { id: 'c', url: 'https://c.com', title: 'C', note: '', tags: ['reading'], createdAt: 3 },
    { id: 'd', url: 'https://d.com', title: 'D', note: '', tags: [], createdAt: 4 },
  ];

  it('returns only bookmarks that include the given tag', () => {
    const result = filterByTag(bookmarks, 'work');
    expect(result.map(b => b.id)).toEqual(['a', 'b']);
  });

  it('does not return bookmarks that do not have the tag', () => {
    const result = filterByTag(bookmarks, 'work');
    expect(result.find(b => b.id === 'c')).toBeUndefined();
    expect(result.find(b => b.id === 'd')).toBeUndefined();
  });

  it('returns an empty array when no bookmarks match', () => {
    expect(filterByTag(bookmarks, 'nonexistent')).toHaveLength(0);
  });

  it('returns the correct count of matching bookmarks', () => {
    expect(filterByTag(bookmarks, 'reading')).toHaveLength(2);
  });

  it('does not mutate the original array', () => {
    filterByTag(bookmarks, 'work');
    expect(bookmarks).toHaveLength(4);
  });
});
