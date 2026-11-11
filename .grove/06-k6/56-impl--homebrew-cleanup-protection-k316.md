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
