# witness-replacement-observers-k39 — brief


## Goal
Challenge epoch-before-preparation and concurrent observation through the real
lease/observer seams, completing witness-interleavings-k36.



## Done when
- While an old epoch reader delays replacement, preserve old lease bytes and
  prove neither new witness can be acquired before exclusive invalidation.
- Independently disable epoch-before-preparation and expose the replacement's
  false attachment under the old epoch. Keep the real Running positive passing;
  restore production and rerun with exact source/run provenance.
- Concurrent observers of unlocked Started bytes cannot manufacture contention
  or Running. Continuous viewers across repeated launches never cause witness
  preparation failure. Use readiness/lock events and bounded failure timeouts.
- A separate foreign shared-holder process for each witness permits shared
  observation and promptly defeats exclusive preparation without blocking launch.
- Expand concurrent read-only snapshots across aliases, absent trees, non-jj,
  missing namespaces and multiple viewers. Preserve bounded handoff/recovery and
  no escaped guards, update affected books/docs and pass principal checks.

## Context

Consume witness-release-orders-k38 and the inherited k36/node contracts. Its
release controls and directory mutation are already independently owned; this
leaf owns the other mutation and all remaining concurrency obligations. k37
owns native macOS/Linux process evidence. Reconcile the whole k36 brief before
closing it; no in-session reviewer alongside k27's scheduled complete review.

## Notes

## Decomposition

The remaining obligations use three different controls. Each child owns its
tests, exact-source book repair and principal gate. The original Done when
remains the node's closure contract.

- witness-concurrent-viewers-k40 covers compatible shared probes of released
  Started bytes, continuous public observation across repeated real launches,
  and concurrent read-only snapshots across aliases and missing-tree/control
  cases. It preserves the existing bounded-handoff and escaped-guard controls.
- witness-epoch-replacement-k41 covers delayed replacement retaining old lease
  bytes, no preparation before exclusive invalidation, and the independent
  epoch-before-preparation mutation with real Running positive and exact
  restoration/run provenance. It combines replacement with concurrent viewers.
- witness-foreign-holders-k42 covers separate shared-holder processes for both
  witnesses, prompt preparation failure with successful launch/admission, and
  reconciles this node and k36 before closure. Native platform evidence stays
  k37; complete protocol review stays k27.

## Decisions (running log)

Split at independently executable observer, replacement-ordering and foreign
process controls. The existing WitnessIo and lease acquisition/preparation
seams support these without a second status provider. Implement only the
concurrent-viewer child in this session.
