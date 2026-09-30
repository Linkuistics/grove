# dispatch-records-k23 — brief

## Goal

Make later evidence attachable to a recorded run. Observations can be imported
against a run and exported, and unknowns stay explicit. Policy can read an
earlier run's immutable launch fields: this is the provenance substrate the
review policy consumes. The store, the run ID and the required pre-exec record
arrived with `handoff-records-k24` in static dispatch, because every run needs
its record.

## Done when

- The evidence classes stay distinct: proposal, handoff attempt, observable
  launch failure, execution confirmation and outcome observation. An attempt is
  never a success. Committed launch fields never change, and no import alters
  them.
- `record show --run R --json` exports a run and its observations. `record
  observe --run R --file F` validates and atomically appends a version-1
  observation, with idempotent repeats, refused conflicts and retained
  supersession. Every measurement is `observed`, `unknown` or `unobserved`, and
  absence never reads as zero.
- `host.run(runId)` is a read-only protocol request to the Rust store. It
  returns the immutable launch fields and any launch-failure detail, or reports
  the run missing. An unreadable store refuses and never reads as missing. The
  worker never opens the database. A run whose selection used a creator
  reference records the creator provenance it used.
- The per-target installed smoke still passes on every target.

## Decomposition

1. `run-observations-k25`: `record observe`, the observation envelope and
   measurement states, and the export view.
2. `run-lookup-k26`: `host.run`, the loaded creator snapshot, and recording the
   creator provenance a selection used.

## Pointers

- Spec sections: `#records-and-outcomes`, `#identity-and-creator` and the
  records row of `#test-seams`.
- ADR: `docs/adr/a-review-carries-its-creator-reference.md`, which explains why
  there is no lookup by task or artifact identity. Do not add one.
- The store and its handoff commit are `handoff-records-k24`'s, in
  `static-dispatch-k12`. Extend its schema as that leaf's running log records;
  never rewrite a committed launch field. `run-observations-k25` took the store
  to schema 2, the observations table, which only `record observe` migrates
  to; its running log records the envelope, the vocabulary and that rule.

## Review

This node is expected to end with a `review-impl` naming this node's handle.
Retiring the node's last leaf closes it. That leaf cuts the review as the node's
sibling, directly after it and ahead of the next increment. Inside the node, a
review would keep the node open, so no session would finish the producer it
reviews (spec `#identity-and-creator`). Immutability under import, idempotency
and conflict handling, and the difference between missing and unreadable are
properties the compiler cannot check. The review policy's safety rests on them.
The durability of the handoff commit itself is reviewed with static dispatch.
