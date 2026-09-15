# witness-foreign-holders-k42


## Goal
Prove observation-only preparation failure against foreign shared processes.



## Context
Consume k40/k41. LaunchWitnesses::prepare_with already has an after-create
barrier for a foreign holder of the new private file. Existing same-process
holder checks do not discharge this leaf's independent process requirement.

## Done when
- Separate shared-holder processes cover directory and private witnesses.
  Readiness proves their lock acquisition; an independent shared probe succeeds
  while exclusive preparation fails promptly. Real launch and admission still
  succeed with activity Unavailable. Reap and verify release/rollback.
- Preserve the containing-directory mutation lock, existing reported lock-error
  controls, bounded handoff/recovery and read-only/no-escaped-guard controls.
- Update affected books/docs and pass focused tests plus the principal gate.
- Reconcile all of k39 and k36, promote useful context and close only if their
  complete contracts hold. Name any remaining gap as a leaf; k37's native
  macOS/Linux evidence and k27's complete review remain separately live.

## Notes
Use bounded readiness/reap events, never elapsed sleeps as proof. Keep required
process tests named with witness_ for k37's cross-platform suite. No in-session
reviewer beside the scheduled complete protocol review.
