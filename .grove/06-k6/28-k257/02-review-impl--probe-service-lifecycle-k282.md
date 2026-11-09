# probe-service-lifecycle-k282

**Reviews:** probe-service-lifecycle-k280

## Goal

Read the probe launcher and the switch's probe-service cleanup with fresh
context. Try to break their contract of ownership and signals.

## Context

- The contract: "Probe process lifetime" of
  `docs/specs/owner-skill-delivery.md`. A probe signals only a process that
  it created. PID reuse never authorises a signal. An existing daemon
  refuses without a signal. No owned process survives the cleanup.
- The artifact: `scripts/owner_switch_probes.py` (`Tracker`, `Launch`,
  `existing_harnesses`) and, in `scripts/owner_switch.py`, `owned_state`,
  `end_owned`, `probe_owned`, `read_journal`, `run_probe` and
  `end_probe_services`.
- The evidence: the section "Probe service lifecycle" of
  `docs/owner-skill-delivery-evidence.md`, `task owner:probes:controls` and
  the rehearsal's `serial-probe-owned-*` controls.

## The specific doubt

An in-session adversarial read found ten defects. probe-service-lifecycle-k280
fixed them substantively, so this review checks the fixed code. Attack these
points first:

- The tracker's roots after the leader is reaped, and the order of the
  process-table read and each identity read.
- The cleanup rounds and the `watch` callback of `end_owned`: can a process
  that an owned process creates while it ends still escape its record?
- `run_probe` after an interrupt or a timeout of the switch: does the probe
  receive SIGTERM, and does the switch end each journal record?
- The validation of journal records: a PID above 1, created after the probe
  started.
- `owned_state` for a PID that exists but cannot be read.

The interval between the identity check and the signal, and an escape faster
than the 50 ms reading, are recorded limits. Judge whether either needs more
than its record.

## Done when

- Each finding names its file and line, a concrete scenario and a severity,
  or the review states that it found none.
