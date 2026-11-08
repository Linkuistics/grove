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

## Decisions (running log)

- The plan format becomes schema `owner-switch-plan/2`, with no `switch`
  section. Its fields are top-level: `writes`, `restore`, `inventory`,
  `inputs`, `assertions`, `native_assertions`, `tools` and `retained`,
  beside `state`, `manual`, `baseline`, `gates`, `accepted`, `probes`,
  `native_probe` and `mac_check`. `parts` and `outside` go. The real HOME
  holds no prepared plan, so no reader of schema 1 stays.
- Each item of `writes` states its switch action. An `entry` item is a
  deactivation. A `file` item may hold `set`, a list of `{"key",
  "value"}` pairs that the switch writes. The Ravel settings need a `set`
  of `skills`. No item holds an earlier value or an entry identity.
- The read-only plan report is the entry `report`. It replaces `run
  --dry-run`. It lists each write with its planned original and current
  identity, each value of `set` with its current value, the restore order,
  the command inputs and the commands.
- The fixture suite stops its rerun of `settings-writes.sh` and the
  option `--skip-settings-writes`. The rerun only produced the admission
  record, and its interval measured the race of the held link. Both
  mechanisms go.
- The gate, baseline, plan-review and process controls that ran the old
  `run` move to `switch`. The controls of the exchange, held links, kept
  copies, merge, inverse, pending records, run state and the preludes go.
- The draft writes `switch-plan-writes.tsv` in place of the part table, and
  the check writes `report.txt` in place of `dry-run.txt`. The draft runs
  no tool: it reads Claude Code's held version from the canonical path of
  the installed executable, and the original driver's Ravel keg from the
  canonical path of `ravel`. A prepared input that does not exist yet
  carries the digest of its repository source. The probe program has no
  source yet, so the inputs do not name it until owner-switch-probes-k257.
- The draft's `mac_check` is `mac-check --only "Owner skill switch"`, and
  its native Ravel assertion accepts the off value or an absent key, as
  owner-switch-status-k269 found that the real plan needs.
- Incident, 2026-11-08: the first run of the new check chose a Codex entry
  that is an absolute link into the real `~/.agents/.grove`. Its
  changed-entry control appended one newline through the link to a real
  snapshot file, then wrote the saved bytes back. The change identity of
  `~/.agents/.grove` afterwards equals the value recorded before the
  session (`b14c7deb…`), so the content is unchanged. The file's
  modification time changed. The control now chooses only a real folder
  inside the disposable HOME and stops if the file leads outside it. A
  second run left that modification time unchanged.
- The fixture task keeps the name `owner:switch:rehearse`. The leaf's note
  allows a rename only when owner-switch-dry-run-k271 agrees, and that leaf
  has not run.
- Seven of eight program mutants failed their controls at once. The mutant
  that writes the Ravel settings first survived every switch control, so
  `serial-switch-and-later-restore` now checks the order of step 6's writes.
  That mutant then failed it.
- Verification: `task owner:switch:rehearse` passed all 142 controls on the
  final bytes, with unchanged subject digests. `task owner:switch-plan:check`
  passed on the regenerated draft. `bash scripts/check.sh` passed its 18
  principal checks after three historical links to the replaced draft files
  became plain names. The real `mac-check` reports that the switch is not
  prepared. The section "Owner switch contraction" of
  `docs/owner-skill-delivery-evidence.md` records the details.
- Retirement: no ADR changes, because the specification owns these rules.
  The controls and mutants are executable, so this leaf cuts no review and
  spent no in-session reviewer. No review names this leaf. The parent node
  keeps live leaves, so no node closes.
