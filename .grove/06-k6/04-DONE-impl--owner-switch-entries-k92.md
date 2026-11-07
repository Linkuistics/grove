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

owner-switch-program-k243 changed the plan loader, the rehearsal record and
the receipt. The parent brief's "Owner-delivery handoff" states the items
for this leaf.

## Decisions (running log)

**Base.** On 2026-11-07 this checkout was at `f20abe70d6d`, the commit of
owner-switch-program-k243. The working copy also held `.claude/settings.json`,
which this leaf did not write and does not commit.

**The owner's inverse.** The entry `inverse --plan PLAN` finds the latest
complete backup and runs that backup's program copy, as a continuation
does. It runs only while the run is open. It runs the inverse of
"Rollback", which lists the processes of step 1 first, and then the plan's
native probe. It appends one line to the maintained manual.

It ends with 0
only in the native state. A third value leaves the changed state, which is
a stop for classification.

**The merge.** The entry `merge --plan PLAN COPY` applies the difference
between the flagged copy and its last recorded content to the live file,
a removed key included. Each record therefore now keeps the bytes that its
reading read, beside its digests.

The merge record keeps the copy's digest
and its bytes, so a later merge takes its difference from them. The merge
writes the live file by a settings record and the exchange, so detection
covers its own write. A cell takes the copy's value only when that value is
neither a known earlier nor the switched value, as `ownerMerge` in the
model takes it. `--record-only` records a merge that the owner applied by
hand, such as for content that is not a JSON object. No recovery rule
changes, so the model stays unchanged.

**The purge check.** The entry `purge-check --plan PLAN` reads the
`retained` list of each receipt. Without a receipt, it reads the backup's
entry parts. It compares each retained entry with its recorded change
identity. It reports each flagged copy and each open run outside the
complete state. It lists the three conditions that only the owner can
establish, and it changes nothing.

**The pending-activation record.** A stopped `run` or `inverse` writes its
command evidence and its reason into `<state>/runs/`. The entry `pending
--plan PLAN [--grove-commit SHA]` writes the record from the latest such
log. It takes the Grove version and keg and the Ravel path and version from
step 4's evidence. It reads them itself when the run did not reach step 4.

It runs
`mise -C HOME bootstrap packages status --json` itself, because a failed
step 5 emits no result. Finish supplies the release commit, which no local
file holds. The receipt's entry names each earlier pending-activation
record as replaced, and the manual stays append-only.

**`mac-check`.** The owner's prepared plan and program are at
`<state>/prepared/plan.json` and `<state>/prepared/owner_switch.py`, with
`.local/state/owner-switch` as the real plan's `state`. `mac-check` runs
that program's `state --json` and maps the state to a fixed message. It
reports each flagged copy by the HOME-relative path of its live file.
Without a prepared plan and without a backup, it reports native delivery.
`state` of an open run runs the backup's program copy, as each
continuation does. The rehearsal runs the real `mac-check` command with
HOME set to a disposable HOME that links the checker's folder.

**The snapshot.** The first full rehearsal passed 215 of 216 controls.
`prelude-rehearsal-missing` failed, because a stop now writes its run log.
Without a rehearsal record, that log also creates the state folder, and the
snapshot counted the new folders as a change. The snapshot now skips the run
logs, the prepared copies, the checker's link and the folders above them.
The seven controls that compare snapshots passed again after that change.

**Mutants.** Ten mutants each lacked one mechanism, and each failed its
guarding control. Nine changed the program. They removed the merge's
removal of a key, the merge authority, the cell rule and the occupied path.
They also removed the program copy of the inverse, the mise status, the
replaced record, the identity of a retained entry and the run log.

One copy of `mac-check` ignored flagged copies.
It failed the three flagged controls and passed `mac-check-native` and
`mac-check-complete`, which are the positive controls.

**Verification.** On program `885171751f02`, `task owner:switch:rehearse`
passed 216 of 216 controls with Python 3.15.1. Its rerun of
`settings-writes.sh` passed with Claude Code 2.1.319, and the record passed.
Each subject file was last changed before that run started. `task
model:owner-switch` reported "all claims hold". The checker's tests passed
48 of 48, and Ruff and strict pyright reported nothing.

The real `mac-check --only "Owner skill switch"` reported native delivery,
because this machine holds no prepared switch. The section "Maintenance and Recovery"
of the manual now describes the switch row.

**A rename.** The first `bash scripts/check.sh` failed only the sweep of
`GROVE_*` names. It found the rehearsal constant `GROVE_ENTRY`. The
constants are now `CLAUDE_SKILL_ENTRY` and `CODEX_SKILL_LINK`. The rename
changed no mechanism and no program byte, so the mutants did not run again.
The 32 entry controls passed again on the renamed rehearsal script.

The second run of `bash scripts/check.sh` passed all 18 of its principal
steps, with 1,344 tests passed.

**Retirement.** No ADR changes: each entry applies a rule that the
specification already stated, and the specification now names the entries.
This producer's controls are executable, and ten mutants proved them, so it
cuts no review and spent no in-session reviewer. No review names this leaf,
so this session writes no Creator line. The parent node keeps live leaves, so no node
closes.
