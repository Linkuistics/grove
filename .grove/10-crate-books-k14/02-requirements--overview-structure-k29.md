# overview-structure-k29

## Goal

Elicit, from the human, the structure brief for the system overview: what the
overview is *for*, its conceptual order, how much of `docs/ARCHITECTURE.md`'s
descriptive account it absorbs and in what shape, and what a reader should be
able to do at the end.

## Context

- Decision 15 of `plan-k1`: every book takes a human-authored structure brief;
  the source does not carry audience, conceptual order or emphasis. Do not invent
  one.
- The overview is the odd deliverable of the set. Its source corpus is the
  smallest — `crates/grove`, 3 roots and 204 lines — but it also absorbs the
  descriptive half of a 1,700-line architecture document (decision 5 of
  `plan-k1`): runtime flow, command surfaces and module seams. The proportion is
  what the human has to settle: a source-exact walkthrough of 204 lines with a
  large descriptive account around it, or a descriptive system account with the
  binary's own source as one chapter.
- What is already settled and must not be re-elicited: the audience (decision 7),
  the depth (decision 1 — complete source-exact over the corpus), the corpus (the
  root brief's table), and the method.
- The other books will link into this one, and the user guide is the reader's
  entry point ahead of it. Where the overview sits between the guide and the
  per-crate books is a structure question, not an authoring one.

## Done when

- A committed structure brief states, in the human's own words: the overview's
  purpose and reader outcome, its chapter sequence, which architecture material
  it takes and how it is reshaped, and what it deliberately leaves in
  `docs/ARCHITECTURE.md`.
- It is specific enough that `architecture-move-k31` can decide clause by clause
  what moves without a second interview.
- `bash scripts/check.sh` passes.

## Notes

**HITL: the loop stalls here by design.** If the human is not available, stop and
say so.

**The ownership table is the constraint on the answer.** `docs/ARCHITECTURE.md`'s
*Documentation ownership* section fixes one canonical source per subject and
bounds what may sit directly under `docs/`. An overview that duplicates a subject
another row owns is not a structure choice the human can make; say so if the
conversation heads there.
