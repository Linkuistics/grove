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

## Decisions (running log)

- `docs/models/` held only the two subjects, so the directory went with
  them. Their final digests equal the last run of record in the evidence
  (`c90bbf9ab658…`, `0fe7ef316e63…`), so the evidence names that table
  rather than a commit hash, which a rebase would change.
- Historical sections in the evidence keep every observation, table and
  digest. Live links to the deleted files became plain paths, and a scope
  paragraph now opens the section. Later dated passages that mention the
  model's runs (the entries and reader-marker sections) stay as written.
- `known_cells` loses its citation of the model's merge. The merge code
  itself stays: removing it belongs to `owner-switch-contraction-k270`.
- Report: the closed node brief `01-k91/_owner-switch-program.md` still says
  the model must stay green. The node has no live child, so nothing requires
  the model. The brief stays unchanged as the node's record.
