# skill-keg-link-layout-k274

## Goal

Make every skill formula's source reach Ravel's default skill directory as
one directory link. The installation order must not matter.

## Context

- owner-staging-setup-k248 installed `personal`, `superpowers` and
  `quint-llm-kit` after `testanyware` 2.8.0. TestAnyware was then the only
  formula with `share/ravel`, so Homebrew linked the whole directory.
  The next link made Homebrew run `Keg#resolve_any_conflicts`, which relinks
  the displaced keg with `link_dir(src) { :mkpath }`. Every directory of
  TestAnyware's source became a real directory, and every file became a
  relative link that leaves the source.
- Ravel 2.0 then refused the whole resolution as `skill_resource_invalid`:
  "a snapshot keeps only regular files, directories and relative links that
  stay inside the source". `brew unlink testanyware && brew link testanyware`
  restored one directory link, and the ten digests then matched.
- `docs/specs/codex-skill-provisioning.md`, in "The keg layout", says that
  Homebrew "replaces the first keg's directory link by a real directory and
  links each entry". That claim is wrong for the first skill formula linked
  on any machine. Homebrew 7.0.12 at `3ccfa1d07143`, `Library/Homebrew/keg.rb`,
  holds the code.
- A launch with default sources refuses as a whole. A new machine's
  bootstrap installs the formulae one by one, so every Grove launch there
  can fail.
  The owner's machine is repaired, but `mac-bootstrap` on a new machine and
  any later reinstall can repeat the state.
- The section "Staging setup and isolation trial" of
  `docs/owner-skill-delivery-evidence.md` records the observation.

## Done when

- The keg-layout section states Homebrew's actual behaviour, with its source.
- The design names a remedy and its owner. One candidate is a formula step
  or a documented relink in the installation commands. Others are a
  `mac-check` report of the state, or a Ravel rule for links inside one keg.
- A test seam reproduces the state with two fixture kegs and shows the
  remedy, or the design records why no seam can reproduce it.
- owner-pack-checkouts-k93 and the finish handoff receive the remedy.

## Notes

The staging trials do not depend on this leaf, because owner-staging-setup-k248
relinked TestAnyware. The real switch's source verification reads the
default directory. The remedy must therefore exist before the switch.
