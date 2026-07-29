# install-workspace-guard-k8

**Kind:** impl

## Goal

Stop `install.sh` from silently hijacking a user's skill install when it is run
from a **secondary jj workspace or a linked git worktree** instead of the main
checkout.

## Context

Found by `distribution-k5` while verifying that `install.sh` needs no change to
pick up a new skill. It doesn't — but it cannot be run from a grove working tree.

`install.sh` derives `repo_root` from `${BASH_SOURCE[0]}`, so it links from
*whichever tree it lives in*. It then `rm`s any existing symlink and re-links
unconditionally. Run from a grove workspace, it re-points **every** linkuistics
skill — all 15 already installed, not just the new one — at that workspace. The
links survive until the workspace is torn down, then all dangle.

Nothing surfaces the breakage: `ln -s` does not require its target to persist,
the harnesses read `SKILL.md` lazily, and a dangling skill directory reads as
"skill not installed" rather than as an error.

Concrete state at discovery time:

- default workspace: `/Users/antony/Development/grove`
- this grove's workspace: `/Users/antony/Development/grove.using-codebase-memory`
- all 15 links in `~/.codex/skills` → the **default** workspace, correctly.

The trigger is structural, not jj-specific: a linked `git worktree` has the same
shape, which is why the check should key off the working tree's relationship to
the repo rather than off jj.

Grove working trees are ephemeral by design (`user-owned-worktrees` — the user
provides and tears down the tree), so this is not an exotic case. Any grove run
on this repo can hit it.

## Done when

- Running `install.sh` from a non-default jj workspace or a linked git worktree
  does **not** silently relink. Whether it refuses, warns, or requires a
  `--force`-style opt-in is this session's call — pick the smallest guard that
  makes the failure loud, and say why in the commit.
- The main-checkout path is untouched: running it from the default workspace
  behaves exactly as before.
- Both paths exercised against an isolated `HOME`, never the real one. Set
  `HOME` to a scratch dir with `.codex`, `.gemini` and `.pi/agent` pre-created;
  a full run links `<skill count> × 3`.
- `linkuistics:coding-style-bash` consulted — `install.sh` is strict-mode bash
  and the guard should match its house style.

## Notes

**Do not "fix" this by writing to the real `$HOME` to check.** The whole defect
is an unwanted write to `$HOME`; reproducing it there costs the user a manual
repair. The isolated-`HOME` harness above is sufficient and is how
`distribution-k5` verified the install path.

**Detection sketch, not a decision.** `jj workspace root --name default` gives
the default workspace's path in a jj-enabled tree, and `git rev-parse
--git-common-dir` distinguishes a linked worktree in a git one. Either is a
probe, not a mandate — `symmetric-vcs-rule` governs the jj-first ordering if a
probe is used at all.

**Open question worth answering explicitly:** is linking from a non-default tree
ever *wanted* — e.g. deliberately testing an unmerged skill against a live
harness? If so the guard is an opt-in prompt, not a refusal. That is the choice
the done-when leaves open.
