# owner-switch-model-removal-k265

## Goal

Remove the Quint model of the old switch recovery, its task and its run
script. Reconcile each reference to them, and keep the earlier measurements
as history.

## Context

- The section "The recovery model" of `docs/specs/owner-skill-delivery.md`
  states the reason. The model describes cells, exchanges, outside writes
  and partial continuations that the simplified design removes. Its success
  is not evidence for the simplified switch. The design adds no replacement
  model.
- The subjects are `docs/models/owner-switch.qnt`,
  `docs/models/run-owner-switch.sh` and `task model:owner-switch` in
  `Taskfile.yml`. `scripts/check.sh` does not run them, and no fixture
  control of `scripts/owner_switch_rehearsal.py` depends on them.
- `scripts/owner_switch.py` and `docs/owner-skill-delivery-evidence.md`
  name the model. Search the claim, including `.grove/` with hidden files,
  not a file list.

## Done when

- The model, its run script and its task are gone. No other model or task
  in `docs/models/` or `Taskfile.yml` changes.
- No current document, docstring or task description presents the model or
  its claims as acceptance evidence. Historical evidence keeps its
  observations and states their scope.
- A positive control shows that the search finds a known historical mention.
  The evidence document names the search and its result.
- `bash scripts/check.sh` passes.

## Notes

Live task files and briefs may still name the model as the old rule. Leave
retired task files unchanged. Report a live one that still requires the model.
