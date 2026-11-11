# owner-marketplace-freeze-k253

## Goal

Freeze automatic plugin updates for every plugin that finish can load, and
hold Claude Code's version from the dry run to the switch. Write the pause
and resume instructions of the owner's checklist.

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
- The switch reads no rehearsal record. The dry run of
  owner-switch-rehearsal-k255 tests Claude Code's commands with the held
  version. The comparison command compares tool versions before each real
  attempt. A different version needs another dry run.
- The section "The owner's checklist" lists the writers to pause. The
  narrow process inspection refuses only other Claude Code, Codex, Grove and
  Ravel processes.
- staging-testanyware-trial-k287 found that the TestAnyware app does not
  start at login. `testanyware doctor` fails its check `login_item` in the
  real HOME. After a login without the app, each VM command fails as
  `APP_UNREACHABLE` until the owner opens TestAnyware. "The doctor check" of
  "TestAnyware in staging" in `docs/owner-skill-delivery-evidence.md` holds
  the result. The checklist names this setting for the owner.
- owner-staging-hooks-k251 measured the quiet interval with Claude Code
  2.1.322. If the held version is another version, run
  `task owner:quiet-interval` again with it before the dry run. "Staging
  acceptance" of `docs/owner-skill-delivery-evidence.md` states this limit.

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
- A committed checklist names the current writers, with pause and resume
  instructions for each. It covers the owner's tools, such as
  `mise dot watch`, herdr, Orca configuration writers and codebase-memory
  updaters. It also covers the Claude and ChatGPT applications, loaded jobs
  and login items, and Obsidian with its synchronisation.
<!-- tc: depart synonym-rotation because "launchd" is the name of the macOS service manager, not the verb "launch" -->
- The checklist records each inspection limit, such as Login Items that
  need authorisation. It requires no launchd scanner.
- Each changed file follows the machine configuration contract. mise
  tracks `~/.claude/settings.json` in place.
- Native discovery is unchanged.

## Notes

Changing these settings changes the owner's live configuration. Show the
owner each change and ask before you write.

A setting that this leaf writes into `~/.claude/settings.json` becomes part
of the real plan's backed-up native state. owner-switch-plan-k254 reads the
file after this leaf.

The checklist is owner instruction, not a program exemption. Do not pause
or stop any real tool in this leaf. Inspect read only.

## Decisions (running log)

- 2026-11-11: The owner approved the freeze setting in `~/.claude/settings.json`.
  `env.DISABLE_AUTOUPDATER` is `"1"`. It holds Claude Code's own background
  update and stops the whole plugin auto-update pass. `autoUpdate` is `false` on
  the `extraKnownMarketplaces` entries `linkuistics`, `quint-llm-kit` and a new
  `claude-plugins-official` entry with its existing source. The two holds have
  different lifetimes. The owner releases the Claude Code hold after the switch.
  The marketplace freeze lasts until purge. `DISABLE_UPDATES` was rejected,
  because staging ran with `DISABLE_AUTOUPDATER` and its effect on plugin
  commands is not measured. A disposable-HOME `claude doctor` on 2.1.322 accepted
  the setting. A control with a string `autoUpdate` was reported as invalid.
- 2026-11-11: The owner chose a separate check leaf for the one-day check. It
  sits before owner-switch-rehearsal-k255 and uses the baseline that this leaf
  records. It hands back to the owner if fewer than 24 hours have passed.
