# the-job-k114

## Goal

Draft chapter 7 of the `keyed-launch` book — *The child is a job*, `07-the-job.md`,
slice `nothing-else-added` — owning `src/run.rs` lines 1–123 (123) and 244–448
(205): 328 lines, the heaviest chapter in the book.

## Context

- Draft stage, child 7 of 10 of `keyed-launch-k107`. Responsibilities are the
  structure brief's *7 · The child is a job — nothing else added*.
- `Escalation`'s two waits and what each is for; `Launch` as *everything one
  launch is*, every field the caller's, and `run`'s promise that nothing else is
  added — no argument, no flag, no variable; the scrub list as the caller's
  obligation discharged here, and why an environment is inherited rather than
  addressed; `cwd` and why `None` is rarely what a launcher wants.
- `Ended` and `End`'s three cases, and **the distinction the chapter must make
  carefully** — `Signalled` is narrower than *a token appeared*, because a child
  that signals and exits inside its own grace was never touched and comes back
  `Exited` with a token.
- `DEFAULT_DISPOSITION_IN_CHILD` and its argument that **only an ignored
  disposition survives `execve`**, so the list is about a launcher's own ignores
  rather than its handlers; `Terminal::open` and why `/dev/tty` rather than stdin
  is the gate that needs no flag; the spawn itself — the child's own process
  group, the terminal handed over, and why a new *session* was rejected; and
  `POLL_INTERVAL` as not a knob. Non-Unix portability is closed rather than
  unexamined: one sentence, no more.
- **Prose obligation 3 reverses here.** `src/run.rs` is 53% comment and argues its
  cases in situ; the fragment graph quotes those comments verbatim on the page.
  **Do not restate them.** Connect the arguments across items, name the test, and
  do nothing else. A page that paraphrases an argument the reader has just read in
  the source has failed the obligation.
- **`End::Interrupted` is defined here and produced only in chapter 8**; name it
  and defer. The early-use ledger already carries the row for
  `install_termination_handler`, `INTERRUPTED_BY` and `supervise`, which `run`
  reaches as its first and last acts — state the minimum at the anchor.
- Required example anchor: `the-spawn` — `run` with that argv and channel through
  to the child in its own group, holding the terminal, `GROVE_SIGNAL_FILE` set and
  the scrub applied.

## Done when

- `book-check --repo . --book docs/walkthroughs/keyed-launch --through
  nothing-else-added --check all` is valid: 1,557 resolved lines, 516 deferred,
  `final=false`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

Two blocks with chapter 8's 124–243 between them. The split is by whose signal it
is: everything done *to the child* is here, everything about *endings* is
chapter 8's.

## Decisions (running log)

**1 · Nineteen literal fragments under two composites, cut at the item outside
`run` and at the argument inside it.** Lines 1–123 are a thesis, a constant and
four public types, so six fragments follow the items exactly. Lines 244–448 hold
a constant, a private type with four methods, a free function and a 114-line
`run`, and the item cut breaks down there: one fragment for the whole of `run`
would have been the longest literal in the book by a factor of two, with six
separate arguments inside it and no place to answer any of them. So `run` is cut
at its own seams — the doc comment, the command and environment, the terminal
handover, the process group, the `pre_exec` closure, the latch and spawn, and the
parent's `setpgid` — seven fragments over 114 lines, each with one argument to
connect. Rejected: cutting `run` at every statement, which would have produced
fragments too small to carry a claim.

**2 · The chapter states a coverage gap the source does not: nothing in this
repository *asserts* on the terminal handover.** The wording matters — the path
runs, in whichever of its two branches the runner produces, and it is the
assertion that is missing rather than the execution; the first draft of this
paragraph got that wrong and the in-session review (decision 6) is what caught
it. Verified by search with both controls rather than by a clean grep: the pattern `tty|foreground|tcsetpgrp|pgid|
process_group|setpgid|getpgrp` finds nothing under `crates/keyed-launch/tests/`,
finds 46 lines under the same crate's `src/` with identical flags (the positive
control), and finds the class in three *other* crates' `tests/` directories (the
cross-tree control), so a clean read here is not an instrument that reads clean
everywhere. Three rows of the chapter's item table therefore carry `—`, and the
prose says in those words that every claim about the handover rests on the source
and the operating system rather than on this suite. `POLL_INTERVAL` is a fourth
`—` for a different reason, which the constant's own comment supplies. This is
the method's rule against turning an inference into a source fact, applied to
absence rather than to a claim.

**3 · No new early-use row is owed, checked rather than assumed.** The manifest
already carries the row this chapter's source forces —
`install_termination_handler`, `INTERRUPTED_BY`, `supervise` at
`07-the-job.md#the-spawn`, owned by `the-launchers-job` — and the chapter states
the minimum for all three at that anchor. The 328 reproduced lines were then
searched for every other chapter-8 symbol: `reraise` (line 116) and
`take_interrupt` (lines 120, 429) both fall under the ledger's existing
`reraise`/`take_interrupt` row, and `Watch` and `on_terminate` do not appear at
all. The row this chapter *owns* — `run`, `Launch`, `Ended`, `End`, `Escalation`
— moves from `pending` to `explained`.

**4 · Thirteen sections, and the required anchor ends at a process rather than at
an `Ended`.** `#the-spawn` is the brief's named example anchor and its stated
observable end is *the child in its own group, holding the terminal, with
`GROVE_SIGNAL_FILE` set and the scrub applied* — none of which is a return value.
So the section renders the end state as a process picture (pids, groups, argv,
cwd, environment, dispositions) and says explicitly that the poll, the grace and
the value coming back are chapter 8's. Rejected: running the example through to
`Ended`, which would have made chapter 8's own anchor `#the-two-graces` a repeat.

**5 · The third prose obligation is the *do not restate* one, and this chapter is
where the structure brief names it directly.** `src/run.rs` is 320 comment lines
of 607 — 53%, measured this session — and those comments argue their cases in
situ. The prose between fragments therefore does only the two things a doc
comment structurally cannot: it connects an argument in one item to an argument
in another, and it names the test. The connections it makes, none of which any
single comment holds: chapter 6's *allocation writes nothing* is why a leaked
channel path in `scrub`'s comment is authority rather than mere untidiness;
SIGTTOU appears in the disposition list, in `hand_to` and in `pre_exec` for one
reason, restored two different ways; SIGINT's presence in the disposition list
and its absence from chapter 8's handler are one decision seen from both ends;
`setsid`'s rejection in `run`'s doc and SIGTTIN's membership of the disposition
list are likewise one decision; and `Terminal::open` returning `Option` is
`Escalation` having no `Default` arrived at from the opposite direction.

**6 · The allowance was spent, and it moved two claims from the page into a
defect leaf.** The leaf's one in-session reviewer was given eight assertions with
the reasoning stripped and an adversarial *find what is wrong* brief. Classified:

- **Valid and actionable, fixed on the page.** `O_CLOEXEC` was framed as keeping
  the child from holding a descriptor onto the terminal — **wrong**, since `run`
  configures no stdio and the child inherits fds 0/1/2, which in the case that
  matters *are* that terminal; the flag is descriptor hygiene and the page now
  says so. *Only an ignored disposition* was **overstated**: six of the seven
  entries defend against an inherited ignore, and SIGTTOU is also the restore
  half of the closure's own `SIG_IGN`, which the page's own later section already
  said — the two paragraphs contradicted each other and now do not. The coverage
  claim *no test exercises the handover* was **overstated**: the path runs, in
  whichever branch the runner produces, and what no test does is assert on it.
- **Valid, and beyond this leaf's authority to fix.** Two source comments state
  mechanisms the code does not exercise. Re-measured here rather than accepted:
  thirty spawns of `run`'s exact shape returned `EACCES` thirty times, with the
  child already its own group leader, against controls showing the same call can
  return success and `ESRCH` — so lines 439–442's race and lines 591–592's
  attribution of group leadership to both sides of the fork are both wrong, and
  `spawn-ordering-comments-k119` owns the fix under the freeze. The page states
  the measurement and says nothing in the repository pins it.
- **Valid, and deliberately left undecided.** The SIGTTIN account at lines
  351–353 could not be checked: this session had no controlling terminal
  (`open("/dev/tty")` → `ENXIO`) and no test covers the case. The page therefore
  reproduces the argument and declines to adopt it, naming the competing account,
  and k119 owes the run. **This is the one place the book says it has not
  checked something**, which is the method's rule against turning an inference
  into a source fact applied to a claim the source itself makes.
- **Noise.** That `run` constructs exactly one `LaunchError` is true and vacuous;
  the `foreground`/`hand_to`/`tcsetpgrp` counts came back as a quarrel with the
  source comment's arithmetic rather than with the page, which never repeats it.

No second reviewer was materialised and no re-review was cut: every fix either
narrowed an overclaim to a measured fact or moved a claim off the page into a
leaf, and none introduced a new unverified assertion.

## Result

`book-check --repo . --book docs/walkthroughs/keyed-launch --through
nothing-else-added --check all` is **valid: 9 files, 1,557 resolved lines, 516
deferred lines, `final=false`** — the figures this leaf's *Done when* names. Two
blocks moved from `deferred` to `resolved`: `launch-shape` (123) and
`terminal-and-spawn` (205), 328 lines over nineteen literal fragments under two
composites. `src/run.rs` is now the book's second partly-reconstructed root; its
remaining 279 lines are chapter 8's.

`bash scripts/check.sh` exits 1: **FAILED — 1 of 8, and the one is `book-check`**,
as every child of this node but the last is expected to leave it. Under `--final`
the keyed-launch book reports the three unwritten pages (`M101`), two navigation
and contents entries (`M103`), the four source blocks chapters 8 and 9 own
(`F003`, eight findings) and the ownership and early-use rows still `pending` for
them (`F009`, five findings). **No finding names `nothing-else-added`,
`launch-shape` or `terminal-and-spawn`**, checked by search rather than by
reading; the single finding that names `07-the-job.md` is the `M103` requiring a
`Next` link to `08-the-escalation.md`, which cannot exist until chapter 8 does —
the same position chapter 6 was in, and this session supplied chapter 6's. The
other seven checks pass, `cargo test` among them, so
`every_repository_markdown_reference_resolves` and the corpus-inventory tests are
green; the other four books still validate `final=true` (1,017 / 698 / 8,720 /
204 lines).

The early-use row this chapter owns — `run`, `Launch`, `Ended`, `End`,
`Escalation`, first used at `01-orientation.md#the-cast` — is now `explained`, and
it does not appear in the `--final` `F009` list, which is the check that it was
accepted rather than merely edited. **No new row was owed**, established by
searching the 328 reproduced lines for every chapter-8 symbol rather than by
assuming: `install_termination_handler`, `INTERRUPTED_BY` and `supervise` are the
row the manifest already carries at `07-the-job.md#the-spawn`, `reraise` and
`take_interrupt` fall under an existing row, and `Watch` and `on_terminate` do
not appear.

**One leaf was cut and one note handed forward, both for the same finding.**
`spawn-ordering-comments-k119` (`impl`, `leaf-insert`ed ahead of
`architecture-residue-k75`) owns two source comments that state mechanisms the
code does not exercise — measured here at thirty spawns with controls — and owes
a measurement of a third this session could not run. It is deferred behind this
whole book because `src/run.rs` is a root. The node brief gained a *Found while
drafting* section because **the second site of the measured defect is in chapter
8's block** (lines 591–592) and the brief chain is the only thing a later sibling
reads. No book page is wrong today: chapter 7 reports both rather than repeating
them, which is why this is a handed-forward note and not a correction run.

**The corpus was not edited.** Five files changed under
`docs/walkthroughs/keyed-launch/` — the new chapter, the source index, the
concept index, the contents entry, and chapter 6's `Next` link — plus the two
`.grove/` files and the new leaf.
