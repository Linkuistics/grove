# run-ending-k10

## Goal

Dispatch records and reports how each run ended (D7, W5):

- **The end observation.** Once the harness is reaped, dispatch appends an
  ordinary version-1 observation in its own short transaction, under the
  commit's lock wait. Its `source` is `harness-dispatch`, its ID names the run,
  and its evidence says dispatch supervised the harness to its end. It
  observes `executionConfirmation`, `ending`, `exit` and `duration`, the last
  measured from the harness's start to its reap.
- **Migration.** A version-1 store is migrated by the first observation
  appended to it, this one included, inside that append's own transaction. A
  failed append leaves the store at version 1.
- **The `ending` measurement** (`exit_signal`, `harness_exit` or `cancelled`)
  joins the observation schema, so imports, corrections and `record show`
  handle it like any other measurement.
- **`--ending-file PATH`.** The path must not exist and its directory must;
  both are checked before selection. Dispatch creates the file exclusively and
  owner-only, holding the same observation, once the harness is reaped and its
  group is confirmed gone.
- **Supervision failure.** It appends the observation as observed and writes no
  ending file.
- **Failures change nothing.** A failed append or ending file is reported on
  stderr and changes neither the ending nor the exit. A run that started no
  harness has no ending.

## Context

- The spec's *Records and later observations* (the end observation, the
  migration rule, the measurement table) and *Supervision* (the run ending, the
  ending file). `harness-wrapper-k5`'s I4 settled the migration.
- `append_observation` in `crates/harness-dispatch/src/store.rs` already
  migrates inside its own exclusive transaction. Observation validation lives
  in `crates/harness-dispatch/src/observation.rs`.
- Seam 2's suite is the record cases under `crates/harness-dispatch/tests/`.

## Done when

- Seam 2:
  - After a supervised run, `record show` carries dispatch's end observation
    and reads `execution_confirmed`. That holds against an existing version-1
    store as well, which the end observation migrates with no import between.
  - A dispatch killed while its harness runs leaves the attempt as it stood,
    with no end observation.
  - Outside imports still append to and correct any run, a supervised run's
    end observation included.
- Seam 1: each of the three endings appears in the ending file with the
  harness's exit and duration, and gives the exit status its row states. An
  existing ending file refuses before selection.
- `## Unreleased` records the end observation and the ending file.
- `bash scripts/check.sh` passes.

## Notes

- Grove does not read the ending file yet. `standalone-through-dispatch` and
  `launch-cutover`'s driver are its readers. Until then, dispatch's own records
  and any independent caller make it useful.
- This leaf's retirement closes `supervised-run-k7`, so it runs that node's
  close and names its run on the pre-cut review `supervised-run-k11`
  (`references/retire.md`).
