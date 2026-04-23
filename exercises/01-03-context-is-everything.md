 1. Take one of the prompts you wrote in the previous chapter's exercises. Now write the context you would need to provide alongside it. What project context, technical context, constraints, and preferences would the agent need?

 I am a home cook building a recipe app. It needs to be a single-page web app using TypeScript, HTML, and CSS that's easy to use and navigate on my phone because I will be using it to reference the recipes and instructions while I'm in the kitchen cooking. I want to implement a search bar that filters by ingredients so I can see that I have an ingredient like chicken then filter my recipes to see what recipes have chicken in the name, description, or ingredients.

2. Start a conversation with an agent. Give it zero context and ask it to "add a dark mode toggle." Note what it produces. Then start a new conversation, provide full context about a specific project, and make the same request. Compare the results. This exercise shows the impact of context better than any explanation can.

### Zero-context result

**Without context, the agent was stuck.** The repo contains only markdown files — no HTML, CSS, JavaScript, or any web project. There was nothing to add a dark mode toggle *to*.

Without context, the agent had to ask:
- What framework? (React, Vue, vanilla HTML, Next.js, etc.)
- Where does styling live? (CSS variables, Tailwind, CSS-in-JS, etc.)
- What does "toggle" mean here — a button, OS preference detection, localStorage persistence?

The exercise worked exactly as intended: zero context produced zero useful output, or worse — confident output that would be wrong for the project.

When running the comparison with full context (a specific project, stack, file paths, and preferences), the difference is stark. The agent produces code that fits, not a generic snippet that needs to be adapted.

### Full-context result

With specific context — stack (TypeScript, HTML, CSS), platform (mobile), and environment (dim kitchen, no overhead lights) — the agent made targeted decisions immediately, without asking any clarifying questions:

- **Warm dark palette, not pure black** — cool/blue-tinted darks increase eye strain in already dim environments
- **Large tap target for the toggle** — kitchen use means potentially wet or floury hands
- **Persist the preference in `localStorage`** — no re-toggling every time the app is opened mid-cook
- **CSS custom properties** for theming — the right approach for a vanilla TypeScript/HTML/CSS stack
- **Default to dark if `prefers-color-scheme: dark` is set** — reflects the environment preference already set at the OS level

The context didn't just answer the clarifying questions from the zero-context attempt — it surfaced decisions the agent wouldn't have known to make at all, like the warm palette choice driven by the specific low-light kitchen environment.

3. Write a one-paragraph "context summary" for a project idea you have, something you could paste at the start of any conversation to instantly ground the agent. Share it in the guild for feedback.

I use Obsidian.md for note-taking and VS Code for development. My Obsidian vault is my personal knowledge base — it contains links, raw data, screenshots, documents, TODOs, short idea descriptions, and project design documents spanning my personal life, work, side projects, and hobbies. I want to build a read-only integration between my Obsidian vault and Claude that lets me provide notes as context for conversations. Data integrity is a hard requirement: the integration must never edit, move, or delete any notes. The stack should work with VS Code as my primary development environment.