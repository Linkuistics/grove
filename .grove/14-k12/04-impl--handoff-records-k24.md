# handoff-records-k24

## Goal

Before `run` execs, commit a durable handoff attempt with a fresh run ID, and
export that identity to the harness. A failed commit launches nothing.
`record show` exports what was recorded. From this leaf on, the required record
is a precondition of every `run`.

## Context

The spec's `#records-and-outcomes` lists the launch fields and the evidence
classes. This leaf belongs to `static-dispatch-k12`, not to
`dispatch-records-k23`. The record is a precondition of every run, so no
increment boundary may expose `run` without it (review
`harness-selection-and-execution-k42`, finding F3).

Some launch fields come from later increments. The reviewed-artifact
association and the context digests arrive with `bounded-context-k22`. The
adapter version arrives with `grove-review-adapter-k37`, and the creator
provenance used with `run-lookup-k26`. Until then these fields are recorded as
absent. Choose a store schema and version that lets those leaves fill them in
without rewriting any committed launch field. Record the choice in this leaf's
running log. The commit must also leave room for
`signal-transparent-handoff-k29`'s not-executed append.

Bundled SQLite brings C into the Linux cross-build, which is zigbuild at
glibc 2.17. `dispatch-delivery-k16`, the next increment, builds it and smokes
it on every target. A clean host build is not evidence of the cross-build.

## Done when

- The store lives in the front process. Its default directory is
  `~/.local/state/harness-dispatch`, and `--state-dir` replaces it. It is
  created on first use with private permissions and a schema version. It uses
  durable, short transactions and a fixed 2-second lock wait that neither
  extends nor consumes the selection bound. No lock is held across evaluation.
  A readable but corrupt or unknown-version store refuses; it never becomes an
  empty default.
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
- `task check` builds bundled SQLite on the host. Its cross-target build, its
  license notice and its per-target smoke belong to `dispatch-delivery-k16`.
  The spec's notice states that `run` is delivered with its required record.
