# paired-witness-owner-k32


## Goal
Have real launches own the directory and fresh private witness in one lease-owned
value, independently verifiable before mandate publication.



## Context

## Done when
- Both locks are exclusive, nonblocking and close-on-exec; neither is acquired
  before exclusive predecessor epoch invalidation and the second root check.
- Fresh private files use independent OS-random 128-bit suffixes, exclusive
  creation and eight bounded collision attempts. Foreign holders and reported
  allocation/lock faults release partial setup without preventing admission.
- Reaped, failed spawn and lease drop release private before directory;
  unconfirmed reap and epoch invalidation alone retain both. Cleanup follows
  invalidation and skips a still-owned launch.
- Native lock/event tests cover ownership, release, rollback, collisions,
  replacement ordering and actual configured successful/failed launches.
- Active observation remains Unavailable. Affected documentation/books and
  scripts/check.sh pass. Marker/extension publication belongs to witnessed-epoch-k33.

## Notes

## Decisions (running log)

Keep the selected pin in a lease-owned pair even when witness setup fails;
release its observation lock and any private descriptor on that failure. This
preserves the existing stale-root refusal and supervision ownership seam.

Use a private witness module for allocation, rollback, release and cleanup;
the lease retains epoch ordering. The eight-draw bound matches keyed-launch,
with a fresh random draw independent of both lease nonce and signal channel.
Witness cleanup recognizes only its basename grammar and excludes the still-owned
launch even if an error path invalidates its epoch without a confirmed reap.

The focused fresh-context review found no production issues in acquisition,
ownership or cleanup. Native lock controls and real configured launch tests
cover this child's boundary. The tests do not claim Linux, observer inference,
Started publication or the complete writer matrix; those remain explicit in k33
and the enclosing observation leaf.
