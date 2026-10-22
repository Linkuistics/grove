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

- **Conversion.** Two changes converted the touched sections first. Chapter 03
  section *The lifecycle behind the call* went from 43 to 40 hard findings in the
  file, and the section has none. Chapter 04 section *Three different guarantees*
  has none, and the file went from 9 to 5. The other findings sit in sections that
  this leaf does not edit.
- **Prose.** Chapter 03 now reads the end in one order: interruption, force-stop,
  teardown, then exactly `grove-continue`, and any other end stops. It says that
  Grove takes a successful Ravel status as the only evidence of cleanup and probes
  no process group. Chapter 04 says that the Grove Loop cases build the end of the
  job directly, as a status with an optional interruption signal. Neither chapter
  contrasts the new reading with the removed one.
- **Extra edit.** The search found one more sentence in `grove-loop` chapter 19:
  "A surviving process group no longer stops the loop." `driver-job-k20` wrote it.
  It carried the removed rule forward as a contrast, so I removed it.
- **Indices.** The source index and the concept index of `overview` needed no
  change. The slices count quoted source lines, this leaf edited no fragment, and
  no anchor moved.
- **Search.** The pattern was
  `surviv[a-z]* (ravel[- ])?(process[- ])?group|group[^.]{0,60}surviv|survivor (check|report)|group state|runner-result cases`.
  The subject was all of `docs/walkthroughs/` without the `keyed-launch` book. It
  found nothing, on lines and on whitespace-normalised text. The positive control
  ran the same pattern over commit `6ac8656be5` before this leaf and found five
  hits in chapters 03, 04 and 19. The cross-tree control found 13 hits in the
  `keyed-launch` book. Both controls came back dirty on subjects I know are wrong
  before I credited the clean read. The `keyed-launch` book describes the removed
  interpretation by design until `keyed-launch-removal-k11` removes it.
- **Finding.** Chapter 20 line 365 lists "the watch and the escalation" as chapter
  19 content, and chapter 19 lost the section on the cancellation bound. The claim
  predates this leaf and is not survivor wording, so I left it for the review.
- **Review.** The leaf `lifecycle-job-k22` sits before `interrupted-errors-k10`.
  Its notes cover the foreground handover, the latch, the post-reap order and the
  errors that stop the loop. This session has no provider evidence, so the review
  has no Creator line.
- **Result.** `bash scripts/check.sh` passes all 20 checks, including `book-check`
  on all six books.
