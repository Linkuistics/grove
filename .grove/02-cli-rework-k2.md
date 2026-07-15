# cli-rework-k2

**Kind:** work

## Goal

Strip all git-topology handling from the grove CLI per ADR
*user-owned-worktrees*: `grove do` becomes argument-less (run from inside the
working tree), `grove retire` takes `<node-path>` only, and every line of
worktree/branch creation, attach, and layout-derivation code is deleted.

## Context

Decisions: `.grove/01-plan-k1.md` running log; ADRs *user-owned-worktrees*,
*do-is-sole-lifecycle-verb*. Sites (verified during planning):

- `src/cli.rs` — `Do(StartArgs)` loses `name` and `--start-point`;
  `RetireArgs.path` becomes a bare node path; `MODEL_ENV_HELP` example says
  `grove do <name>`; dead `NameArgs`.
- `src/launch.rs` — `do_grove` loses the create/attach dispatch arms (cwd's
  toplevel *is* the worktree); unwired `start`/`continue_grove` remnants
  deleted; `retire` stops splitting `<name>/<node-path>`; grove name =
  toplevel basename.
- `src/repo.rs` — `create_grove_worktree`, `attach_grove_worktree`,
  `default_start_point`, `grove_worktree`, `grove_worktrees_dir`,
  `branch_exists` all die; `resolve` + `git_toplevel` survive (harness stamps
  and session naming still key off the main repo).
- `src/loop_driver.rs` — takes the worktree from cwd/toplevel; name derived
  from its basename.
- `src/tree_lifecycle.rs` — `grove_name` doc comment drops the
  branch-equality claim (code already basename-based).
- Tests: `tests/launch.rs` et al. drive via the kept `--no-launch` seam
  (provision + adoption-migrate + report, no exec); tests now create their
  scratch worktrees themselves.

## Done when

`cargo test` green; `grove do` (argument-less) runs from any git working tree
— linked worktree or main checkout, any branch; `grove retire <node-path>`
works in-worktree; `grep -rn "grove-worktrees\|start_point\|branch_exists"
src/` returns nothing load-bearing.

## Notes

Breaking CLI change — the release leaf bumps the major version.
