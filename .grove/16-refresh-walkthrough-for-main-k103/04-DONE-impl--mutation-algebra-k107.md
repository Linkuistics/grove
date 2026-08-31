# mutation-algebra-k107


## Goal

Refresh the pure mutation algebra against the current operations, planning, and
reporting sources.


## Context

- Source roots: `crates/ordinal-fs-tree/src/ops.rs`, `src/plan.rs`, and
  `src/report.rs`.
- Book surfaces: `05-mutation-algebra.md` and `source-index.md`.

## Done when

- All three owned roots tangle byte-for-byte and their inventory entries are
  current.
- Exposition accurately covers the current operation set, decisions,
  refusals, effects, reports, and whole-tree deletion semantics.
- Full validation has no mismatch for an owned root.

## Notes

Keep filesystem interpretation out of this pure-algebra slice.

## Decisions (running log)

The existing source-coherent fragment IDs remain stable. The operations source
adds one literal `ops-initialize` fragment for lines 120–183 and repartitions
the surrounding literals at semantic boundaries; plan refusals split at
433/434, and report structure now includes the path-based `Removed` value
through line 153 while hand-written debug implementations remain separate.

Initialization is explained as ordinary plan algebra over an empty snapshot,
including its shared `NoDistinguishedChild` refusal. Whole-tree deletion is
explained only at the pure seam: it cannot be a name-based plan because it acts
on the root and foreign entries, and its successful value reports paths. Root
walking, partial failure, and other filesystem interpretation remain owned by
`filesystem-lifecycle-k108`.

The validator constants, validator fixture, book-system spec, and source ledger
are current-state mirrors of the owned source roots. This leaf updates all four
to 634 operations lines, 597 plan lines, and 186 report lines, raising the
mutation slice to 1,417 owned lines and the current sixteen-root corpus to
7,527 lines.

The orientation page's early-use statement now matches the current `NewEntry`
shape: its byte vector may be empty but is not optional. The mutation page
states that `Vacancy::initialize` constructs the empty snapshot used for
planning rather than implying that the vacancy stores a snapshot.
