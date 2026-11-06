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

## Decisions (running log)

**The version (2026-11-06).** The version is `2026-11-06`, the ISO date of
commit `cddcfde901cd`. Its author and committer dates agree. `brew audit
--strict` accepts it.

**The marketplace file (2026-11-06).** PersonalSkills holds no keg-root
marketplace file, so the install method writes it from a Ruby constant.
Homebrew's `Pathname#write` creates no parent directory, so the method makes
`.claude-plugin` first.

**No dependency (2026-11-06).** The formula declares no dependency. Ravel reads
the source when it exists, and an installation without Ravel does no harm.

**A reusable test in Grove (2026-11-06).** The tap has no tooling of its own.
`third-party-skill-formulae-k233` and `testanyware-skill-formula-k234` need
the same measurement. Grove therefore holds `scripts/tap-formula.test.sh`, its
`brew ruby` harness and `task tap:formula:test`. Homebrew 7 loads no formula
file outside a tap through `Formulary`, so the harness loads the file and
constructs its class, as `release-formula.test.rb` does. The test models the
Homebrew prefix with an `opt` link and a relative `share/ravel/skills` link.

**The audit (2026-11-06).** `brew audit` accepts only a tapped name. The tap
clone under Homebrew was fast-forwarded to the local tap commit before the
push, so the audit read the pushed bytes.

**A side effect (2026-11-06).** A first run set
`HOMEBREW_NO_INSTALL_FROM_API=1`, and Homebrew cloned `homebrew/core` into
the real Homebrew repository. The test no longer sets the variable. The
removal of that tap was refused by the session's permission classifier, so
the tap remains for the owner to remove with `brew untap homebrew/core`.
