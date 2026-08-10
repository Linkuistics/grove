# dead-non-launch-exports-k166

## Goal

Resolve the public exports that have no caller anywhere and are outside the
legacy-launch surface: either delete them or record why the surface is kept
whole.

## Context

- Surfaced by `legacy-launch-cleanup-k83`, whose scope was explicitly bounded to
  artifacts the legacy-launch removals made unreachable. These predate those
  removals, so they were left alone rather than swept in.
- The library exports almost everything as `pub`, so `dead_code` reports none of
  this. `legacy-launch-cleanup-k83` found it by copying `src/` to a scratch
  crate, making every module private except `cli` and `llm_cli`, and reading the
  compiler's reachability warnings; that technique is the cheap way to reproduce
  the list.
- Two distinct groups, and they may deserve opposite answers:
  - **No caller at all.** `repo`'s `pub use finish_commit::commit_finish` and
    its `commit_finish` function; the `validate_finish_commit` re-export, whose
    function is live only inside `finish_commit`.
  - **Test-only locked wrappers.** `tree_grow::{leaf_add, leaf_add_chain,
    leaf_add_pair, leaf_insert, surface_cross_refs}`, `tree_read::brief_chain`,
    `tree_id::Entry::{is_done, is_abandoned, is_brief, is_live_leaf}`, and
    `repo::resolve`'s argument form. Production reaches the `*_unlocked`
    variants through `llm_cli`; these acquire the tree lock and are what the
    suite drives.
- `tree_lifecycle::transition_to_current` looks like the same shape and is not:
  it is the test-visible seam for the live lifecycle transition, whose driver
  twin is `pub(crate)`. Keep it.
- `leaf_id`'s unused parser surface is already an argued decision recorded in
  that module's header — a frozen grammar kept whole. Do not re-litigate it
  without engaging that reasoning.
- `task_relationship` and the three `tree_read` receipt helpers are
  `legacy-review-removal-k47`'s; leave them alone.

## Done when

- Every item above is either deleted or carries a recorded reason for surviving,
  with the test-only wrappers answered as one group rather than case by case.
- No coverage is lost: a wrapper that is deleted has its tests moved onto the
  interface that replaces it.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes

The interesting question is the second group, and it is a design question rather
than a cleanup one: is a locked wrapper with only test callers a seam worth
keeping, or a duplicate interface the suite should stop preferring? Answer it
once and apply it, rather than deleting the five that look easiest.
