# docs-user-facing-sweep-k5

**Kind:** work

## Goal

Sweep the user-facing docs (`docs/grove.md`, `docs/workflows/*.md`) to the
user-owned-worktrees scheme — out of scope for methodology-content-k3, which
covered only `content/` and `README.md`.

## Context

ADRs *user-owned-worktrees*, *do-is-sole-lifecycle-verb*,
*in-session-finish-cycle* (already reworked; `content/SKILL.md` and
`README.md` now match — methodology-content-k3). Stale sites found while
sweeping (this leaf, not yet fixed):

- `docs/grove.md:56,58,63-64,67,69,105` — "Layout" section describes a grove
  as `<repo>/.grove-worktrees/<name>/` on branch `<name>`; `grove do <name>`;
  the old 6-step finish (merge + worktree/branch teardown).
- `docs/workflows/README.md:8-9` — describes `grove retire <name>/<node-path>`
  and a finish that "merges to the default branch, and removes the worktree
  and branch."
- `docs/workflows/start.md` — full walkthrough of `grove do <name>` creating
  `.grove-worktrees/<name>/` on a new branch (command transcripts throughout,
  e.g. lines 16-121). Needs re-capturing against the shipped scheme: the user
  creates the working tree themselves (e.g. `git worktree add` or `git init`),
  then runs argument-less `grove do` from inside it.
- `docs/workflows/multi-step.md:16` — `cd` into a `.grove-worktrees/...` path.
- `docs/workflows/finish.md` — walkthrough of the old 6-step finish (merge,
  worktree remove, branch delete); needs to shrink to the 3-step cycle
  (promote → delete `.grove/` → `complete --done`) with a note that
  integration/teardown is the user's own git/gh from here.
- `docs/research/*.md` mentions of `grove start`/`grove continue` are
  historical research notes, not live docs — leave alone.

## Done when

`grep -rn "grove-worktrees\|grove do <name>\|<name>/<node-path>" docs/grove.md docs/workflows/` returns nothing load-bearing; each workflow's command transcript is re-captured against the shipped `v11` CLI (a hand-created working tree, argument-less `grove do`); the finish walkthrough matches the 3-step cycle.

## Notes

Surfaced while sweeping content/ + README for methodology-content-k3
(clean-cutover-prose: describe the current scheme on its own terms, don't
carry the old layout forward as contrast). Sequence wherever convenient —
doesn't block release-k4's Done when (which only checks live CLI behaviour,
not docs).
