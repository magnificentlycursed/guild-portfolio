export interface Bookmark {
  id: string;
  url: string;
  title: string;
  note: string;
  tags: string[];
  createdAt: number;
}

export interface BookmarkStorage {
  getItem(key: string): string | null;
  setItem(key: string, value: string): void;
}

export const STORAGE_KEY = 'bookmarks';

export function loadBookmarks(storage: BookmarkStorage): Bookmark[] {
  const data = storage.getItem(STORAGE_KEY);
  if (!data) return [];
  try {
    const parsed: unknown = JSON.parse(data);
    if (!Array.isArray(parsed)) return [];
    return parsed.map(normalizeBookmark).filter((b): b is Bookmark => b !== null);
  } catch {
    return [];
  }
}

function normalizeBookmark(value: unknown): Bookmark | null {
  if (!value || typeof value !== 'object' || Array.isArray(value)) return null;
  const b = value as Record<string, unknown>;
  if (typeof b.id !== 'string' || typeof b.url !== 'string' || typeof b.title !== 'string') {
    return null;
  }
  return {
    id: b.id,
    url: b.url,
    title: b.title,
    note: typeof b.note === 'string' ? b.note : '',
    tags: Array.isArray(b.tags)
      ? (b.tags as unknown[]).filter((t): t is string => typeof t === 'string')
      : [],
    createdAt: typeof b.createdAt === 'number' ? b.createdAt : 0,
  };
}

export function saveBookmarks(storage: BookmarkStorage, bookmarks: Bookmark[]): void {
  storage.setItem(STORAGE_KEY, JSON.stringify(bookmarks));
}

export function generateId(): string {
  return `${Date.now()}-${Math.random().toString(36).slice(2)}`;
}

export function validateTitle(title: string): string | null {
  if (title.trim() === '') {
    return 'Title cannot be empty';
  }
  return null;
}

export function validateUrl(url: string): string | null {
  if (url.trim() === '') {
    return 'URL cannot be empty';
  }
  try {
    const parsed = new URL(url);
    if (parsed.protocol !== 'http:' && parsed.protocol !== 'https:') {
      return 'URL must start with http:// or https://';
    }
    return null;
  } catch {
    return 'URL must start with http:// or https://';
  }
}

export function parseTags(input: string): string[] {
  return input
    .split(',')
    .map(t => t.trim())
    .filter(t => t.length > 0);
}

export function sortBookmarks(bookmarks: Bookmark[]): Bookmark[] {
  return [...bookmarks].sort((a, b) =>
    b.createdAt - a.createdAt || a.id.localeCompare(b.id)
  );
}

export function updateBookmark(
  bookmarks: Bookmark[],
  id: string,
  updates: Partial<Pick<Bookmark, 'title' | 'url' | 'note' | 'tags'>>
): Bookmark[] {
  return bookmarks.map(b => b.id === id ? { ...b, ...updates } : b);
}

export function deleteBookmark(bookmarks: Bookmark[], id: string): Bookmark[] {
  return bookmarks.filter(b => b.id !== id);
}

export function getUniqueTags(bookmarks: Bookmark[]): string[] {
  const tags = new Set<string>();
  for (const bookmark of bookmarks) {
    for (const tag of bookmark.tags) {
      tags.add(tag);
    }
  }
  return [...tags].sort();
}

export function filterByTag(bookmarks: Bookmark[], tag: string): Bookmark[] {
  return bookmarks.filter(b => b.tags.includes(tag));
}

export function searchBookmarks(bookmarks: Bookmark[], query: string): Bookmark[] {
  if (query.trim() === '') return bookmarks;
  const q = query.toLowerCase();
  return bookmarks.filter(b =>
    b.title.toLowerCase().includes(q) || b.note.toLowerCase().includes(q)
  );
}

export function applyFilters(bookmarks: Bookmark[], tag: string | null, query: string): Bookmark[] {
  const tagged = tag !== null ? filterByTag(bookmarks, tag) : bookmarks;
  return searchBookmarks(tagged, query);
}

export function extractDomain(url: string): string {
  try {
    return new URL(url).hostname;
  } catch {
    return '';
  }
}
