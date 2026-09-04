# no-word-for-k127

## Goal

Draft Part III of the `grove-loop` book — chapters 11 to 14, the whole of
`crates/grove-loop/src/tree_lifecycle.rs` (2,725 lines, nine blocks) — and prove
the prefix through slice `the-tree-deletes-itself`.

## Context

- Draft stage, child 4 of 7 of `grove-loop-k123`. The structure brief is
  `docs/specs/grove-loop-book-structure.md`; the four chapters are its sections
  *11 · A grove begins* through *14 · Finishing*, and the mapping is its
  *Top-level ownership blocks* and *`tree_lifecycle.rs` four ways in nine
  blocks*.
- **Expect this to decompose, one child per chapter.** Four chapters at 612, 775,
  808 and 530 lines, over the largest root in the corpus.
- The blocks, in file order and with their owning chapter: `1-331` (14),
  `332-489` (11), `490-695` (12), `696-1012` (13), `1013-1076` (11),
  `1077-1466` (11), `1467-1665` (14), `1666-2234` (12), `2235-2725` (13). **The
  file opens on finishing and the book closes on it**: chapter 14 owns `1-331`
  and chapter 11 owns `1013-1076`. `1467-1665` — the `materialize_finish` and
  `transition_to_current` tests — is chapter 14's, not chapter 11's.
- **The prose obligation is *supply the claim*, at the largest scale in the
  book.** 1,649 of these lines are the inline test module at 15% prose. Every
  reproduced test states the property it establishes **and what would have to be
  true for it to pass while the property was broken**.
- Chapter 11 owns the shared test-support block the next two chapters' tests use,
  and the three body-writing helpers shared with chapter 12.
- The brief names the pairs that carry each chapter's rule:
  `root_init_creates_the_whole_grove_through_one_store_operation` with
  `a_refused_grove_leaves_no_root_behind` (11);
  `decompose_converts_leaf_file_to_node_dir_preserving_the_key` and the four
  refusals (12); `retire_adds_done_infix_keeping_position_and_key` with
  `retire_does_not_rewrite_the_header_or_body`, and
  `pruning_a_node_takes_one_guard_per_mark` as the cost the atomicity is *not*
  paid with (13); `materialize_finish_writes_a_handle_that_matches_its_own_filename`
  and `a_tree_at_the_last_key_refuses_the_sentinel_rather_than_wrapping` (14).
- **Chapter 14's observable end is the grove ceasing to exist**, which is why the
  book is not illustrated from a live `.grove/`: `finish-commit` removes it.
- Chapter 13's thesis is *the tree's shape is the only state* — the spine the
  whole brief rejected as the book's, kept as this chapter's and contrasted with
  chapter 16's in a sentence chapter 1 has already written.
- Chapter 1's cast rows owned by `never-mistaken-for-finished` move to
  `explained` when chapter 11 lands.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  the-tree-deletes-itself --check all` is valid: 13 files, 7,416 resolved lines,
  3,117 deferred, `final=false`.
- Chapters 11–14 exist, contents and navigation are updated, and the nine
  `tree_lifecycle.rs` ownership rows read `resolved`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

**The corpus is frozen.** A defect found while drafting becomes its own leaf and
is not fixed inline.

**The two glossaries collide here.** `ordinal-fs-tree`'s *leaf* and *node* are
not grove's, and its *ordinal* is grove's *position* (`CONTEXT-MAP.md`). These
four chapters speak of both trees in nearly every paragraph; say which tree is
meant, sentence by sentence.

## Decisions (running log)
