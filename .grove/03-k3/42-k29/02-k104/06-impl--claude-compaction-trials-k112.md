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

## Decisions (running log)

- Compaction mechanism: Claude Code 2.1.316 runs `/compact <instructions>`
  as one user turn of `claude -p --input-format stream-json`. A probe in a
  sandbox home with `claude-haiku-5-5` compacted on the second turn. The third
  turn answered "UNKNOWN" for a fact that the instructions removed. The same
  probe through `claude -p --resume <id> "/compact …"` behaved the same way.
- The trial compacts inside the Ravel-managed process. A feeder runs as the
  route program, sends the prompt, then `/compact`, then the answer, each after
  the `result` event of the turn before. The harness therefore keeps Ravel's
  own `RAVEL_SKILL_DIRECTORIES` and result channel. A resume outside Ravel
  would supply the variable from the runner, and it would lose the ending.
- Compaction cases omit `--add-dir`. Claude Code writes additional working
  directories into the system prompt, and the system prompt survives
  compaction. That would keep the snapshot paths. `bypassPermissions` already
  admits reads outside the working directory.
- Claude Code records the `/compact` command and its arguments as a user
  message after the boundary, so the agent sees the summary instructions. The
  instructions of the preamble-loss case therefore name no discovery term.
- The summary ends with Claude Code's own line that names the full transcript
  file. A read of that file is a third route to the preamble. The extraction
  gives it the class `transcript`.
- How the trial reads the context after compaction: the session file holds an
  `api-request` entry for each model request, with `keep`, the `tail` blob
  hashes and the shape of the system prompt. The first request after the
  `compact_boundary` entry, with `keep` 0, is the exact context that the model
  received. The runner scans its blobs and system text for preamble text.
- The feeder gates the answer turn. After compaction it scans the session
  lines after the boundary. If a preamble path or digest remains, or the
  path-loss case lost the variable's name, it sends no answer. A failed gate
  is a failed precondition, not an observation of the agent.
