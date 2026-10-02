# book-thesis-k24

## Goal

The `keyed-launch` book and its structure brief say what the book is for now
that the crate only launches an argv. The book validates today and its chapters
are true of the code; what it claims as its outcome is not yet settled.

## Context

- `argv-runner-k23` deleted chapters 2 to 5, renumbered the rest to seven pages,
  rewrote chapter 1, cut the conformance kit from chapter 5, and cut the first
  two parts of the outcome test from chapter 6. Its commit is the diff to read.
- `docs/specs/keyed-launch-book-structure.md` opens with a note that the book
  has moved ahead of it. Everything under that note still describes the
  ten-page book: the three-part pass-through outcome, the nine-row spine, the
  chapter sequence, the corpus mapping, what each chapter's prose owes, the
  worked example and the early uses.
- The brief was settled with the human in an interview. Its outcome named three
  places a layer learns a meaning it was never given: on the way in, on the way
  through and on the way out. The first two were the template half's. Only the
  way out is left, with `Argv` as the type that holds *nothing is re-read*.

## Done when

- The brief states one outcome the seven-page book serves, a spine with a row
  per source-owning chapter, the chapter sequence and corpus mapping as the
  manifest has them, and a worked example that starts from an argv. Its status
  note is gone.
- `README.md`, `01-orientation.md` and `06-what-passes-through.md` state that
  outcome the same way. Chapter 6 reads as one argument and not as a three-part
  test with two parts removed.
- The slice and page identifiers, the anchors and the fragment identifiers
  either fit the outcome or are renamed through the manifest, the pages and the
  source index together.
- `concept-index.md` is curated against the chapters as they stand.
- `book-check` validates the book, and `bash scripts/check.sh` passes.

## Notes

- Whether the outcome keeps the pass-through question or becomes the
  out-of-band ending the brief once rejected is the open decision. The brief
  rejected the second because it left the template chapters unserved, and they
  are gone. If the choice is not clear from the brief's own reasoning, ask.
- Names that outlived their reason: the slice `understands-neither`, the anchor
  `the-two-errors` over one error type, `the-seam-type` over `Argv`, the root
  `source-error-types`.
- Chapters 2, 3, 4 and 7 were renumbered and reworded only where a sentence
  named a deleted chapter or a template. Their measured figures were not
  re-measured.
- The source is not expected to change. If it does, rebase the fragments from
  the files and regenerate the source index from the manifest and the fragment
  headers. Both are derived data.

## Decisions (running log)

**The outcome is the ending test, and the spine stays.** The brief called the
out-of-band ending *sharper than the chosen outcome and genuinely transferable*
and rejected it on one ground, the template chapters it did not serve. They are
gone. The pass-through test named three places and the corpus now shows one, and
what is left of its four steps is the step that asks what ends the trip and who
decides. So the reader leaves able to name the three ways a launch ends, what
breaks without each, and the ending no observable reaches. The spine is a
separate thing and is unchanged: each of chapters 1 to 5 still opens on what its
stage must not add or interpret. Not asked: the brief's own ranking decides it.
The cost is that *nothing is added* is carried by the spine and not by the
outcome. The human can overrule this.

**The closing page is renamed and one slice name is kept.** Chapter 6 is
`06-what-ends-a-launch.md`, page `what-ends-a-launch`, with `#three-endings`
where `#where-does-it-learn` was. The anchors `the-two-errors` and
`the-seam-type` are `one-opaque-error` and `the-argv`, and the root is
`source-error-type`. `understands-neither` stays: chapter 1's heading now gives
it a reason, the program and the token, and the brief says so. No fragment
identifier needed a new name.

**Chapter 7 is recorded where it is.** It follows the assembly and does not
open on a refusal, and the brief had never heard of it. The brief now gives it a
spine row and says the assembly covers chapters 1 to 5. Moving it ahead of the
assembly is a renumbering nobody asked for.

**Statements that were false of the code were fixed where they stood.** The
`src/run.rs` line figures in chapters 3 and 4, which were one off since
`argv-runner-k23` shortened a comment, and two chapter totals; a deleted test
named twice in chapter 3; the manifest rule quoted in chapter 2 and a count in
chapter 4; the ordinal `walkthrough-books.md` quotes from the concept index;
and *every chapter* in the architecture document's row for the book. Other
measured figures in chapters 2, 3, 4 and 7 were not re-measured.

**The brief lost three sections whose subjects are gone.** The residue marker
it described is no longer in the architecture document, the interface sketch it
called stale was rewritten by `argv-runner-k23`, and KDL and shell-word
splitting left with the template half. Its figures were measured again from the
crate: `src/run.rs` is 40% comment, not 53%.

**The leaf's one reviewer was spent on the rewritten passages, against the
source.** It returned 21 findings. Valid and fixed: the scrub list was credited
with making a path this launch's alone, which is the fresh draw's doing, and
the test cited for it holds only the order; `read` refuses more than an empty
file; `relaunch` is written and discarded in the corpus, not read back, and is
not in every chapter; *nothing chooses a program* is false of the confinement
backends; the chapter-6 table disagreed with what chapters 1, 2 and 4 open on;
three figures in the brief; and four sentences that overstated. One was a
visible trade-off: the early-use table gives the two noninteractive entry points
to chapter 7 because the manifest's ledger does, though chapter 3 owns their
lines. The rest were noise. Every fix was a correction to a stated fact, checked
against the lines the reviewer cited, so no second review was cut.
