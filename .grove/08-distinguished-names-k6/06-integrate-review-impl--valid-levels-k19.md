# valid-levels-k19

**Integrates:** valid-levels-k18

## Goal

Triage the findings of the `review-impl` leaf `valid-levels-k18` against the
enforcement boundary delivered at `valid-levels-k16`, apply the ones that are
real, and leave the library's stated contract, its CLI contract and both books
current before `node-files-k7` builds Grove's required node-file grammar on
them.

## Context

- The findings are in the review's own file, found by its handle; read them
  from there rather than from this body. They are anchored to commit
  `yrquyqyp` (`bbd60935`) and to `path:line` coordinates in that tree. This
  leaf was appended after the review, so no sibling position moved.
- The producer's contract is `04-DONE-impl--valid-levels-k16.md` and the parent
  brief in this directory; the previous integration at `supplied-names-k17`
  settled the refusal mapping and the empty-sample limit, and nothing here
  reopens them.
- The artifact is the trait documentation, the syllabus CLI's exit-code
  contract, the conformance kit's coverage statement, and the `ordinal-fs-tree`
  and `grove-loop` books under `docs/walkthroughs/`. The review found no defect
  in the reader, planner or projection; enforcement code is expected to stay as
  it is unless a triaged finding says otherwise.

## Done when

- Every finding is triaged as applied, rejected with the reason, or a visible
  accepted trade-off, in this file's running log.
- Where a finding names a kit-coverage or diagnostic decision the grammar
  consumer inherits, the decision is settled and recorded here so
  `node-files-k7` builds on it rather than reopening it.
- Each touched source root lands with its fragments, ledgers, indexes and prose
  in this commit; the book-validation inventory follows any line-count change;
  `bash scripts/check.sh` passes on unchanged tracked inputs.
- `node-files-k7` still follows this node; positions and keys are otherwise
  untouched. The installed driver, plugin and live-tree grammar remain
  untouched.

## Notes

- Substantial redesign is not this leaf's: externalise it as a new producer
  review chain beside the leaf being integrated.
- One narrow in-session reviewer is available for a non-mechanical fix; a
  second need is the signal to cut a `review-impl` leaf instead.
- If every real finding is documentation, the parent's *Done when* is met on
  this leaf's retirement and the node closes in the same commit; check the
  brief's criteria against what the three children delivered before closing.
