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

## Layer 2: Status flow

### Phases

**Decomposition**

DESIGN.md already covered Layer 2 (`tracker status` command, `--status` filter). TODO.md Layer 2 was filled in pre-Red-Gate with 16 acceptance criteria and 8 manual testing checklist items, plus a Red Gate plan listing 15 integration tests + 3 unit tests. No new spec phase was needed — the spec phase (Layer 1) had already specified all of Layer 2's behaviors.

**Red Gate (2026-05-01)**

Commit `04f0d22` introduced the Layer 2 Red Gate: 18 tests added and confirmed failing against stubs (no `Status` subcommand, no `parse_status`, no `--status` filter on `cmd_list`). The Red Gate commit precedes the implementation commit by ~33 minutes (`c873b69`). Test discipline at the commit-pattern level is intact.

**Implementation (2026-05-01)**

`c873b69` added `parse_status`, `parse_id`, `cmd_status`, and the `--status` filter wiring. The biggest implementation friction was the borrow-checker conflict in `cmd_status`: the natural shape (`iter_mut().find()` to get a `&mut Issue`, mutate, then `save_issues(&issues)`) produced a borrow conflict because the `&mut Issue` extends through the surrounding `println!`, conflicting with the `&issues` immutable borrow that `save_issues` requires. The first attempt used `new_status.clone()` to escape the conflict — which worked but was wasteful. SE Review 7 caught this and refactored to `iter().position()`, which returns a `usize` index that carries no borrow. Zero clones, no conflict. The lesson: when borrow-checker pressure produces an unnecessary clone, the right response is usually to restructure the access pattern, not to clone.

**IAR iterations**

Layer 2 round 1 (in-session) produced four real findings:
- **QE Review 7** added `list_nonempty_status_filter_with_no_match_shows_filter_message` — verified that `tracker list --status done` with no done issues prints "No issues match the given filters." This caught a mutation in `is_open_view` that would have survived all 37 prior tests. (This same heuristic later regressed in Layer 3, caught by SO Review 11.)
- **SA Review 6** unified `parse_status` against `VALID_STATUSES` — the `match` arm in `parse_status` was a second source of truth for the valid status set, separate from the `VALID_STATUSES` slice used in `issue_fields_are_valid`. SA's deferred enum item from Layer 1 came due, and the minimum-correct fix was to iterate the slice rather than introduce an enum.
- **SE Review 7** caught the unnecessary `new_status.clone()` and refactored to `iter().position()` (above).
- **SO Review 10** found that CHANGELOG.md and README.md were stale (no Layer 2 entry; status block still said "Layer 1 complete"). Same documentation-currency pattern recurred at Layer 3 — same finding pattern, same fix.

A cold-session QE pass (Review 8, 2026-05-02) ran the day after the in-session pass and found two more real defects:
- The truncation test caught one off-by-one mutation but not the symmetric one (truncating to 50 content chars + `…` = 51 display chars). Fixed.
- `status_not_found_exits_one` asserted `contains("not found")` rather than the spec-mandated full message including the issue ID. Fixed.

The cold-session pass produced higher-value findings than round 1 in the same domain. This is the expected value of cold-session adversarial review.

**Gate closure (2026-05-02)**

Layer 2 merged via PR #11. VDD-IAR Review 8 had two Open items at merge time:
- **Finding 2** (cold-session requirement) — partially closed by QE Review 8, but other domains (SO, SA, SE, VDD-IAR) did not get a cold-session round 2 for Layer 2.
- **Finding 3** (MVR via second IAR pass) — QE round 2 produced new real findings, but no formal third pass confirmed MVR for QE; other domains did not get a round 2 at all.

Layer 2 merged with these items technically Open. The director's judgment was that the cold-session pass had been done for the highest-leverage domain (QE) and the remaining domains had no reasonable signal of pending defects. VDD-IAR Review 9 (Layer 3) flagged this pattern: the "gates merge" classification needs an explicit closure mechanism, not just director judgment.

---

### What was hardest

*[Your reflection here — borrow-checker pressure in `cmd_status`? The cold-session deficit decision? The fact that Layer 2 closed with Open VDD-IAR items?]*

---

### What I got wrong

The first `cmd_status` implementation cloned `new_status` to escape a borrow conflict rather than restructuring the access pattern. SE Review 7 caught it. The right shape (`iter().position()` returning an index) was not the first thing I reached for; I reached for `iter_mut().find()` and then patched the borrow conflict with a clone.

The cold-session deficit was a process decision rather than a code error: I ran round 1 in-session with the implementation, then closed the layer with only one cold-session pass (QE) rather than fanning out cold sessions across all domains. The artifact (VDD-IAR Open items) reflects this.

*[Anything else?]*

---

### What the process felt like

*[First-person reflection on Layer 2.]*

---

## Layer 3: Priority

### Phases

**Decomposition**

DESIGN.md already covered Layer 3 (`--priority` on create, `--priority` filter, sort by priority then ID). TODO.md Layer 3 was filled in with 11 acceptance criteria, 8 manual testing checklist items, and a Red Gate plan of 7 integration tests + 4 unit tests. The decomposition was unchanged from spec phase.

**Red Gate (2026-05-03 21:57 PDT)**

Commit `71d2137` introduced the 11 Red Gate tests against stubs (no `--priority` flag in clap, no `parse_priority`, no `sort_issues`). All Red Gate tests fail without the Layer 3 implementation. The Red Gate commit precedes the implementation commit by 4 minutes.

**Implementation (2026-05-03 22:01 PDT)**

`caf5f9a` added `parse_priority`, `priority_rank`, `sort_issues`, the `--priority` flag on `cmd_create`, and the `--priority` filter on `cmd_list`. Implementation was mechanical given the Layer 2 patterns: `parse_priority` mirrors `parse_status` (case-insensitive, validates against a constant slice). `sort_issues` was the only novel piece and used the standard `cmp().then(...)` pattern for compound ordering. The `cmd_list` extension added a second `retain` for the priority filter, AND-combined with the status filter.

**Manual testing complete (2026-05-03 22:22 PDT)**

Director ran the binary against the manual testing checklist and signed off in commit `6f7fd46`. All 8 checklist items checked.

**IAR iterations**

Layer 3 round 1 produced four real findings, with one cold-session pass (SO) and three same-session passes (SA, QE, SE) plus a same-session VDD-IAR meta pass:

- **SO Review 11** (cold session) caught an `is_open_view` empty-state regression introduced by Layer 3. Adding the `--priority` filter without updating the empty-state heuristic meant `tracker list --priority high` with no matches printed "No open issues. Nice work!" instead of "No issues match the given filters." A user-observable spec violation. Fixed with one line: `is_open_view = effective_status == "open" && effective_priority.is_none()`. This is exactly the kind of regression that the QE Review 7 work (Layer 2) had set up the test pattern to catch — and the test was missing for Layer 3 until QE Review 9 added it.
- **SA Review 7** (director-applied) caught duplicate priority constants — `VALID_PRIORITIES` (membership) and `PRIORITY_ORDER` (sort rank) were maintained independently. The fix mirrors SA Review 6 from Layer 2: collapse to a single ordered slice, use `.contains()` for membership and `.iter().position()` for rank. The director chose to apply this directly rather than have an AI agent do it.
- **QE Review 9** (same session) added the regression test for SO Review 11's fix — `list_priority_filter_no_match_shows_filter_message`. Both positive (post-fix message present) and negative (pre-fix message absent) assertions, so neither half of the bug can pass alone.
- **SE Review 8** (same session) found `priority_rank`'s `usize::MAX` defensive fallback was undocumented — a future reader looking at the function would not know whether the fallback was reachable, why it routed to the bottom, or whether to panic instead. Added a doc comment explaining the unreachability invariant and the design choice.

Two findings remained Open at end of round 1:
- **SA Review 7 Finding 2** — `tracker()` test helper now duplicated across three files. SA Review 6 (Layer 2) had explicitly deferred extraction to "when Layer 3 introduces a third file." Layer 3 introduced `tests/layer3.rs`. The threshold met; extraction applied at gate closure (see below).
- **SE Review 8 Finding 2** — `is_open_view` is no longer accurately named after the SO Review 11 fix; the variable now also implicitly tracks "no priority filter applied." Recommended rename or helper extraction; held for a fresh-context SE pass.

**VDD-IAR Review 9** (same session, meta) flagged three Open gate items:
1. Cold-session deficit: only SO had a cold-session pass.
2. MVR not reached: round 1 only; two findings still Open.
3. PROCESS.md retrospective absent for Layer 2 (overdue) and Layer 3.

**Gate closure (in progress)**

The director split the remaining gate work:
- SE round 2 ran in a separate cold session.
- The orchestrator session applied the test helper extraction (SA F2 — Resolved) and wrote this retrospective entry (Finding 9 — addressed).
- SE F2 (`is_open_view` rename) held for the cold SE pass to address.

After SE round 2 lands and any new findings resolve, MVR for Layer 3 is the cold-session-confirmed exit.

---

### What was hardest

*[Your reflection here. Likely candidates: deciding to apply SA Review 7 directly rather than through an AI agent; the cold-session-vs-batched-session quality tradeoff for the in-flight IAR run; the discomfort of merging Layer 2 with Open VDD-IAR items and watching that pattern repeat at Layer 3.]*

---

### What I got wrong

The `is_open_view` regression (`tracker list --priority X` with no matches) is the kind of cross-layer regression that the IAR process is designed to catch and that this implementation pass did not catch in advance. The Layer 2 QE Review 7 test (`list_nonempty_status_filter_with_no_match_shows_filter_message`) had established exactly this empty-state pattern for status; the analogous priority test was not in the Red Gate plan. SO caught it in round 1, but the gap should have been visible from the Red Gate plan: every filter dimension should have an empty-state assertion. Layer 4 (labels) and Layer 5 (compound filters) need this lens applied at Red Gate time, not at SO review time.

The same-session IAR batch was a deliberate quality tradeoff (acknowledged in each entry's session note) rather than an unintentional process violation, but the tradeoff has a cost: same-session reviewers reconcile findings across domains rather than applying fresh adversarial pressure to each. The cold SO pass produced the only round-1 defect-class finding; the same-session passes mostly produced documentation, structural, and naming findings — the kinds of findings that survive cold-session pressure least.

*[Anything else?]*

---

### What the process felt like

*[First-person reflection on Layer 3. Possible threads: the recurrence of the documentation-currency pattern (CHANGELOG/README stale at every layer close — could a hook catch this?); the value of the cold SO session vs. the batched same-session domains; the decision to apply SA Review 7 directly rather than through an agent.]*
