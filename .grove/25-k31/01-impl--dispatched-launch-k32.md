# dispatched-launch-k32

## Goal

Prove at Grove's existing launch boundary that a lifecycle session routed
through `harness-dispatch` receives the right data and authority, and that
existing direct-harness routes are unaffected.

## Context

The slots come from `grove-task-slots-k11`. The run identity comes from
`handoff-records-k24` and the environment scrubbing from `ambient-authority-k30`.
Grove's launch-boundary integration suite builds temporary configurations with
fake harnesses. Extend it rather than creating a parallel harness. The suite
must obtain `harness-dispatch` and its worker by the same deterministic route
the skeleton chose, never by skipping.

## Done when

- A temporary personal Grove configuration routes a kind to `harness-dispatch
  run` with the `kind`, `task_file`, `task_id` and `prompt` slots. A temporary
  dispatch policy and state directory route it to a fake harness. A driven
  session shows the fake harness receiving the unchanged prompt, including
  spaces, quotes, shell punctuation and newlines, plus the authoritative kind,
  absolute task path and handle, each as one native argument.
- The fake harness receives `HARNESS_DISPATCH_RUN_ID`, matching a recorded run,
  and Grove's fresh completion channel, and can signal completion through it.
  A policy probe records its own environment, which lacks that channel.
- Task authoring (`grove-llm leaf-add`) succeeds under a valid wrapper whose
  dispatch policy lacks the kind. The next launch refuses with dispatch's
  incomplete-mapping diagnostic and the equivalent inspect invocation, and the
  leaf stays live for the next attempt.
- The existing direct-harness launch tests pass unchanged, and a direct and a
  dispatched kind coexist in one configuration.
- A literal `--choice` in the personal command definition reaches policy as the
  explicit choice.
