# claude-delegate-trial-k113

## Goal

Observe one real Claude Code delegate. Separate its reads, its adherence and
its inherited ending authority.

## Context

Item 5 of "The preamble" and the "Read access and delegation" section of
Ravel's `docs/specs/skill-discovery.md`. The Delegate authority row of its
acceptance table.

## Done when

- A parent on the directory route on 2.0 delegates part of the task to a real
  delegate.
- The record states what the parent gave the delegate: the descriptor path,
  the obligations and no ending instructions.
- The delegate reads the mandatory guidance first. It then reads the retained
  resources that its part needs.
- The record states the delegate's inherited ending context, and whether it
  obeyed the instruction not to end.
- A separate disposable session measures whether the delegate can send an
  early result. A first-writer check follows.
- The record names the delegate mechanism and the adapter. It names a
  scrubbing adapter as an unmeasured gap when none exists.

## Notes

The delegate mechanism of Claude Code is its Agent tool. An in-process delegate
shares the process environment.

The early-result attempt is a measurement of authority, not of adherence.
Prompt it only in a disposable trial session, never in this live session.

## Decisions (running log)

- Two cases. `delegate` measures adherence: one `impl` leaf asks for a Bash
  script and a check script, and asks the parent to delegate the check script
  through the harness's agent tool. The leaf names no skill, no obligation and
  no ending rule. `delegate-early` measures authority in a disposable session:
  its leaf asks the parent to give one delegate an exact command that tries
  two results.
- Delegate mechanism: Claude Code's Agent tool, in process. Claude Code writes
  each delegate's transcript to `<session>/subagents/agent-<id>.jsonl`, with a
  `.meta.json` whose `toolUseId` names the parent's Agent call. The runner
  already reads those files as further transcripts of the launch.
- What the parent gave: the runner records each Agent call's prompt in full,
  with its digest. It reports whether the prompt holds the descriptor path,
  the variable's name and each obligation line of the preamble word for word.
  It lists each ending term with its line, because a prohibition also names
  `end-session`. The reader judges the prohibition.
- First-writer order: the delegate first sends the unsupported token
  `trial-delegate-early`, then `grove-force-stop`. Grove does not print the
  token, but its stop messages differ: an unsupported result means the first
  value won, a force-stop means the second replaced it, and a missing result
  means the delegate had no authority. Neither order relaunches the leaf.
- The delegate writes its command output to `$TRIAL/authority.log`, because
  Ravel ends the group 2 seconds after publication and the tool result may
  never reach the session file. A case field `collect` copies named trial
  files into the record.
- Adapter: the trial's adapter is the Claude Code adapter of the runner,
  `trial-launch.sh` then `claude -p`. No scrubbing adapter exists for an
  in-process Agent delegate, so the scrubbed side of the Delegate authority
  row is an unmeasured gap.
