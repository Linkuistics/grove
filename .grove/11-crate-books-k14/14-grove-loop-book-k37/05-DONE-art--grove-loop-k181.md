# grove-loop-k181

## Goal

The `art` stage over the `grove-loop` book — `docs/walkthroughs/grove-loop/`,
twenty-one chapters, two lookup surfaces and a contents page over 30,421 lines of
page — holding the whole document to the two figure rules in
`docs/specs/walkthrough-books.md`, *Figures*, and to nothing else.

## Context

- **The medium is the page's own Markdown and no other.** A book directory
  carries no image and no diagram file, and the validator has no concept of an
  asset. A figure here is a table, a list figure, or a drawing inside a `text`
  fence.
- **The inventory was measured, not read.** 542 fenced-or-tabular blocks divide
  into 475 four-backtick source fragments — not figures, governed by the
  source-fragment introduction rule — and 67 figures. That split is what let one
  session hold 30,421 lines against both rules.
- The draft owned structure and technical truth and the copy edit owned prose;
  both are discharged. Nothing here reorders a chapter, re-attributes a test or
  re-adjudicates a claim about the source.

## Done when

- Every figure states its role adjacent to it. — done
- Every relation a figure would carry is drawn rather than left in running prose.
  — done
- `book-check --repo . --book docs/walkthroughs/grove-loop --final --check all`
  valid at 13 files, 10,533 resolved, 0 deferred, `final=true`. — **passes**
- `bash scripts/check.sh` — **all eight principal checks pass**
- **Last act:** `grove-llm leaf-add grove-loop-book-k37 grove-loop --kind proof`.

## What this stage changed

Four edits, all prose outside a fence, and no number, count or claim derived,
adjusted or introduced. Every cell of both new tables is taken from a sentence
already standing on its own page.

**Rule 1 — a relation carried only by running prose.**

1. **Chapter 2, `## The classification, and where a name grammar loses data`** —
   the four verdicts, drawn. The chapter is *named* for the four and argued them
   a paragraph at a time across ~480 lines: `Foreign` and `Malformed` in one
   passage, `Reserved` in a second, `Entry` only implicitly and then again in a
   test section far below. A reader had to reassemble the partition — and its
   load-bearing column is not what each verdict says about a name but what the
   store does next, which no single paragraph put beside its neighbours. Now a
   four-row table: verdict, what grove is saying, what the store does with it,
   whether this module ever returns it.

2. **Chapter 21, `## The crate's answer`** — the three questions, drawn. This
   discharges the draft's one `## Handed forward` entry, which left the call
   here deliberately. The three questions are the book's **stated outcome**; they
   run through twenty-one pages as three column headings and three recurring
   subheadings and were set out whole in exactly one place — `README.md`, before
   the reader has met any of the material. Chapter 21's own closing paragraph
   carries the relation a reader is meant to take away — question, where meaning
   is expensive, what to ask of their own layer, what happened when it was asked
   of this crate — in running prose. Now a three-row table, placed as the page's
   last figure because it is the only thing on it a reader uses on a codebase
   this book says nothing about.

**Rule 2 — a figure standing without a role statement.** Two, and they were the
only two in 67.

3. **Chapter 20, the mutation run** — three consecutive tables with the middle
   one having no adjacent prose on either side. The lead-in reached only the
   first table and the follow-up reached only the first two. Added the sentence
   that names the three-way split — the four choices, the machinery that places
   them, the mutations that moved nothing — which gives the second and third
   tables their role and makes the partition itself visible.

4. **Chapter 15, the carried-example figure** — the only carried-example figure
   in the book with nothing after it; it ran straight into an `<a id>` heading.
   Its lead-in states the figure's *subject* and, in saying *every verb answering
   with the paths it wrote*, is looser than the figure's own annotation. Added a
   following sentence carrying the figure's actual reading — a path, a report of
   paths, or the fact that there was nothing — in the page's own words.

## Figures declined, and which reason each was

Recorded because the three reasons are different findings and only the third is
evidence about the medium. **None of the three below is the third.**

- **Chapter 18, the four slots and their cardinalities** — *editorial judgement*.
  It is a mapping over four members and would ordinarily be drawn, but the
  reproduced `SLOTS` array is itself the four names beside their `Requirement`
  variants, in one block, in order. The reader reassembles nothing. A table here
  would restate a fragment rather than carry a relation the fragment does not.
- **Chapter 19, `Mandate`'s four fields** — *editorial judgement*, and the same
  ground: the struct and its four doc comments are the figure.
- **Chapter 3, the two reserved labels** — *editorial judgement*. Two members
  with an asymmetry that one paragraph states completely; a two-row table would
  be ceremony.

**Nothing was declined because the medium could not carry it.** Every relation
this stage found wanting a figure was a partition or a mapping, and a Markdown
table carried each one. The contract's medium is not reopened by this book.

## Notes

**The corpus is frozen and nothing here touched it.** No edit reached inside a
four-backtick fence, so no ledger, range or fragment moved; `book-check` reports
the same 13 files and 10,533 resolved lines as the draft and the copy edit left.

**No manifest data changed.** No H1, no navigation label and no anchor was
touched, so no `walkthrough.toml` edit is owed and no early-use row moved.

**The book's figure inventory is 69** after this stage: 46 tables and 23
non-fragment fenced blocks — 21 `text`, 2 `console`.
