# witness-replacement-observers-k39


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
