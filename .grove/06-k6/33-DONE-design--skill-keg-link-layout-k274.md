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

## Decisions (running log)

The seam uses Homebrew's installed `Keg#link` and `Keg#unlink`, with only the
filesystem roots redirected to temporary prefixes, and the accepted Ravel 2.0
resolver. The direct-directory layout reproduced `skill_resource_invalid` in
both orders. A formula-owned directory symlink at each source entry preserved
the source through the same conflict resolution. A model would restate the
linker's filesystem behaviour; the real linker and resolver are the useful
instrument here.

Store source payloads privately and expose each
through a relative directory symlink at `share/ravel/skills/<source-id>` in
the same keg. Homebrew links that entry without descending into the payload.
Keep Ravel's source-contained-link rule. This preserves installation as the
whole skill setup, without post-install relinking of another formula.

All five formula owners must adopt the projection, with new formula revisions
or releases for installed users. During the mixed-layout transition, upgrade
every installed skill formula and verify resolution before resuming launches.
Relinking an affected old formula is recovery only. owner-pack-checkouts-k93
owns real-machine preparation; finish receives the final Grove installation
and source-acceptance obligations through discovery-rollout-k12. A planning
leaf will sequence the formula and acceptance work before k93. The durable
layout amendment earns a focused review of migration and test coverage before
that planning step; no in-session reviewer duplicates it.

The private path is `libexec/skill-sources/<source-id>`. Inspection of the
installed-smoke seam found that `libexec/ravel` and `libexec/skills` are
retired layouts whose absence remains checked. Avoid those namespaces rather
than weaken the independent-Ravel or no-provisioning checks. No new ADR is
needed: the storage path is readily reversible, and the spec holds its trade-off.
