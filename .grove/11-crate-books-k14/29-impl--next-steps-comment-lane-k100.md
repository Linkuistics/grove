# next-steps-comment-lane-k100

## Goal

Correct the one residue in the `eprint_next_steps` comment of
`crates/grove-llm/src/cli.rs` (lines 768–774) that names a lane this build
cannot reach — *the commit itself is jj/git* — and land it as a corpus change
the book contract permits.

## Context

- Observed at `ending-work-k96` while drafting the `grove-llm` book's chapter 5,
  which owns `handlers-ending` (`cli.rs` 768–825) and reproduces this comment
  in its `eprint-next-steps` fragment (768–785). The comment reads, in part:
  *`leaf-retire` and `leaf-prune` are the terminal-marking pair and the **last
  grove verbs a session runs** — Retire precedes Commit, and the commit itself
  is jj/git — so their output lands in the agent's context at the moment of
  decision*.
- **Grove drives jj only.** `docs/adr/jj-is-the-only-lane.md` records the
  decision; `worktree` (`cli.rs` 863–870) resolves through
  `Workspace::resolve` and refuses a working tree that is not jj-enabled,
  which chapter 2 of the book read and measured. The reminder the function
  prints says *commit this session's work* without naming a tool, so only the
  comment carries the residue.
- The same comment's *last grove verbs a session runs* is one verb short —
  `grove-llm complete` follows, and the reminder's own second line says so.
  Chapter 5 reads it as the last verbs that touch the tree. Whether to tighten
  that phrase in the same edit is this leaf's call; it is not a false claim in
  the way `/git` is, and the chapter's adjudication stands either way.
- `crates/grove-llm/src/cli.rs` is the `source-command-surface` root of the
  `grove-llm` book (`grove-llm-book-k33`), which owns lines 768–785 in one
  literal fragment and must reconstruct them. A rewording that keeps the line
  count moves no boundary; one that changes it moves every range below in that
  root.

## Done when

- The comment names no version-control lane other than jj, and states nothing
  the source contradicts.
- The corpus-freeze rule in `.grove/BRIEF.md` is honoured: **one commit**
  carries the source change, every affected fragment and page of the
  `grove-llm` book (at least `eprint-next-steps` and the adjudicating prose in
  `05-ending-work.md`), and a green `book-check --final` over that book.
- `bash scripts/check.sh` passes.

## Notes

**Placed after every crate book deliberately**, beside the other frozen-corpus
comment defects (`grove-llm-version-comment-k83`,
`root-init-drop-order-comment-k99` and their neighbours) and ahead of
`architecture-residue-k75`: editing a byte of a frozen root while the book that
quotes it is being written invalidates the ranges the freeze protects. The
chapter has already adjudicated the stale clause beside the fragment, so this
leaf rewrites the comment and that page's adjudication in the same commit.
