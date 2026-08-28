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

## Decisions (running log)

The page partitions `src/name.rs` into six line-aligned literal fragments under
the stable `name-seam-source` composite: identifiers, classification,
representation, the consumer trait, sealed derived readings, and path-component
enforcement. This keeps every source range beside the concept that explains it
while preserving one gapless ownership block.

The worked classification examples use an explicitly hypothetical document
consumer rather than the syllabus grammar. This makes accepted, foreign,
malformed, reserved, and filesystem-species outcomes concrete without
presenting the shipped reference domain as a library default.

The page states canonicality in both directions: accepted filenames re-render
byte-for-byte, and composed or distinguished names reparse with the same view
and species. `same_name` is described narrowly as occupancy equivalence rather
than exact rendered-filename identity because same-species parts equality may
be coarser than rendering.

The type-system discharge is limited to each call's visible structural shape.
Cross-call determinism and freedom from hidden mutable state remain semantic
trait assumptions that Rust does not prove. The source-level contract question
is outside this frozen book slice and is externalised as
`entry-name-discharge-contract-k32`.

Snapshot-halting language is bounded to directories the walk reaches. A foreign
directory is skipped recursively, so malformed or reserved descendants below it
are never observed.
