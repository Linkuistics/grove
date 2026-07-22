# skills.use-jujutsu-when-possible — brief

## Goal

Make agents use Jujutsu (jj) instead of git whenever a repo permits it. Two new
skills in `plugins/linkuistics/skills/`, plus a reconciliation pass over the
existing skills that mention git:

1. **Workflow skill** — auto-fires on VCS work; detects whether the repo is
   jj-enabled and, if so, drives all version control through jj's *native*
   model (working-copy-as-commit, `jj new`/`jj describe`, bookmarks, op-log
   undo) rather than git-brained command substitution.
2. **Mapping skill** — an on-demand git→jj reference (command/concept mapping)
   the workflow skill or the user can pull in when translation is needed.

## Done when

- Both skills exist, follow the house `authoring-conventions`, and are listed
  in the README skill table.
- Existing skills that mention git (`guardrail`, `decision-records`,
  `cli-tool-design`) are reconciled so their guidance holds in jj-enabled
  repos too.
- Prior art has been surveyed first (`docs/research/`); adopt or adapt beats
  rewrite.

## Decomposition

- `jj-prior-art-k2` — research: survey existing jj skills / agent-jj
  configurations before designing.
- `skill-design-k3` — planning: digest research, settle names / triggers /
  harness scope, grow the work leaves.

## Pointers

- Settled behaviour semantics (see CONTEXT.md for terms):
  - **jj-enabled repo** (`.jj/` present) → jj is the primary VCS interface.
  - jj installed but repo not jj-enabled → offer `jj git init --colocate`
    once per session; never convert silently.
  - no jj binary → skills stay silent.
- Reconciliation surface (as of planning): `guardrail` (git commands in its
  destructive-pattern list), `decision-records` ("git holds the history"
  phrasing), `cli-tool-design` (glancing mention).

## Notes

- jj 0.43.0 is installed locally; this worktree itself is plain git (no
  `.jj/`).
