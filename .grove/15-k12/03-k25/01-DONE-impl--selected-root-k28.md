# selected-root-k28


## Goal
Refuse foreground launches whose selected task-root directory was replaced
before epoch activation, retaining the selected root independently of Selection.



## Context
Use loop_driver's picked and launch_configured_session, and the existing
TreeLifetime pin. This is the first child of launch-witnesses-k25.

## Done when
- Selection captures/checks its root with the validated tree guard and returns
  value data plus a separately owned pin, with no advisory guard escaping.
- Both ordinary and newly materialized finish selections take this path.
- Launch rechecks the retained root before epoch activation and refuses missing
  or replaced roots without spawning or publishing an active epoch.
- Tests prove valid launch, same-key replacement refusal, removal refusal,
  finish selection, malformed-tree refusal and containing-directory mutation.
- Current protocol documentation and affected walkthrough fragments/indexes
  match the shipped behavior; focused tests and scripts/check.sh pass.

## Notes
Witness ownership/publication stays in witness-owner-k29. No new RUNNING evidence
is claimed by this slice.

## Decisions (running log)

Reuse TreeLifetime without adding descriptors to Selection. Pin before snapshot
read and check under its guard; recheck before activation. After finish
materialization, discard its value selection and perform a fresh guarded pick,
so any intervening change is selected anew rather than rebound to stale values.

The initial real-launch regression failed because the child spawned after root
removal. With the pin check it passes alongside the valid-launch control,
same-key replacement, finish selection and independent mutation-lock checks.
The later publication-time recheck belongs to witness-owner-k29, including root
changes during epoch-lock acquisition. Full protocol review stays with the
witnessed node's final producer; this slice's behavioral doubt used the red test.
