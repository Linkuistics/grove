# methodology-content-k3

**Kind:** work

## Goal

Rework the embedded methodology `content/` to the user-owned-worktrees scheme:
the loop's prose, the finish cycle, and the launcher prompts describe a grove
that runs in a user-provided working tree and never touches git topology.

## Context

Decisions: `.grove/01-plan-k1.md` running log; ADRs *user-owned-worktrees*,
*in-session-finish-cycle* (already reworked — the content must match them).
Known stale sites in `content/SKILL.md`: the loop intro ("All sessions of one
grove run in the same git worktree at `<repo>/.grove-worktrees/<name>/` …",
"`grove do <name>` is the sole lifecycle entry verb — for a brand-new grove it
creates the worktree…"), the "Starting a new grove" section, the **Finish**
section (old steps 3–6 → promote / delete-`.grove/` / `complete --done`), the
resume-state checks, and the rename-ritual paragraph (derives `<repo-basename>`
from the worktree's path parent — must derive from the git common dir instead).
Also sweep `content/prompts/*.md` and the README for worktree-layout and
`grove do <name>` references. Repo `CONTEXT.md` glossary was already
reconciled during planning.

Clean-cutover prose: describe the new scheme on its own terms; do not carry
the superseded layout forward as contrast (the ADRs and git hold history).

## Done when

`grep -rn "grove-worktrees" content/ README.md` returns nothing load-bearing;
the Finish prose matches *in-session-finish-cycle*; `cargo build` green (the
binary embeds `content/`); prompts still satisfy "launcher prompts stay small".

## Notes

Sequence after cli-rework-k2 so prose describes shipped behaviour, not intent.
