# developmental-edit-k40

## Goal

Run the **developmental edit** over the drafted `jj-workspace` book: structure,
conceptual order, what a chapter is for, and whether the book delivers the reader
outcome its structure brief asked for. One commit.

## Context

- The inputs are the draft from `jj-workspace-book-k25` and the human's structure
  brief from `jj-workspace-structure-k17`. The structure brief is the standard
  this stage judges against — it is not this session's to revise, and a
  disagreement with it is a finding, not a licence.
- This is the first stage after the draft, so its baseline is the draft commit
  itself, and it is the stage with the largest licence to move prose. Moving prose
  in a source-exact book moves fragments; the graph must still close.
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

- The book's structure answers the structure brief, or the report will say where
  it does not.
- Final validation over `docs/walkthroughs/jj-workspace/` is green — this stage
  leaves the book provable, not merely improved.
- The stage's record exists in the form the preregistration's attribution rule
  requires.
- `bash scripts/check.sh` passes.

## Notes

**Structure, not sentences.** Copy edit is `copy-edit-k42`'s and technical
accuracy is `technical-edit-k41`'s. A developmental pass that also fixes commas
makes its own diff unreadable as evidence, and the attribution rule then credits
one stage for another's work.