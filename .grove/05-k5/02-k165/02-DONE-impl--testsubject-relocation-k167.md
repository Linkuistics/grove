# testsubject-relocation-k167

## Goal

Move `~/Development/AgenticWriting/TestSubject` to
`~/Development/AgenticOrchestration/TestSubject` by the steps of the parent
brief.

## Context

At planning, TestSubject was colocated Git and jj with one jj workspace and an
empty working copy. Git `main` and `main@origin` were at the same commit. Its
remote is `https://github.com/Linkuistics/TestSubject.git`. It holds a
`website/` directory and no Taskfile.

## Done when

- The parent brief's Done when holds for TestSubject.
- Any website build that the project documents still runs at the new path,
  or the record states that the project documents none.

## Decisions (running log)

- The project documents no website build. `website/` holds only `index.md`
  and `meta.yml`, and the README and the TODO name no build. No build ran.
- TestSubject has no checks to run. It is pre-development, with no Taskfile,
  manifest or test.
- `rhai-book-use-k74` in Ravel's task tree names the old path. It stays
  unchanged, because this grove does not edit another grove's task tree.
  `docs/repository-moves-record.md` carries the entry for rollout's
  paused-grove text.
