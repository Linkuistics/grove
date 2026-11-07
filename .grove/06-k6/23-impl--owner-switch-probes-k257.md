# owner-switch-probes-k257

## Goal

Write the probe program that the switch plan names for step 10 and for the
native probe of Rollback. The plan's `probes` and `native_probe` run it.

## Context

- owner-switch-plan-draft-k247 named the interface in
  `docs/owner-cutover/switch-plan-draft.json`. The program is
  `<state>/prepared/owner_switch_probes.py`, run as `$python -I` with a path
  relative to HOME:
  - `resolve --expect switched` and `resolve --expect disabled`.
  - `session --harness claude|codex --route ravel|native`.
- The sections "The switch inside finish" (step 10 and "Each probe must
  observe these results") and "Rollback" of
  `docs/specs/owner-skill-delivery.md` state what each probe observes.
- The program's docstring in `scripts/owner_switch.py` states the
  environment of a command. It runs in HOME, with HOME set, and it receives
  `OWNER_SWITCH_PLAN`, so a probe can read `accepted.sources` for the
  accepted digests. It inherits no loop control variable, no Claude Code
  location variable, no `CODEX_HOME` and no `RAVEL_*` name.
- `scripts/harness_trial.py` already extracts reads, delegations and
  transcripts from real Claude Code and Codex sessions. Reuse its mechanics
  where they fit.
- The staging HOME of owner-staging-setup-k248 is the place to develop the
  session probes against keg sources without the real HOME.

## Done when

- The program implements the interface, or a changed interface is recorded
  for owner-switch-plan-k254 to put into the plan.
- `resolve` uses the promoted `$(brew --prefix)/bin/ravel` and the HOME's
  configuration. `switched` requires the ten sources with their accepted
  digests and the mandatory file. `disabled` requires the status `disabled`.
- A `ravel` session requires each probe observation of step 10 that the
  harness supports:
  - The agent reads the mandatory file first.
  - The agent reads an unnamed craft skill before its action.
  - The agent reads a named Grove kind skill from the `grove-core` snapshot.
  - The agent reads `using-testanyware`, runs `testanyware llm-instructions`
    and runs `testanyware doctor`.
  - On Claude Code, the hooks run.
  - Native discovery lists no superseded skill.
  - Each source gives its accepted digest.
- A `native` session requires native discovery of the superseded skills and
  a native read of a Grove kind skill. The kind is not `finish`, and the
  task asks only for one quoted line.
- Each probe launches outside every Grove workspace and fails with a clear
  message. Its last line is `owner-switch-result: <JSON>` with its evidence.
- Each mode passes once in the staging HOME, or carries a recorded limit.
  A control shows that each mode fails for a wrong layout.

## Notes

The probes run inside finish against the real HOME. They must change no
entry of the plan's `outside` manifest. A probe session that rewrites a
platform folder is the reason the draft names no root manifest for
`.claude/skills` or `.codex/skills`. Record whether a probe session's MCP
servers, such as `codebase-memory-mcp`, rewrite their skill copies or agent
definitions when the session launches them. The plan's `outside` manifest names those entries.
