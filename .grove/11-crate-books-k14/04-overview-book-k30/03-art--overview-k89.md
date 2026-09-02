# overview-k89

## Goal

Run the art stage over the system overview at `docs/walkthroughs/overview/` as
the copy edit left it: read every figure against the figure rules in
`docs/specs/walkthrough-books.md` and make each relation and each figure's role
explicit in the document's own Markdown medium.

## Context

- The document is `README.md`, five chapters, two lookup indexes and
  `walkthrough.toml`, at `final=true` over 3 files and 204 lines. Its structure
  brief is `docs/specs/overview-book-structure.md`; its prose precedent is
  `docs/walkthroughs/jj-workspace/`.
- The copy edit changed sentence-level prose only. It removed the draft's
  figurative, emotive and persuasive wording, restated the repeated *goes red*
  idiom as the concrete failing check, and cleared `## Handed forward` from this
  node's `BRIEF.md`. It changed no fragment, figure, page order or manifest
  field.
- The draft's six source adjudications, manifest-dependent names and chapter 5
  sentences reserved for `architecture-move-k31` remain fixed under *Pointers*
  in this node's `BRIEF.md`; this stage must not undo them.

## Done when

- The whole document has been read against the prose contract's *Figures*
  section, every change is within the art charter, and later-stage work is
  handed forward through this node's brief.
- `cargo run --quiet -p book-validation --bin book-check -- --repo . --book
  docs/walkthroughs/overview --final --check all` reports 3 files, 204 resolved
  lines, 0 deferred lines and `final=true`; `bash scripts/check.sh` passes.
- The required `proof` successor is cut under the editorial family rule unless
  a live later sibling already holds it.

## Notes

The book contract permits only tables, diagrams and other fenced Markdown
figures inside the book directory. It permits no image or diagram asset beside
the pages.
