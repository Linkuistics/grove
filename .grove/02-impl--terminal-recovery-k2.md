# terminal-recovery-k2

## Goal

Restore usable terminal input after an LLM calls Ravel's force-stop or quits
unexpectedly, even when that LLM leaves terminal modes enabled. Reproduce the
failure, fix its cause and cover both exit paths through the agreed test seam.

## Context

- Requirements and the human's corrections are recorded in `plan-k1`; the root
  brief carries the agreed behavior and test seams.
- The reported setup is an LLM running in a Herdr pane inside iTerm. The LLM
  called Ravel's force-stop; Herdr itself did not force-stop. Ordinary keys then
  appear as escape sequences and are ignored, with `c` producing something like
  `5u_`. The human also sees this after unexpected LLM-session quits.
- Existing terminal reset and result handling:
  `crates/grove-loop/src/loop_driver.rs` (`reset_terminal`, `drive`).
- Foreground ownership and saved termios restoration:
  `crates/grove-loop/src/ravel_job.rs` (`restore`).
- Existing process/PTY integration seam:
  `crates/grove/tests/loop_driver.rs`, especially
  `force_stop_leaves_remaining_work_resumable_after_continuation` and
  `a_driver_term_cancels_dispatch_and_restores_the_terminal_after_reaping_the_harness`.
- Existing execution contract:
  `docs/specs/harness-selection-and-execution.md`; relevant glossary terms are
  Terminal disposition and Exit signal / session completion.

## Done when

- The failing terminal state is reproduced and its cause established, including
  which layer of the Herdr/iTerm setup retains it. Treat extended keyboard
  reporting as a hypothesis until evidence confirms it.
- After either agent-issued force-stop or unexpected LLM termination, ordinary
  text, Enter, Backspace and Ctrl-C work when control returns. Recovery requires
  no manual terminal reset and does not depend on the LLM's own cleanup.
- Automated regression coverage through the existing process/terminal-session
  boundary demonstrates the failure before the repair and recovery after it for
  both exit paths. Observe terminal keyboard behavior or protocol state; an
  assertion only about saved termios or an emitted reset string is insufficient.
- Foreground-terminal ownership, descendant reaping, result interpretation and
  unfinished-work resumption retain their existing behavior. In particular,
  force-stop exits with status 1 and leaves live work resumable; an unexpected
  exit must not acknowledge task completion.
- The agreed smoke check with an LLM in a Herdr pane inside iTerm confirms the
  user-visible recovery. If this needs the human, request that check and keep
  unverified app behavior explicit.
- Relevant reusable project checks pass, and any durable terminal-cleanup
  contract clarified by the repair is reflected in the existing documentation.

## Notes

The requirements investigation found that Grove calls `reset_terminal()` after
the Ravel job returns, before interpreting the session result. The function
checks foreground ownership, runs `stty sane`, leaves the alternate screen,
shows the cursor and resets text attributes. Its current sequence does not reset
extended keyboard reporting. Ravel-job restoration handles termios separately.
These observations suggest investigating the shared cleanup boundary for both
exit paths; they do not establish the root cause or mandate a particular fix.

The [Kitty keyboard protocol](https://sw.kovidgoyal.net/kitty/keyboard-protocol/)
can encode ordinary keys as escape sequences and maintains independent keyboard
mode stacks for the main and alternate screens. Account for that separation
when reproducing and testing a possible protocol leak. Preserve the existing
boundary between Ravel's process supervision and Grove's result conventions.

Use the root brief's agreed test seams. A fake agent may leave extended keyboard
reporting enabled and either force-stop or quit without cleanup. Retain the
existing lifecycle/resumption assertions alongside keyboard recovery. The app
smoke check must use the corrected Herdr-pane setup, rather than stopping Herdr.
No implementation or regression test has yet been written or run.
