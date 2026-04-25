## Question 1

> Deliberately break something. Take your bookmark manager and introduce a bug: delete a closing bracket, misspell a variable name, or remove a function call. Then pretend you don't know where the bug is. Read the error message. Can you find the problem from the error alone, before fixing it?

**Bug introduced:** Misspelled `createdAt` as `creatdAt` in the `sortBookmarks` function in `src/bookmarks.ts`.

**Error output:**

```
src/bookmarks.ts(86,7): error TS2551: Property 'creatdAt' does not exist on type 'Bookmark'. Did you mean 'createdAt'?
src/bookmarks.ts(86,20): error TS2551: Property 'creatdAt' does not exist on type 'Bookmark'. Did you mean 'createdAt'?
```

**What the error tells you:**

The error gives everything needed to fix the bug without running the app or reading any surrounding code:

- **File and line number** (`src/bookmarks.ts(86,7)` and `(86,20)`) pinpoint the exact location — both occurrences on the same line.
- **Error code** (`TS2551`) is a property-does-not-exist variant, meaning the type system knows what properties `Bookmark` has and `creatdAt` isn't one of them.
- **"Did you mean 'createdAt'?"** — TypeScript found the closest matching property name and suggested it directly in the error. No guessing required.

The fix was unambiguous from the error alone: two characters transposed, flagged at the exact column, with the correct spelling supplied. This is the best-case scenario for a typo bug — the type system caught it at compile time before the code ran at all. A dynamically typed language would have silently treated the missing property as `undefined`, and the sort would have produced `NaN` comparisons and undefined ordering with no error anywhere.

## Question 2

> Ask your agent to build something slightly ambiguous on purpose: "make a timer." Don't specify what kind of timer, what it counts, or how it displays. Look at what you get. Identify every gap between what you wanted and what the agent assumed. Write the corrected prompt that would have gotten the right result the first time.

**What was built:** A standalone HTML countdown timer (`exercises/timer.html`). It counts down from 5:00, displays MM:SS, and has Start, Pause, and Reset buttons. When it reaches zero it shows "Done."

**Assumptions made — and what I might have wanted instead:**

| Assumption | Other reasonable interpretation |
|---|---|
| Countdown timer (counts down to zero) | Stopwatch (counts up from zero) |
| Fixed 5-minute duration hardcoded | User sets the duration before starting |
| No alert or sound at the end | Plays a sound or shows a browser notification when done |
| MM:SS display format | HH:MM:SS, or just total seconds, or a progress ring |
| Single timer | Multiple concurrent named timers (kitchen timer use case) |
| Standalone HTML file | A component to embed in an existing page |
| Pause/resume | No pause — start and reset only |

Every one of these was a decision point with no signal from the prompt. The result is a plausible timer that could be wrong in six different ways for six different people.

**Corrected prompt:**

> Build a countdown timer as a standalone HTML file with no external dependencies.
>
> The user types in a duration in MM:SS format before starting. The timer counts down to 00:00 and plays a short beep (Web Audio API, no audio files) when it reaches zero. Display the remaining time in large MM:SS digits. Buttons: Start, Pause/Resume (toggled), Reset. Reset returns to the user's entered duration, not a hardcoded default.
>
> Styling: centered on the page, readable at a glance, no framework.

The gap between the vague prompt and the corrected one is entirely specification: *what kind* (countdown), *what it counts* (user-entered MM:SS), *how it displays* (large MM:SS digits), and *what happens at zero* (beep). None of that is inferable from "make a timer."

## Question 3

> Start a conversation with your agent and build something small (a unit converter, a tip calculator, whatever). When it works, break it by asking for a feature that conflicts with the existing code: "now make it work backwards too." Watch how the agent handles the conflict. If it introduces a bug, practice the debugging loop: read the error, share it back, iterate.

**What was built:** A tip calculator (`exercises/tip-calculator.html`) — bill input, tip preset buttons (15/18/20/25%), custom tip %, split by N people, live outputs for tip amount, total, and per-person cost.

**Conflicting feature requested:** "Add a feature for calculating the bill's miles per gallon."

**What the agent did:** Added the MPG calculator without pushback — miles driven input, gallons used input, live MPG output. It works. There are no bugs and no errors.

**The conflict:** MPG calculation has nothing to do with a tip calculator. The two features share no inputs, no outputs, no domain, and no user. Putting them in the same file produces something that is technically correct and completely incoherent. The agent complied because the request was unambiguous at an implementation level — "add these inputs, compute this formula, show this output" has a clear mechanical answer. The incoherence is at the product level, and agents don't push back on product decisions unless asked.

**What this illustrates:** The agent's job is to execute stated intent. It doesn't have a model of what the thing is *for*, so it can't flag that two features belong in different tools. A code review wouldn't catch this either — the code is correct. The only check that catches "these two things shouldn't be in the same file" is a human with context about what the tool is supposed to do. Prompt review and product judgment are not substitutes for each other.