# writing-projects-k165 — brief

## Goal

Move `Stylepack`, `TestSubject`, `TheGreatExplainer` and `Writegood` from
`~/Development/AgenticWriting/` to `~/Development/AgenticOrchestration/`. Keep
each project's contents, history, working-copy changes and workspace
registrations. Change no product behaviour.

## Done when

- Each project is at `~/Development/AgenticOrchestration/<name>`, and nothing
  is left at its old path.
- Before and after each move, the same commands give the same results:
  `jj log` of all visible commits, `jj st`, `jj workspace list`,
  `git worktree list`, `git remote -v`, `git status --porcelain --ignored`
  and a digest of every tracked file.
- Each registration that names the old path is repaired with a supported jj
  or Git command, or the record states why it stays. Each registration that
  names the new path resolves.
- The project's own checks pass at the new path where it has checks.
  Regenerated tool state, such as a Python `.venv`, is not product content.
- Each reference to the old path outside the project is reconciled or
  recorded. Do not edit another grove's task tree. Record each such reference
  for rollout's paused-grove text.
- The record of each move is in `docs/repository-moves-record.md` of this
  repository. moves-handoff-k187 reads it.

## Decomposition

One leaf for each project. The leaves are independent, and each leaves a
working project. Their order is alphabetical.

## Notes

Each leaf follows the same steps. The leaf states only what differs.

1. Check that no process uses the project: no harness session, no Grove
   driver and no editor. Check with `lsof +D` and a process list.
2. Check that the destination does not exist.
3. Record the "before" readings of Done when.
4. Move the directory with one `mv` on the same file system. A colocated jj
   repository holds no absolute path of its own root. Check this fact for
   each project before you rely on it.
5. Record the "after" readings, and compare them with the "before" readings.
6. Search for the old path. Search the project, the owner's harness
   configuration (`~/.claude.json`, `~/.claude/`, `~/.codex/`), the mise
   configuration and the other repositories under `~/Development`. Shell
   history is out of scope. Use a positive control: the same search must find
   the old path in this grove's root brief.
7. Repair or record each reference.

The machine contract, through `linkuistics:machine-environment`, owns a
change to persistent configuration. The maintained manual is the Obsidian
`AI Dev Tooling` note. Record a manual change there when a project is named
in it.

`~/Development/AgenticWriting/` can be left empty. Do not delete it unless the
human asks.
