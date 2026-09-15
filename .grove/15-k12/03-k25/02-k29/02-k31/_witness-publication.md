# witness-publication-k31 — brief


## Goal
Complete the witness-owner-k29 writer contract on the lease-owned launch value
and event adapter introduced by lease-root-owner-k30.



## Context

## Done when
- Every original writer criterion in witness-owner-k29 and launch-witnesses-k25
  holds, including nonblocking exclusive locks on both witnesses only after
  predecessor epoch invalidation, independent random exclusive bounded allocation,
  optional admission-independent extension, exact Started marker and cleanup
  after invalidation. Extend the existing lease-owned value to own both members
  and close the private witness before its tree pin on every release path.
- Allocation, publication, lock, collision/exhaustion and mandatory-write faults;
  foreign holders; signal-before-reap; old-reader replacement; unwind/drop and
  confirmed/unconfirmed reap have event/lock and real-launch controls. Preserve
  the second root check under the exclusive epoch guard and guard-free spawn.
- Production try_observe remains Unavailable for active records. Existing
  admission/handoff/root-replacement tests pass, affected docs/books match and
  scripts/check.sh passes. Native observer/platform/forced-reuse evidence stays
  with witnessed-observation-k26. Close ancestors only against full contracts.

## Notes

## Decomposition

paired-witness-owner-k32 makes actual launches acquire and release the two
exclusive witnesses, including bounded allocation, failed-setup rollback and
post-invalidation cleanup. This is independently observable through native lock
probes while records remain legacy and activity remains Unavailable.
witnessed-epoch-k33 then binds selected identity into the optional epoch
extension and publishes Started, completing the original writer fault matrix.
All original Done when criteria remain here; this node cannot close on locks alone.
