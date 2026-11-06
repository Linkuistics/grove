# homebrew-skill-delivery-k215 — brief

## Goal

Replace native marketplace distribution of the packs with Homebrew
installation and Ravel delivery, as `skill-marketplace-decision-k201`
settled. Homebrew installs every Linkuistics skill source, the owner's
machine included. Ravel is the only route that delivers skills to a session.

## Done when

- Grove's formula installs core and the five domain packs: requirements,
  design, engineering, research and writing. Each pack is at the commit that
  Grove's pack check accepted.
- Ravel 2.0.0 reads each of those sources from its default skill directory,
  `$(brew --prefix)/share/ravel/skills/`, with no settings. `brew install` and
  `brew upgrade` are the whole skill setup.
- Grove keeps its own record of the accepted pack commits, with the fields
  `url`, `path` and `sha`. `task check:packs` reads that record. The release
  takes the formula's pack commits from it.
- The personal pack, superpowers and quint-llm-kit each have a formula in the
  Linkuistics tap. TestAnyware's formula installs `using-testanyware` in its
  keg.
- Grove's keg holds a Claude Code marketplace file and a Codex marketplace
  file for the writing hooks, and the Pi extension. The personal keg holds a
  Claude Code marketplace file for `guardrail`.
- Each of those marketplace files has a name other than `linkuistics`. One
  documented command for each harness registers each file by its Homebrew
  path. Codex repeats its command after each Grove upgrade, because it stores
  the versioned keg path.
- No remote marketplace lists a pack, and the manual installer links no
  skill. Grove's guidance, the six pack READMEs and TestAnyware's README
  describe Homebrew installation and Ravel delivery.
- The owner design installs the owner's sources through formulae that mise's
  `[bootstrap.packages]` declares, in place of the `[bootstrap.repos]`
  checkouts. Finish installs the writing hooks plugin, disabled, after the
  release and before the backup of the switch.
- The external pack contract, the delivery specification and the
  pack-membership design agree with the decision of k201. So do the decision
  record "Homebrew installs skills and Ravel delivers them", the glossary and
  the moves record.
- The six seams of the root brief's "Agreed test seams" pass. Grove's
  `bash scripts/check.sh` passes.

## Decomposition

1. `homebrew-delivery-design` (design): amend the specifications and the
   decision records, and settle the questions below.
2. `ravel-default-skill-directory` (design): Ravel 2.0.0's default skill
   directory and its effect on the owner switch. If the two designs earn a
   review chain, cut it with `leaf-insert` before the planning leaf.
3. `homebrew-delivery-plan` (planning): cut the implementation leaves from
   the settled design, in Grove, Ravel and the packs. Each Grove leaf leaves
   `bash scripts/check.sh` green.

## Questions for the design

`homebrew-delivery-design-k216` answered these in the delivery specification,
the owner delivery design and the decision record. Its running log holds the
answers and the human's two later decisions.

- Layout: the folder of core and of each pack in a keg, and the stable path
  that Ravel names. Linkuistics/Ravel#4 may later find installed skills by
  location, so prefer a layout that a provider can scan.
- Identity: Homebrew's cleaner makes installed files read-only. Decide how
  the accepted `resourceDigest` of a keg relates to that of a Git checkout,
  and what the staging trials read.
- Lifetime: Homebrew can remove an earlier keg after an upgrade. A local
  marketplace loads its plugin in place. Measure a hook plugin across
  `brew upgrade`.
- Release: how Grove's release takes the pack commits from Grove's record
  into the formula. How a change to a pack reaches a Grove patch release.
- Timing: Grove's keg holds core and the five packs only after Grove's
  release, inside finish. Decide what the staging trials and cutover order 6
  read before that release. Decide what the switch checks in the keg after
  it.
- Rollback: the owner design's last native return installs the current
  marketplace plugins. No remote marketplace lists a pack after this work, so
  state the return that replaces it.
- Native users: what Grove's `.claude-plugin/` holds after finish. What the
  release notes tell a user of the `linkuistics` marketplace.
- The owner's frozen registration: finish reads its own guidance from the
  plugins of the owner's `linkuistics` registration. That registration stays
  frozen until purge, and the switch must not update it.
- Wording: `authoring-conventions` says "this marketplace". The phrase loses
  its referent, as the PersonalSkills section of the moves record states.

## Notes

The running log of `skill-marketplace-decision-k201` holds each decision in
the human's words, and the comparison of the routes.

Linkuistics/Ravel#4 holds the later universal skills provider and the
"super-skill" format. Linkuistics/grove#28 holds the single-repository
reorganisation. This node builds neither.

The pruned `skill-marketplace-k185` noted one fact that still holds. Native
users see the marketplace only through Grove's `main`, which moves in finish.
The intermediate states of this work therefore never reach them. Only the
final state must be complete.
