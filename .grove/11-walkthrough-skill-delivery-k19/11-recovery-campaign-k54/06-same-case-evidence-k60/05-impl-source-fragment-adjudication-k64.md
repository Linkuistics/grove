# source-fragment-adjudication-k64

## Goal

Audit and publish the exact historical-rubric source/fragments result.

## Context

- Assignment records: `source-fragment-runs-k63`.
- Frozen auditor, normalized bundle, scorer, criteria, and outcome contracts:
  `campaign-freeze-k59`.

## Done when

- Complete attempt history is replayed against the historical Case B access,
  replacement, sampling, and scoring rules before acceptance scoring. Only the
  historical B01–B27 rows determine the accepted result.
- Stronger exposure classification, two-scorer coverage beyond the one complete
  case required by the historical rubric, forced arm guesses, and new rows are
  excluded and cannot fill a shortfall.
- The record distinguishes judged presence of inventory, reader order,
  fragment ownership, worked execution, and proposed validation from the
  deterministic fact that the campaign fixture and manifests stayed exact.
- Historical per-row counts, invalid-attempt counts, truncations, discovery
  count, and incompleteness are published with their exact rubric semantics;
  new pair-aware, absolute, arm-guess, and fail-closed rules are out of scope.
- Because Case B has no valid historical baseline operand, its materiality and
  regression classifications are published as `undefined`; descriptive counts
  and any terminal shortfall are not converted into zero, miss, or omission.

## Notes

An answer promising byte equality is behavioral evidence; only the harness and
fixture comparison establish campaign-byte equality.
