# owner-switch-restore-k268 — brief

## Goal

Add the simplified serial `switch` and its full `restore --backup` to the
switch program, beside the old run. Include the backup, the attempt note,
the receipt, the manual entry and the drift refusal.

## Context

- `docs/specs/owner-skill-delivery.md` owns the contract in "The switch
  inside finish", "The hooks installation inside finish", "Claude Code's own
  writes", "Concurrent writers", "Resuming the switch" and "Rollback".
  "Test seams" lists the controls.
- This leaf is the expand step. owner-switch-status-k269 moves `mac-check`
  and purge reporting to the new entries. owner-switch-contraction-k270 then
  removes the old run, inverse, merge, pending and state entries. Until then,
  both paths work.
- Reuse these functions of `scripts/owner_switch.py`: source acceptance,
  the Grove release check, resolution and the probe calls. Also reuse the
  `mac-check` call and the identity functions. Do not reuse the exchange,
  held links, kept copies or cell inverse.
- The new path reads the current fixture plan and ignores its recovery-only
  fields. It needs plan fields that the old plan lacks. These are the whole
  write set with metadata and absence, expected inventories, command-input
  identities, delivery assertions and restore destinations. Add them to the
  fixture plan.
  The contraction changes the draft generator.

## Done when

- `switch` takes the steps of "The switch inside finish" in order. It
  creates and verifies one complete backup before its first write. That
  backup precedes the hooks installation.
- The backup holds content, absence, mode, ACL, extended attributes and link
  text of each planned file and entry. It also holds the plan and
  command-input identities, source acceptance, reader evidence and attempt
  identity. It holds no program copy. An incomplete backup allows no write.
- The hooks installation runs Claude Code's commands serially, reads the
  settings again after each command, and refuses metadata that a command
  would lose. It uses no held link and no exchange.
- Any caught failure after backup publication restores and exits
  unsuccessfully. `restore --backup` restores whole files and entries,
  including absence. It enforces Ravel's off value at completion.
- Restore verifies the backup against its attempt note and HOME, and refuses
  a mismatch without a write. A later restore refuses before its first write
  when any destination differs from the receipt's post-switch identity.
- The attempt note is immutable, outside `.grove/` and discovery roots. It
  names the attempt, HOME, plan and tool identities, backup path and literal
  restore command. It records no progress.
- The receipt keeps delivery assertions apart from whole-file restore
  identities. It lists retained resources with their purge conditions. The
  manual receives appended entries only, and recovery appends a correction.
- Fixture controls cover each new case of "Test seams" for backup, failures,
  hard interruption, restore, drift, the manual and the hooks commands.
  The existing controls of the old path still pass.

## Notes

This leaf proves bigger than one session if the controls are slow to write.
Then decompose it. Keep the switch and its restore in the first child,
because neither is useful alone.

Probe-owned service cleanup is owner-switch-probes-k257's work. Give the
switch a seam that receives ownership evidence from a probe result.

## Decomposition

The controls proved slow to write, so this leaf became a node on
2026-11-08. owner-switch-serial-restore-k272 delivers the switch, its
restore and the core controls. owner-switch-restore-controls-k273
delivers the remaining controls of "Test seams". k272's running log lists
them.
