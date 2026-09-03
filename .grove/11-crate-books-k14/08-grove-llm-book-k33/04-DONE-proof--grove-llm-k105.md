# grove-llm-k105

## Goal

Read the whole `grove-llm` book at `docs/walkthroughs/grove-llm/` as the art
stage left it — the last whole-document read, every class in charter, nothing
handed forward when you retire.

## Context

- The document is a contents page, seven chapters and two lookup indexes over
  four frozen source roots and 1,017 lines. Its structure brief is
  `docs/specs/grove-llm-book-structure.md`; the node brief carries the fixed
  editorial-pipeline context and the one entry still on the hand-forward list.
- **The book is a coherence risk by construction, and that is where to look.**
  Five chapters trace one carried session over one tree with fixed values, and
  each chapter's last table is a per-verb roll-up that chapter 7 then
  re-tabulates for all twelve verbs. So one fact can be stated in a chapter's
  prose, in that chapter's own table, and again in chapter 7's — three places,
  drifting independently. Chapter 7 also restates counts (nineteen items,
  twenty-five ownership blocks, fourteen early-use rows, 229 integration tests,
  the 107+119+214+376+101+100 split) that the earlier chapters and
  `source-index.md` are the source of.
- **What the earlier stages settled and this one may not undo.** The draft's
  adjudications are `grove-llm-k91`'s running log — in particular the six
  corpus comments judged worth a source change and the five left as written.
  The copy edit retained the named what-is-left test, the intentional *what
  order holds* title/heading, and chapter 7's `Evidence` column. The art stage
  added two figures and left five relations in prose on editorial judgement,
  with its reasons in `03-DONE-art--grove-llm-k104.md`'s log.
- **Two figures are new since the last whole-document read**, so they have had
  no coherence pass over them: the three-state carried-tree figure in
  `01-orientation.md#one-session`, whose claims about which chapter reads which
  state are checkable against chapters 2 to 6 and against the tree figures in
  chapters 5 and 6; and the drop-order table in
  `04-growing-the-tree.md#the-vacancy`, which sits beside the prose it draws and
  must not contradict it.

## Done when

- The whole book has been read for coherence across pages, and every error found
  is either fixed here or, where an earlier stage's charter owns it, carried by a
  contiguous re-run leaf from that stage through `proof`.
- The node brief's `## Handed forward` list is **empty**: its one entry — the
  concept index carrying no chapter 7 entry — is closed, or sent back, or stated
  at node close as surviving and why.
- `cargo run -p book-validation --bin book-check -- --repo . --book
  docs/walkthroughs/grove-llm --final --check all` reports 4 files, 1,017
  resolved lines, 0 deferred lines and `final=true`; `bash scripts/check.sh`
  passes.

## Notes

In the ordinary case this cuts nothing and the node closes when it retires. A
correction run is the exception, and it is what would keep the node open.

The book contract permits tables, diagrams and other fenced Markdown figures
inside the book directory. It permits no image or diagram asset beside the
pages.

## Decisions (running log)

**1 · The chapter 7 omission was not useful curation, so the concept index now
includes it.** The index remains non-exhaustive, but sixteen entries make the
assembly chapter's comparative verb table, three orders, what-is-left test,
stream partition, compiler/test boundary, external boundary, closed ledgers and
verification surfaces discoverable in the same page-order form as chapters 1
to 6. The node brief's hand-forward list is now empty.

**2 · The final read found and fixed one cross-page technical contradiction.**
Chapter 1 said the production lock scan proves the self-deadlock itself cannot
arise, while chapter 4 and chapter 7 correctly delimit that evidence: the scan
proves grove adds no blocking lock, but it cannot prove a handler never opens
the blocking store twice. Chapter 1 and the concept-index label now agree that
sequential openings are held by construction. The two art-stage figures, the
carried values, per-verb tables, six source-comment leaves, five deliberately
unchanged comments, and repeated corpus/test/ledger counts otherwise agree
across the complete document.
