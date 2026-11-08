# owner-switch-probes-k257

## Goal

Write the probe program for the switch's probes and the native probe of
restore. Give each probe launcher a measured service lifecycle, and make the
switch program verify that each probe-owned service ends.

## Context

- owner-switch-plan-draft-k247 named the interface in
  `docs/owner-cutover/switch-plan-draft.json`. The program is
  `<state>/prepared/owner_switch_probes.py`, run as `$python -I` with a path
  relative to HOME:
  - `resolve --expect switched` and `resolve --expect disabled`.
  - `session --harness claude|codex --route ravel|native`.
- `docs/specs/owner-skill-delivery.md` states what each probe observes, in
  "The switch inside finish" and "Rollback". "Probe process lifetime" states
  the lifecycle. "Concurrent writers" states the comparison of the native
  skill roots.
- A command runs in HOME, with HOME set, and receives `OWNER_SWITCH_PLAN`.
  It inherits no loop control variable, no Claude Code location variable, no
  `CODEX_HOME` and no `RAVEL_*` name.
- `scripts/harness_trial.py` already extracts reads, delegations and
  transcripts from real Claude Code and Codex sessions. Reuse its mechanics
  where they fit.
- The staging HOME of owner-staging-setup-k248 is the place to develop the
  session probes against keg sources without the real HOME.
- owner-switch-process-inspection-k267 and owner-switch-restore-k268 leave a
  seam for ownership evidence from a probe result.

## Done when

- The program implements the interface, or a changed interface is recorded
  for owner-switch-plan-k254 to put into the plan.
- `resolve` uses the promoted `$(brew --prefix)/bin/ravel` and the HOME's
  configuration. `switched` requires the ten sources with their accepted
  digests and the mandatory file. `disabled` requires the status `disabled`.
- A `ravel` session requires each probe observation that the harness
  supports:
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
- Measurements in a fresh HOME without a daemon cover normal exit, failure
  and interruption. They show which Codex services outlive the probe.
- Each launcher uses a measured mode that prevents persistent services, or a
  supported shutdown of a service whose creation it proves. Its evidence
  names process identity, creation time, executable and configuration HOME.
- The switch program ends each owned service and verifies its exit before
  final inspection or restore. It reaps its children. An existing daemon
  refuses without a signal. PID reuse never permits a signal.
- Probe comparisons see each ordinary child of both native skill roots. They
  skip only the exact subtrees `synced` and `.system`.
- Each mode passes once in the staging HOME, or carries a recorded limit.
  A control shows that each mode fails for a wrong layout. Fixture controls
  cover reparenting and PID reuse.

## Notes

The probes run inside finish against the real HOME. Record each file that a
probe session writes, and whether its MCP servers rewrite skill copies or
agent definitions. No root-wide exclusion may hide such a write. An
incidental write is a finding for the plan's expected inventories.

If the measured launcher cannot establish ownership or cleanup, no real
switch is possible. Record that as a blocking finding for the owner.
