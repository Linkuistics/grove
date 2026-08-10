# quiescent-index-filter-child-k148

**Kind:** impl

## Goal

Guarantee that `remove_grove_entries` has reaped its Git child before returning,
so later cleanup cannot race a delayed index publication.

## Context

- `git update-index --stdin` owns an index lock and may replace the success-index
  artifact when it exits.
- The current `stdin.take()?.write_all(...)?` path returns immediately on a
  setup or write error, leaving the spawned child live while callers begin
  auxiliary cleanup.
- This is the first prerequisite slice of `atomic-colocated-index-rebind-k147`;
  it does not introduce the replacement-marker protocol.

## Done when

- Every post-spawn result path waits for the Git child and returns only after it
  is quiescent.
- The primary stdin error remains visible; a wait failure is attached rather
  than silently replacing it.
- A deterministic test proves a simulated stdin failure reaps the child before
  the error is returned.

## Notes

Keep process lifecycle ownership inside the index-filter helper. Do not make
auxiliary cleanup responsible for killing a repository child it did not spawn.
