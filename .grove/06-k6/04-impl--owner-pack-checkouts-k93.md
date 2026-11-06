# owner-pack-checkouts-k93

## Goal

Install the owner's skill sources in the real HOME through Homebrew, at their
accepted commits. Cover each formula that publishes before finish. Make
`mac-check` report drift. Discovery stays native.

## Context

- skill-marketplace-decision-k201 replaced the mise checkouts with Homebrew
  installation. mise only declares the formulae, in `[bootstrap.packages]`.
  The root brief's "Skill installation" states the decisions.
- homebrew-delivery-design-k216 amends `docs/specs/owner-skill-delivery.md`.
  That document holds the contract and the real-HOME preparation of cutover
  order 6. The amended design gives this leaf its commands and its status
  report.
- Grove's formula carries core and the five domain packs. The personal pack,
  superpowers and quint-llm-kit have formulae of their own. TestAnyware's
  formula carries `using-testanyware`.
- The staging leaves of owner-ravel-skill-inventory-k38 supply the accepted
  commits and the accepted `resourceDigest` of each source. Read them from the
  staging evidence.

## Done when

- `~/.config/mise/config.toml` declares each skill formula in
  `[bootstrap.packages]`. No `[bootstrap.repos]` entry names a skill source.
- Homebrew installed the formulae of personal, superpowers, quint-llm-kit and
  TestAnyware at their accepted commits. Each of those sources gives its
  accepted `resourceDigest`.
- Claude Code registers the personal keg's marketplace file, and holds
  `guardrail` installed and disabled.
- `mac-check` reports each source's installed commit against its accepted
  commit. It tells drift from a configuration error, and a fixture control
  shows each outcome.
- The machine contract and the `mac-bootstrap` guidance state that each
  Homebrew installation or upgrade of a skill formula is an advance. The
  amended owner design states why an advance needs no process check.
- Native discovery is unchanged. The real HOME's Ravel settings name no skill
  source, and no harness root holds a new entry.

## Notes

Grove's keg holds core and the five packs only after Grove's release, which
happens inside finish. The amended owner design states what the switch checks
in that keg after the release.

An advance links a complete new keg, so a capture reads one keg or refuses.
`homebrew-delivery-design-k216` therefore withdrew the process check before an
advance.

owner-ravel-skill-inventory-k38 puts its leaves for policy conversion, drafts
and staging trials before this leaf with `leaf-insert`. The marketplace
freeze and the rehearsal on the real plan follow this leaf, also in order 6.
