# handoff-records-k24

## Goal

Before `run` execs, commit a durable handoff attempt with a fresh run ID, and
export that identity to the harness. A failed commit launches nothing.
`record show` exports what was recorded.

## Context

The spec's `#records-and-outcomes` lists the launch fields and the evidence
classes. Bundled SQLite brings C into the Linux cross-build, which is
zigbuild at glibc 2.17. Verify it through the delivery smoke task. A clean host
build is not evidence of it.

## Done when

- The store is created on first use with private permissions and a schema
  version. It uses durable, short transactions and a fixed 2-second lock wait
  that neither extends nor consumes the selection bound. A readable but corrupt
  or unknown-version store refuses; it never becomes an empty default.
- `run` commits one transaction before exec. It holds the run ID, timestamp,
  task identity, reviewed-artifact association, kind, selected catalog values,
  explicit choice, resolved executable and argv, and the original cwd. It also
  holds the policy entry's authority, digest and version, the worker and
  adapter versions, the context source digests and sizes, the effective
  limits, the reason and the selection timing. Raw environment values are
  never stored.
- Command-seam tests cover commit failures: an unwritable directory, a
  held lock past its wait, and a simulated disk-full or equivalent. Each exits 4,
  and the fake harness never starts.
- The harness receives `HARNESS_DISPATCH_RUN_ID` and `HARNESS_DISPATCH_STATE_DIR`,
  replacing inherited values, and the `runId` slot expands to the same ID.
  `inspect` shows a visibly marked proposed ID and writes nothing.
- When exec returns an error, an observable launch-failure detail is appended
  to the attempt. If that append fails, the attempt stays unknown and never
  becomes a success. A pre-commit refusal leaves the store without a run.
- `record show --run R --json` exports the run's launch fields and evidence
  class, with every outcome unobserved.
- The per-target installed smoke passes on all three targets with SQLite in the
  build. The archive's license notices include SQLite's.
