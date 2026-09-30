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

## Decisions (running log)

**The store is `<state dir>/records.sqlite3`, and nothing but `--state-dir`
moves it.** The default directory is `$HOME/.local/state/harness-dispatch`. HOME
must be absolute, as for the personal policy, and `XDG_STATE_HOME` is not
consulted: the leaf fixes the default, and an environment-selected location is
what the authority module already declines for policy. `--state-dir` is joined
to the original cwd, kept lexical like `--task-file`, and exported to the
harness as that absolute path. The state directory is resolved after policy
authority and before the worker starts. A HOME problem then refuses with the
same exit in `inspect` and `run`, and does so without evaluating anything. With
no `--config`, the authority refusal still comes first.

**Durability: a rollback journal, `synchronous = EXTRA`, `fullfsync = ON`.** The
SQLite pragma documentation (https://www.sqlite.org/pragma.html#pragma_synchronous)
says FULL "is not necessarily durable across a power loss in rollback mode, so
if durability is desired, it is best to set the synchronous mode to EXTRA". It
also says `fullfsync` defaults off, and it is the only way to reach
`F_FULLFSYNC` on macOS. WAL, where FULL is durable, was rejected. It needs
shared memory that network filesystems do not provide, and home directories
often live on them. It is also persistent in the file. The journal mode is
therefore never set: a store this command creates keeps SQLite's default
DELETE mode.

**One lock wait of at most 2 seconds, after the worker is gone.** The
connection's busy timeout is 2000 ms, overriding rusqlite's own 5000 ms default
(rusqlite 0.40.2, `src/inner_connection.rs:118`). The commit's first statement
is `BEGIN EXCLUSIVE`. In rollback mode that takes the only lock the commit
needs, so there is one wait, and COMMIT waits for nothing further. The bundled
build defines `HAVE_USLEEP=1` (libsqlite3-sys 0.38.2 `build.rs`, SQLite
3.53.2), so SQLite's busy handler sleeps in milliseconds rather than whole
seconds. The store is opened only after the worker has been reaped and the
program resolved. No lock is held across evaluation, and the wait can neither
extend nor consume the selection bound, which has already been met.

**Version: `application_id` 0x48445253 and `user_version` 1, both checked
inside the exclusive transaction.** A pristine file, with both zero and no
schema objects, is initialized in that same transaction. A probe with the
system `sqlite3` showed that both header pragmas roll back with their
transaction, so two first uses cannot both initialize. Anything else refuses
with exit 4 and is never reset: another application's database, a newer or
older version, or a file SQLite reports as not a database or corrupt. A
missing table in a store that claims version 1 is a schema error, and it
refuses the same way.

**Schema 1: launch fields as one immutable JSON document per run.** The table
is `runs(run_id PRIMARY KEY, recorded_at, launch)`, and `launch` carries its
own `schemaVersion` of 1. Every launch field is present from this version on.
Fields that later increments supply are `null`: `reviewedArtifact` and
`context` (`bounded-context-k22`), `adapter` (`grove-review-adapter-k37`),
`creator` (`run-lookup-k26`), and `selection.explicitChoice`
(`choice-and-refusals-k15`). Each of those leaves fills its field for new runs
only. A committed row is never rewritten, and `BEFORE UPDATE` and
`BEFORE DELETE` triggers abort any attempt. If a later leaf needs a field that
schema 1 lacks, it writes a launch document of version 2 for new rows, and
readers take both. `launch_failures(run_id PRIMARY KEY REFERENCES runs,
recorded_at, detail)` holds at most one detail per attempt. The detail JSON
names its `cause`: `exec_error` here, and `not_executed` from
`signal-transparent-handoff-k29`. The same triggers guard it.
`run-observations-k25` adds its observation table in schema 2, by a migration
that only creates tables.

**Run ID: a version-4 UUID from `/dev/urandom`.** It is 122 random bits in the
canonical lowercase 8-4-4-4-12 form. Reading `/dev/urandom` works on macOS and
at the glibc 2.17 floor, where the `getrandom` wrapper does not exist (it
arrived in glibc 2.25). No crate is added for it. `record show --run` accepts
only the canonical shape and refuses anything else as malformed. It does not
normalize case.

**The timestamp is SQLite's own.** `recorded_at` is
`strftime('%Y-%m-%dT%H:%M:%fZ', 'now')`, UTC to the millisecond, evaluated in
the committing INSERT and returned by `RETURNING`. That avoids a hand-written
calendar.

**Selected catalog values are the chosen candidate's entry as configured.**
That entry is `id`, `provider`, `model`, `effort`, `program` and `args`, with
slots kept as slot objects. The spec's "catalog snapshot" in run lookup is
this entry, the configured launched choice. The whole catalog is not copied.

**The policy entry is digested by the front before the worker starts.** It is
SHA-256 over the canonical entry's bytes, and inspection reports it too, as
`policy.sha256`. It describes the file as it was read just before evaluation.
As the spec says, it does not claim to hash what the entry imports.

**Raw environment values are not stored.** The resolved executable is recorded
by its path and resolution kind. The PATH entry that matched is left out, even
though inspection shows it, because it is a piece of the caller's PATH value.
The original cwd and resolved paths are recorded as UTF-8, lossily when a path
is not UTF-8, the same way the worker request already carries the cwd.

**`inspect` never opens the store.** It reports the effective state directory
and a proposed run ID, marked as proposed in both forms. A `runId` slot's
argument is `{"proposedRunId": "…"}` in JSON and a marked word in text. A
preflight therefore cannot learn from inspection that the store would refuse.
Probing the store would mean opening it, and inspection writes nothing.

**Exit 4 is stage `record`.** Its codes are `record_store_unwritable`,
`record_store_locked`, `record_store_full`, `record_store_invalid` (another
application's file, an unknown version or corruption), `record_commit_failed`
(any other SQLite or I/O failure), `run_id_unavailable`, and `home_unset` when
HOME cannot place the default state directory. `record show` refuses an unknown
run with exit 3 (`run_not_found`). The message says whether the store file
exists at all, and a store that cannot be read exits 4.

**The connection is closed before exec, and an exec error opens a new one to
append.** The harness therefore inherits no store descriptor. A failed append
is reported on stderr beside the exec failure, whose exit stands. The run stays
a handoff attempt whose execution is unknown. The store has no success state
for it to become.

**The application ID literal was wrong, and the constants are now the only
source.** The first schema wrote the application ID as a decimal literal, and
the hand conversion was wrong: 1212436051 instead of 0x48445253, which is
1212437075. A second run in the same store therefore refused the store as
foreign. The existing `run.rs` tests caught it, because several of them run
twice in one sandbox. Initialization now writes both header pragmas with
`format!` from `APPLICATION_ID` and `SCHEMA_VERSION`.

**The pre-create states `truncate(false)`.** Clippy refused `create(true)`
without an explicit truncate behavior. Truncating would silently empty an
existing store into a pristine one. The truncate mutation below shows the tests
would catch that.

**`record show` exports `evidence`, `execution`, `launchFailure`,
`observations` and `outcomes`.** `evidence` is `handoff_attempt`, with
`execution` `unknown`, or `launch_failure`, with `not_executed`. The run's ID
and commit time sit beside the launch document, not inside it. `outcomes`
lists the spec's measurements, each `{"state": "unobserved"}`:
`executionConfirmation`, `exit`, `duration`, the three usage fields,
`acceptance`, `missedDefects`, `falseFindings`, `downstreamRepair` and
`humanWork`. `run-observations-k25` owns the envelope, and it may rename these
before the grove's single release. `record observe` stays refused by name.
`record show` opens the store read-write without create. So a missing file is
reported as no store, and a store left mid-commit is rolled back rather than
refused.

**An exec failure carries a `run` note.** The exec failure keeps its own code
and exit. Its error gains `run: {id, launchFailure: "recorded"}`, or
`"unrecorded"` with the append's code and message, and in that case the
evidence stays `handoff_attempt` with execution unknown. Text mode adds a
`run:` line.

**The worker's `runId` comment changed, so the worker digest changed.** The
SDK's `Slot` documentation no longer says `runId` is refused. The worker was
rebuilt with `task dispatch:worker`. The worker code is otherwise unchanged.

**MSRV still holds.** `rustup run 1.85 cargo check --locked --all-targets`
passes with rusqlite 0.40.2, libsqlite3-sys 0.38.2 and cc 1.5.1. The new crates
in the lock are cc, fallible-iterator, fallible-streaming-iterator,
find-msvc-tools, libsqlite3-sys, pkg-config, rusqlite, shlex and vcpkg. sha2
was already there.

**No in-session reviewer.** The node's scheduled `review-impl` names the
durability of the pre-exec commit among its doubts, so this producer's review
is already scheduled.

**A recurring deadline-test race was externalized, not fixed here.** The first
`task check` failed one test outside this leaf:
`a_worker_that_never_identifies_itself_is_stopped_at_the_deadline`. Its fake
worker had not written its PID file within the 3000 ms bound, with a VM running
on the host. That is the race `selection-deadline-k44` recorded. Three isolated
reruns of `--test worker` passed, and so did a full `cargo test --workspace`
(1535 tests). Nothing in this leaf touches worker spawn. The repair does not
serve this leaf's goal, so it went to the tree as `deadline-test-load-race-k45`,
inserted ahead of `choice-and-refusals-k15` so that k15 still closes the node.
The node brief lists it.

**Controls seen to fire.** Each was a mutation run against `tests/records.rs`,
then reverted, with the four sources' digests compared afterwards:

- The commit skipped: 14 of the 16 tests failed.
- A commit failure ignored, launching anyway: the unwritable, locked,
  cannot-grow and corrupt-store tests failed.
- A store lock held across evaluation: `no_lock_is_held_while_the_policy_evaluates`
  failed, with the tests that meet that lock.
- The busy timeout left at rusqlite's 5000 ms: only the lock-wait test failed,
  on its bound measured from the end of selection. Measured from spawn, the
  first version passed this mutation, so it was tightened before the run.
- The launch-failure append removed: the exec-failure, append and
  immutability tests failed.
- The two variables not exported: the identity tests failed.
- The `runs` update trigger neutered: the immutability test failed.
- Any application ID accepted: the corrupt, foreign and newer store test failed.
- `inspect` committing a record: the inspection test failed.
- `truncate(true)`: six tests failed, among them the two-runs count.
- Mode 0644: the first-use permissions test failed.
- The environment added to the launch document: the export test failed on the
  secret.
- The state directory created before selection: the pre-commit refusal test
  failed.
- The state directory resolved after evaluation: the HOME test failed on its
  policy sentinel.

The fake harness's descriptor probe is controlled by the existing
`the_harness_keeps_the_callers_stdin_stdout_and_other_descriptors`, which now
asserts that the probe sees exactly the caller's descriptor 7.

**Done-when instruments.** The command-seam tests are in
`crates/harness-dispatch/tests/records.rs` unless another file is named.

- The store's location, private creation, schema and versions: the first-use
  permissions test, `a_state_dir_replaces_the_default_and_resolves_against_the_cwd`
  and `without_a_home_the_default_state_directory_refuses_before_any_policy_runs`.
  Corrupt, foreign and newer stores refusing and staying unchanged:
  `a_store_that_is_corrupt_foreign_or_newer_refuses_and_is_left_as_it_was`,
  whose positive control is an empty file that initializes. Durability is the
  pragma settings. SQLite's documentation, cited in `store.rs`, is the evidence
  for them. No test can observe a power loss.
- The 2-second lock wait, and the selection bound it leaves alone:
  `a_lock_held_past_the_wait_launches_nothing_and_spends_no_selection_time`
  (exit 4 under a 1000 ms selection bound, waited 1.95 s to 4.5 s after
  selection). No lock across evaluation:
  `no_lock_is_held_while_the_policy_evaluates`.
- One commit before exec, with the launch fields:
  `a_run_commits_its_handoff_record_before_the_harness_starts`, in which the
  harness's own copy of the store holds its run, and
  `record_show_exports_the_launch_fields_with_the_attempt_unknown_and_every_outcome_unobserved`.
  The latter checks every field, the policy digest against the file,
  later-increment fields as `null`, and a distinctive environment value absent
  from the store's bytes.
- Commit failures, each exit 4 with no harness, each beside its positive control:
  `an_unwritable_record_directory_launches_nothing`, the lock-wait test, and
  `a_store_that_cannot_grow_launches_nothing`. The last uses `RLIMIT_FSIZE`
  with `SIGXFSZ` ignored, so writes fail with `EFBIG` as a full disk fails them
  with `ENOSPC`, on first use and on a later commit.
- The exported identity, inherited values replaced, and the `runId` slot:
  `the_harness_receives_its_run_identity_in_place_of_inherited_values` and
  `every_run_is_a_new_run_with_its_own_identity`. The marked proposal, with
  nothing written: `inspect_proposes_a_marked_run_id_and_writes_nothing`.
- The exec-failure append: `an_exec_failure_is_appended_to_its_attempt`. A
  failed append leaving the attempt unknown:
  `a_failed_append_leaves_the_attempt_unknown`, where a fixture trigger refuses
  exactly the append. No run from a pre-commit refusal:
  `a_refusal_before_the_commit_creates_no_run`. Immutability:
  `committed_launch_fields_never_change`.
- `record show`: the export test, and
  `record_show_refuses_a_malformed_id_an_unknown_run_and_a_missing_store`.
  Unit tests in `run_id.rs` cover the ID's form and parsing.
- Checks: `task check` passed all 12 principal checks, with 1535 tests and none
  failing. Every changed file outside `.grove/` had the same digest before and
  after that run. The HOME test was added after it, and `cargo fmt --all
  --check`, package clippy and all package tests passed again with it.
  Bundled SQLite 3.53.2 builds on the host. Its cross-build, notice and smoke
  are `dispatch-delivery-k16`'s.

**No ADR.** The store design fails the all-three test. The journal mode can be
switched in a later release, so it is not hard to reverse. A versioned launch
document with `null` placeholders is not surprising without context. The
durable contract is in the spec's *Records and later observations*, and the
reasons are in `store.rs`'s module comment. The node's brief needs nothing
promoted, because the node stays open.
