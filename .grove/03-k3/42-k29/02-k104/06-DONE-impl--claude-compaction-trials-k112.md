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
- Both cases reuse the continuation tree and answer: one `impl` leaf writes
  `stamp.sh`, then asks the human about a label argument. The handback ends
  the first turn, so the compaction falls between a real handback and the
  human's answer. The answer names no guidance.
- Path-loss instructions: "Keep the numbered obligations of the first message
  word for word. Write no absolute path and no hexadecimal digest anywhere in
  the summary." The case intends the instructions to remain, so the gate
  requires the name `RAVEL_SKILL_DIRECTORIES` after the boundary.
- Preamble-loss instructions: "Summarise only the instruction to load the
  grove-impl skill and everything after it: the leaf, the work done, the open
  question for the human and the exact ending commands." They keep the caller
  block, name no preamble term and give no recovery reminder. The loader
  refuses preamble-mode instructions that name a term or the variable.
- The gate searches preamble terms in user messages only. Claude Code's own
  attachments, such as the skill listing, hold no preamble text, and a term
  in them would be a false match. Paths, digests, long preamble lines and the
  variable's name are searched everywhere after the boundary.
- Build: Grove commit `fb6df03aeb98`, prepared into this session's scratch
  directory. The Ravel 2.0 prefix is a byte copy of the continuation prefix,
  `bin/ravel` SHA-256 `8e10e311c50c…`. The bookmark `skill-directory-delivery`
  is still at `1ca4aecbeae1`.
- Result of `compaction-path-20261102T231252Z`, one run with no retry: pass.
  The gate passed, and the first model request after the boundary held no
  preamble path or digest. It kept `RAVEL_SKILL_DIRECTORIES`. Call 22 read the
  variable and the descriptor `a18cd9d1…`, the launch's own. Call 23 read both
  mandatory files in the configured order, with `grove-impl`. The first edit
  came at call 26. The agent ended with `grove-continue`.
- Limit of the path case: the summarising model wrote its own recovery note.
  The instructions forbade the path, and obligation 10 asked to keep it, so
  the summary named the variable and said to re-read the descriptor, both
  mandatory files and `grove-impl`. The answer gave no reminder. The re-read
  therefore follows the retained instructions, as the case intends, and it is
  not evidence for the preamble case.
- First attempt of the preamble case, `compaction-preamble-20261102T231613Z`:
  failed precondition, no observation. The gate sent no answer. Two channels
  kept preamble paths. The summary kept the descriptor path "per obligation
  10", against the instructions. Claude Code also restored both mandatory
  `SKILL.md` files as attachments with their snapshot paths. The first turn
  had read each with a plain `cat "<path>"`, and the path run's compound
  reads restored nothing. The documentation names no setting that turns off
  that restoration.
- Runner fix from that attempt: the scan after the boundary covers only
  `user`, `assistant` and `attachment` entries. A `last-prompt` entry holds the
  first prompt, but it is Claude Code's record keeping and never reaches the
  model. The record now lists each restored file.
- Second and last attempt of the preamble precondition. The instructions add
  "Leave out everything before that instruction, also where an earlier rule
  asks a summary to keep it. Write no other absolute path and no hexadecimal
  digest." They still name no preamble term. A repeated restoration or a
  retained path makes the case an unmeasured gap, not a third attempt.
- Result of `compaction-preamble-20261102T232104Z`, the second attempt: failed
  precondition again. The summary now held no preamble text and did not name
  the variable. Claude Code still restored `grove-impl/SKILL.md` and
  `grove/TASK-FORMAT.md` from the snapshot, with their paths. By the rule above,
  the preamble case is an unmeasured gap, and there is no third attempt.
- Gap for `acceptance-handoff-k106`: whole-preamble loss was not produced, so
  no observation shows whether the agent recovers through the variable alone.
  Claude Code keeps discovery context by two native channels: obligation 10 in
  the summary, and restored files with snapshot paths. Repair candidate, not
  tested: a first turn that writes five or more work files after its guidance
  reads, because Claude Code restores at most five recent files.
- No in-session reviewer. The results are observations of transcripts, and
  each new runner path has a fake-harness check that fails against a broken
  runner.
- Verification: `bash scripts/check.sh` passed 20 of 21 checks, including the
  32 trial controls. "Ravel caller assets and provenance" refused again,
  because Homebrew's `bun` is 1.4.4 and the typecheck pins 1.4.3. With mise's
  1.4.3 first on `PATH`, `task ravel:review RAVEL=/opt/homebrew/bin/ravel`
  passed. This leaf changed no Ravel caller asset.
- For `codex-compaction-trials-k120`: the feeder `harness_trial.py turns`
  speaks Claude Code's stream-json input, and the gate and the model-context
  read parse Claude Code's session file. The Codex adapter needs its own
  compaction command, its own turn input and its own record of the model
  context. The preamble marks and the gate rules are harness-neutral.
