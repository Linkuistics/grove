# dispatch-records-k51

**Integrates:** dispatch-records-k50

## Goal

Triage the findings of `dispatch-records-k50` against the finished
`dispatch-records-k23` implementation and its contract. Apply the findings
that hold, record every disposition, and verify the integrated result.

## Context

Read the review's committed artifact by its stable handle; its findings are
not this task's charter and remain open to independent triage. The producer
changes are `637b8805` (`run-observations-k25`) and `e03558c0`
(`run-lookup-k26`, closing the node). The area contract is
`docs/specs/harness-selection-and-execution.md`.

Use the existing command seam and the repository Taskfile for verification.
Keep accepted creator-attestation costs and the later evaluation-boundary,
review-policy and creator-reference work in their existing owning leaves.

## Done when

- Every finding from `dispatch-records-k50` has a reasoned disposition in the
  running log, with valid findings addressed and rejected findings explained.
- Required post-fix verification passes, and the area specification and usage
  remain accurate for the resulting behavior.
- Any remaining substantial redesign or review work is externalized according
  to the integration skill; this task is retired and committed as one change.

## Decisions (running log)

**Boundary.** The review is `dispatch-records-k50` at `1daef4e6`; the producer
tip is `e03558c0`. This session began on an empty working change. Both findings
were read against the source at that tip, not taken from the review's summary.

**F1 is a real issue: fix the artifact.** `record::lookup` calls `store::load`,
which queries and JSON-parses every observation of the run
(`store.rs:478-500`), and the projection discards them. A creator lookup's work
therefore grows with an unbounded later history. The fix: `store::load` takes
whether to read observations, and a run lookup does not. Its read is then one
primary-key row of launch fields and one launch-failure row, bounded by the
launch document the commit wrote, whatever the run's later history.

**F1's "enforce the deadline during store work" is accepted as a bounded
trade-off, not implemented.** Once observations are out of the read, the work
after the lock wait is opening the file, the identity pragmas and one bounded
row. That row is bounded by the prompt and protocol-message bounds. The front
already checks the deadline when the lookup returns, so a late answer is still
`selection_timeout`. Interrupting SQLite mid-statement would need rusqlite's
`hooks` feature and a progress handler to shorten an overrun measured in
milliseconds. The spec says the lookup's read is bounded rather than claiming
the deadline interrupts it.

**F1's test is deterministic, not timed.** The review asked for a large history
under an elapsed bound. A timing test over hundreds of MiB would be slow and
flaky, and it would not show that the lookup never reads observations. Instead,
a run carries a real history, including a correction, plus one stored
observation row that is not JSON. The lookup's answer is unchanged. `record
show` refuses that same store, which shows the row is really unreadable. Before
the fix, the lookup refused on it, since `load` parsed every observation.

**F2 is a real issue: fix the artifact.** Verified: `store::document` only
parses; `record::exported` substitutes `{}` for a non-object; and
`observation::confirms_execution` ignores the value. `record show` returns an
unchecked load, and `record observe` checks only `SELECT 1`. So a future
launch-document version, an observation `{}` and an observed confirmation of
`false` were all read with today's vocabulary. The fix puts the check at the
store's read boundary, so no caller can forget it. Every read of a run, in
`load` and before `append_observation` inserts, requires two things. The launch
document must be a version-1 JSON object. A launch-failure detail must be an
object with a string `cause`. A stored observation is checked when `load` reads
it, by the import's own validator, against its row's run, ID and `supersedes`.
Each failure is `record_store_invalid`, exit 4, inside the transaction, so a
refused import rolls back its version-1 migration too.
`confirms_execution` now also requires the value `true`, and the importer uses
that same definition.

**The common check is the version, not every field.** `host.run` still
requires string `kind`, `taskId` and candidate fields. It is the only reader
that types them into an answer. `record show` exports the launch document
whole, and `record observe` reads nothing from it. A version-1 document
missing a field that `run` always writes is still exported for diagnosis. Its
lookup refuses.

**`record observe` does not validate the run's other stored observations.** It
interprets none of them. A repeat is compared by encoding, and a corrupt stored
document with the same ID can only conflict. `supersedes` uses the columns.
The launch document and the launch-failure detail are what it reads, and those
are checked.

**Stored-observation messages name the store, not `--run`.** `stored` reuses
the import's `Check`, whose run-mismatch message said "--run names". `Check`
now carries what names the expected run. The import's message is unchanged. A
stored document reads "the store holds it for run R". Every stored-document
refusal names the run and, for an observation, its ID.

**Tests and controls.** These tests are at the command seam.
`tests/lookup.rs`, `a_lookup_reads_the_runs_launch_fields_never_its_observation_history`,
covers F1. `a_launch_record_this_release_cannot_read_refuses_every_read_of_the_run`
covers a later version, non-JSON, a non-object and a detail with no cause.
Each is run through `record show`, `record observe` and `host.run`, beside a
rewritten copy of a real launch record as the control.
`tests/observations.rs`, `a_stored_observation_this_release_cannot_read_refuses_the_export`,
covers non-JSON, `{}`, an observed confirmation of `false`, version 2, and a
mismatched ID, run and `supersedes`. It has two controls, one of which is a
correction row, and it shows that only a well-formed confirmation derives
`execution_confirmed`. The version-1 migration test now also shows that a
refused launch-version import leaves the store at version 1. Each of three
mutations was seen to fail its test, and each source was then restored.
The lookup was switched back to reading observations. The stored-observation
check was removed. The launch-version check was disabled, which failed both
the every-read test and the migration test. A unit assertion pins
`confirms_execution` to the value `true`. That assertion is internal, since
store validation now precedes it at the seam.

**Verification.** `task check` passed all 12 principal checks, including fmt,
clippy, `dispatch:typecheck` and book-check. Its tests were 1657 passed and 0
failed: the producer's recorded 1654 plus these three. SHA-256 digests of the
86 package, spec, Taskfile and script files it read were identical before and
after the run. `task release:smoke` was not rerun. This leaf changes neither
the worker, the installed layout nor the native dependencies, which are its
trigger in the plan.

**ADRs and closes.** No ADR states the read boundary or the lookup's read
extent, and none changes. The spec and the package README carry the new
contract, including the records seam row. The leaf sits at the grove root,
which still has live work, so retirement closes no node.
