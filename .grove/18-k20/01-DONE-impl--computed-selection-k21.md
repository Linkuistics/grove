# computed-selection-k21

## Goal

Replace the "`select` not yet supported" refusal with the computed form. An
asynchronous `select` chooses a configured candidate, or refuses, from the
versioned request. Explicit choices under `select` are accepted, refused or
caught as a mismatch.

## Context

The contract is the spec's `#policy-and-choice`. The request here carries
`schemaVersion`, `kind`, `cwd`, optional `taskFile`, `taskId` and
`explicitChoice`, and the effective `limits`. Caller and loaded context arrive
with `bounded-context-k22`. Until then, pass an absent context and refuse
`--context` explicitly.

## Done when

- A policy with both or neither of `routes` and `select` refuses. `select` may
  be synchronous or return a promise. `selected` requires a configured
  candidate ID and a nonblank reason. `refused` requires a code, message and
  remedy, and inspection and `run` report them with exit 3.
- Exceptions, a promise left unsettled once the worker's event loop drains, an
  unknown ID, abstention and malformed results refuse with distinct codes. A
  result cannot supply argv. Rust validates the result against the catalog
  snapshot.
- The whole-selection deadline from `selection-deadline-k44` ends a `select`
  that awaits a promise kept pending by a live timer. It also ends one that
  spins synchronously. Command-seam tests show each exiting 124 with no
  fake-harness marker and no surviving worker. Variants that finish within the
  bound, seen to select, are the positive controls.
- With `--choice`, `select` receives `explicitChoice`. Returning the same ID
  accepts it and returning `refused` refuses it. Any other ID is
  `explicit_choice_mismatch`, including one the policy calls a fallback.
- Inspection distinguishes computed selection from a route and from an explicit
  choice under `routes`, and it shows the policy's reason.
- The deterministic dynamic example is registered as an embedded specifier,
  with its declarations and readable source. A command-seam test selects
  through it.
- The per-target installed smoke gains the computed TypeScript case, and it
  passes on every target. The archive assertions include the new example.

## Decisions (running log)

**Two phases: Rust judges the policy before `select` runs.** The worker
returns the entry's snapshot first. Rust validates it, and checks an explicit
choice against its catalog, before it asks the worker to call `select`; the
result then comes back and is validated against that same snapshot. So an
invalid catalog, both or neither form, and an unknown `--choice` refuse
without running any `select` code, and a hanging `select` on an invalid
policy cannot turn a `policy_invalid` into a timeout. The snapshot is taken
at import, before `select` runs, so a result cannot add a candidate by
mutating the catalog. After a routes snapshot the front just drops the
channel; the worker reads that clean end as "nothing more" and exits quietly.
The spec's "returns the catalog snapshot with the result" is reworded to this
order; the protocol is private to the release.

**The worker drives evaluation from an async function, never a top-level
await.** Probed with the pinned Bun 1.4.2, plain and compiled: a main module
whose top-level `await` is left pending with nothing live busy-spins at 100%
CPU and never emits `beforeExit`, while the same await inside an async
function called without TLA lets the loop drain and `beforeExit` fire (for a
pending `select` and for a pending dynamic import alike). The worker's
`beforeExit` handler reports what was pending: an unsettled `select` refuses
`selection_unsettled`; an unsettled import is a `policy_import_failed`, where
before it spun until the deadline.

**Result codes, all exit 3 at stage `selection`:** `selection_threw`
(exception or rejection), `selection_unsettled`, `selection_abstained`
(`undefined` or `null`), `selection_malformed` (with the `result.…`
location; an extra field such as `args` is malformed, so a result cannot
supply argv), `unknown_candidate`, `explicit_choice_mismatch`. A policy's own
`refused` is reported under the stable code `policy_refused`, with its code
as `policyCode` and its message and remedy, so owner codes never collide with
harness-dispatch's documented set. An explicit choice is matched first: with
`--choice`, any other returned ID is a mismatch, known or not.

**`select(request)` only, in this release.** Context and host operations
arrive with `bounded-context-k22`; the SDK declares only the request
parameter rather than a context typed `undefined` or an empty host, and the
worker passes nothing else. It is called as a method, so `this` is the
policy. The request's `limits` is `{ selectionMs }`, the effective bound;
k22 adds the context bounds beside it.

**Reporting.** `selection.form` is `routes` or `select`; `selectedBy` gains
`select`, which with a non-null `explicitChoice` means the policy accepted
the choice. The reason is the policy's own, verbatim.

**A refusal decided mid-conversation stops the worker gracefully.** The
front drops the channel and gives the worker its cleanup grace before
killing it, as after a result, so output the policy buffered is still
captured; only protocol failures kill at once.

**The dynamic example is `harness-dispatch/examples/dynamic`, built on the
static example.** It imports `catalog` and `routes` from
`harness-dispatch/examples/static` by specifier (the worker's tsconfig
`paths` gains `harness-dispatch/examples/*`, so the bundle shares one static
module), and computes what a table cannot say: with no choice it applies the
route and names the entry; with a choice it accepts effort at or above the
route's and refuses below it (`effort_below_route`); an unrouted kind sets no
floor, and without a choice refuses in the policy's own words. No clock,
file, network or model, so it is deterministic.

**Usage documentation for `select` is in the crate README now**, not left to
k22: leaving it saying `select` is refused would be false once this lands.
k22 adds context, the loader and the limits to it. The README's example was
type-checked against the shipped declarations and run through the worker.

**Acceptance rows owned here, and their tests.** Computed selection:
`select.rs` `a_synchronous_or_asynchronous_select_chooses_a_configured_candidate`
and `run_launches_the_computed_choice_and_records_how_it_was_made`. Policy
errors and explicit-choice mismatch launch nothing:
`each_way_select_can_fail_refuses_with_its_own_code_and_launches_nothing`,
`any_other_id_for_an_explicit_choice_is_a_mismatch_whatever_the_reason`,
`a_choice_the_catalog_lacks_refuses_before_select_runs`. Callback timeout
launches nothing: `deadline.rs` `a_select_that_spins_is_stopped_at_the_deadline`
and `a_select_awaiting_a_promise_that_live_work_keeps_pending_is_stopped_at_the_deadline`,
with `a_hold_that_ends_within_the_bound_reaches_the_harness` as the control
seen to select. Diagnostics stay out of the report:
`what_select_prints_is_kept_apart_from_the_report`. The dynamic example:
`examples.rs` `the_dynamic_example_consults_the_static_routes_and_polices_explicit_choices`.
Mutations seen to fail: skipping the front's pre-`select` choice check, and
disabling the worker's `beforeExit` report; removing the SDK's
`select?: never` guard made `refused-shapes.ts` fail the type check.

**Delivery.** `task release:smoke` on 2026-10-01 passed the static and new
computed cases through both fronts on all three targets, worker build
`77b6f49de68c…`, with the glibc and CPU controls firing; the archives carry
`examples/dynamic.{ts,d.ts}`, which the manifest now lists.
`bash scripts/check.sh` passed on the same tree.
