# recoverable-marker-replacement-k149

**Kind:** impl

## Goal

Replace an auxiliary marker without an unconditional overwrite window, using a
recoverable state that preserves substituted marker bytes across interruption.

## Context

- Validate-then-rename is racy: a marker substituted between those operations
  is overwritten by an unconditional replacing rename.
- Recovery must be able to distinguish and resolve any intermediate exchange
  before trusting the canonical marker.

## Done when

- Each marker-replacement state is parseable and tied to the same handle and
  attempt.
- A substitution at every exchange boundary fails closed without deleting or
  adopting external marker bytes.
- Unit tests cover normal completion, interruption recovery, and substitutions.

## Notes

Keep the primitive generic to auxiliary markers; do not special-case the Git
success-index role.
