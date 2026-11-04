# Repository moves record

This record holds the results of the moves under `repository-moves-k5`. Each
move adds one section. A section names the source and the destination, the
version-control arrangement, the readings before and after, the checks and the
references. `moves-handoff-k187` reads this record, and `discovery-rollout-k6`
uses it to integrate and release the products.

The readings are those of the Done when of `writing-projects-k165`. A script
prints them with the repository root replaced by `<ROOT>`. A comparison then
ignores the change of root and of column padding.

## Stylepack

`stylepack-relocation-k166` moved `~/Development/AgenticWriting/Stylepack` to
`~/Development/AgenticOrchestration/Stylepack` on 2026-11-04. The move changed
no file in the project, so it has no owning-repository commit.

### Arrangement

- Stylepack is a colocated Git and Jujutsu repository.
- It has one jj workspace, `default`. Its working copy is empty, on `main`.
- `main` and `origin/main` are both at `377daaa28e55`. This published commit
  is the one that a pin can name.
- The remote `origin` is `https://github.com/Linkuistics/Stylepack.git`. It
  also carries the bookmark `grove-meta@origin` at `d66019ba08fa`.
- `.jj/repo/store/git_target` holds the relative path `../../../.git`.
- `.git/config` sets no `core.worktree`.
- A search of `.jj/` and `.git/` outside the object stores found no absolute
  path of the root.

### Stale worktree registration

Git registers a second worktree at `~/Development/Stylepack/.grove-meta`.
`git worktree list` marks it prunable, because that directory does not exist.
Its `gitdir` file names `~/Development/Stylepack`, an earlier location of the
project. The move did not touch this registration.

The registration stays unchanged. No human stated that it is obsolete, and the
leaf permits a prune only after that statement. When the human makes the
statement, run `git worktree prune` in the project.

### Readings

The readings before and after the move are identical.

- `jj log -r 'all()'` shows the same 118 visible commits.
- `jj st`, `jj workspace list`, `git worktree list` and `git remote -v` give
  the same output.
- `git status --porcelain --ignored` gives the same ignored entries and no
  other entries.
- The 252 tracked files have the same digest,
  `1866e2577250c7c9ba430caf3cd93d1ceac75cc15039c08029bbbf03ee2d069a`.

Before the move, `lsof +D` found no open file under the project. No process
other than this session's launcher named the project.

### Checks

The moved `.venv` was already broken before the move. Its console scripts
named `~/Development/Stylepack/.venv/bin/python`, from the earlier location.
At the old path, `uv run --frozen pytest` stopped with 42 collection errors.
Each error was a `ModuleNotFoundError`.

At the new path, `uv sync --frozen` alone kept the old script headers.
`uv sync --frozen --reinstall` rewrote them. Then `uv run --frozen pytest`
gave 1366 passed and 4 skipped, with exit status 0. The `--frozen` flag kept
the tracked `uv.lock` unchanged.

The test run added four ignored `__pycache__` directories. They are
regenerated tool state, not product content. The tracked-file digest did not
change.

### References

The search for the parent name `AgenticWriting` covered the following
locations, without ignore rules.

- `~/.claude.json`
- `~/.claude/` and `~/.codex/`
- `~/.config/mise`
- Every repository under `~/Development`, which includes the project

The search found the name once in this grove's root brief, which is the
positive control. A mutated pattern found nothing there.

No hit is a reference to the old path of Stylepack that needs a repair. The
hits are of four classes.

- Claude Code and Codex session transcripts and tool output are history.
- This grove's task notes describe the moves.
- `ResearchSkills/.cache/grove/` holds a cached copy of this grove's tree.
- `rhai-book-use-k74` in three Ravel workspaces names the old path of
  TestSubject, not of Stylepack.

The rollout text for paused groves needs no entry for Stylepack. The Obsidian
manual `AI Dev Tooling` does not name Stylepack or `AgenticWriting`, so it needs
no change.
