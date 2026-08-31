# jj-workspace-structure-k17

## Goal

Elicit, from the human, the structure brief for the `jj-workspace` book:
audience refinements, conceptual order, what deserves emphasis, and what the
reader must be able to do at the end. Commit it as an input artifact.

## Context

- Decision 15 of `plan-k1`: the source does not contain enough to structure a
  walkthrough — audience, conceptual order and emphasis are nowhere in the code —
  so every book takes a human-authored structure brief. **Do not invent one.**
  This leaf exists precisely so a session does not.
- What is already settled and must not be re-elicited: the audience (decision 7 —
  knows Rust and jj, has driven a grove, grove vocabulary linked and never
  re-taught), the depth (decision 1 — complete source-exact, every byte in a
  fragment graph), the corpus (the root brief's table: four roots, 698 lines),
  and the method (`linkuistics:writing-code-walkthroughs`, whose eight-field
  intake `plan-k1` already answered). What is open is this book's *shape*.
- The interesting tension to put to the human: `CONTEXT-MAP.md` argues that
  `jj-workspace` is deliberately **not** a bounded context — every term in it is
  Jujutsu's, and what the crate adds is a namespace it will not name for its
  consumer. A book has to decide whether that argument is its spine or a footnote.
- The existing `ordinal-fs-tree` book's page sequence is the shape to react
  against: orientation, then one concept per chapter in reader-dependency order,
  then an assembly-and-trade-offs close, with a concept index and a source index
  as lookup surfaces.

## Done when

- A committed structure brief for the `jj-workspace` book states, in the human's
  own words: the chapter sequence and what each chapter is *for*, the concepts
  that carry the book, what to emphasise and what to pass over, and the
  reader-facing outcome.
- Where it lives is settled and consistent with `docs/ARCHITECTURE.md`'s
  ownership table — a structure brief is an input to a book, not a second
  description of the crate.
- `bash scripts/check.sh` passes.

## Notes

**This is a HITL leaf and the loop stalls here by design.** An agent answering
its own structure questions has broken the distinction the whole leaf exists for.
If the human is not available, stop and say so rather than proceeding.

**This is also the manual form of P4.** How to capture such input generally is
what `specification-capture-k4`/`k5`/`k6` are surveying; nothing here waits on
that, and nothing here should anticipate its answer.
