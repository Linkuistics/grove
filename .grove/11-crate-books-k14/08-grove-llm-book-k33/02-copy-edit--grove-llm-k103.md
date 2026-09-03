# grove-llm-k103

## Goal

Copy-edit the `grove-llm` book — `docs/walkthroughs/grove-llm/`, seven chapters,
two lookup surfaces and a contents page over 1,017 lines of frozen corpus — and
leave it at a green `book-check --final --check all` and a green
`bash scripts/check.sh`.

## Context

- The draft is complete and green: `grove-llm-k91` retired with 4 files, 1,017
  resolved lines, 0 deferred, `final=true`. Read the whole book as the draft left
  it and change only what this stage's charter owns.
- **The charter is prose, and the draft owned structure and technical truth.**
  Conceptual order, what each chapter is for, and whether a claim about the
  binary is true were the draft's obligations and were discharged; a fix here
  that reorders a chapter or re-adjudicates a claim about `cli.rs` is a
  correction run, not a copy edit. Where you find one, cut the run the family
  file describes rather than absorbing it.
- **What is frozen and what is not.** The literal fragments are exact source
  bytes and may not be touched — `book-check` compares them byte for byte and
  `F008` reports a single character. Everything outside a four-backtick fence is
  prose you own.
- **Two names are manifest data, not headings you may reword.**
  `01-orientation.md#the-imports` and `02-the-grammar.md#worked-dispatch` are the
  first-use anchors of all fourteen early-use rows; each chapter's H1 must equal
  its `walkthrough.toml` `title`, and the navigation labels reuse it. A heading
  or anchor change on those is a manifest change.

## What the draft hands to the copy edit

- **The vocabulary decision the node brief has been holding since chapter 1 is
  now due, and chapter 7 added to it.** `## Handed forward` in
  `grove-llm-book-k33`'s brief carries the chapter 1 and chapter 6 entries; make
  the decision once and apply it across all seven chapters rather than per
  occurrence.
- **Chapter 7 carries two idioms borrowed from the `overview` book**, and neither
  was adjudicated as this book's: *the reader is to take from it that …*, used to
  introduce what a table shows, and *the … as a list*. Both appear on that page
  only. Decide whether they are the book's voice or the overview's, and either
  spread them or remove them.
- **Chapter 7 leans hard on one construction** — *what X holds* — as a term of
  art: *what the compiler holds*, *what order holds*, *what tests hold*, *held
  by*, *held by no test*. It is the chapter's title and the structure brief's own
  phrase, so it stays; what needs a decision is the density, which is highest in
  the section headed *What the compiler holds, what order holds, what tests
  hold*.
- **The seven chapters do not agree on how a test is named in prose.** Some name
  the test and its file (`… in crates/grove-llm/tests/pick.rs`), some name the
  test alone, and chapter 7's table column names bare test functions with the
  file given only where it is not this crate's. Pick one rule for the running
  prose and leave the tables as they are.
- **Chapter 7's `Held by` column mixes two grammars in one cell** — a list of
  test names, then a clause in prose (*the diagnostic by the handler alone*, *the
  stderr line itself by no test in this crate*). That is deliberate, and the
  clauses are the chapter's honesty about gaps; the copy edit owns whether the
  two read as one column.

## Done when

- The whole book has been read as one document and every defect this stage's
  charter owns is fixed.
- Anything an earlier stage owns is a correction run cut in pipeline order from
  the owning stage through `proof`, in place of the ordinary last act; anything a
  later stage owns is under `## Handed forward` in `grove-llm-book-k33`'s brief,
  and the entries this stage closed are cleared from it.
- `cargo run -p book-validation --bin book-check -- --repo . --book
  docs/walkthroughs/grove-llm --final --check all` is valid: 4 files, 1,017
  resolved lines, 0 deferred, `final=true`.
- `bash scripts/check.sh` passes.
- **Last act**: `grove-llm leaf-add grove-llm-book-k33 grove-llm --kind art`,
  unless a live later sibling under `grove-llm-book-k33` already holds that
  stage — read that off the node's live entries.
