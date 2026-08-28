# name-seam-k12

## Goal

Add the filename algebra and `EntryName` seam as a complete conceptual slice,
from ordinal/key separation through parsing, composition, species, and consumer
obligations.

## Context

- Inputs: `orientation-k11`, `book-system-k6`, and this subtree's brief.
- Primary source emphasis: `src/name.rs` and the public re-exports that expose
  its types.

## Done when

- The reader can derive why order is mutable, identity is stable, keys are the
  counter, and removal is deliberately absent.
- `Found`, `Verdict`, `Species`, `Parts`, `Triple`, parsing, composition, and
  one-component rendering obligations are explained with concrete filename
  examples and failure consequences.
- The public trait and extension surface are placed in the surrounding control
  flow rather than presented as an isolated API catalogue.
- Assigned fragments tangle exactly and scoped source coverage, Markdown, links,
  and relevant crate tests pass.

## Notes

Keep consumer-owned vocabulary visibly separate from the library-owned algebra;
the reference syllabus remains an example, not a default domain.
