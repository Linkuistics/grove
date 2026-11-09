# staging-trial-runner-k275

## Goal

Build the scripted staging trial runner. Run the source trials and the
discovery case on Claude Code and Codex in the staging HOME.

## Context

- The node brief holds the whole contract. "Test seams" and "Cutover
  contract" of `docs/specs/owner-skill-delivery.md` own the trials.
- `docs/evaluations/owner-skill-delivery/staging_home.py` holds the staging
  launcher, its refusals and its working directory rule. Its `launch`
  passes `--config` to `ravel start-session`, so no trial writes the
  retained `config.rhai`.
- `scripts/harness_trial.py` holds the adapters, the route wrapper
  `scripts/harness-trials/trial-launch.sh`, event extraction and
  `resolve_offline`. Reuse them. Do not change the common-loading cases.
- `ravel skills resolve` reads only `~/.config/ravel/settings.json`. The
  retained staging settings stay unchanged. A disposable HOME carries any
  other settings value.

## Done when

- One script runs every staging trial, with HOME set to the staging HOME
  and `CODEX_HOME` set to its `.codex` tree. A Taskfile entry runs it.
- Source trials, with a control that is seen to fail for each:
  - The off value resolves as `disabled`.
  - With one candidate-keg source moved away, a launch refuses as
    `skill_source_missing` before any worker. The source returns, and its
    `resourceDigest` is unchanged.
  - A launch without a network receives the same directories as a launch
    with a network.
- The discovery case passes on each harness. The mandatory file is read
  first. A task that names no skill activates a relevant craft skill, and
  the agent reads it before the relevant action.
- Each model case records the native skill listing of its session. A
  superseded name fails the case.
- The evidence document has a section for this node. It records each trial,
  its command, its versions and its result.

## Notes

Later leaves add their cases to this runner, so give a case a small data
shape. A failure is a finding for the owner design or for a pack, not a
reason to weaken a case.

## Decisions (running log)

- The runner is `staging_trial.py`, beside `staging_home.py`. It reuses the
  staging launcher's rules and the adapters of `harness_trial.py`. The
  common-loading runner builds a new sandbox HOME for each run, so it cannot
  run in the retained staging HOME without a second mode.
- Each launch passes `--config` with a trial route. The retained
  `config.rhai` stays unchanged.
- The source trials use `/usr/bin/true` behind the route wrapper. The
  wrapper records the request view before the worker runs, so no model
  runs. Ravel then reports `session_result_missing`, which is expected.
- The missing-source trial moves the source's keg directory, its
  `originPath`. The link in the skill directory then dangles, as after a
  removed keg.
- The real `.claude.json` is not compared by digest. The owner's own
  running session rewrites it. The check instead requires that it names no
  staging path.
