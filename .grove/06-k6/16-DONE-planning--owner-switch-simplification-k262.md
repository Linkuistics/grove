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

## Decisions (running log)

- The increments stay in this node as ordered siblings, not separate groves.
  The root brief makes this grove the only executor, and finish owns the
  cutover. A separate grove would need its own finish and release.
- The design prunes no queued leaf, so no prune needs the human's
  agreement. Each live leaf keeps useful work after its rewrite.
- The inventory exceptions D3 and D5 come first. owner-staging-capabilities-k250
  and the switch comparison consume that instrument.
- The model removal is its own small leaf. `scripts/check.sh` does not run the
  model, and no fixture control depends on it.
- `native-off` is an additive increment. Finish needs it right after
  promotion, whatever happens to the rest of the program.
- The narrow process recogniser replaces the process and job patterns in one
  increment. Its plan fields go with it, and the draft generator follows.
- The switch core follows expand, migrate and contract. The new `switch`,
  `restore` and attempt notes come first, beside the old run. `mac-check` and
  the purge report then move to the new status. The contraction then removes
  the old machinery and the plan's recovery fields. Each step leaves the
  fixture suite green.
- The dry-run instrument and its comparison command follow the contraction.
  owner-switch-rehearsal-k255 executes it on the real plan.
- owner-switch-probes-k257 moves ahead of owner-staging-hooks-k251. The
  idle-parent measurement covers both probes. k257 also owns the switch
  program's verification of probe-owned services.
- The parent brief keeps the artifact consequence table as context for the
  implementation leaves. The per-leaf table is gone, because each leaf body
  now holds its consequence. The handoff is regrouped by consumer.
- No in-session reviewer and no review-planning leaf. k261 already reviewed
  the design, and this plan maps its consequences. Each implementation leaf
  carries executable fixture controls that expose a wrong boundary.
