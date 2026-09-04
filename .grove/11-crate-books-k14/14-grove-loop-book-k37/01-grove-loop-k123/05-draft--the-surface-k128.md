# the-surface-k128

## Goal

Draft Part IV of the `grove-loop` book — chapter 15, owning
`crates/grove-loop/src/verbs.rs` (363), `src/driver.rs` (57) and
`src/complete.rs` (96), 516 lines in three blocks — and prove the prefix through
slice `twelve-not-fourteen`.

## Context

- Draft stage, child 5 of 7 of `grove-loop-k123`, and the only part that is one
  chapter. The structure brief is `docs/specs/grove-loop-book-structure.md`, and
  this chapter is its *15 · The twelve verbs, and the two that are not*.
- **One session, no decomposition expected.** 516 lines over three roots, each
  owned whole, and the three are 57%, 73% and 65% comment prose — the most
  argued-in-situ material in the corpus. The obligation here is *do not restate*:
  the comments make the argument and the fragment graph quotes them verbatim.
- **The rule: the surface is twelve verbs, and everything else that touches the
  tree has to say why it is not one.** `verbs.rs` declares fourteen `pub fn`;
  `stale_cross_refs` and `signal_channel` each say in their own doc comment why
  they are not verbs. `driver.rs` holds two more tree operations that are not
  verbs and states that putting them beside the twelve *would say the surface has
  fourteen verbs, which it does not*. `complete.rs` is the twelfth verb and the
  loop's child-side half.
- **Count before writing the count.** *Twelve*, *fourteen* and *two* are the
  chapter's whole argument and each is a claim about the file in front of you:
  enumerate the `pub fn` declarations in `verbs.rs` and the operations in
  `driver.rs` rather than repeating this paragraph.
- Chapter 1's cast rows owned by `twelve-not-fourteen` — `verbs::stale_cross_refs`
  and `verbs::finish_commit`, and `interpret` with `Disposition` — move to
  `explained` when this chapter lands.
- The chapter cites the guide anchor `usage-tree-verbs` and the glossary anchors
  `loop-control-channel` and `task-commit-boundary`; all three exist today.
- `report_insert` is named by `verbs.rs`'s own comment as the owner of a decision
  this crate declines to make. It belongs to the binaries above, and the brief
  puts it outside this book: name the boundary, do not cross it.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  twelve-not-fourteen --check all` is valid: 13 files, 7,932 resolved lines,
  2,601 deferred, `final=false`.
- Chapter 15 exists, contents and navigation are updated, and the three
  ownership rows for `verbs.rs`, `driver.rs` and `complete.rs` read `resolved`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

**The corpus is frozen.** A defect found while drafting becomes its own leaf and
is not fixed inline.

**The last act is this part's only chain obligation and it is nothing**: the next
stage is cut by child 7, not here.

## Decisions (running log)
