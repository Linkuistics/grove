# claude-compaction-trials-k112

## Goal

Measure the two compaction cases in Claude Code as separate trials. The first
loses the path, and the second loses the whole preamble.

## Context

The "`RAVEL_SKILL_DIRECTORIES`" section and the compaction paragraph of
"Actual harness trials" in Ravel's `docs/specs/skill-discovery.md`. The
compaction paragraph after the acceptance table of Grove's delivery
specification.

## Done when

- The mechanism is established first. The record states how compaction runs
  and how the trial reads the context after compaction.
- Path loss: after compaction the prompt path is gone and the instructions
  remain. The agent recovers the same descriptor through
  `RAVEL_SKILL_DIRECTORIES`. It reads the mandatory guidance again before it
  acts.
- Preamble loss: after compaction no text of the preamble remains, and the
  next message gives no recovery reminder. The variable is still set. The
  record states whether the agent recovered the descriptor and read the
  mandatory guidance again before it acted.
- Each trial has its own record. A failed unprompted recovery is an explicit
  gap.

## Notes

Claude Code 2.1.316 marks `/compact` as supported in non-interactive use, and
it takes summary instructions. Summary instructions may remove text. They
must not give the agent a reminder. Show from the session file that the
summary holds no preamble text.

A retained variable alone proves nothing for the second case.
