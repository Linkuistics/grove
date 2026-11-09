# probe-service-lifecycle-k280

## Goal

Deliver the probe program's measured launcher and its `resolve` mode. Make
the switch program end each probe-owned service and verify its exit.

## Context

- The node brief states the interface, the environment rule and the whole
  Done when. This leaf takes the lifecycle part and `resolve`.
- `docs/specs/owner-skill-delivery.md`, "Probe process lifetime" and "The
  narrow process inspection".
- `scripts/owner_switch.py`: `end_probe_services` and `probe_owned` are the
  seam. `process_start`, `executable_path` and `process_arguments` read
  process identity.
- The evidence section "S2" of `docs/owner-skill-delivery-evidence.md`
  records Codex daemons that outlived earlier trials.

## Done when

- `scripts/owner_switch_probes.py` implements `resolve --expect switched`
  and `resolve --expect disabled` as the node brief states. Each passes once
  in the staging HOME, and a control shows that each fails for a wrong
  layout.
- The program has a launcher that the session probes can use. It runs a
  harness outside every Grove workspace and tracks the processes that it
  creates. Its last line is `owner-switch-result: <JSON>`.
- Measurements in a fresh HOME without a daemon cover normal exit, failure
  and interruption for both harnesses. They show which services outlive the
  launch.
- The launcher uses a measured mode that prevents persistent services, or
  it proves the creation of a service and ends it through a supported route.
  Its evidence names process identity, creation time, executable and
  configuration HOME.
- The switch program ends each owned service and verifies its exit before
  final inspection or restore. It reaps its children. An existing daemon
  refuses without a signal. PID reuse never permits a signal.
- Fixture controls cover reparenting and PID reuse.

## Notes

If the measured launcher cannot establish ownership or cleanup, record a
blocking finding for the owner. No real switch is then possible.

## Decisions (running log)

- The first session of owner-switch-probes-k257 decomposed it. This leaf
  takes the launcher, `resolve`, the switch-side cleanup and the fixture
  controls. session-probes-k281 takes the four session modes.
- The probe finds Ravel as the switch does: `brew` on PATH, then
  `$(brew --prefix)/bin/ravel`. A staging pass puts a stub `brew` first on
  PATH, whose prefix holds the accepted 2.0 build. The interface stays as
  the plan draft names it.
- The probe imports `owner_switch` from its own folder for process identity
  and the narrow recogniser. Both the repository and `<state>/prepared` hold
  the two files side by side. The dry run's input table records the import
  as a helper.
- One function in `owner_switch.py` ends owned processes. The probe uses it
  for its own cleanup, and the switch uses it after the probes. It signals a
  process only when its PID, creation time and executable still equal the
  record, read again immediately before each signal.
- The probe writes each process record to a journal as soon as it sees the
  process. The switch names the journal in `OWNER_SWITCH_PROBE_JOURNAL` and
  reads it after the probe, so a probe that dies without its result line
  still leaves its records.
- The interactive TUI path launches the Codex daemon. The owner's
  `config.rhai` launches the TUI, so the `ravel` route runs
  `ravel start-session --config` with a headless probe script instead. The
  lifecycle measurement tests this choice.
- The first lifecycle measurement settled the launch mode. `codex exec`
  launched no daemon in any HOME, including one that holds an installed
  daemon package. MCP servers and Codex helpers ended with their harness.
  Only a crash of the harness left survivors: the orphaned shell command.
  So the measured mode is the headless command of each harness, and the
  launcher ends any survivor by its record.
- Ravel 2.0 fails a session whose managed process exits without
  `ravel end-session`. The probe's script therefore wraps the harness in
  `/bin/sh -c`, which ends the session with the harness's exit status. The
  result then does not depend on the agent.
- A record or process whose executable cannot be read has an unconfirmed
  identity, and nothing signals it. One Codex leader read as unreadable for
  its whole life once, and four repeats did not reproduce it.
- The staging pass of `resolve` uses the staging settings, which name the
  ten sources. The real switched settings omit `sources`. Resolution
  through the promoted Ravel's default directory stays with the finish
  handoff's repeat after the Ravel cut.
- The leaf spent its one in-session reviewer on the signal code. Of its ten
  findings, eight were valid. Each fix has a control. The fixes cover the
  roots after the leader's reap, the read order and journal validation. They
  also cover children during cleanup, interrupts of the switch, unreadable
  PIDs, the exception path and the PID-only exclusion. The interval before a signal and the fast
  escape stay recorded trade-offs, and the fast escape gains evidence.
  Forged journal records from a hostile agent fall outside the quiet-interval
  model, apart from the cheap validation. The fixes were substantive, so
  probe-service-lifecycle-k282 reviews them.
- A fixed control exposed one more defect: the switch waited for the end of
  the probe's output pipes, which a leftover service can hold. `run_probe`
  now writes the probe's output to files and waits for the probe alone.
