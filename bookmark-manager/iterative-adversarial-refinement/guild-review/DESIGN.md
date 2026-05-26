<!-- hook-bypass[check-spec-frozen]: this DESIGN.md predates the R95 F3 hook-enforced spec-frozen attestation discipline. The bookmark-manager project uses a section structure that pre-dates the primer 1ab template alias set (Purpose / Features / Technology / Interface / Out of Scope) and does not enumerate the Data Model / Constraints / Edge Cases / Testing Methodology sections the post-2026-05-26 spec-frozen contract requires. Per the forward-only convention, pre-existing project DESIGN.md files are not retroactively required to refactor; the bypass mechanism is itself a finding for the next registry-walk review. -->
# Bookmark Manager - Design Document

## Purpose
A personal tool for saving and organizing web links with notes.
Single user, no authentication needed. Runs in a web browser.

## Features
1. Add a bookmark: URL, title, optional note, optional tags
2. Display all bookmarks in a list, newest first
3. Click a bookmark to open the link in a new tab
4. Edit a bookmark's title, note, or tags
5. Delete a bookmark
6. Filter bookmarks by tag
7. Search bookmarks by title or note content

## Technology
- HTML, CSS, JavaScript (no frameworks)
- Data stored in the browser's local storage
- Single page, no server needed

## Interface
- Clean, minimal design
- Add form at the top
- Bookmark list below
- Tag filter as clickable buttons above the list
- Search bar above the list, next to the tag filters

## Out of Scope
- User accounts or sharing
- Bookmark folders or nested organization
- Browser extension for quick saving
- Import/export (can add later)