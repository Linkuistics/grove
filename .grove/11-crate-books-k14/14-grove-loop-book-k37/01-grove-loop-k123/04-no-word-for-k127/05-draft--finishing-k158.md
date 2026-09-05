# finishing-k158

## Goal

Draft chapter 14 of the `grove-loop` book, *Finishing*, over
`tree_lifecycle.rs`'s two `the-tree-deletes-itself` blocks — `1-331` and
`1467-1665`, 530 lines — take the Part III prefix to 7,416 resolved lines, and
close `no-word-for-k127`.

## Context

- Child 4 of 4 of `no-word-for-k127`, and the last. The rule is **the driver is
  the only author of the leaf that ends the grove, and the ending deletes the
  tree**.
- Blocks: `finish-transition` (`1-331`: the module header, `CurrentTransition`,
  `transition_to_current`, `materialize_finish`, the four finish-leaf helpers
  `new_finish_leaf` / `finish_handle` / `finish_slug` / `finish_body`,
  `finish_commit`, `delete_and_commit`, `require_recoverable_grove`) and
  `finish-tests` (`1467-1665`).
- **The file opens on finishing and the book closes on it.** This chapter owns
  the root's first 331 lines, which is why it is last rather than first, and the
  block boundary at `1466/1467` is exact: `1465` closes chapter 11's last
  root-init test, `1466` is the blank line after it, and `1467` opens the doc
  comment of `materialize_finish_writes_a_handle_that_matches_its_own_filename`.
- **Chapter 11 owns items this block is the main consumer of.**
  `default_root_slug` and `root_shape` each have exactly one caller anywhere and
  both are here — `transition_to_current` at lines `82` and `87`. `grove_name` has
  two callers, of which line `81` is one and chapter 11's `root_init` is the other.
  Chapter 11 reproduces and explains all three and points forward; the account of
  *what the driver's own scaffold is for* is this chapter's.
- **One of `root_shape`'s three arms is pinned only from this block, and one is
  pinned from both**, established by mutation in `a-grove-begins-k155`. Panicking
  on `RootShape::Unrecognised` reddens `transition_refuses_a_root_holding_no_grove_entry_at_all`
  (`1621`) and nothing else, so that arm is **this chapter's alone**. Panicking on
  `RootShape::ATree` reddens `transition_leaves_a_current_grove_unchanged_and_ready_for_pick`
  (`1564`) **and** `one_process_creating_and_reading_a_grove_never_waits_on_itself`
  (`1385`), which is inside chapter 11's block — so `ATree` is held from both sides.
  `Taskless` is held only from chapter 11. Chapter 11 carries the table and owes
  no more; what this chapter owes is the account of `Unrecognised`.
- **The stale `llm_cli` address is in this block, at line `42`**, and its twin at
  `1147` is chapter 11's. There is no `llm_cli` in this workspace; the code is
  `crates/grove-llm/src/cli.rs`. **The behaviour each comment claims is correct
  and only the address is stale**, which is why chapter 6 adjudicated on the page
  rather than cutting a leaf, and chapter 11 did the same. Owe the same
  one-clause adjudication.
- `finish_commit`, `delete_and_commit` and `require_recoverable_grove` are private
  helpers with many `bail!` arms; enumerate them and settle coverage by mutation
  against an unmutated control, not by reading. The harness is described in this
  node's brief.
- **The chapter's observable end is the grove ceasing to exist**, which is why
  the book is not illustrated from a live `.grove/`.
- The manifest's early-use rows are a floor; enumerate this chapter's own bytes.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  the-tree-deletes-itself --check all` is valid: 13 files, 7,416 resolved lines,
  3,117 deferred, `final=false`.
- `14-finishing.md` exists, `README.md` and chapter 13's navigation are updated,
  and all nine `tree_lifecycle.rs` ownership rows read `resolved`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.
- `no-word-for-k127` has no live leaf left and is closed.

## Notes

**The corpus is frozen.** A defect found while drafting becomes its own leaf.

**This is not the last child of `grove-loop-k123`.** `the-surface-k128`,
`what-a-runner-cannot-k129` and `what-could-not-move-k130` follow, so the
`copy-edit` cut the document node's brief names is **not** this leaf's last act.

## Decisions (running log)
