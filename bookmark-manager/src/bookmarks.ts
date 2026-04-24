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
    return JSON.parse(data) as Bookmark[];
  } catch {
    return [];
  }
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
