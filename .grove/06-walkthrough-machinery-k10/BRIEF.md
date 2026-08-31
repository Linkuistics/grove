# walkthrough-machinery-k10 — brief

## Goal

Make the book system take six books instead of one: a shared specification, a
per-book corpus the fragment validator reads as data, and a single root at
`docs/walkthroughs/` holding every book. Nothing in P1 can be *proved* until this
lands, which is why it sits first in the walk.

## Done when

- `docs/walkthroughs/ordinal-fs-tree/` holds the existing book, every reference
  to its old path is re-pointed, and the whole repository is green.
- One specification describes the book *system* — pages, navigation, prose
  contract, audience, assurance, fragment language — and per-book ledgers are
  data it governs rather than content it contains.
- `book-check` carries no compiled-in knowledge of any particular book: source
  roots, ownership blocks, early uses, slice order and page inventory all come
  from the book directory named by `--book`.
- `scripts/check.sh` gates every book root under `docs/walkthroughs/`, not one
  named book.

## Decomposition

Ordered by what each leaf makes possible for the next.

1. `book-relocation-k15` — the wide rename, first, so every later artifact can
   cite real paths.
2. `walkthrough-books-spec-k20` — the design: where a per-book corpus lives, and
   the split of the existing spec into a shared one plus per-book data.
3. `validator-structure-k21` — the structural half of the ledger becomes data.
4. `validator-fragments-k22` — the fragment half becomes data, and the check
   script gates every book.

The two validator leaves split along `book-check`'s own `--check markdown` /
`--check fragments` seam, which is also where the compiled-in ledger divides.
That is what lets each land with the validator green over the relocated book
rather than as one unlandable rewrite.

## Pointers

- `docs/specs/ordinal-fs-tree-book.md` is the contract the existing book was
  built to, and the artifact leaf 2 rewrites. Its *Rejected alternatives* section
  contains the sidecar-manifest rejection that leaf 2 reopens.
- `crates/book-validation/` holds the validator. The compiled-in ledger is
  `SLICE_ORDER`, `ROOTS` and `BLOCKS` in `src/validator.rs` and `SOURCE_INDEX`,
  `PAGE_BY_OWNER` and `EARLY_USES` in `src/ledger.rs`; `src/cli.rs` additionally
  hard-codes the slice tokens as a `value_parser` list and names one book in its
  `about` and `after_help` text.
- Test seams: the crate's own seven test files; `book-check` itself, which proves
  structure and reconstruction; `every_repository_markdown_reference_resolves`
  (`crates/grove/tests/reference_navigation.rs`), which sweeps every Markdown
  file in the repository and so covers a moved book for free; `scripts/check.sh`
  as the umbrella.
- `docs/adr/entries-are-never-removed.md` is cited from the existing book and
  from `CONTEXT-MAP.md`; the relocation must not break either citation.

## Notes

**`book-validation` is deliberately not documented by this campaign.** It is the
authoring tool, not the system being documented, and the root brief earmarks it
to leave for the walkthrough skill. Changing it here is in scope; booking it is
not.

**The corpus is frozen.** No leaf in this node edits `crates/ordinal-fs-tree/`
source to make a check pass. If a source defect blocks the work, it becomes its
own leaf under the root brief's cross-book rule — one commit carrying the source
change, every affected ledger and page, and a green validator run over every book
it touched.
