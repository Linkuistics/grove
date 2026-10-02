# direct-dispatch-k4

## Goal

Cut the design that `direct-dispatch-k3` delivers into the smallest increments
that each land green and are useful on their own.

## Context

- The human's steer applies to the plan as much as to the code: do not
  overcomplicate, and do not *"let the perfect be the enemy of the good."*
  Prefer fewer, plainer leaves to a finer cut.
- Known now, before the design exists:
  - Requirement 11's parity needs what today's resolver produces for every kind,
    recorded before the resolver is deleted.
  - Grove's switch to dispatch and the deletion of its configuration are one
    wide change. Add the direct launch, move the tests onto it, then delete the
    machinery.
  - `scripts/check.sh` validates every walkthrough book to its last byte. A leaf
    that changes a crate with a book leaves that book valid, or cuts the leaf
    that will.
  - Methodology text that describes configuration ships in the same release as
    the behaviour it describes.

## Done when

- The tree holds the leaves that deliver the root brief's *Done when*, in an
  order each can land green.

## Notes
