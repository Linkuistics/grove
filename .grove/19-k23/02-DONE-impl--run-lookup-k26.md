# run-lookup-k26

## Goal

Let trusted policy read an earlier run's immutable launch fields through
`host.run(runId)`. A reviewed artifact's creator run can then be resolved to its
recorded catalog snapshot, and the selection that used it records that
provenance.

## Context

The spec's `#bounded-context` gives run lookup as a read-only protocol request
to the Rust store. The worker never opens the database. `#identity-and-creator`
defines the two creator forms. Dispatch compares no providers and has no review
semantics: that is the review policy's work, in `review-policy-k35`.

## Done when

- `host.run(runId)` returns the run's immutable launch fields: task identity,
  kind, catalog snapshot with provider, model and effort, and timestamp. It
  also returns any launch-failure or not-executed detail, or reports the run
  missing. An unreadable, corrupt or unknown-version store refuses the
  selection and never reads as missing.
- Loaded context can carry the creator run snapshot it resolved, and it is
  measured and inspectable like any other source. Inspection shows it. So does
  the run record of a selection that used it, together with the creator
  reference and its evidence class.
- Lookup goes only by run ID. There is no lookup by task identity, artifact
  identity or handle.
- Command-seam tests: a round trip of `host.run` against a recorded run; missing
  versus unreadable; a run that carries a launch-failure detail; and a changed
  current catalog that does not alter the returned snapshot.
- This node's `Done when` holds. Retiring this leaf closes the node. As this
  leaf's last act, cut the node's `review-impl` as the node's sibling,
  directly after it, never inside it. Run `grove-llm leaf-insert --kind
  review-impl evaluation-boundary-k27 dispatch-records`, targeting the first root
  entry after this node (today `evaluation-boundary-k27`). Give it
  `**Reviews:** dispatch-records-k23`, and write the node brief's review doubts
  into its body.

## Decisions (running log)

**`host.run` is `loadContext`'s, like the reads.** The root brief asks this
leaf to declare the operation "on both hosts". Both runtime hosts carry `run`,
as both carry `readText`: the loader's answers, and `select`'s throws that it
is available in `loadContext` only, which is not a bound. The SDK declares it
on `ContextHost` alone, as it declares the reads. That follows the spec:
`select`'s host has `diagnostic` and `signal` alone, because the context it
receives is the measured one. A lookup after `loadContext` returned throws as
a late read does.

**The answer is a projection of the launch fields, never the document.** Found:
`{ runId, status: "found", recordedAt, kind, taskId, candidate: { id,
provider, model, effort }, launchFailure }`. Missing: `{ runId, status:
"missing" }`. `taskId` is `null` when the run had none. `launchFailure` is
`null`, or the appended detail as `record show` exports it: its `recordedAt`
beside the detail's own fields, `cause` among them. The whole launch document is not handed over:
its argv holds the prompt, up to 1 MiB, which would break the 256 KiB context
budget. Its `program` and `args` are executable words, which no context
carries.

**Only the front judges the store; the worker only asks.** The worker checks
that the ID is a string in the canonical run-ID form and throws a `TypeError`
otherwise, so a malformed ID never crosses the channel and is not a source.
The front checks the form again, and a non-canonical ID from the worker is a
protocol error. A missing store file, an empty one and a store without the run
all answer `missing`. Any other failure to read refuses the selection with the
store's own exit-4 refusal (`record_store_invalid`, `record_store_unwritable`,
`record_store_locked`, `record_commit_failed`), naming the store. The front
ends the conversation at once and stops the worker. So the refusal is sticky by
construction: no policy code runs after it, and a policy that catches the
error has nothing to catch.

**The lookup's lock wait is inside the selection bound.** The commit's 2-second
wait happens after the worker is reaped, and neither extends nor consumes the
selection bound. A lookup happens while the worker waits, so its wait is the
lesser of 2 seconds and the time left. A wait that runs to the deadline is a
selection timeout, exit 124. A read transaction is short and holds a shared
lock only while it runs, so no lock is held across evaluation.

**The delivered context carries the answers as `runs`.** harness-dispatch
attaches every answer `loadContext` received, in call order, beside
`measured`. The loader cannot supply `runs`, just as it cannot supply
`measured`, so a creator's provider in the delivered context is always the
store's and never a transcription. `runs` is present only when `loadContext`
looked a run up. The delivered context of a policy that looks nothing up is
then unchanged byte for byte, digest included. Each answer is also a measured
source, `{ name: <run ID>, via: "run", bytes, sha256 }`, over the front's
compact encoding of the answer with sorted keys. So a lookup counts against the
256-source bound, sticky like a read, and its digest is in the run record's
context sources. A repeated lookup is another answer and another source, as a
repeated read is. The front keeps its own answers. The worker's context frame
must carry the same `runs`, and its `run` measured entries must be the front's
in order, or the worker has broken the protocol.

**The run record's `creator` names what the delivered context offered.**
Launch version 1 already has a `null` placeholder for it. It stays `null`
without a reviewed-artifact creator. Otherwise it is `{ reference, evidence,
provider, lookup }`. The reference is `{ run }` or `{ declared }` as the context
gave it. The evidence is `execution_recorded` or `declared`. The provider is
the declared label, or the found run's recorded provider, else `null`. The
lookup is the first answer in `runs` for the referenced run, or `null` when
`loadContext` did not look it up. Dispatch cannot tell which facts a policy
used. It records the creator provenance the selection had. Inspection reports
the same object, and both text forms add a `creator` row with the run's task
identity beside it.

**`inspect` opens the store only to answer a lookup, and records nothing.**
The worker's lookups must get the same answers under `inspect` as under `run`,
so inspection reads the store when policy asks. It never creates a store, and
it writes no run. A read that finds a crashed commit's journal rolls it back,
as every reader of a rollback-journal store does.

**`unsupported_operation` is removed.** Nothing is unsupported once run lookup
lands. The worker's `Unsupported` breach, the front's `Unsupported` outcomes
and the refusal code go with it.

**A launch document this release cannot read refuses the lookup.** The answer
is projected field by field. A document of another launch version, or one
whose kind, task identity or candidate fields are not strings, refuses as
`record_store_invalid`, exit 4. A future version-2 document is then refused by
this release, never answered with less, and corruption never becomes a missing
provider.

**Controls seen to fire.** Each was a mutation of one source, reverted, with
every source and test file's digest compared afterwards, all unchanged. Every
one failed the tests named, all in `tests/lookup.rs` unless another file is
named:

- A store failure answered as missing, and an unsearchable directory read as
  an absent store: the unreadable-store test.
- The lookup's wait left at the full 2 seconds, and no expiry check after a
  lookup: the locked-store test. The first waited past the one-second bound,
  and the second refused on the lock instead of timing out.
- The front not attaching `runs`, the worker not delivering them to `select`,
  and the worker reporting none to the front: the round-trip test. The last
  failed as a protocol error.
- Both source-count checks skipped for lookups: the source-bound test. With
  only the worker's check skipped, it passes, because the front's count at
  delivery still refuses. That matches the reads (`bounded-context-k22`).
- The worker accepting any ID, and `select`'s host looking runs up: the
  closed-and-malformed test.
- The launch failure dropped from the answer: the launch-failure test.
- The creator left `null`: the creator-record test.
- A loader allowed to supply `runs`: `context.rs`'s failing-loader test.
- A later launch version answered: the unit test
  `a_later_launch_version_or_a_missing_field_is_refused_never_answered_with_less`.

**`review-selector-k36`'s charter said an unreadable store refuses with the
declaration remedy. It no longer does.** Dispatch refuses such a store with
exit 4 before the selector can see it, as this leaf's `Done when` requires, and
the spec's review section never said otherwise. So that one bullet of k36 now
says dispatch refuses it first, with the store's own remedy. A declaration
would not repair an unreadable store. Its other cases are unchanged.

**No in-session reviewer.** The node's scheduled `review-impl`, cut as this
leaf's last act, carries the doubts: missing versus unreadable, run lookup's
authority and the recorded creator provenance.

**No ADR.** The protocol is private to one build pair, and the answer, `runs`
and `creator` are versioned JSON shapes that a later version can extend, so
none is hard to reverse. The contract is in the spec's *Bounded context* and
*Records and later observations*, the reasons are here, and `host.ts`,
`worker.rs` and `store.rs` carry them at the code.

**Done-when instruments.** Command-seam tests are in
`crates/harness-dispatch/tests/lookup.rs` unless another file is named.

- Immutable launch fields, launch-failure detail, missing, and unreadable
  stores refusing without reading as missing:
  `host_run_returns_a_recorded_runs_immutable_launch_fields` (round trip),
  `a_run_that_carries_a_launch_failure_returns_its_detail`,
  `a_missing_run_is_reported_missing_whether_or_not_there_is_a_store` and
  `a_store_that_cannot_be_read_refuses_and_never_reads_as_missing`. The last
  covers a directory that cannot be searched, an unreadable file, and corrupt,
  foreign and newer stores. Each refuses under `inspect` and `run`, even when
  the loader catches the error, beside a readable control that is found. The
  unit test `a_later_launch_version_or_a_missing_field_is_refused_never_answered_with_less`
  covers launch records this release cannot read. The lock wait:
  `a_lookup_waits_for_a_locked_store_only_within_the_selection_bound`.
- The snapshot carried in the delivered context, measured and inspectable:
  the round trip checks `runs`, the run source's size and digest, inspection's
  `creator` and both text rows. `each_lookup_is_a_measured_source_within_the_source_bound`
  checks the source bound. `context.rs`'s loader and caller cases refuse a
  supplied `runs`. The unit tests in `context.rs` check attachment and the
  creator derivation. The run record's creator: the round trip, and
  `the_run_record_names_the_creator_reference_its_evidence_and_its_lookup` for
  declared, unresolved and absent creators.
- Lookup only by run ID, open to `loadContext` alone:
  `run_lookup_is_the_loaders_alone_and_takes_only_a_run_id`. The SDK has no
  other lookup, and `refused-shapes.ts` pins `select`'s host without `run`.
- A changed current catalog:
  `a_changed_current_catalog_does_not_alter_the_returned_snapshot`, whose
  control is the same report selecting from the changed catalog.
- The declarations and the runtime agree:
  `the_typed_lookup_fixture_selects_through_the_worker` runs
  `worker/typecheck/lookup-policy.ts`, which `task dispatch:typecheck` checks.
  The README's lookup example is the same loader and `select`.

**Checks.** `task check` passed all 12 principal checks, with 1654 tests and
none failing. The ignored ones are other crates' subprocess fixtures and opt-in
native smokes. Every changed file outside `.grove/` had the same digest before
and after that run.

**Delivery.** On 2026-10-01, `task release:smoke` passed the static and computed
cases through both fronts on all three targets. The worker build was
`215ead5d3838…`, and the glibc and CPU controls fired on each Linux target. The
archive layout is unchanged: the lookup is compiled into the worker, and the
SDK still ships as `sdk/index.{ts,d.ts}`.

**The node closes with this leaf.** `dispatch-records-k23`'s `Done when` holds.
The evidence classes, `record observe` and `record show` came from
`run-observations-k25`. `host.run`, missing versus unreadable, the worker never
opening the database, and the recorded creator provenance come from this leaf,
and the per-target smoke passes. The facts later increments build on are
promoted to the root brief. The node's review is cut as its root sibling,
`**Reviews:** dispatch-records-k23`, ahead of `evaluation-boundary-k27`. This
session ran without `HARNESS_DISPATCH_RUN_ID`, and the `**Creator:**` convention
ships only with `creator-reference-k38`, so the review carries no creator line.
