# dispatch-ending-k17

## Goal

The driver hands the end of a session to dispatch and decides from what dispatch
reports:

- **The launch.** The driver runs `harness-dispatch run` in the working-tree
  root with `--exit-dir` (the launch directory) and `--ending-file` (a file in
  it), beside the kind, the task file, the task ID and the prompt. Its runner
  launch carries no channel and no escalation of its own.
- **Cancellation.** It forwards its own TERM or HUP to dispatch and waits up to
  10 s before killing dispatch's group.
- **The terminal.** Once dispatch is reaped, the runner takes the terminal back
  from whichever group holds it and restores it: dispatch's, or, when dispatch
  died, the orphaned harness's. The driver then resets it.
- **Reading the launch.** After the reap and the epoch's invalidation, the
  first match wins:
  - the driver's own TERM or HUP → interrupted;
  - a teardown record → finished;
  - an `exit_signal` ending in the ending file → relaunch;
  - anything else → stopped, with the leaf live. That covers the harness's own
    exit, a cancellation of dispatch, a refusal, dispatch's death, a
    supervision failure, and a missing or unreadable ending file. After
    dispatch's death the stop says that the harness may have survived.
- **The prompt's signalling contract** tells every session to end with
  `harness-dispatch exit` once its task is retired and committed, and a finish
  session to run `grove-llm record-teardown` before it.

## Context

- The spec's *The process and terminal chain* and *Ending a session* under
  *Grove integration*, with its reading table. Decision 9 covers the prompt's
  three parts and the signalling contract's own gap.
  `harness-wrapper-k2`'s W8 and W12.
- The argv builder and the loop body are in
  `crates/grove-loop/src/loop_driver.rs`, and the prompt is in
  `crates/grove-loop/src/prompt.rs`. Their suites are
  `crates/grove/tests/loop_driver.rs` (a PTY with the real front and worker),
  `crates/grove-loop/tests/prompt.rs` and `crates/grove-tui/tests/witnessed.rs`.

## Done when

- Seam 3's endings:
  - the exit signal relaunches;
  - a teardown record then the exit signal finishes;
  - the harness exiting on its own stops with the leaf live.
- Dispatch killed while a raw-mode harness holds the terminal stops with the
  leaf live. While that orphaned harness still lives, the driver's group holds
  the foreground again with the handed-over modes restored.
- A driver started in the background takes no foreground.
- The driver's own TERM interrupts the loop through dispatch, with the
  harness's group reaped and the terminal restored.
- A stale session's exit signal cannot end a later launch.
- A refused kind still leaves its leaf live with the diagnostic on the
  terminal.
- The creator cases pass with fake sessions that end through
  `harness-dispatch exit`.
- The prompt's tests pin the new contract.
- The `grove-loop` book follows (P2), and `## Unreleased` records how a
  session ends its run.
- `bash scripts/check.sh` passes.

## Notes

- `grove-llm complete` still exists after this leaf, but no driver reads it.
  `signal-contract` removes it with the skills that name it, because
  `instructed_verbs` ties the two together.
- Grove still allocates no exit channel. Dispatch allocates it in the launch
  directory, which W3 chose because a session's own sandbox already writes
  under `.jj/grove/`.
