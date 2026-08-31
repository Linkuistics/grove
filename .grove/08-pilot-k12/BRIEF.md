# pilot-k12 — brief

## Goal

Author the `jj-workspace` book by running the editorial stages **by hand**, and
produce the measure that says which of those stages paid for itself. The book is
the visible deliverable; the measure is the point, because nothing in this
campaign currently distinguishes six editorial stages from two.

## Done when

- `docs/walkthroughs/jj-workspace/` holds a complete book over the crate's four
  roots and 698 lines, passing final validation with no deferred holes.
- A preregistration was committed **before** the book was drafted, carrying four
  things: the judged outcome, the alternative the six stages must beat, the
  attribution rule saying how a change is credited to a stage, and the decision
  rule mapping evidence to keep / merge / drop per stage.
- A measurement report exists, evaluated against that preregistration, saying per
  stage whether it paid — including any stage the evidence says to drop.
- A human-authored structure brief for the book exists as a committed input
  artifact, written before the drafting began.

## Decomposition

Four leaves, and the first two must both precede the third.

1. `jj-workspace-structure-k17` — the human's structure brief for this book.
2. `pilot-preregistration-k24` — the measure, written and committed before any
   drafting.
3. `jj-workspace-book-k25` — the draft, taken to green final validation.
4. `pilot-measure-k26` — the remaining stages run one commit each, and the report.

Drafting and editing are separate leaves because a session that must both write a
source-exact book and run five editorial passes over it will run long and produce
a poor measurement. A validated draft is independently useful on its own: it is a
real book, and the campaign's first new one.

## Pointers

- The crate: `crates/jj-workspace` — `src/lib.rs` (343), `src/refusal.rs` (230),
  `src/jj.rs` (81) and `Cargo.toml` (44). Four roots, 698 lines, per the root
  brief's frozen corpus table.
- Why this crate is the pilot (decision 12 of `plan-k1`): it is the smallest new
  corpus and it has a real external boundary — Jujutsu's vocabulary, which
  `CONTEXT-MAP.md` records as deliberately *not* a bounded context of its own.
  That boundary is the crate's most interesting property and the book has to
  carry it.
- Its decisions live in the grove context: decision 8 of
  `docs/specs/module-decomposition.md` for the interface, and
  `docs/adr/jj-is-the-only-lane.md` for the refusal.
- Method: `linkuistics:writing-code-walkthroughs`. Its eight-field intake is
  already answered — decisions 1, 3, 6, 7, 9, 10 and 17 of `plan-k1`, with the
  source manifest in the root brief's *Pointers*. Do not re-elicit it.
- Precedent: the relocated `ordinal-fs-tree` book is the worked example every new
  book is uniform with, and the shared spec `walkthrough-books-spec-k20` wrote is
  the contract.
- In-house prior art for the measurement, all on disk:
  `/Users/antony/Development/grove.gh-issue-12` and this repository's
  `docs/evaluations/writing-code-walkthroughs/` (a preregistered evaluation with
  a frozen rubric, arms, controls, a primary endpoint set and an adjudicated
  verdict); `/Users/antony/Development/Writegood` (a judging protocol, a
  measurement corpus, and the argument that measurement must precede machinery);
  `/Users/antony/Development/TheGreatExplainer` `docs/requirements.md` §1.6 (the
  pipeline with its feedback edges and human gates:
  generate → validate → review → refine → re-validate → publish).

## Notes

**The six stages are draft, developmental edit, technical edit, copy edit, art
and proof** (decision 11 of `plan-k1`). The stated alternative they must beat is
**two** — draft and proof. The preregistration is what makes that a test rather
than a preference, and a stage that cannot be shown to have paid for itself is
not extracted into a kind at `pipeline-kinds-k27`.

**The art stage runs with what Markdown already gives it.** The existing book has
no diagrams, no figures and no captions anywhere, and the validator has no
concept of an asset. Building that machinery now would be exactly the error
`Writegood`'s optimisation-loop record names — expensive machinery ordered ahead
of the measurement that would justify it. Run art by hand, measure it, and let
`figure-contract-k18` decide afterwards whether a format and validator support
are earned.

**The corpus is frozen and no session here fixes code inline.** A defect found
while documenting becomes its own leaf, and that leaf may not break the freeze
either: one commit carries the source change, every affected ledger and page, and
a green validator run over every book it touched, or it is deferred behind the
books it would invalidate and says so in its task file.
