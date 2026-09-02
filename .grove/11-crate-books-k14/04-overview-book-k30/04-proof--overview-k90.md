# overview-k90

## Goal

Run the proof stage over the system overview at `docs/walkthroughs/overview/`
as the art stage left it: the final whole-document read, every class in
charter, for errors the three earlier stages introduced or missed and for
coherence across pages — two places giving two values for one fact, a summary
contradicting a table above it, one term counted one way on one page and
another way elsewhere.

## Context

- The document: `README.md`, five chapters, two lookup indexes and
  `walkthrough.toml`, at `final=true` over 3 files and 204 lines. Its
  structure brief is `docs/specs/overview-book-structure.md`; its prose
  contract is `docs/specs/walkthrough-books.md`; its precedent is
  `docs/walkthroughs/jj-workspace/`.
- **What the art stage changed** is confined to chapters 3 and 5: six figures
  drawn where a relation was carried by prose alone, each with a role
  sentence beside it, and the prose that carried each relation trimmed to what
  the figure does not say. Chapter 3 gained a three-counts table under *The
  entry point*, an exit-status table after the `match` fragment, a
  refusal-or-environmental-failure table under *What `run` refuses*, and a
  six-step numbered list under *One foreground iteration*. Chapter 5 gained a
  `grove` against `grove-llm` table under *The test, applied back* and a
  three-class table partitioning the evidence rows under *The closed ledgers*.
  No fragment, heading, anchor, page order or manifest field changed.
- **The coherence surfaces those figures touch** are the first place to read:
  the new exit-status table in chapter 3 against chapter 2's *Three exit
  statuses* paragraph and its argument-vector table; the new refusal table
  against the two refusal transcripts above it and the `# Errors` fragment
  below it; the iteration list's step numbers against the prose that now
  cites *step 6*; the evidence-class table's row numbers against the ten-row
  evidence table it partitions; and chapter 5's *Mechanism 1's row of the
  table under "The test, applied back"* against that table.
- **What this stage must not undo** is under *Pointers* in this node's
  `BRIEF.md`: six draft adjudications against the frozen corpus, the names the
  manifest depends on, and two chapter 5 sentences reserved for
  `architecture-move-k31`.
- **What is handed to this stage** is under `## Handed forward` in this node's
  `BRIEF.md`.

## Done when

- The whole document has been read, every change is within charter, and every
  entry under `## Handed forward` is closed, turned into a re-run leaf, or
  stated at node close as surviving and why.
- `cargo run --quiet -p book-validation --bin book-check -- --repo . --book
  docs/walkthroughs/overview --final --check all` reports 3 files, 204 resolved
  lines, 0 deferred lines and `final=true`; `bash scripts/check.sh` passes.
- No next stage is cut. This is the last stage; when it retires the node has
  no live leaf and closes under the four-step close in the spine's retire
  procedure — unless this stage sends work back, in which case the correction
  run keeps the node open by construction.

## Notes

A proof session that finds nothing has still done its job. A run of defects
concentrated in one earlier stage's class is evidence about that stage, and
belongs in the body of the re-run leaf cut for it; a second re-run of one stage
against this document is an escalation, not a third leaf.
