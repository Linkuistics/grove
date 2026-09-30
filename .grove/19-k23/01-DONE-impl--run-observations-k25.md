# run-observations-k25

## Goal

Let any observer attach later evidence to a run and read it back. That covers
execution confirmation, exit, duration, usage, acceptance, findings, repair and
human work. Unknown and unobserved values stay explicit, and the launch fields
stay immutable.

## Context

The contract is the envelope and the measurement rules in the spec's
`#records-and-outcomes`. The package validates shape and association, not
external truth. Analytics, scoring aggregation and any learned-selector
benchmark are out of scope.

## Done when

- `record observe --run R --file F` validates a version-1 envelope. The
  envelope holds `schemaVersion`, `observationId`, `runId`, `source`,
  `observedAt`, `evidence`, optional `supersedes` and `measurements`, and
  validation refuses unknown versions and fields with their location. The
  observation is appended atomically.
- Each measurement has `state` equal to `observed`, `unknown` or `unobserved`.
  `observed` requires a typed value, and quantities require a unit. The other
  states carry no numeric value.
- The supported fields cover execution confirmation, exit or signal, duration,
  input, output and total usage with units, acceptance, missed defects, false
  findings, downstream repair, human-work measures and evidence links.
  Findings take stable IDs and may reference later repair observations.
- Routing probability fields keep `choiceProbability` distinct from
  `successProbability`. A success estimate must name its calibration data or
  version, or be labelled uncalibrated.
- A repeated identical ID and content is idempotent, and a conflicting repeat
  refuses. A correction names what it supersedes, and both are retained. No
  import changes launch fields.
- `record show` presents unsupplied fields as unobserved. A run with an
  execution confirmation is distinguishable from an attempt alone.
- Command-seam tests round-trip observe and show. They cover idempotency,
  conflict, supersession, unit and state errors, and observations after the
  task tree that named the run has been deleted.
- Help and the usage documentation carry an observation example.

## Decisions (running log)

**The envelope's own fields are strings, checked for shape only.**
`observationId` is caller-generated, nonblank and at most 1024 bytes, like a
task ID, and unique in the store rather than per run: a correction names it
without naming its run. `runId` must equal `--run`, so a file cannot be
imported against a run it does not name. `source` and `evidence` are nonblank
descriptions, the importer's assertion. `observedAt` is an RFC 3339 date-time
with an uppercase `T` and `Z` or a numeric offset, range-checked (leap years,
second 60) and kept exactly as given. The file is read once, as data, from a
regular file resolved against the cwd, within a fixed 1 MiB bound. There is no
stdin form.

**The measurement vocabulary is closed, and each field has one value type.**
`executionConfirmation` (the literal `true`: the observation *is* the
confirmation; `unknown` says the observer could not tell, and a harness that
never ran is dispatch's own launch-failure record), `exit` (`{code: 0..255}` or
`{signal: "SIG…"}`), `duration`, `inputUsage`, `outputUsage`, `totalUsage`,
`acceptance` (`accepted` | `rejected`), `missedDefects` and `falseFindings`
(arrays of findings `{id, summary?, repairs?}`, IDs unique within the array,
`repairs` naming observation IDs that need not exist yet, since the repair is
later), `downstreamRepair` (repairs `{id, summary?, findings?, runId?}`),
`humanTime`, `humanInterventions` (a count), `evidenceLinks` (nonblank
strings), `choiceProbability` (a number in [0, 1]) and `successProbability`
(`{probability, calibration}` naming its data or version, or
`{probability, uncalibrated: true}`, exactly one). k24's single `humanWork`
became `humanTime` and `humanInterventions`: the research measures elapsed
human time and interventions separately, and one field could not carry both
under the one-value-type rule.

**Quantities are the fields with a unit.** Time quantities (`duration`,
`humanTime`) take `ms`, `s`, `min` or `h`, a closed set because time units are
universal. Usage takes any nonblank unit, because providers meter differently
(tokens, credits, USD, requests), and analytics that would normalise them are
out of scope. A quantity is a non-negative JSON number. A count or any other
value type refuses a `unit`. `unknown` and `unobserved` carry neither `value`
nor `unit`.

**Identity and conflict are decided on the parsed document.** The stored
document is the validated envelope's compact serde_json encoding, whose maps
are sorted, so whitespace and key order do not make a repeat conflict. The
same ID with the same document is idempotent (exit 0, reported as already
recorded, nothing written). Any difference, including a different run, refuses
as `observation_conflict` with exit 3. Numbers compare as parsed, so `1` and
`1.0` differ.

**A correction replaces a whole observation, and corrections form a chain.**
`supersedes` must name an observation of the same run that no other
observation already supersedes. So there is one current observation per chain,
and a field the correction omits becomes unobserved from that chain. Both are
retained, and the export marks the replaced one `supersededBy`. The
idempotency check comes first, so repeating a correction still succeeds after
it has taken effect. A column `UNIQUE` on `supersedes` backs the check.

**An execution confirmation against a run with a launch failure refuses.** The
launch failure is dispatch's own record that exec never happened, not external
truth the package declines to judge. The refusal is
`observation_contradicts_record`, exit 3.

**The export derives evidence, and never aggregates values.** `evidence` is
`launch_failure` (dispatch's own detail wins), else `execution_confirmed` when
a current observation confirms execution, else `handoff_attempt`; `execution`
is `not_executed`, `confirmed` or `unknown` to match. Each exported observation
carries every supported field, the unsupplied ones as `{"state":
"unobserved"}`, with `recordedAt` and `supersededBy`. The run-level
`measurements` (k24's `outcomes`, renamed because probabilities are not
outcomes) gives every field a `state` and the `current` entries that supplied
it. The state is `observed` if any current observation observed it, else
`unknown` if one reported that, else `unobserved`. Several current values are
listed side by side, never merged: aggregation is out of scope.

**Schema 2 adds only the observations table, and only `record observe`
migrates.** `observations(observation_id PRIMARY KEY, run_id REFERENCES runs,
recorded_at, supersedes UNIQUE REFERENCES observations, document)`, with an
index on `run_id` and triggers that abort any update or delete. A pristine
store is created at version 2. `record observe` migrates a version-1 store
inside its own exclusive transaction by creating the table, so a refused
import rolls the migration back with it. `run`, the launch-failure append and
`record show` read and write both versions without migrating. A store of any
other version still refuses with exit 4. Observing never creates a store: a
missing or pristine one holds no run to observe.

**Refusals.** Stage `observation` (new) for the file and its shape:
`observation_unreadable`, `observation_too_large` (naming the fixed bound),
`observation_invalid` with its location, and `unsupported_version`, all exit 3.
Stage `record` for what the store decides: `run_not_found`,
`observation_conflict`, `supersedes_unknown` (whose message says when the ID
belongs to another run), `already_superseded` and
`observation_contradicts_record`, exit 3. Store failures keep their exit-4
codes.

**The export's field names settled with the first release still ahead.**
`outcomes` became `measurements`, each field now `{state, current}`; the
installed smoke reads only `evidence`, `execution` and launch fields, so it
needs no change. No worker, installed layout or native dependency changed, so
the per-target smoke was not rerun here. The node's `Done when` asks for it,
and `run-lookup-k26`, which closes the node, changes the worker.

**No in-session reviewer.** The node ends with a scheduled `review-impl`, and
its brief names immutability under import, idempotency and conflict handling
among the doubts it owns.

**Controls seen to fire.** Each was a mutation applied to one source, then
reverted, with every source's digest compared afterwards. Every one failed the
tests named:

- Idempotency removed (an existing ID always conflicts): the repeat,
  correction and round-trip tests.
- Conflict ignored (an existing ID always repeats): the repeat test.
- The chain check removed: the correction test (the `UNIQUE` column then
  refused as a commit failure, exit 4, not `already_superseded`).
- The same-run check removed: the correction test.
- The contradiction check removed: the launch-failure test.
- `record observe` not migrating: the version-1 test. `run` migrating as well:
  the version-1 test.
- Superseded observations counted as current: the correction test.
- Unsupplied measurements defaulted to `unknown`: the unsupplied test.
- Confirmation ignored by the evidence: the round-trip and unsupplied tests.
- A quantity's unit made optional; `unknown` allowed a value; a mismatched
  `runId` accepted: the unit-and-state test.
- The observations update trigger dropped; a `launch` field accepted: the
  launch-fields test.
- The size bound off by one: the file test, on its at-the-bound control.
- The raw file stored instead of its parsed encoding: the repeat test, on its
  reordered-keys repeat.
- The date check, the duplicate finding ID check and the probability range
  each switched off: the unit test of malformed observations.

**Done-when instruments.** Command-seam tests are in
`crates/harness-dispatch/tests/observations.rs`, unit tests in
`src/observation.rs`.

- Envelope validation, unknown versions and fields with their location, and
  the atomic append: `unit_and_state_errors_refuse_at_their_location_and_record_nothing`,
  `the_observation_file_must_be_a_readable_bounded_json_file`, and the unit
  tests `each_malformed_observation_is_refused_at_its_location`,
  `a_later_version_is_unsupported_and_a_launch_field_says_why` and
  `only_real_rfc_3339_date_times_are_taken`. The append is one exclusive
  transaction; a refused import leaves nothing, which the version-1 test
  shows by its rolled-back migration.
- Measurement states, typed values and units: the same tests, and
  `every_supported_measurement_in_every_state_is_accepted`.
- The supported fields, finding IDs and repair references, and the probability
  fields: `an_observation_round_trips_through_observe_and_show` imports every
  field; the unit tests refuse a success estimate without calibration.
- Idempotency, conflict, supersession, and no import changing launch fields:
  `a_repeat_is_idempotent_and_a_conflicting_repeat_refuses`,
  `a_correction_supersedes_its_observation_and_both_are_kept`,
  `no_import_changes_the_launch_fields` and
  `a_confirmation_contradicting_a_recorded_launch_failure_refuses`.
- Unobserved presentation, and confirmation distinct from an attempt:
  `unsupplied_measurements_stay_unobserved_and_an_attempt_stays_unconfirmed`,
  and `records.rs`'s export test for a run with no observations.
- After the task tree is deleted:
  `observations_survive_the_deletion_of_the_task_tree_that_named_the_run`.
  Unknown runs and missing stores:
  `an_unknown_run_or_a_missing_store_refuses_and_creates_nothing`. Schema 2 and
  migration: `a_version_1_store_is_migrated_by_its_first_observation_and_read_as_it_is_otherwise`.
- Help and usage: `help_carries_an_observation_example` parses the example in
  `record observe --help` and imports it. The README's example was imported by
  hand against a real run. The spec's notice, records section and exit codes,
  and the README's observations section and refusal table, state the contract.

**No ADR.** The envelope is versioned and the vocabulary can grow at a new
version, so nothing here is hard to reverse. The contract is in the spec's
*Records and later observations*, the reasons in this log and the module
comments.

**Checks.** `task check` passed all 12 principal checks, with 1642 tests and
none failing; the 3 ignored are `grove-loop`'s opt-in native smokes. Every
changed file outside `.grove/` had the same digest before and after the run.
