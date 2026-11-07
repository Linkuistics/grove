# owner-settings-drafts-k246

## Goal

Draft and commit the owner's Ravel skill settings and the mise declarations
of the skill formulae. No draft changes the real HOME.

## Context

- Order 4 of the cutover contract in `docs/specs/owner-skill-delivery.md`
  lists these drafts. The parent brief's "Owner-delivery handoff" states
  what they hold.
- The off value is `{"schemaVersion": 1, "sources": []}` under the `skills`
  key. Step 3 of the switch writes it inside finish, and the inverse returns
  to it.
- The switched value names no `sources` and holds only the mandatory file,
  superpowers `skills/using-superpowers/SKILL.md`. The section "Mandatory
  guidance" states why.
- The staging settings name each of the ten sources in `sources`. Grove's
  six sources come from the candidate keg of `candidate-keg-k238`. The four
  others come from their installed kegs. They name no versioned keg path.
  They hold the same mandatory file. The section "Settings isolation and the
  staging HOME" states them.
- The section "Skill formulae" states the five `[bootstrap.packages]`
  declarations. owner-pack-checkouts-k93 writes them into the real
  `~/.config/mise/config.toml`.
- Ravel's skill discovery specification on the line `skill-directory-delivery`
  owns the settings format. Read it at the build that
  `scripts/harness-trials/builds.json` names.

## Done when

- A committed drafts folder holds the off value, the switched value, the
  staging settings and the declarations. Choose its place, such as
  `docs/owner-cutover/`, and name it in the evidence document.
- The staging settings name the intended path of the candidate keg. The
  staging setup builds the keg at that path.
- In a disposable HOME, the accepted build parses each settings draft. With
  the off value and a default skill directory that holds the ten sources,
  `ravel skills resolve --json` reports `disabled`. With the switched value,
  it resolves the ten sources and reads the mandatory file.
- No draft launches a harness session.
- The real HOME's `~/.config/ravel/settings.json` is unchanged or absent.

## Notes

A 1.1 launch refuses settings that hold the `skills` key. Keep every draft
outside the real `~/.config/ravel` until finish.

The real switch plan takes its settings cells from the off value and the
switched value. owner-switch-plan-draft-k247 consumes these drafts.
