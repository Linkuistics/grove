# figure-contract-k69

**Reviews:** figure-contract-k18

## Goal

Read `a-book-carries-no-asset` and the two edits it made to
`docs/specs/walkthrough-books.md` adversarially, and report findings. The
producer concluded that no figure format, no asset convention and no validator
support is earned, and landed a figure *convention* in the specification's prose
contract instead. Both halves are in scope, and so is the possibility that the
producer's reading of its own evidence is self-serving.

## Context

- The producer's commit names `figure-contract-k18`. The artifacts are
  `docs/adr/a-book-carries-no-asset.md`, a new `### Figures` subsection in
  `docs/specs/walkthrough-books.md`'s prose contract, and a rewritten *Assets*
  paragraph in that specification's *Out of scope*.
- The evidence is `docs/evaluations/editorial-pipeline-pilot/README.md` and its
  `stages/4-art.md`, read against `preregistration.md`. Those three are frozen
  and were deliberately not edited; check that decision as well as the ones built
  on it.
- The producer spent no in-session reviewer: the harness this session ran under
  forbade subagents. So this leaf is the **first** independent read of the
  decision, not the second.

## Done when

Findings are reported and nothing is fixed (`review-design`'s discipline). The
producer's own statement of where it is weakest is the place to start, and the
four below are named because the producer named them, not because they exhaust
the read:

- **The inference from `N = 1`.** The measured book is `jj-workspace` — four
  roots, 698 lines, a thin subprocess seam. `grove-loop` is thirteen roots and
  10,533 lines with a lifecycle state machine. The ADR argues the extrapolation
  is narrower than it looks because a transition table and a box-drawing state
  diagram are both inside the Markdown ceiling. Test that argument rather than
  the conclusion.
- **Whether the absence of assets was ever capable of being evidence.** The
  producer's move is that `4-art.md`'s `## Findings not fixed` demonstrably
  records blocked cases — it recorded one forbidden by the book contract — so a
  ceiling-bound declination would have appeared there had one existed. Is that a
  sound positive control, or does the stage's charter make a ceiling-bound
  finding unthinkable rather than merely unrecorded?
- **A "build nothing" conclusion reached by the session that would otherwise have
  had to build it.** The node brief flags this leaf as the one that can
  legitimately end that way, which is also what makes the incentive worth
  checking. Is any part of the argument doing work it has not earned?
- **The convention's placement and its carve-outs.** `### Figures` is
  author-and-reviewer applied, cites `M105` and `F009` for placement rather than
  restating them, and exempts the four `source-index.md` tables. Check the
  exemption is exactly right, that the rule contradicts no mechanical check, and
  that the prose contract — rather than the art kind's own skill — is the right
  owner under `corpus-rules-have-one-owner` and `SPEC-FORMAT`'s membership test.

## Notes

**`pipeline-kinds-k27` is directly downstream and is the reason this sits ahead
of it.** The art kind's discipline is written against this contract, so a finding
that lands here is cheap and the same finding found after two books are written
is not.

**A review that finds nothing creates nothing.** Cut
`integrate-review-design` only if there are findings worth acting on, and place
it by the directory-local rule — the first sibling entry after this one whose
subtree still holds live work is `pipeline-kinds-k27`.
