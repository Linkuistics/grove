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

## Decisions (running log)

**Both clauses are corrected in one edit, not just the lane.** The task file
leaves the *last grove verbs a session runs* phrase to this leaf's judgement.
It goes, because the *Done when* asks that the comment state nothing the source
contradicts and the contradiction is eight lines below it — `eprintln!("  2.
run \`grove-llm complete\` as your last action")` in the very function the
comment heads. Leaving it would also strand half of chapter 5's adjudicating
passage: the page argues the corrected reading at length, so a fix that landed
only the lane would leave the page adjudicating a comment that had just been
edited around it.

**The replacement is hand-wrapped to hold the line count.** *jj/git* → *jj's*
and *a session runs* → *to touch the tree* net out at +1 character, so the
comment stays seven lines and `cli.rs` stays 944; only lines 770–771 change and
the other five bytes-identically survive. No `[[block]]` range in
`walkthrough.toml` moves, and no other book's `check.sh` transcript is
falsified, which is what makes this a one-book commit.

**The new phrase is the book's own gloss, promoted into the source.** Chapter 5
already read *last grove verbs* as "the last that touch the tree"; the comment
now says that, so the page explains a wording that holds instead of adjudicating
one that does not — the shape `root-init-drop-order-comment-k99` left in chapter
4. Verified against the source rather than the book: `cmd_complete` and
`cmd_finish_commit` (`cli.rs` 438–483) take no `readable`/`writable` opening,
and `worktree` resolves through `Workspace::resolve`, which
`docs/adr/jj-is-the-only-lane.md` records as refusing a tree that is not
jj-enabled.

**`docs/specs/grove-llm-book-structure.md` is left alone.** Its `## Known in
advance` section is scoped to what chapter 1 adjudicates and to claims known
*before* drafting; this one was found while drafting and was never recorded
there. Its chapter-5 plan (line 296) already says *the commit is jj's* — the
pre-authoring brief was right and the comment was wrong — and its thesis table
quotes the comment as it then stood. A structure brief records what was decided
before the book was written; editing it would make it false about what it
asserts.

**`CHANGELOG.md` line 1857 is left alone** for the same reason: it is the
release note for the change that introduced this comment, and it is in no
book's corpus.

**Corrected after review: the phrase is *the last tree verbs a session runs*,
not *the last grove verbs to touch the tree*.** The first wording this leaf
tried dropped the session scope, and unscoped it is false: `finish-commit` does
open the tree, exclusively, at `crates/grove-loop/src/tree_lifecycle.rs:227`
(`task_tree::write`), reached from `cli.rs:453` — the handler takes neither
`readable` nor `writable`, which is what the earlier reasoning mistook for the
verb not opening a tree. *Tree verbs* excludes `complete`, which is what the
original clause got wrong; *a session runs* excludes `finish-commit`, which
belongs to the finish session. Both qualifiers are load-bearing, and chapter 5
now says so and names the excluded verb rather than leaving the reader to
notice it. The phrase is one character shorter than the original, so the
comment is still seven lines and `cli.rs` still 944.

This also settles the term: *tree verbs* is already the book's word for the
distinction — chapter 5's own heading, the concept index, and
`crates/grove-llm/tests/jj_tree_verbs.rs` — so chapters 1 and 4 restate the
thesis with it too, where they previously said *grove verbs* and disagreed with
the heading they were pointing at.
