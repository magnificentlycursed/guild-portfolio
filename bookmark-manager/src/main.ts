import {
  type Bookmark,
  loadBookmarks,
  saveBookmarks,
  generateId,
  validateTitle,
  validateUrl,
  parseTags,
  sortBookmarks,
  updateBookmark,
  deleteBookmark,
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
    li.dataset.id = bookmark.id;

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

    const actions = document.createElement('div');
    actions.className = 'bookmark-actions';

    const editBtn = document.createElement('button');
    editBtn.textContent = 'Edit';
    editBtn.type = 'button';
    editBtn.className = 'edit-btn';
    editBtn.addEventListener('click', () => handleEditClick(bookmark.id));
    actions.appendChild(editBtn);

    const deleteBtn = document.createElement('button');
    deleteBtn.textContent = 'Delete';
    deleteBtn.type = 'button';
    deleteBtn.className = 'delete-btn';
    deleteBtn.addEventListener('click', () => handleDeleteClick(bookmark.id));
    actions.appendChild(deleteBtn);

    li.appendChild(actions);
    list.appendChild(li);
  }
}

function handleEditClick(id: string): void {
  const bookmarks = loadBookmarks(storage);
  const bookmark = bookmarks.find(b => b.id === id);
  if (!bookmark) return;

  const li = document.querySelector(`[data-id="${id}"]`) as HTMLElement | null;
  if (!li) return;

  const form = document.createElement('form');
  form.className = 'edit-form';
  form.dataset.id = id;

  const fields: { label: string; name: string; type: string; value: string }[] = [
    { label: 'Title', name: 'title', type: 'text', value: bookmark.title },
    { label: 'URL', name: 'url', type: 'text', value: bookmark.url },
    { label: 'Note', name: 'note', type: 'textarea', value: bookmark.note },
    { label: 'Tags', name: 'tags', type: 'text', value: bookmark.tags.join(', ') },
  ];

  for (const field of fields) {
    const group = document.createElement('div');
    group.className = 'form-group';
    const label = document.createElement('label');
    label.textContent = field.label;
    group.appendChild(label);
    if (field.type === 'textarea') {
      const textarea = document.createElement('textarea');
      textarea.name = field.name;
      textarea.value = field.value;
      textarea.rows = 2;
      group.appendChild(textarea);
    } else {
      const input = document.createElement('input');
      input.type = 'text';
      input.name = field.name;
      input.value = field.value;
      input.autocomplete = 'off';
      group.appendChild(input);
    }
    form.appendChild(group);
  }

  const errorEl = document.createElement('p');
  errorEl.className = 'edit-error error';
  errorEl.setAttribute('role', 'alert');
  form.appendChild(errorEl);

  const saveBtn = document.createElement('button');
  saveBtn.type = 'submit';
  saveBtn.textContent = 'Save';
  form.appendChild(saveBtn);

  const cancelBtn = document.createElement('button');
  cancelBtn.type = 'button';
  cancelBtn.className = 'cancel-edit';
  cancelBtn.textContent = 'Cancel';
  cancelBtn.addEventListener('click', renderBookmarks);
  form.appendChild(cancelBtn);

  form.addEventListener('submit', handleEditSave);

  li.innerHTML = '';
  li.appendChild(form);
}

function handleEditSave(event: Event): void {
  event.preventDefault();
  const form = event.target as HTMLFormElement;
  const id = form.dataset.id!;

  const title = (form.elements.namedItem('title') as HTMLInputElement).value.trim();
  const url = (form.elements.namedItem('url') as HTMLInputElement).value.trim();
  const note = (form.elements.namedItem('note') as HTMLTextAreaElement).value.trim();
  const tags = parseTags((form.elements.namedItem('tags') as HTMLInputElement).value);
  const errorEl = form.querySelector('.edit-error') as HTMLParagraphElement;

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

  const bookmarks = loadBookmarks(storage);
  const updated = updateBookmark(bookmarks, id, { title, url, note, tags });
  saveBookmarks(storage, updated);
  renderBookmarks();
}

function handleDeleteClick(id: string): void {
  if (!window.confirm('Delete this bookmark?')) return;
  const bookmarks = loadBookmarks(storage);
  const updated = deleteBookmark(bookmarks, id);
  saveBookmarks(storage, updated);
  renderBookmarks();
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
