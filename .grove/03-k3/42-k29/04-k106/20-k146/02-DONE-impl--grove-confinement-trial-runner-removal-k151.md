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

## Decisions (running log)

- The standalone mode stays, as the handoff states. It loses only its
  confined parts. No case uses it after the two confined cases go, and the
  fake-harness tests keep it working.
- The access probes go with the wrapper options `--probe` and
  `--probes-only`. Only the two confined cases set `probes`, and the probes
  measured the confinement profile. The record loses `contrast` and each
  launch loses `probes`.
- `TRIAL_OWNED` and its exemption in the control go. Only the confined
  set-up gave those names.
- The Codex record loses `confinement`. It names the sandbox mode, as the
  delivery specification states.
- A standalone launch keeps its record on standard output, `-`, in Grove's
  run log. That path works without confinement and stays tested.
- `reextract` of a historical confined run keeps its JSON probes, and a
  new `record.md` of that run omits them. The committed record does not
  change.
