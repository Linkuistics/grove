# The owner's checklist for the skill switch

<!-- tc: depart synonym-rotation because "launchd", "launchctl", "LaunchAgents", "LaunchDaemons" and "launcher" are names of macOS components and of the Claude Code executable link, not the verb "start" -->

This document is the owner's checklist for the dry run, the switch and any
restore. It also records the update freeze that holds the tested plugin and
Claude Code versions until then.

[The owner's checklist](../specs/owner-skill-delivery.md#the-owners-checklist)
in the owner delivery design owns each rule. This document names the current
writers on the owner's Mac, with pause and resume instructions for each. The
instructions are owner actions, not program exemptions. The narrow process
inspection of the switch program refuses only other Claude Code, Codex, Grove
and Ravel processes. Every other writer below is the owner's responsibility.

owner-marketplace-freeze-k253 wrote this document on 2026-11-11 from a
read-only inventory. It paused and stopped no tool.

## The update freeze

### The setting

The owner approved this setting on 2026-11-11.

| Field | Value |
| --- | --- |
| File | `~/.claude/settings.json` |
| Owner | The machine owner. mise tracks the file in place through `[dotfiles]` in `~/.config/mise/config.toml`. The history watcher saved the change as checkpoint 128. |
| Claude Code hold | `"env": {"DISABLE_AUTOUPDATER": "1"}` |
| Marketplace freeze | `"autoUpdate": false` on the `extraKnownMarketplaces` entries `linkuistics`, `quint-llm-kit` and `claude-plugins-official` |
| Held version | Claude Code `2.1.322`. The launcher `~/.local/bin/claude` links to `~/.local/share/claude/versions/2.1.322`. |
| Frozen marketplaces | `claude-plugins-official`, `linkuistics` and `quint-llm-kit`. These are the marketplaces of every enabled plugin. |
| Baseline | [`update-freeze-baseline.json`](update-freeze-baseline.json), recorded at 2026-11-11T03:33:44Z |

`DISABLE_AUTOUPDATER` stops Claude Code's background update. It also stops
the whole plugin auto-update pass. A marketplace `autoUpdate` value in
settings has the highest precedence for that marketplace. The two settings
are separate because their lifetimes differ. The Claude Code hold ends after
the switch. The marketplace freeze lasts until purge.

The `personal` marketplace has a `directory` source. Its plugins load in
place, and it does not update automatically by default, so it needs no flag.

The legacy `"autoUpdates": false` in `~/.claude.json` does not hold the
version. Claude Code moved from 2.1.319 to 2.1.321 and then to 2.1.322
between 6 and 10 November with that key set.

The held version is the version of the quiet-interval measurement of
owner-staging-hooks-k251. `task owner:quiet-interval` therefore needs no
second run before the dry run.

### What the freeze does not stop

The setting stops automatic updates only. During the hold, do not run these
commands:

- `claude update` or `claude install`.
- `claude plugin update`, `claude plugin marketplace update`, or "Update now"
  and "Update marketplace" in `/plugin`.
- A `claude plugin install <name>@<marketplace>` for a frozen marketplace.
  Such an install refreshes the named marketplace whatever its `autoUpdate`
  value is. The switch installs only from the `writing-hooks` directory
  marketplace, which has no refresh.
- An "Enable auto-update" toggle under `/plugin` Marketplaces.

The `env` block reaches only sessions that read the user settings. A session
started with a `--setting-sources` list that leaves out `user` does not
receive the hold. The Claude desktop application updates itself through its
own updater. Quit it as item 1 of the steps below states.

### Check the freeze

1. Run `task owner:freeze:check BASELINE=docs/owner-cutover/update-freeze-baseline.json`.
2. Run `claude doctor`, and find the line
   `Auto-updates: disabled (set by env: DISABLE_AUTOUPDATER)`.

The check fails on a changed freeze setting or Claude Code version. It also
fails on a new plugin version directory, a new orphan marker, a changed
installation and a marketplace refresh. Claude Code removes an orphaned
directory 14 days after its marker. The check reports that removal as a note
and passes. `task owner:freeze:controls` shows that the check sees each class
of change.

The check after one day belongs to the leaf before
owner-switch-rehearsal-k255. Repeat the check before the dry run and before
each real attempt.

### Release the holds

After a successful switch receipt, release the Claude Code hold:

1. Remove the `DISABLE_AUTOUPDATER` key from the `env` block of
   `~/.claude/settings.json`. Keep `MCP_TOOL_TIMEOUT`.
2. Keep the three marketplace `autoUpdate` values.
3. Run `claude update`, or let the background update run.
4. Run `claude doctor`, and find `Auto-updates: enabled`.

Do not release the marketplace freeze before purge. The
[resource lifetime](../specs/owner-skill-delivery.md#resource-lifetime) rule
keeps the frozen plugin versions registered and current until then. The purge
removes the superseded native routes. After purge, remove each remaining
`autoUpdate` value only for a marketplace that the owner keeps.

## Steps for each attempt

Do these steps before the dry run, the real switch and any restore. Keep the
quiet interval until the probes, the receipt or the restore end.

1. End other agent sessions and applications.
   - End each other Claude Code, Codex and Pi session. Pi is installed at
     `/opt/homebrew/bin/pi` and `/usr/local/bin/pi`, and `~/.pi` exists. No Pi
     process ran at the inventory.
   - Quit `Claude.app` and `ChatGPT.app`. Both are installed and neither ran
     at the inventory. Either can host tools independently of CLI sessions.
   - End each other Grove or Ravel invocation. The inventory found a Ravel
     1.1.0 `release-notes` session from 22 October, started from a temporary
     directory, with its `harness.sh` child.
   - End the operator's own delegates and background work.
   - Resume: open the applications again after the receipt or the restore.
2. Stop the owner's Codex app-server daemon.
   - It ran from `~/.codex/packages/app-server-daemon/releases/0.167.0-aarch64-apple-darwin`
     as `codex app-server daemon pid-update-loop`, with a managed
     `codex app-server` child and `codex-code-mode-host`.
   - Pause: run `codex app-server daemon stop` with the owner's HOME.
   - Resume: run `codex app-server daemon start`, or let Codex start it.
3. End the left-over Codex daemons of earlier trials.
   - About 20 `app-server-daemon` processes ran from disposable HOMEs under
     `/private/tmp/claude-501/…/scratchpad`. Their parents had exited. The
     narrow process inspection refuses each one.
   - Pause: for each such HOME, run `codex app-server daemon stop` with
     `CODEX_HOME` set to that HOME's `.codex`. If one stays, end its process.
   - Resume: none. These daemons belong to finished trials.
4. Pause `mise dot watch`.
   - The LaunchAgent `dev.mise.mise-history` runs it from
     `~/Library/LaunchAgents/dev.mise.mise-history.plist`. Its `KeepAlive`
     restarts it after a failed exit, so do not end the process alone.
   - `history.sync` is `manual`. The watcher therefore applies no incoming
     change without `mise dot sync`.
   - Pause: run `launchctl bootout gui/$(id -u)/dev.mise.mise-history`.
   - Resume: run `mise bootstrap services apply`. The watcher then saves the
     switch's changes to tracked files as checkpoints.
5. Pause herdr.
   - `~/.local/bin/herdr-core serve` ran with no loaded job behind it.
     `Herdr.app` is installed. The user `SessionStart` hook runs
     `~/.claude/hooks/herdr-agent-state.sh`.
   - Pause: quit `Herdr.app`, then run `herdr-core server stop`. Stopping
     parks the kept terminals.
   - Resume: run `herdr-core server start`, or open `Herdr.app`.
6. Remove the left-over herdr jobs.
   - The jobs `dev.herdr.server.78950.1792987156121559000` and
     `dev.herdr.server.79199.1792987156998465000` are loaded but do not run.
     Their plists are under `/private/tmp/hp.LiqtfP`, and their program is a
     herdr build in another grove's scratchpad.
   - Pause: run `launchctl bootout gui/$(id -u)/<label>` for each label.
   - Resume: none. They belong to a finished trial.
7. Quit Orca.
   - `Orca.app` is installed and did not run at the inventory. `OrcaSlicer.app`
     is unrelated.
   - Resume: open `Orca.app`.
8. Stop codebase-memory updates.
   - `~/.local/bin/codebase-memory-mcp` runs as an MCP server under each
     Claude Code session, with a `--cbm-daemon-internal` daemon. Those end
     with their sessions.
   - Pause: run no `codebase-memory-mcp install`, `update` or `uninstall`.
     These commands can rewrite harness configuration.
   - Resume: no action.
9. Pause Obsidian and its synchronisation.
   - Obsidian ran at the inventory. The maintained manual is in the iCloud
     vault `Brain`.
   - Pause: quit Obsidian on this Mac and on each other device that edits the
     vault. Make no edit to the vault through another program or device.
   - Resume: open Obsidian only after the appended entries agree with the
     operation's result.
10. Prevent a new start of each writer.
    - Open System Settings, then General, then Login Items & Extensions.
      Record each item of Claude, ChatGPT, Herdr, Orca, Obsidian or Codex
      that opens at login. Turn each such item off for the interval.
    - Run `launchctl list` for the real account, and find each job that names
      a writer above. A job can exist without a LaunchAgents file. Run
      `launchctl print gui/$(id -u)/<label>` to read its program.
    - Run `crontab -l`. The inventory found no crontab.
    - Resume: turn each recorded login item on again.
11. Run no bootstrap, plugin update, Homebrew upgrade or configuration
    synchronisation. This includes `mise bootstrap`, `mise dot apply`,
    `mise dot sync` and `brew upgrade`.
12. Leave the operator's Claude Code settings unchanged until the program
    returns. Invoke no `/model`, `/config`, plugin action, mutating MCP
    operation or in-process delegate.
13. Keep the update freeze, and check it as "Check the freeze" states.
    Keep the original Ravel 1.1.0 keg. Compare tool versions with
    `task owner:switch:compare` before the dry run and the real attempt.
14. Review the plan, the staging evidence and the input comparison of the dry
    run. Check the planned native layout, and check that the writers above
    stay paused.

## TestAnyware after a login

The TestAnyware application does not start at login. `testanyware doctor`
fails its check `login_item` in the real HOME. After a login without the
application, each VM command fails as `APP_UNREACHABLE`.

- Before a probe or any TestAnyware use, open `TestAnyware.app`.
- Alternatively, add it in System Settings under Login Items & Extensions.
  Then `testanyware doctor` passes `login_item`.

The application ran at the inventory. It writes no file of the switch set.
"The doctor check" of "TestAnyware in staging" in the
[evidence](../owner-skill-delivery-evidence.md) holds the result.

## Reviewed jobs without an action

The inventory read these loaded jobs and their property lists. None of them names a
writer of the switch set or a harness, so the checklist asks no action for
them:

- User LaunchAgents of Dropbox, Watchman and JetBrains Toolbox.
- System LaunchAgents of Adobe, Arturia, Google, Microsoft, Jump Desktop,
  PACE, Waves, Vienna, XQuartz and Zoom.
- System LaunchDaemons of Adobe, Arturia, Docker, Google, Native
  Instruments, Muse, Spitfire, Steinberg, Klevgrand, TeamViewer and Nix.
- `application.*` entries of running applications, such as iTerm2, Visual
  Studio Code, Slack, Telegram and 1Password.

## Inspection limits

- Login items: `sfltool dumpbtm` needs authorisation, so this inventory did
  not read login items. The owner reads them in System Settings, as step 10
  states. No exhaustive unprivileged login-item command is assumed.
- The `application.*` entries of `launchctl list` show applications that
  run. They do not show which applications open at login.
- The process list is a snapshot of 2026-11-11. Repeat steps 1 to 10 before
  each attempt, because a process or job can appear later.
- The inventory did not inspect the owner's other devices. Step 9 covers
  their edits to the vault.
- The checklist requires no scanner of service jobs. The steps above name
  each known job, and the job listing of step 10 finds the others.
