# claude-confined-rerun-k134

## Goal

Run `discovery-confined` and `missing-kind-confined` once more in Claude Code,
with Ravel's sign-in handoff.

## Context

- The record sections "Confined harness repair", "discovery-confined,
  repaired" and "missing-kind-confined, repaired".
- The build of ravel-token-handoff-k133, and the trial sandbox settings that
  declare the handoff.
- `scripts/harness_trial.py` and the trial wrapper, with
  `CLAUDE_CODE_TMPDIR` from confined-harness-trials-k128.

## Done when

- The runner declares the handoff in the trial's sandbox settings only. The
  authority control admits the handoff's name with the trial's own value.
- `discovery-confined` runs once in Claude Code. The record states the
  required reads, the denials and the ending, as for "discovery-confined in
  Codex, repaired".
- `missing-kind-confined` runs once in Claude Code. Its record states the
  result under the decision of standalone-default-kind-k131: the agent part
  is an explicit gap, and the probes show the sandbox part.
- A launch that still cannot sign in is a gap, with its exact cause.

## Notes

An unconfined retry gives no credit to a confined row. Do not change the
owner's settings.
