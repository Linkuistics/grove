# outcomes-k157

## Goal

Draft chapter 13 of the `grove-loop` book, *Outcomes are marked in place*, over
`tree_lifecycle.rs`'s two `marked-in-place` blocks — `696-1012` and `2235-2725`,
808 lines — and prove the prefix through slice `marked-in-place`.

## Context

- Child 3 of 4 of `no-word-for-k127`. The rule is **done-ness and abandonment are
  marked in the name, and the body is not rewritten**.
- Blocks: `outcomes-in-place` (`696-1012`: `leaf_retire`, `retire_parts`,
  `PruneResult`, `leaf_prune`, `Planned`, `plan_prune`, `plan_subtree`,
  `plan_leaf`, `apply_prune`, `stopped_partway`, `marked_path`) and
  `retire-and-prune-tests` (`2235-2725`), whose four labelled sections are
  *leaf-retire* at `2235`, *lifecycle over untracked leaves (issue #3's root
  cause)* at `2342`, *leaf-prune (pruning)* at `2395` and *leaf-prune on a node:
  bulk arity (pruning)* at `2496`.
- **This chapter's thesis is *the tree's shape is the only state*** — the spine
  the whole brief rejected as the book's, kept as this chapter's and contrasted
  with chapter 16's in a sentence chapter 1 has already written. Read chapter 1
  before writing the opening; do not re-argue it.
- The pairs the brief names: `retire_adds_done_infix_keeping_position_and_key`
  with `retire_does_not_rewrite_the_header_or_body` are what make *in place* mean
  something, and `pruning_a_node_takes_one_guard_per_mark` is the cost the
  atomicity is **not** paid with. That last is a claim about a measurement — a
  guard count — so it is worth exactly the re-run, not the reading.
- **`plan_prune`, `plan_subtree`, `plan_leaf` and `apply_prune` are private
  helpers with many `bail!` arms**, which is the shape that has produced an
  attribution defect in every chapter that has met it so far (`leaf_entry`: five
  of seven arms unobserved; chapter 9's arm 3: four observers where the page
  carried two). Enumerate the arms, mutate each to a **panic** in a workspace
  copy, and diff against an unmutated control run of the same copy; the harness
  `a-grove-begins-k155` used is described in this node's brief. A reworded `bail!`
  is not a mutation.
- `stopped_partway` renders an operator-facing message about partial work; check
  what actually observes it before writing that it is checked.
- The manifest's early-use rows are a floor; enumerate this chapter's own bytes,
  in both the Rust-path and the hyphenated-verb spelling.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  marked-in-place --check all` is valid: 13 files, 6,886 resolved lines, 3,647
  deferred, `final=false`.
- `13-outcomes.md` exists, `README.md` and chapter 12's navigation are updated,
  and the two `marked-in-place` ownership rows read `resolved`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

**The corpus is frozen.** A defect found while drafting becomes its own leaf.

## Decisions (running log)
