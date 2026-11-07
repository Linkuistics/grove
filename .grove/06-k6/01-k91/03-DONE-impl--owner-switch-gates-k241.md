# owner-switch-gates-k241

## Goal

Implement the concrete checks of switch steps 4 to 7 as commands of the
program, so that a plan can name them. Extend the rehearsal task with their
source controls. Cut the review of the whole program.

## Context

- The node brief holds the contract. The earlier slices built the program
  and its preludes.
- `docs/specs/owner-skill-delivery.md`: steps 4 to 7 of "The switch inside
  finish", and the source controls of "Test seams".

## Done when

- Step 4 stops unless the installed Grove is the latest published release
  and its `opt` link reaches it. It compares the digest of each of Grove's
  six sources with its accepted digest, and it names each source whose
  staging trials must run again.
- Step 5 stops unless each skill formula is declared and installed at its
  accepted version.
- Step 6 stops unless the resolved source IDs are exactly the ten accepted
  IDs and each `resourceDigest` equals its accepted digest.
- Step 7 records the reader evidence and stops at an `.orphaned_at` marker.
- The rehearsal runs the controls of an eleventh source and of a one-byte
  difference, with fixture commands in place of `ravel`, `brew` and `mise`.

## Notes

The recovery mechanism has no executable test outside its rehearsal. After
this slice, cut one `review-impl` with the slug `owner-switch-program`
beside this node, as the last act of the session.

owner-switch-core-k239 left three items without an owner inside this node.
Decide each one in this slice, or hand it on by name:

- Step 3 reruns the baseline instrument, and its anomaly rows count as
  changed entries. The core compares only the plan's outside manifest by
  the program's own change identity.
- The continuation after a stop writes a pending-activation record into the
  maintained manual. The core reports the state and stops.
- The rehearsal of the real plan in cutover order 6 needs a disposable
  layout built from that plan. The core rehearses only its fixture plan, and
  the fixture's watch patterns stand in for the real programs.

## Decisions (running log)

**Base.** On 2026-11-07 this checkout was at `81c63a67b1f`, the commit of
owner-switch-preludes-k240.

**Gates are subcommands.** `owner_switch.py gate 4` to `gate 7` are the
checks. A plan names each one as a command whose argv holds `$python` and
`$program`. The program replaces them with its interpreter and its own path.
A continuation therefore runs the gates of the program copy in its backup,
and the plan's digest is the same in each HOME.

**What a gate reads.** The program gives each command `OWNER_SWITCH_PLAN`,
the path of the plan that names it, and HOME set to the switch's HOME. The
plan's new section `accepted` holds the accepted values, so the plan's digest
covers them and the rehearsal record certifies them:

- `sources`: each of the ten IDs with its formula and accepted
  `resourceDigest`
- `versions`: the accepted version of each skill formula except Grove's
- `trials`: for each source, the commands that rerun its staging trials

The Grove formula `linkuistics/taps/grove` is a constant of the program, as
the preludes' `opt/grove` path already is. The drafted real Ravel settings
are the switched cells of the plan's part for `.config/ravel/settings.json`.

**What step 4 admits.** A gate prints its report and then one last line,
`owner-switch-result: <JSON>`. The program keeps that object in the
command's evidence. Step 4's object names the admitted Grove version and
each rerun source's new digest. The program passes these to steps 5 to 7 as
`OWNER_SWITCH_ADMITTED`. The active continuation reads them from the step-4
evidence in its backup, so its rerun of step 6 compares the same digests.

**Disposable HOMEs.** Steps 4 and 6 each resolve in a new temporary HOME
that holds only the drafted settings, and each erases it afterwards. Ravel
reads its settings and its cache from HOME, as Ravel's
`docs/specs/skill-discovery.md` and `configuration-evaluation.md` state.
Two such HOMEs therefore give one resolution. The resolution drops every
`RAVEL_*` variable and runs `$(brew --prefix)/bin/ravel`, the promoted
executable.

**Reader directories.** Step 7 cannot see which directories finish loaded.
It reads each registration of each plugin that `enabledPlugins` enables. It
also reads each directory that `OWNER_SWITCH_READERS` names, which finish
sets from the cache paths that it read. Such a directory needs a
registration. Scanning every cached version would stop on old markers: on
2026-11-07 this account held `.orphaned_at` in ten unregistered version
directories and in no registered one.

**Tool formats.** Measured on 2026-11-07 with mise 2026.11.2 and Homebrew
7.0.12: `mise bootstrap packages status --json` groups rows by backend, and
each `brew` row holds `package`, `desired_state`, `state` and
`installed_version`. `brew info --json=v2` gives `versions.stable`,
`revision`, `linked_keg` and `installed[].version`. `grove --version` prints
`grove <version>`.

**HOME for every command.** Each command of a plan now runs with HOME set
to the switch's HOME. Before, it inherited the caller's HOME. The real run
has the same value, and a rehearsal can no longer read the real HOME
through a plan's command.

**Gate reports are visible.** The program prints the report of each gate and
the tail of each failed command. Before, a failed gate printed only its name.

**The three items of owner-switch-core-k239.**

- The baseline rows: decided here. The plan's `baseline` command runs in
  step 3 after the preludes. Each row with an anomaly field counts as a
  changed entry outside the switch set. The native state then reads as
  changed, and the active continuation counts it as a failed probe before
  its probes. Step 14 counts it too. Step 3 prints each `unverified` row,
  and finish compares it with its own listing. The finish handoff of
  discovery-rollout-k12 receives that comparison and `OWNER_SWITCH_READERS`.
- The pending-activation record: handed on to owner-switch-entries-k92,
  which owns the other entries that write for the owner.
- The rehearsal of the real plan: handed on to
  owner-ravel-skill-inventory-k38, with the shape of the real plan's new
  sections. The off-value control of `ravel` goes to its staging trials.

**A repair of the core control.** `prepared-plan-differs` dropped the Ravel
settings part from its changed plan, so step 4 could no longer draft the
settings. Its changed plan now drops the Claude Code settings part instead.

**Mutants.** Seventeen mutants each lacked one mechanism of this slice, and
each failed its guarding control. The first run of `step4-tap` survived,
because the control of a newer tap stopped at the uninstalled version
first. The control `grove-linked-older` then killed it. The control
`baseline-anomaly-active` asserts that no probe ran, so the early check of
the active continuation has its own guard beside step 14.

**A repair of this slice.** The first `bash scripts/check.sh` failed the
sweep of `GROVE_*` names. It found this slice's constant `GROVE_FORMULA` in
both scripts. The constant is now `FORMULA_OF_GROVE`, and the sweep passes.

**An unrelated failure.** That first run also failed the standalone test
`a_timed_out_selection_launches_nothing_and_publishes_nothing` with "select
never ran". It failed again in four direct runs. The test reads no file that
this slice changed, and the slice changed no Rust source. The final check
run passed it on the same inputs. Its result therefore depends on the
machine's state, not on this slice.

**Verification.** The mutants ran before the rename, which changed no
mechanism. On the final program `f0c8d2d9e27f` and rehearsal script
`b121fbced4e5`, `task owner:switch:rehearse` passed 173 of 173 controls in
8 minutes with Python 3.15.1. The system Python 3.9.6 passed 25 of 25 gate
and core controls.

`mise exec -- task model:owner-switch` exited 0 with "all claims hold". The model digest `c90bbf9ab658` and the runner digest
`0fe7ef316e63` did not change. The digests of the program, the rehearsal
script, the Taskfile and both model files were the same before and after
these runs. Then `bash scripts/check.sh` passed all 18 principal checks,
with 1,344 tests passed.

**Model.** This slice changed no recovery rule. The active continuation's
check of changed entries before its probes applies the existing row of the
state table.
