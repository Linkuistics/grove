# overview-book-k30

## Goal

Write the system overview under `docs/walkthroughs/`: a source-exact account of
`crates/grove` — 3 roots, 204 lines — in the shape the human's structure brief
settled, passing final validation.

## Context

- Inputs: the structure brief from `overview-structure-k29`, the shared
  specification from `walkthrough-books-spec-k20`, and the pipeline
  `publishing-pipeline-k13` extracted.
- The corpus, exactly: `crates/grove/src/main.rs` (13), `crates/grove/src/cli.rs`
  (137) and `crates/grove/Cargo.toml` (54). Every byte belongs to a fragment
  graph. `crates/grove/tests/` is evidence, not a root — which matters here more
  than anywhere, because that directory holds the link-integrity suite this whole
  arm depends on.
- `crates/grove` is the human's entry point onto the loop and adds no vocabulary
  of its own (`CONTEXT-MAP.md`): it *is* the grove context along with `grove-llm`
  and `grove-loop`. The overview should read as the system's account, not as a
  fourth crate's.
- Decision 6 of `plan-k1`: `crates/grove` is covered inside the overview rather
  than getting a book of its own, because the overview absorbs command surfaces
  anyway.

## Done when

- The overview exists under `docs/walkthroughs/` and final validation over it
  passes with no deferred holes.
- It is uniform with the other books' page conventions, navigation and prose
  contract, and it is gated by `scripts/check.sh` through the book discovery
  rather than a hand-added line.
- Its links into `docs/USAGE.md` resolve, per the shared specification's
  guide-link contract, and `every_repository_markdown_reference_resolves` is what
  proves it.
- `bash scripts/check.sh` passes.

## Notes

**This leaf writes the overview; it does not perform the move.**
`architecture-move-k31` relocates `docs/ARCHITECTURE.md`'s descriptive account
and re-points its citations. Leave the destination shaped for it — the structure
brief says what shape — but do not start moving prose, or the move and the guard
that proves it land in one unreviewable commit.

**Author it through the pipeline.** If the extracted kinds are installed,
`leaf-decompose` this leaf into one leaf per stage and do only the first. If they
are not, stop and say so rather than authoring by hand.
