# harness-selection-and-execution-k9

**Reviews:** harness-selection-and-execution-k8

## Goal

Independently assess the whole current harness-dispatch design against the
accepted requirements and current Grove contracts before planning turns it into
implementation work. This covers the redesigned original-creator mechanism and
the integration repairs no review has yet read.

## Context

Read the producer's committed artifact via its stable handle, against the whole
current design, not only k8's diff:

- `docs/specs/harness-selection-and-execution.md`.
- The ADRs `harness-selection-is-owned-by-policy`,
  `policy-evaluation-precedes-process-replacement` and the new
  `a-review-carries-its-creator-reference`.
- The glossary entries the design uses.
- `docs/design/harness-selection-and-execution/`.

That design includes the repairs `harness-selection-and-execution-k7` applied
for review `harness-selection-and-execution-k5`: F6–F11 and the human's Linux
floor choice for F7. No review has read those repairs. The requirements
authority is the root brief, with the two acceptance sentences the human amended
in k8, together with the k1, k4, k7 and k8 running logs. The design is not
evidence of implemented behaviour or of new human approval beyond the choices
those logs record.

## Done when

- Findings distinguish contract gaps, unnecessary obligations and accepted
  trade-offs. Source or runtime evidence supports each one where it bears on the
  conclusion.
- The whole acceptance surface is assessed: standalone use, runtime delivery,
  authority, context limits, the original creator, records and both process
  seams. The spec and ADR set are coherent and claim no implementation results.
- If findings warrant integration, the integration leaf is inserted immediately
  before planning `harness-selection-and-execution-k6`. Its body names this
  review rather than copying the findings.

## Notes

Producer doubts worth adversarial examination:

- The creator reference depends on a session copying its own run ID. Test
  whether a wrong but existing run can pass. The design dropped a
  task-identity equality check because a decomposed producer is finished by a
  child task with its own handle. Decide whether that trade-off holds.
- The finishing producer writes its line on the review it cuts and on any live
  review already naming its handle. Check that a session can follow this, and
  that crash restarts, correction runs and decomposed producers all reach a
  deterministic creator.
- The methodology amendments are stated in the new ADR rather than applied, and
  ship with the implementation. They touch TASK-FORMAT, retire.md,
  decompose.md, the "no code reads them" wording and their conformance rows.
  Check that they are precise enough to implement and that nothing else in the
  corpus contradicts a `**Creator:**` line.
- Removing the scope and registration must not lose anything the requirements
  need, such as later outcome association or records after teardown.

The producer spent no in-session reviewer; this scheduled review is the
adversarial read.
