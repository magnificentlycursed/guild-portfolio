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
  getUniqueTags,
  applyFilters,
  extractDomain,
} from './bookmarks';

const storage = localStorage;

let activeTag: string | null = null;
let searchQuery = '';
let formOpen = false;

function setFormOpen(open: boolean): void {
  const form = document.getElementById('add-form') as HTMLFormElement;
  const toggle = document.getElementById('add-form-toggle') as HTMLButtonElement;
  formOpen = open;
  form.hidden = !open;
  toggle.setAttribute('aria-expanded', String(open));
  if (open) {
    const firstInput = form.querySelector('input') as HTMLInputElement | null;
    firstInput?.focus();
  } else {
    toggle.focus();
  }
}

function renderTagFilters(bookmarks: Bookmark[]): void {
  const uniqueTags = getUniqueTags(bookmarks);
  if (activeTag !== null && !uniqueTags.includes(activeTag)) {
    activeTag = null;
  }

  const container = document.getElementById('tag-filters') as HTMLDivElement;
  container.innerHTML = '';

  const allBtn = document.createElement('button');
  allBtn.type = 'button';
  allBtn.className = 'filter-btn' + (activeTag === null ? ' filter-btn--active' : '');
  allBtn.textContent = 'All';
  allBtn.addEventListener('click', () => {
    activeTag = null;
    renderBookmarks();
  });
  container.appendChild(allBtn);

  for (const tag of uniqueTags) {
    const btn = document.createElement('button');
    btn.type = 'button';
    btn.className = 'filter-btn' + (activeTag === tag ? ' filter-btn--active' : '');
    btn.textContent = tag;
    btn.dataset.tag = tag;
    btn.addEventListener('click', () => {
      activeTag = activeTag === tag ? null : tag;
      renderBookmarks();
    });
    container.appendChild(btn);
  }
}

function renderBookmarks(): void {
  const bookmarks = loadBookmarks(storage);
  const list = document.getElementById('bookmark-list') as HTMLUListElement;
  list.innerHTML = '';

  renderTagFilters(bookmarks);

  const visible = applyFilters(bookmarks, activeTag, searchQuery);
  const sorted = sortBookmarks(visible);

  const statusEl = document.getElementById('search-status') as HTMLParagraphElement;
  const isFiltering = searchQuery.trim() !== '' || activeTag !== null;
  if (!isFiltering || bookmarks.length === 0) {
    statusEl.textContent = '';
  } else if (sorted.length === 0) {
    statusEl.textContent = 'No bookmarks match your search.';
  } else {
    statusEl.textContent = `${sorted.length} bookmark${sorted.length === 1 ? '' : 's'} shown.`;
  }

  if (sorted.length === 0) {
    const li = document.createElement('li');
    li.className = 'list-empty';
    li.textContent = bookmarks.length === 0
      ? 'No bookmarks yet. Add one above.'
      : 'No bookmarks match your search.';
    list.appendChild(li);
  }

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

    const domain = extractDomain(bookmark.url);
    if (domain) {
      const domainEl = document.createElement('p');
      domainEl.textContent = domain;
      domainEl.className = 'bookmark-domain';
      li.appendChild(domainEl);
    }

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
        const badge = document.createElement('button');
        badge.type = 'button';
        badge.textContent = tag;
        badge.className = 'tag-badge';
        badge.addEventListener('click', () => {
          activeTag = activeTag === tag ? null : tag;
          renderBookmarks();
        });
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

  const fields: { label: string; hint?: string; name: string; type: string; value: string }[] = [
    { label: 'Title', name: 'title', type: 'text', value: bookmark.title },
    { label: 'URL', name: 'url', type: 'text', value: bookmark.url },
    { label: 'Note', hint: '(optional)', name: 'note', type: 'textarea', value: bookmark.note },
    { label: 'Tags', hint: '(optional, comma-separated)', name: 'tags', type: 'text', value: bookmark.tags.join(', ') },
  ];

  for (const field of fields) {
    const fieldId = `edit-${id}-${field.name}`;
    const group = document.createElement('div');
    group.className = 'form-group';
    const label = document.createElement('label');
    label.htmlFor = fieldId;
    label.textContent = field.label;
    if (field.hint) {
      const hint = document.createElement('span');
      hint.className = 'optional';
      hint.textContent = ' ' + field.hint;
      label.appendChild(hint);
    }
    group.appendChild(label);
    if (field.type === 'textarea') {
      const textarea = document.createElement('textarea');
      textarea.id = fieldId;
      textarea.name = field.name;
      textarea.value = field.value;
      textarea.rows = 2;
      group.appendChild(textarea);
    } else {
      const input = document.createElement('input');
      input.id = fieldId;
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
  cancelBtn.addEventListener('click', () => {
    renderBookmarks();
    (document.querySelector(`[data-id="${id}"] .edit-btn`) as HTMLElement | null)?.focus();
  });
  form.appendChild(cancelBtn);

  form.addEventListener('submit', handleEditSave);
  form.addEventListener('input', () => { errorEl.textContent = ''; });

  li.innerHTML = '';
  li.appendChild(form);

  const firstField = form.querySelector('input, textarea') as HTMLElement | null;
  firstField?.focus();
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
  const bookmarks = loadBookmarks(storage);
  const bookmark = bookmarks.find(b => b.id === id);
  if (!bookmark) return;

  const li = document.querySelector(`[data-id="${id}"]`) as HTMLElement | null;
  if (!li) return;

  const actions = li.querySelector('.bookmark-actions') as HTMLElement | null;
  if (!actions) return;

  const confirm = document.createElement('div');
  confirm.className = 'delete-confirm';

  const msg = document.createElement('span');
  msg.className = 'delete-confirm-msg';
  msg.textContent = `Delete "${bookmark.title}"?`;
  confirm.appendChild(msg);

  const confirmBtn = document.createElement('button');
  confirmBtn.type = 'button';
  confirmBtn.className = 'delete-confirm-btn';
  confirmBtn.textContent = 'Delete';
  confirmBtn.addEventListener('click', () => {
    const updated = deleteBookmark(loadBookmarks(storage), id);
    saveBookmarks(storage, updated);
    renderBookmarks();
  });

  const cancelBtn = document.createElement('button');
  cancelBtn.type = 'button';
  cancelBtn.className = 'delete-cancel-btn';
  cancelBtn.textContent = 'Cancel';
  cancelBtn.addEventListener('click', () => {
    renderBookmarks();
    (document.querySelector(`[data-id="${id}"] .delete-btn`) as HTMLElement | null)?.focus();
  });

  confirm.appendChild(confirmBtn);
  confirm.appendChild(cancelBtn);
  actions.replaceWith(confirm);
  confirmBtn.focus();
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
  saveBookmarks(storage, [...bookmarks, newBookmark]);

  form.reset();
  setFormOpen(false);
  renderBookmarks();
}

document.addEventListener('DOMContentLoaded', () => {
  const toggle = document.getElementById('add-form-toggle') as HTMLButtonElement;
  toggle.addEventListener('click', () => setFormOpen(!formOpen));

  const form = document.getElementById('add-form') as HTMLFormElement;
  const errorEl = document.getElementById('form-error') as HTMLParagraphElement;
  form.addEventListener('submit', handleSubmit);
  form.addEventListener('input', () => { errorEl.textContent = ''; });

  const searchInput = document.getElementById('search-input') as HTMLInputElement;
  searchInput.addEventListener('input', () => {
    searchQuery = searchInput.value;
    renderBookmarks();
  });

  renderBookmarks();
});
