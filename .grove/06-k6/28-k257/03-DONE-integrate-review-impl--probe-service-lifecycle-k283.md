# probe-service-lifecycle-k283

**Integrates:** probe-service-lifecycle-k282

## Goal

Triage the implementation review and apply its actionable findings before
the session probes build on this launcher.

## Context

- Read the findings from probe-service-lifecycle-k282's commit, in
  `docs/owner-skill-delivery-evidence.md`, "Probe service lifecycle
  implementation review".
- The reviewed producer is probe-service-lifecycle-k280. The node brief
  and `docs/specs/owner-skill-delivery.md`, "Probe process lifetime", state
  the contract.

## Done when

- Each finding has an evidenced disposition, the applicable corrections
  are implemented and verified, and the durable evidence states the result
  and its limits.

## Notes

session-probes-k281 follows this integration and consumes its settled launcher.

## Decisions (running log)

- L1, L2, L3 and L4 are real issues: each one reproduces from the source at
  the producer commit. None asks for a redesign, so this session fixes all
  four.
- L1: `owned_record` accepts a null executable. `owned_state` already calls
  such a record `changed`, so it authorises no signal and leaves recovery
  incomplete.
- L2: one shared rounds loop, `end_in_rounds`, serves the launcher and the
  switch. The switch's watcher records each child that a recorded process
  creates after itself. A process-table failure during switch-side cleanup
  becomes `Unresolved`, never `Stop`, so restore cannot follow it.
- L3: `run_probes` alone discharges the obligations of every started probe
  on each exit path. `run_probe` only stops its probe. A cleanup failure
  replaces an ordinary error, and an interrupt continues after a logged one.
- L4: the launcher's exception path runs the owned-process cleanup, adds any
  unresolved identity to a `Fail`, and keeps the work folder when a record
  remains unresolved.
- The stored plan draft keeps the producer's program digests. Its generator
  reads the real HOME, session-probes-k281 changes the probe program again,
  and owner-switch-plan-k254 regenerates the draft. The evidence states this
  limit.
