# third-party-skill-formulae-k233

## Goal

Publish the `superpowers` and `quint-llm-kit` formulae in the Linkuistics
tap.

## Context

- The tap is `~/Development/Delivery/homebrew-taps`.
- "Other formulae" of "The keg layout" in
  `docs/specs/codex-skill-provisioning.md`. "Ravel sources" of
  `docs/specs/owner-skill-delivery.md` names the source IDs, the roots and
  the first commits.

## Done when

- The formula author reads both upstream pins again. The superpowers commit
  is `bb92a77741419a4ab5f06e711a283343f1ada0c3` and the quint-llm-kit commit
  is `cc75369f741af7d490936f82002c2d28e3b3d78d`, or the leaf records each new
  commit and its reason.
- Each formula pins its commit through a GitHub source archive and its
  `sha256`. Its version is the upstream release of that commit.
- The `superpowers` keg holds the repository root at
  `share/ravel/skills/superpowers`, so `skills/using-superpowers/SKILL.md`
  stays at its path. The `quint-llm-kit` keg holds `quint-llm-kit-plugin` at
  `share/ravel/skills/quint`, with the root `skills`.
- An installation of each in a private prefix gives a source whose
  `resourceDigest` equals that of the same subtree of a clean plain
  checkout. A Ravel 2.0 build with a record resolves both sources.
- Each formula declares the upstream licence.
- The tap's own checks pass. Both formulae are pushed to the tap's remote.
