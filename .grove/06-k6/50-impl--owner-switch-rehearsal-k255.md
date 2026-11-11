# owner-switch-rehearsal-k255

## Goal

Execute the prepared real plan once in a disposable copy of HOME, including
full restore. Retain a readable, content-identified result and its
comparison with the real inputs.

## Context

- The section "The rehearsed plan" of `docs/specs/owner-skill-delivery.md`
  states the rule. The switch consumes prepared values, not a passing token
  or a record of an earlier run.
- owner-switch-dry-run-k271 supplies the dry-run instrument and the
  comparison command. owner-switch-plan-k254 supplies the prepared plan.
- The plan's `accepted` holds real digests. The dry run therefore resolves
  with the accepted 2.0 build against a copy of the real kegs. The promoted
  `ravel` exists only inside finish, so the result declares the prefix
  substitution from `ravel@2`.
- The dry run runs the real `mac-check` with HOME set to the disposable
  HOME.
- owner-marketplace-freeze-k253 holds Claude Code's version. Sign-in reuse
  follows the staging contract of owner-staging-setup-k248.
- core-candidate-restaging-k300 found a limit of the native probe. In the
  `codex-native-bare` case, Codex searched HOME and read `grove-planning`
  from the frozen `grove@linkuistics` copy in Claude Code's plugin cache.
  The probe's `kindSkill` observation counts that copy as native. Only the
  `listing` observation failed, so the probe still failed. The section
  "Core candidate restaging" of `docs/owner-skill-delivery-evidence.md`
  holds the records.

## Done when

- The dry run executes the ordinary writes, hook commands, probes, probe
  cleanup, final comparison and full restore. It also runs `native-off`
  without a switch backup. The real HOME is unchanged.
- The result's input table identifies each input by content, before and
  after the run. The hook commands ran with the held Claude Code version.
- The comparison command, run against the real prepared inputs, reports
  each row as equal or as a declared equal-content prefix substitution.
- The result holds per-probe outcomes, the restore comparison and its
  limits. It is committed, and it is also readable outside `.grove/`.
- The evidence document states when another dry run is necessary. A
  changed plan, source digest, program, helper, command input or tool
  version requires one.

## Notes

discovery-rollout-k12 records each accepted digest again with the published
Ravel archives. A differing digest first needs that source's staging trials
and revised acceptance, then another dry run.

The dry run is reproducible, so a new run after a change costs little.
Never edit a result by hand.

owner-switch-dry-run-k271 delivered the instrument as
`scripts/owner_switch_dry_run.py`, with `task owner:switch:dry-run` and
`task owner:switch:compare`. Its module docstring states the input table,
the copied layout and the declared prefixes. Declare the candidate keg and
`ravel@2` with `run --prefix grove-keg=DIR --prefix ravel=DIR`. Pass sign-in
files and the plugin cache with `--include` as the staging contract needs.
The comparison compares the row `tool:grove` by content and by
`grove --version`. A candidate keg that was built before the release bump
reports another version than the released keg. Inside finish, that row then
differs, so finish needs another dry run against the released keg, or a
candidate keg built at the release commit. The section "Owner switch dry
run" of `docs/owner-skill-delivery-evidence.md` records this limit.
