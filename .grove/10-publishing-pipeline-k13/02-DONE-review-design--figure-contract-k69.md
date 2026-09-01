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

## Findings

### F1 · High — The ADR's load-bearing inventory cost is false of the current design

The ADR says the current closed-directory rule makes `M101` a complete inventory
check rather than a check against a list, and that declaring assets would make
inventory only as complete as the author's declarations
(`docs/adr/a-book-carries-no-asset.md:55-69`). The implementation already builds
the expected inventory from the manifest's authored `[[page]]` list plus
`walkthrough.toml`, then compares that set with every entry found on disk in both
directions (`crates/book-validation/src/markdown.rs:29-48`). The specification
states the same contract: exactly the manifest and its declared pages
(`docs/specs/walkthrough-books.md:1245-1249`).

Adding declared assets to that expected set would preserve the closed-world
property: an undeclared file would remain extra, a declared-but-missing asset
would remain missing, and the recursive inventory would still admit nothing
else. The ADR's other costs are real — a schema group, snapshot input, link
target and diagnostics — but loss of inventory completeness is not one of them.
It also conflates the externally witnessed *source corpus* with presentation
files inside the book. Because the producer calls this the cost that decides the
trade-off, rework the rationale around the actual additional interface and
maintenance cost (or around waiting for observed demand); the no-asset conclusion
cannot rest on a property asset declarations would preserve.

### F2 · High — The art pass could not turn a missing ceiling-bound finding into positive evidence

The art task was explicitly bounded to what plain Markdown can carry
(`.grove/09-pilot-k12/06-pilot-measure-k26/04-DONE-impl--art-k43.md:3-6`), and
the frozen `A1` class exists only where a Markdown table, list figure or diagram
*would* carry the relation (`docs/evaluations/editorial-pipeline-pilot/preregistration.md:203-220`).
The stage then says its two reads were precisely an `A1` pass and an `A2` pass
(`docs/evaluations/editorial-pipeline-pilot/stages/4-art.md:63-78`). A relation
Markdown cannot carry is therefore outside the task's production boundary and
outside both active survey classes.

`## Findings not fixed` is only an outlet for defects the stage happened to see;
the preregistration creates no search obligation beyond the taxonomy
(`docs/evaluations/editorial-pipeline-pilot/preregistration.md:490-493,813-820`).
The recorded source-index case is an `A2` found by the scheduled `A2` pass and
blocked only at the fix boundary
(`docs/evaluations/editorial-pipeline-pilot/stages/4-art.md:153-162`). It proves
that an in-taxonomy forbidden fix can be recorded, not that ceiling-bound demand
was capable of being found. The ADR consequently overstates “not one declination
was ceiling-bound” as the observation that turns absence into evidence
(`docs/adr/a-book-carries-no-asset.md:32-48`). Recast this as the weaker result
the pilot supports — no ceiling-bound demand was observed by a Markdown-bounded,
single-book pass — and let reversibility plus a future explicit ceiling finding,
not a claimed positive control, carry the decision to wait.

### F3 · Medium — The four-table carve-out leaves the required fifth lookup table nonconforming

The carve-out is exact for the four `F009` tables: the specification names
`Source roots`, `Ownership blocks`, `Fragment index` and `Early uses`, and the
validator requests exactly those four headings
(`docs/specs/walkthrough-books.md:693-725`;
`crates/book-validation/src/ledger.rs:33-37`). But the same specification also
requires `source-index.md` to carry an owned-source totals table
(`docs/specs/walkthrough-books.md:767-771`). The existing `jj-workspace` book has
that fifth table immediately after its heading, with no sentence stating its
role (`docs/walkthroughs/jj-workspace/source-index.md:169-181`).

The new rule makes every table a figure that needs an adjacent role statement,
while exempting only the four machine-reconciled tables
(`docs/specs/walkthrough-books.md:1332-1365`). It therefore creates an editorial
finding in the pilot book at the moment it is adopted. Decide the fifth table's
class explicitly: either exempt it too and explain why an editorial, currently
unreconciled totals table earns the same treatment, or give it a role statement
(which `F009` does not prevent) and make that placement part of the contract.

### F4 · Medium — The durable convention narrows the measured `A2` class

The frozen `A2` test reaches every table, diagram and non-fragment code block
(`docs/evaluations/editorial-pipeline-pilot/preregistration.md:219-220`), and the
art stage operationalised it by inspecting every table, every non-fragment fenced
block and every diagram — including 45 fenced blocks
(`docs/evaluations/editorial-pipeline-pilot/stages/4-art.md:63-78`). The new spec
instead defines a figure as a table, a diagram, or another non-fragment fence only
when it carries a relation, then applies the role-statement rule only to figures
(`docs/specs/walkthrough-books.md:1332-1355`). A console or text fence that does
not encode a relation has silently fallen out of the obligation the kept art
stage actually measured.

That contraction contradicts the ADR's reason for writing the convention down:
to move the pilot's classes from a frozen instrument into the live contract.
Make the durable rule cover all non-fragment fenced blocks as `A2` did, or record
and justify an explicit change of class before `pipeline-skills-k28` turns the
narrower wording into the art kind's discipline.
