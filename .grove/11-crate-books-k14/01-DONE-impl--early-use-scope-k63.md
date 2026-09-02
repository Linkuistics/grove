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

## Decisions (running log)

**1 · The offered candidate is taken: the anchor check is scoped to the proved
prefix, and the manifest keeps its authority over later-chapter rows.** The
alternative — declaring that `[[early-use]]` carries only rows provable at every
scope — was rejected because it contradicts the specification's own account of the
manifest twice over: *Authoring workflow and scoped proof* says the manifest is
complete from the start precisely so that `--through` compares a prefix against a
plan, and *The manifest is the contract* makes the mandatory rows the ones a book
may not omit. Under the alternative, a book with an early use first made on
chapter 2 would carry that promise only in the brief, which is the weaker
guarantee the workaround already fell back to. The scoped reading costs one
condition and one comparison clause, and it makes `--through` at slice one say
what it should: *the plan reaches further than this prefix, and nothing here
contradicts it*.

**2 · No decision record.** The three-part test in grove's `ADR-FORMAT.md` is an
AND. The rejected alternative is real and the reasoning is worth stating, but the
change is one condition in `check_early_uses` plus two paragraphs of
specification: it is cheap to reverse, and nothing about it is surprising once the
specification says it. It is therefore recorded where it lands — in
`docs/specs/walkthrough-books.md`, in the code comment at the condition, and in
this log.

**3 · The ordering rule is scoped with the anchor check, not left behind it.**
Canonical row order is *first-use page, then anchor occurrence in that page*, and
anchor occurrence is exactly what an out-of-prefix page cannot supply. Left as it
was — the missing offset defaulting to `usize::MAX` — two rows sharing an
out-of-prefix page would fall through to owner order and symbol text, so a prefix
run could demand an order the final run then forbids: a ledger that cannot be
authored to satisfy both. Rows sharing an out-of-prefix first-use page are
therefore unordered relative to each other until the page exists. Seen to fail:
mutating the clause back reddens
`early_uses_sharing_an_out_of_prefix_page_are_ordered_only_at_final`.

**4 · The regression case lives in the fixture book, because the book that found
it can no longer express it.** `docs/walkthroughs/jj-workspace/` is finished, so
every defer is resolved and a `--through` run over it now reports `F003` for the
blocks a prefix would still owe — the historical slice-one state is not
reconstructible from the committed book, and reconstructing it would mean editing
a frozen book's pages, which the task forbids. The `ordinal-fs-tree` test fixture
builds its own scoped snapshot, so the case is exercised there: a manifest row
first used on chapter 2, proved through chapter 1.
