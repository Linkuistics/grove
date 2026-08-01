# producer-target-receipt-k15

**Kind:** impl

## Goal

Record the effective producer launch target during successful retirement of a
reviewed producer without making advisory metadata lifecycle-critical.

## Context

Consume the stable relationships and tree-access seam from
`promotion-transaction-k14`. Implement the structured routing peek, scrubbed
`GROVE_SESSION_TARGET` launch context, relationship/receipt parsing, and the
post-`DONE` best-effort atomic receipt replacement.

## Done when

- One guarded structured peek supplies path, stable handle, kind, and declared
  harness to readiness, routing, launch diagnostics, and session context.
- Only the real foreground session receives validated target context; every
  other harness spawn and the test environment scrub it.
- Retirement writes `DONE` first, then unconditionally replaces the unique
  sibling review receipt when worktree, routed handle, and factual pick agree.
- Missing, stale, ambiguous, malformed, or unwritable metadata diagnoses an
  uncheckable receipt without reversing or blocking retirement, with focused
  Git/Jujutsu and failure tests green.

## Notes

Do not implement review-target comparison or prompt/stderr warnings in this
slice; they consume the receipt in the next child.
