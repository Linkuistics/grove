# the-escalation-k115

## Goal

Draft chapter 8 of the `keyed-launch` book — *The watch and the escalation*,
`08-the-escalation.md`, slice `the-launchers-job` — owning `src/run.rs` lines
124–243 (120) and 449–607 (159): 279 lines.

## Context

- Draft stage, child 8 of 10 of `keyed-launch-k107`. Responsibilities are the
  structure brief's *8 · The watch and the escalation — the launcher's job*.
- `Watch` as the supervisor's state machine; `watch`'s three observables and **the
  honest statement that they are the only three ways a launch ends** — a child that
  finishes its work and never signals reaches none of them, and the launch stalls
  rather than ends. The source names it as a real failure mode with no cheap fix
  and **the book must not soften it**; chapter 10 closes on it.
- Why ending an interactive child is the *launcher's* job — it is the child's
  parent, outside whatever sandbox the child runs under, and a child asked to end
  itself may be denied silently; the escalation addressed to `-pgid` as well as
  the pid, and what that reaps; `supervise` taking the terminal back only from the
  job this launch owned, and why returning while the terminal belonged to a dead
  group would be a SIGTTOU stop rather than an error anybody could read; `kill`'s
  deliberately ignored failure as *the shell's `kill … 2>/dev/null`, written down*.
- Then the launcher's own signals — `INTERRUPTED_BY` as process-global because a
  disposition is, latched because the launch on which the child finally exits still
  has to report it, carrying the *number* because a launcher that re-raises SIGTERM
  for a SIGHUP has told its parent the wrong thing, and cleared immediately before
  each spawn because **a latch that outlives its launch is a loaded gun**;
  `take_interrupt` for the signal that arrives between launches; `reraise` and why
  an exit code cannot express *was signalled* at all; and SIGINT's deliberate
  absence from the handler.
- **Prose obligation 3 is *do not restate*.** Same instruction as chapter 7, same
  reason. Connect and name the test.
- This chapter completes the early-use row first used at chapter 7 — move its
  status to `explained` — and produces the `End::Interrupted` chapter 7 deferred.
- Required example anchor: `the-two-graces` — the token appearing through grace →
  SIGTERM → kill-grace → SIGKILL, and the launcher's own SIGTERM as
  `End::Interrupted`.

## Done when

- `book-check --repo . --book docs/walkthroughs/keyed-launch --through
  the-launchers-job --check all` is valid: 1,836 resolved lines, 237 deferred,
  `final=false`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

This chapter closes `src/run.rs`: after it, all four blocks of that root are
resolved and only `src/channel.rs` 272–404 and `src/conformance.rs` remain.

## Decisions (running log)

**1 · Fourteen literal fragments under two composites, cut at the item in the
first block and inside the loop in the second.** Lines 124–243 are an enum, a
static and four functions, so six fragments follow the items exactly. Lines
449–607 are three functions, but `watch` alone is 110 of the 159 and carries five
separate arguments — the terminal re-check, the `try_wait` failure, the interrupt
forwarding, the `signalled` latch's placement, and the two kill arms' different
endings. One fragment for it would have left no place to answer any of them. So
`watch` is cut at its own seams: signature and locals, the `ended` closure, the
per-tick handover, the `try_wait` block, the interrupt block, and the state
machine with its sleep — six fragments over 110 lines, each with one thing to
connect. Chapter 7 cut `run` the same way for the same reason. Every fragment
begins on the blank line preceding its item, which is the convention the file's
earlier blocks already set.

**2 · The stall is stated in the chapter's opening, not saved for the close.**
The structure brief makes it chapter 8's thesis and says the book must not soften
it, and the source states it in `run`'s own doc comment, which is chapter 7's
block. So this page states it in the second paragraph, in the source's own terms
— an interactive child returns to its prompt, the launch stalls rather than ends,
and no second observable would help because nothing here distinguishes a child
that forgot from one still working — and then gives it a row of its own in the
three-observables table, with `nowhere`, `none — the launch stalls`, and
`nothing, because there is nothing to hold`. Rejected: holding it to the closing
paragraph, which would have made the chapter's own honest limit read as an
afterthought to chapter 10's.

**3 · The `-pgid` attribution is corrected by pointing rather than by repeating,
and the correction is quarantined to one paragraph.** Lines 591–592 attribute the
child's group leadership to *the `setpgid` on both sides of the fork*; the node
brief's *Found while drafting* carries `the-job-k114`'s measurement showing the
leadership comes from `command.process_group(0)` alone. The fragment reproduces
the sentence, as it must, and the prose says once that this one sentence is not
the book's to repeat, links `07-the-job.md#the-latch-and-the-child-away` for the
measurement and its controls, and then says explicitly that the comment's
*conclusion* is untouched — `-pgid` still cannot name an unrelated job. Nowhere
else on the page is group leadership attributed to anything. Rejected: restating
the measurement here, which would duplicate chapter 7's paragraph in a chapter
whose prose obligation is *do not restate*; and saying nothing, which would leave
the reproduced sentence standing as the book's own claim.

**4 · The record *the launched child is a job* is named here, for both halves.**
The structure brief requires each record to be named in prose at the chapter that
keeps it. Chapter 7 owns the spawn side of this one — the group, the terminal,
the dispositions — and does not name it; this chapter owns the escalation side,
which is the half that makes the group load-bearing. So `#the-whole-group` names
the record, states what it settles (which of the child's identities changes: its
group, explicitly not a session), and says that the two chapters keep it between
them. That discharges the obligation for the book without a correction run,
because nothing chapter 7 says is wrong — the record was simply unnamed.

**5 · No new early-use row is owed, checked rather than assumed.** The 279
reproduced lines were searched for every symbol owned by a later slice.
`checked-without-meaning` owns `conformance::check`, the `conformance` module and
`is_channel_name`; none of the three appears in either block. Everything the
blocks call is chapter 6's (`Channel::read`, `Channel::path`) or chapter 7's
(`POLL_INTERVAL`, `Terminal`, `own_group`, `Launch`, `Ended`, `End`,
`LaunchError`), all of them already explained. Both rows this chapter *owns* move
from `pending` to `explained`: `reraise`/`take_interrupt` first used at
`01-orientation.md#the-cast`, and `install_termination_handler`/`INTERRUPTED_BY`/
`supervise` first used at `07-the-job.md#the-spawn`.

**6 · The allowance was spent, and it corrected thirteen claims — three of them
about tests this chapter had described wrongly.** The leaf's one in-session
reviewer was given the page, the source, the five test files and the three
neighbouring chapters, with an adversarial *find what is wrong* brief and every
conclusion stripped. It reported sixteen findings. Classified:

- **Valid and actionable, fixed on the page — the substantive four.**
  `POLL_INTERVAL` was said to be *coarser than either of the test escalation's
  graces*, which is **backwards**: 500ms is finer than `FAST`'s 600ms and 900ms.
  The page had repeated `tests/launch.rs`'s own inline comment, which is wrong
  for the same reason; the conclusion it supports survives, so the page now
  derives it from the constant instead — every observation is quantised to a
  tick, so `elapsed` carries slack the two escalation steps do not separate. Two
  test claims were **false**: `on_terminate` is not exercised by the five tests
  in the `watch` row, because none of them signals the *launcher* — only
  `tests/interrupt.rs` runs it — and `End::Exited` is not *the arm every other
  test lands in*, since two of those five assert `End::Signalled`. And the
  entitlement guard was said to be **the same at all three `tcsetpgrp` sites**;
  `supervise`'s compares against `pgid` and the other two against `own_group()`,
  which is the very distinction the page had drawn correctly eleven paragraphs
  earlier.
- **Valid and actionable, fixed — the nine narrower ones.** *The two runs differ
  in one field* (three do). The record *the launched child is a job* rejects four
  alternatives, not two. `Watch::Terminated` is set by the interrupt path with
  whatever signal arrived, so it does not mean SIGTERM specifically.
  `the_escalation_reaps_the_childs_descendants`'s child has no `trap` and dies at
  the SIGTERM step, so it does not run the *full* escalation as this page defined
  it. Chapter 7 does not say its controls were *in the same binary*. Interrupt
  phase 1 does carry an assertion. `on_terminate` is three lines, not two.
  `Channel::read` is unique to this *file*, not to the crate. `path().exists()`
  *can* see a file mid-write; that it *does* is unobserved. And the `End::Exited`
  row's *with or without a token* was pinned by two tests that both reach it
  without one.
- **Noise.** One: that nothing on disk assigns the stall to chapter 10. The
  structure brief's *10 · What passes through* settles exactly that — it closes
  on the child that finishes and never signals — and naming an unwritten
  chapter's charter is the convention chapters 6 and 7 already use. The reviewer
  could not see the brief.

No second reviewer was materialised and no re-review cut. Twelve fixes narrowed
an overclaim to a checked fact and one replaced a borrowed wrong reason with
arithmetic over a constant the page already reproduces; none introduced a new
unverified assertion. **The three test-behaviour errors are the finding worth
carrying forward**: they were all produced by trusting a test's name or a test
file's own comment instead of its assertions, which is the failure mode a
chapter that names a test per claim is most exposed to.

## Result

`book-check --repo . --book docs/walkthroughs/keyed-launch --through
the-launchers-job --check all` is **valid: 9 files, 1,836 resolved lines, 237
deferred lines, `final=false`** — the figures this leaf's *Done when* names. Two
blocks moved from `deferred` to `resolved`: `watch-and-launcher-signals` (120)
and `supervise-and-escalate` (159), 279 lines over fourteen literal fragments
under two composites. **`src/run.rs` is closed**: all four of its blocks are
resolved and the book's only unreconstructed source is now `src/channel.rs`
272–404 and `src/conformance.rs`, both chapter 9's.

`bash scripts/check.sh` exits 1: **FAILED — 1 of 8, and the one is `book-check`**,
as every child of this node but the last is expected to leave it. Under `--final`
the keyed-launch book reports the two unwritten pages (`M101`), two navigation
and contents entries (`M103`), the two source blocks chapter 9 owns (`F003`, four
findings) and the ownership and early-use rows still `pending` for them (`F009`,
three findings — down from five, which is the two this chapter resolved). **No
finding names `the-launchers-job`, `watch-and-launcher-signals` or
`supervise-and-escalate`**, checked by search rather than by reading; the single
finding that names `08-the-escalation.md` is the `M103` requiring a `Next` link
to `09-how-checked.md`, which cannot exist until chapter 9 does — the same
position chapter 7 was in, and this session supplied chapter 7's. The other seven
checks pass, `cargo test` among them, so
`every_repository_markdown_reference_resolves` and the corpus-inventory tests are
green over the new page; the other four books still validate `final=true` (1,017
/ 698 / 8,720 / 204 lines).

**Both early-use rows this chapter owns are now `explained`** — `reraise`,
`take_interrupt` first used at `01-orientation.md#the-cast`, and
`install_termination_handler`, `INTERRUPTED_BY`, `supervise` first used at
`07-the-job.md#the-spawn` — and neither appears in the `--final` `F009` list,
which is the check that they were accepted rather than merely edited. **No new
row was owed**, established by searching the 279 reproduced lines for every
chapter-9 symbol with three controls: `conformance|is_channel_name` finds nothing
in the two blocks, ten lines under the same crate's `src/` and seven under its
`tests/`, and a deliberately-matching pattern over the extracted blocks confirms
the instrument reads them at all.

**One note was handed forward and nothing was sent back.** The node brief gained
a `## Handed forward` entry for `copy-edit`: `07-the-job.md` writes its dashes as
`&mdash;`/`&ndash;` (49 and 4 occurrences) where chapters 1–6 and 8 carry none —
house-style consistency across the whole document, invisible to a stage reading
one chapter, and not a defect in the page. No correction run was cut, because
nothing an earlier chapter says is wrong: the one obligation chapter 7 left
unmet — naming the record *the launched child is a job* — is discharged here,
where the escalation half of that record lives.

The inherited note is discharged. The node brief's *Found while drafting* warned
that lines 591–592 repeat `the-job-k114`'s measured error, attributing the child's
group leadership to *the `setpgid` on both sides of the fork*. The fragment
reproduces the sentence; `#the-whole-group` names it as the one sentence the book
does not repeat, links `07-the-job.md#the-latch-and-the-child-away` for the
measurement, and states that the comment's conclusion is untouched.
`spawn-ordering-comments-k119` still owns the fix and this chapter does not wait
on it.

**Chapter 9 is next**, and it is the last source-owning chapter: `how-checked-k116`
over `src/channel.rs` 272–404 and `src/conformance.rs` — 237 lines, taking the
book to 2,073 resolved and 0 deferred, still `final=false`.
