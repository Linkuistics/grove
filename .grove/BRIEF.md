# install-and-update-create-commits — root brief

## Goal
Two coupled deliverables:
1. Make `grove install` and `grove update` create a git commit for the materialisation by default, removing the manual `git add` + `git commit` step. Behavior is fully specified by [ADR-0001](../docs/adr/0001-install-and-update-create-commits.md); see `CONTEXT.md` for terminology.
2. Add user-facing lifecycle walkthrough documentation for grove's main flows (install, update, start, multi-step grove, finish) — each showing the command, what happens, and what changes in the repo/worktree.

The two are paired here because the auto-commit changes the user-visible install/update story, and the walkthroughs are the place that story is told end-to-end. They can be implemented and shipped independently.

## Done when
- `grove install` and `grove update` produce one path-scoped commit per invocation covering all targeted harnesses, by default.
- `--no-commit` opts out cleanly and prints a follow-up staging command.
- `--message <text>` overrides the default message.
- Pre-existing staged hunks on install-scope paths cause an explicit error before materialisation; unrelated dirty state elsewhere is left untouched.
- A commit failure (hook reject, etc.) leaves the materialisation in place, exits non-zero, and prints a follow-up command.
- A no-op materialisation does not produce an empty commit.
- README and `grove --help` reflect the new install/update default.
- Lifecycle walkthroughs exist for: installing, updating, starting a grove, working through a multi-step grove, and finishing a grove. Each shows the command(s), explains what happens, and shows what changed in the repo/worktree.

## Decomposition
Numeric prefix = execution order. `010` first because the walkthroughs in `020` will demonstrate its new behavior.

- `010-implement-path-scoped-commit.md` — implement install/update auto-commit, tests, README + help updates.
- `020-plan-workflow-docs.md` — planning task: grill on doc structure (where they live, single file vs per-verb, narrative vs transcript style), then grow the tree with per-walkthrough leaves.

## Pointers
- ADR: `docs/adr/0001-install-and-update-create-commits.md`
- Glossary terms in play: **install scope**, **path-scoped commit** (see `CONTEXT.md`)
- Code surfaces: `src/install.rs`, `src/cli.rs::InstallArgs`, `src/repo.rs` (the existing `git` shell-out style)

## Notes
- Multi-harness invocations produce one combined commit, not one per harness.
- Default commit messages: `Install grove v<version>` / `Update grove to v<version>`.
- Empty-commit detection: after `git add`, check `git diff --cached --quiet -- <paths>`; if true, skip the commit and succeed silently.
- Pre-existing staged-hunk detection (refusal condition): before materialising, `git diff --cached --quiet -- <install-scope paths>` — if it returns non-zero, abort with an explicit error.
