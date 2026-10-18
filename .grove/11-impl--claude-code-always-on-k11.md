# claude-code-always-on-k11

## Goal

Make the skill always on in Claude Code, the harness the experiment runs in, and
build the pieces every other harness reuses: the shared hook scripts and the
switch. Claude Code gets the whole always-on layer: the instruction at session
start, the reminder before each prompt, the document gate, the reply log and
the switch.

## Context

- The design `docs/specs/technical-communication.md` fixes the layers, the
  switch, the log and the gate. `harness-hooks-a.md` holds Claude Code's hook
  events, payloads, configuration schema and pitfalls, with the installed
  version they were checked against.
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
- The running log says whether hooks also ran in a subagent session.
- The reminder is within the size budget in the spec.

## Notes

- Verify the live behaviour. An earlier release of this project shipped with a
  feature not wired in, and a green build did not show it.
- If hooks, gate and log do not fit one session, decompose and finish the first
  child (instruction, reminder and switch).
