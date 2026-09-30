# signal-transparent-handoff-k29

## Goal

Make the handoff a linearization point and make it transparent to signal state.
Cancellation observed before the final check launches nothing and marks the
committed attempt not executed. The harness receives the entry signal mask and
every disposition that survives exec, exactly as the front process inherited
them, including SIGPIPE.

## Context

The spec's `#execution-contract` spells out the order: commit, block the handled
signals, check pending cancellation, then append-and-re-raise or restore-and-exec.
It also states the Rust runtime facts. The standard runtime ignores SIGPIPE
before `main`. Rust's exec path resets SIGPIPE to default before pre-exec
hooks, while keeping the thread's mask. So record the entry SIGPIPE
disposition before runtime initialization changes it, and reinstate it after
that reset. The leaf chooses the mechanism, verifies it against the Rust
source for the workspace's minimum toolchain, and records it in its running log.

## Done when

- After the commit, the handled signals are blocked and pending cancellation is
  checked once more. On cancellation nothing launches, a not-executed detail is
  appended where possible, and the process re-raises. A failed append leaves
  the attempt unknown, never a success.
- Otherwise the entry mask and dispositions are restored and the process execs.
  A caller that ignored SIGPIPE or HUP, or blocked a signal, has that state
  observed unchanged by the fake harness. A caller with default SIGPIPE has
  default observed. Positive control: a fixture that deliberately alters the
  state is seen to change what the harness observes.
- Command-seam tests cover cancellation between commit and exec. They show
  `record show` reporting the attempt not executed, with no harness marker.
- The usage documentation states the one window that remains: a signal between
  restoring the mask and exec can leave the attempt recorded with execution
  unknown.
