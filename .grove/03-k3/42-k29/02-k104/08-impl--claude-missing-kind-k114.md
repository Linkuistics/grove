# claude-missing-kind-k114

## Goal

Observe the native-only missing-kind case in Claude Code. Separately, observe
the unreadable confined case.

## Context

The Source and skill failures and Confined standalone rows of Grove's delivery
specification. The native-only rows of its Supported routes section.

## Done when

- Missing kind: the case runs on the native route on 2.0, with delivery
  disabled. The sandbox native roots hold the spine and lack the leaf's kind
  skill.
- The agent reports the missing kind skill. It runs the exact force-stop
  command of its launch prompt.
- After the run, the leaf is still live. No teardown is recorded, no output
  is published and the driver does not launch again.
- Unreadable confined: a confined `grove run` with injection disabled has
  native trees outside every grant. The agent reports the unreadable guidance
  and force-stops, and Grove publishes no output.
- The two cases have separate records. Neither credits the native stage's
  fixture.

## Notes

If Claude Code cannot run under Ravel confinement, record the unreadable
confined case as a gap with the refusal.

The force-stop command reaches only the trial's own Ravel channel.
