# claude-preamble-write-tool-trial-k137

## Goal

Produce the loss of the whole preamble in Claude Code with work files that
the Write tool writes. Then observe whether the agent recovers the descriptor
through `RAVEL_SKILL_DIRECTORIES`.

## Context

- The record section "compaction-preamble-five" of
  `docs/common-loading-acceptance-record.md`, from
  `claude-preamble-loss-trial-k130`. Its mechanism table states which calls
  put a file in Claude Code's restoration set. Bash heredoc writes do not.
  The Write tool's writes and a Bash call that is only `cat "<path>"` do.
- The case `scripts/harness-trials/cases/compaction-preamble-five`. Its
  summary instructions are those of the second `compaction-preamble` attempt.
- Claude Code's restoration rule:
  https://code.claude.com/docs/en/context-window#what-survives-compaction.
  It restores up to five read or edited files, the most recently modified
  first.

## Done when

- A new case is a copy of `compaction-preamble-five`. It tells the agent to
  write each of the five work files with the Write tool, one file in each call. It
  keeps the summary instructions and the answer unchanged. It names no
  preamble term.
- The task file records the attempt count before the run.
- The record states, from the session file, which tool wrote each work file.
  It also states which files Claude Code restored, and whether any of them
  lies in a snapshot.
- When the gate passes, the record states whether the agent recovered the
  descriptor through the variable and read the mandatory guidance again
  before it acted. Recovery is a pass. Anything else is a gap.
- When the gate fails, the Claude Code `compaction-preamble` row is an
  explicit unmeasured gap, with no further repair. The record and the note of
  `acceptance-completion-audit-k127` name it, with the mechanism.

## Notes

The human chose this repair on 2026-11-03, in
`claude-preamble-loss-trial-k130`, over accepting the gap. The option the
human chose states that a failure of this repair becomes an explicit gap
with no further repair leaf.

A prompted retry is no credit, and the retained variable alone is no
evidence. A case that directs the tool is artificial. The record states that
limit beside the result.

## Decisions (running log)

- **The new case is `compaction-preamble-write`.** It is a copy of
  `compaction-preamble-five`. Only the name, the purpose and the Notes of its
  leaf differ. The Notes tell the agent to write each work file with the
  Write tool, one file in each call, and not through a shell command. The
  summary instructions, the answer, the sources and the mandatory files are
  unchanged. The runner's term check passes for the instructions, because
  they are the earlier instructions word for word.
- **The copied Goal keeps its long sentence.**
  <!-- tc: depart long-sentence because the decision names the linter's finding on a trial input -->
  The linter finds one sentence of 22 words in the copied item for `stamp.sh`.
  The tree file is a trial input, and the agent reads its bytes. The copy
  therefore keeps the sentence and carries no marker.
- **The case gets one counted attempt.** One further run is allowed only if
  the first turn writes fewer than five work files with the Write tool after
  its last guidance read. That run did not test the variant. A gate failure
  after five such writes ends the case, with no further run. A failure makes
  the Claude Code `compaction-preamble` row an explicit gap.
- **The run reuses the preparation at Grove commit `1f7b8013d4f6`.** Before
  the run, `grove` is `fd71aa5d…`, `grove-llm` is `56f6815a…` and Ravel's
  `bin/ravel` is `d076dc01…`. Each digest equals the record. Claude Code is
  still 2.1.317. Cases come from this working tree, so a new case needs no
  new preparation.
