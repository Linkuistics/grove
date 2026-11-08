# owner-switch-contraction-k270

## Goal

Remove the old switch machinery and shrink the plan format to the
simplified design. Regenerate the plan draft and reconcile every
description of the removed mechanisms.

## Context

- The parent brief's artifact consequences, and the sections "The switch
  inside finish" and "The rehearsed plan" of
  `docs/specs/owner-skill-delivery.md`, name the removals.
- owner-switch-restore-k268 and owner-switch-status-k269 deliver the
  replacements, and the old path still works beside them.
- The old machinery in `scripts/owner_switch.py` covers the exchange, kept
  and flagged copies, held links, the merge entry, the cell inverse,
  pending-activation records and partial continuations. It also covers
  program copies in backups, run-state detection, rehearsal admission and
  the old `run`, `inverse`, `merge`, `pending`, `state` and `purge-check`
  entries.
- `docs/evaluations/owner-skill-delivery/switch_plan_draft.py` writes
  `docs/owner-cutover/switch-plan-draft.json` and its part table.
  `task owner:switch-plan:check` checks the draft in a disposable HOME.

## Done when

- The program holds the plan report, `native-off`, `switch`, `restore`,
  status and the purge report, with their helpers. No code for a removed
  mechanism remains. The fixture suite holds no control for one.
- The plan holds the whole write set with metadata and absence, expected
  inventories, source acceptance and reader evidence. It also holds commands
  and their input identities, probes, delivery assertions and restore
  destinations. It holds no cell or part recovery field and no rehearsal
  token.
- The draft generator writes that plan from the real HOME, reading only. It
  identifies each command script, local helper and input by content. The
  regenerated draft passes `task owner:switch-plan:check`.
- Task descriptions, docstrings and the evidence document describe only the
  current program. Historical measurements keep their observations and state
  their scope.
- `task owner:switch:rehearse` and `bash scripts/check.sh` pass. A search
  for each removed entry name, with a positive control, finds only history.

## Notes

The real HOME has no prepared program or backup, so no existing backup
format needs a reader. Record that fact in the evidence document.

The fixture suite's name still says "rehearse". The dry run of
owner-switch-dry-run-k271 replaces the old rehearsal of the real plan. Rename
the task only if both leaves agree on the new name.
