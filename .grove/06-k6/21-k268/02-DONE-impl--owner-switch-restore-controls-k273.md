# owner-switch-restore-controls-k273

## Goal

Complete the fixture controls of "Test seams" for the serial switch and its
restore, which owner-switch-serial-restore-k272 added.

## Context

- The node brief `_owner-switch-restore.md` states the contract and the
  whole Done when. k272's "Decisions (running log)" names the program's
  entries, exit statuses, hook points, plan fields and the controls that
  it delivered.
- The controls run in `scripts/owner_switch_rehearsal.py` through the
  existing disposable-HOME seam.

## Done when

- Controls cover each case of "Test seams" for backup, failures, hard
  interruption, restore, drift, the manual and the hooks commands that k272
  did not cover. k272's running log lists what remains.
- Each control passes, and the old controls still pass.

## Notes

If k272 leaves program behaviour unimplemented, implement it here before
its control.

## Decisions (running log)

- Four gaps of k272 need program changes before their controls:
  - `nodes_of` lets an `OSError` escape. An unreadable original before
    the backup then ends `switch` with a traceback and exit 1, not a
    refusal with exit 3. It now raises `Stop`.
  - `load_backup` trusts the manifest's items. A changed manifest with
    matching copies passed, and an item path outside the plan gave a
    `KeyError`. Each item must now match the plan's write set by path,
    type and `data/<n>`. For a switch backup, each item identity must equal
    the plan's original. The plan digest must equal the note's. The chain
    note, plan, manifest and copies then anchors at the immutable note.
  - A restore that ended with exit 4 for an unwritable manual wrote its
    `restored` record first. A repeated restore then reported "done" and
    lost the pending manual entry. The "done" mode now appends each
    missing entry, by its marker.
  - Restore recorded no operator. The `restored` record now names the
    admitted ancestors, and `restore --guidance` records each guidance
    file by digest and reads it again at the end.
- The 19 new controls are in `serial_controls()`, after k272's 23. A
  constructed backup for the off-value exception uses the program's own
  `create_backup` through `program_module`. The probe-owned control uses
  the fixture flag `probe_owned`. A copy that differs before publication
  uses the new hook action `append-glob`.
- `serial_failure` now takes hook actions, an expected text and a check.
  Each caught failure also compares every planned original by restore
  identity, so metadata counts, not only content and mode.
- A failing ACL read of an open descriptor has no fixture injection. A
  folder with mode 000 stands for the failed metadata read, and a file
  with mode 000 for the failed copy read. Both reach the same conversion.
- 24 mutants of the program each made their control fail. Two failed by
  the refusal text only: without the copy-name check, the copy check
  still refuses, and an ignored failed `claude plugin install` still ends
  with 5 through the later hooks state.
- The final run passed all 307 controls on the final bytes, and the 24
  mutants ran again on them. The section "Owner switch restore controls"
  of `docs/owner-skill-delivery-evidence.md` records the digests, the
  results, the mutants and the limits.
