## What this PR does

<!-- Which layer(s) or tasks from TODO.md does this cover? Link to specific items if helpful. -->

## Layer gate checklist

- [ ] All acceptance criteria in TODO.md are met
- [ ] Unit tests pass (`npm run test:unit`)
- [ ] Browser tests pass (`npm run test:browser`)
- [ ] `bookmarks.ts` maintains 100% statement, branch, and function coverage (`npm run test:coverage`)
- [ ] Manual testing checklist in TODO.md completed against the running app
- [ ] AIR suite run (see `adversarial-iterative-refinement/README.md`); all domains complete
- [ ] QA: all findings resolved or dismissed — logged in `adversarial-iterative-refinement/QA-REVIEW.md`
- [ ] UX: all findings resolved or dismissed — logged in `adversarial-iterative-refinement/UX-REVIEW.md`
- [ ] Security: all findings resolved, dismissed, or risk accepted — logged in `adversarial-iterative-refinement/SECURITY-REVIEW.md`
- [ ] SA: layer boundaries intact, no mutation inconsistencies, decisions documented — logged in `adversarial-iterative-refinement/SOLUTION-ARCHITECT-REVIEW.md`
- [ ] PE: pipeline complete, coverage enforced, audit clean — logged in `adversarial-iterative-refinement/PLATFORM-ENGINEERING-REVIEW.md`
- [ ] `CHANGELOG.md` updated
- [ ] `DECISIONS.md` updated

## Test results

| Suite | Result |
|---|---|
| Unit tests | <!-- e.g. 42 passed --> |
| Browser tests | <!-- e.g. 38 passed --> |

## Notes

<!-- Anything worth calling out: dismissed QA findings, deferred scope, known issues, design decisions made during implementation. -->
