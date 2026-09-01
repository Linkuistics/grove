# proof-k44

## Goal

Run the **proof** stage over the `jj-workspace` book: the final read for errors
the earlier stages introduced or missed, and the last check that the book is
whole. One commit.

## Context

- Proof is one of the two stages in the alternative the six must beat — draft plus
  proof — so its diff is load-bearing in both arms of the comparison, and it is
  the stage whose contribution the decision rule scrutinises hardest.
- Every earlier stage has already landed and been validated. Anything this stage
  finds that belongs to an earlier stage is reported as such rather than silently
  absorbed, or the attribution rule credits proof for another stage's miss.
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

- The book is complete with no deferred holes, and this is the state the report
  is written against.
- Final validation over `docs/walkthroughs/jj-workspace/` is green — this stage
  leaves the book provable, not merely improved.
- The stage's record exists in the form the preregistration's attribution rule
  requires.
- `bash scripts/check.sh` passes.

## Notes

**This is the last stage, not the report.** `measurement-report-k45` does the
reading; a proof session that also starts scoring has adjusted the instrument
mid-reading.