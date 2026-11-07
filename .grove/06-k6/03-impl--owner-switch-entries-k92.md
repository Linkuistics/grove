# owner-switch-entries-k92

## Goal

Add the owner's entries to the switch program: the inverse after finish and
the record of a merge. Add the report of the switch state to `mac-check`.

## Context

- owner-switch-program-k91 delivers the program, its state reader and its
  rehearsal task. Extend them.
- The contract is the same set of sections in
  `docs/specs/owner-skill-delivery.md`. Items 2 and 3 of the k87 handoff, and
  items 3 and 4 of the k89 amendments, state this leaf's obligations.
- `mac-check` runs `~/.local/lib/machine-config/check.py`. The machine
  contract owns that file and its tracking.

## Done when

- The owner's inverse runs as an entry of the program copy in the latest
  backup. It renames an entry only onto an absent original path, whatever the
  entry's change identity. An occupied path is a third value: the inverse
  leaves both entries and reports the cell.
- The merge entry records the digest of the merged copy. That digest is then
  the only recorded content of the copy. The merge applies the difference
  from the last recorded content, a removed key included.
- The purge procedure checks each retained entry against its recorded change
  identity.
- `mac-check` reports the state of the switch set and each flagged kept copy.
  Before any switch, it reports the native state.
- The rehearsal task runs the controls of "Test seams" for the owner's
  inverse, merged copies, occupied original paths and the `mac-check` report.
  `task model:owner-switch` stays green.

## Notes

The `mac-check` change follows the machine configuration contract. It changes
no discovery. The pack-checkout status of `mac-check` belongs to
owner-pack-checkouts-k93.

owner-switch-gates-k241 hands this leaf the pending-activation record.
After a stop or an inverse, finish's continuation writes that record into
the maintained manual. Items 2 and 3 of the continuation after "The switch
inside finish" state its content. The receipt later replaces it.

The program reports the state and stops. Its command evidence holds the
output of each gate, such as the Grove version and keg of step 4 and the
`mise` status of step 5. Add an entry of the program that writes the record from that
evidence. Add its rehearsal controls beside those of `mac-check`.
