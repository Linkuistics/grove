# harness-wrapper-k5

**Integrates:** harness-wrapper-k3

## Goal

Triage `harness-wrapper-k3`'s design findings against the settled requirements
and apply those that hold, leaving a coherent design that
`supervised-dispatch-k4` can plan. This session owns design corrections and
their verification, not implementation of the harness wrapper.

## Context

- Read the findings in `harness-wrapper-k3`'s committed review artifact, and
  its diff against its parent. Classify each finding on evidence rather than
  treating it as an agreed work list.
- The reviewed producer is `harness-wrapper-k2`, commit `f653dd72`. Its
  specification, ADRs, related specs, glossary and visual design are the
  artifacts to reconcile where triage supports a correction.
- The root brief carries the owner's requirements and four agreed seams.
  `plan-k1`'s running log holds the owner's answers; `harness-wrapper-k2`'s
  running log explains the design choices. Planning follows this task.

## Done when

- Every finding has an explicit, evidenced disposition in this task's running
  log, including any rejection or accepted trade-off.
- Supported corrections are applied consistently to the design artifacts and
  their acceptance obligations, and verified by this kind's procedure.
- Any remaining requirement-level trade-off is resolved or externalized before
  planning consumes the design.

## Notes

This grove still runs the installed v22 binaries and skills. Do not change the
installed signal contract as part of design integration; end this session with
the verb its launch prompt and installed skill name.
