# grove-llm-structure-k32

## Goal

Elicit, from the human, the structure brief for the `grove-llm` book: audience
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
- The corpus, exactly, from the root brief: 4 roots, 1,017 lines — every
  `crates/grove-llm/src/**/*.rs` plus `crates/grove-llm/Cargo.toml`. `tests/` is
  evidence, not a root.
- `CONTEXT-MAP.md` records that `grove-llm` **is** the grove context — it is the
  session's verb surface over the loop and adds no vocabulary of its own. The book
  therefore links `CONTEXT.md` rather than defining anything.
- Almost the whole crate is one file: `src/cli.rs` is 944 of its 1,017 lines. How
  a book divides a single large module is the structural question here.
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
