# dispatched-terminal-k33

## Goal

Prove under a controlling PTY that a dispatched session behaves as Grove's
foreground job. Identity, terminal, cwd, native exits and signal state reach
the harness intact. Cancellation during selection or execution follows Grove's
job contract.

## Context

The spec's controlling-PTY row lists the cases. Grove's existing PTY-driven loop
tests show how the foreground job, grace, TERM and KILL escalation are
observed. The signal semantics under test were built in
`evaluation-boundary-k27`. This leaf observes them from Grove's side.

## Done when

- The dispatched fake harness reports the PID and process group Grove launched,
  the intended cwd, and the controlling terminal as its foreground group. Its
  native exit code and a native signal death reach Grove unmodified.
- The fake harness observes Grove's entry signal mask and dispositions,
  including SIGPIPE, unchanged.
- The policy worker observes null stdin and a scrubbed control environment.
  The final harness observes the fresh completion channel.
- Interrupting through the PTY during a deliberately slow selection launches
  nothing and leaves no worker. Grove's response to the ended child is the
  existing one. Interrupting during execution, and Grove's descendant
  escalation, behave as they do for a direct harness.
- Positive controls: each observation is seen to change when the fixture
  deliberately alters it, so that no case can pass without being exercised.
