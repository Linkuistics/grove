# staging-compaction-stale-trials-k277

## Goal

Add two cases to the staging trial runner and pass each on Claude Code and
Codex. A compaction occurs, and the agent still reads skills afterwards. A
stale native copy coexists with the Ravel copy.

## Context

- staging-trial-runner-k275 delivered the runner, its case shape and the
  evidence section of this node.
- `harness_trial.py turns` feeds a compaction turn. Claude Code compacts
  with `/compact`, and Codex compacts through `codex app-server`. The
  earlier cases `compaction-preamble-five` and `problem-stale-native` show
  the shape.
- The stale copy goes into the working directory of the trial, never into
  the staging HOME. Claude Code reads `.claude/skills` in each parent
  folder of its working directory. Measure where Codex reads a project
  skill before relying on it.

## Done when

- On each harness, the compaction case passes. After the compaction, the
  agent reads a skill again.
- On each harness, the stale-copy case records which copy the agent reads.
  The staging HOME holds no stale copy before or after.
- The evidence section records each trial, its command, its versions and
  its result.

## Notes

A failure is a finding for the owner design or for a pack, not a reason to
weaken a case.
