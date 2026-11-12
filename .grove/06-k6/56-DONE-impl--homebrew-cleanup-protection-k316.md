# homebrew-cleanup-protection-k316

## Goal

Protect the original Ravel 1.1.0 keg from Homebrew's periodic cleanup on
every Homebrew invocation on the machine, before finish.

## Context

- `docs/specs/coordinated-release.md`, section "Activation and running
  readers".
- Ravel `docs/specs/release-channels.md`, section "Recovery and existing
  processes".
- The `linkuistics:machine-environment` contract for persistent settings.

## Done when

- A user `brew.env` that every Homebrew invocation reads adds `ravel` to
  `HOMEBREW_NO_CLEANUP_FORMULAE`. Existing entries stay.
- The file is tracked as the machine contract requires. The maintained manual
  records why it exists and when the owner may remove it. That time is after
  the reap of the last driver that uses the 1.1.0 keg.
- A read-only Homebrew query shows the value in effect.
- A control in a scratch Homebrew environment shows that the setting stops a
  cleanup of an old `ravel` version. The same control without the setting
  removes that version.

## Notes

No user `brew.env` existed on 2026-11-11. Do not run `brew cleanup` on the
real prefix to test the setting.

## Decisions (running log)

- The file is `~/.homebrew/brew.env`. Homebrew 7.0.13's `bin/brew` reads
  `/etc/homebrew/brew.env`, then `$HOMEBREW_PREFIX/etc/homebrew/brew.env`,
  then `$HOMEBREW_USER_CONFIG_HOME/brew.env`. That last directory is
  `$XDG_CONFIG_HOME/homebrew` when set, else `~/.homebrew`. No shell or
  tracked dotfile sets `XDG_CONFIG_HOME`. A run under another HOME does not
  read this file. The manual records that limit.
- The file holds one literal line. `bin/brew` exports each line as it
  stands, with no expansion, so the file cannot append to an inherited
  value. On 2026-11-12 no file, shell profile or environment set the
  variable, so there are no existing entries to keep.
- `Cleanup#clean!` rejects each formula that `skip_clean_formula?` names
  before it cleans, and periodic cleanup calls `clean!`. The install
  cleanup and `brew cleanup ravel` use the same test. The match is on the
  exact formula name, so `ravel` does not cover `ravel@2`.
- The control uses a scratch prefix with an APFS-clone copy of
  `/opt/homebrew/Library`. A symlinked `bin/brew` would take the real
  Cellar from the repository, by `brew.sh`'s `HOMEBREW_CELLAR` rule. The
  control runs under `env -i` with a scratch HOME, cache, logs and temp.
- The control is a repository script with a Taskfile task, beside the other
  `owner:` controls, so release readiness can run it again.
- The read-only query is `brew config`. `brew ruby` is a developer command,
  and it turns on developer mode in a repository with `.git/config`. The real
  repository already had `homebrew.devcmdrun true` before this session.
- Each scratch HOME trusts `linkuistics/taps/ravel`. Homebrew 7 otherwise
  loads no formula of the tap, and `Formula.installed` omits `ravel` without
  an error. The first control run showed that. A guard now requires the old
  keg to be eligible for cleanup in both runs.
- The manual record is the section "Homebrew cleanup protection" of the
  Obsidian note "Maintenance and Recovery", under "Update software". That
  section already held the advance rule for `HOMEBREW_NO_INSTALL_CLEANUP`.
- `mise dot track ~/.homebrew/brew.env` refused twice. A foreground
  `mise bootstrap packages upgrade` (pid 70495, ttys002, started 21:11
  local) holds mise's history lock and has no child process. The tracking
  waits for the owner.
- The owner's upgrade ended on 2026-11-12. `mise dot track` then added the
  entry and saved checkpoint 137. `mac-check` reports only the earlier
  unsaved iTerm2 plist, which has autosave off.
