# finish-scope-k76

**Integrates:** finish-scope-k75

## Goal

Integrate the four findings from `finish-scope-k75` before
`lifecycle-scope-k72` consumes the changed state vocabulary and model claims.

## Context

Read the review's `## Findings` verbatim; its `path:line` citations are the
handoff. The producer is `finish-scope-k71`, and its committed change is the
review baseline.

The findings are coupled around the difference between a state predicate and
the outcome selected from it:

1. `FN-25.a` says the diagnosis definitions are disjoint, while the contract
   also declares reachable overlap and Alloy exempts it
   (`docs/specs/semantic-contract.md:891-904`, `946-955`, `1950-1959`;
   `crates/grove-finish/models/finish.als:1339-1382`, `4906-4931`).
2. Every standing quarantine is classified `Reserved(Quarantined)` and described
   as unfinished, even though the fourth proof can already have returned
   `Committed` and the finish can be `Applied` with cleanup outstanding
   (`docs/specs/semantic-contract.md:414-418`, `1845-1848`, `2023-2034`;
   `crates/grove-finish/models/finish.qnt:756-767`, `1722-1731`;
   `src/finish_transaction.rs:1953-1974`).
3. Alloy's `EN-08`/`FN-31.c` cell is declared unmeetable from an estimated
   seventeen-state trace against a thirteen-state bound, without the deeper run
   that would distinguish cost from impossibility
   (`docs/specs/semantic-contract.md:1130-1150`;
   `crates/grove-finish/models/README.md:1565-1582`).
4. The `W9SlotPending` behavior is `NoOp`, but `FN-11`'s comments still call the
   early branch a reachable refusal
   (`crates/grove-finish/models/finish.als:1841-1859`, `2940-2950`).

## Done when

- `FN-25.a` makes one coherent, falsifiable claim: either its two state
  predicates are genuinely disjoint, or it explicitly claims that precedence
  selects exactly one diagnosis from overlapping predicates. The contract,
  glossary, Alloy and Quint classifiers, checks, witnesses, controls and README
  all state and test that same claim.
- The state vocabulary distinguishes the unsettled post-rename window from a
  proven success whose disposal is merely outstanding, or gives one state a
  meaning and consequences that are valid for both. `FN-22`, `FN-24`, `FN-28`,
  `SY-05`, both finish models, both ADRs, `CONTEXT.md`, and the handoff in
  `06-design-lifecycle-scope-k72.md` are reconciled; a disposal failure after
  the fourth successful proof cannot turn `Applied` back into unfinished.
- The Alloy `EN-08`/`FN-31.c` cell is supported by the deeper positive witness
  and crash-removed negative control, or is recorded consistently as a bounded
  gap rather than a logical incompatibility. The exact command, bound, result
  and cost are recorded in the finish README and reflected in the assumption
  table.
- `doCommitAttempt`, `FN-11` and the finish README consistently describe the
  early ordering guard as an internal `NoOp`; the command still has reached
  antecedents and the applied-after-evacuation witness remains reachable.
- Every affected obligation has the required property, witness and isolating
  control in both families, or that family's precise declared gap. Run the
  finish-scope verification and any changed control/mutation cells against the
  final files, and record the commands, bounds and results here.
- Reconcile `finish-controls.qnt`, both durable handoffs and the ADR set after
  the four repairs. The product question already routed to `handoff-audit-k66`
  — whether the shipped reaper should re-read repository disposition — remains
  there unless a repair makes it moot.

## Notes

This leaf was inserted at the first live sibling after the review. Do not let
`lifecycle-scope-k72` absorb the repairs: it consumes both the state order and
the meaning of `Reserved(Quarantined)`, so it must start from the integrated
version.
