# viewer-terminal-k11


## Goal
Harden terminal lifetime and prove cleanup with live PTY observations.



## Context
The parent brief owns the full exceptional-exit contract. The previous child
completes application interaction; extend the existing production terminal owner.

## Done when
Satisfy the parent's cleanup, actual-binary PTY, and test-only fault-child bullets
in full: partial setup, input/draw errors, actual unwinding panic, q, Ctrl-c,
SIGTERM, navigation/resize, termios equality and captured cursor/screen cleanup
before diagnostics, bounded waits and child reaping, separate closed-master case.
No shipped fault flag. Record commands and observations here. Update accurate
usage/changelog and affected books if their owned source changes. Run the
parent's required validation and close the parent against its full contract.

## Notes
The current Session restores on returned errors and Drop, but this is not yet
evidence for panic diagnostic ordering or handled termination. Timed event
polling must permit signal handling without doing terminal I/O in raw handlers.
