# what-passes-through-k117

## Goal

Draft chapter 10 of the `keyed-launch` book — *What passes through*,
`10-what-passes-through.md`, slice `assembly` — and take the book to green
**final** validation and a green `bash scripts/check.sh`.

## Context

- Draft stage, child 10 of 10 of `keyed-launch-k107`, and the only one that owns
  no source. It resolves no legitimate hole: every source-owning slice has already
  replaced its defers, and this slice is final-only under
  `docs/specs/walkthrough-books.md`, *Authoring workflow and scoped proof*.
- Responsibilities are the structure brief's *10 · What passes through —
  assembly*: state the pass-through test in the form under *Audience and intended
  outcome*, and apply it to all nine source-owning chapters — for each of the three
  places a layer learns a meaning it was never given (**on the way in**, **on the
  way through**, **on the way out**), which chapters proved the crate does not, and
  by which test. The six tests the brief names are the evidence.
- **It closes on the one thing the crate cannot do anything about**: a child that
  finishes and never signals, which no observable here can distinguish from one
  still working, and which is the caller's to close at the layer that instructs the
  child. Chapter 8 stated it; this page does not soften it either.
- Required example anchor: `where-does-it-learn` — the three parts of the test,
  each answered against the chapters that proved it.
- Close the indexes, move every early-use row to `explained`, and record the final
  evidence.

## Done when

- `book-check --repo . --book docs/walkthroughs/keyed-launch --final --check all`
  is valid: 10 pages, 9 source roots, 2,073 resolved lines, no deferred holes,
  `final=true`.
- `bash scripts/check.sh` passes, all 8 principal checks, the book gated by
  discovery rather than by a hand-added line.
- **Last act:** `grove-llm leaf-add keyed-launch-book-k35 keyed-launch --kind
  copy-edit`, unless a live later sibling under `keyed-launch-book-k35` already
  holds that stage — read that condition off the node's live entries before
  cutting. A defect an earlier stage owns is instead one contiguous correction run
  in pipeline order, replacing this last act.

## Notes

Retiring this leaf leaves `keyed-launch-k107` with no live leaf: close the node
under the spine's four steps, promote what survives into
`keyed-launch-book-k35`'s brief, and name both handles in the commit message.
