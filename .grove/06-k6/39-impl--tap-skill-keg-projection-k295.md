# tap-skill-keg-projection-k295

## Goal

Revise the tap's `personal`, `superpowers` and `quint-llm-kit` formulae to
the projected keg layout, at their unchanged source commits. Adapt
`tap:formula:test` to the layout and pass it for each formula. Commit the
tap change without a push.

## Context

- The tap working copy is `~/Development/Delivery/homebrew-taps`. Its
  formulae install real directories under `share/ravel/skills` today.
- "The keg layout" and "Other formulae" of
  `docs/specs/codex-skill-provisioning.md` own the layout and the narrowest
  subtree of each third-party source.
- "Skill formulae" of `docs/specs/owner-skill-delivery.md` owns the full
  package version and packaging-only acceptance.
- `scripts/tap-formula.test.sh` asserts that each source lies at
  `share/ravel/skills/<id>` in the keg, and builds the `brew link` links
  itself.
- `personal` also installs `libexec/plugins/guardrail` and a Claude Code
  marketplace file at the keg root.

## Done when

- Each formula moves its source into `libexec/skill-sources/<source-id>/`
  and creates the relative alias at `share/ravel/skills/<source-id>`. Each
  adds a formula `revision`, so an ordinary upgrade installs the remedy.
  Source URLs, `sha256` values and upstream versions stay unchanged.
- `tap-formula.test.sh` asserts the alias, the canonical private payload
  and regular payload files. It compares each source with its plain
  checkout as before. It runs the shared-directory seam of
  shared-directory-seam-k294 for the formula. Until
  testanyware-keg-projection-k296, `testanyware` takes part as an
  old-layout formula.
- The test also compares the complete `guardrail` payload of the revised
  `personal` keg with its accepted subject. It covers files, modes, links
  and hook declarations.
- `task tap:formula:test` passes for each revised formula. Each run records
  the full package version, including its revision.
- The tap holds one local commit with the three revisions. It is not on
  the tap's pushed `main`, so no user and no release task sees it yet.

## Notes

skill-formula-publication-k298 pushes this commit together with the
TestAnyware revision. It does so after the all-five seam and the
hook-adapter seam pass.

The tap test enforces the layout for every later edit of these formulae.
These three have no producer release, so that test is their protection.

A source digest that differs from its accepted value needs that source's
staging trials. That is not expected here, because the commits do not
change. If it happens, stop and record it rather than accepting it.

shared-directory-seam-k294 delivered the seam. Change the formula's `layout`
in `scripts/skill-formulae.json` to `projected` with its revision. The seam
refuses a keg that differs from its declaration. `task tap:formula:test` now
also needs `RAVEL_FORMULA` and `RAVEL_ARCHIVE`, and its seam reads the
published formula from the tap's `origin/main` as the migration predecessor.
The revision needs a new package version, or the seam refuses the pair.
