# the-surface-k78

## Goal

Draft chapter 2 of the overview: slice `no-arguments`, `02-the-surface.md`,
owning `surface-grammar` — `crates/grove/src/cli.rs` lines 1–19.

## Context

- Draft stage, child 2 of 5 of `overview-k76`. Responsibilities are the
  structure brief's *2 · The surface* section: the audience split between a
  human binary and an agent binary; the human grammar having nothing left to
  select and why; the twelve `grove-llm` verbs as a flat surface; and one
  version constant read by both binaries. Read the brief's *What the book
  deliberately does not cover*: `crates/grove-llm/src/cli.rs` is another book's
  corpus and no byte of it appears here.
- The required example anchor is `worked-argv`: the carried invocation's argv
  — `grove` alone; `grove --help` and `grove --version` stopping before the
  flow, discovering no repository and acquiring no lease; and
  `grove --harness claude` refused, with the text clap renders. **Measure the
  refusal text by running the built binary** (`cargo run --quiet -p grove --
  --harness claude` in a scratch directory) rather than reconstructing it from
  clap's conventions; do the same for `--help` and `--version`, and compare
  against the transcripts in `docs/USAGE.md`.
- **This page must carry `<a id="the-imports"></a>` on the section that reads
  lines 1–2.** Four early-use rows in the manifest name
  `02-the-surface.md#the-imports` as their first use (`DriverLease`,
  `LoopOutcome`, `TemplateSource`, `Workspace`), all owned by `one-call`; state
  each row's minimum local statement at that anchor and leave the rows
  `pending`. `grove_loop::VERSION` is owned here and is not an early use.
- Cite `docs/USAGE.md#usage-tree-verbs` beside the twelve verbs. The catalogue
  rule binds: the twelve verbs follow the worked example, never precede it.
  Read the verb list from `crates/grove-llm/src/cli.rs` (evidence) and
  `crates/grove-llm/tests/instructed_verbs.rs`, and count them.
- Leave the page shaped to receive *Command surfaces* from
  `docs/ARCHITECTURE.md` at `architecture-move-k31`; move nothing.

## Done when

- The literal and composite fragments for lines 1–19 are defined on the page,
  the source-index defer for `surface-grammar` is replaced by an insert, its
  ownership row reads `resolved`, and the fragment index has its rows.
- `README.md` links the page, chapter 1's navigation gains a Next, and
  `concept-index.md` has this chapter's entries.
- `book-check --through no-arguments --check all` is valid: 73 resolved lines,
  131 deferred. The repository Markdown sweep passes. `scripts/check.sh` stays
  red on `book-check` alone, by design.
