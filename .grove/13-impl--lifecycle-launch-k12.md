# lifecycle-launch-k12

## Goal

Bare `grove` launches every lifecycle session by running `harness-dispatch run`
itself. Nothing on the launch path and no tree verb reads Grove configuration.

## Context

- `docs/specs/harness-selection-and-execution.md`, *Grove integration*: *A
  lifecycle session* and *A refusal*; and the two Grove launch boundary rows of
  the test seams.
- Root brief, requirements 1, 3, 6 and 9.
- `direct-dispatch-k4`'s note *Where the design lands in the code*, the *Grove*
  entry.

## Done when

- The driver runs `harness-dispatch run` as the foreground job in the
  working-tree root, passing the kind, the task file, the handle, the mandate
  and the three parameters, and nothing else.
- Kind admission is gone from the driver, the tree verbs, root scaffolding and
  the finish sentinel. A refused launch leaves its leaf live and the loop
  stopped, and rerunning `grove` continues.
- Every Grove test that launches a session goes through the real front and its
  compiled worker, with the policy in a temporary HOME. There is no fake
  `harness-dispatch`.
- The Grove launch boundary cases hold, the controlling-terminal ones included.
  A `config.kdl` and a `.grove.kdl` on disk, valid, invalid or tracked, change
  nothing about a launch.
- The `grove-loop`, `grove-llm` and overview books are valid for what changed.
- `bash scripts/check.sh` passes.

## Notes

- This is the wide test migration. Most launching tests take their
  configuration from one shared fixture home, so start there.
- A test of what configuration does to a launch goes here, since it can no
  longer pass. A test of the configuration code itself stays until
  `grove-configuration-k13` deletes that code.
- After this leaf the `grove config` commands still exist and describe a
  configuration nothing launches from. That state is not released.
