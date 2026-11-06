# ravel-default-skill-directory-k218

## Goal

Design Ravel 2.0.0's default skill directory, so that `brew install` and
`brew upgrade` are the whole skill setup. Settle its effect on the owner
switch before the plan's rehearsal.

## Context

- The human wrote: "My goal is to have nothing more to do beyond homebrew
  install/upgrade". The human then chose "Ravel default in 2.0.0" over
  explicit settings. The running log of `homebrew-delivery-design-k216` holds
  both, with the readings behind them.
- Homebrew cannot write a link or a setting in HOME. Each skill formula
  therefore installs its sources at `share/ravel/skills/<source-id>`, and
  Homebrew links them into `$(brew --prefix)/share/ravel/skills/`. Ravel reads
  that directory when the settings name no sources.
- `docs/specs/codex-skill-provisioning.md`, "The keg layout", states the
  layout. It leaves the contract to Ravel's specification.
- `docs/specs/owner-skill-delivery.md`, "Open question: the default
  directory", states the conflict with the switch. Three rules read an absent
  `skills` key as delivery off: preparation, the inverse and the native probe.
- Ravel's specification is `docs/specs/skill-discovery.md` on Ravel's
  `skill-directory-delivery` line. Its protected-path rule and
  `crates/ravel/src/skills/mod.rs` refuse a source inside `~/.config/ravel`.
- `docs/homebrew-skill-delivery-evidence.md` holds the Homebrew and Ravel
  readings. Ravel 2.0.0 has no cut yet, and this grove owns its release.

## Done when

- Ravel's specification states the default skill directory. It answers each
  question below, and its own decision records change as their test demands.
- The open question of the owner design is settled in that design. A change to
  a recovery rule changes `docs/models/owner-switch.qnt` first, and
  `task model:owner-switch` stays green.
- Grove's delivery design and owner design cite the settled contract and
  restate none of it.
- A review chain is judged for this design and the amendments of
  `homebrew-delivery-design-k216` together. A chain that earns its place is cut
  with `leaf-insert` before `homebrew-delivery-plan-k217`.

## Notes

Questions for the design:

- **Finding the directory.** How Ravel finds the Homebrew prefix: from its own
  canonical executable, from a value that its formula records, or otherwise.
  The keg-only `ravel@2` and the Linux prefix must work. A Ravel installed
  outside Homebrew needs a stated behaviour.
- **Roots.** How Ravel reads each source's roots without settings: a
  convention such as `skills/` and `craft/`, or a descriptor in each source.
  The third-party root `quint-llm-kit-plugin/skills` is a case.
- **Entries.** The source ID of an entry is its directory name. State what a
  malformed name, a dangling link, a file, a dot entry and an empty directory
  do.
- **Settings.** Whether a `skills` key replaces the default or adds to it. How
  a mandatory file names a default source, and how a user turns the default
  off.
- **Schema and routes.** Whether default delivery changes what schema 4 or
  "delivery disabled" means. Grove's "native route on 2.0" depends on the
  answer.
- **The owner switch.** How the native state turns delivery off after
  promotion, what the inverse writes, and what the native probe observes.
  Finish's own launch stays on Ravel 1.1, which refuses a 2.0 key.
- **Grove's checks.** `release:formula:test`, `release:smoke` and
  `directory_route.rs` resolve the keg sources from the default directory.

The default directory is a narrow slice of Linkuistics/Ravel#4. The universal
provider and the configuration of hooks stay there.
