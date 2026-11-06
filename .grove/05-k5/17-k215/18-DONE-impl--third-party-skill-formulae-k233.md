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

## Decisions (running log)

**The superpowers pin (2026-11-07).** The formula keeps
`bb92a77741419a4ab5f06e711a283343f1ada0c3`, the commit of tag `v7.0.0`. Its
version is `7.0.0`. Upstream has since released `v7.0.1` at `2ce8aa15d05b`
and `v7.0.2` at `22f0b463e7a5`. Both keep the fifteen skills.

The owner's
native plugin is 7.0.0 at `bb92a777`, and "The owner's classes" of
`docs/specs/owner-skill-delivery.md` selects Superpowers 7.0.0. The switch
therefore changes the route of the skills and not their text. A later advance
is one formula change after the cutover.

**The quint-llm-kit pin (2026-11-07).** The formula keeps
`cc75369f741af7d490936f82002c2d28e3b3d78d`, which is still the head of
`main`. The repository has no release and no tag. Its version is `1.0.0`, the
version of the plugin manifest at that commit. The owner design names the same
version.

**No export-ignore (2026-11-07).** Neither repository has an `export-ignore`
attribute, so each GitHub source archive holds the files of the commit. The
relative paths in `quint-llm-kit-plugin` stay inside it.

**The superpowers keg (2026-11-07).** The install method copies each entry
of the build directory except `.brew_home`. Homebrew's stage creates that
directory there before the install method runs. The formula's test asserts
its absence.

**The shared test (2026-11-07).** `scripts/tap-formula.test.rb` now sets the
formula's `buildpath` and creates `.brew_home`, as Homebrew's stage does.
`scripts/tap-formula.test.sh` accepts `.` for the repository root. It sorted
its keg list by path and its commit list by mode, so any executable file in
a source failed the comparison. Both lists now sort as whole lines. The
`personal` source has no executable file, so `personal-formula-k232` did not
meet the defect.
