# owner-marketplace-freeze-k253

## Goal

Freeze automatic plugin updates for every plugin that finish can load, and
hold Claude Code's version from the rehearsal to the switch.

## Context

- The section "Resource lifetime" of `docs/specs/owner-skill-delivery.md`
  keeps each version that a live reader uses registered and current. An
  update or an uninstall writes `.orphaned_at`. After 14 days, Claude Code
  2.1.319 checks live reader markers before deletion. The design keeps the
  freeze independently of that undocumented check. Automatic updates stay
  frozen from the preparation step onward. The rule covers every plugin that
  finish loaded, not only Grove's.
- The `linkuistics` marketplace follows Grove's `main`. Finish moves `main`
  before the switch, so an automatic update could orphan finish's own
  guidance.
- The re-inventory of 2026-11-07 saw the official marketplace update
  `context7` and `playwright` within five days. Claude Code itself moved
  from 2.1.316 to 2.1.319 in the same days.
- The hooks installation runs a Claude Code command only with a version
  that a passing rehearsal record names. The design's section "The
  rehearsed plan" states that rule.

## Done when

- Automatic updates are off for the `linkuistics`, official and
  `quint-llm-kit` marketplaces, and for any other marketplace of an enabled
  plugin. A committed record names the setting, its file and its owner.
- Claude Code's own automatic update is held, by a setting that the owner
  approved. The record names the held version and how the owner releases
  the hold after the switch.
- A check after one day shows no new version directory and no
  `.orphaned_at` marker for an enabled plugin. It shows the same
  `claude --version`.
- Each changed file follows the machine configuration contract. mise
  tracks `~/.claude/settings.json` in place.
- Native discovery is unchanged.

## Notes

Changing these settings changes the owner's live configuration. Show the
owner each change and ask before you write.

A setting that this leaf writes into `~/.claude/settings.json` becomes an
earlier value of the real plan. owner-switch-plan-k254 reads the file after
this leaf.
