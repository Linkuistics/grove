# copy-edit-k42

## Goal

Run the **copy edit** over the `jj-workspace` book: sentences, terminology,
consistency with the shared specification's prose contract, and the grove
vocabulary rule. One commit.

## Context

- The prose contract, the audience and the vocabulary rule are the shared
  specification's (`walkthrough-books-spec-k20`) and decision 7 of `plan-k1`: a
  reader who knows Rust and jj and has driven a grove, with grove vocabulary
  linked to `CONTEXT.md` and never re-taught.
- The crate's whole vocabulary is Jujutsu's. Terminology consistency here means
  jj's terms used as jj uses them, and the book's own additions kept visibly
  separate.
- The preregistration `pilot-preregistration-k24` committed is binding and not
  reopenable here. Read its attribution rule before you start and satisfy it as
  you go; reconstructing afterwards what a stage changed is what the one-commit
  boundary exists to make unnecessary.
- The baseline this stage is measured against is the previous stage's commit, and
  the record of what this stage did is the diff between that commit and yours. So
  this stage lands in **exactly one commit**, carrying its edits and nothing else.
- The corpus is frozen: do not edit `crates/jj-workspace/`. A defect found here
  becomes its own leaf under the root brief's cross-book rule.

## Done when

- The book is consistent in terminology and voice with the relocated
  `ordinal-fs-tree` book, which is the uniformity precedent.
- Final validation over `docs/walkthroughs/jj-workspace/` is green — this stage
  leaves the book provable, not merely improved.
- The stage's record exists in the form the preregistration's attribution rule
  requires.
- `bash scripts/check.sh` passes.

## Notes

**Do not restructure.** A copy edit that reorders chapters is a developmental
edit arriving late, and the report cannot then say which of the two paid.