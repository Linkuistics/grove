# witness-owner-k29 — brief


## Goal
Complete the launch-witnesses-k25 writer contract using the selected-root pin
landed by selected-root-k28 and the launch-events-k24 runner notifications.



## Context
The parent brief retains the complete writer acceptance contract. Read its
Done when in full. Move the selected pin into DriverLease ownership with the
private witness; do not leave either resource on a helper stack after an
unconfirmed-reap error. Keep Selection descriptor-free.

## Done when
- All parent writer criteria hold, including epoch-before-either-witness,
  nonblocking foreign-holder failure, close-on-exec, random exclusive bounded
  allocation, optional extension independent of mandatory admission, exact
  Started marker, private-before-directory release and post-invalidation cleanup.
- Real configured launches and fault/event barriers cover failed/immediate spawn,
  signal-before-reap, confirmed/unconfirmed reap, unwind/drop, lock/write/allocation
  failures, collisions/exhaustion and old-reader replacement ordering.
- The production observer still returns Unavailable for active records. Existing
  admission, replacement and handoff controls pass. Native observer/platform and
  forced-reuse evidence stays in witnessed-observation-k26.
- Shipped descriptions and affected source-derived books match; focused tests and
  scripts/check.sh pass. The parent node closes only against its full contract.

## Notes

## Decomposition

Source inspection separated an independently verifiable ownership/handoff step
from witness allocation and publication. lease-root-owner-k30 transfers the
selected pin to DriverLease, checks it after epoch acquisition, and wires
parent-side events. witness-publication-k31 completes both witness locks,
optional record fields, marker, cleanup and the remaining fault matrix. This
node retains every original writer criterion; the first child publishes no
observation extension and cannot establish RUNNING.

The selected-root slice checks before calling activate_session_epoch. When
reworking publication, recheck the lease-owned pin after obtaining the exclusive
epoch guard as well: that acquisition can wait while the root changes. Preserve
the no-tree/epoch-guard-across-spawn rule. k28's tests cover replacement before
entering launch preparation, not replacement during epoch acquisition.
