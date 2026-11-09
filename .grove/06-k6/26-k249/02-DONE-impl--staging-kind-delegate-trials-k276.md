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

## Decisions (running log)

- The kind case names `grove-planning`. Of the Grove kind skills, only
  `configure-grove`, `grove-finish` and the planning family lie in the
  `grove-core` source. `grove-impl` lies in `engineering`. A relative
  resource is any file other than a `SKILL.md` in the `grove-core`
  snapshot, such as `grove/BRIEF-FORMAT.md` or `grove/references/*.md`,
  which the kind skill and the spine name by relative path.
- The kind case plants a small grove as `.grove/` of a new jj repository
  in the work directory. The case field `tree` names a directory under
  `staging-cases/trees`. The session runs through `ravel start-session`
  with a prompt shaped like Grove's launch prompt, not through the `grove`
  driver.
- The delegate case asks for two Bash scripts and a delegate for one. The
  prompt asks the parent to tell the delegate to find the skill for Bash
  style. The verdict needs a read by a delegate of a file of
  `coding-style-bash` in the `engineering` snapshot.
- The runner judges the mandatory order of every case, and fails it only
  when the case sets `mandatoryFirst`. The kind case leaves it unset: a
  Grove mandate tells the agent to load the kind skill first.
- The first two kind trials showed three defects of the instrument. The
  planted repository was colocated, and Codex's sandbox denies writes below
  `.git`, so the Codex agent could not commit or end. The plant is now not
  colocated, as the common-loading plant was. Claude Code did the planning
  through `grove-llm` and shell writes, with no edit tool. The order point
  is now the first event that edits a file, names a path in the work tree
  or mentions `.grove`. The owner's own session rewrote `.in_use` markers in
  the real plugin cache. The check now leaves out each version directory's
  top-level `.in_use`, as inventory.py does, and fails on a new marker of a
  process that has ended.
- The kind verdict needs the kind skill strictly before the first touch of
  the work tree, and the resource no later than it. Claude Code read
  `grove/references/bootstrap.md` and then the task file in one command.
  The bootstrap reference names the task file, so that order is Grove's own
  procedure, and a strict order for the resource was a defect of the
  verdict.
- The second Claude Code kind run read the task file at event 8 and
  `grove/references/bootstrap.md` at event 9, before its first write at
  event 12. The spine's bootstrap names the task file, so this order is
  Grove's procedure too. The done-when needs only the two reads. The
  verdict now judges one order, the kind skill before the first touch of
  the work tree, which the Grove mandate sets. It records the position of
  the resource read without a judgement.
- `bash scripts/check.sh` failed one test on the parent commit.
  `removed_surface every_grove_name_in_the_tree_is_classified` finds
  `GROVE_ROWS` in `scripts/owner_switch_rehearsal.py`, which
  owner-switch-dry-run-k271 added. This leaf touches neither file, and the
  other 1343 tests and every other gate pass. The fix went to the new leaf
  `classify-grove-rows-name-k279`, ahead of the next live entry of the
  discovery-rollout node.
