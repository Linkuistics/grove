# duplicated-prose-k36

## Goal

Remove two accidental consecutive sentence duplications found while
`classification-k35` was reading the durable classification evidence. Keep this
as a prose-only correction; it is not a unit-classification repair.

## Context

- `content/driving.md`, inside `driving-review-chain-habits`: “diff against the
  current source. Nothing was written down for intervening work” appears twice
  consecutively.
- `docs/specs/mandate-delivered-methodology.md`, in *A unit names the procedure it
  defers to*: “Together with partition, that yields the structural claim the
  design is actually” appears twice consecutively.
- Both defects predate the classification marker edits; preserve the surrounding
  meaning and make no marker changes merely because this leaf exists.

## Done when

- Each duplicated sentence appears once.
- The prose around each edit reads continuously.
- Focused documentation checks, if the repo supplies any, pass.

## Notes

This leaf is a new concern externalized by `classification-k35`; do not absorb
classification-review findings into it.
