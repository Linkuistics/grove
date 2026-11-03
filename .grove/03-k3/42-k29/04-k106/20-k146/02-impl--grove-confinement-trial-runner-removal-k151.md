# grove-confinement-trial-runner-removal-k151

## Goal

Remove the confined set-up from the harness trial runner and its two confined
cases.

## Context

- The trial runner items of the "Scripts and Taskfile" list of the handoff in
  `docs/confinement-withdrawal-evidence.md`.
- `scripts/harness_trial.py`, `scripts/test_harness_trial.py`,
  `scripts/harness-trials/` and `trial-launch.sh`.
- The runner sections "Conventions", "What one run does", the two adapters and
  "The controls" of `docs/common-loading-acceptance-record.md`.

## Done when

- The runner and its tests have no confined mode. Codex keeps
  `--sandbox workspace-write` on every route.
- The cases `discovery-confined` and `missing-kind-confined` are removed.
- `profile-copy.py` and `descriptor-probe.py` stay, as the handoff states.
- The runner sections of the acceptance record describe the runner.
- `task trial:controls` passes, and the change is a described commit.

## Notes

stage-evidence-reconciliation-k125 marks the confined results as historical.
Do not change the recorded results here.
