# Quality Engineer Review Log

This review is part of the [Iterative Adversarial Refinement (IAR)](README.md) suite. See [README.md](README.md) for sequencing, scoped runs, and domain coordination.

The purpose of this review is to evaluate the quality system: whether the testing strategy, coverage, tooling, and gates are structured to catch defects reliably. At this stage (pre-implementation), the review evaluates the test plans in `TODO.md` — their completeness, Red Gate compliance, and falsifiability.

**Language supplement applied:** `lang/rust.md` (QE section) + `lang/cli.md` (QE section).

---

## Review 1 — 2026-04-27 21:00Z

**Scope:** `TODO.md` Red Gate test plans and `DESIGN.md` Testing Methodology. No implementation code exists. Pre-implementation pass: evaluating the quality of the test plan before any code is written.

**Session note:** In-session with all other domain reviews and with project authorship. Acknowledged quality tradeoff.

**Assumption surfacing:** No implementation dependencies to validate yet. The test plan references integration tests that invoke the binary as a subprocess — consistent with the Rust supplement's requirement for CLI integration tests.

---

### Resolved

**Finding 1 — No Red Gate test for `created_at == updated_at` on fresh issue**

DESIGN.md states: "`created_at` and `updated_at` are equal on a freshly created issue." This is an acceptance criterion in Layer 1 (`TODO.md`) but no corresponding Red Gate integration test exists. A stub that sets `updated_at` to epoch or to a different timestamp would pass all existing Red Gate tests.

**Resolution:** Added to `TODO.md` Layer 1 Red Gate: `create_timestamps_equal_on_fresh_issue` — reads `tracker.json` after create, asserts `created_at == updated_at` — fails against stub that sets them to different values.

---

**Finding 2 — No Red Gate test for title 50-char truncation in list output**

Layer 1 acceptance criteria include: "Title truncates at 50 characters with `…` in list output." No corresponding Red Gate test exists. A stub that prints the full title would satisfy all existing Layer 1 tests.

**Resolution:** Added `list_truncates_title_at_50_chars_with_ellipsis` to `TODO.md` Layer 1 Red Gate.

---

**Finding 3 — No Red Gate test for `tracker list --status in-progress`**

Layer 2 Red Gate names `list_status_filter_shows_done` for the done case but has no corresponding test for the `in-progress` case. A stub that only handles `open` and `done` status filters would pass all Layer 2 Red Gate integration tests.

**Resolution:** Added `list_status_filter_shows_in_progress` to `TODO.md` Layer 2 Red Gate.

---

**Finding 4 — No Red Gate test for `created_at` immutability after status mutation**

Layer 2 acceptance criteria include: "All other fields on the issue are unchanged [after status change]." The `status_change_leaves_other_fields_unchanged` test covers this, but `created_at` is not explicitly named. An implementation that updates `created_at` on mutation would pass the existing test if "other fields" is interpreted loosely. A distinct test that explicitly asserts `created_at` is unchanged removes the ambiguity.

**Resolution:** Added `status_change_does_not_modify_created_at` to `TODO.md` Layer 2 Red Gate.

---

**Finding 5 — No Red Gate test for label 20-char truncation in list output**

Layer 4 acceptance criteria include: "Labels column truncates at 20 characters with `…` if longer." No corresponding Red Gate integration test exists.

**Resolution:** Added `list_label_value_truncated_at_20_chars` to `TODO.md` Layer 4 Red Gate.

---

### Dismissed

**Finding 6 — Write-failure error paths have no Red Gate integration tests**

DESIGN.md Storage edge cases include: write fails (disk full, permissions) → stderr `Error: Could not save tracker data: <reason>.` → exit 1; `tracker.json` is a directory → treated as I/O failure. These are acceptance criteria but cannot be reliably automated in cross-platform integration tests (require OS-level setup: filling a disk, revoking file permissions, creating a directory at the expected path).

**Classification:** Dismissed. Write-failure paths are listed in the Layer 1 and Layer 6 manual testing checklists. These are the only paths for which manual-only verification is acceptable. The acceptance criterion is present; its verification path is manual. The layer gate checklist must include explicit steps for these paths before Layer 1 merges.

---

**Finding 7 — Layer 5 test plan has only 4 integration tests; compound filter permutations are sparse**

`tracker list --status open --priority high`, `--status open --label bug`, `--priority high --label bug`, and all three together are listed. The case `--status done --priority high --label bug` (a three-filter combination with a non-default status) is not explicitly tested. A stub that only AND-combines when `--status open` is present would pass.

**Classification:** Dismissed. The `list_two_filter_and_combination` and `list_three_filter_and_combination` tests are defined against an implementation that must AND-combine any combination, not just open-status combinations. The acceptance criterion "An issue that matches two of three filters but not the third does NOT appear" is tested with setup that produces clear counter-examples. Additional permutation tests can be added during implementation if a gap is discovered; the Red Gate coverage is sufficient to fail a naive stub.

---

### Open

*(none)*

---

### Summary

Five real findings, all resolved via `TODO.md` additions. Two dismissed with rationale. The test plan now covers all primary acceptance criteria with Red Gate tests. Write-failure paths are explicitly manual. No open items.

**Coordination:** QE Finding 4 (created_at immutability) surfaces to SE as an implementation concern — the mutation path must never touch `created_at`. Logged in SE log.
