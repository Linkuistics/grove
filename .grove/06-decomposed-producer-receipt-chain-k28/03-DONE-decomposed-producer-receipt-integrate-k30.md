# decomposed-producer-receipt-integrate-k30

**Kind:** integrate-review-design
**Integrates:** decomposed-producer-receipt-review-k29

## Goal

Apply the verified findings from `decomposed-producer-receipt-review-k29` while
preserving the reviewed artifact's contract.

## Context

Re-read the design artifacts and each finding in
`decomposed-producer-receipt-review-k29`. Reproduce the claim against the
current-state contract before editing the ADR/spec; keep implementation and
canonical documentation work in the already-cut
`decomposed-producer-receipt-implementation-k25` review chain.

## Done when

- Every finding is classified as contract misread, valid/actionable, visible
  trade-off, or noise.
- The ADR/spec are reworked in place for every verified issue without creating
  a superseding record.
- The implementation review chain remains an accurate, ordered work order for
  the integrated design.

## Notes

Externalise any new implementation concern rather than absorbing it into this
design integration leaf.

## Review reconciliation

- **R1 — visible trade-off.** The handoff source remains kind-agnostic: a nested
  review or integration leaf can factually close and hand off the aggregate.
  Restricting it to producer kinds would guess authorship. The ADR now records
  that rejected alternative.
- **R2 — valid/actionable.** Checkable warnings name a distinct source session;
  the ADR records why a contributor count adds no diversity guarantee and cannot
  repair the required silent-success case.
- **R3 — valid/actionable.** The ADR/spec now distinguish no node lifecycle mark
  from the advisory review-task write and anchor the no-question result on the
  confirmation boundary's second test. Canonical methodology wording remains in
  `decomposed-producer-receipt-implementation-k25`.
- **R4 — valid/actionable.** Compatibility now admits that old strict readers
  diagnose newer receipts as malformed; the designed reader contract ignores
  unknown keys while validating known fields.
- **R5 — valid/actionable.** Receipt replacement now skips terminal reviews, and
  the close-cascade contract states that at most one linked review can be live.
- **R6 — valid/actionable.** The ADR records that abandoning reviewed work means
  pruning the chain node; pruning only the producer deliberately schedules its
  still-live review next.
- **R7 — valid/actionable.** Receipt policy moved back behind the ADR citation;
  the spec retains wire shapes, module interfaces, lifecycle choreography, and
  test seams.
- **R8 — valid/actionable.** The decomposed examples and test seam now separate
  the closing session key from the producer generation key.
- **R9 — valid/actionable.** A prepended notice is addressed to its routed review
  handle and must be discarded when the session's factual pick differs.
- **R10 — valid/actionable, externalised.** The exact glossary and methodology
  reconciliation remains a done-when clause of
  `decomposed-producer-receipt-implementation-k25`, whose work order now names
  source session, generation, and advisory node-close effects.
- **R11 — valid/actionable in narrowed form.** The launcher peek,
  `grove-llm kind --with-harness --json`, stays the one guarded read; its help
  must expose that role and validated historical routing is nested under
  `producer-target` rather than overloading `harness` and `model`.

## In-session doubt reconciliation

- **D1 — contract misread.** “Ordered work order” describes the existing
  producer → review → integration chain, not an implementation recipe inside the
  producer leaf. Grove task files state outcomes and constraints rather than
  prescribing file-by-file construction order.
- **D2 — valid/actionable.** The implementation leaf now enumerates every
  canonical documentation and help surface instead of relying on category names.
- **D3 — valid/actionable.** Its test obligations now name the omitted
  relationship, schema, lifecycle, compatibility, and edit-preservation cases.
- **D4 — valid/actionable.** Policy restatements were removed from producer
  handoff, compatibility, and out-of-scope prose. The spec now delegates policy
  to the ADR and retains choreography, wire shapes, interfaces, and seams.
- **D5 — valid/actionable.** Only a checkable receipt's source session is factual.
  An uncheckable warning never presents syntactically valid but stale receipt
  evidence as the handoff session.
- **D6 — valid/actionable.** The spec now enumerates every receipt field's type,
  nullability, required/legacy status, and unknown-key behavior.
- **D7 — visible trade-off.** Tests can prove notice scoping and discard
  instructions, not absence of model influence. Mechanically withholding a
  stale notice would require binding the session to the routing forecast, which
  conflicts with factual pick winning.

These corrections clarify the reviewed decision and its work order without
changing the handoff strategy, lifecycle ownership, or module seams. They do not
constitute the substantial redesign that would require a new producer review
chain from this integration leaf.
