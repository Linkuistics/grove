# complete-help-grove-do-k101

## Goal

Correct the one stale name in the `Complete` variant's doc comment in
`crates/grove-llm/src/cli.rs` (line 287) — *a session not under `grove do`* —
and land it as a corpus change the book contract permits.

## Context

- Observed at `leaving-the-loop-k97` while drafting the `grove-llm` book's
  chapter 6, which owns `verbs-leaving` (`cli.rs` 248–289) and reproduces this
  comment in its `verbs-complete-help` fragment (272–289). The clause reads, in
  full: *The signal-file default comes from the loop driver's environment
  (`GROVE_SIGNAL_FILE`); when that is absent (a session not under `grove do`)
  it is a safe near-no-op that just tells you to exit manually.*
- **Neither binary has a `do` verb.**
  `crates/grove-llm/tests/removed_surface.rs` carries it in `REMOVED_VERBS`
  with the reason *lifecycle: bare `grove`'s business, removed from both
  binaries*, and that test walks the clap model and fails if it comes back. The
  driver the rest of the same comment describes is bare `grove`, which is what
  chapter 6's live measurement ran.
- Only the parenthesis is wrong. The rest of the sentence was measured against
  the built binary: with no channel the verb prints *grove complete: no
  GROVE_SIGNAL_FILE — not running under the loop driver; exit this session
  manually*, on stderr, exit `0`, writing nothing.
- `crates/grove-llm/src/cli.rs` is the `source-command-surface` root of the
  `grove-llm` book (`grove-llm-book-k33`), whose chapter 6 owns lines 272–289
  in one literal fragment and must reconstruct them. A rewording that keeps the
  line count moves no boundary; one that changes it moves every range below in
  that root.

## Done when

- The clause names the loop driver by something both binaries carry, and states
  nothing the binary contradicts.
- The corpus-freeze rule in `.grove/BRIEF.md` is honoured: **one commit**
  carries the source change, every affected fragment and page of the
  `grove-llm` book (at least `verbs-complete-help` and the adjudicating prose
  in `06-leaving-the-loop.md`), and a green `book-check --final` over that book.
- `bash scripts/check.sh` passes.

## Notes

**Placed after every crate book deliberately**, beside the other frozen-corpus
comment defects (`grove-llm-version-comment-k83`,
`root-init-drop-order-comment-k99`, `next-steps-comment-lane-k100`) and ahead
of `architecture-residue-k75`: editing a byte of a frozen root while the book
that quotes it is being written invalidates the ranges the freeze protects. The
chapter has already adjudicated the stale name beside the fragment, so this leaf
rewrites the comment and that page's adjudication in the same commit.
