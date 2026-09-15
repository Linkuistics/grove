# item-status-k7

**Reviews:** item-status-k6

## Goal

Adversarially review the item-status process-death correction before
`item-status-k4` cuts implementation work. Find where the proposed runtime
protocol or its evidence fails the root brief's truthful RUNNING and
no-rebinding contract.

## Context

Read the producer's commit by its stable handle, the current
`docs/specs/item-status.md`, and the driver-lease ADR it cites. The spec owns
protocol and typed results; the ADR owns rationale, primary-source evidence and
trade-offs. Earlier review `item-status-k3` and integration `item-status-k5`
explain the correction's origin but do not establish its correctness.

Inspect arbitrary relative release of the private witness and directory pin,
observation opened after replacement, inode/key reuse, the final probe order,
and replacement waiting with predecessor epoch bytes. Challenge the assumption
that exclusive directory contention belongs to the witnessed launch, including
whether reserving this lock location is compatible with required participants.
Check that the supported filesystem boundary is explicit and implementable,
and that the Linux/Darwin sources support each lifetime claim actually made.

The producer ran a temporary macOS primitive probe, not the future observer or
a Linux implementation test. Assess the proposed real-process and forced-reuse
controls on their own merits: they must discriminate the claimed mechanism from
blanket unavailability. Keep the short runtime-only epoch guard, its explicit
suspension trade-off, and the previously integrated rendering, selection and
workspace-identity requirements in scope for regressions.

## Done when

Findings or an explicit no-findings result are recorded against the producer's
commit with source locations and reasons. Any integration follows this review
before `item-status-k4`; its charter points to this review rather than copying
the findings as obligations.
