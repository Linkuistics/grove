# stylepack-relocation-k166

## Goal

Move `~/Development/AgenticWriting/Stylepack` to
`~/Development/AgenticOrchestration/Stylepack` by the steps of the parent
brief.

## Context

At planning, Stylepack was colocated Git and jj with one jj workspace and an
empty working copy on `main`. Its remote is
`https://github.com/Linkuistics/Stylepack.git`. It has `.venv` and
`.pytest_cache`, and a `pyproject.toml` with `uv.lock`.

Its Git metadata registers a second worktree at
`~/Development/Stylepack/.grove-meta`, which `git worktree list` marks
prunable. Its directory no longer exists.

## Done when

- The parent brief's Done when holds for Stylepack.
- The prunable worktree registration has a recorded decision. Either keep it
  unchanged, or remove it with `git worktree prune` after the human states
  that it is obsolete. Do not prune it without that statement.
- The project's tests pass at the new path, after `uv sync` recreates the
  environment if the moved `.venv` fails.

## Notes

Read the project's own `CONTEXT.md` and README before you run its checks.

## Decisions (running log)

- The prunable worktree registration at `~/Development/Stylepack/.grove-meta`
  stays unchanged. No human stated that it is obsolete. The move does not
  touch it, because its `gitdir` names an earlier location.
- The moved `.venv` was already broken at the old path, from an earlier
  location. `uv sync --frozen --reinstall` recreated its scripts at the new
  path. `--frozen` keeps the tracked `uv.lock` unchanged.
- No reference outside the project needed a repair.
  `docs/repository-moves-record.md` holds the readings, the checks and the
  classes of hits.
