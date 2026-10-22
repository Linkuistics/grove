# ravel-job-k18

**Integrates:** ravel-job-k17

## Goal

Triage the findings of `ravel-job-k17` and apply the valid corrections to the
implementation produced by `ravel-job-k7`.

## Context

- Read the findings from the committed review `ravel-job-k17`.
- The producer is `ravel-job-k7`. The root brief supplies the caller contract
  and the agreed test seams.

## Done when

- Each finding has a recorded classification and its supporting evidence.
- Valid corrections include the corresponding source-book changes.
- Required checks pass for the corrected implementation.

## Notes

This step precedes both caller migrations. The findings remain in the review,
so this task can reject a finding when the evidence warrants rejection.
