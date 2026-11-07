# native-marketplace-retirement-k235

## Goal

Retire Grove's `linkuistics` marketplace listing. Measure what a refresh of
the retired listing does to an installed plugin. Write the release notes for
its users.

## Context

- "The retired marketplace" of `docs/specs/codex-skill-provisioning.md`, and
  its rows "Retired listing" of "Test seams".
- "Native users" and "Frozen registration" in the running log of
  `homebrew-delivery-design-k216`.
- The formulae of `personal-formula-k232`, `third-party-skill-formulae-k233`
  and `testanyware-skill-formula-k234` exist before this leaf.

## Done when

- `.claude-plugin/marketplace.json` keeps the name `linkuistics` and an
  empty `plugins` array. Its description says that the marketplace is
  retired, and points to Homebrew and Ravel. `claude plugin validate`
  accepts it with one warning.
- No remote marketplace that Grove owns lists a pack.
- An isolated Claude Code home adds the `linkuistics` marketplace from a Git
  remote at a commit that lists the packs, and installs `grove@linkuistics`.
  A refresh against a commit with the retired listing shows whether the
  plugin stays installed. The evidence document records the reading.
- The Unreleased section of `CHANGELOG.md` gives the four steps for a user of
  the marketplace, and the reading of the refresh.
- Each test that read the marketplace's pack entries reads its new source or
  goes.
- `bash scripts/check.sh` passes.

## Notes

The Git remote of the measurement can be a local bare repository. Do not push
a retired listing to a remote that users read before finish.

Nothing here refreshes or updates the owner's frozen `linkuistics`
registration.

## Decisions (running log)

**Retired file (2026-11-07).** `.claude-plugin/marketplace.json` keeps the
name `linkuistics` and the owner, with an empty `plugins` array. Its
description says that the marketplace is retired, names
`brew install linkuistics/taps/grove` and Ravel 2.0's default skill
directory, and points to the CHANGELOG. `claude plugin validate` accepts it
with one warning: "Marketplace has no plugins defined".

**Remote marketplaces (2026-11-07).** After `git fetch`, only Grove's
`origin/main` (`238c3a839ab4`) holds marketplace files: the root Claude Code
file with ten plugins and the root Codex file `linkuistics-hooks`. The six
pack repositories and TestAnyware hold none on `origin/main`. Grove's file
list is the positive control. This branch removed the Codex file in
`grove-hook-adapters-k230` and empties the Claude Code file, so finish's move
of `main` leaves no remote listing of a pack. Nothing is pushed before finish.

**Measurement route (2026-11-07).** Claude Code refuses a `file://` source.
A Git `insteadOf` rule in each scratch home sends both GitHub URL forms of
`Linkuistics/grove` to a local bare repository. The user's own command then
clones it, and the registration records the source `github`. The first
commit has the tree of Grove `3cf8dfc702fb`, which the script checks.

**Reading (2026-11-07).** A refresh succeeds. `grove@linkuistics` stays
installed and enabled, but it fails to load with "Plugin grove not found in
marketplace linkuistics". Sessions get no `grove:` skill, `plugin update`
fails, and an interactive start shows no notice. Removal uninstalls the
plugin. Without a sign-in, `autoUpdate: true` refreshed nothing, which is a
reading without a result.

**The owner's registration (2026-11-07).** A read-only look at the owner's
`known_marketplaces.json` found `autoUpdate: true` for `linkuistics`. Its
`lastUpdated` is 04:33:28Z, 47 seconds after this session's Claude Code
process started, and its clone is at `238c3a839ab4`. A signed-in launch
therefore refreshes it. The owner design already freezes automatic updates
from the preparation step. The moves record's "For rollout" now names the
consequence of a missing freeze. Nothing changed the owner's settings.

**Tests (2026-11-07).** No test reads the current listing. `check:packs`
reads `plugins/packs.json`. The release tests read the keg marketplace files
of `assets/keg-root`. `test_harness_trial.py` writes its own fake listing.
The three `*-native-namespace` cases stay. The runner reads the listing of
each trial's prepared commit, and the acceptance record names those commits.
At a commit after this leaf, the runner refuses them, as it refuses any case
whose supplier lacks a path. The runner's docstring says so.

**Formula caveats (2026-11-07).** The caveats told users to add
`Linkuistics/grove` and turn on its auto-update. This leaf removed those
lines. The installer lines and the settings route belong to k236 and k237.
k237's task file now names the caveats.
