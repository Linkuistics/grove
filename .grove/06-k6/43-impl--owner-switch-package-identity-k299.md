# owner-switch-package-identity-k299

## Goal

Reconcile the owner switch tooling with the revised packages. The switch
compares full package versions, including revisions, and every owner tool
reads keg sources through their private payloads.

## Context

- "Skill formulae" of `docs/specs/owner-skill-delivery.md` states the
  comparison: the plan's non-Grove `accepted.versions` name full package
  versions. If mise omits a revision, the comparison uses Homebrew's
  package identity.
- `gate_5` of `scripts/owner_switch.py` compares mise's
  `installed_version` with the plan's accepted string. Finding L1 of the
  evidence section "Directory-link layout review" shows why that fails for
  a revision.
- `docs/evaluations/owner-skill-delivery/switch_plan_draft.py` and its
  `switch-plan-draft.json` name `2026-11-06`, `2.8.0`, `7.0.0` and
  `1.0.0`.
- These tools build or read keg sources at `share/ravel/skills` as real
  directories: `scripts/owner_switch_rehearsal.py`,
  `scripts/owner_switch_dry_run.py`, `quiet_interval.py`,
  `settings_drafts.py` and the inventory under
  `docs/evaluations/owner-skill-delivery/`. Enumerate the whole class with
  a search, not with this list.
- skill-formula-publication-k298 records the published full package
  versions.

## Done when

- The switch's version step compares the full installed package version
  with the accepted one. It reads Homebrew's package identity when mise
  omits the revision. Fixtures distinguish the earlier revision from the
  accepted revision, and a missing formula from drift.
- The plan draft names the published full package versions. Its check
  task passes.
- Rehearsal and dry-run fixture kegs use the projected layout. Prepared
  input comparisons and declared prefix substitutions accept the private
  payload paths. The inventory classifies the aliases and private payloads
  without an anomaly row.
- Each hook registration that a tool records or compares uses the path
  that the hook-adapter evidence measured. Claude Code keeps the `opt`
  path. Codex stores the canonical private path.
- Each changed tool's controls and `bash scripts/check.sh` pass. A mutated
  version or path fails its control.

## Notes

`mac-check` belongs to owner-pack-checkouts-k93, which already owns its
full-version report and its invalid-layout report. This leaf changes the
switch program and its Grove-side tools.

A changed program, helper or input needs another dry run.
owner-switch-rehearsal-k255 runs it after owner-switch-plan-k254.
