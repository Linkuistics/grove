# two-sided-handover-window-k193

## Goal

Correct the first half of the timing argument at `crates/keyed-launch/src/run.rs`
lines 388–389 — "the parent can reach `tcsetpgrp` before the child's `setpgid`
has created the group" — which the same ordering measured at
`spawn-ordering-comments-k119` has already closed, and re-anchor chapter 7's
paragraph that adopts it as fact.

## Context

- Found at `spawn-ordering-comments-k119` while rewriting the two comments
  beside it. Different fragment, different claim, so it is a leaf rather than
  that session's to absorb.

- **The measurement already exists and does not need re-running.** k119's task
  file records it: `run` installs a `pre_exec` closure, which takes `std` off its
  `posix_spawn` fast path and onto fork-and-exec, whose synchronisation returns
  from `Command::spawn` only after the child has `execve`d — thirty spawns of the
  same shape, `EACCES` thirty times, with `getpgid(child) == child` already true
  on every run, and controls in the same binary showing the call can also return
  0 and `ESRCH`. So the child's group exists **before** `spawn` returns, and the
  parent's first `Terminal::hand_to(pgid)` is a `watch` tick after that (line
  504). The window the comment describes cannot open.

- **Only the first clause is wrong, and the second is what earns the design.**
  "The child can reach its first read before the parent has handed anything
  over" is true and is the whole reason the handover is also done from inside
  `pre_exec`. Deleting the false clause without rewriting the sentence would
  leave the *why* resting on the surviving one alone — check that it reads as a
  complete argument, not as a rescued half.

  | Line(s) | Claim | What holds |
  | --- | --- | --- |
  | 388–389 | "the parent can reach `tcsetpgrp` before the child's `setpgid` has created the group" | The group is created between fork and exec by `command.process_group(0)`, and `spawn` returns after exec, so the parent has no reachable window. |
  | 390–391 | "the child can reach its first read before the parent has handed anything over" | Holds, and is what the two-sided handover buys. |

## Done when

- Lines 387–392 say what actually justifies handing the terminal over from
  inside the child, and `07-the-job.md`'s paragraph under
  `#both-sides-of-the-handover` (the one beginning *The comment's own argument is
  about **timing***) no longer repeats the closed window as fact.
- **The corpus freeze is honoured in one commit**, exactly as k119's was: the
  affected literal fragment is `run-terminal-handover` (`src/run.rs` 386–397, page
  `the-job`), and `book-check --final --check all` over the `keyed-launch` book is
  green in the same commit as the source change.
- `bash scripts/check.sh` passes.

## Notes

**Fit the rewrite to the existing line budget.** k119 kept `run.rs` at exactly
607 lines for a reason: the line count is quoted in
`docs/specs/keyed-launch-book-structure.md` (2,073 corpus lines, `src/run.rs`
607, 53% comment, 324 of 607), in the root brief's frozen corpus table, in the
book's `README.md`, in chapters 1, 7, 8, 9 and 10, and in chapter 10's two
reproduced `book-check` transcripts. A one-line shift falsifies all of them and
turns a three-file change into a ten-file one. Six comment lines (387–392) are
the budget; spend them.

**Nothing here is a behavioural defect.** The two-sided handover is right; one
clause of its stated reason is not.
