# claude-preamble-loss-trial-k130

## Goal

Produce the loss of the whole preamble in Claude Code, and observe whether the
agent recovers the descriptor through `RAVEL_SKILL_DIRECTORIES`.

## Context

- The record sections "Compaction", `compaction-preamble` and "Compaction:
  Codex and Claude Code".
- The triage section of `docs/common-loading-acceptance-record.md`.
- The compaction feeder and gate of `scripts/harness_trial.py`.

## Done when

- A variant of `compaction-preamble` writes at least five work files after
  the last guidance read of its first turn. It uses the summary instructions
  of the second attempt.
- The record states, from the session file, which files Claude Code restored
  after the boundary, and whether any of them lies in a snapshot.
- When the gate passes, the record states whether the agent recovered the
  descriptor through the variable and read the mandatory guidance again before
  it acted. Recovery is a pass. Anything else is a gap.
- When the gate fails again, the record holds the mechanism evidence, and the
  leaf asks the human a scope question about the unmeasured case.

## Notes

Record in the task file, before the run, how many attempts the case gets.
A prompted retry is no credit, and the retained variable alone is no evidence.
