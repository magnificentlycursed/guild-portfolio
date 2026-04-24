# Bookmark Manager

A personal tool for saving and organizing web links with titles, notes, and tags. No accounts, no backend, no frameworks — data lives in the browser's localStorage.

Built as a guild portfolio project following a design-first, test-driven methodology.

## Features

- Add bookmarks with a URL, title, optional note, and optional tags
- View all bookmarks in a list, newest first
- Click any bookmark to open it in a new tab
- Edit or delete individual bookmarks
- Filter bookmarks by tag
- Search by title or note content in real time
- Data persists across page refreshes

Layers 1–3 (core, notes and tags, edit and delete) are complete. Layers 4–6 (tag filtering, search, and polish) are in progress.

## Stack

- TypeScript, HTML, CSS — no frameworks
- [Vite](https://vitejs.dev/) — dev server and build tool
- [Vitest](https://vitest.dev/) — unit tests
- [Playwright](https://playwright.dev/) — browser tests

## Getting started

```sh
npm install
npm run dev
```

Open `http://localhost:5173` in a browser.

## Scripts

| Command | What it does |
|---|---|
| `npm run dev` | Start the Vite dev server |
| `npm run build` | Compile and bundle to `dist/` |
| `npm run typecheck` | Run `tsc` without emitting files |
| `npm run test:unit` | Run Vitest unit tests |
| `npm run test:browser` | Run Playwright browser tests |
| `npm test` | Run unit tests then browser tests |

## Tests

The project has two test suites.

**Unit tests** (`tests/unit/`) run in Node.js via Vitest with no browser or DOM required. They cover all pure logic in `src/bookmarks.ts`: title and URL validation, tag parsing, localStorage read/write, ID generation, and sort stability.

```sh
npm run test:unit
```

**Browser tests** (`tests/browser/`) run in Chromium via Playwright against the live Vite dev server. They verify end-to-end behavior: form submission, rendered output, link attributes, localStorage contents, and error states. Playwright starts the dev server automatically — no need to run `npm run dev` separately.

```sh
# Install Chromium the first time
npx playwright install chromium

npm run test:browser
```

**Run both suites together:**

```sh
npm test
```

Current coverage: 31 unit tests, 22 browser tests.

## Project structure

```
src/
  bookmarks.ts   Pure logic: validation, storage, ID generation, sorting
  main.ts        DOM wiring: form handling and list rendering
tests/
  unit/          Vitest tests for src/bookmarks.ts (no DOM required)
  browser/       Playwright tests against the running app
index.html       App shell
styles.css       Styles
```

## Documentation

| File | Contents |
|---|---|
| `DESIGN.md` | Purpose, features, technology choices, constraints, and testing methodology |
| `TODO.md` | Build layers with testable acceptance criteria; tracks completion |
| `REFINEMENT_LOG.md` | Chronological record of design and process decisions |
| `CHANGELOG.md` | What changed in each work session |
| `ADVERSARIAL.md` | QA review log: bugs found, test weaknesses, and dismissed findings |
