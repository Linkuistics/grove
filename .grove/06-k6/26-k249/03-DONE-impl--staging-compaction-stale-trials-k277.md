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

## Decisions (running log)

- Codex 0.167.0 lists a project skill from `.codex/skills` and from
  `.agents/skills` of its working directory, or of the Git root above it.
  It ignores `.claude/skills`. `codex debug prompt-input` with the staging
  environment measured this in a plain directory, a Git root and a Git
  subdirectory. So the stale case plants one copy in `.claude/skills` and
  one in `.codex/skills` of the work directory, and each harness lists
  exactly one.
- The stale copy is the owner's real stale `grove` copy,
  `~/.claude/skills/grove`. It is an older provisioned spine with
  `references/planning.md` and a kind table, and it matches no single
  commit. The case copies it read-only from the real HOME and pins its
  tree digest. A changed or missing copy refuses the case. The switch
  deactivates the real copy, and the coexistence that the case measures
  goes with it. A rerun before a fresh attempt runs in native delivery,
  where the copy exists.
- The stale case uses the planning mandate and the tree of the kind case,
  because a Grove mandate leads the agent to the `grove` spine. Its verdict
  only records which copy the agent reads. It fails when the agent reads
  neither copy, or when the native listing does not hold the planted name.
  It also fails when the listing holds another superseded name.
- A stale read names a path of a planted copy, or calls the native `Skill`
  tool with that name. A result that holds a line only the stale `SKILL.md`
  holds is a stale read too. A
  Ravel read is a classified read from `grove-core`, or a result that holds
  a line only the current `SKILL.md` holds.
- The compaction case uses each harness's default compaction, not an
  artificial loss. Claude Code compacts with a bare `/compact`, and Codex
  with `thread/compact/start` and remote compaction. The common-loading
  cases measured the path and preamble losses, and the preamble loss is a
  known gap. The record keeps what the context after compaction still holds
  of the preamble.
- The compaction case's first turn writes a Python tool and stops. The turn
  after the compaction asks for a Bash script and the end command. The
  verdict needs a read of `coding-style-bash` from `engineering` after the
  boundary and before the first edit after it.
- `trials/compaction-claude-1` and `-2` failed. In each, the Claude Code
  agent read the mandatory file and then wrote both files with no skill
  search, before and after the compaction. The context after compaction
  held the descriptor path and named the variable, and Claude Code restored
  `using-superpowers/SKILL.md`. So compaction did not remove the means of
  discovery. `trials/compaction-codex-1` passed.
- Two diagnostics outside the case files separated the causes. The
  discovery case without snapshot `--add-dir`, the owner's real launch
  shape, passed. The compaction case's first prompt in plain `-p` mode,
  without the feeder, also passed. The first model requests of a plain run
  and a feeder run have identical messages and parameters. Their tool lists
  differ only in a background timeout default. The difference is therefore
  sampling variance in skill search at low effort, not the input mode, the
  prompt or `--add-dir`. The case stays unchanged, and the runner measures a
  rate with three more Claude Code runs. Every attempt goes into the record.
- The owner's real Claude Code policy passes no snapshot root as
  `--add-dir`, but the trial routes of the discovery, kind and delegate
  cases do. The diagnostic shows that discovery still occurs without it.
- `distinct_lines` first compared whole lines, while the match tests a
  substring. `trials/stale-copy-codex-1` then showed a false stale read: the
  current `grove/SKILL.md` holds `compressed technical shorthand.` inside a
  longer, differently wrapped line. A unique line now must not occur
  anywhere in the other file. `rejudge` on the unchanged records then showed
  a read of Ravel's copy only.
- The first `bash scripts/check.sh` failed two tests. One is the known
  `removed_surface` test of `classify-grove-rows-name-k279`. The other,
  `a_timed_out_selection_launches_nothing_and_publishes_nothing`, failed with
  `select never ran`, and passed alone three times. Its policy had one
  second to start before the timeout fired. The owner directed generous
  timeouts, so the test now gives `timeoutMs: 10000` against the fixture's
  25-second stall.
