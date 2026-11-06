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

## Decisions (running log)

**One session (2026-11-06).** The six changes are text and one file removal,
so they fit one session. WritingSkills and PersonalSkills come first.

**No new GROVE_COMMIT (2026-11-06).** No pack commit changes a kind skill or
the checker's inputs. Each pack keeps its recorded `GROVE_COMMIT`, which
Grove's public remote holds. No Grove push therefore has to come before a
pack push.

**Pack documents in scope (2026-11-06).** WritingSkills' user guide, its
specification and one decision record name the root Codex marketplace file.
Each of those passages changes to the keg's marketplace file. Dated records,
such as the harness survey and the evaluation records, keep their bytes. A
provenance sentence that says that a commit removed the file is history, not
a reference to a present file.

**Codex refresh (2026-11-06).** WritingSkills' user guide said that trust
stays after a refresh, because the entries never change shape. The delivery
specification's evidence says that Codex trust covers the plugin's bytes, and
that each upgrade needs both commands and the hook review again. The guide and
the specification of WritingSkills now follow the measured evidence.

**Gemini CLI (2026-11-06).** Ravel's skill-directory line names no Gemini CLI
harness. WritingSkills' documents therefore say that Gemini CLI finds no
skill by itself, and its user can point it to the skill directory.
