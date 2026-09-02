# what-the-call-reaches-k81

## Goal

Draft chapter 5 of the overview — slice `assembly`, `05-what-the-call-reaches.md`,
owning no source — and take the book to green **final** validation and a green
`bash scripts/check.sh`.

## Context

- Draft stage, child 5 of 5 of `overview-k76`, and the only child whose `Done
  when` carries the final run and the umbrella script. Responsibilities are the
  structure brief's *5 · What the call reaches* section: the map of what the
  call reaches; the statement of this book's boundary — everything behind
  `grove_loop::run` is named and explained nowhere here, said once rather than
  per row; and the transferable thin-entry-point test applied back across the
  three mechanisms. It has no worked-example section; its job is synthesis, as
  `jj-workspace`'s seventh chapter's is.
- **The module table is `architecture-move-k31`'s to bring, and this session
  moves nothing.** What this chapter can carry from the corpus and the
  workspace as they stand is a package-level map: the six workspace packages
  (`Cargo.toml` at the root names the members), which of them `grove` reaches
  and through what — every one only as a `grove-loop` re-export — and the one
  row that is a module this book explained, `grove::cli`. Shape the page so
  the twelve-module table from *Main module seams* lands under its own heading
  at k31, and do not name a book per row (brief, decision 11).
- The glossary anchor `guaranteed-core` is declared for this page, beside the
  `prompt` module's row. That row arrives with k31; if this session cites the
  term elsewhere, cite it at the first use of *guaranteed core* and nowhere
  twice. A declared anchor no page cites is not a finding.
- The catalogue rule allows the map to open the page because the chapter has
  no example section.
- Close the indexes: the concept index's last entries, the owned-source totals'
  role sentence, and the final-verification section that records the exact
  commands run and what each proved, as `07-what-jj-owns.md` does.

## Done when

- `book-check --book docs/walkthroughs/overview --final --check all` is valid:
  3 files, 204 resolved lines, 0 deferred, `final=true`.
- `bash scripts/check.sh` passes, the overview gated by discovery.
- **Last act**: `grove-llm leaf-add overview-book-k30 overview --kind copy-edit`,
  unless a live later sibling under `overview-book-k30` already holds that
  stage — read that off the node's live entries. Write into the new leaf's body
  the specific things this draft hands to the copy edit, and put anything a
  later stage owns under `## Handed forward` in `overview-book-k30`'s brief.
