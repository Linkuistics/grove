# witness-interleavings-k36


## Goal
Challenge witnessed observation with deterministic release/replacement controls
and concurrent observers through the real lease/observer seams.



## Context
Consume witnessed-binding-k35. Follow the node brief's complete interleaving
and concurrency obligations; internal filesystem/lock seams may be replaced,
the public activity provider may not.

## Done when
- Force directory-first release with numeric/key reuse, private-first release,
  both released and release between directory/private probes.
- Disable directory verification and independently epoch-before-preparation;
  each exposes its specified false attachment while real Running positives
  still pass. Restore production and rerun with exact source/run provenance.
- Concurrent viewers of unlocked Started bytes cannot create contention;
  replacement preserves old lease bytes without preparing under an old epoch.
  Old-epoch-delayed handoff, repeated launches under continuous viewing and a
  separate foreign shared-holder process are covered with readiness events.
- Preserve bounded handoff/recovery, no escaped guards and read-only snapshots
  across aliases/absent/non-jj/missing-namespace/multi-viewer cases. Update books
  and documentation, and pass focused tests plus the principal gate.

## Notes
These controls simulate interleavings, never host inode reuse or paused kernel
teardown. k37 runs the native process suite; keep all node exit conditions live.
