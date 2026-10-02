# owner-settings-k9

## Goal

An owner sets the selection bound, the context budget, the record directory and
the policy's grants in one settings file with no flag passed. The bound's
ceiling admits minutes, and a policy can start a deciding agent and use its
answer.

## Context

- `docs/specs/harness-selection-and-execution.md`: *Owner settings*, the bounds
  table in *Bounded context*, *Dynamic dispatch*, and the command row of the
  test seams.
- Root brief, requirements 7 and 8.

## Done when

- Every command reads the settings file before any worker starts, as *Owner
  settings* states. A flag replaces a setting, `--policy-env` adds to the
  grants, a malformed file refuses, and inspection reports where each value
  came from.
- The whole-selection bound has a 600-second ceiling and keeps its 30-second
  default.
- At the command seam, a scripted stand-in started by a test policy receives
  the prompt in the caller's directory and its answer decides which command
  launches. A stand-in still running at the bound launches nothing.
- The installed-layout smoke test runs a policy that starts a child and uses
  its answer.
- The dispatch README documents the settings file. The README and the SDK's
  description of `prompt` say that keeping a deciding agent from executing the
  task is the policy owner's job.
- `bash scripts/check.sh` passes.

## Notes

- No agent policy and no model-calling policy ships, and no test calls a model.
- The settings are the same for every kind.
