# Symmetric VCS rule

The jj skills (`using-jujutsu`, `git-to-jj-mapping`) pick the VCS interface
from repository state alone: a jj-enabled repo (`.jj/` present — `jj root`
succeeds) is driven through jj; any other repo is driven through git,
silently. The skills never convert a repo and never offer to — repo setup
belongs to the human, and an auto-firing skill that mutates it (or lobbies
to) trades away the zero-surprise guarantee that makes auto-firing safe to
ship.

The same rule binds the **`grove` binary**, which decides jj-first in
`repo::vcs_of` — the closest marker walking up, `.jj/` winning over a `.git`
beside it — and likewise converts nothing: grove runs in a working tree the
user provides and creates none (*user-owned-worktrees*).

It binds **`install.sh`** too, whose workspace guard asks the same question of
the tree it lives in — is this the repo's main checkout, or a side tree? — and
must therefore answer it the same way. There the ordering is not a matter of
consistency but of correctness: a secondary jj workspace of a colocated repo is
not a git worktree at all, so a git-first probe reports "not a repository" and
misses precisely the case the guard exists to catch. The guard likewise converts
nothing and mutates nothing — it probes with `--ignore-working-copy`, because
every other jj invocation snapshots the working copy as a side effect.

One decision with three enforcers, not three decisions.

## Considered options

- **Ask-first colocation offer** — `jj git init --colocate`, offered once
  per session when jj was installed but the repo not jj-enabled. The
  workstream's original design; no prior art implements it
  (`docs/research/jj-agent-prior-art.md` §Q4 — every surveyed system either
  forces, converts silently, or stays out). Rejected during the
  `skill-design-k3` grilling: even an offer is mid-task evangelism, and the
  repo's VCS setup is not the agent's to change. **Reopened by:** field
  evidence that users routinely want in-session conversion — e.g. repeated
  manual `jj git init --colocate` requests in sessions where the skills
  stayed silent.
- **Silent conversion** — rivet-style mandate: initialize jj wherever it is
  missing (`docs/research/jj-agent-prior-art.md` §Q4). Rejected: mutates
  user repos without consent, the strongest distrust trigger surveyed.
  Nothing short of an explicit standing instruction from the user would
  reopen this.
