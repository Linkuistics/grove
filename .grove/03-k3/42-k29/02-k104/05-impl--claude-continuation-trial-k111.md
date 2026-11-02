# claude-continuation-trial-k111

## Goal

Observe continuation in Claude Code, and the retained reads after a source
change and a settings change.

## Context

Item 4 of "The preamble" and the Lifetime row of Ravel's
`docs/specs/skill-discovery.md`. The Resource lifetime row of Grove's delivery
specification.

## Done when

- A first turn on the directory route on 2.0 reads the mandatory and relevant
  guidance and stops before the work completes.
- Between the turns, a new commit changes a read file in the source checkout.
  The settings change too.
- The continued session keeps the original descriptor. It reads the mandatory
  and relevant guidance again before work continues, and gets the original
  bytes.
- A fresh launch after the change receives a new descriptor with the new
  bytes.
- The record states the continuation mechanism and whether each re-read came
  unprompted.

## Notes

Establish the continuation mechanism before the case. `claude --resume` is
the candidate. A continued session outside Ravel's managed process loses the
variable, so record the environment of each turn.

## Decisions (running log)

- Continuation mechanism: `claude -p --resume <session id>` without
  `--fork-session`. Claude Code 2.1.316 then keeps the session ID and appends
  the resumed turn to the same session file. The runner splits that file into
  one line window for each turn.
- The resumed turn runs outside Ravel's managed process, as an owner's manual
  resume does. It runs from the working directory of the first turn, with the
  sandbox home and the allow-list of the driver, through the same launch
  wrapper, which records its environment. It has no `--add-dir`, no
  `RAVEL_SKILL_DIRECTORIES` and no result channel.
- The first turn stops at a real handback. The leaf asks for a script, then
  for required input from the human, as Grove's handback procedure directs.
  The turn ends, Ravel refuses as `session_result_missing` and the leaf stays
  live. The prompt of the resumed turn is the human's answer only. It names no
  guidance, so each re-read in that turn is unprompted.
- One plain Git clone of the prepared commit in the trial directory supplies
  both sources. Between the turns, one commit in that clone inserts the marker
  `k111-revision-2` after the front matter of four read files: the two
  mandatory files, `grove-impl` and `coding-style-bash`. In
  `coding-style-bash` the marker is a rule that a script carries the comment
  `# k111-revision-2`, so the written script also shows which bytes governed
  it. The settings change reverses the order of the mandatory files.
- The runner marks each read event whose tool result holds the marker. It
  also checks the copy of each edited file in each launch's snapshot. The
  first turn and the fresh launch are the negative and positive controls of
  that detection.
- The fresh launch: the runner commits a second `impl` leaf into the trial
  tree after the resumed turn, then runs the driver again. One leaf in the
  first tree keeps the first driver run to one launch, even if the agent does
  not stop.
