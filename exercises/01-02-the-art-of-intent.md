1. A personal recipe collection that lets you save, search, and organize recipes

## What I'm Building
A personal recipe collection web app for saving, organizing, and sharing recipes, built entirely in vanilla TypeScript, HTML, and CSS.

## Who It's For
A single user for personal use.

## What It Should Do
- Add, edit, delete, and reorder recipes
- Each recipe requires a description, ingredients, and instructions; a picture is optional
- Display recipes in a list view; clicking a recipe opens a detailed view
- Persist recipe data in browser storage
- Export individual recipes as JSON
- Share individual recipes as Markdown
- Import a single recipe from a JSON file that conforms to the app's schema

## What It Should Look Like
Minimal and modern. The UI must be responsive for mobile devices.

## What It Should NOT Do
- No user accounts or authentication
- No social or sharing features beyond exporting/importing individual recipes
- No gamification
- Nothing beyond the minimal viable product

## Additional Context
- Must use vanilla TypeScript, HTML, and CSS — no frameworks or build tools beyond what's necessary for TypeScript
- A JSON schema must be defined to represent the recipe data structure
- Imported recipes must validate against that schema before being accepted

2. A simple timer app that lets you create named timers (like "Laundry" or "Meeting in 30 min")

## What I'm Building
A personal named-timer app for running multiple concurrent countdown timers, built entirely in vanilla TypeScript, HTML, and CSS.

## Who It's For
A single user for personal use.

## What It Should Do
- Display timers in a list, sorted most recently used to least recently used
- Add, delete, and rename timers
- Each timer requires a name and a duration
- Clicking a timer starts it; clicking it again pauses it
- Multiple timers can run simultaneously
- When a timer completes, display a browser alert and play a sound
- Persist timer data (names and durations) in browser storage

## What It Should Look Like
Minimal and modern. The UI must be responsive for mobile devices.

## What It Should NOT Do
- No user accounts or authentication
- No gamification
- No recurring or repeating timers
- No timer history or analytics
- Nothing beyond the minimal viable product

## Additional Context
- Must use vanilla TypeScript, HTML, and CSS — no frameworks or build tools beyond what's necessary for TypeScript
- Use the Web Audio API to generate the completion sound — no external audio files

3. A reading list tracker where you can add books, mark them as read, and rate them

## What I'm Building
A personal reading list tracker for managing books you want to read and have read, built entirely in vanilla TypeScript, HTML, and CSS.

## Who It's For
A single user for personal use.

## What It Should Do
- Display books in a list, with unread books shown before read books
- Add, edit, and delete books
- Each book requires a title and author
- Toggle a book between read and unread
- Rate read books on a 1–5 star scale
- Persist data in browser storage

## What It Should Look Like
Minimal and modern. The UI must be responsive for mobile devices.

## What It Should NOT Do
- No user accounts or authentication
- No social or sharing features
- No gamification
- No external book metadata lookups or ISBN scanning
- Nothing beyond the minimal viable product

## Additional Context
- Must use vanilla TypeScript, HTML, and CSS — no frameworks or build tools beyond what's necessary for TypeScript
