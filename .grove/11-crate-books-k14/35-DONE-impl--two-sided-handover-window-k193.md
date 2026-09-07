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

## Decisions (running log)

**The rewrite does not invoke the ordering at all, and that is the point.** The
first draft replaced the false clause with the measurement that closed it —
"`spawn` returns only after the child has exec'd, so the child can reach its first
read before `watch` hands anything over". That is true, but it stakes the
justification on the strongest form of a premise the argument does not need: the
window exists because **nothing orders** the child's first read after the parent's
handover, and on the `posix_spawn` fast path — where `spawn` may return *before*
the exec — the window is wider rather than closed. Worse, the same file hedges
that very premise fifty lines later (line 442, "that ordering is undocumented, not
a rule"), so the draft would have asserted as fact what its neighbour calls an
implementation detail. The landed comment says instead that the parent's handover
waits on `spawn` returning and nothing orders the child's first read after that,
which holds on either path.

**Line budget held exactly.** `run.rs` stays 607 lines and the comment stays six
lines (388–393), so no ledger range moves and none of the quoted corpus totals —
`docs/specs/keyed-launch-book-structure.md`, the root brief's table, the book's
`README.md`, chapters 1, 7, 8, 9, 10, and chapter 10's two reproduced `book-check`
transcripts — is falsified. `book-check --final --check all` over `keyed-launch`
is green at 2,073 lines, in the same commit as the source change.

**The fan-out was three surfaces, and it was enumerated rather than assumed.**
Grepping the whole of `docs/` and `crates/` for the claim's phrasings found the
source literal, its reproduction inside the `run-terminal-handover` fragment, and
the one prose paragraph under `#both-sides-of-the-handover` that argues *from* it.
`docs/adr/the-launched-child-is-a-job.md` carries the SIGTTIN/session reasoning
`spawn-ordering-comments-k119` settled and never the window; the structure brief
and `concept-index.md` do not carry it either — the index's three entries for this
heading ("Entitlement and timing", `process_group(0)`, "Two restorations of
SIGTTOU") all survive the rewrite unchanged, since the section still turns on two
conditions and still has two sides.

**The in-session reviewer was spent on the rewrite, and it found two defects the
freeze would otherwise have preserved.** The pass was given the source, the
chapter section and the contract (*every clause true of this code; the argument
whole rather than a rescued half*), with the conclusion stripped. Classified:

- **Valid and actionable — the premise was unnecessary and locally contradicted.**
  The first decision above is the reviewer's finding, not the draft's.
- **Valid and actionable, and pre-existing.** The paragraph's closing sentence —
  "the same operation appears twice in this function — once here as a captured
  descriptor and once in `pre_exec` below" — was wrong before this leaf touched
  it and was carried through verbatim. Those two are one handover: an entitlement
  check and the descriptor it captures, spent once inside the closure. The real
  pair is the child's `tcsetpgrp` at line 418 and the parent's at line 504 — and
  the parent's is **not in `run` at all**, it is in `watch`, which chapter 8 owns.
  Enumerated: `run` (367–448) contains exactly one `tcsetpgrp`; `supervise`'s
  `hand_to(own_group())` at 465 is the terminal coming *back*, a different
  operation, so the forward handover has exactly two sites. Fixed here rather than
  leafed, because it sits inside a sentence this leaf had to rewrite anyway.
- **Valid and actionable, minor.** "ordered before the child's first instruction"
  was loose — `pre_exec` runs *in* the child, after `std` has already done its
  own work there. Now "before the first instruction of the program being
  launched".
- **A visible trade-off, left alone.** The entitlement clause ("Only when this
  launcher is the terminal's current owner — …") is a verbless fragment attaching
  back across a colon to the imperative three lines above. That is the original
  wording, it is what the chapter's *entitlement* paragraph reads, and rewriting
  it would spend the line budget on style rather than on a defect.
- **Noise.** That "the parent below" has no referent inside `run` — "below" is
  positional in the file, which is how the comment has always used it and how the
  next clause's `watch` resolves it.

The pass also independently confirmed against `std`'s own source what the leaf
did not need to re-measure: `pre_exec` closures short-circuit the `posix_spawn`
fast path, `process_group(0)` runs before the callbacks, and the parent blocks on
the CLOEXEC pipe until `execvp` — the k119 measurement's mechanism, checked from
the other direction.
