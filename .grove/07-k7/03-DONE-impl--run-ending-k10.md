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

## Decisions (running log)

**E1 — the end observation is built, then validated as an import is.**
`observation::end_observation` assembles the document and passes it through the
same `Check::envelope` an import passes, so a document dispatch writes is one
`record show` reads back (a unit test runs it through `stored`). Its ID is
`harness-dispatch-end-<run id>`, so there is one per run and a repeat is the
idempotent kind. `observedAt` comes from a small civil-date conversion over
`SystemTime` rather than a new dependency; the store's own `recordedAt` stays
SQLite's clock. A signal with no conventional `SIG` name has no valid `exit`
value, so it is reported as `unknown`, never as a made-up name.

**E2 — the ending file does not wait on the store.** It holds the same
document and is written once the group is confirmed gone, whether or not the
append succeeded: the file is the caller's channel, and a locked or unwritable
store must not take it away. Its path is checked before selection (exit 2,
`ending_file_unusable`): a name, an existing directory, nothing at the path
(`symlink_metadata`, so a dangling link refuses too). It is named absolutely
from the canonical directory, and creation is `create_new` with mode 0600, so a
path that appeared in the meantime is refused by the system, not by the check.

**E3 — the end notice says whether the observation was recorded.** `recorded`
joins the JSON notice and the text line, as the spec's *The run ending* says.
A supervision failure (group still present) appends the observation and
announces, writes no file, and exits 5, as before.

**E4 — a supervised run is no longer an unconfirmed attempt, so the premise of
every test that counted a run's observations changed.** `record show` after any supervised run now reads
`execution_confirmed` with dispatch's observation on it. The observation suite
is about what an *outside* observer imports and counts and orders what it
imports, so its `launched` helper takes dispatch's observation back out (the
one route the store's triggers leave, dropping and restoring the removal
trigger, test-only); dispatch's own observation has its cases in
`tests/supervision.rs`. `handoff.rs`, `records.rs` and Grove's `loop_driver.rs` assert the confirmed
state, and the review-findings case there counts dispatch's observation first. The version-1 migration test now has the new run's end observation
be the first append.

**E5 — what stays for `current-state-docs`.** `crates/harness-dispatch/README.md`
still describes `run` as replacing itself with the harness, and its *Run
records* section says nothing is recorded after exec: both predate
`dispatch-supervises` and are that leaf's. Only the observation table gains the
`ending` row, because it states what an import accepts. No walkthrough book
covers `harness-dispatch`, and `keyed-launch` and `grove-loop` are unchanged
here.
