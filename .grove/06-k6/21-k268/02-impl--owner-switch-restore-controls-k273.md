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
