# witness-epoch-replacement-k41


## Goal
Disprove early witness preparation while old epoch readers delay replacement.



## Context
Consume k40 concurrency controls, k38 forced-reuse release fixture and the
existing DriverLease acquire_with/prepare_launch_using seams. The required
mutation is independent of k38's directory-probe mutation.

## Done when
- A readiness/lock barrier holds an old epoch reader while replacement owns
  the lease. Old lease bytes remain intact and neither new witness is prepared
  before exclusive invalidation. Concurrent observers of released Started
  bytes remain Idle and cannot create contention.
- Model directory-first release with reused numeric identity/key as necessary
  to expose a new directory witness under the old epoch. Independently move
  preparation before epoch acquisition/invalidation in production code and
  observe the specified false attachment. The real Running positive still
  passes. Restore and rerun, preserving per-file source digests and per-case
  commands/results before and after each run.
- Preserve bounded handoff/recovery, no returned locks and read-only snapshots;
  update affected books/docs and pass focused tests and the principal gate.

## Notes
No sleeps as ordering proof, no fake public provider, no assertion of host
inode reuse. k42 reconciles k39/k36; k37 owns native platform evidence and k27
owns complete review. No competing in-session reviewer.
