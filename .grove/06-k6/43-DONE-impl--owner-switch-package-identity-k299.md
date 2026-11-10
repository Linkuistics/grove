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

## Decisions (running log)

- Step 5 reads mise only for the declared and installed state. The full
  package version always comes from `linked_keg` of
  `brew info --json=v2 <formula>`. mise must report that keg, with or
  without its revision. Reason: an unrevised string from mise cannot tell
  revision 0 from an omitted revision, and the linked keg is the one that
  the default skill directory names. On 2026-11-11, Homebrew 7.0.12 gave
  `zstd` `linked_keg` `1.5.7_1`, and mise 2026.11.2 reported the same.
- Grove's admitted version from step 4 was already the full package
  version. Step 4 now requires each Grove source's `originPath` to be the
  private payload `libexec/skill-sources/<id>`. A real directory at the
  alias's place no longer passes.
- The fixture kegs carry the published full versions with revision 1 for
  the four other formulae. Mutations of step 4's path, of the string
  comparison and of a revision-blind comparison each failed their controls.
- The plan draft's versions are the full package versions. The draft now
  checks them against the table of "Publication" in "Packaging-only
  acceptance", not against the moves record, which names the earlier
  unrevised versions. The redraft also renewed the program and inventory
  digests, two Claude Code registry identities and the held Claude Code
  version 2.1.322, from the real HOME.
- The quiet-interval prefix links each keg's alias, as Homebrew does, and
  refuses a keg at another version or a source outside its private payload.
  On 2026-11-11 the real prefix step refused both the staging candidate,
  which predates the projection (k300 rebuilds it), and the installed kegs
  at the earlier versions (k93 upgrades them).
- The inventory classifies the hook adapters as native residue, in the
  manner of `guardrail`. Claude Code must register `writing-hooks` from
  `/opt/homebrew/opt/grove`. Codex must name the canonical linked keg.
  Both marketplace files must name the private payload. A Pi link through
  the writing alias must resolve into that payload. Each other state says
  UNCLASSIFIED. The real inventory equals the earlier instrument's output
  row for row, because the real HOME has no hook adapter yet.
