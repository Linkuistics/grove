# pack-homebrew-commits-k231

## Goal

Publish one commit in each of the six pack repositories that describes
Homebrew installation and Ravel delivery. Record the five domain commits in
Grove's record of pack commits.

## Context

- The packs are RequirementsSkills, DesignSkills, EngineeringSkills,
  ResearchSkills, WritingSkills and PersonalSkills, under
  `~/Development/AgenticOrchestration/`. Read each repository's own
  instructions first.
- "The keg layout", "Hook adapters" and "The retired marketplace" of
  `docs/specs/codex-skill-provisioning.md`. The section "Names" in the
  running log of `homebrew-delivery-design-k216`.
- "The checker without a copy" of `docs/specs/external-pack-contract.md`
  states how a pack commit records `GROVE_COMMIT`, and the push order.
- The record comes from `grove-pack-record-k227`. Grove's keg holds the
  Codex marketplace file `writing-hooks` after `grove-hook-adapters-k230`.

## Done when

- Each pack's README names its formula: `grove` for the five domain packs,
  `personal` for PersonalSkills. It states that Ravel delivers the skills,
  and it gives no native marketplace or installer step. WritingSkills'
  README and PersonalSkills' README give the documented hook registration
  commands for their adapters.
- WritingSkills removes its root `.agents/plugins/marketplace.json` and each
  reference to it.
- PersonalSkills' `authoring-conventions` says "the Linkuistics skill packs"
  in place of "this marketplace". It states that no installer reads the
  `harnesses:` key.
- Each pack passes its own `task check`.
- If a pack commit records a new `GROVE_COMMIT`, the Grove line is pushed
  through jj before that pack commit publishes.
- Each pack commit is on its public remote.
- `plugins/packs.json` names the five new domain commits. `task check:packs`
  passes, and `bash scripts/check.sh` passes.
- `docs/repository-moves-record.md` records each new commit.

## Notes

If six repositories do not fit one session, decompose by repository. Do
WritingSkills and PersonalSkills first, because later leaves consume them.

The personal formula pins the PersonalSkills commit of this leaf, in
`personal-formula-k232`.
