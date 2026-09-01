# art-k43

## Goal

Run the **art** stage over the `jj-workspace` book by hand: whatever figures,
diagrams or tables plain Markdown can carry, and nothing else. One commit.

## Context

- There is no figure format, no asset convention and no validator concept of an
  asset, and building them now is exactly the error the root brief's *Notes* name
  — machinery ordered ahead of the measurement that would justify it.
  `figure-contract-k18` decides afterwards, from this node's report.
- The preregistration's attribution rule was written to be able to credit a stage
  that produced Markdown figures with no machinery behind them. If it cannot, say
  so as a protocol breach; do not invent machinery so the stage becomes
  measurable.
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

- Whatever this stage produced is in the book and in one diff, including the case
  where it produced nothing — a stage that legitimately added nothing is an
  observation the decision rule can use, and it is recorded as one.
- Final validation over `docs/walkthroughs/jj-workspace/` is green — this stage
  leaves the book provable, not merely improved.
- The stage's record exists in the form the preregistration's attribution rule
  requires.
- `bash scripts/check.sh` passes.

## Notes

**Producing nothing is a permissible outcome and must be recorded, not skipped.**
A stage that is quietly not run is missing data; a stage that ran and found
nothing worth drawing is evidence. The report cannot tell those apart unless this
session says which happened.