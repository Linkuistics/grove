# owner-switch-preludes-k240

## Goal

Add step 3's gate and its two preludes to the switch program: the native
settings and the hooks installation. Extend the rehearsal task with their
controls.

## Context

- The node brief holds the contract. owner-switch-core-k239 built the program
  and the rehearsal task that this slice extends.
- `docs/specs/owner-skill-delivery.md`: "The native settings inside finish",
  "The hooks installation inside finish", "Claude Code's own writes" and the
  prelude controls of "Test seams".
- `docs/evaluations/hooks-installation-writes/settings-writes.sh` measures
  Claude Code's write method.

## Done when

- Step 3 erases each held link whose inode is still at the path. It runs the
  preludes only while no run is open, and only when a passing rehearsal
  record names the program.
- The native settings create an absent file by an exclusive rename, or add
  the off value by a settings record and an exchange, or stop.
- The hooks installation runs each Claude Code command under a held link, in
  the six steps of "Claude Code's own writes". It disables the plugin by the
  exchange, and it stops in every other state.
- Each run reports a flagged copy of a settings record, and step 8 stops
  while one is flagged.
- The rehearsal task runs each prelude control of "Test seams" with stub
  Claude Code commands. Its record names the Claude Code version of a rerun
  of `settings-writes.sh` when one runs, and counts the commands of each run.
  `task model:owner-switch` stays green.

## Notes

owner-switch-core-k239 left these seams for this slice. Each prelude write
is a write interval, so the program's process gate runs before it. The
kept-copy scan reads `<state>/backups/*/stage/*.record.json`, and a settings
record needs its own scan beside the backups. The rehearsal hook runs at
named points, so each prelude write needs its own points for the controls.

## Decisions (running log)

**Base.** On 2026-11-07 this checkout was at `e6839fc88a1`, the commit of
owner-switch-core-k239.

**Prelude constants.** The paths and values of the preludes are constants
of the program, not fields of the plan. A prelude needs a rehearsal record
that names the program only, so an unrehearsed plan must not steer a
prelude write. The constants are `~/.config/ravel/settings.json` with the
off value `{"schemaVersion": 1, "sources": []}`, and
`technical-communication-hooks@writing-hooks` from the marketplace at
`$(brew --prefix)/opt/grove`. The release formula test names that path.

**Commands.** The program finds `claude` and `brew` on PATH. Each runs in
HOME, with HOME set to HOME and without Claude Code's five location
variables. The rehearsal puts stub commands first on PATH and sets
`CLAUDE_CONFIG_DIR`, which the stub refuses. No override variable exists
for a real run to honour.

**List shapes.** A scratch-HOME measurement with Claude Code 2.1.319 gave
the rows of `claude plugin list --json` ("id", "scope", "enabled") and of
`claude plugin marketplace list --json` ("name", "source", "path"). The
marketplace row keeps the path as registered, not resolved through `opt`.
The plugin's `enabledPlugins` cell decides whether it is enabled. A plugin
that is absent while its cell holds `false` reads as absent, so an outside
uninstallation that leaves the cell still installs again.

**Settings records.** Each record is a directory in `<state>/settings`. It
holds `record.json` and the file `copy`, and it is complete when it is
renamed from its temporary name. To create an absent Ravel settings file,
the program stages the file in that directory. It then renames the file
exclusively onto the path, so no temporary file appears beside the settings.

**Held links.** Step 3 erases a held link when its inode is still at the
path. It also erases a record whose link is gone and an empty temporary
record. A held record holds a kept copy only after another inode replaced
it at the path. The flag rule therefore skips a link whose inode is live.

**Rehearsal of the preludes.** A program without a passing record that
names it stops at step 3 before any prelude. The fixture layouts after the
preludes stay the default, so the core controls keep their meaning. The
plan now lists the hooks plugin's cache directory as an entry outside the
switch set. The stub installation must therefore reproduce the row that the
plan records.

**The rerun's criterion.** `--settings-writes` (`SETTINGS_WRITES=1`) reruns
`settings-writes.sh` with the installed `claude`. The rerun passes on two
conditions:

- `marketplace add` and `plugin install` each rename one file onto the path.
- The measured interval of each command meets 30 to 38 ms.

The interval lies between the largest lead that lost the outside write and
the smallest lead that kept it.

**Mutants.** Twelve mutants of the program each lacked one mechanism of this
slice, and each failed its guarding control. The mechanisms were these:

- the erasure of held links, the held-copy comparison and the inode check
- the native comparison and the native process gate
- the open-run gate of the preludes and the prelude rehearsal condition
- item 2, the state reading under the link and the disabling
- the five location variables and the scan of settings records

**The rerun's shell and exit status.** The rehearsal runs
`settings-writes.sh` through its own shebang. The system's bash 3.2 expands
braces inside a quoted command substitution and breaks two jq filters, and
the first full run showed it. The criterion reads the measured sections and
records the script's exit status beside them. The script's last section
exits 1 here, because codebase-memory-mcp refuses to start in a scratch
HOME while this account's daemon serves another cache directory. That
section measures the server, not the write method.

**A repair of k239.** `bash scripts/check.sh` failed one test at the base:
the sweep of `GROVE_*` names found the fixture constant `GROVE_PLUGIN` that
owner-switch-core-k239 added to the rehearsal script. This slice renamed it
`SUPERSEDED_PLUGIN`, and the sweep then passed.

**Verification.** On the final program `ae2a56cedebc` and rehearsal script
`d42655c6b7ad`, `task owner:switch:rehearse` passed 151 of 151 controls with
Python 3.15.1. Then `bash scripts/check.sh` passed all 18 principal checks.
The digests of both scripts and the Taskfile were the same before and after.

An earlier full run with `SETTINGS_WRITES=1` passed 151 of 151 controls and
the rerun. The rerun reads no file of the program. It used Claude Code
2.1.319, and each command renamed one file onto the path. The measured
intervals were 35.1 to 38.5 ms for `marketplace add` and 35.3 to 37.1 ms for
`plugin install`. Nine prelude controls also passed with the system Python
3.9.6.

**Model.** `mise exec -- task model:owner-switch` exited 0 with "all claims
hold". The model digest `c90bbf9ab658` and the runner digest `0fe7ef316e63`
were the same before and after the run. This slice changed no recovery rule.
