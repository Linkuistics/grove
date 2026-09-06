# lease-size-ranking-k171

## Goal

Correct the two places that rank `crates/grove-loop/src/driver_lease.rs` by size
and get the rank wrong — the structure brief's chapter 16 section and
`docs/walkthroughs/grove-loop/13-outcomes.md` — so that no later chapter inherits
either number.

## Context

- **The enumeration.** The corpus's thirteen roots by line count are
  `tree_lifecycle.rs` 2,725, `task_tree.rs` 2,023, `task_name.rs` 1,714,
  **`driver_lease.rs` 1,383**, `loop_driver.rs` 615, `task_grow.rs` 518,
  `lib.rs` 377, `verbs.rs` 363, `session_config.rs` 358, `prompt.rs` 245,
  `complete.rs` 96, `Cargo.toml` 59, `driver.rs` 57. `driver_lease.rs` is the
  **fourth** largest. Separately, the twenty owned blocks by line count begin
  819 (chapter 16), 808 (13), 775 (12), 700 (4) — so chapter 16's block is the
  **largest single owned block in the book**.
- **Defect 1 — `docs/specs/grove-loop-book-structure.md`, *16 · One live driver
  per working tree*.** *At 12% comment prose over 819 lines it is the
  thinnest-argued root in the corpus and the third largest.* The first clause is
  true — 103 comment lines in 819 is 12.6% and is the minimum across the corpus.
  The second is wrong under either reading: the **root** is fourth, and the
  **block** is first.
- **Defect 2 — `13-outcomes.md`, the *second thesis* paragraph.** *The second
  largest block in the corpus is 1,383 lines of locking whose whole purpose is
  to hold state the tree must not hold.* 1,383 is the whole root rather than a
  block — the root splits at its `#[cfg(test)]` line into 819 and 564, and
  neither is 1,383 — and as a root it is fourth rather than second. **The
  argument the sentence makes is correct and only the ranking is wrong**, which
  is why chapter 16 inherits the argument and states the sizes itself rather
  than repeating the number.
- **The phrase is invisible to a one-line grep** in the structure brief, where it
  wraps as `the third\nlargest`. Enumerating the roots is what finds this class;
  searching for the sentence is not.
- **Nothing in the frozen corpus changes**, so this leaf is not deferred behind
  the book: both defects are in a specification and a book page, and no
  ownership range, manifest value or fragment range moves.

## Done when

- The structure brief's chapter 16 section states the rank that enumeration
  gives, or drops the ranking clause; its *thinnest-argued* clause is left alone,
  because it is true.
- `13-outcomes.md`'s sentence keeps its argument and states a size a reader can
  check — the root's 1,383 lines without a rank, or chapter 16's 819-line block
  as the largest — and chapter 16's own *The rule* section still agrees with it.
- No other page or brief repeats either ranking. Check by **enumerating the
  roots**, not by grepping a phrase.
- `book-check --repo . --book docs/walkthroughs/grove-loop --check all` is green
  at whatever slice the book is proved at when this runs, and
  `bash scripts/check.sh` is no worse than before.

## Notes

**Placed before chapter 17** so the remaining Part V chapters, and chapter 21's
assembly page, read a corrected brief. That is the same placement
`pick-test-count-k147` and `structure-brief-chapter-attributions-k163` took for
the same reason.

**Do not widen this into a sweep of every size claim in the book.** Chapter 13's
own *third largest inline-test block in the book behind chapter 12's 569 and
chapter 14's …* was not checked here and is not this leaf's; a chapter that
carries a ranking checks its own by enumeration.

## Decisions (running log)
