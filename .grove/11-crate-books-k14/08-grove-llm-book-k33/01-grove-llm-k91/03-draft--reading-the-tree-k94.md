# reading-the-tree-k94

## Goal

Draft chapter 3 of the `grove-llm` book: slice `information-not-error`,
`03-reading-the-tree.md`, owning `verbs-reading` (`cli.rs` 75–124),
`handlers-reading-and-rendering` (514–636) and `path-and-label-helpers`
(904–944).

## Context

- Draft stage, child 3 of 7 of `grove-llm-k91`. Responsibilities are the
  structure brief's *3 · Reading the tree* section. Thesis: an absent answer is
  information, so `pick` on a finished grove prints its diagnostic on stderr and
  exits zero, `resolve` reports not-found and ambiguity the same way, and a
  `DONE` or `ABANDONED` match prints its path **and** a note. Cover the four
  contracts as their help states them; `leaf_in`; `normalize_leaf_path` and its
  three cases; `label` and `no_live_leaves`; the root as the one answer with no
  entry behind it; and `render_resolution` — the crate's one pure function,
  `pub` and `#[must_use]` because it is unit-tested through the library target.
- Carry the through-line: the manifest's separate-crate argument is why
  `lib.rs` exists, which is why `render_resolution` is `pub`; `worktree`
  resolving through `Workspace::resolve` is why `normalize_leaf_path` may pass a
  bare grove-relative name through for the verb to join.
- Link the glossary at `task-tree-scheme` beside the reference grammar.
- The required example anchor is `worked-resolve`: `resolve` at full
  resolution with three renderings — the live leaf's path on stdout and nothing
  on stderr; the same handle after retirement — the path, and the note; and a
  bare slug two entries share — empty stdout, the keys on stderr, exit zero.
  `brief-chain` for the same leaf follows on the same values. Measure the
  renderings against the built binary on a scratch tree.
- Evidence: `pick.rs`, `brief_chain.rs`, `kind.rs`, `resolve.rs` and
  `resolve_rendering.rs` in this crate's tests. Mark the three early-use rows
  this slice owns (`Sought`, `Resolution`; `Reference`; `Outcome`) and the
  reading-handlers row `explained`.

## Done when

- The fragments for the three blocks are defined on the page, the defers are
  replaced, the ownership rows read `resolved`, the fragment index has their
  rows, and navigation, contents and the concept index are updated.
- `book-check --through information-not-error --check all` is valid: 440
  resolved lines, 577 deferred. The repository Markdown sweep passes.
  `scripts/check.sh` stays red on `book-check` alone, by design.
