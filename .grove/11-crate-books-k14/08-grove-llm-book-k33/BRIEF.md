# grove-llm-book-k33 — brief

## Goal

Write the `grove-llm` book under `docs/walkthroughs/grove-llm/`: a complete,
source-exact walkthrough of the crate's 4 roots and 1,017 lines, in the shape the
human's structure brief settled, passing final validation.

## Context

- Inputs: **the structure brief at `docs/specs/grove-llm-book-structure.md`**
  (elicited at `grove-llm-structure-k32`, whose decision log carries every
  rejected alternative), the shared specification from
  `walkthrough-books-spec-k20`, and the pipeline `publishing-pipeline-k13`
  extracted. The brief states the reader and the outcome, the ordered chapter
  plan with each chapter's responsibilities, and what deserves emphasis and what
  the book does not cover — the three things `grove-draft` requires of a named
  artifact — and its chapter sequence and twenty-five ownership blocks are what
  `docs/walkthroughs/grove-llm/walkthrough.toml` records.
- **Two obligations outside the book, owed before the book validates** (brief,
  *Outbound links* and *The book's row*): promote *Session epoch* and *Tree
  access lock* in `CONTEXT.md` from bold paragraphs to `###` headings with
  explicit anchors `session-epoch` and `tree-access-lock`, keeping each phrase
  as the heading text, as `overview-book-k30` did for `guaranteed-core`; and
  add the book's row to `docs/ARCHITECTURE.md`'s *Documentation ownership*
  table. **Both landed with the draft's first child** (decision 2).
- **Three stale claims in the corpus are known in advance** (brief, *Known in
  advance*): the manifest's reachability claim beside the direct `jj-workspace`
  dependency, `lib.rs`'s *or `grove`*, and the `0.1.0` comment already leafed as
  `grove-llm-version-comment-k83`. Each page states the checkable fact beside
  the fragment; none is fixed from inside the book.
- The corpus, exactly: every `crates/grove-llm/src/**/*.rs` plus
  `crates/grove-llm/Cargo.toml` — 4 roots, 1,017 lines. Every byte belongs to a
  fragment graph; `tests/` is evidence, not a root.
- `src/cli.rs` carries 944 of the 1,017 lines, and its verb surface is what
  `docs/USAGE.md` documents for the human. The book explains the same surface for
  a reader of the code; do not restate the guide.
- Scoped proof exists so a partial book is provable. Validate per slice as you
  go rather than discovering at the end that the graph does not close.

## Done when

- `docs/walkthroughs/grove-llm/` holds the book and final validation over it passes
  with no deferred holes.
- It is uniform with the other books' page conventions, navigation and prose
  contract, and is gated by `scripts/check.sh` through the book discovery rather
  than a hand-added line.
- Its links into `docs/USAGE.md` resolve, per the shared specification's
  guide-link contract, and `every_repository_markdown_reference_resolves` is what
  proves it.
- `bash scripts/check.sh` passes.

## Decomposition

The editorial pipeline's chain, under this node, in the shape
`docs/adr/the-editorial-pipeline-is-four-kinds.md` and the stage skills fix.

- `grove-llm-k91` — the **draft** stage. Decomposed into one child per slice of
  the book, in canonical page order, because seven chapters over 1,017 lines
  with a prose-to-source ratio at least the overview's is more than one
  session's work; its seventh child is the one that takes the book to green
  final validation.
- `copy-edit`, `art` and `proof` are **cut lazily**, each as the last act of the
  stage before it, every one slugged `grove-llm` and cut with
  `grove-llm leaf-add grove-llm-book-k33 grove-llm --kind <next>` — unless a
  live later sibling under this node already holds that stage. The draft's last
  child cuts `copy-edit`.

## Pointers

- **The structure brief** is `docs/specs/grove-llm-book-structure.md`. Its
  chapter sequence, its twenty-five ownership blocks, its fourteen minimum
  early-use rows and its two outbound-link tables are what
  `docs/walkthroughs/grove-llm/walkthrough.toml` records, and the brief's
  *Concept and seam responsibilities* is each chapter's charter.
- **The carried session's values are fixed at `orientation-k92`** and every
  later chapter reuses them unchanged. The tree is the overview's, extended by
  one template: `/work/atlas/` with `.jj/`, a grove holding `BRIEF.md` and one
  live leaf `01-impl--rate-limit-k3.md` (handle `rate-limit-k3`), and
  `~/.config/grove/config.kdl` declaring exactly two kinds —
  `impl "claude --add-dir ${repo} ${prompt}"` and
  `review-impl "claude --add-dir ${repo} ${prompt}"`. The driver holds
  `/work/atlas/.jj/grove/driver.lease`, wrote
  `/work/atlas/.jj/grove/session.epoch`, and launched the session with
  `GROVE_SIGNAL_FILE=/work/atlas/.jj/grove/signal-3f9c2a7e5b1d4c8890aa61e0f27b4d13`
  in its environment and its shell at `/work/atlas`. The session's verbs, in
  chapter order: `resolve rate-limit-k3`; `brief-chain` on the resolved path;
  `leaf-add . rate-limit --kind review-impl`, landing
  `02-review-impl--rate-limit-k4.md` (a fresh key is the maximum over the whole
  tree plus one); `leaf-retire` of its own leaf, landing
  `01-DONE-impl--rate-limit-k3.md`; `jj commit`; `complete`. Chapter 4's second
  ending is the same `leaf-add` with `--kind prototype`, a kind the
  configuration does not declare. The two driver verbs, `root-init` and
  `finish-commit finish-k0001`, get one short trace each in their chapters.
- **Names the manifest depends on.** `01-orientation.md#the-imports` is the
  first-use anchor of ten early-use rows and `02-the-grammar.md#worked-dispatch`
  of the other four; every chapter's H1 equals its manifest `title` and the
  navigation labels reuse it. A heading or anchor change on those is a manifest
  change, not a copy edit.
- **What the draft adjudicates, and no later stage may undo**, is the running
  log of `grove-llm-k91`'s brief as each chapter lands; the three claims known
  in advance are under *Context* above.

## Handed forward

- **`art`, the whole book:** **the draft drew no figures.** `grove-llm-k91`'s
  brief fixed that as the draft's rule — figures where the prose contract
  requires a relation to be drawn, and left to `art` otherwise — and no chapter
  met that bar, so every relation in this book is a table. Three of them are
  chapter 7's and they are the widest: the twelve-verb table is seven columns
  with sentence-length cells, and the three-orders and boundary tables are five
  and three columns of the same. Whether any of the three is a figure, or should
  be split, is this stage's call and no earlier stage's.
- **`art`, chapters 2 to 6:** the five worked examples are `console` transcripts
  with the trace in prose beneath them. They are uniform with each other by
  construction and no chapter drew the tree they act on, although four of them
  mutate it. A figure of the carried tree — `/work/atlas/.grove/` before and
  after the session — would serve five chapters at once if it earns its place.

## Decisions (running log)

**1 · Decomposed with `--kind draft`; the extracted kinds are installed.**
Verified rather than assumed: `plugins/grove/skills/` holds `grove-draft`,
`grove-copy-edit`, `grove-art` and `grove-proof`; `~/.config/grove/config.kdl`
declares all four; `leaf-decompose … --kind draft` and `leaf-add … --kind draft`
were accepted. The Claude Code plugin cache this session loaded
(`~/.claude/plugins/cache/linkuistics/grove/`) still predates the four stage
skills, so `grove-draft` was read from the working tree, as `overview-k76`'s
children did.

**2 · This `impl` session did the draft's first slice rather than the whole
draft**, on the task file's own instruction (*do only the first*) and the
precedent of `overview-book-k30`, decision 2. The draft node's brief carries the
rest. The two obligations outside the book — the `CONTEXT.md` anchors and the
ownership row — landed with that slice, because the manifest declares both
anchors and `every_book_root_has_a_documentation_ownership_row` goes red the
moment the book root exists.

**3 · `grove-llm-k91` closes with its `Done when` met in full**, checked rather
than assumed at `what-order-holds-k98`: the book is at `valid: 4 files, 1017
resolved lines, 0 deferred lines, final=true`; `README.md` cites
`docs/USAGE.md#usage-tree-verbs` and every anchor the manifest declares resolves
in its target, which `book-check`'s outbound-link checks and
`every_repository_markdown_reference_resolves` both prove; and
`bash scripts/check.sh` passes all eight principal checks with the book gated by
discovery. Seven children, one per slice, in canonical page order, each leaving
the script red on `book-check` alone until this one. Nothing was promoted out of
that node's brief beyond what this brief already points at: its decision log is
named above as *what the draft adjudicates, and no later stage may undo*, and it
stays where it is, in the brief chain of every stage that follows.
