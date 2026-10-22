# interpretation-prose-k21

## Goal

Remove the description of a surviving group from chapters 03 and 04 of the
`overview` book. Then cut the review of `lifecycle-job-k9`. This is the last
leaf of that node.

## Context

- The brief of `lifecycle-job-k9`. Its *Context* names the two passages.
- `docs/walkthroughs/overview/03-three-steps.md:814`: "a surviving Ravel group
  ... stops".
- `docs/walkthroughs/overview/04-proving-a-negative.md:263`: the surviving-group
  combinations as a runner-result input.
- The running log of `driver-job-k20`. It says what the interpretation now is
  and which fragments changed.
- Measured before the cut: chapter 03 has 43 hard findings of the checker, and
  chapter 04 has 9. Convert only the sections that this leaf edits.

## Done when

- Chapters 03 and 04 of `overview` no longer describe a surviving group as a stop
  condition or as a result input. The conversions of the touched sections land
  first, each as its own change.
- The source index, the concept index and any moved line counts of `overview`
  follow.
- A search for the survivor wording over the whole of `docs/walkthroughs/` finds
  nothing that describes the removed interpretation. The search has a positive
  control and a cross-tree control. Each control fails once before you credit it.
- `bash scripts/check.sh` passes.
- A `review-impl` leaf exists for `lifecycle-job-k9`. It sits before
  `interrupted-errors-k10`, because that leaf builds on this one.
- The body of the review names the specific doubt. The doubt covers the terminal
  and signal contracts of the lifecycle. These are the foreground handover, the
  latch, the post-reap order and the errors that stop the loop.

## Notes

- Cut the review as the last act, with `grove-llm leaf-insert` at the leaf
  `interrupted-errors-k10`. The review carries the bare stem `lifecycle-job`.
- The reviewer finds the producer commits from the handle `lifecycle-job-k9`.
  Its body names the handles of the three children too.

## Decisions (running log)

