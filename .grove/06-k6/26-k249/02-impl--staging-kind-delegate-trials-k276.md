# staging-kind-delegate-trials-k276

## Goal

Add two cases to the staging trial runner and pass each on Claude Code and
Codex. One task names a Grove kind skill. One delegate reads a skill
resource.

## Context

- staging-trial-runner-k275 delivered the runner, its case shape and the
  evidence section of this node.
- The kind case resembles a Grove mandate: its prompt names a kind skill,
  such as `grove-impl`. The agent must read that skill from the
  `grove-core` snapshot and one relative resource of the same source.
- `scripts/harness-trials/cases/delegate` and `research-delegate` hold
  earlier delegate cases. The runner's adapters already extract
  delegations.

## Done when

- On each harness, the kind case passes. The agent reads the named kind
  skill and a relative resource of its source.
- On each harness, the delegate case passes. A delegate reads a skill
  resource.
- The evidence section records each trial, its command, its versions and
  its result.

## Notes

A failure is a finding for the owner design or for a pack, not a reason to
weaken a case.
