# crate-books-k14 — brief

## Goal

Write the remaining documentation: a system overview covering `crates/grove`, and
one book each for `grove-llm`, `keyed-launch` and `grove-loop` — all through the
pipeline `publishing-pipeline-k13` extracted — and move
`docs/ARCHITECTURE.md`'s descriptive half into the overview.

## Done when

- `docs/walkthroughs/` holds the overview and books for `grove-llm`,
  `keyed-launch` and `grove-loop`, each proving complete reconstruction of its
  source roots under the fragment validator, alongside the pilot's
  `jj-workspace` book and the relocated `ordinal-fs-tree` one.
- `docs/ARCHITECTURE.md` carries decisions, constraints and measurement records
  only; its descriptive account of runtime flow, command surfaces and module
  seams lives in the overview, with every citation of a moved anchor re-pointed
  and the link-integrity suite green.
- The repository-wide link sweep and the architecture-anchor resolver are both
  green over the moved anchors, in Markdown **and** in Rust sources.
- `scripts/check.sh` gates every book.

## Decomposition

Ten leaves. The first is a prerequisite of the fourth; after that the pattern is
a structure brief and then its book, cheapest corpus first so the pipeline is
exercised on small books before the largest one.

1. `architecture-anchors-k19` — widen the link-integrity suite to resolve
   `docs/ARCHITECTURE.md#<anchor>` citations in Rust sources.
2–4. `overview-structure-k29`, `overview-book-k30`, `architecture-move-k31` —
   the overview, then the move into it.
5–6. `grove-llm-structure-k32`, `grove-llm-book-k33` — 4 roots, 1,017 lines.
7–8. `keyed-launch-structure-k34`, `keyed-launch-book-k35` — 9 roots, 2,073 lines.
9–10. `grove-loop-structure-k36`, `grove-loop-book-k37` — 13 roots, 10,533 lines.

The overview goes first among the books because the architecture move is on the
campaign's *Done when* and would otherwise queue behind three books; because the
`crates/grove` corpus is the smallest at 204 lines, so the shipped pipeline is
exercised gently; and because the other books link into it.

The move is a separate leaf from the overview because it is separately
verifiable: the overview is proved by the validator, the move by the link suites.

## Pointers

- The frozen corpus, per deliverable, is the root brief's *Pointers* table: 3
  roots / 204 lines for the overview over `crates/grove`, 4 / 1,017 for
  `grove-llm`, 9 / 2,073 for `keyed-launch`, 13 / 10,533 for `grove-loop`. Per
  crate that is every `src/**/*.rs` plus the crate's own `Cargo.toml`, with
  `crates/grove-loop/src/task_grow/tests.rs` excluded as an inline test module.
  `tests/` directories are evidence, not roots.
- `docs/ARCHITECTURE.md`'s descriptive sections are *Runtime flow*, *Command
  surfaces*, *Main module seams*, and the descriptive parts of *Task-tree data
  model*, *Task kinds and composition*, *Lifecycle and resumption*, *Human
  authority and completion*, *Version-control seam* and *How the methodology
  reaches a session*. Which clauses of the mixed sections are description and
  which are decisions is `architecture-move-k31`'s to settle.
- The document's stable anchors — `task-tree-scheme`, `symmetric-vcs-rule`,
  `task-kind-taxonomy` and the rest — are cited from source comments and tests as
  compact design references, and its own *Documentation ownership* section says
  changing a section title does not change the anchor.
- Test seams shared by every leaf here:
  `every_repository_markdown_reference_resolves` and
  `every_adr_citation_names_a_decision_record`
  (`crates/grove/tests/reference_navigation.rs`); the `include_str!` content
  assertions in `crates/grove-llm/tests/composition_guidance.rs`, which read
  `docs/ARCHITECTURE.md` in eight places and go red on the move **by design**;
  `book-check`; `scripts/check.sh`.
- `CONTEXT-MAP.md` argues `grove`, `grove-llm` and `grove-loop` *are* the grove
  context, and that `keyed-launch` and `jj-workspace` are deliberately not
  contexts. Each book has to respect that; `docs/walkthroughs/` was chosen as a
  flat root precisely so the directory asserts no boundary the map denies.

## Notes

**Each book leaf here is a placeholder for the pipeline's own decomposition.**
The books are to be authored through the extracted kinds, and those kinds did not
exist when this subtree was cut — grove refuses a kind no launch template
declares, so their leaves could not be written in advance. A picked book leaf
whose pipeline kinds are installed should `leaf-decompose` itself into one leaf
per stage and do only the first. A book leaf picked when they are not installed
should say so and stop, not quietly author the book by hand: the point of the
pilot was to earn a pipeline, and bypassing it wastes that.

**`grove-loop` is 72% of the remaining corpus and will decompose.** 13 roots and
10,533 lines, with `tree_lifecycle.rs` at 2,725 and `task_tree.rs` at 2,023.
Expect a node, and expect its structure brief to be the longest conversation in
this arm.

**The corpus is frozen and no session here fixes code inline.** A defect becomes
its own leaf, and that leaf may not break the freeze either: one commit carries
the source change, every affected ledger and page, and a green validator run over
every book it touched — or it is deferred behind the books it would invalidate,
and says so in its task file.
