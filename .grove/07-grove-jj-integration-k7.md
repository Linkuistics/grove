# grove-jj-integration-k7

**Kind:** planning

## Goal

Grill grove-side jj support and grow whatever leaves it needs. The skills
(`using-jujutsu`, `git-to-jj-mapping`) are the baseline: what must grove
itself change so grove-driven sessions work first-class in jj-enabled
working trees?

## Context

- **House rule stands:** all grove code/doc edits and their commits land in
  the Linkuistics/grove repo (`~/Development/grove`) — this leaf lives here
  only to *track* the work inside the workstream that motivated it
  (settled in `03-DONE-skill-design-k3.md`'s running log, Q9b).
- Known git-specific surface in grove, to grill through (verify against
  the current grove source, don't trust this list):
  - `SKILL.md`/prompt prose: "git is the history", "one focused commit",
    `git rev-parse` for repo/worktree names, "provide a working tree via
    git init / clone / worktree".
  - `grove-llm` verbs shell out to git: `leaf-decompose`/`leaf-insert` do
    `git mv`; renumber relies on it.
  - `grove do`'s launch flow and the user-owned-worktrees ADR assume git
    working trees; jj's analogue is `jj workspace`.
  - Session-naming probe: `git rev-parse --git-common-dir` for the main
    repo's basename.
- Open questions for the grilling (seed list): does grove need jj-native
  verbs, or does colocation + `using-jujutsu` cover it (jj colocated repos
  still answer `git rev-parse` and `git mv` works)? Is one-task-one-commit
  phrased VCS-neutrally? Do grove worktrees become jj workspaces or stay
  git worktrees in colocated repos?

## Done when

- Grilling run with the user; decisions logged inline here.
- Any resulting work externalized as leaves (here for tracking, with
  execution in ~/Development/grove) or as grove-repo issues — the grilling
  decides which.
- One focused commit naming `grove-jj-integration-k7`; leaf retired.

## Notes

HITL — needs the user present. Do not start editing the grove repo in this
session; the deliverable is decisions + tree growth.
