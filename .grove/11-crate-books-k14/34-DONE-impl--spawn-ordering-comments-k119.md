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

## Decisions (running log)

**The third claim is wrong, and measured wrong.** The SIGTTIN account at lines
351–353 does not survive a run of the case. `the-job-k114` could not run it
because this session has no controlling terminal, but it does not need one: a
program can manufacture the whole situation by allocating a pty, forking a leader
that `setsid`s and opens the slave — which makes that pty its controlling
terminal — and running the arms under it. Three arms, reproduced identically on
three consecutive runs (clang, darwin/arm64):

| Arm | Setup | Result |
| --- | --- | --- |
| control | same session, own group, never handed the terminal | `read` → **stopped by signal 21 (SIGTTIN)** |
| control | same session, own group, `tcsetpgrp(slave, self)` — what `run` does | `tcsetpgrp` → 0; `read` → 6 bytes |
| the claim | `setsid()`, then the same handover, then the first read | `tcsetpgrp` → **ENOTTY**; `open("/dev/tty")` → **ENXIO**; first read → **5 bytes, not stopped**; the leader's own `tcsetpgrp(slave, child)` → **EPERM** |

The first arm is the control that matters: the instrument was seen to report a
SIGTTIN stop when one was due, so "not stopped" in the third arm is a reading
rather than a blind spot. So the competing account is the right one and the
consequence is worse than the comment claims, not milder. A child in a new
session is **not** stopped. It has no controlling terminal, and SIGTTIN is raised
only for a background group *of a controlling terminal*, so its first read
succeeds and it competes with the launcher for the human's typing — while the
handover that was supposed to protect it is impossible in both directions
(ENOTTY from the child's own `tcsetpgrp`, EPERM from the launcher's), leaving the
terminal's foreground group the launcher's own and every Ctrl-C with it.

Chapter 7's `#the-child-is-a-job` paragraph and `docs/adr/the-launched-child-is-a-job.md`
both carry the SIGTTIN mechanism and both change with the comment.

**Line counts are preserved, so no range moves.** Every rewrite is fitted to the
exact line budget it replaces — `run.rs` stays 607 lines, `run-the-child-is-a-job`
stays 335–366, `run-parent-group-and-supervise` 438–448, `run-kill` 581–607, and
`kill`'s doc comment stays the eighteen lines chapter 8 counts. That is not
tidiness: the corpus totals are quoted in `docs/specs/keyed-launch-book-structure.md`
(2,073 lines, 53% of `run.rs` comment, 324 of 607), in the root brief's frozen
corpus table, in `README.md`, in chapters 1, 7, 8, 9 and 10, and in chapter 10's
two reproduced `book-check` transcripts. A one-line shift falsifies all of them;
a byte-for-byte in-place rewrite falsifies only the three literal fragments.

**A fourth claim was found and externalised, not fixed here.** Lines 388–389 say
the two-sided handover is needed because "the parent can reach `tcsetpgrp` before
the child's `setpgid` has created the group". The same ordering measured above
closes that window too — the group exists before `spawn` returns, and the
parent's first `hand_to` is a `watch` tick after that. The clause beside it (the
child can reach its first read before the parent has handed anything over) is
true and is what actually earns the two-sided handover. Different fragment,
different claim, and this leaf's goal names two sites — so it is a leaf.

**The leaf's one in-session reviewer was spent on the measurement, and it found
three real gaps.** The pass was given the harness and its output with the
conclusion stripped, and asked to break it. Classified:

- **Valid and actionable — the control was confounded.** Arm 0 ran with an empty
  input queue and arm 2 with a line already waiting, so the run compared two
  things at once. Re-run with the line queued for both: the same-session
  background reader is still stopped by SIGTTIN, so the session is now the only
  variable between control and claim.
- **Valid and actionable — "the handover is impossible" rested on one route.**
  A serious implementation of the rejected option would call `TIOCSCTTY` rather
  than give up at `tcsetpgrp`, and `open("/dev/tty")` returning ENXIO is
  tautological (it resolves *through* the controlling terminal the child hasn't
  got), so it proves nothing about that route. Measured: `TIOCSCTTY` is EPERM in
  both its plain and its stealing form, and reopening the slave by name yields a
  descriptor but still no controlling terminal. The claim now stands on three
  closed routes rather than on one.
- **Valid and actionable — the consequence was asserted, not measured.** "Reads
  in competition with the launcher" and "the launcher still takes the Ctrl-C"
  had no evidence in the first run. Measured: the child in its own session read
  a queued line while the launcher's group held the foreground, and a `\003` fed
  to the master reached the launcher's handler and not the child's.
- **A contract stated unclearly.** The reviewer read "give the child a new
  session" as `setsid` *added to* `process_group(0)`, which would be EPERM — a
  process group leader cannot start a session. The option this ADR rejects is
  `setsid` *instead of* `process_group(0)`, which is what the arm does; the
  prompt did not say so.
- **Noise, or a visible trade-off.** That a fresh session's group is also
  orphaned is a second sufficient reason for "not stopped" the arm cannot
  separate — but the read returned data rather than EIO, so neither mechanism
  produced a stop and the conclusion is unaffected. One platform (macOS 26.6,
  arm64) is a real limit and is now named in both the ADR and the chapter rather
  than left implicit. The engineered sequencing (a pipe and a `usleep`) removes
  a race rather than probing one; three identical runs are reproducibility, not
  robustness, and nothing here leans on them as more.

The pass also produced the best sentence in the rewrite, which no arm measured:
both handover sites discard `tcsetpgrp`'s return and `watch` retries the
launcher's every poll tick, so under the rejected design the failure would be
silent and repeated rather than reported. That is now the comment's own wording.
