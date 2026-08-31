# jj-workspace-book-k25

## Goal

Draft the `jj-workspace` book to green final validation: a complete,
source-exact walkthrough of the crate's four roots and 698 lines under
`docs/walkthroughs/jj-workspace/`, uniform with the relocated `ordinal-fs-tree`
book.

## Context

- Inputs, both committed before this session and both binding: the human's
  structure brief from `jj-workspace-structure-k17`, and the preregistration from
  `pilot-preregistration-k24`. The preregistration's attribution rule tells you
  what this session owes the record about the draft stage; satisfy it as you go
  rather than reconstructing it afterwards.
- The corpus, exactly, from the root brief: `crates/jj-workspace/src/lib.rs`
  (343), `src/refusal.rs` (230), `src/jj.rs` (81), and `crates/jj-workspace/Cargo.toml`
  (44). `tests/` is evidence, not a root. Every byte of those four files belongs
  to a fragment graph.
- The contract is the shared specification `walkthrough-books-spec-k20` landed,
  and the per-book corpus format `validator-fragments-k22` implemented. The
  validator takes the book directory; there is no compiled-in ledger to edit.
- Method: `linkuistics:writing-code-walkthroughs`. Scoped proof exists so a
  partial book is provable — use `--through` per slice as you go rather than
  discovering at the end that the graph does not close.

## Done when

- `docs/walkthroughs/jj-workspace/` holds the book, and final validation over it
  passes with no deferred holes.
- Its links into `docs/USAGE.md` resolve, per the shared specification's
  guide-link contract, and `every_repository_markdown_reference_resolves` is what
  proves it.
- `bash scripts/check.sh` passes, with the new book gated by the discovery
  `validator-fragments-k22` built rather than by a hand-added line.
- The draft stage's record exists in the form the preregistration's attribution
  rule requires.

## Notes

**This is the draft stage only.** Developmental edit, technical edit, copy edit,
art and proof are the `pilot-measure-k26` node's — one leaf and one commit each. A draft that has been
quietly polished destroys the attribution rule's ability to credit anything,
because there is then no unedited baseline for the later stages to be measured
against.

**The corpus is frozen.** Do not edit `crates/jj-workspace/`. A defect found here
becomes its own leaf under the root brief's cross-book rule.

**If this proves bigger than one session, decompose it** — one child per slice of
the book's own sequence, doing only the first. That is cheaper than a long
session, and the scoped validator was built to make a prefix provable.
