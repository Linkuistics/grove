# review-k3

**Kind:** review

## Goal

Fresh-context adversarial review of driver-side-kill-k2 before it is
released: try to disprove that the watcher is correct and that the killer
removal left nothing dangling.

## Context

The change touches the loop's liveness core: a bug here either fails to kill
(the codex symptom again) or kills a session that didn't signal. Review runs
per the trial convention (K3 reviews everywhere when loop-driven).

## Done when

Findings triaged: real defects fixed in this leaf or externalized as new
leaves; the reviewer's pass leaves no CONFIRMED correctness finding open.
Specific angles: signal-file races (stale file at iteration start, file
appearing during teardown), zombie/reap correctness after SIGKILL, terminal
state after a killed TUI, docs/tests still referencing removed env handles.

## Notes
