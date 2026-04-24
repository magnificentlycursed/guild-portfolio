import {
  type Bookmark,
  loadBookmarks,
  saveBookmarks,
  generateId,
  validateTitle,
  validateUrl,
  parseTags,
  sortBookmarks,
} from './bookmarks';

const storage = localStorage;

function renderBookmarks(): void {
  const bookmarks = loadBookmarks(storage);
  const list = document.getElementById('bookmark-list') as HTMLUListElement;
  list.innerHTML = '';

  const sorted = sortBookmarks(bookmarks);

  for (const bookmark of sorted) {
    const li = document.createElement('li');
    li.className = 'bookmark-item';

    const link = document.createElement('a');
    link.href = bookmark.url;
    link.target = '_blank';
    link.rel = 'noopener noreferrer';
    link.textContent = bookmark.title;
    link.className = 'bookmark-title';
    li.appendChild(link);

    if (bookmark.note) {
      const note = document.createElement('p');
      note.textContent = bookmark.note;
      note.className = 'bookmark-note';
      li.appendChild(note);
    }

    if (bookmark.tags.length > 0) {
      const tagList = document.createElement('div');
      tagList.className = 'bookmark-tags';
      for (const tag of bookmark.tags) {
        const badge = document.createElement('span');
        badge.textContent = tag;
        badge.className = 'tag-badge';
        tagList.appendChild(badge);
      }
      li.appendChild(tagList);
    }

    list.appendChild(li);
  }
}

function handleSubmit(event: Event): void {
  event.preventDefault();

  const form = event.target as HTMLFormElement;
  const urlInput = form.elements.namedItem('url') as HTMLInputElement;
  const titleInput = form.elements.namedItem('title') as HTMLInputElement;
  const noteInput = form.elements.namedItem('note') as HTMLTextAreaElement;
  const tagsInput = form.elements.namedItem('tags') as HTMLInputElement;
  const errorEl = document.getElementById('form-error') as HTMLParagraphElement;

  const url = urlInput.value.trim();
  const title = titleInput.value.trim();
  const note = noteInput.value.trim();
  const tags = parseTags(tagsInput.value);

  const titleError = validateTitle(title);
  if (titleError) {
    errorEl.textContent = titleError;
    return;
  }

  const urlError = validateUrl(url);
  if (urlError) {
    errorEl.textContent = urlError;
    return;
  }

  errorEl.textContent = '';

  const bookmarks = loadBookmarks(storage);
  const newBookmark: Bookmark = {
    id: generateId(),
    url,
    title,
    note,
    tags,
    createdAt: Date.now(),
  };
  bookmarks.push(newBookmark);
  saveBookmarks(storage, bookmarks);

  form.reset();
  renderBookmarks();
}

document.addEventListener('DOMContentLoaded', () => {
  const form = document.getElementById('add-form') as HTMLFormElement;
  form.addEventListener('submit', handleSubmit);
  renderBookmarks();
});
