# owner-switch-simplification-k262

## Goal

Reshape the owner-switch leaves of this node for the reviewed simplified
switch design.

## Context

- owner-switch-simplification-k260 states the simplified switch and its
  consequence for each live leaf from owner-staging-setup-k248 to
  owner-switch-rehearsal-k255. owner-switch-simplification-k261 reviewed it,
  and an integration of that review may follow before this leaf.
- The parent brief's "Owner-delivery handoff" assigns many items about the
  removed mechanisms to those leaves and to discovery-rollout-k12.

## Done when

- Each live owner-switch leaf serves the simplified design. Rewrite its
  body, or replace it with the implementation that the design needs. An
  example is the removal of program code, the rehearsal, the model and the
  `mac-check` states.
- The parent brief's "Owner-delivery handoff" is current. Each item about a
  removed mechanism is gone or rewritten.
- A leaf that the design makes unnecessary is pruned only after the human
  confirms it. Otherwise it stays live, with its body stating the open
  question.

## Notes

The cutover's ordering inside finish and the finish handoff of
discovery-rollout-k12 stay consistent with the reviewed design.
