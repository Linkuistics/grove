# transfer-adjudication-k70

## Goal

Audit and publish the exact historical transfer verdict.

## Context

- Assignment records: `transfer-runs-k69`.
- Frozen transfer criteria, auditor, scorer instruments, thresholds, and
  unavailable semantics: `campaign-freeze-k59`.

## Done when

- Complete acceptance attempt history is replayed against the historical
  transfer access, replacement, sampling, and scoring rules. Only the frozen
  T01–T20 criteria and historical thresholds determine that verdict.
- New dual scoring, forced arm guesses, exposure classifications, and fail-
  closed rules are excluded.
- The report publishes every historical atomic count, material delta,
  regression classification, invalid-attempt count, truncation, discovery count,
  and required adjudication disagreement.
- Historical invalid and shortfall outcomes retain their exact frozen meaning.
  The result makes no same-case, population, reader-outcome, or wording-
  causation claim.
- The result is labelled either an amended acceptance conjunct or separately
  reported generality evidence exactly as settled by
  `acceptance-replication-authority`; it never derives acceptance authority from
  the parent brief's `Context` sentence.

## Notes

The transfer result remains visible even when adverse, but cannot be pooled with
the larger same-case row set or used to change treatment bytes. It is required
for parent acceptance only if the requirements decision explicitly amends the
parent `Done when` to say so.
