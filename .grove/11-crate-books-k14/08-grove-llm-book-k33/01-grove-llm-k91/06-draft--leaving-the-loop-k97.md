# leaving-the-loop-k97

## Goal

Draft chapter 6 of the `grove-llm` book: slice `admit-before-signal`,
`06-leaving-the-loop.md`, owning `verbs-leaving` (`cli.rs` 248–289),
`args-complete` (311–322) and `handlers-leaving` (438–483). After this slice
every source line is resolved and only the final-only chapter remains.

## Context

- Draft stage, child 6 of 7 of `grove-llm-k91`. Responsibilities are the
  structure brief's *6 · Leaving the loop* section. Thesis: the completion
  channel is resolved and checked against the admitted epoch **before** it is
  written, because an answer that came back with the signal would come back too
  late; and `finish-commit` reads the operator's handle by the type that owns
  the grammar, lenient on the key, and quotes the operator's own spelling once.
  Cover `CompleteArgs`; `cmd_complete`'s order and its two endings, `Wrote` and
  `NoLoop`; the help's argument that the verb writes a flag and returns;
  `cmd_finish_commit`; and that these are the two verbs that open no tree.
- Link the glossary at `loop-control-channel` beside the channel and the guide
  at `usage-finish` beside `finish-commit`.
- The required example anchor is `worked-complete`: `complete` with
  `GROVE_SIGNAL_FILE=/work/atlas/.jj/grove/signal-3f9c2a7e5b1d4c8890aa61e0f27b4d13`
  — the channel resolved, checked against the admitted epoch, written, and *the
  loop will start the next task* on stderr; the same verb with no channel —
  `NoLoop`, and the instruction to exit manually. `finish-commit finish-k0001`
  is a second, shorter trace: accepted leniently, committed as `finish-k1`, the
  change id on stderr. Measure the `NoLoop` line against the built binary; the
  `Wrote` path needs a live driver and is held by
  `grove_llm_admits_only_the_live_epoch_while_version_remains_exempt` in
  `crates/grove-loop/tests/driver_lease.rs`, which the page cites.
- Evidence: `complete.rs` and `finish_commit.rs` here. Mark the rows this slice
  owns (`Handle`; `Signalled`; the two handlers that open no tree) `explained`.

## Done when

- The fragments for the three blocks are defined on the page, the defers are
  replaced, the ownership rows read `resolved`, the fragment index has their
  rows, and navigation, contents and the concept index are updated.
- `book-check --through admit-before-signal --check all` is valid: 1,017
  resolved lines, 0 deferred, `final=false`. The repository Markdown sweep
  passes. `scripts/check.sh` stays red on `book-check` alone, by design: the
  final run still reports the seventh page missing.
