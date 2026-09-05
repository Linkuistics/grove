# the-walk-k143

## Goal

Draft chapter 7 of the `grove-loop` book — *The walk: pick and select*,
`docs/walkthroughs/grove-loop/07-the-walk.md` — and prove the prefix through
slice `first-live-leaf`.

## Context

- The third of `the-walk-k126`'s six chapter children. **Name this leaf by its handle,
  never by its bare slug**: `the-walk` is both this leaf and its parent node
  `the-walk-k126`, so a bare-slug reference is ambiguous and `resolve` refuses it
  listing both keys.
- Its two ownership blocks are `walk-selection` (`task_tree.rs` 571–637) and
  `pick-tests` (1,106–1,360) — 322 lines, of which 255 are tests. **This is the
  most test-heavy chapter of the six.**
- The structure brief's section is *7 · The walk: pick and select*. The rule is
  **the first live leaf in walk order, and position in that walk is the only
  schedule there is**: `Selection`, `pick_in`, `select_in`, `select_in_write`,
  `selected`.
- **Fifteen tests, and the brief names the specific form the claim takes.** The
  prose owes the negative case for each, and the brief singles one out:
  `pick_orders_numerically_not_lexically` passes under a lexical sort too until
  there are ten leaves, so it is `10` against `9` that makes it a test. Count the
  tests in the block before writing *fifteen* — a count claim is the highest-yield
  defect class in this book and the brief's number is the brief's, not the
  block's.
- `pick_refuses_a_species_mismatch_at_a_task_shaped_name` is the spine's sharpest
  case: the store would have accepted the entry, and the grammar that refuses it —
  defined in chapters 2 to 4 — is the thing that did not move.
- **Chapter 1's cast row `Selection`, owned by `first-live-leaf`, closes here**
  and moves from `pending` to `explained`. So does the row for `pick` added at
  `02-the-tokens.md#the-outcome`, whose owner is also `first-live-leaf`.
- **The early-use ledger is a floor**: enumerate this chapter's reproduced bytes
  for later-owned symbols named or exercised, and add the rows they owe.
- The chapter's carried-example row is the brief's row 7: from the tree as
  `root_init` left it, to the first live leaf in walk order.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  first-live-leaf --check all` is valid: 13 files, 3,132 resolved lines, 7,401
  deferred, `final=false`.
- Chapter 7 exists, `README.md`'s contents entry and chapter 6's two navigation
  lines are updated, and both `first-live-leaf` ownership rows read `resolved`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

**The corpus is frozen.** A defect found while drafting becomes its own leaf and
is not fixed inline.

## Decisions (running log)
