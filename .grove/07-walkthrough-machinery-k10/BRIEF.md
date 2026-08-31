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
- Decision 8 of `plan-k1` holds for books and holds mechanically: every book root
  under `docs/walkthroughs/` is inside the curated user-documentation surface and
  has a tested row in `docs/ARCHITECTURE.md`'s *Documentation ownership* table,
  both by discovery, so a sixth book joins with no edit.

## Decomposition

Ordered by what each leaf makes possible for the next.

1. `book-relocation-k15` — the wide rename, first, so every later artifact can
   cite real paths.
2. `walkthrough-books-spec-k20` — the design: where a per-book corpus lives, and
   the split of the existing spec into a shared one plus per-book data.
3. `walkthrough-books-spec-k46` — an adversarial read of leaf 2, `leaf-insert`ed
   ahead of the two leaves that implement against it. Cut by leaf 2 because it
   did not leave the sidecar rejection standing unchanged: it split it, kept the
   fragment-graph half, and moved the authoring contract into a per-book
   `walkthrough.toml`.
4. `walkthrough-books-spec-k47` — triage leaf 3's findings and rework the
   agreement point before either validator implements against it.
5. `validator-structure-k21` — the structural half of the ledger becomes data.
6. `validator-fragments-k22` — the fragment half becomes data, and the check
   script gates every book.
7. `book-assurance-surface-k39` — the curated user surface and the ownership
   table discover book roots, and both checks are seen to fail first.

The last leaf was added by `walkthroughs-k38` against `walkthroughs-k9`'s F3: decision 8
of `plan-k1` had settled both obligations and no leaf in the subtree carried
either, so the campaign could have closed green in breach of a settled
requirement. It sits in this node because it is book-*system* assurance, and last
because it needs `docs/walkthroughs/` to exist and nothing else in the node needs
it.

The two validator leaves split by **which constants each owns** — structure
metadata (`SLICE_ORDER`, `PAGE_BY_OWNER`, `SOURCE_INDEX`) against corpus data
(`ROOTS`, `BLOCKS`, `EARLY_USES`) — and *not* along `book-check`'s `--check
markdown` / `--check fragments` modes. `walkthroughs-k3` decision 4 asserted the
latter and was wrong: `ledger::check` runs only under `--check fragments`, yet
`src/ledger.rs` is where `PAGE_BY_OWNER` and `SOURCE_INDEX` are defined and where
`SLICE_ORDER` is imported, so both halves are read on the fragment path. What
lets each leaf land with the validator green over the relocated book is that the
two constant sets are disjoint, not that their readers are — and each leaf owes
`--check all`, not one mode.

## Pointers

- `docs/specs/walkthrough-books.md` is the contract every book is built to, and
  the artifact leaf 2 produced by rewriting the one-book
  `ordinal-fs-tree-book.md` in place. It settles the sidecar question leaf 2
  reopened: per-book data lives in a `walkthrough.toml` manifest beside each
  book, the fragment graph stays in Markdown, and
  `docs/adr/a-book-cannot-witness-its-own-corpus.md` records why. Leaves 3 and 4
  implement against its *The manifest* and *The corpus rule and its witness*
  sections.
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
