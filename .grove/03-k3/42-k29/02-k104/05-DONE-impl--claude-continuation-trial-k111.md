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
  sandbox home and the allow-list of the driver. The same launch wrapper
  records its environment. It has no `--add-dir`, no
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
- The JSONL reader now splits only on a newline. `str.splitlines` also split
  on a raw U+2028 inside a JSON string, and it then dropped that entry
  silently. The line windows use the same splitter.
- Results of `continuation-20261102T223923Z`, one run with no retry. The
  first turn read both mandatory files, the kind skill, the supporting
  procedures and `coding-style-bash`. It then wrote `stamp.sh` and stopped
  at the handback. The resumed turn kept the original descriptor and got the
  original bytes. The fresh launch received a new descriptor and the new
  bytes, in the new mandatory order.
- Gap: the resumed turn did not read the mandatory or relevant guidance again
  before it edited the script. Its only guidance read was the retire and
  commit procedures, a procedure step. The record names three candidate
  causes: the agent, the scope of Ravel's obligation 4 for a resume with the
  full history, and Grove's handback wording. `acceptance-handoff-k106`
  chooses between them. The case is not retried, because a second
  run would be a sample, not a repair.
- Observation for `acceptance-handoff-k106`: the fresh launch did not read
  `coding-style-bash`. It copied the style of the existing `stamp.sh`.
- For `codex-continuation-trial-k119`: the runner's resumed turn builds
  Claude Code's `--resume` arguments. The Codex adapter must supply its own
  resume command and its own session-file window.
- No in-session reviewer. The results are observations of transcripts. Each
  new runner path has a fake-harness check that fails against a broken
  runner.
- Verification: `bash scripts/check.sh` passed 20 of 21 checks. "Ravel
  caller assets and provenance" refused, because Homebrew's `bun` is now
  1.4.4 and the typecheck pins 1.4.3. mise holds 1.4.3, but no project mise
  configuration activates it. With that 1.4.3 first on `PATH`, `task
  ravel:review RAVEL=/opt/homebrew/bin/ravel` passed. The integrated checks
  of `acceptance-handoff-k106` meet the same environment.
