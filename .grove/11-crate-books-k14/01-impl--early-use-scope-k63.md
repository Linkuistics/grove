# early-use-scope-k63

## Goal

Settle the disagreement between `docs/specs/walkthrough-books.md` and
`book-validation` over whether a manifest `[[early-use]]` row may name a first use
on a chapter later than the one being proved, and make the two agree — in the
validator, in the specification, or in both.

## Context

- **The disagreement, as found.** `check_early_uses`
  (`crates/book-validation/src/ledger.rs`) builds a required row per manifest
  `[[early-use]]` entry and requires each to appear in `source-index.md` exactly
  once; separately, it resolves every ledger row's first-use anchor by looking the
  page up in `snapshot.book_files` and reports `F009` when it is absent. In a
  scoped run the snapshot holds only the prefix's pages, and Markdown validation
  independently forbids a later page from existing. So a manifest row whose
  first use is on chapter 2 cannot be satisfied while proving chapter 1: including
  it fails the anchor check, omitting it fails the mandatory-row check.
- The specification says the manifest is **complete from the start** (*Authoring
  workflow and scoped proof*), which is what makes `--through` a comparison
  against a plan rather than against the artifacts themselves. It also says the
  `[[early-use]]` entries are "the rows a book may not omit" and that "authors add
  further rows to the book's own ledger", which is the sentence the workaround
  below rests on.
- **Where it bit.** `jj-workspace-book-k25`. Its structure brief names seven
  minimum early-use rows, two of them first used at
  `02-the-gate.md#worked-resolution`. `orientation-k55` declared only the five
  chapter-1 rows in `walkthrough.toml` and left the other two to be added to the
  ledger by `the-gate-k56`. That prefix proves green, and the two rows are still
  obligations — but they are the *brief's* obligations rather than the manifest's,
  which is a weaker guarantee than the specification intends.
- Every remaining crate book will meet this: `grove-loop` in particular has 13
  roots and will force early uses across many chapters.

## Done when

- The disagreement is resolved and the resolution is recorded — a decision record
  if it changes what the manifest means, an amendment to
  `docs/specs/walkthrough-books.md` in any case.
- Whatever the resolution, it has a test that has been **seen to fail** against
  the case that found it: a manifest declaring an early use on a later chapter,
  proved at an earlier slice.
- If the resolution restores the manifest's authority over later-chapter rows,
  `docs/walkthroughs/jj-workspace/walkthrough.toml` regains the two rows the
  workaround left out, and the book still validates `--final`.
- `bash scripts/check.sh` passes.

## Notes

**Do not touch the book's pages to make this pass.** The books are frozen
artifacts of a measured pilot by the time this runs; a change to a book's prose
here would land inside no stage's charter and account for nothing.

**One candidate resolution, offered rather than mandated.** Scope the anchor check
the way the status column is already scoped: require the anchor to exist when the
first-use page is inside the prefix, and require only structural validity when it
is not. That keeps the manifest complete from the start, keeps the row's promise
visible in the ledger from slice one, and costs one condition. Weigh it against
the alternative — declaring that the manifest carries only rows provable at every
scope — which is simpler but makes the manifest a weaker plan than the
specification claims for it.
