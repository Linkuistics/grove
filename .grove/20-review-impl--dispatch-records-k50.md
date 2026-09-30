# dispatch-records-k50

**Reviews:** dispatch-records-k23

## Goal

An adversarial, inspection-only read of the whole `dispatch-records-k23` node:
later observations against a recorded run (`run-observations-k25`), and policy
run lookup with the creator provenance a run records (`run-lookup-k26`).
Produce findings, not fixes.

## Context

The review policy's safety rests on properties the compiler cannot check, and
the node brief names them. The supplied review selector
(`review-selector-k36`) reads a creator's provider from what this node built.
A lookup that answers wrongly, or a store failure read as a missing run, would
read exactly like a working policy in every test that does not target it.
Correcting it here is cheaper than after the review policy builds on it.

## What to doubt

- **Immutability under import.** No `record observe` can change a committed
  launch field, a launch-failure detail or an earlier observation. Triggers
  abort updates and deletes. Is there a path around them, the version-1
  migration inside the import's own transaction included?
- **Idempotency and conflict handling.** A repeat is compared by the validated
  envelope's compact encoding with sorted keys. Can two documents that differ
  in meaning encode alike, or equal ones differ (`1` against `1.0`, key order,
  whitespace)? Does supersession stay a chain under concurrent imports?
- **Missing against unreadable.** A missing store file, an empty file and a
  store without the run read as missing, for `record show`, `record observe`
  and `host.run` alike. Anything else refuses with exit 4. Is there a failure
  that reads as missing? Consider a directory that cannot be searched, a lock
  held past the wait, a crashed commit's journal, a launch document of another
  version, and a store another application wrote.
- **Run lookup's authority.** The front answers every `host.run` from its own
  store, keeps its own answers, and requires the delivered context's `runs` and
  run sources to be those answers in order. The worker never opens the
  database. Can a loader, the policy's `select` or a late callback, or a worker
  bug, put a provider into `runs`, inspection's `creator` or the run record's
  `creator` that the store did not give?
- **The lookup inside the selection bound.** A lookup's lock wait is cut to the
  time left, and a wait that reaches the deadline is a timeout. Is the deadline
  kept on every path? Is the worker always stopped and reaped after a lookup
  refusal, including one whose policy holds `host.signal`?
- **The recorded creator provenance.** `creator` records what the delivered
  context carried: the reference, its evidence class, the provider, and the
  first lookup of that run. Is that what a review policy and an owner auditing
  it need, or can it mislead? Consider a routes policy that never read its
  creator, a repeated lookup, and a creator run that was never looked up.

## Pointers

- Spec: `docs/specs/harness-selection-and-execution.md`, sections
  `#records-and-outcomes`, `#bounded-context` and `#identity-and-creator`, and
  the records row of `#test-seams`.
- Decisions and the mutations seen to fire: the running logs of
  `run-observations-k25` and `run-lookup-k26`.
- Code: `crates/harness-dispatch/src/store.rs`, `record.rs`, `observation.rs`,
  `context.rs`, `worker.rs` and `choice.rs`, and `worker/src/host.ts`.
- Tests: `crates/harness-dispatch/tests/observations.rs`, `records.rs` and
  `lookup.rs`.

## Done when

- Every doubt above has been read against the code and the tests, and each
  finding names its file, its line and a failure scenario, or the doubt is
  recorded as examined and found sound.
- A review with findings worth acting on cuts its `integrate-review-impl` leaf
  where `pick` reaches it next.
