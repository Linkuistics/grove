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

## Decisions (running log)

**Review boundary.** The producer commits are `637b8805` (`run-observations-k25`)
and `e03558c0` (`run-lookup-k26`, closing `dispatch-records-k23`). The reviewed
tip is `e03558c0`; the working change began empty. This review reads source,
requirements, committed changes and the producer's recorded verification. It
runs no tests, builds, lints, formatters or runtime probes, and changes no
production or test code.

**Source fallback.** Tier 2 verification queried project
`Users-antony-Development-grove.new-tool-for-harness-selection-and-execution`.
The coverage generation is `2026-09-29T11:18:53Z`, older than this package.
Package searches returned zero symbols with no more pages. Coverage reports
the package paths as `not_tracked`, changed metadata for the root context and
Taskfile, and excluded `docs/`. The review therefore relies on direct source
reads; no graph absence is treated as source absence.

**The lookup's deadline is checked after unbounded work.** `host.run` uses the
same store load as `record show`, materializing every observation of the run.
Its supplied duration limits SQLite lock waits, while the front checks the
selection deadline only when that whole synchronous load returns. This is an
actionable finding: a compact creator lookup must not inherit an unbounded
observation history or postpone deadline enforcement until after processing it.

**Stored JSON is not validated as a record.** The new observation load only
parses JSON, and export treats missing measurements as unobserved and an
`observed` confirmation state as confirmation without checking its value.
`record show` also bypasses the new launch-version checks, while `record
observe` checks the run row's existence without reading its launch document.
This is an actionable read-side validation gap, distinct from refusing an
unknown SQLite schema version or malformed incoming observation.

**Keep accepted provenance costs and scheduled work outside the findings.** A
run reference is an attestation, not a verified artifact association. A routes
policy that does not look up a creator intentionally records a null provider
and lookup; repeated lookups intentionally use the first answer for creator
provenance. Those cases match the spec and tests. Handled cancellation, signal
transparency, the shipped review selector and the Grove adapter have later
owning leaves; their absence in this increment is not a finding.

**Integration is next.** Cut `dispatch-records-k51` immediately after this
review and before the first later sibling with live work,
`evaluation-boundary-k27`. Its body names this review's stable handle and
leaves both findings open to triage. Retirement closes no ancestor: the root
still has live implementation work. No ADR or glossary change is warranted
by unresolved review findings.

## Findings

Both findings are P2 correctness issues inferred from the reviewed source.
Their failure scenarios were not executed in this inspection-only session.
Paths below are relative to `crates/harness-dispatch/`, and line numbers refer
to producer tip `e03558c0`.

### F1 — P2: Keep creator lookup independent of observation history and within its deadline

`src/record.rs:159-166` implements `host.run` with `store::load`. That load,
at `src/store.rs:478-500`, queries, copies and JSON-parses every observation
of the run before returning its launch fields. The projection never uses
those observations. Observation imports may each contain 1 MiB, and neither
the number of imports nor the cumulative history is bounded. Thus a creator
lookup whose returned context is a few hundred bytes can materialize an
arbitrarily large history in the front process, including superseded evidence.
Repeating the lookup repeats that work.

This work is outside active deadline enforcement. `src/choice.rs:201-204`
passes the remaining time as a duration to `record::lookup`, but
`src/store.rs:575-585` applies it only as SQLite's busy timeout.
`src/worker.rs:242-247` checks the deadline after the synchronous lookup
returns; it cannot stop the SQL iteration or JSON parsing in progress. With
a large legitimate observation history and `--timeout-ms 1000`, the front
can outlive the selection deadline by the time needed to read and parse that
history, or exhaust memory before returning an actionable timeout. The worker
remains waiting during that interval, and its stop/reap path is reached only
afterward. This breaks the whole-selection time contract even though a late
successful lookup is eventually refused.

The lookup needs a bounded launch-field read independent of observation
history, with deadline enforcement during store work rather than only after
it returns. `tests/lookup.rs:474-539` exercises a lock wait against a run with
no observations; it cannot expose this path. Retain that control and cover a
creator with substantial imported and superseded evidence while checking the
selection's elapsed bound and worker cleanup.

### F2 — P2: Validate stored documents before exporting or attaching evidence

`src/store.rs:658-661` accepts any syntactically valid JSON as a stored
document. The new observation read at `src/store.rs:493-498` does not check
its version, required fields, measurement types or association with the row.
`src/record.rs:462-470` then substitutes an empty object for a non-object
observation and expands absent measurements as unobserved. Moreover,
`src/observation.rs:1014-1015` treats an `executionConfirmation` whose state
is `observed` as confirmation without checking its required `true` value.
A logically corrupt stored document such as `{}` is therefore exported as
unobserved evidence rather than refused; an observed confirmation with a
null or false value can mark execution confirmed. A future observation
version is likewise interpreted using today's measurement vocabulary.
These are normal read-side data-validation failures, not a claim to defend
against a hostile owner who can replace the database.

The same inconsistency reaches launch documents. Only `record::lookup`
checks launch version and projected fields (`src/record.rs:167-203`).
`record show` returns the store load directly (`src/record.rs:146-151`),
and `record observe` checks only `SELECT 1 FROM runs`
(`src/store.rs:313-321`). For example, a future writer can keep SQLite schema
2 while writing launch-document version 2, as the store design permits.
Today's `host.run` refuses that run, but `record show` succeeds and
`record observe` appends evidence to it. A malformed launch JSON string is
even accepted by an ordinary observation import, since the import never
parses it. The spec requires unreadable records and unsupported versions to
refuse, and this leaf explicitly asks for that distinction across all three
operations.

Validate the stored shapes and versions at the appropriate common read/import
boundary, preserve missing versus unreadable, and refuse with exit 4 before
exporting derived evidence or committing an import/migration. The unknown
launch-version test at `src/record.rs:555-590` exercises `lookup` only.
The observation tests validate incoming files and a later SQLite version
(`tests/observations.rs:420-495,716-737`), not stored observation envelopes or
unknown launch-document versions through `show` and `observe`. Add those
read-side cases with valid-record controls and verify refusal leaves the store
and any version-1 migration unchanged.

## Examination of the mandated doubts

- **Immutability under import:** examined and sound on the intended store.
  `src/store.rs:80-118` installs update/delete abort triggers for runs,
  launch failures and observations. The importer issues only an observation
  INSERT, and the version-1 migration only creates the observation schema.
  `tests/observations.rs:533-590` checks launch bytes before and after import
  and correction, plus update/delete refusals; `tests/records.rs:815-840`
  checks launch and launch-failure triggers. The migration and its rollback
  are covered at `tests/observations.rs:678-714`.
- **Idempotency and conflicts:** examined and sound within the documented
  parsed-JSON semantics. The validated envelope's sorted compact encoding is
  compared before correction checks (`src/observation.rs:145-155`,
  `src/store.rs:323-347`). Key order and whitespace disappear; integer and
  floating representations remain different by the producer's explicit
  decision. Imports serialize under BEGIN EXCLUSIVE, and the UNIQUE
  supersedes column backs the same-run/current-target checks, so concurrent
  corrections cannot branch. `tests/observations.rs:263-418` covers repeats,
  conflicts, cross-run correction refusal and chain extension. The review
  makes no arbitrary-precision numeric guarantee beyond the parsed values.
- **Missing versus unreadable:** filesystem permission errors are preserved
  by `try_exists().unwrap_or(true)`; absent and pristine files are missing,
  while SQLite identity/version, open and lock errors refuse. The command
  cases at `tests/lookup.rs:290-425,474-539`, `tests/records.rs:674-719`
  and `tests/observations.rs:593-617,678-737` support those paths.
  SQLite is responsible for crashed rollback-journal recovery; the code has
  no custom journal bypass. These files provide no executed crash-recovery
  evidence in this review. Stored-document validation is the exception in F2.
- **Lookup authority:** examined and sound on the intended worker path.
  The front retains each answer and checks the worker's runs and run sources
  against those answers in order (`src/worker.rs:226-299`); delivery rejects
  a supplied runs field (`src/context.rs:569-586`) and attaches checked
  answers itself. The worker freezes lookup answers and delivered context
  (`worker/src/host.ts:171-199`, `worker/src/main.ts:274-297`), and closes
  lookup access after loadContext. The worker never receives the store path
  or opens the database. `tests/lookup.rs:179-259,542-666` covers delivery,
  source limits and closed/late lookup access; caller/loader forged runs
  are refused in `tests/context.rs`. These checks are not a sandbox against
  hostile trusted policy replacing runtime globals or using descriptor 3.
- **Selection bound and worker cleanup:** the lock wait is cut to time left,
  and reaching the deadline takes timeout precedence over a lock refusal.
  Once a lookup refuses, the front closes the channel; the synchronous
  worker lookup exits on EOF even if host.signal was obtained. The front's
  refusal cleanup waits at most its grace, then kills and waits
  (`src/worker.rs:547-567,937-957`); expiry sends TERM first. The unbounded
  lookup work before that cleanup is F1. The inspected lookup command tests
  cover the lock deadline but do not obtain host.signal or assert reaping
  after store refusal; this review derives that path from source.
- **Creator provenance:** examined and sound under the accepted contract.
  `src/context.rs:228-252` uses the first lookup of the referenced run and
  records null when it was not looked up, with declared provenance separate.
  `tests/lookup.rs:668-731` covers routes without lookup, declarations and
  absent creators; `src/context.rs:1209-1256` checks repeated/other-run
  answers. Inspection and launch use the same creator derivation. This is
  the provenance offered to selection, not proof that policy used it or that
  the referenced run authored the artifact; those are explicit spec/ADR costs.

## Verification evidence reviewed

`run-observations-k25` records command-seam/unit checks and mutation controls
for repeats, conflicts, correction, immutability, envelope validation and
migration. `run-lookup-k26` records `task check` passing with 1654 tests,
source digests unchanged across that run, and `task release:smoke` passing
on all three targets with Linux glibc/CPU controls. Those are the producers'
recorded runs, not checks rerun here, and their covered cases do not close F1
or F2. This review changed only task-tree artifacts.
