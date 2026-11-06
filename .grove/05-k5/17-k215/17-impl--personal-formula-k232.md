# personal-formula-k232

## Goal

Publish the `personal` formula in the Linkuistics tap. It installs the
`personal` source and the `guardrail` plugin with its Claude Code marketplace
file.

## Context

- The tap is `~/Development/Delivery/homebrew-taps`. Read its own
  instructions first.
- "The keg layout" and "Hook adapters" of
  `docs/specs/codex-skill-provisioning.md`. "Skill formulae" of
  `docs/specs/owner-skill-delivery.md`.
- PersonalSkills holds `plugins/personal` with the root `craft`, and
  `plugins/guardrail`. The commit comes from `pack-homebrew-commits-k231`.

## Done when

- `Formula/personal.rb` pins that PersonalSkills commit through a GitHub
  source archive and its `sha256`. Its version is the date of the commit.
- The keg holds `share/ravel/skills/personal` as a copy of
  `plugins/personal`, and `libexec/plugins/guardrail` as a copy of
  `plugins/guardrail`. Each copy keeps executable bits.
- The keg root holds `.claude-plugin/marketplace.json` with the name
  `personal`. It lists `guardrail` from `./libexec/plugins/guardrail`.
  `claude plugin validate` accepts it.
- An installation in a private prefix gives a `personal` source whose
  `resourceDigest` equals that of a clean plain checkout of the commit. A
  Ravel 2.0 build with a record that names the prefix's `share/ravel/skills`
  resolves it with the root `craft`.
- In an isolated Claude Code home, the documented commands register the
  marketplace and install `guardrail@personal`.
- The tap's own checks pass, such as `brew audit` and `brew style`. The
  formula is pushed to the tap's remote.

## Notes

Installing the formula in the owner's real prefix belongs to
`owner-pack-checkouts-k93`. The owner design states that an installation
changes no HOME and no discovery.
