# spawn-ordering-comments-k119

## Goal

Correct two claims in `crates/keyed-launch/src/run.rs`'s comments that describe
mechanisms the code does not exercise — the parent-side `setpgid` "race", and the
attribution of the child's group leadership to both sides of the fork — and
re-settle the third, the SIGTTIN account of why a new session was rejected, which
this grove has not been able to check.

## Context

- Found at `the-job-k114` while writing chapter 7 of the `keyed-launch` book. The
  chapter reports each of them rather than repeating them, so nothing in
  `docs/walkthroughs/keyed-launch/` is wrong today; this leaf is what those two
  paragraphs point at.

- **Measured (high confidence), the parent's `setpgid` never wins a race.**
  `run` installs a `pre_exec` closure (line 412), which takes `std` off its
  `posix_spawn` fast path and onto fork-and-exec; that path's synchronisation
  makes `Command::spawn` return only after the child has `execve`d, at which
  point `setpgid` on that child is `EACCES` by definition. Thirty spawns of the
  same shape — `process_group(0)` plus a no-op `pre_exec`, rustc 1.98.0 on
  darwin/arm64 — returned `EACCES` thirty times, with `getpgid(child) == child`
  already true on every run. Controls in the same binary: `setpgid(self, self)`
  returned 0, and `setpgid` on an absent pid returned `ESRCH`, so the instrument
  was seen to give both other answers.

  | Line(s) | Claim | What holds |
  | --- | --- | --- |
  | 439–442 | "Whichever side runs first wins and the other fails harmlessly … doing it on both sides is what closes the window" | The window is already closed by `spawn`'s own ordering. The parent's call is insurance against that undocumented ordering changing, not one half of a live race. |
  | 591–592 | "`pgid` is the child's pid, made a group leader by the `setpgid` on both sides of the fork" | Group leadership comes solely from `command.process_group(0)` (line 403). |

  Neither claim's **conclusion** is affected: `-pgid` still cannot name an
  unrelated job, the ignored return values are still right, and line 594's
  "impossible case where both `setpgid` calls failed" already hedges correctly.
  Only the stated mechanism is wrong. The parent's call is worth keeping — the
  ordering it relies on is `std`'s implementation detail, not its contract.

- **Unsettled, and this leaf owes the measurement.** Lines 351–353 reject a new
  *session* because "a session leader has no controlling terminal, which is what
  would put an interactive child in a background group and stop it with SIGTTIN
  on its first read." SIGTTIN is raised for a process in a background process
  group **of its controlling terminal**; a process that has just been detached
  from that terminal arguably has no such relationship to it, so the stop may not
  be the mechanism at all — the competing account is that the handover itself
  becomes impossible, `tcsetpgrp` being unable to name a group in another
  session. `the-job-k114` could not decide it: the session had no controlling
  terminal (`open("/dev/tty")` → `ENXIO`), and nothing in this repository tests
  the case. **Run it before rewording it.** If the comment turns out right, this
  bullet closes with a note and chapter 7's paragraph is the thing that changes.

## Done when

- The two measured claims are reworded, and the third is either reworded against
  a run of the case or recorded as confirmed.
- **The corpus freeze is honoured in one commit.** `src/run.rs` is a book root:
  any byte change invalidates the `keyed-launch` book's `launch-shape`,
  `watch-and-launcher-signals`, `terminal-and-spawn` and `supervise-and-escalate`
  fragments and every ledger row that names their ranges. So this leaf carries
  the source change, the affected literal fragments in `07-the-job.md` and
  `08-the-escalation.md`, the `source-index.md` ranges, and a green
  `book-check --final` over the `keyed-launch` book — or it does not land.
- Chapter 7's two paragraphs that currently report these as open — the measured
  one under `#the-latch-and-the-child-away` and the unsettled one under
  `#the-child-is-a-job` — are corrected in the same commit, since a landed fix
  makes both stale.
- `bash scripts/check.sh` passes.

## Notes

**Deferred behind the book it would invalidate, and this is why it sits here.**
The root brief's cross-book rule allows a defect leaf to land only if one commit
carries the source change, every affected ledger and page, and a green validator
run over every book it touched. `keyed-launch` is the only book over `run.rs`, so
this leaf must run after `keyed-launch-book-k35` has finished all four editorial
stages — not merely after the draft. Placed ahead of `architecture-residue-k75`
with `leaf-insert`, as the root brief directs for work found while documenting.

**Nothing here is a behavioural defect.** The code does the right thing in all
three cases; what is wrong is what two comments say about why. That is still
worth fixing in a file that is 53% comment and whose comments are the argument
the book reproduces verbatim.
