# growing-k146

## Goal

Draft chapter 10 of the `grove-loop` book — *Growing: `leaf-add` and
`leaf-insert`*, `docs/walkthroughs/grove-loop/10-growing.md` — and prove the
prefix through slice `what-the-library-cannot-see`, which completes Part II.

## Context

- The last of `the-walk-k126`'s six chapter children. Its **one** ownership block is
  `growing-the-tree` (`task_grow.rs` 1–518), the whole file, 518 lines.
- The structure brief's section is *10 · Growing: `leaf-add` and `leaf-insert`*.
  The rule is **the preconditions the library cannot see, checked against the
  same snapshot the operation then plans from**. The brief calls this the spine
  at its most explicit, because the module header is a list of exactly it — the
  reference grammar, the preconditions, the task-file template, and the
  cross-reference lint, each with its own paragraph saying why it could not move.
- **The chapter carries key prediction.** Because the template's bytes embed the
  key, grove predicts the allocation and checks it against the store's report.
  That is the carried example's row 10 and its observable end: a sibling at the
  next position, its key predicted and checked.
- **This chapter's proof is entirely outside its own pages, and it is the only
  chapter of the book of which that is true — and the chapter says so.**
  `src/task_grow/tests.rs` (1,680 lines) is the book's one declared
  `[[corpus.exclude]]`, so its tests are cited by name and never reproduced. The
  four the brief names are
  `add_refuses_an_ambiguous_parent_slug_and_lists_the_keys`,
  `add_preserves_a_gap_a_hand_edit_left_rather_than_filling_it`,
  `a_refused_run_does_not_consume_positions_or_keys` and
  `insert_at_occupied_position_shifts_occupant_and_later_siblings_keys_preserved`.
  Verify each name against the file before citing it; a cited test that has been
  renamed is a claim about the subject that the subject does not bear out.
- **The block is wholly production at 49% prose and takes *do not restate*.**
  There is no inline test to supply a claim for — the two-line `#[cfg(test)] mod`
  declaration names the external file instead.
- **The early-use ledger is a floor**: enumerate this chapter's reproduced bytes
  for later-owned symbols named or exercised, and add the rows they owe.
- `grove-does-not-stage-its-own-renames` and `entries-are-never-removed` are
  among the records the structure brief names for the chapters that keep them.
  Under the link contract they are **named in prose and never cited**.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  what-the-library-cannot-see --check all` is valid: 13 files, 4,691 resolved
  lines, 5,842 deferred, `final=false`. That is `the-walk-k126`'s own
  `Done when` and this leaf discharges it.
- Chapter 10 exists, `README.md`'s contents entry and chapter 9's two navigation
  lines are updated, and the `growing-the-tree` ownership row reads `resolved`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

**The corpus is frozen.** A defect found while drafting becomes its own leaf and
is not fixed inline.

**This leaf's last act is not a `leaf-add`.** The pipeline's next stage is cut by
`grove-loop-k123`'s last child, `what-could-not-move-k130`, under the node
brief's own `Done when`; this leaf closes a part, not the document.

## Decisions (running log)
