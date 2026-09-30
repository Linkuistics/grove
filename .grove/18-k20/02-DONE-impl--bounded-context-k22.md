# bounded-context-k22

## Goal

Give policy a bounded, measured context. Caller JSON arrives through
`--context`, and a `loadContext` callback reads sources through the SDK. Every
delivered byte is attributed and hashed, and it stays within documented limits
that refuse instead of truncating. Inspection shows exactly what selection saw.

## Context

The contract is the spec's `#bounded-context`, plus the caller-context shape in
`#policy-and-choice`. The request `cwd` is data. It is not the worker's runtime
cwd, which stays private. Policy imports keep module-relative resolution. SDK
reads resolve explicit relative paths against the request cwd.

## Done when

- `--context` accepts only a version-1 caller context of the specified shape,
  including `reviewedArtifact` with an ID and at most one creator form, `run`
  or `declared`. Unknown versions, unknown fields and executable fields refuse
  with their location.
- `loadContext(request, host)` returns the final serializable context, which the
  worker and Rust both validate and measure. The `select` callback receives
  that measured value.
- `host.readText(path, maxBytes)` and `host.readJson(path, maxBytes)` return the
  content with its canonical source name, byte count and SHA-256. Other
  operations are `host.diagnostic(text)` and the `host.signal` abort signal.
  `host.run` refuses as not yet supported.
- Each bound holds and each overflow refuses by name:
  - context 256 KiB by default, up to 8 MiB through `--context-bytes`;
  - one source read 64 KiB, capped at the context budget;
  - 256 sources;
  - a 1 MiB result/catalog message;
  - 256 KiB of diagnostics across both streams, drained within the bound, with
    excess ending evaluation with an output-limit error.
  None of these truncates. Policy cannot raise a ceiling after evaluation
  starts.
- Inspection reports the sources, digests, actual source bytes, final encoded
  bytes, effective limits and any bound errors. Policy diagnostics never
  corrupt `--json` output.
- `run`'s handoff record carries the reviewed-artifact association and the
  context source digests and sizes. These fill the fields
  `handoff-records-k24` recorded as absent, and `record show` exports them.
- The whole-selection deadline ends a `loadContext` that awaits a promise kept
  pending by a live timer, and one that spins synchronously. Each exits 124,
  starts no fake harness and leaves no worker. Variants that finish within the
  bound, seen to select, are the positive controls.
- Command-seam tests cover a missing required source and a failed loader. They
  cover each overflow at and just past its bound, with the boundary case seen
  to pass. They also show that an oversize prompt does not count against the
  context budget.
- The usage documentation covers computed policy, context and the limits. The
  spec's notice states what is delivered. The node brief's `Done when` holds.

## Decisions (running log)

**The caller context's open shapes, fixed here.** The spec named the fields
and left three shapes open. A **source record** is `{ name, sha256?, bytes?,
version? }`, with a nonempty `name` and at least one of `sha256` (64 lowercase
hex) or `version` (nonempty). That is the spec's "versions/digests for the
evidence". **`assessments`** is attributed per entry: each member is
`{ by, value }`, a nonempty assessor and any JSON value. **`reviewedArtifact`**
is `{ id, creator? }`, and a creator has exactly one of `run` (a canonical run
ID) or `declared` (a nonempty provider label). `facts` is any JSON object and
`acceptanceCriteria` an array of strings. Absent stays absent and `{}` stays
`{}`: nothing is defaulted. An unknown field refuses with its location. A
field whose name is executable (`program`, `args`, `argv`, `command`,
`catalog`, `candidate(s)`, `routes`, `select`, `loadContext`, `env`,
`environment`, `exec`, `shell`, `script`) gets its own message saying that a
context is data. Keys inside `facts` and assessment values are data and are
never checked. An object whose only key is `$harnessDispatch` is reserved
anywhere in a context: it is the worker's marker for a value JSON cannot
carry.

**The delivered context is the loader's result plus `measured`.** The spec's
"loaded context ... additionally carries measured source metadata" is a
`measured` array that the worker attaches and nothing else may supply. Each
entry is `{ name, via, bytes, sha256 }`: the `--context` document first, as the
front read it, then each SDK read in call order. `via` is `--context`,
`readText` or `readJson`, and `name` is the canonical (realpath) source. A
loader result that carries `measured` refuses as an unknown field. Without a
loader, the delivered context is the caller's document plus `measured`.
Without either, there is no context phase and `select` receives `undefined`.
A `routes` policy with a loader or a caller context runs the context phase
too, so its context is measured and inspected, and a failing loader refuses
it.

**Measurement.** The front re-encodes the delivered value compactly, with
object keys sorted (serde_json's map, since `preserve_order` is nowhere in the
workspace), and that encoding's length and SHA-256 are the **final encoded
bytes** and the context digest. The worker checks its own `JSON.stringify`
length first. For the same value that is never shorter than the front's,
because ryu never writes a float longer than JavaScript does, and integers
print alike. A context the worker passes therefore passes the front. The
front still checks, and it is the authority. **Actual source bytes** is the
sum of the measured entries' bytes.

**Protocol: a third, optional phase.** After the front accepts the snapshot,
it sends `context` if the policy has a loader or the request carries a caller
context. The worker answers with `{ context, measured }` or a failure. Only
then does `select` run, with `(request, delivered, host)`. `select` sees the
value `JSON.parse` made from the worker's own encoding, so it is equal to what
the front measured. The context frame is bounded by the context budget plus a
1 KiB envelope. Every other worker frame is bounded by the fixed 1 MiB message
bound. The front's frames to the worker carry the caller context, so the
worker accepts up to the 8 MiB ceiling plus 1 MiB.

**Bound violations are sticky.** An SDK read over its limit, the 257th source,
a `maxBytes` above the context budget, a context over budget, a snapshot or
result over 1 MiB, and `host.run` are recorded by the worker the first time
they happen. The policy also sees a thrown error, but catching that error
changes nothing: the phase reports the recorded failure instead of its value.
The worker takes its bounds from the evaluate message, never from the
`request.limits` the policy can see. The request is deep-frozen before any
policy code receives it.

**The limits.** `--context-bytes` is plain digits from 1 to 8388608, default
262144, exit 2 otherwise, like `--timeout-ms`. The per-read default is
`min(65536, context budget)`. A call's `maxBytes` may set any positive integer
up to the context budget, and a larger one is a sticky `source_too_large`,
because policy cannot raise a ceiling. At most 256 measured sources, the
`--context` document included, and a context's `sources` array holds at most
256 records under the same code. The message bound is 1048576 bytes and the
diagnostics bound 262144. `request.limits` carries all six. `Bound` gains a
`fixed` origin, and its `from` is `default`, the flag or argument that set it,
or `fixed`.

**Refusal codes.** At stage `context`, exit 3: `context_unreadable` (the
`--context` file), `context_invalid` (with location, for the caller document
or the loader's result, abstention included), `unsupported_version`,
`context_loader_failed`, `context_loader_unsettled`,
`context_source_unreadable` (a failed SDK read the loader did not survive,
naming that source), `context_too_large`, `source_too_large` and
`too_many_sources`. `message_too_large` is exit 3 at stage `load` or
`selection`. `output_limit` is exit 3 at stage `evaluation`, since the front
cannot tell which part printed. `unsupported_operation` (`host.run`) is exit 3
at the stage it was called in. Each bound refusal names its bound.

**SDK reads are synchronous and belong to `loadContext`.** `readText(path,
maxBytes?)` returns `{ text, source }` and `readJson` returns
`{ value, source }`, where `source` is `{ name, bytes, sha256 }`, a source
record the loader can put into `sources`. A relative path is resolved against
`request.cwd`, never the worker's directory. The file is opened
`O_NONBLOCK`, so a FIFO cannot block, and it must be a regular file. It is
decoded as strict UTF-8 with any BOM kept. `select`'s host has `diagnostic`
and `signal` only. Its reads throw, because the loader's result is the
measured value that selection sees. `host.diagnostic(text)` writes a line to
the worker's stderr, which is captured and bounded like any other policy
output.

**`host.signal` aborts when the worker is asked to stop.** The worker's TERM
listener, registered before any policy code, aborts it. It then exits with
143 unless the policy registered its own TERM listener, so that a policy
without one still dies on TERM as it did before. Probed with Bun 1.4.2:
`process.listenerCount("SIGTERM")` counts listeners, and a FIFO opened
`O_NONBLOCK` returns at once.

**Diagnostics are drained within the bound.** Both drain threads share one
count. Past 262144 bytes they keep reading and discard, so that the worker
never blocks, and set a flag. The front's channel reads wait in slices of at
most 50 ms and check that flag, so a policy that prints without end is stopped
with `output_limit`, not a timeout. The flag takes precedence over any other
outcome, a completed selection included.

**Records.** `run` fills `reviewedArtifact` from the delivered context and
`context` with `{ loader, sources, sourceBytes, encodedBytes, sha256 }`. The
delivered value itself is not stored. `bounds` gains the five new bounds
inside launch version 1, because `bounds` is the open set of effective limits
the spec lists, not a new field. `creator` stays `null` for `run-lookup-k26`.

**An explicit choice, and for routes the route, are checked before any loader
runs.** Those checks need no more policy code, so a `--choice` the catalog lacks
and an unrouted kind refuse without running `loadContext`. That extends the
spec's "before any `select` runs" to "before any `loadContext` or `select`
runs".

**`host.signal` installs its TERM listener lazily**, on first access, so a
policy that never reads the signal keeps TERM's default and dies at once, as
before this leaf. The host's listener exits only when it is the sole TERM
listener. A loader that takes the signal and then registers its own listener
therefore gets both: the abort, then its own listener, which owns its exit.

**Both sides check the count and budget bounds, and each alone is enough at
the seam.** With the worker's source-count check removed, or its context-size
check, the front's own check still refuses with the same code, so those
command-seam tests pass. With both sides removed they fail. The front's checks
are pinned by the unit tests `deliver`'s budget test and the validator's
sources test. This is the spec's "the worker and Rust both validate".

**Usage documentation is the crate README**, which gains *Context* and
*Bounds* sections. Its `loadContext` example was type-checked against the
shipped declarations and run through the worker. `CONTEXT.md` gains
**Delivered context**. The runtime evidence records the FIFO, BOM and
TERM-listener probes.

**Acceptance rows owned here, and their tests.** The command-seam tests are in
`crates/harness-dispatch/tests/`.
- Caller context as data: `context.rs`
  `a_caller_context_reaches_the_policy_as_data_and_inspection_measures_it`,
  with its empty-versus-absent and routes cases. Refusals with location,
  before any policy runs:
  `each_invalid_caller_context_refuses_at_its_location_before_any_policy_runs`,
  which covers an unknown version, an unknown field, an executable field, the
  creator forms and the reserved marker. Unreadable documents, a FIFO
  included:
  `a_context_document_that_cannot_be_read_refuses_without_waiting_on_it`.
- `loadContext` and the SDK reads:
  `a_loader_reads_measured_sources_against_the_callers_directory`, which
  checks request-cwd resolution, module-relative imports, canonical names
  through a symlink, digests, sizes and the explicit choice seen by the
  loader. The type-checked fixture runs in
  `the_typed_context_fixture_selects_through_the_worker`. Reads are closed to
  `select`: `select_reads_nothing_through_its_host_and_the_loaders_reads_close`.
  Diagnostics stay out of `--json`:
  `diagnostics_the_host_writes_stay_out_of_the_json_report`. `host.run`
  refuses even when caught:
  `host_run_refuses_even_when_the_policy_catches_the_error`.
- A missing required source and a failed loader:
  `a_missing_required_source_refuses_and_names_it` covers missing, not UTF-8,
  not JSON, a directory and a FIFO, with the cause chain.
  `a_failing_loader_refuses_and_launches_nothing` covers throw, reject,
  unsettled, abstain, invalid shape, `measured` supplied, a function, NaN, a
  cycle and a frozen-request mutation, each under both forms. Each has a
  control that selects.
- Each bound at and past its limit, in `bounds.rs`. Context at 256 KiB and at
  8 MiB:
  `the_context_budget_holds_at_its_default_and_at_its_ceiling`. Measured
  metadata counts, and an oversized document:
  `the_context_budget_counts_the_measured_sources_and_a_caller_document`.
  One read, by default, by `maxBytes`, past the budget and capped by a small
  budget, caught or not:
  `one_read_holds_to_its_limit_whatever_the_policy_does_with_the_error`.
  Sources:
  `a_context_holds_at_most_256_sources_the_document_included`. Message:
  `a_catalog_snapshot_or_result_holds_to_the_message_bound`. Diagnostics:
  `diagnostics_hold_to_their_bound_across_both_streams` and
  `a_policy_that_prints_without_end_is_stopped_for_its_output_not_its_time`.
  The flag's range:
  `context_bytes_is_one_byte_to_eight_mebibytes`. An oversize prompt:
  `an_oversize_prompt_does_not_count_against_the_context_budget`.
- The loader's hang cases in `deadline.rs`:
  `a_load_context_that_spins_is_stopped_at_the_deadline` and
  `a_load_context_awaiting_a_promise_that_live_work_keeps_pending_is_stopped_at_the_deadline`.
  Their controls are in `a_hold_that_ends_within_the_bound_reaches_the_harness`.
  The signal is covered by
  `the_host_signal_aborts_when_the_deadline_stops_a_waiting_loader`.
- The handoff record: `context.rs`
  `run_records_the_reviewed_artifact_and_the_context_by_digest_and_size`.
  `record show` exports the fields, and the value is absent from the store.

**Mutations seen to fail**, each against its named test: a sticky breach
ignored after the loader returns; reads resolved against the worker's cwd;
`readText` unmeasured; diagnostics unbounded; the channel not polling the
overflow flag (the test took 60 s and failed on elapsed time); the message
bound unchecked by the worker; loader results unvalidated; the caller context
left out of the request; the read default not capped by a small budget;
`maxBytes` allowed past the budget; `host.run` accepted; the signal never
aborting; the host's TERM listener exiting regardless of the policy's own.

**Delivery.** On 2026-10-01, `task release:smoke` passed the static and computed
cases through both fronts on all three targets. The worker build was
`0a13e916bb83…`, and the glibc and CPU controls fired. The archive layout is
unchanged: the host module is compiled into the worker, and the SDK ships as
the same `sdk/index.{ts,d.ts}`.

**The node closes with this leaf.** `computed-policy-k20`'s `Done when` holds:
`select` and the dynamic example came from `computed-selection-k21`, and the
context, the SDK operations, the bounds and inspection come from this leaf.
The facts later leaves need are promoted to the root brief. No ADR: the
context schema is versioned and the protocol private, so nothing here is both
hard to reverse and surprising without context.
