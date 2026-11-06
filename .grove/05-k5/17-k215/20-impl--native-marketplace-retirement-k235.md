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
