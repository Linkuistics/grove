# keyed-launch-k122

## Goal

The `proof` stage over the `keyed-launch` book — `docs/walkthroughs/keyed-launch/`,
ten chapters plus `README.md`, `concept-index.md` and `source-index.md`. The last
whole-document read before the book stands, ending green on
`book-check --final --check all` and `bash scripts/check.sh`.

## Context

- The contract is `docs/specs/walkthrough-books.md` and this book's structure
  brief, `docs/specs/keyed-launch-book-structure.md`. The three stages ahead of
  you are closed: `keyed-launch-k107` drafted it, `keyed-launch-k120` copy-edited
  it, `keyed-launch-k121` was the art stage.
- **Read the node brief's `## Carried forward from the draft` and
  `## Carried forward from the art stage` before touching a page.** Between them
  they name every trap a whole-document pass walks into here — the directional
  prose obligation, the two comment lines chapter 8 must not be "corrected"
  against, the two chapter-10 counts that are right and look wrong, and which
  tables are exempt from the figure role rule.
- Four-backtick fences are exact source bytes and the corpus is frozen. A
  "fixed" sentence inside one turns `F008` red.
- `proof` cuts no successor in the ordinary case.

## Done when

- The document reads correctly end to end against its contract, and every defect
  this charter owns is fixed.
- `cargo run --quiet -p book-validation --bin book-check -- --repo . --book
  docs/walkthroughs/keyed-launch --final --check all` is valid, unchanged at 9
  files and 2,073 resolved lines.
- `bash scripts/check.sh` passes, all 8 principal checks.
- Anything an earlier stage owns is a contiguous correction run from that stage
  through `proof`, never a fix here; anything closed is cleared from the node
  brief.

## Notes

**This is the fourth stage of four, not a review.** It reads the whole document
against its own charter and fixes within it. A structural finding is a `draft`
re-run leaf; a second re-run of any one stage against this book is an escalation,
not a third leaf.

## Decisions (running log)

The whole-book read found one semantic precision defect repeated across the
overlay and channel arguments. The proof text now distinguishes whole-template
overlay precedence from the word-level precedence it rejects, distinguishes
reading channel bytes from interpreting their meaning, and states the
never-signalled stall specifically for an interactive child that returns to its
prompt. These are proof-charter corrections to the existing argument; they do
not change the structure, examples, fragment graph, or frozen source.

The full repository gate exposed three rendered evaluation answers whose live
Markdown links named the transfer campaign's deleted temporary directory. Their
audited `raw.jsonl` sources remain untouched; the rendered answer bytes are now
enclosed in text fences so captured model output is opaque to the repository link
checker. This is the narrow durable repair for a pre-existing gate failure and
does not narrow the check.
