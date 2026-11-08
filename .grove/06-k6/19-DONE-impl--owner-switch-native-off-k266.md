# owner-switch-native-off-k266

## Goal

Add the `native-off` operation to the switch program. It writes only
Ravel's off value, needs no switch backup, and survives interruption.

## Context

- The section "The native settings inside finish" of
  `docs/specs/owner-skill-delivery.md` owns the contract. Finish runs this
  operation immediately after promotion, before source acceptance or any
  new default launch. The owner runs the same operation after a crash or a
  stop before backup creation.
- The program is `scripts/owner_switch.py`. Its fixture seam is
  `scripts/owner_switch_rehearsal.py`, which `task owner:switch:rehearse`
  runs. The old prelude `native_settings` writes the off value through an
  exchange under a settings record. Leave it in place: the contraction
  owner-switch-contraction-k270 removes it.
- The section "Test seams" lists the `native-off` controls.

## Done when

- `native-off` parses the current Ravel settings and keeps every unrelated
  key and its metadata. An absent file becomes a file with the off value.
  An absent `skills` key gains it. An existing off value stays unchanged.
- Any other `skills` value or a malformed file refuses without a write. So
  does another running Ravel driver.
- The operation publishes the file through a complete, synchronised
  temporary sibling and an atomic replacement. It never truncates the live
  file.
- Fixture controls cover each case of "Test seams". They interrupt creation
  of the temporary file and the replacement. The live file stays original or
  completely published, and a rerun of the same command completes it.
- A write failure reports incomplete native delivery and exits
  unsuccessfully.
- A control with the accepted 2.0 build shows that the result resolves as
  `disabled`.
- The program states the operation's exact command form for the prepared
  recovery instructions. The evidence document records each control's
  result and the persistence measurement on the HOME volume.

## Notes

This operation is not an exchange and has no displaced-copy recovery. It
runs no harness and captures no source. Keep the existing `run`, `inverse`
and other entries working.

## Decisions (running log)

- The entry is `native-off [--home HOME] [--command]`. It reads no plan, so
  a recovery after a crash needs only the program and HOME. `--command`
  prints the exact command, executable, program, HOME and settings path as
  one JSON object and writes nothing.
- Exit 0: the off value is in place, written or unchanged. Exit 3: a
  refusal, with no write to the settings. Exit 4: a write failed, and native
  delivery is incomplete. The message names the rerun command.
- A repeated member name or a non-finite number is a malformed file. The
  accepted 2.0 build refuses repeated names as `settings_invalid` too.
- The recogniser of another Ravel driver compares the basename of the
  executable path (`proc_pidpath`) and of argv[0] with `ravel`. Only this
  process and its exact ancestors are exempt. owner-switch-process-inspection-k267
  may fold this check into the narrow recogniser.
- Fixture seam: `OWNER_SWITCH_REHEARSAL_SCOPE` limits recognition to
  executables inside one folder. The program refuses it, like the hook, when
  HOME is the account's home directory. The machine had an orphaned Ravel
  1.1.0 process (PPID 1) during this session, which a machine-wide fixture
  check would always see.
- The temporary sibling gets the original's mode, owner, flags, ACL and
  extended attributes by `fcopyfile`, and the current time as its
  modification time. The comparison skips `com.apple.provenance`, as the
  hooks installation does. A different owner refuses.
- Persistence: `F_FULLFSYNC` on the temporary file and on the folder after
  the rename, with `fsync` only where the filesystem refuses `F_FULLFSYNC`.
  The program prints which method each sync used.
- The control "after each preflight refusal" uses the current switch's stop
  before a backup. The later preflight of the simplified switch belongs to
  its own leaves.
