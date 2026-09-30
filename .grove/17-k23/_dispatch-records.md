# dispatch-records-k23 — brief

## Goal

Make every `run` durably recorded before its handoff, and make later evidence
attachable to it. A committed handoff attempt carries a collision-resistant run
ID that the harness receives. Observations can be imported against a run and
exported, and unknowns stay explicit. Policy can read an earlier run's immutable
launch fields: this is the provenance substrate the review policy consumes.

## Done when

- A versioned local SQLite store lives in the front process. It uses bundled
  SQLite, private permissions, durable short transactions and a fixed 2-second
  lock wait, and holds no lock across evaluation. The default directory is
  `~/.local/state/harness-dispatch`, and `--state-dir` replaces it. Any store
  failure before handoff refuses with exit 4 and launches nothing.
- One committed transaction before exec persists every launch field the spec
  lists. Raw environment values are never stored. The harness receives
  `HARNESS_DISPATCH_RUN_ID` and `HARNESS_DISPATCH_STATE_DIR`, which replace
  inherited values, and the `runId` slot works. Inspection shows a visibly
  marked proposed ID and creates no run.
- The evidence classes stay distinct: proposal, handoff attempt, observable
  launch failure, execution confirmation and outcome observation. An attempt is
  never a success. A pre-commit refusal creates no run. Committed launch fields
  never change.
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
- The per-target installed smoke still passes on every target after bundled
  SQLite joins the cross-build.

## Decomposition

1. `handoff-records-k24`: the store, the run ID, the required pre-exec commit,
   exported run identity, the exec-failure detail and `record show`.
2. `run-observations-k25`: `record observe`, the observation envelope and
   measurement states, and the export view.
3. `run-lookup-k26`: `host.run`, the loaded creator snapshot, and recording the
   creator provenance a selection used.

## Pointers

- Spec sections: `#records-and-outcomes`, `#identity-and-creator` and the
  records row of `#test-seams`.
- ADR: `docs/adr/a-review-carries-its-creator-reference.md`, which explains why
  there is no lookup by task or artifact identity. Do not add one.
- Cancellation observed after the commit is
  `signal-transparent-handoff-k29`'s. The commit must leave room for that
  leaf's not-executed append.

## Review

This node is expected to end with a `review-impl` naming this node's handle.
Its last leaf cuts that review inside this node. Durability, immutability,
idempotency and conflict handling, and the difference between missing and
unreadable, are properties the compiler cannot check. The review policy's
safety rests on them.
