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

## Decisions (running log)

1. **The replacement name is `bare \`grove\``**, giving line 287 *(a session
   bare `grove` did not launch)*. It is the repository's established spelling
   for the driver — `crates/grove/src/main.rs:1`, `crates/grove/src/cli.rs:4`
   and `:124` — and, crucially for this leaf's *Done when*, it is the spelling
   `crates/grove-llm/tests/removed_surface.rs:463` itself uses in the reason
   that records `do` as removed, so the corrected clause names the driver by a
   token both binaries carry. Rejected *the loop driver did not launch*: the
   same sentence already says *the loop driver's environment*, so the
   parenthesis would have explained nothing.
2. **The substitution is line-count preserving by construction.** The old
   parenthesis and the new one both fit one `///` line, so `cli.rs` stays at
   944 lines, the `verbs-complete-help` fragment keeps `272-289`, every range
   below it in the root is unmoved, and `walkthrough.toml`'s `lines = 944` is
   untouched. That is what lets one commit carry source, pages and validator —
   and it means no other book's `check.sh` transcript is falsified, since none
   of them names a `grove-llm` line total that moved.
3. **Five surfaces, enumerated rather than swept.** The source literal; the
   `verbs-complete-help` fragment (the only surface `book-check` expands); the
   adjudicating paragraph in `06-leaving-the-loop.md`; the assembly tally in
   `07-what-order-holds.md`; and the `concept-index.md` row literally titled
   for the defect. Checked and found **unaffected**: `docs/specs/
   grove-llm-book-structure.md`'s *Known in advance* section — its heading
   scopes it to what **chapter 1** adjudicates and to claims known *before*
   drafting, and this one was found *while* drafting, so it was never recorded
   there; the spec's §6 chapter plan, which does not name the clause; and
   `overview/04-proving-a-negative.md` and `05-what-the-call-reaches.md`, whose
   `grove do` mentions are about the removed verb and stay true.
4. **The assembly tally is re-enumerated, not decremented.** *Two of the five
   have since landed* becomes *All three found while drafting have since
   landed*, naming k99, k100 and k101 and the three pages each rewrote, and the
   residue clause becomes *the two known before drafting began*, which
   `grove-llm-dependency-comments-k102` carries. *Five … and four leaves carry
   them* still holds: k99, k100, k101, k102.
5. **The adjudicating paragraph is rewritten, not deleted.** It keeps what the
   page owned — that `do` is a verb neither binary has, `removed_surface.rs` as
   the record, and that only the parenthesis was ever wrong — and converts it
   into the explanation of why the comment now reads *bare `grove`*.
6. **The leaf's one in-session reviewer was spent** on the contract *every
   count, uniqueness claim, line range and cross-reference in the `grove-llm`
   book holds against the source as it now stands*, told to count for itself.
   Classified four ways:
   - **No valid finding against this book.** It re-derived 944 / 16 / 3 / 54 =
     1,017, re-tiled all 22 `cli.rs` blocks over 1–944 with no gap or overlap,
     compared all 98 fragments byte-for-byte against their declared ranges, and
     re-added the six chapter slices to 1,017. It also confirmed the
     cross-tree control this leaf needed: all five `bash scripts/check.sh`
     transcripts across the books print `1017 resolved lines` for `grove-llm`,
     so nothing outside this book moved.
   - **Valid, and not this change's** — a fragment-boundary off-by-one in
     `docs/walkthroughs/ordinal-fs-tree/04-read-path.md`, pre-existing and in
     another book. Reproduced here independently before acting on it, and cut
     as `read-path-fragment-boundary-k192` rather than absorbed. It carries the
     more interesting half — that `book-check` is green over it, so the
     per-fragment `lines="A-B"` may be an unchecked annotation.
   - **Noise** — a remark that `07-what-order-holds.md`'s list item *the
     `Complete` variant's* reads as present-tense ownership. The governing verb
     is *Three were found while drafting*, and the item's own trailing clause
     (*naming a verb neither binary has*) is still true of the string `grove
     do`, so nothing is asserted that the source contradicts.
   - **A contract stated unclearly** — none.
7. **Verified, not asserted.** `bash scripts/check.sh` exits `0` with *all 8
   principal checks pass* and all six books valid. Note that the first
   `book-check` run of this session printed `U001 unexpected argument` while the
   shell reported exit `0`, because `| tail -30` replaces a pipeline's status:
   the green readings recorded here are from unpiped runs whose text was read.
