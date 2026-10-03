# supervised-run-k7 — brief

## Goal

`harness-dispatch run` spawns the harness it selected as its own child and
supervises it to the end, for every caller (D3, the dispatch half of D4, and
D7). Supervision covers the harness's job and terminal, a fresh exit channel and
`harness-dispatch exit`, the escalation, cancellation, and the harness's group
ending with the run. Dispatch then records and reports how the run ended.
Grove's current launch keeps working above it, unchanged in shape until
`launch-cutover`.

## Done when

- Seam 1's supervision cases and seam 2's end-observation cases pass through
  dispatch's command. The spec's *Agreed test seams* row lists them.
- `keyed-launch`'s own suite covers the runner contract that decision 7 states,
  apart from what later subtrees own: removing the token (`launch-cutover`) and
  the confined launch writing to the launcher's own output (`confined-run`).
- Grove's launch-boundary suite still passes, with dispatch supervising
  beneath Grove's own channel.
- `bash scripts/check.sh` passes at every leaf's commit.

## Decomposition

1. `runner-job` covers the runner's job contract both supervisors share. The
   child's group ends with every launch and is confirmed gone. The terminal's
   modes come back. The terminal is taken back after a death by signal. Entry
   signal handling is fixed, and cancellation gains its modes. Grove's own
   launches gain all of it at once, which makes this a working increment rather
   than a layer.
2. `dispatch-supervises`: `run` spawns instead of exec'ing, publishes a fresh
   exit channel, and `exit` sends the exit signal. The escalation, cancellation
   past the linearization point and the exit status follow. The runner gains
   the options that are new with this caller: the transparent caller's entry
   signal state, a channel whose appearance is the whole signal, and granted
   variables.
3. `run-ending` covers the end observation in the store and the same document
   in `--ending-file`.

The order is the order of dependence. Dispatch supervises through the runner
contract (1), and the ending is computed by the supervision it reports (2).

## Pointers

- Spec `docs/specs/harness-selection-and-execution.md`: *Execution and
  authority* (from the linearization point on), *Supervision* (the exit
  signal, the escalation, cancellation, the group ending with the run, the run
  ending, the ending file, when dispatch dies), *Records and later
  observations* (the end observation, migration), and *Diagnostics and exits*.
- Decision 7 of `docs/specs/module-decomposition.md` (the runner).
- ADRs: `docs/adr/dispatch-supervises-the-harness.md`,
  `docs/adr/the-launched-child-is-a-job.md`,
  `docs/adr/policy-evaluation-precedes-the-launch.md`.
- `harness-wrapper-k5`'s I1 (the group ends with the run), I3 (the terminal
  after a death by signal), I5 (pending signals) and I6 (sixteen measured
  findings: macOS `WNOWAIT` reports stops, the repeated SIGKILL, only ESRCH
  confirms, an ignored SIGCHLD). `harness-wrapper-k2`'s W1–W9 give the reasons.

## Notes

- **The transitional window is expected.** Until `launch-cutover`, Grove still
  allocates `GROVE_SIGNAL_FILE` and escalates dispatch's group when it appears.
  Dispatch then cancels its harness, so a Grove-driven run recorded in this
  window ends `cancelled`. Nothing in that window is released (P1), and the
  launch-boundary suite only needs to stay green through it.
- **From the moment dispatch supervises, Grove's kill-grace must outlast
  dispatch's end.** That end is dispatch's kill-grace, the group confirmation
  and the record-store lock wait: 10 s (W7, I1). Otherwise Grove's SIGKILL
  races dispatch's escalation and orphans the harness. `dispatch-supervises`
  raises the driver's kill-grace with it.
- `supervised-run` (`review-impl`), cut beside this node, reviews the whole
  node. The session whose retirement closes this node names its run on it
  (`references/retire.md`).
