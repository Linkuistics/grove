# thegreatexplainer-relocation-k168

## Goal

Move `~/Development/AgenticWriting/TheGreatExplainer` to
`~/Development/AgenticOrchestration/TheGreatExplainer` by the steps of the
parent brief.

## Context

At planning, TheGreatExplainer was colocated Git and jj with one jj workspace
and an empty working copy. Git `main` and `main@origin` were at the same
commit. Its remote is `https://github.com/Linkuistics/TheGreatExplainer.git`.
It holds `docs/` and `website/`, and no Taskfile.

## Done when

- The parent brief's Done when holds for TheGreatExplainer.
- Any website build that the project documents still runs at the new path,
  or the record states that the project documents none.

## Decisions (running log)

- The project documents no website build. `website/` holds only content and
  `meta.yml`, and the README and the TODO name no build. No build ran.
- TheGreatExplainer has no checks to run. It is in the requirements phase,
  with no Taskfile, manifest or test.
- No reference outside this grove's task notes and session history names the
  old path. The rollout text for paused groves needs no entry for this move.
