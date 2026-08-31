# ordinal-structure-brief-k48

## Goal

Recover the `ordinal-fs-tree` book's structure brief — the page-by-page concept
and seam responsibilities and the worked-example table that
`walkthrough-books-spec-k20` moved out of the one-book specification and put
nowhere — into the durable form the shared specification now requires, at
`docs/specs/ordinal-fs-tree-book-structure.md`.

## Context

- Cut by `walkthrough-books-spec-k47` against `walkthrough-books-spec-k46`'s F5.
  The finished book shows what was written, never which responsibilities were
  binding, so today the only record of the relocated book's design is version
  control history — and a book that cannot be read against its own design cannot
  be reviewed or edited safely.
- The deleted content is in the producer's parent revision:
  `docs/specs/ordinal-fs-tree-book.md@d2d839bb-`, the page-by-page concept
  responsibilities and the worked-examples table. Read it there and recover it;
  do not re-derive responsibilities from the finished pages, which would record
  what was written rather than what was owed.
- `docs/specs/walkthrough-books.md`, *The structure brief*, fixes what a brief
  settles and where it lives. This leaf writes the first one, so the location it
  establishes is the location every later `*-structure-k*` leaf follows.
- **This is recovery, not elicitation.** The human decisions already exist and
  were written down once; a `requirements` leaf would re-ask questions that have
  answers. Where the recovered text is genuinely silent — deliberate exclusions,
  for instance — say so in the brief rather than inventing a decision.

## Done when

- `docs/specs/ordinal-fs-tree-book-structure.md` exists and covers every field
  *The structure brief* names: chapter sequence with titles, slice IDs and
  concept/seam responsibilities; the mapping onto the corpus; each chapter's
  worked example with its anchor and its start and observable end; the early uses
  the order forces; and what the book deliberately does not cover.
- Every recovered statement is traceable to the deleted specification text rather
  than to the finished book, and anything the deleted text did not settle is
  marked as unsettled instead of filled in.
- The chapter sequence and ownership mapping it records are exactly what
  `validator-structure-k21` will write into `walkthrough.toml` — this leaf is the
  human contract that manifest is the machine-readable form of, which is why it
  sits immediately ahead of it.
- No book page and no source file changes; `bash scripts/check.sh` passes and no
  link is left dangling.

## Notes

**The book itself is not edited.** The corpus is frozen and the pages are proved;
this leaf adds an input artifact beside them and touches nothing under
`docs/walkthroughs/`.

**Placement, if the ownership table objects.** `docs/specs/` is one of the
directories `docs/ARCHITECTURE.md` permits focused files under, and a structure
brief specifies a book's shape rather than describing the crate — that is the
argument for the location. If the check disagrees, fix the location and the
specification's *The structure brief* section together in this commit rather than
leaving the two disagreeing.
