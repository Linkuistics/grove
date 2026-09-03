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
