## Question 1

> Take this project idea: "A personal journal app where I can write daily entries, tag them with moods, and look back at past entries." Break it down into layers using the approach described above. What's the core? What are the layers? What's the build order?

**Core:** Write a journal entry and read it back. A text input, a save button, and a list of past entries with their dates. No moods, no tags, no search — just write something and see it later.

**Layers:**
1. Core: create an entry and display a list of past entries sorted by date
2. Layer: view a single entry in full (click to expand/open)
3. Layer: add mood tags when creating an entry (a fixed set — happy, sad, anxious, calm, etc.)
4. Layer: filter past entries by mood tag
5. Layer: edit and delete existing entries
6. Layer: search entries by text content
7. Layer: make it look and feel like a journal — typography, layout, polish

**Build order rationale:** Viewing a single entry (layer 2) comes before tagging (layer 3) because tags are useless if you can't open the entry they're attached to. Filtering by mood (layer 4) depends on tags existing, so it can't come before layer 3. Edit/delete (layer 5) is intentionally late — you want a stable data structure before adding mutation operations. Search (layer 6) is last among features because it's a convenience on top of already-working browsing.

---

## Question 2

> Take this project idea: "A tool for planning road trips. Enter your start and end points, add stops along the way, and see the total driving time." Write out the user stories. Each story should be one thing a user does.

1. "As a traveler, I want to enter a start point and destination, so that I can see a route and total driving time."
2. "As a traveler, I want to add a stop along my route, so that I can plan where to break up the drive."
3. "As a traveler, I want to see the total driving time update as I add or remove stops, so that I always know how long the full trip will take."
4. "As a traveler, I want to remove a stop, so that I can adjust my plans if something changes."
5. "As a traveler, I want to reorder my stops, so that I can find the most efficient route."
6. "As a traveler, I want to save a trip by name, so that I can come back to it without re-entering everything."
7. "As a traveler, I want to see a list of my saved trips, so that I can pick up a plan where I left off."

---

## Question 3

> Open a conversation with an agent and describe a project idea you're interested in. Ask the agent to help you decompose it. Review the agent's breakdown critically. Is anything too big? Too small? In the wrong order? Revise it.

### Project context summary

I use Obsidian.md for note-taking and VS Code for development. My Obsidian vault is my personal knowledge base — it contains links, raw data, screenshots, documents, TODOs, short idea descriptions, and project design documents spanning my personal life, work, side projects, and hobbies. I want to build a read-only integration between my Obsidian vault and Claude that lets me provide notes as context for conversations. Data integrity is a hard requirement: the integration must never edit, move, or delete any notes. The stack should work with VS Code as my primary development environment.

### Initial decomposition

**Core:** Read a single note by file path and return its contents as context. No search, no browsing, no link following — just: give Claude a path, get the note back read-only.

**Layers:**
1. Core: read a single note by file path and return its text content — with a hard read-only constraint enforced at this layer (no write, move, or delete permissions ever opened)
2. Layer: list all notes in the vault so you can see what's available to reference
3. Layer: search notes by keyword so you can find relevant notes without knowing the exact path
4. Layer: resolve Obsidian `[[wikilinks]]` so when a note links to another note, Claude can follow the link and read the linked note too
5. Layer: handle non-markdown files (images, PDFs, attachments) that are referenced in notes
6. Layer: filter by folder or tag to scope context to a specific area of the vault

**Open question at this stage:** what is the integration surface? An MCP server, a VS Code extension, or an Obsidian plugin — each has different tradeoffs and the decomposition can't be finalized without deciding.

### Refinement

**Refinement 1 — platform:** Decided the integration should be a VS Code or Obsidian extension so it can be packaged and published to their respective marketplaces. Tradeoffs: Obsidian has native vault API access and is better for following wikilinks and tags; VS Code has a larger audience and is more natural for referencing notes while coding.

**Refinement 2 — scope:** Decided to build both extensions. This changed the architecture: rather than two separate projects, the shared logic becomes a platform-agnostic core module published to npm, with two thin platform wrappers — one per extension. This avoids duplicating logic and lets the core diverge independently of the platform layers.

**Refinement 3 — user independence:** Each extension must be usable without the other installed. This made explicit that the core module must be separately packaged — each extension declares it as a dependency rather than bundling a copy.

**Refinement 4 — documentation:** Documentation is required for all three scenarios: core module only, Obsidian-only, VS Code-only, and using both. Docs for each scenario are written after their respective platform layer is complete and assume the user has not installed the other extension.

**Refinement 5 — testing:** All functionality must be unit tested. Scope restrictions and data integrity constraints must pass testing before anything can be published. This added hard publish gates: publish core is blocked until all core tests pass; publish extensions are blocked until platform tests pass.

**Refinement 6 — TDD:** Tests must be written before the implementation they cover. This flipped every feature/test pair so tests come first and implementation makes them pass.

### Final decomposition

1. **Tests:** write failing tests for read a note by path, enforce read-only — including tests that verify write, move, and delete operations are not possible
2. **Core module:** read a note by path, enforce read-only — make the tests pass
3. **Tests:** write failing tests for list notes
4. **Core module:** list notes — make the tests pass
5. **Tests:** write failing tests for search notes
6. **Core module:** search notes — make the tests pass
7. **Tests:** write failing tests for wikilink resolution — including tests that verify resolution never triggers a write
8. **Core module:** resolve `[[wikilinks]]` — make the tests pass
9. **Tests:** write failing tests for folder and tag filtering
10. **Core module:** filter by folder or tag — make the tests pass
11. **Tests:** write failing tests for non-markdown attachment handling — including tests that verify attachments are read-only
12. **Core module:** handle non-markdown attachments — make the tests pass
13. **Package and publish the core module** to npm — all core tests must pass before this step
14. **Tests:** write failing tests for the Obsidian plugin — scope restrictions, data integrity, integration with core
15. **Platform layer:** Obsidian plugin — make the tests pass
16. **Tests:** write failing tests for the VS Code extension — scope restrictions, data integrity, integration with core
17. **Platform layer:** VS Code extension — make the tests pass
18. **Package and publish each extension** — platform tests must pass before this step
19. **Docs:** core module — installation and usage examples
20. **Docs:** Obsidian-only setup
21. **Docs:** VS Code-only setup
22. **Docs:** using both

**Hard gates:** layer 13 (publish core) is blocked until layers 2, 4, 6, 8, 10, and 12 all pass. Layer 18 (publish extensions) is blocked until layers 15 and 17 pass. Layers 15 and 17 are independent of each other — build whichever platform first.

---

## Question 4

> Share your decomposition in the guild [Discord](https://discord.gg/kfM6Q4UBbM) **#apprentice-level** channel and ask for feedback. Can other members spot dependencies you missed? Steps that could be broken down further? This is lightweight adversarial review, exactly the kind of thinking the guild develops.

Shared to the guild Discord **#apprentice-level** channel on 2026-04-23.
