# system-k60

**Integrates:** system-k59

## Goal

Repair the Quint lifecycle instrument so its search relation, guard ownership,
negative controls and durable measurements support the claims the column makes.

## Context

Read the six findings and discharged doubts in `system-k59` before changing
anything. The reviewed producer is `system-k13`; its artifacts are
`models/system/lifecycle.qnt`, `models/system/lifecycle-controls.qnt`, the Quint
column of `models/system/README.md`, and entry 046 of
`docs/formalism-findings.md`.

The independence barrier still applies. Do not open an `.als` file, an Alloy
section of a model-directory README, or entries 026–043. This leaf integrates a
review of the Quint instrument; cross-model replay remains `cross-model-replay-k15`.

The review found:

1. `driverStep` forces a launched finish session through teardown, omitting
   decline, early failure/no-signal, and add-work/relaunch, and it interprets the
   signal before closing the epoch while production invalidates first.
2. `launchOp` models the foreground session as continuously holding the launch
   generation, but production gives a shared epoch guard to each ambient
   operation only; the current `SY-11.b` mutant is not evidence about Grove's
   real wait graph.
3. `mutant_no_signal_is_done` leaves `SY-09.a` and `SY-09.b` green; the README's
   claimed bundle control does not exist.
4. `sweepBlocked` treats any `OApplied` as clearing a block, so the claimed
   literal `SY-14.a` failure is a measurement defect.
5. The flat-menu 5-of-25 result is not retained or reproducible, and the README
   contradicts itself about 23/25 in base versus 21 base plus four scenarios.
6. `SY-05.b` checks two values manufactured by one summary setter rather than an
   independently composed task-tree/finish observation.

## Done when

- `driverStep` represents every real successful/failed finish-session ending
  and matches the production epoch-invalidation/signal-interpretation order, or
  each deliberately omitted ordering is justified against current Rust source.
- The `SY-11.b` wait graph models per-operation shared epoch ownership and the
  driver's exclusive activation/invalidation handoff. Its negative control
  remains only if that faithful graph has a reachable cycle.
- Isolating negative controls exist for `SY-07.a`, `SY-09.a`, `SY-09.b`, and
  `SY-14.b`. Each target fails while its neighbour remains green.
- The `SY-14.a` sweep detects a block-state transition, not mere action success;
  the `touchesTree` narrowing and entry 046 finding 3 are retained, revised or
  withdrawn according to that corrected experiment.
- The flat-menu comparison is either reproduced from a committed, runner-owned
  instrument at both 2,000 and 8,000 samples or removed. README instance and
  witness counts agree with the runner's actual module rule.
- `SY-05.b` is restated as the internal consistency check it is, or the model
  consumes independently established component observations so the stronger
  composition claim becomes falsifiable.
- The `SY-13` catalogue contradiction, the environment-budget result, the two
  thin witnesses and the verified 8/19 shortest depths remain recorded with the
  review's corrected evidence.
- `models/run.sh --scope lifecycle --family quint` is green with all coverage
  cells, witnesses and controls non-empty, and the exact command, counts, seed,
  depth, tool version and wall time are reconciled in the README and entry 046.

## Notes

Do not preserve a producer conclusion merely because entry 046 already calls it
a catalogue finding. The review distinguished catalogue defects (`SY-13`) from
model/instrument defects (`SY-11`, `SY-14`) and from a claim that must be
downgraded unless the abstraction changes (`SY-05.b`).
