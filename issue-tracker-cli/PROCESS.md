# Process Retrospective

This file records the process of building the issue tracker CLI — how decisions were made, what the adversarial review process caught, what was missed and later corrected, and what the experience of working this way actually felt like. It is a process artifact, not a feature description. For what was built, see CHANGELOG.md. For why decisions were made, see DECISIONS.md.

---

## Layer 1: Core create + list

### Phases

**Spec crystallization (2026-04-27)**

The spec phase ran before any code. DESIGN.md was written first, then reviewed by the IAR suite across 8 SO rounds and multiple SA, QE, Security, UX, Data Engineer, TW, and Platform passes. This felt like a lot of overhead before writing a single line of Rust — but several decisions that would have been wrong got corrected here rather than in the implementation:

- The storage format started as a wrapped object (`{"issues": [...], "next_id": ...}`). The `next_id` field was removed by SA Review 1 (unnecessary complexity). After that removal, the wrapper added nothing. SO Review 7 approved simplifying to a plain top-level array. The simpler format is what shipped. Without the spec review, I would have built the more complex format first.
- Post-deserialization validation was specified in Security Review 1 and Data Engineer Review 1 — the spec explicitly required treating semantically invalid field values as corrupt data. I noted this. Then I forgot to implement it.

**Decomposition**

TODO.md with 7 layers was written before implementation. Having the full layer plan before Layer 1 meant the Layer 1 scope was bounded from the start. The sort algorithm being "full algorithm from the start — not a simplified ID-only sort" came from SA Review 2 and was in the decomposition before any code. That kind of constraint is easy to miss without the review process.

**Red Gate (2026-04-27)**

17 integration and unit tests were written and confirmed failing before the implementation existed. Writing tests first forced behavioral precision — I had to decide exactly what `Created issue #1: Fix bug\n` meant before writing the code that produced it. The two places where this was uncomfortable were also the two places where the tests were most useful: the timestamp invariant (`created_at == updated_at` on fresh issue) and the title truncation (the test required knowing the exact 49-char + `…` form before I'd written the truncation function).

**Implementation (2026-04-27 to 2026-04-28)**

The implementation was straightforward once the spec and tests existed. The main decision at implementation time was the `lib.rs` / `main.rs` split — separating the library logic from the CLI entrypoint so integration tests could invoke the binary while unit tests could call functions directly. This pattern was in the SA review from the start.

**IAR iterations**

The most significant finding that the IAR process caught was the post-deserialization validation gap. I had read the spec requirement. I had noted it during the spec phase. Then I wrote `load_issues` without it. Three independent domains caught this independently: Security Review 3, Data Engineer Review 3, and Red Team Review 2. QE Review 4 added the test.

This is what adversarial review is for. The gap was not subtle — it was a spec requirement I explicitly had in front of me. But implementation focus narrows attention, and the adversarial domains caught the miss.

The other significant catch was the process violation: DESIGN.md was modified to reflect an implementation decision (storage format simplification) before the SO review had run. The VDD-IAR Alignment review caught this and required correcting the order — DESIGN.md reverted to draft, SO review ran, SO approved, DESIGN.md updated. The violation was minor in consequence but real in process: the authority chain was inverted.

**Gate closure (2026-04-30)**

After the implementation IAR suite, two additional adversarial passes ran before closing the gate:

1. A full suite pass that found `(none)` missing from a test assertion and `tracker.json` not gitignored (the latter surfaced during manual testing — running the binary from the project directory created a file that would have shown up as untracked in git).

2. A general adversarial pass using the review-session primer (explicit adversarial framing, obligation to the spec rather than the developer). This found two mutations that survived all 20 prior tests: the sort direction mutation (swapping `a.id.cmp` to `b.id.cmp` would have produced wrong list order with zero tests catching it) and the `id > 0` validation branch being independently removable. Both were fixed with new tests.

The review-session primer pass also found that prior "MVR reached" signals were premature — the process worked correctly in surfacing this.

Pre-commit hooks caught a real problem: the hook script used `$HOME` at runtime to avoid hardcoding usernames, but the review log text itself contained the literal username as example text from Review 2. The hook failed on the first run and required fixing the review documentation. The git history was then rewritten with `git filter-repo` to remove the historical occurrence from commit `f874a60`.

`git filter-repo` has a consequence that the manual does not emphasize: when run on a non-fresh clone (i.e., a working repo with an `origin` remote), it rewrites **all** commits in the local repository — not just the branch in use — and removes the `origin` remote as a side effect. The result was that every commit SHA in the local repo changed, including commits already shared with `main` on the remote. The local `issue-tracker-cli` branch was now based on a rewritten `main` that GitHub had never seen, producing the error "There isn't anything to compare — main and issue-tracker-cli are entirely different commit histories" when attempting to open a PR.

The fix required several steps:
1. Delete a corrupted remote tracking ref (`refs/remotes/origin/issue-tracker-cli 2` — a ref with a literal space in its name, created by a prior bad push attempt) that was causing every `git fetch` to fail.
2. Restore the `origin` remote (filter-repo removes it).
3. Reset local `main` to `origin/main` to recover the authoritative commit graph.
4. Identify that the remote `main` already contained the first two branch commits (merged as PR #9 earlier in the session), so only 5 commits needed to be replayed.
5. `git rebase --onto main <old-base-sha> issue-tracker-cli` to replay those 5 commits on top of real remote `main`.
6. Force-push the rebased branch.

The lesson: `git filter-repo` on a working repo should be treated as a full history replacement. Before running it, note the current remote `main` SHA, remove and re-add the remote afterward, and plan for a rebase to restore a common ancestor with the remote. Running it on a fresh clone (no remote) avoids the problem entirely.

---

### What was hardest

*[Your reflection here — what specifically was difficult, mentally or technically, about Layer 1? Was it the spec phase? The Red Gate discipline of writing tests first? The IAR iteration process? Something in the Rust implementation?]*

---

### What I got wrong

The post-deserialization validation gap is the most honest answer. I read the spec requirement. I acknowledged it during review. I did not implement it. The IAR process caught it, but it should have been in the first implementation pass.

*[Anything else you got wrong the first time? What surprised you about how the build went?]*

---

### What the process felt like

*[First-person reflection on the experience of working this way — spec-before-code, Red Gate, adversarial review. What was useful? What was friction? What would you do differently in Layer 2?]*

---

## Layer 2 and beyond

*(To be written after each layer closes.)*
