# claude-code-always-on-k11

## Goal

Make the skill always on in Claude Code, the harness the experiment runs in, and
build the pieces every other harness reuses: the shared hook scripts and the
switch. Claude Code gets the whole always-on layer: the instruction at session
start, the reminder before each prompt, the document gate, the reply log and
the switch.

## Context

- The design `docs/specs/technical-communication.md` fixes the layers, the
  switch, the log and the gate. This leaf builds the hook core
  (`scripts/tc-hook`, the POSIX sh guard, and `scripts/tc_hook.py`, with the
  Claude Code adapter), the gate's file classes and message, and the entry
  `plugins/linkuistics/hooks/hooks.json`. The entry is in exec form with fixed
  timeouts, and it never changes shape after this leaf. `harness-hooks-a.md`
  holds Claude Code's hook events, payloads, configuration schema and pitfalls,
  with the installed version they were checked against.
- Set `TECHNICAL_COMMUNICATION_STATE_DIR` for every test run, so that the
  human's real off file and log are untouched.
- The `guardrail` skill's hook is skill-scoped and runs only while that skill
  is active. This skill's hooks must run in every session, so the survey says
  where a plugin ships always-on hooks. The plugin is `plugins/linkuistics`,
  published through `.claude-plugin/marketplace.json`.
- A hook that misfires affects every session of the human who installs the
  plugin. A hook failure must never block or break a turn: it fails open and
  quiet, and a missing checker or an unreadable log changes nothing visible.
- `scripts/check.sh` must stay green, and the plugin must still install.

## Done when

- With the plugin loaded in a real Claude Code session, the instruction reaches
  the model at session start and the reminder reaches it before a prompt; a
  prose file with a violation written in the session gives the model the
  findings to fix; each reply appends one record to the log; and the switch
  turns all four off. Show this by running it, not by reading the
  configuration. A project memory records that `script -q` allows driving a real
  session from inside one; a headless `claude -p` run with the plugin directory
  also works. Put the evidence in the running log.
- The shared scripts take what they need from the harness through a thin adapter,
  so the Codex, Pi and Gemini leaves reuse them.
- `SubagentStart` sends the instruction to a subagent. The running log says
  whether the gate and the log also ran inside a subagent.
- The reminder is within the size budget in the spec.

## Notes

- Verify the live behaviour. An earlier release of this project shipped with a
  feature not wired in, and a green build did not show it.
- If hooks, gate and log do not fit one session, decompose and finish the first
  child (instruction, reminder and switch).

## Decisions (running log)

- **Built.** `scripts/tc-hook` (POSIX sh guard), `scripts/tc_hook.py` (core plus
  a `ClaudeCode` adapter in an `ADAPTERS` table) and
  `plugins/linkuistics/hooks/hooks.json`. The entry is in exec form
  (`/bin/sh` with `args`), with timeouts 10 (`SessionStart`, `SubagentStart`,
  `Stop`, `SubagentStop`), 5 (`UserPromptSubmit`) and 15 (`PostToolUse`,
  matcher `Write|Edit|NotebookEdit`). It does not change shape after this leaf.
- **The guard has no external command.** It uses `${0%/*}`, not `dirname`,
  because a probe with `PATH=/nonexistent` printed `dirname: command not found`
  on standard error. It tries `python3` on the path and then three absolute
  locations, runs `python3 -I`, discards standard error and exits 0 on every path.
- **The core loads the checker by path** (`importlib`), checks the off file
  through the checker's own `state_dir()` (one rule for the location, in one
  place), and prints one ASCII-escaped JSON object through `sys.stdout.buffer`,
  so a C locale cannot break the output. A reply event prints nothing.
- **A first marker of `off` or `exempt` needs no separate gate skip.** When it
  comes before any prose and nothing closes it, `analyse` already returns no
  findings, so the gate sends no message. The other skip classes (temporary
  directories, `.git`/`.jj`, licences, vendored text, finished task files,
  generated files) are in `never_gated`.
- **The gate message** names each file with its hard count, lists up to ten
  findings, says how many more exist, and ends with one sentence. That sentence
  states what the skill expects, instead of giving a bare command, because the
  survey warns that command-shaped hook text can be shown to the human. A
  1,500-character cap is enforced by listing fewer findings (probe: 160
  findings in a file with a 120-character name gave 1,078 characters).
- **`model` comes from the transcript.** Claude Code 2.1.302's `Stop` and
  `SubagentStop` payloads have no model, so the adapter reads the latest
  assistant entry from the last megabyte of `transcript_path`, or of
  `agent_transcript_path` for a subagent. A failure gives null. The spec has a
  paragraph on this under "The reply log".
- **No Taskfile task and no test was added.** `plan-k1` decided that no leaf
  builds tests, and the live recipe below costs model quota. `tc-hook` joined
  the `shellcheck` list in `scripts/check.sh`, which is a check of an existing
  kind.

## Evidence (all runs used `TECHNICAL_COMMUNICATION_STATE_DIR` in the scratchpad)

Real sessions: `claude -p --plugin-dir plugins/linkuistics --model haiku
--permission-mode acceptEdits` in `~/.cache/tc-live-k11` (outside every temporary
directory, so the gate does not skip it), with a throwaway `--settings` file
whose hooks appended each raw payload to a file. Claude Code 2.1.302.

1. **Instruction.** The `SessionStart` hook response carried the text. In the
   subagent run, the subagent quoted the instruction's first sentence back, so
   `SubagentStart` reached it.
2. **Reminder.** The session transcript holds `UserPromptSubmit hook additional
   context: The human's message arrives in a session that writes by the
   technical-communication skill…` before the prompt. (`stream-json` does not
   show `UserPromptSubmit` responses, so the transcript is the evidence.)
3. **Gate.** `Write` of `notes.md` ("We utilize the optimized system; it was
   organized.") produced a `PostToolUse hook additional context` with four
   hard findings, and the model's final answer reported "four style issues
   (American spellings … plus a semicolon)".
4. **Log.** One `start` and one `reply` record for the main thread, with the
   payload's `session_id`; `model` is `claude-haiku-5-5` once the transcript
   lookup was in (null before it, in the first two runs).
5. **Subagent: the gate and the log both ran inside it.** The subagent's own
   transcript holds the gate message for `sub.md`; `PostToolUse` carried
   `agent_id`; `SubagentStop` produced a reply record with `agent` set to the
   subagent id and the parent's `session`; the `start` record has
   `source: "subagent"`.
6. **Switch.** With the `off` file present, the same prompt gave no instruction,
   no reminder and no gate message in the stream (the only `technical-communication`
   text was the skill's name in the skill list, because the skill stays
   installed), and the state directory held only `off`. After deleting the file,
   the next run wrote a `start` and a `reply` record and the gate message came
   back.
7. **Fail open.** Garbage and array payloads, an unknown event, no arguments, an
   empty `PATH`, a copy of the skill without `ste-lint.py`, a copy without
   `always-on/`, and a non-UTF-8 file all gave exit 0 and no output; the copy
   without `always-on/` wrote no start record.
8. **Skip classes.** Under one directory, the same bad text gave a message from
   `bad.md` and `UPPER.MD` and none from `LICENSES/`, `vendor/`, a `DONE` task
   file under `.grove/`, a `.py` file, a file with `@generated`, a file with an
   `exempt` marker or a missing file.
9. **Budgets.** `instruction.md` is 1,889 of 2,500 characters; `reminder.md` is
   119 of 200; the gate message stays under 1,500.
10. **Plugin.** `claude plugin validate plugins/linkuistics` passes (its one
    warning, no `version`, is intended: the plugin updates by commit).

**For the next leaves.** Codex adds a `Codex` adapter to `ADAPTERS` and reuses
`handle`, `gate`, `gate_message` and `never_gated`; Pi's extension calls
`tc-hook` as a child process. Two facts about the payloads help: `agent_id` is
present only inside a subagent, and the plugin's `${CLAUDE_PLUGIN_ROOT}`
placeholder is substituted inside `args`.
