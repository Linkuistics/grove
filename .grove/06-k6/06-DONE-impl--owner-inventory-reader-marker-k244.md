# owner-inventory-reader-marker-k244

## Goal

Make the inventory's change identity of a plugin version directory
independent of the running Claude Code sessions.

## Context

- owner-ravel-skill-inventory-k38 found that Claude Code writes
  `.in_use/<pid>` into each plugin version directory that a running session
  loaded. The marker holds the PID and the start time of that process.
  `docs/evaluations/owner-skill-delivery/inventory.py` digests the whole
  version directory, so each loaded plugin's row changes with the session.
  The section "Re-inventory before the cutover" of
  `docs/owner-skill-delivery-evidence.md` records the finding.
- The baseline held one marker for each loaded plugin. The 2026-11-07 rerun
  held one marker from another PID in the same places.
- Step 3 of the switch runs this instrument as the plan's baseline command.
  It reads only the flag fields, so the digest change does not stop a run.
  The final inventory of rollout compares digests, so the noise would hide
  a real change there.

## Done when

- The digest of a plugin version directory leaves out `.in_use` and its
  contents at the top level of that directory. The docstring states the
  exclusion and its reason. No other entry class changes.
- `controls.py` has a control that adds, changes and erases an `.in_use`
  marker. The row stays the same. A control that changes another file of
  the version directory still changes the row. Each control fails against
  the earlier instrument, and the record says so.
- The evidence document records the new SHA-256 values of the instrument
  and the controls script. It states how a later comparison reads the
  baseline's plugin rows, which still include an earlier marker.
- `task owner:inventory:controls` passes. A rerun in the real HOME, run twice,
  gives identical output with no flagged row.

## Notes

Decide whether to commit a new reference inventory beside the baseline.
Keep the baseline itself unchanged, because the moves record and the design
cite its line numbers.

Find whether Claude Code's cleanup reads `.in_use`. The design's section
"Resource lifetime" says that the cleanup ignores readers. If a source
shows otherwise, record it as a finding for the owner design. Do not change
the design in this leaf.

## Decisions (running log)

- Scope of the exclusion: only the Claude Code plugin row leaves out
  `.in_use`. Codex's cache holds no `.in_use` and Codex does not write one,
  so the Codex installed-plugin row stays as it was. The exclusion applies
  only at the top level of the version directory, whatever the entry's type.
  `.orphaned_at` stays in the digest, because it records a real update or
  uninstall.
- Precedent: Claude Code 2.1.319 has its own folder digest for managed
  plugin releases. That digest leaves out `.git`, `.in_use`, `.orphaned_at`,
  `.claude-plugin-link` and `.links_materialized` at the root. Source: the
  settings-schema text in `~/.local/share/claude/versions/2.1.319`.
- Finding for the owner design: Claude Code 2.1.319's orphan cleanup reads
  `.in_use`. After the 14-day age, it calls a liveness check on the version
  directory. That check parses each `.in_use/<pid>` marker (`pid`,
  `procStart`) and skips the removal for a live session ("Skipping orphan
  cleanup, in use by live session"). Each session erases its own marker at
  exit, so an empty `.in_use` directory remains. The official plugin loading
  reference does not mention the check. The design's "Resource lifetime"
  says that the cleanup ignores readers. This leaf records the finding in
  the evidence document and leaves the design unchanged.
- Reference inventory: commit `docs/owner-skill-inventory-reference.tsv`
  beside the baseline. The baseline's Claude Code plugin digests include an
  earlier session's marker, and nobody can reconstruct its bytes. The
  reference is the only comparable record of those version directories.
