# Symmetric VCS rule

The jj skills (`using-jujutsu`, `git-to-jj-mapping`) pick the VCS interface
from repository state alone: a jj-enabled repo (`.jj/` present — `jj root`
succeeds) is driven through jj; any other repo is driven through git,
silently. The skills never convert a repo and never offer to — repo setup
belongs to the human, and an auto-firing skill that mutates it (or lobbies
to) trades away the zero-surprise guarantee that makes auto-firing safe to
ship.

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
