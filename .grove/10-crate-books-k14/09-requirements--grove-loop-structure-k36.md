# grove-loop-structure-k36

## Goal

Elicit, from the human, the structure brief for the `grove-loop` book: audience
refinements, conceptual order, what deserves emphasis, and what the reader must
be able to do at the end. Commit it as an input artifact.

## Context

- Decision 15 of `plan-k1`: the source does not carry audience, conceptual order
  or emphasis, so every book takes a human-authored structure brief. **Do not
  invent one.** This leaf exists so that no session has to.
- Already settled, and not to be re-elicited: the audience (decision 7 — knows
  Rust and jj, has driven a grove, grove vocabulary linked to `CONTEXT.md` and
  never re-taught), the depth (decision 1 — complete source-exact, every byte in
  a fragment graph), the method (`linkuistics:writing-code-walkthroughs`, whose
  eight-field intake `plan-k1` already answered), and the corpus.
- The corpus, exactly, from the root brief: 13 roots, 10,533 lines — every
  `crates/grove-loop/src/**/*.rs` plus `crates/grove-loop/Cargo.toml`. `tests/` is
  evidence, not a root.
- This is 72% of the remaining corpus, and the largest structural conversation in
  the campaign. `src/tree_lifecycle.rs` is 2,725 lines, `src/task_tree.rs` 2,023,
  `src/task_name.rs` 1,714 and `src/driver_lease.rs` 1,383.
  `src/task_grow/tests.rs` (1,680) is excluded by the root brief as an inline test
  module rather than production source.
- `CONTEXT-MAP.md` records that `grove-loop` **is** the grove context: kind,
  handle, brief chain, outcome, selection and finishing all live here, and
  `CONTEXT.md` is already their glossary. It takes the store's `Key` and `Entry`
  from `ordinal-fs-tree` and adds `Kind`, `Handle` and `Outcome` beside them — a
  collision the context map keeps apart by hand, and one the book must not blur.
- Expect this book to need many chapters. Its structure brief should say where
  the chapter boundaries fall, because the authoring leaf will decompose along
  them.
- The books already written are the shape to react against, and the pilot's
  `jj-workspace` book is the closest precedent for a new one.

## Done when

- A committed structure brief states, in the human's own words: the chapter
  sequence and what each chapter is for, the concepts that carry the book, what
  to emphasise and what to pass over, and the reader-facing outcome.
- It is specific enough that the authoring leaf needs no second interview.
- `bash scripts/check.sh` passes.

## Notes

**HITL: the loop stalls here by design.** An agent answering its own structure
questions has broken the distinction this leaf exists for. If the human is not
available, stop and say so.

**Structure, not prose.** The deliverable is the book's shape and emphasis, not
draft text; drafting is the authoring leaf's, through the pipeline.
