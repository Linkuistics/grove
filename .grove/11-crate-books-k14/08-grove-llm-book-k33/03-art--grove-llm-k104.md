# grove-llm-k104

## Goal

Run the art stage over the `grove-llm` walkthrough book at
`docs/walkthroughs/grove-llm/` as the copy edit left it: assess every existing
figure and every relation carried only by prose against the figure rules in
`docs/specs/walkthrough-books.md`, then make any relation that warrants a figure
explicit in the book's Markdown medium.

## Context

- The document is a contents page, seven chapters and two lookup indexes over
  four frozen source roots and 1,017 lines. Its structure brief is
  `docs/specs/grove-llm-book-structure.md`; the node brief carries the draft's
  visual handoffs and the fixed editorial-pipeline context.
- The draft drew no figures beyond tables and console/text blocks. Chapter 7
  contains the widest candidates: a seven-column twelve-verb table, a
  five-column three-orders table and a three-column boundary table. Decide
  whether any should become a diagram or be split; do not assume width alone
  earns a change.
- Chapters 2 through 6 each carry a console transcript and prose trace over the
  same `/work/atlas/.grove/` session. Assess whether one before/after tree figure
  would serve those five chapters without duplicating their worked examples.
- The copy edit changed prose only, retained the named what-is-left test and the
  intentional *what order holds* title/heading, standardized first-use test-file
  references in running prose, and changed chapter 7's mixed `Held by` column to
  a grammatically consistent `Evidence` column. Do not reopen those decisions.

## Done when

- The whole book has been read against the prose contract's *Figures* section,
  every change stays within the art charter, and later-stage work is handed
  forward through the node brief.
- `cargo run -p book-validation --bin book-check -- --repo . --book
  docs/walkthroughs/grove-llm --final --check all` reports 4 files, 1,017
  resolved lines, 0 deferred lines and `final=true`; `bash scripts/check.sh`
  passes.
- The required `proof` successor is cut under the editorial family rule unless
  a live later sibling already holds it.

## Notes

The book contract permits tables, diagrams and other fenced Markdown figures
inside the book directory. It permits no image or diagram asset beside the
pages.
