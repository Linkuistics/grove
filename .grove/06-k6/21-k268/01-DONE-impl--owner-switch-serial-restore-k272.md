# owner-switch-serial-restore-k272

## Goal

Add the serial `switch`, its `attempt` preparation and its full
`restore --backup` to `scripts/owner_switch.py`, beside the old run. Add the
new plan fields to the fixture plan. Prove the program with the core fixture
controls.

## Context

- The node brief `_owner-switch-restore.md` states the contract, the reuse
  rules and the whole Done when. This leaf delivers the program and the
  core controls. owner-switch-restore-controls-k273 delivers the other
  controls of "Test seams".
- The old run, inverse, merge, pending and state entries stay unchanged.
  Their controls must still pass.

## Done when

- `attempt` writes the immutable attempt note. `switch` takes the steps of
  "The switch inside finish" in order, with one verified backup before its
  first write and before the hooks installation.
- `restore --backup` verifies the backup against its attempt note and HOME,
  restores whole files and entries with absence, and enforces the off
  value. A later restore refuses on drift from the receipt's post-switch
  identities.
- The receipt keeps delivery assertions apart from restore identities and
  lists retained resources. The manual receives appended entries only.
- The fixture plan holds the new fields. Core controls pass: the full switch
  and restore, a caught failure, and a hard interruption with explicit
  restore. They also cover a backup stopped before publication, an attempt
  mismatch and a later drift refusal. The old controls still pass.

## Notes

## Decisions (running log)

- The new entries are `attempt`, `switch`, `restore` and
  `restore-identity`. The program's docstring, section "The serial switch",
  states each step, exit status, hook point and plan field.
- Exit statuses: 3 refuses before any switch-set write. 4 reports
  incomplete recovery, and the backup stays. 5 reports a failed switch
  after a restore that verified native delivery.
- The plan carries a new `switch` section: `writes` with originals by
  restore identity or absence, `restore` order, `inventory` before and
  after, `inputs` by content, `assertions`, `tools` and optional
  `retained`. The switch reuses `parts` for deactivations and switched
  cells, and ignores `earlier` values and entry identities.
- The restore identity of a path is the digest of its nodes. It covers the
  mode, ACL, attributes and content of each file and folder, and each
  link's text. It leaves out `com.apple.provenance`, because macOS gives
  that attribute to each new file of the process.
- The backup mirrors each written entry with `fcopyfile` metadata, and its
  manifest records absences. A measurement on 2026-11-08 showed that
  `fcopyfile` copies a folder's ACL, attributes and mode too.
- Deactivation and restore move entries into `<state>/discard/<attempt>`
  before they erase them. No partial entry stays in a discovery root.
- The receipt is `<state>/receipts/<attempt>.json`. Invalidation renames it
  to `<attempt>.invalid.json`. A completed restore writes the outcome
  record `<state>/restored/<attempt>.json`. A later restore of that backup
  then passes with no write while the layout is unchanged. It refuses on
  any change since.
- No stop may happen inside `create_backup` after its publication rename.
  The flush and the hook point `backup:published` run in the restoring
  path. The first full run of the controls found this defect.
- An eligible later restore first writes a recovery note and backup. Its
  retry command adds `--recovery`. A retry accepts each path at the
  recovery backup's identity or the restored identity, and refuses others.
- Probe-owned services: a probe result `{"owned": [...]}` names them. A
  service that still runs after the probes stops with 4 and no restore.
  owner-switch-probes-k257 adds the measured shutdown route.
- The plan field `retained` lets the plan name old binaries, such as the
  original driver's Ravel keg, because the program cannot infer them.
- The 23 serial controls are `serial-*` in `serial_controls()`. They cover
  the full switch with a later restore and its recovery backup, a stop
  before publication and an attempt or HOME mismatch. They also cover a
  later drift refusal, refusals before any write, 11 caught failures and 7
  crashes with explicit restore.
- These cases of "Test seams" remain for owner-switch-restore-controls-k273:
  - An ACL and an attribute on entry files and folders, through backup and
    restore. A failed copy or metadata read requires no mutation.
  - An interrupted restore, repeated by the same command. The off-value
    exception with an absent settings file or key needs a constructed
    backup, because the switch refuses unless the off value is in place.
  - A simulated permission or I/O failure in restore: exit 4, the backup
    stays and the output names the failed paths.
  - A failed Claude Code command and an unexpected settings change, both
    through the stub. Also an ACL or attribute that appears after the
    preflight and stops before the command.
  - An interrupted eligible later restore, retried with `--recovery`.
  - Owner operation after finish: restore under another harness lineage.
  - An unwritable manual: configuration recovery succeeds with exit 4 and
    a pending manual entry.
  - A probe result that names a live owned process: exit 4, no restore.
  - A switched setting, an outside native entry or a registration that
    changes during the probes. The final comparison must fail, and no
    success receipt may remain.
  - A changed backup manifest or copy, which restore must refuse.
- The mutant without the note's backup-exists refusal first survived,
  because the changed originals refused later. The crash and failure
  controls now require the note's refusal text. The comparison of
  originals with their copies still has no control, which k273 owns.
- The final run passed all 288 controls on the final bytes. The section
  "Owner switch serial restore" of `docs/owner-skill-delivery-evidence.md`
  records the digests, the results and the mutation controls.
